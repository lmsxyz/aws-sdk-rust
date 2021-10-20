// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateHttpNamespace`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_http_namespace`](crate::client::Client::create_http_namespace).
///
/// See [`crate::client::fluent_builders::CreateHttpNamespace`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateHttpNamespace {
    _private: (),
}
impl CreateHttpNamespace {
    /// Creates a new builder-style object to manufacture [`CreateHttpNamespaceInput`](crate::input::CreateHttpNamespaceInput)
    pub fn builder() -> crate::input::create_http_namespace_input::Builder {
        crate::input::create_http_namespace_input::Builder::default()
    }
    /// Creates a new `CreateHttpNamespace` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateHttpNamespace {
    type Output = std::result::Result<
        crate::output::CreateHttpNamespaceOutput,
        crate::error::CreateHttpNamespaceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_http_namespace_error(response)
        } else {
            crate::operation_deser::parse_create_http_namespace_response(response)
        }
    }
}

/// Operation shape for `CreatePrivateDnsNamespace`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_private_dns_namespace`](crate::client::Client::create_private_dns_namespace).
///
/// See [`crate::client::fluent_builders::CreatePrivateDnsNamespace`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreatePrivateDnsNamespace {
    _private: (),
}
impl CreatePrivateDnsNamespace {
    /// Creates a new builder-style object to manufacture [`CreatePrivateDnsNamespaceInput`](crate::input::CreatePrivateDnsNamespaceInput)
    pub fn builder() -> crate::input::create_private_dns_namespace_input::Builder {
        crate::input::create_private_dns_namespace_input::Builder::default()
    }
    /// Creates a new `CreatePrivateDnsNamespace` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreatePrivateDnsNamespace {
    type Output = std::result::Result<
        crate::output::CreatePrivateDnsNamespaceOutput,
        crate::error::CreatePrivateDnsNamespaceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_private_dns_namespace_error(response)
        } else {
            crate::operation_deser::parse_create_private_dns_namespace_response(response)
        }
    }
}

/// Operation shape for `CreatePublicDnsNamespace`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_public_dns_namespace`](crate::client::Client::create_public_dns_namespace).
///
/// See [`crate::client::fluent_builders::CreatePublicDnsNamespace`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreatePublicDnsNamespace {
    _private: (),
}
impl CreatePublicDnsNamespace {
    /// Creates a new builder-style object to manufacture [`CreatePublicDnsNamespaceInput`](crate::input::CreatePublicDnsNamespaceInput)
    pub fn builder() -> crate::input::create_public_dns_namespace_input::Builder {
        crate::input::create_public_dns_namespace_input::Builder::default()
    }
    /// Creates a new `CreatePublicDnsNamespace` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreatePublicDnsNamespace {
    type Output = std::result::Result<
        crate::output::CreatePublicDnsNamespaceOutput,
        crate::error::CreatePublicDnsNamespaceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_public_dns_namespace_error(response)
        } else {
            crate::operation_deser::parse_create_public_dns_namespace_response(response)
        }
    }
}

/// Operation shape for `CreateService`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_service`](crate::client::Client::create_service).
///
/// See [`crate::client::fluent_builders::CreateService`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateService {
    _private: (),
}
impl CreateService {
    /// Creates a new builder-style object to manufacture [`CreateServiceInput`](crate::input::CreateServiceInput)
    pub fn builder() -> crate::input::create_service_input::Builder {
        crate::input::create_service_input::Builder::default()
    }
    /// Creates a new `CreateService` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateService {
    type Output =
        std::result::Result<crate::output::CreateServiceOutput, crate::error::CreateServiceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_service_error(response)
        } else {
            crate::operation_deser::parse_create_service_response(response)
        }
    }
}

/// Operation shape for `DeleteNamespace`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_namespace`](crate::client::Client::delete_namespace).
///
/// See [`crate::client::fluent_builders::DeleteNamespace`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteNamespace {
    _private: (),
}
impl DeleteNamespace {
    /// Creates a new builder-style object to manufacture [`DeleteNamespaceInput`](crate::input::DeleteNamespaceInput)
    pub fn builder() -> crate::input::delete_namespace_input::Builder {
        crate::input::delete_namespace_input::Builder::default()
    }
    /// Creates a new `DeleteNamespace` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteNamespace {
    type Output = std::result::Result<
        crate::output::DeleteNamespaceOutput,
        crate::error::DeleteNamespaceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_namespace_error(response)
        } else {
            crate::operation_deser::parse_delete_namespace_response(response)
        }
    }
}

