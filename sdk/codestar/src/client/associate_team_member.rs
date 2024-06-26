// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AssociateTeamMember`](crate::operation::associate_team_member::builders::AssociateTeamMemberFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`project_id(impl Into<String>)`](crate::operation::associate_team_member::builders::AssociateTeamMemberFluentBuilder::project_id) / [`set_project_id(Option<String>)`](crate::operation::associate_team_member::builders::AssociateTeamMemberFluentBuilder::set_project_id):<br>required: **true**<br><p>The ID of the project to which you will add the IAM user.</p><br>
    ///   - [`client_request_token(impl Into<String>)`](crate::operation::associate_team_member::builders::AssociateTeamMemberFluentBuilder::client_request_token) / [`set_client_request_token(Option<String>)`](crate::operation::associate_team_member::builders::AssociateTeamMemberFluentBuilder::set_client_request_token):<br>required: **false**<br><p>A user- or system-generated token that identifies the entity that requested the team member association to the project. This token can be used to repeat the request.</p><br>
    ///   - [`user_arn(impl Into<String>)`](crate::operation::associate_team_member::builders::AssociateTeamMemberFluentBuilder::user_arn) / [`set_user_arn(Option<String>)`](crate::operation::associate_team_member::builders::AssociateTeamMemberFluentBuilder::set_user_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) for the IAM user you want to add to the AWS CodeStar project.</p><br>
    ///   - [`project_role(impl Into<String>)`](crate::operation::associate_team_member::builders::AssociateTeamMemberFluentBuilder::project_role) / [`set_project_role(Option<String>)`](crate::operation::associate_team_member::builders::AssociateTeamMemberFluentBuilder::set_project_role):<br>required: **true**<br><p>The AWS CodeStar project role that will apply to this user. This role determines what actions a user can take in an AWS CodeStar project.</p><br>
    ///   - [`remote_access_allowed(bool)`](crate::operation::associate_team_member::builders::AssociateTeamMemberFluentBuilder::remote_access_allowed) / [`set_remote_access_allowed(Option<bool>)`](crate::operation::associate_team_member::builders::AssociateTeamMemberFluentBuilder::set_remote_access_allowed):<br>required: **false**<br><p>Whether the team member is allowed to use an SSH public/private key pair to remotely access project resources, for example Amazon EC2 instances.</p><br>
    /// - On success, responds with [`AssociateTeamMemberOutput`](crate::operation::associate_team_member::AssociateTeamMemberOutput) with field(s):
    ///   - [`client_request_token(Option<String>)`](crate::operation::associate_team_member::AssociateTeamMemberOutput::client_request_token): <p>The user- or system-generated token from the initial request that can be used to repeat the request.</p>
    /// - On failure, responds with [`SdkError<AssociateTeamMemberError>`](crate::operation::associate_team_member::AssociateTeamMemberError)
    pub fn associate_team_member(&self) -> crate::operation::associate_team_member::builders::AssociateTeamMemberFluentBuilder {
        crate::operation::associate_team_member::builders::AssociateTeamMemberFluentBuilder::new(self.handle.clone())
    }
}
