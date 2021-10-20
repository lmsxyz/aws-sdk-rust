// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_experiment_template_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateExperimentTemplateInput,
) {
    if let Some(var_1) = &input.actions {
        let mut object_2 = object.key("actions").start_object();
        for (key_3, value_4) in var_1 {
            {
                let mut object_5 = object_2.key(key_3).start_object();
                crate::json_ser::serialize_structure_crate_model_create_experiment_template_action_input(&mut object_5, value_4);
                object_5.finish();
            }
        }
        object_2.finish();
    }
    if let Some(var_6) = &input.client_token {
        object.key("clientToken").string(var_6);
    }
    if let Some(var_7) = &input.description {
        object.key("description").string(var_7);
    }
    if let Some(var_8) = &input.role_arn {
        object.key("roleArn").string(var_8);
    }
    if let Some(var_9) = &input.stop_conditions {
        let mut array_10 = object.key("stopConditions").start_array();
        for item_11 in var_9 {
            {
                let mut object_12 = array_10.value().start_object();
                crate::json_ser::serialize_structure_crate_model_create_experiment_template_stop_condition_input(&mut object_12, item_11);
                object_12.finish();
            }
        }
        array_10.finish();
    }
    if let Some(var_13) = &input.tags {
        let mut object_14 = object.key("tags").start_object();
        for (key_15, value_16) in var_13 {
            {
                object_14.key(key_15).string(value_16);
            }
        }
        object_14.finish();
    }
    if let Some(var_17) = &input.targets {
        let mut object_18 = object.key("targets").start_object();
        for (key_19, value_20) in var_17 {
            {
                let mut object_21 = object_18.key(key_19).start_object();
                crate::json_ser::serialize_structure_crate_model_create_experiment_template_target_input(&mut object_21, value_20);
                object_21.finish();
            }
        }
        object_18.finish();
    }
}

pub fn serialize_structure_crate_input_start_experiment_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartExperimentInput,
) {
    if let Some(var_22) = &input.client_token {
        object.key("clientToken").string(var_22);
    }
    if let Some(var_23) = &input.experiment_template_id {
        object.key("experimentTemplateId").string(var_23);
    }
    if let Some(var_24) = &input.tags {
        let mut object_25 = object.key("tags").start_object();
        for (key_26, value_27) in var_24 {
            {
                object_25.key(key_26).string(value_27);
            }
        }
        object_25.finish();
    }
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) {
    if let Some(var_28) = &input.tags {
        let mut object_29 = object.key("tags").start_object();
        for (key_30, value_31) in var_28 {
            {
                object_29.key(key_30).string(value_31);
            }
        }
        object_29.finish();
    }
}

pub fn serialize_structure_crate_input_update_experiment_template_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateExperimentTemplateInput,
) {
    if let Some(var_32) = &input.actions {
        let mut object_33 = object.key("actions").start_object();
        for (key_34, value_35) in var_32 {
            {
                let mut object_36 = object_33.key(key_34).start_object();
                crate::json_ser::serialize_structure_crate_model_update_experiment_template_action_input_item(&mut object_36, value_35);
                object_36.finish();
            }
        }
        object_33.finish();
    }
    if let Some(var_37) = &input.description {
        object.key("description").string(var_37);
    }
    if let Some(var_38) = &input.role_arn {
        object.key("roleArn").string(var_38);
    }
    if let Some(var_39) = &input.stop_conditions {
        let mut array_40 = object.key("stopConditions").start_array();
        for item_41 in var_39 {
            {
                let mut object_42 = array_40.value().start_object();
                crate::json_ser::serialize_structure_crate_model_update_experiment_template_stop_condition_input(&mut object_42, item_41);
                object_42.finish();
            }
        }
        array_40.finish();
    }
    if let Some(var_43) = &input.targets {
        let mut object_44 = object.key("targets").start_object();
        for (key_45, value_46) in var_43 {
            {
                let mut object_47 = object_44.key(key_45).start_object();
                crate::json_ser::serialize_structure_crate_model_update_experiment_template_target_input(&mut object_47, value_46);
                object_47.finish();
            }
        }
        object_44.finish();
    }
}

pub fn serialize_structure_crate_model_create_experiment_template_action_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CreateExperimentTemplateActionInput,
) {
    if let Some(var_48) = &input.action_id {
        object.key("actionId").string(var_48);
    }
    if let Some(var_49) = &input.description {
        object.key("description").string(var_49);
    }
    if let Some(var_50) = &input.parameters {
        let mut object_51 = object.key("parameters").start_object();
        for (key_52, value_53) in var_50 {
            {
                object_51.key(key_52).string(value_53);
            }
        }
        object_51.finish();
    }
    if let Some(var_54) = &input.targets {
        let mut object_55 = object.key("targets").start_object();
        for (key_56, value_57) in var_54 {
            {
                object_55.key(key_56).string(value_57);
            }
        }
        object_55.finish();
    }
    if let Some(var_58) = &input.start_after {
        let mut array_59 = object.key("startAfter").start_array();
        for item_60 in var_58 {
            {
                array_59.value().string(item_60);
            }
        }
        array_59.finish();
    }
}

