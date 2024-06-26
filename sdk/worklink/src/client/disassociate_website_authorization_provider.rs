// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DisassociateWebsiteAuthorizationProvider`](crate::operation::disassociate_website_authorization_provider::builders::DisassociateWebsiteAuthorizationProviderFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`fleet_arn(impl Into<String>)`](crate::operation::disassociate_website_authorization_provider::builders::DisassociateWebsiteAuthorizationProviderFluentBuilder::fleet_arn) / [`set_fleet_arn(Option<String>)`](crate::operation::disassociate_website_authorization_provider::builders::DisassociateWebsiteAuthorizationProviderFluentBuilder::set_fleet_arn):<br>required: **true**<br><p>The ARN of the fleet.</p><br>
    ///   - [`authorization_provider_id(impl Into<String>)`](crate::operation::disassociate_website_authorization_provider::builders::DisassociateWebsiteAuthorizationProviderFluentBuilder::authorization_provider_id) / [`set_authorization_provider_id(Option<String>)`](crate::operation::disassociate_website_authorization_provider::builders::DisassociateWebsiteAuthorizationProviderFluentBuilder::set_authorization_provider_id):<br>required: **true**<br><p>A unique identifier for the authorization provider.</p><br>
    /// - On success, responds with [`DisassociateWebsiteAuthorizationProviderOutput`](crate::operation::disassociate_website_authorization_provider::DisassociateWebsiteAuthorizationProviderOutput)
    /// - On failure, responds with [`SdkError<DisassociateWebsiteAuthorizationProviderError>`](crate::operation::disassociate_website_authorization_provider::DisassociateWebsiteAuthorizationProviderError)
    #[deprecated(note = "Amazon WorkLink is no longer supported. This will be removed in a future version of the SDK.")]
    pub fn disassociate_website_authorization_provider(
        &self,
    ) -> crate::operation::disassociate_website_authorization_provider::builders::DisassociateWebsiteAuthorizationProviderFluentBuilder {
        crate::operation::disassociate_website_authorization_provider::builders::DisassociateWebsiteAuthorizationProviderFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
