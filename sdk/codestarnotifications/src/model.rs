// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
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
pub enum DetailType {
    #[allow(missing_docs)] // documentation missing in model
    Basic,
    #[allow(missing_docs)] // documentation missing in model
    Full,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for DetailType {
    fn from(s: &str) -> Self {
        match s {
            "BASIC" => DetailType::Basic,
            "FULL" => DetailType::Full,
            other => DetailType::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for DetailType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(DetailType::from(s))
    }
}
impl DetailType {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            DetailType::Basic => "BASIC",
            DetailType::Full => "FULL",
            DetailType::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["BASIC", "FULL"]
    }
}
impl AsRef<str> for DetailType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>Information about the SNS topics associated with a  notification rule.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct Target {
    /// <p>The target type. Can be an Amazon SNS topic.</p>
    pub target_type: std::option::Option<std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the SNS topic.</p>
    pub target_address: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Target");
        formatter.field("target_type", &self.target_type);
        formatter.field("target_address", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
/// See [`Target`](crate::model::Target)
pub mod target {
    /// A builder for [`Target`](crate::model::Target)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) target_type: std::option::Option<std::string::String>,
        pub(crate) target_address: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The target type. Can be an Amazon SNS topic.</p>
        pub fn target_type(mut self, input: impl Into<std::string::String>) -> Self {
            self.target_type = Some(input.into());
            self
        }
        /// <p>The target type. Can be an Amazon SNS topic.</p>
        pub fn set_target_type(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.target_type = input;
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the SNS topic.</p>
        pub fn target_address(mut self, input: impl Into<std::string::String>) -> Self {
            self.target_address = Some(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the SNS topic.</p>
        pub fn set_target_address(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.target_address = input;
            self
        }
        /// Consumes the builder and constructs a [`Target`](crate::model::Target)
        pub fn build(self) -> crate::model::Target {
            crate::model::Target {
                target_type: self.target_type,
                target_address: self.target_address,
            }
        }
    }
}
impl Target {
    /// Creates a new builder-style object to manufacture [`Target`](crate::model::Target)
    pub fn builder() -> crate::model::target::Builder {
        crate::model::target::Builder::default()
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
pub enum NotificationRuleStatus {
    #[allow(missing_docs)] // documentation missing in model
    Disabled,
    #[allow(missing_docs)] // documentation missing in model
    Enabled,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for NotificationRuleStatus {
    fn from(s: &str) -> Self {
        match s {
            "DISABLED" => NotificationRuleStatus::Disabled,
            "ENABLED" => NotificationRuleStatus::Enabled,
            other => NotificationRuleStatus::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for NotificationRuleStatus {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(NotificationRuleStatus::from(s))
    }
}
impl NotificationRuleStatus {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            NotificationRuleStatus::Disabled => "DISABLED",
            NotificationRuleStatus::Enabled => "ENABLED",
            NotificationRuleStatus::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["DISABLED", "ENABLED"]
    }
}
impl AsRef<str> for NotificationRuleStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>Information about the targets specified for a notification rule.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct TargetSummary {
    /// <p>The Amazon Resource Name (ARN) of the SNS topic.</p>
    pub target_address: std::option::Option<std::string::String>,
    /// <p>The type of the target (for example, SNS).</p>
    pub target_type: std::option::Option<std::string::String>,
    /// <p>The status of the target.</p>
    pub target_status: std::option::Option<crate::model::TargetStatus>,
}
impl std::fmt::Debug for TargetSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("TargetSummary");
        formatter.field("target_address", &"*** Sensitive Data Redacted ***");
        formatter.field("target_type", &self.target_type);
        formatter.field("target_status", &self.target_status);
        formatter.finish()
    }
}
/// See [`TargetSummary`](crate::model::TargetSummary)
pub mod target_summary {
    /// A builder for [`TargetSummary`](crate::model::TargetSummary)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) target_address: std::option::Option<std::string::String>,
        pub(crate) target_type: std::option::Option<std::string::String>,
        pub(crate) target_status: std::option::Option<crate::model::TargetStatus>,
    }
    impl Builder {
        /// <p>The Amazon Resource Name (ARN) of the SNS topic.</p>
        pub fn target_address(mut self, input: impl Into<std::string::String>) -> Self {
            self.target_address = Some(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the SNS topic.</p>
        pub fn set_target_address(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.target_address = input;
            self
        }
        /// <p>The type of the target (for example, SNS).</p>
        pub fn target_type(mut self, input: impl Into<std::string::String>) -> Self {
            self.target_type = Some(input.into());
            self
        }
        /// <p>The type of the target (for example, SNS).</p>
        pub fn set_target_type(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.target_type = input;
            self
        }
        /// <p>The status of the target.</p>
        pub fn target_status(mut self, input: crate::model::TargetStatus) -> Self {
            self.target_status = Some(input);
            self
        }
        /// <p>The status of the target.</p>
        pub fn set_target_status(
            mut self,
            input: std::option::Option<crate::model::TargetStatus>,
        ) -> Self {
            self.target_status = input;
            self
        }
        /// Consumes the builder and constructs a [`TargetSummary`](crate::model::TargetSummary)
        pub fn build(self) -> crate::model::TargetSummary {
            crate::model::TargetSummary {
                target_address: self.target_address,
                target_type: self.target_type,
                target_status: self.target_status,
            }
        }
    }
}
impl TargetSummary {
    /// Creates a new builder-style object to manufacture [`TargetSummary`](crate::model::TargetSummary)
    pub fn builder() -> crate::model::target_summary::Builder {
        crate::model::target_summary::Builder::default()
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
pub enum TargetStatus {
    #[allow(missing_docs)] // documentation missing in model
    Active,
    #[allow(missing_docs)] // documentation missing in model
    Deactivated,
    #[allow(missing_docs)] // documentation missing in model
    Inactive,
    #[allow(missing_docs)] // documentation missing in model
    Pending,
    #[allow(missing_docs)] // documentation missing in model
    Unreachable,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for TargetStatus {
    fn from(s: &str) -> Self {
        match s {
            "ACTIVE" => TargetStatus::Active,
            "DEACTIVATED" => TargetStatus::Deactivated,
            "INACTIVE" => TargetStatus::Inactive,
            "PENDING" => TargetStatus::Pending,
            "UNREACHABLE" => TargetStatus::Unreachable,
            other => TargetStatus::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for TargetStatus {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(TargetStatus::from(s))
    }
}
impl TargetStatus {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            TargetStatus::Active => "ACTIVE",
            TargetStatus::Deactivated => "DEACTIVATED",
            TargetStatus::Inactive => "INACTIVE",
            TargetStatus::Pending => "PENDING",
            TargetStatus::Unreachable => "UNREACHABLE",
            TargetStatus::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &[
            "ACTIVE",
            "DEACTIVATED",
            "INACTIVE",
            "PENDING",
            "UNREACHABLE",
        ]
    }
}
impl AsRef<str> for TargetStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>Information about a filter to apply to the list of returned targets. You can filter by
/// target type, address, or status. For example, to filter results to notification rules
/// that have active Amazon SNS topics as targets, you could specify a ListTargetsFilter
/// Name as TargetType and a Value of SNS, and a Name of TARGET_STATUS and a Value of
/// ACTIVE.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListTargetsFilter {
    /// <p>The name of the attribute you want to use to filter the returned targets.</p>
    pub name: std::option::Option<crate::model::ListTargetsFilterName>,
    /// <p>The value of the attribute you want to use to filter the returned targets. For example,
    /// if you specify <i>SNS</i> for the Target type, you could specify an Amazon
    /// Resource Name (ARN) for a topic as the value.</p>
    pub value: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ListTargetsFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListTargetsFilter");
        formatter.field("name", &self.name);
        formatter.field("value", &self.value);
        formatter.finish()
    }
}
/// See [`ListTargetsFilter`](crate::model::ListTargetsFilter)
pub mod list_targets_filter {
    /// A builder for [`ListTargetsFilter`](crate::model::ListTargetsFilter)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) name: std::option::Option<crate::model::ListTargetsFilterName>,
        pub(crate) value: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The name of the attribute you want to use to filter the returned targets.</p>
        pub fn name(mut self, input: crate::model::ListTargetsFilterName) -> Self {
            self.name = Some(input);
            self
        }
        /// <p>The name of the attribute you want to use to filter the returned targets.</p>
        pub fn set_name(
            mut self,
            input: std::option::Option<crate::model::ListTargetsFilterName>,
        ) -> Self {
            self.name = input;
            self
        }
        /// <p>The value of the attribute you want to use to filter the returned targets. For example,
        /// if you specify <i>SNS</i> for the Target type, you could specify an Amazon
        /// Resource Name (ARN) for a topic as the value.</p>
        pub fn value(mut self, input: impl Into<std::string::String>) -> Self {
            self.value = Some(input.into());
            self
        }
        /// <p>The value of the attribute you want to use to filter the returned targets. For example,
        /// if you specify <i>SNS</i> for the Target type, you could specify an Amazon
        /// Resource Name (ARN) for a topic as the value.</p>
        pub fn set_value(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.value = input;
            self
        }
        /// Consumes the builder and constructs a [`ListTargetsFilter`](crate::model::ListTargetsFilter)
        pub fn build(self) -> crate::model::ListTargetsFilter {
            crate::model::ListTargetsFilter {
                name: self.name,
                value: self.value,
            }
        }
    }
}
impl ListTargetsFilter {
    /// Creates a new builder-style object to manufacture [`ListTargetsFilter`](crate::model::ListTargetsFilter)
    pub fn builder() -> crate::model::list_targets_filter::Builder {
        crate::model::list_targets_filter::Builder::default()
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
pub enum ListTargetsFilterName {
    #[allow(missing_docs)] // documentation missing in model
    TargetAddress,
    #[allow(missing_docs)] // documentation missing in model
    TargetStatus,
    #[allow(missing_docs)] // documentation missing in model
    TargetType,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for ListTargetsFilterName {
    fn from(s: &str) -> Self {
        match s {
            "TARGET_ADDRESS" => ListTargetsFilterName::TargetAddress,
            "TARGET_STATUS" => ListTargetsFilterName::TargetStatus,
            "TARGET_TYPE" => ListTargetsFilterName::TargetType,
            other => ListTargetsFilterName::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for ListTargetsFilterName {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(ListTargetsFilterName::from(s))
    }
}
impl ListTargetsFilterName {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            ListTargetsFilterName::TargetAddress => "TARGET_ADDRESS",
            ListTargetsFilterName::TargetStatus => "TARGET_STATUS",
            ListTargetsFilterName::TargetType => "TARGET_TYPE",
            ListTargetsFilterName::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["TARGET_ADDRESS", "TARGET_STATUS", "TARGET_TYPE"]
    }
}
impl AsRef<str> for ListTargetsFilterName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>Information about a specified notification rule.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct NotificationRuleSummary {
    /// <p>The unique ID of the notification rule.</p>
    pub id: std::option::Option<std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the notification rule.</p>
    pub arn: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for NotificationRuleSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("NotificationRuleSummary");
        formatter.field("id", &self.id);
        formatter.field("arn", &self.arn);
        formatter.finish()
    }
}
/// See [`NotificationRuleSummary`](crate::model::NotificationRuleSummary)
pub mod notification_rule_summary {
    /// A builder for [`NotificationRuleSummary`](crate::model::NotificationRuleSummary)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) id: std::option::Option<std::string::String>,
        pub(crate) arn: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The unique ID of the notification rule.</p>
        pub fn id(mut self, input: impl Into<std::string::String>) -> Self {
            self.id = Some(input.into());
            self
        }
        /// <p>The unique ID of the notification rule.</p>
        pub fn set_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.id = input;
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the notification rule.</p>
        pub fn arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.arn = Some(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the notification rule.</p>
        pub fn set_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.arn = input;
            self
        }
        /// Consumes the builder and constructs a [`NotificationRuleSummary`](crate::model::NotificationRuleSummary)
        pub fn build(self) -> crate::model::NotificationRuleSummary {
            crate::model::NotificationRuleSummary {
                id: self.id,
                arn: self.arn,
            }
        }
    }
}
impl NotificationRuleSummary {
    /// Creates a new builder-style object to manufacture [`NotificationRuleSummary`](crate::model::NotificationRuleSummary)
    pub fn builder() -> crate::model::notification_rule_summary::Builder {
        crate::model::notification_rule_summary::Builder::default()
    }
}

/// <p>Information about a filter to apply to the list of returned notification rules. You can
/// filter by event type, owner, resource, or target.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListNotificationRulesFilter {
    /// <p>The name of the attribute you want to use to filter the returned notification rules.</p>
    pub name: std::option::Option<crate::model::ListNotificationRulesFilterName>,
    /// <p>The value of the attribute you want to use to filter the returned notification rules. For example, if you specify filtering by <i>RESOURCE</i>
    /// in Name, you might specify the ARN of a pipeline in AWS CodePipeline for the value.</p>
    pub value: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ListNotificationRulesFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListNotificationRulesFilter");
        formatter.field("name", &self.name);
        formatter.field("value", &self.value);
        formatter.finish()
    }
}
/// See [`ListNotificationRulesFilter`](crate::model::ListNotificationRulesFilter)
pub mod list_notification_rules_filter {
    /// A builder for [`ListNotificationRulesFilter`](crate::model::ListNotificationRulesFilter)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) name: std::option::Option<crate::model::ListNotificationRulesFilterName>,
        pub(crate) value: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The name of the attribute you want to use to filter the returned notification rules.</p>
        pub fn name(mut self, input: crate::model::ListNotificationRulesFilterName) -> Self {
            self.name = Some(input);
            self
        }
        /// <p>The name of the attribute you want to use to filter the returned notification rules.</p>
        pub fn set_name(
            mut self,
            input: std::option::Option<crate::model::ListNotificationRulesFilterName>,
        ) -> Self {
            self.name = input;
            self
        }
        /// <p>The value of the attribute you want to use to filter the returned notification rules. For example, if you specify filtering by <i>RESOURCE</i>
        /// in Name, you might specify the ARN of a pipeline in AWS CodePipeline for the value.</p>
        pub fn value(mut self, input: impl Into<std::string::String>) -> Self {
            self.value = Some(input.into());
            self
        }
        /// <p>The value of the attribute you want to use to filter the returned notification rules. For example, if you specify filtering by <i>RESOURCE</i>
        /// in Name, you might specify the ARN of a pipeline in AWS CodePipeline for the value.</p>
        pub fn set_value(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.value = input;
            self
        }
        /// Consumes the builder and constructs a [`ListNotificationRulesFilter`](crate::model::ListNotificationRulesFilter)
        pub fn build(self) -> crate::model::ListNotificationRulesFilter {
            crate::model::ListNotificationRulesFilter {
                name: self.name,
                value: self.value,
            }
        }
    }
}
impl ListNotificationRulesFilter {
    /// Creates a new builder-style object to manufacture [`ListNotificationRulesFilter`](crate::model::ListNotificationRulesFilter)
    pub fn builder() -> crate::model::list_notification_rules_filter::Builder {
        crate::model::list_notification_rules_filter::Builder::default()
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
pub enum ListNotificationRulesFilterName {
    #[allow(missing_docs)] // documentation missing in model
    CreatedBy,
    #[allow(missing_docs)] // documentation missing in model
    EventTypeId,
    #[allow(missing_docs)] // documentation missing in model
    Resource,
    #[allow(missing_docs)] // documentation missing in model
    TargetAddress,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for ListNotificationRulesFilterName {
    fn from(s: &str) -> Self {
        match s {
            "CREATED_BY" => ListNotificationRulesFilterName::CreatedBy,
            "EVENT_TYPE_ID" => ListNotificationRulesFilterName::EventTypeId,
            "RESOURCE" => ListNotificationRulesFilterName::Resource,
            "TARGET_ADDRESS" => ListNotificationRulesFilterName::TargetAddress,
            other => ListNotificationRulesFilterName::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for ListNotificationRulesFilterName {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(ListNotificationRulesFilterName::from(s))
    }
}
impl ListNotificationRulesFilterName {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            ListNotificationRulesFilterName::CreatedBy => "CREATED_BY",
            ListNotificationRulesFilterName::EventTypeId => "EVENT_TYPE_ID",
            ListNotificationRulesFilterName::Resource => "RESOURCE",
            ListNotificationRulesFilterName::TargetAddress => "TARGET_ADDRESS",
            ListNotificationRulesFilterName::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["CREATED_BY", "EVENT_TYPE_ID", "RESOURCE", "TARGET_ADDRESS"]
    }
}
impl AsRef<str> for ListNotificationRulesFilterName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>Returns information about an event that has triggered a notification rule.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct EventTypeSummary {
    /// <p>The system-generated ID of the event.</p>
    pub event_type_id: std::option::Option<std::string::String>,
    /// <p>The name of the service for which the event applies.</p>
    pub service_name: std::option::Option<std::string::String>,
    /// <p>The name of the event.</p>
    pub event_type_name: std::option::Option<std::string::String>,
    /// <p>The resource type of the event.</p>
    pub resource_type: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for EventTypeSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("EventTypeSummary");
        formatter.field("event_type_id", &self.event_type_id);
        formatter.field("service_name", &self.service_name);
        formatter.field("event_type_name", &self.event_type_name);
        formatter.field("resource_type", &self.resource_type);
        formatter.finish()
    }
}
/// See [`EventTypeSummary`](crate::model::EventTypeSummary)
pub mod event_type_summary {
    /// A builder for [`EventTypeSummary`](crate::model::EventTypeSummary)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) event_type_id: std::option::Option<std::string::String>,
        pub(crate) service_name: std::option::Option<std::string::String>,
        pub(crate) event_type_name: std::option::Option<std::string::String>,
        pub(crate) resource_type: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The system-generated ID of the event.</p>
        pub fn event_type_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.event_type_id = Some(input.into());
            self
        }
        /// <p>The system-generated ID of the event.</p>
        pub fn set_event_type_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.event_type_id = input;
            self
        }
        /// <p>The name of the service for which the event applies.</p>
        pub fn service_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.service_name = Some(input.into());
            self
        }
        /// <p>The name of the service for which the event applies.</p>
        pub fn set_service_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.service_name = input;
            self
        }
        /// <p>The name of the event.</p>
        pub fn event_type_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.event_type_name = Some(input.into());
            self
        }
        /// <p>The name of the event.</p>
        pub fn set_event_type_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.event_type_name = input;
            self
        }
        /// <p>The resource type of the event.</p>
        pub fn resource_type(mut self, input: impl Into<std::string::String>) -> Self {
            self.resource_type = Some(input.into());
            self
        }
        /// <p>The resource type of the event.</p>
        pub fn set_resource_type(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.resource_type = input;
            self
        }
        /// Consumes the builder and constructs a [`EventTypeSummary`](crate::model::EventTypeSummary)
        pub fn build(self) -> crate::model::EventTypeSummary {
            crate::model::EventTypeSummary {
                event_type_id: self.event_type_id,
                service_name: self.service_name,
                event_type_name: self.event_type_name,
                resource_type: self.resource_type,
            }
        }
    }
}
impl EventTypeSummary {
    /// Creates a new builder-style object to manufacture [`EventTypeSummary`](crate::model::EventTypeSummary)
    pub fn builder() -> crate::model::event_type_summary::Builder {
        crate::model::event_type_summary::Builder::default()
    }
}

/// <p>Information about a filter to apply to the list of returned event types. You can filter
/// by resource type or service name.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListEventTypesFilter {
    /// <p>The system-generated name of the filter type you want to filter by.</p>
    pub name: std::option::Option<crate::model::ListEventTypesFilterName>,
    /// <p>The name of the resource type (for example, pipeline) or service name (for example,
    /// CodePipeline) that you want to filter by.</p>
    pub value: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ListEventTypesFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListEventTypesFilter");
        formatter.field("name", &self.name);
        formatter.field("value", &self.value);
        formatter.finish()
    }
}
/// See [`ListEventTypesFilter`](crate::model::ListEventTypesFilter)
pub mod list_event_types_filter {
    /// A builder for [`ListEventTypesFilter`](crate::model::ListEventTypesFilter)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) name: std::option::Option<crate::model::ListEventTypesFilterName>,
        pub(crate) value: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The system-generated name of the filter type you want to filter by.</p>
        pub fn name(mut self, input: crate::model::ListEventTypesFilterName) -> Self {
            self.name = Some(input);
            self
        }
        /// <p>The system-generated name of the filter type you want to filter by.</p>
        pub fn set_name(
            mut self,
            input: std::option::Option<crate::model::ListEventTypesFilterName>,
        ) -> Self {
            self.name = input;
            self
        }
        /// <p>The name of the resource type (for example, pipeline) or service name (for example,
        /// CodePipeline) that you want to filter by.</p>
        pub fn value(mut self, input: impl Into<std::string::String>) -> Self {
            self.value = Some(input.into());
            self
        }
        /// <p>The name of the resource type (for example, pipeline) or service name (for example,
        /// CodePipeline) that you want to filter by.</p>
        pub fn set_value(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.value = input;
            self
        }
        /// Consumes the builder and constructs a [`ListEventTypesFilter`](crate::model::ListEventTypesFilter)
        pub fn build(self) -> crate::model::ListEventTypesFilter {
            crate::model::ListEventTypesFilter {
                name: self.name,
                value: self.value,
            }
        }
    }
}
impl ListEventTypesFilter {
    /// Creates a new builder-style object to manufacture [`ListEventTypesFilter`](crate::model::ListEventTypesFilter)
    pub fn builder() -> crate::model::list_event_types_filter::Builder {
        crate::model::list_event_types_filter::Builder::default()
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
pub enum ListEventTypesFilterName {
    #[allow(missing_docs)] // documentation missing in model
    ResourceType,
    #[allow(missing_docs)] // documentation missing in model
    ServiceName,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for ListEventTypesFilterName {
    fn from(s: &str) -> Self {
        match s {
            "RESOURCE_TYPE" => ListEventTypesFilterName::ResourceType,
            "SERVICE_NAME" => ListEventTypesFilterName::ServiceName,
            other => ListEventTypesFilterName::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for ListEventTypesFilterName {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(ListEventTypesFilterName::from(s))
    }
}
impl ListEventTypesFilterName {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            ListEventTypesFilterName::ResourceType => "RESOURCE_TYPE",
            ListEventTypesFilterName::ServiceName => "SERVICE_NAME",
            ListEventTypesFilterName::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["RESOURCE_TYPE", "SERVICE_NAME"]
    }
}
impl AsRef<str> for ListEventTypesFilterName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
