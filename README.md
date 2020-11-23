# Gstreamer Rust Audio Plugins using Android Oboe Library

## Build

Please setup your android environment variables and your android version of gstreamer libraries at first.

```
export GST_PKG_CONFIG=/path/to/your/android/version/of/gstreamer/pkgconfig
export BINDGEN_EXTRA_CLANG_ARGS=-I$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/include/c++/v1/
PKG_CONFIG_PATH= PKG_CONFIG_LIBDIR=$GST_PKG_CONFIG/armv7 cargo ndk --platform 21 --target=armv7-linux-androideabi build --release
```

## LICENSE
gst-oboe-rs and all crates contained in here are licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or
  http://opensource.org/licenses/MIT)

at your option.

## Contribution

Any kinds of contributions are welcome as a pull request.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in gst-oboe-rs by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
