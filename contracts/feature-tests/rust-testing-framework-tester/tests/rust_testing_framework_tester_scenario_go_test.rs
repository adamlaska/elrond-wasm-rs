use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
#[ignore]
fn test_go() {
    world().run("scenarios/test.scen.json");
}

#[test]
#[ignore]
fn test_esdt_generation_go() {
    world().run("scenarios/test_esdt_generation.scen.json");
}

#[test]
#[ignore]
fn test_multiple_sc_go() {
    world().run("scenarios/test_multiple_sc.scen.json");
}

#[test]
#[ignore = "not supported"]
fn trace_deploy_go() {
    world().run("scenarios/trace-deploy.scen.json");
}
