use hyperloglog::HyperLogLog;
use std::time::Instant;

fn main() {
    let mut hll = HyperLogLog::new(10);

    let insertion_start = Instant::now();
    for _ in 0u64..1_000_000_000_0 {
        let value: i64 = rand::random();
        hll.insert(value);
    }
    let insertion_end = insertion_start.elapsed();

    

    let start_cardinality = Instant::now();
    let estimate = hll.calculate_cardinality();
    let duration_cardinality = start_cardinality.elapsed();

    println!("Estimated cardinality: {}", estimate);
    println!("Time to insert: {:.2?}", insertion_end);
    println!("Time to calculate cardinality: {:.2?}", duration_cardinality);
}
