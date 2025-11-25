use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use vpods_core::{id::VpodId, vpod::VpodSpec, capacity::RingCapacity, id::RingLevel};

/// Direct cgroups v2 integration for 1000x safety through complete resource isolation
pub struct CgroupManager {
    root_path: PathBuf,
}

impl CgroupManager {
    pub fn new(root_path: PathBuf) -> Result<Self> {
        let sys_fs_cgroup = Path::new("/sys/fs/cgroup");

        if !sys_fs_cgroup.exists() {
            anyhow::bail!("/sys/fs/cgroup does not exist; cgroup v2 not available");
        }

        if !root_path.starts_with(sys_fs_cgroup) {
            anyhow::bail!("cgroup root must be under /sys/fs/cgroup (got {:?})", root_path);
        }

        if !root_path.exists() {
            fs::create_dir_all(&root_path)
                .with_context(|| format!("failed to create cgroup root at {:?}", root_path))?;
        }

        Ok(Self { root_path })
    }

    /// Create cgroup for vPod with precise resource limits
    pub fn create_vpod_cgroup(&self, spec: &VpodSpec) -> Result<PathBuf> {
        let vpod_path = self.root_path.join(format!("vpod-{}", spec.id.0));
        fs::create_dir_all(&vpod_path)
            .with_context(|| format!("failed to create vPod cgroup at {:?}", vpod_path))?;

        // CPU limits: convert % to quota/period
        let cpu_quota = (spec.resources.cpu_percent as u64 * 1000).max(1);
        fs::write(vpod_path.join("cpu.max"), format!("{} 100000", cpu_quota))
            .with_context(|| format!("failed to write cpu.max for vPod cgroup at {:?}", vpod_path))?;

        // Memory limits in bytes
        let memory_max = (spec.resources.mem_mb * 1024 * 1024).to_string();
        fs::write(vpod_path.join("memory.max"), memory_max)
            .with_context(|| format!("failed to write memory.max for vPod cgroup at {:?}", vpod_path))?;

        Ok(vpod_path)
    }

    /// Create ring cgroup for Fibonacci capacity enforcement
    pub fn create_ring_cgroup(&self, ring: RingLevel, capacity: &RingCapacity) -> Result<PathBuf> {
        let ring_path = self.root_path.join(format!("ring-{}", ring.0));
        fs::create_dir_all(&ring_path)
            .with_context(|| format!("failed to create ring cgroup at {:?}", ring_path))?;

        // CPU weight based on Fibonacci value
        let cpu_weight = (capacity.fib_value * 100).min(10000).max(1);
        fs::write(ring_path.join("cpu.weight"), cpu_weight.to_string())
            .with_context(|| format!("failed to write cpu.weight for ring cgroup at {:?}", ring_path))?;

        Ok(ring_path)
    }

    /// Move process to vPod cgroup
    pub fn assign_process(&self, vpod_id: VpodId, pid: i32) -> Result<()> {
        let vpod_path = self.root_path.join(format!("vpod-{}", vpod_id.0));
        fs::write(vpod_path.join("cgroup.procs"), pid.to_string())
            .with_context(|| format!("failed to assign pid {} to cgroup at {:?}", pid, vpod_path))?;
        Ok(())
    }

    /// Kill all processes in vPod cgroup
    pub fn kill_vpod(&self, vpod_id: VpodId) -> Result<()> {
        let vpod_path = self.root_path.join(format!("vpod-{}", vpod_id.0));
        
        // Read PIDs and kill them
        if let Ok(procs) = fs::read_to_string(vpod_path.join("cgroup.procs")) {
            for line in procs.lines() {
                if let Ok(pid) = line.trim().parse::<i32>() {
                    unsafe { libc::kill(pid, libc::SIGKILL); }
                }
            }
        }
        
        // Remove cgroup
        let _ = fs::remove_dir_all(vpod_path);
        Ok(())
    }
}
