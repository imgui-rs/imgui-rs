//! Storage for selection data.
//!
//! Selection data refers to the selection state of each item.
//! The task of a selection storage is to store this data, and to update it
//! based on the [`SelectionRequest`]s in [`MultiSelectIo`].
//! The [`SelectionStorage`] trait can be used for this purpose:
//!
//! * [`SelectionBasicStorage`] is a basic implementation of
//!   [`SelectionStorage`].
//! * [`SelectionExternalStorage`] allows you to use an existing storage without
//!   manually implementing [`SelectionStorage`].
//! * You can implement [`SelectionStorage`] for your own storage type. Due to
//!   the orphan rules, this is only possible for types you define in the same
//!   crate.
//!
//! You can also store selection data and handle [`SelectionRequest`]s without
//! any [`SelectionStorage`] implementation, but [`SelectionStorage`] allows you
//! to use [`Ui::multi_select`] and [`MultiSelect::build`] to update the
//! selection storage automatically.
//!
//! [`SelectionRequest`]: super::SelectionRequest
//! [`Ui::multi_select`]: crate::Ui::multi_select
//! [`MultiSelect::build`]: super::MultiSelect::build

use std::borrow::Borrow;
use std::ffi::c_void;
use std::mem::{self, ManuallyDrop, MaybeUninit};
use std::ptr;

use crate::internal::RawCast;

use super::MultiSelectIo;

/// A storage for selection data.
///
/// See the [module-level documentation](self) for more.
pub trait SelectionStorage {
    /// Updates this selection with [`MultiSelectIo`].
    ///
    /// This function applies the [`SelectionRequest`]s of [`MultiSelectIo`] to this
    /// selection.
    /// It may rely on [`MultiSelectIo::items_count`], in which case the
    /// `items_count` must be set when beginning the multi-selection. It may also
    /// rely on [`SelectionUserData`] representing indices.
    ///
    /// [`SelectionRequest`]: super::SelectionRequest
    /// [`SelectionUserData`]: super::SelectionUserData
    fn apply_requests(&mut self, ms_io: &mut MultiSelectIo);

    /// The number of selected items.
    ///
    /// This may also return -1 if the number is costly to compute, or 0/1 to only
    /// indicate that the selection is empty/non-empty.
    fn size(&self) -> i32 {
        -1
    }
}

/// A basic [`SelectionStorage`].
///
/// `SelectionBasicStorage` stores selected items as `u32` identifiers.
/// It requires [`SelectionUserData`] to represent indices. By default, these
/// indices are also used as persistent identifiers.
/// [`set_adapter_index_to_storage_id`] can be used to define a different
/// mapping of indices to identifiers, which is typically required if the list
/// of items can change.
///
/// `SelectionBasicStorage` requires `items_count` to be set when beginning a
/// multi-selection.
///
/// [`SelectionUserData`]: super::SelectionUserData
/// [`set_adapter_index_to_storage_id`]: Self::set_adapter_index_to_storage_id
pub struct SelectionBasicStorage {
    inner: *mut sys::ImGuiSelectionBasicStorage,
    drop_adapter_index_to_storage_id: unsafe fn(*mut c_void),
}

impl SelectionBasicStorage {
    /// Creates an empty `SelectionBasicStorage`.
    pub fn new() -> Self {
        let inner = unsafe { sys::ImGuiSelectionBasicStorage_ImGuiSelectionBasicStorage() };
        Self {
            inner,
            drop_adapter_index_to_storage_id: |_| {},
        }
    }

    /// Returns `true` if the item with `id` is selected.
    pub fn contains(&self, id: u32) -> bool {
        unsafe { sys::ImGuiSelectionBasicStorage_Contains(self.inner, id) }
    }

