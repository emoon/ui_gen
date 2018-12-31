////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// This file is auto-generated by rute_gen. DO NOT EDIT
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#include "../rute_base.h"
#include "../rute_manual.h"
#include <QHBoxLayout>
#include "h_box_layout_ffi.h"

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUHBoxLayout create_h_box_layout(
    struct RUBase* priv_data,
    RUDeleteCallback delete_callback,
    void* private_user_data)
{
    auto ctl = generic_create_func_with_delete<struct RUHBoxLayout, WRHBoxLayout>(priv_data, delete_callback, private_user_data);
    ctl.all_funcs = &s_h_box_layout_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_h_box_layout(struct RUBase* priv_data) {
    destroy_generic<WRHBoxLayout>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUHBoxLayoutFuncs s_h_box_layout_funcs = {
    destroy_h_box_layout,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUHBoxLayoutAllFuncs s_h_box_layout_all_funcs = {
    &s_object_funcs,
    &s_layout_item_funcs,
    &s_layout_funcs,
    &s_box_layout_funcs,
    &s_h_box_layout_funcs,
};
