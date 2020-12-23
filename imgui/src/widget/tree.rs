use bitflags::bitflags;
use std::os::raw::{c_char, c_void};
use std::ptr;
use std::thread;

use crate::context::Context;
use crate::string::ImStr;
use crate::sys;
use crate::{Condition, Ui};

bitflags!(
    /// Flags for tree nodes
    #[repr(transparent)]
    pub struct TreeNodeFlags: u32 {
        /// Draw as selected
        const SELECTED = sys::ImGuiTreeNodeFlags_Selected as u32;
        /// Full colored frame (e.g. for CollapsingHeader)
        const FRAMED = sys::ImGuiTreeNodeFlags_Framed as u32;
        /// Hit testing to allow subsequent widgets to overlap this one
        const ALLOW_ITEM_OVERLAP = sys::ImGuiTreeNodeFlags_AllowItemOverlap as u32;
        /// Don't push a tree node when open (e.g. for CollapsingHeader) = no extra indent nor
        /// pushing on ID stack
        const NO_TREE_PUSH_ON_OPEN = sys::ImGuiTreeNodeFlags_NoTreePushOnOpen as u32;
        /// Don't automatically and temporarily open node when Logging is active (by default
        /// logging will automatically open tree nodes)
        const NO_AUTO_OPEN_ON_LOG = sys::ImGuiTreeNodeFlags_NoAutoOpenOnLog as u32;
        /// Default node to be open
        const DEFAULT_OPEN = sys::ImGuiTreeNodeFlags_DefaultOpen as u32;
        /// Need double-click to open node
        const OPEN_ON_DOUBLE_CLICK = sys::ImGuiTreeNodeFlags_OpenOnDoubleClick as u32;
        /// Only open when clicking on the arrow part.
        ///
        /// If `TreeNodeFlags::OPEN_ON_DOUBLE_CLICK` is also set, single-click arrow or
        /// double-click all box to open.
        const OPEN_ON_ARROW = sys::ImGuiTreeNodeFlags_OpenOnArrow as u32;
        /// No collapsing, no arrow (use as a convenience for leaf nodes)
        const LEAF = sys::ImGuiTreeNodeFlags_Leaf as u32;
        /// Display a bullet instead of arrow
        const BULLET = sys::ImGuiTreeNodeFlags_Bullet as u32;
        /// Use `Style::frame_padding` (even for an unframed text node) to vertically align text
        /// baseline to regular widget height.
        ///
        /// Equivalent to calling `Ui::align_text_to_frame_padding`.
        const FRAME_PADDING = sys::ImGuiTreeNodeFlags_FramePadding as u32;
        /// Extend hit box to the right-most edge, even if not framed.
        ///
        /// This is not the default in order to allow adding other items on the same line. In the
        /// future we may refactor the hit system to be front-to-back, allowing natural overlaps
        /// and then this can become the default.
        const SPAN_AVAIL_WIDTH = sys::ImGuiTreeNodeFlags_SpanAvailWidth as u32;
        /// Extend hit box to the left-most and right-most edges (bypass the indented area)
        const SPAN_FULL_WIDTH = sys::ImGuiTreeNodeFlags_SpanFullWidth as u32;
        /// (WIP) Nav: left direction may move to this tree node from any of its child
        const NAV_LEFT_JUMPS_BACK_HERE = sys::ImGuiTreeNodeFlags_NavLeftJumpsBackHere as u32;
    }
);

static FMT: &[u8] = b"%s\0";

#[inline]
fn fmt_ptr() -> *const c_char {
    FMT.as_ptr() as *const c_char
}

/// Unique ID used by tree nodes
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TreeNodeId<'a> {
    Str(&'a ImStr),
    Ptr(*const c_void),
}

impl<'a, T: ?Sized + AsRef<ImStr>> From<&'a T> for TreeNodeId<'a> {
    fn from(s: &'a T) -> Self {
        TreeNodeId::Str(s.as_ref())
    }
}

impl<T> From<*const T> for TreeNodeId<'static> {
    fn from(p: *const T) -> Self {
        TreeNodeId::Ptr(p as *const c_void)
    }
}

impl<T> From<*mut T> for TreeNodeId<'static> {
    fn from(p: *mut T) -> Self {
        TreeNodeId::Ptr(p as *const T as *const c_void)
    }
}

/// Builder for a tree node widget
#[derive(Copy, Clone, Debug)]
#[must_use]
pub struct TreeNode<'a> {
    id: TreeNodeId<'a>,
    label: Option<&'a ImStr>,
    opened: bool,
    opened_cond: Condition,
    flags: TreeNodeFlags,
}

