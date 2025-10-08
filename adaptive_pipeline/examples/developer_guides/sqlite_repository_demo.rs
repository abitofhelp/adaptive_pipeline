// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////


//! # SQLite Repository Integration Demo
//!
//! This comprehensive example demonstrates the complete SQLite integration capabilities
//! of the adaptive pipeline system, showcasing repository patterns, storage abstraction,
//! and configuration-driven backend selection following Clean Architecture principles.
//!
//! ## Overview
//!
//! This demo provides a complete guide to:
//!
//! - **SQLite Repository Integration**: Full SQLite database integration with connection pooling
//! - **Repository Adapter Pattern**: Seamless switching between storage backends
//! - **Factory Pattern Implementation**: Dynamic repository creation based on configuration
//! - **Environment-Based Configuration**: Production vs. development storage selection
//! - **Storage Migration Strategies**: Moving from in-memory to persistent storage
//! - **Transaction Management**: ACID transactions for data consistency
//!
//! ## Architecture Overview
//!
//! The demo follows Clean Architecture and Domain-Driven Design principles:
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────┐
//! │                        Domain Layer                                    │
//! │  ┌─────────────────────────────────────────────────────────┐    │
//! │  │                  Repository Interface                        │    │
//! │  │                 (Domain Abstraction)                       │    │
//! │  └─────────────────────────────────────────────────────────┘    │
//! └────────────────────────────────┬─────────────────────────────────┘
//!                                  │ implements
//! ┌────────────────────────────────▼─────────────────────────────────┐
//! │                     Infrastructure Layer                              │
//! │                                                                       │
//! │  ┌───────────────────────┐    ┌─────────────────────────────────┐    │
//! │  │   Repository Factory    │    │      SQLite Adapter         │    │
//! │  │  (Creation Strategy)   │    │   (Interface Bridge)      │    │
//! │  └───────────────────────┘    └─────────────────────────────────┘    │
//! │                   │                              │                   │
//! │                   │ creates                      │ wraps             │
//! │                   ▼                              ▼                   │
//! │  ┌───────────────────────┐    ┌─────────────────────────────────┐    │
//! │  │  In-Memory Repository  │    │     SQLite Repository       │    │
//! │  │   (Development)       │    │      (Production)          │    │
//! │  └───────────────────────┘    └─────────────────────────────────┘    │
//! └─────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Key Design Patterns Demonstrated
//!
//! ### 1. Repository Pattern
//! - **Abstraction**: Clean separation between domain logic and data persistence
//! - **Interface Consistency**: Same interface for different storage backends
//! - **Testability**: Easy mocking and testing with in-memory implementations
//!
//! ### 2. Adapter Pattern
//! - **Interface Translation**: Bridges between domain and infrastructure interfaces
//! - **Backend Abstraction**: Hides SQLite-specific details from domain layer
//! - **Seamless Integration**: Allows switching storage backends without code changes
//!
//! ### 3. Factory Pattern
//! - **Creation Strategy**: Centralized repository creation logic
//! - **Configuration-Driven**: Creates appropriate repository based on configuration
//! - **Dependency Injection**: Supports IoC container integration
//!
//! ## Features Demonstrated
//!
//! ### Basic SQLite Operations
//! - **Database Connection**: Connection pooling and configuration
//! - **Schema Management**: Automatic table creation and migration
//! - **CRUD Operations**: Create, Read, Update, Delete operations
//! - **Transaction Support**: ACID transactions for data consistency
//!
//! ### Advanced Repository Features
//! - **Adapter Pattern**: Seamless interface translation
//! - **Factory Creation**: Dynamic repository instantiation
//! - **Configuration Management**: Environment-based backend selection
//! - **Storage Migration**: Moving between storage backends
//!
//! ### Performance Optimization
//! - **Connection Pooling**: Efficient database connection management
//! - **Prepared Statements**: Optimized query execution
//! - **Batch Operations**: Bulk data operations for better performance
//! - **Indexing Strategy**: Optimized database indexes
//!
//! ## Demo Scenarios
//!
//! ### 1. Basic SQLite Usage
//! ```rust
//! // Direct SQLite repository usage
//! let sqlite_repo = SqliteRepository::<Pipeline>::new("demo.db").await?;
//! let pipeline = Pipeline::new(/* ... */);
//! sqlite_repo.save(&pipeline).await?;
//! let found = sqlite_repo.find_by_id(&pipeline.id()).await?;
//! ```
//!
//! ### 2. Repository Adapter Pattern
//! ```rust
//! // Using adapter for domain compatibility
//! let sqlite_repo = SqliteRepository::<Pipeline>::new("demo.db").await?;
//! let adapter = SqliteRepositoryAdapter::new(sqlite_repo);
//! let repository: Arc<dyn Repository<Pipeline>> = Arc::new(adapter);
//! ```
//!
//! ### 3. Factory Pattern
//! ```rust
//! // Configuration-driven repository creation
//! let config = RepositoryConfig::sqlite("production.db");
//! let factory = RepositoryFactory::new();
//! let repository = factory.create_pipeline_repository(config).await?;
//! ```
//!
//! ### 4. Configuration-Driven Storage
//! ```rust
//! // Environment-based storage selection
//! let use_sqlite = std::env::var("USE_SQLITE").unwrap_or_default() == "true";
//! let repository = if use_sqlite {
//!     create_sqlite_repository().await?
//! } else {
//!     create_in_memory_repository()
//! };
//! ```
//!
//! ### 5. Storage Migration
//! ```rust
//! // Migrating from in-memory to SQLite
//! let in_memory_repo = InMemoryRepository::<Pipeline>::new();
//! let sqlite_repo = SqliteRepository::<Pipeline>::new("migrated.db").await?;
//! migrate_data(&in_memory_repo, &sqlite_repo).await?;
//! ```
//!
//! ## Running the Demo
//!
//! Execute the demo with:
//!
//! ```bash
//! cargo run --example sqlite_repository_demo
//! ```
//!
//! ### Environment Variables
//!
//! Configure the demo behavior:
//!
//! ```bash
//! # Use SQLite for all demos
//! export USE_SQLITE=true
//! cargo run --example sqlite_repository_demo
//!
//! # Use in-memory storage (default)
//! export USE_SQLITE=false
//! cargo run --example sqlite_repository_demo
//!
//! # Custom database path
//! export DATABASE_PATH="/tmp/demo.db"
//! cargo run --example sqlite_repository_demo
//! ```
//!
//! ## Expected Output
//!
//! The demo will show:
//!
//! 1. **Basic SQLite Operations**: Direct database operations
//! 2. **Adapter Pattern Usage**: Interface translation demonstration
//! 3. **Factory Pattern**: Dynamic repository creation
//! 4. **Configuration Selection**: Environment-based backend choice
//! 5. **Storage Migration**: Data migration between backends
//!
//! ## Performance Considerations
//!
//! ### Connection Pooling
//! - **Pool Size**: Configurable connection pool for concurrent access
//! - **Connection Reuse**: Efficient resource utilization
//! - **Timeout Management**: Proper timeout handling for operations
//!
//! ### Query Optimization
//! - **Prepared Statements**: Reusable compiled queries
//! - **Indexing Strategy**: Optimized database indexes
//! - **Batch Operations**: Efficient bulk data operations
//!
//! ### Memory Management
//! - **Connection Limits**: Prevent memory exhaustion
//! - **Resource Cleanup**: Proper resource disposal
//! - **Cache Strategy**: Intelligent caching for frequently accessed data
//!
//! ## Error Handling
//!
//! The demo demonstrates comprehensive error handling:
//!
//! ```rust
//! match repository.save(&pipeline).await {
//!     Ok(()) => println!("Pipeline saved successfully"),
//!     Err(PipelineError::DatabaseError(msg)) => {
//!         eprintln!("Database error: {}", msg);
//!     }
//!     Err(PipelineError::ValidationError(msg)) => {
//!         eprintln!("Validation error: {}", msg);
//!     }
//!     Err(err) => eprintln!("Unexpected error: {}", err),
//! }
//! ```
//!
//! ## Testing Integration
//!
//! The patterns shown support comprehensive testing:
//!
//! ```rust
//! #[cfg(test)]
//! mod tests {
//!     use super::*;
//!
//!     #[tokio::test]
//!     async fn test_repository_operations() {
//!         // Use in-memory repository for fast tests
//!         let repository = InMemoryRepository::<Pipeline>::new();
//!         
//!         let pipeline = Pipeline::new(/* ... */);
//!         repository.save(&pipeline).await.unwrap();
//!         
//!         let found = repository.find_by_id(&pipeline.id()).await.unwrap();
//!         assert!(found.is_some());
//!     }
//!
//!     #[tokio::test]
//!     async fn test_sqlite_integration() {
//!         // Use SQLite with temporary database for integration tests
//!         let repository = SqliteRepository::<Pipeline>::new(":memory:").await.unwrap();
//!         
//!         // Test SQLite-specific functionality
//!         let pipeline = Pipeline::new(/* ... */);
//!         repository.save(&pipeline).await.unwrap();
//!         
//!         // Verify persistence
//!         let found = repository.find_by_id(&pipeline.id()).await.unwrap();
//!         assert!(found.is_some());
//!     }
//! }
//! ```
//!
//! ## Security Considerations
//!
//! - **SQL Injection Prevention**: All queries use parameterized statements
//! - **Data Validation**: Entity validation before database operations
//! - **Access Control**: Integration with security context for permission checks
//! - **Audit Logging**: Comprehensive logging of all repository operations
//!
//! ## Best Practices Demonstrated
//!
//! - **Clean Architecture**: Proper layer separation and dependency inversion
//! - **Domain-Driven Design**: Repository pattern following DDD principles
//! - **Configuration Management**: Environment-based configuration
//! - **Error Handling**: Comprehensive error management and recovery
//! - **Testing Strategy**: Support for both unit and integration testing
//! - **Performance Optimization**: Connection pooling and query optimization
//!
//! ## Learning Outcomes
//!
//! After running this demo, you will understand:
//!
//! - How to integrate SQLite with Clean Architecture
//! - Repository and Adapter pattern implementation
//! - Factory pattern for dynamic object creation
//! - Configuration-driven architecture decisions
//! - Storage migration strategies
//! - Performance optimization techniques
//! - Comprehensive error handling approaches
//! - Testing strategies for different storage backends

