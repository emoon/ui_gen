// This file is auto-generated by rute_gen. DO NOT EDIT.
use std::cell::Cell;
use std::rc::Rc;

#[allow(unused_imports)]
use std::marker::PhantomData;

#[allow(unused_imports)]
use std::os::raw::c_void;

#[allow(unused_imports)]
use std::mem::transmute;

#[allow(unused_imports)]
use std::ffi::{CStr, CString};

use rute_ffi_base::*;

#[allow(unused_imports)]
use auto::*;

#[derive(Clone)]
pub struct Size<'a> {
    pub data: Rc<Cell<Option<*const RUBase>>>,
    pub all_funcs: *const RUSizeAllFuncs,
    pub owned: bool,
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> Size<'a> {
    pub fn new() -> Size<'a> {
        let data = Rc::new(Cell::new(None));

        let ffi_data = unsafe {
            ((*rute_ffi_get()).create_size)(
                ::std::ptr::null(),
                transmute(rute_object_delete_callback as usize),
                Rc::into_raw(data.clone()) as *const c_void,
            )
        };

        data.set(Some(ffi_data.qt_data));

        Size {
            data,
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    pub fn new_from_rc(ffi_data: RUSize) -> Size<'a> {
        Size {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    pub fn new_from_owned(ffi_data: RUSize) -> Size<'a> {
        Size {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }
}
pub trait SizeType {
    fn is_null(&self) -> bool {
        let (obj_data, funcs) = self.get_size_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_null)(obj_data);
            ret_val
        }
    }

    fn is_empty(&self) -> bool {
        let (obj_data, funcs) = self.get_size_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_empty)(obj_data);
            ret_val
        }
    }

    fn is_valid(&self) -> bool {
        let (obj_data, funcs) = self.get_size_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_valid)(obj_data);
            ret_val
        }
    }

    fn width(&self) -> i32 {
        let (obj_data, funcs) = self.get_size_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).width)(obj_data);
            ret_val
        }
    }

    fn height(&self) -> i32 {
        let (obj_data, funcs) = self.get_size_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).height)(obj_data);
            ret_val
        }
    }

    fn set_width(&self, w: i32) -> &Self {
        let (obj_data, funcs) = self.get_size_obj_funcs();
        unsafe {
            ((*funcs).set_width)(obj_data, w);
        }
        self
    }

    fn set_height(&self, h: i32) -> &Self {
        let (obj_data, funcs) = self.get_size_obj_funcs();
        unsafe {
            ((*funcs).set_height)(obj_data, h);
        }
        self
    }

    fn scale(&self, w: i32, h: i32, mode: AspectRatioMode) -> &Self {
        let enum_mode_3 = mode as i32;

        let (obj_data, funcs) = self.get_size_obj_funcs();
        unsafe {
            ((*funcs).scale)(obj_data, w, h, enum_mode_3);
        }
        self
    }

    fn scale_by_size<S: SizeType>(&self, s: &S, mode: AspectRatioMode) -> &Self {
        let (obj_s_1, _funcs) = s.get_size_obj_funcs();
        let enum_mode_2 = mode as i32;

        let (obj_data, funcs) = self.get_size_obj_funcs();
        unsafe {
            ((*funcs).scale_by_size)(obj_data, obj_s_1, enum_mode_2);
        }
        self
    }

    fn scaled(&self, w: i32, h: i32, mode: AspectRatioMode) -> Size {
        let enum_mode_3 = mode as i32;

        let (obj_data, funcs) = self.get_size_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).scaled)(obj_data, w, h, enum_mode_3);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Size::new_from_rc(t);
            } else {
                ret_val = Size::new_from_owned(t);
            }
            ret_val
        }
    }

    fn scaled_by_size<S: SizeType>(&self, s: &S, mode: AspectRatioMode) -> Size {
        let (obj_s_1, _funcs) = s.get_size_obj_funcs();
        let enum_mode_2 = mode as i32;

        let (obj_data, funcs) = self.get_size_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).scaled_by_size)(obj_data, obj_s_1, enum_mode_2);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Size::new_from_rc(t);
            } else {
                ret_val = Size::new_from_owned(t);
            }
            ret_val
        }
    }

    fn expanded_to<S: SizeType>(&self, arg0: &S) -> Size {
        let (obj_arg0_1, _funcs) = arg0.get_size_obj_funcs();

        let (obj_data, funcs) = self.get_size_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).expanded_to)(obj_data, obj_arg0_1);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Size::new_from_rc(t);
            } else {
                ret_val = Size::new_from_owned(t);
            }
            ret_val
        }
    }

    fn bounded_to<S: SizeType>(&self, arg0: &S) -> Size {
        let (obj_arg0_1, _funcs) = arg0.get_size_obj_funcs();

        let (obj_data, funcs) = self.get_size_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).bounded_to)(obj_data, obj_arg0_1);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Size::new_from_rc(t);
            } else {
                ret_val = Size::new_from_owned(t);
            }
            ret_val
        }
    }

    fn get_size_obj_funcs(&self) -> (*const RUBase, *const RUSizeFuncs);
}

impl<'a> SizeType for Size<'a> {
    fn get_size_obj_funcs(&self) -> (*const RUBase, *const RUSizeFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).size_funcs) }
    }
}
