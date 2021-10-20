// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_configuration_set_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateConfigurationSetInput,
) {
    if let Some(var_1) = &input.configuration_set_name {
        object.key("ConfigurationSetName").string(var_1);
    }
    if let Some(var_2) = &input.delivery_options {
        let mut object_3 = object.key("DeliveryOptions").start_object();
        crate::json_ser::serialize_structure_crate_model_delivery_options(&mut object_3, var_2);
        object_3.finish();
    }
    if let Some(var_4) = &input.reputation_options {
        let mut object_5 = object.key("ReputationOptions").start_object();
        crate::json_ser::serialize_structure_crate_model_reputation_options(&mut object_5, var_4);
        object_5.finish();
    }
    if let Some(var_6) = &input.sending_options {
        let mut object_7 = object.key("SendingOptions").start_object();
        crate::json_ser::serialize_structure_crate_model_sending_options(&mut object_7, var_6);
        object_7.finish();
    }
    if let Some(var_8) = &input.tags {
        let mut array_9 = object.key("Tags").start_array();
        for item_10 in var_8 {
            {
                let mut object_11 = array_9.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_11, item_10);
                object_11.finish();
            }
        }
        array_9.finish();
    }
    if let Some(var_12) = &input.tracking_options {
        let mut object_13 = object.key("TrackingOptions").start_object();
        crate::json_ser::serialize_structure_crate_model_tracking_options(&mut object_13, var_12);
        object_13.finish();
    }
}

pub fn serialize_structure_crate_input_create_configuration_set_event_destination_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateConfigurationSetEventDestinationInput,
) {
    if let Some(var_14) = &input.event_destination {
        let mut object_15 = object.key("EventDestination").start_object();
        crate::json_ser::serialize_structure_crate_model_event_destination_definition(
            &mut object_15,
            var_14,
        );
        object_15.finish();
    }
    if let Some(var_16) = &input.event_destination_name {
        object.key("EventDestinationName").string(var_16);
    }
}

pub fn serialize_structure_crate_input_create_dedicated_ip_pool_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateDedicatedIpPoolInput,
) {
    if let Some(var_17) = &input.pool_name {
        object.key("PoolName").string(var_17);
    }
    if let Some(var_18) = &input.tags {
        let mut array_19 = object.key("Tags").start_array();
        for item_20 in var_18 {
            {
                let mut object_21 = array_19.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_21, item_20);
                object_21.finish();
            }
        }
        array_19.finish();
    }
}

pub fn serialize_structure_crate_input_create_deliverability_test_report_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateDeliverabilityTestReportInput,
) {
    if let Some(var_22) = &input.content {
        let mut object_23 = object.key("Content").start_object();
        crate::json_ser::serialize_structure_crate_model_email_content(&mut object_23, var_22);
        object_23.finish();
    }
    if let Some(var_24) = &input.from_email_address {
        object.key("FromEmailAddress").string(var_24);
    }
    if let Some(var_25) = &input.report_name {
        object.key("ReportName").string(var_25);
    }
    if let Some(var_26) = &input.tags {
        let mut array_27 = object.key("Tags").start_array();
        for item_28 in var_26 {
            {
                let mut object_29 = array_27.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_29, item_28);
                object_29.finish();
            }
        }
        array_27.finish();
    }
}

pub fn serialize_structure_crate_input_create_email_identity_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateEmailIdentityInput,
) {
    if let Some(var_30) = &input.email_identity {
        object.key("EmailIdentity").string(var_30);
    }
    if let Some(var_31) = &input.tags {
        let mut array_32 = object.key("Tags").start_array();
        for item_33 in var_31 {
            {
                let mut object_34 = array_32.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_34, item_33);
                object_34.finish();
            }
        }
        array_32.finish();
    }
}

