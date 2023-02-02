use imgui::*;

mod support;

fn main() {
    let system = support::init(file!());

    let mut humans = vec![
        HumanData {
            name: "Joonas",
            favorite_number: 102,
            favorite_fruit_maybe: "Dutch",
        },
        HumanData {
            name: "Thom",
            favorite_number: 314,
            favorite_fruit_maybe: "Rice",
        },
        HumanData {
            name: "Jack",
            favorite_number: 611,
            favorite_fruit_maybe: "Mangoes",
        },
    ];

    let mut t2_flags = TableFlags::REORDERABLE
        | TableFlags::HIDEABLE
        | TableFlags::RESIZABLE
        | TableFlags::NO_BORDERS_IN_BODY;

    system.main_loop(move |_, ui| {
        ui.window("Input text callbacks")
            .size([800.0, 400.0], Condition::FirstUseEver)
            .build(|| {
                if let Some(_t) = ui.begin_table("Basic-Table", 3) {
                    // we must also call `next_row` here, because we declined
                    // to set up header rows. If we set up header rows ourselves,
                    // we will call `table_header_rows` instead, and if we use
                    // `begin_table_header`, then the initial call will be handled for us.

                    ui.table_next_row();

                    // note you MUST call `next_column` at least to START
                    // Let's walk through a table like it's an iterator...
                    ui.table_set_column_index(0);
                    ui.text("x: 0, y: 0");

                    ui.table_next_column();
                    ui.text("x: 1, y: 0");

                    ui.table_next_column();
                    ui.text("x: 2, y: 0");

                    // // calling next column again will wrap us around to 0-1,
                    // // since we've exhausted our 3 columns.
                    ui.table_next_column();
                    ui.text("x: 0, y: 1");

                    // // Let's do this manually now -- we can set each column ourselves...
                    ui.table_set_column_index(1);
                    ui.text("x: 1, y: 1");

                    ui.table_set_column_index(2);
                    ui.text("x: 2, y: 1");

                    // you CAN go back...
                    ui.table_set_column_index(1);
                    // however, you should call `new_line`, since otherwise
                    // we'd right on top of our `x: 1, y: 1` text.
                    ui.new_line();
                    ui.text("our of order txt");
                }

                ui.separator();
                ui.text("Let's add some headers");
                if let Some(_t) = ui.begin_table_header(
                    "table-headers",
                    [
                        TableColumnSetup::new("Name"),
                        TableColumnSetup::new("Age"),
                        TableColumnSetup::new("Favorite fruit"),
                    ],
                ) {
                    // note that we DON'T have to call "table_next_row" here -- that's taken care
                    // of for us by `begin_table_header`, since it actually calls `table_headers_row`

                    // but we DO need to call column!
                    // but that's fine, we'll use a loop
                    for i in 0..3 {
                        let names = ["Joonas", "Thom", "Jack"];
                        let fruit = ["Dutch", "Rice", "Mangoes"];

                        ui.table_next_column();
                        ui.text(names[i]);

                        ui.table_next_column();
                        ui.text((i * 9).to_string());

                        ui.table_next_column();
                        ui.text(fruit[i]);
                    }
                }

                ui.separator();
                ui.text("Let's do some context menus");
                ui.text(
                    "context menus are created, by default, from the flags passed\
                while making the table, or each row.\n\
                Notice how toggling these checkboxes changes the context menu.",
                );

                ui.checkbox_flags("Reorderable", &mut t2_flags, TableFlags::REORDERABLE);
                ui.same_line();
                ui.checkbox_flags("Hideable", &mut t2_flags, TableFlags::HIDEABLE);
                ui.same_line();
                ui.checkbox_flags("Resizable", &mut t2_flags, TableFlags::RESIZABLE);

                if let Some(_t) = ui.begin_table_header_with_flags(
                    "table-headers2",
                    [
                        TableColumnSetup::new("Name"),
                        TableColumnSetup::new("Age"),
                        TableColumnSetup::new("Favorite fruit"),
                    ],
                    t2_flags,
                ) {
                    // note that we DON'T have to call "table_next_row" here -- that's taken care
                    // of for us by `begin_table_header`, since it actually calls `table_headers_row`

                    // but we DO need to call column!
                    // but that's fine, we'll use a loop
                    for i in 0..3 {
                        let names = ["Joonas", "Thom", "Jack"];
                        let fruit = ["Dutch", "Rice", "Mangoes"];

                        ui.table_next_column();
                        ui.text(names[i]);

                        ui.table_next_column();
                        ui.text((i * 9).to_string());

                        ui.table_next_column();
                        ui.text(fruit[i]);
                    }
                }

                ui.separator();

                ui.text("Here's a table you can sort!");
                ui.text("Check the code to see the two methods of doing it.");

                if let Some(_t) = ui.begin_table_header_with_flags(
                    "table-headers3",
                    [
                        TableColumnSetup::new("Name"),
                        TableColumnSetup::new("Favorite Number"),
                        TableColumnSetup::new("Favorite fruit"),
                    ],
                    TableFlags::SORTABLE,
                ) {
                    if let Some(sort_data) = ui.table_sort_specs_mut() {
                        sort_data
                            .conditional_sort(|specs| HumanData::sort_humans(&mut humans, specs));

                        // Can also sort this other way...
                        // if sort_data.should_sort() {
                        //     HumanData::sort_humans(&mut humans, sort_data.specs());
                        //     sort_data.set_sorted();
                        // }
                    }

                    for human in humans.iter() {
                        ui.table_next_column();
                        ui.text(human.name);

                        ui.table_next_column();
                        ui.text(human.favorite_number.to_string());

                        ui.table_next_column();
                        ui.text(human.favorite_fruit_maybe);
                    }
                }
            });
    });
}
struct HumanData {
    name: &'static str,
    favorite_number: usize,
    favorite_fruit_maybe: &'static str,
}

impl HumanData {
    pub fn sort_humans(humans: &mut [Self], specs: Specs<'_>) {
        let spec = specs.iter().next().unwrap();
        if let Some(kind) = spec.sort_direction() {
            match kind {
                TableSortDirection::Ascending => match spec.column_idx() {
                    0 => humans.sort_by_key(|h| h.name),
                    1 => humans.sort_by_key(|h| h.favorite_number),
                    2 => humans.sort_by_key(|h| h.favorite_fruit_maybe),
                    _ => unimplemented!(),
                },
                TableSortDirection::Descending => match spec.column_idx() {
                    0 => humans.sort_by_key(|h| std::cmp::Reverse(h.name)),
                    1 => humans.sort_by_key(|h| std::cmp::Reverse(h.favorite_number)),
                    2 => humans.sort_by_key(|h| std::cmp::Reverse(h.favorite_fruit_maybe)),
                    _ => unimplemented!(),
                },
            }
        }
    }
}
