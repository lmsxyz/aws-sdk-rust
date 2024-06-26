// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_instance_tpm_ek_pub::_get_instance_tpm_ek_pub_output::GetInstanceTpmEkPubOutputBuilder;

pub use crate::operation::get_instance_tpm_ek_pub::_get_instance_tpm_ek_pub_input::GetInstanceTpmEkPubInputBuilder;

impl crate::operation::get_instance_tpm_ek_pub::builders::GetInstanceTpmEkPubInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_instance_tpm_ek_pub::GetInstanceTpmEkPubOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_instance_tpm_ek_pub::GetInstanceTpmEkPubError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_instance_tpm_ek_pub();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetInstanceTpmEkPub`.
///
/// <p>Gets the public endorsement key associated with the Nitro Trusted Platform Module (NitroTPM) for the specified instance.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetInstanceTpmEkPubFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_instance_tpm_ek_pub::builders::GetInstanceTpmEkPubInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_instance_tpm_ek_pub::GetInstanceTpmEkPubOutput,
        crate::operation::get_instance_tpm_ek_pub::GetInstanceTpmEkPubError,
    > for GetInstanceTpmEkPubFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_instance_tpm_ek_pub::GetInstanceTpmEkPubOutput,
            crate::operation::get_instance_tpm_ek_pub::GetInstanceTpmEkPubError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetInstanceTpmEkPubFluentBuilder {
    /// Creates a new `GetInstanceTpmEkPubFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetInstanceTpmEkPub as a reference.
    pub fn as_input(&self) -> &crate::operation::get_instance_tpm_ek_pub::builders::GetInstanceTpmEkPubInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_instance_tpm_ek_pub::GetInstanceTpmEkPubOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_instance_tpm_ek_pub::GetInstanceTpmEkPubError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_instance_tpm_ek_pub::GetInstanceTpmEkPub::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_instance_tpm_ek_pub::GetInstanceTpmEkPub::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_instance_tpm_ek_pub::GetInstanceTpmEkPubOutput,
        crate::operation::get_instance_tpm_ek_pub::GetInstanceTpmEkPubError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
        self.set_config_override(::std::option::Option::Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The ID of the instance for which to get the public endorsement key.</p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_id(input.into());
        self
    }
    /// <p>The ID of the instance for which to get the public endorsement key.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_instance_id(input);
        self
    }
    /// <p>The ID of the instance for which to get the public endorsement key.</p>
    pub fn get_instance_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_instance_id()
    }
    /// <p>The required public endorsement key type.</p>
    pub fn key_type(mut self, input: crate::types::EkPubKeyType) -> Self {
        self.inner = self.inner.key_type(input);
        self
    }
    /// <p>The required public endorsement key type.</p>
    pub fn set_key_type(mut self, input: ::std::option::Option<crate::types::EkPubKeyType>) -> Self {
        self.inner = self.inner.set_key_type(input);
        self
    }
    /// <p>The required public endorsement key type.</p>
    pub fn get_key_type(&self) -> &::std::option::Option<crate::types::EkPubKeyType> {
        self.inner.get_key_type()
    }
    /// <p>The required public endorsement key format. Specify <code>der</code> for a DER-encoded public key that is compatible with OpenSSL. Specify <code>tpmt</code> for a TPM 2.0 format that is compatible with tpm2-tools. The returned key is base64 encoded.</p>
    pub fn key_format(mut self, input: crate::types::EkPubKeyFormat) -> Self {
        self.inner = self.inner.key_format(input);
        self
    }
    /// <p>The required public endorsement key format. Specify <code>der</code> for a DER-encoded public key that is compatible with OpenSSL. Specify <code>tpmt</code> for a TPM 2.0 format that is compatible with tpm2-tools. The returned key is base64 encoded.</p>
    pub fn set_key_format(mut self, input: ::std::option::Option<crate::types::EkPubKeyFormat>) -> Self {
        self.inner = self.inner.set_key_format(input);
        self
    }
    /// <p>The required public endorsement key format. Specify <code>der</code> for a DER-encoded public key that is compatible with OpenSSL. Specify <code>tpmt</code> for a TPM 2.0 format that is compatible with tpm2-tools. The returned key is base64 encoded.</p>
    pub fn get_key_format(&self) -> &::std::option::Option<crate::types::EkPubKeyFormat> {
        self.inner.get_key_format()
    }
    /// <p>Specify this parameter to verify whether the request will succeed, without actually making the request. If the request will succeed, the response is <code>DryRunOperation</code>. Otherwise, the response is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Specify this parameter to verify whether the request will succeed, without actually making the request. If the request will succeed, the response is <code>DryRunOperation</code>. Otherwise, the response is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Specify this parameter to verify whether the request will succeed, without actually making the request. If the request will succeed, the response is <code>DryRunOperation</code>. Otherwise, the response is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        self.inner.get_dry_run()
    }
}
