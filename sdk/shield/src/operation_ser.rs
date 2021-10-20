// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_operation_crate_operation_associate_drt_log_bucket(
    input: &crate::input::AssociateDrtLogBucketInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_associate_drt_log_bucket_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_associate_drt_role(
    input: &crate::input::AssociateDrtRoleInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_associate_drt_role_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_associate_health_check(
    input: &crate::input::AssociateHealthCheckInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_associate_health_check_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_associate_proactive_engagement_details(
    input: &crate::input::AssociateProactiveEngagementDetailsInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_associate_proactive_engagement_details_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_create_protection(
    input: &crate::input::CreateProtectionInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_create_protection_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_create_protection_group(
    input: &crate::input::CreateProtectionGroupInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_create_protection_group_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_create_subscription(
    _input: &crate::input::CreateSubscriptionInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    Ok(aws_smithy_http::body::SdkBody::from("{}"))
}

pub fn serialize_operation_crate_operation_delete_protection(
    input: &crate::input::DeleteProtectionInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_delete_protection_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_delete_protection_group(
    input: &crate::input::DeleteProtectionGroupInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_delete_protection_group_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_delete_subscription(
    _input: &crate::input::DeleteSubscriptionInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    Ok(aws_smithy_http::body::SdkBody::from("{}"))
}

pub fn serialize_operation_crate_operation_describe_attack(
    input: &crate::input::DescribeAttackInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_describe_attack_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_describe_attack_statistics(
    _input: &crate::input::DescribeAttackStatisticsInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    Ok(aws_smithy_http::body::SdkBody::from("{}"))
}

pub fn serialize_operation_crate_operation_describe_drt_access(
    _input: &crate::input::DescribeDrtAccessInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    Ok(aws_smithy_http::body::SdkBody::from("{}"))
}

pub fn serialize_operation_crate_operation_describe_emergency_contact_settings(
    _input: &crate::input::DescribeEmergencyContactSettingsInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    Ok(aws_smithy_http::body::SdkBody::from("{}"))
}

pub fn serialize_operation_crate_operation_describe_protection(
    input: &crate::input::DescribeProtectionInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_describe_protection_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_describe_protection_group(
    input: &crate::input::DescribeProtectionGroupInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_describe_protection_group_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_describe_subscription(
    _input: &crate::input::DescribeSubscriptionInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    Ok(aws_smithy_http::body::SdkBody::from("{}"))
}

pub fn serialize_operation_crate_operation_disable_proactive_engagement(
    _input: &crate::input::DisableProactiveEngagementInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    Ok(aws_smithy_http::body::SdkBody::from("{}"))
}

pub fn serialize_operation_crate_operation_disassociate_drt_log_bucket(
    input: &crate::input::DisassociateDrtLogBucketInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_disassociate_drt_log_bucket_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_disassociate_drt_role(
    _input: &crate::input::DisassociateDrtRoleInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    Ok(aws_smithy_http::body::SdkBody::from("{}"))
}

pub fn serialize_operation_crate_operation_disassociate_health_check(
    input: &crate::input::DisassociateHealthCheckInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_disassociate_health_check_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_enable_proactive_engagement(
    _input: &crate::input::EnableProactiveEngagementInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    Ok(aws_smithy_http::body::SdkBody::from("{}"))
}

pub fn serialize_operation_crate_operation_get_subscription_state(
    _input: &crate::input::GetSubscriptionStateInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    Ok(aws_smithy_http::body::SdkBody::from("{}"))
}

pub fn serialize_operation_crate_operation_list_attacks(
    input: &crate::input::ListAttacksInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_list_attacks_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_list_protection_groups(
    input: &crate::input::ListProtectionGroupsInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_list_protection_groups_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_list_protections(
    input: &crate::input::ListProtectionsInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_list_protections_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_list_resources_in_protection_group(
    input: &crate::input::ListResourcesInProtectionGroupInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_list_resources_in_protection_group_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_list_tags_for_resource(
    input: &crate::input::ListTagsForResourceInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_list_tags_for_resource_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_tag_resource(
    input: &crate::input::TagResourceInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_tag_resource_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_untag_resource(
    input: &crate::input::UntagResourceInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_untag_resource_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_update_emergency_contact_settings(
    input: &crate::input::UpdateEmergencyContactSettingsInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_update_emergency_contact_settings_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_update_protection_group(
    input: &crate::input::UpdateProtectionGroupInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_update_protection_group_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_update_subscription(
    input: &crate::input::UpdateSubscriptionInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_update_subscription_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}
