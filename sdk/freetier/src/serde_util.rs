// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn internal_server_exception_correct_errors(
    mut builder: crate::types::error::builders::InternalServerExceptionBuilder,
) -> crate::types::error::builders::InternalServerExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
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

pub(crate) fn get_free_tier_usage_output_output_correct_errors(
    mut builder: crate::operation::get_free_tier_usage::builders::GetFreeTierUsageOutputBuilder,
) -> crate::operation::get_free_tier_usage::builders::GetFreeTierUsageOutputBuilder {
    if builder.free_tier_usages.is_none() {
        builder.free_tier_usages = Some(Default::default())
    }
    builder
}
