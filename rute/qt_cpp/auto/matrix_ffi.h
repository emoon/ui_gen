// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once

#include <stdbool.h>
#include <stdint.h>

#include "../rute_base.h"

#ifdef __cplusplus
extern "C" {
#endif
#include "line_f_ffi.h"
#include "line_ffi.h"
#include "matrix_ffi.h"
#include "point_f_ffi.h"
#include "point_ffi.h"
#include "polygon_f_ffi.h"
#include "polygon_ffi.h"
#include "rect_f_ffi.h"
#include "rect_ffi.h"
#include "region_ffi.h"

struct RUMatrixFuncs;
struct RUMatrix;

typedef struct RUMatrixFuncs {
    void (*destroy)(struct RUBase* self);
    float (*m11)(struct RUBase* self_c);
    float (*m12)(struct RUBase* self_c);
    float (*m21)(struct RUBase* self_c);
    float (*m22)(struct RUBase* self_c);
    float (*dx)(struct RUBase* self_c);
    float (*dy)(struct RUBase* self_c);
    struct RURect (*map_rect)(struct RUBase* self_c, struct RUBase* arg0);
    struct RURectF (*map_rect_2)(struct RUBase* self_c, struct RUBase* arg0);
    struct RUPoint (*map_3)(struct RUBase* self_c, struct RUBase* p);
    struct RUPointF (*map_4)(struct RUBase* self_c, struct RUBase* p);
    struct RULine (*map_5)(struct RUBase* self_c, struct RUBase* l);
    struct RULineF (*map_6)(struct RUBase* self_c, struct RUBase* l);
    struct RUPolygonF (*map_7)(struct RUBase* self_c, struct RUBase* a);
    struct RUPolygon (*map_8)(struct RUBase* self_c, struct RUBase* a);
    struct RURegion (*map_9)(struct RUBase* self_c, struct RUBase* r);
    struct RUPolygon (*map_to_polygon)(struct RUBase* self_c, struct RUBase* r);
    void (*reset)(struct RUBase* self_c);
    bool (*is_identity)(struct RUBase* self_c);
    struct RUMatrix (*scale)(struct RUBase* self_c, float sx, float sy);
    struct RUMatrix (*shear)(struct RUBase* self_c, float sh, float sv);
    struct RUMatrix (*rotate)(struct RUBase* self_c, float a);
    bool (*is_invertible)(struct RUBase* self_c);
    float (*determinant)(struct RUBase* self_c);
} RUMatrixFuncs;

typedef struct RUMatrixAllFuncs {
    struct RUMatrixFuncs* matrix_funcs;
} RUMatrixAllFuncs;

typedef struct RUMatrix {
    RUBase* qt_data;
    RUBase* host_data;
    struct RUMatrixAllFuncs* all_funcs;
} RUMatrix;

extern RUMatrixFuncs s_matrix_funcs;
extern RUMatrixAllFuncs s_matrix_all_funcs;

#ifdef __cplusplus
}
#endif
