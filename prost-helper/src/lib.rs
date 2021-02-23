//! A set of prost helper functions to make prost generated code easy to work with.
//!
//! If you use `prost-serde` to build your protobuf files, mostly you need this crate to provide
//! functions for `is_zero` and `deserialize_null_default`.
//!
//! You can also use the macros to convert protobuf messages to / try_from `Vec<u8>`.
//!
//! For example, if prost generated a data struct `Hello`, You can use the macros to generate
//! `From` and /`TryFrom` respectively, and then use them:
//!
//! ```ignore
//! use prost_helper::{prost_into_vec, vec_try_into_prost};
//! use prost::Message;
//!
//! #[derive(Clone, PartialEq, prost::Message)]
//! pub struct Hello {
//!     #[prost(string, tag = "1")]
//!     pub msg: String,
//! }
//!
//! prost_into_vec!(Hello, 32);
//! vec_try_into_prost!(Hello);
//!
//! fn send_hello(data: Vec<u8>) {
//!     unimplemented!();
//! }
//!
//! let hello = Hello::default();
//! send_hello(hello.into());
//!```
//!
use num_traits::Num;
use serde::{Deserialize, Deserializer};

pub mod macros;

#[cfg(feature = "json")]
/// Convert the prost message to JSON string for debugging purpose. Need serde_json support.
pub trait ToJson {
    fn to_json(&self) -> String;
}

/// customized skip_serializing_if function to skip 0 for numbers.
pub fn is_zero(v: impl Num) -> bool {
    v.is_zero()
}

/// customized deserialize function to use default for JSON null value.
pub fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

#[cfg(feature = "b64")]
pub mod b64 {
    use base64::{decode_config, display::Base64Display, URL_SAFE_NO_PAD};
    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // serializer.serialize_str(&base64::encode(bytes))

        // use a wrapper type with a Display implementation to avoid
        // allocating the String.
        //
        serializer.collect_str(&Base64Display::with_config(bytes, URL_SAFE_NO_PAD))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&str>::deserialize(deserializer)?;
        decode_config(s, URL_SAFE_NO_PAD).map_err(de::Error::custom)
    }
}

#[cfg(feature = "b64")]
pub mod b64vec {
    use base64::{decode_config, encode_config, URL_SAFE_NO_PAD};
    use serde::{de, ser::SerializeSeq, Deserializer, Serializer};

    pub fn serialize<S>(data: &[Vec<u8>], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(data.len()))?;
        for item in data {
            let e = encode_config(item, URL_SAFE_NO_PAD);
            seq.serialize_element(&e)?;
        }
        seq.end()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Vec<u8>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = Vec<Vec<u8>>;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a sequence of base64 ASCII text")
            }

            fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
            where
                S: de::SeqAccess<'de>,
            {
                let mut data: Vec<Vec<u8>> = Vec::with_capacity(seq.size_hint().unwrap_or(0));
                while let Some(v) = seq.next_element::<Vec<u8>>()? {
                    data.push(decode_config(v, URL_SAFE_NO_PAD).map_err(de::Error::custom)?);
                }
                Ok(data)
            }
        }

        deserializer.deserialize_seq(Visitor)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn is_zero_work_for_i32() {
        assert_eq!(is_zero(0i32), true);
    }

    #[test]
    fn is_zero_work_for_u64() {
        assert_eq!(is_zero(0u64), true);
    }

    #[cfg(all(feature = "b64", feature = "json"))]
    #[test]
    fn bytes_encoded_with_base64() {
        use prost::Message;
        use serde::{Deserialize, Serialize};

        #[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
        pub struct Hello {
            #[prost(string, tag = "1")]
            pub msg: String,
            #[serde(with = "b64")]
            #[prost(bytes, tag = "2")]
            pub value: ::prost::alloc::vec::Vec<u8>,
        }
        let hello = Hello {
            msg: "world".to_owned(),
            value: b"world".to_vec(),
        };
        let s = serde_json::to_string(&hello).unwrap();
        assert_eq!(s, r#"{"msg":"world","value":"d29ybGQ"}"#);
    }

    #[cfg(all(feature = "b64", feature = "json"))]
    #[test]
    fn bytes_in_enum_encoded_with_base64() {
        use prost::{Message, Oneof};
        use serde::{Deserialize, Serialize};

        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(default)]
        #[derive(Clone, PartialEq, Message)]
        pub struct ObjectId {
            /// the value of the id
            #[prost(oneof = "Data", tags = "2, 3")]
            pub data: ::core::option::Option<Data>,
        }
        #[derive(Clone, PartialEq, Serialize, Deserialize, Oneof)]
        pub enum Data {
            #[prost(string, tag = "2")]
            Result(String),
            #[serde(with = "b64")]
            #[prost(bytes, tag = "3")]
            Value(Vec<u8>),
        }
        let data = ObjectId {
            data: Some(Data::Value(b"world".to_vec())),
        };
        let s = serde_json::to_string(&data).unwrap();
        assert_eq!(s, r#"{"data":{"Value":"d29ybGQ"}}"#);
    }

    #[cfg(all(feature = "b64", feature = "json"))]
    #[test]
    fn vec_bytes_encoded_with_base64() {
        use prost::Message;
        use serde::{Deserialize, Serialize};

        #[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
        pub struct Hello {
            #[prost(string, tag = "1")]
            pub msg: String,
            #[serde(with = "b64vec")]
            #[prost(bytes = "vec", repeated, tag = "2")]
            pub value: Vec<Vec<u8>>,
        }
        let hello = Hello {
            msg: "world".to_owned(),
            value: vec![b"world".to_vec()],
        };
        let s = serde_json::to_string(&hello).unwrap();
        assert_eq!(s, r#"{"msg":"world","value":["d29ybGQ"]}"#);
    }
}
