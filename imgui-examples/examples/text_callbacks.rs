use imgui::*;

mod support;

fn main() {
    let system = support::init(file!());
    let mut buffers = vec![String::default(), String::default(), String::default()];

    system.main_loop(move |_, ui| {
        ui.window("Input text callbacks")
            .size([500.0, 300.0], Condition::FirstUseEver)
            .build(|| {
                ui.text("You can make a variety of buffer callbacks on an Input Text");
                ui.text(
                    "or on an InputTextMultiline. In this example, we'll use \
                InputText primarily.",
                );
                ui.text(
                    "The only difference is that InputTextMultiline doesn't get \
                the `History` callback,",
                );
                ui.text("since, of course, you need the up/down keys to navigate.");

                ui.separator();

                ui.text("No callbacks:");

                ui.input_text("buf0", &mut buffers[0]).build();
                ui.input_text("buf1", &mut buffers[1]).build();
                ui.input_text("buf2", &mut buffers[2]).build();

                ui.separator();

                ui.text("Here's a callback which printlns when each is ran.");

                struct AllCallback;
                impl InputTextCallbackHandler for AllCallback {
                    fn char_filter(&mut self, c: char) -> Option<char> {
                        println!("Char filter fired! This means a char was inputted.");
                        Some(c)
                    }
                    fn on_completion(&mut self, _: TextCallbackData) {
                        println!("Completion request fired! This means the tab key was hit.");
                    }

                    fn on_edit(&mut self, _: TextCallbackData) {
                        println!("Edit was fired! Any edit will cause this to fire.")
                    }

                    fn on_history(&mut self, dir: HistoryDirection, _: TextCallbackData) {
                        println!("History was fired by pressing {:?}", dir);
                    }

                    fn on_always(&mut self, _: TextCallbackData) {
                        // We don't actually print this out because it will flood your log a lot!
                        // println!("The always callback fired! It always fires.");
                    }
                }

                ui.input_text("All Callbacks logging", buffers.get_mut(0).unwrap())
                    .callback(InputTextCallback::all(), AllCallback)
                    .build();

                ui.separator();

                ui.text("You can also define a callback on structs with data.");
                ui.text("Here we implement the callback handler on a wrapper around &mut String");
                ui.text("to duplicate edits to buf0 on buf1");

                struct Wrapper<'a>(&'a mut String);
                impl<'a> InputTextCallbackHandler for Wrapper<'a> {
                    fn on_always(&mut self, data: TextCallbackData) {
                        *self.0 = data.str().to_owned();
                    }
                }

                let (buf0, brwchk_dance) = buffers.split_first_mut().unwrap();
                let buf1 = Wrapper(&mut brwchk_dance[0]);

                ui.input_text("Edits copied to buf1", buf0)
                    .callback(InputTextCallback::ALWAYS, buf1)
                    .build();

                ui.separator();

                ui.text("Finally, we'll do some whacky history to show inserting and removing");
                ui.text("characters from the buffer.");
                ui.text(
                    "Here, pressing UP (while editing the below widget) will remove the\n\
                first and last character from buf2",
                );
                ui.text("and pressing DOWN will prepend the first char from buf0 AND");
                ui.text("append the last char from buf1");

                let (buf0, brwchk_dance) = buffers.split_first_mut().unwrap();
                let (buf1, buf2_dance) = brwchk_dance.split_first_mut().unwrap();
                let buf2 = &mut buf2_dance[0];

                struct Wrapper2<'a>(&'a str, &'a str);

                impl<'a> InputTextCallbackHandler for Wrapper2<'a> {
                    fn on_history(&mut self, dir: HistoryDirection, mut data: TextCallbackData) {
                        match dir {
                            HistoryDirection::Up => {
                                // remove first char...
                                if !data.str().is_empty() {
                                    data.remove_chars(0, 1);

                                    if let Some((idx, _)) = data.str().char_indices().next_back() {
                                        data.remove_chars(idx, 1);
                                    }
                                }
                            }
                            HistoryDirection::Down => {
                                // insert first char...
                                if let Some(first_char) = self.0.get(0..1) {
                                    data.insert_chars(0, first_char);
                                }

                                // insert last char
                                if let Some((idx, _)) = self.1.char_indices().next_back() {
                                    data.push_str(&self.1[idx..]);
                                }
                            }
                        }
                    }
                }

                ui.input_text("Wild buf2 editor", buf2)
                    .callback(InputTextCallback::HISTORY, Wrapper2(buf0, buf1))
                    .build();

                ui.text(
                    "For more examples on how to use callbacks non-chaotically, check the demo",
                );
            });
    });
}
