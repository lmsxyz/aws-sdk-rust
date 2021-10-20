// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateEndpoint`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_endpoint`](crate::client::Client::create_endpoint).
///
/// See [`crate::client::fluent_builders::CreateEndpoint`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateEndpoint {
    _private: (),
}
impl CreateEndpoint {
    /// Creates a new builder-style object to manufacture [`CreateEndpointInput`](crate::input::CreateEndpointInput)
    pub fn builder() -> crate::input::create_endpoint_input::Builder {
        crate::input::create_endpoint_input::Builder::default()
    }
    /// Creates a new `CreateEndpoint` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateEndpoint {
    type Output =
        std::result::Result<crate::output::CreateEndpointOutput, crate::error::CreateEndpointError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_endpoint_error(response)
        } else {
            crate::operation_deser::parse_create_endpoint_response(response)
        }
    }
}

/// Operation shape for `DeleteEndpoint`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_endpoint`](crate::client::Client::delete_endpoint).
///
/// See [`crate::client::fluent_builders::DeleteEndpoint`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteEndpoint {
    _private: (),
}
impl DeleteEndpoint {
    /// Creates a new builder-style object to manufacture [`DeleteEndpointInput`](crate::input::DeleteEndpointInput)
    pub fn builder() -> crate::input::delete_endpoint_input::Builder {
        crate::input::delete_endpoint_input::Builder::default()
    }
    /// Creates a new `DeleteEndpoint` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteEndpoint {
    type Output =
        std::result::Result<crate::output::DeleteEndpointOutput, crate::error::DeleteEndpointError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_endpoint_error(response)
        } else {
            crate::operation_deser::parse_delete_endpoint_response(response)
        }
    }
}

/// Operation shape for `ListEndpoints`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_endpoints`](crate::client::Client::list_endpoints).
///
/// See [`crate::client::fluent_builders::ListEndpoints`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListEndpoints {
    _private: (),
}
impl ListEndpoints {
    /// Creates a new builder-style object to manufacture [`ListEndpointsInput`](crate::input::ListEndpointsInput)
    pub fn builder() -> crate::input::list_endpoints_input::Builder {
        crate::input::list_endpoints_input::Builder::default()
    }
    /// Creates a new `ListEndpoints` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListEndpoints {
    type Output =
        std::result::Result<crate::output::ListEndpointsOutput, crate::error::ListEndpointsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_endpoints_error(response)
        } else {
            crate::operation_deser::parse_list_endpoints_response(response)
        }
    }
}
