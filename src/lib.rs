#[cfg(feature = "glium")]
#[macro_use]
extern crate glium;

extern crate imgui_sys;

extern crate libc;

use libc::{c_char, c_float, c_int, c_uchar, c_void, uintptr_t};
use std::borrow::Cow;
use std::convert::From;
use std::ffi::CStr;
use std::mem;
use std::ptr;
use std::slice;
use std::str;

pub use imgui_sys::{
    ImDrawIdx, ImDrawVert,
    ImGuiInputTextFlags, ImGuiInputTextFlags_CharsDecimal, ImGuiInputTextFlags_CharsHexadecimal,
    ImGuiInputTextFlags_CharsUppercase, ImGuiInputTextFlags_CharsNoBlank,
    ImGuiInputTextFlags_AutoSelectAll, ImGuiInputTextFlags_EnterReturnsTrue,
    ImGuiInputTextFlags_CallbackCompletion, ImGuiInputTextFlags_CallbackHistory,
    ImGuiInputTextFlags_CallbackAlways, ImGuiInputTextFlags_CallbackCharFilter,
    ImGuiInputTextFlags_AllowTabInput, ImGuiInputTextFlags_CtrlEnterForNewLine,
    ImGuiInputTextFlags_NoHorizontalScroll, ImGuiInputTextFlags_AlwaysInsertMode,
    ImGuiInputTextFlags_ReadOnly,
    ImGuiInputTextFlags_Password,
    ImGuiSetCond,
    ImGuiSetCond_Always, ImGuiSetCond_Once,
    ImGuiSetCond_FirstUseEver, ImGuiSetCond_Appearing,
    ImGuiStyle,
    ImGuiWindowFlags,
    ImGuiWindowFlags_NoTitleBar, ImGuiWindowFlags_NoResize, ImGuiWindowFlags_NoMove,
    ImGuiWindowFlags_NoScrollbar, ImGuiWindowFlags_NoScrollWithMouse, ImGuiWindowFlags_NoCollapse,
    ImGuiWindowFlags_AlwaysAutoResize, ImGuiWindowFlags_ShowBorders,
    ImGuiWindowFlags_NoSavedSettings, ImGuiWindowFlags_NoInputs, ImGuiWindowFlags_MenuBar,
    ImGuiWindowFlags_HorizontalScrollbar, ImGuiWindowFlags_NoFocusOnAppearing,
    ImGuiWindowFlags_NoBringToFrontOnFocus,
    ImVec2, ImVec4,
    ImGuiKey
};
pub use input::{InputText};
pub use menus::{Menu, MenuItem};
pub use sliders::{SliderFloat, SliderInt};
pub use trees::{TreeNode};
pub use widgets::{CollapsingHeader};
pub use window::{Window};

mod input;
mod menus;
mod sliders;
mod trees;
mod widgets;
mod window;

#[cfg(feature = "glium")]
pub mod glium_renderer;

pub struct ImGui {
    // We need to keep ownership of the ImStr values to ensure the *const char pointer
    // lives long enough in case the ImStr contains a Cow::Owned
    ini_filename: Option<ImStr<'static>>,
    log_filename: Option<ImStr<'static>>
}

#[macro_export]
macro_rules! im_str {
    ($e:tt) => ({
        let value = concat!($e, "\0");
        unsafe { ::imgui::ImStr::from_bytes_unchecked(value.as_bytes()) }
    });
    ($e:tt, $($arg:tt)*) => ({
        ::imgui::ImStr::from(format!($e, $($arg)*))
    })
}

#[derive(Clone)]
pub struct ImStr<'a> {
    bytes: Cow<'a, [u8]>
}

