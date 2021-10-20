// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateAlarmModel`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_alarm_model`](crate::client::Client::create_alarm_model).
///
/// See [`crate::client::fluent_builders::CreateAlarmModel`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateAlarmModel {
    _private: (),
}
impl CreateAlarmModel {
    /// Creates a new builder-style object to manufacture [`CreateAlarmModelInput`](crate::input::CreateAlarmModelInput)
    pub fn builder() -> crate::input::create_alarm_model_input::Builder {
        crate::input::create_alarm_model_input::Builder::default()
    }
    /// Creates a new `CreateAlarmModel` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateAlarmModel {
    type Output = std::result::Result<
        crate::output::CreateAlarmModelOutput,
        crate::error::CreateAlarmModelError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_alarm_model_error(response)
        } else {
            crate::operation_deser::parse_create_alarm_model_response(response)
        }
    }
}

/// Operation shape for `CreateDetectorModel`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_detector_model`](crate::client::Client::create_detector_model).
///
/// See [`crate::client::fluent_builders::CreateDetectorModel`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateDetectorModel {
    _private: (),
}
impl CreateDetectorModel {
    /// Creates a new builder-style object to manufacture [`CreateDetectorModelInput`](crate::input::CreateDetectorModelInput)
    pub fn builder() -> crate::input::create_detector_model_input::Builder {
        crate::input::create_detector_model_input::Builder::default()
    }
    /// Creates a new `CreateDetectorModel` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateDetectorModel {
    type Output = std::result::Result<
        crate::output::CreateDetectorModelOutput,
        crate::error::CreateDetectorModelError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_detector_model_error(response)
        } else {
            crate::operation_deser::parse_create_detector_model_response(response)
        }
    }
}

/// Operation shape for `CreateInput`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_input`](crate::client::Client::create_input).
///
/// See [`crate::client::fluent_builders::CreateInput`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateInput {
    _private: (),
}
impl CreateInput {
    /// Creates a new builder-style object to manufacture [`CreateInputInput`](crate::input::CreateInputInput)
    pub fn builder() -> crate::input::create_input_input::Builder {
        crate::input::create_input_input::Builder::default()
    }
    /// Creates a new `CreateInput` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateInput {
    type Output =
        std::result::Result<crate::output::CreateInputOutput, crate::error::CreateInputError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 201 {
            crate::operation_deser::parse_create_input_error(response)
        } else {
            crate::operation_deser::parse_create_input_response(response)
        }
    }
}

/// Operation shape for `DeleteAlarmModel`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_alarm_model`](crate::client::Client::delete_alarm_model).
///
/// See [`crate::client::fluent_builders::DeleteAlarmModel`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteAlarmModel {
    _private: (),
}
impl DeleteAlarmModel {
    /// Creates a new builder-style object to manufacture [`DeleteAlarmModelInput`](crate::input::DeleteAlarmModelInput)
    pub fn builder() -> crate::input::delete_alarm_model_input::Builder {
        crate::input::delete_alarm_model_input::Builder::default()
    }
    /// Creates a new `DeleteAlarmModel` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteAlarmModel {
    type Output = std::result::Result<
        crate::output::DeleteAlarmModelOutput,
        crate::error::DeleteAlarmModelError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 204 {
            crate::operation_deser::parse_delete_alarm_model_error(response)
        } else {
            crate::operation_deser::parse_delete_alarm_model_response(response)
        }
    }
}

/// Operation shape for `DeleteDetectorModel`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_detector_model`](crate::client::Client::delete_detector_model).
///
/// See [`crate::client::fluent_builders::DeleteDetectorModel`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteDetectorModel {
    _private: (),
}
impl DeleteDetectorModel {
    /// Creates a new builder-style object to manufacture [`DeleteDetectorModelInput`](crate::input::DeleteDetectorModelInput)
    pub fn builder() -> crate::input::delete_detector_model_input::Builder {
        crate::input::delete_detector_model_input::Builder::default()
    }
    /// Creates a new `DeleteDetectorModel` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteDetectorModel {
    type Output = std::result::Result<
        crate::output::DeleteDetectorModelOutput,
        crate::error::DeleteDetectorModelError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 204 {
            crate::operation_deser::parse_delete_detector_model_error(response)
        } else {
            crate::operation_deser::parse_delete_detector_model_response(response)
        }
    }
}

