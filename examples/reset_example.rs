// 5) Reset example
use hyperloglog::HyperLogLog;

pub fn main() {

    let p = 10;
    let mut hll: HyperLogLog<i32> = HyperLogLog::new(p);

    for i in 1..=10 {
        hll.insert(i);
    }

    println!("Cadinality: {}", hll.calculate_cardinality());

    hll.reset();

    for i in 1..=10_000 {
        hll.insert(i);
    }

    println!("Cardinality: {}", hll.calculate_cardinality());
}