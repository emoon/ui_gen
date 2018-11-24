// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once

#include <stdbool.h>
#include <stdint.h>

#include "../rute_base.h"

#ifdef __cplusplus
extern "C" {
#endif
#include "point_f_ffi.h"
#include "point_ffi.h"

struct RUMouseEventFuncs;
struct RUMouseEvent;

typedef struct RUMouseEventFuncs {
    struct RUPoint (*pos)(struct RUBase* self_c);
    struct RUPoint (*global_pos)(struct RUBase* self_c);
    int (*x)(struct RUBase* self_c);
    int (*y)(struct RUBase* self_c);
    int (*global_x)(struct RUBase* self_c);
    int (*global_y)(struct RUBase* self_c);
    struct RUPointF (*local_pos)(struct RUBase* self_c);
    struct RUPointF (*window_pos)(struct RUBase* self_c);
    struct RUPointF (*screen_pos)(struct RUBase* self_c);
    int (*button)(struct RUBase* self_c);
    int (*buttons)(struct RUBase* self_c);
    void (*set_local_pos)(struct RUBase* self_c, struct RUBase* local_position);
    int (*source)(struct RUBase* self_c);
    int (*flags)(struct RUBase* self_c);
} RUMouseEventFuncs;

typedef struct RUMouseEventAllFuncs {
    struct RUEventFuncs* event_funcs;
    struct RUInputEventFuncs* input_event_funcs;
    struct RUMouseEventFuncs* mouse_event_funcs;
} RUMouseEventAllFuncs;

typedef struct RUMouseEvent {
    RUBase* qt_data;
    RUBase* host_data;
    struct RUMouseEventAllFuncs* all_funcs;
} RUMouseEvent;

extern RUMouseEventFuncs s_mouse_event_funcs;
extern RUMouseEventAllFuncs s_mouse_event_all_funcs;

#ifdef __cplusplus
}
#endif