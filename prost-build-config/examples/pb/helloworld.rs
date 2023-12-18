#[derive(serde::Serialize, serde::Deserialize, validator::Validate)]
#[serde(default)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hello {
    #[prost(string, tag = "1")]
    #[serde(skip_serializing_if = "String::is_empty", default)]
    #[validate(email)]
    pub msg: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[serde(deserialize_with = "prost_helper::deserialize_null_default")]
    pub field_may_be_null: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub field_skip_zero: u64,
    #[prost(string, tag = "4")]
    pub filed_skip: ::prost::alloc::string::String,
    #[prost(bytes = "bytes", tag = "5")]
    #[serde(
        serialize_with = "prost_helper::serialize_buf",
        deserialize_with = "prost_helper::deserialize_buf_bytes"
    )]
    pub data1: ::prost::bytes::Bytes,
    #[prost(bytes = "bytes", tag = "6")]
    #[serde(
        serialize_with = "prost_helper::serialize_buf",
        deserialize_with = "prost_helper::deserialize_buf_bytes"
    )]
    pub data2: ::prost::bytes::Bytes,
    #[prost(btree_map = "string, bytes", tag = "7")]
    #[serde(
        serialize_with = "prost_helper::serialize_buf",
        deserialize_with = "prost_helper::deserialize_buf_bytes"
    )]
    pub map: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::bytes::Bytes,
    >,
    #[prost(bytes = "bytes", repeated, tag = "8")]
    #[serde(
        serialize_with = "prost_helper::serialize_repeat_buf",
        deserialize_with = "prost_helper::deserialize_repeat_buf_bytes"
    )]
    pub list_data: ::prost::alloc::vec::Vec<::prost::bytes::Bytes>,
    #[prost(message, optional, tag = "9")]
    #[validate(required)]
    pub world: ::core::option::Option<World>,
}
#[derive(serde::Serialize, serde::Deserialize, validator::Validate)]
#[serde(default)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct World {
    #[prost(uint32, tag = "1")]
    pub world: u32,
}
#[derive(serde::Serialize, serde::Deserialize, validator::Validate)]
#[serde(rename_all = "lowercase")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Status {
    Ok = 0,
    NotFound = 1,
    InternalError = 2,
}
impl Status {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Status::Ok => "Ok",
            Status::NotFound => "NotFound",
            Status::InternalError => "InternalError",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Ok" => Some(Self::Ok),
            "NotFound" => Some(Self::NotFound),
            "InternalError" => Some(Self::InternalError),
            _ => None,
        }
    }
}
