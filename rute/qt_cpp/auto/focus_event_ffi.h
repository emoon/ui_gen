// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once

#include <stdbool.h>
#include <stdint.h>

#include "../rute_base.h"

#ifdef __cplusplus
extern "C" {
#endif

struct RUFocusEventFuncs;
struct RUFocusEvent;

typedef struct RUFocusEventFuncs {
    bool (*got_focus)(struct RUBase* self_c);
    bool (*lost_focus)(struct RUBase* self_c);
    uint32_t (*reason)(struct RUBase* self_c);
} RUFocusEventFuncs;

typedef struct RUFocusEventAllFuncs {
    struct RUEventFuncs* event_funcs;
    struct RUFocusEventFuncs* focus_event_funcs;
} RUFocusEventAllFuncs;

typedef struct RUFocusEvent {
    RUBase* qt_data;
    RUBase* host_data;
    struct RUFocusEventAllFuncs* all_funcs;
} RUFocusEvent;

extern RUFocusEventFuncs s_focus_event_funcs;
extern RUFocusEventAllFuncs s_focus_event_all_funcs;

#ifdef __cplusplus
}
#endif
