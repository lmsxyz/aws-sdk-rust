// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_event_bridge_rule_templates::_list_event_bridge_rule_templates_output::ListEventBridgeRuleTemplatesOutputBuilder;

pub use crate::operation::list_event_bridge_rule_templates::_list_event_bridge_rule_templates_input::ListEventBridgeRuleTemplatesInputBuilder;

impl crate::operation::list_event_bridge_rule_templates::builders::ListEventBridgeRuleTemplatesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_event_bridge_rule_templates::ListEventBridgeRuleTemplatesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_event_bridge_rule_templates::ListEventBridgeRuleTemplatesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_event_bridge_rule_templates();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListEventBridgeRuleTemplates`.
///
/// Lists eventbridge rule templates.
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListEventBridgeRuleTemplatesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_event_bridge_rule_templates::builders::ListEventBridgeRuleTemplatesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_event_bridge_rule_templates::ListEventBridgeRuleTemplatesOutput,
        crate::operation::list_event_bridge_rule_templates::ListEventBridgeRuleTemplatesError,
    > for ListEventBridgeRuleTemplatesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_event_bridge_rule_templates::ListEventBridgeRuleTemplatesOutput,
            crate::operation::list_event_bridge_rule_templates::ListEventBridgeRuleTemplatesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListEventBridgeRuleTemplatesFluentBuilder {
    /// Creates a new `ListEventBridgeRuleTemplatesFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListEventBridgeRuleTemplates as a reference.
    pub fn as_input(&self) -> &crate::operation::list_event_bridge_rule_templates::builders::ListEventBridgeRuleTemplatesInputBuilder {
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
        crate::operation::list_event_bridge_rule_templates::ListEventBridgeRuleTemplatesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_event_bridge_rule_templates::ListEventBridgeRuleTemplatesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_event_bridge_rule_templates::ListEventBridgeRuleTemplates::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_event_bridge_rule_templates::ListEventBridgeRuleTemplates::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_event_bridge_rule_templates::ListEventBridgeRuleTemplatesOutput,
        crate::operation::list_event_bridge_rule_templates::ListEventBridgeRuleTemplatesError,
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_event_bridge_rule_templates::paginator::ListEventBridgeRuleTemplatesPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_event_bridge_rule_templates::paginator::ListEventBridgeRuleTemplatesPaginator {
        crate::operation::list_event_bridge_rule_templates::paginator::ListEventBridgeRuleTemplatesPaginator::new(self.handle, self.inner)
    }
    /// An eventbridge rule template group's identifier. Can be either be its id or current name.
    pub fn group_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.group_identifier(input.into());
        self
    }
    /// An eventbridge rule template group's identifier. Can be either be its id or current name.
    pub fn set_group_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_group_identifier(input);
        self
    }
    /// An eventbridge rule template group's identifier. Can be either be its id or current name.
    pub fn get_group_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_group_identifier()
    }
    /// Placeholder documentation for MaxResults
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// Placeholder documentation for MaxResults
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// Placeholder documentation for MaxResults
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// A token used to retrieve the next set of results in paginated list responses.
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// A token used to retrieve the next set of results in paginated list responses.
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// A token used to retrieve the next set of results in paginated list responses.
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// A signal map's identifier. Can be either be its id or current name.
    pub fn signal_map_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.signal_map_identifier(input.into());
        self
    }
    /// A signal map's identifier. Can be either be its id or current name.
    pub fn set_signal_map_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_signal_map_identifier(input);
        self
    }
    /// A signal map's identifier. Can be either be its id or current name.
    pub fn get_signal_map_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_signal_map_identifier()
    }
}
