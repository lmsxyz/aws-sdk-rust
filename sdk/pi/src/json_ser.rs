// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_describe_dimension_keys_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeDimensionKeysInput,
) {
    if let Some(var_1) = &input.service_type {
        object.key("ServiceType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.identifier {
        object.key("Identifier").string(var_2);
    }
    if let Some(var_3) = &input.start_time {
        object
            .key("StartTime")
            .instant(var_3, aws_smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_4) = &input.end_time {
        object
            .key("EndTime")
            .instant(var_4, aws_smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_5) = &input.metric {
        object.key("Metric").string(var_5);
    }
    if let Some(var_6) = &input.period_in_seconds {
        object.key("PeriodInSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_6).into()),
        );
    }
    if let Some(var_7) = &input.group_by {
        let mut object_8 = object.key("GroupBy").start_object();
        crate::json_ser::serialize_structure_crate_model_dimension_group(&mut object_8, var_7);
        object_8.finish();
    }
    if let Some(var_9) = &input.partition_by {
        let mut object_10 = object.key("PartitionBy").start_object();
        crate::json_ser::serialize_structure_crate_model_dimension_group(&mut object_10, var_9);
        object_10.finish();
    }
    if let Some(var_11) = &input.filter {
        let mut object_12 = object.key("Filter").start_object();
        for (key_13, value_14) in var_11 {
            {
                object_12.key(key_13).string(value_14);
            }
        }
        object_12.finish();
    }
    if let Some(var_15) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_15).into()),
        );
    }
    if let Some(var_16) = &input.next_token {
        object.key("NextToken").string(var_16);
    }
}

pub fn serialize_structure_crate_input_get_dimension_key_details_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetDimensionKeyDetailsInput,
) {
    if let Some(var_17) = &input.service_type {
        object.key("ServiceType").string(var_17.as_str());
    }
    if let Some(var_18) = &input.identifier {
        object.key("Identifier").string(var_18);
    }
    if let Some(var_19) = &input.group {
        object.key("Group").string(var_19);
    }
    if let Some(var_20) = &input.group_identifier {
        object.key("GroupIdentifier").string(var_20);
    }
    if let Some(var_21) = &input.requested_dimensions {
        let mut array_22 = object.key("RequestedDimensions").start_array();
        for item_23 in var_21 {
            {
                array_22.value().string(item_23);
            }
        }
        array_22.finish();
    }
}

pub fn serialize_structure_crate_input_get_resource_metrics_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetResourceMetricsInput,
) {
    if let Some(var_24) = &input.service_type {
        object.key("ServiceType").string(var_24.as_str());
    }
    if let Some(var_25) = &input.identifier {
        object.key("Identifier").string(var_25);
    }
    if let Some(var_26) = &input.metric_queries {
        let mut array_27 = object.key("MetricQueries").start_array();
        for item_28 in var_26 {
            {
                let mut object_29 = array_27.value().start_object();
                crate::json_ser::serialize_structure_crate_model_metric_query(
                    &mut object_29,
                    item_28,
                );
                object_29.finish();
            }
        }
        array_27.finish();
    }
    if let Some(var_30) = &input.start_time {
        object
            .key("StartTime")
            .instant(var_30, aws_smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_31) = &input.end_time {
        object
            .key("EndTime")
            .instant(var_31, aws_smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_32) = &input.period_in_seconds {
        object.key("PeriodInSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_32).into()),
        );
    }
    if let Some(var_33) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_33).into()),
        );
    }
    if let Some(var_34) = &input.next_token {
        object.key("NextToken").string(var_34);
    }
}

pub fn serialize_structure_crate_model_dimension_group(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DimensionGroup,
) {
    if let Some(var_35) = &input.group {
        object.key("Group").string(var_35);
    }
    if let Some(var_36) = &input.dimensions {
        let mut array_37 = object.key("Dimensions").start_array();
        for item_38 in var_36 {
            {
                array_37.value().string(item_38);
            }
        }
        array_37.finish();
    }
    if let Some(var_39) = &input.limit {
        object.key("Limit").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_39).into()),
        );
    }
}

pub fn serialize_structure_crate_model_metric_query(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::MetricQuery,
) {
    if let Some(var_40) = &input.metric {
        object.key("Metric").string(var_40);
    }
    if let Some(var_41) = &input.group_by {
        let mut object_42 = object.key("GroupBy").start_object();
        crate::json_ser::serialize_structure_crate_model_dimension_group(&mut object_42, var_41);
        object_42.finish();
    }
    if let Some(var_43) = &input.filter {
        let mut object_44 = object.key("Filter").start_object();
        for (key_45, value_46) in var_43 {
            {
                object_44.key(key_45).string(value_46);
            }
        }
        object_44.finish();
    }
}
