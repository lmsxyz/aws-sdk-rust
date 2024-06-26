// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StopCampaign`](crate::operation::stop_campaign::builders::StopCampaignFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::operation::stop_campaign::builders::StopCampaignFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::stop_campaign::builders::StopCampaignFluentBuilder::set_id):<br>required: **true**<br>Identifier representing a Campaign<br>
    /// - On success, responds with [`StopCampaignOutput`](crate::operation::stop_campaign::StopCampaignOutput)
    /// - On failure, responds with [`SdkError<StopCampaignError>`](crate::operation::stop_campaign::StopCampaignError)
    pub fn stop_campaign(&self) -> crate::operation::stop_campaign::builders::StopCampaignFluentBuilder {
        crate::operation::stop_campaign::builders::StopCampaignFluentBuilder::new(self.handle.clone())
    }
}
