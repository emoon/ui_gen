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
/// QMimeData is used to describe information that can be stored in
/// the [clipboard](QClipboard)
/// , and transferred via the [drag
/// and drop](drag%0A%20%20%20%20and%20drop)
/// mechanism. QMimeData objects associate the data that
/// they hold with the corresponding MIME types to ensure that
/// information can be safely transferred between applications, and
/// copied around within the same application.
///
/// QMimeData objects are usually created using `new` and supplied
/// to QDrag or QClipboard objects. This is to enable Qt to manage
/// the memory that they use.
///
/// A single QMimeData object can store the same data using several
/// different formats at the same time. The formats() function
/// returns a list of the available formats in order of preference.
/// The data() function returns the raw data associated with a MIME
/// type, and setData() allows you to set the data for a MIME type.
///
/// For the most common MIME types, QMimeData provides convenience
/// functions to access the data:
///
/// * Tester
/// * Getter
/// * Setter
/// * MIME Types
/// * hasText()
/// * text()
/// * setText()
/// * `text/plain`
/// * hasHtml()
/// * html()
/// * setHtml()
/// * `text/html`
/// * hasUrls()
/// * urls()
/// * setUrls()
/// * `text/uri-list`
/// * hasImage()
/// * imageData()
/// * setImageData()
/// * `image/` *
/// * hasColor()
/// * colorData()
/// * setColorData()
/// * `application/x-color`
///
/// For example, if your write a widget that accepts URL drags, you
/// would end up writing code like this:
///
/// There are three approaches for storing custom data in a QMimeData
/// object:
///
/// * Custom data can be stored directly in a QMimeData object as a QByteArray using setData(). For example:
/// * We can subclass QMimeData and reimplement hasFormat(), formats(), and retrieveData().
/// * If the drag and drop operation occurs within a single application, we can subclass QMimeData and add extra data in it, and use a qobject_cast() in the receiver's drop event handler. For example:
///
/// # Platform-Specific MIME Types
///
/// On Windows, formats() will also return custom formats available
/// in the MIME data, using the `x-qt-windows-mime` subtype to
/// indicate that they represent data in non-standard formats.
/// The formats will take the following form:
///
/// The following are examples of custom MIME types:
///
/// The `value` declaration of each format describes the way in which the
/// data is encoded.
///
/// In some cases (e.g. dropping multiple email attachments), multiple data
/// values are available. They can be accessed by adding an `index` value:
///
/// On Windows, the MIME format does not always map directly to the
/// clipboard formats. Qt provides QWinMime to map clipboard
/// formats to open-standard MIME formats. Similarly, the
/// QMacPasteboardMime maps MIME to Mac flavors.
///
/// **See also:** [`Clipboard`]
/// [`DragEnterEvent`]
/// [`DragMoveEvent`]
/// [`DropEvent`]
/// [`Drag`]
/// [`MacPasteboardMime`]
/// {Drag and Drop}
/// # Licence
///
/// The documentation is an adoption of the original [Qt Documentation](http://doc.qt.io/) and provided herein is licensed under the terms of the [GNU Free Documentation License version 1.3](http://www.gnu.org/licenses/fdl.html) as published by the Free Software Foundation.
#[derive(Clone)]
pub struct MimeData<'a> {
    #[doc(hidden)]
    pub data: Rc<Cell<Option<*const RUBase>>>,
    #[doc(hidden)]
    pub all_funcs: *const RUMimeDataAllFuncs,
    #[doc(hidden)]
    pub owned: bool,
    #[doc(hidden)]
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> MimeData<'a> {
    pub fn new() -> MimeData<'a> {
        let ffi_data = unsafe { ((*rute_ffi_get()).create_mime_data)(::std::ptr::null()) };
        MimeData {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }
    #[allow(dead_code)]
    pub(crate) fn new_from_rc(ffi_data: RUMimeData) -> MimeData<'a> {
        MimeData {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_owned(ffi_data: RUMimeData) -> MimeData<'a> {
        MimeData {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_temporary(ffi_data: RUMimeData) -> MimeData<'a> {
        MimeData {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }
    ///
    /// Returns a list of URLs contained within the MIME data object.
    ///
    /// URLs correspond to the MIME type `text/uri-list.`
    ///
    /// **See also:** [`has_urls()`]
    /// [`data()`]
    ///
    /// Sets the URLs stored in the MIME data object to those specified by *urls.*
    ///
    /// URLs correspond to the MIME type `text/uri-list.`
    ///
    /// Since Qt 5.0, setUrls also exports the urls as plain text, if setText
    /// was not called before, to make it possible to drop them into any lineedit
    /// and text editor.
    ///
    /// **See also:** [`has_urls()`]
    /// [`set_data()`]
    ///
    /// Returns `true` if the object can return a list of urls; otherwise
    /// returns `false.`
    ///
    /// URLs correspond to the MIME type `text/uri-list.`
    ///
    /// **See also:** [`set_urls()`]
    /// [`urls()`]
    /// [`has_format()`]
    pub fn has_urls(&self) -> bool {
        let (obj_data, funcs) = self.get_mime_data_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).has_urls)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns a plain text (MIME type `text/plain)` representation of
    /// the data.
    ///
    /// **See also:** [`has_text()`]
    /// [`html()`]
    /// [`data()`]
    pub fn text(&self) -> String {
        let (obj_data, funcs) = self.get_mime_data_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).text)(obj_data);
            let ret_val = CStr::from_ptr(ret_val).to_string_lossy().into_owned();
            ret_val
        }
    }
    ///
    /// Sets *text* as the plain text (MIME type `text/plain)` used to
    /// represent the data.
    ///
    /// **See also:** [`has_text()`]
    /// [`set_html()`]
    /// [`set_data()`]
    pub fn set_text(&self, text: &str) -> &Self {
        let str_in_text_1 = CString::new(text).unwrap();

        let (obj_data, funcs) = self.get_mime_data_obj_funcs();
        unsafe {
            ((*funcs).set_text)(obj_data, str_in_text_1.as_ptr());
        }
        self
    }
    ///
    /// Returns `true` if the object can return plain text (MIME type `text/plain);` otherwise returns `false.`
    ///
    /// **See also:** [`set_text()`]
    /// [`text()`]
    /// [`has_html()`]
    /// [`has_format()`]
    pub fn has_text(&self) -> bool {
        let (obj_data, funcs) = self.get_mime_data_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).has_text)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns a string if the data stored in the object is HTML (MIME
    /// type `text/html);` otherwise returns an empty string.
    ///
    /// **See also:** [`has_html()`]
    /// [`set_data()`]
    pub fn html(&self) -> String {
        let (obj_data, funcs) = self.get_mime_data_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).html)(obj_data);
            let ret_val = CStr::from_ptr(ret_val).to_string_lossy().into_owned();
            ret_val
        }
    }
    ///
    /// Sets *html* as the HTML (MIME type `text/html)` used to
    /// represent the data.
    ///
    /// **See also:** [`has_html()`]
    /// [`set_text()`]
    /// [`set_data()`]
    pub fn set_html(&self, html: &str) -> &Self {
        let str_in_html_1 = CString::new(html).unwrap();

        let (obj_data, funcs) = self.get_mime_data_obj_funcs();
        unsafe {
            ((*funcs).set_html)(obj_data, str_in_html_1.as_ptr());
        }
        self
    }
    ///
    /// Returns `true` if the object can return HTML (MIME type `text/html);` otherwise returns `false.`
    ///
    /// **See also:** [`set_html()`]
    /// [`html()`]
    /// [`has_format()`]
    pub fn has_html(&self) -> bool {
        let (obj_data, funcs) = self.get_mime_data_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).has_html)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns a QVariant storing a QImage if the object can return an
    /// image; otherwise returns a null variant.
    ///
    /// A QVariant is used because QMimeData belongs to the Qt Core
    /// module, whereas QImage belongs to Qt GUI. To convert the
    /// QVariant to a QImage, simply use qvariant_cast(). For example:
    ///
    /// **See also:** [`has_image()`]
    ///
    /// Sets the data in the object to the given *image.*
    ///
    /// A QVariant is used because QMimeData belongs to the Qt Core
    /// module, whereas QImage belongs to Qt GUI. The conversion
    /// from QImage to QVariant is implicit. For example:
    ///
    /// **See also:** [`has_image()`]
    /// [`set_data()`]
    ///
    /// Returns `true` if the object can return an image; otherwise returns
    /// false.
    ///
    /// **See also:** [`set_image_data()`]
    /// [`image_data()`]
    /// [`has_format()`]
    pub fn has_image(&self) -> bool {
        let (obj_data, funcs) = self.get_mime_data_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).has_image)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns a color if the data stored in the object represents a
    /// color (MIME type `application/x-color);` otherwise returns a
    /// null variant.
    ///
    /// A QVariant is used because QMimeData belongs to the Qt Core
    /// module, whereas QColor belongs to Qt GUI. To convert the
    /// QVariant to a QColor, simply use qvariant_cast(). For example:
    ///
    /// **See also:** [`has_color()`]
    /// [`set_color_data()`]
    /// [`data()`]
    ///
    /// Sets the color data in the object to the given *color.*
    ///
    /// Colors correspond to the MIME type `application/x-color.`
    ///
    /// **See also:** [`has_color()`]
    /// [`set_data()`]
    ///
    /// Returns `true` if the object can return a color (MIME type `application/x-color);` otherwise returns `false.`
    ///
    /// **See also:** [`set_color_data()`]
    /// [`color_data()`]
    /// [`has_format()`]
    pub fn has_color(&self) -> bool {
        let (obj_data, funcs) = self.get_mime_data_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).has_color)(obj_data);
            ret_val
        }
    }
    ///
    /// Removes the data entry for *mimeType* in the object.
    pub fn remove_format(&self, mimetype: &str) -> &Self {
        let str_in_mimetype_1 = CString::new(mimetype).unwrap();

        let (obj_data, funcs) = self.get_mime_data_obj_funcs();
        unsafe {
            ((*funcs).remove_format)(obj_data, str_in_mimetype_1.as_ptr());
        }
        self
    }
    ///
    /// Returns `true` if the object can return data for the MIME type
    /// specified by *mimeType;* otherwise returns `false.`
    ///
    /// For the most common types of data, you can call the higher-level
    /// functions hasText(), hasHtml(), hasUrls(), hasImage(), and
    /// hasColor() instead.
    ///
    /// **See also:** [`formats()`]
    /// [`set_data()`]
    /// [`data()`]
    pub fn has_format(&self, mimetype: &str) -> bool {
        let str_in_mimetype_1 = CString::new(mimetype).unwrap();

        let (obj_data, funcs) = self.get_mime_data_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).has_format)(obj_data, str_in_mimetype_1.as_ptr());
            ret_val
        }
    }
    ///
    /// Returns a list of formats supported by the object. This is a list
    /// of MIME types for which the object can return suitable data. The
    /// formats in the list are in a priority order.
    ///
    /// For the most common types of data, you can call the higher-level
    /// functions hasText(), hasHtml(), hasUrls(), hasImage(), and
    /// hasColor() instead.
    ///
    /// **See also:** [`has_format()`]
    /// [`set_data()`]
    /// [`data()`]
    ///
    /// Removes all the MIME type and data entries in the object.
    pub fn clear(&self) -> &Self {
        let (obj_data, funcs) = self.get_mime_data_obj_funcs();
        unsafe {
            ((*funcs).clear)(obj_data);
        }
        self
    }
    #[doc(hidden)]
    pub fn object_name(&self) -> String {
        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).object_name)(obj_data);
            let ret_val = CStr::from_ptr(ret_val).to_string_lossy().into_owned();
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn set_object_name(&self, name: &str) -> &Self {
        let str_in_name_1 = CString::new(name).unwrap();

        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            ((*funcs).set_object_name)(obj_data, str_in_name_1.as_ptr());
        }
        self
    }
    #[doc(hidden)]
    pub fn is_widget_type(&self) -> bool {
        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_widget_type)(obj_data);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn is_window_type(&self) -> bool {
        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_window_type)(obj_data);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn signals_blocked(&self) -> bool {
        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).signals_blocked)(obj_data);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn block_signals(&self, b: bool) -> bool {
        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).block_signals)(obj_data, b);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn start_timer(&self, interval: i32, timer_type: TimerType) -> i32 {
        let enum_timer_type_2 = timer_type as i32;

        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).start_timer)(obj_data, interval, enum_timer_type_2);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn start_timer_2(&self, time: u32, timer_type: TimerType) -> i32 {
        let enum_timer_type_2 = timer_type as i32;

        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).start_timer_2)(obj_data, time, enum_timer_type_2);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn kill_timer(&self, id: i32) -> &Self {
        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            ((*funcs).kill_timer)(obj_data, id);
        }
        self
    }
    #[doc(hidden)]
    pub fn set_parent<O: ObjectTrait<'a>>(&self, parent: &O) -> &Self {
        let (obj_parent_1, _funcs) = parent.get_object_obj_funcs();

        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            ((*funcs).set_parent)(obj_data, obj_parent_1);
        }
        self
    }
    #[doc(hidden)]
    pub fn install_event_filter<O: ObjectTrait<'a>>(&self, filter_obj: &O) -> &Self {
        let (obj_filter_obj_1, _funcs) = filter_obj.get_object_obj_funcs();

        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            ((*funcs).install_event_filter)(obj_data, obj_filter_obj_1);
        }
        self
    }
    #[doc(hidden)]
    pub fn dump_object_tree(&self) -> &Self {
        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            ((*funcs).dump_object_tree)(obj_data);
        }
        self
    }
    #[doc(hidden)]
    pub fn dump_object_info(&self) -> &Self {
        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            ((*funcs).dump_object_info)(obj_data);
        }
        self
    }
    #[doc(hidden)]
    pub fn dump_object_tree_2(&self) -> &Self {
        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            ((*funcs).dump_object_tree_2)(obj_data);
        }
        self
    }
    #[doc(hidden)]
    pub fn dump_object_info_2(&self) -> &Self {
        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            ((*funcs).dump_object_info_2)(obj_data);
        }
        self
    }
    #[doc(hidden)]
    pub fn parent(&self) -> Option<Object> {
        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).parent)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Object::new_from_rc(t);
            } else {
                ret_val = Object::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    #[doc(hidden)]
    pub fn delete_later(&self) -> &Self {
        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            ((*funcs).delete_later)(obj_data);
        }
        self
    }
    #[doc(hidden)]
    pub fn set_custom_event_ud<F, T>(&self, data: &'a T, func: F) -> &Self
    where
        F: Fn(&T, &Event) + 'a,
        T: 'a,
    {
        let (obj_data, funcs) = self.get_object_obj_funcs();

        let f: Box<Box<Fn(&T, &Event) + 'a>> = Box::new(Box::new(func));
        let user_data = data as *const _ as *const c_void;

        unsafe {
            ((*funcs).set_custom_event)(
                obj_data,
                user_data,
                Box::into_raw(f) as *const _,
                transmute(object_custom_trampoline_ud::<T> as usize),
            );
        }

        self
    }

    pub fn set_custom_event<F>(&self, func: F) -> &Self
    where
        F: Fn(&Event) + 'a,
    {
        let (obj_data, funcs) = self.get_object_obj_funcs();
        let f: Box<Box<Fn(&Event) + 'a>> = Box::new(Box::new(func));

        unsafe {
            ((*funcs).set_custom_event)(
                obj_data,
                ::std::ptr::null(),
                Box::into_raw(f) as *const _,
                transmute(object_custom_trampoline as usize),
            );
        }

        self
    }
}
pub trait MimeDataTrait<'a> {
    #[inline]
    #[doc(hidden)]
    fn get_mime_data_obj_funcs(&self) -> (*const RUBase, *const RUMimeDataFuncs);
}

impl<'a> ObjectTrait<'a> for MimeData<'a> {
    #[doc(hidden)]
    fn get_object_obj_funcs(&self) -> (*const RUBase, *const RUObjectFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).object_funcs) }
    }
}

impl<'a> MimeDataTrait<'a> for MimeData<'a> {
    #[doc(hidden)]
    fn get_mime_data_obj_funcs(&self) -> (*const RUBase, *const RUMimeDataFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).mime_data_funcs) }
    }
}
