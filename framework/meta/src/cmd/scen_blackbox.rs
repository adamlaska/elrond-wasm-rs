mod check_state_gen;
mod const_state;
mod format_args;
mod num_format;
mod scenario_loader;
mod set_state_gen;
mod step_code_gen;
mod test_gen;
mod tx_gen;

use crate::cli::ScenBlackboxArgs;
use crate::folder_structure::{RelevantDirectories, RelevantDirectory, dir_pretty_print};
use std::path::Path;

pub use test_gen::generate_scen_blackbox_tests;

pub fn scen_blackbox_tool(args: &ScenBlackboxArgs) {
    let path = if let Some(some_path) = &args.path {
        Path::new(some_path.as_str())
    } else {
        Path::new("./")
    };

    let dirs = RelevantDirectories::find_all(path, &args.ignore);
    let num_dirs = dirs.len();

    if num_dirs == 0 {
        println!("No contracts found");
        return;
    }

    println!("Generating blackbox tests for {num_dirs} contract(s) ...\n");

    for dir in dirs.iter() {
        generate_for_contract(dir, args.overwrite);
    }
}

fn generate_for_contract(dir: &RelevantDirectory, overwrite: bool) {
    dir_pretty_print(std::iter::once(dir), "", &|_| {});

    // Read the contract ABI
    let dir_name = dir.dir_name();
    let output_abi_path = dir
        .path
        .join("output")
        .join(format!("{}.abi.json", dir_name));

    if !output_abi_path.exists() {
        println!(
            "  ⚠️  No ABI found at {}, skipping",
            output_abi_path.display()
        );
        return;
    }

    let abi_json = std::fs::read_to_string(&output_abi_path)
        .unwrap_or_else(|_| panic!("Failed to read ABI file: {}", output_abi_path.display()));

    let abi_json: multiversx_sc_meta_lib::abi_json::ContractAbiJson =
        serde_json::from_str(&abi_json)
            .unwrap_or_else(|_| panic!("Failed to parse ABI file: {}", output_abi_path.display()));

    let abi: multiversx_sc::abi::ContractAbi = abi_json.into();

    // Change to contract directory and generate
    let original_dir = std::env::current_dir().expect("Failed to get current directory");
    std::env::set_current_dir(&dir.path).expect("Failed to change directory");

    generate_scen_blackbox_tests(overwrite, &abi);

    // Return to original directory
    std::env::set_current_dir(original_dir).expect("Failed to restore directory");
}
