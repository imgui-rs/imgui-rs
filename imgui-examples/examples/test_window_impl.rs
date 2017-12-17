extern crate glium;
extern crate imgui;
extern crate imgui_glium_renderer;

use imgui::*;

mod support;

struct State {
    show_app_main_menu_bar: bool,
    show_app_console: bool,
    show_app_log: bool,
    show_app_layout: bool,
    show_app_property_editor: bool,
    show_app_long_text: bool,
    show_app_auto_resize: bool,
    show_app_constrained_resize: bool,
    show_app_fixed_overlay: bool,
    show_app_manipulating_window_title: bool,
    show_app_custom_rendering: bool,
    show_app_style_editor: bool,
    show_app_metrics: bool,
    show_app_about: bool,
    no_titlebar: bool,
    no_border: bool,
    no_resize: bool,
    no_move: bool,
    no_scrollbar: bool,
    no_collapse: bool,
    no_menu: bool,
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
    radio_button: i32,
    color_edit: ColorEditState,
}

impl Default for State {
    fn default() -> Self {
        let mut buf = ImString::with_capacity(32);
        buf.push_str("日本語");
        let mut text = ImString::with_capacity(128);
        text.push_str("Hello, world!");
        State {
            show_app_main_menu_bar: false,
            show_app_console: false,
            show_app_log: false,
            show_app_layout: false,
            show_app_property_editor: false,
            show_app_long_text: false,
            show_app_auto_resize: false,
            show_app_fixed_overlay: false,
            show_app_constrained_resize: false,
            show_app_manipulating_window_title: false,
            show_app_custom_rendering: false,
            show_app_style_editor: false,
            show_app_metrics: false,
            show_app_about: false,
            no_titlebar: false,
            no_border: true,
            no_resize: false,
            no_move: false,
            no_scrollbar: false,
            no_collapse: false,
            no_menu: false,
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
            radio_button: 0,
            color_edit: ColorEditState::default(),
        }
    }
}

struct ColorEditState {
    color: [f32; 4],
    hdr: bool,
    alpha_preview: bool,
    alpha_half_preview: bool,
    options_menu: bool,
    alpha: bool,
    alpha_bar: bool,
    side_preview: bool,
    ref_color: bool,
    ref_color_v: [f32; 4],
}

impl Default for ColorEditState {
    fn default() -> Self {
        ColorEditState {
            color: [114.0 / 255.0, 144.0 / 255.0, 154.0 / 255.0, 200.0 / 255.0],
            hdr: false,
            alpha_preview: true,
            alpha_half_preview: false,
            options_menu: true,
            alpha: true,
            alpha_bar: true,
            side_preview: true,
            ref_color: false,
            ref_color_v: [1.0, 0.0, 1.0, 0.5],
        }
    }
}

struct FileMenuState {
    enabled: bool,
    f: f32,
    n: i32,
    b: bool,
}

impl Default for FileMenuState {
    fn default() -> Self {
        FileMenuState {
            enabled: true,
            f: 0.5,
            n: 0,
            b: true,
        }
    }
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

fn show_help_marker(ui: &Ui, desc: &str) {
    ui.text_disabled("(?)");
    if ui.is_item_hovered() {
        ui.tooltip(|| { ui.text(desc); });
    }
}

fn show_user_guide(ui: &Ui) {
    ui.bullet_text("Double-click on title bar to collapse window.");
    ui.bullet_text("Click and drag on lower right corner to resize window.");
    ui.bullet_text("Click and drag on any empty space to move window.");
    ui.bullet_text("Mouse Wheel to scroll.");
    // TODO: check font_allow_user_scaling
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
  Use +- to subtract.\n"
  );
}

