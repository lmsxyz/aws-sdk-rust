// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateDataset`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_dataset`](crate::client::Client::create_dataset).
///
/// See [`crate::client::fluent_builders::CreateDataset`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateDataset {
    _private: (),
}
impl CreateDataset {
    /// Creates a new builder-style object to manufacture [`CreateDatasetInput`](crate::input::CreateDatasetInput)
    pub fn builder() -> crate::input::create_dataset_input::Builder {
        crate::input::create_dataset_input::Builder::default()
    }
    /// Creates a new `CreateDataset` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateDataset {
    type Output =
        std::result::Result<crate::output::CreateDatasetOutput, crate::error::CreateDatasetError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_dataset_error(response)
        } else {
            crate::operation_deser::parse_create_dataset_response(response)
        }
    }
}

/// Operation shape for `CreateInferenceScheduler`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_inference_scheduler`](crate::client::Client::create_inference_scheduler).
///
/// See [`crate::client::fluent_builders::CreateInferenceScheduler`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateInferenceScheduler {
    _private: (),
}
impl CreateInferenceScheduler {
    /// Creates a new builder-style object to manufacture [`CreateInferenceSchedulerInput`](crate::input::CreateInferenceSchedulerInput)
    pub fn builder() -> crate::input::create_inference_scheduler_input::Builder {
        crate::input::create_inference_scheduler_input::Builder::default()
    }
    /// Creates a new `CreateInferenceScheduler` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateInferenceScheduler {
    type Output = std::result::Result<
        crate::output::CreateInferenceSchedulerOutput,
        crate::error::CreateInferenceSchedulerError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_inference_scheduler_error(response)
        } else {
            crate::operation_deser::parse_create_inference_scheduler_response(response)
        }
    }
}

/// Operation shape for `CreateModel`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_model`](crate::client::Client::create_model).
///
/// See [`crate::client::fluent_builders::CreateModel`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateModel {
    _private: (),
}
impl CreateModel {
    /// Creates a new builder-style object to manufacture [`CreateModelInput`](crate::input::CreateModelInput)
    pub fn builder() -> crate::input::create_model_input::Builder {
        crate::input::create_model_input::Builder::default()
    }
    /// Creates a new `CreateModel` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateModel {
    type Output =
        std::result::Result<crate::output::CreateModelOutput, crate::error::CreateModelError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_model_error(response)
        } else {
            crate::operation_deser::parse_create_model_response(response)
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

/// Operation shape for `DeleteInferenceScheduler`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_inference_scheduler`](crate::client::Client::delete_inference_scheduler).
///
/// See [`crate::client::fluent_builders::DeleteInferenceScheduler`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteInferenceScheduler {
    _private: (),
}
impl DeleteInferenceScheduler {
    /// Creates a new builder-style object to manufacture [`DeleteInferenceSchedulerInput`](crate::input::DeleteInferenceSchedulerInput)
    pub fn builder() -> crate::input::delete_inference_scheduler_input::Builder {
        crate::input::delete_inference_scheduler_input::Builder::default()
    }
    /// Creates a new `DeleteInferenceScheduler` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteInferenceScheduler {
    type Output = std::result::Result<
        crate::output::DeleteInferenceSchedulerOutput,
        crate::error::DeleteInferenceSchedulerError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_inference_scheduler_error(response)
        } else {
            crate::operation_deser::parse_delete_inference_scheduler_response(response)
        }
    }
}

/// Operation shape for `DeleteModel`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_model`](crate::client::Client::delete_model).
///
/// See [`crate::client::fluent_builders::DeleteModel`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteModel {
    _private: (),
}
impl DeleteModel {
    /// Creates a new builder-style object to manufacture [`DeleteModelInput`](crate::input::DeleteModelInput)
    pub fn builder() -> crate::input::delete_model_input::Builder {
        crate::input::delete_model_input::Builder::default()
    }
    /// Creates a new `DeleteModel` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteModel {
    type Output =
        std::result::Result<crate::output::DeleteModelOutput, crate::error::DeleteModelError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_model_error(response)
        } else {
            crate::operation_deser::parse_delete_model_response(response)
        }
    }
}

