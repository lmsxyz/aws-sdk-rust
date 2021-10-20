#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <fullname>AWS Auto Scaling</fullname>
//!
//!
//! <p>Use AWS Auto Scaling to create scaling plans for your applications to
//! automatically scale your scalable AWS resources. </p>
//! <p>
//! <b>API Summary</b>
//! </p>
//! <p>You can use the AWS Auto Scaling service API to accomplish the following tasks:</p>
//! <ul>
//! <li>
//! <p>Create and manage scaling plans</p>
//! </li>
//! <li>
//! <p>Define target tracking scaling policies to dynamically scale your resources based
//! on utilization</p>
//! </li>
//! <li>
//! <p>Scale Amazon EC2 Auto Scaling groups using predictive scaling and dynamic scaling to scale your
//! Amazon EC2 capacity faster</p>
//! </li>
//! <li>
//! <p>Set minimum and maximum capacity limits</p>
//! </li>
//! <li>
//! <p>Retrieve information on existing scaling plans</p>
//! </li>
//! <li>
//! <p>Access current forecast data and historical forecast data for up to 56 days
//! previous</p>
//! </li>
//! </ul>
//!
//! <p>To learn more about AWS Auto Scaling, including information about granting IAM users required
//! permissions for AWS Auto Scaling actions, see the <a href="https://docs.aws.amazon.com/autoscaling/plans/userguide/what-is-aws-auto-scaling.html">AWS Auto Scaling User Guide</a>. </p>

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
    aws_http::user_agent::ApiMetadata::new("autoscalingplans", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_smithy_types::retry::RetryConfig;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[cfg(feature = "client")]
pub use client::Client;
