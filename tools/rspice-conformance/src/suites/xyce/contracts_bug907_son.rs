use super::*;

const LABEL: &str = "BUG_907_SON nested library equivalence";
const FAMILY_DIRECTORY: &str = "Netlists/Certification_Tests/BUG_907_SON";
const OWNER_CONTRACT: &str = "bug907_nested_library_relational_wrapper_owner";
const REFERENCE_CONTRACT: &str = "bug907_flat_release_reference";

const MEMBERS: [&str; 4] = ["test2.cir", "test2_nolib.cir", "lib3.lib", "lib4.lib"];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Bug907SonRole {
    HierarchicalOwner,
    FlatReference,
}

impl Bug907SonRole {
    const ALL: [Self; 2] = [Self::HierarchicalOwner, Self::FlatReference];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let record = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL.into_iter().find(|role| role.record() == record)
    }

    fn path(self) -> &'static str {
        match self {
            Self::HierarchicalOwner => "Netlists/Certification_Tests/BUG_907_SON/test2.cir",
            Self::FlatReference => "Netlists/Certification_Tests/BUG_907_SON/test2_nolib.cir",
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::HierarchicalOwner => "netlists/certification_tests/bug_907_son/test2.cir",
            Self::FlatReference => "netlists/certification_tests/bug_907_son/test2_nolib.cir",
        }
    }

    pub(super) fn contract(self) -> &'static str {
        match self {
            Self::HierarchicalOwner => OWNER_CONTRACT,
            Self::FlatReference => REFERENCE_CONTRACT,
        }
    }
}

