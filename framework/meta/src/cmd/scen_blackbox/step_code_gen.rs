use multiversx_sc_scenario::scenario::model::Step;

use super::{scenario_loader::scenario_to_function_name, test_gen::TestGenerator};

impl<'a> TestGenerator<'a> {
    // -------------------------------------------------------------------------
    // Step dispatcher
    // -------------------------------------------------------------------------

    /// Generates code for a single step
    pub fn generate_step_code(&mut self, step: &Step) {
        match step {
            Step::ExternalSteps(step_data) => {
                self.generate_external_steps(&step_data.path, step_data.comment.as_deref());
            }
            Step::SetState(set_state) => {
                self.generate_set_state(set_state);
            }
            Step::ScDeploy(sc_deploy) => {
                self.generate_sc_deploy(sc_deploy);
            }
            Step::ScCall(sc_call) => {
                self.generate_sc_call(sc_call);
            }
            Step::ScQuery(sc_query) => {
                self.generate_sc_query(sc_query);
            }
            Step::CheckState(check_state) => {
                self.generate_check_state(check_state.comment.as_deref(), &check_state.accounts);
            }
            Step::Transfer(_transfer) => {
                self.step_writeln("    // TODO: Transfer step");
            }
            Step::ValidatorReward(_) => {
                self.step_writeln("    // TODO: ValidatorReward step");
            }
            Step::DumpState(_) => {
                self.step_writeln("    // TODO: DumpState step");
            }
        }
    }

    fn generate_external_steps(&mut self, path: &str, comment: Option<&str>) {
        if let Some(comment_text) = comment {
            self.step_writeln(format!("    // {}", comment_text));
        }

        let scenario_name = std::path::Path::new(path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(path);

        let steps_function_name = format!("{}_steps", scenario_to_function_name(scenario_name));

        self.step_writeln(format!("    {}(world);", steps_function_name));
        self.step_writeln("");
    }

    // -------------------------------------------------------------------------
    // ABI lookups (shared across all generators)
    // -------------------------------------------------------------------------

    /// Looks up the ABI inputs for an endpoint by its scenario name.
    pub(super) fn find_endpoint_inputs(
        &self,
        endpoint_name: &str,
    ) -> Option<Vec<multiversx_sc::abi::InputAbi>> {
        self.abi
            .endpoints
            .iter()
            .find(|e| e.name == endpoint_name)
            .map(|e| e.inputs.clone())
    }

    /// Looks up the ABI outputs for an endpoint by its scenario name.
    pub(super) fn find_endpoint_outputs(
        &self,
        endpoint_name: &str,
    ) -> Option<Vec<multiversx_sc::abi::OutputAbi>> {
        self.abi
            .endpoints
            .iter()
            .find(|e| e.name == endpoint_name)
            .map(|e| e.outputs.clone())
    }

    /// Looks up the ABI inputs for the constructor.
    pub(super) fn find_constructor_inputs(&self) -> Option<Vec<multiversx_sc::abi::InputAbi>> {
        self.abi.constructors.first().map(|e| e.inputs.clone())
    }

    /// Maps an endpoint name from the scenario (usually camelCase) to the Rust method name (snake_case)
    /// by looking it up in the contract ABI.
    pub(super) fn map_endpoint_name(&self, scenario_endpoint_name: &str) -> String {
        for endpoint in &self.abi.endpoints {
            if endpoint.name == scenario_endpoint_name {
                return endpoint.rust_method_name.clone();
            }
        }
        scenario_endpoint_name.to_string()
    }

    pub(super) fn generate_proxy_type(&self) -> String {
        // Convert crate name to CamelCase for the proxy struct name
        let struct_name = self
            .crate_name
            .split('_')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<String>();

        format!("{}_proxy::{}Proxy", self.crate_name, struct_name)
    }
}
