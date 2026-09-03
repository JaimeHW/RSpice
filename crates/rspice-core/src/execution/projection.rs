//! One projection from authored output directives onto a typed result.
//!
//! `.SAVE`, `.PROBE`, `.PRINT`, and `.PLOT` all select circuit quantities.
//! The parser already records them in one [`SaveSet`](crate::netlist::SaveSet)
//! plus typed [`OutputRequest`](crate::netlist::OutputRequest) provenance;
//! this module is the single decision procedure that turns that authored
//! contract into the ordered, typed columns one analysis result must export.
//!
//! Frontends supply a [`ProjectionSource`]: the analysis family, its axis, the
//! signals the result actually materialized, and — for the two families that
//! own a core ordered-output resolver — the columns that resolver produced for
//! the `.PRINT`/`.PLOT` cards. Nothing else about a `.SAVE` symbol is decided
//! outside this module, so the CLI, the Python bindings, and the engine
//! adapter cannot disagree about what a deck asked for.
//!
//! An authored symbol the analysis cannot supply is a typed
//! [`SimulationError::RequestedSignalUnavailable`] naming the original
//! spelling and the analysis instance. It is never an empty column, a zero, or
//! a silently narrower export.

use std::borrow::Cow;
use std::collections::HashMap;

use crate::abort_signal::AbortSignal;
use crate::analysis::measure_signals::{
    CanonicalMeasureSignalIndex, OutputColumnKind, OutputOperandEvaluationError,
    evaluate_output_operand,
};
use crate::engine::SimulationError;
use crate::netlist::{
    Netlist, NetlistSourceLocation, OutputAnalysisKind, OutputDirectiveKind, OutputOperandKind,
    ParamContext, SaveSignal,
};
use crate::{Value, netlist::SaveSet};

use super::capability::AnalysisResultKind;
use super::schema::{
    SignalDescriptor, SignalKind, SignalOwner, SignalSchema, SignalSchemaError, SignalShape,
    SignalUnit, SignalValueType,
};

/// The `.PRINT`/`.PLOT` analysis qualifier that owns one result family.
///
/// A family with no qualifier keyword is only selected by an unqualified
/// output card; it is never silently folded into another family's cards.
/// The match is exhaustive so a new [`AnalysisResultKind`] cannot compile
/// until its authored-output identity is decided.
pub const fn projection_analysis_kind(kind: AnalysisResultKind) -> Option<OutputAnalysisKind> {
    match kind {
        AnalysisResultKind::OperatingPoint => Some(OutputAnalysisKind::Op),
        AnalysisResultKind::DcSweep => Some(OutputAnalysisKind::Dc),
        AnalysisResultKind::Ac => Some(OutputAnalysisKind::Ac),
        AnalysisResultKind::Transient => Some(OutputAnalysisKind::Tran),
        AnalysisResultKind::Noise | AnalysisResultKind::PortNoise => {
            Some(OutputAnalysisKind::Noise)
        }
        AnalysisResultKind::SParameters => Some(OutputAnalysisKind::Sp),
        AnalysisResultKind::Distortion => Some(OutputAnalysisKind::Disto),
        AnalysisResultKind::TransferFunction => Some(OutputAnalysisKind::Tf),
        AnalysisResultKind::Pss => Some(OutputAnalysisKind::Pss),
        AnalysisResultKind::HarmonicBalance => Some(OutputAnalysisKind::Hb),
        // Xyce and ngspice define no `.PRINT` keyword for these families, so
        // only an unqualified output card selects them.
        AnalysisResultKind::Stability
        | AnalysisResultKind::Sensitivity
        | AnalysisResultKind::PoleZero
        | AnalysisResultKind::Fourier
        | AnalysisResultKind::Fft
        | AnalysisResultKind::MonteCarlo
        | AnalysisResultKind::Pac
        | AnalysisResultKind::PNoise
        | AnalysisResultKind::Envelope => None,
    }
}

/// Values of one signal offered to, or produced by, output projection.
#[derive(Debug, Clone, PartialEq)]
pub enum ProjectionValues<'a> {
    Real(Cow<'a, [Value]>),
    Complex {
        real: Cow<'a, [Value]>,
        imag: Cow<'a, [Value]>,
    },
}

impl ProjectionValues<'_> {
    pub fn len(&self) -> usize {
        match self {
            Self::Real(values) => values.len(),
            Self::Complex { real, .. } => real.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub const fn value_type(&self) -> SignalValueType {
        match self {
            Self::Real(_) => SignalValueType::Real,
            Self::Complex { .. } => SignalValueType::Complex,
        }
    }

    fn into_owned(self) -> ProjectionValues<'static> {
        match self {
            Self::Real(values) => ProjectionValues::Real(Cow::Owned(values.into_owned())),
            Self::Complex { real, imag } => ProjectionValues::Complex {
                real: Cow::Owned(real.into_owned()),
                imag: Cow::Owned(imag.into_owned()),
            },
        }
    }
}

/// One signal an analysis result materialized, offered for selection.
#[derive(Debug, Clone, PartialEq)]
pub struct ProjectionSourceSignal<'a> {
    descriptor: SignalDescriptor,
    values: ProjectionValues<'a>,
    validity: Option<Cow<'a, [bool]>>,
}

impl<'a> ProjectionSourceSignal<'a> {
    /// Describe one materialized column from its authored display name, its
    /// registry name, and its typed kind.
    ///
    /// This is the only place a frontend name becomes a typed descriptor, so
    /// `@device[param]` columns keep device ownership instead of degrading to
    /// an anonymous analysis scalar.
    pub fn new(
        display_name: impl AsRef<str>,
        registry_name: impl AsRef<str>,
        kind: SignalKind,
        values: ProjectionValues<'a>,
    ) -> Result<Self, SignalSchemaError> {
        let descriptor = signal_descriptor(
            display_name.as_ref(),
            registry_name.as_ref(),
            kind,
            values.value_type(),
        )?;
        Ok(Self {
            descriptor,
            values,
            validity: None,
        })
    }

    /// Attach the result's own per-sample validity mask.
    ///
    /// Absent samples stay absent through projection; they are never filled
    /// with a numeric placeholder.
    pub fn with_validity(mut self, validity: impl Into<Cow<'a, [bool]>>) -> Self {
        self.validity = Some(validity.into());
        self
    }

    pub const fn descriptor(&self) -> &SignalDescriptor {
        &self.descriptor
    }

