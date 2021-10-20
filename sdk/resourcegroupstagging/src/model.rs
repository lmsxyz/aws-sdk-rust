// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Information about the errors that are returned for each failed resource. This
/// information can include <code>InternalServiceException</code> and
/// <code>InvalidParameterException</code> errors. It can also include any valid error
/// code returned by the AWS service that hosts the resource that the ARN key
/// represents.</p>
/// <p>The following are common error codes that you might receive from other AWS
/// services:</p>
/// <ul>
/// <li>
/// <p>
/// <b>InternalServiceException</b> – This can
/// mean that the Resource Groups Tagging API didn't receive a response from another
/// AWS service. It can also mean the the resource type in the request is not
/// supported by the Resource Groups Tagging API. In these cases, it's safe to retry
/// the request and then call <a href="http://docs.aws.amazon.com/resourcegroupstagging/latest/APIReference/API_GetResources.html">GetResources</a> to verify the changes.</p>
/// </li>
/// <li>
/// <p>
/// <b>AccessDeniedException</b> – This can mean
/// that you need permission to calling tagging operations in the AWS service that
/// contains the resource. For example, to use the Resource Groups Tagging API to
/// tag a CloudWatch alarm resource, you need permission to call <a href="http://docs.aws.amazon.com/resourcegroupstagging/latest/APIReference/API_TagResources.html">
/// <code>TagResources</code>
/// </a>
/// <i>and</i>
/// <a href="http://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_TagResource.html">
/// <code>TagResource</code>
/// </a> in the CloudWatch API. </p>
/// </li>
/// </ul>
/// <p>For more information on errors that are generated from other AWS services, see the
/// documentation for that service. </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct FailureInfo {
    /// <p>The HTTP status code of the common error.</p>
    pub status_code: i32,
    /// <p>The code of the common error. Valid values include
    /// <code>InternalServiceException</code>, <code>InvalidParameterException</code>, and
    /// any valid error code returned by the AWS service that hosts the resource that you want
    /// to tag.</p>
    pub error_code: std::option::Option<crate::model::ErrorCode>,
    /// <p>The message of the common error.</p>
    pub error_message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for FailureInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("FailureInfo");
        formatter.field("status_code", &self.status_code);
        formatter.field("error_code", &self.error_code);
        formatter.field("error_message", &self.error_message);
        formatter.finish()
    }
}
/// See [`FailureInfo`](crate::model::FailureInfo)
pub mod failure_info {
    /// A builder for [`FailureInfo`](crate::model::FailureInfo)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) status_code: std::option::Option<i32>,
        pub(crate) error_code: std::option::Option<crate::model::ErrorCode>,
        pub(crate) error_message: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The HTTP status code of the common error.</p>
        pub fn status_code(mut self, input: i32) -> Self {
            self.status_code = Some(input);
            self
        }
        /// <p>The HTTP status code of the common error.</p>
        pub fn set_status_code(mut self, input: std::option::Option<i32>) -> Self {
            self.status_code = input;
            self
        }
        /// <p>The code of the common error. Valid values include
        /// <code>InternalServiceException</code>, <code>InvalidParameterException</code>, and
        /// any valid error code returned by the AWS service that hosts the resource that you want
        /// to tag.</p>
        pub fn error_code(mut self, input: crate::model::ErrorCode) -> Self {
            self.error_code = Some(input);
            self
        }
        /// <p>The code of the common error. Valid values include
        /// <code>InternalServiceException</code>, <code>InvalidParameterException</code>, and
        /// any valid error code returned by the AWS service that hosts the resource that you want
        /// to tag.</p>
        pub fn set_error_code(
            mut self,
            input: std::option::Option<crate::model::ErrorCode>,
        ) -> Self {
            self.error_code = input;
            self
        }
        /// <p>The message of the common error.</p>
        pub fn error_message(mut self, input: impl Into<std::string::String>) -> Self {
            self.error_message = Some(input.into());
            self
        }
        /// <p>The message of the common error.</p>
        pub fn set_error_message(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.error_message = input;
            self
        }
        /// Consumes the builder and constructs a [`FailureInfo`](crate::model::FailureInfo)
        pub fn build(self) -> crate::model::FailureInfo {
            crate::model::FailureInfo {
                status_code: self.status_code.unwrap_or_default(),
                error_code: self.error_code,
                error_message: self.error_message,
            }
        }
    }
}
impl FailureInfo {
    /// Creates a new builder-style object to manufacture [`FailureInfo`](crate::model::FailureInfo)
    pub fn builder() -> crate::model::failure_info::Builder {
        crate::model::failure_info::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum ErrorCode {
    #[allow(missing_docs)] // documentation missing in model
    InternalServiceException,
    #[allow(missing_docs)] // documentation missing in model
    InvalidParameterException,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for ErrorCode {
    fn from(s: &str) -> Self {
        match s {
            "InternalServiceException" => ErrorCode::InternalServiceException,
            "InvalidParameterException" => ErrorCode::InvalidParameterException,
            other => ErrorCode::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for ErrorCode {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(ErrorCode::from(s))
    }
}
impl ErrorCode {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            ErrorCode::InternalServiceException => "InternalServiceException",
            ErrorCode::InvalidParameterException => "InvalidParameterException",
            ErrorCode::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["InternalServiceException", "InvalidParameterException"]
    }
}
impl AsRef<str> for ErrorCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>A list of resource ARNs and the tags (keys and values) that are associated with
/// each.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ResourceTagMapping {
    /// <p>The ARN of the resource.</p>
    pub resource_arn: std::option::Option<std::string::String>,
    /// <p>The tags that have been applied to one or more AWS resources.</p>
    pub tags: std::option::Option<std::vec::Vec<crate::model::Tag>>,
    /// <p>Information that shows whether a resource is compliant with the effective tag policy,
    /// including details on any noncompliant tag keys.</p>
    pub compliance_details: std::option::Option<crate::model::ComplianceDetails>,
}
impl std::fmt::Debug for ResourceTagMapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ResourceTagMapping");
        formatter.field("resource_arn", &self.resource_arn);
        formatter.field("tags", &self.tags);
        formatter.field("compliance_details", &self.compliance_details);
        formatter.finish()
    }
}
/// See [`ResourceTagMapping`](crate::model::ResourceTagMapping)
pub mod resource_tag_mapping {
    /// A builder for [`ResourceTagMapping`](crate::model::ResourceTagMapping)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) resource_arn: std::option::Option<std::string::String>,
        pub(crate) tags: std::option::Option<std::vec::Vec<crate::model::Tag>>,
        pub(crate) compliance_details: std::option::Option<crate::model::ComplianceDetails>,
    }
    impl Builder {
        /// <p>The ARN of the resource.</p>
        pub fn resource_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.resource_arn = Some(input.into());
            self
        }
        /// <p>The ARN of the resource.</p>
        pub fn set_resource_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.resource_arn = input;
            self
        }
        /// Appends an item to `tags`.
        ///
        /// To override the contents of this collection use [`set_tags`](Self::set_tags).
        ///
        /// <p>The tags that have been applied to one or more AWS resources.</p>
        pub fn tags(mut self, input: impl Into<crate::model::Tag>) -> Self {
            let mut v = self.tags.unwrap_or_default();
            v.push(input.into());
            self.tags = Some(v);
            self
        }
        /// <p>The tags that have been applied to one or more AWS resources.</p>
        pub fn set_tags(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Tag>>,
        ) -> Self {
            self.tags = input;
            self
        }
        /// <p>Information that shows whether a resource is compliant with the effective tag policy,
        /// including details on any noncompliant tag keys.</p>
        pub fn compliance_details(mut self, input: crate::model::ComplianceDetails) -> Self {
            self.compliance_details = Some(input);
            self
        }
        /// <p>Information that shows whether a resource is compliant with the effective tag policy,
        /// including details on any noncompliant tag keys.</p>
        pub fn set_compliance_details(
            mut self,
            input: std::option::Option<crate::model::ComplianceDetails>,
        ) -> Self {
            self.compliance_details = input;
            self
        }
        /// Consumes the builder and constructs a [`ResourceTagMapping`](crate::model::ResourceTagMapping)
        pub fn build(self) -> crate::model::ResourceTagMapping {
            crate::model::ResourceTagMapping {
                resource_arn: self.resource_arn,
                tags: self.tags,
                compliance_details: self.compliance_details,
            }
        }
    }
}
impl ResourceTagMapping {
    /// Creates a new builder-style object to manufacture [`ResourceTagMapping`](crate::model::ResourceTagMapping)
    pub fn builder() -> crate::model::resource_tag_mapping::Builder {
        crate::model::resource_tag_mapping::Builder::default()
    }
}

