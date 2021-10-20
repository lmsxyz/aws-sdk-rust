// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `BatchAssociateClientDeviceWithCoreDevice`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`batch_associate_client_device_with_core_device`](crate::client::Client::batch_associate_client_device_with_core_device).
///
/// See [`crate::client::fluent_builders::BatchAssociateClientDeviceWithCoreDevice`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct BatchAssociateClientDeviceWithCoreDevice {
    _private: (),
}
impl BatchAssociateClientDeviceWithCoreDevice {
    /// Creates a new builder-style object to manufacture [`BatchAssociateClientDeviceWithCoreDeviceInput`](crate::input::BatchAssociateClientDeviceWithCoreDeviceInput)
    pub fn builder() -> crate::input::batch_associate_client_device_with_core_device_input::Builder
    {
        crate::input::batch_associate_client_device_with_core_device_input::Builder::default()
    }
    /// Creates a new `BatchAssociateClientDeviceWithCoreDevice` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for BatchAssociateClientDeviceWithCoreDevice {
    type Output = std::result::Result<
        crate::output::BatchAssociateClientDeviceWithCoreDeviceOutput,
        crate::error::BatchAssociateClientDeviceWithCoreDeviceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_batch_associate_client_device_with_core_device_error(
                response,
            )
        } else {
            crate::operation_deser::parse_batch_associate_client_device_with_core_device_response(
                response,
            )
        }
    }
}

/// Operation shape for `BatchDisassociateClientDeviceFromCoreDevice`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`batch_disassociate_client_device_from_core_device`](crate::client::Client::batch_disassociate_client_device_from_core_device).
///
/// See [`crate::client::fluent_builders::BatchDisassociateClientDeviceFromCoreDevice`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct BatchDisassociateClientDeviceFromCoreDevice {
    _private: (),
}
impl BatchDisassociateClientDeviceFromCoreDevice {
    /// Creates a new builder-style object to manufacture [`BatchDisassociateClientDeviceFromCoreDeviceInput`](crate::input::BatchDisassociateClientDeviceFromCoreDeviceInput)
    pub fn builder(
    ) -> crate::input::batch_disassociate_client_device_from_core_device_input::Builder {
        crate::input::batch_disassociate_client_device_from_core_device_input::Builder::default()
    }
    /// Creates a new `BatchDisassociateClientDeviceFromCoreDevice` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse
    for BatchDisassociateClientDeviceFromCoreDevice
{
    type Output = std::result::Result<
        crate::output::BatchDisassociateClientDeviceFromCoreDeviceOutput,
        crate::error::BatchDisassociateClientDeviceFromCoreDeviceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_batch_disassociate_client_device_from_core_device_error(
                response,
            )
        } else {
            crate::operation_deser::parse_batch_disassociate_client_device_from_core_device_response(
                response,
            )
        }
    }
}

/// Operation shape for `CancelDeployment`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`cancel_deployment`](crate::client::Client::cancel_deployment).
///
/// See [`crate::client::fluent_builders::CancelDeployment`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CancelDeployment {
    _private: (),
}
impl CancelDeployment {
    /// Creates a new builder-style object to manufacture [`CancelDeploymentInput`](crate::input::CancelDeploymentInput)
    pub fn builder() -> crate::input::cancel_deployment_input::Builder {
        crate::input::cancel_deployment_input::Builder::default()
    }
    /// Creates a new `CancelDeployment` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CancelDeployment {
    type Output = std::result::Result<
        crate::output::CancelDeploymentOutput,
        crate::error::CancelDeploymentError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_cancel_deployment_error(response)
        } else {
            crate::operation_deser::parse_cancel_deployment_response(response)
        }
    }
}

/// Operation shape for `CreateComponentVersion`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_component_version`](crate::client::Client::create_component_version).
///
/// See [`crate::client::fluent_builders::CreateComponentVersion`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateComponentVersion {
    _private: (),
}
impl CreateComponentVersion {
    /// Creates a new builder-style object to manufacture [`CreateComponentVersionInput`](crate::input::CreateComponentVersionInput)
    pub fn builder() -> crate::input::create_component_version_input::Builder {
        crate::input::create_component_version_input::Builder::default()
    }
    /// Creates a new `CreateComponentVersion` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateComponentVersion {
    type Output = std::result::Result<
        crate::output::CreateComponentVersionOutput,
        crate::error::CreateComponentVersionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 201 {
            crate::operation_deser::parse_create_component_version_error(response)
        } else {
            crate::operation_deser::parse_create_component_version_response(response)
        }
    }
}

