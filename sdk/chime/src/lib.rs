#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <p>The Amazon Chime API (application programming interface) is designed for developers to
//! perform key tasks, such as creating and managing Amazon Chime accounts, users, and Voice
//! Connectors. This guide provides detailed information about the Amazon Chime API,
//! including operations, types, inputs and outputs, and error codes. It also includes some
//! server-side API actions to use with the Amazon Chime SDK. For more information about the
//! Amazon Chime SDK, see <a href="https://docs.aws.amazon.com/chime/latest/dg/meetings-sdk.html">
//! Using the Amazon Chime SDK
//! </a> in the <i>Amazon Chime Developer Guide</i>.</p>
//! <p>You can use an AWS SDK, the AWS Command Line Interface (AWS CLI), or the REST API to make API calls. We recommend using an AWS SDK or the
//! AWS CLI. Each API operation includes links to information about using it with a language-specific AWS SDK or the AWS CLI.</p>
//! <dl>
//! <dt>Using an AWS SDK</dt>
//! <dd>
//! <p>
//! You don't need to write code to calculate a signature for request authentication. The SDK clients authenticate your requests by using access keys that you provide. For more information about AWS SDKs, see the
//! <a href="http://aws.amazon.com/developer/">AWS Developer Center</a>.
//! </p>
//! </dd>
//! <dt>Using the AWS CLI</dt>
//! <dd>
//! <p>Use your access keys with the AWS CLI to make API calls. For information about setting up the AWS CLI, see
//! <a href="https://docs.aws.amazon.com/cli/latest/userguide/installing.html">Installing the AWS Command Line Interface</a>
//! in the <i>AWS Command Line Interface User Guide</i>. For a list of available Amazon Chime commands, see the
//! <a href="https://docs.aws.amazon.com/cli/latest/reference/chime/index.html">Amazon Chime commands</a> in the
//! <i>AWS CLI Command Reference</i>.
//! </p>
//! </dd>
//! <dt>Using REST APIs</dt>
//! <dd>
//! <p>If you use REST to make API calls, you must authenticate your request by providing a signature. Amazon Chime supports signature version 4. For more information, see
//! <a href="https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html">Signature Version 4 Signing Process</a>
//! in the <i>Amazon Web Services General Reference</i>.</p>
//!
//! <p>When making REST API calls, use the service name <code>chime</code> and REST endpoint <code>https://service.chime.aws.amazon.com</code>.</p>
//! </dd>
//! </dl>
//!
//! <p>Administrative permissions are controlled using AWS Identity and Access Management (IAM). For more information, see
//! <a href="https://docs.aws.amazon.com/chime/latest/ag/security-iam.html">Identity and Access Management for Amazon Chime</a>
//! in the <i>Amazon Chime Administration Guide</i>.</p>

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
    aws_http::user_agent::ApiMetadata::new("chime", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_smithy_types::retry::RetryConfig;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[cfg(feature = "client")]
pub use client::Client;
