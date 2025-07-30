//! Support for common multi-selection idioms.
//!
//! This module provides support for common multi-selection idioms such as Shift
//! for range-selection and Ctrl+A to select all.
//! The supported widget types are [`selectable`], [`checkbox`] and
//! [`tree_node`].
//!
//! The selection data is owned by your code, not by `imgui`.
//! To communicate selection changes, `imgui` provides you with a
//! [`MultiSelectIo`] struct at the beginning and end of a multi-selection,
//! which contains changes in the form of [`SelectionRequest`]s which you can
//! apply to the selection.
//! To allow `imgui` to refer to items in [`MultiSelectIo`], you need to assign
//! [`SelectionUserData`] to each item.
//! For more on how to handle selection data, see the [`storage`] module.
//!
//! There are different ways to use multi-select:
//!
//! * [`multi_select`] is the simplest method as it automatically handles
//!   selection changes.
//! * [`begin_multi_select`] lets you handle beginning and end of a
//!   multi-selection yourself.
//! * [`multi_select_config`] creates a builder which allows configuration of a
//!   multi-selection before starting it with [`build`](MultiSelect::build)
//!   (similarly to [`multi_select`]) or [`begin`](MultiSelect::begin)
//!   (similarly to [`begin_multi_select`]).
//!
//! Multi-select is compatible with [`ListClipper`].
//! [`MultiSelectIo::range_src_item`] must never be clipped, which can be
//! ensured with [`ListClipper::include_item_by_index`].
//!
//! # Examples
//!
//! Multi-select with a list of [`selectable`]s:
//!
//! ```
//! use imgui::Ui;
//! use imgui::multi_select::SelectionBasicStorage;
//!
//! fn show_selection(selection: &mut SelectionBasicStorage, items_count: i32, ui: &Ui) {
//!     // `multi_select` handles the changes to the selection.
//!     ui.multi_select(selection, items_count, |ms, selection| {
//!         for item in 0..items_count {
//!             // Use the current selection state.
//!             let selected = selection.contains(item as u32);
//!             // Assign index as the item's user data.
//!             ms.set_next_item_selection_user_data(item as i64);
//!             ui.selectable_config(format!("Item {item}"))
//!                 .selected(selected)
//!                 .build();
//!         }
//!     });
//! }
//! ```
//!
//! # `selection_size` and `items_count`
//!
//! `selection_size` and `items_count` can be specified when beginning a
//! multi-selection. `selection_size` is the number of currently selected items,
//! while `items_count` is the total number of selectable items.
//! Both are optional so you can omit them by using a value of -1 if they are
//! costly to compute. However, they are required for some features.
//!
//! `selection_size` affects the shortcut routing of the Escape key when using
//! [`MultiSelectFlags::CLEAR_ON_ESCAPE`], so the flag doesn't claim the key if
//! the selection is empty. You may also set it to 0/1 to indicate an
//! empty/non-empty selection.
//! `items_count` is made accessible through [`MultiSelectIo::items_count`] and
//! is required by some implementations of [`SelectionStorage`] and by
//! [`MultiSelectIo::request_items`].
//!
//! [`selectable`]: Ui::selectable
//! [`checkbox`]: Ui::checkbox
//! [`tree_node`]: Ui::tree_node
//! [`multi_select`]: Ui::multi_select
//! [`begin_multi_select`]: Ui::begin_multi_select
//! [`multi_select_config`]: Ui::multi_select_config
//! [`ListClipper`]: crate::list_clipper::ListClipper
//! [`ListClipper::include_item_by_index`]: crate::list_clipper::ListClipper::include_item_by_index

pub mod storage;

pub use self::storage::{SelectionBasicStorage, SelectionExternalStorage, SelectionStorage};

use std::mem::{self, ManuallyDrop};

use bitflags::bitflags;

use crate::internal::{ImVector, RawCast};
use crate::Ui;

