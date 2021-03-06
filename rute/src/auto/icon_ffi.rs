// This file is auto-generated by rute_gen. DO NOT EDIT.
use rute_ffi_base::*;

#[allow(unused_imports)]
use auto::pixmap_ffi::RUPixmap;
#[allow(unused_imports)]
use auto::size_ffi::RUSize;
#[allow(unused_imports)]
use std::os::raw::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUIconFuncs {
    pub destroy: extern "C" fn(self_c: *const RUBase),
    pub swap: extern "C" fn(self_c: *const RUBase, other: *const RUBase),
    pub pixmap: extern "C" fn(
        self_c: *const RUBase,
        size: *const RUBase,
        mode: u32,
        state: u32,
    ) -> RUPixmap,
    pub pixmap_2:
        extern "C" fn(self_c: *const RUBase, w: i32, h: i32, mode: u32, state: u32) -> RUPixmap,
    pub pixmap_3:
        extern "C" fn(self_c: *const RUBase, extent: i32, mode: u32, state: u32) -> RUPixmap,
    pub pixmap_4: extern "C" fn(
        self_c: *const RUBase,
        window: *const RUBase,
        size: *const RUBase,
        mode: u32,
        state: u32,
    ) -> RUPixmap,
    pub actual_size:
        extern "C" fn(self_c: *const RUBase, size: *const RUBase, mode: u32, state: u32) -> RUSize,
    pub actual_size_2: extern "C" fn(
        self_c: *const RUBase,
        window: *const RUBase,
        size: *const RUBase,
        mode: u32,
        state: u32,
    ) -> RUSize,
    pub name: extern "C" fn(self_c: *const RUBase) -> *const ::std::os::raw::c_char,
    pub paint: extern "C" fn(
        self_c: *const RUBase,
        painter: *const RUBase,
        rect: *const RUBase,
        alignment: u32,
        mode: u32,
        state: u32,
    ),
    pub paint_2: extern "C" fn(
        self_c: *const RUBase,
        painter: *const RUBase,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        alignment: u32,
        mode: u32,
        state: u32,
    ),
    pub is_null: extern "C" fn(self_c: *const RUBase) -> bool,
    pub is_detached: extern "C" fn(self_c: *const RUBase) -> bool,
    pub detach: extern "C" fn(self_c: *const RUBase),
    pub cache_key: extern "C" fn(self_c: *const RUBase) -> i64,
    pub add_pixmap:
        extern "C" fn(self_c: *const RUBase, pixmap: *const RUBase, mode: u32, state: u32),
    pub add_file: extern "C" fn(
        self_c: *const RUBase,
        file_name: *const ::std::os::raw::c_char,
        size: *const RUBase,
        mode: u32,
        state: u32,
    ),
    pub set_is_mask: extern "C" fn(self_c: *const RUBase, is_mask: bool),
    pub is_mask: extern "C" fn(self_c: *const RUBase) -> bool,
    pub from_theme:
        extern "C" fn(self_c: *const RUBase, name: *const ::std::os::raw::c_char) -> RUIcon,
    pub from_theme_2: extern "C" fn(
        self_c: *const RUBase,
        name: *const ::std::os::raw::c_char,
        fallback: *const RUBase,
    ) -> RUIcon,
    pub has_theme_icon:
        extern "C" fn(self_c: *const RUBase, name: *const ::std::os::raw::c_char) -> bool,
    pub theme_name: extern "C" fn(self_c: *const RUBase) -> *const ::std::os::raw::c_char,
    pub set_theme_name: extern "C" fn(self_c: *const RUBase, path: *const ::std::os::raw::c_char),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUIconAllFuncs {
    pub icon_funcs: *const RUIconFuncs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUIcon {
    pub qt_data: *const RUBase,
    pub host_data: *const RUBase,
    pub all_funcs: *const RUIconAllFuncs,
}
