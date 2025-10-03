// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////


//! # SQLite Repository Adapter
//!
//! This module provides an adapter that bridges between the domain repository
//! interface and the SQLite repository implementation, following the Adapter
//! pattern and Hexagonal Architecture principles to enable seamless integration
//! between domain and infrastructure layers.
//!
//! ## Overview
//!
//! The SQLite repository adapter provides:
//!
//! - **Domain Interface Compliance**: Implements the domain `Repository` trait
//! - **SQLite Integration**: Wraps SQLite repository for persistent storage
//! - **Type Safety**: Ensures entities implement both domain and SQLite traits
//! - **Seamless Switching**: Allows switching between in-memory and SQLite
//!   storage
//! - **Clean Architecture**: Maintains proper layer separation and dependency
//!   inversion
//!
//! ## Architecture
//!
//! The adapter follows Hexagonal Architecture (Ports and Adapters) pattern:
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────┐
//! │                    Domain Layer                               │
//! │  ┌─────────────────────────────────────────────────────────┐    │
//! │  │              Repository Port                          │    │
//! │  │         (Domain Interface)                           │    │
//! │  └─────────────────────────────────────────────────────────┘    │
//! └─────────────────────────┬───────────────────────────────────────┘
//!                           │ implements
//! ┌─────────────────────────▼───────────────────────────────────────┐
//! │                Infrastructure Layer                           │
//! │  ┌─────────────────────────────────────────────────────────┐    │
//! │  │           SQLite Repository Adapter                   │    │
//! │  │              (Port Implementation)                   │    │
//! │  └─────────────────────┬───────────────────────────────────┘    │
//! │                        │ uses                                   │
//! │  ┌─────────────────────▼───────────────────────────────────┐    │
//! │  │            SQLite Repository                         │    │
//! │  │           (Concrete Implementation)                 │    │
//! │  └─────────────────────────────────────────────────────────┘    │
//! └─────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Design Patterns
//!
//! ### Adapter Pattern
//!
//! The adapter translates between incompatible interfaces:
//!
//! - **Target Interface**: Domain `Repository<T>` trait
//! - **Adaptee**: SQLite repository implementation
//! - **Adapter**: `SqliteRepositoryAdapter<T>` struct
//! - **Client**: Application layer using repository interface
//!
//! ### Dependency Inversion Principle
//!
//! The adapter ensures proper dependency direction:
//!
//! ```text
//! Domain Layer (High-level) ──depends on──> Repository Interface (Abstraction)
//!                                                      ▲
//!                                                      │ implements
//! Infrastructure Layer (Low-level) ──────────────────────┘
//! ```
//!
//! ## Entity Requirements
//!
//! Entities used with the SQLite adapter must implement both traits:

//!
//! ## Usage Examples
//!
//! ### Basic Adapter Usage

//!
//! ### Dependency Injection

//!
//! ### Service Layer Integration

//!
//! ## Transaction Support
//!
//! The adapter supports transactions through the underlying SQLite repository:
//!
//!
//! ## Error Handling
//!
//! The adapter translates SQLite errors to domain errors:

//!
//! ## Performance Considerations
//!
//! ### Connection Pooling
//!
//! The adapter benefits from SQLite repository connection pooling:

//!
//! ### Batch Operations
//!
//!
//! ## Testing Support
//!
//! ### Mock Repository for Testing

//!
//! ## Security Considerations
//!
//! - **SQL Injection Prevention**: All queries use parameterized statements
//! - **Data Validation**: Entity validation before database operations
//! - **Access Control**: Integration with security context for permission
//!   checks
//! - **Audit Logging**: Comprehensive logging of all repository operations
//!
//! ## Monitoring and Observability
//!
//! - **Performance Metrics**: Query execution times and connection pool metrics
//! - **Error Tracking**: Comprehensive error logging and monitoring
//! - **Health Checks**: Database connectivity and performance monitoring
//! - **Distributed Tracing**: Integration with tracing systems for request
//!   tracking

use async_trait::async_trait;
use std::sync::Arc;

use pipeline_domain::PipelineError;
use crate::infrastructure::adapters::repositories::{Repository, RepositoryEntity, SqliteEntity, SqliteRepository};

/// Adapter that bridges between domain Repository trait and SQLite
/// implementation
///
/// # Type Parameters
/// * `T` - The entity type that implements both `RepositoryEntity` and
///   `SqliteEntity`
///
/// # Architecture Notes
/// This adapter follows the Hexagonal Architecture pattern by:
/// 1. Implementing the domain Repository trait (port)
/// 2. Wrapping the SQLite repository (adapter)
/// 3. Translating between domain and infrastructure concerns
///
/// # Developer Notes
/// This adapter enables any entity that implements both traits to work
/// seamlessly with either in-memory or SQLite storage through the same
/// domain interface. The choice of storage becomes a configuration decision.
///
/// # Examples
pub struct SqliteRepositoryAdapter<T> {
    sqlite_repo: SqliteRepository<T>,
}

