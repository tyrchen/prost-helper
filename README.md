# prost-helper

Two crates are provided to facilitate [prost](https://github.com/danburkert/prost) to better work with protobuf:
- prost-serde: help you to generate serde compatible code with prost and protobuf files. More in [documentation](https://docs.rs/prost-serde).
- prost-helper: macros and functions to facilitate prost. More in [documentation](https://docs.rs/prost-helper).

## Using `prost-helper` in a Cargo Project

First, add `prost-serde` and `prost-helper` to your `Cargo.toml`:

```
[dependencies]
prost-helper = "0.1"

[build-dependencies]
prost-serde = "0.1"
```

Then copy the [default_build_config.json](prost-serde/default_build_config.json) to your project and customize it. See more information in [prost-serde/examples/build_config.json](prost-serde/examples/build_config.json). Then you could add this in your build.rs:

```rust
fn main() {
    let json = include_str!("path/to/your/build_config.json");
    build_with_serde(json);
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
