use imgui_sys;
use std::marker::PhantomData;

use super::{Ui, ImGuiSetCond, ImStr};

#[must_use]
pub struct TreeNode<'ui, 'p> {
    id: &'p str,
    label: Option<&'p str>,
    opened: bool,
    opened_cond: ImGuiSetCond,
    _phantom: PhantomData<&'ui Ui<'ui>>
}

impl<'ui, 'p> TreeNode<'ui, 'p> {
    pub fn new(id: &'p str) -> Self {
        TreeNode {
            id: id,
            label: None,
            opened: false,
            opened_cond: ImGuiSetCond::empty(),
            _phantom: PhantomData
        }
    }
    #[inline]
    pub fn label(self, label: &'p str) -> Self {
        TreeNode {
            label: Some(label),
            .. self
        }
    }
    #[inline]
    pub fn opened(self, opened: bool, cond: ImGuiSetCond) -> Self {
        TreeNode {
            opened: opened,
            opened_cond: cond,
            .. self
        }
    }
    pub fn build<F: FnOnce()>(self, f: F) {
        let id = imgui_sys::ImStr::from(self.id);
        let (label, len) = match self.label {
            Some(label) => (label.as_ptr(), label.len() as i32),
            None => (self.id.as_ptr(), self.id.len() as i32)
        };
        let render = unsafe {
            if !self.opened_cond.is_empty() {
                imgui_sys::igSetNextTreeNodeOpened(self.opened, self.opened_cond);
            }
            imgui_sys::igTreeNodeStr(id, super::fmt_ptr(), len, label)
        };
        if render {
            f();
            unsafe { imgui_sys::igTreePop() };
        }
    }
}
