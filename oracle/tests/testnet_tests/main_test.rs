use std::str::FromStr;

use fuels::{
    prelude::{Bech32ContractId, Tokenizable, TxParameters, Bech32Address},
    tx::{Address, ContractId},
};

use crate::utils::{
    local_tests_utils::OracleContract,
    number_utils::{format_units, parse_units},
    testnet_tests_utils::setup_wallet,
};

const ORACLE_ADDRESS: &str = "0xde764394c83bb3c8a3aec5f75f383ff86e64728964fab4469df5910ca01b1a59";
struct AssetConfig {
    symbol: String,
    assetId: String,
    decimals: u8,
    price: u64,
}

#[tokio::test]
async fn main_test() {
    let assets = vec![
        AssetConfig {
            symbol: String::from("BNB"),
            decimals: 8,
            price: 250,
            assetId: String::from(
                "fuel1dn2xden4gugzv43x0f0kqpg38eydraf6dprwdqvussd8704d4l5s9ucfwd",
            ),
        },
        AssetConfig {
            symbol: String::from("BTC"),
            decimals: 8,
            price: 19000,
            assetId: String::from(
                "fuel1s50vtcz05dy9hgrefv6qxzau6u8fd03g9n2znksrck8gmex5dsqqnweysd",
            ),
        },
        AssetConfig {
            symbol: String::from("BUSD"),
            decimals: 6,
            price: 1,
            assetId: String::from(
                "fuel1lnwv27svtxlr3mk2h96am5pu8nfvkxzjj4akyt2kz0tqaj857tpq22vvqu",
            ),
        },
        AssetConfig {
            symbol: String::from("USDC"),
            decimals: 6,
            price: 1,
            assetId: String::from(
                "fuel1uzwycupwd2pr0hg872fz3sfkesrkk7wtn58plzga88z5mj2sdxkqep4fya",
            ),
        },
        AssetConfig {
            symbol: String::from("USDT"),
            decimals: 6,
            price: 1,
            assetId: String::from(
                "fuel1049jc47sepc4hc6jyjefx4a6y3zwgrmv68vjy75karv0f285fwjquwej5c",
            ),
        },
    ];

    let (wallet, _provider) = setup_wallet().await;
    let tx_params = TxParameters::new(Some(1), Some(1000000), None);
    let oracle_dapp_id = Bech32ContractId::from(ContractId::from_str(ORACLE_ADDRESS).unwrap());
    let oracle = OracleContract::new(oracle_dapp_id, wallet.clone());
    let _res = oracle
        .methods()
        .initialize(Address::from(wallet.address()))
        .tx_params(tx_params)
        .call()
        .await;
    println!("{} Initialize\n", if _res.is_ok() { "✅" } else { "❌" });

    for asset in assets {
        let bech32_address =
            Bech32ContractId::from_str(asset.assetId.as_str()).expect("failed to create Bech32 address from string");

        let asset_id = ContractId::from(bech32_address);
        let symbol = asset.symbol;
        let last_price = oracle
            .methods()
            .get_price(asset_id)
            .simulate()
            .await
            .unwrap()
            .value
            .price;
        let _res = oracle
            .methods()
            .set_price(asset_id, parse_units(asset.price, asset.decimals))
            .tx_params(tx_params)
            .call()
            .await;
        let new_price = oracle
            .methods()
            .get_price(asset_id)
            .simulate()
            .await
            .unwrap()
            .value
            .price;
        println!("{} Set price", if _res.is_ok() { "✅" } else { "❌" },);
        println!(
            "{symbol} price was changed {}{symbol} -> {}{symbol}",
            format_units(last_price, asset.decimals),
            format_units(new_price, asset.decimals)
        );
    }
}
