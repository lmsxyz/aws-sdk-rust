// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_payload_invoke_endpoint_input(
    payload: std::option::Option<aws_smithy_types::Blob>,
) -> std::result::Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::BuildError> {
    let payload = match payload {
        Some(t) => t,
        None => return Ok(aws_smithy_http::body::SdkBody::from("")),
    };
    #[allow(clippy::useless_conversion)]
    Ok(aws_smithy_http::body::SdkBody::from(payload.into_inner()))
}