/// Operation shape for `CreateDeployment`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_deployment`](crate::client::Client::create_deployment).
///
/// See [`crate::client::fluent_builders::CreateDeployment`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateDeployment {
    _private: (),
}
impl CreateDeployment {
    /// Creates a new builder-style object to manufacture [`CreateDeploymentInput`](crate::input::CreateDeploymentInput)
    pub fn builder() -> crate::input::create_deployment_input::Builder {
        crate::input::create_deployment_input::Builder::default()
    }
    /// Creates a new `CreateDeployment` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateDeployment {
    type Output = std::result::Result<
        crate::output::CreateDeploymentOutput,
        crate::error::CreateDeploymentError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 201 {
            crate::operation_deser::parse_create_deployment_error(response)
        } else {
            crate::operation_deser::parse_create_deployment_response(response)
        }
    }
}

/// Operation shape for `DeleteComponent`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_component`](crate::client::Client::delete_component).
///
/// See [`crate::client::fluent_builders::DeleteComponent`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteComponent {
    _private: (),
}
impl DeleteComponent {
    /// Creates a new builder-style object to manufacture [`DeleteComponentInput`](crate::input::DeleteComponentInput)
    pub fn builder() -> crate::input::delete_component_input::Builder {
        crate::input::delete_component_input::Builder::default()
    }
    /// Creates a new `DeleteComponent` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteComponent {
    type Output = std::result::Result<
        crate::output::DeleteComponentOutput,
        crate::error::DeleteComponentError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 204 {
            crate::operation_deser::parse_delete_component_error(response)
        } else {
            crate::operation_deser::parse_delete_component_response(response)
        }
    }
}

/// Operation shape for `DeleteCoreDevice`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_core_device`](crate::client::Client::delete_core_device).
///
/// See [`crate::client::fluent_builders::DeleteCoreDevice`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteCoreDevice {
    _private: (),
}
impl DeleteCoreDevice {
    /// Creates a new builder-style object to manufacture [`DeleteCoreDeviceInput`](crate::input::DeleteCoreDeviceInput)
    pub fn builder() -> crate::input::delete_core_device_input::Builder {
        crate::input::delete_core_device_input::Builder::default()
    }
    /// Creates a new `DeleteCoreDevice` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteCoreDevice {
    type Output = std::result::Result<
        crate::output::DeleteCoreDeviceOutput,
        crate::error::DeleteCoreDeviceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 204 {
            crate::operation_deser::parse_delete_core_device_error(response)
        } else {
            crate::operation_deser::parse_delete_core_device_response(response)
        }
    }
}

/// Operation shape for `DescribeComponent`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_component`](crate::client::Client::describe_component).
///
/// See [`crate::client::fluent_builders::DescribeComponent`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeComponent {
    _private: (),
}
impl DescribeComponent {
    /// Creates a new builder-style object to manufacture [`DescribeComponentInput`](crate::input::DescribeComponentInput)
    pub fn builder() -> crate::input::describe_component_input::Builder {
        crate::input::describe_component_input::Builder::default()
    }
    /// Creates a new `DescribeComponent` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeComponent {
    type Output = std::result::Result<
        crate::output::DescribeComponentOutput,
        crate::error::DescribeComponentError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_component_error(response)
        } else {
            crate::operation_deser::parse_describe_component_response(response)
        }
    }
}

/// Operation shape for `GetComponent`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_component`](crate::client::Client::get_component).
///
/// See [`crate::client::fluent_builders::GetComponent`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetComponent {
    _private: (),
}
impl GetComponent {
    /// Creates a new builder-style object to manufacture [`GetComponentInput`](crate::input::GetComponentInput)
    pub fn builder() -> crate::input::get_component_input::Builder {
        crate::input::get_component_input::Builder::default()
    }
    /// Creates a new `GetComponent` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetComponent {
    type Output =
        std::result::Result<crate::output::GetComponentOutput, crate::error::GetComponentError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_component_error(response)
        } else {
            crate::operation_deser::parse_get_component_response(response)
        }
    }
}

/// Operation shape for `GetComponentVersionArtifact`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_component_version_artifact`](crate::client::Client::get_component_version_artifact).
///
/// See [`crate::client::fluent_builders::GetComponentVersionArtifact`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetComponentVersionArtifact {
    _private: (),
}
impl GetComponentVersionArtifact {
    /// Creates a new builder-style object to manufacture [`GetComponentVersionArtifactInput`](crate::input::GetComponentVersionArtifactInput)
    pub fn builder() -> crate::input::get_component_version_artifact_input::Builder {
        crate::input::get_component_version_artifact_input::Builder::default()
    }
    /// Creates a new `GetComponentVersionArtifact` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetComponentVersionArtifact {
    type Output = std::result::Result<
        crate::output::GetComponentVersionArtifactOutput,
        crate::error::GetComponentVersionArtifactError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_component_version_artifact_error(response)
        } else {
            crate::operation_deser::parse_get_component_version_artifact_response(response)
        }
    }
}

