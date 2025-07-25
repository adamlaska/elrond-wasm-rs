use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/benchmarks/mappers/vec-repeat");

    blockchain.register_contract(
        "mxsc:output/vec-repeat.mxsc.json",
        vec_repeat::ContractBuilder,
    );
    blockchain
}

#[test]
fn vec_repeat_rs() {
    world().run("scenarios/vec_repeat.scen.json");
}

#[test]
fn vec_repeat_struct_rs() {
    world().run("scenarios/vec_repeat_struct.scen.json");
}