impl<'a> TreeNode<'a> {
    /// Constructs a new tree node builder
    pub fn new<I: Into<TreeNodeId<'a>>>(id: I) -> TreeNode<'a> {
        TreeNode {
            id: id.into(),
            label: None,
            opened: false,
            opened_cond: Condition::Never,
            flags: TreeNodeFlags::empty(),
        }
    }
    /// Sets the tree node label
    #[inline]
    pub fn label(mut self, label: &'a ImStr) -> Self {
        self.label = Some(label);
        self
    }
    /// Sets the opened state of the tree node, which is applied based on the given condition value
    #[inline]
    pub fn opened(mut self, opened: bool, cond: Condition) -> Self {
        self.opened = opened;
        self.opened_cond = cond;
        self
    }
    /// Replaces all current settings with the given flags.
    #[inline]
    pub fn flags(mut self, flags: TreeNodeFlags) -> Self {
        self.flags = flags;
        self
    }
    /// Enables/disables drawing the tree node in selected state.
    ///
    /// Disabled by default.
    #[inline]
    pub fn selected(mut self, value: bool) -> Self {
        self.flags.set(TreeNodeFlags::SELECTED, value);
        self
    }
    /// Enables/disables full-colored frame.
    ///
    /// Disabled by default.
    #[inline]
    pub fn framed(mut self, value: bool) -> Self {
        self.flags.set(TreeNodeFlags::FRAMED, value);
        self
    }
    /// Enables/disables allowing the tree node to overlap subsequent widgets.
    ///
    /// Disabled by default.
    #[inline]
    pub fn allow_item_overlap(mut self, value: bool) -> Self {
        self.flags.set(TreeNodeFlags::ALLOW_ITEM_OVERLAP, value);
        self
    }
    /// Enables/disables automatic tree push when the tree node is open (= adds extra indentation
    /// and pushes to the ID stack).
    ///
    /// Enabled by default.
    #[inline]
    pub fn tree_push_on_open(mut self, value: bool) -> Self {
        self.flags.set(TreeNodeFlags::NO_TREE_PUSH_ON_OPEN, !value);
        self
    }
    /// Enables/disables automatic opening of the tree node when logging is active.
    ///
    /// By default, logging will automatically open all tree nodes.
    ///
    /// Enabled by default.
    #[inline]
    pub fn auto_open_on_log(mut self, value: bool) -> Self {
        self.flags.set(TreeNodeFlags::NO_AUTO_OPEN_ON_LOG, !value);
        self
    }
    /// Sets the default open state for the tree node.
    ///
    /// Tree nodes are closed by default.
    #[inline]
    pub fn default_open(mut self, value: bool) -> Self {
        self.flags.set(TreeNodeFlags::DEFAULT_OPEN, value);
        self
    }
    /// Only open when the tree node is double-clicked.
    ///
    /// Disabled by default.
    #[inline]
    pub fn open_on_double_click(mut self, value: bool) -> Self {
        self.flags.set(TreeNodeFlags::OPEN_ON_DOUBLE_CLICK, value);
        self
    }
    /// Only open when clicking the arrow part of the tree node.
    ///
    /// Disabled by default.
    #[inline]
    pub fn open_on_arrow(mut self, value: bool) -> Self {
        self.flags.set(TreeNodeFlags::OPEN_ON_ARROW, value);
        self
    }
    /// Enable/disables leaf mode (no collapsing, no arrow).
    ///
    /// Disabled by default.
    #[inline]
    pub fn leaf(mut self, value: bool) -> Self {
        self.flags.set(TreeNodeFlags::LEAF, value);
        self
    }
    /// Display a bullet instead of arrow.
    ///
    /// Disabled by default.
    #[inline]
    pub fn bullet(mut self, value: bool) -> Self {
        self.flags.set(TreeNodeFlags::BULLET, value);
        self
    }
    /// Use `frame_padding` to vertically align text baseline to regular widget height.
    ///
    /// Disabled by default.
    #[inline]
    pub fn frame_padding(mut self, value: bool) -> Self {
        self.flags.set(TreeNodeFlags::FRAME_PADDING, value);
        self
    }
    /// Left direction may move to this tree node from any of its child.
    ///
    /// Disabled by default.
    #[inline]
    pub fn nav_left_jumps_back_here(mut self, value: bool) -> Self {
        self.flags
            .set(TreeNodeFlags::NAV_LEFT_JUMPS_BACK_HERE, value);
        self
    }
    /// Pushes a tree node and starts appending to it.
    ///
    /// Returns `Some(TreeNodeToken)` if the tree node is open. After content has been
    /// rendered, the token must be popped by calling `.pop()`.
    ///
    /// Returns `None` if the tree node is not open and no content should be rendered.
    #[must_use]
    pub fn push(self, ui: &Ui) -> Option<TreeNodeToken> {
        let open = unsafe {
            if self.opened_cond != Condition::Never {
                sys::igSetNextItemOpen(self.opened, self.opened_cond as i32);
            }
            match self.id {
                TreeNodeId::Str(id) => sys::igTreeNodeExStrStr(
                    id.as_ptr(),
                    self.flags.bits() as i32,
                    fmt_ptr(),
                    self.label.unwrap_or(id).as_ptr(),
                ),
                TreeNodeId::Ptr(id) => sys::igTreeNodeExPtr(
                    id,
                    self.flags.bits() as i32,
                    fmt_ptr(),
                    self.label.unwrap_or_default().as_ptr(),
                ),
            }
        };
        if open {
            Some(TreeNodeToken {
                ctx: if self.flags.contains(TreeNodeFlags::NO_TREE_PUSH_ON_OPEN) {
                    ptr::null()
                } else {
                    ui.ctx
                },
            })
        } else {
            None
        }
    }
    /// Creates a tree node and runs a closure to construct the contents.
    ///
    /// Note: the closure is not called if the tree node is not open.
    pub fn build<F: FnOnce()>(self, ui: &Ui, f: F) {
        if let Some(node) = self.push(ui) {
            f();
            node.pop(ui);
        }
    }
}

