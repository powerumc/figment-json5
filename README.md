# Figment JSON5 Provider [![ci.svg]][ci] [![crates.io]][crate] [![docs.rs]][docs]

[Figment](https://docs.rs/figment/latest/figment/) provider for [JSON5](https://json5.org/) format

[examples/config.json5](./examples/config.json5)

```json5
{
  // Allow comments
  "name": "json5",
  "age": 0x28, // Allow hexadecimal numbers
  "description": "This is a \
test config", // Allow multiline strings
  "leadingDecimalPoint": .8675309,
  "andTrailing": 8675309.,
  "positiveSign": +1,
  "address": "Seoul", // Allow trailing commas
}
```

```rust
use figment::Figment;
use figment::providers::Format;
use serde::Deserialize;
use figment_json5::Json5;

#[derive(Debug, Deserialize)]
struct Config {
    name: String,
    description: String,

    #[serde(rename = "leadingDecimalPoint")]
    leading_decimal_point: f64,

    #[serde(rename = "andTrailing")]
    and_trailing: f64,

    #[serde(rename = "positiveSign")]
    positive_sign: i32,

    age: u32
}

fn main() {
    let config: Config = Figment::new()
        .merge(Json5::file("./examples/config.json5"))
        .extract()
        .unwrap();

    println!("{:#?}", config)

    // print result
    /*
Config {
    name: "json5",
    description: "This is a test config",
    leading_decimal_point: 0.8675309,
    and_trailing: 8675309.0,
    positive_sign: 1,
    age: 40,
}
    */
}
```

# Overview

This crate provides a [Figment](https://docs.rs/figment/latest/figment/) provider for JSON5 format.
JSON5 is a superset of JSON that allows comments, trailing commas, and more.

[What is JSON5?](https://json5.org/)
> JSON5 is an extension to the popular JSON file format
> that aims to be easier to write and maintain by hand (e.g. for config files).
> It is not intended to be used for machine-to-machine communication. (Keep using JSON or other file formats for that.
> 🙂)

# Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
figment = "0.10"
figment-json5 = "0.1.1"
```

# License

`figment-json5` is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0).


[crates.io]: https://img.shields.io/crates/v/figment-json5.svg

[crate]: https://crates.io/crates/figment-json5

[docs.rs]: https://docs.rs/figment-json5/badge.svg

[docs]: https://docs.rs/figment-json5

[ci.svg]: https://github.com/powerumc/figment-json5/actions/workflows/ci.yml/badge.svg

[ci]: https://github.com/powerumc/figment-json5/actions