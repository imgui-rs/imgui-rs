# Change Log

## [Unreleased]

### Added

- Support for image buttons: `Ui::image_button`

### Removed

- Various things that were deprecated in imgui-rs 0.0.21 and 0.0.22

## [0.0.22] - 2019-02-05

### Added

- `Ui::with_test_wrap_pos`
- `Ui::get_content_region_max`
- `Ui::get_window_content_region_min`
- `Ui::get_window_content_region_max`

### Changed

- Upgrade to cimgui 1.66.2+ / imgui 1.66b. **This is a very big update, so there
  are a lot of breaking changes**
- Bump minimum Rust version to 1.31 (1.28 required by the glutin crate, and
  1.31 required by the stb_truetype crate)
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
  - *imgui-sys*:
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
  - Rename `sys::igGetItemsLineHeightWithSpacing` to `sys::igGetFrameHeightWithSpacing`.
  Keep redirection.
  - Rename `ImGuiTreeNodeFlags::AllowOverlapMode` to `ImGuiTreeNodeFlags::AllowItemOverlap`.
  `sys::igSetNextWindowContentSize()`. Keep redirection.
  - Rename `sys::ImGuiTextBuffer_append()` helper to `appendf()`.
  - Rename `ImGuiStyleVar::ChildWindowRounding` to `ImGuiStyleVar::ChildRounding`.
  Keep redirection.
  - Rename `StyleVar::ChildWindowRounding` to `StyleVar::ChildRounding`.
  Keep redirection.
  - Rename `ImGuiCol::ChildWindowBg` to `ImGuiCol::ChildBg`.
  Keep redirection.
- Upgrade glium to 0.22.0. This updates winit to 0.16. This changes the way
HIDPI are calculated. Depending on your needs, you may want to set HIDPI to 1
by setting the environment variable `WINIT_HIDPI_FACTOR=1` if you use X11.
- `frame()` now takes a single `FrameSize` argument
- Bump minimum Rust version to 1.24
- `set_mouse_down` takes button states by value, not by reference

### Deprecated

- Various imgui-sys things that were deprecated in imgui/cimgui 1.53.1
  - Obsolete `sys::igIsRootWindowFocused()` in favor of using
  `sys::igIsWindowFocused(ImGuiFocusedFlags::RootWindow)`.
  - Obsolete `sys::igIsRootWindowOrAnyChildFocused()` in favor of using
  `sys::igIsWindowFocused(ImGuiFocusedFlags::RootAndChildWindows)`.
  - Obsolete `sys::igIsRootWindowOrAnyChildHovered()` in favor of using
  `sys::igIsWindowHovered(ImGuiHoveredFlags::RootAndChildWindows)`.
  - Obsolete `sys::SetNextWindowContentWidth()` in favor of using
  - Obsolete `Window::show_borders`. Use `StyleVar` instead.
  - Obsolete `ImGuiCol::ComboBg`. Use `PopupBg` instead.

### Removed

- Features that were removed in imgui/cimgui 1.53.1
  - Remove `anti_aliased: bool` final parameter of `sys::ImDrawList_AddPolyline`
  and `sys::ImDrawList_AddConvexPolyFilled`.
  - Remove `ImGuiWindowFlags::ShowBorders` window flag. Borders are now fully
  set up in the ImGuiStyle structure.
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
- imgui-gfx-renderer `Renderer::init` now requires a `shaders: Shaders`
  parameter. Please see examples/support_gfx/mod.rs for a shader resolution
  example
- Bump minimum Rust version to 1.19 because some dependencies require it.

### Fixed

- Glium renderer now uses MinifySamplerFilter::Nearest. This fixes a blurry font
  issue in some configurations

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
  + `ImString::from_utf8_unchecked` (renamed from `ImString::from_bytes_unchecked`)
  + `ImString::from_utf8_with_nul_unchecked`)
  + `ImStr::from_utf8_with_nul_unchecked` (renamed from `ImStr::from_bytes_unchecked`)

### Changed

- Button, selectable, histogram, plotlines, and progress bar accept size with `Into<ImVec2>`
- `ImString::new` always succeeds and any interior NULs truncate the string. **Breaking change**
- All builder constructor functions (e.g. Window::new) now take `&Ui` reference
  to tie the lifetime of the builder to it.
- Bumped minimum Rust version to 1.17 because some dependencies require it.
- Upgrade to glium 0.17

### Deprecated

- `ImString::from_string_unchecked` (please use `ImString::new`)
- `ImString::from_bytes_unchecked` (renamed to `ImString::from_utf8_unchecked`)
- `ImStr::from_bytes_unchecked` (renamed to `ImStr::from_utf8_with_nul_unchecked`)

### Fixed

- Histogram, plotlines, progressbar builders were not tied to the `&Ui`
  lifetime, so it was possible to misuse them.

## [0.0.14] - 2017-06-18

### Added

- ImString owned type for strings
- Experimental support for gfx-rs in imgui-sys
- Experimental renderer for gfx-rs

### Changed

- ImStr is now "a dear imgui -compatible string slice". This change
  significantly affects how strings are handled.
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
    * Several imgui_sys structs have changed
    * CollapsingHeader API has changed
    * New window flags are supported

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

[Unreleased]: https://github.com/Gekkio/imgui-rs/compare/v0.0.22...HEAD
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
