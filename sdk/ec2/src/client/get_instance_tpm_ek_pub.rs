// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetInstanceTpmEkPub`](crate::operation::get_instance_tpm_ek_pub::builders::GetInstanceTpmEkPubFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`instance_id(impl Into<String>)`](crate::operation::get_instance_tpm_ek_pub::builders::GetInstanceTpmEkPubFluentBuilder::instance_id) / [`set_instance_id(Option<String>)`](crate::operation::get_instance_tpm_ek_pub::builders::GetInstanceTpmEkPubFluentBuilder::set_instance_id):<br>required: **true**<br><p>The ID of the instance for which to get the public endorsement key.</p><br>
    ///   - [`key_type(EkPubKeyType)`](crate::operation::get_instance_tpm_ek_pub::builders::GetInstanceTpmEkPubFluentBuilder::key_type) / [`set_key_type(Option<EkPubKeyType>)`](crate::operation::get_instance_tpm_ek_pub::builders::GetInstanceTpmEkPubFluentBuilder::set_key_type):<br>required: **true**<br><p>The required public endorsement key type.</p><br>
    ///   - [`key_format(EkPubKeyFormat)`](crate::operation::get_instance_tpm_ek_pub::builders::GetInstanceTpmEkPubFluentBuilder::key_format) / [`set_key_format(Option<EkPubKeyFormat>)`](crate::operation::get_instance_tpm_ek_pub::builders::GetInstanceTpmEkPubFluentBuilder::set_key_format):<br>required: **true**<br><p>The required public endorsement key format. Specify <code>der</code> for a DER-encoded public key that is compatible with OpenSSL. Specify <code>tpmt</code> for a TPM 2.0 format that is compatible with tpm2-tools. The returned key is base64 encoded.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::get_instance_tpm_ek_pub::builders::GetInstanceTpmEkPubFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::get_instance_tpm_ek_pub::builders::GetInstanceTpmEkPubFluentBuilder::set_dry_run):<br>required: **false**<br><p>Specify this parameter to verify whether the request will succeed, without actually making the request. If the request will succeed, the response is <code>DryRunOperation</code>. Otherwise, the response is <code>UnauthorizedOperation</code>.</p><br>
    /// - On success, responds with [`GetInstanceTpmEkPubOutput`](crate::operation::get_instance_tpm_ek_pub::GetInstanceTpmEkPubOutput) with field(s):
    ///   - [`instance_id(Option<String>)`](crate::operation::get_instance_tpm_ek_pub::GetInstanceTpmEkPubOutput::instance_id): <p>The ID of the instance.</p>
    ///   - [`key_type(Option<EkPubKeyType>)`](crate::operation::get_instance_tpm_ek_pub::GetInstanceTpmEkPubOutput::key_type): <p>The public endorsement key type.</p>
    ///   - [`key_format(Option<EkPubKeyFormat>)`](crate::operation::get_instance_tpm_ek_pub::GetInstanceTpmEkPubOutput::key_format): <p>The public endorsement key format.</p>
    ///   - [`key_value(Option<String>)`](crate::operation::get_instance_tpm_ek_pub::GetInstanceTpmEkPubOutput::key_value): <p>The public endorsement key material.</p>
    /// - On failure, responds with [`SdkError<GetInstanceTpmEkPubError>`](crate::operation::get_instance_tpm_ek_pub::GetInstanceTpmEkPubError)
    pub fn get_instance_tpm_ek_pub(&self) -> crate::operation::get_instance_tpm_ek_pub::builders::GetInstanceTpmEkPubFluentBuilder {
        crate::operation::get_instance_tpm_ek_pub::builders::GetInstanceTpmEkPubFluentBuilder::new(self.handle.clone())
    }
}
