// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn deser_header_delete_session_limit_exceeded_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_get_session_limit_exceeded_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_post_content_post_content_output_active_contexts(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-active-contexts").iter();
    let var_1: Vec<std::string::String> = aws_smithy_http::header::read_many_from_str(headers)?;
    let var_1: std::result::Result<Vec<_>, _> = var_1
        .iter()
        .map(|s| {
            aws_smithy_types::base64::decode(s)
                .map_err(|_| {
                    aws_smithy_http::header::ParseError::new_with_message("failed to decode base64")
                })
                .and_then(|bytes| {
                    String::from_utf8(bytes).map_err(|_| {
                        aws_smithy_http::header::ParseError::new_with_message(
                            "base64 encoded data was not valid utf-8",
                        )
                    })
                })
        })
        .collect();
    let var_1 = var_1?;
    if var_1.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_1.len()),
        ))
    } else {
        let mut var_1 = var_1;
        Ok(var_1.pop())
    }
}

pub fn deser_header_post_content_post_content_output_alternative_intents(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-alternative-intents").iter();
    let var_2: Vec<std::string::String> = aws_smithy_http::header::read_many_from_str(headers)?;
    let var_2: std::result::Result<Vec<_>, _> = var_2
        .iter()
        .map(|s| {
            aws_smithy_types::base64::decode(s)
                .map_err(|_| {
                    aws_smithy_http::header::ParseError::new_with_message("failed to decode base64")
                })
                .and_then(|bytes| {
                    String::from_utf8(bytes).map_err(|_| {
                        aws_smithy_http::header::ParseError::new_with_message(
                            "base64 encoded data was not valid utf-8",
                        )
                    })
                })
        })
        .collect();
    let var_2 = var_2?;
    if var_2.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_2.len()),
        ))
    } else {
        let mut var_2 = var_2;
        Ok(var_2.pop())
    }
}

pub fn deser_payload_post_content_post_content_output_audio_stream(
    body: &mut aws_smithy_http::body::SdkBody,
) -> std::result::Result<aws_smithy_http::byte_stream::ByteStream, crate::error::PostContentError> {
    // replace the body with an empty body
    let body = std::mem::replace(body, aws_smithy_http::body::SdkBody::taken());
    Ok(aws_smithy_http::byte_stream::ByteStream::new(body))
}

pub fn deser_header_post_content_post_content_output_bot_version(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-bot-version").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_post_content_post_content_output_content_type(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Content-Type").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_post_content_post_content_output_dialog_state(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<crate::model::DialogState>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-dialog-state").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_post_content_post_content_output_encoded_input_transcript(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map
        .get_all("x-amz-lex-encoded-input-transcript")
        .iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_post_content_post_content_output_encoded_message(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-encoded-message").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_post_content_post_content_output_input_transcript(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-input-transcript").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_post_content_post_content_output_intent_name(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-intent-name").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_post_content_post_content_output_message(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-message").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_post_content_post_content_output_message_format(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<crate::model::MessageFormatType>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-message-format").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_post_content_post_content_output_nlu_intent_confidence(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-nlu-intent-confidence").iter();
    let var_3: Vec<std::string::String> = aws_smithy_http::header::read_many_from_str(headers)?;
    let var_3: std::result::Result<Vec<_>, _> = var_3
        .iter()
        .map(|s| {
            aws_smithy_types::base64::decode(s)
                .map_err(|_| {
                    aws_smithy_http::header::ParseError::new_with_message("failed to decode base64")
                })
                .and_then(|bytes| {
                    String::from_utf8(bytes).map_err(|_| {
                        aws_smithy_http::header::ParseError::new_with_message(
                            "base64 encoded data was not valid utf-8",
                        )
                    })
                })
        })
        .collect();
    let var_3 = var_3?;
    if var_3.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_3.len()),
        ))
    } else {
        let mut var_3 = var_3;
        Ok(var_3.pop())
    }
}

pub fn deser_header_post_content_post_content_output_sentiment_response(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-sentiment").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_post_content_post_content_output_session_attributes(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-session-attributes").iter();
    let var_4: Vec<std::string::String> = aws_smithy_http::header::read_many_from_str(headers)?;
    let var_4: std::result::Result<Vec<_>, _> = var_4
        .iter()
        .map(|s| {
            aws_smithy_types::base64::decode(s)
                .map_err(|_| {
                    aws_smithy_http::header::ParseError::new_with_message("failed to decode base64")
                })
                .and_then(|bytes| {
                    String::from_utf8(bytes).map_err(|_| {
                        aws_smithy_http::header::ParseError::new_with_message(
                            "base64 encoded data was not valid utf-8",
                        )
                    })
                })
        })
        .collect();
    let var_4 = var_4?;
    if var_4.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_4.len()),
        ))
    } else {
        let mut var_4 = var_4;
        Ok(var_4.pop())
    }
}

