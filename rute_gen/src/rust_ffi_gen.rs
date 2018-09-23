use api_parser::*;
use header_ffi_gen::HeaderFFIGen;
use heck::{CamelCase, SnakeCase};
use std::borrow::Cow;
use std::collections::BTreeMap;
///
/// This code is responisble for generating the Rute.h file that allows usage of Rute from C
///
use std::io;
use std::io::Write;

///
/// Header for Rust FFI
///
static MAIN_HEADER: &str = "// This file is auto-generated by rute_gen. DO NOT EDIT.
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
/// Header for Rust FFI
///
static HEADER: &str = "// This file is auto-generated by rute_gen. DO NOT EDIT.
use std::os::raw::c_void;
use rute_ffi_base::*;
\n";

static FOOTER: &str = "
extern \"C\" {
    pub fn rute_get() -> *const RuteFFI;
}\n";

///
/// Add function to convert a type into a Rust FFI type
///
impl Variable {
    pub fn get_rust_ffi_type(&self) -> Cow<str> {
        if self.array {
            return "RUArray".into();
        }

        let name = self.type_name.as_str();

        match &self.vtype {
            VariableType::None => "<illegal type>".into(),
            VariableType::Enum => "i32".into(),
            VariableType::SelfType => "*const RUBase".into(),
            VariableType::Primitive => name.to_owned().into(),
            VariableType::Reference => "*const RUBase".into(),
            VariableType::Regular => format!(" RU{}", name).into(),
            VariableType::Str => "*const ::std::os::raw::c_char".into(),
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
    pub fn rust_func_def<F: Fn(&Variable) -> String>(
        &self,
        include_parens: bool,
        replace_first_arg: Option<&'static str>,
        filter: F,
    ) -> String {
        let arg_count = self.function_args.len();

        let mut res = String::with_capacity(100);

        if include_parens {
            res += "(";
        }

        for (i, arg) in self.function_args.iter().enumerate() {
            let filter_arg = filter(&arg);

            if i == 0 && replace_first_arg.is_some() {
                res += &format!("{}: {}", arg.name, replace_first_arg.unwrap());
            } else {
                res += &format!("{}: {}", arg.name, filter_arg);
            }

            if i != arg_count - 1 {
                res += ", ";
            }
        }

        if include_parens {
            res += ")";
        }

        if let Some(ref ret_var) = self.return_val {
            let filter_arg = filter(&ret_var);
            res += &format!(" -> {}", filter_arg);
        }

        res
    }
}

pub struct RustFFIGenerator {
    _dummy: u32,
}

impl HeaderFFIGen for RustFFIGenerator {
    ///
    /// Generate the header for the file
    ///
    fn gen_header<W: Write>(&mut self, dest: &mut W) -> io::Result<()> {
        write!(dest, "{}", HEADER)
    }

    ///
    /// This will find all the variables and figure out what use
    ///
    fn gen_forward_declaration<W: Write>(&mut self, dest: &mut W, sdef: &Struct) -> io::Result<()> {
        let mut imports = BTreeMap::new();

        for func in &sdef.functions {
            if let Some(ref ret_val) = func.return_val {
                match ret_val.vtype {
                    VariableType::Regular => {
                        imports.insert(&ret_val.type_name, format!("RU{}", ret_val.type_name));
                    }
                    VariableType::Reference => {
                        imports.insert(&ret_val.type_name, format!("RU{}", ret_val.type_name));
                    }

                    _ => (),
                }
            }
        }

        for (module, name) in imports {
            writeln!(dest, "use auto::{}_ffi::{};", module.to_snake_case(), name);
        }

        for name in &sdef.full_inherit {
            if name != &sdef.name {
                writeln!(dest, "use auto::{}_ffi::*;", name.to_snake_case());
            }
        }

        writeln!(dest, "")
    }

    ///
    /// Generate enum
    ///
    fn gen_enum<W: Write>(&mut self, dest: &mut W, enum_def: &Enum) -> io::Result<()> {
        writeln!(dest, "#[repr(C)]")?;
        writeln!(dest, "#[derive(Copy, Clone, Debug)]")?;
        writeln!(dest, "pub enum RU{} {{", enum_def.name)?;

        for entry in &enum_def.entries {
            match *entry {
                EnumEntry::Enum(ref name) => writeln!(dest, "    {},", name.to_camel_case())?,
                EnumEntry::EnumValue(ref name, ref val) => {
                    writeln!(dest, "    {} = {},", name.to_camel_case(), val)?
                }
            }
        }

        writeln!(dest, "}}\n")
    }

    ///
    /// Generate start of struct declaration
    ///
    fn gen_struct_declaration<W: Write>(
        &mut self,
        dest: &mut W,
        struct_name: &str,
    ) -> io::Result<()> {
        writeln!(dest, "#[repr(C)]")?;
        writeln!(dest, "#[derive(Copy, Clone)]")?;
        writeln!(dest, "pub struct {} {{", struct_name)
    }

    ///
    /// Generate end of struct declaration
    ///
    fn gen_struct_end_declaration<W: Write>(
        &mut self,
        dest: &mut W,
        _struct_name: &str,
    ) -> io::Result<()> {
        writeln!(dest, "}}\n")
    }

    ///
    /// Generate destroy function
    ///
    fn gen_destroy_func<W: Write>(&mut self, dest: &mut W, _function_name: &str) -> io::Result<()> {
        writeln!(
            dest,
            "    pub destroy: extern \"C\" fn(self_c: *const RUBase),"
        )
    }

    ///
    /// Generate create function for owned data function
    ///
    fn gen_owned_data_create<W: Write>(
        &mut self,
        dest: &mut W,
        struct_name: &str,
    ) -> io::Result<()> {
        writeln!(
            dest,
            "    pub create_{}: extern \"C\" fn(
        priv_data: *const RUBase,
        callback: unsafe extern \"C\" fn(),
        host_data: *const c_void) -> RU{},",
            struct_name.to_snake_case(),
            struct_name
        )
    }

    ///
    /// Generate create function
    ///
    fn gen_create_gen<W: Write>(
        &mut self,
        dest: &mut W,
        prefix: &str,
        struct_name: &str,
    ) -> io::Result<()> {
        writeln!(
            dest,
            "    pub {}_{}: extern \"C\" fn(priv_data: *const RUBase) -> RU{},\n",
            prefix,
            struct_name.to_snake_case(),
            struct_name
        )
    }
    ///
    /// Generate the funcs declaration
    ///
    fn gen_funcs_declaration<W: Write>(&mut self, dest: &mut W, name: &str, type_name: &str) -> io::Result<()> {
        writeln!(
            dest,
            "    pub {}_funcs: *const RU{}Funcs,",
            name.to_snake_case(),
            type_name
        )
    }

    ///
    /// Generate function
    ///
    fn gen_function<W: Write>(&mut self, dest: &mut W, func: &Function) -> io::Result<()> {
        match func.func_type {
            FunctionType::Regular => Self::generate_function(dest, func),
            FunctionType::Static => Self::generate_function(dest, func),
            FunctionType::Replace => Self::generate_event(dest, func),
            FunctionType::Event => Self::generate_callback(dest, func),
        }
    }

    ///
    /// Generate void data entry
    ///
    fn gen_rubase_ptr_data<W: Write>(&mut self, dest: &mut W, name: &str) -> io::Result<()> {
        writeln!(dest, "    pub {}: *const RUBase,", name)
    }

    ///
    /// Generate forward declarations of needed
    ///
    fn generate_post_declarations<W: Write>(
        &mut self,
        dest: &mut W,
        api_def: &ApiDef,
    ) -> io::Result<()> {
        Ok(())
    }
}

//
// Generator for Rust FFI bindings
//
//
impl RustFFIGenerator {
    ///
    ///
    ///
    pub fn new() -> RustFFIGenerator {
        RustFFIGenerator { _dummy: 0 }
    }

