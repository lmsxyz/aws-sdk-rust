#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <fullname>Amazon Elasticsearch Configuration Service</fullname>
//! <p>Use the Amazon Elasticsearch Configuration API to create, configure, and manage Elasticsearch domains.</p>
//! <p>For sample code that uses the Configuration API, see the <a href="https://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-configuration-samples.html">Amazon Elasticsearch Service Developer Guide</a>.
//! The guide also contains <a href="https://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-request-signing.html">sample code for sending signed HTTP requests to the Elasticsearch APIs</a>.</p>
//! <p>The endpoint for configuration service requests is region-specific: es.<i>region</i>.amazonaws.com.
//! For example, es.us-east-1.amazonaws.com. For a current list of supported regions and endpoints,
//! see <a href="http://docs.aws.amazon.com/general/latest/gr/rande.html#elasticsearch-service-regions" target="_blank">Regions and Endpoints</a>.</p>

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

pub use config::Config;

mod aws_endpoint;
/// Client and fluent builders for calling the service.
#[cfg(feature = "client")]
pub mod client;
/// Configuration for the service.
pub mod config;
/// Errors that can occur when calling the service.
pub mod error;
mod error_meta;
/// Input structures for operations.
pub mod input;
mod json_deser;
mod json_errors;
mod json_ser;
/// Data structures used by operation inputs/outputs.
pub mod model;
mod no_credentials;
/// All operations that this crate can perform.
pub mod operation;
mod operation_deser;
mod operation_ser;
/// Output structures for operations.
pub mod output;
/// Crate version number.
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use aws_smithy_http::byte_stream::ByteStream;
pub use aws_smithy_http::result::SdkError;
pub use aws_smithy_types::Blob;
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("elasticsearchservice", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_smithy_types::retry::RetryConfig;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[cfg(feature = "client")]
pub use client::Client;
