// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `AssociateTeamMember`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`associate_team_member`](crate::client::Client::associate_team_member).
///
/// See [`crate::client::fluent_builders::AssociateTeamMember`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct AssociateTeamMember {
    _private: (),
}
impl AssociateTeamMember {
    /// Creates a new builder-style object to manufacture [`AssociateTeamMemberInput`](crate::input::AssociateTeamMemberInput)
    pub fn builder() -> crate::input::associate_team_member_input::Builder {
        crate::input::associate_team_member_input::Builder::default()
    }
    /// Creates a new `AssociateTeamMember` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for AssociateTeamMember {
    type Output = std::result::Result<
        crate::output::AssociateTeamMemberOutput,
        crate::error::AssociateTeamMemberError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_associate_team_member_error(response)
        } else {
            crate::operation_deser::parse_associate_team_member_response(response)
        }
    }
}

/// Operation shape for `CreateProject`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_project`](crate::client::Client::create_project).
///
/// See [`crate::client::fluent_builders::CreateProject`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateProject {
    _private: (),
}
impl CreateProject {
    /// Creates a new builder-style object to manufacture [`CreateProjectInput`](crate::input::CreateProjectInput)
    pub fn builder() -> crate::input::create_project_input::Builder {
        crate::input::create_project_input::Builder::default()
    }
    /// Creates a new `CreateProject` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateProject {
    type Output =
        std::result::Result<crate::output::CreateProjectOutput, crate::error::CreateProjectError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_project_error(response)
        } else {
            crate::operation_deser::parse_create_project_response(response)
        }
    }
}

/// Operation shape for `CreateUserProfile`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_user_profile`](crate::client::Client::create_user_profile).
///
/// See [`crate::client::fluent_builders::CreateUserProfile`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateUserProfile {
    _private: (),
}
impl CreateUserProfile {
    /// Creates a new builder-style object to manufacture [`CreateUserProfileInput`](crate::input::CreateUserProfileInput)
    pub fn builder() -> crate::input::create_user_profile_input::Builder {
        crate::input::create_user_profile_input::Builder::default()
    }
    /// Creates a new `CreateUserProfile` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateUserProfile {
    type Output = std::result::Result<
        crate::output::CreateUserProfileOutput,
        crate::error::CreateUserProfileError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_user_profile_error(response)
        } else {
            crate::operation_deser::parse_create_user_profile_response(response)
        }
    }
}

/// Operation shape for `DeleteProject`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_project`](crate::client::Client::delete_project).
///
/// See [`crate::client::fluent_builders::DeleteProject`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteProject {
    _private: (),
}
impl DeleteProject {
    /// Creates a new builder-style object to manufacture [`DeleteProjectInput`](crate::input::DeleteProjectInput)
    pub fn builder() -> crate::input::delete_project_input::Builder {
        crate::input::delete_project_input::Builder::default()
    }
    /// Creates a new `DeleteProject` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteProject {
    type Output =
        std::result::Result<crate::output::DeleteProjectOutput, crate::error::DeleteProjectError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_project_error(response)
        } else {
            crate::operation_deser::parse_delete_project_response(response)
        }
    }
}

/// Operation shape for `DeleteUserProfile`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_user_profile`](crate::client::Client::delete_user_profile).
///
/// See [`crate::client::fluent_builders::DeleteUserProfile`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteUserProfile {
    _private: (),
}
impl DeleteUserProfile {
    /// Creates a new builder-style object to manufacture [`DeleteUserProfileInput`](crate::input::DeleteUserProfileInput)
    pub fn builder() -> crate::input::delete_user_profile_input::Builder {
        crate::input::delete_user_profile_input::Builder::default()
    }
    /// Creates a new `DeleteUserProfile` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteUserProfile {
    type Output = std::result::Result<
        crate::output::DeleteUserProfileOutput,
        crate::error::DeleteUserProfileError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_user_profile_error(response)
        } else {
            crate::operation_deser::parse_delete_user_profile_response(response)
        }
    }
}

/// Operation shape for `DescribeProject`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_project`](crate::client::Client::describe_project).
///
/// See [`crate::client::fluent_builders::DescribeProject`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeProject {
    _private: (),
}
impl DescribeProject {
    /// Creates a new builder-style object to manufacture [`DescribeProjectInput`](crate::input::DescribeProjectInput)
    pub fn builder() -> crate::input::describe_project_input::Builder {
        crate::input::describe_project_input::Builder::default()
    }
    /// Creates a new `DescribeProject` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeProject {
    type Output = std::result::Result<
        crate::output::DescribeProjectOutput,
        crate::error::DescribeProjectError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_project_error(response)
        } else {
            crate::operation_deser::parse_describe_project_response(response)
        }
    }
}

