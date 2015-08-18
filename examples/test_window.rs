#[macro_use]
extern crate glium;
#[macro_use]
extern crate imgui;
extern crate time;

use imgui::Frame;

mod support;

fn main() {
    // let mut show_app_metrics = false;
    let show_app_main_menu_bar = true;

    support::main_with_frame(|frame| {
        // if show_app_metrics { show_metrics_window(&mut show_app_metrics) }
        if show_app_main_menu_bar { show_example_app_main_menu_bar(frame) }
    });
}

fn show_example_app_main_menu_bar<'a>(frame: &Frame<'a>) {
    frame.main_menu_bar(|| {
        frame.menu(im_str!("File")).build(|| {
            show_example_menu_file(frame);
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

fn show_example_menu_file<'a>(frame: &Frame<'a>) {
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
                show_example_menu_file(frame);
            });
        });
    });
    if frame.menu_item(im_str!("Save")).shortcut(im_str!("Ctrl+S")).build() { }
    if frame.menu_item(im_str!("Save As..")).build() { }
    frame.separator();
    frame.menu(im_str!("Options")).build(|| {
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
