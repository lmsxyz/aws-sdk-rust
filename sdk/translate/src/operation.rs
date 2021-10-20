// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateParallelData`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_parallel_data`](crate::client::Client::create_parallel_data).
///
/// See [`crate::client::fluent_builders::CreateParallelData`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateParallelData {
    _private: (),
}
impl CreateParallelData {
    /// Creates a new builder-style object to manufacture [`CreateParallelDataInput`](crate::input::CreateParallelDataInput)
    pub fn builder() -> crate::input::create_parallel_data_input::Builder {
        crate::input::create_parallel_data_input::Builder::default()
    }
    /// Creates a new `CreateParallelData` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateParallelData {
    type Output = std::result::Result<
        crate::output::CreateParallelDataOutput,
        crate::error::CreateParallelDataError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_parallel_data_error(response)
        } else {
            crate::operation_deser::parse_create_parallel_data_response(response)
        }
    }
}

/// Operation shape for `DeleteParallelData`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_parallel_data`](crate::client::Client::delete_parallel_data).
///
/// See [`crate::client::fluent_builders::DeleteParallelData`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteParallelData {
    _private: (),
}
impl DeleteParallelData {
    /// Creates a new builder-style object to manufacture [`DeleteParallelDataInput`](crate::input::DeleteParallelDataInput)
    pub fn builder() -> crate::input::delete_parallel_data_input::Builder {
        crate::input::delete_parallel_data_input::Builder::default()
    }
    /// Creates a new `DeleteParallelData` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteParallelData {
    type Output = std::result::Result<
        crate::output::DeleteParallelDataOutput,
        crate::error::DeleteParallelDataError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_parallel_data_error(response)
        } else {
            crate::operation_deser::parse_delete_parallel_data_response(response)
        }
    }
}

/// Operation shape for `DeleteTerminology`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_terminology`](crate::client::Client::delete_terminology).
///
/// See [`crate::client::fluent_builders::DeleteTerminology`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteTerminology {
    _private: (),
}
impl DeleteTerminology {
    /// Creates a new builder-style object to manufacture [`DeleteTerminologyInput`](crate::input::DeleteTerminologyInput)
    pub fn builder() -> crate::input::delete_terminology_input::Builder {
        crate::input::delete_terminology_input::Builder::default()
    }
    /// Creates a new `DeleteTerminology` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteTerminology {
    type Output = std::result::Result<
        crate::output::DeleteTerminologyOutput,
        crate::error::DeleteTerminologyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_terminology_error(response)
        } else {
            crate::operation_deser::parse_delete_terminology_response(response)
        }
    }
}

/// Operation shape for `DescribeTextTranslationJob`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_text_translation_job`](crate::client::Client::describe_text_translation_job).
///
/// See [`crate::client::fluent_builders::DescribeTextTranslationJob`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeTextTranslationJob {
    _private: (),
}
impl DescribeTextTranslationJob {
    /// Creates a new builder-style object to manufacture [`DescribeTextTranslationJobInput`](crate::input::DescribeTextTranslationJobInput)
    pub fn builder() -> crate::input::describe_text_translation_job_input::Builder {
        crate::input::describe_text_translation_job_input::Builder::default()
    }
    /// Creates a new `DescribeTextTranslationJob` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeTextTranslationJob {
    type Output = std::result::Result<
        crate::output::DescribeTextTranslationJobOutput,
        crate::error::DescribeTextTranslationJobError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_text_translation_job_error(response)
        } else {
            crate::operation_deser::parse_describe_text_translation_job_response(response)
        }
    }
}

/// Operation shape for `GetParallelData`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_parallel_data`](crate::client::Client::get_parallel_data).
///
/// See [`crate::client::fluent_builders::GetParallelData`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetParallelData {
    _private: (),
}
impl GetParallelData {
    /// Creates a new builder-style object to manufacture [`GetParallelDataInput`](crate::input::GetParallelDataInput)
    pub fn builder() -> crate::input::get_parallel_data_input::Builder {
        crate::input::get_parallel_data_input::Builder::default()
    }
    /// Creates a new `GetParallelData` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetParallelData {
    type Output = std::result::Result<
        crate::output::GetParallelDataOutput,
        crate::error::GetParallelDataError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_parallel_data_error(response)
        } else {
            crate::operation_deser::parse_get_parallel_data_response(response)
        }
    }
}

