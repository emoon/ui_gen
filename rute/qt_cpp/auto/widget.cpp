////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// This file is auto-generated by rute_gen. DO NOT EDIT
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#include "../rute_base.h"
#include "../rute_manual.h"
#include <QWidget>

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void widget_show(struct RUBase* self_c) {
    WRWidget* qt_value = (WRWidget*)self_c;

    qt_value->show();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void widget_set_fixed_height(struct RUBase* self_c, int width) {
    WRWidget* qt_value = (WRWidget*)self_c;

    qt_value->setFixedHeight(width);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void widget_set_fixed_width(struct RUBase* self_c, int width) {
    WRWidget* qt_value = (WRWidget*)self_c;

    qt_value->setFixedWidth(width);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void widget_resize(struct RUBase* self_c, int width, int height) {
    WRWidget* qt_value = (WRWidget*)self_c;

    qt_value->resize(width, height);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void widget_set_parent(struct RUBase* self_c, struct RUBase* parent) {
    WRWidget* qt_value = (WRWidget*)self_c;

    qt_value->setParent((QWidget*)parent);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void widget_update(struct RUBase* self_c) {
    WRWidget* qt_value = (WRWidget*)self_c;

    qt_value->update();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_widget_window_title_changed_event(void* object, void* user_data, void* trampoline_func, void (*event)(void* self_c, const char* title)) {
    QSlotWrapperSignal_self_string_void* wrap = new QSlotWrapperSignal_self_string_void(user_data, (Signal_self_string_void)trampoline_func, (void*)event);
    QObject* q_obj = (QObject*)object;
    QObject::connect(q_obj, SIGNAL(windowTitleChanged(QString)), wrap, SLOT(method(QString)));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUWidget create_widget(
    struct RUBase* priv_data,
    RUDeleteCallback delete_callback,
    void* private_user_data)
{
    auto ctl = create_widget_func<struct RUWidget, WRWidget>(priv_data, delete_callback, private_user_data);
    ctl.all_funcs = &s_widget_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_widget(struct RUBase* priv_data) {
    destroy_generic<WRWidget>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUWidgetFuncs s_widget_funcs = {
    destroy_widget,
    widget_show,
    widget_set_fixed_height,
    widget_set_fixed_width,
    widget_resize,
    widget_set_parent,
    widget_update,
    set_widget_window_title_changed_event,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUWidgetAllFuncs s_widget_all_funcs = {
    &s_widget_funcs,
};

