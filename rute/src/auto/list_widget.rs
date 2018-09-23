
use std::cell::Cell;
use std::marker::PhantomData;
use std::mem::transmute;
use std::os::raw::{c_void, c_char};
use std::cell::RefCell;
use std::rc::Rc;
use std::ffi::CString;
use std::ffi::CStr;
use auto::rute_enums::*;


#[derive(Clone)]
pub struct ListWidget<'a> {
    data: Rc<Cell<Option<RUListWidget>>>,
    _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

pub trait ListWidgetType {

    pub fn add_item(&self, label: &str) -> &Self {
        let str_in_label_1 = CString::new(label).unwrap();

        let (obj_data, funcs) = self.get_list_widget_obj_funcs();
        unsafe {
            ((*funcs).add_item)(obj_data, str_in_label_1.as_ptr());
        }
        self
    }

    pub fn clear(&self) -> &Self {

        let (obj_data, funcs) = self.get_list_widget_obj_funcs();
        unsafe {
            ((*funcs).clear)(obj_data);
        }
        self
    }

    fn get_list_widget_obj_funcs(&self) -> (*const RUBase, *const RUListWidgetFuncs);
}

impl<'a> WidgetType for ListWidget<'a> {
    fn get_widget_obj_funcs(&self) -> (*const RUBase, *const RUWidgetFuncs) {
        let obj = self.data.get().unwrap();
        (obj, self.all_funcs.widget_funcs)
    }
}

impl<'a> ListWidgetType for ListWidget<'a> {
    fn get_list_widget_obj_funcs(&self) -> (*const RUBase, *const RUListWidgetFuncs) {
        let obj = self.data.get().unwrap();
        (obj, self.all_funcs.list_widget_funcs)
    }
}
