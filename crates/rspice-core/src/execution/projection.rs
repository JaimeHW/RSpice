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
//! outside this module, so the CLI and the engine adapter cannot disagree
//! about what a deck asked for.
//!
//! An authored symbol the analysis cannot supply is a typed
//! [`SimulationError::RequestedSignalUnavailable`] naming the original
//! spelling and the analysis instance. It is never an empty column, a zero, or
//! a silently narrower export.

use std::borrow::Cow;
use std::collections::HashMap;

use crate::abort_signal::AbortSignal;
use crate::analysis::measure_signals::{
    CanonicalMeasureSignalIndex, NoiseSweepSeries, OutputColumnKind, OutputOperandEvaluationError,
    PhaseUnit, evaluate_output_operand,
};
use crate::engine::SimulationError;
use crate::netlist::{
    Netlist, NetlistSourceLocation, OutputAnalysisKind, OutputDirectiveKind, OutputOperandKind,
    ParamContext, SaveSignal,
};
use crate::{Value, netlist::SaveSet};

use super::capability::AnalysisResultKind;

mod complex_expression;
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
        | AnalysisResultKind::Pxf
        | AnalysisResultKind::PNoise
        | AnalysisResultKind::Pstb
        | AnalysisResultKind::Envelope
        | AnalysisResultKind::Qpss
        | AnalysisResultKind::Qpac
        | AnalysisResultKind::Qpxf
        | AnalysisResultKind::Qpnoise
        | AnalysisResultKind::DcMatch => None,
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

    /// The same column with a node potential in its discipline's units
    /// (`K` on a thermal net) where they are not volts.
    fn in_potential_units(mut self, units: Option<&str>) -> Result<Self, SignalSchemaError> {
        if let Some(units) = units {
            let descriptor = &self.descriptor;
            self.descriptor = SignalDescriptor::new(
                descriptor.canonical_name(),
                descriptor.display_name(),
                descriptor.kind(),
                SignalUnit::Custom(units.to_owned()),
                descriptor.value_type(),
                descriptor.shape(),
                descriptor.owner().clone(),
            )?;
        }
        Ok(self)
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
    let owner = match kind {
        SignalKind::Voltage | SignalKind::Digital => SignalOwner::Node(registry_name.to_string()),
        SignalKind::Current => SignalOwner::Branch(registry_name.to_string()),
        SignalKind::DeviceObservable => SignalOwner::Device(
            device_of_parameter_probe(registry_name)
                .unwrap_or(registry_name)
                .to_string(),
        ),
        SignalKind::Scalar => SignalOwner::Analysis,
    };
    let unit = signal_unit(kind);
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

    /// Whether `name` is one of the additional lookup spellings itself, in
    /// any case and either hierarchy separator (`X1:RS:TC1` is the flattened
    /// `X1.RS:TC1`).
    fn looks_up(&self, name: &str) -> bool {
        let name = crate::netlist::canonical_symbol(name);
        self.lookup
            .keys()
            .any(|key| crate::netlist::canonical_symbol(key) == name)
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
    /// Whether a `save_only` symbol no vector answers to is passed over.
    ignores_unknown_saves: bool,
    keeps_everything: bool,
    authored: bool,
    /// The unit a rendered phase (`VP(x)`, `ph(v(x))`) is printed in.
    phase_unit: PhaseUnit,
    /// Whether an AC or HB spectrum expression is evaluated in complex
    /// arithmetic, as Xyce evaluates it ([`complex_expression`]).
    complex_expressions: bool,
    /// Whether a `.PRINT DC` card also prints the deck's `.OP`, which Xyce
    /// runs as a one-point DC analysis (IOInterfacePKG/N_IO_OutputMgr.C:635-641).
    dc_cards_print_operating_point: bool,
    /// The names a `.PRINT AC` operand prints the sweep frequency by
    /// ([`frequency_axis_column`]).
    frequency_axis_names: &'static [&'static str],
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
            if request.operands.len() != request.operand_kinds().len() {
                return Err(SimulationError::Netlist(format!(
                    "{} card at {} has {} authored operand(s) and {} typed operand(s)",
                    request.directive,
                    request.origin,
                    request.operands.len(),
                    request.operand_kinds().len()
                )));
            }
            for kind in request.operand_kinds() {
                match kind {
                    OutputOperandKind::Probe(signal) => card_owned.push(signal.clone()),
                    // The parser saves an accessor or expression operand
                    // (`VR(a)`, `{V(a)*2}`) under its raw spelling. That save
                    // belongs to the card's analysis: a `.PRINT AC VR(a)`
                    // does not ask the `.OP` beside it for a column.
                    OutputOperandKind::Expression { body } => {
                        card_owned.push(SaveSignal::Raw(body.clone()));
                    }
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
                    .zip(request.operand_kinds().iter().cloned())
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

        // Expression-only PRINT/PLOT cards do not populate the probe SaveSet,
        // but still restrict the output to their authored expression columns.
        let authored = !cards.is_empty() || !netlist.saves.signals.is_empty();
        Ok(Self {
            cards,
            save_only,
            ignores_unknown_saves: netlist.saves.ignores_unknown_names,
            keeps_everything: netlist
                .saves
                .signals
                .iter()
                .any(|signal| matches!(signal, SaveSignal::All)),
            authored,
            phase_unit: PhaseUnit::of(netlist),
            complex_expressions: netlist.parse_dialect() == crate::config::SpiceDialect::Xyce,
            dc_cards_print_operating_point: netlist.parse_dialect()
                == crate::config::SpiceDialect::Xyce,
            frequency_axis_names: match netlist.parse_dialect() {
                crate::config::SpiceDialect::Xyce => &["FREQ", "HERTZ"],
                crate::config::SpiceDialect::Ngspice => &["FREQUENCY"],
                crate::config::SpiceDialect::BestAvailable => &["FREQ"],
            },
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

    /// Evaluate this deck's ordered `.PRINT`/`.PLOT NOISE` columns.
    ///
    /// Xyce prints exactly the operands of a `.PRINT NOISE` card. `V(x)` and
    /// `I(x)` are the complex small-signal solution and its accessors render
    /// one real column, as on an AC card; `ONOISE`, `INOISE`, `DNO()` and
    /// `DNI()` are spectral densities in V²/Hz or A²/Hz
    /// (N_ANP_NOISE.C:1390-1393, N_IO_Op.C:123-141), and an expression reads
    /// them. ngspice's `onoise_spectrum` and `inoise_spectrum` keep the
    /// amplitude densities of the default noise table. `operating_point`
    /// holds the `device:param` values of the analysis's bias point
    /// (`Engine::run_noise_named_reporting_operating_point_with_abort`),
    /// which an operand naming one prints on every row. `None` means the deck
    /// has no such card, so the frontend keeps that default table.
    pub fn ordered_noise_columns(
        &self,
        netlist: &Netlist,
        sweep: &[crate::analysis::NoiseResult],
        operating_point: &[(String, Value)],
        abort: &dyn AbortSignal,
    ) -> Result<Option<Vec<ProjectedSignal>>, SimulationError> {
        if self.keeps_everything || !self.has_ordered_card(AnalysisResultKind::Noise) {
            return Ok(None);
        }
        let Some(series) = NoiseSweepSeries::from_sweep_with_abort(sweep, self.phase_unit, abort)?
        else {
            return Ok(None);
        };
        let output_amplitude = sweep
            .iter()
            .map(crate::analysis::NoiseResult::output_noise_rms)
            .collect::<Vec<_>>();
        let input_amplitude = sweep
            .iter()
            .map(crate::analysis::NoiseResult::input_referred_rms)
            .collect::<Vec<_>>();
        let parameter_rows = operating_point
            .iter()
            .map(|(name, value)| (name.as_str(), vec![*value; sweep.len()]))
            .collect::<Vec<_>>();
        let printed_measures = crate::analysis::measure_signals::printed_noise_measure_traces(
            netlist,
            sweep,
            &crate::ResourceLimits::default(),
            abort,
        )?;
        let mut lookup = series.signal_map();
        lookup.retain(|name, _| {
            !name.eq_ignore_ascii_case("onoise_spectrum")
                && !name.eq_ignore_ascii_case("inoise_spectrum")
        });
        for (name, values) in [
            ("onoise_spectrum", &output_amplitude),
            ("inoise_spectrum", &input_amplitude),
        ] {
            lookup.insert(name.to_string(), values.as_slice());
            lookup.insert(format!("V({name})"), values.as_slice());
        }
        crate::analysis::measure_signals::insert_printed_measure_traces(
            &mut lookup,
            &printed_measures,
        );
        for (name, values) in &parameter_rows {
            lookup
                .entry((*name).to_string())
                .or_insert(values.as_slice());
        }
        let source = ProjectionSource::new(AnalysisResultKind::Noise, "NOISE")
            .with_axis(series.axis())
            .with_signals(noise_inventory(sweep)?)
            .with_lookup(lookup);
        let mut columns = Vec::new();
        for (card, (authored, kind)) in self.ordered_operands(Some(OutputAnalysisKind::Noise)) {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let column = match resolve_complex_operand(&source, authored, kind, self.phase_unit) {
                Ok(column) => column,
                Err(_) => self.resolve_real_operand(
                    &netlist.params,
                    &source,
                    card,
                    authored,
                    kind,
                    abort,
                )?,
            };
            columns.push(column);
        }
        Ok(Some(columns))
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
                (Some(OutputAnalysisKind::Dc), Some(OutputAnalysisKind::Op)) => {
                    self.dc_cards_print_operating_point
                }
                (Some(card_analysis), Some(analysis)) => card_analysis == analysis,
                (Some(_), None) => false,
            })
            .flat_map(|card| card.operands.iter().map(move |operand| (card, operand)))
            .collect()
    }

    /// Whether the deck's own output cards shape this projection. When they
    /// do not, [`Self::project`] exports the inventory it is given unchanged.
    pub fn is_authored(&self) -> bool {
        self.authored
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
        // The complex columns a rendering accessor printed (`VM(out)` reads
        // `V(out)`), so the `.SAVE` the parser records beside an ordered
        // operand does not export that probe a second time.
        let mut rendered_sources = Vec::new();
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
                if source_value_type(source) == SignalValueType::Complex
                    && complex_rendering(authored).is_some()
                    && let Ok(column) = complex_operand_column(source, authored, kind)
                {
                    rendered_sources.push(column.identity());
                }
                if self.complex_expressions
                    && matches!(
                        source.kind,
                        AnalysisResultKind::Ac | AnalysisResultKind::HarmonicBalance
                    )
                    && let OutputOperandKind::Expression { body } = kind
                    && let Some(expression) = complex_expression::xyce_ac_expression_columns(
                        source,
                        params,
                        authored,
                        body,
                        self.phase_unit,
                        abort,
                    )?
                {
                    columns.extend(expression);
                    continue;
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
                    // `select_inventory` enumerates `source.signals`, so the
                    // index is one it just produced.
                    let Some(inventory) = source.signals.get(index) else {
                        continue;
                    };
                    let candidate = inventory.clone().into_projected();
                    if !rendered_sources.contains(&candidate.identity())
                        && !columns
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
            match self.resolve_operand(
                params,
                source,
                &card,
                &authored,
                &OutputOperandKind::Probe(signal.clone()),
                abort,
            ) {
                Err(SimulationError::RequestedSignalUnavailable(_))
                    if self.ignores_unknown_saves =>
                {
                    // ngspice keeps an empty vector for a device quantity no
                    // device answers to (frontend/outitf.c, addSpecialDesc);
                    // the name is passed over for any other kind of signal.
                    if matches!(signal, SaveSignal::DeviceParam { .. }) {
                        columns.push(unanswered_device_quantity(&authored, source)?);
                    }
                }
                resolved => columns.push(resolved?),
            }
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
        if let Some(column) = frequency_axis_column(source, authored, self.frequency_axis_names) {
            return column;
        }
        match source_value_type(source) {
            // A real series the caller looked up for a complex result, as the
            // running value of a measure Xyce prints by name, prints as is.
            SignalValueType::Complex if source.looks_up(authored) => {
                self.resolve_real_operand(params, source, card, authored, kind, abort)
            }
            // An expression over those real series alone (`{R1:TC1*2}`)
            // prints as a real column too; one that reads the complex
            // result keeps the complex refusal.
            SignalValueType::Complex
                if matches!(kind, OutputOperandKind::Expression { .. })
                    && !source.lookup.is_empty() =>
            {
                match self.resolve_real_operand(params, source, card, authored, kind, abort) {
                    Err(SimulationError::Aborted) => Err(SimulationError::Aborted),
                    Ok(column) => Ok(column),
                    Err(_) => resolve_complex_operand(source, authored, kind, self.phase_unit),
                }
            }
            SignalValueType::Complex => {
                resolve_complex_operand(source, authored, kind, self.phase_unit)
            }
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

/// The column of a saved `@device[quantity]` that nothing computes: every
/// sample of the axis reads zero, which is what ngspice writes for it.
fn unanswered_device_quantity(
    authored: &str,
    source: &ProjectionSource<'_>,
) -> Result<ProjectedSignal, SimulationError> {
    let length = source.axis.as_ref().len();
    let descriptor = signal_descriptor(
        authored,
        registry_name(authored),
        SignalKind::DeviceObservable,
        SignalValueType::Real,
    )
    .map_err(|error| {
        SimulationError::Netlist(format!("save '{authored}' cannot be described: {error}"))
    })?;
    ProjectedSignal::new(
        descriptor,
        ProjectionValues::Real(Cow::Owned(vec![0.0; length])),
        vec![true; length],
    )
}

/// Evaluate a deck's `.PRINT SENS` columns at each operating point of a Xyce
/// `.SENS` study, in card and operand order.
pub fn sensitivity_print_columns(
    netlist: &Netlist,
    sweep: &[(Value, crate::solver::SimulationResult)],
    limits: crate::ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<ProjectedSignals, SimulationError> {
    crate::analysis::measure_signals::evaluate_sens_output_requests_with_abort(
        netlist, sweep, limits, abort,
    )
    .and_then(frontend_columns)
    .map(|signals| ProjectedSignals { signals })
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

/// The physical unit one signal kind's column carries.
///
/// The same fact [`raw_variable_type`] states as a rawfile type name, in the
/// vocabulary a descriptor and a published artifact use. It is one function
/// because [`signal_descriptor`] stamps every descriptor with it and a
/// frontend that has only the kind — the CLI's export rows keep the kind, not
/// the whole descriptor — must reach the same answer, or one run's `.raw`
/// header and its `.h5` unit attribute would describe different quantities.
///
/// [`SignalUnit::symbol`] turns the result into the string an artifact writes.
#[must_use]
pub fn signal_unit(kind: SignalKind) -> SignalUnit {
    match kind {
        SignalKind::Voltage => SignalUnit::Volt,
        SignalKind::Current => SignalUnit::Ampere,
        SignalKind::Digital => SignalUnit::Logic,
        SignalKind::DeviceObservable | SignalKind::Scalar => SignalUnit::Dimensionless,
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
/// branch currents, and digital (`D(node)`) and real (`E(node)`) event traces
/// sampled onto the analysis time grid.
/// Device observables stay out of it deliberately — they are resolvable
/// spellings through [`crate::analysis::measure_signals::transient_signal_map`],
/// not columns an unrestricted export should grow.
pub fn transient_projection_signals(
    result: &crate::engine::TransientResult,
) -> Result<Vec<ProjectionSourceSignal<'_>>, SignalSchemaError> {
    let mut signals = Vec::new();
    let mut instance_currents = Vec::new();
    for (index, waveform) in result.voltages.iter().enumerate() {
        if !result.time.is_empty() && waveform.is_empty() {
            continue;
        }
        let registry = result_registry_name(result.node_names.get(index), index + 1);
        let values = ProjectionValues::Real(Cow::Borrowed(waveform.as_slice()));
        // A Verilog-A instance's own unknowns, listed as the operating-point
        // inventory below lists them.
        match result.instance_unknowns.get(index).copied().flatten() {
            None => signals.push(
                ProjectionSourceSignal::new(
                    format!("V({registry})"),
                    &registry,
                    SignalKind::Voltage,
                    values,
                )?
                .in_potential_units(result.potential_units.get(index).and_then(Option::as_deref))?,
            ),
            Some(crate::solver::InstanceUnknown::BranchCurrent) => {
                instance_currents.push(ProjectionSourceSignal::new(
                    format!("I({registry})"),
                    &registry,
                    SignalKind::Current,
                    values,
                )?);
            }
            Some(crate::solver::InstanceUnknown::Private) => {}
        }
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
    signals.append(&mut instance_currents);
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
    for trace in &result.real_traces {
        let sampled = sampled_real_trace(
            &result.time,
            trace.points.iter().map(|point| (point.time, point.value)),
        );
        signals.push(ProjectionSourceSignal::new(
            format!("E({})", trace.node_name),
            &trace.node_name,
            SignalKind::Scalar,
            ProjectionValues::Real(Cow::Owned(sampled)),
        )?);
    }
    Ok(signals)
}

/// Sample one real event trace onto the analysis time grid, as the `E(node)`
/// column its rawfile event plot is named by.
///
/// Each value holds until the next recorded event, as [`sampled_digital_trace`]
/// holds a level. Before the first event the column reads `0.0`, the value
/// XSPICE's real node type initializes an event node to; a table cannot carry
/// an absent sample, and a NaN would fail every export's finiteness check.
fn sampled_real_trace(
    times: &[Value],
    events: impl IntoIterator<Item = (Value, Value)>,
) -> Vec<Value> {
    // The same last-bit allowance `sampled_digital_trace` makes.
    const TIME_EPSILON: Value = 1.0e-18;

    let mut events = events.into_iter().peekable();
    let mut values = Vec::with_capacity(times.len());
    let mut current = 0.0;
    for &time in times {
        while let Some((_, value)) =
            events.next_if(|(event_time, _)| *event_time <= time + TIME_EPSILON)
        {
            current = value;
        }
        values.push(current);
    }
    values
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
    let mut instance_currents = Vec::new();
    for (node_id, value) in result.node_voltages.iter().enumerate().skip(1) {
        // A net only the event domain resolves publishes no voltage column:
        // its slot holds the placeholder row the assembly closed with `v = 0`,
        // and every table built from this inventory — the CLI's OP print, the
        // rawfile, the HDF5 and CSV exports — would otherwise carry 0 V for a
        // net that carries events.
        if result.event_only_node_kind(node_id).is_some() {
            continue;
        }
        let registry = result_registry_name(result.node_names.get(node_id), node_id);
        let values = ProjectionValues::Real(Cow::Owned(vec![*value]));
        // A Verilog-A instance's own unknowns are not nets: a named branch's
        // current is listed with the currents, in amperes, and an unknown the
        // module never named is not listed at all.
        match result.instance_unknown(node_id) {
            None => signals.push(
                ProjectionSourceSignal::new(
                    format!("V({registry})"),
                    &registry,
                    SignalKind::Voltage,
                    values,
                )?
                .in_potential_units(result.potential_units(node_id))?,
            ),
            Some(crate::solver::InstanceUnknown::BranchCurrent) => {
                instance_currents.push(ProjectionSourceSignal::new(
                    format!("I({registry})"),
                    &registry,
                    SignalKind::Current,
                    values,
                )?);
            }
            Some(crate::solver::InstanceUnknown::Private) => {}
        }
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
    signals.extend(instance_currents);
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
            // The slot was sized from this same sweep on first insertion.
            if let Some(cell) = slot.get_mut(row) {
                *cell = Some(*value);
            }
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
        ..SaveSet::default()
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
        complex_accessor(authored).is_some_and(|(kind, argument)| {
            kind == candidate.descriptor.kind() && argument.eq_ignore_ascii_case(registry)
        })
    })
}

/// The complex column an authored accessor renders: `VM(out)`, `IDB(v1)`,
/// or ngspice's vector functions of one probe, `db(i(v1))`, `ph(v(out))`.
/// A lead accessor, `IG(M1)`, reads the spectrum of that lead, `@M1[ig]`.
fn complex_accessor(authored: &str) -> Option<(SignalKind, String)> {
    let (operator, argument) = split_accessor(authored)?;
    match operator.as_str() {
        "V" | "VR" | "VI" | "VM" | "VP" | "VDB" => Some((SignalKind::Voltage, argument)),
        "I" | "IR" | "II" | "IM" | "IP" | "IDB" => Some((SignalKind::Current, argument)),
        lead if is_lead_accessor(lead) => Some((
            SignalKind::Current,
            format!("@{argument}[{}]", lead.to_ascii_lowercase()),
        )),
        "DB" | "MAG" | "PH" | "PHASE" | "CPH" | "REAL" | "RE" | "IMAG" => {
            let (inner, argument) = split_accessor(&argument)?;
            match inner.as_str() {
                "V" => Some((SignalKind::Voltage, argument)),
                "I" => Some((SignalKind::Current, argument)),
                _ => None,
            }
        }
        _ => None,
    }
}

/// A Xyce lead current: the drain, gate, source, bulk, collector, base,
/// emitter or substrate current of a device, `ID(M1)` ... `IE(Q1)`, each the
/// current into that terminal.
pub(super) fn is_lead_accessor(operator: &str) -> bool {
    matches!(
        operator.to_ascii_uppercase().as_str(),
        "ID" | "IG" | "IS" | "IB" | "IC" | "IE"
    )
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
///
/// `V(x)` and `I(x)` print the complex column. A rendering accessor — `VR`,
/// `VI`, `VM`, `VP`, `VDB` and their `I` forms, or ngspice's `db()`, `mag()`,
/// `ph()`, `cph()`, `real()` and `imag()` of one probe — prints one real
/// column under its own name, as Xyce and ngspice both do.
fn resolve_complex_operand(
    source: &ProjectionSource<'_>,
    authored: &str,
    kind: &OutputOperandKind,
    phase_unit: PhaseUnit,
) -> Result<ProjectedSignal, SimulationError> {
    let column = complex_operand_column(source, authored, kind)?;
    // A distortion table derives its own magnitude, phase and ratio columns
    // from each phasor, so it keeps the phasor.
    if source.kind == AnalysisResultKind::Distortion {
        return Ok(column);
    }
    match complex_rendering(authored) {
        Some(rendering) => render_complex_column(column, authored, rendering, phase_unit),
        None => Ok(column),
    }
}

/// How a rendering accessor turns a complex column into a real one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ComplexRendering {
    Real,
    Imaginary,
    Magnitude,
    Phase,
    ContinuousPhase,
    Decibels,
}

/// The rendering an authored accessor asks for; `None` for a bare `V(x)` or
/// `I(x)`, which keeps the complex column.
fn complex_rendering(authored: &str) -> Option<ComplexRendering> {
    complex_accessor(authored)?;
    let (operator, _) = split_accessor(authored)?;
    match operator.as_str() {
        "VR" | "IR" | "REAL" | "RE" => Some(ComplexRendering::Real),
        "VI" | "II" | "IMAG" => Some(ComplexRendering::Imaginary),
        "VM" | "IM" | "MAG" => Some(ComplexRendering::Magnitude),
        "VP" | "IP" | "PH" | "PHASE" => Some(ComplexRendering::Phase),
        "CPH" => Some(ComplexRendering::ContinuousPhase),
        "VDB" | "IDB" | "DB" => Some(ComplexRendering::Decibels),
        _ => None,
    }
}

/// Render one complex column as the real column an accessor prints.
///
/// The continuous phase unwraps each sample onto the branch nearest the one
/// before it, as ngspice's `cph()` does (cmath1.c `cx_cph`).
fn render_complex_column(
    column: ProjectedSignal,
    authored: &str,
    rendering: ComplexRendering,
    phase_unit: PhaseUnit,
) -> Result<ProjectedSignal, SimulationError> {
    let Some((real, imag)) = column.complex() else {
        return Ok(column);
    };
    let to_unit = |radians: Value| phase_unit.express(radians);
    let samples = real.iter().zip(imag);
    let values = match rendering {
        ComplexRendering::Real => real.to_vec(),
        ComplexRendering::Imaginary => imag.to_vec(),
        ComplexRendering::Magnitude => samples.map(|(re, im)| re.hypot(*im)).collect(),
        ComplexRendering::Decibels => samples
            .map(|(re, im)| 20.0 * re.hypot(*im).log10())
            .collect(),
        ComplexRendering::Phase => samples.map(|(re, im)| to_unit(im.atan2(*re))).collect(),
        ComplexRendering::ContinuousPhase => {
            let mut last = None::<Value>;
            samples
                .map(|(re, im)| {
                    let phase = im.atan2(*re);
                    let unwrapped = last.map_or(phase, |last| {
                        phase
                            - std::f64::consts::TAU
                                * ((phase - last) / std::f64::consts::TAU + 0.5).floor()
                    });
                    last = Some(unwrapped);
                    to_unit(unwrapped)
                })
                .collect()
        }
    };
    // Real and imaginary parts and the magnitude keep the probe's unit; a
    // phase or a decibel figure is a bare number.
    let kind = match rendering {
        ComplexRendering::Real | ComplexRendering::Imaginary | ComplexRendering::Magnitude => {
            column.descriptor.kind()
        }
        _ => SignalKind::Scalar,
    };
    let registry = complex_accessor(authored).map_or_else(String::new, |(_, argument)| argument);
    let name = authored.trim();
    let descriptor =
        signal_descriptor(name, &registry, kind, SignalValueType::Real).map_err(|error| {
            SimulationError::Netlist(format!("output '{name}' cannot be described: {error}"))
        })?;
    Ok(ProjectedSignal {
        descriptor,
        values: ProjectionValues::Real(Cow::Owned(values)),
        validity: column.validity,
    })
}

/// The sweep frequency an AC print operand names, as a real column beside
/// the phasors. Each dialect reads its own simulator's names: Xyce prints
/// `FREQ` and `HERTZ` (and refuses `FREQUENCY`), ngspice prints
/// `frequency` (and cannot parse `freq`), and the default dialect reads
/// `FREQ`, since neither QSPICE nor LTspice prints a table a name could
/// select. Xyce writes the column under the authored name after its own
/// `FREQ` column.
fn frequency_axis_column(
    source: &ProjectionSource<'_>,
    authored: &str,
    names: &[&str],
) -> Option<Result<ProjectedSignal, SimulationError>> {
    let name = authored.trim();
    if source.kind != AnalysisResultKind::Ac
        || !names.iter().any(|axis| name.eq_ignore_ascii_case(axis))
    {
        return None;
    }
    let length = source.axis.len();
    Some(
        signal_descriptor(name, name, SignalKind::Scalar, SignalValueType::Real)
            .map_err(|error| {
                SimulationError::Netlist(format!("output '{name}' cannot be described: {error}"))
            })
            .map(|descriptor| ProjectedSignal {
                descriptor,
                values: ProjectionValues::Real(Cow::Owned(source.axis.to_vec())),
                validity: vec![true; length],
            }),
    )
}

/// The complex column one authored operand selects, before any rendering.
fn complex_operand_column(
    source: &ProjectionSource<'_>,
    authored: &str,
    kind: &OutputOperandKind,
) -> Result<ProjectedSignal, SimulationError> {
    if let Some(column) = complex_accessor_column(source, authored) {
        return Ok(column);
    }
    // The parser keeps a rendering of a difference, `VM(a,b)`, as an
    // expression; it reads the same difference `V(a,b)` does.
    let rendered_difference = complex_rendering(authored)
        .and(complex_accessor(authored))
        .and_then(|(kind, argument)| {
            let (positive, negative) = argument.split_once(',')?;
            (kind == SignalKind::Voltage).then(|| {
                SaveSignal::VoltageDiff(positive.trim().to_owned(), negative.trim().to_owned())
            })
        });
    let signal = match (kind, &rendered_difference) {
        (_, Some(difference)) => difference,
        (OutputOperandKind::Probe(signal), None) => signal,
        (_, None) => {
            return Err(SimulationError::Netlist(format!(
                "output expression '{authored}' is not evaluable over a complex {} result",
                source.instance
            )));
        }
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
    let lead = match signal {
        SaveSignal::DeviceParam { device, param } => Some(format!("{device}:{param}")),
        SaveSignal::Raw(name) if name.contains(':') && !name.trim_start().starts_with('@') => {
            Some(name.trim().to_owned())
        }
        _ => None,
    };
    if let Some(lead) = lead
        && let Some(column) = device_lead_branch(source, &lead)
    {
        let descriptor = signal_descriptor(
            authored,
            authored,
            SignalKind::Current,
            SignalValueType::Complex,
        )
        .map_err(|error| {
            SimulationError::Netlist(format!(
                "terminal current '{authored}' cannot be described: {error}"
            ))
        })?;
        let ProjectionSourceSignal {
            values, validity, ..
        } = column;
        let length = values.len();
        return Ok(ProjectedSignal {
            descriptor,
            values: values.into_owned(),
            validity: validity.map_or_else(|| vec![true; length], Cow::into_owned),
        });
    }
    Err(source.unavailable(authored))
}

/// The branch a terminal current `@device[param]` (or `device:param`) is
/// solved in.
///
/// A metered Verilog-A terminal's current is the branch current of a meter
/// named `device:param`; a complex result carries it under that name, and
/// the device-parameter spelling selects it.
fn device_lead_branch<'a>(
    source: &ProjectionSource<'a>,
    lead: &str,
) -> Option<ProjectionSourceSignal<'a>> {
    let branch = crate::netlist::canonical_symbol(lead);
    source
        .signals
        .iter()
        .find(|candidate| {
            candidate.descriptor.kind() == SignalKind::Current
                && candidate.values.value_type() == SignalValueType::Complex
                && crate::netlist::canonical_symbol(registry_name(
                    candidate.descriptor.display_name(),
                )) == branch
        })
        .cloned()
}

/// The materialized complex column one authored accessor addresses.
fn complex_accessor_column(
    source: &ProjectionSource<'_>,
    authored: &str,
) -> Option<ProjectedSignal> {
    let (expected, argument) = complex_accessor(authored)?;
    let column = source
        .signals
        .iter()
        .find(|candidate| {
            candidate.descriptor.kind() == expected
                && candidate.values.value_type() == SignalValueType::Complex
                && registry_name(candidate.descriptor.display_name())
                    .eq_ignore_ascii_case(&argument)
        })
        .cloned()
        .map(ProjectionSourceSignal::into_projected)?;
    // A lead prints under its accessor, `IG(M1)`, as Xyce prints it, not
    // under the spectrum's `@M1[ig]`.
    let (operator, _) = split_accessor(authored)?;
    if !is_lead_accessor(&operator) {
        return Some(column);
    }
    let display = format!(
        "{operator}({})",
        column
            .descriptor
            .display_name()
            .strip_prefix('@')?
            .split_once('[')?
            .0
    );
    let descriptor = signal_descriptor(
        &display,
        &display,
        SignalKind::Current,
        SignalValueType::Complex,
    )
    .ok()?;
    Some(ProjectedSignal {
        descriptor,
        values: column.values,
        validity: column.validity,
    })
}

/// The complex small-signal node voltages and branch currents of a noise
/// sweep, as the `V(node)` and `I(branch)` columns an AC result carries.
fn noise_inventory(
    sweep: &[crate::analysis::NoiseResult],
) -> Result<Vec<ProjectionSourceSignal<'static>>, SimulationError> {
    type Phasor = fn(&crate::analysis::NoiseResult, usize) -> Option<(Value, Value)>;
    let Some(first) = sweep.first() else {
        return Ok(Vec::new());
    };
    let families: [(SignalKind, char, &[String], Phasor); 2] = [
        (
            SignalKind::Voltage,
            'V',
            &first.node_names,
            |point, index| point.voltages.get(index).map(|value| (value.re, value.im)),
        ),
        (
            SignalKind::Current,
            'I',
            &first.branch_names,
            |point, index| point.currents.get(index).map(|value| (value.re, value.im)),
        ),
    ];
    let mut signals = Vec::with_capacity(first.voltages.len() + first.currents.len());
    for (kind, prefix, names, phasor) in families {
        for (index, name) in names.iter().enumerate() {
            let registry = if name.is_empty() {
                (index + 1).to_string()
            } else {
                name.clone()
            };
            let display = format!("{prefix}({registry})");
            let (real, imag) = sweep
                .iter()
                .map(|point| phasor(point, index))
                .collect::<Option<Vec<_>>>()
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "noise result lost {display} at a frequency point"
                    ))
                })?
                .into_iter()
                .unzip();
            let signal = ProjectionSourceSignal::new(
                &display,
                &registry,
                kind,
                ProjectionValues::Complex {
                    real: Cow::Owned(real),
                    imag: Cow::Owned(imag),
                },
            )
            .map_err(|error| {
                SimulationError::Circuit(format!(
                    "noise result signal {display} cannot be described: {error}"
                ))
            })?;
            signals.push(signal);
        }
    }
    Ok(signals)
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
    fn expression_only_print_and_plot_cards_select_their_authored_columns() {
        for directive in ["PRINT", "PLOT"] {
            let netlist = parse(&format!(
                "expression selection\nV1 out 0 2\nR1 out 0 1k\n.OP\n.{directive} OP {{-V(out)*I(V1)}}\n.END\n"
            ));
            let projection = SignalProjection::from_netlist(&netlist).unwrap();
            assert!(projection.authored());
            let source = ProjectionSource::new(AnalysisResultKind::OperatingPoint, "DC OP")
                .with_axis(vec![0.0])
                .with_signals(vec![
                    real_signal("V(out)", "out", SignalKind::Voltage, vec![2.0]),
                    real_signal("I(V1)", "v1", SignalKind::Current, vec![-0.002]),
                ]);
            let projected = projection
                .project(&netlist.params, &source, &crate::abort_signal::NoAbort)
                .unwrap();
            assert_eq!(projected.signals().len(), 1);
            assert_eq!(projected.signals()[0].real(), Some(&[0.004][..]));
        }
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
    fn a_control_save_passes_over_a_name_no_vector_answers_to() {
        let mut netlist = parse(
            "missing save\nV1 out 0 1\nR1 out 0 1k\n.TRAN 1u 10u\n.SAVE V(out) V(gone)\n.END\n",
        );
        netlist.saves.ignores_unknown_names = true;
        let projection = SignalProjection::from_netlist(&netlist).expect("projection builds");
        let source = ProjectionSource::new(AnalysisResultKind::Transient, "TRAN")
            .with_axis(vec![0.0, 1.0])
            .with_signals(vec![real_signal(
                "V(out)",
                "out",
                SignalKind::Voltage,
                vec![1.0, 1.0],
            )]);
        let projected = projection
            .project(&netlist.params, &source, &crate::abort_signal::NoAbort)
            .expect("ngspice ignores a saved name no vector answers to");
        assert_eq!(names(&projected), ["V(out)"]);
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

    /// The AC result the rendering tests project: `V(a)` = 3 + 4j, then -1,
    /// and the branch current `I(V1)` = 0 + 2j, then 1 - 1j.
    fn rendering_source() -> ProjectionSource<'static> {
        ProjectionSource::new(AnalysisResultKind::Ac, "AC")
            .with_axis(vec![1.0, 10.0])
            .with_signals(vec![
                complex_signal(
                    "V(a)",
                    "a",
                    SignalKind::Voltage,
                    vec![3.0, -1.0],
                    vec![4.0, 0.0],
                ),
                complex_signal(
                    "I(V1)",
                    "V1",
                    SignalKind::Current,
                    vec![0.0, 1.0],
                    vec![2.0, -1.0],
                ),
            ])
    }

    fn rendered(projected: &ProjectedSignals, index: usize) -> Vec<Value> {
        projected.signals()[index]
            .real()
            .expect("a rendering accessor prints one real column")
            .to_vec()
    }

    /// `.PRINT AC VR(a) VI(a) VM(a) VP(a) VDB(a) IR(V1) II(V1) IM(V1)` prints
    /// one real column each under its own name, and `V(a)`/`I(V1)` keep the
    /// complex pair, as Xyce prints them (phase in degrees).
    #[test]
    fn a_rendering_accessor_prints_one_real_column_and_a_bare_probe_stays_complex() {
        let netlist = parse(
            "render\nV1 a 0 AC 1\nR1 a 0 1k\n.AC DEC 1 1 10\n\
             .PRINT AC VR(a) VI(a) VM(a) VP(a) VDB(a) IR(V1) II(V1) IM(V1) IP(V1) V(a) I(V1)\n\
             .END\n",
        );
        let projection = SignalProjection::from_netlist(&netlist).expect("projection builds");
        let projected = projection
            .project(
                &netlist.params,
                &rendering_source(),
                &crate::abort_signal::NoAbort,
            )
            .expect("every accessor resolves");
        assert_eq!(
            names(&projected),
            [
                "VR(a)", "VI(a)", "VM(a)", "VP(a)", "VDB(a)", "IR(V1)", "II(V1)", "IM(V1)",
                "IP(V1)", "V(a)", "I(V1)"
            ]
        );
        assert_eq!(rendered(&projected, 0), [3.0, -1.0]);
        assert_eq!(rendered(&projected, 1), [4.0, 0.0]);
        assert_eq!(rendered(&projected, 2), [5.0, 1.0]);
        assert_eq!(
            rendered(&projected, 3),
            [4.0_f64.atan2(3.0).to_degrees(), 180.0]
        );
        assert_eq!(rendered(&projected, 4), [20.0 * 5.0_f64.log10(), 0.0]);
        assert_eq!(rendered(&projected, 5), [0.0, 1.0]);
        assert_eq!(rendered(&projected, 6), [2.0, -1.0]);
        assert_eq!(rendered(&projected, 7), [2.0, 2.0_f64.sqrt()]);
        assert_eq!(rendered(&projected, 8), [90.0, -45.0]);
        assert_eq!(
            projected.signals()[2].descriptor().kind(),
            SignalKind::Voltage
        );
        assert_eq!(
            projected.signals()[7].descriptor().kind(),
            SignalKind::Current
        );
        assert_eq!(
            projected.signals()[9].complex(),
            Some((&[3.0, -1.0][..], &[4.0, 0.0][..]))
        );
        assert!(projected.signals()[10].complex().is_some());
    }

    /// `.PRINT AC FREQ VM(a)` prints the sweep frequency as a real column
    /// before the magnitude, by the names each dialect's simulator prints it
    /// under: `FREQ` and `HERTZ` in Xyce 7.10, `frequency` in ngspice 46,
    /// `FREQ` in the default dialect. A name the dialect's simulator does not
    /// know stays the unavailable signal it was; before, every one was.
    #[test]
    fn an_ac_print_reads_the_sweep_frequency_by_the_dialect_s_names() {
        for (dialect, read, refused) in [
            (crate::SpiceDialect::Xyce, "FREQ HERTZ", "frequency"),
            (crate::SpiceDialect::Ngspice, "frequency", "freq"),
            (crate::SpiceDialect::BestAvailable, "freq", "hertz"),
        ] {
            let parse = |operands: &str| {
                Netlist::parse_with_options(
                    &format!(
                        "axis\nV1 a 0 AC 1\nR1 a 0 1k\n.AC DEC 1 1 10\n\
                         .PRINT AC {operands} VM(a)\n.END\n"
                    ),
                    crate::netlist::NetlistParseOptions {
                        spice_dialect: Some(dialect),
                        ..Default::default()
                    },
                )
                .expect("test netlist parses")
            };
            let netlist = parse(read);
            let projected = SignalProjection::from_netlist(&netlist)
                .expect("projection builds")
                .project(
                    &netlist.params,
                    &rendering_source(),
                    &crate::abort_signal::NoAbort,
                )
                .unwrap_or_else(|error| panic!("{dialect:?}: {error}"));
            let names = names(&projected);
            let axis = read.split_whitespace().count();
            assert_eq!(
                names[..axis],
                read.split_whitespace().collect::<Vec<_>>()[..]
            );
            assert_eq!(names[axis], "VM(a)", "{dialect:?}");
            for index in 0..axis {
                assert_eq!(rendered(&projected, index), [1.0, 10.0], "{dialect:?}");
                assert_eq!(
                    projected.signals()[index].descriptor().kind(),
                    SignalKind::Scalar
                );
            }
            assert_eq!(rendered(&projected, axis), [5.0, 1.0]);

            let netlist = parse(refused);
            let error = SignalProjection::from_netlist(&netlist)
                .expect("projection builds")
                .project(
                    &netlist.params,
                    &rendering_source(),
                    &crate::abort_signal::NoAbort,
                )
                .expect_err("a name the dialect's simulator does not know");
            assert!(
                matches!(error, SimulationError::RequestedSignalUnavailable(_)),
                "{dialect:?}: {error}"
            );
        }
    }

    /// Xyce's `.OPTIONS OUTPUT PHASE_OUTPUT_RADIANS` prints `VP()` in radians.
    #[test]
    fn the_xyce_phase_option_prints_phase_in_radians() {
        let netlist = parse(
            "radians\nV1 a 0 AC 1\nR1 a 0 1k\n.AC DEC 1 1 10\n\
             .OPTIONS OUTPUT PHASE_OUTPUT_RADIANS=true\n.PRINT AC VP(a) IP(V1)\n.END\n",
        );
        let projection = SignalProjection::from_netlist(&netlist).expect("projection builds");
        let projected = projection
            .project(
                &netlist.params,
                &rendering_source(),
                &crate::abort_signal::NoAbort,
            )
            .expect("both accessors resolve");
        assert_eq!(
            rendered(&projected, 0),
            [4.0_f64.atan2(3.0), std::f64::consts::PI]
        );
        assert_eq!(
            rendered(&projected, 1),
            [std::f64::consts::FRAC_PI_2, -std::f64::consts::FRAC_PI_4]
        );
    }

    /// A `.PRINT AC VR(a)` card asks nothing of the `.OP` run beside it
    /// (Xyce BUG_1238 bug1238-ac-op): the raw save the parser records for an
    /// accessor operand belongs to its card.
    #[test]
    fn an_ac_accessor_operand_asks_no_column_of_the_operating_point() {
        let netlist = parse(
            "ac op
V1 a 0 DC 1 AC 1
R1 a 0 1k
.OP
.AC DEC 1 1 10
             .PRINT AC VR(a) IR(V1)
.END
",
        );
        let projection = SignalProjection::from_netlist(&netlist).expect("projection builds");
        let source = ProjectionSource::new(AnalysisResultKind::OperatingPoint, "DC OP")
            .with_axis(vec![0.0])
            .with_signals(vec![real_signal(
                "V(a)",
                "a",
                SignalKind::Voltage,
                vec![1.0],
            )]);
        let projected = projection
            .project(&netlist.params, &source, &crate::abort_signal::NoAbort)
            .expect("the operating point projects");
        assert!(
            !names(&projected).iter().any(|name| name.starts_with("VR")),
            "{:?}",
            names(&projected)
        );
        let projected = projection
            .project(
                &netlist.params,
                &rendering_source(),
                &crate::abort_signal::NoAbort,
            )
            .expect("the AC result projects");
        assert_eq!(names(&projected), ["VR(a)", "IR(V1)"]);
    }

    /// A differential `VM(a,0)` renders the synthesized difference.
    #[test]
    fn a_differential_rendering_accessor_prints_one_real_column() {
        let netlist =
            parse("render diff\nV1 a 0 AC 1\nR1 a 0 1k\n.AC DEC 1 1 10\n.PRINT AC VM(a,0)\n.END\n");
        let projection = SignalProjection::from_netlist(&netlist).expect("projection builds");
        let projected = projection
            .project(
                &netlist.params,
                &rendering_source(),
                &crate::abort_signal::NoAbort,
            )
            .expect("the differential resolves");
        assert_eq!(names(&projected), ["VM(a,0)"]);
        assert_eq!(rendered(&projected, 0), [5.0, 1.0]);
    }

    /// ngspice prints `vp()` and `ph()` in radians (cmath1.c `cx_ph`), and
    /// `cph()` keeps the phase continuous from one point to the next.
    #[test]
    fn the_ngspice_dialect_prints_phase_in_radians() {
        let netlist = Netlist::parse_with_options(
            "radians\nV1 a 0 AC 1\nR1 a 0 1k\n.AC DEC 1 1 10\n\
             .PRINT AC vp(a) ph(v(a)) cph(i(v1))\n.END\n",
            crate::netlist::NetlistParseOptions {
                spice_dialect: Some(crate::SpiceDialect::Ngspice),
                ..Default::default()
            },
        )
        .expect("ngspice test netlist parses");
        let projection = SignalProjection::from_netlist(&netlist).expect("projection builds");
        let source = ProjectionSource::new(AnalysisResultKind::Ac, "AC")
            .with_axis(vec![1.0, 10.0])
            .with_signals(vec![
                complex_signal(
                    "V(a)",
                    "a",
                    SignalKind::Voltage,
                    vec![3.0, -1.0],
                    vec![4.0, 0.0],
                ),
                complex_signal(
                    "I(V1)",
                    "V1",
                    SignalKind::Current,
                    vec![-1.0, -1.0],
                    vec![1.0e-3, -1.0e-3],
                ),
            ]);
        let projected = projection
            .project(&netlist.params, &source, &crate::abort_signal::NoAbort)
            .expect("every accessor resolves");
        assert_eq!(
            rendered(&projected, 0),
            [4.0_f64.atan2(3.0), std::f64::consts::PI]
        );
        assert_eq!(rendered(&projected, 1), rendered(&projected, 0));
        let continuous = rendered(&projected, 2);
        assert!((continuous[0] - 1.0e-3_f64.atan2(-1.0)).abs() < 1.0e-12);
        assert!(
            (continuous[1] - (std::f64::consts::TAU + (-1.0e-3_f64).atan2(-1.0))).abs() < 1.0e-12,
            "cph unwraps across the -pi cut: {continuous:?}"
        );
    }

    /// ngspice's `db()`/`ph()` of one probe (vbic/CEamp prints
    /// `db(i(vmeas))`) select the probe's complex column as `idb(vmeas)` does,
    /// and print it as one real column.
    #[test]
    fn an_ngspice_function_of_one_probe_selects_its_complex_column() {
        let netlist = parse(
            "functions\nI1 0 a AC 1\nVM a b 0\nR1 b 0 1k\n.AC DEC 1 1 10\n\
             .PRINT AC db(i(vm)) ph(v(a))\n.END\n",
        );
        let projection = SignalProjection::from_netlist(&netlist).expect("projection builds");
        let source = ProjectionSource::new(AnalysisResultKind::Ac, "AC")
            .with_axis(vec![1.0, 10.0])
            .with_signals(vec![
                complex_signal(
                    "V(a)",
                    "a",
                    SignalKind::Voltage,
                    vec![1e3, 1e3],
                    vec![0.0, 1.0],
                ),
                complex_signal(
                    "I(VM)",
                    "VM",
                    SignalKind::Current,
                    vec![1.0, 1.0],
                    vec![0.0, 1e-3],
                ),
            ]);
        let projected = projection
            .project(&netlist.params, &source, &crate::abort_signal::NoAbort)
            .expect("both functions resolve");
        assert_eq!(names(&projected), ["db(i(vm))", "ph(v(a))"]);
        assert_eq!(
            rendered(&projected, 0),
            [0.0, 20.0 * 1.0e-3_f64.hypot(1.0).log10()]
        );
        assert_eq!(
            rendered(&projected, 1),
            [0.0, 1.0_f64.atan2(1e3).to_degrees()]
        );
    }

    /// A terminal current `@x1[ia]` is solved in the branch of the meter on
    /// that terminal, `x1:ia`, and a complex result selects it under the
    /// spelling the deck wrote, as a current.
    #[test]
    fn a_terminal_current_selects_its_meters_complex_branch() {
        let netlist = parse(
            "terminal current\nV1 a 0 AC 1\nX1 a 0 dev\n.AC DEC 1 1 10\n.SAVE @x1[ia]\n.END\n",
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
                    "I(X1:IA)",
                    "X1:IA",
                    SignalKind::Current,
                    vec![1e-3, 1e-3],
                    vec![2e-6, 2e-5],
                ),
            ]);
        let projected = projection
            .project(&netlist.params, &source, &crate::abort_signal::NoAbort)
            .expect("the terminal current resolves");
        assert_eq!(names(&projected), ["@x1[ia]"]);
        assert_eq!(
            projected.signals()[0].descriptor().kind(),
            SignalKind::Current
        );
        let (real, imag) = projected.signals()[0].complex().expect("complex column");
        assert_eq!(real, [1e-3, 1e-3]);
        assert_eq!(imag, [2e-6, 2e-5]);
    }

    /// A port name longer than one letter gives a longer label, `@x1[iout]`,
    /// and its `x1:iinp` spelling selects the meter's branch the same way.
    #[test]
    fn a_multi_letter_terminal_current_selects_its_meters_complex_branch() {
        let netlist = parse(
            "terminal current\nV1 a 0 AC 1\nX1 a 0 dev\n.AC DEC 1 1 10\n\
             .SAVE @x1[iout] x1:iinp\n.END\n",
        );
        let projection = SignalProjection::from_netlist(&netlist).expect("projection builds");
        let source = ProjectionSource::new(AnalysisResultKind::Ac, "AC")
            .with_axis(vec![1.0])
            .with_signals(vec![
                complex_signal(
                    "I(X1:IINP)",
                    "X1:IINP",
                    SignalKind::Current,
                    vec![2.0],
                    vec![0.5],
                ),
                complex_signal(
                    "I(X1:IOUT)",
                    "X1:IOUT",
                    SignalKind::Current,
                    vec![-2.0],
                    vec![-0.5],
                ),
            ]);
        let projected = projection
            .project(&netlist.params, &source, &crate::abort_signal::NoAbort)
            .expect("both terminal currents resolve");
        assert_eq!(names(&projected), ["@x1[iout]", "x1:iinp"]);
        for (signal, expected) in projected.signals().iter().zip([-2.0, 2.0]) {
            assert_eq!(signal.descriptor().kind(), SignalKind::Current);
            assert_eq!(signal.descriptor().unit(), &SignalUnit::Ampere);
            assert_eq!(signal.complex().expect("complex column").0, [expected]);
        }
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
