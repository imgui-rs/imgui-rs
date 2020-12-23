use imgui::*;
use imgui_sys::*;
use std::ptr::null_mut;

mod support;

fn main() {
    let system = support::init(file!());
    system.main_loop(move |_, ui| {
        Window::new(im_str!("Hello world"))
            .size([300.0, 110.0], Condition::FirstUseEver)
            .build(ui, || {
                ui.text("collapse me to get rid of him!");
                ui.separator();
                let mouse_pos = ui.io().mouse_pos;
                ui.text(format!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos[0], mouse_pos[1]
                ));
            });

        unsafe {
            let mut previous_is_collapsed = true;
            if let Some(previous_window) =
                igFindWindowByName(im_str!("Hello world").as_ptr()).as_ref()
            {
                if !previous_window.Collapsed {
                    previous_is_collapsed = false;
                    let new_window_pos = ImVec2 {
                        x: previous_window.Pos.x + previous_window.Size.x + 50.0,
                        y: previous_window.Pos.y,
                    };
                    igSetNextWindowPos(new_window_pos, ImGuiCond_Always, ImVec2::default());
                    igSetNextWindowSize(previous_window.Size, ImGuiCond_Always);
                }
            };
            igBegin(
                im_str!("Hello from raw cimgui").as_ptr(),
                null_mut(),
                ImGuiWindowFlags_None,
            );
            if previous_is_collapsed {
                igText(im_str!("Where is he gone!?").as_ptr());
            } else {
                igText(im_str!("I am an Imposter!").as_ptr());
            }

            igSeparator();

            let io = igGetIO().as_ref().unwrap();

            igText(im_str!("Mouse Position : {:.1},{:.1}", io.MousePos.x, io.MousePos.y).as_ptr());

            let current_window = igGetCurrentWindow().as_ref().unwrap();

            igText(
                im_str!(
                    "Current Window Position : {},{}",
                    current_window.Pos.x,
                    current_window.Pos.y
                )
                .as_ptr(),
            );
            igText(im_str!("Current Window Id : {}", current_window.ID).as_ptr());
            igEnd();
        }
    });
}
