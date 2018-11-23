// This file is auto-generated by rute_gen. DO NOT EDIT.
use rute_ffi_base::*;

#[allow(unused_imports)]
use auto::size_ffi::RUSize;
#[allow(unused_imports)]
use std::os::raw::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUScreenFuncs {
    pub name: extern "C" fn(self_c: *const RUBase) -> *const ::std::os::raw::c_char,
    pub manufacturer: extern "C" fn(self_c: *const RUBase) -> *const ::std::os::raw::c_char,
    pub model: extern "C" fn(self_c: *const RUBase) -> *const ::std::os::raw::c_char,
    pub serial_number: extern "C" fn(self_c: *const RUBase) -> *const ::std::os::raw::c_char,
    pub depth: extern "C" fn(self_c: *const RUBase) -> i32,
    pub physical_dots_per_inch_x: extern "C" fn(self_c: *const RUBase) -> f32,
    pub physical_dots_per_inch_y: extern "C" fn(self_c: *const RUBase) -> f32,
    pub physical_dots_per_inch: extern "C" fn(self_c: *const RUBase) -> f32,
    pub logical_dots_per_inch_x: extern "C" fn(self_c: *const RUBase) -> f32,
    pub logical_dots_per_inch_y: extern "C" fn(self_c: *const RUBase) -> f32,
    pub logical_dots_per_inch: extern "C" fn(self_c: *const RUBase) -> f32,
    pub device_pixel_ratio: extern "C" fn(self_c: *const RUBase) -> f32,
    pub available_size: extern "C" fn(self_c: *const RUBase) -> RUSize,
    pub virtual_size: extern "C" fn(self_c: *const RUBase) -> RUSize,
    pub available_virtual_size: extern "C" fn(self_c: *const RUBase) -> RUSize,
    pub primary_orientation: extern "C" fn(self_c: *const RUBase) -> i32,
    pub angle_between: extern "C" fn(self_c: *const RUBase, a: i32, b: i32) -> i32,
    pub is_landscape: extern "C" fn(self_c: *const RUBase, orientation: i32) -> bool,
    pub refresh_rate: extern "C" fn(self_c: *const RUBase) -> f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUScreenAllFuncs {
    pub screen_funcs: *const RUScreenFuncs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUScreen {
    pub qt_data: *const RUBase,
    pub host_data: *const RUBase,
    pub all_funcs: *const RUScreenAllFuncs,
}
