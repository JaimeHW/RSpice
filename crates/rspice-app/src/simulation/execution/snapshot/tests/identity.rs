//! Output-selection and derived-identity snapshot coverage.

use super::*;

#[test]
fn output_selection_mode_is_authenticated_by_the_snapshot_digest() {
    let policy = |output_selection_mode| SavePolicy::PlanOwned {
        output_selection_mode,
        retained_dataset_limit: 20,
        maximum_storage_bytes: 1024 * 1024,
        live_streaming_enabled: true,
        retain_failure_diagnostics: true,
    };
    let mut automatic_parts = parts();
    automatic_parts.save_policy = policy(crate::state::OutputSelectionMode::Automatic);
    let mut explicit_parts = parts();
    explicit_parts.save_policy = policy(crate::state::OutputSelectionMode::ExplicitOnly);
    let mut save_all_parts = parts();
    save_all_parts.save_policy = policy(crate::state::OutputSelectionMode::SaveAll);

    let automatic = PreparedRunSnapshot::new(automatic_parts).expect("automatic snapshot");
    let explicit = PreparedRunSnapshot::new(explicit_parts).expect("explicit snapshot");
    let save_all = PreparedRunSnapshot::new(save_all_parts).expect("save-all snapshot");

    assert_ne!(automatic.digest(), explicit.digest());
    assert_ne!(automatic.digest(), save_all.digest());
    assert_ne!(explicit.digest(), save_all.digest());
}

/// What preparation seals and what a project file authenticates have to be one
/// statement. This walks the receipt an expanded run actually produces and
/// checks that every task either is an authored plan instance or re-derives
/// from one — exactly the predicate project load applies to a restored file.
fn assert_receipt_identities_close_over(snapshot: &PreparedRunSnapshot, authored: &[&str]) {
    let authored = authored
        .iter()
        .map(|name| instance_id(name))
        .collect::<Vec<_>>();
    let receipt = snapshot
        .prepared_run_receipt()
        .expect("an expanded run seals a receipt");
    assert!(
        receipt.tasks().len() > authored.len(),
        "an expanded run dispatches more tasks than the plan authored"
    );
    for task in receipt.tasks() {
        match task.derived_from() {
            None => assert!(
                authored.contains(&task.instance_id()),
                "a task with no derivation must be an authored instance: {}",
                task.instance_id()
            ),
            Some(derived) => {
                assert!(
                    authored.contains(&derived.authored()),
                    "a derivation must root at an authored instance: {}",
                    derived.authored()
                );
                assert_eq!(
                    derived.instance_id(),
                    task.instance_id(),
                    "a derivation must reproduce the identity its task claims"
                );
            }
        }
    }
}

#[test]
fn a_global_run_set_seals_a_derivation_for_every_point() {
    let mut run = parts();
    run.run_set = Some(global_temperature_run_set(&["-40", "125"]));
    let snapshot = PreparedRunSnapshot::new(run).expect("global Run Set prepares");

    assert_receipt_identities_close_over(&snapshot, &["op"]);
}

#[test]
fn a_declared_corner_space_seals_a_derivation_for_every_point() {
    let snapshot = PreparedRunSnapshot::new(transient_corner_parts()).expect("corner run prepares");

    assert_receipt_identities_close_over(&snapshot, &["corner"]);
}

/// Preparing the same declaration twice has to land on the same identities, or
/// an unchanged plan would prepare to a new snapshot digest on every build and
/// expire its own authorization at dispatch.
#[test]
fn the_same_declaration_prepares_to_the_same_derived_identities() {
    let first = PreparedRunSnapshot::new(transient_corner_parts()).expect("first preparation");
    let second = PreparedRunSnapshot::new(transient_corner_parts()).expect("second preparation");

    assert_eq!(
        first
            .tasks
            .iter()
            .map(|task| task.instance_id)
            .collect::<Vec<_>>(),
        second
            .tasks
            .iter()
            .map(|task| task.instance_id)
            .collect::<Vec<_>>()
    );
}

