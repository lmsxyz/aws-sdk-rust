// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateApnsVoipSandboxChannel`](crate::operation::update_apns_voip_sandbox_channel::builders::UpdateApnsVoipSandboxChannelFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`apns_voip_sandbox_channel_request(ApnsVoipSandboxChannelRequest)`](crate::operation::update_apns_voip_sandbox_channel::builders::UpdateApnsVoipSandboxChannelFluentBuilder::apns_voip_sandbox_channel_request) / [`set_apns_voip_sandbox_channel_request(Option<ApnsVoipSandboxChannelRequest>)`](crate::operation::update_apns_voip_sandbox_channel::builders::UpdateApnsVoipSandboxChannelFluentBuilder::set_apns_voip_sandbox_channel_request):<br>required: **true**<br><p>Specifies the status and settings of the APNs (Apple Push Notification service) VoIP sandbox channel for an application.</p><br>
    ///   - [`application_id(impl Into<String>)`](crate::operation::update_apns_voip_sandbox_channel::builders::UpdateApnsVoipSandboxChannelFluentBuilder::application_id) / [`set_application_id(Option<String>)`](crate::operation::update_apns_voip_sandbox_channel::builders::UpdateApnsVoipSandboxChannelFluentBuilder::set_application_id):<br>required: **true**<br><p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p><br>
    /// - On success, responds with [`UpdateApnsVoipSandboxChannelOutput`](crate::operation::update_apns_voip_sandbox_channel::UpdateApnsVoipSandboxChannelOutput) with field(s):
    ///   - [`apns_voip_sandbox_channel_response(Option<ApnsVoipSandboxChannelResponse>)`](crate::operation::update_apns_voip_sandbox_channel::UpdateApnsVoipSandboxChannelOutput::apns_voip_sandbox_channel_response): <p>Provides information about the status and settings of the APNs (Apple Push Notification service) VoIP sandbox channel for an application.</p>
    /// - On failure, responds with [`SdkError<UpdateApnsVoipSandboxChannelError>`](crate::operation::update_apns_voip_sandbox_channel::UpdateApnsVoipSandboxChannelError)
    pub fn update_apns_voip_sandbox_channel(
        &self,
    ) -> crate::operation::update_apns_voip_sandbox_channel::builders::UpdateApnsVoipSandboxChannelFluentBuilder {
        crate::operation::update_apns_voip_sandbox_channel::builders::UpdateApnsVoipSandboxChannelFluentBuilder::new(self.handle.clone())
    }
}
