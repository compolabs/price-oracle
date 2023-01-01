use std::str::FromStr;

use fuels::{
    prelude::{Bech32ContractId, TxParameters},
    tx::{Address, ContractId},
};

use crate::utils::{
    local_tests_utils::OracleContract,
    number_utils::{format_units},
    testnet_tests_utils::setup_wallet,
};

const ORACLE_ADDRESS: &str = "0xde764394c83bb3c8a3aec5f75f383ff86e64728964fab4469df5910ca01b1a59";
#[derive(Debug)]
struct AssetConfig<'a> {
    symbol: &'a str,
    asset_id: &'a str,
    decimals: u8,
    default_price: u64,
    coingeco_id: &'a str,
}

#[tokio::test]
async fn main_test() {
    let assets = vec![
        AssetConfig {
            symbol: "BNB",
            decimals: 8,
            default_price: 250,
            asset_id: "0x6cd466e67547102656267a5f6005113e48d1f53a6846e6819c841a7f3eadafe9",
            coingeco_id: "binancecoin",
        },
        AssetConfig {
            symbol: "BTC",
            decimals: 8,
            default_price: 19000,
            asset_id: "0x851ec5e04fa3485ba0794b34030bbcd70e96be282cd429da03c58e8de4d46c00",
            coingeco_id: "bitcoin",
        },
        AssetConfig {
            symbol: "BUSD",
            decimals: 6,
            default_price: 1,
            asset_id: "0xfcdcc57a0c59be38eecab975ddd03c3cd2cb1852957b622d5613d60ec8f4f2c2",
            coingeco_id: "binance-usd",
        },
        AssetConfig {
            symbol: "USDC",
            decimals: 6,
            default_price: 1,
            asset_id: "0xe09c4c702e6a8237dd07f29228c136cc076b79cb9d0e1f891d39c54dc95069ac",
            coingeco_id: "usd-coin",
        },
        AssetConfig {
            symbol: "USDT",
            decimals: 6,
            default_price: 1,
            asset_id: "0x7d4b2c57d0c8715be35224b29357ba2444e40f6cd1d9227a96e8d8f4a8f44ba4",
            coingeco_id: "tether",
        },
        AssetConfig {
            symbol: "UNI",
            decimals: 9,
            default_price: 5,
            asset_id: "0xcc28b139c7664ac9cddc2c01c00559fbbebd6fa8a879db341adf3a4aafdaa137",
            coingeco_id: "uniswap",
        },
        AssetConfig {
            symbol: "LINK",
            decimals: 9,
            default_price: 5,
            asset_id: "0x579cd9e73d2471fd0ce20156e06e34c09cdf2fd655c993af8d236185305461ee",
            coingeco_id: "chainlink",
        },
        AssetConfig {
            symbol: "ETH",
            decimals: 9,
            default_price: 1200,
            asset_id: "0x0000000000000000000000000000000000000000000000000000000000000000",
            coingeco_id: "ethereum",
        },
    ];

    let (wallet, _provider) = setup_wallet().await;
    let tx_params = TxParameters::new(Some(1), Some(1000000), None);
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
    let req = "https://api.coingecko.com/api/v3/simple/price?ids=binancecoin%2Cbitcoin%2Cbinance-usd%2Cusd-coin%2Ctether%2Cuniswap%2Cethereum%2Cchainlink&vs_currencies=usd&include_market_cap=false&include_24hr_vol=false&include_24hr_change=false&include_last_updated_at=false&precision=9";
    let body = client.get(req).send().await.unwrap().text().await.unwrap();
    let responce: serde_json::Value = serde_json::from_str(body.as_str()).unwrap();

    for asset in assets {
        let contract_id = ContractId::from_str(asset.asset_id)
            .expect("failed to create ContractId address from string");
        let bech32_address = Bech32ContractId::from(contract_id);

        let asset_id = ContractId::from(bech32_address);
        let symbol = asset.symbol;
        let last_price = methods.get_price(asset_id).simulate().await.unwrap();

        let price = match responce[asset.coingeco_id]["usd"].as_f64() {
            Some(p) => (p * 10f64.powf(9f64)).round() as u64,
            _ => asset.default_price,
        };

        let _res = methods
            .set_price(asset_id, price)
            .tx_params(tx_params)
            .call()
            .await;
        let new_price = methods.get_price(asset_id).simulate().await.unwrap();
        println!("{} Set price", if _res.is_ok() { "✅" } else { "❌" },);
        println!(
            "{symbol} price was changed {} {symbol} ({}) -> {} {symbol} ({})",
            format_units(last_price.value.price, 9),
            last_price.value.price,
            format_units(new_price.value.price, 9),
            new_price.value.price
        );
    }
}
