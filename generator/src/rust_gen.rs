use std::io;
use std::fs::File;
use std::io::Write;
use std::io::{Error, ErrorKind};
use api_parser::*;
use std::collections::HashMap;
use heck::SnakeCase;

static HEADER: &'static [u8] = b"
// ***************************************************************
// AUTO-GENERATED! DO NOT EDIT!
// ***************************************************************

use ffi_gen::*;
use std::ffi::CString;\n\n";

static UI_HEADER: &'static [u8] = b"pub struct Ui {
    pu: *const PU
}

impl Ui {
    pub fn new(pu: *const PU) -> Ui { Ui { pu: pu } }
\n";

trait TypeHandler {
    fn match_type(&self) -> String;

    fn replace_arg(&self, arg: &Variable) -> (String, String) {
        (arg.name.to_owned(), arg.vtype.to_owned())
    }

    fn gen_body(&self, arg: &str, f: &mut File, index: usize) -> String;
}

///
/// Type handler for traits as the function arguments when using a trait needs to use "get_obj"
///
struct TraitTypeHandler {
    name: String,
}

impl TypeHandler for TraitTypeHandler {
    fn match_type(&self) -> String {
        self.name.clone()
    }

    fn replace_arg(&self, arg: &Variable) -> (String, String) {
        (arg.name.to_owned(), format!("&{}", arg.vtype.to_owned()))
    }

