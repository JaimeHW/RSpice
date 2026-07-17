use super::*;
use crate::netlist::{ModelDef, StepSweep};

#[derive(Clone, Copy)]
enum DeviceStepResolution {
    Device(usize),
    Model(usize),
}

/// Explicit resource bound for Cartesian `.STEP` execution planning.
///
/// The planner does not impose a hidden policy limit. Callers choose a finite
/// maximum appropriate to their execution environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StepPlanLimits {
    max_runs: usize,
    max_dimensions: usize,
    max_bindings_per_run: usize,
    max_stored_values: usize,
}

impl StepPlanLimits {
    pub const fn new(
        max_runs: usize,
        max_dimensions: usize,
        max_bindings_per_run: usize,
        max_stored_values: usize,
    ) -> Self {
        Self {
            max_runs,
            max_dimensions,
            max_bindings_per_run,
            max_stored_values,
        }
    }

    pub const fn max_runs(self) -> usize {
        self.max_runs
    }

    pub const fn max_dimensions(self) -> usize {
        self.max_dimensions
    }

    pub const fn max_bindings_per_run(self) -> usize {
        self.max_bindings_per_run
    }

    pub const fn max_stored_values(self) -> usize {
        self.max_stored_values
    }
}

#[derive(Debug, Clone)]
enum StepPlanDimension {
    Values(Vec<Value>),
    PendingDataRows {
        row_count: usize,
    },
    DataRows {
        params: Vec<String>,
        rows: Vec<Vec<Value>>,
    },
}

impl StepPlanDimension {
    fn len(&self) -> usize {
        match self {
            Self::Values(values) => values.len(),
            Self::PendingDataRows { row_count } => *row_count,
            Self::DataRows { rows, .. } => rows.len(),
        }
    }

    fn binding_at(&self, index: usize) -> StepPlanBindingValue {
        match self {
            Self::Values(values) => StepPlanBindingValue::Scalar(values[index]),
            Self::PendingDataRows { .. } | Self::DataRows { .. } => {
                StepPlanBindingValue::DataRow(index)
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum StepPlanBindingValue {
    Scalar(Value),
    DataRow(usize),
}

/// Checked, lazy Cartesian plan for one or more parsed `.STEP` commands.
///
/// Combinations are decoded by mixed radix instead of being stored. The first
/// command varies fastest, matching Xyce's nested `.STEP` ordering.
#[derive(Debug, Clone)]
pub struct StepPlan<'a> {
    base_netlist: &'a Netlist,
    steps: Vec<StepCommand>,
    dimensions: Vec<StepPlanDimension>,
    total_runs: usize,
    bindings_per_run: usize,
    stored_values: usize,
}

impl StepPlan<'_> {
    pub fn steps(&self) -> &[StepCommand] {
        &self.steps
    }

    pub const fn total_runs(&self) -> usize {
        self.total_runs
    }

    pub const fn bindings_per_run(&self) -> usize {
        self.bindings_per_run
    }

    pub const fn stored_values(&self) -> usize {
        self.stored_values
    }

    pub fn step_values(&self, run_index: usize) -> Option<Vec<Value>> {
        self.bindings_for_run(run_index).map(|bindings| {
            bindings
                .into_iter()
                .map(|binding| match binding {
                    StepPlanBindingValue::Scalar(value) => value,
                    StepPlanBindingValue::DataRow(row_index) => row_index as Value,
                })
                .collect()
        })
    }

    fn bindings_for_run(&self, run_index: usize) -> Option<Vec<StepPlanBindingValue>> {
        if run_index >= self.total_runs {
            return None;
        }

        let mut quotient = run_index;
        let mut bindings = Vec::with_capacity(self.dimensions.len());
        for dimension in &self.dimensions {
            let index = quotient % dimension.len();
            quotient /= dimension.len();
            bindings.push(dimension.binding_at(index));
        }
        Some(bindings)
    }
}

/// A netlist materialized for one checked Cartesian `.STEP` combination.
#[derive(Debug, Clone)]
pub struct MaterializedStepRun {
    run_index: usize,
    step_values: Vec<Value>,
    netlist: Netlist,
}

impl MaterializedStepRun {
    pub const fn run_index(&self) -> usize {
        self.run_index
    }

    pub fn step_values(&self) -> &[Value] {
        &self.step_values
    }

    pub fn netlist(&self) -> &Netlist {
        &self.netlist
    }

    pub fn into_parts(self) -> (Vec<Value>, Netlist) {
        (self.step_values, self.netlist)
    }
}

impl Engine {
    fn push_step_result(
        &self,
        results: &mut Vec<(Value, SimulationResult)>,
        retained_values: &mut usize,
        value: Value,
        result: SimulationResult,
    ) -> Result<(), SimulationError> {
        *retained_values = retained_values
            .saturating_add(Self::simulation_result_value_count(&result))
            .saturating_add(1);
        self.ensure_result_values(*retained_values)?;
        results.push((value, result));
        Ok(())
    }

    /// Build a checked Cartesian plan without materializing any run netlists.
    pub fn plan_step_commands<'a>(
        &self,
        netlist: &'a Netlist,
        steps: &[StepCommand],
        limits: StepPlanLimits,
    ) -> Result<StepPlan<'a>, SimulationError> {
        self.plan_step_commands_with_abort(netlist, steps, limits, &NoAbort)
    }

