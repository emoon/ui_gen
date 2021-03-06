// This file is auto-generated by rute_gen. DO NOT EDIT.
use rute_ffi_base::*;

#[allow(unused_imports)]
use auto::layout_ffi::*;
#[allow(unused_imports)]
use auto::layout_item_ffi::RULayoutItem;
#[allow(unused_imports)]
use auto::layout_item_ffi::*;
#[allow(unused_imports)]
use auto::object_ffi::*;
#[allow(unused_imports)]
use auto::rect_ffi::RURect;
#[allow(unused_imports)]
use std::os::raw::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUGridLayoutFuncs {
    pub destroy: extern "C" fn(self_c: *const RUBase),
    pub set_horizontal_spacing: extern "C" fn(self_c: *const RUBase, spacing: i32),
    pub horizontal_spacing: extern "C" fn(self_c: *const RUBase) -> i32,
    pub set_vertical_spacing: extern "C" fn(self_c: *const RUBase, spacing: i32),
    pub vertical_spacing: extern "C" fn(self_c: *const RUBase) -> i32,
    pub set_spacing: extern "C" fn(self_c: *const RUBase, spacing: i32),
    pub spacing: extern "C" fn(self_c: *const RUBase) -> i32,
    pub set_row_minimum_height: extern "C" fn(self_c: *const RUBase, row: i32, min_size: i32),
    pub set_column_minimum_width: extern "C" fn(self_c: *const RUBase, column: i32, min_size: i32),
    pub row_minimum_height: extern "C" fn(self_c: *const RUBase, row: i32) -> i32,
    pub column_minimum_width: extern "C" fn(self_c: *const RUBase, column: i32) -> i32,
    pub column_count: extern "C" fn(self_c: *const RUBase) -> i32,
    pub row_count: extern "C" fn(self_c: *const RUBase) -> i32,
    pub cell_rect: extern "C" fn(self_c: *const RUBase, row: i32, column: i32) -> RURect,
    pub add_widget: extern "C" fn(self_c: *const RUBase, w: *const RUBase),
    pub add_widget_row_column:
        extern "C" fn(self_c: *const RUBase, arg0: *const RUBase, row: i32, column: i32, arg1: u32),
    pub add_widget_row_column_span: extern "C" fn(
        self_c: *const RUBase,
        arg0: *const RUBase,
        row: i32,
        column: i32,
        row_span: i32,
        column_span: i32,
        arg1: u32,
    ),
    pub add_layout:
        extern "C" fn(self_c: *const RUBase, arg0: *const RUBase, row: i32, column: i32, arg1: u32),
    pub add_layout_2: extern "C" fn(
        self_c: *const RUBase,
        arg0: *const RUBase,
        row: i32,
        column: i32,
        row_span: i32,
        column_span: i32,
        arg1: u32,
    ),
    pub set_origin_corner: extern "C" fn(self_c: *const RUBase, arg0: u32),
    pub origin_corner: extern "C" fn(self_c: *const RUBase) -> u32,
    pub item_at_position:
        extern "C" fn(self_c: *const RUBase, row: i32, column: i32) -> RULayoutItem,
    pub count: extern "C" fn(self_c: *const RUBase) -> i32,
    pub set_default_positioning: extern "C" fn(self_c: *const RUBase, n: i32, orient: u32),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUGridLayoutAllFuncs {
    pub object_funcs: *const RUObjectFuncs,
    pub layout_item_funcs: *const RULayoutItemFuncs,
    pub layout_funcs: *const RULayoutFuncs,
    pub grid_layout_funcs: *const RUGridLayoutFuncs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUGridLayout {
    pub qt_data: *const RUBase,
    pub host_data: *const RUBase,
    pub all_funcs: *const RUGridLayoutAllFuncs,
}
