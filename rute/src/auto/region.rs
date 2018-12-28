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
/// QRegion is used with QPainter::setClipRegion() to limit the paint
/// area to what needs to be painted. There is also a QWidget::repaint()
/// function that takes a QRegion parameter. QRegion is the best tool for
/// minimizing the amount of screen area to be updated by a repaint.
///
/// This class is not suitable for constructing shapes for rendering, especially
/// as outlines. Use QPainterPath to create paths and shapes for use with
/// QPainter.
///
/// QRegion is an [implicitly shared](implicitly%20shared)
/// class.
///
/// # Creating and Using Regions
///
/// A region can be created from a rectangle, an ellipse, a polygon or
/// a bitmap. Complex regions may be created by combining simple
/// regions using united(), intersected(), subtracted(), or xored() (exclusive
/// or). You can move a region using translate().
///
/// You can test whether a region isEmpty() or if it
/// contains() a QPoint or QRect. The bounding rectangle can be found
/// with boundingRect().
///
/// Iteration over the region (with begin(), end(), or C++11
/// ranged-for loops) gives a decomposition of the region into
/// rectangles.
///
/// Example of using complex regions:
///
/// # Additional License Information
///
/// On Embedded Linux and X11 platforms, parts of this class rely on
/// code obtained under the following licenses:
///
/// ```
/// Copyright (c) 1987  X Consortium
///
/// Permission is hereby granted, free of charge, to any person obtaining a copy
/// of this software and associated documentation files (the "Software"), to deal
/// in the Software without restriction, including without limitation the rights
/// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
/// copies of the Software, and to permit persons to whom the Software is
/// furnished to do so, subject to the following conditions:
///
/// The above copyright notice and this permission notice shall be included in
/// all copies or substantial portions of the Software.
///
/// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
/// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
/// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL THE
/// X CONSORTIUM BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN
/// AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
/// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
///
/// Except as contained in this notice, the name of the X Consortium shall not be
/// used in advertising or otherwise to promote the sale, use or other dealings
/// in this Software without prior written authorization from the X Consortium.
/// ```
///
/// **r**
///
/// ```
/// Copyright 1987 by Digital Equipment Corporation, Maynard, Massachusetts.
///
/// All Rights Reserved
///
/// Permission to use, copy, modify, and distribute this software and its
/// documentation for any purpose and without fee is hereby granted,
/// provided that the above copyright notice appear in all copies and that
/// both that copyright notice and this permission notice appear in
/// supporting documentation, and that the name of Digital not be
/// used in advertising or publicity pertaining to distribution of the
/// software without specific, written prior permission.
///
/// DIGITAL DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE, INCLUDING
/// ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO EVENT SHALL
/// DIGITAL BE LIABLE FOR ANY SPECIAL, INDIRECT OR CONSEQUENTIAL DAMAGES OR
/// ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS,
/// WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION,
/// ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS
/// SOFTWARE.
/// ```
///
/// **See also:** [`Painter::set_clip_region`]
/// [`Painter::set_clip_rect`]
/// [`PainterPath`]
/// # Licence
///
/// The documentation is an adoption of the original [Qt Documentation](http://doc.qt.io/) and provided herein is licensed under the terms of the [GNU Free Documentation License version 1.3](http://www.gnu.org/licenses/fdl.html) as published by the Free Software Foundation.
#[derive(Clone)]
pub struct Region<'a> {
    #[doc(hidden)]
    pub data: Rc<Cell<Option<*const RUBase>>>,
    #[doc(hidden)]
    pub all_funcs: *const RURegionAllFuncs,
    #[doc(hidden)]
    pub owned: bool,
    #[doc(hidden)]
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> Region<'a> {
    pub fn new() -> Region<'a> {
        let data = Rc::new(Cell::new(None));

        let ffi_data = unsafe {
            ((*rute_ffi_get()).create_region)(
                ::std::ptr::null(),
                transmute(rute_object_delete_callback as usize),
                Rc::into_raw(data.clone()) as *const c_void,
            )
        };

        data.set(Some(ffi_data.qt_data));

        Region {
            data,
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }
    #[allow(dead_code)]
    pub(crate) fn new_from_rc(ffi_data: RURegion) -> Region<'a> {
        Region {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_owned(ffi_data: RURegion) -> Region<'a> {
        Region {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_temporary(ffi_data: RURegion) -> Region<'a> {
        Region {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }
    ///
    /// Swaps region *other* with this region. This operation is very
    /// fast and never fails.
    pub fn swap<R: RegionTrait<'a>>(&self, other: &R) -> &Self {
        let (obj_other_1, _funcs) = other.get_region_obj_funcs();

        let (obj_data, funcs) = self.get_region_obj_funcs();
        unsafe {
            ((*funcs).swap)(obj_data, obj_other_1);
        }
        self
    }
    ///
    /// Returns `true` if the region is empty; otherwise returns `false.` An
    /// empty region is a region that contains no points.
    ///
    /// Example:
    pub fn is_empty(&self) -> bool {
        let (obj_data, funcs) = self.get_region_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_empty)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns `true` if the region is empty; otherwise returns `false.` An
    /// empty region is a region that contains no points. This function is
    /// the same as isEmpty
    ///
    /// **See also:** [`is_empty()`]
    pub fn is_null(&self) -> bool {
        let (obj_data, funcs) = self.get_region_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_null)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns `true` if the region contains the point *p;* otherwise
    /// returns `false.`
    ///
    /// **Overloads**
    /// Returns `true` if the region overlaps the rectangle *r;* otherwise
    /// returns `false.`
    pub fn contains<P: PointTrait<'a>>(&self, p: &P) -> bool {
        let (obj_p_1, _funcs) = p.get_point_obj_funcs();

        let (obj_data, funcs) = self.get_region_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).contains)(obj_data, obj_p_1);
            ret_val
        }
    }
    ///
    /// Returns `true` if the region contains the point *p;* otherwise
    /// returns `false.`
    ///
    /// **Overloads**
    /// Returns `true` if the region overlaps the rectangle *r;* otherwise
    /// returns `false.`
    pub fn contains_2<R: RectTrait<'a>>(&self, r: &R) -> bool {
        let (obj_r_1, _funcs) = r.get_rect_obj_funcs();

        let (obj_data, funcs) = self.get_region_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).contains_2)(obj_data, obj_r_1);
            ret_val
        }
    }
    ///
    /// Returns a region which is the union of this region and the given *rect.*
    ///
    /// **See also:** [`intersected()`]
    /// [`subtracted()`]
    /// [`xored()`]
    ///
    /// Returns a region which is the union of this region and *r.*
    ///
    /// ![Region Union](runion.png)
    ///
    /// The figure shows the union of two elliptical regions.
    ///
    /// **See also:** [`intersected()`]
    /// [`subtracted()`]
    /// [`xored()`]
    pub fn united<R: RegionTrait<'a>>(&self, r: &R) -> Region {
        let (obj_r_1, _funcs) = r.get_region_obj_funcs();

        let (obj_data, funcs) = self.get_region_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).united)(obj_data, obj_r_1);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Region::new_from_rc(t);
            } else {
                ret_val = Region::new_from_owned(t);
            }
            ret_val
        }
    }
    ///
    /// Returns a region which is the union of this region and the given *rect.*
    ///
    /// **See also:** [`intersected()`]
    /// [`subtracted()`]
    /// [`xored()`]
    ///
    /// Returns a region which is the union of this region and *r.*
    ///
    /// ![Region Union](runion.png)
    ///
    /// The figure shows the union of two elliptical regions.
    ///
    /// **See also:** [`intersected()`]
    /// [`subtracted()`]
    /// [`xored()`]
    pub fn united_2<R: RectTrait<'a>>(&self, r: &R) -> Region {
        let (obj_r_1, _funcs) = r.get_rect_obj_funcs();

        let (obj_data, funcs) = self.get_region_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).united_2)(obj_data, obj_r_1);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Region::new_from_rc(t);
            } else {
                ret_val = Region::new_from_owned(t);
            }
            ret_val
        }
    }
    ///
    /// Returns a region which is the intersection of this region and the given *rect.*
    ///
    /// **See also:** [`subtracted()`]
    /// [`united()`]
    /// [`xored()`]
    ///
    /// Returns a region which is the intersection of this region and *r.*
    ///
    /// ![Region Intersection](rintersect.png)
    ///
    /// The figure shows the intersection of two elliptical regions.
    ///
    /// **See also:** [`subtracted()`]
    /// [`united()`]
    /// [`xored()`]
    pub fn intersected<R: RegionTrait<'a>>(&self, r: &R) -> Region {
        let (obj_r_1, _funcs) = r.get_region_obj_funcs();

        let (obj_data, funcs) = self.get_region_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).intersected)(obj_data, obj_r_1);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Region::new_from_rc(t);
            } else {
                ret_val = Region::new_from_owned(t);
            }
            ret_val
        }
    }
    ///
    /// Returns a region which is the intersection of this region and the given *rect.*
    ///
    /// **See also:** [`subtracted()`]
    /// [`united()`]
    /// [`xored()`]
    ///
    /// Returns a region which is the intersection of this region and *r.*
    ///
    /// ![Region Intersection](rintersect.png)
    ///
    /// The figure shows the intersection of two elliptical regions.
    ///
    /// **See also:** [`subtracted()`]
    /// [`united()`]
    /// [`xored()`]
    pub fn intersected_2<R: RectTrait<'a>>(&self, r: &R) -> Region {
        let (obj_r_1, _funcs) = r.get_rect_obj_funcs();

        let (obj_data, funcs) = self.get_region_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).intersected_2)(obj_data, obj_r_1);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Region::new_from_rc(t);
            } else {
                ret_val = Region::new_from_owned(t);
            }
            ret_val
        }
    }
    ///
    /// Returns a region which is the exclusive or (XOR) of this region
    /// and *r.*
    ///
    /// ![Region XORed](rxor.png)
    ///
    /// The figure shows the exclusive or of two elliptical regions.
    ///
    /// **See also:** [`intersected()`]
    /// [`united()`]
    /// [`subtracted()`]
    pub fn xored<R: RegionTrait<'a>>(&self, r: &R) -> Region {
        let (obj_r_1, _funcs) = r.get_region_obj_funcs();

        let (obj_data, funcs) = self.get_region_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).xored)(obj_data, obj_r_1);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Region::new_from_rc(t);
            } else {
                ret_val = Region::new_from_owned(t);
            }
            ret_val
        }
    }
    ///
    /// Returns `true` if this region intersects with *region,* otherwise
    /// returns `false.`
    ///
    /// Returns `true` if this region intersects with *rect,* otherwise
    /// returns `false.`
    pub fn intersects<R: RegionTrait<'a>>(&self, r: &R) -> bool {
        let (obj_r_1, _funcs) = r.get_region_obj_funcs();

        let (obj_data, funcs) = self.get_region_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).intersects)(obj_data, obj_r_1);
            ret_val
        }
    }
    ///
    /// Returns `true` if this region intersects with *region,* otherwise
    /// returns `false.`
    ///
    /// Returns `true` if this region intersects with *rect,* otherwise
    /// returns `false.`
    pub fn intersects_2<R: RectTrait<'a>>(&self, r: &R) -> bool {
        let (obj_r_1, _funcs) = r.get_rect_obj_funcs();

        let (obj_data, funcs) = self.get_region_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).intersects_2)(obj_data, obj_r_1);
            ret_val
        }
    }
    ///
    /// Returns the bounding rectangle of this region. An empty region
    /// gives a rectangle that is QRect::isNull().
    pub fn bounding_rect(&self) -> Rect {
        let (obj_data, funcs) = self.get_region_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).bounding_rect)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Rect::new_from_rc(t);
            } else {
                ret_val = Rect::new_from_owned(t);
            }
            ret_val
        }
    }
    ///
    /// Use begin() and end() instead.
    ///
    /// Returns an array of non-overlapping rectangles that make up the
    /// region.
    ///
    /// The union of all the rectangles is equal to the original region.
    ///
    /// Sets the region using the array of rectangles specified by *rects* and
    /// *number.*
    /// The rectangles *must* be optimally Y-X sorted and follow these restrictions:
    ///
    /// * The rectangles must not intersect.
    /// * All rectangles with a given top coordinate must have the same height.
    /// * No two rectangles may abut horizontally (they should be combined into a single wider rectangle in that case).
    /// * The rectangles must be sorted in ascending order, with Y as the major sort key and X as the minor sort key.
    ///
    pub fn set_rects<R: RectTrait<'a>>(&self, rect: &R, num: i32) -> &Self {
        let (obj_rect_1, _funcs) = rect.get_rect_obj_funcs();

        let (obj_data, funcs) = self.get_region_obj_funcs();
        unsafe {
            ((*funcs).set_rects)(obj_data, obj_rect_1, num);
        }
        self
    }
    ///
    /// Returns the number of rectangles that this region is composed of.
    /// Same as `end() - begin()` .
    pub fn rect_count(&self) -> i32 {
        let (obj_data, funcs) = self.get_region_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).rect_count)(obj_data);
            ret_val
        }
    }

    pub fn build(&self) -> Self {
        self.clone()
    }
}

impl<'a> From<(WrapperRcOwn, bool)> for Region<'a> {
    fn from(t: (WrapperRcOwn, bool)) -> Self {
        if t.1 {
            Region::new_from_rc(t.0 as *const RURegion)
        } else {
            Region::new_from_temporary(t.0 as *const RURegion)
        }
    }
}

pub trait RegionTrait<'a> {
    #[inline]
    #[doc(hidden)]
    fn get_region_obj_funcs(&self) -> (*const RUBase, *const RURegionFuncs);
}

impl<'a> RegionTrait<'a> for Region<'a> {
    #[doc(hidden)]
    fn get_region_obj_funcs(&self) -> (*const RUBase, *const RURegionFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).region_funcs) }
    }
}
#[repr(u32)]
pub enum RegionType {
    Rectangle = 0,
    Ellipse = 1,
}
