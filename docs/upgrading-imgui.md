# Updating to new imgui versions

This document covers how to upgrade imgui-rs to a new version of the upstream C++ library.

The process is much the same to build imgui-rs for a tagged release (as shown) as it is for any arbitrary revision (such as one on a different branch)

## Step by step

1. Ensure the submodules are populated (`git submodule init` and `git submodule update --recursive`)

2. Check out the desired version of the `imgui-sys/third-party/imgui/` submodule

   ```sh
       $ pwd
       .../imgui-sys/third-party/imgui
       $ git checkout v1.81
       Previous HEAD position was 58075c44 Version 1.80
       HEAD is now at 4df57136 Version 1.81
   ```

3. Ensure `luajit` is installed, as this is required by cimgui's generator.

   $ luajit --help

4. Check out the `cimgui` project somewhere, as we use use the generator within this

   ```sh
       git clone --recursive https://github.com/cimgui/cimgui.git /tmp/cimgui
   ```

5. Ensure the `imgui` submodule within `cimgui` is pointing to the same revision as in `imgui-rs`

   ```sh
   $ cd /tmp/cimgui/imgui
   $ git checkout v1.81
   HEAD is now at 4df57136 Version 1.81
   ```

6. Back in `imgui-rs/imgui-sys/third-party/` - run the `update-cimgui-output.sh` helper script to execute cimgui's generator

   ```sh
       $ pwd
       .../imgui-sys/third-party
       $ ./update-cimgui-output.sh /tmp/cimgui/
       [...]
       copyfile ./output/cimgui.h ../cimgui.h
       copyfile ./output/cimgui.cpp ../cimgui.cpp
       all done!!
   ```

   This updates various files in the imgui-sys folder like `cimgui.cpp`, `definitions.json` and so on

   With this step, we now have new C bindings to the desired verison of Dear ImGui.

7. Back in the root of the imgui-rs repo, run `cargo xtask bindgen`

    ```sh
        $ cargo xtask bindgen
        Finished dev [unoptimized + debuginfo] target(s) in 0.04s
        Running `target/debug/xtask bindgen`
        Executing bindgen [output = .../imgui-rs/imgui-sys/src/bindings.rs]
        Success [output = .../imgui-rs/imgui-sys/src/bindings.rs]
        Executing bindgen [output = .../imgui-rs/imgui-sys/src/wasm_bindings.rs]
        Success [output = .../imgui-rs/imgui-sys/src/wasm_bindings.rs]
    ```

    This requires bindgen to be installed (`cargo install bindgen` should do it)

    This step generates `imgui-sys/src/bindings.rs` which is used by `imgui/src/*`

8. Run `cargo build` and fix any errors caused by changes upstream (see next section)

9. Run the tests with `cargo test`.

10. Try running one of the examples

    ```sh
        cargo run --example test_window_impl
    ```

## Common sources of problems

### Function changes

Check the upstream imgui release notes for the new versions, as they detail any breaking changes.

If functions have been renamed, the required changes to the bindings are usually simple.

If functions have been removed, the changes are usually also simple but the implications may require some thought. Note by default `cimgui` generator will exclude any obsolete API.

If new function overloads are added - for example `imgui::Thing()` existed but `imgui::Thing(float)` was added - `bindings.rs` will previously have contained only `igThing`, but will now contain `igThingNil()` and `igThingFloat(...)`

### Memory layout changes

It is common for upstream to add/remove/reorder fields, so the bindings will compile but the memory layout will not match - which will (hopefully) result in the bindings causing a segfault. These are not tagged as breaking changes in the release notes.

The `*_memory_layout` tests when running `cargo test` should catch these (if they are created for every relevant struct!)

The fix for this is usually to compare the struct in (read-only) `imgui-sys/src/bindings.rs` compared to the relevant struct in `imgui/src/...` - the ordering and data-types must match, but the names do not (structs in `imgui/src/...` should use conventional Rust naming/casing)
