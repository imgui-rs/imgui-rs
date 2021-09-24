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
    buf: String,
    item: usize,
    item2: usize,
    item3: i32,
    text: String,
    text_with_hint: String,
    text_multiline: String,
    i0: i32,
    f0: f32,
    vec2f: [f32; 2],
    vec3f: [f32; 3],
    vec2i: [i32; 2],
    vec3i: [i32; 3],
    col1: [f32; 3],
    col2: [f32; 4],
    selected_fish: Option<usize>,
    selected_fish2: Option<usize>,
    auto_resize_state: AutoResizeState,
    file_menu: FileMenuState,
    radio_button: i32,
    color_edit: ColorEditState,
    custom_rendering: CustomRenderingState,
    dont_ask_me_next_time: bool,
    stacked_modals_item: usize,
    stacked_modals_color: [f32; 4],
    app_log: Vec<String>,

    tabs: TabState,
}

impl Default for State {
    fn default() -> Self {
        let mut buf = String::with_capacity(32);
        buf.push_str("日本語");
        let mut text = String::with_capacity(128);
        text.push_str("Hello, world!");
        let text_with_hint = String::with_capacity(128);
        let mut text_multiline = String::with_capacity(128);
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
            buf,
            item: 0,
            item2: 0,
            item3: 0,
            text,
            text_with_hint,
            text_multiline,
            i0: 123,
            f0: 0.001,
            vec2f: [0.10, 0.20],
            vec3f: [0.10, 0.20, 0.30],
            vec2i: [10, 20],
            vec3i: [10, 20, 30],
            col1: [1.0, 0.0, 0.2],
            col2: [0.4, 0.7, 0.0, 0.5],
            selected_fish: None,
            selected_fish2: None,
            auto_resize_state: Default::default(),
            file_menu: Default::default(),
            radio_button: 0,
            color_edit: ColorEditState::default(),
            custom_rendering: Default::default(),
            dont_ask_me_next_time: false,
            stacked_modals_item: 0,
            stacked_modals_color: [0.4, 0.7, 0.0, 0.5],
            app_log: Vec::new(),
            tabs: TabState::default(),
        }
    }
}

struct TabState {
    // flags for the advanced tab example
    reorderable: bool,
    autoselect: bool,
    listbutton: bool,
    noclose_middlebutton: bool,
    fitting_resizedown: bool,
    fitting_scroll: bool,

    // opened state for tab items
    artichoke_tab: bool,
    beetroot_tab: bool,
    celery_tab: bool,
    daikon_tab: bool,
}

