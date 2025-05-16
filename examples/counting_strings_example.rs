// 2) Counting Strings
use hyperloglog::HyperLogLog;

fn main() {
    // p=12 → 4096 buckets
    let mut hll = HyperLogLog::<String>::new(12).unwrap();

    let fruits = ["apple", "banana", "apple", "cherry", "banana"];
    for &f in &fruits {
        hll.insert(f.to_string());
    }

    // should print “Estimate ~3”
    println!("Distinct fruits: {}", hll.calculate_cardinality());
}