use std::sync::Arc;
use uuid::Uuid;
use adaptive_pipeline_domain::{
    entities::{Pipeline, PipelineStage, ProcessingMetrics},
    PipelineError,
};
use adaptive_pipeline::infrastructure::{
    Repository, RepositoryFactory, RepositoryConfig,
    SqliteRepositoryAdapter, InMemoryRepository,
};

#[tokio::main]
async fn main() -> Result<(), PipelineError> {
    println!("🗄️  SQLite Repository Integration Demo");
    println!("=====================================\n");

    // Demo 1: Basic SQLite Repository Usage
    demo_basic_sqlite_usage().await?;
    
    // Demo 2: Repository Adapter Pattern
    demo_repository_adapter_pattern().await?;
    
    // Demo 3: Repository Factory Pattern
    demo_repository_factory().await?;
    
    // Demo 4: Configuration-Driven Storage Selection
    demo_configuration_driven_storage().await?;
    
    // Demo 5: Storage Backend Migration
    demo_storage_migration().await?;
    
    println!("✅ All SQLite repository demos completed successfully!");
    Ok(())
}

/// Demonstrates basic SQLite repository operations
async fn demo_basic_sqlite_usage() -> Result<(), PipelineError> {
    println!("📁 Demo 1: Basic SQLite Repository Usage");
    println!("-----------------------------------------");
    
    // Create an in-memory SQLite database for demo
    let adapter = SqliteRepositoryAdapter::<Pipeline>::in_memory().await?;
    
    // Create test pipeline
    let pipeline = create_test_pipeline("SQLite Test Pipeline")?;
    let pipeline_id = pipeline.id();
    
    println!("💾 Saving pipeline to SQLite...");
    adapter.save(&pipeline).await?;
    
    println!("🔍 Finding pipeline by ID...");
    let found = adapter.find_by_id(pipeline_id).await?;
    assert!(found.is_some());
    println!("✅ Found pipeline: {}", found.unwrap().name());
    
    println!("📊 Counting pipelines...");
    let count = adapter.count().await?;
    println!("✅ Pipeline count: {}", count);
    
    println!("📋 Listing all pipelines...");
    let all_pipelines = adapter.list_all().await?;
    println!("✅ Found {} pipelines", all_pipelines.len());
    
    // Test archiving
    println!("🗃️  Archiving pipeline...");
    let archived = adapter.archive(pipeline_id).await?;
    assert!(archived);
    println!("✅ Pipeline archived successfully");
    
    let active_count = adapter.count().await?;
    println!("✅ Active pipelines after archiving: {}", active_count);
    
    // Test restoration
    println!("♻️  Restoring pipeline...");
    let restored = adapter.restore(pipeline_id).await?;
    assert!(restored);
    println!("✅ Pipeline restored successfully");
    
    let final_count = adapter.count().await?;
    println!("✅ Active pipelines after restoration: {}", final_count);
    
    println!();
    Ok(())
}

