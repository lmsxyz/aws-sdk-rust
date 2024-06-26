// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`VerifySession`](crate::operation::verify_session::builders::VerifySessionFluentBuilder) operation.
    ///
    /// - The fluent builder takes no input, just [`send`](crate::operation::verify_session::builders::VerifySessionFluentBuilder::send) it.
    /// - On success, responds with [`VerifySessionOutput`](crate::operation::verify_session::VerifySessionOutput) with field(s):
    ///   - [`identity(Option<String>)`](crate::operation::verify_session::VerifySessionOutput::identity): <p>The system-generated unique ID of the user in Amazon CodeCatalyst.</p>
    /// - On failure, responds with [`SdkError<VerifySessionError>`](crate::operation::verify_session::VerifySessionError)
    pub fn verify_session(&self) -> crate::operation::verify_session::builders::VerifySessionFluentBuilder {
        crate::operation::verify_session::builders::VerifySessionFluentBuilder::new(self.handle.clone())
    }
}
