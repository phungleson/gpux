# GPUX

> GPUX is highly experimental and in an early stage of development. Its API may change frequently.

GPUX is a set of components and helpers for building applications using [GPUI](https://gpui.rs).

## Crates

GPUX includes multiple crates:

* [gpux-nes-css](https://github.com/phungleson/gpux/blob/main/crates/nes-css/README.md)
* [gpux-radix-colors](https://github.com/phungleson/gpux/blob/main/crates/radix-colors/README.md)
* [gpux-radix-themes](https://github.com/phungleson/gpux/blob/main/crates/radix-themes/README.md)

## Usage

We need to use the `phungleson/zed` fork at this time because it contains several important fixes.

```toml
[dependencies]
gpui = { git = "https://github.com/phungleson/gpui.git" }
```
