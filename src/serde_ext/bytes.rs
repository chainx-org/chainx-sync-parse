use std::fmt;

use parity_codec_derive::{Decode, Encode};
use serde::de::{Deserialize, Deserializer, Error, Visitor};
use serde::ser::{Serialize, Serializer};

#[derive(PartialEq, Eq, Clone, Default, Debug, Encode, Decode)]
pub struct Bytes(pub Vec<u8>);

impl Bytes {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<T: Into<Vec<u8>>> From<T> for Bytes {
    fn from(data: T) -> Bytes {
        Bytes(data.into())
    }
}

impl Serialize for Bytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let hex = hex::encode(&self.0);
        serializer.serialize_str(&format!("0x{}", hex))
    }
}

impl<'de> Deserialize<'de> for Bytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_identifier(BytesVisitor)
    }
}

struct BytesVisitor;

impl<'de> Visitor<'de> for BytesVisitor {
    type Value = Bytes;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a 0x-prefixed hex-encoded vector of bytes")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if v.len() >= 2 && &v[0..2] == "0x" && v.len() & 1 == 0 {
            Ok(Bytes(
                hex::decode(&v[2..]).map_err(|_| Error::custom("invalid hex"))?,
            ))
        } else {
            Err(Error::custom("invalid format"))
        }
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_str(v.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use serde_derive::{Deserialize, Serialize};
    use serde_json::json;

    use super::*;

    #[test]
    fn test_bytes_from() {
        let bytes = Bytes::from(vec![0x01, 0x03, 0x05, 0x07]);
        assert_eq!(bytes.0, vec![0x01, 0x03, 0x05, 0x07]);
    }

    #[derive(Serialize, Deserialize)]
    struct TestBytes {
        pub bytes: Bytes,
    }

    #[test]
    fn test_bytes_serialize() {
        let bytes = Bytes::from(vec![0x01, 0x03, 0x05, 0x07]);
        let ser = json!({ "bytes": bytes });
        assert_eq!(r#"{"bytes":"0x01030507"}"#, format!("{}", ser));
    }

    #[test]
    fn test_bytes_deserialize() {
        let bytes = r#"{"bytes": "0x01030507"}"#;
        let de = serde_json::from_str::<TestBytes>(bytes).unwrap();
        assert_eq!(de.bytes.0, vec![0x01, 0x03, 0x05, 0x07]);
    }
}
