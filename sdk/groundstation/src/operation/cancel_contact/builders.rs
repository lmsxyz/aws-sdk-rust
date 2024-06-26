// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::cancel_contact::_cancel_contact_output::CancelContactOutputBuilder;

pub use crate::operation::cancel_contact::_cancel_contact_input::CancelContactInputBuilder;

impl crate::operation::cancel_contact::builders::CancelContactInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::cancel_contact::CancelContactOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::cancel_contact::CancelContactError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.cancel_contact();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CancelContact`.
///
/// <p>Cancels a contact with a specified contact ID.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CancelContactFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::cancel_contact::builders::CancelContactInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::cancel_contact::CancelContactOutput,
        crate::operation::cancel_contact::CancelContactError,
    > for CancelContactFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::cancel_contact::CancelContactOutput,
            crate::operation::cancel_contact::CancelContactError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CancelContactFluentBuilder {
    /// Creates a new `CancelContactFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CancelContact as a reference.
    pub fn as_input(&self) -> &crate::operation::cancel_contact::builders::CancelContactInputBuilder {
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
        crate::operation::cancel_contact::CancelContactOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::cancel_contact::CancelContactError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::cancel_contact::CancelContact::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::cancel_contact::CancelContact::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::cancel_contact::CancelContactOutput,
        crate::operation::cancel_contact::CancelContactError,
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
    /// <p>UUID of a contact.</p>
    pub fn contact_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.contact_id(input.into());
        self
    }
    /// <p>UUID of a contact.</p>
    pub fn set_contact_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_contact_id(input);
        self
    }
    /// <p>UUID of a contact.</p>
    pub fn get_contact_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_contact_id()
    }
}
