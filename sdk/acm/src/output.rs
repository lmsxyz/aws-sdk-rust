// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct UpdateCertificateOptionsOutput {}
impl std::fmt::Debug for UpdateCertificateOptionsOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("UpdateCertificateOptionsOutput");
        formatter.finish()
    }
}
/// See [`UpdateCertificateOptionsOutput`](crate::output::UpdateCertificateOptionsOutput)
pub mod update_certificate_options_output {
    /// A builder for [`UpdateCertificateOptionsOutput`](crate::output::UpdateCertificateOptionsOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`UpdateCertificateOptionsOutput`](crate::output::UpdateCertificateOptionsOutput)
        pub fn build(self) -> crate::output::UpdateCertificateOptionsOutput {
            crate::output::UpdateCertificateOptionsOutput {}
        }
    }
}
impl UpdateCertificateOptionsOutput {
    /// Creates a new builder-style object to manufacture [`UpdateCertificateOptionsOutput`](crate::output::UpdateCertificateOptionsOutput)
    pub fn builder() -> crate::output::update_certificate_options_output::Builder {
        crate::output::update_certificate_options_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ResendValidationEmailOutput {}
impl std::fmt::Debug for ResendValidationEmailOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ResendValidationEmailOutput");
        formatter.finish()
    }
}
/// See [`ResendValidationEmailOutput`](crate::output::ResendValidationEmailOutput)
pub mod resend_validation_email_output {
    /// A builder for [`ResendValidationEmailOutput`](crate::output::ResendValidationEmailOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`ResendValidationEmailOutput`](crate::output::ResendValidationEmailOutput)
        pub fn build(self) -> crate::output::ResendValidationEmailOutput {
            crate::output::ResendValidationEmailOutput {}
        }
    }
}
impl ResendValidationEmailOutput {
    /// Creates a new builder-style object to manufacture [`ResendValidationEmailOutput`](crate::output::ResendValidationEmailOutput)
    pub fn builder() -> crate::output::resend_validation_email_output::Builder {
        crate::output::resend_validation_email_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct RequestCertificateOutput {
    /// <p>String that contains the ARN of the issued certificate. This must be of the form:</p>
    /// <p>
    /// <code>arn:aws:acm:us-east-1:123456789012:certificate/12345678-1234-1234-1234-123456789012</code>
    /// </p>
    pub certificate_arn: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for RequestCertificateOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("RequestCertificateOutput");
        formatter.field("certificate_arn", &self.certificate_arn);
        formatter.finish()
    }
}
/// See [`RequestCertificateOutput`](crate::output::RequestCertificateOutput)
pub mod request_certificate_output {
    /// A builder for [`RequestCertificateOutput`](crate::output::RequestCertificateOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) certificate_arn: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>String that contains the ARN of the issued certificate. This must be of the form:</p>
        /// <p>
        /// <code>arn:aws:acm:us-east-1:123456789012:certificate/12345678-1234-1234-1234-123456789012</code>
        /// </p>
        pub fn certificate_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.certificate_arn = Some(input.into());
            self
        }
        /// <p>String that contains the ARN of the issued certificate. This must be of the form:</p>
        /// <p>
        /// <code>arn:aws:acm:us-east-1:123456789012:certificate/12345678-1234-1234-1234-123456789012</code>
        /// </p>
        pub fn set_certificate_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.certificate_arn = input;
            self
        }
        /// Consumes the builder and constructs a [`RequestCertificateOutput`](crate::output::RequestCertificateOutput)
        pub fn build(self) -> crate::output::RequestCertificateOutput {
            crate::output::RequestCertificateOutput {
                certificate_arn: self.certificate_arn,
            }
        }
    }
}
impl RequestCertificateOutput {
    /// Creates a new builder-style object to manufacture [`RequestCertificateOutput`](crate::output::RequestCertificateOutput)
    pub fn builder() -> crate::output::request_certificate_output::Builder {
        crate::output::request_certificate_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct RenewCertificateOutput {}
impl std::fmt::Debug for RenewCertificateOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("RenewCertificateOutput");
        formatter.finish()
    }
}
/// See [`RenewCertificateOutput`](crate::output::RenewCertificateOutput)
pub mod renew_certificate_output {
    /// A builder for [`RenewCertificateOutput`](crate::output::RenewCertificateOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`RenewCertificateOutput`](crate::output::RenewCertificateOutput)
        pub fn build(self) -> crate::output::RenewCertificateOutput {
            crate::output::RenewCertificateOutput {}
        }
    }
}
impl RenewCertificateOutput {
    /// Creates a new builder-style object to manufacture [`RenewCertificateOutput`](crate::output::RenewCertificateOutput)
    pub fn builder() -> crate::output::renew_certificate_output::Builder {
        crate::output::renew_certificate_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct RemoveTagsFromCertificateOutput {}
impl std::fmt::Debug for RemoveTagsFromCertificateOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("RemoveTagsFromCertificateOutput");
        formatter.finish()
    }
}
/// See [`RemoveTagsFromCertificateOutput`](crate::output::RemoveTagsFromCertificateOutput)
pub mod remove_tags_from_certificate_output {
    /// A builder for [`RemoveTagsFromCertificateOutput`](crate::output::RemoveTagsFromCertificateOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`RemoveTagsFromCertificateOutput`](crate::output::RemoveTagsFromCertificateOutput)
        pub fn build(self) -> crate::output::RemoveTagsFromCertificateOutput {
            crate::output::RemoveTagsFromCertificateOutput {}
        }
    }
}
impl RemoveTagsFromCertificateOutput {
    /// Creates a new builder-style object to manufacture [`RemoveTagsFromCertificateOutput`](crate::output::RemoveTagsFromCertificateOutput)
    pub fn builder() -> crate::output::remove_tags_from_certificate_output::Builder {
        crate::output::remove_tags_from_certificate_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct PutAccountConfigurationOutput {}
impl std::fmt::Debug for PutAccountConfigurationOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("PutAccountConfigurationOutput");
        formatter.finish()
    }
}
/// See [`PutAccountConfigurationOutput`](crate::output::PutAccountConfigurationOutput)
pub mod put_account_configuration_output {
    /// A builder for [`PutAccountConfigurationOutput`](crate::output::PutAccountConfigurationOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`PutAccountConfigurationOutput`](crate::output::PutAccountConfigurationOutput)
        pub fn build(self) -> crate::output::PutAccountConfigurationOutput {
            crate::output::PutAccountConfigurationOutput {}
        }
    }
}
impl PutAccountConfigurationOutput {
    /// Creates a new builder-style object to manufacture [`PutAccountConfigurationOutput`](crate::output::PutAccountConfigurationOutput)
    pub fn builder() -> crate::output::put_account_configuration_output::Builder {
        crate::output::put_account_configuration_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListTagsForCertificateOutput {
    /// <p>The key-value pairs that define the applied tags.</p>
    pub tags: std::option::Option<std::vec::Vec<crate::model::Tag>>,
}
impl std::fmt::Debug for ListTagsForCertificateOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListTagsForCertificateOutput");
        formatter.field("tags", &self.tags);
        formatter.finish()
    }
}
/// See [`ListTagsForCertificateOutput`](crate::output::ListTagsForCertificateOutput)
pub mod list_tags_for_certificate_output {
    /// A builder for [`ListTagsForCertificateOutput`](crate::output::ListTagsForCertificateOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) tags: std::option::Option<std::vec::Vec<crate::model::Tag>>,
    }
    impl Builder {
        /// Appends an item to `tags`.
        ///
        /// To override the contents of this collection use [`set_tags`](Self::set_tags).
        ///
        /// <p>The key-value pairs that define the applied tags.</p>
        pub fn tags(mut self, input: impl Into<crate::model::Tag>) -> Self {
            let mut v = self.tags.unwrap_or_default();
            v.push(input.into());
            self.tags = Some(v);
            self
        }
        /// <p>The key-value pairs that define the applied tags.</p>
        pub fn set_tags(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Tag>>,
        ) -> Self {
            self.tags = input;
            self
        }
        /// Consumes the builder and constructs a [`ListTagsForCertificateOutput`](crate::output::ListTagsForCertificateOutput)
        pub fn build(self) -> crate::output::ListTagsForCertificateOutput {
            crate::output::ListTagsForCertificateOutput { tags: self.tags }
        }
    }
}
impl ListTagsForCertificateOutput {
    /// Creates a new builder-style object to manufacture [`ListTagsForCertificateOutput`](crate::output::ListTagsForCertificateOutput)
    pub fn builder() -> crate::output::list_tags_for_certificate_output::Builder {
        crate::output::list_tags_for_certificate_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListCertificatesOutput {
    /// <p>When the list is truncated, this value is present and contains the value to use for the
    /// <code>NextToken</code> parameter in a subsequent pagination request.</p>
    pub next_token: std::option::Option<std::string::String>,
    /// <p>A list of ACM certificates.</p>
    pub certificate_summary_list:
        std::option::Option<std::vec::Vec<crate::model::CertificateSummary>>,
}
impl std::fmt::Debug for ListCertificatesOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListCertificatesOutput");
        formatter.field("next_token", &self.next_token);
        formatter.field("certificate_summary_list", &self.certificate_summary_list);
        formatter.finish()
    }
}
/// See [`ListCertificatesOutput`](crate::output::ListCertificatesOutput)
pub mod list_certificates_output {
    /// A builder for [`ListCertificatesOutput`](crate::output::ListCertificatesOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) next_token: std::option::Option<std::string::String>,
        pub(crate) certificate_summary_list:
            std::option::Option<std::vec::Vec<crate::model::CertificateSummary>>,
    }
    impl Builder {
        /// <p>When the list is truncated, this value is present and contains the value to use for the
        /// <code>NextToken</code> parameter in a subsequent pagination request.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>When the list is truncated, this value is present and contains the value to use for the
        /// <code>NextToken</code> parameter in a subsequent pagination request.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Appends an item to `certificate_summary_list`.
        ///
        /// To override the contents of this collection use [`set_certificate_summary_list`](Self::set_certificate_summary_list).
        ///
        /// <p>A list of ACM certificates.</p>
        pub fn certificate_summary_list(
            mut self,
            input: impl Into<crate::model::CertificateSummary>,
        ) -> Self {
            let mut v = self.certificate_summary_list.unwrap_or_default();
            v.push(input.into());
            self.certificate_summary_list = Some(v);
            self
        }
        /// <p>A list of ACM certificates.</p>
        pub fn set_certificate_summary_list(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::CertificateSummary>>,
        ) -> Self {
            self.certificate_summary_list = input;
            self
        }
        /// Consumes the builder and constructs a [`ListCertificatesOutput`](crate::output::ListCertificatesOutput)
        pub fn build(self) -> crate::output::ListCertificatesOutput {
            crate::output::ListCertificatesOutput {
                next_token: self.next_token,
                certificate_summary_list: self.certificate_summary_list,
            }
        }
    }
}
impl ListCertificatesOutput {
    /// Creates a new builder-style object to manufacture [`ListCertificatesOutput`](crate::output::ListCertificatesOutput)
    pub fn builder() -> crate::output::list_certificates_output::Builder {
        crate::output::list_certificates_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ImportCertificateOutput {
    /// <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Name
    /// (ARN)</a> of the imported certificate.</p>
    pub certificate_arn: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ImportCertificateOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ImportCertificateOutput");
        formatter.field("certificate_arn", &self.certificate_arn);
        formatter.finish()
    }
}
/// See [`ImportCertificateOutput`](crate::output::ImportCertificateOutput)
pub mod import_certificate_output {
    /// A builder for [`ImportCertificateOutput`](crate::output::ImportCertificateOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) certificate_arn: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Name
        /// (ARN)</a> of the imported certificate.</p>
        pub fn certificate_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.certificate_arn = Some(input.into());
            self
        }
        /// <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Name
        /// (ARN)</a> of the imported certificate.</p>
        pub fn set_certificate_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.certificate_arn = input;
            self
        }
        /// Consumes the builder and constructs a [`ImportCertificateOutput`](crate::output::ImportCertificateOutput)
        pub fn build(self) -> crate::output::ImportCertificateOutput {
            crate::output::ImportCertificateOutput {
                certificate_arn: self.certificate_arn,
            }
        }
    }
}
impl ImportCertificateOutput {
    /// Creates a new builder-style object to manufacture [`ImportCertificateOutput`](crate::output::ImportCertificateOutput)
    pub fn builder() -> crate::output::import_certificate_output::Builder {
        crate::output::import_certificate_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetCertificateOutput {
    /// <p>The ACM-issued certificate corresponding to the ARN specified as input.</p>
    pub certificate: std::option::Option<std::string::String>,
    /// <p>Certificates forming the requested certificate's chain of trust. The chain consists of the
    /// certificate of the issuing CA and the intermediate certificates of any other subordinate CAs.
    /// </p>
    pub certificate_chain: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for GetCertificateOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetCertificateOutput");
        formatter.field("certificate", &self.certificate);
        formatter.field("certificate_chain", &self.certificate_chain);
        formatter.finish()
    }
}
/// See [`GetCertificateOutput`](crate::output::GetCertificateOutput)
pub mod get_certificate_output {
    /// A builder for [`GetCertificateOutput`](crate::output::GetCertificateOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) certificate: std::option::Option<std::string::String>,
        pub(crate) certificate_chain: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The ACM-issued certificate corresponding to the ARN specified as input.</p>
        pub fn certificate(mut self, input: impl Into<std::string::String>) -> Self {
            self.certificate = Some(input.into());
            self
        }
        /// <p>The ACM-issued certificate corresponding to the ARN specified as input.</p>
        pub fn set_certificate(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.certificate = input;
            self
        }
        /// <p>Certificates forming the requested certificate's chain of trust. The chain consists of the
        /// certificate of the issuing CA and the intermediate certificates of any other subordinate CAs.
        /// </p>
        pub fn certificate_chain(mut self, input: impl Into<std::string::String>) -> Self {
            self.certificate_chain = Some(input.into());
            self
        }
        /// <p>Certificates forming the requested certificate's chain of trust. The chain consists of the
        /// certificate of the issuing CA and the intermediate certificates of any other subordinate CAs.
        /// </p>
        pub fn set_certificate_chain(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.certificate_chain = input;
            self
        }
        /// Consumes the builder and constructs a [`GetCertificateOutput`](crate::output::GetCertificateOutput)
        pub fn build(self) -> crate::output::GetCertificateOutput {
            crate::output::GetCertificateOutput {
                certificate: self.certificate,
                certificate_chain: self.certificate_chain,
            }
        }
    }
}
impl GetCertificateOutput {
    /// Creates a new builder-style object to manufacture [`GetCertificateOutput`](crate::output::GetCertificateOutput)
    pub fn builder() -> crate::output::get_certificate_output::Builder {
        crate::output::get_certificate_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetAccountConfigurationOutput {
    /// <p>Expiration events
    /// configuration options associated with the Amazon Web Services account.</p>
    pub expiry_events: std::option::Option<crate::model::ExpiryEventsConfiguration>,
}
impl std::fmt::Debug for GetAccountConfigurationOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetAccountConfigurationOutput");
        formatter.field("expiry_events", &self.expiry_events);
        formatter.finish()
    }
}
/// See [`GetAccountConfigurationOutput`](crate::output::GetAccountConfigurationOutput)
pub mod get_account_configuration_output {
    /// A builder for [`GetAccountConfigurationOutput`](crate::output::GetAccountConfigurationOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) expiry_events: std::option::Option<crate::model::ExpiryEventsConfiguration>,
    }
    impl Builder {
        /// <p>Expiration events
        /// configuration options associated with the Amazon Web Services account.</p>
        pub fn expiry_events(mut self, input: crate::model::ExpiryEventsConfiguration) -> Self {
            self.expiry_events = Some(input);
            self
        }
        /// <p>Expiration events
        /// configuration options associated with the Amazon Web Services account.</p>
        pub fn set_expiry_events(
            mut self,
            input: std::option::Option<crate::model::ExpiryEventsConfiguration>,
        ) -> Self {
            self.expiry_events = input;
            self
        }
        /// Consumes the builder and constructs a [`GetAccountConfigurationOutput`](crate::output::GetAccountConfigurationOutput)
        pub fn build(self) -> crate::output::GetAccountConfigurationOutput {
            crate::output::GetAccountConfigurationOutput {
                expiry_events: self.expiry_events,
            }
        }
    }
}
impl GetAccountConfigurationOutput {
    /// Creates a new builder-style object to manufacture [`GetAccountConfigurationOutput`](crate::output::GetAccountConfigurationOutput)
    pub fn builder() -> crate::output::get_account_configuration_output::Builder {
        crate::output::get_account_configuration_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ExportCertificateOutput {
    /// <p>The base64 PEM-encoded certificate.</p>
    pub certificate: std::option::Option<std::string::String>,
    /// <p>The base64 PEM-encoded certificate chain. This does not include the certificate that you
    /// are exporting.</p>
    pub certificate_chain: std::option::Option<std::string::String>,
    /// <p>The encrypted private key associated with the public key in the certificate. The key is
    /// output in PKCS #8 format and is base64 PEM-encoded. </p>
    pub private_key: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ExportCertificateOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ExportCertificateOutput");
        formatter.field("certificate", &self.certificate);
        formatter.field("certificate_chain", &self.certificate_chain);
        formatter.field("private_key", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
/// See [`ExportCertificateOutput`](crate::output::ExportCertificateOutput)
pub mod export_certificate_output {
    /// A builder for [`ExportCertificateOutput`](crate::output::ExportCertificateOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) certificate: std::option::Option<std::string::String>,
        pub(crate) certificate_chain: std::option::Option<std::string::String>,
        pub(crate) private_key: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The base64 PEM-encoded certificate.</p>
        pub fn certificate(mut self, input: impl Into<std::string::String>) -> Self {
            self.certificate = Some(input.into());
            self
        }
        /// <p>The base64 PEM-encoded certificate.</p>
        pub fn set_certificate(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.certificate = input;
            self
        }
        /// <p>The base64 PEM-encoded certificate chain. This does not include the certificate that you
        /// are exporting.</p>
        pub fn certificate_chain(mut self, input: impl Into<std::string::String>) -> Self {
            self.certificate_chain = Some(input.into());
            self
        }
        /// <p>The base64 PEM-encoded certificate chain. This does not include the certificate that you
        /// are exporting.</p>
        pub fn set_certificate_chain(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.certificate_chain = input;
            self
        }
        /// <p>The encrypted private key associated with the public key in the certificate. The key is
        /// output in PKCS #8 format and is base64 PEM-encoded. </p>
        pub fn private_key(mut self, input: impl Into<std::string::String>) -> Self {
            self.private_key = Some(input.into());
            self
        }
        /// <p>The encrypted private key associated with the public key in the certificate. The key is
        /// output in PKCS #8 format and is base64 PEM-encoded. </p>
        pub fn set_private_key(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.private_key = input;
            self
        }
        /// Consumes the builder and constructs a [`ExportCertificateOutput`](crate::output::ExportCertificateOutput)
        pub fn build(self) -> crate::output::ExportCertificateOutput {
            crate::output::ExportCertificateOutput {
                certificate: self.certificate,
                certificate_chain: self.certificate_chain,
                private_key: self.private_key,
            }
        }
    }
}
impl ExportCertificateOutput {
    /// Creates a new builder-style object to manufacture [`ExportCertificateOutput`](crate::output::ExportCertificateOutput)
    pub fn builder() -> crate::output::export_certificate_output::Builder {
        crate::output::export_certificate_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DescribeCertificateOutput {
    /// <p>Metadata about an ACM certificate.</p>
    pub certificate: std::option::Option<crate::model::CertificateDetail>,
}
impl std::fmt::Debug for DescribeCertificateOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeCertificateOutput");
        formatter.field("certificate", &self.certificate);
        formatter.finish()
    }
}
/// See [`DescribeCertificateOutput`](crate::output::DescribeCertificateOutput)
pub mod describe_certificate_output {
    /// A builder for [`DescribeCertificateOutput`](crate::output::DescribeCertificateOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) certificate: std::option::Option<crate::model::CertificateDetail>,
    }
    impl Builder {
        /// <p>Metadata about an ACM certificate.</p>
        pub fn certificate(mut self, input: crate::model::CertificateDetail) -> Self {
            self.certificate = Some(input);
            self
        }
        /// <p>Metadata about an ACM certificate.</p>
        pub fn set_certificate(
            mut self,
            input: std::option::Option<crate::model::CertificateDetail>,
        ) -> Self {
            self.certificate = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeCertificateOutput`](crate::output::DescribeCertificateOutput)
        pub fn build(self) -> crate::output::DescribeCertificateOutput {
            crate::output::DescribeCertificateOutput {
                certificate: self.certificate,
            }
        }
    }
}
impl DescribeCertificateOutput {
    /// Creates a new builder-style object to manufacture [`DescribeCertificateOutput`](crate::output::DescribeCertificateOutput)
    pub fn builder() -> crate::output::describe_certificate_output::Builder {
        crate::output::describe_certificate_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DeleteCertificateOutput {}
impl std::fmt::Debug for DeleteCertificateOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DeleteCertificateOutput");
        formatter.finish()
    }
}
/// See [`DeleteCertificateOutput`](crate::output::DeleteCertificateOutput)
pub mod delete_certificate_output {
    /// A builder for [`DeleteCertificateOutput`](crate::output::DeleteCertificateOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`DeleteCertificateOutput`](crate::output::DeleteCertificateOutput)
        pub fn build(self) -> crate::output::DeleteCertificateOutput {
            crate::output::DeleteCertificateOutput {}
        }
    }
}
impl DeleteCertificateOutput {
    /// Creates a new builder-style object to manufacture [`DeleteCertificateOutput`](crate::output::DeleteCertificateOutput)
    pub fn builder() -> crate::output::delete_certificate_output::Builder {
        crate::output::delete_certificate_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct AddTagsToCertificateOutput {}
impl std::fmt::Debug for AddTagsToCertificateOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("AddTagsToCertificateOutput");
        formatter.finish()
    }
}
/// See [`AddTagsToCertificateOutput`](crate::output::AddTagsToCertificateOutput)
pub mod add_tags_to_certificate_output {
    /// A builder for [`AddTagsToCertificateOutput`](crate::output::AddTagsToCertificateOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`AddTagsToCertificateOutput`](crate::output::AddTagsToCertificateOutput)
        pub fn build(self) -> crate::output::AddTagsToCertificateOutput {
            crate::output::AddTagsToCertificateOutput {}
        }
    }
}
impl AddTagsToCertificateOutput {
    /// Creates a new builder-style object to manufacture [`AddTagsToCertificateOutput`](crate::output::AddTagsToCertificateOutput)
    pub fn builder() -> crate::output::add_tags_to_certificate_output::Builder {
        crate::output::add_tags_to_certificate_output::Builder::default()
    }
}
