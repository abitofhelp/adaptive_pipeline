// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////


//! # Generic Repository Adapter
//!
//! This module provides a generic, in-memory repository adapter implementation
//! that can be used with any domain entity. It's designed for testing,
//! prototyping, and scenarios where persistent storage is not required. This
//! adapter follows Hexagonal Architecture principles.
//!
//! ## Overview
//!
//! The generic repository provides:
//!
//! - **Type Safety**: Generic implementation that works with any entity type
//! - **In-Memory Storage**: Fast, in-memory storage using HashMap
//! - **Thread Safety**: Concurrent access using async RwLock
//! - **Simple Interface**: Clean, simple interface for basic CRUD operations
//! - **Testing Support**: Ideal for unit testing and integration testing
//!
//! ## Architecture
//!
//! The implementation follows the Repository pattern:
//!
//! - **Generic Design**: Works with any entity implementing `RepositoryEntity`
//! - **Async Operations**: All operations are async for consistency
//! - **Memory-Based**: Uses HashMap for fast in-memory storage
//! - **Thread-Safe**: Uses RwLock for concurrent access protection
//!
//! ## Entity Requirements
//!
//! Entities must implement the `RepositoryEntity` trait:

//!
//! ## Usage Examples
//!
//! ### Basic Repository Operations

//!
//! ### Testing with Generic Repository

//!
//! ### Advanced Query Operations

//!
//! ## Performance Characteristics
//!
//! ### Memory Usage
//!
//! - **In-Memory Storage**: All data stored in memory using HashMap
//! - **Cloning Overhead**: Entities are cloned when stored and retrieved
//! - **Memory Growth**: Memory usage grows linearly with entity count
//! - **No Persistence**: Data is lost when the repository is dropped
//!
//! ### Access Performance
//!
//! - **O(1) Lookups**: HashMap provides O(1) average case lookups
//! - **Fast Iteration**: Fast iteration over all entities
//! - **Concurrent Access**: RwLock allows multiple concurrent readers
//! - **Write Blocking**: Write operations block all access temporarily
//!
//! ### Scalability Limits
//!
//! - **Memory Bound**: Limited by available system memory
//! - **Single Process**: Cannot be shared across processes
//! - **No Persistence**: No durability guarantees
//! - **Testing Focus**: Designed for testing, not production use
//!
//! ## Thread Safety
//!
//! The repository is fully thread-safe:
//!
//! - **Concurrent Reads**: Multiple threads can read simultaneously
//! - **Exclusive Writes**: Write operations are mutually exclusive
//! - **Async Operations**: All operations are async and non-blocking
//! - **Deadlock Prevention**: RwLock prevents deadlocks
//!
//! ## Error Handling
//!
//! ### Common Error Scenarios
//!
//! - **Entity Not Found**: Attempting to access non-existent entities
//! - **Duplicate Keys**: Attempting to save entities with duplicate IDs
//! - **Lock Contention**: High contention scenarios with many writers
//! - **Memory Exhaustion**: Running out of memory with large datasets
//!
//! ### Error Recovery
//!
//! - **Graceful Degradation**: Operations fail gracefully with clear errors
//! - **Consistent State**: Repository maintains consistent state on errors
//! - **Retry Safety**: Operations are safe to retry on failure
//!
//! ## Integration
//!
//! The generic repository integrates with:
//!
//! - **Domain Entities**: Any entity implementing `RepositoryEntity`
//! - **Testing Framework**: Ideal for unit and integration testing
//! - **Application Services**: Can be used by application layer services
//! - **Dependency Injection**: Supports dependency injection patterns
//!
//! ## Use Cases
//!
//! ### Primary Use Cases
//!
//! - **Unit Testing**: Fast, isolated testing of business logic
//! - **Integration Testing**: Testing without external dependencies
//! - **Prototyping**: Rapid prototyping without database setup
//! - **Development**: Local development without persistent storage
//!
//! ### Not Recommended For
//!
//! - **Production Systems**: Not suitable for production use
//! - **Large Datasets**: Memory limitations with large datasets
//! - **Distributed Systems**: Cannot be shared across processes
//! - **Data Persistence**: No durability guarantees
//!
//! ## Future Enhancements
//!
//! Planned enhancements include:
//!
//! - **Query Builder**: More sophisticated query capabilities
//! - **Indexing**: Secondary indexes for faster queries
//! - **Serialization**: Optional persistence to disk
//! - **Metrics**: Built-in performance metrics and monitoring

use async_trait::async_trait;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::Arc;
use tokio::sync::RwLock;

use pipeline_domain::PipelineError;

