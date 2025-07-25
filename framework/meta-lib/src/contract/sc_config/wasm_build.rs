use crate::contract::sc_config::execute_command::execute_command;
use crate::tools::build_target;
use colored::Colorize;
use std::process::{exit, ExitStatus};
use std::{
    collections::HashMap,
    env,
    ffi::{OsStr, OsString},
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use super::execute_command::{execute_spawn_command, ExecuteCommandError};
use super::ContractVariant;
use crate::{
    abi_json::ContractAbiJson,
    cli::BuildArgs,
    ei::EIVersion,
    ei_check_json::EiCheckJson,
    mxsc_file_json::{save_mxsc_file_json, MxscFileJson},
    print_util::*,
    report_info_json::ReportInfoJson,
    tools::{self, WasmInfo, WasmReport},
};

impl ContractVariant {
    pub fn build_contract(
        &self,
        build_args: &BuildArgs,
        output_path: &Path,
    ) -> Result<(), ExecuteCommandError> {
        let mut build_command = self.compose_build_command(build_args);

        print_build_command(self.wasm_output_name(build_args), &build_command);

        let output_build_command = execute_spawn_command(&mut build_command, "cargo");

        if let Err(ExecuteCommandError::JobFailed(_)) = output_build_command {
            let mut rustup = self.rustup_target_command();
            let target_list = rustup.arg("list").arg("--installed");

            let output_rustup_command = execute_command(target_list, "rustup");

            let str_output_rustup = match output_rustup_command {
                Ok(output) => output,
                Err(err) => {
                    println!("\n{}", err.to_string().red().bold());
                    exit(1);
                }
            };

            let rustc_target_str = self.settings.rustc_target.as_str();

            if !str_output_rustup.contains(rustc_target_str) {
                self.install_wasm_target(rustc_target_str, build_command)?;
            }
        }

        self.finalize_build(build_args, output_path);

        Ok(())
    }

    fn compose_build_command(&self, build_args: &BuildArgs) -> Command {
        let mut command = Command::new("cargo");
        command
            .arg("build")
            .arg(format!("--target={}", &self.settings.rustc_target))
            .arg("--release")
            .current_dir(self.wasm_crate_path());
        if build_args.locked {
            command.arg("--locked");
        }
        if let Some(target_dir_wasm) = &build_args.target_dir_wasm {
            command.args(["--target-dir", target_dir_wasm]);
        }
        let rustflags = self.compose_rustflags(build_args);
        if !rustflags.is_empty() {
            command.env("RUSTFLAGS", rustflags);
        }
        command
    }

    fn rustup_target_command(&self) -> Command {
        let rustup = env::var_os("RUSTUP").unwrap_or_else(|| OsString::from("rustup"));

        let mut command = Command::new(rustup);
        command.arg("target");

        command
    }

    fn compose_rustflags(&self, build_args: &BuildArgs) -> Rustflags {
        let mut rustflags = Rustflags::default();

        if !build_args.wasm_symbols {
            rustflags.push_flag("-C link-arg=-s");
        }

        rustflags.push_flag(&format!(
            "-C link-arg=-zstack-size={}",
            self.settings.stack_size
        ));

        if build_args.emit_mir {
            rustflags.push_flag("--emit=mir");
        }

        if build_args.emit_llvm_ir {
            rustflags.push_flag("--emit=llvm-ir");
        }
        rustflags
    }

    fn install_wasm_target(
        &self,
        target: &str,
        mut build_command: Command,
    ) -> Result<ExitStatus, ExecuteCommandError> {
        println!(
            "\n{}{}{}",
            "Installing target \"".yellow(),
            target.yellow(),
            "\"...".yellow()
        );

        if target == build_target::WASM32V1_TARGET {
            build_target::install_target(tools::build_target::WASM32V1_TARGET);
        } else {
            build_target::install_target(tools::build_target::WASM32_TARGET);
        }

        execute_spawn_command(&mut build_command, "cargo")
    }

    fn finalize_build(&self, build_args: &BuildArgs, output_path: &Path) {
        self.copy_contracts_to_output(build_args, output_path);
        self.run_wasm_opt(build_args, output_path);
        self.run_wasm2wat(build_args, output_path);
        let report = self.extract_wasm_info(build_args, output_path);
        self.run_twiggy(build_args, output_path);
        self.pack_mxsc_file(build_args, output_path, &report);
    }

    fn copy_contracts_to_output(&self, build_args: &BuildArgs, output_path: &Path) {
        let source_wasm_path = self.wasm_compilation_output_path(&build_args.target_dir_wasm);
        let output_wasm_path = output_path.join(self.wasm_output_name(build_args));
        print_copy_contract(
            &source_wasm_path.to_string_lossy(),
            &output_wasm_path.to_string_lossy(),
        );
        fs::copy(source_wasm_path, output_wasm_path)
            .expect("failed to copy compiled contract to output directory");
    }

    fn pack_mxsc_file(&self, build_args: &BuildArgs, output_path: &Path, wasm_report: &WasmReport) {
        let output_wasm_path = output_path.join(self.wasm_output_name(build_args));
        let compiled_bytes = fs::read(output_wasm_path).expect("failed to open compiled contract");
        let output_mxsc_path = output_path.join(self.mxsc_file_output_name(build_args));
        print_pack_mxsc_file(&output_mxsc_path.to_string_lossy());
        print_contract_size(compiled_bytes.len());
        let mut abi = ContractAbiJson::from(&self.abi);
        let build_info = core::mem::take(&mut abi.build_info).unwrap();
        let ei_check_json = EiCheckJson::new(&self.settings.check_ei, wasm_report.ei_check);
        let report = ReportInfoJson::new(wasm_report, ei_check_json, compiled_bytes.len());
        let mxsc_file_json = MxscFileJson {
            build_info,
            abi,
            code: hex::encode(compiled_bytes),
            report,
        };

        save_mxsc_file_json(&mxsc_file_json, output_mxsc_path);
    }

    fn run_wasm_opt(&self, build_args: &BuildArgs, output_path: &Path) {
        if !build_args.wasm_opt {
            return;
        }

        let output_wasm_path = output_path.join(self.wasm_output_name(build_args));

        print_call_wasm_opt(&output_wasm_path.to_string_lossy());
        tools::run_wasm_opt(&output_wasm_path.to_string_lossy());
    }

    fn run_wasm2wat(&self, build_args: &BuildArgs, output_path: &Path) {
        if !build_args.wat {
            return;
        }

        let output_wasm_path = output_path.join(self.wasm_output_name(build_args));
        let output_wat_path = output_path.join(self.wat_output_name(build_args));

        print_call_wasm2wat(
            &output_wasm_path.to_string_lossy(),
            &output_wat_path.to_string_lossy(),
        );
        tools::wasm_to_wat(
            &output_wasm_path.to_string_lossy(),
            &output_wat_path.to_string_lossy(),
        );
    }

    fn extract_wasm_info(&self, build_args: &BuildArgs, output_path: &Path) -> WasmReport {
        let output_wasm_path = output_path.join(self.wasm_output_name(build_args));

        let abi = ContractAbiJson::from(&self.abi);
        let mut endpoints: HashMap<&str, bool> = HashMap::new();

        if abi.constructor.is_some() {
            endpoints.insert("init", false);
        }

        if abi.upgrade_constructor.is_some() {
            endpoints.insert("upgrade", false);
        }

        for endpoint in &abi.endpoints {
            if let crate::abi_json::EndpointMutabilityAbiJson::Readonly = endpoint.mutability {
                endpoints.insert(&endpoint.name, true);
            } else {
                endpoints.insert(&endpoint.name, false);
            }
        }

        if !build_args.extract_imports {
            return WasmInfo::extract_wasm_report(
                &output_wasm_path,
                build_args.extract_imports,
                self.settings.check_ei.as_ref(),
                &endpoints,
            );
        }

        let output_imports_json_path = output_path.join(self.imports_json_output_name(build_args));

        print_extract_imports(&output_imports_json_path.to_string_lossy());

        let wasm_report = WasmInfo::extract_wasm_report(
            &output_wasm_path,
            true,
            self.settings.check_ei.as_ref(),
            &endpoints,
        );

        write_imports_output(&output_imports_json_path, wasm_report.imports.as_slice());
        print_ei_check(&wasm_report, &self.settings.check_ei);

        wasm_report
    }
}

fn write_imports_output(dest_path: &PathBuf, import_names: &[String]) {
    let json = serde_json::to_string_pretty(import_names).unwrap();
    fs::write(dest_path, json).expect("failed to write imports json file");
}

fn print_ei_check(wasm_report: &WasmReport, check_ei: &Option<EIVersion>) {
    if let Some(ei) = check_ei {
        print_check_ei(ei.name());

        if wasm_report.ei_check {
            print_check_ei_ok();
        } else {
            for import_name in &wasm_report.imports {
                if !ei.contains_vm_hook(import_name.as_str()) {
                    print_invalid_vm_hook(import_name.as_str(), ei.name());
                }
            }

            println!();
        }
    } else {
        print_ignore_ei_check();
    }
}

impl ContractVariant {
    fn run_twiggy(&self, build_args: &BuildArgs, output_path: &Path) {
        if build_args.has_twiggy_call() {
            let output_wasm_path = output_path.join(self.wasm_output_name(build_args));

            if build_args.twiggy_top {
                let output_twiggy_top_path = output_path.join(self.twiggy_top_name(build_args));

                tools::twiggy::run_twiggy_top(&output_wasm_path, &output_twiggy_top_path);
            }
            if build_args.twiggy_paths {
                let output_twiggy_paths_path = output_path.join(self.twiggy_paths_name(build_args));

                tools::twiggy::run_twiggy_paths(&output_wasm_path, &output_twiggy_paths_path);
            }
            if build_args.twiggy_monos {
                let output_twiggy_monos_path = output_path.join(self.twiggy_monos_name(build_args));

                tools::twiggy::run_twiggy_monos(&output_wasm_path, &output_twiggy_monos_path);
            }
            if build_args.twiggy_dominators {
                let output_twiggy_dominators_path =
                    output_path.join(self.twiggy_dominators_name(build_args));

                tools::twiggy::run_twiggy_dominators(
                    &output_wasm_path,
                    &output_twiggy_dominators_path,
                );
            }
        }
    }
}

/// For convenience, for building rustflags.
#[derive(Default)]
struct Rustflags(String);

impl Rustflags {
    fn push_flag(&mut self, s: &str) {
        if !self.0.is_empty() {
            self.0.push(' ');
        }
        self.0.push_str(s);
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl AsRef<OsStr> for Rustflags {
    fn as_ref(&self) -> &OsStr {
        self.0.as_ref()
    }
}