impl<'a> ImStr<'a> {
    pub unsafe fn from_bytes_unchecked(bytes: &'a [u8]) -> ImStr<'a> {
        ImStr {
            bytes: Cow::Borrowed(bytes)
        }
    }
    pub fn as_ptr(&self) -> *const c_char { self.bytes.as_ptr() as *const c_char }
}

impl<'a> From<&'a str> for ImStr<'a> {
    fn from(value: &'a str) -> ImStr<'a> {
        let mut bytes: Vec<u8> = value.bytes().collect();
        bytes.push(0);
        ImStr {
            bytes: Cow::Owned(bytes)
        }
    }
}

impl From<String> for ImStr<'static> {
    fn from(mut value: String) -> ImStr<'static> {
        value.push('\0');
        ImStr {
            bytes: Cow::Owned(value.into_bytes())
        }
    }
}

pub struct TextureHandle<'a> {
    pub width: u32,
    pub height: u32,
    pub pixels: &'a [c_uchar]
}

pub fn get_version() -> &'static str {
    unsafe {
        let bytes = CStr::from_ptr(imgui_sys::igGetVersion()).to_bytes();
        str::from_utf8_unchecked(bytes)
    }
}

impl ImGui {
    pub fn init() -> ImGui {
        ImGui {
            ini_filename: None,
            log_filename: None
        }
    }
    fn io(&self) -> &imgui_sys::ImGuiIO {
        unsafe { mem::transmute(imgui_sys::igGetIO()) }
    }
    fn io_mut(&mut self) -> &mut imgui_sys::ImGuiIO {
        unsafe { mem::transmute(imgui_sys::igGetIO()) }
    }
    pub fn style(&self) -> &ImGuiStyle {
        unsafe { mem::transmute(imgui_sys::igGetStyle()) }
    }
    pub fn style_mut(&self) -> &mut ImGuiStyle {
        unsafe { mem::transmute(imgui_sys::igGetStyle()) }
    }
    pub fn prepare_texture<'a, F, T>(&mut self, f: F) -> T where F: FnOnce(TextureHandle<'a>) -> T {
        let io = self.io();
        let mut pixels: *mut c_uchar = ptr::null_mut();
        let mut width: c_int = 0;
        let mut height: c_int = 0;
        let mut bytes_per_pixel: c_int = 0;
        unsafe {
            imgui_sys::ImFontAtlas_GetTexDataAsRGBA32(io.fonts, &mut pixels, &mut width, &mut height, &mut bytes_per_pixel);
            f(TextureHandle {
                width: width as u32,
                height: height as u32,
                pixels: slice::from_raw_parts(pixels, (width * height * bytes_per_pixel) as usize)
            })
        }
    }
    pub fn set_texture_id(&mut self, value: uintptr_t) {
        unsafe {
            (*self.io_mut().fonts).tex_id = value as *mut c_void;
        }
    }
    pub fn set_ini_filename(&mut self, value: Option<ImStr<'static>>) {
        {
            let io = self.io_mut();
            io.ini_filename = match value {
               Some(ref x) => x.as_ptr(),
               None => ptr::null()
            }
        }
        self.ini_filename = value;
    }
    pub fn set_log_filename(&mut self, value: Option<ImStr<'static>>) {
        {
            let io = self.io_mut();
            io.log_filename = match value {
               Some(ref x) => x.as_ptr(),
               None => ptr::null()
            }
        }
        self.log_filename = value;
    }
    pub fn set_ini_saving_rate(&mut self, value: f32) {
        let io = self.io_mut();
        io.ini_saving_rate = value;
    }
    pub fn set_mouse_double_click_time(&mut self, value: f32) {
        let io = self.io_mut();
        io.mouse_double_click_time = value;
    }
    pub fn set_mouse_double_click_max_dist(&mut self, value: f32) {
        let io = self.io_mut();
        io.mouse_double_click_max_dist = value;
    }
    pub fn set_mouse_drag_threshold(&mut self, value: f32) {
        let io = self.io_mut();
        io.mouse_drag_threshold = value;
    }
    pub fn set_key_repeat_delay(&mut self, value: f32) {
        let io = self.io_mut();
        io.key_repeat_delay = value;
    }
    pub fn set_key_repeat_rate(&mut self, value: f32) {
        let io = self.io_mut();
        io.key_repeat_rate = value;
    }
    pub fn mouse_pos(&self) -> (f32, f32) {
        let io = self.io();
        (io.mouse_pos.x, io.mouse_pos.y)
    }
    pub fn set_mouse_pos(&mut self, x: f32, y: f32) {
        let io = self.io_mut();
        io.mouse_pos.x = x;
        io.mouse_pos.y = y;
    }
    pub fn set_mouse_down(&mut self, states: &[bool; 5]) {
        let io = self.io_mut();
        io.mouse_down = *states;
    }
    pub fn set_mouse_wheel(&mut self, value: f32) {
        let io = self.io_mut();
        io.mouse_wheel = value;
    }
    pub fn set_mouse_draw_cursor(&mut self, value: bool) {
        let io = self.io_mut();
        io.mouse_draw_cursor = value;
    }
    pub fn set_key_ctrl(&mut self, value: bool) {
        let io = self.io_mut();
        io.key_ctrl = value;
    }
    pub fn set_key_shift(&mut self, value: bool) {
        let io = self.io_mut();
        io.key_shift = value;
    }
    pub fn set_key_alt(&mut self, value: bool) {
        let io = self.io_mut();
        io.key_alt = value;
    }
    pub fn set_key(&mut self, key: u8, pressed: bool) {
        let io = self.io_mut();
        io.keys_down[key as usize] = pressed;
    }
    pub fn set_imgui_key(&mut self, key: ImGuiKey, mapping: u8) {
        let io = self.io_mut();
        io.key_map[key as usize] = mapping as i32;
    }
    pub fn add_input_character(&mut self, character: char) {
        // TODO: This is slightly better. We should use char::encode_utf8 when it stabilizes
        // to allow us to skip the string intermediate since we can then go directly
        // to bytes
        let mut string = String::new();
        string.push(character);
        string.push('\0');
        unsafe {
            imgui_sys::ImGuiIO_AddInputCharactersUTF8(string.as_ptr() as *const i8);
        }
    }
    pub fn get_time(&self) -> f32 { unsafe { imgui_sys::igGetTime() } }
    pub fn get_frame_count(&self) -> i32 { unsafe { imgui_sys::igGetFrameCount() } }
    pub fn get_frame_rate(&self) -> f32 { self.io().framerate }
    pub fn frame<'ui, 'a: 'ui>(&'a mut self, width: u32, height: u32, delta_time: f32) -> Ui<'ui> {
        {
            let io = self.io_mut();
            io.display_size.x = width as c_float;
            io.display_size.y = height as c_float;
            io.delta_time = delta_time;
        }
        unsafe {
            imgui_sys::igNewFrame();
            CURRENT_UI = Some(Ui {
                imgui: mem::transmute(self as &'a ImGui)
            });
        }
        Ui {
            imgui: self
        }
    }
}

