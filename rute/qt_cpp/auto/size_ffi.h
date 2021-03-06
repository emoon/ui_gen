// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once

#include <stdbool.h>
#include <stdint.h>

#include "../rute_base.h"

#ifdef __cplusplus
extern "C" {
#endif
#include "size_ffi.h"

struct RUSizeFuncs;
struct RUSize;

typedef struct RUSizeFuncs {
    void (*destroy)(struct RUBase* self);
    bool (*is_null)(struct RUBase* self_c);
    bool (*is_empty)(struct RUBase* self_c);
    bool (*is_valid)(struct RUBase* self_c);
    int (*width)(struct RUBase* self_c);
    int (*height)(struct RUBase* self_c);
    void (*set_width)(struct RUBase* self_c, int w);
    void (*set_height)(struct RUBase* self_c, int h);
    void (*scale)(struct RUBase* self_c, int w, int h, uint32_t mode);
    void (*scale_2)(struct RUBase* self_c, struct RUBase* s, uint32_t mode);
    struct RUSize (*scaled)(struct RUBase* self_c, int w, int h, uint32_t mode);
    struct RUSize (*scaled_2)(struct RUBase* self_c, struct RUBase* s,
                              uint32_t mode);
    struct RUSize (*expanded_to)(struct RUBase* self_c, struct RUBase* arg0);
    struct RUSize (*bounded_to)(struct RUBase* self_c, struct RUBase* arg0);
    int (*rwidth)(struct RUBase* self_c);
    int (*rheight)(struct RUBase* self_c);
} RUSizeFuncs;

typedef struct RUSizeAllFuncs {
    struct RUSizeFuncs* size_funcs;
} RUSizeAllFuncs;

typedef struct RUSize {
    RUBase* qt_data;
    RUBase* host_data;
    struct RUSizeAllFuncs* all_funcs;
} RUSize;

extern RUSizeFuncs s_size_funcs;
extern RUSizeAllFuncs s_size_all_funcs;

#ifdef __cplusplus
}
#endif
