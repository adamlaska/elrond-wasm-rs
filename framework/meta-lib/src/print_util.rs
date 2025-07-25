use std::{path::Path, process::Command};

use colored::Colorize;

/// Just for convenience, since we seem to be printing many things in green.
///
/// The argument is of type `String` because the argument is always a `format!` expression.
pub fn println_green(s: String) {
    println!("{}", s.green());
}

pub fn format_command(command: &Command) -> String {
    let mut result = String::new();
    for (key, opt_value) in command.get_envs() {
        if let Some(value) = opt_value {
            result +=
                format!("{}=\"{}\" ", key.to_string_lossy(), value.to_string_lossy()).as_str();
        }
    }
    result.push_str(command.get_program().to_string_lossy().as_ref());

    for arg in command.get_args() {
        result.push(' ');
        result.push_str(arg.to_string_lossy().as_ref());
    }

    result
}

pub fn print_build_command(contract_name: String, command: &Command) {
    let path = command
        .get_current_dir()
        .expect("missing command dir")
        .canonicalize()
        .expect("command dir canonicalization failed");
    println!(
        "{}\n{}",
        format!("Building {} in {} ...", contract_name, path.display()).green(),
        format_command(command).green(),
    );
}

pub fn print_copy_contract(source_wasm_path: &str, output_wasm_path: &str) {
    println!(
        "{}",
        format!("Copying {source_wasm_path} to {output_wasm_path} ...").green(),
    );
}

pub fn print_call_wasm_opt(wasm_path: &str) {
    println_green(format!("Calling wasm-opt on {wasm_path} ..."));
}

pub fn print_call_wasm2wat(wasm_path: &str, wat_path: &str) {
    println_green(format!("Extracting wat from {wasm_path} to {wat_path} ..."));
}

pub fn print_pack_mxsc_file(output_mxsc_path: &str) {
    println_green(format!("Packing {output_mxsc_path} ..."));
}

pub fn print_contract_size(size: usize) {
    println!("{}", format!("Contract size: {size} bytes.").blue(),);
}

pub fn print_extract_imports(imports_path: &str) {
    println_green(format!("Extracting imports to {imports_path} ..."));
}

pub fn print_check_ei(ei_version: &str) {
    print!(
        "{}",
        format!("Checking EI version: {ei_version} ...").green(),
    );
}

pub fn print_invalid_vm_hook(import_name: &str, ei_version: &str) {
    print!(
        "\n{}",
        format!("WARNING! Import '{import_name}' is not available on EI version {ei_version}!")
            .yellow(),
    );
}

pub fn print_check_ei_ok() {
    println!("{}", " OK".green(),);
}

pub fn print_ignore_ei_check() {
    println!("{}", "EI version check explicitly ignored".yellow(),);
}

pub fn print_workspace_target_dir(target_path_str: &str) {
    println_green(format!(
        "Using workspace target directory: {target_path_str} ..."
    ));
}

pub fn print_removing_wasm_crate(dir_name: &str) {
    println!("{}", format!("Removing wasm crate: {dir_name}").red(),);
}

pub fn print_sc_config_main_deprecated(path: &Path) {
    println!(
        "{}",
        format!(
            "In {}: `main` field under `[settings]` is now deprecated",
            path.display()
        )
        .yellow(),
    );
}

pub fn print_proxy_error(path: &Path, error: String) {
    println!(
        "{}",
        format!("Could not write proxy file {}: {error}", path.display()).red(),
    );
}
