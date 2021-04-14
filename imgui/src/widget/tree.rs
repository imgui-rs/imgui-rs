use bitflags::bitflags;
use std::os::raw::{c_char, c_void};

use crate::string::ImStr;
use crate::sys;
use crate::{Condition, Ui};

bitflags!(
    /// Flags for tree nodes
    #[repr(transparent)]
    pub struct TreeNodeFlags: u32 {
        /// Draw as selected
        const SELECTED = sys::ImGuiTreeNodeFlags_Selected;
        /// Full colored frame (e.g. for CollapsingHeader)
        const FRAMED = sys::ImGuiTreeNodeFlags_Framed;
        /// Hit testing to allow subsequent widgets to overlap this one
        const ALLOW_ITEM_OVERLAP = sys::ImGuiTreeNodeFlags_AllowItemOverlap;
        /// Don't push a tree node when open (e.g. for CollapsingHeader) = no extra indent nor
        /// pushing on ID stack
        const NO_TREE_PUSH_ON_OPEN = sys::ImGuiTreeNodeFlags_NoTreePushOnOpen;
        /// Don't automatically and temporarily open node when Logging is active (by default
        /// logging will automatically open tree nodes)
        const NO_AUTO_OPEN_ON_LOG = sys::ImGuiTreeNodeFlags_NoAutoOpenOnLog;
        /// Default node to be open
        const DEFAULT_OPEN = sys::ImGuiTreeNodeFlags_DefaultOpen;
        /// Need double-click to open node
        const OPEN_ON_DOUBLE_CLICK = sys::ImGuiTreeNodeFlags_OpenOnDoubleClick;
        /// Only open when clicking on the arrow part.
        ///
        /// If `TreeNodeFlags::OPEN_ON_DOUBLE_CLICK` is also set, single-click arrow or
        /// double-click all box to open.
        const OPEN_ON_ARROW = sys::ImGuiTreeNodeFlags_OpenOnArrow;
        /// No collapsing, no arrow (use as a convenience for leaf nodes)
        const LEAF = sys::ImGuiTreeNodeFlags_Leaf;
        /// Display a bullet instead of arrow
        const BULLET = sys::ImGuiTreeNodeFlags_Bullet;
        /// Use `Style::frame_padding` (even for an unframed text node) to vertically align text
        /// baseline to regular widget height.
        ///
        /// Equivalent to calling `Ui::align_text_to_frame_padding`.
        const FRAME_PADDING = sys::ImGuiTreeNodeFlags_FramePadding;
        /// Extend hit box to the right-most edge, even if not framed.
        ///
        /// This is not the default in order to allow adding other items on the same line. In the
        /// future we may refactor the hit system to be front-to-back, allowing natural overlaps
        /// and then this can become the default.
        const SPAN_AVAIL_WIDTH = sys::ImGuiTreeNodeFlags_SpanAvailWidth;
        /// Extend hit box to the left-most and right-most edges (bypass the indented area)
        const SPAN_FULL_WIDTH = sys::ImGuiTreeNodeFlags_SpanFullWidth;
        /// (WIP) Nav: left direction may move to this tree node from any of its child
        const NAV_LEFT_JUMPS_BACK_HERE = sys::ImGuiTreeNodeFlags_NavLeftJumpsBackHere;
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
    #[inline]
    fn from(s: &'a T) -> Self {
        TreeNodeId::Str(s.as_ref())
    }
}

impl<T> From<*const T> for TreeNodeId<'static> {
    #[inline]
    fn from(p: *const T) -> Self {
        TreeNodeId::Ptr(p as *const c_void)
    }
}

impl<T> From<*mut T> for TreeNodeId<'static> {
    #[inline]
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
    /// rendered, the token can be popped by calling `.pop()`.
    ///
    /// Returns `None` if the tree node is not open and no content should be rendered.
    pub fn push<'ui>(self, ui: &Ui<'ui>) -> Option<TreeNodeToken<'ui>> {
        let open = unsafe {
            if self.opened_cond != Condition::Never {
                sys::igSetNextItemOpen(self.opened, self.opened_cond as i32);
            }
            match self.id {
                TreeNodeId::Str(id) => sys::igTreeNodeEx_StrStr(
                    id.as_ptr(),
                    self.flags.bits() as i32,
                    fmt_ptr(),
                    self.label.unwrap_or(id).as_ptr(),
                ),
                TreeNodeId::Ptr(id) => sys::igTreeNodeEx_Ptr(
                    id,
                    self.flags.bits() as i32,
                    fmt_ptr(),
                    self.label.unwrap_or_default().as_ptr(),
                ),
            }
        };
        if open {
            Some(TreeNodeToken::new(
                ui,
                !self.flags.contains(TreeNodeFlags::NO_TREE_PUSH_ON_OPEN),
            ))
        } else {
            None
        }
    }
    /// Creates a tree node and runs a closure to construct the contents.
    /// Returns the result of the closure, if it is called.
    ///
    /// Note: the closure is not called if the tree node is not open.
    pub fn build<T, F: FnOnce() -> T>(self, ui: &Ui<'_>, f: F) -> Option<T> {
        self.push(ui).map(|_node| f())
    }
}

/// Tracks a tree node that can be popped by calling `.pop()`, `end()`, or by dropping.
///
/// If `TreeNodeFlags::NO_TREE_PUSH_ON_OPEN` was used when this token was created, calling `.pop()`
/// is not mandatory and is a no-op.
#[must_use]
pub struct TreeNodeToken<'a>(core::marker::PhantomData<crate::Ui<'a>>, bool);

impl<'a> TreeNodeToken<'a> {
    /// Creates a new token type. This takes a bool for the no-op variant on NO_TREE_PUSH_ON_OPEN.
    pub(crate) fn new(_: &crate::Ui<'a>, execute_drop: bool) -> Self {
        Self(std::marker::PhantomData, execute_drop)
    }

    /// Pops a tree node
    #[inline]
    pub fn end(self) {
        // left empty for drop
    }

    /// Pops a tree node
    #[inline]
    pub fn pop(self) {
        self.end()
    }
}

impl Drop for TreeNodeToken<'_> {
    #[doc(alias = "TreePop")]
    fn drop(&mut self) {
        if self.1 {
            unsafe { sys::igTreePop() }
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
    #[doc(alias = "CollapsingHeader")]
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
            sys::igCollapsingHeader_TreeNodeFlags(self.label.as_ptr(), self.flags.bits() as i32)
        }
    }
    /// Builds the collapsing header, and adds an additional close button that changes the value of
    /// the given mutable reference when clicked.
    ///
    /// Returns true if the collapsing header is open and content should be rendered.
    #[must_use]
    pub fn build_with_close_button(self, _: &Ui, opened: &mut bool) -> bool {
        unsafe {
            sys::igCollapsingHeader_BoolPtr(
                self.label.as_ptr(),
                opened as *mut bool,
                self.flags.bits() as i32,
            )
        }
    }
}
