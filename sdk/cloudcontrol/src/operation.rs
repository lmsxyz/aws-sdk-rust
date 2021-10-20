// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CancelResourceRequest`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`cancel_resource_request`](crate::client::Client::cancel_resource_request).
///
/// See [`crate::client::fluent_builders::CancelResourceRequest`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CancelResourceRequest {
    _private: (),
}
impl CancelResourceRequest {
    /// Creates a new builder-style object to manufacture [`CancelResourceRequestInput`](crate::input::CancelResourceRequestInput)
    pub fn builder() -> crate::input::cancel_resource_request_input::Builder {
        crate::input::cancel_resource_request_input::Builder::default()
    }
    /// Creates a new `CancelResourceRequest` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CancelResourceRequest {
    type Output = std::result::Result<
        crate::output::CancelResourceRequestOutput,
        crate::error::CancelResourceRequestError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_cancel_resource_request_error(response)
        } else {
            crate::operation_deser::parse_cancel_resource_request_response(response)
        }
    }
}

/// Operation shape for `CreateResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_resource`](crate::client::Client::create_resource).
///
/// See [`crate::client::fluent_builders::CreateResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateResource {
    _private: (),
}
impl CreateResource {
    /// Creates a new builder-style object to manufacture [`CreateResourceInput`](crate::input::CreateResourceInput)
    pub fn builder() -> crate::input::create_resource_input::Builder {
        crate::input::create_resource_input::Builder::default()
    }
    /// Creates a new `CreateResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateResource {
    type Output =
        std::result::Result<crate::output::CreateResourceOutput, crate::error::CreateResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_resource_error(response)
        } else {
            crate::operation_deser::parse_create_resource_response(response)
        }
    }
}

/// Operation shape for `DeleteResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_resource`](crate::client::Client::delete_resource).
///
/// See [`crate::client::fluent_builders::DeleteResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteResource {
    _private: (),
}
impl DeleteResource {
    /// Creates a new builder-style object to manufacture [`DeleteResourceInput`](crate::input::DeleteResourceInput)
    pub fn builder() -> crate::input::delete_resource_input::Builder {
        crate::input::delete_resource_input::Builder::default()
    }
    /// Creates a new `DeleteResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteResource {
    type Output =
        std::result::Result<crate::output::DeleteResourceOutput, crate::error::DeleteResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_resource_error(response)
        } else {
            crate::operation_deser::parse_delete_resource_response(response)
        }
    }
}

/// Operation shape for `GetResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_resource`](crate::client::Client::get_resource).
///
/// See [`crate::client::fluent_builders::GetResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetResource {
    _private: (),
}
impl GetResource {
    /// Creates a new builder-style object to manufacture [`GetResourceInput`](crate::input::GetResourceInput)
    pub fn builder() -> crate::input::get_resource_input::Builder {
        crate::input::get_resource_input::Builder::default()
    }
    /// Creates a new `GetResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetResource {
    type Output =
        std::result::Result<crate::output::GetResourceOutput, crate::error::GetResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_resource_error(response)
        } else {
            crate::operation_deser::parse_get_resource_response(response)
        }
    }
}

/// Operation shape for `GetResourceRequestStatus`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_resource_request_status`](crate::client::Client::get_resource_request_status).
///
/// See [`crate::client::fluent_builders::GetResourceRequestStatus`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetResourceRequestStatus {
    _private: (),
}
impl GetResourceRequestStatus {
    /// Creates a new builder-style object to manufacture [`GetResourceRequestStatusInput`](crate::input::GetResourceRequestStatusInput)
    pub fn builder() -> crate::input::get_resource_request_status_input::Builder {
        crate::input::get_resource_request_status_input::Builder::default()
    }
    /// Creates a new `GetResourceRequestStatus` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetResourceRequestStatus {
    type Output = std::result::Result<
        crate::output::GetResourceRequestStatusOutput,
        crate::error::GetResourceRequestStatusError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_resource_request_status_error(response)
        } else {
            crate::operation_deser::parse_get_resource_request_status_response(response)
        }
    }
}

/// Operation shape for `ListResourceRequests`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_resource_requests`](crate::client::Client::list_resource_requests).
///
/// See [`crate::client::fluent_builders::ListResourceRequests`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListResourceRequests {
    _private: (),
}
impl ListResourceRequests {
    /// Creates a new builder-style object to manufacture [`ListResourceRequestsInput`](crate::input::ListResourceRequestsInput)
    pub fn builder() -> crate::input::list_resource_requests_input::Builder {
        crate::input::list_resource_requests_input::Builder::default()
    }
    /// Creates a new `ListResourceRequests` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListResourceRequests {
    type Output = std::result::Result<
        crate::output::ListResourceRequestsOutput,
        crate::error::ListResourceRequestsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_resource_requests_error(response)
        } else {
            crate::operation_deser::parse_list_resource_requests_response(response)
        }
    }
}

/// Operation shape for `ListResources`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_resources`](crate::client::Client::list_resources).
///
/// See [`crate::client::fluent_builders::ListResources`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListResources {
    _private: (),
}
impl ListResources {
    /// Creates a new builder-style object to manufacture [`ListResourcesInput`](crate::input::ListResourcesInput)
    pub fn builder() -> crate::input::list_resources_input::Builder {
        crate::input::list_resources_input::Builder::default()
    }
    /// Creates a new `ListResources` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListResources {
    type Output =
        std::result::Result<crate::output::ListResourcesOutput, crate::error::ListResourcesError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_resources_error(response)
        } else {
            crate::operation_deser::parse_list_resources_response(response)
        }
    }
}

/// Operation shape for `UpdateResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_resource`](crate::client::Client::update_resource).
///
/// See [`crate::client::fluent_builders::UpdateResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateResource {
    _private: (),
}
impl UpdateResource {
    /// Creates a new builder-style object to manufacture [`UpdateResourceInput`](crate::input::UpdateResourceInput)
    pub fn builder() -> crate::input::update_resource_input::Builder {
        crate::input::update_resource_input::Builder::default()
    }
    /// Creates a new `UpdateResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateResource {
    type Output =
        std::result::Result<crate::output::UpdateResourceOutput, crate::error::UpdateResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_resource_error(response)
        } else {
            crate::operation_deser::parse_update_resource_response(response)
        }
    }
}