    /// Build a checked Cartesian plan with cooperative cancellation.
    pub fn plan_step_commands_with_abort<'a>(
        &self,
        netlist: &'a Netlist,
        steps: &[StepCommand],
        limits: StepPlanLimits,
        abort: &dyn AbortSignal,
    ) -> Result<StepPlan<'a>, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        self.ensure_batch_runs(1)?;
        let configured_limits = [
            ("max_runs", limits.max_runs),
            ("max_dimensions", limits.max_dimensions),
            ("max_bindings_per_run", limits.max_bindings_per_run),
            ("max_stored_values", limits.max_stored_values),
        ];
        if let Some((name, _)) = configured_limits.iter().find(|(_, value)| *value == 0) {
            return Err(SimulationError::Circuit(format!(
                ".STEP execution limit {name} must be at least 1; configured limit is 0"
            )));
        }
        if steps.len() > limits.max_dimensions {
            return Err(SimulationError::Circuit(format!(
                ".STEP plan requests {} dimension(s), exceeding configured max_dimensions {}",
                steps.len(),
                limits.max_dimensions
            )));
        }

        let mut dimensions = Vec::new();
        let mut canonical_targets = HashSet::new();
        let mut total_runs = 1usize;
        let mut bindings_per_run = 0usize;
        let mut stored_values = 0usize;
        for (dimension_index, step) in steps.iter().enumerate() {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }

            let dimension = match &step.sweep {
                StepSweep::Data { table_name } => {
                    let table = Self::validated_data_step_table(netlist, table_name, abort)?;
                    bindings_per_run = checked_step_resource_sum(
                        bindings_per_run,
                        table.params.len(),
                        limits.max_bindings_per_run,
                        "bindings per run",
                    )?;
                    let data_cells = table
                        .rows
                        .len()
                        .checked_mul(table.params.len())
                        .ok_or_else(|| {
                            SimulationError::Circuit(format!(
                                ".STEP DATA table '{}' cell count overflows usize",
                                table.name
                            ))
                        })?;
                    stored_values = checked_step_resource_sum(
                        stored_values,
                        data_cells,
                        limits.max_stored_values,
                        "stored values",
                    )?;
                    for param in &table.params {
                        Self::insert_step_target(
                            &mut canonical_targets,
                            Self::canonical_reparse_override_target(netlist, param)?,
                        )?;
                    }
                    StepPlanDimension::PendingDataRows {
                        row_count: table.rows.len(),
                    }
                }
                _ => {
                    let remaining_values = limits.max_stored_values - stored_values;
                    let values = step
                        .sweep
                        .values_bounded_with_abort(remaining_values, abort)
                        .map_err(|error| match error {
                            crate::netlist::SweepPointGenerationError::Aborted => {
                                SimulationError::Aborted
                            }
                            crate::netlist::SweepPointGenerationError::LimitExceeded {
                                requested,
                                limit,
                            } => SimulationError::Circuit(format!(
                                ".STEP dimension {} requests {requested} stored value(s), exceeding remaining max_stored_values {limit}",
                                dimension_index + 1
                            )),
                        })?;
                    validate_step_values(&values)?;
                    stored_values = checked_step_resource_sum(
                        stored_values,
                        values.len(),
                        limits.max_stored_values,
                        "stored values",
                    )?;
                    let dimension_bindings = if step.target == StepTarget::Temp {
                        3
                    } else {
                        1
                    };
                    bindings_per_run = checked_step_resource_sum(
                        bindings_per_run,
                        dimension_bindings,
                        limits.max_bindings_per_run,
                        "bindings per run",
                    )?;
                    for target in Self::canonical_step_targets(netlist, step)? {
                        Self::insert_step_target(&mut canonical_targets, target)?;
                    }
                    StepPlanDimension::Values(values)
                }
            };
            total_runs = checked_step_cardinality(
                total_runs,
                dimension.len(),
                limits.max_runs,
                dimension_index,
            )?;
            self.ensure_batch_runs(total_runs)?;
            dimensions.push(dimension);
        }

        // DATA coordinates are copied only after the complete Cartesian count
        // has been checked against overflow and the caller's execution limit.
        for (step, dimension) in steps.iter().zip(dimensions.iter_mut()) {
            if let StepPlanDimension::PendingDataRows { .. } = dimension {
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                let StepSweep::Data { table_name } = &step.sweep else {
                    return Err(SimulationError::Circuit(
                        "Internal .STEP DATA plan mismatch".to_string(),
                    ));
                };
                let table = netlist
                    .data_tables
                    .iter()
                    .find(|table| table.name.eq_ignore_ascii_case(table_name))
                    .ok_or_else(|| {
                        SimulationError::Circuit(format!(
                            ".STEP DATA table '{table_name}' disappeared while planning"
                        ))
                    })?;
                let (params, rows) = Self::copy_data_step_coordinates(table, abort)?;
                *dimension = StepPlanDimension::DataRows { params, rows };
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
            }
        }

        Ok(StepPlan {
            base_netlist: netlist,
            steps: steps.to_vec(),
            dimensions,
            total_runs,
            bindings_per_run,
            stored_values,
        })
    }

    /// Materialize every member of an already checked `.STEP` plan.
    pub fn materialize_step_plan(
        &self,
        plan: &StepPlan<'_>,
    ) -> Result<Vec<MaterializedStepRun>, SimulationError> {
        self.materialize_step_plan_with_abort(plan, &NoAbort)
    }

    /// Materialize one indexed member of a checked `.STEP` plan.
    ///
    /// The plan retains its immutable planning base, so DATA coordinates and
    /// target resolution cannot be applied to a different netlist.
    pub fn materialize_step_run(
        &self,
        plan: &StepPlan<'_>,
        run_index: usize,
    ) -> Result<MaterializedStepRun, SimulationError> {
        self.materialize_step_run_with_abort(plan, run_index, &NoAbort)
    }

    /// Materialize one indexed member with cooperative cancellation.
    pub fn materialize_step_run_with_abort(
        &self,
        plan: &StepPlan<'_>,
        run_index: usize,
        abort: &dyn AbortSignal,
    ) -> Result<MaterializedStepRun, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let bindings = plan.bindings_for_run(run_index).ok_or_else(|| {
            SimulationError::Circuit(format!(
                ".STEP run index {run_index} is outside planned cardinality {}",
                plan.total_runs
            ))
        })?;
        let (step_values, netlist) = self.materialize_step_bindings(
            plan.base_netlist,
            &plan.steps,
            &plan.dimensions,
            &bindings,
            abort,
        )?;
        Ok(MaterializedStepRun {
            run_index,
            step_values,
            netlist,
        })
    }

    /// Materialize an already checked `.STEP` plan with cooperative cancellation.
    pub fn materialize_step_plan_with_abort(
        &self,
        plan: &StepPlan<'_>,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<MaterializedStepRun>, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        self.ensure_batch_runs(plan.total_runs)?;

        let mut runs = Vec::with_capacity(plan.total_runs);
        for run_index in 0..plan.total_runs {
            runs.push(self.materialize_step_run_with_abort(plan, run_index, abort)?);
        }
        Ok(runs)
    }

    fn materialize_step_bindings(
        &self,
        base_netlist: &Netlist,
        steps: &[StepCommand],
        dimensions: &[StepPlanDimension],
        bindings: &[StepPlanBindingValue],
        abort: &dyn AbortSignal,
    ) -> Result<(Vec<Value>, Netlist), SimulationError> {
        if steps.len() != bindings.len() || steps.len() != dimensions.len() {
            return Err(SimulationError::Circuit(
                "Internal .STEP plan dimension mismatch".to_string(),
            ));
        }

        let mut step_values = Vec::with_capacity(bindings.len());
        let mut reparse_overrides = Vec::new();
        let mut temperature = None;
        let mut ast_bindings = Vec::new();
        for ((step, dimension), binding) in
            steps.iter().zip(dimensions).zip(bindings.iter().copied())
        {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            match binding {
                StepPlanBindingValue::DataRow(row_index) => {
                    let StepSweep::Data { .. } = &step.sweep else {
                        return Err(SimulationError::Circuit(
                            "Internal .STEP DATA binding mismatch".to_string(),
                        ));
                    };
                    let StepPlanDimension::DataRows { params, rows } = dimension else {
                        return Err(SimulationError::Circuit(
                            "Internal .STEP DATA coordinate mismatch".to_string(),
                        ));
                    };
                    let row = rows.get(row_index).ok_or_else(|| {
                        SimulationError::Circuit(format!(".STEP DATA plan has no row {row_index}"))
                    })?;
                    reparse_overrides.extend(params.iter().cloned().zip(row.iter().copied()));
                    step_values.push(row_index as Value);
                }
                StepPlanBindingValue::Scalar(value) => {
                    step_values.push(value);
                    match step.target {
                        StepTarget::Param => {
                            reparse_overrides.push((step.name.clone(), value));
                        }
                        StepTarget::Temp => {
                            let vt = thermal_voltage_celsius(value);
                            reparse_overrides.extend([
                                ("TEMP".to_string(), value),
                                ("TEMPER".to_string(), value),
                                ("VT".to_string(), vt),
                            ]);
                            temperature = Some((value, vt));
                        }
                        StepTarget::Device | StepTarget::Model => {
                            ast_bindings.push((step, value));
                        }
                    }
                }
            }
        }

        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let mut netlist = if reparse_overrides.is_empty() {
            base_netlist.clone()
        } else {
            Self::create_perturbed_netlist_multi_with_limits_and_abort(
                base_netlist,
                &reparse_overrides,
                self.config.resource_limits,
                abort,
            )?
            .0
        };
        if let Some((temp_c, vt)) = temperature {
            apply_temperature_scalars(&mut netlist, temp_c, vt);
        }
        for (step, value) in ast_bindings {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            Self::apply_ast_step_value_in_place(&mut netlist, step, value)?;
        }
        if steps
            .iter()
            .any(|step| matches!(step.target, StepTarget::Device | StepTarget::Model))
        {
            Self::mark_ast_stepped_netlist(&mut netlist);
        }

        Ok((step_values, netlist))
    }

    fn insert_step_target(
        targets: &mut HashSet<String>,
        target: String,
    ) -> Result<(), SimulationError> {
        if !targets.insert(target.clone()) {
            return Err(SimulationError::Circuit(format!(
                "Duplicate .STEP target '{target}' has ambiguous nested-sweep semantics"
            )));
        }
        Ok(())
    }

    fn canonical_step_targets(
        netlist: &Netlist,
        step: &StepCommand,
    ) -> Result<Vec<String>, SimulationError> {
        let name = step.name.to_ascii_uppercase();
        match step.target {
            StepTarget::Param => Ok(vec![format!("PARAM:{name}")]),
            StepTarget::Temp => Ok(vec![
                "PARAM:TEMP".to_string(),
                "PARAM:TEMPER".to_string(),
                "PARAM:VT".to_string(),
            ]),
            StepTarget::Model => {
                let explicit_param = step.param_name.as_deref().ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        ".STEP MODEL {} requires an explicit parameter name",
                        step.name
                    ))
                })?;
                let model = Self::find_step_model(netlist, &step.name)?;
                Ok(vec![format!(
                    "MODEL:{}:{}",
                    model.name.to_ascii_uppercase(),
                    explicit_param.to_ascii_uppercase()
                )])
            }
            StepTarget::Device if step.name.contains(':') => {
                let element = Self::resolve_hierarchical_step_element(netlist, &step.name)?;
                let param =
                    Self::canonical_device_parameter(&element.kind, step.param_name.as_deref());
                Ok(vec![format!("DEVICE:{name}:{param}")])
            }
            StepTarget::Device => {
                match Self::resolve_device_or_model_step_target(
                    netlist,
                    &step.name,
                    step.param_name.as_deref(),
                )? {
                    DeviceStepResolution::Device(index) => {
                        let element = netlist.elements.get(index).ok_or_else(|| {
                            SimulationError::Circuit(
                                "Internal .STEP device target index error".to_string(),
                            )
                        })?;
                        let param = Self::canonical_device_parameter(
                            &element.kind,
                            step.param_name.as_deref(),
                        );
                        Ok(vec![format!(
                            "DEVICE:{}:{param}",
                            element.name.to_ascii_uppercase()
                        )])
                    }
                    DeviceStepResolution::Model(index) => {
                        let model = netlist.models.get(index).ok_or_else(|| {
                            SimulationError::Circuit(
                                "Internal .STEP model target index error".to_string(),
                            )
                        })?;
                        let param = step
                            .param_name
                            .as_deref()
                            .unwrap_or("VALUE")
                            .to_ascii_uppercase();
                        Ok(vec![format!(
                            "MODEL:{}:{param}",
                            model.name.to_ascii_uppercase()
                        )])
                    }
                }
            }
        }
    }

    fn canonical_reparse_override_target(
        netlist: &Netlist,
        name: &str,
    ) -> Result<String, SimulationError> {
        let Some((device_name, param_name)) = name.split_once(':') else {
            return Ok(format!("PARAM:{}", name.to_ascii_uppercase()));
        };
        if device_name.is_empty() || param_name.is_empty() || param_name.contains(':') {
            return Err(SimulationError::Circuit(format!(
                ".STEP DATA column '{name}' has an unsupported hierarchical device override; expected device:param"
            )));
        }
        let element = netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case(device_name))
            .ok_or_else(|| {
                SimulationError::Circuit(format!(
                    ".STEP DATA column '{name}' references unknown top-level device '{device_name}'"
                ))
            })?;
        let param = Self::canonical_device_parameter(&element.kind, Some(param_name));
        Ok(format!(
            "DEVICE:{}:{param}",
            element.name.to_ascii_uppercase()
        ))
    }

    fn canonical_device_parameter(kind: &ElementKind, param_name: Option<&str>) -> String {
        let raw = param_name.unwrap_or("VALUE").trim().to_ascii_uppercase();
        macro_rules! alias {
            ($names:expr, $canonical:expr) => {
                $names
                    .iter()
                    .any(|name| raw.eq_ignore_ascii_case(name))
                    .then_some($canonical)
            };
        }
        let canonical = match kind {
            ElementKind::Resistor { .. } => alias!(&["R", "VALUE"], "R")
                .or_else(|| alias!(&["L", "LENGTH"], "L"))
                .or_else(|| alias!(&["W", "WIDTH"], "W"))
                .or_else(|| alias!(&["A", "AREA"], "AREA"))
                .or_else(|| alias!(&["M", "MULT"], "M"))
                .or_else(|| alias!(&["NRS", "NRSQ", "NSQ", "SQUARES"], "NRS"))
                .or_else(|| alias!(&["TC", "TC1"], "TC1")),
            ElementKind::Capacitor { .. } => alias!(&["C", "CAP", "VALUE", "CAPACITANCE"], "C")
                .or_else(|| alias!(&["L", "LENGTH"], "L"))
                .or_else(|| alias!(&["W", "WIDTH"], "W"))
                .or_else(|| alias!(&["M", "MULT"], "M")),
            ElementKind::Inductor { .. } => alias!(&["L", "IND", "VALUE", "INDUCTANCE"], "L")
                .or_else(|| alias!(&["M", "MULT"], "M")),
            ElementKind::JilesAthertonInductor { .. } => {
                alias!(&["L", "VALUE", "INDUCTANCE"], "L")
            }
            ElementKind::VoltageSource(_) | ElementKind::CurrentSource(_) => {
                alias!(&["DC", "VALUE"], "DC")
                    .or_else(|| alias!(&["VO", "OFFSET"], "VO"))
                    .or_else(|| alias!(&["VA", "AMPLITUDE"], "VA"))
                    .or_else(|| alias!(&["F", "FREQ", "FREQUENCY"], "FREQ"))
                    .or_else(|| alias!(&["THETA", "DAMPING"], "THETA"))
            }
            ElementKind::Vcvs { .. } | ElementKind::Cccs { .. } => {
                alias!(&["GAIN", "VALUE"], "GAIN")
            }
            ElementKind::Vccs { .. } => alias!(&["GM", "TRANSCONDUCTANCE", "VALUE"], "GM"),
            ElementKind::Ccvs { .. } => alias!(&["RM", "TRANSRESISTANCE", "VALUE"], "RM"),
            ElementKind::Coupling { .. } => alias!(&["K", "COUPLING", "VALUE"], "K"),
            ElementKind::TransmissionLine { .. } => {
                alias!(&["Z0", "VALUE"], "Z0").or_else(|| alias!(&["F", "FREQ"], "FREQ"))
            }
            _ => None,
        };
        canonical.unwrap_or(&raw).to_string()
    }

    fn resolve_hierarchical_step_element<'a>(
        netlist: &'a Netlist,
        target_name: &str,
    ) -> Result<&'a crate::netlist::Element, SimulationError> {
        let segments = target_name.split(':').collect::<Vec<_>>();
        if segments.len() < 2 || segments.iter().any(|segment| segment.is_empty()) {
            return Err(SimulationError::Circuit(format!(
                ".STEP hierarchical DEVICE target '{target_name}' is malformed"
            )));
        }
        let root = netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case(segments[0]))
            .ok_or_else(|| {
                SimulationError::Circuit(format!(
                    ".STEP hierarchical DEVICE root instance '{}' not found in netlist",
                    segments[0]
                ))
            })?;
        let ElementKind::Subcircuit { subckt_name, .. } = &root.kind else {
            return Err(SimulationError::Circuit(format!(
                ".STEP hierarchical DEVICE root '{}' is not a subcircuit instance",
                segments[0]
            )));
        };
        let mut subcircuit_name = subckt_name.as_str();
        for (index, segment) in segments[1..].iter().enumerate() {
            let definition = netlist
                .subcircuits
                .iter()
                .find(|definition| definition.name.eq_ignore_ascii_case(subcircuit_name))
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        ".STEP hierarchical DEVICE target '{target_name}' references undefined subcircuit '{subcircuit_name}'"
                    ))
                })?;
            let element = definition
                .elements
                .iter()
                .find(|element| element.name.eq_ignore_ascii_case(segment))
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        ".STEP hierarchical DEVICE target '{target_name}' cannot find '{segment}' inside subcircuit '{}'",
                        definition.name
                    ))
                })?;
            if index == segments.len() - 2 {
                return Ok(element);
            }
            let ElementKind::Subcircuit { subckt_name, .. } = &element.kind else {
                return Err(SimulationError::Circuit(format!(
                    ".STEP hierarchical DEVICE path component '{segment}' in '{target_name}' is not a subcircuit instance"
                )));
            };
            subcircuit_name = subckt_name;
        }
        unreachable!("hierarchical target has at least one child segment")
    }

    /// Run .STEP parametric sweep
    ///
    /// Executes multiple simulations with different parameter values.
    /// Returns all results indexed by step values.
    pub fn run_step(
        &self,
        netlist: &Netlist,
        param_name: &str,
        values: &[Value],
    ) -> Result<Vec<(Value, SimulationResult)>, SimulationError> {
        self.run_step_with_abort(netlist, param_name, values, &NoAbort)
    }

    /// Run a parameter sweep with cooperative cancellation.
    pub fn run_step_with_abort(
        &self,
        netlist: &Netlist,
        param_name: &str,
        values: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<(Value, SimulationResult)>, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        validate_step_values(values)?;
        self.ensure_batch_runs(values.len())?;

        let mut results = Vec::with_capacity(values.len());
        let mut retained_values = 0usize;
        let mut any_binding = false;

        for &value in values {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let (modified_netlist, rebuilt) = Self::create_perturbed_netlist_with_limits_and_abort(
                netlist,
                param_name,
                value,
                self.config.resource_limits,
                abort,
            )?;
            any_binding |= rebuilt > 0;

            match self.run_dc_op_with_abort(&modified_netlist, abort) {
                Ok(result) => {
                    self.push_step_result(&mut results, &mut retained_values, value, result)?
                }
                Err(e) => return Err(step_point_error("PARAM", param_name, value, e)),
            }
        }

        if netlist.source_text.is_some() && !any_binding {
            return Err(SimulationError::Circuit(format!(
                "Parameter '{}' is not bound to any netlist expression",
                param_name
            )));
        }

        Ok(results)
    }

    /// Run `.STEP` command execution for PARAM/DEVICE/MODEL targets.
    pub fn run_step_command(
        &self,
        netlist: &Netlist,
        step_cmd: &StepCommand,
        values: &[Value],
    ) -> Result<Vec<(Value, SimulationResult)>, SimulationError> {
        self.run_step_command_with_abort(netlist, step_cmd, values, &NoAbort)
    }

    /// Execute a parsed `.STEP` command with cooperative cancellation.
    pub fn run_step_command_with_abort(
        &self,
        netlist: &Netlist,
        step_cmd: &StepCommand,
        values: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<(Value, SimulationResult)>, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        self.ensure_batch_runs(values.len())?;
        if matches!(step_cmd.sweep, StepSweep::Data { .. }) {
            let stepped_netlists =
                self.step_netlists_for_command_with_abort(netlist, step_cmd, values, abort)?;
            let mut results = Vec::with_capacity(stepped_netlists.len());
            let mut retained_values = 0usize;
            for (index, stepped_netlist) in stepped_netlists {
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                match self.run_dc_op_with_abort(&stepped_netlist, abort) {
                    Ok(result) => {
                        self.push_step_result(&mut results, &mut retained_values, index, result)?
                    }
                    Err(e) => return Err(step_point_error("DATA", &step_cmd.name, index, e)),
                }
            }
            return Ok(results);
        }

        match step_cmd.target {
            StepTarget::Param => self.run_step_with_abort(netlist, &step_cmd.name, values, abort),
            StepTarget::Device => self.run_step_device(
                netlist,
                &step_cmd.name,
                step_cmd.param_name.as_deref(),
                values,
                abort,
            ),
            StepTarget::Model => self.run_step_model(
                netlist,
                &step_cmd.name,
                step_cmd.param_name.as_deref(),
                values,
                abort,
            ),
            StepTarget::Temp => self.run_step_temp(netlist, values, abort),
        }
    }

    pub(crate) fn step_netlists_for_command(
        &self,
        netlist: &Netlist,
        step_cmd: &StepCommand,
        values: &[Value],
    ) -> Result<Vec<(Value, Netlist)>, SimulationError> {
        self.step_netlists_for_command_with_abort(netlist, step_cmd, values, &NoAbort)
    }

    fn step_netlists_for_command_with_abort(
        &self,
        netlist: &Netlist,
        step_cmd: &StepCommand,
        values: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<(Value, Netlist)>, SimulationError> {
        if let StepSweep::Data { table_name } = &step_cmd.sweep {
            return self.data_step_netlists_for_table(netlist, table_name, abort);
        }

        validate_step_values(values)?;
        self.ensure_batch_runs(values.len())?;

        let mut stepped = Vec::with_capacity(values.len());
        let mut any_binding = false;
        for &value in values {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let (netlist, bindings) =
                self.step_netlist_for_command_value(netlist, step_cmd, value, abort)?;
            any_binding |= bindings > 0;
            stepped.push((value, netlist));
        }

        if step_cmd.target == StepTarget::Param && netlist.source_text.is_some() && !any_binding {
            return Err(SimulationError::Circuit(format!(
                "Parameter '{}' is not bound to any netlist expression",
                step_cmd.name
            )));
        }

        Ok(stepped)
    }

    fn data_step_netlists_for_table(
        &self,
        netlist: &Netlist,
        table_name: &str,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<(Value, Netlist)>, SimulationError> {
        let table = Self::validated_data_step_table(netlist, table_name, abort)?;
        self.ensure_batch_runs(table.rows.len())?;

        let mut stepped = Vec::with_capacity(table.rows.len());
        for (row_index, row) in table.rows.iter().enumerate() {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let overrides = table
                .params
                .iter()
                .cloned()
                .zip(row.iter().copied())
                .collect::<Vec<_>>();
            let (netlist, _) = Self::create_perturbed_netlist_multi_with_limits_and_abort(
                netlist,
                &overrides,
                self.config.resource_limits,
                abort,
            )?;
            stepped.push((row_index as Value, netlist));
        }

        Ok(stepped)
    }

    fn validated_data_step_table<'a>(
        netlist: &'a Netlist,
        table_name: &str,
        abort: &dyn AbortSignal,
    ) -> Result<&'a crate::netlist::DataTable, SimulationError> {
        let table = netlist
            .data_tables
            .iter()
            .find(|table| table.name.eq_ignore_ascii_case(table_name))
            .ok_or_else(|| {
                SimulationError::Circuit(format!(".STEP DATA table '{table_name}' not found"))
            })?;
        if table.params.is_empty() {
            return Err(SimulationError::Circuit(format!(
                ".STEP DATA table '{}' has no parameter columns",
                table.name
            )));
        }
        if table.rows.is_empty() {
            return Err(SimulationError::Circuit(format!(
                ".STEP DATA table '{}' has no rows",
                table.name
            )));
        }

        for (row_index, row) in table.rows.iter().enumerate() {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            if row.len() != table.params.len() {
                return Err(SimulationError::Circuit(format!(
                    ".STEP DATA table '{}' row {} has {} value(s), expected {}",
                    table.name,
                    row_index,
                    row.len(),
                    table.params.len()
                )));
            }
            for (column_index, value) in row.iter().enumerate() {
                if column_index.is_multiple_of(64) && abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                if !value.is_finite() {
                    return Err(SimulationError::Circuit(format!(
                        ".STEP DATA table '{}' row {} contains non-finite value {}",
                        table.name, row_index, value
                    )));
                }
            }
        }
        Ok(table)
    }

    fn copy_data_step_coordinates(
        table: &crate::netlist::DataTable,
        abort: &dyn AbortSignal,
    ) -> Result<(Vec<String>, Vec<Vec<Value>>), SimulationError> {
        let mut params = Vec::with_capacity(table.params.len());
        for (index, param) in table.params.iter().enumerate() {
            if index.is_multiple_of(64) && abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            params.push(param.clone());
        }

        let mut rows = Vec::with_capacity(table.rows.len());
        let mut cell_index = 0usize;
        for row in &table.rows {
            let mut copied_row = Vec::with_capacity(row.len());
            for value in row {
                if cell_index.is_multiple_of(64) && abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                copied_row.push(*value);
                cell_index += 1;
            }
            rows.push(copied_row);
        }
        Ok((params, rows))
    }

    fn step_netlist_for_command_value(
        &self,
        netlist: &Netlist,
        step_cmd: &StepCommand,
        value: Value,
        abort: &dyn AbortSignal,
    ) -> Result<(Netlist, usize), SimulationError> {
        match step_cmd.target {
            StepTarget::Param => Self::create_perturbed_netlist_with_limits_and_abort(
                netlist,
                &step_cmd.name,
                value,
                self.config.resource_limits,
                abort,
            ),
            StepTarget::Device => {
                let mut stepped = netlist.clone();
                Self::apply_ast_step_value_in_place(&mut stepped, step_cmd, value)?;
                Self::mark_ast_stepped_netlist(&mut stepped);
                Ok((stepped, 1))
            }
            StepTarget::Model => {
                let mut stepped = netlist.clone();
                Self::apply_ast_step_value_in_place(&mut stepped, step_cmd, value)?;
                Self::mark_ast_stepped_netlist(&mut stepped);
                Ok((stepped, 1))
            }
            StepTarget::Temp => self.step_temperature_netlist(netlist, value, abort),
        }
    }

    fn apply_ast_step_value_in_place(
        netlist: &mut Netlist,
        step_cmd: &StepCommand,
        value: Value,
    ) -> Result<(), SimulationError> {
        match step_cmd.target {
            StepTarget::Device if step_cmd.name.contains(':') => {
                Self::apply_hierarchical_device_step_value(
                    netlist,
                    &step_cmd.name,
                    step_cmd.param_name.as_deref(),
                    value,
                )
            }
            StepTarget::Device => {
                match Self::resolve_device_or_model_step_target(
                    netlist,
                    &step_cmd.name,
                    step_cmd.param_name.as_deref(),
                )? {
                    DeviceStepResolution::Device(device_idx) => {
                        let element = netlist.elements.get_mut(device_idx).ok_or_else(|| {
                            SimulationError::Circuit("Internal step device index error".to_string())
                        })?;
                        Self::apply_device_step_value(
                            &mut element.kind,
                            step_cmd.param_name.as_deref(),
                            value,
                        )
                    }
                    DeviceStepResolution::Model(model_idx) => {
                        let param_name = step_cmd.param_name.as_deref().ok_or_else(|| {
                            SimulationError::Circuit(format!(
                                ".STEP MODEL {} requires an explicit parameter name",
                                step_cmd.name
                            ))
                        })?;
                        let model = netlist.models.get_mut(model_idx).ok_or_else(|| {
                            SimulationError::Circuit("Internal step model index error".to_string())
                        })?;
                        Self::apply_model_step_value(model, param_name, value);
                        Ok(())
                    }
                }
            }
            StepTarget::Model => {
                let param_name = step_cmd.param_name.as_deref().ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        ".STEP MODEL {} requires an explicit parameter name",
                        step_cmd.name
                    ))
                })?;
                let model = Self::find_step_model_mut(netlist, &step_cmd.name)?;
                Self::apply_model_step_value(model, param_name, value);
                Ok(())
            }
            StepTarget::Param | StepTarget::Temp => Err(SimulationError::Circuit(
                "Internal non-AST .STEP binding reached AST materialization".to_string(),
            )),
        }
    }

    fn step_temperature_netlist(
        &self,
        netlist: &Netlist,
        value: Value,
        abort: &dyn AbortSignal,
    ) -> Result<(Netlist, usize), SimulationError> {
        let vt = thermal_voltage_celsius(value);
        let overrides = [
            ("TEMP".to_string(), value),
            ("TEMPER".to_string(), value),
            ("VT".to_string(), vt),
        ];
        let (mut stepped, bindings) = Self::create_perturbed_netlist_multi_with_limits_and_abort(
            netlist,
            &overrides,
            self.config.resource_limits,
            abort,
        )?;
        apply_temperature_scalars(&mut stepped, value, vt);
        Ok((stepped, bindings.max(1)))
    }

    pub(in crate::engine::advanced) fn run_step_temp(
        &self,
        netlist: &Netlist,
        values: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<(Value, SimulationResult)>, SimulationError> {
        if values.is_empty() {
            return Ok(Vec::new());
        }
        self.ensure_batch_runs(values.len())?;

        let mut results = Vec::with_capacity(values.len());
        let mut retained_values = 0usize;
        for &value in values {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let (stepped, _) = self.step_temperature_netlist(netlist, value, abort)?;

            match self.run_dc_op_with_abort(&stepped, abort) {
                Ok(result) => {
                    self.push_step_result(&mut results, &mut retained_values, value, result)?
                }
                Err(error @ SimulationError::Aborted)
                | Err(error @ SimulationError::ResourceLimit(_))
                | Err(error @ SimulationError::Configuration(_)) => return Err(error),
                Err(e) => {
                    log::warn!("Step TEMP = {} failed: {}", value, e);
                }
            }
        }

        Ok(results)
    }

    pub(in crate::engine::advanced) fn run_step_device(
        &self,
        netlist: &Netlist,
        device_name: &str,
        param_name: Option<&str>,
        values: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<(Value, SimulationResult)>, SimulationError> {
        validate_step_values(values)?;
        self.ensure_batch_runs(values.len())?;

        if device_name.contains(':') {
            let mut results = Vec::with_capacity(values.len());
            let mut retained_values = 0usize;
            for &value in values {
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                let stepped = Self::hierarchical_device_step_netlist(
                    netlist,
                    device_name,
                    param_name,
                    value,
                )?;
                match self.run_dc_op_with_abort(&stepped, abort) {
                    Ok(result) => {
                        self.push_step_result(&mut results, &mut retained_values, value, result)?
                    }
                    Err(error) => {
                        let target = format!(
                            "{}{}",
                            device_name,
                            param_name
                                .map(|name| format!(":{name}"))
                                .unwrap_or_default()
                        );
                        return Err(step_point_error("DEVICE", &target, value, error));
                    }
                }
            }
            return Ok(results);
        }

        let resolved = Self::resolve_device_or_model_step_target(netlist, device_name, param_name)?;

        let mut results = Vec::with_capacity(values.len());
        let mut retained_values = 0usize;
        for &value in values {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let mut stepped = netlist.clone();
            let target = format!(
                "{}{}",
                device_name,
                param_name.map(|p| format!(".{}", p)).unwrap_or_default()
            );
            match resolved {
                DeviceStepResolution::Device(device_idx) => {
                    let element = stepped.elements.get_mut(device_idx).ok_or_else(|| {
                        SimulationError::Circuit("Internal step device index error".to_string())
                    })?;
                    Self::apply_device_step_value(&mut element.kind, param_name, value)
                        .map_err(|e| step_point_error("DEVICE", &target, value, e))?;
                }
                DeviceStepResolution::Model(model_idx) => {
                    let param_name = param_name.ok_or_else(|| {
                        SimulationError::Circuit(format!(
                            ".STEP MODEL {device_name} requires an explicit parameter name"
                        ))
                    })?;
                    let model = stepped.models.get_mut(model_idx).ok_or_else(|| {
                        SimulationError::Circuit("Internal step model index error".to_string())
                    })?;
                    Self::apply_model_step_value(model, param_name, value);
                }
            }
            Self::mark_ast_stepped_netlist(&mut stepped);

            match self.run_dc_op_with_abort(&stepped, abort) {
                Ok(result) => {
                    self.push_step_result(&mut results, &mut retained_values, value, result)?
                }
                Err(e) => {
                    return Err(step_point_error("DEVICE", &target, value, e));
                }
            }
        }

        Ok(results)
    }

    pub(in crate::engine::advanced) fn run_step_model(
        &self,
        netlist: &Netlist,
        model_name: &str,
        param_name: Option<&str>,
        values: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<(Value, SimulationResult)>, SimulationError> {
        validate_step_values(values)?;
        self.ensure_batch_runs(values.len())?;

        let param_name = param_name.ok_or_else(|| {
            SimulationError::Circuit(format!(
                ".STEP MODEL {} requires an explicit parameter name",
                model_name
            ))
        })?;

        let mut results = Vec::with_capacity(values.len());
        let mut retained_values = 0usize;
        for &value in values {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let mut stepped = netlist.clone();
            let model = Self::find_step_model_mut(&mut stepped, model_name)?;
            Self::apply_model_step_value(model, param_name, value);
            Self::mark_ast_stepped_netlist(&mut stepped);

            match self.run_dc_op_with_abort(&stepped, abort) {
                Ok(result) => {
                    self.push_step_result(&mut results, &mut retained_values, value, result)?
                }
                Err(e) => {
                    let target = format!("{}.{}", model_name, param_name.to_ascii_uppercase());
                    return Err(step_point_error("MODEL", &target, value, e));
                }
            }
        }

        Ok(results)
    }

    fn resolve_device_or_model_step_target(
        netlist: &Netlist,
        target_name: &str,
        param_name: Option<&str>,
    ) -> Result<DeviceStepResolution, SimulationError> {
        if let Some(device_idx) = netlist
            .elements
            .iter()
            .position(|e| e.name.eq_ignore_ascii_case(target_name))
        {
            return Ok(DeviceStepResolution::Device(device_idx));
        }

        if param_name.is_some()
            && let Some(model_idx) = netlist
                .models
                .iter()
                .position(|m| m.name.eq_ignore_ascii_case(target_name))
        {
            return Ok(DeviceStepResolution::Model(model_idx));
        }

        Err(SimulationError::Circuit(format!(
            ".STEP DEVICE target '{}' not found in netlist",
            target_name
        )))
    }

    fn hierarchical_device_step_netlist(
        netlist: &Netlist,
        target_name: &str,
        param_name: Option<&str>,
        value: Value,
    ) -> Result<Netlist, SimulationError> {
        let mut stepped = netlist.clone();
        Self::apply_hierarchical_device_step_value(&mut stepped, target_name, param_name, value)?;
        Self::mark_ast_stepped_netlist(&mut stepped);
        Ok(stepped)
    }

    fn apply_hierarchical_device_step_value(
        stepped: &mut Netlist,
        target_name: &str,
        param_name: Option<&str>,
        value: Value,
    ) -> Result<(), SimulationError> {
        let segments = target_name.split(':').collect::<Vec<_>>();
        if segments.len() < 2 || segments.iter().any(|segment| segment.is_empty()) {
            return Err(SimulationError::Circuit(format!(
                ".STEP hierarchical DEVICE target '{target_name}' is malformed"
            )));
        }

        let root_index = stepped
            .elements
            .iter()
            .position(|element| element.name.eq_ignore_ascii_case(segments[0]))
            .ok_or_else(|| {
                SimulationError::Circuit(format!(
                    ".STEP hierarchical DEVICE root instance '{}' not found in netlist",
                    segments[0]
                ))
            })?;
        let root_subcircuit = match &stepped.elements[root_index].kind {
            ElementKind::Subcircuit { subckt_name, .. } => subckt_name.clone(),
            _ => {
                return Err(SimulationError::Circuit(format!(
                    ".STEP hierarchical DEVICE root '{}' is not a subcircuit instance",
                    segments[0]
                )));
            }
        };

        let (specialized_root, specialized_definitions) = Self::specialize_step_subcircuit(
            &stepped.subcircuits,
            &root_subcircuit,
            &segments[1..],
            target_name,
            param_name,
            value,
            0,
        )?;
        if let ElementKind::Subcircuit { subckt_name, .. } = &mut stepped.elements[root_index].kind
        {
            *subckt_name = specialized_root;
        }
        stepped.subcircuits.extend(specialized_definitions);
        Ok(())
    }

    fn specialize_step_subcircuit(
        definitions: &[crate::netlist::SubcircuitDef],
        subcircuit_name: &str,
        path: &[&str],
        full_target: &str,
        param_name: Option<&str>,
        value: Value,
        depth: usize,
    ) -> Result<(String, Vec<crate::netlist::SubcircuitDef>), SimulationError> {
        let original = definitions
            .iter()
            .find(|definition| definition.name.eq_ignore_ascii_case(subcircuit_name))
            .ok_or_else(|| {
                SimulationError::Circuit(format!(
                    ".STEP hierarchical DEVICE target '{full_target}' references undefined subcircuit '{subcircuit_name}'"
                ))
            })?;
        let target = path.first().copied().ok_or_else(|| {
            SimulationError::Circuit(format!(
                ".STEP hierarchical DEVICE target '{full_target}' has no leaf device"
            ))
        })?;
        let mut specialized = original.clone();
        let element_index = specialized
            .elements
            .iter()
            .position(|element| element.name.eq_ignore_ascii_case(target))
            .ok_or_else(|| {
                SimulationError::Circuit(format!(
                    ".STEP hierarchical DEVICE target '{full_target}' cannot find '{target}' inside subcircuit '{}'",
                    original.name
                ))
            })?;

        let mut generated_definitions = Vec::new();
        if path.len() == 1 {
            Self::apply_device_step_value(
                &mut specialized.elements[element_index].kind,
                param_name,
                value,
            )?;
        } else {
            let child_subcircuit = match &specialized.elements[element_index].kind {
                ElementKind::Subcircuit { subckt_name, .. } => subckt_name.clone(),
                _ => {
                    return Err(SimulationError::Circuit(format!(
                        ".STEP hierarchical DEVICE path component '{target}' in '{full_target}' is not a subcircuit instance"
                    )));
                }
            };
            let (specialized_child, mut child_definitions) = Self::specialize_step_subcircuit(
                definitions,
                &child_subcircuit,
                &path[1..],
                full_target,
                param_name,
                value,
                depth + 1,
            )?;
            if let ElementKind::Subcircuit { subckt_name, .. } =
                &mut specialized.elements[element_index].kind
            {
                *subckt_name = specialized_child;
            }
            generated_definitions.append(&mut child_definitions);
        }

        let sanitized_target = full_target
            .chars()
            .map(|character| {
                if character.is_ascii_alphanumeric() {
                    character.to_ascii_uppercase()
                } else {
                    '_'
                }
            })
            .collect::<String>();
        let specialized_name =
            format!("__RSPICE_STEP_{sanitized_target}_{depth}_{}", original.name);
        if definitions
            .iter()
            .chain(generated_definitions.iter())
            .any(|definition| definition.name.eq_ignore_ascii_case(&specialized_name))
        {
            return Err(SimulationError::Circuit(format!(
                ".STEP hierarchical DEVICE specialization name '{specialized_name}' collides with an existing subcircuit"
            )));
        }
        specialized.name = specialized_name.clone();
        generated_definitions.push(specialized);
        Ok((specialized_name, generated_definitions))
    }

    fn find_step_model_mut<'a>(
        netlist: &'a mut Netlist,
        model_name: &str,
    ) -> Result<&'a mut ModelDef, SimulationError> {
        netlist
            .models
            .iter_mut()
            .find(|model| model.name.eq_ignore_ascii_case(model_name))
            .ok_or_else(|| {
                SimulationError::Circuit(format!(
                    ".STEP MODEL target '{}' not found in netlist",
                    model_name
                ))
            })
    }

    fn find_step_model<'a>(
        netlist: &'a Netlist,
        model_name: &str,
    ) -> Result<&'a ModelDef, SimulationError> {
        netlist
            .models
            .iter()
            .find(|model| model.name.eq_ignore_ascii_case(model_name))
            .ok_or_else(|| {
                SimulationError::Circuit(format!(
                    ".STEP MODEL target '{}' not found in netlist",
                    model_name
                ))
            })
    }

    fn apply_model_step_value(model: &mut ModelDef, param_name: &str, value: Value) {
        let param_upper = param_name.to_ascii_uppercase();
        if let Some((_, existing)) = model
            .params
            .iter_mut()
            .find(|(name, _)| name.eq_ignore_ascii_case(&param_upper))
        {
            *existing = value;
        } else {
            model.params.push((param_upper, value));
        }
    }

    fn mark_ast_stepped_netlist(netlist: &mut Netlist) {
        netlist.source_text = None;
        netlist.source_path = None;
    }

    pub(in crate::engine::advanced) fn apply_device_step_value(
        kind: &mut ElementKind,
        param_name: Option<&str>,
        value: Value,
    ) -> Result<(), SimulationError> {
        let param_upper = param_name.map(|p| p.trim().to_ascii_uppercase());
        let matches_param = |aliases: &[&str]| -> bool {
            match &param_upper {
                None => true,
                Some(name) => aliases.iter().any(|alias| name.eq_ignore_ascii_case(alias)),
            }
        };
        let set_instance_param =
            |instance_params: &mut Vec<(String, Value)>, name: &str, value: Value| {
                if let Some((_, existing)) = instance_params
                    .iter_mut()
                    .find(|(param, _)| param.eq_ignore_ascii_case(name))
                {
                    *existing = value;
                } else {
                    instance_params.push((name.to_ascii_uppercase(), value));
                }
            };
        let set_instance_param_alias = |instance_params: &mut Vec<(String, Value)>,
                                        aliases: &[&str],
                                        canonical_name: &str,
                                        value: Value| {
            if let Some((name, existing)) = instance_params.iter_mut().find(|(param, _)| {
                aliases
                    .iter()
                    .any(|alias| param.eq_ignore_ascii_case(alias))
            }) {
                *name = canonical_name.to_ascii_uppercase();
                *existing = value;
            } else {
                instance_params.push((canonical_name.to_ascii_uppercase(), value));
            }
        };

        match kind {
            ElementKind::Resistor {
                value: r,
                model,
                instance_params,
                ..
            } => {
                match param_upper.as_deref() {
                    None | Some("R") | Some("VALUE") => {
                        *r = value;
                        if model.is_some() {
                            set_instance_param(instance_params, "R", value);
                        }
                    }
                    Some(param_name) => {
                        set_instance_param(instance_params, param_name, value);
                    }
                }
                Ok(())
            }
            ElementKind::Capacitor {
                value: c,
                initial_voltage,
                instance_params,
                ..
            } => {
                match param_upper.as_deref() {
                    None | Some("C") | Some("CAP") | Some("VALUE") | Some("CAPACITANCE") => {
                        *c = value;
                        set_instance_param_alias(
                            instance_params,
                            &["C", "CAP", "VALUE", "CAPACITANCE"],
                            "C",
                            value,
                        );
                    }
                    Some("IC") => {
                        *initial_voltage = Some(value);
                        set_instance_param(instance_params, "IC", value);
                    }
                    Some("L") | Some("LENGTH") => {
                        set_instance_param_alias(instance_params, &["L", "LENGTH"], "L", value);
                    }
                    Some("W") | Some("WIDTH") => {
                        set_instance_param_alias(instance_params, &["W", "WIDTH"], "W", value);
                    }
                    Some("M") | Some("MULT") => {
                        set_instance_param_alias(instance_params, &["M", "MULT"], "M", value);
                    }
                    Some("SCALE") | Some("TEMP") | Some("DTEMP") | Some("TC1") | Some("TC2") => {
                        set_instance_param(
                            instance_params,
                            param_upper.as_deref().expect("param is present"),
                            value,
                        );
                    }
                    Some(_) => {
                        return Err(SimulationError::Circuit(
                            "Unsupported capacitor step parameter; use C, VALUE, CAP, CAPACITANCE, M, MULT, SCALE, L, W, TEMP, DTEMP, TC1, TC2, or IC"
                                .to_string(),
                        ));
                    }
                }
                Ok(())
            }
            ElementKind::Inductor {
                value: l,
                model,
                instance_params,
                ..
            } => {
                match param_upper.as_deref() {
                    None | Some("L") | Some("IND") | Some("VALUE") | Some("INDUCTANCE") => {
                        *l = value;
                        if model.is_some() {
                            set_instance_param(instance_params, "L", value);
                        }
                    }
                    Some("M") | Some("MULT") | Some("SCALE") | Some("TEMP") | Some("DTEMP")
                    | Some("TC1") | Some("TC2") => {
                        set_instance_param(
                            instance_params,
                            param_upper.as_deref().expect("param is present"),
                            value,
                        );
                    }
                    Some(_) => {
                        return Err(SimulationError::Circuit(
                            "Unsupported inductor step parameter; use L, VALUE, M, MULT, SCALE, TEMP, DTEMP, TC1, or TC2"
                                .to_string(),
                        ));
                    }
                }
                Ok(())
            }
            ElementKind::JilesAthertonInductor { value: l, .. } => {
                if !matches_param(&["L", "VALUE", "INDUCTANCE"]) {
                    return Err(SimulationError::Circuit(
                        "Unsupported Jiles-Atherton inductor step parameter; use L or VALUE"
                            .to_string(),
                    ));
                }
                *l = value;
                Ok(())
            }
            ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => {
                Self::set_source_step_value(spec, param_upper.as_deref(), value)
            }
            ElementKind::Vcvs {
                gain, gain_expr, ..
            }
            | ElementKind::Cccs {
                gain, gain_expr, ..
            } => {
                if !matches_param(&["GAIN", "VALUE"]) {
                    return Err(SimulationError::Circuit(
                        "Unsupported controlled-source step parameter; use GAIN".to_string(),
                    ));
                }
                *gain = value;
                *gain_expr = None;
                Ok(())
            }
            ElementKind::Vccs {
                transconductance,
                transconductance_expr,
                ..
            } => {
                if !matches_param(&["GM", "TRANSCONDUCTANCE", "VALUE"]) {
                    return Err(SimulationError::Circuit(
                        "Unsupported VCCS step parameter; use GM".to_string(),
                    ));
                }
                *transconductance = value;
                *transconductance_expr = None;
                Ok(())
            }
            ElementKind::Ccvs {
                transresistance,
                transresistance_expr,
                ..
            } => {
                if !matches_param(&["RM", "TRANSRESISTANCE", "VALUE"]) {
                    return Err(SimulationError::Circuit(
                        "Unsupported CCVS step parameter; use RM".to_string(),
                    ));
                }
                *transresistance = value;
                *transresistance_expr = None;
                Ok(())
            }
            ElementKind::Coupling { coefficient, .. } => {
                if !matches_param(&["K", "COUPLING", "VALUE"]) {
                    return Err(SimulationError::Circuit(
                        "Unsupported coupling step parameter; use K".to_string(),
                    ));
                }
                *coefficient = value;
                Ok(())
            }
            ElementKind::TransmissionLine {
                z0, td, freq, nl, ..
            } => {
                match param_upper.as_deref() {
                    None => *z0 = Some(value),
                    Some("Z0") | Some("VALUE") => *z0 = Some(value),
                    Some("TD") => *td = Some(value),
                    Some("F") | Some("FREQ") => *freq = Some(value),
                    Some("NL") => *nl = Some(value),
                    Some(other) => {
                        return Err(SimulationError::Circuit(format!(
                            "Unsupported transmission-line step parameter '{}' (use Z0, TD, FREQ, NL)",
                            other
                        )));
                    }
                }
                Ok(())
            }
            _ => Err(SimulationError::Circuit(
                "Unsupported .STEP DEVICE target for this element type".to_string(),
            )),
        }
    }

    fn set_source_step_value(
        spec: &mut SourceSpec,
        param_name: Option<&str>,
        value: Value,
    ) -> Result<(), SimulationError> {
        match param_name {
            None | Some("DC") | Some("VALUE") => Self::set_source_dc_value(spec, value),
            Some(
                "VO" | "OFFSET" | "VA" | "AMPLITUDE" | "F" | "FREQ" | "FREQUENCY" | "THETA"
                | "DAMPING" | "PHASE",
            ) => Self::set_sin_source_parameter(spec, param_name.unwrap(), value),
            Some("TD") => Self::set_transient_delay_source_parameter(spec, value),
            Some("VHI" | "VLO" | "TR" | "TF" | "TSAMPLE") => {
                Self::set_pat_source_parameter(spec, param_name.unwrap(), value)
            }
            Some(other) => Err(SimulationError::Circuit(format!(
                "Unsupported source step parameter '{other}'; use DC, VALUE, SIN parameters VO, VA, FREQ, TD, THETA, PHASE, or PAT parameters VHI, VLO, TD, TR, TF, TSAMPLE"
            ))),
        }
    }

    fn set_transient_delay_source_parameter(
        spec: &mut SourceSpec,
        value: Value,
    ) -> Result<(), SimulationError> {
        match Self::set_sin_source_parameter(spec, "TD", value) {
            Ok(()) => Ok(()),
            Err(sin_error) => match Self::set_pat_source_parameter(spec, "TD", value) {
                Ok(()) => Ok(()),
                Err(_) => Err(sin_error),
            },
        }
    }

    fn set_sin_source_parameter(
        spec: &mut SourceSpec,
        param_name: &str,
        value: Value,
    ) -> Result<(), SimulationError> {
        if !value.is_finite() {
            return Err(SimulationError::Circuit(format!(
                "SIN source step parameter '{param_name}' must be finite, got {value}"
            )));
        }
        match spec {
            SourceSpec::RfPort { inner, .. } => {
                Self::set_sin_source_parameter(inner, param_name, value)
            }
            SourceSpec::DcTransient { transient, .. }
            | SourceSpec::DcAcTransient { transient, .. } => {
                Self::set_sin_source_parameter(transient, param_name, value)
            }
            SourceSpec::Sin {
                offset,
                amplitude,
                frequency,
                delay,
                damping,
                phase,
            } => {
                match param_name {
                    "VO" | "OFFSET" => *offset = value,
                    "VA" | "AMPLITUDE" => *amplitude = value,
                    "F" | "FREQ" | "FREQUENCY" => *frequency = value,
                    "TD" => *delay = value,
                    "THETA" | "DAMPING" => *damping = value,
                    "PHASE" => *phase = value.to_radians(),
                    _ => unreachable!("validated SIN parameter"),
                }
                Ok(())
            }
            _ => Err(SimulationError::Circuit(format!(
                "Source step parameter '{param_name}' requires a SIN source"
            ))),
        }
    }

    fn set_pat_source_parameter(
        spec: &mut SourceSpec,
        param_name: &str,
        value: Value,
    ) -> Result<(), SimulationError> {
        if !value.is_finite() {
            return Err(SimulationError::Circuit(format!(
                "PAT source step parameter '{param_name}' must be finite, got {value}"
            )));
        }
        match spec {
            SourceSpec::RfPort { inner, .. } => {
                Self::set_pat_source_parameter(inner, param_name, value)
            }
            SourceSpec::DcTransient { transient, .. }
            | SourceSpec::DcAcTransient { transient, .. } => {
                Self::set_pat_source_parameter(transient, param_name, value)
            }
            SourceSpec::Pat {
                vhi,
                vlo,
                delay,
                rise,
                fall,
                sample,
                ..
            } => {
                match param_name {
                    "VHI" => *vhi = value,
                    "VLO" => *vlo = value,
                    "TD" => *delay = value,
                    "TR" => {
                        if value <= 0.0 {
                            return Err(SimulationError::Circuit(
                                "PAT TR must be positive".to_string(),
                            ));
                        }
                        *rise = value;
                    }
                    "TF" => {
                        if value <= 0.0 {
                            return Err(SimulationError::Circuit(
                                "PAT TF must be positive".to_string(),
                            ));
                        }
                        *fall = value;
                    }
                    "TSAMPLE" => {
                        if value <= 0.0 {
                            return Err(SimulationError::Circuit(
                                "PAT TSAMPLE must be positive".to_string(),
                            ));
                        }
                        *sample = value;
                    }
                    _ => unreachable!("validated PAT parameter"),
                }
                Ok(())
            }
            _ => Err(SimulationError::Circuit(format!(
                "Source step parameter '{param_name}' requires a PAT source"
            ))),
        }
    }

    pub(in crate::engine::advanced) fn set_source_dc_value(
        spec: &mut SourceSpec,
        value: Value,
    ) -> Result<(), SimulationError> {
        match spec {
            SourceSpec::RfPort { inner, .. } => Self::set_source_dc_value(inner, value),
            SourceSpec::Dc(v) => {
                *v = value;
                Ok(())
            }
            SourceSpec::DcAc { dc_value, .. } => {
                *dc_value = value;
                Ok(())
            }
            SourceSpec::DcTransient { dc_value, .. } => {
                *dc_value = value;
                Ok(())
            }
            SourceSpec::DcAcTransient { dc_value, .. } => {
                *dc_value = value;
                Ok(())
            }
            _ => Err(SimulationError::Circuit(
                "Stepping source VALUE/DC is supported for DC, DC+AC, DC+transient, and DC+AC+transient source definitions only"
                    .to_string(),
            )),
        }
    }
}

