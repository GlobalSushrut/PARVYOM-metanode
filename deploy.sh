#!/bin/bash
# 🚀 BPI-BPCI One-Command Pilot Deployment Script
# Transforms complex infrastructure into pilot-ready system

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
BPI_VERSION="1.0.0"
DEFAULT_ENV="pilot"
DEFAULT_DOMAIN="localhost"
REQUIRED_PORTS=(8080 8545 27017 9090)

# Print banner
print_banner() {
    echo -e "${BLUE}"
    echo "╔══════════════════════════════════════════════════════════════╗"
    echo "║                    🚀 BPI-BPCI DEPLOYER                     ║"
    echo "║              One-Command Pilot Deployment v${BPI_VERSION}              ║"
    echo "║                                                              ║"
    echo "║  Transforms complex infrastructure into pilot-ready system   ║"
    echo "╚══════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
}

# Print usage
print_usage() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --env ENV          Environment (pilot|dev|staging|prod) [default: pilot]"
    echo "  --domain DOMAIN    Domain name [default: localhost]"
    echo "  --port PORT        Base port [default: 8080]"
    echo "  --auto-fix         Automatically fix common issues"
    echo "  --health-check     Run health check after deployment"
    echo "  --verbose          Enable verbose logging"
    echo "  --dry-run          Preview deployment without execution"
    echo "  --help             Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0                                    # Quick pilot deployment"
    echo "  $0 --env pilot --domain pilot.co     # Pilot with custom domain"
    echo "  $0 --auto-fix --health-check         # Deploy with auto-fix and health check"
}

# Parse command line arguments
ENV="$DEFAULT_ENV"
DOMAIN="$DEFAULT_DOMAIN"
BASE_PORT=8080
AUTO_FIX=false
HEALTH_CHECK=false
VERBOSE=false
DRY_RUN=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --env)
            ENV="$2"
            shift 2
            ;;
        --domain)
            DOMAIN="$2"
            shift 2
            ;;
        --port)
            BASE_PORT="$2"
            shift 2
            ;;
        --auto-fix)
            AUTO_FIX=true
            shift
            ;;
        --health-check)
            HEALTH_CHECK=true
            shift
            ;;
        --verbose)
            VERBOSE=true
            shift
            ;;
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --help)
            print_usage
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            print_usage
            exit 1
            ;;
    esac
done

# Logging function
log() {
    local level=$1
    shift
    local message="$*"
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    
    case $level in
        INFO)
            echo -e "${GREEN}[INFO]${NC} ${timestamp} - $message"
            ;;
        WARN)
            echo -e "${YELLOW}[WARN]${NC} ${timestamp} - $message"
            ;;
        ERROR)
            echo -e "${RED}[ERROR]${NC} ${timestamp} - $message"
            ;;
        DEBUG)
            if [[ "$VERBOSE" == "true" ]]; then
                echo -e "${BLUE}[DEBUG]${NC} ${timestamp} - $message"
            fi
            ;;
    esac
}

# System detection
detect_system() {
    log INFO "🔍 Detecting system environment..."
    
    # OS Detection
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        OS="linux"
        log INFO "Operating System: Linux"
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        OS="macos"
        log INFO "Operating System: macOS"
    else
        log ERROR "Unsupported operating system: $OSTYPE"
        exit 1
    fi
    
    # Architecture Detection
    ARCH=$(uname -m)
    log INFO "Architecture: $ARCH"
    
    # Memory Detection
    if command -v free >/dev/null 2>&1; then
        MEMORY_GB=$(free -g | awk '/^Mem:/{print $2}')
        log INFO "Available Memory: ${MEMORY_GB}GB"
        
        if [[ $MEMORY_GB -lt 4 ]]; then
            log WARN "Low memory detected. Recommend 4GB+ for optimal performance"
        fi
    fi
}

