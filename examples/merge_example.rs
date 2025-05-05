use hyperloglog::HyperLogLog;


pub fn main() {

    let p = 10;

    let mut hll: HyperLogLog<i32> = HyperLogLog::new(p);

    let mut hll2: HyperLogLog<i32> = HyperLogLog::new(p);

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