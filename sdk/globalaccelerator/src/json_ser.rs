// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_add_custom_routing_endpoints_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AddCustomRoutingEndpointsInput,
) {
    if let Some(var_1) = &input.endpoint_configurations {
        let mut array_2 = object.key("EndpointConfigurations").start_array();
        for item_3 in var_1 {
            {
                let mut object_4 = array_2.value().start_object();
                crate::json_ser::serialize_structure_crate_model_custom_routing_endpoint_configuration(&mut object_4, item_3);
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.endpoint_group_arn {
        object.key("EndpointGroupArn").string(var_5);
    }
}

pub fn serialize_structure_crate_input_advertise_byoip_cidr_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AdvertiseByoipCidrInput,
) {
    if let Some(var_6) = &input.cidr {
        object.key("Cidr").string(var_6);
    }
}

pub fn serialize_structure_crate_input_allow_custom_routing_traffic_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AllowCustomRoutingTrafficInput,
) {
    if let Some(var_7) = &input.endpoint_group_arn {
        object.key("EndpointGroupArn").string(var_7);
    }
    if let Some(var_8) = &input.endpoint_id {
        object.key("EndpointId").string(var_8);
    }
    if let Some(var_9) = &input.destination_addresses {
        let mut array_10 = object.key("DestinationAddresses").start_array();
        for item_11 in var_9 {
            {
                array_10.value().string(item_11);
            }
        }
        array_10.finish();
    }
    if let Some(var_12) = &input.destination_ports {
        let mut array_13 = object.key("DestinationPorts").start_array();
        for item_14 in var_12 {
            {
                array_13.value().number(
                    #[allow(clippy::useless_conversion)]
                    aws_smithy_types::Number::NegInt((*item_14).into()),
                );
            }
        }
        array_13.finish();
    }
    if let Some(var_15) = &input.allow_all_traffic_to_endpoint {
        object.key("AllowAllTrafficToEndpoint").boolean(*var_15);
    }
}

pub fn serialize_structure_crate_input_create_accelerator_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateAcceleratorInput,
) {
    if let Some(var_16) = &input.name {
        object.key("Name").string(var_16);
    }
    if let Some(var_17) = &input.ip_address_type {
        object.key("IpAddressType").string(var_17.as_str());
    }
    if let Some(var_18) = &input.ip_addresses {
        let mut array_19 = object.key("IpAddresses").start_array();
        for item_20 in var_18 {
            {
                array_19.value().string(item_20);
            }
        }
        array_19.finish();
    }
    if let Some(var_21) = &input.enabled {
        object.key("Enabled").boolean(*var_21);
    }
    if let Some(var_22) = &input.idempotency_token {
        object.key("IdempotencyToken").string(var_22);
    }
    if let Some(var_23) = &input.tags {
        let mut array_24 = object.key("Tags").start_array();
        for item_25 in var_23 {
            {
                let mut object_26 = array_24.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_26, item_25);
                object_26.finish();
            }
        }
        array_24.finish();
    }
}

pub fn serialize_structure_crate_input_create_custom_routing_accelerator_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateCustomRoutingAcceleratorInput,
) {
    if let Some(var_27) = &input.name {
        object.key("Name").string(var_27);
    }
    if let Some(var_28) = &input.ip_address_type {
        object.key("IpAddressType").string(var_28.as_str());
    }
    if let Some(var_29) = &input.ip_addresses {
        let mut array_30 = object.key("IpAddresses").start_array();
        for item_31 in var_29 {
            {
                array_30.value().string(item_31);
            }
        }
        array_30.finish();
    }
    if let Some(var_32) = &input.enabled {
        object.key("Enabled").boolean(*var_32);
    }
    if let Some(var_33) = &input.idempotency_token {
        object.key("IdempotencyToken").string(var_33);
    }
    if let Some(var_34) = &input.tags {
        let mut array_35 = object.key("Tags").start_array();
        for item_36 in var_34 {
            {
                let mut object_37 = array_35.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_37, item_36);
                object_37.finish();
            }
        }
        array_35.finish();
    }
}

