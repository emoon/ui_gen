use pest::iterators::Pair;
use pest::Parser;
use std::borrow::Cow;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::collections::HashMap;

#[cfg(debug_assertions)]
const _GRAMMAR: &str = include_str!("api.pest");

///
/// Current primitive types
///
const PRMITIVE_TYPES: &[&str] = &[
    "i8", "u8", "i16", "u16", "i32", "u32", "i64", "u64", "bool", "f32", "f64",
];

///
/// Attribute tagged on struct of they shouldn't have a create function in the main
/// exported Rute struct
///
static ATTRIB_NO_CREATE: &'static str = "NoCreate";
static ATTRIB_MANUAL_CREATE: &'static str = "ManualCreate";
static ATTRIB_NO_WRAP_CLASS: &'static str = "NoWrapClass";
static ATTRIB_DROP: &'static str = "Drop";

///
/// Variable type
///
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum VariableType {
    None,
    /// Self (aka this pointer in C++ and self in Rust)
    SelfType,
    /// Enum type
    Enum,
    /// Struct/other type
    Regular,
    /// Struct/other type
    Str,
    /// Prmitive type (such as i32,u64,etc)
    Primitive,
    /// Reference type
    Reference,
}

///

///
/// Holds the data for a variable. It's name and it's type
///
#[derive(Debug, Clone)]
pub struct Variable {
    /// Name of the variable
    pub name: String,
    /// Type of the variable
    pub vtype: VariableType,
    /// Name of the variable type
    pub type_name: String,
    /// Rest name of a enum. "test" in the case of Rute::test
    pub enum_sub_type: String,
    /// If variable is an array
    pub array: bool,
    /// If variable is optional (nullable)
    pub optional: bool,
}

///
/// Default implementation for Variable
///
impl Default for Variable {
    fn default() -> Self {
        Variable {
            name: String::new(),
            vtype: VariableType::None,
            type_name: String::new(),
            enum_sub_type: String::new(),
            array: false,
            optional: false,
        }
    }
}

///
/// Function type
///
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FunctionType {
    /// This is a regular function in a Qt Class
    Regular,
    /// Replace functions maps to a virtual override in a Qt Class
    Replace,
    /// Event functions maps to a signal in Qt
    Event,
    /// Static function is that doesn't belong to a class
    Static,
}

///
/// Holds the data for a function. Name, function_args, return_type, etc
///
#[derive(Debug, Clone)]
pub struct Function {
    /// Name of the function
    pub name: String,
    /// Function argumnts
    pub function_args: Vec<Variable>,
    /// Return value
    pub return_val: Option<Variable>,
    /// Type of function. See FunctionType descrition for more info
    pub func_type: FunctionType,
    /// If the function is manually implemented (not auto-generated)
    pub is_manual: bool,
    // If a function supports chaining but shouldn't use it
    //pub no_chain: bool,
}

///
/// Default implementation for Function
///
impl Default for Function {
    fn default() -> Self {
        Function {
            name: String::new(),
            function_args: Vec::new(),
            return_val: None,
            func_type: FunctionType::Regular,
            is_manual: false,
        }
    }
}

///
/// Holds the data for a struct
///
#[derive(Debug, Default)]
pub struct Struct {
    /// Name
    pub name: String,
    /// Name for the C++ class in the generation
    pub cpp_name: String,
    /// Name of the Qt Class/Struct
    pub qt_name: String,
    /// Variables in the struct
    pub variables: Vec<Variable>,
    /// Functions for the struct
    pub functions: Vec<Function>,
    /// Attributes of thu struct
    pub attributes: Vec<String>,
    /// Traits
    pub traits: Vec<String>,
    /// If the struct inherits another
    pub inherit: Option<String>,
    /// The full inherit chain
    pub full_inherit: Vec<String>,
    /// If the struct is a widget or not
    pub is_widget: bool,
    /// If there are other structs that inherits this one
    /// we need to generate a trait implementation for it
    pub should_generate_trait: bool,
}

