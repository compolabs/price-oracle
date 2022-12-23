use dotenv::dotenv;
use fuels::prelude::*;

const RPC: &str = "node-beta-2.fuel.network";

pub async fn setup_wallet() -> (WalletUnlocked, Provider) {
    let provider = match Provider::connect(RPC).await {
        Ok(p) => p,
        Err(error) => panic!("❌ Problem creating provider: {:#?}", error),
    };

    dotenv().ok();
    let secret = match std::env::var("SECRET") {
        Ok(s) => s,
        Err(error) => panic!("❌ Cannot find .env file: {:#?}", error),
    };

    let wallet =
        WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider.clone()));

    return (wallet, provider);
}