bitflags! {
    /// Flags for multi-select.
    #[repr(transparent)]
    pub struct MultiSelectFlags: u32 {
        /// Disables selecting more than one item.
        ///
        /// This is available to allow single-selection code to share the same
        /// code/logic if desired. It essentially disables the main purpose of
        /// multi-select though.
        const SINGLE_SELECT = sys::ImGuiMultiSelectFlags_SingleSelect;
        /// Disables the Ctrl+A shortcut to select all.
        const NO_SELECT_ALL = sys::ImGuiMultiSelectFlags_NoSelectAll;
        /// Disables Shift+selection mouse/keyboard support.
        ///
        /// This guarantees that for all [`SetRange`] requests in [`MultiSelectIo`],
        /// `first_item` will be equal to `last_item`, even with box-selection. This
        /// allows not handling interpolation in [`SetRange`] requests.
        ///
        /// [`SetRange`]: SelectionRequest::SetRange
        const NO_RANGE_SELECT = sys::ImGuiMultiSelectFlags_NoRangeSelect;
        /// Disables selecting items when navigating.
        const NO_AUTO_SELECT = sys::ImGuiMultiSelectFlags_NoAutoSelect;
        /// Disables clearing the selection when navigating or selecting another item.
        const NO_AUTO_CLEAR = sys::ImGuiMultiSelectFlags_NoAutoClear;
        /// Disables clearing the selection when clicking/selecting an already selected
        /// item.
        const NO_AUTO_CLEAR_ON_RESELECT = sys::ImGuiMultiSelectFlags_NoAutoClearOnReselect;
        /// Enables box-selection with items with uniform x position and width (e.g.
        /// full row [`selectable`]s).
        ///
        /// Box-selection works better with a little bit of spacing between items in
        /// order to be able to aim at empty space.
        ///
        /// [`selectable`]: Ui::selectable
        const BOX_SELECT_1D = sys::ImGuiMultiSelectFlags_BoxSelect1d;
        /// Enables box-selection with items with varying x position or width (e.g.
        /// different width labels or 2D layout/grid).
        ///
        /// This is slower as it alters clipping logic so that e.g. horizontal movements
        /// will update the selection of normally clipped items.
        const BOX_SELECT_2D = sys::ImGuiMultiSelectFlags_BoxSelect2d;
        /// Disables scrolling when box-selecting near the edges of the scope.
        const BOX_SELECT_NO_SCROLL = sys::ImGuiMultiSelectFlags_BoxSelectNoScroll;
        /// Clears the selection when pressing Escape while the scope is focused.
        const CLEAR_ON_ESCAPE = sys::ImGuiMultiSelectFlags_ClearOnEscape;
        /// Clears the selection when clicking on an empty location within the scope.
        const CLEAR_ON_CLICK_VOID = sys::ImGuiMultiSelectFlags_ClearOnClickVoid;
        /// The scope for box-selection and `CLEAR_ON_CLICK_VOID` is the whole window
        /// (default).
        ///
        /// Use this if multi-select covers a whole window or is used a single time in
        /// the same window.
        const SCOPE_WINDOW = sys::ImGuiMultiSelectFlags_ScopeWindow;
        /// The scope for box-selection and `CLEAR_ON_CLICK_VOID` is a rectangle
        /// encompassing the multi-selection.
        ///
        /// Use this if multi-select is used multiple times in the same window.
        const SCOPE_RECT = sys::ImGuiMultiSelectFlags_ScopeRect;
        /// Applies selection on mouse press when clicking an unselected item (default).
        const SELECT_ON_CLICK = sys::ImGuiMultiSelectFlags_SelectOnClick;
        /// Applies selection on mouse release when clicking an unselected item.
        ///
        /// This allows dragging an unselected item without altering the selection.
        const SELECT_ON_CLICK_RELEASE = sys::ImGuiMultiSelectFlags_SelectOnClickRelease;

        // `ImGuiMultiSelectFlags_NavWrapX` is marked as temporary as of Dear ImGui
        // 1.91.0.
    }
}