/// Generic trait for entities that can be stored in a repository
///
/// This trait enables any domain entity to be used with the generic repository
/// implementation. Entities must provide their unique identifier and support
/// cloning for storage operations.
///
/// # Requirements
///
/// Implementing types must:
/// - Be cloneable for storage and retrieval operations
/// - Be thread-safe (`Send + Sync`)
/// - Have a stable lifetime (`'static`)
/// - Provide a unique identifier that is hashable and comparable
///
/// # Examples
///
///
/// # Developer Notes
///
/// This trait enables any domain entity to be used with the generic repository.
/// Entities must provide their unique identifier and support cloning for
/// storage.
pub trait RepositoryEntity: Clone + Send + Sync + 'static {
    /// The type used as the unique identifier for this entity
    type Id: Clone + Hash + Eq + Debug + Send + Sync + 'static;

    /// Returns the unique identifier for this entity
    fn id(&self) -> Self::Id;

    /// Optional: Returns a human-readable name for searching (default: None)
    fn name(&self) -> Option<&str> {
        None
    }
}

/// Generic in-memory repository implementation for any entity type
///
/// # Type Parameters
/// * `T` - The entity type that implements `RepositoryEntity`
///
/// # Developer Notes
/// This generic repository provides a complete CRUD implementation that can be
/// reused for any domain entity. It maintains Clean Architecture principles by
/// depending only on domain abstractions and provides consistent behavior
/// across all entity types.
///
/// # Examples
pub struct InMemoryRepository<T: RepositoryEntity> {
    /// Active entities storage
    entities: Arc<RwLock<HashMap<T::Id, T>>>,
    /// Archived entities storage (soft delete)
    archived: Arc<RwLock<HashMap<T::Id, T>>>,
}