/// <p>Information that shows whether a resource is compliant with the effective tag policy,
/// including details on any noncompliant tag keys.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ComplianceDetails {
    /// <p>These tag keys on the resource are noncompliant with the effective tag policy.</p>
    pub noncompliant_keys: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>These are keys defined in the effective policy that are on the resource with either
    /// incorrect case treatment or noncompliant values. </p>
    pub keys_with_noncompliant_values: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>Whether a resource is compliant with the effective tag policy.</p>
    pub compliance_status: std::option::Option<bool>,
}
impl std::fmt::Debug for ComplianceDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ComplianceDetails");
        formatter.field("noncompliant_keys", &self.noncompliant_keys);
        formatter.field(
            "keys_with_noncompliant_values",
            &self.keys_with_noncompliant_values,
        );
        formatter.field("compliance_status", &self.compliance_status);
        formatter.finish()
    }
}
/// See [`ComplianceDetails`](crate::model::ComplianceDetails)
pub mod compliance_details {
    /// A builder for [`ComplianceDetails`](crate::model::ComplianceDetails)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) noncompliant_keys: std::option::Option<std::vec::Vec<std::string::String>>,
        pub(crate) keys_with_noncompliant_values:
            std::option::Option<std::vec::Vec<std::string::String>>,
        pub(crate) compliance_status: std::option::Option<bool>,
    }
    impl Builder {
        /// Appends an item to `noncompliant_keys`.
        ///
        /// To override the contents of this collection use [`set_noncompliant_keys`](Self::set_noncompliant_keys).
        ///
        /// <p>These tag keys on the resource are noncompliant with the effective tag policy.</p>
        pub fn noncompliant_keys(mut self, input: impl Into<std::string::String>) -> Self {
            let mut v = self.noncompliant_keys.unwrap_or_default();
            v.push(input.into());
            self.noncompliant_keys = Some(v);
            self
        }
        /// <p>These tag keys on the resource are noncompliant with the effective tag policy.</p>
        pub fn set_noncompliant_keys(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.noncompliant_keys = input;
            self
        }
        /// Appends an item to `keys_with_noncompliant_values`.
        ///
        /// To override the contents of this collection use [`set_keys_with_noncompliant_values`](Self::set_keys_with_noncompliant_values).
        ///
        /// <p>These are keys defined in the effective policy that are on the resource with either
        /// incorrect case treatment or noncompliant values. </p>
        pub fn keys_with_noncompliant_values(
            mut self,
            input: impl Into<std::string::String>,
        ) -> Self {
            let mut v = self.keys_with_noncompliant_values.unwrap_or_default();
            v.push(input.into());
            self.keys_with_noncompliant_values = Some(v);
            self
        }
        /// <p>These are keys defined in the effective policy that are on the resource with either
        /// incorrect case treatment or noncompliant values. </p>
        pub fn set_keys_with_noncompliant_values(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.keys_with_noncompliant_values = input;
            self
        }
        /// <p>Whether a resource is compliant with the effective tag policy.</p>
        pub fn compliance_status(mut self, input: bool) -> Self {
            self.compliance_status = Some(input);
            self
        }
        /// <p>Whether a resource is compliant with the effective tag policy.</p>
        pub fn set_compliance_status(mut self, input: std::option::Option<bool>) -> Self {
            self.compliance_status = input;
            self
        }
        /// Consumes the builder and constructs a [`ComplianceDetails`](crate::model::ComplianceDetails)
        pub fn build(self) -> crate::model::ComplianceDetails {
            crate::model::ComplianceDetails {
                noncompliant_keys: self.noncompliant_keys,
                keys_with_noncompliant_values: self.keys_with_noncompliant_values,
                compliance_status: self.compliance_status,
            }
        }
    }
}
impl ComplianceDetails {
    /// Creates a new builder-style object to manufacture [`ComplianceDetails`](crate::model::ComplianceDetails)
    pub fn builder() -> crate::model::compliance_details::Builder {
        crate::model::compliance_details::Builder::default()
    }
}

