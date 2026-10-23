use super::*;

const LABEL: &str = "BUG_42_SON archived deprecated .LIB alias failure";
const FAMILY_DIRECTORY: &str = "Netlists/Certification_Tests/BUG_42_SON";
const SIDECAR_NAME: &str = "resistance.txt";
const OWNER_PATH: &str = "Netlists/Certification_Tests/BUG_42_SON/bug42.cir";
const OWNER_RECORD: &str = "netlists/certification_tests/bug_42_son/bug42.cir";
const CONTRACT: &str = "archived_expected_failure_bug42_deprecated_lib_alias";

const EXPECTED_PARSE_LINE: usize = 3;
const EXPECTED_PARSE_MESSAGE: &str = "Library section 'resistance.txt' missing .ENDL";
const UPSTREAM_DIAGNOSTIC: &str = "Netlist error: Could not find .ENDL statement for '.LIB RESISTANCE.TXT'.  Maybe '.LIB RESISTANCE.TXT <library_name>' or '.INC RESISTANCE.TXT' was intended.";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Bug42SonRole;

impl Bug42SonRole {
    pub(super) fn for_record(record: &str) -> Option<Self> {
        (XyceTestRunner::normalize_manifest_key(record) == OWNER_RECORD).then_some(Self)
    }

    pub(super) const fn contract(self) -> &'static str {
        CONTRACT
    }
}

#[derive(Debug, Clone, Copy)]
enum Bug42ParseMode {
    Filesystem,
    SealedRootOnly,
}

impl XyceTestRunner {
    fn validate_bug42_source_shape(source: &str) -> Result<(), String> {
        let lines = source.lines().collect::<Vec<_>>();
        if !source.ends_with('\n')
            || lines.len() != 9
            || lines[0] != "*Test to make sure that .lib is a synonym for .inc"
            || !lines[1].is_empty()
            || lines[2] != ".lib resistance.txt"
            || !lines[3].is_empty()
            || lines[4] != "V1 1 0 DC 1"
            || lines[5] != "R1 1 0 res"
            || !lines[6].is_empty()
            || lines[7] != ".DC V1 1 1 0.1"
            || lines[8] != ".END"
        {
            return Err(format!("{LABEL} exact nine-line source envelope changed"));
        }
        Ok(())
    }

    fn bug42_parse_options() -> NetlistParseOptions {
        NetlistParseOptions {
            statistical_mode: StatisticalParamMode::Nominal,
            expression_dialect: ExpressionDialect::Xyce,
            parameter_redefinition_policy: ParameterRedefinitionPolicy::UseLast,
            ..NetlistParseOptions::default()
        }
    }

    fn parse_bug42_expected_failure(
        source: &str,
        deck_path: &Path,
        mode: Bug42ParseMode,
        abort: &dyn AbortSignal,
    ) -> Result<(), String> {
        let parsed = match mode {
            Bug42ParseMode::Filesystem => Netlist::parse_with_path_and_options_and_abort(
                source,
                deck_path,
                Self::bug42_parse_options(),
                abort,
            ),
            Bug42ParseMode::SealedRootOnly => {
                let sealed =
                    SealedSourceBundle::try_new([(deck_path.to_path_buf(), source.to_string())])
                        .map_err(|error| format!("{LABEL} sealed root is invalid: {error}"))?;
                if sealed.len() != 1 {
                    return Err(format!("{LABEL} sealed-root census changed"));
                }
                Netlist::parse_with_path_and_sealed_sources_and_options_and_abort(
                    source,
                    deck_path,
                    sealed,
                    Self::bug42_parse_options(),
                    abort,
                )
            }
        };
        match parsed {
            Err(rspice_core::netlist::ParseWithAbortError::Parse(ParseError::Syntax {
                line,
                message,
            })) if line == EXPECTED_PARSE_LINE && message == EXPECTED_PARSE_MESSAGE => Ok(()),
            Err(rspice_core::netlist::ParseWithAbortError::Parse(error)) => Err(format!(
                "{LABEL} {mode:?} produced the wrong typed parse failure: {error:?}"
            )),
            Err(rspice_core::netlist::ParseWithAbortError::Aborted) => {
                Err(format!("{LABEL} {mode:?} parsing exceeded its deadline"))
            }
            Ok(_) => Err(format!("{LABEL} {mode:?} unexpectedly parsed successfully")),
        }
    }

