// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::types::_suspended_state::SuspendedState;

pub use crate::types::_scalable_dimension::ScalableDimension;

pub use crate::types::_service_namespace::ServiceNamespace;

pub use crate::types::_scalable_target_action::ScalableTargetAction;

pub use crate::types::_alarm::Alarm;

pub use crate::types::_target_tracking_scaling_policy_configuration::TargetTrackingScalingPolicyConfiguration;

pub use crate::types::_customized_metric_specification::CustomizedMetricSpecification;

pub use crate::types::_target_tracking_metric_data_query::TargetTrackingMetricDataQuery;

pub use crate::types::_target_tracking_metric_stat::TargetTrackingMetricStat;

pub use crate::types::_target_tracking_metric::TargetTrackingMetric;

pub use crate::types::_target_tracking_metric_dimension::TargetTrackingMetricDimension;

pub use crate::types::_metric_statistic::MetricStatistic;

pub use crate::types::_metric_dimension::MetricDimension;

pub use crate::types::_predefined_metric_specification::PredefinedMetricSpecification;

pub use crate::types::_metric_type::MetricType;

pub use crate::types::_step_scaling_policy_configuration::StepScalingPolicyConfiguration;

pub use crate::types::_metric_aggregation_type::MetricAggregationType;

pub use crate::types::_step_adjustment::StepAdjustment;

pub use crate::types::_adjustment_type::AdjustmentType;

pub use crate::types::_policy_type::PolicyType;

pub use crate::types::_scheduled_action::ScheduledAction;

pub use crate::types::_scaling_policy::ScalingPolicy;

pub use crate::types::_scaling_activity::ScalingActivity;

pub use crate::types::_not_scaled_reason::NotScaledReason;

pub use crate::types::_scaling_activity_status_code::ScalingActivityStatusCode;

pub use crate::types::_scalable_target::ScalableTarget;

mod _adjustment_type;

mod _alarm;

mod _customized_metric_specification;

mod _metric_aggregation_type;

mod _metric_dimension;

mod _metric_statistic;

mod _metric_type;

mod _not_scaled_reason;

mod _policy_type;

mod _predefined_metric_specification;

mod _scalable_dimension;

mod _scalable_target;

mod _scalable_target_action;

mod _scaling_activity;

mod _scaling_activity_status_code;

mod _scaling_policy;

mod _scheduled_action;

mod _service_namespace;

mod _step_adjustment;

mod _step_scaling_policy_configuration;

mod _suspended_state;

mod _target_tracking_metric;

mod _target_tracking_metric_data_query;

mod _target_tracking_metric_dimension;

mod _target_tracking_metric_stat;

mod _target_tracking_scaling_policy_configuration;

/// Builders
pub mod builders;

/// Error types that Application Auto Scaling can respond with.
pub mod error;
