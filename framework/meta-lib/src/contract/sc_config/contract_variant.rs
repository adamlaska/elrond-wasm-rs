use std::path::{Path, PathBuf};

use super::{contract_variant_builder::default_wasm_crate_name, ContractVariantSettings};
use crate::cli::BuildArgs;
use multiversx_sc::abi::ContractAbi;

/// Represents a contract created by the framework when building.
///
/// It might have only some of the endpoints written by the developer and maybe some other function.
pub struct ContractVariant {
    /// Initially, there was only 1 wasm crate, called `wasm`.
    ///
    /// Then, we got multi-contracts, but the wasm crate became the "main" one.
    ///
    /// Then we removed that mechanism completely, and it remained purely cosmetic,
    /// being named just `wasm`, instead of `wasm-{contract-name}`.
    ///
    /// We are still keeping this around for contracts with only one output contract, for the cosmetic bit.
    pub wasm_dir_short_name: bool,

    /// The contract id is defined in `multicontract.toml`. It has no effect on the produced assets.
    ///
    /// It can be the same as the contract name, but it is not necessary.
    pub contract_id: String,

    /// The name, as seen in the generated contract names.
    ///
    /// It is either defined in the multicontract.toml, or is inferred from the main crate name.
    pub contract_name: String,

    /// The name of the wasm crate, as it appear in Cargo.toml. It is normally the `contract_name` field, followed by the `-wasm` suffix.
    ///
    /// However, the main contract Cargo.toml is given explicitly, so this name might differ.
    pub wasm_crate_name: String,

    /// Collection of flags, specified in the multicontract config.
    pub settings: ContractVariantSettings,

    /// Filtered and processed ABI of the output contract.
    pub abi: ContractAbi,
}

impl ContractVariant {
    pub fn default_from_abi(abi: &ContractAbi) -> Self {
        let default_contract_config_name = abi.build_info.contract_crate.name.to_string();
        let wasm_crate_name = default_wasm_crate_name(&default_contract_config_name);
        ContractVariant {
            wasm_dir_short_name: true,
            settings: ContractVariantSettings::default(),
            contract_id: default_contract_config_name.clone(),
            contract_name: default_contract_config_name,
            wasm_crate_name,
            abi: abi.clone(),
        }
    }

    pub fn public_name_snake_case(&self) -> String {
        self.contract_name.replace('-', "_")
    }

    /// The name of the directory of the wasm crate.
    ///
    /// Note this does not necessarily have to match the wasm crate name defined in Cargo.toml.
    pub fn wasm_crate_dir_name(&self) -> String {
        if self.wasm_dir_short_name {
            "wasm".to_string()
        } else {
            format!("wasm-{}", &self.contract_name)
        }
    }

    pub fn wasm_crate_path(&self) -> PathBuf {
        Path::new("..").join(self.wasm_crate_dir_name())
    }

    pub fn cargo_toml_path(&self) -> PathBuf {
        Path::new(&self.wasm_crate_path()).join("Cargo.toml")
    }

    pub fn some_other_test_path(&self) -> PathBuf {
        Path::new(&self.wasm_crate_path()).join("test-Cargo.toml")
    }

    pub fn wasm_crate_name_snake_case(&self) -> String {
        self.wasm_crate_name.replace('-', "_")
    }

    pub fn resolve_wasm_target_dir(&self, explicit_target_dir: &Option<String>) -> PathBuf {
        let wasm_crate_path = self.wasm_crate_path();
        if let Some(explicit_target_dir) = explicit_target_dir {
            // usually the explicit_target_dir is absolute,
            // but if it isn't, we need to take the path of the wasm crate into account
            Path::new(&wasm_crate_path).join(explicit_target_dir)
        } else {
            Path::new(&wasm_crate_path).join("target")
        }
    }

    /// This is where Rust will initially compile the WASM binary.
    pub fn wasm_compilation_output_path(&self, explicit_target_dir: &Option<String>) -> PathBuf {
        let target_dir = self.resolve_wasm_target_dir(explicit_target_dir);
        let wasm_file_name = format!("{}.wasm", &self.wasm_crate_name_snake_case());

        Path::new(&target_dir)
            .join(&self.settings.rustc_target)
            .join("release")
            .join(wasm_file_name)
    }

    pub fn abi_output_name(&self) -> String {
        format!("{}.abi.json", &self.contract_name)
    }

    fn output_name_base(&self, build_args: &BuildArgs) -> String {
        if let Some(wasm_name_override) = &build_args.wasm_name_override {
            wasm_name_override.clone()
        } else if let Some(suffix) = &build_args.wasm_name_suffix {
            format!("{}-{suffix}", &self.contract_name)
        } else {
            self.contract_name.clone()
        }
    }

    pub fn wasm_output_name(&self, build_args: &BuildArgs) -> String {
        format!("{}.wasm", self.output_name_base(build_args))
    }

    pub fn wat_output_name(&self, build_args: &BuildArgs) -> String {
        format!("{}.wat", self.output_name_base(build_args))
    }

    pub fn mxsc_file_output_name(&self, build_args: &BuildArgs) -> String {
        format!("{}.mxsc.json", self.output_name_base(build_args))
    }

    pub fn imports_json_output_name(&self, build_args: &BuildArgs) -> String {
        format!("{}.imports.json", self.output_name_base(build_args))
    }

    pub fn twiggy_top_name(&self, build_args: &BuildArgs) -> String {
        format!("twiggy-top-{}.txt", self.output_name_base(build_args))
    }

    pub fn twiggy_paths_name(&self, build_args: &BuildArgs) -> String {
        format!("twiggy-paths-{}.txt", self.output_name_base(build_args))
    }

    pub fn twiggy_monos_name(&self, build_args: &BuildArgs) -> String {
        format!("twiggy-monos-{}.txt", self.output_name_base(build_args))
    }

    pub fn twiggy_dominators_name(&self, build_args: &BuildArgs) -> String {
        format!(
            "twiggy-dominators-{}.txt",
            self.output_name_base(build_args)
        )
    }

    pub fn endpoint_names(&self) -> Vec<String> {
        self.abi
            .endpoints
            .iter()
            .map(|endpoint| endpoint.name.to_string())
            .collect()
    }

    /// Yields "init" + all endpoint names + "callBack" (if it exists).
    ///
    /// Should correspond to all wasm exported functions.
    pub fn all_exported_function_names(&self) -> Vec<String> {
        let mut result = vec!["init".to_string()];
        if !self.abi.upgrade_constructors.is_empty() {
            result.push("upgrade".to_string())
        }
        result.append(&mut self.endpoint_names());
        if self.abi.has_callback {
            result.push("callBack".to_string());
        }
        result
    }
}

impl std::fmt::Debug for ContractVariant {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ContractVariant")
            .field("wasm_dir_short_name", &self.wasm_dir_short_name)
            .field("config_name", &self.contract_id)
            .field("public_name", &self.contract_name)
            .field("num-constructors", &self.abi.constructors.len())
            .field(
                "num-upgrade-constructors",
                &self.abi.upgrade_constructors.len(),
            )
            .field("num-endpoints", &self.abi.endpoints.len())
            .field("settings", &self.settings)
            .finish()
    }
}
