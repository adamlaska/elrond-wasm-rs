use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn factorial_go() {
    world().run("scenarios/factorial.scen.json");
}
