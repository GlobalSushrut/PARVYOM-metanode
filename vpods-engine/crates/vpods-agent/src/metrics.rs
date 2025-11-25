use anyhow::Result;
use procfs::{process::Process, CpuInfo, Meminfo, LoadAverage, Current};
use crate::{SystemMetrics, VpodMetrics, RingMetrics};
use vpods_core::id::{VpodId, RingLevel};

/// High-performance metrics collection for 10x efficiency
pub struct MetricsCollector;

impl MetricsCollector {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    /// Collect system-wide metrics from /proc
    pub fn collect_system_metrics(&self) -> Result<SystemMetrics> {
        let meminfo = Meminfo::current()?;
        let loadavg = LoadAverage::current()?;
        let cpuinfo = CpuInfo::current()?;

        // Calculate memory usage
        let memory_total_mb = meminfo.mem_total / 1024;
        let memory_free_mb = meminfo.mem_free / 1024;
        let memory_usage_mb = memory_total_mb - memory_free_mb;

        // Get CPU count for usage calculation
        let cpu_count = cpuinfo.num_cores();

        Ok(SystemMetrics {
            cpu_usage_percent: (loadavg.one * 100.0) / cpu_count as f32,
            memory_usage_mb,
            memory_total_mb,
            io_read_bytes: 0,  // TODO: Read from /proc/diskstats
            io_write_bytes: 0,
            network_rx_bytes: 0, // TODO: Read from /proc/net/dev
            network_tx_bytes: 0,
            processes_running: self.count_running_processes()?,
            load_average_1min: loadavg.one,
        })
    }

    /// Collect vPod-specific metrics from /proc
    pub fn collect_vpod_metrics(&self, vpod_id: VpodId, pid: i32) -> Result<VpodMetrics> {
        let process = Process::new(pid)?;
        let stat = process.stat()?;
        let statm = process.statm()?;
        let io = process.io().unwrap_or(procfs::process::Io {
            read_bytes: 0,
            write_bytes: 0,
            cancelled_write_bytes: 0,
            rchar: 0,
            wchar: 0,
            syscr: 0,
            syscw: 0,
        });

        Ok(VpodMetrics {
            vpod_id,
            cpu_nanos: (stat.utime + stat.stime) * 1_000_000, // Convert jiffies to nanos (approx)
            memory_bytes: statm.resident * 4096, // Pages to bytes (4KB pages)
            io_read_bytes: io.read_bytes,
            io_write_bytes: io.write_bytes,
            network_rx_bytes: 0, // TODO: Parse /proc/net/dev for this process
            network_tx_bytes: 0,
            syscalls_count: 0, // TODO: Get from eBPF
            page_faults: stat.majflt + stat.minflt,
        })
    }

    /// Collect ring-level aggregated metrics
    pub fn collect_ring_metrics(&self, ring: RingLevel) -> Result<RingMetrics> {
        // TODO: Aggregate metrics from all vPods/HyperCells in this ring
        // This would typically read from cgroup stats and eBPF maps
        
        Ok(RingMetrics {
            ring,
            hypercells_active: 0,
            cpu_usage_nanos: 0,
            memory_usage_bytes: 0,
            throttle_events: 0,
        })
    }

    /// Detect node hardware capacity from /proc
    pub fn detect_node_capacity(&self) -> Result<vpods_core::capacity::NodeCapacity> {
        let cpuinfo = CpuInfo::current()?;
        let meminfo = Meminfo::current()?;

        Ok(vpods_core::capacity::NodeCapacity {
            cores: cpuinfo.num_cores() as u32,
            ram_mb: meminfo.mem_total / 1024,
        })
    }

    fn count_running_processes(&self) -> Result<u32> {
        let mut count = 0;
        for entry in std::fs::read_dir("/proc")? {
            let entry = entry?;
            if let Ok(pid) = entry.file_name().to_string_lossy().parse::<i32>() {
                if let Ok(process) = Process::new(pid) {
                    if let Ok(stat) = process.stat() {
                        if stat.state == 'R' { // Running state
                            count += 1;
                        }
                    }
                }
            }
        }
        Ok(count)
    }
}
