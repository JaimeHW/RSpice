use super::*;

const LABEL: &str = "BUG_1152 device/node namespace separation";
const RLC_PATH: &str = "Netlists/Certification_Tests/BUG_1152/bug_1152.cir";
const COLLISION_PATH: &str = "Netlists/Certification_Tests/BUG_1152/bug_767b.cir";
const CONTROL_PATH: &str = "Netlists/Certification_Tests/BUG_1152/bug_767b_nodupes.cir";
const RLC_RECORD: &str = "netlists/certification_tests/bug_1152/bug_1152.cir";
const COLLISION_RECORD: &str = "netlists/certification_tests/bug_1152/bug_767b.cir";
const CONTROL_RECORD: &str = "netlists/certification_tests/bug_1152/bug_767b_nodupes.cir";
const RLC_CONTRACT: &str = "bug1152_rlc_device_node_name_analytic_tran_wrapper_owner";
const COLLISION_CONTRACT: &str = "bug1152_duplicate_device_node_name_relational_wrapper_owner";
const CONTROL_CONTRACT: &str = "bug1152_duplicate_device_node_name_relational_control";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Bug1152Role {
    RlcDeviceNodeNameOwner,
    DuplicateDeviceNodeOwner,
    NoDuplicateControl,
}

impl Bug1152Role {
    const ALL: [Self; 3] = [
        Self::RlcDeviceNodeNameOwner,
        Self::DuplicateDeviceNodeOwner,
        Self::NoDuplicateControl,
    ];

    pub(super) fn for_record(relative_path: &str) -> Option<Self> {
        match XyceTestRunner::normalize_manifest_key(relative_path).as_str() {
            RLC_RECORD => Some(Self::RlcDeviceNodeNameOwner),
            COLLISION_RECORD => Some(Self::DuplicateDeviceNodeOwner),
            CONTROL_RECORD => Some(Self::NoDuplicateControl),
            _ => None,
        }
    }

    pub(super) fn contract(self) -> &'static str {
        match self {
            Self::RlcDeviceNodeNameOwner => RLC_CONTRACT,
            Self::DuplicateDeviceNodeOwner => COLLISION_CONTRACT,
            Self::NoDuplicateControl => CONTROL_CONTRACT,
        }
    }

    fn path(self) -> &'static str {
        match self {
            Self::RlcDeviceNodeNameOwner => RLC_PATH,
            Self::DuplicateDeviceNodeOwner => COLLISION_PATH,
            Self::NoDuplicateControl => CONTROL_PATH,
        }
    }

    fn file_name(self) -> &'static str {
        Path::new(self.path())
            .file_name()
            .and_then(|name| name.to_str())
            .expect("BUG1152 paths use UTF-8 file names")
    }

    fn label(self) -> &'static str {
        match self {
            Self::RlcDeviceNodeNameOwner => "RLC analytic wrapper owner",
            Self::DuplicateDeviceNodeOwner => "duplicate-name relational wrapper owner",
            Self::NoDuplicateControl => "renamed-node relational control",
        }
    }
}

impl XyceTestRunner {
    fn bug1152_nodes_match(actual: &[String], expected: &[&str]) -> bool {
        actual.len() == expected.len()
            && actual
                .iter()
                .zip(expected)
                .all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
    }

