// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateSPICECapacityConfiguration`](crate::operation::update_spice_capacity_configuration::builders::UpdateSPICECapacityConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`aws_account_id(impl Into<String>)`](crate::operation::update_spice_capacity_configuration::builders::UpdateSPICECapacityConfigurationFluentBuilder::aws_account_id) / [`set_aws_account_id(Option<String>)`](crate::operation::update_spice_capacity_configuration::builders::UpdateSPICECapacityConfigurationFluentBuilder::set_aws_account_id):<br>required: **true**<br><p>The ID of the Amazon Web Services account that contains the SPICE configuration that you want to update.</p><br>
    ///   - [`purchase_mode(PurchaseMode)`](crate::operation::update_spice_capacity_configuration::builders::UpdateSPICECapacityConfigurationFluentBuilder::purchase_mode) / [`set_purchase_mode(Option<PurchaseMode>)`](crate::operation::update_spice_capacity_configuration::builders::UpdateSPICECapacityConfigurationFluentBuilder::set_purchase_mode):<br>required: **true**<br><p>Determines how SPICE capacity can be purchased. The following options are available.</p> <ul>  <li>   <p><code>MANUAL</code>: SPICE capacity can only be purchased manually.</p></li>  <li>   <p><code>AUTO_PURCHASE</code>: Extra SPICE capacity is automatically purchased on your behalf as needed. SPICE capacity can also be purchased manually with this option.</p></li> </ul><br>
    /// - On success, responds with [`UpdateSpiceCapacityConfigurationOutput`](crate::operation::update_spice_capacity_configuration::UpdateSpiceCapacityConfigurationOutput) with field(s):
    ///   - [`request_id(Option<String>)`](crate::operation::update_spice_capacity_configuration::UpdateSpiceCapacityConfigurationOutput::request_id): <p>The Amazon Web Services request ID for this operation.</p>
    ///   - [`status(i32)`](crate::operation::update_spice_capacity_configuration::UpdateSpiceCapacityConfigurationOutput::status): <p>The HTTP status of the request.</p>
    /// - On failure, responds with [`SdkError<UpdateSPICECapacityConfigurationError>`](crate::operation::update_spice_capacity_configuration::UpdateSPICECapacityConfigurationError)
    pub fn update_spice_capacity_configuration(
        &self,
    ) -> crate::operation::update_spice_capacity_configuration::builders::UpdateSPICECapacityConfigurationFluentBuilder {
        crate::operation::update_spice_capacity_configuration::builders::UpdateSPICECapacityConfigurationFluentBuilder::new(self.handle.clone())
    }
}
