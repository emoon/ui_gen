// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once

#include <stdbool.h>
#include <stdint.h>

#include "../rute_base.h"

#ifdef __cplusplus
extern "C" {
#endif
#include "rect_ffi.h"

struct RUDragMoveEventFuncs;
struct RUDragMoveEvent;

typedef struct RUDragMoveEventFuncs {
    struct RURect (*answer_rect)(struct RUBase* self_c);
    void (*accept)(struct RUBase* self_c);
    void (*ignore)(struct RUBase* self_c);
    void (*accept_2)(struct RUBase* self_c, struct RUBase* r);
    void (*ignore_2)(struct RUBase* self_c, struct RUBase* r);
} RUDragMoveEventFuncs;

typedef struct RUDragMoveEventAllFuncs {
    struct RUEventFuncs* event_funcs;
    struct RUDropEventFuncs* drop_event_funcs;
    struct RUDragMoveEventFuncs* drag_move_event_funcs;
} RUDragMoveEventAllFuncs;

typedef struct RUDragMoveEvent {
    RUBase* qt_data;
    RUBase* host_data;
    struct RUDragMoveEventAllFuncs* all_funcs;
} RUDragMoveEvent;

extern RUDragMoveEventFuncs s_drag_move_event_funcs;
extern RUDragMoveEventAllFuncs s_drag_move_event_all_funcs;

#ifdef __cplusplus
}
#endif