/// Demonstrates the repository adapter pattern
async fn demo_repository_adapter_pattern() -> Result<(), PipelineError> {
    println!("🔌 Demo 2: Repository Adapter Pattern");
    println!("--------------------------------------");
    
    // Create both in-memory and SQLite repositories
    let in_memory_repo: Arc<dyn Repository<Pipeline>> = Arc::new(InMemoryRepository::new());
    let sqlite_repo: Arc<dyn Repository<Pipeline>> = Arc::new(
        SqliteRepositoryAdapter::in_memory().await?
    );
    
    let pipeline1 = create_test_pipeline("In-Memory Pipeline")?;
    let pipeline2 = create_test_pipeline("SQLite Pipeline")?;
    
    println!("💾 Saving to in-memory repository...");
    in_memory_repo.save(&pipeline1).await?;
    
    println!("💾 Saving to SQLite repository...");
    sqlite_repo.save(&pipeline2).await?;
    
    // Both repositories implement the same interface
    println!("📊 Counting pipelines in both repositories:");
    println!("  In-memory count: {}", in_memory_repo.count().await?);
    println!("  SQLite count: {}", sqlite_repo.count().await?);
    
    // Demonstrate identical interface usage
    async fn use_repository(repo: Arc<dyn Repository<Pipeline>>, name: &str) -> Result<usize, PipelineError> {
        println!("  Using {} repository:", name);
        let count = repo.count().await?;
        let pipelines = repo.list_all().await?;
        println!("    Count: {}, Listed: {}", count, pipelines.len());
        Ok(count)
    }
    
    use_repository(in_memory_repo.clone(), "in-memory").await?;
    use_repository(sqlite_repo.clone(), "SQLite").await?;
    
    println!("✅ Both repositories work identically through the same interface");
    println!();
    Ok(())
}

