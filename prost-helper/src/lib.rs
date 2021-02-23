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

#[cfg(feature = "b64")]
mod buf;

#[cfg(feature = "b64")]
pub use buf::*;

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
}
