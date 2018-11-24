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
use auto::event::*;
#[allow(unused_imports)]
use auto::event_ffi::*;
#[allow(unused_imports)]
use auto::input_event::*;
#[allow(unused_imports)]
use auto::input_event_ffi::*;
#[allow(unused_imports)]
use auto::key_event_ffi::*;
#[allow(unused_imports)]
use auto::key_sequence::StandardKey;
#[allow(unused_imports)]
use auto::rute::*;
#[allow(unused_imports)]
use auto::rute_enums::KeyboardModifiers;
#[allow(unused_imports)]
use auto::rute_ffi::*;
///
/// Key events are sent to the widget with keyboard input focus
/// when keys are pressed or released.
///
/// A key event contains a special accept flag that indicates whether
/// the receiver will handle the key event. This flag is set by default
/// for QEvent::KeyPress and QEvent::KeyRelease, so there is no need to
/// call accept() when acting on a key event. For QEvent::ShortcutOverride
/// the receiver needs to explicitly accept the event to trigger the override.
/// Calling ignore() on a key event will propagate it to the parent widget.
/// The event is propagated up the parent widget chain until a widget
/// accepts it or an event filter consumes it.
///
/// The QWidget::setEnabled() function can be used to enable or disable
/// mouse and keyboard events for a widget.
///
/// The event handlers QWidget::keyPressEvent(), QWidget::keyReleaseEvent(),
/// QGraphicsItem::keyPressEvent() and QGraphicsItem::keyReleaseEvent()
/// receive key events.
///
/// **See also:** [`FocusEvent`]
/// [`Widget::grab_keyboard`]
/// # Licence
///
/// The documentation is an adoption of the original [Qt Documentation](http://doc.qt.io/) and provided herein is licensed under the terms of the [GNU Free Documentation License version 1.3](http://www.gnu.org/licenses/fdl.html) as published by the Free Software Foundation.
#[derive(Clone)]
pub struct KeyEvent<'a> {
    pub data: Rc<Cell<Option<*const RUBase>>>,
    pub all_funcs: *const RUKeyEventAllFuncs,
    pub owned: bool,
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> KeyEvent<'a> {
    pub fn new_from_rc(ffi_data: RUKeyEvent) -> KeyEvent<'a> {
        KeyEvent {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    pub fn new_from_owned(ffi_data: RUKeyEvent) -> KeyEvent<'a> {
        KeyEvent {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    pub fn new_from_temporary(ffi_data: RUKeyEvent) -> KeyEvent<'a> {
        KeyEvent {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }
}
pub trait KeyEventTrait<'a> {
    ///
    /// Returns the code of the key that was pressed or released.
    ///
    /// See [Qt::Key](Qt::Key)
    /// for the list of keyboard codes. These codes are
    /// independent of the underlying window system. Note that this
    /// function does not distinguish between capital and non-capital
    /// letters, use the text() function (returning the Unicode text the
    /// key generated) for this purpose.
    ///
    /// A value of either 0 or Qt::Key_unknown means that the event is not
    /// the result of a known key; for example, it may be the result of
    /// a compose sequence, a keyboard macro, or due to key event
    /// compression.
    ///
    /// **See also:** [`t::wa_key_compression()`]
    fn key(&self) -> i32 {
        let (obj_data, funcs) = self.get_key_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).key)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns `true` if the key event matches the given standard *key;*
    /// otherwise returns `false.`
    fn matches(&self, key: StandardKey) -> bool {
        let enum_key_1 = key as i32;

        let (obj_data, funcs) = self.get_key_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).matches)(obj_data, enum_key_1);
            ret_val
        }
    }
    ///
    /// Returns the keyboard modifier flags that existed immediately
    /// after the event occurred.
    ///
    /// **Warning**: This function cannot always be trusted. The user can
    /// confuse it by pressing both **{Shift}** keys simultaneously and
    /// releasing one of them, for example.
    ///
    /// **See also:** [`GuiApplication::keyboard_modifiers`]
    fn modifiers(&self) -> KeyboardModifiers {
        let (obj_data, funcs) = self.get_key_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).modifiers)(obj_data);
            let ret_val = { transmute::<i32, KeyboardModifiers>(ret_val) };
            ret_val
        }
    }
    ///
    /// Returns the Unicode text that this key generated.
    ///
    /// Return values when modifier keys such as
    /// Shift, Control, Alt, and Meta are pressed
    /// differ among platforms and could return an empty string.
    ///
    /// **Note**: [key()](key())
    /// will always return a valid value,
    /// independent of modifier keys.
    ///
    /// **See also:** [`t::wa_key_compression()`]
    fn text(&self) -> String {
        let (obj_data, funcs) = self.get_key_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).text)(obj_data);
            let ret_val = CStr::from_ptr(ret_val).to_string_lossy().into_owned();
            ret_val
        }
    }
    ///
    /// Returns `true` if this event comes from an auto-repeating key;
    /// returns `false` if it comes from an initial key press.
    ///
    /// Note that if the event is a multiple-key compressed event that is
    /// partly due to auto-repeat, this function could return either true
    /// or false indeterminately.
    fn is_auto_repeat(&self) -> bool {
        let (obj_data, funcs) = self.get_key_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_auto_repeat)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the number of keys involved in this event. If text()
    /// is not empty, this is simply the length of the string.
    ///
    /// **See also:** [`t::wa_key_compression()`]
    fn count(&self) -> i32 {
        let (obj_data, funcs) = self.get_key_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).count)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the native scan code of the key event. If the key event
    /// does not contain this data 0 is returned.
    ///
    /// Note: The native scan code may be 0, even if the key event contains
    /// extended information.
    ///
    /// Note: On Mac OS/X, this function is not useful, because there is no
    /// way to get the scan code from Carbon or Cocoa. The function always
    /// returns 1 (or 0 in the case explained above).
    fn native_scan_code(&self) -> u32 {
        let (obj_data, funcs) = self.get_key_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).native_scan_code)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the native virtual key, or key sym of the key event.
    /// If the key event does not contain this data 0 is returned.
    ///
    /// Note: The native virtual key may be 0, even if the key event contains extended information.
    fn native_virtual_key(&self) -> u32 {
        let (obj_data, funcs) = self.get_key_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).native_virtual_key)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the native modifiers of a key event.
    /// If the key event does not contain this data 0 is returned.
    ///
    /// Note: The native modifiers may be 0, even if the key event contains extended information.
    fn native_modifiers(&self) -> u32 {
        let (obj_data, funcs) = self.get_key_event_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).native_modifiers)(obj_data);
            ret_val
        }
    }

    #[inline]
    fn get_key_event_obj_funcs(&self) -> (*const RUBase, *const RUKeyEventFuncs);
}

impl<'a> EventTrait<'a> for KeyEvent<'a> {
    #[inline]
    fn get_event_obj_funcs(&self) -> (*const RUBase, *const RUEventFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).event_funcs) }
    }
}

impl<'a> InputEventTrait<'a> for KeyEvent<'a> {
    #[inline]
    fn get_input_event_obj_funcs(&self) -> (*const RUBase, *const RUInputEventFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).input_event_funcs) }
    }
}

impl<'a> KeyEventTrait<'a> for KeyEvent<'a> {
    #[inline]
    fn get_key_event_obj_funcs(&self) -> (*const RUBase, *const RUKeyEventFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).key_event_funcs) }
    }
}