/// Operation shape for `GetCoreDevice`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_core_device`](crate::client::Client::get_core_device).
///
/// See [`crate::client::fluent_builders::GetCoreDevice`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetCoreDevice {
    _private: (),
}
impl GetCoreDevice {
    /// Creates a new builder-style object to manufacture [`GetCoreDeviceInput`](crate::input::GetCoreDeviceInput)
    pub fn builder() -> crate::input::get_core_device_input::Builder {
        crate::input::get_core_device_input::Builder::default()
    }
    /// Creates a new `GetCoreDevice` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetCoreDevice {
    type Output =
        std::result::Result<crate::output::GetCoreDeviceOutput, crate::error::GetCoreDeviceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_core_device_error(response)
        } else {
            crate::operation_deser::parse_get_core_device_response(response)
        }
    }
}

/// Operation shape for `GetDeployment`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_deployment`](crate::client::Client::get_deployment).
///
/// See [`crate::client::fluent_builders::GetDeployment`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetDeployment {
    _private: (),
}
impl GetDeployment {
    /// Creates a new builder-style object to manufacture [`GetDeploymentInput`](crate::input::GetDeploymentInput)
    pub fn builder() -> crate::input::get_deployment_input::Builder {
        crate::input::get_deployment_input::Builder::default()
    }
    /// Creates a new `GetDeployment` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetDeployment {
    type Output =
        std::result::Result<crate::output::GetDeploymentOutput, crate::error::GetDeploymentError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_deployment_error(response)
        } else {
            crate::operation_deser::parse_get_deployment_response(response)
        }
    }
}

/// Operation shape for `ListClientDevicesAssociatedWithCoreDevice`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_client_devices_associated_with_core_device`](crate::client::Client::list_client_devices_associated_with_core_device).
///
/// See [`crate::client::fluent_builders::ListClientDevicesAssociatedWithCoreDevice`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListClientDevicesAssociatedWithCoreDevice {
    _private: (),
}
impl ListClientDevicesAssociatedWithCoreDevice {
    /// Creates a new builder-style object to manufacture [`ListClientDevicesAssociatedWithCoreDeviceInput`](crate::input::ListClientDevicesAssociatedWithCoreDeviceInput)
    pub fn builder() -> crate::input::list_client_devices_associated_with_core_device_input::Builder
    {
        crate::input::list_client_devices_associated_with_core_device_input::Builder::default()
    }
    /// Creates a new `ListClientDevicesAssociatedWithCoreDevice` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListClientDevicesAssociatedWithCoreDevice {
    type Output = std::result::Result<
        crate::output::ListClientDevicesAssociatedWithCoreDeviceOutput,
        crate::error::ListClientDevicesAssociatedWithCoreDeviceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_client_devices_associated_with_core_device_error(
                response,
            )
        } else {
            crate::operation_deser::parse_list_client_devices_associated_with_core_device_response(
                response,
            )
        }
    }
}

/// Operation shape for `ListComponents`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_components`](crate::client::Client::list_components).
///
/// See [`crate::client::fluent_builders::ListComponents`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListComponents {
    _private: (),
}
impl ListComponents {
    /// Creates a new builder-style object to manufacture [`ListComponentsInput`](crate::input::ListComponentsInput)
    pub fn builder() -> crate::input::list_components_input::Builder {
        crate::input::list_components_input::Builder::default()
    }
    /// Creates a new `ListComponents` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListComponents {
    type Output =
        std::result::Result<crate::output::ListComponentsOutput, crate::error::ListComponentsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_components_error(response)
        } else {
            crate::operation_deser::parse_list_components_response(response)
        }
    }
}

/// Operation shape for `ListComponentVersions`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_component_versions`](crate::client::Client::list_component_versions).
///
/// See [`crate::client::fluent_builders::ListComponentVersions`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListComponentVersions {
    _private: (),
}
impl ListComponentVersions {
    /// Creates a new builder-style object to manufacture [`ListComponentVersionsInput`](crate::input::ListComponentVersionsInput)
    pub fn builder() -> crate::input::list_component_versions_input::Builder {
        crate::input::list_component_versions_input::Builder::default()
    }
    /// Creates a new `ListComponentVersions` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListComponentVersions {
    type Output = std::result::Result<
        crate::output::ListComponentVersionsOutput,
        crate::error::ListComponentVersionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_component_versions_error(response)
        } else {
            crate::operation_deser::parse_list_component_versions_response(response)
        }
    }
}

