// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutClusterPolicy`](crate::operation::put_cluster_policy::builders::PutClusterPolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`cluster_arn(impl Into<String>)`](crate::operation::put_cluster_policy::builders::PutClusterPolicyFluentBuilder::cluster_arn) / [`set_cluster_arn(Option<String>)`](crate::operation::put_cluster_policy::builders::PutClusterPolicyFluentBuilder::set_cluster_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the cluster.</p><br>
    ///   - [`current_version(impl Into<String>)`](crate::operation::put_cluster_policy::builders::PutClusterPolicyFluentBuilder::current_version) / [`set_current_version(Option<String>)`](crate::operation::put_cluster_policy::builders::PutClusterPolicyFluentBuilder::set_current_version):<br>required: **false**<br><p>The policy version.</p><br>
    ///   - [`policy(impl Into<String>)`](crate::operation::put_cluster_policy::builders::PutClusterPolicyFluentBuilder::policy) / [`set_policy(Option<String>)`](crate::operation::put_cluster_policy::builders::PutClusterPolicyFluentBuilder::set_policy):<br>required: **true**<br><p>The policy.</p><br>
    /// - On success, responds with [`PutClusterPolicyOutput`](crate::operation::put_cluster_policy::PutClusterPolicyOutput) with field(s):
    ///   - [`current_version(Option<String>)`](crate::operation::put_cluster_policy::PutClusterPolicyOutput::current_version): <p>The policy version.</p>
    /// - On failure, responds with [`SdkError<PutClusterPolicyError>`](crate::operation::put_cluster_policy::PutClusterPolicyError)
    pub fn put_cluster_policy(&self) -> crate::operation::put_cluster_policy::builders::PutClusterPolicyFluentBuilder {
        crate::operation::put_cluster_policy::builders::PutClusterPolicyFluentBuilder::new(self.handle.clone())
    }
}
