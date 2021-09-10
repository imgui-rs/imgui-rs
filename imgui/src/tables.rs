use bitflags::bitflags;
use std::convert::From;

use crate::sys;
use crate::{Id, ImColor32, ImStr, Ui};

bitflags! {
    /// Item hover check option flags
    #[repr(transparent)]
    pub struct TableFlags: u32 {
        const NONE = sys::ImGuiTableFlags_None;
        const RESIZABLE = sys::ImGuiTableFlags_Resizable;
        const REORDERABLE =sys::ImGuiTableFlags_Reorderable;
        const HIDEABLE = sys::ImGuiTableFlags_Hideable;
        const SORTABLE = sys::ImGuiTableFlags_Sortable;
        const NO_SAVED_SETTINGS = sys::ImGuiTableFlags_NoSavedSettings;
        const CONTEXT_MENU_IN_BODY = sys::ImGuiTableFlags_ContextMenuInBody;
        // Decorations
        const ROW_BG = sys::ImGuiTableFlags_RowBg;
        const BORDERS_INNER_H = sys::ImGuiTableFlags_BordersInnerH;
        const BORDERS_OUTER_H = sys::ImGuiTableFlags_BordersOuterH;
        const BORDERS_INNER_V = sys::ImGuiTableFlags_BordersInnerV;
        const BORDERS_OUTER_V = sys::ImGuiTableFlags_BordersOuterV;
        const BORDERS_H = sys::ImGuiTableFlags_BordersH;
        const BORDERS_V = sys::ImGuiTableFlags_BordersV;
        const BORDERS_INNER = sys::ImGuiTableFlags_BordersInner;
        const BORDERS_OUTER = sys::ImGuiTableFlags_BordersOuter;
        const BORDERS= sys::ImGuiTableFlags_Borders;
        const NO_BORDERS_IN_BODY = sys::ImGuiTableFlags_NoBordersInBody;
        const NO_BORDERS_IN_BODY_UNTIL_RESIZE = sys::ImGuiTableFlags_NoBordersInBodyUntilResize;
        // Sizing Policy (read above for defaults)
        const SIZING_FIXED_FIT = sys::ImGuiTableFlags_SizingFixedFit;
        const SIZING_FIXED_SAME = sys::ImGuiTableFlags_SizingFixedSame;
        const SIZING_STRETCH_PROP = sys::ImGuiTableFlags_SizingStretchProp;
        const SIZING_STRETCH_SAME = sys::ImGuiTableFlags_SizingStretchSame;
        // Sizing Extra Options
        const NO_HOST_EXTEND_X = sys::ImGuiTableFlags_NoHostExtendX;
        const NO_HOST_EXTEND_Y = sys::ImGuiTableFlags_NoHostExtendY;
        const NO_KEEP_COLUMNS_VISIBLE = sys::ImGuiTableFlags_NoKeepColumnsVisible;
        const PRECISE_WIDTHS = sys::ImGuiTableFlags_PreciseWidths;
        // Clipping
        const NO_CLIP = sys::ImGuiTableFlags_NoClip;
        // Padding
        const PAD_OUTER_X = sys::ImGuiTableFlags_PadOuterX;
        const NO_PAD_OUTER_X = sys::ImGuiTableFlags_NoPadOuterX;
        const NO_PAD_INNER_X = sys::ImGuiTableFlags_NoPadInnerX;
        // Scrolling
        const SCROLL_X = sys::ImGuiTableFlags_ScrollX;
        const SCROLL_Y = sys::ImGuiTableFlags_ScrollY;
        // Sorting
        const SORT_MULTI = sys::ImGuiTableFlags_SortMulti;
        const SORT_TRISTATE = sys::ImGuiTableFlags_SortTristate;
    }
}

