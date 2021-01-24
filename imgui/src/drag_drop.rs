use std::{ffi, marker::PhantomData, ptr};

use crate::{sys, Condition, ImStr, Ui};
use bitflags::bitflags;

bitflags!(
    /// Flags for igBeginDragDropSource(), igAcceptDragDropPayload()
    #[repr(transparent)]
    pub struct DragDropFlags: u32 {
        /// By default, a successful call to igBeginDragDropSource opens a tooltip so you can
        /// display a preview or description of the source contents. This flag disable this
        /// behavior.
        const SOURCE_NO_PREVIEW_TOOLTIP = sys::ImGuiDragDropFlags_SourceNoPreviewTooltip;
        /// By default, when dragging we clear data so that igIsItemHovered() will return false, to
        /// avoid subsequent user code submitting tooltips. This flag disable this behavior so you
        /// can still call igIsItemHovered() on the source item.
        const SOURCE_NO_DISABLE_HOVER = sys::ImGuiDragDropFlags_SourceNoDisableHover;
        /// Disable the behavior that allows to open tree nodes and collapsing header by holding
        /// over them while dragging a source item.
        const SOURCE_NO_HOLD_TO_OPEN_OTHERS = sys::ImGuiDragDropFlags_SourceNoHoldToOpenOthers;
        /// Allow items such as igText(), igImage() that have no unique identifier to be used as
        /// drag source, by manufacturing a temporary identifier based on their window-relative
        /// position. This is extremely unusual within the dear imgui ecosystem and so we made it
        /// explicit.
        const SOURCE_ALLOW_NULL_ID = sys::ImGuiDragDropFlags_SourceAllowNullID;
        /// External source (from outside of imgui), won't attempt to read current item/window
        /// info. Will always return true. Only one Extern source can be active simultaneously.
        const SOURCE_EXTERN = sys::ImGuiDragDropFlags_SourceExtern;
        /// Automatically expire the payload if the source ceases to be submitted (otherwise
        /// payloads are persisting while being dragged)
        const SOURCE_AUTO_EXPIRE_PAYLOAD = sys::ImGuiDragDropFlags_SourceAutoExpirePayload;
        /// igAcceptDragDropPayload() will returns true even before the mouse button is released.
        /// You can then call igIsDelivery() to test if the payload needs to be delivered.
        const ACCEPT_BEFORE_DELIVERY = sys::ImGuiDragDropFlags_AcceptBeforeDelivery;
        /// Do not draw the default highlight rectangle when hovering over target.
        const ACCEPT_NO_DRAW_DEFAULT_RECT = sys::ImGuiDragDropFlags_AcceptNoDrawDefaultRect;
        /// Request hiding the igBeginDragDropSource tooltip from the igBeginDragDropTarget site.
        const ACCEPT_NO_PREVIEW_TOOLTIP = sys::ImGuiDragDropFlags_AcceptNoPreviewTooltip;
        /// For peeking ahead and inspecting the payload before delivery. This is just a convenience
        /// flag for the intersection of `ACCEPT_BEFORE_DELIVERY` and `ACCEPT_NO_DRAW_DEFAULT_RECT`
        const ACCEPT_PEEK_ONLY = sys::ImGuiDragDropFlags_AcceptPeekOnly;
    }
);

#[derive(Debug)]
pub struct DragDropSource<'a> {
    name: &'a ImStr,
    flags: DragDropFlags,
    payload: *const ffi::c_void,
    size: usize,
    cond: Condition,
}

impl<'a> DragDropSource<'a> {
    /// Creates a new [DragDropSource] with no flags and the `Condition::Always` with the given name.
    /// ImGui refers to this `name` field as a `type`, but really it's just an identifier to match up
    /// Source/Target for DragDrop.
    pub fn new(name: &'a ImStr) -> Self {
        Self {
            name,
            flags: DragDropFlags::empty(),
            payload: ptr::null(),
            size: 0,
            cond: Condition::Always,
        }
    }

    /// Creates a new [DragDropSource] with no flags and the `Condition::Always` with the given name.
    /// ImGui refers to this `name` field as a `type`, but really it's just an identifier to match up
    /// Source/Target for DragDrop.
    ///
    /// This payload will be passed to ImGui, which will provide it to
    /// a target when it runs [accept_drag_drop_payload](DragDropTarget::accept_drag_drop_payload).
    ///
    /// ## Safety
    /// This function is not inherently unsafe, and won't panic itself, but using it opts you into
    /// managing the lifetime yourself. When you dereference the pointer given in [accept_drag_drop_payload](DragDropTarget::accept_drag_drop_payload),
    /// you can easily create memory safety problems.
    pub unsafe fn payload<T>(name: &'a ImStr, payload: *const T) -> Self {
        let mut output = Self::new(name);
        output.payload = payload as *const ffi::c_void;
        output.size = std::mem::size_of::<T>();
        output
    }

