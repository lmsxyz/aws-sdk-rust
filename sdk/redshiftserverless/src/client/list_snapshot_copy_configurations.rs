// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListSnapshotCopyConfigurations`](crate::operation::list_snapshot_copy_configurations::builders::ListSnapshotCopyConfigurationsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_snapshot_copy_configurations::builders::ListSnapshotCopyConfigurationsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`namespace_name(impl Into<String>)`](crate::operation::list_snapshot_copy_configurations::builders::ListSnapshotCopyConfigurationsFluentBuilder::namespace_name) / [`set_namespace_name(Option<String>)`](crate::operation::list_snapshot_copy_configurations::builders::ListSnapshotCopyConfigurationsFluentBuilder::set_namespace_name):<br>required: **false**<br><p>The namespace from which to list all snapshot copy configurations.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_snapshot_copy_configurations::builders::ListSnapshotCopyConfigurationsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_snapshot_copy_configurations::builders::ListSnapshotCopyConfigurationsFluentBuilder::set_next_token):<br>required: **false**<br><p>If <code>nextToken</code> is returned, there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_snapshot_copy_configurations::builders::ListSnapshotCopyConfigurationsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_snapshot_copy_configurations::builders::ListSnapshotCopyConfigurationsFluentBuilder::set_max_results):<br>required: **false**<br><p>An optional parameter that specifies the maximum number of results to return. You can use <code>nextToken</code> to display the next page of results.</p><br>
    /// - On success, responds with [`ListSnapshotCopyConfigurationsOutput`](crate::operation::list_snapshot_copy_configurations::ListSnapshotCopyConfigurationsOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::operation::list_snapshot_copy_configurations::ListSnapshotCopyConfigurationsOutput::next_token): <p>If <code>nextToken</code> is returned, there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page.</p>
    ///   - [`snapshot_copy_configurations(Vec::<SnapshotCopyConfiguration>)`](crate::operation::list_snapshot_copy_configurations::ListSnapshotCopyConfigurationsOutput::snapshot_copy_configurations): <p>All of the returned snapshot copy configurations.</p>
    /// - On failure, responds with [`SdkError<ListSnapshotCopyConfigurationsError>`](crate::operation::list_snapshot_copy_configurations::ListSnapshotCopyConfigurationsError)
    pub fn list_snapshot_copy_configurations(
        &self,
    ) -> crate::operation::list_snapshot_copy_configurations::builders::ListSnapshotCopyConfigurationsFluentBuilder {
        crate::operation::list_snapshot_copy_configurations::builders::ListSnapshotCopyConfigurationsFluentBuilder::new(self.handle.clone())
    }
}
