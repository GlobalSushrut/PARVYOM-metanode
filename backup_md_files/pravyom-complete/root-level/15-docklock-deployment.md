# 🔒 DockLock Deployment - Secure Container Mastery

**Master DockLock**: Learn to deploy applications in secure, deterministic containers with complete audit trails and military-grade security.

---

## 🎯 **What You'll Learn**

- Deploy complex applications in DockLock containers
- Configure security policies and syscall filtering
- Set up deterministic execution environments
- Implement witness recording and audit trails
- Connect multiple containers in secure networks

---

## 🏗️ **DockLock Architecture Overview**

```
┌─────────────────────────────────────────────────────────────┐
│                    DOCKLOCK CONTAINER                       │
├─────────────────────────────────────────────────────────────┤
│  Application Code                                           │
│  ├── Your App (Python, Node.js, Rust, etc.)               │
│  ├── Dependencies and Libraries                            │
│  └── Configuration Files                                   │
├─────────────────────────────────────────────────────────────┤
│  DockLock Security Layer                                   │
│  ├── Syscall Filtering (seccomp)                          │
│  ├── Deterministic Execution                              │
│  ├── RNG Seed Injection                                   │
│  ├── I/O Witness Recording                                │
│  └── Resource Constraints                                 │
├─────────────────────────────────────────────────────────────┤
│  Audit & Monitoring                                        │
│  ├── Step Receipts Generation                             │
│  ├── Cryptographic Signatures                             │
│  ├── Performance Metrics                                  │
│  └── Security Event Logging                               │
└─────────────────────────────────────────────────────────────┘
```

---

## 🚀 **Basic DockLock Deployment**

### **Step 1: Prepare Your Application**

```bash
# Create a more complex application
mkdir advanced-bpci-app
cd advanced-bpci-app

# Create a Node.js application with database
cat > app.js << 'EOF'
const express = require('express');
const sqlite3 = require('sqlite3').verbose();
const crypto = require('crypto');

const app = express();
app.use(express.json());

// Initialize SQLite database
const db = new sqlite3.Database(':memory:');
db.serialize(() => {
  db.run("CREATE TABLE transactions (id TEXT, data TEXT, timestamp INTEGER)");
});

// API endpoints
app.post('/transaction', (req, res) => {
  const id = crypto.randomUUID();
  const timestamp = Date.now();
  const data = JSON.stringify(req.body);
  
  db.run("INSERT INTO transactions VALUES (?, ?, ?)", [id, data, timestamp], function(err) {
    if (err) {
      res.status(500).json({ error: err.message });
      return;
    }
    res.json({ 
      id, 
      timestamp, 
      message: "Transaction recorded in DockLock",
      auditable: true 
    });
  });
});

app.get('/transactions', (req, res) => {
  db.all("SELECT * FROM transactions ORDER BY timestamp DESC", [], (err, rows) => {
    if (err) {
      res.status(500).json({ error: err.message });
      return;
    }
    res.json({ transactions: rows, auditable: true });
  });
});

app.listen(3000, () => {
  console.log('Advanced BPCI app running on port 3000');
});
EOF

# Package.json
cat > package.json << 'EOF'
{
  "name": "advanced-bpci-app",
  "version": "1.0.0",
  "main": "app.js",
  "dependencies": {
    "express": "^4.18.2",
    "sqlite3": "^5.1.6"
  }
}
EOF

# Dockerfile with DockLock optimization
cat > Dockerfile << 'EOF'
FROM node:18-alpine
WORKDIR /app
COPY package.json .
RUN npm install
COPY app.js .
EXPOSE 3000
CMD ["node", "app.js"]
EOF
```

### **Step 2: Configure DockLock Security Policy**