///
/// C/C++ style enum
///
#[derive(Debug)]
pub enum EnumEntry {
    /// Enum in the style of: EnumEntry,
    Enum(String),
    /// Enum in the style of: EnumEntry = SomeValue,
    EnumValue(String, String),
}

///
/// Maps to a C++ enum
///
#[derive(Debug, Default)]
pub struct Enum {
    /// Name of the enum
    pub name: String,
    /// Original class name (like Qt, QAccesibility)
    pub original_class_name: String,
    /// All the enem entries
    pub entries: Vec<EnumEntry>,
}

///
/// Api definition for a file
///
#[derive(Debug, Default)]
pub struct ApiDef {
    /// full filename path
    pub filename: String,
    /// Base filename (such as foo/file/some_name.def) is some_name
    pub base_filename: String,
    /// Structs that only holds data (such as Rect, NOTE: currently not in use)
    pub pod_structs: Vec<Struct>,
    /// Structs that holds functions (such as QPushButton)
    pub class_structs: Vec<Struct>,
    /// Enums
    pub enums: Vec<Enum>,
}

///
/// Checks if name is a primitive
///
fn is_primitve(name: &str) -> bool {
    PRMITIVE_TYPES.iter().any(|&type_name| type_name == name)
}

#[derive(Parser)]
#[grammar = "api.pest"]
pub struct ApiParser;

///
/// Build struct info for a parsed API def file
///
impl ApiParser {
    pub fn parse_file<P: AsRef<Path>>(path: P, api_def: &mut ApiDef) {
        let mut buffer = String::new();

        let pathname = path.as_ref().to_str().unwrap();

        let mut f = File::open(pathname)
            .unwrap_or_else(|e| panic!("ApiParser: Unable to open {}: {}", pathname, e));
        f.read_to_string(&mut buffer)
            .unwrap_or_else(|e| panic!("ApiParser: Unable to read from {}: {}", pathname, e));

        Self::parse_string(&buffer, &pathname, api_def);
    }

    pub fn parse_string(buffer: &str, filename: &str, api_def: &mut ApiDef) {
        let chunks = ApiParser::parse(Rule::chunk, buffer)
            .unwrap_or_else(|e| panic!("APiParser: {} {}", filename, e));

        let base_filename = Path::new(filename).file_name().unwrap().to_str().unwrap();
        let base_filename = &base_filename[..base_filename.len() - 4];

        api_def.filename = filename.to_owned();
        api_def.base_filename = base_filename.to_owned();

        for chunk in chunks {
            match chunk.as_rule() {
                Rule::structdef => {
                    let sdef = Self::fill_struct(chunk);

                    // If we have some variables in the struct we push it to pod_struct
                    if !sdef.variables.is_empty() {
                        api_def.pod_structs.push(sdef);
                    } else {
                        api_def.class_structs.push(sdef);
                    }
                }

                Rule::enumdef => {
                    let mut enum_def = Enum::default();

                    for entry in chunk.into_inner() {
                        match entry.as_rule() {
                            Rule::name => enum_def.name = entry.as_str().to_owned(),
                            Rule::fieldlist => enum_def.entries = Self::fill_field_list_enum(entry),
                            Rule::org_name => {
                                enum_def.original_class_name = entry
                                    .into_inner()
                                    .next()
                                    .map(|e| e.as_str())
                                    .unwrap()
                                    .to_owned();
                            }
                            _ => (),
                        }
                    }

                    api_def.enums.push(enum_def);
                }

                _ => (),
            }
        }
    }

