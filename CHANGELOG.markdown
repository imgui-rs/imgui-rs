# Change Log

## [Unreleased]

### Added

- ImString owned type for strings

### Changed

- ImStr is now "a dear imgui -compatible string slice". This change
  significantly affects how strings are handled.

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

[Unreleased]: https://github.com/Gekkio/imgui-rs/compare/v0.0.13...HEAD
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
