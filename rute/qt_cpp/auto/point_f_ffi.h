// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once

#include <stdint.h>
#include <stdbool.h>

#include "../rute_base.h"

#ifdef __cplusplus
extern "C" {
#endif
#include "point_ffi.h"

struct RUPointFFuncs;
struct RUPointF;

typedef struct RUPointFFuncs {
    void (*destroy)(struct RUBase* self);
    float (*manhattan_length)(struct RUBase* self_c);
    bool (*is_null)(struct RUBase* self_c);
    float (*x)(struct RUBase* self_c);
    float (*y)(struct RUBase* self_c);
    void (*set_x)(struct RUBase* self_c, float x);
    void (*set_y)(struct RUBase* self_c, float y);
    float (*rx)(struct RUBase* self_c);
    float (*ry)(struct RUBase* self_c);
    float (*dot_product)(struct RUBase* self_c, struct RUBase* p1, struct RUBase* p2);
    struct RUPoint (*to_point)(struct RUBase* self_c);
} RUPointFFuncs;

typedef struct RUPointFAllFuncs {
    struct RUPointFFuncs* point_f_funcs;
} RUPointFAllFuncs;

typedef struct RUPointF {
    RUBase* qt_data;
    RUBase* host_data;
    struct RUPointFAllFuncs* all_funcs;
} RUPointF;

extern RUPointFFuncs s_point_f_funcs;
extern RUPointFAllFuncs s_point_f_all_funcs;

#ifdef __cplusplus
}
#endif