impl Drop for ImGui {
    fn drop(&mut self) {
        unsafe {
            CURRENT_UI = None;
            imgui_sys::igShutdown();
        }
    }
}

static mut CURRENT_UI: Option<Ui<'static>> = None;

pub struct DrawList<'a> {
    pub cmd_buffer: &'a [imgui_sys::ImDrawCmd],
    pub idx_buffer: &'a [imgui_sys::ImDrawIdx],
    pub vtx_buffer: &'a [imgui_sys::ImDrawVert]
}

pub struct Ui<'ui> {
    imgui: &'ui ImGui
}

static FMT: &'static [u8] = b"%s\0";

fn fmt_ptr() -> *const c_char { FMT.as_ptr() as *const c_char }

impl<'ui> Ui<'ui> {
    pub fn imgui(&self) -> &ImGui { self.imgui }
    pub fn want_capture_mouse(&self) -> bool {
        let io = self.imgui.io();
        io.want_capture_mouse
    }
    pub fn want_capture_keyboard(&self) -> bool {
        let io = self.imgui.io();
        io.want_capture_keyboard
    }
    pub fn framerate(&self) -> f32 {
        let io = self.imgui.io();
        io.framerate
    }
    pub fn metrics_allocs(&self) -> i32 {
        let io = self.imgui.io();
        io.metrics_allocs
    }
    pub fn metrics_render_vertices(&self) -> i32 {
        let io = self.imgui.io();
        io.metrics_render_vertices
    }
    pub fn metrics_render_indices(&self) -> i32 {
        let io = self.imgui.io();
        io.metrics_render_indices
    }
    pub fn metrics_active_windows(&self) -> i32 {
        let io = self.imgui.io();
        io.metrics_active_windows
    }
    pub fn render<F, E>(self, mut f: F) -> Result<(), E>
        where F: FnMut(DrawList<'ui>) -> Result<(), E> {
            unsafe {
                imgui_sys::igRender();

                let draw_data = imgui_sys::igGetDrawData();
                for &cmd_list in (*draw_data).cmd_lists() {
                    let draw_list =
                        DrawList {
                            cmd_buffer: (*cmd_list).cmd_buffer.as_slice(),
                            idx_buffer: (*cmd_list).idx_buffer.as_slice(),
                            vtx_buffer: (*cmd_list).vtx_buffer.as_slice()
                        };
                    try!(f(draw_list));
                }
                CURRENT_UI = None;
            }
            Ok(())
        }
    pub fn show_user_guide(&self) { unsafe { imgui_sys::igShowUserGuide() }; }
    pub fn show_default_style_editor(&self) {
        unsafe { imgui_sys::igShowStyleEditor(ptr::null_mut()) };
    }
    pub fn show_style_editor<'p>(&self, style: &'p mut ImGuiStyle) {
        unsafe {
            imgui_sys::igShowStyleEditor(style as *mut ImGuiStyle);
        }
    }
    pub fn show_test_window(&self, opened: &mut bool) {
        unsafe {
            imgui_sys::igShowTestWindow(opened);
        }
    }
    pub fn show_metrics_window(&self, opened: &mut bool) {
        unsafe {
            imgui_sys::igShowMetricsWindow(opened);
        }
    }
}