impl Default for TabState {
    fn default() -> Self {
        Self {
            reorderable: true,
            autoselect: false,
            listbutton: false,
            noclose_middlebutton: false,
            fitting_resizedown: true,
            fitting_scroll: false,

            artichoke_tab: true,
            beetroot_tab: true,
            celery_tab: true,
            daikon_tab: true,
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
    n: usize,
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
    system.main_loop(move |run, ui| show_test_window(ui, &mut state, run));
}

fn show_help_marker(ui: &Ui, desc: &str) {
    ui.text_disabled("(?)");
    if ui.is_item_hovered() {
        ui.tooltip(|| {
            ui.text(desc);
        });
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
  Use +- to subtract.\n",
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

    if state.show_app_log {
        show_app_log(ui, &mut state.app_log);
    }

    let mut window = ui
        .window("ImGui Demo")
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
        ui.push_item_width(-140.0);
        ui.text(format!("dear imgui says hello. ({})", imgui::dear_imgui_version()));
        if let Some(menu_bar) = ui.begin_menu_bar() {
            if let Some(menu) = ui.begin_menu("Menu") {
                show_example_menu_file(ui, &mut state.file_menu);
                menu.end();
            }
            if let Some(menu) = ui.begin_menu("Examples") {
                MenuItem::new("Main menu bar")
                    .build_with_ref(ui, &mut state.show_app_main_menu_bar);
                MenuItem::new("Console")
                    .build_with_ref(ui, &mut state.show_app_console);
                MenuItem::new("Log")
                    .build_with_ref(ui, &mut state.show_app_log);
                MenuItem::new("Simple layout")
                    .build_with_ref(ui, &mut state.show_app_layout);
                MenuItem::new("Property editor")
                    .build_with_ref(ui, &mut state.show_app_property_editor);
                MenuItem::new("Long text display")
                    .build_with_ref(ui, &mut state.show_app_long_text);
                MenuItem::new("Auto-resizing window")
                    .build_with_ref(ui, &mut state.show_app_auto_resize);
                MenuItem::new("Constrained-resizing window")
                    .build_with_ref(ui, &mut state.show_app_constrained_resize);
                MenuItem::new("Simple overlay")
                    .build_with_ref(ui, &mut state.show_app_fixed_overlay);
                MenuItem::new("Manipulating window title")
                    .build_with_ref(ui, &mut state.show_app_manipulating_window_title);
                MenuItem::new("Custom rendering")
                    .build_with_ref(ui, &mut state.show_app_custom_rendering);
                menu.end();
            }
            if let Some(menu) = ui.begin_menu("Help") {
                MenuItem::new("Metrics")
                    .build_with_ref(ui, &mut state.show_app_metrics);
                MenuItem::new("Style Editor")
                    .build_with_ref(ui, &mut state.show_app_style_editor);
                MenuItem::new("About ImGui")
                    .build_with_ref(ui, &mut state.show_app_about);
                menu.end();
            }
            menu_bar.end();
        }
        ui.spacing();
        if CollapsingHeader::new("Help").build(ui) {
            ui.text_wrapped(
                "This window is being created by the show_test_window() \
                 function. Please refer to the code for programming \
                 reference.\n\nUser Guide:"
            );
            show_user_guide(ui);
        }

        if CollapsingHeader::new("Window options").build(ui) {
            ui.checkbox("No titlebar", &mut state.no_titlebar);
            ui.same_line_with_pos(150.0);
            ui.checkbox("No scrollbar", &mut state.no_scrollbar);
            ui.same_line_with_pos(300.0);
            ui.checkbox("No menu", &mut state.no_menu);
            ui.checkbox("No move", &mut state.no_move);
            ui.same_line_with_pos(150.0);
            ui.checkbox("No resize", &mut state.no_resize);
            ui.same_line_with_pos(300.0);
            ui.checkbox("No collapse", &mut state.no_collapse);
            ui.checkbox("No close", &mut state.no_close);

            TreeNode::new("Style").build(ui, || {
                ui.show_default_style_editor();
            });
        }
        if CollapsingHeader::new("Widgets").build(ui) {
            TreeNode::new("Tree").build(ui, || {
                for i in 0..5 {
                    TreeNode::new(format!("Child {}", i)).build(ui, || {
                        ui.text("blah blah");
                        ui.same_line();
                        if ui.small_button("print") {
                            println!("Child {} pressed", i);
                        }
                    });
                }
            });

            TreeNode::new("Bullets").build(ui, || {
                ui.bullet_text("Bullet point 1");
                ui.bullet_text("Bullet point 2\nOn multiple lines");
                ui.bullet();
                ui.text("Bullet point 3 (two calls)");

                ui.bullet();
                ui.small_button("Button");
            });
            TreeNode::new("Colored text").build(ui, || {
                ui.text_colored([1.0, 0.0, 1.0, 1.0], "Pink");
                ui.text_colored([1.0, 1.0, 0.0, 1.0], "Yellow");
                ui.text_disabled("Disabled");
            });

            TreeNode::new("Multi-line text").build(ui, || {
                ui.input_text_multiline(
                    "multiline",
                    &mut state.text_multiline,
                    [300., 100.],
                ).build();
            });

            TreeNode::new("Word wrapping").build(ui, || {
                ui.text_wrapped(
                    "This text should automatically wrap on the edge of \
                     the window.The current implementation for text \
                     wrapping follows simple rulessuitable for English \
                     and possibly other languages."
                );
                ui.spacing();

                Slider::new("Wrap width", -20.0, 600.0)
                    .display_format("%.0f")
                    .build(ui, &mut state.wrap_width);

                ui.text("Test paragraph 1:");
                // TODO

                ui.text("Test paragraph 2:");
                // TODO
            });
            TreeNode::new("UTF-8 Text").build(ui, || {
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
            ui.same_line();
            ui.radio_button("radio b", &mut state.radio_button, 1);
            ui.same_line();
            ui.radio_button("radio c", &mut state.radio_button, 2);

            ui.separator();
            ui.label_text("label", "Value");
            ui.combo_simple_string("combo",
                &mut state.item,
                &[
                    "aaaa",
                    "bbbb",
                    "cccc",
                    "dddd",
                    "eeee",
                ]);
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
            ui.combo_simple_string("combo scroll", &mut state.item2, &items);
            ui.list_box("list", &mut state.item3, &items, 8);


            let names = [
                "Bream",
                "Haddock",
                "Mackerel",
                "Pollock",
                "Tilefish",
            ];

            ListBox::new("selectables list").build(ui, || {
                for (index, name) in names.iter().enumerate() {
                    let selected = matches!(state.selected_fish2, Some(i) if i == index );
                    if Selectable::new(name).selected(selected).build(ui) {
                        state.selected_fish2 = Some(index);
                    }
                }
            });

            let last_size = ui.item_rect_size();
            ListBox::new("selectable list 2").size([0.0, last_size[1] * 0.66]).build(ui, || {
                for (index, name) in names.iter().enumerate() {
                    let selected = matches!(state.selected_fish2, Some(i) if i == index );
                    if Selectable::new(name).selected(selected).build(ui) {
                        state.selected_fish2 = Some(index);
                    }
                }
            });

            ui.input_text("input text", &mut state.text)
                .build();
            ui.input_text("input text with hint", &mut state.text_with_hint)
                .hint("enter text here")
                .build();
            ui.input_int("input int", &mut state.i0).build();
            // Drag::new("drag int").build(ui, &mut state.i0);
            ui.input_float("input float", &mut state.f0)
                .step(0.01)
                .step_fast(1.0)
                .build();
            Drag::new("drag float").range(-1.0, 1.0).speed(0.001).build(ui, &mut state.f0);
            ui.input_float3("input float3", &mut state.vec3f)
                .build();
            ColorEdit3::new("color 1", &mut state.col1).build(ui);
            ColorEdit3::new("color 2", &mut state.col2).build(ui);

            TreeNode::new("Multi-component Widgets").build(ui, || {
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

            TreeNode::new("Color/Picker Widgets").build(ui, || {
                let s = &mut state.color_edit;
                ui.checkbox("With HDR", &mut s.hdr);
                ui.same_line();
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
                ui.same_line();
                show_help_marker(
                    ui,
                    "Right-click on the individual color widget to \
                     show options.",
                );
                let misc_flags = {
                    let mut f = ColorEditFlags::empty();
                    f.set(ColorEditFlags::HDR, s.hdr);
                    f.set(ColorEditFlags::ALPHA_PREVIEW_HALF, s.alpha_half_preview);
                    if !s.alpha_half_preview {
                        f.set(ColorEditFlags::ALPHA_PREVIEW, s.alpha_preview);
                    }
                    f.set(ColorEditFlags::NO_OPTIONS, !s.options_menu);
                    f
                };

                ui.text("Color widget:");
                ui.same_line();
                show_help_marker(
                    ui,
                    "Click on the colored square to open a color picker.
CTRL+click on individual component to input value.\n",
                );
                ColorEdit3::new("MyColor##1", &mut s.color)
                    .flags(misc_flags)
                    .alpha(false)
                    .build(ui);

                ui.text("Color widget HSV with Alpha:");
                ColorEdit3::new("MyColor##2", &mut s.color)
                    .flags(misc_flags)
                    .input_mode(ColorEditInputMode::HSV)
                    .build(ui);

                ui.text("Color widget with Float Display:");
                ColorEdit3::new("MyColor##2f", &mut s.color)
                    .flags(misc_flags)
                    .format(ColorFormat::Float)
                    .build(ui);

                ui.text("Color button with Picker:");
                ui.same_line();
                show_help_marker(
                    ui,
                    "With the inputs(false) function you can hide all \
                     the slider/text inputs.\n \
                     With the label(false) function you can pass a non-empty label which \
                     will only be used for the tooltip and picker popup.",
                );
                ColorEdit3::new("MyColor##3", &mut s.color)
                    .flags(misc_flags)
                    .inputs(false)
                    .label(false)
                    .build(ui);

                ui.text("Color picker:");
                ui.checkbox("With Alpha", &mut s.alpha);
                ui.checkbox("With Alpha Bar", &mut s.alpha_bar);
                ui.checkbox("With Side Preview", &mut s.side_preview);
                if s.side_preview {
                    ui.same_line();
                    ui.checkbox("With Ref Color", &mut s.ref_color);
                    if s.ref_color {
                        ui.same_line();
                        ColorEdit3::new("##RefColor", &mut s.ref_color_v)
                            .flags(misc_flags)
                            .inputs(false)
                            .build(ui);
                    }
                }
                let mut b = ColorPicker3::new
                    ("MyColor##4", &mut s.color)
                    .flags(misc_flags)
                    .alpha(s.alpha)
                    .alpha_bar(s.alpha_bar)
                    .side_preview(s.side_preview)
                    .display_rgb(true);

                if s.ref_color {
                    b = b.reference_color(&s.ref_color_v)
                }
                b.build(ui);
            });
        }

        if CollapsingHeader::new("Layout").build(ui) {
            TreeNode::new("Tabs").build(ui, || {
                TreeNode::new("Basic").build(ui, || {
                    TabBar::new("basictabbar").build(ui, || {
                        TabItem::new("Avocado").build(ui, || {
                            ui.text("This is the Avocado tab!");
                            ui.text("blah blah blah blah blah");
                        });
                        TabItem::new("Broccoli").build(ui, || {
                            ui.text("This is the Broccoli tab!");
                            ui.text("blah blah blah blah blah");
                        });
                        TabItem::new("Cucumber").build(ui, || {
                            ui.text("This is the Cucumber tab!");
                            ui.text("blah blah blah blah blah");
                        });
                    });

                });
                TreeNode::new("Advanced & Close button").build(ui, || {

                    ui.separator();
                    let s = &mut state.tabs;

                    ui.checkbox("ImGuiTabBarFlags_Reorderable", &mut s.reorderable);
                    ui.checkbox("ImGuiTabBarFlags_AutoSelectNewTabs", &mut s.autoselect);
                    ui.checkbox("ImGuiTabBarFlags_TabListPopupButton", &mut s.listbutton);
                    ui.checkbox("ImGuiTabBarFlags_NoCloseWithMiddleMouseButton", &mut s.noclose_middlebutton);
                    if ui.checkbox("ImGuiTabBarFlags_FittingPolicyResizeDown", &mut s.fitting_resizedown) {
                        s.fitting_scroll = !s.fitting_resizedown;
                    }
                    if ui.checkbox("ImGuiTabBarFlags_FittingPolicyScroll", &mut s.fitting_scroll) {
                        s.fitting_resizedown = !s.fitting_scroll;
                    }
                    let style = ui.push_style_var(StyleVar::FramePadding([0.0, 0.0]));
                    ui.checkbox("Artichoke", &mut s.artichoke_tab);
                    ui.same_line();
                    ui.checkbox("Beetroot", &mut s.beetroot_tab);
                    ui.same_line();
                    ui.checkbox("Celery", &mut s.celery_tab);
                    ui.same_line();
                    ui.checkbox("Daikon", &mut s.daikon_tab);
                    style.pop();

                    let flags = {
                        let mut f = TabBarFlags::empty();
                        f.set(TabBarFlags::REORDERABLE, s.reorderable);
                        f.set(TabBarFlags::AUTO_SELECT_NEW_TABS, s.autoselect);
                        f.set(TabBarFlags::TAB_LIST_POPUP_BUTTON, s.listbutton);
                        f.set(TabBarFlags::NO_CLOSE_WITH_MIDDLE_MOUSE_BUTTON, s.noclose_middlebutton);
                        f.set(TabBarFlags::FITTING_POLICY_RESIZE_DOWN, s.fitting_resizedown);
                        f.set(TabBarFlags::FITTING_POLICY_SCROLL, s.fitting_scroll);
                        f
                    };

                    TabBar::new("tabbar").flags(flags).build(ui, || {
                        TabItem::new("Artichoke").opened(&mut s.artichoke_tab).build(ui, || {
                            ui.text("This is the Artichoke tab!");
                        });
                        TabItem::new("Beetroot").opened(&mut s.beetroot_tab).build(ui, || {
                            ui.text("This is the Beetroot tab!");
                        });
                        TabItem::new("Celery").opened(&mut s.celery_tab).build(ui, || {
                            ui.text("This is the Celery tab!");
                        });
                        TabItem::new("Daikon").opened(&mut s.daikon_tab).build(ui, || {
                            ui.text("This is the Daikon tab!");
                        });
                    });

                });
            });
        }
        if CollapsingHeader::new("Popups & Modal windows").build(ui) {
            TreeNode::new("Popups").build(ui, || {
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
                ui.same_line();
                ui.text(match state.selected_fish {
                    Some(index) => names[index],
                    None => "<None>",
                });
                ui.popup("select", || {
                    ui.text("Aquarium");
                    ui.separator();
                    for (index, name) in names.iter().enumerate() {
                        if Selectable::new(name).build(ui) {
                            state.selected_fish = Some(index);
                        }
                    }
                });
            });

            TreeNode::new("Modals").build(ui, || {
                ui.text_wrapped(
                    "Modal windows are like popups but the user cannot close \
                     them by clicking outside the window."
                );

                if ui.button("Delete..") {
                    ui.open_popup("Delete?");
                }
                PopupModal::new("Delete?").always_auto_resize(true).build(ui, || {
                    ui.text("All those beautiful files will be deleted.\nThis operation cannot be undone!\n\n");
                    ui.separator();
                    let style = ui.push_style_var(StyleVar::FramePadding([0.0, 0.0]));
                    ui.checkbox("Don't ask me next time", &mut state.dont_ask_me_next_time);

                    if ui.button_with_size("OK", [120.0, 0.0]) {
                        ui.close_current_popup();
                    }
                    ui.same_line();
                    if ui.button_with_size("Cancel", [120.0, 0.0]) {
                        ui.close_current_popup();
                    }
                    style.pop();
                });

                if ui.button("Stacked modals..") {
                    ui.open_popup("Stacked 1");
                }
                PopupModal::new("Stacked 1").build(ui, || {
                    ui.text(
                       "Hello from Stacked The First\n\
                        Using style[StyleColor::ModalWindowDarkening] for darkening."
                    );

                    let items = &["aaaa", "bbbb", "cccc", "dddd", "eeee"];
                    ui.combo_simple_string("Combo", &mut state.stacked_modals_item, items);

                    ColorEdit3::new("color", &mut state.stacked_modals_color).build(ui);

                    if ui.button("Add another modal..") {
                        ui.open_popup("Stacked 2")   ;
                    }
                    PopupModal::new("Stacked 2").build(ui, || {
                        ui.text("Hello from Stacked The Second");
                        if ui.button("Close") {
                            ui.close_current_popup();
                        }
                    });

                    if ui.button("Close") {
                        ui.close_current_popup();
                    }
                });
            });
        }
    });
}

fn show_example_app_main_menu_bar<'a>(ui: &Ui<'a>, state: &mut State) {
    if let Some(menu_bar) = ui.begin_main_menu_bar() {
        if let Some(menu) = ui.begin_menu("File") {
            show_example_menu_file(ui, &mut state.file_menu);
            menu.end();
        }
        if let Some(menu) = ui.begin_menu("Edit") {
            MenuItem::new("Undo").shortcut("CTRL+Z").build(ui);
            MenuItem::new("Redo")
                .shortcut("CTRL+Y")
                .enabled(false)
                .build(ui);
            ui.separator();
            MenuItem::new("Cut").shortcut("CTRL+X").build(ui);
            MenuItem::new("Copy").shortcut("CTRL+C").build(ui);
            MenuItem::new("Paste").shortcut("CTRL+V").build(ui);
            menu.end();
        }
        menu_bar.end();
    }
}

fn show_example_menu_file<'a>(ui: &Ui<'a>, state: &mut FileMenuState) {
    MenuItem::new("(dummy menu)").enabled(false).build(ui);
    MenuItem::new("New").build(ui);
    MenuItem::new("Open").shortcut("Ctrl+O").build(ui);
    if let Some(menu) = ui.begin_menu("Open Recent") {
        MenuItem::new("fish_hat.c").build(ui);
        MenuItem::new("fish_hat.inl").build(ui);
        MenuItem::new("fish_hat.h").build(ui);
        if let Some(menu) = ui.begin_menu("More..") {
            MenuItem::new("Hello").build(ui);
            MenuItem::new("Sailor").build(ui);
            if let Some(menu) = ui.begin_menu("Recurse..") {
                show_example_menu_file(ui, state);
                menu.end();
            }
            menu.end();
        }
        menu.end();
    }
    MenuItem::new("Save").shortcut("Ctrl+S").build(ui);
    MenuItem::new("Save As..").build(ui);
    ui.separator();
    if let Some(menu) = ui.begin_menu("Options") {
        MenuItem::new("Enabled").build_with_ref(ui, &mut state.enabled);
        ui.child_window("child")
            .size([0.0, 60.0])
            .border(true)
            .build(|| {
                for i in 0..10 {
                    ui.text(format!("Scrolling Text {}", i));
                }
            });
        Slider::new("Value", 0.0, 1.0).build(ui, &mut state.f);

        ui.input_float("Input", &mut state.f).step(0.1).build();
        let items = ["Yes", "No", "Maybe"];
        ui.combo_simple_string("Combo", &mut state.n, &items);
        ui.checkbox("Check", &mut state.b);
        menu.end();
    }
    if let Some(menu) = ui.begin_menu("Colors") {
        for &col in StyleColor::VARIANTS.iter() {
            MenuItem::new(format!("{:?}", col)).build(ui);
        }
        menu.end();
    }
    assert!(ui.begin_menu_with_enabled("Disabled", false).is_none());
    MenuItem::new("Checked").selected(true).build(ui);
    MenuItem::new("Quit").shortcut("Alt+F4").build(ui);
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
            Slider::new("Number of lines", 1, 20).build(ui, &mut state.lines);
            for i in 0..state.lines {
                ui.text(format!("{:2$}This is line {}", "", i, i as usize * 4));
            }
        });
}

fn show_example_app_fixed_overlay(ui: &Ui, opened: &mut bool) {
    const DISTANCE: f32 = 10.0;
    let window_pos = [DISTANCE, DISTANCE];
    let style = ui.push_style_color(StyleColor::WindowBg, [0.0, 0.0, 0.0, 0.3]);
    ui.window("Example: Fixed Overlay")
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
        });
    style.pop();
}

