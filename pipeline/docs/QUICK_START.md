# Quick Start Guide - Pipeline System

## ⚡ Get Running in 5 Minutes

### 🎯 What You'll Accomplish
- Install and run the pipeline system
- Process your first file
- Understand the basic workflow
- Know where to get help

---

## 🚀 Step 1: Installation (2 minutes)

### Option A: Binary Installation (Recommended)
```bash
# Download the latest release
curl -L https://github.com/abitofhelp/optimized_adaptive_pipeline_rs/releases/latest/download/pipeline-macos.tar.gz | tar xz

# Make executable
chmod +x pipeline

# Move to PATH (optional)
sudo mv pipeline /usr/local/bin/
```

### Option B: Build from Source
```bash
# Clone repository
git clone https://github.com/abitofhelp/optimized_adaptive_pipeline_rs.git
cd optimized_adaptive_pipeline_rs/pipeline

# Build (requires Rust)
cargo build --release

# Binary location
./target/release/pipeline
```

### ✅ Verify Installation
```bash
pipeline --version
# Expected output: Pipeline System v1.0.0
```

---

## 🗄️ Step 2: Database Setup (30 seconds)

### 📊 Automatic Database Initialization

The SQLite database is automatically created on first run:

```bash
# Database auto-creates at: scripts/test_data/test_pipeline.db
pipeline --help
# ✅ Database initialized successfully
```

### 🔍 Verify Database Setup

```bash
# Check if database exists
ls -la scripts/test_data/test_pipeline.db

# View database tables (optional)
sqlite3 scripts/test_data/test_pipeline.db ".tables"
# Expected: pipelines, pipeline_stages, processing_sessions, file_chunks, security_contexts
```

### ⚡ Manual Database Creation (if needed)

```bash
# Only if automatic setup fails
sqlite3 scripts/test_data/test_pipeline.db < scripts/test_data/create_test_database.sql
echo "Database created successfully!"
```

**📍 Database Location**: `scripts/test_data/test_pipeline.db`

---

## 📁 Step 3: Prepare Sample Files (30 seconds)

Create a test directory with sample files:
```bash
mkdir ~/pipeline-test
cd ~/pipeline-test

# Create sample files
echo "Hello, Pipeline!" > sample.txt
echo "This is a test document." > document.txt
```

---

## 🎮 Step 4: Run Your First Pipeline (2 minutes)

### Start the Application
```bash
pipeline start
# Opens dashboard at http://localhost:8080
```

### Process Files via Web Interface
1. **Open Browser**: Go to `http://localhost:8080`
2. **Select Pipeline**: Choose "Document Processing Pipeline"
3. **Add Files**: Drag `sample.txt` into the file area
4. **Click Run**: Start processing
5. **Watch Progress**: See real-time progress bar
6. **Check Results**: Find processed files in `./output/`

### Or Use Command Line
```bash
# Process a single file
pipeline process --input sample.txt --output ./output/

# Process multiple files
pipeline process --input "*.txt" --output ./output/ --pipeline "Document Processing"

# View processing status
pipeline status
```

---

## 📊 Step 4: Understand the Output (30 seconds)

After processing, you'll see:

```
📁 output/
├── sample_processed.txt      # Your processed file
├── processing_log.json       # Detailed processing log
├── metrics.json              # Performance metrics
└── checksums.txt             # File integrity verification
```

**Key Files**:
- **Processed Files**: Your transformed data
- **Logs**: What happened during processing
- **Metrics**: Performance statistics
- **Checksums**: Verify file integrity

---

## 🎯 What Just Happened?

Your file went through these stages:
1. **Validation** ✅ - Checked file format and size
2. **Processing** ⚙️ - Applied transformations
3. **Output** 📤 - Saved results with metadata

**Performance**: Your file was processed at ~38 MB/s with built-in error handling and security validation.

---

## 🔄 Next Steps

### Try Different Pipelines
```bash
# List available pipelines
pipeline list-pipelines

# Try image processing
pipeline process --input photo.jpg --pipeline "Image Processing"

# Try data analytics
pipeline process --input data.csv --pipeline "Data Analytics"
```

### Create Custom Pipeline
```bash
# Open pipeline builder
pipeline create-pipeline --name "My Custom Pipeline"

# Or edit configuration file
pipeline edit-config my-pipeline.yaml
```

### Monitor Performance
```bash
# View real-time metrics
pipeline metrics --live

# View processing history
pipeline history --last 10
```

---

## 🆘 Quick Troubleshooting

### ❌ Common Issues

**"Command not found: pipeline"**
```bash
# Add to PATH or use full path
export PATH=$PATH:/path/to/pipeline
# Or use: ./pipeline instead of pipeline
```

**"Permission denied"**
```bash
# Make executable
chmod +x pipeline

# Check file permissions
ls -la sample.txt
```

**"Processing failed"**
```bash
# Check logs
pipeline logs --last

# Verify file format
file sample.txt

# Try with smaller file
echo "test" > small.txt && pipeline process --input small.txt
```

**"Port 8080 in use"**
```bash
# Use different port
pipeline start --port 8081

# Or stop conflicting service
lsof -ti:8080 | xargs kill
```

---

## 📚 Learn More

### 📖 Essential Reading
- **[Full User Guide](USER_GUIDE.md)** - Complete documentation
- **[API Reference](API_REFERENCE.md)** - For developers
- **[Configuration Guide](CONFIGURATION.md)** - Advanced settings

### 🎓 Tutorials
- **Basic File Processing** - Process documents and images
- **Data Pipeline Creation** - Build analytics workflows
- **Security and Encryption** - Protect sensitive data
- **Performance Optimization** - Handle large files efficiently

### 🤝 Get Help
- **Documentation**: All guides in `/docs/` folder
- **Examples**: Sample configurations in `/examples/`
- **Community**: GitHub Discussions
- **Issues**: GitHub Issues for bugs

---

## 🎉 Success! You're Ready

You now have:
- ✅ Working pipeline system
- ✅ Successfully processed your first file
- ✅ Understanding of basic workflow
- ✅ Knowledge of where to find help

### 🚀 Ready for Production?

**For Personal Use**: You're all set! Start processing your files.

**For Team Use**: Check the [Administrator Guide](ADMIN_GUIDE.md) for:
- Multi-user setup
- Database configuration
- Security policies
- Monitoring and logging

**For Development**: See the [Developer Guide](DEVELOPER_GUIDE.md) for:
- API integration
- Custom stage development
- Plugin architecture
- Testing frameworks

---

*Need help? Check the [Full User Guide](USER_GUIDE.md) or open an issue on GitHub.*
