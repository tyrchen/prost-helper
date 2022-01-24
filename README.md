# prost-helper

Two crates are provided to facilitate [prost](https://github.com/danburkert/prost) to better work with protobuf:

- prost-build-config: help you to generate serde compatible code with prost and protobuf files. More in [documentation](https://docs.rs/prost-build-config).
- prost-helper: macros and functions to facilitate prost. More in [documentation](https://docs.rs/prost-helper).

## Using `prost-helper` in a Cargo Project

First, add `prost-build-config` and `prost-helper` to your `Cargo.toml`:

```
[dependencies]
prost-helper = "0.6"

[build-dependencies]
prost-build-config = "0.4"
```

Then copy the [prost-build-config/default_build_config.yml](prost-build-config/default_build_config.yml) to your project and customize it. See more information in [prost-build-config/examples/build_config.yml](prost-build-config/examples/build_config.yml). Then you could add this in your build.rs:

```rust
use prost_build_config::{BuildConfig, Builder};

fn main() {
    let config: BuildConfig = serde_yaml::from_str(include_str!("path/to/your/build_config.yml")).unwrap();
    Builder::from(config).build_protos();
}
```

That's it!

If you'd like to generate the implementation for `From` trait to `Vec<u8>`, or `TryFrom` trait from `Vec<u8>` to protobuf data structure, you could use `prost_into_vec!` and `vec_try_into_prost!` macros. Here is an example:

```rust
use prost::Message;
use std::convert::TryInto;

#[derive(Clone, PartialEq, Message)]
pub struct Hello {
    #[prost(string, tag = "1")]
    pub msg: String,
}

// implement the traits
prost_into_vec!(Hello, 32);
vec_try_into_prost!(Hello);


let hello = Hello::default();

// use `Into` to convert message to `Vec<u8>`
let data: Vec<u8> = hello.into();

// use `TryInto` to convert `Vec<u8>` back to message
let hello_result: Result<Hello, prost::DecodeError> = data.try_into();
```

Have fun with prost!

## License

`prost-helper` is distributed under the terms of MIT.

See [LICENSE](LICENSE.md) for details.

Copyright 2021 Tyr Chen
