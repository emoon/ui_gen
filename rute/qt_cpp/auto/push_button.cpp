////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// This file is auto-generated by rute_gen. DO NOT EDIT
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#include "../rute_base.h"
#include "../rute_manual.h"
#include <QPushButton>

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool push_button_auto_default(struct RUBase* self_c) {
    WRPushButton* qt_value = (WRPushButton*)self_c;

    auto ret_value = qt_value->autoDefault();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void push_button_set_auto_default(struct RUBase* self_c, bool arg0) {
    WRPushButton* qt_value = (WRPushButton*)self_c;

    qt_value->setAutoDefault(arg0);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool push_button_is_default(struct RUBase* self_c) {
    WRPushButton* qt_value = (WRPushButton*)self_c;

    auto ret_value = qt_value->isDefault();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void push_button_set_default(struct RUBase* self_c, bool arg0) {
    WRPushButton* qt_value = (WRPushButton*)self_c;

    qt_value->setDefault(arg0);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void push_button_set_flat(struct RUBase* self_c, bool arg0) {
    WRPushButton* qt_value = (WRPushButton*)self_c;

    qt_value->setFlat(arg0);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool push_button_is_flat(struct RUBase* self_c) {
    WRPushButton* qt_value = (WRPushButton*)self_c;

    auto ret_value = qt_value->isFlat();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void push_button_show_menu(struct RUBase* self_c) {
    WRPushButton* qt_value = (WRPushButton*)self_c;

    qt_value->showMenu();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void push_button_set_text(struct RUBase* self_c, const char* text) {
    WRPushButton* qt_value = (WRPushButton*)self_c;

    qt_value->setText(QString::fromUtf8(text));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static const char* push_button_text(struct RUBase* self_c) {
    WRPushButton* qt_value = (WRPushButton*)self_c;

    auto ret_value = qt_value->text();
    return q_string_to_const_char(ret_value);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_push_button_pressed_event(void* object, void* user_data, void* trampoline_func, void (*event)(void* self_c)) {
    QSlotWrapperSignal_self_void* wrap = new QSlotWrapperSignal_self_void(user_data, (Signal_self_void)trampoline_func, (void*)event);
    QObject* q_obj = (QObject*)object;
    QObject::connect(q_obj, SIGNAL(pressed()), wrap, SLOT(method()));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_push_button_released_event(void* object, void* user_data, void* trampoline_func, void (*event)(void* self_c)) {
    QSlotWrapperSignal_self_void* wrap = new QSlotWrapperSignal_self_void(user_data, (Signal_self_void)trampoline_func, (void*)event);
    QObject* q_obj = (QObject*)object;
    QObject::connect(q_obj, SIGNAL(released()), wrap, SLOT(method()));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_push_button_clicked_event(void* object, void* user_data, void* trampoline_func, void (*event)(void* self_c, bool checked)) {
    QSlotWrapperSignal_self_bool_void* wrap = new QSlotWrapperSignal_self_bool_void(user_data, (Signal_self_bool_void)trampoline_func, (void*)event);
    QObject* q_obj = (QObject*)object;
    QObject::connect(q_obj, SIGNAL(clicked(bool)), wrap, SLOT(method(bool)));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_push_button_toggled_event(void* object, void* user_data, void* trampoline_func, void (*event)(void* self_c, bool checked)) {
    QSlotWrapperSignal_self_bool_void* wrap = new QSlotWrapperSignal_self_bool_void(user_data, (Signal_self_bool_void)trampoline_func, (void*)event);
    QObject* q_obj = (QObject*)object;
    QObject::connect(q_obj, SIGNAL(toggled(bool)), wrap, SLOT(method(bool)));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPushButton create_push_button(
    struct RUBase* priv_data,
    RUDeleteCallback delete_callback,
    void* private_user_data)
{
    auto ctl = create_widget_func<struct RUPushButton, WRPushButton>(priv_data, delete_callback, private_user_data);
    ctl.all_funcs = &s_push_button_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_push_button(struct RUBase* priv_data) {
    destroy_generic<WRPushButton>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUPushButtonFuncs s_push_button_funcs = {
    destroy_push_button,
    push_button_auto_default,
    push_button_set_auto_default,
    push_button_is_default,
    push_button_set_default,
    push_button_set_flat,
    push_button_is_flat,
    push_button_show_menu,
    push_button_set_text,
    push_button_text,
    set_push_button_pressed_event,
    set_push_button_released_event,
    set_push_button_clicked_event,
    set_push_button_toggled_event,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUPushButtonAllFuncs s_push_button_all_funcs = {
    &s_widget_funcs,
    &s_push_button_funcs,
};
