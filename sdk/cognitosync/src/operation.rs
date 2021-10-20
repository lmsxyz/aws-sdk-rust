// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `BulkPublish`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`bulk_publish`](crate::client::Client::bulk_publish).
///
/// See [`crate::client::fluent_builders::BulkPublish`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct BulkPublish {
    _private: (),
}
impl BulkPublish {
    /// Creates a new builder-style object to manufacture [`BulkPublishInput`](crate::input::BulkPublishInput)
    pub fn builder() -> crate::input::bulk_publish_input::Builder {
        crate::input::bulk_publish_input::Builder::default()
    }
    /// Creates a new `BulkPublish` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for BulkPublish {
    type Output =
        std::result::Result<crate::output::BulkPublishOutput, crate::error::BulkPublishError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_bulk_publish_error(response)
        } else {
            crate::operation_deser::parse_bulk_publish_response(response)
        }
    }
}

/// Operation shape for `DeleteDataset`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_dataset`](crate::client::Client::delete_dataset).
///
/// See [`crate::client::fluent_builders::DeleteDataset`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteDataset {
    _private: (),
}
impl DeleteDataset {
    /// Creates a new builder-style object to manufacture [`DeleteDatasetInput`](crate::input::DeleteDatasetInput)
    pub fn builder() -> crate::input::delete_dataset_input::Builder {
        crate::input::delete_dataset_input::Builder::default()
    }
    /// Creates a new `DeleteDataset` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteDataset {
    type Output =
        std::result::Result<crate::output::DeleteDatasetOutput, crate::error::DeleteDatasetError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_dataset_error(response)
        } else {
            crate::operation_deser::parse_delete_dataset_response(response)
        }
    }
}

/// Operation shape for `DescribeDataset`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_dataset`](crate::client::Client::describe_dataset).
///
/// See [`crate::client::fluent_builders::DescribeDataset`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeDataset {
    _private: (),
}
impl DescribeDataset {
    /// Creates a new builder-style object to manufacture [`DescribeDatasetInput`](crate::input::DescribeDatasetInput)
    pub fn builder() -> crate::input::describe_dataset_input::Builder {
        crate::input::describe_dataset_input::Builder::default()
    }
    /// Creates a new `DescribeDataset` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeDataset {
    type Output = std::result::Result<
        crate::output::DescribeDatasetOutput,
        crate::error::DescribeDatasetError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_dataset_error(response)
        } else {
            crate::operation_deser::parse_describe_dataset_response(response)
        }
    }
}

/// Operation shape for `DescribeIdentityPoolUsage`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_identity_pool_usage`](crate::client::Client::describe_identity_pool_usage).
///
/// See [`crate::client::fluent_builders::DescribeIdentityPoolUsage`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeIdentityPoolUsage {
    _private: (),
}
impl DescribeIdentityPoolUsage {
    /// Creates a new builder-style object to manufacture [`DescribeIdentityPoolUsageInput`](crate::input::DescribeIdentityPoolUsageInput)
    pub fn builder() -> crate::input::describe_identity_pool_usage_input::Builder {
        crate::input::describe_identity_pool_usage_input::Builder::default()
    }
    /// Creates a new `DescribeIdentityPoolUsage` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeIdentityPoolUsage {
    type Output = std::result::Result<
        crate::output::DescribeIdentityPoolUsageOutput,
        crate::error::DescribeIdentityPoolUsageError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_identity_pool_usage_error(response)
        } else {
            crate::operation_deser::parse_describe_identity_pool_usage_response(response)
        }
    }
}

/// Operation shape for `DescribeIdentityUsage`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_identity_usage`](crate::client::Client::describe_identity_usage).
///
/// See [`crate::client::fluent_builders::DescribeIdentityUsage`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeIdentityUsage {
    _private: (),
}
impl DescribeIdentityUsage {
    /// Creates a new builder-style object to manufacture [`DescribeIdentityUsageInput`](crate::input::DescribeIdentityUsageInput)
    pub fn builder() -> crate::input::describe_identity_usage_input::Builder {
        crate::input::describe_identity_usage_input::Builder::default()
    }
    /// Creates a new `DescribeIdentityUsage` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeIdentityUsage {
    type Output = std::result::Result<
        crate::output::DescribeIdentityUsageOutput,
        crate::error::DescribeIdentityUsageError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_identity_usage_error(response)
        } else {
            crate::operation_deser::parse_describe_identity_usage_response(response)
        }
    }
}

/// Operation shape for `GetBulkPublishDetails`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_bulk_publish_details`](crate::client::Client::get_bulk_publish_details).
///
/// See [`crate::client::fluent_builders::GetBulkPublishDetails`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetBulkPublishDetails {
    _private: (),
}
impl GetBulkPublishDetails {
    /// Creates a new builder-style object to manufacture [`GetBulkPublishDetailsInput`](crate::input::GetBulkPublishDetailsInput)
    pub fn builder() -> crate::input::get_bulk_publish_details_input::Builder {
        crate::input::get_bulk_publish_details_input::Builder::default()
    }
    /// Creates a new `GetBulkPublishDetails` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetBulkPublishDetails {
    type Output = std::result::Result<
        crate::output::GetBulkPublishDetailsOutput,
        crate::error::GetBulkPublishDetailsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_bulk_publish_details_error(response)
        } else {
            crate::operation_deser::parse_get_bulk_publish_details_response(response)
        }
    }
}

