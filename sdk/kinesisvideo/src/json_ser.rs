// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_signaling_channel_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateSignalingChannelInput,
) {
    if let Some(var_1) = &input.channel_name {
        object.key("ChannelName").string(var_1);
    }
    if let Some(var_2) = &input.channel_type {
        object.key("ChannelType").string(var_2.as_str());
    }
    if let Some(var_3) = &input.single_master_configuration {
        let mut object_4 = object.key("SingleMasterConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_single_master_configuration(
            &mut object_4,
            var_3,
        );
        object_4.finish();
    }
    if let Some(var_5) = &input.tags {
        let mut array_6 = object.key("Tags").start_array();
        for item_7 in var_5 {
            {
                let mut object_8 = array_6.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_8, item_7);
                object_8.finish();
            }
        }
        array_6.finish();
    }
}

pub fn serialize_structure_crate_input_create_stream_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateStreamInput,
) {
    if let Some(var_9) = &input.data_retention_in_hours {
        object.key("DataRetentionInHours").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_9).into()),
        );
    }
    if let Some(var_10) = &input.device_name {
        object.key("DeviceName").string(var_10);
    }
    if let Some(var_11) = &input.kms_key_id {
        object.key("KmsKeyId").string(var_11);
    }
    if let Some(var_12) = &input.media_type {
        object.key("MediaType").string(var_12);
    }
    if let Some(var_13) = &input.stream_name {
        object.key("StreamName").string(var_13);
    }
    if let Some(var_14) = &input.tags {
        let mut object_15 = object.key("Tags").start_object();
        for (key_16, value_17) in var_14 {
            {
                object_15.key(key_16).string(value_17);
            }
        }
        object_15.finish();
    }
}

pub fn serialize_structure_crate_input_delete_signaling_channel_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteSignalingChannelInput,
) {
    if let Some(var_18) = &input.channel_arn {
        object.key("ChannelARN").string(var_18);
    }
    if let Some(var_19) = &input.current_version {
        object.key("CurrentVersion").string(var_19);
    }
}

pub fn serialize_structure_crate_input_delete_stream_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteStreamInput,
) {
    if let Some(var_20) = &input.current_version {
        object.key("CurrentVersion").string(var_20);
    }
    if let Some(var_21) = &input.stream_arn {
        object.key("StreamARN").string(var_21);
    }
}

pub fn serialize_structure_crate_input_describe_signaling_channel_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeSignalingChannelInput,
) {
    if let Some(var_22) = &input.channel_arn {
        object.key("ChannelARN").string(var_22);
    }
    if let Some(var_23) = &input.channel_name {
        object.key("ChannelName").string(var_23);
    }
}

pub fn serialize_structure_crate_input_describe_stream_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeStreamInput,
) {
    if let Some(var_24) = &input.stream_arn {
        object.key("StreamARN").string(var_24);
    }
    if let Some(var_25) = &input.stream_name {
        object.key("StreamName").string(var_25);
    }
}

pub fn serialize_structure_crate_input_get_data_endpoint_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetDataEndpointInput,
) {
    if let Some(var_26) = &input.api_name {
        object.key("APIName").string(var_26.as_str());
    }
    if let Some(var_27) = &input.stream_arn {
        object.key("StreamARN").string(var_27);
    }
    if let Some(var_28) = &input.stream_name {
        object.key("StreamName").string(var_28);
    }
}

pub fn serialize_structure_crate_input_get_signaling_channel_endpoint_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetSignalingChannelEndpointInput,
) {
    if let Some(var_29) = &input.channel_arn {
        object.key("ChannelARN").string(var_29);
    }
    if let Some(var_30) = &input.single_master_channel_endpoint_configuration {
        let mut object_31 = object
            .key("SingleMasterChannelEndpointConfiguration")
            .start_object();
        crate::json_ser::serialize_structure_crate_model_single_master_channel_endpoint_configuration(&mut object_31, var_30);
        object_31.finish();
    }
}

pub fn serialize_structure_crate_input_list_signaling_channels_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListSignalingChannelsInput,
) {
    if let Some(var_32) = &input.channel_name_condition {
        let mut object_33 = object.key("ChannelNameCondition").start_object();
        crate::json_ser::serialize_structure_crate_model_channel_name_condition(
            &mut object_33,
            var_32,
        );
        object_33.finish();
    }
    if let Some(var_34) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_34).into()),
        );
    }
    if let Some(var_35) = &input.next_token {
        object.key("NextToken").string(var_35);
    }
}

pub fn serialize_structure_crate_input_list_streams_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListStreamsInput,
) {
    if let Some(var_36) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_36).into()),
        );
    }
    if let Some(var_37) = &input.next_token {
        object.key("NextToken").string(var_37);
    }
    if let Some(var_38) = &input.stream_name_condition {
        let mut object_39 = object.key("StreamNameCondition").start_object();
        crate::json_ser::serialize_structure_crate_model_stream_name_condition(
            &mut object_39,
            var_38,
        );
        object_39.finish();
    }
}

pub fn serialize_structure_crate_input_list_tags_for_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTagsForResourceInput,
) {
    if let Some(var_40) = &input.next_token {
        object.key("NextToken").string(var_40);
    }
    if let Some(var_41) = &input.resource_arn {
        object.key("ResourceARN").string(var_41);
    }
}