    ///
    /// Fill struct def
    ///
    fn fill_struct(chunk: Pair<Rule>) -> Struct {
        let mut sdef = Struct::default();

        for entry in chunk.into_inner() {
            match entry.as_rule() {
                Rule::name => sdef.name = entry.as_str().to_owned(),
                Rule::derive => sdef.inherit = Some(Self::get_name(entry)),
                Rule::attributes => sdef.attributes = Self::get_attrbutes(entry),
                Rule::traits => sdef.traits = Self::get_attrbutes(entry),
                Rule::fieldlist => {
                    let (var_entries, func_entries) = Self::fill_field_list(entry);
                    sdef.variables = var_entries;
                    sdef.functions = func_entries;
                }

                _ => (),
            }
        }

        // Set the name we use in C++
        if sdef.should_gen_wrap_class() {
            sdef.cpp_name = format!("WR{}", sdef.name);
        } else if sdef.cpp_name.is_empty() {
            sdef.cpp_name = format!("Q{}", sdef.name);
        }

        // TODO: We need to support the user to override this name if wanted
        // this is to allow supporting generation of code that isn't in the qt
        // strandard library but may be a user class not starting with Q
        if sdef.qt_name.is_empty() {
            sdef.qt_name = format!("Q{}", sdef.name);
        }

        sdef
    }

    ///
    /// Get attributes for a struct
    ///
    fn get_attrbutes(rule: Pair<Rule>) -> Vec<String> {
        let mut attribs = Vec::new();

        for entry in rule.into_inner() {
            if entry.as_rule() == Rule::namelist {
                attribs = Self::get_namelist_list(entry);
            }
        }

        attribs
    }

    ///
    /// collect namelist (array) of strings
    ///
    fn get_namelist_list(rule: Pair<Rule>) -> Vec<String> {
        rule.into_inner().map(|e| e.as_str().to_owned()).collect()
    }

    ///
    /// Fill the entries in a struct
    ///
    /// Returns tuple with two ararys for variables and functions
    ///
    fn fill_field_list(rule: Pair<Rule>) -> (Vec<Variable>, Vec<Function>) {
        let mut var_entries = Vec::new();
        let mut func_entries = Vec::new();

        for entry in rule.into_inner() {
            if entry.as_rule() == Rule::field {
                let field = entry.clone().into_inner().next().unwrap();

                match field.as_rule() {
                    Rule::var => var_entries.push(Self::get_variable(field)),
                    Rule::function => func_entries.push(Self::get_function(field)),
                    _ => (),
                }
            }
        }

        (var_entries, func_entries)
    }

    ///
    /// Get the name for a rule
    ///
    fn get_name(rule: Pair<Rule>) -> String {
        let mut name = String::new();

        for entry in rule.into_inner() {
            if entry.as_rule() == Rule::name {
                name = entry.as_str().to_owned();
                break;
            }
        }

        name
    }

    ///
    /// Get data for function declaration
    ///
    fn get_function(rule: Pair<Rule>) -> Function {
        let mut function = Function::default();

        for entry in rule.into_inner() {
            match entry.as_rule() {
                Rule::name => function.name = entry.as_str().to_owned(),
                Rule::event => function.func_type = FunctionType::Event,
                Rule::replace => function.func_type = FunctionType::Replace,
                Rule::static_typ => function.func_type = FunctionType::Static,
                Rule::varlist => function.function_args = Self::get_variable_list(entry),
                Rule::retexp => function.return_val = Some(Self::get_variable(entry)),
                Rule::manual => {
                    function.is_manual = true;
                    function.func_type = FunctionType::Regular;
                }
                _ => (),
            }
        }

        // if we don't have any function args we add self as first argument as we always have that
        if function.function_args.is_empty() {
            function.function_args.push(Variable {
                name: "self_c".to_owned(),
                vtype: VariableType::SelfType,
                type_name: "self".to_owned(),
                ..Variable::default()
            });
        }

        function
    }

    ///
    /// Gather variable list
    ///
    fn get_variable_list(rule: Pair<Rule>) -> Vec<Variable> {
        let mut variables = Vec::new();

        variables.push(Variable {
            name: "self_c".to_owned(),
            vtype: VariableType::SelfType,
            ..Variable::default()
        });

        for entry in rule.into_inner() {
            variables.push(Self::get_variable(entry));
        }

        variables
    }

