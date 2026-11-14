//! Existing preparation checks over sealed runtimes and executable source.

use std::path::PathBuf;

use super::*;
use crate::netlist_preparation::validated_executable_hierarchy;

fn reject_deferred_external_sources(netlist: &str) -> Result<(), PreparationError> {
    reject_deferred_external_sources_with_project_runtimes(
        netlist,
        &Default::default(),
        &Default::default(),
    )?;
    validated_executable_hierarchy(netlist).map(|_| ())
}

#[test]
fn every_runtime_external_input_category_fails_closed() {
    let cases = [
        ".include model.lib",
        ".lib model.lib TT",
        ".spef_include parasitics.spef",
        ".VERILOGA compact_model.va",
        ".VA compact_model.va",
        ".ahdl_include compact_model.va",
        ".hdl compact_model.va",
        ".verilog compact_model.va",
        ".vams compact_model.va",
        ".load prior.raw",
        "Vstim in 0 PWL FILE \"wave.csv\"",
        ".measure tran fit ERROR V(out) FILE reference.prn DEPVARCOL 2",
        ".model process d_process (process_file = worker)\nAproc [in] [out] null process",
        "Aproc [in] [out] null process process_file = worker",
        ".model cosim d_cosim (simulation = \"payload.dll\")",
        "Aco [in] [out] null cosim simulation = provider",
    ];

    for line in cases {
        let Err(error) = reject_deferred_external_sources(&format!("deck\n{line}\n.end\n")) else {
            panic!("unsealed runtime input must be rejected: {line}");
        };
        assert_eq!(error.stage(), PreparationStage::SourceChecks, "{line}");
        assert!(
            error.message().contains("unsealed external dependency"),
            "{line}"
        );
    }
}

#[test]
fn project_veriloga_path_is_exact_while_spice_identifiers_ignore_case() {
    let source_key = "__rspice_project__/project/digest/Model.va";
    assert!(project_veriloga_directive_matches_exact_identity(
        ".VERILOGA \"__rspice_project__/project/digest/Model.va\" OWNED",
        source_key,
        "owned",
    ));
    assert!(!project_veriloga_directive_matches_exact_identity(
        ".VERILOGA \"__rspice_project__/project/digest/model.va\" OWNED",
        source_key,
        "owned",
    ));
}

#[test]
fn continuation_folding_cannot_hide_external_dependencies() {
    for source in [
        "deck\nBlookup out 0 V=table\n+ (\"C:/curves/transfer.tbl\")\n.end\n",
        "deck\nVstim in 0 PWL(\n+ FILE = \"C:/stimulus/wave.csv\"\n+ )\n.end\n",
        "deck\n.model cosim d_cosim (simulation\n* comment between continued records\n+ = \"C:/plugins/payload.dll\")\n.end\n",
        "deck\n.model process d_process (process_file\n+ = 'workers/stimulus.exe')\nAproc [in] [out] null process\n.end\n",
    ] {
        let error = reject_deferred_external_sources(source)
            .expect_err("continued external dependency must fail closed");
        assert_eq!(error.stage(), PreparationStage::SourceChecks, "{source}");
        assert!(
            error.message().contains("unsealed external dependency"),
            "{source}: {error}"
        );
    }
}

#[test]
fn benign_continuations_remain_accepted() {
    for source in [
        "deck\nBinline out 0 V=table(\n+ V(in), 0, 0, 1, 1)\n.end\n",
        "deck\n.model diode D(\n+ IS=1e-12\n+ N=1.1)\n.end\n",
    ] {
        reject_deferred_external_sources(source)
            .unwrap_or_else(|error| panic!("benign continuation was rejected: {source}: {error}"));
    }
}

#[test]
fn every_behavioral_file_lookup_alias_fails_closed() {
    let functions = [
        "table",
        "tablefile",
        "fasttable",
        "fasttablefile",
        "cubic",
        "cubicfile",
        "akima",
        "akimafile",
        "spline",
        "splinefile",
        "wodicka",
        "wodickafile",
        "bli",
        "blifile",
        "barycentric",
        "barycentricfile",
    ];

    for function in functions {
        let line = format!("Blookup out 0 V={function}(\"curve.dat\")");
        assert_eq!(
            deferred_external_source_reason(&line),
            Some("file-backed behavioral lookup"),
            "{function}"
        );
    }
}

#[test]
fn external_input_audit_ignores_comments_and_inline_data() {
    for line in [
        "* .include ignored.lib",
        "// .VERILOGA ignored.va",
        "R1 out 0 1k $ file=ignored.tbl",
        "R2 out 0 2k ; file=ignored.tbl",
        "R3 out 0 3k // file=ignored.tbl",
        ".data sweep_values",
        "+ 0 1 2 3",
        ".enddata",
        "Binline out 0 V=table(V(in), 0, 0, 1, 1)",
        ".param profile=1",
    ] {
        assert_eq!(deferred_external_source_reason(line), None, "{line}");
    }

    for line in [
        "Aco $G_DPWR [in] [out] null cosim simulation=provider",
        ".model source d_source (input_file='stimulus;production.txt')",
        ".model source d_source (input_file=\"stimulus\\\"production.txt\")",
    ] {
        assert!(
            deferred_external_source_reason(line).is_some(),
            "executable dependency must survive comment scanning: {line}"
        );
    }
}

