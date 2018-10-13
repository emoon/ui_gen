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
pub struct PaintEvent<'a> {
    pub data: Rc<Cell<Option<*const RUBase>>>,
    pub all_funcs: *const RUPaintEventAllFuncs,
    pub owned: bool,
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> PaintEvent<'a> {
    pub fn new_from_rc(ffi_data: RUPaintEvent) -> PaintEvent<'a> {
        PaintEvent {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    pub fn new_from_owned(ffi_data: RUPaintEvent) -> PaintEvent<'a> {
        PaintEvent {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    pub fn new_from_temporary(ffi_data: RUPaintEvent) -> PaintEvent<'a> {
        PaintEvent {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }
}
pub trait PaintEventType<'a> {
    fn rect(&self) -> Option<Rect> {
        let (obj_data, funcs) = self.get_paint_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).rect)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Rect::new_from_rc(t);
            } else {
                ret_val = Rect::new_from_owned(t);
            }
            Some(ret_val)
        }
    }

    #[inline]
    fn get_paint_event_obj_funcs(&self) -> (*const RUBase, *const RUPaintEventFuncs);
}

impl<'a> EventType<'a> for PaintEvent<'a> {
    #[inline]
    fn get_event_obj_funcs(&self) -> (*const RUBase, *const RUEventFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).event_funcs) }
    }
}

impl<'a> PaintEventType<'a> for PaintEvent<'a> {
    #[inline]
    fn get_paint_event_obj_funcs(&self) -> (*const RUBase, *const RUPaintEventFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).paint_event_funcs) }
    }
}
