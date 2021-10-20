#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <p>Welcome to the Amazon Route 53 Application Recovery Controller API Reference Guide for Recovery Control Data Plane .</p>
//! <p>Recovery control in Route 53 Application Recovery Controller includes extremely reliable routing controls that enable you to recover applications
//! by rerouting traffic, for example, across Availability Zones or AWS Regions. Routing controls are simple on/off switches
//! hosted on a cluster. A cluster is a set of five redundant regional endpoints against which you can execute API calls to update or
//! get the state of routing controls. You use routing controls to failover traffic to recover your application
//! across Availability Zones or Regions.</p>
//! <p>This API guide includes information about how to get and update routing control states in Route 53 Application Recovery Controller.</p>
//! <p>For more information about Route 53 Application Recovery Controller, see the following:</p>
//! <ul>
//! <li>
//! <p>You can create clusters, routing controls, and control panels by using the control plane API for Recovery
//! Control. For more information, see <a href="https://docs.aws.amazon.com/recovery-cluster/latest/api/">Amazon Route 53 Application Recovery Controller Recovery Control API Reference</a>.</p>
//! </li>
//! <li>
//! <p>Route 53 Application Recovery Controller also provides continuous readiness checks to ensure that your applications are scaled to handle failover traffic.
//! For more information about the related API actions, see <a href="https://docs.aws.amazon.com/recovery-readiness/latest/api/">Amazon Route 53 Application Recovery Controller Recovery Readiness API Reference</a>.</p>
//! </li>
//! <li>
//! <p>For more information about creating resilient applications and preparing for recovery readiness with Route 53 Application Recovery Controller,
//! see the <a href="r53recovery/latest/dg/">Amazon Route 53 Application Recovery Controller Developer Guide</a>.</p>
//! </li>
//! </ul>

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
    aws_http::user_agent::ApiMetadata::new("route53recoverycluster", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_smithy_types::retry::RetryConfig;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[cfg(feature = "client")]
pub use client::Client;
