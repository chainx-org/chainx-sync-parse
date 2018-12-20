use std::fmt;

use serde::de::{self, Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};

#[derive(Debug, Clone)]
pub struct Bytes(Vec<u8>);

impl From<Vec<u8>> for Bytes {
    fn from(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }
}

impl Into<Vec<u8>> for Bytes {
    fn into(self) -> Vec<u8> {
        self.0
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

/// Expected length of bytes vector.
#[derive(Debug, PartialEq, Eq)]
pub enum ExpectedLen {
    /// Any length in bytes.
    Any,
    /// Exact length in bytes.
    #[allow(unused)]
    Exact(usize),
    /// A bytes length between (min; max].
    #[allow(unused)]
    Between(usize, usize),
}

impl fmt::Display for ExpectedLen {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExpectedLen::Any => write!(fmt, "even length"),
            ExpectedLen::Exact(v) => write!(fmt, "length of {}", v * 2),
            ExpectedLen::Between(min, max) => {
                write!(fmt, "length between ({}; {}]", min * 2, max * 2)
            }
        }
    }
}

impl<'de> Deserialize<'de> for Bytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        bytes_deserialize(deserializer).map(|x| x.into())
    }
}

fn bytes_deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    bytes_deserialize_check_len(deserializer, ExpectedLen::Any)
}

fn bytes_deserialize_check_len<'de, D>(
    deserializer: D,
    len: ExpectedLen,
) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    struct Visitor {
        len: ExpectedLen,
    }

    impl<'a> de::Visitor<'a> for Visitor {
        type Value = Vec<u8>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a 0x-prefixed hex string with {}", self.len)
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if v.len() < 2 || &v[0..2] != "0x" {
                return Err(E::custom("prefix is missing"));
            }

            let is_len_valid = match self.len {
                ExpectedLen::Any => v.len() % 2 == 0,
                ExpectedLen::Exact(len) => v.len() == 2 * len + 2,
                ExpectedLen::Between(min, max) => v.len() <= 2 * max + 2 && v.len() > 2 * min + 2,
            };

            if !is_len_valid {
                return Err(E::invalid_length(v.len() - 2, &self));
            }

            let bytes = match self.len {
                ExpectedLen::Between(..) if v.len() % 2 != 0 => {
                    hex::decode(&*format!("0{}", &v[2..]))
                }
                _ => hex::decode(&v[2..]),
            };

            bytes.map_err(|e| E::custom(format!("invalid hex value: {:?}", e)))
        }
    }

    deserializer.deserialize_str(Visitor { len })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_convert() {
        let bytes = Bytes::from(vec![0x01, 0x03, 0x05, 0x07]);
        let vec: Vec<u8> = bytes.into();
        assert_eq!(vec, vec![0x01, 0x03, 0x05, 0x07]);
    }

    #[derive(Serialize, Deserialize, Debug)]
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
        let vec: Vec<u8> = de.bytes.into();
        assert_eq!(vec, vec![0x01, 0x03, 0x05, 0x07]);
    }
}
