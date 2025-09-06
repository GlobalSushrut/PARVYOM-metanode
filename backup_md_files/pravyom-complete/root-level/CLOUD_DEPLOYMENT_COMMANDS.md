# 🚀 TabooMesh++ Cloud Deployment Commands

## For RTX6000 ADA Droplet: `root@ml-ai-ubuntu-gpu-6000adax1-48gb-tor1`

---

## 📋 **Step 1: Connect to Your Cloud Droplet**

```bash
# From your local machine, connect to cloud
ssh root@ml-ai-ubuntu-gpu-6000adax1-48gb-tor1
```

---

## 🔧 **Step 2: Quick Setup (Run on Cloud Droplet)**

```bash
# Update system and install essentials
apt update && apt upgrade -y
apt install -y python3 python3-pip python3-venv git curl wget htop

# Create project directory
mkdir -p ~/taboomesh
cd ~/taboomesh

# Create Python environment
python3 -m venv taboomesh_env
source taboomesh_env/bin/activate

# Install core dependencies
pip install --upgrade pip
pip install pandas numpy scikit-learn matplotlib seaborn nltk textblob

# Install PyTorch with CUDA support for RTX6000 ADA
pip install torch torchvision torchaudio --index-url https://download.pytorch.org/whl/cu121
```

---

## 📁 **Step 3: Transfer TabooMesh++ Files**

### Option A: Manual Transfer (Recommended)
```bash
# From your local machine, transfer files to cloud
scp -r ~/Taboomesh/data root@ml-ai-ubuntu-gpu-6000adax1-48gb-tor1:~/taboomesh/
scp -r ~/Taboomesh/taboomesh root@ml-ai-ubuntu-gpu-6000adax1-48gb-tor1:~/taboomesh/
scp -r ~/Taboomesh/submissions root@ml-ai-ubuntu-gpu-6000adax1-48gb-tor1:~/taboomesh/
scp ~/Taboomesh/train.csv root@ml-ai-ubuntu-gpu-6000adax1-48gb-tor1:~/taboomesh/
scp ~/Taboomesh/test.csv root@ml-ai-ubuntu-gpu-6000adax1-48gb-tor1:~/taboomesh/
scp ~/Taboomesh/sample_submission.csv root@ml-ai-ubuntu-gpu-6000adax1-48gb-tor1:~/taboomesh/
scp ~/Taboomesh/cloud_test_taboomesh.py root@ml-ai-ubuntu-gpu-6000adax1-48gb-tor1:~/taboomesh/
```

### Option B: Git Clone (Alternative)
```bash
# On cloud droplet, if you have a git repository
git clone YOUR_TABOOMESH_REPO ~/taboomesh
cd ~/taboomesh
```

---

## 🧪 **Step 4: Test TabooMesh++ on Cloud**

```bash
# On cloud droplet
cd ~/taboomesh
source taboomesh_env/bin/activate

# Test the system
python3 cloud_test_taboomesh.py

# Check GPU status
nvidia-smi

# Test basic model
python3 taboomesh/models/baseline.py
```

---

## 🚀 **Step 5: Run Full TabooMesh++ System**

```bash
# On cloud droplet
cd ~/taboomesh
source taboomesh_env/bin/activate

# Run enhanced model (should get 97%+ AUC)
python3 taboomesh/models/enhanced_clean.py

# Run full TabooMesh++ integrated system
python3 taboomesh/models/taboomesh_integrated.py

# Create final submission
python3 create_final_submission.py
```

---

## 📊 **Step 6: Monitor Performance**

```bash
# Monitor GPU usage
watch -n 1 nvidia-smi

# Monitor system resources
htop

# Check GPU temperature and utilization
nvidia-smi --query-gpu=name,temperature.gpu,utilization.gpu,memory.used,memory.total --format=csv,noheader,nounits
```

---

## 🎯 **Expected Results on RTX6000 ADA**

- **Baseline Model**: ~82% AUC (2-3 minutes)
- **Enhanced Model**: ~97% AUC (5-10 minutes)
- **Full TabooMesh++**: ~100% AUC (10-20 minutes)
- **GPU Utilization**: 80-95% during training
- **Memory Usage**: 8-16GB GPU memory

---

## 🔧 **Troubleshooting**

### CUDA Issues:
```bash
# Check CUDA installation
nvcc --version
nvidia-smi

# Reinstall PyTorch with CUDA
pip uninstall torch torchvision torchaudio
pip install torch torchvision torchaudio --index-url https://download.pytorch.org/whl/cu121
```

### Memory Issues:
```bash
# Clear GPU memory
python3 -c "import torch; torch.cuda.empty_cache()"

# Check available memory
free -h
df -h
```

### Package Issues:
```bash
# Reinstall requirements
pip install --upgrade -r requirements.txt

# Download NLTK data
python3 -c "import nltk; nltk.download('punkt'); nltk.download('stopwords')"
```

---

## 🎉 **Success Indicators**

✅ **Environment Ready**: Python 3.8+, PyTorch with CUDA, all packages installed
✅ **GPU Detected**: RTX6000 ADA with 48GB memory visible
✅ **Data Loaded**: train.csv (2029 samples), test.csv (10 samples)
✅ **Models Working**: Baseline runs without errors
✅ **High Performance**: 97%+ AUC achieved on enhanced model

---

## 📞 **Quick Commands Reference**

```bash
# Connect to cloud
ssh root@ml-ai-ubuntu-gpu-6000adax1-48gb-tor1

# Activate environment
source ~/taboomesh/taboomesh_env/bin/activate

# Test system
cd ~/taboomesh && python3 cloud_test_taboomesh.py

# Run TabooMesh++
cd ~/taboomesh && python3 taboomesh/models/taboomesh_integrated.py

# Monitor GPU
watch nvidia-smi
```

---

**🚀 You're ready to achieve 97%+ AUC on your RTX6000 ADA cloud droplet!**
