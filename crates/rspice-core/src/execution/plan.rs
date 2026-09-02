use crate::Value;
use crate::abort_signal::AbortSignal;
use crate::resource::{ResourceKind, ResourceLimitError, ResourceLimits};
use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};
use std::fmt;
use std::mem::size_of;

/// Physical or post-processing analysis identity used by every frontend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum AnalysisKind {
    ImplicitOp,
    Op,
    Dc,
    Ac,
    Tran,
    Noise,
    Sp,
    Stb,
    Distortion,
    PoleZero,
    Sensitivity,
    TransferFunction,
    Pss,
    Pac,
    PNoise,
    HarmonicBalance,
    Envelope,
    MonteCarlo,
    Fourier,
    Fft,
}

impl AnalysisKind {
    pub const fn tag(self) -> &'static str {
        match self {
            Self::ImplicitOp => "implicit-op",
            Self::Op => "op",
            Self::Dc => "dc",
            Self::Ac => "ac",
            Self::Tran => "tran",
            Self::Noise => "noise",
            Self::Sp => "sp",
            Self::Stb => "stb",
            Self::Distortion => "disto",
            Self::PoleZero => "pz",
            Self::Sensitivity => "sens",
            Self::TransferFunction => "tf",
            Self::Pss => "pss",
            Self::Pac => "pac",
            Self::PNoise => "pnoise",
            Self::HarmonicBalance => "hb",
            Self::Envelope => "env",
            Self::MonteCarlo => "mc",
            Self::Fourier => "four",
            Self::Fft => "fft",
        }
    }
}

/// Stable identity of one authored analysis card.
///
/// `ordinal` is zero-based in memory. Its external tag is one-based so a
/// repeated pair of `.AC` cards becomes `ac-001` and `ac-002`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnalysisInstanceId {
    kind: AnalysisKind,
    ordinal: u32,
}

impl AnalysisInstanceId {
    pub(crate) const fn new(kind: AnalysisKind, ordinal: u32) -> Self {
        Self { kind, ordinal }
    }

    pub const fn kind(self) -> AnalysisKind {
        self.kind
    }

    pub const fn ordinal(self) -> u32 {
        self.ordinal
    }

    pub fn tag(self) -> String {
        format!("{}-{:03}", self.kind.tag(), u64::from(self.ordinal) + 1)
    }
}

impl fmt::Display for AnalysisInstanceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.tag())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum AxisKind {
    Alter,
    Data,
    Step,
    Temperature,
}

/// Typed target changed by one authored numeric `.STEP` dimension.
///
/// DATA-backed steps are represented by [`AxisKind::Data`] and coupled
/// [`RunAxisValue::DataRow`] values instead because one row can bind several
/// global parameters at once.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum StepAxisTarget {
    /// Global parameter changed by `.STEP PARAM name ...`.
    Parameter {
        /// Case-normalized parameter name.
        name: String,
    },
    /// Device instance or device parameter changed by `.STEP`.
    Device {
        /// Case-normalized instance or hierarchical instance name.
        name: String,
        /// Optional case-normalized parameter; `None` selects the device's
        /// primary authored value.
        parameter: Option<String>,
    },
    /// Compact/passive model parameter changed by `.STEP MODEL`.
    Model {
        /// Case-normalized model name.
        name: String,
        /// Case-normalized model parameter name.
        parameter: String,
    },
    /// Circuit temperature changed by `.STEP TEMP`.
    Temperature,
}

impl StepAxisTarget {
    fn from_command(command: &crate::netlist::StepCommand) -> Result<Self, DeckPlanError> {
        use crate::netlist::StepTarget;

        match command.target {
            StepTarget::Param => Ok(Self::Parameter {
                name: normalize_step_identifier(&command.name, "parameter")?,
            }),
            StepTarget::Device => Ok(Self::Device {
                name: normalize_step_identifier(&command.name, "device")?,
                parameter: command
                    .param_name
                    .as_deref()
                    .map(|parameter| normalize_step_identifier(parameter, "device parameter"))
                    .transpose()?,
            }),
            StepTarget::Model => Ok(Self::Model {
                name: normalize_step_identifier(&command.name, "model")?,
                parameter: normalize_step_identifier(
                    command.param_name.as_deref().unwrap_or_default(),
                    "model parameter",
                )?,
            }),
            StepTarget::Temp => Ok(Self::Temperature),
        }
    }

    fn axis_name(&self) -> String {
        match self {
            Self::Parameter { name } => format!("param:{name}"),
            Self::Device {
                name,
                parameter: Some(parameter),
            } => format!("device:{name}:{parameter}"),
            Self::Device {
                name,
                parameter: None,
            } => format!("device:{name}"),
            Self::Model { name, parameter } => format!("model:{name}:{parameter}"),
            Self::Temperature => "temperature".to_string(),
        }
    }

    fn binding_name(&self) -> String {
        match self {
            Self::Parameter { name } => name.clone(),
            Self::Temperature => "temperature".to_string(),
            Self::Device { .. } | Self::Model { .. } => self.axis_name(),
        }
    }

    fn retained_dynamic_bytes(&self) -> Result<usize, DeckPlanError> {
        match self {
            Self::Parameter { name } => Ok(name.len()),
            Self::Device { name, parameter } => checked_resource_add(
                name.len(),
                parameter.as_ref().map_or(0, String::len),
                ResourceKind::ExpandedSourceBytes,
            ),
            Self::Model { name, parameter } => checked_resource_add(
                name.len(),
                parameter.len(),
                ResourceKind::ExpandedSourceBytes,
            ),
            Self::Temperature => Ok(0),
        }
    }
}

