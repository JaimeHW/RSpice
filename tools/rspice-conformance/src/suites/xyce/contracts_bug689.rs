use super::*;
use rspice_core::netlist::SourceSpec;

const LABEL: &str = "BUG_689 DOS/Unix line-ending diode relation";
const OWNER_PATH: &str = "Netlists/Certification_Tests/BUG_689/dos-diode.cir";
const REFERENCE_PATH: &str = "Netlists/Certification_Tests/BUG_689/unix-diode.cir";
const OWNER_RECORD: &str = "netlists/certification_tests/bug_689/dos-diode.cir";
const REFERENCE_RECORD: &str = "netlists/certification_tests/bug_689/unix-diode.cir";
const OWNER_CONTRACT: &str = "bug689_crlf_line_endings_wrapper_owner";
const REFERENCE_CONTRACT: &str = "bug689_lf_line_endings_reference";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Bug689Role {
    CrLfOwner,
    LfReference,
}

impl Bug689Role {
    const ALL: [Self; 2] = [Self::CrLfOwner, Self::LfReference];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let normalized = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL
            .into_iter()
            .find(|role| role.record() == normalized)
    }

    pub(super) fn contract(self) -> &'static str {
        match self {
            Self::CrLfOwner => OWNER_CONTRACT,
            Self::LfReference => REFERENCE_CONTRACT,
        }
    }

    fn path(self) -> &'static str {
        match self {
            Self::CrLfOwner => OWNER_PATH,
            Self::LfReference => REFERENCE_PATH,
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::CrLfOwner => OWNER_RECORD,
            Self::LfReference => REFERENCE_RECORD,
        }
    }

    fn file_name(self) -> &'static str {
        self.path().rsplit('/').next().expect("BUG689 file name")
    }

    fn historical_identity(self) -> (usize, &'static str, &'static str) {
        match self {
            Self::CrLfOwner => (
                1_399,
                "7ce387d21f72b26b05c3ed74dc22a2349e1d1f7981bb4104204ed07abbb3383f",
                "f1d5bb9ffa4f30f4c8ff9025e510d2ad2fdef94f5d90d2fb4042707bbb82ddcd",
            ),
            Self::LfReference => (
                1_361,
                "b040d8b0cb496e1d07e45deeb9958e39cc24433542b11bbb67afa03616a31b40",
                "bb5e3cce9512b3f5f99f7dda52e96407d6fa22be596e9b1d2d36ca4ad19d9e44",
            ),
        }
    }
}

impl XyceTestRunner {
    fn historical_bug689_source(role: Bug689Role, canonical_lf: &[u8]) -> Result<String, String> {
        if canonical_lf.contains(&b'\r') || canonical_lf.last() != Some(&b'\n') {
            return Err(format!("{LABEL} canonical source is not LF text"));
        }
        let bytes = match role {
            Bug689Role::CrLfOwner => {
                let mut bytes = Vec::with_capacity(canonical_lf.len() + 64);
                for &byte in canonical_lf {
                    if byte == b'\n' {
                        bytes.push(b'\r');
                    }
                    bytes.push(byte);
                }
                bytes
            }
            Bug689Role::LfReference => canonical_lf.to_vec(),
        };
        let (expected_bytes, expected_sha256, expected_blake3) = role.historical_identity();
        if bytes.len() != expected_bytes
            || format!("{:x}", Sha256::digest(&bytes)) != expected_sha256
            || blake3::hash(&bytes).to_hex().as_str() != expected_blake3
        {
            return Err(format!(
                "{LABEL} {} historical byte representation changed",
                role.file_name()
            ));
        }
        String::from_utf8(bytes)
            .map_err(|error| format!("{LABEL} {} is not UTF-8: {error}", role.file_name()))
    }

