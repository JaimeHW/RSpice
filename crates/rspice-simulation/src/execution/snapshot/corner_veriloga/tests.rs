//! A corner library's sections binding one alias each to their own Verilog-A:
//! each corner runs its own, a section binding one alias twice is refused,
//! and preflight refuses exactly what the run fails.

use rspice_app_types::product::ProcessCorner;
use rspice_core::NoAbort;
use rspice_results::analysis_type::AnalysisType;
use rspice_results::family_metadata::AnalysisResultFamilyMetadata;

use crate::execution::{PreparedRunSnapshot, SavePolicy};
use crate::model_sources::SealedModelExecutionSources;
use crate::runner::pvt_point_evidence::{
    DeclarationVerilogA, prepare_corner_declaration, run_prepared_declaration,
};
use crate::sweeps::{CornerBaseMode, CornerRunConfig};

/// One volt across 1 kΩ in series with the library's `amp`, a conductance G
/// to ground: `out` reads 1 / (1 + 1 kΩ · G).
const DECK: &str = "corner verilog-a\n\
     V1 in 0 DC 1\n\
     R1 in out 1k\n\
     X1 out 0 amp scale=1\n\
     .op\n\
     .end\n";

const CORNERS: [ProcessCorner; 2] = [ProcessCorner::TT, ProcessCorner::SS];

/// A conductance of `scale` times `siemens` from `p` to `n`, as module
/// `module`.
fn conductance(module: &str, siemens: &str) -> Vec<u8> {
    format!(
        "module {module}(p, n); inout p, n; electrical p, n; parameter real scale = 1; \
         analog I(p, n) <+ scale * {siemens} * V(p, n); endmodule\n"
    )
    .into_bytes()
}

/// The model library `root.lib` is the root of, beside `files`, imported and
/// sealed for execution.
fn sealed_library(root: &str, files: [(&str, Vec<u8>); 2]) -> SealedModelExecutionSources {
    let mut bundle = vec![("root.lib".to_owned(), root.as_bytes().to_vec())];
    bundle.extend(
        files
            .into_iter()
            .map(|(path, source)| (path.to_owned(), source)),
    );
    let (_, library) =
        crate::model_import::import_project_source_bundle("amp.lib", None, bundle, None)
            .expect("the corner library imports");
    let mut catalog = rspice_model_library::ModelCatalog::default();
    catalog.add_library(library);
    crate::model_sources::seal_catalog_execution_sources(
        &catalog,
        &rspice_model_library::ModelResolutionRecords::default(),
    )
    .expect("the corner library seals")
}

/// `tt` binds `amp` to a 1 mS conductance in `amp_tt.va`, `ss` to `ss_amp`
/// in `amp_ss.va`.
fn two_corner_library(ss_amp: Vec<u8>) -> SealedModelExecutionSources {
    sealed_library(
        ".lib tt\n.va \"amp_tt.va\" amp\n.model d_tt D is=1e-14\n.endl tt\n\
         .lib ss\n.va \"amp_ss.va\" amp\n.model d_ss D is=1e-14\n.endl ss\n",
        [
            ("amp_tt.va", conductance("amp_tt", "1m")),
            ("amp_ss.va", ss_amp),
        ],
    )
}

/// A corner run of `DECK` at tt and ss, prepared as a run is: the deck carries
/// the reference (tt) sections' cards and names their Verilog-A, and each
/// corner's point is given its own sections' runtimes.
fn prepare(sealed: &SealedModelExecutionSources) -> Result<PreparedRunSnapshot, String> {
    use crate::project_veriloga::preparation::{
        prepared_corner_veriloga_runtimes, prepared_model_library_veriloga_runtimes,
    };
    let reference = prepared_model_library_veriloga_runtimes(sealed, ProcessCorner::TT)
        .map_err(|error| error.to_string())?;
    let corners = prepared_corner_veriloga_runtimes(sealed, ProcessCorner::TT, &reference, CORNERS)
        .map_err(|error| error.to_string())?;
    let cards = sealed
        .reference_model_execution_plan(ProcessCorner::TT)?
        .model_cards();
    let mut deck =
        crate::analysis_preparation::apply_reference_model_bindings_to_netlist(DECK, &cards);
    for source in reference.sources() {
        crate::netlist_preparation::append_project_veriloga_directive(
            &mut deck,
            source.source_key(),
            source.netlist_alias(),
        );
    }
    let contract = CornerRunConfig {
        process_corners: CORNERS.to_vec(),
        voltages: vec![1.0],
        supply_source_names: vec!["V1".to_owned()],
        temperatures_c: vec![27.0],
        full_matrix: true,
        nominal_voltage: Some(1.0),
        base_mode: CornerBaseMode::Op,
        model_bindings: sealed.corner_model_bindings(&CORNERS)?,
        points: Vec::new(),
    };
    prepare_corner_declaration(
        &deck,
        contract,
        27.0,
        DeclarationVerilogA {
            run: reference,
            corners,
        },
    )
}

