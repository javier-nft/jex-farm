use jex_sc_farm::*;
use multiversx_sc::types::{Address, TokenIdentifier};
use multiversx_sc_scenario::{rust_biguint, testing_framework::*, DebugApi};

const WASM_PATH: &str = "output/jex-farm.wasm";

struct ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> jex_sc_farm::ContractObj<DebugApi>,
{
    pub blockchain_wrapper: BlockchainStateWrapper,
    pub owner_address: Address,
    pub contract_wrapper:
        ContractObjWrapper<jex_sc_farm::ContractObj<DebugApi>, ContractObjBuilder>,
}

fn setup_contract<ContractObjBuilder>(
    cf_builder: ContractObjBuilder,
) -> ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> jex_sc_farm::ContractObj<DebugApi>,
{
    let staking_token_id: &[u8] = b"STOK-000000";
    let rewards_token_id: &[u8] = b"RTOK-000000";
    let rust_zero = rust_biguint!(0u64);
    let mut blockchain_wrapper = BlockchainStateWrapper::new();
    let owner_address = blockchain_wrapper.create_user_account(&rust_zero);
    let cf_wrapper = blockchain_wrapper.create_sc_account(
        &rust_zero,
        Some(&owner_address),
        cf_builder,
        WASM_PATH,
    );

    blockchain_wrapper
        .execute_tx(&owner_address, &cf_wrapper, &rust_zero, |sc| {
            sc.init(
                TokenIdentifier::from_esdt_bytes(staking_token_id),
                TokenIdentifier::from_esdt_bytes(rewards_token_id),
            );
        })
        .assert_ok();

    blockchain_wrapper.add_mandos_set_account(cf_wrapper.address_ref());

    ContractSetup {
        blockchain_wrapper,
        owner_address,
        contract_wrapper: cf_wrapper,
    }
}

#[test]
fn deploy_test() {
    let staking_token_id: &[u8] = b"STOK-000000";
    let rewards_token_id: &[u8] = b"RTOK-000000";

    let mut setup = setup_contract(jex_sc_farm::contract_obj);

    // simulate deploy
    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.init(
                    TokenIdentifier::from_esdt_bytes(staking_token_id),
                    TokenIdentifier::from_esdt_bytes(rewards_token_id),
                );
            },
        )
        .assert_ok();
}
