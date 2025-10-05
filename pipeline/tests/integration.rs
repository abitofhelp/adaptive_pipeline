//! Integration Tests
//!
//! This module aggregates all integration tests for the pipeline application.

#[path = "integration/application_integration_test.rs"]
mod application_integration_test;

#[path = "integration/application_layer_integration_test.rs"]
mod application_layer_integration_test;

#[path = "integration/application_services_integration_test.rs"]
mod application_services_integration_test;

#[path = "integration/domain_services_test.rs"]
mod domain_services_test;

#[path = "integration/minimal_application_test.rs"]
mod minimal_application_test;

#[path = "integration/pipeline_name_validation_tests.rs"]
mod pipeline_name_validation_tests;

#[path = "integration/schema_integration_test.rs"]
mod schema_integration_test;
