pub mod de;
pub mod error;
pub mod ser;

mod buf;

pub use error::{Error, Result};

fn sign_extend_le(bytes: &[u8]) -> i128 {
    if bytes.len() > 16 || bytes.is_empty() {
        panic!("invalid bytes length {}", bytes.len());
    }
    if bytes.len() == 16 {
        return i128::from_le_bytes(bytes.try_into().unwrap());
    }

    let sign = bytes.last().unwrap() & 0x80 != 0; // true if negative
    let sign_byte = if sign { 0xffu8 } else { 0x00u8 };

    let mut vec: Vec<u8> = bytes.to_vec();
    vec.extend(vec![sign_byte; 16 - bytes.len()]);

    i128::from_le_bytes(vec.try_into().unwrap())
}

#[test]
fn sign_extend_test() {
    assert_eq!(sign_extend_le((5u8).to_le_bytes().as_slice()), 5);
    assert_eq!(sign_extend_le((-25i8).to_le_bytes().as_slice()), -25);
}
