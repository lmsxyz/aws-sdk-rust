// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_operation_crate_operation_create_access(
    input: &crate::input::CreateAccessInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_create_access_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_create_server(
    input: &crate::input::CreateServerInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_create_server_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_create_user(
    input: &crate::input::CreateUserInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_create_user_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_create_workflow(
    input: &crate::input::CreateWorkflowInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_create_workflow_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_delete_access(
    input: &crate::input::DeleteAccessInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_delete_access_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_delete_server(
    input: &crate::input::DeleteServerInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_delete_server_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_delete_ssh_public_key(
    input: &crate::input::DeleteSshPublicKeyInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_delete_ssh_public_key_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_delete_user(
    input: &crate::input::DeleteUserInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_delete_user_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_delete_workflow(
    input: &crate::input::DeleteWorkflowInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_delete_workflow_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_describe_access(
    input: &crate::input::DescribeAccessInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_describe_access_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_describe_execution(
    input: &crate::input::DescribeExecutionInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_describe_execution_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_describe_security_policy(
    input: &crate::input::DescribeSecurityPolicyInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_describe_security_policy_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_describe_server(
    input: &crate::input::DescribeServerInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_describe_server_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_describe_user(
    input: &crate::input::DescribeUserInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_describe_user_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_describe_workflow(
    input: &crate::input::DescribeWorkflowInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_describe_workflow_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_import_ssh_public_key(
    input: &crate::input::ImportSshPublicKeyInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_import_ssh_public_key_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_list_accesses(
    input: &crate::input::ListAccessesInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_list_accesses_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_list_executions(
    input: &crate::input::ListExecutionsInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_list_executions_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_list_security_policies(
    input: &crate::input::ListSecurityPoliciesInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_list_security_policies_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_list_servers(
    input: &crate::input::ListServersInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_list_servers_input(&mut object, input);
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

pub fn serialize_operation_crate_operation_list_users(
    input: &crate::input::ListUsersInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_list_users_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_list_workflows(
    input: &crate::input::ListWorkflowsInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_list_workflows_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_send_workflow_step_state(
    input: &crate::input::SendWorkflowStepStateInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_send_workflow_step_state_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_start_server(
    input: &crate::input::StartServerInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_start_server_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_stop_server(
    input: &crate::input::StopServerInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_stop_server_input(&mut object, input);
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

pub fn serialize_operation_crate_operation_test_identity_provider(
    input: &crate::input::TestIdentityProviderInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_test_identity_provider_input(
        &mut object,
        input,
    );
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

pub fn serialize_operation_crate_operation_update_access(
    input: &crate::input::UpdateAccessInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_update_access_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_update_server(
    input: &crate::input::UpdateServerInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_update_server_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_update_user(
    input: &crate::input::UpdateUserInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_update_user_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}
