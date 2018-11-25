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
/// A QPolygonF is a QVector<QPointF>. The easiest way to add points
/// to a QPolygonF is to use its streaming operator, as illustrated
/// below:
///
/// In addition to the functions provided by QVector, QPolygonF
/// provides the boundingRect() and translate() functions for geometry
/// operations. Use the QMatrix::map() function for more general
/// transformations of QPolygonFs.
///
/// QPolygonF also provides the isClosed() function to determine
/// whether a polygon's start and end points are the same, and the
/// toPolygon() function returning an integer precision copy of this
/// polygon.
///
/// The QPolygonF class is [implicitly
/// shared](Implicit%20Data%20Sharing)
///
///
/// **See also:** QVector
/// QPolygon
/// QLineF
/// # Licence
///
/// The documentation is an adoption of the original [Qt Documentation](http://doc.qt.io/) and provided herein is licensed under the terms of the [GNU Free Documentation License version 1.3](http://www.gnu.org/licenses/fdl.html) as published by the Free Software Foundation.
#[derive(Clone)]
pub struct PolygonF<'a> {
    #[doc(hidden)]
    pub data: Rc<Cell<Option<*const RUBase>>>,
    #[doc(hidden)]
    pub all_funcs: *const RUPolygonFAllFuncs,
    #[doc(hidden)]
    pub owned: bool,
    #[doc(hidden)]
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> PolygonF<'a> {
    pub fn new() -> PolygonF<'a> {
        let data = Rc::new(Cell::new(None));

        let ffi_data = unsafe {
            ((*rute_ffi_get()).create_polygon_f)(
                ::std::ptr::null(),
                transmute(rute_object_delete_callback as usize),
                Rc::into_raw(data.clone()) as *const c_void,
            )
        };

        data.set(Some(ffi_data.qt_data));

        PolygonF {
            data,
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }
    #[allow(dead_code)]
    pub(crate) fn new_from_rc(ffi_data: RUPolygonF) -> PolygonF<'a> {
        PolygonF {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_owned(ffi_data: RUPolygonF) -> PolygonF<'a> {
        PolygonF {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_temporary(ffi_data: RUPolygonF) -> PolygonF<'a> {
        PolygonF {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }
    ///
    /// Swaps polygon *other* with this polygon. This operation is very
    /// fast and never fails.
    pub fn swap<P: PolygonFTrait<'a>>(&self, other: &P) -> &Self {
        let (obj_other_1, _funcs) = other.get_polygon_f_obj_funcs();

        let (obj_data, funcs) = self.get_polygon_f_obj_funcs();
        unsafe {
            ((*funcs).swap)(obj_data, obj_other_1);
        }
        self
    }
    ///
    /// Creates and returns a QPolygon by converting each QPointF to a
    /// QPoint.
    ///
    /// **See also:** QPointF::toPoint()
    pub fn to_polygon(&self) -> Polygon {
        let (obj_data, funcs) = self.get_polygon_f_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).to_polygon)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Polygon::new_from_rc(t);
            } else {
                ret_val = Polygon::new_from_owned(t);
            }
            ret_val
        }
    }
    ///
    /// Returns `true` if the polygon is closed; otherwise returns `false.`
    ///
    /// A polygon is said to be closed if its start point and end point are equal.
    ///
    /// **See also:** QVector::first()
    /// QVector::last()
    pub fn is_closed(&self) -> bool {
        let (obj_data, funcs) = self.get_polygon_f_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_closed)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the bounding rectangle of the polygon, or QRectF(0,0,0,0)
    /// if the polygon is empty.
    ///
    /// **See also:** QVector::isEmpty()
    pub fn bounding_rect(&self) -> RectF {
        let (obj_data, funcs) = self.get_polygon_f_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).bounding_rect)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = RectF::new_from_rc(t);
            } else {
                ret_val = RectF::new_from_owned(t);
            }
            ret_val
        }
    }
    ///
    /// Returns `true` if the given *point* is inside the polygon according to
    /// the specified *fillRule;* otherwise returns `false.`
    pub fn contains_point<P: PointFTrait<'a>>(&self, pt: &P, fill_rule: FillRule) -> bool {
        let (obj_pt_1, _funcs) = pt.get_point_f_obj_funcs();
        let enum_fill_rule_2 = fill_rule as i32;

        let (obj_data, funcs) = self.get_polygon_f_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).contains_point)(obj_data, obj_pt_1, enum_fill_rule_2);
            ret_val
        }
    }
    ///
    /// Returns a polygon which is the union of this polygon and *r.*
    ///
    /// Set operations on polygons will treat the polygons as
    /// areas. Non-closed polygons will be treated as implicitly closed.
    ///
    /// **See also:** intersected()
    /// subtracted()
    pub fn united<P: PolygonFTrait<'a>>(&self, r: &P) -> PolygonF {
        let (obj_r_1, _funcs) = r.get_polygon_f_obj_funcs();

        let (obj_data, funcs) = self.get_polygon_f_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).united)(obj_data, obj_r_1);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = PolygonF::new_from_rc(t);
            } else {
                ret_val = PolygonF::new_from_owned(t);
            }
            ret_val
        }
    }
    ///
    /// Returns a polygon which is the intersection of this polygon and *r.*
    ///
    /// Set operations on polygons will treat the polygons as
    /// areas. Non-closed polygons will be treated as implicitly closed.
    ///
    /// **See also:** intersects()
    pub fn intersected<P: PolygonFTrait<'a>>(&self, r: &P) -> PolygonF {
        let (obj_r_1, _funcs) = r.get_polygon_f_obj_funcs();

        let (obj_data, funcs) = self.get_polygon_f_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).intersected)(obj_data, obj_r_1);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = PolygonF::new_from_rc(t);
            } else {
                ret_val = PolygonF::new_from_owned(t);
            }
            ret_val
        }
    }
    ///
    /// Returns `true` if the current polygon intersects at any point the given polygon *p.*
    /// Also returns `true` if the current polygon contains or is contained by any part of *p.*
    ///
    /// Set operations on polygons will treat the polygons as
    /// areas. Non-closed polygons will be treated as implicitly closed.
    ///
    /// **See also:** intersected()
    pub fn intersects<P: PolygonFTrait<'a>>(&self, r: &P) -> bool {
        let (obj_r_1, _funcs) = r.get_polygon_f_obj_funcs();

        let (obj_data, funcs) = self.get_polygon_f_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).intersects)(obj_data, obj_r_1);
            ret_val
        }
    }
}
pub trait PolygonFTrait<'a> {
    #[inline]
    #[doc(hidden)]
    fn get_polygon_f_obj_funcs(&self) -> (*const RUBase, *const RUPolygonFFuncs);
}

impl<'a> PolygonFTrait<'a> for PolygonF<'a> {
    #[doc(hidden)]
    fn get_polygon_f_obj_funcs(&self) -> (*const RUBase, *const RUPolygonFFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).polygon_f_funcs) }
    }
}