/// Operation shape for `DeleteService`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_service`](crate::client::Client::delete_service).
///
/// See [`crate::client::fluent_builders::DeleteService`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteService {
    _private: (),
}
impl DeleteService {
    /// Creates a new builder-style object to manufacture [`DeleteServiceInput`](crate::input::DeleteServiceInput)
    pub fn builder() -> crate::input::delete_service_input::Builder {
        crate::input::delete_service_input::Builder::default()
    }
    /// Creates a new `DeleteService` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteService {
    type Output =
        std::result::Result<crate::output::DeleteServiceOutput, crate::error::DeleteServiceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_service_error(response)
        } else {
            crate::operation_deser::parse_delete_service_response(response)
        }
    }
}

/// Operation shape for `DeregisterInstance`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`deregister_instance`](crate::client::Client::deregister_instance).
///
/// See [`crate::client::fluent_builders::DeregisterInstance`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeregisterInstance {
    _private: (),
}
impl DeregisterInstance {
    /// Creates a new builder-style object to manufacture [`DeregisterInstanceInput`](crate::input::DeregisterInstanceInput)
    pub fn builder() -> crate::input::deregister_instance_input::Builder {
        crate::input::deregister_instance_input::Builder::default()
    }
    /// Creates a new `DeregisterInstance` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeregisterInstance {
    type Output = std::result::Result<
        crate::output::DeregisterInstanceOutput,
        crate::error::DeregisterInstanceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_deregister_instance_error(response)
        } else {
            crate::operation_deser::parse_deregister_instance_response(response)
        }
    }
}

/// Operation shape for `DiscoverInstances`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`discover_instances`](crate::client::Client::discover_instances).
///
/// See [`crate::client::fluent_builders::DiscoverInstances`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DiscoverInstances {
    _private: (),
}
impl DiscoverInstances {
    /// Creates a new builder-style object to manufacture [`DiscoverInstancesInput`](crate::input::DiscoverInstancesInput)
    pub fn builder() -> crate::input::discover_instances_input::Builder {
        crate::input::discover_instances_input::Builder::default()
    }
    /// Creates a new `DiscoverInstances` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DiscoverInstances {
    type Output = std::result::Result<
        crate::output::DiscoverInstancesOutput,
        crate::error::DiscoverInstancesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_discover_instances_error(response)
        } else {
            crate::operation_deser::parse_discover_instances_response(response)
        }
    }
}

/// Operation shape for `GetInstance`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_instance`](crate::client::Client::get_instance).
///
/// See [`crate::client::fluent_builders::GetInstance`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetInstance {
    _private: (),
}
impl GetInstance {
    /// Creates a new builder-style object to manufacture [`GetInstanceInput`](crate::input::GetInstanceInput)
    pub fn builder() -> crate::input::get_instance_input::Builder {
        crate::input::get_instance_input::Builder::default()
    }
    /// Creates a new `GetInstance` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetInstance {
    type Output =
        std::result::Result<crate::output::GetInstanceOutput, crate::error::GetInstanceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_instance_error(response)
        } else {
            crate::operation_deser::parse_get_instance_response(response)
        }
    }
}

/// Operation shape for `GetInstancesHealthStatus`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_instances_health_status`](crate::client::Client::get_instances_health_status).
///
/// See [`crate::client::fluent_builders::GetInstancesHealthStatus`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetInstancesHealthStatus {
    _private: (),
}
impl GetInstancesHealthStatus {
    /// Creates a new builder-style object to manufacture [`GetInstancesHealthStatusInput`](crate::input::GetInstancesHealthStatusInput)
    pub fn builder() -> crate::input::get_instances_health_status_input::Builder {
        crate::input::get_instances_health_status_input::Builder::default()
    }
    /// Creates a new `GetInstancesHealthStatus` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetInstancesHealthStatus {
    type Output = std::result::Result<
        crate::output::GetInstancesHealthStatusOutput,
        crate::error::GetInstancesHealthStatusError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_instances_health_status_error(response)
        } else {
            crate::operation_deser::parse_get_instances_health_status_response(response)
        }
    }
}

