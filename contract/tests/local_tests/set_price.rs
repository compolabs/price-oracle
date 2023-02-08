use crate::utils::local_tests_utils::abi_calls::{get_price, initialize, set_price};
use crate::utils::local_tests_utils::test_helpers::{
    get_oracle_contract_instance, get_token_contract_instance, init_wallet,
};
use fuels::tx::{Address, ContractId};

use crate::utils::{local_tests_utils::test_helpers::DeployTokenConfig, number_utils::parse_units};
mod success {

    use super::*;

    #[tokio::test]
    async fn can_set_price() {
        let wallet = init_wallet().await;
        let oracle = get_oracle_contract_instance(&wallet).await;

        let config = DeployTokenConfig {
            name: String::from("BNB"),
            symbol: String::from("BNB"),
            decimals: 8,
            mint_amount: 5,
        };

        let bnb = get_token_contract_instance(&wallet, &config).await;
        let asset_id = ContractId::from(bnb.contract_id());

        initialize(&oracle, Address::from(wallet.address())).await;

        let set_price_amount: u64 = parse_units(250, config.decimals);
        set_price(&oracle, asset_id, set_price_amount).await;

        let price = get_price(&oracle, asset_id).await.price;
        println!("{}", price);
        assert_eq!(price, set_price_amount);
    }
}
