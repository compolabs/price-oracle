contract;

dep data_structures;
dep errors;
dep events;
dep interface;

use std::{
    address::Address,
    chain::auth::msg_sender,
    identity::Identity,
    logging::log,
    result::Result,
    revert::require,
};

use data_structures::State;
use errors::AccessError;
use events::PriceUpdateEvent;
use interface::Oracle;

storage {
    price_eth: u64 = 0,
    price_dai: u64 = 0,
}

impl Oracle for Contract {
    fn owner() -> Identity {
        Identity::Address(~Address::from(owner))
    }

    #[storage(read)]
    fn price_eth() -> u64 {
        storage.price_eth
    }

    #[storage(read)]
    fn price_dai() -> u64 {
        storage.price_dai
    }

    #[storage(write)]
    fn set_price_eth(price_eth: u64) {
        // require(msg_sender().unwrap() == Identity::Address(~Address::from(owner)), AccessError::NotOwner);
        storage.price_eth = price_eth;
    }

    #[storage(write)]
    fn set_price_dai(price_dai: u64) {
        // require(msg_sender().unwrap() == Identity::Address(~Address::from(owner)), AccessError::NotOwner);
        storage.price_dai = price_dai;
    }
}
