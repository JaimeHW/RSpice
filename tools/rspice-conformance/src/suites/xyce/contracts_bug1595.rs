#[cfg(test)]
mod tests {
    use super::super::*;

    fn corpus_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("tests/xyce")
    }

    fn canonical_deck(root: &Path) -> XyceDeck {
        XyceDeck {
            path: root.join(XYCE_BUG1595_EXPECTED_FAILURE_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG1595_EXPECTED_FAILURE_PATH.to_string(),
        }
    }

    #[test]
    fn bug1595_canonical_oracle_and_expired_deadline_are_fail_closed() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let deck = canonical_deck(&root);
        runner
            .validate_expected_failure_oracle(
                &deck,
                XyceExpectedFailureKind::Bug1595HierarchicalMutualInductorReference,
                Instant::now(),
            )
            .expect("canonical BUG1595 typed failure qualifies");
        let error = runner
            .validate_expected_failure_oracle(
                &deck,
                XyceExpectedFailureKind::Bug1595HierarchicalMutualInductorReference,
                Instant::now() - Duration::from_secs(31),
            )
            .expect_err("expired BUG1595 deadline must fail closed");
        assert!(error.contains("bounded"), "unexpected error: {error}");
    }

    #[test]
    fn bug1595_typed_observer_rejects_identity_and_origin_mutations() {
        use rspice_core::abort_signal::ImmediateAbort;

        let path = corpus_root().join(XYCE_BUG1595_EXPECTED_FAILURE_PATH);
        let source = fs::read_to_string(&path).expect("read canonical BUG1595 source");
        let observation =
            XyceTestRunner::observe_bug1595_hierarchical_mutual_inductor_reference_failure(
                &source,
                &path,
                &rspice_core::abort_signal::NoAbort,
            )
            .expect("canonical BUG1595 typed payload is observed");
        assert_eq!(
            observation,
            XyceExpectedFailureKind::Bug1595HierarchicalMutualInductorReference
                .expected_observation()
        );

        let renamed = source.replace("K1 X1:L1 X1:L2 0.75", "K2 X1:L1 X1:L2 0.75");
        assert!(
            XyceTestRunner::observe_bug1595_hierarchical_mutual_inductor_reference_failure(
                &renamed,
                &path,
                &rspice_core::abort_signal::NoAbort,
            )
            .is_err(),
            "a renamed coupling must not satisfy the typed payload"
        );

        let shifted = source.replacen("\nK1 X1:L1", "\n\nK1 X1:L1", 1);
        assert!(
            XyceTestRunner::observe_bug1595_hierarchical_mutual_inductor_reference_failure(
                &shifted,
                &path,
                &rspice_core::abort_signal::NoAbort,
            )
            .is_err(),
            "a shifted physical origin must not satisfy the typed payload"
        );

        for mutation in [
            source.replace("K1 X1:L1 X1:L2 0.75", "K1 L1 L2 0.75"),
            source.replace("K1 X1:L1 X1:L2 0.75", "K1 X1:L2 X1:L1 0.75"),
            source.replace("K1 X1:L1 X1:L2 0.75", "K1 x1:l1 X1:L2 0.75"),
        ] {
            assert!(
                XyceTestRunner::observe_bug1595_hierarchical_mutual_inductor_reference_failure(
                    &mutation,
                    &path,
                    &rspice_core::abort_signal::NoAbort,
                )
                .is_err(),
                "corrected, reordered, and recased references must fail closed"
            );
        }

        let wrong_path = path.with_file_name("renamed.cir");
        assert!(
            XyceTestRunner::observe_bug1595_hierarchical_mutual_inductor_reference_failure(
                &source,
                &wrong_path,
                &rspice_core::abort_signal::NoAbort,
            )
            .is_err(),
            "the observer must bind the authored filename"
        );
        let aborted =
            XyceTestRunner::observe_bug1595_hierarchical_mutual_inductor_reference_failure(
                &source,
                &path,
                &ImmediateAbort,
            )
            .expect_err("the typed parse must honor cooperative cancellation");
        assert!(
            aborted.contains("bounded"),
            "unexpected abort error: {aborted}"
        );
    }
}