/// Operation shape for `GetNamespace`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_namespace`](crate::client::Client::get_namespace).
///
/// See [`crate::client::fluent_builders::GetNamespace`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetNamespace {
    _private: (),
}
impl GetNamespace {
    /// Creates a new builder-style object to manufacture [`GetNamespaceInput`](crate::input::GetNamespaceInput)
    pub fn builder() -> crate::input::get_namespace_input::Builder {
        crate::input::get_namespace_input::Builder::default()
    }
    /// Creates a new `GetNamespace` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetNamespace {
    type Output =
        std::result::Result<crate::output::GetNamespaceOutput, crate::error::GetNamespaceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_namespace_error(response)
        } else {
            crate::operation_deser::parse_get_namespace_response(response)
        }
    }
}

/// Operation shape for `GetOperation`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_operation`](crate::client::Client::get_operation).
///
/// See [`crate::client::fluent_builders::GetOperation`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetOperation {
    _private: (),
}
impl GetOperation {
    /// Creates a new builder-style object to manufacture [`GetOperationInput`](crate::input::GetOperationInput)
    pub fn builder() -> crate::input::get_operation_input::Builder {
        crate::input::get_operation_input::Builder::default()
    }
    /// Creates a new `GetOperation` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetOperation {
    type Output =
        std::result::Result<crate::output::GetOperationOutput, crate::error::GetOperationError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_operation_error(response)
        } else {
            crate::operation_deser::parse_get_operation_response(response)
        }
    }
}

/// Operation shape for `GetService`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_service`](crate::client::Client::get_service).
///
/// See [`crate::client::fluent_builders::GetService`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetService {
    _private: (),
}
impl GetService {
    /// Creates a new builder-style object to manufacture [`GetServiceInput`](crate::input::GetServiceInput)
    pub fn builder() -> crate::input::get_service_input::Builder {
        crate::input::get_service_input::Builder::default()
    }
    /// Creates a new `GetService` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetService {
    type Output =
        std::result::Result<crate::output::GetServiceOutput, crate::error::GetServiceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_service_error(response)
        } else {
            crate::operation_deser::parse_get_service_response(response)
        }
    }
}

/// Operation shape for `ListInstances`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_instances`](crate::client::Client::list_instances).
///
/// See [`crate::client::fluent_builders::ListInstances`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListInstances {
    _private: (),
}
impl ListInstances {
    /// Creates a new builder-style object to manufacture [`ListInstancesInput`](crate::input::ListInstancesInput)
    pub fn builder() -> crate::input::list_instances_input::Builder {
        crate::input::list_instances_input::Builder::default()
    }
    /// Creates a new `ListInstances` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListInstances {
    type Output =
        std::result::Result<crate::output::ListInstancesOutput, crate::error::ListInstancesError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_instances_error(response)
        } else {
            crate::operation_deser::parse_list_instances_response(response)
        }
    }
}

/// Operation shape for `ListNamespaces`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_namespaces`](crate::client::Client::list_namespaces).
///
/// See [`crate::client::fluent_builders::ListNamespaces`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListNamespaces {
    _private: (),
}
impl ListNamespaces {
    /// Creates a new builder-style object to manufacture [`ListNamespacesInput`](crate::input::ListNamespacesInput)
    pub fn builder() -> crate::input::list_namespaces_input::Builder {
        crate::input::list_namespaces_input::Builder::default()
    }
    /// Creates a new `ListNamespaces` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListNamespaces {
    type Output =
        std::result::Result<crate::output::ListNamespacesOutput, crate::error::ListNamespacesError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_namespaces_error(response)
        } else {
            crate::operation_deser::parse_list_namespaces_response(response)
        }
    }
}

/// Operation shape for `ListOperations`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_operations`](crate::client::Client::list_operations).
///
/// See [`crate::client::fluent_builders::ListOperations`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListOperations {
    _private: (),
}
impl ListOperations {
    /// Creates a new builder-style object to manufacture [`ListOperationsInput`](crate::input::ListOperationsInput)
    pub fn builder() -> crate::input::list_operations_input::Builder {
        crate::input::list_operations_input::Builder::default()
    }
    /// Creates a new `ListOperations` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListOperations {
    type Output =
        std::result::Result<crate::output::ListOperationsOutput, crate::error::ListOperationsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_operations_error(response)
        } else {
            crate::operation_deser::parse_list_operations_response(response)
        }
    }
}

