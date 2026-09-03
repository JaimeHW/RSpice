//! The published contract of the engine's typed error taxonomy.
//!
//! Every frontend — the CLI's exit codes, the Python exception classes, the
//! WASM error objects, the engine adapter's wire codes — derives its behavior
//! from [`SimulationError::descriptor`]. These tests pin the round trip from
//! variant to code and category, exhaustively, so a new variant cannot reach a
//! frontend without a deliberate decision about what it means.

use rspice_core::abort_signal::{AbortReason, AbortSignal, ImmediateAbort};
use rspice_core::execution::{
    AnalysisInstanceId, AnalysisKind, AxisKind, DeckPlan, RunCoordinateId,
    numeric_run_coordinate_id,
};
use rspice_core::netlist::NetlistSourceLocation;
use rspice_core::solver::SolverError;
use rspice_core::{
    MaterializationMismatchError, Netlist, OutputCommitError, OutputCommitPhase,
    PersistenceIncompatibleError, RequestedSignalUnavailableError, ResourceKind,
    ResultSchemaMismatchError, SimulationConfigError, SimulationError, SimulationErrorCategory,
    SimulationErrorCode, UnsupportedCapabilityError,
};

/// An analysis identity minted by the planner, which is the only way one is
/// created — the taxonomy carries planner identities, not test fictions.
fn planned_analysis_id() -> AnalysisInstanceId {
    let netlist = Netlist::parse("identity\nV1 in 0 1\nR1 in 0 1k\n.ac dec 1 1 10\n.end\n")
        .expect("identity deck parses");
    let plan = DeckPlan::from_netlist(&netlist, &rspice_core::ResourceLimits::default())
        .expect("canonical deck plan");
    let id = plan.analyses()[0].id();
    assert_eq!(id.kind(), AnalysisKind::Ac);
    id
}

fn coordinate_id() -> RunCoordinateId {
    numeric_run_coordinate_id(&[(AxisKind::Step, "p", 1.0)], 0)
        .expect("numeric coordinate identity")
}

#[test]
fn resource_failures_publish_stable_numeric_metadata() {
    let limit = rspice_core::ResourceLimitError {
        resource: ResourceKind::AnalysisPoints,
        requested: 101,
        limit: 100,
    };

    for error in [
        SimulationError::ResourceLimit(limit),
        SimulationError::Configuration(SimulationConfigError::ResourceLimit(limit)),
    ] {
        let descriptor = error.descriptor();
        assert_eq!(descriptor.code, SimulationErrorCode::ResourceLimit);
        assert_eq!(descriptor.code.as_str(), "resource_limit");
        assert_eq!(descriptor.category, SimulationErrorCategory::ResourceLimit);
        assert!(!descriptor.retryable);
        assert_eq!(descriptor.resource_limit, Some(limit));
        assert_eq!(descriptor.iterations, None);
    }
}

#[test]
fn convergence_metadata_covers_engine_and_nested_solver_failures() {
    let direct = SimulationError::ConvergenceFailed(17).descriptor();
    assert_eq!(direct.code, SimulationErrorCode::ConvergenceError);
    assert_eq!(direct.category, SimulationErrorCategory::Convergence);
    assert_eq!(direct.iterations, Some(17));
    assert!(!direct.retryable);

    let nested = SimulationError::Solver(SolverError::ConvergenceFailed(23)).descriptor();
    assert_eq!(nested.code, SimulationErrorCode::SolverError);
    assert_eq!(nested.category, SimulationErrorCategory::Solver);
    assert_eq!(nested.iterations, Some(23));
}

#[test]
fn stops_are_the_only_automatically_retryable_failures() {
    let cancelled = SimulationError::Aborted.descriptor();
    assert_eq!(cancelled.code, SimulationErrorCode::Aborted);
    assert_eq!(cancelled.category, SimulationErrorCategory::Cancellation);
    assert!(cancelled.retryable);
    assert_eq!(cancelled.iterations, None);
    assert_eq!(cancelled.resource_limit, None);

    let expired = SimulationError::TimeLimitExceeded.descriptor();
    assert_eq!(expired.code, SimulationErrorCode::TimeLimitExceeded);
    assert_eq!(expired.category, SimulationErrorCategory::Timeout);
    assert!(expired.retryable);
}

