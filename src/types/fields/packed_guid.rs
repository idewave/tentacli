use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::SerializeTupleStruct};

#[derive(Debug, Default, Clone)]
pub struct PackedGuid(pub u64);

impl PartialEq<u64> for PackedGuid {
    fn eq(&self, other: &u64) -> bool {
        let PackedGuid(guid) = self;
        guid == other
    }
}

impl PartialEq<PackedGuid> for u64 {
    fn eq(&self, other: &PackedGuid) -> bool {
        let PackedGuid(guid) = other;
        guid == self
    }
}

impl<'de> Deserialize<'de> for PackedGuid {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        todo!()
    }
}

impl Serialize for PackedGuid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        const FIELDS_AMOUNT: usize = 1;
        let mut state = serializer.serialize_tuple_struct("PackedGuid", FIELDS_AMOUNT)?;
        state.serialize_field(&self.0)?;
        state.end()
    }
}