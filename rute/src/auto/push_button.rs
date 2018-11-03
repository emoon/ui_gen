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

#[derive(Clone)]
pub struct PushButton<'a> {
    pub data: Rc<Cell<Option<*const RUBase>>>,
    pub all_funcs: *const RUPushButtonAllFuncs,
    pub owned: bool,
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> PushButton<'a> {
    pub fn new() -> PushButton<'a> {
        let data = Rc::new(Cell::new(None));

        let ffi_data = unsafe {
            ((*rute_ffi_get()).create_push_button)(
                ::std::ptr::null(),
                transmute(rute_object_delete_callback as usize),
                Rc::into_raw(data.clone()) as *const c_void,
            )
        };

        data.set(Some(ffi_data.qt_data));

        PushButton {
            data,
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }
    pub fn new_from_rc(ffi_data: RUPushButton) -> PushButton<'a> {
        PushButton {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    pub fn new_from_owned(ffi_data: RUPushButton) -> PushButton<'a> {
        PushButton {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    pub fn new_from_temporary(ffi_data: RUPushButton) -> PushButton<'a> {
        PushButton {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }
}

unsafe extern "C" fn push_button_pressed_trampoline_ud<T>(
    self_c: *const c_void,
    func: *const c_void,
) {
    let f: &&(Fn(&T) + 'static) = transmute(func);

    let data = self_c as *const T;
    f(&*data);
}

unsafe extern "C" fn push_button_pressed_trampoline(self_c: *const c_void, func: *const c_void) {
    let f: &&(Fn() + 'static) = transmute(func);

    f();
}

unsafe extern "C" fn push_button_released_trampoline_ud<T>(
    self_c: *const c_void,
    func: *const c_void,
) {
    let f: &&(Fn(&T) + 'static) = transmute(func);

    let data = self_c as *const T;
    f(&*data);
}

unsafe extern "C" fn push_button_released_trampoline(self_c: *const c_void, func: *const c_void) {
    let f: &&(Fn() + 'static) = transmute(func);

    f();
}

unsafe extern "C" fn push_button_clicked_trampoline_ud<T>(
    self_c: *const c_void,
    func: *const c_void,
    checked: bool,
) {
    let f: &&(Fn(&T, bool) + 'static) = transmute(func);

    let data = self_c as *const T;
    f(&*data, checked);
}

unsafe extern "C" fn push_button_clicked_trampoline(
    self_c: *const c_void,
    func: *const c_void,
    checked: bool,
) {
    let f: &&(Fn(bool) + 'static) = transmute(func);

    f(checked);
}

unsafe extern "C" fn push_button_toggled_trampoline_ud<T>(
    self_c: *const c_void,
    func: *const c_void,
    checked: bool,
) {
    let f: &&(Fn(&T, bool) + 'static) = transmute(func);

    let data = self_c as *const T;
    f(&*data, checked);
}

unsafe extern "C" fn push_button_toggled_trampoline(
    self_c: *const c_void,
    func: *const c_void,
    checked: bool,
) {
    let f: &&(Fn(bool) + 'static) = transmute(func);

    f(checked);
}

pub trait PushButtonType<'a> {
    fn auto_default(&self) -> bool {
        let (obj_data, funcs) = self.get_push_button_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).auto_default)(obj_data);
            ret_val
        }
    }
    fn set_auto_default(&self, arg0: bool) -> &Self {
        let (obj_data, funcs) = self.get_push_button_obj_funcs();
        unsafe {
            ((*funcs).set_auto_default)(obj_data, arg0);
        }
        self
    }
    fn is_default(&self) -> bool {
        let (obj_data, funcs) = self.get_push_button_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_default)(obj_data);
            ret_val
        }
    }
    fn set_default(&self, arg0: bool) -> &Self {
        let (obj_data, funcs) = self.get_push_button_obj_funcs();
        unsafe {
            ((*funcs).set_default)(obj_data, arg0);
        }
        self
    }
    fn set_flat(&self, arg0: bool) -> &Self {
        let (obj_data, funcs) = self.get_push_button_obj_funcs();
        unsafe {
            ((*funcs).set_flat)(obj_data, arg0);
        }
        self
    }
    fn is_flat(&self) -> bool {
        let (obj_data, funcs) = self.get_push_button_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_flat)(obj_data);
            ret_val
        }
    }
    fn show_menu(&self) -> &Self {
        let (obj_data, funcs) = self.get_push_button_obj_funcs();
        unsafe {
            ((*funcs).show_menu)(obj_data);
        }
        self
    }
    fn set_text(&self, text: &str) -> &Self {
        let str_in_text_1 = CString::new(text).unwrap();

        let (obj_data, funcs) = self.get_push_button_obj_funcs();
        unsafe {
            ((*funcs).set_text)(obj_data, str_in_text_1.as_ptr());
        }
        self
    }
    fn text(&self) -> String {
        let (obj_data, funcs) = self.get_push_button_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).text)(obj_data);
            let ret_val = CStr::from_ptr(ret_val).to_string_lossy().into_owned();
            ret_val
        }
    }

    fn set_pressed_event_ud<F, T>(&self, data: &'a T, func: F) -> &Self
    where
        F: Fn(&T) + 'a,
        T: 'a,
    {
        let (obj_data, funcs) = self.get_push_button_obj_funcs();

        let f: Box<Box<Fn(&T) + 'a>> = Box::new(Box::new(func));
        let user_data = data as *const _ as *const c_void;

        unsafe {
            ((*funcs).set_pressed_event)(
                obj_data,
                user_data,
                Box::into_raw(f) as *const _,
                transmute(push_button_pressed_trampoline_ud::<T> as usize),
            );
        }

        self
    }

    fn set_pressed_event<F>(&self, func: F) -> &Self
    where
        F: Fn() + 'a,
    {
        let (obj_data, funcs) = self.get_push_button_obj_funcs();
        let f: Box<Box<Fn() + 'a>> = Box::new(Box::new(func));

        unsafe {
            ((*funcs).set_pressed_event)(
                obj_data,
                ::std::ptr::null(),
                Box::into_raw(f) as *const _,
                transmute(push_button_pressed_trampoline as usize),
            );
        }

        self
    }

    fn set_released_event_ud<F, T>(&self, data: &'a T, func: F) -> &Self
    where
        F: Fn(&T) + 'a,
        T: 'a,
    {
        let (obj_data, funcs) = self.get_push_button_obj_funcs();

        let f: Box<Box<Fn(&T) + 'a>> = Box::new(Box::new(func));
        let user_data = data as *const _ as *const c_void;

        unsafe {
            ((*funcs).set_released_event)(
                obj_data,
                user_data,
                Box::into_raw(f) as *const _,
                transmute(push_button_released_trampoline_ud::<T> as usize),
            );
        }

        self
    }

    fn set_released_event<F>(&self, func: F) -> &Self
    where
        F: Fn() + 'a,
    {
        let (obj_data, funcs) = self.get_push_button_obj_funcs();
        let f: Box<Box<Fn() + 'a>> = Box::new(Box::new(func));

        unsafe {
            ((*funcs).set_released_event)(
                obj_data,
                ::std::ptr::null(),
                Box::into_raw(f) as *const _,
                transmute(push_button_released_trampoline as usize),
            );
        }

        self
    }

    fn set_clicked_event_ud<F, T>(&self, data: &'a T, func: F) -> &Self
    where
        F: Fn(&T, bool) + 'a,
        T: 'a,
    {
        let (obj_data, funcs) = self.get_push_button_obj_funcs();

        let f: Box<Box<Fn(&T, bool) + 'a>> = Box::new(Box::new(func));
        let user_data = data as *const _ as *const c_void;

        unsafe {
            ((*funcs).set_clicked_event)(
                obj_data,
                user_data,
                Box::into_raw(f) as *const _,
                transmute(push_button_clicked_trampoline_ud::<T> as usize),
            );
        }

        self
    }

    fn set_clicked_event<F>(&self, func: F) -> &Self
    where
        F: Fn(bool) + 'a,
    {
        let (obj_data, funcs) = self.get_push_button_obj_funcs();
        let f: Box<Box<Fn(bool) + 'a>> = Box::new(Box::new(func));

        unsafe {
            ((*funcs).set_clicked_event)(
                obj_data,
                ::std::ptr::null(),
                Box::into_raw(f) as *const _,
                transmute(push_button_clicked_trampoline as usize),
            );
        }

        self
    }

    fn set_toggled_event_ud<F, T>(&self, data: &'a T, func: F) -> &Self
    where
        F: Fn(&T, bool) + 'a,
        T: 'a,
    {
        let (obj_data, funcs) = self.get_push_button_obj_funcs();

        let f: Box<Box<Fn(&T, bool) + 'a>> = Box::new(Box::new(func));
        let user_data = data as *const _ as *const c_void;

        unsafe {
            ((*funcs).set_toggled_event)(
                obj_data,
                user_data,
                Box::into_raw(f) as *const _,
                transmute(push_button_toggled_trampoline_ud::<T> as usize),
            );
        }

        self
    }

    fn set_toggled_event<F>(&self, func: F) -> &Self
    where
        F: Fn(bool) + 'a,
    {
        let (obj_data, funcs) = self.get_push_button_obj_funcs();
        let f: Box<Box<Fn(bool) + 'a>> = Box::new(Box::new(func));

        unsafe {
            ((*funcs).set_toggled_event)(
                obj_data,
                ::std::ptr::null(),
                Box::into_raw(f) as *const _,
                transmute(push_button_toggled_trampoline as usize),
            );
        }

        self
    }

    #[inline]
    fn get_push_button_obj_funcs(&self) -> (*const RUBase, *const RUPushButtonFuncs);
}

impl<'a> WidgetType<'a> for PushButton<'a> {
    #[inline]
    fn get_widget_obj_funcs(&self) -> (*const RUBase, *const RUWidgetFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).widget_funcs) }
    }
}

impl<'a> PushButtonType<'a> for PushButton<'a> {
    #[inline]
    fn get_push_button_obj_funcs(&self) -> (*const RUBase, *const RUPushButtonFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).push_button_funcs) }
    }
}