pub fn serialize_structure_crate_input_create_custom_routing_endpoint_group_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateCustomRoutingEndpointGroupInput,
) {
    if let Some(var_38) = &input.listener_arn {
        object.key("ListenerArn").string(var_38);
    }
    if let Some(var_39) = &input.endpoint_group_region {
        object.key("EndpointGroupRegion").string(var_39);
    }
    if let Some(var_40) = &input.destination_configurations {
        let mut array_41 = object.key("DestinationConfigurations").start_array();
        for item_42 in var_40 {
            {
                let mut object_43 = array_41.value().start_object();
                crate::json_ser::serialize_structure_crate_model_custom_routing_destination_configuration(&mut object_43, item_42);
                object_43.finish();
            }
        }
        array_41.finish();
    }
    if let Some(var_44) = &input.idempotency_token {
        object.key("IdempotencyToken").string(var_44);
    }
}

pub fn serialize_structure_crate_input_create_custom_routing_listener_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateCustomRoutingListenerInput,
) {
    if let Some(var_45) = &input.accelerator_arn {
        object.key("AcceleratorArn").string(var_45);
    }
    if let Some(var_46) = &input.port_ranges {
        let mut array_47 = object.key("PortRanges").start_array();
        for item_48 in var_46 {
            {
                let mut object_49 = array_47.value().start_object();
                crate::json_ser::serialize_structure_crate_model_port_range(
                    &mut object_49,
                    item_48,
                );
                object_49.finish();
            }
        }
        array_47.finish();
    }
    if let Some(var_50) = &input.idempotency_token {
        object.key("IdempotencyToken").string(var_50);
    }
}

pub fn serialize_structure_crate_input_create_endpoint_group_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateEndpointGroupInput,
) {
    if let Some(var_51) = &input.listener_arn {
        object.key("ListenerArn").string(var_51);
    }
    if let Some(var_52) = &input.endpoint_group_region {
        object.key("EndpointGroupRegion").string(var_52);
    }
    if let Some(var_53) = &input.endpoint_configurations {
        let mut array_54 = object.key("EndpointConfigurations").start_array();
        for item_55 in var_53 {
            {
                let mut object_56 = array_54.value().start_object();
                crate::json_ser::serialize_structure_crate_model_endpoint_configuration(
                    &mut object_56,
                    item_55,
                );
                object_56.finish();
            }
        }
        array_54.finish();
    }
    if let Some(var_57) = &input.traffic_dial_percentage {
        object.key("TrafficDialPercentage").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::Float((*var_57).into()),
        );
    }
    if let Some(var_58) = &input.health_check_port {
        object.key("HealthCheckPort").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_58).into()),
        );
    }
    if let Some(var_59) = &input.health_check_protocol {
        object.key("HealthCheckProtocol").string(var_59.as_str());
    }
    if let Some(var_60) = &input.health_check_path {
        object.key("HealthCheckPath").string(var_60);
    }
    if let Some(var_61) = &input.health_check_interval_seconds {
        object.key("HealthCheckIntervalSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_61).into()),
        );
    }
    if let Some(var_62) = &input.threshold_count {
        object.key("ThresholdCount").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_62).into()),
        );
    }
    if let Some(var_63) = &input.idempotency_token {
        object.key("IdempotencyToken").string(var_63);
    }
    if let Some(var_64) = &input.port_overrides {
        let mut array_65 = object.key("PortOverrides").start_array();
        for item_66 in var_64 {
            {
                let mut object_67 = array_65.value().start_object();
                crate::json_ser::serialize_structure_crate_model_port_override(
                    &mut object_67,
                    item_66,
                );
                object_67.finish();
            }
        }
        array_65.finish();
    }
}

