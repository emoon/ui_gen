// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once

#include <stdint.h>
#include <stdbool.h>

#include "../rute_base.h"

#ifdef __cplusplus
extern "C" {
#endif

struct RUPointFuncs;
struct RUPoint;

typedef struct RUPointFuncs {
    void (*destroy)(struct RUBase* self);
    bool (*is_null)(struct RUBase* self_c);
    int (*x)(struct RUBase* self_c);
    int (*y)(struct RUBase* self_c);
    void (*set_x)(struct RUBase* self_c, int x);
    void (*set_y)(struct RUBase* self_c, int y);
    int (*manhattan_length)(struct RUBase* self_c);
    int (*rx)(struct RUBase* self_c);
    int (*ry)(struct RUBase* self_c);
    int (*dot_product)(struct RUBase* self_c, struct RUBase* p1, struct RUBase* p2);
} RUPointFuncs;

typedef struct RUPointAllFuncs {
    struct RUPointFuncs* point_funcs;
} RUPointAllFuncs;

typedef struct RUPoint {
    RUBase* qt_data;
    RUBase* host_data;
    struct RUPointAllFuncs* all_funcs;
} RUPoint;

extern RUPointFuncs s_point_funcs;
extern RUPointAllFuncs s_point_all_funcs;

#ifdef __cplusplus
}
#endif
