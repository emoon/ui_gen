// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once

#include <stdbool.h>
#include <stdint.h>

#include "../rute_base.h"

#ifdef __cplusplus
extern "C" {
#endif

struct RUGradientFuncs;
struct RUGradient;

typedef struct RUGradientFuncs {
    void (*destroy)(struct RUBase* self);
    int (*get_type)(struct RUBase* self_c);
    void (*set_spread)(struct RUBase* self_c, int spread);
    int (*spread)(struct RUBase* self_c);
    void (*set_color_at)(struct RUBase* self_c, float pos,
                         struct RUBase* color);
    int (*coordinate_mode)(struct RUBase* self_c);
    void (*set_coordinate_mode)(struct RUBase* self_c, int mode);
    int (*interpolation_mode)(struct RUBase* self_c);
    void (*set_interpolation_mode)(struct RUBase* self_c, int mode);
} RUGradientFuncs;

typedef struct RUGradientAllFuncs {
    struct RUGradientFuncs* gradient_funcs;
} RUGradientAllFuncs;

typedef struct RUGradient {
    RUBase* qt_data;
    RUBase* host_data;
    struct RUGradientAllFuncs* all_funcs;
} RUGradient;

extern RUGradientFuncs s_gradient_funcs;
extern RUGradientAllFuncs s_gradient_all_funcs;

#ifdef __cplusplus
}
#endif
