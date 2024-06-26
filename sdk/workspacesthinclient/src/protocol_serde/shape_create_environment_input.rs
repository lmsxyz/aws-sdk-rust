// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_environment_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_environment::CreateEnvironmentInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.client_token {
        object.key("clientToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.desired_software_set_id {
        object.key("desiredSoftwareSetId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.desktop_arn {
        object.key("desktopArn").string(var_3.as_str());
    }
    if let Some(var_4) = &input.desktop_endpoint {
        object.key("desktopEndpoint").string(var_4.as_str());
    }
    if let Some(var_5) = &input.kms_key_arn {
        object.key("kmsKeyArn").string(var_5.as_str());
    }
    if let Some(var_6) = &input.maintenance_window {
        #[allow(unused_mut)]
        let mut object_7 = object.key("maintenanceWindow").start_object();
        crate::protocol_serde::shape_maintenance_window::ser_maintenance_window(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.name {
        object.key("name").string(var_8.as_str());
    }
    if let Some(var_9) = &input.software_set_update_mode {
        object.key("softwareSetUpdateMode").string(var_9.as_str());
    }
    if let Some(var_10) = &input.software_set_update_schedule {
        object.key("softwareSetUpdateSchedule").string(var_10.as_str());
    }
    if let Some(var_11) = &input.tags {
        #[allow(unused_mut)]
        let mut object_12 = object.key("tags").start_object();
        for (key_13, value_14) in var_11 {
            {
                object_12.key(key_13.as_str()).string(value_14.as_str());
            }
        }
        object_12.finish();
    }
    Ok(())
}
