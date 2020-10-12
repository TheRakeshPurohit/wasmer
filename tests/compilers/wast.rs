#![cfg(all(feature = "compiler", feature = "engine"))]

use crate::utils::get_compiler;
use std::path::Path;
use wasmer::{Features, Store};
#[cfg(feature = "jit")]
use wasmer_engine_jit::JIT;
// #[cfg(feature = "native")]
// use wasmer_engine_native::Native;
use wasmer_wast::Wast;

// The generated tests (from build.rs) look like:
// #[cfg(test)]
// mod singlepass {
//     mod spec {
//         #[test]
//         fn address() -> anyhow::Result<()> {
//             crate::run_wast("tests/spectests/address.wast", "singlepass")
//         }
//     }
// }
include!(concat!(env!("OUT_DIR"), "/generated_spectests.rs"));

pub fn run_wast(wast_path: &str, compiler: &str) -> anyhow::Result<()> {
    println!(
        "Running wast `{}` with the {} compiler",
        wast_path, compiler
    );
    let try_nan_canonicalization = wast_path.contains("nan-canonicalization");
    let mut features = Features::default();
    let is_bulkmemory = wast_path.contains("bulk-memory");
    let is_simd = wast_path.contains("simd");
    if is_bulkmemory {
        features.bulk_memory(true);
    }
    if is_simd {
        features.simd(true);
    }
    #[cfg(feature = "test-singlepass")]
    features.multi_value(false);
    let compiler_config = get_compiler(try_nan_canonicalization);
    let store = Store::new(&JIT::new(&compiler_config).features(features).engine());
    // let mut compiler_config = compiler_config;
    // let native = Native::new(&mut compiler_config)
    //     .features(features)
    //     .engine();
    // let store = Store::new(&native);
    let mut wast = Wast::new_with_spectest(store);
    if is_simd {
        // We allow this, so tests can be run properly for `simd_const` test.
        wast.allow_instantiation_failures(&[
            "Validation error: multiple tables: tables count must be at most 1",
            "Validation error: unknown memory 0",
            "Validation error: Invalid var_u32",
        ]);
    }
    if compiler == "singlepass" {
        // We don't support multivalue yet in singlepass
        wast.allow_instantiation_failures(&[
            "Validation error: invalid result arity: func type returns multiple values",
            "Validation error: blocks, loops, and ifs accept no parameters when multi-value is not enabled",
        ]);
    } else if compiler == "cranelift" && cfg!(windows) {
        // Cranelift 0.63 have a bug on multivalue in Windows
        // It's fixed by: https://github.com/bytecodealliance/wasmtime/pull/1774/files
        wast.allow_instantiation_failures(&["Compilation error: Implementation limit exceeded"]);
    }
    wast.fail_fast = false;
    let path = Path::new(wast_path);
    wast.run_file(path)
}