/// Operation shape for `ListServices`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_services`](crate::client::Client::list_services).
///
/// See [`crate::client::fluent_builders::ListServices`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListServices {
    _private: (),
}
impl ListServices {
    /// Creates a new builder-style object to manufacture [`ListServicesInput`](crate::input::ListServicesInput)
    pub fn builder() -> crate::input::list_services_input::Builder {
        crate::input::list_services_input::Builder::default()
    }
    /// Creates a new `ListServices` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListServices {
    type Output =
        std::result::Result<crate::output::ListServicesOutput, crate::error::ListServicesError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_services_error(response)
        } else {
            crate::operation_deser::parse_list_services_response(response)
        }
    }
}

/// Operation shape for `ListTagsForResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_tags_for_resource`](crate::client::Client::list_tags_for_resource).
///
/// See [`crate::client::fluent_builders::ListTagsForResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListTagsForResource {
    _private: (),
}
impl ListTagsForResource {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceInput`](crate::input::ListTagsForResourceInput)
    pub fn builder() -> crate::input::list_tags_for_resource_input::Builder {
        crate::input::list_tags_for_resource_input::Builder::default()
    }
    /// Creates a new `ListTagsForResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTagsForResource {
    type Output = std::result::Result<
        crate::output::ListTagsForResourceOutput,
        crate::error::ListTagsForResourceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_tags_for_resource_error(response)
        } else {
            crate::operation_deser::parse_list_tags_for_resource_response(response)
        }
    }
}

/// Operation shape for `RegisterInstance`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`register_instance`](crate::client::Client::register_instance).
///
/// See [`crate::client::fluent_builders::RegisterInstance`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct RegisterInstance {
    _private: (),
}
impl RegisterInstance {
    /// Creates a new builder-style object to manufacture [`RegisterInstanceInput`](crate::input::RegisterInstanceInput)
    pub fn builder() -> crate::input::register_instance_input::Builder {
        crate::input::register_instance_input::Builder::default()
    }
    /// Creates a new `RegisterInstance` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for RegisterInstance {
    type Output = std::result::Result<
        crate::output::RegisterInstanceOutput,
        crate::error::RegisterInstanceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_register_instance_error(response)
        } else {
            crate::operation_deser::parse_register_instance_response(response)
        }
    }
}

/// Operation shape for `TagResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`tag_resource`](crate::client::Client::tag_resource).
///
/// See [`crate::client::fluent_builders::TagResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct TagResource {
    _private: (),
}
impl TagResource {
    /// Creates a new builder-style object to manufacture [`TagResourceInput`](crate::input::TagResourceInput)
    pub fn builder() -> crate::input::tag_resource_input::Builder {
        crate::input::tag_resource_input::Builder::default()
    }
    /// Creates a new `TagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for TagResource {
    type Output =
        std::result::Result<crate::output::TagResourceOutput, crate::error::TagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_tag_resource_error(response)
        } else {
            crate::operation_deser::parse_tag_resource_response(response)
        }
    }
}

/// Operation shape for `UntagResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`untag_resource`](crate::client::Client::untag_resource).
///
/// See [`crate::client::fluent_builders::UntagResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UntagResource {
    _private: (),
}
impl UntagResource {
    /// Creates a new builder-style object to manufacture [`UntagResourceInput`](crate::input::UntagResourceInput)
    pub fn builder() -> crate::input::untag_resource_input::Builder {
        crate::input::untag_resource_input::Builder::default()
    }
    /// Creates a new `UntagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UntagResource {
    type Output =
        std::result::Result<crate::output::UntagResourceOutput, crate::error::UntagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_untag_resource_error(response)
        } else {
            crate::operation_deser::parse_untag_resource_response(response)
        }
    }
}

/// Operation shape for `UpdateHttpNamespace`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_http_namespace`](crate::client::Client::update_http_namespace).
///
/// See [`crate::client::fluent_builders::UpdateHttpNamespace`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateHttpNamespace {
    _private: (),
}
impl UpdateHttpNamespace {
    /// Creates a new builder-style object to manufacture [`UpdateHttpNamespaceInput`](crate::input::UpdateHttpNamespaceInput)
    pub fn builder() -> crate::input::update_http_namespace_input::Builder {
        crate::input::update_http_namespace_input::Builder::default()
    }
    /// Creates a new `UpdateHttpNamespace` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateHttpNamespace {
    type Output = std::result::Result<
        crate::output::UpdateHttpNamespaceOutput,
        crate::error::UpdateHttpNamespaceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_http_namespace_error(response)
        } else {
            crate::operation_deser::parse_update_http_namespace_response(response)
        }
    }
}

