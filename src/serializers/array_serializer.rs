use serde::{Serializer};

use crate::utils::encode_hex;

pub fn serialize_array<S>(item: &[u8], s: S) -> Result<S::Ok, S::Error> where S: Serializer {
    s.serialize_str(encode_hex(item).as_str())
}