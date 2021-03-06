// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once

#include <stdbool.h>
#include <stdint.h>

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
    int (*width)(struct RUBase* self_c);
    int (*height)(struct RUBase* self_c);
    struct RUSize (*size)(struct RUBase* self_c);
    struct RURect (*rect)(struct RUBase* self_c);
    int (*depth)(struct RUBase* self_c);
    int (*default_depth)(struct RUBase* self_c);
    void (*fill)(struct RUBase* self_c, struct RUBase* fill_color);
    void (*fill_2)(struct RUBase* self_c, struct RUBase* device,
                   struct RUBase* ofs);
    void (*fill_3)(struct RUBase* self_c, struct RUBase* device, int xofs,
                   int yofs);
    struct RUBitmap (*mask)(struct RUBase* self_c);
    void (*set_mask)(struct RUBase* self_c, struct RUBase* arg0);
    float (*device_pixel_ratio)(struct RUBase* self_c);
    void (*set_device_pixel_ratio)(struct RUBase* self_c, float scale_factor);
    bool (*has_alpha)(struct RUBase* self_c);
    bool (*has_alpha_channel)(struct RUBase* self_c);
    struct RUBitmap (*create_heuristic_mask)(struct RUBase* self_c,
                                             bool clip_tight);
    struct RUBitmap (*create_mask_from_color)(struct RUBase* self_c,
                                              struct RUBase* mask_color,
                                              uint32_t mode);
    struct RUPixmap (*grab_window)(struct RUBase* self_c, uint64_t arg0, int x,
                                   int y, int w, int h);
    struct RUPixmap (*grab_widget)(struct RUBase* self_c, struct RUBase* widget,
                                   struct RUBase* rect);
    struct RUPixmap (*grab_widget_2)(struct RUBase* self_c,
                                     struct RUBase* widget, int x, int y, int w,
                                     int h);
    struct RUPixmap (*scaled)(struct RUBase* self_c, int w, int h,
                              uint32_t aspect_mode, uint32_t mode);
    struct RUPixmap (*scaled_2)(struct RUBase* self_c, struct RUBase* s,
                                uint32_t aspect_mode, uint32_t mode);
    struct RUPixmap (*scaled_to_width)(struct RUBase* self_c, int w,
                                       uint32_t mode);
    struct RUPixmap (*scaled_to_height)(struct RUBase* self_c, int h,
                                        uint32_t mode);
    struct RUImage (*to_image)(struct RUBase* self_c);
    struct RUPixmap (*from_image)(struct RUBase* self_c, struct RUBase* image,
                                  uint32_t flags);
    struct RUPixmap (*from_image_2)(struct RUBase* self_c, struct RUBase* image,
                                    uint32_t flags);
    bool (*convert_from_image)(struct RUBase* self_c, struct RUBase* img,
                               uint32_t flags);
    struct RUPixmap (*copy)(struct RUBase* self_c, int x, int y, int width,
                            int height);
    struct RUPixmap (*copy_2)(struct RUBase* self_c, struct RUBase* rect);
    void (*scroll)(struct RUBase* self_c, int dx, int dy, int x, int y,
                   int width, int height, struct RUBase* exposed);
    void (*scroll_2)(struct RUBase* self_c, int dx, int dy, struct RUBase* rect,
                     struct RUBase* exposed);
    int64_t (*cache_key)(struct RUBase* self_c);
    bool (*is_detached)(struct RUBase* self_c);
    void (*detach)(struct RUBase* self_c);
    bool (*is_q_bitmap)(struct RUBase* self_c);
    struct RUPaintEngine (*paint_engine)(struct RUBase* self_c);
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
