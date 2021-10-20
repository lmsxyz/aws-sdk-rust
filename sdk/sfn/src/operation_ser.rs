// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_operation_crate_operation_create_activity(
    input: &crate::input::CreateActivityInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_create_activity_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_create_state_machine(
    input: &crate::input::CreateStateMachineInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_create_state_machine_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_delete_activity(
    input: &crate::input::DeleteActivityInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_delete_activity_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_delete_state_machine(
    input: &crate::input::DeleteStateMachineInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_delete_state_machine_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_describe_activity(
    input: &crate::input::DescribeActivityInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_describe_activity_input(&mut object, input);
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

pub fn serialize_operation_crate_operation_describe_state_machine(
    input: &crate::input::DescribeStateMachineInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_describe_state_machine_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_describe_state_machine_for_execution(
    input: &crate::input::DescribeStateMachineForExecutionInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_describe_state_machine_for_execution_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_get_activity_task(
    input: &crate::input::GetActivityTaskInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_get_activity_task_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_get_execution_history(
    input: &crate::input::GetExecutionHistoryInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_get_execution_history_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_list_activities(
    input: &crate::input::ListActivitiesInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_list_activities_input(&mut object, input);
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

pub fn serialize_operation_crate_operation_list_state_machines(
    input: &crate::input::ListStateMachinesInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_list_state_machines_input(&mut object, input);
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

pub fn serialize_operation_crate_operation_send_task_failure(
    input: &crate::input::SendTaskFailureInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_send_task_failure_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_send_task_heartbeat(
    input: &crate::input::SendTaskHeartbeatInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_send_task_heartbeat_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_send_task_success(
    input: &crate::input::SendTaskSuccessInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_send_task_success_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_start_execution(
    input: &crate::input::StartExecutionInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_start_execution_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_start_sync_execution(
    input: &crate::input::StartSyncExecutionInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_start_sync_execution_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_stop_execution(
    input: &crate::input::StopExecutionInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_stop_execution_input(&mut object, input);
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

pub fn serialize_operation_crate_operation_update_state_machine(
    input: &crate::input::UpdateStateMachineInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_update_state_machine_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}