    pub const fn values(&self) -> &ProjectionValues<'a> {
        &self.values
    }

    fn real_values(&self) -> Option<&[Value]> {
        match &self.values {
            ProjectionValues::Real(values) => Some(values.as_ref()),
            ProjectionValues::Complex { .. } => None,
        }
    }

    fn into_projected(self) -> ProjectedSignal {
        let length = self.values.len();
        let validity = self
            .validity
            .map_or_else(|| vec![true; length], |mask| mask.into_owned());
        ProjectedSignal {
            descriptor: self.descriptor,
            values: self.values.into_owned(),
            validity,
        }
    }
}

/// Build the typed descriptor for one exported column.
///
/// `registry_name` is the bare circuit symbol (`out`, `v1`, `@d1[id]`); the
/// display name is the authored spelling that appears in an artifact header.
pub fn signal_descriptor(
    display_name: &str,
    registry_name: &str,
    kind: SignalKind,
    value_type: SignalValueType,
) -> Result<SignalDescriptor, SignalSchemaError> {
    let (unit, owner) = match kind {
        SignalKind::Voltage => (
            SignalUnit::Volt,
            SignalOwner::Node(registry_name.to_string()),
        ),
        SignalKind::Current => (
            SignalUnit::Ampere,
            SignalOwner::Branch(registry_name.to_string()),
        ),
        SignalKind::Digital => (
            SignalUnit::Logic,
            SignalOwner::Node(registry_name.to_string()),
        ),
        SignalKind::DeviceObservable => (
            SignalUnit::Dimensionless,
            SignalOwner::Device(
                device_of_parameter_probe(registry_name)
                    .unwrap_or(registry_name)
                    .to_string(),
            ),
        ),
        SignalKind::Scalar => (SignalUnit::Dimensionless, SignalOwner::Analysis),
    };
    // A digital trace is carried as numeric samples so it can share one table
    // with analog columns, but its declared value type stays logic: the
    // samples are states, not volts.
    let value_type = if kind == SignalKind::Digital {
        SignalValueType::Logic
    } else {
        value_type
    };
    SignalDescriptor::new(
        display_name,
        display_name,
        kind,
        unit,
        value_type,
        SignalShape::Vector,
        owner,
    )
}

/// The device named by an `@device[param]` or `device:param` probe.
fn device_of_parameter_probe(name: &str) -> Option<&str> {
    let trimmed = name.trim();
    if let Some(device) = trimmed
        .strip_prefix('@')
        .and_then(|rest| rest.split_once('['))
        .map(|(device, _)| device.trim())
        .filter(|device| !device.is_empty())
    {
        return Some(device);
    }
    trimmed
        .split_once(':')
        .map(|(device, _)| device.trim())
        .filter(|device| !device.is_empty())
}

/// A typed analysis result presented for authored output projection.
pub struct ProjectionSource<'a> {
    kind: AnalysisResultKind,
    instance: String,
    coordinate: Option<String>,
    signals: Vec<ProjectionSourceSignal<'a>>,
    lookup: HashMap<String, &'a [Value]>,
    axis: Cow<'a, [Value]>,
    ordered_columns: Option<Vec<ProjectedSignal>>,
}

impl<'a> ProjectionSource<'a> {
    /// Start a source for one analysis instance.
    ///
    /// `instance` names the analysis in a typed unavailable-signal error, so
    /// it should be the authored analysis label (`TRAN`, `AC`, `PSS`, ...).
    pub fn new(kind: AnalysisResultKind, instance: impl Into<String>) -> Self {
        Self {
            kind,
            instance: instance.into(),
            coordinate: None,
            signals: Vec::new(),
            lookup: HashMap::new(),
            axis: Cow::Borrowed(&[]),
            ordered_columns: None,
        }
    }

    /// Identify the sweep/step coordinate this result belongs to.
    #[must_use]
    pub fn with_coordinate(mut self, coordinate: Option<String>) -> Self {
        self.coordinate = coordinate;
        self
    }

    /// The analysis axis (time, frequency, sweep value), used by authored
    /// output expressions.
    #[must_use]
    pub fn with_axis(mut self, axis: impl Into<Cow<'a, [Value]>>) -> Self {
        self.axis = axis.into();
        self
    }

    /// The signals this result materialized, in result order.
    #[must_use]
    pub fn with_signals(mut self, signals: Vec<ProjectionSourceSignal<'a>>) -> Self {
        self.signals = signals;
        self
    }

    /// Additional real-valued lookup spellings for the core output resolver.
    ///
    /// These are alternative names for data the result already owns
    /// (hierarchy aliases, `device:param`, lead-current accessors). They are
    /// resolvable but are not columns of their own.
    #[must_use]
    pub fn with_lookup(mut self, lookup: HashMap<String, &'a [Value]>) -> Self {
        self.lookup = lookup;
        self
    }

    /// Columns the core ordered-output resolver already produced for this
    /// family's `.PRINT`/`.PLOT` cards.
    ///
    /// Only the two families with an ordered resolver (`TRAN` and `DC`) set
    /// this. Every other family resolves its print operands as direct symbols
    /// against the inventory, which is why an ordered-only construct such as
    /// an authored expression fails typed there instead of silently vanishing.
    #[must_use]
    pub fn with_ordered_print_columns(mut self, columns: Option<Vec<ProjectedSignal>>) -> Self {
        self.ordered_columns = columns;
        self
    }

    fn unavailable(&self, symbol: &str) -> SimulationError {
        SimulationError::requested_signal_unavailable(
            symbol,
            self.instance.clone(),
            self.coordinate.clone(),
        )
    }

    /// The real-valued signal table the core output resolver searches.
    fn real_signal_map(&self) -> HashMap<String, &[Value]> {
        let mut signals = self.lookup.clone();
        for signal in &self.signals {
            let Some(values) = signal.real_values() else {
                continue;
            };
            signals
                .entry(signal.descriptor.display_name().to_string())
                .or_insert(values);
        }
        signals
    }
}

/// One projected output column.
#[derive(Debug, Clone, PartialEq)]
pub struct ProjectedSignal {
    descriptor: SignalDescriptor,
    values: ProjectionValues<'static>,
    validity: Vec<bool>,
}

impl ProjectedSignal {
    pub fn new(
        descriptor: SignalDescriptor,
        values: ProjectionValues<'static>,
        validity: Vec<bool>,
    ) -> Result<Self, SimulationError> {
        if validity.len() != values.len() {
            return Err(SimulationError::result_schema_mismatch(
                "output projection",
                None,
                "validity",
                vec![descriptor.display_name().to_string()],
                vec![descriptor.display_name().to_string()],
                values.len(),
                validity.len(),
            ));
        }
        Ok(Self {
            descriptor,
            values,
            validity,
        })
    }