bitflags! {
    #[repr(transparent)]
    pub struct TableRowFlags: u32 {
        const NONE = sys::ImGuiTableRowFlags_None;
        const HEADERS = sys::ImGuiTableRowFlags_Headers;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Default)]
    pub struct TableColumnFlags: u32 {
        // Input configuration flags
        const NONE = sys::ImGuiTableColumnFlags_None;
        const DEFAULT_HIDE =sys::ImGuiTableColumnFlags_DefaultHide;
        const DEFAULT_SORT =sys::ImGuiTableColumnFlags_DefaultSort;
        const WIDTH_STRETCH=sys::ImGuiTableColumnFlags_WidthStretch;
        const WIDTH_FIXED = sys::ImGuiTableColumnFlags_WidthFixed;
        const NO_RESIZE = sys::ImGuiTableColumnFlags_NoResize;
        const NO_REORDER = sys::ImGuiTableColumnFlags_NoReorder;
        const NO_HIDE = sys::ImGuiTableColumnFlags_NoHide;
        const NO_CLIP = sys::ImGuiTableColumnFlags_NoClip;
        const NO_SORT = sys::ImGuiTableColumnFlags_NoSort;
        const NO_SORT_ASCENDING = sys::ImGuiTableColumnFlags_NoSortAscending;
        const NO_SORT_DESCENDING = sys::ImGuiTableColumnFlags_NoSortDescending;
        const NO_HEADER_WIDTH = sys::ImGuiTableColumnFlags_NoHeaderWidth;
        const PREFER_SORT_ASCENDING = sys::ImGuiTableColumnFlags_PreferSortAscending;
        const PREFER_SORT_DESCENDING = sys::ImGuiTableColumnFlags_PreferSortDescending;
        const INDENT_ENABLE = sys::ImGuiTableColumnFlags_IndentEnable;
        const INDENT_DISABLE = sys::ImGuiTableColumnFlags_IndentDisable;
        // Output status flags, read-only via TableGetColumnFlags()
        const IS_ENABLED = sys::ImGuiTableColumnFlags_IsEnabled;
        const IS_VISIBLE = sys::ImGuiTableColumnFlags_IsVisible;
        const IS_SORTED = sys::ImGuiTableColumnFlags_IsSorted;
        const IS_HOVERED = sys::ImGuiTableColumnFlags_IsHovered;
    }
}

bitflags! {
    #[repr(transparent)]
    pub struct TableBgTarget: u32 {
        const NONE = sys::ImGuiTableBgTarget_None;
        const ROW_BG0 = sys::ImGuiTableBgTarget_RowBg0;
        const ROW_BG1 = sys::ImGuiTableBgTarget_RowBg1;
        const CELL_BG = sys::ImGuiTableBgTarget_CellBg;
    }
}

