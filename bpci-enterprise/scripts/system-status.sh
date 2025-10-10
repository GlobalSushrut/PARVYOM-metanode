#!/bin/bash

# BPCI Enterprise System Status Script
# Check status of all system components

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}📊 BPCI Enterprise System Status${NC}"
echo -e "${BLUE}================================${NC}"

# Function to check service health
check_service() {
    local url=$1
    local service_name=$2
    local port=$3
    
    if curl -s "$url" >/dev/null 2>&1; then
        echo -e "${GREEN}✅ $service_name: ONLINE (Port $port)${NC}"
        return 0
    else
        echo -e "${RED}❌ $service_name: OFFLINE (Port $port)${NC}"
        return 1
    fi
}

# Function to get service details
get_service_details() {
    local url=$1
    local service_name=$2
    
    local response=$(curl -s "$url" 2>/dev/null || echo "")
    if [ -n "$response" ]; then
        echo -e "${BLUE}   📋 $service_name Details:${NC}"
        echo "$response" | jq -r '. | "      Status: \(.status // "unknown"), Uptime: \(.uptime // "unknown")s, Demo: \(.demo_mode // false)"' 2>/dev/null || echo "      Response received"
    fi
}

echo -e "${YELLOW}Checking system components...${NC}"
echo ""

# Check each component
healthy_count=0
total_count=4

# BPCI Server
if check_service "http://localhost:9999/health" "BPCI Server" "9999"; then
    get_service_details "http://localhost:9999/health" "BPCI Server"
    healthy_count=$((healthy_count + 1))
fi
echo ""

# Wallet Server
if check_service "http://localhost:7778/health" "Wallet Server" "7778"; then
    get_service_details "http://localhost:7778/health" "Wallet Server"
    healthy_count=$((healthy_count + 1))
fi
echo ""

# Admin Dashboard
if check_service "http://localhost:8888/health" "Admin Dashboard" "8888"; then
    get_service_details "http://localhost:8888/health" "Admin Dashboard"
    healthy_count=$((healthy_count + 1))
fi
echo ""

# Website
if check_service "http://localhost:3000" "Website" "3000"; then
    echo -e "${BLUE}   📋 Website Details: Next.js application${NC}"
    healthy_count=$((healthy_count + 1))
fi
echo ""

# Overall system status
echo -e "${BLUE}📊 Overall System Health${NC}"
echo -e "${BLUE}========================${NC}"
echo -e "Services Online: $healthy_count/$total_count"

if [ $healthy_count -eq $total_count ]; then
    echo -e "${GREEN}🎉 ALL SYSTEMS OPERATIONAL!${NC}"
    echo -e "${GREEN}🚀 System is 100% ready for production!${NC}"
elif [ $healthy_count -gt 0 ]; then
    echo -e "${YELLOW}⚠️  Partial system operation${NC}"
    echo -e "${YELLOW}Some services may need attention${NC}"
else
    echo -e "${RED}❌ SYSTEM DOWN${NC}"
    echo -e "${RED}All services are offline${NC}"
fi

echo ""
echo -e "${BLUE}🔗 Access Points:${NC}"
echo -e "   🌐 Website: http://localhost:3000"
echo -e "   📊 Admin Dashboard: http://localhost:8888"
echo -e "   🔗 BPCI Server: http://localhost:9999"
echo -e "   💰 Wallet Server: http://localhost:7778"

echo ""
echo -e "${BLUE}🔐 Demo Credentials:${NC}"
echo -e "   Username: root"
echo -e "   Password: admin"
