// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CancelJobRun`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`cancel_job_run`](crate::client::Client::cancel_job_run).
///
/// See [`crate::client::fluent_builders::CancelJobRun`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CancelJobRun {
    _private: (),
}
impl CancelJobRun {
    /// Creates a new builder-style object to manufacture [`CancelJobRunInput`](crate::input::CancelJobRunInput)
    pub fn builder() -> crate::input::cancel_job_run_input::Builder {
        crate::input::cancel_job_run_input::Builder::default()
    }
    /// Creates a new `CancelJobRun` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CancelJobRun {
    type Output =
        std::result::Result<crate::output::CancelJobRunOutput, crate::error::CancelJobRunError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_cancel_job_run_error(response)
        } else {
            crate::operation_deser::parse_cancel_job_run_response(response)
        }
    }
}

/// Operation shape for `CreateManagedEndpoint`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_managed_endpoint`](crate::client::Client::create_managed_endpoint).
///
/// See [`crate::client::fluent_builders::CreateManagedEndpoint`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateManagedEndpoint {
    _private: (),
}
impl CreateManagedEndpoint {
    /// Creates a new builder-style object to manufacture [`CreateManagedEndpointInput`](crate::input::CreateManagedEndpointInput)
    pub fn builder() -> crate::input::create_managed_endpoint_input::Builder {
        crate::input::create_managed_endpoint_input::Builder::default()
    }
    /// Creates a new `CreateManagedEndpoint` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateManagedEndpoint {
    type Output = std::result::Result<
        crate::output::CreateManagedEndpointOutput,
        crate::error::CreateManagedEndpointError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_managed_endpoint_error(response)
        } else {
            crate::operation_deser::parse_create_managed_endpoint_response(response)
        }
    }
}

/// Operation shape for `CreateVirtualCluster`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_virtual_cluster`](crate::client::Client::create_virtual_cluster).
///
/// See [`crate::client::fluent_builders::CreateVirtualCluster`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateVirtualCluster {
    _private: (),
}
impl CreateVirtualCluster {
    /// Creates a new builder-style object to manufacture [`CreateVirtualClusterInput`](crate::input::CreateVirtualClusterInput)
    pub fn builder() -> crate::input::create_virtual_cluster_input::Builder {
        crate::input::create_virtual_cluster_input::Builder::default()
    }
    /// Creates a new `CreateVirtualCluster` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateVirtualCluster {
    type Output = std::result::Result<
        crate::output::CreateVirtualClusterOutput,
        crate::error::CreateVirtualClusterError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_virtual_cluster_error(response)
        } else {
            crate::operation_deser::parse_create_virtual_cluster_response(response)
        }
    }
}

/// Operation shape for `DeleteManagedEndpoint`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_managed_endpoint`](crate::client::Client::delete_managed_endpoint).
///
/// See [`crate::client::fluent_builders::DeleteManagedEndpoint`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteManagedEndpoint {
    _private: (),
}
impl DeleteManagedEndpoint {
    /// Creates a new builder-style object to manufacture [`DeleteManagedEndpointInput`](crate::input::DeleteManagedEndpointInput)
    pub fn builder() -> crate::input::delete_managed_endpoint_input::Builder {
        crate::input::delete_managed_endpoint_input::Builder::default()
    }
    /// Creates a new `DeleteManagedEndpoint` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteManagedEndpoint {
    type Output = std::result::Result<
        crate::output::DeleteManagedEndpointOutput,
        crate::error::DeleteManagedEndpointError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_managed_endpoint_error(response)
        } else {
            crate::operation_deser::parse_delete_managed_endpoint_response(response)
        }
    }
}

/// Operation shape for `DeleteVirtualCluster`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_virtual_cluster`](crate::client::Client::delete_virtual_cluster).
///
/// See [`crate::client::fluent_builders::DeleteVirtualCluster`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteVirtualCluster {
    _private: (),
}
impl DeleteVirtualCluster {
    /// Creates a new builder-style object to manufacture [`DeleteVirtualClusterInput`](crate::input::DeleteVirtualClusterInput)
    pub fn builder() -> crate::input::delete_virtual_cluster_input::Builder {
        crate::input::delete_virtual_cluster_input::Builder::default()
    }
    /// Creates a new `DeleteVirtualCluster` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteVirtualCluster {
    type Output = std::result::Result<
        crate::output::DeleteVirtualClusterOutput,
        crate::error::DeleteVirtualClusterError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_virtual_cluster_error(response)
        } else {
            crate::operation_deser::parse_delete_virtual_cluster_response(response)
        }
    }
}

/// Operation shape for `DescribeJobRun`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_job_run`](crate::client::Client::describe_job_run).
///
/// See [`crate::client::fluent_builders::DescribeJobRun`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeJobRun {
    _private: (),
}
impl DescribeJobRun {
    /// Creates a new builder-style object to manufacture [`DescribeJobRunInput`](crate::input::DescribeJobRunInput)
    pub fn builder() -> crate::input::describe_job_run_input::Builder {
        crate::input::describe_job_run_input::Builder::default()
    }
    /// Creates a new `DescribeJobRun` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeJobRun {
    type Output =
        std::result::Result<crate::output::DescribeJobRunOutput, crate::error::DescribeJobRunError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_job_run_error(response)
        } else {
            crate::operation_deser::parse_describe_job_run_response(response)
        }
    }
}