    fn validate_bug689_plan(role: Bug689Role, plan: &XyceStaticDcPlan) -> Result<(), String> {
        let probes = plan
            .print
            .probes
            .iter()
            .map(|probe| Self::normalize_probe(probe))
            .collect::<Vec<_>>();
        if plan.execution_dir.is_some()
            || plan.dc_data.is_some()
            || plan.print_format.is_some()
            || !plan.steps.is_empty()
            || plan.dc.sweep2.is_some()
            || !matches!(plan.dc.mode, DcSweepMode::Linear)
            || !plan.dc.source.eq_ignore_ascii_case("VIN")
            || plan.dc.start.to_bits() != 5.0f64.to_bits()
            || plan.dc.stop.to_bits() != 5.0f64.to_bits()
            || plan.dc.step.to_bits() != 1.0f64.to_bits()
            || probes != ["i(vmon)", "v(3)"]
            || !plan.diagnostics.is_empty()
        {
            return Err(format!(
                "{LABEL} {} DC plan changed: {plan:?}",
                role.file_name()
            ));
        }
        Ok(())
    }

    fn validate_bug689_subcircuit(subcircuit: &SubcircuitDef) -> Result<(), String> {
        if !subcircuit.name.eq_ignore_ascii_case("dSUB")
            || !Self::nodes_match(&subcircuit.ports, &["a", "b"])
            || subcircuit.elements.len() != 1
            || !subcircuit.initial_conditions.is_empty()
            || !subcircuit.node_sets.is_empty()
            || !subcircuit.params.is_empty()
            || !subcircuit.expr_params.is_empty()
            || !subcircuit.string_params.is_empty()
            || !subcircuit.body_params.is_empty()
            || !subcircuit.body_expr_params.is_empty()
            || !subcircuit.body_string_params.is_empty()
            || !subcircuit.body_functions.is_empty()
            || !subcircuit.local_options.is_empty()
            || subcircuit.library_ref.is_some()
            || !subcircuit.nested_subcircuits.is_empty()
        {
            return Err(format!("{LABEL} dSUB definition changed: {subcircuit:?}"));
        }
        let diode = &subcircuit.elements[0];
        if diode.provenance != ElementProvenance::Authored
            || !diode.name.eq_ignore_ascii_case("D1")
            || !Self::nodes_match(&diode.nodes, &["a", "b"])
            || !matches!(&diode.kind, ElementKind::Diode { model, instance_params, deferred_params }
                if model.eq_ignore_ascii_case("DMOD")
                    && instance_params.is_empty()
                    && deferred_params.is_empty())
        {
            return Err(format!("{LABEL} subcircuit diode changed: {diode:?}"));
        }
        Ok(())
    }

