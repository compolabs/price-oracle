library interface;

use std::identity::Identity;

abi Oracle {
    fn owner() -> Identity;

    #[storage(read)]
    fn price_eth() -> u64;

    #[storage(read)]
    fn price_dai() -> u64;

    #[storage(write)]
    fn set_price_eth(price_eth: u64);

    #[storage(write)]
    fn set_price_dai(price_dai: u64);
}