/// <p>The metadata that you apply to AWS resources to help you categorize and organize
/// them. Each tag consists of a key and a value, both of which you define. For more
/// information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging AWS
/// Resources</a> in the <i>AWS General Reference</i>.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct Tag {
    /// <p>One part of a key-value pair that makes up a tag. A key is a general label that acts like a category for more specific tag values.</p>
    pub key: std::option::Option<std::string::String>,
    /// <p>One part of a key-value pair that make up a tag. A value acts as a descriptor within a tag category (key). The value can be empty or null.</p>
    pub value: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Tag");
        formatter.field("key", &self.key);
        formatter.field("value", &self.value);
        formatter.finish()
    }
}
/// See [`Tag`](crate::model::Tag)
pub mod tag {
    /// A builder for [`Tag`](crate::model::Tag)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) key: std::option::Option<std::string::String>,
        pub(crate) value: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>One part of a key-value pair that makes up a tag. A key is a general label that acts like a category for more specific tag values.</p>
        pub fn key(mut self, input: impl Into<std::string::String>) -> Self {
            self.key = Some(input.into());
            self
        }
        /// <p>One part of a key-value pair that makes up a tag. A key is a general label that acts like a category for more specific tag values.</p>
        pub fn set_key(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.key = input;
            self
        }
        /// <p>One part of a key-value pair that make up a tag. A value acts as a descriptor within a tag category (key). The value can be empty or null.</p>
        pub fn value(mut self, input: impl Into<std::string::String>) -> Self {
            self.value = Some(input.into());
            self
        }
        /// <p>One part of a key-value pair that make up a tag. A value acts as a descriptor within a tag category (key). The value can be empty or null.</p>
        pub fn set_value(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.value = input;
            self
        }
        /// Consumes the builder and constructs a [`Tag`](crate::model::Tag)
        pub fn build(self) -> crate::model::Tag {
            crate::model::Tag {
                key: self.key,
                value: self.value,
            }
        }
    }
}
impl Tag {
    /// Creates a new builder-style object to manufacture [`Tag`](crate::model::Tag)
    pub fn builder() -> crate::model::tag::Builder {
        crate::model::tag::Builder::default()
    }
}

