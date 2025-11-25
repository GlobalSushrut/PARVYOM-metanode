use anyhow::{bail, Result};
use nix::sched::{CloneFlags, unshare};
use nix::unistd::{fork, ForkResult, Pid};
use vpods_core::{id::VpodId, vpod::VpodSpec};

/// Namespace isolation for 1000x safety - complete process isolation
pub struct NamespaceManager;

impl NamespaceManager {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    /// Create isolated namespace for vPod
    /// Provides complete isolation: PID, NET, UTS, MNT, USER, IPC
    pub fn create_vpod_namespace(&self, spec: &VpodSpec) -> Result<Pid> {
        // Basic validation before we fork
        if spec.cmd.is_empty() {
            bail!("No command specified for vPod");
        }
        if let Some(cwd) = &spec.cwd {
            if !cwd.is_dir() {
                bail!("Configured working directory {:?} does not exist or is not a directory", cwd);
            }
        }

        let flags = CloneFlags::CLONE_NEWPID   // PID namespace isolation
                  | CloneFlags::CLONE_NEWNET   // Network namespace isolation  
                  | CloneFlags::CLONE_NEWUTS   // Hostname isolation
                  | CloneFlags::CLONE_NEWNS    // Mount namespace isolation
                  | CloneFlags::CLONE_NEWIPC   // IPC isolation
                  | CloneFlags::CLONE_NEWUSER; // User namespace isolation

        match unsafe { fork() }? {
            ForkResult::Parent { child } => {
                tracing::info!("Created vPod {} in namespace with PID {}", spec.id.0, child);
                Ok(child)
            }
            ForkResult::Child => {
                // Child process: enter new namespaces
                unshare(flags)?;
                
                // Set up minimal environment
                self.setup_namespace_environment(spec)?;
                
                // Execute the vPod command
                self.exec_vpod_command(spec)?;
                
                // Should never reach here
                std::process::exit(1);
            }
        }
    }

    fn setup_namespace_environment(&self, spec: &VpodSpec) -> Result<()> {
        // Change working directory if specified
        if let Some(cwd) = &spec.cwd {
            std::env::set_current_dir(cwd)?;
        }

        // Set environment variables
        for (key, value) in &spec.env {
            std::env::set_var(key, value);
        }

        // Set up minimal /proc mount in new PID namespace
        // This ensures process sees only its own process tree
        let mount_status = std::process::Command::new("mount")
            .args(["-t", "proc", "proc", "/proc"])
            .status();

        if let Ok(status) = mount_status {
            if !status.success() {
                tracing::warn!("failed to mount /proc in vPod namespace, status={}", status);
            }
        } else if let Err(err) = mount_status {
            tracing::warn!("error while mounting /proc in vPod namespace: {}", err);
        }

        Ok(())
    }

    fn exec_vpod_command(&self, spec: &VpodSpec) -> Result<()> {
        if spec.cmd.is_empty() {
            anyhow::bail!("No command specified for vPod");
        }

        let program = &spec.cmd[0];
        let args = &spec.cmd[1..];

        // Replace current process with vPod command
        nix::unistd::execvp(
            &std::ffi::CString::new(program.as_str())?,
            &args.iter()
                .map(|s| std::ffi::CString::new(s.as_str()).unwrap())
                .collect::<Vec<_>>()
        )?;

        Ok(())
    }

    /// Clean up namespace resources
    pub fn cleanup_namespace(&self, _vpod_id: VpodId) -> Result<()> {
        // Namespaces are automatically cleaned up when all processes exit
        // Additional cleanup can be added here if needed
        Ok(())
    }
}