    pub const fn descriptor(&self) -> &SignalDescriptor {
        &self.descriptor
    }

    pub const fn values(&self) -> &ProjectionValues<'static> {
        &self.values
    }

    /// Per-sample presence. A `false` entry is explicit missingness.
    pub fn validity(&self) -> &[bool] {
        &self.validity
    }

    pub fn real(&self) -> Option<&[Value]> {
        match &self.values {
            ProjectionValues::Real(values) => Some(values.as_ref()),
            ProjectionValues::Complex { .. } => None,
        }
    }

    pub fn complex(&self) -> Option<(&[Value], &[Value])> {
        match &self.values {
            ProjectionValues::Real(_) => None,
            ProjectionValues::Complex { real, imag } => Some((real.as_ref(), imag.as_ref())),
        }
    }

    fn identity(&self) -> String {
        self.descriptor.canonical_name().to_string()
    }
}

/// The ordered result of projecting one analysis result.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ProjectedSignals {
    signals: Vec<ProjectedSignal>,
}

impl ProjectedSignals {
    pub fn signals(&self) -> &[ProjectedSignal] {
        &self.signals
    }

    pub fn into_signals(self) -> Vec<ProjectedSignal> {
        self.signals
    }

    /// The typed schema of this projection, for cross-coordinate unions.
    pub fn schema(&self) -> Result<SignalSchema, SignalSchemaError> {
        SignalSchema::new(
            self.signals
                .iter()
                .map(|signal| signal.descriptor.clone())
                .collect(),
        )
    }
}

/// The deck-wide authored output contract.
///
/// Built once from a parsed netlist and reused for every analysis instance so
/// a `.SAVE` symbol means the same thing in every family and on every surface.
#[derive(Debug, Clone)]
pub struct SignalProjection {
    cards: Vec<ProjectionCard>,
    /// Symbols contributed by `.SAVE`/`.PROBE` alone. `.PRINT`/`.PLOT`
    /// operands are owned by their card and are ordered by it instead.
    save_only: Vec<SaveSignal>,
    keeps_everything: bool,
    authored: bool,
}

#[derive(Debug, Clone)]
struct ProjectionCard {
    directive: OutputDirectiveKind,
    analysis: Option<OutputAnalysisKind>,
    origin: NetlistSourceLocation,
    operands: Vec<(String, OutputOperandKind)>,
}

impl SignalProjection {
    /// Build the authored output contract from a parsed deck.
    pub fn from_netlist(netlist: &Netlist) -> Result<Self, SimulationError> {
        let mut cards = Vec::new();
        let mut card_owned = Vec::<SaveSignal>::new();
        for request in &netlist.output_requests {
            if !matches!(
                request.directive,
                OutputDirectiveKind::Print | OutputDirectiveKind::Plot
            ) {
                continue;
            }
            if request.operands.len() != request.operand_kinds.len() {
                return Err(SimulationError::Netlist(format!(
                    "{} card at {} has {} authored operand(s) and {} typed operand(s)",
                    request.directive,
                    request.origin,
                    request.operands.len(),
                    request.operand_kinds.len()
                )));
            }
            for kind in &request.operand_kinds {
                if let OutputOperandKind::Probe(signal) = kind {
                    card_owned.push(signal.clone());
                }
            }
            cards.push(ProjectionCard {
                directive: request.directive,
                analysis: request.analysis,
                origin: request.origin.clone(),
                operands: request
                    .operands
                    .iter()
                    .cloned()
                    .zip(request.operand_kinds.iter().cloned())
                    .collect(),
            });
        }

        // The parser folds every directive's probes into one `SaveSet`, so a
        // symbol that no ordered card owns is exactly a `.SAVE`/`.PROBE`
        // request. Those are analysis-agnostic and must survive alongside a
        // `.PRINT` card instead of being replaced by it.
        let save_only = netlist
            .saves
            .signals
            .iter()
            .filter(|signal| {
                !card_owned
                    .iter()
                    .any(|owned| save_signals_match(owned, signal))
            })
            .cloned()
            .collect::<Vec<_>>();

        Ok(Self {
            cards,
            save_only,
            keeps_everything: netlist
                .saves
                .signals
                .iter()
                .any(|signal| matches!(signal, SaveSignal::All)),
            authored: !netlist.saves.signals.is_empty(),
        })
    }

    /// Whether the deck authored any output restriction at all.
    pub const fn authored(&self) -> bool {
        self.authored
    }

    /// Whether an `ALL` selector keeps every materialized signal.
    pub const fn keeps_everything(&self) -> bool {
        self.keeps_everything
    }

    /// Whether any ordered `.PRINT`/`.PLOT` card selects one analysis family.
    fn has_ordered_card(&self, kind: AnalysisResultKind) -> bool {
        !self
            .ordered_operands(projection_analysis_kind(kind))
            .is_empty()
    }