    ///
    /// Generate ffi function
    ///
    fn generate_function<W: Write>(dest: &mut W, func: &Function) -> io::Result<()> {
        let func_def = func.rust_func_def(true, None, |arg| arg.get_rust_ffi_type().into());
        writeln!(dest, "    pub {}: extern \"C\" fn{},", func.name, &func_def)
    }

    ///
    /// Generate event function
    ///
    ///    pub set_test_event: extern "C" fn(object: *const RUBase, user_data: *const c_void,
    ///                                      callback: extern "C" fn(widget: *const RUBase, test:
    ///                                      i32)),
    ///
    fn generate_event<W: Write>(dest: &mut W, func: &Function) -> io::Result<()> {
        let func_def = func.rust_func_def(false, Some("*const c_void"), |arg| {
            arg.get_rust_ffi_type().into()
        });

        writeln!(
            dest,
            "    pub set_{}_event: extern \"C\" fn(object: *const RUBase, user_data: *const c_void,
                                            callback: extern \"C\" fn(widget: *const RUBase, {})),",
            func.name, func_def
        )
    }

    ///
    /// Generate callback function
    ///
    fn generate_callback<W: Write>(dest: &mut W, func: &Function) -> io::Result<()> {
        writeln!(dest,
            "    pub set_{}_event: extern \"C\" fn(object: *const RUBase, user_data: *const c_void, trampoline_func: *const c_void,
                                            callback: *const c_void),\n",
            func.name,
        )
    }
}
