// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeSharedVpcConfiguration`](crate::operation::describe_shared_vpc_configuration::builders::DescribeSharedVpcConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder takes no input, just [`send`](crate::operation::describe_shared_vpc_configuration::builders::DescribeSharedVpcConfigurationFluentBuilder::send) it.
    /// - On success, responds with [`DescribeSharedVpcConfigurationOutput`](crate::operation::describe_shared_vpc_configuration::DescribeSharedVpcConfigurationOutput) with field(s):
    ///   - [`enable_fsx_route_table_updates_from_participant_accounts(Option<String>)`](crate::operation::describe_shared_vpc_configuration::DescribeSharedVpcConfigurationOutput::enable_fsx_route_table_updates_from_participant_accounts): <p>Indicates whether participant accounts can create FSx for ONTAP Multi-AZ file systems in shared subnets.</p>
    /// - On failure, responds with [`SdkError<DescribeSharedVpcConfigurationError>`](crate::operation::describe_shared_vpc_configuration::DescribeSharedVpcConfigurationError)
    pub fn describe_shared_vpc_configuration(
        &self,
    ) -> crate::operation::describe_shared_vpc_configuration::builders::DescribeSharedVpcConfigurationFluentBuilder {
        crate::operation::describe_shared_vpc_configuration::builders::DescribeSharedVpcConfigurationFluentBuilder::new(self.handle.clone())
    }
}
