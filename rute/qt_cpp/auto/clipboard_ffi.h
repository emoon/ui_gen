// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once

#include <stdbool.h>
#include <stdint.h>

#include "../rute_base.h"

#ifdef __cplusplus
extern "C" {
#endif
#include "image_ffi.h"
#include "mime_data_ffi.h"
#include "pixmap_ffi.h"

struct RUClipboardFuncs;
struct RUClipboard;

typedef struct RUClipboardFuncs {
    void (*clear)(struct RUBase* self_c, uint32_t mode);
    bool (*supports_selection)(struct RUBase* self_c);
    bool (*supports_find_buffer)(struct RUBase* self_c);
    bool (*owns_selection)(struct RUBase* self_c);
    bool (*owns_clipboard)(struct RUBase* self_c);
    bool (*owns_find_buffer)(struct RUBase* self_c);
    const char* (*text)(struct RUBase* self_c, uint32_t mode);
    void (*set_text)(struct RUBase* self_c, const char* arg0, uint32_t mode);
    struct RUMimeData (*mime_data)(struct RUBase* self_c, uint32_t mode);
    void (*set_mime_data)(struct RUBase* self_c, struct RUBase* data,
                          uint32_t mode);
    struct RUImage (*image)(struct RUBase* self_c, uint32_t mode);
    struct RUPixmap (*pixmap)(struct RUBase* self_c, uint32_t mode);
    void (*set_image)(struct RUBase* self_c, struct RUBase* arg0,
                      uint32_t mode);
    void (*set_pixmap)(struct RUBase* self_c, struct RUBase* arg0,
                       uint32_t mode);
} RUClipboardFuncs;

typedef struct RUClipboardAllFuncs {
    struct RUObjectFuncs* object_funcs;
    struct RUClipboardFuncs* clipboard_funcs;
} RUClipboardAllFuncs;

typedef struct RUClipboard {
    RUBase* qt_data;
    RUBase* host_data;
    struct RUClipboardAllFuncs* all_funcs;
} RUClipboard;

extern RUClipboardFuncs s_clipboard_funcs;
extern RUClipboardAllFuncs s_clipboard_all_funcs;

#ifdef __cplusplus
}
#endif
