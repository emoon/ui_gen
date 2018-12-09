// This file is auto-generated by rute_gen. DO NOT EDIT.
use rute_ffi_base::*;

#[allow(unused_imports)]
use auto::button_group_ffi::RUButtonGroup;
#[allow(unused_imports)]
use auto::icon_ffi::RUIcon;
#[allow(unused_imports)]
use auto::key_sequence_ffi::RUKeySequence;
#[allow(unused_imports)]
use auto::object_ffi::*;
#[allow(unused_imports)]
use auto::paint_device_ffi::*;
#[allow(unused_imports)]
use auto::size_ffi::RUSize;
#[allow(unused_imports)]
use auto::widget_ffi::*;
#[allow(unused_imports)]
use std::os::raw::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUAbstractButtonFuncs {
    pub set_text: extern "C" fn(self_c: *const RUBase, text: *const ::std::os::raw::c_char),
    pub text: extern "C" fn(self_c: *const RUBase) -> *const ::std::os::raw::c_char,
    pub set_icon: extern "C" fn(self_c: *const RUBase, icon: *const RUBase),
    pub icon: extern "C" fn(self_c: *const RUBase) -> RUIcon,
    pub icon_size: extern "C" fn(self_c: *const RUBase) -> RUSize,
    pub set_shortcut: extern "C" fn(self_c: *const RUBase, key: *const RUBase),
    pub shortcut: extern "C" fn(self_c: *const RUBase) -> RUKeySequence,
    pub set_checkable: extern "C" fn(self_c: *const RUBase, arg0: bool),
    pub is_checkable: extern "C" fn(self_c: *const RUBase) -> bool,
    pub is_checked: extern "C" fn(self_c: *const RUBase) -> bool,
    pub set_down: extern "C" fn(self_c: *const RUBase, arg0: bool),
    pub is_down: extern "C" fn(self_c: *const RUBase) -> bool,
    pub set_auto_repeat: extern "C" fn(self_c: *const RUBase, arg0: bool),
    pub auto_repeat: extern "C" fn(self_c: *const RUBase) -> bool,
    pub set_auto_repeat_delay: extern "C" fn(self_c: *const RUBase, arg0: i32),
    pub auto_repeat_delay: extern "C" fn(self_c: *const RUBase) -> i32,
    pub set_auto_repeat_interval: extern "C" fn(self_c: *const RUBase, arg0: i32),
    pub auto_repeat_interval: extern "C" fn(self_c: *const RUBase) -> i32,
    pub set_auto_exclusive: extern "C" fn(self_c: *const RUBase, arg0: bool),
    pub auto_exclusive: extern "C" fn(self_c: *const RUBase) -> bool,
    pub group: extern "C" fn(self_c: *const RUBase) -> RUButtonGroup,
    pub set_icon_size: extern "C" fn(self_c: *const RUBase, size: *const RUBase),
    pub animate_click: extern "C" fn(self_c: *const RUBase, msec: i32),
    pub click: extern "C" fn(self_c: *const RUBase),
    pub toggle: extern "C" fn(self_c: *const RUBase),
    pub set_checked: extern "C" fn(self_c: *const RUBase, arg0: bool),
    pub set_pressed_event: extern "C" fn(
        object: *const RUBase,
        user_data: *const c_void,
        trampoline_func: *const c_void,
        callback: *const c_void,
    ),

    pub set_released_event: extern "C" fn(
        object: *const RUBase,
        user_data: *const c_void,
        trampoline_func: *const c_void,
        callback: *const c_void,
    ),

    pub set_clicked_event: extern "C" fn(
        object: *const RUBase,
        user_data: *const c_void,
        trampoline_func: *const c_void,
        callback: *const c_void,
    ),

    pub set_toggled_event: extern "C" fn(
        object: *const RUBase,
        user_data: *const c_void,
        trampoline_func: *const c_void,
        callback: *const c_void,
    ),

    pub set_paint_event: extern "C" fn(
        object: *const RUBase,
        user_data: *const c_void,
        trampoline_func: *const c_void,
        callback: *const c_void,
    ),

    pub remove_paint_event: extern "C" fn(object: *const RUBase),

    pub set_key_press_event: extern "C" fn(
        object: *const RUBase,
        user_data: *const c_void,
        trampoline_func: *const c_void,
        callback: *const c_void,
    ),

    pub remove_key_press_event: extern "C" fn(object: *const RUBase),

    pub set_key_release_event: extern "C" fn(
        object: *const RUBase,
        user_data: *const c_void,
        trampoline_func: *const c_void,
        callback: *const c_void,
    ),

    pub remove_key_release_event: extern "C" fn(object: *const RUBase),

    pub set_mouse_press_event: extern "C" fn(
        object: *const RUBase,
        user_data: *const c_void,
        trampoline_func: *const c_void,
        callback: *const c_void,
    ),

    pub remove_mouse_press_event: extern "C" fn(object: *const RUBase),

    pub set_mouse_release_event: extern "C" fn(
        object: *const RUBase,
        user_data: *const c_void,
        trampoline_func: *const c_void,
        callback: *const c_void,
    ),

    pub remove_mouse_release_event: extern "C" fn(object: *const RUBase),

    pub set_mouse_move_event: extern "C" fn(
        object: *const RUBase,
        user_data: *const c_void,
        trampoline_func: *const c_void,
        callback: *const c_void,
    ),

    pub remove_mouse_move_event: extern "C" fn(object: *const RUBase),

    pub set_focus_in_event: extern "C" fn(
        object: *const RUBase,
        user_data: *const c_void,
        trampoline_func: *const c_void,
        callback: *const c_void,
    ),

    pub remove_focus_in_event: extern "C" fn(object: *const RUBase),

    pub set_focus_out_event: extern "C" fn(
        object: *const RUBase,
        user_data: *const c_void,
        trampoline_func: *const c_void,
        callback: *const c_void,
    ),

    pub remove_focus_out_event: extern "C" fn(object: *const RUBase),

    pub set_change_event: extern "C" fn(
        object: *const RUBase,
        user_data: *const c_void,
        trampoline_func: *const c_void,
        callback: *const c_void,
    ),

    pub remove_change_event: extern "C" fn(object: *const RUBase),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUAbstractButtonAllFuncs {
    pub object_funcs: *const RUObjectFuncs,
    pub paint_device_funcs: *const RUPaintDeviceFuncs,
    pub widget_funcs: *const RUWidgetFuncs,
    pub abstract_button_funcs: *const RUAbstractButtonFuncs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUAbstractButton {
    pub qt_data: *const RUBase,
    pub host_data: *const RUBase,
    pub all_funcs: *const RUAbstractButtonAllFuncs,
}