/// A builder for a multi-selection.
///
/// This builder is created by [`Ui::multi_select_config`] and allows
/// configuration of a multi-selection before beginning it with
/// [`build`](Self::build) or [`begin`](Self::begin).
pub struct MultiSelect<'ui> {
    ui: &'ui Ui,
    selection_size: i32,
    items_count: i32,
    flags: MultiSelectFlags,
}

impl<'ui> MultiSelect<'ui> {
    /// The number of currently selected items.
    ///
    /// See the [`multi_select` documentation] for more.
    ///
    /// [`multi_select` documentation]: self#selection_size-and-items_count
    pub fn selection_size(mut self, selection_size: i32) -> Self {
        self.selection_size = selection_size;
        self
    }

    /// The total number of selectable items.
    ///
    /// See the [`multi_select` documentation] for more.
    ///
    /// [`multi_select` documentation]: self#selection_size-and-items_count
    pub fn items_count(mut self, items_count: i32) -> Self {
        self.items_count = items_count;
        self
    }

    /// The flags to be used.
    pub fn flags(mut self, flags: MultiSelectFlags) -> Self {
        self.flags = flags;
        self
    }

    /// Begins the multi-selection.
    ///
    /// The returned token represents the active multi-selection and provides access
    /// to the [`MultiSelectIo`] struct.
    ///
    /// If you don't need to handle beginning and end of the multi-selection
    /// manually, you may use [`build`](Self::build) instead.
    ///
    /// # Panics
    ///
    /// Panics if there already is an active multi-selection.
    pub fn begin(self) -> MultiSelectToken<'ui> {
        if self.ui.multi_select_lock.replace(true) {
            panic!("nested multi-select is not supported");
        }

        let ms_io = unsafe {
            sys::igBeginMultiSelect(
                self.flags.bits() as i32,
                self.selection_size,
                self.items_count,
            )
        };

        // SAFETY: `ImGuiMultiSelectIO` and `MultiSelectIo` have the same layout as
        // checked below.
        // The pointer lifetime according to Dear ImGui is limited by the next call to
        // `BeginMultiSelect` or `EndMultiSelect`, or by the end of the frame.
        // `BeginMultiSelect` is guarded dynamically by the check on
        // `Ui::multi_select_lock` above, `EndMultiSelect` consumes the token, and the
        // 'ui lifetime is limited to the end of the frame.
        let ms_io = unsafe { &mut *(ms_io as *mut MultiSelectIo) };

        MultiSelectToken {
            ui: self.ui,
            io: ms_io,
        }
    }

    /// Builds the multi-selection.
    ///
    /// The contents are built from the function `f`. It is provided with a
    /// reference to the [`MultiSelectToken`] and a reference to the selection so it
    /// can still be accessed.
    /// The `selection` is automatically updated at the beginning and end.
    ///
    /// If [`apply_requests`] for `selection` requires
    /// [`MultiSelectIo::items_count`], it should be set on this builder with
    /// [`items_count`](Self::items_count).
    /// If the selection size is not set with
    /// [`selection_size`](Self::selection_size), it is instead queried from
    /// `selection` with [`SelectionStorage::size`].
    ///
    /// If you need more control over beginning and end of the multi-selection, use
    /// [`begin`](Self::begin) instead.
    ///
    /// # Panics
    ///
    /// Panics if there already is an active multi-selection.
    ///
    /// [`apply_requests`]: SelectionStorage::apply_requests
    pub fn build<S, R, F>(mut self, selection: &mut S, f: F) -> R
    where
        S: SelectionStorage,
        F: FnOnce(&mut MultiSelectToken<'_>, &mut S) -> R,
    {
        if self.selection_size == -1 {
            self.selection_size = selection.size();
        }
        let mut ms = self.begin();
        selection.apply_requests(ms.io_mut());
        let result = f(&mut ms, selection);
        let mut ms = ms.end();
        selection.apply_requests(ms.io_mut());
        result
    }
}

