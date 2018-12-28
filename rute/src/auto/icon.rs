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
/// A QIcon can generate smaller, larger, active, and disabled pixmaps
/// from the set of pixmaps it is given. Such pixmaps are used by Qt
/// widgets to show an icon representing a particular action.
///
/// The simplest use of QIcon is to create one from a QPixmap file or
/// resource, and then use it, allowing Qt to work out all the required
/// icon styles and sizes. For example:
///
/// To undo a QIcon, simply set a null icon in its place:
///
/// Use the QImageReader::supportedImageFormats() and
/// QImageWriter::supportedImageFormats() functions to retrieve a
/// complete list of the supported file formats.
///
/// When you retrieve a pixmap using pixmap(QSize, Mode, State), and no
/// pixmap for this given size, mode and state has been added with
/// addFile() or addPixmap(), then QIcon will generate one on the
/// fly. This pixmap generation happens in a QIconEngine. The default
/// engine scales pixmaps down if required, but never up, and it uses
/// the current style to calculate a disabled appearance. By using
/// custom icon engines, you can customize every aspect of generated
/// icons. With QIconEnginePlugin it is possible to register different
/// icon engines for different file suffixes, making it possible for
/// third parties to provide additional icon engines to those included
/// with Qt.
///
/// **Note**: Since Qt 4.2, an icon engine that supports SVG is included.
///
/// # Making Classes that Use QIcon
///
/// If you write your own widgets that have an option to set a small
/// pixmap, consider allowing a QIcon to be set for that pixmap. The
/// Qt class QToolButton is an example of such a widget.
///
/// Provide a method to set a QIcon, and when you draw the icon, choose
/// whichever pixmap is appropriate for the current state of your widget.
/// For example:
///
/// You might also make use of the `Active` mode, perhaps making your
/// widget `Active` when the mouse is over the widget (see [QWidget::enterEvent()),](QWidget::enterEvent()),)
/// while the mouse is pressed pending the
/// release that will activate the function, or when it is the currently
/// selected item. If the widget can be toggled, the mode might be
/// used to draw a different icon.
///
/// ![QIcon](icon.png)
///
/// **Note**: QIcon needs a QGuiApplication instance before the icon is created.
///
/// # High DPI Icons
///
/// There are two ways that QIcon supports [high DPI](High%20DPI%20Displays)
///
/// icons: via [addFile()](addFile())
/// and [fromTheme().](fromTheme().)
///
/// [addFile()](addFile())
/// is useful if you have your own custom directory structure and do
/// not need to use the [freedesktop.org Icon Theme
/// Specification](Icon%20Theme%20Specification)
/// . Icons created via this approach use Qt's ["@nx" high DPI syntax](High%20Resolution%0A%20%20Versions%20of%20Images)
///
///
/// Using [fromTheme()](fromTheme())
/// is necessary if you plan on following the Icon Theme
/// Specification. To make QIcon use the high DPI version of an image, add an
/// additional entry to the appropriate `index.theme` file:
///
/// Your icon theme directory would then look something like this:
///
/// **See also:** {fowler}{GUI Design Handbook: Iconic Label}
/// {Icons Example}
/// # Licence
///
/// The documentation is an adoption of the original [Qt Documentation](http://doc.qt.io/) and provided herein is licensed under the terms of the [GNU Free Documentation License version 1.3](http://www.gnu.org/licenses/fdl.html) as published by the Free Software Foundation.
#[derive(Clone)]
pub struct Icon<'a> {
    #[doc(hidden)]
    pub data: Rc<Cell<Option<*const RUBase>>>,
    #[doc(hidden)]
    pub all_funcs: *const RUIconAllFuncs,
    #[doc(hidden)]
    pub owned: bool,
    #[doc(hidden)]
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> Icon<'a> {
    pub fn new() -> Icon<'a> {
        let data = Rc::new(Cell::new(None));

        let ffi_data = unsafe {
            ((*rute_ffi_get()).create_icon)(
                ::std::ptr::null(),
                transmute(rute_object_delete_callback as usize),
                Rc::into_raw(data.clone()) as *const c_void,
            )
        };

        data.set(Some(ffi_data.qt_data));

        Icon {
            data,
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }
    #[allow(dead_code)]
    pub(crate) fn new_from_rc(ffi_data: RUIcon) -> Icon<'a> {
        Icon {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_owned(ffi_data: RUIcon) -> Icon<'a> {
        Icon {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_temporary(ffi_data: RUIcon) -> Icon<'a> {
        Icon {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }
    ///
    /// Swaps icon *other* with this icon. This operation is very
    /// fast and never fails.
    pub fn swap<I: IconTrait<'a>>(&self, other: &I) -> &Self {
        let (obj_other_1, _funcs) = other.get_icon_obj_funcs();

        let (obj_data, funcs) = self.get_icon_obj_funcs();
        unsafe {
            ((*funcs).swap)(obj_data, obj_other_1);
        }
        self
    }
    ///
    /// Returns a pixmap with the requested *size,* *mode,* and *state,* generating one if necessary. The pixmap might be smaller than
    /// requested, but never larger.
    ///
    /// Setting the Qt::AA_UseHighDpiPixmaps application attribute enables this
    /// function to return pixmaps that are larger than the requested size. Such
    /// images will have a devicePixelRatio larger than 1.
    ///
    /// **See also:** [`actual_size()`]
    /// [`paint()`]
    ///
    /// **Overloads**
    /// Returns a pixmap of size QSize( *w,* *h).* The pixmap might be smaller than
    /// requested, but never larger.
    ///
    /// Setting the Qt::AA_UseHighDpiPixmaps application attribute enables this
    /// function to return pixmaps that are larger than the requested size. Such
    /// images will have a devicePixelRatio larger than 1.
    ///
    /// **Overloads**
    /// Returns a pixmap of size QSize( *extent,* *extent).* The pixmap might be smaller
    /// than requested, but never larger.
    ///
    /// Setting the Qt::AA_UseHighDpiPixmaps application attribute enables this
    /// function to return pixmaps that are larger than the requested size. Such
    /// images will have a devicePixelRatio larger than 1.
    ///
    /// Returns a pixmap with the requested *window* *size,* *mode,* and *state,* generating one if necessary.
    ///
    /// The pixmap can be smaller than the requested size. If *window* is on
    /// a high-dpi display the pixmap can be larger. In that case it will have
    /// a devicePixelRatio larger than 1.
    ///
    /// **See also:** [`actual_size()`]
    /// [`paint()`]
    pub fn pixmap<S: SizeTrait<'a>>(&self, size: &S, mode: Mode, state: State) -> Pixmap {
        let (obj_size_1, _funcs) = size.get_size_obj_funcs();
        let enum_mode_2 = mode as u32;
        let enum_state_3 = state as u32;

        let (obj_data, funcs) = self.get_icon_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).pixmap)(obj_data, obj_size_1, enum_mode_2, enum_state_3);
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
    /// Returns a pixmap with the requested *size,* *mode,* and *state,* generating one if necessary. The pixmap might be smaller than
    /// requested, but never larger.
    ///
    /// Setting the Qt::AA_UseHighDpiPixmaps application attribute enables this
    /// function to return pixmaps that are larger than the requested size. Such
    /// images will have a devicePixelRatio larger than 1.
    ///
    /// **See also:** [`actual_size()`]
    /// [`paint()`]
    ///
    /// **Overloads**
    /// Returns a pixmap of size QSize( *w,* *h).* The pixmap might be smaller than
    /// requested, but never larger.
    ///
    /// Setting the Qt::AA_UseHighDpiPixmaps application attribute enables this
    /// function to return pixmaps that are larger than the requested size. Such
    /// images will have a devicePixelRatio larger than 1.
    ///
    /// **Overloads**
    /// Returns a pixmap of size QSize( *extent,* *extent).* The pixmap might be smaller
    /// than requested, but never larger.
    ///
    /// Setting the Qt::AA_UseHighDpiPixmaps application attribute enables this
    /// function to return pixmaps that are larger than the requested size. Such
    /// images will have a devicePixelRatio larger than 1.
    ///
    /// Returns a pixmap with the requested *window* *size,* *mode,* and *state,* generating one if necessary.
    ///
    /// The pixmap can be smaller than the requested size. If *window* is on
    /// a high-dpi display the pixmap can be larger. In that case it will have
    /// a devicePixelRatio larger than 1.
    ///
    /// **See also:** [`actual_size()`]
    /// [`paint()`]
    pub fn pixmap_2(&self, w: i32, h: i32, mode: Mode, state: State) -> Pixmap {
        let enum_mode_3 = mode as u32;
        let enum_state_4 = state as u32;

        let (obj_data, funcs) = self.get_icon_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).pixmap_2)(obj_data, w, h, enum_mode_3, enum_state_4);
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
    /// Returns a pixmap with the requested *size,* *mode,* and *state,* generating one if necessary. The pixmap might be smaller than
    /// requested, but never larger.
    ///
    /// Setting the Qt::AA_UseHighDpiPixmaps application attribute enables this
    /// function to return pixmaps that are larger than the requested size. Such
    /// images will have a devicePixelRatio larger than 1.
    ///
    /// **See also:** [`actual_size()`]
    /// [`paint()`]
    ///
    /// **Overloads**
    /// Returns a pixmap of size QSize( *w,* *h).* The pixmap might be smaller than
    /// requested, but never larger.
    ///
    /// Setting the Qt::AA_UseHighDpiPixmaps application attribute enables this
    /// function to return pixmaps that are larger than the requested size. Such
    /// images will have a devicePixelRatio larger than 1.
    ///
    /// **Overloads**
    /// Returns a pixmap of size QSize( *extent,* *extent).* The pixmap might be smaller
    /// than requested, but never larger.
    ///
    /// Setting the Qt::AA_UseHighDpiPixmaps application attribute enables this
    /// function to return pixmaps that are larger than the requested size. Such
    /// images will have a devicePixelRatio larger than 1.
    ///
    /// Returns a pixmap with the requested *window* *size,* *mode,* and *state,* generating one if necessary.
    ///
    /// The pixmap can be smaller than the requested size. If *window* is on
    /// a high-dpi display the pixmap can be larger. In that case it will have
    /// a devicePixelRatio larger than 1.
    ///
    /// **See also:** [`actual_size()`]
    /// [`paint()`]
    pub fn pixmap_3(&self, extent: i32, mode: Mode, state: State) -> Pixmap {
        let enum_mode_2 = mode as u32;
        let enum_state_3 = state as u32;

        let (obj_data, funcs) = self.get_icon_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).pixmap_3)(obj_data, extent, enum_mode_2, enum_state_3);
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
    /// Returns a pixmap with the requested *size,* *mode,* and *state,* generating one if necessary. The pixmap might be smaller than
    /// requested, but never larger.
    ///
    /// Setting the Qt::AA_UseHighDpiPixmaps application attribute enables this
    /// function to return pixmaps that are larger than the requested size. Such
    /// images will have a devicePixelRatio larger than 1.
    ///
    /// **See also:** [`actual_size()`]
    /// [`paint()`]
    ///
    /// **Overloads**
    /// Returns a pixmap of size QSize( *w,* *h).* The pixmap might be smaller than
    /// requested, but never larger.
    ///
    /// Setting the Qt::AA_UseHighDpiPixmaps application attribute enables this
    /// function to return pixmaps that are larger than the requested size. Such
    /// images will have a devicePixelRatio larger than 1.
    ///
    /// **Overloads**
    /// Returns a pixmap of size QSize( *extent,* *extent).* The pixmap might be smaller
    /// than requested, but never larger.
    ///
    /// Setting the Qt::AA_UseHighDpiPixmaps application attribute enables this
    /// function to return pixmaps that are larger than the requested size. Such
    /// images will have a devicePixelRatio larger than 1.
    ///
    /// Returns a pixmap with the requested *window* *size,* *mode,* and *state,* generating one if necessary.
    ///
    /// The pixmap can be smaller than the requested size. If *window* is on
    /// a high-dpi display the pixmap can be larger. In that case it will have
    /// a devicePixelRatio larger than 1.
    ///
    /// **See also:** [`actual_size()`]
    /// [`paint()`]
    pub fn pixmap_4<S: SizeTrait<'a>, W: WindowTrait<'a>>(
        &self,
        window: &W,
        size: &S,
        mode: Mode,
        state: State,
    ) -> Pixmap {
        let (obj_window_1, _funcs) = window.get_window_obj_funcs();
        let (obj_size_2, _funcs) = size.get_size_obj_funcs();
        let enum_mode_3 = mode as u32;
        let enum_state_4 = state as u32;

        let (obj_data, funcs) = self.get_icon_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).pixmap_4)(
                obj_data,
                obj_window_1,
                obj_size_2,
                enum_mode_3,
                enum_state_4,
            );
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
    /// Returns the actual size of the icon for the requested *size,* *mode,* and *state.* The result might be smaller than requested, but
    /// never larger. The returned size is in device-independent pixels (This
    /// is relevant for high-dpi pixmaps.)
    ///
    /// **See also:** [`pixmap()`]
    /// [`paint()`]
    ///
    /// Returns the actual size of the icon for the requested *window* *size,* *mode,* and *state.*
    ///
    /// The pixmap can be smaller than the requested size. The returned size
    /// is in device-independent pixels (This is relevant for high-dpi pixmaps.)
    ///
    /// **See also:** [`actual_size()`]
    /// [`pixmap()`]
    /// [`paint()`]
    pub fn actual_size<S: SizeTrait<'a>>(&self, size: &S, mode: Mode, state: State) -> Size {
        let (obj_size_1, _funcs) = size.get_size_obj_funcs();
        let enum_mode_2 = mode as u32;
        let enum_state_3 = state as u32;

        let (obj_data, funcs) = self.get_icon_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).actual_size)(obj_data, obj_size_1, enum_mode_2, enum_state_3);
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
    /// Returns the actual size of the icon for the requested *size,* *mode,* and *state.* The result might be smaller than requested, but
    /// never larger. The returned size is in device-independent pixels (This
    /// is relevant for high-dpi pixmaps.)
    ///
    /// **See also:** [`pixmap()`]
    /// [`paint()`]
    ///
    /// Returns the actual size of the icon for the requested *window* *size,* *mode,* and *state.*
    ///
    /// The pixmap can be smaller than the requested size. The returned size
    /// is in device-independent pixels (This is relevant for high-dpi pixmaps.)
    ///
    /// **See also:** [`actual_size()`]
    /// [`pixmap()`]
    /// [`paint()`]
    pub fn actual_size_2<S: SizeTrait<'a>, W: WindowTrait<'a>>(
        &self,
        window: &W,
        size: &S,
        mode: Mode,
        state: State,
    ) -> Size {
        let (obj_window_1, _funcs) = window.get_window_obj_funcs();
        let (obj_size_2, _funcs) = size.get_size_obj_funcs();
        let enum_mode_3 = mode as u32;
        let enum_state_4 = state as u32;

        let (obj_data, funcs) = self.get_icon_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).actual_size_2)(
                obj_data,
                obj_window_1,
                obj_size_2,
                enum_mode_3,
                enum_state_4,
            );
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
    ///
    /// Returns the name used to create the icon, if available.
    ///
    /// Depending on the way the icon was created, it may have an associated
    /// name. This is the case for icons created with fromTheme() or icons
    /// using a QIconEngine which supports the QIconEngine::IconNameHook.
    ///
    /// **See also:** [`from_theme()`]
    /// [`IconEngine`]
    pub fn name(&self) -> String {
        let (obj_data, funcs) = self.get_icon_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).name)(obj_data);
            let ret_val = CStr::from_ptr(ret_val).to_string_lossy().into_owned();
            ret_val
        }
    }
    ///
    /// Uses the *painter* to paint the icon with specified *alignment,*
    /// required *mode,* and *state* into the rectangle *rect.*
    ///
    /// **See also:** [`actual_size()`]
    /// [`pixmap()`]
    ///
    /// Mode mode, State state) const
    ///
    /// **Overloads**
    /// Paints the icon into the rectangle QRect( *x,* *y,* *w,* *h).*
    pub fn paint<P: PainterTrait<'a>, R: RectTrait<'a>>(
        &self,
        painter: &P,
        rect: &R,
        alignment: Alignment,
        mode: Mode,
        state: State,
    ) -> &Self {
        let (obj_painter_1, _funcs) = painter.get_painter_obj_funcs();
        let (obj_rect_2, _funcs) = rect.get_rect_obj_funcs();
        let enum_alignment_3 = alignment.bits();
        let enum_mode_4 = mode as u32;
        let enum_state_5 = state as u32;

        let (obj_data, funcs) = self.get_icon_obj_funcs();
        unsafe {
            ((*funcs).paint)(
                obj_data,
                obj_painter_1,
                obj_rect_2,
                enum_alignment_3,
                enum_mode_4,
                enum_state_5,
            );
        }
        self
    }
    ///
    /// Uses the *painter* to paint the icon with specified *alignment,*
    /// required *mode,* and *state* into the rectangle *rect.*
    ///
    /// **See also:** [`actual_size()`]
    /// [`pixmap()`]
    ///
    /// Mode mode, State state) const
    ///
    /// **Overloads**
    /// Paints the icon into the rectangle QRect( *x,* *y,* *w,* *h).*
    pub fn paint_2<P: PainterTrait<'a>>(
        &self,
        painter: &P,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        alignment: Alignment,
        mode: Mode,
        state: State,
    ) -> &Self {
        let (obj_painter_1, _funcs) = painter.get_painter_obj_funcs();
        let enum_alignment_6 = alignment.bits();
        let enum_mode_7 = mode as u32;
        let enum_state_8 = state as u32;

        let (obj_data, funcs) = self.get_icon_obj_funcs();
        unsafe {
            ((*funcs).paint_2)(
                obj_data,
                obj_painter_1,
                x,
                y,
                w,
                h,
                enum_alignment_6,
                enum_mode_7,
                enum_state_8,
            );
        }
        self
    }
    ///
    /// Returns `true` if the icon is empty; otherwise returns `false.`
    ///
    /// An icon is empty if it has neither a pixmap nor a filename.
    ///
    /// Note: Even a non-null icon might not be able to create valid
    /// pixmaps, eg. if the file does not exist or cannot be read.
    pub fn is_null(&self) -> bool {
        let (obj_data, funcs) = self.get_icon_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_null)(obj_data);
            ret_val
        }
    }
    pub fn is_detached(&self) -> bool {
        let (obj_data, funcs) = self.get_icon_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_detached)(obj_data);
            ret_val
        }
    }
    pub fn detach(&self) -> &Self {
        let (obj_data, funcs) = self.get_icon_obj_funcs();
        unsafe {
            ((*funcs).detach)(obj_data);
        }
        self
    }
    ///
    /// Returns a number that identifies the contents of this QIcon
    /// object. Distinct QIcon objects can have the same key if
    /// they refer to the same contents.
    ///
    /// The cacheKey() will change when the icon is altered via
    /// addPixmap() or addFile().
    ///
    /// Cache keys are mostly useful in conjunction with caching.
    ///
    /// **See also:** [`Pixmap::cache_key`]
    pub fn cache_key(&self) -> i64 {
        let (obj_data, funcs) = self.get_icon_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).cache_key)(obj_data);
            ret_val
        }
    }
    ///
    /// Adds *pixmap* to the icon, as a specialization for *mode* and
    /// *state.*
    ///
    /// Custom icon engines are free to ignore additionally added
    /// pixmaps.
    ///
    /// **See also:** [`add_file()`]
    pub fn add_pixmap<P: PixmapTrait<'a>>(&self, pixmap: &P, mode: Mode, state: State) -> &Self {
        let (obj_pixmap_1, _funcs) = pixmap.get_pixmap_obj_funcs();
        let enum_mode_2 = mode as u32;
        let enum_state_3 = state as u32;

        let (obj_data, funcs) = self.get_icon_obj_funcs();
        unsafe {
            ((*funcs).add_pixmap)(obj_data, obj_pixmap_1, enum_mode_2, enum_state_3);
        }
        self
    }
    /// Adds an image from the file with the given *fileName* to the
    /// icon, as a specialization for *size,* *mode* and *state.* The
    /// file will be loaded on demand. Note: custom icon engines are free
    /// to ignore additionally added pixmaps.
    ///
    /// If *fileName* contains a relative path (e.g. the filename only)
    /// the relevant file must be found relative to the runtime working
    /// directory.
    ///
    /// The file name can refer to an actual file on disk or to
    /// one of the application's embedded resources. See the
    /// [Resource System](resources.html)
    /// overview for details on how to
    /// embed images and other resource files in the application's
    /// executable.
    ///
    /// Use the QImageReader::supportedImageFormats() and
    /// QImageWriter::supportedImageFormats() functions to retrieve a
    /// complete list of the supported file formats.
    ///
    /// If a high resolution version of the image exists (identified by
    /// the suffix `@2x` on the base name), it is automatically loaded
    /// and added with the *device pixel ratio* set to a value of 2.
    /// This can be disabled by setting the environment variable
    /// `QT_HIGHDPI_DISABLE_2X_IMAGE_LOADING` (see QImageReader).
    ///
    /// **Note**: When you add a non-empty filename to a QIcon, the icon becomes
    /// non-null, even if the file doesn't exist or points to a corrupt file.
    ///
    /// **See also:** [`add_pixmap()`]
    /// [`Pixmap::device_pixel_ratio`]
    pub fn add_file<S: SizeTrait<'a>>(
        &self,
        file_name: &str,
        size: &S,
        mode: Mode,
        state: State,
    ) -> &Self {
        let str_in_file_name_1 = CString::new(file_name).unwrap();
        let (obj_size_2, _funcs) = size.get_size_obj_funcs();
        let enum_mode_3 = mode as u32;
        let enum_state_4 = state as u32;

        let (obj_data, funcs) = self.get_icon_obj_funcs();
        unsafe {
            ((*funcs).add_file)(
                obj_data,
                str_in_file_name_1.as_ptr(),
                obj_size_2,
                enum_mode_3,
                enum_state_4,
            );
        }
        self
    }
    ///
    /// Indicate that this icon is a mask image(boolean *isMask),* and hence can
    /// potentially be modified based on where it's displayed.
    /// **See also:** [`is_mask()`]
    pub fn set_is_mask(&self, is_mask: bool) -> &Self {
        let (obj_data, funcs) = self.get_icon_obj_funcs();
        unsafe {
            ((*funcs).set_is_mask)(obj_data, is_mask);
        }
        self
    }
    ///
    /// Returns `true` if this icon has been marked as a mask image.
    /// Certain platforms render mask icons differently (for example,
    /// menu icons on MacOS ).
    ///
    /// **See also:** [`set_is_mask()`]
    pub fn is_mask(&self) -> bool {
        let (obj_data, funcs) = self.get_icon_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_mask)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the QIcon corresponding to *name* in the current
    /// icon theme.
    ///
    /// The latest version of the freedesktop icon specification and naming
    /// specification can be obtained here:
    ///
    /// * [http://standards.freedesktop.org/icon-theme-spec/icon-theme-spec-latest.html](http://standards.freedesktop.org/icon-theme-spec/icon-theme-spec-latest.html)
    ///
    /// * [http://standards.freedesktop.org/icon-naming-spec/icon-naming-spec-latest.html](http://standards.freedesktop.org/icon-naming-spec/icon-naming-spec-latest.html)
    ///
    /// To fetch an icon from the current icon theme:
    ///
    /// **Note**: By default, only X11 will support themed icons. In order to
    /// use themed icons on Mac and Windows, you will have to bundle a
    /// compliant theme in one of your themeSearchPaths() and set the
    /// appropriate themeName().
    ///
    /// **Note**: Qt will make use of GTK's icon-theme.cache if present to speed up
    /// the lookup. These caches can be generated using gtk-update-icon-cache:
    /// [https://developer.gnome.org/gtk3/stable/gtk-update-icon-cache.html](https://developer.gnome.org/gtk3/stable/gtk-update-icon-cache.html)
    ///
    ///
    /// **Note**: If an icon can't be found in the current theme, then it will be
    /// searched in fallbackSearchPaths() as an unthemed icon.
    ///
    /// **See also:** [`theme_name()`]
    /// [`set_theme_name()`]
    /// [`theme_search_paths()`]
    /// [`fallback_search_paths()`]
    ///
    /// **Overloads**
    /// Returns the QIcon corresponding to *name* in the current
    /// icon theme. If no such icon is found in the current theme
    /// *fallback* is returned instead.
    ///
    /// If you want to provide a guaranteed fallback for platforms that
    /// do not support theme icons, you can use the second argument:
    ///
    pub fn from_theme(name: &str) -> Icon<'a> {
        let str_in_name_1 = CString::new(name).unwrap();

        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_icon)(::std::ptr::null()).all_funcs).icon_funcs,
            )
        };
        unsafe {
            let ret_val = ((*funcs).from_theme)(obj_data, str_in_name_1.as_ptr());
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Icon::new_from_rc(t);
            } else {
                ret_val = Icon::new_from_owned(t);
            }
            ret_val
        }
    }
    ///
    /// Returns the QIcon corresponding to *name* in the current
    /// icon theme.
    ///
    /// The latest version of the freedesktop icon specification and naming
    /// specification can be obtained here:
    ///
    /// * [http://standards.freedesktop.org/icon-theme-spec/icon-theme-spec-latest.html](http://standards.freedesktop.org/icon-theme-spec/icon-theme-spec-latest.html)
    ///
    /// * [http://standards.freedesktop.org/icon-naming-spec/icon-naming-spec-latest.html](http://standards.freedesktop.org/icon-naming-spec/icon-naming-spec-latest.html)
    ///
    /// To fetch an icon from the current icon theme:
    ///
    /// **Note**: By default, only X11 will support themed icons. In order to
    /// use themed icons on Mac and Windows, you will have to bundle a
    /// compliant theme in one of your themeSearchPaths() and set the
    /// appropriate themeName().
    ///
    /// **Note**: Qt will make use of GTK's icon-theme.cache if present to speed up
    /// the lookup. These caches can be generated using gtk-update-icon-cache:
    /// [https://developer.gnome.org/gtk3/stable/gtk-update-icon-cache.html](https://developer.gnome.org/gtk3/stable/gtk-update-icon-cache.html)
    ///
    ///
    /// **Note**: If an icon can't be found in the current theme, then it will be
    /// searched in fallbackSearchPaths() as an unthemed icon.
    ///
    /// **See also:** [`theme_name()`]
    /// [`set_theme_name()`]
    /// [`theme_search_paths()`]
    /// [`fallback_search_paths()`]
    ///
    /// **Overloads**
    /// Returns the QIcon corresponding to *name* in the current
    /// icon theme. If no such icon is found in the current theme
    /// *fallback* is returned instead.
    ///
    /// If you want to provide a guaranteed fallback for platforms that
    /// do not support theme icons, you can use the second argument:
    ///
    pub fn from_theme_2<I: IconTrait<'a>>(name: &str, fallback: &I) -> Icon<'a> {
        let str_in_name_1 = CString::new(name).unwrap();
        let (obj_fallback_2, _funcs) = fallback.get_icon_obj_funcs();

        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_icon)(::std::ptr::null()).all_funcs).icon_funcs,
            )
        };
        unsafe {
            let ret_val = ((*funcs).from_theme_2)(obj_data, str_in_name_1.as_ptr(), obj_fallback_2);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Icon::new_from_rc(t);
            } else {
                ret_val = Icon::new_from_owned(t);
            }
            ret_val
        }
    }
    ///
    /// Returns `true` if there is an icon available for *name* in the
    /// current icon theme, otherwise returns `false.`
    ///
    /// **See also:** [`theme_search_paths()`]
    /// [`from_theme()`]
    /// [`set_theme_name()`]
    pub fn has_theme_icon(name: &str) -> bool {
        let str_in_name_1 = CString::new(name).unwrap();

        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_icon)(::std::ptr::null()).all_funcs).icon_funcs,
            )
        };
        unsafe {
            let ret_val = ((*funcs).has_theme_icon)(obj_data, str_in_name_1.as_ptr());
            ret_val
        }
    }
    ///
    /// Returns the name of the current icon theme.
    ///
    /// On X11, the current icon theme depends on your desktop
    /// settings. On other platforms it is not set by default.
    ///
    /// **See also:** [`set_theme_name()`]
    /// [`theme_search_paths()`]
    /// [`from_theme()`]
    /// [`has_theme_icon()`]
    pub fn theme_name() -> String {
        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_icon)(::std::ptr::null()).all_funcs).icon_funcs,
            )
        };
        unsafe {
            let ret_val = ((*funcs).theme_name)(obj_data);
            let ret_val = CStr::from_ptr(ret_val).to_string_lossy().into_owned();
            ret_val
        }
    }
    ///
    /// Sets the current icon theme to *name.*
    ///
    /// The *name* should correspond to a directory name in the
    /// themeSearchPath() containing an index.theme
    /// file describing it's contents.
    ///
    /// **See also:** [`theme_search_paths()`]
    /// [`theme_name()`]
    pub fn set_theme_name(path: &str) {
        let str_in_path_1 = CString::new(path).unwrap();

        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_icon)(::std::ptr::null()).all_funcs).icon_funcs,
            )
        };
        unsafe {
            ((*funcs).set_theme_name)(obj_data, str_in_path_1.as_ptr());
        }
    }

    pub fn build(&self) -> Self {
        self.clone()
    }
}

impl<'a> From<(WrapperRcOwn, bool)> for Icon<'a> {
    fn from(t: (WrapperRcOwn, bool)) -> Self {
        if t.1 {
            Icon::new_from_rc(t.0 as *const RUIcon)
        } else {
            Icon::new_from_temporary(t.0 as *const RUIcon)
        }
    }
}

pub trait IconTrait<'a> {
    #[inline]
    #[doc(hidden)]
    fn get_icon_obj_funcs(&self) -> (*const RUBase, *const RUIconFuncs);
}

impl<'a> IconTrait<'a> for Icon<'a> {
    #[doc(hidden)]
    fn get_icon_obj_funcs(&self) -> (*const RUBase, *const RUIconFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).icon_funcs) }
    }
}
#[repr(u32)]
pub enum Mode {
    Normal = 0,
    Disabled = 1,
    Active = 2,
    Selected = 3,
}

#[repr(u32)]
pub enum State {
    On = 0,
    Off = 1,
}
