use crate::tolebytes::ToLeBytes;

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}


impl<T: ToLeBytes> ToBytes for T {
    fn to_bytes(&self) -> Vec<u8> {
        self.as_le_bytes()
    }
}

impl ToBytes for &str {
    fn to_bytes(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl ToBytes for String {
    fn to_bytes(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}