pub fn serialize_structure_crate_input_list_tags_for_stream_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTagsForStreamInput,
) {
    if let Some(var_42) = &input.next_token {
        object.key("NextToken").string(var_42);
    }
    if let Some(var_43) = &input.stream_arn {
        object.key("StreamARN").string(var_43);
    }
    if let Some(var_44) = &input.stream_name {
        object.key("StreamName").string(var_44);
    }
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) {
    if let Some(var_45) = &input.resource_arn {
        object.key("ResourceARN").string(var_45);
    }
    if let Some(var_46) = &input.tags {
        let mut array_47 = object.key("Tags").start_array();
        for item_48 in var_46 {
            {
                let mut object_49 = array_47.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_49, item_48);
                object_49.finish();
            }
        }
        array_47.finish();
    }
}

pub fn serialize_structure_crate_input_tag_stream_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagStreamInput,
) {
    if let Some(var_50) = &input.stream_arn {
        object.key("StreamARN").string(var_50);
    }
    if let Some(var_51) = &input.stream_name {
        object.key("StreamName").string(var_51);
    }
    if let Some(var_52) = &input.tags {
        let mut object_53 = object.key("Tags").start_object();
        for (key_54, value_55) in var_52 {
            {
                object_53.key(key_54).string(value_55);
            }
        }
        object_53.finish();
    }
}

pub fn serialize_structure_crate_input_untag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UntagResourceInput,
) {
    if let Some(var_56) = &input.resource_arn {
        object.key("ResourceARN").string(var_56);
    }
    if let Some(var_57) = &input.tag_key_list {
        let mut array_58 = object.key("TagKeyList").start_array();
        for item_59 in var_57 {
            {
                array_58.value().string(item_59);
            }
        }
        array_58.finish();
    }
}

pub fn serialize_structure_crate_input_untag_stream_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UntagStreamInput,
) {
    if let Some(var_60) = &input.stream_arn {
        object.key("StreamARN").string(var_60);
    }
    if let Some(var_61) = &input.stream_name {
        object.key("StreamName").string(var_61);
    }
    if let Some(var_62) = &input.tag_key_list {
        let mut array_63 = object.key("TagKeyList").start_array();
        for item_64 in var_62 {
            {
                array_63.value().string(item_64);
            }
        }
        array_63.finish();
    }
}

pub fn serialize_structure_crate_input_update_data_retention_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateDataRetentionInput,
) {
    if let Some(var_65) = &input.current_version {
        object.key("CurrentVersion").string(var_65);
    }
    if let Some(var_66) = &input.data_retention_change_in_hours {
        object.key("DataRetentionChangeInHours").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_66).into()),
        );
    }
    if let Some(var_67) = &input.operation {
        object.key("Operation").string(var_67.as_str());
    }
    if let Some(var_68) = &input.stream_arn {
        object.key("StreamARN").string(var_68);
    }
    if let Some(var_69) = &input.stream_name {
        object.key("StreamName").string(var_69);
    }
}

pub fn serialize_structure_crate_input_update_signaling_channel_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateSignalingChannelInput,
) {
    if let Some(var_70) = &input.channel_arn {
        object.key("ChannelARN").string(var_70);
    }
    if let Some(var_71) = &input.current_version {
        object.key("CurrentVersion").string(var_71);
    }
    if let Some(var_72) = &input.single_master_configuration {
        let mut object_73 = object.key("SingleMasterConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_single_master_configuration(
            &mut object_73,
            var_72,
        );
        object_73.finish();
    }
}

pub fn serialize_structure_crate_input_update_stream_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateStreamInput,
) {
    if let Some(var_74) = &input.current_version {
        object.key("CurrentVersion").string(var_74);
    }
    if let Some(var_75) = &input.device_name {
        object.key("DeviceName").string(var_75);
    }
    if let Some(var_76) = &input.media_type {
        object.key("MediaType").string(var_76);
    }
    if let Some(var_77) = &input.stream_arn {
        object.key("StreamARN").string(var_77);
    }
    if let Some(var_78) = &input.stream_name {
        object.key("StreamName").string(var_78);
    }
}

pub fn serialize_structure_crate_model_single_master_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SingleMasterConfiguration,
) {
    if let Some(var_79) = &input.message_ttl_seconds {
        object.key("MessageTtlSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_79).into()),
        );
    }
}

pub fn serialize_structure_crate_model_tag(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) {
    if let Some(var_80) = &input.key {
        object.key("Key").string(var_80);
    }
    if let Some(var_81) = &input.value {
        object.key("Value").string(var_81);
    }
}

pub fn serialize_structure_crate_model_single_master_channel_endpoint_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SingleMasterChannelEndpointConfiguration,
) {
    if let Some(var_82) = &input.protocols {
        let mut array_83 = object.key("Protocols").start_array();
        for item_84 in var_82 {
            {
                array_83.value().string(item_84.as_str());
            }
        }
        array_83.finish();
    }
    if let Some(var_85) = &input.role {
        object.key("Role").string(var_85.as_str());
    }
}

pub fn serialize_structure_crate_model_channel_name_condition(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ChannelNameCondition,
) {
    if let Some(var_86) = &input.comparison_operator {
        object.key("ComparisonOperator").string(var_86.as_str());
    }
    if let Some(var_87) = &input.comparison_value {
        object.key("ComparisonValue").string(var_87);
    }
}

pub fn serialize_structure_crate_model_stream_name_condition(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::StreamNameCondition,
) {
    if let Some(var_88) = &input.comparison_operator {
        object.key("ComparisonOperator").string(var_88.as_str());
    }
    if let Some(var_89) = &input.comparison_value {
        object.key("ComparisonValue").string(var_89);
    }
}