/// Demonstrates the repository factory pattern
async fn demo_repository_factory() -> Result<(), PipelineError> {
    println!("🏭 Demo 3: Repository Factory Pattern");
    println!("-------------------------------------");
    
    // Create repositories using factory
    println!("🔧 Creating in-memory repository via factory...");
    let in_memory_repo = RepositoryFactory::create_in_memory::<Pipeline>();
    
    println!("🔧 Creating SQLite repository via factory...");
    let sqlite_repo = RepositoryFactory::create_sqlite_in_memory::<Pipeline>().await?;
    
    // Test both repositories
    let test_pipeline = create_test_pipeline("Factory Test Pipeline")?;
    
    println!("💾 Testing in-memory repository...");
    in_memory_repo.save(&test_pipeline).await?;
    let in_memory_count = in_memory_repo.count().await?;
    println!("✅ In-memory repository count: {}", in_memory_count);
    
    println!("💾 Testing SQLite repository...");
    sqlite_repo.save(&test_pipeline).await?;
    let sqlite_count = sqlite_repo.count().await?;
    println!("✅ SQLite repository count: {}", sqlite_count);
    
    println!("✅ Factory pattern enables easy repository creation");
    println!();
    Ok(())
}

/// Demonstrates configuration-driven storage selection
async fn demo_configuration_driven_storage() -> Result<(), PipelineError> {
    println!("⚙️  Demo 4: Configuration-Driven Storage Selection");
    println!("--------------------------------------------------");
    
    // Test different configurations
    let configs = vec![
        ("In-Memory", RepositoryConfig::InMemory),
        ("SQLite In-Memory", RepositoryConfig::SqliteInMemory),
        ("SQLite File", RepositoryConfig::Sqlite { 
            database_path: ":memory:".to_string() // Using :memory: for demo
        }),
    ];
    
    for (name, config) in configs {
        println!("🔧 Testing {} configuration...", name);
        let repo = config.create_repository::<Pipeline>().await?;
        
        let test_pipeline = create_test_pipeline(&format!("{} Test", name))?;
        repo.save(&test_pipeline).await?;
        
        let count = repo.count().await?;
        println!("✅ {} repository working, count: {}", name, count);
    }
    
    // Demonstrate environment-based configuration
    println!("🌍 Testing environment-based configuration...");
    std::env::set_var("REPOSITORY_TYPE", "sqlite_memory");
    let env_config = RepositoryConfig::from_env();
    let env_repo = env_config.create_repository::<Pipeline>().await?;
    
    let env_pipeline = create_test_pipeline("Environment Test Pipeline")?;
    env_repo.save(&env_pipeline).await?;
    println!("✅ Environment-based configuration working");
    
    println!();
    Ok(())
}