impl XyceTestRunner {
    fn bug907_member_source<'a>(
        members: &'a BTreeMap<String, Vec<u8>>,
        name: &str,
    ) -> Result<&'a str, String> {
        std::str::from_utf8(
            members
                .get(&name.to_ascii_lowercase())
                .ok_or_else(|| format!("{LABEL} authenticated member {name:?} is missing"))?,
        )
        .map_err(|error| format!("{LABEL} member {name:?} is not UTF-8: {error}"))
    }

    fn bug907_sealed_sources(
        &self,
        members: &BTreeMap<String, Vec<u8>>,
        role: Bug907SonRole,
    ) -> Result<SealedSourceBundle, String> {
        let path = |name: &str| self.root.join(FAMILY_DIRECTORY).join(name);
        let root_name = Path::new(role.path())
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| format!("{LABEL} role path is invalid"))?;
        let mut sources = vec![(
            path(root_name),
            Self::bug907_member_source(members, root_name)?.to_string(),
        )];
        let mut edges = Vec::new();
        if role == Bug907SonRole::HierarchicalOwner {
            sources.extend(
                ["lib3.lib", "lib4.lib"]
                    .into_iter()
                    .map(|name| {
                        Ok((
                            path(name),
                            Self::bug907_member_source(members, name)?.to_string(),
                        ))
                    })
                    .collect::<Result<Vec<_>, String>>()?,
            );
            edges.extend([
                SealedSourceEdge {
                    owner: path("test2.cir"),
                    requested_path: "lib3.lib".into(),
                    target: path("lib3.lib"),
                },
                SealedSourceEdge {
                    owner: path("lib3.lib"),
                    requested_path: "lib4.lib".into(),
                    target: path("lib4.lib"),
                },
                SealedSourceEdge {
                    owner: path("lib3.lib"),
                    requested_path: "lib3.lib".into(),
                    target: path("lib3.lib"),
                },
            ]);
        }
        SealedSourceBundle::try_new_with_edges(sources, edges)
            .map_err(|error| format!("{LABEL} sealed source closure is invalid: {error}"))
    }

    fn read_bug907_members(&self) -> Result<BTreeMap<String, Vec<u8>>, String> {
        MEMBERS
            .into_iter()
            .map(|name| {
                let path = self.root.join(FAMILY_DIRECTORY).join(name);
                let bytes = fs::read(&path)
                    .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
                Ok((name.to_ascii_lowercase(), bytes))
            })
            .collect()
    }

    fn bug907_nodes(actual: &[String], expected: &[&str]) -> bool {
        actual.len() == expected.len()
            && actual
                .iter()
                .zip(expected)
                .all(|(a, e)| a.eq_ignore_ascii_case(e))
    }

    fn validate_bug907_worker(
        &self,
        role: Bug907SonRole,
        source: &str,
        path: &Path,
        sealed_sources: SealedSourceBundle,
        abort: &dyn AbortSignal,
    ) -> Result<XyceStaticDcPlan, String> {
        let plan = self.static_dc_plan_for_source_with_sealed_sources_and_abort(
            path,
            source.to_string(),
            ExpressionDialect::Xyce,
            sealed_sources.clone(),
            abort,
        )?;
        if plan.deck_path != path
            || plan.execution_dir.is_some()
            || plan.sealed_sources.is_none()
            || plan.expression_dialect != ExpressionDialect::Xyce
            || plan.parameter_redefinition_policy != ParameterRedefinitionPolicy::UseLast
            || !plan.diagnostics.is_empty()
            || plan.print_format.is_some()
            || plan.dc_data.is_some()
            || !plan.steps.is_empty()
            || !plan.dc.source.eq_ignore_ascii_case("VG")
            || plan.dc.mode != DcSweepMode::Linear
            || plan.dc.start.to_bits() != 0.0f64.to_bits()
            || plan.dc.stop.to_bits() != 1.8f64.to_bits()
            || plan.dc.step.to_bits() != 0.01f64.to_bits()
            || plan.dc.sweep2.is_some()
            || !plan
                .print
                .probes
                .iter()
                .map(String::as_str)
                .eq(["i(vd)", "i(vf)"])
        {
            return Err(format!("{LABEL} {role:?} static plan changed: {plan:?}"));
        }
        let options = NetlistParseOptions {
            statistical_mode: StatisticalParamMode::Nominal,
            expression_dialect: ExpressionDialect::Xyce,
            parameter_redefinition_policy: ParameterRedefinitionPolicy::UseLast,
            parameter_redefinition_diagnostic_policy:
                rspice_core::netlist::ParameterRedefinitionDiagnosticPolicy::Silent,
            ..NetlistParseOptions::default()
        };
        let netlist = Netlist::parse_with_path_and_sealed_sources_and_options_and_abort(
            &plan.source,
            path,
            sealed_sources,
            options,
            abort,
        )
        .map_err(|error| format!("{LABEL} {role:?} Xyce parse failed: {error}"))?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} {role:?} preparation exceeded deadline"));
        }
        if netlist.params.get("X").map(Value::to_bits) != Some(1.0f64.to_bits())
            || netlist.params.get("X2").map(Value::to_bits) != Some((1.0f64 / 3.0).to_bits())
            || netlist.params.all_params().len() != 2
            || !netlist.params.all_parameter_expressions().is_empty()
        {
            return Err(format!(
                "{LABEL} {role:?} parameter snapshot changed: {:?}",
                netlist.params.all_params()
            ));
        }
        if netlist.elements.len() != 5
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || !netlist.models.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.diagnostics.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.initial_conditions.is_empty()
            || netlist.device_initial_conditions.is_some()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.params.all_global_expressions().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
        {
            return Err(format!("{LABEL} {role:?} typed envelope changed"));
        }
        let find = |name: &str| {
            let mut matches = netlist
                .elements
                .iter()
                .filter(|element| element.name.eq_ignore_ascii_case(name));
            let first = matches.next();
            if matches.next().is_some() {
                None
            } else {
                first
            }
        };
        let voltage = |name: &str, nodes: &[&str], value: Value| {
            find(name).is_some_and(|element|
            element.provenance == ElementProvenance::Authored && Self::bug907_nodes(&element.nodes, nodes)
                && matches!(&element.kind, ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(actual)) if actual.to_bits() == value.to_bits()))
        };
        let resistor = |name: &str, nodes: &[&str], value: Value| {
            find(name).is_some_and(|element|
            element.provenance == ElementProvenance::Authored && Self::bug907_nodes(&element.nodes, nodes)
                && matches!(&element.kind, ElementKind::Resistor { value: actual, value_expr: None, model: None, instance_params, deferred_params }
                    if actual.to_bits() == value.to_bits() && instance_params.is_empty() && deferred_params.is_empty()))
        };
        if !voltage("VD", &["D", "0"], 1.8)
            || !voltage("VG", &["G", "0"], 0.0)
            || !voltage("VF", &["F", "0"], 1.5)
            || !resistor("R1", &["G", "D"], 1_000.0)
            || !resistor("R2", &["G", "F"], 2_000.0)
        {
            return Err(format!(
                "{LABEL} {role:?} exact topology changed: {:?}",
                netlist.elements
            ));
        }
        if role == Bug907SonRole::HierarchicalOwner
            && (!source.contains(".param x2  = '(x)/3.0'")
                || !source.contains(".lib lib3.lib lib4_top"))
        {
            return Err(format!(
                "{LABEL} owner lost deferred selected-LIB representation"
            ));
        }
        if !matches!(&netlist.analyses[0], AnalysisCommand::Dc { source, start, stop, step, mode: DcSweepMode::Linear, sweep2: None, .. }
            if source.eq_ignore_ascii_case("VG") && start.to_bits() == 0.0f64.to_bits()
                && stop.to_bits() == 1.8f64.to_bits() && step.to_bits() == 0.01f64.to_bits())
        {
            return Err(format!("{LABEL} typed DC command changed"));
        }
        let request = &netlist.output_requests[0];
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Dc)
            || request.name.is_some()
            || !request.expressions.is_empty()
            || request.dependencies.len() != 2
            || request
                .dependencies
                .iter()
                .zip(["VD", "VF"])
                .any(|(dependency, name)| {
                    dependency.kind != OutputSymbolKind::Device
                        || !dependency.operator.eq_ignore_ascii_case("I")
                        || !dependency.symbol.eq_ignore_ascii_case(name)
                })
        {
            return Err(format!("{LABEL} {role:?} typed PRINT changed: {request:?}"));
        }
        Ok(plan)
    }

    fn validate_bug907_table(role: Bug907SonRole, table: &XycePrnTable) -> Result<(), String> {
        if table.columns != ["Index", "i(vd)", "i(vf)"] || table.rows.len() != 181 {
            return Err(format!(
                "{LABEL} {role:?} PRN shape changed: {:?}/{}",
                table.columns,
                table.rows.len()
            ));
        }
        for (index, row) in table.rows.iter().enumerate() {
            let sweep = index as Value * 0.01;
            if row.len() != 3
                || row.iter().any(|value| !value.is_finite())
                || row[0].to_bits() != (index as Value).to_bits()
                || (row[1] - (sweep - 1.8) / 1_000.0).abs() > 5e-13
                || (row[2] - (sweep - 1.5) / 2_000.0).abs() > 5e-13
            {
                return Err(format!(
                    "{LABEL} {role:?} row {index} violates analytic current gate: {row:?}"
                ));
            }
        }
        Ok(())
    }

    fn run_bug907_worker(
        &self,
        role: Bug907SonRole,
        plan: &XyceStaticDcPlan,
        abort: &DeadlineAbort,
    ) -> Result<XycePrnTable, String> {
        let (netlist, results) = self
            .run_static_dc_results_with_abort(plan, abort)
            .map_err(|error| format!("{LABEL} {role:?} failed: {error}"))?;
        let table = self.dc_results_to_prn_table(plan, &netlist, &results)?;
        Self::validate_bug907_table(role, &table)?;
        Ok(table)
    }

    fn validate_bug907_relation(flat: &XycePrnTable, owner: &XycePrnTable) -> Result<(), String> {
        Self::validate_bug907_table(Bug907SonRole::FlatReference, flat)?;
        Self::validate_bug907_table(Bug907SonRole::HierarchicalOwner, owner)?;
        let flat = Self::xyce_legacy_compact_prn_for_comparison(flat, &PrintDelimiter::Whitespace)?;
        let owner =
            Self::xyce_legacy_compact_prn_for_comparison(owner, &PrintDelimiter::Whitespace)?;
        if flat.as_bytes() != owner.as_bytes() {
            return Err(format!("{LABEL} Release wrapper byte diff failed"));
        }
        Ok(())
    }

    pub(super) fn validate_bug907_son_oracle(&self, start: Instant) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired before validation"));
        }
        let members = self.read_bug907_members()?;
        let mut plans = BTreeMap::new();
        for worker in Bug907SonRole::ALL {
            let name = Path::new(worker.path())
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| format!("{LABEL} lost {worker:?}"))?;
            let source = Self::bug907_member_source(&members, name)?;
            let sealed_sources = self.bug907_sealed_sources(&members, worker)?;
            plans.insert(
                worker,
                self.validate_bug907_worker(
                    worker,
                    source,
                    &self.root.join(worker.path()),
                    sealed_sources,
                    &abort,
                )?,
            );
        }
        if abort.is_aborted() {
            return Err(format!("{LABEL} validation exceeded deadline"));
        }
        let flat = self.run_bug907_worker(
            Bug907SonRole::FlatReference,
            plans.get(&Bug907SonRole::FlatReference).unwrap(),
            &abort,
        )?;
        let owner = self.run_bug907_worker(
            Bug907SonRole::HierarchicalOwner,
            plans.get(&Bug907SonRole::HierarchicalOwner).unwrap(),
            &abort,
        )?;
        Self::validate_bug907_relation(&flat, &owner)?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} execution exceeded deadline"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn canonical_runner() -> XyceTestRunner {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("tests/xyce");
        XyceTestRunner::new(root, XyceRunnerConfig::default())
    }

    fn copied_family() -> (tempfile::TempDir, XyceTestRunner) {
        let canonical = canonical_runner();
        let temporary = tempfile::tempdir().unwrap();
        let family = temporary.path().join(FAMILY_DIRECTORY);
        fs::create_dir_all(&family).unwrap();
        for name in MEMBERS {
            fs::copy(
                canonical.root.join(FAMILY_DIRECTORY).join(name),
                family.join(name),
            )
            .unwrap();
        }
        let runner = XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default());
        (temporary, runner)
    }

    #[test]
    fn bug907_xyce_file_parse_resolves_later_nested_library_parameter() {
        let runner = canonical_runner();
        let members = runner.read_bug907_members().unwrap();
        let abort = DeadlineAbort::new(Instant::now(), 30_000);
        for role in Bug907SonRole::ALL {
            let path = runner.root.join(role.path());
            let name = path.file_name().unwrap().to_str().unwrap();
            let source = XyceTestRunner::bug907_member_source(&members, name).unwrap();
            let sealed = runner.bug907_sealed_sources(&members, role).unwrap();
            runner
                .validate_bug907_worker(role, source, &path, sealed, &abort)
                .expect("sealed Xyce parse resolves X/X2 through selected nested LIB");
        }
    }

    #[test]
    fn bug907_nested_library_and_topology_counterfactuals_fail_closed() {
        let runner = canonical_runner();
        let path = runner.root.join(Bug907SonRole::HierarchicalOwner.path());
        let members = runner.read_bug907_members().unwrap();
        let source = XyceTestRunner::bug907_member_source(&members, "test2.cir").unwrap();
        for mutation in [
            source.replacen("lib4_top", "lib4_B", 1),
            source.replacen("'(x)/3.0'", "'(x)/4.0'", 1),
            source.replacen("r1 g d 1k", "r1 g d 2k", 1),
        ] {
            let mut mutated_members = members.clone();
            mutated_members.insert("test2.cir".into(), mutation.as_bytes().to_vec());
            let sealed = runner
                .bug907_sealed_sources(&mutated_members, Bug907SonRole::HierarchicalOwner)
                .unwrap();
            assert!(
                runner
                    .validate_bug907_worker(
                        Bug907SonRole::HierarchicalOwner,
                        &mutation,
                        &path,
                        sealed,
                        &DeadlineAbort::new(Instant::now(), 30_000),
                    )
                    .is_err()
            );
        }
    }

    #[test]
    fn bug907_exact_prn_relation_rejects_shared_wrong_and_byte_drift() {
        let rows = (0..=180)
            .map(|index| {
                let sweep = index as Value * 0.01;
                vec![
                    index as Value,
                    (sweep - 1.8) / 1_000.0,
                    (sweep - 1.5) / 2_000.0,
                ]
            })
            .collect::<Vec<_>>();
        let baseline = XycePrnTable {
            columns: vec!["Index".into(), "i(vd)".into(), "i(vf)".into()],
            rows,
        };
        XyceTestRunner::validate_bug907_relation(&baseline, &baseline).unwrap();
        let mut shared_wrong = baseline.clone();
        shared_wrong.rows[50][1] += 1e-5;
        assert!(XyceTestRunner::validate_bug907_relation(&shared_wrong, &shared_wrong).is_err());
        let mut byte_drift = baseline.clone();
        byte_drift.rows[180][1] += 1e-13;
        XyceTestRunner::validate_bug907_table(Bug907SonRole::HierarchicalOwner, &byte_drift)
            .expect("counterfactual remains inside the independent analytic tolerance");
        assert!(XyceTestRunner::validate_bug907_relation(&baseline, &byte_drift).is_err());
    }

    #[test]
    fn bug907_sealed_plan_never_executes_live_library_drift() {
        let (_temporary, runner) = copied_family();
        let family = runner.root.join(FAMILY_DIRECTORY);
        let members = runner.read_bug907_members().unwrap();
        let path = family.join("test2.cir");
        let source = XyceTestRunner::bug907_member_source(&members, "test2.cir").unwrap();
        let sealed = runner
            .bug907_sealed_sources(&members, Bug907SonRole::HierarchicalOwner)
            .unwrap();
        fs::write(
            family.join("lib4.lib"),
            ".lib lib4_A\nr2 g f 9k\n.endl lib4_A\n",
        )
        .unwrap();
        let start = Instant::now();
        let plan = runner
            .validate_bug907_worker(
                Bug907SonRole::HierarchicalOwner,
                source,
                &path,
                sealed,
                &DeadlineAbort::new(start, 30_000),
            )
            .expect("authenticated bundle, not changed live library, prepares the plan");
        runner
            .run_bug907_worker(
                Bug907SonRole::HierarchicalOwner,
                &plan,
                &DeadlineAbort::new(start, 30_000),
            )
            .expect("authenticated bundle, not changed live library, executes");
    }

    #[test]
    fn bug907_expired_deadline_aborts_sealed_preparation() {
        let runner = canonical_runner();
        let members = runner.read_bug907_members().unwrap();
        let path = runner.root.join(Bug907SonRole::HierarchicalOwner.path());
        let source = XyceTestRunner::bug907_member_source(&members, "test2.cir").unwrap();
        let sealed = runner
            .bug907_sealed_sources(&members, Bug907SonRole::HierarchicalOwner)
            .unwrap();
        let error = runner
            .validate_bug907_worker(
                Bug907SonRole::HierarchicalOwner,
                source,
                &path,
                sealed,
                &DeadlineAbort::new(Instant::now() - Duration::from_secs(1), 1),
            )
            .expect_err("expired shared deadline must abort during sealed preparation");
        assert!(
            error.contains("aborted"),
            "unexpected deadline error: {error}"
        );
    }

    #[test]
    fn bug907_executes_flat_then_nested_and_matches_exact_prn() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("tests/xyce");
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        runner.validate_bug907_son_oracle(Instant::now()).unwrap();
    }

    #[test]
    fn bug907_shared_deadline_fails_closed_before_execution() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("tests/xyce");
        let runner = XyceTestRunner::new(
            &root,
            XyceRunnerConfig {
                max_time_per_test_ms: 1,
                ..Default::default()
            },
        );
        assert!(
            runner
                .validate_bug907_son_oracle(Instant::now() - Duration::from_secs(1))
                .is_err()
        );
    }
}
