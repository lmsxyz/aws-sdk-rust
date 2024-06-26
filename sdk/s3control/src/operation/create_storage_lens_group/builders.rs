// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_storage_lens_group::_create_storage_lens_group_output::CreateStorageLensGroupOutputBuilder;

pub use crate::operation::create_storage_lens_group::_create_storage_lens_group_input::CreateStorageLensGroupInputBuilder;

impl crate::operation::create_storage_lens_group::builders::CreateStorageLensGroupInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_storage_lens_group::CreateStorageLensGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_storage_lens_group::CreateStorageLensGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_storage_lens_group();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateStorageLensGroup`.
///
/// <p>Creates a new S3 Storage Lens group and associates it with the specified Amazon Web Services account ID. An S3 Storage Lens group is a custom grouping of objects based on prefix, suffix, object tags, object size, object age, or a combination of these filters. For each Storage Lens group that you’ve created, you can also optionally add Amazon Web Services resource tags. For more information about S3 Storage Lens groups, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/storage-lens-groups-overview.html">Working with S3 Storage Lens groups</a>.</p>
/// <p>To use this operation, you must have the permission to perform the <code>s3:CreateStorageLensGroup</code> action. If you’re trying to create a Storage Lens group with Amazon Web Services resource tags, you must also have permission to perform the <code>s3:TagResource</code> action. For more information about the required Storage Lens Groups permissions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/storage_lens_iam_permissions.html#storage_lens_groups_permissions">Setting account permissions to use S3 Storage Lens groups</a>.</p>
/// <p>For information about Storage Lens groups errors, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/ErrorResponses.html#S3LensErrorCodeList">List of Amazon S3 Storage Lens error codes</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateStorageLensGroupFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_storage_lens_group::builders::CreateStorageLensGroupInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_storage_lens_group::CreateStorageLensGroupOutput,
        crate::operation::create_storage_lens_group::CreateStorageLensGroupError,
    > for CreateStorageLensGroupFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_storage_lens_group::CreateStorageLensGroupOutput,
            crate::operation::create_storage_lens_group::CreateStorageLensGroupError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateStorageLensGroupFluentBuilder {
    /// Creates a new `CreateStorageLensGroupFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateStorageLensGroup as a reference.
    pub fn as_input(&self) -> &crate::operation::create_storage_lens_group::builders::CreateStorageLensGroupInputBuilder {
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
        crate::operation::create_storage_lens_group::CreateStorageLensGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_storage_lens_group::CreateStorageLensGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_storage_lens_group::CreateStorageLensGroup::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_storage_lens_group::CreateStorageLensGroup::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_storage_lens_group::CreateStorageLensGroupOutput,
        crate::operation::create_storage_lens_group::CreateStorageLensGroupError,
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
    /// <p>The Amazon Web Services account ID that the Storage Lens group is created from and associated with.</p>
    pub fn account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.account_id(input.into());
        self
    }
    /// <p>The Amazon Web Services account ID that the Storage Lens group is created from and associated with.</p>
    pub fn set_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_account_id(input);
        self
    }
    /// <p>The Amazon Web Services account ID that the Storage Lens group is created from and associated with.</p>
    pub fn get_account_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_account_id()
    }
    /// <p>The Storage Lens group configuration.</p>
    pub fn storage_lens_group(mut self, input: crate::types::StorageLensGroup) -> Self {
        self.inner = self.inner.storage_lens_group(input);
        self
    }
    /// <p>The Storage Lens group configuration.</p>
    pub fn set_storage_lens_group(mut self, input: ::std::option::Option<crate::types::StorageLensGroup>) -> Self {
        self.inner = self.inner.set_storage_lens_group(input);
        self
    }
    /// <p>The Storage Lens group configuration.</p>
    pub fn get_storage_lens_group(&self) -> &::std::option::Option<crate::types::StorageLensGroup> {
        self.inner.get_storage_lens_group()
    }
    ///
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The Amazon Web Services resource tags that you're adding to your Storage Lens group. This parameter is optional.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>The Amazon Web Services resource tags that you're adding to your Storage Lens group. This parameter is optional.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The Amazon Web Services resource tags that you're adding to your Storage Lens group. This parameter is optional.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
}