#[test]
fn a_deadline_signal_relabels_a_propagated_stop_as_a_timeout() {
    struct Deadline;
    impl AbortSignal for Deadline {
        fn is_aborted(&self) -> bool {
            true
        }
        fn abort_reason(&self) -> AbortReason {
            AbortReason::TimeLimit
        }
    }

    assert_eq!(
        SimulationError::Aborted
            .with_abort_reason(&Deadline)
            .descriptor()
            .category,
        SimulationErrorCategory::Timeout
    );
    assert_eq!(
        SimulationError::TimeLimitExceeded
            .with_abort_reason(&ImmediateAbort)
            .descriptor()
            .category,
        SimulationErrorCategory::Cancellation
    );
    // A real failure is not a stop, so re-labelling must not touch it.
    assert_eq!(
        SimulationError::ConvergenceFailed(3)
            .with_abort_reason(&Deadline)
            .descriptor()
            .code,
        SimulationErrorCode::ConvergenceError
    );
}

#[test]
fn capability_refusals_carry_a_stable_token_analysis_and_source_span() {
    let analysis = planned_analysis_id();
    let coordinate = coordinate_id();
    let location = NetlistSourceLocation::in_file("decks/mixer.cir", 91);
    let error = SimulationError::from(
        UnsupportedCapabilityError::new(
            "analysis.hb.device",
            "device 'Q1' (vbic) has no harmonic-balance stamp",
        )
        .with_analysis(analysis)
        .with_coordinate(coordinate)
        .at(location.clone()),
    );

    let descriptor = error.descriptor();
    assert_eq!(descriptor.code, SimulationErrorCode::UnsupportedCapability);
    assert_eq!(descriptor.code.as_str(), "unsupported_capability");
    assert_eq!(descriptor.category, SimulationErrorCategory::Capability);
    assert_eq!(descriptor.category.as_str(), "capability");
    assert!(!descriptor.retryable);
    assert_eq!(descriptor.analysis, Some(analysis));
    assert_eq!(descriptor.coordinate, Some(coordinate));
    assert_eq!(error.source_location(), Some(&location));

    let message = error.to_string();
    assert!(
        message.starts_with("decks/mixer.cir:91: unsupported capability [analysis.hb.device]"),
        "message must lead with the span and the token: {message}"
    );
    assert!(
        message.contains("has no harmonic-balance stamp"),
        "message must keep the human explanation: {message}"
    );
}

#[test]
fn materialization_mismatches_are_engine_errors_that_name_their_coordinate() {
    let coordinate = coordinate_id();
    let error = SimulationError::from(MaterializationMismatchError::AnalysisIdentity {
        coordinate,
        expected: vec![AnalysisKind::Op],
        actual: vec![AnalysisKind::Tran],
    });
    let descriptor = error.descriptor();
    assert_eq!(
        descriptor.code,
        SimulationErrorCode::MaterializationMismatch
    );
    assert_eq!(
        descriptor.category,
        SimulationErrorCategory::Materialization
    );
    assert_eq!(descriptor.coordinate, Some(coordinate));
    assert_eq!(
        SimulationError::from(MaterializationMismatchError::PlanNetlist)
            .descriptor()
            .coordinate,
        None
    );
}

#[test]
fn persistence_and_output_commit_failures_are_separate_categories() {
    let persistence = SimulationError::from(
        PersistenceIncompatibleError::new("transient checkpoint", Some(99), "1..=17")
            .with_detail("written by a newer build"),
    );
    let descriptor = persistence.descriptor();
    assert_eq!(
        descriptor.code,
        SimulationErrorCode::PersistenceIncompatible
    );
    assert_eq!(descriptor.category, SimulationErrorCategory::Persistence);
    assert_eq!(
        persistence.to_string(),
        "transient checkpoint format version 99 is not readable by this build (supported: 1..=17): written by a newer build"
    );

    let commit = SimulationError::from(OutputCommitError::new(
        "results/out.csv",
        OutputCommitPhase::Commit {
            destination_intact: true,
        },
        "Access is denied",
    ));
    let descriptor = commit.descriptor();
    assert_eq!(descriptor.code, SimulationErrorCode::OutputCommitFailed);
    assert_eq!(descriptor.category, SimulationErrorCategory::OutputCommit);
    assert!(
        commit
            .to_string()
            .contains("output commit failed during commit"),
        "commit failures must name their phase: {commit}"
    );
}

