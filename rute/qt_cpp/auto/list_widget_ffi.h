// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once

#include <stdint.h>
#include <stdbool.h>

#include "../rute_base.h"

#ifdef __cplusplus
extern "C" {
#endif

struct RUListWidgetFuncs;
struct RUListWidget;

typedef struct RUListWidgetFuncs {
    void (*destroy)(struct RUBase* self);
    void (*add_item)(struct RUBase* self_c, const char* label);
    void (*clear)(struct RUBase* self_c);
} RUListWidgetFuncs;

typedef struct RUListWidgetAllFuncs {
    struct RUWidgetFuncs* widget_funcs;
    struct RUListWidgetFuncs* list_widget_funcs;
} RUListWidgetAllFuncs;

typedef struct RUListWidget {
    RUBase* qt_data;
    RUBase* host_data;
    struct RUListWidgetAllFuncs* all_funcs;
} RUListWidget;

extern RUListWidgetFuncs s_list_widget_funcs;
extern RUListWidgetAllFuncs s_list_widget_all_funcs;

#ifdef __cplusplus
}
#endif
