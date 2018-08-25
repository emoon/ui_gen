pub static HEADER: &'static [u8] = b"
use std::cell::Cell;
use std::marker::PhantomData;
use std::mem::transmute;
use std::os::raw::{c_void, c_char};
use std::cell::RefCell;
use std::rc::Rc;
use std::ffi::CString;
use std::ffi::CStr;
use auto::rute_auto_ffi::*;

unsafe extern \"C\" fn rute_object_delete_callback<T>(data: *const c_void) {
    let d = Rc::from_raw(data as *const Cell<Option<T>>);
    d.set(None);
}
\n\n";

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub static RUTE_IMPL_HEADER: &'static [u8] = b"
pub struct Rute<'a> {
    rute_ffi: *const RuteFFI,
    priv_data: *const c_void,
    _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> Rute<'a> {
    pub fn new() -> Rute<'a> {
        Rute {
            rute_ffi: unsafe { rute_get() },
            priv_data: ::std::ptr::null(),
            _marker: PhantomData,
        }
    }
";

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub static RUST_NO_WRAP_TEMPLATE: &str = "
    pub fn create_{{widget_snake_name}}(&self) -> {{widget_name}}<'a> {
        let ffi_data = unsafe { ((*self.rute_ffi).create_{{widget_snake_name}})(::std::ptr::null()) };
        {{widget_name}} {
            data: Rc::new(Cell::new(Some(ffi_data))),
            _marker: PhantomData,
        }
    }
";

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub static RUST_GET_STATIC_TEMPLATE: &str = "
    pub fn {{widget_snake_name}}(&self) -> {{widget_name}}Static<'a> {
        let ffi_data = unsafe { ((*self.rute_ffi).get_{{widget_snake_name}})(::std::ptr::null()) };
        {{widget_name}}Static {
            data: ffi_data,
            _marker: PhantomData,
        }
    }
";

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub static RUST_CREATE_TEMPLATE: &str = "
    pub fn create_{{widget_snake_name}}(&self) -> {{widget_name}}<'a> {
        let data = Rc::new(Cell::new(None));

        let ffi_data = unsafe {
            ((*self.rute_ffi).create_{{widget_snake_name}})(
                ::std::ptr::null(),
                transmute(rute_object_delete_callback::<RU{{widget_name}}> as usize),
                Rc::into_raw(data.clone()) as *const c_void)
        };

        data.set(Some(ffi_data));

        {{widget_name}} {
            data,
            _marker: PhantomData,
        }
    }
";

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub static RUST_DROP_TEMPLATE: &str ="
impl<'a> Drop for {{type_name}}<'a> {
    fn drop(&mut self) {
        if Rc::strong_count(&self.data) == 1 {
            let obj = self.data.get().unwrap();
            unsafe {
                ((*obj.{{type_snake_name}}_funcs).destroy)(obj.privd);
            }

            self.data.set(None);
        }
    }
}
";

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub static RUST_FUNC_IMPL_TEMPLATE: &str = "
    pub fn {{func_name}}{{function_def}} {
        {{ body_setup }}
        let (obj_data, funcs) = self.get_{{obj_funcs_name}}_obj_funcs();
    {% if return_value %}
        unsafe {
            let ret_val = ((*funcs).{{func_name}})({{function_args}});
        {% case return_type %}
          {% when 'replaced' %}
           {{replaced_return}}
          {% when 'no_wrap' %}
            {{return_vtype}} {
                data: Rc::new(Cell::new(Some(ret_val))),
                _marker: PhantomData,
            }
          {% else %}
            ret_val
          {% endcase %}
        }
    {% else %}
        unsafe {
            ((*funcs).{{func_name}})({{function_args}});
        }
        self
    {% endif %}
    }
";

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub static RUST_CALLBACK_TEMPLATE: &str ="
    unsafe extern \"C\" fn {{event_name}}_trampoline<T>(
        user_data: *const c_void,
        func: *const c_void
        {{function_arguments}}
    ) {
        let f: &&(Fn(&T{{function_arg_types}}) + 'static) = transmute(func);
        let data = user_data as *const T;
        f(&*data{{function_params}});
    }

    pub fn {{event_name}}<F, T>(&self, data: &'a T, func: F) -> &{{widget_name}}<'a>
    where
        F: Fn(&T{{function_arg_types}}) + 'a,
        T: 'a,
    {
        let (obj_data, funcs) = self.get_{{widget_snake_name}}_obj_funcs();

        let f: Box<Box<Fn(&T{{function_arg_types}}) + 'a>> = Box::new(Box::new(func));
        let user_data = data as *const _ as *const c_void;

        unsafe {
            ((*funcs).set_{{event_name}}_event)(
                obj_data,
                user_data,
                transmute(Self::{{event_name}}_trampoline::<T> as usize),
                Box::into_raw(f) as *const _,
            );
        }

        self
    }
";