pub fn serialize_structure_crate_input_create_listener_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateListenerInput,
) {
    if let Some(var_68) = &input.accelerator_arn {
        object.key("AcceleratorArn").string(var_68);
    }
    if let Some(var_69) = &input.port_ranges {
        let mut array_70 = object.key("PortRanges").start_array();
        for item_71 in var_69 {
            {
                let mut object_72 = array_70.value().start_object();
                crate::json_ser::serialize_structure_crate_model_port_range(
                    &mut object_72,
                    item_71,
                );
                object_72.finish();
            }
        }
        array_70.finish();
    }
    if let Some(var_73) = &input.protocol {
        object.key("Protocol").string(var_73.as_str());
    }
    if let Some(var_74) = &input.client_affinity {
        object.key("ClientAffinity").string(var_74.as_str());
    }
    if let Some(var_75) = &input.idempotency_token {
        object.key("IdempotencyToken").string(var_75);
    }
}

pub fn serialize_structure_crate_input_delete_accelerator_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteAcceleratorInput,
) {
    if let Some(var_76) = &input.accelerator_arn {
        object.key("AcceleratorArn").string(var_76);
    }
}

pub fn serialize_structure_crate_input_delete_custom_routing_accelerator_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteCustomRoutingAcceleratorInput,
) {
    if let Some(var_77) = &input.accelerator_arn {
        object.key("AcceleratorArn").string(var_77);
    }
}

pub fn serialize_structure_crate_input_delete_custom_routing_endpoint_group_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteCustomRoutingEndpointGroupInput,
) {
    if let Some(var_78) = &input.endpoint_group_arn {
        object.key("EndpointGroupArn").string(var_78);
    }
}

pub fn serialize_structure_crate_input_delete_custom_routing_listener_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteCustomRoutingListenerInput,
) {
    if let Some(var_79) = &input.listener_arn {
        object.key("ListenerArn").string(var_79);
    }
}

pub fn serialize_structure_crate_input_delete_endpoint_group_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteEndpointGroupInput,
) {
    if let Some(var_80) = &input.endpoint_group_arn {
        object.key("EndpointGroupArn").string(var_80);
    }
}

pub fn serialize_structure_crate_input_delete_listener_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteListenerInput,
) {
    if let Some(var_81) = &input.listener_arn {
        object.key("ListenerArn").string(var_81);
    }
}

pub fn serialize_structure_crate_input_deny_custom_routing_traffic_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DenyCustomRoutingTrafficInput,
) {
    if let Some(var_82) = &input.endpoint_group_arn {
        object.key("EndpointGroupArn").string(var_82);
    }
    if let Some(var_83) = &input.endpoint_id {
        object.key("EndpointId").string(var_83);
    }
    if let Some(var_84) = &input.destination_addresses {
        let mut array_85 = object.key("DestinationAddresses").start_array();
        for item_86 in var_84 {
            {
                array_85.value().string(item_86);
            }
        }
        array_85.finish();
    }
    if let Some(var_87) = &input.destination_ports {
        let mut array_88 = object.key("DestinationPorts").start_array();
        for item_89 in var_87 {
            {
                array_88.value().number(
                    #[allow(clippy::useless_conversion)]
                    aws_smithy_types::Number::NegInt((*item_89).into()),
                );
            }
        }
        array_88.finish();
    }
    if let Some(var_90) = &input.deny_all_traffic_to_endpoint {
        object.key("DenyAllTrafficToEndpoint").boolean(*var_90);
    }
}

pub fn serialize_structure_crate_input_deprovision_byoip_cidr_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeprovisionByoipCidrInput,
) {
    if let Some(var_91) = &input.cidr {
        object.key("Cidr").string(var_91);
    }
}

pub fn serialize_structure_crate_input_describe_accelerator_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeAcceleratorInput,
) {
    if let Some(var_92) = &input.accelerator_arn {
        object.key("AcceleratorArn").string(var_92);
    }
}