    /// Evaluate this deck's ordered `.PRINT`/`.PLOT TRAN` columns.
    ///
    /// `None` means the family has no ordered card to evaluate, so projection
    /// falls back to selecting from the materialized inventory. Expressions,
    /// hierarchy aliases, lead currents, and Xyce's `V(*)` selector are all
    /// owned by the core resolver rather than reimplemented here.
    pub fn ordered_transient_columns(
        &self,
        netlist: &Netlist,
        result: &crate::engine::TransientResult,
        limits: crate::ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Option<Vec<ProjectedSignal>>, SimulationError> {
        if self.keeps_everything || !self.has_ordered_card(AnalysisResultKind::Transient) {
            return Ok(None);
        }
        crate::analysis::evaluate_tran_output_requests_with_abort(netlist, result, limits, abort)
            .and_then(frontend_columns)
            .map(Some)
    }

    /// Evaluate this deck's ordered `.PRINT`/`.PLOT DC` columns.
    pub fn ordered_dc_columns(
        &self,
        netlist: &Netlist,
        sweep: &[(Value, crate::solver::SimulationResult)],
        limits: crate::ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Option<Vec<ProjectedSignal>>, SimulationError> {
        if self.keeps_everything || !self.has_ordered_card(AnalysisResultKind::DcSweep) {
            return Ok(None);
        }
        crate::analysis::evaluate_dc_output_requests_with_abort(netlist, sweep, limits, abort)
            .and_then(frontend_columns)
            .map(Some)
    }

    /// The `.PRINT`/`.PLOT` operands that select one analysis family, in card
    /// and operand order.
    fn ordered_operands(
        &self,
        analysis: Option<OutputAnalysisKind>,
    ) -> Vec<(&ProjectionCard, &(String, OutputOperandKind))> {
        self.cards
            .iter()
            .filter(|card| match (card.analysis, analysis) {
                (None, _) => true,
                (Some(card_analysis), Some(analysis)) => card_analysis == analysis,
                (Some(_), None) => false,
            })
            .flat_map(|card| card.operands.iter().map(move |operand| (card, operand)))
            .collect()
    }

    /// Project one analysis result onto its authored output contract.
    pub fn project(
        &self,
        params: &ParamContext,
        source: &ProjectionSource<'_>,
        abort: &dyn AbortSignal,
    ) -> Result<ProjectedSignals, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if !self.authored {
            return Ok(ProjectedSignals {
                signals: source
                    .signals
                    .iter()
                    .cloned()
                    .map(ProjectionSourceSignal::into_projected)
                    .collect(),
            });
        }

        let mut columns = Vec::new();
        if self.keeps_everything {
            columns.extend(
                source
                    .signals
                    .iter()
                    .cloned()
                    .map(ProjectionSourceSignal::into_projected),
            );
        } else if let Some(ordered) = &source.ordered_columns {
            columns.extend(ordered.iter().cloned());
        } else {
            let analysis = projection_analysis_kind(source.kind);
            for (card, (authored, kind)) in self.ordered_operands(analysis) {
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                columns.push(self.resolve_operand(params, source, card, authored, kind, abort)?);
            }
        }

        // `.SAVE`/`.PROBE` symbols are analysis-agnostic and additive: an
        // authored device observable must still be exported next to a
        // `.PRINT TRAN V(out)` column rather than being replaced by it.
        for signal in &self.save_only {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let Some(authored) = authored_save_symbol(signal) else {
                continue;
            };
            let matched = select_inventory(source, signal);
            if !matched.is_empty() {
                for index in matched {
                    let candidate = source.signals[index].clone().into_projected();
                    if !columns
                        .iter()
                        .any(|column| column.identity() == candidate.identity())
                    {
                        columns.push(candidate);
                    }
                }
                continue;
            }
            if columns.iter().any(|column| {
                column
                    .descriptor
                    .display_name()
                    .eq_ignore_ascii_case(&authored)
            }) {
                continue;
            }
            let card = ProjectionCard {
                directive: OutputDirectiveKind::Save,
                analysis: None,
                origin: NetlistSourceLocation::in_file("<.SAVE selection>", 0),
                operands: Vec::new(),
            };
            columns.push(self.resolve_operand(
                params,
                source,
                &card,
                &authored,
                &OutputOperandKind::Probe(signal.clone()),
                abort,
            )?);
        }

        Ok(ProjectedSignals { signals: columns })
    }

    fn resolve_operand(
        &self,
        params: &ParamContext,
        source: &ProjectionSource<'_>,
        card: &ProjectionCard,
        authored: &str,
        kind: &OutputOperandKind,
        abort: &dyn AbortSignal,
    ) -> Result<ProjectedSignal, SimulationError> {
        if let Some(column) = select_exact_inventory(source, authored, kind) {
            return Ok(column);
        }
        match source_value_type(source) {
            SignalValueType::Complex => resolve_complex_operand(source, authored, kind),
            _ => self.resolve_real_operand(params, source, card, authored, kind, abort),
        }
    }

    fn resolve_real_operand(
        &self,
        params: &ParamContext,
        source: &ProjectionSource<'_>,
        card: &ProjectionCard,
        authored: &str,
        kind: &OutputOperandKind,
        abort: &dyn AbortSignal,
    ) -> Result<ProjectedSignal, SimulationError> {
        let signals = source.real_signal_map();
        let index = CanonicalMeasureSignalIndex::new(&signals);
        let axis = source.axis.as_ref();
        match evaluate_output_operand(authored, kind, axis, &index, params, abort) {
            Ok(column) => {
                let (name, column_kind, values) = column.into_parts();
                let signal_kind = projected_signal_kind(column_kind, &name);
                let length = values.len();
                let descriptor = signal_descriptor(
                    &name,
                    registry_name(&name),
                    signal_kind,
                    SignalValueType::Real,
                )
                .map_err(|error| {
                    SimulationError::Netlist(format!(
                        "{} operand '{authored}' at {} cannot be described: {error}",
                        card.directive, card.origin
                    ))
                })?;
                Ok(ProjectedSignal {
                    descriptor,
                    values: ProjectionValues::Real(Cow::Owned(values)),
                    validity: vec![true; length],
                })
            }
            Err(OutputOperandEvaluationError::Aborted) => Err(SimulationError::Aborted),
            Err(OutputOperandEvaluationError::Detail { .. }) => Err(source.unavailable(authored)),
        }
    }
}

/// Convert the core resolver's frontend column tuples into typed columns.
fn frontend_columns(
    columns: Vec<crate::analysis::measure_signals::FrontendOutputColumn>,
) -> Result<Vec<ProjectedSignal>, SimulationError> {
    columns
        .into_iter()
        .map(|(name, physical_type, values)| {
            let kind = match physical_type {
                "voltage" => SignalKind::Voltage,
                "current" => SignalKind::Current,
                "parameter" => {
                    if device_of_parameter_probe(&name).is_some() {
                        SignalKind::DeviceObservable
                    } else {
                        SignalKind::Scalar
                    }
                }
                unexpected => {
                    return Err(SimulationError::Netlist(format!(
                        "output column '{name}' has unsupported physical type '{unexpected}'"
                    )));
                }
            };
            let length = values.len();
            let descriptor =
                signal_descriptor(&name, registry_name(&name), kind, SignalValueType::Real)
                    .map_err(|error| {
                        SimulationError::Netlist(format!(
                            "output column '{name}' cannot be described: {error}"
                        ))
                    })?;
            Ok(ProjectedSignal {
                descriptor,
                values: ProjectionValues::Real(Cow::Owned(values)),
                validity: vec![true; length],
            })
        })
        .collect()
}

/// Which typed kind an evaluated real column carries.
fn projected_signal_kind(kind: OutputColumnKind, name: &str) -> SignalKind {
    match kind {
        OutputColumnKind::Voltage => SignalKind::Voltage,
        OutputColumnKind::Current => SignalKind::Current,
        OutputColumnKind::Scalar => {
            if device_of_parameter_probe(name).is_some() {
                SignalKind::DeviceObservable
            } else {
                SignalKind::Scalar
            }
        }
    }
}

/// The rawfile variable type one signal kind serializes as.
///
/// Frontends format results; they do not decide what physical type a signal
/// has, so this mapping lives with the projection that produced the signal.
pub const fn raw_variable_type(kind: SignalKind) -> &'static str {
    match kind {
        SignalKind::Voltage => "voltage",
        SignalKind::Current => "current",
        SignalKind::Digital => "digital",
        SignalKind::DeviceObservable | SignalKind::Scalar => "parameter",
    }
}

