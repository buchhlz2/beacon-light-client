#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unreachable_code)]
#![allow(unused_must_use)]

pub use hex;
pub use crate::light_client_types::{Hash256};

pub fn hex_string_to_bytes(hex: String) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    hex::decode_to_slice(hex.trim_start_matches("0x"), &mut bytes).unwrap();
    return bytes
}