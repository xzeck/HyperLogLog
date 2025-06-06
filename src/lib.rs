pub mod tobytes;
mod error;
use error::HyperLogLogError;
pub use tobytes::ToBytes;

use std::{hash::{BuildHasher, BuildHasherDefault, DefaultHasher, Hasher}, marker::PhantomData};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Error as DeError;

/// HyperLogLog is a probabilistic data structure for estimating cardinality.
/// This implementation uses the HyperLogLog algorithm to estimate the
/// number of distinct elements in a large stream of data, using `p` bits (which determines the number of buckets).
#[derive(Clone)]
pub struct HyperLogLog<T: ToBytes, S = BuildHasherDefault<DefaultHasher>> {
    p: u32, // number of bits
    m: usize, // size of buckets
    buckets: Vec<u8>, // vectors to store the bucket
    hasher_builder: S, // hasher to use
    // Marker to associate the generic type `T` without storing a value of it.
    // Ensures the type system correctly tracks ownership and variance of `T`.
    _marker: PhantomData<T>,
}

/// Struct for serializing HyperLogLog
#[derive(Serialize, Deserialize)]
struct HyperLogLogSerializable {
    p: u32, // p bits
    m: usize, // size of the buckets
    buckets: Vec<u8>, // vector to store the buckets
    fingerprint: u64 // finger value to make sure that when value is saved and loaded it has the same configuration
}

// implementing serialize for HyperLogLog only if T and S meet the criteria of T being ToBytes and S being iether BuildHasher or Default
impl<T: ToBytes, S: BuildHasher + Default> Serialize for HyperLogLog<T, S> {
    fn serialize<Ser>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error>
    where
        Ser: Serializer,
    {
        // generating a fingerprint
        // This is so that if the state is saved and then reloaded we can ensure the same hashing function is used to maintain consistence
        let mut hasher = self.hasher_builder.build_hasher();

        hasher.write(b"__hyperloglog_fingerprint__");
        hasher.write(T::TYPE_ID);
        let fingerprint = hasher.finish();

        // generating serializable structure
        let data = HyperLogLogSerializable {
            p: self.p,
            m: self.m,
            buckets: self.buckets.clone(),
            fingerprint: fingerprint
        };

        
        data.serialize(serializer)
    }
}

impl<'de, T: ToBytes, S: BuildHasher + Default> Deserialize<'de> for HyperLogLog<T, S> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = HyperLogLogSerializable::deserialize(deserializer)?;

        // Recompute fingerprint using S::default()
        let mut hasher = S::default().build_hasher();
        hasher.write(b"__hyperloglog_fingerprint__");
        hasher.write(T::TYPE_ID);
        let expected_fingerprint = hasher.finish();

        if expected_fingerprint != data.fingerprint {
            return Err(D::Error::custom("Hasher mismatch: incompatible hasher or datatype used during deserialization"));
        }

        Ok(Self {
            p: data.p,
            m: data.m,
            buckets: data.buckets,
            hasher_builder: S::default(),
            _marker: PhantomData,
        })
    }
}


impl<T: ToBytes> HyperLogLog<T, BuildHasherDefault<DefaultHasher>> {
    /// Default constructor using the standard RandomState hasher.
    pub fn new(p: u32) -> Result<Self, HyperLogLogError> {
        Self::with_hasher(p, Default::default())
    }
}

impl<T: ToBytes, S: BuildHasher + Default + Clone> HyperLogLog<T, S> {
    
    /// Creates a new `HyperLogLog` with `p` bits.
    /// Panics if `p < 4` or if `p` is too large to shift safely.
    pub fn with_hasher(p: u32, hasher_builder: S) -> Result<Self, HyperLogLogError> {
        
        if p < 4 {
            return Err(HyperLogLogError::PrecisionBelowThreshold);
        }
        // Compute m = 2^p, panic if overflow
        let m = match 1usize.checked_shl(p) {
            Some(m) => m,
            None => return Err(HyperLogLogError::PrecisionTooLarge),
        };
        // Initialize buckets to zero
        // u8 because the maximum number of leading zeros we can have
        // is if the hash equals to 0, so 2^8, 256 (technicall xxh3_64 generates a 64 bit hash)
        // which means max leading zeros is 64 but the smallest data type rust handles is u8
        let buckets = vec![0u8; m];


        Ok(HyperLogLog { p, m, buckets, hasher_builder, _marker: PhantomData })
    }

    /// Generates hashes.
    fn hash_input(&mut self, item: T) -> u64 {
        let mut hasher = self.hasher_builder.build_hasher();
        hasher.write(&item.to_bytes());
        hasher.finish()
    }

    /// Inserts an element into the HyperLogLog structure.
    pub fn insert(&mut self, item: T) {
        let hash = self.hash_input(item);
        // Bucket index: top `p` bits
        let idx = (hash >> (64 - self.p)) as usize;
        // Remaining bits for leading zero count
        let w = hash << self.p;
        // Count leading zeros (cap at 64), then +1
        let leading = (w.leading_zeros() + 1).min(64) as u8;
        // Update the bucket with the max leading count
        self.buckets[idx] = self.buckets[idx].max(leading);
    }

    /// Calculates the cardinality estimate.
    pub fn calculate_cardinality(&self) -> u64 {
        let m = self.m as f64;
        // Harmonic mean of 2^{-bucket_value}
        let sum: f64 = self.buckets.iter()
            .map(|&v| 2f64.powi(-(v as i32)))
            .sum();
        // Count zero buckets
        let zero = self.buckets.iter().filter(|&&v| v == 0).count() as f64;

        // Empty set
        if zero == m {
            return 0;
        }

        let z = 1.0 / sum;
        // Empirical alpha factor
        let alpha = match self.m {
            16 => 0.673,
            32 => 0.697,
            64 => 0.709,
            mm if mm >= 128 => 0.7213 / (1.0 + 1.079 / (mm as f64)),
            _ => unreachable!("Bucket count m={} is unsupported", self.m),
        };
        let mut estimate = alpha * m * m * z;

        // Small-range (linear counting) correction: m * ln(m / V)
        if zero > 0.0 {
            let linear = m * (m / zero).ln();
            if linear <= 2.5 * m {
                return linear.round() as u64;
            }
        }

        // overflow correction
        if estimate > (1.0/30.0) * (2.0f64.powi(32)) {
            estimate = -2.0f64.powi(32) * (1.0 - (estimate / 2.0f64.powi(32))).ln();
        }
        
        estimate.round() as u64
    }

    pub fn merge(&mut self, other: &Self) -> Result<(), HyperLogLogError>{

        // Checking if both the p values are same or not
        if self.p != other.p {
            return Err(HyperLogLogError::MisMatchedPrecision(self.p, other.p));
        }

        // iterating over the bucket and getting the max value
        for (i, &bucket) in other.buckets.iter().enumerate() {
            self.buckets[i] = self.buckets[i].max(bucket);
        }

        return Ok(())
    }

    /// Resets the bucket for reuse, sets value of the buckets to 0, doesn't affect p and m
    pub fn reset(&mut self) {
        self.buckets.fill(0);
    }

    /// Returns a copy of the current state of the bucket.
    ///
    /// Returns A copy of `Vec<u8>` representing the current bucket state.
    ///
    /// 
    pub fn get_buckets(&self) -> Vec<u8> {
        self.buckets.clone()
    }

    pub fn get_p(&self) -> u32 {
        self.p.clone()
    }

    pub fn get_m(&self) -> usize {
        self.m.clone()
    }
}
