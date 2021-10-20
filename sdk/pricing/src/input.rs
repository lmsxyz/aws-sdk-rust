// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;
/// See [`DescribeServicesInput`](crate::input::DescribeServicesInput)
pub mod describe_services_input {
    /// A builder for [`DescribeServicesInput`](crate::input::DescribeServicesInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) service_code: std::option::Option<std::string::String>,
        pub(crate) format_version: std::option::Option<std::string::String>,
        pub(crate) next_token: std::option::Option<std::string::String>,
        pub(crate) max_results: std::option::Option<i32>,
    }
    impl Builder {
        /// <p>The code for the service whose information you want to retrieve, such as <code>AmazonEC2</code>.
        /// You can use
        /// the <code>ServiceCode</code> to filter the results in a <code>GetProducts</code> call.
        /// To retrieve a list of all services, leave this blank.</p>
        pub fn service_code(mut self, input: impl Into<std::string::String>) -> Self {
            self.service_code = Some(input.into());
            self
        }
        /// <p>The code for the service whose information you want to retrieve, such as <code>AmazonEC2</code>.
        /// You can use
        /// the <code>ServiceCode</code> to filter the results in a <code>GetProducts</code> call.
        /// To retrieve a list of all services, leave this blank.</p>
        pub fn set_service_code(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.service_code = input;
            self
        }
        /// <p>The format version that you want the response to be in.</p>
        /// <p>Valid values are: <code>aws_v1</code>
        /// </p>
        pub fn format_version(mut self, input: impl Into<std::string::String>) -> Self {
            self.format_version = Some(input.into());
            self
        }
        /// <p>The format version that you want the response to be in.</p>
        /// <p>Valid values are: <code>aws_v1</code>
        /// </p>
        pub fn set_format_version(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.format_version = input;
            self
        }
        /// <p>The pagination token that indicates the next set of results that you want to retrieve.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>The pagination token that indicates the next set of results that you want to retrieve.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// <p>The maximum number of results that you want returned in the response.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.max_results = Some(input);
            self
        }
        /// <p>The maximum number of results that you want returned in the response.</p>
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.max_results = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeServicesInput`](crate::input::DescribeServicesInput)
        pub fn build(
            self,
        ) -> std::result::Result<
            crate::input::DescribeServicesInput,
            aws_smithy_http::operation::BuildError,
        > {
            Ok(crate::input::DescribeServicesInput {
                service_code: self.service_code,
                format_version: self.format_version,
                next_token: self.next_token,
                max_results: self.max_results,
            })
        }
    }
}
#[doc(hidden)]
pub type DescribeServicesInputOperationOutputAlias = crate::operation::DescribeServices;
#[doc(hidden)]
pub type DescribeServicesInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl DescribeServicesInput {
    /// Consumes the builder and constructs an Operation<[`DescribeServices`](crate::operation::DescribeServices)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::DescribeServices,
            aws_http::AwsErrorRetryPolicy,
        >,
        aws_smithy_http::operation::BuildError,
    > {
        fn uri_base(
            _input: &crate::input::DescribeServicesInput,
            output: &mut String,
        ) -> Result<(), aws_smithy_http::operation::BuildError> {
            write!(output, "/").expect("formatting should succeed");
            Ok(())
        }
        #[allow(clippy::unnecessary_wraps)]
        fn update_http_builder(
            input: &crate::input::DescribeServicesInput,
            builder: http::request::Builder,
        ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
        {
            let mut uri = String::new();
            uri_base(input, &mut uri)?;
            Ok(builder.method("POST").uri(uri))
        }
        #[allow(clippy::unnecessary_wraps)]
        fn request_builder_base(
            input: &crate::input::DescribeServicesInput,
        ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
        {
            let mut builder = update_http_builder(input, http::request::Builder::new())?;
            builder = aws_smithy_http::header::set_header_if_absent(
                builder,
                http::header::HeaderName::from_static("content-type"),
                "application/x-amz-json-1.1",
            );
            builder = aws_smithy_http::header::set_header_if_absent(
                builder,
                http::header::HeaderName::from_static("x-amz-target"),
                "AWSPriceListService.DescribeServices",
            );
            Ok(builder)
        }
        let properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        let request = request_builder_base(&self)?;
        let body =
            crate::operation_ser::serialize_operation_crate_operation_describe_services(&self)
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
        let op = aws_smithy_http::operation::Operation::new(
            request,
            crate::operation::DescribeServices::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "DescribeServices",
            "pricing",
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
    /// Creates a new builder-style object to manufacture [`DescribeServicesInput`](crate::input::DescribeServicesInput)
    pub fn builder() -> crate::input::describe_services_input::Builder {
        crate::input::describe_services_input::Builder::default()
    }
}

/// See [`GetAttributeValuesInput`](crate::input::GetAttributeValuesInput)
pub mod get_attribute_values_input {
    /// A builder for [`GetAttributeValuesInput`](crate::input::GetAttributeValuesInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) service_code: std::option::Option<std::string::String>,
        pub(crate) attribute_name: std::option::Option<std::string::String>,
        pub(crate) next_token: std::option::Option<std::string::String>,
        pub(crate) max_results: std::option::Option<i32>,
    }
    impl Builder {
        /// <p>The service code for the service whose attributes you want to retrieve. For example, if you want
        /// the retrieve an EC2 attribute, use <code>AmazonEC2</code>.</p>
        pub fn service_code(mut self, input: impl Into<std::string::String>) -> Self {
            self.service_code = Some(input.into());
            self
        }
        /// <p>The service code for the service whose attributes you want to retrieve. For example, if you want
        /// the retrieve an EC2 attribute, use <code>AmazonEC2</code>.</p>
        pub fn set_service_code(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.service_code = input;
            self
        }
        /// <p>The name of the attribute that you want to retrieve the values for, such as <code>volumeType</code>.</p>
        pub fn attribute_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.attribute_name = Some(input.into());
            self
        }
        /// <p>The name of the attribute that you want to retrieve the values for, such as <code>volumeType</code>.</p>
        pub fn set_attribute_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.attribute_name = input;
            self
        }
        /// <p>The pagination token that indicates the next set of results that you want to retrieve.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>The pagination token that indicates the next set of results that you want to retrieve.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// <p>The maximum number of results to return in response.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.max_results = Some(input);
            self
        }
        /// <p>The maximum number of results to return in response.</p>
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.max_results = input;
            self
        }
        /// Consumes the builder and constructs a [`GetAttributeValuesInput`](crate::input::GetAttributeValuesInput)
        pub fn build(
            self,
        ) -> std::result::Result<
            crate::input::GetAttributeValuesInput,
            aws_smithy_http::operation::BuildError,
        > {
            Ok(crate::input::GetAttributeValuesInput {
                service_code: self.service_code,
                attribute_name: self.attribute_name,
                next_token: self.next_token,
                max_results: self.max_results,
            })
        }
    }
}
#[doc(hidden)]
pub type GetAttributeValuesInputOperationOutputAlias = crate::operation::GetAttributeValues;
#[doc(hidden)]
pub type GetAttributeValuesInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl GetAttributeValuesInput {
    /// Consumes the builder and constructs an Operation<[`GetAttributeValues`](crate::operation::GetAttributeValues)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::GetAttributeValues,
            aws_http::AwsErrorRetryPolicy,
        >,
        aws_smithy_http::operation::BuildError,
    > {
        fn uri_base(
            _input: &crate::input::GetAttributeValuesInput,
            output: &mut String,
        ) -> Result<(), aws_smithy_http::operation::BuildError> {
            write!(output, "/").expect("formatting should succeed");
            Ok(())
        }
        #[allow(clippy::unnecessary_wraps)]
        fn update_http_builder(
            input: &crate::input::GetAttributeValuesInput,
            builder: http::request::Builder,
        ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
        {
            let mut uri = String::new();
            uri_base(input, &mut uri)?;
            Ok(builder.method("POST").uri(uri))
        }
        #[allow(clippy::unnecessary_wraps)]
        fn request_builder_base(
            input: &crate::input::GetAttributeValuesInput,
        ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
        {
            let mut builder = update_http_builder(input, http::request::Builder::new())?;
            builder = aws_smithy_http::header::set_header_if_absent(
                builder,
                http::header::HeaderName::from_static("content-type"),
                "application/x-amz-json-1.1",
            );
            builder = aws_smithy_http::header::set_header_if_absent(
                builder,
                http::header::HeaderName::from_static("x-amz-target"),
                "AWSPriceListService.GetAttributeValues",
            );
            Ok(builder)
        }
        let properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        let request = request_builder_base(&self)?;
        let body =
            crate::operation_ser::serialize_operation_crate_operation_get_attribute_values(&self)
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
        let op = aws_smithy_http::operation::Operation::new(
            request,
            crate::operation::GetAttributeValues::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "GetAttributeValues",
            "pricing",
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
    /// Creates a new builder-style object to manufacture [`GetAttributeValuesInput`](crate::input::GetAttributeValuesInput)
    pub fn builder() -> crate::input::get_attribute_values_input::Builder {
        crate::input::get_attribute_values_input::Builder::default()
    }
}

/// See [`GetProductsInput`](crate::input::GetProductsInput)
pub mod get_products_input {
    /// A builder for [`GetProductsInput`](crate::input::GetProductsInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) service_code: std::option::Option<std::string::String>,
        pub(crate) filters: std::option::Option<std::vec::Vec<crate::model::Filter>>,
        pub(crate) format_version: std::option::Option<std::string::String>,
        pub(crate) next_token: std::option::Option<std::string::String>,
        pub(crate) max_results: std::option::Option<i32>,
    }
    impl Builder {
        /// <p>The code for the service whose products you want to retrieve. </p>
        pub fn service_code(mut self, input: impl Into<std::string::String>) -> Self {
            self.service_code = Some(input.into());
            self
        }
        /// <p>The code for the service whose products you want to retrieve. </p>
        pub fn set_service_code(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.service_code = input;
            self
        }
        /// Appends an item to `filters`.
        ///
        /// To override the contents of this collection use [`set_filters`](Self::set_filters).
        ///
        /// <p>The list of filters that limit the returned products. only products that match all filters
        /// are returned.</p>
        pub fn filters(mut self, input: impl Into<crate::model::Filter>) -> Self {
            let mut v = self.filters.unwrap_or_default();
            v.push(input.into());
            self.filters = Some(v);
            self
        }
        /// <p>The list of filters that limit the returned products. only products that match all filters
        /// are returned.</p>
        pub fn set_filters(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Filter>>,
        ) -> Self {
            self.filters = input;
            self
        }
        /// <p>The format version that you want the response to be in.</p>
        /// <p>Valid values are: <code>aws_v1</code>
        /// </p>
        pub fn format_version(mut self, input: impl Into<std::string::String>) -> Self {
            self.format_version = Some(input.into());
            self
        }
        /// <p>The format version that you want the response to be in.</p>
        /// <p>Valid values are: <code>aws_v1</code>
        /// </p>
        pub fn set_format_version(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.format_version = input;
            self
        }
        /// <p>The pagination token that indicates the next set of results that you want to retrieve.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>The pagination token that indicates the next set of results that you want to retrieve.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// <p>The maximum number of results to return in the response.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.max_results = Some(input);
            self
        }
        /// <p>The maximum number of results to return in the response.</p>
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.max_results = input;
            self
        }
        /// Consumes the builder and constructs a [`GetProductsInput`](crate::input::GetProductsInput)
        pub fn build(
            self,
        ) -> std::result::Result<
            crate::input::GetProductsInput,
            aws_smithy_http::operation::BuildError,
        > {
            Ok(crate::input::GetProductsInput {
                service_code: self.service_code,
                filters: self.filters,
                format_version: self.format_version,
                next_token: self.next_token,
                max_results: self.max_results,
            })
        }
    }
}
#[doc(hidden)]
pub type GetProductsInputOperationOutputAlias = crate::operation::GetProducts;
#[doc(hidden)]
pub type GetProductsInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl GetProductsInput {
    /// Consumes the builder and constructs an Operation<[`GetProducts`](crate::operation::GetProducts)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::GetProducts,
            aws_http::AwsErrorRetryPolicy,
        >,
        aws_smithy_http::operation::BuildError,
    > {
        fn uri_base(
            _input: &crate::input::GetProductsInput,
            output: &mut String,
        ) -> Result<(), aws_smithy_http::operation::BuildError> {
            write!(output, "/").expect("formatting should succeed");
            Ok(())
        }
        #[allow(clippy::unnecessary_wraps)]
        fn update_http_builder(
            input: &crate::input::GetProductsInput,
            builder: http::request::Builder,
        ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
        {
            let mut uri = String::new();
            uri_base(input, &mut uri)?;
            Ok(builder.method("POST").uri(uri))
        }
        #[allow(clippy::unnecessary_wraps)]
        fn request_builder_base(
            input: &crate::input::GetProductsInput,
        ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
        {
            let mut builder = update_http_builder(input, http::request::Builder::new())?;
            builder = aws_smithy_http::header::set_header_if_absent(
                builder,
                http::header::HeaderName::from_static("content-type"),
                "application/x-amz-json-1.1",
            );
            builder = aws_smithy_http::header::set_header_if_absent(
                builder,
                http::header::HeaderName::from_static("x-amz-target"),
                "AWSPriceListService.GetProducts",
            );
            Ok(builder)
        }
        let properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        let request = request_builder_base(&self)?;
        let body = crate::operation_ser::serialize_operation_crate_operation_get_products(&self)
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
        let op = aws_smithy_http::operation::Operation::new(
            request,
            crate::operation::GetProducts::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "GetProducts",
            "pricing",
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
    /// Creates a new builder-style object to manufacture [`GetProductsInput`](crate::input::GetProductsInput)
    pub fn builder() -> crate::input::get_products_input::Builder {
        crate::input::get_products_input::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetProductsInput {
    /// <p>The code for the service whose products you want to retrieve. </p>
    pub service_code: std::option::Option<std::string::String>,
    /// <p>The list of filters that limit the returned products. only products that match all filters
    /// are returned.</p>
    pub filters: std::option::Option<std::vec::Vec<crate::model::Filter>>,
    /// <p>The format version that you want the response to be in.</p>
    /// <p>Valid values are: <code>aws_v1</code>
    /// </p>
    pub format_version: std::option::Option<std::string::String>,
    /// <p>The pagination token that indicates the next set of results that you want to retrieve.</p>
    pub next_token: std::option::Option<std::string::String>,
    /// <p>The maximum number of results to return in the response.</p>
    pub max_results: std::option::Option<i32>,
}
impl std::fmt::Debug for GetProductsInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetProductsInput");
        formatter.field("service_code", &self.service_code);
        formatter.field("filters", &self.filters);
        formatter.field("format_version", &self.format_version);
        formatter.field("next_token", &self.next_token);
        formatter.field("max_results", &self.max_results);
        formatter.finish()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetAttributeValuesInput {
    /// <p>The service code for the service whose attributes you want to retrieve. For example, if you want
    /// the retrieve an EC2 attribute, use <code>AmazonEC2</code>.</p>
    pub service_code: std::option::Option<std::string::String>,
    /// <p>The name of the attribute that you want to retrieve the values for, such as <code>volumeType</code>.</p>
    pub attribute_name: std::option::Option<std::string::String>,
    /// <p>The pagination token that indicates the next set of results that you want to retrieve.</p>
    pub next_token: std::option::Option<std::string::String>,
    /// <p>The maximum number of results to return in response.</p>
    pub max_results: std::option::Option<i32>,
}
impl std::fmt::Debug for GetAttributeValuesInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetAttributeValuesInput");
        formatter.field("service_code", &self.service_code);
        formatter.field("attribute_name", &self.attribute_name);
        formatter.field("next_token", &self.next_token);
        formatter.field("max_results", &self.max_results);
        formatter.finish()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DescribeServicesInput {
    /// <p>The code for the service whose information you want to retrieve, such as <code>AmazonEC2</code>.
    /// You can use
    /// the <code>ServiceCode</code> to filter the results in a <code>GetProducts</code> call.
    /// To retrieve a list of all services, leave this blank.</p>
    pub service_code: std::option::Option<std::string::String>,
    /// <p>The format version that you want the response to be in.</p>
    /// <p>Valid values are: <code>aws_v1</code>
    /// </p>
    pub format_version: std::option::Option<std::string::String>,
    /// <p>The pagination token that indicates the next set of results that you want to retrieve.</p>
    pub next_token: std::option::Option<std::string::String>,
    /// <p>The maximum number of results that you want returned in the response.</p>
    pub max_results: std::option::Option<i32>,
}
impl std::fmt::Debug for DescribeServicesInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeServicesInput");
        formatter.field("service_code", &self.service_code);
        formatter.field("format_version", &self.format_version);
        formatter.field("next_token", &self.next_token);
        formatter.field("max_results", &self.max_results);
        formatter.finish()
    }
}