pub fn serialize_structure_crate_input_describe_accelerator_attributes_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeAcceleratorAttributesInput,
) {
    if let Some(var_93) = &input.accelerator_arn {
        object.key("AcceleratorArn").string(var_93);
    }
}

pub fn serialize_structure_crate_input_describe_custom_routing_accelerator_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeCustomRoutingAcceleratorInput,
) {
    if let Some(var_94) = &input.accelerator_arn {
        object.key("AcceleratorArn").string(var_94);
    }
}

pub fn serialize_structure_crate_input_describe_custom_routing_accelerator_attributes_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeCustomRoutingAcceleratorAttributesInput,
) {
    if let Some(var_95) = &input.accelerator_arn {
        object.key("AcceleratorArn").string(var_95);
    }
}

pub fn serialize_structure_crate_input_describe_custom_routing_endpoint_group_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeCustomRoutingEndpointGroupInput,
) {
    if let Some(var_96) = &input.endpoint_group_arn {
        object.key("EndpointGroupArn").string(var_96);
    }
}

pub fn serialize_structure_crate_input_describe_custom_routing_listener_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeCustomRoutingListenerInput,
) {
    if let Some(var_97) = &input.listener_arn {
        object.key("ListenerArn").string(var_97);
    }
}

pub fn serialize_structure_crate_input_describe_endpoint_group_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeEndpointGroupInput,
) {
    if let Some(var_98) = &input.endpoint_group_arn {
        object.key("EndpointGroupArn").string(var_98);
    }
}

pub fn serialize_structure_crate_input_describe_listener_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeListenerInput,
) {
    if let Some(var_99) = &input.listener_arn {
        object.key("ListenerArn").string(var_99);
    }
}

pub fn serialize_structure_crate_input_list_accelerators_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListAcceleratorsInput,
) {
    if let Some(var_100) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_100).into()),
        );
    }
    if let Some(var_101) = &input.next_token {
        object.key("NextToken").string(var_101);
    }
}

pub fn serialize_structure_crate_input_list_byoip_cidrs_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListByoipCidrsInput,
) {
    if let Some(var_102) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_102).into()),
        );
    }
    if let Some(var_103) = &input.next_token {
        object.key("NextToken").string(var_103);
    }
}

pub fn serialize_structure_crate_input_list_custom_routing_accelerators_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListCustomRoutingAcceleratorsInput,
) {
    if let Some(var_104) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_104).into()),
        );
    }
    if let Some(var_105) = &input.next_token {
        object.key("NextToken").string(var_105);
    }
}

pub fn serialize_structure_crate_input_list_custom_routing_endpoint_groups_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListCustomRoutingEndpointGroupsInput,
) {
    if let Some(var_106) = &input.listener_arn {
        object.key("ListenerArn").string(var_106);
    }
    if let Some(var_107) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_107).into()),
        );
    }
    if let Some(var_108) = &input.next_token {
        object.key("NextToken").string(var_108);
    }
}

pub fn serialize_structure_crate_input_list_custom_routing_listeners_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListCustomRoutingListenersInput,
) {
    if let Some(var_109) = &input.accelerator_arn {
        object.key("AcceleratorArn").string(var_109);
    }
    if let Some(var_110) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_110).into()),
        );
    }
    if let Some(var_111) = &input.next_token {
        object.key("NextToken").string(var_111);
    }
}

pub fn serialize_structure_crate_input_list_custom_routing_port_mappings_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListCustomRoutingPortMappingsInput,
) {
    if let Some(var_112) = &input.accelerator_arn {
        object.key("AcceleratorArn").string(var_112);
    }
    if let Some(var_113) = &input.endpoint_group_arn {
        object.key("EndpointGroupArn").string(var_113);
    }
    if let Some(var_114) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_114).into()),
        );
    }
    if let Some(var_115) = &input.next_token {
        object.key("NextToken").string(var_115);
    }
}