/// Operation shape for `GetTerminology`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_terminology`](crate::client::Client::get_terminology).
///
/// See [`crate::client::fluent_builders::GetTerminology`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetTerminology {
    _private: (),
}
impl GetTerminology {
    /// Creates a new builder-style object to manufacture [`GetTerminologyInput`](crate::input::GetTerminologyInput)
    pub fn builder() -> crate::input::get_terminology_input::Builder {
        crate::input::get_terminology_input::Builder::default()
    }
    /// Creates a new `GetTerminology` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetTerminology {
    type Output =
        std::result::Result<crate::output::GetTerminologyOutput, crate::error::GetTerminologyError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_terminology_error(response)
        } else {
            crate::operation_deser::parse_get_terminology_response(response)
        }
    }
}

/// Operation shape for `ImportTerminology`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`import_terminology`](crate::client::Client::import_terminology).
///
/// See [`crate::client::fluent_builders::ImportTerminology`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ImportTerminology {
    _private: (),
}
impl ImportTerminology {
    /// Creates a new builder-style object to manufacture [`ImportTerminologyInput`](crate::input::ImportTerminologyInput)
    pub fn builder() -> crate::input::import_terminology_input::Builder {
        crate::input::import_terminology_input::Builder::default()
    }
    /// Creates a new `ImportTerminology` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ImportTerminology {
    type Output = std::result::Result<
        crate::output::ImportTerminologyOutput,
        crate::error::ImportTerminologyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_import_terminology_error(response)
        } else {
            crate::operation_deser::parse_import_terminology_response(response)
        }
    }
}

/// Operation shape for `ListParallelData`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_parallel_data`](crate::client::Client::list_parallel_data).
///
/// See [`crate::client::fluent_builders::ListParallelData`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListParallelData {
    _private: (),
}
impl ListParallelData {
    /// Creates a new builder-style object to manufacture [`ListParallelDataInput`](crate::input::ListParallelDataInput)
    pub fn builder() -> crate::input::list_parallel_data_input::Builder {
        crate::input::list_parallel_data_input::Builder::default()
    }
    /// Creates a new `ListParallelData` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListParallelData {
    type Output = std::result::Result<
        crate::output::ListParallelDataOutput,
        crate::error::ListParallelDataError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_parallel_data_error(response)
        } else {
            crate::operation_deser::parse_list_parallel_data_response(response)
        }
    }
}

/// Operation shape for `ListTerminologies`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_terminologies`](crate::client::Client::list_terminologies).
///
/// See [`crate::client::fluent_builders::ListTerminologies`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListTerminologies {
    _private: (),
}
impl ListTerminologies {
    /// Creates a new builder-style object to manufacture [`ListTerminologiesInput`](crate::input::ListTerminologiesInput)
    pub fn builder() -> crate::input::list_terminologies_input::Builder {
        crate::input::list_terminologies_input::Builder::default()
    }
    /// Creates a new `ListTerminologies` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTerminologies {
    type Output = std::result::Result<
        crate::output::ListTerminologiesOutput,
        crate::error::ListTerminologiesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_terminologies_error(response)
        } else {
            crate::operation_deser::parse_list_terminologies_response(response)
        }
    }
}

