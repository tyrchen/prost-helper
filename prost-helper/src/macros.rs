/// Generate `From` and `Into` for your prost message.
///
/// When you use it, plese include `use prost::Message;` in your code.
/// Note both `$type` and `&$type` will be generated.
#[macro_export]
macro_rules! prost_into_vec {
    ($(($type:ty, $cap:expr)),*) => {
        $(impl std::convert::From<$type> for Vec<u8> {
            fn from(msg: $type) -> Self {
                let mut buf = bytes::BytesMut::with_capacity($cap);
                msg.encode(&mut buf).unwrap();
                buf.to_vec()
            }
        }
        impl std::convert::From<&$type> for Vec<u8> {
            fn from(msg: &$type) -> Self {
                let mut buf = bytes::BytesMut::with_capacity($cap);
                msg.encode(&mut buf).unwrap();
                buf.to_vec()
            }
        })*
    };
}

/// Generate `TryFrom` and `TryInto` for your prost message from `Vec<u8>`.
///
/// When you use it, plese include `use prost::Message;` in your code.
/// Please remember to `use std::convert::TryInto;` when you want to use generated functions.
#[macro_export]
macro_rules! vec_try_into_prost {
    ($($type:ty),*) => {
        $(impl std::convert::TryFrom<Vec<u8>> for $type {
            type Error = prost::DecodeError;
            fn try_from(buf: Vec<u8>) -> Result<Self, Self::Error> {
                let msg: $type = Message::decode(&buf[..])?;
                Ok(msg)
            }
        }

        impl std::convert::TryFrom<&[u8]> for $type {
            type Error = prost::DecodeError;
            fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
                let msg: $type = Message::decode(buf)?;
                Ok(msg)
            }
        })*
    };
}

#[cfg(feature = "json")]
#[macro_export]
macro_rules! prost_to_json {

    ($($type:ty),*) => {
        $(impl crate::ToJson for $type {
            fn to_json(&self) -> String {
                serde_json::to_string(self).unwrap()
            }
        })*
    };
}

#[cfg(test)]
mod tests {
    use prost::Message;
    use serde::{Deserialize, Serialize};
    use std::convert::TryInto;

    #[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
    pub struct Hello {
        #[prost(string, tag = "1")]
        pub msg: String,
        #[prost(uint64, tag = "2")]
        pub value: u64,
    }

    #[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
    struct World {
        #[prost(string, tag = "1")]
        pub msg: String,
    }

    #[test]
    fn test_prost_try_into_vec() {
        prost_into_vec!((Hello, 32), (World, 256));
        vec_try_into_prost!(Hello, World);
        let hello = Hello::default();
        let data: Vec<u8> = hello.clone().into();
        let data_ref = &data[..];

        let hello_result1: Result<Hello, prost::DecodeError> = data_ref.try_into();
        let hello_result2: Result<Hello, prost::DecodeError> = data.try_into();
        assert_eq!(hello_result1.is_ok(), true);
        assert_eq!(hello_result1.unwrap(), hello);
        assert_eq!(hello_result2.is_ok(), true);
        assert_eq!(hello_result2.unwrap(), hello);
    }

    #[cfg(feature = "json")]
    #[test]
    fn test_prost_to_json() {
        use crate::ToJson;

        prost_to_json!(Hello, World);
        let hello = Hello::default();
        let result = hello.to_json();
        assert_eq!(result, r#"{"msg":"","value":0}"#);
    }
}