    /// Sets the flags on the [DragDropSource]. Only the flags `SOURCE_NO_PREVIEW_TOOLTIP`,
    /// `SOURCE_NO_DISABLE_HOVER`, `SOURCE_NO_HOLD_TO_OPEN_OTHERS`, `SOURCE_ALLOW_NULL_ID`,
    /// `SOURCE_EXTERN`, `SOURCE_AUTO_EXPIRE_PAYLOAD` make semantic sense, but any other flags will
    /// be accepted without panic.
    pub fn flags(mut self, flags: DragDropFlags) -> Self {
        self.flags = flags;
        self
    }

    /// Sets the condition on the [DragDropSource]. Defaults to [Always](Condition::Always).
    pub fn condition(mut self, cond: Condition) -> Self {
        self.cond = cond;
        self
    }

    /// When this returns true you need to: a) call SetDragDropPayload() exactly once, b) you may render the payload visual/description, c) call EndDragDropSource()
    pub fn begin<'ui>(self, _ui: &'ui Ui) -> Option<DragDropSourceToolTip<'ui>> {
        let should_begin = unsafe { sys::igBeginDragDropSource(self.flags.bits() as i32) };

        if should_begin {
            unsafe {
                sys::igSetDragDropPayload(
                    self.name.as_ptr(),
                    self.payload,
                    self.size,
                    self.cond as i32,
                );

                Some(DragDropSourceToolTip::push())
            }
        } else {
            None
        }
    }
}

/// A helper struct for RAII drap-drop support.
pub struct DragDropSourceToolTip<'ui>(PhantomData<Ui<'ui>>);

impl DragDropSourceToolTip<'_> {
    /// Creates a new tooltip internally.
    fn push() -> Self {
        Self(PhantomData)
    }

    /// Ends the tooltip directly. You could choose to simply allow this to drop
    /// by not calling this, which will also be fine.
    pub fn pop(self) {
        // left empty to invoke drop...
    }
}

impl Drop for DragDropSourceToolTip<'_> {
    fn drop(&mut self) {
        unsafe { sys::igEndDragDropSource() }
    }
}

#[derive(Debug)]
pub struct DragDropPayload {
    /// Data which is copied and owned by ImGui. If you have accepted the payload, you can
    /// take ownership of the data; otherwise, view it immutably. Interacting with `data` is
    /// very unsafe.
    /// @fixme: this doesn't make a ton of sense.
    pub data: *const ffi::c_void,
    /// Set when [`accept_drag_drop_payload`](Self::accept_drag_drop_payload) was called
    /// and mouse has been hovering the target item (nb: handle overlapping drag targets).
    /// @fixme: literally what does this mean -- I believe this is false on the first
    /// frame when source hovers over target and then is subsequently true? but I'm not sure
    /// when this matters. If DragDropFlags::ACCEPT_NO_PREVIEW is set, it doesn't make a difference
    /// to this flag.
    pub preview: bool,

    /// Set when AcceptDragDropPayload() was called and mouse button is released over the target item.
    /// If this is set to false, then you set DragDropFlags::ACCEPT_BEFORE_DELIVERY and shouldn't
    /// mess with `data`
    /// @fixme: obviously this isn't an impressive implementation of ffi data mutability.
    pub delivery: bool,
}

#[derive(Debug)]
pub struct DragDropTarget<'ui>(PhantomData<Ui<'ui>>);

impl<'ui> DragDropTarget<'ui> {
    pub fn new(_ui: &Ui<'_>) -> Option<Self> {
        let should_begin = unsafe { sys::igBeginDragDropTarget() };
        if should_begin {
            Some(Self(PhantomData))
        } else {
            None
        }
    }

    /// Accepts, popping the drag_drop payload, if it exists. If `DragDropFlags::ACCEPT_BEFORE_DELIVERY` is
    /// set, this function will return `Some` even if the type is wrong as long as there is a payload to accept.
    /// How do we possibly handle communicating that this data is somewhat immutable?
    pub fn accept_drag_drop_payload(
        &self,
        name: &ImStr,
        flags: DragDropFlags,
    ) -> Option<DragDropPayload> {
        unsafe {
            let inner = sys::igAcceptDragDropPayload(name.as_ptr(), flags.bits() as i32);
            if inner.is_null() {
                None
            } else {
                let inner = *inner;

                // @fixme: there are actually other fields on `inner` which I have shorn -- they're
                // considered internal to imgui (such as id of who sent this), so i've left it for
                // now this way.
                Some(DragDropPayload {
                    data: inner.Data,
                    preview: inner.Preview,
                    delivery: inner.Delivery,
                })
            }
        }
    }

    /// Ends the current target. Ironically, this doesn't really do
    /// anything in ImGui or in imgui-rs, but it might in the future.
    pub fn pop(self) {
        // omitted...exists just to run Drop.
    }
}

impl Drop for DragDropTarget<'_> {
    fn drop(&mut self) {
        unsafe { sys::igEndDragDropTarget() }
    }
}