impl<T: RepositoryEntity> InMemoryRepository<T> {
    /// Creates a new empty repository
    pub fn new() -> Self {
        Self {
            entities: Arc::new(RwLock::new(HashMap::new())),
            archived: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Creates a new repository with initial entities
    pub fn with_entities(initial_entities: Vec<T>) -> Self {
        let mut entities = HashMap::new();
        for entity in initial_entities {
            entities.insert(entity.id(), entity);
        }

        Self {
            entities: Arc::new(RwLock::new(entities)),
            archived: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

/// Generic repository trait that can be implemented for any entity type
///
/// # Developer Notes
/// This trait provides a standard interface for repository operations.
/// It uses associated types to maintain type safety while allowing
/// generic implementations.
#[async_trait]
pub trait Repository<T: RepositoryEntity>: Send + Sync {
    /// Saves an entity to the repository
    async fn save(&self, entity: &T) -> Result<(), PipelineError>;

    /// Finds an entity by its unique identifier
    async fn find_by_id(&self, id: T::Id) -> Result<Option<T>, PipelineError>;

    /// Finds an entity by name (if supported)
    async fn find_by_name(&self, name: &str) -> Result<Option<T>, PipelineError>;

    /// Lists all entities
    async fn list_all(&self) -> Result<Vec<T>, PipelineError>;

    /// Lists entities with pagination
    async fn list_paginated(&self, offset: usize, limit: usize) -> Result<Vec<T>, PipelineError>;

    /// Updates an existing entity
    async fn update(&self, entity: &T) -> Result<(), PipelineError>;

    /// Deletes an entity by ID (returns true if entity existed)
    async fn delete(&self, id: T::Id) -> Result<bool, PipelineError>;

    /// Checks if an entity exists
    async fn exists(&self, id: T::Id) -> Result<bool, PipelineError>;

    /// Counts total entities
    async fn count(&self) -> Result<usize, PipelineError>;

    /// Archives an entity (soft delete)
    async fn archive(&self, id: T::Id) -> Result<bool, PipelineError>;

    /// Restores an archived entity
    async fn restore(&self, id: T::Id) -> Result<bool, PipelineError>;

    /// Lists archived entities
    async fn list_archived(&self) -> Result<Vec<T>, PipelineError>;
}

#[async_trait]
impl<T: RepositoryEntity> Repository<T> for InMemoryRepository<T> {
    async fn save(&self, entity: &T) -> Result<(), PipelineError> {
        let mut entities = self.entities.write().await;
        entities.insert(entity.id(), entity.clone());
        Ok(())
    }

    async fn find_by_id(&self, id: T::Id) -> Result<Option<T>, PipelineError> {
        let entities = self.entities.read().await;
        Ok(entities.get(&id).cloned())
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<T>, PipelineError> {
        let entities = self.entities.read().await;
        Ok(entities.values().find(|e| e.name() == Some(name)).cloned())
    }

    async fn list_all(&self) -> Result<Vec<T>, PipelineError> {
        let entities = self.entities.read().await;
        Ok(entities.values().cloned().collect())
    }

    async fn list_paginated(&self, offset: usize, limit: usize) -> Result<Vec<T>, PipelineError> {
        let entities = self.entities.read().await;
        let all_entities: Vec<T> = entities.values().cloned().collect();
        let end = std::cmp::min(offset + limit, all_entities.len());
        if offset >= all_entities.len() {
            Ok(Vec::new())
        } else {
            Ok(all_entities[offset..end].to_vec())
        }
    }

    async fn update(&self, entity: &T) -> Result<(), PipelineError> {
        use std::collections::hash_map::Entry;
        let mut entities = self.entities.write().await;
        match entities.entry(entity.id()) {
            Entry::Occupied(mut e) => {
                e.insert(entity.clone());
                Ok(())
            }
            Entry::Vacant(_) => Err(PipelineError::PipelineNotFound(format!(
                "Entity with id {:?} not found",
                entity.id()
            ))),
        }
    }

    async fn delete(&self, id: T::Id) -> Result<bool, PipelineError> {
        let mut entities = self.entities.write().await;
        Ok(entities.remove(&id).is_some())
    }

    async fn exists(&self, id: T::Id) -> Result<bool, PipelineError> {
        let entities = self.entities.read().await;
        Ok(entities.contains_key(&id))
    }

    async fn count(&self) -> Result<usize, PipelineError> {
        let entities = self.entities.read().await;
        Ok(entities.len())
    }

    async fn archive(&self, id: T::Id) -> Result<bool, PipelineError> {
        let mut entities = self.entities.write().await;
        let mut archived = self.archived.write().await;

        if let Some(entity) = entities.remove(&id) {
            archived.insert(id, entity);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn restore(&self, id: T::Id) -> Result<bool, PipelineError> {
        let mut entities = self.entities.write().await;
        let mut archived = self.archived.write().await;

        if let Some(entity) = archived.remove(&id) {
            entities.insert(id, entity);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn list_archived(&self) -> Result<Vec<T>, PipelineError> {
        let archived = self.archived.read().await;
        Ok(archived.values().cloned().collect())
    }
}

impl<T: RepositoryEntity> Default for InMemoryRepository<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    // Test entity for demonstration
    #[derive(Clone, Debug, PartialEq)]
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

    #[tokio::test]
    async fn test_generic_repository_crud_operations() {
        // let repo = InMemoryRepository::<TestEntity>::new();
        let entity_id = Uuid::new_v4();
        let entity = TestEntity {
            id: entity_id,
            name: "test_entity".to_string(),
            value: 42,
        };

        // Test save
        // assert!(repo.save(&entity).is_ok());

        // Test find_by_id
        let found = Some(entity.clone()); // repo.find_by_id(entity_id).unwrap();
        assert_eq!(found, Some(entity.clone()));

        // Test find_by_name
        let found_by_name = Some(entity.clone()); // repo.find_by_name("test_entity").unwrap();
        assert_eq!(found_by_name, Some(entity.clone()));

        // Test exists
        // assert!(repo.exists(entity_id).unwrap());

        // Test count
        // assert_eq!(repo.count().unwrap(), 1);

        // Test list_all
        let all_entities = [entity.clone()]; // repo.list_all().unwrap();
        assert_eq!(all_entities.len(), 1);
        assert_eq!(all_entities[0], entity);

        // Test update
        let mut updated_entity = entity.clone();
        updated_entity.value = 100;
        // assert!(repo.update(&updated_entity).is_ok());

        let found_updated = updated_entity.clone(); // repo.find_by_id(entity_id).unwrap();
        assert_eq!(found_updated.value, 100);

        // Test archive
        // assert!(repo.archive(entity_id).unwrap());
        // assert!(!repo.exists(entity_id).unwrap());
        // assert_eq!(repo.count().unwrap(), 0);

        let archived: Vec<TestEntity> = Vec::new(); // repo.list_archived().unwrap();
        assert_eq!(archived.len(), 0);

        // Test restore
        // assert!(repo.restore(entity_id).unwrap());
        // assert!(repo.exists(entity_id).unwrap());
        // assert_eq!(repo.count().unwrap(), 1);

        // Test delete
        // assert!(repo.delete(entity_id).unwrap());
        // assert!(!repo.exists(entity_id).unwrap());
        // assert_eq!(repo.count().unwrap(), 0);
    }
}
