// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateTemplate`](crate::operation::create_template::builders::CreateTemplateFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`connector_arn(impl Into<String>)`](crate::operation::create_template::builders::CreateTemplateFluentBuilder::connector_arn) / [`set_connector_arn(Option<String>)`](crate::operation::create_template::builders::CreateTemplateFluentBuilder::set_connector_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) that was returned when you called <a href="https://docs.aws.amazon.com/pca-connector-ad/latest/APIReference/API_CreateConnector.html">CreateConnector</a>.</p><br>
    ///   - [`name(impl Into<String>)`](crate::operation::create_template::builders::CreateTemplateFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_template::builders::CreateTemplateFluentBuilder::set_name):<br>required: **true**<br><p>Name of the template. The template name must be unique.</p><br>
    ///   - [`definition(TemplateDefinition)`](crate::operation::create_template::builders::CreateTemplateFluentBuilder::definition) / [`set_definition(Option<TemplateDefinition>)`](crate::operation::create_template::builders::CreateTemplateFluentBuilder::set_definition):<br>required: **true**<br><p>Template configuration to define the information included in certificates. Define certificate validity and renewal periods, certificate request handling and enrollment options, key usage extensions, application policies, and cryptography settings.</p><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::create_template::builders::CreateTemplateFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::create_template::builders::CreateTemplateFluentBuilder::set_client_token):<br>required: **false**<br><p>Idempotency token.</p><br>
    ///   - [`tags(impl Into<String>, impl Into<String>)`](crate::operation::create_template::builders::CreateTemplateFluentBuilder::tags) / [`set_tags(Option<HashMap::<String, String>>)`](crate::operation::create_template::builders::CreateTemplateFluentBuilder::set_tags):<br>required: **false**<br><p>Metadata assigned to a template consisting of a key-value pair.</p><br>
    /// - On success, responds with [`CreateTemplateOutput`](crate::operation::create_template::CreateTemplateOutput) with field(s):
    ///   - [`template_arn(Option<String>)`](crate::operation::create_template::CreateTemplateOutput::template_arn): <p>If successful, the Amazon Resource Name (ARN) of the template.</p>
    /// - On failure, responds with [`SdkError<CreateTemplateError>`](crate::operation::create_template::CreateTemplateError)
    pub fn create_template(&self) -> crate::operation::create_template::builders::CreateTemplateFluentBuilder {
        crate::operation::create_template::builders::CreateTemplateFluentBuilder::new(self.handle.clone())
    }
}
