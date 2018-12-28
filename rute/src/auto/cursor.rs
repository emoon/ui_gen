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
/// This class is mainly used to create mouse cursors that are
/// associated with particular widgets and to get and set the position
/// of the mouse cursor.
///
/// Qt has a number of standard cursor shapes, but you can also make
/// custom cursor shapes based on a QBitmap, a mask and a hotspot.
///
/// To associate a cursor with a widget, use QWidget::setCursor(). To
/// associate a cursor with all widgets (normally for a short period
/// of time), use QGuiApplication::setOverrideCursor().
///
/// To set a cursor shape use QCursor::setShape() or use the QCursor
/// constructor which takes the shape as argument, or you can use one
/// of the predefined cursors defined in the [Qt::CursorShape](Qt::CursorShape)
/// enum.
///
/// If you want to create a cursor with your own bitmap, either use
/// the QCursor constructor which takes a bitmap and a mask or the
/// constructor which takes a pixmap as arguments.
///
/// To set or get the position of the mouse cursor use the static
/// methods QCursor::pos() and QCursor::setPos().
///
/// **Note:** It is possible to create a QCursor before
/// QGuiApplication, but it is not useful except as a place-holder for a
/// real QCursor created after QGuiApplication. Attempting to use a
/// QCursor that was created before QGuiApplication will result in a
/// crash.
///
/// # A Note for X11 Users
///
/// On X11, Qt supports the [Xcursor](Xcursor)
///
/// library, which allows for full color icon themes. The table below
/// shows the cursor name used for each Qt::CursorShape value. If a
/// cursor cannot be found using the name shown below, a standard X11
/// cursor will be used instead. Note: X11 does not provide
/// appropriate cursors for all possible Qt::CursorShape values. It
/// is possible that some cursors will be taken from the Xcursor
/// theme, while others will use an internal bitmap cursor.
///
/// * Shape
/// * Qt::CursorShape Value
/// * Cursor Name
/// * Shape
/// * Qt::CursorShape Value
/// * Cursor Name
/// * ![cursor-arrow.png](cursor-arrow.png)
///
/// * Qt::ArrowCursor
/// * `left_ptr`
/// * ![cursor-sizev.png](cursor-sizev.png)
///
/// * Qt::SizeVerCursor
/// * `size_ver`
/// * ![cursor-uparrow.png](cursor-uparrow.png)
///
/// * Qt::UpArrowCursor
/// * `up_arrow`
/// * ![cursor-sizeh.png](cursor-sizeh.png)
///
/// * Qt::SizeHorCursor
/// * `size_hor`
/// * ![cursor-cross.png](cursor-cross.png)
///
/// * Qt::CrossCursor
/// * `cross`
/// * ![cursor-sizeb.png](cursor-sizeb.png)
///
/// * Qt::SizeBDiagCursor
/// * `size_bdiag`
/// * ![cursor-ibeam.png](cursor-ibeam.png)
///
/// * Qt::IBeamCursor
/// * `ibeam`
/// * ![cursor-sizef.png](cursor-sizef.png)
///
/// * Qt::SizeFDiagCursor
/// * `size_fdiag`
/// * ![cursor-wait.png](cursor-wait.png)
///
/// * Qt::WaitCursor
/// * `wait`
/// * ![cursor-sizeall.png](cursor-sizeall.png)
///
/// * Qt::SizeAllCursor
/// * `size_all`
/// * ![cursor-busy.png](cursor-busy.png)
///
/// * Qt::BusyCursor
/// * `left_ptr_watch`
/// * ![cursor-vsplit.png](cursor-vsplit.png)
///
/// * Qt::SplitVCursor
/// * `split_v`
/// * ![cursor-forbidden.png](cursor-forbidden.png)
///
/// * Qt::ForbiddenCursor
/// * `forbidden`
/// * ![cursor-hsplit.png](cursor-hsplit.png)
///
/// * Qt::SplitHCursor
/// * `split_h`
/// * ![cursor-hand.png](cursor-hand.png)
///
/// * Qt::PointingHandCursor
/// * `pointing_hand`
/// * ![cursor-openhand.png](cursor-openhand.png)
///
/// * Qt::OpenHandCursor
/// * `openhand`
/// * ![cursor-whatsthis.png](cursor-whatsthis.png)
///
/// * Qt::WhatsThisCursor
/// * `whats_this`
/// * ![cursor-closedhand.png](cursor-closedhand.png)
///
/// * Qt::ClosedHandCursor
/// * `closedhand`
///
/// * Qt::DragMoveCursor
/// * `dnd-move` or `move`
///
/// * Qt::DragCopyCursor
/// * `dnd-copy` or `copy`
///
/// * Qt::DragLinkCursor
/// * `dnd-link` or `link`
///
/// **See also:** [`Widget`]
/// {fowler}{GUI Design Handbook: Cursors}
/// # Licence
///
/// The documentation is an adoption of the original [Qt Documentation](http://doc.qt.io/) and provided herein is licensed under the terms of the [GNU Free Documentation License version 1.3](http://www.gnu.org/licenses/fdl.html) as published by the Free Software Foundation.
#[derive(Clone)]
pub struct Cursor<'a> {
    #[doc(hidden)]
    pub data: Rc<Cell<Option<*const RUBase>>>,
    #[doc(hidden)]
    pub all_funcs: *const RUCursorAllFuncs,
    #[doc(hidden)]
    pub owned: bool,
    #[doc(hidden)]
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> Cursor<'a> {
    pub fn new() -> Cursor<'a> {
        let data = Rc::new(Cell::new(None));

        let ffi_data = unsafe {
            ((*rute_ffi_get()).create_cursor)(
                ::std::ptr::null(),
                transmute(rute_object_delete_callback as usize),
                Rc::into_raw(data.clone()) as *const c_void,
            )
        };

        data.set(Some(ffi_data.qt_data));

        Cursor {
            data,
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }
    #[allow(dead_code)]
    pub(crate) fn new_from_rc(ffi_data: RUCursor) -> Cursor<'a> {
        Cursor {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_owned(ffi_data: RUCursor) -> Cursor<'a> {
        Cursor {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_temporary(ffi_data: RUCursor) -> Cursor<'a> {
        Cursor {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }
    ///
    /// Swaps this cursor with the *other* cursor.
    pub fn swap<C: CursorTrait<'a>>(&self, other: &C) -> &Self {
        let (obj_other_1, _funcs) = other.get_cursor_obj_funcs();

        let (obj_data, funcs) = self.get_cursor_obj_funcs();
        unsafe {
            ((*funcs).swap)(obj_data, obj_other_1);
        }
        self
    }
    ///
    /// Returns the cursor shape identifier. The return value is one of
    /// the [Qt::CursorShape](Qt::CursorShape)
    /// enum values (cast to an int).
    ///
    /// **See also:** [`set_shape()`]
    pub fn shape(&self) -> CursorShape {
        let (obj_data, funcs) = self.get_cursor_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).shape)(obj_data);
            let ret_val = CursorShape::from_bits_truncate(ret_val);
            ret_val
        }
    }
    ///
    /// Sets the cursor to the shape identified by *shape.*
    ///
    /// See [Qt::CursorShape](Qt::CursorShape)
    /// for the list of cursor shapes.
    ///
    /// **See also:** [`shape()`]
    pub fn set_shape(&self, new_shape: CursorShape) -> &Self {
        let enum_new_shape_1 = new_shape.bits();

        let (obj_data, funcs) = self.get_cursor_obj_funcs();
        unsafe {
            ((*funcs).set_shape)(obj_data, enum_new_shape_1);
        }
        self
    }
    ///
    /// Returns the cursor bitmap, or 0 if it is one of the standard
    /// cursors.
    pub fn bitmap(&self) -> Option<Bitmap> {
        let (obj_data, funcs) = self.get_cursor_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).bitmap)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Bitmap::new_from_rc(t);
            } else {
                ret_val = Bitmap::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    ///
    /// Returns the cursor bitmap mask, or 0 if it is one of the standard
    /// cursors.
    pub fn mask(&self) -> Option<Bitmap> {
        let (obj_data, funcs) = self.get_cursor_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).mask)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Bitmap::new_from_rc(t);
            } else {
                ret_val = Bitmap::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    ///
    /// Returns the cursor pixmap. This is only valid if the cursor is a
    /// pixmap cursor.
    pub fn pixmap(&self) -> Pixmap {
        let (obj_data, funcs) = self.get_cursor_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).pixmap)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Pixmap::new_from_rc(t);
            } else {
                ret_val = Pixmap::new_from_owned(t);
            }
            ret_val
        }
    }
    ///
    /// Returns the cursor hot spot, or (0, 0) if it is one of the
    /// standard cursors.
    pub fn hot_spot(&self) -> Point {
        let (obj_data, funcs) = self.get_cursor_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).hot_spot)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Point::new_from_rc(t);
            } else {
                ret_val = Point::new_from_owned(t);
            }
            ret_val
        }
    }
    ///
    /// Returns the position of the cursor (hot spot) of the *screen*
    /// in global screen coordinates.
    ///
    /// You can call QWidget::mapFromGlobal() to translate it to widget
    /// coordinates.
    ///
    /// **See also:** [`set_pos()`]
    /// [`Widget::map_from_global`]
    /// [`Widget::map_to_global`]
    ///
    /// Returns the position of the cursor (hot spot) of
    /// the primary screen in global screen coordinates.
    ///
    /// You can call QWidget::mapFromGlobal() to translate it to widget
    /// coordinates.
    ///
    /// **Note**: The position is queried from the windowing system. If mouse events are generated
    /// via other means (e.g., via QWindowSystemInterface in a unit test), those fake mouse
    /// moves will not be reflected in the returned value.
    ///
    /// **Note**: On platforms where there is no windowing system or cursors are not available, the returned
    /// position is based on the mouse move events generated via QWindowSystemInterface.
    ///
    /// **See also:** [`set_pos()`]
    /// [`Widget::map_from_global`]
    /// [`Widget::map_to_global`]
    /// [`GuiApplication::primary_screen`]
    pub fn pos() -> Point<'a> {
        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_cursor)(::std::ptr::null()).all_funcs).cursor_funcs,
            )
        };
        unsafe {
            let ret_val = ((*funcs).pos)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Point::new_from_rc(t);
            } else {
                ret_val = Point::new_from_owned(t);
            }
            ret_val
        }
    }
    ///
    /// Returns the position of the cursor (hot spot) of the *screen*
    /// in global screen coordinates.
    ///
    /// You can call QWidget::mapFromGlobal() to translate it to widget
    /// coordinates.
    ///
    /// **See also:** [`set_pos()`]
    /// [`Widget::map_from_global`]
    /// [`Widget::map_to_global`]
    ///
    /// Returns the position of the cursor (hot spot) of
    /// the primary screen in global screen coordinates.
    ///
    /// You can call QWidget::mapFromGlobal() to translate it to widget
    /// coordinates.
    ///
    /// **Note**: The position is queried from the windowing system. If mouse events are generated
    /// via other means (e.g., via QWindowSystemInterface in a unit test), those fake mouse
    /// moves will not be reflected in the returned value.
    ///
    /// **Note**: On platforms where there is no windowing system or cursors are not available, the returned
    /// position is based on the mouse move events generated via QWindowSystemInterface.
    ///
    /// **See also:** [`set_pos()`]
    /// [`Widget::map_from_global`]
    /// [`Widget::map_to_global`]
    /// [`GuiApplication::primary_screen`]
    pub fn pos_2<S: ScreenTrait<'a>>(screen: &S) -> Point<'a> {
        let (obj_screen_1, _funcs) = screen.get_screen_obj_funcs();

        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_cursor)(::std::ptr::null()).all_funcs).cursor_funcs,
            )
        };
        unsafe {
            let ret_val = ((*funcs).pos_2)(obj_data, obj_screen_1);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Point::new_from_rc(t);
            } else {
                ret_val = Point::new_from_owned(t);
            }
            ret_val
        }
    }
    ///
    /// Moves the cursor (hot spot) of the *screen* to the global
    /// screen position ( *x,* *y).*
    ///
    /// You can call QWidget::mapToGlobal() to translate widget
    /// coordinates to global screen coordinates.
    ///
    /// **Note**: Calling this function results in changing the cursor position through the windowing
    /// system. The windowing system will typically respond by sending mouse events to the application's
    /// window. This means that the usage of this function should be avoided in unit tests and
    /// everywhere where fake mouse events are being injected via QWindowSystemInterface because the
    /// windowing system's mouse state (with regards to buttons for example) may not match the state in
    /// the application-generated events.
    ///
    /// **Note**: On platforms where there is no windowing system or cursors are not available, this
    /// function may do nothing.
    ///
    /// **See also:** [`pos()`]
    /// [`Widget::map_from_global`]
    /// [`Widget::map_to_global`]
    ///
    /// Moves the cursor (hot spot) of the primary screen
    /// to the global screen position ( *x,* *y).*
    ///
    /// You can call QWidget::mapToGlobal() to translate widget
    /// coordinates to global screen coordinates.
    ///
    /// **See also:** [`pos()`]
    /// [`Widget::map_from_global`]
    /// [`Widget::map_to_global`]
    /// [`GuiApplication::primary_screen`]
    ///
    /// **Overloads**
    /// Moves the cursor (hot spot) to the global screen position at point
    /// *p.*
    ///
    /// **Overloads**
    /// Moves the cursor (hot spot) to the global screen position of the
    /// *screen* at point *p.*
    pub fn set_pos(x: i32, y: i32) {
        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_cursor)(::std::ptr::null()).all_funcs).cursor_funcs,
            )
        };
        unsafe {
            ((*funcs).set_pos)(obj_data, x, y);
        }
    }
    ///
    /// Moves the cursor (hot spot) of the *screen* to the global
    /// screen position ( *x,* *y).*
    ///
    /// You can call QWidget::mapToGlobal() to translate widget
    /// coordinates to global screen coordinates.
    ///
    /// **Note**: Calling this function results in changing the cursor position through the windowing
    /// system. The windowing system will typically respond by sending mouse events to the application's
    /// window. This means that the usage of this function should be avoided in unit tests and
    /// everywhere where fake mouse events are being injected via QWindowSystemInterface because the
    /// windowing system's mouse state (with regards to buttons for example) may not match the state in
    /// the application-generated events.
    ///
    /// **Note**: On platforms where there is no windowing system or cursors are not available, this
    /// function may do nothing.
    ///
    /// **See also:** [`pos()`]
    /// [`Widget::map_from_global`]
    /// [`Widget::map_to_global`]
    ///
    /// Moves the cursor (hot spot) of the primary screen
    /// to the global screen position ( *x,* *y).*
    ///
    /// You can call QWidget::mapToGlobal() to translate widget
    /// coordinates to global screen coordinates.
    ///
    /// **See also:** [`pos()`]
    /// [`Widget::map_from_global`]
    /// [`Widget::map_to_global`]
    /// [`GuiApplication::primary_screen`]
    ///
    /// **Overloads**
    /// Moves the cursor (hot spot) to the global screen position at point
    /// *p.*
    ///
    /// **Overloads**
    /// Moves the cursor (hot spot) to the global screen position of the
    /// *screen* at point *p.*
    pub fn set_pos_2<S: ScreenTrait<'a>>(screen: &S, x: i32, y: i32) {
        let (obj_screen_1, _funcs) = screen.get_screen_obj_funcs();

        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_cursor)(::std::ptr::null()).all_funcs).cursor_funcs,
            )
        };
        unsafe {
            ((*funcs).set_pos_2)(obj_data, obj_screen_1, x, y);
        }
    }
    ///
    /// Moves the cursor (hot spot) of the *screen* to the global
    /// screen position ( *x,* *y).*
    ///
    /// You can call QWidget::mapToGlobal() to translate widget
    /// coordinates to global screen coordinates.
    ///
    /// **Note**: Calling this function results in changing the cursor position through the windowing
    /// system. The windowing system will typically respond by sending mouse events to the application's
    /// window. This means that the usage of this function should be avoided in unit tests and
    /// everywhere where fake mouse events are being injected via QWindowSystemInterface because the
    /// windowing system's mouse state (with regards to buttons for example) may not match the state in
    /// the application-generated events.
    ///
    /// **Note**: On platforms where there is no windowing system or cursors are not available, this
    /// function may do nothing.
    ///
    /// **See also:** [`pos()`]
    /// [`Widget::map_from_global`]
    /// [`Widget::map_to_global`]
    ///
    /// Moves the cursor (hot spot) of the primary screen
    /// to the global screen position ( *x,* *y).*
    ///
    /// You can call QWidget::mapToGlobal() to translate widget
    /// coordinates to global screen coordinates.
    ///
    /// **See also:** [`pos()`]
    /// [`Widget::map_from_global`]
    /// [`Widget::map_to_global`]
    /// [`GuiApplication::primary_screen`]
    ///
    /// **Overloads**
    /// Moves the cursor (hot spot) to the global screen position at point
    /// *p.*
    ///
    /// **Overloads**
    /// Moves the cursor (hot spot) to the global screen position of the
    /// *screen* at point *p.*
    pub fn set_pos_3<P: PointTrait<'a>>(p: &P) {
        let (obj_p_1, _funcs) = p.get_point_obj_funcs();

        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_cursor)(::std::ptr::null()).all_funcs).cursor_funcs,
            )
        };
        unsafe {
            ((*funcs).set_pos_3)(obj_data, obj_p_1);
        }
    }
    ///
    /// Moves the cursor (hot spot) of the *screen* to the global
    /// screen position ( *x,* *y).*
    ///
    /// You can call QWidget::mapToGlobal() to translate widget
    /// coordinates to global screen coordinates.
    ///
    /// **Note**: Calling this function results in changing the cursor position through the windowing
    /// system. The windowing system will typically respond by sending mouse events to the application's
    /// window. This means that the usage of this function should be avoided in unit tests and
    /// everywhere where fake mouse events are being injected via QWindowSystemInterface because the
    /// windowing system's mouse state (with regards to buttons for example) may not match the state in
    /// the application-generated events.
    ///
    /// **Note**: On platforms where there is no windowing system or cursors are not available, this
    /// function may do nothing.
    ///
    /// **See also:** [`pos()`]
    /// [`Widget::map_from_global`]
    /// [`Widget::map_to_global`]
    ///
    /// Moves the cursor (hot spot) of the primary screen
    /// to the global screen position ( *x,* *y).*
    ///
    /// You can call QWidget::mapToGlobal() to translate widget
    /// coordinates to global screen coordinates.
    ///
    /// **See also:** [`pos()`]
    /// [`Widget::map_from_global`]
    /// [`Widget::map_to_global`]
    /// [`GuiApplication::primary_screen`]
    ///
    /// **Overloads**
    /// Moves the cursor (hot spot) to the global screen position at point
    /// *p.*
    ///
    /// **Overloads**
    /// Moves the cursor (hot spot) to the global screen position of the
    /// *screen* at point *p.*
    pub fn set_pos_4<P: PointTrait<'a>, S: ScreenTrait<'a>>(screen: &S, p: &P) {
        let (obj_screen_1, _funcs) = screen.get_screen_obj_funcs();
        let (obj_p_2, _funcs) = p.get_point_obj_funcs();

        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_cursor)(::std::ptr::null()).all_funcs).cursor_funcs,
            )
        };
        unsafe {
            ((*funcs).set_pos_4)(obj_data, obj_screen_1, obj_p_2);
        }
    }

    pub fn build(&self) -> Self {
        self.clone()
    }
}

impl<'a> From<(WrapperRcOwn, bool)> for Cursor<'a> {
    fn from(t: (WrapperRcOwn, bool)) -> Self {
        if t.1 {
            Cursor::new_from_rc(t.0 as *const RUCursor)
        } else {
            Cursor::new_from_temporary(t.0 as *const RUCursor)
        }
    }
}

pub trait CursorTrait<'a> {
    #[inline]
    #[doc(hidden)]
    fn get_cursor_obj_funcs(&self) -> (*const RUBase, *const RUCursorFuncs);
}

impl<'a> CursorTrait<'a> for Cursor<'a> {
    #[doc(hidden)]
    fn get_cursor_obj_funcs(&self) -> (*const RUBase, *const RUCursorFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).cursor_funcs) }
    }
}
