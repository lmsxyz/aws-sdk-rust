// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Fields to be used while uploading the attachment.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct UploadMetadata {
    /// <p>The pre-signed URL using which file would be downloaded from Amazon S3 by the API caller.</p>
    pub url: std::option::Option<std::string::String>,
    /// <p>The expiration time of the URL in ISO timestamp. It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example, 2019-11-08T02:41:28.172Z.</p>
    pub url_expiry: std::option::Option<std::string::String>,
    /// <p>The headers to be provided while uploading the file to the URL.</p>
    pub headers_to_include:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl std::fmt::Debug for UploadMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("UploadMetadata");
        formatter.field("url", &self.url);
        formatter.field("url_expiry", &self.url_expiry);
        formatter.field("headers_to_include", &self.headers_to_include);
        formatter.finish()
    }
}
/// See [`UploadMetadata`](crate::model::UploadMetadata)
pub mod upload_metadata {
    /// A builder for [`UploadMetadata`](crate::model::UploadMetadata)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) url: std::option::Option<std::string::String>,
        pub(crate) url_expiry: std::option::Option<std::string::String>,
        pub(crate) headers_to_include: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    }
    impl Builder {
        /// <p>The pre-signed URL using which file would be downloaded from Amazon S3 by the API caller.</p>
        pub fn url(mut self, input: impl Into<std::string::String>) -> Self {
            self.url = Some(input.into());
            self
        }
        /// <p>The pre-signed URL using which file would be downloaded from Amazon S3 by the API caller.</p>
        pub fn set_url(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.url = input;
            self
        }
        /// <p>The expiration time of the URL in ISO timestamp. It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example, 2019-11-08T02:41:28.172Z.</p>
        pub fn url_expiry(mut self, input: impl Into<std::string::String>) -> Self {
            self.url_expiry = Some(input.into());
            self
        }
        /// <p>The expiration time of the URL in ISO timestamp. It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example, 2019-11-08T02:41:28.172Z.</p>
        pub fn set_url_expiry(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.url_expiry = input;
            self
        }
        /// Adds a key-value pair to `headers_to_include`.
        ///
        /// To override the contents of this collection use [`set_headers_to_include`](Self::set_headers_to_include).
        ///
        /// <p>The headers to be provided while uploading the file to the URL.</p>
        pub fn headers_to_include(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            let mut hash_map = self.headers_to_include.unwrap_or_default();
            hash_map.insert(k.into(), v.into());
            self.headers_to_include = Some(hash_map);
            self
        }
        /// <p>The headers to be provided while uploading the file to the URL.</p>
        pub fn set_headers_to_include(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.headers_to_include = input;
            self
        }
        /// Consumes the builder and constructs a [`UploadMetadata`](crate::model::UploadMetadata)
        pub fn build(self) -> crate::model::UploadMetadata {
            crate::model::UploadMetadata {
                url: self.url,
                url_expiry: self.url_expiry,
                headers_to_include: self.headers_to_include,
            }
        }
    }
}
impl UploadMetadata {
    /// Creates a new builder-style object to manufacture [`UploadMetadata`](crate::model::UploadMetadata)
    pub fn builder() -> crate::model::upload_metadata::Builder {
        crate::model::upload_metadata::Builder::default()
    }
}