fn show_example_app_manipulating_window_title(ui: &Ui) {
    ui.window("Same title as another window##1")
        .position([100.0, 100.0], Condition::FirstUseEver)
        .build(|| {
            ui.text(
                "This is window 1.
My title is the same as window 2, but my identifier is unique.",
            );
        });
    ui.window("Same title as another window##2")
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
    let title = format!("Animated title {} {}###AnimatedTitle", chars[ch_idx], num);
    ui.window(title)
        .position([100.0, 300.0], Condition::FirstUseEver)
        .build(|| ui.text("This window has a changing title"));
}

fn show_example_app_custom_rendering(ui: &Ui, state: &mut CustomRenderingState, opened: &mut bool) {
    ui.window("Example: Custom rendering")
        .size([350.0, 560.0], Condition::FirstUseEver)
        .opened(opened)
        .build(|| {
            ui.text("Primitives");
            // TODO: Add DragFloat to change value of sz
            ColorEdit3::new("Color", &mut state.col).build(ui);
            let draw_list = ui.get_window_draw_list();
            let p = ui.cursor_screen_pos();
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

            ui.text("Canvas example");
            if ui.button("Clear") {
                state.points.clear();
            }
            if state.points.len() >= 2 {
                ui.same_line();
                if ui.button("Undo") {
                    state.points.pop();
                    state.points.pop();
                }
            }
            ui.text("Left-click and drag to add lines,\nRight-click to undo");
            // Here we are using InvisibleButton() as a convenience to
            //  1) advance the cursor, and
            //  2) allows us to use IsItemHovered()
            // However you can draw directly and poll mouse/keyboard by
            // yourself. You can manipulate the cursor using GetCursorPos() and
            // SetCursorPos(). If you only use the ImDrawList API, you can
            // notify the owner window of its extends by using
            // SetCursorPos(max).

            // ImDrawList API uses screen coordinates!
            let canvas_pos = ui.cursor_screen_pos();
            // Resize canvas to what's available
            let mut canvas_size = ui.content_region_avail();
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
            ui.invisible_button("canvas", canvas_size);
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

fn show_app_log(ui: &Ui, app_log: &mut Vec<String>) {
    ui.window("Example: Log")
        .size([500.0, 400.0], Condition::FirstUseEver)
        .build(|| {
            if ui.small_button("[Debug] Add 5 entries") {
                let categories = ["info", "warn", "error"];
                let words = [
                    "Bumfuzzled",
                    "Cattywampus",
                    "Snickersnee",
                    "Abibliophobia",
                    "Absquatulate",
                    "Nincompoop",
                    "Pauciloquent",
                ];
                for _ in 0..5 {
                    let category = categories[app_log.len() % categories.len()];
                    let word = words[app_log.len() % words.len()];
                    let frame = ui.frame_count();
                    let time = ui.time();
                    let text = format!(
                        "{:05} {} Hello, current time is {:.1}, here's a word: {}",
                        frame, category, time, word
                    );
                    app_log.push(text);
                }
            }
            if ui.button("Clear") {
                app_log.clear();
            }
            ui.same_line();
            if ui.button("Copy") {
                ui.set_clipboard_text(&ImString::from(app_log.join("\n")));
            }
            ui.separator();
            ui.child_window("logwindow")
                .flags(WindowFlags::HORIZONTAL_SCROLLBAR)
                .build(|| {
                    if !app_log.is_empty() {
                        let mut clipper = ListClipper::new(app_log.len() as i32).begin(ui);
                        while clipper.step() {
                            for line in clipper.display_start()..clipper.display_end() {
                                ui.text(&app_log[line as usize]);
                            }
                        }
                    }
                    if ui.scroll_y() >= ui.scroll_max_y() {
                        ui.set_scroll_here_y();
                    }
                });
        });
}
