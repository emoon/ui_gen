extern crate argparse;
extern crate heck;
extern crate liquid;
extern crate pest;
extern crate rayon;
extern crate walkdir;

#[macro_use]
extern crate pest_derive;

// Trait for Header and FFI generation
mod header_ffi_gen;

// Code for parsing the API def
mod api_parser;

// Code for FFI generation
mod rust_ffi_gen;

// Code for Rust generation
mod rust_gen;
mod rust_gen_templates;

// Code for C generation (comman helpers for C style code)
mod c_helper;

// Code for Qt C++ generation
mod qt_gen;
mod qt_gen_templates;

// Code for C/Header generation
mod c_gen;

use api_parser::{ApiDef, ApiParser};
use c_gen::CapiHeaderGen;
use header_ffi_gen::HeaderFFIGenerator;
use heck::SnakeCase;
use qt_gen::QtGenerator;
use rayon::prelude::*;
use rust_ffi_gen::RustFFIGenerator;
use rust_gen::RustGenerator;
use std::fs::File;
use std::fs;
use std::io::{BufWriter, Write};
use std::sync::Mutex;
use walkdir::WalkDir;

///
/// Function for creating a directory and just bail in case it already exists.
/// If there is an error here this code will panic as these directories are required in order for
/// this program to work correctly.
///

fn create_dir(path: &str) {
    // dir already existits so just bail
    if let Ok(p) = fs::metadata(path) {
        if p.is_dir() {
            println!("exints {}", path);
            return;
        }
    }

    // This function is expected to succed now when we have checked that the directory exists
    fs::create_dir(path).unwrap();
}

///
/// Generates the Rust mod file to import all FFI and Rust generated code
///
fn generate_auto_mod(filename: &str, api_defs: &[ApiDef]) {
    let mut dest = BufWriter::new(File::create(filename).unwrap());

    writeln!(dest, "// This file is auto-generated by rute_gen. DO NOT EDIT.\n");

    // Collect all then struct names (snake case) and get a sorted list in the end
    let mut names =
        api_defs
        .iter()
        .flat_map(|api_def| api_def.class_structs.iter())
        .map(|s| s.name.to_snake_case())
        .collect::<Vec<String>>();

    // rute auto has some extra stuff used by all the files
    names.push("rute_auto".to_owned());
    names.sort();

    for entry in names {
        writeln!(dest, "pub mod {}_ffi;", entry);
        writeln!(dest, "pub mod {};", entry);
    }
}

///
/// Main
///
fn main() {
    let mut api = ApiDef::default();
    let wd = WalkDir::new("defs");

    // temporary set to one thread during debugging
    rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .build_global()
        .unwrap();

    let rust_dest_dir = "../rute/src/auto";

    // Create the output dirs before doing anything else

    create_dir("../rute/qt_cpp/auto");
    create_dir(rust_dest_dir);

    // Collect all files that needs to be parsed

    let files = wd
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().metadata().unwrap().is_file())
        //.map(|e| e.path().to_str().unwrap().to_owned())
        .collect::<Vec<_>>();

    let api_defs = Mutex::new(Vec::with_capacity(files.len()));

    // Parse the files threaded

    println!("Pass 1: Parsing the definition files...");

    files.par_iter().enumerate().for_each(|(index, f)| {
        let base_filename = f.path().file_name().unwrap().to_str().unwrap();
        let base_filename = &base_filename[..base_filename.len() - 4];
        let mut api_def = ApiDef::default();

        println!("    Parsing file {:?}", f.path());
        ApiParser::parse_file(&f.path(), &mut api_def);

        // Generate Rust FFI
        let rust_ffi_target = format!("{}/{}_ffi.rs", rust_dest_dir, base_filename);
        println!("        Generateing Rust FFI: {}", rust_ffi_target);

        HeaderFFIGenerator::generate(&rust_ffi_target, &api_def, RustFFIGenerator::new()).unwrap();

        // Generate the Rust high-level code
        RustGenerator::new()
            .generate(&format!("{}/{}.rs", rust_dest_dir, base_filename), &api_def)
            .unwrap();

        {
            let mut data = api_defs.lock().unwrap();
            data.push(api_def);
        }
    });

    // This mutex will be forever be locked from here but this part is single threaded anyway
    let mut data = api_defs.lock().unwrap();

    // Generate the main rute.rs file
    RustGenerator::new()
        .generate_rute(&format!("{}/{}.rs", rust_dest_dir, "rute"), &data).unwrap();

    // Generate the mod file for the auto generated Rust code
    generate_auto_mod(&format!("{}/{}.rs", rust_dest_dir, "mod"), &data);


/*

    // Parse all the files in defs
    for entry in walkdir::WalkDir::new("defs") {
        let entry = entry.unwrap();

        // only parse files and skip directories.
        if entry.path().metadata().unwrap().is_file() {
            ApiParser::parse_file(&entry.path(), &mut api);
        }
    }

    // Run a second pass to match up types that may be out of order
    ApiParser::second_pass(&mut api);

    // This holds all the structs,variables,etc
    let api_def = Arc::new(api);

    // TODO: Correct error handling here
    let _ = fs::create_dir("../rute/c_cpp/auto");
    let _ = fs::create_dir("../rute/src/auto");

    // Generate bindings for each backend in threads

    let c_api_def = api_def.clone();
    let c_api_thread = thread::spawn(move || {
        HeaderFFIGenerator::generate(
            "../rute/c_cpp/auto/Rute.h",
            &c_api_def,
            CapiHeaderGen::new(),
        ).unwrap();
    });

    let ffi_api_def = api_def.clone();
    let ffi_api_thread = thread::spawn(move || {
        HeaderFFIGenerator::generate(
            "../rute/src/auto/rute_auto_ffi.rs",
            &ffi_api_def,
            RustFFIGenerator::new(),
        ).unwrap();
    });

    let cpp_api_def = api_def.clone();
    let cpp_api_thread = thread::spawn(move || {
        let qt_gen = QtGenerator::new();
        qt_gen
            .generate("../rute/c_cpp/auto/rute_qt", &cpp_api_def)
            .unwrap();
    });

    let rust_api_def = api_def.clone();
    let rust_gen = RustGenerator::new(&rust_api_def);
    rust_gen
        .generate("../rute/src/auto/rute_auto.rs", &rust_api_def)
        .unwrap();

    // wait for all of them to finish

    c_api_thread.join().unwrap();
    ffi_api_thread.join().unwrap();
    cpp_api_thread.join().unwrap();
    */
}