/// Operation shape for `DeleteInput`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_input`](crate::client::Client::delete_input).
///
/// See [`crate::client::fluent_builders::DeleteInput`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteInput {
    _private: (),
}
impl DeleteInput {
    /// Creates a new builder-style object to manufacture [`DeleteInputInput`](crate::input::DeleteInputInput)
    pub fn builder() -> crate::input::delete_input_input::Builder {
        crate::input::delete_input_input::Builder::default()
    }
    /// Creates a new `DeleteInput` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteInput {
    type Output =
        std::result::Result<crate::output::DeleteInputOutput, crate::error::DeleteInputError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_input_error(response)
        } else {
            crate::operation_deser::parse_delete_input_response(response)
        }
    }
}

/// Operation shape for `DescribeAlarmModel`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_alarm_model`](crate::client::Client::describe_alarm_model).
///
/// See [`crate::client::fluent_builders::DescribeAlarmModel`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeAlarmModel {
    _private: (),
}
impl DescribeAlarmModel {
    /// Creates a new builder-style object to manufacture [`DescribeAlarmModelInput`](crate::input::DescribeAlarmModelInput)
    pub fn builder() -> crate::input::describe_alarm_model_input::Builder {
        crate::input::describe_alarm_model_input::Builder::default()
    }
    /// Creates a new `DescribeAlarmModel` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeAlarmModel {
    type Output = std::result::Result<
        crate::output::DescribeAlarmModelOutput,
        crate::error::DescribeAlarmModelError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_alarm_model_error(response)
        } else {
            crate::operation_deser::parse_describe_alarm_model_response(response)
        }
    }
}

/// Operation shape for `DescribeDetectorModel`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_detector_model`](crate::client::Client::describe_detector_model).
///
/// See [`crate::client::fluent_builders::DescribeDetectorModel`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeDetectorModel {
    _private: (),
}
impl DescribeDetectorModel {
    /// Creates a new builder-style object to manufacture [`DescribeDetectorModelInput`](crate::input::DescribeDetectorModelInput)
    pub fn builder() -> crate::input::describe_detector_model_input::Builder {
        crate::input::describe_detector_model_input::Builder::default()
    }
    /// Creates a new `DescribeDetectorModel` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeDetectorModel {
    type Output = std::result::Result<
        crate::output::DescribeDetectorModelOutput,
        crate::error::DescribeDetectorModelError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_detector_model_error(response)
        } else {
            crate::operation_deser::parse_describe_detector_model_response(response)
        }
    }
}

/// Operation shape for `DescribeDetectorModelAnalysis`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_detector_model_analysis`](crate::client::Client::describe_detector_model_analysis).
///
/// See [`crate::client::fluent_builders::DescribeDetectorModelAnalysis`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeDetectorModelAnalysis {
    _private: (),
}
impl DescribeDetectorModelAnalysis {
    /// Creates a new builder-style object to manufacture [`DescribeDetectorModelAnalysisInput`](crate::input::DescribeDetectorModelAnalysisInput)
    pub fn builder() -> crate::input::describe_detector_model_analysis_input::Builder {
        crate::input::describe_detector_model_analysis_input::Builder::default()
    }
    /// Creates a new `DescribeDetectorModelAnalysis` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeDetectorModelAnalysis {
    type Output = std::result::Result<
        crate::output::DescribeDetectorModelAnalysisOutput,
        crate::error::DescribeDetectorModelAnalysisError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_detector_model_analysis_error(response)
        } else {
            crate::operation_deser::parse_describe_detector_model_analysis_response(response)
        }
    }
}

