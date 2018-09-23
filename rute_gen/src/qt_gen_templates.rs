///
/// Written at the start of the header and cpp output
///
pub static HEADER: &'static [u8] = b"////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// This file is auto-generated by rute_gen. DO NOT EDIT
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#include \"../rute_base.h\"
#include \"../rute_manual.h\"
";

//
// Used to make QT generated code a bit easier to read
//
pub static SEPARATOR: &'static [u8] = b"///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////\n\n";

pub static QT_MAIN_HEADER: &'static [u8] = b"
static char s_temp_string_buffer[1024*1024];\n
#include <map>
std::map<void*, void*> s_host_data_lookup;\n
struct KeyVal { int val, key; };\n

const char* q_string_to_const_char(const QString& ret_value) {
    QByteArray ba = ret_value.toUtf8();
    const char* c_str = ba.data();
    assert((ba.size() + 1) < sizeof(s_temp_string_buffer), \"buffer not large enough\");
    memcpy(s_temp_string_buffer, c_str, ba.size() + 1);
    return s_temp_string_buffer;\n)
}
";

///
/// Writen at the end of the cpp output
///
pub static QT_MAIN_FOOTER: &'static [u8] = b"
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

class WR{{struct_name}} : public {{qt_name}} {
    Q_OBJECT
public:
{%- if widget %}
    WR{{struct_name}}(QWidget* widget) : {{qt_name}}(widget) { }
{%- else %}
    WR{{struct_name}}() : {{qt_name}}() { }
{%- endif %}
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

//////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub static QT_ENUM_MAPPING_TEMPLATE: &str = "
    static KeyVal {{enum_name}}_vals[] =
    {
    {%- for enum in enums -%}
        {  (int){{qt_class}}::{{enum.name}}, {{enum.id}} },
    {% endfor %}};

    for (int i = 0; i < {{enums | size }} ++i) {
        s_{{enum_name}}_lookup[{{enum_name}}_vals[i].key] = {{enum_name}}_vals[i].val;
    }
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

pub static QT_FUNC_DEF_TEMPLATE: &str = "static {{c_return_type}} {{func_name}}({{func_def}}) {
    {{cpp_type_name}}* qt_value = ({{cpp_type_name}}*)self_c;
{% if c_return_type != 'void' %}
    auto ret_value = qt_value->{{qt_func_name}}({{qt_func_args}});

{%- if array_return %}
{%- case return_type %}
{%- when 'string' %}
    return return_string_array(ret_value);
{%- when 'primitive' %}
    return return_primitive_array<{{c_primitive_type}}>(ret_value);
{%- when 'regular' %}
    return return_by_value_array<{{qt_type}}>(ret_value);
{%- when 'reference' %}
    return return_pointer_array<{{qt_type}}>(ret_value);
{%- endcase %}

{%- else %}

{%- case return_type %}
{%- when 'string' %}
    return q_string_to_const_char(&ret_value);
{%- when 'array' %}
    return build_array(ret_value);
{%- when 'primitive' %}
    return ret_value;
{%- else %}
    {{c_return_type}} ctl;
    ctl.qt_data = (struct RUBase*){{qt_ret_value}};
    ctl.host_data = (struct RUBase*)s_host_data_lookup[(void*)ret_value];
    ctl.all_funcs = &s_{{funcs_name}}_all_funcs;
    return ctl;
{%- endcase %}

{%- endif %}

{%- else %}
    qt_value->{{qt_func_name}}({{qt_func_args}});
{%- endif %}
}

";


