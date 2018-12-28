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
/// Context menu events are sent to widgets when a user performs
/// an action associated with opening a context menu.
/// The actions required to open context menus vary between platforms;
/// for example, on Windows, pressing the menu button or clicking the
/// right mouse button will cause this event to be sent.
///
/// When this event occurs it is customary to show a QMenu with a
/// context menu, if this is relevant to the context.
///
/// Context menu events contain a special accept flag that indicates
/// whether the receiver accepted the event. If the event handler does
/// not accept the event then, if possible, whatever triggered the event will be
/// handled as a regular input event.
/// # Licence
///
/// The documentation is an adoption of the original [Qt Documentation](http://doc.qt.io/) and provided herein is licensed under the terms of the [GNU Free Documentation License version 1.3](http://www.gnu.org/licenses/fdl.html) as published by the Free Software Foundation.
#[derive(Clone)]
pub struct ContextMenuEvent<'a> {
    #[doc(hidden)]
    pub data: Rc<Cell<Option<*const RUBase>>>,
    #[doc(hidden)]
    pub all_funcs: *const RUContextMenuEventAllFuncs,
    #[doc(hidden)]
    pub owned: bool,
    #[doc(hidden)]
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> ContextMenuEvent<'a> {
    #[allow(dead_code)]
    pub(crate) fn new_from_rc(ffi_data: RUContextMenuEvent) -> ContextMenuEvent<'a> {
        ContextMenuEvent {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_owned(ffi_data: RUContextMenuEvent) -> ContextMenuEvent<'a> {
        ContextMenuEvent {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_temporary(ffi_data: RUContextMenuEvent) -> ContextMenuEvent<'a> {
        ContextMenuEvent {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }
    ///
    /// Returns the x position of the mouse pointer, relative to the
    /// widget that received the event.
    ///
    /// **See also:** [`y()`]
    /// [`pos()`]
    pub fn x(&self) -> i32 {
        let (obj_data, funcs) = self.get_context_menu_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).x)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the y position of the mouse pointer, relative to the
    /// widget that received the event.
    ///
    /// **See also:** [`x()`]
    /// [`pos()`]
    pub fn y(&self) -> i32 {
        let (obj_data, funcs) = self.get_context_menu_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).y)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the global x position of the mouse pointer at the time of
    /// the event.
    ///
    /// **See also:** [`global_y()`]
    /// [`global_pos()`]
    pub fn global_x(&self) -> i32 {
        let (obj_data, funcs) = self.get_context_menu_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).global_x)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the global y position of the mouse pointer at the time of
    /// the event.
    ///
    /// **See also:** [`global_x()`]
    /// [`global_pos()`]
    pub fn global_y(&self) -> i32 {
        let (obj_data, funcs) = self.get_context_menu_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).global_y)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the position of the mouse pointer relative to the widget
    /// that received the event.
    ///
    /// **See also:** [`x()`]
    /// [`y()`]
    /// [`global_pos()`]
    pub fn pos(&self) -> Option<Point> {
        let (obj_data, funcs) = self.get_context_menu_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).pos)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Point::new_from_rc(t);
            } else {
                ret_val = Point::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    ///
    /// Returns the global position of the mouse pointer at the time of
    /// the event.
    ///
    /// **See also:** [`x()`]
    /// [`y()`]
    /// [`pos()`]
    pub fn global_pos(&self) -> Option<Point> {
        let (obj_data, funcs) = self.get_context_menu_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).global_pos)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Point::new_from_rc(t);
            } else {
                ret_val = Point::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    ///
    /// Returns the reason for this context event.
    pub fn reason(&self) -> Reason {
        let (obj_data, funcs) = self.get_context_menu_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).reason)(obj_data);
            let ret_val = { transmute::<u32, Reason>(ret_val) };
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn modifiers(&self) -> KeyboardModifiers {
        let (obj_data, funcs) = self.get_input_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).modifiers)(obj_data);
            let ret_val = KeyboardModifiers::from_bits_truncate(ret_val);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn set_modifiers(&self, amodifiers: KeyboardModifiers) -> &Self {
        let enum_amodifiers_1 = amodifiers.bits();

        let (obj_data, funcs) = self.get_input_event_obj_funcs();
        unsafe {
            ((*funcs).set_modifiers)(obj_data, enum_amodifiers_1);
        }
        self
    }
    #[doc(hidden)]
    pub fn timestamp(&self) -> u64 {
        let (obj_data, funcs) = self.get_input_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).timestamp)(obj_data);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn set_timestamp(&self, atimestamp: u64) -> &Self {
        let (obj_data, funcs) = self.get_input_event_obj_funcs();
        unsafe {
            ((*funcs).set_timestamp)(obj_data, atimestamp);
        }
        self
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

impl<'a> From<(WrapperRcOwn, bool)> for ContextMenuEvent<'a> {
    fn from(t: (WrapperRcOwn, bool)) -> Self {
        if t.1 {
            ContextMenuEvent::new_from_rc(t.0 as *const RUContextMenuEvent)
        } else {
            ContextMenuEvent::new_from_temporary(t.0 as *const RUContextMenuEvent)
        }
    }
}

pub trait ContextMenuEventTrait<'a> {
    #[inline]
    #[doc(hidden)]
    fn get_context_menu_event_obj_funcs(&self) -> (*const RUBase, *const RUContextMenuEventFuncs);
}

impl<'a> EventTrait<'a> for ContextMenuEvent<'a> {
    #[doc(hidden)]
    fn get_event_obj_funcs(&self) -> (*const RUBase, *const RUEventFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).event_funcs) }
    }
}

impl<'a> InputEventTrait<'a> for ContextMenuEvent<'a> {
    #[doc(hidden)]
    fn get_input_event_obj_funcs(&self) -> (*const RUBase, *const RUInputEventFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).input_event_funcs) }
    }
}

impl<'a> ContextMenuEventTrait<'a> for ContextMenuEvent<'a> {
    #[doc(hidden)]
    fn get_context_menu_event_obj_funcs(&self) -> (*const RUBase, *const RUContextMenuEventFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).context_menu_event_funcs) }
    }
}
#[repr(u32)]
pub enum Reason {
    Mouse = 0,
    Keyboard = 1,
    Other = 2,
}
