// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once

#include <stdbool.h>
#include <stdint.h>

#include "../rute_base.h"

#ifdef __cplusplus
extern "C" {
#endif
#include "size_ffi.h"
#include "surface_format_ffi.h"

struct RUSurfaceFuncs;
struct RUSurface;

typedef struct RUSurfaceFuncs {
    int (*surface_class)(struct RUBase* self_c);
    struct RUSurfaceFormat (*format)(struct RUBase* self_c);
    int (*surface_type)(struct RUBase* self_c);
    struct RUSize (*size)(struct RUBase* self_c);
} RUSurfaceFuncs;

typedef struct RUSurfaceAllFuncs {
    struct RUSurfaceFuncs* surface_funcs;
} RUSurfaceAllFuncs;

typedef struct RUSurface {
    RUBase* qt_data;
    RUBase* host_data;
    struct RUSurfaceAllFuncs* all_funcs;
} RUSurface;

extern RUSurfaceFuncs s_surface_funcs;
extern RUSurfaceAllFuncs s_surface_all_funcs;

#ifdef __cplusplus
}
#endif