/// Operation shape for `GetCognitoEvents`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_cognito_events`](crate::client::Client::get_cognito_events).
///
/// See [`crate::client::fluent_builders::GetCognitoEvents`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetCognitoEvents {
    _private: (),
}
impl GetCognitoEvents {
    /// Creates a new builder-style object to manufacture [`GetCognitoEventsInput`](crate::input::GetCognitoEventsInput)
    pub fn builder() -> crate::input::get_cognito_events_input::Builder {
        crate::input::get_cognito_events_input::Builder::default()
    }
    /// Creates a new `GetCognitoEvents` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetCognitoEvents {
    type Output = std::result::Result<
        crate::output::GetCognitoEventsOutput,
        crate::error::GetCognitoEventsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_cognito_events_error(response)
        } else {
            crate::operation_deser::parse_get_cognito_events_response(response)
        }
    }
}

/// Operation shape for `GetIdentityPoolConfiguration`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_identity_pool_configuration`](crate::client::Client::get_identity_pool_configuration).
///
/// See [`crate::client::fluent_builders::GetIdentityPoolConfiguration`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetIdentityPoolConfiguration {
    _private: (),
}
impl GetIdentityPoolConfiguration {
    /// Creates a new builder-style object to manufacture [`GetIdentityPoolConfigurationInput`](crate::input::GetIdentityPoolConfigurationInput)
    pub fn builder() -> crate::input::get_identity_pool_configuration_input::Builder {
        crate::input::get_identity_pool_configuration_input::Builder::default()
    }
    /// Creates a new `GetIdentityPoolConfiguration` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetIdentityPoolConfiguration {
    type Output = std::result::Result<
        crate::output::GetIdentityPoolConfigurationOutput,
        crate::error::GetIdentityPoolConfigurationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_identity_pool_configuration_error(response)
        } else {
            crate::operation_deser::parse_get_identity_pool_configuration_response(response)
        }
    }
}

/// Operation shape for `ListDatasets`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_datasets`](crate::client::Client::list_datasets).
///
/// See [`crate::client::fluent_builders::ListDatasets`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListDatasets {
    _private: (),
}
impl ListDatasets {
    /// Creates a new builder-style object to manufacture [`ListDatasetsInput`](crate::input::ListDatasetsInput)
    pub fn builder() -> crate::input::list_datasets_input::Builder {
        crate::input::list_datasets_input::Builder::default()
    }
    /// Creates a new `ListDatasets` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListDatasets {
    type Output =
        std::result::Result<crate::output::ListDatasetsOutput, crate::error::ListDatasetsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_datasets_error(response)
        } else {
            crate::operation_deser::parse_list_datasets_response(response)
        }
    }
}

/// Operation shape for `ListIdentityPoolUsage`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_identity_pool_usage`](crate::client::Client::list_identity_pool_usage).
///
/// See [`crate::client::fluent_builders::ListIdentityPoolUsage`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListIdentityPoolUsage {
    _private: (),
}
impl ListIdentityPoolUsage {
    /// Creates a new builder-style object to manufacture [`ListIdentityPoolUsageInput`](crate::input::ListIdentityPoolUsageInput)
    pub fn builder() -> crate::input::list_identity_pool_usage_input::Builder {
        crate::input::list_identity_pool_usage_input::Builder::default()
    }
    /// Creates a new `ListIdentityPoolUsage` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListIdentityPoolUsage {
    type Output = std::result::Result<
        crate::output::ListIdentityPoolUsageOutput,
        crate::error::ListIdentityPoolUsageError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_identity_pool_usage_error(response)
        } else {
            crate::operation_deser::parse_list_identity_pool_usage_response(response)
        }
    }
}

/// Operation shape for `ListRecords`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_records`](crate::client::Client::list_records).
///
/// See [`crate::client::fluent_builders::ListRecords`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListRecords {
    _private: (),
}
impl ListRecords {
    /// Creates a new builder-style object to manufacture [`ListRecordsInput`](crate::input::ListRecordsInput)
    pub fn builder() -> crate::input::list_records_input::Builder {
        crate::input::list_records_input::Builder::default()
    }
    /// Creates a new `ListRecords` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListRecords {
    type Output =
        std::result::Result<crate::output::ListRecordsOutput, crate::error::ListRecordsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_records_error(response)
        } else {
            crate::operation_deser::parse_list_records_response(response)
        }
    }
}

