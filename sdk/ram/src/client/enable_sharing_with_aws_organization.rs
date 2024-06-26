// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`EnableSharingWithAwsOrganization`](crate::operation::enable_sharing_with_aws_organization::builders::EnableSharingWithAwsOrganizationFluentBuilder) operation.
    ///
    /// - The fluent builder takes no input, just [`send`](crate::operation::enable_sharing_with_aws_organization::builders::EnableSharingWithAwsOrganizationFluentBuilder::send) it.
    /// - On success, responds with [`EnableSharingWithAwsOrganizationOutput`](crate::operation::enable_sharing_with_aws_organization::EnableSharingWithAwsOrganizationOutput) with field(s):
    ///   - [`return_value(Option<bool>)`](crate::operation::enable_sharing_with_aws_organization::EnableSharingWithAwsOrganizationOutput::return_value): <p>A return value of <code>true</code> indicates that the request succeeded. A value of <code>false</code> indicates that the request failed.</p>
    /// - On failure, responds with [`SdkError<EnableSharingWithAwsOrganizationError>`](crate::operation::enable_sharing_with_aws_organization::EnableSharingWithAwsOrganizationError)
    pub fn enable_sharing_with_aws_organization(
        &self,
    ) -> crate::operation::enable_sharing_with_aws_organization::builders::EnableSharingWithAwsOrganizationFluentBuilder {
        crate::operation::enable_sharing_with_aws_organization::builders::EnableSharingWithAwsOrganizationFluentBuilder::new(self.handle.clone())
    }
}
