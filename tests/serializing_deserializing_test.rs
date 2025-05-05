// tests/serializing_deserializing_test.rs

use std::hash::{BuildHasherDefault, DefaultHasher};
use xxhash_rust::xxh3::Xxh3DefaultBuilder;
use serde_json;

use hyperloglog::HyperLogLog;

#[test]
fn test_serialize_deserialize_default_hll() {
    let mut hll_def: HyperLogLog<i64, BuildHasherDefault<DefaultHasher>> =
        HyperLogLog::new(10);
    hll_def.insert(1);
    hll_def.insert(2);

    let json = serde_json::to_string(&hll_def).unwrap();
    // round-trip with the SAME hasher should succeed
    let hll_def2: HyperLogLog<i64, BuildHasherDefault<DefaultHasher>> =
        serde_json::from_str(&json).unwrap();
    assert_eq!(hll_def2.calculate_cardinality(), 2);
}

#[test]
fn test_deserialize_with_xxh3_should_fail() {
    let mut hll_def: HyperLogLog<i64> =
        HyperLogLog::new(10);
    hll_def.insert(1);
    hll_def.insert(2);

    let json = serde_json::to_string(&hll_def).unwrap();
    // but if we try to load into an XXH3-hasher HLL, it must fail
    let result: Result<HyperLogLog<i64, Xxh3DefaultBuilder>, _> =
        serde_json::from_str(&json);

    assert!(
        result.is_err(),
        "expected fingerprint-mismatch error when loading default‐dump with XXH3 hasher"
    );
}


#[test]
fn test_stable_json_output() {
    let mut hll = HyperLogLog::<i64>::new(10);
    hll.insert(42);
    let j1 = serde_json::to_string(&hll).unwrap();
    let j2 = serde_json::to_string(&hll).unwrap();
    assert_eq!(j1, j2, "JSON output should be deterministic/stable");
}


// Round-trip identity: serialize → deserialize → serialize yields the same JSON again
#[test]
fn test_roundtrip_json_identity() {
    let mut hll = HyperLogLog::<i64, BuildHasherDefault<DefaultHasher>>::new(10);
    for i in 0..100 { hll.insert(i); }
    let original = serde_json::to_string(&hll).unwrap();
    let recovered: HyperLogLog<i64, BuildHasherDefault<DefaultHasher>> =
        serde_json::from_str(&original).unwrap();
    let round_trip = serde_json::to_string(&recovered).unwrap();

    assert_eq!(original, round_trip, "Serialize→deserialize→serialize should preserve exact JSON");
}

// Missing-field error: drop the "buckets" field and expect a deserialization error
#[test]
fn test_deserialize_missing_field_errors() {
    // build a valid JSON, then remove the "buckets" key
    let mut hll = HyperLogLog::<i64>::new(10);
    let mut json: serde_json::Value = serde_json::to_value(&hll).unwrap();
    let obj = json.as_object_mut().unwrap();
    obj.remove("buckets");
    let broken = serde_json::to_string(&json).unwrap();

    let res: Result<HyperLogLog<i64, BuildHasherDefault<DefaultHasher>>, _> =
        serde_json::from_str(&broken);
    assert!(res.is_err(), "Deserializing JSON with missing buckets should fail");
}

// XXH3 round-trip: serialize with XXH3, deserialize with the same builder, and be successful
#[test]
fn test_xxh3_roundtrip_succeeds() {
    let mut hll = HyperLogLog::<i64, Xxh3DefaultBuilder>::with_hasher(10, Xxh3DefaultBuilder::new());
    for i in 0..200 { hll.insert(i); }
    let json = serde_json::to_string(&hll).unwrap();

    // this time we deserialize into the same hasher type
    let hll2: HyperLogLog<i64, Xxh3DefaultBuilder> =
        serde_json::from_str(&json).unwrap();
    assert_eq!(
        hll2.calculate_cardinality(),
        hll.calculate_cardinality(),
        "XXH3→serde→XXH3 roundtrip should work and preserve cardinality"
    );
}

#[test]
fn test_error_on_deserializing_mismatched_element_type() {
    let p = 4;
    let mut hll: HyperLogLog<i64> = HyperLogLog::new(p);

    let json = serde_json::to_string(&hll).unwrap();

    let res: Result<HyperLogLog<f64>, _> = serde_json::from_str(&json);

    assert!(res.is_err(), "Deserializing different datatype, should fail");

}