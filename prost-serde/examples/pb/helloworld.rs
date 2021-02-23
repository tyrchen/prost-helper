#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hello {
    #[prost(string, tag="1")]
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub msg: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    #[serde(deserialize_with = "prost_helper::deserialize_null_default")]
    pub field_may_be_null: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    #[serde(skip_serializing_if = "prost_helper::is_zero")]
    pub field_skip_zero: u64,
    #[prost(string, tag="4")]
    #[serde(skip_serializing)]
    pub filed_skip: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="5")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(btree_map="string, bytes", tag="6")]
    pub map: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="bytes", repeated, tag="7")]
    pub list_data: ::prost::alloc::vec::Vec<::prost::bytes::Bytes>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Status {
    Ok = 0,
    NotFound = 1,
    InternalError = 2,
}