/// <p>An item - message or event - that has been sent. </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct Item {
    /// <p>The time when the message or event was sent.</p>
    /// <p>It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example,
    /// 2019-11-08T02:41:28.172Z.</p>
    pub absolute_time: std::option::Option<std::string::String>,
    /// <p>The content of the message or event.</p>
    pub content: std::option::Option<std::string::String>,
    /// <p>The type of content of the item.</p>
    pub content_type: std::option::Option<std::string::String>,
    /// <p>The ID of the item.</p>
    pub id: std::option::Option<std::string::String>,
    /// <p>Type of the item: message or event. </p>
    pub r#type: std::option::Option<crate::model::ChatItemType>,
    /// <p>The ID of the sender in the session.</p>
    pub participant_id: std::option::Option<std::string::String>,
    /// <p>The chat display name of the sender.</p>
    pub display_name: std::option::Option<std::string::String>,
    /// <p>The role of the sender. For example, is it a customer, agent, or system.</p>
    pub participant_role: std::option::Option<crate::model::ParticipantRole>,
    /// <p>Provides information about the attachments.</p>
    pub attachments: std::option::Option<std::vec::Vec<crate::model::AttachmentItem>>,
}
impl std::fmt::Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Item");
        formatter.field("absolute_time", &self.absolute_time);
        formatter.field("content", &self.content);
        formatter.field("content_type", &self.content_type);
        formatter.field("id", &self.id);
        formatter.field("r#type", &self.r#type);
        formatter.field("participant_id", &self.participant_id);
        formatter.field("display_name", &self.display_name);
        formatter.field("participant_role", &self.participant_role);
        formatter.field("attachments", &self.attachments);
        formatter.finish()
    }
}
/// See [`Item`](crate::model::Item)
pub mod item {
    /// A builder for [`Item`](crate::model::Item)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) absolute_time: std::option::Option<std::string::String>,
        pub(crate) content: std::option::Option<std::string::String>,
        pub(crate) content_type: std::option::Option<std::string::String>,
        pub(crate) id: std::option::Option<std::string::String>,
        pub(crate) r#type: std::option::Option<crate::model::ChatItemType>,
        pub(crate) participant_id: std::option::Option<std::string::String>,
        pub(crate) display_name: std::option::Option<std::string::String>,
        pub(crate) participant_role: std::option::Option<crate::model::ParticipantRole>,
        pub(crate) attachments: std::option::Option<std::vec::Vec<crate::model::AttachmentItem>>,
    }
    impl Builder {
        /// <p>The time when the message or event was sent.</p>
        /// <p>It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example,
        /// 2019-11-08T02:41:28.172Z.</p>
        pub fn absolute_time(mut self, input: impl Into<std::string::String>) -> Self {
            self.absolute_time = Some(input.into());
            self
        }
        /// <p>The time when the message or event was sent.</p>
        /// <p>It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example,
        /// 2019-11-08T02:41:28.172Z.</p>
        pub fn set_absolute_time(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.absolute_time = input;
            self
        }
        /// <p>The content of the message or event.</p>
        pub fn content(mut self, input: impl Into<std::string::String>) -> Self {
            self.content = Some(input.into());
            self
        }
        /// <p>The content of the message or event.</p>
        pub fn set_content(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.content = input;
            self
        }
        /// <p>The type of content of the item.</p>
        pub fn content_type(mut self, input: impl Into<std::string::String>) -> Self {
            self.content_type = Some(input.into());
            self
        }
        /// <p>The type of content of the item.</p>
        pub fn set_content_type(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.content_type = input;
            self
        }
        /// <p>The ID of the item.</p>
        pub fn id(mut self, input: impl Into<std::string::String>) -> Self {
            self.id = Some(input.into());
            self
        }
        /// <p>The ID of the item.</p>
        pub fn set_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.id = input;
            self
        }
        /// <p>Type of the item: message or event. </p>
        pub fn r#type(mut self, input: crate::model::ChatItemType) -> Self {
            self.r#type = Some(input);
            self
        }
        /// <p>Type of the item: message or event. </p>
        pub fn set_type(mut self, input: std::option::Option<crate::model::ChatItemType>) -> Self {
            self.r#type = input;
            self
        }
        /// <p>The ID of the sender in the session.</p>
        pub fn participant_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.participant_id = Some(input.into());
            self
        }
        /// <p>The ID of the sender in the session.</p>
        pub fn set_participant_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.participant_id = input;
            self
        }
        /// <p>The chat display name of the sender.</p>
        pub fn display_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.display_name = Some(input.into());
            self
        }
        /// <p>The chat display name of the sender.</p>
        pub fn set_display_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.display_name = input;
            self
        }
        /// <p>The role of the sender. For example, is it a customer, agent, or system.</p>
        pub fn participant_role(mut self, input: crate::model::ParticipantRole) -> Self {
            self.participant_role = Some(input);
            self
        }
        /// <p>The role of the sender. For example, is it a customer, agent, or system.</p>
        pub fn set_participant_role(
            mut self,
            input: std::option::Option<crate::model::ParticipantRole>,
        ) -> Self {
            self.participant_role = input;
            self
        }
        /// Appends an item to `attachments`.
        ///
        /// To override the contents of this collection use [`set_attachments`](Self::set_attachments).
        ///
        /// <p>Provides information about the attachments.</p>
        pub fn attachments(mut self, input: impl Into<crate::model::AttachmentItem>) -> Self {
            let mut v = self.attachments.unwrap_or_default();
            v.push(input.into());
            self.attachments = Some(v);
            self
        }
        /// <p>Provides information about the attachments.</p>
        pub fn set_attachments(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::AttachmentItem>>,
        ) -> Self {
            self.attachments = input;
            self
        }
        /// Consumes the builder and constructs a [`Item`](crate::model::Item)
        pub fn build(self) -> crate::model::Item {
            crate::model::Item {
                absolute_time: self.absolute_time,
                content: self.content,
                content_type: self.content_type,
                id: self.id,
                r#type: self.r#type,
                participant_id: self.participant_id,
                display_name: self.display_name,
                participant_role: self.participant_role,
                attachments: self.attachments,
            }
        }
    }
}
impl Item {
    /// Creates a new builder-style object to manufacture [`Item`](crate::model::Item)
    pub fn builder() -> crate::model::item::Builder {
        crate::model::item::Builder::default()
    }
}

