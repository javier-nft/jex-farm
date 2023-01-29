use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract("file:output/jex-sc-farm.wasm", jex_sc_farm::ContractBuilder);
    blockchain
}

#[test]
fn test_rs() {
    multiversx_sc_scenario::run_rs("scenarios/get_pending_rewards_2.scen.json", world());
}
