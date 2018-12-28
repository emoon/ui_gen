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

/// **Notice these docs are heavy WIP and not very relevent yet**
///
/// Expose events are sent to windows when an area of the window is invalidated
/// or window exposure in the windowing system changes.
///
/// A Window with a client area that is completely covered by another window, or
/// is otherwise not visible may be considered obscured by Qt and may in such
/// cases not receive expose events.
///
/// The event handler QWindow::exposeEvent() receives expose events.
/// # Licence
///
/// The documentation is an adoption of the original [Qt Documentation](http://doc.qt.io/) and provided herein is licensed under the terms of the [GNU Free Documentation License version 1.3](http://www.gnu.org/licenses/fdl.html) as published by the Free Software Foundation.
#[derive(Clone)]
pub struct ExposeEvent<'a> {
    #[doc(hidden)]
    pub data: Rc<Cell<Option<*const RUBase>>>,
    #[doc(hidden)]
    pub all_funcs: *const RUExposeEventAllFuncs,
    #[doc(hidden)]
    pub owned: bool,
    #[doc(hidden)]
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> ExposeEvent<'a> {
    #[allow(dead_code)]
    pub(crate) fn new_from_rc(ffi_data: RUExposeEvent) -> ExposeEvent<'a> {
        ExposeEvent {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_owned(ffi_data: RUExposeEvent) -> ExposeEvent<'a> {
        ExposeEvent {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_temporary(ffi_data: RUExposeEvent) -> ExposeEvent<'a> {
        ExposeEvent {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }
    ///
    /// Returns the window area that has been exposed. The region is given in local coordinates.
    pub fn region(&self) -> Option<Region> {
        let (obj_data, funcs) = self.get_expose_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).region)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Region::new_from_rc(t);
            } else {
                ret_val = Region::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    #[doc(hidden)]
    pub fn spontaneous(&self) -> bool {
        let (obj_data, funcs) = self.get_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).spontaneous)(obj_data);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn set_accepted(&self, accepted: bool) -> &Self {
        let (obj_data, funcs) = self.get_event_obj_funcs();
        unsafe {
            ((*funcs).set_accepted)(obj_data, accepted);
        }
        self
    }
    #[doc(hidden)]
    pub fn is_accepted(&self) -> bool {
        let (obj_data, funcs) = self.get_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_accepted)(obj_data);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn accept(&self) -> &Self {
        let (obj_data, funcs) = self.get_event_obj_funcs();
        unsafe {
            ((*funcs).accept)(obj_data);
        }
        self
    }
    #[doc(hidden)]
    pub fn ignore(&self) -> &Self {
        let (obj_data, funcs) = self.get_event_obj_funcs();
        unsafe {
            ((*funcs).ignore)(obj_data);
        }
        self
    }

    pub fn build(&self) -> Self {
        self.clone()
    }
}

impl<'a> From<(WrapperRcOwn, bool)> for ExposeEvent<'a> {
    fn from(t: (WrapperRcOwn, bool)) -> Self {
        if t.1 {
            ExposeEvent::new_from_rc(t.0 as *const RUExposeEvent)
        } else {
            ExposeEvent::new_from_temporary(t.0 as *const RUExposeEvent)
        }
    }
}

pub trait ExposeEventTrait<'a> {
    #[inline]
    #[doc(hidden)]
    fn get_expose_event_obj_funcs(&self) -> (*const RUBase, *const RUExposeEventFuncs);
}

impl<'a> EventTrait<'a> for ExposeEvent<'a> {
    #[doc(hidden)]
    fn get_event_obj_funcs(&self) -> (*const RUBase, *const RUEventFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).event_funcs) }
    }
}

impl<'a> ExposeEventTrait<'a> for ExposeEvent<'a> {
    #[doc(hidden)]
    fn get_expose_event_obj_funcs(&self) -> (*const RUBase, *const RUExposeEventFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).expose_event_funcs) }
    }
}
