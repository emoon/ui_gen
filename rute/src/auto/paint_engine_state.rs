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
/// QPaintEngineState records which properties that have changed since
/// the last time the paint engine was updated, as well as their
/// current value.
///
/// Which properties that have changed can at any time be retrieved
/// using the state() function. This function returns an instance of
/// the QPaintEngine::DirtyFlags type which stores an OR combination
/// of QPaintEngine::DirtyFlag values. The QPaintEngine::DirtyFlag
/// enum defines whether a property has changed since the last update
/// or not.
///
/// If a property is marked with a dirty flag, its current value can
/// be retrieved using the corresponding get function:
///
/// * Property Flag
/// * Current Property Value
/// * QPaintEngine::DirtyBackground
/// * backgroundBrush()
/// * QPaintEngine::DirtyBackgroundMode
/// * backgroundMode()
/// * QPaintEngine::DirtyBrush
/// * brush()
/// * QPaintEngine::DirtyBrushOrigin
/// * brushOrigin()
/// * QPaintEngine::DirtyClipRegion *or* QPaintEngine::DirtyClipPath
/// * clipOperation()
/// * QPaintEngine::DirtyClipPath
/// * clipPath()
/// * QPaintEngine::DirtyClipRegion
/// * clipRegion()
/// * QPaintEngine::DirtyCompositionMode
/// * compositionMode()
/// * QPaintEngine::DirtyFont
/// * font()
/// * QPaintEngine::DirtyTransform
/// * transform()
/// * QPaintEngine::DirtyClipEnabled
/// * isClipEnabled()
/// * QPaintEngine::DirtyPen
/// * pen()
/// * QPaintEngine::DirtyHints
/// * renderHints()
///
/// The QPaintEngineState class also provide the painter() function
/// which returns a pointer to the painter that is currently updating
/// the paint engine.
///
/// An instance of this class, representing the current state of the
/// active paint engine, is passed as argument to the
/// QPaintEngine::updateState() function. The only situation in which
/// you will have to use this class directly is when implementing your
/// own paint engine.
///
/// **See also:** QPaintEngine
/// # Licence
///
/// The documentation is an adoption of the original [Qt Documentation](http://doc.qt.io/) and provided herein is licensed under the terms of the [GNU Free Documentation License version 1.3](http://www.gnu.org/licenses/fdl.html) as published by the Free Software Foundation.
#[derive(Clone)]
pub struct PaintEngineState<'a> {
    #[doc(hidden)]
    pub data: Rc<Cell<Option<*const RUBase>>>,
    #[doc(hidden)]
    pub all_funcs: *const RUPaintEngineStateAllFuncs,
    #[doc(hidden)]
    pub owned: bool,
    #[doc(hidden)]
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> PaintEngineState<'a> {
    pub fn new() -> PaintEngineState<'a> {
        let data = Rc::new(Cell::new(None));

        let ffi_data = unsafe {
            ((*rute_ffi_get()).create_paint_engine_state)(
                ::std::ptr::null(),
                transmute(rute_object_delete_callback as usize),
                Rc::into_raw(data.clone()) as *const c_void,
            )
        };

        data.set(Some(ffi_data.qt_data));

        PaintEngineState {
            data,
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }
    #[allow(dead_code)]
    pub(crate) fn new_from_rc(ffi_data: RUPaintEngineState) -> PaintEngineState<'a> {
        PaintEngineState {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_owned(ffi_data: RUPaintEngineState) -> PaintEngineState<'a> {
        PaintEngineState {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_temporary(ffi_data: RUPaintEngineState) -> PaintEngineState<'a> {
        PaintEngineState {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }
    ///
    /// Returns a combination of flags identifying the set of properties
    /// that need to be updated when updating the paint engine's state
    /// (i.e. during a call to the QPaintEngine::updateState() function).
    ///
    /// **See also:** QPaintEngine::updateState()
    pub fn state(&self) -> DirtyFlags {
        let (obj_data, funcs) = self.get_paint_engine_state_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).state)(obj_data);
            let ret_val = { transmute::<i32, DirtyFlags>(ret_val) };
            ret_val
        }
    }
    ///
    /// Returns the pen in the current paint engine state.
    ///
    /// This variable should only be used when the state() returns a
    /// combination which includes the QPaintEngine::DirtyPen flag.
    ///
    /// **See also:** state()
    /// QPaintEngine::updateState()
    ///
    /// Returns whether the coordinate of the stroke have been specified
    /// as bounded by the current rendering operation and have to be
    /// resolved (about the currently rendered primitive).
    pub fn pen(&self) -> Pen {
        let (obj_data, funcs) = self.get_paint_engine_state_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).pen)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Pen::new_from_rc(t);
            } else {
                ret_val = Pen::new_from_owned(t);
            }
            ret_val
        }
    }
    ///
    /// Returns the brush in the current paint engine state.
    ///
    /// This variable should only be used when the state() returns a
    /// combination which includes the QPaintEngine::DirtyBrush flag.
    ///
    /// **See also:** state()
    /// QPaintEngine::updateState()
    ///
    /// Returns the brush origin in the current paint engine state.
    ///
    /// This variable should only be used when the state() returns a
    /// combination which includes the QPaintEngine::DirtyBrushOrigin flag.
    ///
    /// **See also:** state()
    /// QPaintEngine::updateState()
    ///
    /// Returns whether the coordinate of the fill have been specified
    /// as bounded by the current rendering operation and have to be
    /// resolved (about the currently rendered primitive).
    pub fn brush(&self) -> Brush {
        let (obj_data, funcs) = self.get_paint_engine_state_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).brush)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Brush::new_from_rc(t);
            } else {
                ret_val = Brush::new_from_owned(t);
            }
            ret_val
        }
    }
    ///
    /// Returns the brush origin in the current paint engine state.
    ///
    /// This variable should only be used when the state() returns a
    /// combination which includes the QPaintEngine::DirtyBrushOrigin flag.
    ///
    /// **See also:** state()
    /// QPaintEngine::updateState()
    pub fn brush_origin(&self) -> PointF {
        let (obj_data, funcs) = self.get_paint_engine_state_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).brush_origin)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = PointF::new_from_rc(t);
            } else {
                ret_val = PointF::new_from_owned(t);
            }
            ret_val
        }
    }
    ///
    /// Returns the background brush in the current paint engine state.
    ///
    /// This variable should only be used when the state() returns a
    /// combination which includes the QPaintEngine::DirtyBackground flag.
    ///
    /// **See also:** state()
    /// QPaintEngine::updateState()
    pub fn background_brush(&self) -> Brush {
        let (obj_data, funcs) = self.get_paint_engine_state_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).background_brush)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Brush::new_from_rc(t);
            } else {
                ret_val = Brush::new_from_owned(t);
            }
            ret_val
        }
    }
    ///
    /// Returns the background mode in the current paint engine
    /// state.
    ///
    /// This variable should only be used when the state() returns a
    /// combination which includes the QPaintEngine::DirtyBackgroundMode flag.
    ///
    /// **See also:** state()
    /// QPaintEngine::updateState()
    pub fn background_mode(&self) -> BGMode {
        let (obj_data, funcs) = self.get_paint_engine_state_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).background_mode)(obj_data);
            let ret_val = { transmute::<i32, BGMode>(ret_val) };
            ret_val
        }
    }
    ///
    /// Returns the font in the current paint engine
    /// state.
    ///
    /// This variable should only be used when the state() returns a
    /// combination which includes the QPaintEngine::DirtyFont flag.
    ///
    /// **See also:** state()
    /// QPaintEngine::updateState()
    pub fn font(&self) -> Font {
        let (obj_data, funcs) = self.get_paint_engine_state_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).font)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Font::new_from_rc(t);
            } else {
                ret_val = Font::new_from_owned(t);
            }
            ret_val
        }
    }
    ///
    /// Returns the clip operation in the current paint engine
    /// state.
    ///
    /// This variable should only be used when the state() returns a
    /// combination which includes either the QPaintEngine::DirtyClipPath
    /// or the QPaintEngine::DirtyClipRegion flag.
    ///
    /// **See also:** state()
    /// QPaintEngine::updateState()
    pub fn clip_operation(&self) -> ClipOperation {
        let (obj_data, funcs) = self.get_paint_engine_state_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).clip_operation)(obj_data);
            let ret_val = { transmute::<i32, ClipOperation>(ret_val) };
            ret_val
        }
    }
    ///
    /// Returns the clip region in the current paint engine state.
    ///
    /// This variable should only be used when the state() returns a
    /// combination which includes the QPaintEngine::DirtyClipRegion flag.
    ///
    /// **See also:** state()
    /// QPaintEngine::updateState()
    pub fn clip_region(&self) -> Region {
        let (obj_data, funcs) = self.get_paint_engine_state_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).clip_region)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Region::new_from_rc(t);
            } else {
                ret_val = Region::new_from_owned(t);
            }
            ret_val
        }
    }
    ///
    /// Returns whether clipping is enabled or not in the current paint
    /// engine state.
    ///
    /// This variable should only be used when the state() returns a
    /// combination which includes the QPaintEngine::DirtyClipEnabled
    /// flag.
    ///
    /// **See also:** state()
    /// QPaintEngine::updateState()
    pub fn is_clip_enabled(&self) -> bool {
        let (obj_data, funcs) = self.get_paint_engine_state_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_clip_enabled)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns the render hints in the current paint engine state.
    ///
    /// This variable should only be used when the state() returns a
    /// combination which includes the QPaintEngine::DirtyHints
    /// flag.
    ///
    /// **See also:** state()
    /// QPaintEngine::updateState()
    pub fn render_hints(&self) -> RenderHints {
        let (obj_data, funcs) = self.get_paint_engine_state_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).render_hints)(obj_data);
            let ret_val = { transmute::<i32, RenderHints>(ret_val) };
            ret_val
        }
    }
    ///
    /// Returns the composition mode in the current paint engine state.
    ///
    /// This variable should only be used when the state() returns a
    /// combination which includes the QPaintEngine::DirtyCompositionMode
    /// flag.
    ///
    /// **See also:** state()
    /// QPaintEngine::updateState()
    pub fn composition_mode(&self) -> CompositionMode {
        let (obj_data, funcs) = self.get_paint_engine_state_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).composition_mode)(obj_data);
            let ret_val = { transmute::<i32, CompositionMode>(ret_val) };
            ret_val
        }
    }
    ///
    /// Returns the opacity in the current paint engine state.
    pub fn opacity(&self) -> f32 {
        let (obj_data, funcs) = self.get_paint_engine_state_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).opacity)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns a pointer to the painter currently updating the paint
    /// engine.
    pub fn painter(&self) -> Option<Painter> {
        let (obj_data, funcs) = self.get_paint_engine_state_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).painter)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Painter::new_from_rc(t);
            } else {
                ret_val = Painter::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    ///
    /// Returns whether the coordinate of the fill have been specified
    /// as bounded by the current rendering operation and have to be
    /// resolved (about the currently rendered primitive).
    pub fn brush_needs_resolving(&self) -> bool {
        let (obj_data, funcs) = self.get_paint_engine_state_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).brush_needs_resolving)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns whether the coordinate of the stroke have been specified
    /// as bounded by the current rendering operation and have to be
    /// resolved (about the currently rendered primitive).
    pub fn pen_needs_resolving(&self) -> bool {
        let (obj_data, funcs) = self.get_paint_engine_state_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).pen_needs_resolving)(obj_data);
            ret_val
        }
    }
}
pub trait PaintEngineStateTrait<'a> {
    #[inline]
    #[doc(hidden)]
    fn get_paint_engine_state_obj_funcs(&self) -> (*const RUBase, *const RUPaintEngineStateFuncs);
}

impl<'a> PaintEngineStateTrait<'a> for PaintEngineState<'a> {
    #[doc(hidden)]
    fn get_paint_engine_state_obj_funcs(&self) -> (*const RUBase, *const RUPaintEngineStateFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).paint_engine_state_funcs) }
    }
}
