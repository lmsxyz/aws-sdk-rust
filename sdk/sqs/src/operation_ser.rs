// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_operation_crate_operation_add_permission(
    input: &crate::input::AddPermissionInput,
) -> Result<aws_smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "AddPermission", "2012-11-05");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("QueueUrl");
    if let Some(var_2) = &input.queue_url {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Label");
    if let Some(var_4) = &input.label {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("AWSAccountId");
    if let Some(var_6) = &input.aws_account_ids {
        let mut list_8 = scope_5.start_list(true, None);
        for item_7 in var_6 {
            #[allow(unused_mut)]
            let mut entry_9 = list_8.entry();
            entry_9.string(item_7);
        }
        list_8.finish();
    }
    #[allow(unused_mut)]
    let mut scope_10 = writer.prefix("ActionName");
    if let Some(var_11) = &input.actions {
        let mut list_13 = scope_10.start_list(true, None);
        for item_12 in var_11 {
            #[allow(unused_mut)]
            let mut entry_14 = list_13.entry();
            entry_14.string(item_12);
        }
        list_13.finish();
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_change_message_visibility(
    input: &crate::input::ChangeMessageVisibilityInput,
) -> Result<aws_smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        aws_smithy_query::QueryWriter::new(&mut out, "ChangeMessageVisibility", "2012-11-05");
    #[allow(unused_mut)]
    let mut scope_15 = writer.prefix("QueueUrl");
    if let Some(var_16) = &input.queue_url {
        scope_15.string(var_16);
    }
    #[allow(unused_mut)]
    let mut scope_17 = writer.prefix("ReceiptHandle");
    if let Some(var_18) = &input.receipt_handle {
        scope_17.string(var_18);
    }
    #[allow(unused_mut)]
    let mut scope_19 = writer.prefix("VisibilityTimeout");
    {
        scope_19.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.visibility_timeout).into()),
        );
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_change_message_visibility_batch(
    input: &crate::input::ChangeMessageVisibilityBatchInput,
) -> Result<aws_smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        aws_smithy_query::QueryWriter::new(&mut out, "ChangeMessageVisibilityBatch", "2012-11-05");
    #[allow(unused_mut)]
    let mut scope_20 = writer.prefix("QueueUrl");
    if let Some(var_21) = &input.queue_url {
        scope_20.string(var_21);
    }
    #[allow(unused_mut)]
    let mut scope_22 = writer.prefix("ChangeMessageVisibilityBatchRequestEntry");
    if let Some(var_23) = &input.entries {
        let mut list_25 = scope_22.start_list(true, None);
        for item_24 in var_23 {
            #[allow(unused_mut)]
            let mut entry_26 = list_25.entry();
            crate::query_ser::serialize_structure_crate_model_change_message_visibility_batch_request_entry(entry_26, item_24);
        }
        list_25.finish();
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_create_queue(
    input: &crate::input::CreateQueueInput,
) -> Result<aws_smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "CreateQueue", "2012-11-05");
    #[allow(unused_mut)]
    let mut scope_27 = writer.prefix("QueueName");
    if let Some(var_28) = &input.queue_name {
        scope_27.string(var_28);
    }
    #[allow(unused_mut)]
    let mut scope_29 = writer.prefix("Tag");
    if let Some(var_30) = &input.tags {
        let mut map_31 = scope_29.start_map(true, "Key", "Value");
        for (key_32, value_33) in var_30 {
            #[allow(unused_mut)]
            let mut entry_34 = map_31.entry(key_32);
            {
                entry_34.string(value_33);
            }
        }
        map_31.finish();
    }
    #[allow(unused_mut)]
    let mut scope_35 = writer.prefix("Attribute");
    if let Some(var_36) = &input.attributes {
        let mut map_37 = scope_35.start_map(true, "Name", "Value");
        for (key_38, value_39) in var_36 {
            #[allow(unused_mut)]
            let mut entry_40 = map_37.entry(key_38.as_str());
            {
                entry_40.string(value_39);
            }
        }
        map_37.finish();
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_delete_message(
    input: &crate::input::DeleteMessageInput,
) -> Result<aws_smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "DeleteMessage", "2012-11-05");
    #[allow(unused_mut)]
    let mut scope_41 = writer.prefix("QueueUrl");
    if let Some(var_42) = &input.queue_url {
        scope_41.string(var_42);
    }
    #[allow(unused_mut)]
    let mut scope_43 = writer.prefix("ReceiptHandle");
    if let Some(var_44) = &input.receipt_handle {
        scope_43.string(var_44);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_delete_message_batch(
    input: &crate::input::DeleteMessageBatchInput,
) -> Result<aws_smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        aws_smithy_query::QueryWriter::new(&mut out, "DeleteMessageBatch", "2012-11-05");
    #[allow(unused_mut)]
    let mut scope_45 = writer.prefix("QueueUrl");
    if let Some(var_46) = &input.queue_url {
        scope_45.string(var_46);
    }
    #[allow(unused_mut)]
    let mut scope_47 = writer.prefix("DeleteMessageBatchRequestEntry");
    if let Some(var_48) = &input.entries {
        let mut list_50 = scope_47.start_list(true, None);
        for item_49 in var_48 {
            #[allow(unused_mut)]
            let mut entry_51 = list_50.entry();
            crate::query_ser::serialize_structure_crate_model_delete_message_batch_request_entry(
                entry_51, item_49,
            );
        }
        list_50.finish();
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_delete_queue(
    input: &crate::input::DeleteQueueInput,
) -> Result<aws_smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "DeleteQueue", "2012-11-05");
    #[allow(unused_mut)]
    let mut scope_52 = writer.prefix("QueueUrl");
    if let Some(var_53) = &input.queue_url {
        scope_52.string(var_53);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_get_queue_attributes(
    input: &crate::input::GetQueueAttributesInput,
) -> Result<aws_smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        aws_smithy_query::QueryWriter::new(&mut out, "GetQueueAttributes", "2012-11-05");
    #[allow(unused_mut)]
    let mut scope_54 = writer.prefix("QueueUrl");
    if let Some(var_55) = &input.queue_url {
        scope_54.string(var_55);
    }
    #[allow(unused_mut)]
    let mut scope_56 = writer.prefix("AttributeName");
    if let Some(var_57) = &input.attribute_names {
        let mut list_59 = scope_56.start_list(true, None);
        for item_58 in var_57 {
            #[allow(unused_mut)]
            let mut entry_60 = list_59.entry();
            entry_60.string(item_58.as_str());
        }
        list_59.finish();
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_get_queue_url(
    input: &crate::input::GetQueueUrlInput,
) -> Result<aws_smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "GetQueueUrl", "2012-11-05");
    #[allow(unused_mut)]
    let mut scope_61 = writer.prefix("QueueName");
    if let Some(var_62) = &input.queue_name {
        scope_61.string(var_62);
    }
    #[allow(unused_mut)]
    let mut scope_63 = writer.prefix("QueueOwnerAWSAccountId");
    if let Some(var_64) = &input.queue_owner_aws_account_id {
        scope_63.string(var_64);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_list_dead_letter_source_queues(
    input: &crate::input::ListDeadLetterSourceQueuesInput,
) -> Result<aws_smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        aws_smithy_query::QueryWriter::new(&mut out, "ListDeadLetterSourceQueues", "2012-11-05");
    #[allow(unused_mut)]
    let mut scope_65 = writer.prefix("QueueUrl");
    if let Some(var_66) = &input.queue_url {
        scope_65.string(var_66);
    }
    #[allow(unused_mut)]
    let mut scope_67 = writer.prefix("NextToken");
    if let Some(var_68) = &input.next_token {
        scope_67.string(var_68);
    }
    #[allow(unused_mut)]
    let mut scope_69 = writer.prefix("MaxResults");
    if let Some(var_70) = &input.max_results {
        scope_69.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_70).into()),
        );
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_list_queues(
    input: &crate::input::ListQueuesInput,
) -> Result<aws_smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "ListQueues", "2012-11-05");
    #[allow(unused_mut)]
    let mut scope_71 = writer.prefix("QueueNamePrefix");
    if let Some(var_72) = &input.queue_name_prefix {
        scope_71.string(var_72);
    }
    #[allow(unused_mut)]
    let mut scope_73 = writer.prefix("NextToken");
    if let Some(var_74) = &input.next_token {
        scope_73.string(var_74);
    }
    #[allow(unused_mut)]
    let mut scope_75 = writer.prefix("MaxResults");
    if let Some(var_76) = &input.max_results {
        scope_75.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_76).into()),
        );
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_list_queue_tags(
    input: &crate::input::ListQueueTagsInput,
) -> Result<aws_smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "ListQueueTags", "2012-11-05");
    #[allow(unused_mut)]
    let mut scope_77 = writer.prefix("QueueUrl");
    if let Some(var_78) = &input.queue_url {
        scope_77.string(var_78);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_purge_queue(
    input: &crate::input::PurgeQueueInput,
) -> Result<aws_smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "PurgeQueue", "2012-11-05");
    #[allow(unused_mut)]
    let mut scope_79 = writer.prefix("QueueUrl");
    if let Some(var_80) = &input.queue_url {
        scope_79.string(var_80);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_receive_message(
    input: &crate::input::ReceiveMessageInput,
) -> Result<aws_smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "ReceiveMessage", "2012-11-05");
    #[allow(unused_mut)]
    let mut scope_81 = writer.prefix("QueueUrl");
    if let Some(var_82) = &input.queue_url {
        scope_81.string(var_82);
    }
    #[allow(unused_mut)]
    let mut scope_83 = writer.prefix("AttributeName");
    if let Some(var_84) = &input.attribute_names {
        let mut list_86 = scope_83.start_list(true, None);
        for item_85 in var_84 {
            #[allow(unused_mut)]
            let mut entry_87 = list_86.entry();
            entry_87.string(item_85.as_str());
        }
        list_86.finish();
    }
    #[allow(unused_mut)]
    let mut scope_88 = writer.prefix("MessageAttributeName");
    if let Some(var_89) = &input.message_attribute_names {
        let mut list_91 = scope_88.start_list(true, None);
        for item_90 in var_89 {
            #[allow(unused_mut)]
            let mut entry_92 = list_91.entry();
            entry_92.string(item_90);
        }
        list_91.finish();
    }
    #[allow(unused_mut)]
    let mut scope_93 = writer.prefix("MaxNumberOfMessages");
    if input.max_number_of_messages != 0 {
        scope_93.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_number_of_messages).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_94 = writer.prefix("VisibilityTimeout");
    if input.visibility_timeout != 0 {
        scope_94.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.visibility_timeout).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_95 = writer.prefix("WaitTimeSeconds");
    if input.wait_time_seconds != 0 {
        scope_95.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.wait_time_seconds).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_96 = writer.prefix("ReceiveRequestAttemptId");
    if let Some(var_97) = &input.receive_request_attempt_id {
        scope_96.string(var_97);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_remove_permission(
    input: &crate::input::RemovePermissionInput,
) -> Result<aws_smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "RemovePermission", "2012-11-05");
    #[allow(unused_mut)]
    let mut scope_98 = writer.prefix("QueueUrl");
    if let Some(var_99) = &input.queue_url {
        scope_98.string(var_99);
    }
    #[allow(unused_mut)]
    let mut scope_100 = writer.prefix("Label");
    if let Some(var_101) = &input.label {
        scope_100.string(var_101);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_send_message(
    input: &crate::input::SendMessageInput,
) -> Result<aws_smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "SendMessage", "2012-11-05");
    #[allow(unused_mut)]
    let mut scope_102 = writer.prefix("QueueUrl");
    if let Some(var_103) = &input.queue_url {
        scope_102.string(var_103);
    }
    #[allow(unused_mut)]
    let mut scope_104 = writer.prefix("MessageBody");
    if let Some(var_105) = &input.message_body {
        scope_104.string(var_105);
    }
    #[allow(unused_mut)]
    let mut scope_106 = writer.prefix("DelaySeconds");
    if input.delay_seconds != 0 {
        scope_106.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.delay_seconds).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_107 = writer.prefix("MessageAttribute");
    if let Some(var_108) = &input.message_attributes {
        let mut map_109 = scope_107.start_map(true, "Name", "Value");
        for (key_110, value_111) in var_108 {
            #[allow(unused_mut)]
            let mut entry_112 = map_109.entry(key_110);
            {
                crate::query_ser::serialize_structure_crate_model_message_attribute_value(
                    entry_112, value_111,
                );
            }
        }
        map_109.finish();
    }
    #[allow(unused_mut)]
    let mut scope_113 = writer.prefix("MessageSystemAttribute");
    if let Some(var_114) = &input.message_system_attributes {
        let mut map_115 = scope_113.start_map(true, "Name", "Value");
        for (key_116, value_117) in var_114 {
            #[allow(unused_mut)]
            let mut entry_118 = map_115.entry(key_116.as_str());
            {
                crate::query_ser::serialize_structure_crate_model_message_system_attribute_value(
                    entry_118, value_117,
                );
            }
        }
        map_115.finish();
    }
    #[allow(unused_mut)]
    let mut scope_119 = writer.prefix("MessageDeduplicationId");
    if let Some(var_120) = &input.message_deduplication_id {
        scope_119.string(var_120);
    }
    #[allow(unused_mut)]
    let mut scope_121 = writer.prefix("MessageGroupId");
    if let Some(var_122) = &input.message_group_id {
        scope_121.string(var_122);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_send_message_batch(
    input: &crate::input::SendMessageBatchInput,
) -> Result<aws_smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "SendMessageBatch", "2012-11-05");
    #[allow(unused_mut)]
    let mut scope_123 = writer.prefix("QueueUrl");
    if let Some(var_124) = &input.queue_url {
        scope_123.string(var_124);
    }
    #[allow(unused_mut)]
    let mut scope_125 = writer.prefix("SendMessageBatchRequestEntry");
    if let Some(var_126) = &input.entries {
        let mut list_128 = scope_125.start_list(true, None);
        for item_127 in var_126 {
            #[allow(unused_mut)]
            let mut entry_129 = list_128.entry();
            crate::query_ser::serialize_structure_crate_model_send_message_batch_request_entry(
                entry_129, item_127,
            );
        }
        list_128.finish();
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_set_queue_attributes(
    input: &crate::input::SetQueueAttributesInput,
) -> Result<aws_smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        aws_smithy_query::QueryWriter::new(&mut out, "SetQueueAttributes", "2012-11-05");
    #[allow(unused_mut)]
    let mut scope_130 = writer.prefix("QueueUrl");
    if let Some(var_131) = &input.queue_url {
        scope_130.string(var_131);
    }
    #[allow(unused_mut)]
    let mut scope_132 = writer.prefix("Attribute");
    if let Some(var_133) = &input.attributes {
        let mut map_134 = scope_132.start_map(true, "Name", "Value");
        for (key_135, value_136) in var_133 {
            #[allow(unused_mut)]
            let mut entry_137 = map_134.entry(key_135.as_str());
            {
                entry_137.string(value_136);
            }
        }
        map_134.finish();
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_tag_queue(
    input: &crate::input::TagQueueInput,
) -> Result<aws_smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "TagQueue", "2012-11-05");
    #[allow(unused_mut)]
    let mut scope_138 = writer.prefix("QueueUrl");
    if let Some(var_139) = &input.queue_url {
        scope_138.string(var_139);
    }
    #[allow(unused_mut)]
    let mut scope_140 = writer.prefix("Tag");
    if let Some(var_141) = &input.tags {
        let mut map_142 = scope_140.start_map(true, "Key", "Value");
        for (key_143, value_144) in var_141 {
            #[allow(unused_mut)]
            let mut entry_145 = map_142.entry(key_143);
            {
                entry_145.string(value_144);
            }
        }
        map_142.finish();
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_untag_queue(
    input: &crate::input::UntagQueueInput,
) -> Result<aws_smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "UntagQueue", "2012-11-05");
    #[allow(unused_mut)]
    let mut scope_146 = writer.prefix("QueueUrl");
    if let Some(var_147) = &input.queue_url {
        scope_146.string(var_147);
    }
    #[allow(unused_mut)]
    let mut scope_148 = writer.prefix("TagKey");
    if let Some(var_149) = &input.tag_keys {
        let mut list_151 = scope_148.start_list(true, None);
        for item_150 in var_149 {
            #[allow(unused_mut)]
            let mut entry_152 = list_151.entry();
            entry_152.string(item_150);
        }
        list_151.finish();
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}
