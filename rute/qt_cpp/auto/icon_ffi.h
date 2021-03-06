// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once

#include <stdbool.h>
#include <stdint.h>

#include "../rute_base.h"

#ifdef __cplusplus
extern "C" {
#endif
#include "icon_ffi.h"
#include "pixmap_ffi.h"
#include "size_ffi.h"

struct RUIconFuncs;
struct RUIcon;

typedef struct RUIconFuncs {
    void (*destroy)(struct RUBase* self);
    void (*swap)(struct RUBase* self_c, struct RUBase* other);
    struct RUPixmap (*pixmap)(struct RUBase* self_c, struct RUBase* size,
                              uint32_t mode, uint32_t state);
    struct RUPixmap (*pixmap_2)(struct RUBase* self_c, int w, int h,
                                uint32_t mode, uint32_t state);
    struct RUPixmap (*pixmap_3)(struct RUBase* self_c, int extent,
                                uint32_t mode, uint32_t state);
    struct RUPixmap (*pixmap_4)(struct RUBase* self_c, struct RUBase* window,
                                struct RUBase* size, uint32_t mode,
                                uint32_t state);
    struct RUSize (*actual_size)(struct RUBase* self_c, struct RUBase* size,
                                 uint32_t mode, uint32_t state);
    struct RUSize (*actual_size_2)(struct RUBase* self_c, struct RUBase* window,
                                   struct RUBase* size, uint32_t mode,
                                   uint32_t state);
    const char* (*name)(struct RUBase* self_c);
    void (*paint)(struct RUBase* self_c, struct RUBase* painter,
                  struct RUBase* rect, uint32_t alignment, uint32_t mode,
                  uint32_t state);
    void (*paint_2)(struct RUBase* self_c, struct RUBase* painter, int x, int y,
                    int w, int h, uint32_t alignment, uint32_t mode,
                    uint32_t state);
    bool (*is_null)(struct RUBase* self_c);
    bool (*is_detached)(struct RUBase* self_c);
    void (*detach)(struct RUBase* self_c);
    int64_t (*cache_key)(struct RUBase* self_c);
    void (*add_pixmap)(struct RUBase* self_c, struct RUBase* pixmap,
                       uint32_t mode, uint32_t state);
    void (*add_file)(struct RUBase* self_c, const char* file_name,
                     struct RUBase* size, uint32_t mode, uint32_t state);
    void (*set_is_mask)(struct RUBase* self_c, bool is_mask);
    bool (*is_mask)(struct RUBase* self_c);
    struct RUIcon (*from_theme)(struct RUBase* self_c, const char* name);
    struct RUIcon (*from_theme_2)(struct RUBase* self_c, const char* name,
                                  struct RUBase* fallback);
    bool (*has_theme_icon)(struct RUBase* self_c, const char* name);
    const char* (*theme_name)(struct RUBase* self_c);
    void (*set_theme_name)(struct RUBase* self_c, const char* path);
} RUIconFuncs;

typedef struct RUIconAllFuncs {
    struct RUIconFuncs* icon_funcs;
} RUIconAllFuncs;

typedef struct RUIcon {
    RUBase* qt_data;
    RUBase* host_data;
    struct RUIconAllFuncs* all_funcs;
} RUIcon;

extern RUIconFuncs s_icon_funcs;
extern RUIconAllFuncs s_icon_all_funcs;

#ifdef __cplusplus
}
#endif