/// Operation shape for `DescribeInput`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_input`](crate::client::Client::describe_input).
///
/// See [`crate::client::fluent_builders::DescribeInput`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeInput {
    _private: (),
}
impl DescribeInput {
    /// Creates a new builder-style object to manufacture [`DescribeInputInput`](crate::input::DescribeInputInput)
    pub fn builder() -> crate::input::describe_input_input::Builder {
        crate::input::describe_input_input::Builder::default()
    }
    /// Creates a new `DescribeInput` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeInput {
    type Output =
        std::result::Result<crate::output::DescribeInputOutput, crate::error::DescribeInputError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_input_error(response)
        } else {
            crate::operation_deser::parse_describe_input_response(response)
        }
    }
}

/// Operation shape for `DescribeLoggingOptions`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_logging_options`](crate::client::Client::describe_logging_options).
///
/// See [`crate::client::fluent_builders::DescribeLoggingOptions`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeLoggingOptions {
    _private: (),
}
impl DescribeLoggingOptions {
    /// Creates a new builder-style object to manufacture [`DescribeLoggingOptionsInput`](crate::input::DescribeLoggingOptionsInput)
    pub fn builder() -> crate::input::describe_logging_options_input::Builder {
        crate::input::describe_logging_options_input::Builder::default()
    }
    /// Creates a new `DescribeLoggingOptions` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeLoggingOptions {
    type Output = std::result::Result<
        crate::output::DescribeLoggingOptionsOutput,
        crate::error::DescribeLoggingOptionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_logging_options_error(response)
        } else {
            crate::operation_deser::parse_describe_logging_options_response(response)
        }
    }
}

/// Operation shape for `GetDetectorModelAnalysisResults`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_detector_model_analysis_results`](crate::client::Client::get_detector_model_analysis_results).
///
/// See [`crate::client::fluent_builders::GetDetectorModelAnalysisResults`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetDetectorModelAnalysisResults {
    _private: (),
}
impl GetDetectorModelAnalysisResults {
    /// Creates a new builder-style object to manufacture [`GetDetectorModelAnalysisResultsInput`](crate::input::GetDetectorModelAnalysisResultsInput)
    pub fn builder() -> crate::input::get_detector_model_analysis_results_input::Builder {
        crate::input::get_detector_model_analysis_results_input::Builder::default()
    }
    /// Creates a new `GetDetectorModelAnalysisResults` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetDetectorModelAnalysisResults {
    type Output = std::result::Result<
        crate::output::GetDetectorModelAnalysisResultsOutput,
        crate::error::GetDetectorModelAnalysisResultsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_detector_model_analysis_results_error(response)
        } else {
            crate::operation_deser::parse_get_detector_model_analysis_results_response(response)
        }
    }
}

/// Operation shape for `ListAlarmModels`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_alarm_models`](crate::client::Client::list_alarm_models).
///
/// See [`crate::client::fluent_builders::ListAlarmModels`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListAlarmModels {
    _private: (),
}
impl ListAlarmModels {
    /// Creates a new builder-style object to manufacture [`ListAlarmModelsInput`](crate::input::ListAlarmModelsInput)
    pub fn builder() -> crate::input::list_alarm_models_input::Builder {
        crate::input::list_alarm_models_input::Builder::default()
    }
    /// Creates a new `ListAlarmModels` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListAlarmModels {
    type Output = std::result::Result<
        crate::output::ListAlarmModelsOutput,
        crate::error::ListAlarmModelsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_alarm_models_error(response)
        } else {
            crate::operation_deser::parse_list_alarm_models_response(response)
        }
    }
}

/// Operation shape for `ListAlarmModelVersions`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_alarm_model_versions`](crate::client::Client::list_alarm_model_versions).
///
/// See [`crate::client::fluent_builders::ListAlarmModelVersions`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListAlarmModelVersions {
    _private: (),
}
impl ListAlarmModelVersions {
    /// Creates a new builder-style object to manufacture [`ListAlarmModelVersionsInput`](crate::input::ListAlarmModelVersionsInput)
    pub fn builder() -> crate::input::list_alarm_model_versions_input::Builder {
        crate::input::list_alarm_model_versions_input::Builder::default()
    }
    /// Creates a new `ListAlarmModelVersions` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListAlarmModelVersions {
    type Output = std::result::Result<
        crate::output::ListAlarmModelVersionsOutput,
        crate::error::ListAlarmModelVersionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_alarm_model_versions_error(response)
        } else {
            crate::operation_deser::parse_list_alarm_model_versions_response(response)
        }
    }
}