pub fn serialize_structure_crate_input_put_account_dedicated_ip_warmup_attributes_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutAccountDedicatedIpWarmupAttributesInput,
) {
    if input.auto_warmup_enabled {
        object
            .key("AutoWarmupEnabled")
            .boolean(input.auto_warmup_enabled);
    }
}

pub fn serialize_structure_crate_input_put_account_sending_attributes_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutAccountSendingAttributesInput,
) {
    if input.sending_enabled {
        object.key("SendingEnabled").boolean(input.sending_enabled);
    }
}

pub fn serialize_structure_crate_input_put_configuration_set_delivery_options_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutConfigurationSetDeliveryOptionsInput,
) {
    if let Some(var_35) = &input.sending_pool_name {
        object.key("SendingPoolName").string(var_35);
    }
    if let Some(var_36) = &input.tls_policy {
        object.key("TlsPolicy").string(var_36.as_str());
    }
}

pub fn serialize_structure_crate_input_put_configuration_set_reputation_options_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutConfigurationSetReputationOptionsInput,
) {
    if input.reputation_metrics_enabled {
        object
            .key("ReputationMetricsEnabled")
            .boolean(input.reputation_metrics_enabled);
    }
}

pub fn serialize_structure_crate_input_put_configuration_set_sending_options_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutConfigurationSetSendingOptionsInput,
) {
    if input.sending_enabled {
        object.key("SendingEnabled").boolean(input.sending_enabled);
    }
}

pub fn serialize_structure_crate_input_put_configuration_set_tracking_options_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutConfigurationSetTrackingOptionsInput,
) {
    if let Some(var_37) = &input.custom_redirect_domain {
        object.key("CustomRedirectDomain").string(var_37);
    }
}

pub fn serialize_structure_crate_input_put_dedicated_ip_in_pool_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutDedicatedIpInPoolInput,
) {
    if let Some(var_38) = &input.destination_pool_name {
        object.key("DestinationPoolName").string(var_38);
    }
}

pub fn serialize_structure_crate_input_put_dedicated_ip_warmup_attributes_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutDedicatedIpWarmupAttributesInput,
) {
    if let Some(var_39) = &input.warmup_percentage {
        object.key("WarmupPercentage").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_39).into()),
        );
    }
}

pub fn serialize_structure_crate_input_put_deliverability_dashboard_option_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutDeliverabilityDashboardOptionInput,
) {
    {
        object
            .key("DashboardEnabled")
            .boolean(input.dashboard_enabled);
    }
    if let Some(var_40) = &input.subscribed_domains {
        let mut array_41 = object.key("SubscribedDomains").start_array();
        for item_42 in var_40 {
            {
                let mut object_43 = array_41.value().start_object();
                crate::json_ser::serialize_structure_crate_model_domain_deliverability_tracking_option(&mut object_43, item_42);
                object_43.finish();
            }
        }
        array_41.finish();
    }
}

pub fn serialize_structure_crate_input_put_email_identity_dkim_attributes_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutEmailIdentityDkimAttributesInput,
) {
    if input.signing_enabled {
        object.key("SigningEnabled").boolean(input.signing_enabled);
    }
}

pub fn serialize_structure_crate_input_put_email_identity_feedback_attributes_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutEmailIdentityFeedbackAttributesInput,
) {
    if input.email_forwarding_enabled {
        object
            .key("EmailForwardingEnabled")
            .boolean(input.email_forwarding_enabled);
    }
}

pub fn serialize_structure_crate_input_put_email_identity_mail_from_attributes_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutEmailIdentityMailFromAttributesInput,
) {
    if let Some(var_44) = &input.behavior_on_mx_failure {
        object.key("BehaviorOnMxFailure").string(var_44.as_str());
    }
    if let Some(var_45) = &input.mail_from_domain {
        object.key("MailFromDomain").string(var_45);
    }
}

