// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListSipRules`](crate::operation::list_sip_rules::builders::ListSipRulesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_sip_rules::builders::ListSipRulesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`sip_media_application_id(impl Into<String>)`](crate::operation::list_sip_rules::builders::ListSipRulesFluentBuilder::sip_media_application_id) / [`set_sip_media_application_id(Option<String>)`](crate::operation::list_sip_rules::builders::ListSipRulesFluentBuilder::set_sip_media_application_id):<br>required: **false**<br><p>The SIP media application ID.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_sip_rules::builders::ListSipRulesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_sip_rules::builders::ListSipRulesFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to return in a single call. Defaults to 100.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_sip_rules::builders::ListSipRulesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_sip_rules::builders::ListSipRulesFluentBuilder::set_next_token):<br>required: **false**<br><p>The token to use to retrieve the next page of results.</p><br>
    /// - On success, responds with [`ListSipRulesOutput`](crate::operation::list_sip_rules::ListSipRulesOutput) with field(s):
    ///   - [`sip_rules(Option<Vec::<SipRule>>)`](crate::operation::list_sip_rules::ListSipRulesOutput::sip_rules): <p>List of SIP rules and rule details.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_sip_rules::ListSipRulesOutput::next_token): <p>The token to use to retrieve the next page of results.</p>
    /// - On failure, responds with [`SdkError<ListSipRulesError>`](crate::operation::list_sip_rules::ListSipRulesError)
    #[deprecated(note = "Replaced by ListSipRules in the Amazon Chime SDK Voice Namespace")]
    pub fn list_sip_rules(&self) -> crate::operation::list_sip_rules::builders::ListSipRulesFluentBuilder {
        crate::operation::list_sip_rules::builders::ListSipRulesFluentBuilder::new(self.handle.clone())
    }
}
