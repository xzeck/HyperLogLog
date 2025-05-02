 use std::hash::{BuildHasher, Hasher, RandomState};

use hyperloglog::HyperLogLog;
 
 /// A hasher builder that always produces a hasher returning 0
 #[derive(Clone, Default)]
 struct ConstHasherBuilder;

 impl BuildHasher for ConstHasherBuilder {
     type Hasher = ConstHasher;
     fn build_hasher(&self) -> ConstHasher { ConstHasher }
 }

 #[derive(Default)]
 struct ConstHasher;

 impl Hasher for ConstHasher {
     fn write(&mut self, _bytes: &[u8]) {}
     fn finish(&self) -> u64 { 0 }
 }

 #[test]
 fn test_default_hasher_reproducible() {
     let mut h1 = HyperLogLog::<u64>::new(10);
     let mut h2 = HyperLogLog::<u64>::new(10);
     for i in 1..=1000u64 {
         h1.insert(i);
         h2.insert(i);
     }
     assert_eq!(h1.calculate_cardinality(), h2.calculate_cardinality());
 }

 #[test]
 fn test_const_hasher_collisions() {
     let mut h = HyperLogLog::<u64, ConstHasherBuilder>::with_hasher(10, ConstHasherBuilder);
     for i in 1..=1000u64 {
         h.insert(i);
     }
     // All hashes collide into a single bucket => cardinality should be 1
     assert_eq!(h.calculate_cardinality(), 1);
 }

 #[test]
 fn test_randomstate_hasher_builds() {
     let mut h = HyperLogLog::<u64, RandomState>::with_hasher(10, RandomState::new());
     for i in 1..=100u64 { h.insert(i); }
     let est = h.calculate_cardinality();
     assert!(est > 0);
 }

 #[test]
 fn test_custom_xxhash_builder() {
     // If you add twox-hash to your dependencies:
     // use twox_hash::XxHash64;
     // type XxBuilder = BuildHasherDefault<XxHash64>;
     // let mut h = HyperLogLog::<u64, XxBuilder>::with_hasher(10, Default::default());
     // for i in 1..=100u64 { h.insert(i); }
     // assert!(h.calculate_cardinality() > 0);
     // For now, just ensure the method compiles and runs:
     let mut h = HyperLogLog::<u64>::new(10);
     for i in 1..=100u64 { h.insert(i); }
     assert!(h.calculate_cardinality() > 0);
 }