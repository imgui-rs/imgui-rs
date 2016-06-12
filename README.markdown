# imgui-rs: Rust bindings for ImGui

**Ultra hyper turbo cyber mega extra über experimental!!!**

[![Build Status](https://travis-ci.org/Gekkio/imgui-rs.svg?branch=master)](https://travis-ci.org/Gekkio/imgui-rs)
[![Latest release on crates.io](https://meritbadge.herokuapp.com/imgui)](https://crates.io/crates/imgui)

![Hello world](hello_world.png)

```rust
ui.window("Hello world")
    .size((300.0, 100.0), ImGuiSetCond_FirstUseEver)
    .build(|| {
        ui.text("Hello world!");
        ui.text("This...is...imgui-rs!");
        ui.separator();
        let mouse_pos = ui.imgui().mouse_pos();
        ui.text("Mouse Position: ({:.1},{:.1})", mouse_pos.0, mouse_pos.1);
    })
```

## Currently implemented things

* Low-level API (imgui-sys)
* Renderer for easy integration with [Glium](https://github.com/tomaka/glium) projects (optional)
* Parts of high-level API
* Uses ImGui [fork](https://github.com/bitshifter/imgui/tree/imstr) with string slice support
  for passing Rust strings to ImGui. For more information and justification for this design, please see
  [issue #7 comments](https://github.com/Gekkio/imgui-rs/issues/7#issuecomment-174228805)
* Parts of imgui\_demo.cpp reimplemented in Rust as an API usage example (examples/test\_window\_impl.rs)

## Important but unimplemented things

* Documentation (rustdoc)
* Support passing a custom Program to Glium renderer (e.g. from a shader cache, or custom shader)

## Core design questions and current choices

* Closures VS begin/end pairs (current choice: closures)
* Mutable references VS return values (current choice: mutable references)
* Passing around Ui&lt;'ui&gt; VS passing around &amp;'ui Ui (current choice: Ui&lt;'ui&gt;)
* Splitting the API to smaller pieces VS all draw calls in Ui (current choice: all draw calls in Ui)
* Builder pattern for optional arguments VS something else (current choice: builder)
* Mutation functions in builders VS self-consuming functions in builders (current choice: self-consuming)

## Compiling and running the demos

    git clone https://github.com/Gekkio/imgui-rs
    cd imgui-rs
    git submodule update --init --recursive
    cargo test

    cargo run --example hello_world
    cargo run --example test_window
    cargo run --example test_window_impl

## How to contribute

1. Change or add something
2. Run rustfmt to guarantee code style conformance

        cargo install fmt
        cargo fmt -- --write-mode=overwrite

3. Open a pull request in Github

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Uses [ImGui](https://github.com/ocornut/imgui) and [cimgui](https://github.com/Extrawurst/cimgui).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
