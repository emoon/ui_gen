// This file is auto-generated by rute_gen. DO NOT EDIT.
use rute_ffi_base::*;

#[allow(unused_imports)]
use auto::color_ffi::RUColor;
#[allow(unused_imports)]
use auto::image_ffi::RUImage;
#[allow(unused_imports)]
use auto::pixmap_ffi::RUPixmap;
#[allow(unused_imports)]
use std::os::raw::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUBrushFuncs {
    pub destroy: extern "C" fn(self_c: *const RUBase),
    pub swap: extern "C" fn(self_c: *const RUBase, other: *const RUBase),
    pub style: extern "C" fn(self_c: *const RUBase) -> u32,
    pub set_style: extern "C" fn(self_c: *const RUBase, arg0: u32),
    pub set_transform: extern "C" fn(self_c: *const RUBase, arg0: *const RUBase),
    pub texture: extern "C" fn(self_c: *const RUBase) -> RUPixmap,
    pub set_texture: extern "C" fn(self_c: *const RUBase, pixmap: *const RUBase),
    pub texture_image: extern "C" fn(self_c: *const RUBase) -> RUImage,
    pub set_texture_image: extern "C" fn(self_c: *const RUBase, image: *const RUBase),
    pub color: extern "C" fn(self_c: *const RUBase) -> RUColor,
    pub set_color: extern "C" fn(self_c: *const RUBase, color: *const RUBase),
    pub set_color_2: extern "C" fn(self_c: *const RUBase, color: u32),
    pub is_opaque: extern "C" fn(self_c: *const RUBase) -> bool,
    pub is_detached: extern "C" fn(self_c: *const RUBase) -> bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUBrushAllFuncs {
    pub brush_funcs: *const RUBrushFuncs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUBrush {
    pub qt_data: *const RUBase,
    pub host_data: *const RUBase,
    pub all_funcs: *const RUBrushAllFuncs,
}