fn show_test_window(ui: &Ui, state: &mut State, opened: &mut bool) {
    if state.show_app_main_menu_bar {
        show_example_app_main_menu_bar(ui, state)
    }
    if state.show_app_auto_resize {
        show_example_app_auto_resize(
            ui,
            &mut state.auto_resize_state,
            &mut state.show_app_auto_resize,
        );
    }
    if state.show_app_fixed_overlay {
        show_example_app_fixed_overlay(ui, &mut state.show_app_fixed_overlay);
    }
    if state.show_app_manipulating_window_title {
        show_example_app_manipulating_window_title(ui);
    }
    if state.show_app_metrics {
        ui.show_metrics_window(&mut state.show_app_metrics);
    }
    if state.show_app_style_editor {
        ui.window("Style Editor")
            .opened(&mut state.show_app_style_editor)
            .build(|| ui.show_default_style_editor());
    }
    if state.show_app_about {
        ui.window("About ImGui")
            .always_auto_resize(true)
            .opened(&mut state.show_app_about)
            .build(|| {
                ui.text(&format!("dear imgui, {}", imgui::get_version()));
                ui.separator();
                ui.text("By Omar Cornut and all github contributors.");
                ui.text(
                    "ImGui is licensed under the MIT License, see LICENSE for more \
                        information.",
                );
            });
    }

    ui.window("ImGui Demo")
        .title_bar(!state.no_titlebar)
        .show_borders(!state.no_border)
        .resizable(!state.no_resize)
        .movable(!state.no_move)
        .scroll_bar(!state.no_scrollbar)
        .collapsible(!state.no_collapse)
        .menu_bar(!state.no_menu)
        .size((550.0, 680.0), ImGuiCond::FirstUseEver)
        .opened(opened)
        .build(|| {
            ui.push_item_width(-140.0);
            ui.text(&format!("dear imgui says hello. ({})", imgui::get_version()));
            ui.menu_bar(|| {
                ui.menu("Menu").build(|| {
                    show_example_menu_file(ui, &mut state.file_menu);
                });
                ui.menu("Examples").build(|| {
                    ui.menu_item("Main menu bar")
                        .selected(&mut state.show_app_main_menu_bar)
                        .build();
                    ui.menu_item("Console")
                        .selected(&mut state.show_app_console)
                        .build();
                    ui.menu_item("Log")
                        .selected(&mut state.show_app_log)
                        .build();
                    ui.menu_item("Simple layout")
                        .selected(&mut state.show_app_layout)
                        .build();
                    ui.menu_item("Property editor")
                        .selected(&mut state.show_app_property_editor)
                        .build();
                    ui.menu_item("Long text display")
                        .selected(&mut state.show_app_long_text)
                        .build();
                    ui.menu_item("Auto-resizing window")
                        .selected(&mut state.show_app_auto_resize)
                        .build();
                    ui.menu_item("Constrained-resizing window")
                        .selected(&mut state.show_app_constrained_resize)
                        .build();
                    ui.menu_item("Simple overlay")
                        .selected(&mut state.show_app_fixed_overlay)
                        .build();
                    ui.menu_item("Manipulating window title")
                        .selected(&mut state.show_app_manipulating_window_title)
                        .build();
                    ui.menu_item("Custom rendering")
                        .selected(&mut state.show_app_custom_rendering)
                        .build();
                });
                ui.menu("Help").build(|| {
                    ui.menu_item("Metrics")
                        .selected(&mut state.show_app_metrics)
                        .build();
                    ui.menu_item("Style Editor")
                        .selected(&mut state.show_app_style_editor)
                        .build();
                    ui.menu_item("About ImGui")
                        .selected(&mut state.show_app_about)
                        .build();
                });
            });
            ui.spacing();
            if ui.collapsing_header("Help").build() {
                ui.text_wrapped(
                    "This window is being created by the show_test_window() \
                    function. Please refer to the code for programming \
                    reference.\n\nUser Guide:"
                );
                show_user_guide(ui);
            }

            if ui.collapsing_header("Window options").build() {
                ui.checkbox("No titlebar", &mut state.no_titlebar);
                ui.same_line(150.0);
                ui.checkbox("No border", &mut state.no_border);
                ui.same_line(300.0);
                ui.checkbox("No resize", &mut state.no_resize);
                ui.checkbox("No move", &mut state.no_move);
                ui.same_line(150.0);
                ui.checkbox("No scrollbar", &mut state.no_scrollbar);
                ui.same_line(300.0);
                ui.checkbox("No collapse", &mut state.no_collapse);
                ui.checkbox("No menu", &mut state.no_menu);

                ui.tree_node("Style").build(|| {
                    ui.show_default_style_editor()
                });
            }
            if ui.collapsing_header("Widgets").build() {
                ui.tree_node("Tree").build(|| for i in 0..5 {
                    ui.tree_node(&format!("Child {}", i)).build(|| {
                        ui.text("blah blah");
                        ui.same_line(0.0);
                        if ui.small_button("print") {
                            println!("Child {} pressed", i);
                        }
                    });
                });
                ui.tree_node("Bullets").build(|| {
                    ui.bullet_text("Bullet point 1");
                    ui.bullet_text("Bullet point 2\nOn multiple lines");
                    ui.bullet();
                    ui.text("Bullet point 3 (two calls)");

                    ui.bullet();
                    ui.small_button("Button");
                });
                ui.tree_node("Colored text").build(|| {
                    ui.text_colored((1.0, 0.0, 1.0, 1.0), "Pink");
                    ui.text_colored((1.0, 1.0, 0.0, 1.0), "Yellow");
                    ui.text_disabled("Disabled");
                });
                ui.tree_node("Word Wrapping").build(|| {
                    ui.text_wrapped(
                        "This text should automatically wrap on the edge of \
                                             the window.The current implementation for text \
                                             wrapping follows simple rulessuitable for English \
                                             and possibly other languages."
                    );
                    ui.spacing();

                    ui.slider_float("Wrap width", &mut state.wrap_width, -20.0, 600.0)
                        .display_format("%.0f")
                        .build();

                    ui.text("Test paragraph 1:");
                    // TODO

                    ui.text("Test paragraph 2:");
                    // TODO
                });
                ui.tree_node("UTF-8 Text").build(|| {
                    ui.text_wrapped(
                        "CJK text will only appear if the font was loaded \
                                             with theappropriate CJK character ranges. Call \
                                             io.Font->LoadFromFileTTF()manually to load extra \
                                             character ranges."
                    );

                    ui.text("Hiragana: かきくけこ (kakikukeko)");
                    ui.text("Kanjis: 日本語 (nihongo)");
                    ui.input_text("UTF-8 input", &mut state.buf)
                        .build();
                });

                ui.radio_button("radio a", &mut state.radio_button, 0);
                ui.same_line(0.0);
                ui.radio_button("radio b", &mut state.radio_button, 1);
                ui.same_line(0.0);
                ui.radio_button("radio c", &mut state.radio_button, 2);

                ui.separator();
                ui.label_text("label", "Value");
                ui.combo(
                    "combo",
                    &mut state.item,
                    &[
                        "aaaa",
                        "bbbb",
                        "cccc",
                        "dddd",
                        "eeee",
                    ],
                    -1,
                );
                let items = [
                    "AAAA",
                    "BBBB",
                    "CCCC",
                    "DDDD",
                    "EEEE",
                    "FFFF",
                    "GGGG",
                    "HHHH",
                    "IIII",
                    "JJJJ",
                    "KKKK",
                ];
                ui.combo("combo scroll", &mut state.item2, &items, -1);
                ui.input_text("input text", &mut state.text)
                    .build();
                ui.input_int("input int", &mut state.i0).build();
                ui.input_float("input float", &mut state.f0)
                    .step(0.01)
                    .step_fast(1.0)
                    .build();
                ui.input_float3("input float3", &mut state.vec3f)
                    .build();
                ui.color_edit("color 1", &mut state.col1).build();
                ui.color_edit("color 2", &mut state.col2).build();

                ui.tree_node("Multi-component Widgets").build(|| {
                    ui.input_float2("input float2", &mut state.vec2f)
                        .build();
                    ui.input_int2("input int2", &mut state.vec2i)
                        .build();
                    ui.spacing();

                    ui.input_float3("input float3", &mut state.vec3f)
                        .build();
                    ui.input_int3("input int3", &mut state.vec3i)
                        .build();
                    ui.spacing();
                });

                ui.tree_node("Color/Picker Widgets").build(|| {
                    let s = &mut state.color_edit;
                    ui.checkbox("With HDR", &mut s.hdr);
                    ui.same_line(0.0);
                    show_help_marker(
                        ui,
                        "Currently all this does is to lift the 0..1 \
                                               limits on dragging widgets.",
                    );

                    ui.checkbox("With Alpha Preview", &mut s.alpha_preview);
                    ui.checkbox(
                        "With Half Alpha Preview",
                        &mut s.alpha_half_preview,
                    );
                    ui.checkbox("With Options Menu", &mut s.options_menu);
                    ui.same_line(0.0);
                    show_help_marker(
                        ui,
                        "Right-click on the individual color widget to \
                                               show options.",
                    );
                    let misc_flags = {
                        let mut f = ImGuiColorEditFlags::empty();
                        f.set(ImGuiColorEditFlags::HDR, s.hdr);
                        f.set(ImGuiColorEditFlags::AlphaPreviewHalf, s.alpha_half_preview);
                        if !s.alpha_half_preview {
                            f.set(ImGuiColorEditFlags::AlphaPreview, s.alpha_preview);
                        }
                        f.set(ImGuiColorEditFlags::NoOptions, !s.options_menu);
                        f
                    };

                    ui.text("Color widget:");
                    ui.same_line(0.0);
                    show_help_marker(
                        ui,
                        "Click on the colored square to open a color picker.
CTRL+click on individual component to input value.\n",
                    );
                    ui.color_edit("MyColor##1", &mut s.color)
                        .flags(misc_flags)
                        .alpha(false)
                        .build();

                    ui.text("Color widget HSV with Alpha:");
                    ui.color_edit("MyColor##2", &mut s.color)
                        .flags(misc_flags)
                        .mode(ColorEditMode::HSV)
                        .build();

                    ui.text("Color widget with Float Display:");
                    ui.color_edit("MyColor##2f", &mut s.color)
                        .flags(misc_flags)
                        .format(ColorFormat::Float)
                        .build();

                    ui.text("Color button with Picker:");
                    ui.same_line(0.0);
                    show_help_marker(
                        ui,
                        "With the inputs(false) function you can hide all \
                            the slider/text inputs.\n \
                            With the label(false) function you can pass a non-empty label which \
                            will only be used for the tooltip and picker popup.",
                    );
                    ui.color_edit("MyColor##3", &mut s.color)
                        .flags(misc_flags)
                        .inputs(false)
                        .label(false)
                        .build();

                    ui.text("Color picker:");
                    ui.checkbox("With Alpha", &mut s.alpha);
                    ui.checkbox("With Alpha Bar", &mut s.alpha_bar);
                    ui.checkbox("With Side Preview", &mut s.side_preview);
                    if s.side_preview {
                        ui.same_line(0.0);
                        ui.checkbox("With Ref Color", &mut s.ref_color);
                        if s.ref_color {
                            ui.same_line(0.0);
                            ui.color_edit("##RefColor", &mut s.ref_color_v)
                                .flags(misc_flags)
                                .inputs(false)
                                .build();
                        }
                    }
                    let mut b = ui.color_picker("MyColor##4", &mut s.color)
                        .flags(misc_flags)
                        .alpha(s.alpha)
                        .alpha_bar(s.alpha_bar)
                        .side_preview(s.side_preview)
                        .rgb(true);

                    if s.ref_color {
                        b = b.reference_color(&s.ref_color_v)
                    }
                    b.build();
                });
            }
            if ui.collapsing_header("Popups & Modal windows")
                .build()
            {
                ui.tree_node("Popups").build(|| {
                    ui.text_wrapped(
                        "When a popup is active, it inhibits interacting \
                                             with windows that are behind the popup. Clicking \
                                             outside the popup closes it."
                    );
                    let names = [
                        "Bream",
                        "Haddock",
                        "Mackerel",
                        "Pollock",
                        "Tilefish",
                    ];
                    if ui.small_button("Select..") {
                        ui.open_popup("select");
                    }
                    ui.same_line(0.0);
                    ui.text(match state.selected_fish {
                        Some(index) => names[index],
                        None => "<None>",
                    });
                    ui.popup("select", || {
                        ui.text("Aquarium");
                        ui.separator();
                        for (index, name) in names.iter().enumerate() {
                            if ui.selectable(
                                name,
                                false,
                                ImGuiSelectableFlags::empty(),
                                ImVec2::new(0.0, 0.0),
                            )
                            {
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
        ui.menu("File").build(|| {
            show_example_menu_file(ui, &mut state.file_menu);
        });
        ui.menu("Edit").build(|| {
            ui.menu_item("Undo")
                .shortcut("CTRL+Z")
                .build();
            ui.menu_item("Redo")
                .shortcut("CTRL+Y")
                .enabled(false)
                .build();
            ui.separator();
            ui.menu_item("Cut")
                .shortcut("CTRL+X")
                .build();
            ui.menu_item("Copy")
                .shortcut("CTRL+C")
                .build();
            ui.menu_item("Paste")
                .shortcut("CTRL+V")
                .build();
        });
    });
}

fn show_example_menu_file<'a>(ui: &Ui<'a>, state: &mut FileMenuState) {
    ui.menu_item("(dummy menu)").enabled(false).build();
    ui.menu_item("New").build();
    ui.menu_item("Open")
        .shortcut("Ctrl+O")
        .build();
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
    ui.menu_item("Save")
        .shortcut("Ctrl+S")
        .build();
    ui.menu_item("Save As..").build();
    ui.separator();
    ui.menu("Options").build(|| {
        ui.menu_item("Enabled")
            .selected(&mut state.enabled)
            .build();
        ui.child_frame("child", (0.0, 60.0))
            .show_borders(true)
            .build(|| for i in 0..10 {
                ui.text(&format!("Scrolling Text {}", i));
            });
        ui.slider_float("Value", &mut state.f, 0.0, 1.0)
            .build();
        ui.input_float("Input", &mut state.f)
            .step(0.1)
            .build();
        let items = ["Yes", "No", "Maybe"];
        ui.combo("Combo", &mut state.n, &items, -1);
        ui.checkbox("Check", &mut state.b);
    });
    ui.menu("Colors").build(|| for &col in
        ImGuiCol::values()
    {
        ui.menu_item(imgui::get_style_color_name(col)).build();
    });
    ui.menu("Disabled").enabled(false).build(|| {
        unreachable!();
    });
    ui.menu_item("Checked").selected(&mut true).build();
    ui.menu_item("Quit")
        .shortcut("Alt+F4")
        .build();
}

fn show_example_app_auto_resize(ui: &Ui, state: &mut AutoResizeState, opened: &mut bool) {
    ui.window("Example: Auto-resizing window")
        .opened(opened)
        .always_auto_resize(true)
        .build(|| {
            ui.text(
                "Window will resize every-ui to the size of its content.
Note that you probably don't want to query the window size to
output your content because that would create a feedback loop.",
            );
            ui.slider_int("Number of lines", &mut state.lines, 1, 20)
                .build();
            for i in 0..state.lines {
                ui.text(&format!("{:2$}This is line {}", "", i, i as usize * 4));
            }
        })
}

#[allow(deprecated)]
fn show_example_app_fixed_overlay(ui: &Ui, opened: &mut bool) {
    const DISTANCE: f32 = 10.0;
    let window_pos = (DISTANCE, DISTANCE);
    ui.with_color_var(ImGuiCol::WindowBg, (0.0, 0.0, 0.0, 0.3), || {
        ui.window("Example: Fixed Overlay")
        .opened(opened)
        .position(window_pos, ImGuiCond::Always)
        .title_bar(false)
        .resizable(false)
        .always_auto_resize(true)
        .movable(false)
        .save_settings(false)
        .build(|| {
          ui.text("Simple overlay\nin the corner of the screen.\n(right-click to change position)");
          ui.separator();
          let mouse_pos = ui.imgui().mouse_pos();
          ui.text(&format!(
              "Mouse Position: ({:.1},{:.1})",
              mouse_pos.0,
              mouse_pos.1
              ));
        })
    })
}

fn show_example_app_manipulating_window_title(ui: &Ui) {
    ui.window("Same title as another window##1")
        .position((100.0, 100.0), ImGuiCond::FirstUseEver)
        .build(|| {
            ui.text(
                "This is window 1.
My title is the same as window 2, but my identifier is unique.",
            );
        });
    ui.window("Same title as another window##2")
        .position((100.0, 200.0), ImGuiCond::FirstUseEver)
        .build(|| {
            ui.text(
                "This is window 2.
My title is the same as window 1, but my identifier is unique.",
            );
        });
    let chars = ['|', '/', '-', '\\'];
    let ch_idx = (ui.imgui().get_time() / 0.25) as usize & 3;
    let num = ui.imgui().get_frame_count(); // The C++ version uses rand() here
    let title = format!("Animated title {} {}###AnimatedTitle", chars[ch_idx], num);
    ui.window(&title)
        .position((100.0, 300.0), ImGuiCond::FirstUseEver)
        .build(|| ui.text("This window has a changing title"));
}
