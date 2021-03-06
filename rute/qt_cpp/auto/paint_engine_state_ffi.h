// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once

#include <stdbool.h>
#include <stdint.h>

#include "../rute_base.h"

#ifdef __cplusplus
extern "C" {
#endif
#include "brush_ffi.h"
#include "font_ffi.h"
#include "painter_ffi.h"
#include "pen_ffi.h"
#include "point_f_ffi.h"
#include "region_ffi.h"

struct RUPaintEngineStateFuncs;
struct RUPaintEngineState;

typedef struct RUPaintEngineStateFuncs {
    void (*destroy)(struct RUBase* self);
    uint32_t (*state)(struct RUBase* self_c);
    struct RUPen (*pen)(struct RUBase* self_c);
    struct RUBrush (*brush)(struct RUBase* self_c);
    struct RUPointF (*brush_origin)(struct RUBase* self_c);
    struct RUBrush (*background_brush)(struct RUBase* self_c);
    uint32_t (*background_mode)(struct RUBase* self_c);
    struct RUFont (*font)(struct RUBase* self_c);
    uint32_t (*clip_operation)(struct RUBase* self_c);
    struct RURegion (*clip_region)(struct RUBase* self_c);
    bool (*is_clip_enabled)(struct RUBase* self_c);
    uint32_t (*render_hints)(struct RUBase* self_c);
    uint32_t (*composition_mode)(struct RUBase* self_c);
    float (*opacity)(struct RUBase* self_c);
    struct RUPainter (*painter)(struct RUBase* self_c);
    bool (*brush_needs_resolving)(struct RUBase* self_c);
    bool (*pen_needs_resolving)(struct RUBase* self_c);
} RUPaintEngineStateFuncs;

typedef struct RUPaintEngineStateAllFuncs {
    struct RUPaintEngineStateFuncs* paint_engine_state_funcs;
} RUPaintEngineStateAllFuncs;

typedef struct RUPaintEngineState {
    RUBase* qt_data;
    RUBase* host_data;
    struct RUPaintEngineStateAllFuncs* all_funcs;
} RUPaintEngineState;

extern RUPaintEngineStateFuncs s_paint_engine_state_funcs;
extern RUPaintEngineStateAllFuncs s_paint_engine_state_all_funcs;

#ifdef __cplusplus
}
#endif
