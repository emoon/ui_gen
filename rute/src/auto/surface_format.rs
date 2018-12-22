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
/// The format includes the size of the color buffers, red, green, and blue;
/// the size of the alpha buffer; the size of the depth and stencil buffers;
/// and number of samples per pixel for multisampling. In addition, the format
/// contains surface configuration parameters such as OpenGL profile and
/// version for rendering, whether or not to enable stereo buffers, and swap
/// behaviour.
///
/// **Note**: When troubleshooting context or window format issues, it can be
/// helpful to enable the logging category `qt.qpa.gl` . Depending on the
/// platform, this may print useful debug information when it comes to OpenGL
/// initialization and the native visual or framebuffer configurations which
/// QSurfaceFormat gets mapped to.
/// # Licence
///
/// The documentation is an adoption of the original [Qt Documentation](http://doc.qt.io/) and provided herein is licensed under the terms of the [GNU Free Documentation License version 1.3](http://www.gnu.org/licenses/fdl.html) as published by the Free Software Foundation.
#[derive(Clone)]
pub struct SurfaceFormat<'a> {
    #[doc(hidden)]
    pub data: Rc<Cell<Option<*const RUBase>>>,
    #[doc(hidden)]
    pub all_funcs: *const RUSurfaceFormatAllFuncs,
    #[doc(hidden)]
    pub owned: bool,
    #[doc(hidden)]
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> SurfaceFormat<'a> {
    pub fn new() -> SurfaceFormat<'a> {
        let data = Rc::new(Cell::new(None));

        let ffi_data = unsafe {
            ((*rute_ffi_get()).create_surface_format)(
                ::std::ptr::null(),
                transmute(rute_object_delete_callback as usize),
                Rc::into_raw(data.clone()) as *const c_void,
            )
        };

        data.set(Some(ffi_data.qt_data));

        SurfaceFormat {
            data,
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }
    #[allow(dead_code)]
    pub(crate) fn new_from_rc(ffi_data: RUSurfaceFormat) -> SurfaceFormat<'a> {
        SurfaceFormat {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_owned(ffi_data: RUSurfaceFormat) -> SurfaceFormat<'a> {
        SurfaceFormat {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_temporary(ffi_data: RUSurfaceFormat) -> SurfaceFormat<'a> {
        SurfaceFormat {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }
    ///
    /// Set the minimum depth buffer size to *size.*
    ///
    /// **See also:** [`depth_buffer_size()`]
    pub fn set_depth_buffer_size(&self, size: i32) -> &Self {
        let (obj_data, funcs) = self.get_surface_format_obj_funcs();
        unsafe {
            ((*funcs).set_depth_buffer_size)(obj_data, size);
        }
        self
    }
    ///
    /// Returns the depth buffer size.
    ///
    /// **See also:** [`set_depth_buffer_size()`]
    pub fn depth_buffer_size(&self) -> i32 {
        let (obj_data, funcs) = self.get_surface_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).depth_buffer_size)(obj_data);
            ret_val
        }
    }
    ///
    /// Set the preferred stencil buffer size to *size* bits.
    ///
    /// **See also:** [`stencil_buffer_size()`]
    pub fn set_stencil_buffer_size(&self, size: i32) -> &Self {
        let (obj_data, funcs) = self.get_surface_format_obj_funcs();
        unsafe {
            ((*funcs).set_stencil_buffer_size)(obj_data, size);
        }
        self
    }
    ///
    /// Returns the stencil buffer size in bits.
    ///
    /// **See also:** [`set_stencil_buffer_size()`]
    pub fn stencil_buffer_size(&self) -> i32 {
        let (obj_data, funcs) = self.get_surface_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).stencil_buffer_size)(obj_data);
            ret_val
        }
    }
    ///
    /// Set the desired *size* in bits of the red channel of the color buffer.
    ///
    /// **Note**: On Mac OSX, be sure to set the buffer size of all color channels,
    /// otherwise this setting will have no effect. If one of the buffer sizes is not set,
    /// the current bit-depth of the screen is used.
    pub fn set_red_buffer_size(&self, size: i32) -> &Self {
        let (obj_data, funcs) = self.get_surface_format_obj_funcs();
        unsafe {
            ((*funcs).set_red_buffer_size)(obj_data, size);
        }
        self
    }
    ///
    /// Get the size in bits of the red channel of the color buffer.
    pub fn red_buffer_size(&self) -> i32 {
        let (obj_data, funcs) = self.get_surface_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).red_buffer_size)(obj_data);
            ret_val
        }
    }
    ///
    /// Set the desired *size* in bits of the green channel of the color buffer.
    ///
    /// **Note**: On Mac OSX, be sure to set the buffer size of all color channels,
    /// otherwise this setting will have no effect. If one of the buffer sizes is not set,
    /// the current bit-depth of the screen is used.
    pub fn set_green_buffer_size(&self, size: i32) -> &Self {
        let (obj_data, funcs) = self.get_surface_format_obj_funcs();
        unsafe {
            ((*funcs).set_green_buffer_size)(obj_data, size);
        }
        self
    }
    ///
    /// Get the size in bits of the green channel of the color buffer.
    pub fn green_buffer_size(&self) -> i32 {
        let (obj_data, funcs) = self.get_surface_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).green_buffer_size)(obj_data);
            ret_val
        }
    }
    ///
    /// Set the desired *size* in bits of the blue channel of the color buffer.
    ///
    /// **Note**: On Mac OSX, be sure to set the buffer size of all color channels,
    /// otherwise this setting will have no effect. If one of the buffer sizes is not set,
    /// the current bit-depth of the screen is used.
    pub fn set_blue_buffer_size(&self, size: i32) -> &Self {
        let (obj_data, funcs) = self.get_surface_format_obj_funcs();
        unsafe {
            ((*funcs).set_blue_buffer_size)(obj_data, size);
        }
        self
    }
    ///
    /// Get the size in bits of the blue channel of the color buffer.
    pub fn blue_buffer_size(&self) -> i32 {
        let (obj_data, funcs) = self.get_surface_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).blue_buffer_size)(obj_data);
            ret_val
        }
    }
    ///
    /// Set the desired *size* in bits of the alpha channel of the color buffer.
    pub fn set_alpha_buffer_size(&self, size: i32) -> &Self {
        let (obj_data, funcs) = self.get_surface_format_obj_funcs();
        unsafe {
            ((*funcs).set_alpha_buffer_size)(obj_data, size);
        }
        self
    }
    ///
    /// Get the size in bits of the alpha channel of the color buffer.
    pub fn alpha_buffer_size(&self) -> i32 {
        let (obj_data, funcs) = self.get_surface_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).alpha_buffer_size)(obj_data);
            ret_val
        }
    }
    ///
    /// Set the preferred number of samples per pixel when multisampling
    /// is enabled to *numSamples.* By default, multisampling is disabled.
    ///
    /// **See also:** [`samples()`]
    pub fn set_samples(&self, num_samples: i32) -> &Self {
        let (obj_data, funcs) = self.get_surface_format_obj_funcs();
        unsafe {
            ((*funcs).set_samples)(obj_data, num_samples);
        }
        self
    }
    ///
    /// Returns the number of samples per pixel when multisampling is
    /// enabled. By default, multisampling is disabled.
    ///
    /// **See also:** [`set_samples()`]
    pub fn samples(&self) -> i32 {
        let (obj_data, funcs) = self.get_surface_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).samples)(obj_data);
            ret_val
        }
    }
    ///
    /// Set the swap *behavior* of the surface.
    ///
    /// The swap behavior specifies whether single, double, or triple
    /// buffering is desired. The default, DefaultSwapBehavior,
    /// gives the default swap behavior of the platform.
    pub fn set_swap_behavior(&self, behavior: SwapBehavior) -> &Self {
        let enum_behavior_1 = behavior as u32;

        let (obj_data, funcs) = self.get_surface_format_obj_funcs();
        unsafe {
            ((*funcs).set_swap_behavior)(obj_data, enum_behavior_1);
        }
        self
    }
    ///
    /// Returns the configured swap behaviour.
    ///
    /// **See also:** [`set_swap_behavior()`]
    pub fn swap_behavior(&self) -> SwapBehavior {
        let (obj_data, funcs) = self.get_surface_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).swap_behavior)(obj_data);
            let ret_val = { transmute::<u32, SwapBehavior>(ret_val) };
            ret_val
        }
    }
    ///
    /// Returns `true` if the alpha buffer size is greater than zero.
    ///
    /// This means that the surface might be used with per pixel
    /// translucency effects.
    pub fn has_alpha(&self) -> bool {
        let (obj_data, funcs) = self.get_surface_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).has_alpha)(obj_data);
            ret_val
        }
    }
    ///
    /// Sets the desired OpenGL context *profile.*
    ///
    /// This setting is ignored if the requested OpenGL version is
    /// less than 3.2.
    pub fn set_profile(&self, profile: OpenGLContextProfile) -> &Self {
        let enum_profile_1 = profile as u32;

        let (obj_data, funcs) = self.get_surface_format_obj_funcs();
        unsafe {
            ((*funcs).set_profile)(obj_data, enum_profile_1);
        }
        self
    }
    ///
    /// Get the configured OpenGL context profile.
    ///
    /// This setting is ignored if the requested OpenGL version is
    /// less than 3.2.
    pub fn profile(&self) -> OpenGLContextProfile {
        let (obj_data, funcs) = self.get_surface_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).profile)(obj_data);
            let ret_val = { transmute::<u32, OpenGLContextProfile>(ret_val) };
            ret_val
        }
    }
    ///
    /// Sets the desired renderable *type.*
    ///
    /// Chooses between desktop OpenGL, OpenGL ES, and OpenVG.
    pub fn set_renderable_type(&self, stype: RenderableType) -> &Self {
        let enum_stype_1 = stype.bits();

        let (obj_data, funcs) = self.get_surface_format_obj_funcs();
        unsafe {
            ((*funcs).set_renderable_type)(obj_data, enum_stype_1);
        }
        self
    }
    ///
    /// Gets the renderable type.
    ///
    /// Chooses between desktop OpenGL, OpenGL ES, and OpenVG.
    pub fn renderable_type(&self) -> RenderableType {
        let (obj_data, funcs) = self.get_surface_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).renderable_type)(obj_data);
            let ret_val = RenderableType::from_bits_truncate(ret_val);
            ret_val
        }
    }
    ///
    /// Sets the desired *major* OpenGL version.
    pub fn set_major_version(&self, major_version: i32) -> &Self {
        let (obj_data, funcs) = self.get_surface_format_obj_funcs();
        unsafe {
            ((*funcs).set_major_version)(obj_data, major_version);
        }
        self
    }
    ///
    /// Returns the major OpenGL version.
    ///
    /// The default version is 2.0.
    pub fn major_version(&self) -> i32 {
        let (obj_data, funcs) = self.get_surface_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).major_version)(obj_data);
            ret_val
        }
    }
    ///
    /// Sets the desired *minor* OpenGL version.
    ///
    /// The default version is 2.0.
    pub fn set_minor_version(&self, minor_version: i32) -> &Self {
        let (obj_data, funcs) = self.get_surface_format_obj_funcs();
        unsafe {
            ((*funcs).set_minor_version)(obj_data, minor_version);
        }
        self
    }
    ///
    /// Returns the minor OpenGL version.
    pub fn minor_version(&self) -> i32 {
        let (obj_data, funcs) = self.get_surface_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).minor_version)(obj_data);
            ret_val
        }
    }
    ///
    /// Sets the desired *major* and *minor* OpenGL versions.
    ///
    /// The default version is 2.0.
    pub fn set_version(&self, major: i32, minor: i32) -> &Self {
        let (obj_data, funcs) = self.get_surface_format_obj_funcs();
        unsafe {
            ((*funcs).set_version)(obj_data, major, minor);
        }
        self
    }
    ///
    /// Returns `true` if stereo buffering is enabled; otherwise returns
    /// false. Stereo buffering is disabled by default.
    ///
    /// **See also:** [`set_stereo()`]
    pub fn stereo(&self) -> bool {
        let (obj_data, funcs) = self.get_surface_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).stereo)(obj_data);
            ret_val
        }
    }
    ///
    /// If *enable* is true enables stereo buffering; otherwise disables
    /// stereo buffering.
    ///
    /// Stereo buffering is disabled by default.
    ///
    /// Stereo buffering provides extra color buffers to generate left-eye
    /// and right-eye images.
    ///
    /// **See also:** [`stereo()`]
    pub fn set_stereo(&self, enable: bool) -> &Self {
        let (obj_data, funcs) = self.get_surface_format_obj_funcs();
        unsafe {
            ((*funcs).set_stereo)(obj_data, enable);
        }
        self
    }
    ///
    /// **Overloads**
    /// Use setOption(QSurfaceFormat::FormatOption, bool) or setOptions() instead.
    ///
    /// Sets the format options to the OR combination of *opt* and the
    /// current format options.
    ///
    /// **See also:** [`options()`]
    /// [`test_option()`]
    ///
    /// Sets the format options to *options.*
    ///
    /// **See also:** [`options()`]
    /// [`test_option()`]
    ///
    /// Sets the format option *option* if *on* is true; otherwise, clears the option.
    ///
    /// **See also:** [`set_options()`]
    /// [`options()`]
    /// [`test_option()`]
    pub fn set_option(&self, opt: FormatOptions) -> &Self {
        let enum_opt_1 = opt.bits();

        let (obj_data, funcs) = self.get_surface_format_obj_funcs();
        unsafe {
            ((*funcs).set_option)(obj_data, enum_opt_1);
        }
        self
    }
    ///
    /// **Overloads**
    /// Use testOption(QSurfaceFormat::FormatOption) instead.
    ///
    /// Returns `true` if any of the options in *opt* is currently set
    /// on this object; otherwise returns false.
    ///
    /// **See also:** [`set_option()`]
    ///
    /// Returns true if the format option *option* is set; otherwise returns false.
    ///
    /// **See also:** [`options()`]
    pub fn test_option(&self, opt: FormatOptions) -> bool {
        let enum_opt_1 = opt.bits();

        let (obj_data, funcs) = self.get_surface_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).test_option)(obj_data, enum_opt_1);
            ret_val
        }
    }
    ///
    /// Sets the format options to *options.*
    ///
    /// **See also:** [`options()`]
    /// [`test_option()`]
    pub fn set_options(&self, options: FormatOptions) -> &Self {
        let enum_options_1 = options.bits();

        let (obj_data, funcs) = self.get_surface_format_obj_funcs();
        unsafe {
            ((*funcs).set_options)(obj_data, enum_options_1);
        }
        self
    }
    ///
    /// **Overloads**
    /// Use setOption(QSurfaceFormat::FormatOption, bool) or setOptions() instead.
    ///
    /// Sets the format options to the OR combination of *opt* and the
    /// current format options.
    ///
    /// **See also:** [`options()`]
    /// [`test_option()`]
    ///
    /// Sets the format options to *options.*
    ///
    /// **See also:** [`options()`]
    /// [`test_option()`]
    ///
    /// Sets the format option *option* if *on* is true; otherwise, clears the option.
    ///
    /// **See also:** [`set_options()`]
    /// [`options()`]
    /// [`test_option()`]
    pub fn set_option_2(&self, option: FormatOption, on: bool) -> &Self {
        let enum_option_1 = option.bits();

        let (obj_data, funcs) = self.get_surface_format_obj_funcs();
        unsafe {
            ((*funcs).set_option_2)(obj_data, enum_option_1, on);
        }
        self
    }
    ///
    /// **Overloads**
    /// Use testOption(QSurfaceFormat::FormatOption) instead.
    ///
    /// Returns `true` if any of the options in *opt* is currently set
    /// on this object; otherwise returns false.
    ///
    /// **See also:** [`set_option()`]
    ///
    /// Returns true if the format option *option* is set; otherwise returns false.
    ///
    /// **See also:** [`options()`]
    pub fn test_option_2(&self, option: FormatOption) -> bool {
        let enum_option_1 = option.bits();

        let (obj_data, funcs) = self.get_surface_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).test_option_2)(obj_data, enum_option_1);
            ret_val
        }
    }
    ///
    /// Returns the currently set format options.
    ///
    /// **See also:** [`set_option()`]
    /// [`set_options()`]
    /// [`test_option()`]
    pub fn options(&self) -> FormatOptions {
        let (obj_data, funcs) = self.get_surface_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).options)(obj_data);
            let ret_val = FormatOptions::from_bits_truncate(ret_val);
            ret_val
        }
    }
    ///
    /// Returns the swap interval.
    ///
    /// **See also:** [`set_swap_interval()`]
    pub fn swap_interval(&self) -> i32 {
        let (obj_data, funcs) = self.get_surface_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).swap_interval)(obj_data);
            ret_val
        }
    }
    ///
    /// Sets the preferred swap interval. The swap interval specifies the
    /// minimum number of video frames that are displayed before a buffer
    /// swap occurs. This can be used to sync the GL drawing into a window
    /// to the vertical refresh of the screen.
    ///
    /// Setting an *interval* value of 0 will turn the vertical refresh
    /// syncing off, any value higher than 0 will turn the vertical
    /// syncing on. Setting *interval* to a higher value, for example 10,
    /// results in having 10 vertical retraces between every buffer swap.
    ///
    /// The default interval is 1.
    ///
    /// Changing the swap interval may not be supported by the underlying
    /// platform. In this case, the request will be silently ignored.
    ///
    /// **See also:** [`swap_interval()`]
    pub fn set_swap_interval(&self, interval: i32) -> &Self {
        let (obj_data, funcs) = self.get_surface_format_obj_funcs();
        unsafe {
            ((*funcs).set_swap_interval)(obj_data, interval);
        }
        self
    }
    ///
    /// **Returns** the color space.
    ///
    /// **See also:** [`set_color_space()`]
    pub fn color_space(&self) -> ColorSpace {
        let (obj_data, funcs) = self.get_surface_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).color_space)(obj_data);
            let ret_val = { transmute::<u32, ColorSpace>(ret_val) };
            ret_val
        }
    }
    ///
    /// Sets the preferred *colorSpace.*
    ///
    /// For example, this allows requesting windows with default framebuffers that
    /// are sRGB-capable on platforms that support it.
    ///
    /// **Note**: When the requested color space is not supported by the platform, the
    /// request is ignored. Query the QSurfaceFormat after window creation to verify
    /// if the color space request could be honored or not.
    ///
    /// **Note**: This setting controls if the default framebuffer of the window is
    /// capable of updates and blending in a given color space. It does not change
    /// applications' output by itself. The applications' rendering code will still
    /// have to opt in via the appropriate OpenGL calls to enable updates and
    /// blending to be performed in the given color space instead of using the
    /// standard linear operations.
    ///
    /// **See also:** [`color_space()`]
    pub fn set_color_space(&self, color_space: ColorSpace) -> &Self {
        let enum_color_space_1 = color_space as u32;

        let (obj_data, funcs) = self.get_surface_format_obj_funcs();
        unsafe {
            ((*funcs).set_color_space)(obj_data, enum_color_space_1);
        }
        self
    }
    ///
    /// Sets the global default surface *format.*
    ///
    /// This format is used by default in QOpenGLContext, QWindow, QOpenGLWidget and
    /// similar classes.
    ///
    /// It can always be overridden on a per-instance basis by using the class in
    /// question's own setFormat() function. However, it is often more convenient to
    /// set the format for all windows once at the start of the application. It also
    /// guarantees proper behavior in cases where shared contexts are required,
    /// because settings the format via this function guarantees that all contexts
    /// and surfaces, even the ones created internally by Qt, will use the same
    /// format.
    ///
    /// **Note**: When setting Qt::AA_ShareOpenGLContexts, it is strongly recommended to
    /// place the call to this function before the construction of the
    /// QGuiApplication or QApplication. Otherwise *format* will not be applied to
    /// the global share context and therefore issues may arise with context sharing
    /// afterwards.
    ///
    /// **See also:** [`default_format()`]
    pub fn set_default_format<S: SurfaceFormatTrait<'a>>(format: &S) {
        let (obj_format_1, _funcs) = format.get_surface_format_obj_funcs();

        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_surface_format)(::std::ptr::null()).all_funcs)
                    .surface_format_funcs,
            )
        };
        unsafe {
            ((*funcs).set_default_format)(obj_data, obj_format_1);
        }
    }
    ///
    /// Returns the global default surface format.
    ///
    /// When setDefaultFormat() is not called, this is a default-constructed QSurfaceFormat.
    ///
    /// **See also:** [`set_default_format()`]
    pub fn default_format() -> SurfaceFormat<'a> {
        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_surface_format)(::std::ptr::null()).all_funcs)
                    .surface_format_funcs,
            )
        };
        unsafe {
            let ret_val = ((*funcs).default_format)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = SurfaceFormat::new_from_rc(t);
            } else {
                ret_val = SurfaceFormat::new_from_owned(t);
            }
            ret_val
        }
    }

    pub fn build(&self) -> Self {
        self.clone()
    }
}
pub trait SurfaceFormatTrait<'a> {
    #[inline]
    #[doc(hidden)]
    fn get_surface_format_obj_funcs(&self) -> (*const RUBase, *const RUSurfaceFormatFuncs);
}

impl<'a> SurfaceFormatTrait<'a> for SurfaceFormat<'a> {
    #[doc(hidden)]
    fn get_surface_format_obj_funcs(&self) -> (*const RUBase, *const RUSurfaceFormatFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).surface_format_funcs) }
    }
}
bitflags! {
    pub struct FormatOption: u32 {
        const StereoBuffers = 0x1;
        const DebugContext = 0x2;
        const DeprecatedFunctions = 0x4;
        const ResetNotification = 0x8;
    }
}

pub type FormatOptions = FormatOption;

#[repr(u32)]
pub enum SwapBehavior {
    DefaultSwapBehavior = 0,
    SingleBuffer = 1,
    DoubleBuffer = 2,
    TripleBuffer = 3,
}

bitflags! {
    pub struct RenderableType: u32 {
        const DefaultRenderableType = 0x0;
        const OpenGl = 0x1;
        const OpenGles = 0x2;
        const OpenVg = 0x4;
    }
}

#[repr(u32)]
pub enum OpenGLContextProfile {
    NoProfile = 0,
    CoreProfile = 1,
    CompatibilityProfile = 2,
}

#[repr(u32)]
pub enum ColorSpace {
    DefaultColorSpace = 0,
    SRgbColorSpace = 1,
}
