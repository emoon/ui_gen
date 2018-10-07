// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once

#include <stdint.h>
#include <stdbool.h>

#include "../rute_base.h"

#ifdef __cplusplus
extern "C" {
#endif

struct RUWidgetFuncs;
struct RUWidget;

typedef struct RUWidgetFuncs {
    void (*destroy)(struct RUBase* self);
    void (*show)(struct RUBase* self_c);
    void (*set_fixed_height)(struct RUBase* self_c, int width);
    void (*set_fixed_width)(struct RUBase* self_c, int width);
    void (*resize)(struct RUBase* self_c, int width, int height);
    void (*set_parent)(struct RUBase* self_c, struct RUBase* parent);
    void (*update)(struct RUBase* self_c);
    void (*set_window_title_changed_event)(void* object, void* user_data, void* trampoline_func, void (*event)(void* self_c, const char* title));

} RUWidgetFuncs;

typedef struct RUWidgetAllFuncs {
    struct RUWidgetFuncs* widget_funcs;
} RUWidgetAllFuncs;

typedef struct RUWidget {
    RUBase* qt_data;
    RUBase* host_data;
    struct RUWidgetAllFuncs* all_funcs;
} RUWidget;

extern RUWidgetFuncs s_widget_funcs;
extern RUWidgetAllFuncs s_widget_all_funcs;

#ifdef __cplusplus
}
#endif
