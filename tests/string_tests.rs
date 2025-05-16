mod utils;
use utils::utils::calculate_bounds;
use hyperloglog::{HyperLogLog, ToBytes};

/// Test inserting no strings yields zero cardinality
#[test]
fn test_no_inserts_string() {
    let hll = HyperLogLog::<String>::new(10).unwrap();;
    assert_eq!(hll.calculate_cardinality(), 0);
}

/// Test inserting empty string repeatedly yields cardinality of 1
#[test]
fn test_empty_string() {
    let mut hll = HyperLogLog::<String>::new(10).unwrap();
    for _ in 0..1000 {
        hll.insert(String::new());
    }
    assert_eq!(hll.calculate_cardinality(), 1);
}

/// Test small distinct set of strings with duplicates
#[test]
fn test_small_distinct_strings() {
    let mut hll = HyperLogLog::<String>::new(10).unwrap();
    let samples = vec!["a", "b", "c", "d", "a", "b", "c"];
    for &s in &samples {
        hll.insert(s.to_string());
    }
    let estimate = hll.calculate_cardinality();
    // Expect ~4 distinct values
    assert!((3..=5).contains(&estimate), "estimate {} not in [3,5]", estimate);
}

/// Test case sensitivity: "Foo" != "foo"
#[test]
fn test_case_sensitivity() {
    let mut hll = HyperLogLog::<String>::new(10).unwrap();
    hll.insert("Foo".to_string());
    hll.insert("foo".to_string());
    assert_eq!(hll.calculate_cardinality(), 2);
}

/// Test whitespace variants are distinct
#[test]
fn test_whitespace_variants() {
    let mut hll = HyperLogLog::<String>::new(10).unwrap();
    let variants = vec!["test", " test", "test ", " test "]; 
    for &v in &variants {
        hll.insert(v.to_string());
    }
    assert_eq!(hll.calculate_cardinality(), variants.len() as u64);
}

/// Test unicode/multilingual strings
#[test]
fn test_unicode_strings() {
    let mut hll = HyperLogLog::<String>::new(10).unwrap();
    let words = vec!["こんにちは", "你好", "здравствуйте", "こんにちは"];
    for &w in &words {
        hll.insert(w.to_string());
    }
    // distinct: 3
    assert_eq!(hll.calculate_cardinality(), 3);
}

/// Test very long strings
#[test]
fn test_very_long_strings() {
    let mut hll = HyperLogLog::<String>::new(12).unwrap();
    let long = std::iter::repeat('x').take(10_000).collect::<String>();
    hll.insert(long.clone());
    let mut count = 1;
    // insert some duplicates and one new long string
    for _ in 0..500 {
        hll.insert(long.clone());
    }
    let another = long + "y";
    hll.insert(another);
    count += 1;
    assert_eq!(hll.calculate_cardinality(), count);
}

/// Test special-character strings
#[test]
fn test_special_characters() {
    let mut hll = HyperLogLog::<String>::new(10).unwrap();
    let items = vec!["😊", "©️", "💻🔧", "😊"];
    for &s in &items {
        hll.insert(s.to_string());
    }
    // distinct: 3
    assert_eq!(hll.calculate_cardinality(), 3);
}

/// Test prefix/suffix variations
#[test]
fn test_prefix_suffix_variations() {
    let mut hll = HyperLogLog::<String>::new(10).unwrap();
    let n: u64 = 100;
    for i in 0..n {
        hll.insert(format!("foo{}", i));
    }
    assert_eq!(hll.calculate_cardinality(), n);
}

/// Hash-collision simulation via wrapper
#[derive(Clone)]
struct CollidingString(String);
impl ToBytes for CollidingString {
    fn to_bytes(&self) -> Vec<u8> {
        Vec::new() // all collide
    }

    const TYPE_ID: &'static [u8] = b"CollidingString";
}

#[test]
fn test_string_hash_collisions() {
    let mut hll = HyperLogLog::<CollidingString>::new(8).unwrap();
    // insert 5 distinct wrapper-Strings, but they collide
    for i in 0..5 {
        hll.insert(CollidingString(format!("s{}", i)));
    }
    // should estimate ~1 distinct
    assert_eq!(hll.calculate_cardinality(), 1);
}

/// Low-precision stress test (p=4)
#[test]
fn test_low_precision_many_strings() {
    let p = 4;
    let mut hll = HyperLogLog::<String>::new(p).unwrap();
    let n: u64 = 1000;
    let tolerance = 1.04f64 / ((1u64 << p) as f64).sqrt();
    for i in 0..n {
        hll.insert(format!("test{}", i));
    }
    let est = hll.calculate_cardinality();
    let (lo, hi) = calculate_bounds(n, tolerance);
    assert!(est >= lo && est <= hi,
        "p={}, n={} -> estimate {} not in [{},{}]",
        p, n, est, lo, hi
    );
}

/// High-precision stress test (p=16)
#[test]
fn test_high_precision_some_strings() {
    let p = 16;
    let mut hll = HyperLogLog::<String>::new(p).unwrap();
    let n: u64 = 10_000;
    let tolerance = 1.04f64 / ((1u64 << p) as f64).sqrt();
    for i in 0..n {
        hll.insert(format!("str{}", i));
    }
    let est = hll.calculate_cardinality();
    let (lo, hi) = calculate_bounds(n, tolerance);
    assert!(est >= lo && est <= hi,
        "p={}, n={} -> estimate {} not in [{},{}]",
        p, n, est, lo, hi
    );
}
