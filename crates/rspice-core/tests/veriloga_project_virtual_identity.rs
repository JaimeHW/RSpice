#![cfg(feature = "veriloga")]

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use rspice_core::{Engine, Netlist};

static TEST_NONCE: AtomicU64 = AtomicU64::new(0);

#[test]
fn case_altered_project_virtual_path_cannot_compile_an_ambient_file() {
    let nonce = TEST_NONCE.fetch_add(1, Ordering::Relaxed);
    let unique = format!("{}-{nonce}", std::process::id());
    let exact_source_key = PathBuf::from(format!(
        "__rspice_project__/{unique}/0123456789abcdef/model.va"
    ));
    std::fs::create_dir_all(exact_source_key.parent().expect("virtual key parent"))
        .expect("materialize adversarial ambient directory");
    std::fs::write(
        &exact_source_key,
        "module ambient_escape(p, n); inout p, n; electrical p, n; analog I(p,n) <+ V(p,n); endmodule\n",
    )
    .expect("materialize compilable adversarial ambient source");

    let altered_source_key = format!("__RSPICE_PROJECT__/{unique}/0123456789abcdef/model.va");
    let deck = format!(
        "virtual identity regression\nR1 out 0 1k\n.veriloga \"{altered_source_key}\" ambient_escape\n.op\n.end\n"
    );
    let netlist = Netlist::parse(&deck).expect("parse adversarial deck");
    let error = Engine::default()
        .run_dc_op(&netlist)
        .expect_err("case-altered project key must fail before ambient compilation");
    assert!(
        error
            .to_string()
            .contains("is not installed for this execution"),
        "unexpected error: {error}"
    );

    std::fs::remove_dir_all(PathBuf::from("__rspice_project__").join(unique))
        .expect("remove adversarial ambient path");
}
