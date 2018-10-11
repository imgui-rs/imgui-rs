use libc::c_int;

bitflags!(
    /// Color edit flags
    #[repr(C)]
    pub struct ImGuiColorEditFlags: c_int {
        const NoAlpha          = 1 << 1;
        const NoPicker         = 1 << 2;
        const NoOptions        = 1 << 3;
        const NoSmallPreview   = 1 << 4;
        const NoInputs         = 1 << 5;
        const NoTooltip        = 1 << 6;
        const NoLabel          = 1 << 7;
        const NoSidePreview    = 1 << 8;
        const AlphaBar         = 1 << 9;
        const AlphaPreview     = 1 << 10;
        const AlphaPreviewHalf = 1 << 11;
        const HDR              = 1 << 12;
        const RGB              = 1 << 13;
        const HSV              = 1 << 14;
        const HEX              = 1 << 15;
        const Uint8            = 1 << 16;
        const Float            = 1 << 17;
        const PickerHueBar     = 1 << 18;
        const PickerHueWheel   = 1 << 19;
    }
);

bitflags!(
    /// Window flags
    #[repr(C)]
    pub struct ImGuiWindowFlags: c_int {
        const NoTitleBar                = 1;
        const NoResize                  = 1 << 1;
        const NoMove                    = 1 << 2;
        const NoScrollbar               = 1 << 3;
        const NoScrollWithMouse         = 1 << 4;
        const NoCollapse                = 1 << 5;
        const AlwaysAutoResize          = 1 << 6;
        const NoSavedSettings           = 1 << 8;
        const NoInputs                  = 1 << 9;
        const MenuBar                   = 1 << 10;
        const HorizontalScrollbar       = 1 << 11;
        const NoFocusOnAppearing        = 1 << 12;
        const NoBringToFrontOnFocus     = 1 << 13;
        const AlwaysVerticalScrollbar   = 1 << 14;
        const AlwaysHorizontalScrollbar = 1 << 15;
        const AlwaysUseWindowPadding    = 1 << 16;
        const ResizeFromAnySide         = 1 << 17;
    }
);

bitflags!(
    /// Condition flags
    #[repr(C)]
    pub struct ImGuiCond: c_int {
        const Always       = 1;
        const Once         = 1 << 1;
        const FirstUseEver = 1 << 2;
        const Appearing    = 1 << 3;
    }
);

bitflags!(
    /// Flags for text inputs
    #[repr(C)]
    pub struct ImGuiInputTextFlags: c_int {
        const CharsDecimal        = 1;
        const CharsHexadecimal    = 1 << 1;
        const CharsUppercase      = 1 << 2;
        const CharsNoBlank        = 1 << 3;
        const AutoSelectAll       = 1 << 4;
        const EnterReturnsTrue    = 1 << 5;
        const CallbackCompletion  = 1 << 6;
        const CallbackHistory     = 1 << 7;
        const CallbackAlways      = 1 << 8;
        const CallbackCharFilter  = 1 << 9;
        const AllowTabInput       = 1 << 10;
        const CtrlEnterForNewLine = 1 << 11;
        const NoHorizontalScroll  = 1 << 12;
        const AlwaysInsertMode    = 1 << 13;
        const ReadOnly            = 1 << 14;
        const Password            = 1 << 15;
        const NoUndoRedo          = 1 << 16;
    }
);

bitflags!(
    /// Flags for selectables
    #[repr(C)]
    pub struct ImGuiSelectableFlags: c_int {
        const DontClosePopups  = 1;
        const SpanAllColumns   = 1 << 1;
        const AllowDoubleClick = 1 << 2;
    }
);

bitflags!(
    /// Flags for trees and collapsing headers
    #[repr(C)]
    pub struct ImGuiTreeNodeFlags: c_int {
        const Selected          = 1;
        const Framed            = 1 << 1;
        const AllowItemOverlap  = 1 << 2;
        const NoTreePushOnOpen  = 1 << 3;
        const NoAutoOpenOnLog   = 1 << 4;
        const DefaultOpen       = 1 << 5;
        const OpenOnDoubleClick = 1 << 6;
        const OpenOnArrow       = 1 << 7;
        const Leaf              = 1 << 8;
        const Bullet            = 1 << 9;
        const FramePadding      = 1 << 10;
        const CollapsingHeader  =
            ImGuiTreeNodeFlags::Framed.bits | ImGuiTreeNodeFlags::NoAutoOpenOnLog.bits;
    }
);

bitflags!(
    /// Flags for window focus check
    #[repr(C)]
    pub struct ImGuiFocusedFlags: c_int {
        const ChildWindows = 1 << 0;
        const RootWindow = 1 << 1;
        const RootAndChildWindows =
            ImGuiFocusedFlags::RootWindow.bits | ImGuiFocusedFlags::ChildWindows.bits;
    }
);

