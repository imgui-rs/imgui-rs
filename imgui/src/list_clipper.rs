use std::marker::PhantomData;

use crate::sys;
use crate::Ui;

/// Used to render only the visible items when displaying a
/// long list of items in a scrollable area.
///
/// For example, you can have a huge list of checkboxes.
/// Without the clipper you have to call `ui.checkbox(...)`
/// for every one, even if 99% of of them are not visible in
/// the current frame. Using the `ListClipper`, you can only
/// call `ui.checkbox(...)` for the currently visible items.
///
/// Note the efficiency of list clipper relies on the height
/// of each item being cheaply calculated. The current rust
/// bindings only works with a fixed height for all items.
pub struct ListClipper {
    items_count: i32,
    items_height: f32,
}

impl ListClipper {
    /// Begins configuring a list clipper.
    pub const fn new(items_count: i32) -> Self {
        ListClipper {
            items_count,
            items_height: -1.0,
        }
    }

    /// Manually set item height. If not set, the height of the first item is used for all subsequent rows.
    pub const fn items_height(mut self, items_height: f32) -> Self {
        self.items_height = items_height;
        self
    }

    pub fn begin(self, ui: &Ui) -> ListClipperToken<'_> {
        let list_clipper = unsafe {
            let list_clipper = sys::ImGuiListClipper_ImGuiListClipper();
            sys::ImGuiListClipper_Begin(list_clipper, self.items_count, self.items_height);
            list_clipper
        };
        ListClipperToken::new(ui, list_clipper)
    }
}

/// List clipper is a mechanism to efficiently implement scrolling of
/// large lists with random access.
///
/// For example you have a list of 1 million buttons, and the list
/// clipper will help you only draw the ones which are visible.
pub struct ListClipperToken<'ui> {
    list_clipper: *mut sys::ImGuiListClipper,
    _phantom: PhantomData<&'ui Ui>,

    /// In upstream imgui < 1.87, calling step too many times will
    /// cause a segfault due to null pointer. So we keep track of this
    /// and panic instead.
    ///
    /// Fixed in https://github.com/ocornut/imgui/commit/dca527b which
    /// will likely be part of imgui 1.88 - at which point this can be
    /// removed.
    consumed_workaround: bool,
}

impl<'ui> ListClipperToken<'ui> {
    fn new(_: &Ui, list_clipper: *mut sys::ImGuiListClipper) -> Self {
        Self {
            list_clipper,
            _phantom: PhantomData,
            consumed_workaround: false,
        }
    }

    /// Progress the list clipper.
    ///
    /// If this returns returns `true` then the you can loop between
    /// between `clipper.display_start() .. clipper.display_end()`.
    /// If this returns false, you must stop calling this method.
    ///
    /// Calling step again after it returns `false` will cause imgui
    /// to abort. This mirrors the C++ interface.
    ///
    /// It is recommended to use the iterator interface!
    pub fn step(&mut self) -> bool {
        let is_imgui_1_88_or_higher = false;
        if is_imgui_1_88_or_higher {
            unsafe { sys::ImGuiListClipper_Step(self.list_clipper) }
        } else {
            if self.consumed_workaround {
                panic!("ListClipperToken::step called after it has previously returned false");
            }
            let ret = unsafe { sys::ImGuiListClipper_Step(self.list_clipper) };
            if !ret {
                self.consumed_workaround = true;
            }
            ret
        }
    }

    /// This is automatically called back the final call to
    /// `step`. You can call it sooner but typically not needed.
    pub fn end(&mut self) {
        unsafe {
            sys::ImGuiListClipper_End(self.list_clipper);
        }
    }

    /// First item to call, updated each call to `step`
    pub fn display_start(&self) -> i32 {
        unsafe { (*self.list_clipper).DisplayStart }
    }

    /// End of items to call (exclusive), updated each call to `step`
    pub fn display_end(&self) -> i32 {
        unsafe { (*self.list_clipper).DisplayEnd }
    }

    /// Get an iterator which outputs all visible indexes. This is the
    /// recommended way of using the clipper.
    pub fn iter(self) -> ListClipperIterator<'ui> {
        ListClipperIterator::new(self)
    }
}

impl<'ui> Drop for ListClipperToken<'ui> {
    fn drop(&mut self) {
        unsafe {
            sys::ImGuiListClipper_destroy(self.list_clipper);
        };
    }
}

pub struct ListClipperIterator<'ui> {
    list_clipper: ListClipperToken<'ui>,
    exhausted: bool,
    last_value: Option<i32>,
}

impl<'ui> ListClipperIterator<'ui> {
    fn new(list_clipper: ListClipperToken<'ui>) -> Self {
        Self {
            list_clipper,
            exhausted: false,
            last_value: None,
        }
    }
}

impl Iterator for ListClipperIterator<'_> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(lv) = self.last_value {
            // Currently iterating a chunk (returning all values
            // between display_start and display_end)
            let next_value = lv + 1;

            if lv >= self.list_clipper.display_end() - 1 {
                // If we reach the end of the current chunk, clear
                // last_value so we call step below
                self.last_value = None;
            } else {
                // Otherwise just increment it
                self.last_value = Some(next_value);
            }
        }

        if let Some(lv) = self.last_value {
            // Next item within current step's chunk
            Some(lv)
        } else {
            // Start iterating a new chunk

            if self.exhausted {
                // If the clipper is exhausted, don't call step again!
                None
            } else {
                // Advance the clipper
                let ret = self.list_clipper.step();
                if !ret {
                    self.exhausted = true;
                    None
                } else {
                    // Setup iteration for this step's chunk
                    let start = self.list_clipper.display_start();
                    let end = self.list_clipper.display_end();

                    if start == end {
                        // Somewhat special case: if a single item, we
                        // don't store the last_value so we call
                        // step() again next iteration
                        self.last_value = None;
                    } else {
                        self.last_value = Some(start);
                    }
                    Some(start)
                }
            }
        }
    }
}

#[test]
fn cpp_style_usage() {
    // Setup
    let (_guard, mut ctx) = crate::test::test_ctx_initialized();
    let ui = ctx.frame();

    let _window = ui
        .window("Example")
        .position([0.0, 0.0], crate::Condition::Always)
        .size([100.0, 800.0], crate::Condition::Always)
        .begin();

    // Create clipper
    let clip = ListClipper::new(1000);
    let mut tok = clip.begin(ui);

    let mut ticks = 0;

    while dbg!(tok.step()) {
        for row_num in dbg!(tok.display_start())..dbg!(tok.display_end()) {
            dbg!(row_num);
            ui.text("...");
            ticks += 1;
        }
    }

    // Check it's called an expected amount of time (only the ones
    // visible in given sized window)
    assert_eq!(ticks, 44);

    // Calling end multiple times is fine albeit redundant
    tok.end();
    tok.end();
    tok.end();
    tok.end();
    tok.end();
    tok.end();
}

#[test]
fn iterator_usage() {
    // Setup
    let (_guard, mut ctx) = crate::test::test_ctx_initialized();
    let ui = ctx.frame();

    let _window = ui
        .window("Example")
        .position([0.0, 0.0], crate::Condition::Always)
        .size([100.0, 800.0], crate::Condition::Always)
        .begin();

    // Create clipper
    let clip = ListClipper::new(1000);

    let mut ticks = 0;

    let tok = clip.begin(ui);
    for row_num in tok.iter() {
        dbg!(row_num);
        ui.text("...");
        ticks += 1;
    }

    // Should be consistent with size in `cpp_style_usage`
    assert_eq!(ticks, 44);
}
