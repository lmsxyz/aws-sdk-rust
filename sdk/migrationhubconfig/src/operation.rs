// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateHomeRegionControl`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_home_region_control`](crate::client::Client::create_home_region_control).
///
/// See [`crate::client::fluent_builders::CreateHomeRegionControl`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateHomeRegionControl {
    _private: (),
}
impl CreateHomeRegionControl {
    /// Creates a new builder-style object to manufacture [`CreateHomeRegionControlInput`](crate::input::CreateHomeRegionControlInput)
    pub fn builder() -> crate::input::create_home_region_control_input::Builder {
        crate::input::create_home_region_control_input::Builder::default()
    }
    /// Creates a new `CreateHomeRegionControl` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateHomeRegionControl {
    type Output = std::result::Result<
        crate::output::CreateHomeRegionControlOutput,
        crate::error::CreateHomeRegionControlError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_home_region_control_error(response)
        } else {
            crate::operation_deser::parse_create_home_region_control_response(response)
        }
    }
}

/// Operation shape for `DescribeHomeRegionControls`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_home_region_controls`](crate::client::Client::describe_home_region_controls).
///
/// See [`crate::client::fluent_builders::DescribeHomeRegionControls`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeHomeRegionControls {
    _private: (),
}
impl DescribeHomeRegionControls {
    /// Creates a new builder-style object to manufacture [`DescribeHomeRegionControlsInput`](crate::input::DescribeHomeRegionControlsInput)
    pub fn builder() -> crate::input::describe_home_region_controls_input::Builder {
        crate::input::describe_home_region_controls_input::Builder::default()
    }
    /// Creates a new `DescribeHomeRegionControls` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeHomeRegionControls {
    type Output = std::result::Result<
        crate::output::DescribeHomeRegionControlsOutput,
        crate::error::DescribeHomeRegionControlsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_home_region_controls_error(response)
        } else {
            crate::operation_deser::parse_describe_home_region_controls_response(response)
        }
    }
}

/// Operation shape for `GetHomeRegion`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_home_region`](crate::client::Client::get_home_region).
///
/// See [`crate::client::fluent_builders::GetHomeRegion`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetHomeRegion {
    _private: (),
}
impl GetHomeRegion {
    /// Creates a new builder-style object to manufacture [`GetHomeRegionInput`](crate::input::GetHomeRegionInput)
    pub fn builder() -> crate::input::get_home_region_input::Builder {
        crate::input::get_home_region_input::Builder::default()
    }
    /// Creates a new `GetHomeRegion` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetHomeRegion {
    type Output =
        std::result::Result<crate::output::GetHomeRegionOutput, crate::error::GetHomeRegionError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_home_region_error(response)
        } else {
            crate::operation_deser::parse_get_home_region_response(response)
        }
    }
}
