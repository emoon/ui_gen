// This file is auto-generated by rute_gen. DO NOT EDIT.
use std::cell::Cell;
use std::marker::PhantomData;
use std::rc::Rc;

#[allow(unused_imports)]
use std::ffi::{CStr, CString};

use rute_ffi_base::*;

#[allow(unused_imports)]
use auto::*;

#[derive(Clone)]
pub struct Size<'a> {
    pub data: Rc<Cell<Option<*const RUBase>>>,
    pub all_funcs: *const RUSizeAllFuncs,
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> Size<'a> {
    pub fn new_from_rc(ffi_data: RUSize) -> Size<'a> {
        Size {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            _marker: PhantomData,
        }
    }

    pub fn new_from_owned(ffi_data: RUSize) -> Size<'a> {
        Size {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
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

    fn get_size_obj_funcs(&self) -> (*const RUBase, *const RUSizeFuncs);
}

impl<'a> SizeType for Size<'a> {
    fn get_size_obj_funcs(&self) -> (*const RUBase, *const RUSizeFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).size_funcs) }
    }
}