/// Operation shape for `ListDetectorModels`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_detector_models`](crate::client::Client::list_detector_models).
///
/// See [`crate::client::fluent_builders::ListDetectorModels`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListDetectorModels {
    _private: (),
}
impl ListDetectorModels {
    /// Creates a new builder-style object to manufacture [`ListDetectorModelsInput`](crate::input::ListDetectorModelsInput)
    pub fn builder() -> crate::input::list_detector_models_input::Builder {
        crate::input::list_detector_models_input::Builder::default()
    }
    /// Creates a new `ListDetectorModels` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListDetectorModels {
    type Output = std::result::Result<
        crate::output::ListDetectorModelsOutput,
        crate::error::ListDetectorModelsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_detector_models_error(response)
        } else {
            crate::operation_deser::parse_list_detector_models_response(response)
        }
    }
}

/// Operation shape for `ListDetectorModelVersions`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_detector_model_versions`](crate::client::Client::list_detector_model_versions).
///
/// See [`crate::client::fluent_builders::ListDetectorModelVersions`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListDetectorModelVersions {
    _private: (),
}
impl ListDetectorModelVersions {
    /// Creates a new builder-style object to manufacture [`ListDetectorModelVersionsInput`](crate::input::ListDetectorModelVersionsInput)
    pub fn builder() -> crate::input::list_detector_model_versions_input::Builder {
        crate::input::list_detector_model_versions_input::Builder::default()
    }
    /// Creates a new `ListDetectorModelVersions` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListDetectorModelVersions {
    type Output = std::result::Result<
        crate::output::ListDetectorModelVersionsOutput,
        crate::error::ListDetectorModelVersionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_detector_model_versions_error(response)
        } else {
            crate::operation_deser::parse_list_detector_model_versions_response(response)
        }
    }
}

/// Operation shape for `ListInputRoutings`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_input_routings`](crate::client::Client::list_input_routings).
///
/// See [`crate::client::fluent_builders::ListInputRoutings`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListInputRoutings {
    _private: (),
}
impl ListInputRoutings {
    /// Creates a new builder-style object to manufacture [`ListInputRoutingsInput`](crate::input::ListInputRoutingsInput)
    pub fn builder() -> crate::input::list_input_routings_input::Builder {
        crate::input::list_input_routings_input::Builder::default()
    }
    /// Creates a new `ListInputRoutings` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListInputRoutings {
    type Output = std::result::Result<
        crate::output::ListInputRoutingsOutput,
        crate::error::ListInputRoutingsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_input_routings_error(response)
        } else {
            crate::operation_deser::parse_list_input_routings_response(response)
        }
    }
}

/// Operation shape for `ListInputs`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_inputs`](crate::client::Client::list_inputs).
///
/// See [`crate::client::fluent_builders::ListInputs`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListInputs {
    _private: (),
}
impl ListInputs {
    /// Creates a new builder-style object to manufacture [`ListInputsInput`](crate::input::ListInputsInput)
    pub fn builder() -> crate::input::list_inputs_input::Builder {
        crate::input::list_inputs_input::Builder::default()
    }
    /// Creates a new `ListInputs` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListInputs {
    type Output =
        std::result::Result<crate::output::ListInputsOutput, crate::error::ListInputsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_inputs_error(response)
        } else {
            crate::operation_deser::parse_list_inputs_response(response)
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

/// Operation shape for `PutLoggingOptions`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`put_logging_options`](crate::client::Client::put_logging_options).
///
/// See [`crate::client::fluent_builders::PutLoggingOptions`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct PutLoggingOptions {
    _private: (),
}
impl PutLoggingOptions {
    /// Creates a new builder-style object to manufacture [`PutLoggingOptionsInput`](crate::input::PutLoggingOptionsInput)
    pub fn builder() -> crate::input::put_logging_options_input::Builder {
        crate::input::put_logging_options_input::Builder::default()
    }
    /// Creates a new `PutLoggingOptions` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutLoggingOptions {
    type Output = std::result::Result<
        crate::output::PutLoggingOptionsOutput,
        crate::error::PutLoggingOptionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_logging_options_error(response)
        } else {
            crate::operation_deser::parse_put_logging_options_response(response)
        }
    }
}

