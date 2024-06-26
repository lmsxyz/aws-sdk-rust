// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::cancel_capacity_task::_cancel_capacity_task_output::CancelCapacityTaskOutputBuilder;

pub use crate::operation::cancel_capacity_task::_cancel_capacity_task_input::CancelCapacityTaskInputBuilder;

impl crate::operation::cancel_capacity_task::builders::CancelCapacityTaskInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::cancel_capacity_task::CancelCapacityTaskOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::cancel_capacity_task::CancelCapacityTaskError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.cancel_capacity_task();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CancelCapacityTask`.
///
/// <p>Cancels the capacity task.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CancelCapacityTaskFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::cancel_capacity_task::builders::CancelCapacityTaskInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::cancel_capacity_task::CancelCapacityTaskOutput,
        crate::operation::cancel_capacity_task::CancelCapacityTaskError,
    > for CancelCapacityTaskFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::cancel_capacity_task::CancelCapacityTaskOutput,
            crate::operation::cancel_capacity_task::CancelCapacityTaskError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CancelCapacityTaskFluentBuilder {
    /// Creates a new `CancelCapacityTaskFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CancelCapacityTask as a reference.
    pub fn as_input(&self) -> &crate::operation::cancel_capacity_task::builders::CancelCapacityTaskInputBuilder {
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
        crate::operation::cancel_capacity_task::CancelCapacityTaskOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::cancel_capacity_task::CancelCapacityTaskError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::cancel_capacity_task::CancelCapacityTask::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::cancel_capacity_task::CancelCapacityTask::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::cancel_capacity_task::CancelCapacityTaskOutput,
        crate::operation::cancel_capacity_task::CancelCapacityTaskError,
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
    /// <p>ID of the capacity task that you want to cancel.</p>
    pub fn capacity_task_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.capacity_task_id(input.into());
        self
    }
    /// <p>ID of the capacity task that you want to cancel.</p>
    pub fn set_capacity_task_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_capacity_task_id(input);
        self
    }
    /// <p>ID of the capacity task that you want to cancel.</p>
    pub fn get_capacity_task_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_capacity_task_id()
    }
    /// <p>ID or ARN of the Outpost associated with the capacity task that you want to cancel.</p>
    pub fn outpost_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.outpost_identifier(input.into());
        self
    }
    /// <p>ID or ARN of the Outpost associated with the capacity task that you want to cancel.</p>
    pub fn set_outpost_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_outpost_identifier(input);
        self
    }
    /// <p>ID or ARN of the Outpost associated with the capacity task that you want to cancel.</p>
    pub fn get_outpost_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_outpost_identifier()
    }
}