fn apply_temperature_scalars(netlist: &mut Netlist, temp_c: Value, vt: Value) {
    netlist.options.temp = Some(temp_c);
    netlist.params.set("TEMP", temp_c);
    netlist.params.set("TEMPER", temp_c);
    netlist.params.set("VT", vt);
}

fn thermal_voltage_celsius(temp_c: Value) -> Value {
    crate::constants::thermal_voltage(crate::analysis::temperature::celsius_to_kelvin(temp_c))
}

fn step_point_error(
    target_kind: &str,
    target_name: &str,
    value: Value,
    error: SimulationError,
) -> SimulationError {
    match error {
        error @ SimulationError::Aborted
        | error @ SimulationError::ResourceLimit(_)
        | error @ SimulationError::Configuration(_) => error,
        error => SimulationError::Circuit(format!(
            ".STEP {target_kind} {target_name} = {value} failed: {error}"
        )),
    }
}

fn validate_step_values(values: &[Value]) -> Result<(), SimulationError> {
    if values.is_empty() {
        return Err(SimulationError::Circuit(
            ".STEP produced no sweep values".to_string(),
        ));
    }

    if let Some((index, value)) = values
        .iter()
        .enumerate()
        .find(|(_, value)| !value.is_finite())
    {
        return Err(SimulationError::Circuit(format!(
            ".STEP value at index {index} must be finite, got {value}"
        )));
    }

    Ok(())
}

