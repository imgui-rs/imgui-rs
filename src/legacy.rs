#![allow(non_upper_case_globals)]
use bitflags::bitflags;
use std::os::raw::c_int;

bitflags!(
    /// Flags for igBeginDragDropSource(), igAcceptDragDropPayload()
    #[repr(C)]
    pub struct ImGuiDragDropFlags: c_int {
        /// By default, a successful call to igBeginDragDropSource opens a tooltip so you can
        /// display a preview or description of the source contents. This flag disable this
        /// behavior.
        const SourceNoPreviewTooltip = 1;
        /// By default, when dragging we clear data so that igIsItemHovered() will return false, to
        /// avoid subsequent user code submitting tooltips. This flag disable this behavior so you
        /// can still call igIsItemHovered() on the source item.
        const SourceNoDisableHover = 1 << 1;
        /// Disable the behavior that allows to open tree nodes and collapsing header by holding
        /// over them while dragging a source item.
        const SourceNoHoldToOpenOthers = 1 << 2;
        /// Allow items such as igText(), igImage() that have no unique identifier to be used as
        /// drag source, by manufacturing a temporary identifier based on their window-relative
        /// position. This is extremely unusual within the dear imgui ecosystem and so we made it
        /// explicit.
        const SourceAllowNullID = 1 << 3;
        /// External source (from outside of imgui), won't attempt to read current item/window
        /// info. Will always return true. Only one Extern source can be active simultaneously.
        const SourceExtern = 1 << 4;
        /// Automatically expire the payload if the source cease to be submitted (otherwise
        /// payloads are persisting while being dragged)
        const SourceAutoExpirePayload = 1 << 5;
        /// igAcceptDragDropPayload() will returns true even before the mouse button is released.
        /// You can then call igIsDelivery() to test if the payload needs to be delivered.
        const AcceptBeforeDelivery = 1 << 10;
        /// Do not draw the default highlight rectangle when hovering over target.
        const AcceptNoDrawDefaultRect = 1 << 11;
        /// Request hiding the igBeginDragDropSource tooltip from the igBeginDragDropTarget site.
        const AcceptNoPreviewTooltip = 1 << 12;
        /// For peeking ahead and inspecting the payload before delivery.
        const AcceptPeekOnly = ImGuiDragDropFlags::AcceptBeforeDelivery.bits
            | ImGuiDragDropFlags::AcceptNoDrawDefaultRect.bits;
    }
);

bitflags!(
    /// Flags for indictating which corner of a rectangle should be rounded
    #[repr(C)]
    pub struct ImDrawCornerFlags: c_int {
        const TopLeft = 1;
        const TopRight = 1 << 1;
        const BotLeft = 1 << 2;
        const BotRight = 1 << 3;
        const Top = ImDrawCornerFlags::TopLeft.bits
            | ImDrawCornerFlags::TopRight.bits;
        const Bot = ImDrawCornerFlags::BotLeft.bits
            | ImDrawCornerFlags::BotRight.bits;
        const Left = ImDrawCornerFlags::TopLeft.bits
            | ImDrawCornerFlags::BotLeft.bits;
        const Right = ImDrawCornerFlags::TopRight.bits
            | ImDrawCornerFlags::BotRight.bits;
        const All = 0xF;
    }
);

bitflags!(
    /// Draw list flags
    #[repr(C)]
    pub struct ImDrawListFlags: c_int {
        const AntiAliasedLines = 1;
        const AntiAliasedFill = 1 << 1;
    }
);

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

bitflags!(
    /// Flags for trees and collapsing headers
    #[repr(C)]
    pub struct ImGuiTreeNodeFlags: c_int {
        /// Draw as selected
        const Selected = 1;
        /// Full colored frame (e.g. for collapsing header)
        const Framed = 1 << 1;
        /// Hit testing to allow subsequent widgets to overlap this one
        const AllowItemOverlap = 1 << 2;
        /// Don't do a tree push when open (e.g. for collapsing header) = no extra indent nor
        /// pushing on ID stack
        const NoTreePushOnOpen = 1 << 3;
        /// Don't automatically and temporarily open node when Logging is active (by default
        /// logging will automatically open tree nodes)
        const NoAutoOpenOnLog = 1 << 4;
        /// Default node to be open
        const DefaultOpen = 1 << 5;
        /// Need double-click to open node
        const OpenOnDoubleClick = 1 << 6;
        /// Only open when clicking on the arrow part. If OpenOnDoubleClick is also set,
        /// single-click arrow or double-click all box to open.
        const OpenOnArrow = 1 << 7;
        /// No collapsing, no arrow (use as a convenience for leaf nodes).
        const Leaf = 1 << 8;
        /// Display a bullet instead of arrow
        const Bullet = 1 << 9;
        /// Use FramePadding (even for an unframed text node) to vertically align text baseline to
        /// regular widget height.
        const FramePadding = 1 << 10;
        const NavLeftJumpsBackHere = 1 << 13;

        const CollapsingHeader  =
            ImGuiTreeNodeFlags::Framed.bits | ImGuiTreeNodeFlags::NoTreePushOnOpen.bits |
            ImGuiTreeNodeFlags::NoAutoOpenOnLog.bits;
    }
);
