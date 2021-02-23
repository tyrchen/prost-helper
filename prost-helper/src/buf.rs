use base64::{decode_config, display::Base64Display, encode_config, URL_SAFE_NO_PAD};
use bytes::Bytes;
use serde::{de, ser::SerializeSeq, Deserialize, Deserializer, Serializer};

pub fn serialize_buf<S, T>(bytes: T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: AsRef<[u8]>,
{
    // serializer.serialize_str(&base64::encode(bytes))

    // use a wrapper type with a Display implementation to avoid
    // allocating the String.
    //
    serializer.collect_str(&Base64Display::with_config(bytes.as_ref(), URL_SAFE_NO_PAD))
}

pub fn deserialize_buf_vec<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = <&str>::deserialize(deserializer)?;
    decode_config(s, URL_SAFE_NO_PAD).map_err(de::Error::custom)
}

pub fn deserialize_buf_bytes<'de, D>(deserializer: D) -> Result<Bytes, D::Error>
where
    D: Deserializer<'de>,
{
    let s = <&str>::deserialize(deserializer)?;
    let r = decode_config(s, URL_SAFE_NO_PAD).map_err(de::Error::custom)?;
    Ok(Bytes::from(r))
}

pub fn serialize_repeat_buf<S, T>(data: &[T], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: AsRef<[u8]>,
{
    let mut seq = serializer.serialize_seq(Some(data.len()))?;
    for item in data {
        let e = encode_config(item, URL_SAFE_NO_PAD);
        seq.serialize_element(&e)?;
    }
    seq.end()
}

pub fn deserialize_repeat_buf_vec<'de, D>(deserializer: D) -> Result<Vec<Vec<u8>>, D::Error>
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

pub fn deserialize_repeat_buf_bytes<'de, D>(deserializer: D) -> Result<Vec<Bytes>, D::Error>
where
    D: Deserializer<'de>,
{
    struct Visitor;

    impl<'de> ::serde::de::Visitor<'de> for Visitor {
        type Value = Vec<Bytes>;

        fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            write!(f, "a sequence of base64 ASCII text")
        }

        fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
        where
            S: de::SeqAccess<'de>,
        {
            let mut data: Vec<Bytes> = Vec::with_capacity(seq.size_hint().unwrap_or(0));
            while let Some(v) = seq.next_element::<Vec<u8>>()? {
                let r = decode_config(v, URL_SAFE_NO_PAD).map_err(de::Error::custom)?;
                data.push(Bytes::from(r));
            }
            Ok(data)
        }
    }

    deserializer.deserialize_seq(Visitor)
}

#[cfg(all(feature = "b64", feature = "json"))]
#[cfg(test)]
mod tests {
    use super::*;
    use prost::{Message, Oneof};
    use serde::{Deserialize, Serialize};

    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(default)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Hello {
        #[prost(string, tag = "1")]
        pub msg: String,
        #[serde(
            serialize_with = "serialize_buf",
            deserialize_with = "deserialize_buf_vec"
        )]
        #[prost(bytes, tag = "2")]
        pub value_vec: ::prost::alloc::vec::Vec<u8>,
        #[serde(
            serialize_with = "serialize_repeat_buf",
            deserialize_with = "deserialize_repeat_buf_vec"
        )]
        #[prost(bytes = "vec", repeated, tag = "3")]
        pub list_vec: Vec<Vec<u8>>,
        #[serde(
            serialize_with = "serialize_buf",
            deserialize_with = "deserialize_buf_bytes"
        )]
        #[prost(bytes, tag = "4")]
        pub value_bytes: ::prost::bytes::Bytes,
        #[serde(
            serialize_with = "serialize_repeat_buf",
            deserialize_with = "deserialize_repeat_buf_bytes"
        )]
        #[prost(bytes = "bytes", repeated, tag = "5")]
        pub list_bytes: ::prost::alloc::vec::Vec<::prost::bytes::Bytes>,
    }

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
        #[serde(
            serialize_with = "serialize_buf",
            deserialize_with = "deserialize_buf_vec"
        )]
        #[prost(bytes, tag = "3")]
        Value(Vec<u8>),
    }

    #[test]
    fn vec_u8_encoded_with_base64() {
        let hello = Hello {
            msg: "world".to_owned(),
            value_vec: b"world".to_vec(),
            ..Default::default()
        };
        let s = serde_json::to_string(&hello).unwrap();
        assert_eq!(
            s,
            r#"{"msg":"world","value_vec":"d29ybGQ","list_vec":[],"value_bytes":"","list_bytes":[]}"#
        );
    }

    #[test]
    fn vec_u8_in_enum_encoded_with_base64() {
        let data = ObjectId {
            data: Some(Data::Value(b"world".to_vec())),
        };
        let s = serde_json::to_string(&data).unwrap();
        assert_eq!(s, r#"{"data":{"Value":"d29ybGQ"}}"#);
    }

    #[test]
    fn repeat_vec_u8_encoded_with_base64() {
        let hello = Hello {
            msg: "world".to_owned(),
            list_vec: vec![b"world".to_vec()],
            ..Default::default()
        };
        let s = serde_json::to_string(&hello).unwrap();
        assert_eq!(
            s,
            r#"{"msg":"world","value_vec":"","list_vec":["d29ybGQ"],"value_bytes":"","list_bytes":[]}"#
        );
    }

    #[test]
    fn bytes_encoded_with_base64() {
        let hello = Hello {
            msg: "world".to_owned(),
            value_bytes: Bytes::from("world"),
            ..Default::default()
        };
        let s = serde_json::to_string(&hello).unwrap();
        assert_eq!(
            s,
            r#"{"msg":"world","value_vec":"","list_vec":[],"value_bytes":"d29ybGQ","list_bytes":[]}"#
        );
    }

    #[test]
    fn repeat_bytes_encoded_with_base64() {
        let hello = Hello {
            msg: "world".to_owned(),
            list_bytes: vec![Bytes::from("world")],
            ..Default::default()
        };
        let s = serde_json::to_string(&hello).unwrap();
        assert_eq!(
            s,
            r#"{"msg":"world","value_vec":"","list_vec":[],"value_bytes":"","list_bytes":["d29ybGQ"]}"#
        );
    }
}
