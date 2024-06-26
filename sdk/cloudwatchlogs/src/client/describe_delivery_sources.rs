// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeDeliverySources`](crate::operation::describe_delivery_sources::builders::DescribeDeliverySourcesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_delivery_sources::builders::DescribeDeliverySourcesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_delivery_sources::builders::DescribeDeliverySourcesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_delivery_sources::builders::DescribeDeliverySourcesFluentBuilder::set_next_token):<br>required: **false**<br><p>The token for the next set of items to return. The token expires after 24 hours.</p><br>
    ///   - [`limit(i32)`](crate::operation::describe_delivery_sources::builders::DescribeDeliverySourcesFluentBuilder::limit) / [`set_limit(Option<i32>)`](crate::operation::describe_delivery_sources::builders::DescribeDeliverySourcesFluentBuilder::set_limit):<br>required: **false**<br><p>Optionally specify the maximum number of delivery sources to return in the response.</p><br>
    /// - On success, responds with [`DescribeDeliverySourcesOutput`](crate::operation::describe_delivery_sources::DescribeDeliverySourcesOutput) with field(s):
    ///   - [`delivery_sources(Option<Vec::<DeliverySource>>)`](crate::operation::describe_delivery_sources::DescribeDeliverySourcesOutput::delivery_sources): <p>An array of structures. Each structure contains information about one delivery source in the account.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_delivery_sources::DescribeDeliverySourcesOutput::next_token): <p>The token for the next set of items to return. The token expires after 24 hours.</p>
    /// - On failure, responds with [`SdkError<DescribeDeliverySourcesError>`](crate::operation::describe_delivery_sources::DescribeDeliverySourcesError)
    pub fn describe_delivery_sources(&self) -> crate::operation::describe_delivery_sources::builders::DescribeDeliverySourcesFluentBuilder {
        crate::operation::describe_delivery_sources::builders::DescribeDeliverySourcesFluentBuilder::new(self.handle.clone())
    }
}
