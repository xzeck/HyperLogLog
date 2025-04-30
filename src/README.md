# HyperLogLog

This is a rust implementation of ([Google's HyperLogLog](https://static.googleusercontent.com/media/research.google.com/en//pubs/archive/40671.pdf)) algorithm

Usage:

```rust

let bucket_size = 10
let mut hll = HyperLogLog::new(bucket_size);

let n1 = 1;
let n2 = 42;
let n3 = 150_000;

hll.insert(n1);
hll.insert(n2);
hll.insert(n3);

let cardinality = hll.calculate_cardinality();
```

Right now it only works on i64 types.
