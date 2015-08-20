#[macro_use]
extern crate glium;
#[macro_use]
extern crate imgui;
extern crate time;

use imgui::*;

use self::support::Support;

mod support;

struct State {
    clear_color: (f32, f32, f32, f32),
    show_app_metrics: bool,
    show_app_main_menu_bar: bool,
    show_app_console: bool,
    show_app_layout: bool,
    show_app_long_text: bool,
    show_app_auto_resize: bool,
    show_app_fixed_overlay: bool,
    show_app_custom_rendering: bool,
    show_app_manipulating_window_title: bool,
    show_app_about: bool,
    no_titlebar: bool,
    no_border: bool,
    no_resize: bool,
    no_move: bool,
    no_scrollbar: bool,
    no_collapse: bool,
    no_menu: bool,
    bg_alpha: f32,
    file_menu: FileMenuState
}

impl Default for State {
    fn default() -> Self {
        State {
            clear_color: (114.0 / 255.0, 144.0 / 255.0, 154.0 / 255.0, 1.0),
            show_app_metrics: false,
            show_app_main_menu_bar: false,
            show_app_console: false,
            show_app_layout: false,
            show_app_long_text: false,
            show_app_auto_resize: false,
            show_app_fixed_overlay: false,
            show_app_custom_rendering: false,
            show_app_manipulating_window_title: false,
            show_app_about: false,
            no_titlebar: false,
            no_border: false,
            no_resize: false,
            no_move: false,
            no_scrollbar: false,
            no_collapse: false,
            no_menu: false,
            bg_alpha: 0.65,
            file_menu: Default::default()
        }
    }
}

struct FileMenuState {
    enabled: bool
}

impl Default for FileMenuState {
    fn default() -> Self {
        FileMenuState {
            enabled: true
        }
    }
}

fn main() {
    let mut state = State {
        .. Default::default()
    };
    let mut support = Support::init();

    loop {
        let active = support.render(state.clear_color, |frame| {
            show_test_window(frame, &mut state)
        });
        if !active { break }
    }
}

fn show_user_guide<'a>(frame: &Frame<'a>) {
    frame.bullet_text(im_str!("Double-click on title bar to collapse window."));
    frame.bullet_text(im_str!("Click and drag on lower right corner to resize window."));
    frame.bullet_text(im_str!("Click and drag on any empty space to move window."));
    frame.bullet_text(im_str!("Mouse Wheel to scroll."));
    frame.bullet_text(im_str!("TAB/SHIFT+TAB to cycle through keyboard editable fields."));
    frame.bullet_text(im_str!("CTRL+Click on a slider or drag box to input text."));
    frame.bullet_text(im_str!(
"While editing text:
- Hold SHIFT or use mouse to select text
- CTRL+Left/Right to word jump
- CTRL+A or double-click to select all
- CTRL+X,CTRL+C,CTRL+V clipboard
- CTRL+Z,CTRL+Y undo/redo
- ESCAPE to revert
- You can apply arithmetic operators +,*,/ on numerical values.
  Use +- to subtract."));
}

fn show_test_window<'a>(frame: &Frame<'a>, state: &mut State) -> bool {
    if state.show_app_main_menu_bar { show_example_app_main_menu_bar(frame, state) }
    if state.show_app_fixed_overlay {
        state.show_app_fixed_overlay = show_example_app_fixed_overlay(frame);
    }
    if state.show_app_about {
        state.show_app_about = frame.window()
            .name(im_str!("About ImGui"))
            .always_auto_resize(true)
            .closable(true)
            .build(|| {
                frame.text(ImStr::from_str(&format!("ImGui {}", imgui::get_version())));
                frame.separator();
                frame.text(im_str!("By Omar Cornut and all github contributors."));
                frame.text(im_str!("ImGui is licensed under the MIT License, see LICENSE for more information."));
                show_user_guide(frame);
            })
    }

    frame.window().name(im_str!("ImGui Demo"))
        .title_bar(!state.no_titlebar)
        .show_borders(!state.no_border)
        .resizable(!state.no_resize)
        .movable(!state.no_move)
        .scroll_bar(!state.no_scrollbar)
        .collapsible(!state.no_collapse)
        .menu_bar(!state.no_menu)
        .bg_alpha(state.bg_alpha)
        .size((550.0, 680.0), ImGuiSetCond_FirstUseEver)
        .closable(true)
        .build(|| {
            frame.text(im_str!("ImGui says hello."));
            frame.menu_bar(|| {
                frame.menu(im_str!("Menu")).build(|| {
                    show_example_menu_file(frame, &mut state.file_menu);
                });
                frame.menu(im_str!("Examples")).build(|| {
                    if frame.menu_item(im_str!("Main menu bar")).build() {
                        state.show_app_main_menu_bar = !state.show_app_main_menu_bar;
                    }
                    if frame.menu_item(im_str!("Console")).build() {
                        state.show_app_console = !state.show_app_console;
                    }
                    if frame.menu_item(im_str!("Simple layout")).build() {
                        state.show_app_layout = !state.show_app_layout;
                    }
                    if frame.menu_item(im_str!("Long text display")).build() {
                        state.show_app_long_text = !state.show_app_long_text;
                    }
                    if frame.menu_item(im_str!("Auto-resizing window")).build() {
                        state.show_app_auto_resize = !state.show_app_auto_resize;
                    }
                    if frame.menu_item(im_str!("Simple overlay")).build() {
                        state.show_app_fixed_overlay = !state.show_app_fixed_overlay;
                    }
                    if frame.menu_item(im_str!("Manipulating window title")).build() {
                        state.show_app_manipulating_window_title =
                            !state.show_app_manipulating_window_title;
                    }
                    if frame.menu_item(im_str!("Custom rendering")).build() {
                        state.show_app_custom_rendering = !state.show_app_custom_rendering;
                    }
                });
                frame.menu(im_str!("Help")).build(|| {
                    if frame.menu_item(im_str!("Metrics")).build() {
                        state.show_app_metrics = !state.show_app_metrics;
                    }
                    if frame.menu_item(im_str!("About ImGui")).build() {
                        state.show_app_about = !state.show_app_about;
                    }
                });
            });
            frame.spacing();
            if frame.collapsing_header(im_str!("Help")).build() {
                frame.text_wrapped(im_str!("This window is being created by the show_test_window() function. Please refer to the code for programming reference.\n\nUser Guide:"));
                show_user_guide(frame);
            }
        })
}

fn show_example_app_main_menu_bar<'a>(frame: &Frame<'a>, state: &mut State) {
    frame.main_menu_bar(|| {
        frame.menu(im_str!("File")).build(|| {
            show_example_menu_file(frame, &mut state.file_menu);
        });
        frame.menu(im_str!("Edit")).build(|| {
            if frame.menu_item(im_str!("Undo")).shortcut(im_str!("CTRL+Z")).build() { }
            if frame.menu_item(im_str!("Redo"))
                .shortcut(im_str!("CTRL+Y")).enabled(false).build() { }
            frame.separator();
            if frame.menu_item(im_str!("Cut")).shortcut(im_str!("CTRL+X")).build() { }
            if frame.menu_item(im_str!("Copy")).shortcut(im_str!("CTRL+C")).build() { }
            if frame.menu_item(im_str!("Paste")).shortcut(im_str!("CTRL+V")).build() { }
        });
    });
}

