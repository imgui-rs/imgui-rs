#[macro_use]
extern crate glium;
extern crate imgui;
extern crate time;

use imgui::*;
use std::iter::repeat;

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
    wrap_width: f32,
    buf: String,
    auto_resize_state: AutoResizeState,
    file_menu: FileMenuState
}

impl Default for State {
    fn default() -> Self {
        let mut buf = "日本語".to_owned();
        buf.extend(repeat('\0').take(32));
        buf.truncate(32);
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
            no_border: true,
            no_resize: false,
            no_move: false,
            no_scrollbar: false,
            no_collapse: false,
            no_menu: false,
            bg_alpha: 0.65,
            wrap_width: 200.0,
            buf: buf,
            auto_resize_state: Default::default(),
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

struct AutoResizeState {
    lines: i32
}

impl Default for AutoResizeState {
    fn default() -> Self {
        AutoResizeState {
            lines: 10
        }
    }
}

fn main() {
    let mut state = State::default();
    let mut support = Support::init();
    let mut opened = true;

    loop {
        support.render(state.clear_color, |ui| {
            show_test_window(ui, &mut state, &mut opened);
        });
        let active = support.update_events();
        if !active || !opened { break }
    }
}

fn show_user_guide<'a>(ui: &Ui<'a>) {
    ui.bullet_text("Double-click on title bar to collapse window.");
    ui.bullet_text("Click and drag on lower right corner to resize window.");
    ui.bullet_text("Click and drag on any empty space to move window.");
    ui.bullet_text("Mouse Wheel to scroll.");
    ui.bullet_text("TAB/SHIFT+TAB to cycle through keyboard editable fields.");
    ui.bullet_text("CTRL+Click on a slider or drag box to input text.");
    ui.bullet_text(
"While editing text:
- Hold SHIFT or use mouse to select text
- CTRL+Left/Right to word jump
- CTRL+A or double-click to select all
- CTRL+X,CTRL+C,CTRL+V clipboard
- CTRL+Z,CTRL+Y undo/redo
- ESCAPE to revert
- You can apply arithmetic operators +,*,/ on numerical values.
  Use +- to subtract.");
}

fn show_test_window<'a>(ui: &Ui<'a>, state: &mut State, opened: &mut bool) {
    if state.show_app_metrics {
        ui.show_metrics_window(&mut state.show_app_metrics);
    }
    if state.show_app_main_menu_bar { show_example_app_main_menu_bar(ui, state) }
    if state.show_app_auto_resize {
        show_example_app_auto_resize(ui, &mut state.auto_resize_state, &mut state.show_app_auto_resize);
    }
    if state.show_app_fixed_overlay {
        show_example_app_fixed_overlay(ui, &mut state.show_app_fixed_overlay);
    }
    if state.show_app_manipulating_window_title {
        show_example_app_manipulating_window_title(ui);
    }
    if state.show_app_about {
        ui.window("About ImGui")
            .always_auto_resize(true)
            .opened(&mut state.show_app_about)
            .build(|| {
                ui.text(format!("ImGui {}", imgui::get_version()));
                ui.separator();
                ui.text("By Omar Cornut and all github contributors.");
                ui.text("ImGui is licensed under the MIT License, see LICENSE for more information.");
            })
    }

    ui.window("ImGui Demo")
        .title_bar(!state.no_titlebar)
        .show_borders(!state.no_border)
        .resizable(!state.no_resize)
        .movable(!state.no_move)
        .scroll_bar(!state.no_scrollbar)
        .collapsible(!state.no_collapse)
        .menu_bar(!state.no_menu)
        .bg_alpha(state.bg_alpha)
        .size((550.0, 680.0), ImGuiSetCond_FirstUseEver)
        .opened(opened)
        .build(|| {
            ui.text("ImGui says hello.");
            ui.menu_bar(|| {
                ui.menu("Menu").build(|| {
                    show_example_menu_file(ui, &mut state.file_menu);
                });
                ui.menu("Examples").build(|| {
                    ui.menu_item("Main menu bar")
                        .selected(&mut state.show_app_main_menu_bar).build();
                    ui.menu_item("Console")
                        .selected(&mut state.show_app_console).build();
                    ui.menu_item("Simple layout")
                        .selected(&mut state.show_app_layout).build();
                    ui.menu_item("Long text display")
                        .selected(&mut state.show_app_long_text).build();
                    ui.menu_item("Auto-resizing window")
                        .selected(&mut state.show_app_auto_resize).build();
                    ui.menu_item("Simple overlay")
                        .selected(&mut state.show_app_fixed_overlay).build();
                    ui.menu_item("Manipulating window title")
                        .selected(&mut state.show_app_manipulating_window_title).build();
                    ui.menu_item("Custom rendering")
                        .selected(&mut state.show_app_custom_rendering).build();
                });
                ui.menu("Help").build(|| {
                    ui.menu_item("Metrics")
                        .selected(&mut state.show_app_metrics).build();
                    ui.menu_item("About ImGui")
                        .selected(&mut state.show_app_about).build();
                });
            });
            ui.spacing();
            if ui.collapsing_header("Help").build() {
                ui.text_wrapped("This window is being created by the show_test_window() function. Please refer to the code for programming reference.\n\nUser Guide:");
                show_user_guide(ui);
            }

            if ui.collapsing_header("Window options").build() {
                ui.checkbox("no titlebar", &mut state.no_titlebar);
                ui.same_line(150.0);
                ui.checkbox("no border", &mut state.no_border);
                ui.same_line(300.0);
                ui.checkbox("no resize", &mut state.no_resize);
                ui.checkbox("no move", &mut state.no_move);
                ui.same_line(150.0);
                ui.checkbox("no scrollbar", &mut state.no_scrollbar);
                ui.same_line(300.0);
                ui.checkbox("no collapse", &mut state.no_collapse);
                ui.checkbox("no menu", &mut state.no_menu);
                ui.slider_f32("bg alpha", &mut state.bg_alpha, 0.0, 1.0).build();

                ui.tree_node("Style").build(|| {
                    // TODO: Reimplement style editor
                    ui.show_default_style_editor();
                });
                ui.tree_node("Fonts")
                    .label(format!("Fonts ({})", "TODO"))
                    .build(|| {
                    ui.text_wrapped("Tip: Load fonts with io.Fonts->AddFontFromFileTTF().");
                    ui.tree_node("Atlas texture").build(|| {
                        // TODO
                    });
                });
            }
            if ui.collapsing_header("Widgets").build() {
                ui.tree_node("Tree").build(|| {
                    for i in 0..5 {
                        ui.tree_node(format!("Child {}", i)).build(|| {
                            ui.text("blah blah");
                            ui.same_line(0.0);
                            if ui.small_button("print") {
                                println!("Child {} pressed", i);
                            }
                        });
                    }
                });
                ui.tree_node("Bullets").build(|| {
                    ui.bullet_text("Bullet point 1");
                    ui.bullet_text("Bullet point 2\nOn multiple lines");
                    ui.bullet();
                    ui.text("Bullet point 3 (two calls)");

                    ui.bullet();
                    ui.button("Button", (0.0, 0.0));

                    ui.bullet();
                    ui.small_button("Small Button");

                    ui.bullet();
                    ui.progress_bar(0.5, (-1.0, 0.0), "Progress");
                });
                ui.tree_node("Colored text").build(|| {
                    ui.text_colored((1.0, 0.0, 1.0, 1.0), "Pink");
                    ui.text_colored((1.0, 1.0, 0.0, 1.0), "Yellow");
                    ui.text_disabled("Disabled");
                });
                ui.tree_node("Word Wrapping").build(|| {
                    ui.text_wrapped(
                            "This text should automatically wrap on the edge of the window.\
                            The current implementation for text wrapping follows simple rules\
                            suitable for English and possibly other languages.");
                    ui.spacing();

                    ui.slider_f32("Wrap width", &mut state.wrap_width, -20.0, 600.0)
                        .display_format("%.0f")
                        .build();

                    ui.text("Test paragraph 1:");
                    // TODO

                    ui.text("Test paragraph 2:");
                    // TODO
                });
                ui.tree_node("UTF-8 Text").build(|| {
                    ui.text_wrapped(
                            "CJK text will only appear if the font was loaded with the\
                            appropriate CJK character ranges. Call io.Font->LoadFromFileTTF()\
                            manually to load extra character ranges.");

                    ui.text("Hiragana: かきくけこ (kakikukeko)");
                    ui.text("Kanjis: 日本語 (nihongo)");
                    ui.input_text("UTF-8 input", &mut state.buf).build();
                });
            }
        })
}

fn show_example_app_main_menu_bar<'a>(ui: &Ui<'a>, state: &mut State) {
    ui.main_menu_bar(|| {
        ui.menu("File").build(|| {
            show_example_menu_file(ui, &mut state.file_menu);
        });
        ui.menu("Edit").build(|| {
            ui.menu_item("Undo").shortcut("CTRL+").build();
            ui.menu_item("Redo")
                .shortcut("CTRL+Y").enabled(false).build();
            ui.separator();
            ui.menu_item("Cut").shortcut("CTRL+X").build();
            ui.menu_item("Copy").shortcut("CTRL+C").build();
            ui.menu_item("Paste").shortcut("CTRL+V").build();
        });
    });
}

