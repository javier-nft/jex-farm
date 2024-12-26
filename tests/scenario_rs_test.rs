use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    // blockchain.set_current_dir_from_workspace("relative path to your workspace, if applicable");

    blockchain.register_contract("file:output/jex-sc-farm.wasm", jex_sc_farm::ContractBuilder);
    blockchain
}

#[test]
fn claim_nominal_1_rs() {
    world().run("scenarios/claim_nominal_1.scen.json");
}

#[test]
fn claim_nominal_2_rs() {
    world().run("scenarios/claim_nominal_2.scen.json");
}

#[test]
fn e_2_e_rs() {
    world().run("scenarios/e2e.scen.json");
}

#[test]
fn exit_nominal_1_rs() {
    world().run("scenarios/exit_nominal_1.scen.json");
}

#[test]
fn extend_farm_rs() {
    world().run("scenarios/extend_farm.scen.json");
}

#[test]
fn fund_rs() {
    world().run("scenarios/fund.scen.json");
}

#[test]
fn fund_invalid_balance_rs() {
    world().run("scenarios/fund_invalid_balance.scen.json");
}

#[test]
fn fund_not_owner_rs() {
    world().run("scenarios/fund_not_owner.scen.json");
}

#[test]
fn fund_wrong_token_rs() {
    world().run("scenarios/fund_wrong_token.scen.json");
}

#[test]
fn get_pending_rewards_1_rs() {
    world().run("scenarios/get_pending_rewards_1.scen.json");
}

#[test]
fn get_pending_rewards_1_after_farm_extended_rs() {
    world().run("scenarios/get_pending_rewards_1_after_farm_extended.scen.json");
}

#[test]
fn get_pending_rewards_1_farm_ended_rs() {
    world().run("scenarios/get_pending_rewards_1_farm_ended.scen.json");
}

#[test]
fn get_pending_rewards_2_rs() {
    world().run("scenarios/get_pending_rewards_2.scen.json");
}

#[test]
fn init_rs() {
    world().run("scenarios/init.scen.json");
}

#[test]
fn set_rewards_duration_nominal_rs() {
    world().run("scenarios/set_rewards_duration_nominal.scen.json");
}

#[test]
fn set_rewards_duration_period_not_complete_rs() {
    world().run("scenarios/set_rewards_duration_period_not_complete.scen.json");
}

#[test]
fn set_rewards_duration_period_not_owner_rs() {
    world().run("scenarios/set_rewards_duration_period_not_owner.scen.json");
}

#[test]
fn stake_nominal_1_rs() {
    world().run("scenarios/stake_nominal_1.scen.json");
}

#[test]
fn stake_nominal_2_rs() {
    world().run("scenarios/stake_nominal_2.scen.json");
}

#[test]
fn stake_not_started_rs() {
    world().run("scenarios/stake_not_started.scen.json");
}

#[test]
fn stake_wrong_token_rs() {
    world().run("scenarios/stake_wrong_token.scen.json");
}

#[test]
fn terminate_nominal_rs() {
    world().run("scenarios/terminate_nominal.scen.json");
}

#[test]
fn terminate_not_owner_rs() {
    world().run("scenarios/terminate_not_owner.scen.json");
}

#[test]
fn withdraw_invalid_amount_rs() {
    world().run("scenarios/withdraw_invalid_amount.scen.json");
}

#[test]
fn withdraw_nominal_1_rs() {
    world().run("scenarios/withdraw_nominal_1.scen.json");
}

#[test]
fn withdraw_partial_1_rs() {
    world().run("scenarios/withdraw_partial_1.scen.json");
}
