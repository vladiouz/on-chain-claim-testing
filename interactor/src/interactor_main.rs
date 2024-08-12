#![allow(non_snake_case)]

mod proxy;

use multiversx_sc_snippets::imports::*;
use multiversx_sc_snippets::sdk;
use multiversx_sc_snippets::sdk::wallet::Wallet;
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    path::Path,
};

const GATEWAY: &str = sdk::gateway::DEVNET_GATEWAY;
const STATE_FILE: &str = "state.toml";

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut args = std::env::args();
    let _ = args.next();
    let cmd = args.next().expect("at least one argument required");
    let mut interact = ContractInteract::new().await;
    match cmd.as_str() {
        "deploy" => interact.deploy().await,
        // "upgrade" => interact.upgrade().await,
        "claim" => interact.claim().await,
        "claimAndRepair" => interact.claim_and_repair().await,
        "updateState" => interact.update_state().await,
        "setRepairStreakPayment" => interact.set_repair_streak_payment().await,
        "getAddressInfo" => interact.get_address_info().await,
        "canBeRepaired" => interact.can_be_repaired().await,
        "getRepairStreakPayment" => interact.repair_streak_payment().await,
        "isAdmin" => interact.is_admin().await,
        "addAdmin" => interact.add_admin().await,
        "removeAdmin" => interact.remove_admin().await,
        "getAdmins" => interact.admins().await,
        _ => panic!("unknown command: {}", &cmd),
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct State {
    contract_address: Option<Bech32Address>,
}

impl State {
    // Deserializes state from file
    pub fn load_state() -> Self {
        if Path::new(STATE_FILE).exists() {
            let mut file = std::fs::File::open(STATE_FILE).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            toml::from_str(&content).unwrap()
        } else {
            Self::default()
        }
    }

    /// Sets the contract address
    pub fn set_address(&mut self, address: Bech32Address) {
        self.contract_address = Some(address);
    }

    /// Returns the contract address
    pub fn current_address(&self) -> &Bech32Address {
        self.contract_address
            .as_ref()
            .expect("no known contract, deploy first")
    }
}

impl Drop for State {
    // Serializes state to file
    fn drop(&mut self) {
        let mut file = std::fs::File::create(STATE_FILE).unwrap();
        file.write_all(toml::to_string(self).unwrap().as_bytes())
            .unwrap();
    }
}

struct ContractInteract {
    interactor: Interactor,
    wallet_address: Address,
    contract_code: BytesValue,
    state: State,
}

impl ContractInteract {
    async fn new() -> Self {
        let mut interactor = Interactor::new(GATEWAY).await;
        let wallet_address =
            interactor.register_wallet(Wallet::from_pem_file("wallet.pem").expect("msg"));

        let contract_code = BytesValue::interpret_from(
            "mxsc:../output/on-chain-claim.mxsc.json",
            &InterpreterContext::default(),
        );

        ContractInteract {
            interactor,
            wallet_address,
            contract_code,
            state: State::load_state(),
        }
    }

    async fn deploy(&mut self) {
        let token_identifier_str = "TUD-fff707";
        let repair_streak_token_id =
            TokenIdentifier::from_esdt_bytes(token_identifier_str.as_bytes());
        let repair_streak_token_nonce = 0u64;

        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(NumExpr("30,000,000"))
            .typed(proxy::OnChainClaimContractProxy)
            .init(repair_streak_token_id, repair_streak_token_nonce)
            .code(&self.contract_code)
            .returns(ReturnsNewAddress)
            .prepare_async()
            .run()
            .await;
        let new_address_bech32 = bech32::encode(&new_address);
        self.state.set_address(Bech32Address::from_bech32_string(
            new_address_bech32.clone(),
        ));

        println!("new address: {new_address_bech32}");
    }

    // async fn upgrade(&mut self) {
    //     let response = self
    //         .interactor
    //         .tx()
    //         .from(&self.wallet_address)
    //         .to(self.state.current_address())
    //         .gas(NumExpr("30,000,000"))
    //         .typed(proxy::OnChainClaimContractProxy)
    //         .upgrade()
    //         .returns(ReturnsResultUnmanaged)
    //         .prepare_async()
    //         .run()
    //         .await;

    //     println!("Result: {response:?}");
    // }

    async fn claim(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(NumExpr("30,000,000"))
            .typed(proxy::OnChainClaimContractProxy)
            .claim()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn claim_and_repair(&mut self) {
        let token_id = String::new();
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(NumExpr("30,000,000"))
            .typed(proxy::OnChainClaimContractProxy)
            .claim_and_repair()
            .payment((
                TokenIdentifier::from(token_id.as_str()),
                token_nonce,
                token_amount,
            ))
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn update_state(&mut self) {
        let address = bech32::decode("");
        let current_streak = 0u64;
        let last_epoch_claimed = 0u64;
        let total_epochs_claimed = 0u64;
        let best_streak = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(NumExpr("30,000,000"))
            .typed(proxy::OnChainClaimContractProxy)
            .update_state(
                address,
                current_streak,
                last_epoch_claimed,
                total_epochs_claimed,
                best_streak,
            )
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn set_repair_streak_payment(&mut self) {
        let repair_streak_token_identifier = TokenIdentifier::from_esdt_bytes(&b""[..]);
        let repair_streak_token_nonce = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(NumExpr("30,000,000"))
            .typed(proxy::OnChainClaimContractProxy)
            .set_repair_streak_payment(repair_streak_token_identifier, repair_streak_token_nonce)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn get_address_info(&mut self) {
        let address = bech32::decode("");

        let _result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::OnChainClaimContractProxy)
            .get_address_info(address)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        // println!("Result: {result_value:?}");
    }

    async fn can_be_repaired(&mut self) {
        let address = bech32::decode("");

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::OnChainClaimContractProxy)
            .can_be_repaired(address)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn repair_streak_payment(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::OnChainClaimContractProxy)
            .repair_streak_payment()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn is_admin(&mut self) {
        let address = bech32::decode("");

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::OnChainClaimContractProxy)
            .is_admin(address)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn add_admin(&mut self) {
        let address = bech32::decode("");

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(NumExpr("30,000,000"))
            .typed(proxy::OnChainClaimContractProxy)
            .add_admin(address)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn remove_admin(&mut self) {
        let address = bech32::decode("");

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(NumExpr("30,000,000"))
            .typed(proxy::OnChainClaimContractProxy)
            .remove_admin(address)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn admins(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::OnChainClaimContractProxy)
            .admins()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }
}
