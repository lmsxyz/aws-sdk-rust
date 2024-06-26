// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_agent_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_agent::CreateAgentInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.agent_name {
        object.key("agentName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.agent_resource_role_arn {
        object.key("agentResourceRoleArn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.client_token {
        object.key("clientToken").string(var_3.as_str());
    }
    if let Some(var_4) = &input.customer_encryption_key_arn {
        object.key("customerEncryptionKeyArn").string(var_4.as_str());
    }
    if let Some(var_5) = &input.description {
        object.key("description").string(var_5.as_str());
    }
    if let Some(var_6) = &input.foundation_model {
        object.key("foundationModel").string(var_6.as_str());
    }
    if let Some(var_7) = &input.guardrail_configuration {
        #[allow(unused_mut)]
        let mut object_8 = object.key("guardrailConfiguration").start_object();
        crate::protocol_serde::shape_guardrail_configuration::ser_guardrail_configuration(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.idle_session_ttl_in_seconds {
        object.key("idleSessionTTLInSeconds").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_9).into()),
        );
    }
    if let Some(var_10) = &input.instruction {
        object.key("instruction").string(var_10.as_str());
    }
    if let Some(var_11) = &input.prompt_override_configuration {
        #[allow(unused_mut)]
        let mut object_12 = object.key("promptOverrideConfiguration").start_object();
        crate::protocol_serde::shape_prompt_override_configuration::ser_prompt_override_configuration(&mut object_12, var_11)?;
        object_12.finish();
    }
    if let Some(var_13) = &input.tags {
        #[allow(unused_mut)]
        let mut object_14 = object.key("tags").start_object();
        for (key_15, value_16) in var_13 {
            {
                object_14.key(key_15.as_str()).string(value_16.as_str());
            }
        }
        object_14.finish();
    }
    Ok(())
}
