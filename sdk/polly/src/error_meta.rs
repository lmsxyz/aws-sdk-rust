// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>This engine is not compatible with the voice that you have designated.
    /// Choose a new voice that is compatible with the engine or change the engine
    /// and restart the operation.</p>
    EngineNotSupportedException(crate::error::EngineNotSupportedException),
    /// <p>Amazon Polly can't find the specified lexicon. Verify that the lexicon's
    /// name is spelled correctly, and then try again.</p>
    InvalidLexiconException(crate::error::InvalidLexiconException),
    /// <p>The NextToken is invalid. Verify that it's spelled correctly, and
    /// then try again.</p>
    InvalidNextTokenException(crate::error::InvalidNextTokenException),
    /// <p>The provided Amazon S3 bucket name is invalid. Please check your input
    /// with S3 bucket naming requirements and try again.</p>
    InvalidS3BucketException(crate::error::InvalidS3BucketException),
    /// <p>The provided Amazon S3 key prefix is invalid. Please provide a valid
    /// S3 object key name.</p>
    InvalidS3KeyException(crate::error::InvalidS3KeyException),
    /// <p>The specified sample rate is not valid.</p>
    InvalidSampleRateException(crate::error::InvalidSampleRateException),
    /// <p>The provided SNS topic ARN is invalid. Please provide a valid SNS
    /// topic ARN and try again.</p>
    InvalidSnsTopicArnException(crate::error::InvalidSnsTopicArnException),
    /// <p>The SSML you provided is invalid. Verify the SSML syntax, spelling
    /// of tags and values, and then try again.</p>
    InvalidSsmlException(crate::error::InvalidSsmlException),
    /// <p>The provided Task ID is not valid. Please provide a valid Task ID and
    /// try again.</p>
    InvalidTaskIdException(crate::error::InvalidTaskIdException),
    /// <p>The language specified is not currently supported by Amazon Polly in this
    /// capacity.</p>
    LanguageNotSupportedException(crate::error::LanguageNotSupportedException),
    /// <p>Amazon Polly can't find the specified lexicon. This could be caused by a
    /// lexicon that is missing, its name is misspelled or specifying a lexicon
    /// that is in a different region.</p>
    /// <p>Verify that the lexicon exists, is in the region (see <a>ListLexicons</a>) and that you spelled its name is spelled
    /// correctly. Then try again.</p>
    LexiconNotFoundException(crate::error::LexiconNotFoundException),
    /// <p>The maximum size of the specified lexicon would be exceeded by this
    /// operation.</p>
    LexiconSizeExceededException(crate::error::LexiconSizeExceededException),
    /// <p>Speech marks are not supported for the <code>OutputFormat</code>
    /// selected. Speech marks are only available for content in <code>json</code>
    /// format.</p>
    MarksNotSupportedForFormatException(crate::error::MarksNotSupportedForFormatException),
    /// <p>The maximum size of the lexeme would be exceeded by this
    /// operation.</p>
    MaxLexemeLengthExceededException(crate::error::MaxLexemeLengthExceededException),
    /// <p>The maximum number of lexicons would be exceeded by this
    /// operation.</p>
    MaxLexiconsNumberExceededException(crate::error::MaxLexiconsNumberExceededException),
    /// <p>An unknown condition has caused a service failure.</p>
    ServiceFailureException(crate::error::ServiceFailureException),
    /// <p>SSML speech marks are not supported for plain text-type
    /// input.</p>
    SsmlMarksNotSupportedForTextTypeException(
        crate::error::SsmlMarksNotSupportedForTextTypeException,
    ),
    /// <p>The Speech Synthesis task with requested Task ID cannot be
    /// found.</p>
    SynthesisTaskNotFoundException(crate::error::SynthesisTaskNotFoundException),
    /// <p>The value of the "Text" parameter is longer than the accepted
    /// limits. For the <code>SynthesizeSpeech</code> API, the limit for input
    /// text is a maximum of 6000 characters total, of which no more than 3000 can
    /// be billed characters. For the <code>StartSpeechSynthesisTask</code> API,
    /// the maximum is 200,000 characters, of which no more than 100,000 can be
    /// billed characters. SSML tags are not counted as billed
    /// characters.</p>
    TextLengthExceededException(crate::error::TextLengthExceededException),
    /// <p>The alphabet specified by the lexicon is not a supported alphabet.
    /// Valid values are <code>x-sampa</code> and <code>ipa</code>.</p>
    UnsupportedPlsAlphabetException(crate::error::UnsupportedPlsAlphabetException),
    /// <p>The language specified in the lexicon is unsupported. For a list of
    /// supported languages, see <a href="https://docs.aws.amazon.com/polly/latest/dg/API_LexiconAttributes.html">Lexicon Attributes</a>.</p>
    UnsupportedPlsLanguageException(crate::error::UnsupportedPlsLanguageException),
    /// An unhandled error occurred.
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::EngineNotSupportedException(inner) => inner.fmt(f),
            Error::InvalidLexiconException(inner) => inner.fmt(f),
            Error::InvalidNextTokenException(inner) => inner.fmt(f),
            Error::InvalidS3BucketException(inner) => inner.fmt(f),
            Error::InvalidS3KeyException(inner) => inner.fmt(f),
            Error::InvalidSampleRateException(inner) => inner.fmt(f),
            Error::InvalidSnsTopicArnException(inner) => inner.fmt(f),
            Error::InvalidSsmlException(inner) => inner.fmt(f),
            Error::InvalidTaskIdException(inner) => inner.fmt(f),
            Error::LanguageNotSupportedException(inner) => inner.fmt(f),
            Error::LexiconNotFoundException(inner) => inner.fmt(f),
            Error::LexiconSizeExceededException(inner) => inner.fmt(f),
            Error::MarksNotSupportedForFormatException(inner) => inner.fmt(f),
            Error::MaxLexemeLengthExceededException(inner) => inner.fmt(f),
            Error::MaxLexiconsNumberExceededException(inner) => inner.fmt(f),
            Error::ServiceFailureException(inner) => inner.fmt(f),
            Error::SsmlMarksNotSupportedForTextTypeException(inner) => inner.fmt(f),
            Error::SynthesisTaskNotFoundException(inner) => inner.fmt(f),
            Error::TextLengthExceededException(inner) => inner.fmt(f),
            Error::UnsupportedPlsAlphabetException(inner) => inner.fmt(f),
            Error::UnsupportedPlsLanguageException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteLexiconError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteLexiconError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteLexiconErrorKind::LexiconNotFoundException(inner) => {
                    Error::LexiconNotFoundException(inner)
                }
                crate::error::DeleteLexiconErrorKind::ServiceFailureException(inner) => {
                    Error::ServiceFailureException(inner)
                }
                crate::error::DeleteLexiconErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeVoicesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribeVoicesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeVoicesErrorKind::InvalidNextTokenException(inner) => {
                    Error::InvalidNextTokenException(inner)
                }
                crate::error::DescribeVoicesErrorKind::ServiceFailureException(inner) => {
                    Error::ServiceFailureException(inner)
                }
                crate::error::DescribeVoicesErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetLexiconError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetLexiconError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetLexiconErrorKind::LexiconNotFoundException(inner) => {
                    Error::LexiconNotFoundException(inner)
                }
                crate::error::GetLexiconErrorKind::ServiceFailureException(inner) => {
                    Error::ServiceFailureException(inner)
                }
                crate::error::GetLexiconErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetSpeechSynthesisTaskError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetSpeechSynthesisTaskError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetSpeechSynthesisTaskErrorKind::InvalidTaskIdException(inner) => {
                    Error::InvalidTaskIdException(inner)
                }
                crate::error::GetSpeechSynthesisTaskErrorKind::ServiceFailureException(inner) => {
                    Error::ServiceFailureException(inner)
                }
                crate::error::GetSpeechSynthesisTaskErrorKind::SynthesisTaskNotFoundException(
                    inner,
                ) => Error::SynthesisTaskNotFoundException(inner),
                crate::error::GetSpeechSynthesisTaskErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListLexiconsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListLexiconsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListLexiconsErrorKind::InvalidNextTokenException(inner) => {
                    Error::InvalidNextTokenException(inner)
                }
                crate::error::ListLexiconsErrorKind::ServiceFailureException(inner) => {
                    Error::ServiceFailureException(inner)
                }
                crate::error::ListLexiconsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListSpeechSynthesisTasksError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListSpeechSynthesisTasksError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListSpeechSynthesisTasksErrorKind::InvalidNextTokenException(
                    inner,
                ) => Error::InvalidNextTokenException(inner),
                crate::error::ListSpeechSynthesisTasksErrorKind::ServiceFailureException(inner) => {
                    Error::ServiceFailureException(inner)
                }
                crate::error::ListSpeechSynthesisTasksErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::PutLexiconError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::PutLexiconError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::PutLexiconErrorKind::InvalidLexiconException(inner) => {
                    Error::InvalidLexiconException(inner)
                }
                crate::error::PutLexiconErrorKind::LexiconSizeExceededException(inner) => {
                    Error::LexiconSizeExceededException(inner)
                }
                crate::error::PutLexiconErrorKind::MaxLexemeLengthExceededException(inner) => {
                    Error::MaxLexemeLengthExceededException(inner)
                }
                crate::error::PutLexiconErrorKind::MaxLexiconsNumberExceededException(inner) => {
                    Error::MaxLexiconsNumberExceededException(inner)
                }
                crate::error::PutLexiconErrorKind::ServiceFailureException(inner) => {
                    Error::ServiceFailureException(inner)
                }
                crate::error::PutLexiconErrorKind::UnsupportedPlsAlphabetException(inner) => {
                    Error::UnsupportedPlsAlphabetException(inner)
                }
                crate::error::PutLexiconErrorKind::UnsupportedPlsLanguageException(inner) => {
                    Error::UnsupportedPlsLanguageException(inner)
                }
                crate::error::PutLexiconErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::StartSpeechSynthesisTaskError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::StartSpeechSynthesisTaskError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::StartSpeechSynthesisTaskErrorKind::EngineNotSupportedException(inner) => Error::EngineNotSupportedException(inner),
                crate::error::StartSpeechSynthesisTaskErrorKind::InvalidS3BucketException(inner) => Error::InvalidS3BucketException(inner),
                crate::error::StartSpeechSynthesisTaskErrorKind::InvalidS3KeyException(inner) => Error::InvalidS3KeyException(inner),
                crate::error::StartSpeechSynthesisTaskErrorKind::InvalidSampleRateException(inner) => Error::InvalidSampleRateException(inner),
                crate::error::StartSpeechSynthesisTaskErrorKind::InvalidSnsTopicArnException(inner) => Error::InvalidSnsTopicArnException(inner),
                crate::error::StartSpeechSynthesisTaskErrorKind::InvalidSsmlException(inner) => Error::InvalidSsmlException(inner),
                crate::error::StartSpeechSynthesisTaskErrorKind::LanguageNotSupportedException(inner) => Error::LanguageNotSupportedException(inner),
                crate::error::StartSpeechSynthesisTaskErrorKind::LexiconNotFoundException(inner) => Error::LexiconNotFoundException(inner),
                crate::error::StartSpeechSynthesisTaskErrorKind::MarksNotSupportedForFormatException(inner) => Error::MarksNotSupportedForFormatException(inner),
                crate::error::StartSpeechSynthesisTaskErrorKind::ServiceFailureException(inner) => Error::ServiceFailureException(inner),
                crate::error::StartSpeechSynthesisTaskErrorKind::SsmlMarksNotSupportedForTextTypeException(inner) => Error::SsmlMarksNotSupportedForTextTypeException(inner),
                crate::error::StartSpeechSynthesisTaskErrorKind::TextLengthExceededException(inner) => Error::TextLengthExceededException(inner),
                crate::error::StartSpeechSynthesisTaskErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::SynthesizeSpeechError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::SynthesizeSpeechError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::SynthesizeSpeechErrorKind::EngineNotSupportedException(inner) => Error::EngineNotSupportedException(inner),
                crate::error::SynthesizeSpeechErrorKind::InvalidSampleRateException(inner) => Error::InvalidSampleRateException(inner),
                crate::error::SynthesizeSpeechErrorKind::InvalidSsmlException(inner) => Error::InvalidSsmlException(inner),
                crate::error::SynthesizeSpeechErrorKind::LanguageNotSupportedException(inner) => Error::LanguageNotSupportedException(inner),
                crate::error::SynthesizeSpeechErrorKind::LexiconNotFoundException(inner) => Error::LexiconNotFoundException(inner),
                crate::error::SynthesizeSpeechErrorKind::MarksNotSupportedForFormatException(inner) => Error::MarksNotSupportedForFormatException(inner),
                crate::error::SynthesizeSpeechErrorKind::ServiceFailureException(inner) => Error::ServiceFailureException(inner),
                crate::error::SynthesizeSpeechErrorKind::SsmlMarksNotSupportedForTextTypeException(inner) => Error::SsmlMarksNotSupportedForTextTypeException(inner),
                crate::error::SynthesizeSpeechErrorKind::TextLengthExceededException(inner) => Error::TextLengthExceededException(inner),
                crate::error::SynthesizeSpeechErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}