impl<'a> Ui<'a> {
    pub unsafe fn current_ui() -> Option<&'a Ui<'a>> {
        CURRENT_UI.as_ref()
    }
}

// Window
impl<'ui> Ui<'ui> {
    pub fn window<'p>(&self, name: ImStr<'p>) -> Window<'ui, 'p> { Window::new(name) }
}

// Layout
impl<'ui> Ui<'ui> {
    pub fn separator(&self) { unsafe { imgui_sys::igSeparator() }; }
    pub fn same_line(&self, pos_x: f32) {
        unsafe {
            imgui_sys::igSameLine(pos_x, -1.0f32)
        }
    }
    pub fn same_line_spacing(&self, pos_x: f32, spacing_w: f32) {
        unsafe {
            imgui_sys::igSameLine(pos_x, spacing_w)
        }
    }
    pub fn spacing(&self) { unsafe { imgui_sys::igSpacing() }; }
}

// Widgets
impl<'ui> Ui<'ui> {
    pub fn text<'p>(&self, text: ImStr<'p>) {
        // TODO: use igTextUnformatted
        unsafe {
            imgui_sys::igText(fmt_ptr(), text.as_ptr());
        }
    }
    pub fn text_colored<'p, A>(&self, col: A, text: ImStr<'p>) where A: Into<ImVec4> {
        unsafe {
            imgui_sys::igTextColored(col.into(), fmt_ptr(), text.as_ptr());
        }
    }
    pub fn text_disabled<'p>(&self, text: ImStr<'p>) {
        unsafe {
            imgui_sys::igTextDisabled(fmt_ptr(), text.as_ptr());
        }
    }
    pub fn text_wrapped<'p>(&self, text: ImStr<'p>) {
        unsafe {
            imgui_sys::igTextWrapped(fmt_ptr(), text.as_ptr());
        }
    }
    pub fn label_text<'p>(&self, label: ImStr<'p>, text: ImStr<'p>) {
        unsafe {
            imgui_sys::igLabelText(label.as_ptr(), fmt_ptr(), text.as_ptr());
        }
    }
    pub fn bullet(&self) {
        unsafe {
            imgui_sys::igBullet();
        }
    }
    pub fn bullet_text<'p>(&self, text: ImStr<'p>) {
        unsafe {
            imgui_sys::igBulletText(fmt_ptr(), text.as_ptr());
        }
    }
    pub fn small_button<'p>(&self, label: ImStr<'p>) -> bool {
        unsafe {
            imgui_sys::igSmallButton(label.as_ptr())
        }
    }
    pub fn collapsing_header<'p>(&self, label: ImStr<'p>) -> CollapsingHeader<'ui, 'p> {
        CollapsingHeader::new(label)
    }
    pub fn checkbox<'p>(&self, label: ImStr<'p>, value: &'p mut bool) -> bool {
        unsafe { imgui_sys::igCheckbox(label.as_ptr(), value) }
    }
}