/// Operation shape for `RegisterDevice`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`register_device`](crate::client::Client::register_device).
///
/// See [`crate::client::fluent_builders::RegisterDevice`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct RegisterDevice {
    _private: (),
}
impl RegisterDevice {
    /// Creates a new builder-style object to manufacture [`RegisterDeviceInput`](crate::input::RegisterDeviceInput)
    pub fn builder() -> crate::input::register_device_input::Builder {
        crate::input::register_device_input::Builder::default()
    }
    /// Creates a new `RegisterDevice` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for RegisterDevice {
    type Output =
        std::result::Result<crate::output::RegisterDeviceOutput, crate::error::RegisterDeviceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_register_device_error(response)
        } else {
            crate::operation_deser::parse_register_device_response(response)
        }
    }
}

/// Operation shape for `SetCognitoEvents`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`set_cognito_events`](crate::client::Client::set_cognito_events).
///
/// See [`crate::client::fluent_builders::SetCognitoEvents`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct SetCognitoEvents {
    _private: (),
}
impl SetCognitoEvents {
    /// Creates a new builder-style object to manufacture [`SetCognitoEventsInput`](crate::input::SetCognitoEventsInput)
    pub fn builder() -> crate::input::set_cognito_events_input::Builder {
        crate::input::set_cognito_events_input::Builder::default()
    }
    /// Creates a new `SetCognitoEvents` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for SetCognitoEvents {
    type Output = std::result::Result<
        crate::output::SetCognitoEventsOutput,
        crate::error::SetCognitoEventsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_set_cognito_events_error(response)
        } else {
            crate::operation_deser::parse_set_cognito_events_response(response)
        }
    }
}

/// Operation shape for `SetIdentityPoolConfiguration`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`set_identity_pool_configuration`](crate::client::Client::set_identity_pool_configuration).
///
/// See [`crate::client::fluent_builders::SetIdentityPoolConfiguration`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct SetIdentityPoolConfiguration {
    _private: (),
}
impl SetIdentityPoolConfiguration {
    /// Creates a new builder-style object to manufacture [`SetIdentityPoolConfigurationInput`](crate::input::SetIdentityPoolConfigurationInput)
    pub fn builder() -> crate::input::set_identity_pool_configuration_input::Builder {
        crate::input::set_identity_pool_configuration_input::Builder::default()
    }
    /// Creates a new `SetIdentityPoolConfiguration` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for SetIdentityPoolConfiguration {
    type Output = std::result::Result<
        crate::output::SetIdentityPoolConfigurationOutput,
        crate::error::SetIdentityPoolConfigurationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_set_identity_pool_configuration_error(response)
        } else {
            crate::operation_deser::parse_set_identity_pool_configuration_response(response)
        }
    }
}

/// Operation shape for `SubscribeToDataset`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`subscribe_to_dataset`](crate::client::Client::subscribe_to_dataset).
///
/// See [`crate::client::fluent_builders::SubscribeToDataset`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct SubscribeToDataset {
    _private: (),
}
impl SubscribeToDataset {
    /// Creates a new builder-style object to manufacture [`SubscribeToDatasetInput`](crate::input::SubscribeToDatasetInput)
    pub fn builder() -> crate::input::subscribe_to_dataset_input::Builder {
        crate::input::subscribe_to_dataset_input::Builder::default()
    }
    /// Creates a new `SubscribeToDataset` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for SubscribeToDataset {
    type Output = std::result::Result<
        crate::output::SubscribeToDatasetOutput,
        crate::error::SubscribeToDatasetError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_subscribe_to_dataset_error(response)
        } else {
            crate::operation_deser::parse_subscribe_to_dataset_response(response)
        }
    }
}

/// Operation shape for `UnsubscribeFromDataset`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`unsubscribe_from_dataset`](crate::client::Client::unsubscribe_from_dataset).
///
/// See [`crate::client::fluent_builders::UnsubscribeFromDataset`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UnsubscribeFromDataset {
    _private: (),
}
impl UnsubscribeFromDataset {
    /// Creates a new builder-style object to manufacture [`UnsubscribeFromDatasetInput`](crate::input::UnsubscribeFromDatasetInput)
    pub fn builder() -> crate::input::unsubscribe_from_dataset_input::Builder {
        crate::input::unsubscribe_from_dataset_input::Builder::default()
    }
    /// Creates a new `UnsubscribeFromDataset` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UnsubscribeFromDataset {
    type Output = std::result::Result<
        crate::output::UnsubscribeFromDatasetOutput,
        crate::error::UnsubscribeFromDatasetError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_unsubscribe_from_dataset_error(response)
        } else {
            crate::operation_deser::parse_unsubscribe_from_dataset_response(response)
        }
    }
}

/// Operation shape for `UpdateRecords`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_records`](crate::client::Client::update_records).
///
/// See [`crate::client::fluent_builders::UpdateRecords`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateRecords {
    _private: (),
}
impl UpdateRecords {
    /// Creates a new builder-style object to manufacture [`UpdateRecordsInput`](crate::input::UpdateRecordsInput)
    pub fn builder() -> crate::input::update_records_input::Builder {
        crate::input::update_records_input::Builder::default()
    }
    /// Creates a new `UpdateRecords` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateRecords {
    type Output =
        std::result::Result<crate::output::UpdateRecordsOutput, crate::error::UpdateRecordsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_records_error(response)
        } else {
            crate::operation_deser::parse_update_records_response(response)
        }
    }
}
