use super::{ImGuiSetCond, ImGuiTreeNodeFlags, ImGuiTreeNodeFlags_Bullet,
            ImGuiTreeNodeFlags_DefaultOpen, ImGuiTreeNodeFlags_Leaf,
            ImGuiTreeNodeFlags_OpenOnArrow, ImGuiTreeNodeFlags_OpenOnDoubleClick,
            ImGuiTreeNodeFlags_Selected, Ui};
use imgui_sys;
use std::marker::PhantomData;

#[must_use]
pub struct TreeNode<'ui, 'p> {
    id: &'p str,
    label: Option<&'p str>,
    opened: bool,
    opened_cond: ImGuiSetCond,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> TreeNode<'ui, 'p> {
    pub fn new(id: &'p str) -> Self {
        TreeNode {
            id: id,
            label: None,
            opened: false,
            opened_cond: ImGuiSetCond::empty(),
            _phantom: PhantomData,
        }
    }
    #[inline]
    pub fn label(mut self, label: &'p str) -> Self {
        self.label = Some(label);
        self
    }
    #[inline]
    pub fn opened(mut self, opened: bool, cond: ImGuiSetCond) -> Self {
        self.opened = opened;
        self.opened_cond = cond;
        self
    }
    pub fn build<F: FnOnce()>(self, f: F) {
        let render = unsafe {
            if !self.opened_cond.is_empty() {
                imgui_sys::igSetNextTreeNodeOpen(self.opened, self.opened_cond);
            }
            match self.label {
                Some(label) => imgui_sys::igTreeNodeStr1(
                    imgui_sys::ImStr::from(self.id),
                    imgui_sys::ImStr::from(label),
                ),
                None => imgui_sys::igTreeNode(imgui_sys::ImStr::from(self.id)),
            }
        };
        if render {
            f();
            unsafe { imgui_sys::igTreePop() };
        }
    }
}

#[must_use]
pub struct CollapsingHeader<'ui, 'p> {
    label: &'p str,
    // Some flags are automatically set in ImGui::CollapsingHeader, so
    // we only support a sensible subset here
    flags: ImGuiTreeNodeFlags,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> CollapsingHeader<'ui, 'p> {
    pub fn new(label: &'p str) -> Self {
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
        self.flags.set(ImGuiTreeNodeFlags_Selected, value);
        self
    }
    #[inline]
    pub fn default_open(mut self, value: bool) -> Self {
        self.flags.set(ImGuiTreeNodeFlags_DefaultOpen, value);
        self
    }
    #[inline]
    pub fn open_on_double_click(mut self, value: bool) -> Self {
        self.flags.set(ImGuiTreeNodeFlags_OpenOnDoubleClick, value);
        self
    }
    #[inline]
    pub fn open_on_arrow(mut self, value: bool) -> Self {
        self.flags.set(ImGuiTreeNodeFlags_OpenOnArrow, value);
        self
    }
    #[inline]
    pub fn leaf(mut self, value: bool) -> Self {
        self.flags.set(ImGuiTreeNodeFlags_Leaf, value);
        self
    }
    #[inline]
    pub fn bullet(mut self, value: bool) -> Self {
        self.flags.set(ImGuiTreeNodeFlags_Bullet, value);
        self
    }
    pub fn build(self) -> bool {
        unsafe {
            imgui_sys::igCollapsingHeader(imgui_sys::ImStr::from(self.label), self.flags)
        }
    }
}