    ///
    /// Get variable
    ///
    fn get_variable(rule: Pair<Rule>) -> Variable {
        let mut vtype = Rule::var;
        let mut var = Variable::default();
        let mut type_name = String::new();

        for entry in rule.into_inner() {
            match entry.as_rule() {
                Rule::name => var.name = entry.as_str().to_owned(),
                Rule::refexp => vtype = Rule::refexp,
                Rule::optional => var.optional = true,
                Rule::vtype => type_name = entry.as_str().to_owned(),
                Rule::enum_use => {
                    let name = entry.as_str();
                    var.enum_sub_type = name[2..].to_owned();
                    vtype = Rule::enum_use;
                }

                Rule::array => {
                    // Get the type if we have an array
                    for entry in entry.into_inner() {
                        match entry.as_rule() {
                            Rule::vtype => type_name = entry.as_str().to_owned(),
                            Rule::refexp => vtype = Rule::refexp,
                            _ => (),
                        }
                    }
                    var.array = true;
                }

                _ => (),
            }
        }

        // match up with the correct type
        let var_type = match vtype {
            Rule::refexp => VariableType::Reference,
            Rule::enum_use => VariableType::Enum,
            _ => {
                if type_name == "String" {
                    VariableType::Str
                } else if is_primitve(&type_name) {
                    VariableType::Primitive
                } else {
                    VariableType::Regular
                }
            }
        };

        var.type_name = type_name;
        var.vtype = var_type;
        var
    }

    ///
    /// Get array of enums
    ///
    fn fill_field_list_enum(rule: Pair<Rule>) -> Vec<EnumEntry> {
        let mut entries = Vec::new();

        for entry in rule.into_inner() {
            if entry.as_rule() == Rule::field {
                let field = entry.clone().into_inner().next().unwrap();

                if field.as_rule() == Rule::enum_type {
                    entries.push(Self::get_enum(field));
                }
            }
        }

        entries
    }

    ///
    /// Get enum
    ///
    fn get_enum(rule: Pair<Rule>) -> EnumEntry {
        let mut name = String::new();
        let mut assign = String::new();

        for entry in rule.into_inner() {
            match entry.as_rule() {
                Rule::name => name = entry.as_str().to_owned(),
                Rule::enum_assign => assign = Self::get_enum_assign(entry),
                _ => (),
            }
        }

        if assign.is_empty() {
            EnumEntry::Enum(name)
        } else {
            EnumEntry::EnumValue(name, assign)
        }
    }

    ///
    /// Get enum asign
    ///
    fn get_enum_assign(rule: Pair<Rule>) -> String {
        let mut name_or_num = String::new();

        for entry in rule.into_inner() {
            if entry.as_rule() == Rule::name_or_num {
                name_or_num = entry.as_str().to_owned();
                break;
            }
        }

        name_or_num
    }

    ///
    /// Recursive get the structs
    ///
    fn recursive_get_inherit_structs(
        name: &str,
        include_self: RecurseIncludeSelf,
        lookup: &HashMap<String, Vec<String>>,
        out_structs: &mut Vec<String>)
    {
        if let Some(values) = lookup.get(name) {
            for v in values {
                Self::recursive_get_inherit_structs(
                    v,
                    RecurseIncludeSelf::Yes,
                    lookup,
                    out_structs);
            }
        }

        if include_self == RecurseIncludeSelf::Yes {
            out_structs.push(name.to_owned());
        }
    }

    ///
    /// Get a list of all the traits
    ///
    fn get_inherit_structs(
        name: &str,
        include_self: RecurseIncludeSelf,
        lookup: &HashMap<String, Vec<String>>,
    ) -> Vec<String> {
        let mut out_structs = Vec::new();

        Self::recursive_get_inherit_structs(name, include_self, lookup, &mut out_structs);

        out_structs
    }


