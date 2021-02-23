//! A prost toolkit to build protobuf with serde support.
//!
//! Usually when we define our protobuf messages, we hope some of the generated
//! data structures have good serde support. Fortunately `serde-build` has that
//! capability - you can create a config with `prost_build::Config::new()` and
//! and then set proper attributes for type or field. For exmaple, you can add
//! serde support for this structure by using:
//!
//! ```ignore
//! config.type_attribute("package.RequestPing", "#[derive(serde::Serialize, serde::Deserialize)]");
//! config.type_attribute("package.RequestPing", "#[serde(default)]");
//! ```
//!
//! and you will get this generated code:
//!
//! ```ignore
//! #[derive(serde::Serialize, serde::Deserialize)]
//! #[serde(default)]
//! #[derive(Clone, PartialEq, ::prost::Message)]
//! pub struct RequestPing {
//!     #[prost(string, tag = "1")]
//!     pub ping: ::prost::alloc::string::String,
//! }
//! ```
//!
//! This crate helps to simplify this build script by using a predefined build configuration.
//!
//! # Getting started
//!
//! First of all, you shall create a JSON file which contains configuration for the builder. You can
//! get a copy of a default JSON file from: [default_build_config.json](https://raw.githubusercontent.com/tyrchen/prost-helper/master/prost-serde/default_build_config.json). See an example of [build_config.json](https://raw.githubusercontent.com/tyrchen/prost-helper/master/prost-serde/examples/build_config.json).
//! Please add the proto files, proto includes, output dir, and the data structure or field you want
//! to add the desired attributes.
//! Then you could use it in:
//!
//! ```ignore
//! use prost_serde::build_with_serde;
//!
//! fn main() {
//!     let json = include_str!("../examples/build_config.json");
//!     build_with_serde(json);
//! }
//! ```

use serde::{Deserialize, Serialize};
use std::{fs, process::Command};

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(default)]
pub struct BuildConfig {
    /// protobuf include dirs
    pub includes: Vec<String>,
    /// protobuf files
    pub files: Vec<String>,
    /// dir for generated code
    pub output: String,
    /// build options for serde support
    pub opts: Vec<BuildOption>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(default)]
pub struct BuildOption {
    /// scope of the attribute
    pub scope: String,
    /// description of the option
    pub description: String,
    /// extra attribute to put on generated data structure, for example: `#[derive(Serialize, Deserialize)]`
    pub attr: String,
    /// a list of paths you want to add the attribute
    pub paths: Vec<String>,
}

/// Build the protobuf files with the build opts provided by a JSON string.
///
/// Normally you should put the json file in your crate, and load it with `include_str!`,
/// then pass it to this build function.
pub fn build_with_serde(json: &str) -> BuildConfig {
    let build_config: BuildConfig = serde_json::from_str(json).unwrap();

    let mut config = prost_build::Config::new();
    for opt in build_config.opts.iter() {
        for path in opt.paths.iter() {
            match opt.scope.as_str() {
                "type" => config.type_attribute(path, opt.attr.as_str()),
                "field" => config.field_attribute(path, opt.attr.as_str()),
                "bytes" => config.bytes(&[path]),
                "btree_map" => config.btree_map(&[path]),
                v => panic!("Not supported type: {}", v),
            };
        }
    }

    fs::create_dir_all(&build_config.output).unwrap();
    config.out_dir(&build_config.output);

    config
        .compile_protos(&build_config.files, &build_config.includes)
        .unwrap_or_else(|e| panic!("Failed to compile proto files. Error: {:?}", e));

    Command::new("cargo")
        .args(&["fmt"])
        .status()
        .expect("cargo fmt failed");

    build_config
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generate_serde_supported_code() {
        let json = include_str!("../examples/build_config.json");
        build_with_serde(json);
    }
}
