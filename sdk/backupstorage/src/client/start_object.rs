// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StartObject`](crate::operation::start_object::builders::StartObjectFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`backup_job_id(impl Into<String>)`](crate::operation::start_object::builders::StartObjectFluentBuilder::backup_job_id) / [`set_backup_job_id(Option<String>)`](crate::operation::start_object::builders::StartObjectFluentBuilder::set_backup_job_id):<br>required: **true**<br>Backup job Id for the in-progress backup<br>
    ///   - [`object_name(impl Into<String>)`](crate::operation::start_object::builders::StartObjectFluentBuilder::object_name) / [`set_object_name(Option<String>)`](crate::operation::start_object::builders::StartObjectFluentBuilder::set_object_name):<br>required: **true**<br>Name for the object.<br>
    ///   - [`throw_on_duplicate(bool)`](crate::operation::start_object::builders::StartObjectFluentBuilder::throw_on_duplicate) / [`set_throw_on_duplicate(Option<bool>)`](crate::operation::start_object::builders::StartObjectFluentBuilder::set_throw_on_duplicate):<br>required: **false**<br>Throw an exception if Object name is already exist.<br>
    /// - On success, responds with [`StartObjectOutput`](crate::operation::start_object::StartObjectOutput) with field(s):
    ///   - [`upload_id(String)`](crate::operation::start_object::StartObjectOutput::upload_id): Upload Id for a given upload.
    /// - On failure, responds with [`SdkError<StartObjectError>`](crate::operation::start_object::StartObjectError)
    pub fn start_object(&self) -> crate::operation::start_object::builders::StartObjectFluentBuilder {
        crate::operation::start_object::builders::StartObjectFluentBuilder::new(self.handle.clone())
    }
}
