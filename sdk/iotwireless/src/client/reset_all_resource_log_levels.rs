// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ResetAllResourceLogLevels`](crate::operation::reset_all_resource_log_levels::builders::ResetAllResourceLogLevelsFluentBuilder) operation.
    ///
    /// - The fluent builder takes no input, just [`send`](crate::operation::reset_all_resource_log_levels::builders::ResetAllResourceLogLevelsFluentBuilder::send) it.
    /// - On success, responds with [`ResetAllResourceLogLevelsOutput`](crate::operation::reset_all_resource_log_levels::ResetAllResourceLogLevelsOutput)
    /// - On failure, responds with [`SdkError<ResetAllResourceLogLevelsError>`](crate::operation::reset_all_resource_log_levels::ResetAllResourceLogLevelsError)
    pub fn reset_all_resource_log_levels(&self) -> crate::operation::reset_all_resource_log_levels::builders::ResetAllResourceLogLevelsFluentBuilder {
        crate::operation::reset_all_resource_log_levels::builders::ResetAllResourceLogLevelsFluentBuilder::new(self.handle.clone())
    }
}
