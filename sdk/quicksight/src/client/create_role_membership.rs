// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateRoleMembership`](crate::operation::create_role_membership::builders::CreateRoleMembershipFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`member_name(impl Into<String>)`](crate::operation::create_role_membership::builders::CreateRoleMembershipFluentBuilder::member_name) / [`set_member_name(Option<String>)`](crate::operation::create_role_membership::builders::CreateRoleMembershipFluentBuilder::set_member_name):<br>required: **true**<br><p>The name of the group that you want to add to the role.</p><br>
    ///   - [`aws_account_id(impl Into<String>)`](crate::operation::create_role_membership::builders::CreateRoleMembershipFluentBuilder::aws_account_id) / [`set_aws_account_id(Option<String>)`](crate::operation::create_role_membership::builders::CreateRoleMembershipFluentBuilder::set_aws_account_id):<br>required: **true**<br><p>The ID for the Amazon Web Services account that you want to create a group in. The Amazon Web Services account ID that you provide must be the same Amazon Web Services account that contains your Amazon QuickSight account.</p><br>
    ///   - [`namespace(impl Into<String>)`](crate::operation::create_role_membership::builders::CreateRoleMembershipFluentBuilder::namespace) / [`set_namespace(Option<String>)`](crate::operation::create_role_membership::builders::CreateRoleMembershipFluentBuilder::set_namespace):<br>required: **true**<br><p>The namespace that the role belongs to.</p><br>
    ///   - [`role(Role)`](crate::operation::create_role_membership::builders::CreateRoleMembershipFluentBuilder::role) / [`set_role(Option<Role>)`](crate::operation::create_role_membership::builders::CreateRoleMembershipFluentBuilder::set_role):<br>required: **true**<br><p>The role that you want to add a group to.</p><br>
    /// - On success, responds with [`CreateRoleMembershipOutput`](crate::operation::create_role_membership::CreateRoleMembershipOutput) with field(s):
    ///   - [`request_id(Option<String>)`](crate::operation::create_role_membership::CreateRoleMembershipOutput::request_id): <p>The Amazon Web Services request ID for this operation.</p>
    ///   - [`status(i32)`](crate::operation::create_role_membership::CreateRoleMembershipOutput::status): <p>The HTTP status of the request.</p>
    /// - On failure, responds with [`SdkError<CreateRoleMembershipError>`](crate::operation::create_role_membership::CreateRoleMembershipError)
    pub fn create_role_membership(&self) -> crate::operation::create_role_membership::builders::CreateRoleMembershipFluentBuilder {
        crate::operation::create_role_membership::builders::CreateRoleMembershipFluentBuilder::new(self.handle.clone())
    }
}
