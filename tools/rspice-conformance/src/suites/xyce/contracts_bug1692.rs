use super::*;

const LABEL: &str = "BUG_1692 expression-equivalence relational family";

pub(super) const BUG1692_OWNER_CONTRACT: &str = "bug1692_expression_equivalence_wrapper_owner";
pub(super) const BUG1692_WORKER_CONTRACT: &str = "bug1692_expression_equivalence_worker";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Bug1692Role {
    WrapperOwner,
    Braced,
    Naked,
    Quoted,
    Parameterized,
}

impl Bug1692Role {
    pub(super) const ALL: [Self; 5] = [
        Self::WrapperOwner,
        Self::Braced,
        Self::Naked,
        Self::Quoted,
        Self::Parameterized,
    ];
    const WORKERS: [Self; 4] = [Self::Braced, Self::Naked, Self::Quoted, Self::Parameterized];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let record = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL.into_iter().find(|role| role.record() == record)
    }

    pub(super) const fn contract(self) -> &'static str {
        match self {
            Self::WrapperOwner => BUG1692_OWNER_CONTRACT,
            _ => BUG1692_WORKER_CONTRACT,
        }
    }

    pub(super) const fn path(self) -> &'static str {
        match self {
            Self::WrapperOwner => "Netlists/Certification_Tests/BUG_1692/bug_1692.cir",
            Self::Braced => "Netlists/Certification_Tests/BUG_1692/bug_1692_0.cir",
            Self::Naked => "Netlists/Certification_Tests/BUG_1692/bug_1692_1.cir",
            Self::Quoted => "Netlists/Certification_Tests/BUG_1692/bug_1692_2.cir",
            Self::Parameterized => "Netlists/Certification_Tests/BUG_1692/bug_1692_3.cir",
        }
    }

    const fn file_name(self) -> &'static str {
        match self {
            Self::WrapperOwner => "bug_1692.cir",
            Self::Braced => "bug_1692_0.cir",
            Self::Naked => "bug_1692_1.cir",
            Self::Quoted => "bug_1692_2.cir",
            Self::Parameterized => "bug_1692_3.cir",
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::WrapperOwner => "netlists/certification_tests/bug_1692/bug_1692.cir",
            Self::Braced => "netlists/certification_tests/bug_1692/bug_1692_0.cir",
            Self::Naked => "netlists/certification_tests/bug_1692/bug_1692_1.cir",
            Self::Quoted => "netlists/certification_tests/bug_1692/bug_1692_2.cir",
            Self::Parameterized => "netlists/certification_tests/bug_1692/bug_1692_3.cir",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Bug1692Geometry {
    name: String,
    nodes: Vec<String>,
    model: String,
    values: BTreeMap<String, u64>,
}

impl XyceTestRunner {
    fn validate_bug1692_unique_effective_param(
        params: &[(String, Value)],
        model_name: &str,
        param_name: &str,
        expected: Value,
    ) -> Result<(), String> {
        let authored = params
            .iter()
            .filter(|(candidate, _)| candidate.eq_ignore_ascii_case(param_name))
            .map(|(_, value)| *value)
            .collect::<Vec<_>>();
        if authored.len() != 1 {
            return Err(format!(
                "{LABEL} {model_name} must contain exactly one {param_name}; found {}",
                authored.len()
            ));
        }
        let effective = Self::numeric_param_value(params, param_name)
            .ok_or_else(|| format!("{LABEL} {model_name} lost effective {param_name} semantics"))?;
        if !authored[0].is_finite()
            || authored[0].to_bits() != expected.to_bits()
            || effective.to_bits() != expected.to_bits()
        {
            return Err(format!(
                "{LABEL} {model_name} effective {param_name} must remain {expected}; authored={:?}, effective={effective}",
                authored[0]
            ));
        }
        Ok(())
    }

    fn validate_bug1692_model(
        netlist: &Netlist,
        name: &str,
        model_type: &str,
    ) -> Result<(), String> {
        let model = netlist
            .models
            .iter()
            .find(|model| model.name.eq_ignore_ascii_case(name))
            .ok_or_else(|| format!("{LABEL} lost {name} model"))?;
        if !model.model_type.eq_ignore_ascii_case(model_type) {
            return Err(format!(
                "{LABEL} {name} must remain native BSIMSOI LEVEL=10/SOIMOD=0 (B3SOIPD route)"
            ));
        }
        Self::validate_bug1692_unique_effective_param(&model.params, name, "LEVEL", 10.0)?;
        Self::validate_bug1692_unique_effective_param(&model.params, name, "SOIMOD", 0.0)?;
        if !Self::model_is_native_b3soi_mosfet(model, &[]) {
            return Err(format!(
                "{LABEL} {name} effective model semantics no longer select native BSIMSOI"
            ));
        }
        Ok(())
    }

    fn validate_bug1692_assembled_native_route(
        &self,
        netlist: &Netlist,
        source_name: &str,
    ) -> Result<(), String> {
        let circuit = self
            .create_xyce_engine()
            .build_circuit(netlist)
            .map_err(|error| format!("{LABEL} {source_name} circuit assembly failed: {error}"))?;
        if circuit.has_generated_veriloga_devices() {
            return Err(format!(
                "{LABEL} {source_name} assembled through a generated Verilog-A device"
            ));
        }
        let report = circuit.device_op_report();
        let observed = report
            .entries
            .iter()
            .map(|entry| format!("{}:{}", entry.name, entry.device_kind))
            .collect::<Vec<_>>();
        let expected = ["x1.mn1", "x1.mp1"];
        if report.entries.len() != expected.len() {
            return Err(format!(
                "{LABEL} {source_name} must assemble exactly two semiconductor devices; observed {observed:?}"
            ));
        }
        for expected_name in expected {
            let matches = report
                .entries
                .iter()
                .filter(|entry| Self::device_instance_names_match(&entry.name, expected_name))
                .collect::<Vec<_>>();
            if matches.len() != 1 || !matches[0].device_kind.eq_ignore_ascii_case("B3SOIPD") {
                return Err(format!(
                    "{LABEL} {source_name} {expected_name} must assemble uniquely through native B3SOIPD; observed {observed:?}"
                ));
            }
        }
        Ok(())
    }

    fn bug1692_geometry(netlist: &Netlist) -> Result<Vec<Bug1692Geometry>, String> {
        let flattened = flatten_netlist_with_models(netlist)
            .map_err(|error| format!("{LABEL} hierarchy no longer flattens: {error}"))?;
        if !flattened.scoped_models.is_empty()
            || !flattened.scoped_initial_conditions.is_empty()
            || !flattened.scoped_node_sets.is_empty()
            || !flattened.scoped_startup_directives.is_empty()
            || !flattened.xspice_auto_bridge_node_hints.is_empty()
        {
            return Err(format!(
                "{LABEL} acquired scoped/generated flattening state"
            ));
        }
        let mut geometry = Vec::new();
        for element in &flattened.elements {
            let ElementKind::Mosfet {
                model,
                compact_syntax,
                instance_params,
                deferred_params,
                ..
            } = &element.kind
            else {
                continue;
            };
            if *compact_syntax
                || !deferred_params.is_empty()
                || element.provenance != ElementProvenance::Authored
                || instance_params.len() != 7
            {
                return Err(format!("{LABEL} flattened MOS shape changed: {element:?}"));
            }
            let values = instance_params
                .iter()
                .map(|(name, value)| (name.to_ascii_uppercase(), value.to_bits()))
                .collect::<BTreeMap<_, _>>();
            if values.len() != 7
                || !["L", "W", "M", "AD", "AS", "PD", "PS"]
                    .into_iter()
                    .all(|name| values.contains_key(name))
            {
                return Err(format!("{LABEL} flattened MOS geometry keys changed"));
            }
            geometry.push(Bug1692Geometry {
                name: element.name.to_ascii_lowercase(),
                nodes: element
                    .nodes
                    .iter()
                    .map(|node| node.to_ascii_lowercase())
                    .collect(),
                model: model.to_ascii_lowercase(),
                values,
            });
        }
        geometry.sort_by(|left, right| left.name.cmp(&right.name));
        if geometry.len() != 2 {
            return Err(format!(
                "{LABEL} must flatten to exactly two inverter MOSFETs"
            ));
        }
        Ok(geometry)
    }

    fn validate_bug1692_geometry(geometry: &[Bug1692Geometry]) -> Result<(), String> {
        let expect = |actual: &Bug1692Geometry,
                      name: &str,
                      nodes: &[&str],
                      model: &str,
                      l: Value,
                      w: Value,
                      ad: Value,
                      perimeter: Value|
         -> Result<(), String> {
            let expected = BTreeMap::from([
                ("L".to_string(), l.to_bits()),
                ("W".to_string(), w.to_bits()),
                ("M".to_string(), 1.0f64.to_bits()),
                ("AD".to_string(), ad.to_bits()),
                ("AS".to_string(), ad.to_bits()),
                ("PD".to_string(), perimeter.to_bits()),
                ("PS".to_string(), perimeter.to_bits()),
            ]);
            if actual.name != name
                || actual
                    .nodes
                    .iter()
                    .map(String::as_str)
                    .ne(nodes.iter().copied())
                || actual.model != model
                || actual.values != expected
            {
                return Err(format!(
                    "{LABEL} flattened {name} geometry changed: {actual:?}"
                ));
            }
            Ok(())
        };
        expect(
            &geometry[0],
            "x1.mn1",
            &["out1", "in1", "vss1", "0", "nbulk1"],
            "mn",
            0.35e-6,
            2.0e-6,
            0.95e-6 * 2.0e-6,
            2.0 * (0.95e-6 + 2.0e-6),
        )?;
        expect(
            &geometry[1],
            "x1.mp1",
            &["out1", "in1", "vdd1", "0", "pbulk1"],
            "mp",
            0.45e-6,
            4.0e-6,
            0.95e-6 * 4.0e-6,
            2.0 * (0.95e-6 + 4.0e-6),
        )
    }

    fn validate_bug1692_worker(
        &self,
        role: Bug1692Role,
        source: &str,
        path: &Path,
    ) -> Result<(XyceStaticTranPlan, Vec<Bug1692Geometry>), String> {
        if role == Bug1692Role::WrapperOwner {
            return Err(format!("{LABEL} owner is not an executable worker"));
        }
        if source
            .lines()
            .any(|line| line.trim_start().to_ascii_uppercase().starts_with("*COMP"))
        {
            return Err(format!(
                "{LABEL} {} acquired a *COMP override",
                role.file_name()
            ));
        }
        let plan = self.static_tran_plan_for_path_with_purpose(
            path,
            XyceStaticTranPlanPurpose::RelationalFamily,
        )?;
        if plan.deck_path != path
            || plan.source.as_bytes() != source.as_bytes()
            || !matches!(plan.oracle, XyceStaticTranOracle::None)
            || plan.contract != XyceStaticTranContract::PlainStatic
            || plan.comparison_mode != XyceStaticTranComparisonMode::Pointwise
            || plan.output_override
            || plan.timeint_conststep
            || !plan.steps.is_empty()
            || plan.wrapper_tolerance.is_some()
            || plan.tran.step.to_bits() != 20.0e-9f64.to_bits()
            || plan.tran.stop.to_bits() != 30.0e-6f64.to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
            || plan.print.as_ref().is_none_or(|print| {
                print
                    .probes
                    .iter()
                    .map(|probe| Self::normalize_probe(probe))
                    .ne(["v(in1)".to_string(), "v(out1)".to_string()])
            })
        {
            return Err(format!(
                "{LABEL} {} plan changed: {plan:?}",
                role.file_name()
            ));
        }
        let netlist = Self::parse_xyce_netlist(source, path)
            .map_err(|error| format!("{LABEL} {} no longer parses: {error}", role.file_name()))?;
        if !netlist.diagnostics.is_empty()
            || netlist.elements.len() != 8
            || netlist.models.len() != 2
            || netlist.subcircuits.len() != 1
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.initial_conditions.is_empty()
            || netlist.device_initial_conditions.is_some()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !matches!(&netlist.analyses[0], AnalysisCommand::Tran {
                step, stop, start: None, max_step: None, uic: false,
            } if step.to_bits() == 20.0e-9f64.to_bits()
                && stop.to_bits() == 30.0e-6f64.to_bits())
        {
            return Err(format!(
                "{LABEL} {} typed envelope changed",
                role.file_name()
            ));
        }
        Self::validate_bug1692_model(&netlist, "MN", "NMOS")?;
        Self::validate_bug1692_model(&netlist, "MP", "PMOS")?;
        let request = &netlist.output_requests[0];
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Tran)
            || request.name.is_some()
            || request.print_delimiter.as_ref() != Some(&PrintDelimiter::Whitespace)
            || !request.expressions.is_empty()
            || request.dependencies.len() != 2
            || request
                .dependencies
                .iter()
                .zip(["in1", "out1"])
                .any(|(dependency, node)| {
                    dependency.kind != OutputSymbolKind::Node
                        || dependency.expression
                        || !dependency.operator.eq_ignore_ascii_case("V")
                        || !dependency.symbol.eq_ignore_ascii_case(node)
                })
        {
            return Err(format!(
                "{LABEL} {} typed PRINT changed: {request:?}",
                role.file_name()
            ));
        }
        let geometry = Self::bug1692_geometry(&netlist)?;
        Self::validate_bug1692_geometry(&geometry)?;
        for mos in &geometry {
            if !Self::netlist_device_is_native_b3soi_mosfet(&netlist, &mos.name) {
                return Err(format!("{LABEL} {} lost native B3SOIPD routing", mos.name));
            }
        }
        self.validate_bug1692_assembled_native_route(&netlist, role.file_name())?;
        Ok((plan, geometry))
    }

    fn validate_bug1692_table(role: Bug1692Role, table: &XycePrnTable) -> Result<(), String> {
        let expected = ["Index", "TIME", "V(in1)", "V(out1)"];
        if table.columns.len() != expected.len()
            || table
                .columns
                .iter()
                .zip(expected)
                .any(|(actual, expected)| !actual.eq_ignore_ascii_case(expected))
            || table.rows.len() < 2
        {
            return Err(format!(
                "{LABEL} {} output schema changed: columns={:?}, rows={}",
                role.file_name(),
                table.columns,
                table.rows.len()
            ));
        }
        let mut previous_time = None;
        for (index, row) in table.rows.iter().enumerate() {
            if row.len() != expected.len()
                || row.iter().any(|value| !value.is_finite())
                || row[0].to_bits() != (index as Value).to_bits()
                || previous_time.is_some_and(|previous| row[1] <= previous)
            {
                return Err(format!(
                    "{LABEL} {} malformed row {index}",
                    role.file_name()
                ));
            }
            previous_time = Some(row[1]);
        }
        let first = table.rows.first().expect("checked nonempty");
        let last = table.rows.last().expect("checked nonempty");
        let range = |column: usize| {
            let minimum = table
                .rows
                .iter()
                .map(|row| row[column])
                .fold(Value::INFINITY, Value::min);
            let maximum = table
                .rows
                .iter()
                .map(|row| row[column])
                .fold(Value::NEG_INFINITY, Value::max);
            maximum - minimum
        };
        if first[1].abs() > 1.0e-18
            || (last[1] - 30.0e-6).abs() > 1.0e-14
            || range(2) < 1.0e-3
            || range(3) < 1.0e-3
        {
            return Err(format!(
                "{LABEL} {} output domain became trivial",
                role.file_name()
            ));
        }
        Ok(())
    }

    fn compare_bug1692_relation(
        &self,
        worker: Bug1692Role,
        good: &XycePrnTable,
        test: &XycePrnTable,
    ) -> Result<(), String> {
        let exact_diagnostic = match self.compare_serialized_default_prn_tables(good, test) {
            Ok(mismatches) if mismatches.is_empty() => return Ok(()),
            Ok(mismatches) => format!("{mismatches:?}"),
            Err(error) => format!("comparator error: {error}"),
        };
        let fallback = self.compare_xyce_verify_transient_tables_with_uniform_tolerance(
            good,
            test,
            XyceVerifyTransientTolerance::release_7_10_default(),
            XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
        )
        .map_err(|error| {
            format!(
                "{LABEL} {} exact PRN differed ({exact_diagnostic}) and directional Release-7.10 xyce_verify errored: {error}",
                worker.file_name()
            )
        })?;
        if fallback.is_empty() {
            Ok(())
        } else {
            Err(format!(
                "{LABEL} {} failed both exact PRN and directional Release-7.10 xyce_verify: exact={exact_diagnostic}, fallback={fallback:?}",
                worker.file_name()
            ))
        }
    }

    pub(super) fn validate_bug1692_oracle(&self, start: Instant) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired before execution"));
        }
        let mut baseline_geometry = None;
        let mut tables = BTreeMap::new();
        for worker in Bug1692Role::WORKERS {
            let path = self.root.join(worker.path());
            let source = fs::read_to_string(&path)
                .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
            let (plan, geometry) = self.validate_bug1692_worker(worker, &source, &path)?;
            if let Some(baseline) = &baseline_geometry {
                if baseline != &geometry {
                    return Err(format!(
                        "{LABEL} {} no longer flattens to baseline geometry",
                        worker.file_name()
                    ));
                }
            } else {
                baseline_geometry = Some(geometry);
            }
            let (netlist, result) = self
                .run_transient_family_plan(&plan, start, None, None)
                .map_err(|error| match error {
                    SimulationError::Aborted => format!("{LABEL} execution exceeded deadline"),
                    other => format!("{LABEL} {} execution failed: {other}", worker.file_name()),
                })?;
            let executed_geometry = Self::bug1692_geometry(&netlist)?;
            if !netlist.veriloga_includes.is_empty()
                || baseline_geometry.as_ref() != Some(&executed_geometry)
            {
                return Err(format!(
                    "{LABEL} {} execution geometry changed",
                    worker.file_name()
                ));
            }
            let table = Self::transient_family_result_to_prn_table(&plan, &netlist, &result)
                .map_err(|error| {
                    format!(
                        "{LABEL} {} PRN rendering failed: {error}",
                        worker.file_name()
                    )
                })?;
            Self::validate_bug1692_table(worker, &table)?;
            tables.insert(worker, table);
            if abort.is_aborted() {
                return Err(format!("{LABEL} deadline expired between independent runs"));
            }
        }

        let good = tables.get(&Bug1692Role::Braced).expect("baseline ran");
        for worker in [
            Bug1692Role::Naked,
            Bug1692Role::Quoted,
            Bug1692Role::Parameterized,
        ] {
            let test = tables.get(&worker).expect("worker ran");
            self.compare_bug1692_relation(worker, good, test)?;
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

    #[test]
    fn bug1692_workers_have_exact_typed_flattened_geometry_and_native_route() {
        let runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        let mut baseline = None;
        for role in Bug1692Role::WORKERS {
            let path = runner.root.join(role.path());
            let source = fs::read_to_string(&path).expect("read BUG1692 worker");
            let (_, geometry) = runner
                .validate_bug1692_worker(role, &source, &path)
                .unwrap_or_else(|error| panic!("{role:?}: {error}"));
            if let Some(expected) = &baseline {
                assert_eq!(expected, &geometry);
            } else {
                baseline = Some(geometry);
            }
        }
    }

    #[test]
    fn bug1692_model_selector_repetition_and_parameter_duplicates_are_distinguished() {
        let path = corpus_root().join(Bug1692Role::Braced.path());
        let source = fs::read_to_string(&path).expect("read BUG1692 worker");
        let repeated_selector = source.replacen("LEVEL   = 10", "LEVEL   = 10 LEVEL = 10", 1);
        assert_ne!(repeated_selector, source, "fixture contains LEVEL");
        let netlist = XyceTestRunner::parse_xyce_netlist(&repeated_selector, &path)
            .expect("Xyce permits repeated LEVEL model selectors");
        let error = XyceTestRunner::validate_bug1692_model(&netlist, "MP", "PMOS")
            .expect_err("the sealed BUG1692 worker still rejects source drift");
        assert!(
            error.contains("exactly one LEVEL"),
            "unexpected LEVEL drift diagnostic: {error}"
        );

        let duplicate_parameter = source.replacen("SOIMOD  = 0", "SOIMOD  = 0 SOIMOD = 0", 1);
        assert_ne!(duplicate_parameter, source, "fixture contains SOIMOD");
        let options = NetlistParseOptions {
            statistical_mode: StatisticalParamMode::Nominal,
            expression_dialect: ExpressionDialect::Xyce,
            parameter_redefinition_policy: ParameterRedefinitionPolicy::UseLast,
            ..NetlistParseOptions::default()
        };
        let error = Netlist::parse_with_path_and_options(&duplicate_parameter, &path, options)
            .expect_err("duplicate Xyce device parameter must fail during parsing");
        let ParseError::DuplicateModelParameter(error) = error else {
            panic!("unexpected SOIMOD duplicate error: {error:?}");
        };
        assert_eq!(error.model_name, "MP");
        assert_eq!(error.canonical_model_name, "MP");
        assert_eq!(error.parameter_name, "SOIMOD");
        assert_eq!(error.canonical_parameter_name, "SOIMOD");
        assert_eq!(error.model_origin.line, 32);
        assert!(
            error
                .model_origin
                .path
                .as_deref()
                .is_some_and(|origin| XyceTestRunner::same_path(origin, &path))
        );
    }

    #[test]
    fn bug1692_effective_soimod_and_assembled_route_must_remain_pd() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let path = root.join(Bug1692Role::Braced.path());
        let source = fs::read_to_string(&path).expect("read BUG1692 worker");
        let non_pd = source.replace("SOIMOD  = 0", "SOIMOD  = 1");
        assert_eq!(non_pd.matches("SOIMOD  = 1").count(), 2);
        let netlist = XyceTestRunner::parse_xyce_netlist(&non_pd, &path)
            .expect("parse non-PD BUG1692 worker");
        for (model_name, model_type) in [("MN", "NMOS"), ("MP", "PMOS")] {
            let error = XyceTestRunner::validate_bug1692_model(&netlist, model_name, model_type)
                .expect_err("effective SOIMOD=1 must fail closed");
            assert!(
                error.contains("effective SOIMOD must remain 0"),
                "unexpected effective-semantics diagnostic: {error}"
            );
        }
        let error = runner
            .validate_bug1692_assembled_native_route(&netlist, "SOIMOD=1 fixture")
            .expect_err("assembled B3SOIDD route must fail closed");
        assert!(
            error.contains("B3SOIPD"),
            "unexpected route diagnostic: {error}"
        );
        assert!(
            error.contains("B3SOIDD"),
            "route proof was not assembled: {error}"
        );
    }

    #[test]
    fn bug1692_oracle_executes_all_four_representations() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        runner
            .validate_bug1692_oracle(Instant::now())
            .expect("execute BUG1692 relation");
    }

    #[test]
    fn bug1692_oracle_rejects_expired_deadline() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let expired = Instant::now()
            - Duration::from_millis(
                u64::try_from(runner.config.max_time_per_test_ms.max(1) + 1)
                    .expect("timeout fits u64"),
            );
        assert!(runner.validate_bug1692_oracle(expired).is_err());
    }

    #[test]
    fn bug1692_directional_xyce_verify_fallback_interpolates_good_to_test_grid() {
        let runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        let columns = ["Index", "TIME", "V(in1)", "V(out1)"]
            .map(str::to_string)
            .to_vec();
        let good = XycePrnTable {
            columns: columns.clone(),
            rows: vec![
                vec![0.0, 0.0, 0.0, 4.0],
                vec![1.0, 1.0, 2.0, 2.0],
                vec![2.0, 2.0, 4.0, 0.0],
            ],
        };
        let test = XycePrnTable {
            columns,
            rows: vec![
                vec![0.0, 0.0, 0.0, 4.0],
                vec![1.0, 0.5, 1.0, 3.0],
                vec![2.0, 1.0, 2.0, 2.0],
                vec![3.0, 1.5, 3.0, 1.0],
                vec![4.0, 2.0, 4.0, 0.0],
            ],
        };
        runner
            .compare_bug1692_relation(Bug1692Role::Naked, &good, &test)
            .expect("historical directional fallback accepts equivalent test grid");
        let mut wrong = test;
        wrong.rows[2][3] = 8.0;
        assert!(
            runner
                .compare_bug1692_relation(Bug1692Role::Naked, &good, &wrong)
                .is_err()
        );
    }
}
