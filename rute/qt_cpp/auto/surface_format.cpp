////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// This file is auto-generated by rute_gen. DO NOT EDIT
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#include "../rute_base.h"
#include "../rute_manual.h"
#include <QSurfaceFormat>
#include "surface_format_ffi.h"

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void surface_format_set_depth_buffer_size(struct RUBase* self_c, int size) {
    WRSurfaceFormat* qt_value = (WRSurfaceFormat*)self_c;
    qt_value->setDepthBufferSize(size);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int surface_format_depth_buffer_size(struct RUBase* self_c) {
    WRSurfaceFormat* qt_value = (WRSurfaceFormat*)self_c;
    auto ret_value = qt_value->depthBufferSize();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void surface_format_set_stencil_buffer_size(struct RUBase* self_c, int size) {
    WRSurfaceFormat* qt_value = (WRSurfaceFormat*)self_c;
    qt_value->setStencilBufferSize(size);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int surface_format_stencil_buffer_size(struct RUBase* self_c) {
    WRSurfaceFormat* qt_value = (WRSurfaceFormat*)self_c;
    auto ret_value = qt_value->stencilBufferSize();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void surface_format_set_red_buffer_size(struct RUBase* self_c, int size) {
    WRSurfaceFormat* qt_value = (WRSurfaceFormat*)self_c;
    qt_value->setRedBufferSize(size);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int surface_format_red_buffer_size(struct RUBase* self_c) {
    WRSurfaceFormat* qt_value = (WRSurfaceFormat*)self_c;
    auto ret_value = qt_value->redBufferSize();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void surface_format_set_green_buffer_size(struct RUBase* self_c, int size) {
    WRSurfaceFormat* qt_value = (WRSurfaceFormat*)self_c;
    qt_value->setGreenBufferSize(size);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int surface_format_green_buffer_size(struct RUBase* self_c) {
    WRSurfaceFormat* qt_value = (WRSurfaceFormat*)self_c;
    auto ret_value = qt_value->greenBufferSize();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void surface_format_set_blue_buffer_size(struct RUBase* self_c, int size) {
    WRSurfaceFormat* qt_value = (WRSurfaceFormat*)self_c;
    qt_value->setBlueBufferSize(size);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int surface_format_blue_buffer_size(struct RUBase* self_c) {
    WRSurfaceFormat* qt_value = (WRSurfaceFormat*)self_c;
    auto ret_value = qt_value->blueBufferSize();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void surface_format_set_alpha_buffer_size(struct RUBase* self_c, int size) {
    WRSurfaceFormat* qt_value = (WRSurfaceFormat*)self_c;
    qt_value->setAlphaBufferSize(size);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int surface_format_alpha_buffer_size(struct RUBase* self_c) {
    WRSurfaceFormat* qt_value = (WRSurfaceFormat*)self_c;
    auto ret_value = qt_value->alphaBufferSize();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void surface_format_set_samples(struct RUBase* self_c, int num_samples) {
    WRSurfaceFormat* qt_value = (WRSurfaceFormat*)self_c;
    qt_value->setSamples(num_samples);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int surface_format_samples(struct RUBase* self_c) {
    WRSurfaceFormat* qt_value = (WRSurfaceFormat*)self_c;
    auto ret_value = qt_value->samples();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void surface_format_set_swap_behavior(struct RUBase* self_c, uint32_t behavior) {
    WRSurfaceFormat* qt_value = (WRSurfaceFormat*)self_c;
    qt_value->setSwapBehavior((QSurfaceFormat::SwapBehavior)behavior);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static uint32_t surface_format_swap_behavior(struct RUBase* self_c) {
    WRSurfaceFormat* qt_value = (WRSurfaceFormat*)self_c;
    auto ret_value = qt_value->swapBehavior();
    return (uint32_t)ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool surface_format_has_alpha(struct RUBase* self_c) {
    WRSurfaceFormat* qt_value = (WRSurfaceFormat*)self_c;
    auto ret_value = qt_value->hasAlpha();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void surface_format_set_profile(struct RUBase* self_c, uint32_t profile) {
    WRSurfaceFormat* qt_value = (WRSurfaceFormat*)self_c;
    qt_value->setProfile((QSurfaceFormat::OpenGLContextProfile)profile);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static uint32_t surface_format_profile(struct RUBase* self_c) {
    WRSurfaceFormat* qt_value = (WRSurfaceFormat*)self_c;
    auto ret_value = qt_value->profile();
    return (uint32_t)ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void surface_format_set_renderable_type(struct RUBase* self_c, uint32_t stype) {
    WRSurfaceFormat* qt_value = (WRSurfaceFormat*)self_c;
    qt_value->setRenderableType((QSurfaceFormat::RenderableType)stype);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static uint32_t surface_format_renderable_type(struct RUBase* self_c) {
    WRSurfaceFormat* qt_value = (WRSurfaceFormat*)self_c;
    auto ret_value = qt_value->renderableType();
    return (uint32_t)ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void surface_format_set_major_version(struct RUBase* self_c, int major_version) {
    WRSurfaceFormat* qt_value = (WRSurfaceFormat*)self_c;
    qt_value->setMajorVersion(major_version);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int surface_format_major_version(struct RUBase* self_c) {
    WRSurfaceFormat* qt_value = (WRSurfaceFormat*)self_c;
    auto ret_value = qt_value->majorVersion();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void surface_format_set_minor_version(struct RUBase* self_c, int minor_version) {
    WRSurfaceFormat* qt_value = (WRSurfaceFormat*)self_c;
    qt_value->setMinorVersion(minor_version);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int surface_format_minor_version(struct RUBase* self_c) {
    WRSurfaceFormat* qt_value = (WRSurfaceFormat*)self_c;
    auto ret_value = qt_value->minorVersion();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void surface_format_set_version(struct RUBase* self_c, int major, int minor) {
    WRSurfaceFormat* qt_value = (WRSurfaceFormat*)self_c;
    qt_value->setVersion(major, minor);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool surface_format_stereo(struct RUBase* self_c) {
    WRSurfaceFormat* qt_value = (WRSurfaceFormat*)self_c;
    auto ret_value = qt_value->stereo();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void surface_format_set_stereo(struct RUBase* self_c, bool enable) {
    WRSurfaceFormat* qt_value = (WRSurfaceFormat*)self_c;
    qt_value->setStereo(enable);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void surface_format_set_option(struct RUBase* self_c, uint32_t opt) {
    WRSurfaceFormat* qt_value = (WRSurfaceFormat*)self_c;
    qt_value->setOption((QSurfaceFormat::FormatOptions)opt);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool surface_format_test_option(struct RUBase* self_c, uint32_t opt) {
    WRSurfaceFormat* qt_value = (WRSurfaceFormat*)self_c;
    auto ret_value = qt_value->testOption((QSurfaceFormat::FormatOptions)opt);
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void surface_format_set_options(struct RUBase* self_c, uint32_t options) {
    WRSurfaceFormat* qt_value = (WRSurfaceFormat*)self_c;
    qt_value->setOptions((QSurfaceFormat::FormatOptions)options);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void surface_format_set_option_2(struct RUBase* self_c, uint32_t option, bool on) {
    WRSurfaceFormat* qt_value = (WRSurfaceFormat*)self_c;
    qt_value->setOption((QSurfaceFormat::FormatOption)option, on);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool surface_format_test_option_2(struct RUBase* self_c, uint32_t option) {
    WRSurfaceFormat* qt_value = (WRSurfaceFormat*)self_c;
    auto ret_value = qt_value->testOption((QSurfaceFormat::FormatOption)option);
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static uint32_t surface_format_options(struct RUBase* self_c) {
    WRSurfaceFormat* qt_value = (WRSurfaceFormat*)self_c;
    auto ret_value = qt_value->options();
    return (uint32_t)ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int surface_format_swap_interval(struct RUBase* self_c) {
    WRSurfaceFormat* qt_value = (WRSurfaceFormat*)self_c;
    auto ret_value = qt_value->swapInterval();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void surface_format_set_swap_interval(struct RUBase* self_c, int interval) {
    WRSurfaceFormat* qt_value = (WRSurfaceFormat*)self_c;
    qt_value->setSwapInterval(interval);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static uint32_t surface_format_color_space(struct RUBase* self_c) {
    WRSurfaceFormat* qt_value = (WRSurfaceFormat*)self_c;
    auto ret_value = qt_value->colorSpace();
    return (uint32_t)ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void surface_format_set_color_space(struct RUBase* self_c, uint32_t color_space) {
    WRSurfaceFormat* qt_value = (WRSurfaceFormat*)self_c;
    qt_value->setColorSpace((QSurfaceFormat::ColorSpace)color_space);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void surface_format_set_default_format(struct RUBase* self_c, struct RUBase* format) {
    WRSurfaceFormat* qt_value = (WRSurfaceFormat*)self_c;
    qt_value->setDefaultFormat(*((WRSurfaceFormat*)format));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUSurfaceFormat surface_format_default_format(struct RUBase* self_c) {
    WRSurfaceFormat* qt_value = (WRSurfaceFormat*)self_c;
    auto ret_value = qt_value->defaultFormat();
    WRSurfaceFormat* new_val = new WRSurfaceFormat();
    *new_val = ret_value;
    struct RUSurfaceFormat ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_surface_format_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUSurfaceFormat create_surface_format(
    struct RUBase* priv_data,
    RUDeleteCallback delete_callback,
    void* private_user_data)
{
    auto ctl = generic_create_func_with_delete<struct RUSurfaceFormat, WRSurfaceFormat>(priv_data, delete_callback, private_user_data);
    ctl.all_funcs = &s_surface_format_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_surface_format(struct RUBase* priv_data) {
    destroy_generic<WRSurfaceFormat>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUSurfaceFormat get_surface_format(struct RUBase* priv_data) {
    (void)priv_data;
    RUSurfaceFormat ctl;
    ctl.qt_data = nullptr;
    ctl.host_data = nullptr;
    ctl.all_funcs = &s_surface_format_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUSurfaceFormatFuncs s_surface_format_funcs = {
    destroy_surface_format,
    surface_format_set_depth_buffer_size,
    surface_format_depth_buffer_size,
    surface_format_set_stencil_buffer_size,
    surface_format_stencil_buffer_size,
    surface_format_set_red_buffer_size,
    surface_format_red_buffer_size,
    surface_format_set_green_buffer_size,
    surface_format_green_buffer_size,
    surface_format_set_blue_buffer_size,
    surface_format_blue_buffer_size,
    surface_format_set_alpha_buffer_size,
    surface_format_alpha_buffer_size,
    surface_format_set_samples,
    surface_format_samples,
    surface_format_set_swap_behavior,
    surface_format_swap_behavior,
    surface_format_has_alpha,
    surface_format_set_profile,
    surface_format_profile,
    surface_format_set_renderable_type,
    surface_format_renderable_type,
    surface_format_set_major_version,
    surface_format_major_version,
    surface_format_set_minor_version,
    surface_format_minor_version,
    surface_format_set_version,
    surface_format_stereo,
    surface_format_set_stereo,
    surface_format_set_option,
    surface_format_test_option,
    surface_format_set_options,
    surface_format_set_option_2,
    surface_format_test_option_2,
    surface_format_options,
    surface_format_swap_interval,
    surface_format_set_swap_interval,
    surface_format_color_space,
    surface_format_set_color_space,
    surface_format_set_default_format,
    surface_format_default_format,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUSurfaceFormatAllFuncs s_surface_format_all_funcs = {
    &s_surface_format_funcs,
};