/// Operation shape for `StartDetectorModelAnalysis`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`start_detector_model_analysis`](crate::client::Client::start_detector_model_analysis).
///
/// See [`crate::client::fluent_builders::StartDetectorModelAnalysis`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StartDetectorModelAnalysis {
    _private: (),
}
impl StartDetectorModelAnalysis {
    /// Creates a new builder-style object to manufacture [`StartDetectorModelAnalysisInput`](crate::input::StartDetectorModelAnalysisInput)
    pub fn builder() -> crate::input::start_detector_model_analysis_input::Builder {
        crate::input::start_detector_model_analysis_input::Builder::default()
    }
    /// Creates a new `StartDetectorModelAnalysis` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StartDetectorModelAnalysis {
    type Output = std::result::Result<
        crate::output::StartDetectorModelAnalysisOutput,
        crate::error::StartDetectorModelAnalysisError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_start_detector_model_analysis_error(response)
        } else {
            crate::operation_deser::parse_start_detector_model_analysis_response(response)
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

/// Operation shape for `UpdateAlarmModel`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_alarm_model`](crate::client::Client::update_alarm_model).
///
/// See [`crate::client::fluent_builders::UpdateAlarmModel`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateAlarmModel {
    _private: (),
}
impl UpdateAlarmModel {
    /// Creates a new builder-style object to manufacture [`UpdateAlarmModelInput`](crate::input::UpdateAlarmModelInput)
    pub fn builder() -> crate::input::update_alarm_model_input::Builder {
        crate::input::update_alarm_model_input::Builder::default()
    }
    /// Creates a new `UpdateAlarmModel` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateAlarmModel {
    type Output = std::result::Result<
        crate::output::UpdateAlarmModelOutput,
        crate::error::UpdateAlarmModelError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_alarm_model_error(response)
        } else {
            crate::operation_deser::parse_update_alarm_model_response(response)
        }
    }
}

/// Operation shape for `UpdateDetectorModel`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_detector_model`](crate::client::Client::update_detector_model).
///
/// See [`crate::client::fluent_builders::UpdateDetectorModel`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateDetectorModel {
    _private: (),
}
impl UpdateDetectorModel {
    /// Creates a new builder-style object to manufacture [`UpdateDetectorModelInput`](crate::input::UpdateDetectorModelInput)
    pub fn builder() -> crate::input::update_detector_model_input::Builder {
        crate::input::update_detector_model_input::Builder::default()
    }
    /// Creates a new `UpdateDetectorModel` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateDetectorModel {
    type Output = std::result::Result<
        crate::output::UpdateDetectorModelOutput,
        crate::error::UpdateDetectorModelError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_detector_model_error(response)
        } else {
            crate::operation_deser::parse_update_detector_model_response(response)
        }
    }
}

/// Operation shape for `UpdateInput`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_input`](crate::client::Client::update_input).
///
/// See [`crate::client::fluent_builders::UpdateInput`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateInput {
    _private: (),
}
impl UpdateInput {
    /// Creates a new builder-style object to manufacture [`UpdateInputInput`](crate::input::UpdateInputInput)
    pub fn builder() -> crate::input::update_input_input::Builder {
        crate::input::update_input_input::Builder::default()
    }
    /// Creates a new `UpdateInput` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateInput {
    type Output =
        std::result::Result<crate::output::UpdateInputOutput, crate::error::UpdateInputError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_input_error(response)
        } else {
            crate::operation_deser::parse_update_input_response(response)
        }
    }
}