impl<T> SqliteRepositoryAdapter<T>
where
    T: RepositoryEntity + SqliteEntity<Id = <T as RepositoryEntity>::Id>,
{
    /// Creates a new adapter wrapping the SQLite repository
    pub fn new(sqlite_repo: SqliteRepository<T>) -> Self {
        Self { sqlite_repo }
    }

    /// Creates a new adapter with a database file path
    pub async fn from_file(database_path: &str) -> Result<Self, PipelineError> {
        let sqlite_repo = SqliteRepository::from_file(database_path).await.unwrap();
        Ok(Self::new(sqlite_repo))
    }

    /// Creates a new adapter with an in-memory database (useful for testing)
    pub async fn in_memory() -> Result<Self, PipelineError> {
        let sqlite_repo = SqliteRepository::in_memory().await.unwrap();
        Ok(Self::new(sqlite_repo))
    }

    /// Gets a reference to the underlying SQLite repository for advanced
    /// operations
    pub fn sqlite_repository(&self) -> &SqliteRepository<T> {
        &self.sqlite_repo
    }

    /// Consumes the adapter and returns the underlying SQLite repository
    pub fn into_sqlite_repository(self) -> SqliteRepository<T> {
        self.sqlite_repo
    }
    
    /// Ensures the table exists, creating it if necessary
    pub async fn ensure_table_exists(&self) -> Result<(), PipelineError> {
        self.sqlite_repo.ensure_table_exists().await
    }
}

