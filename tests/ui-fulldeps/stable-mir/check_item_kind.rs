// run-pass
//! Test that item kind works as expected.

// ignore-stage1
// ignore-cross-compile
// ignore-remote
// ignore-windows-gnu mingw has troubles with linking https://github.com/rust-lang/rust/pull/116837
// edition: 2021

#![feature(rustc_private)]
#![feature(assert_matches)]
#![feature(control_flow_enum)]

extern crate rustc_middle;
#[macro_use]
extern crate rustc_smir;
extern crate rustc_driver;
extern crate rustc_interface;
extern crate stable_mir;

use rustc_middle::ty::TyCtxt;
use rustc_smir::rustc_internal;
use stable_mir::*;
use std::io::Write;
use std::ops::ControlFlow;

const CRATE_NAME: &str = "input";

/// This function uses the Stable MIR APIs to get information about the test crate.
fn test_item_kind(_tcx: TyCtxt<'_>) -> ControlFlow<()> {
    let items = stable_mir::all_local_items();
    assert_eq!(items.len(), 3);
    // Constructor item.
    for item in items {
        let expected_kind = match item.name().as_str() {
            "Dummy" => ItemKind::Fn,
            "dummy" => ItemKind::Fn,
            "DUMMY_CONST" => ItemKind::Const,
            name => unreachable!("Unexpected item {name}"),
        };
        assert_eq!(item.kind(), expected_kind, "Mismatched type for {}", item.name());
    }
    ControlFlow::Continue(())
}

/// This test will generate and analyze a dummy crate using the stable mir.
/// For that, it will first write the dummy crate into a file.
/// Then it will create a `StableMir` using custom arguments and then
/// it will run the compiler.
fn main() {
    let path = "item_kind_input.rs";
    generate_input(&path).unwrap();
    let args = vec![
        "rustc".to_string(),
        "-Cpanic=abort".to_string(),
        "--crate-type=lib".to_string(),
        "--crate-name".to_string(),
        CRATE_NAME.to_string(),
        path.to_string(),
    ];
    run!(args, tcx, test_item_kind(tcx)).unwrap();
}

fn generate_input(path: &str) -> std::io::Result<()> {
    let mut file = std::fs::File::create(path)?;
    write!(
        file,
        r#"
        pub struct Dummy(u32);
        pub const DUMMY_CONST: Dummy = Dummy(0);

        pub fn dummy() -> Dummy {{
            Dummy(5)
        }}
        "#
    )?;
    Ok(())
}
