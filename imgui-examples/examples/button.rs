use imgui::*;

mod support;

fn swap_button_text(show_button_text: &bool) -> bool
{
    if *show_button_text == false { return true; }
    else if *show_button_text == true { return false; }
    return false;
}

fn main() {

    let default_width: &f32 = &50.0;
    let default_height: &f32 = &32.0;

    let mut show_button_text = false;
    let system = support::init(file!());
    system.main_loop(move |_, ui| {
        Window::new(im_str!("Button Example"))
            .size([300.0, 110.0], Condition::FirstUseEver)
            .build(ui, || {
                let basic_button_clicked = imgui::Button::new("Test", default_width, default_height).build();
                if basic_button_clicked { show_button_text = swap_button_text(&show_button_text); }
                if show_button_text { ui.text(im_str!("You clicked a button!")); }
            });
    });
}