fn checked_step_cardinality(
    current: usize,
    dimension_len: usize,
    max_runs: usize,
    dimension_index: usize,
) -> Result<usize, SimulationError> {
    if dimension_len == 0 {
        return Err(SimulationError::Circuit(format!(
            ".STEP dimension {} produced no sweep values",
            dimension_index + 1
        )));
    }
    let requested = current.checked_mul(dimension_len).ok_or_else(|| {
        SimulationError::Circuit(format!(
            ".STEP Cartesian run count overflows usize at dimension {}; configured limit is {max_runs}",
            dimension_index + 1
        ))
    })?;
    if requested > max_runs {
        return Err(SimulationError::Circuit(format!(
            ".STEP Cartesian expansion requests {requested} run(s), exceeding configured limit {max_runs}"
        )));
    }
    Ok(requested)
}

fn checked_step_resource_sum(
    current: usize,
    additional: usize,
    limit: usize,
    resource: &str,
) -> Result<usize, SimulationError> {
    let requested = current.checked_add(additional).ok_or_else(|| {
        SimulationError::Circuit(format!(".STEP {resource} count overflows usize"))
    })?;
    if requested > limit {
        return Err(SimulationError::Circuit(format!(
            ".STEP plan requests {requested} {resource}, exceeding configured limit {limit}"
        )));
    }
    Ok(requested)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_point_error_preserves_typed_control_plane_errors() {
        assert!(matches!(
            step_point_error("PARAM", "RLOAD", 1_000.0, SimulationError::Aborted),
            SimulationError::Aborted
        ));
        let resource = crate::resource::ResourceLimitError {
            resource: crate::resource::ResourceKind::ResultValues,
            requested: 2,
            limit: 1,
        };
        assert!(matches!(
            step_point_error(
                "PARAM",
                "RLOAD",
                1_000.0,
                SimulationError::ResourceLimit(resource),
            ),
            SimulationError::ResourceLimit(error) if error == resource
        ));

        let adversarial = step_point_error(
            "PARAM",
            "RLOAD",
            1_000.0,
            SimulationError::Circuit("Simulation aborted".to_string()),
        );
        match adversarial {
            SimulationError::Circuit(message) => {
                assert!(message.contains(".STEP PARAM RLOAD = 1000"));
                assert!(message.contains("Simulation aborted"));
            }
            other => panic!("string-like cancellation must remain a circuit error: {other:?}"),
        }
    }
    use crate::engine::{SimulationConfig, SpiceDialect};
    use crate::netlist::{AnalysisCommand, ElementKind, Netlist, SourceSpec};

    fn step_commands(netlist: &Netlist) -> Vec<StepCommand> {
        netlist
            .analyses
            .iter()
            .filter_map(|analysis| match analysis {
                AnalysisCommand::Step(step) => Some(step.clone()),
                _ => None,
            })
            .collect()
    }

    fn plan_limits(max_runs: usize) -> StepPlanLimits {
        StepPlanLimits::new(max_runs, 64, 256, 4096)
    }

    fn first_step_command(netlist: &Netlist) -> &StepCommand {
        netlist
            .analyses
            .iter()
            .find_map(|analysis| match analysis {
                AnalysisCommand::Step(step) => Some(step),
                _ => None,
            })
            .expect("step command parsed")
    }

    #[test]
    fn step_plan_is_checked_and_uses_xyce_mixed_radix_order() {
        let netlist = Netlist::parse(
            "step plan ordering\n\
             .param first=0 later=0\n\
             .step first list 1 2\n\
             .step later list 10 20 30\n\
             .end\n",
        )
        .expect("deck parses");
        let engine = Engine::new(SimulationConfig::default());
        let steps = step_commands(&netlist);
        let plan = engine
            .plan_step_commands(&netlist, &steps, plan_limits(6))
            .expect("six-run plan fits its explicit limit");

        assert_eq!(plan.total_runs(), 6);
        let coordinates = (0..plan.total_runs())
            .map(|index| plan.step_values(index).expect("planned coordinate exists"))
            .collect::<Vec<_>>();
        assert_eq!(
            coordinates,
            vec![
                vec![1.0, 10.0],
                vec![2.0, 10.0],
                vec![1.0, 20.0],
                vec![2.0, 20.0],
                vec![1.0, 30.0],
                vec![2.0, 30.0],
            ]
        );

        let limit_error = engine
            .plan_step_commands(&netlist, &steps, plan_limits(5))
            .expect_err("requested Cartesian count must be checked against the limit");
        let message = limit_error.to_string();
        assert!(message.contains("requests 6 run(s)"), "{message}");
        assert!(message.contains("limit 5"), "{message}");

        let zero_error = engine
            .plan_step_commands(&netlist, &steps, plan_limits(0))
            .expect_err("zero is not a usable execution limit");
        assert!(zero_error.to_string().contains("configured limit is 0"));
    }

    #[test]
    fn step_cardinality_overflow_is_rejected_without_allocation() {
        let error = checked_step_cardinality(usize::MAX, 2, usize::MAX, 1)
            .expect_err("overflow must fail before run allocation");
        assert!(error.to_string().contains("overflows usize"));
    }

    #[test]
    fn step_plan_limits_dimensions_bindings_and_stored_values_before_copying() {
        let netlist = Netlist::parse("resource limits\n.end\n").expect("deck parses");
        let many_dimensions = (0..5)
            .map(|index| StepCommand {
                target: StepTarget::Param,
                name: format!("p{index}"),
                param_name: None,
                sweep: StepSweep::List(vec![index as Value]),
            })
            .collect::<Vec<_>>();
        let engine = Engine::new(SimulationConfig::default());
        let error = engine
            .plan_step_commands(&netlist, &many_dimensions, StepPlanLimits::new(1, 4, 8, 8))
            .expect_err("many one-point dimensions must respect max_dimensions");
        assert!(error.to_string().contains("max_dimensions 4"));

        let data_netlist = Netlist::parse(
            "wide DATA row\n\
             .data wide a b c d\n\
             1 2 3 4\n\
             .enddata\n\
             .step data=wide\n\
             .end\n",
        )
        .expect("wide DATA deck parses");
        let steps = step_commands(&data_netlist);
        let binding_error = engine
            .plan_step_commands(&data_netlist, &steps, StepPlanLimits::new(1, 1, 3, 4))
            .expect_err("one-row many-column DATA must respect per-run bindings");
        assert!(binding_error.to_string().contains("4 bindings per run"));

        let storage_error = engine
            .plan_step_commands(&data_netlist, &steps, StepPlanLimits::new(1, 1, 4, 3))
            .expect_err("DATA cells must respect stored-value limit");
        assert!(storage_error.to_string().contains("4 stored values"));

        let scalar = [StepCommand {
            target: StepTarget::Param,
            name: "p".to_string(),
            param_name: None,
            sweep: StepSweep::List(vec![1.0, 2.0, 3.0, 4.0]),
        }];
        let scalar_error = engine
            .plan_step_commands(&netlist, &scalar, StepPlanLimits::new(4, 1, 1, 3))
            .expect_err("scalar axes must be bounded before cloning");
        assert!(
            scalar_error
                .to_string()
                .contains("requests 4 stored value(s)")
        );
    }

    #[test]
    fn step_plan_canonicalizes_primary_device_parameter_aliases() {
        let netlist = Netlist::parse(
            "device aliases\n\
             V1 in 0 1\n\
             R1 in n1 10\n\
             C1 n1 n2 1u M=1\n\
             L1 n2 0 1m\n\
             .end\n",
        )
        .expect("device deck parses");
        let engine = Engine::new(SimulationConfig::default());
        let command = |name: &str, param_name: Option<&str>| StepCommand {
            target: StepTarget::Device,
            name: name.to_string(),
            param_name: param_name.map(str::to_string),
            sweep: StepSweep::List(vec![1.0]),
        };
        let alias_pairs = [
            [command("R1", None), command("R1", Some("R"))],
            [
                command("C1", Some("CAPACITANCE")),
                command("C1", Some("VALUE")),
            ],
            [command("L1", Some("IND")), command("L1", Some("VALUE"))],
            [command("V1", None), command("V1", Some("DC"))],
        ];
        for pair in alias_pairs {
            let error = engine
                .plan_step_commands(&netlist, &pair, StepPlanLimits::new(1, 2, 2, 2))
                .expect_err("aliases for one physical parameter must collide");
            assert!(
                error.to_string().contains("Duplicate .STEP target"),
                "{error}"
            );
        }

        let distinct = [command("C1", Some("C")), command("C1", Some("M"))];
        engine
            .plan_step_commands(&netlist, &distinct, StepPlanLimits::new(1, 2, 2, 2))
            .expect("distinct supported capacitor parameters must remain usable");
    }

    #[test]
    fn step_plan_materializes_combined_reparse_and_ast_bindings() {
        let netlist = Netlist::parse(
            "combined step materialization\n\
             .param gain=1 bias=1\n\
             V1 out 0 {gain+bias}\n\
             R1 out 0 10\n\
             .data samples bias\n\
             10\n\
             20\n\
             .enddata\n\
             .step gain list 2\n\
             .step data=samples\n\
             .step temp list 27\n\
             .step R1 list 100 200\n\
             .end\n",
        )
        .expect("deck parses");
        let engine = Engine::new(SimulationConfig::default());
        let steps = step_commands(&netlist);
        let plan = engine
            .plan_step_commands(&netlist, &steps, plan_limits(4))
            .expect("four-run plan fits");
        let run = engine
            .materialize_step_run(&plan, 3)
            .expect("selected mixed binding materializes");

        assert_eq!(run.run_index(), 3);
        assert_eq!(run.step_values(), &[2.0, 1.0, 27.0, 200.0]);
        assert_eq!(run.netlist().params.get("gain"), Some(2.0));
        assert_eq!(run.netlist().params.get("bias"), Some(20.0));
        assert_eq!(run.netlist().options.temp, Some(27.0));
        let source = run
            .netlist()
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case("V1"))
            .expect("V1 exists");
        match &source.kind {
            ElementKind::VoltageSource(spec) => {
                assert_eq!(crate::engine::extract_dc_value(spec), 22.0);
            }
            other => panic!("unexpected V1 kind: {other:?}"),
        }
        let resistor = run
            .netlist()
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case("R1"))
            .expect("R1 exists");
        match &resistor.kind {
            ElementKind::Resistor { value, .. } => assert_eq!(*value, 200.0),
            other => panic!("unexpected R1 kind: {other:?}"),
        }
    }

    #[test]
    fn step_plan_rejects_resolved_model_target_alias_collision() {
        let netlist = Netlist::parse(
            "model step collision\n\
             R1 out 0 RMOD L=1 W=1\n\
             .model RMOD R RSH=1\n\
             .step model RMOD RSH list 1 2\n\
             .step RMOD RSH list 3 4\n\
             .end\n",
        )
        .expect("deck parses");
        let engine = Engine::new(SimulationConfig::default());
        let error = engine
            .plan_step_commands(&netlist, &step_commands(&netlist), plan_limits(4))
            .expect_err("syntactic aliases for one model target must collide");
        let message = error.to_string();
        assert!(message.contains("Duplicate .STEP target"), "{message}");
        assert!(message.contains("MODEL:RMOD:RSH"), "{message}");
    }

    #[test]
    fn step_plan_rejects_data_device_collision_and_hierarchical_column() {
        let collision = Netlist::parse(
            "data device collision\n\
             R1 out 0 10\n\
             .data samples R1:VALUE\n\
             20\n\
             .enddata\n\
             .step data=samples\n\
             .step R1:R list 30\n\
             .end\n",
        )
        .expect("collision deck parses");
        let engine = Engine::new(SimulationConfig::default());
        let error = engine
            .plan_step_commands(&collision, &step_commands(&collision), plan_limits(1))
            .expect_err("DATA device override and explicit DEVICE target must collide");
        let message = error.to_string();
        assert!(message.contains("Duplicate .STEP target"), "{message}");
        assert!(message.contains("DEVICE:R1:R"), "{message}");

        let hierarchical = Netlist::parse(
            "hierarchical data device column\n\
             .data samples X1:R1:R\n\
             20\n\
             .enddata\n\
             .step data=samples\n\
             .end\n",
        )
        .expect("hierarchical-column deck parses");
        let error = engine
            .plan_step_commands(&hierarchical, &step_commands(&hierarchical), plan_limits(1))
            .expect_err("hierarchical DATA device columns must fail during planning");
        assert!(
            error
                .to_string()
                .contains("unsupported hierarchical device override")
        );
    }

    #[test]
    fn data_step_plan_materialization_does_not_rescan_all_rows() {
        let rows = (0..128)
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        let deck = format!(
            "data materialization polling\n\
             .param p=0\n\
             V1 out 0 {{p}}\n\
             .data samples p\n{rows}\n\
             .enddata\n\
             .step data=samples\n\
             .end\n"
        );
        let mut netlist = Netlist::parse(&deck).expect("deck parses");
        // Isolate planner-coordinate access from the legitimate O(deck) source
        // reparse; an AST-only base should not scan all DATA rows per run.
        netlist.source_text = None;
        let engine = Engine::new(SimulationConfig::default());
        let plan = engine
            .plan_step_commands(&netlist, &step_commands(&netlist), plan_limits(128))
            .expect("DATA plan builds");
        let abort = crate::abort_signal::CountingAbort::new(20);
        let run = engine
            .materialize_step_run_with_abort(&plan, 127, &abort)
            .expect("one DATA row materializes without rescanning the full table");
        assert_eq!(run.step_values(), &[127.0]);
        assert!(
            abort.count() < 20,
            "unexpectedly polled {} times",
            abort.count()
        );
    }

    #[test]
    fn step_plan_honors_typed_cancellation() {
        let netlist = Netlist::parse(
            "cancelled step plan\n\
             .param p=0\n\
             .step p list 1 2\n\
             .end\n",
        )
        .expect("deck parses");
        let engine = Engine::new(SimulationConfig::default());
        let steps = step_commands(&netlist);
        assert!(matches!(
            engine.plan_step_commands_with_abort(
                &netlist,
                &steps,
                plan_limits(2),
                &crate::abort_signal::ImmediateAbort,
            ),
            Err(SimulationError::Aborted)
        ));

        let delayed_abort = crate::abort_signal::CountingAbort::new(2);
        assert!(matches!(
            engine.plan_step_commands_with_abort(&netlist, &steps, plan_limits(2), &delayed_abort,),
            Err(SimulationError::Aborted)
        ));
        assert!(
            delayed_abort.count() >= 3,
            "cancellation should be observed during sweep-value generation"
        );
    }

    #[test]
    fn device_step_updates_capacitor_multiplicity_parameter() {
        let netlist = Netlist::parse(
            "\
capacitor m step
V1 in 0 0
C3 in 0 C=32u M=1.25
.STEP C3:M 1.25 2.5 1.25
.END
",
        )
        .expect("deck parses");
        let step = first_step_command(&netlist);
        let engine = Engine::new(SimulationConfig {
            spice_dialect: SpiceDialect::Xyce,
            ..SimulationConfig::default()
        });

        let stepped = engine
            .step_netlists_for_command(&netlist, step, &[2.5])
            .expect("capacitor multiplicity step materializes");
        let stepped_netlist = &stepped[0].1;
        let capacitance = engine
            .resolved_capacitor_value(stepped_netlist, "C3")
            .expect("capacitance resolves")
            .expect("C3 exists");

        assert!(
            (capacitance - 80.0e-6).abs() < 1.0e-18,
            "resolved capacitance {capacitance}"
        );
    }

    #[test]
    fn device_step_capacitor_value_replaces_existing_alias() {
        let netlist = Netlist::parse(
            "\
capacitor value alias step
V1 in 0 0
C3 in 0 C=32u M=2
.STEP C3:VALUE 16u 16u 1u
.END
",
        )
        .expect("deck parses");
        let step = first_step_command(&netlist);
        let engine = Engine::new(SimulationConfig {
            spice_dialect: SpiceDialect::Xyce,
            ..SimulationConfig::default()
        });

        let stepped = engine
            .step_netlists_for_command(&netlist, step, &[16.0e-6])
            .expect("capacitor value step materializes");
        let stepped_netlist = &stepped[0].1;
        let capacitance = engine
            .resolved_capacitor_value(stepped_netlist, "C3")
            .expect("capacitance resolves")
            .expect("C3 exists");

        assert!(
            (capacitance - 32.0e-6).abs() < 1.0e-18,
            "resolved capacitance {capacitance}"
        );
    }

    #[test]
    fn device_step_updates_sin_source_amplitude() {
        let netlist = Netlist::parse(
            "\
sin amplitude source step
VS1 out 0 SIN(0 1 1k 0 0)
.STEP VS1:VA 2 2 1
.END
",
        )
        .expect("deck parses");
        let step = first_step_command(&netlist);
        let engine = Engine::new(SimulationConfig {
            spice_dialect: SpiceDialect::Xyce,
            ..SimulationConfig::default()
        });

        let stepped = engine
            .step_netlists_for_command(&netlist, step, &[2.0])
            .expect("SIN amplitude step materializes");
        let source = stepped[0]
            .1
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case("VS1"))
            .expect("stepped source exists");

        match &source.kind {
            ElementKind::VoltageSource(SourceSpec::Sin { amplitude, .. }) => {
                assert!((*amplitude - 2.0).abs() < 1.0e-15);
            }
            other => panic!("unexpected stepped source kind: {other:?}"),
        }
    }

    #[test]
    fn device_step_updates_wrapped_sin_source_phase_in_degrees() {
        let netlist = Netlist::parse(
            "\
wrapped sin phase source step
I1 out 0 DC 0 SIN(0 1 1k 0 0 0)
.STEP I1:PHASE 90 90 1
.END
",
        )
        .expect("deck parses");
        let step = first_step_command(&netlist);
        let engine = Engine::new(SimulationConfig {
            spice_dialect: SpiceDialect::Xyce,
            ..SimulationConfig::default()
        });

        let stepped = engine
            .step_netlists_for_command(&netlist, step, &[90.0])
            .expect("wrapped SIN phase step materializes");
        let source = stepped[0]
            .1
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case("I1"))
            .expect("stepped source exists");

        match &source.kind {
            ElementKind::CurrentSource(SourceSpec::DcTransient { transient, .. }) => {
                match transient.as_ref() {
                    SourceSpec::Sin { phase, .. } => {
                        assert!((*phase - std::f64::consts::FRAC_PI_2).abs() < 1.0e-15);
                    }
                    other => panic!("unexpected transient source kind: {other:?}"),
                }
            }
            other => panic!("unexpected stepped source kind: {other:?}"),
        }
    }

    #[test]
    fn hierarchical_device_step_specializes_only_the_qualified_instance_path() {
        let netlist = Netlist::parse(
            "\
hierarchical device step
V1 a 0 0
V2 b 0 0
XTOPA a WRAP
XTOPB b WRAP
.SUBCKT LEAF p
RLOCAL p 0 1k
.ENDS LEAF
.SUBCKT WRAP p
XINNER p LEAF
.ENDS WRAP
.STEP XTOPA:XINNER:RLOCAL:R 2k 2k 1
.END
",
        )
        .expect("hierarchical step deck parses");
        let step = first_step_command(&netlist);
        assert_eq!(step.target, StepTarget::Device);
        assert_eq!(step.name, "XTOPA:XINNER:RLOCAL");
        assert_eq!(step.param_name.as_deref(), Some("R"));

        let engine = Engine::new(SimulationConfig {
            spice_dialect: SpiceDialect::Xyce,
            ..SimulationConfig::default()
        });
        let stepped = engine
            .step_netlists_for_command(&netlist, step, &[2.0e3])
            .expect("hierarchical step materializes");
        let flattened = crate::netlist::flatten_netlist_with_models(&stepped[0].1)
            .expect("specialized hierarchy flattens");
        let resistance = |name: &str| {
            flattened
                .elements
                .iter()
                .find(|element| element.name.eq_ignore_ascii_case(name))
                .and_then(|element| match &element.kind {
                    ElementKind::Resistor { value, .. } => Some(*value),
                    _ => None,
                })
                .unwrap_or_else(|| panic!("flattened resistor '{name}' exists"))
        };
        assert_eq!(resistance("XTOPA.XINNER.RLOCAL"), 2.0e3);
        assert_eq!(resistance("XTOPB.XINNER.RLOCAL"), 1.0e3);
        assert!(stepped[0].1.source_text.is_none());
    }

    #[test]
    fn hierarchical_device_step_rejects_empty_path_components() {
        let error = Netlist::parse(
            "\
malformed hierarchical device step
R1 1 0 1k
.STEP X1::R1:R 1k 2k 1k
.END
",
        )
        .expect_err("empty hierarchy path component must fail parsing");
        assert!(error.to_string().contains("device[:child...]:param"));
    }
}
