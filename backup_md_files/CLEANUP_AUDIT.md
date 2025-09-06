# Systematic Cleanup Audit - Military-Grade Project Restructuring

## 🎯 **CLEANUP STRATEGY: PRESERVE FIRST, REMOVE CAREFULLY**

### **Phase 1: SAFE TO REMOVE (Build Artifacts & Temporary Files)**
- ✅ `target/` directories (already removed)
- ✅ `*.tmp`, `*.bak` files (none found)
- ⚠️ `Cargo.lock` files (keep - needed for reproducible builds)

### **Phase 2: DOCUMENTATION & LEGACY ANALYSIS**
**Keep for now - analyze content first:**
- `coredocs/` - Core documentation (31 files)
- `tempdocs/` - Temporary documentation (20 files)
- `docs/` - Main documentation (11 files)
- `v1-reference/` - Version 1 reference
- `preplanning/` - Planning documents (15 files)
- `metanode-spec/` - Specifications (7 files)

### **Phase 3: CONFIGURATION & DEPLOYMENT**
**Analyze before removing:**
- `config/` - Configuration files (7 files)
- `deploy/` - Deployment scripts
- `installer/` - Installer components (19 files)
- `scripts/` - Build/utility scripts (2 files)

### **Phase 4: DASHBOARD & MONITORING**
**Evaluate for consolidation:**
- `dashboards/` - Dashboard components (22 files)
- `server/` - Server components (5 files)

### **Phase 5: RUST CRATES (MOST CRITICAL)**
**DO NOT DELETE - CONSOLIDATE ONLY:**
- `rust/crates/` - 44 existing crates (PRESERVE ALL)
- These contain working implementations we're consolidating

## 🚨 **CRITICAL PRESERVATION RULES**

### **NEVER DELETE:**
1. **Any Rust crate in `rust/crates/`** - These are our source implementations
2. **Cargo.toml files** - Needed for dependency management
3. **src/ directories** - Contains all source code
4. **Any file with active functionality**

### **SAFE TO REMOVE AFTER ANALYSIS:**
1. **Duplicate documentation** (after content review)
2. **Outdated planning documents** (after content extraction)
3. **Legacy configuration** (after migration to new structure)
4. **Unused scripts** (after functionality verification)

## 📋 **SYSTEMATIC AUDIT PROCESS**

### **Step 1: Content Analysis**
For each directory, analyze:
- What functionality does it provide?
- Is it actively used by the build system?
- Does it contain unique information?
- Can it be safely consolidated or removed?

### **Step 2: Dependency Check**
Before removing anything:
- Check if referenced in Cargo.toml files
- Check if referenced in build scripts
- Check if referenced in documentation
- Check if referenced by other components

### **Step 3: Safe Removal Process**
1. Create backup of item to be removed
2. Remove item temporarily
3. Test build system
4. Test functionality
5. If successful, permanently remove
6. If issues, restore immediately

## 🎯 **IMMEDIATE SAFE ACTIONS**

### **Phase A: Documentation Consolidation**
1. Analyze `coredocs/`, `tempdocs/`, `docs/` for duplicates
2. Extract essential content to new `docs/` structure
3. Remove duplicates and outdated content

### **Phase B: Configuration Cleanup**
1. Analyze `config/` directory structure
2. Migrate essential configs to new architecture
3. Remove legacy configuration files

### **Phase C: Build System Cleanup**
1. Analyze `scripts/`, `installer/` components
2. Migrate essential scripts to `tools/` directory
3. Remove outdated build artifacts

## ⚠️ **WHAT WE'RE NOT TOUCHING YET**
- `rust/crates/` - Source of truth for all implementations
- Active Cargo.toml files
- Any file that might be referenced by build system
- Configuration files until we understand their purpose

## 📊 **SUCCESS METRICS**
- No functionality lost
- Build system still works
- All tests still pass
- Documentation remains accessible
- Configuration remains functional
