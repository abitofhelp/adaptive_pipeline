#!/bin/bash

# Pipeline Examples Runner
# Interactive script to help users discover and run examples

set -e

echo "🚀 Pipeline System Examples"
echo "=========================="
echo ""

# Function to show menu
show_menu() {
    echo "📚 Available Example Categories:"
    echo ""
    echo "1. 👤 User Walkthroughs (Interactive Demos)"
    echo "   - Document encryption demo"
    echo "   - Sales analytics demo"
    echo ""
    echo "2. 👨‍💻 Developer Guides (Code Examples)"
    echo "   - File I/O operations"
    echo "   - Generic service patterns"
    echo "   - Database operations"
    echo ""
    echo "3. 🔗 Integration Examples (End-to-End)"
    echo "   - Complete file processing"
    echo ""
    echo "4. 📊 Sample Data (Test Files)"
    echo "   - Browse available test data"
    echo ""
    echo "5. 📖 Documentation Links"
    echo ""
    echo "0. Exit"
    echo ""
}

# Function to run user walkthroughs
run_user_walkthroughs() {
    echo "👤 User Walkthroughs"
    echo "==================="
    echo ""
    echo "1. Document Encryption Demo"
    echo "2. Sales Analytics Demo"
    echo "3. Back to main menu"
    echo ""
    read -p "Choose an option (1-3): " choice
    
    case $choice in
        1)
            echo "🔐 Running Document Encryption Demo..."
            chmod +x user_walkthroughs/document_encryption_demo.sh
            ./user_walkthroughs/document_encryption_demo.sh
            ;;
        2)
            echo "📊 Running Sales Analytics Demo..."
            chmod +x user_walkthroughs/sales_analytics_demo.sh
            ./user_walkthroughs/sales_analytics_demo.sh
            ;;
        3)
            return
            ;;
        *)
            echo "❌ Invalid option"
            ;;
    esac
}

# Function to run developer guides
run_developer_guides() {
    echo "👨‍💻 Developer Guides"
    echo "==================="
    echo ""
    echo "Available Rust examples:"
    echo "1. File I/O Demo"
    echo "2. Generic Service Demo"
    echo "3. Generic Compression Demo"
    echo "4. SQLite Repository Demo"
    echo "5. List all available examples"
    echo "6. Back to main menu"
    echo ""
    read -p "Choose an option (1-6): " choice
    
    case $choice in
        1)
            echo "📁 Running File I/O Demo..."
            cd .. && cargo run --example file_io_demo
            ;;
        2)
            echo "⚙️ Running Generic Service Demo..."
            cd .. && cargo run --example generic_service_demo
            ;;
        3)
            echo "🗜️ Running Generic Compression Demo..."
            cd .. && cargo run --example generic_compression_demo
            ;;
        4)
            echo "🗄️ Running SQLite Repository Demo..."
            cd .. && cargo run --example sqlite_repository_demo
            ;;
        5)
            echo "📋 Available Rust examples:"
            cd .. && cargo run --example 2>&1 | grep "Available examples:" -A 20 || echo "Run 'cargo run --example' from the pipeline directory"
            ;;
        6)
            return
            ;;
        *)
            echo "❌ Invalid option"
            ;;
    esac
}

# Function to run integration examples
run_integration_examples() {
    echo "🔗 Integration Examples"
    echo "======================"
    echo ""
    echo "1. Complete File Processing Demo"
    echo "2. Back to main menu"
    echo ""
    read -p "Choose an option (1-2): " choice
    
    case $choice in
        1)
            echo "🔄 Running Complete File Processing Demo..."
            cd .. && cargo run --example complete_file_processing_demo
            ;;
        2)
            return
            ;;
        *)
            echo "❌ Invalid option"
            ;;
    esac
}

# Function to browse sample data
browse_sample_data() {
    echo "📊 Sample Data"
    echo "============="
    echo ""
    echo "Available test data:"
    echo ""
    
    if [ -d "sample_data" ]; then
        echo "📁 Contracts:"
        ls -la sample_data/contracts/ 2>/dev/null || echo "   (No contract samples yet)"
        echo ""
        echo "📁 Sales Data:"
        ls -la sample_data/sales_data/ 2>/dev/null || echo "   (No sales data samples yet)"
        echo ""
        echo "📁 Images:"
        ls -la sample_data/images/ 2>/dev/null || echo "   (No image samples yet)"
    else
        echo "❌ Sample data directory not found"
    fi
    
    echo ""
    echo "📖 For more details, see: sample_data/README.md"
    echo ""
    read -p "Press Enter to continue..."
}

# Function to show documentation links
show_documentation() {
    echo "📖 Documentation Links"
    echo "====================="
    echo ""
    echo "📚 User Documentation:"
    echo "  • Quick Start Guide: ../docs/QUICK_START.md"
    echo "  • User Guide: ../docs/USER_GUIDE.md"
    echo ""
    echo "🏗️ Technical Documentation:"
    echo "  • Architecture: ../../docs/architecture/"
    echo "  • Requirements: ../../docs/requirements/"
    echo "  • Design: ../../docs/design/"
    echo ""
    echo "💻 Developer Resources:"
    echo "  • API Reference: ../docs/API_REFERENCE.md (when available)"
    echo "  • Code Examples: developer_guides/"
    echo "  • Integration Examples: integration_examples/"
    echo ""
    echo "🔗 Online Resources:"
    echo "  • GitHub Repository: https://github.com/abitofhelp/optimized_adaptive_pipeline_rs"
    echo "  • Issues & Support: GitHub Issues"
    echo ""
    read -p "Press Enter to continue..."
}

# Main loop
while true; do
    clear
    show_menu
    read -p "Choose an option (0-5): " choice
    
    case $choice in
        1)
            clear
            run_user_walkthroughs
            echo ""
            read -p "Press Enter to continue..."
            ;;
        2)
            clear
            run_developer_guides
            echo ""
            read -p "Press Enter to continue..."
            ;;
        3)
            clear
            run_integration_examples
            echo ""
            read -p "Press Enter to continue..."
            ;;
        4)
            clear
            browse_sample_data
            ;;
        5)
            clear
            show_documentation
            ;;
        0)
            echo "👋 Thanks for exploring the pipeline examples!"
            exit 0
            ;;
        *)
            echo "❌ Invalid option. Please choose 0-5."
            sleep 2
            ;;
    esac
done