/// Operation shape for `DescribeUserProfile`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_user_profile`](crate::client::Client::describe_user_profile).
///
/// See [`crate::client::fluent_builders::DescribeUserProfile`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeUserProfile {
    _private: (),
}
impl DescribeUserProfile {
    /// Creates a new builder-style object to manufacture [`DescribeUserProfileInput`](crate::input::DescribeUserProfileInput)
    pub fn builder() -> crate::input::describe_user_profile_input::Builder {
        crate::input::describe_user_profile_input::Builder::default()
    }
    /// Creates a new `DescribeUserProfile` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeUserProfile {
    type Output = std::result::Result<
        crate::output::DescribeUserProfileOutput,
        crate::error::DescribeUserProfileError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_user_profile_error(response)
        } else {
            crate::operation_deser::parse_describe_user_profile_response(response)
        }
    }
}

/// Operation shape for `DisassociateTeamMember`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`disassociate_team_member`](crate::client::Client::disassociate_team_member).
///
/// See [`crate::client::fluent_builders::DisassociateTeamMember`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DisassociateTeamMember {
    _private: (),
}
impl DisassociateTeamMember {
    /// Creates a new builder-style object to manufacture [`DisassociateTeamMemberInput`](crate::input::DisassociateTeamMemberInput)
    pub fn builder() -> crate::input::disassociate_team_member_input::Builder {
        crate::input::disassociate_team_member_input::Builder::default()
    }
    /// Creates a new `DisassociateTeamMember` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DisassociateTeamMember {
    type Output = std::result::Result<
        crate::output::DisassociateTeamMemberOutput,
        crate::error::DisassociateTeamMemberError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_disassociate_team_member_error(response)
        } else {
            crate::operation_deser::parse_disassociate_team_member_response(response)
        }
    }
}

/// Operation shape for `ListProjects`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_projects`](crate::client::Client::list_projects).
///
/// See [`crate::client::fluent_builders::ListProjects`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListProjects {
    _private: (),
}
impl ListProjects {
    /// Creates a new builder-style object to manufacture [`ListProjectsInput`](crate::input::ListProjectsInput)
    pub fn builder() -> crate::input::list_projects_input::Builder {
        crate::input::list_projects_input::Builder::default()
    }
    /// Creates a new `ListProjects` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListProjects {
    type Output =
        std::result::Result<crate::output::ListProjectsOutput, crate::error::ListProjectsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_projects_error(response)
        } else {
            crate::operation_deser::parse_list_projects_response(response)
        }
    }
}

/// Operation shape for `ListResources`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_resources`](crate::client::Client::list_resources).
///
/// See [`crate::client::fluent_builders::ListResources`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListResources {
    _private: (),
}
impl ListResources {
    /// Creates a new builder-style object to manufacture [`ListResourcesInput`](crate::input::ListResourcesInput)
    pub fn builder() -> crate::input::list_resources_input::Builder {
        crate::input::list_resources_input::Builder::default()
    }
    /// Creates a new `ListResources` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListResources {
    type Output =
        std::result::Result<crate::output::ListResourcesOutput, crate::error::ListResourcesError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_resources_error(response)
        } else {
            crate::operation_deser::parse_list_resources_response(response)
        }
    }
}

/// Operation shape for `ListTagsForProject`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_tags_for_project`](crate::client::Client::list_tags_for_project).
///
/// See [`crate::client::fluent_builders::ListTagsForProject`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListTagsForProject {
    _private: (),
}
impl ListTagsForProject {
    /// Creates a new builder-style object to manufacture [`ListTagsForProjectInput`](crate::input::ListTagsForProjectInput)
    pub fn builder() -> crate::input::list_tags_for_project_input::Builder {
        crate::input::list_tags_for_project_input::Builder::default()
    }
    /// Creates a new `ListTagsForProject` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTagsForProject {
    type Output = std::result::Result<
        crate::output::ListTagsForProjectOutput,
        crate::error::ListTagsForProjectError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_tags_for_project_error(response)
        } else {
            crate::operation_deser::parse_list_tags_for_project_response(response)
        }
    }
}

/// Operation shape for `ListTeamMembers`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_team_members`](crate::client::Client::list_team_members).
///
/// See [`crate::client::fluent_builders::ListTeamMembers`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListTeamMembers {
    _private: (),
}
impl ListTeamMembers {
    /// Creates a new builder-style object to manufacture [`ListTeamMembersInput`](crate::input::ListTeamMembersInput)
    pub fn builder() -> crate::input::list_team_members_input::Builder {
        crate::input::list_team_members_input::Builder::default()
    }
    /// Creates a new `ListTeamMembers` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTeamMembers {
    type Output = std::result::Result<
        crate::output::ListTeamMembersOutput,
        crate::error::ListTeamMembersError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_team_members_error(response)
        } else {
            crate::operation_deser::parse_list_team_members_response(response)
        }
    }
}

