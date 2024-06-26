// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteBotReplica`](crate::operation::delete_bot_replica::builders::DeleteBotReplicaFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bot_id(impl Into<String>)`](crate::operation::delete_bot_replica::builders::DeleteBotReplicaFluentBuilder::bot_id) / [`set_bot_id(Option<String>)`](crate::operation::delete_bot_replica::builders::DeleteBotReplicaFluentBuilder::set_bot_id):<br>required: **true**<br><p>The unique ID of the replicated bot to be deleted from the secondary region</p><br>
    ///   - [`replica_region(impl Into<String>)`](crate::operation::delete_bot_replica::builders::DeleteBotReplicaFluentBuilder::replica_region) / [`set_replica_region(Option<String>)`](crate::operation::delete_bot_replica::builders::DeleteBotReplicaFluentBuilder::set_replica_region):<br>required: **true**<br><p>The secondary region of the replicated bot that will be deleted.</p><br>
    /// - On success, responds with [`DeleteBotReplicaOutput`](crate::operation::delete_bot_replica::DeleteBotReplicaOutput) with field(s):
    ///   - [`bot_id(Option<String>)`](crate::operation::delete_bot_replica::DeleteBotReplicaOutput::bot_id): <p>The unique bot ID of the replicated bot generated.</p>
    ///   - [`replica_region(Option<String>)`](crate::operation::delete_bot_replica::DeleteBotReplicaOutput::replica_region): <p>The region of the replicated bot generated.</p>
    ///   - [`bot_replica_status(Option<BotReplicaStatus>)`](crate::operation::delete_bot_replica::DeleteBotReplicaOutput::bot_replica_status): <p>The operational status of the replicated bot generated.</p>
    /// - On failure, responds with [`SdkError<DeleteBotReplicaError>`](crate::operation::delete_bot_replica::DeleteBotReplicaError)
    pub fn delete_bot_replica(&self) -> crate::operation::delete_bot_replica::builders::DeleteBotReplicaFluentBuilder {
        crate::operation::delete_bot_replica::builders::DeleteBotReplicaFluentBuilder::new(self.handle.clone())
    }
}
