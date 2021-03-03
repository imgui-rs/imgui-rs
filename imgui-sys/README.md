# Updating to new imgui versions

1. Ensure the submodules are populated (`git submodule init --recursive` and `git submodule update --recursive`)

2. Change into `imgui-sys/third-party/cimgui/` and check the current version, e.g

    $ git status
    HEAD detached at 1.81
    nothing to commit, working tree clean

3. Update upstream `cimgui`

    $ git remote update
    Fetching origin

4. Switch to a new branch, e.g

    $ git checkout 1.82
    HEAD is now at ...

5. Check the nested `imgui-sys/third-party/cimgui/imgui/` submodule pointing at the correct version:

    $ pwd
    .../imgui-rs/imgui-sys/third-party/cimgui
    $ git status
    HEAD detached at 1.81
    nothing to commit, working tree clean
    $ git log
    ...
    $ cd imgui/
    $ git status
    HEAD detached at v1.81
    nothing to commit, working tree clean
    $ git log
    ...

  If these versions differ, run `git checkout v1.82` in the `imgui` folder (noting cimgui uses a different tag naming convention to imgui!)

6. Back in the root of the imgui-rs repo, run `cargo xtask bindgen`

    $ cargo xtask bindgen
        Finished dev [unoptimized + debuginfo] target(s) in 0.04s
         Running `target/debug/xtask bindgen`
    Executing bindgen [output = .../imgui-rs/imgui-sys/src/bindings.rs]
    Success [output = .../imgui-rs/imgui-sys/src/bindings.rs]
    Executing bindgen [output = .../imgui-rs/imgui-sys/src/wasm_bindings.rs]
    Success [output = .../imgui-rs/imgui-sys/src/wasm_bindings.rs]

  This requires bindgen to be installed (`cargo install bindgen` should do it)

  This step generates `imgui-sys/src/bindings.rs` which is used by `imgui/src/*`

7. Run `cargo build` and fix any errors from upstream.

8. Run the tests with `cargo test`.

9. Try running one of the examples

    cargo run --example test_window_impl

### Building a specific revision

cimgui has pre-generated bindings to specific versions of imgui - usually for each regular imgui release, and the WIP docking branch at the same time as the release.

However this will not work if you need to either

1. Build `imgui-rs` against a specific revision, or
2. Build `imgui-rs` against another branch or fork

Luckily running the generator is quite straight forward:

1. Ensure `luajit` is installed (required by cimgui's generator)
2. In the `cimgui` submodule, check out the master branch
3. Update the nested `imgui` submodule (`imgui-sys/third-party/cimgui/imgui/`) to point to your desired upstream `imgui`
4. Run the generator as per https://github.com/cimgui/cimgui#using-generator
5. Run `cargo xtask bindgen` and follow the rest of the steps as usual

## Common sources of problems

### Function changes

Check the upstream imgui release notes for the new versions, as they detail any breaking changes.

If functions have been renamed, the required changes to the bindings are usually simple.

If functions have been removed, the changes are usually also simple but the implications may require some thought.

If new function overloads are added - for example `imgui::Thing()` existed but `imgui::Thing(float)` was added - `bindings.rs` will previously have contained only `igThing`, but will now contain `igThingNil()` and `igThingFloat(...)`

### Memory layout changes

It is common for upstream to add/remove/reorder fields, so the bindings will compile but the memory layout will not match - which will (hopefully) result in the bindings causing a segfault. These are not tagged as breaking changes in the release notes.

The `*_memory_layout` tests when running `cargo test` should catch these (if they are created for every relevant struct!)

The fix for this is usually to compare the struct in (read-only) `imgui-sys/src/bindings.rs` compared to the relevant struct in `imgui/src/...` - the ordering and data-types must match, but the names do not (structs in `imgui/src/...` should use conventional Rust naming/casing)
