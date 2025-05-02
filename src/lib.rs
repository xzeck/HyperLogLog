pub mod tobytes;

pub use tobytes::ToBytes;

use std::marker::PhantomData;
use xxhash_rust::xxh3::xxh3_64;

/// HyperLogLog is a probabilistic data structure for estimating cardinality.
/// This implementation uses the HyperLogLog algorithm to estimate the
/// number of distinct elements in a large stream of data, using `p` bits (which determines the number of buckets).
pub struct HyperLogLog<T: ToBytes> {
    p: u32,
    m: usize,
    buckets: Vec<u8>,
    _marker: PhantomData<T>,
}

impl<T: ToBytes> HyperLogLog<T> {
    /// Creates a new `HyperLogLog` with `p` bits.
    /// Panics if `p < 4` or if `p` is too large to shift safely.
    pub fn new(p: u32) -> Self {
        assert!(p >= 4, "Precision p must be at least 4");
        // Compute m = 2^p, panic if overflow
        let m = 1usize.checked_shl(p).expect("Precision p is too large");
        // Initialize buckets to zero
        // u8 because the maximum number of leading zeros we can have
        // is if the hash equals to 0, so 2^8, 256 (technicall xxh3_64 generates a 64 bit hash)
        // which means max leading zeros is 64 but the smallest data type rust handles is u8
        let buckets = vec![0u8; m];
        HyperLogLog { p, m, buckets, _marker: PhantomData }
    }

    /// Efficient hash function using XxHash64 for faster hashing.
    fn hash_input(item: T) -> u64 {
        xxh3_64(&item.to_bytes())
    }

    /// Inserts an element into the HyperLogLog structure.
    pub fn insert(&mut self, item: T) {
        let hash = Self::hash_input(item);
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
        let estimate = alpha * m * m * z;

        // Small-range (linear counting) correction: m * ln(m / V)
        if zero > 0.0 {
            let linear = m * (m / zero).ln();
            if linear <= 2.5 * m {
                return linear.round() as u64;
            }
        }

        estimate.round() as u64
    }
}
