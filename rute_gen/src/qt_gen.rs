use c_gen::*;
use heck::MixedCase;
use heck::SnakeCase;
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};
use std::path::Path;

use api_parser::*;
use qt_gen_templates::*;

use liquid::{Object, ParserBuilder, Template, Value};

#[derive(PartialEq)]
pub enum EventType {
    Callback,
    Event,
}

///
/// Type handlers allows us to override certain types that gets passed into functions.
/// Examples are structs such as Rect, Color and Strings
///
trait TypeHandler {
    fn match_type(&self) -> &str;

    fn replace_arg(&self, arg: &Variable) -> String {
        arg.name.clone()
    }

    fn gen_body_return(&self, function_name: &str) -> String {
        format!("    return qt_data->{}()", function_name.to_mixed_case())
    }
}

///
/// Adds some extra functionallity to Struct to make some checks easier
///
impl Struct {
    fn inherits_widget(&self, api_def: &ApiDef) -> bool {
        if self.name == "Widget" {
            return true;
        }

        if let Some(ref inherit_name) = self.inherit {
            if inherit_name == "Widget" {
                return true;
            }

            for sdef in &api_def.class_structs {
                if sdef.inherits_widget(api_def) == true {
                    return true;
                }
            }
        }

        false
    }
}

///
/// Genarate event setup code. It will look something like this
///
///  void Class::paintEvent(QPaintEvent* event) {
///        if (m_paint_event) {
///            RUPainteEvent e;
///            memcpy(&e, s_paint_event, sizeof(e));
///            e.priv_data = event;
///            m_paint_event((RUPainteEvent*)&e, m_paint_user_data);
///        } else {
///            QWidget::paintEvent(event);
///        }
///    }
///
///    RUPaintEventFunc m_paint_event = nullptr;
///    void* m_paint_user_data = nullptr;

/*
fn generate_event_setup_def<W: Write>(f: &mut W, func: &Function) -> io::Result<()> {
    let event_type = &func.function_args[1];

    // Write virtual function def
    f.write_fmt(format_args!(
        "\n    virtual void {}Event(Q{}* event);\n",
        func.name.to_mixed_case(),
        event_type.type_name
    ))?;

    f.write_fmt(format_args!(
        "    void (*m_{})({})",
        func.name,
        generate_c_function_args_signal_wrapper(EventType::Event, func)
    ))?;

    //func.write_c_func_def(f, |_, arg| (arg.get_c_type(), arg.name.to_owned()))?;

    f.write_fmt(format_args!(" = nullptr;\n"))?;
    f.write_fmt(format_args!(
        "    void* m_{}_user_data = nullptr;\n",
        func.name
    ))?;

    Ok(())
}
*/

//
// Generates wrapping code fore "Events" (i.e virtual overrides on Qt objects)
//
/*
fn generate_event_setup_impl<W: Write>(
    f: &mut W,
    sdef: &Struct,
    class_name: &str,
    func: &Function,
    api_def: &ApiDef,
) -> io::Result<()> {
    let event_type = &func.function_args[1];

    f.write_all(SEPARATOR)?;

    f.write_fmt(format_args!(
        "void WR{}::{}Event(Q{}* event) {{\n",
        sdef.name,
        func.name.to_mixed_case(),
        event_type.type_name
    ))?;
    f.write_fmt(format_args!("    if (m_{}) {{\n", func.name))?;
    f.write_fmt(format_args!("        RU{} e;\n", event_type.type_name,))?;
    f.write_fmt(format_args!(
        "        e.{}_event_funcs = &s_{}_event_funcs;\n",
        func.name,
        func.name
    ))?;
    f.write_fmt(format_args!(
        "        e.priv_data = (struct RUBase*)event;\n"
    ))?;

    f.write_fmt(format_args!("        RU{} w;\n", sdef.name))?;

    // TODO: We should do a use a lookup here to find the object for this instead. The reason is
    // that we are tracking the destruction of this object so we can't access bad data on the Rust
    // side. The way we use this here that would still be possible to fail

    for s in api_def.get_inherit_structs(sdef, RecurseIncludeSelf::Yes) {
        let snake_name = s.name.to_snake_case();

        f.write_fmt(format_args!(
            "        w.{}_funcs = &s_{}_funcs;\n",
            snake_name,
            snake_name
        ))?;
    }

    f.write_fmt(format_args!(
        "        w.priv_data = (struct RUBase*)this;\n"
    ))?;

    f.write_fmt(format_args!(
        "        m_{}((struct RUBase*)&w, m_{}_user_data, (struct RUBase*)&e);\n",
        func.name, func.name,
    ))?;
    f.write_fmt(format_args!("    }} else {{\n"))?;
    f.write_fmt(format_args!(
        "        Q{}::{}Event(event);\n",
        class_name,
        func.name.to_mixed_case()
    ))?;
    f.write_fmt(format_args!("    }}\n"))?;
    f.write_fmt(format_args!("}}\n\n"))?;

    Ok(())
}
*/
///
/// Generate something like this
///
/// static void set_paint_event(void* object, void* user_data, void (*event)(void* self_c, struct RUBase* event)) {
///     WRWidget* qt_object = (WRWidget*)object;
///     qt_object->m_paint_user_data = user_data;
///     qt_object->m_paint = event;
/// }
///
/*
fn generate_event_setup_funcs<W: Write>(
    f: &mut W,
    struct_qt_name: &str,
    func: &Function,
) -> io::Result<()> {
    let func_name = format!("{}_{}", struct_qt_name.to_snake_case(), func.name);

    let mut func_def = String::with_capacity(128);

    CapiHeaderGen::callback_fun_def_name(&mut func_def, false, &func_name, func);

    f.write_fmt(format_args!( "{} {{\n", func_def))?;

    f.write_fmt(format_args!(
        "    WR{}* qt_object = (WR{}*)object;\n",
        struct_qt_name, struct_qt_name
    ))?;
    f.write_fmt(format_args!(
        "    qt_object->m_{}_user_data = user_data;\n",
        func.name
    ))?;
    f.write_fmt(format_args!("    qt_object->m_{} = event;\n", func.name))?;
    f.write_all(b"};\n\n")
}
*/