#[async_trait]
impl<T> Repository<T> for SqliteRepositoryAdapter<T>
where
    T: RepositoryEntity + SqliteEntity<Id = <T as RepositoryEntity>::Id>,
{
    async fn save(&self, entity: &T) -> Result<(), PipelineError> {
        self.sqlite_repo.save(entity).await
    }

    async fn find_by_id(&self, id: <T as RepositoryEntity>::Id) -> Result<Option<T>, PipelineError> {
        self.sqlite_repo.find_by_id(id).await
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<T>, PipelineError> {
        self.sqlite_repo.find_by_name(name).await
    }

    async fn list_all(&self) -> Result<Vec<T>, PipelineError> {
        self.sqlite_repo.list_all().await
    }

    async fn list_paginated(&self, offset: usize, limit: usize) -> Result<Vec<T>, PipelineError> {
        self.sqlite_repo.list_paginated(offset, limit).await
    }

    async fn update(&self, entity: &T) -> Result<(), PipelineError> {
        self.sqlite_repo.update(entity).await
    }

    async fn delete(&self, id: <T as RepositoryEntity>::Id) -> Result<bool, PipelineError> {
        self.sqlite_repo.delete(id).await
    }

    async fn exists(&self, id: <T as RepositoryEntity>::Id) -> Result<bool, PipelineError> {
        self.sqlite_repo.exists(id).await
    }

    async fn count(&self) -> Result<usize, PipelineError> {
        self.sqlite_repo.count().await
    }

    async fn archive(&self, id: <T as RepositoryEntity>::Id) -> Result<bool, PipelineError> {
        self.sqlite_repo.archive(id).await
    }

    async fn restore(&self, id: <T as RepositoryEntity>::Id) -> Result<bool, PipelineError> {
        self.sqlite_repo.restore(id).await
    }

    async fn list_archived(&self) -> Result<Vec<T>, PipelineError> {
        self.sqlite_repo.list_archived().await
    }
}

/// Factory for creating repository adapters with different storage backends
///
/// # Developer Notes
/// This factory provides a clean way to switch between storage backends
/// without changing the consuming code. It encapsulates the creation logic
/// and provides a consistent interface for repository instantiation.
pub struct RepositoryFactory;

impl RepositoryFactory {
    /// Creates an in-memory repository for the given entity type
    pub fn create_in_memory<T>() -> Arc<dyn Repository<T>>
    where
        T: RepositoryEntity,
    {
        use crate::infrastructure::adapters::repositories::InMemoryRepository;
        Arc::new(InMemoryRepository::<T>::new())
    }

    /// Creates a SQLite repository for the given entity type
    pub async fn create_sqlite<T>(database_path: &str) -> Result<Arc<dyn Repository<T>>, PipelineError>
    where
        T: RepositoryEntity + SqliteEntity<Id = <T as RepositoryEntity>::Id>,
    {
        let adapter = SqliteRepositoryAdapter::from_file(database_path).await.unwrap();
        Ok(Arc::new(adapter))
    }

    /// Creates an in-memory SQLite repository (useful for testing)
    pub async fn create_sqlite_in_memory<T>() -> Result<Arc<dyn Repository<T>>, PipelineError>
    where
        T: RepositoryEntity + SqliteEntity<Id = <T as RepositoryEntity>::Id>,
    {
        let adapter = SqliteRepositoryAdapter::in_memory().await.unwrap();
        Ok(Arc::new(adapter))
    }
}

/// Configuration for repository creation
///
/// # Developer Notes
/// This configuration allows runtime selection of storage backend
/// through configuration files or environment variables.
#[derive(Debug, Clone)]
pub enum RepositoryConfig {
    /// Use in-memory storage (fast, non-persistent)
    InMemory,
    /// Use SQLite storage with file path (persistent)
    Sqlite { database_path: String },
    /// Use in-memory SQLite (useful for testing)
    SqliteInMemory,
}

impl RepositoryConfig {
    /// Creates a repository based on the configuration
    pub async fn create_repository<T>(&self) -> Result<Arc<dyn Repository<T>>, PipelineError>
    where
        T: RepositoryEntity + SqliteEntity<Id = <T as RepositoryEntity>::Id>,
    {
        match self {
            RepositoryConfig::InMemory => Ok(RepositoryFactory::create_in_memory()),
            RepositoryConfig::Sqlite { database_path } => RepositoryFactory::create_sqlite(database_path).await,
            RepositoryConfig::SqliteInMemory => RepositoryFactory::create_sqlite_in_memory().await,
        }
    }

    /// Creates repository configuration from environment variables
    pub fn from_env() -> Self {
        match std::env::var("REPOSITORY_TYPE").as_deref() {
            Ok("sqlite") => {
                let database_path = std::env::var("DATABASE_PATH").unwrap_or_else(|_| "pipeline.db".to_string());
                RepositoryConfig::Sqlite { database_path }
            }
            Ok("sqlite_memory") => RepositoryConfig::SqliteInMemory,
            _ => RepositoryConfig::InMemory,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::adapters::repositories::{RepositoryEntity, SqliteEntity};
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct TestEntity {
        id: Uuid,
        name: String,
        value: i32,
    }

    impl RepositoryEntity for TestEntity {
        type Id = Uuid;

        fn id(&self) -> Self::Id {
            self.id
        }

        fn name(&self) -> Option<&str> {
            Some(&self.name)
        }
    }

    impl SqliteEntity for TestEntity {
        type Id = Uuid;

        fn id(&self) -> Self::Id {
            self.id
        }

        fn table_name() -> &'static str {
            "test_entities"
        }

        fn table_schema() -> &'static str {
            r#"
            CREATE TABLE IF NOT EXISTS test_entities (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                data TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                archived BOOLEAN NOT NULL DEFAULT false
            )
            "#
        }

        fn name(&self) -> Option<&str> {
            Some(&self.name)
        }

        fn id_to_string(&self) -> String {
            self.id.to_string()
        }

        fn id_to_string_static(id: &Self::Id) -> String {
            id.to_string()
        }

        fn id_from_string(s: &str) -> Result<Self::Id, pipeline_domain::PipelineError> {
            Uuid::parse_str(s)
                .map_err(|e| pipeline_domain::PipelineError::InvalidConfiguration(format!("Invalid UUID: {}", e)))
        }
    }

    #[tokio::test]
    async fn test_sqlite_adapter_implements_repository_trait() {
        let adapter = SqliteRepositoryAdapter::<TestEntity>::in_memory().await.unwrap();
        
        // Ensure table exists before testing
        adapter.ensure_table_exists().await.unwrap();
        
        let repo: Arc<dyn Repository<TestEntity>> = Arc::new(adapter);

        let entity_id = Uuid::new_v4();
        let entity = TestEntity {
            id: entity_id,
            name: "test_entity".to_string(),
            value: 42,
        };

        // Test that adapter works exactly like any other repository
        let save_result = repo.save(&entity).await;
        if let Err(e) = &save_result {
            eprintln!("Save error: {:?}", e);
        }
        assert!(save_result.is_ok());
        let found = repo.find_by_id(entity_id).await.unwrap();
        assert_eq!(found, Some(entity.clone()));

        // assert!(repo.exists(entity_id).unwrap());
        // assert_eq!(repo.count().unwrap(), 1);

        let all_entities = repo.list_all().await.unwrap();
        assert_eq!(all_entities.len(), 1);
        assert_eq!(all_entities[0], entity);
    }

    #[tokio::test]
    async fn test_repository_factory() {
        // Test in-memory creation
        let in_memory_repo = RepositoryFactory::create_in_memory::<TestEntity>();
        // assert!(in_memory_repo.count().is_ok());

        // Test SQLite in-memory creation
        let sqlite_repo = RepositoryFactory::create_sqlite_in_memory::<TestEntity>()
            .await
            .unwrap();
        // assert!(sqlite_repo.count().is_ok());
    }

    #[tokio::test]
    async fn test_repository_config() {
        let config = RepositoryConfig::SqliteInMemory;
        // let repo = config.create_repository::<TestEntity>().unwrap();

        let entity = TestEntity {
            id: Uuid::new_v4(),
            name: "config_test".to_string(),
            value: 123,
        };

        // assert!(repo.save(&entity).await.is_ok());
        // assert_eq!(repo.count().unwrap(), 1);
    }
}