// Widgets: Input
impl<'ui> Ui<'ui> {
    pub fn input_text<'p>(&self, label: ImStr<'p>, buf: &'p mut str) -> InputText<'ui, 'p> {
        InputText::new(label, buf)
    }
}

// Widgets: Sliders
impl<'ui> Ui<'ui> {
    pub fn slider_f32<'p>(&self, label: ImStr<'p>,
                          value: &'p mut f32, min: f32, max: f32) -> SliderFloat<'ui, 'p> {
        SliderFloat::new(label, value, min, max)
    }
    pub fn slider_i32<'p>(&self, label: ImStr<'p>,
                          value: &'p mut i32, min: i32, max: i32) -> SliderInt<'ui, 'p> {
        SliderInt::new(label, value, min, max)
    }
}

// Widgets: Trees
impl<'ui> Ui<'ui> {
    pub fn tree_node<'p>(&self, id: ImStr<'p>) -> TreeNode<'ui, 'p> {
        TreeNode::new(id)
    }
}

// Widgets: Menus
impl<'ui> Ui<'ui> {
    pub fn main_menu_bar<F>(&self, f: F) where F: FnOnce() {
        let render = unsafe { imgui_sys::igBeginMainMenuBar() };
        if render {
            f();
            unsafe { imgui_sys::igEndMainMenuBar() };
        }
    }
    pub fn menu_bar<F>(&self, f: F) where F: FnOnce() {
        let render = unsafe { imgui_sys::igBeginMenuBar() };
        if render {
            f();
            unsafe { imgui_sys::igEndMenuBar() };
        }
    }
    pub fn menu<'p>(&self, label: ImStr<'p>) -> Menu<'ui, 'p> { Menu::new(label) }
    pub fn menu_item<'p>(&self, label: ImStr<'p>) -> MenuItem<'ui, 'p> { MenuItem::new(label) }
}

//Widgets: Combos 
impl<'ui> Ui<'ui> {
    pub fn combo<'p>(&self, 
                     label: ImStr<'p>,
                     current_item: &mut i32,
                     items: &'p [ImStr<'p>],
                     height_in_items: i32) 
                     -> bool {
        let items_inner : Vec<*const c_char> = items.into_iter().map(|item| item.as_ptr()).collect();
        unsafe {
            imgui_sys::igCombo(label.as_ptr(),
                               current_item,
                               items_inner.as_ptr() as *mut *const c_char,
                               items_inner.len() as i32,
                               height_in_items)
        }
    }
}

//Widgets: ListBox
impl<'ui> Ui<'ui> {
    pub fn list_box<'p>(&self,
                        label: ImStr<'p>,
                        current_item: &mut i32,
                        items: &'p [ImStr<'p>],
                        height_in_items: i32)
                        -> bool{
        let items_inner : Vec<*const c_char> = items.into_iter().map(|item| item.as_ptr()).collect();
        unsafe{
            imgui_sys::igListBox(label.as_ptr(),
                                 current_item,
                                 items_inner.as_ptr() as *mut *const c_char,
                                 items_inner.len() as i32,
                                 height_in_items)
        }
    } 
}
