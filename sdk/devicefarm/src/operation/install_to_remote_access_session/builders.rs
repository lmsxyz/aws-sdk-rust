// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::install_to_remote_access_session::_install_to_remote_access_session_output::InstallToRemoteAccessSessionOutputBuilder;

pub use crate::operation::install_to_remote_access_session::_install_to_remote_access_session_input::InstallToRemoteAccessSessionInputBuilder;

impl crate::operation::install_to_remote_access_session::builders::InstallToRemoteAccessSessionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::install_to_remote_access_session::InstallToRemoteAccessSessionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::install_to_remote_access_session::InstallToRemoteAccessSessionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.install_to_remote_access_session();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `InstallToRemoteAccessSession`.
///
/// <p>Installs an application to the device in a remote access session. For Android applications, the file must be in .apk format. For iOS applications, the file must be in .ipa format.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct InstallToRemoteAccessSessionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::install_to_remote_access_session::builders::InstallToRemoteAccessSessionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::install_to_remote_access_session::InstallToRemoteAccessSessionOutput,
        crate::operation::install_to_remote_access_session::InstallToRemoteAccessSessionError,
    > for InstallToRemoteAccessSessionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::install_to_remote_access_session::InstallToRemoteAccessSessionOutput,
            crate::operation::install_to_remote_access_session::InstallToRemoteAccessSessionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl InstallToRemoteAccessSessionFluentBuilder {
    /// Creates a new `InstallToRemoteAccessSessionFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the InstallToRemoteAccessSession as a reference.
    pub fn as_input(&self) -> &crate::operation::install_to_remote_access_session::builders::InstallToRemoteAccessSessionInputBuilder {
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
        crate::operation::install_to_remote_access_session::InstallToRemoteAccessSessionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::install_to_remote_access_session::InstallToRemoteAccessSessionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::install_to_remote_access_session::InstallToRemoteAccessSession::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::install_to_remote_access_session::InstallToRemoteAccessSession::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::install_to_remote_access_session::InstallToRemoteAccessSessionOutput,
        crate::operation::install_to_remote_access_session::InstallToRemoteAccessSessionError,
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
    /// <p>The Amazon Resource Name (ARN) of the remote access session about which you are requesting information.</p>
    pub fn remote_access_session_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.remote_access_session_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the remote access session about which you are requesting information.</p>
    pub fn set_remote_access_session_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_remote_access_session_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the remote access session about which you are requesting information.</p>
    pub fn get_remote_access_session_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_remote_access_session_arn()
    }
    /// <p>The ARN of the app about which you are requesting information.</p>
    pub fn app_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.app_arn(input.into());
        self
    }
    /// <p>The ARN of the app about which you are requesting information.</p>
    pub fn set_app_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_app_arn(input);
        self
    }
    /// <p>The ARN of the app about which you are requesting information.</p>
    pub fn get_app_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_app_arn()
    }
}
