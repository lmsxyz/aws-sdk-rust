// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_operation_crate_operation_generate_data_set(
    input: &crate::input::GenerateDataSetInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_generate_data_set_input(&mut object, input);
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_start_support_data_export(
    input: &crate::input::StartSupportDataExportInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_types::Error> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_start_support_data_export_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}
