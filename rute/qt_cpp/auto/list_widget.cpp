////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// This file is auto-generated by rute_gen. DO NOT EDIT
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#include "../rute_base.h"
#include "../rute_manual.h"
#include <QListWidget>
#include "list_widget_ffi.h"

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_add_item(struct RUBase* self_c, const char* label) {
    WRListWidget* qt_value = (WRListWidget*)self_c;
    qt_value->addItem(QString::fromUtf8(label));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_clear(struct RUBase* self_c) {
    WRListWidget* qt_value = (WRListWidget*)self_c;
    qt_value->clear();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUArray list_widget_selected_items(struct RUBase* self_c) {
    WRListWidget* qt_value = (WRListWidget*)self_c;
    auto ret_value = qt_value->selectedItems();
    RUArray ru_array = alloc_rc_array(ret_value.size());
    ru_array.count = (uint32_t)ret_value.size();
    ru_array.all_funcs = (void*)&s_list_widget_item_all_funcs;
    struct RUBase** ru_dest = (struct RUBase**)ru_array.elements;
    uint8_t* owned = ru_array.owners;
    for (int i = 0, len = (int)ret_value.size(); i < len; ++i) {
        struct RUBase* temp = (struct RUBase*)ret_value.at(i);
        struct RUBase* host_data = (struct RUBase*)s_host_data_lookup[(void*)temp];
        if (host_data) {
            *ru_dest++ = host_data;
            *owned++ = 1;
        } else {
            *ru_dest++ = temp;
            *owned++ = 0;
        }
    }
    return ru_array;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_add_widget_item(struct RUBase* self_c, struct RUBase* item) {
    WRListWidget* qt_value = (WRListWidget*)self_c;
    qt_value->addItem((WRListWidgetItem*)item);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUListWidget create_list_widget(
    struct RUBase* priv_data,
    RUDeleteCallback delete_callback,
    void* private_user_data)
{
    auto ctl = create_widget_func<struct RUListWidget, WRListWidget>(priv_data, delete_callback, private_user_data);
    ctl.all_funcs = &s_list_widget_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_list_widget(struct RUBase* priv_data) {
    destroy_generic<WRListWidget>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUListWidgetFuncs s_list_widget_funcs = {
    destroy_list_widget,
    list_widget_add_item,
    list_widget_clear,
    list_widget_selected_items,
    list_widget_add_widget_item,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUListWidgetAllFuncs s_list_widget_all_funcs = {
    &s_object_funcs,
    &s_paint_device_funcs,
    &s_widget_funcs,
    &s_list_widget_funcs,
};

