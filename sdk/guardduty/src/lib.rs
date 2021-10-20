#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <p>Amazon GuardDuty is a continuous security monitoring service that analyzes and processes
//! the following data sources: VPC Flow Logs, AWS CloudTrail event logs, and DNS logs. It uses
//! threat intelligence feeds (such as lists of malicious IPs and domains) and machine learning to
//! identify unexpected, potentially unauthorized, and malicious activity within your AWS
//! environment. This can include issues like escalations of privileges, uses of exposed
//! credentials, or communication with malicious IPs, URLs, or domains. For example, GuardDuty can
//! detect compromised EC2 instances that serve malware or mine bitcoin. </p>
//! <p>GuardDuty also monitors AWS account access behavior for signs of compromise. Some examples
//! of this are unauthorized infrastructure deployments such as EC2 instances deployed in a Region
//! that has never been used, or unusual API calls like a password policy change to reduce
//! password strength. </p>
//! <p>GuardDuty informs you of the status of your AWS environment by producing security findings
//! that you can view in the GuardDuty console or through Amazon CloudWatch events. For more
//! information, see the <i>
//! <a href="https://docs.aws.amazon.com/guardduty/latest/ug/what-is-guardduty.html">Amazon
//! GuardDuty User Guide</a>
//! </i>. </p>

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
mod idempotency_token;
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
    aws_http::user_agent::ApiMetadata::new("guardduty", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_smithy_types::retry::RetryConfig;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[cfg(feature = "client")]
pub use client::Client;
