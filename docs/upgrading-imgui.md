# Updating to new imgui versions

This document covers how to upgrade imgui-rs to a new version of the upstream C++ library.

The process is much the same to build imgui-rs for a tagged release (as shown) as it is for any arbitrary revision (such as one on a different branch)

## Summary

In short, there are a few steps:

1. Update copy of imgui itself
2. Run cimgui to generate C wrapper for imgui
3. Run bindgen
4. Fix up the imgui-rs wrapper
5. Update version in README badge (edit the URL)

## Step by step

1. Update the copies of `imgui` in `imgui-sys/third-party/imgui-*/imgui/` from the appropriate branches on [the upstream repo](https://github.com/ocornut/imgui)

    Each branch should generally be from roughly the same point in time. Generally just after each imgui release the `docking` branch is updated, so it's usually easy to find an equivalent commit in both.

    We trim some of the "unrequired" parts of imgui, such as it's `.github` directory, the `backends` and `docs`. We are mainly just interested in the main `.cpp` and `.h` files, as well as `misc/freetype/` support files.

    There's a simple shell script to perform the updates at `imgui-sys/third-party/update-imgui.sh` - this also serves as documentation of what revision was used.

2. Ensure `luajit` is installed, as this is required by cimgui's generator.

   $ luajit --help

3. Check out the `cimgui` project somewhere:

   ```sh
       git clone --recursive https://github.com/cimgui/cimgui.git /tmp/cimgui
   ```

    Make sure the checkout is updated and on a reasonable a reasonably recent version. Old versions can produce differently named symbols which can make updates more tediuos than they need to be! Generally the tag corresponding to the latest imgui release is a good choice.

4. For each of the branches, run the corresponding `update-cimgui-output.sh` script.

   ```sh
       $ pwd
       .../imgui-sys/third-party/imgui-master
       $ ./update-cimgui-output.sh /tmp/cimgui/
       [...]
       copyfile ./output/cimgui.h ../cimgui.h
       copyfile ./output/cimgui.cpp ../cimgui.cpp
       all done!!
       $ cd ../imgui-docking/
       $ ./update-cimgui-output.sh /tmp/cimgui/
       ...
       all done!!
   ```

   This updates various files like `cimgui.cpp`, `definitions.json` and so on

   With this step, we now have new C bindings to the desired verison of Dear ImGui.

5. Back in the root of the imgui-rs repo, run `cargo xtask bindgen`

    This step generates `imgui-sys/src/bindings.rs` etc which are then used by `imgui/src/*`

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

    Be sure to check `bindgen --version` versus the previously used version which is recoded in the first line of `imgui-sys/src/bindings.rs` - if you use a different version, you may get slightly different bindings which could also cause an update to be more work than it would otherwise be with matching bindgen versions

6. Run `cargo build` and fix any errors caused by changes upstream (see next section)

7. Run the tests with `cargo test`.

8. Try running one of the examples

    ```sh
        cargo run --example test_window_impl
    ```

9. Update README to reference correct Dear ImGui (updating the URL in the badge)

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
