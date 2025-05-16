// 4) Merging two hyperloglog instance (merged cardinality will fall between the cardinality of both the hyperloglog instance)
use hyperloglog::HyperLogLog;

pub fn main() {

    let p = 10;

    let mut hll: HyperLogLog<i32> = HyperLogLog::new(p).unwrap();

    let mut hll2: HyperLogLog<i32> = HyperLogLog::new(p).unwrap();

    for i in 1..10_000 {
        hll.insert(i);
        hll2.insert(i);
    }

    let result = hll.merge(&hll2);

    match result {
        Ok(_) => {},
        Err(e) => {
            println!("Error while merging {}", e);
        }
    }

    println!("Cardinality: {}", hll.calculate_cardinality());
}