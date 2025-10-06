# Pipeline Examples Directory

This directory contains practical examples demonstrating the pipeline system's capabilities. Examples are organized by audience and complexity level.

## 📁 Directory Structure

```
examples/
├── README.md                           # This file
├── user_walkthroughs/                  # End-user demos (shell scripts)
│   ├── document_encryption_demo.sh     # Encrypt sensitive documents
│   ├── sales_analytics_demo.sh         # Process CSV data for BI
│   └── legal_document_pipeline.py      # Custom pipeline creation
├── developer_guides/                   # Code examples for developers
│   ├── file_io_demo.rs                # File I/O operations
│   ├── generic_service_demo.rs         # Generic service patterns
│   ├── generic_compression_demo.rs     # Compression service usage
│   └── sqlite_repository_demo.rs       # Database operations
├── integration_examples/               # End-to-end scenarios
│   └── complete_file_processing_demo.rs # Full pipeline processing
└── sample_data/                        # Test data for examples
    ├── contracts/                      # Sample legal documents
    ├── sales_data/                     # Sample CSV files
    └── images/                         # Sample image files
```

## 🎯 Quick Start

### For End Users
Start with the **user walkthroughs** - these are interactive demos you can run:

```bash
# Document encryption walkthrough
./examples/user_walkthroughs/document_encryption_demo.sh

# Sales data analytics walkthrough  
./examples/user_walkthroughs/sales_analytics_demo.sh
```

### For Developers
Explore the **developer guides** - these show how to use the API:

```bash
# Run file I/O examples
cargo run --example file_io_demo

# Run generic service examples
cargo run --example generic_service_demo
```

## 📚 Examples by Category

### 🔐 Security & Encryption
- **User**: `user_walkthroughs/document_encryption_demo.sh`
- **Developer**: Code examples in User Guide Task 2

### 📊 Data Analytics
- **User**: `user_walkthroughs/sales_analytics_demo.sh`
- **Developer**: CSV processing patterns in developer guides

### 🔧 Custom Pipelines
- **User**: `user_walkthroughs/legal_document_pipeline.py`
- **Developer**: `developer_guides/generic_service_demo.rs`

### 💾 File Processing
- **User**: Examples in Quick Start Guide
- **Developer**: `developer_guides/file_io_demo.rs`

### 🗄️ Database Operations
- **Developer**: `developer_guides/sqlite_repository_demo.rs`
- **Integration**: Database setup in `scripts/test_data/`

## 🎓 Learning Path

### Beginner (End Users)
1. **Quick Start**: `docs/QUICK_START.md`
2. **Basic Demo**: `user_walkthroughs/document_encryption_demo.sh`
3. **User Guide**: `docs/USER_GUIDE.md`

### Intermediate (Power Users)
1. **Analytics Demo**: `user_walkthroughs/sales_analytics_demo.sh`
2. **Custom Pipeline**: `user_walkthroughs/legal_document_pipeline.py`
3. **Advanced Configuration**: User Guide Task 4

### Advanced (Developers)
1. **API Examples**: `developer_guides/file_io_demo.rs`
2. **Generic Patterns**: `developer_guides/generic_service_demo.rs`
3. **Integration**: `integration_examples/complete_file_processing_demo.rs`

## 🛠️ Running Examples

### Shell Script Demos (User Walkthroughs)
```bash
# Make executable
chmod +x examples/user_walkthroughs/*.sh

# Run individual demos
./examples/user_walkthroughs/document_encryption_demo.sh
```

### Rust Code Examples (Developer Guides)
```bash
# List all available examples
cargo run --example

# Run specific example
cargo run --example file_io_demo

# Run with verbose output
cargo run --example generic_service_demo -- --verbose
```

### Python Examples
```bash
# Install dependencies (if needed)
pip install pyyaml

# Run Python examples
python3 examples/user_walkthroughs/legal_document_pipeline.py
```

## 📋 Prerequisites

### For User Walkthroughs
- **Shell**: bash or zsh
- **Tools**: Basic command line tools (ls, cat, grep)
- **Optional**: bc (for calculations), python3 (for JSON formatting)

### For Developer Guides
- **Rust**: Latest stable version
- **Cargo**: Package manager
- **Dependencies**: Automatically handled by Cargo

## 🔗 Related Documentation

- **[User Guide](../docs/USER_GUIDE.md)** - Comprehensive user documentation
- **[Quick Start](../docs/QUICK_START.md)** - 5-minute getting started guide
- **[Architecture Docs](../../docs/architecture/)** - Technical architecture
- **[API Reference](../docs/API_REFERENCE.md)** - Developer API documentation

## 📊 Example Status

| Example | Status | Audience | Complexity |
|---------|--------|----------|------------|
| document_encryption_demo.sh | ✅ Complete | End User | Beginner |
| sales_analytics_demo.sh | ✅ Complete | End User | Intermediate |
| file_io_demo.rs | ✅ Complete | Developer | Intermediate |
| generic_service_demo.rs | ✅ Complete | Developer | Advanced |
| sqlite_repository_demo.rs | ✅ Complete | Developer | Advanced |
| complete_file_processing_demo.rs | ✅ Complete | Developer | Advanced |
| legal_document_pipeline.py | 🚧 Planned | End User | Advanced |

---

*For questions or issues with examples, please check the main documentation or open an issue.*
