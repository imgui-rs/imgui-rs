use imgui::*;

mod support;

fn main() {
    let mut state = State::default();
    let system = support::init(file!());
    system.main_loop(move |run, ui| {
        example_selector(run, ui, &mut state);
        match state.example {
            1 => example_1(ui, &mut state),
            2 => example_2(ui, &mut state),
            _ => (),
        }
    });
}

fn example_selector(run: &mut bool, ui: &mut Ui, state: &mut State) {
    let w = ui
        .window("Slider examples")
        .opened(run)
        .position([20.0, 20.0], Condition::Appearing)
        .size([700.0, 80.0], Condition::Appearing)
        .resizable(false);
    w.build(|| {
        let mut clicked = false;
        clicked |= ui.radio_button("Example 1: Basic sliders", &mut state.example, 1);
        clicked |= ui.radio_button("Example 2: Slider arrays", &mut state.example, 2);
        if clicked {
            state.reset();
        }
    });
}

fn example_1(ui: &Ui, state: &mut State) {
    let w = ui
        .window("Example 1: Basic sliders")
        .size([700.0, 340.0], Condition::Appearing)
        .position([20.0, 120.0], Condition::Appearing);
    w.build(|| {
        ui.text("All of the following data types are supported:");
        ui.text("Signed:   i8 i16 i32 i64");
        ui.text("Unsigned: u8 u16 u32 u64");
        ui.text("Floats:   f32 f64");

        // Full ranges can be specified with Rust's `::MIN/MAX` constants
        Slider::new("u8 value", u8::MIN, u8::MAX)
            .build(ui, &mut state.u8_value);

        // However for larger data-types, it's usually best to specify
        // a much smaller range. The following slider is hard to use.
        Slider::new("Full range f32 value", -f32::MIN/2.0, f32::MAX/2.0)
            .build(ui, &mut state.f32_value);
        // Note the `... / 2.0` - anything larger is not supported by
        // the upstream C++ library
        ui.text("Note that for 32-bit/64-bit types, sliders are always limited to half of the natural type range!");

        // Most of the time, it's best to specify the range
        ui.separator();
        ui.text("Slider range can be limited:");
        Slider::new("i32 value with range", -999, 999)
            .build(ui, &mut state.i32_value);
        Slider::new("f32 value", -10.0, 10.0)
            .build(ui, &mut state.f32_value);

        ui.separator();
        ui.text("Value formatting can be customized with a C-style printf string:");
        Slider::new("f64 value with custom formatting", -999_999_999.0, 999_999_999.0)
            .display_format("%09.0f")
            .build(ui, &mut state.f64_formatted);

        // This formatting impacts the increments the slider operates in:
        Slider::new("f32 with %.01f", 0.0, 1.0)
            .display_format("%.01f")
            .build(ui, &mut state.f32_value);
        Slider::new("Same f32 with %.05f", 0.0, 1.0)
            .display_format("%.05f")
            .build(ui, &mut state.f32_value);

        ui.separator();
        ui.text("Vertical sliders require a size parameter but otherwise work in a similar way:");
        VerticalSlider::new("vertical\nu8 value", [50.0, 50.0], u8::MIN, u8::MAX)
            .build(ui, &mut state.u8_value);
    });
}

fn example_2(ui: &Ui, state: &mut State) {
    let w = ui
        .window("Example 2: Slider arrays")
        .size([700.0, 260.0], Condition::Appearing)
        .position([20.0, 120.0], Condition::Appearing);
    w.build(|| {
        ui.text("You can easily build a slider group from an array of values:");
        Slider::new("[u8; 4]", 0, u8::MAX).build_array(ui, &mut state.array);

        ui.text("You don't need to use arrays with known length; arbitrary slices can be used:");
        let slice: &mut [u8] = &mut state.array[1..=2];
        Slider::new("subslice", 0, u8::MAX).build_array(ui, slice);
    });
}

#[derive(Default)]
struct State {
    example: u32,
    i32_value: i32,
    u8_value: u8,
    f32_value: f32,
    f64_formatted: f64,
    array: [u8; 4],
}

impl State {
    fn reset(&mut self) {}
}
