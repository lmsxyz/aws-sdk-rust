// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdatePrivateDnsNamespace`](crate::operation::update_private_dns_namespace::builders::UpdatePrivateDnsNamespaceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::operation::update_private_dns_namespace::builders::UpdatePrivateDnsNamespaceFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::update_private_dns_namespace::builders::UpdatePrivateDnsNamespaceFluentBuilder::set_id):<br>required: **true**<br><p>The ID of the namespace that you want to update.</p><br>
    ///   - [`updater_request_id(impl Into<String>)`](crate::operation::update_private_dns_namespace::builders::UpdatePrivateDnsNamespaceFluentBuilder::updater_request_id) / [`set_updater_request_id(Option<String>)`](crate::operation::update_private_dns_namespace::builders::UpdatePrivateDnsNamespaceFluentBuilder::set_updater_request_id):<br>required: **false**<br><p>A unique string that identifies the request and that allows failed <code>UpdatePrivateDnsNamespace</code> requests to be retried without the risk of running the operation twice. <code>UpdaterRequestId</code> can be any unique string (for example, a date/timestamp).</p><br>
    ///   - [`namespace(PrivateDnsNamespaceChange)`](crate::operation::update_private_dns_namespace::builders::UpdatePrivateDnsNamespaceFluentBuilder::namespace) / [`set_namespace(Option<PrivateDnsNamespaceChange>)`](crate::operation::update_private_dns_namespace::builders::UpdatePrivateDnsNamespaceFluentBuilder::set_namespace):<br>required: **true**<br><p>Updated properties for the private DNS namespace.</p><br>
    /// - On success, responds with [`UpdatePrivateDnsNamespaceOutput`](crate::operation::update_private_dns_namespace::UpdatePrivateDnsNamespaceOutput) with field(s):
    ///   - [`operation_id(Option<String>)`](crate::operation::update_private_dns_namespace::UpdatePrivateDnsNamespaceOutput::operation_id): <p>A value that you can use to determine whether the request completed successfully. To get the status of the operation, see <a href="https://docs.aws.amazon.com/cloud-map/latest/api/API_GetOperation.html">GetOperation</a>.</p>
    /// - On failure, responds with [`SdkError<UpdatePrivateDnsNamespaceError>`](crate::operation::update_private_dns_namespace::UpdatePrivateDnsNamespaceError)
    pub fn update_private_dns_namespace(&self) -> crate::operation::update_private_dns_namespace::builders::UpdatePrivateDnsNamespaceFluentBuilder {
        crate::operation::update_private_dns_namespace::builders::UpdatePrivateDnsNamespaceFluentBuilder::new(self.handle.clone())
    }
}
