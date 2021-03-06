////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// This file is auto-generated by rute_gen. DO NOT EDIT
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#include "../rute_base.h"
#include "../rute_manual.h"
#include <QPalette>
#include "palette_ffi.h"

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void palette_swap(struct RUBase* self_c, struct RUBase* other) {
    WRPalette* qt_value = (WRPalette*)self_c;
    qt_value->swap(*((WRPalette*)other));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static uint32_t palette_current_color_group(struct RUBase* self_c) {
    WRPalette* qt_value = (WRPalette*)self_c;
    auto ret_value = qt_value->currentColorGroup();
    return (uint32_t)ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void palette_set_current_color_group(struct RUBase* self_c, uint32_t cg) {
    WRPalette* qt_value = (WRPalette*)self_c;
    qt_value->setCurrentColorGroup((QPalette::ColorGroup)cg);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUColor palette_color(struct RUBase* self_c, uint32_t cg, uint32_t cr) {
    WRPalette* qt_value = (WRPalette*)self_c;
    auto ret_value = qt_value->color((QPalette::ColorGroup)cg, (QPalette::ColorRole)cr);
    WRColor* new_val = new WRColor();
    *new_val = ret_value;
    struct RUColor ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_color_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUBrush palette_brush(struct RUBase* self_c, uint32_t cg, uint32_t cr) {
    WRPalette* qt_value = (WRPalette*)self_c;
    auto ret_value = qt_value->brush((QPalette::ColorGroup)cg, (QPalette::ColorRole)cr);
    WRBrush* new_val = new WRBrush();
    *new_val = ret_value;
    struct RUBrush ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_brush_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void palette_set_color(struct RUBase* self_c, uint32_t cg, uint32_t cr, struct RUBase* color) {
    WRPalette* qt_value = (WRPalette*)self_c;
    qt_value->setColor((QPalette::ColorGroup)cg, (QPalette::ColorRole)cr, *((WRColor*)color));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void palette_set_color_2(struct RUBase* self_c, uint32_t cr, struct RUBase* color) {
    WRPalette* qt_value = (WRPalette*)self_c;
    qt_value->setColor((QPalette::ColorRole)cr, *((WRColor*)color));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void palette_set_brush(struct RUBase* self_c, uint32_t cr, struct RUBase* brush) {
    WRPalette* qt_value = (WRPalette*)self_c;
    qt_value->setBrush((QPalette::ColorRole)cr, *((WRBrush*)brush));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool palette_is_brush_set(struct RUBase* self_c, uint32_t cg, uint32_t cr) {
    WRPalette* qt_value = (WRPalette*)self_c;
    auto ret_value = qt_value->isBrushSet((QPalette::ColorGroup)cg, (QPalette::ColorRole)cr);
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void palette_set_brush_2(struct RUBase* self_c, uint32_t cg, uint32_t cr, struct RUBase* brush) {
    WRPalette* qt_value = (WRPalette*)self_c;
    qt_value->setBrush((QPalette::ColorGroup)cg, (QPalette::ColorRole)cr, *((WRBrush*)brush));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void palette_set_color_group(struct RUBase* self_c, uint32_t cr, struct RUBase* window_text, struct RUBase* button, struct RUBase* light, struct RUBase* dark, struct RUBase* mid, struct RUBase* text, struct RUBase* bright_text, struct RUBase* base, struct RUBase* window) {
    WRPalette* qt_value = (WRPalette*)self_c;
    qt_value->setColorGroup((QPalette::ColorGroup)cr, *((WRBrush*)window_text), *((WRBrush*)button), *((WRBrush*)light), *((WRBrush*)dark), *((WRBrush*)mid), *((WRBrush*)text), *((WRBrush*)bright_text), *((WRBrush*)base), *((WRBrush*)window));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool palette_is_equal(struct RUBase* self_c, uint32_t cr1, uint32_t cr2) {
    WRPalette* qt_value = (WRPalette*)self_c;
    auto ret_value = qt_value->isEqual((QPalette::ColorGroup)cr1, (QPalette::ColorGroup)cr2);
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUColor palette_color_2(struct RUBase* self_c, uint32_t cr) {
    WRPalette* qt_value = (WRPalette*)self_c;
    auto ret_value = qt_value->color((QPalette::ColorRole)cr);
    WRColor* new_val = new WRColor();
    *new_val = ret_value;
    struct RUColor ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_color_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUBrush palette_brush_2(struct RUBase* self_c, uint32_t cr) {
    WRPalette* qt_value = (WRPalette*)self_c;
    auto ret_value = qt_value->brush((QPalette::ColorRole)cr);
    WRBrush* new_val = new WRBrush();
    *new_val = ret_value;
    struct RUBrush ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_brush_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUBrush palette_foreground(struct RUBase* self_c) {
    WRPalette* qt_value = (WRPalette*)self_c;
    auto ret_value = qt_value->foreground();
    WRBrush* new_val = new WRBrush();
    *new_val = ret_value;
    struct RUBrush ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_brush_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUBrush palette_window_text(struct RUBase* self_c) {
    WRPalette* qt_value = (WRPalette*)self_c;
    auto ret_value = qt_value->windowText();
    WRBrush* new_val = new WRBrush();
    *new_val = ret_value;
    struct RUBrush ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_brush_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUBrush palette_button(struct RUBase* self_c) {
    WRPalette* qt_value = (WRPalette*)self_c;
    auto ret_value = qt_value->button();
    WRBrush* new_val = new WRBrush();
    *new_val = ret_value;
    struct RUBrush ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_brush_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUBrush palette_light(struct RUBase* self_c) {
    WRPalette* qt_value = (WRPalette*)self_c;
    auto ret_value = qt_value->light();
    WRBrush* new_val = new WRBrush();
    *new_val = ret_value;
    struct RUBrush ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_brush_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUBrush palette_dark(struct RUBase* self_c) {
    WRPalette* qt_value = (WRPalette*)self_c;
    auto ret_value = qt_value->dark();
    WRBrush* new_val = new WRBrush();
    *new_val = ret_value;
    struct RUBrush ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_brush_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUBrush palette_mid(struct RUBase* self_c) {
    WRPalette* qt_value = (WRPalette*)self_c;
    auto ret_value = qt_value->mid();
    WRBrush* new_val = new WRBrush();
    *new_val = ret_value;
    struct RUBrush ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_brush_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUBrush palette_text(struct RUBase* self_c) {
    WRPalette* qt_value = (WRPalette*)self_c;
    auto ret_value = qt_value->text();
    WRBrush* new_val = new WRBrush();
    *new_val = ret_value;
    struct RUBrush ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_brush_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUBrush palette_base(struct RUBase* self_c) {
    WRPalette* qt_value = (WRPalette*)self_c;
    auto ret_value = qt_value->base();
    WRBrush* new_val = new WRBrush();
    *new_val = ret_value;
    struct RUBrush ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_brush_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUBrush palette_alternate_base(struct RUBase* self_c) {
    WRPalette* qt_value = (WRPalette*)self_c;
    auto ret_value = qt_value->alternateBase();
    WRBrush* new_val = new WRBrush();
    *new_val = ret_value;
    struct RUBrush ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_brush_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUBrush palette_tool_tip_base(struct RUBase* self_c) {
    WRPalette* qt_value = (WRPalette*)self_c;
    auto ret_value = qt_value->toolTipBase();
    WRBrush* new_val = new WRBrush();
    *new_val = ret_value;
    struct RUBrush ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_brush_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUBrush palette_tool_tip_text(struct RUBase* self_c) {
    WRPalette* qt_value = (WRPalette*)self_c;
    auto ret_value = qt_value->toolTipText();
    WRBrush* new_val = new WRBrush();
    *new_val = ret_value;
    struct RUBrush ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_brush_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUBrush palette_background(struct RUBase* self_c) {
    WRPalette* qt_value = (WRPalette*)self_c;
    auto ret_value = qt_value->background();
    WRBrush* new_val = new WRBrush();
    *new_val = ret_value;
    struct RUBrush ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_brush_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUBrush palette_window(struct RUBase* self_c) {
    WRPalette* qt_value = (WRPalette*)self_c;
    auto ret_value = qt_value->window();
    WRBrush* new_val = new WRBrush();
    *new_val = ret_value;
    struct RUBrush ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_brush_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUBrush palette_midlight(struct RUBase* self_c) {
    WRPalette* qt_value = (WRPalette*)self_c;
    auto ret_value = qt_value->midlight();
    WRBrush* new_val = new WRBrush();
    *new_val = ret_value;
    struct RUBrush ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_brush_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUBrush palette_bright_text(struct RUBase* self_c) {
    WRPalette* qt_value = (WRPalette*)self_c;
    auto ret_value = qt_value->brightText();
    WRBrush* new_val = new WRBrush();
    *new_val = ret_value;
    struct RUBrush ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_brush_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUBrush palette_button_text(struct RUBase* self_c) {
    WRPalette* qt_value = (WRPalette*)self_c;
    auto ret_value = qt_value->buttonText();
    WRBrush* new_val = new WRBrush();
    *new_val = ret_value;
    struct RUBrush ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_brush_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUBrush palette_shadow(struct RUBase* self_c) {
    WRPalette* qt_value = (WRPalette*)self_c;
    auto ret_value = qt_value->shadow();
    WRBrush* new_val = new WRBrush();
    *new_val = ret_value;
    struct RUBrush ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_brush_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUBrush palette_highlight(struct RUBase* self_c) {
    WRPalette* qt_value = (WRPalette*)self_c;
    auto ret_value = qt_value->highlight();
    WRBrush* new_val = new WRBrush();
    *new_val = ret_value;
    struct RUBrush ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_brush_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUBrush palette_highlighted_text(struct RUBase* self_c) {
    WRPalette* qt_value = (WRPalette*)self_c;
    auto ret_value = qt_value->highlightedText();
    WRBrush* new_val = new WRBrush();
    *new_val = ret_value;
    struct RUBrush ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_brush_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUBrush palette_link(struct RUBase* self_c) {
    WRPalette* qt_value = (WRPalette*)self_c;
    auto ret_value = qt_value->link();
    WRBrush* new_val = new WRBrush();
    *new_val = ret_value;
    struct RUBrush ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_brush_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUBrush palette_link_visited(struct RUBase* self_c) {
    WRPalette* qt_value = (WRPalette*)self_c;
    auto ret_value = qt_value->linkVisited();
    WRBrush* new_val = new WRBrush();
    *new_val = ret_value;
    struct RUBrush ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_brush_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool palette_is_copy_of(struct RUBase* self_c, struct RUBase* p) {
    WRPalette* qt_value = (WRPalette*)self_c;
    auto ret_value = qt_value->isCopyOf(*((WRPalette*)p));
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int64_t palette_cache_key(struct RUBase* self_c) {
    WRPalette* qt_value = (WRPalette*)self_c;
    auto ret_value = qt_value->cacheKey();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPalette palette_resolve(struct RUBase* self_c, struct RUBase* arg0) {
    WRPalette* qt_value = (WRPalette*)self_c;
    auto ret_value = qt_value->resolve(*((WRPalette*)arg0));
    WRPalette* new_val = new WRPalette();
    *new_val = ret_value;
    struct RUPalette ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_palette_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static uint32_t palette_resolve_2(struct RUBase* self_c) {
    WRPalette* qt_value = (WRPalette*)self_c;
    auto ret_value = qt_value->resolve();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void palette_resolve_3(struct RUBase* self_c, uint32_t mask) {
    WRPalette* qt_value = (WRPalette*)self_c;
    qt_value->resolve(mask);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUPalette create_palette(
    struct RUBase* priv_data,
    RUDeleteCallback delete_callback,
    void* private_user_data)
{
    auto ctl = generic_create_func_with_delete<struct RUPalette, WRPalette>(priv_data, delete_callback, private_user_data);
    ctl.all_funcs = &s_palette_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_palette(struct RUBase* priv_data) {
    destroy_generic<WRPalette>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUPaletteFuncs s_palette_funcs = {
    destroy_palette,
    palette_swap,
    palette_current_color_group,
    palette_set_current_color_group,
    palette_color,
    palette_brush,
    palette_set_color,
    palette_set_color_2,
    palette_set_brush,
    palette_is_brush_set,
    palette_set_brush_2,
    palette_set_color_group,
    palette_is_equal,
    palette_color_2,
    palette_brush_2,
    palette_foreground,
    palette_window_text,
    palette_button,
    palette_light,
    palette_dark,
    palette_mid,
    palette_text,
    palette_base,
    palette_alternate_base,
    palette_tool_tip_base,
    palette_tool_tip_text,
    palette_background,
    palette_window,
    palette_midlight,
    palette_bright_text,
    palette_button_text,
    palette_shadow,
    palette_highlight,
    palette_highlighted_text,
    palette_link,
    palette_link_visited,
    palette_is_copy_of,
    palette_cache_key,
    palette_resolve,
    palette_resolve_2,
    palette_resolve_3,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUPaletteAllFuncs s_palette_all_funcs = {
    &s_palette_funcs,
};

