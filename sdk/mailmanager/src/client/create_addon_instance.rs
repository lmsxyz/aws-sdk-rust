// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateAddonInstance`](crate::operation::create_addon_instance::builders::CreateAddonInstanceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`client_token(impl Into<String>)`](crate::operation::create_addon_instance::builders::CreateAddonInstanceFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::create_addon_instance::builders::CreateAddonInstanceFluentBuilder::set_client_token):<br>required: **false**<br><p>A unique token that Amazon SES uses to recognize subsequent retries of the same request.</p><br>
    ///   - [`addon_subscription_id(impl Into<String>)`](crate::operation::create_addon_instance::builders::CreateAddonInstanceFluentBuilder::addon_subscription_id) / [`set_addon_subscription_id(Option<String>)`](crate::operation::create_addon_instance::builders::CreateAddonInstanceFluentBuilder::set_addon_subscription_id):<br>required: **true**<br><p>The unique ID of a previously created subscription that an Add On instance is created for. You can only have one instance per subscription.</p><br>
    ///   - [`tags(Tag)`](crate::operation::create_addon_instance::builders::CreateAddonInstanceFluentBuilder::tags) / [`set_tags(Option<Vec::<Tag>>)`](crate::operation::create_addon_instance::builders::CreateAddonInstanceFluentBuilder::set_tags):<br>required: **false**<br><p>The tags used to organize, track, or control access for the resource. For example, { "tags": {"key1":"value1", "key2":"value2"} }.</p><br>
    /// - On success, responds with [`CreateAddonInstanceOutput`](crate::operation::create_addon_instance::CreateAddonInstanceOutput) with field(s):
    ///   - [`addon_instance_id(String)`](crate::operation::create_addon_instance::CreateAddonInstanceOutput::addon_instance_id): <p>The unique ID of the Add On instance created by this API.</p>
    /// - On failure, responds with [`SdkError<CreateAddonInstanceError>`](crate::operation::create_addon_instance::CreateAddonInstanceError)
    pub fn create_addon_instance(&self) -> crate::operation::create_addon_instance::builders::CreateAddonInstanceFluentBuilder {
        crate::operation::create_addon_instance::builders::CreateAddonInstanceFluentBuilder::new(self.handle.clone())
    }
}
