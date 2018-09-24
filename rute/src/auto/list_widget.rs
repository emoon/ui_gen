
use std::cell::Cell;
use std::marker::PhantomData;
use std::rc::Rc;

#[allow(unused_imports)]
use std::ffi::{CString, CStr};

use rute_ffi_base::*;

#[allow(unused_imports)]
use auto::*;


#[derive(Clone)]
pub struct ListWidget<'a> {
    pub data: Rc<Cell<Option<*const RUBase>>>,
    pub all_funcs: *const RUListWidgetAllFuncs,
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl <'a>ListWidget<'a> {
    pub fn new_from_rc(ffi_data: RUListWidget) -> ListWidget<'a> {
        ListWidget {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            _marker: PhantomData,
        }
    }

    pub fn new_from_owned(ffi_data: RUListWidget) -> ListWidget<'a> {
        ListWidget {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            _marker: PhantomData,
        }
    }
}

pub trait ListWidgetType {

    fn add_item(&self, label: &str) -> &Self {
        let str_in_label_1 = CString::new(label).unwrap();

        let (obj_data, funcs) = self.get_list_widget_obj_funcs();
        unsafe {
            ((*funcs).add_item)(obj_data, str_in_label_1.as_ptr());
        }
        self
    }

    fn clear(&self) -> &Self {

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
        unsafe {
            (obj, (*self.all_funcs).widget_funcs)
        }
    }
}

impl<'a> ListWidgetType for ListWidget<'a> {
    fn get_list_widget_obj_funcs(&self) -> (*const RUBase, *const RUListWidgetFuncs) {
        let obj = self.data.get().unwrap();
        unsafe {
            (obj, (*self.all_funcs).list_widget_funcs)
        }
    }
}