fn show_example_menu_file<'a>(ui: &Ui<'a>, state: &mut FileMenuState) {
    ui.menu_item("(dummy menu)").enabled(false).build();
    ui.menu_item("New").build();
    ui.menu_item("Open").shortcut("Ctrl+O").build();
    ui.menu("Open Recent").build(|| {
        ui.menu_item("fish_hat.c").build();
        ui.menu_item("fish_hat.inl").build();
        ui.menu_item("fish_hat.h").build();
        ui.menu("More..").build(|| {
            ui.menu_item("Hello").build();
            ui.menu_item("Sailor").build();
            ui.menu("Recurse..").build(|| {
                show_example_menu_file(ui, state);
            });
        });
    });
    ui.menu_item("Save").shortcut("Ctrl+S").build();
    ui.menu_item("Save As..").build();
    ui.separator();
    ui.menu("Options").build(|| {
        ui.menu_item("Enabled").selected(&mut state.enabled).build();
        // TODO
    });
    ui.menu("Colors").build(|| {
        // TODO
    });
    ui.menu("Disabled").enabled(false).build(|| {
        unreachable!();
    });
    let mut checked = true;
    ui.menu_item("Checked").selected(&mut checked).build();
    ui.menu_item("Quit").shortcut("Alt+F4").build();
}