pub fn serialize_structure_crate_input_send_email_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::SendEmailInput,
) {
    if let Some(var_46) = &input.configuration_set_name {
        object.key("ConfigurationSetName").string(var_46);
    }
    if let Some(var_47) = &input.content {
        let mut object_48 = object.key("Content").start_object();
        crate::json_ser::serialize_structure_crate_model_email_content(&mut object_48, var_47);
        object_48.finish();
    }
    if let Some(var_49) = &input.destination {
        let mut object_50 = object.key("Destination").start_object();
        crate::json_ser::serialize_structure_crate_model_destination(&mut object_50, var_49);
        object_50.finish();
    }
    if let Some(var_51) = &input.email_tags {
        let mut array_52 = object.key("EmailTags").start_array();
        for item_53 in var_51 {
            {
                let mut object_54 = array_52.value().start_object();
                crate::json_ser::serialize_structure_crate_model_message_tag(
                    &mut object_54,
                    item_53,
                );
                object_54.finish();
            }
        }
        array_52.finish();
    }
    if let Some(var_55) = &input.feedback_forwarding_email_address {
        object.key("FeedbackForwardingEmailAddress").string(var_55);
    }
    if let Some(var_56) = &input.from_email_address {
        object.key("FromEmailAddress").string(var_56);
    }
    if let Some(var_57) = &input.reply_to_addresses {
        let mut array_58 = object.key("ReplyToAddresses").start_array();
        for item_59 in var_57 {
            {
                array_58.value().string(item_59);
            }
        }
        array_58.finish();
    }
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) {
    if let Some(var_60) = &input.resource_arn {
        object.key("ResourceArn").string(var_60);
    }
    if let Some(var_61) = &input.tags {
        let mut array_62 = object.key("Tags").start_array();
        for item_63 in var_61 {
            {
                let mut object_64 = array_62.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_64, item_63);
                object_64.finish();
            }
        }
        array_62.finish();
    }
}

pub fn serialize_structure_crate_input_update_configuration_set_event_destination_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateConfigurationSetEventDestinationInput,
) {
    if let Some(var_65) = &input.event_destination {
        let mut object_66 = object.key("EventDestination").start_object();
        crate::json_ser::serialize_structure_crate_model_event_destination_definition(
            &mut object_66,
            var_65,
        );
        object_66.finish();
    }
}

pub fn serialize_structure_crate_model_delivery_options(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DeliveryOptions,
) {
    if let Some(var_67) = &input.tls_policy {
        object.key("TlsPolicy").string(var_67.as_str());
    }
    if let Some(var_68) = &input.sending_pool_name {
        object.key("SendingPoolName").string(var_68);
    }
}

pub fn serialize_structure_crate_model_reputation_options(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ReputationOptions,
) {
    if input.reputation_metrics_enabled {
        object
            .key("ReputationMetricsEnabled")
            .boolean(input.reputation_metrics_enabled);
    }
    if let Some(var_69) = &input.last_fresh_start {
        object
            .key("LastFreshStart")
            .instant(var_69, aws_smithy_types::instant::Format::EpochSeconds);
    }
}

pub fn serialize_structure_crate_model_sending_options(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SendingOptions,
) {
    if input.sending_enabled {
        object.key("SendingEnabled").boolean(input.sending_enabled);
    }
}

pub fn serialize_structure_crate_model_tag(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) {
    if let Some(var_70) = &input.key {
        object.key("Key").string(var_70);
    }
    if let Some(var_71) = &input.value {
        object.key("Value").string(var_71);
    }
}

pub fn serialize_structure_crate_model_tracking_options(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::TrackingOptions,
) {
    if let Some(var_72) = &input.custom_redirect_domain {
        object.key("CustomRedirectDomain").string(var_72);
    }
}