    pub fn second_pass(api_defs: &mut [ApiDef]) {
        // Build a hash_set of all classes that are inherited
        let mut inherited_classes = HashMap::new();

        for api_def in api_defs.iter() {
            api_def.class_structs.iter().for_each(|s| {
                s.inherit.as_ref().map_or((), |i| {
                    let mut in_values = Vec::new();
                    in_values.push(i.to_owned());
                    // Using vec here to support multiple classes later
                    inherited_classes.insert(s.name.to_owned(), in_values);
                })
            });
        }

        // Fill all
        for api_def in api_defs.iter_mut() {
            api_def.class_structs.iter_mut().for_each(|s| {
                if s.inherit.is_some() {
                    s.full_inherit = Self::get_inherit_structs(
                        &s.name,
                        RecurseIncludeSelf::Yes,
                        &inherited_classes);
                } else {
                    s.full_inherit = vec![s.name.to_owned()];
                }
            });
        }
    }
}

///
/// Use if self should be included when finding all the structs
///
#[derive(Copy, Clone, PartialEq)]
pub enum RecurseIncludeSelf {
    Yes,
}

///
/// Some helper functions for ApiDef
///
impl ApiDef {
    //
    // Get a list of all the traits
    //
   	/* 
    pub fn get_all_traits<'a>(&'a self) -> Vec<&'a String> {
        // Get all the traits name into a hashset so we can stort them afterwards
        let traits = self
            .class_structs
            .iter()
            .flat_map(|s| s.traits.iter())
            .map(|t| t)
            .collect::<HashSet<&'a String>>();

        let mut sorted_traits = Vec::with_capacity(traits.len());
        let mut sorted_list = traits.iter().collect::<Vec<&&'a String>>();

        sorted_list.sort();
        //sorted_list.collect()
        //sorted_list.collect();

        for entry in sorted_list {
            sorted_traits.push(*entry);
        }

        sorted_traits
    }
    */

    //
    // Recursive get the structs
    //
	/*
	fn recursive_get_inherit_structs<'a>(
        sdef: &'a Struct,
        api_def: &'a ApiDef,
        include_self: RecurseIncludeSelf,
        out_structs: &mut Vec<&'a Struct>,
    ) {
        sdef.inherit.as_ref().map(|name| {
            api_def
                .class_structs
                .iter()
                .find(|s| s.name == *name)
                .map(|s| {
                    Self::recursive_get_inherit_structs(
                        s,
                        api_def,
                        RecurseIncludeSelf::Yes,
                        out_structs,
                    );
                })
        });

        if include_self == RecurseIncludeSelf::Yes {
            out_structs.push(sdef);
        }
    }
    */

    //
    // Get a list of all the traits
    //
	/*
	pub fn get_inherit_structs<'a>(
        &'a self,
        sdef: &'a Struct,
        include_self: RecurseIncludeSelf,
    ) -> Vec<&'a Struct> {
        let mut out_structs = Vec::new();

        Self::recursive_get_inherit_structs(sdef, self, include_self, &mut out_structs);

        out_structs
    }
    */

    ///
    /// Get functions from all structs that matches the filter
    ///
    pub fn get_functions<'a>(&'a self, func_type: FunctionType) -> Vec<&'a Function> {
        self.class_structs
            .iter()
            .flat_map(|s| s.functions.iter())
            .filter(|f| f.func_type == func_type)
            .collect()
    }
}

