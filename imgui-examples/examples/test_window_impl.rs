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
    no_resize: bool,
    no_move: bool,
    no_scrollbar: bool,
    no_collapse: bool,
    no_menu: bool,
    no_close: bool,
    wrap_width: f32,
    buf: ImString,
    item: i32,
    item2: i32,
    text: ImString,
    text_multiline: ImString,
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
    custom_rendering: CustomRenderingState,
    dont_ask_me_next_time: bool,
    stacked_modals_item: i32,
    stacked_modals_color: [f32; 4],
}

impl Default for State {
    fn default() -> Self {
        let mut buf = ImString::with_capacity(32);
        buf.push_str("日本語");
        let mut text = ImString::with_capacity(128);
        text.push_str("Hello, world!");
        let mut text_multiline = ImString::with_capacity(128);
        text_multiline.push_str("Hello, world!\nMultiline");
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
            no_resize: false,
            no_move: false,
            no_scrollbar: false,
            no_collapse: false,
            no_menu: false,
            no_close: false,
            wrap_width: 200.0,
            buf: buf,
            item: 0,
            item2: 0,
            text: text,
            text_multiline: text_multiline,
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
            custom_rendering: Default::default(),
            dont_ask_me_next_time: false,
            stacked_modals_item: 0,
            stacked_modals_color: [0.4, 0.7, 0.0, 0.5],
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
    fn default() -> Self {
        AutoResizeState { lines: 10 }
    }
}

struct CustomRenderingState {
    sz: f32,
    col: [f32; 3],
    points: Vec<[f32; 2]>,
    adding_line: bool,
}

impl Default for CustomRenderingState {
    fn default() -> Self {
        CustomRenderingState {
            sz: 36.0,
            col: [1.0, 1.0, 0.4],
            points: vec![],
            adding_line: false,
        }
    }
}

fn main() {
    let mut state = State::default();

    let system = support::init(file!());
    system.main_loop(|run, ui| show_test_window(ui, &mut state, run));
}

fn show_help_marker(ui: &Ui, desc: &str) {
    ui.text_disabled(im_str!("(?)"));
    if ui.is_item_hovered() {
        ui.tooltip(|| {
            ui.text(desc);
        });
    }
}

fn show_user_guide(ui: &Ui) {
    ui.bullet_text(im_str!("Double-click on title bar to collapse window."));
    ui.bullet_text(im_str!(
        "Click and drag on lower right corner to resize window."
    ));
    ui.bullet_text(im_str!("Click and drag on any empty space to move window."));
    ui.bullet_text(im_str!("Mouse Wheel to scroll."));
    // TODO: check font_allow_user_scaling
    ui.bullet_text(im_str!(
        "TAB/SHIFT+TAB to cycle through keyboard editable fields."
    ));
    ui.bullet_text(im_str!("CTRL+Click on a slider or drag box to input text."));
    ui.bullet_text(im_str!(
        "While editing text:
- Hold SHIFT or use mouse to select text
- CTRL+Left/Right to word jump
- CTRL+A or double-click to select all
- CTRL+X,CTRL+C,CTRL+V clipboard
- CTRL+Z,CTRL+Y undo/redo
- ESCAPE to revert
- You can apply arithmetic operators +,*,/ on numerical values.
  Use +- to subtract.\n"
    ));
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
        ui.window(im_str!("Style Editor"))
            .opened(&mut state.show_app_style_editor)
            .build(|| ui.show_default_style_editor());
    }
    if state.show_app_about {
        ui.window(im_str!("About ImGui"))
            .always_auto_resize(true)
            .opened(&mut state.show_app_about)
            .build(|| {
                ui.text(format!("dear imgui, {}", imgui::dear_imgui_version()));
                ui.separator();
                ui.text("By Omar Cornut and all github contributors.");
                ui.text(
                    "ImGui is licensed under the MIT License, see LICENSE for more \
                     information.",
                );
            });
    }

    if state.show_app_custom_rendering {
        show_example_app_custom_rendering(
            ui,
            &mut state.custom_rendering,
            &mut state.show_app_custom_rendering,
        );
    }

