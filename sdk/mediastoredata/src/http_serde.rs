// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn deser_header_describe_object_describe_object_output_cache_control(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Cache-Control").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_describe_object_describe_object_output_content_length(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i64>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Content-Length").iter();
    let var_1 = aws_smithy_http::header::read_many_primitive::<i64>(headers)?;
    if var_1.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_1.len()),
        ))
    } else {
        let mut var_1 = var_1;
        Ok(var_1.pop())
    }
}

pub fn deser_header_describe_object_describe_object_output_content_type(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Content-Type").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_describe_object_describe_object_output_e_tag(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("ETag").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_describe_object_describe_object_output_last_modified(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<aws_smithy_types::Instant>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Last-Modified").iter();
    let var_2: Vec<aws_smithy_types::Instant> =
        aws_smithy_http::header::many_dates(headers, aws_smithy_types::instant::Format::HttpDate)?;
    if var_2.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_2.len()),
        ))
    } else {
        let mut var_2 = var_2;
        Ok(var_2.pop())
    }
}

pub fn deser_payload_get_object_get_object_output_body(
    body: &mut aws_smithy_http::body::SdkBody,
) -> std::result::Result<aws_smithy_http::byte_stream::ByteStream, crate::error::GetObjectError> {
    // replace the body with an empty body
    let body = std::mem::replace(body, aws_smithy_http::body::SdkBody::taken());
    Ok(aws_smithy_http::byte_stream::ByteStream::new(body))
}

pub fn deser_header_get_object_get_object_output_cache_control(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Cache-Control").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_get_object_get_object_output_content_length(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i64>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Content-Length").iter();
    let var_3 = aws_smithy_http::header::read_many_primitive::<i64>(headers)?;
    if var_3.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_3.len()),
        ))
    } else {
        let mut var_3 = var_3;
        Ok(var_3.pop())
    }
}

pub fn deser_header_get_object_get_object_output_content_range(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Content-Range").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_get_object_get_object_output_content_type(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Content-Type").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_get_object_get_object_output_e_tag(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("ETag").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_get_object_get_object_output_last_modified(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<aws_smithy_types::Instant>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Last-Modified").iter();
    let var_4: Vec<aws_smithy_types::Instant> =
        aws_smithy_http::header::many_dates(headers, aws_smithy_types::instant::Format::HttpDate)?;
    if var_4.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_4.len()),
        ))
    } else {
        let mut var_4 = var_4;
        Ok(var_4.pop())
    }
}
