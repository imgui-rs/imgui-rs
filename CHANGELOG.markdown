# Change Log

## [unreleased]
- Breaking: Removed `im_str!` macro - deprecated since v0.8.

  `ui.button(im_str!("Example"))` just becomes `ui.button("Example")` and `ui.button(&im_str!("My age is {}", 100))` becomes `ui.button!(format!("My age is {}", 100))`

- Breaking: Updated to Dear ImGui 1.89.1.

  This introduces some breaking changes like the `imgui::Key` now contains a full set of keys (previously it was a small subset of to cover copy/paste/undo)

- Breaking (partially): `ImageButton::new` is now deprecated, replaced by `ui.image_button_config(...)`.

  The old `new` method should be backwards-compatible in most common situations. Exception is if the `ImageButton` builder struct was explicitly specified, say in a method like `fn configure_my_button(button: &mut imgui::ImageButton)` (in which case either change `ImageButton` to `ImageButtonDeprecated`, or update to the new constructor)

- Updated `imgui-winit-support` to use new "event based IO" (detailed in the Dear ImGui 1.87 release notes, but basically it aims to improve behaviour at low frame rates). Existing custom backends should work without changes, but are advised to update to the new API.

- Accept `usize` and `isize` for parameters which use `DataTypeKind` (such as `Ui::input_scalar`). This treats them as `u64`/`i64` (or `u32`/`i32`) as appropriate

- The `examples` directories have been reorganized slightly.

  There is now an example in `imgui-glium-renderer` showing basic usage, consistent with the glow.

## [0.9.0] - 2022-11-30

- MSRV is now **1.57**. We soft-updated to this to Rust 1.54 in the v0.8.0 release (with a feature `min-const-generics`), which has now been removed (and as such, we resume having no default features). Rust 1.56 is required for the Rust 2021 edition, and 1.57 is required by some dependencies

