// 3) Using a custom hasher (e.g. RandomState) instead of the default SipHash
use hyperloglog::HyperLogLog;
use std::collections::hash_map::RandomState;

fn main() {
    // now uses RandomState::new() internally
    let mut hll = HyperLogLog::<u64, RandomState>::with_hasher(10, RandomState::new());

    for i in 1..=1000 {
        hll.insert(i);
    }

    println!("Estimate with RandomState: {}", hll.calculate_cardinality());
}
