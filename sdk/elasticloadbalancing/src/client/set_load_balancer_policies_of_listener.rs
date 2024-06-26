// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`SetLoadBalancerPoliciesOfListener`](crate::operation::set_load_balancer_policies_of_listener::builders::SetLoadBalancerPoliciesOfListenerFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`load_balancer_name(impl Into<String>)`](crate::operation::set_load_balancer_policies_of_listener::builders::SetLoadBalancerPoliciesOfListenerFluentBuilder::load_balancer_name) / [`set_load_balancer_name(Option<String>)`](crate::operation::set_load_balancer_policies_of_listener::builders::SetLoadBalancerPoliciesOfListenerFluentBuilder::set_load_balancer_name):<br>required: **true**<br><p>The name of the load balancer.</p><br>
    ///   - [`load_balancer_port(i32)`](crate::operation::set_load_balancer_policies_of_listener::builders::SetLoadBalancerPoliciesOfListenerFluentBuilder::load_balancer_port) / [`set_load_balancer_port(Option<i32>)`](crate::operation::set_load_balancer_policies_of_listener::builders::SetLoadBalancerPoliciesOfListenerFluentBuilder::set_load_balancer_port):<br>required: **true**<br><p>The external port of the load balancer.</p><br>
    ///   - [`policy_names(impl Into<String>)`](crate::operation::set_load_balancer_policies_of_listener::builders::SetLoadBalancerPoliciesOfListenerFluentBuilder::policy_names) / [`set_policy_names(Option<Vec::<String>>)`](crate::operation::set_load_balancer_policies_of_listener::builders::SetLoadBalancerPoliciesOfListenerFluentBuilder::set_policy_names):<br>required: **true**<br><p>The names of the policies. This list must include all policies to be enabled. If you omit a policy that is currently enabled, it is disabled. If the list is empty, all current policies are disabled.</p><br>
    /// - On success, responds with [`SetLoadBalancerPoliciesOfListenerOutput`](crate::operation::set_load_balancer_policies_of_listener::SetLoadBalancerPoliciesOfListenerOutput)
    /// - On failure, responds with [`SdkError<SetLoadBalancerPoliciesOfListenerError>`](crate::operation::set_load_balancer_policies_of_listener::SetLoadBalancerPoliciesOfListenerError)
    pub fn set_load_balancer_policies_of_listener(
        &self,
    ) -> crate::operation::set_load_balancer_policies_of_listener::builders::SetLoadBalancerPoliciesOfListenerFluentBuilder {
        crate::operation::set_load_balancer_policies_of_listener::builders::SetLoadBalancerPoliciesOfListenerFluentBuilder::new(self.handle.clone())
    }
}
