// This file is auto-generated by rute_gen. DO NOT EDIT.
use rute_ffi_base::*;

#[allow(unused_imports)]
use std::os::raw::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUColorFuncs {
    pub destroy: extern "C" fn(self_c: *const RUBase),
    pub is_valid: extern "C" fn(self_c: *const RUBase) -> bool,
    pub name: extern "C" fn(self_c: *const RUBase) -> *const ::std::os::raw::c_char,
    pub name_2: extern "C" fn(self_c: *const RUBase, format: i32) -> *const ::std::os::raw::c_char,
    pub set_named_color: extern "C" fn(self_c: *const RUBase, name: *const ::std::os::raw::c_char),
    pub spec: extern "C" fn(self_c: *const RUBase) -> i32,
    pub alpha: extern "C" fn(self_c: *const RUBase) -> i32,
    pub set_alpha: extern "C" fn(self_c: *const RUBase, alpha: i32),
    pub alpha_f: extern "C" fn(self_c: *const RUBase) -> f32,
    pub set_alpha_f: extern "C" fn(self_c: *const RUBase, alpha: f32),
    pub red: extern "C" fn(self_c: *const RUBase) -> i32,
    pub green: extern "C" fn(self_c: *const RUBase) -> i32,
    pub blue: extern "C" fn(self_c: *const RUBase) -> i32,
    pub set_red: extern "C" fn(self_c: *const RUBase, red: i32),
    pub set_green: extern "C" fn(self_c: *const RUBase, green: i32),
    pub set_blue: extern "C" fn(self_c: *const RUBase, blue: i32),
    pub red_f: extern "C" fn(self_c: *const RUBase) -> f32,
    pub green_f: extern "C" fn(self_c: *const RUBase) -> f32,
    pub blue_f: extern "C" fn(self_c: *const RUBase) -> f32,
    pub set_red_f: extern "C" fn(self_c: *const RUBase, red: f32),
    pub set_green_f: extern "C" fn(self_c: *const RUBase, green: f32),
    pub set_blue_f: extern "C" fn(self_c: *const RUBase, blue: f32),
    pub set_rgb: extern "C" fn(self_c: *const RUBase, r: i32, g: i32, b: i32, a: i32),
    pub set_rgb_f: extern "C" fn(self_c: *const RUBase, r: f32, g: f32, b: f32, a: f32),
    pub hue: extern "C" fn(self_c: *const RUBase) -> i32,
    pub saturation: extern "C" fn(self_c: *const RUBase) -> i32,
    pub hsv_hue: extern "C" fn(self_c: *const RUBase) -> i32,
    pub hsv_saturation: extern "C" fn(self_c: *const RUBase) -> i32,
    pub value: extern "C" fn(self_c: *const RUBase) -> i32,
    pub hue_f: extern "C" fn(self_c: *const RUBase) -> f32,
    pub saturation_f: extern "C" fn(self_c: *const RUBase) -> f32,
    pub hsv_hue_f: extern "C" fn(self_c: *const RUBase) -> f32,
    pub hsv_saturation_f: extern "C" fn(self_c: *const RUBase) -> f32,
    pub value_f: extern "C" fn(self_c: *const RUBase) -> f32,
    pub set_hsv: extern "C" fn(self_c: *const RUBase, h: i32, s: i32, v: i32, a: i32),
    pub set_hsv_f: extern "C" fn(self_c: *const RUBase, h: f32, s: f32, v: f32, a: f32),
    pub cyan: extern "C" fn(self_c: *const RUBase) -> i32,
    pub magenta: extern "C" fn(self_c: *const RUBase) -> i32,
    pub yellow: extern "C" fn(self_c: *const RUBase) -> i32,
    pub black: extern "C" fn(self_c: *const RUBase) -> i32,
    pub cyan_f: extern "C" fn(self_c: *const RUBase) -> f32,
    pub magenta_f: extern "C" fn(self_c: *const RUBase) -> f32,
    pub yellow_f: extern "C" fn(self_c: *const RUBase) -> f32,
    pub black_f: extern "C" fn(self_c: *const RUBase) -> f32,
    pub set_cmyk: extern "C" fn(self_c: *const RUBase, c: i32, m: i32, y: i32, k: i32, a: i32),
    pub set_cmyk_f: extern "C" fn(self_c: *const RUBase, c: f32, m: f32, y: f32, k: f32, a: f32),
    pub hsl_hue: extern "C" fn(self_c: *const RUBase) -> i32,
    pub hsl_saturation: extern "C" fn(self_c: *const RUBase) -> i32,
    pub lightness: extern "C" fn(self_c: *const RUBase) -> i32,
    pub hsl_hue_f: extern "C" fn(self_c: *const RUBase) -> f32,
    pub hsl_saturation_f: extern "C" fn(self_c: *const RUBase) -> f32,
    pub lightness_f: extern "C" fn(self_c: *const RUBase) -> f32,
    pub set_hsl: extern "C" fn(self_c: *const RUBase, h: i32, s: i32, l: i32, a: i32),
    pub set_hsl_f: extern "C" fn(self_c: *const RUBase, h: f32, s: f32, l: f32, a: f32),
    pub to_rgb: extern "C" fn(self_c: *const RUBase) -> RUColor,
    pub to_hsv: extern "C" fn(self_c: *const RUBase) -> RUColor,
    pub to_cmyk: extern "C" fn(self_c: *const RUBase) -> RUColor,
    pub to_hsl: extern "C" fn(self_c: *const RUBase) -> RUColor,
    pub convert_to: extern "C" fn(self_c: *const RUBase, color_spec: i32) -> RUColor,
    pub from_rgb_2: extern "C" fn(self_c: *const RUBase, r: i32, g: i32, b: i32, a: i32) -> RUColor,
    pub from_rgb_f: extern "C" fn(self_c: *const RUBase, r: f32, g: f32, b: f32, a: f32) -> RUColor,
    pub from_rgba64:
        extern "C" fn(self_c: *const RUBase, r: u16, g: u16, b: u16, a: u16) -> RUColor,
    pub from_hsv: extern "C" fn(self_c: *const RUBase, h: i32, s: i32, v: i32, a: i32) -> RUColor,
    pub from_hsv_f: extern "C" fn(self_c: *const RUBase, h: f32, s: f32, v: f32, a: f32) -> RUColor,
    pub from_cmyk:
        extern "C" fn(self_c: *const RUBase, c: i32, m: i32, y: i32, k: i32, a: i32) -> RUColor,
    pub from_cmyk_f:
        extern "C" fn(self_c: *const RUBase, c: f32, m: f32, y: f32, k: f32, a: f32) -> RUColor,
    pub from_hsl: extern "C" fn(self_c: *const RUBase, h: i32, s: i32, l: i32, a: i32) -> RUColor,
    pub from_hsl_f: extern "C" fn(self_c: *const RUBase, h: f32, s: f32, l: f32, a: f32) -> RUColor,
    pub light: extern "C" fn(self_c: *const RUBase, f: i32) -> RUColor,
    pub lighter: extern "C" fn(self_c: *const RUBase, f: i32) -> RUColor,
    pub dark: extern "C" fn(self_c: *const RUBase, f: i32) -> RUColor,
    pub darker: extern "C" fn(self_c: *const RUBase, f: i32) -> RUColor,
    pub is_valid_color:
        extern "C" fn(self_c: *const RUBase, name: *const ::std::os::raw::c_char) -> bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUColorAllFuncs {
    pub color_funcs: *const RUColorFuncs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUColor {
    pub qt_data: *const RUBase,
    pub host_data: *const RUBase,
    pub all_funcs: *const RUColorAllFuncs,
}
