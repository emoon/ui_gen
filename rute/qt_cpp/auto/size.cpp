////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// This file is auto-generated by rute_gen. DO NOT EDIT
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#include "../rute_base.h"
#include "../rute_manual.h"
#include <QSize>

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool size_is_null(struct RUBase* self_c) {
    WRSize* qt_value = (WRSize*)self_c;
    auto ret_value = qt_value->isNull();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool size_is_empty(struct RUBase* self_c) {
    WRSize* qt_value = (WRSize*)self_c;
    auto ret_value = qt_value->isEmpty();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool size_is_valid(struct RUBase* self_c) {
    WRSize* qt_value = (WRSize*)self_c;
    auto ret_value = qt_value->isValid();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int size_width(struct RUBase* self_c) {
    WRSize* qt_value = (WRSize*)self_c;
    auto ret_value = qt_value->width();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int size_height(struct RUBase* self_c) {
    WRSize* qt_value = (WRSize*)self_c;
    auto ret_value = qt_value->height();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void size_set_width(struct RUBase* self_c, int w) {
    WRSize* qt_value = (WRSize*)self_c;
    qt_value->setWidth(w);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void size_set_height(struct RUBase* self_c, int h) {
    WRSize* qt_value = (WRSize*)self_c;
    qt_value->setHeight(h);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void size_scale(struct RUBase* self_c, int w, int h, int mode) {
    WRSize* qt_value = (WRSize*)self_c;
    qt_value->scale(w, h, (Qt::AspectRatioMode)s_aspect_ratio_mode_lookup[mode]);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void size_scale_by_size(struct RUBase* self_c, struct RUBase* s, int mode) {
    WRSize* qt_value = (WRSize*)self_c;
    qt_value->scale(*((QSize*)s), (Qt::AspectRatioMode)s_aspect_ratio_mode_lookup[mode]);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUSize size_scaled(struct RUBase* self_c, int w, int h, int mode) {
    WRSize* qt_value = (WRSize*)self_c;
    auto ret_value = qt_value->scaled(w, h, (Qt::AspectRatioMode)s_aspect_ratio_mode_lookup[mode]);
    WRSize* new_val = new WRSize();
    *new_val = ret_value;
    struct RUSize ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_size_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUSize size_scaled_by_size(struct RUBase* self_c, struct RUBase* s, int mode) {
    WRSize* qt_value = (WRSize*)self_c;
    auto ret_value = qt_value->scaled(*((QSize*)s), (Qt::AspectRatioMode)s_aspect_ratio_mode_lookup[mode]);
    WRSize* new_val = new WRSize();
    *new_val = ret_value;
    struct RUSize ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_size_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUSize size_expanded_to(struct RUBase* self_c, struct RUBase* arg0) {
    WRSize* qt_value = (WRSize*)self_c;
    auto ret_value = qt_value->expandedTo(*((QSize*)arg0));
    WRSize* new_val = new WRSize();
    *new_val = ret_value;
    struct RUSize ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_size_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUSize size_bounded_to(struct RUBase* self_c, struct RUBase* arg0) {
    WRSize* qt_value = (WRSize*)self_c;
    auto ret_value = qt_value->boundedTo(*((QSize*)arg0));
    WRSize* new_val = new WRSize();
    *new_val = ret_value;
    struct RUSize ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_size_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUSize create_size(
    struct RUBase* priv_data,
    RUDeleteCallback delete_callback,
    void* private_user_data)
{
    auto ctl = generic_create_func_with_delete<struct RUSize, WRSize>(priv_data, delete_callback, private_user_data);
    ctl.all_funcs = &s_size_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_size(struct RUBase* priv_data) {
    destroy_generic<WRSize>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUSizeFuncs s_size_funcs = {
    destroy_size,
    size_is_null,
    size_is_empty,
    size_is_valid,
    size_width,
    size_height,
    size_set_width,
    size_set_height,
    size_scale,
    size_scale_by_size,
    size_scaled,
    size_scaled_by_size,
    size_expanded_to,
    size_bounded_to,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUSizeAllFuncs s_size_all_funcs = {
    &s_size_funcs,
};

