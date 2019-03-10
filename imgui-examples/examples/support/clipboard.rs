use clipboard::{ClipboardContext, ClipboardProvider};
use imgui::{ClipboardBackend, ImStr, ImString};

pub struct ClipboardSupport(ClipboardContext);

pub fn init() -> Option<ClipboardSupport> {
    ClipboardContext::new()
        .ok()
        .map(|ctx| ClipboardSupport(ctx))
}

impl ClipboardBackend for ClipboardSupport {
    fn get(&mut self) -> Option<ImString> {
        self.0.get_contents().ok().map(|text| text.into())
    }
    fn set(&mut self, text: &ImStr) {
        let _ = self.0.set_contents(text.to_str().to_owned());
    }
}