```bash
# Create DockLock configuration
cat > docklock-config.yaml << 'EOF'
apiVersion: docklock.bpci.org/v1
kind: ContainerPolicy
metadata:
  name: advanced-app-policy
spec:
  security:
    # Syscall filtering for enhanced security
    seccomp:
      defaultAction: SCMP_ACT_ERRNO
      allowedSyscalls:
        - read
        - write
        - open
        - close
        - socket
        - bind
        - listen
        - accept
        - connect
        - sendto
        - recvfrom
        - mmap
        - munmap
        - brk
        - rt_sigaction
        - rt_sigprocmask
        - clone
        - execve
        - exit_group
        - futex
        - getpid
        - getuid
        - getgid
        - access
        - stat
        - fstat
        - lstat
        - poll
        - lseek
        - mprotect
        - rt_sigreturn
        - ioctl
        - pread64
        - pwrite64
        - readv
        - writev
        - pipe
        - select
        - sched_yield
        - mremap
        - msync
        - mincore
        - madvise
        - shmget
        - shmat
        - shmctl
        - dup
        - dup2
        - pause
        - nanosleep
        - getitimer
        - alarm
        - setitimer
        - getpgrp
        - setsid
        - getpgid
        - setpgid
        - getsid
        - sethostname
        - setrlimit
        - getrlimit
        - getrusage
        - gettimeofday
        - times
        - ptrace
        - getuid
        - syslog
        - getgid
        - setuid
        - setgid
        - geteuid
        - getegid
        - setpgid
        - getppid
        - getpgrp
        - setsid
        - setreuid
        - setregid
        - getgroups
        - setgroups
        - setresuid
        - getresuid
        - setresgid
        - getresgid
        - getpgid
        - setfsuid
        - setfsgid
        - getsid
        - capget
        - capset
        - rt_sigpending
        - rt_sigtimedwait
        - rt_sigqueueinfo
        - rt_sigsuspend
        - sigaltstack
        - utime
        - mknod
        - uselib
        - personality
        - ustat
        - statfs
        - fstatfs
        - sysfs
        - getpriority
        - setpriority
        - sched_setparam
        - sched_getparam
        - sched_setscheduler
        - sched_getscheduler
        - sched_get_priority_max
        - sched_get_priority_min
        - sched_rr_get_interval
        - mlock
        - munlock
        - mlockall
        - munlockall
        - vhangup
        - modify_ldt
        - pivot_root
        - _sysctl
        - prctl
        - arch_prctl
        - adjtimex
        - setrlimit
        - chroot
        - sync
        - acct
        - settimeofday
        - mount
        - umount2
        - swapon
        - swapoff
        - reboot
        - sethostname
        - setdomainname
        - iopl
        - ioperm
        - create_module
        - init_module
        - delete_module
        - get_kernel_syms
        - query_module
        - quotactl
        - nfsservctl
        - getpmsg
        - putpmsg
        - afs_syscall
        - tuxcall
        - security
        - gettid
        - readahead
        - setxattr
        - lsetxattr
        - fsetxattr
        - getxattr
        - lgetxattr
        - fgetxattr
        - listxattr
        - llistxattr
        - flistxattr
        - removexattr
        - lremovexattr
        - fremovexattr
        - tkill
        - time
        - futex
        - sched_setaffinity
        - sched_getaffinity
        - set_thread_area
        - io_setup
        - io_destroy
        - io_getevents
        - io_submit
        - io_cancel
        - get_thread_area
        - lookup_dcookie
        - epoll_create
        - epoll_ctl_old
        - epoll_wait_old
        - remap_file_pages
        - getdents64
        - set_tid_address
        - restart_syscall
        - semtimedop
        - fadvise64
        - timer_create
        - timer_settime
        - timer_gettime
        - timer_getoverrun
        - timer_delete
        - clock_settime
        - clock_gettime
        - clock_getres
        - clock_nanosleep
        - exit_group
        - epoll_wait
        - epoll_ctl
        - tgkill
        - utimes
        - vserver
        - mbind
        - set_mempolicy
        - get_mempolicy
        - mq_open
        - mq_unlink
        - mq_timedsend
        - mq_timedreceive
        - mq_notify
        - mq_getsetattr
        - kexec_load
        - waitid
        - add_key
        - request_key
        - keyctl
        - ioprio_set
        - ioprio_get
        - inotify_init
        - inotify_add_watch
        - inotify_rm_watch
        - migrate_pages
        - openat
        - mkdirat
        - mknodat
        - fchownat
        - futimesat
        - newfstatat
        - unlinkat
        - renameat
        - linkat
        - symlinkat
        - readlinkat
        - fchmodat
        - faccessat
        - pselect6
        - ppoll
        - unshare
        - set_robust_list
        - get_robust_list
        - splice
        - tee
        - sync_file_range
        - vmsplice
        - move_pages
        - utimensat
        - epoll_pwait
        - signalfd
        - timerfd_create
        - eventfd
        - fallocate
        - timerfd_settime
        - timerfd_gettime
        - accept4
        - signalfd4
        - eventfd2
        - epoll_create1
        - dup3
        - pipe2
        - inotify_init1
        - preadv
        - pwritev
        - rt_tgsigqueueinfo
        - perf_event_open
        - recvmmsg
        - fanotify_init
        - fanotify_mark
        - prlimit64
        - name_to_handle_at
        - open_by_handle_at
        - clock_adjtime
        - syncfs
        - sendmmsg
        - setns
        - getcpu
        - process_vm_readv
        - process_vm_writev
        - kcmp
        - finit_module
        - sched_setattr
        - sched_getattr
        - renameat2
        - seccomp
        - getrandom
        - memfd_create
        - kexec_file_load
        - bpf
        - execveat
        - userfaultfd
        - membarrier
        - mlock2
        - copy_file_range
        - preadv2
        - pwritev2
        - pkey_mprotect
        - pkey_alloc
        - pkey_free
        - statx
        - io_pgetevents
        - rseq
        - pidfd_send_signal
        - io_uring_setup
        - io_uring_enter
        - io_uring_register
        - open_tree
        - move_mount
        - fsopen
        - fsconfig
        - fsmount
        - fspick
        - pidfd_open
        - clone3
        - close_range
        - openat2
        - pidfd_getfd
        - faccessat2
        - process_madvise
        - epoll_pwait2
        - mount_setattr
        - quotactl_fd
        - landlock_create_ruleset
        - landlock_add_rule
        - landlock_restrict_self
        - memfd_secret
        - process_mrelease
        - futex_waitv
        - set_mempolicy_home_node

    # Resource constraints
    resources:
      memory: "512Mi"
      cpu: "500m"
      disk: "1Gi"
      
    # Network isolation
    network:
      mode: "bridge"
      allowedPorts: [3000]
      
  # Deterministic execution
  deterministic:
    enabled: true
    rngSeed: "bpci-deterministic-seed-12345"
    
  # Audit configuration
  audit:
    enabled: true
    witnessRecording: true
    stepReceipts: true
    cryptographicSigning: true
    
  # I/O monitoring
  monitoring:
    fileAccess: true
    networkTraffic: true
    systemCalls: true
    performanceMetrics: true
EOF
```