/// <p>A list of tags (keys and values) that are used to specify the associated
/// resources.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct TagFilter {
    /// <p>One part of a key-value pair that makes up a tag. A key is a general label that acts like a category for more specific tag values.</p>
    pub key: std::option::Option<std::string::String>,
    /// <p>One part of a key-value pair that make up a tag. A value acts as a descriptor within a tag category (key). The value can be empty or null.</p>
    pub values: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl std::fmt::Debug for TagFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("TagFilter");
        formatter.field("key", &self.key);
        formatter.field("values", &self.values);
        formatter.finish()
    }
}
/// See [`TagFilter`](crate::model::TagFilter)
pub mod tag_filter {
    /// A builder for [`TagFilter`](crate::model::TagFilter)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) key: std::option::Option<std::string::String>,
        pub(crate) values: std::option::Option<std::vec::Vec<std::string::String>>,
    }
    impl Builder {
        /// <p>One part of a key-value pair that makes up a tag. A key is a general label that acts like a category for more specific tag values.</p>
        pub fn key(mut self, input: impl Into<std::string::String>) -> Self {
            self.key = Some(input.into());
            self
        }
        /// <p>One part of a key-value pair that makes up a tag. A key is a general label that acts like a category for more specific tag values.</p>
        pub fn set_key(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.key = input;
            self
        }
        /// Appends an item to `values`.
        ///
        /// To override the contents of this collection use [`set_values`](Self::set_values).
        ///
        /// <p>One part of a key-value pair that make up a tag. A value acts as a descriptor within a tag category (key). The value can be empty or null.</p>
        pub fn values(mut self, input: impl Into<std::string::String>) -> Self {
            let mut v = self.values.unwrap_or_default();
            v.push(input.into());
            self.values = Some(v);
            self
        }
        /// <p>One part of a key-value pair that make up a tag. A value acts as a descriptor within a tag category (key). The value can be empty or null.</p>
        pub fn set_values(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.values = input;
            self
        }
        /// Consumes the builder and constructs a [`TagFilter`](crate::model::TagFilter)
        pub fn build(self) -> crate::model::TagFilter {
            crate::model::TagFilter {
                key: self.key,
                values: self.values,
            }
        }
    }
}
impl TagFilter {
    /// Creates a new builder-style object to manufacture [`TagFilter`](crate::model::TagFilter)
    pub fn builder() -> crate::model::tag_filter::Builder {
        crate::model::tag_filter::Builder::default()
    }
}

