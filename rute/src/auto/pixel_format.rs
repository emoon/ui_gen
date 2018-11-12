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
/// In Qt there is a often a need to represent the layout of the pixels in a
/// graphics buffer. Internally QPixelFormat stores everything in a 64 bit
/// datastructure. This gives performance but also some limitations.
///
/// QPixelFormat can describe 5 color channels and 1 alpha channel, each can use
/// 6 bits to describe the size of the color channel.
///
/// The position of the alpha channel is described with a separate enum. This is
/// to make it possible to describe QImage formats like ARGB32, and also
/// describe typical OpenGL formats like RBGA8888.
///
/// How pixels are suppose to be read is determined by the TypeInterpretation
/// enum. It describes if color values are suppose to be read byte per byte,
/// or if a pixel is suppose to be read as a complete int and then masked.
/// **See also:** TypeInterpretation
///
/// There is no support for describing YUV's macro pixels. Instead a list of YUV
/// formats has been made. When a QPixelFormat is describing a YUV format, the
/// bitsPerPixel value has been deduced by the YUV Layout enum. Also, the color
/// channels should all be set to zero except the fifth color channel that
/// should store the bitsPerPixel value.
/// # Licence
///
/// The documentation is an adoption of the original [Qt Documentation](http://doc.qt.io/) and provided herein is licensed under the terms of the [GNU Free Documentation License version 1.3](http://www.gnu.org/licenses/fdl.html) as published by the Free Software Foundation.
#[derive(Clone)]
pub struct PixelFormat<'a> {
    pub data: Rc<Cell<Option<*const RUBase>>>,
    pub all_funcs: *const RUPixelFormatAllFuncs,
    pub owned: bool,
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> PixelFormat<'a> {
    pub fn new() -> PixelFormat<'a> {
        let data = Rc::new(Cell::new(None));

        let ffi_data = unsafe {
            ((*rute_ffi_get()).create_pixel_format)(
                ::std::ptr::null(),
                transmute(rute_object_delete_callback as usize),
                Rc::into_raw(data.clone()) as *const c_void,
            )
        };

        data.set(Some(ffi_data.qt_data));

        PixelFormat {
            data,
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }
    pub fn new_from_rc(ffi_data: RUPixelFormat) -> PixelFormat<'a> {
        PixelFormat {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    pub fn new_from_owned(ffi_data: RUPixelFormat) -> PixelFormat<'a> {
        PixelFormat {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    pub fn new_from_temporary(ffi_data: RUPixelFormat) -> PixelFormat<'a> {
        PixelFormat {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }
}

pub struct PixelFormatStatic<'a> {
    pub all_funcs: *const RUPixelFormatAllFuncs,
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}
pub trait PixelFormatType<'a> {
    fn get(&self, offset: Field, width: FieldWidth) -> u8 {
        let enum_offset_1 = offset as i32;
        let enum_width_2 = width as i32;

        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).get)(obj_data, enum_offset_1, enum_width_2);
            ret_val
        }
    }
    ///
    /// Accessor function for getting the colorModel.
    fn color_model(&self) -> ColorModel {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).color_model)(obj_data);
            let ret_val = { transmute::<i32, ColorModel>(ret_val) };
            ret_val
        }
    }
    ///
    /// Accessor function for getting the channelCount. Channel Count is deduced
    /// by color channels with a size > 0 and if the size of the alpha channel is > 0.
    fn channel_count(&self) -> u8 {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).channel_count)(obj_data);
            ret_val
        }
    }
    ///
    /// Accessor function for the size of the red color channel.
    fn red_size(&self) -> u8 {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).red_size)(obj_data);
            ret_val
        }
    }
    ///
    /// Accessor function for the size of the green color channel.
    fn green_size(&self) -> u8 {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).green_size)(obj_data);
            ret_val
        }
    }
    ///
    /// Accessor function for the size of the blue color channel.
    fn blue_size(&self) -> u8 {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).blue_size)(obj_data);
            ret_val
        }
    }
    ///
    /// Accessor function for the cyan color channel.
    fn cyan_size(&self) -> u8 {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).cyan_size)(obj_data);
            ret_val
        }
    }
    ///
    /// Accessor function for the megenta color channel.
    fn magenta_size(&self) -> u8 {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).magenta_size)(obj_data);
            ret_val
        }
    }
    ///
    /// Accessor function for the yellow color channel.
    fn yellow_size(&self) -> u8 {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).yellow_size)(obj_data);
            ret_val
        }
    }
    ///
    /// Accessor function for the black/key color channel.
    fn black_size(&self) -> u8 {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).black_size)(obj_data);
            ret_val
        }
    }
    ///
    /// Accessor function for the hue channel size.
    fn hue_size(&self) -> u8 {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).hue_size)(obj_data);
            ret_val
        }
    }
    ///
    /// Accessor function for the saturation channel size.
    fn saturation_size(&self) -> u8 {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).saturation_size)(obj_data);
            ret_val
        }
    }
    ///
    /// Accessor function for the lightness channel size.
    fn lightness_size(&self) -> u8 {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).lightness_size)(obj_data);
            ret_val
        }
    }
    ///
    /// Accessor function for the brightness channel size.
    fn brightness_size(&self) -> u8 {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).brightness_size)(obj_data);
            ret_val
        }
    }
    ///
    /// Accessor function for the alpha channel size.
    fn alpha_size(&self) -> u8 {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).alpha_size)(obj_data);
            ret_val
        }
    }
    ///
    /// Accessor function for the bits used per pixel. This function returns the
    /// sum of the color channels + the size of the alpha channel.
    fn bits_per_pixel(&self) -> u8 {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).bits_per_pixel)(obj_data);
            ret_val
        }
    }
    ///
    /// Accessor function for alphaUsage.
    fn alpha_usage(&self) -> AlphaUsage {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).alpha_usage)(obj_data);
            let ret_val = { transmute::<i32, AlphaUsage>(ret_val) };
            ret_val
        }
    }
    ///
    /// Accessor function for alphaPosition.
    fn alpha_position(&self) -> AlphaPosition {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).alpha_position)(obj_data);
            let ret_val = { transmute::<i32, AlphaPosition>(ret_val) };
            ret_val
        }
    }
    ///
    /// Accessor function for the AlphaPremultiplied enum. This indicates if the
    /// alpha channel is multiplied in to the color channels.
    ///
    fn premultiplied(&self) -> AlphaPremultiplied {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).premultiplied)(obj_data);
            let ret_val = { transmute::<i32, AlphaPremultiplied>(ret_val) };
            ret_val
        }
    }
    ///
    /// Accessor function for the type representation of a color channel or a pixel.
    ///
    /// **See also:** TypeInterpretation
    fn type_interpretation(&self) -> TypeInterpretation {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).type_interpretation)(obj_data);
            let ret_val = { transmute::<i32, TypeInterpretation>(ret_val) };
            ret_val
        }
    }
    ///
    /// The byte order is almost always set the the byte order of the current
    /// system. However, it can be useful to describe some YUV formats. This
    /// function should never return QPixelFormat::CurrentSystemEndian as this
    /// value is translated to a endian value in the constructor.
    fn byte_order(&self) -> ByteOrder {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).byte_order)(obj_data);
            let ret_val = { transmute::<i32, ByteOrder>(ret_val) };
            ret_val
        }
    }
    ///
    /// Accessor function for the YUVLayout. It is difficult to describe the color
    /// channels of a YUV pixel format since YUV color model uses macro pixels.
    /// Instead the layout of the pixels are stored as an enum.
    fn yuv_layout(&self) -> YUVLayout {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).yuv_layout)(obj_data);
            let ret_val = { transmute::<i32, YUVLayout>(ret_val) };
            ret_val
        }
    }
    fn sub_enum(&self) -> u8 {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).sub_enum)(obj_data);
            ret_val
        }
    }

    #[inline]
    fn get_pixel_format_obj_funcs(&self) -> (*const RUBase, *const RUPixelFormatFuncs);
}

impl<'a> PixelFormatType<'a> for PixelFormat<'a> {
    #[inline]
    fn get_pixel_format_obj_funcs(&self) -> (*const RUBase, *const RUPixelFormatFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).pixel_format_funcs) }
    }
}
pub trait PixelFormatStaticType {
    fn set<'a>(offset: Field, width: FieldWidth, value: u8) -> u64 {
        let enum_offset_1 = offset as i32;
        let enum_width_2 = width as i32;

        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_pixel_format)(::std::ptr::null()).all_funcs)
                    .pixel_format_funcs,
            )
        };
        unsafe {
            let ret_val = ((*funcs).set)(obj_data, enum_offset_1, enum_width_2, value);
            ret_val
        }
    }
}

impl<'a> PixelFormatStaticType for PixelFormat<'a> {}

impl<'a> PixelFormatStaticType for PixelFormatStatic<'a> {}