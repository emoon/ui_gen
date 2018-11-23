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

// Auto-generated imports

#[allow(unused_imports)]
use auto::drag_leave_event_ffi::*;
#[allow(unused_imports)]
use auto::event::*;
#[allow(unused_imports)]
use auto::event_ffi::*;
#[allow(unused_imports)]
use auto::rute::*;
#[allow(unused_imports)]
use auto::rute_ffi::*;
///
/// This event is always preceded by a QDragEnterEvent and a series
/// of [QDragMoveEvent](QDragMoveEvent)
/// s. It is not sent if a QDropEvent is sent
/// instead.
///
/// **See also:** [`DragEnterEvent`]
/// [`DragMoveEvent`]
/// [`DropEvent`]
/// # Licence
///
/// The documentation is an adoption of the original [Qt Documentation](http://doc.qt.io/) and provided herein is licensed under the terms of the [GNU Free Documentation License version 1.3](http://www.gnu.org/licenses/fdl.html) as published by the Free Software Foundation.
#[derive(Clone)]
pub struct DragLeaveEvent<'a> {
    pub data: Rc<Cell<Option<*const RUBase>>>,
    pub all_funcs: *const RUDragLeaveEventAllFuncs,
    pub owned: bool,
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> DragLeaveEvent<'a> {
    pub fn new_from_rc(ffi_data: RUDragLeaveEvent) -> DragLeaveEvent<'a> {
        DragLeaveEvent {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    pub fn new_from_owned(ffi_data: RUDragLeaveEvent) -> DragLeaveEvent<'a> {
        DragLeaveEvent {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    pub fn new_from_temporary(ffi_data: RUDragLeaveEvent) -> DragLeaveEvent<'a> {
        DragLeaveEvent {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }
}
pub trait DragLeaveEventTrait<'a> {
    #[inline]
    fn get_drag_leave_event_obj_funcs(&self) -> (*const RUBase, *const RUDragLeaveEventFuncs);
}

impl<'a> EventTrait<'a> for DragLeaveEvent<'a> {
    #[inline]
    fn get_event_obj_funcs(&self) -> (*const RUBase, *const RUEventFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).event_funcs) }
    }
}

impl<'a> DragLeaveEventTrait<'a> for DragLeaveEvent<'a> {
    #[inline]
    fn get_drag_leave_event_obj_funcs(&self) -> (*const RUBase, *const RUDragLeaveEventFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).drag_leave_event_funcs) }
    }
}
