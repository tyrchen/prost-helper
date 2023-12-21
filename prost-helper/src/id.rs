use bytes::Bytes;
use serde::{de, ser, ser::SerializeSeq, Deserialize, Deserializer, Serializer};
use uuid7::Uuid;

pub fn serialize_id<S, T>(bytes: T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: AsRef<[u8]>,
{
    let bytes = bytes.as_ref();
    if bytes.is_empty() {
        return serializer.serialize_str("");
    }
    if bytes.len() != 16 {
        return Err(ser::Error::custom("invalid id length"));
    }
    let id = u128::from_be_bytes(bytes.try_into().map_err(ser::Error::custom)?);
    serializer.collect_str(&Uuid::from(id).to_string())
}

pub fn deserialize_id_vec<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = <&str>::deserialize(deserializer)?;
    if s.is_empty() {
        return Ok(vec![]);
    }
    let id: Uuid = s.parse().map_err(de::Error::custom)?;
    Ok(id.as_bytes().to_vec())
}

pub fn deserialize_id_bytes<'de, D>(deserializer: D) -> Result<Bytes, D::Error>
where
    D: Deserializer<'de>,
{
    deserialize_id_vec(deserializer).map(Bytes::from)
}

pub fn serialize_repeat_id<S, T>(data: &[T], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: AsRef<[u8]>,
{
    let mut seq = serializer.serialize_seq(Some(data.len()))?;
    for item in data {
        let bytes = item.as_ref();
        if bytes.is_empty() {
            seq.serialize_element("")?;
            continue;
        }
        if bytes.len() != 16 {
            return Err(ser::Error::custom("invalid id length"));
        }
        let id = u128::from_be_bytes(item.as_ref().try_into().map_err(ser::Error::custom)?);
        let e = Uuid::from(id).to_string();
        seq.serialize_element(&e)?;
    }
    seq.end()
}

pub fn deserialize_repeat_id_vec<'de, D>(deserializer: D) -> Result<Vec<Vec<u8>>, D::Error>
where
    D: Deserializer<'de>,
{
    struct Visitor;

    impl<'de> ::serde::de::Visitor<'de> for Visitor {
        type Value = Vec<Vec<u8>>;

        fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            write!(f, "a sequence of uuid7 ASCII text")
        }

        fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
        where
            S: de::SeqAccess<'de>,
        {
            let mut data: Vec<Vec<u8>> = Vec::with_capacity(seq.size_hint().unwrap_or(0));
            while let Some(s) = seq.next_element::<String>()? {
                if s.is_empty() {
                    data.push(vec![]);
                    continue;
                }
                let id: Uuid = s.parse().map_err(de::Error::custom)?;
                data.push(id.as_bytes().to_vec());
            }
            Ok(data)
        }
    }

    deserializer.deserialize_seq(Visitor)
}

pub fn deserialize_repeat_id_bytes<'de, D>(deserializer: D) -> Result<Vec<Bytes>, D::Error>
where
    D: Deserializer<'de>,
{
    struct Visitor;

    impl<'de> ::serde::de::Visitor<'de> for Visitor {
        type Value = Vec<Bytes>;

        fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            write!(f, "a sequence of uuid7 ASCII text")
        }

        fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
        where
            S: de::SeqAccess<'de>,
        {
            let mut data: Vec<Bytes> = Vec::with_capacity(seq.size_hint().unwrap_or(0));
            while let Some(s) = seq.next_element::<String>()? {
                if s.is_empty() {
                    data.push(Bytes::new());
                    continue;
                }
                let id: Uuid = s.parse().map_err(de::Error::custom)?;
                data.push(Bytes::from(id.as_bytes().to_vec()));
            }
            Ok(data)
        }
    }

    deserializer.deserialize_seq(Visitor)
}

