#!/bin/bash
# Metanode CLI Demo Script - Production Ready
# Showcases complete Metanode/BankCoin/BISO story in one bashable demo

set -euo pipefail

# Colors for demo output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m'

# Demo configuration
DEMO_BANK_NAME="BRICS Bank A"
DEMO_JURISDICTION="BR"
DEMO_AMOUNT="50000"

echo -e "${BLUE}╔══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║                 METANODE CLI DEMO SCRIPT                     ║${NC}"
echo -e "${BLUE}║            Complete BankCoin + BISO Workflow                 ║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════════════════════════════╝${NC}"
echo ""

demo_step() {
    echo -e "${CYAN}🎬 DEMO STEP: $1${NC}"
    echo -e "${YELLOW}Command: $2${NC}"
    echo ""
}

# Step 1: Bank Registration & Onboarding
demo_step "Bank Registration & Onboarding" "metanode bank register"
echo "# Register a new validator bank"
metanode bank register --name "$DEMO_BANK_NAME" --jurisdiction "$DEMO_JURISDICTION" --dry-run --json
echo ""

# Step 2: Proof of Reserves Setup
demo_step "Proof of Reserves (PoR) Setup" "metanode bank por run"
echo "# Set up PoR for fiat and gold attestations"
metanode bank por run --fiat BRL --gold LBMA --publish --dry-run --json
echo ""

# Step 3: Mother Coin Issuance
demo_step "Mother Coin Issuance" "metanode coin issue"
echo "# Issue a mother coin (governance-gated)"
metanode coin issue --type mother --parent genesis --dry-run --json
echo ""

# Step 4: Coin Activation
demo_step "Coin Activation" "metanode coin activate"
echo "# Activate coin with PoE job (locks value, flips to active)"
COIN_ID="mother-coin-001"
JOB_ID="poe-job-001"
metanode coin activate "$COIN_ID" --job "$JOB_ID" --dry-run --json
echo ""

# Step 5: Cross-Border Settlement via Gold
demo_step "Cross-Border Settlement" "metanode settle xborder"
echo "# Execute cross-border payment via gold bridge"
metanode settle xborder --from INR --to USD --amount "$DEMO_AMOUNT" --via gold --receipt --dry-run --json
echo ""

# Step 6: Receipt Verification
demo_step "Receipt Verification" "metanode receipt verify"
echo "# Verify settlement receipt with attestations"
RECEIPT_ID="settlement-receipt-001"
metanode receipt verify "$RECEIPT_ID" --json --dry-run
echo ""

# Step 7: BISO Policy Application
demo_step "BISO Policy Application" "metanode biso apply"
echo "# Apply GDPR compliance policy"
cat > demo-gdpr-policy.hcl << 'EOF'
policy "gdpr-strict" {
  description = "GDPR compliance for EU banking operations"
  
  geographic_restrictions {
    allowed_regions = ["EU", "EEA"]
    blocked_regions = ["CN", "RU"]
  }
  
  data_classification {
    require_encryption = true
    retention_days = 90
    consent_required = true
  }
  
  processing_purpose {
    allowed = ["banking", "compliance", "audit"]
    blocked = ["marketing", "analytics"]
  }
}
EOF

metanode biso lint demo-gdpr-policy.hcl --dry-run
metanode biso apply demo-gdpr-policy.hcl --dry-run --json
echo ""

# Step 8: PoE Monitoring
demo_step "PoE Index Monitoring" "metanode economics poe"
echo "# Monitor PoE index and thresholds"
metanode economics poe --show --json
metanode analytics poe --by-epoch --heatmap --json
echo ""

# Step 9: Governance Proposal
demo_step "Governance Proposal" "metanode gov propose"
echo "# Propose PoE threshold adjustment"
metanode gov propose set-threshold --tau1 100 --tau2 250 --tau3 500 --dry-run --json
echo ""

# Step 10: Security Posture Check
demo_step "Security Posture Check" "metanode security posture"
echo "# Check overall security and compliance status"
metanode security posture --json
metanode biso dashboard --red-yellow-green-counts --json
echo ""

# Step 11: Observability Dashboard
demo_step "Observability Dashboard" "metanode dashboard open"
echo "# Launch comprehensive monitoring dashboard"
metanode dashboard open --service governance --dry-run
metanode dashboard open --service banking --dry-run
echo ""

# Step 12: Attestation Publishing
demo_step "Attestation Publishing" "metanode attest publish"
echo "# Publish signed PoR attestation"
metanode attest publish --type por --bank "brics-bank-a" --signed --dry-run --json
echo ""

echo ""
echo -e "${GREEN}🎉 DEMO COMPLETE! 🎉${NC}"
echo ""
echo -e "${PURPLE}Key Achievements Demonstrated:${NC}"
echo "✅ Bank onboarding with regulatory compliance"
echo "✅ Proof of Reserves for fiat and gold"
echo "✅ Mother coin issuance and activation"
echo "✅ Cross-border settlement via gold bridge"
echo "✅ Receipt verification with attestations"
echo "✅ BISO policy enforcement (GDPR)"
echo "✅ PoE monitoring and governance"
echo "✅ Security posture and compliance dashboard"
echo ""
echo -e "${CYAN}Next Steps:${NC}"
echo "• Remove --dry-run flags for live execution"
echo "• Configure real bank credentials and keys"
echo "• Set up production monitoring and alerts"
echo "• Deploy to mainnet with full compliance"
echo ""

# Cleanup demo files
rm -f demo-gdpr-policy.hcl

echo -e "${BLUE}Demo script completed successfully!${NC}"
