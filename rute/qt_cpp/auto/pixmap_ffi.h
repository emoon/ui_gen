// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once

#include <stdint.h>
#include <stdbool.h>

#include "../rute_base.h"

#ifdef __cplusplus
extern "C" {
#endif
#include "bitmap_ffi.h"
#include "image_ffi.h"
#include "paint_engine_ffi.h"
#include "pixmap_ffi.h"
#include "rect_ffi.h"
#include "size_ffi.h"

struct RUPixmapFuncs;
struct RUPixmap;

typedef struct RUPixmapFuncs {
    void (*destroy)(struct RUBase* self);
    void (*swap)(struct RUBase* self_c, struct RUBase* other);
    bool (*is_null)(struct RUBase* self_c);
    void (*set_dev_type)(void* object, void* user_data, void* wrapped_func, void (*event)(void*, void* self_c));
    void (*remove_dev_type)(void* object);
    int (*width)(struct RUBase* self_c);
    int (*height)(struct RUBase* self_c);
    struct RUSize (*size)(struct RUBase* self_c);
    struct RURect (*rect)(struct RUBase* self_c);
    int (*depth)(struct RUBase* self_c);
    int (*default_depth)(struct RUBase* self_c);
    void (*fill_by_color_type)(struct RUBase* self_c, struct RUBase* fill_color);
    void (*fill_by_device)(struct RUBase* self_c, struct RUBase* device, struct RUBase* ofs);
    void (*fill_by_device_offset)(struct RUBase* self_c, struct RUBase* device, int xofs, int yofs);
    struct RUBitmap (*mask)(struct RUBase* self_c);
    void (*set_mask)(struct RUBase* self_c, struct RUBase* arg0);
    float (*device_pixel_ratio)(struct RUBase* self_c);
    void (*set_device_pixel_ratio)(struct RUBase* self_c, float scale_factor);
    bool (*has_alpha)(struct RUBase* self_c);
    bool (*has_alpha_channel)(struct RUBase* self_c);
    struct RUBitmap (*create_heuristic_mask)(struct RUBase* self_c, bool clip_tight);
    struct RUBitmap (*create_mask_from_color)(struct RUBase* self_c, struct RUBase* mask_color, int mode);
    struct RUPixmap (*grab_window)(struct RUBase* self_c, struct RUWId arg0, int x, int y, int w, int h);
    struct RUPixmap (*grab_widget_by_rect)(struct RUBase* self_c, struct RUBase* widget, struct RUBase* rect);
    struct RUPixmap (*grab_widget)(struct RUBase* self_c, struct RUBase* widget, int x, int y, int w, int h);
    struct RUPixmap (*scaled)(struct RUBase* self_c, int w, int h, int aspect_mode, int mode);
    struct RUPixmap (*scaled_by_size)(struct RUBase* self_c, struct RUBase* s, int aspect_mode, int mode);
    struct RUPixmap (*scaled_to_width)(struct RUBase* self_c, int w, int mode);
    struct RUPixmap (*scaled_to_height)(struct RUBase* self_c, int h, int mode);
    struct RUImage (*to_image)(struct RUBase* self_c);
    struct RUPixmap (*from_image_reader)(struct RUBase* self_c, struct RUBase* image_reader, int flags);
    struct RUPixmap (*from_image)(struct RUBase* self_c, struct RUBase* image, int flags);
    bool (*load)(struct RUBase* self_c, const char* file_name, struct RUBase* format, int flags);
    bool (*load_from_data)(struct RUBase* self_c, struct RUBase* buf, struct RUuint len, struct RUBase* format, int flags);
    bool (*save)(struct RUBase* self_c, const char* file_name, struct RUBase* format, int quality);
    bool (*save_by_io_device)(struct RUBase* self_c, struct RUBase* device, struct RUBase* format, int quality);
    bool (*convert_from_image)(struct RUBase* self_c, struct RUBase* img, int flags);
    struct RUPixmap (*copy)(struct RUBase* self_c, int x, int y, int width, int height);
    struct RUPixmap (*copy_by_rect)(struct RUBase* self_c, struct RUBase* rect);
    void (*scroll)(struct RUBase* self_c, int dx, int dy, int x, int y, int width, int height, struct RUBase* exposed);
    void (*scroll_by_rect)(struct RUBase* self_c, int dx, int dy, struct RUBase* rect, struct RUBase* exposed);
    int64_t (*cache_key)(struct RUBase* self_c);
    bool (*is_detached)(struct RUBase* self_c);
    void (*detach)(struct RUBase* self_c);
    bool (*is_q_bitmap)(struct RUBase* self_c);
    void (*set_paint_engine)(void* object, void* user_data, void* wrapped_func, void (*event)(void*, void* self_c));
    void (*remove_paint_engine)(void* object);
    struct RUPixmap (*from_image_in_place)(struct RUBase* self_c, struct RUBase* image, int flags);
} RUPixmapFuncs;

typedef struct RUPixmapAllFuncs {
    struct RUPaintDeviceFuncs* paint_device_funcs;
    struct RUPixmapFuncs* pixmap_funcs;
} RUPixmapAllFuncs;

typedef struct RUPixmap {
    RUBase* qt_data;
    RUBase* host_data;
    struct RUPixmapAllFuncs* all_funcs;
} RUPixmap;

extern RUPixmapFuncs s_pixmap_funcs;
extern RUPixmapAllFuncs s_pixmap_all_funcs;

#ifdef __cplusplus
}
#endif
