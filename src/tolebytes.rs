pub trait ToLeBytes {
    fn as_le_bytes(&self) -> Vec<u8>;
}

macro_rules! impl_to_le_bytes {
    ($($t:ty),*) => {
        $(
            impl ToLeBytes for $t {
                fn as_le_bytes(&self) -> Vec<u8> {
                    self.to_le_bytes().to_vec()
                }
            }
        )*
    };
}

impl_to_le_bytes!(u32, u64, u128, i32, i64, i128);