/// <p>A count of noncompliant resources.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct Summary {
    /// <p>The timestamp that shows when this summary was generated in this Region. </p>
    pub last_updated: std::option::Option<std::string::String>,
    /// <p>The account identifier or the root identifier of the organization. If you don't know
    /// the root ID, you can call the AWS Organizations <a href="http://docs.aws.amazon.com/organizations/latest/APIReference/API_ListRoots.html">ListRoots</a> API.</p>
    pub target_id: std::option::Option<std::string::String>,
    /// <p>Whether the target is an account, an OU, or the organization root.</p>
    pub target_id_type: std::option::Option<crate::model::TargetIdType>,
    /// <p>The AWS Region that the summary applies to.</p>
    pub region: std::option::Option<std::string::String>,
    /// <p>The AWS resource type.</p>
    pub resource_type: std::option::Option<std::string::String>,
    /// <p>The count of noncompliant resources.</p>
    pub non_compliant_resources: i64,
}
impl std::fmt::Debug for Summary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Summary");
        formatter.field("last_updated", &self.last_updated);
        formatter.field("target_id", &self.target_id);
        formatter.field("target_id_type", &self.target_id_type);
        formatter.field("region", &self.region);
        formatter.field("resource_type", &self.resource_type);
        formatter.field("non_compliant_resources", &self.non_compliant_resources);
        formatter.finish()
    }
}
/// See [`Summary`](crate::model::Summary)
pub mod summary {
    /// A builder for [`Summary`](crate::model::Summary)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) last_updated: std::option::Option<std::string::String>,
        pub(crate) target_id: std::option::Option<std::string::String>,
        pub(crate) target_id_type: std::option::Option<crate::model::TargetIdType>,
        pub(crate) region: std::option::Option<std::string::String>,
        pub(crate) resource_type: std::option::Option<std::string::String>,
        pub(crate) non_compliant_resources: std::option::Option<i64>,
    }
    impl Builder {
        /// <p>The timestamp that shows when this summary was generated in this Region. </p>
        pub fn last_updated(mut self, input: impl Into<std::string::String>) -> Self {
            self.last_updated = Some(input.into());
            self
        }
        /// <p>The timestamp that shows when this summary was generated in this Region. </p>
        pub fn set_last_updated(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.last_updated = input;
            self
        }
        /// <p>The account identifier or the root identifier of the organization. If you don't know
        /// the root ID, you can call the AWS Organizations <a href="http://docs.aws.amazon.com/organizations/latest/APIReference/API_ListRoots.html">ListRoots</a> API.</p>
        pub fn target_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.target_id = Some(input.into());
            self
        }
        /// <p>The account identifier or the root identifier of the organization. If you don't know
        /// the root ID, you can call the AWS Organizations <a href="http://docs.aws.amazon.com/organizations/latest/APIReference/API_ListRoots.html">ListRoots</a> API.</p>
        pub fn set_target_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.target_id = input;
            self
        }
        /// <p>Whether the target is an account, an OU, or the organization root.</p>
        pub fn target_id_type(mut self, input: crate::model::TargetIdType) -> Self {
            self.target_id_type = Some(input);
            self
        }
        /// <p>Whether the target is an account, an OU, or the organization root.</p>
        pub fn set_target_id_type(
            mut self,
            input: std::option::Option<crate::model::TargetIdType>,
        ) -> Self {
            self.target_id_type = input;
            self
        }
        /// <p>The AWS Region that the summary applies to.</p>
        pub fn region(mut self, input: impl Into<std::string::String>) -> Self {
            self.region = Some(input.into());
            self
        }
        /// <p>The AWS Region that the summary applies to.</p>
        pub fn set_region(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.region = input;
            self
        }
        /// <p>The AWS resource type.</p>
        pub fn resource_type(mut self, input: impl Into<std::string::String>) -> Self {
            self.resource_type = Some(input.into());
            self
        }
        /// <p>The AWS resource type.</p>
        pub fn set_resource_type(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.resource_type = input;
            self
        }
        /// <p>The count of noncompliant resources.</p>
        pub fn non_compliant_resources(mut self, input: i64) -> Self {
            self.non_compliant_resources = Some(input);
            self
        }
        /// <p>The count of noncompliant resources.</p>
        pub fn set_non_compliant_resources(mut self, input: std::option::Option<i64>) -> Self {
            self.non_compliant_resources = input;
            self
        }
        /// Consumes the builder and constructs a [`Summary`](crate::model::Summary)
        pub fn build(self) -> crate::model::Summary {
            crate::model::Summary {
                last_updated: self.last_updated,
                target_id: self.target_id,
                target_id_type: self.target_id_type,
                region: self.region,
                resource_type: self.resource_type,
                non_compliant_resources: self.non_compliant_resources.unwrap_or_default(),
            }
        }
    }
}
impl Summary {
    /// Creates a new builder-style object to manufacture [`Summary`](crate::model::Summary)
    pub fn builder() -> crate::model::summary::Builder {
        crate::model::summary::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum TargetIdType {
    #[allow(missing_docs)] // documentation missing in model
    Account,
    #[allow(missing_docs)] // documentation missing in model
    Ou,
    #[allow(missing_docs)] // documentation missing in model
    Root,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for TargetIdType {
    fn from(s: &str) -> Self {
        match s {
            "ACCOUNT" => TargetIdType::Account,
            "OU" => TargetIdType::Ou,
            "ROOT" => TargetIdType::Root,
            other => TargetIdType::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for TargetIdType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(TargetIdType::from(s))
    }
}
impl TargetIdType {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            TargetIdType::Account => "ACCOUNT",
            TargetIdType::Ou => "OU",
            TargetIdType::Root => "ROOT",
            TargetIdType::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["ACCOUNT", "OU", "ROOT"]
    }
}
impl AsRef<str> for TargetIdType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum GroupByAttribute {
    #[allow(missing_docs)] // documentation missing in model
    Region,
    #[allow(missing_docs)] // documentation missing in model
    ResourceType,
    #[allow(missing_docs)] // documentation missing in model
    TargetId,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for GroupByAttribute {
    fn from(s: &str) -> Self {
        match s {
            "REGION" => GroupByAttribute::Region,
            "RESOURCE_TYPE" => GroupByAttribute::ResourceType,
            "TARGET_ID" => GroupByAttribute::TargetId,
            other => GroupByAttribute::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for GroupByAttribute {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(GroupByAttribute::from(s))
    }
}
impl GroupByAttribute {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            GroupByAttribute::Region => "REGION",
            GroupByAttribute::ResourceType => "RESOURCE_TYPE",
            GroupByAttribute::TargetId => "TARGET_ID",
            GroupByAttribute::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["REGION", "RESOURCE_TYPE", "TARGET_ID"]
    }
}
impl AsRef<str> for GroupByAttribute {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
