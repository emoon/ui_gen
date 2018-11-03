// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once

#include <stdint.h>
#include <stdbool.h>

#include "../rute_base.h"

#ifdef __cplusplus
extern "C" {
#endif

struct RUMarginsFuncs;
struct RUMargins;

typedef struct RUMarginsFuncs {
    void (*destroy)(struct RUBase* self);
    bool (*is_null)(struct RUBase* self_c);
    int (*left)(struct RUBase* self_c);
    int (*top)(struct RUBase* self_c);
    int (*right)(struct RUBase* self_c);
    int (*bottom)(struct RUBase* self_c);
    void (*set_left)(struct RUBase* self_c, int left);
    void (*set_top)(struct RUBase* self_c, int top);
    void (*set_right)(struct RUBase* self_c, int right);
    void (*set_bottom)(struct RUBase* self_c, int bottom);
} RUMarginsFuncs;

typedef struct RUMarginsAllFuncs {
    struct RUMarginsFuncs* margins_funcs;
} RUMarginsAllFuncs;

typedef struct RUMargins {
    RUBase* qt_data;
    RUBase* host_data;
    struct RUMarginsAllFuncs* all_funcs;
} RUMargins;

extern RUMarginsFuncs s_margins_funcs;
extern RUMarginsAllFuncs s_margins_all_funcs;

#ifdef __cplusplus
}
#endif