/// Operation shape for `ListUserProfiles`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_user_profiles`](crate::client::Client::list_user_profiles).
///
/// See [`crate::client::fluent_builders::ListUserProfiles`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListUserProfiles {
    _private: (),
}
impl ListUserProfiles {
    /// Creates a new builder-style object to manufacture [`ListUserProfilesInput`](crate::input::ListUserProfilesInput)
    pub fn builder() -> crate::input::list_user_profiles_input::Builder {
        crate::input::list_user_profiles_input::Builder::default()
    }
    /// Creates a new `ListUserProfiles` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListUserProfiles {
    type Output = std::result::Result<
        crate::output::ListUserProfilesOutput,
        crate::error::ListUserProfilesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_user_profiles_error(response)
        } else {
            crate::operation_deser::parse_list_user_profiles_response(response)
        }
    }
}

/// Operation shape for `TagProject`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`tag_project`](crate::client::Client::tag_project).
///
/// See [`crate::client::fluent_builders::TagProject`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct TagProject {
    _private: (),
}
impl TagProject {
    /// Creates a new builder-style object to manufacture [`TagProjectInput`](crate::input::TagProjectInput)
    pub fn builder() -> crate::input::tag_project_input::Builder {
        crate::input::tag_project_input::Builder::default()
    }
    /// Creates a new `TagProject` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for TagProject {
    type Output =
        std::result::Result<crate::output::TagProjectOutput, crate::error::TagProjectError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_tag_project_error(response)
        } else {
            crate::operation_deser::parse_tag_project_response(response)
        }
    }
}

/// Operation shape for `UntagProject`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`untag_project`](crate::client::Client::untag_project).
///
/// See [`crate::client::fluent_builders::UntagProject`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UntagProject {
    _private: (),
}
impl UntagProject {
    /// Creates a new builder-style object to manufacture [`UntagProjectInput`](crate::input::UntagProjectInput)
    pub fn builder() -> crate::input::untag_project_input::Builder {
        crate::input::untag_project_input::Builder::default()
    }
    /// Creates a new `UntagProject` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UntagProject {
    type Output =
        std::result::Result<crate::output::UntagProjectOutput, crate::error::UntagProjectError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_untag_project_error(response)
        } else {
            crate::operation_deser::parse_untag_project_response(response)
        }
    }
}

/// Operation shape for `UpdateProject`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_project`](crate::client::Client::update_project).
///
/// See [`crate::client::fluent_builders::UpdateProject`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateProject {
    _private: (),
}
impl UpdateProject {
    /// Creates a new builder-style object to manufacture [`UpdateProjectInput`](crate::input::UpdateProjectInput)
    pub fn builder() -> crate::input::update_project_input::Builder {
        crate::input::update_project_input::Builder::default()
    }
    /// Creates a new `UpdateProject` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateProject {
    type Output =
        std::result::Result<crate::output::UpdateProjectOutput, crate::error::UpdateProjectError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_project_error(response)
        } else {
            crate::operation_deser::parse_update_project_response(response)
        }
    }
}

/// Operation shape for `UpdateTeamMember`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_team_member`](crate::client::Client::update_team_member).
///
/// See [`crate::client::fluent_builders::UpdateTeamMember`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateTeamMember {
    _private: (),
}
impl UpdateTeamMember {
    /// Creates a new builder-style object to manufacture [`UpdateTeamMemberInput`](crate::input::UpdateTeamMemberInput)
    pub fn builder() -> crate::input::update_team_member_input::Builder {
        crate::input::update_team_member_input::Builder::default()
    }
    /// Creates a new `UpdateTeamMember` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateTeamMember {
    type Output = std::result::Result<
        crate::output::UpdateTeamMemberOutput,
        crate::error::UpdateTeamMemberError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_team_member_error(response)
        } else {
            crate::operation_deser::parse_update_team_member_response(response)
        }
    }
}

/// Operation shape for `UpdateUserProfile`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_user_profile`](crate::client::Client::update_user_profile).
///
/// See [`crate::client::fluent_builders::UpdateUserProfile`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateUserProfile {
    _private: (),
}
impl UpdateUserProfile {
    /// Creates a new builder-style object to manufacture [`UpdateUserProfileInput`](crate::input::UpdateUserProfileInput)
    pub fn builder() -> crate::input::update_user_profile_input::Builder {
        crate::input::update_user_profile_input::Builder::default()
    }
    /// Creates a new `UpdateUserProfile` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateUserProfile {
    type Output = std::result::Result<
        crate::output::UpdateUserProfileOutput,
        crate::error::UpdateUserProfileError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_user_profile_error(response)
        } else {
            crate::operation_deser::parse_update_user_profile_response(response)
        }
    }
}
