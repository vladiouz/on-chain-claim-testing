use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn sc1() {
    world().run("scenarios/trace1.scen.json");
}

#[test]
fn sc2() {
    world().run("scenarios/trace2.scen.json");
}

#[test]
fn sc3() {
    world().run("scenarios/trace3.scen.json");
}

#[test]
fn sc5() {
    world().run("scenarios/trace5.scen.json");
}

#[test]
fn sc6() {
    world().run("scenarios/trace6.scen.json");
}

#[test]
fn sc7() {
    world().run("scenarios/trace7.scen.json");
}

#[test]
fn sc8() {
    world().run("scenarios/trace8.scen.json");
}