    fn gen_body(&self, arg_name: &str, _f: &mut File, _index: usize) -> String {
        format!("{}.get_obj() as *const PU{}", arg_name, self.name)
    }
}

///
///
///
fn generate_traits(
    f: &mut File,
    type_handlers: &mut Vec<Box<TypeHandler>>,
    api_def: &ApiDef,
) -> io::Result<()> {
    let traits = api_def.get_all_traits();

    for trait_name in traits {
        f.write_fmt(format_args!("pub trait {} {{\n", trait_name))?;
        f.write_all(b"    fn get_obj(&self) -> *const ::std::os::raw::c_void;\n}\n\n")?;

        type_handlers.push(Box::new(TraitTypeHandler {
            name: trait_name.clone(),
        }));
    }

    Ok(())
}

///
///
///
fn generate_struct(f: &mut File, structs: &Vec<Struct>) -> io::Result<()> {
    for sdef in structs {
        if sdef.is_pod() {
            // for pod structs we re-use the FFI implementation
            f.write_fmt(format_args!(
                "pub use ffi_gen::PU{} as {};\n\n",
                sdef.name, sdef.name
            ))?;
        } else {
            f.write_fmt(format_args!("pub struct {} {{\n", sdef.name))?;

            if sdef.is_pod() {
                for entry in &sdef.entries {
                    match *entry {
                        StructEntry::Var(ref var) => {
                            f.write_fmt(format_args!("    pub {}: {},\n", var.name, var.vtype))?
                        }
                        _ => (),
                    }
                }
            } else {
                // Assume for non-pod that we only use the FFI interface to do stuff.
                f.write_fmt(format_args!(
                    "    pub obj: Option<*const PU{}>,\n",
                    sdef.name
                ))?;
            }

            f.write_all(b"}\n\n")?;
        }
    }

    Ok(())
}

fn get_arg(arg: &Variable, type_handlers: &Vec<Box<TypeHandler>>) -> (String, String) {
    for handler in type_handlers.iter() {
        if handler.match_type() == arg.vtype {
            return handler.replace_arg(&arg);
        }
    }

    if arg.vtype == "self" {
        ("&self".to_owned(), "".to_owned())
    } else if arg.primitive {
        (arg.name.to_owned(), arg.vtype.clone())
    } else if arg.reference {
        (arg.name.clone(), format!("&{}", arg.vtype.to_owned()))
    } else {
        (arg.name.clone(), arg.vtype.to_owned())
    }
}

/*
fn get_func_def(index: usize, arg: &Variable, type_handlers: &Vec<Box<TypeHandler>>) -> (String, String) {
    if index == 0 {
        return ("(*self.obj).privd))".to_owned(), "".to_owned());
    }

    if !arg.primitive {
        for handler in type_handlers.iter() {
            if handler.match_type() == arg.vtype {
                return handler.replace_arg(&arg);
            }
        }
    }

    //    (format!("{}", name_remap.get(&arg.name.to_owned()).unwrap()), "".to_owned())


    } else {
        (arg.name.to_owned(), "".to_owned())
    }
}
*/

fn generate_func_impl(
    f: &mut File,
    func: &Function,
    type_handlers: &Vec<Box<TypeHandler>>,
) -> io::Result<()> {
    f.write_fmt(format_args!("    pub fn {}", func.name))?;

    func.write_func_def_full(f, |_, arg| get_arg(arg, type_handlers))?;

    f.write_all(b" {\n")?;

    // Handle strings (as they need to use CString before call down to the C code

    let mut name_remap = HashMap::with_capacity(func.function_args.len());

    for (i, arg) in func.function_args.iter().enumerate() {
        for handler in type_handlers.iter() {
            if arg.vtype == handler.match_type() {
                let arg_name = handler.gen_body(&arg.name, f, i);
                println!("arg_name {}", arg_name);
                name_remap.insert(i, arg_name);
                break;
            }
        }

        name_remap.entry(i).or_insert(arg.name.to_owned());
    }

    f.write_all(b"        unsafe {\n")?;
    f.write_all(b"            let obj = self.obj.unwrap();\n")?;
    f.write_fmt(format_args!("            ((*obj).{})(", func.name))?;

    func.write_func_def(f, |index, arg| {
        if index == 0 {
            ("(*obj).privd".to_owned(), String::new())
        } else if !arg.primitive {
            if let Some(name) = name_remap.get(&index) {
                println!("nameremap {} for index {}", name, index);
                (name.to_owned(), String::new())
            } else {
                (arg.name.to_owned(), String::new())
            }
        } else {
            (arg.name.to_owned(), String::new())
        }
    })?;

    f.write_all(b")\n")?;

    f.write_all(b"        }\n")?;
    f.write_all(b"    }\n\n")?;

    Ok(())
}

fn get_function_args(func: &Function) -> String {
    let mut args = String::new();

    for arg in &func.function_args {
        args.push_str(&arg.vtype);
        args.push_str(", ");
    }

    args
}

///
/// Generate something that looks like this
///
/// macro_rules! set_released_event {
///     ($sender:expr, $data:expr, $call_type:ident, $callback:path) => {
///         {
///             extern "C" fn temp_call(target: *mut std::os::raw::c_void) {
///                 unsafe {
///                     let app = target as *mut $call_type;
///                     $callback(&mut *app);
///                 }
///             }
///
///             unsafe {
///                 let obj = $sender.obj.unwrap();
///                 ((*obj).set_value_changed_event)((*obj).privd, ::std::mem::transmute($data), temp_call);
///             }
///         }
///     }
/// }
fn generate_set_event_impl(
    f: &mut File,
    connect_funcs: &Vec<(&String, &Function)>,
) -> io::Result<()> {
    for funcs in connect_funcs {
        f.write_fmt(format_args!(
            "#[macro_export]\nmacro_rules! set_{}_event {{\n",
            funcs.0
        ))?;
        f.write_all(b"  ($sender:expr, $data:expr, $call_type:ident, $callback:path) => {\n")?;
        f.write_all(b"    {\n")?;
        f.write_all(b"      extern \"C\" fn temp_call(")?;

        funcs
            .1
            .write_func_def(f, |_, arg| (arg.name.to_owned(), arg.get_rust_ffi_type()))?;
        f.write_all(b") {\n")?;
        f.write_all(b"          unsafe {\n")?;
        f.write_all(b"              let app = self_c as *mut $call_type;\n")?;
        f.write_all(b"              $callback(")?;

        funcs.1.write_func_def(f, |index, arg| {
            if index == 0 {
                ("&mut *app".to_owned(), "".to_owned())
            } else {
                (arg.name.to_owned(), "".to_owned())
            }
        })?;

        f.write_all(b");\n")?;
        f.write_all(b"          }\n")?;
        f.write_all(b"      }\n")?;
        f.write_all(b"      unsafe {\n")?;
        f.write_all(b"          let obj = $sender.obj.unwrap();\n")?;
        f.write_fmt(format_args!("         ((*obj).set_{}_event)((*obj).privd, ::std::mem::transmute($data), temp_call);\n", funcs.0))?;
        f.write_all(b"      }\n")?;
        f.write_all(b"    }\n")?;
        f.write_all(b"}}\n\n")?;
    }

    Ok(())
}

///
/// This code assumes that the connection name has the same number of args
///
fn generate_set_event(f: &mut File, api_def: &ApiDef) -> io::Result<()> {
    let mut event_names: HashMap<String, Function> = HashMap::new();

    for sdef in api_def.entries.iter().filter(|s| !s.is_pod()) {
        let funcs = api_def.collect_callback_functions(&sdef);

        for func in funcs
            .iter()
            .filter(|s| s.func_type == FunctionType::Callback)
        {
            let args = get_function_args(&func);
            let mut found = true;

            if let Some(ref f) = event_names.get(&func.name) {
                let current_args = get_function_args(&f);
                if &current_args != &args {
                    println!(
                        "Signal: {} - has versions with diffrent args {} - {}",
                        func.name, current_args, args
                    );
                    return Err(Error::new(ErrorKind::Other, "Fail"));
                }
            } else {
                found = false;
            }

            if !found {
                event_names.insert(func.name.clone(), func.clone());
            }
        }
    }

    let mut event_list = event_names.iter().collect::<Vec<(&String, &Function)>>();
    event_list.sort_by(|a, b| a.0.cmp(b.0));

    // println!("{:?}", event_list);

    generate_set_event_impl(f, &event_list)
}

fn generate_impl(
    f: &mut File,
    api_def: &ApiDef,
    type_handlers: &Vec<Box<TypeHandler>>,
) -> io::Result<()> {
    for sdef in api_def.entries.iter().filter(|s| !s.is_pod()) {
        f.write_fmt(format_args!("impl {} {{\n", sdef.name))?;

        for func in api_def.collect_regular_functions(&sdef) {
            generate_func_impl(f, &func, type_handlers)?;
        }

        f.write_all(b"}\n\n")?;

        // If we have a create function we implement drop on this also

        if !sdef.is_pod() && sdef.should_have_create_func() {
            f.write_fmt(format_args!("impl Drop for {} {{\n", sdef.name))?;
            f.write_all(b"    fn drop(&mut self) {\n")?;
            f.write_all(b"       unsafe {\n")?;
            f.write_all(b"          let obj = self.obj.unwrap();\n")?;
            f.write_all(b"          ((*obj).destroy)(obj as *const ::std::os::raw::c_void);\n")?;
            f.write_all(b"          self.obj = None;\n")?;
            f.write_all(b"       }\n")?;
            f.write_all(b"    }\n")?;
            f.write_all(b"}\n\n")?;
        }

        for trait_name in api_def.get_traits(&sdef) {
            f.write_fmt(format_args!("impl {} for {} {{\n", trait_name, sdef.name))?;
            f.write_all(b"    fn get_obj(&self) -> *const ::std::os::raw::c_void {\n")?;
            f.write_all(b"       unsafe {\n")?;
            f.write_all(b"           let obj = self.obj.unwrap();\n")?;
            f.write_all(b"           (*obj).privd as *const ::std::os::raw::c_void\n")?;
            f.write_all(b"       }\n")?;
            f.write_all(b"    }\n")?;
            f.write_all(b"}\n\n")?;
        }
    }

    Ok(())
}

fn generate_ui_impl(f: &mut File, api_def: &ApiDef) -> io::Result<()> {
    f.write_all(UI_HEADER)?;

    for sdef in api_def
        .entries
        .iter()
        .filter(|s| !s.is_pod() && s.should_have_create_func())
    {
        let snake_name = sdef.name.to_snake_case();

        f.write_fmt(format_args!(
            "    pub fn create_{}(&self) -> {} {{\n",
            snake_name, sdef.name
        ))?;
        f.write_fmt(format_args!(
            "        {} {{ obj: Some(unsafe {{ ((*self.pu).create_{})((*self.pu).privd) }}) }}\n",
            sdef.name, snake_name
        ))?;
        f.write_all(b"    }\n\n")?;
    }

    f.write_all(b"}\n")?;

    Ok(())
}

struct StringTypeHandler;

///
/// We need to handle strings in a special way. They need to be sent down using CString and the
/// pointer to it so have a generator for it
///
impl TypeHandler for StringTypeHandler {
    fn match_type(&self) -> String {
        "String".to_owned()
    }

    fn replace_arg(&self, arg: &Variable) -> (String, String) {
        (arg.name.to_owned(), "&str".to_owned())
    }

    fn gen_body(&self, arg: &str, f: &mut File, index: usize) -> String {
        let arg_name = format!("str_in_{}_{}", arg, index);
        f.write_fmt(format_args!(
            "        let {} = CString::new({}).unwrap();\n",
            arg_name, arg
        )).unwrap();
        format!("{}.as_ptr()", arg_name)
    }
}

pub fn generate_rust_bindings(filename: &str, api_def: &ApiDef) -> io::Result<()> {
    let mut f = File::create(filename)?;
    let mut type_handlers: Vec<Box<TypeHandler>> = Vec::new();

    type_handlers.push(Box::new(StringTypeHandler {}));

    f.write_all(HEADER)?;

    generate_struct(&mut f, &api_def.entries)?;
    generate_traits(&mut f, &mut type_handlers, api_def)?;
    generate_impl(&mut f, &api_def, &type_handlers)?;
    generate_set_event(&mut f, &api_def)?;
    generate_ui_impl(&mut f, &api_def)?;

    Ok(())
}
