use std::io;
use std::fs::File;
use std::io::{Write, BufWriter};
use std::borrow::Cow;
use heck::{SnakeCase, CamelCase};
use api_parser::*;

///
/// Header for Rust FFI
///
static HEADER: &'static [u8] = b"
// This file is auto-generated by rute_gen. DO NOT EDIT.
use std::os::raw::c_void;\n
#[repr(C)]
#[derive(Default, Copy, Clone, Debug)]
pub struct RUBase {
    _unused: [u8; 0],
}\n
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct RUArray {
    pub priv_data: *const c_void,
    pub elements: *const c_void,
    pub count: i32,
}\n\n";

///
/// Add function to convert a type into a Rust FFI type
///
impl Variable {
    pub fn get_rust_ffi_type<'a>(&'a self) -> Cow<'a, str> {
        if self.array {
            return "RUArray".into();
        }

        match &self.vtype {
            VariableType::None => "<illegal type>".into(),
            VariableType::SelfType => "*const RUBase".into(),
            VariableType::Primitive(ref name) => name.to_owned().into(),
            VariableType::Reference(_) => "*const RUBase".into(),
            VariableType::Optional(name) => format!(" RU{}", name).into(),
            VariableType::Enum(ref name) => format!(" RU{}", name).into(),
            VariableType::Regular(ref name) => {
                if name == "String" {
                    "*const ::std::os::raw::c_char".into()
                } else {
                    format!(" RU{}", name).into()
                }
            }
        }
    }
}

///
/// These are some helper functions to make generation of Rust style functions
///
impl Function {
    ///
    /// Generate a function defition in the style of
    ///
    /// (test: i32, foo: u32) -> u32
    ///
    pub fn rust_func_def<F: Fn(&Variable) -> String>(&self,
        replace_first_arg: Option<&'static str>,
        filter: F) -> String
    {
        let arg_count = self.function_args.len();

        let mut res = String::with_capacity(100);

        for (i, arg) in self.function_args.iter().enumerate() {
            let filter_arg = filter(&arg);

            if i == 0 && replace_first_arg.is_some() {
                res += &format!("{}: {}", replace_first_arg.unwrap(), filter_arg);
            } else {
                res += &format!("{}: {}", arg.name, filter_arg);
            }

            if i != arg_count - 1 {
                res += ", ";
            }
        }

        if let Some(ref ret_var) = self.return_val {
            let filter_arg = filter(&ret_var);
            res += &format!(" -> {}", filter_arg);
        }

        res
    }
}


pub struct RustFFIGenerator;

///
/// Generator for Rust FFI bindings
///
///
impl RustFFIGenerator {
    ///
    /// Entry point for FFI generation
    ///
    pub fn generate(filename: &str, api_def: &ApiDef) -> io::Result<()> {
        let mut f = BufWriter::new(File::create(filename)?);

        f.write_all(HEADER)?;

        //
        // Write the trait forward structs
        //

        for trait_name in api_def.get_all_traits() {
            f.write_all(b"#[repr(C)]\n")?;
            f.write_all(b"#[derive(Default, Copy, Clone, Debug)]\n")?;
            f.write_fmt(format_args!("pub struct RU{} {{\n", trait_name))?;
            f.write_all(b"    _unused: [u8; 0],\n")?;
            f.write_all(b"}\n\n")?;
        }

        //
        // Generate all enums like:
        //
        // ...
        // pub enum RUKeys {
        //     KeyEscape,
        //     KeySpace
        //     ..
        // }

        for enum_def in &api_def.enums {
            f.write_all(b"#[repr(C)]\n")?;
            f.write_all(b"#[derive(Copy, Clone, Debug)]\n")?;
            f.write_fmt(format_args!("pub enum RU{} {{\n", enum_def.name))?;

            for entry in &enum_def.entries {
                match *entry {
                    EnumEntry::Enum(ref name) => f.write_fmt(format_args!("    {},\n", name.to_camel_case()))?,
                    EnumEntry::EnumValue(ref name, ref val) => f.write_fmt(format_args!("    {} = {},\n", name.to_camel_case(), val))?,
                }
            }

            f.write_all(b"}\n\n")?;
        }

        //
        // Generate all pod structs like
        //
        // struct Foo {
        //   pub x: f32,
        //   ...
        // }
        //

        for sdef in &api_def.pod_structs {
            f.write_all(b"#[repr(C)]\n")?;
            f.write_all(b"#[derive(Default, Copy, Clone, Debug)]")?;
            f.write_fmt(format_args!("pub struct RU{} {{\n", sdef.name))?;

            for var in &sdef.variables {
                f.write_fmt(format_args!(
                    "    pub {}: {},\n",
                    var.name,
                    var.get_rust_ffi_type()
                ))?;
            }
        }

        f.write_all(b"}\n\n")?;

        //
        // Generate all the class defs
        //

        for sdef in &api_def.class_structs {
            f.write_all(b"#[repr(C)]\n")?;
            f.write_fmt(format_args!("pub struct RU{}Funcs {{\n", sdef.name))?;

            for func in &sdef.functions {
                match func.func_type {
                    FunctionType::Regular => Self::generate_function(&mut f, &func)?,
                    FunctionType::Callback => Self::generate_callback(&mut f, &func)?,
                    FunctionType::Event => Self::generate_event(&mut f, &func)?,
                    _ => (),
                }
            }

            f.write_all(b"\n\n")?;
            f.write_fmt(format_args!("struct RU{} {{\n", sdef.name))?;

            for s in api_def.get_inherit_structs(&sdef, RecurseIncludeSelf::Yes) {
                f.write_fmt(format_args!("    pub {}_funcs: *const RU{}Funcs,\n", s.name.to_snake_case(), s.name))?;
            }

            f.write_all(b"    pub privd: *const RUBase,\n")?;
            f.write_all(b"}\n\n")?;
        }

        //
        // Generate main etry
        //

        Ok(())
    }

    ///
    /// Generate ffi function
    ///
    fn generate_function< W: Write>(f: &mut W, func: &Function) -> io::Result<()> {
        let func_def = func.rust_func_def(None, |arg| arg.get_rust_ffi_type().into());

        f.write_fmt(format_args!("    pub {}: extern \"C\" fn({}),\n", func.name, func_def))
    }

    ///
    /// Generate event function
    ///
    ///    pub set_test_event: extern "C" fn(object: *const RUBase, user_data: *const c_void,
    ///                                      callback: extern "C" fn(widget: *const RUBase, test:
    ///                                      i32)),
    ///
    fn generate_event<W: Write>(f: &mut W, func: &Function) -> io::Result<()> {
        let func_def = func.rust_func_def(Some("*const c_void*"), |arg| arg.get_rust_ffi_type().into());

        f.write_fmt(format_args!(
            "    pub set_{}_event: extern \"C\" fn(object: *const RUBase, user_data: *const c_void,
                                            callback: extern \"C\" fn(widget: *const RUBase, {})),\n", func.name, func_def))
    }

    ///
    /// Generate callback function
    ///
    fn generate_callback<W: Write>(f: &mut W, func: &Function) -> io::Result<()> {
        let func_def = func.rust_func_def(Some("*const c_void*"), |arg| arg.get_rust_ffi_type().into());

        f.write_fmt(format_args!(
            "    pub set_{}_event: extern \"C\" fn(object: *const RUBase, user_data: *const c_void,
                                            callback: extern \"C\" fn({}),\n", func.name, func_def))
    }
}
