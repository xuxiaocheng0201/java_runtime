# Java Runtime

[![Crate](https://img.shields.io/crates/v/java_runtime.svg)](https://crates.io/crates/java_runtime)
![Crates.io License](https://img.shields.io/crates/l/java_runtime)
[![GitHub last commit](https://img.shields.io/github/last-commit/xuxiaocheng0201/java_runtime)](https://github.com/xuxiaocheng0201/java_runtime/commits/master)
[![GitHub issues](https://img.shields.io/github/issues-raw/xuxiaocheng0201/java_runtime)](https://github.com/xuxiaocheng0201/java_runtime/issues)
[![GitHub pull requests](https://img.shields.io/github/issues-pr/xuxiaocheng0201/java_runtime)](https://github.com/xuxiaocheng0201/java_runtime/pulls)

**Read this in other languages: [English](README.md), [简体中文](README_zh.md).**

# Description

Automatically detect Java path and download and install Jdk8u402 if it does not exist.

So that you can easily use [`j4rs`](https://crates.io/crates/j4rs) in any environment.


# Features

- [x] Detect installed Java path. (Using [`java_locator`](https://crates.io/crates/java_locator) crate.)
- [ ] Validate Java version.
- [x] Auto download and install Java`.
- [ ] Download progress callback.
- [ ] Offline feature.
- [ ] All platform support. (Now is only `Windows`.)
- [ ] More jre/jdk version support. (Now is only `jdk8u402`.)


# Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
java_runtime = "~0.0"
```


# Example

```rust
use java_runtime::{Result, prepare_java8};

fn main() -> Result<()> {
    prepare_java8()?;
    Ok(())
}
```
