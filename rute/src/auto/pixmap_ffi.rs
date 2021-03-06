// This file is auto-generated by rute_gen. DO NOT EDIT.
use rute_ffi_base::*;

#[allow(unused_imports)]
use auto::bitmap_ffi::RUBitmap;
#[allow(unused_imports)]
use auto::image_ffi::RUImage;
#[allow(unused_imports)]
use auto::paint_device_ffi::*;
#[allow(unused_imports)]
use auto::paint_engine_ffi::RUPaintEngine;
#[allow(unused_imports)]
use auto::rect_ffi::RURect;
#[allow(unused_imports)]
use auto::size_ffi::RUSize;
#[allow(unused_imports)]
use std::os::raw::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUPixmapFuncs {
    pub destroy: extern "C" fn(self_c: *const RUBase),
    pub swap: extern "C" fn(self_c: *const RUBase, other: *const RUBase),
    pub is_null: extern "C" fn(self_c: *const RUBase) -> bool,
    pub width: extern "C" fn(self_c: *const RUBase) -> i32,
    pub height: extern "C" fn(self_c: *const RUBase) -> i32,
    pub size: extern "C" fn(self_c: *const RUBase) -> RUSize,
    pub rect: extern "C" fn(self_c: *const RUBase) -> RURect,
    pub depth: extern "C" fn(self_c: *const RUBase) -> i32,
    pub default_depth: extern "C" fn(self_c: *const RUBase) -> i32,
    pub fill: extern "C" fn(self_c: *const RUBase, fill_color: *const RUBase),
    pub fill_2: extern "C" fn(self_c: *const RUBase, device: *const RUBase, ofs: *const RUBase),
    pub fill_3: extern "C" fn(self_c: *const RUBase, device: *const RUBase, xofs: i32, yofs: i32),
    pub mask: extern "C" fn(self_c: *const RUBase) -> RUBitmap,
    pub set_mask: extern "C" fn(self_c: *const RUBase, arg0: *const RUBase),
    pub device_pixel_ratio: extern "C" fn(self_c: *const RUBase) -> f32,
    pub set_device_pixel_ratio: extern "C" fn(self_c: *const RUBase, scale_factor: f32),
    pub has_alpha: extern "C" fn(self_c: *const RUBase) -> bool,
    pub has_alpha_channel: extern "C" fn(self_c: *const RUBase) -> bool,
    pub create_heuristic_mask: extern "C" fn(self_c: *const RUBase, clip_tight: bool) -> RUBitmap,
    pub create_mask_from_color:
        extern "C" fn(self_c: *const RUBase, mask_color: *const RUBase, mode: u32) -> RUBitmap,
    pub grab_window:
        extern "C" fn(self_c: *const RUBase, arg0: u64, x: i32, y: i32, w: i32, h: i32) -> RUPixmap,
    pub grab_widget: extern "C" fn(
        self_c: *const RUBase,
        widget: *const RUBase,
        rect: *const RUBase,
    ) -> RUPixmap,
    pub grab_widget_2: extern "C" fn(
        self_c: *const RUBase,
        widget: *const RUBase,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
    ) -> RUPixmap,
    pub scaled: extern "C" fn(
        self_c: *const RUBase,
        w: i32,
        h: i32,
        aspect_mode: u32,
        mode: u32,
    ) -> RUPixmap,
    pub scaled_2: extern "C" fn(
        self_c: *const RUBase,
        s: *const RUBase,
        aspect_mode: u32,
        mode: u32,
    ) -> RUPixmap,
    pub scaled_to_width: extern "C" fn(self_c: *const RUBase, w: i32, mode: u32) -> RUPixmap,
    pub scaled_to_height: extern "C" fn(self_c: *const RUBase, h: i32, mode: u32) -> RUPixmap,
    pub to_image: extern "C" fn(self_c: *const RUBase) -> RUImage,
    pub from_image:
        extern "C" fn(self_c: *const RUBase, image: *const RUBase, flags: u32) -> RUPixmap,
    pub from_image_2:
        extern "C" fn(self_c: *const RUBase, image: *const RUBase, flags: u32) -> RUPixmap,
    pub convert_from_image:
        extern "C" fn(self_c: *const RUBase, img: *const RUBase, flags: u32) -> bool,
    pub copy:
        extern "C" fn(self_c: *const RUBase, x: i32, y: i32, width: i32, height: i32) -> RUPixmap,
    pub copy_2: extern "C" fn(self_c: *const RUBase, rect: *const RUBase) -> RUPixmap,
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
    pub scroll_2: extern "C" fn(
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
    pub paint_engine: extern "C" fn(self_c: *const RUBase) -> RUPaintEngine,
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
