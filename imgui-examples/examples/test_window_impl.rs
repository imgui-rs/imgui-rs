extern crate glium;
#[macro_use]
extern crate imgui;
extern crate imgui_glium_renderer;

use imgui::*;

mod support;

struct State {
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
    buf: ImString,
    item: i32,
    item2: i32,
    text: ImString,
    i0: i32,
    f0: f32,
    vec2f: [f32; 2],
    vec3f: [f32; 3],
    vec2i: [i32; 2],
    vec3i: [i32; 3],
    col1: [f32; 3],
    col2: [f32; 4],
    selected_fish: Option<usize>,
    auto_resize_state: AutoResizeState,
    file_menu: FileMenuState,
}

impl Default for State {
    fn default() -> Self {
        let mut buf = ImString::with_capacity(32);
        buf.push_str("日本語");
        let mut text = ImString::with_capacity(128);
        text.push_str("Hello, world!");
        State {
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
            item: 0,
            item2: 0,
            text: text,
            i0: 123,
            f0: 0.001,
            vec2f: [0.10, 0.20],
            vec3f: [0.10, 0.20, 0.30],
            vec2i: [10, 20],
            vec3i: [10, 20, 30],
            col1: [1.0, 0.0, 0.2],
            col2: [0.4, 0.7, 0.0, 0.5],
            selected_fish: None,
            auto_resize_state: Default::default(),
            file_menu: Default::default(),
        }
    }
}

struct FileMenuState {
    enabled: bool,
}

impl Default for FileMenuState {
    fn default() -> Self { FileMenuState { enabled: true } }
}

struct AutoResizeState {
    lines: i32,
}

impl Default for AutoResizeState {
    fn default() -> Self { AutoResizeState { lines: 10 } }
}

const CLEAR_COLOR: [f32; 4] = [114.0 / 255.0, 144.0 / 255.0, 154.0 / 255.0, 1.0];

fn main() {
    let mut state = State::default();

    support::run("test_window.rs".to_owned(), CLEAR_COLOR, |ui| {
        let mut open = true;
        show_test_window(ui, &mut state, &mut open);
        open
    });
}