    let mut window = ui
        .window(im_str!("ImGui Demo"))
        .title_bar(!state.no_titlebar)
        .resizable(!state.no_resize)
        .movable(!state.no_move)
        .scroll_bar(!state.no_scrollbar)
        .collapsible(!state.no_collapse)
        .menu_bar(!state.no_menu)
        .size([550.0, 680.0], Condition::FirstUseEver);
    if !state.no_close {
        window = window.opened(opened)
    }
    window.build(|| {
        let _token = ui.push_item_width(-140.0);
        ui.text(format!("dear imgui says hello. ({})", imgui::dear_imgui_version()));
        ui.menu_bar(|| {
            ui.menu(im_str!("Menu")).build(|| {
                show_example_menu_file(ui, &mut state.file_menu);
            });
            ui.menu(im_str!("Examples")).build(|| {
                ui.menu_item(im_str!("Main menu bar"))
                    .selected(&mut state.show_app_main_menu_bar)
                    .build();
                ui.menu_item(im_str!("Console"))
                    .selected(&mut state.show_app_console)
                    .build();
                ui.menu_item(im_str!("Log"))
                    .selected(&mut state.show_app_log)
                    .build();
                ui.menu_item(im_str!("Simple layout"))
                    .selected(&mut state.show_app_layout)
                    .build();
                ui.menu_item(im_str!("Property editor"))
                    .selected(&mut state.show_app_property_editor)
                    .build();
                ui.menu_item(im_str!("Long text display"))
                    .selected(&mut state.show_app_long_text)
                    .build();
                ui.menu_item(im_str!("Auto-resizing window"))
                    .selected(&mut state.show_app_auto_resize)
                    .build();
                ui.menu_item(im_str!("Constrained-resizing window"))
                    .selected(&mut state.show_app_constrained_resize)
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
            ui.menu(im_str!("Help")).build(|| {
                ui.menu_item(im_str!("Metrics"))
                    .selected(&mut state.show_app_metrics)
                    .build();
                ui.menu_item(im_str!("Style Editor"))
                    .selected(&mut state.show_app_style_editor)
                    .build();
                ui.menu_item(im_str!("About ImGui"))
                    .selected(&mut state.show_app_about)
                    .build();
            });
        });
        ui.spacing();
        if ui.collapsing_header(im_str!("Help")).build() {
            ui.text_wrapped(im_str!(
                "This window is being created by the show_test_window() \
                 function. Please refer to the code for programming \
                 reference.\n\nUser Guide:"
            ));
            show_user_guide(ui);
        }

        if ui.collapsing_header(im_str!("Window options")).build() {
            ui.checkbox(im_str!("No titlebar"), &mut state.no_titlebar);
            ui.same_line(150.0);
            ui.checkbox(im_str!("No scrollbar"), &mut state.no_scrollbar);
            ui.same_line(300.0);
            ui.checkbox(im_str!("No menu"), &mut state.no_menu);
            ui.checkbox(im_str!("No move"), &mut state.no_move);
            ui.same_line(150.0);
            ui.checkbox(im_str!("No resize"), &mut state.no_resize);
            ui.same_line(300.0);
            ui.checkbox(im_str!("No collapse"), &mut state.no_collapse);
            ui.checkbox(im_str!("No close"), &mut state.no_close);

            ui.tree_node(im_str!("Style"))
                .build(|| ui.show_default_style_editor());
        }
        if ui.collapsing_header(im_str!("Widgets")).build() {
            ui.tree_node(im_str!("Tree")).build(|| {
                for i in 0..5 {
                    ui.tree_node(&im_str!("Child {}", i)).build(|| {
                        ui.text(im_str!("blah blah"));
                        ui.same_line(0.0);
                        if ui.small_button(im_str!("print")) {
                            println!("Child {} pressed", i);
                        }
                    });
                }
            });
            ui.tree_node(im_str!("Bullets")).build(|| {
                ui.bullet_text(im_str!("Bullet point 1"));
                ui.bullet_text(im_str!("Bullet point 2\nOn multiple lines"));
                ui.bullet();
                ui.text(im_str!("Bullet point 3 (two calls)"));

                ui.bullet();
                ui.small_button(im_str!("Button"));
            });
            ui.tree_node(im_str!("Colored text")).build(|| {
                ui.text_colored([1.0, 0.0, 1.0, 1.0], im_str!("Pink"));
                ui.text_colored([1.0, 1.0, 0.0, 1.0], im_str!("Yellow"));
                ui.text_disabled(im_str!("Disabled"));
            });

            ui.tree_node(im_str!("Multi-line text")).build(|| {
                ui.input_text_multiline(
                    im_str!("multiline"),
                    &mut state.text_multiline,
                    [300., 100.],
                ).build();
            });

            ui.tree_node(im_str!("Word Wrapping")).build(|| {
                ui.text_wrapped(im_str!(
                    "This text should automatically wrap on the edge of \
                     the window.The current implementation for text \
                     wrapping follows simple rulessuitable for English \
                     and possibly other languages."
                ));
                ui.spacing();

                ui.slider_float(im_str!("Wrap width"), &mut state.wrap_width, -20.0, 600.0)
                    .display_format(im_str!("%.0f"))
                    .build();

                ui.text(im_str!("Test paragraph 1:"));
                // TODO

                ui.text(im_str!("Test paragraph 2:"));
                // TODO
            });
            ui.tree_node(im_str!("UTF-8 Text")).build(|| {
                ui.text_wrapped(im_str!(
                    "CJK text will only appear if the font was loaded \
                     with theappropriate CJK character ranges. Call \
                     io.Font->LoadFromFileTTF()manually to load extra \
                     character ranges."
                ));

                ui.text(im_str!("Hiragana: かきくけこ (kakikukeko)"));
                ui.text(im_str!("Kanjis: 日本語 (nihongo)"));
                ui.input_text(im_str!("UTF-8 input"), &mut state.buf)
                    .build();
            });

            ui.radio_button(im_str!("radio a"), &mut state.radio_button, 0);
            ui.same_line(0.0);
            ui.radio_button(im_str!("radio b"), &mut state.radio_button, 1);
            ui.same_line(0.0);
            ui.radio_button(im_str!("radio c"), &mut state.radio_button, 2);

            ui.separator();
            ui.label_text(im_str!("label"), im_str!("Value"));
            ui.combo(
                im_str!("combo"),
                &mut state.item,
                &[
                    im_str!("aaaa"),
                    im_str!("bbbb"),
                    im_str!("cccc"),
                    im_str!("dddd"),
                    im_str!("eeee"),
                ],
                -1,
            );
            let items = [
                im_str!("AAAA"),
                im_str!("BBBB"),
                im_str!("CCCC"),
                im_str!("DDDD"),
                im_str!("EEEE"),
                im_str!("FFFF"),
                im_str!("GGGG"),
                im_str!("HHHH"),
                im_str!("IIII"),
                im_str!("JJJJ"),
                im_str!("KKKK"),
            ];
            ui.combo(im_str!("combo scroll"), &mut state.item2, &items, -1);
            ui.input_text(im_str!("input text"), &mut state.text)
                .build();
            ui.input_int(im_str!("input int"), &mut state.i0).build();
            ui.drag_int(im_str!("drag int"), &mut state.i0).build();
            ui.input_float(im_str!("input float"), &mut state.f0)
                .step(0.01)
                .step_fast(1.0)
                .build();
            ui.drag_float(im_str!("drag float"), &mut state.f0)
                .speed(0.001)
                .min(-1.0)
                .max(1.0)
                .build();
            ui.input_float3(im_str!("input float3"), &mut state.vec3f)
                .build();
            ui.color_edit(im_str!("color 1"), &mut state.col1).build();
            ui.color_edit(im_str!("color 2"), &mut state.col2).build();

            ui.tree_node(im_str!("Multi-component Widgets")).build(|| {
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

            ui.tree_node(im_str!("Color/Picker Widgets")).build(|| {
                let s = &mut state.color_edit;
                ui.checkbox(im_str!("With HDR"), &mut s.hdr);
                ui.same_line(0.0);
                show_help_marker(
                    ui,
                    "Currently all this does is to lift the 0..1 \
                     limits on dragging widgets.",
                );

                ui.checkbox(im_str!("With Alpha Preview"), &mut s.alpha_preview);
                ui.checkbox(
                    im_str!("With Half Alpha Preview"),
                    &mut s.alpha_half_preview,
                );
                ui.checkbox(im_str!("With Options Menu"), &mut s.options_menu);
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

                ui.text(im_str!("Color widget:"));
                ui.same_line(0.0);
                show_help_marker(
                    ui,
                    "Click on the colored square to open a color picker.
CTRL+click on individual component to input value.\n",
                );
                ui.color_edit(im_str!("MyColor##1"), &mut s.color)
                    .flags(misc_flags)
                    .alpha(false)
                    .build();

                ui.text(im_str!("Color widget HSV with Alpha:"));
                ui.color_edit(im_str!("MyColor##2"), &mut s.color)
                    .flags(misc_flags)
                    .mode(ColorEditMode::HSV)
                    .build();

                ui.text(im_str!("Color widget with Float Display:"));
                ui.color_edit(im_str!("MyColor##2f"), &mut s.color)
                    .flags(misc_flags)
                    .format(ColorFormat::Float)
                    .build();

                ui.text(im_str!("Color button with Picker:"));
                ui.same_line(0.0);
                show_help_marker(
                    ui,
                    "With the inputs(false) function you can hide all \
                     the slider/text inputs.\n \
                     With the label(false) function you can pass a non-empty label which \
                     will only be used for the tooltip and picker popup.",
                );
                ui.color_edit(im_str!("MyColor##3"), &mut s.color)
                    .flags(misc_flags)
                    .inputs(false)
                    .label(false)
                    .build();

                ui.text(im_str!("Color picker:"));
                ui.checkbox(im_str!("With Alpha"), &mut s.alpha);
                ui.checkbox(im_str!("With Alpha Bar"), &mut s.alpha_bar);
                ui.checkbox(im_str!("With Side Preview"), &mut s.side_preview);
                if s.side_preview {
                    ui.same_line(0.0);
                    ui.checkbox(im_str!("With Ref Color"), &mut s.ref_color);
                    if s.ref_color {
                        ui.same_line(0.0);
                        ui.color_edit(im_str!("##RefColor"), &mut s.ref_color_v)
                            .flags(misc_flags)
                            .inputs(false)
                            .build();
                    }
                }
                let mut b = ui
                    .color_picker(im_str!("MyColor##4"), &mut s.color)
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
        if ui
            .collapsing_header(im_str!("Popups & Modal windows"))
            .build()
        {
            ui.tree_node(im_str!("Popups")).build(|| {
                ui.text_wrapped(im_str!(
                    "When a popup is active, it inhibits interacting \
                     with windows that are behind the popup. Clicking \
                     outside the popup closes it."
                ));
                let names = [
                    im_str!("Bream"),
                    im_str!("Haddock"),
                    im_str!("Mackerel"),
                    im_str!("Pollock"),
                    im_str!("Tilefish"),
                ];
                if ui.small_button(im_str!("Select..")) {
                    ui.open_popup(im_str!("select"));
                }
                ui.same_line(0.0);
                ui.text(match state.selected_fish {
                    Some(index) => names[index],
                    None => im_str!("<None>"),
                });
                ui.popup(im_str!("select"), || {
                    ui.text(im_str!("Aquarium"));
                    ui.separator();
                    for (index, name) in names.iter().enumerate() {
                        if ui.selectable(
                            name,
                            false,
                            ImGuiSelectableFlags::empty(),
                            [0.0, 0.0]
                        ) {
                            state.selected_fish = Some(index);
                        }
                    }
                });
            });

            ui.tree_node(im_str!("Modals")).build(|| {
                ui.text_wrapped(im_str!(
                    "Modal windows are like popups but the user cannot close \
                     them by clicking outside the window."
                ));

                if ui.button(im_str!("Delete.."), [0.0, 0.0]) {
                    ui.open_popup(im_str!("Delete?"));
                }
                ui.popup_modal(im_str!("Delete?")).always_auto_resize(true).build(|| {
                    ui.text("All those beautiful files will be deleted.\nThis operation cannot be undone!\n\n");
                    ui.separator();
                    let _token = ui.push_style_var(StyleVar::FramePadding([0.0, 0.0]));
                    ui.checkbox(im_str!("Don't ask me next time"), &mut state.dont_ask_me_next_time);

                    if ui.button(im_str!("OK"), [120.0, 0.0]) {
                        ui.close_current_popup();
                    }
                    ui.same_line(0.0);
                    if ui.button(im_str!("Cancel"), [120.0, 0.0]) {
                        ui.close_current_popup();
                    }
                });

                if ui.button(im_str!("Stacked modals.."), [0.0, 0.0]) {
                    ui.open_popup(im_str!("Stacked 1"));
                }
                ui.popup_modal(im_str!("Stacked 1")).build(|| {
                    ui.text(
                       "Hello from Stacked The First\n\
                        Using style[StyleColor::ModalWindowDarkening] for darkening."
                    );

                    let items = &[im_str!("aaaa"), im_str!("bbbb"), im_str!("cccc"), im_str!("dddd"), im_str!("eeee")];
                    ui.combo(im_str!("Combo"), &mut state.stacked_modals_item, items, -1);

                    ui.color_edit(im_str!("color"), &mut state.stacked_modals_color).build();

                    if ui.button(im_str!("Add another modal.."), [0.0, 0.0]) {
                        ui.open_popup(im_str!("Stacked 2"))   ;
                    }
                    ui.popup_modal(im_str!("Stacked 2")).build(|| {
                        ui.text("Hello from Stacked The Second");
                        if ui.button(im_str!("Close"), [0.0, 0.0]) {
                            ui.close_current_popup();
                        }
                    });

                    if ui.button(im_str!("Close"), [0.0, 0.0]) {
                        ui.close_current_popup();
                    }
                });
            });
        }
    })
}

fn show_example_app_main_menu_bar<'a>(ui: &Ui<'a>, state: &mut State) {
    ui.main_menu_bar(|| {
        ui.menu(im_str!("File")).build(|| {
            show_example_menu_file(ui, &mut state.file_menu);
        });
        ui.menu(im_str!("Edit")).build(|| {
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
    ui.menu_item(im_str!("(dummy menu)")).enabled(false).build();
    ui.menu_item(im_str!("New")).build();
    ui.menu_item(im_str!("Open"))
        .shortcut(im_str!("Ctrl+O"))
        .build();
    ui.menu(im_str!("Open Recent")).build(|| {
        ui.menu_item(im_str!("fish_hat.c")).build();
        ui.menu_item(im_str!("fish_hat.inl")).build();
        ui.menu_item(im_str!("fish_hat.h")).build();
        ui.menu(im_str!("More..")).build(|| {
            ui.menu_item(im_str!("Hello")).build();
            ui.menu_item(im_str!("Sailor")).build();
            ui.menu(im_str!("Recurse..")).build(|| {
                show_example_menu_file(ui, state);
            });
        });
    });
    ui.menu_item(im_str!("Save"))
        .shortcut(im_str!("Ctrl+S"))
        .build();
    ui.menu_item(im_str!("Save As..")).build();
    ui.separator();
    ui.menu(im_str!("Options")).build(|| {
        ui.menu_item(im_str!("Enabled"))
            .selected(&mut state.enabled)
            .build();
        ui.child_frame(im_str!("child"), [0.0, 60.0])
            .show_borders(true)
            .build(|| {
                for i in 0..10 {
                    ui.text(format!("Scrolling Text {}", i));
                }
            });
        ui.slider_float(im_str!("Value"), &mut state.f, 0.0, 1.0)
            .build();
        ui.input_float(im_str!("Input"), &mut state.f)
            .step(0.1)
            .build();
        let items = [im_str!("Yes"), im_str!("No"), im_str!("Maybe")];
        ui.combo(im_str!("Combo"), &mut state.n, &items, -1);
        ui.checkbox(im_str!("Check"), &mut state.b);
    });
    ui.menu(im_str!("Colors")).build(|| {
        for &col in StyleColor::VARIANTS.iter() {
            ui.menu_item(&im_str!("{:?}", col)).build();
        }
    });
    ui.menu(im_str!("Disabled")).enabled(false).build(|| {
        unreachable!();
    });
    ui.menu_item(im_str!("Checked")).selected(&mut true).build();
    ui.menu_item(im_str!("Quit"))
        .shortcut(im_str!("Alt+F4"))
        .build();
}

fn show_example_app_auto_resize(ui: &Ui, state: &mut AutoResizeState, opened: &mut bool) {
    ui.window(im_str!("Example: Auto-resizing window"))
        .opened(opened)
        .always_auto_resize(true)
        .build(|| {
            ui.text(
                "Window will resize every-ui to the size of its content.
Note that you probably don't want to query the window size to
output your content because that would create a feedback loop.",
            );
            ui.slider_int(im_str!("Number of lines"), &mut state.lines, 1, 20)
                .build();
            for i in 0..state.lines {
                ui.text(format!("{:2$}This is line {}", "", i, i as usize * 4));
            }
        })
}

fn show_example_app_fixed_overlay(ui: &Ui, opened: &mut bool) {
    const DISTANCE: f32 = 10.0;
    let window_pos = [DISTANCE, DISTANCE];
    let _token = ui.push_style_color(StyleColor::WindowBg, [0.0, 0.0, 0.0, 0.3]);
    ui.window(im_str!("Example: Fixed Overlay"))
        .opened(opened)
        .position(window_pos, Condition::Always)
        .title_bar(false)
        .resizable(false)
        .always_auto_resize(true)
        .movable(false)
        .save_settings(false)
        .build(|| {
            ui.text(
                "Simple overlay\nin the corner of the screen.\n(right-click to change position)",
            );
            ui.separator();
            let mouse_pos = ui.io().mouse_pos;
            ui.text(format!(
                "Mouse Position: ({:.1},{:.1})",
                mouse_pos[0], mouse_pos[1]
            ));
        })
}

fn show_example_app_manipulating_window_title(ui: &Ui) {
    ui.window(im_str!("Same title as another window##1"))
        .position([100.0, 100.0], Condition::FirstUseEver)
        .build(|| {
            ui.text(
                "This is window 1.
My title is the same as window 2, but my identifier is unique.",
            );
        });
    ui.window(im_str!("Same title as another window##2"))
        .position([100.0, 200.0], Condition::FirstUseEver)
        .build(|| {
            ui.text(
                "This is window 2.
My title is the same as window 1, but my identifier is unique.",
            );
        });
    let chars = ['|', '/', '-', '\\'];
    let ch_idx = (ui.time() / 0.25) as usize & 3;
    let num = ui.frame_count(); // The C++ version uses rand() here
    let title = im_str!("Animated title {} {}###AnimatedTitle", chars[ch_idx], num);
    ui.window(&title)
        .position([100.0, 300.0], Condition::FirstUseEver)
        .build(|| ui.text("This window has a changing title"));
}

fn show_example_app_custom_rendering(ui: &Ui, state: &mut CustomRenderingState, opened: &mut bool) {
    ui.window(im_str!("Example: Custom rendering"))
        .size([350.0, 560.0], Condition::FirstUseEver)
        .opened(opened)
        .build(|| {
            ui.text("Primitives");
            // TODO: Add DragFloat to change value of sz
            ui.color_edit(im_str!("Color"), &mut state.col).build();
            let draw_list = ui.get_window_draw_list();
            let p = ui.get_cursor_screen_pos();
            let spacing = 8.0;
            let mut y = p[1] + 4.0;
            for n in 0..2 {
                let mut x = p[0] + 4.0;
                let thickness = if n == 0 { 1.0 } else { 4.0 };
                draw_list
                    .add_circle(
                        [x + state.sz * 0.5, y + state.sz * 0.5],
                        state.sz * 0.5,
                        state.col,
                    )
                    .num_segments(20)
                    .thickness(thickness)
                    .build();
                x += state.sz + spacing;
                draw_list
                    .add_rect([x, y], [x + state.sz, y + state.sz], state.col)
                    .thickness(thickness)
                    .build();
                x += state.sz + spacing;
                draw_list
                    .add_rect([x, y], [x + state.sz, y + state.sz], state.col)
                    .thickness(thickness)
                    .rounding(10.0)
                    .build();
                x += state.sz + spacing;
                draw_list
                    .add_rect([x, y], [x + state.sz, y + state.sz], state.col)
                    .thickness(thickness)
                    .rounding(10.0)
                    .round_top_right(false)
                    .round_bot_left(false)
                    .build();
                x += state.sz + spacing;
                draw_list
                    .add_triangle(
                        [x + state.sz * 0.5, y],
                        [x + state.sz, y + state.sz - 0.5],
                        [x, y + state.sz - 0.5],
                        state.col,
                    )
                    .thickness(thickness)
                    .build();
                x += state.sz + spacing;
                draw_list
                    .add_line([x, y], [x + state.sz, y], state.col)
                    .thickness(thickness)
                    .build();
                x += state.sz + spacing;
                draw_list
                    .add_line([x, y], [x + state.sz, y + state.sz], state.col)
                    .thickness(thickness)
                    .build();
                x += state.sz + spacing;
                draw_list
                    .add_line([x, y], [x, y + state.sz], state.col)
                    .thickness(thickness)
                    .build();
                x += spacing;
                draw_list
                    .add_bezier_curve(
                        [x, y],
                        [x + state.sz * 1.3, y + state.sz * 0.3],
                        [x + state.sz - state.sz * 1.3, y + state.sz - state.sz * 0.3],
                        [x + state.sz, y + state.sz],
                        state.col,
                    )
                    .thickness(thickness)
                    .build();
                y += state.sz + spacing;
            }
            let mut x = p[0] + 4.0;
            draw_list
                .add_circle(
                    [x + state.sz * 0.5, y + state.sz * 0.5],
                    state.sz * 0.5,
                    state.col,
                )
                .num_segments(32)
                .filled(true)
                .build();
            x += state.sz + spacing;
            draw_list
                .add_rect([x, y], [x + state.sz, y + state.sz], state.col)
                .filled(true)
                .build();
            x += state.sz + spacing;
            draw_list
                .add_rect([x, y], [x + state.sz, y + state.sz], state.col)
                .filled(true)
                .rounding(10.0)
                .build();
            x += state.sz + spacing;
            draw_list
                .add_rect([x, y], [x + state.sz, y + state.sz], state.col)
                .filled(true)
                .rounding(10.0)
                .round_top_right(false)
                .round_bot_left(false)
                .build();
            x += state.sz + spacing;
            draw_list
                .add_triangle(
                    [x + state.sz * 0.5, y],
                    [x + state.sz, y + state.sz - 0.5],
                    [x, y + state.sz - 0.5],
                    state.col,
                )
                .filled(true)
                .build();
            x += state.sz + spacing;
            const MULTICOLOR_RECT_CORNER_COLOR1: [f32; 3] = [0.0, 0.0, 0.0];
            const MULTICOLOR_RECT_CORNER_COLOR2: [f32; 3] = [1.0, 0.0, 0.0];
            const MULTICOLOR_RECT_CORNER_COLOR3: [f32; 3] = [1.0, 1.0, 0.0];
            const MULTICOLOR_RECT_CORNER_COLOR4: [f32; 3] = [0.0, 1.0, 0.0];
            draw_list.add_rect_filled_multicolor(
                [x, y],
                [x + state.sz, y + state.sz],
                MULTICOLOR_RECT_CORNER_COLOR1,
                MULTICOLOR_RECT_CORNER_COLOR2,
                MULTICOLOR_RECT_CORNER_COLOR3,
                MULTICOLOR_RECT_CORNER_COLOR4,
            );
            ui.dummy([(state.sz + spacing) * 8.0, (state.sz + spacing) * 3.0]);
            ui.separator();

            ui.text(im_str!("Canvas example"));
            if ui.button(im_str!("Clear"), [0.0, 0.0]) {
                state.points.clear();
            }
            if state.points.len() >= 2 {
                ui.same_line(0.0);
                if ui.button(im_str!("Undo"), [0.0, 0.0]) {
                    state.points.pop();
                    state.points.pop();
                }
            }
            ui.text(im_str!(
                "Left-click and drag to add lines,\nRight-click to undo"
            ));
            // Here we are using InvisibleButton() as a convenience to
            //  1) advance the cursor, and
            //  2) allows us to use IsItemHovered()
            // However you can draw directly and poll mouse/keyboard by
            // yourself. You can manipulate the cursor using GetCursorPos() and
            // SetCursorPos(). If you only use the ImDrawList API, you can
            // notify the owner window of its extends by using
            // SetCursorPos(max).

            // ImDrawList API uses screen coordinates!
            let canvas_pos = ui.get_cursor_screen_pos();
            // Resize canvas to what's available
            let mut canvas_size = ui.get_content_region_avail();
            if canvas_size[0] < 50.0 {
                canvas_size[0] = 50.0;
            }
            if canvas_size[1] < 50.0 {
                canvas_size[1] = 50.0;
            }
            const CANVAS_CORNER_COLOR1: [f32; 3] = [0.2, 0.2, 0.2];
            const CANVAS_CORNER_COLOR2: [f32; 3] = [0.2, 0.2, 0.24];
            const CANVAS_CORNER_COLOR3: [f32; 3] = [0.24, 0.24, 0.27];
            const CANVAS_CORNER_COLOR4: [f32; 3] = [0.2, 0.2, 0.24];
            draw_list.add_rect_filled_multicolor(
                canvas_pos,
                [
                    canvas_pos[0] + canvas_size[0],
                    canvas_pos[1] + canvas_size[1],
                ],
                CANVAS_CORNER_COLOR1,
                CANVAS_CORNER_COLOR2,
                CANVAS_CORNER_COLOR3,
                CANVAS_CORNER_COLOR4,
            );
            const CANVAS_BORDER_COLOR: [f32; 3] = [1.0, 1.0, 1.0];
            draw_list
                .add_rect(
                    canvas_pos,
                    [
                        canvas_pos[0] + canvas_size[0],
                        canvas_pos[1] + canvas_size[1],
                    ],
                    CANVAS_BORDER_COLOR,
                )
                .build();

            let mut adding_preview = false;
            ui.invisible_button(im_str!("canvas"), canvas_size);
            let mouse_pos = ui.io().mouse_pos;
            let mouse_pos_in_canvas = [mouse_pos[0] - canvas_pos[0], mouse_pos[1] - canvas_pos[1]];
            if state.adding_line {
                adding_preview = true;
                state.points.push(mouse_pos_in_canvas);
                if !ui.is_mouse_down(MouseButton::Left) {
                    state.adding_line = false;
                    adding_preview = false;
                }
            }
            if ui.is_item_hovered() {
                if !state.adding_line && ui.is_mouse_clicked(MouseButton::Left) {
                    state.points.push(mouse_pos_in_canvas);
                    state.adding_line = true;
                }
                if ui.is_mouse_clicked(MouseButton::Right) && !state.points.is_empty() {
                    state.adding_line = false;
                    adding_preview = false;
                    state.points.pop();
                    state.points.pop();
                }
            }
            draw_list.with_clip_rect_intersect(
                canvas_pos,
                [
                    canvas_pos[0] + canvas_size[0],
                    canvas_pos[1] + canvas_size[1],
                ],
                || {
                    const LINE_COLOR: [f32; 3] = [1.0, 1.0, 0.0];
                    for line in state.points.chunks(2) {
                        if line.len() < 2 {
                            break;
                        }
                        let (p1, p2) = (line[0], line[1]);
                        draw_list
                            .add_line(
                                [canvas_pos[0] + p1[0], canvas_pos[1] + p1[1]],
                                [canvas_pos[0] + p2[0], canvas_pos[1] + p2[1]],
                                LINE_COLOR,
                            )
                            .thickness(2.0)
                            .build();
                    }
                },
            );
            if adding_preview {
                state.points.pop();
            }
        });
}
