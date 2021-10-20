// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn parse_http_generic_error(
    response: &http::Response<bytes::Bytes>,
) -> Result<aws_smithy_types::Error, aws_smithy_json::deserialize::Error> {
    crate::json_errors::parse_generic_error(response.body(), response.headers())
}

pub fn deser_structure_crate_error_internal_failure_exception_json_err(
    value: &[u8],
    mut builder: crate::error::internal_failure_exception::Builder,
) -> Result<crate::error::internal_failure_exception::Builder, aws_smithy_json::deserialize::Error>
{
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "message" => {
                        builder = builder.set_message(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(aws_smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_structure_crate_error_invalid_request_exception_json_err(
    value: &[u8],
    mut builder: crate::error::invalid_request_exception::Builder,
) -> Result<crate::error::invalid_request_exception::Builder, aws_smithy_json::deserialize::Error> {
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "message" => {
                        builder = builder.set_message(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(aws_smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_structure_crate_error_method_not_allowed_exception_json_err(
    value: &[u8],
    mut builder: crate::error::method_not_allowed_exception::Builder,
) -> Result<crate::error::method_not_allowed_exception::Builder, aws_smithy_json::deserialize::Error>
{
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "message" => {
                        builder = builder.set_message(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(aws_smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_structure_crate_error_resource_not_found_exception_json_err(
    value: &[u8],
    mut builder: crate::error::resource_not_found_exception::Builder,
) -> Result<crate::error::resource_not_found_exception::Builder, aws_smithy_json::deserialize::Error>
{
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "message" => {
                        builder = builder.set_message(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(aws_smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_structure_crate_error_service_unavailable_exception_json_err(
    value: &[u8],
    mut builder: crate::error::service_unavailable_exception::Builder,
) -> Result<crate::error::service_unavailable_exception::Builder, aws_smithy_json::deserialize::Error>
{
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "message" => {
                        builder = builder.set_message(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(aws_smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_structure_crate_error_throttling_exception_json_err(
    value: &[u8],
    mut builder: crate::error::throttling_exception::Builder,
) -> Result<crate::error::throttling_exception::Builder, aws_smithy_json::deserialize::Error> {
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "message" => {
                        builder = builder.set_message(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(aws_smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_structure_crate_error_unauthorized_exception_json_err(
    value: &[u8],
    mut builder: crate::error::unauthorized_exception::Builder,
) -> Result<crate::error::unauthorized_exception::Builder, aws_smithy_json::deserialize::Error> {
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "message" => {
                        builder = builder.set_message(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(aws_smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_structure_crate_error_unsupported_document_encoding_exception_json_err(
    value: &[u8],
    mut builder: crate::error::unsupported_document_encoding_exception::Builder,
) -> Result<
    crate::error::unsupported_document_encoding_exception::Builder,
    aws_smithy_json::deserialize::Error,
> {
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "message" => {
                        builder = builder.set_message(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(aws_smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_operation_crate_operation_get_retained_message(
    value: &[u8],
    mut builder: crate::output::get_retained_message_output::Builder,
) -> Result<crate::output::get_retained_message_output::Builder, aws_smithy_json::deserialize::Error>
{
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "lastModifiedTime" => {
                        builder = builder.set_last_modified_time(
                            aws_smithy_json::deserialize::token::expect_number_or_null(
                                tokens.next(),
                            )?
                            .map(|v| v.to_i64()),
                        );
                    }
                    "payload" => {
                        builder = builder.set_payload(
                            aws_smithy_json::deserialize::token::expect_blob_or_null(
                                tokens.next(),
                            )?,
                        );
                    }
                    "qos" => {
                        builder = builder.set_qos(
                            aws_smithy_json::deserialize::token::expect_number_or_null(
                                tokens.next(),
                            )?
                            .map(|v| v.to_i32()),
                        );
                    }
                    "topic" => {
                        builder = builder.set_topic(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(aws_smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_operation_crate_operation_list_named_shadows_for_thing(
    value: &[u8],
    mut builder: crate::output::list_named_shadows_for_thing_output::Builder,
) -> Result<
    crate::output::list_named_shadows_for_thing_output::Builder,
    aws_smithy_json::deserialize::Error,
> {
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "nextToken" => {
                        builder = builder.set_next_token(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "results" => {
                        builder = builder.set_results(
                            crate::json_deser::deser_list_com_amazonaws_iotdataplane_named_shadow_list(tokens)?
                        );
                    }
                    "timestamp" => {
                        builder = builder.set_timestamp(
                            aws_smithy_json::deserialize::token::expect_number_or_null(
                                tokens.next(),
                            )?
                            .map(|v| v.to_i64()),
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(aws_smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_operation_crate_operation_list_retained_messages(
    value: &[u8],
    mut builder: crate::output::list_retained_messages_output::Builder,
) -> Result<
    crate::output::list_retained_messages_output::Builder,
    aws_smithy_json::deserialize::Error,
> {
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "nextToken" => {
                        builder = builder.set_next_token(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "retainedTopics" => {
                        builder = builder.set_retained_topics(
                            crate::json_deser::deser_list_com_amazonaws_iotdataplane_retained_message_list(tokens)?
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(aws_smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_structure_crate_error_conflict_exception_json_err(
    value: &[u8],
    mut builder: crate::error::conflict_exception::Builder,
) -> Result<crate::error::conflict_exception::Builder, aws_smithy_json::deserialize::Error> {
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "message" => {
                        builder = builder.set_message(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(aws_smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_structure_crate_error_request_entity_too_large_exception_json_err(
    value: &[u8],
    mut builder: crate::error::request_entity_too_large_exception::Builder,
) -> Result<
    crate::error::request_entity_too_large_exception::Builder,
    aws_smithy_json::deserialize::Error,
> {
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "message" => {
                        builder = builder.set_message(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(aws_smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

#[allow(clippy::type_complexity, non_snake_case)]
pub fn deser_list_com_amazonaws_iotdataplane_named_shadow_list<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<std::vec::Vec<std::string::String>>, aws_smithy_json::deserialize::Error>
where
    I: Iterator<
        Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::Error>,
    >,
{
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(aws_smithy_json::deserialize::Token::StartArray { .. }) => {
            let mut items = Vec::new();
            loop {
                match tokens.peek() {
                    Some(Ok(aws_smithy_json::deserialize::Token::EndArray { .. })) => {
                        tokens.next().transpose().unwrap();
                        break;
                    }
                    _ => {
                        let value = aws_smithy_json::deserialize::token::expect_string_or_null(
                            tokens.next(),
                        )?
                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                        .transpose()?;
                        if let Some(value) = value {
                            items.push(value);
                        }
                    }
                }
            }
            Ok(Some(items))
        }
        _ => Err(aws_smithy_json::deserialize::Error::custom(
            "expected start array or null",
        )),
    }
}

#[allow(clippy::type_complexity, non_snake_case)]
pub fn deser_list_com_amazonaws_iotdataplane_retained_message_list<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<
    Option<std::vec::Vec<crate::model::RetainedMessageSummary>>,
    aws_smithy_json::deserialize::Error,
>
where
    I: Iterator<
        Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::Error>,
    >,
{
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(aws_smithy_json::deserialize::Token::StartArray { .. }) => {
            let mut items = Vec::new();
            loop {
                match tokens.peek() {
                    Some(Ok(aws_smithy_json::deserialize::Token::EndArray { .. })) => {
                        tokens.next().transpose().unwrap();
                        break;
                    }
                    _ => {
                        let value =
                            crate::json_deser::deser_structure_crate_model_retained_message_summary(tokens)?
                        ;
                        if let Some(value) = value {
                            items.push(value);
                        }
                    }
                }
            }
            Ok(Some(items))
        }
        _ => Err(aws_smithy_json::deserialize::Error::custom(
            "expected start array or null",
        )),
    }
}

pub fn deser_structure_crate_model_retained_message_summary<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<crate::model::RetainedMessageSummary>, aws_smithy_json::deserialize::Error>
where
    I: Iterator<
        Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::Error>,
    >,
{
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::RetainedMessageSummary::builder();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "topic" => {
                                builder = builder.set_topic(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "payloadSize" => {
                                builder = builder.set_payload_size(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|v| v.to_i64()),
                                );
                            }
                            "qos" => {
                                builder = builder.set_qos(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|v| v.to_i32()),
                                );
                            }
                            "lastModifiedTime" => {
                                builder = builder.set_last_modified_time(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|v| v.to_i64()),
                                );
                            }
                            _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                        }
                    }
                    _ => {
                        return Err(aws_smithy_json::deserialize::Error::custom(
                            "expected object key or end object",
                        ))
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(aws_smithy_json::deserialize::Error::custom(
            "expected start object or null",
        )),
    }
}
