////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// This file is auto-generated by rute_gen. DO NOT EDIT
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#include "../rute_base.h"
#include "../rute_manual.h"
#include <QPaintDevice>
#include "paint_device_ffi.h"

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool paint_device_painting_active(struct RUBase* self_c) {
    WRPaintDevice* qt_value = (WRPaintDevice*)self_c;
    auto ret_value = qt_value->paintingActive();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPaintEngine paint_device_paint_engine(struct RUBase* self_c) {
    WRPaintDevice* qt_value = (WRPaintDevice*)self_c;
    auto ret_value = qt_value->paintEngine();
    struct RUPaintEngine ctl;
    ctl.qt_data = (struct RUBase*)ret_value;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)ret_value];
    ctl.all_funcs = &s_paint_engine_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int paint_device_width(struct RUBase* self_c) {
    WRPaintDevice* qt_value = (WRPaintDevice*)self_c;
    auto ret_value = qt_value->width();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int paint_device_height(struct RUBase* self_c) {
    WRPaintDevice* qt_value = (WRPaintDevice*)self_c;
    auto ret_value = qt_value->height();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int paint_device_logical_dpi_x(struct RUBase* self_c) {
    WRPaintDevice* qt_value = (WRPaintDevice*)self_c;
    auto ret_value = qt_value->logicalDpiX();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int paint_device_logical_dpi_y(struct RUBase* self_c) {
    WRPaintDevice* qt_value = (WRPaintDevice*)self_c;
    auto ret_value = qt_value->logicalDpiY();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int paint_device_physical_dpi_x(struct RUBase* self_c) {
    WRPaintDevice* qt_value = (WRPaintDevice*)self_c;
    auto ret_value = qt_value->physicalDpiX();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int paint_device_physical_dpi_y(struct RUBase* self_c) {
    WRPaintDevice* qt_value = (WRPaintDevice*)self_c;
    auto ret_value = qt_value->physicalDpiY();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int paint_device_device_pixel_ratio(struct RUBase* self_c) {
    WRPaintDevice* qt_value = (WRPaintDevice*)self_c;
    auto ret_value = qt_value->devicePixelRatio();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float paint_device_device_pixel_ratio_f(struct RUBase* self_c) {
    WRPaintDevice* qt_value = (WRPaintDevice*)self_c;
    auto ret_value = qt_value->devicePixelRatioF();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int paint_device_color_count(struct RUBase* self_c) {
    WRPaintDevice* qt_value = (WRPaintDevice*)self_c;
    auto ret_value = qt_value->colorCount();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int paint_device_depth(struct RUBase* self_c) {
    WRPaintDevice* qt_value = (WRPaintDevice*)self_c;
    auto ret_value = qt_value->depth();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUPaintDeviceFuncs s_paint_device_funcs = {
    paint_device_painting_active,
    paint_device_paint_engine,
    paint_device_width,
    paint_device_height,
    paint_device_logical_dpi_x,
    paint_device_logical_dpi_y,
    paint_device_physical_dpi_x,
    paint_device_physical_dpi_y,
    paint_device_device_pixel_ratio,
    paint_device_device_pixel_ratio_f,
    paint_device_color_count,
    paint_device_depth,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUPaintDeviceAllFuncs s_paint_device_all_funcs = {
    &s_paint_device_funcs,
};