#[test]
fn result_schema_mismatch_is_a_typed_non_retryable_output_failure() {
    let expected_names = vec!["0".to_string(), "out".to_string()];
    let actual_names = vec!["out".to_string(), "0".to_string()];
    let error = SimulationError::result_schema_mismatch(
        "AC",
        Some("frequency point 7 (1.0000000000000000e+6 Hz)".to_string()),
        "node voltages",
        expected_names.clone(),
        actual_names.clone(),
        2,
        1,
    );

    let descriptor = error.descriptor();
    assert_eq!(descriptor.code, SimulationErrorCode::ResultSchemaMismatch);
    assert_eq!(descriptor.code.as_str(), "result_schema_mismatch");
    assert_eq!(descriptor.category, SimulationErrorCategory::ResultSchema);
    assert!(!descriptor.retryable);
    assert_eq!(descriptor.iterations, None);
    assert_eq!(descriptor.resource_limit, None);
    assert_eq!(
        error.to_string(),
        "result schema mismatch for AC analysis at frequency point 7 (1.0000000000000000e+6 Hz) in node voltages: expected names [\"0\", \"out\"] with 2 value(s), got names [\"out\", \"0\"] with 1 value(s)"
    );

    let SimulationError::ResultSchemaMismatch(detail) = error else {
        panic!("typed result-schema variant was lost");
    };
    assert_eq!(
        *detail,
        ResultSchemaMismatchError {
            analysis: None,
            analysis_label: "AC".to_string(),
            coordinate: None,
            coordinate_label: Some("frequency point 7 (1.0000000000000000e+6 Hz)".to_string()),
            signal_family: "node voltages".to_string(),
            expected_names,
            actual_names,
            expected_value_count: 2,
            actual_value_count: 1,
        }
    );
}

#[test]
fn schema_mismatch_without_a_coordinate_preserves_empty_registries() {
    let detail = ResultSchemaMismatchError::new(
        "TRAN",
        None,
        "branch currents",
        Vec::new(),
        vec!["V1".to_string()],
        0,
        1,
    );
    assert_eq!(
        detail.to_string(),
        "result schema mismatch for TRAN analysis in branch currents: expected names [] with 0 value(s), got names [\"V1\"] with 1 value(s)"
    );
    assert_eq!(detail.coordinate_label, None);
}

#[test]
fn result_errors_carry_typed_identities_when_the_producer_knows_them() {
    let analysis = planned_analysis_id();
    let coordinate = coordinate_id();

    let unavailable = SimulationError::from(
        RequestedSignalUnavailableError::new("@Mdriver[Id]", "AC", None)
            .with_analysis(analysis)
            .with_coordinate(coordinate),
    );
    assert_eq!(unavailable.analysis_instance(), Some(analysis));
    assert_eq!(unavailable.run_coordinate(), Some(coordinate));

    let schema = SimulationError::from(
        ResultSchemaMismatchError::new("AC", None, "node voltages", Vec::new(), Vec::new(), 0, 0)
            .with_analysis(analysis)
            .with_coordinate(coordinate),
    );
    assert_eq!(schema.descriptor().analysis, Some(analysis));
    assert_eq!(schema.descriptor().coordinate, Some(coordinate));
}

/// The code and category every variant must publish.
///
/// This match is deliberately exhaustive at the variant level: adding a
/// `SimulationError` variant without deciding what it means to a frontend
/// fails to compile here rather than silently reaching one as a wildcard.
fn expected_descriptor(
    error: &SimulationError,
) -> (SimulationErrorCode, SimulationErrorCategory, bool) {
    use SimulationErrorCategory as Category;
    use SimulationErrorCode as Code;
    match error {
        // The nested resource-limit configuration case is checked separately
        // above; this arm covers every other configuration failure.
        SimulationError::Configuration(SimulationConfigError::ResourceLimit(_)) => {
            (Code::ResourceLimit, Category::ResourceLimit, false)
        }
        SimulationError::Configuration(_) => {
            (Code::InvalidConfiguration, Category::Configuration, false)
        }
        SimulationError::ResourceLimit(_) => (Code::ResourceLimit, Category::ResourceLimit, false),
        SimulationError::Circuit(_) => (Code::CircuitError, Category::Simulation, false),
        SimulationError::BehavioralReference(_) => {
            (Code::BehavioralReferenceError, Category::Simulation, false)
        }
        SimulationError::UnsupportedCapability(_) => {
            (Code::UnsupportedCapability, Category::Capability, false)
        }
        SimulationError::MaterializationMismatch(_) => (
            Code::MaterializationMismatch,
            Category::Materialization,
            false,
        ),
        SimulationError::Solver(_) => (Code::SolverError, Category::Solver, false),
        SimulationError::Netlist(_) => (Code::NetlistError, Category::Netlist, false),
        SimulationError::RequestedSignalUnavailable(_) => (
            Code::RequestedSignalUnavailable,
            Category::SignalUnavailable,
            false,
        ),
        SimulationError::ResultSchemaMismatch(_) => {
            (Code::ResultSchemaMismatch, Category::ResultSchema, false)
        }
        SimulationError::PersistenceIncompatible(_) => {
            (Code::PersistenceIncompatible, Category::Persistence, false)
        }
        SimulationError::OutputCommitFailed(_) => {
            (Code::OutputCommitFailed, Category::OutputCommit, false)
        }
        SimulationError::ConvergenceFailed(_) => {
            (Code::ConvergenceError, Category::Convergence, false)
        }
        SimulationError::Aborted => (Code::Aborted, Category::Cancellation, true),
        SimulationError::TimeLimitExceeded => (Code::TimeLimitExceeded, Category::Timeout, true),
    }
}

