// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn serialize_structure_crate_model_dimension(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::Dimension,
) {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Name");
    if let Some(var_2) = &input.name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Value");
    if let Some(var_4) = &input.value {
        scope_3.string(var_4);
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_metric_data_query(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::MetricDataQuery,
) {
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("Id");
    if let Some(var_6) = &input.id {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("MetricStat");
    if let Some(var_8) = &input.metric_stat {
        crate::query_ser::serialize_structure_crate_model_metric_stat(scope_7, var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("Expression");
    if let Some(var_10) = &input.expression {
        scope_9.string(var_10);
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("Label");
    if let Some(var_12) = &input.label {
        scope_11.string(var_12);
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("ReturnData");
    if let Some(var_14) = &input.return_data {
        scope_13.boolean(*var_14);
    }
    #[allow(unused_mut)]
    let mut scope_15 = writer.prefix("Period");
    if let Some(var_16) = &input.period {
        scope_15.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_16).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_17 = writer.prefix("AccountId");
    if let Some(var_18) = &input.account_id {
        scope_17.string(var_18);
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_label_options(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::LabelOptions,
) {
    #[allow(unused_mut)]
    let mut scope_19 = writer.prefix("Timezone");
    if let Some(var_20) = &input.timezone {
        scope_19.string(var_20);
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_dimension_filter(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::DimensionFilter,
) {
    #[allow(unused_mut)]
    let mut scope_21 = writer.prefix("Name");
    if let Some(var_22) = &input.name {
        scope_21.string(var_22);
    }
    #[allow(unused_mut)]
    let mut scope_23 = writer.prefix("Value");
    if let Some(var_24) = &input.value {
        scope_23.string(var_24);
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_anomaly_detector_configuration(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::AnomalyDetectorConfiguration,
) {
    #[allow(unused_mut)]
    let mut scope_25 = writer.prefix("ExcludedTimeRanges");
    if let Some(var_26) = &input.excluded_time_ranges {
        let mut list_28 = scope_25.start_list(false, None);
        for item_27 in var_26 {
            #[allow(unused_mut)]
            let mut entry_29 = list_28.entry();
            crate::query_ser::serialize_structure_crate_model_range(entry_29, item_27);
        }
        list_28.finish();
    }
    #[allow(unused_mut)]
    let mut scope_30 = writer.prefix("MetricTimezone");
    if let Some(var_31) = &input.metric_timezone {
        scope_30.string(var_31);
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_tag(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::Tag,
) {
    #[allow(unused_mut)]
    let mut scope_32 = writer.prefix("Key");
    if let Some(var_33) = &input.key {
        scope_32.string(var_33);
    }
    #[allow(unused_mut)]
    let mut scope_34 = writer.prefix("Value");
    if let Some(var_35) = &input.value {
        scope_34.string(var_35);
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_metric_datum(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::MetricDatum,
) {
    #[allow(unused_mut)]
    let mut scope_36 = writer.prefix("MetricName");
    if let Some(var_37) = &input.metric_name {
        scope_36.string(var_37);
    }
    #[allow(unused_mut)]
    let mut scope_38 = writer.prefix("Dimensions");
    if let Some(var_39) = &input.dimensions {
        let mut list_41 = scope_38.start_list(false, None);
        for item_40 in var_39 {
            #[allow(unused_mut)]
            let mut entry_42 = list_41.entry();
            crate::query_ser::serialize_structure_crate_model_dimension(entry_42, item_40);
        }
        list_41.finish();
    }
    #[allow(unused_mut)]
    let mut scope_43 = writer.prefix("Timestamp");
    if let Some(var_44) = &input.timestamp {
        scope_43.instant(var_44, aws_smithy_types::instant::Format::DateTime);
    }
    #[allow(unused_mut)]
    let mut scope_45 = writer.prefix("Value");
    if let Some(var_46) = &input.value {
        scope_45.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::Float((*var_46).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_47 = writer.prefix("StatisticValues");
    if let Some(var_48) = &input.statistic_values {
        crate::query_ser::serialize_structure_crate_model_statistic_set(scope_47, var_48);
    }
    #[allow(unused_mut)]
    let mut scope_49 = writer.prefix("Values");
    if let Some(var_50) = &input.values {
        let mut list_52 = scope_49.start_list(false, None);
        for item_51 in var_50 {
            #[allow(unused_mut)]
            let mut entry_53 = list_52.entry();
            entry_53.number(
                #[allow(clippy::useless_conversion)]
                aws_smithy_types::Number::Float((*item_51).into()),
            );
        }
        list_52.finish();
    }
    #[allow(unused_mut)]
    let mut scope_54 = writer.prefix("Counts");
    if let Some(var_55) = &input.counts {
        let mut list_57 = scope_54.start_list(false, None);
        for item_56 in var_55 {
            #[allow(unused_mut)]
            let mut entry_58 = list_57.entry();
            entry_58.number(
                #[allow(clippy::useless_conversion)]
                aws_smithy_types::Number::Float((*item_56).into()),
            );
        }
        list_57.finish();
    }
    #[allow(unused_mut)]
    let mut scope_59 = writer.prefix("Unit");
    if let Some(var_60) = &input.unit {
        scope_59.string(var_60.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_61 = writer.prefix("StorageResolution");
    if let Some(var_62) = &input.storage_resolution {
        scope_61.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_62).into()),
        );
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_metric_stream_filter(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::MetricStreamFilter,
) {
    #[allow(unused_mut)]
    let mut scope_63 = writer.prefix("Namespace");
    if let Some(var_64) = &input.namespace {
        scope_63.string(var_64);
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_metric_stat(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::MetricStat,
) {
    #[allow(unused_mut)]
    let mut scope_65 = writer.prefix("Metric");
    if let Some(var_66) = &input.metric {
        crate::query_ser::serialize_structure_crate_model_metric(scope_65, var_66);
    }
    #[allow(unused_mut)]
    let mut scope_67 = writer.prefix("Period");
    if let Some(var_68) = &input.period {
        scope_67.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_68).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_69 = writer.prefix("Stat");
    if let Some(var_70) = &input.stat {
        scope_69.string(var_70);
    }
    #[allow(unused_mut)]
    let mut scope_71 = writer.prefix("Unit");
    if let Some(var_72) = &input.unit {
        scope_71.string(var_72.as_str());
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_range(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::Range,
) {
    #[allow(unused_mut)]
    let mut scope_73 = writer.prefix("StartTime");
    if let Some(var_74) = &input.start_time {
        scope_73.instant(var_74, aws_smithy_types::instant::Format::DateTime);
    }
    #[allow(unused_mut)]
    let mut scope_75 = writer.prefix("EndTime");
    if let Some(var_76) = &input.end_time {
        scope_75.instant(var_76, aws_smithy_types::instant::Format::DateTime);
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_statistic_set(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::StatisticSet,
) {
    #[allow(unused_mut)]
    let mut scope_77 = writer.prefix("SampleCount");
    if let Some(var_78) = &input.sample_count {
        scope_77.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::Float((*var_78).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_79 = writer.prefix("Sum");
    if let Some(var_80) = &input.sum {
        scope_79.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::Float((*var_80).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_81 = writer.prefix("Minimum");
    if let Some(var_82) = &input.minimum {
        scope_81.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::Float((*var_82).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_83 = writer.prefix("Maximum");
    if let Some(var_84) = &input.maximum {
        scope_83.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::Float((*var_84).into()),
        );
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_metric(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::Metric,
) {
    #[allow(unused_mut)]
    let mut scope_85 = writer.prefix("Namespace");
    if let Some(var_86) = &input.namespace {
        scope_85.string(var_86);
    }
    #[allow(unused_mut)]
    let mut scope_87 = writer.prefix("MetricName");
    if let Some(var_88) = &input.metric_name {
        scope_87.string(var_88);
    }
    #[allow(unused_mut)]
    let mut scope_89 = writer.prefix("Dimensions");
    if let Some(var_90) = &input.dimensions {
        let mut list_92 = scope_89.start_list(false, None);
        for item_91 in var_90 {
            #[allow(unused_mut)]
            let mut entry_93 = list_92.entry();
            crate::query_ser::serialize_structure_crate_model_dimension(entry_93, item_91);
        }
        list_92.finish();
    }
}
