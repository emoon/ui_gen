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
use auto::rute::*;
#[allow(unused_imports)]
use auto::rute_enums::Orientations;
#[allow(unused_imports)]
use auto::rute_ffi::*;
#[allow(unused_imports)]
use auto::size_policy_ffi::*;
///
/// The size policy of a widget is an expression of its willingness to
/// be resized in various ways, and affects how the widget is treated
/// by the [layout engine](Layout%20Management)
/// . Each widget returns a
/// QSizePolicy that describes the horizontal and vertical resizing
/// policy it prefers when being laid out. You can change this for
/// a specific widget by changing its QWidget::sizePolicy property.
///
/// QSizePolicy contains two independent QSizePolicy::Policy values
/// and two stretch factors; one describes the widgets's horizontal
/// size policy, and the other describes its vertical size policy. It
/// also contains a flag to indicate whether the height and width of
/// its preferred size are related.
///
/// The horizontal and vertical policies can be set in the
/// constructor, and altered using the setHorizontalPolicy() and
/// setVerticalPolicy() functions. The stretch factors can be set
/// using the setHorizontalStretch() and setVerticalStretch()
/// functions. The flag indicating whether the widget's
/// [sizeHint()](QWidget::sizeHint())
/// is width-dependent (such as a
/// menu bar or a word-wrapping label) can be set using the
/// setHeightForWidth() function.
///
/// The current size policies and stretch factors be retrieved using
/// the horizontalPolicy(), verticalPolicy(), horizontalStretch() and
/// verticalStretch() functions. Alternatively, use the transpose()
/// function to swap the horizontal and vertical policies and
/// stretches. The hasHeightForWidth() function returns the current
/// status of the flag indicating the size hint dependencies.
///
/// Use the expandingDirections() function to determine whether the
/// associated widget can make use of more space than its
/// [sizeHint()](QWidget::sizeHint())
/// function indicates, as well as
/// find out in which directions it can expand.
///
/// Finally, the QSizePolicy class provides operators comparing this
/// size policy to a given policy, as well as a QVariant operator
/// storing this QSizePolicy as a QVariant object.
///
/// **See also:** [`Size`]
/// [`Widget::size_hint`]
/// [`Widget::size_policy()`]
/// [`LayoutItem::size_hint`]
/// # Licence
///
/// The documentation is an adoption of the original [Qt Documentation](http://doc.qt.io/) and provided herein is licensed under the terms of the [GNU Free Documentation License version 1.3](http://www.gnu.org/licenses/fdl.html) as published by the Free Software Foundation.
#[derive(Clone)]
pub struct SizePolicy<'a> {
    pub data: Rc<Cell<Option<*const RUBase>>>,
    pub all_funcs: *const RUSizePolicyAllFuncs,
    pub owned: bool,
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> SizePolicy<'a> {
    pub fn new() -> SizePolicy<'a> {
        let data = Rc::new(Cell::new(None));

        let ffi_data = unsafe {
            ((*rute_ffi_get()).create_size_policy)(
                ::std::ptr::null(),
                transmute(rute_object_delete_callback as usize),
                Rc::into_raw(data.clone()) as *const c_void,
            )
        };

        data.set(Some(ffi_data.qt_data));

        SizePolicy {
            data,
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }
    pub fn new_from_rc(ffi_data: RUSizePolicy) -> SizePolicy<'a> {
        SizePolicy {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    pub fn new_from_owned(ffi_data: RUSizePolicy) -> SizePolicy<'a> {
        SizePolicy {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    pub fn new_from_temporary(ffi_data: RUSizePolicy) -> SizePolicy<'a> {
        SizePolicy {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }
}
pub trait SizePolicyTrait<'a> {
    ///
    /// Returns the horizontal component of the size policy.
    ///
    /// **See also:** [`set_horizontal_policy()`]
    /// [`vertical_policy()`]
    /// [`horizontal_stretch()`]
    fn horizontal_policy(&self) -> Policy {
        let (obj_data, funcs) = self.get_size_policy_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).horizontal_policy)(obj_data);
            let ret_val = { transmute::<i32, Policy>(ret_val) };
            ret_val
        }
    }
    ///
    /// Returns the vertical component of the size policy.
    ///
    /// **See also:** [`set_vertical_policy()`]
    /// [`horizontal_policy()`]
    /// [`vertical_stretch()`]
    fn vertical_policy(&self) -> Policy {
        let (obj_data, funcs) = self.get_size_policy_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).vertical_policy)(obj_data);
            let ret_val = { transmute::<i32, Policy>(ret_val) };
            ret_val
        }
    }
    ///
    /// Sets the horizontal component to the given *policy.*
    ///
    /// **See also:** [`horizontal_policy()`]
    /// [`set_vertical_policy()`]
    /// [`set_horizontal_stretch()`]
    fn set_horizontal_policy(&self, d: Policy) {
        let enum_d_1 = d as i32;

        let (obj_data, funcs) = self.get_size_policy_obj_funcs();
        unsafe {
            ((*funcs).set_horizontal_policy)(obj_data, enum_d_1);
        }
    }
    ///
    /// Sets the vertical component to the given *policy.*
    ///
    /// **See also:** [`vertical_policy()`]
    /// [`set_horizontal_policy()`]
    /// [`set_vertical_stretch()`]
    fn set_vertical_policy(&self, d: Policy) {
        let enum_d_1 = d as i32;

        let (obj_data, funcs) = self.get_size_policy_obj_funcs();
        unsafe {
            ((*funcs).set_vertical_policy)(obj_data, enum_d_1);
        }
    }
    ///
    /// Returns whether a widget can make use of more space than the
    /// QWidget::sizeHint() function indicates.
    ///
    /// A value of Qt::Horizontal or Qt::Vertical means that the widget
    /// can grow horizontally or vertically (i.e., the horizontal or
    /// vertical policy is [Expanding](Expanding)
    /// or [MinimumExpanding),](MinimumExpanding),)
    /// whereas
    /// Qt::Horizontal | Qt::Vertical means that it can grow in both
    /// dimensions.
    ///
    /// **See also:** [`horizontal_policy()`]
    /// [`vertical_policy()`]
    fn expanding_directions(&self) -> Orientations {
        let (obj_data, funcs) = self.get_size_policy_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).expanding_directions)(obj_data);
            let ret_val = { transmute::<i32, Orientations>(ret_val) };
            ret_val
        }
    }
    ///
    /// Sets the flag determining whether the widget's preferred height
    /// depends on its width, to *dependent.*
    ///
    /// **See also:** [`has_height_for_width()`]
    /// [`set_width_for_height()`]
    fn set_height_for_width(&self, b: bool) {
        let (obj_data, funcs) = self.get_size_policy_obj_funcs();
        unsafe {
            ((*funcs).set_height_for_width)(obj_data, b);
        }
    }
    ///
    /// Returns `true` if the widget's preferred height depends on its
    /// width; otherwise returns `false.`
    ///
    /// **See also:** [`set_height_for_width()`]
    fn has_height_for_width(&self) -> bool {
        let (obj_data, funcs) = self.get_size_policy_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).has_height_for_width)(obj_data);
            ret_val
        }
    }
    ///
    /// Sets the flag determining whether the widget's width
    /// depends on its height, to *dependent.*
    ///
    /// This is only supported for QGraphicsLayout's subclasses.
    /// It is not possible to have a layout with both height-for-width
    /// and width-for-height constraints at the same time.
    ///
    /// **See also:** [`has_width_for_height()`]
    /// [`set_height_for_width()`]
    fn set_width_for_height(&self, b: bool) {
        let (obj_data, funcs) = self.get_size_policy_obj_funcs();
        unsafe {
            ((*funcs).set_width_for_height)(obj_data, b);
        }
    }
    ///
    /// Returns `true` if the widget's width depends on its
    /// height; otherwise returns `false.`
    ///
    /// **See also:** [`set_width_for_height()`]
    fn has_width_for_height(&self) -> bool {
        let (obj_data, funcs) = self.get_size_policy_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).has_width_for_height)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns whether the layout should retain the widget's size when it is hidden.
    /// This is `false` by default.
    ///
    /// **See also:** [`set_retain_size_when_hidden()`]
    fn retain_size_when_hidden(&self) -> bool {
        let (obj_data, funcs) = self.get_size_policy_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).retain_size_when_hidden)(obj_data);
            ret_val
        }
    }
    ///
    /// Sets whether a layout should retain the widget's size when it is hidden.
    /// If *retainSize* is `true,` the layout will not be changed by hiding the widget.
    ///
    /// **See also:** [`retain_size_when_hidden()`]
    fn set_retain_size_when_hidden(&self, retain_size: bool) {
        let (obj_data, funcs) = self.get_size_policy_obj_funcs();
        unsafe {
            ((*funcs).set_retain_size_when_hidden)(obj_data, retain_size);
        }
    }

    #[inline]
    fn get_size_policy_obj_funcs(&self) -> (*const RUBase, *const RUSizePolicyFuncs);
}

impl<'a> SizePolicyTrait<'a> for SizePolicy<'a> {
    #[inline]
    fn get_size_policy_obj_funcs(&self) -> (*const RUBase, *const RUSizePolicyFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).size_policy_funcs) }
    }
}
#[repr(u32)]
pub enum PolicyFlag {
    GrowFlag,
    ExpandFlag,
    ShrinkFlag,
    IgnoreFlag,
}

#[repr(u32)]
pub enum Policy {
    Fixed,
    Minimum,
    Maximum,
    Preferred,
    MinimumExpanding,
    Expanding,
    Ignored,
}

#[repr(u32)]
pub enum ControlType {
    DefaultType,
    ButtonBox,
    CheckBox,
    ComboBox,
    Frame,
    GroupBox,
    Label,
    Line,
    LineEdit,
    PushButton,
    RadioButton,
    Slider,
    SpinBox,
    TabWidget,
    ToolButton,
}

pub type ControlTypes = ControlType;
