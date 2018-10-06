// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once

#include <stdint.h>
#include <stdbool.h>

#include "../rute_base.h"

#ifdef __cplusplus
extern "C" {
#endif

struct RUPushButtonFuncs;
struct RUPushButton;

typedef struct RUPushButtonFuncs {
    void (*destroy)(struct RUBase* self);
    bool (*auto_default)(struct RUBase* self_c);
    void (*set_auto_default)(struct RUBase* self_c, bool arg0);
    bool (*is_default)(struct RUBase* self_c);
    void (*set_default)(struct RUBase* self_c, bool arg0);
    void (*set_flat)(struct RUBase* self_c, bool arg0);
    bool (*is_flat)(struct RUBase* self_c);
    void (*show_menu)(struct RUBase* self_c);
    void (*set_text)(struct RUBase* self_c, const char* text);
    const char* (*text)(struct RUBase* self_c);
    void (*set_pressed_event)(void* object, void* user_data, void* trampoline_func, void (*event)(void* self_c));

    void (*set_released_event)(void* object, void* user_data, void* trampoline_func, void (*event)(void* self_c));

    void (*set_clicked_event)(void* object, void* user_data, void* trampoline_func, void (*event)(void* self_c, bool checked));

    void (*set_toggled_event)(void* object, void* user_data, void* trampoline_func, void (*event)(void* self_c, bool checked));

} RUPushButtonFuncs;

typedef struct RUPushButtonAllFuncs {
    struct RUWidgetFuncs* widget_funcs;
    struct RUPushButtonFuncs* push_button_funcs;
} RUPushButtonAllFuncs;

typedef struct RUPushButton {
    RUBase* qt_data;
    RUBase* host_data;
    struct RUPushButtonAllFuncs* all_funcs;
} RUPushButton;

extern RUPushButtonFuncs s_push_button_funcs;
extern RUPushButtonAllFuncs s_push_button_all_funcs;

#ifdef __cplusplus
}
#endif