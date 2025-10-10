#!/bin/bash

# BPCI Enterprise System Stop Script
# Gracefully stop all system components

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
BPCI_ROOT="/home/umesh/metanode/bpci-enterprise"
PID_FILE="$BPCI_ROOT/system.pid"

echo -e "${BLUE}🛑 BPCI Enterprise System Shutdown${NC}"
echo -e "${BLUE}==================================${NC}"

# Function to log messages
log() {
    echo -e "$1"
}

# Function to stop process by PID
stop_process() {
    local pid=$1
    local service_name=$2
    
    if [ -n "$pid" ] && kill -0 "$pid" 2>/dev/null; then
        log "${YELLOW}Stopping $service_name (PID: $pid)...${NC}"
        kill -TERM "$pid" 2>/dev/null || true
        
        # Wait for graceful shutdown
        local count=0
        while [ $count -lt 10 ] && kill -0 "$pid" 2>/dev/null; do
            sleep 1
            count=$((count + 1))
        done
        
        # Force kill if still running
        if kill -0 "$pid" 2>/dev/null; then
            log "${RED}Force killing $service_name...${NC}"
            kill -KILL "$pid" 2>/dev/null || true
        fi
        
        log "${GREEN}✅ $service_name stopped${NC}"
    else
        log "${YELLOW}⚠️  $service_name not running${NC}"
    fi
}

# Stop all processes
if [ -f "$PID_FILE" ]; then
    log "${BLUE}📋 Reading process IDs...${NC}"
    
    local pids=($(cat "$PID_FILE" | grep -v '^$'))
    local services=("BPCI Server" "Wallet Server" "Admin Dashboard" "Website")
    
    if [ ${#pids[@]} -gt 0 ]; then
        log "${YELLOW}Stopping ${#pids[@]} services...${NC}"
        
        for i in "${!pids[@]}"; do
            local service_name="${services[$i]:-Service $((i+1))}"
            stop_process "${pids[$i]}" "$service_name"
        done
        
        # Clear PID file
        echo "" > "$PID_FILE"
        
        log "${GREEN}🎯 All services stopped successfully${NC}"
    else
        log "${YELLOW}⚠️  No running services found${NC}"
    fi
else
    log "${YELLOW}⚠️  PID file not found, attempting to stop by port...${NC}"
    
    # Stop by port if PID file doesn't exist
    local ports=(3000 8888 9999 7778)
    local service_names=("Website" "Admin Dashboard" "BPCI Server" "Wallet Server")
    
    for i in "${!ports[@]}"; do
        local port="${ports[$i]}"
        local service_name="${service_names[$i]}"
        local pid=$(lsof -ti:$port 2>/dev/null || true)
        
        if [ -n "$pid" ]; then
            stop_process "$pid" "$service_name"
        fi
    done
fi

log "${GREEN}✅ BPCI Enterprise System shutdown complete${NC}"
