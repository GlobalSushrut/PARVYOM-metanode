#!/bin/bash
# BPCI Server Management Script
# Version: 1.0.0
# Purpose: Manage BPCI server operations and monitoring

set -euo pipefail

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

show_menu() {
    echo -e "${CYAN}🌐 BPCI Server Management${NC}"
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    echo "1) Server Status       - Check all services"
    echo "2) Start Services      - Start BPCI server"
    echo "3) Stop Services       - Stop BPCI server"
    echo "4) Restart Services    - Restart BPCI server"
    echo "5) View Logs          - Real-time server logs"
    echo "6) System Health      - Resource monitoring"
    echo "7) Registry Stats     - Node/identity statistics"
    echo "8) Network Status     - P2P network health"
    echo "9) Backup Data        - Create system backup"
    echo "0) Exit"
    echo ""
}

server_status() {
    echo -e "${BLUE}📊 BPCI Server Status${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    
    services=("bpci-server" "bpci-api" "nginx" "postgresql" "redis-server")
    
    for service in "${services[@]}"; do
        if systemctl is-active --quiet "$service"; then
            echo -e "• ${service}: ${GREEN}✅ Running${NC}"
        else
            echo -e "• ${service}: ${RED}❌ Stopped${NC}"
        fi
    done
    
    echo ""
    echo -e "${BLUE}🌐 Endpoints:${NC}"
    echo "• API: http://localhost/api/"
    echo "• RPC: http://localhost/rpc"
    echo "• Dashboard: http://localhost/dashboard/"
}

system_health() {
    echo -e "${BLUE}💊 System Health${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    
    # Memory usage
    MEMORY_USED=$(free | awk '/^Mem:/{printf "%.1f", $3/$2 * 100.0}')
    echo -e "• Memory: ${MEMORY_USED}% used"
    
    # Disk usage
    DISK_USED=$(df / | awk 'NR==2{print $5}')
    echo -e "• Disk: ${DISK_USED} used"
    
    # Load average
    LOAD=$(uptime | awk -F'load average:' '{print $2}')
    echo -e "• Load:${LOAD}"
    
    # Network connections
    CONNECTIONS=$(ss -tuln | wc -l)
    echo -e "• Network connections: ${CONNECTIONS}"
}

registry_stats() {
    echo -e "${BLUE}📋 Registry Statistics${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    
    if command -v bpci-server &> /dev/null; then
        bpci-server registry list-nodes --json | jq -r '.summary' 2>/dev/null || echo "Registry service starting..."
    else
        echo "BPCI server not installed"
    fi
}

view_logs() {
    echo -e "${BLUE}📝 BPCI Server Logs${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo "Press Ctrl+C to exit log view"
    echo ""
    journalctl -u bpci-server -f --no-pager
}

backup_data() {
    echo -e "${BLUE}💾 Creating BPCI Server Backup${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    
    BACKUP_FILE="/var/backups/bpci-server/backup-$(date +%Y%m%d-%H%M%S).tar.gz"
    mkdir -p "$(dirname "$BACKUP_FILE")"
    
    tar -czf "$BACKUP_FILE" \
        "$INSTALL_DIR/data" \
        "$INSTALL_DIR/config" \
        "$CONFIG_DIR" \
        2>/dev/null
    
    echo -e "${GREEN}✅ Backup created: $BACKUP_FILE${NC}"
}

# Main menu loop
while true; do
    show_menu
    read -p "Enter your choice (0-9): " choice
    echo ""
    
    case $choice in
        1) server_status ;;
        2) systemctl start bpci-server bpci-api nginx && echo -e "${GREEN}✅ Services started${NC}" ;;
        3) systemctl stop bpci-server bpci-api && echo -e "${YELLOW}⏹️ Services stopped${NC}" ;;
        4) systemctl restart bpci-server bpci-api nginx && echo -e "${GREEN}🔄 Services restarted${NC}" ;;
        5) view_logs ;;
        6) system_health ;;
        7) registry_stats ;;
        8) bpci-server network status 2>/dev/null || echo "Network service starting..." ;;
        9) backup_data ;;
        0) echo -e "${GREEN}👋 Goodbye!${NC}"; exit 0 ;;
        *) echo -e "${RED}❌ Invalid choice${NC}" ;;
    esac
    
    echo ""
    read -p "Press Enter to continue..."
    clear
done