pub fn serialize_structure_crate_input_list_custom_routing_port_mappings_by_destination_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListCustomRoutingPortMappingsByDestinationInput,
) {
    if let Some(var_116) = &input.endpoint_id {
        object.key("EndpointId").string(var_116);
    }
    if let Some(var_117) = &input.destination_address {
        object.key("DestinationAddress").string(var_117);
    }
    if let Some(var_118) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_118).into()),
        );
    }
    if let Some(var_119) = &input.next_token {
        object.key("NextToken").string(var_119);
    }
}

pub fn serialize_structure_crate_input_list_endpoint_groups_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListEndpointGroupsInput,
) {
    if let Some(var_120) = &input.listener_arn {
        object.key("ListenerArn").string(var_120);
    }
    if let Some(var_121) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_121).into()),
        );
    }
    if let Some(var_122) = &input.next_token {
        object.key("NextToken").string(var_122);
    }
}

pub fn serialize_structure_crate_input_list_listeners_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListListenersInput,
) {
    if let Some(var_123) = &input.accelerator_arn {
        object.key("AcceleratorArn").string(var_123);
    }
    if let Some(var_124) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_124).into()),
        );
    }
    if let Some(var_125) = &input.next_token {
        object.key("NextToken").string(var_125);
    }
}

pub fn serialize_structure_crate_input_list_tags_for_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTagsForResourceInput,
) {
    if let Some(var_126) = &input.resource_arn {
        object.key("ResourceArn").string(var_126);
    }
}

pub fn serialize_structure_crate_input_provision_byoip_cidr_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ProvisionByoipCidrInput,
) {
    if let Some(var_127) = &input.cidr {
        object.key("Cidr").string(var_127);
    }
    if let Some(var_128) = &input.cidr_authorization_context {
        let mut object_129 = object.key("CidrAuthorizationContext").start_object();
        crate::json_ser::serialize_structure_crate_model_cidr_authorization_context(
            &mut object_129,
            var_128,
        );
        object_129.finish();
    }
}

pub fn serialize_structure_crate_input_remove_custom_routing_endpoints_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RemoveCustomRoutingEndpointsInput,
) {
    if let Some(var_130) = &input.endpoint_ids {
        let mut array_131 = object.key("EndpointIds").start_array();
        for item_132 in var_130 {
            {
                array_131.value().string(item_132);
            }
        }
        array_131.finish();
    }
    if let Some(var_133) = &input.endpoint_group_arn {
        object.key("EndpointGroupArn").string(var_133);
    }
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) {
    if let Some(var_134) = &input.resource_arn {
        object.key("ResourceArn").string(var_134);
    }
    if let Some(var_135) = &input.tags {
        let mut array_136 = object.key("Tags").start_array();
        for item_137 in var_135 {
            {
                let mut object_138 = array_136.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_138, item_137);
                object_138.finish();
            }
        }
        array_136.finish();
    }
}

pub fn serialize_structure_crate_input_untag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UntagResourceInput,
) {
    if let Some(var_139) = &input.resource_arn {
        object.key("ResourceArn").string(var_139);
    }
    if let Some(var_140) = &input.tag_keys {
        let mut array_141 = object.key("TagKeys").start_array();
        for item_142 in var_140 {
            {
                array_141.value().string(item_142);
            }
        }
        array_141.finish();
    }
}

pub fn serialize_structure_crate_input_update_accelerator_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateAcceleratorInput,
) {
    if let Some(var_143) = &input.accelerator_arn {
        object.key("AcceleratorArn").string(var_143);
    }
    if let Some(var_144) = &input.name {
        object.key("Name").string(var_144);
    }
    if let Some(var_145) = &input.ip_address_type {
        object.key("IpAddressType").string(var_145.as_str());
    }
    if let Some(var_146) = &input.enabled {
        object.key("Enabled").boolean(*var_146);
    }
}