### **Step 3: Deploy with Advanced Configuration**

```bash
# Build the application
docker build -t advanced-bpci-app .

# Deploy with DockLock security policy
bpci docklock deploy \
  --image advanced-bpci-app \
  --name advanced-app \
  --config docklock-config.yaml \
  --port 3000 \
  --security-level maximum \
  --audit-enabled \
  --deterministic \
  --witness-recording \
  --enc-cluster-connect \
  --bpi-ledger-connect

# Check deployment status
bpci docklock status advanced-app
```

---

## 🔍 **Advanced DockLock Features**

### **Multi-Container Deployment**

```bash
# Deploy a microservices architecture
cat > microservices-config.yaml << 'EOF'
apiVersion: docklock.bpci.org/v1
kind: MultiContainerDeployment
metadata:
  name: microservices-stack
spec:
  containers:
    - name: frontend
      image: nginx:alpine
      ports: [80]
      security: standard
      
    - name: api
      image: advanced-bpci-app
      ports: [3000]
      security: maximum
      
    - name: database
      image: postgres:15
      ports: [5432]
      security: maximum
      environment:
        POSTGRES_DB: bpci_app
        POSTGRES_USER: app_user
        POSTGRES_PASSWORD: secure_password
        
  network:
    mode: "isolated"
    allowedConnections:
      - from: frontend
        to: api
        ports: [3000]
      - from: api
        to: database
        ports: [5432]
        
  audit:
    enabled: true
    crossContainerTracking: true
EOF

# Deploy the microservices stack
bpci docklock deploy-stack microservices-config.yaml
```

