# imgui-rs: Rust bindings for ImGui

**Ultra hyper turbo cyber mega extra über experimental!!!**

![Hello world](hello_world.png)

```rust
frame.window()
    .name(im_str!("Hello world"))
    .size((300.0, 100.0), ImGuiSetCond_FirstUseEver)
    .build(|| {
        frame.text(im_str!("Hello world!"));
        frame.text(im_str!("This...is...imgui-rs!"));
        frame.separator();
        let mouse_pos = frame.imgui().mouse_pos();
        frame.text(im_str!("Mouse Position: ({:.1},{:.1})", mouse_pos.0, mouse_pos.1));
    })
```

## Currently implemented things

* Low-level API (ffi module)
* Renderer for easy integration with [Glium](https://github.com/tomaka/glium) projects (optional)
* Parts of high-level API
* Not horrible way of defining and passing null-terminated UTF-8 to ImGui
* Parts of imgui\_demo.cpp reimplemented in Rust as an API usage example (examples/test\_window.rs)

## Important but unimplemented things

* Documentation (rustdoc)
* Support passing a custom Program to Glium renderer (e.g. from a shader cache, or custom shader)

## Core design questions and current choices

* Closures VS begin/end pairs (current choice: closures)
* Mutable references VS return values (current choice: mutable references)
* Passing around Frame&lt;'fr&gt; VS passing around &amp;'fr Frame (current choice: Frame&lt;'fr&gt;)
* Splitting the API to smaller pieces VS all draw calls in Frame (current choice: all draw calls in Frame)
* Builder pattern for optional arguments VS something else (current choice: builder)
* Mutation functions in builders VS self-consuming functions in builders (current choice: self-consuming)

## Compiling and running the demos

    git clone https://github.com/Gekkio/imgui-rs
    cd imgui-rs
    git submodule update --init --recursive
    cargo test

    target/debug/examples/hello_world
    target/debug/examples/test_window

## License

imgui-rs is licensed under the MIT license.

Uses [ImGui](https://github.com/ocornut/imgui) and [cimgui](https://github.com/Extrawurst/cimgui).
