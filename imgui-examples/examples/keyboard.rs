use imgui::*;

mod support;

fn main() {
    let system = support::init(file!());
    let mut press_counter = 0u32;
    let mut press_no_repeat_counter = 0u32;
    let mut release_counter = 0u32;
    let mut ctrl_a_counter = 0u32;
    let mut uncaptured_counter = 0u32;
    let mut home_counter = 0u32;
    let mut f1_release_count = 0u32;
    let mut text_buffer = ImString::new("");

    system.main_loop(move |_, ui| {
        Window::new(im_str!("Means of accessing key state"))
            .size([500.0, 300.0], Condition::FirstUseEver)
            .build(ui, || {
                // You can check if a key is currently held down
                if ui.is_key_down(Key::A) {
                    ui.text("The A key is down!");
                } else {
                    ui.text("The A key is not down");
                }

                // You can also check if the key has been pressed
                // down, which is true for one frame. This has "key
                // repeat" so will be true again based on the repeat
                // delay and rate.
                if ui.is_key_pressed(Key::A) {
                    press_counter += 1;
                }
                ui.text(format!(
                    "The A key has been pressed {} times",
                    press_counter
                ));

                // You can also check if the key has been pressed
                // down, which is true for one frame. This has "key
                // repeat" so will be true again based on the repeat
                // delay and rate.
                if ui.is_key_pressed_no_repeat(Key::A) {
                    press_no_repeat_counter += 1;
                }
                ui.text(format!(
                    "The A key has been pressed {} times (ignoring key repeat)",
                    press_no_repeat_counter
                ));

                // Note due to the key-repeat behaviour that the key
                // may be pressed more often than it is released.
                if ui.is_key_released(Key::A) {
                    release_counter += 1;
                }
                ui.text(format!(
                    "The A key has been released {} times",
                    release_counter
                ));

                // Modifiers are accessed via bools on the `imgui::Io`
                // struct,
                if ui.io().key_ctrl {
                    ui.text("Ctrl is down!");
                } else {
                    ui.text("Ctrl is up!");
                }

                // Using modifiers in conjunction with key press
                // events is simple:
                if ui.io().key_ctrl && ui.is_key_released(Key::A) {
                    ctrl_a_counter += 1;
                }
                ui.text(format!(
                    "The Ctrl+A key has been released {} times",
                    ctrl_a_counter
                ));

                // Note that `is_key_released` gives the state of the
                // key regardless of what widget has focus, for
                // example, if you try to type into this input, the
                // above interaction still counts the key presses.
                ui.input_text(im_str!("##Dummy text input widget"), &mut text_buffer)
                    .resize_buffer(true) // Auto-resize ImString as required
                    .hint(im_str!("Example text input"))
                    .build();

                // If you want to check if a widget is capturing
                // keyboard input, you can check
                // `Io::want_capture_keyboard`
                if !ui.io().want_capture_keyboard && ui.is_key_pressed(Key::A) {
                    uncaptured_counter += 1;
                }
                ui.text(format!(
                    "There has been {} uncaptured A key presses",
                    uncaptured_counter
                ));

                // These examples all use `Key::A`. The `imgui::Key`
                // enum only contains the few keys which imgui
                // internally uses (such as for the `Ctrl + A` select
                // all shortcut).  This may expand in future versions
                // of imgui, but will likely never contain every
                // possible key
                //
                // Instead we can use keys using an index, the meaning
                // of which is defined by the implementation of "IO"
                // backend.
                //
                // For example, in the `WinitPlatform` backend each
                // key is indexed by it's integer value of
                // `winit::VirtualKeyCode`. So we can query if a key
                // is down based on it's virtual key code,

                let home_key_idx = 65; // Hardcoded for imgui-examples only, instead use `winit::event::VirtualKeyCode::Home`
                if ui.io().keys_down[home_key_idx as usize] {
                    home_counter += 1;
                }
                ui.text(format!("Home has been pressed for {} frames", home_counter));
                // It is important to remember that unlike using
                // `imgui::Key`, there is nothing enforcing the index
                // is the key you expect. For example if you hardcode
                // your key ID's and switch backends, you may be
                // querying different keys!
                if ui.io().keys_down[123] {
                    // A mystery key is down!
                }

                // It is also possible to use the `is_key_...` methods
                // with arbitrary key indexes. For example, to check
                // if the F1 key is been pressed

                if ui.is_key_index_released(37) {
                    // Index is hardcoded for imgui-examples only, instead do this:
                    //if ui.is_key_index_released(winit::event::VirtualKeyCode::F1 as i32) {
                    f1_release_count += 1;
                }
                ui.text(format!("F1 has been released {} times", f1_release_count));
            });
    });
}