///
/// Impl for struct. Mostly helper functions to make it easier to extract info
///
impl Struct {
    ///
    /// Check if the struct should has static functions
    ///
    pub fn has_static_functions(&self) -> bool {
        self.functions
            .iter()
            .any(|ref f| f.func_type == FunctionType::Static)
    }
    //
    // Check if the struct should has static functions
    //
    /*
    pub fn has_regular_functions(&self) -> bool {
        self.functions
            .iter()
            .any(|ref f| f.func_type == FunctionType::Regular)
    }
    */
    ///
    /// Check if the struct should has static functions
    ///
    pub fn has_event_replace_functions(&self) -> bool {
        self.functions
            .iter()
            .any(|ref f| f.func_type == FunctionType::Replace || f.func_type == FunctionType::Event)
    }
    ///
    /// Check if the struct should have a create function
    ///
    pub fn should_have_create_func(&self) -> bool {
        self.attributes
            .iter()
            .find(|&s| s == ATTRIB_NO_CREATE)
            .is_none()
    }

    ///
    /// Check if the struct is manually created
    ///
    pub fn has_manual_create(&self) -> bool {
        self.attributes
            .iter()
            .any(|ref s| *s == ATTRIB_MANUAL_CREATE)
    }

    ///
    /// Check if no wrapping class should be generated
    ///
    pub fn should_gen_wrap_class(&self) -> bool {
        self.attributes
            .iter()
            .find(|&s| s == ATTRIB_NO_WRAP_CLASS)
            .is_none()
    }

    ///
    /// Check if no wrapping class should be generated
    ///
    pub fn should_generate_drop(&self) -> bool {
        self.attributes.iter().any(|ref s| *s == ATTRIB_DROP)
    }
}

///
/// Impl for Variable. Helper functions to make C and Rust generation easier
///
impl Variable {
    pub fn get_c_type(&self) -> Cow<str> {
        if self.array {
            return "struct RUArray".into();
        }

        let tname = self.type_name.as_str();

        match self.vtype {
            VariableType::SelfType => "struct RUBase*".into(),
            VariableType::Primitive => match tname {
                "f32" => "float".into(),
                "bool" => "bool".into(),
                "f64" => "double".into(),
                "i32" => "int".into(),
                _ => {
                    if self.type_name.starts_with('u') {
                        format!("uint{}_t", &tname[1..]).into()
                    } else {
                        format!("int{}_t", &tname[1..]).into()
                    }
                }
            },

            VariableType::Reference => "struct RUBase*".into(),

            VariableType::Regular => {
                if tname == "String" {
                    "const char*".into()
                } else {
                    format!("struct RU{}", tname).into()
                }
            }

            VariableType::Enum => "int".into(),
            VariableType::Str => "const char*".into(),

            _ => {
                println!("Should not be here {}", self.name);
                "<error>".into()
            }
        }
    }
}

///
/// This is used to replace the type of the first argument (self)
///
pub enum FirstArgType {
    /// The agument as is
    Keep,
    // Remove the argument
    //Remove,
    // Replace the argument type with this
    //Replace(&'static str),
}

///
/// This is used to replace the name of the first argument (self)
///
pub enum FirstArgName {
    // Keep the name
    //Keep,
    /// Remove the name
    Remove,
    // Replace the name with this
    //Replace(&'static str),
}

///
/// Impl for Function. Helper functions to make C and Rust generation easier
///
impl Function {
    ///
    /// Takes a function definition and generates a C function def from it
    ///
    /// For example: "float test, uint32_t bar"
    ///
    pub fn generate_c_function_def(&self, replace_first: FirstArgType) -> String {
        let mut function_args = String::with_capacity(128);
        let len = self.function_args.len();

        // write arguments
        for (i, arg) in self.function_args.iter().enumerate() {
            if i == 0 {
                match replace_first {
                    FirstArgType::Keep => function_args.push_str(&arg.get_c_type()),
                    //FirstArgType::Remove => continue,
                    //FirstArgType::Replace(ref arg) => function_args.push_str(&arg),
                }
            } else {
                function_args.push_str(&arg.get_c_type());
            }

            function_args.push_str(" ");
            function_args.push_str(&arg.name);

            if i != len - 1 {
                function_args.push_str(", ");
            }
        }

        function_args
    }

