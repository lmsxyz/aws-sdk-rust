// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateRelay`](crate::operation::create_relay::builders::CreateRelayFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`client_token(impl Into<String>)`](crate::operation::create_relay::builders::CreateRelayFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::create_relay::builders::CreateRelayFluentBuilder::set_client_token):<br>required: **false**<br><p>A unique token that Amazon SES uses to recognize subsequent retries of the same request.</p><br>
    ///   - [`relay_name(impl Into<String>)`](crate::operation::create_relay::builders::CreateRelayFluentBuilder::relay_name) / [`set_relay_name(Option<String>)`](crate::operation::create_relay::builders::CreateRelayFluentBuilder::set_relay_name):<br>required: **true**<br><p>The unique name of the relay resource.</p><br>
    ///   - [`server_name(impl Into<String>)`](crate::operation::create_relay::builders::CreateRelayFluentBuilder::server_name) / [`set_server_name(Option<String>)`](crate::operation::create_relay::builders::CreateRelayFluentBuilder::set_server_name):<br>required: **true**<br><p>The destination relay server address.</p><br>
    ///   - [`server_port(i32)`](crate::operation::create_relay::builders::CreateRelayFluentBuilder::server_port) / [`set_server_port(Option<i32>)`](crate::operation::create_relay::builders::CreateRelayFluentBuilder::set_server_port):<br>required: **true**<br><p>The destination relay server port.</p><br>
    ///   - [`authentication(RelayAuthentication)`](crate::operation::create_relay::builders::CreateRelayFluentBuilder::authentication) / [`set_authentication(Option<RelayAuthentication>)`](crate::operation::create_relay::builders::CreateRelayFluentBuilder::set_authentication):<br>required: **true**<br><p>Authentication for the relay destination server—specify the secretARN where the SMTP credentials are stored.</p><br>
    ///   - [`tags(Tag)`](crate::operation::create_relay::builders::CreateRelayFluentBuilder::tags) / [`set_tags(Option<Vec::<Tag>>)`](crate::operation::create_relay::builders::CreateRelayFluentBuilder::set_tags):<br>required: **false**<br><p>The tags used to organize, track, or control access for the resource. For example, { "tags": {"key1":"value1", "key2":"value2"} }.</p><br>
    /// - On success, responds with [`CreateRelayOutput`](crate::operation::create_relay::CreateRelayOutput) with field(s):
    ///   - [`relay_id(String)`](crate::operation::create_relay::CreateRelayOutput::relay_id): <p>A unique identifier of the created relay resource.</p>
    /// - On failure, responds with [`SdkError<CreateRelayError>`](crate::operation::create_relay::CreateRelayError)
    pub fn create_relay(&self) -> crate::operation::create_relay::builders::CreateRelayFluentBuilder {
        crate::operation::create_relay::builders::CreateRelayFluentBuilder::new(self.handle.clone())
    }
}