/// The registry name a result's own metadata gives one column.
///
/// Result metadata occasionally arrives already wrapped (`V(out)`); an
/// ordinal is a last resort so a column is never anonymous.
fn result_registry_name(name: Option<&String>, fallback_index: usize) -> String {
    let candidate = name
        .map(|name| probe_registry_name(name).trim())
        .unwrap_or("");
    if candidate.is_empty() {
        fallback_index.to_string()
    } else {
        candidate.to_string()
    }
}

/// The signals one transient result materializes, in result order.
///
/// This is the inventory every surface projects against: node voltages,
/// branch currents, and digital traces sampled onto the analysis time grid.
/// Device observables stay out of it deliberately — they are resolvable
/// spellings through [`crate::analysis::measure_signals::transient_signal_map`],
/// not columns an unrestricted export should grow.
pub fn transient_projection_signals(
    result: &crate::engine::TransientResult,
) -> Result<Vec<ProjectionSourceSignal<'_>>, SignalSchemaError> {
    let mut signals = Vec::new();
    for (index, waveform) in result.voltages.iter().enumerate() {
        if !result.time.is_empty() && waveform.is_empty() {
            continue;
        }
        let registry = result_registry_name(result.node_names.get(index), index + 1);
        signals.push(ProjectionSourceSignal::new(
            format!("V({registry})"),
            &registry,
            SignalKind::Voltage,
            ProjectionValues::Real(Cow::Borrowed(waveform.as_slice())),
        )?);
    }
    for (index, waveform) in result.branch_currents.iter().enumerate() {
        if !result.time.is_empty() && waveform.is_empty() {
            continue;
        }
        let registry = result_registry_name(result.branch_names.get(index), index + 1);
        signals.push(ProjectionSourceSignal::new(
            format!("I({registry})"),
            &registry,
            SignalKind::Current,
            ProjectionValues::Real(Cow::Borrowed(waveform.as_slice())),
        )?);
    }
    for trace in &result.digital_traces {
        let sampled = sampled_digital_trace(
            &result.time,
            trace.points.iter().map(|point| (point.time, point.value)),
        );
        signals.push(ProjectionSourceSignal::new(
            format!("D({})", trace.node_name),
            &trace.node_name,
            SignalKind::Digital,
            ProjectionValues::Real(Cow::Owned(sampled)),
        )?);
    }
    Ok(signals)
}

/// Sample one event-driven digital trace onto the analysis time grid.
///
/// A state holds until the next recorded event, which is what a digital
/// waveform means; it is not interpolated and never invented.
fn sampled_digital_trace(
    times: &[Value],
    events: impl IntoIterator<Item = (Value, crate::xspice::DigitalValue)>,
) -> Vec<Value> {
    // Event times and grid times are both produced by the same integrator, so
    // an event that lands exactly on a grid point must not be pushed to the
    // next sample by a last-bit rounding difference.
    const TIME_EPSILON: Value = 1.0e-18;

    let mut events = events.into_iter().peekable();
    let mut values = Vec::with_capacity(times.len());
    let mut current = crate::xspice::DigitalValue::default();
    for &time in times {
        while events
            .peek()
            .is_some_and(|(event_time, _)| *event_time <= time + TIME_EPSILON)
        {
            if let Some((_, value)) = events.next() {
                current = value;
            }
        }
        values.push(match current.to_bool() {
            Some(false) => 0.0,
            Some(true) => 1.0,
            None => 0.5,
        });
    }
    values
}

/// The signals one DC solution materializes, in result order.
///
/// Ground is omitted: it is a reference, not a solved column.
pub fn operating_point_projection_signals(
    result: &crate::solver::SimulationResult,
) -> Result<Vec<ProjectionSourceSignal<'_>>, SignalSchemaError> {
    let mut signals = Vec::new();
    for (node_id, value) in result.node_voltages.iter().enumerate().skip(1) {
        let registry = result_registry_name(result.node_names.get(node_id), node_id);
        signals.push(ProjectionSourceSignal::new(
            format!("V({registry})"),
            &registry,
            SignalKind::Voltage,
            ProjectionValues::Real(Cow::Owned(vec![*value])),
        )?);
    }
    for (index, current) in result.branch_currents.iter().enumerate() {
        let registry = result_registry_name(result.branch_names.get(index), index + 1);
        signals.push(ProjectionSourceSignal::new(
            format!("I({registry})"),
            &registry,
            SignalKind::Current,
            ProjectionValues::Real(Cow::Owned(vec![*current])),
        )?);
    }
    Ok(signals)
}

/// Operating-point observables a single DC solution reports, as one-sample
/// lookup series.
///
/// These are resolvable spellings, not export columns: an unrestricted export
/// must not suddenly grow a column per internal observable, but an authored
/// `.SAVE @D1[Id]` must still find one.
pub fn operating_point_observable_series(
    result: &crate::solver::SimulationResult,
) -> Vec<(String, Vec<Value>)> {
    result
        .dc_observables
        .iter()
        .map(|(name, value)| (name.clone(), vec![*value]))
        .collect()
}

/// Operating-point observables reported at every point of a DC sweep.
///
/// A name that is absent from any point is omitted entirely rather than being
/// padded: a partially present observable is not a signal the sweep supplies,
/// and projecting it would have to invent values.
pub fn dc_sweep_observable_series(
    sweep: &[(Value, crate::solver::SimulationResult)],
) -> Vec<(String, Vec<Value>)> {
    let mut order = Vec::<String>::new();
    let mut values = HashMap::<String, Vec<Option<Value>>>::new();
    for (row, (_, result)) in sweep.iter().enumerate() {
        for (name, value) in &result.dc_observables {
            let slot = values.entry(name.clone()).or_insert_with(|| {
                order.push(name.clone());
                vec![None; sweep.len()]
            });
            slot[row] = Some(*value);
        }
    }
    order
        .into_iter()
        .filter_map(|name| {
            let samples = values.remove(&name)?;
            let complete = samples.iter().copied().collect::<Option<Vec<_>>>()?;
            Some((name, complete))
        })
        .collect()
}