/// Operation shape for `DescribeManagedEndpoint`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_managed_endpoint`](crate::client::Client::describe_managed_endpoint).
///
/// See [`crate::client::fluent_builders::DescribeManagedEndpoint`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeManagedEndpoint {
    _private: (),
}
impl DescribeManagedEndpoint {
    /// Creates a new builder-style object to manufacture [`DescribeManagedEndpointInput`](crate::input::DescribeManagedEndpointInput)
    pub fn builder() -> crate::input::describe_managed_endpoint_input::Builder {
        crate::input::describe_managed_endpoint_input::Builder::default()
    }
    /// Creates a new `DescribeManagedEndpoint` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeManagedEndpoint {
    type Output = std::result::Result<
        crate::output::DescribeManagedEndpointOutput,
        crate::error::DescribeManagedEndpointError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_managed_endpoint_error(response)
        } else {
            crate::operation_deser::parse_describe_managed_endpoint_response(response)
        }
    }
}

/// Operation shape for `DescribeVirtualCluster`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_virtual_cluster`](crate::client::Client::describe_virtual_cluster).
///
/// See [`crate::client::fluent_builders::DescribeVirtualCluster`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeVirtualCluster {
    _private: (),
}
impl DescribeVirtualCluster {
    /// Creates a new builder-style object to manufacture [`DescribeVirtualClusterInput`](crate::input::DescribeVirtualClusterInput)
    pub fn builder() -> crate::input::describe_virtual_cluster_input::Builder {
        crate::input::describe_virtual_cluster_input::Builder::default()
    }
    /// Creates a new `DescribeVirtualCluster` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeVirtualCluster {
    type Output = std::result::Result<
        crate::output::DescribeVirtualClusterOutput,
        crate::error::DescribeVirtualClusterError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_virtual_cluster_error(response)
        } else {
            crate::operation_deser::parse_describe_virtual_cluster_response(response)
        }
    }
}

/// Operation shape for `ListJobRuns`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_job_runs`](crate::client::Client::list_job_runs).
///
/// See [`crate::client::fluent_builders::ListJobRuns`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListJobRuns {
    _private: (),
}
impl ListJobRuns {
    /// Creates a new builder-style object to manufacture [`ListJobRunsInput`](crate::input::ListJobRunsInput)
    pub fn builder() -> crate::input::list_job_runs_input::Builder {
        crate::input::list_job_runs_input::Builder::default()
    }
    /// Creates a new `ListJobRuns` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListJobRuns {
    type Output =
        std::result::Result<crate::output::ListJobRunsOutput, crate::error::ListJobRunsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_job_runs_error(response)
        } else {
            crate::operation_deser::parse_list_job_runs_response(response)
        }
    }
}

/// Operation shape for `ListManagedEndpoints`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_managed_endpoints`](crate::client::Client::list_managed_endpoints).
///
/// See [`crate::client::fluent_builders::ListManagedEndpoints`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListManagedEndpoints {
    _private: (),
}
impl ListManagedEndpoints {
    /// Creates a new builder-style object to manufacture [`ListManagedEndpointsInput`](crate::input::ListManagedEndpointsInput)
    pub fn builder() -> crate::input::list_managed_endpoints_input::Builder {
        crate::input::list_managed_endpoints_input::Builder::default()
    }
    /// Creates a new `ListManagedEndpoints` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListManagedEndpoints {
    type Output = std::result::Result<
        crate::output::ListManagedEndpointsOutput,
        crate::error::ListManagedEndpointsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_managed_endpoints_error(response)
        } else {
            crate::operation_deser::parse_list_managed_endpoints_response(response)
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

/// Operation shape for `ListVirtualClusters`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_virtual_clusters`](crate::client::Client::list_virtual_clusters).
///
/// See [`crate::client::fluent_builders::ListVirtualClusters`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListVirtualClusters {
    _private: (),
}
impl ListVirtualClusters {
    /// Creates a new builder-style object to manufacture [`ListVirtualClustersInput`](crate::input::ListVirtualClustersInput)
    pub fn builder() -> crate::input::list_virtual_clusters_input::Builder {
        crate::input::list_virtual_clusters_input::Builder::default()
    }
    /// Creates a new `ListVirtualClusters` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListVirtualClusters {
    type Output = std::result::Result<
        crate::output::ListVirtualClustersOutput,
        crate::error::ListVirtualClustersError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_virtual_clusters_error(response)
        } else {
            crate::operation_deser::parse_list_virtual_clusters_response(response)
        }
    }
}

/// Operation shape for `StartJobRun`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`start_job_run`](crate::client::Client::start_job_run).
///
/// See [`crate::client::fluent_builders::StartJobRun`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StartJobRun {
    _private: (),
}
impl StartJobRun {
    /// Creates a new builder-style object to manufacture [`StartJobRunInput`](crate::input::StartJobRunInput)
    pub fn builder() -> crate::input::start_job_run_input::Builder {
        crate::input::start_job_run_input::Builder::default()
    }
    /// Creates a new `StartJobRun` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StartJobRun {
    type Output =
        std::result::Result<crate::output::StartJobRunOutput, crate::error::StartJobRunError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_start_job_run_error(response)
        } else {
            crate::operation_deser::parse_start_job_run_response(response)
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