fn show_user_guide<'a>(ui: &Ui<'a>) {
    ui.bullet_text(im_str!("Double-click on title bar to collapse window."));
    ui.bullet_text(im_str!("Click and drag on lower right corner to resize window."));
    ui.bullet_text(im_str!("Click and drag on any empty space to move window."));
    ui.bullet_text(im_str!("Mouse Wheel to scroll."));
    ui.bullet_text(im_str!("TAB/SHIFT+TAB to cycle through keyboard editable fields."));
    ui.bullet_text(im_str!("CTRL+Click on a slider or drag box to input text."));
    ui.bullet_text(im_str!("While editing text:
- Hold SHIFT or use mouse to select text
- \
                            CTRL+Left/Right to word jump
- CTRL+A or double-click to select all
\
                            - CTRL+X,CTRL+C,CTRL+V clipboard
- CTRL+Z,CTRL+Y undo/redo
- ESCAPE \
                            to revert
- You can apply arithmetic operators +,*,/ on numerical \
                            values.
  Use +- to subtract."));
}

fn show_test_window<'a>(ui: &Ui<'a>, state: &mut State, opened: &mut bool) {
    if state.show_app_metrics {
        ui.show_metrics_window(&mut state.show_app_metrics);
    }
    if state.show_app_main_menu_bar {
        show_example_app_main_menu_bar(ui, state)
    }
    if state.show_app_auto_resize {
        show_example_app_auto_resize(ui,
                                     &mut state.auto_resize_state,
                                     &mut state.show_app_auto_resize);
    }
    if state.show_app_fixed_overlay {
        show_example_app_fixed_overlay(ui, &mut state.show_app_fixed_overlay);
    }
    if state.show_app_manipulating_window_title {
        show_example_app_manipulating_window_title(ui);
    }
    if state.show_app_about {
        ui.window(im_str!("About ImGui The forced recompilation !"))
            .always_auto_resize(true)
            .opened(&mut state.show_app_about)
            .build(|| {
                ui.text(im_str!("ImGui {}", imgui::get_version()));
                ui.separator();
                ui.text(im_str!("By Omar Cornut and all github contributors."));
                ui.text(im_str!("ImGui is licensed under the MIT License, see LICENSE for more \
                                 information."));
            })
    }

    ui.window(im_str!("ImGui Demo"))
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
            ui.text(im_str!("ImGui says hello."));
            ui.menu_bar(|| {
                ui.menu(im_str!("Menu"))
                    .build(|| { show_example_menu_file(ui, &mut state.file_menu); });
                ui.menu(im_str!("Examples"))
                    .build(|| {
                        ui.menu_item(im_str!("Main menu bar"))
                            .selected(&mut state.show_app_main_menu_bar)
                            .build();
                        ui.menu_item(im_str!("Console"))
                            .selected(&mut state.show_app_console)
                            .build();
                        ui.menu_item(im_str!("Simple layout"))
                            .selected(&mut state.show_app_layout)
                            .build();
                        ui.menu_item(im_str!("Long text display"))
                            .selected(&mut state.show_app_long_text)
                            .build();
                        ui.menu_item(im_str!("Auto-resizing window"))
                            .selected(&mut state.show_app_auto_resize)
                            .build();
                        ui.menu_item(im_str!("Simple overlay"))
                            .selected(&mut state.show_app_fixed_overlay)
                            .build();
                        ui.menu_item(im_str!("Manipulating window title"))
                            .selected(&mut state.show_app_manipulating_window_title)
                            .build();
                        ui.menu_item(im_str!("Custom rendering"))
                            .selected(&mut state.show_app_custom_rendering)
                            .build();
                    });
                ui.menu(im_str!("Help"))
                    .build(|| {
                               ui.menu_item(im_str!("Metrics"))
                                   .selected(&mut state.show_app_metrics)
                                   .build();
                               ui.menu_item(im_str!("About ImGui"))
                                   .selected(&mut state.show_app_about)
                                   .build();
                           });
            });
            ui.spacing();
            if ui.collapsing_header(im_str!("Help")).build() {
                ui.text_wrapped(im_str!("This window is being created by the show_test_window() \
                                         function. Please refer to the code for programming \
                                         reference.\n\nUser Guide:"));
                show_user_guide(ui);
            }

            if ui.collapsing_header(im_str!("Window options")).build() {
                ui.checkbox(im_str!("no titlebar"), &mut state.no_titlebar);
                ui.same_line(150.0);
                ui.checkbox(im_str!("no border"), &mut state.no_border);
                ui.same_line(300.0);
                ui.checkbox(im_str!("no resize"), &mut state.no_resize);
                ui.checkbox(im_str!("no move"), &mut state.no_move);
                ui.same_line(150.0);
                ui.checkbox(im_str!("no scrollbar"), &mut state.no_scrollbar);
                ui.same_line(300.0);
                ui.checkbox(im_str!("no collapse"), &mut state.no_collapse);
                ui.checkbox(im_str!("no menu"), &mut state.no_menu);
                ui.slider_float(im_str!("bg alpha"), &mut state.bg_alpha, 0.0, 1.0)
                    .build();

                ui.tree_node(im_str!("Style"))
                    .build(|| {
                               // TODO: Reimplement style editor
                               ui.show_default_style_editor();
                           });
                ui.tree_node(im_str!("Fonts"))
                    .label(im_str!("Fonts ({})", "TODO"))
                    .build(|| {
                               ui.text_wrapped(im_str!("Tip: Load fonts with \
                                                 io.Fonts->AddFontFromFileTTF()."));
                               ui.tree_node(im_str!("Atlas texture"))
                                   .build(|| {
                                              // TODO
                                          });
                           });
            }
            if ui.collapsing_header(im_str!("Widgets")).build() {
                ui.tree_node(im_str!("Tree"))
                    .build(|| for i in 0..5 {
                               ui.tree_node(im_str!("Child {}", i))
                                   .build(|| {
                                              ui.text(im_str!("blah blah"));
                                              ui.same_line(0.0);
                                              if ui.small_button(im_str!("print")) {
                                                  println!("Child {} pressed", i);
                                              }
                                          });
                           });
                ui.tree_node(im_str!("Bullets"))
                    .build(|| {
                        ui.bullet_text(im_str!("Bullet point 1"));
                        ui.bullet_text(im_str!("Bullet point 2\nOn multiple lines"));
                        ui.bullet();
                        ui.text(im_str!("Bullet point 3 (two calls)"));

                        ui.bullet();
                        ui.small_button(im_str!("Button"));
                    });
                ui.tree_node(im_str!("Colored text"))
                    .build(|| {
                               ui.text_colored((1.0, 0.0, 1.0, 1.0), im_str!("Pink"));
                               ui.text_colored((1.0, 1.0, 0.0, 1.0), im_str!("Yellow"));
                               ui.text_disabled(im_str!("Disabled"));
                           });
                ui.tree_node(im_str!("Word Wrapping")).build(|| {
                    ui.text_wrapped(im_str!("This text should automatically wrap on the edge of \
                                             the window.The current implementation for text \
                                             wrapping follows simple rulessuitable for English \
                                             and possibly other languages."));
                    ui.spacing();

                    ui.slider_float(im_str!("Wrap width"), &mut state.wrap_width, -20.0, 600.0)
                        .display_format(im_str!("%.0f"))
                        .build();

                    ui.text(im_str!("Test paragraph 1:"));
                    // TODO

                    ui.text(im_str!("Test paragraph 2:"));
                    // TODO
                });
                ui.tree_node(im_str!("UTF-8 Text"))
                    .build(|| {
                        ui.text_wrapped(im_str!("CJK text will only appear if the font was loaded \
                                             with theappropriate CJK character ranges. Call \
                                             io.Font->LoadFromFileTTF()manually to load extra \
                                             character ranges."));

                        ui.text(im_str!("Hiragana: かきくけこ (kakikukeko)"));
                        ui.text(im_str!("Kanjis: 日本語 (nihongo)"));
                        ui.input_text(im_str!("UTF-8 input"), &mut state.buf)
                            .build();
                    });

                ui.separator();
                ui.label_text(im_str!("label"), im_str!("Value"));
                ui.combo(im_str!("combo"),
                         &mut state.item,
                         &[im_str!("aaaa"),
                           im_str!("bbbb"),
                           im_str!("cccc"),
                           im_str!("dddd"),
                           im_str!("eeee")],
                         -1);
                let items = [im_str!("AAAA"),
                             im_str!("BBBB"),
                             im_str!("CCCC"),
                             im_str!("DDDD"),
                             im_str!("EEEE"),
                             im_str!("FFFF"),
                             im_str!("GGGG"),
                             im_str!("HHHH"),
                             im_str!("IIII"),
                             im_str!("JJJJ"),
                             im_str!("KKKK")];
                ui.combo(im_str!("combo scroll"), &mut state.item2, &items, -1);
                ui.input_text(im_str!("input text"), &mut state.text)
                    .build();
                ui.input_int(im_str!("input int"), &mut state.i0).build();
                ui.input_float(im_str!("input float"), &mut state.f0)
                    .step(0.01)
                    .step_fast(1.0)
                    .build();
                ui.input_float3(im_str!("input float3"), &mut state.vec3f)
                    .build();
                ui.color_edit3(im_str!("color 1"), &mut state.col1)
                    .build();
                ui.color_edit4(im_str!("color 2"), &mut state.col2)
                    .build();

                ui.tree_node(im_str!("Multi-component Widgets"))
                    .build(|| {
                        ui.input_float2(im_str!("input float2"), &mut state.vec2f)
                            .build();
                        ui.input_int2(im_str!("input int2"), &mut state.vec2i)
                            .build();
                        ui.spacing();

                        ui.input_float3(im_str!("input float3"), &mut state.vec3f)
                            .build();
                        ui.input_int3(im_str!("input int3"), &mut state.vec3i)
                            .build();
                        ui.spacing();
                    });
            }
            if ui.collapsing_header(im_str!("Popups & Modal windows"))
                   .build() {
                ui.tree_node(im_str!("Popups"))
                    .build(|| {
                        ui.text_wrapped(im_str!("When a popup is active, it inhibits interacting \
                                             with windows that are behind the popup. Clicking \
                                             outside the popup closes it."));
                        let names = [im_str!("Bream"),
                                     im_str!("Haddock"),
                                     im_str!("Mackerel"),
                                     im_str!("Pollock"),
                                     im_str!("Tilefish")];
                        if ui.small_button(im_str!("Select..")) {
                            ui.open_popup(im_str!("select"));
                        }
                        ui.same_line(0.0);
                        ui.text(match state.selected_fish {
                                    Some(index) => names[index].clone(),
                                    None => im_str!("<None>"),
                                });
                        ui.popup(im_str!("select"), || {
                            ui.text(im_str!("Aquarium"));
                            ui.separator();
                            for (index, name) in names.iter().enumerate() {
                                if ui.selectable(name.clone(),
                                                 false,
                                                 ImGuiSelectableFlags::empty(),
                                                 ImVec2::new(0.0, 0.0)) {
                                    state.selected_fish = Some(index);
                                }
                            }
                        });
                    });
            }
        })
}