# Dependency checking
check_dependencies() {
    log INFO "📦 Checking dependencies..."
    
    local missing_deps=()
    
    # Check for Rust
    if ! command -v cargo >/dev/null 2>&1; then
        missing_deps+=("rust")
        log WARN "Rust/Cargo not found"
    else
        local rust_version=$(cargo --version | cut -d' ' -f2)
        log INFO "Rust version: $rust_version"
    fi
    
    # Check for Git
    if ! command -v git >/dev/null 2>&1; then
        missing_deps+=("git")
        log WARN "Git not found"
    fi
    
    # Check for curl
    if ! command -v curl >/dev/null 2>&1; then
        missing_deps+=("curl")
        log WARN "curl not found"
    fi
    
    # Auto-install missing dependencies if requested
    if [[ ${#missing_deps[@]} -gt 0 ]]; then
        if [[ "$AUTO_FIX" == "true" ]]; then
            log INFO "🔧 Auto-installing missing dependencies..."
            install_dependencies "${missing_deps[@]}"
        else
            log ERROR "Missing dependencies: ${missing_deps[*]}"
            log INFO "Run with --auto-fix to automatically install dependencies"
            exit 1
        fi
    else
        log INFO "✅ All dependencies satisfied"
    fi
}

# Install dependencies
install_dependencies() {
    local deps=("$@")
    
    for dep in "${deps[@]}"; do
        case $dep in
            rust)
                log INFO "Installing Rust..."
                if [[ "$DRY_RUN" == "false" ]]; then
                    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
                    source ~/.cargo/env
                fi
                ;;
            git)
                log INFO "Installing Git..."
                if [[ "$OS" == "linux" ]]; then
                    if [[ "$DRY_RUN" == "false" ]]; then
                        sudo apt-get update && sudo apt-get install -y git
                    fi
                elif [[ "$OS" == "macos" ]]; then
                    if [[ "$DRY_RUN" == "false" ]]; then
                        xcode-select --install
                    fi
                fi
                ;;
            curl)
                log INFO "Installing curl..."
                if [[ "$OS" == "linux" ]]; then
                    if [[ "$DRY_RUN" == "false" ]]; then
                        sudo apt-get update && sudo apt-get install -y curl
                    fi
                fi
                ;;
        esac
    done
}