/// Borrow an observable series as a resolver lookup table.
pub fn observable_lookup(series: &[(String, Vec<Value>)]) -> HashMap<String, &[Value]> {
    series
        .iter()
        .map(|(name, values)| (name.clone(), values.as_slice()))
        .collect()
}

/// The bare circuit symbol inside an authored probe spelling.
///
/// `V(out)` names node `out`, `I(V1)` names branch `v1`, and a spelling with
/// no accessor (`@d1[id]`) is already its own registry name.
pub fn probe_registry_name(display_name: &str) -> &str {
    registry_name(display_name)
}

/// Why an authored probe spelling is not well formed, if it is not.
///
/// Re-exported from `netlist`, where the probe grammar itself is parsed: the
/// arity contract belongs beside the grammar rather than beside the projection
/// that consumes it, and `analysis` needs it too without reaching up here.
pub use crate::netlist::probe_specification_error;

/// Whether a probe spelling names no circuit symbol at all.
///
/// A result whose metadata carries `V( )` or a blank name has lost the
/// identity of that column; projecting it would have to invent an ordinal.
pub fn probe_names_nothing(display_name: &str) -> bool {
    let trimmed = display_name.trim();
    if trimmed.is_empty() {
        return true;
    }
    match trimmed.find('(') {
        Some(open) if trimmed.ends_with(')') => {
            trimmed[open + 1..trimmed.len() - 1].trim().is_empty()
        }
        _ => false,
    }
}

fn registry_name(display_name: &str) -> &str {
    let trimmed = display_name.trim();
    let Some(open) = trimmed.find('(') else {
        return trimmed;
    };
    if !trimmed.ends_with(')') {
        return trimmed;
    }
    let inner = trimmed[open + 1..trimmed.len() - 1].trim();
    if inner.is_empty() { trimmed } else { inner }
}

fn source_value_type(source: &ProjectionSource<'_>) -> SignalValueType {
    source
        .signals
        .first()
        .map_or(SignalValueType::Real, |signal| signal.values.value_type())
}

/// The canonical authored spelling of one selected symbol.
fn authored_save_symbol(signal: &SaveSignal) -> Option<String> {
    match signal {
        SaveSignal::All => None,
        SaveSignal::Voltage(node) => Some(format!("V({node})")),
        SaveSignal::VoltageDiff(positive, negative) => Some(format!("V({positive},{negative})")),
        SaveSignal::Current(device) => Some(format!("I({device})")),
        SaveSignal::DeviceParam { device, param } => Some(format!("@{device}[{param}]")),
        SaveSignal::Raw(raw) => Some(raw.clone()),
    }
}

fn save_signals_match(first: &SaveSignal, second: &SaveSignal) -> bool {
    match (first, second) {
        (SaveSignal::All, SaveSignal::All) => true,
        (SaveSignal::Voltage(a), SaveSignal::Voltage(b))
        | (SaveSignal::Current(a), SaveSignal::Current(b))
        | (SaveSignal::Raw(a), SaveSignal::Raw(b)) => a.eq_ignore_ascii_case(b),
        (SaveSignal::VoltageDiff(a1, a2), SaveSignal::VoltageDiff(b1, b2)) => {
            a1.eq_ignore_ascii_case(b1) && a2.eq_ignore_ascii_case(b2)
        }
        (
            SaveSignal::DeviceParam {
                device: a_device,
                param: a_param,
            },
            SaveSignal::DeviceParam {
                device: b_device,
                param: b_param,
            },
        ) => a_device.eq_ignore_ascii_case(b_device) && a_param.eq_ignore_ascii_case(b_param),
        _ => false,
    }
}

/// Inventory entries selected by one authored symbol, using the `SaveSet`
/// wildcard language (`*` matches inside one hierarchy level).
fn select_inventory(source: &ProjectionSource<'_>, signal: &SaveSignal) -> Vec<usize> {
    let selection = SaveSet {
        signals: vec![signal.clone()],
    };
    source
        .signals
        .iter()
        .enumerate()
        .filter(|(_, candidate)| {
            let display = candidate.descriptor.display_name();
            selection.selects(display)
                || (candidate.descriptor.kind() == SignalKind::Digital
                    && selection.selects_raw_name(registry_name(display)))
                || complex_alias_selects(&selection, candidate)
        })
        .map(|(index, _)| index)
        .collect()
}

/// Whether an authored complex accessor (`VM(out)`, `IDB(v1)`, ...) selects a
/// materialized complex signal.
///
/// Magnitude, phase, and decibel accessors are renderings of one complex
/// column, so they select it rather than being a different signal.
fn complex_alias_selects(selection: &SaveSet, candidate: &ProjectionSourceSignal<'_>) -> bool {
    if candidate.values.value_type() != SignalValueType::Complex {
        return false;
    }
    let display = candidate.descriptor.display_name();
    let registry = registry_name(display);
    selection.signals.iter().any(|signal| {
        let SaveSignal::Raw(authored) = signal else {
            return false;
        };
        let Some((operator, argument)) = split_accessor(authored) else {
            return false;
        };
        let compatible = match candidate.descriptor.kind() {
            SignalKind::Voltage => {
                matches!(operator.as_str(), "V" | "VR" | "VI" | "VM" | "VP" | "VDB")
            }
            SignalKind::Current => {
                matches!(operator.as_str(), "I" | "IR" | "II" | "IM" | "IP" | "IDB")
            }
            SignalKind::Digital | SignalKind::Scalar | SignalKind::DeviceObservable => false,
        };
        compatible && argument.eq_ignore_ascii_case(registry)
    })
}

fn split_accessor(authored: &str) -> Option<(String, String)> {
    let compact = authored
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect::<String>();
    let (operator, rest) = compact.split_once('(')?;
    let argument = rest.strip_suffix(')')?;
    Some((operator.to_ascii_uppercase(), argument.to_string()))
}

/// Take one inventory entry whose display name is exactly the authored probe.
fn select_exact_inventory(
    source: &ProjectionSource<'_>,
    authored: &str,
    kind: &OutputOperandKind,
) -> Option<ProjectedSignal> {
    if !matches!(kind, OutputOperandKind::Probe(_)) {
        return None;
    }
    source
        .signals
        .iter()
        .find(|candidate| {
            candidate
                .descriptor
                .display_name()
                .eq_ignore_ascii_case(authored.trim())
        })
        .cloned()
        .map(ProjectionSourceSignal::into_projected)
}