bitflags!(
    /// Flags for hover checks
    #[repr(C)]
    pub struct ImGuiHoveredFlags: c_int {
        const ChildWindows                 = 1 << 0;
        const RootWindow                   = 1 << 1;
        const AllowWhenBlockedByPopup      = 1 << 2;
        const AllowWhenBlockedByActiveItem = 1 << 4;
        const AllowWhenOverlapped          = 1 << 5;
        const RectOnly = ImGuiHoveredFlags::AllowWhenBlockedByPopup.bits
            | ImGuiHoveredFlags::AllowWhenBlockedByActiveItem.bits
            | ImGuiHoveredFlags::AllowWhenOverlapped.bits;
        const RootAndChildWindows = ImGuiFocusedFlags::RootWindow.bits
            | ImGuiFocusedFlags::ChildWindows.bits;
    }
);

bitflags!(
    /// Flags for igBeginCombo
    #[repr(C)]
    pub struct ImGuiComboFlags: c_int {
        /// Align the popup toward the left by default
        const PopupAlignLeft = 1 << 0;
        /// Max ~4 items visible.
        /// Tip: If you want your combo popup to be a specific size you can use
        /// igSetNextWindowSizeConstraints() prior to calling igBeginCombo()
        const HeightSmall    = 1 << 1;
        /// Max ~8 items visible (default)
        const HeightRegular  = 1 << 2;
        /// Max ~20 items visible
        const HeightLarge    = 1 << 3;
        /// As many fitting items as possible
        const HeightLargest  = 1 << 4;
        const HeightMask     = ImGuiComboFlags::HeightSmall.bits
            | ImGuiComboFlags::HeightRegular.bits
            | ImGuiComboFlags::HeightLarge.bits
            | ImGuiComboFlags::HeightLargest.bits;
    }
);

bitflags!(
    /// Flags for igBeginDragDropSource(), igAcceptDragDropPayload()
    #[repr(C)]
    pub struct ImGuiDragDropFlags: c_int {
        // BeginDragDropSource() flags
        /// By default, a successful call to igBeginDragDropSource opens a
        /// tooltip so you can display a preview or description of the source
        /// contents. This flag disable this behavior.
        const SourceNoPreviewTooltip   = 1 << 0;
        /// By default, when dragging we clear data so that igIsItemHovered()
        /// will return true, to avoid subsequent user code submitting tooltips.
        /// This flag disable this behavior so you can still call
        /// igIsItemHovered() on the source item.
        const SourceNoDisableHover     = 1 << 1;
        /// Disable the behavior that allows to open tree nodes and collapsing
        /// header by holding over them while dragging a source item.
        const SourceNoHoldToOpenOthers = 1 << 2;
        /// Allow items such as igText(), igImage() that have no unique
        /// identifier to be used as drag source, by manufacturing a temporary
        /// identifier based on their window-relative position. This is
        /// extremely unusual within the dear imgui ecosystem and so we made it
        /// explicit.
        const SourceAllowNullID        = 1 << 3;
        /// External source (from outside of imgui), won't attempt to read
        /// current item/window info. Will always return true. Only one Extern
        /// source can be active simultaneously.
        const SourceExtern             = 1 << 4;
        // AcceptDragDropPayload() flags
        /// igAcceptDragDropPayload() will returns true even before the mouse
        /// button is released. You can then call igIsDelivery() to test if the
        /// payload needs to be delivered.
        const AcceptBeforeDelivery     = 1 << 10;
        /// Do not draw the default highlight rectangle when hovering over target.
        const AcceptNoDrawDefaultRect  = 1 << 11;
        /// For peeking ahead and inspecting the payload before delivery.
        const AcceptPeekOnly           = ImGuiDragDropFlags::AcceptBeforeDelivery.bits
            | ImGuiDragDropFlags::AcceptNoDrawDefaultRect.bits;
    }
);

bitflags!(
    /// Flags for indictating which corner of a rectangle should be rounded
    #[repr(C)]
    pub struct ImDrawCornerFlags: c_int {
        const TopLeft  = 1 << 0;
        const TopRight = 1 << 1;
        const BotLeft  = 1 << 2;
        const BotRight = 1 << 3;
        const Top      = ImDrawCornerFlags::TopLeft.bits
                       | ImDrawCornerFlags::TopRight.bits;
        const Bot      = ImDrawCornerFlags::BotLeft.bits
                       | ImDrawCornerFlags::BotRight.bits;
        const Left     = ImDrawCornerFlags::TopLeft.bits
                       | ImDrawCornerFlags::BotLeft.bits;
        const Right    = ImDrawCornerFlags::TopRight.bits
                       | ImDrawCornerFlags::BotRight.bits;
        const All      = 0xF;
    }
);

bitflags!(
    #[repr(C)]
    pub struct ImDrawListFlags: c_int {
        const AntiAliasedLines = 1 << 0;
        const AntiAliasedFill  = 1 << 1;
    }
);