impl<'ui> Ui<'ui> {
    pub fn begin_table(&self, str_id: &ImStr, column_count: i32) -> Option<TableToken<'ui>> {
        self.begin_table_with_flags(str_id, column_count, TableFlags::NONE)
    }
    pub fn begin_table_with_flags(
        &self,
        str_id: &ImStr,
        column_count: i32,
        flags: TableFlags,
    ) -> Option<TableToken<'ui>> {
        self.begin_table_with_sizing(str_id, column_count, flags, [0.0, 0.0], 0.0)
    }
    pub fn begin_table_with_sizing(
        &self,
        str_id: &ImStr,
        column: i32,
        flags: TableFlags,
        outer_size: [f32; 2],
        inner_width: f32,
    ) -> Option<TableToken<'ui>> {
        let should_render = unsafe {
            sys::igBeginTable(
                str_id.as_ptr(),
                column,
                flags.bits() as i32,
                outer_size.into(),
                inner_width,
            )
        };

        should_render.then(|| TableToken::new(self))
    }

    pub fn begin_table_header<'a, const N: usize>(
        &self,
        str_id: &ImStr,
        column_data: [TableHeader<'a>; N],
    ) -> Option<TableToken<'ui>> {
        self.begin_table_header_with_flags(str_id, column_data, TableFlags::NONE)
    }
    pub fn begin_table_header_with_flags<'a, const N: usize>(
        &self,
        str_id: &ImStr,
        column_data: [TableHeader<'a>; N],
        flags: TableFlags,
    ) -> Option<TableToken<'ui>> {
        self.begin_table_header_with_sizing(str_id, column_data, flags, [0.0, 0.0], 0.0)
    }
    pub fn begin_table_header_with_sizing<'a, const N: usize>(
        &self,
        str_id: &ImStr,
        column_data: [TableHeader<'a>; N],
        flags: TableFlags,
        outer_size: [f32; 2],
        inner_width: f32,
    ) -> Option<TableToken<'ui>> {
        self.begin_table_with_sizing(str_id, N as i32, flags, outer_size, inner_width)
            .map(|data| {
                for value in column_data {
                    self.table_setup_column_with_id(
                        value.name,
                        value.flags,
                        value.init_width_or_weight,
                        value.user_id,
                    );
                }
                self.table_headers_row();

                data
            })
    }

    #[inline]
    pub fn table_next_row(&self) {
        self.table_next_row_with_flags(TableRowFlags::NONE);
    }

    #[inline]
    pub fn table_next_row_with_flags(&self, flags: TableRowFlags) {
        self.table_next_row_with_height(flags, 0.0);
    }

    #[inline]
    pub fn table_next_row_with_height(&self, flags: TableRowFlags, min_row_height: f32) {
        unsafe {
            sys::igTableNextRow(flags.bits() as i32, min_row_height);
        }
    }

    pub fn table_next_column(&self) -> bool {
        unsafe { sys::igTableNextColumn() }
    }
    pub fn table_set_column_index(&self, column_index: usize) -> bool {
        unsafe { sys::igTableSetColumnIndex(column_index as i32) }
    }

    pub fn table_setup_column(&self, str_id: &ImStr) {
        self.table_setup_column_with_flags(str_id, TableColumnFlags::NONE)
    }

    pub fn table_setup_column_with_flags(&self, str_id: &ImStr, flags: TableColumnFlags) {
        self.table_setup_column_with_id(str_id, flags, 0.0, Id::Int(0))
    }
    pub fn table_setup_column_with_id(
        &self,
        str_id: &ImStr,
        flags: TableColumnFlags,
        init_width_or_weight: f32,
        user_id: Id,
    ) {
        unsafe {
            sys::igTableSetupColumn(
                str_id.as_ptr(),
                flags.bits() as i32,
                init_width_or_weight,
                user_id.as_imgui_id(),
            )
        }
    }

    pub fn table_setup_scroll_freeze(&self, locked_columns: i32, locked_rows: i32) {
        unsafe {
            sys::igTableSetupScrollFreeze(locked_columns, locked_rows);
        }
    }
    pub fn table_headers_row(&self) {
        unsafe {
            sys::igTableHeadersRow();
        }
    }
    pub fn table_header(&self, label: &ImStr) {
        unsafe {
            sys::igTableHeader(label.as_ptr());
        }
    }

    //pub fn table_get_sort_specs(&self) -> &TableSortSpecs {
    //    unsafe { sys::igTableGetSortSpecs() }
    //}

    pub fn table_get_column_count(&self) -> i32 {
        unsafe { sys::igTableGetColumnCount() }
    }
    pub fn table_get_column_index(&self) -> i32 {
        unsafe { sys::igTableGetColumnIndex() }
    }
    pub fn table_get_row_index(&self) -> i32 {
        unsafe { sys::igTableGetRowIndex() }
    }

    pub fn table_get_column_flags(&self) -> TableColumnFlags {
        self.table_get_column_flags_with_column(-1)
    }
    pub fn table_get_column_flags_with_column(&self, column_n: i32) -> TableColumnFlags {
        unsafe {
            TableColumnFlags::from_bits_unchecked(sys::igTableGetColumnFlags(column_n) as u32)
        }
    }

    pub fn table_set_bg_color(&self, target: TableBgTarget, color: [f32; 4]) {
        self.table_set_bg_color_with_column(target, color, -1);
    }
    pub fn table_set_bg_color_with_column(
        &self,
        target: TableBgTarget,
        color: [f32; 4],
        column_n: i32,
    ) {
        let color = ImColor32::from(color);
        unsafe {
            sys::igTableSetBgColor(target.bits() as i32, color.to_bits(), column_n);
        }
    }
}

pub struct TableHeader<'a> {
    pub name: &'a ImStr,
    pub flags: TableColumnFlags,
    pub init_width_or_weight: f32,
    pub user_id: Id<'a>,
}

impl<'a> TableHeader<'a> {
    pub fn new(name: &'a ImStr) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }
}

impl<'a> Default for TableHeader<'a> {
    fn default() -> Self {
        Self {
            name: Default::default(),
            flags: Default::default(),
            init_width_or_weight: Default::default(),
            user_id: Id::Int(0),
        }
    }
}

create_token!(
    /// Tracks a table which can be rendered onto, ending with `.end()`
    /// or by dropping.
    pub struct TableToken<'ui>;

    /// Pops a change from the font stack.
    drop { sys::igEndTable() }
);