impl Ui {
    /// Begins a multi-selection.
    ///
    /// This function returns a [`MultiSelectToken`], which allows you to handle
    /// beginning and end of the multi-selection manually. If you don't need this
    /// flexibility, consider using [`multi_select`](Self::multi_select) instead.
    /// For further configuration, see
    /// [`multi_select_config`](Self::multi_select_config).
    ///
    /// See the [`multi_select` module-level documentation](self) for more on
    /// multi-select and `selection_size`/`items_count`.
    ///
    /// # Panics
    ///
    /// Panics if there already is an active multi-selection.
    ///
    /// # Examples
    ///
    /// ```
    /// use imgui::Ui;
    /// use imgui::multi_select::{SelectionBasicStorage, SelectionStorage};
    ///
    /// fn show_selection(selection: &mut SelectionBasicStorage, items_count: i32, ui: &Ui) {
    ///     let mut ms = ui.begin_multi_select(selection.size(), items_count);
    ///     selection.apply_requests(ms.io_mut());
    ///
    ///     for item in 0..items_count {
    ///         let selected = selection.contains(item as u32);
    ///         ms.set_next_item_selection_user_data(item as i64);
    ///         ui.selectable_config(format!("Item {item}"))
    ///             .selected(selected)
    ///             .build();
    ///     }
    ///
    ///     let mut ms = ms.end();
    ///     selection.apply_requests(ms.io_mut());
    /// }
    /// ```
    pub fn begin_multi_select(
        &self,
        selection_size: i32,
        items_count: i32,
    ) -> MultiSelectToken<'_> {
        self.multi_select_config()
            .selection_size(selection_size)
            .items_count(items_count)
            .begin()
    }

    /// Creates a multi-selection.
    ///
    /// The contents are built from the function `f`. It is provided with a
    /// reference to the [`MultiSelectToken`] and a reference to the selection so it
    /// can still be accessed.
    /// The `selection` is automatically updated at the beginning and end.
    /// `items_count` is the total number of selectable items.
    ///
    /// If you need more control over beginning and end of the multi-selection, use
    /// [`begin_multi_select`](Self::begin_multi_select) instead. For further
    /// configuration, see [`multi_select_config`](Self::multi_select_config).
    ///
    /// See the [`multi_select` module-level documentation](self) for more.
    ///
    /// # Panics
    ///
    /// Panics if there already is an active multi-selection.
    ///
    /// # Examples
    ///
    /// ```
    /// use imgui::Ui;
    /// use imgui::multi_select::SelectionBasicStorage;
    ///
    /// fn show_selection(selection: &mut SelectionBasicStorage, items_count: i32, ui: &Ui) {
    ///     ui.multi_select(selection, items_count, |ms, selection| {
    ///         for item in 0..items_count {
    ///             let selected = selection.contains(item as u32);
    ///             ms.set_next_item_selection_user_data(item as i64);
    ///             ui.selectable_config(format!("Item {item}"))
    ///                 .selected(selected)
    ///                 .build();
    ///         }
    ///     });
    /// }
    /// ```
    pub fn multi_select<S, R, F>(&self, selection: &mut S, items_count: i32, f: F) -> R
    where
        S: SelectionStorage,
        F: FnOnce(&mut MultiSelectToken<'_>, &mut S) -> R,
    {
        self.multi_select_config()
            .items_count(items_count)
            .build(selection, f)
    }

    /// Creates a builder for a multi-selection.
    ///
    /// This is the most flexible way of using multi-select. If you don't need this
    /// flexibility, consider using [`multi_select`](Self::multi_select) or
    /// [`begin_multi_select`](Self::begin_multi_select) instead.
    ///
    /// See the [`multi_select` module-level documentation](self) for more.
    ///
    /// # Examples
    ///
    /// Setting [`MultiSelectFlags`]:
    ///
    /// ```
    /// use imgui::Ui;
    /// use imgui::multi_select::{MultiSelectFlags, SelectionBasicStorage};
    ///
    /// fn show_selection(selection: &mut SelectionBasicStorage, items_count: i32, ui: &Ui) {
    ///     ui.multi_select_config()
    ///         .flags(MultiSelectFlags::BOX_SELECT_1D | MultiSelectFlags::CLEAR_ON_ESCAPE)
    ///         .items_count(items_count)
    ///         .build(selection, |ms, selection| {
    ///             for item in 0..items_count {
    ///                 let selected = selection.contains(item as u32);
    ///                 ms.set_next_item_selection_user_data(item as i64);
    ///                 ui.selectable_config(format!("Item {item}"))
    ///                     .selected(selected)
    ///                     .build();
    ///             }
    ///         });
    /// }
    /// ```
    pub fn multi_select_config(&self) -> MultiSelect<'_> {
        MultiSelect {
            ui: self,
            selection_size: -1,
            items_count: -1,
            flags: MultiSelectFlags::empty(),
        }
    }
}

