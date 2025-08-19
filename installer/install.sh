#!/bin/bash
# BPCI Universal Installer - Three-tier deployment system
# Version: 1.0.0
# Purpose: Unified installer for Dev/Community/Enterprise tiers

set -euo pipefail

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${CYAN}🚀 BPCI Universal Installer v1.0.0${NC}"
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo "Select your deployment tier:"
echo ""
echo -e "${GREEN}1) Dev Installer${NC}      - Lightweight development setup"
echo -e "${BLUE}2) Community Installer${NC} - Community node with governance"
echo -e "${CYAN}3) Enterprise Installer${NC} - Banking-grade deployment"
echo ""
echo -e "${YELLOW}Note: Server installer is owner-only (separate script)${NC}"
echo ""
read -p "Enter your choice (1-3): " choice

case $choice in
    1)
        echo -e "${GREEN}🔧 Starting Dev Installer...${NC}"
        exec ./pravyom-dev-installer.sh
        ;;
    2)
        echo -e "${BLUE}🌐 Starting Community Installer...${NC}"
        exec ./bpci-community-installer.sh
        ;;
    3)
        echo -e "${CYAN}🏦 Starting Enterprise Installer...${NC}"
        exec ./enterprise-installer.sh
        ;;
    *)
        echo "❌ Invalid choice. Please select 1, 2, or 3."
        exit 1
        ;;
esac
