// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once

#include <stdbool.h>
#include <stdint.h>

#include "../rute_base.h"

#ifdef __cplusplus
extern "C" {
#endif
#include "size_ffi.h"
#include "size_policy_ffi.h"
#include "spacer_item_ffi.h"

struct RUSpacerItemFuncs;
struct RUSpacerItem;

typedef struct RUSpacerItemFuncs {
    void (*change_size)(struct RUBase* self_c, int w, int h, int h_data,
                        int v_data);
    struct RUSize (*size_hint)(struct RUBase* self_c);
    struct RUSize (*minimum_size)(struct RUBase* self_c);
    struct RUSize (*maximum_size)(struct RUBase* self_c);
    int (*expanding_directions)(struct RUBase* self_c);
    bool (*is_empty)(struct RUBase* self_c);
    struct RUSpacerItem (*spacer_item)(struct RUBase* self_c);
    struct RUSizePolicy (*size_policy)(struct RUBase* self_c);
} RUSpacerItemFuncs;

typedef struct RUSpacerItemAllFuncs {
    struct RULayoutItemFuncs* layout_item_funcs;
    struct RUSpacerItemFuncs* spacer_item_funcs;
} RUSpacerItemAllFuncs;

typedef struct RUSpacerItem {
    RUBase* qt_data;
    RUBase* host_data;
    struct RUSpacerItemAllFuncs* all_funcs;
} RUSpacerItem;

extern RUSpacerItemFuncs s_spacer_item_funcs;
extern RUSpacerItemAllFuncs s_spacer_item_all_funcs;

#ifdef __cplusplus
}
#endif
