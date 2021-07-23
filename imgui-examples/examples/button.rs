use imgui::*;

mod support;

fn swap_button_text(show_button_text: &bool) -> bool
{
    if *show_button_text == false { return true; }
    else if *show_button_text == true { return false; }
    return false;
}

fn main() {
    let mut show_button_text = false;
    let system = support::init(file!());
    system.main_loop(move |_, ui| {
        Window::new(im_str!("Button Example"))
            .size([300.0, 110.0], Condition::FirstUseEver)
            .build(ui, || {
                let button_clicked = imgui::Button::new("Test", &50.0f32, &32.0f32).build();
                if button_clicked { show_button_text = swap_button_text(&show_button_text); }
                if show_button_text { ui.text(im_str!("You clicked a button!")); }
            });
    });
}
