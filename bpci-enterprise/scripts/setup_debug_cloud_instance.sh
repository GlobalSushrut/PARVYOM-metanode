#!/bin/bash

# BSO-K8 Debug Cloud Instance Setup Script
# Creates a temporary 4GB RAM instance for compilation and debugging

set -e

echo "🚀 Setting up BSO-K8 Debug Cloud Instance"

# Configuration
INSTANCE_NAME="bso-k8-debug-$(date +%Y%m%d-%H%M%S)"
INSTANCE_SIZE="s-2vcpu-4gb"  # 2 vCPU, 4GB RAM, 80GB SSD
REGION="nyc1"
IMAGE="ubuntu-22-04-x64"
SSH_KEY_NAME="bso-debug-key"

echo "📋 Instance Configuration:"
echo "   Name: $INSTANCE_NAME"
echo "   Size: $INSTANCE_SIZE (2 vCPU, 4GB RAM)"
echo "   Region: $REGION"
echo "   Image: $IMAGE"

# Check if doctl is installed
if ! command -v doctl &> /dev/null; then
    echo "❌ doctl (DigitalOcean CLI) not found. Installing..."
    
    # Download and install doctl
    cd /tmp
    wget https://github.com/digitalocean/doctl/releases/download/v1.104.0/doctl-1.104.0-linux-amd64.tar.gz
    tar xf doctl-1.104.0-linux-amd64.tar.gz
    sudo mv doctl /usr/local/bin
    
    echo "✅ doctl installed successfully"
    echo "⚠️  Please run 'doctl auth init' to authenticate with your DigitalOcean account"
    echo "⚠️  Then run this script again"
    exit 1
fi

# Check if authenticated
if ! doctl account get &> /dev/null; then
    echo "❌ Not authenticated with DigitalOcean"
    echo "⚠️  Please run 'doctl auth init' to authenticate"
    exit 1
fi

echo "✅ DigitalOcean CLI authenticated"

# Generate SSH key if it doesn't exist
if [ ! -f ~/.ssh/bso_debug_key ]; then
    echo "🔑 Generating SSH key for debug instance..."
    ssh-keygen -t rsa -b 4096 -f ~/.ssh/bso_debug_key -N "" -C "bso-debug-$(date +%Y%m%d)"
    echo "✅ SSH key generated: ~/.ssh/bso_debug_key"
fi

# Add SSH key to DigitalOcean if not exists
if ! doctl compute ssh-key list --format Name --no-header | grep -q "$SSH_KEY_NAME"; then
    echo "📤 Uploading SSH key to DigitalOcean..."
    doctl compute ssh-key import "$SSH_KEY_NAME" --public-key-file ~/.ssh/bso_debug_key.pub
    echo "✅ SSH key uploaded"
else
    echo "✅ SSH key already exists in DigitalOcean"
fi

# Create the instance
echo "🚀 Creating debug instance..."
DROPLET_ID=$(doctl compute droplet create "$INSTANCE_NAME" \
    --size "$INSTANCE_SIZE" \
    --image "$IMAGE" \
    --region "$REGION" \
    --ssh-keys "$SSH_KEY_NAME" \
    --enable-monitoring \
    --enable-ipv6 \
    --format ID --no-header)

echo "✅ Instance created with ID: $DROPLET_ID"

# Wait for instance to be ready
echo "⏳ Waiting for instance to be ready..."
while true; do
    STATUS=$(doctl compute droplet get "$DROPLET_ID" --format Status --no-header)
    if [ "$STATUS" = "active" ]; then
        break
    fi
    echo "   Status: $STATUS - waiting..."
    sleep 10
done

# Get instance IP
INSTANCE_IP=$(doctl compute droplet get "$DROPLET_ID" --format PublicIPv4 --no-header)
echo "✅ Instance ready! IP: $INSTANCE_IP"

