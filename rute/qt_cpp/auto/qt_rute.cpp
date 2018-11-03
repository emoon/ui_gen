// This file is auto-generated by rute_gen. DO NOT EDIT!

#include <map>
#include <stdint.h>
#include "rute.h"

static char* s_temp_string_buffer = nullptr;
static int s_largest_string_size = 0;
std::map<void*, void*> s_host_data_lookup;

const char* q_string_to_const_char(const QString& ret_value) {
    QByteArray ba = ret_value.toUtf8();
    const char* c_str = ba.data();

    int size = ba.size() + 1;

    if (size > s_largest_string_size) {
        delete [] s_temp_string_buffer;
        s_temp_string_buffer = new char[size];
        s_largest_string_size = size;
    }

    memcpy(s_temp_string_buffer, c_str, size);
    return s_temp_string_buffer;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RuteFFI s_rute = {

    create_application,
    get_application,
    create_backing_store,
    create_brush,
    create_color,
    get_color,
    create_font,
    create_gradient,
    create_image,
    get_image,
    create_list_widget,
    create_list_widget_item,
    create_margins,
    create_paint_device,
    get_paint_device,
    create_paint_engine,
    create_paint_engine_state,
    create_painter,
    get_painter,
    create_pixel_format,
    get_pixel_format,
    create_point,
    get_point,
    create_push_button,
    create_rect,
    create_size,
    create_widget,
    get_widget,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

//#ifdef _WIN32
//extern "C" __declspec(dllexport) struct RuteFFI* rute_static_ffi_get() {
//#else
extern "C" struct RuteFFI* rute_static_ffi_get() {
//#endif
    return (RuteFFI*)&s_rute;
}

