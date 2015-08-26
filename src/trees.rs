use imgui_sys;
use std::marker::PhantomData;

use super::{Ui, ImGuiSetCond, ImStr};

pub struct TreeNode<'ui, 'p> {
    id: ImStr<'p>,
    label: Option<ImStr<'p>>,
    opened: bool,
    opened_cond: ImGuiSetCond,
    _phantom: PhantomData<&'ui Ui<'ui>>
}

impl<'ui, 'p> TreeNode<'ui, 'p> {
    pub fn new(id: ImStr<'p>) -> Self {
        TreeNode {
            id: id,
            label: None,
            opened: false,
            opened_cond: ImGuiSetCond::empty(),
            _phantom: PhantomData
        }
    }
    #[inline]
    pub fn label(self, label: ImStr<'p>) -> Self {
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
        let render = unsafe {
            if !self.opened_cond.is_empty() {
                imgui_sys::igSetNextTreeNodeOpened(self.opened, self.opened_cond);
            }
            imgui_sys::igTreeNodeStr(
                self.id.as_ptr(),
                super::fmt_ptr(),
                self.label.unwrap_or(self.id).as_ptr()
            )
        };
        if render {
            f();
            unsafe { imgui_sys::igTreePop() };
        }
    }
}
