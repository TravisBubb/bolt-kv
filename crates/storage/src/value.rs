use bincode::{
    config::{self},
    error::{DecodeError, EncodeError},
    Decode, Encode,
};

/// An enum to represent the different value types supported by the KV store
#[derive(Clone, Debug, PartialEq)]
pub struct Value {
    data: Vec<u8>,
}

impl Value {
    /// Serialize any serializable object into a `Value`
    pub fn from_serializable<T: Encode>(obj: &T) -> Result<Self, EncodeError> {
        Ok(Value {
            data: bincode::encode_to_vec(obj, config::standard())?,
        })
    }

    /// Deserialize the Object variant back into a specified type
    pub fn to_deserializable<T: Decode<()>>(&self) -> Result<T, Box<DecodeError>> {
        let (decoded, _) = bincode::decode_from_slice(&self.data, config::standard())?;
        Ok(decoded)
    }
}