pub fn serialize_structure_crate_model_event_destination_definition(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EventDestinationDefinition,
) {
    if input.enabled {
        object.key("Enabled").boolean(input.enabled);
    }
    if let Some(var_73) = &input.matching_event_types {
        let mut array_74 = object.key("MatchingEventTypes").start_array();
        for item_75 in var_73 {
            {
                array_74.value().string(item_75.as_str());
            }
        }
        array_74.finish();
    }
    if let Some(var_76) = &input.kinesis_firehose_destination {
        let mut object_77 = object.key("KinesisFirehoseDestination").start_object();
        crate::json_ser::serialize_structure_crate_model_kinesis_firehose_destination(
            &mut object_77,
            var_76,
        );
        object_77.finish();
    }
    if let Some(var_78) = &input.cloud_watch_destination {
        let mut object_79 = object.key("CloudWatchDestination").start_object();
        crate::json_ser::serialize_structure_crate_model_cloud_watch_destination(
            &mut object_79,
            var_78,
        );
        object_79.finish();
    }
    if let Some(var_80) = &input.sns_destination {
        let mut object_81 = object.key("SnsDestination").start_object();
        crate::json_ser::serialize_structure_crate_model_sns_destination(&mut object_81, var_80);
        object_81.finish();
    }
    if let Some(var_82) = &input.pinpoint_destination {
        let mut object_83 = object.key("PinpointDestination").start_object();
        crate::json_ser::serialize_structure_crate_model_pinpoint_destination(
            &mut object_83,
            var_82,
        );
        object_83.finish();
    }
}

pub fn serialize_structure_crate_model_email_content(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EmailContent,
) {
    if let Some(var_84) = &input.simple {
        let mut object_85 = object.key("Simple").start_object();
        crate::json_ser::serialize_structure_crate_model_message(&mut object_85, var_84);
        object_85.finish();
    }
    if let Some(var_86) = &input.raw {
        let mut object_87 = object.key("Raw").start_object();
        crate::json_ser::serialize_structure_crate_model_raw_message(&mut object_87, var_86);
        object_87.finish();
    }
    if let Some(var_88) = &input.template {
        let mut object_89 = object.key("Template").start_object();
        crate::json_ser::serialize_structure_crate_model_template(&mut object_89, var_88);
        object_89.finish();
    }
}