/// A data file a code model reads is named for what it is, and a run's own
/// deck passes: its preparation seals the file through the engine's own
/// parameter resolution, defaults included.
#[test]
fn code_model_data_files_are_left_to_preparation_to_seal() {
    for line in [
        ".model touchstone xfer (file = \"network.s2p\")",
        ".model source d_source (input_file = \"stimulus.txt\")",
        ".model state d_state (state_file = \"state.tbl\")",
        ".model dump d_vcd (file=\"ports.vcd\" signals=\"tb.clk\")",
        "Asrc [d] source input_file=\"other.txt\"",
    ] {
        assert_eq!(
            deferred_external_source_reason(line),
            Some(CODE_MODEL_DATA_FILE),
            "{line}"
        );
        reject_deferred_external_sources_with_project_runtimes(
            &format!("deck\n{line}\n.end\n"),
            &Default::default(),
            &Default::default(),
        )
        .unwrap_or_else(|error| panic!("{line}: {error}"));
    }
}

fn process_deck(program: &str) -> String {
    format!(
        "deck\n.model peer d_process (process_file = \"{program}\")\n\
         Aone [clk] clk null [q] peer\nAtwo [clk] clk null [r] peer\n.end\n"
    )
}

#[test]
fn a_d_process_program_named_absolutely_passes_where_programs_start() {
    let program = std::env::temp_dir().join("toggle");
    let program = program.to_string_lossy();
    reject_deferred_external_sources_on(&process_deck(&program), &Default::default(), true, None)
        .unwrap_or_else(|error| {
            panic!("an absolute program is started as the CLI starts it: {error}")
        });
    let error = reject_deferred_external_sources_on(
        &process_deck("peers/toggle"),
        &Default::default(),
        true,
        None,
    )
    .expect_err("a relative program no deck directory resolved is refused");
    assert_eq!(error.stage(), PreparationStage::SourceChecks);
    assert!(
        error
            .message()
            .contains("'peers/toggle' is named relative to no deck directory")
            && error.message().contains("line 2"),
        "{error}"
    );
}

/// A directory holding one program, `name`, for a search path.
fn search_path_holding(name: &str) -> PathBuf {
    let directory =
        std::env::temp_dir().join(format!("rspice-search-path-{name}-{}", std::process::id()));
    std::fs::create_dir_all(&directory).unwrap();
    std::fs::write(directory.join(name), "").unwrap();
    directory
}

#[test]
fn a_bare_d_process_program_on_the_search_path_passes_and_one_found_nowhere_is_refused() {
    let directory = search_path_holding("rspice-peer");
    let search_path = std::env::join_paths([std::env::temp_dir(), directory.clone()]).unwrap();
    reject_deferred_external_sources_on(
        &process_deck("rspice-peer"),
        &Default::default(),
        true,
        Some(&search_path),
    )
    .unwrap_or_else(|error| panic!("a bare program on PATH starts from there: {error}"));
    let error = reject_deferred_external_sources_on(
        &process_deck("rspice-absent-peer"),
        &Default::default(),
        true,
        Some(&search_path),
    )
    .expect_err("a bare program found nowhere is refused");
    assert_eq!(error.stage(), PreparationStage::SourceChecks);
    assert!(
        error.message().contains(
            "no file beside the deck and no directory on PATH holds the program \
             'rspice-absent-peer'"
        ) && error.message().contains("line 2"),
        "{error}"
    );
    // A FIFO pair names files, never a program to look up.
    let error = reject_deferred_external_sources_on(
        &process_deck("rspice-peer|"),
        &Default::default(),
        true,
        Some(&search_path),
    )
    .expect_err("a FIFO pair no deck directory resolved is refused");
    assert!(
        error
            .message()
            .contains("named relative to no deck directory"),
        "{error}"
    );
    std::fs::remove_dir_all(&directory).unwrap();
}

#[test]
fn a_host_that_cannot_start_programs_refuses_each_d_process_instance_by_name() {
    let program = std::env::temp_dir().join("toggle");
    let program = program.to_string_lossy();
    let error = reject_deferred_external_sources_on(
        &process_deck(&program),
        &Default::default(),
        false,
        None,
    )
    .expect_err("a browser run cannot start a program");
    assert_eq!(error.stage(), PreparationStage::SourceChecks);
    let message = error.message();
    assert!(
        message.contains("d_process instance 'Aone', 'Atwo' starts the program")
            && message.contains(&*program)
            && message.contains("line 2")
            && message.contains("cannot start a program"),
        "{message}"
    );
    // The refusal is structured: the capability, the instances needing it,
    // and the line naming their program.
    let refusal = error.refusal().expect("a refusal, not a defect");
    assert_eq!(refusal.capability, PROCESS_PROGRAM_CAPABILITY);
    assert_eq!(refusal.instances, ["Aone", "Atwo"]);
    assert_eq!(error.line(), Some(2));
}

#[test]
fn a_d_process_model_no_instance_names_starts_nothing() {
    let deck = "deck\n.model peer d_process (process_file = \"toggle\")\n.end\n";
    for starts_programs in [true, false] {
        reject_deferred_external_sources_on(deck, &Default::default(), starts_programs, None)
            .unwrap_or_else(|error| panic!("an unused model starts nothing: {error}"));
    }
}
