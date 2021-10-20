// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateToken`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_token`](crate::client::Client::create_token).
///
/// See [`crate::client::fluent_builders::CreateToken`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateToken {
    _private: (),
}
impl CreateToken {
    /// Creates a new builder-style object to manufacture [`CreateTokenInput`](crate::input::CreateTokenInput)
    pub fn builder() -> crate::input::create_token_input::Builder {
        crate::input::create_token_input::Builder::default()
    }
    /// Creates a new `CreateToken` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateToken {
    type Output =
        std::result::Result<crate::output::CreateTokenOutput, crate::error::CreateTokenError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_token_error(response)
        } else {
            crate::operation_deser::parse_create_token_response(response)
        }
    }
}

/// Operation shape for `RegisterClient`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`register_client`](crate::client::Client::register_client).
///
/// See [`crate::client::fluent_builders::RegisterClient`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct RegisterClient {
    _private: (),
}
impl RegisterClient {
    /// Creates a new builder-style object to manufacture [`RegisterClientInput`](crate::input::RegisterClientInput)
    pub fn builder() -> crate::input::register_client_input::Builder {
        crate::input::register_client_input::Builder::default()
    }
    /// Creates a new `RegisterClient` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for RegisterClient {
    type Output =
        std::result::Result<crate::output::RegisterClientOutput, crate::error::RegisterClientError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_register_client_error(response)
        } else {
            crate::operation_deser::parse_register_client_response(response)
        }
    }
}

/// Operation shape for `StartDeviceAuthorization`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`start_device_authorization`](crate::client::Client::start_device_authorization).
///
/// See [`crate::client::fluent_builders::StartDeviceAuthorization`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StartDeviceAuthorization {
    _private: (),
}
impl StartDeviceAuthorization {
    /// Creates a new builder-style object to manufacture [`StartDeviceAuthorizationInput`](crate::input::StartDeviceAuthorizationInput)
    pub fn builder() -> crate::input::start_device_authorization_input::Builder {
        crate::input::start_device_authorization_input::Builder::default()
    }
    /// Creates a new `StartDeviceAuthorization` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StartDeviceAuthorization {
    type Output = std::result::Result<
        crate::output::StartDeviceAuthorizationOutput,
        crate::error::StartDeviceAuthorizationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_start_device_authorization_error(response)
        } else {
            crate::operation_deser::parse_start_device_authorization_response(response)
        }
    }
}