# Wait for SSH to be available
echo "⏳ Waiting for SSH to be available..."
while ! ssh -o ConnectTimeout=5 -o StrictHostKeyChecking=no -i ~/.ssh/bso_debug_key root@"$INSTANCE_IP" echo "SSH ready" &> /dev/null; do
    echo "   SSH not ready yet - waiting..."
    sleep 10
done

echo "✅ SSH connection ready"

# Create setup script for the instance
cat > /tmp/instance_setup.sh << 'EOF'
#!/bin/bash

set -e

echo "🔧 Setting up BSO-K8 debug environment..."

# Update system
apt-get update -y
apt-get upgrade -y

# Install essential tools
apt-get install -y \
    curl \
    wget \
    git \
    build-essential \
    pkg-config \
    libssl-dev \
    htop \
    tree \
    unzip

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env

# Install additional Rust components
rustup component add clippy rustfmt

# Install Node.js (for any frontend debugging)
curl -fsSL https://deb.nodesource.com/setup_18.x | bash -
apt-get install -y nodejs

# Create workspace directory
mkdir -p /workspace
cd /workspace

# Set up git (will be configured when code is uploaded)
git config --global init.defaultBranch main
git config --global user.name "BSO Debug"
git config --global user.email "debug@bso-k8.local"

echo "✅ BSO-K8 debug environment setup complete!"
echo "📊 System Resources:"
free -h
echo ""
echo "🔧 Installed Tools:"
echo "   Rust: $(rustc --version)"
echo "   Cargo: $(cargo --version)"
echo "   Node.js: $(node --version)"
echo "   Git: $(git --version)"

EOF

# Upload and run setup script
echo "📤 Uploading setup script to instance..."
scp -o StrictHostKeyChecking=no -i ~/.ssh/bso_debug_key /tmp/instance_setup.sh root@"$INSTANCE_IP":/tmp/

echo "🔧 Running setup script on instance..."
ssh -o StrictHostKeyChecking=no -i ~/.ssh/bso_debug_key root@"$INSTANCE_IP" "chmod +x /tmp/instance_setup.sh && /tmp/instance_setup.sh"

# Create connection info file
cat > /tmp/bso_debug_connection.txt << EOF
BSO-K8 Debug Cloud Instance Connection Info
==========================================

Instance ID: $DROPLET_ID
Instance IP: $INSTANCE_IP
SSH Command: ssh -i ~/.ssh/bso_debug_key root@$INSTANCE_IP

To connect:
1. ssh -i ~/.ssh/bso_debug_key root@$INSTANCE_IP
2. cd /workspace

To upload code:
scp -r -i ~/.ssh/bso_debug_key /path/to/local/code root@$INSTANCE_IP:/workspace/

To destroy instance when done:
doctl compute droplet delete $DROPLET_ID

Instance Specs:
- 2 vCPU cores
- 4GB RAM
- 80GB SSD storage
- Ubuntu 22.04 LTS
- Rust toolchain installed
- Node.js 18 installed

EOF

cp /tmp/bso_debug_connection.txt ~/bso_debug_connection.txt

echo ""
echo "🎉 BSO-K8 Debug Cloud Instance Setup Complete!"
echo ""
echo "📋 Connection Details:"
echo "   Instance IP: $INSTANCE_IP"
echo "   SSH Command: ssh -i ~/.ssh/bso_debug_key root@$INSTANCE_IP"
echo ""
echo "📁 Connection info saved to: ~/bso_debug_connection.txt"
echo ""
echo "🚀 Next Steps:"
echo "1. Upload your code: scp -r -i ~/.ssh/bso_debug_key /home/umesh/metanode/bpci-enterprise root@$INSTANCE_IP:/workspace/"
echo "2. Connect to instance: ssh -i ~/.ssh/bso_debug_key root@$INSTANCE_IP"
echo "3. Debug and fix BSO-K8 orchestrator"
echo "4. When done: doctl compute droplet delete $DROPLET_ID"
echo ""
echo "💰 Estimated cost: ~$0.036/hour (~$0.86/day)"
