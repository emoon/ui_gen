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
/// The QBitmap class is a monochrome off-screen paint device used
/// mainly for creating custom QCursor and QBrush objects,
/// constructing QRegion objects, and for setting masks for pixmaps
/// and widgets.
///
/// QBitmap is a QPixmap subclass ensuring a depth of 1, except for
/// null objects which have a depth of 0. If a pixmap with a depth
/// greater than 1 is assigned to a bitmap, the bitmap will be
/// dithered automatically.
///
/// Use the QColor objects Qt::color0 and Qt::color1 when drawing on a
/// QBitmap object (or a QPixmap object with depth 1).
///
/// Painting with Qt::color0 sets the bitmap bits to 0, and painting
/// with Qt::color1 sets the bits to 1. For a bitmap, 0-bits indicate
/// background (or transparent pixels) and 1-bits indicate foreground
/// (or opaque pixels). Use the clear() function to set all the bits
/// to Qt::color0. Note that using the Qt::black and Qt::white colors
/// make no sense because the QColor::pixel() value is not necessarily
/// 0 for black and 1 for white.
///
/// The QBitmap class provides the transformed() function returning a
/// transformed copy of the bitmap; use the QTransform argument to
/// translate, scale, shear, and rotate the bitmap. In addition,
/// QBitmap provides the static fromData() function which returns a
/// bitmap constructed from the given `uchar` data, and the static
/// fromImage() function returning a converted copy of a QImage
/// object.
///
/// Just like the QPixmap class, QBitmap is optimized by the use of
/// implicit data sharing. For more information, see the [Implicit
/// Data Sharing](Implicit%0A%20%20%20%20Data%20Sharing)
/// documentation.
///
/// **See also:** [`Pixmap`]
/// [`Image`]
/// [`ImageReader`]
/// [`ImageWriter`]
/// # Licence
///
/// The documentation is an adoption of the original [Qt Documentation](http://doc.qt.io/) and provided herein is licensed under the terms of the [GNU Free Documentation License version 1.3](http://www.gnu.org/licenses/fdl.html) as published by the Free Software Foundation.
#[derive(Clone)]
pub struct Bitmap<'a> {
    #[doc(hidden)]
    pub data: Rc<Cell<Option<*const RUBase>>>,
    #[doc(hidden)]
    pub all_funcs: *const RUBitmapAllFuncs,
    #[doc(hidden)]
    pub owned: bool,
    #[doc(hidden)]
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> Bitmap<'a> {
    pub fn new() -> Bitmap<'a> {
        let ffi_data = unsafe { ((*rute_ffi_get()).create_bitmap)(::std::ptr::null()) };
        Bitmap {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }
    #[allow(dead_code)]
    pub(crate) fn new_from_rc(ffi_data: RUBitmap) -> Bitmap<'a> {
        Bitmap {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_owned(ffi_data: RUBitmap) -> Bitmap<'a> {
        Bitmap {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_temporary(ffi_data: RUBitmap) -> Bitmap<'a> {
        Bitmap {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }
    ///
    /// Swaps bitmap *other* with this bitmap. This operation is very
    /// fast and never fails.
    pub fn swap<B: BitmapTrait<'a>>(&self, other: &B) -> &Self {
        let (obj_other_1, _funcs) = other.get_bitmap_obj_funcs();

        let (obj_data, funcs) = self.get_bitmap_obj_funcs();
        unsafe {
            ((*funcs).swap)(obj_data, obj_other_1);
        }
        self
    }
    ///
    /// Clears the bitmap, setting all its bits to Qt::color0.
    pub fn clear(&self) -> &Self {
        let (obj_data, funcs) = self.get_bitmap_obj_funcs();
        unsafe {
            ((*funcs).clear)(obj_data);
        }
        self
    }
    ///
    /// Returns a copy of the given *image* converted to a bitmap using
    /// the specified image conversion *flags.*
    ///
    /// **See also:** [`from_data()`]
    pub fn from_image<I: ImageTrait<'a>>(image: &I, flags: ImageConversionFlags) -> Bitmap<'a> {
        let (obj_image_1, _funcs) = image.get_image_obj_funcs();
        let enum_flags_2 = flags.bits();

        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_bitmap)(::std::ptr::null()).all_funcs).bitmap_funcs,
            )
        };
        unsafe {
            let ret_val = ((*funcs).from_image)(obj_data, obj_image_1, enum_flags_2);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Bitmap::new_from_rc(t);
            } else {
                ret_val = Bitmap::new_from_owned(t);
            }
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn is_null(&self) -> bool {
        let (obj_data, funcs) = self.get_pixmap_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_null)(obj_data);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn width(&self) -> i32 {
        let (obj_data, funcs) = self.get_pixmap_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).width)(obj_data);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn height(&self) -> i32 {
        let (obj_data, funcs) = self.get_pixmap_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).height)(obj_data);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn size(&self) -> Size {
        let (obj_data, funcs) = self.get_pixmap_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).size)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Size::new_from_rc(t);
            } else {
                ret_val = Size::new_from_owned(t);
            }
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn rect(&self) -> Rect {
        let (obj_data, funcs) = self.get_pixmap_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).rect)(obj_data);
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
    #[doc(hidden)]
    pub fn depth(&self) -> i32 {
        let (obj_data, funcs) = self.get_pixmap_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).depth)(obj_data);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn default_depth() -> i32 {
        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_pixmap)(::std::ptr::null()).all_funcs).pixmap_funcs,
            )
        };
        unsafe {
            let ret_val = ((*funcs).default_depth)(obj_data);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn fill<C: ColorTrait<'a>>(&self, fill_color: &C) -> &Self {
        let (obj_fill_color_1, _funcs) = fill_color.get_color_obj_funcs();

        let (obj_data, funcs) = self.get_pixmap_obj_funcs();
        unsafe {
            ((*funcs).fill)(obj_data, obj_fill_color_1);
        }
        self
    }
    #[doc(hidden)]
    pub fn fill_2<P: PaintDeviceTrait<'a>, Q: PointTrait<'a>>(&self, device: &P, ofs: &Q) -> &Self {
        let (obj_device_1, _funcs) = device.get_paint_device_obj_funcs();
        let (obj_ofs_2, _funcs) = ofs.get_point_obj_funcs();

        let (obj_data, funcs) = self.get_pixmap_obj_funcs();
        unsafe {
            ((*funcs).fill_2)(obj_data, obj_device_1, obj_ofs_2);
        }
        self
    }
    #[doc(hidden)]
    pub fn fill_3<P: PaintDeviceTrait<'a>>(&self, device: &P, xofs: i32, yofs: i32) -> &Self {
        let (obj_device_1, _funcs) = device.get_paint_device_obj_funcs();

        let (obj_data, funcs) = self.get_pixmap_obj_funcs();
        unsafe {
            ((*funcs).fill_3)(obj_data, obj_device_1, xofs, yofs);
        }
        self
    }
    #[doc(hidden)]
    pub fn mask(&self) -> Bitmap {
        let (obj_data, funcs) = self.get_pixmap_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).mask)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Bitmap::new_from_rc(t);
            } else {
                ret_val = Bitmap::new_from_owned(t);
            }
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn set_mask<B: BitmapTrait<'a>>(&self, arg0: &B) -> &Self {
        let (obj_arg0_1, _funcs) = arg0.get_bitmap_obj_funcs();

        let (obj_data, funcs) = self.get_pixmap_obj_funcs();
        unsafe {
            ((*funcs).set_mask)(obj_data, obj_arg0_1);
        }
        self
    }
    #[doc(hidden)]
    pub fn device_pixel_ratio(&self) -> f32 {
        let (obj_data, funcs) = self.get_pixmap_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).device_pixel_ratio)(obj_data);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn set_device_pixel_ratio(&self, scale_factor: f32) -> &Self {
        let (obj_data, funcs) = self.get_pixmap_obj_funcs();
        unsafe {
            ((*funcs).set_device_pixel_ratio)(obj_data, scale_factor);
        }
        self
    }
    #[doc(hidden)]
    pub fn has_alpha(&self) -> bool {
        let (obj_data, funcs) = self.get_pixmap_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).has_alpha)(obj_data);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn has_alpha_channel(&self) -> bool {
        let (obj_data, funcs) = self.get_pixmap_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).has_alpha_channel)(obj_data);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn create_heuristic_mask(&self, clip_tight: bool) -> Bitmap {
        let (obj_data, funcs) = self.get_pixmap_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).create_heuristic_mask)(obj_data, clip_tight);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Bitmap::new_from_rc(t);
            } else {
                ret_val = Bitmap::new_from_owned(t);
            }
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn create_mask_from_color<C: ColorTrait<'a>>(
        &self,
        mask_color: &C,
        mode: MaskMode,
    ) -> Bitmap {
        let (obj_mask_color_1, _funcs) = mask_color.get_color_obj_funcs();
        let enum_mode_2 = mode as u32;

        let (obj_data, funcs) = self.get_pixmap_obj_funcs();
        unsafe {
            let ret_val =
                ((*funcs).create_mask_from_color)(obj_data, obj_mask_color_1, enum_mode_2);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Bitmap::new_from_rc(t);
            } else {
                ret_val = Bitmap::new_from_owned(t);
            }
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn grab_window(arg0: u64, x: i32, y: i32, w: i32, h: i32) -> Pixmap<'a> {
        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_pixmap)(::std::ptr::null()).all_funcs).pixmap_funcs,
            )
        };
        unsafe {
            let ret_val = ((*funcs).grab_window)(obj_data, arg0, x, y, w, h);
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
    #[doc(hidden)]
    pub fn grab_widget<O: ObjectTrait<'a>, R: RectTrait<'a>>(widget: &O, rect: &R) -> Pixmap<'a> {
        let (obj_widget_1, _funcs) = widget.get_object_obj_funcs();
        let (obj_rect_2, _funcs) = rect.get_rect_obj_funcs();

        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_pixmap)(::std::ptr::null()).all_funcs).pixmap_funcs,
            )
        };
        unsafe {
            let ret_val = ((*funcs).grab_widget)(obj_data, obj_widget_1, obj_rect_2);
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
    #[doc(hidden)]
    pub fn grab_widget_2<O: ObjectTrait<'a>>(
        widget: &O,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
    ) -> Pixmap<'a> {
        let (obj_widget_1, _funcs) = widget.get_object_obj_funcs();

        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_pixmap)(::std::ptr::null()).all_funcs).pixmap_funcs,
            )
        };
        unsafe {
            let ret_val = ((*funcs).grab_widget_2)(obj_data, obj_widget_1, x, y, w, h);
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
    #[doc(hidden)]
    pub fn scaled(
        &self,
        w: i32,
        h: i32,
        aspect_mode: AspectRatioMode,
        mode: TransformationMode,
    ) -> Pixmap {
        let enum_aspect_mode_3 = aspect_mode as u32;
        let enum_mode_4 = mode as u32;

        let (obj_data, funcs) = self.get_pixmap_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).scaled)(obj_data, w, h, enum_aspect_mode_3, enum_mode_4);
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
    #[doc(hidden)]
    pub fn scaled_2<S: SizeTrait<'a>>(
        &self,
        s: &S,
        aspect_mode: AspectRatioMode,
        mode: TransformationMode,
    ) -> Pixmap {
        let (obj_s_1, _funcs) = s.get_size_obj_funcs();
        let enum_aspect_mode_2 = aspect_mode as u32;
        let enum_mode_3 = mode as u32;

        let (obj_data, funcs) = self.get_pixmap_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).scaled_2)(obj_data, obj_s_1, enum_aspect_mode_2, enum_mode_3);
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
    #[doc(hidden)]
    pub fn scaled_to_width(&self, w: i32, mode: TransformationMode) -> Pixmap {
        let enum_mode_2 = mode as u32;

        let (obj_data, funcs) = self.get_pixmap_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).scaled_to_width)(obj_data, w, enum_mode_2);
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
    #[doc(hidden)]
    pub fn scaled_to_height(&self, h: i32, mode: TransformationMode) -> Pixmap {
        let enum_mode_2 = mode as u32;

        let (obj_data, funcs) = self.get_pixmap_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).scaled_to_height)(obj_data, h, enum_mode_2);
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
    #[doc(hidden)]
    pub fn to_image(&self) -> Image {
        let (obj_data, funcs) = self.get_pixmap_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).to_image)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Image::new_from_rc(t);
            } else {
                ret_val = Image::new_from_owned(t);
            }
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn from_image_2<I: ImageTrait<'a>>(image: &I, flags: ImageConversionFlags) -> Pixmap<'a> {
        let (obj_image_1, _funcs) = image.get_image_obj_funcs();
        let enum_flags_2 = flags.bits();

        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_pixmap)(::std::ptr::null()).all_funcs).pixmap_funcs,
            )
        };
        unsafe {
            let ret_val = ((*funcs).from_image_2)(obj_data, obj_image_1, enum_flags_2);
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
    #[doc(hidden)]
    pub fn convert_from_image<I: ImageTrait<'a>>(
        &self,
        img: &I,
        flags: ImageConversionFlags,
    ) -> bool {
        let (obj_img_1, _funcs) = img.get_image_obj_funcs();
        let enum_flags_2 = flags.bits();

        let (obj_data, funcs) = self.get_pixmap_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).convert_from_image)(obj_data, obj_img_1, enum_flags_2);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn copy(&self, x: i32, y: i32, width: i32, height: i32) -> Pixmap {
        let (obj_data, funcs) = self.get_pixmap_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).copy)(obj_data, x, y, width, height);
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
    #[doc(hidden)]
    pub fn copy_2<R: RectTrait<'a>>(&self, rect: &R) -> Pixmap {
        let (obj_rect_1, _funcs) = rect.get_rect_obj_funcs();

        let (obj_data, funcs) = self.get_pixmap_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).copy_2)(obj_data, obj_rect_1);
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
    #[doc(hidden)]
    pub fn scroll<R: RegionTrait<'a>>(
        &self,
        dx: i32,
        dy: i32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        exposed: &R,
    ) -> &Self {
        let (obj_exposed_7, _funcs) = exposed.get_region_obj_funcs();

        let (obj_data, funcs) = self.get_pixmap_obj_funcs();
        unsafe {
            ((*funcs).scroll)(obj_data, dx, dy, x, y, width, height, obj_exposed_7);
        }
        self
    }
    #[doc(hidden)]
    pub fn scroll_2<R: RectTrait<'a>, S: RegionTrait<'a>>(
        &self,
        dx: i32,
        dy: i32,
        rect: &R,
        exposed: &S,
    ) -> &Self {
        let (obj_rect_3, _funcs) = rect.get_rect_obj_funcs();
        let (obj_exposed_4, _funcs) = exposed.get_region_obj_funcs();

        let (obj_data, funcs) = self.get_pixmap_obj_funcs();
        unsafe {
            ((*funcs).scroll_2)(obj_data, dx, dy, obj_rect_3, obj_exposed_4);
        }
        self
    }
    #[doc(hidden)]
    pub fn cache_key(&self) -> i64 {
        let (obj_data, funcs) = self.get_pixmap_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).cache_key)(obj_data);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn is_detached(&self) -> bool {
        let (obj_data, funcs) = self.get_pixmap_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_detached)(obj_data);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn detach(&self) -> &Self {
        let (obj_data, funcs) = self.get_pixmap_obj_funcs();
        unsafe {
            ((*funcs).detach)(obj_data);
        }
        self
    }
    #[doc(hidden)]
    pub fn is_q_bitmap(&self) -> bool {
        let (obj_data, funcs) = self.get_pixmap_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_q_bitmap)(obj_data);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn paint_engine(&self) -> Option<PaintEngine> {
        let (obj_data, funcs) = self.get_pixmap_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).paint_engine)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = PaintEngine::new_from_rc(t);
            } else {
                ret_val = PaintEngine::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    #[doc(hidden)]
    pub fn painting_active(&self) -> bool {
        let (obj_data, funcs) = self.get_paint_device_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).painting_active)(obj_data);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn logical_dpi_x(&self) -> i32 {
        let (obj_data, funcs) = self.get_paint_device_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).logical_dpi_x)(obj_data);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn logical_dpi_y(&self) -> i32 {
        let (obj_data, funcs) = self.get_paint_device_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).logical_dpi_y)(obj_data);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn physical_dpi_x(&self) -> i32 {
        let (obj_data, funcs) = self.get_paint_device_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).physical_dpi_x)(obj_data);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn physical_dpi_y(&self) -> i32 {
        let (obj_data, funcs) = self.get_paint_device_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).physical_dpi_y)(obj_data);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn device_pixel_ratio_f(&self) -> f32 {
        let (obj_data, funcs) = self.get_paint_device_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).device_pixel_ratio_f)(obj_data);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn color_count(&self) -> i32 {
        let (obj_data, funcs) = self.get_paint_device_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).color_count)(obj_data);
            ret_val
        }
    }

    pub fn build(&self) -> Self {
        self.clone()
    }
}
pub trait BitmapTrait<'a> {
    #[inline]
    #[doc(hidden)]
    fn get_bitmap_obj_funcs(&self) -> (*const RUBase, *const RUBitmapFuncs);
}

impl<'a> PaintDeviceTrait<'a> for Bitmap<'a> {
    #[doc(hidden)]
    fn get_paint_device_obj_funcs(&self) -> (*const RUBase, *const RUPaintDeviceFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).paint_device_funcs) }
    }
}

impl<'a> PixmapTrait<'a> for Bitmap<'a> {
    #[doc(hidden)]
    fn get_pixmap_obj_funcs(&self) -> (*const RUBase, *const RUPixmapFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).pixmap_funcs) }
    }
}

impl<'a> BitmapTrait<'a> for Bitmap<'a> {
    #[doc(hidden)]
    fn get_bitmap_obj_funcs(&self) -> (*const RUBase, *const RUBitmapFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).bitmap_funcs) }
    }
}
