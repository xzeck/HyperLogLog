use hyperloglog::HyperLogLog;
use std::{collections::HashSet, time::Instant};

fn main() {
    let mut hll: HyperLogLog<i64> = HyperLogLog::new(4);

    let insertion_start = Instant::now();
    let mut hashset: HashSet<i64> = HashSet::new();

    for i in 0i64..1_000_000 {
        hll.insert(i);
        hashset.insert(i);
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