///
/// Generate wrapper classes for all the Widges. This allows us to override
/// virtual functions from the outside (in C and other langs)
/// defs
///
fn generate_wrapper_classes_impl<W: Write>(_f: &mut W, api_def: &ApiDef) -> io::Result<()> {
    for _sdef in api_def
        .class_structs
        .iter()
        .filter(|v| v.inherits_widget(api_def))
    {
        /*
        let iterator = sdef
            .functions
            .iter()
            .filter(|func| func.func_type == FunctionType::Event);
        */

        // Get all the structs that thu current struct inherit from

        /*
        for func in iterator.clone() {
            generate_event_setup_impl(f, &sdef, &struct_qt_name, &func, api_def)?;
        }

        // Generate the setup functions for events
        for func in iterator {
            f.write_all(SEPARATOR)?;
            generate_event_setup_funcs(f, &struct_qt_name, &func)?;
        }
        */
    }

    Ok(())
}

///
/// Generate a string in this style;
///
/// void* self_c, void* wrapped_func, int row
///
pub fn generate_c_function_args_signal_wrapper(event_type: EventType, func: &Function) -> String {
    let mut function_args = String::new();
    let len = func.function_args.len();

    // write arguments
    for (i, arg) in func.function_args.iter().enumerate() {
        if i == 0 {
            /*
            if event_type == EventType::Event {
                function_args.push_str("RUBase*, void*");
            } else {
                function_args.push_str("void* self_c, void* trampoline_func");
            }
            */
        } else {
            // References are passed as pointers to the C code
            match arg.vtype {
                VariableType::Reference => function_args.push_str("struct RUBase*"),
                _ => function_args.push_str(&arg.get_c_type(IsReturnType::No)),
            }

            function_args.push_str(" ");
            function_args.push_str(&arg.name);
        }

        if i != len - 1 {
            function_args.push_str(", ");
        }
    }

    if func.function_args.is_empty() && event_type == EventType::Event {
        function_args.push_str("RUBase*");
    }

    function_args
}

///
/// Generate includes from all the structs which are non-pod
///
fn generate_includes<W: Write>(f: &mut W, api_def: &ApiDef) -> io::Result<()> {
    // There is a small hack here where we include generating includes for the static
    // funcs structs as it doesn't map to a qt class
    for sdef in api_def
        .class_structs
        .iter()
        .filter(|s| s.name != "StaticFuncs")
    {
        f.write_fmt(format_args!("#include <{}>\n", sdef.qt_name))?;
    }

    f.write_all(b"\n")
}

///
/// Generate signal wrapper name.
///
/// Example:
///
/// self_i32_u32_void
///
fn signal_type_callback(func: &Function) -> String {
    let mut name_def = String::with_capacity(128);

    for arg in &func.function_args {
        match arg.vtype {
            VariableType::SelfType => name_def.push_str("self"),
            VariableType::Reference => name_def.push_str(&arg.type_name),
            _ => name_def.push_str(&arg.get_c_type(IsReturnType::No)),
        }

        name_def.push_str("_")
    }

    name_def.push_str("void");
    name_def
}

///
/// In order to figure out what combination of of SignalWrappers we need to generate
/// we go over all the the registered callback function and create a hash for the
/// function arguments. This way we get one unique function wrapper in the cases
/// where several wrapers has the same input
///
fn build_signal_wrappers_info<'a>(api_defs: &'a [ApiDef]) -> HashMap<String, &'a Function> {
    let mut wrapper_info = HashMap::new();
    let mut funcs = Vec::new();

    for api_def in api_defs {
        for t in api_def.get_functions(FunctionType::Event) {
            funcs.push(t);
        }
    }

    funcs.iter().for_each(|func| {
        let input_args = signal_type_callback(func);
        wrapper_info.entry(input_args).or_insert(*func);
    });

    wrapper_info
}

//
// Generate all forward declarations function pointer structs
//
// Example output:
//
// extern struct RUWidgetFuncs s_widget_funcs;
// extern struct RUWidgetAllFuncs s_widget_all_funcs;
//
/*
fn generate_forward_declare_struct_defs<W: Write>(f: &mut W, api_def: &ApiDef) -> io::Result<()> {
    for s in &api_def.class_structs {
        f.write_fmt(format_args!(
            "extern struct RU{}Funcs s_{}_funcs;\n",
            s.name,
            s.name.to_snake_case()
        ))?;
        f.write_fmt(format_args!(
            "extern struct RU{}AllFuncs s_{}_all_funcs;\n",
            s.name,
            s.name.to_snake_case()
        ))?;
    }

    f.write_all(b"\n")
}
*/

