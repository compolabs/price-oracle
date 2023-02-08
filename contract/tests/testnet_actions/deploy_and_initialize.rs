use fuels::{
    prelude::{abigen, Contract, StorageConfiguration, TxParameters},
    tx::Salt,
    types::Address,
};
use rand::Rng;

use crate::utils::testnet_tests_utils::setup_wallet;

abigen!(Contract(
    name = "OracleContract",
    abi = "out/debug/oracle-abi.json"
));

#[tokio::test]
async fn deploy_and_initialize() {
    let (wallet, _) = setup_wallet().await;

    let mut rng = rand::thread_rng();
    let salt = rng.gen::<[u8; 32]>();

    let id = Contract::deploy_with_parameters(
        "./out/debug/oracle.bin",
        &wallet,
        TxParameters::new(Some(1), None, None),
        StorageConfiguration::with_storage_path(Some(
            "./out/debug/oracle-storage_slots.json".to_string(),
        )),
        Salt::from(salt),
    )
    .await
    .unwrap();
    let instance = OracleContract::new(id, wallet.clone());
    
    let tx_params = TxParameters::new(Some(1), None, None);
    let _res = instance
        .methods()
        .initialize(Address::from(wallet.address()))
        .tx_params(tx_params)
        .call()
        .await;
    println!("{} Initialize", if _res.is_ok() { "✅" } else { "❌" });

    println!(
        "✅ Oracle contract deployed\nHash:   0x{}\nBech32: {}",
        instance.contract_id().hash(),
        instance.contract_id()
    );
}
