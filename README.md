# kvmi-sys

![](https://github.com/Wenzel/kvmi-sys/workflows/Build/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/kvmi-sys.svg)](https://crates.io/crates/kvmi-sys)
![100% unsafe](https://img.shields.io/badge/unsafe-100%25-blue.svg)

> Unsafe Rust FFI bindings for libkvmi library

## Table of Contents

- [Overview](#overview)
- [Requirements](#requirements)
- [Build](#build)
- [License](#license)

## Overview

This crate will compile _unsafe_ Rust bindings for the VM introspection library [libkvmi](https://github.com/bitdefender/libkvmi)

Check `libkvmi.h` header for the API.

## Requirements

- `Rust` toolchain
- `clang`

## Build

~~~
cargo build
~~~

## License

[GNU General Public License v3.0](https://github.com/Wenzel/kvmi-sys/blob/master/LICENSE)

