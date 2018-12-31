fn main() {
    cc::Build::new()
        .cpp(true)
        .file("third-party/cimgui/cimgui.cpp")
        .file("third-party/cimgui/imgui/imgui.cpp")
        .file("third-party/cimgui/imgui/imgui_demo.cpp")
        .file("third-party/cimgui/imgui/imgui_draw.cpp")
        .file("third-party/cimgui/imgui/imgui_widgets.cpp")
        .compile("libcimgui.a");
}
