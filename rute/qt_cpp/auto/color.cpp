////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// This file is auto-generated by rute_gen. DO NOT EDIT
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#include "../rute_base.h"
#include "../rute_manual.h"
#include <QColor>
#include "color_ffi.h"

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool color_is_valid(struct RUBase* self_c) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->isValid();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static const char* color_name(struct RUBase* self_c) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->name();
    return q_string_to_const_char(ret_value);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static const char* color_name(struct RUBase* self_c, int format) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->name((Color::NameFormat)s_name_format_lookup[format]);
    return q_string_to_const_char(ret_value);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void color_set_named_color(struct RUBase* self_c, const char* name) {
    WRColor* qt_value = (WRColor*)self_c;
    qt_value->setNamedColor(QString::fromUtf8(name));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void color_set_named_color(struct RUBase* self_c, struct RUStringViewType name) {
    WRColor* qt_value = (WRColor*)self_c;
    qt_value->setNamedColor(*((QStringView*)name));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void color_set_named_color(struct RUBase* self_c, struct RULatin1StringType name) {
    WRColor* qt_value = (WRColor*)self_c;
    qt_value->setNamedColor(*((QLatin1String*)name));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUArray color_color_names(struct RUBase* self_c) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->colorNames();
    return return_string_array(ret_value);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int color_spec(struct RUBase* self_c) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->spec();
    return s_spec_lookup[(int)ret_value];
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int color_alpha(struct RUBase* self_c) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->alpha();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void color_set_alpha(struct RUBase* self_c, int alpha) {
    WRColor* qt_value = (WRColor*)self_c;
    qt_value->setAlpha(alpha);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float color_alpha_f(struct RUBase* self_c) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->alphaF();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void color_set_alpha_f(struct RUBase* self_c, float alpha) {
    WRColor* qt_value = (WRColor*)self_c;
    qt_value->setAlphaF(alpha);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int color_red(struct RUBase* self_c) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->red();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int color_green(struct RUBase* self_c) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->green();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int color_blue(struct RUBase* self_c) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->blue();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void color_set_red(struct RUBase* self_c, int red) {
    WRColor* qt_value = (WRColor*)self_c;
    qt_value->setRed(red);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void color_set_green(struct RUBase* self_c, int green) {
    WRColor* qt_value = (WRColor*)self_c;
    qt_value->setGreen(green);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void color_set_blue(struct RUBase* self_c, int blue) {
    WRColor* qt_value = (WRColor*)self_c;
    qt_value->setBlue(blue);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float color_red_f(struct RUBase* self_c) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->redF();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float color_green_f(struct RUBase* self_c) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->greenF();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float color_blue_f(struct RUBase* self_c) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->blueF();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void color_set_red_f(struct RUBase* self_c, float red) {
    WRColor* qt_value = (WRColor*)self_c;
    qt_value->setRedF(red);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void color_set_green_f(struct RUBase* self_c, float green) {
    WRColor* qt_value = (WRColor*)self_c;
    qt_value->setGreenF(green);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void color_set_blue_f(struct RUBase* self_c, float blue) {
    WRColor* qt_value = (WRColor*)self_c;
    qt_value->setBlueF(blue);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void color_set_rgb(struct RUBase* self_c, int r, int g, int b, int a) {
    WRColor* qt_value = (WRColor*)self_c;
    qt_value->setRgb(r, g, b, a);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void color_get_rgb_f(struct RUBase* self_c, struct RUBase* r, struct RUBase* g, struct RUBase* b, struct RUBase* a) {
    WRColor* qt_value = (WRColor*)self_c;
    qt_value->getRgbF(r, g, b, a);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void color_set_rgb_f(struct RUBase* self_c, float r, float g, float b, float a) {
    WRColor* qt_value = (WRColor*)self_c;
    qt_value->setRgbF(r, g, b, a);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int color_hue(struct RUBase* self_c) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->hue();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int color_saturation(struct RUBase* self_c) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->saturation();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int color_hsv_hue(struct RUBase* self_c) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->hsvHue();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int color_hsv_saturation(struct RUBase* self_c) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->hsvSaturation();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int color_value(struct RUBase* self_c) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->value();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float color_hue_f(struct RUBase* self_c) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->hueF();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float color_saturation_f(struct RUBase* self_c) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->saturationF();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float color_hsv_hue_f(struct RUBase* self_c) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->hsvHueF();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float color_hsv_saturation_f(struct RUBase* self_c) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->hsvSaturationF();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float color_value_f(struct RUBase* self_c) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->valueF();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void color_get_hsv(struct RUBase* self_c, struct RUBase* h, struct RUBase* s, struct RUBase* v, struct RUBase* a) {
    WRColor* qt_value = (WRColor*)self_c;
    qt_value->getHsv(h, s, v, a);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void color_set_hsv(struct RUBase* self_c, int h, int s, int v, int a) {
    WRColor* qt_value = (WRColor*)self_c;
    qt_value->setHsv(h, s, v, a);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void color_get_hsv_f(struct RUBase* self_c, struct RUBase* h, struct RUBase* s, struct RUBase* v, struct RUBase* a) {
    WRColor* qt_value = (WRColor*)self_c;
    qt_value->getHsvF(h, s, v, a);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void color_set_hsv_f(struct RUBase* self_c, float h, float s, float v, float a) {
    WRColor* qt_value = (WRColor*)self_c;
    qt_value->setHsvF(h, s, v, a);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int color_cyan(struct RUBase* self_c) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->cyan();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int color_magenta(struct RUBase* self_c) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->magenta();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int color_yellow(struct RUBase* self_c) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->yellow();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int color_black(struct RUBase* self_c) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->black();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float color_cyan_f(struct RUBase* self_c) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->cyanF();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float color_magenta_f(struct RUBase* self_c) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->magentaF();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float color_yellow_f(struct RUBase* self_c) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->yellowF();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float color_black_f(struct RUBase* self_c) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->blackF();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void color_get_cmyk(struct RUBase* self_c, struct RUBase* c, struct RUBase* m, struct RUBase* y, struct RUBase* k, struct RUBase* a) {
    WRColor* qt_value = (WRColor*)self_c;
    qt_value->getCmyk(c, m, y, k, a);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void color_set_cmyk(struct RUBase* self_c, int c, int m, int y, int k, int a) {
    WRColor* qt_value = (WRColor*)self_c;
    qt_value->setCmyk(c, m, y, k, a);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void color_get_cmyk_f(struct RUBase* self_c, struct RUBase* c, struct RUBase* m, struct RUBase* y, struct RUBase* k, struct RUBase* a) {
    WRColor* qt_value = (WRColor*)self_c;
    qt_value->getCmykF(c, m, y, k, a);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void color_set_cmyk_f(struct RUBase* self_c, float c, float m, float y, float k, float a) {
    WRColor* qt_value = (WRColor*)self_c;
    qt_value->setCmykF(c, m, y, k, a);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int color_hsl_hue(struct RUBase* self_c) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->hslHue();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int color_hsl_saturation(struct RUBase* self_c) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->hslSaturation();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int color_lightness(struct RUBase* self_c) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->lightness();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float color_hsl_hue_f(struct RUBase* self_c) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->hslHueF();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float color_hsl_saturation_f(struct RUBase* self_c) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->hslSaturationF();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static float color_lightness_f(struct RUBase* self_c) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->lightnessF();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void color_get_hsl(struct RUBase* self_c, struct RUBase* h, struct RUBase* s, struct RUBase* l, struct RUBase* a) {
    WRColor* qt_value = (WRColor*)self_c;
    qt_value->getHsl(h, s, l, a);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void color_set_hsl(struct RUBase* self_c, int h, int s, int l, int a) {
    WRColor* qt_value = (WRColor*)self_c;
    qt_value->setHsl(h, s, l, a);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void color_get_hsl_f(struct RUBase* self_c, struct RUBase* h, struct RUBase* s, struct RUBase* l, struct RUBase* a) {
    WRColor* qt_value = (WRColor*)self_c;
    qt_value->getHslF(h, s, l, a);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void color_set_hsl_f(struct RUBase* self_c, float h, float s, float l, float a) {
    WRColor* qt_value = (WRColor*)self_c;
    qt_value->setHslF(h, s, l, a);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUColor color_to_rgb(struct RUBase* self_c) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->toRgb();
    WRColor* new_val = new WRColor();
    *new_val = ret_value;
    struct RUColor ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_color_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUColor color_to_hsv(struct RUBase* self_c) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->toHsv();
    WRColor* new_val = new WRColor();
    *new_val = ret_value;
    struct RUColor ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_color_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUColor color_to_cmyk(struct RUBase* self_c) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->toCmyk();
    WRColor* new_val = new WRColor();
    *new_val = ret_value;
    struct RUColor ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_color_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUColor color_to_hsl(struct RUBase* self_c) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->toHsl();
    WRColor* new_val = new WRColor();
    *new_val = ret_value;
    struct RUColor ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_color_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUColor color_convert_to(struct RUBase* self_c, int color_spec) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->convertTo((Color::Spec)s_spec_lookup[color_spec]);
    WRColor* new_val = new WRColor();
    *new_val = ret_value;
    struct RUColor ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_color_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUColor color_from_rgb(struct RUBase* self_c, int r, int g, int b, int a) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->fromRgb(r, g, b, a);
    WRColor* new_val = new WRColor();
    *new_val = ret_value;
    struct RUColor ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_color_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUColor color_from_rgb_f(struct RUBase* self_c, float r, float g, float b, float a) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->fromRgbF(r, g, b, a);
    WRColor* new_val = new WRColor();
    *new_val = ret_value;
    struct RUColor ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_color_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUColor color_from_rgba64(struct RUBase* self_c, uint16_t r, uint16_t g, uint16_t b, uint16_t a) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->fromRgba64(r, g, b, a);
    WRColor* new_val = new WRColor();
    *new_val = ret_value;
    struct RUColor ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_color_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUColor color_from_hsv(struct RUBase* self_c, int h, int s, int v, int a) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->fromHsv(h, s, v, a);
    WRColor* new_val = new WRColor();
    *new_val = ret_value;
    struct RUColor ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_color_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUColor color_from_hsv_f(struct RUBase* self_c, float h, float s, float v, float a) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->fromHsvF(h, s, v, a);
    WRColor* new_val = new WRColor();
    *new_val = ret_value;
    struct RUColor ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_color_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUColor color_from_cmyk(struct RUBase* self_c, int c, int m, int y, int k, int a) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->fromCmyk(c, m, y, k, a);
    WRColor* new_val = new WRColor();
    *new_val = ret_value;
    struct RUColor ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_color_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUColor color_from_cmyk_f(struct RUBase* self_c, float c, float m, float y, float k, float a) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->fromCmykF(c, m, y, k, a);
    WRColor* new_val = new WRColor();
    *new_val = ret_value;
    struct RUColor ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_color_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUColor color_from_hsl(struct RUBase* self_c, int h, int s, int l, int a) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->fromHsl(h, s, l, a);
    WRColor* new_val = new WRColor();
    *new_val = ret_value;
    struct RUColor ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_color_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUColor color_from_hsl_f(struct RUBase* self_c, float h, float s, float l, float a) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->fromHslF(h, s, l, a);
    WRColor* new_val = new WRColor();
    *new_val = ret_value;
    struct RUColor ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_color_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUColor color_light(struct RUBase* self_c, int f) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->light(f);
    WRColor* new_val = new WRColor();
    *new_val = ret_value;
    struct RUColor ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_color_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUColor color_lighter(struct RUBase* self_c, int f) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->lighter(f);
    WRColor* new_val = new WRColor();
    *new_val = ret_value;
    struct RUColor ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_color_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUColor color_dark(struct RUBase* self_c, int f) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->dark(f);
    WRColor* new_val = new WRColor();
    *new_val = ret_value;
    struct RUColor ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_color_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUColor color_darker(struct RUBase* self_c, int f) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->darker(f);
    WRColor* new_val = new WRColor();
    *new_val = ret_value;
    struct RUColor ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_color_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool color_is_valid_color(struct RUBase* self_c, const char* name) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->isValidColor(QString::fromUtf8(name));
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool color_is_valid_color(struct RUBase* self_c, struct RUStringViewType arg0) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->isValidColor(*((QStringView*)arg0));
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool color_is_valid_color(struct RUBase* self_c, struct RULatin1StringType arg0) {
    WRColor* qt_value = (WRColor*)self_c;
    auto ret_value = qt_value->isValidColor(*((QLatin1String*)arg0));
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUColor create_color(
    struct RUBase* priv_data,
    RUDeleteCallback delete_callback,
    void* private_user_data)
{
    auto ctl = generic_create_func_with_delete<struct RUColor, WRColor>(priv_data, delete_callback, private_user_data);
    ctl.all_funcs = &s_color_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_color(struct RUBase* priv_data) {
    destroy_generic<WRColor>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUColor get_color(struct RUBase* priv_data) {
    (void)priv_data;
    RUColor ctl;
    ctl.qt_data = nullptr;
    ctl.host_data = nullptr;
    ctl.all_funcs = &s_color_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUColorFuncs s_color_funcs = {
    destroy_color,
    color_is_valid,
    color_name,
    color_name,
    color_set_named_color,
    color_set_named_color,
    color_set_named_color,
    color_color_names,
    color_spec,
    color_alpha,
    color_set_alpha,
    color_alpha_f,
    color_set_alpha_f,
    color_red,
    color_green,
    color_blue,
    color_set_red,
    color_set_green,
    color_set_blue,
    color_red_f,
    color_green_f,
    color_blue_f,
    color_set_red_f,
    color_set_green_f,
    color_set_blue_f,
    color_set_rgb,
    color_get_rgb_f,
    color_set_rgb_f,
    color_hue,
    color_saturation,
    color_hsv_hue,
    color_hsv_saturation,
    color_value,
    color_hue_f,
    color_saturation_f,
    color_hsv_hue_f,
    color_hsv_saturation_f,
    color_value_f,
    color_get_hsv,
    color_set_hsv,
    color_get_hsv_f,
    color_set_hsv_f,
    color_cyan,
    color_magenta,
    color_yellow,
    color_black,
    color_cyan_f,
    color_magenta_f,
    color_yellow_f,
    color_black_f,
    color_get_cmyk,
    color_set_cmyk,
    color_get_cmyk_f,
    color_set_cmyk_f,
    color_hsl_hue,
    color_hsl_saturation,
    color_lightness,
    color_hsl_hue_f,
    color_hsl_saturation_f,
    color_lightness_f,
    color_get_hsl,
    color_set_hsl,
    color_get_hsl_f,
    color_set_hsl_f,
    color_to_rgb,
    color_to_hsv,
    color_to_cmyk,
    color_to_hsl,
    color_convert_to,
    color_from_rgb,
    color_from_rgb_f,
    color_from_rgba64,
    color_from_hsv,
    color_from_hsv_f,
    color_from_cmyk,
    color_from_cmyk_f,
    color_from_hsl,
    color_from_hsl_f,
    color_light,
    color_lighter,
    color_dark,
    color_darker,
    color_is_valid_color,
    color_is_valid_color,
    color_is_valid_color,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUColorAllFuncs s_color_all_funcs = {
    &s_color_funcs,
};

