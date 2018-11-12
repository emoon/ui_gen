// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once

#include <stdint.h>
#include <stdbool.h>

#include "../rute_base.h"

#ifdef __cplusplus
extern "C" {
#endif
#include "paint_device_ffi.h"
#include "platform_backing_store_ffi.h"
#include "region_ffi.h"
#include "size_ffi.h"
#include "window_ffi.h"

struct RUBackingStoreFuncs;
struct RUBackingStore;

typedef struct RUBackingStoreFuncs {
    void (*destroy)(struct RUBase* self);
    struct RUWindow (*window)(struct RUBase* self_c);
    struct RUPaintDevice (*paint_device)(struct RUBase* self_c);
    void (*flush)(struct RUBase* self_c, struct RUBase* region, struct RUBase* window, struct RUBase* offset);
    void (*resize)(struct RUBase* self_c, struct RUBase* size);
    struct RUSize (*size)(struct RUBase* self_c);
    bool (*scroll)(struct RUBase* self_c, struct RUBase* area, int dx, int dy);
    void (*begin_paint)(struct RUBase* self_c, struct RUBase* arg0);
    void (*end_paint)(struct RUBase* self_c);
    void (*set_static_contents)(struct RUBase* self_c, struct RUBase* region);
    struct RURegion (*static_contents)(struct RUBase* self_c);
    bool (*has_static_contents)(struct RUBase* self_c);
    struct RUPlatformBackingStore (*handle)(struct RUBase* self_c);
} RUBackingStoreFuncs;

typedef struct RUBackingStoreAllFuncs {
    struct RUBackingStoreFuncs* backing_store_funcs;
} RUBackingStoreAllFuncs;

typedef struct RUBackingStore {
    RUBase* qt_data;
    RUBase* host_data;
    struct RUBackingStoreAllFuncs* all_funcs;
} RUBackingStore;

extern RUBackingStoreFuncs s_backing_store_funcs;
extern RUBackingStoreAllFuncs s_backing_store_all_funcs;

#ifdef __cplusplus
}
#endif