fn show_example_app_main_menu_bar<'a>(ui: &Ui<'a>, state: &mut State) {
    ui.main_menu_bar(|| {
        ui.menu(im_str!("File"))
            .build(|| { show_example_menu_file(ui, &mut state.file_menu); });
        ui.menu(im_str!("Edit"))
            .build(|| {
                ui.menu_item(im_str!("Undo"))
                    .shortcut(im_str!("CTRL+Z"))
                    .build();
                ui.menu_item(im_str!("Redo"))
                    .shortcut(im_str!("CTRL+Y"))
                    .enabled(false)
                    .build();
                ui.separator();
                ui.menu_item(im_str!("Cut"))
                    .shortcut(im_str!("CTRL+X"))
                    .build();
                ui.menu_item(im_str!("Copy"))
                    .shortcut(im_str!("CTRL+C"))
                    .build();
                ui.menu_item(im_str!("Paste"))
                    .shortcut(im_str!("CTRL+V"))
                    .build();
            });
    });
}

fn show_example_menu_file<'a>(ui: &Ui<'a>, state: &mut FileMenuState) {
    ui.menu_item(im_str!("(dummy menu)"))
        .enabled(false)
        .build();
    ui.menu_item(im_str!("New")).build();
    ui.menu_item(im_str!("Open"))
        .shortcut(im_str!("Ctrl+O"))
        .build();
    ui.menu(im_str!("Open Recent"))
        .build(|| {
            ui.menu_item(im_str!("fish_hat.c")).build();
            ui.menu_item(im_str!("fish_hat.inl")).build();
            ui.menu_item(im_str!("fish_hat.h")).build();
            ui.menu(im_str!("More.."))
                .build(|| {
                           ui.menu_item(im_str!("Hello")).build();
                           ui.menu_item(im_str!("Sailor")).build();
                           ui.menu(im_str!("Recurse.."))
                               .build(|| { show_example_menu_file(ui, state); });
                       });
        });
    ui.menu_item(im_str!("Save"))
        .shortcut(im_str!("Ctrl+S"))
        .build();
    ui.menu_item(im_str!("Save As..")).build();
    ui.separator();
    ui.menu(im_str!("Options"))
        .build(|| {
                   ui.menu_item(im_str!("Enabled"))
                       .selected(&mut state.enabled)
                       .build();
                   // TODO
               });
    ui.menu(im_str!("Colors"))
        .build(|| {
                   // TODO
               });
    ui.menu(im_str!("Disabled"))
        .enabled(false)
        .build(|| {
                   unreachable!();
               });
    let mut checked = true;
    ui.menu_item(im_str!("Checked"))
        .selected(&mut checked)
        .build();
    ui.menu_item(im_str!("Quit"))
        .shortcut(im_str!("Alt+F4"))
        .build();
}