    ///
    /// Takes a function definition and generates a C function def from it
    ///
    /// For example: "self, test, bar"
    ///
    pub fn generate_invoke(&self, replace_first_arg: FirstArgName) -> String {
        let mut function_invoke = String::with_capacity(128);
        let len = self.function_args.len();

        // write arguments
        for (i, arg) in self.function_args.iter().enumerate() {
            if i == 0 {
                match replace_first_arg {
                    //FirstArgName::Keep => function_invoke.push_str(&arg.name),
                    FirstArgName::Remove => continue,
                    //FirstArgName::Replace(ref name) => function_invoke.push_str(name),
                }
            } else {
                function_invoke.push_str(&arg.name);
            }

            if i != len - 1 {
                function_invoke.push_str(", ");
            }
        }

        function_invoke
    }

    ///
    /// This function allows to replace any of the types when generating a c function declaration
    /// Type replacement depends highly on where the function is being use.
    ///
    pub fn gen_c_def_filter<F>(&self, replace_first: Option<Option<Cow<str>>>, filter: F) -> String
    where
        F: Fn(usize, &Variable) -> Option<Cow<str>>,
    {
        let mut output = String::with_capacity(256);
        let arg_count = self.function_args.len();
        let mut skip_first = false;

        // This allows us to change the first parameter and it also supports to not have any parameter at all
        replace_first
            .map(|arg| {
                skip_first = true;
                arg
            }).and_then(|v| v)
            .map(|v| {
                if arg_count > 0 {
                    output.push_str(&format!("{} {}", v, self.function_args[0].name));
                }

                if arg_count > 1 {
                    output.push_str(", ");
                }
            });

        // iterater over all the parameters and run the filter

        for (i, arg) in self.function_args.iter().enumerate() {
            if i == 0 && skip_first {
                continue;
            }

            let filter_arg = filter(i, &arg);
            let current_arg = filter_arg.map_or_else(|| arg.get_c_type(), |v| v);

            output.push_str(&format!("{} {}", current_arg, arg.name));

            if i != arg_count - 1 {
                output.push_str(", ");
            }
        }

        output
    }

    ///
    /// This function allows to replace any of the parameter names when generating a c function
    /// definition
    ///
    pub fn gen_c_invoke_filter<F>(&self, replace_first: FirstArgName, filter: F) -> String
    where
        F: Fn(usize, &Variable) -> Option<Cow<str>>,
    {
        let mut output = String::with_capacity(256);
        let arg_count = self.function_args.len();

        // iterater over all the parameters and run the filter

        for (i, arg) in self.function_args.iter().enumerate() {
            if i == 0 {
                match replace_first {
                    //FirstArgName::Keep => (),
                    FirstArgName::Remove => continue,
                    /*
                               FirstArgName::Replace(ref name) => {
                               if arg_count > 0 {
                               output.push_str(name);
                               }

                               if arg_count > 1 {
                               output.push_str(", ");
                               }

                               continue;
                               }
                               */
                }
            }

            let filter_arg = filter(i, &arg);
            let current_arg = filter_arg.map_or_else(|| arg.name.clone().into(), |v| v);

            output.push_str(&format!("{}", current_arg));

            if i != arg_count - 1 {
                output.push_str(", ");
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_primitve_ok() {
        assert_eq!(is_primitve("i32"), true);
    }

    #[test]
    fn test_primitve_false() {
        assert_eq!(is_primitve("dummy"), false);
    }

    ///
    /// Make sure parsing of "struct Widget { show() }"
    ///
    #[test]
    fn test_basic_class_struct() {
        let mut api_def = ApiDef::default();
        ApiParser::parse_string("struct Widget { show() }", "", &mut api_def);
        assert_eq!(api_def.pod_structs.is_empty(), true);
        assert_eq!(api_def.class_structs.is_empty(), false);

        let sdef = &api_def.class_structs[0];

        assert_eq!(sdef.name, "Widget");
        assert_eq!(sdef.functions.len(), 1);
        assert_eq!(sdef.functions[0].name, "show");
    }
}
