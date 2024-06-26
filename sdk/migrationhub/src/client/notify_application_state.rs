// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`NotifyApplicationState`](crate::operation::notify_application_state::builders::NotifyApplicationStateFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`application_id(impl Into<String>)`](crate::operation::notify_application_state::builders::NotifyApplicationStateFluentBuilder::application_id) / [`set_application_id(Option<String>)`](crate::operation::notify_application_state::builders::NotifyApplicationStateFluentBuilder::set_application_id):<br>required: **true**<br><p>The configurationId in Application Discovery Service that uniquely identifies the grouped application.</p><br>
    ///   - [`status(ApplicationStatus)`](crate::operation::notify_application_state::builders::NotifyApplicationStateFluentBuilder::status) / [`set_status(Option<ApplicationStatus>)`](crate::operation::notify_application_state::builders::NotifyApplicationStateFluentBuilder::set_status):<br>required: **true**<br><p>Status of the application - Not Started, In-Progress, Complete.</p><br>
    ///   - [`update_date_time(DateTime)`](crate::operation::notify_application_state::builders::NotifyApplicationStateFluentBuilder::update_date_time) / [`set_update_date_time(Option<DateTime>)`](crate::operation::notify_application_state::builders::NotifyApplicationStateFluentBuilder::set_update_date_time):<br>required: **false**<br><p>The timestamp when the application state changed.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::notify_application_state::builders::NotifyApplicationStateFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::notify_application_state::builders::NotifyApplicationStateFluentBuilder::set_dry_run):<br>required: **false**<br><p>Optional boolean flag to indicate whether any effect should take place. Used to test if the caller has permission to make the call.</p><br>
    /// - On success, responds with [`NotifyApplicationStateOutput`](crate::operation::notify_application_state::NotifyApplicationStateOutput)
    /// - On failure, responds with [`SdkError<NotifyApplicationStateError>`](crate::operation::notify_application_state::NotifyApplicationStateError)
    pub fn notify_application_state(&self) -> crate::operation::notify_application_state::builders::NotifyApplicationStateFluentBuilder {
        crate::operation::notify_application_state::builders::NotifyApplicationStateFluentBuilder::new(self.handle.clone())
    }
}
