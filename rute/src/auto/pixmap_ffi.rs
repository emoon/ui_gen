// This file is auto-generated by rute_gen. DO NOT EDIT.
use rute_ffi_base::*;

use auto::bitmap_ffi::RUBitmap;
use auto::image_ffi::RUImage;
use auto::paint_device_ffi::*;
use auto::paint_engine_ffi::RUPaintEngine;
use auto::rect_ffi::RURect;
use auto::size_ffi::RUSize;
#[allow(unused_imports)]
use std::os::raw::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUPixmapFuncs {
    pub destroy: extern "C" fn(self_c: *const RUBase),
    pub swap: extern "C" fn(self_c: *const RUBase, other: *const RUBase),
    pub is_null: extern "C" fn(self_c: *const RUBase) -> bool,
    pub set_dev_type: extern "C" fn(
        object: *const RUBase,
        user_data: *const c_void,
        trampoline_func: *const c_void,
        callback: *const c_void,
    ),

    pub remove_dev_type: extern "C" fn(object: *const RUBase),

    pub width: extern "C" fn(self_c: *const RUBase) -> i32,
    pub height: extern "C" fn(self_c: *const RUBase) -> i32,
    pub size: extern "C" fn(self_c: *const RUBase) -> RUSize,
    pub rect: extern "C" fn(self_c: *const RUBase) -> RURect,
    pub depth: extern "C" fn(self_c: *const RUBase) -> i32,
    pub default_depth: extern "C" fn(self_c: *const RUBase) -> i32,
    pub fill_by_color_type: extern "C" fn(self_c: *const RUBase, fill_color: *const RUBase),
    pub fill_by_device:
        extern "C" fn(self_c: *const RUBase, device: *const RUBase, ofs: *const RUBase),
    pub fill_by_device_offset:
        extern "C" fn(self_c: *const RUBase, device: *const RUBase, xofs: i32, yofs: i32),
    pub mask: extern "C" fn(self_c: *const RUBase) -> RUBitmap,
    pub set_mask: extern "C" fn(self_c: *const RUBase, arg0: *const RUBase),
    pub device_pixel_ratio: extern "C" fn(self_c: *const RUBase) -> f32,
    pub set_device_pixel_ratio: extern "C" fn(self_c: *const RUBase, scale_factor: f32),
    pub has_alpha: extern "C" fn(self_c: *const RUBase) -> bool,
    pub has_alpha_channel: extern "C" fn(self_c: *const RUBase) -> bool,
    pub create_heuristic_mask: extern "C" fn(self_c: *const RUBase, clip_tight: bool) -> RUBitmap,
    pub create_mask_from_color:
        extern "C" fn(self_c: *const RUBase, mask_color: *const RUBase, mode: i32) -> RUBitmap,
    pub grab_window:
        extern "C" fn(self_c: *const RUBase, arg0: RUWId, x: i32, y: i32, w: i32, h: i32)
            -> RUPixmap,
    pub grab_widget_by_rect:
        extern "C" fn(self_c: *const RUBase, widget: *const RUBase, rect: *const RUBase)
            -> RUPixmap,
    pub grab_widget:
        extern "C" fn(self_c: *const RUBase, widget: *const RUBase, x: i32, y: i32, w: i32, h: i32)
            -> RUPixmap,
    pub scaled: extern "C" fn(self_c: *const RUBase, w: i32, h: i32, aspect_mode: i32, mode: i32)
        -> RUPixmap,
    pub scaled: extern "C" fn(self_c: *const RUBase, s: *const RUBase, aspect_mode: i32, mode: i32)
        -> RUPixmap,
    pub scaled_to_width: extern "C" fn(self_c: *const RUBase, w: i32, mode: i32) -> RUPixmap,
    pub scaled_to_height: extern "C" fn(self_c: *const RUBase, h: i32, mode: i32) -> RUPixmap,
    pub to_image: extern "C" fn(self_c: *const RUBase) -> RUImage,
    pub from_image:
        extern "C" fn(self_c: *const RUBase, image: *const RUBase, flags: i32) -> RUPixmap,
    pub from_image_reader:
        extern "C" fn(self_c: *const RUBase, image_reader: *const RUBase, flags: i32) -> RUPixmap,
    pub from_image:
        extern "C" fn(self_c: *const RUBase, image: *const RUBase, flags: i32) -> RUPixmap,
    pub load: extern "C" fn(
        self_c: *const RUBase,
        file_name: *const ::std::os::raw::c_char,
        format: *const RUBase,
        flags: i32,
    ) -> bool,
    pub load_from_data: extern "C" fn(
        self_c: *const RUBase,
        buf: *const RUBase,
        len: RUuint,
        format: *const RUBase,
        flags: i32,
    ) -> bool,
    pub load_from_data: extern "C" fn(
        self_c: *const RUBase,
        data: *const RUBase,
        format: *const RUBase,
        flags: i32,
    ) -> bool,
    pub save: extern "C" fn(
        self_c: *const RUBase,
        file_name: *const ::std::os::raw::c_char,
        format: *const RUBase,
        quality: i32,
    ) -> bool,
    pub save: extern "C" fn(
        self_c: *const RUBase,
        device: *const RUBase,
        format: *const RUBase,
        quality: i32,
    ) -> bool,
    pub convert_from_image:
        extern "C" fn(self_c: *const RUBase, img: *const RUBase, flags: i32) -> bool,
    pub copy:
        extern "C" fn(self_c: *const RUBase, x: i32, y: i32, width: i32, height: i32) -> RUPixmap,
    pub copy: extern "C" fn(self_c: *const RUBase, rect: *const RUBase) -> RUPixmap,
    pub scroll: extern "C" fn(
        self_c: *const RUBase,
        dx: i32,
        dy: i32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        exposed: *const RUBase,
    ),
    pub scroll: extern "C" fn(
        self_c: *const RUBase,
        dx: i32,
        dy: i32,
        rect: *const RUBase,
        exposed: *const RUBase,
    ),
    pub cache_key: extern "C" fn(self_c: *const RUBase) -> i64,
    pub is_detached: extern "C" fn(self_c: *const RUBase) -> bool,
    pub detach: extern "C" fn(self_c: *const RUBase),
    pub is_q_bitmap: extern "C" fn(self_c: *const RUBase) -> bool,
    pub set_paint_engine: extern "C" fn(
        object: *const RUBase,
        user_data: *const c_void,
        trampoline_func: *const c_void,
        callback: *const c_void,
    ),

    pub remove_paint_engine: extern "C" fn(object: *const RUBase),

    pub from_image_in_place:
        extern "C" fn(self_c: *const RUBase, image: *const RUBase, flags: i32) -> RUPixmap,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUPixmapAllFuncs {
    pub paint_device_funcs: *const RUPaintDeviceFuncs,
    pub pixmap_funcs: *const RUPixmapFuncs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUPixmap {
    pub qt_data: *const RUBase,
    pub host_data: *const RUBase,
    pub all_funcs: *const RUPixmapAllFuncs,
}