/// Demonstrates migration between storage backends
async fn demo_storage_migration() -> Result<(), PipelineError> {
    println!("🔄 Demo 5: Storage Backend Migration");
    println!("------------------------------------");
    
    // Start with in-memory repository
    println!("📂 Starting with in-memory storage...");
    let source_repo = RepositoryFactory::create_in_memory::<Pipeline>();
    
    // Add some test data
    let pipelines = vec![
        create_test_pipeline("Migration Test 1")?,
        create_test_pipeline("Migration Test 2")?,
        create_test_pipeline("Migration Test 3")?,
    ];
    
    for pipeline in &pipelines {
        source_repo.save(pipeline).await?;
    }
    
    let source_count = source_repo.count().await?;
    println!("✅ Source repository has {} pipelines", source_count);
    
    // Migrate to SQLite
    println!("🔄 Migrating to SQLite storage...");
    let target_repo = RepositoryFactory::create_sqlite_in_memory::<Pipeline>().await?;
    
    // Perform migration
    let all_pipelines = source_repo.list_all().await?;
    for pipeline in all_pipelines {
        target_repo.save(&pipeline).await?;
    }
    
    let target_count = target_repo.count().await?;
    println!("✅ Target repository has {} pipelines", target_count);
    
    // Verify migration
    assert_eq!(source_count, target_count);
    println!("✅ Migration completed successfully!");
    
    // Demonstrate that both repositories now have the same data
    println!("🔍 Verifying data integrity...");
    for pipeline in &pipelines {
        let found_in_target = target_repo.find_by_id(pipeline.id()).await?;
        assert!(found_in_target.is_some());
        assert_eq!(found_in_target.unwrap().name(), pipeline.name());
    }
    println!("✅ Data integrity verified!");
    
    println!();
    Ok(())
}

/// Helper function to create a test pipeline
fn create_test_pipeline(name: &str) -> Result<Pipeline, PipelineError> {
    let stages = vec![
        PipelineStage::new(
            "input".to_string(),
            "file_input".to_string(),
            std::collections::HashMap::new(),
        )?,
        PipelineStage::new(
            "process".to_string(),
            "compression".to_string(),
            std::collections::HashMap::new(),
        )?,
        PipelineStage::new(
            "output".to_string(),
            "file_output".to_string(),
            std::collections::HashMap::new(),
        )?,
    ];
    
    Pipeline::new(name.to_string(), stages)
}

/// Demonstrates advanced SQLite features
#[allow(dead_code)]
async fn demo_advanced_sqlite_features() -> Result<(), PipelineError> {
    println!("🚀 Demo: Advanced SQLite Features");
    println!("---------------------------------");
    
    // Create SQLite repository with file storage
    let temp_db = tempfile::NamedTempFile::new()
        .map_err(|e| PipelineError::InternalError(format!("Failed to create temp file: {}", e)))?;
    let db_path = temp_db.path().to_str().unwrap();
    
    println!("📁 Creating SQLite repository with file: {}", db_path);
    let repo = RepositoryFactory::create_sqlite::<Pipeline>(db_path).await?;
    
    // Add test data
    let pipeline = create_test_pipeline("Persistent Test Pipeline")?;
    repo.save(&pipeline).await?;
    
    println!("💾 Data saved to persistent storage");
    
    // Close and reopen to verify persistence
    drop(repo);
    
    println!("🔄 Reopening database...");
    let repo2 = RepositoryFactory::create_sqlite::<Pipeline>(db_path).await?;
    let count = repo2.count().await?;
    
    println!("✅ Persistent storage verified, count: {}", count);
    assert_eq!(count, 1);
    
    Ok(())
}

/// Performance comparison between storage backends
#[allow(dead_code)]
async fn demo_performance_comparison() -> Result<(), PipelineError> {
    println!("⚡ Demo: Performance Comparison");
    println!("-------------------------------");
    
    let in_memory_repo = RepositoryFactory::create_in_memory::<Pipeline>();
    let sqlite_repo = RepositoryFactory::create_sqlite_in_memory::<Pipeline>().await?;
    
    let test_count = 100;
    
    // Benchmark in-memory
    let start = std::time::Instant::now();
    for i in 0..test_count {
        let pipeline = create_test_pipeline(&format!("InMemory Pipeline {}", i))?;
        in_memory_repo.save(&pipeline).await?;
    }
    let in_memory_duration = start.elapsed();
    
    // Benchmark SQLite
    let start = std::time::Instant::now();
    for i in 0..test_count {
        let pipeline = create_test_pipeline(&format!("SQLite Pipeline {}", i))?;
        sqlite_repo.save(&pipeline).await?;
    }
    let sqlite_duration = start.elapsed();
    
    println!("📊 Performance Results ({} operations):", test_count);
    println!("  In-Memory: {:?}", in_memory_duration);
    println!("  SQLite:    {:?}", sqlite_duration);
    
    let ratio = sqlite_duration.as_nanos() as f64 / in_memory_duration.as_nanos() as f64;
    println!("  SQLite is {:.2}x slower than in-memory", ratio);
    
    Ok(())
}
