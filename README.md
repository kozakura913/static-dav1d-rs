# libdav1d bindings [![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

It is a simple FFI binding and safe abstraction over [dav1d][1].


## Building
```
apt install nasm ninja meson  
cargo test
```
## Cross Compile
[Default supported targets](https://github.com/kozakura913/static-dav1d-rs/tree/master/dav1d-sys/crossfiles)  
Customized cross compilation with "*.meson" path specified in "DAV1D_CROSS_FILE" environment variable

## Supported versions

Built-in dav1d version is 1.1.0

## TODO
- [x] Simple bindings
- [x] Safe abstraction
- [ ] Examples

[1]: https://github.com/videolan/dav1d
