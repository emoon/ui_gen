// This file is auto-generated by rute_gen. DO NOT EDIT.
use rute_ffi_base::*;

#[allow(unused_imports)]
use auto::event_ffi::*;
#[allow(unused_imports)]
use auto::mime_data_ffi::RUMimeData;
#[allow(unused_imports)]
use auto::object_ffi::RUObject;
#[allow(unused_imports)]
use auto::point_f_ffi::RUPointF;
#[allow(unused_imports)]
use auto::point_ffi::RUPoint;
#[allow(unused_imports)]
use std::os::raw::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUDropEventFuncs {
    pub pos: extern "C" fn(self_c: *const RUBase) -> RUPoint,
    pub pos_f: extern "C" fn(self_c: *const RUBase) -> RUPointF,
    pub mouse_buttons: extern "C" fn(self_c: *const RUBase) -> i32,
    pub keyboard_modifiers: extern "C" fn(self_c: *const RUBase) -> i32,
    pub possible_actions: extern "C" fn(self_c: *const RUBase) -> i32,
    pub proposed_action: extern "C" fn(self_c: *const RUBase) -> i32,
    pub accept_proposed_action: extern "C" fn(self_c: *const RUBase),
    pub drop_action: extern "C" fn(self_c: *const RUBase) -> i32,
    pub set_drop_action: extern "C" fn(self_c: *const RUBase, action: i32),
    pub source: extern "C" fn(self_c: *const RUBase) -> RUObject,
    pub mime_data: extern "C" fn(self_c: *const RUBase) -> RUMimeData,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUDropEventAllFuncs {
    pub event_funcs: *const RUEventFuncs,
    pub drop_event_funcs: *const RUDropEventFuncs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUDropEvent {
    pub qt_data: *const RUBase,
    pub host_data: *const RUBase,
    pub all_funcs: *const RUDropEventAllFuncs,
}