#[cfg(all(feature = "id", feature = "json"))]
#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use prost::{Message, Oneof};
    use serde::{Deserialize, Serialize};

    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(default)]
    #[derive(Clone, PartialEq, Eq, ::prost::Message)]
    pub struct Hello {
        #[prost(string, tag = "1")]
        pub msg: String,
        #[serde(
            serialize_with = "serialize_id",
            deserialize_with = "deserialize_id_vec"
        )]
        #[prost(bytes, tag = "2")]
        pub value_vec: ::prost::alloc::vec::Vec<u8>,
        #[serde(
            serialize_with = "serialize_repeat_id",
            deserialize_with = "deserialize_repeat_id_vec"
        )]
        #[prost(bytes = "vec", repeated, tag = "3")]
        pub list_vec: Vec<Vec<u8>>,
        #[serde(
            serialize_with = "serialize_id",
            deserialize_with = "deserialize_id_bytes"
        )]
        #[prost(bytes, tag = "4")]
        pub value_bytes: ::prost::bytes::Bytes,
        #[serde(
            serialize_with = "serialize_repeat_id",
            deserialize_with = "deserialize_repeat_id_bytes"
        )]
        #[prost(bytes = "bytes", repeated, tag = "5")]
        pub list_bytes: ::prost::alloc::vec::Vec<::prost::bytes::Bytes>,
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(default)]
    #[derive(Clone, PartialEq, Eq, Message)]
    pub struct ObjectId {
        /// the value of the id
        #[prost(oneof = "Data", tags = "2, 3")]
        pub data: ::core::option::Option<Data>,
    }

    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Oneof)]
    pub enum Data {
        #[prost(string, tag = "2")]
        Result(String),
        #[serde(
            serialize_with = "serialize_id",
            deserialize_with = "deserialize_id_vec"
        )]
        #[prost(bytes, tag = "3")]
        Value(Vec<u8>),
    }

    #[test]
    fn vec_u8_encoded_with_uuid7() {
        let hello = Hello {
            msg: "world".to_owned(),
            value_vec: uuid_vec(),
            ..Default::default()
        };
        let s = serde_json::to_string(&hello).unwrap();
        assert_eq!(
            s,
            r#"{"msg":"world","value_vec":"018c8afb-43d7-7f73-be38-95ed30027670","list_vec":[],"value_bytes":"","list_bytes":[]}"#
        );
    }

    #[test]
    fn vec_u8_in_enum_encoded_with_uuid7() {
        let data = ObjectId {
            data: Some(Data::Value(uuid_vec())),
        };
        let s = serde_json::to_string(&data).unwrap();
        assert_eq!(
            s,
            r#"{"data":{"Value":"018c8afb-43d7-7f73-be38-95ed30027670"}}"#
        );
    }

    #[test]
    fn repeat_vec_u8_encoded_with_uuid7() {
        let hello = Hello {
            msg: "world".to_owned(),
            list_vec: vec![uuid_vec(), vec![]],
            ..Default::default()
        };
        let s = serde_json::to_string(&hello).unwrap();
        assert_eq!(
            s,
            r#"{"msg":"world","value_vec":"","list_vec":["018c8afb-43d7-7f73-be38-95ed30027670",""],"value_bytes":"","list_bytes":[]}"#
        );
    }

    #[test]
    fn bytes_encoded_with_uuid7() {
        let hello = Hello {
            msg: "world".to_owned(),
            value_bytes: uuid_vec().into(),
            ..Default::default()
        };
        let s = serde_json::to_string(&hello).unwrap();
        assert_eq!(
            s,
            r#"{"msg":"world","value_vec":"","list_vec":[],"value_bytes":"018c8afb-43d7-7f73-be38-95ed30027670","list_bytes":[]}"#
        );
    }

    #[test]
    fn repeat_bytes_encoded_with_uuid7() {
        let hello = Hello {
            msg: "world".to_owned(),
            list_bytes: vec![uuid_vec().into(), Bytes::new()],
            ..Default::default()
        };
        let s = serde_json::to_string(&hello).unwrap();
        assert_eq!(
            s,
            r#"{"msg":"world","value_vec":"","list_vec":[],"value_bytes":"","list_bytes":["018c8afb-43d7-7f73-be38-95ed30027670",""]}"#
        );
    }

    fn uuid() -> &'static str {
        "018c8afb-43d7-7f73-be38-95ed30027670"
    }

    fn uuid_vec() -> Vec<u8> {
        Uuid::from_str(uuid()).unwrap().as_bytes().to_vec()
    }
}