    /// An iterator over the ids in this selection.
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            inner: IterInner::new(self),
        }
    }

    /// Sets the selection state of the item with `id`.
    ///
    /// To update the selection with [`MultiSelectIo`], use
    /// [`apply_requests`](Self::apply_requests) instead.
    pub fn set_item_selected(&mut self, id: u32, selected: bool) {
        unsafe {
            sys::ImGuiSelectionBasicStorage_SetItemSelected(self.inner, id, selected);
        }
    }

    /// Clears the selection.
    pub fn clear(&mut self) {
        unsafe {
            sys::ImGuiSelectionBasicStorage_Clear(self.inner);
        }
    }

    /// Swaps this selection with another.
    ///
    /// Only the selected items are swapped. The mappings of indices to identifiers
    /// set with [`set_adapter_index_to_storage_id`] remain unchanged.
    ///
    /// [`set_adapter_index_to_storage_id`]: Self::set_adapter_index_to_storage_id
    pub fn swap(&mut self, other: &mut Self) {
        unsafe {
            sys::ImGuiSelectionBasicStorage_Swap(self.inner, other.inner);
        }
    }

    /// Returns the id for `index`.
    ///
    /// By default, indices are returned unchanged. The mapping can be set with
    /// [`set_adapter_index_to_storage_id`](Self::set_adapter_index_to_storage_id).
    pub fn get_storage_id_from_index(&self, index: i32) -> u32 {
        unsafe { sys::ImGuiSelectionBasicStorage_GetStorageIdFromIndex(self.inner, index) }
    }

    /// Defines how indices are mapped to ids.
    ///
    /// By default, indices are used as persistent identifiers unchanged. Defining a
    /// different mapping is typically necessary when the list of selectable items
    /// is dynamic, for example when supporting deletion.
    pub fn set_adapter_index_to_storage_id<F>(&mut self, index_to_storage_id: F)
    where
        // `Fn` is required because `index_to_storage_id` might be called recursively
        // through `Self::get_storage_id_from_index`. 'static is required because the
        // lifetime of `index_to_storage_id` within the storage is not limited.
        F: Fn(i32) -> u32 + 'static,
    {
        // We cannot store `index_to_storage_id` directly as the callback, since it is a
        // closure and not a plain function pointer, and it has the wrong parameters and
        // ABI. Instead, we store it in `UserData` and use an intermediary function as
        // adapter.

        // This can also be implemented with trait objects instead of generics with
        // `Box<Box<dyn Fn(i32) -> u32>>`, but that requires another level of
        // indirection since `Box<dyn Fn(i32) -> u32>` is a wide pointer, so this incurs
        // the costs of this indirection, dynamic dispatch and poorer optimization.

        unsafe extern "C" fn adapter<F>(
            self_: *mut sys::ImGuiSelectionBasicStorage,
            index: i32,
        ) -> u32
        where
            F: Fn(i32) -> u32 + 'static,
        {
            // SAFETY: the closure is owned by the storage and its type has a 'static
            // lifetime. When this function is called, we have at least a shared reference
            // to the storage, so we can take a shared reference to the closure. Note that
            // taking a mutable reference here would be unsound since the closure can call
            // itself recursively (through `Self::get_storage_id_from_index`).
            let index_to_storage_id = unsafe { (*self_).UserData } as *const F;
            let index_to_storage_id = unsafe { &*index_to_storage_id };
            index_to_storage_id(index)
        }

        unsafe fn drop_adapter<F>(index_to_storage_id: *mut c_void) {
            let _ = unsafe { Box::from_raw(index_to_storage_id as *mut F) };
        }

        let inner = unsafe { &mut *self.inner };

        // `inner.UserData` and `self.drop_adapter_index_to_storage_id` must be
        // overwritten before the latter is called so that if it panics, it cannot be
        // called again, and `UserData` can no longer be accessed.
        let old_user_data = inner.UserData;
        let old_drop_adapter = self.drop_adapter_index_to_storage_id;

        let index_to_storage_id = Box::into_raw(Box::new(index_to_storage_id));
        inner.UserData = index_to_storage_id as *mut c_void;
        inner.AdapterIndexToStorageId = Some(adapter::<F>);
        self.drop_adapter_index_to_storage_id = drop_adapter::<F>;

        unsafe {
            old_drop_adapter(old_user_data);
        }
    }
}

