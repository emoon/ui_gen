// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once

#include <stdbool.h>
#include <stdint.h>

#include "../rute_base.h"

#ifdef __cplusplus
extern "C" {
#endif
#include "key_sequence_ffi.h"

struct RUKeySequenceFuncs;
struct RUKeySequence;

typedef struct RUKeySequenceFuncs {
    void (*destroy)(struct RUBase* self);
    int (*count)(struct RUBase* self_c);
    bool (*is_empty)(struct RUBase* self_c);
    int (*matches)(struct RUBase* self_c, struct RUBase* seq);
    struct RUKeySequence (*mnemonic)(struct RUBase* self_c, const char* text);
    void (*swap)(struct RUBase* self_c, struct RUBase* other);
    bool (*is_detached)(struct RUBase* self_c);
} RUKeySequenceFuncs;

typedef struct RUKeySequenceAllFuncs {
    struct RUKeySequenceFuncs* key_sequence_funcs;
} RUKeySequenceAllFuncs;

typedef struct RUKeySequence {
    RUBase* qt_data;
    RUBase* host_data;
    struct RUKeySequenceAllFuncs* all_funcs;
} RUKeySequence;

extern RUKeySequenceFuncs s_key_sequence_funcs;
extern RUKeySequenceAllFuncs s_key_sequence_all_funcs;

#ifdef __cplusplus
}
#endif
