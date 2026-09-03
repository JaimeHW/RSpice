//! The published wire contract is derived from the constants, not repeated
//! beside them.
//!
//! `README.md` publishes the protocol version, the two digest versions, and
//! the schema identifier and version of every document the executor writes.
//! A controller is built against those numbers, so a table that drifts from
//! the code is a compatibility break that looks like documentation. This test
//! reads the published table and requires every row to be the constant it
//! names.
//!
//! It also covers the transient checkpoint's format version, which the
//! `rspice-core` README publishes as the header a saved checkpoint carries:
//! that header is what a reader matches on, so a README that names a
//! different one sends an operator looking for the wrong file.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use rspice_core::engine::{Engine, SimulationConfig, TransientCheckpointEncoding};
use rspice_core::execution::{ANALYSIS_RESULT_DOCUMENT_SCHEMA, ANALYSIS_RESULT_DOCUMENT_VERSION};
use rspice_core::netlist::Netlist;
use rspice_engine_adapter::axis_execution_document::{
    AXIS_EXECUTION_SCHEMA, AXIS_EXECUTION_VERSION,
};
use rspice_engine_adapter::fft_result_document::{
    FFT_RESULT_DOCUMENT_SCHEMA, FFT_RESULT_DOCUMENT_VERSION,
};
use rspice_engine_adapter::wire::{
    CURRENT_REVISION_CONTENT_DIGEST_VERSION, CURRENT_SIMULATION_REQUEST_DIGEST_VERSION,
    INTEGRITY_ENGINE_PROTOCOL_VERSION,
};

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("rspice-engine-adapter is a workspace crate under crates/")
        .to_path_buf()
}

fn read_readme(crate_name: &str) -> String {
    let path = workspace_root()
        .join("crates")
        .join(crate_name)
        .join("README.md");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
}

/// Parse a two-column markdown table into `element -> value`.
fn two_column_table(markdown: &str, heading: &str) -> BTreeMap<String, String> {
    let section = markdown
        .split_once(heading)
        .unwrap_or_else(|| panic!("README documents a `{heading}` section"))
        .1;
    let section = section
        .split_once("\n## ")
        .map_or(section, |(before, _)| before);

    let mut rows = BTreeMap::new();
    for line in section.lines() {
        let line = line.trim();
        let Some(row) = line.strip_prefix('|').and_then(|row| row.strip_suffix('|')) else {
            continue;
        };
        let cells = row.split('|').map(str::trim).collect::<Vec<_>>();
        if cells.len() != 2 || cells[1].starts_with("---") {
            continue;
        }
        rows.insert(cells[0].to_owned(), cells[1].to_owned());
    }
    rows
}

/// Strip markdown code fencing so `` `4` `` compares as `4`.
fn unfenced(value: &str) -> String {
    value.replace('`', "")
}

#[test]
fn the_published_wire_contract_matches_the_constants_it_names() {
    let readme = read_readme("rspice-engine-adapter");
    let table = two_column_table(&readme, "## Wire contract");
    assert!(
        !table.is_empty(),
        "the adapter README's wire-contract table is empty or was restructured"
    );

    let expected: BTreeMap<&str, String> = BTreeMap::from([
        (
            "Protocol version",
            format!(
                "{INTEGRITY_ENGINE_PROTOCOL_VERSION} (RSPICE_ENGINE_PROTOCOL_VERSION={INTEGRITY_ENGINE_PROTOCOL_VERSION})"
            ),
        ),
        (
            "Request digest version",
            CURRENT_SIMULATION_REQUEST_DIGEST_VERSION.to_string(),
        ),
        (
            "Revision content digest version",
            CURRENT_REVISION_CONTENT_DIGEST_VERSION.to_string(),
        ),
        (
            "Typed result document",
            format!(
                "{ANALYSIS_RESULT_DOCUMENT_SCHEMA} v{ANALYSIS_RESULT_DOCUMENT_VERSION} (rspice-core)"
            ),
        ),
        (
            "Transient FFT bundle",
            format!("{FFT_RESULT_DOCUMENT_SCHEMA} v{FFT_RESULT_DOCUMENT_VERSION}"),
        ),
        (
            "Run-axis orchestration record",
            format!("{AXIS_EXECUTION_SCHEMA} v{AXIS_EXECUTION_VERSION}"),
        ),
    ]);

    for (element, value) in expected {
        let documented = table
            .get(element)
            .unwrap_or_else(|| panic!("the wire-contract table no longer documents '{element}'"));
        assert_eq!(
            unfenced(documented),
            value,
            "the README documents '{element}' as {documented:?}, but the constant says {value:?}"
        );
    }
}

#[test]
fn the_documented_result_document_version_is_the_one_the_adapter_writes() {
    // The README's claim is checked against a real published document rather
    // than against the constant alone, so a schema string that stopped being
    // written would fail here too.
    let readme = read_readme("rspice-engine-adapter");
    assert!(
        readme.contains(&format!(
            "`{ANALYSIS_RESULT_DOCUMENT_SCHEMA}` v{ANALYSIS_RESULT_DOCUMENT_VERSION}"
        )),
        "the adapter README no longer names the shared result document's schema and version"
    );

    let wasm_readme = read_readme("rspice-wasm");
    assert!(
        wasm_readme.contains(&format!(
            "`{ANALYSIS_RESULT_DOCUMENT_SCHEMA}` version {ANALYSIS_RESULT_DOCUMENT_VERSION}"
        )),
        "the WASM README documents a different shared result document version"
    );
}

#[test]
fn a_saved_checkpoint_carries_the_header_its_documentation_names() {
    let readme = read_readme("rspice-core");
    let netlist = Netlist::parse(
        "checkpoint header contract\n\
         V1 in 0 PULSE(0 1 0 100n 100n 4u 10u)\n\
         R1 in out 1k\n\
         C1 out 0 2n\n\
         .TRAN 50n 2u\n\
         .END\n",
    )
    .expect("the checkpoint fixture parses");
    let (_, checkpoint) = Engine::new(SimulationConfig::default())
        .run_tran_checkpointed(&netlist, 1.0e-6, 50.0e-9)
        .expect("the checkpoint fixture solves its first segment");
    let text = checkpoint.to_text();
    let header = text
        .lines()
        .next()
        .expect("a checkpoint's first line is its versioned header")
        .trim()
        .to_owned();
    assert!(
        header.starts_with("RSPICE-CHECKPOINT "),
        "a saved checkpoint no longer begins with its versioned header: {header:?}"
    );

    // The packed envelope is a different representation of the same
    // checkpoint and carries its own magic; both are part of the contract a
    // reader matches on.
    let packed = checkpoint
        .to_bytes(TransientCheckpointEncoding::Packed)
        .expect("the fixture packs");
    assert!(packed.starts_with(b"RSPICE-CPACK"));

    // The core README names the checkpoint path; if it ever quotes the header
    // itself, the quoted spelling must be the one the encoder writes.
    if let Some(quoted) = readme.split("RSPICE-CHECKPOINT ").nth(1) {
        let documented_version: String = quoted.chars().take_while(char::is_ascii_digit).collect();
        assert!(
            !documented_version.is_empty()
                && header.ends_with(&documented_version)
                && header == format!("RSPICE-CHECKPOINT {documented_version}"),
            "the core README names checkpoint header version {documented_version:?} but a \
             saved checkpoint writes {header:?}"
        );
    }
}
