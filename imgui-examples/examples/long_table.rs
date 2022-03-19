use imgui::*;

mod support;

fn main() {
    let system = support::init(file!());

    system.main_loop(move |_, ui| {
        ui.show_demo_window(&mut true);

        ui.window("Table with list clipper")
            .size([800.0, 700.0], Condition::FirstUseEver)
            .build(|| {
                let num_cols = 3;
                let num_rows = 1000;

                let flags = imgui::TableFlags::ROW_BG
                    | imgui::TableFlags::RESIZABLE
                    | imgui::TableFlags::BORDERS_H
                    | imgui::TableFlags::BORDERS_V; //| imgui::TableFlags::SCROLL_Y;

                if let Some(_t) = ui.begin_table_with_sizing(
                    "longtable",
                    num_cols,
                    flags,
                    [300.0, 100.0],
                    /*inner width=*/ 0.0,
                ) {
                    ui.table_setup_column("A");
                    ui.table_setup_column("B");
                    ui.table_setup_column("C");

                    // Freeze first row so headers are visible even
                    // when scrolling
                    ui.table_setup_scroll_freeze(num_cols, 1);

                    // Done with headers row
                    ui.table_headers_row();

                    // Create clipper with st
                    let clip = imgui::ListClipper::new(num_rows).begin(ui);
                    for row_num in clip.iter() {
                        ui.table_next_row();
                        for col_num in 0..num_cols {
                            ui.table_set_column_index(col_num);
                            ui.text(format!("Hello {},{}", col_num, row_num));
                        }
                    }
                }
            });
    });
}
