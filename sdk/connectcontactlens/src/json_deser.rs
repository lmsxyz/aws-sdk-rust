// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn parse_http_generic_error(
    response: &http::Response<bytes::Bytes>,
) -> Result<aws_smithy_types::Error, aws_smithy_json::deserialize::Error> {
    crate::json_errors::parse_generic_error(response.body(), response.headers())
}

pub fn deser_structure_crate_error_access_denied_exception_json_err(
    value: &[u8],
    mut builder: crate::error::access_denied_exception::Builder,
) -> Result<crate::error::access_denied_exception::Builder, aws_smithy_json::deserialize::Error> {
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
                    "Message" => {
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

pub fn deser_structure_crate_error_internal_service_exception_json_err(
    value: &[u8],
    mut builder: crate::error::internal_service_exception::Builder,
) -> Result<crate::error::internal_service_exception::Builder, aws_smithy_json::deserialize::Error>
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
                    "Message" => {
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
                    "Message" => {
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
                    "Message" => {
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
                    "Message" => {
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

pub fn deser_operation_crate_operation_list_realtime_contact_analysis_segments(
    value: &[u8],
    mut builder: crate::output::list_realtime_contact_analysis_segments_output::Builder,
) -> Result<
    crate::output::list_realtime_contact_analysis_segments_output::Builder,
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
                    "NextToken" => {
                        builder = builder.set_next_token(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "Segments" => {
                        builder = builder.set_segments(
                            crate::json_deser::deser_list_com_amazonaws_connectcontactlens_realtime_contact_analysis_segments(tokens)?
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
pub fn deser_list_com_amazonaws_connectcontactlens_realtime_contact_analysis_segments<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<
    Option<std::vec::Vec<crate::model::RealtimeContactAnalysisSegment>>,
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
                            crate::json_deser::deser_structure_crate_model_realtime_contact_analysis_segment(tokens)?
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

pub fn deser_structure_crate_model_realtime_contact_analysis_segment<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<crate::model::RealtimeContactAnalysisSegment>, aws_smithy_json::deserialize::Error>
where
    I: Iterator<
        Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::Error>,
    >,
{
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::RealtimeContactAnalysisSegment::builder();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Transcript" => {
                                builder = builder.set_transcript(
                                    crate::json_deser::deser_structure_crate_model_transcript(
                                        tokens,
                                    )?,
                                );
                            }
                            "Categories" => {
                                builder = builder.set_categories(
                                    crate::json_deser::deser_structure_crate_model_categories(
                                        tokens,
                                    )?,
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

pub fn deser_structure_crate_model_transcript<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<crate::model::Transcript>, aws_smithy_json::deserialize::Error>
where
    I: Iterator<
        Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::Error>,
    >,
{
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::Transcript::builder();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Id" => {
                                builder = builder.set_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "ParticipantId" => {
                                builder = builder.set_participant_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "ParticipantRole" => {
                                builder = builder.set_participant_role(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "Content" => {
                                builder = builder.set_content(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "BeginOffsetMillis" => {
                                builder = builder.set_begin_offset_millis(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|v| v.to_i32()),
                                );
                            }
                            "EndOffsetMillis" => {
                                builder = builder.set_end_offset_millis(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|v| v.to_i32()),
                                );
                            }
                            "Sentiment" => {
                                builder = builder.set_sentiment(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped()
                                            .map(|u| crate::model::SentimentValue::from(u.as_ref()))
                                    })
                                    .transpose()?,
                                );
                            }
                            "IssuesDetected" => {
                                builder = builder.set_issues_detected(
                                    crate::json_deser::deser_list_com_amazonaws_connectcontactlens_issues_detected(tokens)?
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

pub fn deser_structure_crate_model_categories<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<crate::model::Categories>, aws_smithy_json::deserialize::Error>
where
    I: Iterator<
        Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::Error>,
    >,
{
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::Categories::builder();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "MatchedCategories" => {
                                builder = builder.set_matched_categories(
                                    crate::json_deser::deser_list_com_amazonaws_connectcontactlens_matched_categories(tokens)?
                                );
                            }
                            "MatchedDetails" => {
                                builder = builder.set_matched_details(
                                    crate::json_deser::deser_map_com_amazonaws_connectcontactlens_matched_details(tokens)?
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

#[allow(clippy::type_complexity, non_snake_case)]
pub fn deser_list_com_amazonaws_connectcontactlens_issues_detected<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<std::vec::Vec<crate::model::IssueDetected>>, aws_smithy_json::deserialize::Error>
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
                            crate::json_deser::deser_structure_crate_model_issue_detected(tokens)?;
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
pub fn deser_list_com_amazonaws_connectcontactlens_matched_categories<'a, I>(
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
pub fn deser_map_com_amazonaws_connectcontactlens_matched_details<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<
    Option<std::collections::HashMap<std::string::String, crate::model::CategoryDetails>>,
    aws_smithy_json::deserialize::Error,
>
where
    I: Iterator<
        Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::Error>,
    >,
{
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            let mut map = std::collections::HashMap::new();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        let key = key.to_unescaped().map(|u| u.into_owned())?;
                        let value =
                            crate::json_deser::deser_structure_crate_model_category_details(
                                tokens,
                            )?;
                        if let Some(value) = value {
                            map.insert(key, value);
                        }
                    }
                    _ => {
                        return Err(aws_smithy_json::deserialize::Error::custom(
                            "expected object key or end object",
                        ))
                    }
                }
            }
            Ok(Some(map))
        }
        _ => Err(aws_smithy_json::deserialize::Error::custom(
            "expected start object or null",
        )),
    }
}

pub fn deser_structure_crate_model_issue_detected<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<crate::model::IssueDetected>, aws_smithy_json::deserialize::Error>
where
    I: Iterator<
        Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::Error>,
    >,
{
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::IssueDetected::builder();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "CharacterOffsets" => {
                                builder = builder.set_character_offsets(
                                    crate::json_deser::deser_structure_crate_model_character_offsets(tokens)?
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

pub fn deser_structure_crate_model_category_details<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<crate::model::CategoryDetails>, aws_smithy_json::deserialize::Error>
where
    I: Iterator<
        Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::Error>,
    >,
{
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::CategoryDetails::builder();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "PointsOfInterest" => {
                                builder = builder.set_points_of_interest(
                                    crate::json_deser::deser_list_com_amazonaws_connectcontactlens_points_of_interest(tokens)?
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

pub fn deser_structure_crate_model_character_offsets<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<crate::model::CharacterOffsets>, aws_smithy_json::deserialize::Error>
where
    I: Iterator<
        Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::Error>,
    >,
{
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::CharacterOffsets::builder();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "BeginOffsetChar" => {
                                builder = builder.set_begin_offset_char(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|v| v.to_i32()),
                                );
                            }
                            "EndOffsetChar" => {
                                builder = builder.set_end_offset_char(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|v| v.to_i32()),
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

#[allow(clippy::type_complexity, non_snake_case)]
pub fn deser_list_com_amazonaws_connectcontactlens_points_of_interest<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<std::vec::Vec<crate::model::PointOfInterest>>, aws_smithy_json::deserialize::Error>
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
                            crate::json_deser::deser_structure_crate_model_point_of_interest(
                                tokens,
                            )?;
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

pub fn deser_structure_crate_model_point_of_interest<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<crate::model::PointOfInterest>, aws_smithy_json::deserialize::Error>
where
    I: Iterator<
        Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::Error>,
    >,
{
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::PointOfInterest::builder();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "BeginOffsetMillis" => {
                                builder = builder.set_begin_offset_millis(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|v| v.to_i32()),
                                );
                            }
                            "EndOffsetMillis" => {
                                builder = builder.set_end_offset_millis(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|v| v.to_i32()),
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
