// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_traffic_mirror_session::_delete_traffic_mirror_session_output::DeleteTrafficMirrorSessionOutputBuilder;

pub use crate::operation::delete_traffic_mirror_session::_delete_traffic_mirror_session_input::DeleteTrafficMirrorSessionInputBuilder;

impl crate::operation::delete_traffic_mirror_session::builders::DeleteTrafficMirrorSessionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_traffic_mirror_session::DeleteTrafficMirrorSessionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_traffic_mirror_session::DeleteTrafficMirrorSessionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_traffic_mirror_session();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteTrafficMirrorSession`.
///
/// <p>Deletes the specified Traffic Mirror session.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteTrafficMirrorSessionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_traffic_mirror_session::builders::DeleteTrafficMirrorSessionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_traffic_mirror_session::DeleteTrafficMirrorSessionOutput,
        crate::operation::delete_traffic_mirror_session::DeleteTrafficMirrorSessionError,
    > for DeleteTrafficMirrorSessionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_traffic_mirror_session::DeleteTrafficMirrorSessionOutput,
            crate::operation::delete_traffic_mirror_session::DeleteTrafficMirrorSessionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteTrafficMirrorSessionFluentBuilder {
    /// Creates a new `DeleteTrafficMirrorSessionFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteTrafficMirrorSession as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_traffic_mirror_session::builders::DeleteTrafficMirrorSessionInputBuilder {
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
        crate::operation::delete_traffic_mirror_session::DeleteTrafficMirrorSessionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_traffic_mirror_session::DeleteTrafficMirrorSessionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_traffic_mirror_session::DeleteTrafficMirrorSession::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_traffic_mirror_session::DeleteTrafficMirrorSession::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_traffic_mirror_session::DeleteTrafficMirrorSessionOutput,
        crate::operation::delete_traffic_mirror_session::DeleteTrafficMirrorSessionError,
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
    /// <p>The ID of the Traffic Mirror session.</p>
    pub fn traffic_mirror_session_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.traffic_mirror_session_id(input.into());
        self
    }
    /// <p>The ID of the Traffic Mirror session.</p>
    pub fn set_traffic_mirror_session_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_traffic_mirror_session_id(input);
        self
    }
    /// <p>The ID of the Traffic Mirror session.</p>
    pub fn get_traffic_mirror_session_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_traffic_mirror_session_id()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        self.inner.get_dry_run()
    }
}
