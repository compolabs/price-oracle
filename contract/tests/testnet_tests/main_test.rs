use std::str::FromStr;

use fuels::{
    prelude::{Bech32ContractId, TxParameters},
    tx::{Address, ContractId},
};

use crate::utils::{
    local_tests_utils::OracleContract, number_utils::format_units,
    testnet_tests_utils::setup_wallet,
};

const ORACLE_ADDRESS: &str = "0x4bf2826201fb74fc479a6a785cb70f2ce8e45b67010acfd47906993d130a21ff";
#[derive(Debug)]
struct AssetConfig<'a> {
    symbol: &'a str,
    asset_id: &'a str,
    default_price: u64,
    coingeco_id: &'a str,
}

#[tokio::test]
async fn main_test() {
    let assets = vec![
        AssetConfig {
            symbol: "COMP",
            default_price: 50 * 10u64.pow(9),
            asset_id: "0x13397cf760e15cd30194fa9d884cf4dd810c5d9e6459a4053e65f74f80b92f32",
            coingeco_id: "compound-governance-token",
        },
        AssetConfig {
            symbol: "SWAY",
            default_price: 50 * 10u64.pow(9),
            asset_id: "0x99075448d291a8f8f69e5f3d25a309c38ad38def9f709a69ae4a2aeaed1701fe",
            coingeco_id: "compound-governance-token",
        },
        AssetConfig {
            symbol: "BTC",
            default_price: 19000 * 10u64.pow(9),
            asset_id: "0xdd17dda6eeee55f6d327020e6d61b9fa7b3c2ab205c46cdca690a46966f4e1c7",
            coingeco_id: "bitcoin",
        },
        AssetConfig {
            symbol: "USDC",
            default_price: 1 * 10u64.pow(9),
            asset_id: "0xd7d5e5c1220872e6f42b38f85ae80c6072b1b4723e7a7218bbf6717aca962536",
            coingeco_id: "usd-coin",
        },
        AssetConfig {
            symbol: "UNI",
            default_price: 5 * 10u64.pow(9),
            asset_id: "0x76c4fda9074c4509eaf2652f82bace86e2c7a21bf9faff7bf6228034ebc0f8a2",
            coingeco_id: "uniswap",
        },
        AssetConfig {
            symbol: "LINK",
            default_price: 5 * 10u64.pow(9),
            asset_id: "0x71be783354a9bccfa9de0e7edf291797775e4a730d0922a9675258dbb47f557b",
            coingeco_id: "chainlink",
        },
        AssetConfig {
            symbol: "ETH",
            default_price: 1200 * 10u64.pow(9),
            asset_id: "0x0000000000000000000000000000000000000000000000000000000000000000",
            coingeco_id: "ethereum",
        },
    ];

    let (wallet, _provider) = setup_wallet().await;
    let tx_params = TxParameters::new(Some(1), None, None);
    let oracle_dapp_id = Bech32ContractId::from(ContractId::from_str(ORACLE_ADDRESS).unwrap());
    let oracle = OracleContract::new(oracle_dapp_id, wallet.clone());
    let methods = oracle.methods();
    let _res = methods
        .initialize(Address::from(wallet.address()))
        .tx_params(tx_params)
        .call()
        .await;
    println!("{} Initialize\n", if _res.is_ok() { "✅" } else { "❌" });

    let client = reqwest::Client::new();
    let req = "https://api.coingecko.com/api/v3/simple/price?ids=compound-governance-token%2Cbinancecoin%2Cbitcoin%2Cbinance-usd%2Cusd-coin%2Ctether%2Cuniswap%2Cethereum%2Cchainlink&vs_currencies=usd&include_market_cap=false&include_24hr_vol=false&include_24hr_change=false&include_last_updated_at=false&precision=9";
    let body = client.get(req).send().await.unwrap().text().await.unwrap();
    let responce: serde_json::Value = serde_json::from_str(body.as_str()).unwrap();

    let mut prices: Vec<(ContractId, u64)> = vec![];
    let mut message = String::from("💰 Price oracle uppdate\n");
    for asset in assets {
        let contract_id = ContractId::from_str(asset.asset_id)
            .expect("failed to create ContractId address from string");
        let bech32_address = Bech32ContractId::from(contract_id);

        let asset_id = ContractId::from(bech32_address);
        let symbol = asset.symbol;

        let price = match responce[asset.coingeco_id]["usd"].as_f64() {
            Some(p) => (p * 10f64.powf(9f64)).round() as u64,
            _ => asset.default_price,
        };
        prices.push((asset_id, price));

        message += format!("1 {symbol} = ${} ({})\n", format_units(price, 9), price).as_str();
    }
    let _res = methods.set_prices(prices).tx_params(tx_params).call().await.unwrap();
    println!("{message}");
}