### **Security Policy Templates**

```bash
# Use predefined security templates
bpci docklock deploy \
  --image my-app \
  --security-template banking \
  --compliance-level sox

# Available templates:
# - standard: Basic security for general applications
# - banking: Enhanced security for financial applications
# - government: Maximum security for government applications
# - healthcare: HIPAA-compliant security for healthcare
# - defense: Military-grade security for defense applications
```

### **Real-time Monitoring**

```bash
# Monitor container in real-time
bpci docklock monitor advanced-app

# View security events
bpci docklock security-events advanced-app

# Check compliance status
bpci docklock compliance-check advanced-app

# View performance metrics
bpci docklock metrics advanced-app
```

---

## 🔐 **Security Features Deep Dive**

### **Syscall Filtering**
DockLock implements comprehensive syscall filtering to prevent malicious operations:

```bash
# View allowed syscalls for a container
bpci docklock syscalls advanced-app

# Add custom syscall rules
bpci docklock add-syscall-rule advanced-app \
  --syscall openat \
  --action allow \
  --condition "path=/app/data/*"
```

### **Deterministic Execution**
Ensures reproducible behavior across deployments:

```bash
# Enable deterministic mode
bpci docklock set-deterministic advanced-app \
  --rng-seed "custom-seed-123" \
  --time-freeze "2024-01-01T00:00:00Z"

# Verify deterministic behavior
bpci docklock verify-deterministic advanced-app
```

### **Witness Recording**
Records all I/O operations for complete audit trails:

```bash
# View witness records
bpci docklock witness-records advanced-app

# Export witness data
bpci docklock export-witness advanced-app \
  --format json \
  --output witness-data.json
```

---

## 📊 **Monitoring and Debugging**

### **Container Logs**
```bash
# View container logs
bpci docklock logs advanced-app

# Stream logs in real-time
bpci docklock logs advanced-app --follow

# Filter logs by security events
bpci docklock logs advanced-app --filter security
```

### **Performance Analysis**
```bash
# Get performance metrics
bpci docklock metrics advanced-app

# Generate performance report
bpci docklock performance-report advanced-app \
  --duration 24h \
  --format pdf
```

### **Security Audit**
```bash
# Run security audit
bpci docklock security-audit advanced-app

# Generate compliance report
bpci docklock compliance-report advanced-app \
  --standard sox,pci-dss,hipaa
```

---

## 🚀 **Next Steps**

Now that you've mastered DockLock deployment:

1. **[ENC Cluster Integration](16-enc-cluster-integration.md)** - Connect to audit and notary systems
2. **[BPI Ledger Connection](17-bpi-ledger-connection.md)** - Integrate with blockchain
3. **[BPCI Enterprise Integration](18-bpci-ledger-integration.md)** - Connect to governance
4. **[CUE/Sruti Smart Contracts](19-cue-sruti-introduction.md)** - Add smart contract orchestration

---

## 🆘 **Troubleshooting**

### **Common Issues**

**Container Won't Start**
```bash
bpci docklock debug advanced-app
bpci docklock logs advanced-app --debug
```

**Security Policy Violations**
```bash
bpci docklock security-violations advanced-app
bpci docklock adjust-policy advanced-app --interactive
```

**Performance Issues**
```bash
bpci docklock performance-analysis advanced-app
bpci docklock optimize advanced-app
```

---

**🎉 Congratulations! You've mastered DockLock secure container deployment!**

*Continue with [ENC Cluster Integration](16-enc-cluster-integration.md) to add audit trails and notary services.*