/// Operation shape for `DescribeDataIngestionJob`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_data_ingestion_job`](crate::client::Client::describe_data_ingestion_job).
///
/// See [`crate::client::fluent_builders::DescribeDataIngestionJob`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeDataIngestionJob {
    _private: (),
}
impl DescribeDataIngestionJob {
    /// Creates a new builder-style object to manufacture [`DescribeDataIngestionJobInput`](crate::input::DescribeDataIngestionJobInput)
    pub fn builder() -> crate::input::describe_data_ingestion_job_input::Builder {
        crate::input::describe_data_ingestion_job_input::Builder::default()
    }
    /// Creates a new `DescribeDataIngestionJob` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeDataIngestionJob {
    type Output = std::result::Result<
        crate::output::DescribeDataIngestionJobOutput,
        crate::error::DescribeDataIngestionJobError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_data_ingestion_job_error(response)
        } else {
            crate::operation_deser::parse_describe_data_ingestion_job_response(response)
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

/// Operation shape for `DescribeInferenceScheduler`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_inference_scheduler`](crate::client::Client::describe_inference_scheduler).
///
/// See [`crate::client::fluent_builders::DescribeInferenceScheduler`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeInferenceScheduler {
    _private: (),
}
impl DescribeInferenceScheduler {
    /// Creates a new builder-style object to manufacture [`DescribeInferenceSchedulerInput`](crate::input::DescribeInferenceSchedulerInput)
    pub fn builder() -> crate::input::describe_inference_scheduler_input::Builder {
        crate::input::describe_inference_scheduler_input::Builder::default()
    }
    /// Creates a new `DescribeInferenceScheduler` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeInferenceScheduler {
    type Output = std::result::Result<
        crate::output::DescribeInferenceSchedulerOutput,
        crate::error::DescribeInferenceSchedulerError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_inference_scheduler_error(response)
        } else {
            crate::operation_deser::parse_describe_inference_scheduler_response(response)
        }
    }
}

/// Operation shape for `DescribeModel`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_model`](crate::client::Client::describe_model).
///
/// See [`crate::client::fluent_builders::DescribeModel`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeModel {
    _private: (),
}
impl DescribeModel {
    /// Creates a new builder-style object to manufacture [`DescribeModelInput`](crate::input::DescribeModelInput)
    pub fn builder() -> crate::input::describe_model_input::Builder {
        crate::input::describe_model_input::Builder::default()
    }
    /// Creates a new `DescribeModel` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeModel {
    type Output =
        std::result::Result<crate::output::DescribeModelOutput, crate::error::DescribeModelError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_model_error(response)
        } else {
            crate::operation_deser::parse_describe_model_response(response)
        }
    }
}

/// Operation shape for `ListDataIngestionJobs`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_data_ingestion_jobs`](crate::client::Client::list_data_ingestion_jobs).
///
/// See [`crate::client::fluent_builders::ListDataIngestionJobs`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListDataIngestionJobs {
    _private: (),
}
impl ListDataIngestionJobs {
    /// Creates a new builder-style object to manufacture [`ListDataIngestionJobsInput`](crate::input::ListDataIngestionJobsInput)
    pub fn builder() -> crate::input::list_data_ingestion_jobs_input::Builder {
        crate::input::list_data_ingestion_jobs_input::Builder::default()
    }
    /// Creates a new `ListDataIngestionJobs` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListDataIngestionJobs {
    type Output = std::result::Result<
        crate::output::ListDataIngestionJobsOutput,
        crate::error::ListDataIngestionJobsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_data_ingestion_jobs_error(response)
        } else {
            crate::operation_deser::parse_list_data_ingestion_jobs_response(response)
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

/// Operation shape for `ListInferenceExecutions`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_inference_executions`](crate::client::Client::list_inference_executions).
///
/// See [`crate::client::fluent_builders::ListInferenceExecutions`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListInferenceExecutions {
    _private: (),
}
impl ListInferenceExecutions {
    /// Creates a new builder-style object to manufacture [`ListInferenceExecutionsInput`](crate::input::ListInferenceExecutionsInput)
    pub fn builder() -> crate::input::list_inference_executions_input::Builder {
        crate::input::list_inference_executions_input::Builder::default()
    }
    /// Creates a new `ListInferenceExecutions` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListInferenceExecutions {
    type Output = std::result::Result<
        crate::output::ListInferenceExecutionsOutput,
        crate::error::ListInferenceExecutionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_inference_executions_error(response)
        } else {
            crate::operation_deser::parse_list_inference_executions_response(response)
        }
    }
}

/// Operation shape for `ListInferenceSchedulers`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_inference_schedulers`](crate::client::Client::list_inference_schedulers).
///
/// See [`crate::client::fluent_builders::ListInferenceSchedulers`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListInferenceSchedulers {
    _private: (),
}
impl ListInferenceSchedulers {
    /// Creates a new builder-style object to manufacture [`ListInferenceSchedulersInput`](crate::input::ListInferenceSchedulersInput)
    pub fn builder() -> crate::input::list_inference_schedulers_input::Builder {
        crate::input::list_inference_schedulers_input::Builder::default()
    }
    /// Creates a new `ListInferenceSchedulers` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListInferenceSchedulers {
    type Output = std::result::Result<
        crate::output::ListInferenceSchedulersOutput,
        crate::error::ListInferenceSchedulersError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_inference_schedulers_error(response)
        } else {
            crate::operation_deser::parse_list_inference_schedulers_response(response)
        }
    }
}