/// Operation shape for `ListCoreDevices`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_core_devices`](crate::client::Client::list_core_devices).
///
/// See [`crate::client::fluent_builders::ListCoreDevices`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListCoreDevices {
    _private: (),
}
impl ListCoreDevices {
    /// Creates a new builder-style object to manufacture [`ListCoreDevicesInput`](crate::input::ListCoreDevicesInput)
    pub fn builder() -> crate::input::list_core_devices_input::Builder {
        crate::input::list_core_devices_input::Builder::default()
    }
    /// Creates a new `ListCoreDevices` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListCoreDevices {
    type Output = std::result::Result<
        crate::output::ListCoreDevicesOutput,
        crate::error::ListCoreDevicesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_core_devices_error(response)
        } else {
            crate::operation_deser::parse_list_core_devices_response(response)
        }
    }
}

/// Operation shape for `ListDeployments`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_deployments`](crate::client::Client::list_deployments).
///
/// See [`crate::client::fluent_builders::ListDeployments`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListDeployments {
    _private: (),
}
impl ListDeployments {
    /// Creates a new builder-style object to manufacture [`ListDeploymentsInput`](crate::input::ListDeploymentsInput)
    pub fn builder() -> crate::input::list_deployments_input::Builder {
        crate::input::list_deployments_input::Builder::default()
    }
    /// Creates a new `ListDeployments` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListDeployments {
    type Output = std::result::Result<
        crate::output::ListDeploymentsOutput,
        crate::error::ListDeploymentsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_deployments_error(response)
        } else {
            crate::operation_deser::parse_list_deployments_response(response)
        }
    }
}

/// Operation shape for `ListEffectiveDeployments`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_effective_deployments`](crate::client::Client::list_effective_deployments).
///
/// See [`crate::client::fluent_builders::ListEffectiveDeployments`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListEffectiveDeployments {
    _private: (),
}
impl ListEffectiveDeployments {
    /// Creates a new builder-style object to manufacture [`ListEffectiveDeploymentsInput`](crate::input::ListEffectiveDeploymentsInput)
    pub fn builder() -> crate::input::list_effective_deployments_input::Builder {
        crate::input::list_effective_deployments_input::Builder::default()
    }
    /// Creates a new `ListEffectiveDeployments` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListEffectiveDeployments {
    type Output = std::result::Result<
        crate::output::ListEffectiveDeploymentsOutput,
        crate::error::ListEffectiveDeploymentsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_effective_deployments_error(response)
        } else {
            crate::operation_deser::parse_list_effective_deployments_response(response)
        }
    }
}

/// Operation shape for `ListInstalledComponents`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_installed_components`](crate::client::Client::list_installed_components).
///
/// See [`crate::client::fluent_builders::ListInstalledComponents`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListInstalledComponents {
    _private: (),
}
impl ListInstalledComponents {
    /// Creates a new builder-style object to manufacture [`ListInstalledComponentsInput`](crate::input::ListInstalledComponentsInput)
    pub fn builder() -> crate::input::list_installed_components_input::Builder {
        crate::input::list_installed_components_input::Builder::default()
    }
    /// Creates a new `ListInstalledComponents` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListInstalledComponents {
    type Output = std::result::Result<
        crate::output::ListInstalledComponentsOutput,
        crate::error::ListInstalledComponentsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_installed_components_error(response)
        } else {
            crate::operation_deser::parse_list_installed_components_response(response)
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

/// Operation shape for `ResolveComponentCandidates`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`resolve_component_candidates`](crate::client::Client::resolve_component_candidates).
///
/// See [`crate::client::fluent_builders::ResolveComponentCandidates`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ResolveComponentCandidates {
    _private: (),
}
impl ResolveComponentCandidates {
    /// Creates a new builder-style object to manufacture [`ResolveComponentCandidatesInput`](crate::input::ResolveComponentCandidatesInput)
    pub fn builder() -> crate::input::resolve_component_candidates_input::Builder {
        crate::input::resolve_component_candidates_input::Builder::default()
    }
    /// Creates a new `ResolveComponentCandidates` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ResolveComponentCandidates {
    type Output = std::result::Result<
        crate::output::ResolveComponentCandidatesOutput,
        crate::error::ResolveComponentCandidatesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_resolve_component_candidates_error(response)
        } else {
            crate::operation_deser::parse_resolve_component_candidates_response(response)
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