#[test]
fn periodic_op_handoff_snapshot_preserves_source_basis_and_distinct_numerics() {
    use crate::simulation::execution::ExecutionArtifactEnvelope;
    use crate::simulation::plan::{
        AnalysisKind, AnalysisNumericOverride, NumericOverrideOption, QpssDraft,
        QuasiPeriodicAcDraft,
    };
    for hb in [false, true] {
        let source = "QP basis\nV1 out 0 1\nR1 out 0 1k\n.end\n";
        let make_numeric = |kind, value: &str| {
            let mut options = AnalysisNumericOverride::default();
            options
                .set_for_instance(kind, Default::default(), NumericOverrideOption::Gmin, value)
                .unwrap();
            Some(options)
        };
        let mut op = configured_op_task(crate::simulation::dialog::OpConfig {
            temperature_mode: crate::simulation::dialog::OpTemperatureMode::Explicit,
            temperature_celsius: 37.0,
            ..Default::default()
        });
        op.numeric_override = make_numeric(AnalysisKind::OperatingPoint, "1e-7");
        let mut op = prepared("op", "OP", op);
        op.executable_netlist_override = Some(source.into());
        let (qp_spec, producer_line, producer_kind) = if hb {
            let spec = crate::simulation::multi_run::AnalysisSpec::HarmonicBalance {
                tones: vec![crate::simulation::multi_run::HbToneSpec::new(1000.0, 3)],
                reltol: 1e-6,
                abstol: 1e-12,
                max_iterations: 40,
                damping: 1.0,
                min_damping: 0.01,
                oversample: 4,
                collocation_points: None,
                max_mixing_order: 3,
                use_krylov: false,
                gmres_restart: 12,
                source_stepping: false,
                use_exact_jacobian: true,
                verbose: false,
            };
            (spec, ".hb 1000".to_owned(), AnalysisKind::HarmonicBalance)
        } else {
            let spec = QpssDraft {
                tones: "1000,1414.2135623730951".into(),
                harmonics: "1,1".into(),
                dc_initialization: true,
                ..Default::default()
            }
            .to_spec()
            .unwrap();
            let line = spec.driven_qpss_config().unwrap().to_spice().unwrap();
            (spec, line, AnalysisKind::Qpss)
        };
        let qp_task = QueuedAnalysis {
            analysis_line: producer_line,
            spec: qp_spec,
            config: None,
            spec_options: Default::default(),
            numeric_override: make_numeric(producer_kind, "0"),
        };
        let mut qp = prepared_with(
            "qp",
            ObjectRevision::INITIAL,
            vec![op.instance_id],
            "QPSS",
            qp_task,
        );
        qp.set_dependency_bindings(vec![PreparedDependencyBinding::dc_operating_point_seed(
            op.instance_id,
            op.source_revision,
            op.config_digest,
        )]);
        let mut ac = prepared_with(
            "qpac",
            ObjectRevision::INITIAL,
            vec![qp.instance_id],
            "QPAC",
            QueuedAnalysis {
                spec: if hb {
                    AnalysisSpec::Pac
                } else {
                    QuasiPeriodicAcDraft::default().to_spec().unwrap()
                },
                config: None,
                spec_options: Default::default(),
                // The plan builder inherits this from the exact carrier.
                numeric_override: make_numeric(producer_kind, "0"),
                analysis_line: ".qpac".into(),
            },
        );
        let bind = if hb {
            PreparedDependencyBinding::hb_state
        } else {
            PreparedDependencyBinding::qpss_state
        };
        ac.set_dependency_bindings(vec![bind(
            qp.instance_id,
            qp.source_revision,
            qp.config_digest,
        )]);
        let mut inputs = parts();
        inputs.executable_netlist = source.replace("1k", "2k");
        inputs.tasks = vec![op, qp, ac];
        let snapshot = PreparedRunSnapshot::new(inputs).unwrap();
        let digest = snapshot.digest();
        let issuer = crate::simulation::execution::ExecutionPermitIssuer::default();
        let proof = issuer
            .issue(digest)
            .unwrap()
            .consume(digest, digest)
            .unwrap();
        let mut authorized = snapshot.authorize_dispatch(proof).unwrap();
        let op = authorized.tasks.pop_front().unwrap();
        let qp = authorized.tasks.pop_front().unwrap();
        let ac = authorized.tasks.pop_front().unwrap();
        assert_eq!(op.source_basis_digest, qp.source_basis_digest);
        assert_eq!(qp.source_basis_digest, ac.source_basis_digest);
        assert_eq!(op.source_basis_digest, crate::state::content_digest(source));
        assert_ne!(op.executable_netlist, qp.executable_netlist);
        assert!(ac.executable_netlist.contains("R1 out 0 1k"));
        let config = op.config().unwrap();
        let result = crate::simulation::EngineBridge::new()
            .run(config, &op.executable_netlist)
            .unwrap();
        let AnalysisConfig::DcOp(config) = config else {
            unreachable!()
        };
        let artifact = ExecutionArtifactEnvelope::from_dc_operating_point_result(
            digest,
            op.instance_id,
            op.source_revision,
            op.config_digest,
            op.source_basis_digest,
            config,
            &result,
        )
        .unwrap()
        .unwrap();
        let deck = qp.executable_netlist.clone();
        let resolved = qp
            .resolve_dependency_artifacts(&HashMap::from([(op.instance_id, artifact)]))
            .unwrap();
        resolved
            .dependencies
            .validate_source_basis(&deck, op.source_basis_digest)
            .unwrap();
        let (metadata, buffers) = resolved.dependencies.encode_transfer().unwrap();
        let restored = ResolvedExecutionDependencies::decode_transfer(&metadata, buffers).unwrap();
        restored
            .validate_source_basis(&deck, op.source_basis_digest)
            .unwrap();
        assert!(
            restored
                .validate_source_basis(&deck.replace("1k", "2k"), op.source_basis_digest)
                .is_err()
        );
    }
}
