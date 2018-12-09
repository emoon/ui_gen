// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once

#include <stdbool.h>
#include <stdint.h>

#include "../rute_base.h"

#ifdef __cplusplus
extern "C" {
#endif
#include "margins_ffi.h"

struct RULineEditFuncs;
struct RULineEdit;

typedef struct RULineEditFuncs {
    void (*destroy)(struct RUBase* self);
    const char* (*text)(struct RUBase* self_c);
    const char* (*display_text)(struct RUBase* self_c);
    const char* (*placeholder_text)(struct RUBase* self_c);
    void (*set_placeholder_text)(struct RUBase* self_c, const char* arg0);
    int (*max_length)(struct RUBase* self_c);
    void (*set_max_length)(struct RUBase* self_c, int arg0);
    void (*set_frame)(struct RUBase* self_c, bool arg0);
    bool (*has_frame)(struct RUBase* self_c);
    void (*set_clear_button_enabled)(struct RUBase* self_c, bool enable);
    bool (*is_clear_button_enabled)(struct RUBase* self_c);
    int (*echo_mode)(struct RUBase* self_c);
    void (*set_echo_mode)(struct RUBase* self_c, int arg0);
    bool (*is_read_only)(struct RUBase* self_c);
    void (*set_read_only)(struct RUBase* self_c, bool arg0);
    int (*cursor_position)(struct RUBase* self_c);
    void (*set_cursor_position)(struct RUBase* self_c, int arg0);
    int (*cursor_position_at)(struct RUBase* self_c, struct RUBase* pos);
    void (*set_alignment)(struct RUBase* self_c, int flag);
    int (*alignment)(struct RUBase* self_c);
    void (*cursor_forward)(struct RUBase* self_c, bool mark, int steps);
    void (*cursor_backward)(struct RUBase* self_c, bool mark, int steps);
    void (*cursor_word_forward)(struct RUBase* self_c, bool mark);
    void (*cursor_word_backward)(struct RUBase* self_c, bool mark);
    void (*backspace)(struct RUBase* self_c);
    void (*del)(struct RUBase* self_c);
    void (*home)(struct RUBase* self_c, bool mark);
    void (*end)(struct RUBase* self_c, bool mark);
    bool (*is_modified)(struct RUBase* self_c);
    void (*set_modified)(struct RUBase* self_c, bool arg0);
    void (*set_selection)(struct RUBase* self_c, int arg0, int arg1);
    bool (*has_selected_text)(struct RUBase* self_c);
    const char* (*selected_text)(struct RUBase* self_c);
    int (*selection_start)(struct RUBase* self_c);
    int (*selection_end)(struct RUBase* self_c);
    int (*selection_length)(struct RUBase* self_c);
    bool (*is_undo_available)(struct RUBase* self_c);
    bool (*is_redo_available)(struct RUBase* self_c);
    void (*set_drag_enabled)(struct RUBase* self_c, bool b);
    bool (*drag_enabled)(struct RUBase* self_c);
    void (*set_cursor_move_style)(struct RUBase* self_c, int style);
    int (*cursor_move_style)(struct RUBase* self_c);
    const char* (*input_mask)(struct RUBase* self_c);
    void (*set_input_mask)(struct RUBase* self_c, const char* input_mask);
    bool (*has_acceptable_input)(struct RUBase* self_c);
    void (*set_text_margins)(struct RUBase* self_c, int left, int top,
                             int right, int bottom);
    void (*set_text_margins_2)(struct RUBase* self_c, struct RUBase* margins);
    struct RUMargins (*text_margins)(struct RUBase* self_c);
    void (*set_text)(struct RUBase* self_c, const char* arg0);
    void (*clear)(struct RUBase* self_c);
    void (*select_all)(struct RUBase* self_c);
    void (*undo)(struct RUBase* self_c);
    void (*redo)(struct RUBase* self_c);
    void (*cut)(struct RUBase* self_c);
    void (*copy)(struct RUBase* self_c);
    void (*paste)(struct RUBase* self_c);
    void (*deselect)(struct RUBase* self_c);
    void (*insert)(struct RUBase* self_c, const char* arg0);
    void (*set_text_changed_event)(void* object, void* user_data,
                                   void* wrapped_func,
                                   void (*event)(void*, void* self_c,
                                                 const char* arg0));
    void (*set_text_edited_event)(void* object, void* user_data,
                                  void* wrapped_func,
                                  void (*event)(void*, void* self_c,
                                                const char* arg0));
    void (*set_cursor_position_changed_event)(
        void* object, void* user_data, void* wrapped_func,
        void (*event)(void*, void* self_c, int arg0, int arg1));
    void (*set_return_pressed_event)(void* object, void* user_data,
                                     void* wrapped_func,
                                     void (*event)(void*, void* self_c));
    void (*set_editing_finished_event)(void* object, void* user_data,
                                       void* wrapped_func,
                                       void (*event)(void*, void* self_c));
    void (*set_selection_changed_event)(void* object, void* user_data,
                                        void* wrapped_func,
                                        void (*event)(void*, void* self_c));
    void (*set_mouse_press_event)(void* object, void* user_data,
                                  void* wrapped_func,
                                  void (*event)(void*, void* self_c,
                                                struct RUBase* arg0));
    void (*remove_mouse_press_event)(void* object);
    void (*set_mouse_move_event)(void* object, void* user_data,
                                 void* wrapped_func,
                                 void (*event)(void*, void* self_c,
                                               struct RUBase* arg0));
    void (*remove_mouse_move_event)(void* object);
    void (*set_mouse_release_event)(void* object, void* user_data,
                                    void* wrapped_func,
                                    void (*event)(void*, void* self_c,
                                                  struct RUBase* arg0));
    void (*remove_mouse_release_event)(void* object);
    void (*set_mouse_double_click_event)(void* object, void* user_data,
                                         void* wrapped_func,
                                         void (*event)(void*, void* self_c,
                                                       struct RUBase* arg0));
    void (*remove_mouse_double_click_event)(void* object);
    void (*set_key_press_event)(void* object, void* user_data,
                                void* wrapped_func,
                                void (*event)(void*, void* self_c,
                                              struct RUBase* arg0));
    void (*remove_key_press_event)(void* object);
    void (*set_focus_in_event)(void* object, void* user_data,
                               void* wrapped_func,
                               void (*event)(void*, void* self_c,
                                             struct RUBase* arg0));
    void (*remove_focus_in_event)(void* object);
    void (*set_focus_out_event)(void* object, void* user_data,
                                void* wrapped_func,
                                void (*event)(void*, void* self_c,
                                              struct RUBase* arg0));
    void (*remove_focus_out_event)(void* object);
    void (*set_paint_event)(void* object, void* user_data, void* wrapped_func,
                            void (*event)(void*, void* self_c,
                                          struct RUBase* arg0));
    void (*remove_paint_event)(void* object);
    void (*set_drag_enter_event)(void* object, void* user_data,
                                 void* wrapped_func,
                                 void (*event)(void*, void* self_c,
                                               struct RUBase* arg0));
    void (*remove_drag_enter_event)(void* object);
    void (*set_drag_move_event)(void* object, void* user_data,
                                void* wrapped_func,
                                void (*event)(void*, void* self_c,
                                              struct RUBase* e));
    void (*remove_drag_move_event)(void* object);
    void (*set_drag_leave_event)(void* object, void* user_data,
                                 void* wrapped_func,
                                 void (*event)(void*, void* self_c,
                                               struct RUBase* e));
    void (*remove_drag_leave_event)(void* object);
    void (*set_drop_event)(void* object, void* user_data, void* wrapped_func,
                           void (*event)(void*, void* self_c,
                                         struct RUBase* arg0));
    void (*remove_drop_event)(void* object);
    void (*set_change_event)(void* object, void* user_data, void* wrapped_func,
                             void (*event)(void*, void* self_c,
                                           struct RUBase* arg0));
    void (*remove_change_event)(void* object);
    void (*set_context_menu_event)(void* object, void* user_data,
                                   void* wrapped_func,
                                   void (*event)(void*, void* self_c,
                                                 struct RUBase* arg0));
    void (*remove_context_menu_event)(void* object);
} RULineEditFuncs;

typedef struct RULineEditAllFuncs {
    struct RUObjectFuncs* object_funcs;
    struct RUPaintDeviceFuncs* paint_device_funcs;
    struct RUWidgetFuncs* widget_funcs;
    struct RULineEditFuncs* line_edit_funcs;
} RULineEditAllFuncs;

typedef struct RULineEdit {
    RUBase* qt_data;
    RUBase* host_data;
    struct RULineEditAllFuncs* all_funcs;
} RULineEdit;

extern RULineEditFuncs s_line_edit_funcs;
extern RULineEditAllFuncs s_line_edit_all_funcs;

#ifdef __cplusplus
}
#endif
