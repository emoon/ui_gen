// This file is auto-generated by rute_gen. DO NOT EDIT.
use rute_ffi_base::*;

#[allow(unused_imports)]
use auto::layout_ffi::*;
#[allow(unused_imports)]
use auto::layout_item_ffi::*;
#[allow(unused_imports)]
use auto::object_ffi::*;
#[allow(unused_imports)]
use std::os::raw::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUBoxLayoutFuncs {
    pub direction: extern "C" fn(self_c: *const RUBase) -> u32,
    pub set_direction: extern "C" fn(self_c: *const RUBase, arg0: u32),
    pub add_spacing: extern "C" fn(self_c: *const RUBase, size: i32),
    pub add_spacer_item: extern "C" fn(self_c: *const RUBase, spacer_item: *const RUBase),
    pub add_widget:
        extern "C" fn(self_c: *const RUBase, arg0: *const RUBase, stretch: i32, alignment: u32),
    pub add_layout: extern "C" fn(self_c: *const RUBase, layout: *const RUBase, stretch: i32),
    pub insert_spacing: extern "C" fn(self_c: *const RUBase, index: i32, size: i32),
    pub insert_widget: extern "C" fn(
        self_c: *const RUBase,
        index: i32,
        widget: *const RUBase,
        stretch: i32,
        alignment: u32,
    ),
    pub insert_layout:
        extern "C" fn(self_c: *const RUBase, index: i32, layout: *const RUBase, stretch: i32),
    pub insert_item: extern "C" fn(self_c: *const RUBase, index: i32, arg0: *const RUBase),
    pub spacing: extern "C" fn(self_c: *const RUBase) -> i32,
    pub set_spacing: extern "C" fn(self_c: *const RUBase, spacing: i32),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUBoxLayoutAllFuncs {
    pub object_funcs: *const RUObjectFuncs,
    pub layout_item_funcs: *const RULayoutItemFuncs,
    pub layout_funcs: *const RULayoutFuncs,
    pub box_layout_funcs: *const RUBoxLayoutFuncs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUBoxLayout {
    pub qt_data: *const RUBase,
    pub host_data: *const RUBase,
    pub all_funcs: *const RUBoxLayoutAllFuncs,
}
