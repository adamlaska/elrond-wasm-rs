use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn esdt_system_sc_go() {
    world().run("scenarios/esdt_system_sc.scen.json");
}
