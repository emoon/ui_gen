
use std::cell::Cell;
use std::marker::PhantomData;
use std::mem::transmute;
use std::os::raw::{c_void, c_char};
use std::cell::RefCell;
use std::rc::Rc;
use std::ffi::CString;
use auto::rute_auto_ffi::*;

unsafe extern "C" fn rute_object_delete_callback<T>(data: *const c_void) {
    let d = Rc::from_raw(data as *const Cell<Option<T>>);
    d.set(None);
}


#[derive(Clone)]
pub struct Application<'a> {
    data: Rc<Cell<Option<RUApplication>>>,
    _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

#[derive(Clone)]
pub struct Widget<'a> {
    data: Rc<Cell<Option<RUWidget>>>,
    _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

#[derive(Clone)]
pub struct Font<'a> {
    data: Rc<Cell<Option<RUFont>>>,
    _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> Application<'a> {
    pub fn set_style(&self, style: &str) -> &Application<'a> {
        let str_in_style_1 = CString::new(style).unwrap();

        let (obj_data, funcs) = self.get_application_obj_funcs();
    
        unsafe {
            ((*funcs).set_style)(obj_data, str_in_style_1.as_ptr());
        }
        self
    
    }

    pub fn exec(&self) -> i32 {
        
        let (obj_data, funcs) = self.get_application_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).exec)(obj_data);
        
            ret_val
          
        }
    
    }

    pub fn font(&self) -> Font {
        
        let (obj_data, funcs) = self.get_application_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).font)(obj_data);
        
            Font {
                data: Rc::new(Cell::new(Some(ret_val))),
                _marker: PhantomData,
            }
          
        }
    
    }
    fn get_application_obj_funcs(&self) -> (*const RUBase, *const RUApplicationFuncs) {
        let obj = self.data.get().unwrap();
        (obj.privd, obj.application_funcs)
    }
}
impl<'a> Widget<'a> {
    pub fn show(&self) -> &Widget<'a> {
        
        let (obj_data, funcs) = self.get_widget_obj_funcs();
    
        unsafe {
            ((*funcs).show)(obj_data);
        }
        self
    
    }

    pub fn set_fixed_height(&self, width: i32) -> &Widget<'a> {
        
        let (obj_data, funcs) = self.get_widget_obj_funcs();
    
        unsafe {
            ((*funcs).set_fixed_height)(obj_data, width);
        }
        self
    
    }

    pub fn set_fixed_width(&self, width: i32) -> &Widget<'a> {
        
        let (obj_data, funcs) = self.get_widget_obj_funcs();
    
        unsafe {
            ((*funcs).set_fixed_width)(obj_data, width);
        }
        self
    
    }

    pub fn resize(&self, width: i32, height: i32) -> &Widget<'a> {
        
        let (obj_data, funcs) = self.get_widget_obj_funcs();
    
        unsafe {
            ((*funcs).resize)(obj_data, width, height);
        }
        self
    
    }

    pub fn update(&self) -> &Widget<'a> {
        
        let (obj_data, funcs) = self.get_widget_obj_funcs();
    
        unsafe {
            ((*funcs).update)(obj_data);
        }
        self
    
    }
    fn get_widget_obj_funcs(&self) -> (*const RUBase, *const RUWidgetFuncs) {
        let obj = self.data.get().unwrap();
        (obj.privd, obj.widget_funcs)
    }
}
impl<'a> Font<'a> {
    pub fn set_pixel_size(&self, size: i32) -> &Font<'a> {
        
        let (obj_data, funcs) = self.get_font_obj_funcs();
    
        unsafe {
            ((*funcs).set_pixel_size)(obj_data, size);
        }
        self
    
    }

    pub fn pixel_size(&self) -> i32 {
        
        let (obj_data, funcs) = self.get_font_obj_funcs();
    
        unsafe {
            let ret_val = ((*funcs).pixel_size)(obj_data);
        
            ret_val
          
        }
    
    }
    fn get_font_obj_funcs(&self) -> (*const RUBase, *const RUFontFuncs) {
        let obj = self.data.get().unwrap();
        (obj.privd, obj.font_funcs)
    }
}

impl Drop for Font {
    fn drop(&mut self) {
        if Rc::strong_count(&self.data) == 1 {
            let obj = self.data.get().unwrap();
            unsafe {
                ((*obj.font_funcs).destroy)(obj.privd);
            }

            self.data.set(None);
        }
    }
}

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

    pub fn create_application(&self) -> Application<'a> {
        let ffi_data = unsafe { ((*self.rute_ffi).create_application)(::std::ptr::null()) };
        Application {
            data: Rc::new(Cell::new(Some(ffi_data))),
            _marker: PhantomData,
        }
    }

    pub fn create_widget(&self) -> Widget<'a> {
        let data = Rc::new(Cell::new(None));

        let ffi_data = unsafe {
            ((*self.rute_ffi).create_widget)(
                ::std::ptr::null(),
                transmute(rute_object_delete_callback::<RUWidget> as usize),
                Rc::into_raw(data.clone()) as *const c_void)
        };

        data.set(Some(ffi_data));

        Widget {
            data,
            _marker: PhantomData,
        }
    }

    pub fn create_font(&self) -> Font<'a> {
        let ffi_data = unsafe { ((*self.rute_ffi).create_font)(::std::ptr::null()) };
        Font {
            data: Rc::new(Cell::new(Some(ffi_data))),
            _marker: PhantomData,
        }
    }
}
