// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_operation_crate_operation_configure_logs(
    input: &crate::input::ConfigureLogsInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_configure_logs_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_create_channel(
    input: &crate::input::CreateChannelInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_create_channel_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_create_harvest_job(
    input: &crate::input::CreateHarvestJobInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_create_harvest_job_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_create_origin_endpoint(
    input: &crate::input::CreateOriginEndpointInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_create_origin_endpoint_input(
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

pub fn serialize_operation_crate_operation_update_channel(
    input: &crate::input::UpdateChannelInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_update_channel_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_update_origin_endpoint(
    input: &crate::input::UpdateOriginEndpointInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_update_origin_endpoint_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}
