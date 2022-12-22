use std::str::FromStr;

use fuels::{
    prelude::{Bech32ContractId, TxParameters},
    tx::{Address, ContractId},
};

use crate::utils::{
    local_tests_utils::OracleContract,
    number_utils::{format_units, parse_units},
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
            asset_id: "fuel1dn2xden4gugzv43x0f0kqpg38eydraf6dprwdqvussd8704d4l5s9ucfwd",
            coingeco_id: "binancecoin",
        },
        AssetConfig {
            symbol: "BTC",
            decimals: 8,
            default_price: 19000,
            asset_id: "fuel1s50vtcz05dy9hgrefv6qxzau6u8fd03g9n2znksrck8gmex5dsqqnweysd",
            coingeco_id: "bitcoin",
        },
        AssetConfig {
            symbol: "BUSD",
            decimals: 6,
            default_price: 1,
            asset_id: "fuel1lnwv27svtxlr3mk2h96am5pu8nfvkxzjj4akyt2kz0tqaj857tpq22vvqu",
            coingeco_id: "binance-usd",
        },
        AssetConfig {
            symbol: "USDC",
            decimals: 6,
            default_price: 1,
            asset_id: "fuel1uzwycupwd2pr0hg872fz3sfkesrkk7wtn58plzga88z5mj2sdxkqep4fya",
            coingeco_id: "usd-coin",
        },
        AssetConfig {
            symbol: "USDT",
            decimals: 6,
            default_price: 1,
            asset_id: "fuel1049jc47sepc4hc6jyjefx4a6y3zwgrmv68vjy75karv0f285fwjquwej5c",
            coingeco_id: "tether",
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
    let req = "https://api.coingecko.com/api/v3/simple/price?ids=binancecoin%2Cbitcoin%2Cbinance-usd%2Cusd-coin%2Ctether&vs_currencies=usd&include_market_cap=false&include_24hr_vol=false&include_24hr_change=false&include_last_updated_at=false&precision=8";
    let body = client.get(req).send().await.unwrap().text().await.unwrap();
    let responce: serde_json::Value = serde_json::from_str(body.as_str()).unwrap();

    for asset in assets {
        let bech32_address = Bech32ContractId::from_str(asset.asset_id)
            .expect("failed to create Bech32 address from string");

        let asset_id = ContractId::from(bech32_address);
        let symbol = asset.symbol;
        let last_price = methods.get_price(asset_id).simulate().await.unwrap();

        let price = match responce[asset.coingeco_id]["usd"].as_f64() {
            Some(p) => (p * 10f64.powf(asset.decimals as f64)).round() as u64,
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
            format_units(last_price.value.price, asset.decimals),
            last_price.value.price,
            format_units(new_price.value.price, asset.decimals),
            new_price.value.price
        );
    }
}
