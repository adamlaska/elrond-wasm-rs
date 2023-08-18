use multiversx_sc_meta::template::{
    template_names_from_repo, ContractCreator, ContractCreatorTarget, RepoSource,
};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

const TEMPLATE_TEMP_DIR_NAME: &str = "template-test-current";
const BUILD_CONTRACTS: bool = false;

#[test]
fn test_template_list() {
    let workspace_path = find_workspace();
    let repo_source = RepoSource::from_local_path(workspace_path);
    let mut template_names = template_names_from_repo(&repo_source);
    template_names.sort();
    assert_eq!(
        template_names,
        [
            "adder".to_string(),
            "crypto-zombies".to_string(),
            "empty".to_string()
        ]
    );
}

#[test]
#[cfg_attr(not(feature = "template-test-current"), ignore)]
fn template_test_current_adder() {
    template_test_current("adder", "examples", "new-adder");
}

#[test]
#[cfg_attr(not(feature = "template-test-current"), ignore)]
fn template_test_current_crypto_zombies() {
    template_test_current("crypto-zombies", "examples", "new-crypto-zombies");
}

#[test]
#[cfg_attr(not(feature = "template-test-current"), ignore)]
fn template_test_current_empty() {
    template_test_current("empty", "examples", "new-empty");
}

/// Recreates the folder structure in `contracts`, on the same level.
/// This way, the relative paths are still valid in this case, and we can test the templates with the versions on the current branch.
fn template_test_current(template_name: &str, sub_path: &str, new_name: &str) {
    let workspace_path = find_workspace();
    let target = ContractCreatorTarget {
        target_path: workspace_path.join(TEMPLATE_TEMP_DIR_NAME).join(sub_path),
        new_name: new_name.to_string(),
    };

    let repo_source = RepoSource::from_local_path(workspace_path);

    prepare_target_dir(&target);

    let downloader = ContractCreator::new(
        &repo_source,
        template_name.to_string(),
        target.clone(),
        true,
    );

    downloader.create_contract();

    if BUILD_CONTRACTS {
        build_contract(&target);
    }
    cargo_test(&target);
}

fn prepare_target_dir(target: &ContractCreatorTarget) {
    fs::create_dir_all(&target.target_path).unwrap();

    let contract_dir = target.contract_dir();
    if contract_dir.exists() {
        fs::remove_dir_all(&contract_dir).unwrap();
    }
}

pub fn cargo_test(target: &ContractCreatorTarget) {
    let workspace_target_dir = find_workspace().join("target");

    let mut args = vec![
        "test",
        "--target-dir",
        workspace_target_dir.to_str().unwrap(),
    ];
    if BUILD_CONTRACTS {
        args.push("--features");
        args.push("multiversx-sc-scenario/run-go-tests");
    }

    let exit_status = Command::new("cargo")
        .args(args)
        .current_dir(target.contract_dir())
        .spawn()
        .expect("failed to spawn contract clean process")
        .wait()
        .expect("contract test process was not running");

    assert!(exit_status.success(), "contract test process failed");
}

pub fn build_contract(target: &ContractCreatorTarget) {
    let workspace_target_dir = find_workspace().join("target");

    let exit_status = Command::new("cargo")
        .args([
            "run",
            "build",
            "--target-dir",
            workspace_target_dir.to_str().unwrap(),
        ])
        .current_dir(target.contract_dir().join("meta"))
        .spawn()
        .expect("failed to spawn contract clean process")
        .wait()
        .expect("contract test process was not running");

    assert!(exit_status.success(), "contract build process failed");
}

/// Finds the workspace by taking the `current_exe` and working its way up.
/// Works in debug mode too.
///
/// TODO: duplicated code from scenario_world. De-duplicate after dependencies are reorganized.
pub fn find_workspace() -> PathBuf {
    let current_exe = std::env::current_exe().unwrap();
    let mut path = current_exe.as_path();
    while !is_target(path) {
        path = path.parent().unwrap();
    }

    path.parent().unwrap().into()
}

fn is_target(path_buf: &Path) -> bool {
    path_buf.file_name().unwrap() == "target"
}