pub fn serialize_structure_crate_input_update_accelerator_attributes_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateAcceleratorAttributesInput,
) {
    if let Some(var_147) = &input.accelerator_arn {
        object.key("AcceleratorArn").string(var_147);
    }
    if let Some(var_148) = &input.flow_logs_enabled {
        object.key("FlowLogsEnabled").boolean(*var_148);
    }
    if let Some(var_149) = &input.flow_logs_s3_bucket {
        object.key("FlowLogsS3Bucket").string(var_149);
    }
    if let Some(var_150) = &input.flow_logs_s3_prefix {
        object.key("FlowLogsS3Prefix").string(var_150);
    }
}

pub fn serialize_structure_crate_input_update_custom_routing_accelerator_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateCustomRoutingAcceleratorInput,
) {
    if let Some(var_151) = &input.accelerator_arn {
        object.key("AcceleratorArn").string(var_151);
    }
    if let Some(var_152) = &input.name {
        object.key("Name").string(var_152);
    }
    if let Some(var_153) = &input.ip_address_type {
        object.key("IpAddressType").string(var_153.as_str());
    }
    if let Some(var_154) = &input.enabled {
        object.key("Enabled").boolean(*var_154);
    }
}

pub fn serialize_structure_crate_input_update_custom_routing_accelerator_attributes_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateCustomRoutingAcceleratorAttributesInput,
) {
    if let Some(var_155) = &input.accelerator_arn {
        object.key("AcceleratorArn").string(var_155);
    }
    if let Some(var_156) = &input.flow_logs_enabled {
        object.key("FlowLogsEnabled").boolean(*var_156);
    }
    if let Some(var_157) = &input.flow_logs_s3_bucket {
        object.key("FlowLogsS3Bucket").string(var_157);
    }
    if let Some(var_158) = &input.flow_logs_s3_prefix {
        object.key("FlowLogsS3Prefix").string(var_158);
    }
}

pub fn serialize_structure_crate_input_update_custom_routing_listener_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateCustomRoutingListenerInput,
) {
    if let Some(var_159) = &input.listener_arn {
        object.key("ListenerArn").string(var_159);
    }
    if let Some(var_160) = &input.port_ranges {
        let mut array_161 = object.key("PortRanges").start_array();
        for item_162 in var_160 {
            {
                let mut object_163 = array_161.value().start_object();
                crate::json_ser::serialize_structure_crate_model_port_range(
                    &mut object_163,
                    item_162,
                );
                object_163.finish();
            }
        }
        array_161.finish();
    }
}

pub fn serialize_structure_crate_input_update_endpoint_group_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateEndpointGroupInput,
) {
    if let Some(var_164) = &input.endpoint_group_arn {
        object.key("EndpointGroupArn").string(var_164);
    }
    if let Some(var_165) = &input.endpoint_configurations {
        let mut array_166 = object.key("EndpointConfigurations").start_array();
        for item_167 in var_165 {
            {
                let mut object_168 = array_166.value().start_object();
                crate::json_ser::serialize_structure_crate_model_endpoint_configuration(
                    &mut object_168,
                    item_167,
                );
                object_168.finish();
            }
        }
        array_166.finish();
    }
    if let Some(var_169) = &input.traffic_dial_percentage {
        object.key("TrafficDialPercentage").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::Float((*var_169).into()),
        );
    }
    if let Some(var_170) = &input.health_check_port {
        object.key("HealthCheckPort").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_170).into()),
        );
    }
    if let Some(var_171) = &input.health_check_protocol {
        object.key("HealthCheckProtocol").string(var_171.as_str());
    }
    if let Some(var_172) = &input.health_check_path {
        object.key("HealthCheckPath").string(var_172);
    }
    if let Some(var_173) = &input.health_check_interval_seconds {
        object.key("HealthCheckIntervalSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_173).into()),
        );
    }
    if let Some(var_174) = &input.threshold_count {
        object.key("ThresholdCount").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_174).into()),
        );
    }
    if let Some(var_175) = &input.port_overrides {
        let mut array_176 = object.key("PortOverrides").start_array();
        for item_177 in var_175 {
            {
                let mut object_178 = array_176.value().start_object();
                crate::json_ser::serialize_structure_crate_model_port_override(
                    &mut object_178,
                    item_177,
                );
                object_178.finish();
            }
        }
        array_176.finish();
    }
}

