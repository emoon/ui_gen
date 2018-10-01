// This file is auto-generated by rute_gen. DO NOT EDIT.
use rute_ffi_base::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUWidgetFuncs {
    pub destroy: extern "C" fn(self_c: *const RUBase),
    pub show: extern "C" fn(self_c: *const RUBase),
    pub set_fixed_height: extern "C" fn(self_c: *const RUBase, width: i32),
    pub set_fixed_width: extern "C" fn(self_c: *const RUBase, width: i32),
    pub resize: extern "C" fn(self_c: *const RUBase, width: i32, height: i32),
    pub update: extern "C" fn(self_c: *const RUBase),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUWidgetAllFuncs {
    pub widget_funcs: *const RUWidgetFuncs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUWidget {
    pub qt_data: *const RUBase,
    pub host_data: *const RUBase,
    pub all_funcs: *const RUWidgetAllFuncs,
}
