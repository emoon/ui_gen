use api_parser::*;
use c_helper::*;
use heck::SnakeCase;
use std::fs::File;
///
/// This code is responisble for generating the Rute.h file that allows usage of Rute from C
///
use std::io;
use std::io::BufWriter;
use std::io::Write;

///
/// Header that is generated at the start of the Rute.h file
///
static HEADER: &'static [u8] = b"
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
};\n\n";

///
/// Footer that is generated at the end of the the Rute.h
///
static FOOTER: &'static [u8] = b"
#ifdef __cplusplus
}
#endif\n";

pub struct CapiGenerator;

impl CapiGenerator {
    ///
    /// Generates the C API definition to the output filename
    ///
    pub fn generate(filename: &str, api_def: &ApiDef) -> io::Result<()> {
        let mut f = BufWriter::new(File::create(filename)?);

        f.write_all(HEADER)?;

        //
        // Write function structs forward declarations:
        //
        // struct RUWidgetFuncs;
        // struct RUWidget;
        // struct RUPushBUttonFuncs;
        // struct RUPushBUtton;
        // ...

        for sdef in &api_def.class_structs {
            f.write_fmt(format_args!("struct RU{}Funcs;\n", sdef.name))?;
            f.write_fmt(format_args!("struct RU{};\n", sdef.name))?;
        }

        //
        // Write variable structs forward declarations:
        //
        // struct RUWidget;
        // struct RUPushBUtton;
        // ...

        for sdef in &api_def.pod_structs {
            f.write_fmt(format_args!("struct RU{};\n", sdef.name))?;
        }

        //
        // Write traits structs forward declarations:
        //
        // struct RUWidget;
        // struct RUPushBUtton;
        // ...

        for name in api_def.get_all_traits() {
            f.write_fmt(format_args!("struct RU{};\n", name))?;
        }

        f.write_all(b"\n")?;

        //
        // Write all enums in the stype of
        //
        // typedef RUEnumFoobar {
        //     Value,
        //     Value = 2,
        // } RUEnumFoobar;
        //

        for enum_def in &api_def.enums {
            f.write_fmt(format_args!("typedef enum RU{} {{\n", enum_def.name))?;

            for entry in &enum_def.entries {
                match *entry {
                    EnumEntry::Enum(ref name) => {
                        f.write_fmt(format_args!("    RU{}_{},\n", enum_def.name, name))?
                    }
                    EnumEntry::EnumValue(ref name, ref val) => f.write_fmt(format_args!(
                        "    RU{}_{} = {},\n",
                        enum_def.name, name, val
                    ))?,
                }
            }

            f.write_fmt(format_args!("}} RU{};\n\n", enum_def.name))?;
        }

        //
        // Write structs for POD types like:
        //
        // struct RUSphere {
        //    float x;
        //    float y;
        //    ...
        // };
        //

        for sdef in &api_def.pod_structs {
            f.write_fmt(format_args!("struct RU{} {{\n", sdef.name))?;

            for entry in &sdef.variables {
                f.write_fmt(format_args!(
                    "    {} {};\n",
                    get_c_type(&entry, UseTypeRef::No),
                    entry.name
                ))?;
            }

            f.write_all(b"};\n\n")?;
        }

        //
        // Write struct defs with functions
        //
        // struct RULayoutFuncs {
        //     struct Foo (*resize)(uint32_t x, uint32_t y);
        //     ...
        // };
        //
        // struct RULayout {
        //   struct RUWidgetFuncs* widget_funcs;
        //   struct RULayoutFuncs* layout_funcs;
        //   ...
        // }
        //

        for sdef in &api_def.class_structs {
            f.write_fmt(format_args!("struct RU{}Funcs {{\n", sdef.name))?;

            if sdef.should_have_create_func() {
                f.write_all(b"    void (*destroy)(struct RUBase* self);\n")?;
            }

            for func in &sdef.functions {
                match func.func_type {
                    FunctionType::Regular => Self::generate_func_def(&mut f, func)?,
                    FunctionType::Callback => Self::generate_callback_def(&mut f, func)?,
                    _ => (),
                }
            }

            f.write_all(b"};\n\n")?;
            f.write_fmt(format_args!("struct RU{} {{\n", sdef.name))?;

            for s in api_def.get_inherit_structs(&sdef, RecurseIncludeSelf::Yes) {
                f.write_fmt(format_args!(
                    "    struct RU{}Funcs* {}_funcs;\n",
                    s.name,
                    s.name.to_snake_case()
                ))?;
            }

            f.write_all(b"    struct RUBase* priv_data;\n")?;
            f.write_all(b"};\n\n")?;
        }

        //
        // Write main Rute entry for creation
        //
        // typedef struct Rute {
        //     struct PUWidget (*create_widget)(struct PUBase* self);
        //     ..
        // } Rute;
        //

        f.write_all(b"typedef struct Rute { \n")?;

        for sdef in api_def
            .class_structs
            .iter()
            .filter(|s| s.should_have_create_func())
        {
            f.write_fmt(format_args!(
                "    struct RU{} (*create_{})(struct RUBase* self);\n",
                sdef.name,
                sdef.name.to_snake_case()
            ))?;
        }

        //
        // Generate all the static functions
        //

        for sdef in &api_def.class_structs {
            for func in sdef
                .functions
                .iter()
                .filter(|func| func.func_type == FunctionType::Static)
            {
                Self::generate_func_def(&mut f, func)?;
            }
        }

        f.write_all(b"} Rute;\n\n")?;

        //
        // Generate the defines for easier usage of the API
        //
        //

        Self::generate_define_funcs(&mut f, api_def)?;

        f.write_all(FOOTER)
    }

