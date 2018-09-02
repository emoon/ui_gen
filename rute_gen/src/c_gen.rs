use api_parser::*;
use heck::SnakeCase;
///
/// This code is responisble for generating the Rute.h file that allows usage of Rute from C
///
use std::io;
use std::io::Write;
use header_ffi_gen::HeaderFFIGen;

///
/// Header that is generated at the start of the Rute.h file
///
static HEADER: &str = "
// This file is auto-generated by rute_gen. DO NOT EDIT!
#pragma once\n
#include <stdint.h>
#include <stdbool.h>\n
#ifdef __cplusplus
extern \"C\" {
#endif\n
struct RUBase;\n
struct RUArray {
    void* priv_data;
    void* elements;
    uint32_t count;
};\n
typedef void (*RUDeleteCallback)(void* data);\n\n";

///
/// Footer that is generated at the end of the the Rute.h
///
static FOOTER: &str = "
#ifdef __cplusplus
}
#endif\n";

pub struct CapiHeaderGen {
    temp_string: String,
}

impl HeaderFFIGen for CapiHeaderGen {
    ///
    /// Generate the header for the file
    ///
    fn gen_header<W: Write>(&mut self, dest: &mut W) -> io::Result<()> {
        writeln!(dest, "{}", HEADER)
    }

    ///
    /// Generate forward declarations
    ///
    fn gen_forward_declaration<W: Write>(&mut self, dest: &mut W, struct_name: &str) -> io::Result<()> {
        writeln!(dest, "struct RU{}Funcs;", struct_name)?;
        writeln!(dest, "struct RU{};", struct_name)
    }

    ///
    /// Generate enum
    ///
    fn gen_enum<W: Write>(&mut self, dest: &mut W, enum_def: &Enum) -> io::Result<()> {
        writeln!(dest, "typedef enum RU{} {{\n", enum_def.name)?;

        for entry in &enum_def.entries {
            match *entry {
                EnumEntry::Enum(ref name) => {
                    writeln!(dest, "    RU{}_{},", enum_def.name, name)?;
                },

                EnumEntry::EnumValue(ref name, ref val) => {
                    writeln!(dest, "    RU{}_{} = {},", enum_def.name, name, val)?;
                }
            }
        }

        writeln!(dest, "}} RU{};\n", enum_def.name)
    }

    ///
    /// Generate start of struct declaration
    ///
    fn gen_struct_declaration<W: Write>(&mut self, dest: &mut W, struct_name: &str) -> io::Result<()> {
        writeln!(dest, "typedef struct {} {{", struct_name)
    }

    ///
    /// Generate end of struct declaration
    ///
    fn gen_struct_end_declaration<W: Write>(&mut self, dest: &mut W, struct_name: &str) -> io::Result<()> {
        writeln!(dest, "}} {};\n", struct_name)
    }

    ///
    /// Generate destroy function
    ///
    fn gen_destroy_func<W: Write>(&mut self, dest: &mut W, _function_name: &str) -> io::Result<()> {
        writeln!(dest, "    void (*destroy)(struct RUBase* self);")
    }

    ///
    /// Generate create function for owned data function
    ///
    fn gen_owned_data_create<W: Write>(&mut self, dest: &mut W, struct_name: &str) -> io::Result<()> {
        writeln!(dest,
                "    struct RU{} (*create_{})(
        struct RUBase* priv_data,
        RUDeleteCallback delete_callback, void* host_data);",
                struct_name,
                struct_name.to_snake_case())
    }

    ///
    /// Generate create function
    ///
    fn gen_create_gen<W: Write>(&mut self, dest: &mut W, prefix: &str, struct_name: &str) -> io::Result<()> {
        writeln!(dest,
                "    struct RU{} (*{}_{})(struct RUBase* priv_data);",
                struct_name,
                prefix,
                struct_name.to_snake_case())
    }
    ///
    /// Generate the funcs declaration
    ///
    fn gen_funcs_declaration<W: Write>(&mut self, dest: &mut W, name: &str) -> io::Result<()> {
        writeln!(dest, "    struct RU{}Funcs* {}_funcs;\n", name, name.to_snake_case())
    }

    ///
    /// Generate function
    ///
    fn gen_function<W: Write>(&mut self, dest: &mut W, func: &Function) -> io::Result<()> {
        match func.func_type {
            FunctionType::Regular => self.generate_func_def(dest, func)?,
            FunctionType::Static => self.generate_func_def(dest, func)?,
            FunctionType::Callback => {
                self.generate_callback_def(dest, func)?;
                writeln!(dest, ");\n")?;
            }
            _ => (),
        }

        Ok(())
    }

    ///
    /// Generate void data entry
    ///
    fn gen_rubase_ptr_data<W: Write>(&mut self, dest: &mut W, name: &str) -> io::Result<()> {
        writeln!(dest, "    RUBase* {};", name)
    }

    ///
    /// Generate forward declarations of needed
    ///
    fn generate_post_declarations<W: Write>(&mut self, dest: &mut W, _api_def: &ApiDef) -> io::Result<()> {
        write!(dest, "{}", FOOTER)
    }
}

impl CapiHeaderGen {
    pub fn new() -> CapiHeaderGen {
        CapiHeaderGen {
            temp_string: String::with_capacity(1024),
        }
    }

    ///
    /// Generate def for connecting events
    ///
    /// TODO: Cleanup this code
    pub fn callback_fun_def_name(dest: &mut String, def: bool, name: &str, func: &Function) {
        use std::fmt::Write;
        if def {
            write!(dest,
                "void (*set_{}_event)(void* object, void* user_data, void* trampoline_func, void (*event)(",
                name).unwrap()
        } else {
            write!(dest,
                "void set_{}_event(void* object, void* user_data, void* trampoline_func, void (*event)(",
                name).unwrap();
        };

        write!(dest, "{})", func.gen_c_def_filter(Some(Some("void*".into())), |_, _| None)).unwrap()
    }

    ///
    /// Code to write down callback def
    ///
    fn generate_callback_def<W: Write>(&mut self, dest: &mut W, func: &Function) -> io::Result<()> {
        self.temp_string.clear();

        Self::callback_fun_def_name(&mut self.temp_string, true, &func.name, func);
        write!(dest, "    {}", self.temp_string)
    }

    ///
    /// Generate function definition in the style of
    ///
    /// struct Foo (*foobar)(uint32_t x, uint32_t)
    ///
    fn generate_func_def<W: Write>(&mut self, dest: &mut W, func: &Function) -> io::Result<()> {
        let ret_value = func
            .return_val
            .as_ref()
            .map_or("void".into(), |r| r.get_c_type());

        // write return value and function name
        writeln!(dest, "    {} (*{})({});",
            ret_value,
            func.name,
            func.generate_c_function_def(FirstArgType::Keep))
    }
}

