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
/// A point is specified by a x coordinate and an y coordinate which
/// can be accessed using the x() and y() functions. The coordinates
/// of the point are specified using floating point numbers for
/// accuracy. The isNull() function returns `true` if both x and y are
/// set to 0.0. The coordinates can be set (or altered) using the setX()
/// and setY() functions, or alternatively the rx() and ry() functions which
/// return references to the coordinates (allowing direct
/// manipulation).
///
/// Given a point *p,* the following statements are all equivalent:
///
/// A QPointF object can also be used as a vector: Addition and
/// subtraction are defined as for vectors (each component is added
/// separately). A QPointF object can also be divided or multiplied by
/// an `int` or a `qreal.`
///
/// In addition, the QPointF class provides a constructor converting a
/// QPoint object into a QPointF object, and a corresponding toPoint()
/// function which returns a QPoint copy of *this* point. Finally,
/// QPointF objects can be streamed as well as compared.
///
/// **See also:** QPoint
/// QPolygonF
/// # Licence
///
/// The documentation is an adoption of the original [Qt Documentation](http://doc.qt.io/) and provided herein is licensed under the terms of the [GNU Free Documentation License version 1.3](http://www.gnu.org/licenses/fdl.html) as published by the Free Software Foundation.
#[derive(Clone)]
pub struct PointF<'a> {
    #[doc(hidden)]
    pub data: Rc<Cell<Option<*const RUBase>>>,
    #[doc(hidden)]
    pub all_funcs: *const RUPointFAllFuncs,
    #[doc(hidden)]
    pub owned: bool,
    #[doc(hidden)]
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> PointF<'a> {
    pub fn new() -> PointF<'a> {
        let data = Rc::new(Cell::new(None));

        let ffi_data = unsafe {
            ((*rute_ffi_get()).create_point_f)(
                ::std::ptr::null(),
                transmute(rute_object_delete_callback as usize),
                Rc::into_raw(data.clone()) as *const c_void,
            )
        };

        data.set(Some(ffi_data.qt_data));

        PointF {
            data,
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }
    #[allow(dead_code)]
    pub(crate) fn new_from_rc(ffi_data: RUPointF) -> PointF<'a> {
        PointF {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_owned(ffi_data: RUPointF) -> PointF<'a> {
        PointF {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_temporary(ffi_data: RUPointF) -> PointF<'a> {
        PointF {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }
    ///
    /// Returns the sum of the absolute values of x() and y(),
    /// traditionally known as the of the vector from
    /// the origin to the point.
    ///
    /// **See also:** QPoint::manhattanLength()
    pub fn manhattan_length(&self) -> f32 {
        let (obj_data, funcs) = self.get_point_f_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).manhattan_length)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns `true` if both the x and y coordinates are set to 0.0 (ignoring
    /// the sign); otherwise returns `false.`
    pub fn is_null(&self) -> bool {
        let (obj_data, funcs) = self.get_point_f_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_null)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the x coordinate of this point.
    ///
    /// **See also:** setX()
    /// rx()
    pub fn x(&self) -> f32 {
        let (obj_data, funcs) = self.get_point_f_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).x)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the y coordinate of this point.
    ///
    /// **See also:** setY()
    /// ry()
    pub fn y(&self) -> f32 {
        let (obj_data, funcs) = self.get_point_f_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).y)(obj_data);
            ret_val
        }
    }
    ///
    /// Sets the x coordinate of this point to the given *x* coordinate.
    ///
    /// **See also:** x()
    /// setY()
    pub fn set_x(&self, x: f32) -> &Self {
        let (obj_data, funcs) = self.get_point_f_obj_funcs();
        unsafe {
            ((*funcs).set_x)(obj_data, x);
        }
        self
    }
    ///
    /// Sets the y coordinate of this point to the given *y* coordinate.
    ///
    /// **See also:** y()
    /// setX()
    pub fn set_y(&self, y: f32) -> &Self {
        let (obj_data, funcs) = self.get_point_f_obj_funcs();
        unsafe {
            ((*funcs).set_y)(obj_data, y);
        }
        self
    }
    ///
    /// Returns a reference to the x coordinate of this point.
    ///
    /// Using a reference makes it possible to directly manipulate x. For example:
    ///
    /// **See also:** x()
    /// setX()
    pub fn rx(&self) -> f32 {
        let (obj_data, funcs) = self.get_point_f_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).rx)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns a reference to the y coordinate of this point.
    ///
    /// Using a reference makes it possible to directly manipulate y. For example:
    ///
    /// **See also:** y()
    /// setY()
    pub fn ry(&self) -> f32 {
        let (obj_data, funcs) = self.get_point_f_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).ry)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the dot product of *p1* and *p2.*
    pub fn dot_product<P: PointFTrait<'a>>(p1: &P, p2: &P) -> f32 {
        let (obj_p1_1, _funcs) = p1.get_point_f_obj_funcs();
        let (obj_p2_2, _funcs) = p2.get_point_f_obj_funcs();

        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_point_f)(::std::ptr::null()).all_funcs).point_f_funcs,
            )
        };
        unsafe {
            let ret_val = ((*funcs).dot_product)(obj_data, obj_p1_1, obj_p2_2);
            ret_val
        }
    }
    ///
    /// Rounds the coordinates of this point to the nearest integer, and
    /// returns a QPoint object with the rounded coordinates.
    ///
    /// **See also:** QPointF()
    pub fn to_point(&self) -> Point {
        let (obj_data, funcs) = self.get_point_f_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).to_point)(obj_data);
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
}
pub trait PointFTrait<'a> {
    #[inline]
    #[doc(hidden)]
    fn get_point_f_obj_funcs(&self) -> (*const RUBase, *const RUPointFFuncs);
}

impl<'a> PointFTrait<'a> for PointF<'a> {
    #[doc(hidden)]
    fn get_point_f_obj_funcs(&self) -> (*const RUBase, *const RUPointFFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).point_f_funcs) }
    }
}
