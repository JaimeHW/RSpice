//! Existing preparation checks over sealed runtimes and executable source.

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
        ".load prior.raw",
        "Vstim in 0 PWL FILE \"wave.csv\"",
        ".measure tran fit ERROR V(out) FILE reference.prn DEPVARCOL 2",
        ".model touchstone transfer (file = \"network.s2p\")",
        ".model source d_source (input_file = \"stimulus.txt\")",
        ".model state d_state (state_file = \"state.tbl\")",
        ".model process d_process (process_file = worker)",
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
        "deck\n.model source d_source (input_file\n+ = 'C:/stimulus/input.txt')\n.end\n",
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