fn show_example_app_auto_resize<'a>(ui: &Ui<'a>, state: &mut AutoResizeState, opened: &mut bool) {
    ui.window(im_str!("Example: Auto-resizing window"))
        .opened(opened)
        .always_auto_resize(true)
        .build(|| {
            ui.text(im_str!("Window will resize every-ui to the size of its content.
Note that you probably don't want to query the window size to
output your content because that would create a feedback loop."));
            ui.slider_int(im_str!("Number of lines"), &mut state.lines, 1, 20)
                .build();
            for i in 0..state.lines {
                ui.text(im_str!("{:2$}This is line {}", "", i, i as usize * 4));
            }
        })
}

fn show_example_app_fixed_overlay<'a>(ui: &Ui<'a>, opened: &mut bool) {
    ui.window(im_str!("Example: Fixed Overlay"))
        .opened(opened)
        .bg_alpha(0.3)
        .title_bar(false)
        .resizable(false)
        .movable(false)
        .save_settings(false)
        .build(|| {
                   ui.text(im_str!("Simple overlay\non the top-left side of the screen."));
                   ui.separator();
                   let mouse_pos = ui.imgui().mouse_pos();
                   ui.text(im_str!("Mouse Position: ({:.1},{:.1})", mouse_pos.0, mouse_pos.1));
               })
}

fn show_example_app_manipulating_window_title<'a>(ui: &Ui<'a>) {
    ui.window(im_str!("Same title as another window##1"))
        .position((100.0, 100.0), ImGuiSetCond_FirstUseEver)
        .build(|| {
                   ui.text(im_str!("This is window 1.
My title is the same as window 2, but my identifier is unique."));
               });
    ui.window(im_str!("Same title as another window##2"))
        .position((100.0, 200.0), ImGuiSetCond_FirstUseEver)
        .build(|| {
                   ui.text(im_str!("This is window 2.
My title is the same as window 1, but my identifier is unique."));
               });
    let chars = ['|', '/', '-', '\\'];
    let ch_idx = (ui.imgui().get_time() / 0.25) as usize & 3;
    let num = ui.imgui().get_frame_count(); // The C++ version uses rand() here
    let title = im_str!("Animated title {} {}###AnimatedTitle", chars[ch_idx], num);
    ui.window(title)
        .position((100.0, 300.0), ImGuiSetCond_FirstUseEver)
        .build(|| { ui.text(im_str!("This window has a changing title")); });
}