pub fn serialize_structure_crate_input_update_listener_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateListenerInput,
) {
    if let Some(var_179) = &input.listener_arn {
        object.key("ListenerArn").string(var_179);
    }
    if let Some(var_180) = &input.port_ranges {
        let mut array_181 = object.key("PortRanges").start_array();
        for item_182 in var_180 {
            {
                let mut object_183 = array_181.value().start_object();
                crate::json_ser::serialize_structure_crate_model_port_range(
                    &mut object_183,
                    item_182,
                );
                object_183.finish();
            }
        }
        array_181.finish();
    }
    if let Some(var_184) = &input.protocol {
        object.key("Protocol").string(var_184.as_str());
    }
    if let Some(var_185) = &input.client_affinity {
        object.key("ClientAffinity").string(var_185.as_str());
    }
}

pub fn serialize_structure_crate_input_withdraw_byoip_cidr_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::WithdrawByoipCidrInput,
) {
    if let Some(var_186) = &input.cidr {
        object.key("Cidr").string(var_186);
    }
}

pub fn serialize_structure_crate_model_custom_routing_endpoint_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CustomRoutingEndpointConfiguration,
) {
    if let Some(var_187) = &input.endpoint_id {
        object.key("EndpointId").string(var_187);
    }
}

pub fn serialize_structure_crate_model_tag(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) {
    if let Some(var_188) = &input.key {
        object.key("Key").string(var_188);
    }
    if let Some(var_189) = &input.value {
        object.key("Value").string(var_189);
    }
}

pub fn serialize_structure_crate_model_custom_routing_destination_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CustomRoutingDestinationConfiguration,
) {
    if let Some(var_190) = &input.from_port {
        object.key("FromPort").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_190).into()),
        );
    }
    if let Some(var_191) = &input.to_port {
        object.key("ToPort").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_191).into()),
        );
    }
    if let Some(var_192) = &input.protocols {
        let mut array_193 = object.key("Protocols").start_array();
        for item_194 in var_192 {
            {
                array_193.value().string(item_194.as_str());
            }
        }
        array_193.finish();
    }
}

pub fn serialize_structure_crate_model_port_range(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PortRange,
) {
    if let Some(var_195) = &input.from_port {
        object.key("FromPort").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_195).into()),
        );
    }
    if let Some(var_196) = &input.to_port {
        object.key("ToPort").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_196).into()),
        );
    }
}

pub fn serialize_structure_crate_model_endpoint_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EndpointConfiguration,
) {
    if let Some(var_197) = &input.endpoint_id {
        object.key("EndpointId").string(var_197);
    }
    if let Some(var_198) = &input.weight {
        object.key("Weight").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_198).into()),
        );
    }
    if let Some(var_199) = &input.client_ip_preservation_enabled {
        object.key("ClientIPPreservationEnabled").boolean(*var_199);
    }
}

pub fn serialize_structure_crate_model_port_override(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PortOverride,
) {
    if let Some(var_200) = &input.listener_port {
        object.key("ListenerPort").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_200).into()),
        );
    }
    if let Some(var_201) = &input.endpoint_port {
        object.key("EndpointPort").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_201).into()),
        );
    }
}

pub fn serialize_structure_crate_model_cidr_authorization_context(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CidrAuthorizationContext,
) {
    if let Some(var_202) = &input.message {
        object.key("Message").string(var_202);
    }
    if let Some(var_203) = &input.signature {
        object.key("Signature").string(var_203);
    }
}