/// Operation shape for `UpdateInstanceCustomHealthStatus`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_instance_custom_health_status`](crate::client::Client::update_instance_custom_health_status).
///
/// See [`crate::client::fluent_builders::UpdateInstanceCustomHealthStatus`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateInstanceCustomHealthStatus {
    _private: (),
}
impl UpdateInstanceCustomHealthStatus {
    /// Creates a new builder-style object to manufacture [`UpdateInstanceCustomHealthStatusInput`](crate::input::UpdateInstanceCustomHealthStatusInput)
    pub fn builder() -> crate::input::update_instance_custom_health_status_input::Builder {
        crate::input::update_instance_custom_health_status_input::Builder::default()
    }
    /// Creates a new `UpdateInstanceCustomHealthStatus` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateInstanceCustomHealthStatus {
    type Output = std::result::Result<
        crate::output::UpdateInstanceCustomHealthStatusOutput,
        crate::error::UpdateInstanceCustomHealthStatusError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_instance_custom_health_status_error(response)
        } else {
            crate::operation_deser::parse_update_instance_custom_health_status_response(response)
        }
    }
}

/// Operation shape for `UpdatePrivateDnsNamespace`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_private_dns_namespace`](crate::client::Client::update_private_dns_namespace).
///
/// See [`crate::client::fluent_builders::UpdatePrivateDnsNamespace`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdatePrivateDnsNamespace {
    _private: (),
}
impl UpdatePrivateDnsNamespace {
    /// Creates a new builder-style object to manufacture [`UpdatePrivateDnsNamespaceInput`](crate::input::UpdatePrivateDnsNamespaceInput)
    pub fn builder() -> crate::input::update_private_dns_namespace_input::Builder {
        crate::input::update_private_dns_namespace_input::Builder::default()
    }
    /// Creates a new `UpdatePrivateDnsNamespace` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdatePrivateDnsNamespace {
    type Output = std::result::Result<
        crate::output::UpdatePrivateDnsNamespaceOutput,
        crate::error::UpdatePrivateDnsNamespaceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_private_dns_namespace_error(response)
        } else {
            crate::operation_deser::parse_update_private_dns_namespace_response(response)
        }
    }
}

/// Operation shape for `UpdatePublicDnsNamespace`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_public_dns_namespace`](crate::client::Client::update_public_dns_namespace).
///
/// See [`crate::client::fluent_builders::UpdatePublicDnsNamespace`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdatePublicDnsNamespace {
    _private: (),
}
impl UpdatePublicDnsNamespace {
    /// Creates a new builder-style object to manufacture [`UpdatePublicDnsNamespaceInput`](crate::input::UpdatePublicDnsNamespaceInput)
    pub fn builder() -> crate::input::update_public_dns_namespace_input::Builder {
        crate::input::update_public_dns_namespace_input::Builder::default()
    }
    /// Creates a new `UpdatePublicDnsNamespace` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdatePublicDnsNamespace {
    type Output = std::result::Result<
        crate::output::UpdatePublicDnsNamespaceOutput,
        crate::error::UpdatePublicDnsNamespaceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_public_dns_namespace_error(response)
        } else {
            crate::operation_deser::parse_update_public_dns_namespace_response(response)
        }
    }
}

/// Operation shape for `UpdateService`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_service`](crate::client::Client::update_service).
///
/// See [`crate::client::fluent_builders::UpdateService`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateService {
    _private: (),
}
impl UpdateService {
    /// Creates a new builder-style object to manufacture [`UpdateServiceInput`](crate::input::UpdateServiceInput)
    pub fn builder() -> crate::input::update_service_input::Builder {
        crate::input::update_service_input::Builder::default()
    }
    /// Creates a new `UpdateService` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateService {
    type Output =
        std::result::Result<crate::output::UpdateServiceOutput, crate::error::UpdateServiceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_service_error(response)
        } else {
            crate::operation_deser::parse_update_service_response(response)
        }
    }
}
