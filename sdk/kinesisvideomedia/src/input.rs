// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;
/// See [`GetMediaInput`](crate::input::GetMediaInput)
pub mod get_media_input {
    /// A builder for [`GetMediaInput`](crate::input::GetMediaInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) stream_name: std::option::Option<std::string::String>,
        pub(crate) stream_arn: std::option::Option<std::string::String>,
        pub(crate) start_selector: std::option::Option<crate::model::StartSelector>,
    }
    impl Builder {
        /// <p>The Kinesis video stream name from where you want to get the media content. If you
        /// don't specify the <code>streamName</code>, you must specify the
        /// <code>streamARN</code>.</p>
        pub fn stream_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.stream_name = Some(input.into());
            self
        }
        /// <p>The Kinesis video stream name from where you want to get the media content. If you
        /// don't specify the <code>streamName</code>, you must specify the
        /// <code>streamARN</code>.</p>
        pub fn set_stream_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.stream_name = input;
            self
        }
        /// <p>The ARN of the stream from where you want to get the media content. If you don't
        /// specify the <code>streamARN</code>, you must specify the <code>streamName</code>.</p>
        pub fn stream_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.stream_arn = Some(input.into());
            self
        }
        /// <p>The ARN of the stream from where you want to get the media content. If you don't
        /// specify the <code>streamARN</code>, you must specify the <code>streamName</code>.</p>
        pub fn set_stream_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.stream_arn = input;
            self
        }
        /// <p>Identifies the starting chunk to get from the specified stream. </p>
        pub fn start_selector(mut self, input: crate::model::StartSelector) -> Self {
            self.start_selector = Some(input);
            self
        }
        /// <p>Identifies the starting chunk to get from the specified stream. </p>
        pub fn set_start_selector(
            mut self,
            input: std::option::Option<crate::model::StartSelector>,
        ) -> Self {
            self.start_selector = input;
            self
        }
        /// Consumes the builder and constructs a [`GetMediaInput`](crate::input::GetMediaInput)
        pub fn build(
            self,
        ) -> std::result::Result<crate::input::GetMediaInput, aws_smithy_http::operation::BuildError>
        {
            Ok(crate::input::GetMediaInput {
                stream_name: self.stream_name,
                stream_arn: self.stream_arn,
                start_selector: self.start_selector,
            })
        }
    }
}
#[doc(hidden)]
pub type GetMediaInputOperationOutputAlias = crate::operation::GetMedia;
#[doc(hidden)]
pub type GetMediaInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl GetMediaInput {
    /// Consumes the builder and constructs an Operation<[`GetMedia`](crate::operation::GetMedia)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::GetMedia,
            aws_http::AwsErrorRetryPolicy,
        >,
        aws_smithy_http::operation::BuildError,
    > {
        fn uri_base(
            _input: &crate::input::GetMediaInput,
            output: &mut String,
        ) -> Result<(), aws_smithy_http::operation::BuildError> {
            write!(output, "/getMedia").expect("formatting should succeed");
            Ok(())
        }
        #[allow(clippy::unnecessary_wraps)]
        fn update_http_builder(
            input: &crate::input::GetMediaInput,
            builder: http::request::Builder,
        ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
        {
            let mut uri = String::new();
            uri_base(input, &mut uri)?;
            Ok(builder.method("POST").uri(uri))
        }
        #[allow(clippy::unnecessary_wraps)]
        fn request_builder_base(
            input: &crate::input::GetMediaInput,
        ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
        {
            let mut builder = update_http_builder(input, http::request::Builder::new())?;
            builder = aws_smithy_http::header::set_header_if_absent(
                builder,
                http::header::HeaderName::from_static("content-type"),
                "application/json",
            );
            Ok(builder)
        }
        let properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        let request = request_builder_base(&self)?;
        let body = crate::operation_ser::serialize_operation_crate_operation_get_media(&self)
            .map_err(|err| {
                aws_smithy_http::operation::BuildError::SerializationError(err.into())
            })?;
        let request = Self::assemble(request, body);
        #[allow(unused_mut)]
        let mut request = aws_smithy_http::operation::Request::from_parts(
            request.map(aws_smithy_http::body::SdkBody::from),
            properties,
        );
        request
            .properties_mut()
            .insert(aws_http::user_agent::AwsUserAgent::new_from_environment(
                crate::API_METADATA.clone(),
            ));
        #[allow(unused_mut)]
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        request.properties_mut().insert(signing_config);
        request
            .properties_mut()
            .insert(aws_types::SigningService::from_static(
                _config.signing_service(),
            ));
        aws_endpoint::set_endpoint_resolver(
            &mut request.properties_mut(),
            _config.endpoint_resolver.clone(),
        );
        if let Some(region) = &_config.region {
            request.properties_mut().insert(region.clone());
        }
        aws_http::auth::set_provider(
            &mut request.properties_mut(),
            _config.credentials_provider.clone(),
        );
        let op =
            aws_smithy_http::operation::Operation::new(request, crate::operation::GetMedia::new())
                .with_metadata(aws_smithy_http::operation::Metadata::new(
                    "GetMedia",
                    "kinesisvideomedia",
                ));
        let op = op.with_retry_policy(aws_http::AwsErrorRetryPolicy::new());
        Ok(op)
    }
    fn assemble(
        mut builder: http::request::Builder,
        body: aws_smithy_http::body::SdkBody,
    ) -> http::request::Request<aws_smithy_http::body::SdkBody> {
        if let Some(content_length) = body.content_length() {
            builder = aws_smithy_http::header::set_header_if_absent(
                builder,
                http::header::CONTENT_LENGTH,
                content_length,
            );
        }
        builder.body(body).expect("should be valid request")
    }
    /// Creates a new builder-style object to manufacture [`GetMediaInput`](crate::input::GetMediaInput)
    pub fn builder() -> crate::input::get_media_input::Builder {
        crate::input::get_media_input::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetMediaInput {
    /// <p>The Kinesis video stream name from where you want to get the media content. If you
    /// don't specify the <code>streamName</code>, you must specify the
    /// <code>streamARN</code>.</p>
    pub stream_name: std::option::Option<std::string::String>,
    /// <p>The ARN of the stream from where you want to get the media content. If you don't
    /// specify the <code>streamARN</code>, you must specify the <code>streamName</code>.</p>
    pub stream_arn: std::option::Option<std::string::String>,
    /// <p>Identifies the starting chunk to get from the specified stream. </p>
    pub start_selector: std::option::Option<crate::model::StartSelector>,
}
impl std::fmt::Debug for GetMediaInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetMediaInput");
        formatter.field("stream_name", &self.stream_name);
        formatter.field("stream_arn", &self.stream_arn);
        formatter.field("start_selector", &self.start_selector);
        formatter.finish()
    }
}
