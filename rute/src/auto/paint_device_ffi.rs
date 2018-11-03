// This file is auto-generated by rute_gen. DO NOT EDIT.
use rute_ffi_base::*;

use auto::paint_engine_ffi::RUPaintEngine;
use auto::painter_ffi::RUPainter;
#[allow(unused_imports)]
use std::os::raw::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUPaintDeviceFuncs {
    pub destroy: extern "C" fn(self_c: *const RUBase),
    pub set_dev_type: extern "C" fn(
        object: *const RUBase,
        user_data: *const c_void,
        trampoline_func: *const c_void,
        callback: *const c_void,
    ),

    pub remove_dev_type: extern "C" fn(object: *const RUBase),

    pub painting_active: extern "C" fn(self_c: *const RUBase) -> bool,
    pub set_paint_engine: extern "C" fn(
        object: *const RUBase,
        user_data: *const c_void,
        trampoline_func: *const c_void,
        callback: *const c_void,
    ),

    pub remove_paint_engine: extern "C" fn(object: *const RUBase),

    pub width: extern "C" fn(self_c: *const RUBase) -> i32,
    pub height: extern "C" fn(self_c: *const RUBase) -> i32,
    pub width_mm: extern "C" fn(self_c: *const RUBase) -> i32,
    pub height_mm: extern "C" fn(self_c: *const RUBase) -> i32,
    pub logical_dpi_x: extern "C" fn(self_c: *const RUBase) -> i32,
    pub logical_dpi_y: extern "C" fn(self_c: *const RUBase) -> i32,
    pub physical_dpi_x: extern "C" fn(self_c: *const RUBase) -> i32,
    pub physical_dpi_y: extern "C" fn(self_c: *const RUBase) -> i32,
    pub device_pixel_ratio: extern "C" fn(self_c: *const RUBase) -> i32,
    pub device_pixel_ratio_f: extern "C" fn(self_c: *const RUBase) -> f32,
    pub color_count: extern "C" fn(self_c: *const RUBase) -> i32,
    pub depth: extern "C" fn(self_c: *const RUBase) -> i32,
    pub device_pixel_ratio_f_scale: extern "C" fn(self_c: *const RUBase) -> f32,
    pub set_init_painter: extern "C" fn(
        object: *const RUBase,
        user_data: *const c_void,
        trampoline_func: *const c_void,
        callback: *const c_void,
    ),

    pub remove_init_painter: extern "C" fn(object: *const RUBase),

    pub set_redirected: extern "C" fn(
        object: *const RUBase,
        user_data: *const c_void,
        trampoline_func: *const c_void,
        callback: *const c_void,
    ),

    pub remove_redirected: extern "C" fn(object: *const RUBase),

    pub set_shared_painter: extern "C" fn(
        object: *const RUBase,
        user_data: *const c_void,
        trampoline_func: *const c_void,
        callback: *const c_void,
    ),

    pub remove_shared_painter: extern "C" fn(object: *const RUBase),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUPaintDeviceAllFuncs {
    pub paint_device_funcs: *const RUPaintDeviceFuncs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUPaintDevice {
    pub qt_data: *const RUBase,
    pub host_data: *const RUBase,
    pub all_funcs: *const RUPaintDeviceAllFuncs,
}