/// Operation shape for `ListModels`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_models`](crate::client::Client::list_models).
///
/// See [`crate::client::fluent_builders::ListModels`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListModels {
    _private: (),
}
impl ListModels {
    /// Creates a new builder-style object to manufacture [`ListModelsInput`](crate::input::ListModelsInput)
    pub fn builder() -> crate::input::list_models_input::Builder {
        crate::input::list_models_input::Builder::default()
    }
    /// Creates a new `ListModels` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListModels {
    type Output =
        std::result::Result<crate::output::ListModelsOutput, crate::error::ListModelsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_models_error(response)
        } else {
            crate::operation_deser::parse_list_models_response(response)
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

/// Operation shape for `StartDataIngestionJob`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`start_data_ingestion_job`](crate::client::Client::start_data_ingestion_job).
///
/// See [`crate::client::fluent_builders::StartDataIngestionJob`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StartDataIngestionJob {
    _private: (),
}
impl StartDataIngestionJob {
    /// Creates a new builder-style object to manufacture [`StartDataIngestionJobInput`](crate::input::StartDataIngestionJobInput)
    pub fn builder() -> crate::input::start_data_ingestion_job_input::Builder {
        crate::input::start_data_ingestion_job_input::Builder::default()
    }
    /// Creates a new `StartDataIngestionJob` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StartDataIngestionJob {
    type Output = std::result::Result<
        crate::output::StartDataIngestionJobOutput,
        crate::error::StartDataIngestionJobError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_start_data_ingestion_job_error(response)
        } else {
            crate::operation_deser::parse_start_data_ingestion_job_response(response)
        }
    }
}

/// Operation shape for `StartInferenceScheduler`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`start_inference_scheduler`](crate::client::Client::start_inference_scheduler).
///
/// See [`crate::client::fluent_builders::StartInferenceScheduler`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StartInferenceScheduler {
    _private: (),
}
impl StartInferenceScheduler {
    /// Creates a new builder-style object to manufacture [`StartInferenceSchedulerInput`](crate::input::StartInferenceSchedulerInput)
    pub fn builder() -> crate::input::start_inference_scheduler_input::Builder {
        crate::input::start_inference_scheduler_input::Builder::default()
    }
    /// Creates a new `StartInferenceScheduler` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StartInferenceScheduler {
    type Output = std::result::Result<
        crate::output::StartInferenceSchedulerOutput,
        crate::error::StartInferenceSchedulerError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_start_inference_scheduler_error(response)
        } else {
            crate::operation_deser::parse_start_inference_scheduler_response(response)
        }
    }
}

/// Operation shape for `StopInferenceScheduler`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`stop_inference_scheduler`](crate::client::Client::stop_inference_scheduler).
///
/// See [`crate::client::fluent_builders::StopInferenceScheduler`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StopInferenceScheduler {
    _private: (),
}
impl StopInferenceScheduler {
    /// Creates a new builder-style object to manufacture [`StopInferenceSchedulerInput`](crate::input::StopInferenceSchedulerInput)
    pub fn builder() -> crate::input::stop_inference_scheduler_input::Builder {
        crate::input::stop_inference_scheduler_input::Builder::default()
    }
    /// Creates a new `StopInferenceScheduler` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StopInferenceScheduler {
    type Output = std::result::Result<
        crate::output::StopInferenceSchedulerOutput,
        crate::error::StopInferenceSchedulerError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_stop_inference_scheduler_error(response)
        } else {
            crate::operation_deser::parse_stop_inference_scheduler_response(response)
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

/// Operation shape for `UpdateInferenceScheduler`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_inference_scheduler`](crate::client::Client::update_inference_scheduler).
///
/// See [`crate::client::fluent_builders::UpdateInferenceScheduler`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateInferenceScheduler {
    _private: (),
}
impl UpdateInferenceScheduler {
    /// Creates a new builder-style object to manufacture [`UpdateInferenceSchedulerInput`](crate::input::UpdateInferenceSchedulerInput)
    pub fn builder() -> crate::input::update_inference_scheduler_input::Builder {
        crate::input::update_inference_scheduler_input::Builder::default()
    }
    /// Creates a new `UpdateInferenceScheduler` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateInferenceScheduler {
    type Output = std::result::Result<
        crate::output::UpdateInferenceSchedulerOutput,
        crate::error::UpdateInferenceSchedulerError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_inference_scheduler_error(response)
        } else {
            crate::operation_deser::parse_update_inference_scheduler_response(response)
        }
    }
}