    fn bug42_upstream_diagnostic(source: &str) -> Result<String, String> {
        let directive = source
            .lines()
            .nth(EXPECTED_PARSE_LINE - 1)
            .ok_or_else(|| format!("{LABEL} lost its .LIB line"))?;
        let tokens = directive.split_ascii_whitespace().collect::<Vec<_>>();
        if tokens.len() != 2 || !tokens[0].eq_ignore_ascii_case(".LIB") {
            return Err(format!("{LABEL} .LIB diagnostic source changed"));
        }
        let library = tokens[1].to_ascii_uppercase();
        Ok(format!(
            "Netlist error: Could not find .ENDL statement for '.LIB {library}'.  Maybe '.LIB {library} <library_name>' or '.INC {library}' was intended."
        ))
    }

    fn bug42_wrapper_exit_code(simulator_exit: i32) -> i32 {
        if simulator_exit == 0 { 0 } else { 10 }
    }

    fn bug42_positive_include_netlists(
        &self,
        source: &str,
        sidecar: &str,
        deck_path: &Path,
        abort: &dyn AbortSignal,
    ) -> Result<(Netlist, Netlist), String> {
        let include_source = source.replacen(".lib resistance.txt", ".inc resistance.txt", 1);
        if include_source == source || include_source.matches(".inc resistance.txt").count() != 1 {
            return Err(format!("{LABEL} positive-control rewrite changed"));
        }
        let filesystem = Netlist::parse_with_path_and_options_and_abort(
            &include_source,
            deck_path,
            Self::bug42_parse_options(),
            abort,
        )
        .map_err(|error| format!("{LABEL} filesystem .INC positive control failed: {error}"))?;
        let sidecar_path = deck_path
            .parent()
            .ok_or_else(|| format!("{LABEL} owner has no family directory"))?
            .join(SIDECAR_NAME);
        let sealed = SealedSourceBundle::try_new_with_edges(
            [
                (deck_path.to_path_buf(), include_source.clone()),
                (sidecar_path.clone(), sidecar.to_string()),
            ],
            [SealedSourceEdge {
                owner: deck_path.to_path_buf(),
                requested_path: SIDECAR_NAME.to_string(),
                target: sidecar_path,
            }],
        )
        .map_err(|error| format!("{LABEL} positive-control closure is invalid: {error}"))?;
        if sealed.len() != 2 {
            return Err(format!("{LABEL} positive-control source census changed"));
        }
        let sealed = Netlist::parse_with_path_and_sealed_sources_and_options_and_abort(
            &include_source,
            deck_path,
            sealed,
            Self::bug42_parse_options(),
            abort,
        )
        .map_err(|error| format!("{LABEL} sealed .INC positive control failed: {error}"))?;
        Ok((filesystem, sealed))
    }