/// Tracks a tree node that must be popped by calling `.pop()`.
///
/// If `TreeNodeFlags::NO_TREE_PUSH_ON_OPEN` was used when this token was created, calling `.pop()`
/// is not mandatory and is a no-op.
#[must_use]
pub struct TreeNodeToken {
    ctx: *const Context,
}

impl TreeNodeToken {
    /// Pops a tree node
    pub fn pop(mut self, _: &Ui) {
        if !self.ctx.is_null() {
            self.ctx = ptr::null();
            unsafe { sys::igTreePop() };
        }
    }
}

impl Drop for TreeNodeToken {
    fn drop(&mut self) {
        if !self.ctx.is_null() && !thread::panicking() {
            panic!("A TreeNodeToken was leaked. Did you call .pop()?");
        }
    }
}

/// Builder for a collapsing header widget
#[derive(Copy, Clone, Debug)]
#[must_use]
pub struct CollapsingHeader<'a> {
    label: &'a ImStr,
    flags: TreeNodeFlags,
}

impl<'a> CollapsingHeader<'a> {
    /// Constructs a new collapsing header builder
    pub fn new(label: &ImStr) -> CollapsingHeader {
        CollapsingHeader {
            label,
            flags: TreeNodeFlags::empty(),
        }
    }
    /// Replaces all current settings with the given flags.
    #[inline]
    pub fn flags(mut self, flags: TreeNodeFlags) -> Self {
        self.flags = flags;
        self
    }
    /// Enables/disables allowing the collapsing header to overlap subsequent widgets.
    ///
    /// Disabled by default.
    #[inline]
    pub fn allow_item_overlap(mut self, value: bool) -> Self {
        self.flags.set(TreeNodeFlags::ALLOW_ITEM_OVERLAP, value);
        self
    }
    /// Sets the default open state for the collapsing header.
    ///
    /// Collapsing headers are closed by default.
    #[inline]
    pub fn default_open(mut self, value: bool) -> Self {
        self.flags.set(TreeNodeFlags::DEFAULT_OPEN, value);
        self
    }
    /// Only open when the collapsing header is double-clicked.
    ///
    /// Disabled by default.
    #[inline]
    pub fn open_on_double_click(mut self, value: bool) -> Self {
        self.flags.set(TreeNodeFlags::OPEN_ON_DOUBLE_CLICK, value);
        self
    }
    /// Only open when clicking the arrow part of the collapsing header.
    ///
    /// Disabled by default.
    #[inline]
    pub fn open_on_arrow(mut self, value: bool) -> Self {
        self.flags.set(TreeNodeFlags::OPEN_ON_ARROW, value);
        self
    }
    /// Enable/disables leaf mode (no collapsing, no arrow).
    ///
    /// Disabled by default.
    #[inline]
    pub fn leaf(mut self, value: bool) -> Self {
        self.flags.set(TreeNodeFlags::LEAF, value);
        self
    }
    /// Display a bullet instead of arrow.
    ///
    /// Disabled by default.
    #[inline]
    pub fn bullet(mut self, value: bool) -> Self {
        self.flags.set(TreeNodeFlags::BULLET, value);
        self
    }
    /// Use `frame_padding` to vertically align text baseline to regular widget height.
    ///
    /// Disabled by default.
    #[inline]
    pub fn frame_padding(mut self, value: bool) -> Self {
        self.flags.set(TreeNodeFlags::FRAME_PADDING, value);
        self
    }
    /// Builds the collapsing header.
    ///
    /// Returns true if the collapsing header is open and content should be rendered.
    #[must_use]
    pub fn build(self, _: &Ui) -> bool {
        unsafe {
            sys::igCollapsingHeaderTreeNodeFlags(self.label.as_ptr(), self.flags.bits() as i32)
        }
    }
    /// Builds the collapsing header, and adds an additional close button that changes the value of
    /// the given mutable reference when clicked.
    ///
    /// Returns true if the collapsing header is open and content should be rendered.
    #[must_use]
    pub fn build_with_close_button(self, _: &Ui, opened: &mut bool) -> bool {
        unsafe {
            sys::igCollapsingHeaderBoolPtr(
                self.label.as_ptr(),
                opened as *mut bool,
                self.flags.bits() as i32,
            )
        }
    }
}
