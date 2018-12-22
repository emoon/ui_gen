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
/// A QLineF describes a finite length line (or line segment) on a
/// two-dimensional surface. QLineF defines the start and end points
/// of the line using floating point accuracy for coordinates. Use
/// the toLine() function to retrieve an integer based copy of this
/// line.
///
/// * ![qline-point.png](qline-point.png)
///
/// * ![qline-coordinates.png](qline-coordinates.png)
///
/// The positions of the line's start and end points can be retrieved
/// using the p1(), x1(), y1(), p2(), x2(), and y2() functions. The
/// dx() and dy() functions return the horizontal and vertical
/// components of the line, respectively.
///
/// The line's length can be retrieved using the length() function,
/// and altered using the setLength() function. Similarly, angle()
/// and setAngle() are respectively used for retrieving and altering
/// the angle of the line. Use the isNull()
/// function to determine whether the QLineF represents a valid line
/// or a null line.
///
/// The intersect() function determines the IntersectType for this
/// line and a given line, while the angleTo() function returns the
/// angle between the lines. In addition, the unitVector() function
/// returns a line that has the same starting point as this line, but
/// with a length of only 1, while the normalVector() function returns
/// a line that is perpendicular to this line with the same starting
/// point and length.
///
/// Finally, the line can be translated a given offset using the
/// translate() function, and can be traversed using the pointAt()
/// function.
///
/// # Constraints
///
/// QLine is limited to the minimum and maximum values for the
/// `int` type. Operations on a QLine that could potentially result
/// in values outside this range will result in undefined behavior.
///
/// **See also:** [`Line`]
/// [`PolygonF`]
/// [`RectF`]
/// # Licence
///
/// The documentation is an adoption of the original [Qt Documentation](http://doc.qt.io/) and provided herein is licensed under the terms of the [GNU Free Documentation License version 1.3](http://www.gnu.org/licenses/fdl.html) as published by the Free Software Foundation.
#[derive(Clone)]
pub struct LineF<'a> {
    #[doc(hidden)]
    pub data: Rc<Cell<Option<*const RUBase>>>,
    #[doc(hidden)]
    pub all_funcs: *const RULineFAllFuncs,
    #[doc(hidden)]
    pub owned: bool,
    #[doc(hidden)]
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> LineF<'a> {
    pub fn new() -> LineF<'a> {
        let data = Rc::new(Cell::new(None));

        let ffi_data = unsafe {
            ((*rute_ffi_get()).create_line_f)(
                ::std::ptr::null(),
                transmute(rute_object_delete_callback as usize),
                Rc::into_raw(data.clone()) as *const c_void,
            )
        };

        data.set(Some(ffi_data.qt_data));

        LineF {
            data,
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }
    #[allow(dead_code)]
    pub(crate) fn new_from_rc(ffi_data: RULineF) -> LineF<'a> {
        LineF {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_owned(ffi_data: RULineF) -> LineF<'a> {
        LineF {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_temporary(ffi_data: RULineF) -> LineF<'a> {
        LineF {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }
    ///
    /// Returns a QLineF with the given *length* and *angle.*
    ///
    /// The first point of the line will be on the origin.
    ///
    /// Positive values for the angles mean counter-clockwise while negative values
    /// mean the clockwise direction. Zero degrees is at the 3 o'clock position.
    pub fn from_polar(length: f32, angle: f32) -> LineF<'a> {
        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_line_f)(::std::ptr::null()).all_funcs).line_f_funcs,
            )
        };
        unsafe {
            let ret_val = ((*funcs).from_polar)(obj_data, length, angle);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = LineF::new_from_rc(t);
            } else {
                ret_val = LineF::new_from_owned(t);
            }
            ret_val
        }
    }
    ///
    /// Returns `true` if the line is not set up with valid start and end point;
    /// otherwise returns `false.`
    pub fn is_null(&self) -> bool {
        let (obj_data, funcs) = self.get_line_f_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_null)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the line's start point.
    ///
    /// **See also:** [`x1()`]
    /// [`y1()`]
    /// [`p2()`]
    pub fn p1(&self) -> PointF {
        let (obj_data, funcs) = self.get_line_f_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).p1)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = PointF::new_from_rc(t);
            } else {
                ret_val = PointF::new_from_owned(t);
            }
            ret_val
        }
    }
    ///
    /// Returns the line's end point.
    ///
    /// **See also:** [`x2()`]
    /// [`y2()`]
    /// [`p1()`]
    pub fn p2(&self) -> PointF {
        let (obj_data, funcs) = self.get_line_f_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).p2)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = PointF::new_from_rc(t);
            } else {
                ret_val = PointF::new_from_owned(t);
            }
            ret_val
        }
    }
    ///
    /// Returns the x-coordinate of the line's start point.
    ///
    /// **See also:** [`p1()`]
    pub fn x1(&self) -> f32 {
        let (obj_data, funcs) = self.get_line_f_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).x1)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the y-coordinate of the line's start point.
    ///
    /// **See also:** [`p1()`]
    pub fn y1(&self) -> f32 {
        let (obj_data, funcs) = self.get_line_f_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).y1)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the x-coordinate of the line's end point.
    ///
    /// **See also:** [`p2()`]
    pub fn x2(&self) -> f32 {
        let (obj_data, funcs) = self.get_line_f_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).x2)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the y-coordinate of the line's end point.
    ///
    /// **See also:** [`p2()`]
    pub fn y2(&self) -> f32 {
        let (obj_data, funcs) = self.get_line_f_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).y2)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the horizontal component of the line's vector.
    ///
    /// **See also:** [`dy()`]
    /// [`point_at()`]
    pub fn dx(&self) -> f32 {
        let (obj_data, funcs) = self.get_line_f_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).dx)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the vertical component of the line's vector.
    ///
    /// **See also:** [`dx()`]
    /// [`point_at()`]
    pub fn dy(&self) -> f32 {
        let (obj_data, funcs) = self.get_line_f_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).dy)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the length of the line.
    ///
    /// **See also:** [`set_length()`]
    pub fn length(&self) -> f32 {
        let (obj_data, funcs) = self.get_line_f_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).length)(obj_data);
            ret_val
        }
    }
    ///
    /// Sets the length of the line to the given *length.* QLineF will
    /// move the end point - p2() - of the line to give the line its new length.
    ///
    /// If the line is a null line, the length will remain zero regardless
    /// of the length specified.
    ///
    /// **See also:** [`length()`]
    /// [`is_null()`]
    pub fn set_length(&self, len: f32) -> &Self {
        let (obj_data, funcs) = self.get_line_f_obj_funcs();
        unsafe {
            ((*funcs).set_length)(obj_data, len);
        }
        self
    }
    ///
    /// Returns the angle of the line in degrees.
    ///
    /// The return value will be in the range of values from 0.0 up to but not
    /// including 360.0. The angles are measured counter-clockwise from a point
    /// on the x-axis to the right of the origin (x > 0).
    ///
    /// **See also:** [`set_angle()`]
    ///
    /// Returns the angle (in degrees) from this line to the given *line,* taking the direction of the lines into account. If the lines
    /// do not intersect within their range, it is the intersection point of
    /// the extended lines that serves as origin (see
    /// QLineF::UnboundedIntersection).
    ///
    /// The returned value represents the number of degrees you need to add
    /// to this line to make it have the same angle as the given *line,*
    /// going counter-clockwise.
    ///
    /// **See also:** [`intersect()`]
    ///
    /// Returns the angle (in degrees) between this line and the given *line,* taking the direction of the lines into account. If the lines
    /// do not intersect within their range, it is the intersection point of
    /// the extended lines that serves as origin (see
    /// QLineF::UnboundedIntersection).
    ///
    /// * ![qlinef-angle-identicaldirection.png](qlinef-angle-identicaldirection.png)
    ///
    /// * ![qlinef-angle-oppositedirection.png](qlinef-angle-oppositedirection.png)
    ///
    /// When the lines are parallel, this function returns 0 if they have
    /// the same direction; otherwise it returns 180.
    ///
    /// **See also:** [`intersect()`]
    pub fn angle(&self) -> f32 {
        let (obj_data, funcs) = self.get_line_f_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).angle)(obj_data);
            ret_val
        }
    }
    ///
    /// Sets the angle of the line to the given *angle* (in degrees).
    /// This will change the position of the second point of the line such that
    /// the line has the given angle.
    ///
    /// Positive values for the angles mean counter-clockwise while negative values
    /// mean the clockwise direction. Zero degrees is at the 3 o'clock position.
    ///
    /// **See also:** [`angle()`]
    pub fn set_angle(&self, angle: f32) -> &Self {
        let (obj_data, funcs) = self.get_line_f_obj_funcs();
        unsafe {
            ((*funcs).set_angle)(obj_data, angle);
        }
        self
    }
    ///
    /// Returns the angle (in degrees) from this line to the given *line,* taking the direction of the lines into account. If the lines
    /// do not intersect within their range, it is the intersection point of
    /// the extended lines that serves as origin (see
    /// QLineF::UnboundedIntersection).
    ///
    /// The returned value represents the number of degrees you need to add
    /// to this line to make it have the same angle as the given *line,*
    /// going counter-clockwise.
    ///
    /// **See also:** [`intersect()`]
    pub fn angle_to<L: LineFTrait<'a>>(&self, l: &L) -> f32 {
        let (obj_l_1, _funcs) = l.get_line_f_obj_funcs();

        let (obj_data, funcs) = self.get_line_f_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).angle_to)(obj_data, obj_l_1);
            ret_val
        }
    }
    ///
    /// Returns the unit vector for this line, i.e a line starting at the
    /// same point as *this* line with a length of 1.0.
    ///
    /// **See also:** [`normal_vector()`]
    pub fn unit_vector(&self) -> LineF {
        let (obj_data, funcs) = self.get_line_f_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).unit_vector)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = LineF::new_from_rc(t);
            } else {
                ret_val = LineF::new_from_owned(t);
            }
            ret_val
        }
    }
    ///
    /// Returns a line that is perpendicular to this line with the same starting
    /// point and length.
    ///
    /// ![qlinef-normalvector.png](qlinef-normalvector.png)
    ///
    /// **See also:** [`unit_vector()`]
    pub fn normal_vector(&self) -> LineF {
        let (obj_data, funcs) = self.get_line_f_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).normal_vector)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = LineF::new_from_rc(t);
            } else {
                ret_val = LineF::new_from_owned(t);
            }
            ret_val
        }
    }
    ///
    /// Returns a value indicating whether or not *this* line intersects
    /// with the given *line.*
    ///
    /// The actual intersection point is extracted to *intersectionPoint*
    /// (if the pointer is valid). If the lines are parallel, the
    /// intersection point is undefined.
    pub fn intersect<L: LineFTrait<'a>, P: PointFTrait<'a>>(
        &self,
        l: &L,
        intersection_point: &P,
    ) -> IntersectType {
        let (obj_l_1, _funcs) = l.get_line_f_obj_funcs();
        let (obj_intersection_point_2, _funcs) = intersection_point.get_point_f_obj_funcs();

        let (obj_data, funcs) = self.get_line_f_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).intersect)(obj_data, obj_l_1, obj_intersection_point_2);
            let ret_val = { transmute::<u32, IntersectType>(ret_val) };
            ret_val
        }
    }
    ///
    /// Returns the angle of the line in degrees.
    ///
    /// The return value will be in the range of values from 0.0 up to but not
    /// including 360.0. The angles are measured counter-clockwise from a point
    /// on the x-axis to the right of the origin (x > 0).
    ///
    /// **See also:** [`set_angle()`]
    ///
    /// Returns the angle (in degrees) from this line to the given *line,* taking the direction of the lines into account. If the lines
    /// do not intersect within their range, it is the intersection point of
    /// the extended lines that serves as origin (see
    /// QLineF::UnboundedIntersection).
    ///
    /// The returned value represents the number of degrees you need to add
    /// to this line to make it have the same angle as the given *line,*
    /// going counter-clockwise.
    ///
    /// **See also:** [`intersect()`]
    ///
    /// Returns the angle (in degrees) between this line and the given *line,* taking the direction of the lines into account. If the lines
    /// do not intersect within their range, it is the intersection point of
    /// the extended lines that serves as origin (see
    /// QLineF::UnboundedIntersection).
    ///
    /// * ![qlinef-angle-identicaldirection.png](qlinef-angle-identicaldirection.png)
    ///
    /// * ![qlinef-angle-oppositedirection.png](qlinef-angle-oppositedirection.png)
    ///
    /// When the lines are parallel, this function returns 0 if they have
    /// the same direction; otherwise it returns 180.
    ///
    /// **See also:** [`intersect()`]
    pub fn angle_2<L: LineFTrait<'a>>(&self, l: &L) -> f32 {
        let (obj_l_1, _funcs) = l.get_line_f_obj_funcs();

        let (obj_data, funcs) = self.get_line_f_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).angle_2)(obj_data, obj_l_1);
            ret_val
        }
    }
    ///
    /// Returns the point at the parameterized position specified by *t.* The function returns the line's start point if t = 0, and its end
    /// point if t = 1.
    ///
    /// **See also:** [`dx()`]
    /// [`dy()`]
    pub fn point_at(&self, t: f32) -> PointF {
        let (obj_data, funcs) = self.get_line_f_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).point_at)(obj_data, t);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = PointF::new_from_rc(t);
            } else {
                ret_val = PointF::new_from_owned(t);
            }
            ret_val
        }
    }
    ///
    /// Returns the center point of this line. This is equivalent to
    /// 0.5 * p1() + 0.5 * p2().
    pub fn center(&self) -> PointF {
        let (obj_data, funcs) = self.get_line_f_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).center)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = PointF::new_from_rc(t);
            } else {
                ret_val = PointF::new_from_owned(t);
            }
            ret_val
        }
    }
    ///
    /// Sets the starting point of this line to *p1.*
    ///
    /// **See also:** [`set_p2()`]
    /// [`p1()`]
    pub fn set_p1<P: PointFTrait<'a>>(&self, p1: &P) -> &Self {
        let (obj_p1_1, _funcs) = p1.get_point_f_obj_funcs();

        let (obj_data, funcs) = self.get_line_f_obj_funcs();
        unsafe {
            ((*funcs).set_p1)(obj_data, obj_p1_1);
        }
        self
    }
    ///
    /// Sets the end point of this line to *p2.*
    ///
    /// **See also:** [`set_p1()`]
    /// [`p2()`]
    pub fn set_p2<P: PointFTrait<'a>>(&self, p2: &P) -> &Self {
        let (obj_p2_1, _funcs) = p2.get_point_f_obj_funcs();

        let (obj_data, funcs) = self.get_line_f_obj_funcs();
        unsafe {
            ((*funcs).set_p2)(obj_data, obj_p2_1);
        }
        self
    }
    ///
    /// Sets the start point of this line to *p1* and the end point of this line to *p2.*
    ///
    /// **See also:** [`set_p1()`]
    /// [`set_p2()`]
    /// [`p1()`]
    /// [`p2()`]
    pub fn set_points<P: PointFTrait<'a>>(&self, p1: &P, p2: &P) -> &Self {
        let (obj_p1_1, _funcs) = p1.get_point_f_obj_funcs();
        let (obj_p2_2, _funcs) = p2.get_point_f_obj_funcs();

        let (obj_data, funcs) = self.get_line_f_obj_funcs();
        unsafe {
            ((*funcs).set_points)(obj_data, obj_p1_1, obj_p2_2);
        }
        self
    }
    ///
    /// Sets this line to the start in *x1,* *y1* and end in *x2,* *y2.*
    ///
    /// **See also:** [`set_p1()`]
    /// [`set_p2()`]
    /// [`p1()`]
    /// [`p2()`]
    pub fn set_line(&self, x1: f32, y1: f32, x2: f32, y2: f32) -> &Self {
        let (obj_data, funcs) = self.get_line_f_obj_funcs();
        unsafe {
            ((*funcs).set_line)(obj_data, x1, y1, x2, y2);
        }
        self
    }
    ///
    /// Returns an integer based copy of this line.
    ///
    /// Note that the returned line's start and end points are rounded to
    /// the nearest integer.
    ///
    /// **See also:** [`q_line_f()`]
    pub fn to_line(&self) -> Line {
        let (obj_data, funcs) = self.get_line_f_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).to_line)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Line::new_from_rc(t);
            } else {
                ret_val = Line::new_from_owned(t);
            }
            ret_val
        }
    }

    pub fn build(&self) -> Self {
        self.clone()
    }
}
pub trait LineFTrait<'a> {
    #[inline]
    #[doc(hidden)]
    fn get_line_f_obj_funcs(&self) -> (*const RUBase, *const RULineFFuncs);
}

impl<'a> LineFTrait<'a> for LineF<'a> {
    #[doc(hidden)]
    fn get_line_f_obj_funcs(&self) -> (*const RUBase, *const RULineFFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).line_f_funcs) }
    }
}
#[repr(u32)]
pub enum IntersectType {
    NoIntersection = 0,
    BoundedIntersection = 1,
    UnboundedIntersection = 2,
}