    fn validate_bug42_positive_include(netlist: &Netlist) -> Result<(), String> {
        let resistance = netlist
            .params
            .get("RES")
            .ok_or_else(|| format!("{LABEL} .INC did not consume resistance.txt"))?;
        if resistance.to_bits() != 1_000.0f64.to_bits()
            || netlist.params.all_params().len() != 1
            || netlist.elements.len() != 2
            || netlist.analyses.len() != 1
            || !netlist.output_requests.is_empty()
            || !netlist.diagnostics.is_empty()
            || !netlist.models.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.data_tables.is_empty()
        {
            return Err(format!("{LABEL} .INC positive-control envelope changed"));
        }
        let voltage = &netlist.elements[0];
        if !voltage.name.eq_ignore_ascii_case("V1")
            || voltage.nodes != ["1", "0"]
            || !matches!(
                &voltage.kind,
                ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(value))
                    if value.to_bits() == 1.0f64.to_bits()
            )
        {
            return Err(format!("{LABEL} .INC voltage-source topology changed"));
        }
        let resistor = &netlist.elements[1];
        let ElementKind::Resistor {
            value,
            value_expr,
            model,
            instance_params,
            deferred_params,
        } = &resistor.kind
        else {
            return Err(format!("{LABEL} .INC R1 is no longer a resistor"));
        };
        if !resistor.name.eq_ignore_ascii_case("R1")
            || resistor.nodes != ["1", "0"]
            || value.to_bits() != resistance.to_bits()
            || value_expr.is_some()
            || model.is_some()
            || !instance_params.is_empty()
            || !deferred_params.is_empty()
        {
            return Err(format!("{LABEL} .INC resistor topology changed"));
        }
        let AnalysisCommand::Dc {
            source,
            start,
            stop,
            step,
            mode,
            sweep2,
            ..
        } = &netlist.analyses[0]
        else {
            return Err(format!("{LABEL} .INC lost its DC analysis"));
        };
        if !source.eq_ignore_ascii_case("V1")
            || start.to_bits() != 1.0f64.to_bits()
            || stop.to_bits() != 1.0f64.to_bits()
            || step.to_bits() != 0.1f64.to_bits()
            || !matches!(mode, DcSweepMode::Linear)
            || sweep2.is_some()
        {
            return Err(format!("{LABEL} .INC DC request changed"));
        }
        Ok(())
    }

    pub(super) fn validate_bug42_oracle(&self, start: Instant) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!(
                "{LABEL} aborted: deadline expired before execution"
            ));
        }
        let path = self.root.join(OWNER_PATH);
        let owner = Self::read_corpus_canonical(LABEL, &path)?;
        let source = std::str::from_utf8(&owner)
            .map_err(|error| format!("{LABEL} owner is not UTF-8: {error}"))?;
        let sidecar = Self::read_corpus_canonical(
            LABEL,
            &self.root.join(FAMILY_DIRECTORY).join(SIDECAR_NAME),
        )?;
        let sidecar = std::str::from_utf8(&sidecar)
            .map_err(|error| format!("{LABEL} sidecar is not UTF-8: {error}"))?;
        Self::validate_bug42_source_shape(source)?;
        Self::parse_bug42_expected_failure(source, &path, Bug42ParseMode::Filesystem, &abort)?;
        Self::parse_bug42_expected_failure(source, &path, Bug42ParseMode::SealedRootOnly, &abort)?;
        let diagnostic = Self::bug42_upstream_diagnostic(source)?;
        if diagnostic != UPSTREAM_DIAGNOSTIC || Self::bug42_wrapper_exit_code(1) != 10 {
            return Err(format!(
                "{LABEL} historical diagnostic/wrapper projection changed: {diagnostic:?}"
            ));
        }
        let (filesystem_positive, sealed_positive) =
            self.bug42_positive_include_netlists(source, sidecar, &path, &abort)?;
        Self::validate_bug42_positive_include(&filesystem_positive)?;
        Self::validate_bug42_positive_include(&sealed_positive)?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} execution exceeded its deadline"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn corpus_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("tests/xyce")
    }

    fn runner() -> XyceTestRunner {
        XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default())
    }

    #[test]
    fn bug42_route_is_exactly_owner_only() {
        assert_eq!(Bug42SonRole::for_record(OWNER_PATH), Some(Bug42SonRole));
        assert_eq!(Bug42SonRole::for_record(OWNER_RECORD), Some(Bug42SonRole));
        for rejected in [
            "Netlists/Certification_Tests/BUG_42_SON/resistance.txt",
            "Netlists/Certification_Tests/BUG_42_SON/bug42.cir.sh",
            "Netlists/Certification_Tests/BUG_42_SON/sibling.cir",
            "Netlists/Certification_Tests/BUG_42/bug42.cir",
            "OutputData/Certification_Tests/BUG_42_SON/bug42.cir.prn",
        ] {
            assert_eq!(
                Bug42SonRole::for_record(rejected),
                None,
                "accepted {rejected}"
            );
        }
    }

    #[test]
    fn bug42_checked_in_owner_has_exact_typed_failure_in_both_resolvers() {
        let runner = runner();
        let abort = DeadlineAbort::new(Instant::now(), 30_000);
        let path = runner.root.join(OWNER_PATH);
        let source =
            String::from_utf8(XyceTestRunner::read_corpus_canonical(LABEL, &path).unwrap())
                .unwrap();
        XyceTestRunner::validate_bug42_source_shape(&source).unwrap();
        XyceTestRunner::parse_bug42_expected_failure(
            &source,
            &path,
            Bug42ParseMode::Filesystem,
            &abort,
        )
        .unwrap();
        XyceTestRunner::parse_bug42_expected_failure(
            &source,
            &path,
            Bug42ParseMode::SealedRootOnly,
            &abort,
        )
        .unwrap();
        assert_eq!(
            XyceTestRunner::bug42_upstream_diagnostic(&source).unwrap(),
            UPSTREAM_DIAGNOSTIC
        );
    }

    #[test]
    fn bug42_inc_positive_control_consumes_exact_sidecar_and_topology() {
        let runner = runner();
        let abort = DeadlineAbort::new(Instant::now(), 30_000);
        let path = runner.root.join(OWNER_PATH);
        let canonical = |path: &Path| {
            String::from_utf8(XyceTestRunner::read_corpus_canonical(LABEL, path).unwrap()).unwrap()
        };
        let source = canonical(&path);
        let sidecar = canonical(&runner.root.join(FAMILY_DIRECTORY).join(SIDECAR_NAME));
        let (filesystem, sealed) = runner
            .bug42_positive_include_netlists(&source, &sidecar, &path, &abort)
            .unwrap();
        XyceTestRunner::validate_bug42_positive_include(&filesystem).unwrap();
        XyceTestRunner::validate_bug42_positive_include(&sealed).unwrap();

        let (filesystem, poisoned) = runner
            .bug42_positive_include_netlists(&source, ".param res = 2K\n", &path, &abort)
            .unwrap();
        XyceTestRunner::validate_bug42_positive_include(&filesystem).unwrap();
        assert!(XyceTestRunner::validate_bug42_positive_include(&poisoned).is_err());
    }

    #[test]
    fn bug42_stale_wrapper_exit_semantics_are_exact() {
        assert_eq!(XyceTestRunner::bug42_wrapper_exit_code(0), 0);
        for exit in [-9, -1, 1, 2, 10, 127] {
            assert_eq!(XyceTestRunner::bug42_wrapper_exit_code(exit), 10);
        }
    }

    #[test]
    fn bug42_full_oracle_observes_its_shared_deadline() {
        let runner = runner();
        let expired_ms = u64::try_from(runner.config.max_time_per_test_ms.saturating_add(1))
            .expect("BUG42 test deadline fits u64 milliseconds");
        let expired_start = Instant::now()
            .checked_sub(Duration::from_millis(expired_ms))
            .expect("construct expired BUG42 full-oracle deadline");
        let failure = runner
            .validate_bug42_oracle(expired_start)
            .expect_err("expired BUG42 full oracle must abort");
        assert!(failure.contains("aborted"), "{failure}");
    }

    #[test]
    fn bug42_counterfactuals_fail_closed() {
        let runner = runner();
        let abort = DeadlineAbort::new(Instant::now(), 30_000);
        let path = runner.root.join(OWNER_PATH);
        let source =
            String::from_utf8(XyceTestRunner::read_corpus_canonical(LABEL, &path).unwrap())
                .unwrap();

        let repaired = source.replace(".lib resistance.txt", ".inc resistance.txt");
        assert!(
            XyceTestRunner::parse_bug42_expected_failure(
                &repaired,
                &path,
                Bug42ParseMode::Filesystem,
                &abort,
            )
            .is_err()
        );
        let wrong_library = source.replace("resistance.txt", "other.txt");
        assert!(
            XyceTestRunner::parse_bug42_expected_failure(
                &wrong_library,
                &path,
                Bug42ParseMode::SealedRootOnly,
                &abort,
            )
            .is_err()
        );
        let relocated = format!(
            "{}\n{}",
            &source[..source.find(".lib").unwrap()],
            &source[source.find(".lib").unwrap()..]
        );
        assert!(
            XyceTestRunner::parse_bug42_expected_failure(
                &relocated,
                &path,
                Bug42ParseMode::SealedRootOnly,
                &abort,
            )
            .is_err()
        );
        let expired_start = Instant::now()
            .checked_sub(Duration::from_millis(2))
            .expect("construct expired BUG42 deadline");
        let expired = DeadlineAbort::new(expired_start, 1);
        let failure = XyceTestRunner::parse_bug42_expected_failure(
            &source,
            &path,
            Bug42ParseMode::Filesystem,
            &expired,
        )
        .expect_err("expired BUG42 parse must abort");
        assert!(failure.contains("deadline"), "{failure}");
    }

    #[test]
    fn bug42_checked_in_owner_executes_complete_oracle() {
        let runner = runner();
        let result = runner.run_test(runner.root.join(OWNER_PATH));
        assert!(result.passed, "BUG42 failed: {:?}", result.error);
        assert_eq!(result.contract, CONTRACT);
        assert!(!result.expected_unsupported);
        assert!(!result.upstream_excluded);
    }
}