impl AxisKind {
    const fn tag(self) -> &'static str {
        match self {
            Self::Alter => "alter",
            Self::Data => "data",
            Self::Step => "step",
            Self::Temperature => "temp",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum RunAxisValue {
    Numeric(Value),
    DataRow(Vec<DataBinding>),
    AlterVariant {
        label: String,
        materialization_digest: [u8; 32],
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct DataBinding {
    name: String,
    value: Value,
}

impl DataBinding {
    pub fn new(name: impl Into<String>, value: Value) -> Result<Self, DeckPlanError> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(DeckPlanError::EmptyBindingName);
        }
        if !value.is_finite() {
            return Err(DeckPlanError::NonFiniteBindingValue {
                binding: name.trim().to_string(),
                value,
            });
        }
        Ok(Self {
            name: name.trim().to_ascii_lowercase(),
            value: if value == 0.0 { 0.0 } else { value },
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub const fn value(&self) -> Value {
        self.value
    }
}

impl RunAxisValue {
    fn validate(&self, axis_name: &str) -> Result<(), DeckPlanError> {
        match self {
            Self::Numeric(value) if !value.is_finite() => Err(DeckPlanError::NonFiniteAxisValue {
                axis: axis_name.to_string(),
                value: *value,
            }),
            Self::DataRow(bindings) if bindings.is_empty() => Err(DeckPlanError::EmptyDataRow {
                axis: axis_name.to_string(),
            }),
            Self::DataRow(bindings) => {
                let mut names = BTreeSet::new();
                for binding in bindings {
                    if !names.insert(binding.name.clone()) {
                        return Err(DeckPlanError::DuplicateDataBinding {
                            axis: axis_name.to_string(),
                            binding: binding.name.clone(),
                        });
                    }
                }
                Ok(())
            }
            Self::AlterVariant {
                label,
                materialization_digest,
            } if label.trim().is_empty()
                || materialization_digest.iter().all(|byte| *byte == 0) =>
            {
                Err(DeckPlanError::InvalidAlterVariant {
                    axis: axis_name.to_string(),
                })
            }
            _ => Ok(()),
        }
    }

    fn update_stable_hash(&self, hasher: &mut blake3::Hasher) {
        match self {
            Self::Numeric(value) => {
                hasher.update(b"n");
                hasher.update(&value.to_bits().to_le_bytes());
            }
            Self::DataRow(bindings) => {
                hasher.update(b"d");
                // Rows are normalized into binding-name order by `RunAxis::new`.
                for binding in bindings {
                    update_hash_field(hasher, binding.name.as_bytes());
                    hasher.update(&binding.value.to_bits().to_le_bytes());
                }
            }
            Self::AlterVariant {
                label,
                materialization_digest,
            } => {
                hasher.update(b"a");
                update_hash_field(hasher, label.as_bytes());
                hasher.update(materialization_digest);
            }
        }
    }

    fn retained_dynamic_bytes(&self) -> Result<usize, DeckPlanError> {
        match self {
            Self::Numeric(_) => Ok(0),
            Self::DataRow(bindings) => {
                let binding_storage = checked_resource_mul(
                    bindings.len(),
                    size_of::<DataBinding>(),
                    ResourceKind::ExpandedSourceBytes,
                )?;
                bindings.iter().try_fold(binding_storage, |bytes, binding| {
                    checked_resource_add(
                        bytes,
                        binding.name.len(),
                        ResourceKind::ExpandedSourceBytes,
                    )
                })
            }
            Self::AlterVariant { label, .. } => Ok(label.len()),
        }
    }

    fn normalized(self) -> Self {
        match self {
            Self::Numeric(0.0) => Self::Numeric(0.0),
            Self::DataRow(mut bindings) => {
                bindings.sort_by(|first, second| first.name.cmp(&second.name));
                Self::DataRow(bindings)
            }
            Self::AlterVariant {
                label,
                materialization_digest,
            } => Self::AlterVariant {
                label: label.trim().to_string(),
                materialization_digest,
            },
            other => other,
        }
    }

    fn data_binding_names(&self) -> Option<BTreeSet<String>> {
        match self {
            Self::DataRow(bindings) => Some(
                bindings
                    .iter()
                    .map(|binding| binding.name.clone())
                    .collect(),
            ),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RunAxis {
    kind: AxisKind,
    name: String,
    values: Vec<RunAxisValue>,
    step_target: Option<StepAxisTarget>,
}

impl RunAxis {
    pub fn new(
        kind: AxisKind,
        name: impl Into<String>,
        values: Vec<RunAxisValue>,
    ) -> Result<Self, DeckPlanError> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(DeckPlanError::EmptyAxisName);
        }
        if values.is_empty() {
            return Err(DeckPlanError::EmptyAxis {
                axis: name.trim().to_string(),
            });
        }
        for value in &values {
            value.validate(name.trim())?;
            let compatible = matches!(
                (kind, value),
                (AxisKind::Alter, RunAxisValue::AlterVariant { .. })
                    | (AxisKind::Data, RunAxisValue::DataRow(_))
                    | (
                        AxisKind::Step | AxisKind::Temperature,
                        RunAxisValue::Numeric(_)
                    )
            );
            if !compatible {
                return Err(DeckPlanError::AxisValueKind {
                    axis: name.trim().to_string(),
                    kind,
                });
            }
        }
        if kind == AxisKind::Data {
            let expected = values[0].data_binding_names().unwrap_or_default();
            if values
                .iter()
                .skip(1)
                .any(|value| value.data_binding_names().unwrap_or_default() != expected)
            {
                return Err(DeckPlanError::InconsistentDataColumns {
                    axis: name.trim().to_string(),
                });
            }
        }
        let values = values.into_iter().map(RunAxisValue::normalized).collect();
        Ok(Self {
            kind,
            name: name.trim().to_string(),
            values,
            step_target: None,
        })
    }

    fn from_step_command(
        command: &crate::netlist::StepCommand,
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Self, DeckPlanError> {
        let target = StepAxisTarget::from_command(command)?;
        let generated = command
            .sweep
            .values_bounded_with_abort(limits.max_batch_runs, abort)
            .map_err(|error| match error {
                crate::netlist::SweepPointGenerationError::Aborted => DeckPlanError::Aborted,
                crate::netlist::SweepPointGenerationError::LimitExceeded { requested, limit } => {
                    DeckPlanError::ResourceLimit(ResourceLimitError {
                        resource: ResourceKind::BatchRuns,
                        requested,
                        limit,
                    })
                }
            })?;
        let mut values = try_vec_with_capacity(generated.len(), "STEP run-axis values")?;
        values.extend(generated.into_iter().map(RunAxisValue::Numeric));
        let kind = if target == StepAxisTarget::Temperature {
            AxisKind::Temperature
        } else {
            AxisKind::Step
        };
        let mut axis = Self::new(kind, target.axis_name(), values)?;
        axis.step_target = Some(target);
        Ok(axis)
    }

    pub const fn kind(&self) -> AxisKind {
        self.kind
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn values(&self) -> &[RunAxisValue] {
        &self.values
    }

    /// Return the typed `.STEP` target that authored this axis.
    ///
    /// This is `None` for `.TEMP`, `.ALTER`, and DATA-row axes. A `.STEP TEMP`
    /// axis has [`AxisKind::Temperature`] and returns
    /// [`StepAxisTarget::Temperature`].
    pub const fn step_target(&self) -> Option<&StepAxisTarget> {
        self.step_target.as_ref()
    }

    fn identity(&self) -> (AxisKind, String) {
        (self.kind, self.name.to_ascii_lowercase())
    }

    fn binding_names(&self) -> BTreeSet<String> {
        if let Some(target) = &self.step_target {
            return [target.binding_name()].into_iter().collect();
        }
        match self.kind {
            AxisKind::Alter => BTreeSet::new(),
            AxisKind::Data => self.values[0].data_binding_names().unwrap_or_default(),
            AxisKind::Step => [self.name.to_ascii_lowercase()].into_iter().collect(),
            AxisKind::Temperature => ["temperature".to_string()].into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AxisAssignment {
    kind: AxisKind,
    name: String,
    value: RunAxisValue,
    value_index: usize,
    step_target: Option<StepAxisTarget>,
}

impl AxisAssignment {
    pub const fn kind(&self) -> AxisKind {
        self.kind
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub const fn value(&self) -> &RunAxisValue {
        &self.value
    }

    pub const fn value_index(&self) -> usize {
        self.value_index
    }

    /// Typed target for a coordinate produced from an authored numeric
    /// `.STEP` dimension.
    pub const fn step_target(&self) -> Option<&StepAxisTarget> {
        self.step_target.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RunCoordinate {
    ordinal: usize,
    stable_id: RunCoordinateId,
    assignments: Vec<AxisAssignment>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RunCoordinateId {
    semantic: [u8; 16],
    occurrence: u32,
}

impl RunCoordinateId {
    pub(crate) const fn from_parts(semantic: [u8; 16], occurrence: u32) -> Self {
        Self {
            semantic,
            occurrence,
        }
    }

    pub const fn semantic_bytes(self) -> [u8; 16] {
        self.semantic
    }

    pub const fn occurrence(self) -> u32 {
        self.occurrence
    }
}

impl fmt::Display for RunCoordinateId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.semantic {
            write!(formatter, "{byte:02x}")?;
        }
        write!(formatter, "-{:03}", u64::from(self.occurrence) + 1)?;
        Ok(())
    }
}

impl RunCoordinate {
    fn new(
        ordinal: usize,
        assignments: Vec<AxisAssignment>,
        semantic: [u8; 16],
        occurrence: u32,
    ) -> Self {
        Self {
            ordinal,
            stable_id: RunCoordinateId::from_parts(semantic, occurrence),
            assignments,
        }
    }

    pub const fn ordinal(&self) -> usize {
        self.ordinal
    }

    pub const fn stable_id(&self) -> RunCoordinateId {
        self.stable_id
    }

    pub fn stable_tag(&self) -> String {
        format!("run-{}", self.stable_id)
    }

    pub fn assignments(&self) -> &[AxisAssignment] {
        &self.assignments
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalysisRequest {
    kind: AnalysisKind,
}

impl AnalysisRequest {
    pub const fn new(kind: AnalysisKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> AnalysisKind {
        self.kind
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannedAnalysis {
    id: AnalysisInstanceId,
    request: AnalysisRequest,
}

impl PlannedAnalysis {
    pub const fn id(&self) -> AnalysisInstanceId {
        self.id
    }

    pub const fn request(&self) -> &AnalysisRequest {
        &self.request
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeckPlan {
    axes: Vec<RunAxis>,
    analyses: Vec<PlannedAnalysis>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CoordinateResourceEstimate {
    coordinate_count: usize,
    total_assignments: usize,
    retained_dynamic_bytes: usize,
}

impl DeckPlan {
    /// Build the complete target-neutral plan for a parsed deck.
    ///
    /// DATA rows are placed first, numeric `.STEP` dimensions remain in their
    /// authored relative order, and temperature dimensions are placed last.
    /// Physical analyses retain their authored order and per-kind ordinals.
    /// Textual ALTER expansion and coordinate-dependent topology/schema
    /// materialization happen in later planning stages and are intentionally
    /// not inferred here.
    pub fn from_netlist(
        netlist: &crate::netlist::Netlist,
        limits: &ResourceLimits,
    ) -> Result<Self, DeckPlanError> {
        Self::from_netlist_with_abort(netlist, limits, &crate::NoAbort)
    }

    /// Abort-aware form of [`Self::from_netlist`].
    pub fn from_netlist_with_abort(
        netlist: &crate::netlist::Netlist,
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Self, DeckPlanError> {
        use crate::netlist::{AnalysisCommand, StepSweep, StepTarget};

        if abort.is_aborted() {
            return Err(DeckPlanError::Aborted);
        }

        let data_axis_count = netlist
            .analyses
            .iter()
            .filter(|command| {
                matches!(command, AnalysisCommand::Step(step) if matches!(&step.sweep, StepSweep::Data { .. }))
            })
            .count();
        let step_axis_count = netlist
            .analyses
            .iter()
            .filter(|command| {
                matches!(command, AnalysisCommand::Step(step) if !matches!(&step.sweep, StepSweep::Data { .. }) && step.target != StepTarget::Temp)
            })
            .count();
        let temperature_axis_count = netlist
            .analyses
            .iter()
            .filter(|command| {
                matches!(command, AnalysisCommand::Temp { .. })
                    || matches!(command, AnalysisCommand::Step(step) if !matches!(&step.sweep, StepSweep::Data { .. }) && step.target == StepTarget::Temp)
            })
            .count();
        let meta_axis_count = data_axis_count
            .checked_add(step_axis_count)
            .and_then(|count| count.checked_add(temperature_axis_count))
            .ok_or(DeckPlanError::CoordinateCountOverflow)?;
        let mut data_axes = try_vec_with_capacity(data_axis_count, "DATA run axes")?;
        let mut step_axes = try_vec_with_capacity(step_axis_count, "STEP run axes")?;
        let mut temperature_axes =
            try_vec_with_capacity(temperature_axis_count, "temperature run axes")?;
        let mut analyses = try_vec_with_capacity(
            netlist.analyses.len().saturating_sub(meta_axis_count),
            "authored analysis requests",
        )?;

        for (command_index, command) in netlist.analyses.iter().enumerate() {
            if command_index.is_multiple_of(64) && abort.is_aborted() {
                return Err(DeckPlanError::Aborted);
            }
            match command {
                AnalysisCommand::Step(step) => match &step.sweep {
                    StepSweep::Data { table_name } => {
                        data_axes.push(data_axis_from_table(netlist, table_name, limits, abort)?)
                    }
                    _ => {
                        let axis = RunAxis::from_step_command(step, limits, abort)?;
                        if axis.kind == AxisKind::Temperature {
                            temperature_axes.push(axis);
                        } else {
                            step_axes.push(axis);
                        }
                    }
                },
                AnalysisCommand::Temp { temperatures } => temperature_axes.push(RunAxis::new(
                    AxisKind::Temperature,
                    "temperature",
                    try_copy_numeric_axis_values(temperatures, "temperature", abort)?,
                )?),
                AnalysisCommand::Four { .. } => {
                    // FOUR is attached to a transient result by the executor;
                    // it is not a physical analysis that suppresses implicit
                    // OP when it appears alone.
                }
                command => analyses.push(AnalysisRequest::new(analysis_kind(command))),
            }
        }
        if abort.is_aborted() {
            return Err(DeckPlanError::Aborted);
        }

        let mut axes = try_vec_with_capacity(meta_axis_count, "ordered run axes")?;
        axes.extend(data_axes);
        axes.extend(step_axes);
        axes.extend(temperature_axes);
        let plan = Self::new(axes, analyses)?;
        plan.preflight_coordinates(limits)?;
        if abort.is_aborted() {
            return Err(DeckPlanError::Aborted);
        }
        Ok(plan)
    }

    /// Compatibility query that returns a plan only when the deck contains a
    /// run axis. New executors should use [`Self::from_netlist`] so authored
    /// analysis ordinals also come from the canonical planner for scalar runs.
    pub fn from_netlist_run_axes(
        netlist: &crate::netlist::Netlist,
        limits: &ResourceLimits,
    ) -> Result<Option<Self>, DeckPlanError> {
        Self::from_netlist_run_axes_with_abort(netlist, limits, &crate::NoAbort)
    }

    /// Abort-aware form of [`Self::from_netlist_run_axes`].
    pub fn from_netlist_run_axes_with_abort(
        netlist: &crate::netlist::Netlist,
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Option<Self>, DeckPlanError> {
        let plan = Self::from_netlist_with_abort(netlist, limits, abort)?;
        Ok((!plan.axes.is_empty()).then_some(plan))
    }

    /// Compatibility bridge for callers that still request a standalone
    /// `.TEMP` axis. New executors should use [`Self::from_netlist`] so DATA,
    /// `.STEP`, and temperature axes share one checked Cartesian plan.
    pub fn from_netlist_temperature_axis(
        netlist: &crate::netlist::Netlist,
    ) -> Result<Option<Self>, DeckPlanError> {
        let has_temperature = netlist
            .analyses
            .iter()
            .any(|analysis| matches!(analysis, crate::netlist::AnalysisCommand::Temp { .. }));
        if !has_temperature {
            return Ok(None);
        }
        if netlist
            .analyses
            .iter()
            .any(|analysis| matches!(analysis, crate::netlist::AnalysisCommand::Step(_)))
        {
            return Err(DeckPlanError::TemperatureAxisWithStep);
        }
        Self::from_netlist_run_axes(netlist, &ResourceLimits::unlimited())
    }

    pub fn new(
        axes: Vec<RunAxis>,
        analysis_requests: Vec<AnalysisRequest>,
    ) -> Result<Self, DeckPlanError> {
        let mut axis_ids = BTreeSet::new();
        let mut binding_owners = std::collections::BTreeMap::<String, AxisKind>::new();
        let mut previous_axis_kind = None;
        for axis in &axes {
            if let Some(previous) = previous_axis_kind
                && axis.kind < previous
            {
                return Err(DeckPlanError::AxisOrder {
                    previous,
                    current: axis.kind,
                });
            }
            previous_axis_kind = Some(axis.kind);
            if !axis_ids.insert(axis.identity()) {
                return Err(DeckPlanError::DuplicateAxis {
                    kind: axis.kind,
                    axis: axis.name.clone(),
                });
            }
            for binding in axis.binding_names() {
                if let Some(first) = binding_owners.insert(binding.clone(), axis.kind) {
                    return Err(DeckPlanError::BindingCollision {
                        binding,
                        first,
                        second: axis.kind,
                    });
                }
            }
        }

        if analysis_requests
            .iter()
            .any(|request| request.kind == AnalysisKind::ImplicitOp)
        {
            return Err(DeckPlanError::ExplicitImplicitOp);
        }

        let requests = if analysis_requests.is_empty() {
            vec![AnalysisRequest::new(AnalysisKind::ImplicitOp)]
        } else {
            analysis_requests
        };
        let mut next_ordinals = std::collections::BTreeMap::<AnalysisKind, u32>::new();
        let mut analyses = Vec::new();
        analyses
            .try_reserve_exact(requests.len())
            .map_err(|_| DeckPlanError::Allocation {
                object: "planned analyses",
            })?;
        for request in requests {
            let ordinal = next_ordinals.entry(request.kind).or_default();
            let id = AnalysisInstanceId::new(request.kind, *ordinal);
            *ordinal = ordinal
                .checked_add(1)
                .ok_or(DeckPlanError::AnalysisCountOverflow(request.kind))?;
            analyses.push(PlannedAnalysis { id, request });
        }

        Ok(Self { axes, analyses })
    }

    pub fn axes(&self) -> &[RunAxis] {
        &self.axes
    }

    pub fn analyses(&self) -> &[PlannedAnalysis] {
        &self.analyses
    }

    fn coordinate_resource_estimate(&self) -> Result<CoordinateResourceEstimate, DeckPlanError> {
        let coordinate_count =
            checked_coordinate_count(self.axes.iter().map(|axis| axis.values.len()))?;
        let bindings_per_coordinate = self.axes.iter().try_fold(0usize, |count, axis| {
            let width = match axis.kind {
                AxisKind::Data => match axis.values.first() {
                    Some(RunAxisValue::DataRow(bindings)) => bindings.len(),
                    _ => 0,
                },
                AxisKind::Alter | AxisKind::Step | AxisKind::Temperature => 1,
            };
            checked_resource_add(count, width, ResourceKind::ResultValues)
        })?;
        let total_assignments = checked_resource_mul(
            coordinate_count,
            bindings_per_coordinate,
            ResourceKind::ResultValues,
        )?;

        let mut retained_dynamic_bytes = checked_resource_mul(
            coordinate_count,
            size_of::<RunCoordinate>(),
            ResourceKind::ExpandedSourceBytes,
        )?;
        for axis in &self.axes {
            // Every value from one axis appears the same number of times in a
            // Cartesian product. DATA rows remain one axis value, so their
            // coupled bindings never multiply one another.
            let repetitions = coordinate_count / axis.values.len();
            let assignment_storage = checked_resource_mul(
                axis.values.len(),
                size_of::<AxisAssignment>(),
                ResourceKind::ExpandedSourceBytes,
            )?;
            let cloned_axis_names = checked_resource_mul(
                axis.values.len(),
                axis.name.len(),
                ResourceKind::ExpandedSourceBytes,
            )?;
            let value_payloads = axis.values.iter().try_fold(0usize, |bytes, value| {
                checked_resource_add(
                    bytes,
                    value.retained_dynamic_bytes()?,
                    ResourceKind::ExpandedSourceBytes,
                )
            })?;
            let target_payloads = checked_resource_mul(
                axis.values.len(),
                axis.step_target
                    .as_ref()
                    .map(StepAxisTarget::retained_dynamic_bytes)
                    .transpose()?
                    .unwrap_or(0),
                ResourceKind::ExpandedSourceBytes,
            )?;
            let one_value_cycle = checked_resource_add(
                checked_resource_add(
                    assignment_storage,
                    cloned_axis_names,
                    ResourceKind::ExpandedSourceBytes,
                )?,
                checked_resource_add(
                    value_payloads,
                    target_payloads,
                    ResourceKind::ExpandedSourceBytes,
                )?,
                ResourceKind::ExpandedSourceBytes,
            )?;
            retained_dynamic_bytes = checked_resource_add(
                retained_dynamic_bytes,
                checked_resource_mul(
                    repetitions,
                    one_value_cycle,
                    ResourceKind::ExpandedSourceBytes,
                )?,
                ResourceKind::ExpandedSourceBytes,
            )?;
        }

        Ok(CoordinateResourceEstimate {
            coordinate_count,
            total_assignments,
            retained_dynamic_bytes,
        })
    }

    fn preflight_coordinates(
        &self,
        limits: &ResourceLimits,
    ) -> Result<CoordinateResourceEstimate, DeckPlanError> {
        let estimate = self.coordinate_resource_estimate()?;
        ResourceLimitError::ensure(
            ResourceKind::BatchRuns,
            estimate.coordinate_count,
            limits.max_batch_runs,
        )?;
        ResourceLimitError::ensure(
            ResourceKind::ResultValues,
            estimate.total_assignments,
            limits.max_result_values,
        )?;
        ResourceLimitError::ensure(
            ResourceKind::ExpandedSourceBytes,
            estimate.retained_dynamic_bytes,
            limits.max_expanded_source_bytes,
        )?;
        Ok(estimate)
    }

    /// Materialize Cartesian coordinates in declared axis order.
    ///
    /// The first axis varies fastest, matching Xyce's nested `.STEP` ordering
    /// and the established RSpice materialization contract. A plan without
    /// axes has exactly one coordinate, which prevents frontends from
    /// inventing separate implicit no-axis behavior.
    pub fn coordinates_with_abort(
        &self,
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<RunCoordinate>, DeckPlanError> {
        if abort.is_aborted() {
            return Err(DeckPlanError::Aborted);
        }
        let estimate = self.preflight_coordinates(limits)?;
        if abort.is_aborted() {
            return Err(DeckPlanError::Aborted);
        }

        let mut coordinates =
            try_vec_with_capacity(estimate.coordinate_count, "materialized run coordinates")?;
        if self.axes.is_empty() {
            let assignments = Vec::new();
            let semantic = coordinate_semantic_id(&assignments)?;
            coordinates.push(RunCoordinate::new(0, assignments, semantic, 0));
            return Ok(coordinates);
        }

        let mut indices = try_vec_with_capacity(self.axes.len(), "run-axis indices")?;
        indices.resize(self.axes.len(), 0usize);
        let mut occurrences = HashMap::<[u8; 16], u32>::new();
        occurrences
            .try_reserve(estimate.coordinate_count)
            .map_err(|_| DeckPlanError::Allocation {
                object: "coordinate occurrence identities",
            })?;
        for ordinal in 0..estimate.coordinate_count {
            if abort.is_aborted() {
                return Err(DeckPlanError::Aborted);
            }
            let mut assignments =
                try_vec_with_capacity(self.axes.len(), "coordinate axis assignments")?;
            for (axis, &value_index) in self.axes.iter().zip(&indices) {
                if abort.is_aborted() {
                    return Err(DeckPlanError::Aborted);
                }
                assignments.push(try_clone_assignment(axis, value_index)?);
            }
            let semantic = coordinate_semantic_id(&assignments)?;
            let occurrence = occurrences.entry(semantic).or_default();
            coordinates.push(RunCoordinate::new(
                ordinal,
                assignments,
                semantic,
                *occurrence,
            ));
            *occurrence = occurrence
                .checked_add(1)
                .ok_or(DeckPlanError::CoordinateCountOverflow)?;

            for axis_index in 0..indices.len() {
                indices[axis_index] += 1;
                if indices[axis_index] < self.axes[axis_index].values.len() {
                    break;
                }
                indices[axis_index] = 0;
            }
        }
        Ok(coordinates)
    }
}

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum DeckPlanError {
    EmptyAxisName,
    EmptyAxis {
        axis: String,
    },
    EmptyAxisValue {
        axis: String,
    },
    EmptyBindingName,
    EmptyDataRow {
        axis: String,
    },
    DuplicateDataBinding {
        axis: String,
        binding: String,
    },
    NonFiniteBindingValue {
        binding: String,
        value: Value,
    },
    InvalidAlterVariant {
        axis: String,
    },
    AxisValueKind {
        axis: String,
        kind: AxisKind,
    },
    InconsistentDataColumns {
        axis: String,
    },
    NonFiniteAxisValue {
        axis: String,
        value: Value,
    },
    InvalidStepTarget {
        target: &'static str,
    },
    UnknownStepDataTable {
        table: String,
    },
    AmbiguousStepDataTable {
        table: String,
    },
    StepDataRowWidth {
        table: String,
        row: usize,
        expected: usize,
        actual: usize,
    },
    DuplicateAxis {
        kind: AxisKind,
        axis: String,
    },
    BindingCollision {
        binding: String,
        first: AxisKind,
        second: AxisKind,
    },
    AxisOrder {
        previous: AxisKind,
        current: AxisKind,
    },
    ExplicitImplicitOp,
    TemperatureAxisWithStep,
    AnalysisCountOverflow(AnalysisKind),
    Allocation {
        object: &'static str,
    },
    ResourceLimit(ResourceLimitError),
    ResourceAccountingOverflow {
        resource: ResourceKind,
    },
    CoordinateCountOverflow,
    CoordinateIndex {
        axis: String,
        index: usize,
        value_count: usize,
    },
    Aborted,
}

impl fmt::Display for DeckPlanError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyAxisName => formatter.write_str("run axis name must not be empty"),
            Self::EmptyAxis { axis } => write!(formatter, "run axis '{axis}' has no values"),
            Self::EmptyAxisValue { axis } => {
                write!(formatter, "run axis '{axis}' contains an empty value")
            }
            Self::EmptyBindingName => formatter.write_str("data binding name must not be empty"),
            Self::EmptyDataRow { axis } => write!(formatter, "DATA axis '{axis}' has an empty row"),
            Self::DuplicateDataBinding { axis, binding } => write!(
                formatter,
                "DATA axis '{axis}' row binds '{binding}' more than once"
            ),
            Self::NonFiniteBindingValue { binding, value } => {
                write!(
                    formatter,
                    "DATA binding '{binding}' has non-finite value {value}"
                )
            }
            Self::InvalidAlterVariant { axis } => write!(
                formatter,
                "ALTER axis '{axis}' has an empty label or zero materialization digest"
            ),
            Self::AxisValueKind { axis, kind } => write!(
                formatter,
                "run axis '{axis}' contains a value incompatible with {kind:?}"
            ),
            Self::InconsistentDataColumns { axis } => write!(
                formatter,
                "DATA axis '{axis}' rows do not bind one consistent column set"
            ),
            Self::NonFiniteAxisValue { axis, value } => {
                write!(
                    formatter,
                    "run axis '{axis}' contains non-finite value {value}"
                )
            }
            Self::InvalidStepTarget { target } => {
                write!(formatter, ".STEP {target} target must not be empty")
            }
            Self::UnknownStepDataTable { table } => {
                write!(formatter, ".STEP DATA references unknown table '{table}'")
            }
            Self::AmbiguousStepDataTable { table } => write!(
                formatter,
                ".STEP DATA table name '{table}' is ambiguous under case-insensitive matching"
            ),
            Self::StepDataRowWidth {
                table,
                row,
                expected,
                actual,
            } => write!(
                formatter,
                ".STEP DATA table '{table}' row {row} contains {actual} value(s); expected {expected}"
            ),
            Self::DuplicateAxis { kind, axis } => {
                write!(formatter, "duplicate {kind:?} run axis '{axis}'")
            }
            Self::BindingCollision {
                binding,
                first,
                second,
            } => write!(
                formatter,
                "run binding '{binding}' is owned by both {first:?} and {second:?} axes"
            ),
            Self::AxisOrder { previous, current } => write!(
                formatter,
                "run axes are out of semantic order: {current:?} cannot follow {previous:?}"
            ),
            Self::ExplicitImplicitOp => formatter.write_str(
                "implicit operating point is planner-owned and cannot be authored explicitly",
            ),
            Self::TemperatureAxisWithStep => formatter.write_str(
                "the legacy temperature-only execution adapter cannot compose .TEMP with .STEP; use the canonical run-axis plan",
            ),
            Self::AnalysisCountOverflow(kind) => {
                write!(
                    formatter,
                    "too many authored {kind:?} analyses to assign stable IDs"
                )
            }
            Self::Allocation { object } => write!(formatter, "unable to allocate {object}"),
            Self::ResourceLimit(error) => fmt::Display::fmt(error, formatter),
            Self::ResourceAccountingOverflow { resource } => {
                write!(formatter, "run-plan {resource} accounting overflows usize")
            }
            Self::CoordinateCountOverflow => {
                formatter.write_str("run coordinate cardinality overflows usize")
            }
            Self::CoordinateIndex {
                axis,
                index,
                value_count,
            } => write!(
                formatter,
                "run axis '{axis}' coordinate index {index} is outside {value_count} values"
            ),
            Self::Aborted => formatter.write_str("deck planning aborted"),
        }
    }
}

impl std::error::Error for DeckPlanError {}

pub(super) fn analysis_kind(command: &crate::netlist::AnalysisCommand) -> AnalysisKind {
    use crate::netlist::AnalysisCommand;

    match command {
        AnalysisCommand::Op => AnalysisKind::Op,
        AnalysisCommand::Dc { .. } => AnalysisKind::Dc,
        AnalysisCommand::Tran { .. } => AnalysisKind::Tran,
        AnalysisCommand::Ac { .. } | AnalysisCommand::AcData { .. } => AnalysisKind::Ac,
        AnalysisCommand::Hb { .. } => AnalysisKind::HarmonicBalance,
        AnalysisCommand::Disto { .. } => AnalysisKind::Distortion,
        AnalysisCommand::Sp { .. } => AnalysisKind::Sp,
        AnalysisCommand::Noise { .. } | AnalysisCommand::NoiseData { .. } => AnalysisKind::Noise,
        AnalysisCommand::Tf { .. } => AnalysisKind::TransferFunction,
        AnalysisCommand::Stb { .. } => AnalysisKind::Stb,
        AnalysisCommand::PoleZero { .. } => AnalysisKind::PoleZero,
        AnalysisCommand::MonteCarlo(_) => AnalysisKind::MonteCarlo,
        AnalysisCommand::Sensitivity { .. } => AnalysisKind::Sensitivity,
        AnalysisCommand::Four { .. } => AnalysisKind::Fourier,
        AnalysisCommand::Step(_) | AnalysisCommand::Temp { .. } => AnalysisKind::ImplicitOp,
    }
}

impl From<ResourceLimitError> for DeckPlanError {
    fn from(error: ResourceLimitError) -> Self {
        Self::ResourceLimit(error)
    }
}

fn normalize_step_identifier(value: &str, target: &'static str) -> Result<String, DeckPlanError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(DeckPlanError::InvalidStepTarget { target });
    }
    let mut normalized = String::new();
    normalized
        .try_reserve_exact(value.len())
        .map_err(|_| DeckPlanError::Allocation {
            object: "normalized STEP target",
        })?;
    normalized.push_str(value);
    normalized.make_ascii_lowercase();
    Ok(normalized)
}

fn try_copy_numeric_axis_values(
    values: &[Value],
    axis: &str,
    abort: &dyn AbortSignal,
) -> Result<Vec<RunAxisValue>, DeckPlanError> {
    let mut copied = try_vec_with_capacity(values.len(), "numeric run-axis values")?;
    for (index, value) in values.iter().copied().enumerate() {
        if index.is_multiple_of(64) && abort.is_aborted() {
            return Err(DeckPlanError::Aborted);
        }
        if !value.is_finite() {
            return Err(DeckPlanError::NonFiniteAxisValue {
                axis: axis.to_string(),
                value,
            });
        }
        copied.push(RunAxisValue::Numeric(if value == 0.0 {
            0.0
        } else {
            value
        }));
    }
    Ok(copied)
}

fn data_axis_from_table(
    netlist: &crate::netlist::Netlist,
    table_name: &str,
    limits: &ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<RunAxis, DeckPlanError> {
    let mut matching = netlist
        .data_tables
        .iter()
        .filter(|table| table.name.eq_ignore_ascii_case(table_name));
    let table = matching
        .next()
        .ok_or_else(|| DeckPlanError::UnknownStepDataTable {
            table: table_name.to_string(),
        })?;
    if matching.next().is_some() {
        return Err(DeckPlanError::AmbiguousStepDataTable {
            table: table_name.to_string(),
        });
    }
    ResourceLimitError::ensure(
        ResourceKind::BatchRuns,
        table.rows.len(),
        limits.max_batch_runs,
    )?;
    let data_value_count = checked_resource_mul(
        table.rows.len(),
        table.params.len(),
        ResourceKind::ResultValues,
    )?;
    ResourceLimitError::ensure(
        ResourceKind::ResultValues,
        data_value_count,
        limits.max_result_values,
    )?;

    let mut values = try_vec_with_capacity(table.rows.len(), "STEP DATA rows")?;
    for (row_index, row) in table.rows.iter().enumerate() {
        if row_index.is_multiple_of(64) && abort.is_aborted() {
            return Err(DeckPlanError::Aborted);
        }
        if row.len() != table.params.len() {
            return Err(DeckPlanError::StepDataRowWidth {
                table: table.name.clone(),
                row: row_index + 1,
                expected: table.params.len(),
                actual: row.len(),
            });
        }
        let mut bindings = try_vec_with_capacity(table.params.len(), "STEP DATA bindings")?;
        for (column_index, (name, value)) in table.params.iter().zip(row).enumerate() {
            if column_index.is_multiple_of(64) && abort.is_aborted() {
                return Err(DeckPlanError::Aborted);
            }
            bindings.push(DataBinding::new(
                try_clone_string(name, "STEP DATA column name")?,
                *value,
            )?);
        }
        values.push(RunAxisValue::DataRow(bindings));
    }
    RunAxis::new(AxisKind::Data, table.name.clone(), values)
}

fn checked_coordinate_count(
    axis_lengths: impl IntoIterator<Item = usize>,
) -> Result<usize, DeckPlanError> {
    axis_lengths.into_iter().try_fold(1usize, |count, length| {
        count
            .checked_mul(length)
            .ok_or(DeckPlanError::CoordinateCountOverflow)
    })
}

fn checked_resource_add(
    first: usize,
    second: usize,
    resource: ResourceKind,
) -> Result<usize, DeckPlanError> {
    first
        .checked_add(second)
        .ok_or(DeckPlanError::ResourceAccountingOverflow { resource })
}

fn checked_resource_mul(
    first: usize,
    second: usize,
    resource: ResourceKind,
) -> Result<usize, DeckPlanError> {
    first
        .checked_mul(second)
        .ok_or(DeckPlanError::ResourceAccountingOverflow { resource })
}

fn try_vec_with_capacity<T>(
    capacity: usize,
    object: &'static str,
) -> Result<Vec<T>, DeckPlanError> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(capacity)
        .map_err(|_| DeckPlanError::Allocation { object })?;
    Ok(values)
}

fn try_clone_string(value: &str, object: &'static str) -> Result<String, DeckPlanError> {
    let mut cloned = String::new();
    cloned
        .try_reserve_exact(value.len())
        .map_err(|_| DeckPlanError::Allocation { object })?;
    cloned.push_str(value);
    Ok(cloned)
}

fn try_clone_axis_value(value: &RunAxisValue) -> Result<RunAxisValue, DeckPlanError> {
    match value {
        RunAxisValue::Numeric(value) => Ok(RunAxisValue::Numeric(*value)),
        RunAxisValue::DataRow(bindings) => {
            let mut cloned = try_vec_with_capacity(bindings.len(), "DATA-row bindings")?;
            for binding in bindings {
                cloned.push(DataBinding {
                    name: try_clone_string(&binding.name, "DATA binding name")?,
                    value: binding.value,
                });
            }
            Ok(RunAxisValue::DataRow(cloned))
        }
        RunAxisValue::AlterVariant {
            label,
            materialization_digest,
        } => Ok(RunAxisValue::AlterVariant {
            label: try_clone_string(label, "ALTER variant label")?,
            materialization_digest: *materialization_digest,
        }),
    }
}

fn try_clone_step_target(target: &StepAxisTarget) -> Result<StepAxisTarget, DeckPlanError> {
    match target {
        StepAxisTarget::Parameter { name } => Ok(StepAxisTarget::Parameter {
            name: try_clone_string(name, "STEP parameter target")?,
        }),
        StepAxisTarget::Device { name, parameter } => Ok(StepAxisTarget::Device {
            name: try_clone_string(name, "STEP device target")?,
            parameter: parameter
                .as_deref()
                .map(|parameter| try_clone_string(parameter, "STEP device parameter target"))
                .transpose()?,
        }),
        StepAxisTarget::Model { name, parameter } => Ok(StepAxisTarget::Model {
            name: try_clone_string(name, "STEP model target")?,
            parameter: try_clone_string(parameter, "STEP model parameter target")?,
        }),
        StepAxisTarget::Temperature => Ok(StepAxisTarget::Temperature),
    }
}

fn try_clone_assignment(
    axis: &RunAxis,
    value_index: usize,
) -> Result<AxisAssignment, DeckPlanError> {
    let value = axis
        .values
        .get(value_index)
        .ok_or_else(|| DeckPlanError::CoordinateIndex {
            axis: axis.name.clone(),
            index: value_index,
            value_count: axis.values.len(),
        })?;
    Ok(AxisAssignment {
        kind: axis.kind,
        name: try_clone_string(&axis.name, "coordinate axis name")?,
        value: try_clone_axis_value(value)?,
        value_index,
        step_target: axis
            .step_target
            .as_ref()
            .map(try_clone_step_target)
            .transpose()?,
    })
}

fn update_hash_field(hasher: &mut blake3::Hasher, value: &[u8]) {
    hasher.update(&(value.len() as u64).to_le_bytes());
    hasher.update(value);
}

fn compare_ascii_case_insensitive(first: &str, second: &str) -> Ordering {
    first
        .bytes()
        .map(|byte| byte.to_ascii_lowercase())
        .cmp(second.bytes().map(|byte| byte.to_ascii_lowercase()))
}

fn update_ascii_lowercase_hash_field(hasher: &mut blake3::Hasher, value: &str) {
    hasher.update(&(value.len() as u64).to_le_bytes());
    for byte in value.bytes() {
        hasher.update(&[byte.to_ascii_lowercase()]);
    }
}

fn coordinate_semantic_id(assignments: &[AxisAssignment]) -> Result<[u8; 16], DeckPlanError> {
    let mut hasher = blake3::Hasher::new();
    update_hash_field(&mut hasher, b"rspice-coordinate-v1");
    let mut canonical_assignments =
        try_vec_with_capacity(assignments.len(), "canonical coordinate identity")?;
    canonical_assignments.extend(assignments);
    canonical_assignments.sort_by(|first, second| {
        first
            .kind
            .cmp(&second.kind)
            .then_with(|| compare_ascii_case_insensitive(&first.name, &second.name))
    });
    for assignment in canonical_assignments {
        update_hash_field(&mut hasher, assignment.kind.tag().as_bytes());
        update_ascii_lowercase_hash_field(&mut hasher, &assignment.name);
        assignment.value.update_stable_hash(&mut hasher);
    }
    let digest = hasher.finalize();
    let mut id = [0u8; 16];
    id.copy_from_slice(&digest.as_bytes()[..16]);
    Ok(id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::ImmediateAbort;

    fn coordinate_limits(maximum_coordinates: usize, maximum_assignments: usize) -> ResourceLimits {
        let mut limits = ResourceLimits::default();
        limits.max_batch_runs = maximum_coordinates;
        limits.max_result_values = maximum_assignments;
        limits
    }

    fn numeric_coordinate_ids(
        coordinates: &[RunCoordinate],
    ) -> std::collections::BTreeMap<Vec<(String, u64)>, RunCoordinateId> {
        coordinates
            .iter()
            .map(|coordinate| {
                let mut assignments = coordinate
                    .assignments()
                    .iter()
                    .map(|assignment| {
                        let RunAxisValue::Numeric(value) = assignment.value() else {
                            panic!("numeric identity fixture must contain numeric axes")
                        };
                        (assignment.name().to_ascii_lowercase(), value.to_bits())
                    })
                    .collect::<Vec<_>>();
                assignments.sort();
                (assignments, coordinate.stable_id())
            })
            .collect()
    }

    #[test]
    fn repeated_analysis_kinds_have_distinct_stable_tags() {
        let plan = DeckPlan::new(
            Vec::new(),
            vec![
                AnalysisRequest::new(AnalysisKind::Ac),
                AnalysisRequest::new(AnalysisKind::Tran),
                AnalysisRequest::new(AnalysisKind::Ac),
            ],
        )
        .expect("planner assigns analysis IDs");
        let first = plan.analyses()[0].id();
        let second = plan.analyses()[2].id();
        assert_eq!(first.tag(), "ac-001");
        assert_eq!(second.tag(), "ac-002");
        assert_ne!(first, second);

        assert!(matches!(
            DeckPlan::new(
                Vec::new(),
                vec![AnalysisRequest::new(AnalysisKind::ImplicitOp)]
            ),
            Err(DeckPlanError::ExplicitImplicitOp)
        ));
    }

    #[test]
    fn cartesian_coordinates_are_first_axis_fastest_bounded_abortable_and_stable() {
        let temperature = RunAxis::new(
            AxisKind::Temperature,
            "temperature",
            vec![RunAxisValue::Numeric(25.0), RunAxisValue::Numeric(125.0)],
        )
        .expect("temperature axis");
        let process = RunAxis::new(
            AxisKind::Step,
            "corner",
            vec![RunAxisValue::Numeric(0.0), RunAxisValue::Numeric(1.0)],
        )
        .expect("process axis");
        let plan = DeckPlan::new(vec![process, temperature], Vec::new()).expect("valid plan");
        let limits = coordinate_limits(4, 8);
        let coordinates = plan
            .coordinates_with_abort(&limits, &crate::NoAbort)
            .expect("four coordinates");
        assert_eq!(coordinates.len(), 4);
        assert_eq!(coordinates[0].assignments()[0].value_index(), 0);
        assert_eq!(coordinates[0].assignments()[1].value_index(), 0);
        assert_eq!(coordinates[1].assignments()[0].value_index(), 1);
        assert_eq!(coordinates[1].assignments()[1].value_index(), 0);
        assert_eq!(coordinates[2].assignments()[0].value_index(), 0);
        assert_eq!(coordinates[2].assignments()[1].value_index(), 1);
        // This explicitly distinguishes Xyce's first-axis-fastest order from
        // ordinary row-major products, where coordinate 1 would be (0, 1).
        assert!(
            coordinates
                .iter()
                .all(|coordinate| coordinate.stable_id().semantic_bytes() != [0; 16])
        );
        let repeated = plan
            .coordinates_with_abort(&limits, &crate::NoAbort)
            .expect("repeat materialization");
        assert_eq!(coordinates, repeated);

        let limited = coordinate_limits(3, 8);
        assert!(matches!(
            plan.coordinates_with_abort(&limited, &crate::NoAbort),
            Err(DeckPlanError::ResourceLimit(ResourceLimitError {
                resource: ResourceKind::BatchRuns,
                requested: 4,
                limit: 3,
            }))
        ));
        assert!(matches!(
            plan.coordinates_with_abort(&limits, &ImmediateAbort),
            Err(DeckPlanError::Aborted)
        ));
    }

    #[test]
    fn coordinate_identity_is_stable_across_axis_and_value_reordering() {
        let first = DeckPlan::new(
            vec![
                RunAxis::new(
                    AxisKind::Step,
                    "a",
                    vec![RunAxisValue::Numeric(1.0), RunAxisValue::Numeric(2.0)],
                )
                .expect("a axis"),
                RunAxis::new(
                    AxisKind::Step,
                    "b",
                    vec![RunAxisValue::Numeric(10.0), RunAxisValue::Numeric(20.0)],
                )
                .expect("b axis"),
            ],
            Vec::new(),
        )
        .expect("first plan");
        let reversed = DeckPlan::new(
            vec![
                RunAxis::new(
                    AxisKind::Step,
                    "B",
                    vec![RunAxisValue::Numeric(20.0), RunAxisValue::Numeric(10.0)],
                )
                .expect("reversed b axis"),
                RunAxis::new(
                    AxisKind::Step,
                    "A",
                    vec![RunAxisValue::Numeric(2.0), RunAxisValue::Numeric(1.0)],
                )
                .expect("reversed a axis"),
            ],
            Vec::new(),
        )
        .expect("reversed plan");
        let limits = coordinate_limits(4, 8);
        let first_coordinates = first
            .coordinates_with_abort(&limits, &crate::NoAbort)
            .expect("first coordinates");
        let reversed_coordinates = reversed
            .coordinates_with_abort(&limits, &crate::NoAbort)
            .expect("reversed coordinates");

        assert_eq!(
            numeric_coordinate_ids(&first_coordinates),
            numeric_coordinate_ids(&reversed_coordinates)
        );
    }

    #[test]
    fn data_rows_remain_coupled_instead_of_forming_a_column_product() {
        let row = |x, y| {
            RunAxisValue::DataRow(vec![
                DataBinding::new("x", x).expect("x binding"),
                DataBinding::new("y", y).expect("y binding"),
            ])
        };
        let data = RunAxis::new(
            AxisKind::Data,
            "table",
            vec![row(1.0, 10.0), row(2.0, 20.0)],
        )
        .expect("DATA rows");
        let step = RunAxis::new(
            AxisKind::Step,
            "corner",
            vec![RunAxisValue::Numeric(0.0), RunAxisValue::Numeric(1.0)],
        )
        .expect("STEP axis");
        let plan = DeckPlan::new(vec![data, step], Vec::new()).expect("DATA/STEP plan");
        let coordinates = plan
            .coordinates_with_abort(&coordinate_limits(4, 12), &crate::NoAbort)
            .expect("two DATA rows times two STEP values");

        assert_eq!(coordinates.len(), 4);
        for coordinate in coordinates {
            assert_eq!(coordinate.assignments().len(), 2);
            let RunAxisValue::DataRow(bindings) = coordinate.assignments()[0].value() else {
                panic!("first assignment must retain one coupled DATA row")
            };
            assert_eq!(bindings.len(), 2);
            assert!(
                matches!(
                    (bindings[0].value(), bindings[1].value()),
                    (1.0, 10.0) | (2.0, 20.0)
                ),
                "columns from separate rows must never be combined"
            );
        }
    }

    #[test]
    fn axes_cannot_claim_the_same_coordinate_binding() {
        let data = RunAxis::new(
            AxisKind::Data,
            "table",
            vec![RunAxisValue::DataRow(vec![
                DataBinding::new("gain", 1.0).expect("gain binding"),
            ])],
        )
        .expect("DATA axis");
        let step = RunAxis::new(AxisKind::Step, "GAIN", vec![RunAxisValue::Numeric(2.0)])
            .expect("STEP axis");

        assert!(matches!(
            DeckPlan::new(vec![data, step], Vec::new()),
            Err(DeckPlanError::BindingCollision {
                binding,
                first: AxisKind::Data,
                second: AxisKind::Step,
            }) if binding == "gain"
        ));
    }

    #[test]
    fn coordinate_resources_are_preflighted_before_materialization() {
        let first = RunAxis::new(
            AxisKind::Step,
            "a",
            vec![RunAxisValue::Numeric(1.0), RunAxisValue::Numeric(2.0)],
        )
        .expect("first axis");
        let second = RunAxis::new(
            AxisKind::Step,
            "b",
            vec![RunAxisValue::Numeric(3.0), RunAxisValue::Numeric(4.0)],
        )
        .expect("second axis");
        let plan = DeckPlan::new(vec![first, second], Vec::new()).expect("resource plan");
        let estimate = plan
            .coordinate_resource_estimate()
            .expect("bounded estimate");
        assert_eq!(estimate.coordinate_count, 4);
        assert_eq!(estimate.total_assignments, 8);

        let batch_limited = coordinate_limits(3, 8);
        assert!(matches!(
            plan.coordinates_with_abort(&batch_limited, &crate::NoAbort),
            Err(DeckPlanError::ResourceLimit(ResourceLimitError {
                resource: ResourceKind::BatchRuns,
                requested: 4,
                limit: 3,
            }))
        ));

        let assignment_limited = coordinate_limits(4, 7);
        assert!(matches!(
            plan.coordinates_with_abort(&assignment_limited, &crate::NoAbort),
            Err(DeckPlanError::ResourceLimit(ResourceLimitError {
                resource: ResourceKind::ResultValues,
                requested: 8,
                limit: 7,
            }))
        ));

        let mut byte_limited = coordinate_limits(4, 8);
        byte_limited.max_expanded_source_bytes = estimate.retained_dynamic_bytes - 1;
        assert!(matches!(
            plan.coordinates_with_abort(&byte_limited, &crate::NoAbort),
            Err(DeckPlanError::ResourceLimit(ResourceLimitError {
                resource: ResourceKind::ExpandedSourceBytes,
                requested,
                limit,
            })) if requested == estimate.retained_dynamic_bytes
                && limit + 1 == estimate.retained_dynamic_bytes
        ));
    }

    #[test]
    fn resource_arithmetic_and_capacity_overflow_fail_closed() {
        assert!(matches!(
            checked_coordinate_count([usize::MAX, 2]),
            Err(DeckPlanError::CoordinateCountOverflow)
        ));
        assert!(matches!(
            checked_resource_mul(usize::MAX, 2, ResourceKind::ResultValues),
            Err(DeckPlanError::ResourceAccountingOverflow {
                resource: ResourceKind::ResultValues,
            })
        ));
        assert!(matches!(
            try_vec_with_capacity::<u8>(usize::MAX, "overflow fixture"),
            Err(DeckPlanError::Allocation {
                object: "overflow fixture",
            })
        ));
    }

    #[test]
    fn no_axis_plan_has_one_explicit_coordinate() {
        let plan = DeckPlan::new(Vec::new(), Vec::new()).expect("empty plan is valid");
        let coordinates = plan
            .coordinates_with_abort(&coordinate_limits(1, 0), &crate::NoAbort)
            .expect("implicit coordinate");
        assert_eq!(coordinates.len(), 1);
        assert!(coordinates[0].assignments().is_empty());
        assert_eq!(plan.analyses().len(), 1);
        assert_eq!(plan.analyses()[0].id().kind(), AnalysisKind::ImplicitOp);
    }

    #[test]
    fn netlist_temperature_axis_wraps_authored_analyses_with_stable_ids() {
        let netlist = crate::Netlist::parse(
            "temperature planner\nR1 in out 1k\nV1 in 0 1\n.temp -40 125\n.ac lin 2 1 2\n.tran 1u 2u\n.ac lin 2 10 20\n.end",
        )
        .expect("temperature deck parses");
        let plan = DeckPlan::from_netlist_temperature_axis(&netlist)
            .expect("temperature plan is valid")
            .expect("temperature axis is present");

        assert_eq!(plan.axes().len(), 1);
        assert_eq!(plan.axes()[0].kind(), AxisKind::Temperature);
        assert_eq!(
            plan.analyses()
                .iter()
                .map(|analysis| analysis.id().tag())
                .collect::<Vec<_>>(),
            ["ac-001", "tran-001", "ac-002"]
        );
        let coordinates = plan
            .coordinates_with_abort(&ResourceLimits::default(), &crate::NoAbort)
            .expect("temperature coordinates materialize");
        assert_eq!(coordinates.len(), 2);
        assert_eq!(
            coordinates
                .iter()
                .map(RunCoordinate::stable_tag)
                .collect::<Vec<_>>(),
            plan.coordinates_with_abort(&ResourceLimits::default(), &crate::NoAbort)
                .expect("repeated materialization")
                .iter()
                .map(RunCoordinate::stable_tag)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn temperature_axis_without_physical_analysis_uses_implicit_op() {
        let netlist = crate::Netlist::parse("implicit temperature\nR1 1 0 1k\n.temp 25 50\n.end")
            .expect("temperature deck parses");
        let plan = DeckPlan::from_netlist_temperature_axis(&netlist)
            .expect("temperature plan is valid")
            .expect("temperature axis is present");
        assert_eq!(plan.analyses().len(), 1);
        assert_eq!(plan.analyses()[0].id().kind(), AnalysisKind::ImplicitOp);
    }

    fn numeric_values(axis: &RunAxis) -> Vec<Value> {
        axis.values()
            .iter()
            .map(|value| match value {
                RunAxisValue::Numeric(value) => *value,
                other => panic!("expected numeric axis value, got {other:?}"),
            })
            .collect()
    }

    #[test]
    fn authored_step_list_linear_decade_and_octave_become_ordered_axes() {
        let netlist = crate::Netlist::parse(
            "STEP sweep modes\n\
             .step param listed list 3 1 4\n\
             .step lin param linear 1 3 1\n\
             .step dec param decade 1 100 1\n\
             .step oct param octave 1 4 1\n\
             .op\n\
             .end\n",
        )
        .expect("all numeric STEP modes parse");
        let plan = DeckPlan::from_netlist_run_axes(&netlist, &ResourceLimits::default())
            .expect("numeric STEP axes plan")
            .expect("STEP axes are present");

        assert_eq!(
            plan.axes().iter().map(RunAxis::name).collect::<Vec<_>>(),
            [
                "param:listed",
                "param:linear",
                "param:decade",
                "param:octave",
            ]
        );
        assert_eq!(numeric_values(&plan.axes()[0]), [3.0, 1.0, 4.0]);
        assert_eq!(numeric_values(&plan.axes()[1]), [1.0, 2.0, 3.0]);
        assert_eq!(numeric_values(&plan.axes()[2]), [1.0, 10.0, 100.0]);
        assert_eq!(numeric_values(&plan.axes()[3]), [1.0, 2.0, 4.0]);
        assert!(plan.axes().iter().all(|axis| axis.kind() == AxisKind::Step));
    }

    #[test]
    fn authored_step_targets_remain_typed_on_axes_and_coordinates() {
        let netlist = crate::Netlist::parse(
            "STEP targets\n\
             R1 out 0 1k\n\
             .step param gain list 1 2\n\
             .step R1:resistance list 1k 2k\n\
             .step model RMOD r list 10 20\n\
             .step temp list -40 125\n\
             .op\n\
             .end\n",
        )
        .expect("typed STEP targets parse");
        let plan = DeckPlan::from_netlist_run_axes(&netlist, &ResourceLimits::default())
            .expect("typed STEP targets plan")
            .expect("STEP axes are present");

        assert!(matches!(
            plan.axes()[0].step_target(),
            Some(StepAxisTarget::Parameter { name }) if name == "gain"
        ));
        assert!(matches!(
            plan.axes()[1].step_target(),
            Some(StepAxisTarget::Device {
                name,
                parameter: Some(parameter),
            }) if name == "r1" && parameter == "resistance"
        ));
        assert!(matches!(
            plan.axes()[2].step_target(),
            Some(StepAxisTarget::Model { name, parameter })
                if name == "rmod" && parameter == "r"
        ));
        assert_eq!(plan.axes()[3].kind(), AxisKind::Temperature);
        assert_eq!(
            plan.axes()[3].step_target(),
            Some(&StepAxisTarget::Temperature)
        );

        let coordinates = plan
            .coordinates_with_abort(&ResourceLimits::default(), &crate::NoAbort)
            .expect("typed coordinates materialize");
        assert_eq!(coordinates.len(), 16);
        assert!(matches!(
            coordinates[0].assignments()[2].step_target(),
            Some(StepAxisTarget::Model { name, parameter })
                if name == "rmod" && parameter == "r"
        ));
        assert_eq!(
            coordinates[0].assignments()[3].step_target(),
            Some(&StepAxisTarget::Temperature)
        );
    }

    #[test]
    fn data_step_numeric_step_and_temp_use_canonical_axis_order() {
        let netlist = crate::Netlist::parse(
            "axis ordering\n\
             .step param gain list 1 2\n\
             .temp -40 125\n\
             .data corners\n\
             + bias scale\n\
             + 1 10\n\
             + 2 20\n\
             .enddata\n\
             .step data=corners\n\
             .op\n\
             .end\n",
        )
        .expect("DATA, STEP, and TEMP deck parses");
        let plan = DeckPlan::from_netlist_run_axes(&netlist, &ResourceLimits::default())
            .expect("mixed axes plan")
            .expect("mixed axes are present");

        assert_eq!(
            plan.axes().iter().map(RunAxis::kind).collect::<Vec<_>>(),
            [AxisKind::Data, AxisKind::Step, AxisKind::Temperature]
        );
        let coordinates = plan
            .coordinates_with_abort(&ResourceLimits::default(), &crate::NoAbort)
            .expect("mixed coordinates materialize");
        assert_eq!(coordinates.len(), 8);
        assert_eq!(
            coordinates[0]
                .assignments()
                .iter()
                .map(AxisAssignment::kind)
                .collect::<Vec<_>>(),
            [AxisKind::Data, AxisKind::Step, AxisKind::Temperature]
        );
        assert_eq!(numeric_values(&plan.axes()[2]), [-40.0, 125.0]);
    }

    #[test]
    fn canonical_temp_step_plan_is_cartesian_and_preserves_analysis_ordinals() {
        let netlist = crate::Netlist::parse(
            "TEMP by STEP\n\
             .param resistance=1k\n\
             R1 in out {resistance}\n\
             V1 in 0 1\n\
             .temp -40 25 125\n\
             .step param resistance list 1k 2k\n\
             .ac lin 2 1 2\n\
             .tran 1u 2u\n\
             .ac lin 2 10 20\n\
             .end\n",
        )
        .expect("TEMP by STEP deck parses");
        let plan = DeckPlan::from_netlist_run_axes(&netlist, &ResourceLimits::default())
            .expect("TEMP and STEP compose")
            .expect("run axes are present");

        assert_eq!(
            plan.axes().iter().map(RunAxis::kind).collect::<Vec<_>>(),
            [AxisKind::Step, AxisKind::Temperature]
        );
        assert_eq!(
            plan.analyses()
                .iter()
                .map(|analysis| analysis.id().tag())
                .collect::<Vec<_>>(),
            ["ac-001", "tran-001", "ac-002"]
        );
        let coordinates = plan
            .coordinates_with_abort(&ResourceLimits::default(), &crate::NoAbort)
            .expect("six Cartesian coordinates");
        assert_eq!(coordinates.len(), 6);
        assert_eq!(coordinates[0].assignments()[0].value_index(), 0);
        assert_eq!(coordinates[0].assignments()[1].value_index(), 0);
        assert_eq!(coordinates[1].assignments()[0].value_index(), 1);
        assert_eq!(coordinates[1].assignments()[1].value_index(), 0);
        assert_eq!(coordinates[2].assignments()[0].value_index(), 0);
        assert_eq!(coordinates[2].assignments()[1].value_index(), 1);
        assert_eq!(
            coordinates,
            plan.coordinates_with_abort(&ResourceLimits::default(), &crate::NoAbort)
                .expect("coordinate IDs are deterministic")
        );

        assert!(matches!(
            DeckPlan::from_netlist_temperature_axis(&netlist),
            Err(DeckPlanError::TemperatureAxisWithStep)
        ));
    }

    #[test]
    fn scalar_deck_plan_preserves_repeated_analysis_ordinals_without_axes() {
        let netlist = crate::Netlist::parse(
            "scalar analyses\nV1 in 0 DC 1 AC 1\nR1 in 0 1k\n.ac lin 2 1 2\n.tran 1u 2u\n.ac lin 3 10 30\n.end\n",
        )
        .expect("scalar fixture parses");

        let plan = DeckPlan::from_netlist(&netlist, &ResourceLimits::default())
            .expect("complete scalar plan builds");

        assert!(plan.axes().is_empty());
        assert_eq!(
            plan.analyses()
                .iter()
                .map(|analysis| analysis.id().tag())
                .collect::<Vec<_>>(),
            ["ac-001", "tran-001", "ac-002"]
        );
        assert!(
            DeckPlan::from_netlist_run_axes(&netlist, &ResourceLimits::default())
                .expect("compatibility query succeeds")
                .is_none()
        );
    }

    #[test]
    fn netlist_axis_cardinality_and_scalar_bindings_are_preflighted() {
        let netlist = crate::Netlist::parse(
            "bounded TEMP by STEP\n\
             .step param p list 1 2\n\
             .temp -40 25 125\n\
             .op\n\
             .end\n",
        )
        .expect("bounded axis deck parses");
        let mut limits = ResourceLimits::default();
        limits.max_batch_runs = 5;
        assert!(matches!(
            DeckPlan::from_netlist_run_axes(&netlist, &limits),
            Err(DeckPlanError::ResourceLimit(ResourceLimitError {
                resource: ResourceKind::BatchRuns,
                requested: 6,
                limit: 5,
            }))
        ));

        limits.max_batch_runs = 6;
        limits.max_result_values = 11;
        assert!(matches!(
            DeckPlan::from_netlist_run_axes(&netlist, &limits),
            Err(DeckPlanError::ResourceLimit(ResourceLimitError {
                resource: ResourceKind::ResultValues,
                requested: 12,
                limit: 11,
            }))
        ));
    }

    #[test]
    fn step_data_resolution_is_typed_and_fail_closed() {
        let missing = crate::Netlist::parse("missing DATA\n.step data=absent\n.op\n.end\n")
            .expect("unresolved DATA reference remains typed");
        let error = DeckPlan::from_netlist_run_axes(&missing, &ResourceLimits::default())
            .expect_err("unknown STEP DATA table must fail planning");
        assert!(
            matches!(&error, DeckPlanError::UnknownStepDataTable { table } if table.eq_ignore_ascii_case("absent")),
            "unexpected planning error: {error:?}"
        );

        let abort = ImmediateAbort;
        let list = crate::Netlist::parse("aborted STEP\n.step param p list 1 2\n.op\n.end\n")
            .expect("STEP list parses");
        assert!(matches!(
            DeckPlan::from_netlist_run_axes_with_abort(&list, &ResourceLimits::default(), &abort,),
            Err(DeckPlanError::Aborted)
        ));
    }
}
