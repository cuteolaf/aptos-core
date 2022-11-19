// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use framework::{BuildOptions, BuiltPackage};
use move_binary_format::CompiledModule;
use std::io::Write;


// Update `raw_module_data.rs` in
// `crates/transaction-emitter-lib/src/transaction_generator/publishing/`.
// That file contains `Lazy` static variables for the binary of all the modules in
// `testsuit/smoke-test/src/aptos/module_publishing/` as `Lazy`.
// In `crates/transaction-emitter-lib/src/transaction_generator/publishing` you should
// also find the files that can load, manipulate and use the modules.
// Typically those modules will be altered (publishing at different addresses requires a module
// address rewriting, versioning may benefit from real changes), published and used in transaction.
// Code to conveniently do that should be in that crate.
//
// All of that considered, please be careful when changing this file or the modules in
// `testsuit/smoke-test/src/aptos/module_publishing/` given that it will likely require
// changes in `crates/transaction-emitter-lib/src/transaction_generator/publishing`.
#[ignore]
#[test]
fn publish_for_emitter() {
    // build GenericModule
    let base_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = base_dir.join("src/aptos/module_publishing/");
    let package = BuiltPackage::build(path,BuildOptions::default())
        .expect("building package must succeed");
    let code = package.extract_code();
    let package_metadata = package.extract_metadata().expect("Metadata must exist");
    let metadata = bcs::to_bytes(&package_metadata).expect("Metadata must serialize");

    // this is gotta be the most brittle solution ever!
    // If directory structure changes this breaks.
    // However it is a test that is ignored and runs only with the intent of creating files
    // for the modules compiled, so people can change it as they wish and need to.
    let base_path = base_dir.join(
        "../../crates/transaction-emitter-lib/src/transaction_generator/publishing/"
    );
    let mut generic_mod = std::fs::File::create(&base_path.join("raw_module_data.rs")).unwrap();

    //
    // File header
    //
    writeln!(
        generic_mod,
r#"// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0"#
    ).expect("Writing header comment failed");

    //
    // Module comment
    //
    writeln!(
        generic_mod,
        r#"
// This file was generated. Do not modify!
//
// To update this code, run `cargo test publish_for_emitter -- --ignore`.
// from `testsuite/smoke-test` in aptos core.
// That test compiles the set of modules defined in
// `testsuite/smoke-test/src/aptos/module_publishing/sources/`
// and it writes the binaries here.
// The module name (prefixed with `MODULE_`) is a `Lazy` instance that returns the
// byte array of the module binary.
// This create should also provide a Rust file that allows proper manipulation of each
// module defined below."#
    ).expect("Writing header comment failed");

    //
    // use ... directives
    //
    writeln!(
        generic_mod,
r#"
use once_cell::sync::Lazy;
"#,
    ).expect("Use directive failed");

    //
    // write out package metadata
    //
    // start Lazy declaration
    writeln!(
        generic_mod,
        "pub static PACKAGE_METADATA_SIMPLE: Lazy<Vec<u8>> = Lazy::new(|| {{",
    ).expect("Lazy declaration failed");
    // write package metadata
    writeln!(generic_mod, "\tvec!{:?}", metadata).expect("Lazy declaration failed");
    // close Lazy declaration
    writeln!(generic_mod, "}});\n").expect("Lazy declaration closing } failed");

    //
    // write out all modules
    //
    for module in &code {
        // this is an unfortunate way to find the module name but it is not
        // clear how to do it otherwise
        let compiled_module = CompiledModule::deserialize(module).expect("Module must deserialize");
        let module_name = compiled_module.self_id().name().to_owned().into_string();
        // start Lazy declaration
        writeln!(
            generic_mod,
            "pub static MODULE_{}: Lazy<Vec<u8>> = Lazy::new(|| {{",
            module_name.to_uppercase(),
        ).expect("Lazy declaration failed");
        // write raw module data
        writeln!(generic_mod, "\tvec!{:?}", module).expect("Lazy declaration failed");
        // close Lazy declaration
        writeln!(generic_mod, "}});").expect("Lazy declaration closing } failed");
    }
}

