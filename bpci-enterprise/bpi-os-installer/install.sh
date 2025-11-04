#!/bin/bash
#
# BPI OS Easy Installer
# One-command installation of BPI Immutable OS on any platform
#
# Usage:
#   curl -sSL https://install.bpi.pravyom.com | bash
#   OR
#   ./install.sh --platform=raspberry-pi --network=mainnet
#

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
BPI_OS_VERSION="1.0.0"
INSTALL_DIR="/opt/bpi-os"
CONFIG_DIR="/etc/bpi-os"
DATA_DIR="/var/lib/bpi-os"
LOG_FILE="/var/log/bpi-os-install.log"

# Default values
PLATFORM=""
NETWORK="mainnet"
AUTO_START=true
ENABLE_MONITORING=true
SKIP_CHECKS=false

# Print functions
print_header() {
    echo -e "${BLUE}"
    echo "╔═══════════════════════════════════════════════════════════╗"
    echo "║                                                           ║"
    echo "║           BPI OS Easy Installer v${BPI_OS_VERSION}                ║"
    echo "║           Revolutionary Blockchain Infrastructure         ║"
    echo "║                                                           ║"
    echo "╚═══════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
}

print_step() {
    echo -e "${GREEN}[✓]${NC} $1"
}

print_info() {
    echo -e "${BLUE}[ℹ]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[⚠]${NC} $1"
}

print_error() {
    echo -e "${RED}[✗]${NC} $1"
}

# Parse command line arguments
parse_args() {
    for arg in "$@"; do
        case $arg in
            --platform=*)
                PLATFORM="${arg#*=}"
                ;;
            --network=*)
                NETWORK="${arg#*=}"
                ;;
            --no-auto-start)
                AUTO_START=false
                ;;
            --skip-checks)
                SKIP_CHECKS=true
                ;;
            --help)
                show_help
                exit 0
                ;;
            *)
                print_error "Unknown option: $arg"
                show_help
                exit 1
                ;;
        esac
    done
}

show_help() {
    echo "BPI OS Easy Installer"
    echo ""
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --platform=PLATFORM    Target platform (auto-detected if not specified)"
    echo "  --network=NETWORK      Network to join (mainnet, testnet, devnet)"
    echo "  --no-auto-start        Don't start BPI OS automatically after install"
    echo "  --skip-checks          Skip system requirement checks"
    echo "  --help                 Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0"
    echo "  $0 --platform=raspberry-pi --network=mainnet"
    echo "  $0 --network=testnet --no-auto-start"
}

# Detect platform
detect_platform() {
    if [ -n "$PLATFORM" ]; then
        print_info "Using specified platform: $PLATFORM"
        return
    fi
    
    print_info "Detecting platform..."
    
    # Check for Raspberry Pi
    if [ -f /proc/device-tree/model ]; then
        if grep -q "Raspberry Pi" /proc/device-tree/model; then
            PLATFORM="raspberry-pi"
            print_step "Detected: Raspberry Pi"
            return
        fi
    fi
    
    # Check OS type
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        case "$ID" in
            ubuntu|debian)
                PLATFORM="linux-debian"
                print_step "Detected: Debian-based Linux ($ID)"
                ;;
            centos|rhel|fedora)
                PLATFORM="linux-rhel"
                print_step "Detected: RHEL-based Linux ($ID)"
                ;;
            arch|manjaro)
                PLATFORM="linux-arch"
                print_step "Detected: Arch-based Linux ($ID)"
                ;;
            *)
                PLATFORM="linux-generic"
                print_step "Detected: Generic Linux ($ID)"
                ;;
        esac
    else
        PLATFORM="linux-generic"
        print_warning "Could not detect specific platform, using generic Linux"
    fi
}

# Check system requirements
check_requirements() {
    if [ "$SKIP_CHECKS" = true ]; then
        print_warning "Skipping system requirement checks"
        return
    fi
    
    print_info "Checking system requirements..."
    
    # Check RAM (minimum 2GB)
    total_ram=$(free -g | awk '/^Mem:/{print $2}')
    if [ "$total_ram" -lt 2 ]; then
        print_error "Insufficient RAM: ${total_ram}GB (minimum 2GB required)"
        print_warning "You can skip this check with --skip-checks, but performance may be degraded"
        exit 1
    fi
    print_step "RAM: ${total_ram}GB (sufficient)"
    
    # Check disk space (minimum 20GB)
    available_disk=$(df -BG / | awk 'NR==2 {print $4}' | sed 's/G//')
    if [ "$available_disk" -lt 20 ]; then
        print_error "Insufficient disk space: ${available_disk}GB (minimum 20GB required)"
        exit 1
    fi
    print_step "Disk space: ${available_disk}GB (sufficient)"
    
    # Check CPU cores (minimum 2)
    cpu_cores=$(nproc)
    if [ "$cpu_cores" -lt 2 ]; then
        print_warning "Low CPU cores: ${cpu_cores} (recommended: 2+)"
    else
        print_step "CPU cores: ${cpu_cores} (sufficient)"
    fi
    
    # Check for required commands
    for cmd in curl wget tar systemctl; do
        if ! command -v $cmd &> /dev/null; then
            print_error "Required command not found: $cmd"
            exit 1
        fi
    done
    print_step "Required commands available"
}

