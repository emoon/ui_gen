// This file is auto-generated by rute_gen. DO NOT EDIT.
use rute_ffi_base::*;

#[allow(unused_imports)]
use auto::layout_item_ffi::RULayoutItem;
#[allow(unused_imports)]
use auto::layout_item_ffi::*;
#[allow(unused_imports)]
use auto::margins_ffi::RUMargins;
#[allow(unused_imports)]
use auto::object_ffi::*;
#[allow(unused_imports)]
use auto::rect_ffi::RURect;
#[allow(unused_imports)]
use auto::size_ffi::RUSize;
#[allow(unused_imports)]
use auto::widget_ffi::RUWidget;
#[allow(unused_imports)]
use std::os::raw::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RULayoutFuncs {
    pub margin: extern "C" fn(self_c: *const RUBase) -> i32,
    pub spacing: extern "C" fn(self_c: *const RUBase) -> i32,
    pub set_margin: extern "C" fn(self_c: *const RUBase, arg0: i32),
    pub set_spacing: extern "C" fn(self_c: *const RUBase, arg0: i32),
    pub set_contents_margins:
        extern "C" fn(self_c: *const RUBase, left: i32, top: i32, right: i32, bottom: i32),
    pub set_contents_margins_2: extern "C" fn(self_c: *const RUBase, margins: *const RUBase),
    pub contents_margins: extern "C" fn(self_c: *const RUBase) -> RUMargins,
    pub contents_rect: extern "C" fn(self_c: *const RUBase) -> RURect,
    pub set_alignment:
        extern "C" fn(self_c: *const RUBase, w: *const RUBase, alignment: u32) -> bool,
    pub set_alignment_2:
        extern "C" fn(self_c: *const RUBase, l: *const RUBase, alignment: u32) -> bool,
    pub set_menu_bar: extern "C" fn(self_c: *const RUBase, w: *const RUBase),
    pub menu_bar: extern "C" fn(self_c: *const RUBase) -> RUWidget,
    pub parent_widget: extern "C" fn(self_c: *const RUBase) -> RUWidget,
    pub invalidate: extern "C" fn(self_c: *const RUBase),
    pub activate: extern "C" fn(self_c: *const RUBase) -> bool,
    pub update: extern "C" fn(self_c: *const RUBase),
    pub add_widget: extern "C" fn(self_c: *const RUBase, w: *const RUBase),
    pub add_item: extern "C" fn(self_c: *const RUBase, arg0: *const RUBase),
    pub remove_widget: extern "C" fn(self_c: *const RUBase, w: *const RUBase),
    pub remove_item: extern "C" fn(self_c: *const RUBase, arg0: *const RUBase),
    pub expanding_directions: extern "C" fn(self_c: *const RUBase) -> u32,
    pub minimum_size: extern "C" fn(self_c: *const RUBase) -> RUSize,
    pub maximum_size: extern "C" fn(self_c: *const RUBase) -> RUSize,
    pub item_at: extern "C" fn(self_c: *const RUBase, index: i32) -> RULayoutItem,
    pub take_at: extern "C" fn(self_c: *const RUBase, index: i32) -> RULayoutItem,
    pub index_of: extern "C" fn(self_c: *const RUBase, arg0: *const RUBase) -> i32,
    pub count: extern "C" fn(self_c: *const RUBase) -> i32,
    pub is_empty: extern "C" fn(self_c: *const RUBase) -> bool,
    pub replace_widget: extern "C" fn(
        self_c: *const RUBase,
        from: *const RUBase,
        to: *const RUBase,
        options: u32,
    ) -> RULayoutItem,
    pub total_height_for_width: extern "C" fn(self_c: *const RUBase, w: i32) -> i32,
    pub total_minimum_size: extern "C" fn(self_c: *const RUBase) -> RUSize,
    pub total_maximum_size: extern "C" fn(self_c: *const RUBase) -> RUSize,
    pub total_size_hint: extern "C" fn(self_c: *const RUBase) -> RUSize,
    pub layout: extern "C" fn(self_c: *const RUBase) -> RULayout,
    pub set_size_constraint: extern "C" fn(self_c: *const RUBase, constraint: u32),
    pub size_constraint: extern "C" fn(self_c: *const RUBase) -> u32,
    pub set_enabled: extern "C" fn(self_c: *const RUBase, arg0: bool),
    pub is_enabled: extern "C" fn(self_c: *const RUBase) -> bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RULayoutAllFuncs {
    pub object_funcs: *const RUObjectFuncs,
    pub layout_item_funcs: *const RULayoutItemFuncs,
    pub layout_funcs: *const RULayoutFuncs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RULayout {
    pub qt_data: *const RUBase,
    pub host_data: *const RUBase,
    pub all_funcs: *const RULayoutAllFuncs,
}
