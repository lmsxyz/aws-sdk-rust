// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::restart_channel_pipelines::_restart_channel_pipelines_output::RestartChannelPipelinesOutputBuilder;

pub use crate::operation::restart_channel_pipelines::_restart_channel_pipelines_input::RestartChannelPipelinesInputBuilder;

impl crate::operation::restart_channel_pipelines::builders::RestartChannelPipelinesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::restart_channel_pipelines::RestartChannelPipelinesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::restart_channel_pipelines::RestartChannelPipelinesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.restart_channel_pipelines();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `RestartChannelPipelines`.
///
/// Restart pipelines in one channel that is currently running.
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RestartChannelPipelinesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::restart_channel_pipelines::builders::RestartChannelPipelinesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::restart_channel_pipelines::RestartChannelPipelinesOutput,
        crate::operation::restart_channel_pipelines::RestartChannelPipelinesError,
    > for RestartChannelPipelinesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::restart_channel_pipelines::RestartChannelPipelinesOutput,
            crate::operation::restart_channel_pipelines::RestartChannelPipelinesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl RestartChannelPipelinesFluentBuilder {
    /// Creates a new `RestartChannelPipelinesFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the RestartChannelPipelines as a reference.
    pub fn as_input(&self) -> &crate::operation::restart_channel_pipelines::builders::RestartChannelPipelinesInputBuilder {
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
        crate::operation::restart_channel_pipelines::RestartChannelPipelinesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::restart_channel_pipelines::RestartChannelPipelinesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::restart_channel_pipelines::RestartChannelPipelines::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::restart_channel_pipelines::RestartChannelPipelines::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::restart_channel_pipelines::RestartChannelPipelinesOutput,
        crate::operation::restart_channel_pipelines::RestartChannelPipelinesError,
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
    /// ID of channel
    pub fn channel_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.channel_id(input.into());
        self
    }
    /// ID of channel
    pub fn set_channel_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_channel_id(input);
        self
    }
    /// ID of channel
    pub fn get_channel_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_channel_id()
    }
    ///
    /// Appends an item to `PipelineIds`.
    ///
    /// To override the contents of this collection use [`set_pipeline_ids`](Self::set_pipeline_ids).
    ///
    /// An array of pipelines to restart in this channel. Format PIPELINE_0 or PIPELINE_1.
    pub fn pipeline_ids(mut self, input: crate::types::ChannelPipelineIdToRestart) -> Self {
        self.inner = self.inner.pipeline_ids(input);
        self
    }
    /// An array of pipelines to restart in this channel. Format PIPELINE_0 or PIPELINE_1.
    pub fn set_pipeline_ids(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ChannelPipelineIdToRestart>>) -> Self {
        self.inner = self.inner.set_pipeline_ids(input);
        self
    }
    /// An array of pipelines to restart in this channel. Format PIPELINE_0 or PIPELINE_1.
    pub fn get_pipeline_ids(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ChannelPipelineIdToRestart>> {
        self.inner.get_pipeline_ids()
    }
}
