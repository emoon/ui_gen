////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// This file is auto-generated by rute_gen. DO NOT EDIT
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#include "../rute_base.h"
#include "../rute_manual.h"
#include <QListWidgetItem>
#include "list_widget_item_ffi.h"

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUListWidget list_widget_item_list_widget(struct RUBase* self_c) {
    WRListWidgetItem* qt_value = (WRListWidgetItem*)self_c;
    auto ret_value = qt_value->listWidget();
    struct RUListWidget ctl;
    ctl.qt_data = (struct RUBase*)ret_value;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)ret_value];
    ctl.all_funcs = &s_list_widget_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_item_set_selected(struct RUBase* self_c, bool select) {
    WRListWidgetItem* qt_value = (WRListWidgetItem*)self_c;
    qt_value->setSelected(select);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool list_widget_item_is_selected(struct RUBase* self_c) {
    WRListWidgetItem* qt_value = (WRListWidgetItem*)self_c;
    auto ret_value = qt_value->isSelected();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_item_set_hidden(struct RUBase* self_c, bool hide) {
    WRListWidgetItem* qt_value = (WRListWidgetItem*)self_c;
    qt_value->setHidden(hide);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool list_widget_item_is_hidden(struct RUBase* self_c) {
    WRListWidgetItem* qt_value = (WRListWidgetItem*)self_c;
    auto ret_value = qt_value->isHidden();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static uint32_t list_widget_item_flags(struct RUBase* self_c) {
    WRListWidgetItem* qt_value = (WRListWidgetItem*)self_c;
    auto ret_value = qt_value->flags();
    return (uint32_t)ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_item_set_flags(struct RUBase* self_c, uint32_t flags) {
    WRListWidgetItem* qt_value = (WRListWidgetItem*)self_c;
    qt_value->setFlags((Qt::ItemFlags)flags);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static const char* list_widget_item_text(struct RUBase* self_c) {
    WRListWidgetItem* qt_value = (WRListWidgetItem*)self_c;
    auto ret_value = qt_value->text();
    return q_string_to_const_char(ret_value);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_item_set_text(struct RUBase* self_c, const char* text) {
    WRListWidgetItem* qt_value = (WRListWidgetItem*)self_c;
    qt_value->setText(QString::fromUtf8(text));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUIcon list_widget_item_icon(struct RUBase* self_c) {
    WRListWidgetItem* qt_value = (WRListWidgetItem*)self_c;
    auto ret_value = qt_value->icon();
    WRIcon* new_val = new WRIcon();
    *new_val = ret_value;
    struct RUIcon ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_icon_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_item_set_icon(struct RUBase* self_c, struct RUBase* icon) {
    WRListWidgetItem* qt_value = (WRListWidgetItem*)self_c;
    qt_value->setIcon(*((WRIcon*)icon));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static const char* list_widget_item_status_tip(struct RUBase* self_c) {
    WRListWidgetItem* qt_value = (WRListWidgetItem*)self_c;
    auto ret_value = qt_value->statusTip();
    return q_string_to_const_char(ret_value);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_item_set_status_tip(struct RUBase* self_c, const char* status_tip) {
    WRListWidgetItem* qt_value = (WRListWidgetItem*)self_c;
    qt_value->setStatusTip(QString::fromUtf8(status_tip));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static const char* list_widget_item_tool_tip(struct RUBase* self_c) {
    WRListWidgetItem* qt_value = (WRListWidgetItem*)self_c;
    auto ret_value = qt_value->toolTip();
    return q_string_to_const_char(ret_value);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_item_set_tool_tip(struct RUBase* self_c, const char* tool_tip) {
    WRListWidgetItem* qt_value = (WRListWidgetItem*)self_c;
    qt_value->setToolTip(QString::fromUtf8(tool_tip));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static const char* list_widget_item_whats_this(struct RUBase* self_c) {
    WRListWidgetItem* qt_value = (WRListWidgetItem*)self_c;
    auto ret_value = qt_value->whatsThis();
    return q_string_to_const_char(ret_value);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_item_set_whats_this(struct RUBase* self_c, const char* whats_this) {
    WRListWidgetItem* qt_value = (WRListWidgetItem*)self_c;
    qt_value->setWhatsThis(QString::fromUtf8(whats_this));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUFont list_widget_item_font(struct RUBase* self_c) {
    WRListWidgetItem* qt_value = (WRListWidgetItem*)self_c;
    auto ret_value = qt_value->font();
    WRFont* new_val = new WRFont();
    *new_val = ret_value;
    struct RUFont ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_font_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_item_set_font(struct RUBase* self_c, struct RUBase* font) {
    WRListWidgetItem* qt_value = (WRListWidgetItem*)self_c;
    qt_value->setFont(*((WRFont*)font));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int list_widget_item_text_alignment(struct RUBase* self_c) {
    WRListWidgetItem* qt_value = (WRListWidgetItem*)self_c;
    auto ret_value = qt_value->textAlignment();
    return ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_item_set_text_alignment(struct RUBase* self_c, int alignment) {
    WRListWidgetItem* qt_value = (WRListWidgetItem*)self_c;
    qt_value->setTextAlignment(alignment);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUBrush list_widget_item_background(struct RUBase* self_c) {
    WRListWidgetItem* qt_value = (WRListWidgetItem*)self_c;
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

static void list_widget_item_set_background(struct RUBase* self_c, struct RUBase* brush) {
    WRListWidgetItem* qt_value = (WRListWidgetItem*)self_c;
    qt_value->setBackground(*((WRBrush*)brush));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUColor list_widget_item_text_color(struct RUBase* self_c) {
    WRListWidgetItem* qt_value = (WRListWidgetItem*)self_c;
    auto ret_value = qt_value->textColor();
    WRColor* new_val = new WRColor();
    *new_val = ret_value;
    struct RUColor ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_color_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_item_set_text_color(struct RUBase* self_c, struct RUBase* color) {
    WRListWidgetItem* qt_value = (WRListWidgetItem*)self_c;
    qt_value->setTextColor(*((WRColor*)color));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUBrush list_widget_item_foreground(struct RUBase* self_c) {
    WRListWidgetItem* qt_value = (WRListWidgetItem*)self_c;
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

static void list_widget_item_set_foreground(struct RUBase* self_c, struct RUBase* brush) {
    WRListWidgetItem* qt_value = (WRListWidgetItem*)self_c;
    qt_value->setForeground(*((WRBrush*)brush));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static uint32_t list_widget_item_check_state(struct RUBase* self_c) {
    WRListWidgetItem* qt_value = (WRListWidgetItem*)self_c;
    auto ret_value = qt_value->checkState();
    return (uint32_t)ret_value;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_item_set_check_state(struct RUBase* self_c, uint32_t state) {
    WRListWidgetItem* qt_value = (WRListWidgetItem*)self_c;
    qt_value->setCheckState((Qt::CheckState)state);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUSize list_widget_item_size_hint(struct RUBase* self_c) {
    WRListWidgetItem* qt_value = (WRListWidgetItem*)self_c;
    auto ret_value = qt_value->sizeHint();
    WRSize* new_val = new WRSize();
    *new_val = ret_value;
    struct RUSize ctl;
    ctl.qt_data = (struct RUBase*)new_val;
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)new_val];
    ctl.all_funcs = &s_size_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void list_widget_item_set_size_hint(struct RUBase* self_c, struct RUBase* size) {
    WRListWidgetItem* qt_value = (WRListWidgetItem*)self_c;
    qt_value->setSizeHint(*((WRSize*)size));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct RUListWidgetItem create_list_widget_item(
    struct RUBase* priv_data,
    RUDeleteCallback delete_callback,
    void* private_user_data)
{
    auto ctl = generic_create_func_with_delete<struct RUListWidgetItem, WRListWidgetItem>(priv_data, delete_callback, private_user_data);
    ctl.all_funcs = &s_list_widget_item_all_funcs;
    return ctl;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_list_widget_item(struct RUBase* priv_data) {
    destroy_generic<WRListWidgetItem>(priv_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUListWidgetItemFuncs s_list_widget_item_funcs = {
    destroy_list_widget_item,
    list_widget_item_list_widget,
    list_widget_item_set_selected,
    list_widget_item_is_selected,
    list_widget_item_set_hidden,
    list_widget_item_is_hidden,
    list_widget_item_flags,
    list_widget_item_set_flags,
    list_widget_item_text,
    list_widget_item_set_text,
    list_widget_item_icon,
    list_widget_item_set_icon,
    list_widget_item_status_tip,
    list_widget_item_set_status_tip,
    list_widget_item_tool_tip,
    list_widget_item_set_tool_tip,
    list_widget_item_whats_this,
    list_widget_item_set_whats_this,
    list_widget_item_font,
    list_widget_item_set_font,
    list_widget_item_text_alignment,
    list_widget_item_set_text_alignment,
    list_widget_item_background,
    list_widget_item_set_background,
    list_widget_item_text_color,
    list_widget_item_set_text_color,
    list_widget_item_foreground,
    list_widget_item_set_foreground,
    list_widget_item_check_state,
    list_widget_item_set_check_state,
    list_widget_item_size_hint,
    list_widget_item_set_size_hint,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct RUListWidgetItemAllFuncs s_list_widget_item_all_funcs = {
    &s_list_widget_item_funcs,
};

