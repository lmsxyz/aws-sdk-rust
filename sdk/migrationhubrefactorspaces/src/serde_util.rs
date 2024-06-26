// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn access_denied_exception_correct_errors(
    mut builder: crate::types::error::builders::AccessDeniedExceptionBuilder,
) -> crate::types::error::builders::AccessDeniedExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn conflict_exception_correct_errors(
    mut builder: crate::types::error::builders::ConflictExceptionBuilder,
) -> crate::types::error::builders::ConflictExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    if builder.resource_id.is_none() {
        builder.resource_id = Some(Default::default())
    }
    if builder.resource_type.is_none() {
        builder.resource_type = Some(Default::default())
    }
    builder
}

pub(crate) fn internal_server_exception_correct_errors(
    mut builder: crate::types::error::builders::InternalServerExceptionBuilder,
) -> crate::types::error::builders::InternalServerExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn resource_not_found_exception_correct_errors(
    mut builder: crate::types::error::builders::ResourceNotFoundExceptionBuilder,
) -> crate::types::error::builders::ResourceNotFoundExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    if builder.resource_id.is_none() {
        builder.resource_id = Some(Default::default())
    }
    if builder.resource_type.is_none() {
        builder.resource_type = Some(Default::default())
    }
    builder
}

pub(crate) fn service_quota_exceeded_exception_correct_errors(
    mut builder: crate::types::error::builders::ServiceQuotaExceededExceptionBuilder,
) -> crate::types::error::builders::ServiceQuotaExceededExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    if builder.resource_id.is_none() {
        builder.resource_id = Some(Default::default())
    }
    if builder.resource_type.is_none() {
        builder.resource_type = Some(Default::default())
    }
    if builder.service_code.is_none() {
        builder.service_code = Some(Default::default())
    }
    builder
}

pub(crate) fn throttling_exception_correct_errors(
    mut builder: crate::types::error::builders::ThrottlingExceptionBuilder,
) -> crate::types::error::builders::ThrottlingExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn validation_exception_correct_errors(
    mut builder: crate::types::error::builders::ValidationExceptionBuilder,
) -> crate::types::error::builders::ValidationExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn invalid_resource_policy_exception_correct_errors(
    mut builder: crate::types::error::builders::InvalidResourcePolicyExceptionBuilder,
) -> crate::types::error::builders::InvalidResourcePolicyExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn lambda_endpoint_input_correct_errors(
    mut builder: crate::types::builders::LambdaEndpointInputBuilder,
) -> crate::types::builders::LambdaEndpointInputBuilder {
    if builder.arn.is_none() {
        builder.arn = Some(Default::default())
    }
    builder
}

pub(crate) fn uri_path_route_input_correct_errors(
    mut builder: crate::types::builders::UriPathRouteInputBuilder,
) -> crate::types::builders::UriPathRouteInputBuilder {
    if builder.source_path.is_none() {
        builder.source_path = Some(Default::default())
    }
    if builder.activation_state.is_none() {
        builder.activation_state = "no value was set".parse::<crate::types::RouteActivationState>().ok()
    }
    builder
}

pub(crate) fn url_endpoint_input_correct_errors(
    mut builder: crate::types::builders::UrlEndpointInputBuilder,
) -> crate::types::builders::UrlEndpointInputBuilder {
    if builder.url.is_none() {
        builder.url = Some(Default::default())
    }
    builder
}