pub fn serialize_structure_crate_model_domain_deliverability_tracking_option(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DomainDeliverabilityTrackingOption,
) {
    if let Some(var_90) = &input.domain {
        object.key("Domain").string(var_90);
    }
    if let Some(var_91) = &input.subscription_start_date {
        object
            .key("SubscriptionStartDate")
            .instant(var_91, aws_smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_92) = &input.inbox_placement_tracking_option {
        let mut object_93 = object.key("InboxPlacementTrackingOption").start_object();
        crate::json_ser::serialize_structure_crate_model_inbox_placement_tracking_option(
            &mut object_93,
            var_92,
        );
        object_93.finish();
    }
}

pub fn serialize_structure_crate_model_destination(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Destination,
) {
    if let Some(var_94) = &input.to_addresses {
        let mut array_95 = object.key("ToAddresses").start_array();
        for item_96 in var_94 {
            {
                array_95.value().string(item_96);
            }
        }
        array_95.finish();
    }
    if let Some(var_97) = &input.cc_addresses {
        let mut array_98 = object.key("CcAddresses").start_array();
        for item_99 in var_97 {
            {
                array_98.value().string(item_99);
            }
        }
        array_98.finish();
    }
    if let Some(var_100) = &input.bcc_addresses {
        let mut array_101 = object.key("BccAddresses").start_array();
        for item_102 in var_100 {
            {
                array_101.value().string(item_102);
            }
        }
        array_101.finish();
    }
}

pub fn serialize_structure_crate_model_message_tag(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::MessageTag,
) {
    if let Some(var_103) = &input.name {
        object.key("Name").string(var_103);
    }
    if let Some(var_104) = &input.value {
        object.key("Value").string(var_104);
    }
}

pub fn serialize_structure_crate_model_kinesis_firehose_destination(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::KinesisFirehoseDestination,
) {
    if let Some(var_105) = &input.iam_role_arn {
        object.key("IamRoleArn").string(var_105);
    }
    if let Some(var_106) = &input.delivery_stream_arn {
        object.key("DeliveryStreamArn").string(var_106);
    }
}

pub fn serialize_structure_crate_model_cloud_watch_destination(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CloudWatchDestination,
) {
    if let Some(var_107) = &input.dimension_configurations {
        let mut array_108 = object.key("DimensionConfigurations").start_array();
        for item_109 in var_107 {
            {
                let mut object_110 = array_108.value().start_object();
                crate::json_ser::serialize_structure_crate_model_cloud_watch_dimension_configuration(&mut object_110, item_109);
                object_110.finish();
            }
        }
        array_108.finish();
    }
}

pub fn serialize_structure_crate_model_sns_destination(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SnsDestination,
) {
    if let Some(var_111) = &input.topic_arn {
        object.key("TopicArn").string(var_111);
    }
}

pub fn serialize_structure_crate_model_pinpoint_destination(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PinpointDestination,
) {
    if let Some(var_112) = &input.application_arn {
        object.key("ApplicationArn").string(var_112);
    }
}

pub fn serialize_structure_crate_model_message(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Message,
) {
    if let Some(var_113) = &input.subject {
        let mut object_114 = object.key("Subject").start_object();
        crate::json_ser::serialize_structure_crate_model_content(&mut object_114, var_113);
        object_114.finish();
    }
    if let Some(var_115) = &input.body {
        let mut object_116 = object.key("Body").start_object();
        crate::json_ser::serialize_structure_crate_model_body(&mut object_116, var_115);
        object_116.finish();
    }
}

pub fn serialize_structure_crate_model_raw_message(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RawMessage,
) {
    if let Some(var_117) = &input.data {
        object
            .key("Data")
            .string_unchecked(&aws_smithy_types::base64::encode(var_117));
    }
}

pub fn serialize_structure_crate_model_template(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Template,
) {
    if let Some(var_118) = &input.template_arn {
        object.key("TemplateArn").string(var_118);
    }
    if let Some(var_119) = &input.template_data {
        object.key("TemplateData").string(var_119);
    }
}

pub fn serialize_structure_crate_model_inbox_placement_tracking_option(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::InboxPlacementTrackingOption,
) {
    if input.global {
        object.key("Global").boolean(input.global);
    }
    if let Some(var_120) = &input.tracked_isps {
        let mut array_121 = object.key("TrackedIsps").start_array();
        for item_122 in var_120 {
            {
                array_121.value().string(item_122);
            }
        }
        array_121.finish();
    }
}

pub fn serialize_structure_crate_model_cloud_watch_dimension_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CloudWatchDimensionConfiguration,
) {
    if let Some(var_123) = &input.dimension_name {
        object.key("DimensionName").string(var_123);
    }
    if let Some(var_124) = &input.dimension_value_source {
        object.key("DimensionValueSource").string(var_124.as_str());
    }
    if let Some(var_125) = &input.default_dimension_value {
        object.key("DefaultDimensionValue").string(var_125);
    }
}

pub fn serialize_structure_crate_model_content(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Content,
) {
    if let Some(var_126) = &input.data {
        object.key("Data").string(var_126);
    }
    if let Some(var_127) = &input.charset {
        object.key("Charset").string(var_127);
    }
}

pub fn serialize_structure_crate_model_body(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Body,
) {
    if let Some(var_128) = &input.text {
        let mut object_129 = object.key("Text").start_object();
        crate::json_ser::serialize_structure_crate_model_content(&mut object_129, var_128);
        object_129.finish();
    }
    if let Some(var_130) = &input.html {
        let mut object_131 = object.key("Html").start_object();
        crate::json_ser::serialize_structure_crate_model_content(&mut object_131, var_130);
        object_131.finish();
    }
}