/// <p>The case-insensitive input to indicate standard MIME type that describes the format of the file
/// that will be uploaded.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct AttachmentItem {
    /// <p>Describes the MIME file type of the attachment. For a list of supported file types, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-service-limits.html#feature-limits">Feature specifications</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    pub content_type: std::option::Option<std::string::String>,
    /// <p>A unique identifier for the attachment.</p>
    pub attachment_id: std::option::Option<std::string::String>,
    /// <p>A case-sensitive name of the attachment being uploaded.</p>
    pub attachment_name: std::option::Option<std::string::String>,
    /// <p>Status of the attachment.</p>
    pub status: std::option::Option<crate::model::ArtifactStatus>,
}
impl std::fmt::Debug for AttachmentItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("AttachmentItem");
        formatter.field("content_type", &self.content_type);
        formatter.field("attachment_id", &self.attachment_id);
        formatter.field("attachment_name", &self.attachment_name);
        formatter.field("status", &self.status);
        formatter.finish()
    }
}
/// See [`AttachmentItem`](crate::model::AttachmentItem)
pub mod attachment_item {
    /// A builder for [`AttachmentItem`](crate::model::AttachmentItem)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) content_type: std::option::Option<std::string::String>,
        pub(crate) attachment_id: std::option::Option<std::string::String>,
        pub(crate) attachment_name: std::option::Option<std::string::String>,
        pub(crate) status: std::option::Option<crate::model::ArtifactStatus>,
    }
    impl Builder {
        /// <p>Describes the MIME file type of the attachment. For a list of supported file types, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-service-limits.html#feature-limits">Feature specifications</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
        pub fn content_type(mut self, input: impl Into<std::string::String>) -> Self {
            self.content_type = Some(input.into());
            self
        }
        /// <p>Describes the MIME file type of the attachment. For a list of supported file types, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-service-limits.html#feature-limits">Feature specifications</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
        pub fn set_content_type(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.content_type = input;
            self
        }
        /// <p>A unique identifier for the attachment.</p>
        pub fn attachment_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.attachment_id = Some(input.into());
            self
        }
        /// <p>A unique identifier for the attachment.</p>
        pub fn set_attachment_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.attachment_id = input;
            self
        }
        /// <p>A case-sensitive name of the attachment being uploaded.</p>
        pub fn attachment_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.attachment_name = Some(input.into());
            self
        }
        /// <p>A case-sensitive name of the attachment being uploaded.</p>
        pub fn set_attachment_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.attachment_name = input;
            self
        }
        /// <p>Status of the attachment.</p>
        pub fn status(mut self, input: crate::model::ArtifactStatus) -> Self {
            self.status = Some(input);
            self
        }
        /// <p>Status of the attachment.</p>
        pub fn set_status(
            mut self,
            input: std::option::Option<crate::model::ArtifactStatus>,
        ) -> Self {
            self.status = input;
            self
        }
        /// Consumes the builder and constructs a [`AttachmentItem`](crate::model::AttachmentItem)
        pub fn build(self) -> crate::model::AttachmentItem {
            crate::model::AttachmentItem {
                content_type: self.content_type,
                attachment_id: self.attachment_id,
                attachment_name: self.attachment_name,
                status: self.status,
            }
        }
    }
}
impl AttachmentItem {
    /// Creates a new builder-style object to manufacture [`AttachmentItem`](crate::model::AttachmentItem)
    pub fn builder() -> crate::model::attachment_item::Builder {
        crate::model::attachment_item::Builder::default()
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
pub enum ArtifactStatus {
    #[allow(missing_docs)] // documentation missing in model
    Approved,
    #[allow(missing_docs)] // documentation missing in model
    InProgress,
    #[allow(missing_docs)] // documentation missing in model
    Rejected,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for ArtifactStatus {
    fn from(s: &str) -> Self {
        match s {
            "APPROVED" => ArtifactStatus::Approved,
            "IN_PROGRESS" => ArtifactStatus::InProgress,
            "REJECTED" => ArtifactStatus::Rejected,
            other => ArtifactStatus::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for ArtifactStatus {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(ArtifactStatus::from(s))
    }
}
impl ArtifactStatus {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            ArtifactStatus::Approved => "APPROVED",
            ArtifactStatus::InProgress => "IN_PROGRESS",
            ArtifactStatus::Rejected => "REJECTED",
            ArtifactStatus::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["APPROVED", "IN_PROGRESS", "REJECTED"]
    }
}
impl AsRef<str> for ArtifactStatus {
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
pub enum ParticipantRole {
    #[allow(missing_docs)] // documentation missing in model
    Agent,
    #[allow(missing_docs)] // documentation missing in model
    Customer,
    #[allow(missing_docs)] // documentation missing in model
    System,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for ParticipantRole {
    fn from(s: &str) -> Self {
        match s {
            "AGENT" => ParticipantRole::Agent,
            "CUSTOMER" => ParticipantRole::Customer,
            "SYSTEM" => ParticipantRole::System,
            other => ParticipantRole::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for ParticipantRole {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(ParticipantRole::from(s))
    }
}
impl ParticipantRole {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            ParticipantRole::Agent => "AGENT",
            ParticipantRole::Customer => "CUSTOMER",
            ParticipantRole::System => "SYSTEM",
            ParticipantRole::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["AGENT", "CUSTOMER", "SYSTEM"]
    }
}
impl AsRef<str> for ParticipantRole {
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
pub enum ChatItemType {
    #[allow(missing_docs)] // documentation missing in model
    Attachment,
    #[allow(missing_docs)] // documentation missing in model
    ChatEnded,
    #[allow(missing_docs)] // documentation missing in model
    ConnectionAck,
    #[allow(missing_docs)] // documentation missing in model
    Event,
    #[allow(missing_docs)] // documentation missing in model
    Message,
    #[allow(missing_docs)] // documentation missing in model
    ParticipantJoined,
    #[allow(missing_docs)] // documentation missing in model
    ParticipantLeft,
    #[allow(missing_docs)] // documentation missing in model
    TransferFailed,
    #[allow(missing_docs)] // documentation missing in model
    TransferSucceeded,
    #[allow(missing_docs)] // documentation missing in model
    Typing,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for ChatItemType {
    fn from(s: &str) -> Self {
        match s {
            "ATTACHMENT" => ChatItemType::Attachment,
            "CHAT_ENDED" => ChatItemType::ChatEnded,
            "CONNECTION_ACK" => ChatItemType::ConnectionAck,
            "EVENT" => ChatItemType::Event,
            "MESSAGE" => ChatItemType::Message,
            "PARTICIPANT_JOINED" => ChatItemType::ParticipantJoined,
            "PARTICIPANT_LEFT" => ChatItemType::ParticipantLeft,
            "TRANSFER_FAILED" => ChatItemType::TransferFailed,
            "TRANSFER_SUCCEEDED" => ChatItemType::TransferSucceeded,
            "TYPING" => ChatItemType::Typing,
            other => ChatItemType::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for ChatItemType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(ChatItemType::from(s))
    }
}
impl ChatItemType {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            ChatItemType::Attachment => "ATTACHMENT",
            ChatItemType::ChatEnded => "CHAT_ENDED",
            ChatItemType::ConnectionAck => "CONNECTION_ACK",
            ChatItemType::Event => "EVENT",
            ChatItemType::Message => "MESSAGE",
            ChatItemType::ParticipantJoined => "PARTICIPANT_JOINED",
            ChatItemType::ParticipantLeft => "PARTICIPANT_LEFT",
            ChatItemType::TransferFailed => "TRANSFER_FAILED",
            ChatItemType::TransferSucceeded => "TRANSFER_SUCCEEDED",
            ChatItemType::Typing => "TYPING",
            ChatItemType::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &[
            "ATTACHMENT",
            "CHAT_ENDED",
            "CONNECTION_ACK",
            "EVENT",
            "MESSAGE",
            "PARTICIPANT_JOINED",
            "PARTICIPANT_LEFT",
            "TRANSFER_FAILED",
            "TRANSFER_SUCCEEDED",
            "TYPING",
        ]
    }
}
impl AsRef<str> for ChatItemType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>A filtering option for where to start. For example, if you sent 100 messages, start
/// with message 50. </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct StartPosition {
    /// <p>The ID of the message or event where to start. </p>
    pub id: std::option::Option<std::string::String>,
    /// <p>The time in ISO format where to start.</p>
    /// <p>It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example,
    /// 2019-11-08T02:41:28.172Z.</p>
    pub absolute_time: std::option::Option<std::string::String>,
    /// <p>The start position of the most recent message where you want to start. </p>
    pub most_recent: i32,
}
impl std::fmt::Debug for StartPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("StartPosition");
        formatter.field("id", &self.id);
        formatter.field("absolute_time", &self.absolute_time);
        formatter.field("most_recent", &self.most_recent);
        formatter.finish()
    }
}
/// See [`StartPosition`](crate::model::StartPosition)
pub mod start_position {
    /// A builder for [`StartPosition`](crate::model::StartPosition)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) id: std::option::Option<std::string::String>,
        pub(crate) absolute_time: std::option::Option<std::string::String>,
        pub(crate) most_recent: std::option::Option<i32>,
    }
    impl Builder {
        /// <p>The ID of the message or event where to start. </p>
        pub fn id(mut self, input: impl Into<std::string::String>) -> Self {
            self.id = Some(input.into());
            self
        }
        /// <p>The ID of the message or event where to start. </p>
        pub fn set_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.id = input;
            self
        }
        /// <p>The time in ISO format where to start.</p>
        /// <p>It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example,
        /// 2019-11-08T02:41:28.172Z.</p>
        pub fn absolute_time(mut self, input: impl Into<std::string::String>) -> Self {
            self.absolute_time = Some(input.into());
            self
        }
        /// <p>The time in ISO format where to start.</p>
        /// <p>It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example,
        /// 2019-11-08T02:41:28.172Z.</p>
        pub fn set_absolute_time(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.absolute_time = input;
            self
        }
        /// <p>The start position of the most recent message where you want to start. </p>
        pub fn most_recent(mut self, input: i32) -> Self {
            self.most_recent = Some(input);
            self
        }
        /// <p>The start position of the most recent message where you want to start. </p>
        pub fn set_most_recent(mut self, input: std::option::Option<i32>) -> Self {
            self.most_recent = input;
            self
        }
        /// Consumes the builder and constructs a [`StartPosition`](crate::model::StartPosition)
        pub fn build(self) -> crate::model::StartPosition {
            crate::model::StartPosition {
                id: self.id,
                absolute_time: self.absolute_time,
                most_recent: self.most_recent.unwrap_or_default(),
            }
        }
    }
}
impl StartPosition {
    /// Creates a new builder-style object to manufacture [`StartPosition`](crate::model::StartPosition)
    pub fn builder() -> crate::model::start_position::Builder {
        crate::model::start_position::Builder::default()
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
pub enum SortKey {
    #[allow(missing_docs)] // documentation missing in model
    Ascending,
    #[allow(missing_docs)] // documentation missing in model
    Descending,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for SortKey {
    fn from(s: &str) -> Self {
        match s {
            "ASCENDING" => SortKey::Ascending,
            "DESCENDING" => SortKey::Descending,
            other => SortKey::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for SortKey {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(SortKey::from(s))
    }
}
impl SortKey {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            SortKey::Ascending => "ASCENDING",
            SortKey::Descending => "DESCENDING",
            SortKey::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["ASCENDING", "DESCENDING"]
    }
}
impl AsRef<str> for SortKey {
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
pub enum ScanDirection {
    #[allow(missing_docs)] // documentation missing in model
    Backward,
    #[allow(missing_docs)] // documentation missing in model
    Forward,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for ScanDirection {
    fn from(s: &str) -> Self {
        match s {
            "BACKWARD" => ScanDirection::Backward,
            "FORWARD" => ScanDirection::Forward,
            other => ScanDirection::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for ScanDirection {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(ScanDirection::from(s))
    }
}
impl ScanDirection {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            ScanDirection::Backward => "BACKWARD",
            ScanDirection::Forward => "FORWARD",
            ScanDirection::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["BACKWARD", "FORWARD"]
    }
}
impl AsRef<str> for ScanDirection {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>Connection credentials. </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ConnectionCredentials {
    /// <p>The connection token.</p>
    pub connection_token: std::option::Option<std::string::String>,
    /// <p>The expiration of the token.</p>
    /// <p>It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example,
    /// 2019-11-08T02:41:28.172Z.</p>
    pub expiry: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ConnectionCredentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ConnectionCredentials");
        formatter.field("connection_token", &self.connection_token);
        formatter.field("expiry", &self.expiry);
        formatter.finish()
    }
}
/// See [`ConnectionCredentials`](crate::model::ConnectionCredentials)
pub mod connection_credentials {
    /// A builder for [`ConnectionCredentials`](crate::model::ConnectionCredentials)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) connection_token: std::option::Option<std::string::String>,
        pub(crate) expiry: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The connection token.</p>
        pub fn connection_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.connection_token = Some(input.into());
            self
        }
        /// <p>The connection token.</p>
        pub fn set_connection_token(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.connection_token = input;
            self
        }
        /// <p>The expiration of the token.</p>
        /// <p>It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example,
        /// 2019-11-08T02:41:28.172Z.</p>
        pub fn expiry(mut self, input: impl Into<std::string::String>) -> Self {
            self.expiry = Some(input.into());
            self
        }
        /// <p>The expiration of the token.</p>
        /// <p>It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example,
        /// 2019-11-08T02:41:28.172Z.</p>
        pub fn set_expiry(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.expiry = input;
            self
        }
        /// Consumes the builder and constructs a [`ConnectionCredentials`](crate::model::ConnectionCredentials)
        pub fn build(self) -> crate::model::ConnectionCredentials {
            crate::model::ConnectionCredentials {
                connection_token: self.connection_token,
                expiry: self.expiry,
            }
        }
    }
}
impl ConnectionCredentials {
    /// Creates a new builder-style object to manufacture [`ConnectionCredentials`](crate::model::ConnectionCredentials)
    pub fn builder() -> crate::model::connection_credentials::Builder {
        crate::model::connection_credentials::Builder::default()
    }
}

/// <p>The websocket for the participant's connection.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct Websocket {
    /// <p>The URL of the websocket.</p>
    pub url: std::option::Option<std::string::String>,
    /// <p>The URL expiration timestamp in ISO date format.</p>
    /// <p>It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example,
    /// 2019-11-08T02:41:28.172Z.</p>
    pub connection_expiry: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for Websocket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Websocket");
        formatter.field("url", &self.url);
        formatter.field("connection_expiry", &self.connection_expiry);
        formatter.finish()
    }
}
/// See [`Websocket`](crate::model::Websocket)
pub mod websocket {
    /// A builder for [`Websocket`](crate::model::Websocket)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) url: std::option::Option<std::string::String>,
        pub(crate) connection_expiry: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The URL of the websocket.</p>
        pub fn url(mut self, input: impl Into<std::string::String>) -> Self {
            self.url = Some(input.into());
            self
        }
        /// <p>The URL of the websocket.</p>
        pub fn set_url(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.url = input;
            self
        }
        /// <p>The URL expiration timestamp in ISO date format.</p>
        /// <p>It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example,
        /// 2019-11-08T02:41:28.172Z.</p>
        pub fn connection_expiry(mut self, input: impl Into<std::string::String>) -> Self {
            self.connection_expiry = Some(input.into());
            self
        }
        /// <p>The URL expiration timestamp in ISO date format.</p>
        /// <p>It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example,
        /// 2019-11-08T02:41:28.172Z.</p>
        pub fn set_connection_expiry(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.connection_expiry = input;
            self
        }
        /// Consumes the builder and constructs a [`Websocket`](crate::model::Websocket)
        pub fn build(self) -> crate::model::Websocket {
            crate::model::Websocket {
                url: self.url,
                connection_expiry: self.connection_expiry,
            }
        }
    }
}
impl Websocket {
    /// Creates a new builder-style object to manufacture [`Websocket`](crate::model::Websocket)
    pub fn builder() -> crate::model::websocket::Builder {
        crate::model::websocket::Builder::default()
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
pub enum ConnectionType {
    #[allow(missing_docs)] // documentation missing in model
    ConnectionCredentials,
    #[allow(missing_docs)] // documentation missing in model
    Websocket,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for ConnectionType {
    fn from(s: &str) -> Self {
        match s {
            "CONNECTION_CREDENTIALS" => ConnectionType::ConnectionCredentials,
            "WEBSOCKET" => ConnectionType::Websocket,
            other => ConnectionType::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for ConnectionType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(ConnectionType::from(s))
    }
}
impl ConnectionType {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            ConnectionType::ConnectionCredentials => "CONNECTION_CREDENTIALS",
            ConnectionType::Websocket => "WEBSOCKET",
            ConnectionType::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["CONNECTION_CREDENTIALS", "WEBSOCKET"]
    }
}
impl AsRef<str> for ConnectionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