/// Resolve one authored operand against a complex-valued result.
///
/// Complex families carry node voltages, branch currents, and device
/// observables. Differential probes are synthesized from two materialized
/// node voltages; anything the result cannot supply — including an authored
/// output expression, which has no complex evaluator — is a typed error.
fn resolve_complex_operand(
    source: &ProjectionSource<'_>,
    authored: &str,
    kind: &OutputOperandKind,
) -> Result<ProjectedSignal, SimulationError> {
    // Magnitude, phase, real, imaginary and decibel accessors are renderings
    // of one complex column, so they select it rather than naming a different
    // signal. The exporter decides how to render; the projection decides only
    // which column the deck asked for.
    if let Some(column) = complex_accessor_column(source, authored) {
        return Ok(column);
    }
    let OutputOperandKind::Probe(signal) = kind else {
        return Err(SimulationError::Netlist(format!(
            "output expression '{authored}' is not evaluable over a complex {} result",
            source.instance
        )));
    };
    if let SaveSignal::VoltageDiff(positive, negative) = signal {
        let (positive_real, positive_imag) =
            complex_node(source, positive).ok_or_else(|| source.unavailable(authored))?;
        let (negative_real, negative_imag) =
            complex_node(source, negative).ok_or_else(|| source.unavailable(authored))?;
        if positive_real.len() != negative_real.len() || positive_imag.len() != negative_imag.len()
        {
            return Err(SimulationError::result_schema_mismatch(
                source.instance.clone(),
                source.coordinate.clone(),
                "voltage",
                vec![format!("V({positive})")],
                vec![format!("V({negative})")],
                positive_real.len(),
                negative_real.len(),
            ));
        }
        let real = positive_real
            .iter()
            .zip(&negative_real)
            .map(|(high, low)| high - low)
            .collect::<Vec<_>>();
        let imag = positive_imag
            .iter()
            .zip(&negative_imag)
            .map(|(high, low)| high - low)
            .collect::<Vec<_>>();
        let length = real.len();
        let registry = format!("{positive},{negative}");
        let descriptor = signal_descriptor(
            authored,
            &registry,
            SignalKind::Voltage,
            SignalValueType::Complex,
        )
        .map_err(|error| {
            SimulationError::Netlist(format!(
                "differential probe '{authored}' cannot be described: {error}"
            ))
        })?;
        return Ok(ProjectedSignal {
            descriptor,
            values: ProjectionValues::Complex {
                real: Cow::Owned(real),
                imag: Cow::Owned(imag),
            },
            validity: vec![true; length],
        });
    }
    Err(source.unavailable(authored))
}

/// The materialized complex column one authored accessor addresses.
fn complex_accessor_column(
    source: &ProjectionSource<'_>,
    authored: &str,
) -> Option<ProjectedSignal> {
    let (operator, argument) = split_accessor(authored)?;
    let expected = match operator.as_str() {
        "V" | "VR" | "VI" | "VM" | "VP" | "VDB" => SignalKind::Voltage,
        "I" | "IR" | "II" | "IM" | "IP" | "IDB" => SignalKind::Current,
        _ => return None,
    };
    source
        .signals
        .iter()
        .find(|candidate| {
            candidate.descriptor.kind() == expected
                && candidate.values.value_type() == SignalValueType::Complex
                && registry_name(candidate.descriptor.display_name())
                    .eq_ignore_ascii_case(&argument)
        })
        .cloned()
        .map(ProjectionSourceSignal::into_projected)
}

