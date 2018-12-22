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
/// A brush has a style, a color, a gradient and a texture.
///
/// The brush style() defines the fill pattern using the
/// Qt::BrushStyle enum. The default brush style is Qt::NoBrush
/// (depending on how you construct a brush). This style tells the
/// painter to not fill shapes. The standard style for filling is
/// Qt::SolidPattern. The style can be set when the brush is created
/// using the appropriate constructor, and in addition the setStyle()
/// function provides means for altering the style once the brush is
/// constructed.
///
/// ![Brush Styles](brush-styles.png)
///
/// The brush color() defines the color of the fill pattern. The color
/// can either be one of Qt's predefined colors, Qt::GlobalColor, or
/// any other custom QColor. The currently set color can be retrieved
/// and altered using the color() and setColor() functions,
/// respectively.
///
/// The gradient() defines the gradient fill used when the current
/// style is either Qt::LinearGradientPattern,
/// Qt::RadialGradientPattern or Qt::ConicalGradientPattern. Gradient
/// brushes are created by giving a QGradient as a constructor
/// argument when creating the QBrush. Qt provides three different
/// gradients: QLinearGradient, QConicalGradient, and QRadialGradient
/// - all of which inherit QGradient.
///
/// The texture() defines the pixmap used when the current style is
/// Qt::TexturePattern. You can create a brush with a texture by
/// providing the pixmap when the brush is created or by using
/// setTexture().
///
/// Note that applying setTexture() makes style() ==
/// Qt::TexturePattern, regardless of previous style
/// settings. Also, calling setColor() will not make a difference if
/// the style is a gradient. The same is the case if the style is
/// Qt::TexturePattern style unless the current texture is a QBitmap.
///
/// The isOpaque() function returns `true` if the brush is fully opaque
/// otherwise false. A brush is considered opaque if:
///
/// * The alpha component of the color() is 255.
/// * Its texture() does not have an alpha channel and is not a QBitmap.
/// * The colors in the gradient() all have an alpha component that is 255.
///
/// * ![Outlines](brush-outline.png)
///
/// * To specify the style and color of lines and outlines, use the QPainter's [pen](QPen)
/// combined with Qt::PenStyle and Qt::GlobalColor: Note that, by default, QPainter renders the outline (using the currently set pen) when drawing shapes. Use [\c
/// painter.setPen(Qt::NoPen)](Qt::NoPen)
/// to disable this behavior.
///
/// For more information about painting in general, see the [Paint
/// System](Paint%0A%20%20%20%20System)
///
///
/// **See also:** [`t::brush_style()`]
/// [`Painter`]
/// [`Color`]
/// # Licence
///
/// The documentation is an adoption of the original [Qt Documentation](http://doc.qt.io/) and provided herein is licensed under the terms of the [GNU Free Documentation License version 1.3](http://www.gnu.org/licenses/fdl.html) as published by the Free Software Foundation.
#[derive(Clone)]
pub struct Brush<'a> {
    #[doc(hidden)]
    pub data: Rc<Cell<Option<*const RUBase>>>,
    #[doc(hidden)]
    pub all_funcs: *const RUBrushAllFuncs,
    #[doc(hidden)]
    pub owned: bool,
    #[doc(hidden)]
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> Brush<'a> {
    pub fn new() -> Brush<'a> {
        let data = Rc::new(Cell::new(None));

        let ffi_data = unsafe {
            ((*rute_ffi_get()).create_brush)(
                ::std::ptr::null(),
                transmute(rute_object_delete_callback as usize),
                Rc::into_raw(data.clone()) as *const c_void,
            )
        };

        data.set(Some(ffi_data.qt_data));

        Brush {
            data,
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }
    #[allow(dead_code)]
    pub(crate) fn new_from_rc(ffi_data: RUBrush) -> Brush<'a> {
        Brush {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_owned(ffi_data: RUBrush) -> Brush<'a> {
        Brush {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_temporary(ffi_data: RUBrush) -> Brush<'a> {
        Brush {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }
    ///
    /// Swaps brush *other* with this brush. This operation is very
    /// fast and never fails.
    pub fn swap<B: BrushTrait<'a>>(&self, other: &B) -> &Self {
        let (obj_other_1, _funcs) = other.get_brush_obj_funcs();

        let (obj_data, funcs) = self.get_brush_obj_funcs();
        unsafe {
            ((*funcs).swap)(obj_data, obj_other_1);
        }
        self
    }
    ///
    /// Returns the brush style.
    ///
    /// **See also:** [`set_style()`]
    pub fn style(&self) -> BrushStyle {
        let (obj_data, funcs) = self.get_brush_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).style)(obj_data);
            let ret_val = { transmute::<u32, BrushStyle>(ret_val) };
            ret_val
        }
    }
    ///
    /// Sets the brush style to *style.*
    ///
    /// **See also:** [`style()`]
    pub fn set_style(&self, arg0: BrushStyle) -> &Self {
        let enum_arg0_1 = arg0 as u32;

        let (obj_data, funcs) = self.get_brush_obj_funcs();
        unsafe {
            ((*funcs).set_style)(obj_data, enum_arg0_1);
        }
        self
    }
    ///
    /// Sets *matrix* as an explicit transformation matrix on the
    /// current brush. The brush transformation matrix is merged with
    /// QPainter transformation matrix to produce the final result.
    ///
    /// **See also:** [`transform()`]
    pub fn set_transform<T: TransformTrait<'a>>(&self, arg0: &T) -> &Self {
        let (obj_arg0_1, _funcs) = arg0.get_transform_obj_funcs();

        let (obj_data, funcs) = self.get_brush_obj_funcs();
        unsafe {
            ((*funcs).set_transform)(obj_data, obj_arg0_1);
        }
        self
    }
    ///
    /// Returns the custom brush pattern, or a null pixmap if no custom brush pattern
    /// has been set.
    ///
    /// **See also:** [`set_texture()`]
    ///
    /// Returns the custom brush pattern, or a null image if no custom
    /// brush pattern has been set.
    ///
    /// If the texture was set as a QPixmap it will be converted to a
    /// QImage.
    ///
    /// **See also:** [`set_texture_image()`]
    pub fn texture(&self) -> Pixmap {
        let (obj_data, funcs) = self.get_brush_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).texture)(obj_data);
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
    /// Sets the brush pixmap to *pixmap.* The style is set to
    /// Qt::TexturePattern.
    ///
    /// The current brush color will only have an effect for monochrome
    /// pixmaps, i.e. for QPixmap::depth() == 1 ( [QBitmaps](QBitmap)
    /// ).
    ///
    /// **See also:** [`texture()`]
    ///
    /// Sets the brush image to *image.* The style is set to
    /// Qt::TexturePattern.
    ///
    /// Note the current brush color will *not* have any affect on
    /// monochrome images, as opposed to calling setTexture() with a
    /// QBitmap. If you want to change the color of monochrome image
    /// brushes, either convert the image to QBitmap with `QBitmap::fromImage()` and set the resulting QBitmap as a texture,
    /// or change the entries in the color table for the image.
    ///
    /// **See also:** [`texture_image()`]
    /// [`set_texture()`]
    pub fn set_texture<P: PixmapTrait<'a>>(&self, pixmap: &P) -> &Self {
        let (obj_pixmap_1, _funcs) = pixmap.get_pixmap_obj_funcs();

        let (obj_data, funcs) = self.get_brush_obj_funcs();
        unsafe {
            ((*funcs).set_texture)(obj_data, obj_pixmap_1);
        }
        self
    }
    ///
    /// Returns the custom brush pattern, or a null image if no custom
    /// brush pattern has been set.
    ///
    /// If the texture was set as a QPixmap it will be converted to a
    /// QImage.
    ///
    /// **See also:** [`set_texture_image()`]
    pub fn texture_image(&self) -> Image {
        let (obj_data, funcs) = self.get_brush_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).texture_image)(obj_data);
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
    ///
    /// Sets the brush image to *image.* The style is set to
    /// Qt::TexturePattern.
    ///
    /// Note the current brush color will *not* have any affect on
    /// monochrome images, as opposed to calling setTexture() with a
    /// QBitmap. If you want to change the color of monochrome image
    /// brushes, either convert the image to QBitmap with `QBitmap::fromImage()` and set the resulting QBitmap as a texture,
    /// or change the entries in the color table for the image.
    ///
    /// **See also:** [`texture_image()`]
    /// [`set_texture()`]
    pub fn set_texture_image<I: ImageTrait<'a>>(&self, image: &I) -> &Self {
        let (obj_image_1, _funcs) = image.get_image_obj_funcs();

        let (obj_data, funcs) = self.get_brush_obj_funcs();
        unsafe {
            ((*funcs).set_texture_image)(obj_data, obj_image_1);
        }
        self
    }
    ///
    /// Returns the brush color.
    ///
    /// **See also:** [`set_color()`]
    pub fn color(&self) -> Option<Color> {
        let (obj_data, funcs) = self.get_brush_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).color)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Color::new_from_rc(t);
            } else {
                ret_val = Color::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    ///
    /// Sets the brush color to the given *color.*
    ///
    /// Note that calling setColor() will not make a difference if the
    /// style is a gradient. The same is the case if the style is
    /// Qt::TexturePattern style unless the current texture is a QBitmap.
    ///
    /// **See also:** [`color()`]
    ///
    /// **Overloads**
    /// Sets the brush color to the given *color.*
    pub fn set_color<C: ColorTrait<'a>>(&self, color: &C) -> &Self {
        let (obj_color_1, _funcs) = color.get_color_obj_funcs();

        let (obj_data, funcs) = self.get_brush_obj_funcs();
        unsafe {
            ((*funcs).set_color)(obj_data, obj_color_1);
        }
        self
    }
    ///
    /// Sets the brush color to the given *color.*
    ///
    /// Note that calling setColor() will not make a difference if the
    /// style is a gradient. The same is the case if the style is
    /// Qt::TexturePattern style unless the current texture is a QBitmap.
    ///
    /// **See also:** [`color()`]
    ///
    /// **Overloads**
    /// Sets the brush color to the given *color.*
    pub fn set_color_2(&self, color: GlobalColor) -> &Self {
        let enum_color_1 = color as u32;

        let (obj_data, funcs) = self.get_brush_obj_funcs();
        unsafe {
            ((*funcs).set_color_2)(obj_data, enum_color_1);
        }
        self
    }
    ///
    /// Returns `true` if the brush is fully opaque otherwise false. A brush
    /// is considered opaque if:
    ///
    /// * The alpha component of the color() is 255.
    /// * Its texture() does not have an alpha channel and is not a QBitmap.
    /// * The colors in the gradient() all have an alpha component that is 255.
    /// * It is an extended radial gradient.
    pub fn is_opaque(&self) -> bool {
        let (obj_data, funcs) = self.get_brush_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_opaque)(obj_data);
            ret_val
        }
    }
    pub fn is_detached(&self) -> bool {
        let (obj_data, funcs) = self.get_brush_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_detached)(obj_data);
            ret_val
        }
    }

    pub fn build(&self) -> Self {
        self.clone()
    }
}
pub trait BrushTrait<'a> {
    #[inline]
    #[doc(hidden)]
    fn get_brush_obj_funcs(&self) -> (*const RUBase, *const RUBrushFuncs);
}

impl<'a> BrushTrait<'a> for Brush<'a> {
    #[doc(hidden)]
    fn get_brush_obj_funcs(&self) -> (*const RUBase, *const RUBrushFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).brush_funcs) }
    }
}
