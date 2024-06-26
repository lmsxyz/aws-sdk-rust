// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateSchemaMapping`](crate::operation::update_schema_mapping::builders::UpdateSchemaMappingFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`schema_name(impl Into<String>)`](crate::operation::update_schema_mapping::builders::UpdateSchemaMappingFluentBuilder::schema_name) / [`set_schema_name(Option<String>)`](crate::operation::update_schema_mapping::builders::UpdateSchemaMappingFluentBuilder::set_schema_name):<br>required: **true**<br><p>The name of the schema. There can't be multiple <code>SchemaMappings</code> with the same name.</p><br>
    ///   - [`description(impl Into<String>)`](crate::operation::update_schema_mapping::builders::UpdateSchemaMappingFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_schema_mapping::builders::UpdateSchemaMappingFluentBuilder::set_description):<br>required: **false**<br><p>A description of the schema.</p><br>
    ///   - [`mapped_input_fields(SchemaInputAttribute)`](crate::operation::update_schema_mapping::builders::UpdateSchemaMappingFluentBuilder::mapped_input_fields) / [`set_mapped_input_fields(Option<Vec::<SchemaInputAttribute>>)`](crate::operation::update_schema_mapping::builders::UpdateSchemaMappingFluentBuilder::set_mapped_input_fields):<br>required: **true**<br><p>A list of <code>MappedInputFields</code>. Each <code>MappedInputField</code> corresponds to a column the source data table, and contains column name plus additional information that Entity Resolution uses for matching.</p><br>
    /// - On success, responds with [`UpdateSchemaMappingOutput`](crate::operation::update_schema_mapping::UpdateSchemaMappingOutput) with field(s):
    ///   - [`schema_name(String)`](crate::operation::update_schema_mapping::UpdateSchemaMappingOutput::schema_name): <p>The name of the schema.</p>
    ///   - [`schema_arn(String)`](crate::operation::update_schema_mapping::UpdateSchemaMappingOutput::schema_arn): <p>The ARN (Amazon Resource Name) that Entity Resolution generated for the <code>SchemaMapping</code>.</p>
    ///   - [`description(Option<String>)`](crate::operation::update_schema_mapping::UpdateSchemaMappingOutput::description): <p>A description of the schema.</p>
    ///   - [`mapped_input_fields(Vec::<SchemaInputAttribute>)`](crate::operation::update_schema_mapping::UpdateSchemaMappingOutput::mapped_input_fields): <p>A list of <code>MappedInputFields</code>. Each <code>MappedInputField</code> corresponds to a column the source data table, and contains column name plus additional information that Entity Resolution uses for matching.</p>
    /// - On failure, responds with [`SdkError<UpdateSchemaMappingError>`](crate::operation::update_schema_mapping::UpdateSchemaMappingError)
    pub fn update_schema_mapping(&self) -> crate::operation::update_schema_mapping::builders::UpdateSchemaMappingFluentBuilder {
        crate::operation::update_schema_mapping::builders::UpdateSchemaMappingFluentBuilder::new(self.handle.clone())
    }
}