/// A token representing an active multi-selection.
///
/// Dropping the token ends the multi-selection. To manually handle the second
/// [`MultiSelectIo`] struct, call [`end`](Self::end) explicitly.
pub struct MultiSelectToken<'ui> {
    ui: &'ui Ui,
    io: &'ui mut MultiSelectIo,
}

impl<'ui> MultiSelectToken<'ui> {
    /// A reference to the [`MultiSelectIo`] struct.
    pub fn io(&self) -> &MultiSelectIo {
        self.io
    }

    /// A mutable reference to the [`MultiSelectIo`] struct.
    pub fn io_mut(&mut self) -> &mut MultiSelectIo {
        self.io
    }

    /// Assigns [`SelectionUserData`] to the next item.
    pub fn set_next_item_selection_user_data(&mut self, selection_user_data: SelectionUserData) {
        unsafe {
            sys::igSetNextItemSelectionUserData(selection_user_data);
        }
    }

    /// Returns `true` if the selection state of the previous item was toggled.
    ///
    /// This may be useful if you need per-item information immediately. However,
    /// this will only return toggle events, not selection updates in general.
    pub fn is_item_toggled_selection(&self) -> bool {
        unsafe { sys::igIsItemToggledSelection() }
    }

    /// Ends the multi-selection.
    ///
    /// The multi-selection also ends when this token is dropped, but `end` provides
    /// access to the corresponding [`MultiSelectIo`] struct.
    /// The multi-selection remains active until the returned
    /// [`MultiSelectEndToken`] is dropped.
    pub fn end(self) -> MultiSelectEndToken<'ui> {
        // We cannot drop `self` here because we need the pointer returned from
        // `EndMultiSelect` and we must not release `Ui::multi_select_lock` yet, so we
        // handle `EndMultiSelect` here instead.

        let self_ = ManuallyDrop::new(self);

        let ms_io = unsafe { sys::igEndMultiSelect() };
        // Layout is checked below.
        let ms_io = ms_io as *mut MultiSelectIo;
        // SAFETY: the pointer lifetime according to Dear ImGui is limited by the next
        // call to `BeginMultiSelect` or `EndMultiSelect`, or by the end of the frame.
        // `BeginMultiSelect` is guarded dynamically by `Ui::multi_select_lock`,
        // `EndMultiSelect` cannot be called without another call to `BeginMultiSelect`
        // as we prevent nested multi-selections, and the 'ui lifetime is limited to the
        // end of the frame.
        let ms_io = unsafe { &mut *ms_io };

        MultiSelectEndToken {
            ui: self_.ui,
            io: ms_io,
        }
    }
}

