// This file is auto-generated by rute_gen. DO NOT EDIT.
use rute_ffi_base::*;

#[allow(unused_imports)]
use std::os::raw::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUSurfaceFormatFuncs {
    pub destroy: extern "C" fn(self_c: *const RUBase),
    pub set_depth_buffer_size: extern "C" fn(self_c: *const RUBase, size: i32),
    pub depth_buffer_size: extern "C" fn(self_c: *const RUBase) -> i32,
    pub set_stencil_buffer_size: extern "C" fn(self_c: *const RUBase, size: i32),
    pub stencil_buffer_size: extern "C" fn(self_c: *const RUBase) -> i32,
    pub set_red_buffer_size: extern "C" fn(self_c: *const RUBase, size: i32),
    pub red_buffer_size: extern "C" fn(self_c: *const RUBase) -> i32,
    pub set_green_buffer_size: extern "C" fn(self_c: *const RUBase, size: i32),
    pub green_buffer_size: extern "C" fn(self_c: *const RUBase) -> i32,
    pub set_blue_buffer_size: extern "C" fn(self_c: *const RUBase, size: i32),
    pub blue_buffer_size: extern "C" fn(self_c: *const RUBase) -> i32,
    pub set_alpha_buffer_size: extern "C" fn(self_c: *const RUBase, size: i32),
    pub alpha_buffer_size: extern "C" fn(self_c: *const RUBase) -> i32,
    pub set_samples: extern "C" fn(self_c: *const RUBase, num_samples: i32),
    pub samples: extern "C" fn(self_c: *const RUBase) -> i32,
    pub set_swap_behavior: extern "C" fn(self_c: *const RUBase, behavior: u32),
    pub swap_behavior: extern "C" fn(self_c: *const RUBase) -> u32,
    pub has_alpha: extern "C" fn(self_c: *const RUBase) -> bool,
    pub set_profile: extern "C" fn(self_c: *const RUBase, profile: u32),
    pub profile: extern "C" fn(self_c: *const RUBase) -> u32,
    pub set_renderable_type: extern "C" fn(self_c: *const RUBase, stype: u32),
    pub renderable_type: extern "C" fn(self_c: *const RUBase) -> u32,
    pub set_major_version: extern "C" fn(self_c: *const RUBase, major_version: i32),
    pub major_version: extern "C" fn(self_c: *const RUBase) -> i32,
    pub set_minor_version: extern "C" fn(self_c: *const RUBase, minor_version: i32),
    pub minor_version: extern "C" fn(self_c: *const RUBase) -> i32,
    pub set_version: extern "C" fn(self_c: *const RUBase, major: i32, minor: i32),
    pub stereo: extern "C" fn(self_c: *const RUBase) -> bool,
    pub set_stereo: extern "C" fn(self_c: *const RUBase, enable: bool),
    pub set_option: extern "C" fn(self_c: *const RUBase, opt: u32),
    pub test_option: extern "C" fn(self_c: *const RUBase, opt: u32) -> bool,
    pub set_options: extern "C" fn(self_c: *const RUBase, options: u32),
    pub set_option_2: extern "C" fn(self_c: *const RUBase, option: u32, on: bool),
    pub test_option_2: extern "C" fn(self_c: *const RUBase, option: u32) -> bool,
    pub options: extern "C" fn(self_c: *const RUBase) -> u32,
    pub swap_interval: extern "C" fn(self_c: *const RUBase) -> i32,
    pub set_swap_interval: extern "C" fn(self_c: *const RUBase, interval: i32),
    pub color_space: extern "C" fn(self_c: *const RUBase) -> u32,
    pub set_color_space: extern "C" fn(self_c: *const RUBase, color_space: u32),
    pub set_default_format: extern "C" fn(self_c: *const RUBase, format: *const RUBase),
    pub default_format: extern "C" fn(self_c: *const RUBase) -> RUSurfaceFormat,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUSurfaceFormatAllFuncs {
    pub surface_format_funcs: *const RUSurfaceFormatFuncs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUSurfaceFormat {
    pub qt_data: *const RUBase,
    pub host_data: *const RUBase,
    pub all_funcs: *const RUSurfaceFormatAllFuncs,
}
