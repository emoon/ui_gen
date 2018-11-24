// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once

#include <stdbool.h>
#include <stdint.h>

#include "../rute_base.h"

#ifdef __cplusplus
extern "C" {
#endif
#include "region_ffi.h"

struct RUExposeEventFuncs;
struct RUExposeEvent;

typedef struct RUExposeEventFuncs {
    struct RURegion (*region)(struct RUBase* self_c);
} RUExposeEventFuncs;

typedef struct RUExposeEventAllFuncs {
    struct RUEventFuncs* event_funcs;
    struct RUExposeEventFuncs* expose_event_funcs;
} RUExposeEventAllFuncs;

typedef struct RUExposeEvent {
    RUBase* qt_data;
    RUBase* host_data;
    struct RUExposeEventAllFuncs* all_funcs;
} RUExposeEvent;

extern RUExposeEventFuncs s_expose_event_funcs;
extern RUExposeEventAllFuncs s_expose_event_all_funcs;

#ifdef __cplusplus
}
#endif