impl Drop for SelectionBasicStorage {
    fn drop(&mut self) {
        unsafe {
            (self.drop_adapter_index_to_storage_id)((*self.inner).UserData);
            sys::ImGuiSelectionBasicStorage_destroy(self.inner);
        }
    }
}

impl Default for SelectionBasicStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl SelectionStorage for SelectionBasicStorage {
    fn apply_requests(&mut self, ms_io: &mut MultiSelectIo) {
        unsafe {
            sys::ImGuiSelectionBasicStorage_ApplyRequests(self.inner, ms_io.raw_mut());
        }
    }

    fn size(&self) -> i32 {
        unsafe { (*self.inner).Size }
    }
}

impl IntoIterator for SelectionBasicStorage {
    type Item = u32;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: IterInner::new(self),
        }
    }
}

impl<'a> IntoIterator for &'a SelectionBasicStorage {
    type Item = u32;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// An iterator over a [`SelectionBasicStorage`] that takes ownership of the
/// storage.
///
/// This struct is created by [`SelectionBasicStorage::into_iter`].
pub struct IntoIter {
    inner: IterInner<SelectionBasicStorage>,
}

impl Iterator for IntoIter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

/// An iterator over a [`SelectionBasicStorage`].
///
/// This struct is created by [`SelectionBasicStorage::iter`].
pub struct Iter<'a> {
    inner: IterInner<&'a SelectionBasicStorage>,
}

impl Iterator for Iter<'_> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

struct IterInner<S: Borrow<SelectionBasicStorage>> {
    storage: S,
    iter: *mut c_void,
    remaining: i32,
}

impl<S: Borrow<SelectionBasicStorage>> IterInner<S> {
    fn new(storage: S) -> Self {
        let size = storage.borrow().size();
        Self {
            storage,
            iter: ptr::null_mut(),
            remaining: size,
        }
    }

    fn next(&mut self) -> Option<u32> {
        if self.remaining > 0 {
            self.remaining -= 1;

            let mut id = MaybeUninit::<u32>::uninit();
            Some(unsafe {
                sys::ImGuiSelectionBasicStorage_GetNextSelectedItem(
                    self.storage.borrow().inner,
                    &raw mut self.iter,
                    id.as_mut_ptr(),
                );
                id.assume_init()
            })
        } else {
            None
        }
    }
}

/// A wrapper to use an existing storage for selection data.
///
/// `SelectionExternalStorage` allows you to use a storage as
/// [`SelectionStorage`] without a separate implementation of that trait, for
/// example if such an implementation is not allowed by the orphan rules.
///
/// `SelectionExternalStorage` requires indices as [`SelectionUserData`], and
/// `items_count` must be set when beginning the multi-selection.
///
/// [`SelectionUserData`]: super::SelectionUserData
pub struct SelectionExternalStorage<S> {
    inner: *mut sys::ImGuiSelectionExternalStorage,
    drop_adapter_set_item_selected: unsafe fn(*mut c_void) -> S,
    adapter_size: Box<dyn Fn(&S) -> i32>,
}

