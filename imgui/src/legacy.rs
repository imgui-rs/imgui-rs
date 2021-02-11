#![allow(non_upper_case_globals)]
use bitflags::bitflags;
use std::os::raw::c_int;

use crate::widget::tree::TreeNodeFlags;

bitflags!(
    /// Flags for text inputs
    #[repr(C)]
    pub struct ImGuiInputTextFlags: c_int {
        /// Allow 0123456789.+-*/
        const CharsDecimal = 1;
        /// Allow 0123456789ABCDEFabcdef
        const CharsHexadecimal = 1 << 1;
        /// Turn a..z into A..Z
        const CharsUppercase = 1 << 2;
        /// Filter out spaces, tabs
        const CharsNoBlank = 1 << 3;
        /// Select entire text when first taking mouse focus
        const AutoSelectAll = 1 << 4;
        /// Return 'true' when Enter is pressed (as opposed to when the value was modified)
        const EnterReturnsTrue = 1 << 5;
        /// Call user function on pressing TAB (for completion handling)
        const CallbackCompletion = 1 << 6;
        /// Call user function on pressing Up/Down arrows (for history handling)
        const CallbackHistory = 1 << 7;
        /// Call user function every time. User code may query cursor position, modify text buffer.
        const CallbackAlways = 1 << 8;
        /// Call user function to filter character.
        const CallbackCharFilter = 1 << 9;
        /// Pressing TAB input a '\t' character into the text field
        const AllowTabInput = 1 << 10;
        /// In multi-line mode, unfocus with Enter, add new line with Ctrl+Enter (default is
        /// opposite: unfocus with Ctrl+Enter, add line with Enter).
        const CtrlEnterForNewLine = 1 << 11;
        /// Disable following the cursor horizontally
        const NoHorizontalScroll = 1 << 12;
        /// Insert mode
        const AlwaysInsertMode = 1 << 13;
        /// Read-only mode
        const ReadOnly = 1 << 14;
        /// Password mode, display all characters as '*'
        const Password = 1 << 15;
        /// Disable undo/redo.
        const NoUndoRedo = 1 << 16;
        /// Allow 0123456789.+-*/eE (Scientific notation input)
        const CharsScientific = 1 << 17;
        /// Allow buffer capacity resize + notify when the string wants to be resized
        const CallbackResize = 1 << 18;
    }
);

pub type ImGuiTreeNodeFlags = TreeNodeFlags;
