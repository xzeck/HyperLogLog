// 1) Counting integers with the default (deterministic) hasher
use hyperloglog::HyperLogLog;

fn main() {
    // p=10 → 1024 buckets
    let mut hll = HyperLogLog::<u64>::new(10);

    // insert 1..=5000
    for i in 1..=5000 {
        hll.insert(i);
    }

    println!("Estimate ~5000: {}", hll.calculate_cardinality());
}
