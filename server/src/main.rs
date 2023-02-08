use dotenv::dotenv;
use fuels::{
    prelude::{abigen, Bech32ContractId, ContractId, Provider, WalletUnlocked, BASE_ASSET_ID},
    types::{Address, Identity},
};
use std::{env, fs::read_to_string, str::FromStr, thread::sleep, time::Duration};
mod utils;
use utils::print_swaygang_sign::print_swaygang_sign;

use crate::utils::oracle_abi_calls::oracle_abi_calls::set_prices;

abigen!(Contract(
    name = "OracleContract",
    abi = "../contract/out/debug/oracle-abi.json"
));

const RPC: &str = "node-beta-2.fuel.network";

#[tokio::main]
async fn main() {
    let deploy_config_json_str =
        read_to_string("../tokens.json").expect("Should have been able to read the file");
    let assets: serde_json::Value = serde_json::from_str(deploy_config_json_str.as_str()).unwrap();
    let assets = assets.as_array().unwrap();
    // contract
    let provider = match Provider::connect(RPC).await {
        Ok(p) => p,
        Err(error) => panic!("❌ Problem creating provider: {:#?}", error),
    };
    dotenv().ok();
    let secret = env::var("SECRET").expect("❌ Expected a account secret in the environment");
    let wallet = WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider));

    let oracle_address =
        env::var("ORACLE_ADDRESS").expect("❌ Expected a ORACLE_ADDRESS in the environment");
    let bech32_id = Bech32ContractId::from(ContractId::from_str(oracle_address.as_str()).unwrap());
    let oracle = OracleContract::new(bech32_id, wallet.clone());

    let frequency = match env::var("FREQUENCY") {
        Ok(f) => f.parse::<u64>().expect("❌ Invalid FREQUENCY"),
        _ => 60,
    };

    print_swaygang_sign("✅ Oracle is alive");
    let owner = oracle.methods().owner().simulate().await.unwrap().value;
    println!("Oracle owner   = {:?}", owner);
    println!("Wallet address = {:?}\n", Identity::Address(Address::from(wallet.address())));
    loop {
        let c = reqwest::Client::new();
        let req = "https://api.coingecko.com/api/v3/simple/price?ids=compound-governance-token%2Cbinancecoin%2Cbitcoin%2Cbinance-usd%2Cusd-coin%2Ctether%2Cuniswap%2Cethereum%2Cchainlink&vs_currencies=usd&include_market_cap=false&include_24hr_vol=false&include_24hr_change=false&include_last_updated_at=false&precision=9";
        let body = c.get(req).send().await.unwrap().text().await.unwrap();
        let responce: serde_json::Value = serde_json::from_str(body.as_str()).unwrap();
        let mut prices: Vec<(ContractId, u64)> = vec![];
        let mut message = String::from("🪬 Price oracle update\n");
        for asset in assets {
            let contract_id = ContractId::from_str(asset["asset_id"].as_str().unwrap())
                .expect("failed to create ContractId address from string");
            let bech32_address = Bech32ContractId::from(contract_id);

            let asset_id = ContractId::from(bech32_address);
            let symbol = asset["symbol"].as_str().unwrap();

            let price = match responce[asset["coingeco_id"].as_str().unwrap()]["usd"].as_f64() {
                Some(p) => (p * 10f64.powf(9f64)).round() as u64,
                _ => (asset["default_price"].as_f64().unwrap() * 10f64.powf(9f64)) as u64,
            };
            prices.push((asset_id, price));

            message += format!("1 {symbol} = ${}\n", price as f64 / 10f64.powf(9f64)).as_str();
        }
        let res = set_prices(&oracle, prices).await;
        if res.is_ok() {
            message += format!("\n⛽️ Gas used: {}", res.unwrap().gas_used).as_str();
            let balance = wallet.get_asset_balance(&BASE_ASSET_ID).await.unwrap();
            message += format!("\n⚖️ Balance: {} ETH", balance as f64 / 10f64.powf(9f64)).as_str();
            message += format!("\n👁 Oracle address: {oracle_address}").as_str();
            message += format!("\n-----------------------------------").as_str();
            println!("{message}");
        } else {
            println!("❌ Cannot update prices");
            println!("{}", res.err().unwrap());
        }

        sleep(Duration::from_secs(frequency));
    }
}