/// Operation shape for `ListTextTranslationJobs`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_text_translation_jobs`](crate::client::Client::list_text_translation_jobs).
///
/// See [`crate::client::fluent_builders::ListTextTranslationJobs`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListTextTranslationJobs {
    _private: (),
}
impl ListTextTranslationJobs {
    /// Creates a new builder-style object to manufacture [`ListTextTranslationJobsInput`](crate::input::ListTextTranslationJobsInput)
    pub fn builder() -> crate::input::list_text_translation_jobs_input::Builder {
        crate::input::list_text_translation_jobs_input::Builder::default()
    }
    /// Creates a new `ListTextTranslationJobs` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTextTranslationJobs {
    type Output = std::result::Result<
        crate::output::ListTextTranslationJobsOutput,
        crate::error::ListTextTranslationJobsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_text_translation_jobs_error(response)
        } else {
            crate::operation_deser::parse_list_text_translation_jobs_response(response)
        }
    }
}

/// Operation shape for `StartTextTranslationJob`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`start_text_translation_job`](crate::client::Client::start_text_translation_job).
///
/// See [`crate::client::fluent_builders::StartTextTranslationJob`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StartTextTranslationJob {
    _private: (),
}
impl StartTextTranslationJob {
    /// Creates a new builder-style object to manufacture [`StartTextTranslationJobInput`](crate::input::StartTextTranslationJobInput)
    pub fn builder() -> crate::input::start_text_translation_job_input::Builder {
        crate::input::start_text_translation_job_input::Builder::default()
    }
    /// Creates a new `StartTextTranslationJob` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StartTextTranslationJob {
    type Output = std::result::Result<
        crate::output::StartTextTranslationJobOutput,
        crate::error::StartTextTranslationJobError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_start_text_translation_job_error(response)
        } else {
            crate::operation_deser::parse_start_text_translation_job_response(response)
        }
    }
}

/// Operation shape for `StopTextTranslationJob`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`stop_text_translation_job`](crate::client::Client::stop_text_translation_job).
///
/// See [`crate::client::fluent_builders::StopTextTranslationJob`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StopTextTranslationJob {
    _private: (),
}
impl StopTextTranslationJob {
    /// Creates a new builder-style object to manufacture [`StopTextTranslationJobInput`](crate::input::StopTextTranslationJobInput)
    pub fn builder() -> crate::input::stop_text_translation_job_input::Builder {
        crate::input::stop_text_translation_job_input::Builder::default()
    }
    /// Creates a new `StopTextTranslationJob` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StopTextTranslationJob {
    type Output = std::result::Result<
        crate::output::StopTextTranslationJobOutput,
        crate::error::StopTextTranslationJobError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_stop_text_translation_job_error(response)
        } else {
            crate::operation_deser::parse_stop_text_translation_job_response(response)
        }
    }
}

/// Operation shape for `TranslateText`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`translate_text`](crate::client::Client::translate_text).
///
/// See [`crate::client::fluent_builders::TranslateText`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct TranslateText {
    _private: (),
}
impl TranslateText {
    /// Creates a new builder-style object to manufacture [`TranslateTextInput`](crate::input::TranslateTextInput)
    pub fn builder() -> crate::input::translate_text_input::Builder {
        crate::input::translate_text_input::Builder::default()
    }
    /// Creates a new `TranslateText` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for TranslateText {
    type Output =
        std::result::Result<crate::output::TranslateTextOutput, crate::error::TranslateTextError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_translate_text_error(response)
        } else {
            crate::operation_deser::parse_translate_text_response(response)
        }
    }
}

/// Operation shape for `UpdateParallelData`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_parallel_data`](crate::client::Client::update_parallel_data).
///
/// See [`crate::client::fluent_builders::UpdateParallelData`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateParallelData {
    _private: (),
}
impl UpdateParallelData {
    /// Creates a new builder-style object to manufacture [`UpdateParallelDataInput`](crate::input::UpdateParallelDataInput)
    pub fn builder() -> crate::input::update_parallel_data_input::Builder {
        crate::input::update_parallel_data_input::Builder::default()
    }
    /// Creates a new `UpdateParallelData` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateParallelData {
    type Output = std::result::Result<
        crate::output::UpdateParallelDataOutput,
        crate::error::UpdateParallelDataError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_parallel_data_error(response)
        } else {
            crate::operation_deser::parse_update_parallel_data_response(response)
        }
    }
}
