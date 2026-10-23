use super::*;

const LABEL: &str = "BUG_411 duplicate model-parameter error wrapper";
const OWNER_PATH: &str = "Netlists/Certification_Tests/BUG_411/perl-generated.cir";
const OWNER_RECORD: &str = "netlists/certification_tests/bug_411/perl-generated.cir";
const CONTRACT: &str = "bug411_duplicate_model_parameter_wrapper_owner";

const UPSTREAM_DIAGNOSTIC_PREFIX: &str = "in file perl-generated.cir at or near line 2195";
const UPSTREAM_DIAGNOSTIC: &str =
    "Device model CD4012_PMOS: Duplicate specification of parameter NFS";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Bug411Role;

impl Bug411Role {
    pub(super) fn for_record(record: &str) -> Option<Self> {
        (XyceTestRunner::normalize_manifest_key(record) == OWNER_RECORD).then_some(Self)
    }

    pub(super) const fn contract(self) -> &'static str {
        CONTRACT
    }
}

impl XyceTestRunner {
    fn bug411_device_card_count(source: &str, prefix: u8) -> usize {
        source
            .lines()
            .filter_map(|line| line.trim_start().split_ascii_whitespace().next())
            .filter(|token| {
                token
                    .as_bytes()
                    .first()
                    .is_some_and(|byte| byte.eq_ignore_ascii_case(&prefix))
                    && !token.starts_with('.')
                    && !token.starts_with('*')
            })
            .count()
    }

    fn validate_bug411_source_shape(source: &str) -> Result<(), String> {
        let lines = source.lines().collect::<Vec<_>>();
        if !source.ends_with('\n')
            || lines.len() != 2_214
            || lines.first().copied() != Some("discrete81.esp Boolean Network Circuit")
            || lines
                .last()
                .is_none_or(|line| !line.trim().eq_ignore_ascii_case(".END"))
            || lines.get(1_792).map(|line| line.trim()) != Some(".TRAN .1s 20s")
            || lines.get(2_194).map(|line| line.trim()) != Some(".MODEL cd4012_pmos PMOS (")
            || lines.get(2_202).map(|line| line.trim()) != Some(".MODEL cd4012_nmos NMOS (")
        {
            return Err(format!("{LABEL} physical source envelope changed"));
        }

        let logical = Self::logical_netlist_lines(source);
        let directive_count = |name: &str| {
            logical
                .iter()
                .filter(|line| {
                    line.split_ascii_whitespace()
                        .next()
                        .is_some_and(|token| token.eq_ignore_ascii_case(name))
                })
                .count()
        };
        let subckts = directive_count(".SUBCKT");
        let ends = directive_count(".ENDS");
        let models = directive_count(".MODEL");
        let trans = directive_count(".TRAN");
        let prints = logical
            .iter()
            .filter(|line| {
                line.split_ascii_whitespace()
                    .next()
                    .is_some_and(|token| token.eq_ignore_ascii_case(".PRINT"))
            })
            .collect::<Vec<_>>();
        if subckts != 25
            || ends != 25
            || models != 2
            || trans != 1
            || prints.len() != 1
            || Self::bug411_device_card_count(source, b'X') != 664
            || Self::bug411_device_card_count(source, b'R') != 593
            || Self::bug411_device_card_count(source, b'M') != 20
        {
            return Err(format!(
                "{LABEL} structural census changed: subckt={subckts}, ends={ends}, models={models}, tran={trans}, print={}, X={}, R={}, M={}",
                prints.len(),
                Self::bug411_device_card_count(source, b'X'),
                Self::bug411_device_card_count(source, b'R'),
                Self::bug411_device_card_count(source, b'M'),
            ));
        }
        let print_tokens = prints[0].split_ascii_whitespace().collect::<Vec<_>>();
        if print_tokens.len() != 83
            || !print_tokens[0].eq_ignore_ascii_case(".PRINT")
            || !print_tokens[1].eq_ignore_ascii_case("TRAN")
            || !(0..=80)
                .all(|index| print_tokens[index + 2].eq_ignore_ascii_case(&format!("V(n{index})")))
        {
            return Err(format!("{LABEL} V(n0)..V(n80) output request changed"));
        }
        if logical.iter().any(|line| {
            let directive = line.split_ascii_whitespace().next().unwrap_or_default();
            directive.eq_ignore_ascii_case(".INC")
                || directive.eq_ignore_ascii_case(".INCLUDE")
                || directive.eq_ignore_ascii_case(".LIB")
                || directive.eq_ignore_ascii_case(".MEASURE")
                || directive.eq_ignore_ascii_case(".MEAS")
        }) {
            return Err(format!(
                "{LABEL} acquired external input or invented numerical oracle"
            ));
        }
        Ok(())
    }