    ///
    /// Generate function definition in the style of
    ///
    /// struct Foo (*foobar)(uint32_t x, uint32_t)
    ///
    fn generate_func_def<W: Write>(f: &mut W, func: &Function) -> io::Result<()> {
        let ret_value = func
            .return_val
            .as_ref()
            .map_or("void".to_owned(), |r| get_c_type(&r, UseTypeRef::No));

        // write return value and function name
        f.write_fmt(format_args!(
            "    {} (*{})({});\n",
            ret_value,
            func.name,
            generate_c_function_args(func, None)
        ))
    }

    ///
    /// Generate def for connecting events
    ///
    /// TODO: Cleanup this code
    pub fn callback_fun_def_name(def: bool, name: &str, func: &Function) -> String {
        let mut func_def = if def {
            format!(
                "void (*set_{}_event)(void* object, void* user_data, void (*event)(",
                name
            )
        } else {
            format!(
                "void set_{}_event(void* object, void* user_data, void (*event)(",
                name
            )
        };

        let arg_count = func.function_args.len();

        for (i, arg) in func.function_args.iter().enumerate() {
            if i == 0 {
                func_def.push_str("struct RUBase* widget, void*");
            } else {
                func_def.push_str(&get_c_type(&arg, UseTypeRef::No));
            }

            func_def.push_str(" ");
            func_def.push_str(&arg.name);

            if i != arg_count - 1 {
                func_def.push_str(", ");
            }
        }

        if func.function_args.is_empty() {
            func_def.push_str("struct RUBase* widget");
        }

        func_def.push_str("))");
        func_def
    }

    ///
    /// Code to write down callback def
    ///
    fn generate_callback_def<W: Write>(f: &mut W, func: &Function) -> io::Result<()> {
        f.write_fmt(format_args!(
            "    {};\n",
            Self::callback_fun_def_name(true, &func.name, func)
        ))
    }

    ///
    /// Generate defines to make C API usage easier to use. The defines are genereted in the
    /// following format:
    ///
    /// #define PUMenuBar_set_parent(obj, widget) obj.funcs->set_parent(obj.priv_data, widget)
    ///
    fn generate_define_funcs<W: Write>(f: &mut W, api_def: &ApiDef) -> io::Result<()> {
        for sdef in &api_def.class_structs {
            for s in api_def.get_inherit_structs(&sdef, RecurseIncludeSelf::Yes) {
                let funcs_name = format!("{}_funcs", s.name.to_snake_case());
                for func in &s.functions {
                    match func.func_type {
                        FunctionType::Regular => {
                            Self::generate_define_func(f, &sdef.name, &funcs_name, func)?
                        }
                        FunctionType::Callback => {
                            f.write_fmt(
                                format_args!("#define RU{}_set_{}_event(obj, user_data, event) obj.{}->set_{}_event(obj.priv_data, user_data, event)\n",
                                    s.name, func.name, func.name, funcs_name))?;
                        }
                        _ => (),
                    }
                }
            }

            f.write_all(b"\n")?;
        }

        Ok(())
    }

    ///
    /// Generate a define invoke in this style
    ///
    /// #define PUMenuBar_set_parent(obj, widget) obj.funcs->set_parent(obj.priv_data, widget)
    ///
    fn generate_define_func<W: Write>(
        f: &mut W,
        base_name: &str,
        funcs_name: &str,
        func: &Function,
    ) -> io::Result<()> {
        let define_args = generate_c_function_invoke(func, Some("obj"));
        let call_args = generate_c_function_invoke(func, Some("obj.priv_data"));

        f.write_fmt(format_args!(
            "#define RU{}_{}({}) obj.{}->{}({})\n",
            base_name, func.name, define_args, funcs_name, funcs_name, call_args
        ))
    }
}
