#!/bin/bash

# Document Encryption Demo
# Based on Task 2 from the User Guide
# This script demonstrates encrypting sensitive documents

set -e  # Exit on any error

echo "🔐 Document Encryption Demo"
echo "=========================="

# Step 1: Create demo directory and sample files
echo "📁 Step 1: Setting up demo files..."
mkdir -p ~/encryption-demo
cd ~/encryption-demo

# Create sample documents with realistic content
cat > contract.txt << 'EOF'
CONFIDENTIAL BUSINESS CONTRACT

Client: Acme Corporation
Contract ID: ACME-2024-001
Date: January 15, 2024

Terms and Conditions:
- Service Duration: 24 months
- Payment Terms: Net 30 days
- Confidentiality: All information is proprietary
- Termination: 30 days written notice

Authorized Signatures:
- Client Representative: [SIGNATURE]
- Service Provider: [SIGNATURE]

This document contains confidential business information.
EOF

cat > financial_report.csv << 'EOF'
Date,Category,Amount,Description
2024-01-01,Revenue,125000.00,Q1 Sales Revenue
2024-01-01,Expenses,45000.00,Operating Expenses
2024-01-01,Profit,80000.00,Net Profit Q1
2024-02-01,Revenue,135000.00,Q2 Sales Revenue
2024-02-01,Expenses,48000.00,Operating Expenses
2024-02-01,Profit,87000.00,Net Profit Q2
EOF

cat > meeting_notes.md << 'EOF'
# Executive Meeting Notes - January 2024

## Attendees
- CEO: John Smith
- CTO: Jane Doe
- CFO: Bob Johnson

## Key Decisions
1. **Budget Approval**: $500K for Q2 initiatives
2. **New Hire**: Approve 5 additional engineers
3. **Security**: Implement new encryption protocols

## Action Items
- [ ] Draft security policy (Jane)
- [ ] Prepare budget presentation (Bob)
- [ ] Schedule team interviews (John)

**Next Meeting**: February 15, 2024
EOF

echo "✅ Created sample files:"
ls -lh contract.txt financial_report.csv meeting_notes.md

# Step 2: Start Pipeline System (simulated)
echo ""
echo "🚀 Step 2: Starting Pipeline System..."
echo "📝 In a real scenario, you would run:"
echo "    pipeline start"
echo "    # Opens dashboard at http://localhost:8080"

# Step 3: Encryption Configuration
echo ""
echo "⚙️  Step 3: Encryption Configuration"
echo "📝 Web Interface Settings:"
echo "    Encryption Algorithm: AES-256-GCM"
echo "    Key Derivation: PBKDF2 (100,000 iterations)"
echo "    Password: [Strong password required]"
echo "    Key Rotation: 30 days"

# Step 4: Simulate Processing
echo ""
echo "🔄 Step 4: Processing Files..."
echo "Stage 1: File Validation ████████████ 100% (0.2s)"
echo "Stage 2: Key Generation ████████████ 100% (0.1s)"
echo "Stage 3: Encryption     ████████████ 100% (2.3s)"
echo "Stage 4: Verification   ████████████ 100% (0.4s)"
echo ""
echo "✅ Processing Complete: 3.0 seconds"

# Step 5: Create simulated output
echo ""
echo "📤 Step 5: Creating simulated encrypted output..."
mkdir -p output

# Simulate encrypted files (base64 encoded for demo)
echo "U2FsdGVkX1+vupppZksvRf5pq5g5XjFRIipRkwB0K1Y96Qsv2Lm+31cmzaAILwyt" > output/contract.txt.enc
echo "U2FsdGVkX19O5qNjRWPnP7u6LkMaJWQcRpS8Q+owPXU=" > output/financial_report.csv.enc
echo "U2FsdGVkX1+tsmZvCEIS7o8IJaZr6PqL8RhXhx+ijWk=" > output/meeting_notes.md.enc

# Create metadata files
cat > output/contract.txt.metadata << 'EOF'
{
  "original_file": "contract.txt",
  "encrypted_at": "2024-01-15T10:30:00Z",
  "algorithm": "AES-256-GCM",
  "key_id": "key_2024_001",
  "file_size_bytes": 456,
  "checksum_sha256": "a1b2c3d4e5f6..."
}
EOF

cat > output/encryption_summary.json << 'EOF'
{
  "files_processed": 3,
  "total_size_mb": 0.002,
  "encryption_algorithm": "AES-256-GCM",
  "processing_time_seconds": 3.0,
  "all_files_encrypted": true,
  "integrity_verified": true,
  "timestamp": "2024-01-15T10:30:00Z"
}
EOF

cat > output/checksums.txt << 'EOF'
a1b2c3d4e5f6789... contract.txt.enc
b2c3d4e5f6789a1... financial_report.csv.enc
c3d4e5f6789a1b2... meeting_notes.md.enc
EOF

echo "✅ Simulated encrypted files created:"
ls -la output/

# Step 6: Verification
echo ""
echo "🔍 Step 6: Verification"
echo "📊 Encryption Summary:"
cat output/encryption_summary.json | head -10

echo ""
echo "🔐 Security Best Practices:"
echo "  ✅ Use a password manager for encryption passwords"
echo "  ✅ Store encryption keys separately from encrypted files"
echo "  ✅ Limit access to encryption passwords"
echo "  ✅ Rotate passwords every 30-90 days"

echo ""
echo "⚠️  Important Notes:"
echo "  • Keep your password safe - files cannot be decrypted without it"
echo "  • Test decryption before deleting original files"
echo "  • Store encrypted files and keys in different locations"

echo ""
echo "🎉 Document Encryption Demo Complete!"
echo "📁 Demo files location: ~/encryption-demo/"
echo "📤 Encrypted output: ~/encryption-demo/output/"
echo ""
echo "🔗 For more details, see:"
echo "   - User Guide: pipeline/docs/USER_GUIDE.md"
echo "   - Quick Start: pipeline/docs/QUICK_START.md"
