// This file is auto-generated by rute_gen. DO NOT EDIT.
use rute_ffi_base::*;

#[allow(unused_imports)]
use auto::event_ffi::*;
#[allow(unused_imports)]
use auto::input_event_ffi::*;
#[allow(unused_imports)]
use std::os::raw::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUKeyEventFuncs {
    pub key: extern "C" fn(self_c: *const RUBase) -> i32,
    pub matches: extern "C" fn(self_c: *const RUBase, key: u32) -> bool,
    pub modifiers: extern "C" fn(self_c: *const RUBase) -> u32,
    pub text: extern "C" fn(self_c: *const RUBase) -> *const ::std::os::raw::c_char,
    pub is_auto_repeat: extern "C" fn(self_c: *const RUBase) -> bool,
    pub count: extern "C" fn(self_c: *const RUBase) -> i32,
    pub native_scan_code: extern "C" fn(self_c: *const RUBase) -> u32,
    pub native_virtual_key: extern "C" fn(self_c: *const RUBase) -> u32,
    pub native_modifiers: extern "C" fn(self_c: *const RUBase) -> u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUKeyEventAllFuncs {
    pub event_funcs: *const RUEventFuncs,
    pub input_event_funcs: *const RUInputEventFuncs,
    pub key_event_funcs: *const RUKeyEventFuncs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUKeyEvent {
    pub qt_data: *const RUBase,
    pub host_data: *const RUBase,
    pub all_funcs: *const RUKeyEventAllFuncs,
}
