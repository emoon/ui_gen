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

///
/// QMargin defines a set of four margins; left, top, right and bottom,
/// that describe the size of the borders surrounding a rectangle.
///
/// The isNull() function returns `true` only if all margins are set to zero.
///
/// QMargin objects can be streamed as well as compared.
/// # Licence
///
/// The documentation is an adoption of the original [Qt Documentation](http://doc.qt.io/) and provided herein is licensed under the terms of the [GNU Free Documentation License version 1.3](http://www.gnu.org/licenses/fdl.html) as published by the Free Software Foundation.
#[derive(Clone)]
pub struct Margins<'a> {
    pub data: Rc<Cell<Option<*const RUBase>>>,
    pub all_funcs: *const RUMarginsAllFuncs,
    pub owned: bool,
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> Margins<'a> {
    pub fn new() -> Margins<'a> {
        let data = Rc::new(Cell::new(None));

        let ffi_data = unsafe {
            ((*rute_ffi_get()).create_margins)(
                ::std::ptr::null(),
                transmute(rute_object_delete_callback as usize),
                Rc::into_raw(data.clone()) as *const c_void,
            )
        };

        data.set(Some(ffi_data.qt_data));

        Margins {
            data,
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }
    pub fn new_from_rc(ffi_data: RUMargins) -> Margins<'a> {
        Margins {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    pub fn new_from_owned(ffi_data: RUMargins) -> Margins<'a> {
        Margins {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    pub fn new_from_temporary(ffi_data: RUMargins) -> Margins<'a> {
        Margins {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }
}
pub trait MarginsType<'a> {
    ///
    /// Returns `true` if all margins are is 0; otherwise returns
    /// false.
    fn is_null(&self) -> bool {
        let (obj_data, funcs) = self.get_margins_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_null)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the left margin.
    ///
    /// **See also:** setLeft()
    fn left(&self) -> i32 {
        let (obj_data, funcs) = self.get_margins_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).left)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the top margin.
    ///
    /// **See also:** setTop()
    fn top(&self) -> i32 {
        let (obj_data, funcs) = self.get_margins_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).top)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the right margin.
    fn right(&self) -> i32 {
        let (obj_data, funcs) = self.get_margins_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).right)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the bottom margin.
    fn bottom(&self) -> i32 {
        let (obj_data, funcs) = self.get_margins_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).bottom)(obj_data);
            ret_val
        }
    }
    ///
    /// Sets the left margin to *left.*
    fn set_left(&self, left: i32) -> &Self {
        let (obj_data, funcs) = self.get_margins_obj_funcs();
        unsafe {
            ((*funcs).set_left)(obj_data, left);
        }
        self
    }
    ///
    /// Sets the Top margin to *Top.*
    fn set_top(&self, top: i32) -> &Self {
        let (obj_data, funcs) = self.get_margins_obj_funcs();
        unsafe {
            ((*funcs).set_top)(obj_data, top);
        }
        self
    }
    ///
    /// Sets the right margin to *right.*
    fn set_right(&self, right: i32) -> &Self {
        let (obj_data, funcs) = self.get_margins_obj_funcs();
        unsafe {
            ((*funcs).set_right)(obj_data, right);
        }
        self
    }
    ///
    /// Sets the bottom margin to *bottom.*
    fn set_bottom(&self, bottom: i32) -> &Self {
        let (obj_data, funcs) = self.get_margins_obj_funcs();
        unsafe {
            ((*funcs).set_bottom)(obj_data, bottom);
        }
        self
    }

    #[inline]
    fn get_margins_obj_funcs(&self) -> (*const RUBase, *const RUMarginsFuncs);
}

impl<'a> MarginsType<'a> for Margins<'a> {
    #[inline]
    fn get_margins_obj_funcs(&self) -> (*const RUBase, *const RUMarginsFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).margins_funcs) }
    }
}