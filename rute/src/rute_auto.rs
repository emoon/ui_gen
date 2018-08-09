
use std::cell::Cell;
use std::marker::PhantomData;
use std::mem::transmute;
use std::os::raw::{c_void, c_char};
use std::cell::RefCell;
use std::rc::Rc;
use std::ffi::CString;

#[derive(Clone)]
pub struct Application<'a> {
    data: Rc<Cell<Option<RUApplication>>>,
    _marker: PhantomData<std::cell::Cell<&'a ()>>,
}
pub struct Rute<'a> {
    rute_ffi: *const RuteFFI,
    priv_data: *const c_void,
    _marker: PhantomData<std::cell::Cell<&'a ()>>,
}

impl<'a> Rute<'a> {
    pub fn new() -> Rute<'a> {
        Rute {
            rute_ffi: unsafe { rute_get() },
            _marker: PhantomData,
        }
    }

    pub fn create_application(&self) -> Application<'a> {
        let ffi_data = unsafe { ((*self.rute_ffi).create_application)(self.privd) };
        Widget {
            data: Rc::new(Cell::new(Some(ffi_data))),
            _marker: PhantomData,
        }
    }
}
