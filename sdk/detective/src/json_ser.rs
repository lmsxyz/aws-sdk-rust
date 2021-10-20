// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_accept_invitation_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AcceptInvitationInput,
) {
    if let Some(var_1) = &input.graph_arn {
        object.key("GraphArn").string(var_1);
    }
}

pub fn serialize_structure_crate_input_create_graph_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateGraphInput,
) {
    if let Some(var_2) = &input.tags {
        let mut object_3 = object.key("Tags").start_object();
        for (key_4, value_5) in var_2 {
            {
                object_3.key(key_4).string(value_5);
            }
        }
        object_3.finish();
    }
}

pub fn serialize_structure_crate_input_create_members_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateMembersInput,
) {
    if let Some(var_6) = &input.accounts {
        let mut array_7 = object.key("Accounts").start_array();
        for item_8 in var_6 {
            {
                let mut object_9 = array_7.value().start_object();
                crate::json_ser::serialize_structure_crate_model_account(&mut object_9, item_8);
                object_9.finish();
            }
        }
        array_7.finish();
    }
    if input.disable_email_notification {
        object
            .key("DisableEmailNotification")
            .boolean(input.disable_email_notification);
    }
    if let Some(var_10) = &input.graph_arn {
        object.key("GraphArn").string(var_10);
    }
    if let Some(var_11) = &input.message {
        object.key("Message").string(var_11);
    }
}

pub fn serialize_structure_crate_input_delete_graph_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteGraphInput,
) {
    if let Some(var_12) = &input.graph_arn {
        object.key("GraphArn").string(var_12);
    }
}

pub fn serialize_structure_crate_input_delete_members_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteMembersInput,
) {
    if let Some(var_13) = &input.account_ids {
        let mut array_14 = object.key("AccountIds").start_array();
        for item_15 in var_13 {
            {
                array_14.value().string(item_15);
            }
        }
        array_14.finish();
    }
    if let Some(var_16) = &input.graph_arn {
        object.key("GraphArn").string(var_16);
    }
}

pub fn serialize_structure_crate_input_disassociate_membership_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DisassociateMembershipInput,
) {
    if let Some(var_17) = &input.graph_arn {
        object.key("GraphArn").string(var_17);
    }
}

pub fn serialize_structure_crate_input_get_members_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetMembersInput,
) {
    if let Some(var_18) = &input.account_ids {
        let mut array_19 = object.key("AccountIds").start_array();
        for item_20 in var_18 {
            {
                array_19.value().string(item_20);
            }
        }
        array_19.finish();
    }
    if let Some(var_21) = &input.graph_arn {
        object.key("GraphArn").string(var_21);
    }
}

pub fn serialize_structure_crate_input_list_graphs_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListGraphsInput,
) {
    if let Some(var_22) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_22).into()),
        );
    }
    if let Some(var_23) = &input.next_token {
        object.key("NextToken").string(var_23);
    }
}

pub fn serialize_structure_crate_input_list_invitations_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListInvitationsInput,
) {
    if let Some(var_24) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_24).into()),
        );
    }
    if let Some(var_25) = &input.next_token {
        object.key("NextToken").string(var_25);
    }
}

pub fn serialize_structure_crate_input_list_members_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListMembersInput,
) {
    if let Some(var_26) = &input.graph_arn {
        object.key("GraphArn").string(var_26);
    }
    if let Some(var_27) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_27).into()),
        );
    }
    if let Some(var_28) = &input.next_token {
        object.key("NextToken").string(var_28);
    }
}

pub fn serialize_structure_crate_input_reject_invitation_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RejectInvitationInput,
) {
    if let Some(var_29) = &input.graph_arn {
        object.key("GraphArn").string(var_29);
    }
}

pub fn serialize_structure_crate_input_start_monitoring_member_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartMonitoringMemberInput,
) {
    if let Some(var_30) = &input.account_id {
        object.key("AccountId").string(var_30);
    }
    if let Some(var_31) = &input.graph_arn {
        object.key("GraphArn").string(var_31);
    }
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) {
    if let Some(var_32) = &input.tags {
        let mut object_33 = object.key("Tags").start_object();
        for (key_34, value_35) in var_32 {
            {
                object_33.key(key_34).string(value_35);
            }
        }
        object_33.finish();
    }
}

pub fn serialize_structure_crate_model_account(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Account,
) {
    if let Some(var_36) = &input.account_id {
        object.key("AccountId").string(var_36);
    }
    if let Some(var_37) = &input.email_address {
        object.key("EmailAddress").string(var_37);
    }
}
