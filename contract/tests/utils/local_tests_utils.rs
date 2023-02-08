use fuels::prelude::*;
use rand::prelude::Rng;

use crate::utils::number_utils::parse_units;

abigen!(
    Contract(name = "OracleContract", abi = "out/debug/oracle-abi.json"),
    Contract(
        name = "TokenContract",
        abi = "tests/artefacts/token/token_contract-abi.json"
    )
);

pub mod abi_calls {
    use fuels::programs::call_response::FuelCallResponse;

    use super::*;

    pub async fn initialize(contract: &OracleContract, owner: Address) -> FuelCallResponse<()> {
        contract.methods().initialize(owner).call().await.unwrap()
    }

    pub async fn get_price(contract: &OracleContract, asset_id: ContractId) -> Price {
        contract
            .methods()
            .get_price(asset_id)
            .call()
            .await
            .unwrap()
            .value
    }

    pub async fn set_price(
        contract: &OracleContract,
        asset_id: ContractId,
        new_price: u64,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .set_price(asset_id, new_price)
            .call()
            .await
            .unwrap()
    }
}

pub mod test_helpers {
    use fuels::types::SizedAsciiString;

    use super::{abigen_bindings::token_contract_mod, *};

    pub struct DeployTokenConfig {
        pub name: String,
        pub symbol: String,
        pub decimals: u8,
        pub mint_amount: u64,
    }

    pub async fn init_wallet() -> WalletUnlocked {
        let mut wallets = launch_custom_provider_and_get_wallets(
            WalletsConfig::new(
                Some(1),             /* Single wallet */
                Some(1),             /* Single coin (UTXO) */
                Some(1_000_000_000), /* Amount per coin */
            ),
            None,
            None,
        )
        .await;
        wallets.pop().unwrap()
    }

    pub async fn get_oracle_contract_instance(wallet: &WalletUnlocked) -> OracleContract {
        let id = Contract::deploy(
            "./out/debug/oracle.bin",
            &wallet,
            TxParameters::default(),
            StorageConfiguration::default(),
        )
        .await
        .unwrap();

        OracleContract::new(id, wallet.clone())
    }

    pub async fn get_token_contract_instance(
        wallet: &WalletUnlocked,
        deploy_config: &DeployTokenConfig,
    ) -> TokenContract {
        let mut name = deploy_config.name.clone();
        let mut symbol = deploy_config.symbol.clone();
        let decimals = deploy_config.decimals;

        let mut rng = rand::thread_rng();
        let salt = rng.gen::<[u8; 32]>();

        let id = Contract::deploy_with_parameters(
            "./tests/artefacts/token/token_contract.bin",
            &wallet,
            TxParameters::default(),
            StorageConfiguration::default(),
            Salt::from(salt),
        )
        .await
        .unwrap();

        let instance = TokenContract::new(id, wallet.clone());
        let methods = instance.methods();

        let mint_amount = parse_units(deploy_config.mint_amount, decimals);
        name.push_str(" ".repeat(32 - deploy_config.name.len()).as_str());
        symbol.push_str(" ".repeat(8 - deploy_config.symbol.len()).as_str());

        let config: token_contract_mod::Config = token_contract_mod::Config {
            name: SizedAsciiString::<32>::new(name).unwrap(),
            symbol: SizedAsciiString::<8>::new(symbol).unwrap(),
            decimals,
        };

        let _res = methods
            .initialize(config, mint_amount, Address::from(wallet.address()))
            .call()
            .await;
        let _res = methods.mint().append_variable_outputs(1).call().await;

        instance
    }
}