fn show_example_menu_file<'a>(frame: &Frame<'a>, state: &mut FileMenuState) {
    frame.menu_item(im_str!("(dummy menu)")).enabled(false).build();
    if frame.menu_item(im_str!("New")).build() { }
    if frame.menu_item(im_str!("Open")).shortcut(im_str!("Ctrl+O")).build() { }
    frame.menu(im_str!("Open Recent")).build(|| {
        frame.menu_item(im_str!("fish_hat.c")).build();
        frame.menu_item(im_str!("fish_hat.inl")).build();
        frame.menu_item(im_str!("fish_hat.h")).build();
        frame.menu(im_str!("More..")).build(|| {
            frame.menu_item(im_str!("Hello"));
            frame.menu_item(im_str!("Sailor"));
            frame.menu(im_str!("Recurse..")).build(|| {
                show_example_menu_file(frame, state);
            });
        });
    });
    if frame.menu_item(im_str!("Save")).shortcut(im_str!("Ctrl+S")).build() { }
    if frame.menu_item(im_str!("Save As..")).build() { }
    frame.separator();
    frame.menu(im_str!("Options")).build(|| {
        if frame.menu_item(im_str!("Enabled")).selected(state.enabled).build() {
            state.enabled = !state.enabled;
        }
        // TODO
    });
    frame.menu(im_str!("Colors")).build(|| {
        // TODO
    });
    frame.menu(im_str!("Disabled")).enabled(false).build(|| {
        unreachable!();
    });
    if frame.menu_item(im_str!("Checked")).selected(true).build() { }
    if frame.menu_item(im_str!("Quit")).shortcut(im_str!("Alt+F4")).build() { }
}

fn show_example_app_fixed_overlay<'a>(frame: &Frame<'a>) -> bool {
    frame.window()
        .name(im_str!("Example: Fixed Overlay"))
        .closable(true)
        .bg_alpha(0.3)
        .title_bar(false)
        .resizable(false)
        .movable(false)
        .save_settings(false)
        .build(|| {
            frame.text(im_str!("Simple overlay\non the top-left side of the screen."));
            frame.separator();
            let mouse_pos = frame.imgui().mouse_pos();
            frame.text(im_str!("Mouse Position: ({:.1},{:.1})", mouse_pos.0, mouse_pos.1));
        })
}
