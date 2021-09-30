mod support;

fn main() {
    let system = support::init(file!());
    system.main_loop(move |_, ui| {
        let items = vec!["a", "b", "c", "d"];

        ui.window("Broken Example")
            .position([0.0, 0.0], imgui::Condition::FirstUseEver)
            .size([390.0, 200.0], imgui::Condition::FirstUseEver)
            .build(|| {
                ui.text("Broken! Only first button responds to clicks");

                // Because all our buttons have the same label (and thus ID),
                // only the first button responds to clicks!
                for it in &items {
                    ui.text(it);
                    for num in 0..5 {
                        ui.same_line();
                        if ui.button("Example") {
                            println!("{}: {}", it, num);
                        }
                    }
                }
            });

        ui.window("Good Example")
            .position([400.0, 0.0], imgui::Condition::FirstUseEver)
            .size([390.0, 200.0], imgui::Condition::FirstUseEver)
            .build(|| {
                ui.text("Works!");
                for it in &items {
                    let _label_id = ui.push_id(it);
                    ui.text(it);
                    for num in 0..5 {
                        let _num_id = ui.push_id(num);
                        ui.same_line();
                        if ui.button("Example") {
                            println!("{}: {}", it, num);
                        }
                    }
                }
            });
    });
}
