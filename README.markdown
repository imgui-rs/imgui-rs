# imgui-rs: Rust bindings for ImGui

**Ultra hyper turbo cyber mega extra über experimental!!!**

ffi module (low-level API) is complete, the safe API is not!

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