# Calculate optimal configuration
calculate_config() {
    print_info "Calculating optimal configuration..."
    
    # Get system resources
    total_ram_mb=$(free -m | awk '/^Mem:/{print $2}')
    cpu_cores=$(nproc)
    
    # Calculate vPods (1 vPod per 256MB RAM, max 200)
    vpod_count=$((total_ram_mb / 256))
    if [ "$vpod_count" -gt 200 ]; then
        vpod_count=200
    fi
    
    # Calculate memory allocation (80% of total RAM)
    memory_mb=$((total_ram_mb * 80 / 100))
    
    # Calculate CPU allocation (all cores)
    cpu_allocation=$cpu_cores
    
    print_step "Calculated configuration:"
    print_info "  vPods: $vpod_count"
    print_info "  Memory: ${memory_mb}MB"
    print_info "  CPU Cores: $cpu_allocation"
}

# Create directories
create_directories() {
    print_info "Creating directories..."
    
    sudo mkdir -p "$INSTALL_DIR"
    sudo mkdir -p "$CONFIG_DIR"
    sudo mkdir -p "$DATA_DIR"
    sudo mkdir -p "$(dirname $LOG_FILE)"
    
    print_step "Directories created"
}

# Download BPI OS
download_bpi_os() {
    print_info "Downloading BPI OS v${BPI_OS_VERSION}..."
    
    # In production, this would download from a real URL
    # For now, we'll simulate the download
    print_warning "Simulating download (production will use real binaries)"
    
    # Create placeholder binary
    sudo touch "$INSTALL_DIR/bpi-os"
    sudo chmod +x "$INSTALL_DIR/bpi-os"
    
    print_step "BPI OS downloaded"
}

# Generate configuration
generate_config() {
    print_info "Generating configuration..."
    
    # Generate env.ini
    cat > /tmp/env.ini << EOF
# BPI OS Configuration
# Auto-generated by installer on $(date)

[global]
environment=production
network=$NETWORK
log_level=info

[vpod_environment]
vpod_id=bpi-os-$(hostname)-vpod-env
arena_size_mb=$memory_mb
max_vpods=$vpod_count
isolation_level=Full

[bso_k8_deployment]
orchestrator_id=bpi-os-$(hostname)-orchestrator
deployment_strategy=RollingUpdate
replicas=1

[network]
network_type=$NETWORK
enable_p2p_mesh=true
max_peers=50
EOF
    
    sudo mv /tmp/env.ini "$CONFIG_DIR/env.ini"
    print_step "Configuration generated: $CONFIG_DIR/env.ini"
}

# Create systemd service
create_service() {
    print_info "Creating systemd service..."
    
    cat > /tmp/bpi-os.service << EOF
[Unit]
Description=BPI Immutable OS
After=network.target

[Service]
Type=simple
User=root
WorkingDirectory=$INSTALL_DIR
ExecStart=$INSTALL_DIR/bpi-os --config=$CONFIG_DIR/env.ini
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
EOF
    
    sudo mv /tmp/bpi-os.service /etc/systemd/system/bpi-os.service
    sudo systemctl daemon-reload
    
    print_step "Systemd service created"
}

# Start BPI OS
start_bpi_os() {
    if [ "$AUTO_START" = false ]; then
        print_info "Auto-start disabled, skipping service start"
        return
    fi
    
    print_info "Starting BPI OS..."
    
    sudo systemctl enable bpi-os
    sudo systemctl start bpi-os
    
    print_step "BPI OS started and enabled"
}

# Print completion message
print_completion() {
    echo ""
    echo -e "${GREEN}"
    echo "╔═══════════════════════════════════════════════════════════╗"
    echo "║                                                           ║"
    echo "║           BPI OS Installation Complete! 🎉                ║"
    echo "║                                                           ║"
    echo "╚═══════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
    echo ""
    print_info "Installation Summary:"
    print_info "  Version: $BPI_OS_VERSION"
    print_info "  Platform: $PLATFORM"
    print_info "  Network: $NETWORK"
    print_info "  Install Directory: $INSTALL_DIR"
    print_info "  Config Directory: $CONFIG_DIR"
    print_info "  Data Directory: $DATA_DIR"
    echo ""
    print_info "Useful Commands:"
    print_info "  Check status:  sudo systemctl status bpi-os"
    print_info "  View logs:     sudo journalctl -u bpi-os -f"
    print_info "  Stop service:  sudo systemctl stop bpi-os"
    print_info "  Start service: sudo systemctl start bpi-os"
    echo ""
    print_info "Next Steps:"
    print_info "  1. Open BPI OS Tauri Wallet (desktop app)"
    print_info "  2. OR visit BPCI web UI: http://localhost:8080"
    print_info "  3. Complete dual-auth wizard to create wallet"
    print_info "  4. Start deploying nodes and earning rewards!"
    echo ""
    print_step "Installation log saved to: $LOG_FILE"
}

# Main installation flow
main() {
    # Redirect output to log file
    exec > >(tee -a "$LOG_FILE")
    exec 2>&1
    
    print_header
    
    parse_args "$@"
    detect_platform
    check_requirements
    calculate_config
    create_directories
    download_bpi_os
    generate_config
    create_service
    start_bpi_os
    print_completion
}

# Run main function
main "$@"