fn show_example_app_auto_resize<'a>(ui: &Ui<'a>, state: &mut AutoResizeState, opened: &mut bool) {
    ui.window("Example: Auto-resizing window")
        .opened(opened)
        .always_auto_resize(true)
        .build(|| {
            ui.text("Window will resize every-ui to the size of its content.
Note that you probably don't want to query the window size to
output your content because that would create a feedback loop.");
            ui.slider_i32("Number of lines", &mut state.lines, 1, 20).build();
            for i in 0 .. state.lines {
                ui.text(format!("{:2$}This is line {}", "", i, i as usize * 4));
            }
        })
}

fn show_example_app_fixed_overlay<'a>(ui: &Ui<'a>, opened: &mut bool) {
    ui.window("Example: Fixed Overlay")
        .opened(opened)
        .bg_alpha(0.3)
        .title_bar(false)
        .resizable(false)
        .movable(false)
        .save_settings(false)
        .build(|| {
            ui.text("Simple overlay\non the top-left side of the screen.");
            ui.separator();
            let mouse_pos = ui.imgui().mouse_pos();
            ui.text(format!("Mouse Position: ({:.1},{:.1})", mouse_pos.0, mouse_pos.1));
        })
}

fn show_example_app_manipulating_window_title<'a>(ui: &Ui<'a>) {
    ui.window("Same title as another window##1")
        .position((100.0, 100.0), ImGuiSetCond_FirstUseEver)
        .build(|| {
            ui.text("This is window 1.
My title is the same as window 2, but my identifier is unique.");
        });
    ui.window("Same title as another window##2")
        .position((100.0, 200.0), ImGuiSetCond_FirstUseEver)
        .build(|| {
            ui.text("This is window 2.
My title is the same as window 1, but my identifier is unique.");
        });
    let chars = ['|', '/', '-', '\\'];
    let ch_idx = (ui.imgui().get_time() / 0.25) as usize & 3;
    let num = ui.imgui().get_frame_count(); // The C++ version uses rand() here
    let title = format!("Animated title {} {}###AnimatedTitle", chars[ch_idx], num);
    ui.window(title)
        .position((100.0, 300.0), ImGuiSetCond_FirstUseEver)
        .build(|| {
            ui.text("This window has a changing title");
        });
}
