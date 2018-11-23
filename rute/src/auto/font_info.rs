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
use auto::font::Style;
#[allow(unused_imports)]
use auto::font_info_ffi::*;
#[allow(unused_imports)]
use auto::rute::*;
#[allow(unused_imports)]
use auto::rute_ffi::*;
#[allow(unused_imports)]
use auto::style::StyleHint;
///
/// The QFontInfo class provides the same access functions as QFont,
/// e.g. family(), pointSize(), italic(), weight(), fixedPitch(),
/// styleHint() etc. But whilst the QFont access functions return the
/// values that were set, a QFontInfo object returns the values that
/// apply to the font that will actually be used to draw the text.
///
/// For example, when the program asks for a 25pt Courier font on a
/// machine that has a non-scalable 24pt Courier font, QFont will
/// (normally) use the 24pt Courier for rendering. In this case,
/// QFont::pointSize() returns 25 and QFontInfo::pointSize() returns
/// 24.
///
/// There are three ways to create a QFontInfo object.
/// * Calling the QFontInfo constructor with a QFont creates a font info object for a screen-compatible font, i.e. the font cannot be a printer font. If the font is changed later, the font info object is *not* updated. (Note: If you use a printer font the values returned may be inaccurate. Printer fonts are not always accessible so the nearest screen font is used if a printer font is supplied.)
/// * QWidget::fontInfo() returns the font info for a widget's font. This is equivalent to calling QFontInfo(widget->font()). If the widget's font is changed later, the font info object is *not* updated.
/// * QPainter::fontInfo() returns the font info for a painter's current font. If the painter's font is changed later, the font info object is *not* updated.
///
/// **See also:** QFont
/// QFontMetrics
/// QFontDatabase
/// # Licence
///
/// The documentation is an adoption of the original [Qt Documentation](http://doc.qt.io/) and provided herein is licensed under the terms of the [GNU Free Documentation License version 1.3](http://www.gnu.org/licenses/fdl.html) as published by the Free Software Foundation.
#[derive(Clone)]
pub struct FontInfo<'a> {
    pub data: Rc<Cell<Option<*const RUBase>>>,
    pub all_funcs: *const RUFontInfoAllFuncs,
    pub owned: bool,
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> FontInfo<'a> {
    pub fn new_from_rc(ffi_data: RUFontInfo) -> FontInfo<'a> {
        FontInfo {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    pub fn new_from_owned(ffi_data: RUFontInfo) -> FontInfo<'a> {
        FontInfo {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    pub fn new_from_temporary(ffi_data: RUFontInfo) -> FontInfo<'a> {
        FontInfo {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }
}
pub trait FontInfoTrait<'a> {
    ///
    /// Swaps this font info instance with *other.* This function is very
    /// fast and never fails.
    fn swap(&self, other: &FontInfoTrait) {
        let (obj_other_1, _funcs) = other.get_font_info_obj_funcs();

        let (obj_data, funcs) = self.get_font_info_obj_funcs();
        unsafe {
            ((*funcs).swap)(obj_data, obj_other_1);
        }
    }
    ///
    /// Returns the family name of the matched window system font.
    ///
    /// **See also:** QFont::family()
    fn family(&self) -> String {
        let (obj_data, funcs) = self.get_font_info_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).family)(obj_data);
            let ret_val = CStr::from_ptr(ret_val).to_string_lossy().into_owned();
            ret_val
        }
    }
    ///
    /// Returns the style name of the matched window system font on
    /// systems that support it.
    ///
    /// **See also:** QFont::styleName()
    fn style_name(&self) -> String {
        let (obj_data, funcs) = self.get_font_info_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).style_name)(obj_data);
            let ret_val = CStr::from_ptr(ret_val).to_string_lossy().into_owned();
            ret_val
        }
    }
    ///
    /// Returns the pixel size of the matched window system font.
    ///
    /// **See also:** QFont::pointSize()
    fn pixel_size(&self) -> i32 {
        let (obj_data, funcs) = self.get_font_info_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).pixel_size)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the point size of the matched window system font.
    ///
    /// **See also:** pointSizeF()
    /// QFont::pointSize()
    ///
    /// Returns the point size of the matched window system font.
    ///
    /// **See also:** QFont::pointSizeF()
    fn point_size(&self) -> i32 {
        let (obj_data, funcs) = self.get_font_info_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).point_size)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the point size of the matched window system font.
    ///
    /// **See also:** QFont::pointSizeF()
    fn point_size_f(&self) -> f32 {
        let (obj_data, funcs) = self.get_font_info_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).point_size_f)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the italic value of the matched window system font.
    ///
    /// **See also:** QFont::italic()
    fn italic(&self) -> bool {
        let (obj_data, funcs) = self.get_font_info_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).italic)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the style name of the matched window system font on
    /// systems that support it.
    ///
    /// **See also:** QFont::styleName()
    ///
    /// Returns the style value of the matched window system font.
    ///
    /// **See also:** QFont::style()
    ///
    /// Returns the style of the matched window system font.
    ///
    /// Currently only returns the style hint set in QFont.
    ///
    /// **See also:** QFont::styleHint()
    /// QFont::StyleHint
    fn style(&self) -> Style {
        let (obj_data, funcs) = self.get_font_info_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).style)(obj_data);
            let ret_val = { transmute::<i32, Style>(ret_val) };
            ret_val
        }
    }
    ///
    /// Returns the weight of the matched window system font.
    ///
    /// **See also:** QFont::weight()
    /// bold()
    fn weight(&self) -> i32 {
        let (obj_data, funcs) = self.get_font_info_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).weight)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns `true` if weight() would return a value greater than
    /// QFont::Normal; otherwise returns `false.`
    ///
    /// **See also:** weight()
    /// QFont::bold()
    fn bold(&self) -> bool {
        let (obj_data, funcs) = self.get_font_info_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).bold)(obj_data);
            ret_val
        }
    }
    fn underline(&self) -> bool {
        let (obj_data, funcs) = self.get_font_info_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).underline)(obj_data);
            ret_val
        }
    }
    fn overline(&self) -> bool {
        let (obj_data, funcs) = self.get_font_info_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).overline)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the fixed pitch value of the matched window system font.
    ///
    /// **See also:** QFont::fixedPitch()
    fn fixed_pitch(&self) -> bool {
        let (obj_data, funcs) = self.get_font_info_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).fixed_pitch)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the style of the matched window system font.
    ///
    /// Currently only returns the style hint set in QFont.
    ///
    /// **See also:** QFont::styleHint()
    /// QFont::StyleHint
    fn style_hint(&self) -> StyleHint {
        let (obj_data, funcs) = self.get_font_info_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).style_hint)(obj_data);
            let ret_val = { transmute::<i32, StyleHint>(ret_val) };
            ret_val
        }
    }
    ///
    /// Returns `true` if the font is a raw mode font; otherwise returns
    /// false.
    ///
    /// If it is a raw mode font, all other functions in QFontInfo will
    /// return the same values set in the QFont, regardless of the font
    /// actually used.
    ///
    /// **See also:** QFont::rawMode()
    fn raw_mode(&self) -> bool {
        let (obj_data, funcs) = self.get_font_info_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).raw_mode)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns `true` if the matched window system font is exactly the same
    /// as the one specified by the font; otherwise returns `false.`
    ///
    /// **See also:** QFont::exactMatch()
    fn exact_match(&self) -> bool {
        let (obj_data, funcs) = self.get_font_info_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).exact_match)(obj_data);
            ret_val
        }
    }

    #[inline]
    fn get_font_info_obj_funcs(&self) -> (*const RUBase, *const RUFontInfoFuncs);
}

impl<'a> FontInfoTrait<'a> for FontInfo<'a> {
    #[inline]
    fn get_font_info_obj_funcs(&self) -> (*const RUBase, *const RUFontInfoFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).font_info_funcs) }
    }
}
