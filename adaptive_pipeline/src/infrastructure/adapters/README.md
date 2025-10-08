<!--
Adaptive Pipeline
Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
SPDX-License-Identifier: BSD-3-Clause
See LICENSE file in the project root.
-->

# Infrastructure Adapters

This module provides adapter implementations for integrating with external systems
and services. Adapters follow the Adapter pattern to bridge between the internal
domain model and external system interfaces.

## Overview

Infrastructure adapters provide:

- **External Integration**: Connect to databases, APIs, and third-party services
- **Protocol Translation**: Translate between internal and external data formats
- **Error Handling**: Handle external system errors and translate to domain errors
- **Configuration Management**: Manage connection settings and authentication
- **Resilience**: Implement retry logic, circuit breakers, and fallback mechanisms

## Architecture

The adapter layer follows these design principles:

- **Separation of Concerns**: Isolate external system details from domain logic
- **Interface Segregation**: Provide focused interfaces for specific integrations
- **Dependency Inversion**: Depend on abstractions, not concrete implementations
- **Single Responsibility**: Each adapter handles one external system integration

## Adapter Types

### Database Adapters

Adapters for various database systems:

### API Adapters

Adapters for external REST and GraphQL APIs:

```rust
// Example API adapter interface
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait APIAdapter {
    type Request: Serialize;
    type Response: for<'de> Deserialize<'de>;
    
    fn send_request(&self, request: Self::Request) -> Result<Self::Response, String>;
    fn authenticate(&self) -> Result<(), String>;
    fn refresh_token(&self) -> Result<(), String>;
}

// REST API adapter
pub struct RestAPIAdapter {
    base_url: String,
    client: reqwest::Client,
    auth_token: String,
}

// GraphQL API adapter
pub struct GraphQLAdapter {
    endpoint: String,
    client: graphql_client::Client,
    schema: String,
}
```

### Message Queue Adapters

Adapters for message queuing systems:

```rust
// Example message queue adapter
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait MessageQueueAdapter {
    type Message: Serialize + for<'de> Deserialize<'de>;
    
    fn publish(&self, topic: &str, message: Self::Message) -> Result<(), String>;
    fn subscribe(&self, topic: &str) -> Result<String<Self::Message>, String>;
    fn create_topic(&self, topic: &str) -> Result<(), String>;
}

// Apache Kafka adapter
pub struct KafkaAdapter {
    brokers: String,
    producer: Option<rdkafka::producer::FutureProducer>,
    consumer: Option<rdkafka::consumer::StreamConsumer>,
}

// Redis Streams adapter
pub struct RedisStreamsAdapter {
    connection_string: String,
    client: Option<redis::Client>,
}
```

### Cloud Service Adapters

Adapters for cloud platform services:

```rust
// AWS S3 adapter for file storage
pub struct S3Adapter {
    bucket: String,
    region: String,
    client: Option<aws_sdk_s3::Client>,
}

// Azure Blob Storage adapter
pub struct AzureBlobAdapter {
    account: String,
    container: String,
    client: Option<azure_storage_blobs::BlobServiceClient>,
}

// Google Cloud Storage adapter
pub struct GCSAdapter {
    bucket: String,
    project_id: String,
    client: Option<google_cloud_storage::Client>,
}
```

## Implementation Patterns

### Connection Management

Adapters should implement proper connection lifecycle management:

### Error Handling

Translate external errors to domain errors:

```rust
impl From<sqlx::Error> for String {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::Database(db_err) => {
                String::String(db_err.to_string())
            }
            sqlx::Error::Io(io_err) => {
                String::IOError(io_err.to_string())
            }
            _ => String::UnknownError(error.to_string()),
        }
    }
}
```

### Configuration

Adapters should support flexible configuration:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterConfig {
    pub connection_string: String,
    pub timeout_seconds: u64,
    pub retry_attempts: u32,
    pub pool_size: u32,
    pub enable_ssl: bool,
}

impl Default for AdapterConfig {
    fn default() -> Self {
        Self {
            connection_string: "localhost:5432".to_string(),
            timeout_seconds: 30,
            retry_attempts: 3,
            pool_size: 10,
            enable_ssl: true,
        }
    }
}
```

## Resilience Patterns

### Circuit Breaker

Implement circuit breaker pattern for external service calls:

```rust
pub struct CircuitBreaker {
    failure_threshold: u32,
    timeout: Duration,
    state: CircuitState,
}

enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}
```

### Retry Logic

Implement exponential backoff for transient failures:

```rust
fn retry_with_backoff<F, String>(
    operation: F,
    max_attempts: u32,
) -> String
where
    F: Fn() -> String,
{
    let mut attempts = 0;
    let mut delay = Duration::from_millis(100);
    
    loop {
        match operation() {
            Ok(result) => return Ok(result),
            Err(error) if attempts < max_attempts => {
                attempts += 1;
                std::thread::sleep(delay);
                delay *= 2; // Exponential backoff
            }
            Err(error) => return Err(error),
        }
    }
}
```

## Testing

Adapters should provide mock implementations for testing:

```rust
#[cfg(test)]
pub struct MockDatabaseAdapter {
    pub pipelines: std::collections::HashMap<String, String>,
    pub should_fail: bool,
}

#[cfg(test)]
#[async_trait]
impl DatabaseAdapter for MockDatabaseAdapter {
    fn save_pipeline(&self, pipeline: &String) -> Result<(), String> {
        if self.should_fail {
            Err(String::String("Mock failure".to_string()))
        } else {
            // Mock implementation
        }
    }
}
```

## Security Considerations

- **Authentication**: Secure credential management and token handling
- **Authorization**: Proper access control and permission validation
- **Encryption**: Encrypt data in transit and at rest
- **Input Validation**: Validate all external inputs and responses
- **Audit Logging**: Log all external system interactions

## Performance Optimization

- **Connection Pooling**: Reuse connections to reduce overhead
- **Caching**: Cache frequently accessed data
- **Batch Operations**: Batch multiple operations when possible
- **Async Processing**: Use async operations for I/O-bound tasks
- **Resource Limits**: Implement proper resource limits and quotas

## Monitoring and Observability

- **Metrics**: Collect performance and usage metrics
- **Health Checks**: Implement health check endpoints
- **Distributed Tracing**: Support for distributed tracing
- **Logging**: Comprehensive logging of operations and errors
- **Alerting**: Alert on failures and performance degradation

## Future Implementations

Future implementations will be added here as needed. Examples:

- Database adapters (PostgreSQL, MongoDB, etc.)
- Message queue adapters (Kafka, RabbitMQ, etc.)
- Cloud service adapters (AWS, Azure, GCP)
- API adapters (REST, GraphQL, gRPC)
- File system adapters (S3, Azure Blob, etc.)

## Current Adapter Implementations

### Existing Adapters

The following adapter implementations currently exist in the codebase:

1. **Repository Adapters** (`repositories/sqlite_repository_adapter.rs`)
   - `SqliteRepositoryAdapter<T>` - Bridges domain Repository trait with SQLite implementation
   - Follows Hexagonal Architecture (Ports and Adapters) pattern
   - Enables seamless switching between in-memory and SQLite storage

2. **Service Adapters** (`services/chunk_processor_adapters.rs`)
   - `ServiceChunkAdapter<T>` - Generic adapter for wrapping services as chunk processors
   - `CompressionChunkAdapter` - Specialized adapter for compression services
   - `EncryptionChunkAdapter` - Specialized adapter for encryption services
   - `ServiceAdapterFactory` - Factory for creating different adapter types

### Recommended Directory Structure

```
infrastructure/
├── adapters/           # ← This directory
│   ├── README.md      # ← This file
│   ├── mod.rs         # ← Module definitions
│   ├── repository/    # ← Repository adapters
│   │   ├── mod.rs
│   │   └── sqlite_repository_adapter.rs
│   ├── service/       # ← Service adapters  
│   │   ├── mod.rs
│   │   └── chunk_processor_adapters.rs
│   ├── database/      # ← Future database adapters
│   ├── messaging/     # ← Future message queue adapters
│   ├── cloud/         # ← Future cloud service adapters
│   └── api/           # ← Future API adapters
├── services/
└── repositories/
```

## Migration Plan

To properly organize the adapter code:

1. **Move existing adapters** from their current locations to the appropriate subdirectories
2. **Create module files** (`mod.rs`) for each adapter category
3. **Update imports** throughout the codebase to reflect the new structure
4. **Convert `adapters.rs`** to a proper module directory with this README