    fn parse_bug411_duplicate_model_error(
        source: &str,
        deck_path: &Path,
        abort: &dyn AbortSignal,
    ) -> Result<rspice_core::netlist::DuplicateModelParameterError, String> {
        let options = NetlistParseOptions {
            statistical_mode: StatisticalParamMode::Nominal,
            expression_dialect: ExpressionDialect::Xyce,
            parameter_redefinition_policy: ParameterRedefinitionPolicy::UseLast,
            ..NetlistParseOptions::default()
        };
        match Netlist::parse_with_path_and_options_and_abort(source, deck_path, options, abort) {
            Err(rspice_core::netlist::ParseWithAbortError::Parse(
                ParseError::DuplicateModelParameter(error),
            )) => Ok(*error),
            Err(rspice_core::netlist::ParseWithAbortError::Parse(error)) => Err(format!(
                "{LABEL} produced the wrong parse failure: {error:?}"
            )),
            Err(rspice_core::netlist::ParseWithAbortError::Aborted) => {
                Err(format!("{LABEL} parsing exceeded its bounded contract"))
            }
            Ok(_) => Err(format!("{LABEL} unexpectedly parsed successfully")),
        }
    }

    pub(super) fn observe_bug411_failure(
        source: &str,
        deck_path: &Path,
        abort: &dyn AbortSignal,
    ) -> Result<rspice_core::netlist::DuplicateModelParameterError, String> {
        let error = Self::parse_bug411_duplicate_model_error(source, deck_path, abort)?;
        if error.model_name != "CD4012_PMOS"
            || error.canonical_model_name != "CD4012_PMOS"
            || error.parameter_name != "NFS"
            || error.canonical_parameter_name != "NFS"
            || error.model_origin.line != 2_195
            || error
                .model_origin
                .path
                .as_deref()
                .is_none_or(|path| !Self::same_path(path, deck_path))
        {
            return Err(format!("{LABEL} typed failure identity changed: {error:?}"));
        }
        let file_name = error
            .model_origin
            .path
            .as_deref()
            .and_then(Path::file_name)
            .and_then(|name| name.to_str())
            .ok_or_else(|| format!("{LABEL} diagnostic lost its UTF-8 file name"))?;
        let prefix = format!(
            "in file {file_name} at or near line {}",
            error.model_origin.line
        );
        let diagnostic = format!(
            "Device model {}: Duplicate specification of parameter {}",
            error.canonical_model_name, error.canonical_parameter_name
        );
        if prefix != UPSTREAM_DIAGNOSTIC_PREFIX || diagnostic != UPSTREAM_DIAGNOSTIC {
            return Err(format!(
                "{LABEL} wrapper diagnostic projection changed: {prefix:?} / {diagnostic:?}"
            ));
        }
        let projected = format!("Netlist error {prefix}\n{diagnostic}\n");
        if Self::bug411_wrapper_exit_code(1, "", &projected) != 0 {
            return Err(format!(
                "{LABEL} no longer satisfies the historical wrapper"
            ));
        }
        Ok(error)
    }

    // The historical Perl wrapper used regexes. Its only metacharacter is the
    // dot in `perl-generated.cir`, so this bounded matcher reproduces Perl's
    // one-character (non-newline) dot semantics without adding a runtime regex
    // dependency to the conformance leaf crate.
    fn bug411_regex_find(stream: &str, pattern: &str) -> Option<usize> {
        let stream = stream.as_bytes();
        let pattern = pattern.as_bytes();
        if pattern.len() > stream.len() {
            return None;
        }
        (0..=stream.len() - pattern.len()).find(|start| {
            stream[*start..*start + pattern.len()]
                .iter()
                .zip(pattern)
                .all(|(actual, expected)| {
                    (*expected == b'.' && *actual != b'\n') || actual == expected
                })
        })
    }

