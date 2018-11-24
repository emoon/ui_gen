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

// Auto-generated imports

#[allow(unused_imports)]
use auto::event::Event;
#[allow(unused_imports)]
use auto::event::EventTrait;
#[allow(unused_imports)]
use auto::event_ffi::*;
#[allow(unused_imports)]
use auto::object_ffi::*;
#[allow(unused_imports)]
use auto::rute::*;
#[allow(unused_imports)]
use auto::rute_enums::TimerType;
#[allow(unused_imports)]
use auto::rute_ffi::*;
///
/// QObject is the heart of the Qt [Object Model](Object%20Model)
/// . The central
/// feature in this model is a very powerful mechanism for seamless
/// object communication called [signals and slots](signals%20and%20slots)
/// . You can
/// connect a signal to a slot with connect() and destroy the
/// connection with disconnect(). To avoid never ending notification
/// loops you can temporarily block signals with blockSignals(). The
/// protected functions connectNotify() and disconnectNotify() make
/// it possible to track connections.
///
/// QObjects organize themselves in [object trees](Object%20Trees%20&%20Ownership)
/// . When you create a QObject with another object as
/// parent, the object will automatically add itself to the parent's
/// children() list. The parent takes ownership of the object; i.e.,
/// it will automatically delete its children in its destructor. You
/// can look for an object by name and optionally type using
/// findChild() or findChildren().
///
/// Every object has an objectName() and its class name can be found
/// via the corresponding metaObject() (see QMetaObject::className()).
/// You can determine whether the object's class inherits another
/// class in the QObject inheritance hierarchy by using the
/// inherits() function.
///
/// When an object is deleted, it emits a destroyed() signal. You can
/// catch this signal to avoid dangling references to QObjects.
///
/// QObjects can receive events through event() and filter the events
/// of other objects. See installEventFilter() and eventFilter() for
/// details. A convenience handler, childEvent(), can be reimplemented
/// to catch child events.
///
/// Last but not least, QObject provides the basic timer support in
/// Qt; see QTimer for high-level support for timers.
///
/// Notice that the Q_OBJECT macro is mandatory for any object that
/// implements signals, slots or properties. You also need to run the
/// [Meta Object Compiler](moc)
/// on the source file. We strongly
/// recommend the use of this macro in all subclasses of QObject
/// regardless of whether or not they actually use signals, slots and
/// properties, since failure to do so may lead certain functions to
/// exhibit strange behavior.
///
/// All Qt widgets inherit QObject. The convenience function
/// isWidgetType() returns whether an object is actually a widget. It
/// is much faster than
/// [qobject_cast](qobject_cast())
/// <QWidget *>( *obj* ) or
/// *obj* -> [inherits](inherits())
/// ("QWidget").
///
/// Some QObject functions, e.g. children(), return a QObjectList.
/// QObjectList is a typedef for QList<QObject *>.
///
/// # Thread Affinity
///
/// A QObject instance is said to have a *thread affinity* , or that
/// it *lives* in a certain thread. When a QObject receives a
/// [queued signal](Qt::QueuedConnection)
/// or a [posted event](The%20Event%0A%20%20%20%20System%23Sending%20Events)
/// , the slot or event handler
/// will run in the thread that the object lives in.
///
/// **Note**: If a QObject has no thread affinity (that is, if thread()
/// returns zero), or if it lives in a thread that has no running event
/// loop, then it cannot receive queued signals or posted events.
///
/// By default, a QObject lives in the thread in which it is created.
/// An object's thread affinity can be queried using thread() and
/// changed using moveToThread().
///
/// All QObjects must live in the same thread as their parent. Consequently:
///
/// * setParent() will fail if the two QObjects involved live in different threads.
/// * When a QObject is moved to another thread, all its children will be automatically moved too.
/// * moveToThread() will fail if the QObject has a parent.
/// * If QObjects are created within QThread::run(), they cannot become children of the QThread object because the QThread does not live in the thread that calls QThread::run().
///
/// **Note**: A QObject's member variables *do not* automatically become
/// its children. The parent-child relationship must be set by either
/// passing a pointer to the child's [constructor](QObject())
/// , or by
/// calling setParent(). Without this step, the object's member variables
/// will remain in the old thread when moveToThread() is called.
///
/// # No Copy Constructor or Assignment Operator
///
/// QObject has neither a copy constructor nor an assignment operator.
/// This is by design. Actually, they are declared, but in a
/// `private` section with the macro Q_DISABLE_COPY(). In fact, all
/// Qt classes derived from QObject (direct or indirect) use this
/// macro to declare their copy constructor and assignment operator to
/// be private. The reasoning is found in the discussion on
/// [Identity vs Value](Identity%20vs%20Value)
/// on the Qt [Object
/// Model](Object%0A%20%20%20%20Model)
/// page.
///
/// The main consequence is that you should use pointers to QObject
/// (or to your QObject subclass) where you might otherwise be tempted
/// to use your QObject subclass as a value. For example, without a
/// copy constructor, you can't use a subclass of QObject as the value
/// to be stored in one of the container classes. You must store
/// pointers.
///
/// # Auto-Connection
///
/// Qt's meta-object system provides a mechanism to automatically connect
/// signals and slots between QObject subclasses and their children. As long
/// as objects are defined with suitable object names, and slots follow a
/// simple naming convention, this connection can be performed at run-time
/// by the QMetaObject::connectSlotsByName() function.
///
/// [uic](uic)
/// generates code that invokes this function to enable
/// auto-connection to be performed between widgets on forms created
/// with *Qt Designer* . More information about using auto-connection with *Qt Designer* is
/// given in the [Using a Designer UI File in Your Application](Using%20a%20Designer%20UI%20File%20in%20Your%20Application)
/// section of
/// the *Qt Designer* manual.
///
/// # Dynamic Properties
///
/// From Qt 4.2, dynamic properties can be added to and removed from QObject
/// instances at run-time. Dynamic properties do not need to be declared at
/// compile-time, yet they provide the same advantages as static properties
/// and are manipulated using the same API - using property() to read them
/// and setProperty() to write them.
///
/// From Qt 4.3, dynamic properties are supported by
/// [Qt Designer](Qt%20Designer's%20Widget%20Editing%20Mode%23The%20Property%20Editor)
///
/// and both standard Qt widgets and user-created forms can be given dynamic
/// properties.
///
/// # Internationalization (I18n)
///
/// All QObject subclasses support Qt's translation features, making it possible
/// to translate an application's user interface into different languages.
///
/// To make user-visible text translatable, it must be wrapped in calls to
/// the tr() function. This is explained in detail in the
/// [Writing Source Code for Translation](Writing%20Source%20Code%20for%20Translation)
/// document.
///
/// **See also:** [`MetaObject`]
/// [`Pointer`]
/// [`ObjectCleanupHandler`]
/// [`q_disable_copy()`]
/// **See also:** {Object Trees & Ownership}
/// # Licence
///
/// The documentation is an adoption of the original [Qt Documentation](http://doc.qt.io/) and provided herein is licensed under the terms of the [GNU Free Documentation License version 1.3](http://www.gnu.org/licenses/fdl.html) as published by the Free Software Foundation.
#[derive(Clone)]
pub struct Object<'a> {
    pub data: Rc<Cell<Option<*const RUBase>>>,
    pub all_funcs: *const RUObjectAllFuncs,
    pub owned: bool,
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> Object<'a> {
    pub fn new() -> Object<'a> {
        let data = Rc::new(Cell::new(None));

        let ffi_data = unsafe {
            ((*rute_ffi_get()).create_object)(
                ::std::ptr::null(),
                transmute(rute_object_delete_callback as usize),
                Rc::into_raw(data.clone()) as *const c_void,
            )
        };

        data.set(Some(ffi_data.qt_data));

        Object {
            data,
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }
    pub fn new_from_rc(ffi_data: RUObject) -> Object<'a> {
        Object {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    pub fn new_from_owned(ffi_data: RUObject) -> Object<'a> {
        Object {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    pub fn new_from_temporary(ffi_data: RUObject) -> Object<'a> {
        Object {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }
    pub fn set_custom_event_ud<F, T>(&self, data: &'a T, func: F)
    where
        F: Fn(&T, &EventTrait) + 'a,
        T: 'a,
    {
        let (obj_data, funcs) = self.get_object_obj_funcs();

        let f: Box<Box<Fn(&T, &EventTrait) + 'a>> = Box::new(Box::new(func));
        let user_data = data as *const _ as *const c_void;

        unsafe {
            ((*funcs).set_custom_event)(
                obj_data,
                user_data,
                Box::into_raw(f) as *const _,
                transmute(object_custom_trampoline_ud::<T> as usize),
            );
        }
    }

    pub fn set_custom_event<F>(&self, func: F)
    where
        F: Fn(&EventTrait) + 'a,
    {
        let (obj_data, funcs) = self.get_object_obj_funcs();
        let f: Box<Box<Fn(&EventTrait) + 'a>> = Box::new(Box::new(func));

        unsafe {
            ((*funcs).set_custom_event)(
                obj_data,
                ::std::ptr::null(),
                Box::into_raw(f) as *const _,
                transmute(object_custom_trampoline as usize),
            );
        }
    }
}
///
/// Returns a pointer to the object that sent the signal, if called in
/// a slot activated by a signal; otherwise it returns 0. The pointer
/// is valid only during the execution of the slot that calls this
/// function from this object's thread context.
///
/// The pointer returned by this function becomes invalid if the
/// sender is destroyed, or if the slot is disconnected from the
/// sender's signal.
///
/// **Warning**: This function violates the object-oriented principle of
/// modularity. However, getting access to the sender might be useful
/// when many signals are connected to a single slot.
///
/// **Warning**: As mentioned above, the return value of this function is
/// not valid when the slot is called via a Qt::DirectConnection from
/// a thread different from this object's thread. Do not use this
/// function in this type of scenario.
///
/// **See also:** [`sender_signal_index()`]
///
/// Returns the meta-method index of the signal that called the currently
/// executing slot, which is a member of the class returned by sender().
/// If called outside of a slot activated by a signal, -1 is returned.
///
/// For signals with default parameters, this function will always return
/// the index with all parameters, regardless of which was used with
/// connect(). For example, the signal `destroyed(QObject *obj = 0)`
/// will have two different indexes (with and without the parameter), but
/// this function will always return the index with a parameter. This does
/// not apply when overloading signals with different parameters.
///
/// **Warning**: This function violates the object-oriented principle of
/// modularity. However, getting access to the signal index might be useful
/// when many signals are connected to a single slot.
///
/// **Warning**: The return value of this function is not valid when the slot
/// is called via a Qt::DirectConnection from a thread different from this
/// object's thread. Do not use this function in this type of scenario.
///
/// **See also:** [`sender()`]
/// [`MetaObject::index_of_signal`]
/// [`MetaObject::method`]
///
/// Returns the meta-method index of the signal that called the currently
/// executing slot, which is a member of the class returned by sender().
/// If called outside of a slot activated by a signal, -1 is returned.
///
/// For signals with default parameters, this function will always return
/// the index with all parameters, regardless of which was used with
/// connect(). For example, the signal `destroyed(QObject *obj = 0)`
/// will have two different indexes (with and without the parameter), but
/// this function will always return the index with a parameter. This does
/// not apply when overloading signals with different parameters.
///
/// **Warning**: This function violates the object-oriented principle of
/// modularity. However, getting access to the signal index might be useful
/// when many signals are connected to a single slot.
///
/// **Warning**: The return value of this function is not valid when the slot
/// is called via a Qt::DirectConnection from a thread different from this
/// object's thread. Do not use this function in this type of scenario.
///
/// **See also:** [`sender()`]
/// [`MetaObject::index_of_signal`]
/// [`MetaObject::method`]
///
/// Returns `true` if the *signal* is connected to at least one receiver,
/// otherwise returns `false.`
///
/// *signal* must be a signal member of this object, otherwise the behaviour
/// is undefined.
///
/// As the code snippet above illustrates, you can use this function
/// to avoid emitting a signal that nobody listens to.
///
/// **Warning**: This function violates the object-oriented principle of
/// modularity. However, it might be useful when you need to perform
/// expensive initialization only if something is connected to a
/// signal.
///
/// This event handler can be reimplemented in a subclass to receive
/// custom events. Custom events are user-defined events with a type
/// value at least as large as the QEvent::User item of the
/// QEvent::Type enum, and is typically a QEvent subclass. The event
/// is passed in the *event* parameter.
///
/// **See also:** [`event()`]
/// [`Event`]

unsafe extern "C" fn object_custom_trampoline_ud<T>(
    self_c: *const c_void,
    func: *const c_void,
    event: *const RUBase,
) {
    let f: &&(Fn(&T, &EventTrait) + 'static) = transmute(func);
    let obj_event_0 = Event::new_from_temporary(*(event as *const RUEvent));
    let data = self_c as *const T;
    f(&*data, &obj_event_0);
}

unsafe extern "C" fn object_custom_trampoline(
    self_c: *const c_void,
    func: *const c_void,
    event: *const RUBase,
) {
    let f: &&(Fn(&EventTrait) + 'static) = transmute(func);
    let obj_event_0 = Event::new_from_temporary(*(event as *const RUEvent));
    f(&obj_event_0);
}

pub trait ObjectTrait<'a> {
    ///
    /// This virtual function receives events to an object and should
    /// return true if the event *e* was recognized and processed.
    ///
    /// The event() function can be reimplemented to customize the
    /// behavior of an object.
    ///
    /// Make sure you call the parent event class implementation
    /// for all the events you did not handle.
    ///
    /// Example:
    ///
    /// **See also:** [`install_event_filter()`]
    /// [`timer_event()`]
    /// [`CoreApplication::send_event`]
    /// [`CoreApplication::post_event`]
    ///
    /// Filters events if this object has been installed as an event
    /// filter for the *watched* object.
    ///
    /// In your reimplementation of this function, if you want to filter
    /// the *event* out, i.e. stop it being handled further, return
    /// true; otherwise return false.
    ///
    /// Example:
    ///
    /// Notice in the example above that unhandled events are passed to
    /// the base class's eventFilter() function, since the base class
    /// might have reimplemented eventFilter() for its own internal
    /// purposes.
    ///
    /// **Warning**: If you delete the receiver object in this function, be
    /// sure to return true. Otherwise, Qt will forward the event to the
    /// deleted object and the program might crash.
    ///
    /// **See also:** [`install_event_filter()`]
    ///
    /// Filters events if this object has been installed as an event
    /// filter for the *watched* object.
    ///
    /// In your reimplementation of this function, if you want to filter
    /// the *event* out, i.e. stop it being handled further, return
    /// true; otherwise return false.
    ///
    /// Example:
    ///
    /// Notice in the example above that unhandled events are passed to
    /// the base class's eventFilter() function, since the base class
    /// might have reimplemented eventFilter() for its own internal
    /// purposes.
    ///
    /// **Warning**: If you delete the receiver object in this function, be
    /// sure to return true. Otherwise, Qt will forward the event to the
    /// deleted object and the program might crash.
    ///
    /// **See also:** [`install_event_filter()`]
    ///
    /// You can find an object by name (and type) using findChild().
    /// You can find a set of objects with findChildren().
    ///
    /// By default, this property contains an empty string.
    ///
    /// **See also:** [`meta_object()`]
    /// [`MetaObject::class_name`]
    ///
    /// This signal is emitted after the object's name has been changed. The new object name is passed as *objectName.*
    ///
    /// **See also:** [`Object::object_name()`]
    fn object_name(&self) -> String {
        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).object_name)(obj_data);
            let ret_val = CStr::from_ptr(ret_val).to_string_lossy().into_owned();
            ret_val
        }
    }
    fn set_object_name(&self, name: &str) {
        let str_in_name_1 = CString::new(name).unwrap();

        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            ((*funcs).set_object_name)(obj_data, str_in_name_1.as_ptr());
        }
    }
    ///
    /// Returns `true` if the object is a widget; otherwise returns `false.`
    ///
    /// Calling this function is equivalent to calling
    /// `inherits("QWidget")` , except that it is much faster.
    fn is_widget_type(&self) -> bool {
        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_widget_type)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns `true` if the object is a window; otherwise returns `false.`
    ///
    /// Calling this function is equivalent to calling
    /// `inherits("QWindow")` , except that it is much faster.
    fn is_window_type(&self) -> bool {
        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_window_type)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns `true` if signals are blocked; otherwise returns `false.`
    ///
    /// Signals are not blocked by default.
    ///
    /// **See also:** [`block_signals()`]
    /// [`SignalBlocker`]
    fn signals_blocked(&self) -> bool {
        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).signals_blocked)(obj_data);
            ret_val
        }
    }
    ///
    /// If *block* is true, signals emitted by this object are blocked
    /// (i.e., emitting a signal will not invoke anything connected to it).
    /// If *block* is false, no such blocking will occur.
    ///
    /// The return value is the previous value of signalsBlocked().
    ///
    /// Note that the destroyed() signal will be emitted even if the signals
    /// for this object have been blocked.
    ///
    /// Signals emitted while being blocked are not buffered.
    ///
    /// **See also:** [`signals_blocked()`]
    /// [`SignalBlocker`]
    fn block_signals(&self, b: bool) -> bool {
        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).block_signals)(obj_data, b);
            ret_val
        }
    }
    ///
    /// Starts a timer and returns a timer identifier, or returns zero if
    /// it could not start a timer.
    ///
    /// A timer event will occur every *interval* milliseconds until
    /// killTimer() is called. If *interval* is 0, then the timer event
    /// occurs once every time there are no more window system events to
    /// process.
    ///
    /// The virtual timerEvent() function is called with the QTimerEvent
    /// event parameter class when a timer event occurs. Reimplement this
    /// function to get timer events.
    ///
    /// If multiple timers are running, the QTimerEvent::timerId() can be
    /// used to find out which timer was activated.
    ///
    /// Example:
    ///
    /// Note that QTimer's accuracy depends on the underlying operating system and
    /// hardware. The *timerType* argument allows you to customize the accuracy of
    /// the timer. See Qt::TimerType for information on the different timer types.
    /// Most platforms support an accuracy of 20 milliseconds; some provide more.
    /// If Qt is unable to deliver the requested number of timer events, it will
    /// silently discard some.
    ///
    /// The QTimer class provides a high-level programming interface with
    /// single-shot timers and timer signals instead of events. There is
    /// also a QBasicTimer class that is more lightweight than QTimer and
    /// less clumsy than using timer IDs directly.
    ///
    /// **See also:** [`timer_event()`]
    /// [`kill_timer()`]
    /// [`Timer::single_shot`]
    ///
    /// **Overloads**
    /// Starts a timer and returns a timer identifier, or returns zero if
    /// it could not start a timer.
    ///
    /// A timer event will occur every *time* interval until killTimer()
    /// is called. If *time* is equal to `std::chrono::duration::zero()` ,
    /// then the timer event occurs once every time there are no more window
    /// system events to process.
    ///
    /// The virtual timerEvent() function is called with the QTimerEvent
    /// event parameter class when a timer event occurs. Reimplement this
    /// function to get timer events.
    ///
    /// If multiple timers are running, the QTimerEvent::timerId() can be
    /// used to find out which timer was activated.
    ///
    /// Example:
    ///
    /// Note that QTimer's accuracy depends on the underlying operating system and
    /// hardware. The *timerType* argument allows you to customize the accuracy of
    /// the timer. See Qt::TimerType for information on the different timer types.
    /// Most platforms support an accuracy of 20 milliseconds; some provide more.
    /// If Qt is unable to deliver the requested number of timer events, it will
    /// silently discard some.
    ///
    /// The QTimer class provides a high-level programming interface with
    /// single-shot timers and timer signals instead of events. There is
    /// also a QBasicTimer class that is more lightweight than QTimer and
    /// less clumsy than using timer IDs directly.
    ///
    /// **See also:** [`timer_event()`]
    /// [`kill_timer()`]
    /// [`Timer::single_shot`]
    fn start_timer(&self, interval: i32, timer_type: TimerType) -> i32 {
        let enum_timer_type_2 = timer_type as i32;

        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).start_timer)(obj_data, interval, enum_timer_type_2);
            ret_val
        }
    }
    ///
    /// Starts a timer and returns a timer identifier, or returns zero if
    /// it could not start a timer.
    ///
    /// A timer event will occur every *interval* milliseconds until
    /// killTimer() is called. If *interval* is 0, then the timer event
    /// occurs once every time there are no more window system events to
    /// process.
    ///
    /// The virtual timerEvent() function is called with the QTimerEvent
    /// event parameter class when a timer event occurs. Reimplement this
    /// function to get timer events.
    ///
    /// If multiple timers are running, the QTimerEvent::timerId() can be
    /// used to find out which timer was activated.
    ///
    /// Example:
    ///
    /// Note that QTimer's accuracy depends on the underlying operating system and
    /// hardware. The *timerType* argument allows you to customize the accuracy of
    /// the timer. See Qt::TimerType for information on the different timer types.
    /// Most platforms support an accuracy of 20 milliseconds; some provide more.
    /// If Qt is unable to deliver the requested number of timer events, it will
    /// silently discard some.
    ///
    /// The QTimer class provides a high-level programming interface with
    /// single-shot timers and timer signals instead of events. There is
    /// also a QBasicTimer class that is more lightweight than QTimer and
    /// less clumsy than using timer IDs directly.
    ///
    /// **See also:** [`timer_event()`]
    /// [`kill_timer()`]
    /// [`Timer::single_shot`]
    ///
    /// **Overloads**
    /// Starts a timer and returns a timer identifier, or returns zero if
    /// it could not start a timer.
    ///
    /// A timer event will occur every *time* interval until killTimer()
    /// is called. If *time* is equal to `std::chrono::duration::zero()` ,
    /// then the timer event occurs once every time there are no more window
    /// system events to process.
    ///
    /// The virtual timerEvent() function is called with the QTimerEvent
    /// event parameter class when a timer event occurs. Reimplement this
    /// function to get timer events.
    ///
    /// If multiple timers are running, the QTimerEvent::timerId() can be
    /// used to find out which timer was activated.
    ///
    /// Example:
    ///
    /// Note that QTimer's accuracy depends on the underlying operating system and
    /// hardware. The *timerType* argument allows you to customize the accuracy of
    /// the timer. See Qt::TimerType for information on the different timer types.
    /// Most platforms support an accuracy of 20 milliseconds; some provide more.
    /// If Qt is unable to deliver the requested number of timer events, it will
    /// silently discard some.
    ///
    /// The QTimer class provides a high-level programming interface with
    /// single-shot timers and timer signals instead of events. There is
    /// also a QBasicTimer class that is more lightweight than QTimer and
    /// less clumsy than using timer IDs directly.
    ///
    /// **See also:** [`timer_event()`]
    /// [`kill_timer()`]
    /// [`Timer::single_shot`]
    fn start_timer_2(&self, time: u32, timer_type: TimerType) -> i32 {
        let enum_timer_type_2 = timer_type as i32;

        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).start_timer_2)(obj_data, time, enum_timer_type_2);
            ret_val
        }
    }
    ///
    /// Kills the timer with timer identifier, *id.*
    ///
    /// The timer identifier is returned by startTimer() when a timer
    /// event is started.
    ///
    /// **See also:** [`timer_event()`]
    /// [`start_timer()`]
    fn kill_timer(&self, id: i32) {
        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            ((*funcs).kill_timer)(obj_data, id);
        }
    }
    ///
    /// Makes the object a child of *parent.*
    ///
    /// **See also:** [`parent()`]
    /// [`children()`]
    fn set_parent(&self, parent: &ObjectTrait) {
        let (obj_parent_1, _funcs) = parent.get_object_obj_funcs();

        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            ((*funcs).set_parent)(obj_data, obj_parent_1);
        }
    }
    ///
    /// Installs an event filter *filterObj* on this object. For example:
    ///
    /// An event filter is an object that receives all events that are
    /// sent to this object. The filter can either stop the event or
    /// forward it to this object. The event filter *filterObj* receives
    /// events via its eventFilter() function. The eventFilter() function
    /// must return true if the event should be filtered, (i.e. stopped);
    /// otherwise it must return false.
    ///
    /// If multiple event filters are installed on a single object, the
    /// filter that was installed last is activated first.
    ///
    /// Here's a `KeyPressEater` class that eats the key presses of its
    /// monitored objects:
    ///
    /// And here's how to install it on two widgets:
    ///
    /// The QShortcut class, for example, uses this technique to intercept
    /// shortcut key presses.
    ///
    /// **Warning**: If you delete the receiver object in your eventFilter()
    /// function, be sure to return true. If you return false, Qt sends
    /// the event to the deleted object and the program will crash.
    ///
    /// Note that the filtering object must be in the same thread as this
    /// object. If *filterObj* is in a different thread, this function does
    /// nothing. If either *filterObj* or this object are moved to a different
    /// thread after calling this function, the event filter will not be
    /// called until both objects have the same thread affinity again (it
    /// is *not* removed).
    ///
    /// **See also:** [`remove_event_filter()`]
    /// [`event_filter()`]
    /// [`event()`]
    fn install_event_filter(&self, filter_obj: &ObjectTrait) {
        let (obj_filter_obj_1, _funcs) = filter_obj.get_object_obj_funcs();

        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            ((*funcs).install_event_filter)(obj_data, obj_filter_obj_1);
        }
    }
    ///
    /// Removes an event filter object *obj* from this object. The
    /// request is ignored if such an event filter has not been installed.
    ///
    /// All event filters for this object are automatically removed when
    /// this object is destroyed.
    ///
    /// It is always safe to remove an event filter, even during event
    /// filter activation (i.e. from the eventFilter() function).
    ///
    /// **See also:** [`install_event_filter()`]
    /// [`event_filter()`]
    /// [`event()`]
    ///
    /// **Overloads**
    /// Dumps a tree of children to the debug output.
    ///
    /// **See also:** [`dump_object_info()`]
    ///
    /// Dumps a tree of children to the debug output.
    ///
    /// **Note**: before Qt 5.9, this function was not const.
    ///
    /// **See also:** [`dump_object_info()`]
    fn dump_object_tree(&self) {
        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            ((*funcs).dump_object_tree)(obj_data);
        }
    }
    ///
    /// **Overloads**
    /// Dumps information about signal connections, etc. for this object
    /// to the debug output.
    ///
    /// **See also:** [`dump_object_tree()`]
    ///
    /// Dumps information about signal connections, etc. for this object
    /// to the debug output.
    ///
    /// **Note**: before Qt 5.9, this function was not const.
    ///
    /// **See also:** [`dump_object_tree()`]
    fn dump_object_info(&self) {
        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            ((*funcs).dump_object_info)(obj_data);
        }
    }
    ///
    /// **Overloads**
    /// Dumps a tree of children to the debug output.
    ///
    /// **See also:** [`dump_object_info()`]
    ///
    /// Dumps a tree of children to the debug output.
    ///
    /// **Note**: before Qt 5.9, this function was not const.
    ///
    /// **See also:** [`dump_object_info()`]
    fn dump_object_tree_2(&self) {
        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            ((*funcs).dump_object_tree_2)(obj_data);
        }
    }
    ///
    /// **Overloads**
    /// Dumps information about signal connections, etc. for this object
    /// to the debug output.
    ///
    /// **See also:** [`dump_object_tree()`]
    ///
    /// Dumps information about signal connections, etc. for this object
    /// to the debug output.
    ///
    /// **Note**: before Qt 5.9, this function was not const.
    ///
    /// **See also:** [`dump_object_tree()`]
    fn dump_object_info_2(&self) {
        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            ((*funcs).dump_object_info_2)(obj_data);
        }
    }
    ///
    /// Sets the value of the object's *name* property to *value.*
    ///
    /// If the property is defined in the class using Q_PROPERTY then
    /// true is returned on success and false otherwise. If the property
    /// is not defined using Q_PROPERTY, and therefore not listed in the
    /// meta-object, it is added as a dynamic property and false is returned.
    ///
    /// Information about all available properties is provided through the
    /// metaObject() and dynamicPropertyNames().
    ///
    /// Dynamic properties can be queried again using property() and can be
    /// removed by setting the property value to an invalid QVariant.
    /// Changing the value of a dynamic property causes a QDynamicPropertyChangeEvent
    /// to be sent to the object.
    ///
    /// **Note:** Dynamic properties starting with are reserved for internal
    /// purposes.
    ///
    /// **See also:** [`property()`]
    /// [`meta_object()`]
    /// [`dynamic_property_names()`]
    /// [`MetaProperty::write`]
    ///
    /// Returns the value of the object's *name* property.
    ///
    /// If no such property exists, the returned variant is invalid.
    ///
    /// Information about all available properties is provided through the
    /// metaObject() and dynamicPropertyNames().
    ///
    /// **See also:** [`set_property()`]
    /// [`Variant::is_valid`]
    /// [`meta_object()`]
    /// [`dynamic_property_names()`]
    ///
    /// This signal is emitted after the object's name has been changed. The new object name is passed as *objectName.*
    ///
    /// **See also:** [`Object::object_name()`]
    ///
    /// Returns a pointer to the parent object.
    ///
    /// **See also:** [`children()`]
    fn parent(&self) -> Option<Object> {
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
    ///
    /// Schedules this object for deletion.
    ///
    /// The object will be deleted when control returns to the event
    /// loop. If the event loop is not running when this function is
    /// called (e.g. deleteLater() is called on an object before
    /// QCoreApplication::exec()), the object will be deleted once the
    /// event loop is started. If deleteLater() is called after the main event loop
    /// has stopped, the object will not be deleted.
    /// Since Qt 4.8, if deleteLater() is called on an object that lives in a
    /// thread with no running event loop, the object will be destroyed when the
    /// thread finishes.
    ///
    /// Note that entering and leaving a new event loop (e.g., by opening a modal
    /// dialog) will *not* perform the deferred deletion; for the object to be
    /// deleted, the control must return to the event loop from which
    /// deleteLater() was called.
    ///
    /// **Note:** It is safe to call this function more than once; when the
    /// first deferred deletion event is delivered, any pending events for the
    /// object are removed from the event queue.
    ///
    /// **See also:** [`destroyed()`]
    /// [`Pointer`]
    fn delete_later(&self) {
        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            ((*funcs).delete_later)(obj_data);
        }
    }

    #[inline]
    fn get_object_obj_funcs(&self) -> (*const RUBase, *const RUObjectFuncs);
}

impl<'a> ObjectTrait<'a> for Object<'a> {
    #[inline]
    fn get_object_obj_funcs(&self) -> (*const RUBase, *const RUObjectFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).object_funcs) }
    }
}