    fn validate_bug689_netlist(role: Bug689Role, netlist: &Netlist) -> Result<(), String> {
        if netlist.elements.len() != 4
            || netlist.models.len() != 1
            || netlist.subcircuits.len() != 1
            || netlist.output_requests.len() != 1
            || netlist.lin_analysis.is_some()
            || !netlist.fft_analyses.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.initial_conditions.is_empty()
            || netlist.device_initial_conditions.is_some()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.diagnostics.is_empty()
            || !netlist.params.all_params().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
            || !matches!(netlist.analyses.as_slice(), [AnalysisCommand::Dc {
                source,
                start,
                stop,
                step,
                mode: DcSweepMode::Linear,
                sweep2: None,
                ..
            }] if source.eq_ignore_ascii_case("VIN")
                && start.to_bits() == 5.0f64.to_bits()
                && stop.to_bits() == 5.0f64.to_bits()
                && step.to_bits() == 1.0f64.to_bits())
        {
            return Err(format!(
                "{LABEL} {} typed envelope changed",
                role.file_name()
            ));
        }
        let model = &netlist.models[0];
        if !model.name.eq_ignore_ascii_case("DMOD")
            || !model.model_type.eq_ignore_ascii_case("D")
            || !matches!(model.params.as_slice(), [(name, value)]
                if name.eq_ignore_ascii_case("IS")
                    && value.to_bits() == 1.0e-13f64.to_bits())
            || !model.expr_params.is_empty()
            || !model.string_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
        {
            return Err(format!("{LABEL} DMOD model changed: {model:?}"));
        }
        let elements = netlist
            .elements
            .iter()
            .map(|element| (element.name.to_ascii_lowercase(), element))
            .collect::<BTreeMap<_, _>>();
        if elements.keys().map(String::as_str).collect::<BTreeSet<_>>()
            != BTreeSet::from(["vin", "r1", "xdos", "vmon"])
        {
            return Err(format!("{LABEL} top-level element inventory changed"));
        }
        let vin = elements["vin"];
        if vin.provenance != ElementProvenance::Authored
            || !Self::nodes_match(&vin.nodes, &["1", "0"])
            || !matches!(vin.kind, ElementKind::VoltageSource(SourceSpec::Dc(value))
                if value.to_bits() == 5.0f64.to_bits())
        {
            return Err(format!("{LABEL} VIN changed: {vin:?}"));
        }
        let resistor = elements["r1"];
        if resistor.provenance != ElementProvenance::Authored
            || !Self::nodes_match(&resistor.nodes, &["1", "2"])
            || !matches!(&resistor.kind, ElementKind::Resistor {
                value,
                value_expr: None,
                model: None,
                instance_params,
                deferred_params,
            } if value.to_bits() == 2.0e3f64.to_bits()
                && instance_params.is_empty()
                && deferred_params.is_empty())
        {
            return Err(format!("{LABEL} R1 changed: {resistor:?}"));
        }
        let instance = elements["xdos"];
        if instance.provenance != ElementProvenance::Authored
            || !Self::nodes_match(&instance.nodes, &["3", "0"])
            || !matches!(&instance.kind, ElementKind::Subcircuit { subckt_name, params }
                if subckt_name.eq_ignore_ascii_case("dSUB") && params.is_empty())
        {
            return Err(format!("{LABEL} xDOS changed: {instance:?}"));
        }
        let monitor = elements["vmon"];
        if monitor.provenance != ElementProvenance::Authored
            || !Self::nodes_match(&monitor.nodes, &["2", "3"])
            || !matches!(monitor.kind, ElementKind::VoltageSource(SourceSpec::Dc(value))
                if value.to_bits() == 0.0f64.to_bits())
        {
            return Err(format!("{LABEL} VMON changed: {monitor:?}"));
        }
        Self::validate_bug689_subcircuit(&netlist.subcircuits[0])?;

        let request = &netlist.output_requests[0];
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Dc)
            || request.name.is_some()
            || request.print_delimiter != Some(PrintDelimiter::Whitespace)
            || !request.expressions.is_empty()
            || request.dependencies.len() != 2
            || request.dependencies[0].kind != OutputSymbolKind::Device
            || !request.dependencies[0].operator.eq_ignore_ascii_case("I")
            || !request.dependencies[0].symbol.eq_ignore_ascii_case("VMON")
            || request.dependencies[0].expression
            || request.dependencies[1].kind != OutputSymbolKind::Node
            || !request.dependencies[1].operator.eq_ignore_ascii_case("V")
            || !request.dependencies[1].symbol.eq_ignore_ascii_case("3")
            || request.dependencies[1].expression
            || !matches!(netlist.saves.signals.as_slice(),
                [rspice_core::netlist::SaveSignal::Current(device),
                 rspice_core::netlist::SaveSignal::Voltage(node)]
                    if device.eq_ignore_ascii_case("VMON") && node == "3")
        {
            return Err(format!("{LABEL} typed .PRINT request changed: {request:?}"));
        }
        Ok(())
    }

    fn bug689_analytic_operating_point() -> (Value, Value) {
        const VIN: Value = 5.0;
        const RESISTANCE: Value = 2.0e3;
        const IS: Value = 1.0e-13;
        const TEMPERATURE_K: Value = 300.15;
        const XYCE_K_OVER_Q: Value = 1.3806226e-23 / 1.6021918e-19;
        let thermal_voltage = XYCE_K_OVER_Q * TEMPERATURE_K;
        let mut lower = 0.0;
        let mut upper = VIN;
        for _ in 0..128 {
            let voltage = 0.5 * (lower + upper);
            let current = IS * (voltage / thermal_voltage).exp_m1();
            if voltage + RESISTANCE * current < VIN {
                lower = voltage;
            } else {
                upper = voltage;
            }
        }
        let voltage = 0.5 * (lower + upper);
        (voltage, (VIN - voltage) / RESISTANCE)
    }

    fn validate_bug689_table(role: Bug689Role, table: &XycePrnTable) -> Result<(), String> {
        const COLUMNS: [&str; 3] = ["Index", "I(VMON)", "V(3)"];
        if table.columns.len() != COLUMNS.len()
            || table
                .columns
                .iter()
                .zip(COLUMNS)
                .any(|(actual, expected)| !actual.eq_ignore_ascii_case(expected))
            || table.rows.len() != 1
        {
            return Err(format!(
                "{LABEL} {} table shape changed: {table:?}",
                role.file_name()
            ));
        }
        let row = &table.rows[0];
        if row.len() != COLUMNS.len()
            || row.iter().any(|value| !value.is_finite())
            || row[0].to_bits() != 0.0f64.to_bits()
        {
            return Err(format!("{LABEL} {} row is malformed", role.file_name()));
        }
        let actual_current = row[1];
        let actual_voltage = row[2];
        let (expected_voltage, expected_current) = Self::bug689_analytic_operating_point();
        let kvl_current = (5.0 - actual_voltage) / 2.0e3;
        if (actual_voltage - expected_voltage).abs() > 5.0e-5
            || (actual_current - expected_current).abs() > 5.0e-8
            || (actual_current - kvl_current).abs() > 2.0e-9
            || actual_current <= 2.0e-3
            || !(0.6..0.63).contains(&actual_voltage)
        {
            return Err(format!(
                "{LABEL} {} violated the diode/load-line oracle: actual I={actual_current}, V={actual_voltage}; expected I={expected_current}, V={expected_voltage}",
                role.file_name()
            ));
        }
        Ok(())
    }

    fn bug689_plan(
        &self,
        role: Bug689Role,
        canonical_lf: &[u8],
    ) -> Result<XyceStaticDcPlan, String> {
        let path = self.root.join(role.path());
        let mut plan = self.static_dc_plan_for_path(&path, ExpressionDialect::Xyce)?;
        let planned_canonical = Self::canonical_lf_text_identity(LABEL, plan.source.as_bytes())?;
        if planned_canonical != canonical_lf {
            return Err(format!(
                "{LABEL} {} source changed between provenance and planning",
                role.file_name()
            ));
        }
        plan.source = Self::historical_bug689_source(role, canonical_lf)?;
        Self::validate_bug689_plan(role, &plan)?;
        Ok(plan)
    }

    pub(super) fn validate_bug689_oracle(&self, start: Instant) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired before execution"));
        }
        let owner =
            Self::read_corpus_canonical(LABEL, &self.root.join(Bug689Role::CrLfOwner.path()))?;
        let reference =
            Self::read_corpus_canonical(LABEL, &self.root.join(Bug689Role::LfReference.path()))?;
        if owner != reference {
            return Err(format!(
                "{LABEL} retained sources no longer normalize to identical text"
            ));
        }
        let run = |member_role: Bug689Role| {
            let plan = self.bug689_plan(member_role, &owner)?;
            let (netlist, results) = self.run_static_dc_results(&plan, start).map_err(|error| {
                format!(
                    "{LABEL} {} execution failed: {error}",
                    member_role.file_name()
                )
            })?;
            Self::validate_bug689_netlist(member_role, &netlist)?;
            let table = self.dc_results_to_prn_table(&plan, &netlist, &results)?;
            Self::validate_bug689_table(member_role, &table)?;
            Ok::<_, String>(table)
        };
        let dos = run(Bug689Role::CrLfOwner)?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired between independent runs"));
        }
        let unix = run(Bug689Role::LfReference)?;
        let mismatches = self.compare_serialized_default_prn_tables(&dos, &unix)?;
        if !mismatches.is_empty() {
            return Err(format!(
                "{LABEL} failed its historical byte relation: {mismatches:?}"
            ));
        }
        if abort.is_aborted() {
            return Err(format!("{LABEL} execution exceeded deadline"));
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
    fn bug689_historical_line_endings_are_exact() {
        let bytes = fs::read(corpus_root().join(OWNER_PATH)).expect("read retained owner");
        let canonical =
            XyceTestRunner::canonical_lf_text_identity(LABEL, &bytes).expect("canonicalize owner");
        let dos = XyceTestRunner::historical_bug689_source(Bug689Role::CrLfOwner, &canonical)
            .expect("reconstruct DOS source");
        let unix = XyceTestRunner::historical_bug689_source(Bug689Role::LfReference, &canonical)
            .expect("reconstruct Unix source");
        assert_eq!(dos.matches("\r\n").count(), 38);
        assert_eq!(unix.matches('\n').count(), 38);
        assert!(!unix.contains('\r'));
    }

    #[test]
    fn bug689_both_historical_representations_parse_and_execute() {
        let runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        let bytes = fs::read(corpus_root().join(OWNER_PATH)).expect("read retained owner");
        let canonical =
            XyceTestRunner::canonical_lf_text_identity(LABEL, &bytes).expect("canonicalize owner");
        for role in Bug689Role::ALL {
            let plan = runner
                .bug689_plan(role, &canonical)
                .expect("build exact historical plan");
            let (netlist, results) = runner
                .run_static_dc_results(&plan, Instant::now())
                .expect("execute exact historical line endings");
            XyceTestRunner::validate_bug689_netlist(role, &netlist)
                .expect("typed semantics remain exact");
            let table = runner
                .dc_results_to_prn_table(&plan, &netlist, &results)
                .expect("serialize result");
            XyceTestRunner::validate_bug689_table(role, &table)
                .expect("diode/load-line oracle passes");
        }
    }

    #[test]
    fn bug689_live_oracle_executes_both_roles() {
        let runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        runner
            .validate_bug689_oracle(Instant::now())
            .expect("BUG689 relation should execute natively");
    }

    #[test]
    fn bug689_load_line_oracle_rejects_shared_wrong_physics() {
        let (voltage, current) = XyceTestRunner::bug689_analytic_operating_point();
        let valid = XycePrnTable {
            columns: vec!["Index".into(), "I(VMON)".into(), "V(3)".into()],
            rows: vec![vec![0.0, current, voltage]],
        };
        XyceTestRunner::validate_bug689_table(Bug689Role::CrLfOwner, &valid)
            .expect("analytic row passes");
        let mut wrong = valid;
        wrong.rows[0][1] = 0.0;
        wrong.rows[0][2] = 0.0;
        assert!(XyceTestRunner::validate_bug689_table(Bug689Role::CrLfOwner, &wrong).is_err());
    }

    #[test]
    fn bug689_typed_mutations_fail_closed() {
        for (label, from, to) in [
            ("model", "IS=100FA", "IS=200FA"),
            ("subcircuit", "D1 a b DMOD", "D1 b a DMOD"),
            ("analysis", ".DC VIN 5 5 1", ".DC VIN 4 5 1"),
            ("probe", ".PRINT DC I(VMON) V(3)", ".PRINT DC I(VMON) V(2)"),
        ] {
            let path = corpus_root().join(OWNER_PATH);
            let canonical = fs::read_to_string(&path).expect("read canonical source");
            let mutated = canonical.replacen(from, to, 1);
            assert_ne!(mutated, canonical, "mutation {label} must apply");
            let netlist = XyceTestRunner::parse_xyce_netlist(&mutated, &path)
                .expect("typed mutation remains parseable");
            assert!(
                XyceTestRunner::validate_bug689_netlist(Bug689Role::CrLfOwner, &netlist).is_err(),
                "mutation {label} escaped the typed contract"
            );
        }
    }

    #[test]
    fn bug689_expired_deadline_fails_closed() {
        let mut runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        runner.config.max_time_per_test_ms = 1;
        assert!(
            runner
                .validate_bug689_oracle(Instant::now() - Duration::from_millis(10))
                .is_err()
        );
    }
}
