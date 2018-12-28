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
/// Close events are sent to widgets that the user wants to close,
/// usually by choosing from the window menu, or by clicking
/// the **{X}** title bar button. They are also sent when you call
/// QWidget::close() to close a widget programmatically.
///
/// Close events contain a flag that indicates whether the receiver
/// wants the widget to be closed or not. When a widget accepts the
/// close event, it is hidden (and destroyed if it was created with
/// the Qt::WA_DeleteOnClose flag). If it refuses to accept the close
/// event nothing happens. (Under X11 it is possible that the window
/// manager will forcibly close the window; but at the time of writing
/// we are not aware of any window manager that does this.)
///
/// The event handler QWidget::closeEvent() receives close events. The
/// default implementation of this event handler accepts the close
/// event. If you do not want your widget to be hidden, or want some
/// special handling, you should reimplement the event handler and
/// ignore() the event.
///
/// The [closeEvent() in the
/// Application example](mainwindows/application%23close%20event%20handler)
/// shows a close event handler that
/// asks whether to save a document before closing.
///
/// If you want the widget to be deleted when it is closed, create it
/// with the Qt::WA_DeleteOnClose flag. This is very useful for
/// independent top-level windows in a multi-window application.
///
/// [QObject](QObject)
/// s emits the [destroyed()](QObject::destroyed())
///
/// signal when they are deleted.
///
/// If the last top-level window is closed, the
/// QGuiApplication::lastWindowClosed() signal is emitted.
///
/// The isAccepted() function returns `true` if the event's receiver has
/// agreed to close the widget; call accept() to agree to close the
/// widget and call ignore() if the receiver of this event does not
/// want the widget to be closed.
///
/// **See also:** [`Widget::close`]
/// [`Widget::hide`]
/// [`Object::destroyed`]
/// [`CoreApplication::exec`]
/// [`CoreApplication::quit`]
/// [`GuiApplication::last_window_closed`]
/// # Licence
///
/// The documentation is an adoption of the original [Qt Documentation](http://doc.qt.io/) and provided herein is licensed under the terms of the [GNU Free Documentation License version 1.3](http://www.gnu.org/licenses/fdl.html) as published by the Free Software Foundation.
#[derive(Clone)]
pub struct CloseEvent<'a> {
    #[doc(hidden)]
    pub data: Rc<Cell<Option<*const RUBase>>>,
    #[doc(hidden)]
    pub all_funcs: *const RUCloseEventAllFuncs,
    #[doc(hidden)]
    pub owned: bool,
    #[doc(hidden)]
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> CloseEvent<'a> {
    #[allow(dead_code)]
    pub(crate) fn new_from_rc(ffi_data: RUCloseEvent) -> CloseEvent<'a> {
        CloseEvent {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_owned(ffi_data: RUCloseEvent) -> CloseEvent<'a> {
        CloseEvent {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_temporary(ffi_data: RUCloseEvent) -> CloseEvent<'a> {
        CloseEvent {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
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

impl<'a> From<(WrapperRcOwn, bool)> for CloseEvent<'a> {
    fn from(t: (WrapperRcOwn, bool)) -> Self {
        if t.1 {
            CloseEvent::new_from_rc(t.0 as *const RUCloseEvent)
        } else {
            CloseEvent::new_from_temporary(t.0 as *const RUCloseEvent)
        }
    }
}

pub trait CloseEventTrait<'a> {
    #[inline]
    #[doc(hidden)]
    fn get_close_event_obj_funcs(&self) -> (*const RUBase, *const RUCloseEventFuncs);
}

impl<'a> EventTrait<'a> for CloseEvent<'a> {
    #[doc(hidden)]
    fn get_event_obj_funcs(&self) -> (*const RUBase, *const RUEventFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).event_funcs) }
    }
}

impl<'a> CloseEventTrait<'a> for CloseEvent<'a> {
    #[doc(hidden)]
    fn get_close_event_obj_funcs(&self) -> (*const RUBase, *const RUCloseEventFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).close_event_funcs) }
    }
}
