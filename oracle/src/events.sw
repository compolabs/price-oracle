library events;

pub struct PriceUpdateEvent {
    /// Updated price
    price_eth: u64,
    price_dai: u64,
}
