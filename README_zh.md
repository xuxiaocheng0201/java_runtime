# Java 运行时

[![Crate](https://img.shields.io/crates/v/java_runtime.svg)](https://crates.io/crates/java_runtime)
![Crates.io License](https://img.shields.io/crates/l/java_runtime)
[![GitHub last commit](https://img.shields.io/github/last-commit/xuxiaocheng0201/java_runtime)](https://github.com/xuxiaocheng0201/java_runtime/commits/master)
[![GitHub issues](https://img.shields.io/github/issues-raw/xuxiaocheng0201/java_runtime)](https://github.com/xuxiaocheng0201/java_runtime/issues)
[![GitHub pull requests](https://img.shields.io/github/issues-pr/xuxiaocheng0201/java_runtime)](https://github.com/xuxiaocheng0201/java_runtime/pulls)

**其他语言版本：[English](README.md)，[简体中文](README_zh.md)。**

# 描述

自动检测java路径，在不存在的情况下自动下载安装java。

这样，就可以更方便地在任何环境下使用 [`j4rs`](https://crates.io/crates/j4rs) 库。


# Features

- [x] 检测已安装的 Java 路径 （通过 [`java_locator`](https://crates.io/crates/java_locator) 库）
- [ ] 验证 Java 版本
- [x] 自动下载安装 Java
- [ ] 下载进度回调
- [ ] 离线安装
- [ ] 全平台支持（现在仅支持 `Windows`）
- [ ] 更多 jre/jdk 版本支持（现在仅有 `jdk8u402`）


# 用法

将以下内容添加到你的`Cargo.toml`：

```toml
[dependencies]
java_runtime = "~0.0"
```


# 示例

```rust
use java_runtime::{Result, prepare_java8};

fn main() -> Result<()> {
    prepare_java8()?;
    Ok(())
}
```
