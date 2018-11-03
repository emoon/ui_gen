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
/// Qt currently supports three types of gradient fills:
///
/// * *Linear* gradients interpolate colors between start and end points.
/// * *Simple* radial gradients interpolate colors between a focal point and end points on a circle surrounding it.
/// * *Extended* radial gradients interpolate colors between a center and a focal circle.
/// * *Conical* gradients interpolate colors around a center point.
///
/// A gradient's type can be retrieved using the type() function.
/// Each of the types is represented by a subclass of QGradient:
///
/// * QLinearGradient
/// * QRadialGradient
/// * QConicalGradient
///
/// * ![qgradient-linear.png](qgradient-linear.png)
///
/// * ![qgradient-radial.png](qgradient-radial.png)
///
/// * ![qgradient-conical.png](qgradient-conical.png)
///
/// The colors in a gradient are defined using stop points of the
/// QGradientStop type; i.e., a position and a color. Use the setColorAt()
/// function to define a single stop point. Alternatively, use the
/// setStops() function to define several stop points in one go. Note that
/// the latter function *replaces* the current set of stop points.
///
/// It is the gradient's complete set of stop points (accessible
/// through the stops() function) that describes how the gradient area
/// should be filled. If no stop points have been specified, a gradient
/// of black at 0 to white at 1 is used.
///
/// A diagonal linear gradient from black at (100, 100) to white at
/// (200, 200) could be specified like this:
///
/// A gradient can have an arbitrary number of stop points. The
/// following would create a radial gradient starting with
/// red in the center, blue and then green on the edges:
///
/// It is possible to repeat or reflect the gradient outside its area
/// by specifiying the [spread method](QGradient::Spread)
/// using the
/// setSpread() function. The default is to pad the outside area with
/// the color at the closest stop point. The currently set [spread method](QGradient::Spread)
/// can be retrieved using the
/// spread() function. The QGradient::Spread enum defines three
/// different methods:
///
/// * ![qradialgradient-pad.png](qradialgradient-pad.png)
///
/// * ![qradialgradient-repeat.png](qradialgradient-repeat.png)
///
/// * ![qradialgradient-reflect.png](qradialgradient-reflect.png)
///
/// * [PadSpread](QGradient::PadSpread)
///
/// * [RepeatSpread](QGradient::RepeatSpread)
///
/// * [ReflectSpread](QGradient::ReflectSpread)
///
/// Note that the setSpread() function only has effect for linear and
/// radial gradients. The reason is that the conical gradient is
/// closed by definition, i.e. the *conical* gradient fills the
/// entire circle from 0 - 360 degrees, while the boundary of a radial
/// or a linear gradient can be specified through its radius or final
/// stop points, respectively.
///
/// The gradient coordinates can be specified in logical coordinates,
/// relative to device coordinates, or relative to object bounding box coordinates.
/// The [coordinate mode](QGradient::CoordinateMode)
/// can be set using the
/// setCoordinateMode() function. The default is LogicalMode, where the
/// gradient coordinates are specified in the same way as the object
/// coordinates. To retrieve the currently set [coordinate mode](QGradient::CoordinateMode)
/// use coordinateMode().
///
/// **See also:** {painting/gradients}{The Gradients Example}
/// QBrush
/// # Licence
///
/// The documentation is an adoption of the original [Qt Documentation](http://doc.qt.io/) and provided herein is licensed under the terms of the [GNU Free Documentation License version 1.3](http://www.gnu.org/licenses/fdl.html) as published by the Free Software Foundation.
#[derive(Clone)]
pub struct Gradient<'a> {
    pub data: Rc<Cell<Option<*const RUBase>>>,
    pub all_funcs: *const RUGradientAllFuncs,
    pub owned: bool,
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> Gradient<'a> {
    pub fn new() -> Gradient<'a> {
        let data = Rc::new(Cell::new(None));

        let ffi_data = unsafe {
            ((*rute_ffi_get()).create_gradient)(
                ::std::ptr::null(),
                transmute(rute_object_delete_callback as usize),
                Rc::into_raw(data.clone()) as *const c_void,
            )
        };

        data.set(Some(ffi_data.qt_data));

        Gradient {
            data,
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }
    pub fn new_from_rc(ffi_data: RUGradient) -> Gradient<'a> {
        Gradient {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    pub fn new_from_owned(ffi_data: RUGradient) -> Gradient<'a> {
        Gradient {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    pub fn new_from_temporary(ffi_data: RUGradient) -> Gradient<'a> {
        Gradient {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }
}
pub trait GradientType<'a> {
    /// Returns the type of gradient.
    fn get_type(&self) -> Type {
        let (obj_data, funcs) = self.get_gradient_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).get_type)(obj_data);
            let ret_val = { transmute::<i32, Type>(ret_val) };
            ret_val
        }
    }
    ///
    /// Specifies the spread *method* that should be used for this
    /// gradient.
    ///
    /// Note that this function only has effect for linear and radial
    /// gradients.
    ///
    /// **See also:** spread()
    fn set_spread(&self, spread: Spread) -> &Self {
        let enum_spread_1 = spread as i32;

        let (obj_data, funcs) = self.get_gradient_obj_funcs();
        unsafe {
            ((*funcs).set_spread)(obj_data, enum_spread_1);
        }
        self
    }
    ///
    /// Returns the spread method use by this gradient. The default is
    /// PadSpread.
    ///
    /// **See also:** setSpread()
    fn spread(&self) -> Spread {
        let (obj_data, funcs) = self.get_gradient_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).spread)(obj_data);
            let ret_val = { transmute::<i32, Spread>(ret_val) };
            ret_val
        }
    }
    ///
    /// Creates a stop point at the given *position* with the given *color.* The given *position* must be in the range 0 to 1.
    ///
    /// **See also:** setStops()
    /// stops()
    fn set_color_at<C: ColorType<'a>>(&self, pos: f32, color: &C) -> &Self {
        let (obj_color_2, _funcs) = color.get_color_obj_funcs();

        let (obj_data, funcs) = self.get_gradient_obj_funcs();
        unsafe {
            ((*funcs).set_color_at)(obj_data, pos, obj_color_2);
        }
        self
    }
    ///
    /// Returns the coordinate mode of this gradient. The default mode is
    /// LogicalMode.
    fn coordinate_mode(&self) -> CoordinateMode {
        let (obj_data, funcs) = self.get_gradient_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).coordinate_mode)(obj_data);
            let ret_val = { transmute::<i32, CoordinateMode>(ret_val) };
            ret_val
        }
    }
    ///
    /// Sets the coordinate mode of this gradient to *mode.* The default
    /// mode is LogicalMode.
    fn set_coordinate_mode(&self, mode: CoordinateMode) -> &Self {
        let enum_mode_1 = mode as i32;

        let (obj_data, funcs) = self.get_gradient_obj_funcs();
        unsafe {
            ((*funcs).set_coordinate_mode)(obj_data, enum_mode_1);
        }
        self
    }
    fn interpolation_mode(&self) -> InterpolationMode {
        let (obj_data, funcs) = self.get_gradient_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).interpolation_mode)(obj_data);
            let ret_val = { transmute::<i32, InterpolationMode>(ret_val) };
            ret_val
        }
    }
    fn set_interpolation_mode(&self, mode: InterpolationMode) -> &Self {
        let enum_mode_1 = mode as i32;

        let (obj_data, funcs) = self.get_gradient_obj_funcs();
        unsafe {
            ((*funcs).set_interpolation_mode)(obj_data, enum_mode_1);
        }
        self
    }

    #[inline]
    fn get_gradient_obj_funcs(&self) -> (*const RUBase, *const RUGradientFuncs);
}

impl<'a> GradientType<'a> for Gradient<'a> {
    #[inline]
    fn get_gradient_obj_funcs(&self) -> (*const RUBase, *const RUGradientFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).gradient_funcs) }
    }
}
