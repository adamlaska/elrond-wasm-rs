#![allow(clippy::type_complexity)]

pub mod api;
pub mod display_util;
pub mod executor;
mod facade;
pub mod managed_test_util;
pub mod scenario;
pub mod scenario_macros;
mod vm_go_tool;

pub mod whitebox_legacy;

/// Keeping this for backwards compatibility.
/// Unfortunately, the `deprecated` annotation doesn't function for reexports.
pub use whitebox_legacy as testing_framework;

pub use api::DebugApi;
pub use multiversx_chain_vm;

/// Re-exporting for convenience.
pub use num_bigint;

pub use multiversx_sc;

pub use multiversx_sc_meta_lib as meta;

/// Exposing the scenario model. Might be moved in the future,
/// but the export will hopefully remain the same.
pub use crate::scenario::model as scenario_model;

/// For backwards compatibility, will be removed.
pub use crate::scenario as mandos_system;

// Re-exporting the whole mandos crate for easier use in tests.
pub use multiversx_chain_scenario_format as scenario_format;

pub use facade::{result_handlers::*, world_tx::*, ContractInfo, ScenarioWorld, WhiteboxContract};

use std::path::Path;

/// Imports normally needed in integration tests, grouped together.
pub mod imports;

/// Legacy function for running a scenario test using the Go VM tool.
///
/// Use `sc-meta test-gen` to replace all calls to it automatically.
#[deprecated(
    since = "0.42.0",
    note = "Call `sc-meta test-gen` in the project folder to automatically upgrade all scenario tests."
)]
pub fn run_go<P: AsRef<Path>>(relative_path: P) {
    ScenarioWorld::vm_go().run(relative_path);
}

#[deprecated(
    since = "0.39.0",
    note = "Call `sc-meta test-gen` in the project folder to automatically upgrade all scenario tests."
)]
pub fn mandos_go<P: AsRef<Path>>(relative_path: P) {
    ScenarioWorld::vm_go().run(relative_path);
}

/// Legacy function for running a scenario test using the Go VM tool.
///
/// Use `sc-meta test-gen` to replace all calls to it automatically.
#[deprecated(
    since = "0.42.0",
    note = "Call `sc-meta test-gen` in the project folder to automatically upgrade all scenario tests."
)]
pub fn run_rs<P: AsRef<Path>>(relative_path: P, world: ScenarioWorld) {
    world.run(relative_path);
}

#[deprecated(
    since = "0.39.0",
    note = "Call `sc-meta test-gen` in the project folder to automatically upgrade all scenario tests."
)]
pub fn mandos_rs<P: AsRef<Path>>(relative_path: P, world: ScenarioWorld) {
    world.run(relative_path);
}

#[deprecated(
    since = "0.39.0",
    note = "Alias provided for backwards compatibility. Do replace `BlockchainMock` with `ScenarioWorld` after upgrading, though."
)]
pub type BlockchainMock = ScenarioWorld;