fn function_name(struct_name: &str, func: &Function) -> String {
    format!("{}_{}", struct_name.to_snake_case(), func.name)
}
///
/// Generate code for returing array
///
///    int count = ret_value.size();
///     RUArray array = { 0 };
///     if (count > 0) {
///         RUListWidgetItem* elements = new RUListWidgetItem[count];
///         for (int i = 0; i < count; ++i) {
///             elements[i].funcs = &s_list_widget_item_funcs;
///             elements[i].priv_data = (struct RUBase*)data.at(i);
///         }
///         array.elements = (void*)elements;
///         array.count = int(count);
///     }
///     return array;

/*
fn generate_return_array<W: Write>(f: &mut W, ret_val: &Variable) -> io::Result<()> {
    // TODO: Use templates

    let vtype = &ret_val.type_name;

    // Reference in this case means that the data from Qt is by pointer
    f.write_all(b"    int count = ret_value.size();\n")?;
    f.write_all(b"    RUArray array = { 0 };\n")?;
    f.write_all(b"    if (count > 0) {\n")?;
    f.write_fmt(format_args!("        RU{}* elements = new RU{}[count];\n", vtype, vtype))?;
    f.write_all(b"        for (int i = 0; i < count; ++i) {\n")?;
    f.write_fmt(format_args!("            elements[i].funcs = &s_{}_funcs;\n", vtype.to_snake_case()))?;
    if ret_val.vtype == VariableType::Reference {
        f.write_all(b"            elements[i].priv_data = (struct RUBase*)ret_value.at(i);\n")?;
    } else {
        // this is hacky as it leaks memory and needs to be fixed
        f.write_fmt(format_args!("            Q{}* temp = new Q{}(ret_value.at(i));\n", vtype, vtype))?;
        f.write_all(b"            elements[i].priv_data = (struct RUBase*)temp;\n")?;
        //f.write_all(b"            elements[i].priv_data = (struct RUBase*)&ret_value.at(i);\n")?;
    }

    f.write_all(b"       }\n")?;
    f.write_all(b"       array.elements = (void*)elements;\n")?;
    f.write_all(b"       array.count = int(count);\n")?;
    f.write_all(b"   }\n")?;
    f.write_all(b"   return array;\n")?;

    Ok(())
}
*/

///
/// Geerate a declaration with only function types. Used for the SIGNAL/SLOT macros
///

fn generate_func_def_input_parms_only(func: &Function) -> String {
    let mut function_args = String::with_capacity(128);
    let len = func.function_args.len() - 1;

    // write arguments
    for (i, arg) in func.function_args[1..].iter().enumerate() {
        let a = match arg.vtype {
            VariableType::Reference => {
                if arg.pointer {
                    format!("Q{}*", arg.type_name).into()
                } else {
                    format!("*(Q{})*", arg.type_name).into()
                }
            }

            _ => arg.get_c_type(IsReturnType::No),
        };

        function_args.push_str(&a);

        if i != len - 1 {
            function_args.push_str(", ");
        }
    }

    function_args
}

///
/// Generate get functions to be used with for static functions
///
fn generate_static_get_functions<W: Write>(f: &mut W, api_def: &ApiDef) -> io::Result<()> {
    // TODO: Template here
    //
    // Generate create functions for all structs that are flagged with create function
    // and doesn't have manual create selected on them
    //
    for sdef in api_def
        .class_structs
        .iter()
        .filter(|s| s.has_static_functions())
    {
        f.write_all(SEPARATOR)?;
        let snake_name = sdef.name.to_snake_case();

        //
        // Get the name if we have remapped the name (for example we use Button while Qt uses
        // AbstractButton)
        //
        f.write_fmt(format_args!(
            "static struct RU{} get_{}(struct RUBase* priv_data) {{\n",
            sdef.name, snake_name
        ))?;
        f.write_all(b"    (void)priv_data;\n")?;
        f.write_fmt(format_args!("    RU{} ctl;\n", sdef.name))?;
        f.write_all(b"    ctl.qt_data = nullptr;\n")?;
        f.write_all(b"    ctl.host_data = nullptr;\n")?;
        f.write_fmt(format_args!(
            "    ctl.all_funcs = &s_{}_all_funcs;\n",
            snake_name
        ))?;
        f.write_all(b"    return ctl;\n}\n\n")?;
    }

    Ok(())
}

