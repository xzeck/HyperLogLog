mod utils;
use utils::utils::calculate_bounds;
use hyperloglog::{HyperLogLog, ToBytes};

// A type whose to_bytes() always returns the same bytes, forcing hash collisions
#[derive(Clone)]
struct Colliding(u64);

impl ToBytes for Colliding {
    fn to_bytes(&self) -> Vec<u8> {
        vec![0; 8]
    }
}

/// Sequential test of insert and cardinality
#[test]
fn test_insert_and_cardinality() {
    let p = 5;
    let mut hll = HyperLogLog::<i64>::new(p);
    let n: u64 = 10_000;
    let tolerance = 1.04f64 / ((1u64 << p) as f64).sqrt();

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

/// Empty set should return zero
#[test]
fn test_empty_cardinality() {
    let hll = HyperLogLog::<i64>::new(5);
    assert_eq!(hll.calculate_cardinality(), 0);
}

/// Large sequential range tests randomness via hash
#[test]
fn test_large_sequential_numbers() {
    let p = 10;
    let mut hll = HyperLogLog::<i64>::new(p);
    let n: u64 = 100_000;
    let tolerance = 1.04f64 / ((1u64 << p) as f64).sqrt();

    for i in 1..=n {
        hll.insert(i as i64);
    }

    let (lo, hi) = calculate_bounds(n, tolerance);
    let est = hll.calculate_cardinality();
    assert!(est >= lo && est <= hi,
        "p={}, sequential n={} -> estimate {} not in [{}, {}]",
        p, n, est, lo, hi
    );
}

/// Repeated inserts of same value should not change cardinality
#[test]
fn test_repeated_inserts() {
    let mut hll = HyperLogLog::<i64>::new(5);
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

/// High-precision sequential test
#[test]
fn test_high_precision_sequential() {
    let p = 12;
    let mut hll = HyperLogLog::<i32>::new(p);
    let n: u64 = 100_000;
    let tolerance = 1.04f64 / ((1u64 << p) as f64).sqrt();

    for i in 1..=n {
        hll.insert(i as i32);
    }

    let (lo, hi) = calculate_bounds(n, tolerance);
    let est = hll.calculate_cardinality();
    assert!(est >= lo && est <= hi,
        "p={}, sequential i32 n={} -> estimate {} not in [{}, {}]",
        p, n, est, lo, hi
    );
}

/// Min value of p test
#[test]
fn test_min_value_of_p() {
    let p = 4;
    let mut hll = HyperLogLog::<i64>::new(p);
    let n: u64 = 10_000;
    let tolerance = 1.04f64 / ((1u64 << p) as f64).sqrt();

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

/// Max value of p test
#[test]
fn test_max_value_of_p() {
    let p = 16;
    let mut hll = HyperLogLog::<i64>::new(p);
    let n: u64 = 50_000;
    let tolerance = 1.04f64 / ((1u64 << p) as f64).sqrt();

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

/// Uncommon dataset size (few elements)
#[test]
fn test_uncommon_dataset_size() {
    let p = 16;
    let mut hll = HyperLogLog::<i64>::new(p);
    let values = [10, 20, 30, 40, 50];
    for &v in &values {
        hll.insert(v);
    }
    let n: u64 = values.len() as u64;
    let tolerance = 1.04f64 / ((1u64 << p) as f64).sqrt();

    let (lo, hi) = calculate_bounds(n, tolerance);
    let est = hll.calculate_cardinality();
    assert!(est >= lo && est <= hi,
        "p={}, values={:?} -> estimate {} not in [{}, {}]",
        p, values, est, lo, hi
    );
}

/// Very large numbers test
#[test]
fn test_very_large_numbers() {
    let p = 10;
    let mut hll = HyperLogLog::<i64>::new(p);
    let values: Vec<i64> = (0..10_000).map(|i| i64::MAX - i as i64).collect();
    for &v in &values {
        hll.insert(v);
    }
    let n: u64 = values.len() as u64;
    let tolerance = 1.04f64 / ((1u64 << p) as f64).sqrt();

    let (lo, hi) = calculate_bounds(n, tolerance);
    let est = hll.calculate_cardinality();
    assert!(est >= lo && est <= hi,
        "p={}, very large inputs -> estimate {} not in [{}, {}]",
        p, est, lo, hi
    );
}

/// Hash collisions test
#[test]
fn test_hash_collisions() {
    let p = 8;
    let mut hll = HyperLogLog::<Colliding>::new(p);
    for i in 1..=3 {
        hll.insert(Colliding(i));
    }
    let (lo, hi) = calculate_bounds(1, 1.04f64 / ((1u64 << p) as f64).sqrt());
    let est = hll.calculate_cardinality();
    assert!(est >= lo && est <= hi,
        "p={}, collisions of 3 values -> estimate {} not in [{}, {}]",
        p, est, lo, hi
    );
}
