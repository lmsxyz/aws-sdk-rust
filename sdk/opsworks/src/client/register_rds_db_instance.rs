// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`RegisterRdsDbInstance`](crate::operation::register_rds_db_instance::builders::RegisterRdsDbInstanceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`stack_id(impl Into<String>)`](crate::operation::register_rds_db_instance::builders::RegisterRdsDbInstanceFluentBuilder::stack_id) / [`set_stack_id(Option<String>)`](crate::operation::register_rds_db_instance::builders::RegisterRdsDbInstanceFluentBuilder::set_stack_id):<br>required: **true**<br><p>The stack ID.</p><br>
    ///   - [`rds_db_instance_arn(impl Into<String>)`](crate::operation::register_rds_db_instance::builders::RegisterRdsDbInstanceFluentBuilder::rds_db_instance_arn) / [`set_rds_db_instance_arn(Option<String>)`](crate::operation::register_rds_db_instance::builders::RegisterRdsDbInstanceFluentBuilder::set_rds_db_instance_arn):<br>required: **true**<br><p>The Amazon RDS instance's ARN.</p><br>
    ///   - [`db_user(impl Into<String>)`](crate::operation::register_rds_db_instance::builders::RegisterRdsDbInstanceFluentBuilder::db_user) / [`set_db_user(Option<String>)`](crate::operation::register_rds_db_instance::builders::RegisterRdsDbInstanceFluentBuilder::set_db_user):<br>required: **true**<br><p>The database's master user name.</p><br>
    ///   - [`db_password(impl Into<String>)`](crate::operation::register_rds_db_instance::builders::RegisterRdsDbInstanceFluentBuilder::db_password) / [`set_db_password(Option<String>)`](crate::operation::register_rds_db_instance::builders::RegisterRdsDbInstanceFluentBuilder::set_db_password):<br>required: **true**<br><p>The database password.</p><br>
    /// - On success, responds with [`RegisterRdsDbInstanceOutput`](crate::operation::register_rds_db_instance::RegisterRdsDbInstanceOutput)
    /// - On failure, responds with [`SdkError<RegisterRdsDbInstanceError>`](crate::operation::register_rds_db_instance::RegisterRdsDbInstanceError)
    pub fn register_rds_db_instance(&self) -> crate::operation::register_rds_db_instance::builders::RegisterRdsDbInstanceFluentBuilder {
        crate::operation::register_rds_db_instance::builders::RegisterRdsDbInstanceFluentBuilder::new(self.handle.clone())
    }
}
