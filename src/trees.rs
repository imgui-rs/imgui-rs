use sys;
use std::marker::PhantomData;

use super::{ImGuiCond, ImGuiTreeNodeFlags, ImStr, Ui};

#[must_use]
pub struct TreeNode<'ui, 'p> {
    id: &'p ImStr,
    label: Option<&'p ImStr>,
    opened: bool,
    opened_cond: ImGuiCond,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> TreeNode<'ui, 'p> {
    pub fn new(_: &Ui<'ui>, id: &'p ImStr) -> Self {
        TreeNode {
            id: id,
            label: None,
            opened: false,
            opened_cond: ImGuiCond::empty(),
            _phantom: PhantomData,
        }
    }
    #[inline]
    pub fn label(mut self, label: &'p ImStr) -> Self {
        self.label = Some(label);
        self
    }
    #[inline]
    pub fn opened(mut self, opened: bool, cond: ImGuiCond) -> Self {
        self.opened = opened;
        self.opened_cond = cond;
        self
    }
    pub fn build<F: FnOnce()>(self, f: F) {
        let render = unsafe {
            if !self.opened_cond.is_empty() {
                sys::igSetNextTreeNodeOpen(self.opened, self.opened_cond);
            }
            sys::igTreeNodeStr(
                self.id.as_ptr(),
                super::fmt_ptr(),
                self.label.unwrap_or(self.id).as_ptr(),
            )
        };
        if render {
            f();
            unsafe { sys::igTreePop() };
        }
    }
}

#[must_use]
pub struct CollapsingHeader<'ui, 'p> {
    label: &'p ImStr,
    // Some flags are automatically set in ImGui::CollapsingHeader, so
    // we only support a sensible subset here
    flags: ImGuiTreeNodeFlags,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> CollapsingHeader<'ui, 'p> {
    pub fn new(_: &Ui<'ui>, label: &'p ImStr) -> Self {
        CollapsingHeader {
            label: label,
            flags: ImGuiTreeNodeFlags::empty(),
            _phantom: PhantomData,
        }
    }
    #[inline]
    pub fn flags(mut self, flags: ImGuiTreeNodeFlags) -> Self {
        self.flags = flags;
        self
    }
    #[inline]
    pub fn selected(mut self, value: bool) -> Self {
        self.flags.set(ImGuiTreeNodeFlags::Selected, value);
        self
    }
    #[inline]
    pub fn default_open(mut self, value: bool) -> Self {
        self.flags.set(ImGuiTreeNodeFlags::DefaultOpen, value);
        self
    }
    #[inline]
    pub fn open_on_double_click(mut self, value: bool) -> Self {
        self.flags.set(ImGuiTreeNodeFlags::OpenOnDoubleClick, value);
        self
    }
    #[inline]
    pub fn open_on_arrow(mut self, value: bool) -> Self {
        self.flags.set(ImGuiTreeNodeFlags::OpenOnArrow, value);
        self
    }
    #[inline]
    pub fn leaf(mut self, value: bool) -> Self {
        self.flags.set(ImGuiTreeNodeFlags::Leaf, value);
        self
    }
    #[inline]
    pub fn bullet(mut self, value: bool) -> Self {
        self.flags.set(ImGuiTreeNodeFlags::Bullet, value);
        self
    }
    pub fn build(self) -> bool {
        unsafe { sys::igCollapsingHeader(self.label.as_ptr(), self.flags) }
    }
}