fn complex_node(source: &ProjectionSource<'_>, node: &str) -> Option<(Vec<Value>, Vec<Value>)> {
    let sample_count = source
        .signals
        .first()
        .map_or(0, |signal| signal.values.len());
    if node.trim() == "0" {
        return Some((vec![0.0; sample_count], vec![0.0; sample_count]));
    }
    source.signals.iter().find_map(|candidate| {
        if candidate.descriptor.kind() != SignalKind::Voltage {
            return None;
        }
        if !registry_name(candidate.descriptor.display_name()).eq_ignore_ascii_case(node.trim()) {
            return None;
        }
        match &candidate.values {
            ProjectionValues::Complex { real, imag } => {
                Some((real.as_ref().to_vec(), imag.as_ref().to_vec()))
            }
            ProjectionValues::Real(_) => None,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(source: &str) -> Netlist {
        Netlist::parse(source).expect("test netlist parses")
    }

    fn real_signal(
        display: &str,
        registry: &str,
        kind: SignalKind,
        values: Vec<Value>,
    ) -> ProjectionSourceSignal<'static> {
        ProjectionSourceSignal::new(
            display,
            registry,
            kind,
            ProjectionValues::Real(Cow::Owned(values)),
        )
        .expect("valid source signal")
    }

    fn complex_signal(
        display: &str,
        registry: &str,
        kind: SignalKind,
        real: Vec<Value>,
        imag: Vec<Value>,
    ) -> ProjectionSourceSignal<'static> {
        ProjectionSourceSignal::new(
            display,
            registry,
            kind,
            ProjectionValues::Complex {
                real: Cow::Owned(real),
                imag: Cow::Owned(imag),
            },
        )
        .expect("valid complex source signal")
    }

    fn names(projected: &ProjectedSignals) -> Vec<String> {
        projected
            .signals()
            .iter()
            .map(|signal| signal.descriptor().display_name().to_string())
            .collect()
    }

    #[test]
    fn an_absent_sample_stays_absent_and_the_schema_describes_the_selection() {
        let netlist =
            parse("partial retention\nV1 out 0 1\nR1 out 0 1k\n.TRAN 1u 10u\n.SAVE V(out)\n.END\n");
        let projection = SignalProjection::from_netlist(&netlist).expect("projection builds");
        let validity = vec![true, false, true];
        let source = ProjectionSource::new(AnalysisResultKind::Transient, "TRAN")
            .with_axis(vec![0.0, 1.0, 2.0])
            .with_signals(vec![
                real_signal("V(out)", "out", SignalKind::Voltage, vec![1.0, 0.0, 3.0])
                    .with_validity(validity.clone()),
            ]);
        let projected = projection
            .project(&netlist.params, &source, &crate::abort_signal::NoAbort)
            .expect("a retained signal projects");
        assert_eq!(projected.signals()[0].validity(), validity.as_slice());

        let schema = projected.schema().expect("projected schema is well formed");
        let described = schema
            .descriptors()
            .iter()
            .map(SignalDescriptor::canonical_name)
            .collect::<Vec<_>>();
        assert_eq!(described, ["v(out)"]);
    }

    #[test]
    fn every_analysis_family_has_an_explicit_output_qualifier_decision() {
        for kind in AnalysisResultKind::ALL {
            // The match in `projection_analysis_kind` is exhaustive; this
            // asserts the decision is reachable for every registered family.
            let _ = projection_analysis_kind(kind);
        }
        assert_eq!(
            projection_analysis_kind(AnalysisResultKind::Transient),
            Some(OutputAnalysisKind::Tran)
        );
        assert_eq!(projection_analysis_kind(AnalysisResultKind::PoleZero), None);
    }

    #[test]
    fn an_unauthored_deck_keeps_every_materialized_signal() {
        let netlist = parse("no outputs\nV1 out 0 1\nR1 out 0 1k\n.OP\n.END\n");
        let projection = SignalProjection::from_netlist(&netlist).expect("projection builds");
        assert!(!projection.authored());
        let source = ProjectionSource::new(AnalysisResultKind::OperatingPoint, "DC OP")
            .with_axis(vec![0.0])
            .with_signals(vec![
                real_signal("V(out)", "out", SignalKind::Voltage, vec![1.0]),
                real_signal("I(V1)", "v1", SignalKind::Current, vec![-1.0e-3]),
            ]);
        let projected = projection
            .project(&netlist.params, &source, &crate::abort_signal::NoAbort)
            .expect("unauthored projection keeps everything");
        assert_eq!(names(&projected), ["V(out)", "I(V1)"]);
    }

    #[test]
    fn a_save_symbol_survives_alongside_a_print_card() {
        let netlist = parse(
            "save plus print\nV1 out 0 1\nR1 out 0 1k\n.TRAN 1u 10u\n.PRINT TRAN V(out)\n.SAVE I(V1)\n.END\n",
        );
        let projection = SignalProjection::from_netlist(&netlist).expect("projection builds");
        let source = ProjectionSource::new(AnalysisResultKind::Transient, "TRAN")
            .with_axis(vec![0.0, 1.0])
            .with_signals(vec![
                real_signal("V(out)", "out", SignalKind::Voltage, vec![1.0, 1.0]),
                real_signal("I(V1)", "v1", SignalKind::Current, vec![-1.0, -1.0]),
                real_signal("V(other)", "other", SignalKind::Voltage, vec![2.0, 2.0]),
            ])
            .with_ordered_print_columns(Some(vec![
                ProjectedSignal::new(
                    signal_descriptor("V(out)", "out", SignalKind::Voltage, SignalValueType::Real)
                        .expect("descriptor"),
                    ProjectionValues::Real(Cow::Owned(vec![1.0, 1.0])),
                    vec![true, true],
                )
                .expect("column"),
            ]));
        let projected = projection
            .project(&netlist.params, &source, &crate::abort_signal::NoAbort)
            .expect("projection succeeds");
        assert_eq!(names(&projected), ["V(out)", "I(V1)"]);
    }

    #[test]
    fn an_unsupplied_authored_symbol_is_a_typed_unavailable_error() {
        let netlist =
            parse("missing save\nV1 out 0 1\nR1 out 0 1k\n.TRAN 1u 10u\n.SAVE @D1[Id]\n.END\n");
        let projection = SignalProjection::from_netlist(&netlist).expect("projection builds");
        let source = ProjectionSource::new(AnalysisResultKind::Transient, "TRAN")
            .with_axis(vec![0.0, 1.0])
            .with_signals(vec![real_signal(
                "V(out)",
                "out",
                SignalKind::Voltage,
                vec![1.0, 1.0],
            )]);
        let error = projection
            .project(&netlist.params, &source, &crate::abort_signal::NoAbort)
            .expect_err("an absent device observable cannot be exported");
        let SimulationError::RequestedSignalUnavailable(detail) = error else {
            panic!("expected a typed unavailable-signal error");
        };
        assert_eq!(detail.signal, "@D1[Id]");
        assert_eq!(detail.analysis_label, "TRAN");
    }

    #[test]
    fn a_complex_differential_probe_is_synthesized_from_two_node_voltages() {
        let netlist = parse(
            "differential\nV1 a 0 AC 1\nR1 a b 1k\nR2 b 0 1k\n.AC DEC 1 1 10\n.SAVE V(a,b)\n.END\n",
        );
        let projection = SignalProjection::from_netlist(&netlist).expect("projection builds");
        let source = ProjectionSource::new(AnalysisResultKind::Ac, "AC")
            .with_axis(vec![1.0, 10.0])
            .with_signals(vec![
                complex_signal(
                    "V(a)",
                    "a",
                    SignalKind::Voltage,
                    vec![1.0, 1.0],
                    vec![0.0, 0.0],
                ),
                complex_signal(
                    "V(b)",
                    "b",
                    SignalKind::Voltage,
                    vec![0.5, 0.5],
                    vec![0.25, 0.25],
                ),
            ]);
        let projected = projection
            .project(&netlist.params, &source, &crate::abort_signal::NoAbort)
            .expect("differential resolves");
        assert_eq!(names(&projected), ["V(a,b)"]);
        let (real, imag) = projected.signals()[0].complex().expect("complex column");
        assert_eq!(real, [0.5, 0.5]);
        assert_eq!(imag, [-0.25, -0.25]);
    }

    #[test]
    fn a_wildcard_save_expands_against_the_materialized_inventory() {
        let netlist = parse("wildcard\nV1 out 0 1\nR1 out 0 1k\n.TRAN 1u 10u\n.SAVE V(o*)\n.END\n");
        let projection = SignalProjection::from_netlist(&netlist).expect("projection builds");
        let source = ProjectionSource::new(AnalysisResultKind::Transient, "TRAN")
            .with_axis(vec![0.0])
            .with_signals(vec![
                real_signal("V(out)", "out", SignalKind::Voltage, vec![1.0]),
                real_signal("V(other)", "other", SignalKind::Voltage, vec![2.0]),
                real_signal("V(net3)", "net3", SignalKind::Voltage, vec![3.0]),
            ]);
        let projected = projection
            .project(&netlist.params, &source, &crate::abort_signal::NoAbort)
            .expect("wildcard resolves");
        assert_eq!(names(&projected), ["V(out)", "V(other)"]);
    }
}
