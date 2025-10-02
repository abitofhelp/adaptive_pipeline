# Sample Data for Pipeline Examples

This directory contains test data used by the pipeline examples. All data is synthetic and safe for testing.

## 📁 Directory Structure

```
sample_data/
├── contracts/          # Legal document samples
│   ├── service_agreement.pdf
│   ├── nda_template.docx
│   └── contract_with_pii.txt
├── sales_data/         # Business data samples
│   ├── sales_q1_2024.csv
│   ├── sales_q2_2024.csv
│   └── customer_data.json
└── images/             # Image processing samples
    ├── document_scan.jpg
    ├── logo.png
    └── chart.svg
```

## 🔒 Data Privacy

All sample data is:
- **Synthetic**: Generated for testing purposes only
- **Non-sensitive**: Contains no real personal or business information
- **Safe**: Can be shared, modified, or deleted without concern

## 📊 Data Descriptions

### Contracts (`contracts/`)
- **service_agreement.pdf**: Standard business contract template
- **nda_template.docx**: Non-disclosure agreement sample
- **contract_with_pii.txt**: Text contract with fake PII for redaction testing

### Sales Data (`sales_data/`)
- **sales_q1_2024.csv**: Quarterly sales data with data quality issues
- **sales_q2_2024.csv**: Clean quarterly sales data
- **customer_data.json**: Customer information in JSON format

### Images (`images/`)
- **document_scan.jpg**: Scanned document for OCR testing
- **logo.png**: Company logo for image processing
- **chart.svg**: Vector graphics for format conversion

## 🛠️ Usage in Examples

### User Walkthroughs
```bash
# Document encryption uses contracts/
./user_walkthroughs/document_encryption_demo.sh

# Analytics uses sales_data/
./user_walkthroughs/sales_analytics_demo.sh
```

### Developer Guides
```bash
# File I/O examples use all data types
cargo run --example file_io_demo

# Image processing uses images/
cargo run --example image_processing_demo
```

## 🔄 Regenerating Sample Data

To create fresh sample data:

```bash
# Run the data generation script
./scripts/generate_sample_data.sh

# Or generate specific types
./scripts/generate_sample_data.sh --contracts-only
./scripts/generate_sample_data.sh --sales-only
```

## 📝 Adding New Sample Data

When adding new sample data:

1. **Keep it synthetic**: Never use real data
2. **Document purpose**: Update this README
3. **Include variety**: Add both clean and messy data for testing
4. **Consider size**: Keep files small for fast testing
5. **Follow naming**: Use descriptive, consistent names

---

*All sample data is for testing purposes only and contains no real sensitive information.*
