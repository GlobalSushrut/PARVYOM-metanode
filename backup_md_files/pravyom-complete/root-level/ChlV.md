# TabooMesh++ - Symbolic-Neural Violation Detection AI

## 🎯 Project Overview
TabooMesh++ is an advanced AI system for detecting rule violations in community content using symbolic-neural hybrid approaches.

## 🏆 Best Performance
- **Breakthrough AUC**: **0.8218** (`breakthrough_95_analysis.py`)
- **Approach**: Multiple TF-IDF + Feature Selection + Deep Random Forest
- **Key Features**: Exact pattern matching, semantic features, calibrated ensemble
- **Training Time**: ~50 seconds on RTX6000 ADA
- **Best Individual Fold**: 0.8498 AUC

## 📁 Core Components

### `/taboomesh/` - Core TabooMesh++ Framework
- `models/` - Core model implementations
  - `baseline.py` - Baseline violation detection
  - `enhanced.py` - Enhanced feature extraction
  - `advanced_features.py` - Advanced feature engineering
  - `emotion_detector.py` - Emotional analysis
  - `category_mapper.py` - Rule category mapping
  - `morphism_matrix.py` - Symbolic morphism system
  - `taboomesh_integrated.py` - Integrated model system
  - `calibration.py` - Model calibration
  - `optimization.py` - Performance optimization
  - `production_deployment.py` - Production deployment

### Core Files
- `train.csv` - Training dataset (2,029 samples)
- `test.csv` - Test dataset (10 samples)
- **`breakthrough_95_analysis.py`** - **Best performing model (0.8218 AUC)**
- `breakthrough_95_train_submission.csv` - Best submission file

### Deployment
- `deployment/` - Deployment configurations
- `cloud_deploy.sh` - Cloud deployment script
- `setup_rtx6000_droplet.sh` - RTX6000 ADA setup

## 🚀 Quick Start

1. **Setup Environment**:
   ```bash
   pip install -r requirements.txt
   ```

2. **Run Best Model**:
   ```bash
   python breakthrough_95_analysis.py
   ```

3. **Deploy to Cloud**:
   ```bash
   ./cloud_deploy.sh
   ```

## 📊 Performance History
- **Initial baseline**: ~0.71 AUC
- **Traditional ML ceiling**: ~0.80 AUC
- **Breakthrough approach**: **0.8218 AUC** ⭐
- **Competition target**: 0.95+ AUC (leaderboard shows 0.92+ achievable)

## 🔬 Breakthrough Techniques (0.8218 AUC)
1. **Multiple TF-IDF approaches**: Character-level (3-6 grams), Word-level (1-3 grams), Count vectorizer
2. **Feature selection with chi2**: Reduced 17k features to 8k most important
3. **SVD dimensionality reduction**: 500 components for text features
4. **Deep Random Forest**: 1000 trees, depth 50, max_features='log2'
5. **Exact pattern matching**: Legal, advertising, harassment violations
6. **Top features**: certainty_count, authority_count, exact_harassment_count

## 🎮 Cloud Infrastructure
- **GPU**: NVIDIA RTX 6000 Ada Generation
- **GPU Memory**: 47.4 GB available
- **Server**: DigitalOcean RTX6000 ADA (IP: 143.198.32.40)
- **Environment**: Python 3.10, PyTorch 2.5.1, CUDA 12.1

## 🧹 Project Cleanup
Experimental files have been moved to `experimental_backup/` to maintain a clean core codebase while preserving all research work.

## 📝 Documentation
See `FINAL_PROJECT_REPORT.md` for complete technical documentation.

## 🎯 Next Steps
1. **Submit breakthrough model** (0.8218 AUC) to competition
2. **Investigate 0.95+ techniques** used by top performers
3. **Scale with transformer approaches** for semantic understanding
4. **Deploy production API** with FastAPI

---

**🏆 Current Status**: Clean core TabooMesh++ with breakthrough model achieving **0.8218 AUC** - ready for competition submission and further optimization.