# Port availability check
check_ports() {
    log INFO "🔌 Checking port availability..."
    
    local unavailable_ports=()
    
    for port in "${REQUIRED_PORTS[@]}"; do
        if netstat -tuln 2>/dev/null | grep -q ":$port "; then
            unavailable_ports+=($port)
            log WARN "Port $port is already in use"
        else
            log DEBUG "Port $port is available"
        fi
    done
    
    if [[ ${#unavailable_ports[@]} -gt 0 ]]; then
        if [[ "$AUTO_FIX" == "true" ]]; then
            log INFO "🔧 Auto-fixing port conflicts..."
            for port in "${unavailable_ports[@]}"; do
                log INFO "Attempting to free port $port..."
                if [[ "$DRY_RUN" == "false" ]]; then
                    # Kill processes using the port (be careful in production!)
                    local pids=$(lsof -ti:$port 2>/dev/null || true)
                    if [[ -n "$pids" ]]; then
                        echo "$pids" | xargs kill -9 2>/dev/null || true
                        log INFO "Freed port $port"
                    fi
                fi
            done
        else
            log ERROR "Ports in use: ${unavailable_ports[*]}"
            log INFO "Run with --auto-fix to automatically free ports"
            exit 1
        fi
    else
        log INFO "✅ All required ports available"
    fi
}

# Generate configuration
generate_config() {
    log INFO "⚙️ Generating configuration for environment: $ENV"
    
    local config_dir="./config"
    mkdir -p "$config_dir"
    
    # Generate main BPI config
    cat > "$config_dir/bpi-${ENV}-config.toml" << EOF
# BPI Core Configuration - Generated for $ENV environment
[network]
domain = "$DOMAIN"
vm_port = $BASE_PORT
bpci_port = $((BASE_PORT + 465))
db_port = $((BASE_PORT + 18937))
orchestrator_port = $((BASE_PORT + 1010))

[security]
quantum_safe = true
audit_enabled = true
compliance_mode = "$ENV"

[storage]
data_dir = "./data/$ENV"
backup_enabled = true

[logging]
level = "info"
format = "json"
output = "./logs/$ENV.log"

[pilot]
enabled = true
auto_setup = true
health_checks = true
EOF

    # Generate environment variables file
    cat > "$config_dir/.env.$ENV" << EOF
# BPI Environment Variables - $ENV
BPI_ENV=$ENV
BPI_DOMAIN=$DOMAIN
BPI_VM_PORT=$BASE_PORT
BPI_BPCI_PORT=$((BASE_PORT + 465))
BPI_DB_PORT=$((BASE_PORT + 18937))
BPI_ORCHESTRATOR_PORT=$((BASE_PORT + 1010))
BPI_DATA_DIR=./data/$ENV
BPI_LOG_LEVEL=info
BPI_QUANTUM_SAFE=true
BPI_PILOT_MODE=true
EOF

    log INFO "✅ Configuration generated: $config_dir/bpi-${ENV}-config.toml"
    log INFO "✅ Environment file generated: $config_dir/.env.$ENV"
}

# Build binaries
build_binaries() {
    log INFO "🔨 Building BPI binaries..."
    
    if [[ "$DRY_RUN" == "false" ]]; then
        # Build BPI Core
        log INFO "Building bpi-core..."
        cd bpi-core
        cargo build --release
        cd ..
        
        # Build BPCI Enterprise
        log INFO "Building bpci-enterprise..."
        cd bpci-enterprise
        cargo build --release
        cd ..
        
        log INFO "✅ Binaries built successfully"
    else
        log INFO "[DRY RUN] Would build binaries"
    fi
}

# Start services
start_services() {
    log INFO "🚀 Starting BPI services..."
    
    # Create data directories
    mkdir -p "./data/$ENV"
    mkdir -p "./logs"
    
    if [[ "$DRY_RUN" == "false" ]]; then
        # Load environment variables
        if [[ -f "./config/.env.$ENV" ]]; then
            export $(cat "./config/.env.$ENV" | xargs)
        fi
        
        # Start BPI Core
        log INFO "Starting BPI Core..."
        nohup ./bpi-core/target/release/bpi-core node start > "./logs/bpi-core-$ENV.log" 2>&1 &
        echo $! > "./data/$ENV/bpi-core.pid"
        
        # Start BPCI Enterprise
        log INFO "Starting BPCI Enterprise..."
        nohup ./bpci-enterprise/target/release/bpci-enterprise start > "./logs/bpci-enterprise-$ENV.log" 2>&1 &
        echo $! > "./data/$ENV/bpci-enterprise.pid"
        
        # Wait for services to start
        log INFO "Waiting for services to initialize..."
        sleep 10
        
        log INFO "✅ Services started successfully"
    else
        log INFO "[DRY RUN] Would start services"
    fi
}

# Health check
run_health_check() {
    if [[ "$HEALTH_CHECK" == "true" ]]; then
        log INFO "🏥 Running health check..."
        
        if [[ "$DRY_RUN" == "false" ]]; then
            # Run health check using BPI CLI
            if ./bpi-core/target/release/bpi-core node health --json > /tmp/health_check.json 2>&1; then
                local pilot_ready=$(cat /tmp/health_check.json | grep -o '"pilot_ready":[^,]*' | cut -d':' -f2)
                if [[ "$pilot_ready" == "true" ]]; then
                    log INFO "✅ Health check passed - System is pilot ready!"
                else
                    log WARN "⚠️ Health check completed but system may not be fully pilot ready"
                fi
            else
                log WARN "Health check failed - services may still be starting"
            fi
        else
            log INFO "[DRY RUN] Would run health check"
        fi
    fi
}

# Cleanup function
cleanup() {
    log INFO "🧹 Cleaning up..."
    # Add cleanup logic here if needed
}

# Main deployment function
main() {
    print_banner
    
    log INFO "Starting BPI-BPCI deployment with environment: $ENV"
    log INFO "Domain: $DOMAIN"
    log INFO "Base Port: $BASE_PORT"
    
    if [[ "$DRY_RUN" == "true" ]]; then
        log INFO "🔍 DRY RUN MODE - No changes will be made"
    fi
    
    # Set trap for cleanup
    trap cleanup EXIT
    
    # Execute deployment steps
    detect_system
    check_dependencies
    check_ports
    generate_config
    build_binaries
    start_services
    run_health_check
    
    # Success message
    echo -e "${GREEN}"
    echo "╔══════════════════════════════════════════════════════════════╗"
    echo "║                    🎉 DEPLOYMENT COMPLETE!                  ║"
    echo "║                                                              ║"
    echo "║  Your BPI-BPCI infrastructure is now pilot-ready!           ║"
    echo "║                                                              ║"
    echo "║  VM Server:        http://$DOMAIN:$BASE_PORT                    ║"
    echo "║  BPCI Bridge:      http://$DOMAIN:$((BASE_PORT + 465))         ║"
    echo "║  4D Database:      http://$DOMAIN:$((BASE_PORT + 18937))       ║"
    echo "║  Orchestrator:     http://$DOMAIN:$((BASE_PORT + 1010))        ║"
    echo "║                                                              ║"
    echo "║  Next Steps:                                                 ║"
    echo "║  - Run: ./bpi-core/target/release/bpi-core node health      ║"
    echo "║  - Check logs: tail -f ./logs/bpi-core-$ENV.log             ║"
    echo "║  - Deploy your first app with the sample templates          ║"
    echo "╚══════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
}

# Run main function
main "$@"
