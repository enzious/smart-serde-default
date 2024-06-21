# smart-serde-default

[![License](https://img.shields.io/github/license/enzious/smart-serde-default)](https://github.com/enzious/smart-serde-default/blob/master/LICENSE.md)
[![Contributors](https://img.shields.io/github/contributors/enzious/smart-serde-default)](https://github.com/enzious/smart-serde-default/graphs/contributors)
[![GitHub Repo stars](https://img.shields.io/github/stars/enzious/smart-serde-default?style=social)](https://github.com/smart-serde-default/smart-serde-default)
[![crates.io](https://img.shields.io/crates/v/smart-serde-default.svg)](https://crates.io/crates/smart-serde-default)

A crate that basically extends the [serde-inline-default] and [smart-default]
crates functionality to allow you to specify a default for both
[std::default::Default] and [serde] with one annotation.

## Documentation

- [API Documentation](https://docs.rs/smart-serde-default)

## Defining defaults
```rust
#[smart_serde_default]
#[derive(Debug, Deserialize, Serialize, SmartDefault)]
pub struct FuzionRedisConfigBuilder {
  #[smart_default(String::from("127.0.0.1"))]
  host: String,

  #[smart_default(6379)]
  port: u16,
}
```

[serde-inline-default]: https://docs.rs/serde-inline-default
[smart-default]: https://docs.rs/smart-default
[serde]: https://docs.rs/serde
[std::default::Default]: https://doc.rust-lang.org/std/default/trait.Default.html
