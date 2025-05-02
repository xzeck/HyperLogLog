use hyperloglog::HyperLogLog;
use std::{collections::HashSet, time::Instant};

fn main() {
    let mut hll = HyperLogLog::new(4);

    let insertion_start = Instant::now();
    let mut hashset: HashSet<i64> = HashSet::new();

    for _ in 0u64..1_000_000 {
        let value: i64 = rand::random();
        hll.insert(value);
        hashset.insert(value);
    }
    let insertion_end = insertion_start.elapsed();

    

    let start_cardinality = Instant::now();
    let estimate = hll.calculate_cardinality();
    let duration_cardinality = start_cardinality.elapsed();

    println!("Estimated cardinality: {}", estimate);
    println!("Actual cardinality: {}", hashset.len());
    println!("Differences in %{}", (estimate as f64/ (hashset.len() as f64)) * 100.0);
    println!("Time to insert: {:.2?}", insertion_end);
    println!("Time to calculate cardinality: {:.2?}", duration_cardinality);
}