    fn bug411_wrapper_stream_matches(stream: &str) -> bool {
        let Some(first) = Self::bug411_regex_find(stream, UPSTREAM_DIAGNOSTIC_PREFIX) else {
            return false;
        };
        let tail = &stream[first + UPSTREAM_DIAGNOSTIC_PREFIX.len()..];
        Self::bug411_regex_find(tail, UPSTREAM_DIAGNOSTIC).is_some()
    }

    fn bug411_wrapper_exit_code(simulator_exit: i32, stdout: &str, stderr: &str) -> i32 {
        if simulator_exit == 0 {
            2
        } else if Self::bug411_wrapper_stream_matches(stderr)
            || Self::bug411_wrapper_stream_matches(stdout)
        {
            0
        } else {
            10
        }
    }

    pub(super) fn validate_bug411_oracle(&self, start: Instant) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.clamp(1, 30_000));
        if abort.is_aborted() {
            return Err(format!(
                "{LABEL} aborted: deadline expired before execution"
            ));
        }
        let path = self.root.join(OWNER_PATH);
        let owner = Self::read_corpus_canonical(LABEL, &path)?;
        let source = std::str::from_utf8(&owner)
            .map_err(|error| format!("{LABEL} owner is not UTF-8: {error}"))?;
        Self::validate_bug411_source_shape(source)?;
        Self::observe_bug411_failure(source, &path, &abort)?;
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

    fn owner_source() -> String {
        fs::read_to_string(corpus_root().join(OWNER_PATH))
            .expect("read BUG411 owner")
            .replace("\r\n", "\n")
    }

    #[test]
    fn bug411_route_is_owner_only() {
        assert_eq!(Bug411Role::for_record(OWNER_PATH), Some(Bug411Role));
        assert_eq!(Bug411Role::for_record(OWNER_RECORD), Some(Bug411Role));
        for rejected in [
            "Netlists/Certification_Tests/BUG_411/README",
            "Netlists/Certification_Tests/BUG_411/options",
            "Netlists/Certification_Tests/BUG_411/perl-generated.cir.sh",
            "Netlists/Certification_Tests/BUG_411/sibling.cir",
            "OutputData/Certification_Tests/BUG_411/perl-generated.cir.prn",
        ] {
            assert_eq!(
                Bug411Role::for_record(rejected),
                None,
                "accepted {rejected}"
            );
        }
    }

    #[test]
    fn bug411_checked_in_owner_has_exact_shape_and_typed_failure() {
        let source = owner_source();
        let path = corpus_root().join(OWNER_PATH);
        XyceTestRunner::validate_bug411_source_shape(&source).unwrap();
        let abort = DeadlineAbort::new(Instant::now(), 30_000);
        let error = XyceTestRunner::observe_bug411_failure(&source, &path, &abort).unwrap();
        assert_eq!(error.model_origin.line, 2_195);
        assert_eq!(error.canonical_model_name, "CD4012_PMOS");
        assert_eq!(error.canonical_parameter_name, "NFS");

        let expired_start = Instant::now()
            .checked_sub(Duration::from_millis(2))
            .expect("construct expired BUG411 deadline");
        let expired = DeadlineAbort::new(expired_start, 1);
        let failure = XyceTestRunner::parse_bug411_duplicate_model_error(&source, &path, &expired)
            .expect_err("expired BUG411 observation must abort");
        assert!(failure.contains("bounded contract"), "{failure}");
    }

    #[test]
    fn bug411_full_oracle_enforces_historical_thirty_second_deadline() {
        let runner = runner();
        assert!(runner.config.max_time_per_test_ms > 30_000);
        let expired_start = Instant::now()
            .checked_sub(Duration::from_secs(31))
            .expect("construct expired BUG411 oracle deadline");
        let failure = runner
            .validate_bug411_oracle(expired_start)
            .expect_err("BUG411 must retain its historical 30-second timelimit");
        assert!(failure.contains("aborted"), "{failure}");
    }

    #[test]
    fn bug411_first_error_order_and_parser_precedence_fail_closed() {
        let source = owner_source();
        let path = corpus_root().join(OWNER_PATH);
        let abort = DeadlineAbort::new(Instant::now(), 30_000);

        let lowercase = source.replacen("NFS=1E10", "nfs=1E10", 1);
        let error =
            XyceTestRunner::parse_bug411_duplicate_model_error(&lowercase, &path, &abort).unwrap();
        assert_eq!(error.parameter_name, "nfs");
        assert_eq!(error.canonical_parameter_name, "NFS");
        assert_eq!(error.model_origin.line, 2_195);

        let repaired_first = source.replacen("NFS=1E10", "        ", 1);
        let error =
            XyceTestRunner::parse_bug411_duplicate_model_error(&repaired_first, &path, &abort)
                .unwrap();
        assert_eq!(error.canonical_model_name, "CD4012_NMOS");
        assert_eq!(error.canonical_parameter_name, "VMAX");
        assert_eq!(error.model_origin.line, 2_203);

        let earlier = source.replacen("UO = 310", "UO = 310 UO=311", 1);
        let error =
            XyceTestRunner::parse_bug411_duplicate_model_error(&earlier, &path, &abort).unwrap();
        assert_eq!(error.canonical_parameter_name, "UO");
        assert_eq!(error.model_origin.line, 2_195);

        let malformed = source.replacen("NFS=1E10", "NFS=", 1);
        let options = NetlistParseOptions {
            expression_dialect: ExpressionDialect::Xyce,
            ..NetlistParseOptions::default()
        };
        let failure =
            Netlist::parse_with_path_and_options_and_abort(&malformed, &path, options, &abort);
        let Err(rspice_core::netlist::ParseWithAbortError::Parse(ParseError::Syntax {
            line,
            message,
        })) = failure
        else {
            panic!("malformed BUG411 duplicate produced {failure:?}");
        };
        assert_eq!(line, 2_195);
        assert!(!message.contains("Duplicate specification"), "{message}");
    }

    #[test]
    fn bug411_historical_wrapper_semantics_are_exact() {
        let canonical = format!(
            "prefix\n{UPSTREAM_DIAGNOSTIC_PREFIX}\nintervening diagnostic\n{UPSTREAM_DIAGNOSTIC}\ntrailer\n"
        );
        assert_eq!(
            XyceTestRunner::bug411_wrapper_exit_code(1, "", &canonical),
            0
        );
        assert_eq!(
            XyceTestRunner::bug411_wrapper_exit_code(1, &canonical, ""),
            0
        );
        assert_eq!(
            XyceTestRunner::bug411_wrapper_exit_code(0, "", &canonical),
            2
        );

        let wildcard = canonical.replace("perl-generated.cir", "perl-generatedXcir");
        assert_eq!(
            XyceTestRunner::bug411_wrapper_exit_code(1, "", &wildcard),
            0
        );
        let reversed = format!("{UPSTREAM_DIAGNOSTIC}\n{UPSTREAM_DIAGNOSTIC_PREFIX}\n");
        assert_eq!(
            XyceTestRunner::bug411_wrapper_exit_code(1, "", &reversed),
            10
        );
        assert_eq!(
            XyceTestRunner::bug411_wrapper_exit_code(
                1,
                UPSTREAM_DIAGNOSTIC_PREFIX,
                UPSTREAM_DIAGNOSTIC,
            ),
            10
        );
        assert_eq!(
            XyceTestRunner::bug411_wrapper_exit_code(1, "", "unrelated"),
            10
        );
    }

    #[test]
    fn bug411_checked_in_owner_executes_complete_oracle() {
        let runner = runner();
        let result = runner.run_test(runner.root.join(OWNER_PATH));
        assert!(result.passed, "BUG411 failed: {:?}", result.error);
        assert_eq!(result.contract, CONTRACT);
    }
}