impl Drop for MultiSelectToken<'_> {
    fn drop(&mut self) {
        unsafe {
            sys::igEndMultiSelect();
        }
        self.ui.multi_select_lock.set(false);
    }
}

/// A token representing an ended multi-selection.
///
/// `MultiSelectEndToken` provides access to the second [`MultiSelectIo`]
/// struct. The token must be dropped before another multi-selection can be
/// started, which can be done explicitly by calling [`end`](Self::end).
pub struct MultiSelectEndToken<'ui> {
    ui: &'ui Ui,
    io: &'ui mut MultiSelectIo,
}

impl MultiSelectEndToken<'_> {
    /// A reference to the [`MultiSelectIo`] struct.
    pub fn io(&self) -> &MultiSelectIo {
        self.io
    }

    /// A mutable reference to the [`MultiSelectIo`] struct.
    pub fn io_mut(&mut self) -> &mut MultiSelectIo {
        self.io
    }

    /// Explicitly drops this token.
    pub fn end(self) {
        // Just drop `self`.
    }
}

impl Drop for MultiSelectEndToken<'_> {
    fn drop(&mut self) {
        self.ui.multi_select_lock.set(false);
    }
}

/// A type for interacting with multi-select.
///
/// This is returned when beginning or ending a multi-selection.
/// [`requests`](Self::requests) returns the selection changes that occurred,
/// which can be applied to the selection data with
/// [`SelectionStorage::apply_requests`].
/// `MultiSelectIo` also provides further information as well as
/// [`set_range_src_reset`](Self::set_range_src_reset) to support advanced use
/// cases such as multi-select with a [`ListClipper`] or a dynamic item list.
///
/// [`ListClipper`]: crate::list_clipper::ListClipper
#[repr(C)]
pub struct MultiSelectIo {
    // These requests must always represent valid instances of
    // `sys::ImGuiSelectionRequest`, i.e., all fields must be initialized even for
    // variants that don't have these fields. Therefore, these requests must never
    // be moved or edited in a way that might leave fields uninitialized. This is
    // required because instances of `MultiSelectIo` may be accessed by Dear ImGui.
    requests: ImVector<SelectionRequest>,
    range_src_item: SelectionUserData,
    nav_id_item: SelectionUserData,
    nav_id_selected: bool,
    range_src_reset: bool,
    items_count: i32,
}

impl MultiSelectIo {
    /// The changes to apply to the selection.
    pub fn requests(&self) -> &[SelectionRequest] {
        self.requests.as_slice()
    }

    /// An iterator over selection changes for individual items.
    ///
    /// `request_items` requires indices as [`SelectionUserData`].
    /// Additionally, [`items_count`](Self::items_count) must be set. Alternatively,
    /// you can provide it with
    /// [`request_items_with_items_count`](Self::request_items_with_items_count).
    pub fn request_items(&self) -> RequestItems<'_> {
        self.request_items_with_items_count(self.items_count)
    }

    /// An iterator over selection changes for individual items, using the given
    /// `items_count`.
    ///
    /// `request_items_with_items_count` requires indices as [`SelectionUserData`].
    pub fn request_items_with_items_count(&self, items_count: i32) -> RequestItems<'_> {
        RequestItems::new(self.requests(), items_count)
    }

    /// The source item for range-selection.
    ///
    /// When using multi-select with a [`ListClipper`], this item must never be
    /// clipped. [`ListClipper::include_item_by_index`] can be used to ensure it is
    /// submitted.
    ///
    /// [`ListClipper`]: crate::list_clipper::ListClipper
    /// [`ListClipper::include_item_by_index`]: crate::list_clipper::ListClipper::include_item_by_index
    pub fn range_src_item(&self) -> SelectionUserData {
        self.range_src_item
    }

    /// The currently focused item.
    pub fn nav_id_item(&self) -> SelectionUserData {
        self.nav_id_item
    }

    /// The selection state of the currently focused item.
    pub fn nav_id_selected(&self) -> bool {
        self.nav_id_selected
    }

    /// Requests to reset `range_src_item`.
    ///
    /// This can only be set if this `MultiSelectIo` belongs to the beginning of a
    /// multi-selection. `range_src_item` will be reset to the focused item.
    pub fn set_range_src_reset(&mut self, range_src_reset: bool) {
        self.range_src_reset = range_src_reset;
    }

    /// The total number of selectable items.
    ///
    /// This is the value set when beginning the multi-selection and is provided for
    /// use in [`SelectionStorage::apply_requests`].
    pub fn items_count(&self) -> i32 {
        self.items_count
    }
}

