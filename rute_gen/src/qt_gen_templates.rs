///
/// Written at the start of the header and cpp output
///
pub static HEADER: &'static [u8] = b"
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// This file is auto-generated by rute_gen. DO NOT EDIT
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#include \"Rute.h\"
#include \"rute_cpp.h\"
#include \"../rute_manual.h\"
";

//
// Used to make QT generated code a bit easier to read
//
pub static SEPARATOR: &'static [u8] = b"///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////\n\n";

pub static QT_HEADER: &'static [u8] = b"
static char s_temp_string_buffer[1024*1024];\n
#include <map>
std::map<QWidget*, void*> s_widget_lookup;\n
";

///
/// Writen at the end of the cpp output
///
pub static FOOTER: &'static [u8] = b"
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#ifdef _WIN32
extern \"C\" __declspec(dllexport) struct Rute* rute_get() {
#else
extern \"C\" struct Rute* rute_get() {
#endif
    return (Rute*)&s_rute;
}
";


///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub static QT_GEN_WRAPPER_TEMPLATE: &str = "
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

class WR{{struct_name}} : public {{cpp_name}} {
    Q_OBJECT
public:
{% if widget %}
    WR{{struct_name}}({{cpp_name}}* widget) : {{cpp_name}}(widget) { }
{% else %}
    WR{{struct_name}}() : {{cpp_name}}() { }
{% endif %}
    virtual ~WR{{struct_name}}() {
        if (m_delete_callback) {
             m_delete_callback(m_private_data);
         }
    }

    {{events}}

    RUDeleteCallback m_delete_callback = nullptr;
    void* m_private_data = nullptr;
};
";

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub static SIGNAL_WRAPPER_TEMPLATE: &str = "
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

typedef void (*Signal_{{signal_func_name}})(void* self_c, void* trampoline_func{{c_args}});

class QSlotWrapperSignal_{{signal_func_name}} : public QObject {
    Q_OBJECT
public:
    QSlotWrapperSignal_self_int_void(void* data, Signal_{{signal_func_name}} trampoline_func, void* wrapped_func) {
        m_trampoline_func = trampoline_func;
        m_data = data;
        m_wrapped_func = wrapped_func;
    }

    Q_SLOT void method({{c_args | remove_first: \",\"}}) {
        m_trampoline_func(m_data, m_wrapped_func{{c_call_args}});
    }
private:
    Signal_{{signal_func_name}} m_trampoline_func;
    void* m_data;
    void* m_wrapped_func;
};
";

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Definition for a Qt wrapping function

/*
pub static QT_FUNC_DEF_TEMPLATE: &str = "
static {{func_name}}({{func_def}}) {
    {{qt_type_name}} qt_data = ({{qt_type_name}}*)self_c;
{% if early_return != "" %}
    {{early_return}}
{% else %}

{% endif}



}
";
*/


