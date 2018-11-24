// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once

#include <stdbool.h>
#include <stdint.h>

#include "../rute_base.h"

#ifdef __cplusplus
extern "C" {
#endif

struct RUMimeDataFuncs;
struct RUMimeData;

typedef struct RUMimeDataFuncs {
    void (*destroy)(struct RUBase* self);
    bool (*has_urls)(struct RUBase* self_c);
    const char* (*text)(struct RUBase* self_c);
    void (*set_text)(struct RUBase* self_c, const char* text);
    bool (*has_text)(struct RUBase* self_c);
    const char* (*html)(struct RUBase* self_c);
    void (*set_html)(struct RUBase* self_c, const char* html);
    bool (*has_html)(struct RUBase* self_c);
    bool (*has_image)(struct RUBase* self_c);
    bool (*has_color)(struct RUBase* self_c);
    void (*remove_format)(struct RUBase* self_c, const char* mimetype);
    bool (*has_format)(struct RUBase* self_c, const char* mimetype);
    void (*clear)(struct RUBase* self_c);
} RUMimeDataFuncs;

typedef struct RUMimeDataAllFuncs {
    struct RUObjectFuncs* object_funcs;
    struct RUMimeDataFuncs* mime_data_funcs;
} RUMimeDataAllFuncs;

typedef struct RUMimeData {
    RUBase* qt_data;
    RUBase* host_data;
    struct RUMimeDataAllFuncs* all_funcs;
} RUMimeData;

extern RUMimeDataFuncs s_mime_data_funcs;
extern RUMimeDataAllFuncs s_mime_data_all_funcs;

#ifdef __cplusplus
}
#endif