unsafe impl RawCast<sys::ImGuiMultiSelectIO> for MultiSelectIo {}

const _: () = {
    assert!(mem::size_of::<MultiSelectIo>() == mem::size_of::<sys::ImGuiMultiSelectIO>());
    assert!(mem::align_of::<MultiSelectIo>() == mem::align_of::<sys::ImGuiMultiSelectIO>());

    macro_rules! assert_field_offset {
        ($l:ident, $r:ident) => {
            assert!(
                mem::offset_of!(MultiSelectIo, $l) == mem::offset_of!(sys::ImGuiMultiSelectIO, $r)
            )
        };
    }
    assert_field_offset!(requests, Requests);
    assert_field_offset!(range_src_item, RangeSrcItem);
    assert_field_offset!(nav_id_item, NavIdItem);
    assert_field_offset!(nav_id_selected, NavIdSelected);
    assert_field_offset!(range_src_reset, RangeSrcReset);
    assert_field_offset!(items_count, ItemsCount);
};

/// A request to update selection data.
///
/// This is usually handled by an implementation of [`SelectionStorage`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum SelectionRequest {
    /// An empty request.
    None = sys::ImGuiSelectionRequestType_None,
    /// A request to set the selection state of all items.
    SetAll {
        /// The new selection state.
        selected: bool,
    } = sys::ImGuiSelectionRequestType_SetAll,
    /// A request to set the selection state of an item range.
    SetRange {
        /// The new selection state.
        selected: bool,
        /// The direction of selection.
        direction: SelectionDirection,
        /// The first item of the range.
        ///
        /// This is equal to [`range_src_item`] when selecting from top to bottom.
        ///
        /// [`range_src_item`]: MultiSelectIo::range_src_item
        first_item: SelectionUserData,
        /// The last item of the range, inclusive.
        ///
        /// This is equal to [`range_src_item`] when selecting from bottom to top.
        ///
        /// [`range_src_item`]: MultiSelectIo::range_src_item
        last_item: SelectionUserData,
    } = sys::ImGuiSelectionRequestType_SetRange,
}

const _: () = {
    // Important: despite these checks, `SelectionRequest` cannot always be
    // converted to `sys::ImGuiSelectionRequest`, because depending on the variant,
    // not all fields may be initialized. This is also why `SelectionRequest` does
    // not implement `RawCast`.

    assert!(mem::size_of::<SelectionRequest>() == mem::size_of::<sys::ImGuiSelectionRequest>());
    assert!(mem::align_of::<SelectionRequest>() == mem::align_of::<sys::ImGuiSelectionRequest>());

    // FIXME: offset computations can be simplified once offset_of_enum is
    // stabilized.

    let set_all = SelectionRequest::SetAll { selected: false };
    let SelectionRequest::SetAll { selected } = &set_all else {
        unreachable!()
    };
    assert!(
        unsafe { (selected as *const bool).byte_offset_from(&set_all) } as usize
            == mem::offset_of!(sys::ImGuiSelectionRequest, Selected)
    );

    let set_range = SelectionRequest::SetRange {
        selected: false,
        direction: SelectionDirection::TopToBottom,
        first_item: 0,
        last_item: 0,
    };
    let SelectionRequest::SetRange {
        selected,
        direction,
        first_item,
        last_item,
    } = &set_range
    else {
        unreachable!()
    };
    assert!(
        unsafe { (selected as *const bool).byte_offset_from(&set_range) } as usize
            == mem::offset_of!(sys::ImGuiSelectionRequest, Selected)
    );
    assert!(
        unsafe { (direction as *const SelectionDirection).byte_offset_from(&set_range) } as usize
            == mem::offset_of!(sys::ImGuiSelectionRequest, RangeDirection)
    );
    assert!(
        unsafe { (first_item as *const SelectionUserData).byte_offset_from(&set_range) } as usize
            == mem::offset_of!(sys::ImGuiSelectionRequest, RangeFirstItem)
    );
    assert!(
        unsafe { (last_item as *const SelectionUserData).byte_offset_from(&set_range) } as usize
            == mem::offset_of!(sys::ImGuiSelectionRequest, RangeLastItem)
    );
};

