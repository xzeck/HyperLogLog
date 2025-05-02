use hyperloglog::{HyperLogLog, ToBytes};
use rand::random;

mod utils;
use utils::utils::calculate_bounds;

// A type whose to_bytes() always returns the same bytes, forcing hash collisions
#[derive(Clone)]
struct Colliding(u64);

impl ToBytes for Colliding {
    fn to_bytes(&self) -> Vec<u8> {
        vec![0; 8]
    }
}

#[test]
fn test_insert_and_cardinality() {
    let p = 5;
    let mut hll = HyperLogLog::<i64>::new(p);
    let n = 10_000;
    let tolerance = 1.04f64 / ((1u64 << p) as f64).sqrt();

    // Insert distinct values 1..=n
    for i in 1..=n {
        hll.insert(i as i64);
    }

    let (lo, hi) = calculate_bounds(n, tolerance);
    let est = hll.calculate_cardinality();
    assert!(est >= lo && est <= hi,
        "p={}, n={} -> estimate {} not in [{}, {}]",
        p, n, est, lo, hi
    );
}

#[test]
fn test_empty_cardinality() {
    let hll = HyperLogLog::<i64>::new(5);
    assert_eq!(hll.calculate_cardinality(), 0);
}

#[test]
fn test_large_random_numbers() {
    let p = 10;
    let mut hll = HyperLogLog::<i64>::new(p);
    let n = 100_000;
    let tolerance = 1.04f64 / ((1u64 << p) as f64).sqrt();

    for _ in 0..n {
        let value: i64 = random();
        hll.insert(value);
    }

    let (lo, hi) = calculate_bounds(n, tolerance);
    let est = hll.calculate_cardinality();
    assert!(est >= lo && est <= hi,
        "p={}, random n={} -> estimate {} not in [{}, {}]",
        p, n, est, lo, hi
    );
}

#[test]
fn test_repeated_inserts() {
    let mut hll = HyperLogLog::<i64>::new(5);
    // Insert one value multiple times
    hll.insert(42);
    let before = hll.calculate_cardinality();
    for _ in 0..1_000_000 {
        hll.insert(42);
    }
    let after = hll.calculate_cardinality();
    assert_eq!(before, after,
        "Repeated inserts changed cardinality from {} to {}",
        before, after
    );
}

#[test]
fn test_high_precision_cardinality() {
    let p = 10;
    let mut hll = HyperLogLog::<i32>::new(p);
    let n = 100_000;
    let tolerance = 1.04f64 / ((1u64 << p) as f64).sqrt();

    for _ in 0..n {
        let value: i32 = random();
        hll.insert(value);
    }

    let (lo, hi) = calculate_bounds(n, tolerance);
    let est = hll.calculate_cardinality();
    assert!(est >= lo && est <= hi,
        "p={}, random i32 n={} -> estimate {} not in [{}, {}]",
        p, n, est, lo, hi
    );
}

#[test]
fn test_min_value_of_p() {
    let p = 4;
    let mut hll = HyperLogLog::<i64>::new(p);
    let n = 1_000;
    let tolerance = 1.04f64 / ((1u64 << p) as f64).sqrt();

    for _ in 0..n {
        let value: i64 = random();
        hll.insert(value);
    }

    let (lo, hi) = calculate_bounds(n, tolerance);
    let est = hll.calculate_cardinality();
    assert!(est >= lo && est <= hi,
        "p={}, n={} -> estimate {} not in [{}, {}]",
        p, n, est, lo, hi
    );
}

#[test]
fn test_max_value_of_p() {
    let p = 16;
    let mut hll = HyperLogLog::<i64>::new(p);
    let n = 50_000;
    let tolerance = 1.04f64 / ((1u64 << p) as f64).sqrt();

    for _ in 0..n {
        let value: i64 = random();
        hll.insert(value);
    }

    let (lo, hi) = calculate_bounds(n, tolerance);
    let est = hll.calculate_cardinality();
    assert!(est >= lo && est <= hi,
        "p={}, n={} -> estimate {} not in [{}, {}]",
        p, n, est, lo, hi
    );
}

#[test]
fn test_uncommon_dataset_size() {
    let p = 16;
    let mut hll = HyperLogLog::<i64>::new(p);
    let values = [10, 20, 30, 40, 50];
    for &v in &values {
        hll.insert(v);
    }
    let n = values.len() as u64;
    let tolerance = 1.04f64 / ((1u64 << p) as f64).sqrt();

    let (lo, hi) = calculate_bounds(n, tolerance);
    let est = hll.calculate_cardinality();
    assert!(est >= lo && est <= hi,
        "p={}, values={:?} -> estimate {} not in [{}, {}]",
        p, values, est, lo, hi
    );
}

#[test]
fn test_very_large_numbers() {
    let p = 10;
    let mut hll = HyperLogLog::<i64>::new(p);
    let values: Vec<i64> = (0..1_000).map(|i| i64::MAX - i).collect();
    for &v in &values {
        hll.insert(v);
    }
    let n = values.len() as u64;
    let tolerance = 1.04f64 / ((1u64 << p) as f64).sqrt();

    let (lo, hi) = calculate_bounds(n, tolerance);
    let est = hll.calculate_cardinality();
    assert!(est >= lo && est <= hi,
        "p={}, very large inputs -> estimate {} not in [{}, {}]",
        p, est, lo, hi
    );
}

#[test]
fn test_hash_collisions() {
    let p = 8;
    let mut hll = HyperLogLog::<Colliding>::new(p);
    for i in 1..=3 {
        hll.insert(Colliding(i));
    }
    let tolerance = 1.04f64 / ((1u64 << p) as f64).sqrt();
    let (lo, hi) = calculate_bounds(1, tolerance);
    let est = hll.calculate_cardinality();
    assert!(est >= lo && est <= hi,
        "p={}, collisions of 3 values -> estimate {} not in [{}, {}]",
        p, est, lo, hi
    );
}
