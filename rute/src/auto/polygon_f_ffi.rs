// This file is auto-generated by rute_gen. DO NOT EDIT.
use rute_ffi_base::*;

#[allow(unused_imports)]
use auto::polygon_ffi::RUPolygon;
#[allow(unused_imports)]
use auto::rect_f_ffi::RURectF;
#[allow(unused_imports)]
use std::os::raw::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUPolygonFFuncs {
    pub destroy: extern "C" fn(self_c: *const RUBase),
    pub swap: extern "C" fn(self_c: *const RUBase, other: *const RUBase),
    pub to_polygon: extern "C" fn(self_c: *const RUBase) -> RUPolygon,
    pub is_closed: extern "C" fn(self_c: *const RUBase) -> bool,
    pub bounding_rect: extern "C" fn(self_c: *const RUBase) -> RURectF,
    pub contains_point:
        extern "C" fn(self_c: *const RUBase, pt: *const RUBase, fill_rule: u32) -> bool,
    pub united: extern "C" fn(self_c: *const RUBase, r: *const RUBase) -> RUPolygonF,
    pub intersected: extern "C" fn(self_c: *const RUBase, r: *const RUBase) -> RUPolygonF,
    pub intersects: extern "C" fn(self_c: *const RUBase, r: *const RUBase) -> bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUPolygonFAllFuncs {
    pub polygon_f_funcs: *const RUPolygonFFuncs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUPolygonF {
    pub qt_data: *const RUBase,
    pub host_data: *const RUBase,
    pub all_funcs: *const RUPolygonFAllFuncs,
}