///
/// Generates the create functions
///
fn generate_create_functions<W: Write>(f: &mut W, api_def: &ApiDef) -> io::Result<()> {
    //
    // Generate create functions for all structs that are flagged with create function
    // and doesn't have manual create selected on them
    //
    for sdef in api_def
        .class_structs
        .iter()
        .filter(|s| s.should_have_create_func() && !s.has_manual_create())
    {
        let snake_name = sdef.name.to_snake_case();
        let struct_name = sdef.name.as_str();
        let struct_qt_name = &sdef.cpp_name;
        let is_inherits_widget = sdef.inherits_widget(api_def);
        f.write_all(SEPARATOR)?;

        //
        // We create wrapper widgets for all Qt widges so we can have our own functionallity there
        // So we prefix them with WR (WRapper) otherwise we assume it's a regular Q* class
        //
        let create_func = if is_inherits_widget {
            "create_widget_func"
        } else if sdef.should_gen_wrap_class() {
            "generic_create_func_with_delete"
        } else {
            "generic_create_func"
        };

        //
        // Get the name if we have remapped the name (for example we use Button while Qt uses
        // AbstractButton)
        //

        if sdef.should_gen_wrap_class() {
            f.write_fmt(format_args!(
                "static struct RU{} create_{}(
    struct RUBase* priv_data,
    RUDeleteCallback delete_callback,
    void* private_user_data)
{{\n",
                sdef.name, snake_name,
            ))?;

            f.write_fmt(format_args!(
                "    auto ctl = {}<struct RU{}, {}>(priv_data, delete_callback, private_user_data);\n", create_func,
                struct_name,
                struct_qt_name,
            ))?;
        } else {
            f.write_fmt(format_args!(
                "static struct RU{} create_{}(struct RUBase* priv_data) {{\n",
                sdef.name,
                sdef.name.to_snake_case()
            ))?;

            f.write_fmt(format_args!(
                "    auto ctl = {}<struct RU{}, {}>(priv_data);\n",
                create_func, struct_name, struct_qt_name,
            ))?;
        }

        f.write_fmt(format_args!(
            "    ctl.all_funcs = &s_{}_all_funcs;\n",
            snake_name
        ))?;
        f.write_all(b"    return ctl;\n}\n\n")?;

        f.write_all(SEPARATOR)?;

        //
        // Generate destroy functions as well
        //
        f.write_fmt(format_args!(
            "static void destroy_{}(struct RUBase* priv_data) {{\n",
            sdef.name.to_snake_case()
        ))?;
        f.write_fmt(format_args!(
            "    destroy_generic<{}>(priv_data);\n}}\n\n",
            struct_qt_name
        ))?;
    }

    Ok(())
}

///
/// Generate the function struct defs in the following style
///
/// struct PUMimeDataFuncs s_mime_data_funcs = {
///    mime_data_has_color,
///    mime_data_urls,
///    ...
/// };
///
fn generate_struct_defs<W: Write>(f: &mut W, api_def: &ApiDef) -> io::Result<()> {
    for sdef in &api_def.class_structs {
        f.write_all(SEPARATOR)?;

        let struct_name = &sdef.name;

        f.write_fmt(format_args!(
            "struct RU{}Funcs s_{}_funcs = {{\n",
            struct_name,
            struct_name.to_snake_case()
        ))?;

        if sdef.should_have_create_func() {
            f.write_fmt(format_args!(
                "    destroy_{},\n",
                struct_name.to_snake_case()
            ))?;
        }

        for func in &sdef.functions {
            match func.func_type {
                FunctionType::Regular => {
                    f.write_fmt(format_args!("    {},\n", function_name(struct_name, func)))?;
                }

                FunctionType::Static => {
                    f.write_fmt(format_args!("    {},\n", function_name(struct_name, func)))?;
                }

                FunctionType::Event => {
                    f.write_fmt(format_args!(
                        "    set_{}_event,\n",
                        function_name(struct_name, func)
                    ))?;
                }

                FunctionType::Replace => {
                    f.write_fmt(format_args!(
                        "    set_{}_event,\n",
                        function_name(struct_name, func)
                    ))?;
                }
            }
        }

        f.write_all(b"};\n\n")?;

        f.write_all(SEPARATOR)?;
        // generate the alla funcs

        for sdef in &api_def.class_structs {
            f.write_fmt(format_args!(
                "struct RU{}AllFuncs s_{}_all_funcs = {{\n",
                struct_name,
                struct_name.to_snake_case()
            ))?;

            for name in &sdef.full_inherit {
                f.write_fmt(format_args!("    &s_{}_funcs,\n", name.to_snake_case()))?;
            }

            f.write_all(b"};\n\n")?;
        }
    }

    Ok(())
}

///
/// Handling for Traits
///
/*
struct TraitTypeHandler(String);

impl TypeHandler for TraitTypeHandler {
    fn match_type(&self) -> &str {
        self.0.as_str()
    }

    fn replace_arg(&self, arg: &Variable) -> String {
        // This is a bit hacky but not sure how to solve this right now
        if self.0 == "WidgetType" {
            format!("(QWidget*){}", &arg.name)
        } else if self.0 == "LayoutType" {
            format!("(QLayout*){}", &arg.name)
        } else {
            format!("dynamic_cast<Q{}*>((QObject*){})", self.0, &arg.name)
        }
    }
}
*/

pub struct QtGenerator {
    wrapper_template: Template,
    signal_wrapper_template: Template,
    enum_mapping_template: Template,
    func_def_template: Template,
    set_event_template: Template,
}

impl QtGenerator {
    pub fn new() -> QtGenerator {
        let parser = ParserBuilder::with_liquid().build();

        QtGenerator {
            wrapper_template: parser.parse(QT_GEN_WRAPPER_TEMPLATE).unwrap(),
            signal_wrapper_template: parser.parse(SIGNAL_WRAPPER_TEMPLATE).unwrap(),
            enum_mapping_template: parser.parse(QT_ENUM_MAPPING_TEMPLATE).unwrap(),
            func_def_template: parser.parse(QT_FUNC_DEF_TEMPLATE).unwrap(),
            set_event_template: parser.parse(SET_EVENT_TEMPLATE).unwrap(),
        }
    }
    ///
    /// Generation function def for callbacks/slots
    ///

    fn generate_func_callback<W: Write>(
        &self,
        dest: &mut W,
        struct_name: &str,
        func: &Function,
    ) -> io::Result<()> {
        let signal_type_name = signal_type_callback(&func);
        let func_name = function_name(struct_name, func);

        let mut callback_def = String::with_capacity(128);
        CapiHeaderGen::callback_fun_def_name(&mut callback_def, false, &func_name, func);

        let func_def = generate_func_def_input_parms_only(func);
        let mut object = Object::new();

        object.insert("event_def".to_owned(), Value::str(&callback_def));
        object.insert("signal_type_name".to_owned(), Value::str(&signal_type_name));
        object.insert("qt_signal_name".to_owned(), Value::Str(func.name.to_mixed_case()));
        object.insert("func_def".to_owned(), Value::str(&func_def));

        let res = self.set_event_template.render(&object).unwrap();
        dest.write_all(res.as_bytes())
    }

    ///
    /// Generate function wrappers such as:
    ///
    /// static void widget_resize(struct PUBase* self_c, int width, int height) {
    ///     WRWidget* qt_data = (WRWidget*)self_c;
    ///     qt_data->resize(width, height);
    /// }
    ///
    fn generate_func_def<W: Write>(
        &self,
        f: &mut W,
        sdef: &Struct,
        func: &Function,
        type_handlers: &[Box<TypeHandler>],
    ) -> io::Result<()> {
        // TODO: This isn't fully correct
        let ret_value = func
            .return_val
            .as_ref()
            .map_or("void".into(), |v| v.get_c_type(IsReturnType::Yes));

        let mut object = Object::new();

        // Build function args:
        // TODO: Cleanup

        let func_def = func.gen_c_invoke_filter(FirstArgName::Remove, |_index, arg| {
            if arg.type_name == "String" {
                Some(format!("QString::fromUtf8({})", &arg.name).into())
            } else {
                for handler in type_handlers.iter() {
                    if arg.type_name == handler.match_type() {
                        return Some(handler.replace_arg(&arg).into());
                    }
                }

                match arg.vtype {
                    VariableType::Reference => {
                        let t_name;
                        if arg.type_name.ends_with("Type") {
                            t_name = &arg.type_name[..arg.type_name.len() - 4];
                        } else {
                            t_name = &arg.type_name;
                        }

                        if arg.pointer {
                            Some(format!("(Q{}*){}", t_name, &arg.name).into())
                        } else {
                            Some(format!("*((Q{}*){})", t_name, &arg.name).into())
                        }
                    }

                    VariableType::Enum => {
                        let base_name;

                        if arg.type_name == "Rute" {
                            base_name = "Qt";
                        } else {
                            base_name = &arg.type_name;
                        }

                        println!("{:?}", arg);
                        Some(
                            format!(
                                "({}::{})s_{}_lookup[{}]",
                                base_name,
                                arg.enum_sub_type,
                                arg.enum_sub_type.to_snake_case(),
                                &arg.name
                            ).into(),
                        )
                    }

                    _ => None,
                }
            }
        });

        object.insert(
            "func_name".to_owned(),
            Value::Str(function_name(&sdef.name, func)),
        );
        object.insert(
            "func_def".to_owned(),
            Value::Str(func.generate_c_function_def(FirstArgType::Keep)),
        );
        object.insert(
            "qt_func_name".to_owned(),
            Value::Str(func.cpp_name.to_mixed_case()),
        );
        object.insert("cpp_type_name".to_owned(), Value::str(&sdef.cpp_name));
        object.insert("qt_func_args".to_owned(), Value::Str(func_def));
        object.insert(
            "type_snake_name".to_owned(),
            Value::Str(func.name.to_snake_case()),
        );
        object.insert("c_return_type".to_owned(), Value::str(&ret_value));

        if let Some(ref ret_val) = func.return_val {
            object.insert("array_return".to_owned(), Value::Bool(ret_val.array));
            object.insert("qt_ret_value".to_owned(), Value::str("ret_value"));
            object.insert("funcs_name".to_owned(), Value::str(""));
            object.insert(
                "qt_return_type".to_owned(),
                Value::Str(ret_val.qt_type_name.to_owned()),
            );

            match ret_val.vtype {
                VariableType::Primitive => {
                    object.insert("return_type".to_owned(), Value::str("primitive"))
                }
                VariableType::Str => object.insert("return_type".to_owned(), Value::str("string")),
                VariableType::Regular => {
                    object.insert(
                        "funcs_name".to_owned(),
                        Value::Str(ret_val.type_name.to_snake_case()),
                    );
                    object.insert("return_type".to_owned(), Value::str("regular"))
                }
                VariableType::Reference => {
                    object.insert(
                        "funcs_name".to_owned(),
                        Value::Str(ret_val.type_name.to_snake_case()),
                    );
                    object.insert("return_type".to_owned(), Value::str("reference"))
                }
                _ => object.insert("return_type".to_owned(), Value::str("<illegal>")),
            };
        } else {
            object.insert("array_return".to_owned(), Value::Bool(false));
            object.insert("return_type".to_owned(), Value::str(""));
            object.insert("qt_ret_value".to_owned(), Value::str(""));
        }

        let res = self.func_def_template.render(&object).unwrap();
        f.write_all(res.as_bytes())
    }

    ///
    /// Generate function wrappers such as:
    ///
    /// static void widget_resize(struct PUBase* self_c, int width, int height) {
    ///     WRWidget* qt_data = (WRWidget*)self_c;
    ///     qt_data->resize(width, height);
    /// }
    ///

    fn generate_function_wrappers<W: Write>(
        &self,
        f: &mut W,
        type_handlers: &[Box<TypeHandler>],
        api_def: &ApiDef,
    ) -> io::Result<()> {
        for sdef in &api_def.class_structs {
            //let is_widget = sdef.inherits_widget(api_def);

            for func in sdef.functions.iter().filter(|func| !func.is_manual) {
                f.write_all(SEPARATOR)?;

                match func.func_type {
                    FunctionType::Static => {
                        self.generate_func_def(f, &sdef, func, type_handlers)?
                    }
                    FunctionType::Regular => {
                        self.generate_func_def(f, &sdef, func, type_handlers)?
                    }
                    FunctionType::Event => self.generate_func_callback(f, &sdef.name, func)?,
                    _ => (),
                }
            }
        }

        Ok(())
    }

    /// Generate a signal wrapper that is in the style of this:
    ///
    /// typedef void (*Signal_self_int_void)(void* self_c, void* wrapped_func, int row);
    ///
    /// class QSlotWrapperSignal_self_int_void : public QObject {
    ///     Q_OBJECT
    /// public:
    ///     QSlotWrapperSignal_self_int_void(void* data, Signal_self_int_void trampoline_func, void* wrapped_func) {
    ///         m_tampoline_func = trampoline_func;
    ///         m_data = data;
    ///         m_wrapped_func = wrapped_func;
    ///     }
    ///
    ///     Q_SLOT void method(int row) {
    ///         m_tampoline_func(m_data, m_wrapped_func, row);
    ///     }
    /// private:
    ///     Signal_self_int_void m_tampoline_func;
    ///     void* m_data;
    ///     void* m_wrapped_func;
    /// };
    ///
    pub fn generate_signal_wrappers<W: Write>(
        &self,
        f: &mut W,
        api_defs: &[ApiDef],
    ) -> io::Result<()> {
        // Sort the signals by their names to have stable generation
        let temp = build_signal_wrappers_info(api_defs);
        let ordered: BTreeMap<_, _> = temp.iter().collect();

        for (signal_type_name, func) in ordered {
            let mut template_data = Object::new();

            let c_args = generate_c_function_args_signal_wrapper(EventType::Callback, func);
            //let func_def = func.gen_c_def_filter(Some(None), |_, _| None);
            let c_call_args = func.generate_invoke(FirstArgName::Remove);

            template_data.insert(
                "signal_func_name".to_owned(),
                Value::Str(signal_type_name.clone()),
            );
            template_data.insert("c_args".to_owned(), Value::Str(c_args));

            // we need to add, at the front of the call args as this is being used inside a
            // function call argument already if we have some args
            if c_call_args != "" {
                template_data.insert(
                    "c_call_args".to_owned(),
                    Value::Str(format!(", {}", c_call_args)),
                );
            } else {
                template_data.insert("c_call_args".to_owned(), Value::str(""));
            }

            let res = self.signal_wrapper_template.render(&template_data).unwrap();
            f.write_all(res.as_bytes())?;
        }

        Ok(())
    }

    ///
    /// Generate wrapper classes for all the Widges. This allows us to override
    /// virtual functions from the outside (in C and other langs)
    /// defs and add custom data that we need. This also supports a callback that is called
    /// upon destruction of the object from the C++ side. This allows the callback to tag
    /// the object (on the Rust side) as invalid. This allows for proper error handling if user
    /// tries to access a "deleted" object
    ///
    fn generate_wrapper_classes_defs<W: Write>(
        &self,
        f: &mut W,
        api_def: &ApiDef,
    ) -> io::Result<()> {
        for sdef in api_def
            .class_structs
            .iter()
            .filter(|s| s.should_gen_wrap_class())
        {
            let mut template_data = Object::new();
            let inherits_widget = sdef.inherits_widget(api_def);

            template_data.insert("struct_name".to_owned(), Value::str(&sdef.name));
            template_data.insert("qt_name".to_owned(), Value::str(&sdef.qt_name));
            template_data.insert("widget".to_owned(), Value::Bool(inherits_widget));

            /*
            for func in sdef
                .functions
                .iter()
                .filter(|func| func.func_type == FunctionType::Event)
            {
                self.generate_event_setup_def(f, &func)?;
            }
            */

            // TODO: Fix me
            template_data.insert("events".to_owned(), Value::str(""));

            let res = self.wrapper_template.render(&template_data).unwrap();
            f.write_all(res.as_bytes())?;
        }

        Ok(())
    }

    pub fn generate(&self, target_name: &str, api_def: &ApiDef) -> io::Result<()> {
        let header_path = format!("{}.h", target_name);
        let cpp_path = format!("{}.cpp", target_name);

        // Set up all the type handlers that are being used to do special
        // generation when needed
        let type_handlers: Vec<Box<TypeHandler>> = Vec::new();

        // Create the header and cpp out
        let mut h_out = BufWriter::new(File::create(header_path)?);
        let mut cpp_out = BufWriter::new(File::create(cpp_path)?);

        // Write the header to cpp generated code
        h_out.write_all(b"#pragma once\n")?;
        h_out.write_all(HEADER)?;
        cpp_out.write_all(HEADER)?;

        // Generate includes for all non-POD structs(
        //generate_includes(&mut h_out, &api_def)?;
        generate_includes(&mut cpp_out, &api_def)?;

        //cpp_out.write_all(QT_HEADER)?;

        // Generate all the struct func forward declarations
        //generate_forward_declare_struct_defs(&mut h_out, api_def)?;
        //generate_forward_declare_struct_defs(&mut cpp_out, api_def)?;

        // Generate the wrapping classes declartion is used as for Qt.
        //self.generate_wrapper_classes_defs(&mut h_out, api_def)?;

        // Generate the wrapping implementation that is used as for Qt.
        generate_wrapper_classes_impl(&mut cpp_out, api_def)?;

        // Generate wrapper functions for are regular defined functions
        self.generate_function_wrappers(&mut cpp_out, &type_handlers, api_def)?;

        // generate the create functions for all widgets
        generate_create_functions(&mut cpp_out, api_def)?;

        // generate the static get functions that returns a struct to be used with static functions
        generate_static_get_functions(&mut cpp_out, api_def)?;

        // generate the structs that holds the wrapper functions
        generate_struct_defs(&mut cpp_out, api_def)?;

        // Generate the main Rute struct definition
        //generate_rute_struct(&mut cpp_out, api_def)?;

        // write the end footer of the file that is used for the export
        //cpp_out.write_all(QT_MAIN_FOOTER)

        Ok(())
    }

    ///
    /// Generate all the signal wrappers
    ///
    pub fn generate_all_signal_wrappers(
        &self,
        target_name: &str,
        api_defs: &[ApiDef],
    ) -> io::Result<()> {
        let mut dest = BufWriter::with_capacity(512 * 1024, File::create(target_name)?);

        // Write header first

        dest.write_all(b"#pragma once\n")?;
        dest.write_all(HEADER)?;

        for api_def in api_defs {
            generate_includes(&mut dest, &api_def)?;
        }

        for api_def in api_defs {
            self.generate_wrapper_classes_defs(&mut dest, api_def)?;
        }

        self.generate_signal_wrappers(&mut dest, api_defs)
    }

    ///
    /// Generate enum remappings from rute enums to Qt
    ///
    pub fn generate_enum_mappings(&self, target_name: &str, api_defs: &[ApiDef]) -> io::Result<()> {
        let mut dest = BufWriter::with_capacity(512 * 1024, File::create(target_name)?);
        let mut enums = BTreeMap::new();

        for api_def in api_defs {
            for enum_def in &api_def.enums {
                enums.insert(enum_def.name.clone(), enum_def);
            }
        }

        let mut enum_org_names = BTreeMap::new();

        for (_, enum_def) in &enums {
            enum_org_names.insert(&enum_def.original_class_name, ());
        }

        for (name, _) in enum_org_names {
            writeln!(dest, "#include <{}>", name)?;
        }

        dest.write_all(b"#include <map>\n\n")?;
        dest.write_all(b"struct KeyVal { int val, key; };\n\n")?;

        for (name, _) in &enums {
            dest.write_fmt(format_args!(
                "std::map<int, int> s_{}_lookup;\n",
                name.to_snake_case()
            ))?;
        }

        dest.write_all(b"\n")?;
        dest.write_all(SEPARATOR)?;
        dest.write_all(b"extern void create_enum_mappings() {\n")?;

        for (_, enum_def) in enums {
            let enum_name = enum_def.name.to_snake_case();
            let mut template_data = Object::new();

            template_data.insert("enum_name".to_owned(), Value::Str(enum_name));
            template_data.insert(
                "qt_class".to_owned(),
                Value::str(&enum_def.original_class_name),
            );

            let mut values = Vec::with_capacity(enum_def.entries.len());
            let mut index = 0;

            for entry in &enum_def.entries {
                match *entry {
                    EnumEntry::Enum(ref name) => {
                        let mut enum_data = Object::new();
                        enum_data.insert("name".to_owned(), Value::str(name));
                        enum_data.insert("id".to_owned(), Value::Str(format!("{}", index)));
                        values.push(Value::Object(enum_data));
                        index += 1;
                    }

                    EnumEntry::EnumValue(ref name, ref value) => {
                        let mut enum_data = Object::new();
                        enum_data.insert("name".to_owned(), Value::str(name));
                        enum_data.insert("id".to_owned(), Value::str(value));
                        values.push(Value::Object(enum_data));
                        index = value.parse().unwrap();
                        index += 1;
                    }
                }
            }

            template_data.insert("enums".to_owned(), Value::Array(values));

            let res = self.enum_mapping_template.render(&template_data).unwrap();
            dest.write_all(res.as_bytes())?;
        }

        dest.write_all(b"}\n")
    }

    ///
    /// Generate enum remappings from rute enums to Qt
    ///
    pub fn generate_enum_mappings_header(&self, target_name: &str, api_defs: &[ApiDef]) -> io::Result<()> {
        let mut dest = BufWriter::with_capacity(4 * 1024, File::create(target_name)?);
        let mut enums = BTreeMap::new();

        for api_def in api_defs {
            for enum_def in &api_def.enums {
                enums.insert(enum_def.name.clone(), enum_def);
            }
        }

        let mut enum_org_names = BTreeMap::new();

        for (_, enum_def) in &enums {
            enum_org_names.insert(&enum_def.original_class_name, ());
        }

        dest.write_all(b"#pragma once\n")?;
        dest.write_all(AUTO_GEN_HEADER)?;
        dest.write_all(b"#include <map>\n\n")?;

        for (name, _) in &enums {
            dest.write_fmt(format_args!(
                "extern std::map<int, int> s_{}_lookup;\n",
                name.to_snake_case()
            ))?;
        }

        Ok(())
    }

    ///
    /// Generate the function struct defs in the following style
    ///
    /// struct Rute s_mime_data_funcs = {
    ///    mime_data_has_color,
    ///    mime_data_urls,
    ///    ...
    /// };
    ///
    pub fn generate_rute_struct(filename: &str, api_defs: &[ApiDef]) -> io::Result<()> {
        let mut dest = BufWriter::new(File::create(filename)?);

        writeln!(dest, "{}", QT_MAIN_HEADER)?;

        for sdef in api_defs
            .iter()
            .flat_map(|d| d.class_structs.iter())
            .filter(|s| s.should_have_create_func())
        {
            writeln!(dest, "    create_{},", sdef.name.to_snake_case())?;

            if sdef.has_static_functions() {
                writeln!(dest, "    get_{},", sdef.name.to_snake_case())?;
            }
        }

        writeln!(dest, "{}", QT_MAIN_FOOTER)
    }

    ///
    /// Generate a big bulk file for cpp code for now to allow better code opt and easier to build
    /// We may need to change this if we run into memory issues on machines trying to compile it
    ///
    pub fn generate_bulk_cpp(filename: &str, api_defs: &[ApiDef]) -> io::Result<()> {
        let mut dest = BufWriter::new(File::create(filename).unwrap());

        writeln!(
            &mut dest,
            "// This file is auto-generated by rute_gen. DO NOT EDIT!\n"
        )?;

        let mut files = Vec::new();

        for sdef in api_defs {
            let base_filename = Path::new(&sdef.filename)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap();
            let base_filename = &base_filename[..base_filename.len() - 4];
            // TODO: Fixme
            if base_filename != "qnamespace" {
                files.push(base_filename.to_owned());
            }
        }

        files.sort();

        for f in &files {
            writeln!(dest, "#include \"{}_ffi.h\"", f)?;
            writeln!(dest, "#include \"{}.h\"", f)?;
        }

        writeln!(dest, "#include \"rute_signal_wrappers.h\"")?;
        writeln!(dest, "#include \"qt_enum_mapping.h\"")?;
        writeln!(dest, "")?;

        for f in &files {
            writeln!(dest, "#include \"{}.cpp\"", f)?;
        }

        // include main file as last file
        writeln!(dest, "#include \"qt_enum_mapping.cpp\"")?;
        writeln!(dest, "#include \"qt_rute.cpp\"")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_wrapper_default_sdef() -> (Struct, QtGenerator) {
        let generator = QtGenerator::new();
        let mut sdef = Struct::default();
        sdef.name = "Foo".to_owned();
        sdef.qt_name = "QFoo".to_owned();
        sdef.cpp_name = "WRFoo".to_owned();
        (sdef, generator)
    }

    fn init_default_func() -> Function {
        Function {
            name: "test".to_owned(),
            function_args: vec![Variable {
                name: "self_c".to_owned(),
                vtype: VariableType::SelfType,
                type_name: "struct RUBase".to_owned(),
                ..Variable::default()
            }],
            return_val: None,
            func_type: FunctionType::Regular,
            is_manual: false,
            cpp_name: "test".to_owned(),
        }
    }

    //
    // This will test that the signal_type_callback generates the correct output
    //
    #[test]
    fn test_signal_type_callback_0() {
        let func = Function {
            name: "test".to_owned(),
            function_args: vec![Variable {
                name: "var0".to_owned(),
                vtype: VariableType::Primitive,
                type_name: "i32".to_owned(),
                ..Variable::default()
            }],
            return_val: None,
            func_type: FunctionType::Regular,
            is_manual: false,
            cpp_name: "test".to_owned(),
        };

        let signal_gen = signal_type_callback(&func);
        assert_eq!(signal_gen, "int_void");
    }

    //
    // This will test that the signal_type_callback generates the correct output
    //
    #[test]
    fn test_signal_type_callback_1() {
        let func = Function {
            name: "test".to_owned(),
            function_args: vec![Variable {
                name: "var0".to_owned(),
                vtype: VariableType::Reference,
                type_name: "DropEvent".to_owned(),
                ..Variable::default()
            }],
            return_val: None,
            func_type: FunctionType::Regular,
            is_manual: false,
            cpp_name: "test".to_owned(),
        };

        let signal_gen = signal_type_callback(&func);
        assert_eq!(signal_gen, "DropEvent_void");
    }

    //
    // test and validate function generation
    //
    #[test]
    fn test_qt_wrap_func_gen_no_return() {
        let (sdef, gen) = init_wrapper_default_sdef();
        let func = init_default_func();
        let mut dest = Vec::new();

        gen.generate_func_def(&mut dest, &sdef, &func, &[]).unwrap();
        assert_eq!(
            String::from_utf8(dest).unwrap(),
            "static void foo_test(struct RUBase* self_c) {
    WRFoo* qt_value = (WRFoo*)self_c;

    qt_value->test();
}

"
        );
    }

    //
    // Test returing a 32-bit value
    //
    #[test]
    fn test_qt_wrap_func_i32_return() {
        let (sdef, gen) = init_wrapper_default_sdef();
        let mut func = init_default_func();
        let mut dest = Vec::new();

        func.return_val = Some(Variable {
            name: "ret".to_owned(),
            vtype: VariableType::Primitive,
            type_name: "i32".to_owned(),
            ..Variable::default()
        });

        gen.generate_func_def(&mut dest, &sdef, &func, &[]).unwrap();
        assert_eq!(
            String::from_utf8(dest).unwrap(),
            "static int foo_test(struct RUBase* self_c) {
    WRFoo* qt_value = (WRFoo*)self_c;

    auto ret_value = qt_value->test();
    return ret_value;
}

"
        );
    }

    //
    // Test returing a string value
    //
    #[test]
    fn test_qt_wrap_func_string_return() {
        let (sdef, gen) = init_wrapper_default_sdef();
        let mut func = init_default_func();
        let mut dest = Vec::new();

        func.return_val = Some(Variable {
            name: String::new(),
            vtype: VariableType::Str,
            type_name: String::new(),
            ..Variable::default()
        });

        gen.generate_func_def(&mut dest, &sdef, &func, &[]).unwrap();
        assert_eq!(
            String::from_utf8(dest).unwrap(),
            "static const char* foo_test(struct RUBase* self_c) {
    WRFoo* qt_value = (WRFoo*)self_c;

    auto ret_value = qt_value->test();
    return q_string_to_const_char(ret_value);
}

"
        );
    }
}