- Upgraded from Dear ImGui 1.84.2 to 1.86. See [the 1.85](https://github.com/ocornut/imgui/releases/tag/v1.85) and [the 1.86](https://github.com/ocornut/imgui/releases/tag/v1.86) release notes

- Upgraded winit version to `v0.27` for `imgu-winit-support`

- The `imgui-winit-support` and `imgui-glow-renderer` re-export `winit` and `glow` respectively to make setup easier for simple projects. [PR #676](https://github.com/imgui-rs/imgui-rs/pull/676)

- BREAKING: Removed `push_style_colors` and `push_style_vars`. Instead, use `push_style_color` in a loop. This was deprecated in `0.7.0` and should have been removed in `0.8.0`. This also removes their associated tokens.

- BREAKING: Ui now does not have a lifetime associated with it, but is only ever given to users in the form of `&mut Ui`. Additionally, the `render` function has been moved to the `Context` instead of `Ui`.

- BREAKING: `SharedFontAtlas` now hides an `Rc` within its wrapper -- this simplifies the codebase and more accurately reflects how we expect `SharedFontAtlas` to be used (ie, you're probably going to set it up once, and then give it around, rather than constantly edit it). `SharedFontAtlas` users, if this change is very bad for you, please let us know with issues!

- BREAKING: `Id` is now a simpler facade, but requires the `Ui` struct to generate. `push_id`, equally, has been split into multiple functions for simplicity. New example `imgui-examples/examples/id_wrangling.rs` shows some of the `push_id` usage

- Added `imgui-sdl2-support` to provide a simple ImGui platform wrapper. Please give it a try! Thank you to @NightShade256 for [implementing this here](https://github.com/imgui-rs/imgui-rs/pull/541)

- BREAKING: We now only support `glium 0.30`. We're in a difficult position supporting arbitrary `glium` versions in our renderer, since `glium` is only in a semi-maintained state. `glium` users, please get in contact in issues to let us know what will work best for your needs!

- Added `InputScalar` and `InputScalarN`. These are the core Input modules that Dear ImGui uses, and ultimately what `InputFloat` and `InputInt` turn into. See deprecation of `InputFloat` and `InputInt` as a result. Thank you to @EmbersArc for [implementing this here](https://github.com/imgui-rs/imgui-rs/pull/544).

- BREAKING: `ui.input_int` and `ui.input_float` now return `InputScalar<'ui, 'l, f32/i32>`, instead of `InputFloat`/`InputInt`. This struct has all of the same flags as `InputFloat` and `InputInt` did.

- DEPRECATED: `InputFloat` and `InputInt` have been deprecated. `ui.input_float` and `ui.input_int` are _not_, however, and instead will just call `input_scalar` as appropriate. Therefore, please switch your code to `ui.input_float` or `ui.input_int`.

- Added `add_polyline` method to `DrawListMut`, which binds to Dear ImGui's `AddPolyline` and `AddConvexPolyFilled`

- BREAKING: The following structs have had their `new` method changed and deprecated; they now also take `ui` in their `new`, but you should create them on the `Ui` struct instead.
  - `Window` should be made with `ui.window` - e.g `ui.window("My Window").build(|| { ui.text("Contents") });`
  - `ChildWindow` should be made with `ui.child_window`
  - `MenuItem` should be made with `ui.menu_item` or `ui.menu_item_config`.
  - `DragDropSource` and `DragDropTarget` should be made with `ui.drag_drop_source_config` or `ui.drag_drop_target`. Both of these methods, and the DragDrop API in general, are likely to change.

- Added `docking` feature which builds against the upstream docking branch. Only basic API is exposed currently, just enough to enable the docking `imgui_context.io_mut().config_flags |= imgui::ConfigFlags::DOCKING_ENABLE;` - a safe API for programtically docking windows and so on will be added later (until then the internal docking API can be accessed, `imgui::sys::igDockBuilderDockWindow` and so on)

- Fixed dpi related issues when not in `HiDpiMode::Default` mode. The wrong scale factor was used when converting winit physical size to logical size, causing the imgui display size to be incorrect.

- Fixed creation of `.crate` (published to crates.io) so required files for freetype feature are included

- Added binding to TextFilter API. [PR #658](https://github.com/imgui-rs/imgui-rs/pull/658)

## [0.8.0] - 2021-09-17

Welcome to the `0.8.0` update. This is one of the largest updates imgui-rs has ever seen; it will generate errors in a `0.7` project, but hopefully it should be both quick to fix, and enjoyable to update. See our [release page](https://github.com/imgui-rs/imgui-rs/releases/tag/v0.8.0) for more information and a list of contributors to this cycle. Thank you to everyone who uses `imgui-rs`, files issues, and spend their time and effort to PR new changes into the codebase. Because of all that effort, this is by far the best `imgui-rs` has looked!

- **Removed ImStr and ImString from the API.** Currently `im_str!` is deprecated and **will be removed in 0.9.0**. To change your code:

  - If you were just wrapping a string literal, like `im_str!("button")`, just use `"button"`. (Help: the regex `im_str!\("((?:(?=(\\?))\2.)*?)"\)`, replacing matches with `"$1"`, can get the majority of these quickly.);
  - If you were formatting, like `&im_str!("My age is {}", 100)`, you can now just use format like `format!("My age is {}, 100)`. Notice that due to the trait bounds, you can pass the string in directly too.

- BREAKING: Most tokens through the repository (eg. `WindowToken`, `TabBarToken`, `FontStackToken`, etc) now allow for permissive dropping -- i.e, you don't need to actually call the `.end()` method on them anymore. In exchange, these tokens have taken on a lifetime, which allows them to be safe. This could make some patterns impossible. Please file an issue if this causes a problem.

  - `end()` no longer takes `Ui`. This is a breaking change, but hopefully should be trivial (and perhaps nice) for users to fix. Simply delete the argument, or add a `_` before the token's binding name and allow it to be dropped on its own. In our code, we tend to write these now like:

```rs
if let Some(_t) = ui.begin_popup("example") {
  // your code here
}
```

- BREAKING: Created `with_x` variants for most functions which previously took multiple parameters where some had default arguments in the C++. This makes calling most functions simpler and more similar to the C++.

  - The most likely breaking changes users will see is `button` and `same_line` now take one fewer parameter -- if you were calling `button` with `[0.0, 0.0]`, simply delete that -- otherwise, call `button_with_size`. Similarly, for `same_line`, if you were passing in `0.0.` simply delete that argument. Otherwise, call `same_line_with_pos`.

- ADDED: support for the `tables` API which was added in dear imgui `1.80`. We currently have this _feature gated_ behind `tables-api`. You should feel safe to use this in stable production, but be aware of two things:

  1. The tables API is marked as "beta" meaning that it may change with fewer stability promises. This is unlikely and it seems fairly settled.
  2. There are a few cases where the tables API will segfault by dereferencing a `NULL` where it should instead `ASSERT` and crash. This is simply annoying because you won't get a stacktrace. [See here for more info on that.](https://github.com/imgui-rs/imgui-rs/issues/524). If this is fixed upstream, we will issue a patch.

- ADDED: an `imgui-glow-renderer` which targets `glow 0.10`. Before release, this will be updated to target current `0.11` glow when further features are added. Thank you to @jmaargh for the work [implementing this here](https://github.com/imgui-rs/imgui-rs/pull/495)!

- UPGRADED: from v1.80 to [Dear ImGui v1.84.2](https://github.com/ocornut/imgui/releases/tag/v1.84.2) See the [Dear ImGui v1.84](https://github.com/ocornut/imgui/releases/tag/v1.84) release notes for more information. Thank you to @dbr for doing the work (twice actually) of [upgrading the repository](https://github.com/imgui-rs/imgui-rs/pull/519).

- BREAKING: Reworked how callbacks on `InputText` and `InputTextMultiline` work.

  - REMOVED `.callback_name()` methods in favor of one method: `.callback(FLAGS, CallbackStruct)`.
  - Wrapped callback kinds into their own enums, `InputTextCallback` and `InputTextCallbackMultiline`.
  - Created a trait, `InputTextCallbackHandler`.
  - To see how to create an InputText callback, see `examples/text_callback.rs`.
  - Finally, please note that editing an `&mut String` which contains `\0` within it will produce _surprising_ truncation within ImGui. If you need to edit such a string, please pre-process it.

- ADDED: `begin_disable` and `begin_enable` methods. These add (finally) support for disabling _any_ widget. Thank you to @dbr for [implementing this here](https://github.com/imgui-rs/imgui-rs/pull/519).

- BREAKING: MSRV is now **1.54**. This is gives us access to min-const-generics, which we use in a few places, but will gradually use more. Because this is the first time we've bumped MSRV intentionally, we have added a new feature `min-const-generics`, which is _enabled by default_. If you are pre-1.54, you can hang onto this update by disabling that feature. In our next update, this feature will be removed and we will commit to our MSRVs going forward. Thank you to @dbr for changing our CI infrastructure to support better MSRVs [here](https://github.com/imgui-rs/imgui-rs/pull/512).

- BREAKING: Changed default version of Winit in `imgui-winit-support` to `winit 0.25`. Thank you to @repi [for implementing this here](https://github.com/imgui-rs/imgui-rs/pull/485).

  - Removed automatically adding default features for `imgui-winit-support`
    with the exception of the current default winit feature/dep version. If you want to not have the default features of winit with 0.25, set `default-features = false` and add `winit-25` as a normal feature. Thank you to @dzil123 for the work [implementing this here](https://github.com/imgui-rs/imgui-rs/pull/477)!

- ADDED: Support for the freetype font rasterizer. Enabled by the non-default `freetype` feature, e.g `imgui = {version = "...", features=["freetype"]})`
  Thank you to @dbr for this work [implementing this here](https://github.com/imgui-rs/imgui-rs/pull/496).

- ADDED: `doc alias` support throughout the repository. You can now, [inside the docs](https://docs.rs/imgui), search for `imgui-rs` functions by their `Dear ImGui` C++ names. For example, searching for `InputText` will pull up `Ui::input_text`. This was quite a lot of documentation and effort, so thank you to @toyboot4e [for implementing this here](https://github.com/imgui-rs/imgui-rs/pull/458).

- ADDED: text hinting into `InputText`. Thank you to @lwiklendt [for implementing this here](https://github.com/imgui-rs/imgui-rs/pull/449).

- BREAKING: Reworked `.range` calls on `Slider`, `VerticalSlider`, and `Drag` to simply take two min and max values, and requires that they are provided in the constructor.

  - To update without changing behavior, use the range `T::MIN` and `T::MAX` for the given numerical type (such as `i8::MIN` and `i8::MAX`).
  - Using `.range` is still maintained for simplicity, but will likely be deprecated in 0.9 and removed in 0.10!

- `DrawListMut` has new methods to draw images

  - The methods are `add_image`, `add_image_quad`, and `add_image_rounded`. The `imgui-examples/examples/custom_textures.rs` has been updated to show their usage.
  - Additionally the `imgui::draw_list` module is now public, which contains the various draw list objects. While the `add_*` methods are preferred, `imgui::draw_list::Circle::new(&draw_list_mut, ...).build()` is equivalent
  - Finally, we have relaxed the limits around having multiple draw lists such that you can have multiple mutable draw lists of different kinds (ie, a `foreground` and a `background` at the same time.).
  - Thank you to @dbr for [implementing these changes](https://github.com/imgui-rs/imgui-rs/pull/445).

- ADDED: the `ButtonFlags` which previously prevented `invisible_button` from being usable. Thank you to @dbr for [implementing this change here](https://github.com/imgui-rs/imgui-rs/pull/509).

- BREAKING: `PopupModal`'s `new` was reworked so that it didn't take `Ui` until `build` was called. This is a breaking change if you were invoking it directly. Simply move your `ui` call to `build` or `begin`.

- BREAKING: Restored methods to access keyboard based on backend-defined keyboard map indexes. These allow access to most keys, not just those defined in the small subset of `imgui::Keys` (note the available keys may be expanded in future by [imgui PR #2625](https://github.com/ocornut/imgui/pull/2625))

  - The new methods on `imgui::Ui` are `is_key_index_down`, `is_key_index_pressed`, `is_key_index_pressed_no_repeat`, `is_key_index_released`, `is_key_index_released`
  - For example `ui.is_key_released(imgui::Key::A)` is same as `ui.is_key_index_released(winit::events::VirtualKeyCode::A as i32)` when using the winit backend

- BREAKING: Modifies `build` style methods to allow the provide closure to return a value. The build call will then return Some(value) if the closure is called, and None if it isn't.

  - The most likely breaking changes users will see is that they will need to add semicolons after calling `build`, because these function no longer return `()`.
  - Thank you to @AngelOfSol for [implementing this here](https://github.com/imgui-rs/imgui-rs/pull/468).

- BREAKING: Removed `imgui::legacy` which contained the old style of flags. The remaining flags in `imgui::legacy` have been updated to be consistent with other flags in the project.

  - `imgui::legacy::ImGuiDragDropFlags` were accidentally not cleared when they were remade in `drag_drop.rs` in v0.7.0.
  - `imgui::legacy::ImGuiInputTextFlags` is now `imgui::input_widgets::InputTextFlags`
  - `imgui::legacy::ImGuiTreeNodeFlags` is now `imgui::widget::tree::TreeNodeFlags`
  - `imgui::legacy::ImDrawListFlags` is now `imgui::draw_list::DrawListFlags`

- Full (32-bit) unicode support is enabled in Dear Imgui (e.g. `-DIMGUI_USE_WCHAR32` is enabled now). Previously UTF-16 was used internally.

  - BREAKING: Some parts of the font atlas code now use `char` (or `u32`) instead of `u16` to reflect this.
    - Note: `u32` is used over `char` in some situations, such as when surrogates are allowed
  - BREAKING (sorta): Dear Imgui now will use 32 bits for character data internally. This impacts the ABI, including sizes of structs and such, and can break some low level or advanced use cases:
    - If you're linking against extensions or plugins to Dear Imgui not written in Rust, you need to ensure it is built using `-DIMGUI_USE_WCHAR32`.
      - However, if the `DEP_IMGUI_DEFINE_` vars are [used properly](https://github.com/4bb4/implot-rs/blob/f2a4c6a3d8919ec3438631873ce6a9f94135089c/implot-sys/build.rs#L37-L45), this is non-breaking.
    - If you're using `features="wasm"` to "link" against emscripten-compiled Dear Imgui, you need to ensure you use `-DIMGUI_USE_WCHAR32` when compile the C and C++ code.
      - If you're using `DEP_IMGUI_DEFINE_`s for this already, then no change is needed.
    - If you're using `.cargo/config` to apply a build script override and link against a prebuilt `Dear Imgui` (or something else along these lines), you need to ensure you link with a version that was built using `-DIMGUI_USE_WCHAR32`.

## [0.7.0] - 2021-02-04

- Upgrade to [Dear ImGui v1.80](https://github.com/ocornut/imgui/releases/tag/v1.80). (Note that the new table functionality is not yet supported, however)

- `Ui::key_index()` is now called internally when needed, and the various `is_key_foo` now take a `Key` directly: https://github.com/imgui-rs/imgui-rs/pull/416

  - `is_key_down`, `is_key_pressed`, `is_key_released` and `key_pressed_amount` now take a `Key` instead of `u32` (breaking).
  - `key_index` is no longer public (breaking). If you need access to the key map, it can be accessed as `ui.io().key_map[key]` (If you need to do this, file a bug, since I'm open to exposing this if there's actually a use case).

- `winit` 0.23/0.24 handling has been (hopefully) fixed: https://github.com/imgui-rs/imgui-rs/pull/420 (breaking, see also https://github.com/imgui-rs/imgui-rs/issues/412).

  - `imgui-winit-support`'s `winit-23` feature no longer supports `winit` version `0.24` (this caused an unintentional semver breakage before, unfortunately).
  - `imgui-winit-support` has a new `winit-24` feature for 0.24 support.
  - By default `imgui-winit-support` feature now enables `winit-24`, and not `winit-23` (by default it will always enable the latest).

- The `imgui` crate no longer depends on `gfx` or `glium` directly: https://github.com/imgui-rs/imgui-rs/pull/420 (breaking, related to the previous change).

  - That is, the `gfx` and `glium` features are removed to reduce version compatibility issues going forward.
    - This only matters if you manually implement `gfx` or `glium` renderers without using the `imgui-glium-renderer` or `imgui-gfx-renderer` crates.
    - In the (somewhat unlikely) case you were relying on these this, you should define your own vertex type that's layout-compatible with `imgui::DrawVert`, and replace calls to `imgui::DrawList::vtx_buffer()` with `imgui::DrawList::transmute_vtx_buffer::<MyDrawVert>()`. You can see `imgui_glium_renderer::GliumDrawVert` and `imgui_gfx_renderer::GfxDrawVert` types respectively for examples of this, if needed, but it should be straightforward enough if you're already implementing a renderer from scratch.
  - This is admittedly less convenient, but avoids depending on any specific version of `gfx` or `glium` in the core `imgui` crate, which will ease maintenance and reduce unintentional breakage in the future.

- Non-window DrawList support has been fixed/improved: https://github.com/imgui-rs/imgui-rs/pull/414

  - `WindowDrawList` has been renamed to `DrawListMut`, to reflect that it may refer to other kinds of draw lists, and is mutable, unlike `imgui::DrawList` (breaking).
  - `Ui::get_background_draw_list()` has been fixed when used outside of a window context, and now has an example.
  - `Ui::get_foreground_draw_list()` has been added, analogous to `Ui::get_background_draw_list()`.

- Added drag drop support, with a safe and an unsafe variant: https://github.com/imgui-rs/imgui-rs/pull/428

  - `DragDropSource` allows users to create a dragdrop payload which is either empty, of `'static + Copy` data,
    or `unsafe`, allowing for theoretically arbitrary payloads.
  - `DragDropTarget` allows users to accept any of the above payloads.
  - Extensive documentation has been made on all of these features, hopefully as a target for future features.

- `ImColor` (which is a wrapper around `u32`) has been renamed to `ImColor32` in order to avoid confusion with the `ImColor` type from the Dear ImGui C++ code (which is a wrapper around `ImVec4`). In the future an `ImColor` type which maps more closely to the C++ one will be added.

  - Additionally, a number of constructor and accessor methods have been added to it `ImColor`, which are `const fn` where possible.

- The `im_str!` macro can now be used in `const` contexts (when the `format!` version is not used).

- `im_str!` now verifies that the parameter has no interior nuls at compile time. This can be avoided to get the old (truncating) behavior by forcing it to use the `format!`-like version, e.g. `im_str!("for_some_reason_this_should_be_truncated\0 there {}", "")`.

  - This is not recommended, and is probably not useful.

- Many functions are now `const fn`.

- A large number of small functions are now `#[inline]`, but many still aren't, so you probably will want to build with LTO for release builds if you use `imgui` heavily.

- The `io.config_windows_memory_compact_timer` flag has been renamed to `io.config_memory_compact_timer`. This follows the similar rename in the C++ ImGui, and was done because it no longer only applies to window memory usage.

- The variants of `ColorEditInputMode` and `ColorEditDisplayMode` have been renamed to be CamelCase instead of upper case (e.g. `ColorEditFooMode::RGB` => `ColorEditFooMode::Rgb`).
  - However, this change is probably not breaking (in practice if not in theory) because const aliases using the old names are provided.

## [0.6.1] - 2020-12-16

- Support for winit 0.24.x
  - Note: this change was accidentally semver-breaking, see the caveat below.
- Support multiple simultaneous winit versions in imgui-winit-support:
  - The latest will be if more than one is specified, and a single warning will be logged in debug builds (based on `cfg!(debug_assertions)`) at runtime if multiple are specified.
  - This is intended to make features behave a bit more closely to additively, and reduce the pain of using this crate in a larger workspace.
- Avoid dropping mouse events when press/release are on the same frame (macos)
- Substantial repository layout reorganization

### Caveat: Semver broken in 0.6.1 in `imgui-winit-support`

_Note from the future: `imgui-winit-support@0.6.1` has been yanked. I don't believe the breakage impacted the other crates so I'm leaving those to avoid impacting non-`winit` usages._

This release accidentally broke semver, and should have been 0.7.0. It will be yanked when 0.7.0 is released, unless there are objections.

This happened when updating the glium/winit/glium versions, adding support for winit `0.24` and related. Unfortunately, while an attempt to avoid breakage was made, it happened regardless. This mainly happened as it was the holidays and not enough attention was paid to the changes in an urgent-sounding request for supporting the new version, and more care will be taken in the future to avoid cutting a hasty release without adequate testing.

As mentioned, the 0.6.1 release of `imgui-winit-support` has been yanked.

## [0.6.0] - 2020-11-15

### Added

- `Io::peek_input_characters`

### Changed

- Upgrade to cimgui / imgui 1.79
- Upgrade to winit 0.23
- Bump minimum Rust version to 1.43

## [0.5.0] - 2020-09-20

### Added

- Support for ConfigFlags::RENDERER_HAS_VTX_OFFSET in imgui-glium-renderer and imgui-gfx-renderer. This makes it possible to output large meshes (e.g. complex UIs) without problems when using these renderers
- `Ui::begin_tooltip` to support using tooltips with stack tokens instead of closures
- API for accessing the background drawlist
- Tab bar / tab item API
- Redesigned drag slider API

### Changed

- Upgrade to cimgui / imgui 1.78
- Store per-texture sampler parameters in imgui-glium-renderer to support customizing them
- Slider widget constructors no longer require the range parameter. Call the range function on the builder to set the range.

### Fixed

- Reduce unnecessary winit cursor changes which earlier caused cursor flickering or invalid cursors on some platforms (at least Windows)

### Removed

- Various things that were deprecated in imgui-rs 0.4.0

## [0.4.0] - 2020-05-27

### Added

- WebAssembly FFI shells

### Changed

- Redesign tree / collapsing header API
- Bump minimum Rust version to 1.40 (at least xml-rs crate requires it)
- Upgrade to glium 0.27 / winit 0.22
- Switch Glium renderer to use GL_CLAMP_TO_BORDER

### Fixed

- Bug in font name length checking

## [0.3.1] - 2020-03-16

### Fixed

- Narrowed supported winit version range in imgui-winit-support

## [0.3.0] - 2020-02-15

### Added

- Add `ChildWindow::movable`
- ImString now implements fmt::Write

### Changed

- Upgrade to cimgui / imgui 1.75
- Bump minimum Rust version to 1.38 (at least backtrace crate requires it)
- Upgrade to glium 0.26 / winit 0.21
- Switch imgui-winit-support to 0.20+ by default. Winit 0.19 support is still
  available via the `winit-19` feature flag
- Resources used by examples are no longer included in the published crate

### Removed

- Various things that were deprecated in imgui-rs 0.2.0

### Fixed

- Fix toggling behavior on using `MenuItem::build_with_ref` and `Selectable::build_with_ref`.
- ImString nul terminator handling

## [0.2.1] - 2019-09-09

### Fixed

- Fix backspace handling on macOS
- Fix ImageButton bool return value

## [0.2.0] - 2019-09-07

### Added

- Window scrolling API
- Full support for the column API
- Almost all small utility functions from upstream API
- Support for winit 0.20 alpha via `winit-20` feature

### Changed

- Redesigned window API
- Redesigned progress bar API
- Redesigned color editor/picker API
- Redesigned child window API (previously known as child frame)
- Redesigned image / image button API
- Redesigned combo box API
- Redesigned selectable API
- Redesigned slider API. Generic scalar sliders support all main data types and replace previous individual sliders (int, int2, int3, int4, etc...)
- Redesigned menu API
- Updated layout API
- Renderer errors implement std::error::Error
- Glium renderer re-exports imgui and glium
- Gfx renderer re-exports imgui and gfx
- These functions now take/return PathBuf: log_filename, set_log_filename, ini_filename, set_logfilename
- ID stack manipulation now uses stack tokens
- Parameter stack pushes _must almost always be paired by a manual call to stack pop_
- Container widget tokens _must be ended manually by calling end_. Closure-based function (e.g. build()) are unaffected and do this automatically
- Bump minimum Rust version to 1.36 (some dependencies, including winit, require MaybeUninit)
- Upgrade to cimgui / imgui 1.72b

### Removed

- Various things that were deprecated in imgui-rs 0.1.0

## [0.1.0] - 2019-07-12

### Added

- Support for font atlas sharing
- Support for using multiple fonts
- Support for suspended contexts (useful for having multiple independent operating system windows)
- Support for DX11 in imgui-gfx-renderer
- Support for navigation input system
- Support for backend/renderer name strings
- Support for saving/loading INI settings manually
- Pluggable clipboard support

### Changed

- imgui-sys is now almost completely automatically generated. **This is a big breaking change in imgui-sys API**
- ImGui/Context API is now safer
- The library context (known as Context, previously known as ImGui) is no longer Send or Sync
- Many getter/setter APIs have been replaced with direct access to struct fields
- [f32; 2] and [f32; 4] are now the main vector types. ImVec/ImVec4 and corresponding tuples are no longer used in the main API
- imgui-gfx-renderer is parameterized over the color format, so Rgba8 and Srgba8 are both supported
- imgui-winit-support has been rewritten to provide a more robust abstraction that is easier to use correctly
- Parameter stack (e.g. StyleVar) manipulation is now done using push functions and automatically or manually droppable stack tokens
- Upgrade to glium 0.25
- Upgrade to cimgui / imgui 1.71
- Bump minimum Rust version to 1.33

## [0.0.23] - 2019-04-10

### Added

- Support for image buttons: `Ui::image_button`
- `Ui::set_keyboard_focus_here`
- Support for window position pivot

### Changed

- Upgrade to gfx 0.18

### Removed

- Various things that were deprecated in imgui-rs 0.0.21 and 0.0.22

## [0.0.22] - 2019-02-05

### Added

- `Ui::with_test_wrap_pos`
- `Ui::get_content_region_max`
- `Ui::get_window_content_region_min`
- `Ui::get_window_content_region_max`

### Changed

- Upgrade to cimgui 1.66.2+ / imgui 1.66b. **This is a very big update, so there are a lot of breaking changes**
- Bump minimum Rust version to 1.31 (1.28 required by the glutin crate, and 1.31 required by the stb_truetype crate)
- Upgrade to glium 0.23
- Replaced `imgui-glutin-support` with `imgui-winit-support`

## [0.0.21] - 2018-10-11

### Added

- `ImGui::mouse_down`
- `ImGui::key_super`
- `Ui::get_window_pos`
- `Ui::is_window_focused`
- `Ui::is_root_window_focused`
- `Ui::is_child_window_focused`
- `Ui::popup_modal`
- `imgui-glutin-support` crate
- Support for custom textures

### Fixed

- Possible crash if rendering was skipped during a frame

### Changed

- Bump minimum Rust version to 1.26 (required by the parking_lot_core crate)

## [0.0.20] - 2018-08-13

### Fixed

- Clip rect regression in the glium renderer

### Removed

- Various things that were deprecated in imgui-rs 0.0.19

## [0.0.19] - 2018-08-12

### Added

- New things in imgui/cimgui 1.53.1
  - Style: Add `PopupRounding`, `FrameBorderSize`, `WindowBorderSize`, `PopupBorderSize`.
  - DemoWindow: Add `no_close` state.
  - Input: Add `no_undo_redo` method.
  - _imgui-sys_:
    - `igStyleColorsDark` and `igStyleColorsLight`
    - DragDrop low level API
    - `igGetFrameHeight`
    - `igBeginCombo`, `igEndCombo`
    - `igSetItemDefaultFocus`
    - `igGetOverlayDrawList` and `igGetDrawListSharedData`
    - `ImFontConfig_DefaultConstructor`
    - `ImDrawList_AddImageRounded`
- Input: Add `read_only` and `password` methods.
- Various utility functions
- Support for changing the mouse cursor
- Custom font support
- Support for item grouping (`group` function)
- Custom drawing with draw list manipulation
- Drag widgets
- Input: Add `input_text_multiline` method

### Changed

- Upgrade to imgui/cimgui 1.53.1
  - Rename `Ui::show_test_window` to `Ui::show_demo_window`. Keep redirection.
  - Rename `sys::igGetItemsLineHeightWithSpacing` to `sys::igGetFrameHeightWithSpacing`. Keep redirection.
  - Rename `ImGuiTreeNodeFlags::AllowOverlapMode` to `ImGuiTreeNodeFlags::AllowItemOverlap`. `sys::igSetNextWindowContentSize()`. Keep redirection.
  - Rename `sys::ImGuiTextBuffer_append()` helper to `appendf()`.
  - Rename `ImGuiStyleVar::ChildWindowRounding` to `ImGuiStyleVar::ChildRounding`. Keep redirection.
  - Rename `StyleVar::ChildWindowRounding` to `StyleVar::ChildRounding`. Keep redirection.
  - Rename `ImGuiCol::ChildWindowBg` to `ImGuiCol::ChildBg`. Keep redirection.
- Upgrade glium to 0.22.0. This updates winit to 0.16. This changes the way
  HIDPI are calculated. Depending on your needs, you may want to set HIDPI to 1
  by setting the environment variable `WINIT_HIDPI_FACTOR=1` if you use X11.
- `frame()` now takes a single `FrameSize` argument
- Bump minimum Rust version to 1.24
- `set_mouse_down` takes button states by value, not by reference

### Deprecated

- Various imgui-sys things that were deprecated in imgui/cimgui 1.53.1
  - Obsolete `sys::igIsRootWindowFocused()` in favor of using `sys::igIsWindowFocused(ImGuiFocusedFlags::RootWindow)`.
  - Obsolete `sys::igIsRootWindowOrAnyChildFocused()` in favor of using `sys::igIsWindowFocused(ImGuiFocusedFlags::RootAndChildWindows)`.
  - Obsolete `sys::igIsRootWindowOrAnyChildHovered()` in favor of using `sys::igIsWindowHovered(ImGuiHoveredFlags::RootAndChildWindows)`.
  - Obsolete `sys::SetNextWindowContentWidth()` in favor of using - Obsolete `Window::show_borders`. Use `StyleVar` instead.
  - Obsolete `ImGuiCol::ComboBg`. Use `PopupBg` instead.

### Removed

- Features that were removed in imgui/cimgui 1.53.1
  - Remove `anti_aliased: bool` final parameter of `sys::ImDrawList_AddPolyline` and `sys::ImDrawList_AddConvexPolyFilled`.
  - Remove `ImGuiWindowFlags::ShowBorders` window flag. Borders are now fully set up in the ImGuiStyle structure.
- Various imgui-sys things that were deprecated in imgui/cimgui 1.52

## [0.0.18] - 2017-12-23

### Added

- `is_item_hovered`
- `tooltip`
- `tooltip_text`

### Changed

- Upgrade to imgui/cimgui 1.52
- Upgrade to glium 0.19

### Deprecated

- Various imgui-sys things that were deprecated in imgui/cimgui 1.52

### Removed

- Non-namespaced flags
- Various imgui-sys things that were deprecated in imgui/cimgui 1.51
- `Window::bg_alpha`. Push a color change with `with_color_var` instead
- `color_edit3`
- `color_edit4`

## [0.0.17] - 2017-11-07

### Added

- Namespaced flags (e.g. `ImGuiWindowFlags`)
- Color picker widget
- Color button widget
- `imgui_sys` is now re-exported as `sys` in the main create
- `imgui::get_style_color_name`

### Changed

- Upgrade to imgui/cimgui 1.51
- Adapt to latest cimgui API changes
- Bump minimum Rust version to 1.20
- Upgrade to bitflags 1.0
- Various minor ImString/ImStr changes
- `text` now accepts normal Rust strings. ImStr is still needed everywhere else

### Fixed

- Default impl for ImString was incorrect and could cause a crash

### Deprecated

- Non-namespaced flags
- Various imgui-sys things that were deprecated in imgui/cimgui 1.51
- `Window::bg_alpha`. Push a color change with `with_color_var` instead
- `color_edit3`. Use `color_edit` instead
- `color_edit4`. Use `color_edit` instead

### Removed

- ImStr -> str Deref. Use `to_str` instead.

## [0.0.16] - 2017-10-26

### Added

- OpenGL ES 2.0+ support in gfx and glium renderers
- Separate OpenGL 2.0, 3.0, 4.0 shaders in both renderers. This should fix an
  issue with some systems that refuse to use old GLSL shaders with modern
  OpenGL contexts
- `ImGui::add_font_global_scale`
- Support for radio buttons

### Changed

- Upgrade to glium 0.18
- imgui-gfx-renderer `Renderer::init` now requires a `shaders: Shaders` parameter. Please see examples/support_gfx/mod.rs for a shader resolution example
- Bump minimum Rust version to 1.19 because some dependencies require it.

### Fixed

- Glium renderer now uses MinifySamplerFilter::Nearest. This fixes a blurry font issue in some configurations

### Removed

- `ImString::from_string_unchecked`
- `ImString::from_bytes_unchecked`
- `ImStr::from_bytes_unchecked`

## [0.0.15] - 2017-07-23

### Added

- Support for new_line function
- Support for text size calculation
- Support for scoped style customization
- Support for scoped color customization
- Support for child frames
- Unsafe ImString/ImStr creation functions for advanced users:
  - `ImString::from_utf8_unchecked` (renamed from `ImString::from_bytes_unchecked`)
  - `ImString::from_utf8_with_nul_unchecked`
  - `ImStr::from_utf8_with_nul_unchecked` (renamed from `ImStr::from_bytes_unchecked`)

### Changed

- Button, selectable, histogram, plotlines, and progress bar accept size with `Into<ImVec2>`
- `ImString::new` always succeeds and any interior NULs truncate the string. **Breaking change**
- All builder constructor functions (e.g. Window::new) now take `&Ui` reference to tie the lifetime of the builder to it.
- Bumped minimum Rust version to 1.17 because some dependencies require it.
- Upgrade to glium 0.17

### Deprecated

- `ImString::from_string_unchecked` (please use `ImString::new`)
- `ImString::from_bytes_unchecked` (renamed to `ImString::from_utf8_unchecked`)
- `ImStr::from_bytes_unchecked` (renamed to `ImStr::from_utf8_with_nul_unchecked`)

### Fixed

- Histogram, plotlines, progressbar builders were not tied to the `&Ui` lifetime, so it was possible to misuse them.

## [0.0.14] - 2017-06-18

### Added

- ImString owned type for strings
- Experimental support for gfx-rs in imgui-sys
- Experimental renderer for gfx-rs

### Changed

- ImStr is now "a dear imgui -compatible string slice". This change significantly affects how strings are handled.
- Upgrade to imgui/cimgui 1.50
- Upgrade to bitflags 0.9

### Fixed

- String pointer compilation problems on ARM

## [0.0.13] - 2017-04-25

### Changed

- Make the crates publishable again after the Glium renderer separation

## [0.0.12] - 2017-04-25 [YANKED]

### Added

- Support for progress bar
- Support for push/pop item width
- Support for ID stack manipulation (integer values)
- Support for 2-4 -element int sliders
- Support for 2-4 -element float sliders
- `ImVec4::zero()`
- `Into` array and tuple conversions for ImVec2 and ImVec4
- gfx 0.15 support in imgui-sys
- gfx 0.15 renderer implementation

### Changed

- imgui-sys no longer includes glium support by default
- Move Glium renderer to a separate crate

### Removed

- `Window::always_vertical_scollbar` (typo)
- `igPushStyleVavrVec` (typo)
- `ImGuiInputTextFlags::with`
- `ImGuiTreeNodeFlags::with`
- `ImGuiWindowFlags::with`

## [0.0.11] - 2017-02-15

### Added

- `ImVec2::zero()`
- Support for buttons
- Support for closing current popup
- `Window::always_vertical_scrollbar` (fix typo)
- `igPushStyleVarVec` (fix typo)

### Changed

- Upgrade to bitflags 0.8
- Upgrade to glium 0.16
- Replace libc dependency with `std::os::raw`
- Upgrade cimgui to include MinGW compilation fix

### Deprecated

- `Window::always_vertical_scollbar` (typo)
- `igPushStyleVavrVec` (typo)
- `ImGuiInputTextFlags::with`
- `ImGuiTreeNodeFlags::with`
- `ImGuiWindowFlags::with`

## [0.0.10] - 2016-08-09

### Changed

- Upgrade to glium 0.15
- Examples use std::time instead of the deprecated time crate

## [0.0.9] - 2016-07-07

### Added

- Support for columns, combo, listbox
- Support for plothistogram, plotlines
- Support for color edit widgets
- Support for int and float inputs
- Support for int and float array inputs
- Support for popups
- Support for selectable
- Better support for hidpi environments

### Changed

- ImStr::as_ptr is now part of the public API
- Upgrade to bitflags 0.7
- Upgrade to imgui/cimgui 1.49
  - Several imgui_sys structs have changed
  - CollapsingHeader API has changed
  - New window flags are supported

## [0.0.8] - 2016-04-15

### Added

- Add a change log

### Changed

- Upgrade to glium 0.14

## [0.0.7] - 2016-03-26

### Changed

- Upgrade to imgui/cimgui 1.47

### Fixed

- Fix Glium rendering error when more than one texture is used ([issue #17](https://github.com/Gekkio/imgui-rs/issues/17))

## [0.0.6] - 2016-01-12

### Changed

- Relicensed to dual MIT/Apache-2.0
- Upgrade to glium 0.13
- Upgrade to imgui/cimgui 1.46

## [0.0.5] - 2015-11-30

### Changed

- Upgrade to glium 0.12
- Upgrade to libc 0.2

## [0.0.4] - 2015-10-26

### Changed

- Upgrade to glium 0.10
- Lots of other changes

## [0.0.3] - 2015-09-27

### Changed

- Upgrade to glium 0.9
- Lots of other changes

## [0.0.2] - 2015-08-31

### Changed

- Lots of changes

## 0.0.1 - 2015-08-20

### Added

- Initial release with cimgui/imgui 1.44, glium 0.9

[unreleased]: https://github.com/Gekkio/imgui-rs/compare/v0.9.0...HEAD
[0.9.0]: https://github.com/Gekkio/imgui-rs/compare/v0.8.0...v0.9.0
[0.8.0]: https://github.com/Gekkio/imgui-rs/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/Gekkio/imgui-rs/compare/v0.6.1...v0.7.0
[0.6.1]: https://github.com/Gekkio/imgui-rs/compare/v0.6.0...v0.6.1
[0.6.0]: https://github.com/Gekkio/imgui-rs/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/Gekkio/imgui-rs/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/Gekkio/imgui-rs/compare/v0.3.0...v0.4.0
[0.3.1]: https://github.com/Gekkio/imgui-rs/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/Gekkio/imgui-rs/compare/v0.2.0...v0.3.0
[0.2.1]: https://github.com/Gekkio/imgui-rs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/Gekkio/imgui-rs/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/Gekkio/imgui-rs/compare/v0.0.23...v0.1.0
[0.0.23]: https://github.com/Gekkio/imgui-rs/compare/v0.0.22...v0.0.23
[0.0.22]: https://github.com/Gekkio/imgui-rs/compare/v0.0.21...v0.0.22
[0.0.21]: https://github.com/Gekkio/imgui-rs/compare/v0.0.20...v0.0.21
[0.0.20]: https://github.com/Gekkio/imgui-rs/compare/v0.0.19...v0.0.20
[0.0.19]: https://github.com/Gekkio/imgui-rs/compare/v0.0.18...v0.0.19
[0.0.18]: https://github.com/Gekkio/imgui-rs/compare/v0.0.17...v0.0.18
[0.0.17]: https://github.com/Gekkio/imgui-rs/compare/v0.0.16...v0.0.17
[0.0.16]: https://github.com/Gekkio/imgui-rs/compare/v0.0.15...v0.0.16
[0.0.15]: https://github.com/Gekkio/imgui-rs/compare/v0.0.14...v0.0.15
[0.0.14]: https://github.com/Gekkio/imgui-rs/compare/v0.0.13...v0.0.14
[0.0.13]: https://github.com/Gekkio/imgui-rs/compare/v0.0.12...v0.0.13
[0.0.12]: https://github.com/Gekkio/imgui-rs/compare/v0.0.11...v0.0.12
[0.0.11]: https://github.com/Gekkio/imgui-rs/compare/v0.0.10...v0.0.11
[0.0.10]: https://github.com/Gekkio/imgui-rs/compare/v0.0.9...v0.0.10
[0.0.9]: https://github.com/Gekkio/imgui-rs/compare/v0.0.8...v0.0.9
[0.0.8]: https://github.com/Gekkio/imgui-rs/compare/v0.0.7...v0.0.8
[0.0.7]: https://github.com/Gekkio/imgui-rs/compare/v0.0.6...v0.0.7
[0.0.6]: https://github.com/Gekkio/imgui-rs/compare/v0.0.5...v0.0.6
[0.0.5]: https://github.com/Gekkio/imgui-rs/compare/v0.0.4...v0.0.5
[0.0.4]: https://github.com/Gekkio/imgui-rs/compare/v0.0.3...v0.0.4
[0.0.3]: https://github.com/Gekkio/imgui-rs/compare/v0.0.2...v0.0.3
[0.0.2]: https://github.com/Gekkio/imgui-rs/compare/v0.0.1...v0.0.2
