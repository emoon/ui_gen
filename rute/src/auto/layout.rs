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
/// This is an abstract base class inherited by the concrete classes
/// QBoxLayout, QGridLayout, QFormLayout, and QStackedLayout.
///
/// For users of QLayout subclasses or of QMainWindow there is seldom
/// any need to use the basic functions provided by QLayout, such as
/// setSizeConstraint() or setMenuBar(). See [Layout Management](Layout%20Management)
/// for more information.
///
/// To make your own layout manager, implement the functions
/// addItem(), sizeHint(), setGeometry(), itemAt() and takeAt(). You
/// should also implement minimumSize() to ensure your layout isn't
/// resized to zero size if there is too little space. To support
/// children whose heights depend on their widths, implement
/// hasHeightForWidth() and heightForWidth(). See the
/// [Border Layout](layouts/borderlayout)
/// and
/// [Flow Layout](layouts/flowlayout)
/// examples for
/// more information about implementing custom layout managers.
///
/// Geometry management stops when the layout manager is deleted.
///
/// **See also:** [`LayoutItem`]
/// {Layout Management}
/// {Basic Layouts Example}
/// {Border Layout Example}
/// {Flow Layout Example}
/// # Licence
///
/// The documentation is an adoption of the original [Qt Documentation](http://doc.qt.io/) and provided herein is licensed under the terms of the [GNU Free Documentation License version 1.3](http://www.gnu.org/licenses/fdl.html) as published by the Free Software Foundation.
#[derive(Clone)]
pub struct Layout<'a> {
    #[doc(hidden)]
    pub data: Rc<Cell<Option<*const RUBase>>>,
    #[doc(hidden)]
    pub all_funcs: *const RULayoutAllFuncs,
    #[doc(hidden)]
    pub owned: bool,
    #[doc(hidden)]
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> Layout<'a> {
    #[allow(dead_code)]
    pub(crate) fn new_from_rc(ffi_data: RULayout) -> Layout<'a> {
        Layout {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_owned(ffi_data: RULayout) -> Layout<'a> {
        Layout {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_temporary(ffi_data: RULayout) -> Layout<'a> {
        Layout {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }
    ///
    /// Use setContentsMargins() and getContentsMargins() instead.
    ///
    /// **See also:** [`contents_rect()`]
    /// spacing
    ///
    pub fn margin(&self) -> i32 {
        let (obj_data, funcs) = self.get_layout_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).margin)(obj_data);
            ret_val
        }
    }
    ///
    /// If no value is explicitly set, the layout's spacing is inherited
    /// from the parent layout, or from the style settings for the parent
    /// widget.
    ///
    /// For QGridLayout and QFormLayout, it is possible to set different horizontal and
    /// vertical spacings using [setHorizontalSpacing()](QGridLayout::)
    ///
    /// and [setVerticalSpacing()](QGridLayout::)
    /// . In that case,
    /// spacing() returns -1.
    ///
    /// **See also:** [`contents_rect()`]
    /// [`get_contents_margins()`]
    /// [`Style::layout_spacing`]
    /// [`Style::pixel_metric`]
    pub fn spacing(&self) -> i32 {
        let (obj_data, funcs) = self.get_layout_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).spacing)(obj_data);
            ret_val
        }
    }
    ///
    pub fn set_margin(&self, arg0: i32) -> &Self {
        let (obj_data, funcs) = self.get_layout_obj_funcs();
        unsafe {
            ((*funcs).set_margin)(obj_data, arg0);
        }
        self
    }
    pub fn set_spacing(&self, arg0: i32) -> &Self {
        let (obj_data, funcs) = self.get_layout_obj_funcs();
        unsafe {
            ((*funcs).set_spacing)(obj_data, arg0);
        }
        self
    }
    ///
    /// Sets the *left,* *top,* *right,* and *bottom* margins to use
    /// around the layout.
    ///
    /// By default, QLayout uses the values provided by the style. On
    /// most platforms, the margin is 11 pixels in all directions.
    ///
    /// **See also:** [`get_contents_margins()`]
    /// [`Style::pixel_metric`]
    /// {QStyle::}{PM_LayoutLeftMargin}
    /// {QStyle::}{PM_LayoutTopMargin}
    /// {QStyle::}{PM_LayoutRightMargin}
    /// {QStyle::}{PM_LayoutBottomMargin}
    ///
    /// Sets the *margins* to use around the layout.
    ///
    /// By default, QLayout uses the values provided by the style. On
    /// most platforms, the margin is 11 pixels in all directions.
    ///
    /// **See also:** [`contents_margins()`]
    pub fn set_contents_margins(&self, left: i32, top: i32, right: i32, bottom: i32) -> &Self {
        let (obj_data, funcs) = self.get_layout_obj_funcs();
        unsafe {
            ((*funcs).set_contents_margins)(obj_data, left, top, right, bottom);
        }
        self
    }
    ///
    /// Sets the *left,* *top,* *right,* and *bottom* margins to use
    /// around the layout.
    ///
    /// By default, QLayout uses the values provided by the style. On
    /// most platforms, the margin is 11 pixels in all directions.
    ///
    /// **See also:** [`get_contents_margins()`]
    /// [`Style::pixel_metric`]
    /// {QStyle::}{PM_LayoutLeftMargin}
    /// {QStyle::}{PM_LayoutTopMargin}
    /// {QStyle::}{PM_LayoutRightMargin}
    /// {QStyle::}{PM_LayoutBottomMargin}
    ///
    /// Sets the *margins* to use around the layout.
    ///
    /// By default, QLayout uses the values provided by the style. On
    /// most platforms, the margin is 11 pixels in all directions.
    ///
    /// **See also:** [`contents_margins()`]
    pub fn set_contents_margins_2<M: MarginsTrait<'a>>(&self, margins: &M) -> &Self {
        let (obj_margins_1, _funcs) = margins.get_margins_obj_funcs();

        let (obj_data, funcs) = self.get_layout_obj_funcs();
        unsafe {
            ((*funcs).set_contents_margins_2)(obj_data, obj_margins_1);
        }
        self
    }
    ///
    /// Extracts the left, top, right, and bottom margins used around the
    /// layout, and assigns them to * *left,* * *top,* * *right,* and * *bottom* (unless they are null pointers).
    ///
    /// By default, QLayout uses the values provided by the style. On
    /// most platforms, the margin is 11 pixels in all directions.
    ///
    /// **See also:** [`set_contents_margins()`]
    /// [`Style::pixel_metric`]
    /// {QStyle::}{PM_LayoutLeftMargin}
    /// {QStyle::}{PM_LayoutTopMargin}
    /// {QStyle::}{PM_LayoutRightMargin}
    /// {QStyle::}{PM_LayoutBottomMargin}
    pub fn contents_margins(&self) -> Margins {
        let (obj_data, funcs) = self.get_layout_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).contents_margins)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Margins::new_from_rc(t);
            } else {
                ret_val = Margins::new_from_owned(t);
            }
            ret_val
        }
    }
    ///
    /// Returns the layout's geometry() rectangle, but taking into account the
    /// contents margins.
    ///
    /// **See also:** [`set_contents_margins()`]
    /// [`get_contents_margins()`]
    pub fn contents_rect(&self) -> Rect {
        let (obj_data, funcs) = self.get_layout_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).contents_rect)(obj_data);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Rect::new_from_rc(t);
            } else {
                ret_val = Rect::new_from_owned(t);
            }
            ret_val
        }
    }
    ///
    /// Sets the alignment for widget *w* to *alignment* and returns
    /// true if *w* is found in this layout (not including child
    /// layouts); otherwise returns `false.`
    ///
    /// **Overloads**
    /// Sets the alignment for the layout *l* to *alignment* and
    /// returns `true` if *l* is found in this layout (not including child
    /// layouts); otherwise returns `false.`
    pub fn set_alignment<W: WidgetTrait<'a>>(&self, w: &W, alignment: Alignment) -> bool {
        let (obj_w_1, _funcs) = w.get_widget_obj_funcs();
        let enum_alignment_2 = alignment.bits();

        let (obj_data, funcs) = self.get_layout_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).set_alignment)(obj_data, obj_w_1, enum_alignment_2);
            ret_val
        }
    }
    ///
    /// Sets the alignment for widget *w* to *alignment* and returns
    /// true if *w* is found in this layout (not including child
    /// layouts); otherwise returns `false.`
    ///
    /// **Overloads**
    /// Sets the alignment for the layout *l* to *alignment* and
    /// returns `true` if *l* is found in this layout (not including child
    /// layouts); otherwise returns `false.`
    pub fn set_alignment_2<L: LayoutTrait<'a>>(&self, l: &L, alignment: Alignment) -> bool {
        let (obj_l_1, _funcs) = l.get_layout_obj_funcs();
        let enum_alignment_2 = alignment.bits();

        let (obj_data, funcs) = self.get_layout_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).set_alignment_2)(obj_data, obj_l_1, enum_alignment_2);
            ret_val
        }
    }
    ///
    /// Tells the geometry manager to place the menu bar *widget* at the
    /// top of parentWidget(), outside QWidget::contentsMargins(). All
    /// child widgets are placed below the bottom edge of the menu bar.
    pub fn set_menu_bar<W: WidgetTrait<'a>>(&self, w: &W) -> &Self {
        let (obj_w_1, _funcs) = w.get_widget_obj_funcs();

        let (obj_data, funcs) = self.get_layout_obj_funcs();
        unsafe {
            ((*funcs).set_menu_bar)(obj_data, obj_w_1);
        }
        self
    }
    ///
    /// Returns the menu bar set for this layout, or 0 if no menu bar is
    /// set.
    pub fn menu_bar(&self) -> Option<Widget> {
        let (obj_data, funcs) = self.get_layout_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).menu_bar)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Widget::new_from_rc(t);
            } else {
                ret_val = Widget::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    ///
    /// Returns the parent widget of this layout, or 0 if this layout is
    /// not installed on any widget.
    ///
    /// If the layout is a sub-layout, this function returns the parent
    /// widget of the parent layout.
    ///
    /// **See also:** [`parent()`]
    pub fn parent_widget(&self) -> Option<Widget> {
        let (obj_data, funcs) = self.get_layout_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).parent_widget)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Widget::new_from_rc(t);
            } else {
                ret_val = Widget::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    ///
    pub fn invalidate(&self) -> &Self {
        let (obj_data, funcs) = self.get_layout_obj_funcs();
        unsafe {
            ((*funcs).invalidate)(obj_data);
        }
        self
    }
    ///
    /// Redoes the layout for parentWidget() if necessary.
    ///
    /// You should generally not need to call this because it is
    /// automatically called at the most appropriate times. It returns
    /// true if the layout was redone.
    ///
    /// **See also:** [`update()`]
    /// [`Widget::update_geometry`]
    pub fn activate(&self) -> bool {
        let (obj_data, funcs) = self.get_layout_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).activate)(obj_data);
            ret_val
        }
    }
    ///
    /// Updates the layout for parentWidget().
    ///
    /// You should generally not need to call this because it is
    /// automatically called at the most appropriate times.
    ///
    /// **See also:** [`activate()`]
    /// [`invalidate()`]
    pub fn update(&self) -> &Self {
        let (obj_data, funcs) = self.get_layout_obj_funcs();
        unsafe {
            ((*funcs).update)(obj_data);
        }
        self
    }
    ///
    /// Adds widget *w* to this layout in a manner specific to the
    /// layout. This function uses addItem().
    pub fn add_widget<W: WidgetTrait<'a>>(&self, w: &W) -> &Self {
        let (obj_w_1, _funcs) = w.get_widget_obj_funcs();

        let (obj_data, funcs) = self.get_layout_obj_funcs();
        unsafe {
            ((*funcs).add_widget)(obj_data, obj_w_1);
        }
        self
    }
    ///
    /// Implemented in subclasses to add an *item.* How it is added is
    /// specific to each subclass.
    ///
    /// This function is not usually called in application code. To add a widget
    /// to a layout, use the addWidget() function; to add a child layout, use the
    /// addLayout() function provided by the relevant QLayout subclass.
    ///
    /// **Note:** The ownership of *item* is transferred to the layout, and it's
    /// the layout's responsibility to delete it.
    ///
    /// **See also:** [`add_widget()`]
    /// [`BoxLayout::add_layout`]
    /// [`GridLayout::add_layout`]
    pub fn add_item<L: LayoutItemTrait<'a>>(&self, arg0: &L) -> &Self {
        let (obj_arg0_1, _funcs) = arg0.get_layout_item_obj_funcs();

        let (obj_data, funcs) = self.get_layout_obj_funcs();
        unsafe {
            ((*funcs).add_item)(obj_data, obj_arg0_1);
        }
        self
    }
    ///
    /// Removes the widget *widget* from the layout. After this call, it
    /// is the caller's responsibility to give the widget a reasonable
    /// geometry or to put the widget back into a layout or to explicitly
    /// hide it if necessary.
    ///
    /// **Note:** The ownership of *widget* remains the same as
    /// when it was added.
    ///
    /// **See also:** [`remove_item()`]
    /// [`Widget::set_geometry`]
    /// [`add_widget()`]
    pub fn remove_widget<W: WidgetTrait<'a>>(&self, w: &W) -> &Self {
        let (obj_w_1, _funcs) = w.get_widget_obj_funcs();

        let (obj_data, funcs) = self.get_layout_obj_funcs();
        unsafe {
            ((*funcs).remove_widget)(obj_data, obj_w_1);
        }
        self
    }
    ///
    /// Removes the layout item *item* from the layout. It is the
    /// caller's responsibility to delete the item.
    ///
    /// Notice that *item* can be a layout (since QLayout inherits
    /// QLayoutItem).
    ///
    /// **See also:** [`remove_widget()`]
    /// [`add_item()`]
    pub fn remove_item<L: LayoutItemTrait<'a>>(&self, arg0: &L) -> &Self {
        let (obj_arg0_1, _funcs) = arg0.get_layout_item_obj_funcs();

        let (obj_data, funcs) = self.get_layout_obj_funcs();
        unsafe {
            ((*funcs).remove_item)(obj_data, obj_arg0_1);
        }
        self
    }
    ///
    /// Returns whether this layout can make use of more space than
    /// sizeHint(). A value of Qt::Vertical or Qt::Horizontal means that
    /// it wants to grow in only one dimension, whereas Qt::Vertical |
    /// Qt::Horizontal means that it wants to grow in both dimensions.
    ///
    /// The default implementation returns Qt::Horizontal | Qt::Vertical.
    /// Subclasses reimplement it to return a meaningful value based on
    /// their child widgets's [size policies](QSizePolicy)
    ///
    ///
    /// **See also:** [`size_hint()`]
    pub fn expanding_directions(&self) -> Orientations {
        let (obj_data, funcs) = self.get_layout_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).expanding_directions)(obj_data);
            let ret_val = Orientations::from_bits_truncate(ret_val);
            ret_val
        }
    }
    ///
    /// Returns the minimum size of this layout. This is the smallest
    /// size that the layout can have while still respecting the
    /// specifications.
    ///
    /// The returned value doesn't include the space required by
    /// QWidget::setContentsMargins() or menuBar().
    ///
    /// The default implementation allows unlimited resizing.
    pub fn minimum_size(&self) -> Size {
        let (obj_data, funcs) = self.get_layout_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).minimum_size)(obj_data);
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
    /// Returns the maximum size of this layout. This is the largest size
    /// that the layout can have while still respecting the
    /// specifications.
    ///
    /// The returned value doesn't include the space required by
    /// QWidget::setContentsMargins() or menuBar().
    ///
    /// The default implementation allows unlimited resizing.
    pub fn maximum_size(&self) -> Size {
        let (obj_data, funcs) = self.get_layout_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).maximum_size)(obj_data);
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
    /// Must be implemented in subclasses to return the layout item at *index.* If there is no such item, the function must return 0.
    /// Items are numbered consecutively from 0. If an item is deleted, other items will be renumbered.
    ///
    /// This function can be used to iterate over a layout. The following
    /// code will draw a rectangle for each layout item in the layout structure of the widget.
    ///
    /// **See also:** [`count()`]
    /// [`take_at()`]
    pub fn item_at(&self, index: i32) -> Option<LayoutItem> {
        let (obj_data, funcs) = self.get_layout_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).item_at)(obj_data, index);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = LayoutItem::new_from_rc(t);
            } else {
                ret_val = LayoutItem::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    ///
    /// Must be implemented in subclasses to remove the layout item at *index* from the layout, and return the item. If there is no such
    /// item, the function must do nothing and return 0. Items are numbered
    /// consecutively from 0. If an item is removed, other items will be
    /// renumbered.
    ///
    /// The following code fragment shows a safe way to remove all items
    /// from a layout:
    ///
    /// **See also:** [`item_at()`]
    /// [`count()`]
    pub fn take_at(&self, index: i32) -> Option<LayoutItem> {
        let (obj_data, funcs) = self.get_layout_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).take_at)(obj_data, index);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = LayoutItem::new_from_rc(t);
            } else {
                ret_val = LayoutItem::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    ///
    /// Searches for widget *widget* in this layout (not including child
    /// layouts).
    ///
    /// Returns the index of *widget,* or -1 if *widget* is not found.
    ///
    /// The default implementation iterates over all items using itemAt()
    pub fn index_of<W: WidgetTrait<'a>>(&self, arg0: &W) -> i32 {
        let (obj_arg0_1, _funcs) = arg0.get_widget_obj_funcs();

        let (obj_data, funcs) = self.get_layout_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).index_of)(obj_data, obj_arg0_1);
            ret_val
        }
    }
    ///
    /// Must be implemented in subclasses to return the number of items
    /// in the layout.
    ///
    /// **See also:** [`item_at()`]
    pub fn count(&self) -> i32 {
        let (obj_data, funcs) = self.get_layout_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).count)(obj_data);
            ret_val
        }
    }
    ///
    pub fn is_empty(&self) -> bool {
        let (obj_data, funcs) = self.get_layout_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_empty)(obj_data);
            ret_val
        }
    }
    ///
    /// Searches for widget *from* and replaces it with widget *to* if found.
    /// Returns the layout item that contains the widget *from* on success.
    /// Otherwise `0` is returned. If *options* contains `Qt::FindChildrenRecursively`
    /// (the default), sub-layouts are searched for doing the replacement.
    /// Any other flag in *options* is ignored.
    ///
    /// Notice that the returned item therefore might not belong to this layout,
    /// but to a sub-layout.
    ///
    /// The returned layout item is no longer owned by the layout and should be
    /// either deleted or inserted to another layout. The widget *from* is no
    /// longer managed by the layout and may need to be deleted or hidden. The
    /// parent of widget *from* is left unchanged.
    ///
    /// This function works for the built-in Qt layouts, but might not work for
    /// custom layouts.
    ///
    /// **See also:** [`index_of()`]
    pub fn replace_widget<W: WidgetTrait<'a>>(
        &self,
        from: &W,
        to: &W,
        options: FindChildOptions,
    ) -> Option<LayoutItem> {
        let (obj_from_1, _funcs) = from.get_widget_obj_funcs();
        let (obj_to_2, _funcs) = to.get_widget_obj_funcs();
        let enum_options_3 = options.bits();

        let (obj_data, funcs) = self.get_layout_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).replace_widget)(obj_data, obj_from_1, obj_to_2, enum_options_3);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = LayoutItem::new_from_rc(t);
            } else {
                ret_val = LayoutItem::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    pub fn total_height_for_width(&self, w: i32) -> i32 {
        let (obj_data, funcs) = self.get_layout_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).total_height_for_width)(obj_data, w);
            ret_val
        }
    }
    pub fn total_minimum_size(&self) -> Size {
        let (obj_data, funcs) = self.get_layout_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).total_minimum_size)(obj_data);
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
    pub fn total_maximum_size(&self) -> Size {
        let (obj_data, funcs) = self.get_layout_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).total_maximum_size)(obj_data);
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
    pub fn total_size_hint(&self) -> Size {
        let (obj_data, funcs) = self.get_layout_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).total_size_hint)(obj_data);
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
    pub fn layout(&self) -> Option<Layout> {
        let (obj_data, funcs) = self.get_layout_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).layout)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Layout::new_from_rc(t);
            } else {
                ret_val = Layout::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    /// Sets the constraint of the layout.
    pub fn set_size_constraint(&self, constraint: SizeConstraint) -> &Self {
        let enum_constraint_1 = constraint as u32;

        let (obj_data, funcs) = self.get_layout_obj_funcs();
        unsafe {
            ((*funcs).set_size_constraint)(obj_data, enum_constraint_1);
        }
        self
    }
    /// Sets the constraint( of the layout.
    pub fn size_constraint(&self) -> SizeConstraint {
        let (obj_data, funcs) = self.get_layout_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).size_constraint)(obj_data);
            let ret_val = { transmute::<u32, SizeConstraint>(ret_val) };
            ret_val
        }
    }
    ///
    /// Enables this layout if *enable* is true, otherwise disables it.
    ///
    /// An enabled layout adjusts dynamically to changes; a disabled
    /// layout acts as if it did not exist.
    ///
    /// By default all layouts are enabled.
    ///
    /// **See also:** [`is_enabled()`]
    pub fn set_enabled(&self, arg0: bool) -> &Self {
        let (obj_data, funcs) = self.get_layout_obj_funcs();
        unsafe {
            ((*funcs).set_enabled)(obj_data, arg0);
        }
        self
    }
    ///
    /// Returns `true` if the layout is enabled; otherwise returns `false.`
    ///
    /// **See also:** [`set_enabled()`]
    pub fn is_enabled(&self) -> bool {
        let (obj_data, funcs) = self.get_layout_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_enabled)(obj_data);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn size_hint(&self) -> Size {
        let (obj_data, funcs) = self.get_layout_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).size_hint)(obj_data);
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
    #[doc(hidden)]
    pub fn has_height_for_width(&self) -> bool {
        let (obj_data, funcs) = self.get_layout_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).has_height_for_width)(obj_data);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn height_for_width(&self, arg0: i32) -> i32 {
        let (obj_data, funcs) = self.get_layout_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).height_for_width)(obj_data, arg0);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn minimum_height_for_width(&self, arg0: i32) -> i32 {
        let (obj_data, funcs) = self.get_layout_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).minimum_height_for_width)(obj_data, arg0);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn widget(&self) -> Option<Widget> {
        let (obj_data, funcs) = self.get_layout_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).widget)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Widget::new_from_rc(t);
            } else {
                ret_val = Widget::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    #[doc(hidden)]
    pub fn spacer_item(&self) -> Option<SpacerItem> {
        let (obj_data, funcs) = self.get_layout_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).spacer_item)(obj_data);
            if ret_val.qt_data == ::std::ptr::null() {
                return None;
            }
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = SpacerItem::new_from_rc(t);
            } else {
                ret_val = SpacerItem::new_from_owned(t);
            }
            Some(ret_val)
        }
    }
    #[doc(hidden)]
    pub fn alignment(&self) -> Alignment {
        let (obj_data, funcs) = self.get_layout_item_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).alignment)(obj_data);
            let ret_val = Alignment::from_bits_truncate(ret_val);
            ret_val
        }
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
        let enum_timer_type_2 = timer_type as u32;

        let (obj_data, funcs) = self.get_object_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).start_timer)(obj_data, interval, enum_timer_type_2);
            ret_val
        }
    }
    #[doc(hidden)]
    pub fn start_timer_2(&self, time: u32, timer_type: TimerType) -> i32 {
        let enum_timer_type_2 = timer_type as u32;

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

    pub fn build(&self) -> Self {
        self.clone()
    }
}
pub trait LayoutTrait<'a> {
    #[inline]
    #[doc(hidden)]
    fn get_layout_obj_funcs(&self) -> (*const RUBase, *const RULayoutFuncs);
}

impl<'a> ObjectTrait<'a> for Layout<'a> {
    #[doc(hidden)]
    fn get_object_obj_funcs(&self) -> (*const RUBase, *const RUObjectFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).object_funcs) }
    }
}

impl<'a> LayoutItemTrait<'a> for Layout<'a> {
    #[doc(hidden)]
    fn get_layout_item_obj_funcs(&self) -> (*const RUBase, *const RULayoutItemFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).layout_item_funcs) }
    }
}

impl<'a> LayoutTrait<'a> for Layout<'a> {
    #[doc(hidden)]
    fn get_layout_obj_funcs(&self) -> (*const RUBase, *const RULayoutFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).layout_funcs) }
    }
}
#[repr(u32)]
pub enum SizeConstraint {
    SetDefaultConstraint = 0,
    SetNoConstraint = 1,
    SetMinimumSize = 2,
    SetFixedSize = 3,
    SetMaximumSize = 4,
    SetMinAndMaxSize = 5,
}