/// One instance of every variant, so the round trip is checked against real
/// values rather than against the mapping table alone.
fn one_of_every_variant() -> Vec<SimulationError> {
    vec![
        SimulationError::Configuration(SimulationConfigError::InvalidCount {
            field: "max_iterations",
            value: 0,
        }),
        SimulationError::ResourceLimit(rspice_core::ResourceLimitError {
            resource: ResourceKind::MatrixUnknowns,
            requested: 2,
            limit: 1,
        }),
        SimulationError::Circuit("device stamp failed".to_string()),
        SimulationError::BehavioralReference(Box::new(
            rspice_core::device::BehavioralReferenceError {
                owner_name: "b2".to_string(),
                canonical_owner_name: "B2".to_string(),
                dependency_name: "b1".to_string(),
                canonical_dependency_name: "B1".to_string(),
                reason:
                    rspice_core::device::BehavioralReferenceReason::LeadCurrentNotSolutionVariable,
            },
        )),
        SimulationError::unsupported_capability("analysis.pz.device", "no PZ stamp"),
        SimulationError::from(MaterializationMismatchError::PlanNetlist),
        SimulationError::Solver(SolverError::ConvergenceFailed(9)),
        SimulationError::Netlist("no analyses".to_string()),
        SimulationError::requested_signal_unavailable("V(x)", "DC", None),
        SimulationError::result_schema_mismatch(
            "TRAN",
            None,
            "node voltages",
            Vec::new(),
            Vec::new(),
            0,
            0,
        ),
        SimulationError::from(PersistenceIncompatibleError::new(
            "transient checkpoint",
            Some(3),
            "1..=2",
        )),
        SimulationError::from(OutputCommitError::new(
            "out.csv",
            OutputCommitPhase::Prepare,
            "no such directory",
        )),
        SimulationError::ConvergenceFailed(5),
        SimulationError::Aborted,
        SimulationError::TimeLimitExceeded,
    ]
}

#[test]
fn every_variant_round_trips_through_its_descriptor() {
    let mut seen_codes = std::collections::BTreeSet::new();
    let mut seen_categories = std::collections::BTreeSet::new();

    for error in one_of_every_variant() {
        let (code, category, retryable) = expected_descriptor(&error);
        let descriptor = error.descriptor();
        assert_eq!(descriptor.code, code, "code mismatch for {error}");
        assert_eq!(
            descriptor.category, category,
            "category mismatch for {error}"
        );
        assert_eq!(
            descriptor.retryable, retryable,
            "retry policy mismatch for {error}"
        );
        assert!(
            !descriptor.code.as_str().is_empty() && !descriptor.category.as_str().is_empty(),
            "every descriptor needs stable wire text: {error}"
        );
        seen_codes.insert(descriptor.code.as_str());
        seen_categories.insert(descriptor.category.as_str());
    }

    // Only the two resource-limit spellings share a code, so the sample set
    // must have produced one code per variant it contains.
    assert_eq!(seen_codes.len(), 15, "codes covered: {seen_codes:?}");

    for category in SimulationErrorCategory::ALL {
        assert!(
            seen_categories.contains(category.as_str()),
            "category {category} is declared but no variant produces it"
        );
    }
}

#[test]
fn codes_and_categories_have_distinct_stable_text() {
    let mut categories = std::collections::BTreeSet::new();
    for category in SimulationErrorCategory::ALL {
        assert!(
            categories.insert(category.as_str()),
            "duplicate category text: {category}"
        );
    }
    assert_eq!(categories.len(), SimulationErrorCategory::ALL.len());
}
