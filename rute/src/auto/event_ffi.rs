// This file is auto-generated by rute_gen. DO NOT EDIT.
use rute_ffi_base::*;

#[allow(unused_imports)]
use std::os::raw::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUEventFuncs {
    pub spontaneous: extern "C" fn(self_c: *const RUBase) -> bool,
    pub set_accepted: extern "C" fn(self_c: *const RUBase, accepted: bool),
    pub is_accepted: extern "C" fn(self_c: *const RUBase) -> bool,
    pub accept: extern "C" fn(self_c: *const RUBase),
    pub ignore: extern "C" fn(self_c: *const RUBase),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUEventAllFuncs {
    pub event_funcs: *const RUEventFuncs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUEvent {
    pub qt_data: *const RUBase,
    pub host_data: *const RUBase,
    pub all_funcs: *const RUEventAllFuncs,
}