pub fn deser_header_post_content_post_content_output_session_id(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-session-id").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_post_content_post_content_output_slot_to_elicit(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-slot-to-elicit").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_post_content_post_content_output_slots(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-slots").iter();
    let var_5: Vec<std::string::String> = aws_smithy_http::header::read_many_from_str(headers)?;
    let var_5: std::result::Result<Vec<_>, _> = var_5
        .iter()
        .map(|s| {
            aws_smithy_types::base64::decode(s)
                .map_err(|_| {
                    aws_smithy_http::header::ParseError::new_with_message("failed to decode base64")
                })
                .and_then(|bytes| {
                    String::from_utf8(bytes).map_err(|_| {
                        aws_smithy_http::header::ParseError::new_with_message(
                            "base64 encoded data was not valid utf-8",
                        )
                    })
                })
        })
        .collect();
    let var_5 = var_5?;
    if var_5.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_5.len()),
        ))
    } else {
        let mut var_5 = var_5;
        Ok(var_5.pop())
    }
}

pub fn deser_header_post_content_limit_exceeded_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_post_text_limit_exceeded_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_put_session_put_session_output_active_contexts(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-active-contexts").iter();
    let var_6: Vec<std::string::String> = aws_smithy_http::header::read_many_from_str(headers)?;
    let var_6: std::result::Result<Vec<_>, _> = var_6
        .iter()
        .map(|s| {
            aws_smithy_types::base64::decode(s)
                .map_err(|_| {
                    aws_smithy_http::header::ParseError::new_with_message("failed to decode base64")
                })
                .and_then(|bytes| {
                    String::from_utf8(bytes).map_err(|_| {
                        aws_smithy_http::header::ParseError::new_with_message(
                            "base64 encoded data was not valid utf-8",
                        )
                    })
                })
        })
        .collect();
    let var_6 = var_6?;
    if var_6.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_6.len()),
        ))
    } else {
        let mut var_6 = var_6;
        Ok(var_6.pop())
    }
}

pub fn deser_payload_put_session_put_session_output_audio_stream(
    body: &mut aws_smithy_http::body::SdkBody,
) -> std::result::Result<aws_smithy_http::byte_stream::ByteStream, crate::error::PutSessionError> {
    // replace the body with an empty body
    let body = std::mem::replace(body, aws_smithy_http::body::SdkBody::taken());
    Ok(aws_smithy_http::byte_stream::ByteStream::new(body))
}

pub fn deser_header_put_session_put_session_output_content_type(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Content-Type").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_put_session_put_session_output_dialog_state(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<crate::model::DialogState>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-dialog-state").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_put_session_put_session_output_encoded_message(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-encoded-message").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_put_session_put_session_output_intent_name(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-intent-name").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_put_session_put_session_output_message(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-message").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_put_session_put_session_output_message_format(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<crate::model::MessageFormatType>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-message-format").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_put_session_put_session_output_session_attributes(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-session-attributes").iter();
    let var_7: Vec<std::string::String> = aws_smithy_http::header::read_many_from_str(headers)?;
    let var_7: std::result::Result<Vec<_>, _> = var_7
        .iter()
        .map(|s| {
            aws_smithy_types::base64::decode(s)
                .map_err(|_| {
                    aws_smithy_http::header::ParseError::new_with_message("failed to decode base64")
                })
                .and_then(|bytes| {
                    String::from_utf8(bytes).map_err(|_| {
                        aws_smithy_http::header::ParseError::new_with_message(
                            "base64 encoded data was not valid utf-8",
                        )
                    })
                })
        })
        .collect();
    let var_7 = var_7?;
    if var_7.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_7.len()),
        ))
    } else {
        let mut var_7 = var_7;
        Ok(var_7.pop())
    }
}

pub fn deser_header_put_session_put_session_output_session_id(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-session-id").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_put_session_put_session_output_slot_to_elicit(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-slot-to-elicit").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_put_session_put_session_output_slots(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-slots").iter();
    let var_8: Vec<std::string::String> = aws_smithy_http::header::read_many_from_str(headers)?;
    let var_8: std::result::Result<Vec<_>, _> = var_8
        .iter()
        .map(|s| {
            aws_smithy_types::base64::decode(s)
                .map_err(|_| {
                    aws_smithy_http::header::ParseError::new_with_message("failed to decode base64")
                })
                .and_then(|bytes| {
                    String::from_utf8(bytes).map_err(|_| {
                        aws_smithy_http::header::ParseError::new_with_message(
                            "base64 encoded data was not valid utf-8",
                        )
                    })
                })
        })
        .collect();
    let var_8 = var_8?;
    if var_8.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_8.len()),
        ))
    } else {
        let mut var_8 = var_8;
        Ok(var_8.pop())
    }
}

pub fn deser_header_put_session_limit_exceeded_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}