    fn validate_bug1152_rlc_plan(
        &self,
        source: &str,
        path: &Path,
    ) -> Result<(XyceStaticTranPlan, Netlist), String> {
        let plan = self.static_tran_plan_for_path_with_purpose(
            path,
            XyceStaticTranPlanPurpose::AnalyticOracle,
        )?;
        let probes = plan
            .print
            .as_ref()
            .map(|print| {
                print
                    .probes
                    .iter()
                    .map(|probe| Self::normalize_probe(probe))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        if plan.deck_path != path
            || plan.source != source
            || !matches!(plan.oracle, XyceStaticTranOracle::None)
            || plan.contract != XyceStaticTranContract::WrapperStatic
            || plan.comparison_mode != XyceStaticTranComparisonMode::Pointwise
            || probes != ["v(vin)", "{i(vin)-1.0}"]
            || !plan.steps.is_empty()
            || plan.output_override
            || plan.timeint_conststep
            || plan.wrapper_tolerance.is_some()
            || plan.tran.step.to_bits() != 0.01f64.to_bits()
            || plan.tran.stop.to_bits() != 10.0f64.to_bits()
            || plan.tran.start.map(Value::to_bits) != Some(0.0f64.to_bits())
            || plan.tran.max_step.is_some()
            || plan.tran.uic
        {
            return Err(format!("{LABEL} RLC plan changed: {plan:?}"));
        }

        let netlist = Self::parse_xyce_netlist(source, path)
            .map_err(|error| format!("{LABEL} RLC owner no longer parses: {error}"))?;
        if netlist.elements.len() != 4
            || !netlist.models.is_empty()
            || !netlist.subcircuits.is_empty()
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || !netlist.diagnostics.is_empty()
            || netlist.options.timeint_reltol.map(Value::to_bits) != Some(1.0e-3f64.to_bits())
        {
            return Err(format!(
                "{LABEL} RLC typed envelope changed: elements={:?}, analyses={:?}, outputs={:?}, diagnostics={:?}",
                netlist.elements, netlist.analyses, netlist.output_requests, netlist.diagnostics
            ));
        }
        let find = |name: &str| {
            netlist
                .elements
                .iter()
                .find(|element| element.name.eq_ignore_ascii_case(name))
        };
        let resistor = find("R1").ok_or_else(|| format!("{LABEL} lost R1"))?;
        let inductor = find("L1").ok_or_else(|| format!("{LABEL} lost L1"))?;
        let capacitor = find("C1").ok_or_else(|| format!("{LABEL} lost C1"))?;
        let source_element = find("VIN").ok_or_else(|| format!("{LABEL} lost VIN source"))?;
        if !Self::bug1152_nodes_match(&resistor.nodes, &["vin", "2"])
            || !matches!(&resistor.kind, ElementKind::Resistor { value, value_expr: None, model: None, instance_params, deferred_params }
                if value.to_bits() == 3.0f64.to_bits()
                    && instance_params.is_empty() && deferred_params.is_empty())
            || !Self::bug1152_nodes_match(&inductor.nodes, &["2", "3"])
            || !matches!(&inductor.kind, ElementKind::Inductor { value, value_expr: None, initial_current: None, model: None, instance_params, deferred_params }
                if value.to_bits() == 1.0f64.to_bits()
                    && instance_params.is_empty() && deferred_params.is_empty())
            || !Self::bug1152_nodes_match(&capacitor.nodes, &["3", "0"])
            || !matches!(&capacitor.kind, ElementKind::Capacitor { value, value_expr: None, initial_voltage: None, model: None, instance_params, deferred_params }
                if value.to_bits() == 0.5f64.to_bits()
                    && instance_params.is_empty() && deferred_params.is_empty())
            || !source_element
                .name
                .eq_ignore_ascii_case(&source_element.nodes[0])
            || !Self::bug1152_nodes_match(&source_element.nodes, &["vin", "0"])
        {
            return Err(format!(
                "{LABEL} RLC topology changed: {:?}",
                netlist.elements
            ));
        }
        let ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::DcTransient {
            dc_value,
            transient,
        }) = &source_element.kind
        else {
            return Err(format!("{LABEL} VIN lost its DC-plus-PULSE source form"));
        };
        if dc_value.to_bits() != 10.0f64.to_bits()
            || !matches!(transient.as_ref(), rspice_core::netlist::SourceSpec::Pulse {
                v1, v2, delay, rise, fall, width, period, pulse_count, width_defaults_to_zero,
            } if v1.to_bits() == 0.0f64.to_bits()
                && v2.to_bits() == 10.0f64.to_bits()
                && delay.to_bits() == 0.0f64.to_bits()
                && rise.to_bits() == 0.0f64.to_bits()
                && fall.to_bits() == 0.0f64.to_bits()
                && width.to_bits() == 10.0f64.to_bits()
                && period.to_bits() == 10.0f64.to_bits()
                && pulse_count.to_bits() == 0.0f64.to_bits()
                && !width_defaults_to_zero)
        {
            return Err(format!("{LABEL} VIN source parameters changed"));
        }
        validate_output_symbols(&netlist)
            .map_err(|error| format!("{LABEL} RLC node/device probes do not resolve: {error}"))?;
        Ok((plan, netlist))
    }

    fn bug1152_rlc_reference(actual: &XycePrnTable) -> Result<XycePrnTable, String> {
        if actual.columns.len() != 4 {
            return Err(format!(
                "{LABEL} RLC table requires Index/TIME/V(VIN)/expression, got {:?}",
                actual.columns
            ));
        }
        let mut rows = Vec::with_capacity(actual.rows.len());
        for (row_index, row) in actual.rows.iter().enumerate() {
            if row.len() != 4 || row.iter().any(|value| !value.is_finite()) {
                return Err(format!("{LABEL} RLC row {row_index} is malformed: {row:?}"));
            }
            // The removed Perl generator read the already serialized time and
            // copied the serialized node voltage from the candidate PRN. Only
            // its current-expression column was independently analytic.
            let time = Self::xyce_default_prn_roundtrip(row[1])?;
            let voltage = Self::xyce_default_prn_roundtrip(row[2])?;
            let current_expression = Self::xyce_default_prn_roundtrip(
                10.0 * ((-2.0 * time).exp() - (-time).exp()) - 1.0,
            )?;
            rows.push(vec![row[0], time, voltage, current_expression]);
        }
        Ok(XycePrnTable {
            columns: actual.columns.clone(),
            rows,
        })
    }

    fn run_bug1152_rlc_oracle(&self, source: &str, start: Instant) -> Result<(), String> {
        let path = self.root.join(RLC_PATH);
        let (plan, _) = self.validate_bug1152_rlc_plan(source, &path)?;
        let (netlist, result) = self
            .run_transient_family_plan(&plan, start, None, None)
            .map_err(|error| format!("{LABEL} RLC simulation failed: {error}"))?;
        let actual = Self::transient_family_result_to_prn_table(&plan, &netlist, &result)
            .map_err(|error| format!("{LABEL} RLC PRN projection failed: {error}"))?;
        if actual.rows.len() < 100
            || actual.rows.first().is_none_or(|row| {
                row.len() != 4 || row[1].abs() > 1.0e-15 || row[2].abs() > 1.0e-12
            })
            || actual
                .rows
                .last()
                .is_none_or(|row| row.len() != 4 || (row[1] - 10.0).abs() > 1.0e-9)
            || actual
                .rows
                .iter()
                .skip(1)
                .any(|row| (row[2] - 10.0).abs() > 1.0e-8)
            || actual
                .rows
                .iter()
                .map(|row| row[3])
                .fold(Value::INFINITY, Value::min)
                > -3.0
        {
            return Err(format!(
                "{LABEL} RLC output is incomplete or trivial: columns={:?}, rows={}",
                actual.columns,
                actual.rows.len()
            ));
        }
        let reference = Self::bug1152_rlc_reference(&actual)?;
        let mismatches = self.compare_xyce_verify_transient_tables(&reference, &actual)?;
        if !mismatches.is_empty() {
            return Err(format!(
                "{LABEL} RLC current differs from the Release analytic generator: {mismatches:?}"
            ));
        }
        Ok(())
    }

    fn validate_bug1152_relational_plan(
        &self,
        role: Bug1152Role,
        source: &str,
        path: &Path,
    ) -> Result<(XyceStaticTranPlan, Netlist), String> {
        let purpose = match role {
            Bug1152Role::DuplicateDeviceNodeOwner => {
                XyceStaticTranPlanPurpose::GeneratedReferenceRelationalFamily
            }
            Bug1152Role::NoDuplicateControl => XyceStaticTranPlanPurpose::RelationalFamily,
            Bug1152Role::RlcDeviceNodeNameOwner => {
                return Err(format!("{LABEL} RLC role is not a relational worker"));
            }
        };
        let plan = self.static_tran_plan_for_path_with_purpose(path, purpose)?;
        let probes = plan
            .print
            .as_ref()
            .map(|print| {
                print
                    .probes
                    .iter()
                    .map(|probe| Self::normalize_probe(probe))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let expected_contract = if role == Bug1152Role::DuplicateDeviceNodeOwner {
            XyceStaticTranContract::WrapperStatic
        } else {
            XyceStaticTranContract::PlainStatic
        };
        if plan.deck_path != path
            || plan.source != source
            || !matches!(plan.oracle, XyceStaticTranOracle::None)
            || plan.contract != expected_contract
            || plan.comparison_mode != XyceStaticTranComparisonMode::Pointwise
            || probes != ["{v(20)+1}", "v(3)"]
            || !plan.steps.is_empty()
            || plan.output_override
            || plan.timeint_conststep
            || plan.wrapper_tolerance.is_some()
            || plan.tran.step.to_bits() != 0.2e-6f64.to_bits()
            || plan.tran.stop.to_bits() != 3.0e-3f64.to_bits()
            || plan.tran.start.map(Value::to_bits) != Some(0.5e-3f64.to_bits())
            || plan.tran.max_step.is_some()
            || plan.tran.uic
        {
            return Err(format!("{LABEL} {} plan changed: {plan:?}", role.label()));
        }

        let netlist = Self::parse_xyce_netlist(source, path)
            .map_err(|error| format!("{LABEL} {} no longer parses: {error}", role.label()))?;
        let [subcircuit] = netlist.subcircuits.as_slice() else {
            return Err(format!("{LABEL} {} requires one OPAMP#0", role.label()));
        };
        if netlist.elements.len() != 13
            || netlist.models.len() != 1
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || !netlist.diagnostics.is_empty()
            || !subcircuit.name.eq_ignore_ascii_case("OPAMP#0")
            || !Self::bug1152_nodes_match(&subcircuit.ports, &["2", "3", "6", "7", "4"])
            || subcircuit.elements.len() != 28
        {
            return Err(format!(
                "{LABEL} {} typed envelope changed: top={}, models={}, subcircuits={:?}, diagnostics={:?}",
                role.label(),
                netlist.elements.len(),
                netlist.models.len(),
                netlist.subcircuits,
                netlist.diagnostics
            ));
        }
        let resistor = subcircuit
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case("R1"))
            .ok_or_else(|| format!("{LABEL} {} lost internal resistor R1", role.label()))?;
        if !Self::bug1152_nodes_match(&resistor.nodes, &["11", "12"])
            || !matches!(&resistor.kind, ElementKind::Resistor { value, .. }
                if value.to_bits() == 5_000.0f64.to_bits())
        {
            return Err(format!(
                "{LABEL} {} internal device R1 changed",
                role.label()
            ));
        }
        let collision_node = if role == Bug1152Role::DuplicateDeviceNodeOwner {
            "R1"
        } else {
            "14"
        };
        let colliding_members = ["B", "C2", "RO", "L", "RL"];
        for member in colliding_members {
            let element = subcircuit
                .elements
                .iter()
                .find(|element| element.name.eq_ignore_ascii_case(member))
                .ok_or_else(|| format!("{LABEL} {} lost internal {member}", role.label()))?;
            if !element
                .nodes
                .iter()
                .any(|node| node.eq_ignore_ascii_case(collision_node))
            {
                return Err(format!(
                    "{LABEL} {} internal {member} lost node {collision_node}",
                    role.label()
                ));
            }
        }
        let r1_node_uses = subcircuit
            .elements
            .iter()
            .flat_map(|element| &element.nodes)
            .filter(|node| node.eq_ignore_ascii_case("R1"))
            .count();
        let node14_uses = subcircuit
            .elements
            .iter()
            .flat_map(|element| &element.nodes)
            .filter(|node| node.eq_ignore_ascii_case("14"))
            .count();
        if (role == Bug1152Role::DuplicateDeviceNodeOwner
            && (r1_node_uses != 5 || node14_uses != 0))
            || (role == Bug1152Role::NoDuplicateControl && (r1_node_uses != 0 || node14_uses != 5))
        {
            return Err(format!(
                "{LABEL} {} namespace delta changed: R1-node uses={r1_node_uses}, 14-node uses={node14_uses}",
                role.label()
            ));
        }
        validate_output_symbols(&netlist).map_err(|error| {
            format!(
                "{LABEL} {} output symbols do not resolve: {error}",
                role.label()
            )
        })?;
        Ok((plan, netlist))
    }

    fn run_bug1152_relational_oracle(
        &self,
        sources: &BTreeMap<Bug1152Role, Vec<u8>>,
        start: Instant,
    ) -> Result<(), String> {
        let mut outputs = BTreeMap::new();
        let mut owner_source = None;
        for role in [
            Bug1152Role::DuplicateDeviceNodeOwner,
            Bug1152Role::NoDuplicateControl,
        ] {
            let source = std::str::from_utf8(
                sources
                    .get(&role)
                    .ok_or_else(|| format!("{LABEL} lost {} source", role.label()))?,
            )
            .map_err(|error| format!("{LABEL} {} is not UTF-8: {error}", role.label()))?;
            if role == Bug1152Role::DuplicateDeviceNodeOwner {
                owner_source = Some(source.to_string());
            }
            let path = self.root.join(role.path());
            let (plan, _) = self.validate_bug1152_relational_plan(role, source, &path)?;
            let (netlist, result) = self
                .run_transient_family_plan(&plan, start, None, None)
                .map_err(|error| format!("{LABEL} {} simulation failed: {error}", role.label()))?;
            let table = Self::transient_family_result_to_prn_table(&plan, &netlist, &result)
                .map_err(|error| format!("{LABEL} {} PRN failed: {error}", role.label()))?;
            let expression_min = table
                .rows
                .iter()
                .map(|row| row.get(2).copied().unwrap_or(Value::NAN))
                .fold(Value::INFINITY, Value::min);
            let expression_max = table
                .rows
                .iter()
                .map(|row| row.get(2).copied().unwrap_or(Value::NAN))
                .fold(Value::NEG_INFINITY, Value::max);
            let input_min = table
                .rows
                .iter()
                .map(|row| row.get(3).copied().unwrap_or(Value::NAN))
                .fold(Value::INFINITY, Value::min);
            let input_max = table
                .rows
                .iter()
                .map(|row| row.get(3).copied().unwrap_or(Value::NAN))
                .fold(Value::NEG_INFINITY, Value::max);
            if table.columns.len() != 4
                || table.rows.len() < 20
                || table
                    .rows
                    .iter()
                    .any(|row| row.len() != 4 || row.iter().any(|value| !value.is_finite()))
                || table.rows.first().is_none_or(|row| {
                    row[1] < 0.5e-3 - 1.0e-12 || row[1] >= 0.75e-3 || row[3].abs() > 1.0e-8
                })
                || table
                    .rows
                    .last()
                    .is_none_or(|row| (row[1] - 3.0e-3).abs() > 1.0e-12)
                || expression_max - expression_min < 0.1
                || input_max - input_min < 4.9
            {
                return Err(format!(
                    "{LABEL} {} produced an invalid or trivial table: columns={:?}, rows={}, expression-range={:?}, input-range={:?}",
                    role.label(),
                    table.columns,
                    table.rows.len(),
                    expression_max - expression_min,
                    input_max - input_min,
                ));
            }
            outputs.insert(role, table);
        }
        let good = outputs
            .get(&Bug1152Role::DuplicateDeviceNodeOwner)
            .expect("both BUG1152 relational workers ran");
        let test = outputs
            .get(&Bug1152Role::NoDuplicateControl)
            .expect("both BUG1152 relational workers ran");
        let tolerances = Self::xyce_verify_comp_tolerances(
            owner_source.as_deref().expect("owner source was recorded"),
            &good.columns[2..],
        )?;
        if tolerances.len() != 2
            || tolerances[0].relative.to_bits() != 0.02f64.to_bits()
            || tolerances[1].relative.to_bits() != 0.01f64.to_bits()
        {
            return Err(format!(
                "{LABEL} *COMP tolerance contract changed: {tolerances:?}"
            ));
        }
        let mismatches = self.compare_xyce_verify_transient_tables_with_probe_tolerances(
            good,
            test,
            &tolerances,
            XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
        )?;
        if !mismatches.is_empty() {
            return Err(format!(
                "{LABEL} duplicate-name owner differs from renamed control: {mismatches:?}"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_bug1152_oracle(
        &self,
        role: Bug1152Role,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired before validation"));
        }
        let sources = Bug1152Role::ALL
            .into_iter()
            .map(|member| {
                fs::read(self.root.join(member.path()))
                    .map(|bytes| (member, bytes))
                    .map_err(|error| format!("{LABEL} cannot read {}: {error}", member.file_name()))
            })
            .collect::<Result<BTreeMap<_, _>, String>>()?;
        match role {
            Bug1152Role::RlcDeviceNodeNameOwner => {
                let source = std::str::from_utf8(
                    sources
                        .get(&role)
                        .ok_or_else(|| format!("{LABEL} lost RLC owner source"))?,
                )
                .map_err(|error| format!("{LABEL} RLC owner is not UTF-8: {error}"))?;
                self.run_bug1152_rlc_oracle(source, start)?;
            }
            Bug1152Role::DuplicateDeviceNodeOwner | Bug1152Role::NoDuplicateControl => {
                self.run_bug1152_relational_oracle(&sources, start)?;
            }
        }
        if abort.is_aborted() {
            return Err(format!(
                "{LABEL} execution exceeded timeout ({}ms)",
                self.config.max_time_per_test_ms
            ));
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
    fn bug1152_typed_plans_preserve_both_namespace_contracts() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let rlc_source = fs::read_to_string(root.join(RLC_PATH)).expect("read BUG1152 RLC");
        runner
            .validate_bug1152_rlc_plan(&rlc_source, &root.join(RLC_PATH))
            .expect("RLC node/device-name contract remains exact");
        for role in [
            Bug1152Role::DuplicateDeviceNodeOwner,
            Bug1152Role::NoDuplicateControl,
        ] {
            let source = fs::read_to_string(root.join(role.path())).expect("read BUG1152 worker");
            runner
                .validate_bug1152_relational_plan(role, &source, &root.join(role.path()))
                .unwrap_or_else(|error| panic!("canonical {role:?} failed: {error}"));
        }
    }

    #[test]
    fn bug1152_analytic_rlc_oracle_executes() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        runner
            .validate_bug1152_oracle(Bug1152Role::RlcDeviceNodeNameOwner, Instant::now())
            .expect("canonical BUG1152 analytic owner passes");
    }

    #[test]
    fn bug1152_relational_oracle_executes_both_topologies() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        runner
            .validate_bug1152_oracle(Bug1152Role::DuplicateDeviceNodeOwner, Instant::now())
            .expect("canonical BUG1152 collision relation passes");
    }
}
