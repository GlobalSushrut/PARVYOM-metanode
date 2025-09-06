# TabooMesh+++ Cloud GPU Deployment Guide

## 🚀 Cloud Server Setup
**Target Server:** `138.197.133.196`  
**Deployment Type:** GPU-Accelerated Training  
**Goal:** Achieve 0.95+ AUC with comprehensive TabooMesh+++ integration

## 📦 Deployment Package Contents

### Core Files Created:
1. **`deploy_cloud_gpu.sh`** - Main deployment script
2. **`cloud_train_optimized.py`** - GPU-optimized training script
3. **`cloud_requirements.txt`** - Cloud package requirements
4. **`train_taboomesh_gpu_ultimate.py`** - Ultimate GPU integration
5. **`train_taboomesh_ultimate_integration.py`** - Comprehensive integration

### TabooMesh+++ Components Included:
- ✅ **UTKGCE Core** - Unified Trigonometric-Knot Graph Cognition Engine
- ✅ **Pattern Graph Engine** - Graph-based pattern analysis
- ✅ **Rule Specialization Engine** - Rule-specific violation detection
- ✅ **Feature Depth Engine** - Deep semantic feature extraction
- ✅ **Fusion Meta Model** - Advanced ensemble fusion
- ✅ **All Core Modules** - FICG, DEWM, ITLM symbolic reasoning

## 🎯 Deployment Strategy

### Phase 1: Automated Cloud Setup
```bash
# Run the deployment script
./deploy_cloud_gpu.sh
```

**What it does:**
1. Creates deployment package (`taboomesh_deployment.tar.gz`)
2. Uploads to cloud server via SCP
3. Sets up Python environment with GPU support
4. Installs all required packages:
   - XGBoost with GPU support
   - CuML/RAPIDS for GPU acceleration
   - CuPy for GPU arrays
   - All TabooMesh dependencies
5. Downloads NLTK data
6. Runs GPU-accelerated training
7. Downloads results back to local machine

### Phase 2: GPU-Accelerated Training

**Training Components:**
- **Breakthrough Features** (19): Proven 0.87 AUC baseline patterns
- **UTKGCE Features** (25): Advanced symbolic cognitive features
- **Pattern Graph Features** (10): Graph-based violation patterns
- **Rule Specialist Features** (8): Rule-specific detection
- **Feature Depth Features** (20): Deep semantic analysis
- **TF-IDF Features** (500): Optimized text vectorization

**Total Features:** 582 comprehensive features

**GPU-Accelerated Models:**
- XGBoost with `tree_method='gpu_hist'`
- CuML Random Forest (if GPU available)
- Optimized sklearn ensemble
- Adaptive ensemble weighting

## 🔧 Manual Deployment (Alternative)

If automatic deployment fails, use manual steps:

### 1. Connect to Cloud Server
```bash
ssh root@138.197.133.196
```

### 2. Upload Files
```bash
scp -r /home/umesh/Taboomesh root@138.197.133.196:/root/
```

### 3. Setup Environment
```bash
# On cloud server
cd /root/Taboomesh
python3 -m venv taboomesh_env
source taboomesh_env/bin/activate
pip install -r cloud_requirements.txt
```

### 4. Run Training
```bash
PYTHONPATH=/root/Taboomesh python3 cloud_train_optimized.py
```

## 📊 Expected Performance

### Target Metrics:
- **AUC:** 0.95+ (target breakthrough)
- **Baseline:** 0.87 AUC (current best)
- **Improvement:** +0.08 AUC minimum
- **Training Speed:** 10x faster with GPU

### Performance Monitoring:
- Real-time logging to `taboomesh_training.log`
- Cross-validation scores for each model
- Ensemble weight optimization
- GPU utilization tracking

## 🎪 Model Architecture

### Ensemble Configuration:
```
Final Score = Σ(weight_i × model_i_prediction)

Where:
- XGBoost GPU: Adaptive weight based on CV performance
- Random Forest: Adaptive weight based on CV performance  
- Logistic Regression: Adaptive weight based on CV performance
- Gradient Boosting: Adaptive weight based on CV performance
```

### Feature Integration:
```
Symbolic Features (82) + TF-IDF SVD (500) = 582 Total Features

Symbolic Breakdown:
- Breakthrough: 19 (proven violation patterns)
- UTKGCE: 25 (cognitive reasoning)
- Pattern Graph: 10 (graph analysis)
- Rule Specialist: 8 (rule-specific)
- Feature Depth: 20 (semantic depth)
```

## 🏆 Success Criteria

### ✅ Deployment Success:
- [ ] Cloud environment setup complete
- [ ] All packages installed without errors
- [ ] GPU acceleration detected and enabled
- [ ] Training completes without crashes

### ✅ Performance Success:
- [ ] AUC ≥ 0.95 (primary target)
- [ ] AUC > 0.87 (improvement over baseline)
- [ ] All ensemble models train successfully
- [ ] Test predictions generated

### ✅ Integration Success:
- [ ] All TabooMesh+++ components working
- [ ] UTKGCE cognitive engine operational
- [ ] Symbolic reasoning features extracted
- [ ] No performance regression from baseline

## 🚨 Troubleshooting

### Common Issues:

1. **GPU Not Detected:**
   - Check `nvidia-smi` output
   - Verify CUDA installation
   - Fallback to CPU training

2. **Memory Issues:**
   - Reduce batch sizes in training script
   - Use feature selection more aggressively
   - Enable memory optimization flags

3. **Package Installation Failures:**
   - Use CPU alternatives (lightgbm, catboost)
   - Skip CuML if installation fails
   - Fallback to sklearn-only ensemble

4. **SSH Connection Issues:**
   - Verify server IP: `138.197.133.196`
   - Check SSH key authentication
   - Use password authentication if needed

## 📁 Output Files

After successful deployment:

### On Cloud Server:
- `taboomesh_cloud_optimized_model.pkl` - Trained model
- `taboomesh_cloud_submission.csv` - Test predictions
- `taboomesh_training.log` - Training logs

### Downloaded Locally:
- `taboomesh_cloud_optimized_model.pkl` - Complete model
- `taboomesh_cloud_submission.csv` - Submission file

## 🎯 Next Steps After Deployment

1. **Validate Performance:** Check if AUC ≥ 0.95 achieved
2. **Analyze Results:** Review training logs and model performance
3. **Submit Predictions:** Use generated CSV for competition submission
4. **Optimize Further:** If target not reached, tune hyperparameters
5. **Production Deploy:** If successful, prepare for production deployment

## 🔗 Quick Commands

```bash
# Deploy to cloud
./deploy_cloud_gpu.sh

# Check deployment status
ssh root@138.197.133.196 "tail -f /root/Taboomesh/taboomesh_training.log"

# Download results manually
scp root@138.197.133.196:/root/Taboomesh/*.pkl ./
scp root@138.197.133.196:/root/Taboomesh/*.csv ./
```

---

**Ready for deployment!** Run `./deploy_cloud_gpu.sh` to start the automated cloud GPU training process.