pub fn serialize_structure_crate_model_create_experiment_template_stop_condition_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CreateExperimentTemplateStopConditionInput,
) {
    if let Some(var_61) = &input.source {
        object.key("source").string(var_61);
    }
    if let Some(var_62) = &input.value {
        object.key("value").string(var_62);
    }
}

pub fn serialize_structure_crate_model_create_experiment_template_target_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CreateExperimentTemplateTargetInput,
) {
    if let Some(var_63) = &input.resource_type {
        object.key("resourceType").string(var_63);
    }
    if let Some(var_64) = &input.resource_arns {
        let mut array_65 = object.key("resourceArns").start_array();
        for item_66 in var_64 {
            {
                array_65.value().string(item_66);
            }
        }
        array_65.finish();
    }
    if let Some(var_67) = &input.resource_tags {
        let mut object_68 = object.key("resourceTags").start_object();
        for (key_69, value_70) in var_67 {
            {
                object_68.key(key_69).string(value_70);
            }
        }
        object_68.finish();
    }
    if let Some(var_71) = &input.filters {
        let mut array_72 = object.key("filters").start_array();
        for item_73 in var_71 {
            {
                let mut object_74 = array_72.value().start_object();
                crate::json_ser::serialize_structure_crate_model_experiment_template_target_input_filter(&mut object_74, item_73);
                object_74.finish();
            }
        }
        array_72.finish();
    }
    if let Some(var_75) = &input.selection_mode {
        object.key("selectionMode").string(var_75);
    }
}

pub fn serialize_structure_crate_model_update_experiment_template_action_input_item(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::UpdateExperimentTemplateActionInputItem,
) {
    if let Some(var_76) = &input.action_id {
        object.key("actionId").string(var_76);
    }
    if let Some(var_77) = &input.description {
        object.key("description").string(var_77);
    }
    if let Some(var_78) = &input.parameters {
        let mut object_79 = object.key("parameters").start_object();
        for (key_80, value_81) in var_78 {
            {
                object_79.key(key_80).string(value_81);
            }
        }
        object_79.finish();
    }
    if let Some(var_82) = &input.targets {
        let mut object_83 = object.key("targets").start_object();
        for (key_84, value_85) in var_82 {
            {
                object_83.key(key_84).string(value_85);
            }
        }
        object_83.finish();
    }
    if let Some(var_86) = &input.start_after {
        let mut array_87 = object.key("startAfter").start_array();
        for item_88 in var_86 {
            {
                array_87.value().string(item_88);
            }
        }
        array_87.finish();
    }
}

pub fn serialize_structure_crate_model_update_experiment_template_stop_condition_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::UpdateExperimentTemplateStopConditionInput,
) {
    if let Some(var_89) = &input.source {
        object.key("source").string(var_89);
    }
    if let Some(var_90) = &input.value {
        object.key("value").string(var_90);
    }
}

pub fn serialize_structure_crate_model_update_experiment_template_target_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::UpdateExperimentTemplateTargetInput,
) {
    if let Some(var_91) = &input.resource_type {
        object.key("resourceType").string(var_91);
    }
    if let Some(var_92) = &input.resource_arns {
        let mut array_93 = object.key("resourceArns").start_array();
        for item_94 in var_92 {
            {
                array_93.value().string(item_94);
            }
        }
        array_93.finish();
    }
    if let Some(var_95) = &input.resource_tags {
        let mut object_96 = object.key("resourceTags").start_object();
        for (key_97, value_98) in var_95 {
            {
                object_96.key(key_97).string(value_98);
            }
        }
        object_96.finish();
    }
    if let Some(var_99) = &input.filters {
        let mut array_100 = object.key("filters").start_array();
        for item_101 in var_99 {
            {
                let mut object_102 = array_100.value().start_object();
                crate::json_ser::serialize_structure_crate_model_experiment_template_target_input_filter(&mut object_102, item_101);
                object_102.finish();
            }
        }
        array_100.finish();
    }
    if let Some(var_103) = &input.selection_mode {
        object.key("selectionMode").string(var_103);
    }
}

pub fn serialize_structure_crate_model_experiment_template_target_input_filter(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ExperimentTemplateTargetInputFilter,
) {
    if let Some(var_104) = &input.path {
        object.key("path").string(var_104);
    }
    if let Some(var_105) = &input.values {
        let mut array_106 = object.key("values").start_array();
        for item_107 in var_105 {
            {
                array_106.value().string(item_107);
            }
        }
        array_106.finish();
    }
}