impl<S> SelectionExternalStorage<S> {
    /// Wraps a selection in a `SelectionExternalStorage`.
    ///
    /// `set_item_selected` is used by [`apply_requests`](Self::apply_requests) to
    /// set the selection state of individual items. `size` is used by
    /// [`Self::size`].
    ///
    /// # Examples
    ///
    /// Wrapping a [`Vec<bool>`] that stores the selection state of a fixed number
    /// of items:
    ///
    /// ```
    /// use imgui::multi_select::SelectionExternalStorage;
    ///
    /// let items_count = 10;
    /// let selection = SelectionExternalStorage::new(
    ///     vec![false; items_count],
    ///     |selection, index, selected| selection[index as usize] = selected,
    ///     |selection| selection.iter().filter(|selected| **selected).count() as i32,
    /// );
    /// ```
    pub fn new<FSet, FSize>(selection: S, set_item_selected: FSet, size: FSize) -> Self
    where
        FSet: FnMut(&mut S, i32, bool) + 'static,
        FSize: Fn(&S) -> i32 + 'static,
    {
        // The implementation regarding `set_item_selected` is similar to
        // `SelectionBasicStorage::set_adapter_index_to_storage_id`. Refer to that
        // function for more.

        #[repr(C)]
        struct UserData<S, FSet> {
            selection: S,
            set_item_selected: FSet,
        }

        const {
            // This is ensured by `repr(C)` and field order and is required to access
            // `selection` without knowing `FSet`.
            assert!(mem::offset_of!(UserData<S, FSet>, selection) == 0);
        }

        unsafe extern "C" fn adapter<S, FSet>(
            self_: *mut sys::ImGuiSelectionExternalStorage,
            index: i32,
            selected: bool,
        ) where
            FSet: FnMut(&mut S, i32, bool),
        {
            let user_data = unsafe { (*self_).UserData } as *mut UserData<S, FSet>;
            // SAFETY: this is only called from `apply_requests`, where we have a
            // `&mut self`, so we can take a `&mut` to the user data here.
            let UserData {
                selection,
                set_item_selected,
            } = unsafe { &mut *user_data };
            set_item_selected(selection, index, selected);
        }

        // Drops `set_item_selected` and deallocates user data, but returns `selection`.
        unsafe fn drop_adapter<S, FSet>(user_data: *mut c_void) -> S {
            unsafe { Box::from_raw(user_data as *mut UserData<S, FSet>) }.selection
        }

        let inner =
            unsafe { &mut *sys::ImGuiSelectionExternalStorage_ImGuiSelectionExternalStorage() };
        let user_data = UserData {
            selection,
            set_item_selected,
        };
        let user_data = Box::into_raw(Box::new(user_data));
        inner.UserData = user_data as *mut c_void;
        inner.AdapterSetItemSelected = Some(adapter::<S, FSet>);

        Self {
            inner,
            drop_adapter_set_item_selected: drop_adapter::<S, FSet>,
            adapter_size: Box::new(size),
        }
    }

    /// A reference to the underlying selection storage.
    pub fn inner(&self) -> &S {
        // SAFETY: `selection` has an offset of 0 in `UserData` as checked in `new`.
        unsafe { &*((*self.inner).UserData as *const S) }
    }

    /// A mutable reference to the underlying selection storage.
    pub fn inner_mut(&mut self) -> &mut S {
        // SAFETY: `selection` has an offset of 0 in `UserData` as checked in `new`.
        unsafe { &mut *((*self.inner).UserData as *mut S) }
    }

    /// Consumes the wrapper and returns the underlying selection storage.
    pub fn into_inner(self) -> S {
        // Dropping `self` would also drop the selection we want to return, so we handle
        // the pieces manually instead.
        let mut self_ = ManuallyDrop::new(self);
        let selection = unsafe { (self_.drop_adapter_set_item_selected)((*self_.inner).UserData) };
        unsafe {
            sys::ImGuiSelectionExternalStorage_destroy(self_.inner);
            ptr::drop_in_place(&mut self_.adapter_size);
        }
        selection
    }
}

impl<S> Drop for SelectionExternalStorage<S> {
    fn drop(&mut self) {
        unsafe {
            (self.drop_adapter_set_item_selected)((*self.inner).UserData);
            sys::ImGuiSelectionExternalStorage_destroy(self.inner);
        }
    }
}

impl<S> SelectionStorage for SelectionExternalStorage<S> {
    fn apply_requests(&mut self, ms_io: &mut MultiSelectIo) {
        unsafe {
            sys::ImGuiSelectionExternalStorage_ApplyRequests(self.inner, ms_io.raw_mut());
        }
    }

    fn size(&self) -> i32 {
        (self.adapter_size)(self.inner())
    }
}
