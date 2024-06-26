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
    builder
}

pub(crate) fn service_quota_exceeded_exception_correct_errors(
    mut builder: crate::types::error::builders::ServiceQuotaExceededExceptionBuilder,
) -> crate::types::error::builders::ServiceQuotaExceededExceptionBuilder {
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

pub(crate) fn invalid_state_exception_correct_errors(
    mut builder: crate::types::error::builders::InvalidStateExceptionBuilder,
) -> crate::types::error::builders::InvalidStateExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn invalid_campaign_state_exception_correct_errors(
    mut builder: crate::types::error::builders::InvalidCampaignStateExceptionBuilder,
) -> crate::types::error::builders::InvalidCampaignStateExceptionBuilder {
    if builder.state.is_none() {
        builder.state = "no value was set".parse::<crate::types::CampaignState>().ok()
    }
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn campaign_correct_errors(mut builder: crate::types::builders::CampaignBuilder) -> crate::types::builders::CampaignBuilder {
    if builder.id.is_none() {
        builder.id = Some(Default::default())
    }
    if builder.arn.is_none() {
        builder.arn = Some(Default::default())
    }
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.connect_instance_id.is_none() {
        builder.connect_instance_id = Some(Default::default())
    }
    if builder.dialer_config.is_none() {
        builder.dialer_config = Some(crate::types::DialerConfig::Unknown)
    }
    if builder.outbound_call_config.is_none() {
        builder.outbound_call_config = {
            let builder = crate::types::builders::OutboundCallConfigBuilder::default();
            crate::serde_util::outbound_call_config_correct_errors(builder).build().ok()
        }
    }
    builder
}

pub(crate) fn instance_config_correct_errors(
    mut builder: crate::types::builders::InstanceConfigBuilder,
) -> crate::types::builders::InstanceConfigBuilder {
    if builder.connect_instance_id.is_none() {
        builder.connect_instance_id = Some(Default::default())
    }
    if builder.service_linked_role_arn.is_none() {
        builder.service_linked_role_arn = Some(Default::default())
    }
    if builder.encryption_config.is_none() {
        builder.encryption_config = {
            let builder = crate::types::builders::EncryptionConfigBuilder::default();
            Some(crate::serde_util::encryption_config_correct_errors(builder).build())
        }
    }
    builder
}

pub(crate) fn instance_onboarding_job_status_correct_errors(
    mut builder: crate::types::builders::InstanceOnboardingJobStatusBuilder,
) -> crate::types::builders::InstanceOnboardingJobStatusBuilder {
    if builder.connect_instance_id.is_none() {
        builder.connect_instance_id = Some(Default::default())
    }
    if builder.status.is_none() {
        builder.status = "no value was set".parse::<crate::types::InstanceOnboardingJobStatusCode>().ok()
    }
    builder
}

pub(crate) fn campaign_summary_correct_errors(
    mut builder: crate::types::builders::CampaignSummaryBuilder,
) -> crate::types::builders::CampaignSummaryBuilder {
    if builder.id.is_none() {
        builder.id = Some(Default::default())
    }
    if builder.arn.is_none() {
        builder.arn = Some(Default::default())
    }
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.connect_instance_id.is_none() {
        builder.connect_instance_id = Some(Default::default())
    }
    builder
}

pub(crate) fn encryption_config_correct_errors(
    mut builder: crate::types::builders::EncryptionConfigBuilder,
) -> crate::types::builders::EncryptionConfigBuilder {
    if builder.enabled.is_none() {
        builder.enabled = Some(Default::default())
    }
    builder
}

pub(crate) fn outbound_call_config_correct_errors(
    mut builder: crate::types::builders::OutboundCallConfigBuilder,
) -> crate::types::builders::OutboundCallConfigBuilder {
    if builder.connect_contact_flow_id.is_none() {
        builder.connect_contact_flow_id = Some(Default::default())
    }
    builder
}

pub(crate) fn answer_machine_detection_config_correct_errors(
    mut builder: crate::types::builders::AnswerMachineDetectionConfigBuilder,
) -> crate::types::builders::AnswerMachineDetectionConfigBuilder {
    if builder.enable_answer_machine_detection.is_none() {
        builder.enable_answer_machine_detection = Some(Default::default())
    }
    builder
}

pub(crate) fn predictive_dialer_config_correct_errors(
    mut builder: crate::types::builders::PredictiveDialerConfigBuilder,
) -> crate::types::builders::PredictiveDialerConfigBuilder {
    if builder.bandwidth_allocation.is_none() {
        builder.bandwidth_allocation = Some(Default::default())
    }
    builder
}

pub(crate) fn progressive_dialer_config_correct_errors(
    mut builder: crate::types::builders::ProgressiveDialerConfigBuilder,
) -> crate::types::builders::ProgressiveDialerConfigBuilder {
    if builder.bandwidth_allocation.is_none() {
        builder.bandwidth_allocation = Some(Default::default())
    }
    builder
}