/// The direction of a selection.
///
/// The numeric value of the direction can be accessed by using a cast, for
/// example:
///
/// ```
/// use imgui::multi_select::SelectionDirection;
///
/// let increment = SelectionDirection::TopToBottom as i32;
/// assert_eq!(increment, 1);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i8)]
pub enum SelectionDirection {
    TopToBottom = 1,
    BottomToTop = -1,
}

/// User data to identify items in a multi-selection.
///
/// Each item in a multi-selection is assigned `SelectionUserData` with
/// [`MultiSelectToken::set_next_item_selection_user_data`]. These values are
/// then used by `imgui` to refer to items in [`MultiSelectIo`].
///
/// `SelectionUserData` is typically an index into the list of selectable items,
/// but you may also use any other identifier. However, some implementations of
/// [`SelectionStorage`] as well as the [`RequestItems`] iterator require
/// indices.
pub type SelectionUserData = i64;

/// A request to set the selection state of a single item.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SelectionRequestItem {
    /// The index of the item.
    pub index: i32,
    /// The new selection state.
    pub selected: bool,
}

/// An iterator over selection requests for individual items.
///
/// This struct is created by [`MultiSelectIo::request_items`] and
/// [`MultiSelectIo::request_items_with_items_count`].
pub struct RequestItems<'a> {
    requests: &'a [SelectionRequest],
    index: i32,
    items_count: i32,
}

impl<'a> RequestItems<'a> {
    fn new(requests: &'a [SelectionRequest], items_count: i32) -> Self {
        let mut self_ = Self {
            requests,
            index: -1,
            items_count,
        };

        self_.init_request();

        self_
    }

    fn init_request(&mut self) {
        self.index = match *self.requests {
            [SelectionRequest::SetRange {
                direction,
                first_item,
                last_item,
                ..
            }, ..] => match direction {
                SelectionDirection::TopToBottom => first_item as i32 - 1,
                SelectionDirection::BottomToTop => last_item as i32 + 1,
            },
            _ => -1,
        };
    }
}

impl Iterator for RequestItems<'_> {
    type Item = SelectionRequestItem;

    fn next(&mut self) -> Option<Self::Item> {
        let selected = loop {
            let request = self.requests.first()?;
            match *request {
                SelectionRequest::None => {}
                SelectionRequest::SetAll { selected } => {
                    self.index += 1;
                    if self.index < self.items_count {
                        break selected;
                    }
                }
                SelectionRequest::SetRange {
                    selected,
                    direction,
                    first_item,
                    last_item,
                } => {
                    self.index += direction as i32;
                    if match direction {
                        SelectionDirection::TopToBottom => self.index <= last_item as i32,
                        SelectionDirection::BottomToTop => self.index >= first_item as i32,
                    } {
                        break selected;
                    }
                }
            }

            self.requests = &self.requests[1..];
            self.init_request();
        };

        Some(SelectionRequestItem {
            index: self.index,
            selected,
        })
    }
}
