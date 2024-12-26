use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn claim_nominal_1_go() {
    world().run("scenarios/claim_nominal_1.scen.json");
}

#[test]
fn claim_nominal_2_go() {
    world().run("scenarios/claim_nominal_2.scen.json");
}

#[test]
fn e_2_e_go() {
    world().run("scenarios/e2e.scen.json");
}

#[test]
fn exit_nominal_1_go() {
    world().run("scenarios/exit_nominal_1.scen.json");
}

#[test]
fn extend_farm_go() {
    world().run("scenarios/extend_farm.scen.json");
}

#[test]
fn fund_go() {
    world().run("scenarios/fund.scen.json");
}

#[test]
fn fund_invalid_balance_go() {
    world().run("scenarios/fund_invalid_balance.scen.json");
}

#[test]
fn fund_not_owner_go() {
    world().run("scenarios/fund_not_owner.scen.json");
}

#[test]
fn fund_wrong_token_go() {
    world().run("scenarios/fund_wrong_token.scen.json");
}

#[test]
fn get_pending_rewards_1_go() {
    world().run("scenarios/get_pending_rewards_1.scen.json");
}

#[test]
fn get_pending_rewards_1_after_farm_extended_go() {
    world().run("scenarios/get_pending_rewards_1_after_farm_extended.scen.json");
}

#[test]
fn get_pending_rewards_1_farm_ended_go() {
    world().run("scenarios/get_pending_rewards_1_farm_ended.scen.json");
}

#[test]
fn get_pending_rewards_2_go() {
    world().run("scenarios/get_pending_rewards_2.scen.json");
}

#[test]
fn init_go() {
    world().run("scenarios/init.scen.json");
}

#[test]
fn set_rewards_duration_nominal_go() {
    world().run("scenarios/set_rewards_duration_nominal.scen.json");
}

#[test]
fn set_rewards_duration_period_not_complete_go() {
    world().run("scenarios/set_rewards_duration_period_not_complete.scen.json");
}

#[test]
fn set_rewards_duration_period_not_owner_go() {
    world().run("scenarios/set_rewards_duration_period_not_owner.scen.json");
}

#[test]
fn stake_nominal_1_go() {
    world().run("scenarios/stake_nominal_1.scen.json");
}

#[test]
fn stake_nominal_2_go() {
    world().run("scenarios/stake_nominal_2.scen.json");
}

#[test]
fn stake_not_started_go() {
    world().run("scenarios/stake_not_started.scen.json");
}

#[test]
fn stake_wrong_token_go() {
    world().run("scenarios/stake_wrong_token.scen.json");
}

#[test]
fn terminate_nominal_go() {
    world().run("scenarios/terminate_nominal.scen.json");
}

#[test]
fn terminate_not_owner_go() {
    world().run("scenarios/terminate_not_owner.scen.json");
}

#[test]
fn withdraw_invalid_amount_go() {
    world().run("scenarios/withdraw_invalid_amount.scen.json");
}

#[test]
fn withdraw_nominal_1_go() {
    world().run("scenarios/withdraw_nominal_1.scen.json");
}

#[test]
fn withdraw_partial_1_go() {
    world().run("scenarios/withdraw_partial_1.scen.json");
}