/// Each point's label and whether it solved, in run order.
fn point_outcomes(run: &rspice_results::run::SimulationRun) -> Vec<(String, bool)> {
    run.analyses
        .iter()
        .filter(|analysis| {
            analysis
                .provenance()
                .is_some_and(|provenance| provenance.pvt_point().is_some())
        })
        .map(|analysis| (analysis.label.clone(), analysis.success))
        .collect()
}

/// `amp` is 1 mS at tt and 2 mS at ss. Both corners run, and each solves its
/// own module: `out` is 1/2 V at tt and 1/3 V at ss.
#[test]
fn each_corner_runs_the_verilog_a_its_own_section_binds() {
    let snapshot = prepare(&two_corner_library(conductance("amp_ss", "2m")))
        .expect("a library binding one alias per section prepares");
    let run = run_prepared_declaration(snapshot, SavePolicy::RetainEngineProducedResults)
        .expect("the corner run authorizes and runs");

    let family = run
        .analyses
        .iter()
        .find(|analysis| analysis.analysis_type == AnalysisType::Corner)
        .expect("the declaration assembles a corner family");
    assert!(
        family.success,
        "corner family failed: {:?} {:?}",
        family.error_message,
        point_outcomes(&run)
    );
    let Some(AnalysisResultFamilyMetadata::Corner {
        corner_labels,
        failed_corners,
        ..
    }) = &family.family_metadata
    else {
        panic!("a corner family: {:?}", family.family_metadata);
    };
    assert_eq!(
        corner_labels,
        &["TT_1.000000V_27.000000C", "SS_1.000000V_27.000000C"]
    );
    assert_eq!(*failed_corners, 0);
    let out = family
        .waveforms
        .iter()
        .find(|waveform| waveform.name == "V(OUT)")
        .expect("the family traces V(OUT)");
    let expected = [0.5, 1.0 / 3.0];
    assert_eq!(out.y.len(), expected.len(), "{:?}", out.y);
    for (corner, (solved, expected)) in out.y.iter().zip(expected).enumerate() {
        assert!(
            (solved - expected).abs() < 1.0e-9,
            "{}: V(OUT) is {solved}, expected {expected}",
            corner_labels[corner]
        );
    }
}

/// `ss` binds `amp` to two modules, so its deck would name two definitions
/// of one model. It is refused with both sources named, though `tt` alone is
/// well defined. Preflight starts from the same preparation, so it refuses
/// what the run refuses.
#[test]
fn a_section_binding_one_alias_twice_is_refused() {
    let sealed = sealed_library(
        ".lib tt\n.va \"amp_tt.va\" amp\n.model d_tt D is=1e-14\n.endl tt\n\
         .lib ss\n.va \"amp_tt.va\" amp\n.va \"amp_ss.va\" amp\n.model d_ss D is=1e-14\n.endl ss\n",
        [
            ("amp_tt.va", conductance("amp_tt", "1m")),
            ("amp_ss.va", conductance("amp_ss", "2m")),
        ],
    );

    let error = prepare(&sealed).expect_err("one alias bound twice by one selection is refused");
    assert!(
        error.contains("'amp'") && error.contains("amp_tt.va") && error.contains("amp_ss.va"),
        "{error}"
    );
}

/// Preflight builds each point's deck with the Verilog-A that point installs.
/// With both modules well formed it refuses nothing and every point solves.
/// With an ss module that declares no `scale`, the instance setting it does
/// not build at ss: preflight refuses the ss point alone, and the run fails
/// that point alone.
///
/// The Verilog-A cache is the process's, so a module another test installed
/// stays installed. Every module here is its own, so no point's deck can
/// build from a source this test did not install for it.
#[test]
fn preflight_refuses_exactly_the_corner_points_the_run_fails() {
    let well_formed = prepare(&two_corner_library(conductance("amp_ss", "4m")))
        .expect("a library binding one alias per section prepares");
    assert_eq!(well_formed.engine_refusals(&NoAbort), Vec::new());
    let run = run_prepared_declaration(well_formed, SavePolicy::RetainEngineProducedResults)
        .expect("the corner run authorizes and runs");
    let outcomes = point_outcomes(&run);
    assert!(
        outcomes.len() == 2 && outcomes.iter().all(|(_, solved)| *solved),
        "{outcomes:?}"
    );

    let unscaled = b"module amp_ss(p, n); inout p, n; electrical p, n; \
        analog I(p, n) <+ 2m * V(p, n); endmodule\n"
        .to_vec();
    let snapshot =
        prepare(&two_corner_library(unscaled)).expect("each section's module compiles on its own");
    let refusals = snapshot.engine_refusals(&NoAbort);
    let refused = refusals
        .iter()
        .flat_map(|refusal| refusal.tasks.iter().map(|task| task.label.clone()))
        .collect::<Vec<_>>();
    assert!(
        refused.len() == 1 && refused[0].contains("SS"),
        "only the ss point is refused: {refusals:#?}"
    );
    let run = run_prepared_declaration(snapshot, SavePolicy::RetainEngineProducedResults)
        .expect("the corner run authorizes and runs");
    let failed = point_outcomes(&run)
        .into_iter()
        .filter(|(_, solved)| !solved)
        .map(|(label, _)| label)
        .collect::<Vec<_>>();
    assert_eq!(failed, refused, "the run fails what preflight refused");
}
