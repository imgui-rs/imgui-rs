mod support;

fn main() {
    support::simple_init(file!(), move |_, ui| {
        // If we don't explicitly create a window before creating some kind of widget, then Dear Imgui will automatically create one
        ui.text("This text will appear in a default window titled 'Debug'");

        // However, in almost all cases it's best to make a window, so it has a useful title etc

        // imgui-rs has two main methods of creating windows (and these same approaches
        // apply to many other widgets). First, callback based:

        ui.window("My window via callback").build(|| {
            ui.text("This content appears in a window");

            // Everything in this callback appears in the window, like this button:
            ui.button("This button");
        });

        // Often the callback approach is most convenient, however occasionally the callbacks can be hard to use.
        // In this case, there is the "token based" approach. You call a method and get a "window token",
        // everything that happens until the token is dropped is included in the window this is more-or-less how
        // the Dear ImGui C++ API works)

        // Here we (maybe) get a window token:
        let window_token = ui.window("Token based window").begin();
        if let Some(_t) = window_token {
            // If the token is Some(...) then the window contents are visible, so we need to draw them!
            ui.text("Window contents!")
        }

        // Here we create a window with a specific size, and force it to always have a vertical scrollbar visible
        ui.window("Big complex window")
            .size([200.0, 100.0], imgui::Condition::FirstUseEver)
            .always_vertical_scrollbar(true)
            .build(|| {
                ui.text("Imagine something complicated here..");

                // Note you can create windows inside other windows, however, they both appear as separate windows.
                // For example, somewhere deep inside a complex window, we can quickly create a widget to display a
                // variable, like a graphical "debug print"
                ui.window("Confusion")
                    .build(|| ui.text(format!("Some variable: {:?}", ui.io().mouse_pos)))
            });

        // If you want to nest windows inside other windows, you can a "child window".
        // This is essentially a scrollable area, with all the same properties as a regular window
        ui.window("Parent window").build(|| {
            ui.child_window("Child window")
                .size([100.0, 100.0])
                .build(|| {
                    for _ in 0..10 {
                        ui.text("Lines and");
                    }
                });
            ui.child_window("Second child window")
                .size([100.0, 100.0])
                .build(|| {
                    for _ in 0..10 {
                        ui.text("More and");
                    }
                });
        });
    });
}
