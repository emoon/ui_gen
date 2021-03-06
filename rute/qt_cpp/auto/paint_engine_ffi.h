// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once

#include <stdbool.h>
#include <stdint.h>

#include "../rute_base.h"

#ifdef __cplusplus
extern "C" {
#endif
#include "paint_device_ffi.h"
#include "point_ffi.h"
#include "rect_ffi.h"
#include "region_ffi.h"

struct RUPaintEngineFuncs;
struct RUPaintEngine;

typedef struct RUPaintEngineFuncs {
    bool (*is_active)(struct RUBase* self_c);
    void (*set_active)(struct RUBase* self_c, bool new_state);
    bool (*begin)(struct RUBase* self_c, struct RUBase* pdev);
    bool (*end)(struct RUBase* self_c);
    void (*update_state)(struct RUBase* self_c, struct RUBase* state);
    void (*draw_rects)(struct RUBase* self_c, struct RUBase* rects,
                       int rect_count);
    void (*draw_rects_2)(struct RUBase* self_c, struct RUBase* rects,
                         int rect_count);
    void (*draw_lines)(struct RUBase* self_c, struct RUBase* lines,
                       int line_count);
    void (*draw_lines_2)(struct RUBase* self_c, struct RUBase* lines,
                         int line_count);
    void (*draw_ellipse)(struct RUBase* self_c, struct RUBase* r);
    void (*draw_ellipse_2)(struct RUBase* self_c, struct RUBase* r);
    void (*draw_points)(struct RUBase* self_c, struct RUBase* points,
                        int point_count);
    void (*draw_points_2)(struct RUBase* self_c, struct RUBase* points,
                          int point_count);
    void (*draw_polygon)(struct RUBase* self_c, struct RUBase* points,
                         int point_count, uint32_t mode);
    void (*draw_polygon_2)(struct RUBase* self_c, struct RUBase* points,
                           int point_count, uint32_t mode);
    void (*draw_pixmap)(struct RUBase* self_c, struct RUBase* r,
                        struct RUBase* pm, struct RUBase* sr);
    void (*draw_tiled_pixmap)(struct RUBase* self_c, struct RUBase* r,
                              struct RUBase* pixmap, struct RUBase* s);
    void (*draw_image)(struct RUBase* self_c, struct RUBase* r,
                       struct RUBase* pm, struct RUBase* sr, uint32_t flags);
    void (*set_paint_device)(struct RUBase* self_c, struct RUBase* device);
    struct RUPaintDevice (*paint_device)(struct RUBase* self_c);
    void (*set_system_clip)(struct RUBase* self_c, struct RUBase* base_clip);
    struct RURegion (*system_clip)(struct RUBase* self_c);
    void (*set_system_rect)(struct RUBase* self_c, struct RUBase* rect);
    struct RURect (*system_rect)(struct RUBase* self_c);
    struct RUPoint (*coordinate_offset)(struct RUBase* self_c);
} RUPaintEngineFuncs;

typedef struct RUPaintEngineAllFuncs {
    struct RUPaintEngineFuncs* paint_engine_funcs;
} RUPaintEngineAllFuncs;

typedef struct RUPaintEngine {
    RUBase* qt_data;
    RUBase* host_data;
    struct RUPaintEngineAllFuncs* all_funcs;
} RUPaintEngine;

extern RUPaintEngineFuncs s_paint_engine_funcs;
extern RUPaintEngineAllFuncs s_paint_engine_all_funcs;

#ifdef __cplusplus
}
#endif
