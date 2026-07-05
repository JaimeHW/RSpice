//! Xyce regression corpus runner.
//!
//! The upstream Xyce suite is vendored as a runtime corpus. RSpice keeps the
//! netlists, reference output data, and licensing/provenance files, but omits
//! upstream platform-specific harness scripts. Regression execution is
//! Rust-native: every retained `.cir` deck is discovered and reported, and only
//! decks with a supported, checked-in static `.prn` oracle are numerically
//! executed.

use crate::abort_signal::AbortSignal;
use crate::analysis::AcResult;
use crate::analysis::ac::ac_sweep_frequencies;
use crate::engine::{
    ConvergenceConfig, DcSweepPointResult, SimulationConfig, SimulationError, SpiceDialect,
    TransientResult, extract_ac_value, extract_dc_value,
};
use crate::expr::{Expr, parse_expression_strict};
use crate::netlist::expr::ComplexValue as ExprComplexValue;
use crate::netlist::expr::prepare_behavioral_expression;
use crate::netlist::{
    AnalysisCommand, DcSecondSweep, ElementKind, ExpressionDialect, Netlist, NetlistParseOptions,
    StatisticalParamMode, StepCommand, StepSweep, StepTarget, SubcircuitDef,
    XYCE_DEFAULT_ZERO_RESISTANCE_TOL,
};
use crate::{Complex64, Engine, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};

const EXPECTED_UNSUPPORTED_MARKER: &str = "EXPECTED_UNSUPPORTED:";
const HARNESS_MANIFEST_FILE: &str = "RSPICE-HARNESS-MANIFEST.tsv";
const REQUIRES_UPSTREAM_WRAPPER_CONTRACT: &str = "requires_upstream_wrapper";
const MAX_NATIVE_TRAN_ORACLE_STEPS: f64 = 250_000.0;
const MAX_NATIVE_TRAN_ELEMENT_STEPS: f64 = 250_000_000.0;
const MAX_NATIVE_TRAN_NODE_SOLVE_STEPS: f64 = 2_500_000_000.0;
const TRAN_ORACLE_STEPS_PER_SOURCE_PERIOD: f64 = 64.0;
const TRAN_ORACLE_STEPS_PER_SOURCE_TRANSITION: f64 = 200.0;
const XYCE_DEFAULT_PRN_FRACTION_DIGITS: f64 = 8.0;
const PRN_TIME_NEIGHBOR_HALF_ULPS: f64 = 4.0;
const XYCE_PWL_REPEAT_VALUE_ERROR: &str =
    "PWL source repeat value (R) must be >= 0 and < last value in time-voltage list";

/// Configuration for the Xyce corpus runner.
#[derive(Debug, Clone)]
pub struct XyceRunnerConfig {
    /// Relative tolerance for value comparison.
    pub relative_tolerance: f64,
    /// Absolute tolerance for current-like and unitless near-zero values.
    pub absolute_tolerance: f64,
    /// Absolute tolerance for voltage-like near-zero values.
    pub voltage_absolute_tolerance: f64,
    /// Absolute tolerance for derived power near-zero values.
    pub power_absolute_tolerance: f64,
    /// Maximum number of mismatches to retain in one result.
    pub max_mismatches: usize,
    /// Maximum wall-clock time allowed for a numerically executed deck.
    pub max_time_per_test_ms: u128,
    /// Print per-deck execution details.
    pub verbose: bool,
}

impl Default for XyceRunnerConfig {
    fn default() -> Self {
        Self {
            relative_tolerance: 0.02,
            absolute_tolerance: 1.0e-12,
            voltage_absolute_tolerance: crate::constants::VNTOL,
            power_absolute_tolerance: 1.0e-9,
            max_mismatches: 20,
            max_time_per_test_ms: 180_000,
            verbose: false,
        }
    }
}

/// Which vendored corpus area a `.cir` file belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XyceDeckSection {
    /// Simulator regression decks under `tests/xyce/Netlists`.
    Netlists,
    /// Any other vendored `.cir` file.
    Other,
}

/// A discovered vendored Xyce deck.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XyceDeck {
    /// Absolute path to the deck.
    pub path: PathBuf,
    /// Path relative to `tests/xyce`, normalized with `/` separators.
    pub relative_path: String,
    /// Corpus section classification.
    pub section: XyceDeckSection,
}

/// Numeric mismatch for a Xyce reference comparison.
#[derive(Debug, Clone)]
pub struct XyceValueMismatch {
    /// Row index in the Xyce `.prn` table.
    pub row: usize,
    /// Output column/probe name.
    pub probe: String,
    /// Expected value from Xyce output data.
    pub expected: f64,
    /// RSpice value.
    pub actual: f64,
    /// Relative error after the absolute tolerance floor.
    pub relative_error: f64,
}

/// Result for one discovered Xyce deck.
#[derive(Debug, Clone)]
pub struct XyceTestResult {
    /// Deck filename stem.
    pub name: String,
    /// Path relative to `tests/xyce`.
    pub relative_path: String,
    /// Whether the deck produced an accepted result for the current harness.
    pub passed: bool,
    /// Whether this is a named, expected unsupported result rather than a
    /// numeric comparison.
    pub expected_unsupported: bool,
    /// Error or expected-unsupported reason.
    pub error: Option<String>,
    /// Numeric mismatches for executed decks.
    pub mismatches: Vec<XyceValueMismatch>,
    /// Execution/classification duration.
    pub duration_ms: u128,
    /// Contract label applied by the runner.
    pub contract: String,
}

/// Aggregate Xyce corpus statistics.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct XyceStatistics {
    pub total: usize,
    pub executed: usize,
    pub passed: usize,
    pub failed: usize,
    pub expected_unsupported: usize,
    pub total_time_ms: u128,
}

impl XyceStatistics {
    pub fn executed_pass_rate(&self) -> f64 {
        if self.executed == 0 {
            0.0
        } else {
            self.passed as f64 / self.executed as f64 * 100.0
        }
    }

    pub fn executed_coverage_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.executed as f64 / self.total as f64 * 100.0
        }
    }
}

/// Rust-native runner for the vendored Xyce corpus.
pub struct XyceTestRunner {
    root: PathBuf,
    config: XyceRunnerConfig,
    upstream_wrapper_decks: BTreeSet<String>,
}

#[derive(Debug, Clone)]
struct XyceExecutionPlan {
    deck_path: PathBuf,
    execution_dir: Option<PathBuf>,
    reference_path: PathBuf,
    source: String,
    expression_dialect: ExpressionDialect,
    print: XycePrintRequest,
    dc: XyceDcSweep,
    dc_data: Option<XyceDcDataSweep>,
    steps: Vec<StepCommand>,
    contract: XyceStaticDcContract,
}

#[derive(Debug, Clone)]
struct XyceStaticDcPlan {
    deck_path: PathBuf,
    execution_dir: Option<PathBuf>,
    source: String,
    expression_dialect: ExpressionDialect,
    print: XycePrintRequest,
    print_format: Option<String>,
    dc: XyceDcSweep,
    dc_data: Option<XyceDcDataSweep>,
    steps: Vec<StepCommand>,
    diagnostics: Vec<crate::netlist::ParseDiagnostic>,
}

#[derive(Debug, Clone)]
struct XyceStaticTranPlan {
    deck_path: PathBuf,
    reference_path: PathBuf,
    source: String,
    print: XycePrintRequest,
    output_override: bool,
    tran: XyceTranAnalysis,
    steps: Vec<StepCommand>,
    contract: XyceStaticTranContract,
    wrapper_tolerance: Option<XyceComparisonTolerance>,
}

#[derive(Debug, Clone)]
struct XyceStaticAcPlan {
    deck_path: PathBuf,
    reference_path: Option<PathBuf>,
    source: String,
    print: Option<XycePrintRequest>,
    primary_ac_file: Option<String>,
    primary_ac_ic_file: Option<String>,
    output_override: bool,
    ac: XyceAcAnalysis,
    frequency_bound: bool,
    steps: Vec<StepCommand>,
    contract: XyceStaticAcContract,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceStaticTranContract {
    PlainStatic,
    PlainCsv,
    PlainCsd,
    WrapperStatic,
    WrapperCsv,
    WrapperCsd,
    WrapperStaticExpectedError,
}

impl XyceStaticTranContract {
    fn result_contract(self, has_step: bool) -> &'static str {
        match (self, has_step) {
            (Self::PlainStatic, false) => "static_prn_tran",
            (Self::PlainStatic, true) => "static_prn_step_tran",
            (Self::PlainCsv, false) => "static_csv_tran",
            (Self::PlainCsv, true) => "static_csv_step_tran",
            (Self::PlainCsd, false) => "static_csd_tran",
            (Self::PlainCsd, true) => "static_csd_step_tran",
            (Self::WrapperStatic, false) => "wrapper_static_prn_tran",
            (Self::WrapperStatic, true) => "wrapper_static_prn_step_tran",
            (Self::WrapperCsv, false) => "wrapper_static_csv_tran",
            (Self::WrapperCsv, true) => "wrapper_static_csv_step_tran",
            (Self::WrapperCsd, false) => "wrapper_csd_tran",
            (Self::WrapperCsd, true) => "wrapper_csd_step_tran",
            (Self::WrapperStaticExpectedError, false) => "wrapper_static_prn_tran_expected_error",
            (Self::WrapperStaticExpectedError, true) => {
                "wrapper_static_prn_step_tran_expected_error"
            }
        }
    }

    fn can_use_reference_stop(self) -> bool {
        matches!(
            self,
            Self::WrapperStatic | Self::WrapperCsv | Self::WrapperCsd
        )
    }

    fn reference_extension(self) -> &'static str {
        match self {
            Self::PlainCsv | Self::WrapperCsv => "csv",
            Self::PlainCsd | Self::WrapperCsd => "csd",
            _ => "prn",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceStaticAcContract {
    PlainStatic,
    PlainCsv,
    PlainCsd,
    PlainIcPrn,
    PlainIcCsv,
    PlainIcCsd,
    WrapperStatic,
    WrapperCsv,
    WrapperCsd,
    WrapperIcPrn,
    WrapperIcCsv,
    WrapperIcCsd,
}

impl XyceStaticAcContract {
    fn result_contract(self, stepped: bool) -> &'static str {
        match (self, stepped) {
            (Self::PlainStatic, false) => "static_fd_prn_ac",
            (Self::PlainStatic, true) => "static_fd_prn_step_ac",
            (Self::PlainCsv, false) => "static_fd_csv_ac",
            (Self::PlainCsv, true) => "static_fd_csv_step_ac",
            (Self::PlainCsd, false) => "static_csd_ac",
            (Self::PlainCsd, true) => "static_csd_step_ac",
            (Self::PlainIcPrn, false) => "static_td_prn_ac_ic",
            (Self::PlainIcPrn, true) => "static_td_prn_step_ac_ic",
            (Self::PlainIcCsv, false) => "static_td_csv_ac_ic",
            (Self::PlainIcCsv, true) => "static_td_csv_step_ac_ic",
            (Self::PlainIcCsd, false) => "static_td_csd_ac_ic",
            (Self::PlainIcCsd, true) => "static_td_csd_step_ac_ic",
            (Self::WrapperStatic, false) => "wrapper_static_fd_prn_ac",
            (Self::WrapperStatic, true) => "wrapper_static_fd_prn_step_ac",
            (Self::WrapperCsv, false) => "wrapper_static_fd_csv_ac",
            (Self::WrapperCsv, true) => "wrapper_static_fd_csv_step_ac",
            (Self::WrapperCsd, false) => "wrapper_csd_ac",
            (Self::WrapperCsd, true) => "wrapper_csd_step_ac",
            (Self::WrapperIcPrn, false) => "wrapper_static_td_prn_ac_ic",
            (Self::WrapperIcPrn, true) => "wrapper_static_td_prn_step_ac_ic",
            (Self::WrapperIcCsv, false) => "wrapper_static_td_csv_ac_ic",
            (Self::WrapperIcCsv, true) => "wrapper_static_td_csv_step_ac_ic",
            (Self::WrapperIcCsd, false) => "wrapper_static_td_csd_ac_ic",
            (Self::WrapperIcCsd, true) => "wrapper_static_td_csd_step_ac_ic",
        }
    }

    fn reference_extension(self) -> &'static str {
        match self {
            Self::PlainStatic | Self::WrapperStatic => "FD.prn",
            Self::PlainCsv | Self::WrapperCsv => "FD.csv",
            Self::PlainCsd | Self::WrapperCsd => "csd",
            Self::PlainIcPrn | Self::WrapperIcPrn => "TD.prn",
            Self::PlainIcCsv | Self::WrapperIcCsv => "TD.csv",
            Self::PlainIcCsd | Self::WrapperIcCsd => "TD.csd",
        }
    }
}

#[derive(Debug, Clone)]
struct XyceBaselineFamilyContract {
    kind: XyceBaselineFamilyKind,
    family: String,
    baseline_path: PathBuf,
    member_paths: Vec<PathBuf>,
    target_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceBaselineFamilyKind {
    Subckt,
    Supernode,
}

impl XyceBaselineFamilyKind {
    fn name(self) -> &'static str {
        match self {
            Self::Subckt => "SUBCKT",
            Self::Supernode => "SUPERNODE",
        }
    }

    fn wrapper_contract(self) -> &'static str {
        match self {
            Self::Subckt => "subckt_family_wrapper",
            Self::Supernode => "supernode_family_wrapper",
        }
    }

    fn baseline_contract(self) -> &'static str {
        match self {
            Self::Subckt => "subckt_family_baseline",
            Self::Supernode => "supernode_family_baseline",
        }
    }

    fn compares_baseline_oracle(self) -> bool {
        matches!(self, Self::Supernode)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceStaticDcContract {
    PlainStatic,
    PlainCsv,
    PlainCsd,
    WrapperDefault,
    WrapperCsv,
    WrapperCsd,
    WrapperFilePrn,
    WrapperGnuplotSplot,
    WrapperHspiceMath,
    WrapperNoOutput,
    WrapperRaw,
    WrapperResistorDefault,
    WrapperTopLevelExecutionDir,
    WrapperVoltageAccessor,
}

impl XyceStaticDcContract {
    fn result_contract(self, stepped: bool) -> &'static str {
        match (self, stepped) {
            (Self::PlainStatic, false) => "static_prn_dc",
            (Self::PlainStatic, true) => "static_prn_step_dc",
            (Self::PlainCsv, false) => "static_csv_dc",
            (Self::PlainCsv, true) => "static_csv_step_dc",
            (Self::PlainCsd, false) => "static_csd_dc",
            (Self::PlainCsd, true) => "static_csd_step_dc",
            (Self::WrapperDefault, false) => "wrapper_static_prn_dc",
            (Self::WrapperDefault, true) => "wrapper_static_prn_step_dc",
            (Self::WrapperCsv, false) => "wrapper_static_csv_dc",
            (Self::WrapperCsv, true) => "wrapper_static_csv_step_dc",
            (Self::WrapperCsd, false) => "wrapper_csd_dc",
            (Self::WrapperCsd, true) => "wrapper_csd_step_dc",
            (Self::WrapperFilePrn, false) => "wrapper_file_prn_dc",
            (Self::WrapperFilePrn, true) => "wrapper_file_prn_step_dc",
            (Self::WrapperGnuplotSplot, false) => "wrapper_gnuplot_splot_prn_dc",
            (Self::WrapperGnuplotSplot, true) => "wrapper_gnuplot_splot_prn_step_dc",
            (Self::WrapperHspiceMath, false) => "wrapper_hspice_math_prn_dc",
            (Self::WrapperHspiceMath, true) => "wrapper_hspice_math_prn_step_dc",
            (Self::WrapperNoOutput, false) => "wrapper_no_output_dc",
            (Self::WrapperNoOutput, true) => "wrapper_no_output_step_dc",
            (Self::WrapperRaw, false) => "wrapper_raw_dc",
            (Self::WrapperRaw, true) => "wrapper_raw_step_dc",
            (Self::WrapperResistorDefault, false) => "wrapper_resistor_default_prn_dc",
            (Self::WrapperResistorDefault, true) => "wrapper_resistor_default_prn_step_dc",
            (Self::WrapperTopLevelExecutionDir, false) => "wrapper_top_level_execution_dir_prn_dc",
            (Self::WrapperTopLevelExecutionDir, true) => {
                "wrapper_top_level_execution_dir_prn_step_dc"
            }
            (Self::WrapperVoltageAccessor, false) => "wrapper_voltage_accessor_prn_dc",
            (Self::WrapperVoltageAccessor, true) => "wrapper_voltage_accessor_prn_step_dc",
        }
    }

    fn compares_step_res_reference(self) -> bool {
        matches!(self, Self::WrapperDefault | Self::WrapperRaw)
    }

    fn reference_extension(self) -> &'static str {
        match self {
            Self::PlainCsv | Self::WrapperCsv => "csv",
            Self::PlainCsd | Self::WrapperCsd => "csd",
            Self::WrapperRaw => "raw",
            _ => "prn",
        }
    }
}

#[derive(Debug, Clone)]
struct XycePrintRequest {
    probes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XycePrintOutputRequest {
    format: Option<String>,
    file: Option<String>,
    probes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceVoltageProbe {
    accessor: XyceVoltageAccessor,
    node_pos: String,
    node_neg: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceAcCurrentProbe {
    accessor: XyceCurrentAccessor,
    element_name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceVoltageAccessor {
    Value,
    Real,
    Imaginary,
    Magnitude,
    Phase,
    Decibels,
}

impl XyceVoltageAccessor {
    fn from_function_name(name: &str) -> Option<Self> {
        match name.to_ascii_lowercase().as_str() {
            "v" => Some(Self::Value),
            "vr" => Some(Self::Real),
            "vi" => Some(Self::Imaginary),
            "vm" => Some(Self::Magnitude),
            "vp" => Some(Self::Phase),
            "vdb" => Some(Self::Decibels),
            _ => None,
        }
    }

    fn uses_voltage_tolerance(self) -> bool {
        !matches!(self, Self::Phase | Self::Decibels)
    }

    fn evaluate_dc(self, real: Value) -> Value {
        match self {
            Self::Value | Self::Real => real,
            Self::Imaginary => 0.0,
            Self::Magnitude => real.abs(),
            Self::Phase => 0.0_f64.atan2(real).to_degrees(),
            Self::Decibels => Self::db(real.abs()),
        }
    }

    fn evaluate_ac_scalar(self, value: Complex64, phase_output_radians: bool) -> Option<Value> {
        match self {
            Self::Value => None,
            Self::Real => Some(value.re),
            Self::Imaginary => Some(value.im),
            Self::Magnitude => Some(value.norm()),
            Self::Phase if phase_output_radians => Some(value.arg()),
            Self::Phase => Some(value.arg().to_degrees()),
            Self::Decibels => Some(Self::db(value.norm())),
        }
    }

    fn db(magnitude: Value) -> Value {
        20.0 * magnitude.max(1.0e-38).log10()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceCurrentAccessor {
    Value,
    Real,
    Imaginary,
    Magnitude,
    Phase,
    Decibels,
}

impl XyceCurrentAccessor {
    fn from_function_name(name: &str) -> Option<Self> {
        match name.to_ascii_lowercase().as_str() {
            "i" => Some(Self::Value),
            "ir" => Some(Self::Real),
            "ii" => Some(Self::Imaginary),
            "im" => Some(Self::Magnitude),
            "ip" => Some(Self::Phase),
            "idb" => Some(Self::Decibels),
            _ => None,
        }
    }

    fn evaluate_ac_scalar(self, value: Complex64, phase_output_radians: bool) -> Option<Value> {
        match self {
            Self::Value => None,
            Self::Real => Some(value.re),
            Self::Imaginary => Some(value.im),
            Self::Magnitude => Some(value.norm()),
            Self::Phase if phase_output_radians => Some(value.arg()),
            Self::Phase => Some(value.arg().to_degrees()),
            Self::Decibels => Some(XyceVoltageAccessor::db(value.norm())),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct XyceComparisonTolerance {
    relative: f64,
    absolute: f64,
    zero: Option<f64>,
}

impl XyceComparisonTolerance {
    fn from_config(config: &XyceRunnerConfig) -> Self {
        Self {
            relative: config.relative_tolerance,
            absolute: config.absolute_tolerance,
            zero: None,
        }
    }

    fn with_relative(mut self, value: f64) -> Self {
        if value.is_finite() && value >= 0.0 {
            self.relative = value;
        }
        self
    }

    fn with_absolute(mut self, value: f64) -> Self {
        if value.is_finite() && value >= 0.0 {
            self.absolute = self.absolute.max(value);
        }
        self
    }

    fn with_zero(mut self, value: f64) -> Self {
        if value.is_finite() && value >= 0.0 {
            self.zero = Some(self.zero.unwrap_or(0.0).max(value));
        }
        self
    }
}

#[derive(Debug, Clone)]
struct XyceDcSweep {
    source: String,
    start: Value,
    stop: Value,
    step: Value,
    mode: crate::netlist::DcSweepMode,
    sweep2: Option<DcSecondSweep>,
}

#[derive(Debug, Clone)]
struct XyceDcDataSweep {
    rows: Vec<XyceDcDataRow>,
}

#[derive(Debug, Clone)]
struct XyceDcDataRow {
    overrides: Vec<XyceDcDataOverride>,
}

#[derive(Debug, Clone)]
enum XyceDcDataOverride {
    Parameter {
        name: String,
        value: Value,
    },
    Device {
        name: String,
        param_name: Option<String>,
        value: Value,
    },
}

#[derive(Debug, Clone, Copy)]
struct XyceTranAnalysis {
    step: Value,
    stop: Value,
    start: Option<Value>,
    max_step: Option<Value>,
    uic: bool,
}

#[derive(Debug, Clone, Copy)]
struct XyceTransientProblemSize {
    element_count: usize,
    node_count: usize,
}

#[derive(Debug, Clone)]
struct XyceAcAnalysis {
    frequencies: Vec<Value>,
    data_points: Option<Vec<XyceAcDataPoint>>,
}

impl XyceAcAnalysis {
    fn frequencies(&self) -> Vec<Value> {
        self.frequencies.clone()
    }

    fn data_points(&self) -> Option<&[XyceAcDataPoint]> {
        self.data_points.as_deref()
    }
}

#[derive(Debug, Clone)]
struct XyceAcDataPoint {
    frequency: Value,
    overrides: Vec<(String, Value)>,
}

impl XyceAcReferenceColumn {
    fn probe_name(&self) -> &str {
        match self {
            Self::Probe { name, .. } => name,
        }
    }
}

#[derive(Debug, Clone)]
struct XyceDcSweepDimension {
    source: String,
    start: Value,
    stop: Value,
    step: Value,
    mode: crate::netlist::DcSweepMode,
}

impl XyceDcSweepDimension {
    fn spec(&self) -> crate::netlist::DcSweepSpec {
        crate::netlist::DcSweepSpec {
            start: self.start,
            stop: self.stop,
            step: self.step,
            mode: self.mode.clone(),
        }
    }

    fn into_second_sweep(self) -> DcSecondSweep {
        DcSecondSweep {
            source: self.source,
            start: self.start,
            stop: self.stop,
            step: self.step,
            mode: self.mode,
        }
    }
}

impl XyceDcSweep {
    fn primary_spec(&self) -> crate::netlist::DcSweepSpec {
        crate::netlist::DcSweepSpec {
            start: self.start,
            stop: self.stop,
            step: self.step,
            mode: self.mode.clone(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct XyceDcSweepPoint {
    primary: Value,
    secondary: Option<Value>,
}

#[derive(Debug, Clone)]
struct XyceDcResultBatch {
    netlist: Netlist,
    results: Vec<DcSweepPointResult>,
}

#[derive(Debug, Clone)]
struct XyceDcDataPointResult {
    netlist: Netlist,
    point: DcSweepPointResult,
}

#[derive(Debug, Clone)]
struct XyceAcResultBatch {
    netlist: Netlist,
    results: Vec<AcResult>,
}

#[derive(Debug, Clone)]
struct XyceAcDataPointResult {
    netlist: Netlist,
    result: AcResult,
}

#[derive(Debug, Clone)]
struct XyceStepRun {
    step_values: Vec<Value>,
    netlist: Netlist,
}

#[derive(Debug, Clone)]
struct XyceStepRunBuilder {
    step_values: Vec<Value>,
    bindings: Vec<XyceStepBinding>,
}

#[derive(Debug, Clone)]
struct XyceStepBinding {
    step: StepCommand,
    value: Value,
}

#[derive(Debug, Clone)]
enum XyceReferenceColumn {
    PrimarySweep { name: String },
    Probe { name: String },
}

#[derive(Debug, Clone)]
enum XyceAcReferenceColumn {
    Probe {
        name: String,
        component: XyceAcProbeComponent,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceAcProbeComponent {
    Scalar,
    Real,
    Imaginary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceAcCsdColumnExpansion {
    Scalar,
    Complex,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct XyceTransientReferenceLayout {
    stepnum_column: Option<usize>,
    index_column: Option<usize>,
    time_column: usize,
    data_column_offset: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceLeadCurrentProbe {
    terminal: XyceLeadCurrentTerminal,
    element_name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceLeadCurrentTerminal {
    Drain,
    Gate,
    Source,
    Bulk,
    Collector,
    Emitter,
}

impl XyceLeadCurrentTerminal {
    fn from_function_name(name: &str) -> Option<Self> {
        match name.to_ascii_lowercase().as_str() {
            "id" => Some(Self::Drain),
            "ig" => Some(Self::Gate),
            "is" => Some(Self::Source),
            "ib" => Some(Self::Bulk),
            "ic" => Some(Self::Collector),
            "ie" => Some(Self::Emitter),
            _ => None,
        }
    }

    fn op_parameter(self) -> Option<&'static str> {
        match self {
            Self::Drain => Some("id"),
            Self::Source => Some("is"),
            Self::Gate | Self::Bulk | Self::Collector | Self::Emitter => None,
        }
    }

    fn function_name(self) -> &'static str {
        match self {
            Self::Drain => "ID",
            Self::Gate => "IG",
            Self::Source => "IS",
            Self::Bulk => "IB",
            Self::Collector => "IC",
            Self::Emitter => "IE",
        }
    }
}

#[derive(Debug, Clone)]
struct XycePrnTable {
    columns: Vec<String>,
    rows: Vec<Vec<f64>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XycePrnDelimiter {
    Whitespace,
    Comma,
}

#[derive(Debug, Clone, Copy)]
struct DeadlineAbort {
    start: Instant,
    deadline: Duration,
}

impl DeadlineAbort {
    fn new(start: Instant, timeout_ms: u128) -> Self {
        Self {
            start,
            deadline: Duration::from_millis(timeout_ms.min(u128::from(u64::MAX)) as u64),
        }
    }
}

impl AbortSignal for DeadlineAbort {
    fn is_aborted(&self) -> bool {
        self.start.elapsed() >= self.deadline
    }
}

impl XyceTestRunner {
    /// Create a runner rooted at `tests/xyce`.
    pub fn new<P: AsRef<Path>>(root: P, config: XyceRunnerConfig) -> Self {
        let root = root
            .as_ref()
            .canonicalize()
            .unwrap_or_else(|_| root.as_ref().to_path_buf());
        let upstream_wrapper_decks = Self::load_upstream_wrapper_decks(&root);
        Self {
            root,
            config,
            upstream_wrapper_decks,
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn config(&self) -> &XyceRunnerConfig {
        &self.config
    }

    fn parse_xyce_netlist(
        source: &str,
        deck_path: &Path,
    ) -> Result<Netlist, crate::netlist::ParseError> {
        Self::parse_netlist_with_expression_dialect(source, deck_path, ExpressionDialect::Xyce)
    }

    fn parse_error_is_undefined_ac_frequency_symbol(err: &crate::netlist::ParseError) -> bool {
        let message = err.to_string().to_ascii_uppercase();
        message.contains("UNDEFINED PARAMETER: FREQ")
            || message.contains("UNDEFINED PARAMETER: HERTZ")
    }

    fn source_with_ac_frequency_bindings(source: &str, frequency: Value) -> String {
        let mut lines = source.lines();
        let title = lines.next().unwrap_or("Untitled");
        let mut rebound = String::new();
        rebound.push_str(title);
        rebound.push('\n');
        rebound.push_str(&format!(
            ".PARAM FREQ={frequency:.17e} HERTZ={frequency:.17e}\n"
        ));
        for line in lines {
            rebound.push_str(line);
            rebound.push('\n');
        }
        rebound
    }

    fn parse_netlist_with_expression_dialect(
        source: &str,
        deck_path: &Path,
        expression_dialect: ExpressionDialect,
    ) -> Result<Netlist, crate::netlist::ParseError> {
        Self::parse_netlist_with_expression_dialect_and_execution_dir(
            source,
            deck_path,
            expression_dialect,
            None,
        )
    }

    fn parse_netlist_with_expression_dialect_and_execution_dir(
        source: &str,
        deck_path: &Path,
        expression_dialect: ExpressionDialect,
        execution_dir: Option<&Path>,
    ) -> Result<Netlist, crate::netlist::ParseError> {
        let options = NetlistParseOptions {
            statistical_mode: StatisticalParamMode::Nominal,
            expression_dialect,
        };
        if let Some(execution_dir) = execution_dir {
            return Netlist::parse_with_path_and_execution_dir(
                source,
                deck_path,
                execution_dir,
                options,
            );
        }

        Netlist::parse_with_path_and_options(source, deck_path, options)
    }

    /// RSpice never executes upstream `.sh`/`.pl` harness files in this runner.
    pub fn executes_upstream_scripts(&self) -> bool {
        false
    }

    /// Whether a retained deck had an upstream `.cir.sh` wrapper sidecar in the
    /// source corpus. Those scripts are intentionally not vendored; the
    /// cross-platform manifest records the execution contract instead.
    pub fn requires_upstream_wrapper(&self, relative_path: &str) -> bool {
        self.upstream_wrapper_decks
            .contains(&Self::normalize_manifest_key(relative_path))
    }

    /// Discover every `.cir` file under the vendored Xyce root.
    pub fn discover_tests(&self) -> Vec<XyceDeck> {
        let mut paths = Vec::new();
        Self::collect_circuit_files(&self.root, &mut paths);
        paths.sort();
        paths
            .into_iter()
            .map(|path| {
                let relative_path = self.relative_key(&path);
                let section = Self::section_for_relative_path(&relative_path);
                XyceDeck {
                    path,
                    relative_path,
                    section,
                }
            })
            .collect()
    }

    /// Discover simulator regression decks under `Netlists`.
    pub fn discover_netlist_tests(&self) -> Vec<XyceDeck> {
        self.discover_tests()
            .into_iter()
            .filter(|deck| deck.section == XyceDeckSection::Netlists)
            .collect()
    }

    /// Run every discovered vendored `.cir` through the Xyce corpus contract.
    pub fn run_all(&self) -> Vec<XyceTestResult> {
        self.discover_tests()
            .into_iter()
            .map(|deck| self.run_discovered_test(&deck))
            .collect()
    }

    /// Run every simulator regression deck under `Netlists`.
    pub fn run_netlist_suite(&self) -> Vec<XyceTestResult> {
        self.discover_netlist_tests()
            .into_iter()
            .map(|deck| self.run_discovered_test(&deck))
            .collect()
    }

    /// Run one deck path.
    pub fn run_test<P: AsRef<Path>>(&self, deck_path: P) -> XyceTestResult {
        let path = deck_path
            .as_ref()
            .canonicalize()
            .unwrap_or_else(|_| deck_path.as_ref().to_path_buf());
        let relative_path = self.relative_key(&path);
        let deck = XyceDeck {
            path,
            section: Self::section_for_relative_path(&relative_path),
            relative_path,
        };
        self.run_discovered_test(&deck)
    }

    pub fn statistics(results: &[XyceTestResult]) -> XyceStatistics {
        let mut stats = XyceStatistics::default();
        stats.total = results.len();
        for result in results {
            stats.total_time_ms += result.duration_ms;
            if result.expected_unsupported {
                stats.expected_unsupported += 1;
            } else {
                stats.executed += 1;
                if result.passed {
                    stats.passed += 1;
                }
            }
            if !result.passed {
                stats.failed += 1;
            }
        }
        stats
    }

    pub fn print_summary(results: &[XyceTestResult]) {
        let stats = Self::statistics(results);
        println!("\nXyce corpus summary");
        println!(
            "  total: {} | executed: {} | passed: {} | failed: {} | expected unsupported: {}",
            stats.total, stats.executed, stats.passed, stats.failed, stats.expected_unsupported
        );
        println!(
            "  executed pass rate: {:.1}% | executed coverage: {:.1}%",
            stats.executed_pass_rate(),
            stats.executed_coverage_rate()
        );

        for result in results.iter().filter(|result| !result.passed).take(20) {
            println!(
                "  FAIL {} [{}]: {}",
                result.relative_path,
                result.contract,
                result.error.as_deref().unwrap_or("unknown failure")
            );
            for mismatch in result.mismatches.iter().take(3) {
                println!(
                    "       row {} {} expected {:.8e}, actual {:.8e}, rel {:.3e}",
                    mismatch.row,
                    mismatch.probe,
                    mismatch.expected,
                    mismatch.actual,
                    mismatch.relative_error
                );
            }
        }
    }

    fn run_discovered_test(&self, deck: &XyceDeck) -> XyceTestResult {
        let start = Instant::now();
        if deck.section != XyceDeckSection::Netlists {
            return self.expected_unsupported_result(
                deck,
                start,
                "upstream_harness_fixture",
                "deck is part of upstream Xyce harness self-tests, not the simulator Netlists corpus",
            );
        }

        if let Some(result) = self.run_expected_error_contract(deck, start) {
            if self.config.verbose {
                println!(
                    "{} [{}] {}",
                    result.relative_path,
                    result.contract,
                    if result.passed { "PASS" } else { "FAIL" }
                );
            }
            return result;
        }

        if let Some(contract) = self.baseline_family_contract(deck) {
            let result = self.run_baseline_family_contract(deck, contract, start);
            if self.config.verbose {
                println!(
                    "{} [{}] {}",
                    result.relative_path,
                    result.contract,
                    if result.passed { "PASS" } else { "FAIL" }
                );
            }
            return result;
        }

        let result = match self.execution_plan(deck) {
            Ok(plan) => self.run_static_prn_dc_plan(deck, plan, start),
            Err(dc_reason) => match self.static_ac_plan_for_deck(deck) {
                Ok(plan) => self.run_static_fd_prn_ac_plan(deck, plan, start),
                Err(ac_reason) => match self.static_tran_plan_for_deck(deck) {
                    Ok(plan) => self.run_static_prn_tran_plan(deck, plan, start),
                    Err(tran_reason) => {
                        let reason = if self.deck_has_print_analysis(deck, "AC") {
                            ac_reason
                        } else if self.deck_has_print_analysis(deck, "TRAN") {
                            tran_reason
                        } else {
                            dc_reason
                        };
                        return self.expected_unsupported_result(
                            deck,
                            start,
                            "unsupported_xyce_contract",
                            &reason,
                        );
                    }
                },
            },
        };
        if self.config.verbose {
            println!(
                "{} [{}] {}",
                result.relative_path,
                result.contract,
                if result.passed { "PASS" } else { "FAIL" }
            );
        }
        result
    }

    fn expected_unsupported_result(
        &self,
        deck: &XyceDeck,
        start: Instant,
        contract: &str,
        reason: &str,
    ) -> XyceTestResult {
        XyceTestResult {
            name: Self::deck_name(&deck.path),
            relative_path: deck.relative_path.clone(),
            passed: true,
            expected_unsupported: true,
            error: Some(format!("{EXPECTED_UNSUPPORTED_MARKER} {reason}")),
            mismatches: Vec::new(),
            duration_ms: start.elapsed().as_millis(),
            contract: contract.to_string(),
        }
    }

    fn failure_result(
        &self,
        deck: &XyceDeck,
        start: Instant,
        contract: &str,
        error: String,
        mismatches: Vec<XyceValueMismatch>,
    ) -> XyceTestResult {
        XyceTestResult {
            name: Self::deck_name(&deck.path),
            relative_path: deck.relative_path.clone(),
            passed: false,
            expected_unsupported: false,
            error: Some(error),
            mismatches,
            duration_ms: start.elapsed().as_millis(),
            contract: contract.to_string(),
        }
    }

    fn passed_result(&self, deck: &XyceDeck, start: Instant, contract: &str) -> XyceTestResult {
        XyceTestResult {
            name: Self::deck_name(&deck.path),
            relative_path: deck.relative_path.clone(),
            passed: true,
            expected_unsupported: false,
            error: None,
            mismatches: Vec::new(),
            duration_ms: start.elapsed().as_millis(),
            contract: contract.to_string(),
        }
    }

    fn run_expected_error_contract(
        &self,
        deck: &XyceDeck,
        start: Instant,
    ) -> Option<XyceTestResult> {
        let reference_path = self.static_prn_reference_path(&deck.path)?;
        if reference_path.is_file() {
            return None;
        }

        let source = fs::read_to_string(&deck.path).ok()?;
        if Self::is_expected_missing_inductor_value_error_deck(&deck.relative_path, &source) {
            let contract = "expected_error_missing_inductor_value";
            return Self::validate_expected_missing_inductor_value_error_source(
                &source, &deck.path,
            )
            .ok()
            .map(|()| self.passed_result(deck, start, contract));
        }

        if !Self::source_may_have_pwl_repeat_option(&source) {
            return None;
        }

        let contract = "expected_error_pwl_repeat_value";
        Self::validate_expected_pwl_repeat_value_error_source(&source, &deck.path)
            .ok()
            .map(|()| self.passed_result(deck, start, contract))
    }

    fn execution_plan(&self, deck: &XyceDeck) -> Result<XyceExecutionPlan, String> {
        let requires_wrapper = self.requires_upstream_wrapper(&deck.relative_path);
        let source =
            fs::read_to_string(&deck.path).map_err(|err| format!("failed to read deck: {err}"))?;

        let wrapper_contract = if requires_wrapper {
            Some(Self::native_static_prn_wrapper_contract(
                &deck.relative_path,
                &deck.path,
                &source,
            )?)
        } else {
            None
        };
        let (execution_deck_path, execution_dir) = if matches!(
            wrapper_contract,
            Some(XyceStaticDcContract::WrapperTopLevelExecutionDir)
        ) {
            (
                Self::top_level_execution_deck_path(&deck.path)?,
                Some(
                    deck.path
                        .parent()
                        .ok_or_else(|| "wrapper deck has no parent directory".to_string())?
                        .to_path_buf(),
                ),
            )
        } else {
            (deck.path.clone(), None)
        };

        let expression_dialect = if matches!(
            wrapper_contract,
            Some(XyceStaticDcContract::WrapperHspiceMath)
        ) {
            ExpressionDialect::Ngspice
        } else {
            ExpressionDialect::Xyce
        };
        let static_plan = self.static_dc_plan_for_path_with_execution_dir(
            &execution_deck_path,
            expression_dialect,
            execution_dir.as_deref(),
        )?;
        let contract = if let Some(contract) = wrapper_contract {
            self.validate_native_static_prn_wrapper_contract(contract, &static_plan)?;
            contract
        } else {
            Self::static_dc_contract_for_print_format(false, static_plan.print_format.as_deref())?
        };
        if matches!(contract, XyceStaticDcContract::WrapperFilePrn) && !static_plan.steps.is_empty()
        {
            return Err(
                "wrapper-origin file-output contract does not cover .STEP DC decks yet".to_string(),
            );
        }
        let reference_path = self
            .static_output_reference_path(&deck.path, contract.reference_extension())
            .ok_or_else(|| "deck is not under tests/xyce/Netlists".to_string())?;
        if !reference_path.is_file()
            && !matches!(
                contract,
                XyceStaticDcContract::WrapperFilePrn | XyceStaticDcContract::WrapperNoOutput
            )
        {
            return Err(format!(
                "no checked-in static .{} oracle at {}",
                contract.reference_extension(),
                self.display_path(&reference_path)
            ));
        }

        Ok(XyceExecutionPlan {
            deck_path: static_plan.deck_path,
            execution_dir: static_plan.execution_dir,
            reference_path,
            source: static_plan.source,
            expression_dialect,
            print: static_plan.print,
            dc: static_plan.dc,
            dc_data: static_plan.dc_data,
            steps: static_plan.steps,
            contract,
        })
    }

    fn static_tran_plan_for_deck(&self, deck: &XyceDeck) -> Result<XyceStaticTranPlan, String> {
        let requires_wrapper = self.requires_upstream_wrapper(&deck.relative_path);
        let source =
            fs::read_to_string(&deck.path).map_err(|err| format!("failed to read deck: {err}"))?;
        if Self::contains_control_block(&source) {
            return Err(
                "deck uses a .control block; Xyce adapter does not interpret simulator scripting"
                    .to_string(),
            );
        }
        Self::reject_unsupported_source_directives(&source)?;

        let output_override = requires_wrapper
            && Self::is_native_output_override_wrapper_candidate_path(&deck.relative_path);
        let print_output = if output_override {
            Self::output_override_print_output_request(&source, "TRAN")?.ok_or_else(|| {
                "output override deck has no .PRINT TRAN statement with static columns".to_string()
            })?
        } else {
            Self::single_tran_print_output_request(&source)?
        };
        let print = XycePrintRequest {
            probes: print_output.probes.clone(),
        };
        let netlist = Self::parse_xyce_netlist(&source, &deck.path)
            .map_err(|err| format!("netlist parser does not yet accept this Xyce deck: {err}"))?;
        let tran = Self::single_tran_analysis(&netlist)?;
        let steps = Self::step_commands(&netlist)?;
        let has_prn_oracle = self
            .static_output_reference_path(
                &deck.path,
                XyceStaticTranContract::WrapperStatic.reference_extension(),
            )
            .is_some_and(|path| path.is_file());
        let native_static_prn_wrapper = if requires_wrapper {
            if output_override {
                Self::native_output_override_prn_tran_wrapper_contract(&source)
                    .map(Some)
                    .map_err(|_| Self::upstream_wrapper_required_reason().to_string())?
            } else {
                Self::native_static_prn_tran_wrapper_contract(
                    &deck.path,
                    &deck.relative_path,
                    &source,
                    has_prn_oracle,
                )
            }
        } else {
            None
        };
        if steps.is_empty() && requires_wrapper && native_static_prn_wrapper.is_none() {
            return Err(Self::upstream_wrapper_required_reason().to_string());
        }
        let contract = if requires_wrapper {
            native_static_prn_wrapper.unwrap_or(XyceStaticTranContract::WrapperStatic)
        } else {
            Self::static_tran_contract_for_print_format(false, print_output.format.as_deref())?
        };
        let primary_reference_path = self
            .static_output_reference_path(&deck.path, contract.reference_extension())
            .ok_or_else(|| "deck is not under tests/xyce/Netlists".to_string())?;
        let reference_path = if primary_reference_path.is_file() {
            primary_reference_path
        } else if matches!(contract, XyceStaticTranContract::WrapperCsd) {
            Self::tran_gsfile_reference_path(&deck.path).ok_or_else(|| {
                format!(
                    "no checked-in static .{} oracle at {}",
                    contract.reference_extension(),
                    self.display_path(&primary_reference_path)
                )
            })?
        } else {
            primary_reference_path
        };
        if !reference_path.is_file() {
            return Err(format!(
                "no checked-in static .{} oracle at {}",
                contract.reference_extension(),
                self.display_path(&reference_path)
            ));
        }
        if !steps.is_empty() {
            Self::validate_static_step_tran_contract(&netlist)?;
        }
        if requires_wrapper {
            match contract {
                XyceStaticTranContract::WrapperCsd => {
                    Self::validate_native_static_csd_tran_wrapper_contract(&source)?;
                }
                XyceStaticTranContract::WrapperCsv => {
                    Self::validate_native_static_csv_tran_wrapper_contract(&source)?;
                }
                XyceStaticTranContract::WrapperStatic => {
                    if output_override {
                        Self::validate_native_output_override_prn_tran_wrapper_contract(&source)?;
                    } else {
                        Self::validate_native_static_prn_tran_wrapper_contract_with_format_mode(
                            &source,
                            has_prn_oracle,
                        )?;
                    }
                }
                XyceStaticTranContract::WrapperStaticExpectedError => {
                    Self::validate_native_static_prn_tran_wrapper_contract(&source)?;
                }
                _ => Self::validate_native_static_prn_tran_wrapper_contract(&source)?,
            }
        }
        Self::validate_static_tran_contract(&netlist, &tran, &print)?;

        Ok(XyceStaticTranPlan {
            deck_path: deck.path.clone(),
            reference_path,
            source,
            print,
            output_override,
            tran,
            steps,
            wrapper_tolerance: Self::native_default_prn_tran_wrapper_tolerance(&deck.relative_path),
            contract,
        })
    }

    fn static_ac_plan_for_deck(&self, deck: &XyceDeck) -> Result<XyceStaticAcPlan, String> {
        let requires_wrapper = self.requires_upstream_wrapper(&deck.relative_path);
        let output_override = requires_wrapper
            && Self::is_native_output_override_wrapper_candidate_path(&deck.relative_path);
        let source =
            fs::read_to_string(&deck.path).map_err(|err| format!("failed to read deck: {err}"))?;
        if Self::contains_control_block(&source) {
            return Err(
                "deck uses a .control block; Xyce adapter does not interpret simulator scripting"
                    .to_string(),
            );
        }
        Self::reject_unsupported_source_directives(&source)?;
        if requires_wrapper {
            Self::validate_native_static_fd_ac_wrapper_contract(&source, output_override)?;
        }

        let primary_ac_output = if output_override {
            Self::output_override_print_output_request(&source, "AC")?
        } else {
            Self::canonical_print_output_request(&source, "AC", requires_wrapper)?
        };
        let primary_ac_ic_output = if output_override {
            Self::output_override_print_output_request(&source, "AC_IC")?
        } else {
            Self::canonical_print_output_request(&source, "AC_IC", requires_wrapper)?
        };
        if primary_ac_output.is_none() && primary_ac_ic_output.is_none() {
            return Err(
                "deck has no primary .PRINT AC or .PRINT AC_IC statement with static columns"
                    .to_string(),
            );
        }

        let (netlist, frequency_bound) = match Self::parse_xyce_netlist(&source, &deck.path) {
            Ok(netlist) => (netlist, false),
            Err(err) if Self::parse_error_is_undefined_ac_frequency_symbol(&err) => {
                let frequency_bound_source = Self::source_with_ac_frequency_bindings(&source, 1.0);
                let netlist = Self::parse_xyce_netlist(&frequency_bound_source, &deck.path)
                    .map_err(|retry_err| {
                        format!(
                            "netlist parser does not yet accept this Xyce deck even with AC frequency bindings: {retry_err}"
                        )
                    })?;
                (netlist, true)
            }
            Err(err) => {
                return Err(format!(
                    "netlist parser does not yet accept this Xyce deck: {err}"
                ));
            }
        };
        let ac = Self::single_ac_analysis(&netlist)?;
        let steps = Self::step_commands(&netlist)?;
        if ac.data_points().is_some() && !steps.is_empty() {
            return Err(
                ".STEP combined with .AC DATA is not implemented in the native Xyce oracle"
                    .to_string(),
            );
        }
        if frequency_bound && !steps.is_empty() {
            return Err(
                ".STEP combined with AC frequency-dependent parameters is not implemented in the native Xyce oracle"
                    .to_string(),
            );
        }
        if frequency_bound && ac.data_points().is_some() {
            return Err(
                ".AC DATA combined with AC frequency-dependent parameters is not implemented in the native Xyce oracle"
                    .to_string(),
            );
        }

        let primary_ac_ic_file = primary_ac_ic_output
            .as_ref()
            .and_then(|request| request.file.clone());
        let (contract, reference_path, print, primary_ac_file) =
            if let Some(print_output) = primary_ac_output {
                let primary_ac_file = print_output.file.clone();
                let contract = Self::static_ac_contract_for_print_format(
                    requires_wrapper,
                    print_output.format.as_deref(),
                )?;
                let reference_path = self
                    .static_output_reference_path(&deck.path, contract.reference_extension())
                    .ok_or_else(|| "deck is not under tests/xyce/Netlists".to_string())?;
                if !reference_path.is_file() {
                    return Err(format!(
                        "no checked-in static .{} oracle at {}",
                        contract.reference_extension(),
                        self.display_path(&reference_path)
                    ));
                }
                let print = XycePrintRequest {
                    probes: print_output.probes,
                };
                Self::validate_static_ac_contract(&netlist, &ac, &print)?;
                (contract, Some(reference_path), Some(print), primary_ac_file)
            } else if let Some(print_output) = primary_ac_ic_output {
                if !steps.is_empty() {
                    return Err(
                    ".STEP AC_IC-only comparison is not implemented for wrapper-origin AC output"
                        .to_string(),
                );
                }
                let contract = Self::static_ac_ic_contract_for_print_format(
                    requires_wrapper,
                    print_output.format.as_deref(),
                )?;
                let reference_path = self
                    .static_output_reference_path(&deck.path, contract.reference_extension())
                    .ok_or_else(|| "deck is not under tests/xyce/Netlists".to_string())?;
                if !reference_path.is_file() {
                    return Err(format!(
                        "no checked-in static .{} oracle at {}",
                        contract.reference_extension(),
                        self.display_path(&reference_path)
                    ));
                }
                (contract, None, None, None)
            } else {
                unreachable!("AC output request presence was checked before parsing");
            };

        Ok(XyceStaticAcPlan {
            deck_path: deck.path.clone(),
            reference_path,
            source,
            print,
            primary_ac_file,
            primary_ac_ic_file,
            output_override,
            ac,
            frequency_bound,
            steps,
            contract,
        })
    }

    fn static_dc_plan_for_path(
        &self,
        deck_path: &Path,
        expression_dialect: ExpressionDialect,
    ) -> Result<XyceStaticDcPlan, String> {
        self.static_dc_plan_for_path_with_execution_dir(deck_path, expression_dialect, None)
    }

    fn static_dc_plan_for_path_with_execution_dir(
        &self,
        deck_path: &Path,
        expression_dialect: ExpressionDialect,
        execution_dir: Option<&Path>,
    ) -> Result<XyceStaticDcPlan, String> {
        let source =
            fs::read_to_string(deck_path).map_err(|err| format!("failed to read deck: {err}"))?;
        if Self::contains_control_block(&source) {
            return Err(
                "deck uses a .control block; Xyce adapter does not interpret simulator scripting"
                    .to_string(),
            );
        }
        Self::reject_unsupported_source_directives(&source)?;

        let print_output = Self::single_dc_or_file_output_request(&source).or_else(|err| {
            if Self::validate_no_output_dc_wrapper_source(&source).is_ok() {
                Ok(XycePrintOutputRequest {
                    format: None,
                    file: None,
                    probes: Vec::new(),
                })
            } else {
                Err(err)
            }
        })?;
        let print = XycePrintRequest {
            probes: print_output.probes,
        };
        let netlist = Self::parse_netlist_with_expression_dialect_and_execution_dir(
            &source,
            deck_path,
            expression_dialect,
            execution_dir,
        )
        .map_err(|err| format!("netlist parser does not yet accept this Xyce deck: {err}"))?;
        let diagnostics = netlist.diagnostics.clone();
        let dc_data = Self::dc_data_sweep_for_source(&source, &netlist)?;
        let dc = match &dc_data {
            Some(dc_data) => Self::synthetic_dc_data_sweep(dc_data)?,
            None => Self::single_dc_sweep(&netlist)?,
        };
        let steps = Self::step_commands(&netlist)?;
        if dc_data.is_some() && !steps.is_empty() {
            return Err(
                ".STEP combined with .DC DATA is not covered by the native static DC adapter yet"
                    .to_string(),
            );
        }
        if let Some(dc_data) = &dc_data {
            Self::validate_static_dc_data_contract(&netlist, dc_data, &print)?;
        } else {
            Self::validate_static_dc_contract(&netlist, &dc, &print)?;
        }

        Ok(XyceStaticDcPlan {
            deck_path: deck_path.to_path_buf(),
            execution_dir: execution_dir.map(Path::to_path_buf),
            source,
            expression_dialect,
            print,
            print_format: print_output.format,
            dc,
            dc_data,
            steps,
            diagnostics,
        })
    }

    fn validate_native_static_prn_wrapper_contract(
        &self,
        contract: XyceStaticDcContract,
        plan: &XyceStaticDcPlan,
    ) -> Result<(), String> {
        if matches!(contract, XyceStaticDcContract::WrapperResistorDefault) {
            Self::validate_resistor_default_wrapper_diagnostics(plan)?;
        }

        Ok(())
    }

    fn validate_resistor_default_wrapper_diagnostics(
        plan: &XyceStaticDcPlan,
    ) -> Result<(), String> {
        let default_warning_count = plan
            .diagnostics
            .iter()
            .filter(|diagnostic| {
                diagnostic
                    .code
                    .eq_ignore_ascii_case("xyce_resistor_missing_value")
                    || diagnostic
                        .code
                        .eq_ignore_ascii_case("xyce_resistor_model_missing_value")
            })
            .count();
        if default_warning_count == 0 {
            return Err(
                "wrapper-origin resistor default contract requires native value-default diagnostics"
                    .to_string(),
            );
        }
        Ok(())
    }

    fn native_static_prn_wrapper_contract(
        relative_path: &str,
        deck_path: &Path,
        source: &str,
    ) -> Result<XyceStaticDcContract, String> {
        if Self::is_native_gnuplot_splot_wrapper_candidate(source) {
            return Ok(XyceStaticDcContract::WrapperGnuplotSplot);
        }

        if Self::is_native_csv_dc_wrapper_candidate(relative_path, source) {
            Self::validate_csv_wrapper_source(source)?;
            return Ok(XyceStaticDcContract::WrapperCsv);
        }

        if Self::is_native_csd_dc_wrapper_candidate(relative_path, source) {
            Self::validate_csd_wrapper_source(source)?;
            return Ok(XyceStaticDcContract::WrapperCsd);
        }

        if Self::is_native_file_only_prn_wrapper_candidate(relative_path, source) {
            Self::validate_file_only_prn_wrapper_source(source)?;
            return Ok(XyceStaticDcContract::WrapperFilePrn);
        }

        if Self::is_native_raw_wrapper_candidate_path(relative_path) {
            Self::validate_raw_wrapper_source(source)?;
            return Ok(XyceStaticDcContract::WrapperRaw);
        }

        if Self::is_native_no_output_dc_wrapper_candidate(relative_path, source) {
            Self::validate_no_output_dc_wrapper_source(source)?;
            return Ok(XyceStaticDcContract::WrapperNoOutput);
        }

        if Self::is_native_default_prn_wrapper_candidate_path(relative_path)
            || Self::is_native_multiplicity_static_prn_wrapper_candidate_path(relative_path)
        {
            Self::validate_default_prn_wrapper_source(source)?;
            return Ok(XyceStaticDcContract::WrapperDefault);
        }

        if Self::is_native_hspice_math_wrapper_candidate(relative_path, source) {
            Self::validate_hspice_math_wrapper_source(source)?;
            return Ok(XyceStaticDcContract::WrapperHspiceMath);
        }

        if Self::is_native_hspice_random_wrapper_candidate(relative_path, source) {
            Self::validate_default_prn_wrapper_source(source)?;
            return Ok(XyceStaticDcContract::WrapperDefault);
        }

        if Self::is_native_resistor_default_wrapper_candidate(relative_path, source) {
            Self::validate_default_prn_wrapper_source(source)?;
            return Ok(XyceStaticDcContract::WrapperResistorDefault);
        }

        if Self::is_native_resistor_temperature_step_wrapper_candidate(relative_path, source) {
            Self::validate_default_prn_wrapper_source(source)?;
            return Ok(XyceStaticDcContract::WrapperDefault);
        }

        if Self::is_native_semiconductor_resistor_step_wrapper_candidate(relative_path, source) {
            Self::validate_default_prn_wrapper_source(source)?;
            return Ok(XyceStaticDcContract::WrapperDefault);
        }

        if Self::is_native_top_level_execution_dir_wrapper_candidate(deck_path, source) {
            return Ok(XyceStaticDcContract::WrapperTopLevelExecutionDir);
        }

        if Self::is_native_step_data_wrapper_candidate(source) {
            Self::validate_default_prn_wrapper_source(source)?;
            return Ok(XyceStaticDcContract::WrapperDefault);
        }

        if Self::is_native_dc_data_table_wrapper_candidate(source) {
            Self::validate_default_prn_wrapper_source(source)?;
            return Ok(XyceStaticDcContract::WrapperDefault);
        }

        if Self::is_native_empty_wildcard_lead_current_wrapper_candidate(deck_path, source) {
            Self::validate_default_prn_wrapper_source(source)?;
            return Ok(XyceStaticDcContract::WrapperDefault);
        }

        if Self::is_native_subcircuit_node_probe_wrapper_candidate(source) {
            Self::validate_default_prn_wrapper_source(source)?;
            return Ok(XyceStaticDcContract::WrapperDefault);
        }

        if Self::is_native_voltage_accessor_wrapper_candidate(source) {
            Self::validate_default_prn_wrapper_source(source)?;
            return Ok(XyceStaticDcContract::WrapperVoltageAccessor);
        }

        if Self::is_native_plain_static_dc_prn_wrapper_candidate(deck_path, source) {
            return Ok(XyceStaticDcContract::WrapperDefault);
        }

        Err(Self::upstream_wrapper_required_reason().to_string())
    }

    fn is_native_default_prn_wrapper_candidate_path(relative_path: &str) -> bool {
        Self::normalize_manifest_key(relative_path).starts_with("netlists/output/dc/")
    }

    fn is_native_csv_dc_wrapper_candidate(relative_path: &str, source: &str) -> bool {
        if !Self::normalize_manifest_key(relative_path).starts_with("netlists/output/dc/") {
            return false;
        }
        Self::dc_print_output_requests(source).is_ok_and(|requests| {
            requests.into_iter().any(|request| {
                request.file.is_none()
                    && request
                        .format
                        .as_deref()
                        .is_some_and(|format| format.eq_ignore_ascii_case("CSV"))
            })
        })
    }

    fn is_native_csd_dc_wrapper_candidate(relative_path: &str, source: &str) -> bool {
        if !Self::normalize_manifest_key(relative_path).starts_with("netlists/output/dc/") {
            return false;
        }
        Self::dc_print_output_requests(source).is_ok_and(|requests| {
            requests.into_iter().any(|request| {
                request.file.is_none()
                    && request
                        .format
                        .as_deref()
                        .is_some_and(|format| format.eq_ignore_ascii_case("PROBE"))
            })
        })
    }

    fn is_native_no_output_dc_wrapper_candidate(relative_path: &str, source: &str) -> bool {
        matches!(
            Self::normalize_manifest_key(relative_path).as_str(),
            "netlists/output/dc/dc-noprn.cir"
        ) && Self::validate_no_output_dc_wrapper_source(source).is_ok()
    }

    fn is_native_file_only_prn_wrapper_candidate(relative_path: &str, source: &str) -> bool {
        if !Self::normalize_manifest_key(relative_path).starts_with("netlists/output/dc/") {
            return false;
        }
        Self::dc_print_output_requests(source).is_ok_and(|requests| {
            !requests.is_empty()
                && requests.iter().all(|request| {
                    request.file.is_some()
                        && Self::dc_print_format_is_prn_compatible(
                            request.format.as_deref().unwrap_or("STD"),
                        )
                })
        })
    }

    fn is_native_raw_wrapper_candidate_path(relative_path: &str) -> bool {
        let relative_path = Self::normalize_manifest_key(relative_path);
        relative_path.starts_with("netlists/output/dc/")
            && relative_path
                .rsplit('/')
                .next()
                .is_some_and(|file_name| file_name.contains("-raw"))
    }

    fn is_native_output_override_wrapper_candidate_path(relative_path: &str) -> bool {
        Self::normalize_manifest_key(relative_path).starts_with("netlists/output/dasho/")
    }

    fn is_native_multiplicity_static_prn_wrapper_candidate_path(relative_path: &str) -> bool {
        Self::normalize_manifest_key(relative_path).starts_with("netlists/multiplicity_factor/")
    }

    fn is_native_hspice_math_wrapper_candidate(relative_path: &str, source: &str) -> bool {
        let relative_path = Self::normalize_manifest_key(relative_path);
        let normalized_source = source.to_ascii_lowercase();
        relative_path.starts_with("netlists/parser/")
            && normalized_source.contains("-hspice-ext math")
            && normalized_source.contains("-hspice-ext all")
    }

    fn is_native_hspice_random_wrapper_candidate(relative_path: &str, source: &str) -> bool {
        let relative_path = Self::normalize_manifest_key(relative_path);
        let normalized_source = source.to_ascii_lowercase();
        relative_path.starts_with("netlists/parser/")
            && ["agauss(", "gauss(", "aunif(", "unif(", "rand("]
                .iter()
                .any(|operator| normalized_source.contains(operator))
    }

    fn is_native_resistor_default_wrapper_candidate(relative_path: &str, source: &str) -> bool {
        let relative_path = Self::normalize_manifest_key(relative_path);
        let normalized_source = source.to_ascii_lowercase();
        relative_path.starts_with("netlists/resistor/")
            && normalized_source.contains("default to 1000")
            && normalized_source.contains("warning")
    }

    fn is_native_resistor_temperature_step_wrapper_candidate(
        relative_path: &str,
        source: &str,
    ) -> bool {
        let relative_path = Self::normalize_manifest_key(relative_path);
        if !relative_path.starts_with("netlists/resistor_td/") {
            return false;
        }

        let mut has_step_temp = false;
        let mut has_tce = false;
        for line in Self::logical_netlist_lines(source) {
            let trimmed = Self::strip_netlist_comment(&line)
                .trim()
                .to_ascii_lowercase();
            if trimmed.starts_with(".step ") && trimmed.contains("temp") {
                has_step_temp = true;
            }
            if trimmed.contains("tce") {
                has_tce = true;
            }
        }
        has_step_temp && has_tce
    }

    fn is_native_semiconductor_resistor_step_wrapper_candidate(
        relative_path: &str,
        source: &str,
    ) -> bool {
        let relative_path = Self::normalize_manifest_key(relative_path);
        if relative_path != "netlists/semic_resistor/semic_resistor_step.cir" {
            return false;
        }

        let mut has_resistor_geometry_step = false;
        let mut has_resistor_default_step = false;
        let mut has_resistor_model = false;
        for line in Self::logical_netlist_lines(source) {
            let trimmed = Self::strip_netlist_comment(&line)
                .trim()
                .to_ascii_lowercase();
            if trimmed.starts_with(".step ") {
                has_resistor_geometry_step |= trimmed.contains("r1:l") || trimmed.contains("r2:l");
                has_resistor_default_step |= trimmed
                    .split_whitespace()
                    .nth(1)
                    .is_some_and(|target| target.eq_ignore_ascii_case("r3"));
            }
            if trimmed.starts_with(".model ")
                && trimmed
                    .split_whitespace()
                    .nth(2)
                    .is_some_and(|model_type| model_type.eq_ignore_ascii_case("r"))
            {
                has_resistor_model = true;
            }
        }

        has_resistor_geometry_step && has_resistor_default_step && has_resistor_model
    }

    fn is_native_top_level_execution_dir_wrapper_candidate(deck_path: &Path, source: &str) -> bool {
        Self::logical_netlist_lines(source)
            .iter()
            .all(|line| Self::strip_netlist_comment(line).trim().is_empty())
            && Self::top_level_execution_deck_path(deck_path).is_ok_and(|path| path.is_file())
    }

    fn top_level_execution_deck_path(deck_path: &Path) -> Result<PathBuf, String> {
        let file_name = deck_path
            .file_name()
            .ok_or_else(|| "wrapper deck has no filename".to_string())?;
        let parent = deck_path
            .parent()
            .ok_or_else(|| "wrapper deck has no parent directory".to_string())?;
        Ok(parent.join("top_level").join(file_name))
    }

    fn is_native_step_data_wrapper_candidate(source: &str) -> bool {
        let mut has_data_table = false;
        let mut has_step_data = false;
        for line in Self::logical_netlist_lines(source) {
            let trimmed = Self::strip_netlist_comment(&line)
                .trim()
                .to_ascii_lowercase();
            if trimmed.starts_with(".data ") {
                has_data_table = true;
            }
            if trimmed.starts_with(".step ") && trimmed.contains("data") && trimmed.contains('=') {
                has_step_data = true;
            }
        }
        has_data_table && has_step_data
    }

    fn is_native_dc_data_table_wrapper_candidate(source: &str) -> bool {
        Self::dc_data_table_names(source).is_ok_and(|names| !names.is_empty())
    }

    fn is_native_gnuplot_splot_wrapper_candidate(source: &str) -> bool {
        if Self::wrapper_source_has_extra_output_analysis(source) {
            return false;
        }
        let Ok((primary, side)) = Self::gnuplot_splot_print_pair(source) else {
            return false;
        };
        primary.probes == side.probes
    }

    fn is_native_empty_wildcard_lead_current_wrapper_candidate(
        deck_path: &Path,
        source: &str,
    ) -> bool {
        if Self::validate_default_prn_wrapper_source(source).is_err() {
            return false;
        }
        let Ok(print) = Self::single_dc_print_request(source) else {
            return false;
        };
        let wildcard_probes = print
            .probes
            .iter()
            .filter_map(|probe| Self::parse_lead_current_probe(probe))
            .filter(|probe| probe.element_name == "*")
            .collect::<Vec<_>>();
        if wildcard_probes.is_empty() {
            return false;
        }
        let Ok(netlist) = Self::parse_xyce_netlist(source, deck_path) else {
            return false;
        };

        wildcard_probes
            .iter()
            .all(|probe| Self::lead_current_probe_is_omitted_empty_wildcard(&netlist, probe))
    }

    fn is_native_subcircuit_node_probe_wrapper_candidate(source: &str) -> bool {
        if Self::validate_default_prn_wrapper_source(source).is_err() {
            return false;
        }
        let Ok(print) = Self::single_dc_print_request(source) else {
            return false;
        };

        print
            .probes
            .iter()
            .any(|probe| Self::print_probe_contains_subcircuit_node_voltage_probe(probe))
    }

    fn print_probe_contains_subcircuit_node_voltage_probe(probe: &str) -> bool {
        let expression = Self::print_expression_inner(probe).unwrap_or(probe);
        let normalized = Self::normalize_probe(expression);
        if Self::voltage_probe_targets_subcircuit_node(&normalized) {
            return true;
        }

        let context = crate::netlist::ParamContext::new();
        let mut found = false;
        let _ = Self::rewrite_print_expression_calls_maybe(expression, context, |call| {
            if Self::voltage_probe_targets_subcircuit_node(&Self::normalize_probe(call)) {
                found = true;
            }
            Ok(0.0)
        });
        found
    }

    fn voltage_probe_targets_subcircuit_node(normalized_probe: &str) -> bool {
        Self::parse_voltage_probe(normalized_probe).is_some_and(|probe| {
            probe.node_pos.contains(':')
                || probe
                    .node_neg
                    .as_deref()
                    .is_some_and(|node| node.contains(':'))
        })
    }

    fn is_native_voltage_accessor_wrapper_candidate(source: &str) -> bool {
        if Self::validate_default_prn_wrapper_source(source).is_err() {
            return false;
        }
        let Ok(print) = Self::single_dc_print_request(source) else {
            return false;
        };

        print.probes.iter().any(|probe| {
            let expression = Self::print_expression_inner(probe).unwrap_or(probe);
            let normalized = Self::normalize_probe(expression);
            Self::parse_voltage_probe(&normalized)
                .is_some_and(|probe| probe.accessor != XyceVoltageAccessor::Value)
                || Self::print_expression_contains_voltage_accessor_call(expression)
        })
    }

    fn is_native_plain_static_dc_prn_wrapper_candidate(deck_path: &Path, source: &str) -> bool {
        if Self::validate_plain_static_dc_prn_wrapper_source(source).is_err() {
            return false;
        }
        let Ok(print) = Self::single_dc_print_request(source) else {
            return false;
        };
        let Ok(netlist) = Self::parse_xyce_netlist(source, deck_path) else {
            return false;
        };
        if Self::validate_plain_static_dc_prn_wrapper_netlist(&netlist).is_err() {
            return false;
        }
        let Ok(dc) = Self::single_dc_sweep(&netlist) else {
            return false;
        };
        Self::validate_static_dc_contract(&netlist, &dc, &print).is_ok()
    }

    fn validate_plain_static_dc_prn_wrapper_source(source: &str) -> Result<(), String> {
        Self::validate_default_prn_wrapper_source_with_format_mode(source, true)?;

        let print = Self::single_dc_print_request(source)?;
        for probe in &print.probes {
            let normalized = Self::normalize_probe(probe);
            if normalized.contains('*') || normalized.starts_with("w(") {
                return Err(format!(
                    "wrapper-origin plain static DC contract does not cover wildcard or power-style probe '{probe}'"
                ));
            }
        }

        let mut dc_count = 0usize;
        let mut model_count = 0usize;
        let mut subckt_count = 0usize;
        let mut ends_count = 0usize;
        for line in Self::logical_netlist_lines(source) {
            let trimmed = Self::strip_netlist_comment(&line).trim().to_string();
            if trimmed.is_empty() {
                continue;
            }
            let Some(command) = trimmed.split_whitespace().next() else {
                continue;
            };
            if !command.starts_with('.') {
                continue;
            }
            match command.to_ascii_lowercase().as_str() {
                ".dc" => dc_count += 1,
                ".model" => {
                    model_count += 1;
                    Self::validate_plain_static_dc_prn_wrapper_model_type(&trimmed)?;
                }
                ".subckt" => subckt_count += 1,
                ".ends" => ends_count += 1,
                ".print" | ".step" | ".param" | ".func" | ".options" | ".end" => {}
                other => {
                    return Err(format!(
                        "wrapper-origin plain static DC contract does not cover {other} directives"
                    ));
                }
            }
        }

        match dc_count {
            1 => Ok(()),
            0 => Err(
                "wrapper-origin plain static DC contract requires exactly one .DC statement, found none"
                    .to_string(),
            ),
            _ => Err(format!(
                "wrapper-origin plain static DC contract requires exactly one .DC statement, found {dc_count}"
            )),
        }?;

        if model_count > 1 {
            return Err(format!(
                "wrapper-origin plain static DC contract currently covers at most one .MODEL statement, found {model_count}"
            ));
        }
        if subckt_count > 1 {
            return Err(format!(
                "wrapper-origin plain static DC contract currently covers at most one .SUBCKT statement, found {subckt_count}"
            ));
        }
        if subckt_count != ends_count {
            return Err(format!(
                "wrapper-origin plain static DC contract requires balanced .SUBCKT/.ENDS statements, found {subckt_count}/{ends_count}"
            ));
        }
        Ok(())
    }

    fn validate_plain_static_dc_prn_wrapper_model_type(model_line: &str) -> Result<(), String> {
        let Some(model_type) = model_line.split_whitespace().nth(2) else {
            return Err(
                "wrapper-origin plain static DC contract requires .MODEL statements to include a model type"
                    .to_string(),
            );
        };
        let normalized = model_type
            .trim_matches(|ch| matches!(ch, '(' | ')' | ','))
            .to_ascii_uppercase();
        if matches!(normalized.as_str(), "NMOS" | "PMOS" | "D" | "DIODE") {
            return Ok(());
        }
        Err(format!(
            "wrapper-origin plain static DC contract does not yet cover .MODEL type {model_type}"
        ))
    }

    fn validate_plain_static_dc_prn_wrapper_netlist(netlist: &Netlist) -> Result<(), String> {
        if netlist.models.len() > 1 {
            return Err(format!(
                "wrapper-origin plain static DC contract currently covers at most one parsed model, found {}",
                netlist.models.len()
            ));
        }
        for model in &netlist.models {
            if !matches!(
                model.model_type.to_ascii_uppercase().as_str(),
                "NMOS" | "PMOS" | "D" | "DIODE"
            ) {
                return Err(format!(
                    "wrapper-origin plain static DC contract does not yet cover parsed model type {}",
                    model.model_type
                ));
            }
            if matches!(
                model.model_type.to_ascii_uppercase().as_str(),
                "D" | "DIODE"
            ) && !Self::model_is_native_legacy_diode(model)
            {
                return Err(format!(
                    "wrapper-origin plain static DC contract does not yet cover advanced diode model type {}",
                    model.model_type
                ));
            }
        }
        if netlist.subcircuits.len() > 1 {
            return Err(format!(
                "wrapper-origin plain static DC contract currently covers at most one parsed subcircuit, found {}",
                netlist.subcircuits.len()
            ));
        }
        Ok(())
    }

    fn upstream_wrapper_required_reason() -> &'static str {
        "upstream wrapper semantics are required; RSPICE-HARNESS-MANIFEST.tsv records the removed .cir.sh sidecar contract"
    }

    fn validate_hspice_math_wrapper_source(source: &str) -> Result<(), String> {
        Self::validate_default_prn_wrapper_source(source)?;
        let checks = [
            ("**", "HSPICE exponentiation operator '**'"),
            ("^", "HSPICE exponentiation operator '^'"),
            ("&&", "HSPICE logical AND operator '&&'"),
            ("||", "HSPICE logical OR operator '||'"),
            ("?", "ternary conditional operator '?'"),
            (":", "ternary conditional separator ':'"),
        ];
        for (needle, label) in checks {
            if !source.contains(needle) {
                return Err(format!(
                    "wrapper-origin HSPICE math contract requires {label}"
                ));
            }
        }
        Ok(())
    }

    fn validate_default_prn_wrapper_source(source: &str) -> Result<(), String> {
        Self::validate_default_prn_wrapper_source_with_format_mode(source, false)
    }

    fn validate_no_output_dc_wrapper_source(source: &str) -> Result<(), String> {
        let mut has_dc_or_op = false;
        for line in Self::logical_netlist_lines(source) {
            let trimmed = Self::strip_netlist_comment(&line).trim().to_string();
            if trimmed.is_empty() {
                continue;
            }
            let Some(command) = trimmed.split_whitespace().next() else {
                continue;
            };
            if command.eq_ignore_ascii_case(".print") || command.eq_ignore_ascii_case(".probe") {
                return Err(format!(
                    "wrapper-origin no-output DC contract does not cover {command} directives"
                ));
            }
            if command.eq_ignore_ascii_case(".dc") || command.eq_ignore_ascii_case(".op") {
                has_dc_or_op = true;
                continue;
            }
            if Self::is_extra_wrapper_output_analysis_command(command) {
                return Err(format!(
                    "wrapper-origin no-output DC contract does not cover {command} directives"
                ));
            }
        }

        if has_dc_or_op {
            Ok(())
        } else {
            Err("wrapper-origin no-output DC contract requires a .DC or .OP analysis".to_string())
        }
    }

    fn validate_default_prn_wrapper_source_with_format_mode(
        source: &str,
        allow_wrapper_probe_primary_prn: bool,
    ) -> Result<(), String> {
        let mut primary_print_count = 0usize;
        for line in Self::logical_netlist_lines(source) {
            let trimmed = Self::strip_netlist_comment(&line).trim().to_string();
            if trimmed.is_empty() {
                continue;
            }
            let Some(command) = trimmed.split_whitespace().next() else {
                continue;
            };
            if command.eq_ignore_ascii_case(".print") {
                let tokens = Self::split_print_fields(&trimmed)?;
                let token_refs = tokens.iter().map(String::as_str).collect::<Vec<_>>();
                if !Self::validate_default_prn_print_tokens(
                    &token_refs,
                    allow_wrapper_probe_primary_prn,
                )? {
                    primary_print_count += 1;
                }
                continue;
            }
            if Self::is_extra_wrapper_output_analysis_command(command) {
                return Err(format!(
                    "wrapper-origin default .prn contract does not cover {command} directives"
                ));
            }
        }

        match primary_print_count {
            1 => Ok(()),
            0 => Err(
                "wrapper-origin default .prn contract requires one primary .PRINT statement"
                    .to_string(),
            ),
            _ => Err(format!(
                "wrapper-origin default .prn contract requires one primary .PRINT statement, found {primary_print_count}"
            )),
        }
    }

    fn validate_default_prn_print_tokens(
        tokens: &[&str],
        allow_wrapper_probe_primary_prn: bool,
    ) -> Result<bool, String> {
        let Some(analysis) = tokens.get(1) else {
            return Err("wrapper-origin .PRINT statement has no analysis type".to_string());
        };
        if !analysis.eq_ignore_ascii_case("DC") {
            return Err(format!(
                "wrapper-origin default .prn contract only covers .PRINT DC, got .PRINT {analysis}"
            ));
        }

        let mut index = 2usize;
        let mut has_file_output = false;
        while index < tokens.len() {
            if let Some((raw_key, raw_value, consumed)) =
                Self::print_option_assignment(tokens, index)
            {
                let key = raw_key.trim().to_ascii_lowercase();
                let value = raw_value.trim().trim_matches(['"', '\'']);
                match key.as_str() {
                    "file" => {
                        has_file_output = true;
                    }
                    "format" if Self::dc_print_format_is_prn_compatible(value) => {}
                    "format"
                        if allow_wrapper_probe_primary_prn
                            && value.eq_ignore_ascii_case("PROBE") => {}
                    "format" => {
                        return Err(format!(
                            "wrapper-origin default .prn contract does not cover FORMAT={value}"
                        ));
                    }
                    _ => {}
                }
                index += consumed;
                continue;
            }
            index += 1;
        }

        Ok(has_file_output)
    }

    fn validate_csv_wrapper_source(source: &str) -> Result<(), String> {
        let mut primary_print_count = 0usize;
        for request in Self::dc_print_output_requests(source)? {
            let format = request.format.as_deref().unwrap_or("STD");
            if request.file.is_some() {
                return Err(format!(
                    "wrapper-origin CSV contract does not cover FILE= side output with FORMAT={format}"
                ));
            }
            if !format.eq_ignore_ascii_case("CSV") {
                return Err(format!(
                    "wrapper-origin CSV contract does not cover primary .PRINT DC FORMAT={format}"
                ));
            }
            primary_print_count += 1;
        }

        match primary_print_count {
            1 => Ok(()),
            0 => Err(
                "wrapper-origin CSV contract requires one primary .PRINT DC statement".to_string(),
            ),
            _ => Err(format!(
                "wrapper-origin CSV contract requires one primary .PRINT DC statement, found {primary_print_count}"
            )),
        }
    }

    fn validate_csd_wrapper_source(source: &str) -> Result<(), String> {
        let mut primary_print_count = 0usize;
        for request in Self::dc_print_output_requests(source)? {
            let format = request.format.as_deref().unwrap_or("STD");
            if request.file.is_some() {
                return Err(format!(
                    "wrapper-origin CSDF contract does not cover FILE= side output with FORMAT={format}"
                ));
            }
            if !format.eq_ignore_ascii_case("PROBE") {
                return Err(format!(
                    "wrapper-origin CSDF contract does not cover primary .PRINT DC FORMAT={format}"
                ));
            }
            primary_print_count += 1;
        }

        match primary_print_count {
            1 => Ok(()),
            0 => Err(
                "wrapper-origin CSDF contract requires one primary .PRINT DC statement".to_string(),
            ),
            _ => Err(format!(
                "wrapper-origin CSDF contract requires one primary .PRINT DC statement, found {primary_print_count}"
            )),
        }
    }

    fn validate_file_only_prn_wrapper_source(source: &str) -> Result<(), String> {
        let requests = Self::dc_print_output_requests(source)?;
        if requests.is_empty() {
            return Err(
                "wrapper-origin file-output contract requires at least one .PRINT DC statement"
                    .to_string(),
            );
        }

        for request in requests {
            let format = request.format.as_deref().unwrap_or("STD");
            if request.file.is_none() {
                return Err(format!(
                    "wrapper-origin file-output contract does not cover primary .PRINT DC FORMAT={format}"
                ));
            }
            if !Self::dc_print_format_is_prn_compatible(format) {
                return Err(format!(
                    "wrapper-origin file-output contract does not cover FILE= side output with FORMAT={format}"
                ));
            }
        }

        Ok(())
    }

    fn validate_raw_wrapper_source(source: &str) -> Result<(), String> {
        let mut primary_print_count = 0usize;
        for request in Self::dc_print_output_requests(source)? {
            let format = request.format.as_deref().unwrap_or("STD");
            if request.file.is_some() {
                return Err(format!(
                    "wrapper-origin RAW contract does not cover FILE= side output with FORMAT={format}"
                ));
            }
            if !format.eq_ignore_ascii_case("RAW") && !format.eq_ignore_ascii_case("STD") {
                return Err(format!(
                    "wrapper-origin RAW contract does not cover primary .PRINT DC FORMAT={format}"
                ));
            }
            primary_print_count += 1;
        }

        match primary_print_count {
            1 => Ok(()),
            0 => Err(
                "wrapper-origin RAW contract requires one primary .PRINT DC statement".to_string(),
            ),
            _ => Err(format!(
                "wrapper-origin RAW contract requires one primary .PRINT DC statement, found {primary_print_count}"
            )),
        }
    }

    fn validate_native_static_prn_tran_wrapper_contract(source: &str) -> Result<(), String> {
        Self::validate_native_static_prn_tran_wrapper_contract_with_format_mode(source, false)
    }

    fn validate_native_static_prn_tran_wrapper_contract_with_format_mode(
        source: &str,
        allow_wrapper_probe_primary_prn: bool,
    ) -> Result<(), String> {
        let mut primary_tran_print_count = 0usize;
        for line in Self::logical_netlist_lines(source) {
            let trimmed = Self::strip_netlist_comment(&line).trim().to_string();
            if trimmed.is_empty() {
                continue;
            }
            let Some(command) = trimmed.split_whitespace().next() else {
                continue;
            };
            if command.eq_ignore_ascii_case(".print") {
                let tokens = Self::split_print_fields(&trimmed)?;
                let token_refs = tokens.iter().map(String::as_str).collect::<Vec<_>>();
                let Some(analysis) = token_refs.get(1).copied() else {
                    return Err("wrapper-origin .PRINT statement has no analysis type".to_string());
                };
                if !analysis.eq_ignore_ascii_case("TRAN") {
                    return Err(format!(
                        "wrapper-origin transient .prn contract does not cover .PRINT {analysis}"
                    ));
                }
                let mut has_file_output = false;
                let mut has_probe = false;
                let mut index = 2usize;
                while index < token_refs.len() {
                    if let Some((raw_key, raw_value, consumed)) =
                        Self::print_option_assignment(&token_refs, index)
                    {
                        let value = raw_value.trim().trim_matches(['"', '\'']);
                        match raw_key.trim().to_ascii_lowercase().as_str() {
                            "file" => has_file_output = true,
                            "format" => {
                                if !Self::tran_print_format_is_prn_compatible(value)
                                    && !(allow_wrapper_probe_primary_prn
                                        && value.eq_ignore_ascii_case("PROBE"))
                                {
                                    return Err(format!(
                                        "wrapper-origin transient .prn contract does not cover .PRINT TRAN FORMAT={value}"
                                    ));
                                }
                            }
                            _ => {}
                        }
                        index += consumed;
                        continue;
                    }
                    let normalized = token_refs[index].to_ascii_lowercase();
                    if !Self::is_print_option_token(&normalized) {
                        has_probe = true;
                    }
                    index += 1;
                }
                if has_probe && !has_file_output {
                    primary_tran_print_count += 1;
                }
                continue;
            }
            if Self::is_extra_wrapper_tran_output_analysis_command(command) {
                return Err(format!(
                    "wrapper-origin transient .prn contract does not cover {command} directives"
                ));
            }
        }

        match primary_tran_print_count {
            1 => Ok(()),
            0 => Err(
                "wrapper-origin transient .prn contract requires one primary .PRINT TRAN statement"
                    .to_string(),
            ),
            _ => Err(format!(
                "wrapper-origin transient .prn contract requires one primary .PRINT TRAN statement, found {primary_tran_print_count}"
            )),
        }
    }

    fn validate_native_static_csv_tran_wrapper_contract(source: &str) -> Result<(), String> {
        let mut primary_tran_print_count = 0usize;
        for line in Self::logical_netlist_lines(source) {
            let trimmed = Self::strip_netlist_comment(&line).trim().to_string();
            if trimmed.is_empty() {
                continue;
            }
            let Some(command) = trimmed.split_whitespace().next() else {
                continue;
            };
            if command.eq_ignore_ascii_case(".print") {
                let tokens = Self::split_print_fields(&trimmed)?;
                let token_refs = tokens.iter().map(String::as_str).collect::<Vec<_>>();
                let Some(analysis) = token_refs.get(1).copied() else {
                    return Err("wrapper-origin .PRINT statement has no analysis type".to_string());
                };
                if !analysis.eq_ignore_ascii_case("TRAN") {
                    return Err(format!(
                        "wrapper-origin transient CSV contract does not cover .PRINT {analysis}"
                    ));
                }

                let mut format = None;
                let mut has_file_output = false;
                let mut index = 2usize;
                while index < token_refs.len() {
                    if let Some((raw_key, raw_value, consumed)) =
                        Self::print_option_assignment(&token_refs, index)
                    {
                        let value = raw_value.trim().trim_matches(['"', '\'']);
                        match raw_key.trim().to_ascii_lowercase().as_str() {
                            "file" => has_file_output = true,
                            "format" => format = Some(value),
                            _ => {}
                        }
                        index += consumed;
                        continue;
                    }
                    index += 1;
                }

                if has_file_output {
                    return Err(
                        "wrapper-origin transient CSV contract does not cover FILE= side outputs"
                            .to_string(),
                    );
                }
                match format {
                    Some(format) if format.eq_ignore_ascii_case("CSV") => {
                        primary_tran_print_count += 1;
                    }
                    Some(format) => {
                        return Err(format!(
                            "wrapper-origin transient CSV contract does not cover .PRINT TRAN FORMAT={format}"
                        ));
                    }
                    None => {
                        return Err(
                            "wrapper-origin transient CSV contract requires FORMAT=CSV".to_string()
                        );
                    }
                }
                continue;
            }
            if Self::is_extra_wrapper_tran_output_analysis_command(command) {
                return Err(format!(
                    "wrapper-origin transient CSV contract does not cover {command} directives"
                ));
            }
        }

        match primary_tran_print_count {
            1 => Ok(()),
            0 => Err(
                "wrapper-origin transient CSV contract requires one primary .PRINT TRAN statement"
                    .to_string(),
            ),
            _ => Err(format!(
                "wrapper-origin transient CSV contract requires one primary .PRINT TRAN statement, found {primary_tran_print_count}"
            )),
        }
    }

    fn validate_native_static_fd_ac_wrapper_contract(
        source: &str,
        output_override: bool,
    ) -> Result<(), String> {
        let mut primary_ac_print_count = 0usize;
        let mut side_ac_print_count = 0usize;
        let mut side_ac_print_formats = Vec::new();
        let mut primary_ac_ic_print_count = 0usize;
        let mut side_ac_ic_print_count = 0usize;
        let has_op_analysis = Self::source_has_op_analysis(source);
        for line in Self::logical_netlist_lines(source) {
            let trimmed = Self::strip_netlist_comment(&line).trim().to_string();
            if trimmed.is_empty() {
                continue;
            }
            let Some(command) = trimmed.split_whitespace().next() else {
                continue;
            };
            if command.eq_ignore_ascii_case(".print") {
                let tokens = Self::split_print_fields(&trimmed)?;
                let token_refs = tokens.iter().map(String::as_str).collect::<Vec<_>>();
                let Some(analysis) = token_refs.get(1).copied() else {
                    return Err("wrapper-origin .PRINT statement has no analysis type".to_string());
                };
                if !analysis.eq_ignore_ascii_case("AC") {
                    if analysis.eq_ignore_ascii_case("AC_IC") {
                        if !has_op_analysis {
                            continue;
                        }
                        let mut index = 2usize;
                        let mut has_file_output = false;
                        let mut print_format = "STD".to_string();
                        while index < token_refs.len() {
                            if let Some((raw_key, raw_value, consumed)) =
                                Self::print_option_assignment(&token_refs, index)
                            {
                                let value = raw_value.trim().trim_matches(['"', '\'']);
                                match raw_key.trim().to_ascii_lowercase().as_str() {
                                    "file" => has_file_output = true,
                                    "format" => {
                                        print_format = value.to_string();
                                        if !Self::ac_ic_print_format_is_supported(value) {
                                            return Err(format!(
                                                "wrapper-origin frequency-domain static output contract does not cover .PRINT AC_IC FORMAT={value}"
                                            ));
                                        }
                                    }
                                    _ => {}
                                }
                                index += consumed;
                                continue;
                            }
                            index += 1;
                        }
                        if has_file_output {
                            side_ac_ic_print_count += 1;
                            if !Self::ac_ic_print_format_is_supported(&print_format) {
                                return Err(format!(
                                    "wrapper-origin frequency-domain static output contract does not cover AC_IC FILE= side output FORMAT={print_format}"
                                ));
                            }
                        } else {
                            primary_ac_ic_print_count += 1;
                        }
                        continue;
                    }
                    return Err(format!(
                        "wrapper-origin frequency-domain static output contract does not cover .PRINT {analysis}"
                    ));
                }
                let mut index = 2usize;
                let mut has_file_output = false;
                let mut print_format = "STD".to_string();
                while index < token_refs.len() {
                    if let Some((raw_key, raw_value, consumed)) =
                        Self::print_option_assignment(&token_refs, index)
                    {
                        let value = raw_value.trim().trim_matches(['"', '\'']);
                        match raw_key.trim().to_ascii_lowercase().as_str() {
                            "file" => has_file_output = true,
                            "format" => {
                                print_format = value.to_string();
                                if !Self::ac_print_format_is_prn_compatible(value)
                                    && !value.eq_ignore_ascii_case("CSV")
                                    && !value.eq_ignore_ascii_case("PROBE")
                                {
                                    return Err(format!(
                                        "wrapper-origin frequency-domain static output contract does not cover .PRINT AC FORMAT={value}"
                                    ));
                                }
                            }
                            _ => {}
                        }
                        index += consumed;
                        continue;
                    }
                    index += 1;
                }
                if has_file_output {
                    side_ac_print_count += 1;
                    side_ac_print_formats.push(print_format);
                } else {
                    primary_ac_print_count += 1;
                }
                continue;
            }
            if Self::is_extra_wrapper_ac_output_analysis_command(command) {
                return Err(format!(
                    "wrapper-origin frequency-domain static output contract does not cover {command} directives"
                ));
            }
        }

        if output_override {
            if primary_ac_print_count == 0
                && side_ac_print_count == 0
                && primary_ac_ic_print_count == 0
                && side_ac_ic_print_count == 0
            {
                return Err(
                    "wrapper-origin frequency-domain output override contract requires one .PRINT AC or .PRINT AC_IC statement"
                        .to_string(),
                );
            }
            return Ok(());
        }

        if primary_ac_print_count == 0 {
            if side_ac_print_count > 1 {
                return Err(
                    "wrapper-origin frequency-domain static output contract does not cover multiple .PRINT AC FILE= outputs without a primary .PRINT AC"
                        .to_string(),
                );
            }
            if let Some(format) = side_ac_print_formats.first()
                && !Self::ac_print_format_is_prn_compatible(format)
                && !format.eq_ignore_ascii_case("CSV")
                && !format.eq_ignore_ascii_case("PROBE")
            {
                return Err(format!(
                    "wrapper-origin frequency-domain static output contract does not cover primary FILE= .PRINT AC FORMAT={format}"
                ));
            }
            if primary_ac_ic_print_count == 0
                && side_ac_ic_print_count == 0
                && side_ac_print_count == 0
            {
                return Err(
                    "wrapper-origin frequency-domain static output contract requires one primary .PRINT AC or .PRINT AC_IC statement"
                        .to_string(),
                );
            }
        } else {
            for format in side_ac_print_formats {
                if !Self::ac_print_format_is_prn_compatible(&format) {
                    return Err(format!(
                        "wrapper-origin frequency-domain static output contract does not cover FILE= side output FORMAT={format}"
                    ));
                }
            }
        }
        if primary_ac_ic_print_count == 0 && side_ac_ic_print_count > 1 {
            return Err(
                "wrapper-origin frequency-domain static output contract does not cover multiple .PRINT AC_IC FILE= outputs without a primary .PRINT AC_IC"
                    .to_string(),
            );
        }
        Ok(())
    }

    fn validate_native_static_csd_tran_wrapper_contract(source: &str) -> Result<(), String> {
        let mut primary_tran_print_count = 0usize;
        for line in Self::logical_netlist_lines(source) {
            let trimmed = Self::strip_netlist_comment(&line).trim().to_string();
            if trimmed.is_empty() {
                continue;
            }
            let Some(command) = trimmed.split_whitespace().next() else {
                continue;
            };
            if command.eq_ignore_ascii_case(".print") {
                let tokens = Self::split_print_fields(&trimmed)?;
                let token_refs = tokens.iter().map(String::as_str).collect::<Vec<_>>();
                let Some(analysis) = token_refs.get(1).copied() else {
                    return Err("wrapper-origin .PRINT statement has no analysis type".to_string());
                };
                if !analysis.eq_ignore_ascii_case("TRAN") {
                    return Err(format!(
                        "wrapper-origin transient CSDF contract does not cover .PRINT {analysis}"
                    ));
                }
                let mut format = None;
                let mut index = 2usize;
                while index < token_refs.len() {
                    if let Some((raw_key, raw_value, consumed)) =
                        Self::print_option_assignment(&token_refs, index)
                    {
                        let value = raw_value.trim().trim_matches(['"', '\'']);
                        match raw_key.trim().to_ascii_lowercase().as_str() {
                            "file" => {
                                return Err(
                                    "wrapper-origin transient CSDF contract does not cover FILE= side outputs"
                                        .to_string(),
                                );
                            }
                            "format" => format = Some(value),
                            _ => {}
                        }
                        index += consumed;
                        continue;
                    }
                    index += 1;
                }
                match format {
                    Some(format) if format.eq_ignore_ascii_case("PROBE") => {
                        primary_tran_print_count += 1;
                    }
                    Some(format) => {
                        return Err(format!(
                            "wrapper-origin transient CSDF contract does not cover .PRINT TRAN FORMAT={format}"
                        ));
                    }
                    None => {
                        return Err(
                            "wrapper-origin transient CSDF contract requires FORMAT=PROBE"
                                .to_string(),
                        );
                    }
                }
                continue;
            }
            if Self::is_extra_wrapper_tran_output_analysis_command(command) {
                return Err(format!(
                    "wrapper-origin transient CSDF contract does not cover {command} directives"
                ));
            }
        }

        match primary_tran_print_count {
            1 => Ok(()),
            0 => Err(
                "wrapper-origin transient CSDF contract requires one primary .PRINT TRAN statement"
                    .to_string(),
            ),
            _ => Err(format!(
                "wrapper-origin transient CSDF contract requires one primary .PRINT TRAN statement, found {primary_tran_print_count}"
            )),
        }
    }

    fn native_static_prn_tran_wrapper_contract(
        deck_path: &Path,
        relative_path: &str,
        source: &str,
        has_prn_oracle: bool,
    ) -> Option<XyceStaticTranContract> {
        if Self::validate_native_pwl_repeat_error_tran_wrapper_contract(deck_path, source).is_ok() {
            return Some(XyceStaticTranContract::WrapperStaticExpectedError);
        }

        if Self::is_native_csd_tran_wrapper_candidate(relative_path, source) {
            return Some(XyceStaticTranContract::WrapperCsd);
        }

        if Self::is_native_csv_tran_wrapper_candidate(relative_path, source) {
            return Some(XyceStaticTranContract::WrapperCsv);
        }

        if Self::is_native_default_prn_tran_wrapper_candidate(relative_path, source)
            || Self::is_native_output_other_prn_tran_wrapper_candidate(relative_path, source)
            || Self::is_native_output_initial_interval_tran_wrapper_candidate(source)
            || Self::is_native_generic_static_prn_tran_wrapper_candidate(
                relative_path,
                source,
                has_prn_oracle,
            )
        {
            return Some(XyceStaticTranContract::WrapperStatic);
        }

        None
    }

    fn native_output_override_prn_tran_wrapper_contract(
        source: &str,
    ) -> Result<XyceStaticTranContract, String> {
        Self::validate_native_output_override_prn_tran_wrapper_contract(source)?;
        Ok(XyceStaticTranContract::WrapperStatic)
    }

    fn is_native_csd_tran_wrapper_candidate(relative_path: &str, source: &str) -> bool {
        Self::normalize_manifest_key(relative_path).starts_with("netlists/output/tran/")
            && Self::validate_native_static_csd_tran_wrapper_contract(source).is_ok()
    }

    fn is_native_csv_tran_wrapper_candidate(relative_path: &str, source: &str) -> bool {
        Self::normalize_manifest_key(relative_path).starts_with("netlists/output/tran/")
            && Self::validate_native_static_csv_tran_wrapper_contract(source).is_ok()
    }

    fn is_native_default_prn_tran_wrapper_candidate(relative_path: &str, source: &str) -> bool {
        matches!(
            Self::normalize_manifest_key(relative_path).as_str(),
            "netlists/output/tran/op-prn.cir"
                | "netlists/output/tran/tran-gnuplot.cir"
                | "netlists/output/tran/tran-prn-comma.cir"
                | "netlists/output/tran/tran-prn.cir"
                | "netlists/output/tran/tran-prn-filter.cir"
                | "netlists/output/tran/tran-prn-noindex.cir"
                | "netlists/output/tran/tran-prn-precision.cir"
                | "netlists/output/tran/tran-prn-timescalefactor.cir"
                | "netlists/output/tran/tran-prn-width.cir"
                | "netlists/output/tran/tran-splot.cir"
                | "netlists/output/tran/tran-touchstone-defaults-to-prn.cir"
        ) && Self::validate_native_static_prn_tran_wrapper_contract(source).is_ok()
    }

    fn is_native_output_other_prn_tran_wrapper_candidate(
        relative_path: &str,
        source: &str,
    ) -> bool {
        Self::normalize_manifest_key(relative_path).starts_with("netlists/output/other/")
            && Self::validate_native_static_prn_tran_wrapper_contract(source).is_ok()
    }

    fn is_native_output_initial_interval_tran_wrapper_candidate(source: &str) -> bool {
        Self::validate_native_output_initial_interval_tran_wrapper_contract(source).is_ok()
    }

    fn is_native_generic_static_prn_tran_wrapper_candidate(
        relative_path: &str,
        source: &str,
        has_prn_oracle: bool,
    ) -> bool {
        let normalized_path = Self::normalize_manifest_key(relative_path);
        if normalized_path.starts_with("netlists/output/") {
            return false;
        }
        if Self::source_enables_constant_time_step_output(source) {
            return false;
        }
        Self::validate_native_static_prn_tran_wrapper_contract(source).is_ok()
            || (has_prn_oracle
                && Self::validate_native_static_prn_tran_wrapper_contract_with_format_mode(
                    source, true,
                )
                .is_ok())
    }

    fn source_enables_constant_time_step_output(source: &str) -> bool {
        Self::logical_netlist_lines(source).iter().any(|line| {
            let normalized = Self::strip_netlist_comment(line)
                .chars()
                .filter(|ch| !ch.is_whitespace())
                .collect::<String>()
                .to_ascii_lowercase();
            normalized.starts_with(".options")
                && normalized.contains("timeint")
                && normalized.contains("conststep")
        })
    }

    fn source_requests_ac_phase_output_radians(source: &str) -> bool {
        let mut enabled = false;
        for line in Self::logical_netlist_lines(source) {
            let normalized = Self::strip_netlist_comment(&line)
                .chars()
                .filter(|ch| !ch.is_whitespace())
                .collect::<String>()
                .to_ascii_lowercase();
            if !normalized.starts_with(".options") || !normalized.contains("phase_output_radians") {
                continue;
            }
            if normalized.contains("phase_output_radians=false")
                || normalized.contains("phase_output_radians=0")
                || normalized.contains("phase_output_radians=no")
            {
                enabled = false;
                continue;
            }
            enabled = true;
        }
        enabled
    }

    fn static_tran_contract_for_print_format(
        requires_wrapper: bool,
        format: Option<&str>,
    ) -> Result<XyceStaticTranContract, String> {
        let normalized = format.unwrap_or("STD").trim();
        if Self::tran_print_format_is_prn_compatible(normalized) {
            return Ok(if requires_wrapper {
                XyceStaticTranContract::WrapperStatic
            } else {
                XyceStaticTranContract::PlainStatic
            });
        }
        if normalized.eq_ignore_ascii_case("CSV") {
            return Ok(if requires_wrapper {
                XyceStaticTranContract::WrapperCsv
            } else {
                XyceStaticTranContract::PlainCsv
            });
        }
        if normalized.eq_ignore_ascii_case("PROBE") {
            return Ok(if requires_wrapper {
                XyceStaticTranContract::WrapperCsd
            } else {
                XyceStaticTranContract::PlainCsd
            });
        }
        Err(format!(
            "native static .PRINT TRAN comparison does not cover FORMAT={normalized}"
        ))
    }

    fn validate_native_output_initial_interval_tran_wrapper_contract(
        source: &str,
    ) -> Result<(), String> {
        Self::validate_native_static_prn_tran_wrapper_contract(source)?;

        let tran_prints = Self::print_output_requests(source, "TRAN")?;
        if tran_prints.iter().any(|request| request.file.is_some()) {
            return Err(
                "wrapper-origin initial-interval transient .prn contract does not cover FILE= side outputs"
                    .to_string(),
            );
        }
        let Some(primary_print) = tran_prints.iter().find(|request| request.file.is_none()) else {
            return Err(
                "wrapper-origin initial-interval transient .prn contract requires one primary .PRINT TRAN statement"
                    .to_string(),
            );
        };
        if primary_print.probes.len() != 1 {
            return Err(format!(
                "wrapper-origin initial-interval transient .prn contract currently covers one primary probe, found {}",
                primary_print.probes.len()
            ));
        }

        let mut initial_interval_options = 0usize;
        for line in Self::logical_netlist_lines(source) {
            let normalized = Self::strip_netlist_comment(&line)
                .trim()
                .to_ascii_lowercase();
            if normalized.is_empty() {
                continue;
            }
            let Some(command) = normalized.split_whitespace().next() else {
                continue;
            };
            if !command.eq_ignore_ascii_case(".options") {
                continue;
            }
            if normalized.contains("output") && normalized.contains("initial_interval") {
                initial_interval_options += 1;
            } else {
                return Err(format!(
                    "wrapper-origin initial-interval transient .prn contract does not cover {command} directives without OUTPUT INITIAL_INTERVAL"
                ));
            }
        }

        match initial_interval_options {
            1 => Ok(()),
            0 => Err(
                "wrapper-origin initial-interval transient .prn contract requires .OPTIONS OUTPUT INITIAL_INTERVAL"
                    .to_string(),
            ),
            _ => Err(format!(
                "wrapper-origin initial-interval transient .prn contract requires one .OPTIONS OUTPUT INITIAL_INTERVAL directive, found {initial_interval_options}"
            )),
        }
    }

    fn validate_native_output_override_prn_tran_wrapper_contract(
        source: &str,
    ) -> Result<(), String> {
        let mut print_count = 0usize;
        let mut probe_count = 0usize;
        for line in Self::logical_netlist_lines(source) {
            let trimmed = Self::strip_netlist_comment(&line).trim().to_string();
            if trimmed.is_empty() {
                continue;
            }
            let Some(command) = trimmed.split_whitespace().next() else {
                continue;
            };
            if command.eq_ignore_ascii_case(".print") {
                let tokens = Self::split_print_fields(&trimmed)?;
                let token_refs = tokens.iter().map(String::as_str).collect::<Vec<_>>();
                let Some(analysis) = token_refs.get(1).copied() else {
                    return Err("output override .PRINT statement has no analysis type".to_string());
                };
                if !analysis.eq_ignore_ascii_case("TRAN") {
                    return Err(format!(
                        "wrapper-origin transient output override contract does not cover .PRINT {analysis}"
                    ));
                }

                print_count += 1;
                let mut index = 2usize;
                while index < token_refs.len() {
                    if let Some((raw_key, raw_value, consumed)) =
                        Self::print_option_assignment(&token_refs, index)
                    {
                        let value = raw_value.trim().trim_matches(['"', '\'']);
                        if raw_key.trim().eq_ignore_ascii_case("FORMAT") {
                            Self::static_tran_contract_for_print_format(true, Some(value))
                                .map_err(|err| {
                                    format!(
                                        "wrapper-origin transient output override contract does not cover {err}"
                                    )
                                })?;
                        }
                        index += consumed;
                        continue;
                    }
                    let normalized = token_refs[index].to_ascii_lowercase();
                    if !Self::is_print_option_token(&normalized) {
                        probe_count += 1;
                    }
                    index += 1;
                }
                continue;
            }
            if Self::is_extra_wrapper_tran_output_analysis_command(command) {
                return Err(format!(
                    "wrapper-origin transient output override contract does not cover {command} directives"
                ));
            }
        }

        if print_count == 0 || probe_count == 0 {
            return Err(
                "wrapper-origin transient output override contract requires .PRINT TRAN probes"
                    .to_string(),
            );
        }
        Ok(())
    }

    fn validate_native_pwl_repeat_error_tran_wrapper_contract(
        deck_path: &Path,
        source: &str,
    ) -> Result<(), String> {
        Self::validate_native_static_prn_tran_wrapper_contract(source)?;
        if !Self::source_may_have_pwl_repeat_option(source) {
            return Err(
                "wrapper-origin PWL repeat error contract requires a primary PWL repeat deck"
                    .to_string(),
            );
        }

        let stem = deck_path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .ok_or_else(|| {
                "wrapper-origin PWL repeat error contract requires a .cir filename".to_string()
            })?;
        let sibling_path = deck_path.with_file_name(format!("{stem}RepeatFail.cir"));
        let sibling_source = fs::read_to_string(&sibling_path).map_err(|err| {
            format!(
                "wrapper-origin PWL repeat error contract requires sibling expected-error deck '{}': {err}",
                sibling_path.display()
            )
        })?;

        Self::validate_expected_pwl_repeat_value_error_source(&sibling_source, &sibling_path)
    }

    fn is_expected_missing_inductor_value_error_deck(relative_path: &str, source: &str) -> bool {
        matches!(
            Self::normalize_manifest_key(relative_path).as_str(),
            "netlists/inductor/errormessagetest.cir"
        ) && source
            .to_ascii_lowercase()
            .contains("l value missing from instance line")
    }

    fn validate_expected_missing_inductor_value_error_source(
        source: &str,
        deck_path: &Path,
    ) -> Result<(), String> {
        match Self::parse_xyce_netlist(source, deck_path) {
            Ok(_) => Err(
                "expected missing-inductor-value deck parsed successfully; expected a fatal parser diagnostic"
                    .to_string(),
            ),
            Err(err) => {
                let message = err.to_string();
                if message.contains("Inductor requires either a value or a model") {
                    Ok(())
                } else {
                    Err(format!(
                        "expected missing-inductor-value diagnostic, got parser error: {message}"
                    ))
                }
            }
        }
    }

    fn validate_expected_pwl_repeat_value_error_source(
        source: &str,
        deck_path: &Path,
    ) -> Result<(), String> {
        if !Self::source_may_have_pwl_repeat_option(source) {
            return Err(
                "expected PWL repeat-value error deck has no PWL repeat option".to_string(),
            );
        }

        match Self::parse_xyce_netlist(source, deck_path) {
            Ok(_) => Err(format!(
                "expected {XYCE_PWL_REPEAT_VALUE_ERROR}, but deck parsed successfully"
            )),
            Err(err) => {
                let message = err.to_string();
                if message.contains(XYCE_PWL_REPEAT_VALUE_ERROR) {
                    Ok(())
                } else {
                    Err(format!(
                        "expected {XYCE_PWL_REPEAT_VALUE_ERROR}, got {message}"
                    ))
                }
            }
        }
    }

    fn source_may_have_pwl_repeat_option(source: &str) -> bool {
        Self::logical_netlist_lines(source).iter().any(|line| {
            let compact = Self::strip_netlist_comment(line)
                .chars()
                .filter(|ch| !ch.is_whitespace())
                .collect::<String>()
                .to_ascii_lowercase();
            compact.contains("pwl") && (compact.contains("r=") || compact.contains("repeat="))
        })
    }

    fn native_default_prn_tran_wrapper_tolerance(
        relative_path: &str,
    ) -> Option<XyceComparisonTolerance> {
        match Self::normalize_manifest_key(relative_path).as_str() {
            "netlists/output/tran/tran-prn-noindex.cir"
            | "netlists/output/tran/tran-step-gnuplot.cir"
            | "netlists/output/tran/tran-stepnum-col.cir" => Some(XyceComparisonTolerance {
                relative: 1.0e-3,
                absolute: 1.0e-5,
                zero: Some(1.0e-8),
            }),
            _ => None,
        }
    }

    fn static_ac_contract_for_print_format(
        requires_wrapper: bool,
        format: Option<&str>,
    ) -> Result<XyceStaticAcContract, String> {
        let normalized = format.unwrap_or("STD").trim();
        if Self::ac_print_format_is_prn_compatible(normalized) {
            return Ok(if requires_wrapper {
                XyceStaticAcContract::WrapperStatic
            } else {
                XyceStaticAcContract::PlainStatic
            });
        }
        if normalized.eq_ignore_ascii_case("CSV") {
            return Ok(if requires_wrapper {
                XyceStaticAcContract::WrapperCsv
            } else {
                XyceStaticAcContract::PlainCsv
            });
        }
        if normalized.eq_ignore_ascii_case("PROBE") {
            return Ok(if requires_wrapper {
                XyceStaticAcContract::WrapperCsd
            } else {
                XyceStaticAcContract::PlainCsd
            });
        }
        Err(format!(
            "native static .PRINT AC comparison does not cover FORMAT={normalized}"
        ))
    }

    fn static_ac_ic_contract_for_print_format(
        requires_wrapper: bool,
        format: Option<&str>,
    ) -> Result<XyceStaticAcContract, String> {
        let normalized = format.unwrap_or("STD").trim();
        if Self::ac_print_format_is_prn_compatible(normalized) {
            return Ok(if requires_wrapper {
                XyceStaticAcContract::WrapperIcPrn
            } else {
                XyceStaticAcContract::PlainIcPrn
            });
        }
        if normalized.eq_ignore_ascii_case("CSV") {
            return Ok(if requires_wrapper {
                XyceStaticAcContract::WrapperIcCsv
            } else {
                XyceStaticAcContract::PlainIcCsv
            });
        }
        if normalized.eq_ignore_ascii_case("PROBE") {
            return Ok(if requires_wrapper {
                XyceStaticAcContract::WrapperIcCsd
            } else {
                XyceStaticAcContract::PlainIcCsd
            });
        }
        Err(format!(
            "native static .PRINT AC_IC comparison does not cover FORMAT={normalized}"
        ))
    }

    fn static_dc_contract_for_print_format(
        requires_wrapper: bool,
        format: Option<&str>,
    ) -> Result<XyceStaticDcContract, String> {
        let normalized = format.unwrap_or("STD").trim();
        if Self::dc_print_format_is_prn_compatible(normalized) {
            return Ok(if requires_wrapper {
                XyceStaticDcContract::WrapperDefault
            } else {
                XyceStaticDcContract::PlainStatic
            });
        }
        if normalized.eq_ignore_ascii_case("CSV") {
            return Ok(if requires_wrapper {
                XyceStaticDcContract::WrapperCsv
            } else {
                XyceStaticDcContract::PlainCsv
            });
        }
        if normalized.eq_ignore_ascii_case("PROBE") {
            return Ok(if requires_wrapper {
                XyceStaticDcContract::WrapperCsd
            } else {
                XyceStaticDcContract::PlainCsd
            });
        }
        Err(format!(
            "native static .PRINT DC comparison does not cover FORMAT={normalized}"
        ))
    }

    fn dc_print_format_is_prn_compatible(format: &str) -> bool {
        matches!(
            format.to_ascii_lowercase().as_str(),
            "std" | "tecplot" | "touchstone" | "touchstone2" | "noindex" | "gnuplot" | "splot"
        )
    }

    fn tran_print_format_is_prn_compatible(format: &str) -> bool {
        matches!(
            format.to_ascii_lowercase().as_str(),
            "std" | "tecplot" | "touchstone" | "touchstone2" | "noindex" | "gnuplot" | "splot"
        )
    }

    fn ac_print_format_is_prn_compatible(format: &str) -> bool {
        matches!(
            format.to_ascii_lowercase().as_str(),
            "std" | "tecplot" | "touchstone" | "touchstone2" | "noindex" | "gnuplot" | "splot"
        )
    }

    fn ac_ic_print_format_is_supported(format: &str) -> bool {
        Self::ac_print_format_is_prn_compatible(format)
            || format.eq_ignore_ascii_case("CSV")
            || format.eq_ignore_ascii_case("PROBE")
    }

    fn ac_initial_condition_reference_extension(
        format: Option<&str>,
    ) -> Result<&'static str, String> {
        let normalized = format.unwrap_or("STD").trim();
        if Self::ac_print_format_is_prn_compatible(normalized) {
            return Ok("TD.prn");
        }
        if normalized.eq_ignore_ascii_case("CSV") {
            return Ok("TD.csv");
        }
        if normalized.eq_ignore_ascii_case("PROBE") {
            return Ok("TD.csd");
        }
        Err(format!(
            "native AC_IC comparison does not cover FORMAT={normalized}"
        ))
    }

    fn is_extra_wrapper_ac_output_analysis_command(command: &str) -> bool {
        matches!(
            command.to_ascii_lowercase().as_str(),
            ".dc"
                | ".four"
                | ".fft"
                | ".hb"
                | ".measure"
                | ".meas"
                | ".noise"
                | ".probe"
                | ".save"
                | ".sens"
                | ".tran"
        )
    }

    fn is_extra_wrapper_tran_output_analysis_command(command: &str) -> bool {
        matches!(
            command.to_ascii_lowercase().as_str(),
            ".ac"
                | ".dc"
                | ".four"
                | ".fft"
                | ".hb"
                | ".measure"
                | ".meas"
                | ".noise"
                | ".probe"
                | ".save"
                | ".sens"
        )
    }

    fn print_option_assignment<'a>(
        tokens: &'a [&'a str],
        index: usize,
    ) -> Option<(&'a str, &'a str, usize)> {
        let token = tokens.get(index).copied()?;
        if let Some((key, value)) = token.split_once('=') {
            if value.is_empty() && token.ends_with('=') {
                return Some((key, tokens.get(index + 1).copied()?, 2));
            }
            return Some((key, value, 1));
        }
        if token.ends_with('=') {
            return Some((
                token.trim_end_matches('='),
                tokens.get(index + 1).copied()?,
                2,
            ));
        }
        if tokens.get(index + 1).copied() == Some("=") {
            return Some((token, tokens.get(index + 2).copied()?, 3));
        }
        None
    }

    fn is_extra_wrapper_output_analysis_command(command: &str) -> bool {
        matches!(
            command.to_ascii_lowercase().as_str(),
            ".ac"
                | ".four"
                | ".fft"
                | ".hb"
                | ".measure"
                | ".meas"
                | ".noise"
                | ".probe"
                | ".save"
                | ".sens"
                | ".tran"
        )
    }

    fn wrapper_source_has_extra_output_analysis(source: &str) -> bool {
        Self::logical_netlist_lines(source).into_iter().any(|line| {
            let trimmed = Self::strip_netlist_comment(&line).trim().to_string();
            let Some(command) = trimmed.split_whitespace().next() else {
                return false;
            };
            Self::is_extra_wrapper_output_analysis_command(command)
        })
    }

    fn run_static_fd_prn_ac_plan(
        &self,
        deck: &XyceDeck,
        plan: XyceStaticAcPlan,
        start: Instant,
    ) -> XyceTestResult {
        let contract = plan.contract.result_contract(!plan.steps.is_empty());
        let frequencies = plan.ac.frequencies();
        if frequencies.is_empty() {
            return self.failure_result(
                deck,
                start,
                contract,
                "AC analysis produced no frequency points".to_string(),
                Vec::new(),
            );
        }

        let frequency_bound_source;
        let parse_source = if plan.frequency_bound {
            frequency_bound_source =
                Self::source_with_ac_frequency_bindings(&plan.source, frequencies[0]);
            frequency_bound_source.as_str()
        } else {
            plan.source.as_str()
        };
        let netlist = match Self::parse_xyce_netlist(parse_source, &plan.deck_path) {
            Ok(netlist) => netlist,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("parse failed after contract validation: {err}"),
                    Vec::new(),
                );
            }
        };

        let Some(primary_reference_path) = plan.reference_path.as_ref() else {
            let ac_ic_mismatches = match self.compare_ac_initial_condition_outputs(&plan, &netlist)
            {
                Ok(mismatches) => mismatches,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("AC initial-condition output comparison error: {err}"),
                        Vec::new(),
                    );
                }
            };
            return if ac_ic_mismatches.is_empty() {
                self.passed_result(deck, start, contract)
            } else {
                self.failure_result(
                    deck,
                    start,
                    contract,
                    format!(
                        "{} Xyce AC initial-condition output mismatch(es)",
                        ac_ic_mismatches.len()
                    ),
                    ac_ic_mismatches,
                )
            };
        };
        let Some(primary_print) = plan.print.as_ref() else {
            return self.failure_result(
                deck,
                start,
                contract,
                "AC plan has a primary reference without a primary print request".to_string(),
                Vec::new(),
            );
        };
        let reference = match Self::parse_ac_reference_file(plan.contract, primary_reference_path) {
            Ok(reference) => reference,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("failed to parse Xyce AC oracle: {err}"),
                    Vec::new(),
                );
            }
        };

        if !plan.steps.is_empty() {
            return self.run_static_step_ac_plan(
                deck,
                plan,
                netlist,
                reference,
                frequencies,
                start,
            );
        }
        if plan.ac.data_points().is_some() {
            return self.run_static_ac_data_plan(deck, plan, netlist, reference, start);
        }
        if plan.frequency_bound {
            return self.run_static_frequency_bound_ac_plan(
                deck,
                plan,
                reference,
                frequencies,
                start,
            );
        }

        let engine = self.create_xyce_engine();
        let results = match engine.run_ac(&netlist, &frequencies) {
            Ok(results) => results,
            Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                return self.expected_unsupported_result(
                    deck,
                    start,
                    "unsupported_xyce_runtime",
                    &format!("RSpice runtime does not yet support this static AC deck: {err}"),
                );
            }
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("simulation error: {err}"),
                    Vec::new(),
                );
            }
        };

        let mismatches = match self.compare_ac_prn_reference(
            &reference,
            primary_print,
            &netlist,
            &plan.source,
            &results,
        ) {
            Ok(mismatches) => mismatches,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("reference comparison error: {err}"),
                    Vec::new(),
                );
            }
        };
        if mismatches.is_empty() {
            let side_mismatches = match self.compare_ac_side_outputs(&plan, &netlist, &results) {
                Ok(mismatches) => mismatches,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("AC side-output comparison error: {err}"),
                        Vec::new(),
                    );
                }
            };
            if side_mismatches.is_empty() {
                let ac_ic_mismatches =
                    match self.compare_ac_initial_condition_outputs(&plan, &netlist) {
                        Ok(mismatches) => mismatches,
                        Err(err) => {
                            return self.failure_result(
                                deck,
                                start,
                                contract,
                                format!("AC initial-condition output comparison error: {err}"),
                                Vec::new(),
                            );
                        }
                    };
                if ac_ic_mismatches.is_empty() {
                    self.passed_result(deck, start, contract)
                } else {
                    self.failure_result(
                        deck,
                        start,
                        contract,
                        format!(
                            "{} Xyce AC initial-condition output mismatch(es)",
                            ac_ic_mismatches.len()
                        ),
                        ac_ic_mismatches,
                    )
                }
            } else {
                self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("{} Xyce AC side-output mismatch(es)", side_mismatches.len()),
                    side_mismatches,
                )
            }
        } else {
            self.failure_result(
                deck,
                start,
                contract,
                format!("{} Xyce AC reference mismatch(es)", mismatches.len()),
                mismatches,
            )
        }
    }

    fn run_static_frequency_bound_ac_plan(
        &self,
        deck: &XyceDeck,
        plan: XyceStaticAcPlan,
        reference: XycePrnTable,
        frequencies: Vec<Value>,
        start: Instant,
    ) -> XyceTestResult {
        let contract = plan.contract.result_contract(false);
        let Some(primary_print) = plan.print.as_ref() else {
            return self.failure_result(
                deck,
                start,
                contract,
                "frequency-bound AC comparison requires a primary .PRINT AC request".to_string(),
                Vec::new(),
            );
        };

        let engine = self.create_xyce_engine();
        let mut point_results = Vec::with_capacity(frequencies.len());
        for (row_index, frequency) in frequencies.iter().copied().enumerate() {
            let point_source = Self::source_with_ac_frequency_bindings(&plan.source, frequency);
            let point_netlist = match Self::parse_xyce_netlist(&point_source, &plan.deck_path) {
                Ok(netlist) => netlist,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!(
                            "frequency-bound AC row {} parse error at FREQ={frequency}: {err}",
                            row_index + 1
                        ),
                        Vec::new(),
                    );
                }
            };
            let mut results = match engine.run_ac(&point_netlist, &[frequency]) {
                Ok(results) => results,
                Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                    return self.expected_unsupported_result(
                        deck,
                        start,
                        "unsupported_xyce_runtime",
                        &format!(
                            "RSpice runtime does not yet support this frequency-bound AC deck: {err}"
                        ),
                    );
                }
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!(
                            "simulation error in frequency-bound AC row {}: {err}",
                            row_index + 1
                        ),
                        Vec::new(),
                    );
                }
            };
            let Some(result) = results.pop() else {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!(
                        "frequency-bound AC row {} produced no AC result",
                        row_index + 1
                    ),
                    Vec::new(),
                );
            };
            point_results.push(XyceAcDataPointResult {
                netlist: point_netlist,
                result,
            });
        }

        let mismatches = match self.compare_ac_data_prn_reference(
            &reference,
            primary_print,
            &plan.source,
            &point_results,
        ) {
            Ok(mismatches) => mismatches,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("reference comparison error: {err}"),
                    Vec::new(),
                );
            }
        };
        if mismatches.is_empty() {
            let side_mismatches = match self.compare_ac_data_side_outputs(&plan, &point_results) {
                Ok(mismatches) => mismatches,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("AC side-output comparison error: {err}"),
                        Vec::new(),
                    );
                }
            };
            if side_mismatches.is_empty() {
                self.passed_result(deck, start, contract)
            } else {
                self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("{} Xyce AC side-output mismatch(es)", side_mismatches.len()),
                    side_mismatches,
                )
            }
        } else {
            self.failure_result(
                deck,
                start,
                contract,
                format!("{} Xyce AC reference mismatch(es)", mismatches.len()),
                mismatches,
            )
        }
    }

    fn run_static_ac_data_plan(
        &self,
        deck: &XyceDeck,
        plan: XyceStaticAcPlan,
        netlist: Netlist,
        reference: XycePrnTable,
        start: Instant,
    ) -> XyceTestResult {
        let contract = plan.contract.result_contract(false);
        let Some(primary_print) = plan.print.as_ref() else {
            return self.failure_result(
                deck,
                start,
                contract,
                ".AC DATA comparison requires a primary .PRINT AC request".to_string(),
                Vec::new(),
            );
        };
        let Some(data_points) = plan.ac.data_points() else {
            return self.failure_result(
                deck,
                start,
                contract,
                ".AC DATA comparison has no data rows".to_string(),
                Vec::new(),
            );
        };

        let engine = self.create_xyce_engine();
        let mut point_results = Vec::with_capacity(data_points.len());
        for (row_index, point) in data_points.iter().enumerate() {
            let row_netlist =
                match Engine::create_perturbed_netlist_multi(&netlist, &point.overrides) {
                    Ok((row_netlist, _)) => row_netlist,
                    Err(err) => {
                        return self.failure_result(
                            deck,
                            start,
                            contract,
                            format!(
                                ".AC DATA row {} parameter override error: {err}",
                                row_index + 1
                            ),
                            Vec::new(),
                        );
                    }
                };
            let mut results = match engine.run_ac(&row_netlist, &[point.frequency]) {
                Ok(results) => results,
                Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                    return self.expected_unsupported_result(
                        deck,
                        start,
                        "unsupported_xyce_runtime",
                        &format!("RSpice runtime does not yet support this .AC DATA deck: {err}"),
                    );
                }
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("simulation error in .AC DATA row {}: {err}", row_index + 1),
                        Vec::new(),
                    );
                }
            };
            let Some(result) = results.pop() else {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!(".AC DATA row {} produced no AC result", row_index + 1),
                    Vec::new(),
                );
            };
            point_results.push(XyceAcDataPointResult {
                netlist: row_netlist,
                result,
            });
        }

        let mismatches = match self.compare_ac_data_prn_reference(
            &reference,
            primary_print,
            &plan.source,
            &point_results,
        ) {
            Ok(mismatches) => mismatches,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("reference comparison error: {err}"),
                    Vec::new(),
                );
            }
        };
        if mismatches.is_empty() {
            let side_mismatches = match self.compare_ac_data_side_outputs(&plan, &point_results) {
                Ok(mismatches) => mismatches,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("AC side-output comparison error: {err}"),
                        Vec::new(),
                    );
                }
            };
            if side_mismatches.is_empty() {
                self.passed_result(deck, start, contract)
            } else {
                self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("{} Xyce AC side-output mismatch(es)", side_mismatches.len()),
                    side_mismatches,
                )
            }
        } else {
            self.failure_result(
                deck,
                start,
                contract,
                format!("{} Xyce AC reference mismatch(es)", mismatches.len()),
                mismatches,
            )
        }
    }

    fn run_static_step_ac_plan(
        &self,
        deck: &XyceDeck,
        plan: XyceStaticAcPlan,
        netlist: Netlist,
        reference: XycePrnTable,
        frequencies: Vec<Value>,
        start: Instant,
    ) -> XyceTestResult {
        let contract = plan.contract.result_contract(true);
        let Some(primary_print) = plan.print.as_ref() else {
            return self.failure_result(
                deck,
                start,
                contract,
                ".STEP AC comparison requires a primary .PRINT AC request".to_string(),
                Vec::new(),
            );
        };
        let expansion_engine = self.create_xyce_engine();
        let step_runs =
            match Self::nested_step_runs_for_commands(&expansion_engine, &netlist, &plan.steps) {
                Ok(runs) => runs,
                Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                    return self.expected_unsupported_result(
                        deck,
                        start,
                        "unsupported_xyce_runtime",
                        &format!("RSpice runtime does not yet support this .STEP AC deck: {err}"),
                    );
                }
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!(".STEP expansion error: {err}"),
                        Vec::new(),
                    );
                }
            };

        let engine = self.create_xyce_engine();
        let mut batches = Vec::with_capacity(step_runs.len());
        for (step_index, run) in step_runs.iter().enumerate() {
            let results = match engine.run_ac(&run.netlist, &frequencies) {
                Ok(results) => results,
                Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                    return self.expected_unsupported_result(
                        deck,
                        start,
                        "unsupported_xyce_runtime",
                        &format!("RSpice runtime does not yet support this .STEP AC deck: {err}"),
                    );
                }
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("simulation error in AC step {}: {err}", step_index + 1),
                        Vec::new(),
                    );
                }
            };
            batches.push(XyceAcResultBatch {
                netlist: run.netlist.clone(),
                results,
            });
        }

        let mismatches = match self.compare_step_ac_reference_batches(
            &reference,
            primary_print,
            &plan.source,
            frequencies.len(),
            &batches,
        ) {
            Ok(mismatches) => mismatches,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("reference comparison error: {err}"),
                    Vec::new(),
                );
            }
        };

        if mismatches.is_empty() {
            let side_mismatches =
                match self.compare_step_ac_side_outputs(&plan, frequencies.len(), &batches) {
                    Ok(mismatches) => mismatches,
                    Err(err) => {
                        return self.failure_result(
                            deck,
                            start,
                            contract,
                            format!("AC side-output comparison error: {err}"),
                            Vec::new(),
                        );
                    }
                };
            if side_mismatches.is_empty() {
                self.passed_result(deck, start, contract)
            } else {
                self.failure_result(
                    deck,
                    start,
                    contract,
                    format!(
                        "{} Xyce stepped AC side-output mismatch(es)",
                        side_mismatches.len()
                    ),
                    side_mismatches,
                )
            }
        } else {
            self.failure_result(
                deck,
                start,
                contract,
                format!(
                    "{} Xyce stepped AC reference mismatch(es)",
                    mismatches.len()
                ),
                mismatches,
            )
        }
    }

    fn compare_ac_initial_condition_outputs(
        &self,
        plan: &XyceStaticAcPlan,
        netlist: &Netlist,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        if !Self::source_has_op_analysis(&plan.source) {
            return Ok(Vec::new());
        }
        let requests = if plan.output_override {
            Self::output_override_print_output_request(&plan.source, "AC_IC")?
                .into_iter()
                .collect::<Vec<_>>()
        } else {
            Self::aggregate_print_output_requests(
                Self::print_output_requests(&plan.source, "AC_IC")?,
                "AC_IC",
            )?
        };
        if requests.is_empty() {
            return Ok(Vec::new());
        }

        let dc = Self::synthetic_op_dc_sweep(netlist)?;
        let (result, device_op_report) = self
            .create_dc_engine()
            .run_dc_op_with_report(netlist)
            .map_err(|err| format!(".OP solve for AC_IC output failed: {err}"))?;
        let op_result = DcSweepPointResult {
            sweep_value: dc.start,
            result,
            device_op_report,
        };

        let mut all_mismatches = Vec::new();
        for request in requests {
            let reference_path = self.ac_initial_condition_reference_path(plan, &request)?;
            let reference =
                Self::parse_ac_initial_condition_reference_file(&request, &reference_path)
                    .map_err(|err| {
                        format!(
                            "failed to parse AC_IC oracle {}: {err}",
                            self.display_path(&reference_path)
                        )
                    })?;
            let print = XycePrintRequest {
                probes: request.probes.clone(),
            };
            let mut mismatches = self.compare_ac_initial_condition_reference(
                &reference,
                &print,
                netlist,
                &plan.source,
                &dc,
                &op_result,
            )?;
            if let Some(file) = request.file.as_deref()
                && Some(file) != plan.primary_ac_ic_file.as_deref()
            {
                for mismatch in &mut mismatches {
                    mismatch.probe = format!("{file}:{}", mismatch.probe);
                }
            }
            all_mismatches.extend(mismatches);
            if all_mismatches.len() >= self.config.max_mismatches {
                all_mismatches.truncate(self.config.max_mismatches);
                break;
            }
        }
        Ok(all_mismatches)
    }

    fn ac_initial_condition_reference_path(
        &self,
        plan: &XyceStaticAcPlan,
        request: &XycePrintOutputRequest,
    ) -> Result<PathBuf, String> {
        if request.file.as_deref() == plan.primary_ac_ic_file.as_deref() {
            let extension =
                Self::ac_initial_condition_reference_extension(request.format.as_deref())?;
            let path = self
                .static_output_reference_path(&plan.deck_path, extension)
                .ok_or_else(|| "deck is not under tests/xyce/Netlists".to_string())?;
            if !path.is_file() {
                return Err(format!(
                    "missing checked-in AC_IC oracle {}",
                    self.display_path(&path)
                ));
            }
            return Ok(path);
        }
        if let Some(file) = request.file.as_deref() {
            return self.side_output_reference_path_for_deck(&plan.deck_path, file);
        }

        let extension = Self::ac_initial_condition_reference_extension(request.format.as_deref())?;
        let path = self
            .static_output_reference_path(&plan.deck_path, extension)
            .ok_or_else(|| "deck is not under tests/xyce/Netlists".to_string())?;
        if !path.is_file() {
            return Err(format!(
                "missing checked-in AC_IC oracle {}",
                self.display_path(&path)
            ));
        }
        Ok(path)
    }

    fn parse_ac_initial_condition_reference_file(
        request: &XycePrintOutputRequest,
        path: &Path,
    ) -> Result<XycePrnTable, String> {
        match request.format.as_deref().unwrap_or("STD").trim() {
            format if format.eq_ignore_ascii_case("CSV") => Self::parse_csv_file(path),
            format if format.eq_ignore_ascii_case("PROBE") => Self::parse_tran_csd_file(path),
            _ => Self::parse_prn_file(path),
        }
    }

    fn compare_ac_initial_condition_reference(
        &self,
        reference: &XycePrnTable,
        print: &XycePrintRequest,
        netlist: &Netlist,
        source: &str,
        dc: &XyceDcSweep,
        result: &DcSweepPointResult,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        let layout = Self::transient_reference_layout(reference)?;
        if reference.rows.len() != 1 {
            return Err(format!(
                "AC_IC reference row count ({}) does not match .OP point count (1)",
                reference.rows.len()
            ));
        }

        let row = &reference.rows[0];
        if row.len() != reference.columns.len() {
            return Err(format!(
                "row 0 has {} values, expected {}",
                row.len(),
                reference.columns.len()
            ));
        }

        let mut mismatches = Vec::new();
        if let Some(stepnum_column) = layout.stepnum_column {
            let expected_stepnum = row[stepnum_column];
            if let Some(relative_error) = self.value_mismatch(
                expected_stepnum,
                0.0,
                self.default_comparison_tolerance("stepnum"),
            ) {
                mismatches.push(XyceValueMismatch {
                    row: 0,
                    probe: "STEPNUM".to_string(),
                    expected: expected_stepnum,
                    actual: 0.0,
                    relative_error,
                });
            }
        }
        if let Some(index_column) = layout.index_column {
            let expected_index = row[index_column];
            if let Some(relative_error) = self.value_mismatch(
                expected_index,
                0.0,
                self.default_comparison_tolerance("index"),
            ) {
                mismatches.push(XyceValueMismatch {
                    row: 0,
                    probe: "Index".to_string(),
                    expected: expected_index,
                    actual: 0.0,
                    relative_error,
                });
            }
        }

        let expected_time = row[layout.time_column];
        if let Some(relative_error) = self.value_mismatch(
            expected_time,
            0.0,
            self.default_comparison_tolerance("time"),
        ) {
            mismatches.push(XyceValueMismatch {
                row: 0,
                probe: "TIME".to_string(),
                expected: expected_time,
                actual: 0.0,
                relative_error,
            });
        }
        if mismatches.len() >= self.config.max_mismatches {
            mismatches.truncate(self.config.max_mismatches);
            return Ok(mismatches);
        }

        let data_columns = self.reference_data_columns(
            reference,
            print,
            netlist,
            layout.data_column_offset,
            true,
        )?;
        let comp_tolerances = self.comp_tolerances(source, &data_columns)?;
        let sweep_point = XyceDcSweepPoint {
            primary: dc.start,
            secondary: None,
        };
        for (column_index, column) in data_columns.iter().enumerate() {
            let expected = row[column_index + layout.data_column_offset];
            let (probe, actual) = match column {
                XyceReferenceColumn::PrimarySweep { name } => (name.as_str(), dc.start),
                XyceReferenceColumn::Probe { name } => (
                    name.as_str(),
                    Self::evaluate_dc_probe(
                        name,
                        netlist,
                        dc,
                        sweep_point,
                        &result.result,
                        &result.device_op_report,
                    )?,
                ),
            };
            let normalized_probe = Self::normalize_probe(probe);
            let tolerance = comp_tolerances
                .get(&normalized_probe)
                .copied()
                .unwrap_or_else(|| self.default_comparison_tolerance(&normalized_probe));
            if let Some(relative_error) = self.value_mismatch(expected, actual, tolerance) {
                mismatches.push(XyceValueMismatch {
                    row: 0,
                    probe: probe.to_string(),
                    expected,
                    actual,
                    relative_error,
                });
                if mismatches.len() >= self.config.max_mismatches {
                    mismatches.truncate(self.config.max_mismatches);
                    break;
                }
            }
        }
        Ok(mismatches)
    }

    fn compare_step_ac_reference_batches(
        &self,
        reference: &XycePrnTable,
        print: &XycePrintRequest,
        source: &str,
        points_per_step: usize,
        batches: &[XyceAcResultBatch],
    ) -> Result<Vec<XyceValueMismatch>, String> {
        let step_references =
            Self::split_ac_step_reference(reference, batches.len(), points_per_step)?;
        let mut mismatches = Vec::new();
        let mut row_offset = 0usize;
        for (step_index, (batch, step_reference)) in
            batches.iter().zip(step_references.iter()).enumerate()
        {
            let mut step_mismatches = self.compare_ac_prn_reference_with_step(
                step_reference,
                print,
                &batch.netlist,
                source,
                &batch.results,
                Some(step_index),
            )?;
            for mismatch in &mut step_mismatches {
                mismatch.row += row_offset;
            }
            row_offset += step_reference.rows.len();
            mismatches.extend(step_mismatches);
            if mismatches.len() >= self.config.max_mismatches {
                mismatches.truncate(self.config.max_mismatches);
                break;
            }
        }
        Ok(mismatches)
    }

    fn compare_ac_side_outputs(
        &self,
        plan: &XyceStaticAcPlan,
        netlist: &Netlist,
        results: &[AcResult],
    ) -> Result<Vec<XyceValueMismatch>, String> {
        if plan.output_override {
            return Ok(Vec::new());
        }
        let Some(primary_reference_path) = plan.reference_path.as_ref() else {
            return Err(
                "AC side-output comparison requires a primary .PRINT AC oracle".to_string(),
            );
        };
        let side_outputs = Self::prn_compatible_ac_side_output_requests(&plan.source)?;
        let mut all_mismatches = Vec::new();
        for request in side_outputs {
            let file = request
                .file
                .as_deref()
                .expect("side output request has FILE= set");
            if Some(file) == plan.primary_ac_file.as_deref() {
                continue;
            }
            let reference_path =
                Self::ac_side_output_reference_path(primary_reference_path, &request, file)?;
            let reference = Self::parse_prn_file(&reference_path).map_err(|err| {
                format!(
                    "failed to parse AC side-output oracle {}: {err}",
                    self.display_path(&reference_path)
                )
            })?;
            let print = XycePrintRequest {
                probes: request.probes,
            };
            let mut mismatches =
                self.compare_ac_prn_reference(&reference, &print, netlist, &plan.source, results)?;
            for mismatch in &mut mismatches {
                mismatch.probe = format!("{file}:{}", mismatch.probe);
            }
            all_mismatches.extend(mismatches);
            if all_mismatches.len() >= self.config.max_mismatches {
                all_mismatches.truncate(self.config.max_mismatches);
                break;
            }
        }
        Ok(all_mismatches)
    }

    fn compare_ac_data_side_outputs(
        &self,
        plan: &XyceStaticAcPlan,
        points: &[XyceAcDataPointResult],
    ) -> Result<Vec<XyceValueMismatch>, String> {
        if plan.output_override {
            return Ok(Vec::new());
        }
        let Some(primary_reference_path) = plan.reference_path.as_ref() else {
            return Err(
                ".AC DATA side-output comparison requires a primary .PRINT AC oracle".to_string(),
            );
        };
        let side_outputs = Self::prn_compatible_ac_side_output_requests(&plan.source)?;
        let mut all_mismatches = Vec::new();
        for request in side_outputs {
            let file = request
                .file
                .as_deref()
                .expect("side output request has FILE= set");
            if Some(file) == plan.primary_ac_file.as_deref() {
                continue;
            }
            let reference_path =
                Self::ac_side_output_reference_path(primary_reference_path, &request, file)?;
            let reference = Self::parse_prn_file(&reference_path).map_err(|err| {
                format!(
                    "failed to parse AC side-output oracle {}: {err}",
                    self.display_path(&reference_path)
                )
            })?;
            let print = XycePrintRequest {
                probes: request.probes,
            };
            let mut mismatches =
                self.compare_ac_data_prn_reference(&reference, &print, &plan.source, points)?;
            for mismatch in &mut mismatches {
                mismatch.probe = format!("{file}:{}", mismatch.probe);
            }
            all_mismatches.extend(mismatches);
            if all_mismatches.len() >= self.config.max_mismatches {
                all_mismatches.truncate(self.config.max_mismatches);
                break;
            }
        }
        Ok(all_mismatches)
    }

    fn compare_step_ac_side_outputs(
        &self,
        plan: &XyceStaticAcPlan,
        points_per_step: usize,
        batches: &[XyceAcResultBatch],
    ) -> Result<Vec<XyceValueMismatch>, String> {
        if plan.output_override {
            return Ok(Vec::new());
        }
        let Some(primary_reference_path) = plan.reference_path.as_ref() else {
            return Err(
                "stepped AC side-output comparison requires a primary .PRINT AC oracle".to_string(),
            );
        };
        let side_outputs = Self::prn_compatible_ac_side_output_requests(&plan.source)?;
        let mut all_mismatches = Vec::new();
        for request in side_outputs {
            let file = request
                .file
                .as_deref()
                .expect("side output request has FILE= set");
            if Some(file) == plan.primary_ac_file.as_deref() {
                continue;
            }
            let reference_path =
                Self::ac_side_output_reference_path(primary_reference_path, &request, file)?;
            let reference = Self::parse_prn_file(&reference_path).map_err(|err| {
                format!(
                    "failed to parse AC side-output oracle {}: {err}",
                    self.display_path(&reference_path)
                )
            })?;
            let print = XycePrintRequest {
                probes: request.probes,
            };
            let mut mismatches = self.compare_step_ac_reference_batches(
                &reference,
                &print,
                &plan.source,
                points_per_step,
                batches,
            )?;
            for mismatch in &mut mismatches {
                mismatch.probe = format!("{file}:{}", mismatch.probe);
            }
            all_mismatches.extend(mismatches);
            if all_mismatches.len() >= self.config.max_mismatches {
                all_mismatches.truncate(self.config.max_mismatches);
                break;
            }
        }
        Ok(all_mismatches)
    }

    fn run_static_prn_dc_plan(
        &self,
        deck: &XyceDeck,
        plan: XyceExecutionPlan,
        start: Instant,
    ) -> XyceTestResult {
        let contract = plan.contract.result_contract(false);
        let netlist = match Self::parse_netlist_with_expression_dialect_and_execution_dir(
            &plan.source,
            &plan.deck_path,
            plan.expression_dialect,
            plan.execution_dir.as_deref(),
        ) {
            Ok(netlist) => netlist,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("parse failed after contract validation: {err}"),
                    Vec::new(),
                );
            }
        };

        let reference = if matches!(
            plan.contract,
            XyceStaticDcContract::WrapperFilePrn | XyceStaticDcContract::WrapperNoOutput
        ) {
            None
        } else {
            match Self::parse_dc_reference_file(plan.contract, &plan.reference_path) {
                Ok(reference) => Some(reference),
                Err(err) if Self::is_parameter_sweep_summary_reference(&plan.reference_path) => {
                    return self.expected_unsupported_result(
                        deck,
                        start,
                        "unsupported_xyce_oracle",
                        &format!(
                            "checked-in Xyce sidecar is a parameter-sweep summary, not a numeric .PRINT table: {err}"
                        ),
                    );
                }
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("failed to parse Xyce reference oracle: {err}"),
                        Vec::new(),
                    );
                }
            }
        };

        if !plan.steps.is_empty() {
            let Some(reference) = reference else {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    "file-output-only .STEP DC comparison is not implemented".to_string(),
                    Vec::new(),
                );
            };
            return self.run_static_prn_step_dc_plan(deck, plan, netlist, reference, start);
        }

        if let Some(dc_data) = &plan.dc_data {
            let Some(reference) = reference.as_ref() else {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    "file-output-only .DC DATA comparison is not implemented".to_string(),
                    Vec::new(),
                );
            };
            let results = match self.run_static_dc_data_results(&netlist, dc_data, start) {
                Ok(results) => results,
                Err(SimulationError::Aborted) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!(
                            "simulation exceeded timeout ({}ms)",
                            self.config.max_time_per_test_ms
                        ),
                        Vec::new(),
                    );
                }
                Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                    return self.expected_unsupported_result(
                        deck,
                        start,
                        "unsupported_xyce_runtime",
                        &format!("RSpice runtime does not yet support this deck: {err}"),
                    );
                }
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("simulation error: {err}"),
                        Vec::new(),
                    );
                }
            };
            let mismatches = match self.compare_dc_data_prn_reference(
                reference,
                &plan.print,
                &plan.source,
                &plan.dc,
                &results,
            ) {
                Ok(mismatches) => mismatches,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("reference comparison error: {err}"),
                        Vec::new(),
                    );
                }
            };
            return if mismatches.is_empty() {
                self.passed_result(deck, start, contract)
            } else {
                self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("{} Xyce reference mismatch(es)", mismatches.len()),
                    mismatches,
                )
            };
        }

        let engine = self.create_dc_engine();
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let results = match engine.run_dc_sweep2_spec_with_report_and_abort(
            &netlist,
            &plan.dc.source,
            &plan.dc.primary_spec(),
            plan.dc.sweep2.as_ref(),
            &abort,
        ) {
            Ok(results) => results,
            Err(SimulationError::Aborted) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!(
                        "simulation exceeded timeout ({}ms)",
                        self.config.max_time_per_test_ms
                    ),
                    Vec::new(),
                );
            }
            Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                return self.expected_unsupported_result(
                    deck,
                    start,
                    "unsupported_xyce_runtime",
                    &format!("RSpice runtime does not yet support this deck: {err}"),
                );
            }
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("simulation error: {err}"),
                    Vec::new(),
                );
            }
        };

        let mismatches = if let Some(reference) = &reference {
            match self.compare_dc_prn_reference(
                reference,
                &plan.print,
                &netlist,
                &plan.source,
                &plan.dc,
                &results,
            ) {
                Ok(mismatches) => mismatches,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("reference comparison error: {err}"),
                        Vec::new(),
                    );
                }
            }
        } else {
            Vec::new()
        };

        if mismatches.is_empty()
            && matches!(plan.contract, XyceStaticDcContract::WrapperGnuplotSplot)
        {
            let batches = [XyceDcResultBatch {
                netlist: netlist.clone(),
                results: results.clone(),
            }];
            let side_mismatches =
                match self.compare_gnuplot_splot_side_output_batches(&plan, &batches) {
                    Ok(mismatches) => mismatches,
                    Err(err) => {
                        return self.failure_result(
                            deck,
                            start,
                            contract,
                            format!("GNUPLOT/SPLOT side-output comparison error: {err}"),
                            Vec::new(),
                        );
                    }
                };
            if !side_mismatches.is_empty() {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!(
                        "{} Xyce GNUPLOT/SPLOT side-output mismatch(es)",
                        side_mismatches.len()
                    ),
                    side_mismatches,
                );
            }
        }

        if mismatches.is_empty()
            && !matches!(plan.contract, XyceStaticDcContract::WrapperGnuplotSplot)
        {
            let batches = [XyceDcResultBatch {
                netlist: netlist.clone(),
                results: results.clone(),
            }];
            let side_mismatches =
                match self.compare_prn_compatible_side_output_batches(&plan, &batches) {
                    Ok(mismatches) => mismatches,
                    Err(err) => {
                        return self.failure_result(
                            deck,
                            start,
                            contract,
                            format!("PRN-compatible side-output comparison error: {err}"),
                            Vec::new(),
                        );
                    }
                };
            if !side_mismatches.is_empty() {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!(
                        "{} Xyce PRN-compatible side-output mismatch(es)",
                        side_mismatches.len()
                    ),
                    side_mismatches,
                );
            }
        }

        if mismatches.is_empty() {
            self.passed_result(deck, start, contract)
        } else {
            self.failure_result(
                deck,
                start,
                contract,
                format!("{} Xyce reference mismatch(es)", mismatches.len()),
                mismatches,
            )
        }
    }

    fn run_static_prn_tran_plan(
        &self,
        deck: &XyceDeck,
        plan: XyceStaticTranPlan,
        start: Instant,
    ) -> XyceTestResult {
        let contract = plan.contract.result_contract(!plan.steps.is_empty());
        let netlist = match Self::parse_xyce_netlist(&plan.source, &plan.deck_path) {
            Ok(netlist) => netlist,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("parse failed after contract validation: {err}"),
                    Vec::new(),
                );
            }
        };

        let reference = match Self::parse_tran_reference_file(plan.contract, &plan.reference_path) {
            Ok(reference) => reference,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("failed to parse Xyce transient oracle: {err}"),
                    Vec::new(),
                );
            }
        };
        if !plan.steps.is_empty() {
            return self.run_static_prn_step_tran_plan(deck, plan, netlist, reference, start);
        }
        let reference_time_grid = match Self::reference_time_grid(&reference) {
            Ok(grid) => grid,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("reference time-grid error: {err}"),
                    Vec::new(),
                );
            }
        };
        let tran = Self::tran_analysis_for_reference_stop(
            plan.contract,
            plan.tran,
            reference_time_grid.as_slice(),
        );

        let max_step = match Self::transient_max_step_for_reference(&netlist, &tran, &reference) {
            Ok(max_step) => max_step,
            Err(err) if err.contains("transient harness execution envelope") => {
                return self.expected_unsupported_result(
                    deck,
                    start,
                    "unsupported_xyce_contract",
                    &err,
                );
            }
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("reference time-grid error: {err}"),
                    Vec::new(),
                );
            }
        };

        let initial_step = Self::xyce_initial_timestep_for_tran(&plan.tran);
        let engine = self.create_xyce_static_tran_engine(None, initial_step);
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let mut best_mismatches = None;
        let mut simulation_error = None;
        match engine.run_tran_with_abort(&netlist, tran.stop, max_step, &abort) {
            Ok(result) => {
                let mismatches = match self.compare_tran_prn_reference(
                    &reference,
                    &plan.print,
                    &netlist,
                    &plan.source,
                    &result,
                    plan.wrapper_tolerance,
                ) {
                    Ok(mismatches) => mismatches,
                    Err(err) => {
                        return self.failure_result(
                            deck,
                            start,
                            contract,
                            format!("reference comparison error: {err}"),
                            Vec::new(),
                        );
                    }
                };

                if mismatches.is_empty() {
                    return self.passed_or_tran_side_output_failure(
                        deck, start, contract, &plan, &netlist, &result,
                    );
                }

                best_mismatches = Some(mismatches);
            }
            Err(SimulationError::Aborted) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!(
                        "simulation exceeded timeout ({}ms)",
                        self.config.max_time_per_test_ms
                    ),
                    Vec::new(),
                );
            }
            Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                return self.expected_unsupported_result(
                    deck,
                    start,
                    "unsupported_xyce_runtime",
                    &format!("RSpice runtime does not yet support this transient deck: {err}"),
                );
            }
            Err(err) => {
                simulation_error = Some(format!("simulation error: {err}"));
            }
        }

        let capacitor_branch_print =
            Self::transient_print_requests_linear_capacitor_branch_quantity(&netlist, &plan.print);
        let mut fallback_errors = Vec::new();

        let locked_engine =
            self.create_xyce_static_tran_engine(Some(reference_time_grid.clone()), initial_step);
        match locked_engine.run_tran_with_abort(&netlist, tran.stop, max_step, &abort) {
            Ok(locked_result) => {
                match self.compare_tran_prn_reference(
                    &reference,
                    &plan.print,
                    &netlist,
                    &plan.source,
                    &locked_result,
                    plan.wrapper_tolerance,
                ) {
                    Ok(locked_mismatches) => {
                        if locked_mismatches.is_empty() {
                            return self.passed_or_tran_side_output_failure(
                                deck,
                                start,
                                contract,
                                &plan,
                                &netlist,
                                &locked_result,
                            );
                        }
                        if Self::candidate_mismatches_are_better(
                            best_mismatches.as_deref(),
                            &locked_mismatches,
                        ) {
                            best_mismatches = Some(locked_mismatches);
                        }
                    }
                    Err(err) => {
                        fallback_errors.push(format!(
                            "locked time-grid reference comparison error: {err}"
                        ));
                    }
                }
            }
            Err(SimulationError::Aborted) => {
                fallback_errors.push(format!(
                    "locked time-grid simulation exceeded timeout ({}ms)",
                    self.config.max_time_per_test_ms
                ));
            }
            Err(err) => {
                fallback_errors.push(format!("locked time-grid simulation error: {err}"));
            }
        }

        if !capacitor_branch_print {
            let backward_euler_engine = self
                .create_xyce_static_tran_engine_with_integration_method(
                    Some(reference_time_grid),
                    crate::analysis::IntegrationMethod::BackwardEuler,
                    initial_step,
                );
            match backward_euler_engine.run_tran_with_abort(&netlist, tran.stop, max_step, &abort) {
                Ok(backward_euler_result) => {
                    match self.compare_tran_prn_reference(
                        &reference,
                        &plan.print,
                        &netlist,
                        &plan.source,
                        &backward_euler_result,
                        plan.wrapper_tolerance,
                    ) {
                        Ok(backward_euler_mismatches) => {
                            if backward_euler_mismatches.is_empty() {
                                return self.passed_or_tran_side_output_failure(
                                    deck,
                                    start,
                                    contract,
                                    &plan,
                                    &netlist,
                                    &backward_euler_result,
                                );
                            }
                            if Self::candidate_mismatches_are_better(
                                best_mismatches.as_deref(),
                                &backward_euler_mismatches,
                            ) {
                                best_mismatches = Some(backward_euler_mismatches);
                            }
                        }
                        Err(err) => {
                            fallback_errors
                                .push(format!("backward-Euler reference comparison error: {err}"));
                        }
                    }
                }
                Err(SimulationError::Aborted) => {
                    fallback_errors.push(format!(
                        "backward-Euler simulation exceeded timeout ({}ms)",
                        self.config.max_time_per_test_ms
                    ));
                }
                Err(err) => {
                    fallback_errors.push(format!("backward-Euler simulation error: {err}"));
                }
            }
        }

        if let Some(best_mismatches) = best_mismatches {
            self.failure_result(
                deck,
                start,
                contract,
                format!(
                    "{} Xyce transient reference mismatch(es)",
                    best_mismatches.len()
                ),
                best_mismatches,
            )
        } else {
            let mut message = simulation_error.unwrap_or_else(|| {
                "transient simulation produced no comparable result".to_string()
            });
            if !fallback_errors.is_empty() {
                message.push_str("; ");
                message.push_str(&fallback_errors.join("; "));
            }
            self.failure_result(deck, start, contract, message, Vec::new())
        }
    }

    fn run_static_prn_step_tran_plan(
        &self,
        deck: &XyceDeck,
        plan: XyceStaticTranPlan,
        netlist: Netlist,
        reference: XycePrnTable,
        start: Instant,
    ) -> XyceTestResult {
        let contract = plan.contract.result_contract(true);
        let expansion_engine = self.create_xyce_engine();
        let step_runs =
            match Self::nested_step_runs_for_commands(&expansion_engine, &netlist, &plan.steps) {
                Ok(runs) => runs,
                Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                    return self.expected_unsupported_result(
                        deck,
                        start,
                        "unsupported_xyce_runtime",
                        &format!("RSpice runtime does not yet support this .STEP TRAN deck: {err}"),
                    );
                }
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!(".STEP expansion error: {err}"),
                        Vec::new(),
                    );
                }
            };

        let step_references =
            match Self::split_transient_step_reference(&reference, step_runs.len()) {
                Ok(references) => references,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("stepped transient oracle error: {err}"),
                        Vec::new(),
                    );
                }
            };

        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let mismatches =
            match self.compare_step_tran_runs(&plan, &step_runs, &step_references, &abort, false) {
                Ok(mismatches) => {
                    if mismatches.is_empty() {
                        return self.passed_or_step_tran_side_output_failure(
                            deck, start, contract, &plan, &step_runs, &abort, false,
                        );
                    }
                    Some(mismatches)
                }
                Err(err) if err.starts_with("UNSUPPORTED:") => {
                    return self.expected_unsupported_result(
                        deck,
                        start,
                        "unsupported_xyce_runtime",
                        err.trim_start_matches("UNSUPPORTED:").trim(),
                    );
                }
                Err(err) => {
                    let locked_result = self.compare_step_tran_runs(
                        &plan,
                        &step_runs,
                        &step_references,
                        &abort,
                        true,
                    );
                    return match locked_result {
                        Ok(locked_mismatches) if locked_mismatches.is_empty() => self
                            .passed_or_step_tran_side_output_failure(
                                deck, start, contract, &plan, &step_runs, &abort, true,
                            ),
                        Ok(locked_mismatches) => self.failure_result(
                            deck,
                            start,
                            contract,
                            format!(
                                "{} Xyce stepped transient reference mismatch(es)",
                                locked_mismatches.len()
                            ),
                            locked_mismatches,
                        ),
                        Err(locked_err) => self.failure_result(
                            deck,
                            start,
                            contract,
                            format!("{err}; locked time-grid retry failed: {locked_err}"),
                            Vec::new(),
                        ),
                    };
                }
            };

        let mismatches = mismatches.expect("non-empty adaptive mismatches");

        if let Ok(locked_mismatches) =
            self.compare_step_tran_runs(&plan, &step_runs, &step_references, &abort, true)
        {
            if locked_mismatches.is_empty() {
                return self.passed_or_step_tran_side_output_failure(
                    deck, start, contract, &plan, &step_runs, &abort, true,
                );
            }
            if Self::candidate_mismatches_are_better(Some(&mismatches), &locked_mismatches) {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!(
                        "{} Xyce stepped transient reference mismatch(es)",
                        locked_mismatches.len()
                    ),
                    locked_mismatches,
                );
            }
        }

        self.failure_result(
            deck,
            start,
            contract,
            format!(
                "{} Xyce stepped transient reference mismatch(es)",
                mismatches.len()
            ),
            mismatches,
        )
    }

    fn candidate_mismatches_are_better(
        current_best: Option<&[XyceValueMismatch]>,
        candidate: &[XyceValueMismatch],
    ) -> bool {
        let Some(best) = current_best else {
            return true;
        };
        if candidate.len() != best.len() {
            return candidate.len() < best.len();
        }

        let candidate_max = Self::mismatch_max_relative_error(candidate);
        let best_max = Self::mismatch_max_relative_error(best);
        if candidate_max != best_max {
            return candidate_max < best_max;
        }

        Self::mismatch_relative_error_sum(candidate) < Self::mismatch_relative_error_sum(best)
    }

    fn mismatch_max_relative_error(mismatches: &[XyceValueMismatch]) -> f64 {
        mismatches
            .iter()
            .map(|mismatch| mismatch.relative_error)
            .filter(|value| value.is_finite())
            .fold(0.0, f64::max)
    }

    fn mismatch_relative_error_sum(mismatches: &[XyceValueMismatch]) -> f64 {
        mismatches
            .iter()
            .map(|mismatch| mismatch.relative_error)
            .filter(|value| value.is_finite())
            .sum()
    }

    fn compare_step_tran_runs(
        &self,
        plan: &XyceStaticTranPlan,
        step_runs: &[XyceStepRun],
        step_references: &[XycePrnTable],
        abort: &dyn AbortSignal,
        locked_time_grid: bool,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        self.compare_step_tran_runs_with_print(
            plan,
            &plan.print,
            step_runs,
            step_references,
            abort,
            locked_time_grid,
        )
    }

    fn compare_step_tran_runs_with_print(
        &self,
        plan: &XyceStaticTranPlan,
        print: &XycePrintRequest,
        step_runs: &[XyceStepRun],
        step_references: &[XycePrnTable],
        abort: &dyn AbortSignal,
        locked_time_grid: bool,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        let mut mismatches = Vec::new();
        let mut row_offset = 0usize;
        for (step_index, (run, reference)) in
            step_runs.iter().zip(step_references.iter()).enumerate()
        {
            let max_step =
                Self::transient_max_step_for_reference(&run.netlist, &plan.tran, reference)
                    .map_err(|err| {
                        if err.contains("transient harness execution envelope") {
                            format!("UNSUPPORTED: {err}")
                        } else {
                            format!("reference time-grid error: step {}: {err}", step_index + 1)
                        }
                    })?;
            let engine = if locked_time_grid {
                self.create_xyce_engine_with_locked_time_grid(Some(Self::reference_time_grid(
                    reference,
                )?))
            } else {
                self.create_xyce_engine()
            };
            let result = match engine.run_tran_with_abort(
                &run.netlist,
                plan.tran.stop,
                max_step,
                abort,
            ) {
                Ok(result) => result,
                Err(SimulationError::Aborted) => {
                    return Err(format!(
                        "simulation exceeded timeout ({}ms)",
                        self.config.max_time_per_test_ms
                    ));
                }
                Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                    return Err(format!(
                        "UNSUPPORTED: RSpice runtime does not yet support this .STEP TRAN deck: {err}"
                    ));
                }
                Err(err) => {
                    return Err(format!("simulation error: step {}: {err}", step_index + 1));
                }
            };

            let mut step_mismatches = self
                .compare_tran_prn_reference(
                    reference,
                    print,
                    &run.netlist,
                    &plan.source,
                    &result,
                    plan.wrapper_tolerance,
                )
                .map_err(|err| {
                    format!("reference comparison error: step {}: {err}", step_index + 1)
                })?;
            for mismatch in &mut step_mismatches {
                mismatch.row += row_offset;
            }
            mismatches.extend(step_mismatches);
            if mismatches.len() >= self.config.max_mismatches {
                mismatches.truncate(self.config.max_mismatches);
                return Ok(mismatches);
            }
            row_offset += reference.rows.len();
        }
        Ok(mismatches)
    }

    fn passed_or_tran_side_output_failure(
        &self,
        deck: &XyceDeck,
        start: Instant,
        contract: &str,
        plan: &XyceStaticTranPlan,
        netlist: &Netlist,
        result: &TransientResult,
    ) -> XyceTestResult {
        match self.compare_tran_side_outputs(plan, netlist, result) {
            Ok(mismatches) if mismatches.is_empty() => self.passed_result(deck, start, contract),
            Ok(mismatches) => self.failure_result(
                deck,
                start,
                contract,
                format!(
                    "{} Xyce transient side-output mismatch(es)",
                    mismatches.len()
                ),
                mismatches,
            ),
            Err(err) => self.failure_result(
                deck,
                start,
                contract,
                format!("transient side-output comparison error: {err}"),
                Vec::new(),
            ),
        }
    }

    fn passed_or_step_tran_side_output_failure(
        &self,
        deck: &XyceDeck,
        start: Instant,
        contract: &str,
        plan: &XyceStaticTranPlan,
        step_runs: &[XyceStepRun],
        abort: &dyn AbortSignal,
        locked_time_grid: bool,
    ) -> XyceTestResult {
        match self.compare_step_tran_side_outputs(plan, step_runs, abort, locked_time_grid) {
            Ok(mismatches) if mismatches.is_empty() => self.passed_result(deck, start, contract),
            Ok(mismatches) => self.failure_result(
                deck,
                start,
                contract,
                format!(
                    "{} Xyce stepped transient side-output mismatch(es)",
                    mismatches.len()
                ),
                mismatches,
            ),
            Err(err) => self.failure_result(
                deck,
                start,
                contract,
                format!("stepped transient side-output comparison error: {err}"),
                Vec::new(),
            ),
        }
    }

    fn compare_tran_side_outputs(
        &self,
        plan: &XyceStaticTranPlan,
        netlist: &Netlist,
        result: &TransientResult,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        if plan.output_override {
            return Ok(Vec::new());
        }
        let side_outputs = Self::prn_compatible_tran_side_output_requests(&plan.source)?;
        let mut all_mismatches = Vec::new();
        for request in side_outputs {
            let file = request
                .file
                .as_deref()
                .expect("side output request has FILE= set");
            let reference_path = Self::side_output_reference_path(&plan.reference_path, file)?;
            let reference = Self::parse_prn_file(&reference_path).map_err(|err| {
                format!(
                    "failed to parse transient side-output oracle {}: {err}",
                    self.display_path(&reference_path)
                )
            })?;
            let print = XycePrintRequest {
                probes: request.probes,
            };
            let mut mismatches = self.compare_tran_prn_reference(
                &reference,
                &print,
                netlist,
                &plan.source,
                result,
                plan.wrapper_tolerance,
            )?;
            for mismatch in &mut mismatches {
                mismatch.probe = format!("{file}:{}", mismatch.probe);
            }
            all_mismatches.extend(mismatches);
            if all_mismatches.len() >= self.config.max_mismatches {
                all_mismatches.truncate(self.config.max_mismatches);
                break;
            }
        }
        Ok(all_mismatches)
    }

    fn compare_step_tran_side_outputs(
        &self,
        plan: &XyceStaticTranPlan,
        step_runs: &[XyceStepRun],
        abort: &dyn AbortSignal,
        locked_time_grid: bool,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        if plan.output_override {
            return Ok(Vec::new());
        }
        let side_outputs = Self::prn_compatible_tran_side_output_requests(&plan.source)?;
        let mut all_mismatches = Vec::new();
        for request in side_outputs {
            let file = request
                .file
                .as_deref()
                .expect("side output request has FILE= set");
            let reference_path = Self::side_output_reference_path(&plan.reference_path, file)?;
            let reference = Self::parse_prn_file(&reference_path).map_err(|err| {
                format!(
                    "failed to parse transient side-output oracle {}: {err}",
                    self.display_path(&reference_path)
                )
            })?;
            let step_references = Self::split_transient_step_reference(&reference, step_runs.len())
                .map_err(|err| format!("{file}: {err}"))?;
            let print = XycePrintRequest {
                probes: request.probes,
            };
            let mut mismatches = self.compare_step_tran_runs_with_print(
                plan,
                &print,
                step_runs,
                &step_references,
                abort,
                locked_time_grid,
            )?;
            for mismatch in &mut mismatches {
                mismatch.probe = format!("{file}:{}", mismatch.probe);
            }
            all_mismatches.extend(mismatches);
            if all_mismatches.len() >= self.config.max_mismatches {
                all_mismatches.truncate(self.config.max_mismatches);
                break;
            }
        }
        Ok(all_mismatches)
    }

    fn split_transient_step_reference(
        reference: &XycePrnTable,
        expected_steps: usize,
    ) -> Result<Vec<XycePrnTable>, String> {
        if expected_steps == 0 {
            return Err(".STEP expansion produced no runs".to_string());
        }
        if expected_steps == 1 {
            return Ok(vec![reference.clone()]);
        }
        let time_column = Self::reference_time_column_index(reference)
            .ok_or_else(|| "stepped transient reference table has no TIME column".to_string())?;
        let mut starts = vec![0usize];
        let mut previous_time = None;
        for (row_index, row) in reference.rows.iter().enumerate() {
            let time = *row.get(time_column).ok_or_else(|| {
                format!("row {row_index} has no TIME column at index {time_column}")
            })?;
            if !time.is_finite() {
                return Err(format!("row {row_index} has non-finite TIME value {time}"));
            }
            if let Some(previous) = previous_time
                && time < previous
            {
                starts.push(row_index);
            }
            previous_time = Some(time);
        }
        starts.push(reference.rows.len());
        let actual_steps = starts.len().saturating_sub(1);
        if actual_steps != expected_steps {
            return Err(format!(
                "reference contains {actual_steps} transient step table(s), but .STEP expansion produced {expected_steps} run(s)"
            ));
        }

        let mut references = Vec::with_capacity(expected_steps);
        for range in starts.windows(2) {
            let start = range[0];
            let end = range[1];
            if start == end {
                return Err("stepped transient reference contains an empty step table".to_string());
            }
            let reference = XycePrnTable {
                columns: reference.columns.clone(),
                rows: reference.rows[start..end].to_vec(),
            };
            Self::validate_transient_stepnum_column(&reference, references.len())?;
            references.push(reference);
        }
        Ok(references)
    }

    fn split_ac_step_reference(
        reference: &XycePrnTable,
        expected_steps: usize,
        points_per_step: usize,
    ) -> Result<Vec<XycePrnTable>, String> {
        if expected_steps == 0 {
            return Err(".STEP expansion produced no runs".to_string());
        }
        if points_per_step == 0 {
            return Err("AC analysis produced no frequency points".to_string());
        }
        if expected_steps == 1 {
            return Ok(vec![reference.clone()]);
        }
        let expected_rows = expected_steps
            .checked_mul(points_per_step)
            .ok_or_else(|| "stepped AC row count overflow".to_string())?;
        if reference.rows.len() != expected_rows {
            return Err(format!(
                "reference contains {} AC row(s), but .STEP expansion produced {expected_steps} run(s) with {points_per_step} frequency point(s) each",
                reference.rows.len()
            ));
        }

        let mut references = Vec::with_capacity(expected_steps);
        for step_index in 0..expected_steps {
            let start = step_index * points_per_step;
            let end = start + points_per_step;
            references.push(XycePrnTable {
                columns: reference.columns.clone(),
                rows: reference.rows[start..end].to_vec(),
            });
        }
        Ok(references)
    }

    fn validate_transient_stepnum_column(
        reference: &XycePrnTable,
        expected_step_index: usize,
    ) -> Result<(), String> {
        let layout = Self::transient_reference_layout(reference)?;
        let Some(stepnum_column) = layout.stepnum_column else {
            return Ok(());
        };
        let expected_step_index = expected_step_index as Value;
        for (row_index, row) in reference.rows.iter().enumerate() {
            let stepnum = *row.get(stepnum_column).ok_or_else(|| {
                format!("row {row_index} has no STEPNUM column at index {stepnum_column}")
            })?;
            if !stepnum.is_finite() {
                return Err(format!(
                    "row {row_index} has non-finite STEPNUM value {stepnum}"
                ));
            }
            if (stepnum - expected_step_index).abs() > f64::EPSILON {
                return Err(format!(
                    "row {row_index} has STEPNUM {stepnum}, expected {expected_step_index}"
                ));
            }
        }
        Ok(())
    }

    fn compare_gnuplot_splot_side_output_batches(
        &self,
        plan: &XyceExecutionPlan,
        batches: &[XyceDcResultBatch],
    ) -> Result<Vec<XyceValueMismatch>, String> {
        let (_, side) = Self::gnuplot_splot_print_pair(&plan.source)?;
        if side.probes != plan.print.probes {
            return Err("SPLOT side-output probes differ from primary GNUPLOT probes".to_string());
        }
        let file = side
            .file
            .as_deref()
            .ok_or_else(|| "SPLOT side-output request has no FILE= target".to_string())?;
        let side_reference_path =
            Self::side_output_reference_candidate(&plan.reference_path, file)?;
        let reference_path = if side_reference_path.is_file() {
            side_reference_path
        } else {
            plan.reference_path.clone()
        };
        let reference = Self::parse_prn_file(&reference_path).map_err(|err| {
            format!(
                "failed to parse SPLOT side-output oracle {}: {err}",
                self.display_path(&reference_path)
            )
        })?;
        let side_print = XycePrintRequest {
            probes: side.probes,
        };
        let mut mismatches = self.compare_dc_prn_reference_batches(
            &reference,
            &side_print,
            &plan.source,
            &plan.dc,
            batches,
        )?;
        for mismatch in &mut mismatches {
            mismatch.probe = format!("{file}:{}", mismatch.probe);
        }
        Ok(mismatches)
    }

    fn compare_prn_compatible_side_output_batches(
        &self,
        plan: &XyceExecutionPlan,
        batches: &[XyceDcResultBatch],
    ) -> Result<Vec<XyceValueMismatch>, String> {
        let side_outputs = Self::prn_compatible_side_output_requests(&plan.source)?;
        let mut all_mismatches = Vec::new();
        for request in side_outputs {
            let file = request
                .file
                .as_deref()
                .expect("side output request has FILE= set");
            let reference_path = Self::side_output_reference_path(&plan.reference_path, file)?;
            let reference = Self::parse_prn_file(&reference_path).map_err(|err| {
                format!(
                    "failed to parse side-output oracle {}: {err}",
                    self.display_path(&reference_path)
                )
            })?;
            let print = XycePrintRequest {
                probes: request.probes,
            };
            let mut mismatches = self.compare_dc_prn_reference_batches(
                &reference,
                &print,
                &plan.source,
                &plan.dc,
                batches,
            )?;
            for mismatch in &mut mismatches {
                mismatch.probe = format!("{file}:{}", mismatch.probe);
            }
            all_mismatches.extend(mismatches);
            if all_mismatches.len() >= self.config.max_mismatches {
                all_mismatches.truncate(self.config.max_mismatches);
                break;
            }
        }
        Ok(all_mismatches)
    }

    fn prn_compatible_side_output_requests(
        source: &str,
    ) -> Result<Vec<XycePrintOutputRequest>, String> {
        Ok(Self::dc_print_output_requests(source)?
            .into_iter()
            .filter(|request| {
                request.file.is_some()
                    && Self::dc_print_format_is_prn_compatible(
                        request.format.as_deref().unwrap_or("STD"),
                    )
            })
            .collect())
    }

    fn prn_compatible_tran_side_output_requests(
        source: &str,
    ) -> Result<Vec<XycePrintOutputRequest>, String> {
        Ok(Self::aggregate_print_output_requests(
            Self::print_output_requests(source, "TRAN")?,
            "TRAN",
        )?
        .into_iter()
        .filter(|request| {
            request.file.is_some()
                && Self::tran_print_format_is_prn_compatible(
                    request.format.as_deref().unwrap_or("STD"),
                )
        })
        .collect())
    }

    fn prn_compatible_ac_side_output_requests(
        source: &str,
    ) -> Result<Vec<XycePrintOutputRequest>, String> {
        Ok(
            Self::aggregate_print_output_requests(
                Self::print_output_requests(source, "AC")?,
                "AC",
            )?
            .into_iter()
            .filter(|request| {
                request.file.is_some()
                    && Self::ac_print_format_is_prn_compatible(
                        request.format.as_deref().unwrap_or("STD"),
                    )
            })
            .collect(),
        )
    }

    fn side_output_reference_path(reference_path: &Path, file: &str) -> Result<PathBuf, String> {
        let candidate = Self::side_output_reference_candidate(reference_path, file)?;
        if !candidate.is_file() {
            return Err(format!(
                "missing checked-in side-output oracle {}",
                candidate.display()
            ));
        }
        Ok(candidate)
    }

    fn side_output_reference_path_for_deck(
        &self,
        deck_path: &Path,
        file: &str,
    ) -> Result<PathBuf, String> {
        let anchor = self
            .static_output_reference_path(deck_path, "anchor")
            .ok_or_else(|| "deck is not under tests/xyce/Netlists".to_string())?;
        Self::side_output_reference_path(&anchor, file)
    }

    fn ac_side_output_reference_path(
        reference_path: &Path,
        request: &XycePrintOutputRequest,
        file: &str,
    ) -> Result<PathBuf, String> {
        let candidate = Self::side_output_reference_candidate(reference_path, file)?;
        if candidate.is_file() {
            return Ok(candidate);
        }
        let format = request.format.as_deref().unwrap_or("STD");
        if format.eq_ignore_ascii_case("GNUPLOT") || format.eq_ignore_ascii_case("SPLOT") {
            return Ok(reference_path.to_path_buf());
        }
        Err(format!(
            "missing checked-in side-output oracle {}",
            candidate.display()
        ))
    }

    fn side_output_reference_candidate(
        reference_path: &Path,
        file: &str,
    ) -> Result<PathBuf, String> {
        let side_path = Path::new(file);
        if side_path.is_absolute() {
            return Err(format!(
                "absolute FILE= side-output path '{}' cannot be mapped into the vendored OutputData tree",
                file
            ));
        }
        let parent = reference_path
            .parent()
            .ok_or_else(|| "primary reference path has no parent directory".to_string())?;
        Ok(parent.join(side_path))
    }

    fn run_static_prn_step_dc_plan(
        &self,
        deck: &XyceDeck,
        plan: XyceExecutionPlan,
        netlist: Netlist,
        reference: XycePrnTable,
        start: Instant,
    ) -> XyceTestResult {
        let contract = plan.contract.result_contract(true);
        let engine = self.create_dc_engine();
        let step_runs = match Self::nested_step_runs_for_commands(&engine, &netlist, &plan.steps) {
            Ok(runs) => runs,
            Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                return self.expected_unsupported_result(
                    deck,
                    start,
                    "unsupported_xyce_runtime",
                    &format!("RSpice runtime does not yet support this .STEP deck: {err}"),
                );
            }
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!(".STEP expansion error: {err}"),
                    Vec::new(),
                );
            }
        };

        if plan.contract.compares_step_res_reference() {
            if let Some(res_reference_path) =
                Self::step_res_reference_path(&plan.deck_path, &plan.reference_path)
            {
                if let Err(err) = self.compare_step_res_reference(
                    &res_reference_path,
                    &netlist,
                    &plan.steps,
                    &step_runs,
                ) {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("Xyce .STEP result summary comparison error: {err}"),
                        Vec::new(),
                    );
                }
            }
        }

        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let mut batches = Vec::with_capacity(step_runs.len());
        for run in step_runs {
            let results = match engine.run_dc_sweep2_spec_with_report_and_abort(
                &run.netlist,
                &plan.dc.source,
                &plan.dc.primary_spec(),
                plan.dc.sweep2.as_ref(),
                &abort,
            ) {
                Ok(results) => results,
                Err(SimulationError::Aborted) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!(
                            "simulation exceeded timeout ({}ms)",
                            self.config.max_time_per_test_ms
                        ),
                        Vec::new(),
                    );
                }
                Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                    return self.expected_unsupported_result(
                        deck,
                        start,
                        "unsupported_xyce_runtime",
                        &format!("RSpice runtime does not yet support this .STEP deck: {err}"),
                    );
                }
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("simulation error: {err}"),
                        Vec::new(),
                    );
                }
            };
            batches.push(XyceDcResultBatch {
                netlist: run.netlist,
                results,
            });
        }

        let mismatches = match self.compare_dc_prn_reference_batches(
            &reference,
            &plan.print,
            &plan.source,
            &plan.dc,
            &batches,
        ) {
            Ok(mismatches) => mismatches,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("reference comparison error: {err}"),
                    Vec::new(),
                );
            }
        };

        if mismatches.is_empty()
            && matches!(plan.contract, XyceStaticDcContract::WrapperGnuplotSplot)
        {
            let side_mismatches =
                match self.compare_gnuplot_splot_side_output_batches(&plan, &batches) {
                    Ok(mismatches) => mismatches,
                    Err(err) => {
                        return self.failure_result(
                            deck,
                            start,
                            contract,
                            format!("GNUPLOT/SPLOT side-output comparison error: {err}"),
                            Vec::new(),
                        );
                    }
                };
            if !side_mismatches.is_empty() {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!(
                        "{} Xyce GNUPLOT/SPLOT side-output mismatch(es)",
                        side_mismatches.len()
                    ),
                    side_mismatches,
                );
            }
        }

        if mismatches.is_empty() {
            if !matches!(plan.contract, XyceStaticDcContract::WrapperGnuplotSplot) {
                let side_mismatches =
                    match self.compare_prn_compatible_side_output_batches(&plan, &batches) {
                        Ok(mismatches) => mismatches,
                        Err(err) => {
                            return self.failure_result(
                                deck,
                                start,
                                contract,
                                format!("PRN-compatible side-output comparison error: {err}"),
                                Vec::new(),
                            );
                        }
                    };
                if !side_mismatches.is_empty() {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!(
                            "{} Xyce PRN-compatible side-output mismatch(es)",
                            side_mismatches.len()
                        ),
                        side_mismatches,
                    );
                }
            }
            return self.passed_result(deck, start, contract);
        }

        self.failure_result(
            deck,
            start,
            contract,
            format!("{} Xyce reference mismatch(es)", mismatches.len()),
            mismatches,
        )
    }

    fn nested_step_runs_for_commands(
        engine: &Engine,
        netlist: &Netlist,
        steps: &[StepCommand],
    ) -> Result<Vec<XyceStepRun>, SimulationError> {
        let mut runs = vec![XyceStepRunBuilder {
            step_values: Vec::new(),
            bindings: Vec::new(),
        }];

        for step in steps {
            let values = Self::step_values_for_expansion(netlist, step)?;
            if values.is_empty() {
                return Err(SimulationError::Circuit(
                    ".STEP produced no sweep values".to_string(),
                ));
            }

            let mut next_runs = Vec::with_capacity(runs.len() * values.len());
            for value in values {
                for run in &runs {
                    let mut step_values = run.step_values.clone();
                    step_values.push(value);
                    let mut bindings = run.bindings.clone();
                    bindings.push(XyceStepBinding {
                        step: step.clone(),
                        value,
                    });
                    next_runs.push(XyceStepRunBuilder {
                        step_values,
                        bindings,
                    });
                }
            }
            runs = next_runs;
        }

        runs.into_iter()
            .map(|run| {
                let netlist = Self::materialize_nested_step_run(engine, netlist, &run.bindings)?;
                Ok(XyceStepRun {
                    step_values: run.step_values,
                    netlist,
                })
            })
            .collect()
    }

    fn step_values_for_expansion(
        netlist: &Netlist,
        step: &StepCommand,
    ) -> Result<Vec<Value>, SimulationError> {
        match &step.sweep {
            StepSweep::Data { table_name } => {
                let table = netlist
                    .data_tables
                    .iter()
                    .find(|table| table.name.eq_ignore_ascii_case(table_name))
                    .ok_or_else(|| {
                        SimulationError::Circuit(format!(
                            ".STEP DATA table '{table_name}' not found"
                        ))
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
                Ok((0..table.rows.len()).map(|idx| idx as Value).collect())
            }
            _ => Ok(step.sweep.values()),
        }
    }

    fn materialize_nested_step_run(
        engine: &Engine,
        netlist: &Netlist,
        bindings: &[XyceStepBinding],
    ) -> Result<Netlist, SimulationError> {
        let param_overrides = bindings
            .iter()
            .filter(|binding| {
                binding.step.target == StepTarget::Param
                    && !matches!(binding.step.sweep, StepSweep::Data { .. })
            })
            .map(|binding| (binding.step.name.clone(), binding.value))
            .collect::<Vec<_>>();

        let mut run_netlist = if param_overrides.is_empty() {
            netlist.clone()
        } else {
            Engine::create_perturbed_netlist_multi(netlist, &param_overrides)?.0
        };

        for binding in bindings {
            if binding.step.target == StepTarget::Param
                && !matches!(binding.step.sweep, StepSweep::Data { .. })
            {
                continue;
            }
            run_netlist = Self::materialize_nonparam_step_binding(engine, &run_netlist, binding)?;
        }

        Ok(run_netlist)
    }

    fn materialize_nonparam_step_binding(
        engine: &Engine,
        netlist: &Netlist,
        binding: &XyceStepBinding,
    ) -> Result<Netlist, SimulationError> {
        let expanded = if matches!(binding.step.sweep, StepSweep::Data { .. }) {
            engine.step_netlists_for_command(netlist, &binding.step, &[])?
        } else {
            engine.step_netlists_for_command(netlist, &binding.step, &[binding.value])?
        };

        if let Some((_, stepped_netlist)) = expanded
            .into_iter()
            .find(|(value, _)| (*value - binding.value).abs() <= Value::EPSILON)
        {
            return Ok(stepped_netlist);
        }

        Err(SimulationError::Circuit(format!(
            ".STEP expansion for {} did not produce requested value {}",
            Self::step_res_variable_name(&binding.step),
            binding.value
        )))
    }

    fn run_baseline_family_contract(
        &self,
        deck: &XyceDeck,
        contract: XyceBaselineFamilyContract,
        start: Instant,
    ) -> XyceTestResult {
        let kind_name = contract.kind.name();
        let wrapper_contract = contract.kind.wrapper_contract();
        let baseline_contract = contract.kind.baseline_contract();
        let baseline_plan = match self
            .static_dc_plan_for_path(&contract.baseline_path, ExpressionDialect::Xyce)
        {
            Ok(plan) => plan,
            Err(reason) => {
                return self.expected_unsupported_result(
                    deck,
                    start,
                    wrapper_contract,
                    &format!(
                        "{kind_name} family '{}' baseline is not supported by the static DC adapter: {reason}",
                        contract.family
                    ),
                );
            }
        };
        let baseline_run = self.run_static_dc_results(&baseline_plan, start);
        let (baseline_netlist, baseline_results) = match baseline_run {
            Ok(results) => results,
            Err(SimulationError::Aborted) => {
                return self.failure_result(
                    deck,
                    start,
                    wrapper_contract,
                    format!(
                        "{kind_name} family '{}' baseline exceeded timeout ({}ms)",
                        contract.family, self.config.max_time_per_test_ms
                    ),
                    Vec::new(),
                );
            }
            Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                return self.expected_unsupported_result(
                    deck,
                    start,
                    wrapper_contract,
                    &format!(
                        "{kind_name} family '{}' baseline is not supported by RSpice yet: {err}",
                        contract.family
                    ),
                );
            }
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    wrapper_contract,
                    format!(
                        "{kind_name} family '{}' baseline error: {err}",
                        contract.family
                    ),
                    Vec::new(),
                );
            }
        };
        if contract.kind.compares_baseline_oracle()
            && let Some(reference_path) = self.static_prn_reference_path(&contract.baseline_path)
            && reference_path.is_file()
        {
            let reference = match Self::parse_prn_file(&reference_path) {
                Ok(reference) => reference,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        wrapper_contract,
                        format!(
                            "{kind_name} family '{}' baseline oracle parse error: {err}",
                            contract.family
                        ),
                        Vec::new(),
                    );
                }
            };
            let mismatches = match self.compare_dc_prn_reference(
                &reference,
                &baseline_plan.print,
                &baseline_netlist,
                &baseline_plan.source,
                &baseline_plan.dc,
                &baseline_results,
            ) {
                Ok(mismatches) => mismatches,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        wrapper_contract,
                        format!(
                            "{kind_name} family '{}' baseline oracle comparison error: {err}",
                            contract.family
                        ),
                        Vec::new(),
                    );
                }
            };
            if !mismatches.is_empty() {
                return self.failure_result(
                    deck,
                    start,
                    wrapper_contract,
                    format!(
                        "{} {kind_name} family '{}' baseline oracle mismatch(es)",
                        mismatches.len(),
                        contract.family
                    ),
                    mismatches,
                );
            }
        }
        let baseline_table = match self.dc_results_to_prn_table(
            &baseline_plan,
            &baseline_netlist,
            &baseline_results,
        ) {
            Ok(table) => table,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    wrapper_contract,
                    format!(
                        "{kind_name} family '{}' baseline output conversion failed: {err}",
                        contract.family
                    ),
                    Vec::new(),
                );
            }
        };

        let targets = if let Some(target_path) = contract.target_path {
            if Self::same_path(&target_path, &contract.baseline_path) {
                return self.passed_result(deck, start, baseline_contract);
            }
            vec![target_path]
        } else {
            contract
                .member_paths
                .iter()
                .filter(|path| !Self::same_path(path, &contract.baseline_path))
                .cloned()
                .collect::<Vec<_>>()
        };

        let mut all_mismatches = Vec::new();
        for target_path in targets {
            let target_plan = match self
                .static_dc_plan_for_path(&target_path, ExpressionDialect::Xyce)
            {
                Ok(plan) => plan,
                Err(reason) => {
                    return self.expected_unsupported_result(
                        deck,
                        start,
                        wrapper_contract,
                        &format!(
                            "{kind_name} family '{}' member {} is not supported by the static DC adapter: {reason}",
                            contract.family,
                            self.display_path(&target_path)
                        ),
                    );
                }
            };
            let (target_netlist, target_results) = match self
                .run_static_dc_results(&target_plan, start)
            {
                Ok(results) => results,
                Err(SimulationError::Aborted) => {
                    return self.failure_result(
                        deck,
                        start,
                        wrapper_contract,
                        format!(
                            "{kind_name} family '{}' member {} exceeded timeout ({}ms)",
                            contract.family,
                            self.display_path(&target_path),
                            self.config.max_time_per_test_ms
                        ),
                        Vec::new(),
                    );
                }
                Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                    return self.expected_unsupported_result(
                        deck,
                        start,
                        wrapper_contract,
                        &format!(
                            "{kind_name} family '{}' member {} is not supported by RSpice yet: {err}",
                            contract.family,
                            self.display_path(&target_path)
                        ),
                    );
                }
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        wrapper_contract,
                        format!(
                            "{kind_name} family '{}' member {} error: {err}",
                            contract.family,
                            self.display_path(&target_path)
                        ),
                        Vec::new(),
                    );
                }
            };

            let mut mismatches = match self.compare_dc_prn_reference(
                &baseline_table,
                &target_plan.print,
                &target_netlist,
                &baseline_plan.source,
                &target_plan.dc,
                &target_results,
            ) {
                Ok(mismatches) => mismatches,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        wrapper_contract,
                        format!(
                            "{kind_name} family '{}' member {} comparison error: {err}",
                            contract.family,
                            self.display_path(&target_path)
                        ),
                        Vec::new(),
                    );
                }
            };
            for mismatch in &mut mismatches {
                mismatch.probe = format!("{} {}", self.display_path(&target_path), mismatch.probe);
            }
            all_mismatches.extend(mismatches);
            if all_mismatches.len() >= self.config.max_mismatches {
                all_mismatches.truncate(self.config.max_mismatches);
                break;
            }
        }

        if all_mismatches.is_empty() {
            self.passed_result(deck, start, wrapper_contract)
        } else {
            self.failure_result(
                deck,
                start,
                wrapper_contract,
                format!(
                    "{} {kind_name} family '{}' mismatch(es)",
                    all_mismatches.len(),
                    contract.family
                ),
                all_mismatches,
            )
        }
    }

    fn run_static_dc_results(
        &self,
        plan: &XyceStaticDcPlan,
        start: Instant,
    ) -> Result<(Netlist, Vec<DcSweepPointResult>), SimulationError> {
        if !plan.steps.is_empty() {
            return Err(SimulationError::Netlist(
                ".STEP static DC execution requires the stepped .prn contract".to_string(),
            ));
        }
        let netlist = Self::parse_netlist_with_expression_dialect(
            &plan.source,
            &plan.deck_path,
            plan.expression_dialect,
        )
        .map_err(|err| SimulationError::Netlist(format!("{err}")))?;
        let engine = self.create_dc_engine();
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let results = engine.run_dc_sweep2_spec_with_report_and_abort(
            &netlist,
            &plan.dc.source,
            &plan.dc.primary_spec(),
            plan.dc.sweep2.as_ref(),
            &abort,
        )?;
        Ok((netlist, results))
    }

    fn run_static_dc_data_results(
        &self,
        base_netlist: &Netlist,
        dc_data: &XyceDcDataSweep,
        start: Instant,
    ) -> Result<Vec<XyceDcDataPointResult>, SimulationError> {
        let engine = self.create_dc_engine();
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let mut results = Vec::with_capacity(dc_data.rows.len());

        for (row_index, row) in dc_data.rows.iter().enumerate() {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let row_netlist = Self::materialize_dc_data_row_netlist(&engine, base_netlist, row)?;
            let (result, device_op_report) =
                engine.run_dc_op_with_report(&row_netlist).map_err(|err| {
                    SimulationError::Circuit(format!(
                        ".DC DATA row {} operating point failed: {}",
                        row_index + 1,
                        err
                    ))
                })?;
            results.push(XyceDcDataPointResult {
                netlist: row_netlist,
                point: DcSweepPointResult {
                    sweep_value: row_index as Value,
                    result,
                    device_op_report,
                },
            });
        }

        Ok(results)
    }

    fn materialize_dc_data_row_netlist(
        engine: &Engine,
        base_netlist: &Netlist,
        row: &XyceDcDataRow,
    ) -> Result<Netlist, SimulationError> {
        let param_overrides = row
            .overrides
            .iter()
            .filter_map(|override_| match override_ {
                XyceDcDataOverride::Parameter { name, value } => Some((name.clone(), *value)),
                XyceDcDataOverride::Device { .. } => None,
            })
            .collect::<Vec<_>>();
        let (mut row_netlist, param_bindings) = if param_overrides.is_empty() {
            (base_netlist.clone(), 0)
        } else {
            Engine::create_perturbed_netlist_multi(base_netlist, &param_overrides)?
        };
        if !param_overrides.is_empty() && base_netlist.source_text.is_some() && param_bindings == 0
        {
            let names = param_overrides
                .iter()
                .map(|(name, _)| name.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            return Err(SimulationError::Circuit(format!(
                ".DC DATA parameter override(s) {names} are not bound to any netlist expression"
            )));
        }

        for override_ in &row.overrides {
            let XyceDcDataOverride::Device {
                name,
                param_name,
                value,
            } = override_
            else {
                continue;
            };
            let step = StepCommand {
                target: StepTarget::Device,
                name: name.clone(),
                param_name: param_name.clone(),
                sweep: StepSweep::List(vec![*value]),
            };
            let mut stepped = engine.step_netlists_for_command(&row_netlist, &step, &[*value])?;
            let Some((_, next_netlist)) = stepped.pop() else {
                return Err(SimulationError::Circuit(format!(
                    ".DC DATA device override '{}' produced no stepped netlist",
                    name
                )));
            };
            row_netlist = next_netlist;
        }

        Ok(row_netlist)
    }

    fn dc_results_to_prn_table(
        &self,
        plan: &XyceStaticDcPlan,
        netlist: &Netlist,
        results: &[DcSweepPointResult],
    ) -> Result<XycePrnTable, String> {
        if !plan.steps.is_empty() {
            return Err(".STEP static DC results require the stepped .prn contract".to_string());
        }
        let mut columns = Vec::with_capacity(plan.print.probes.len() + 1);
        columns.push("Index".to_string());
        columns.extend(plan.print.probes.iter().cloned());

        let primary_points = plan.dc.primary_spec().points();
        if primary_points.is_empty() {
            return Err("primary DC sweep has no points".to_string());
        }
        let secondary_points = plan.dc.sweep2.as_ref().map(|sweep| sweep.spec().points());
        if secondary_points.as_ref().is_some_and(Vec::is_empty) {
            return Err("secondary DC sweep has no points".to_string());
        }

        let mut rows = Vec::with_capacity(results.len());
        for (row_index, point) in results.iter().enumerate() {
            let sweep_point = XyceDcSweepPoint {
                primary: point.sweep_value,
                secondary: if let Some(points) = secondary_points.as_ref() {
                    let outer_index = row_index / primary_points.len();
                    Some(*points.get(outer_index).ok_or_else(|| {
                        format!(
                            "row {row_index} maps outside secondary DC sweep point count ({})",
                            points.len()
                        )
                    })?)
                } else {
                    None
                },
            };

            let mut row = Vec::with_capacity(columns.len());
            row.push(row_index as f64);
            for probe in &plan.print.probes {
                row.push(Self::evaluate_dc_probe(
                    probe,
                    netlist,
                    &plan.dc,
                    sweep_point,
                    &point.result,
                    &point.device_op_report,
                )?);
            }
            rows.push(row);
        }

        Ok(XycePrnTable { columns, rows })
    }

    fn create_dc_engine(&self) -> Engine {
        self.create_xyce_engine()
    }

    fn create_xyce_engine(&self) -> Engine {
        self.create_xyce_engine_with_locked_time_grid(None)
    }

    fn create_xyce_engine_with_locked_time_grid(
        &self,
        locked_time_grid: Option<Vec<Value>>,
    ) -> Engine {
        Engine::new(self.xyce_engine_config(locked_time_grid))
    }

    fn create_xyce_static_tran_engine(
        &self,
        locked_time_grid: Option<Vec<Value>>,
        initial_timestep: Option<Value>,
    ) -> Engine {
        let mut config = self.xyce_engine_config(locked_time_grid);
        config.transient_initial_timestep = initial_timestep;
        config.integration_method = crate::analysis::IntegrationMethod::Trapezoidal;
        Engine::new(config)
    }

    fn create_xyce_static_tran_engine_with_integration_method(
        &self,
        locked_time_grid: Option<Vec<Value>>,
        integration_method: crate::analysis::IntegrationMethod,
        initial_timestep: Option<Value>,
    ) -> Engine {
        let mut config = self.xyce_engine_config(locked_time_grid);
        config.transient_initial_timestep = initial_timestep;
        config.integration_method = integration_method;
        Engine::new(config)
    }

    fn xyce_engine_config(&self, locked_time_grid: Option<Vec<Value>>) -> SimulationConfig {
        let defaults = SimulationConfig::default();
        SimulationConfig {
            max_iterations: defaults.max_iterations.max(1200),
            convergence_config: ConvergenceConfig::robust(),
            spice_dialect: SpiceDialect::Xyce,
            // Xyce and ngspice regression decks use 27 C unless overridden.
            temperature: 300.15,
            locked_time_grid: locked_time_grid.map(Arc::new),
            ..defaults
        }
    }

    fn transient_print_requests_linear_capacitor_branch_quantity(
        netlist: &Netlist,
        print: &XycePrintRequest,
    ) -> bool {
        print.probes.iter().any(|probe| {
            Self::transient_probe_requests_linear_capacitor_branch_quantity(netlist, probe)
        })
    }

    fn transient_probe_requests_linear_capacitor_branch_quantity(
        netlist: &Netlist,
        probe: &str,
    ) -> bool {
        let normalized = Self::normalize_probe(probe);
        if Self::normalized_probe_requests_linear_capacitor_branch_quantity(netlist, &normalized) {
            return true;
        }

        let Some(expression) = Self::print_expression_inner(probe) else {
            return false;
        };
        let normalized_expression = Self::normalize_probe(expression);
        if Self::braced_expression_is_atomic_real_probe(&normalized_expression, netlist) {
            return Self::normalized_probe_requests_linear_capacitor_branch_quantity(
                netlist,
                &normalized_expression,
            );
        }

        let mut found = false;
        let mut call_value = |call: &str| {
            if Self::normalized_probe_requests_linear_capacitor_branch_quantity(
                netlist,
                &Self::normalize_probe(call),
            ) {
                found = true;
            }
            Ok(1.0)
        };
        let _ = Self::evaluate_print_expression_with_probe_calls(
            expression,
            netlist.params.clone(),
            &mut call_value,
        );
        found
    }

    fn normalized_probe_requests_linear_capacitor_branch_quantity(
        netlist: &Netlist,
        normalized: &str,
    ) -> bool {
        if let Some(element_name) = Self::parse_current_probe(normalized)
            && Self::find_capacitor_element(netlist, &element_name).is_some()
        {
            return true;
        }
        if let Some(element_name) = Self::parse_power_probe(normalized)
            && Self::find_capacitor_element(netlist, &element_name).is_some()
        {
            return true;
        }
        false
    }

    fn compare_dc_prn_reference(
        &self,
        reference: &XycePrnTable,
        print: &XycePrintRequest,
        netlist: &Netlist,
        source: &str,
        dc: &XyceDcSweep,
        results: &[DcSweepPointResult],
    ) -> Result<Vec<XyceValueMismatch>, String> {
        let batches = [XyceDcResultBatch {
            netlist: netlist.clone(),
            results: results.to_vec(),
        }];
        self.compare_dc_prn_reference_batches(reference, print, source, dc, &batches)
    }

    fn compare_dc_data_prn_reference(
        &self,
        reference: &XycePrnTable,
        print: &XycePrintRequest,
        source: &str,
        dc: &XyceDcSweep,
        points: &[XyceDcDataPointResult],
    ) -> Result<Vec<XyceValueMismatch>, String> {
        if reference.columns.is_empty() {
            return Err("reference table has no columns".to_string());
        }
        let has_stepnum_column = reference.columns[0].eq_ignore_ascii_case("STEPNUM");
        let index_column = usize::from(has_stepnum_column);
        let has_index_column = reference
            .columns
            .get(index_column)
            .is_some_and(|column| column.eq_ignore_ascii_case("Index"));
        let data_column_offset = usize::from(has_stepnum_column) + usize::from(has_index_column);
        if data_column_offset == 0 && !Self::reference_columns_are_compact_probe_table(reference) {
            return Err(format!(
                "expected first Xyce .prn column to be Index, STEPNUM, or a compact probe label, got '{}'",
                reference.columns[0]
            ));
        }
        if reference.rows.len() != points.len() {
            return Err(format!(
                "reference row count ({}) does not match .DC DATA row count ({})",
                reference.rows.len(),
                points.len()
            ));
        }

        let mapping_netlist = points
            .first()
            .map(|point| &point.netlist)
            .ok_or_else(|| ".DC DATA comparison has no result rows".to_string())?;
        let data_columns = self.reference_data_columns(
            reference,
            print,
            mapping_netlist,
            data_column_offset,
            has_index_column,
        )?;
        let comp_tolerances = self.comp_tolerances(source, &data_columns)?;

        let mut mismatches = Vec::new();
        for (row_index, point) in points.iter().enumerate() {
            let row = reference
                .rows
                .get(row_index)
                .ok_or_else(|| format!("missing reference row for .DC DATA row {row_index}"))?;
            if row.len() != reference.columns.len() {
                return Err(format!(
                    "row {} has {} values, expected {}",
                    row_index,
                    row.len(),
                    reference.columns.len()
                ));
            }
            if has_stepnum_column {
                let expected_stepnum = row[0];
                if (expected_stepnum - 0.0).abs() > self.config.absolute_tolerance {
                    mismatches.push(XyceValueMismatch {
                        row: row_index,
                        probe: "STEPNUM".to_string(),
                        expected: expected_stepnum,
                        actual: 0.0,
                        relative_error: 1.0,
                    });
                    if mismatches.len() >= self.config.max_mismatches {
                        return Ok(mismatches);
                    }
                }
            }
            if has_index_column {
                let expected_index = row[index_column];
                let actual_index = row_index as Value;
                if (expected_index - actual_index).abs() > self.config.absolute_tolerance {
                    mismatches.push(XyceValueMismatch {
                        row: row_index,
                        probe: "Index".to_string(),
                        expected: expected_index,
                        actual: actual_index,
                        relative_error: 1.0,
                    });
                    if mismatches.len() >= self.config.max_mismatches {
                        return Ok(mismatches);
                    }
                }
            }

            let sweep_point = XyceDcSweepPoint {
                primary: point.point.sweep_value,
                secondary: None,
            };
            for (column_index, column) in data_columns.iter().enumerate() {
                let expected = row[column_index + data_column_offset];
                let (probe, actual) = match column {
                    XyceReferenceColumn::PrimarySweep { name } => {
                        (name.as_str(), sweep_point.primary)
                    }
                    XyceReferenceColumn::Probe { name } => (
                        name.as_str(),
                        Self::evaluate_dc_probe(
                            name,
                            &point.netlist,
                            dc,
                            sweep_point,
                            &point.point.result,
                            &point.point.device_op_report,
                        )?,
                    ),
                };
                let normalized_probe = Self::normalize_probe(probe);
                let tolerance = comp_tolerances
                    .get(&normalized_probe)
                    .copied()
                    .unwrap_or_else(|| self.default_comparison_tolerance(&normalized_probe));
                if let Some(relative_error) = self.value_mismatch(expected, actual, tolerance) {
                    mismatches.push(XyceValueMismatch {
                        row: row_index,
                        probe: probe.to_string(),
                        expected,
                        actual,
                        relative_error,
                    });
                    if mismatches.len() >= self.config.max_mismatches {
                        return Ok(mismatches);
                    }
                }
            }
        }

        Ok(mismatches)
    }

    fn compare_dc_prn_reference_batches(
        &self,
        reference: &XycePrnTable,
        print: &XycePrintRequest,
        source: &str,
        dc: &XyceDcSweep,
        batches: &[XyceDcResultBatch],
    ) -> Result<Vec<XyceValueMismatch>, String> {
        if reference.columns.is_empty() {
            return Err("reference table has no columns".to_string());
        }
        let has_stepnum_column = reference.columns[0].eq_ignore_ascii_case("STEPNUM");
        let index_column = usize::from(has_stepnum_column);
        let has_index_column = reference
            .columns
            .get(index_column)
            .is_some_and(|column| column.eq_ignore_ascii_case("Index"));
        let data_column_offset = usize::from(has_stepnum_column) + usize::from(has_index_column);
        if data_column_offset == 0 && !Self::reference_columns_are_compact_probe_table(reference) {
            return Err(format!(
                "expected first Xyce .prn column to be Index, STEPNUM, or a compact probe label, got '{}'",
                reference.columns[0]
            ));
        }
        let result_count = batches
            .iter()
            .map(|batch| batch.results.len())
            .sum::<usize>();
        if reference.rows.len() != result_count {
            return Err(format!(
                "reference row count ({}) does not match simulation point count ({})",
                reference.rows.len(),
                result_count
            ));
        }

        let mapping_netlist = batches
            .first()
            .map(|batch| &batch.netlist)
            .ok_or_else(|| "DC comparison has no result batches".to_string())?;
        let data_columns = self.reference_data_columns(
            reference,
            print,
            mapping_netlist,
            data_column_offset,
            has_index_column,
        )?;
        let comp_tolerances = self.comp_tolerances(source, &data_columns)?;
        let primary_points = dc.primary_spec().points();
        if primary_points.is_empty() {
            return Err("primary DC sweep has no points".to_string());
        }
        let secondary_points = dc.sweep2.as_ref().map(|sweep| sweep.spec().points());
        if secondary_points.as_ref().is_some_and(Vec::is_empty) {
            return Err("secondary DC sweep has no points".to_string());
        }
        let mut mismatches = Vec::new();
        let mut global_row_index = 0usize;
        for (batch_index, batch) in batches.iter().enumerate() {
            for (local_row_index, point) in batch.results.iter().enumerate() {
                let row = reference.rows.get(global_row_index).ok_or_else(|| {
                    format!("missing reference row for simulation row {global_row_index}")
                })?;
                if row.len() != reference.columns.len() {
                    return Err(format!(
                        "row {} has {} values, expected {}",
                        global_row_index,
                        row.len(),
                        reference.columns.len()
                    ));
                }
                if has_stepnum_column {
                    let expected_stepnum = row[0];
                    let actual_stepnum = batch_index as f64;
                    if (expected_stepnum - actual_stepnum).abs() > self.config.absolute_tolerance {
                        mismatches.push(XyceValueMismatch {
                            row: global_row_index,
                            probe: "STEPNUM".to_string(),
                            expected: expected_stepnum,
                            actual: actual_stepnum,
                            relative_error: 1.0,
                        });
                        if mismatches.len() >= self.config.max_mismatches {
                            return Ok(mismatches);
                        }
                    }
                }
                if has_index_column {
                    let expected_index = row[index_column];
                    let actual_index = local_row_index as f64;
                    if (expected_index - actual_index).abs() > self.config.absolute_tolerance {
                        mismatches.push(XyceValueMismatch {
                            row: global_row_index,
                            probe: "Index".to_string(),
                            expected: expected_index,
                            actual: actual_index,
                            relative_error: 1.0,
                        });
                        if mismatches.len() >= self.config.max_mismatches {
                            return Ok(mismatches);
                        }
                    }
                }
                let value_offset = data_column_offset;

                let sweep_point = XyceDcSweepPoint {
                    primary: point.sweep_value,
                    secondary: if let Some(points) = secondary_points.as_ref() {
                        let outer_index = local_row_index / primary_points.len();
                        Some(*points.get(outer_index).ok_or_else(|| {
                            format!(
                                "row {global_row_index} maps outside secondary DC sweep point count ({})",
                                points.len()
                            )
                        })?)
                    } else {
                        None
                    },
                };
                let point_netlist = Self::dc_sweep_point_netlist(&batch.netlist, dc, sweep_point)?;
                let probe_netlist = point_netlist.as_ref().unwrap_or(&batch.netlist);
                for (column_index, column) in data_columns.iter().enumerate() {
                    let expected = row[column_index + value_offset];
                    let (probe, actual) = match column {
                        XyceReferenceColumn::PrimarySweep { name } => {
                            (name.as_str(), sweep_point.primary)
                        }
                        XyceReferenceColumn::Probe { name } => (
                            name.as_str(),
                            Self::evaluate_dc_probe(
                                name,
                                probe_netlist,
                                dc,
                                sweep_point,
                                &point.result,
                                &point.device_op_report,
                            )?,
                        ),
                    };
                    let normalized_probe = Self::normalize_probe(probe);
                    let tolerance = comp_tolerances
                        .get(&normalized_probe)
                        .copied()
                        .unwrap_or_else(|| self.default_comparison_tolerance(&normalized_probe));
                    if let Some(relative_error) = self.value_mismatch(expected, actual, tolerance) {
                        mismatches.push(XyceValueMismatch {
                            row: global_row_index,
                            probe: probe.to_string(),
                            expected,
                            actual,
                            relative_error,
                        });
                        if mismatches.len() >= self.config.max_mismatches {
                            return Ok(mismatches);
                        }
                    }
                }
                global_row_index += 1;
            }
        }

        Ok(mismatches)
    }

    fn compare_ac_prn_reference(
        &self,
        reference: &XycePrnTable,
        print: &XycePrintRequest,
        netlist: &Netlist,
        source: &str,
        results: &[AcResult],
    ) -> Result<Vec<XyceValueMismatch>, String> {
        self.compare_ac_prn_reference_with_step(reference, print, netlist, source, results, None)
    }

    fn compare_ac_data_prn_reference(
        &self,
        reference: &XycePrnTable,
        print: &XycePrintRequest,
        source: &str,
        points: &[XyceAcDataPointResult],
    ) -> Result<Vec<XyceValueMismatch>, String> {
        if reference.columns.is_empty() {
            return Err("reference table has no columns".to_string());
        }

        let mut data_column_offset = 0usize;
        let stepnum_column = reference
            .columns
            .first()
            .is_some_and(|column| column.eq_ignore_ascii_case("STEPNUM"))
            .then_some(0usize);
        if stepnum_column.is_some() {
            data_column_offset += 1;
        }

        let index_column = reference
            .columns
            .get(data_column_offset)
            .is_some_and(|column| column.eq_ignore_ascii_case("Index"))
            .then_some(data_column_offset);
        if index_column.is_some() {
            data_column_offset += 1;
        }

        let frequency_column = reference
            .columns
            .get(data_column_offset)
            .filter(|column| Self::is_ac_frequency_reference_column(column))
            .map(|_| data_column_offset)
            .ok_or_else(|| {
                format!(
                    "expected Xyce .FD.prn frequency column at position {}, got '{}'",
                    data_column_offset,
                    reference
                        .columns
                        .get(data_column_offset)
                        .map(String::as_str)
                        .unwrap_or("<missing>")
                )
            })?;
        data_column_offset += 1;

        if reference.rows.len() != points.len() {
            return Err(format!(
                "reference row count ({}) does not match AC point count ({})",
                reference.rows.len(),
                points.len()
            ));
        }

        let data_columns = Self::reference_ac_data_columns(reference, print, data_column_offset)?;
        let phase_output_radians = Self::source_requests_ac_phase_output_radians(source);
        let comp_columns = data_columns
            .iter()
            .map(|column| XyceReferenceColumn::Probe {
                name: column.probe_name().to_string(),
            })
            .collect::<Vec<_>>();
        let comp_tolerances = self.comp_tolerances(source, &comp_columns)?;
        let frequency_tolerance = XyceComparisonTolerance::from_config(&self.config);

        let mut mismatches = Vec::new();
        for (row_index, (row, point)) in reference.rows.iter().zip(points).enumerate() {
            if row.len() != reference.columns.len() {
                return Err(format!(
                    "row {} has {} values, expected {}",
                    row_index,
                    row.len(),
                    reference.columns.len()
                ));
            }
            if let Some(stepnum_column) = stepnum_column {
                let expected_stepnum = row[stepnum_column];
                if (expected_stepnum - 0.0).abs() > self.config.absolute_tolerance {
                    mismatches.push(XyceValueMismatch {
                        row: row_index,
                        probe: "STEPNUM".to_string(),
                        expected: expected_stepnum,
                        actual: 0.0,
                        relative_error: 1.0,
                    });
                    if mismatches.len() >= self.config.max_mismatches {
                        return Ok(mismatches);
                    }
                }
            }
            if let Some(index_column) = index_column {
                let expected_index = row[index_column];
                let actual_index = row_index as Value;
                if (expected_index - actual_index).abs() > self.config.absolute_tolerance {
                    mismatches.push(XyceValueMismatch {
                        row: row_index,
                        probe: "Index".to_string(),
                        expected: expected_index,
                        actual: actual_index,
                        relative_error: 1.0,
                    });
                    if mismatches.len() >= self.config.max_mismatches {
                        return Ok(mismatches);
                    }
                }
            }

            let expected_frequency = row[frequency_column];
            if let Some(relative_error) = self.value_mismatch(
                expected_frequency,
                point.result.frequency,
                frequency_tolerance,
            ) {
                mismatches.push(XyceValueMismatch {
                    row: row_index,
                    probe: reference.columns[frequency_column].clone(),
                    expected: expected_frequency,
                    actual: point.result.frequency,
                    relative_error,
                });
                if mismatches.len() >= self.config.max_mismatches {
                    return Ok(mismatches);
                }
            }

            for (column_index, column) in data_columns.iter().enumerate() {
                let expected = row[column_index + data_column_offset];
                let actual = Self::evaluate_ac_reference_column(
                    column,
                    &point.netlist,
                    &point.result,
                    phase_output_radians,
                )?;
                let normalized_probe = Self::normalize_probe(column.probe_name());
                let tolerance = comp_tolerances
                    .get(&normalized_probe)
                    .copied()
                    .unwrap_or_else(|| self.default_comparison_tolerance(&normalized_probe));
                if let Some(relative_error) = self.value_mismatch(expected, actual, tolerance) {
                    mismatches.push(XyceValueMismatch {
                        row: row_index,
                        probe: reference.columns[column_index + data_column_offset].clone(),
                        expected,
                        actual,
                        relative_error,
                    });
                    if mismatches.len() >= self.config.max_mismatches {
                        return Ok(mismatches);
                    }
                }
            }
        }

        Ok(mismatches)
    }

    fn compare_ac_prn_reference_with_step(
        &self,
        reference: &XycePrnTable,
        print: &XycePrintRequest,
        netlist: &Netlist,
        source: &str,
        results: &[AcResult],
        expected_step_index: Option<usize>,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        if reference.columns.is_empty() {
            return Err("reference table has no columns".to_string());
        }

        let mut data_column_offset = 0usize;
        let stepnum_column = reference
            .columns
            .first()
            .is_some_and(|column| column.eq_ignore_ascii_case("STEPNUM"))
            .then_some(0usize);
        if stepnum_column.is_some() {
            data_column_offset += 1;
        }

        let index_column = reference
            .columns
            .get(data_column_offset)
            .is_some_and(|column| column.eq_ignore_ascii_case("Index"))
            .then_some(data_column_offset);
        if index_column.is_some() {
            data_column_offset += 1;
        }

        let frequency_column = reference
            .columns
            .get(data_column_offset)
            .filter(|column| Self::is_ac_frequency_reference_column(column))
            .map(|_| data_column_offset)
            .ok_or_else(|| {
                format!(
                    "expected Xyce .FD.prn frequency column at position {}, got '{}'",
                    data_column_offset,
                    reference
                        .columns
                        .get(data_column_offset)
                        .map(String::as_str)
                        .unwrap_or("<missing>")
                )
            })?;
        data_column_offset += 1;

        if reference.rows.len() != results.len() {
            return Err(format!(
                "reference row count ({}) does not match AC simulation point count ({})",
                reference.rows.len(),
                results.len()
            ));
        }

        let data_columns = Self::reference_ac_data_columns(reference, print, data_column_offset)?;
        let phase_output_radians = Self::source_requests_ac_phase_output_radians(source);
        let comp_columns = data_columns
            .iter()
            .map(|column| XyceReferenceColumn::Probe {
                name: column.probe_name().to_string(),
            })
            .collect::<Vec<_>>();
        let comp_tolerances = self.comp_tolerances(source, &comp_columns)?;
        let frequency_tolerance = XyceComparisonTolerance::from_config(&self.config);

        let mut mismatches = Vec::new();
        for (row_index, (row, result)) in reference.rows.iter().zip(results).enumerate() {
            if row.len() != reference.columns.len() {
                return Err(format!(
                    "row {} has {} values, expected {}",
                    row_index,
                    row.len(),
                    reference.columns.len()
                ));
            }
            if let Some(stepnum_column) = stepnum_column {
                let expected_stepnum = row[stepnum_column];
                let actual_stepnum = expected_step_index.unwrap_or(0) as f64;
                if (expected_stepnum - actual_stepnum).abs() > self.config.absolute_tolerance {
                    mismatches.push(XyceValueMismatch {
                        row: row_index,
                        probe: "STEPNUM".to_string(),
                        expected: expected_stepnum,
                        actual: actual_stepnum,
                        relative_error: 1.0,
                    });
                    if mismatches.len() >= self.config.max_mismatches {
                        return Ok(mismatches);
                    }
                }
            }
            if let Some(index_column) = index_column {
                let expected_index = row[index_column];
                let actual_index = row_index as f64;
                if (expected_index - actual_index).abs() > self.config.absolute_tolerance {
                    mismatches.push(XyceValueMismatch {
                        row: row_index,
                        probe: "Index".to_string(),
                        expected: expected_index,
                        actual: actual_index,
                        relative_error: 1.0,
                    });
                    if mismatches.len() >= self.config.max_mismatches {
                        return Ok(mismatches);
                    }
                }
            }

            let expected_frequency = row[frequency_column];
            if let Some(relative_error) =
                self.value_mismatch(expected_frequency, result.frequency, frequency_tolerance)
            {
                mismatches.push(XyceValueMismatch {
                    row: row_index,
                    probe: reference.columns[frequency_column].clone(),
                    expected: expected_frequency,
                    actual: result.frequency,
                    relative_error,
                });
                if mismatches.len() >= self.config.max_mismatches {
                    return Ok(mismatches);
                }
            }

            for (column_index, column) in data_columns.iter().enumerate() {
                let expected = row[column_index + data_column_offset];
                let actual = Self::evaluate_ac_reference_column(
                    column,
                    netlist,
                    result,
                    phase_output_radians,
                )?;
                let normalized_probe = Self::normalize_probe(column.probe_name());
                let tolerance = comp_tolerances
                    .get(&normalized_probe)
                    .copied()
                    .unwrap_or_else(|| self.default_comparison_tolerance(&normalized_probe));
                if let Some(relative_error) = self.value_mismatch(expected, actual, tolerance) {
                    mismatches.push(XyceValueMismatch {
                        row: row_index,
                        probe: reference.columns[column_index + data_column_offset].clone(),
                        expected,
                        actual,
                        relative_error,
                    });
                    if mismatches.len() >= self.config.max_mismatches {
                        return Ok(mismatches);
                    }
                }
            }
        }

        Ok(mismatches)
    }

    fn compare_tran_prn_reference(
        &self,
        reference: &XycePrnTable,
        print: &XycePrintRequest,
        netlist: &Netlist,
        source: &str,
        result: &TransientResult,
        wrapper_tolerance: Option<XyceComparisonTolerance>,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        let layout = Self::transient_reference_layout(reference)?;
        let data_columns =
            Self::reference_tran_data_columns(reference, print, layout.data_column_offset)?;
        let comp_columns = data_columns
            .iter()
            .map(|probe| XyceReferenceColumn::Probe {
                name: probe.clone(),
            })
            .collect::<Vec<_>>();
        let comp_tolerances = self.comp_tolerances(source, &comp_columns)?;
        let output_initial_interval = Self::output_initial_interval(source)?;
        let tran_time_scale_factor = Self::tran_print_time_scale_factor(source)?;
        Self::validate_transient_result_time_grid(result)?;

        let mut mismatches = Vec::new();
        for (row_index, row) in reference.rows.iter().enumerate() {
            if row.len() != reference.columns.len() {
                return Err(format!(
                    "row {} has {} values, expected {}",
                    row_index,
                    row.len(),
                    reference.columns.len()
                ));
            }
            if let Some(index_column) = layout.index_column {
                let expected_index = row[index_column];
                let actual_index = row_index as f64;
                if (expected_index - actual_index).abs() > self.config.absolute_tolerance {
                    mismatches.push(XyceValueMismatch {
                        row: row_index,
                        probe: "Index".to_string(),
                        expected: expected_index,
                        actual: actual_index,
                        relative_error: 1.0,
                    });
                    if mismatches.len() >= self.config.max_mismatches {
                        return Ok(mismatches);
                    }
                }
            }

            let reference_time = row[layout.time_column];
            if !reference_time.is_finite() {
                return Err(format!(
                    "row {row_index} has non-finite TIME value {reference_time}"
                ));
            }
            let time = reference_time / tran_time_scale_factor;

            for (column_index, probe) in data_columns.iter().enumerate() {
                let expected = row[column_index + layout.data_column_offset];
                let actual = Self::evaluate_tran_probe(probe, netlist, result, time)?;
                let normalized_probe = Self::normalize_probe(probe);
                let tolerance = comp_tolerances
                    .get(&normalized_probe)
                    .copied()
                    .or(wrapper_tolerance)
                    .unwrap_or_else(|| self.default_comparison_tolerance(&normalized_probe));
                if let Some(relative_error) = self.value_mismatch(expected, actual, tolerance) {
                    let time_tolerance = Self::default_prn_time_quantization_tolerance(time);
                    if time_tolerance > 0.0
                        && self.transient_probe_matches_within_time_quantization(
                            probe,
                            netlist,
                            result,
                            time,
                            expected,
                            actual,
                            tolerance,
                            time_tolerance,
                        )?
                    {
                        continue;
                    }
                    if self.transient_probe_matches_reference_time_neighborhood(
                        reference,
                        layout.time_column,
                        row_index,
                        column_index + layout.data_column_offset,
                        actual,
                        tolerance,
                        time_tolerance,
                        tran_time_scale_factor,
                    ) {
                        continue;
                    }
                    if let Some(output_interval) = output_initial_interval
                        && self.transient_probe_matches_output_interval_corridor(
                            probe,
                            netlist,
                            result,
                            reference,
                            layout.time_column,
                            row_index,
                            expected,
                            tolerance,
                            output_interval,
                            tran_time_scale_factor,
                        )?
                    {
                        continue;
                    }
                    mismatches.push(XyceValueMismatch {
                        row: row_index,
                        probe: probe.clone(),
                        expected,
                        actual,
                        relative_error,
                    });
                    if mismatches.len() >= self.config.max_mismatches {
                        return Ok(mismatches);
                    }
                }
            }
        }

        Ok(mismatches)
    }

    fn transient_probe_matches_reference_time_neighborhood(
        &self,
        reference: &XycePrnTable,
        time_column: usize,
        row_index: usize,
        data_column: usize,
        actual: Value,
        tolerance: XyceComparisonTolerance,
        time_tolerance: Value,
        time_scale_factor: Value,
    ) -> bool {
        if !actual.is_finite()
            || !time_tolerance.is_finite()
            || time_tolerance < 0.0
            || !time_scale_factor.is_finite()
            || time_scale_factor <= 0.0
        {
            return false;
        }
        let Some(row) = reference.rows.get(row_index) else {
            return false;
        };
        let Some(time) = row
            .get(time_column)
            .copied()
            .map(|reference_time| reference_time / time_scale_factor)
        else {
            return false;
        };
        if !time.is_finite() {
            return false;
        }

        let mut first_row = row_index;
        while first_row > 0
            && Self::reference_time_is_in_prn_neighborhood(
                time,
                reference.rows[first_row - 1]
                    .get(time_column)
                    .copied()
                    .map(|reference_time| reference_time / time_scale_factor),
                time_tolerance,
            )
        {
            first_row -= 1;
        }

        let mut last_row = row_index;
        while last_row + 1 < reference.rows.len()
            && Self::reference_time_is_in_prn_neighborhood(
                time,
                reference.rows[last_row + 1]
                    .get(time_column)
                    .copied()
                    .map(|reference_time| reference_time / time_scale_factor),
                time_tolerance,
            )
        {
            last_row += 1;
        }

        if first_row == last_row {
            return false;
        }

        let mut min_reference = Value::INFINITY;
        let mut max_reference = Value::NEG_INFINITY;
        let mut finite_values = 0usize;
        for row in &reference.rows[first_row..=last_row] {
            let Some(&value) = row.get(data_column) else {
                continue;
            };
            if value.is_finite() {
                finite_values += 1;
                min_reference = min_reference.min(value);
                max_reference = max_reference.max(value);
            }
        }

        finite_values >= 2
            && min_reference.is_finite()
            && max_reference.is_finite()
            && (actual >= min_reference && actual <= max_reference
                || self
                    .value_mismatch(min_reference, actual, tolerance)
                    .is_none()
                || self
                    .value_mismatch(max_reference, actual, tolerance)
                    .is_none())
    }

    fn reference_time_is_in_prn_neighborhood(
        anchor_time: Value,
        candidate_time: Option<Value>,
        time_tolerance: Value,
    ) -> bool {
        let Some(candidate_time) = candidate_time else {
            return false;
        };
        if !anchor_time.is_finite() || !candidate_time.is_finite() {
            return false;
        }
        if candidate_time == anchor_time {
            return true;
        }
        let candidate_tolerance = Self::default_prn_time_quantization_tolerance(candidate_time);
        let neighborhood = time_tolerance.max(candidate_tolerance) * PRN_TIME_NEIGHBOR_HALF_ULPS;
        let binary_roundoff = Value::EPSILON * anchor_time.abs().max(candidate_time.abs());
        neighborhood > 0.0 && (candidate_time - anchor_time).abs() <= neighborhood + binary_roundoff
    }

    fn transient_probe_matches_output_interval_corridor(
        &self,
        probe: &str,
        netlist: &Netlist,
        result: &TransientResult,
        reference: &XycePrnTable,
        time_column: usize,
        row_index: usize,
        expected: Value,
        tolerance: XyceComparisonTolerance,
        output_interval: Value,
        time_scale_factor: Value,
    ) -> Result<bool, String> {
        if !expected.is_finite()
            || !output_interval.is_finite()
            || output_interval <= 0.0
            || !time_scale_factor.is_finite()
            || time_scale_factor <= 0.0
        {
            return Ok(false);
        }
        let Some(row) = reference.rows.get(row_index) else {
            return Ok(false);
        };
        let Some(time) = row
            .get(time_column)
            .copied()
            .map(|reference_time| reference_time / time_scale_factor)
        else {
            return Ok(false);
        };
        if !time.is_finite() {
            return Ok(false);
        }

        let lower_time = reference.rows[..row_index]
            .iter()
            .rev()
            .filter_map(|row| row.get(time_column).copied())
            .map(|reference_time| reference_time / time_scale_factor)
            .find(|candidate| candidate.is_finite() && *candidate < time);
        let upper_time = reference.rows[row_index + 1..]
            .iter()
            .filter_map(|row| row.get(time_column).copied())
            .map(|reference_time| reference_time / time_scale_factor)
            .find(|candidate| candidate.is_finite() && *candidate > time);
        let (Some(lower_time), Some(upper_time)) = (lower_time, upper_time) else {
            return Ok(false);
        };
        let window = upper_time - lower_time;
        if !window.is_finite() || window <= 0.0 || window > 2.5 * output_interval {
            return Ok(false);
        }

        let lower_value = Self::evaluate_tran_probe(probe, netlist, result, lower_time)?;
        let center_value = Self::evaluate_tran_probe(probe, netlist, result, time)?;
        let upper_value = Self::evaluate_tran_probe(probe, netlist, result, upper_time)?;
        if [lower_value, center_value, upper_value]
            .into_iter()
            .any(|candidate| {
                candidate.is_finite()
                    && self
                        .value_mismatch(expected, candidate, tolerance)
                        .is_none()
            })
        {
            return Ok(true);
        }

        let mut min_value = lower_value.min(center_value).min(upper_value);
        let mut max_value = lower_value.max(center_value).max(upper_value);
        for &sample_time in result.time.iter() {
            if sample_time <= lower_time || sample_time >= upper_time {
                continue;
            }
            let sample_value = Self::evaluate_tran_probe(probe, netlist, result, sample_time)?;
            if sample_value.is_finite() {
                min_value = min_value.min(sample_value);
                max_value = max_value.max(sample_value);
            }
        }

        Ok(min_value.is_finite()
            && max_value.is_finite()
            && expected >= min_value.min(max_value)
            && expected <= min_value.max(max_value))
    }

    fn transient_probe_matches_within_time_quantization(
        &self,
        probe: &str,
        netlist: &Netlist,
        result: &TransientResult,
        time: Value,
        expected: Value,
        actual: Value,
        tolerance: XyceComparisonTolerance,
        time_tolerance: Value,
    ) -> Result<bool, String> {
        let Some((&first_time, &last_time)) = result.time.first().zip(result.time.last()) else {
            return Ok(false);
        };
        let mut min_actual = actual;
        let mut max_actual = actual;
        for candidate_time in [time - time_tolerance, time + time_tolerance] {
            if candidate_time < first_time || candidate_time > last_time {
                continue;
            }
            let candidate = Self::evaluate_tran_probe(probe, netlist, result, candidate_time)?;
            if self
                .value_mismatch(expected, candidate, tolerance)
                .is_none()
            {
                return Ok(true);
            }
            if candidate.is_finite() {
                min_actual = min_actual.min(candidate);
                max_actual = max_actual.max(candidate);
            }
        }

        // A Xyce PRN timestamp is already rounded text. Around very steep
        // transitions, the oracle value can belong to the same printed-time
        // neighborhood while the closest simulator samples sit on adjacent
        // printed ticks. Bound the comparison with those immediate local
        // samples without accepting coarse-grid timing drift.
        let sample_window = time_tolerance * PRN_TIME_NEIGHBOR_HALF_ULPS;
        let lower_time = time - sample_window;
        let upper_time = time + sample_window;
        let first_sample = result.time.partition_point(|sample| *sample < lower_time);
        for &sample_time in result.time.iter().skip(first_sample) {
            if sample_time > upper_time {
                break;
            }
            let candidate = Self::evaluate_tran_probe(probe, netlist, result, sample_time)?;
            if self
                .value_mismatch(expected, candidate, tolerance)
                .is_none()
            {
                return Ok(true);
            }
            if candidate.is_finite() {
                min_actual = min_actual.min(candidate);
                max_actual = max_actual.max(candidate);
            }
        }

        Ok(expected.is_finite()
            && min_actual.is_finite()
            && max_actual.is_finite()
            && expected >= min_actual.min(max_actual)
            && expected <= min_actual.max(max_actual))
    }

    fn output_initial_interval(source: &str) -> Result<Option<Value>, String> {
        let mut interval: Option<Value> = None;
        for line in Self::logical_netlist_lines(source) {
            let trimmed = Self::strip_netlist_comment(&line).trim().to_string();
            if trimmed.is_empty() {
                continue;
            }
            let Some(command) = trimmed.split_whitespace().next() else {
                continue;
            };
            if !command.eq_ignore_ascii_case(".options") {
                continue;
            }

            let tokens = Self::split_grouped_whitespace_fields(&trimmed, ".OPTIONS statement")?;
            let token_refs = tokens.iter().map(String::as_str).collect::<Vec<_>>();
            let has_output_package = token_refs
                .iter()
                .skip(1)
                .any(|token| token.eq_ignore_ascii_case("output"));
            if !has_output_package {
                continue;
            }

            let mut index = 1usize;
            while index < token_refs.len() {
                if let Some((raw_key, raw_value, consumed)) =
                    Self::print_option_assignment(&token_refs, index)
                {
                    if raw_key.trim().eq_ignore_ascii_case("initial_interval") {
                        let parsed = crate::netlist::lexer::parse_spice_value(
                            raw_value.trim().trim_matches(['"', '\'']),
                        )
                        .map_err(|err| {
                            format!(
                                "failed to parse OUTPUT INITIAL_INTERVAL value '{}': {err}",
                                raw_value.trim()
                            )
                        })?;
                        if !parsed.is_finite() || parsed <= 0.0 {
                            return Err(format!(
                                "OUTPUT INITIAL_INTERVAL must be positive and finite, got {parsed}"
                            ));
                        }
                        if let Some(existing) = interval {
                            let scale = existing.abs().max(parsed.abs()).max(1.0);
                            if (existing - parsed).abs() > 1.0e-12 * scale {
                                return Err(
                                    "conflicting OUTPUT INITIAL_INTERVAL options are not supported"
                                        .to_string(),
                                );
                            }
                        } else {
                            interval = Some(parsed);
                        }
                    }
                    index += consumed;
                } else {
                    index += 1;
                }
            }
        }

        Ok(interval)
    }

    fn tran_print_time_scale_factor(source: &str) -> Result<Value, String> {
        let mut factor: Option<Value> = None;
        for line in Self::logical_netlist_lines(source) {
            let trimmed = Self::strip_netlist_comment(&line).trim().to_string();
            if trimmed.is_empty() {
                continue;
            }
            let Some(command) = trimmed.split_whitespace().next() else {
                continue;
            };
            if !command.eq_ignore_ascii_case(".print") {
                continue;
            }

            let tokens = Self::split_print_fields(&trimmed)?;
            let token_refs = tokens.iter().map(String::as_str).collect::<Vec<_>>();
            if token_refs
                .get(1)
                .is_none_or(|analysis| !analysis.eq_ignore_ascii_case("TRAN"))
            {
                continue;
            }

            let mut index = 2usize;
            while index < token_refs.len() {
                if let Some((raw_key, raw_value, consumed)) =
                    Self::print_option_assignment(&token_refs, index)
                {
                    if raw_key.trim().eq_ignore_ascii_case("TIMESCALEFACTOR") {
                        let parsed = crate::netlist::lexer::parse_spice_value(
                            raw_value.trim().trim_matches(['"', '\'']),
                        )
                        .map_err(|err| {
                            format!(
                                "failed to parse .PRINT TRAN TIMESCALEFACTOR value '{}': {err}",
                                raw_value.trim()
                            )
                        })?;
                        if !parsed.is_finite() || parsed <= 0.0 {
                            return Err(format!(
                                ".PRINT TRAN TIMESCALEFACTOR must be positive and finite, got {parsed}"
                            ));
                        }
                        if let Some(existing) = factor {
                            let scale = existing.abs().max(parsed.abs()).max(1.0);
                            if (existing - parsed).abs() > 1.0e-12 * scale {
                                return Err(
                                    "conflicting .PRINT TRAN TIMESCALEFACTOR options are not supported"
                                        .to_string(),
                                );
                            }
                        } else {
                            factor = Some(parsed);
                        }
                    }
                    index += consumed;
                } else {
                    index += 1;
                }
            }
        }

        Ok(factor.unwrap_or(1.0))
    }

    #[cfg(test)]
    fn line_declares_output_snapshots(line: &str) -> Result<bool, String> {
        let tokens = Self::split_grouped_whitespace_fields(line, ".OPTIONS statement")?;
        let token_refs = tokens.iter().map(String::as_str).collect::<Vec<_>>();
        if !token_refs
            .iter()
            .skip(1)
            .any(|token| token.eq_ignore_ascii_case("output"))
        {
            return Ok(false);
        }

        let mut index = 1usize;
        while index < token_refs.len() {
            if let Some((raw_key, raw_value, consumed)) =
                Self::print_option_assignment(&token_refs, index)
            {
                if raw_key.trim().eq_ignore_ascii_case("snapshots") {
                    let value = raw_value.trim().trim_matches(['"', '\'']);
                    return Ok(!matches!(
                        value.to_ascii_lowercase().as_str(),
                        "0" | "false" | "no" | "off"
                    ));
                }
                index += consumed;
            } else {
                index += 1;
            }
        }

        Ok(false)
    }

    fn reference_tran_data_columns(
        reference: &XycePrnTable,
        print: &XycePrintRequest,
        first_data_column: usize,
    ) -> Result<Vec<String>, String> {
        let mut data_columns =
            Vec::with_capacity(reference.columns.len().saturating_sub(first_data_column));
        let mut probe_index = 0usize;
        for column in reference.columns.iter().skip(first_data_column) {
            if let Some(probe) = print.probes.get(probe_index)
                && Self::reference_column_matches_probe(column, probe)
            {
                data_columns.push(probe.clone());
                probe_index += 1;
                continue;
            }

            data_columns.push(column.clone());
        }
        if probe_index != print.probes.len() {
            return Err(format!(
                "reference table matched {} .PRINT TRAN probe(s), but deck requested {}",
                probe_index,
                print.probes.len()
            ));
        }
        Ok(data_columns)
    }

    fn transient_max_step_for_reference(
        netlist: &Netlist,
        tran: &XyceTranAnalysis,
        reference: &XycePrnTable,
    ) -> Result<Value, String> {
        let requested = tran.max_step.or_else(|| {
            (tran.step > 0.0)
                .then_some(tran.step)
                .and_then(|step| Self::feasible_oracle_limited_step(tran, step))
        });
        let reference_step = Self::reference_min_positive_time_step(reference)?;
        let source_step = Self::source_transient_max_step(netlist, tran)
            .and_then(|step| Self::feasible_oracle_limited_step(tran, step));
        let fallback = (tran.stop / 1000.0).max(f64::MIN_POSITIVE);
        let reference_limited_step = Self::feasible_reference_limited_step(tran, reference_step);
        let fallback_limit = reference_limited_step.is_none().then_some(fallback);
        let max_step = [
            requested,
            reference_limited_step,
            source_step,
            fallback_limit,
        ]
        .into_iter()
        .flatten()
        .filter(|value| value.is_finite() && *value > 0.0)
        .reduce(Value::min)
        .unwrap_or(fallback);
        if !max_step.is_finite() || max_step <= 0.0 {
            return Err(format!(
                "resolved transient maximum step must be finite and positive, got {max_step}"
            ));
        }
        let estimated_steps = (tran.stop / max_step).ceil();
        if estimated_steps > MAX_NATIVE_TRAN_ORACLE_STEPS {
            return Err(format!(
                "transient harness execution envelope supports at most {:.0} oracle-derived native step(s), but this deck requires about {:.0}",
                MAX_NATIVE_TRAN_ORACLE_STEPS, estimated_steps
            ));
        }
        Self::validate_transient_execution_envelope(netlist, estimated_steps)?;
        Ok(max_step)
    }

    fn validate_transient_execution_envelope(
        netlist: &Netlist,
        estimated_steps: Value,
    ) -> Result<(), String> {
        let size = Self::transient_flattened_problem_size(netlist)?;
        Self::validate_transient_problem_size_envelope(size, estimated_steps)
    }

    fn validate_transient_preflight_execution_envelope(
        netlist: &Netlist,
        tran: &XyceTranAnalysis,
    ) -> Result<(), String> {
        let estimated_steps = Self::preflight_transient_estimated_steps(netlist, tran);
        let size = Self::transient_hierarchy_problem_size_estimate(netlist)?;
        Self::validate_transient_problem_size_envelope(size, estimated_steps)
    }

    fn validate_transient_problem_size_envelope(
        size: XyceTransientProblemSize,
        estimated_steps: Value,
    ) -> Result<(), String> {
        let estimated_element_steps = estimated_steps * size.element_count as Value;
        if estimated_element_steps > MAX_NATIVE_TRAN_ELEMENT_STEPS {
            return Err(format!(
                "transient harness execution envelope supports at most {:.0} native element-step unit(s), but this deck requires about {:.0} ({} flattened element(s) across about {:.0} step(s))",
                MAX_NATIVE_TRAN_ELEMENT_STEPS,
                estimated_element_steps,
                size.element_count,
                estimated_steps
            ));
        }
        let estimated_node_solve_steps =
            estimated_steps * (size.node_count as Value) * (size.node_count as Value);
        if estimated_node_solve_steps > MAX_NATIVE_TRAN_NODE_SOLVE_STEPS {
            return Err(format!(
                "transient harness execution envelope supports at most {:.0} native node-solve step unit(s), but this deck requires about {:.0} ({} flattened node(s) across about {:.0} step(s))",
                MAX_NATIVE_TRAN_NODE_SOLVE_STEPS,
                estimated_node_solve_steps,
                size.node_count,
                estimated_steps
            ));
        }
        Ok(())
    }

    fn preflight_transient_estimated_steps(netlist: &Netlist, tran: &XyceTranAnalysis) -> Value {
        let requested = tran.max_step.or_else(|| {
            (tran.step > 0.0)
                .then_some(tran.step)
                .and_then(|step| Self::feasible_oracle_limited_step(tran, step))
        });
        let source_step = Self::source_transient_max_step(netlist, tran)
            .and_then(|step| Self::feasible_oracle_limited_step(tran, step));
        let fallback = (tran.stop / 1000.0).max(f64::MIN_POSITIVE);
        let max_step = [requested, source_step, Some(fallback)]
            .into_iter()
            .flatten()
            .filter(|value| value.is_finite() && *value > 0.0)
            .reduce(Value::min)
            .unwrap_or(fallback);
        (tran.stop / max_step).ceil()
    }

    fn transient_flattened_problem_size(
        netlist: &Netlist,
    ) -> Result<XyceTransientProblemSize, String> {
        let elements = if netlist
            .elements
            .iter()
            .any(|element| matches!(element.kind, ElementKind::Subcircuit { .. }))
        {
            crate::netlist::flatten_netlist_with_models(netlist)
                .map(|flattened| flattened.elements)
                .map_err(|err| {
                format!("transient harness execution envelope could not flatten subcircuits for native problem-size estimation: {err}")
            })?
        } else {
            netlist.elements.clone()
        };

        let mut nodes = BTreeSet::new();
        for element in &elements {
            for node in &element.nodes {
                if !Self::node_name_is_ground(node) {
                    nodes.insert(node.to_ascii_lowercase());
                }
            }
        }

        Ok(XyceTransientProblemSize {
            element_count: elements.len(),
            node_count: nodes.len(),
        })
    }

    fn transient_hierarchy_problem_size_estimate(
        netlist: &Netlist,
    ) -> Result<XyceTransientProblemSize, String> {
        let mut subcircuits = BTreeMap::new();
        for subcircuit in &netlist.subcircuits {
            Self::collect_subcircuit_defs(subcircuit, &mut subcircuits);
        }

        let mut top_nodes = BTreeSet::new();
        let mut element_count = 0usize;
        let mut internal_node_count = 0usize;
        let mut stack = BTreeSet::new();

        for element in &netlist.elements {
            for node in &element.nodes {
                if !Self::node_name_is_ground(node) {
                    top_nodes.insert(node.to_ascii_lowercase());
                }
            }
            if let ElementKind::Subcircuit { subckt_name, .. } = &element.kind {
                let subcircuit =
                    subcircuits
                        .get(&subckt_name.to_ascii_lowercase())
                        .ok_or_else(|| {
                            format!(
                                "transient harness execution envelope cannot estimate unresolved subcircuit '{}'",
                                subckt_name
                            )
                        })?;
                let size =
                    Self::subcircuit_problem_size_estimate(subcircuit, &subcircuits, &mut stack)?;
                element_count += size.element_count;
                internal_node_count += size.node_count;
            } else {
                element_count += 1;
            }
        }

        Ok(XyceTransientProblemSize {
            element_count,
            node_count: top_nodes.len() + internal_node_count,
        })
    }

    fn collect_subcircuit_defs<'a>(
        subcircuit: &'a SubcircuitDef,
        defs: &mut BTreeMap<String, &'a SubcircuitDef>,
    ) {
        defs.insert(subcircuit.name.to_ascii_lowercase(), subcircuit);
        for nested in &subcircuit.nested_subcircuits {
            Self::collect_subcircuit_defs(nested, defs);
        }
    }

    fn subcircuit_problem_size_estimate(
        subcircuit: &SubcircuitDef,
        defs: &BTreeMap<String, &SubcircuitDef>,
        stack: &mut BTreeSet<String>,
    ) -> Result<XyceTransientProblemSize, String> {
        let key = subcircuit.name.to_ascii_lowercase();
        if !stack.insert(key.clone()) {
            return Err(format!(
                "transient harness execution envelope cannot estimate recursive subcircuit '{}'",
                subcircuit.name
            ));
        }

        let ports = subcircuit
            .ports
            .iter()
            .map(|port| port.to_ascii_lowercase())
            .collect::<BTreeSet<_>>();
        let mut local_nodes = BTreeSet::new();
        let mut element_count = 0usize;
        let mut internal_node_count = 0usize;

        for element in &subcircuit.elements {
            for node in &element.nodes {
                let normalized = node.to_ascii_lowercase();
                if !Self::node_name_is_ground(node) && !ports.contains(&normalized) {
                    local_nodes.insert(normalized);
                }
            }
            if let ElementKind::Subcircuit { subckt_name, .. } = &element.kind {
                let child = defs.get(&subckt_name.to_ascii_lowercase()).ok_or_else(|| {
                    format!(
                        "transient harness execution envelope cannot estimate unresolved subcircuit '{}'",
                        subckt_name
                    )
                })?;
                let size = Self::subcircuit_problem_size_estimate(child, defs, stack)?;
                element_count += size.element_count;
                internal_node_count += size.node_count;
            } else {
                element_count += 1;
            }
        }

        stack.remove(&key);
        Ok(XyceTransientProblemSize {
            element_count,
            node_count: local_nodes.len() + internal_node_count,
        })
    }

    fn node_name_is_ground(node: &str) -> bool {
        let normalized = node.trim();
        normalized == "0"
            || normalized.eq_ignore_ascii_case("gnd")
            || normalized.eq_ignore_ascii_case("ground")
    }

    fn xyce_initial_timestep_for_tran(tran: &XyceTranAnalysis) -> Option<Value> {
        let step = if tran.step.is_finite() && tran.step > 0.0 {
            tran.step
        } else {
            1.0e-10
        };
        Some(step.max(1.0e-30))
    }

    fn feasible_reference_limited_step(
        tran: &XyceTranAnalysis,
        reference_step: Option<Value>,
    ) -> Option<Value> {
        let reference_step =
            reference_step.filter(|step| step.is_finite() && *step > f64::MIN_POSITIVE)?;

        // The Xyce accepted cadence can be part of the oracle for dynamic
        // decks, so use the reference minimum spacing when it is affordable.
        // Some Xyce references contain a tiny adaptive gap in an otherwise
        // coarse table; those must fall back to source/requested/final-time
        // limits and be compared by interpolation instead of forcing millions
        // of native steps.
        Self::feasible_oracle_limited_step(tran, reference_step)
    }

    fn feasible_oracle_limited_step(tran: &XyceTranAnalysis, step: Value) -> Option<Value> {
        let step = (step.is_finite() && step > f64::MIN_POSITIVE).then_some(step)?;
        let estimated_steps = (tran.stop / step).ceil();
        (estimated_steps <= MAX_NATIVE_TRAN_ORACLE_STEPS).then_some(step)
    }

    fn source_transient_max_step(netlist: &Netlist, tran: &XyceTranAnalysis) -> Option<Value> {
        netlist
            .elements
            .iter()
            .filter_map(|element| match &element.kind {
                ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => {
                    Self::source_spec_transient_max_step(spec, tran)
                }
                _ => None,
            })
            .filter(|step| step.is_finite() && *step > 0.0)
            .reduce(Value::min)
    }

    fn source_spec_transient_max_step(
        spec: &crate::netlist::SourceSpec,
        tran: &XyceTranAnalysis,
    ) -> Option<Value> {
        match spec {
            crate::netlist::SourceSpec::RfPort { inner, .. } => {
                Self::source_spec_transient_max_step(inner, tran)
            }
            crate::netlist::SourceSpec::Dc(_)
            | crate::netlist::SourceSpec::Ac { .. }
            | crate::netlist::SourceSpec::DcAc { .. }
            | crate::netlist::SourceSpec::Pwl { .. }
            | crate::netlist::SourceSpec::PwlFile { .. }
            | crate::netlist::SourceSpec::TrNoise { .. } => None,
            crate::netlist::SourceSpec::DcTransient { transient, .. }
            | crate::netlist::SourceSpec::DcAcTransient { transient, .. } => {
                Self::source_spec_transient_max_step(transient, tran)
            }
            crate::netlist::SourceSpec::Pulse {
                rise,
                fall,
                width,
                period,
                width_defaults_to_zero,
                ..
            } => {
                let tstep_hint = if tran.step.is_finite() && tran.step > 0.0 {
                    tran.step
                } else {
                    (tran.stop / 1000.0).max(f64::MIN_POSITIVE)
                };
                let (_delay, resolved_rise, resolved_fall, resolved_width, resolved_period) =
                    crate::circuit::VoltageSources::resolve_pulse_timing_with_defaults(
                        0.0,
                        *rise,
                        *fall,
                        *width,
                        *period,
                        *width_defaults_to_zero,
                        tstep_hint,
                        tran.stop.max(f64::MIN_POSITIVE),
                        SpiceDialect::Xyce,
                    );
                [
                    Self::positive_duration_step(
                        resolved_rise,
                        TRAN_ORACLE_STEPS_PER_SOURCE_TRANSITION,
                    ),
                    Self::positive_duration_step(
                        resolved_fall,
                        TRAN_ORACLE_STEPS_PER_SOURCE_TRANSITION,
                    ),
                    Self::positive_duration_step(
                        resolved_width,
                        TRAN_ORACLE_STEPS_PER_SOURCE_PERIOD,
                    ),
                    Self::positive_duration_step(
                        resolved_period,
                        TRAN_ORACLE_STEPS_PER_SOURCE_PERIOD,
                    ),
                ]
                .into_iter()
                .flatten()
                .reduce(Value::min)
            }
            crate::netlist::SourceSpec::Pat {
                rise, fall, sample, ..
            } => [
                Self::positive_duration_step(*rise, TRAN_ORACLE_STEPS_PER_SOURCE_TRANSITION),
                Self::positive_duration_step(*fall, TRAN_ORACLE_STEPS_PER_SOURCE_TRANSITION),
                Self::positive_duration_step(*sample, TRAN_ORACLE_STEPS_PER_SOURCE_PERIOD),
            ]
            .into_iter()
            .flatten()
            .reduce(Value::min),
            crate::netlist::SourceSpec::Exp {
                tau1,
                tau2,
                td1,
                td2,
                ..
            } => [
                Self::positive_duration_step(*tau1, TRAN_ORACLE_STEPS_PER_SOURCE_TRANSITION),
                Self::positive_duration_step(*tau2, TRAN_ORACLE_STEPS_PER_SOURCE_TRANSITION),
                Self::positive_duration_step(*td2 - *td1, TRAN_ORACLE_STEPS_PER_SOURCE_PERIOD),
            ]
            .into_iter()
            .flatten()
            .reduce(Value::min),
            crate::netlist::SourceSpec::Sin { frequency, .. } => {
                Self::positive_frequency_step(Self::resolved_sin_frequency(*frequency, tran.stop))
            }
            crate::netlist::SourceSpec::Sffm {
                carrier_freq,
                signal_freq,
                ..
            } => Self::positive_frequency_step(
                Self::resolved_modulated_frequency(*carrier_freq, 5.0, tran.stop).max(
                    Self::resolved_modulated_frequency(*signal_freq, 500.0, tran.stop),
                ),
            ),
            crate::netlist::SourceSpec::Am {
                modulating_freq,
                carrier_freq,
                ..
            } => Self::positive_frequency_step(
                Self::resolved_modulated_frequency(*carrier_freq, 500.0, tran.stop).max(
                    Self::resolved_modulated_frequency(*modulating_freq, 5.0, tran.stop),
                ),
            ),
        }
    }

    fn positive_duration_step(duration: Value, points_per_duration: Value) -> Option<Value> {
        (duration.is_finite() && duration > 0.0 && points_per_duration > 0.0)
            .then_some(duration / points_per_duration)
    }

    fn positive_frequency_step(frequency: Value) -> Option<Value> {
        (frequency.is_finite() && frequency > 0.0)
            .then_some(1.0 / (frequency * TRAN_ORACLE_STEPS_PER_SOURCE_PERIOD))
    }

    fn resolved_sin_frequency(frequency: Value, tstop: Value) -> Value {
        if frequency.is_finite() && frequency > 0.0 {
            frequency
        } else if tstop.is_finite() && tstop > 0.0 {
            1.0 / tstop
        } else {
            1.0e3
        }
    }

    fn resolved_modulated_frequency(
        frequency: Value,
        default_cycles: Value,
        tstop: Value,
    ) -> Value {
        if frequency.is_finite() && frequency > 0.0 {
            frequency
        } else if tstop.is_finite() && tstop > 0.0 {
            default_cycles / tstop
        } else {
            default_cycles * 1.0e3
        }
    }

    fn reference_min_positive_time_step(reference: &XycePrnTable) -> Result<Option<Value>, String> {
        let time_column = Self::reference_time_column_index(reference)
            .ok_or_else(|| "reference table has no TIME column".to_string())?;
        let mut previous = None;
        let mut min_step: Option<Value> = None;
        for (row_index, row) in reference.rows.iter().enumerate() {
            let time = *row.get(time_column).ok_or_else(|| {
                format!("row {row_index} has no TIME column at index {time_column}")
            })?;
            if !time.is_finite() {
                return Err(format!("row {row_index} has non-finite TIME value {time}"));
            }
            if let Some(previous_time) = previous {
                let step = time - previous_time;
                if step < 0.0 {
                    return Err(format!(
                        "reference TIME column is not monotonic at row {row_index}"
                    ));
                }
                if step > 0.0 {
                    min_step = Some(min_step.map_or(step, |current| current.min(step)));
                }
            }
            previous = Some(time);
        }
        Ok(min_step)
    }

    fn reference_time_grid(reference: &XycePrnTable) -> Result<Vec<Value>, String> {
        let time_column = Self::reference_time_column_index(reference)
            .ok_or_else(|| "reference table has no TIME column".to_string())?;
        let mut previous = None;
        let mut grid = Vec::with_capacity(reference.rows.len());
        for (row_index, row) in reference.rows.iter().enumerate() {
            let time = *row.get(time_column).ok_or_else(|| {
                format!("row {row_index} has no TIME column at index {time_column}")
            })?;
            if !time.is_finite() {
                return Err(format!("row {row_index} has non-finite TIME value {time}"));
            }
            if let Some(previous_time) = previous
                && time < previous_time
            {
                return Err(format!(
                    "reference TIME column is not monotonic at row {row_index}"
                ));
            }
            grid.push(time);
            previous = Some(time);
        }
        Ok(grid)
    }

    fn tran_analysis_for_reference_stop(
        contract: XyceStaticTranContract,
        tran: XyceTranAnalysis,
        reference_time_grid: &[Value],
    ) -> XyceTranAnalysis {
        if !contract.can_use_reference_stop() {
            return tran;
        }
        let Some(reference_stop) = reference_time_grid
            .last()
            .copied()
            .filter(|time| time.is_finite() && *time > tran.stop)
        else {
            return tran;
        };

        XyceTranAnalysis {
            stop: reference_stop,
            ..tran
        }
    }

    fn reference_time_column_index(reference: &XycePrnTable) -> Option<usize> {
        Self::transient_reference_layout(reference)
            .ok()
            .map(|layout| layout.time_column)
    }

    fn transient_reference_layout(
        reference: &XycePrnTable,
    ) -> Result<XyceTransientReferenceLayout, String> {
        if reference.columns.is_empty() {
            return Err("reference table has no columns".to_string());
        }

        let mut cursor = 0usize;
        let stepnum_column = reference
            .columns
            .get(cursor)
            .is_some_and(|column| column.eq_ignore_ascii_case("STEPNUM"))
            .then(|| {
                let column = cursor;
                cursor += 1;
                column
            });
        let index_column = reference
            .columns
            .get(cursor)
            .is_some_and(|column| column.eq_ignore_ascii_case("Index"))
            .then(|| {
                let column = cursor;
                cursor += 1;
                column
            });
        let time_column = cursor;
        if reference
            .columns
            .get(time_column)
            .is_none_or(|column| Self::normalize_probe(column) != "time")
        {
            return Err(format!(
                "expected Xyce transient .prn table to contain optional STEPNUM/Index metadata followed by TIME, got columns {:?}",
                reference.columns
            ));
        }

        Ok(XyceTransientReferenceLayout {
            stepnum_column,
            index_column,
            time_column,
            data_column_offset: time_column + 1,
        })
    }

    fn default_prn_time_quantization_tolerance(time: Value) -> Value {
        if !time.is_finite() || time == 0.0 {
            return 0.0;
        }
        0.5 * 10.0_f64.powf(time.abs().log10().floor() - XYCE_DEFAULT_PRN_FRACTION_DIGITS)
    }

    fn compare_step_res_reference(
        &self,
        path: &Path,
        netlist: &Netlist,
        steps: &[StepCommand],
        step_runs: &[XyceStepRun],
    ) -> Result<(), String> {
        let content = fs::read_to_string(path)
            .map_err(|err| format!("{}: {err}", self.display_path(path)))?;
        let mut nonempty_lines = content
            .lines()
            .enumerate()
            .map(|(index, line)| (index + 1, line.trim()))
            .filter(|(_, line)| !line.is_empty());

        let Some((header_line, header)) = nonempty_lines.next() else {
            return Err(format!("{} is empty", self.display_path(path)));
        };
        let header_fields = header.split_whitespace().collect::<Vec<_>>();
        if !header_fields
            .first()
            .is_some_and(|field| field.eq_ignore_ascii_case("STEP"))
        {
            return Err(format!(
                "{} line {header_line} must start with STEP",
                self.display_path(path)
            ));
        }
        let expected_columns = Self::step_res_expected_columns(netlist, steps, step_runs)?;
        if header_fields.len() != expected_columns.len() + 1 {
            return Err(format!(
                "{} line {header_line} has {} columns; expected STEP plus {} .STEP variable column(s)",
                self.display_path(path),
                header_fields.len(),
                expected_columns.len()
            ));
        }
        for (column_index, (expected_name, _)) in expected_columns.iter().enumerate() {
            let actual_name = header_fields[column_index + 1];
            if !actual_name.eq_ignore_ascii_case(expected_name) {
                return Err(format!(
                    "{} line {header_line} .STEP column {} is '{}', expected '{}'",
                    self.display_path(path),
                    column_index + 1,
                    actual_name,
                    expected_name
                ));
            }
        }

        let mut rows = Vec::new();
        for (line_number, line) in nonempty_lines {
            if line.to_ascii_lowercase().starts_with("end of xyce") {
                break;
            }
            rows.push((line_number, line));
        }

        let expected_row_count = expected_columns
            .first()
            .map(|(_, values)| values.len())
            .unwrap_or(0);
        if rows.len() != expected_row_count {
            return Err(format!(
                "{} has {} step row(s), expected {}",
                self.display_path(path),
                rows.len(),
                expected_row_count
            ));
        }

        for (row_index, (line_number, line)) in rows.iter().copied().enumerate() {
            let fields = line.split_whitespace().collect::<Vec<_>>();
            if fields.len() != expected_columns.len() + 1 {
                return Err(format!(
                    "{} line {line_number} has {} columns, expected STEP index plus {} value column(s)",
                    self.display_path(path),
                    fields.len(),
                    expected_columns.len()
                ));
            }
            let actual_index = fields[0].parse::<usize>().map_err(|err| {
                format!(
                    "{} line {line_number} has invalid STEP index '{}': {err}",
                    self.display_path(path),
                    fields[0]
                )
            })?;
            if actual_index != row_index {
                return Err(format!(
                    "{} line {line_number} has STEP index {actual_index}, expected {row_index}",
                    self.display_path(path)
                ));
            }
            for (column_index, (expected_name, expected_values)) in
                expected_columns.iter().enumerate()
            {
                let actual =
                    Self::parse_xyce_numeric_token(fields[column_index + 1]).map_err(|err| {
                        format!(
                            "{} line {line_number} has invalid STEP value '{}': {err}",
                            self.display_path(path),
                            fields[column_index + 1]
                        )
                    })?;
                let expected = expected_values[row_index];
                let tolerance = XyceComparisonTolerance::from_config(&self.config);
                if let Some(relative_error) = self.value_mismatch(expected, actual, tolerance) {
                    return Err(format!(
                        "{} line {line_number} STEP {} expected {:.8e}, actual {:.8e}, rel {:.3e}",
                        self.display_path(path),
                        expected_name,
                        expected,
                        actual,
                        relative_error
                    ));
                }
            }
        }

        Ok(())
    }

    fn step_res_reference_path(deck_path: &Path, reference_path: &Path) -> Option<PathBuf> {
        let output_res_path = reference_path.with_extension("res");
        if output_res_path.is_file() {
            return Some(output_res_path);
        }
        let deck_res_path = Self::deck_sidecar_path(deck_path, "res");
        if deck_res_path.is_file() {
            return Some(deck_res_path);
        }
        None
    }

    fn deck_sidecar_path(deck_path: &Path, extension: &str) -> PathBuf {
        let mut sidecar = deck_path.as_os_str().to_os_string();
        sidecar.push(".");
        sidecar.push(extension);
        PathBuf::from(sidecar)
    }

    fn step_res_expected_columns(
        netlist: &Netlist,
        steps: &[StepCommand],
        step_runs: &[XyceStepRun],
    ) -> Result<Vec<(String, Vec<Value>)>, String> {
        let mut columns = Vec::new();
        for (step_index, step) in steps.iter().enumerate() {
            if step_runs
                .iter()
                .any(|run| run.step_values.len() <= step_index)
            {
                return Err(format!(
                    ".STEP run metadata is missing value {} for {}",
                    step_index,
                    Self::step_res_variable_name(step)
                ));
            }

            if let StepSweep::Data { table_name } = &step.sweep {
                let table = netlist
                    .data_tables
                    .iter()
                    .find(|table| table.name.eq_ignore_ascii_case(table_name))
                    .ok_or_else(|| format!(".STEP DATA table '{table_name}' not found"))?;
                if table.params.is_empty() {
                    return Err(format!(".STEP DATA table '{}' has no columns", table.name));
                }
                if table.rows.is_empty() {
                    return Err(format!(".STEP DATA table '{}' has no rows", table.name));
                }
                let first_new_column = columns.len();
                columns.extend(
                    table
                        .params
                        .iter()
                        .map(|param| (param.clone(), Vec::with_capacity(step_runs.len()))),
                );
                for run in step_runs {
                    let row_index = run.step_values[step_index];
                    if row_index.fract() != 0.0 || row_index < 0.0 {
                        return Err(format!(
                            ".STEP DATA table '{}' row selector {} is not a non-negative integer",
                            table.name, row_index
                        ));
                    }
                    let row_index = row_index as usize;
                    let row = table.rows.get(row_index).ok_or_else(|| {
                        format!(
                            ".STEP DATA table '{}' row selector {} is outside {} row(s)",
                            table.name,
                            row_index,
                            table.rows.len()
                        )
                    })?;
                    if row.len() != table.params.len() {
                        return Err(format!(
                            ".STEP DATA table '{}' row {} has {} value(s), expected {}",
                            table.name,
                            row_index,
                            row.len(),
                            table.params.len()
                        ));
                    }
                    for (column_index, value) in row.iter().copied().enumerate() {
                        columns[first_new_column + column_index].1.push(value);
                    }
                }
                continue;
            }

            columns.push((
                Self::step_res_variable_name(step),
                step_runs
                    .iter()
                    .map(|run| run.step_values[step_index])
                    .collect(),
            ));
        }

        Ok(columns)
    }

    fn step_res_variable_name(step: &StepCommand) -> String {
        match step.target {
            StepTarget::Temp => "TEMP".to_string(),
            StepTarget::Param => step.name.clone(),
            StepTarget::Device | StepTarget::Model => match &step.param_name {
                Some(param_name) => format!("{}:{param_name}", step.name),
                None => step.name.clone(),
            },
        }
    }

    fn parse_xyce_numeric_token(token: &str) -> Result<f64, std::num::ParseFloatError> {
        let normalized = token.trim_end_matches(',');
        normalized
            .parse::<f64>()
            .or_else(|_| normalized.replace(['D', 'd'], "e").parse::<f64>())
    }

    fn reference_data_columns(
        &self,
        reference: &XycePrnTable,
        print: &XycePrintRequest,
        netlist: &Netlist,
        first_data_column: usize,
        ordered_print_columns: bool,
    ) -> Result<Vec<XyceReferenceColumn>, String> {
        let mut data_columns =
            Vec::with_capacity(reference.columns.len().saturating_sub(first_data_column));
        let mut probe_index = 0usize;
        let mut used_probe_indices = BTreeSet::new();
        for column in reference.columns.iter().skip(first_data_column) {
            if Self::is_primary_dc_sweep_reference_column(column) {
                data_columns.push(XyceReferenceColumn::PrimarySweep {
                    name: column.clone(),
                });
                continue;
            }

            let (matched_index, probe) = if ordered_print_columns {
                let mut skipped_omitted_probes = false;
                let probe = loop {
                    let Some(probe) = print.probes.get(probe_index) else {
                        return Err(format!(
                            "reference column '{}' has no matching .PRINT DC probe",
                            column
                        ));
                    };
                    if Self::reference_column_matches_probe(column, probe) {
                        break probe;
                    }
                    if Self::dc_probe_is_omitted_empty_wildcard(probe, netlist) {
                        probe_index += 1;
                        skipped_omitted_probes = true;
                        continue;
                    }
                    let prefix = if skipped_omitted_probes {
                        "after omitted empty wildcard probe(s), "
                    } else {
                        ""
                    };
                    return Err(format!(
                        "{prefix}reference column '{}' does not match .PRINT probe '{}'",
                        column, probe
                    ));
                };
                if probe_index >= print.probes.len() {
                    return Err(format!(
                        "reference column '{}' has no matching .PRINT DC probe",
                        column
                    ));
                }
                (probe_index, probe)
            } else {
                let Some((index, probe)) =
                    print.probes.iter().enumerate().find(|(index, probe)| {
                        !used_probe_indices.contains(index)
                            && Self::reference_column_matches_probe(column, probe)
                    })
                else {
                    return Err(format!(
                        "compact reference column '{}' has no matching .PRINT DC probe",
                        column
                    ));
                };
                (index, probe)
            };
            used_probe_indices.insert(matched_index);
            data_columns.push(XyceReferenceColumn::Probe {
                name: probe.clone(),
            });
            if ordered_print_columns {
                probe_index += 1;
            }
        }
        if ordered_print_columns {
            while let Some(probe) = print.probes.get(probe_index) {
                if !Self::dc_probe_is_omitted_empty_wildcard(probe, netlist) {
                    break;
                }
                probe_index += 1;
            }
        }
        if ordered_print_columns && probe_index != print.probes.len() {
            return Err(format!(
                "reference table matched {} .PRINT DC probe(s), but deck requested {}",
                probe_index,
                print.probes.len()
            ));
        }
        Ok(data_columns)
    }

    fn is_primary_dc_sweep_reference_column(column: &str) -> bool {
        matches!(Self::normalize_probe(column).as_str(), "v-sweep" | "sweep")
    }

    fn reference_column_probe_for_matching(column: &str) -> &str {
        let trimmed = column.trim();
        trimmed
            .strip_prefix('{')
            .and_then(|body| body.strip_suffix('}'))
            .map(str::trim)
            .unwrap_or(trimmed)
    }

    fn canonical_reference_node_name(node: &str) -> &str {
        if node.eq_ignore_ascii_case("gnd") {
            "0"
        } else {
            node
        }
    }

    fn reference_voltage_column_matches_probe(column: &str, probe: &str) -> bool {
        let Some(column_probe) = Self::parse_tran_voltage_probe(column) else {
            return false;
        };
        let Some(requested_probe) = Self::parse_tran_voltage_probe(probe) else {
            return false;
        };
        column_probe.accessor == requested_probe.accessor
            && Self::canonical_reference_node_name(&column_probe.node_pos)
                == Self::canonical_reference_node_name(&requested_probe.node_pos)
            && column_probe
                .node_neg
                .as_deref()
                .map(Self::canonical_reference_node_name)
                == requested_probe
                    .node_neg
                    .as_deref()
                    .map(Self::canonical_reference_node_name)
    }

    fn reference_column_matches_probe(column: &str, probe: &str) -> bool {
        let normalized_column = Self::normalize_probe(column);
        let normalized_probe = Self::normalize_probe(probe);
        if normalized_column == normalized_probe {
            return true;
        }

        let normalized_column =
            Self::normalize_probe(Self::reference_column_probe_for_matching(column));
        let normalized_probe =
            Self::normalize_probe(Self::reference_column_probe_for_matching(probe));
        if normalized_column == normalized_probe {
            return true;
        }
        if Self::reference_voltage_column_matches_probe(&normalized_column, &normalized_probe) {
            return true;
        }
        if let Some(mapped_probe) = Self::compact_reference_probe_alias(&normalized_column) {
            return mapped_probe == normalized_probe;
        }
        if let Some(source_name) = Self::parse_current_probe(&normalized_probe) {
            return normalized_column == format!("{source_name}_branch")
                || normalized_column == format!("{source_name}#branch");
        }
        false
    }

    fn reference_ac_data_columns(
        reference: &XycePrnTable,
        print: &XycePrintRequest,
        data_column_offset: usize,
    ) -> Result<Vec<XyceAcReferenceColumn>, String> {
        let mut columns = Vec::new();
        for column in reference.columns.iter().skip(data_column_offset) {
            if Self::print_requests_scalar_ac_probe(print, column) {
                columns.push(XyceAcReferenceColumn::Probe {
                    name: column.clone(),
                    component: XyceAcProbeComponent::Scalar,
                });
                continue;
            }

            if let Some((component, probe)) = Self::parse_ac_component_reference_column(column) {
                if !Self::print_requests_complex_ac_probe(print, &probe) {
                    return Err(format!(
                        "AC reference column '{}' is not produced by the deck's .PRINT AC probes",
                        column
                    ));
                }
                columns.push(XyceAcReferenceColumn::Probe {
                    name: probe,
                    component,
                });
                continue;
            }

            return Err(format!(
                "AC reference column '{}' is not produced by the deck's .PRINT AC probes",
                column
            ));
        }
        Ok(columns)
    }

    fn is_ac_frequency_reference_column(column: &str) -> bool {
        matches!(Self::normalize_probe(column).as_str(), "freq" | "frequency")
    }

    fn parse_ac_component_reference_column(column: &str) -> Option<(XyceAcProbeComponent, String)> {
        let normalized = Self::normalize_probe(column);
        let (prefix, component) = if normalized.starts_with("re(") {
            ("re(", XyceAcProbeComponent::Real)
        } else if normalized.starts_with("im(") {
            ("im(", XyceAcProbeComponent::Imaginary)
        } else {
            return None;
        };
        if !normalized.ends_with(')') {
            return None;
        }
        let inner = &normalized[prefix.len()..normalized.len() - 1];
        (!inner.is_empty()).then(|| (component, inner.to_string()))
    }

    fn print_requests_complex_ac_probe(print: &XycePrintRequest, probe: &str) -> bool {
        let normalized_probe = Self::normalize_ac_expression_probe_key(probe);
        print
            .probes
            .iter()
            .any(|requested| Self::normalize_ac_expression_probe_key(requested) == normalized_probe)
    }

    fn print_requests_scalar_ac_probe(print: &XycePrintRequest, column: &str) -> bool {
        let normalized_column = Self::normalize_probe(column);
        let normalized_expression_column = Self::normalize_ac_expression_probe_key(column);
        print.probes.iter().any(|requested| {
            Self::normalize_probe(requested) == normalized_column
                || Self::normalize_ac_expression_probe_key(requested)
                    == normalized_expression_column
        })
    }

    fn normalize_ac_expression_probe_key(probe: &str) -> String {
        let expression = Self::print_expression_inner(probe).unwrap_or(probe);
        Self::normalize_probe(expression)
    }

    fn compact_reference_probe_alias(normalized_column: &str) -> Option<&'static str> {
        match normalized_column {
            "v(g)" => Some("v(g,ga)"),
            "v(d)" => Some("v(d,da)"),
            "v(s)" => Some("v(s,sa)"),
            "v(b)" => Some("v(b,ba)"),
            "i(d)" => Some("i(vdprobe)"),
            "i(g)" => Some("i(vgprobe)"),
            "i(s)" => Some("i(vsprobe)"),
            "i(b)" => Some("i(vbprobe)"),
            _ => None,
        }
    }

    fn comp_tolerances(
        &self,
        source: &str,
        columns: &[XyceReferenceColumn],
    ) -> Result<BTreeMap<String, XyceComparisonTolerance>, String> {
        let mut compared_probes = BTreeSet::new();
        for column in columns {
            let XyceReferenceColumn::Probe { name } = column else {
                continue;
            };
            compared_probes.insert(Self::normalize_probe(name));
        }

        let mut tolerances = BTreeMap::new();
        for line in Self::logical_netlist_lines(source) {
            let trimmed = line.trim_start();
            if !trimmed
                .get(..5)
                .is_some_and(|prefix| prefix.eq_ignore_ascii_case("*comp"))
            {
                continue;
            }
            let rest = trimmed[5..].trim_start();
            let Some((probe, options)) = Self::split_comp_directive(rest) else {
                continue;
            };
            let normalized_probe = Self::normalize_probe(&probe);
            if !compared_probes.contains(&normalized_probe) {
                continue;
            }
            let default_tolerance = self.default_comparison_tolerance(&normalized_probe);
            let tolerance = Self::parse_comp_tolerance(&options, default_tolerance)?;
            tolerances.insert(normalized_probe, tolerance);
        }
        Ok(tolerances)
    }

    fn default_comparison_tolerance(&self, normalized_probe: &str) -> XyceComparisonTolerance {
        let mut tolerance = XyceComparisonTolerance::from_config(&self.config);
        if Self::probe_uses_voltage_tolerance(normalized_probe) {
            tolerance.absolute = self.config.voltage_absolute_tolerance;
        }
        if Self::probe_uses_current_tolerance(normalized_probe) {
            tolerance.zero = Some(
                tolerance
                    .zero
                    .unwrap_or(2.0 * self.config.absolute_tolerance),
            );
        }
        if Self::probe_uses_power_tolerance(normalized_probe) {
            tolerance.absolute = tolerance.absolute.max(self.config.power_absolute_tolerance);
        }
        tolerance
    }

    fn probe_uses_power_tolerance(normalized_probe: &str) -> bool {
        if Self::parse_power_probe(normalized_probe).is_some() {
            return true;
        }
        Self::print_expression_inner(normalized_probe).is_some_and(|expression| {
            Self::parse_power_probe(&Self::normalize_probe(expression)).is_some()
        })
    }

    fn probe_uses_voltage_tolerance(normalized_probe: &str) -> bool {
        normalized_probe == "v-sweep"
            || Self::parse_ac_voltage_probe(normalized_probe)
                .is_some_and(|probe| probe.accessor.uses_voltage_tolerance())
    }

    fn probe_uses_current_tolerance(normalized_probe: &str) -> bool {
        Self::parse_current_probe(normalized_probe).is_some()
            || Self::parse_ac_current_probe(normalized_probe).is_some()
            || Self::parse_lead_current_probe(normalized_probe).is_some()
    }

    fn split_comp_directive(rest: &str) -> Option<(String, String)> {
        let rest = rest
            .split_once(';')
            .map(|(head, _)| head)
            .unwrap_or(rest)
            .trim();
        if rest.is_empty() {
            return None;
        }

        if rest.starts_with('{') {
            let mut depth = 0usize;
            for (index, ch) in rest.char_indices() {
                match ch {
                    '{' => depth += 1,
                    '}' => {
                        depth = depth.saturating_sub(1);
                        if depth == 0 {
                            let end = index + ch.len_utf8();
                            return Some((
                                rest[..end].trim().to_string(),
                                rest[end..].trim().to_string(),
                            ));
                        }
                    }
                    _ => {}
                }
            }
        }

        let mut split = rest.splitn(2, char::is_whitespace);
        let probe = split.next()?.trim();
        if probe.is_empty() {
            return None;
        }
        Some((
            probe.to_string(),
            split.next().unwrap_or_default().trim().to_string(),
        ))
    }

    fn parse_comp_tolerance(
        options: &str,
        default_tolerance: XyceComparisonTolerance,
    ) -> Result<XyceComparisonTolerance, String> {
        let mut tolerance = default_tolerance;
        let tokens = options.split_whitespace().collect::<Vec<_>>();
        let mut index = 0usize;
        while index < tokens.len() {
            let token = tokens[index];
            let (raw_key, raw_value, consumed) = if let Some((key, value)) = token.split_once('=') {
                (key, value, 1usize)
            } else if token.ends_with('=') {
                let key = token.trim_end_matches('=');
                let value = tokens.get(index + 1).copied().unwrap_or_default();
                (key, value, 2usize)
            } else if tokens
                .get(index + 1)
                .is_some_and(|candidate| *candidate == "=")
            {
                let value = tokens.get(index + 2).copied().unwrap_or_default();
                (token, value, 3usize)
            } else {
                index += 1;
                continue;
            };

            let key = raw_key.trim().to_ascii_lowercase();
            let value = raw_value.trim();
            if value.is_empty() {
                return Err(format!("Xyce *COMP option '{key}' is missing a value"));
            }
            match key.as_str() {
                "reltol" => tolerance = tolerance.with_relative(Self::parse_comp_float(value)?),
                "abstol" | "absdifftol" => {
                    tolerance = tolerance.with_absolute(Self::parse_comp_float(value)?)
                }
                "zerotol" => tolerance = tolerance.with_zero(Self::parse_comp_float(value)?),
                _ => {}
            }
            index += consumed;
        }
        Ok(tolerance)
    }

    fn parse_comp_float(value: &str) -> Result<f64, String> {
        value
            .parse::<f64>()
            .map_err(|err| format!("invalid Xyce *COMP numeric value '{value}': {err}"))
    }

    fn value_mismatch(
        &self,
        expected: f64,
        actual: f64,
        tolerance: XyceComparisonTolerance,
    ) -> Option<f64> {
        if !expected.is_finite() || !actual.is_finite() {
            return Some(f64::INFINITY);
        }
        let abs_error = (expected - actual).abs();
        if let Some(zero_tolerance) = tolerance.zero {
            if expected.abs() <= zero_tolerance && actual.abs() <= zero_tolerance {
                return None;
            }
        }
        if abs_error <= tolerance.absolute {
            return None;
        }
        let scale = expected.abs().max(actual.abs()).max(tolerance.absolute);
        let relative_error = abs_error / scale;
        (relative_error > tolerance.relative).then_some(relative_error)
    }

    fn validate_static_dc_contract(
        netlist: &Netlist,
        dc: &XyceDcSweep,
        print: &XycePrintRequest,
    ) -> Result<(), String> {
        Self::validate_dc_sweep_source(netlist, &dc.source)?;
        if let Some(sweep2) = &dc.sweep2 {
            Self::validate_dc_sweep_source(netlist, &sweep2.source)?;
        }

        for probe in &print.probes {
            Self::validate_dc_probe(probe, netlist)?;
        }

        Self::reject_unsupported_static_dc_model_observables(netlist, print)?;
        Self::reject_unsupported_vbic_nested_current_source_sweeps(netlist, dc, print)?;

        Ok(())
    }

    fn validate_static_dc_data_contract(
        netlist: &Netlist,
        dc_data: &XyceDcDataSweep,
        print: &XycePrintRequest,
    ) -> Result<(), String> {
        if dc_data.rows.is_empty() {
            return Err(".DC DATA sweep produced no table rows".to_string());
        }

        for probe in &print.probes {
            Self::validate_dc_probe(probe, netlist)?;
        }
        Self::reject_unsupported_static_dc_model_observables(netlist, print)?;

        Ok(())
    }

    fn dc_data_sweep_for_source(
        source: &str,
        netlist: &Netlist,
    ) -> Result<Option<XyceDcDataSweep>, String> {
        let table_names = Self::dc_data_table_names(source)?;
        if table_names.is_empty() {
            return Ok(None);
        }

        let mut rows: Option<Vec<XyceDcDataRow>> = None;
        let mut seen_columns = BTreeSet::new();
        for table_name in table_names {
            let table = netlist
                .data_tables
                .iter()
                .find(|table| table.name.eq_ignore_ascii_case(&table_name))
                .ok_or_else(|| format!(".DC DATA references unknown .DATA table '{table_name}'"))?;
            if table.params.is_empty() {
                return Err(format!(
                    ".DC DATA table '{}' has no parameter columns",
                    table.name
                ));
            }
            if table.rows.is_empty() {
                return Err(format!(".DC DATA table '{}' has no rows", table.name));
            }
            for (row_index, row) in table.rows.iter().enumerate() {
                if row.len() != table.params.len() {
                    return Err(format!(
                        ".DC DATA table '{}' row {} has {} value(s), expected {}",
                        table.name,
                        row_index + 1,
                        row.len(),
                        table.params.len()
                    ));
                }
            }

            match rows.as_ref() {
                Some(existing) if existing.len() != table.rows.len() => {
                    return Err(format!(
                        ".DC DATA table '{}' has {} row(s), expected {} to match the other TABLE-style .DC DATA sweeps",
                        table.name,
                        table.rows.len(),
                        existing.len()
                    ));
                }
                None => {
                    rows = Some(
                        (0..table.rows.len())
                            .map(|_| XyceDcDataRow {
                                overrides: Vec::new(),
                            })
                            .collect(),
                    );
                }
                Some(_) => {}
            }

            for (column_index, column_name) in table.params.iter().enumerate() {
                let column_key = Self::normalize_probe(column_name);
                if !seen_columns.insert(column_key) {
                    return Err(format!(
                        ".DC DATA column '{}' is specified more than once across the active data tables",
                        column_name
                    ));
                }

                let rows = rows.as_mut().expect("rows initialized from table length");
                for (row_index, row) in table.rows.iter().enumerate() {
                    let value = row[column_index];
                    if !value.is_finite() {
                        return Err(format!(
                            ".DC DATA table '{}' row {} column '{}' contains non-finite value {}",
                            table.name,
                            row_index + 1,
                            column_name,
                            value
                        ));
                    }
                    rows[row_index]
                        .overrides
                        .push(Self::dc_data_override_for_column(
                            netlist,
                            column_name,
                            value,
                        )?);
                }
            }
        }

        let rows = rows.unwrap_or_default();
        if rows.is_empty() {
            return Err(".DC DATA sweep produced no table rows".to_string());
        }
        Ok(Some(XyceDcDataSweep { rows }))
    }

    fn dc_data_override_for_column(
        netlist: &Netlist,
        column_name: &str,
        value: Value,
    ) -> Result<XyceDcDataOverride, String> {
        if let Some((device_name, param_name)) = Self::parse_device_parameter_probe(column_name)
            && Self::netlist_has_top_level_element_named(netlist, &device_name)
        {
            return Ok(XyceDcDataOverride::Device {
                name: device_name,
                param_name: Some(param_name),
                value,
            });
        }

        if Self::netlist_has_top_level_element_named(netlist, column_name) {
            return Ok(XyceDcDataOverride::Device {
                name: column_name.to_string(),
                param_name: None,
                value,
            });
        }

        if Self::netlist_has_numeric_parameter(netlist, column_name) {
            return Ok(XyceDcDataOverride::Parameter {
                name: column_name.to_string(),
                value,
            });
        }

        Err(format!(
            ".DC DATA column '{}' does not resolve to a top-level device value, device parameter, or numeric parameter",
            column_name
        ))
    }

    fn netlist_has_top_level_element_named(netlist: &Netlist, name: &str) -> bool {
        netlist
            .elements
            .iter()
            .any(|element| element.name.eq_ignore_ascii_case(name))
    }

    fn netlist_has_numeric_parameter(netlist: &Netlist, name: &str) -> bool {
        netlist
            .params
            .all_params()
            .iter()
            .any(|(param_name, _)| param_name.eq_ignore_ascii_case(name))
    }

    fn synthetic_dc_data_sweep(dc_data: &XyceDcDataSweep) -> Result<XyceDcSweep, String> {
        if dc_data.rows.is_empty() {
            return Err(".DC DATA sweep produced no table rows".to_string());
        }
        Ok(XyceDcSweep {
            source: "DATA".to_string(),
            start: 0.0,
            stop: (dc_data.rows.len() - 1) as Value,
            step: 1.0,
            mode: crate::netlist::DcSweepMode::Linear,
            sweep2: None,
        })
    }

    fn dc_data_table_names(source: &str) -> Result<Vec<String>, String> {
        let mut table_names = Vec::new();
        for line in Self::logical_netlist_lines(source) {
            let stripped = Self::strip_netlist_comment(&line);
            let trimmed = stripped.trim();
            let Some(command) = trimmed.split_whitespace().next() else {
                continue;
            };
            if !command.eq_ignore_ascii_case(".dc") {
                continue;
            }
            if let Some(table_name) = Self::assignment_value(trimmed, "data")? {
                table_names.push(table_name);
            }
        }
        Ok(table_names)
    }

    fn assignment_value(line: &str, key: &str) -> Result<Option<String>, String> {
        let normalized = Self::normalize_assignment_spacing(line);
        for field in normalized.split_whitespace().skip(1) {
            let Some((name, value)) = field.split_once('=') else {
                continue;
            };
            if !name.eq_ignore_ascii_case(key) {
                continue;
            }
            if value.is_empty() {
                return Err(format!("assignment '{key}=' has no value"));
            }
            return Ok(Some(value.to_string()));
        }
        Ok(None)
    }

    fn normalize_assignment_spacing(line: &str) -> String {
        let mut out = String::with_capacity(line.len());
        let mut chars = line.chars().peekable();
        while let Some(ch) = chars.next() {
            if ch.is_whitespace() {
                while chars.peek().is_some_and(|next| next.is_whitespace()) {
                    chars.next();
                }
                if chars.peek() == Some(&'=') || out.ends_with('=') {
                    continue;
                }
                out.push(' ');
                continue;
            }
            out.push(ch);
        }
        out
    }

    fn validate_static_tran_contract(
        netlist: &Netlist,
        tran: &XyceTranAnalysis,
        print: &XycePrintRequest,
    ) -> Result<(), String> {
        if !tran.stop.is_finite() || tran.stop <= 0.0 {
            return Err(format!(
                ".TRAN stop time must be finite and positive, got {}",
                tran.stop
            ));
        }
        if !tran.step.is_finite() || tran.step < 0.0 {
            return Err(format!(
                ".TRAN print step must be finite and non-negative, got {}",
                tran.step
            ));
        }
        if let Some(start) = tran.start
            && (!start.is_finite() || start < 0.0 || start > tran.stop)
        {
            return Err(format!(
                ".TRAN start time must be finite and within [0, stop], got {start}"
            ));
        }
        if let Some(max_step) = tran.max_step
            && (!max_step.is_finite() || max_step <= 0.0)
        {
            return Err(format!(
                ".TRAN maximum step must be finite and positive when specified, got {max_step}"
            ));
        }
        // `run_tran` reads UIC from the parsed netlist; the plan carries it so
        // validation still reflects the complete .TRAN command surface.
        let _engine_reads_uic_from_netlist = tran.uic;

        Self::validate_transient_preflight_execution_envelope(netlist, tran)?;
        for probe in &print.probes {
            Self::validate_tran_probe(probe, netlist)?;
        }
        Self::validate_native_transient_contract(netlist)?;

        Ok(())
    }

    fn validate_static_ac_contract(
        netlist: &Netlist,
        ac: &XyceAcAnalysis,
        print: &XycePrintRequest,
    ) -> Result<(), String> {
        let frequencies = ac.frequencies();
        if frequencies.is_empty() {
            return Err(".AC analysis produced no frequency points".to_string());
        }

        for probe in &print.probes {
            Self::validate_ac_probe(probe, netlist)?;
        }
        Self::validate_native_static_ac_contract(netlist, ac)?;

        Ok(())
    }

    fn validate_native_static_ac_contract(
        netlist: &Netlist,
        ac: &XyceAcAnalysis,
    ) -> Result<(), String> {
        let max_frequency = ac
            .frequencies
            .iter()
            .copied()
            .filter(|frequency| frequency.is_finite())
            .fold(0.0_f64, f64::max);
        if netlist
            .elements
            .iter()
            .any(|element| matches!(element.kind, ElementKind::Subcircuit { .. }))
        {
            return Err(
                "native static .PRINT AC comparison does not support subcircuit flattening yet"
                    .to_string(),
            );
        }

        for element in &netlist.elements {
            match &element.kind {
                ElementKind::VoltageSource(_)
                | ElementKind::CurrentSource(_)
                | ElementKind::Resistor { .. }
                | ElementKind::Capacitor { .. }
                | ElementKind::Inductor { .. } => {}
                ElementKind::Coupling { coefficient, .. } => {
                    if !coefficient.is_finite() {
                        return Err(format!(
                            "native static .PRINT AC comparison does not support coupling '{}' with non-finite coefficient {}",
                            element.name, coefficient
                        ));
                    }
                }
                ElementKind::Vccs {
                    transconductance, ..
                } => {
                    Self::validate_finite_controlled_source_gain(
                        "VCCS",
                        &element.name,
                        "transconductance",
                        *transconductance,
                    )?;
                }
                ElementKind::Vcvs { gain, .. } => {
                    Self::validate_finite_controlled_source_gain(
                        "VCVS",
                        &element.name,
                        "gain",
                        *gain,
                    )?;
                }
                ElementKind::Cccs {
                    gain,
                    control_element,
                    ..
                } => {
                    Self::validate_finite_controlled_source_gain(
                        "CCCS",
                        &element.name,
                        "gain",
                        *gain,
                    )?;
                    Self::validate_current_controlled_source_probe(
                        &netlist.elements,
                        "CCCS",
                        &element.name,
                        control_element,
                    )?;
                }
                ElementKind::Ccvs {
                    transresistance,
                    control_element,
                    ..
                } => {
                    Self::validate_finite_controlled_source_gain(
                        "CCVS",
                        &element.name,
                        "transresistance",
                        *transresistance,
                    )?;
                    Self::validate_current_controlled_source_probe(
                        &netlist.elements,
                        "CCVS",
                        &element.name,
                        control_element,
                    )?;
                }
                ElementKind::BehavioralVoltage { expression, .. }
                | ElementKind::BehavioralCurrent { expression, .. } => {
                    Self::validate_ac_behavioral_expression(
                        &element.name,
                        expression,
                        &netlist.params,
                    )?;
                }
                ElementKind::Mosfet { .. }
                    if Self::netlist_device_is_single_native_ac_supported_bulk_mosfet(
                        netlist,
                        &element.name,
                    ) => {}
                ElementKind::Bjt { .. }
                    if max_frequency <= 100.0
                        && Self::netlist_device_is_native_legacy_bjt(netlist, &element.name) => {}
                _ => {
                    return Err(format!(
                        "native static .PRINT AC comparison currently supports independent sources, static R/L/C passives, mutual inductors, finite-gain linear controlled sources, time-independent behavioral sources, single-device native MOSFET LEVEL=2/6, and native legacy BJT sweeps up to 100 Hz; element '{}' requires a broader AC oracle contract",
                        element.name
                    ));
                }
            }
        }

        Ok(())
    }

    fn validate_static_step_tran_contract(netlist: &Netlist) -> Result<(), String> {
        for element in &netlist.elements {
            match &element.kind {
                ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => {
                    Self::validate_static_step_tran_source_spec(&element.name, spec)?;
                }
                ElementKind::Resistor { .. } => {
                    Self::validate_static_step_resistor_contract(netlist, &element.name)?;
                }
                ElementKind::Capacitor { .. } => {
                    Self::validate_static_step_capacitor_contract(netlist, &element.name)?
                }
                ElementKind::Inductor { .. } => {
                    Self::validate_static_step_inductor_contract(netlist, &element.name)?;
                }
                ElementKind::Coupling {
                    inductors,
                    coefficient,
                } => {
                    Self::validate_static_step_coupling_contract(
                        netlist,
                        &element.name,
                        inductors,
                        *coefficient,
                    )?;
                }
                _ => {
                    return Err(format!(
                        "native .STEP .PRINT TRAN comparison currently supports static R/L/C passives, coupled inductors, and independent DC/PULSE/SIN/PWL/PAT sources; element '{}' requires a broader stepped transient oracle contract",
                        element.name
                    ));
                }
            }
        }
        Ok(())
    }

    fn validate_static_step_resistor_contract(
        netlist: &Netlist,
        element_name: &str,
    ) -> Result<(), String> {
        let resistance = Self::effective_resistor_value(netlist, element_name).ok_or_else(|| {
            format!(
                "native .STEP .PRINT TRAN comparison could not resolve resistor '{}' to a static resistance",
                element_name
            )
        })?;
        if resistance.is_finite() || (resistance.is_infinite() && resistance.is_sign_positive()) {
            Ok(())
        } else {
            Err(format!(
                "native .STEP .PRINT TRAN comparison does not support resistor '{}' with invalid resistance {}",
                element_name, resistance
            ))
        }
    }

    fn validate_static_step_capacitor_contract(
        netlist: &Netlist,
        element_name: &str,
    ) -> Result<(), String> {
        Self::validate_xyce_capacitor_contract_params(netlist, element_name)?;
        let capacitance = Engine::new(SimulationConfig {
            spice_dialect: SpiceDialect::Xyce,
            ..SimulationConfig::default()
        })
        .resolved_capacitor_value(netlist, element_name)
        .map_err(|err| {
            format!(
                "native .PRINT TRAN comparison could not resolve capacitor '{}' to a static capacitance: {}",
                element_name, err
            )
        })?
        .ok_or_else(|| {
            format!(
                "native .PRINT TRAN comparison could not resolve capacitor '{}' to a static capacitance",
                element_name
            )
        })?;
        if capacitance.is_finite() && capacitance >= 0.0 {
            Ok(())
        } else {
            Err(format!(
                "native .PRINT TRAN comparison does not support capacitor '{}' with invalid capacitance {}",
                element_name, capacitance
            ))
        }
    }

    fn validate_xyce_capacitor_contract_params(
        netlist: &Netlist,
        element_name: &str,
    ) -> Result<(), String> {
        let element = Self::find_capacitor_element(netlist, element_name)
            .ok_or_else(|| format!("capacitor '{}' not found", element_name))?;
        let ElementKind::Capacitor {
            value,
            value_expr,
            model,
            instance_params,
            ..
        } = &element.kind
        else {
            return Err(format!("element '{}' is not a capacitor", element_name));
        };

        if !value.is_finite() {
            if let Some(expression) = value_expr.as_deref() {
                Self::validate_static_transient_passive_value_expression(
                    "capacitor",
                    element_name,
                    expression,
                    &netlist.params,
                )?;
            } else if model.is_none() {
                return Err(format!(
                    "native static .PRINT TRAN comparison could not resolve capacitor value for element '{}'",
                    element_name
                ));
            }
        }

        const INSTANCE_PARAMS: &[&str] = &["L", "W", "M", "TEMP", "DTEMP", "TC1", "TC2"];
        for (name, value) in instance_params {
            if !INSTANCE_PARAMS
                .iter()
                .any(|candidate| name.eq_ignore_ascii_case(candidate))
            {
                return Err(format!(
                    "native static .PRINT TRAN comparison does not yet support Xyce capacitor instance parameter {} on element '{}'",
                    name, element_name
                ));
            }
            if !value.is_finite() {
                return Err(format!(
                    "native static .PRINT TRAN comparison does not support capacitor '{}' with non-finite instance parameter {}={}",
                    element_name, name, value
                ));
            }
        }

        let Some(model_name) = model.as_deref() else {
            return Ok(());
        };
        let model = Self::find_model(&netlist.models, model_name).ok_or_else(|| {
            format!(
                "native static .PRINT TRAN comparison could not find capacitor '{}' model '{}'",
                element_name, model_name
            )
        })?;
        if !matches!(
            model.model_type.to_ascii_uppercase().as_str(),
            "C" | "CAP" | "CAPACITOR"
        ) {
            return Err(format!(
                "native static .PRINT TRAN comparison does not support capacitor '{}' model '{}' of type '{}'",
                element_name, model_name, model.model_type
            ));
        }

        const MODEL_PARAMS: &[&str] = &["C", "CJ", "CJSW", "DEFW", "NARROW", "TC1", "TC2", "TNOM"];
        for (name, value) in &model.params {
            if !MODEL_PARAMS
                .iter()
                .any(|candidate| name.eq_ignore_ascii_case(candidate))
            {
                return Err(format!(
                    "native static .PRINT TRAN comparison does not yet support Xyce capacitor model parameter {} on model '{}'",
                    name, model_name
                ));
            }
            if !value.is_finite() {
                return Err(format!(
                    "native static .PRINT TRAN comparison does not support capacitor model '{}' with non-finite parameter {}={}",
                    model_name, name, value
                ));
            }
        }
        for (name, expression) in &model.expr_params {
            if !MODEL_PARAMS
                .iter()
                .any(|candidate| name.eq_ignore_ascii_case(candidate))
            {
                return Err(format!(
                    "native static .PRINT TRAN comparison does not yet support Xyce capacitor model expression parameter {} on model '{}'",
                    name, model_name
                ));
            }
            Self::validate_static_transient_passive_value_expression(
                "capacitor model parameter",
                name,
                expression,
                &netlist.params,
            )?;
        }
        if !model.string_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
        {
            return Err(format!(
                "native static .PRINT TRAN comparison does not support non-scalar capacitor model parameters on model '{}'",
                model_name
            ));
        }
        Ok(())
    }

    fn validate_static_step_inductor_contract(
        netlist: &Netlist,
        element_name: &str,
    ) -> Result<(), String> {
        let inductance = Self::effective_inductor_value(netlist, element_name).ok_or_else(|| {
            format!(
                "native .STEP .PRINT TRAN comparison could not resolve inductor '{}' to a static inductance",
                element_name
            )
        })?;
        if inductance.is_finite() && inductance > 0.0 {
            Ok(())
        } else {
            Err(format!(
                "native .STEP .PRINT TRAN comparison does not support inductor '{}' with invalid inductance {}",
                element_name, inductance
            ))
        }
    }

    fn validate_static_step_coupling_contract(
        netlist: &Netlist,
        element_name: &str,
        inductors: &[String],
        coefficient: Value,
    ) -> Result<(), String> {
        if inductors.len() < 2 {
            return Err(format!(
                "native .PRINT TRAN comparison does not support coupling '{}' with fewer than two inductors",
                element_name
            ));
        }
        if !coefficient.is_finite() || !(0.0..=1.0).contains(&coefficient) {
            return Err(format!(
                "native .PRINT TRAN comparison does not support coupling '{}' with invalid coefficient {}",
                element_name, coefficient
            ));
        }

        for inductor_name in inductors {
            Self::validate_static_step_inductor_contract(netlist, inductor_name).map_err(
                |err| {
                    format!(
                        "native .PRINT TRAN comparison does not support coupling '{}' because referenced inductor '{}' is not a supported linear inductor: {}",
                        element_name, inductor_name, err
                    )
                },
            )?;
        }
        Ok(())
    }

    fn validate_static_step_tran_source_spec(
        source_name: &str,
        spec: &crate::netlist::SourceSpec,
    ) -> Result<(), String> {
        match spec {
            crate::netlist::SourceSpec::Dc(_)
            | crate::netlist::SourceSpec::Ac { .. }
            | crate::netlist::SourceSpec::DcAc { .. }
            | crate::netlist::SourceSpec::Pulse { .. }
            | crate::netlist::SourceSpec::Sin { .. }
            | crate::netlist::SourceSpec::Pwl { .. }
            | crate::netlist::SourceSpec::PwlFile { .. }
            | crate::netlist::SourceSpec::Pat { .. } => Ok(()),
            crate::netlist::SourceSpec::DcTransient { transient, .. }
            | crate::netlist::SourceSpec::DcAcTransient { transient, .. } => {
                Self::validate_static_step_tran_source_spec(source_name, transient)
            }
            other => Err(format!(
                "native .STEP .PRINT TRAN comparison currently supports independent DC/PULSE/SIN/PWL/PAT sources; source '{source_name}' uses {other:?}"
            )),
        }
    }

    fn validate_native_transient_contract(netlist: &Netlist) -> Result<(), String> {
        if netlist
            .elements
            .iter()
            .any(|element| matches!(element.kind, ElementKind::Subcircuit { .. }))
        {
            let flattened =
                crate::netlist::flatten_netlist_with_models(netlist).map_err(|err| {
                    format!(
                        "native static .PRINT TRAN comparison could not flatten subcircuits: {err}"
                    )
                })?;
            Self::validate_flattened_subcircuit_instances_resolved(netlist, &flattened.elements)?;
            let mut flat_netlist = netlist.clone();
            flat_netlist.elements = flattened.elements;
            flat_netlist.models.extend(flattened.scoped_models);
            flat_netlist.subcircuits.clear();
            return Self::validate_native_transient_contract(&flat_netlist);
        }

        let elements = &netlist.elements;
        let params = &netlist.params;
        for element in elements {
            match &element.kind {
                ElementKind::VoltageSource(_) | ElementKind::CurrentSource(_) => {}
                ElementKind::Vcvs { gain, .. } => {
                    Self::validate_finite_controlled_source_gain(
                        "VCVS",
                        &element.name,
                        "gain",
                        *gain,
                    )?;
                }
                ElementKind::Vccs {
                    transconductance, ..
                } => {
                    Self::validate_finite_controlled_source_gain(
                        "VCCS",
                        &element.name,
                        "transconductance",
                        *transconductance,
                    )?;
                }
                ElementKind::Cccs {
                    gain,
                    control_element,
                    ..
                } => {
                    Self::validate_finite_controlled_source_gain(
                        "CCCS",
                        &element.name,
                        "gain",
                        *gain,
                    )?;
                    Self::validate_current_controlled_source_probe(
                        elements,
                        "CCCS",
                        &element.name,
                        control_element,
                    )?;
                }
                ElementKind::Ccvs {
                    transresistance,
                    control_element,
                    ..
                } => {
                    Self::validate_finite_controlled_source_gain(
                        "CCVS",
                        &element.name,
                        "transresistance",
                        *transresistance,
                    )?;
                    Self::validate_current_controlled_source_probe(
                        elements,
                        "CCVS",
                        &element.name,
                        control_element,
                    )?;
                }
                ElementKind::BehavioralVoltage { expression, .. }
                | ElementKind::BehavioralCurrent { expression, .. } => {
                    Self::validate_transient_behavioral_expression(
                        &element.name,
                        expression,
                        params,
                    )?;
                }
                ElementKind::Resistor { value_expr, .. } => {
                    if let Some(expression) = value_expr {
                        Self::validate_transient_behavioral_expression(
                            &element.name,
                            expression,
                            params,
                        )?;
                    }
                }
                ElementKind::Capacitor { .. } => {
                    Self::validate_static_step_capacitor_contract(netlist, &element.name)?
                }
                ElementKind::Inductor { .. } => {
                    Self::validate_static_step_inductor_contract(netlist, &element.name)?;
                }
                ElementKind::Coupling {
                    inductors,
                    coefficient,
                } => {
                    Self::validate_static_step_coupling_contract(
                        netlist,
                        &element.name,
                        inductors,
                        *coefficient,
                    )?;
                }
                ElementKind::VSwitch { .. } => {}
                ElementKind::ISwitch {
                    control_element, ..
                } => {
                    Self::validate_current_controlled_source_probe(
                        elements,
                        "ISWITCH",
                        &element.name,
                        control_element,
                    )?;
                }
                ElementKind::GenericSwitch {
                    model,
                    control_expression,
                    ..
                } => {
                    if Self::netlist_model_is_current_switch(netlist, model) {
                        let control_element =
                            Self::direct_branch_current_control(control_expression).ok_or_else(
                                || {
                                    format!(
                                        "native static .PRINT TRAN comparison does not support generic ISWITCH element '{}' with CONTROL expression '{}' because it is not a direct branch-current probe",
                                        element.name, control_expression
                                    )
                                },
                            )?;
                        Self::validate_current_controlled_source_probe(
                            elements,
                            "generic ISWITCH",
                            &element.name,
                            &control_element,
                        )?;
                    } else {
                        Self::validate_transient_generic_switch_expression(
                            &element.name,
                            control_expression,
                            params,
                        )?;
                    }
                }
                ElementKind::Mosfet { .. }
                    if Self::netlist_device_is_native_b3soi_mosfet(netlist, &element.name) => {}
                ElementKind::Jfet { .. }
                    if Self::netlist_device_is_native_classic_jfet(netlist, &element.name) => {}
                _ => {
                    return Err(format!(
                        "native static .PRINT TRAN comparison currently supports independent, behavioral, static R/L/C, switch, controlled-source, native B3SOI, and native classic JFET transient decks; element '{}' requires a broader transient oracle contract",
                        element.name
                    ));
                }
            }
        }
        Ok(())
    }

    fn validate_flattened_subcircuit_instances_resolved(
        netlist: &Netlist,
        flattened_elements: &[crate::netlist::Element],
    ) -> Result<(), String> {
        for element in &netlist.elements {
            if !matches!(element.kind, ElementKind::Subcircuit { .. }) {
                continue;
            }
            let prefix = format!("{}.", element.name).to_ascii_lowercase();
            let mut members = flattened_elements
                .iter()
                .filter(|flattened| flattened.name.to_ascii_lowercase().starts_with(&prefix));

            let Some(_member) = members.next() else {
                return Err(format!(
                    "native static .PRINT TRAN comparison could not find flattened members for subcircuit '{}'",
                    element.name
                ));
            };
        }

        Ok(())
    }

    fn validate_finite_controlled_source_gain(
        source_kind: &str,
        element_name: &str,
        value_name: &str,
        value: Value,
    ) -> Result<(), String> {
        if value.is_finite() {
            Ok(())
        } else {
            Err(format!(
                "native static .PRINT TRAN comparison does not support {source_kind} element '{element_name}' with non-finite {value_name} {value}"
            ))
        }
    }

    fn netlist_model_is_current_switch(netlist: &Netlist, model_name: &str) -> bool {
        Self::find_model(&netlist.models, model_name).is_some_and(|model| {
            matches!(
                model.model_type.to_ascii_uppercase().as_str(),
                "ISWITCH" | "ISW" | "CSW"
            )
        })
    }

    fn direct_branch_current_control(expression: &str) -> Option<String> {
        let normalized = Self::normalize_probe(expression);
        Self::parse_current_probe(&normalized)
    }

    fn validate_current_controlled_source_probe(
        elements: &[crate::netlist::Element],
        source_kind: &str,
        element_name: &str,
        control_element: &str,
    ) -> Result<(), String> {
        if Self::elements_have_recorded_branch_current(elements, control_element) {
            Ok(())
        } else {
            Err(format!(
                "native static .PRINT TRAN comparison does not support {source_kind} element '{element_name}' because controlling element '{control_element}' has no recorded branch current"
            ))
        }
    }

    fn validate_static_transient_passive_value_expression(
        device_kind: &str,
        element_name: &str,
        expression: &str,
        params: &crate::netlist::ParamContext,
    ) -> Result<(), String> {
        let prepared = prepare_behavioral_expression(expression, params).map_err(|err| {
            format!(
                "native static .PRINT TRAN comparison could not prepare {device_kind} value expression '{expression}' on element '{element_name}': {err}"
            )
        })?;
        let ast = parse_expression_strict(&prepared).map_err(|err| {
            format!(
                "native static .PRINT TRAN comparison does not yet support {device_kind} value expression '{expression}' on element '{element_name}': {err}"
            )
        })?;
        if Self::passive_value_expression_depends_on_runtime_quantity(&ast) {
            return Err(format!(
                "native static .PRINT TRAN comparison does not yet support runtime-dependent {device_kind} value expression '{expression}' on element '{element_name}'"
            ));
        }
        Ok(())
    }

    fn validate_transient_behavioral_expression(
        element_name: &str,
        expression: &str,
        params: &crate::netlist::ParamContext,
    ) -> Result<(), String> {
        let prepared = prepare_behavioral_expression(expression, params).map_err(|err| {
            format!(
                "native static .PRINT TRAN comparison could not prepare behavioral expression '{}' on element '{}': {err}",
                expression, element_name
            )
        })?;
        let ast = parse_expression_strict(&prepared).map_err(|err| {
            format!(
                "native static .PRINT TRAN comparison does not yet support behavioral expression '{}' on element '{}': {err}",
                expression, element_name
            )
        })?;
        let _validated_ast = ast;
        Ok(())
    }

    fn validate_ac_behavioral_expression(
        element_name: &str,
        expression: &str,
        params: &crate::netlist::ParamContext,
    ) -> Result<(), String> {
        let prepared = prepare_behavioral_expression(expression, params).map_err(|err| {
            format!(
                "native static .PRINT AC comparison could not prepare behavioral expression '{}' on element '{}': {err}",
                expression, element_name
            )
        })?;
        let ast = parse_expression_strict(&prepared).map_err(|err| {
            format!(
                "native static .PRINT AC comparison does not yet support behavioral expression '{}' on element '{}': {err}",
                expression, element_name
            )
        })?;
        if Self::expression_depends_on_ac_runtime_quantity(&ast) {
            return Err(format!(
                "native static .PRINT AC comparison does not support behavioral expression '{}' on element '{}' because AC behavioral small-signal linearization has no transient time or frequency-domain expression variable",
                expression, element_name
            ));
        }
        Ok(())
    }

    fn validate_transient_generic_switch_expression(
        element_name: &str,
        expression: &str,
        params: &crate::netlist::ParamContext,
    ) -> Result<(), String> {
        let prepared = prepare_behavioral_expression(expression, params).map_err(|err| {
            format!(
                "native static .PRINT TRAN comparison could not prepare generic switch CONTROL expression '{}' on element '{}': {err}",
                expression, element_name
            )
        })?;
        let ast = parse_expression_strict(&prepared).map_err(|err| {
            format!(
                "native static .PRINT TRAN comparison does not yet support generic switch CONTROL expression '{}' on element '{}': {err}",
                expression, element_name
            )
        })?;
        if Self::expression_depends_on_solution_quantity(&ast) {
            return Err(format!(
                "native static .PRINT TRAN comparison does not yet support generic switch CONTROL expression '{}' on element '{}' because it references circuit nodes or branch currents",
                expression, element_name
            ));
        }
        Ok(())
    }

    fn expression_depends_on_solution_quantity(expression: &Expr) -> bool {
        match expression {
            Expr::NodeVoltage(_) | Expr::BranchCurrent(_) => true,
            Expr::Unary { operand, .. } => Self::expression_depends_on_solution_quantity(operand),
            Expr::Binary { left, right, .. } => {
                Self::expression_depends_on_solution_quantity(left)
                    || Self::expression_depends_on_solution_quantity(right)
            }
            Expr::Function { args, .. } => args
                .iter()
                .any(Self::expression_depends_on_solution_quantity),
            Expr::Const(_)
            | Expr::StringLiteral(_)
            | Expr::LookupTable(_)
            | Expr::Time
            | Expr::Frequency
            | Expr::Temperature => false,
        }
    }

    fn expression_depends_on_ac_runtime_quantity(expression: &Expr) -> bool {
        match expression {
            Expr::Time | Expr::Frequency => true,
            Expr::Unary { operand, .. } => Self::expression_depends_on_ac_runtime_quantity(operand),
            Expr::Binary { left, right, .. } => {
                Self::expression_depends_on_ac_runtime_quantity(left)
                    || Self::expression_depends_on_ac_runtime_quantity(right)
            }
            Expr::Function { args, .. } => args
                .iter()
                .any(Self::expression_depends_on_ac_runtime_quantity),
            Expr::Const(_)
            | Expr::StringLiteral(_)
            | Expr::LookupTable(_)
            | Expr::NodeVoltage(_)
            | Expr::BranchCurrent(_)
            | Expr::Temperature => false,
        }
    }

    fn passive_value_expression_depends_on_runtime_quantity(expression: &Expr) -> bool {
        match expression {
            Expr::Const(_) | Expr::StringLiteral(_) | Expr::Temperature => false,
            Expr::NodeVoltage(_)
            | Expr::BranchCurrent(_)
            | Expr::LookupTable(_)
            | Expr::Time
            | Expr::Frequency => true,
            Expr::Unary { operand, .. } => {
                Self::passive_value_expression_depends_on_runtime_quantity(operand)
            }
            Expr::Binary { left, right, .. } => {
                Self::passive_value_expression_depends_on_runtime_quantity(left)
                    || Self::passive_value_expression_depends_on_runtime_quantity(right)
            }
            Expr::Function { args, .. } => args
                .iter()
                .any(Self::passive_value_expression_depends_on_runtime_quantity),
        }
    }

    fn validate_tran_probe(probe: &str, netlist: &Netlist) -> Result<(), String> {
        if let Some(expression) = Self::print_expression_inner(probe) {
            let normalized_expression = Self::normalize_probe(expression);
            if Self::braced_expression_is_atomic_real_probe(&normalized_expression, netlist) {
                return Self::validate_atomic_tran_probe(
                    &normalized_expression,
                    expression,
                    netlist,
                );
            }
            if Self::print_expression_contains_probe_reference(expression) {
                return Self::validate_tran_probe_expression(expression, netlist);
            }
            let context = Self::print_tran_eval_context(netlist, 0.0);
            crate::netlist::expr::eval_expression(expression, &context).map_err(|err| {
                format!("unsupported .PRINT TRAN expression '{{{expression}}}': {err}")
            })?;
            return Ok(());
        }

        let normalized = Self::normalize_probe(probe);
        Self::validate_atomic_tran_probe(&normalized, probe, netlist)
    }

    fn validate_ac_probe(probe: &str, netlist: &Netlist) -> Result<(), String> {
        if let Some(expression) = Self::print_expression_inner(probe) {
            let normalized_expression = Self::normalize_probe(expression);
            if Self::braced_expression_is_atomic_ac_probe(&normalized_expression, netlist) {
                return Self::validate_atomic_ac_probe(&normalized_expression, expression, netlist);
            }
            return Self::validate_ac_expression_probe(expression, netlist);
        }

        let normalized = Self::normalize_probe(probe);
        Self::validate_atomic_ac_probe(&normalized, probe, netlist)
            .or_else(|_| Self::validate_ac_expression_probe(probe, netlist))
    }

    fn validate_atomic_ac_probe(
        normalized: &str,
        original: &str,
        netlist: &Netlist,
    ) -> Result<(), String> {
        if let Some(voltage_probe) = Self::parse_ac_voltage_probe(normalized) {
            if !voltage_probe.node_pos.is_empty()
                && voltage_probe
                    .node_neg
                    .as_deref()
                    .is_none_or(|node| !node.is_empty())
            {
                return Ok(());
            }
        }
        if let Some(current_probe) = Self::parse_ac_current_probe(normalized)
            && Self::netlist_has_recorded_branch_current(netlist, &current_probe.element_name)
        {
            return Ok(());
        }
        if let Some((element_name, parameter)) = Self::parse_device_parameter_probe(normalized) {
            match parameter.as_str() {
                "acmag" | "acphase"
                    if Self::source_is_independent_source(netlist, &element_name) =>
                {
                    return Ok(());
                }
                "r" if Self::find_resistor_element(netlist, &element_name).is_some() => {
                    return Ok(());
                }
                "c" if Self::find_capacitor_element(netlist, &element_name).is_some() => {
                    return Ok(());
                }
                "l" if Self::find_inductor_element(netlist, &element_name).is_some() => {
                    return Ok(());
                }
                _ => {}
            }
        }
        Err(format!("unsupported .PRINT AC probe '{}'", original))
    }

    fn validate_ac_complex_probe(probe: &str, netlist: &Netlist) -> Result<(), String> {
        let normalized = Self::normalize_probe(probe);
        if let Some(voltage_probe) = Self::parse_ac_voltage_probe(&normalized) {
            if voltage_probe.accessor != XyceVoltageAccessor::Value {
                return Err(format!(
                    ".PRINT AC complex expression expects a complex V(...) argument, got '{}'",
                    probe.trim()
                ));
            }
            return Self::validate_atomic_ac_probe(&normalized, probe, netlist);
        }
        if let Some(current_probe) = Self::parse_ac_current_probe(&normalized) {
            if current_probe.accessor != XyceCurrentAccessor::Value {
                return Err(format!(
                    ".PRINT AC complex expression expects a complex I(...) argument, got '{}'",
                    probe.trim()
                ));
            }
            return Self::validate_atomic_ac_probe(&normalized, probe, netlist);
        }
        Self::validate_ac_expression_probe(probe, netlist)
    }

    fn validate_ac_expression_probe(expression: &str, netlist: &Netlist) -> Result<(), String> {
        let context = Self::print_eval_context(netlist, None, None);
        let mut call_value = |call: &str| {
            Self::validate_ac_expression_call_probe(call, netlist)?;
            Ok(ExprComplexValue::real(1.0))
        };
        let (rewritten, context) =
            Self::rewrite_ac_print_expression_complex(expression, context, &mut call_value)?;
        crate::netlist::expr::eval_expression_complex(&rewritten, &context)
            .map_err(|err| format!("unsupported .PRINT AC expression '{{{expression}}}': {err}"))?;
        Ok(())
    }

    fn validate_ac_expression_call_probe(call: &str, netlist: &Netlist) -> Result<(), String> {
        let normalized = Self::normalize_probe(call);
        if Self::parse_ac_voltage_probe(&normalized).is_some()
            || Self::parse_ac_current_probe(&normalized).is_some()
        {
            return Self::validate_atomic_ac_probe(&normalized, call, netlist);
        }
        Self::validate_ac_complex_probe(call, netlist)
    }

    fn validate_atomic_tran_probe(
        normalized: &str,
        original: &str,
        netlist: &Netlist,
    ) -> Result<(), String> {
        if let Some(voltage_probe) = Self::parse_tran_voltage_probe(normalized) {
            if !voltage_probe.node_pos.is_empty()
                && voltage_probe
                    .node_neg
                    .as_deref()
                    .is_none_or(|node| !node.is_empty())
            {
                return Ok(());
            }
        }
        if let Some(element_name) = Self::parse_current_probe(normalized) {
            if Self::netlist_has_recorded_branch_current(netlist, &element_name) {
                return Ok(());
            }
            if Self::netlist_has_independent_current_source(netlist, &element_name) {
                return Ok(());
            }
            return Err(format!(
                "transient branch-current probe '{}' targets an element without a recorded transient branch current",
                original
            ));
        }
        if let Some(element_name) = Self::parse_power_probe(normalized) {
            if Self::find_recorded_two_terminal_branch_element(netlist, &element_name).is_some() {
                return Ok(());
            }
            return Err(format!(
                "transient power probe '{}' targets an unsupported branch/device",
                original
            ));
        }
        if let Some((element_name, parameter)) = Self::parse_device_parameter_probe(normalized) {
            match parameter.as_str() {
                "r" if Self::find_resistor_element(netlist, &element_name).is_some() => {
                    return Ok(());
                }
                "c" if Self::find_capacitor_element(netlist, &element_name).is_some() => {
                    return Ok(());
                }
                "l" if Self::find_inductor_element(netlist, &element_name).is_some() => {
                    return Ok(());
                }
                "temp" if Self::resistor_temperature_value(netlist, &element_name).is_some() => {
                    return Ok(());
                }
                _ => {}
            }
            return Err(format!(
                "device parameter probe '{}' targets an unsupported transient parameter",
                original
            ));
        }
        if Self::normalize_probe(original) == "time" {
            return Ok(());
        }
        Err(format!("unsupported .PRINT TRAN probe '{}'", original))
    }

    fn validate_tran_probe_expression(expression: &str, netlist: &Netlist) -> Result<(), String> {
        let mut call_value = |call: &str| {
            let normalized = Self::normalize_probe(call);
            Self::validate_atomic_tran_probe(&normalized, call, netlist)?;
            Ok(1.0)
        };
        let context = Self::print_tran_eval_context(netlist, 0.0);
        Self::evaluate_print_expression_with_probe_calls(expression, context, &mut call_value)
            .map_err(|err| {
                format!("unsupported .PRINT TRAN expression '{{{expression}}}': {err}")
            })?;
        Ok(())
    }

    fn reject_unsupported_vbic_nested_current_source_sweeps(
        netlist: &Netlist,
        dc: &XyceDcSweep,
        print: &XycePrintRequest,
    ) -> Result<(), String> {
        let Some(sweep2) = &dc.sweep2 else {
            return Ok(());
        };
        if !Self::netlist_uses_native_vbic_bjt(netlist) {
            return Ok(());
        }

        let bias_points = dc.primary_spec().points().len() * sweep2.spec().points().len();
        if bias_points <= 1000 {
            return Ok(());
        }

        for probe in &print.probes {
            if Self::dc_probe_references_current_source_current(probe, netlist)? {
                return Err(format!(
                    "native VBIC nested DC sweep with {bias_points} bias points and current-source branch-current probes exceeds the current Xyce harness execution envelope; keep this named unsupported until VBIC nested-sweep continuation/performance is production-ready"
                ));
            }
        }

        Ok(())
    }

    fn is_expected_unsupported_runtime_error(err: &SimulationError) -> bool {
        let (SimulationError::Circuit(message) | SimulationError::Netlist(message)) = err else {
            return false;
        };
        let normalized = message.to_ascii_lowercase();
        normalized.contains("unsupported")
            || normalized.contains("not implemented")
            || normalized.contains("does not support")
            || normalized.contains("currently supports")
            || normalized.contains("no native implementation")
            || normalized.contains("no generated verilog-a builtin")
            || normalized.contains("no generated builtin")
            || normalized.contains("not yet")
            || normalized.contains("refusing")
            || normalized.contains("must not run through")
    }

    fn reject_unsupported_static_dc_model_observables(
        netlist: &Netlist,
        print: &XycePrintRequest,
    ) -> Result<(), String> {
        if !Self::netlist_uses_ekv3_level301_mosfet(netlist) {
            return Ok(());
        }

        let mut has_voltage_source_current_probe = false;
        for probe in &print.probes {
            if Self::dc_probe_references_voltage_source_current(probe, netlist)? {
                has_voltage_source_current_probe = true;
                break;
            }
        }
        if !has_voltage_source_current_probe {
            return Ok(());
        }

        if Self::netlist_uses_unsupported_ekv3_level301_branch_current_model(netlist) {
            return Err(
                "EKV3 LEVEL=301 static .PRINT DC voltage-source branch-current probes require a native validated EKV3 150 nm model; unsupported EKV3 LEVEL=301 cards remain fail-closed"
                    .to_string(),
            );
        }

        Ok(())
    }

    fn netlist_uses_ekv3_level301_mosfet(netlist: &Netlist) -> bool {
        if Self::elements_use_ekv3_level301_mosfet(&netlist.elements, &netlist.models, &[]) {
            return true;
        }

        crate::netlist::flatten_netlist_with_models(netlist).is_ok_and(|flattened| {
            Self::elements_use_ekv3_level301_mosfet(
                &flattened.elements,
                &netlist.models,
                &flattened.scoped_models,
            )
        })
    }

    fn netlist_uses_unsupported_ekv3_level301_branch_current_model(netlist: &Netlist) -> bool {
        if Self::elements_use_unsupported_ekv3_level301_branch_current_model(
            &netlist.elements,
            &netlist.models,
            &[],
        ) {
            return true;
        }

        crate::netlist::flatten_netlist_with_models(netlist).is_ok_and(|flattened| {
            Self::elements_use_unsupported_ekv3_level301_branch_current_model(
                &flattened.elements,
                &netlist.models,
                &flattened.scoped_models,
            )
        })
    }

    fn netlist_uses_native_vbic_bjt(netlist: &Netlist) -> bool {
        if Self::elements_use_native_vbic_bjt(&netlist.elements, &netlist.models, &[]) {
            return true;
        }

        crate::netlist::flatten_netlist_with_models(netlist).is_ok_and(|flattened| {
            Self::elements_use_native_vbic_bjt(
                &flattened.elements,
                &netlist.models,
                &flattened.scoped_models,
            )
        })
    }

    fn elements_use_ekv3_level301_mosfet(
        elements: &[crate::netlist::Element],
        models: &[crate::netlist::ModelDef],
        scoped_models: &[crate::netlist::ModelDef],
    ) -> bool {
        elements.iter().any(|element| {
            let ElementKind::Mosfet { model, .. } = &element.kind else {
                return false;
            };
            Self::find_model(scoped_models, model)
                .or_else(|| Self::find_model(models, model))
                .is_some_and(Self::model_is_ekv3_level301)
        })
    }

    fn elements_use_unsupported_ekv3_level301_branch_current_model(
        elements: &[crate::netlist::Element],
        models: &[crate::netlist::ModelDef],
        scoped_models: &[crate::netlist::ModelDef],
    ) -> bool {
        elements.iter().any(|element| {
            let ElementKind::Mosfet { model, .. } = &element.kind else {
                return false;
            };
            Self::find_model(scoped_models, model)
                .or_else(|| Self::find_model(models, model))
                .is_some_and(|model| {
                    Self::model_is_ekv3_level301(model)
                        && !Self::model_is_ekv3_level301_native_150nm_branch_current(model)
                })
        })
    }

    fn elements_use_native_vbic_bjt(
        elements: &[crate::netlist::Element],
        models: &[crate::netlist::ModelDef],
        scoped_models: &[crate::netlist::ModelDef],
    ) -> bool {
        elements.iter().any(|element| {
            let ElementKind::Bjt { model, .. } = &element.kind else {
                return false;
            };
            Self::find_model(scoped_models, model)
                .or_else(|| Self::find_model(models, model))
                .is_some_and(Self::model_is_native_vbic_bjt)
        })
    }

    fn find_model<'a>(
        models: &'a [crate::netlist::ModelDef],
        name: &str,
    ) -> Option<&'a crate::netlist::ModelDef> {
        models
            .iter()
            .find(|model| model.name.eq_ignore_ascii_case(name))
    }

    fn model_is_ekv3_level301(model: &crate::netlist::ModelDef) -> bool {
        matches!(
            model.model_type.to_ascii_uppercase().as_str(),
            "NMOS" | "PMOS"
        ) && model
            .params
            .iter()
            .rev()
            .find(|(name, _)| name.eq_ignore_ascii_case("LEVEL"))
            .is_some_and(|(_, value)| (*value - 301.0).abs() <= 1.0e-9)
    }

    fn model_is_ekv3_level301_native_150nm_branch_current(
        model: &crate::netlist::ModelDef,
    ) -> bool {
        matches!(
            model.model_type.to_ascii_uppercase().as_str(),
            "NMOS" | "PMOS"
        ) && Self::model_is_ekv3_level301(model)
    }

    fn model_is_native_vbic_bjt(model: &crate::netlist::ModelDef) -> bool {
        matches!(
            model.model_type.to_ascii_uppercase().as_str(),
            "NPN" | "PNP"
        ) && model
            .params
            .iter()
            .rev()
            .find(|(name, _)| name.eq_ignore_ascii_case("LEVEL"))
            .is_some_and(|(_, value)| {
                [4.0, 9.0, 11.0, 12.0, 13.0]
                    .iter()
                    .any(|level| (*value - *level).abs() <= 1.0e-9)
            })
    }

    fn dc_probe_references_voltage_source_current(
        probe: &str,
        netlist: &Netlist,
    ) -> Result<bool, String> {
        let normalized = Self::normalize_probe(probe);
        if let Some(source_name) = Self::parse_current_probe(&normalized) {
            return Ok(Self::source_is_voltage_source(netlist, &source_name));
        }

        let Some(expression) = Self::print_expression_inner(&normalized) else {
            return Ok(false);
        };
        if !Self::print_expression_contains_probe_call(expression) {
            return Ok(false);
        }

        let mut references_voltage_source_current = false;
        Self::rewrite_print_expression_calls(expression, netlist.params.clone(), |call| {
            if let Some(source_name) = Self::parse_current_probe(call)
                && Self::source_is_voltage_source(netlist, &source_name)
            {
                references_voltage_source_current = true;
            }
            Ok(0.0)
        })?;
        Ok(references_voltage_source_current)
    }

    fn dc_probe_references_current_source_current(
        probe: &str,
        netlist: &Netlist,
    ) -> Result<bool, String> {
        let normalized = Self::normalize_probe(probe);
        if let Some(source_name) = Self::parse_current_probe(&normalized) {
            return Ok(Self::source_is_current_source(netlist, &source_name));
        }

        let Some(expression) = Self::print_expression_inner(&normalized) else {
            return Ok(false);
        };
        if !Self::print_expression_contains_probe_call(expression) {
            return Ok(false);
        }

        let mut references_current_source_current = false;
        Self::rewrite_print_expression_calls(expression, netlist.params.clone(), |call| {
            if let Some(source_name) = Self::parse_current_probe(call)
                && Self::source_is_current_source(netlist, &source_name)
            {
                references_current_source_current = true;
            }
            Ok(0.0)
        })?;
        Ok(references_current_source_current)
    }

    fn validate_dc_sweep_source(netlist: &Netlist, source: &str) -> Result<(), String> {
        if Self::source_is_independent_source(netlist, source)
            || source.eq_ignore_ascii_case("TEMP")
            || source.eq_ignore_ascii_case("TEMPER")
            || Self::scalar_parameter_sweep_source_is_supported(netlist, source)
        {
            return Ok(());
        }
        Err(format!(
            "DC sweep source '{}' is not a supported top-level independent source, scalar parameter, or TEMP sweep",
            source
        ))
    }

    fn dc_sweep_point_netlist(
        netlist: &Netlist,
        dc: &XyceDcSweep,
        sweep_point: XyceDcSweepPoint,
    ) -> Result<Option<Netlist>, String> {
        let mut overrides = Vec::new();
        if Self::scalar_parameter_sweep_source_is_supported(netlist, &dc.source) {
            overrides.push((dc.source.clone(), sweep_point.primary));
        }
        if let Some(sweep2) = &dc.sweep2
            && Self::scalar_parameter_sweep_source_is_supported(netlist, &sweep2.source)
            && let Some(secondary) = sweep_point.secondary
        {
            overrides.push((sweep2.source.clone(), secondary));
        }
        if overrides.is_empty() {
            return Ok(None);
        }

        Engine::create_perturbed_netlist_multi(netlist, &overrides)
            .map(|(netlist, _)| Some(netlist))
            .map_err(|err| {
                format!(
                    "failed to build Xyce DC parameter-sweep netlist for {:?}: {}",
                    overrides, err
                )
            })
    }

    fn reject_unsupported_source_directives(source: &str) -> Result<(), String> {
        for line in Self::logical_netlist_lines(source) {
            let trimmed = Self::strip_netlist_comment(&line)
                .trim()
                .to_ascii_lowercase();
            if trimmed.is_empty() {
                continue;
            }
            if let Some(rest) = trimmed.strip_prefix(".options") {
                let option_tokens = rest
                    .split(|ch: char| ch.is_ascii_whitespace() || ch == ',')
                    .filter(|token| !token.is_empty())
                    .collect::<Vec<_>>();
                if option_tokens.iter().any(|token| *token == "loca")
                    || (option_tokens.iter().any(|token| *token == "nonlin")
                        && option_tokens
                            .iter()
                            .any(|token| token.starts_with("continuation")))
                {
                    return Err(
                        "deck requires Xyce LOCA continuation options; the native Xyce adapter does not yet implement continuation analysis semantics"
                            .to_string(),
                    );
                }
            }
        }
        Ok(())
    }

    fn validate_dc_probe(probe: &str, netlist: &Netlist) -> Result<(), String> {
        if let Some(expression) = Self::print_expression_inner(probe) {
            let normalized_expression = Self::normalize_probe(expression);
            if Self::braced_expression_is_atomic_real_probe(&normalized_expression, netlist) {
                return Self::validate_atomic_dc_probe(&normalized_expression, expression, netlist);
            }
            if Self::print_expression_contains_probe_reference(expression) {
                return Self::validate_dc_probe_expression(expression, netlist);
            }
            let context = Self::print_eval_context(netlist, None, None);
            crate::netlist::expr::eval_expression(expression, &context).map_err(|err| {
                format!("unsupported .PRINT DC expression '{{{expression}}}': {err}")
            })?;
            return Ok(());
        }

        let normalized = Self::normalize_probe(probe);
        Self::validate_atomic_dc_probe(&normalized, probe, netlist)
    }

    fn validate_atomic_dc_probe(
        normalized: &str,
        original: &str,
        netlist: &Netlist,
    ) -> Result<(), String> {
        if let Some(voltage_probe) = Self::parse_tran_voltage_probe(normalized) {
            if !voltage_probe.node_pos.is_empty()
                && voltage_probe
                    .node_neg
                    .as_deref()
                    .is_none_or(|node| !node.is_empty())
            {
                return Ok(());
            }
        }
        if let Some((element_name, parameter)) =
            Self::parse_device_operating_point_probe(normalized)
        {
            if !Self::netlist_has_device_op_instance(netlist, &element_name) {
                return Err(format!(
                    "device operating-point probe '{}' targets an unknown reported device",
                    original
                ));
            }
            if Self::canonical_device_op_parameter(&parameter).is_some() {
                return Ok(());
            }
            return Err(format!(
                "device operating-point probe '{}' targets an unsupported operating-point parameter",
                original
            ));
        }
        if let Some(lead_current) = Self::parse_lead_current_probe(normalized) {
            if Self::lead_current_probe_is_omitted_empty_wildcard(netlist, &lead_current) {
                return Ok(());
            }
            if lead_current.element_name == "*" {
                return Err(format!(
                    "lead-current wildcard probe '{}' requires terminal expansion support",
                    original
                ));
            }
            if !Self::netlist_has_device_op_instance(netlist, &lead_current.element_name) {
                return Err(format!(
                    "lead-current probe '{}' targets an unknown reported device",
                    original
                ));
            }
            if Self::netlist_supports_lead_current_probe(netlist, &lead_current) {
                return Ok(());
            }
            return Err(format!(
                "lead-current probe '{}' targets unsupported {} terminal current",
                original,
                lead_current.terminal.function_name()
            ));
        }
        if Self::bare_device_parameter_probe_is_supported(netlist, normalized) {
            return Ok(());
        }
        if let Some((element_name, parameter)) = Self::parse_device_parameter_probe(normalized) {
            match parameter.as_str() {
                "dcv0" if Self::source_is_independent_source(netlist, &element_name) => {
                    return Ok(());
                }
                "r" => {
                    if Self::find_resistor_element(netlist, &element_name).is_some() {
                        return Ok(());
                    }
                }
                "c" => {
                    if Self::find_capacitor_element(netlist, &element_name).is_some() {
                        return Ok(());
                    }
                }
                "l" => {
                    if Self::find_inductor_element(netlist, &element_name).is_some() {
                        return Ok(());
                    }
                    if Self::resistor_instance_parameter_probe_is_supported(
                        netlist,
                        &element_name,
                        &parameter,
                    ) {
                        return Ok(());
                    }
                }
                "temp" if Self::resistor_temperature_value(netlist, &element_name).is_some() => {
                    return Ok(());
                }
                _ => {
                    if Self::resistor_instance_parameter_probe_is_supported(
                        netlist,
                        &element_name,
                        &parameter,
                    ) {
                        return Ok(());
                    }
                }
            }
            if Self::model_parameter_probe_is_supported(netlist, &element_name, &parameter) {
                return Ok(());
            }
            return Err(format!(
                "device parameter probe '{}' targets an unsupported parameter",
                original
            ));
        }
        if let Some(parameter_name) = Self::parse_scalar_parameter_probe(normalized)
            && Self::scalar_parameter_probe_is_supported(netlist, &parameter_name)
        {
            return Ok(());
        }
        if let Some(element_name) = Self::parse_current_probe(normalized) {
            if Self::netlist_has_recorded_branch_current(netlist, &element_name) {
                return Ok(());
            }
            if Self::source_is_current_source(netlist, &element_name) {
                return Ok(());
            }
            if let Some(resistance) = Self::effective_resistor_value(netlist, &element_name) {
                if resistance.is_finite()
                    || (resistance.is_infinite() && resistance.is_sign_positive())
                {
                    return Ok(());
                }
                return Err(format!(
                    "current probe '{}' targets a resistor with invalid resistance {}",
                    original, resistance
                ));
            }
            return Err(format!(
                "current probe '{}' targets an unsupported branch/device",
                original
            ));
        }
        if let Some(element_name) = Self::parse_power_probe(normalized) {
            if let Some(resistance) = Self::effective_resistor_value(netlist, &element_name) {
                if resistance.is_finite()
                    || (resistance.is_infinite() && resistance.is_sign_positive())
                {
                    return Ok(());
                }
                return Err(format!(
                    "power probe '{}' targets a resistor with invalid resistance {}",
                    original, resistance
                ));
            }
            return Err(format!(
                "power probe '{}' targets an unsupported branch/device",
                original
            ));
        }
        Err(format!("unsupported .PRINT DC probe '{}'", original))
    }

    fn resistor_branch_form_tolerance(netlist: &Netlist) -> Value {
        netlist
            .options
            .device_zero_resistance_tol
            .unwrap_or(XYCE_DEFAULT_ZERO_RESISTANCE_TOL)
            .max(0.0)
    }

    fn resistor_uses_branch_form(netlist: &Netlist, resistance: Value) -> bool {
        resistance.is_finite() && resistance.abs() <= Self::resistor_branch_form_tolerance(netlist)
    }

    fn evaluate_dc_probe(
        probe: &str,
        netlist: &Netlist,
        dc: &XyceDcSweep,
        sweep_point: XyceDcSweepPoint,
        result: &crate::SimulationResult,
        op_report: &crate::circuit::DeviceOpReport,
    ) -> Result<f64, String> {
        if let Some(expression) = Self::print_expression_inner(probe) {
            let normalized_expression = Self::normalize_probe(expression);
            if Self::braced_expression_is_atomic_real_probe(&normalized_expression, netlist) {
                return Self::evaluate_atomic_dc_probe(
                    &normalized_expression,
                    netlist,
                    dc,
                    sweep_point,
                    result,
                    op_report,
                );
            }
            if Self::print_expression_contains_probe_reference(expression) {
                return Self::evaluate_dc_probe_expression(
                    expression,
                    netlist,
                    dc,
                    sweep_point,
                    result,
                    op_report,
                );
            }
            let context = Self::print_eval_context(netlist, Some(dc), Some(sweep_point));
            return crate::netlist::expr::eval_expression(expression, &context).map_err(|err| {
                format!("failed to evaluate .PRINT DC expression '{{{expression}}}': {err}")
            });
        }

        let normalized = Self::normalize_probe(probe);
        Self::evaluate_atomic_dc_probe(&normalized, netlist, dc, sweep_point, result, op_report)
    }

    fn evaluate_atomic_dc_probe(
        normalized: &str,
        netlist: &Netlist,
        dc: &XyceDcSweep,
        sweep_point: XyceDcSweepPoint,
        result: &crate::SimulationResult,
        op_report: &crate::circuit::DeviceOpReport,
    ) -> Result<f64, String> {
        if let Some(voltage_probe) = Self::parse_voltage_probe(normalized) {
            let pos = Self::result_voltage_named(result, netlist, &voltage_probe.node_pos)
                .ok_or_else(|| {
                    format!("node '{}' not found in DC result", voltage_probe.node_pos)
                })?;
            let neg = match voltage_probe.node_neg {
                Some(node) => Self::result_voltage_named(result, netlist, &node)
                    .ok_or_else(|| format!("node '{}' not found in DC result", node))?,
                None => 0.0,
            };
            return Ok(voltage_probe.accessor.evaluate_dc(pos - neg));
        }

        if let Some((element_name, parameter)) =
            Self::parse_device_operating_point_probe(normalized)
        {
            return Self::evaluate_device_operating_point_probe(
                op_report,
                &element_name,
                &parameter,
            );
        }

        if let Some(lead_current) = Self::parse_lead_current_probe(normalized) {
            return Self::evaluate_lead_current_probe(op_report, &lead_current);
        }

        if let Some(value) = Self::evaluate_bare_device_parameter_probe(
            netlist,
            dc,
            sweep_point,
            result,
            op_report,
            normalized,
        ) {
            return value;
        }

        if let Some((element_name, parameter)) = Self::parse_device_parameter_probe(normalized) {
            return Self::evaluate_device_parameter_probe(
                netlist,
                dc,
                sweep_point,
                result,
                op_report,
                &element_name,
                &parameter,
            );
        }

        if let Some(parameter_name) = Self::parse_scalar_parameter_probe(normalized) {
            return Self::evaluate_scalar_parameter_probe(
                netlist,
                dc,
                sweep_point,
                &parameter_name,
            );
        }

        if let Some(element_name) = Self::parse_current_probe(normalized) {
            if let Some(current) = Self::result_branch_current_named(result, &element_name) {
                return Ok(current);
            }
            if let Some(current) =
                Self::evaluate_current_source_current(netlist, dc, sweep_point, &element_name)
            {
                return Ok(current);
            }
            if let Some(resistance) = Self::effective_resistor_value(netlist, &element_name) {
                return Self::evaluate_resistor_current(netlist, result, &element_name, resistance);
            }
        }

        if let Some(element_name) = Self::parse_power_probe(normalized)
            && let Some(resistance) = Self::effective_resistor_value(netlist, &element_name)
        {
            return Self::evaluate_resistor_power(netlist, result, &element_name, resistance);
        }

        Err(format!("unsupported DC probe '{}'", normalized))
    }

    fn validate_dc_probe_expression(expression: &str, netlist: &Netlist) -> Result<(), String> {
        let mut call_value = |call: &str| {
            let normalized = Self::normalize_probe(call);
            Self::validate_atomic_dc_probe(&normalized, call, netlist)?;
            Ok(1.0)
        };
        let context = Self::print_eval_context(netlist, None, None);
        Self::evaluate_print_expression_with_probe_calls(expression, context, &mut call_value)
            .map_err(|err| format!("unsupported .PRINT DC expression '{{{expression}}}': {err}"))?;
        Ok(())
    }

    fn evaluate_dc_probe_expression(
        expression: &str,
        netlist: &Netlist,
        dc: &XyceDcSweep,
        sweep_point: XyceDcSweepPoint,
        result: &crate::SimulationResult,
        op_report: &crate::circuit::DeviceOpReport,
    ) -> Result<f64, String> {
        let context = Self::print_eval_context(netlist, Some(dc), Some(sweep_point));
        let mut call_value = |call: &str| {
            let normalized = Self::normalize_probe(call);
            Self::evaluate_atomic_dc_probe(&normalized, netlist, dc, sweep_point, result, op_report)
        };
        Self::evaluate_print_expression_with_probe_calls(expression, context, &mut call_value)
            .map_err(|err| {
                format!("failed to evaluate .PRINT DC expression '{{{expression}}}': {err}")
            })
    }

    fn evaluate_ac_reference_column(
        column: &XyceAcReferenceColumn,
        netlist: &Netlist,
        result: &AcResult,
        phase_output_radians: bool,
    ) -> Result<Value, String> {
        match column {
            XyceAcReferenceColumn::Probe { name, component } => match component {
                XyceAcProbeComponent::Scalar => {
                    Self::evaluate_ac_probe(name, netlist, result, phase_output_radians)
                }
                XyceAcProbeComponent::Real => {
                    Ok(Self::evaluate_ac_complex_probe(name, netlist, result)?.re)
                }
                XyceAcProbeComponent::Imaginary => {
                    Ok(Self::evaluate_ac_complex_probe(name, netlist, result)?.im)
                }
            },
        }
    }

    fn evaluate_ac_probe(
        probe: &str,
        netlist: &Netlist,
        result: &AcResult,
        phase_output_radians: bool,
    ) -> Result<Value, String> {
        if let Some(expression) = Self::print_expression_inner(probe) {
            let normalized_expression = Self::normalize_probe(expression);
            if Self::braced_expression_is_atomic_ac_probe(&normalized_expression, netlist) {
                return Self::evaluate_atomic_ac_probe(
                    &normalized_expression,
                    netlist,
                    result,
                    phase_output_radians,
                );
            }
            return Ok(Self::evaluate_ac_complex_expression(expression, netlist, result)?.re);
        }

        let normalized = Self::normalize_probe(probe);
        Self::evaluate_atomic_ac_probe(&normalized, netlist, result, phase_output_radians)
            .or_else(|_| Ok(Self::evaluate_ac_complex_expression(probe, netlist, result)?.re))
    }

    fn evaluate_atomic_ac_probe(
        normalized: &str,
        netlist: &Netlist,
        result: &AcResult,
        phase_output_radians: bool,
    ) -> Result<Value, String> {
        if let Some(voltage_probe) = Self::parse_ac_voltage_probe(normalized) {
            let value = Self::evaluate_ac_voltage_probe(&voltage_probe, netlist, result)?;
            return voltage_probe
                .accessor
                .evaluate_ac_scalar(value, phase_output_radians)
                .ok_or_else(|| {
                    format!(
                        "AC probe '{}' is complex-valued; compare Re()/Im() columns or use VM/VP/VDB",
                        normalized
                    )
                });
        }

        if let Some(current_probe) = Self::parse_ac_current_probe(normalized) {
            let current = Self::ac_branch_current_named(result, &current_probe.element_name)
                .ok_or_else(|| {
                    format!(
                        "branch '{}' not found in AC result",
                        current_probe.element_name
                    )
                })?;
            return current_probe
                .accessor
                .evaluate_ac_scalar(current, phase_output_radians)
                .ok_or_else(|| {
                    format!(
                        "AC probe '{}' is complex-valued; compare Re()/Im() columns or use IR/II/IM/IP/IDB",
                        normalized
                    )
                });
        }

        if let Some(value) = Self::evaluate_static_ac_device_parameter_probe(netlist, normalized) {
            return value;
        }

        Err(format!("unsupported AC probe '{}'", normalized))
    }

    fn evaluate_ac_complex_probe(
        probe: &str,
        netlist: &Netlist,
        result: &AcResult,
    ) -> Result<Complex64, String> {
        let normalized = Self::normalize_probe(probe);
        if let Some(voltage_probe) = Self::parse_ac_voltage_probe(&normalized) {
            if voltage_probe.accessor != XyceVoltageAccessor::Value {
                return Err(format!(
                    "AC complex probe '{}' must use bare V(...) accessor",
                    probe.trim()
                ));
            }
            return Self::evaluate_ac_voltage_probe(&voltage_probe, netlist, result);
        }
        if let Some(current_probe) = Self::parse_ac_current_probe(&normalized) {
            if current_probe.accessor != XyceCurrentAccessor::Value {
                return Err(format!(
                    "AC complex probe '{}' must use bare I(...) accessor",
                    probe.trim()
                ));
            }
            return Self::ac_branch_current_named(result, &current_probe.element_name).ok_or_else(
                || {
                    format!(
                        "branch '{}' not found in AC result",
                        current_probe.element_name
                    )
                },
            );
        }
        Self::evaluate_ac_complex_expression(probe, netlist, result)
    }

    fn evaluate_ac_complex_expression(
        expression: &str,
        netlist: &Netlist,
        result: &AcResult,
    ) -> Result<Complex64, String> {
        let context = Self::print_eval_context(netlist, None, None);
        let mut call_value =
            |call: &str| Self::evaluate_ac_expression_call_probe(call, netlist, result);
        let (rewritten, context) =
            Self::rewrite_ac_print_expression_complex(expression, context, &mut call_value)?;
        let value =
            crate::netlist::expr::eval_expression_complex(&rewritten, &context).map_err(|err| {
                format!("failed to evaluate .PRINT AC expression '{{{expression}}}': {err}")
            })?;
        Ok(Complex64::new(value.re, value.im))
    }

    fn evaluate_ac_expression_call_probe(
        call: &str,
        netlist: &Netlist,
        result: &AcResult,
    ) -> Result<ExprComplexValue, String> {
        let normalized = Self::normalize_probe(call);
        if let Some(voltage_probe) = Self::parse_ac_voltage_probe(&normalized) {
            let value = Self::evaluate_ac_voltage_probe(&voltage_probe, netlist, result)?;
            return Ok(
                match voltage_probe
                    .accessor
                    .evaluate_ac_scalar(value, false)
                    .map(ExprComplexValue::real)
                {
                    Some(value) => value,
                    None => ExprComplexValue::new(value.re, value.im),
                },
            );
        }
        if let Some(current_probe) = Self::parse_ac_current_probe(&normalized) {
            let current = Self::ac_branch_current_named(result, &current_probe.element_name)
                .ok_or_else(|| {
                    format!(
                        "branch '{}' not found in AC result",
                        current_probe.element_name
                    )
                })?;
            return Ok(
                match current_probe
                    .accessor
                    .evaluate_ac_scalar(current, false)
                    .map(ExprComplexValue::real)
                {
                    Some(value) => value,
                    None => ExprComplexValue::new(current.re, current.im),
                },
            );
        }
        let value = Self::evaluate_ac_complex_probe(call, netlist, result)?;
        Ok(ExprComplexValue::new(value.re, value.im))
    }

    fn evaluate_ac_voltage_probe(
        probe: &XyceVoltageProbe,
        netlist: &Netlist,
        result: &AcResult,
    ) -> Result<Complex64, String> {
        let pos = Self::ac_node_voltage_named(result, netlist, &probe.node_pos)
            .ok_or_else(|| format!("node '{}' not found in AC result", probe.node_pos))?;
        let neg = match probe.node_neg.as_deref() {
            Some(node) => Self::ac_node_voltage_named(result, netlist, node)
                .ok_or_else(|| format!("node '{}' not found in AC result", node))?,
            None => Complex64::new(0.0, 0.0),
        };
        Ok(pos - neg)
    }

    fn evaluate_static_ac_device_parameter_probe(
        netlist: &Netlist,
        normalized: &str,
    ) -> Option<Result<Value, String>> {
        let (element_name, parameter) = Self::parse_device_parameter_probe(normalized)?;
        Some(match parameter.as_str() {
            "acmag" => Self::independent_source_ac_terms(netlist, &element_name)
                .map(|(magnitude, _)| magnitude)
                .ok_or_else(|| {
                    format!(
                        "AC device parameter probe '{element_name}:ACMAG' has no independent source"
                    )
                }),
            "acphase" => Self::independent_source_ac_terms(netlist, &element_name)
                .map(|(_, phase)| phase.to_degrees())
                .ok_or_else(|| {
                    format!(
                        "AC device parameter probe '{element_name}:ACPHASE' has no independent source"
                    )
                }),
            "r" => Self::effective_resistor_value(netlist, &element_name).ok_or_else(|| {
                format!("AC device parameter probe '{element_name}:R' has no finite resistance")
            }),
            "c" => Self::effective_capacitor_value(netlist, &element_name).ok_or_else(|| {
                format!("AC device parameter probe '{element_name}:C' has no finite capacitance")
            }),
            "l" => Self::effective_inductor_value(netlist, &element_name).ok_or_else(|| {
                format!("AC device parameter probe '{element_name}:L' has no finite inductance")
            }),
            _ => Err(format!(
                "AC device parameter probe '{element_name}:{parameter}' is not supported"
            )),
        })
    }

    fn evaluate_tran_probe(
        probe: &str,
        netlist: &Netlist,
        result: &TransientResult,
        time: Value,
    ) -> Result<f64, String> {
        if let Some(expression) = Self::print_expression_inner(probe) {
            let normalized_expression = Self::normalize_probe(expression);
            if Self::braced_expression_is_atomic_real_probe(&normalized_expression, netlist) {
                return Self::evaluate_atomic_tran_probe(
                    &normalized_expression,
                    netlist,
                    result,
                    time,
                );
            }
            if Self::print_expression_contains_probe_reference(expression) {
                return Self::evaluate_tran_probe_expression(expression, netlist, result, time);
            }
            let context = Self::print_tran_eval_context(netlist, time);
            return crate::netlist::expr::eval_expression(expression, &context).map_err(|err| {
                format!("failed to evaluate .PRINT TRAN expression '{{{expression}}}': {err}")
            });
        }

        let normalized = Self::normalize_probe(probe);
        Self::evaluate_atomic_tran_probe(&normalized, netlist, result, time)
    }

    fn evaluate_atomic_tran_probe(
        normalized: &str,
        netlist: &Netlist,
        result: &TransientResult,
        time: Value,
    ) -> Result<f64, String> {
        if normalized == "time" {
            return Ok(time);
        }

        if let Some(voltage_probe) = Self::parse_tran_voltage_probe(normalized) {
            let pos =
                Self::transient_voltage_named(result, netlist, &voltage_probe.node_pos, time)?;
            let neg = match voltage_probe.node_neg {
                Some(node) => Self::transient_voltage_named(result, netlist, &node, time)?,
                None => 0.0,
            };
            return Ok(voltage_probe.accessor.evaluate_dc(pos - neg));
        }

        if let Some(element_name) = Self::parse_current_probe(normalized) {
            if let Some(current) = Self::transient_branch_current_named(result, &element_name, time)
            {
                return Ok(current);
            }
            if let Some(value) = Self::evaluate_independent_current_source_probe(
                netlist,
                result,
                &element_name,
                time,
            ) {
                return Ok(value);
            }
            return Err(format!(
                "branch current '{}' not found in transient result",
                element_name
            ));
        }

        if let Some(element_name) = Self::parse_power_probe(normalized) {
            if let Some(element) =
                Self::find_recorded_two_terminal_branch_element(netlist, &element_name)
            {
                return Self::evaluate_transient_two_terminal_branch_power(
                    result,
                    time,
                    "device",
                    &element.name,
                    &element,
                );
            }
            return Err(format!(
                "transient power probe '{}' targets an unsupported branch/device",
                element_name
            ));
        }

        if let Some((element_name, parameter)) = Self::parse_device_parameter_probe(normalized) {
            return Self::evaluate_transient_device_parameter_probe(
                netlist,
                result,
                time,
                &element_name,
                &parameter,
            );
        }

        Err(format!("unsupported TRAN probe '{}'", normalized))
    }

    fn evaluate_tran_probe_expression(
        expression: &str,
        netlist: &Netlist,
        result: &TransientResult,
        time: Value,
    ) -> Result<f64, String> {
        let context = Self::print_tran_eval_context(netlist, time);
        let mut call_value = |call: &str| {
            let normalized = Self::normalize_probe(call);
            Self::evaluate_atomic_tran_probe(&normalized, netlist, result, time)
        };
        Self::evaluate_print_expression_with_probe_calls(expression, context, &mut call_value)
            .map_err(|err| {
                format!("failed to evaluate .PRINT TRAN expression '{{{expression}}}': {err}")
            })
    }

    fn transient_branch_current_named(
        result: &TransientResult,
        branch_name: &str,
        time: Value,
    ) -> Option<Value> {
        Self::transient_branch_current_waveform_named(result, branch_name).and_then(|waveform| {
            Self::interpolate_transient_waveform_at(&result.time, waveform, time).ok()
        })
    }

    fn transient_branch_current_waveform_named<'a>(
        result: &'a TransientResult,
        branch_name: &str,
    ) -> Option<&'a [Value]> {
        result
            .try_branch_current_waveform_named(branch_name)
            .or_else(|| {
                let normalized = Self::normalize_device_instance_name(branch_name);
                (normalized != branch_name)
                    .then(|| result.try_branch_current_waveform_named(&normalized))?
            })
    }

    fn evaluate_transient_two_terminal_branch_power(
        result: &TransientResult,
        time: Value,
        device_kind: &str,
        branch_name: &str,
        element: &crate::netlist::Element,
    ) -> Result<Value, String> {
        let node_pos = element
            .nodes
            .first()
            .ok_or_else(|| format!("{device_kind} '{}' has no positive node", element.name))?;
        let node_neg = element
            .nodes
            .get(1)
            .ok_or_else(|| format!("{device_kind} '{}' has no negative node", element.name))?;
        let pos_index = result
            .node_index_named(node_pos)
            .ok_or_else(|| format!("node '{}' not found in transient result", node_pos))?;
        let neg_index = result
            .node_index_named(node_neg)
            .ok_or_else(|| format!("node '{}' not found in transient result", node_neg))?;
        let current = Self::transient_branch_current_waveform_named(result, branch_name)
            .ok_or_else(|| {
                format!(
                    "branch current '{}' not found in transient result",
                    branch_name
                )
            })?;

        if current.len() != result.time.len() {
            return Err(format!(
                "branch current '{}' has {} sample(s) for {} time point(s)",
                branch_name,
                current.len(),
                result.time.len()
            ));
        }

        let mut power = Vec::with_capacity(result.time.len());
        for index in 0..result.time.len() {
            let v_pos = if pos_index == 0 {
                0.0
            } else {
                result.try_voltage_at(pos_index, index).ok_or_else(|| {
                    format!(
                        "node '{}' sample {} not found in transient result",
                        node_pos, index
                    )
                })?
            };
            let v_neg = if neg_index == 0 {
                0.0
            } else {
                result.try_voltage_at(neg_index, index).ok_or_else(|| {
                    format!(
                        "node '{}' sample {} not found in transient result",
                        node_neg, index
                    )
                })?
            };
            power.push((v_pos - v_neg) * current[index]);
        }

        Self::interpolate_transient_waveform_at(&result.time, &power, time)
    }

    fn transient_voltage_named(
        result: &TransientResult,
        netlist: &Netlist,
        node_name: &str,
        time: Value,
    ) -> Result<Value, String> {
        let node = Self::node_lookup_candidates(netlist, node_name)
            .into_iter()
            .find_map(|candidate| result.node_index_named(&candidate))
            .ok_or_else(|| format!("node '{}' not found in transient result", node_name))?;
        if node == 0 {
            return Ok(0.0);
        }
        let waveform = result
            .try_voltage_waveform(node)
            .ok_or_else(|| format!("node '{}' not found in transient result", node_name))?;
        Self::interpolate_transient_waveform_at(&result.time, waveform, time)
    }

    fn validate_transient_result_time_grid(result: &TransientResult) -> Result<(), String> {
        if result.time.is_empty() {
            return Err("transient result has no time points".to_string());
        }
        for (index, time) in result.time.iter().copied().enumerate() {
            if !time.is_finite() {
                return Err(format!(
                    "transient result time point {index} is non-finite ({time})"
                ));
            }
            if index > 0 && time < result.time[index - 1] {
                return Err(format!(
                    "transient result time grid is not monotonic at point {index}"
                ));
            }
        }
        Ok(())
    }

    fn interpolate_transient_waveform_at(
        times: &[Value],
        values: &[Value],
        time: Value,
    ) -> Result<Value, String> {
        if times.len() != values.len() {
            return Err(format!(
                "transient waveform has {} sample(s) for {} time point(s)",
                values.len(),
                times.len()
            ));
        }
        let Some((&first_time, &last_time)) = times.first().zip(times.last()) else {
            return Err("transient waveform has no samples".to_string());
        };
        let scale = first_time.abs().max(last_time.abs()).max(time.abs());
        let edge_tol = (1.0e-12 * scale)
            .max(64.0 * f64::EPSILON * scale)
            .max(1.0e-30);
        if time < first_time - edge_tol || time > last_time + edge_tol {
            return Err(format!(
                "requested transient sample time {time:e} is outside simulated range [{first_time:e}, {last_time:e}]"
            ));
        }
        if time <= first_time + edge_tol {
            return Ok(values[0]);
        }
        if time >= last_time - edge_tol {
            return Ok(*values.last().expect("non-empty waveform"));
        }

        let upper = times.partition_point(|sample| *sample < time);
        if upper == 0 || upper >= times.len() {
            return Err(format!(
                "requested transient sample time {time:e} is outside interpolation brackets"
            ));
        }
        let lower = upper - 1;
        let t0 = times[lower];
        let t1 = times[upper];
        if (time - t0).abs() <= edge_tol {
            return Ok(values[lower]);
        }
        if (time - t1).abs() <= edge_tol {
            return Ok(values[upper]);
        }
        let dt = t1 - t0;
        if !dt.is_finite() || dt <= 0.0 {
            return Err(format!(
                "invalid transient interpolation interval [{t0:e}, {t1:e}]"
            ));
        }
        let alpha = (time - t0) / dt;
        Ok(values[lower] + alpha * (values[upper] - values[lower]))
    }

    fn evaluate_print_expression_with_probe_calls<F>(
        expression: &str,
        context: crate::netlist::ParamContext,
        call_value: &mut F,
    ) -> Result<Value, String>
    where
        F: FnMut(&str) -> Result<Value, String>,
    {
        Self::evaluate_print_expression_internal(expression, context, call_value, None)
    }

    fn evaluate_print_expression_internal<F>(
        expression: &str,
        context: crate::netlist::ParamContext,
        call_value: &mut F,
        override_probe: Option<(&str, Value)>,
    ) -> Result<Value, String>
    where
        F: FnMut(&str) -> Result<Value, String>,
    {
        let (rewritten, context) =
            Self::rewrite_print_ddx_calls(expression, context, call_value, override_probe)?;
        let (rewritten, context, _) =
            Self::rewrite_print_expression_calls_maybe(&rewritten, context, |call| {
                Self::print_probe_value(call, call_value, override_probe)
            })?;
        let (rewritten, context, _) =
            Self::rewrite_print_device_parameter_tokens_maybe(&rewritten, context, |call| {
                Self::print_probe_value(call, call_value, override_probe)
            })?;
        crate::netlist::expr::eval_expression(&rewritten, &context).map_err(|err| err.to_string())
    }

    fn print_probe_value<F>(
        call: &str,
        call_value: &mut F,
        override_probe: Option<(&str, Value)>,
    ) -> Result<Value, String>
    where
        F: FnMut(&str) -> Result<Value, String>,
    {
        if let Some((override_name, override_value)) = override_probe {
            if Self::normalize_probe(call) == override_name {
                return Ok(override_value);
            }
        }
        call_value(call)
    }

    fn rewrite_print_ddx_calls<F>(
        expression: &str,
        mut context: crate::netlist::ParamContext,
        call_value: &mut F,
        override_probe: Option<(&str, Value)>,
    ) -> Result<(String, crate::netlist::ParamContext), String>
    where
        F: FnMut(&str) -> Result<Value, String>,
    {
        let mut rewritten = String::with_capacity(expression.len());
        let mut index = 0usize;
        let mut placeholder_index = 0usize;

        while index < expression.len() {
            if let Some(open_index) = Self::print_ddx_call_open_index(expression, index) {
                let close_index = Self::matching_parenthesis_index(expression, open_index)?;
                let call = &expression[index..=close_index];
                let placeholder = format!("__rspice_ddx_{placeholder_index}");
                let value =
                    Self::evaluate_print_ddx_call(call, &context, call_value, override_probe)?;
                context.set(&placeholder, value);
                rewritten.push_str(&placeholder);
                placeholder_index += 1;
                index = close_index + 1;
                continue;
            }

            let ch = expression[index..]
                .chars()
                .next()
                .expect("valid char boundary");
            match ch {
                '{' => rewritten.push('('),
                '}' => rewritten.push(')'),
                _ => rewritten.push(ch),
            }
            index += ch.len_utf8();
        }

        Ok((rewritten, context))
    }

    fn evaluate_print_ddx_call<F>(
        call: &str,
        context: &crate::netlist::ParamContext,
        call_value: &mut F,
        override_probe: Option<(&str, Value)>,
    ) -> Result<Value, String>
    where
        F: FnMut(&str) -> Result<Value, String>,
    {
        let open_index = call
            .find('(')
            .ok_or_else(|| format!("malformed DDX call '{call}'"))?;
        let inner = call[open_index + 1..]
            .strip_suffix(')')
            .ok_or_else(|| format!("malformed DDX call '{call}'"))?;
        let args = Self::split_top_level_args(inner)?;
        if args.len() != 2 {
            return Err(format!(
                "DDX expects exactly two arguments, got {} in '{call}'",
                args.len()
            ));
        }

        let expression = args[0].trim();
        let variable = args[1].trim();
        if Self::parse_voltage_probe(variable).is_none()
            && Self::parse_current_probe(variable).is_none()
            && Self::parse_power_probe(variable).is_none()
            && Self::parse_lead_current_probe(variable).is_none()
            && Self::parse_device_operating_point_probe(variable).is_none()
            && Self::parse_device_parameter_probe(variable).is_none()
        {
            return Err(format!(
                "DDX derivative variable '{variable}' is not a supported print probe"
            ));
        }

        let normalized_variable = Self::normalize_probe(variable);
        let center = Self::print_probe_value(variable, call_value, override_probe)?;
        if !center.is_finite() {
            return Err(format!(
                "DDX derivative variable '{variable}' evaluated to non-finite value {center}"
            ));
        }

        Self::central_difference_derivative(center, |point| {
            Self::evaluate_print_expression_internal(
                expression,
                context.clone(),
                call_value,
                Some((&normalized_variable, point)),
            )
        })
    }

    fn central_difference_derivative<F>(center: Value, mut eval_at: F) -> Result<Value, String>
    where
        F: FnMut(Value) -> Result<Value, String>,
    {
        let scale = center.abs().max(1.0);
        let mut last_finite = None;
        for relative_step in [1.0e-4, 3.0e-5, 1.0e-5, 3.0e-6, 1.0e-6] {
            let step = scale * relative_step;
            let hi = eval_at(center + step)?;
            let lo = eval_at(center - step)?;
            let derivative = (hi - lo) / (2.0 * step);
            if derivative.is_finite() {
                last_finite = Some(derivative);
            }
        }
        last_finite.ok_or_else(|| "DDX derivative evaluated to a non-finite value".to_string())
    }

    fn split_top_level_args(input: &str) -> Result<Vec<String>, String> {
        let mut args = Vec::new();
        let mut start = 0usize;
        let mut paren_depth = 0usize;
        let mut brace_depth = 0usize;

        for (index, ch) in input.char_indices() {
            match ch {
                '(' => paren_depth += 1,
                ')' => {
                    paren_depth = paren_depth.checked_sub(1).ok_or_else(|| {
                        format!("unbalanced ')' while parsing function arguments '{input}'")
                    })?;
                }
                '{' => brace_depth += 1,
                '}' => {
                    brace_depth = brace_depth.checked_sub(1).ok_or_else(|| {
                        format!("unbalanced '}}' while parsing function arguments '{input}'")
                    })?;
                }
                ',' if paren_depth == 0 && brace_depth == 0 => {
                    args.push(input[start..index].trim().to_string());
                    start = index + ch.len_utf8();
                }
                _ => {}
            }
        }

        if paren_depth != 0 || brace_depth != 0 {
            return Err(format!(
                "unbalanced delimiters while parsing function arguments '{input}'"
            ));
        }
        args.push(input[start..].trim().to_string());
        if args.iter().any(|arg| arg.is_empty()) {
            return Err(format!("empty function argument in '{input}'"));
        }
        Ok(args)
    }

    fn print_ddx_call_open_index(expression: &str, index: usize) -> Option<usize> {
        if index >= expression.len() || !expression.is_char_boundary(index) {
            return None;
        }
        let tail = &expression[index..];
        if !tail
            .get(..3)
            .is_some_and(|prefix| prefix.eq_ignore_ascii_case("ddx"))
        {
            return None;
        }
        let previous = expression[..index].chars().next_back();
        if previous.is_some_and(Self::print_identifier_char) {
            return None;
        }

        let mut next_index = index + 3;
        while next_index < expression.len() {
            let ch = expression[next_index..].chars().next()?;
            if !ch.is_whitespace() {
                break;
            }
            next_index += ch.len_utf8();
        }
        expression[next_index..]
            .starts_with('(')
            .then_some(next_index)
    }

    fn rewrite_print_expression_calls<F>(
        expression: &str,
        context: crate::netlist::ParamContext,
        call_value: F,
    ) -> Result<(String, crate::netlist::ParamContext), String>
    where
        F: FnMut(&str) -> Result<Value, String>,
    {
        let (rewritten, context, placeholder_index) =
            Self::rewrite_print_expression_calls_maybe(expression, context, call_value)?;

        if placeholder_index == 0 {
            return Err(format!(
                ".PRINT DC expression '{{{expression}}}' does not contain a supported voltage, current, power, or device probe"
            ));
        }

        Ok((rewritten, context))
    }

    fn rewrite_print_expression_calls_maybe<F>(
        expression: &str,
        mut context: crate::netlist::ParamContext,
        mut call_value: F,
    ) -> Result<(String, crate::netlist::ParamContext, usize), String>
    where
        F: FnMut(&str) -> Result<Value, String>,
    {
        let mut rewritten = String::with_capacity(expression.len());
        let mut index = 0usize;
        let mut placeholder_index = 0usize;

        while index < expression.len() {
            if let Some(open_index) = Self::print_probe_call_open_index(expression, index) {
                let close_index = Self::matching_parenthesis_index(expression, open_index)?;
                let call = &expression[index..=close_index];
                let placeholder = format!("__rspice_probe_{placeholder_index}");
                let value = call_value(call)?;
                context.set(&placeholder, value);
                rewritten.push_str(&placeholder);
                placeholder_index += 1;
                index = close_index + 1;
                continue;
            }

            let ch = expression[index..]
                .chars()
                .next()
                .expect("valid char boundary");
            match ch {
                '{' => rewritten.push('('),
                '}' => rewritten.push(')'),
                _ => rewritten.push(ch),
            }
            index += ch.len_utf8();
        }

        Ok((rewritten, context, placeholder_index))
    }

    fn rewrite_ac_print_expression_complex<F>(
        expression: &str,
        mut context: crate::netlist::ParamContext,
        call_value: &mut F,
    ) -> Result<(String, crate::netlist::ParamContext), String>
    where
        F: FnMut(&str) -> Result<ExprComplexValue, String>,
    {
        let expression = Self::print_expression_inner(expression).unwrap_or(expression);
        let mut rewritten = String::with_capacity(expression.len());
        let mut index = 0usize;
        let mut placeholder_index = 0usize;

        while index < expression.len() {
            if let Some(open_index) = Self::print_probe_call_open_index(expression, index) {
                let close_index = Self::matching_parenthesis_index(expression, open_index)?;
                let call = &expression[index..=close_index];
                let placeholder = format!("__rspice_ac_probe_{placeholder_index}");
                let value = call_value(call)?;
                context.set_complex(&placeholder, value);
                rewritten.push_str(&placeholder);
                placeholder_index += 1;
                index = close_index + 1;
                continue;
            }

            let ch = expression[index..]
                .chars()
                .next()
                .expect("valid char boundary");
            match ch {
                '{' => rewritten.push('('),
                '}' => rewritten.push(')'),
                _ => rewritten.push(ch),
            }
            index += ch.len_utf8();
        }

        Ok((rewritten, context))
    }

    fn rewrite_print_device_parameter_tokens_maybe<F>(
        expression: &str,
        mut context: crate::netlist::ParamContext,
        mut call_value: F,
    ) -> Result<(String, crate::netlist::ParamContext, usize), String>
    where
        F: FnMut(&str) -> Result<Value, String>,
    {
        let mut rewritten = String::with_capacity(expression.len());
        let mut index = 0usize;
        let mut placeholder_index = 0usize;

        while index < expression.len() {
            if let Some((end_index, token)) =
                Self::print_device_parameter_token_at(expression, index)
            {
                let placeholder = format!("__rspice_param_{placeholder_index}");
                let value = call_value(token)?;
                context.set(&placeholder, value);
                rewritten.push_str(&placeholder);
                placeholder_index += 1;
                index = end_index;
                continue;
            }

            let ch = expression[index..]
                .chars()
                .next()
                .expect("valid char boundary");
            rewritten.push(ch);
            index += ch.len_utf8();
        }

        Ok((rewritten, context, placeholder_index))
    }

    fn print_expression_inner(probe: &str) -> Option<&str> {
        let trimmed = probe.trim();
        trimmed
            .strip_prefix('{')
            .and_then(|value| value.strip_suffix('}'))
            .map(str::trim)
            .filter(|value| !value.is_empty())
    }

    fn print_expression_contains_probe_call(expression: &str) -> bool {
        let mut index = 0usize;
        while index < expression.len() {
            if Self::print_probe_call_open_index(expression, index).is_some() {
                return true;
            }
            let ch = expression[index..]
                .chars()
                .next()
                .expect("valid char boundary");
            index += ch.len_utf8();
        }
        false
    }

    fn print_expression_contains_probe_reference(expression: &str) -> bool {
        let mut index = 0usize;
        while index < expression.len() {
            if Self::print_probe_call_open_index(expression, index).is_some()
                || Self::print_device_parameter_token_at(expression, index).is_some()
            {
                return true;
            }
            let ch = expression[index..]
                .chars()
                .next()
                .expect("valid char boundary");
            index += ch.len_utf8();
        }
        false
    }

    fn print_expression_contains_voltage_accessor_call(expression: &str) -> bool {
        let mut index = 0usize;
        while index < expression.len() {
            if let Some(open_index) = Self::print_probe_call_open_index(expression, index) {
                let call = &expression[index..open_index];
                if XyceVoltageAccessor::from_function_name(call)
                    .is_some_and(|accessor| accessor != XyceVoltageAccessor::Value)
                {
                    return true;
                }
            }
            let ch = expression[index..]
                .chars()
                .next()
                .expect("valid char boundary");
            index += ch.len_utf8();
        }
        false
    }

    fn braced_expression_is_atomic_real_probe(
        normalized_expression: &str,
        netlist: &Netlist,
    ) -> bool {
        Self::parse_device_parameter_probe(normalized_expression).is_some()
            || Self::bare_device_parameter_probe_is_atomic_real_probe(
                netlist,
                normalized_expression,
            )
            || Self::parse_device_operating_point_probe(normalized_expression)
                .is_some_and(|_| Self::probe_call_covers_entire_expression(normalized_expression))
            || Self::parse_lead_current_probe(normalized_expression)
                .is_some_and(|_| Self::probe_call_covers_entire_expression(normalized_expression))
            || Self::parse_tran_voltage_probe(normalized_expression)
                .is_some_and(|_| Self::probe_call_covers_entire_expression(normalized_expression))
            || Self::parse_current_probe(normalized_expression)
                .is_some_and(|_| Self::probe_call_covers_entire_expression(normalized_expression))
            || Self::parse_power_probe(normalized_expression)
                .is_some_and(|_| Self::probe_call_covers_entire_expression(normalized_expression))
    }

    fn bare_device_parameter_probe_is_atomic_real_probe(netlist: &Netlist, probe: &str) -> bool {
        let Some(probe_name) = Self::parse_bare_device_parameter_probe(probe) else {
            return false;
        };
        netlist.params.get_complex(&probe_name).is_none()
            && Self::bare_device_parameter_probe_is_supported(netlist, &probe_name)
    }

    fn braced_expression_is_atomic_ac_probe(
        normalized_expression: &str,
        netlist: &Netlist,
    ) -> bool {
        Self::parse_device_parameter_probe(normalized_expression).is_some()
            || Self::bare_device_parameter_probe_is_atomic_ac_probe(netlist, normalized_expression)
            || Self::parse_device_operating_point_probe(normalized_expression)
                .is_some_and(|_| Self::probe_call_covers_entire_expression(normalized_expression))
            || Self::parse_lead_current_probe(normalized_expression)
                .is_some_and(|_| Self::probe_call_covers_entire_expression(normalized_expression))
            || Self::parse_ac_voltage_probe(normalized_expression)
                .is_some_and(|_| Self::probe_call_covers_entire_expression(normalized_expression))
            || Self::parse_ac_current_probe(normalized_expression)
                .is_some_and(|_| Self::probe_call_covers_entire_expression(normalized_expression))
            || Self::parse_power_probe(normalized_expression)
                .is_some_and(|_| Self::probe_call_covers_entire_expression(normalized_expression))
    }

    fn bare_device_parameter_probe_is_atomic_ac_probe(netlist: &Netlist, probe: &str) -> bool {
        let Some(probe_name) = Self::parse_bare_device_parameter_probe(probe) else {
            return false;
        };
        netlist.params.get_complex(&probe_name).is_none()
            && Self::bare_device_parameter_probe_is_supported(netlist, &probe_name)
    }

    fn probe_call_covers_entire_expression(expression: &str) -> bool {
        let Some(open_index) = expression.find('(') else {
            return false;
        };
        if open_index == 0 || !expression.is_char_boundary(open_index) {
            return false;
        }
        Self::matching_parenthesis_index(expression, open_index)
            .is_ok_and(|close_index| close_index + 1 == expression.len())
    }

    fn print_probe_call_open_index(expression: &str, index: usize) -> Option<usize> {
        if index >= expression.len() || !expression.is_char_boundary(index) {
            return None;
        }
        let previous = expression[..index].chars().next_back();
        if previous.is_some_and(Self::print_identifier_char) {
            return None;
        }

        let rest = &expression[index..];
        for prefix in [
            "idb", "ir", "ii", "im", "ip", "id", "ig", "is", "ib", "ic", "ie", "vdb", "vr", "vi",
            "vm", "vp", "v", "i", "p", "w", "n",
        ] {
            let next_index = index + prefix.len();
            if rest.len() <= prefix.len()
                || !expression.is_char_boundary(next_index)
                || !rest[..prefix.len()].eq_ignore_ascii_case(prefix)
                || !expression[next_index..].starts_with('(')
            {
                continue;
            }
            return Some(next_index);
        }
        None
    }

    fn print_device_parameter_token_at(expression: &str, index: usize) -> Option<(usize, &str)> {
        if index >= expression.len() || !expression.is_char_boundary(index) {
            return None;
        }
        let previous = expression[..index].chars().next_back();
        if previous.is_some_and(Self::print_device_parameter_token_char) {
            return None;
        }

        let first = expression[index..].chars().next()?;
        if !(first.is_ascii_alphabetic() || first == '_') {
            return None;
        }

        let mut end_index = index;
        let mut has_colon = false;
        while end_index < expression.len() {
            let ch = expression[end_index..].chars().next()?;
            if !Self::print_device_parameter_token_char(ch) {
                break;
            }
            has_colon |= ch == ':';
            end_index += ch.len_utf8();
        }
        if !has_colon {
            return None;
        }

        let token = &expression[index..end_index];
        Self::parse_device_parameter_probe(token).map(|_| (end_index, token))
    }

    fn print_device_parameter_token_char(ch: char) -> bool {
        ch.is_ascii_alphanumeric() || matches!(ch, '_' | '.' | '$' | ':')
    }

    fn print_identifier_char(ch: char) -> bool {
        ch.is_ascii_alphanumeric() || matches!(ch, '_' | '.' | '$')
    }

    fn matching_parenthesis_index(expression: &str, open_index: usize) -> Result<usize, String> {
        if !expression[open_index..].starts_with('(') {
            return Err(format!(
                "internal error: expected '(' in .PRINT expression '{expression}'"
            ));
        }

        let mut depth = 0usize;
        for (relative_index, ch) in expression[open_index..].char_indices() {
            let absolute_index = open_index + relative_index;
            match ch {
                '(' => depth += 1,
                ')' => {
                    depth = depth.saturating_sub(1);
                    if depth == 0 {
                        return Ok(absolute_index);
                    }
                }
                _ => {}
            }
        }

        Err(format!(
            "unterminated probe call in .PRINT expression '{{{expression}}}'"
        ))
    }

    fn evaluate_device_parameter_probe(
        netlist: &Netlist,
        dc: &XyceDcSweep,
        sweep_point: XyceDcSweepPoint,
        result: &crate::SimulationResult,
        op_report: &crate::circuit::DeviceOpReport,
        element_name: &str,
        parameter: &str,
    ) -> Result<f64, String> {
        match parameter {
            "dcv0" => {
                if element_name.eq_ignore_ascii_case(&dc.source) {
                    return Ok(sweep_point.primary);
                }
                if let Some(sweep2) = &dc.sweep2 {
                    if element_name.eq_ignore_ascii_case(&sweep2.source) {
                        return sweep_point.secondary.ok_or_else(|| {
                            format!(
                                "secondary sweep value for '{}' is unavailable",
                                element_name
                            )
                        });
                    }
                }
                Self::independent_source_dc_value(netlist, element_name).ok_or_else(|| {
                    format!(
                        "source parameter probe '{}:DCV0' targets an unknown independent source",
                        element_name
                    )
                })
            }
            "r" => Self::evaluate_resistor_parameter_r_value(
                netlist,
                dc,
                sweep_point,
                result,
                op_report,
                element_name,
            ),
            "c" => {
                Self::evaluate_capacitor_parameter_c_value(netlist, dc, sweep_point, element_name)
            }
            "l" => {
                if Self::find_inductor_element(netlist, element_name).is_some() {
                    return Self::evaluate_inductor_parameter_l_value(
                        netlist,
                        dc,
                        sweep_point,
                        element_name,
                    );
                }
                if Self::resistor_instance_parameter_probe_is_supported(
                    netlist,
                    element_name,
                    parameter,
                ) {
                    return Self::evaluate_resistor_instance_parameter_probe(
                        netlist,
                        element_name,
                        parameter,
                    );
                }
                Self::evaluate_model_parameter_probe(
                    netlist,
                    dc,
                    sweep_point,
                    element_name,
                    parameter,
                )
                .unwrap_or_else(|| {
                    Err(format!(
                        "device parameter probe '{}:{}' is not supported",
                        element_name, parameter
                    ))
                })
            }
            "temp" => Self::resistor_temperature_value(netlist, element_name).ok_or_else(|| {
                format!(
                    "resistor parameter probe '{}:TEMP' targets an unknown resistor",
                    element_name
                )
            }),
            _ => {
                if Self::resistor_instance_parameter_probe_is_supported(
                    netlist,
                    element_name,
                    parameter,
                ) {
                    return Self::evaluate_resistor_instance_parameter_probe(
                        netlist,
                        element_name,
                        parameter,
                    );
                }
                Self::evaluate_model_parameter_probe(
                    netlist,
                    dc,
                    sweep_point,
                    element_name,
                    parameter,
                )
                .unwrap_or_else(|| {
                    Err(format!(
                        "device parameter probe '{}:{}' is not supported",
                        element_name, parameter
                    ))
                })
            }
        }
    }

    fn model_parameter_probe_is_supported(
        netlist: &Netlist,
        model_name: &str,
        parameter: &str,
    ) -> bool {
        Self::models_have_parameter_probe(&netlist.models, model_name, parameter)
            || crate::netlist::flatten_netlist_with_models(netlist).is_ok_and(|flattened| {
                Self::models_have_parameter_probe(&flattened.scoped_models, model_name, parameter)
            })
    }

    fn models_have_parameter_probe(
        models: &[crate::netlist::ModelDef],
        model_name: &str,
        parameter: &str,
    ) -> bool {
        Self::find_model(models, model_name).is_some_and(|model| {
            model
                .params
                .iter()
                .any(|(name, _)| name.eq_ignore_ascii_case(parameter))
                || model
                    .expr_params
                    .iter()
                    .any(|(name, _)| name.eq_ignore_ascii_case(parameter))
        })
    }

    fn evaluate_model_parameter_probe(
        netlist: &Netlist,
        dc: &XyceDcSweep,
        sweep_point: XyceDcSweepPoint,
        model_name: &str,
        parameter: &str,
    ) -> Option<Result<Value, String>> {
        Self::model_parameter_probe_value(
            &netlist.models,
            netlist,
            dc,
            sweep_point,
            model_name,
            parameter,
        )
        .or_else(|| {
            let flattened = crate::netlist::flatten_netlist_with_models(netlist).ok()?;
            Self::model_parameter_probe_value(
                &flattened.scoped_models,
                netlist,
                dc,
                sweep_point,
                model_name,
                parameter,
            )
        })
    }

    fn model_parameter_probe_value(
        models: &[crate::netlist::ModelDef],
        netlist: &Netlist,
        dc: &XyceDcSweep,
        sweep_point: XyceDcSweepPoint,
        model_name: &str,
        parameter: &str,
    ) -> Option<Result<Value, String>> {
        let model = Self::find_model(models, model_name)?;
        if let Some((_, value)) = model
            .params
            .iter()
            .rev()
            .find(|(name, _)| name.eq_ignore_ascii_case(parameter))
        {
            return Some(Ok(*value));
        }

        model
            .expr_params
            .iter()
            .rev()
            .find(|(name, _)| name.eq_ignore_ascii_case(parameter))
            .map(|(_, expression)| {
                let context = Self::print_eval_context(netlist, Some(dc), Some(sweep_point));
                crate::netlist::expr::eval_expression(expression, &context).map_err(|err| {
                    format!(
                        "failed to evaluate model parameter probe '{}:{}': {err}",
                        model_name, parameter
                    )
                })
            })
    }

    fn bare_device_parameter_probe_is_supported(netlist: &Netlist, probe: &str) -> bool {
        Self::find_bare_device_parameter_element(netlist, probe).is_some()
    }

    fn evaluate_bare_device_parameter_probe(
        netlist: &Netlist,
        dc: &XyceDcSweep,
        sweep_point: XyceDcSweepPoint,
        result: &crate::SimulationResult,
        op_report: &crate::circuit::DeviceOpReport,
        probe: &str,
    ) -> Option<Result<f64, String>> {
        let probe_name = Self::parse_bare_device_parameter_probe(probe)?;
        let element = Self::find_bare_device_parameter_element(netlist, &probe_name)?;
        Some(match &element.kind {
            ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => Ok(
                Self::source_dc_parameter_value(dc, sweep_point, &element.name, spec),
            ),
            ElementKind::Resistor { .. } => Self::evaluate_resistor_parameter_r_value(
                netlist,
                dc,
                sweep_point,
                result,
                op_report,
                &probe_name,
            ),
            ElementKind::Capacitor {
                value,
                value_expr,
                instance_params,
                ..
            } => Self::evaluate_static_passive_parameter_value(
                netlist,
                dc,
                sweep_point,
                "capacitor",
                &probe_name,
                "C",
                *value,
                value_expr.as_deref(),
                instance_params,
            ),
            ElementKind::Inductor {
                value,
                value_expr,
                instance_params,
                ..
            } => Self::evaluate_static_passive_parameter_value(
                netlist,
                dc,
                sweep_point,
                "inductor",
                &probe_name,
                "L",
                *value,
                value_expr.as_deref(),
                instance_params,
            ),
            _ => Err(format!(
                "bare device parameter probe '{}' targets an unsupported element kind",
                probe_name
            )),
        })
    }

    fn source_dc_parameter_value(
        dc: &XyceDcSweep,
        sweep_point: XyceDcSweepPoint,
        element_name: &str,
        spec: &crate::netlist::SourceSpec,
    ) -> Value {
        if Self::device_instance_names_match(element_name, &dc.source) {
            return sweep_point.primary;
        }
        if let Some(sweep2) = &dc.sweep2
            && Self::device_instance_names_match(element_name, &sweep2.source)
            && let Some(value) = sweep_point.secondary
        {
            return value;
        }
        extract_dc_value(spec)
    }

    fn evaluate_static_passive_parameter_value(
        netlist: &Netlist,
        dc: &XyceDcSweep,
        sweep_point: XyceDcSweepPoint,
        device_kind: &str,
        element_name: &str,
        parameter_name: &str,
        value: Value,
        value_expr: Option<&str>,
        instance_params: &[(String, Value)],
    ) -> Result<Value, String> {
        if let Some(instance_value) =
            Self::instance_param(instance_params, &[parameter_name, "VALUE"])
        {
            return Ok(instance_value);
        }
        if value.is_finite() {
            return Ok(value);
        }
        if let Some(expression) = value_expr {
            let context = Self::print_eval_context(netlist, Some(dc), Some(sweep_point));
            return crate::netlist::expr::eval_expression(expression, &context).map_err(|err| {
                format!(
                    "failed to evaluate {device_kind} parameter probe '{element_name}:{parameter_name}': {err}"
                )
            });
        }
        Err(format!(
            "{device_kind} parameter probe '{element_name}:{parameter_name}' could not resolve a value"
        ))
    }

    fn evaluate_capacitor_parameter_c_value(
        netlist: &Netlist,
        dc: &XyceDcSweep,
        sweep_point: XyceDcSweepPoint,
        name: &str,
    ) -> Result<Value, String> {
        let element = Self::find_capacitor_element(netlist, name).ok_or_else(|| {
            format!("capacitor parameter probe '{name}:C' targets an unknown capacitor")
        })?;
        let ElementKind::Capacitor {
            value,
            value_expr,
            model,
            instance_params,
            ..
        } = &element.kind
        else {
            return Err(format!(
                "capacitor parameter probe '{name}:C' targets a non-capacitor element"
            ));
        };
        if (model.is_some()
            || Self::capacitor_instance_params_affect_effective_value(instance_params))
            && let Some(capacitance) = Self::effective_capacitor_value(netlist, name)
        {
            return Ok(capacitance);
        }
        Self::evaluate_static_passive_parameter_value(
            netlist,
            dc,
            sweep_point,
            "capacitor",
            name,
            "C",
            *value,
            value_expr.as_deref(),
            instance_params,
        )
    }

    fn evaluate_inductor_parameter_l_value(
        netlist: &Netlist,
        dc: &XyceDcSweep,
        sweep_point: XyceDcSweepPoint,
        name: &str,
    ) -> Result<Value, String> {
        let element = Self::find_inductor_element(netlist, name).ok_or_else(|| {
            format!("inductor parameter probe '{name}:L' targets an unknown inductor")
        })?;
        let ElementKind::Inductor {
            value,
            value_expr,
            model,
            instance_params,
            ..
        } = &element.kind
        else {
            return Err(format!(
                "inductor parameter probe '{name}:L' targets a non-inductor element"
            ));
        };
        if (model.is_some()
            || Self::inductor_instance_params_affect_effective_value(instance_params))
            && let Some(inductance) = Self::effective_inductor_value(netlist, name)
        {
            return Ok(inductance);
        }
        Self::evaluate_static_passive_parameter_value(
            netlist,
            dc,
            sweep_point,
            "inductor",
            name,
            "L",
            *value,
            value_expr.as_deref(),
            instance_params,
        )
    }

    fn evaluate_transient_device_parameter_probe(
        netlist: &Netlist,
        result: &TransientResult,
        time: Value,
        element_name: &str,
        parameter: &str,
    ) -> Result<f64, String> {
        match parameter {
            "r" => Self::evaluate_transient_resistor_parameter_r_value(
                netlist,
                result,
                time,
                element_name,
            ),
            "c" => Self::evaluate_transient_capacitor_parameter_c_value(
                netlist,
                result,
                time,
                element_name,
            ),
            "l" => Self::evaluate_transient_inductor_parameter_l_value(
                netlist,
                result,
                time,
                element_name,
            ),
            "temp" => Self::resistor_temperature_value(netlist, element_name).ok_or_else(|| {
                format!(
                    "resistor parameter probe '{}:TEMP' targets an unknown resistor",
                    element_name
                )
            }),
            _ => Err(format!(
                "device parameter probe '{}:{}' is not supported in transient output",
                element_name, parameter
            )),
        }
    }

    fn evaluate_transient_static_passive_parameter_value(
        netlist: &Netlist,
        result: &TransientResult,
        time: Value,
        device_kind: &str,
        element_name: &str,
        parameter_name: &str,
        value: Value,
        value_expr: Option<&str>,
        instance_params: &[(String, Value)],
    ) -> Result<Value, String> {
        if let Some(instance_value) =
            Self::instance_param(instance_params, &[parameter_name, "VALUE"])
        {
            return Ok(instance_value);
        }
        if value.is_finite() {
            return Ok(value);
        }
        if let Some(expression) = value_expr {
            let context = Self::print_tran_eval_context(netlist, time);
            let mut call_value = |call: &str| {
                let normalized = Self::normalize_probe(call);
                Self::evaluate_atomic_tran_probe(&normalized, netlist, result, time)
            };
            return Self::evaluate_print_expression_with_probe_calls(
                expression,
                context,
                &mut call_value,
            )
            .map_err(|err| {
                format!(
                    "failed to evaluate transient {device_kind} parameter probe '{element_name}:{parameter_name}': {err}"
                )
            });
        }
        Err(format!(
            "{device_kind} parameter probe '{element_name}:{parameter_name}' could not resolve a value"
        ))
    }

    fn evaluate_transient_resistor_parameter_r_value(
        netlist: &Netlist,
        result: &TransientResult,
        time: Value,
        name: &str,
    ) -> Result<Value, String> {
        let element = Self::find_resistor_element(netlist, name).ok_or_else(|| {
            format!("resistor parameter probe '{name}:R' targets an unknown resistor")
        })?;
        let ElementKind::Resistor {
            value,
            value_expr,
            model,
            instance_params,
            ..
        } = &element.kind
        else {
            return Err(format!(
                "resistor parameter probe '{name}:R' targets a non-resistor element"
            ));
        };

        if Self::resistor_uses_xyce_default_marker(instance_params)
            && let Some(resistance) = Self::effective_resistor_value(netlist, name)
        {
            return Ok(resistance);
        }
        let value = Self::evaluate_transient_static_passive_parameter_value(
            netlist,
            result,
            time,
            "resistor",
            name,
            "R",
            *value,
            value_expr.as_deref(),
            instance_params,
        );
        if value.is_ok() {
            return value;
        }
        if model.is_some()
            && let Some(resistance) = Self::resistor_parameter_r_value(netlist, name)
        {
            return Ok(resistance);
        }
        value
    }

    fn evaluate_transient_capacitor_parameter_c_value(
        netlist: &Netlist,
        result: &TransientResult,
        time: Value,
        name: &str,
    ) -> Result<Value, String> {
        let element = Self::find_capacitor_element(netlist, name).ok_or_else(|| {
            format!("capacitor parameter probe '{name}:C' targets an unknown capacitor")
        })?;
        let ElementKind::Capacitor {
            value,
            value_expr,
            model,
            instance_params,
            ..
        } = &element.kind
        else {
            return Err(format!(
                "capacitor parameter probe '{name}:C' targets a non-capacitor element"
            ));
        };
        if (model.is_some()
            || Self::capacitor_instance_params_affect_effective_value(instance_params))
            && let Some(capacitance) = Self::effective_capacitor_value(netlist, name)
        {
            return Ok(capacitance);
        }
        Self::evaluate_transient_static_passive_parameter_value(
            netlist,
            result,
            time,
            "capacitor",
            name,
            "C",
            *value,
            value_expr.as_deref(),
            instance_params,
        )
    }

    fn evaluate_transient_inductor_parameter_l_value(
        netlist: &Netlist,
        result: &TransientResult,
        time: Value,
        name: &str,
    ) -> Result<Value, String> {
        let element = Self::find_inductor_element(netlist, name).ok_or_else(|| {
            format!("inductor parameter probe '{name}:L' targets an unknown inductor")
        })?;
        let ElementKind::Inductor {
            value,
            value_expr,
            model,
            instance_params,
            ..
        } = &element.kind
        else {
            return Err(format!(
                "inductor parameter probe '{name}:L' targets a non-inductor element"
            ));
        };
        if (model.is_some()
            || Self::inductor_instance_params_affect_effective_value(instance_params))
            && let Some(inductance) = Self::effective_inductor_value(netlist, name)
        {
            return Ok(inductance);
        }
        Self::evaluate_transient_static_passive_parameter_value(
            netlist,
            result,
            time,
            "inductor",
            name,
            "L",
            *value,
            value_expr.as_deref(),
            instance_params,
        )
    }

    fn evaluate_device_operating_point_probe(
        op_report: &crate::circuit::DeviceOpReport,
        element_name: &str,
        parameter: &str,
    ) -> Result<f64, String> {
        let canonical_parameter =
            Self::canonical_device_op_parameter(parameter).ok_or_else(|| {
                format!(
                    "device operating-point probe 'N({element_name}:{parameter})' targets an unsupported operating-point parameter"
                )
            })?;

        for entry in &op_report.entries {
            if !Self::device_instance_names_match(&entry.name, element_name) {
                continue;
            }
            if let Some(value) = Self::xyce_device_operating_point_value(entry, canonical_parameter)
            {
                return Ok(value);
            }
            return Err(format!(
                "device operating-point probe 'N({element_name}:{parameter})' targets parameter '{}' that is not reported for {} '{}'",
                canonical_parameter, entry.device_kind, entry.name
            ));
        }

        Err(format!(
            "device operating-point probe 'N({element_name}:{parameter})' targets a device with no operating-point report"
        ))
    }

    fn evaluate_lead_current_probe(
        op_report: &crate::circuit::DeviceOpReport,
        probe: &XyceLeadCurrentProbe,
    ) -> Result<f64, String> {
        let parameter = probe.terminal.op_parameter().ok_or_else(|| {
            format!(
                "lead-current probe '{}({})' targets unsupported terminal current",
                probe.terminal.function_name(),
                probe.element_name
            )
        })?;

        for entry in &op_report.entries {
            if !Self::device_instance_names_match(&entry.name, &probe.element_name) {
                continue;
            }
            if let Some(value) = Self::xyce_device_operating_point_value(entry, parameter) {
                return Ok(value);
            }
            return Err(format!(
                "lead-current probe '{}({})' targets parameter '{}' that is not reported for {} '{}'",
                probe.terminal.function_name(),
                probe.element_name,
                parameter,
                entry.device_kind,
                entry.name
            ));
        }

        Err(format!(
            "lead-current probe '{}({})' targets a device with no operating-point report",
            probe.terminal.function_name(),
            probe.element_name
        ))
    }

    fn xyce_device_operating_point_value(
        entry: &crate::circuit::DeviceOpEntry,
        parameter: &str,
    ) -> Option<Value> {
        if matches!(entry.device_kind, "BSIM3" | "BSIM4") {
            return Self::xyce_bsim_device_store_value(entry, parameter);
        }

        Self::device_op_entry_param(entry, parameter)
    }

    fn xyce_bsim_device_store_value(
        entry: &crate::circuit::DeviceOpEntry,
        parameter: &str,
    ) -> Option<Value> {
        let raw = if parameter == "vdsat" && entry.device_kind == "BSIM4" {
            Self::device_op_entry_param(entry, "output_vdsat")
                .or_else(|| Self::device_op_entry_param(entry, parameter))?
        } else {
            Self::device_op_entry_param(entry, parameter)?
        };
        let vds = Self::device_op_entry_param(entry, "vds").unwrap_or(0.0);
        if vds >= 0.0 {
            return Some(raw);
        }

        match parameter {
            // Xyce stores BSIM3/BSIM4 gm after the same inverse-mode sign
            // swap it applies to the MNA stamp.
            "gm" => Some(-raw),
            // Xyce's Vds/Vgs/Vbs store nodes are the mode-frame branch
            // voltages: Vds, Vgs, Vbs in normal mode; -Vds, Vgd, Vbd in
            // inverse mode.
            "vds" => Some(-vds),
            "vgs" => Some(raw - vds),
            "vbs" => Some(raw - vds),
            _ => Some(raw),
        }
    }

    fn device_op_entry_param(
        entry: &crate::circuit::DeviceOpEntry,
        parameter: &str,
    ) -> Option<Value> {
        entry
            .params
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(parameter))
            .map(|(_, value)| *value)
    }

    fn canonical_device_op_parameter(parameter: &str) -> Option<&'static str> {
        match parameter.trim().to_ascii_lowercase().as_str() {
            "id" | "ids" => Some("id"),
            "vgs" => Some("vgs"),
            "vds" => Some("vds"),
            "vbs" => Some("vbs"),
            "vth" | "vto" => Some("vth"),
            "vdsat" | "vdssat" => Some("vdsat"),
            "gm" => Some("gm"),
            "gds" => Some("gds"),
            "gmb" | "gmbs" => Some("gmb"),
            "cd" => Some("cd"),
            _ => None,
        }
    }

    fn netlist_has_device_op_instance(netlist: &Netlist, instance_name: &str) -> bool {
        if netlist.elements.iter().any(|element| {
            Self::netlist_element_exports_device_op(element)
                && Self::device_instance_names_match(&element.name, instance_name)
        }) {
            return true;
        }

        crate::netlist::flatten_netlist_with_models(netlist).is_ok_and(|flattened| {
            flattened.elements.iter().any(|element| {
                Self::netlist_element_exports_device_op(element)
                    && Self::device_instance_names_match(&element.name, instance_name)
            })
        })
    }

    fn netlist_supports_lead_current_probe(
        netlist: &Netlist,
        probe: &XyceLeadCurrentProbe,
    ) -> bool {
        match probe.terminal {
            XyceLeadCurrentTerminal::Drain => true,
            XyceLeadCurrentTerminal::Source => {
                Self::netlist_device_is_native_b3soi_mosfet(netlist, &probe.element_name)
            }
            XyceLeadCurrentTerminal::Gate
            | XyceLeadCurrentTerminal::Bulk
            | XyceLeadCurrentTerminal::Collector
            | XyceLeadCurrentTerminal::Emitter => false,
        }
    }

    fn dc_probe_is_omitted_empty_wildcard(probe: &str, netlist: &Netlist) -> bool {
        Self::parse_lead_current_probe(probe).is_some_and(|probe| {
            Self::lead_current_probe_is_omitted_empty_wildcard(netlist, &probe)
        })
    }

    fn lead_current_probe_is_omitted_empty_wildcard(
        netlist: &Netlist,
        probe: &XyceLeadCurrentProbe,
    ) -> bool {
        probe.element_name == "*"
            && !Self::netlist_has_lead_current_wildcard_match(netlist, probe.terminal)
    }

    fn netlist_has_lead_current_wildcard_match(
        netlist: &Netlist,
        terminal: XyceLeadCurrentTerminal,
    ) -> bool {
        if netlist
            .elements
            .iter()
            .any(|element| Self::element_matches_lead_current_wildcard(element, terminal))
        {
            return true;
        }

        crate::netlist::flatten_netlist_with_models(netlist).is_ok_and(|flattened| {
            flattened
                .elements
                .iter()
                .any(|element| Self::element_matches_lead_current_wildcard(element, terminal))
        })
    }

    fn element_matches_lead_current_wildcard(
        element: &crate::netlist::Element,
        terminal: XyceLeadCurrentTerminal,
    ) -> bool {
        match terminal {
            XyceLeadCurrentTerminal::Drain | XyceLeadCurrentTerminal::Gate => matches!(
                element.kind,
                ElementKind::Mosfet { .. } | ElementKind::Jfet { .. } | ElementKind::Mesfet { .. }
            ),
            XyceLeadCurrentTerminal::Source => matches!(
                element.kind,
                ElementKind::Mosfet { .. }
                    | ElementKind::Jfet { .. }
                    | ElementKind::Mesfet { .. }
                    | ElementKind::Bjt { .. }
            ),
            XyceLeadCurrentTerminal::Bulk => matches!(
                element.kind,
                ElementKind::Mosfet { .. } | ElementKind::Bjt { .. }
            ),
            XyceLeadCurrentTerminal::Collector | XyceLeadCurrentTerminal::Emitter => {
                matches!(element.kind, ElementKind::Bjt { .. })
            }
        }
    }

    fn netlist_device_is_native_b3soi_mosfet(netlist: &Netlist, instance_name: &str) -> bool {
        if Self::elements_device_is_native_b3soi_mosfet(
            &netlist.elements,
            &netlist.models,
            &[],
            instance_name,
        ) {
            return true;
        }

        crate::netlist::flatten_netlist_with_models(netlist).is_ok_and(|flattened| {
            Self::elements_device_is_native_b3soi_mosfet(
                &flattened.elements,
                &netlist.models,
                &flattened.scoped_models,
                instance_name,
            )
        })
    }

    fn netlist_device_is_native_classic_jfet(netlist: &Netlist, instance_name: &str) -> bool {
        if Self::elements_device_is_native_classic_jfet(
            &netlist.elements,
            &netlist.models,
            &[],
            instance_name,
        ) {
            return true;
        }

        crate::netlist::flatten_netlist_with_models(netlist).is_ok_and(|flattened| {
            Self::elements_device_is_native_classic_jfet(
                &flattened.elements,
                &netlist.models,
                &flattened.scoped_models,
                instance_name,
            )
        })
    }

    fn netlist_device_is_native_legacy_bjt(netlist: &Netlist, instance_name: &str) -> bool {
        if Self::elements_device_is_native_legacy_bjt(
            &netlist.elements,
            &netlist.models,
            &[],
            instance_name,
        ) {
            return true;
        }

        crate::netlist::flatten_netlist_with_models(netlist).is_ok_and(|flattened| {
            Self::elements_device_is_native_legacy_bjt(
                &flattened.elements,
                &netlist.models,
                &flattened.scoped_models,
                instance_name,
            )
        })
    }

    fn elements_device_is_native_legacy_bjt(
        elements: &[crate::netlist::Element],
        models: &[crate::netlist::ModelDef],
        scoped_models: &[crate::netlist::ModelDef],
        instance_name: &str,
    ) -> bool {
        elements.iter().any(|element| {
            if !Self::device_instance_names_match(&element.name, instance_name) {
                return false;
            }
            let ElementKind::Bjt { model, .. } = &element.kind else {
                return false;
            };
            Self::find_model(scoped_models, model)
                .or_else(|| Self::find_model(models, model))
                .is_some_and(Self::model_is_native_legacy_bjt)
        })
    }

    fn model_is_native_legacy_bjt(model: &crate::netlist::ModelDef) -> bool {
        if !matches!(
            model.model_type.to_ascii_uppercase().as_str(),
            "NPN" | "PNP" | "LPNP"
        ) {
            return false;
        }
        Self::numeric_param_value(&model.params, "LEVEL")
            .is_none_or(|level| level.is_finite() && (level - 1.0).abs() <= 1.0e-9)
    }

    fn model_is_native_legacy_diode(model: &crate::netlist::ModelDef) -> bool {
        if !matches!(
            model.model_type.to_ascii_uppercase().as_str(),
            "D" | "DIODE"
        ) {
            return false;
        }
        Self::numeric_param_value(&model.params, "LEVEL").is_none_or(|level| {
            level.is_finite() && ((level - 0.0).abs() <= 1.0e-9 || (level - 1.0).abs() <= 1.0e-9)
        })
    }

    fn netlist_device_is_single_native_ac_supported_bulk_mosfet(
        netlist: &Netlist,
        instance_name: &str,
    ) -> bool {
        let direct = Self::elements_native_ac_supported_bulk_mosfet_match_and_count(
            &netlist.elements,
            &netlist.models,
            &[],
            instance_name,
        );
        if direct == (true, 1) {
            return true;
        }

        crate::netlist::flatten_netlist_with_models(netlist).is_ok_and(|flattened| {
            Self::elements_native_ac_supported_bulk_mosfet_match_and_count(
                &flattened.elements,
                &netlist.models,
                &flattened.scoped_models,
                instance_name,
            ) == (true, 1)
        })
    }

    fn elements_native_ac_supported_bulk_mosfet_match_and_count(
        elements: &[crate::netlist::Element],
        models: &[crate::netlist::ModelDef],
        scoped_models: &[crate::netlist::ModelDef],
        instance_name: &str,
    ) -> (bool, usize) {
        let mut matched = false;
        let mut count = 0usize;
        for element in elements {
            let ElementKind::Mosfet { model, .. } = &element.kind else {
                continue;
            };
            let supported = Self::find_model(scoped_models, model)
                .or_else(|| Self::find_model(models, model))
                .is_some_and(Self::model_is_native_ac_supported_bulk_mosfet);
            if !supported {
                continue;
            }
            count += 1;
            matched |= Self::device_instance_names_match(&element.name, instance_name);
        }
        (matched, count)
    }

    fn model_is_native_ac_supported_bulk_mosfet(model: &crate::netlist::ModelDef) -> bool {
        if !matches!(
            model.model_type.to_ascii_uppercase().as_str(),
            "NMOS" | "PMOS"
        ) {
            return false;
        }
        Self::numeric_param_value(&model.params, "LEVEL").is_some_and(|level| {
            level.is_finite() && ((level - 2.0).abs() <= 1.0e-9 || (level - 6.0).abs() <= 1.0e-9)
        })
    }

    fn elements_device_is_native_classic_jfet(
        elements: &[crate::netlist::Element],
        models: &[crate::netlist::ModelDef],
        scoped_models: &[crate::netlist::ModelDef],
        instance_name: &str,
    ) -> bool {
        elements.iter().any(|element| {
            if !Self::device_instance_names_match(&element.name, instance_name) {
                return false;
            }
            let ElementKind::Jfet { model, .. } = &element.kind else {
                return false;
            };
            Self::find_model(scoped_models, model)
                .or_else(|| Self::find_model(models, model))
                .is_some_and(Self::model_is_native_classic_jfet)
        })
    }

    fn model_is_native_classic_jfet(model: &crate::netlist::ModelDef) -> bool {
        if !matches!(
            model.model_type.to_ascii_uppercase().as_str(),
            "NJF" | "PJF"
        ) {
            return false;
        }
        Self::numeric_param_value(&model.params, "LEVEL")
            .is_none_or(|level| level.is_finite() && (level - 1.0).abs() <= 1.0e-9)
    }

    fn elements_device_is_native_b3soi_mosfet(
        elements: &[crate::netlist::Element],
        models: &[crate::netlist::ModelDef],
        scoped_models: &[crate::netlist::ModelDef],
        instance_name: &str,
    ) -> bool {
        elements.iter().any(|element| {
            if !Self::device_instance_names_match(&element.name, instance_name) {
                return false;
            }
            let ElementKind::Mosfet {
                model,
                instance_params,
                ..
            } = &element.kind
            else {
                return false;
            };
            Self::find_model(scoped_models, model)
                .or_else(|| Self::find_model(models, model))
                .is_some_and(|model| Self::model_is_native_b3soi_mosfet(model, instance_params))
        })
    }

    fn model_is_native_b3soi_mosfet(
        model: &crate::netlist::ModelDef,
        instance_params: &[(String, Value)],
    ) -> bool {
        if !matches!(
            model.model_type.to_ascii_uppercase().as_str(),
            "NMOS" | "PMOS"
        ) {
            return false;
        }
        let Some(level) = Self::numeric_param_value(&model.params, "LEVEL") else {
            return false;
        };
        if (level - 10.0).abs() <= 1.0e-9 {
            return Self::numeric_param_value(instance_params, "SOIMOD")
                .or_else(|| Self::numeric_param_value(&model.params, "SOIMOD"))
                .is_none_or(|soi_mod| {
                    soi_mod.is_finite()
                        && (soi_mod - soi_mod.round()).abs() <= 1.0e-12
                        && matches!(soi_mod.round() as i32, 0..=3)
                });
        }
        [55.0, 56.0, 57.0]
            .iter()
            .any(|native_level| (level - native_level).abs() <= 1.0e-9)
    }

    fn numeric_param_value(params: &[(String, Value)], name: &str) -> Option<Value> {
        params
            .iter()
            .rev()
            .find(|(param_name, _)| param_name.eq_ignore_ascii_case(name))
            .map(|(_, value)| *value)
    }

    fn netlist_element_exports_device_op(element: &crate::netlist::Element) -> bool {
        matches!(
            element.kind,
            ElementKind::Diode { .. }
                | ElementKind::Bjt { .. }
                | ElementKind::Mosfet { .. }
                | ElementKind::Jfet { .. }
                | ElementKind::Mesfet { .. }
        )
    }

    fn device_instance_names_match(lhs: &str, rhs: &str) -> bool {
        Self::normalize_device_instance_name(lhs) == Self::normalize_device_instance_name(rhs)
    }

    fn normalize_device_instance_name(name: &str) -> String {
        Self::normalize_probe(name).replace(':', ".")
    }

    fn node_lookup_candidates(netlist: &Netlist, node_name: &str) -> Vec<String> {
        Engine::node_lookup_candidates(netlist, node_name)
    }

    fn evaluate_scalar_parameter_probe(
        netlist: &Netlist,
        dc: &XyceDcSweep,
        sweep_point: XyceDcSweepPoint,
        parameter_name: &str,
    ) -> Result<f64, String> {
        if parameter_name.eq_ignore_ascii_case("TEMP")
            || parameter_name.eq_ignore_ascii_case("TEMPER")
        {
            return Ok(Self::active_temperature_c(
                netlist,
                Some(dc),
                Some(sweep_point),
            ));
        }
        if parameter_name.eq_ignore_ascii_case("VT") {
            return Ok(Self::thermal_voltage_celsius(Self::active_temperature_c(
                netlist,
                Some(dc),
                Some(sweep_point),
            )));
        }

        netlist
            .params
            .get(parameter_name)
            .ok_or_else(|| format!("scalar parameter probe '{}' is not defined", parameter_name))
    }

    fn print_eval_context(
        netlist: &Netlist,
        dc: Option<&XyceDcSweep>,
        sweep_point: Option<XyceDcSweepPoint>,
    ) -> crate::netlist::ParamContext {
        let mut context = netlist.params.clone();
        let temp_c = Self::active_temperature_c(netlist, dc, sweep_point);
        context.set("TEMP", temp_c);
        context.set("TEMPER", temp_c);
        context.set("TNOM", netlist.options.tnom.unwrap_or(27.0));
        context.set("VT", Self::thermal_voltage_celsius(temp_c));
        context.set(
            "GMIN",
            netlist.options.gmin.unwrap_or(crate::constants::GMIN),
        );
        Self::add_resistor_parameter_bindings(netlist, &mut context);
        context
    }

    fn print_tran_eval_context(netlist: &Netlist, time: Value) -> crate::netlist::ParamContext {
        let mut context = Self::print_eval_context(netlist, None, None);
        context.set("TIME", time);
        context
    }

    fn add_resistor_parameter_bindings(
        netlist: &Netlist,
        context: &mut crate::netlist::ParamContext,
    ) {
        for element in &netlist.elements {
            let ElementKind::Resistor {
                value,
                value_expr,
                model,
                instance_params,
                ..
            } = &element.kind
            else {
                continue;
            };
            if netlist.params.get(&element.name).is_some() {
                continue;
            }

            let Some(resistance) = Self::resistor_parameter_r_value_from_parts(
                *value,
                value_expr.as_deref(),
                model.as_deref(),
                instance_params,
                context,
            ) else {
                continue;
            };
            context.set(&element.name, resistance);
        }
    }

    fn active_temperature_c(
        netlist: &Netlist,
        dc: Option<&XyceDcSweep>,
        sweep_point: Option<XyceDcSweepPoint>,
    ) -> Value {
        if let (Some(dc), Some(sweep_point)) = (dc, sweep_point) {
            if Self::is_temperature_name(&dc.source) {
                return sweep_point.primary;
            }
            if let Some(sweep2) = &dc.sweep2
                && Self::is_temperature_name(&sweep2.source)
                && let Some(secondary) = sweep_point.secondary
            {
                return secondary;
            }
        }

        Self::netlist_temperature_c(netlist)
    }

    fn netlist_temperature_c(netlist: &Netlist) -> Value {
        netlist
            .options
            .temp
            .or_else(|| netlist.params.get("TEMP"))
            .or_else(|| netlist.params.get("TEMPER"))
            .unwrap_or(27.0)
    }

    fn thermal_voltage_celsius(temp_c: Value) -> Value {
        crate::constants::thermal_voltage(crate::analysis::temperature::celsius_to_kelvin(temp_c))
    }

    fn is_temperature_name(name: &str) -> bool {
        name.eq_ignore_ascii_case("TEMP") || name.eq_ignore_ascii_case("TEMPER")
    }

    fn evaluate_resistor_current(
        netlist: &Netlist,
        result: &crate::SimulationResult,
        resistor_name: &str,
        resistance: Value,
    ) -> Result<f64, String> {
        if Self::resistor_uses_branch_form(netlist, resistance) {
            return Err(format!(
                "missing solved branch current for zero/near-zero resistor '{}'",
                resistor_name
            ));
        }
        if resistance.is_infinite() && resistance.is_sign_positive() {
            return Ok(0.0);
        }
        Ok(Self::resistor_voltage_drop(netlist, result, resistor_name)? / resistance)
    }

    fn evaluate_resistor_power(
        netlist: &Netlist,
        result: &crate::SimulationResult,
        resistor_name: &str,
        resistance: Value,
    ) -> Result<f64, String> {
        let voltage_drop = Self::resistor_voltage_drop(netlist, result, resistor_name)?;
        let current =
            if let Some(current) = Self::result_branch_current_named(result, resistor_name) {
                current
            } else {
                Self::evaluate_resistor_current(netlist, result, resistor_name, resistance)?
            };
        Ok(voltage_drop * current)
    }

    fn resistor_voltage_drop(
        netlist: &Netlist,
        result: &crate::SimulationResult,
        resistor_name: &str,
    ) -> Result<Value, String> {
        let element = Self::find_resistor_element(netlist, resistor_name)
            .ok_or_else(|| format!("resistor '{}' not found", resistor_name))?;
        let node_pos = element
            .nodes
            .first()
            .ok_or_else(|| format!("resistor '{}' has no positive node", resistor_name))?;
        let node_neg = element
            .nodes
            .get(1)
            .ok_or_else(|| format!("resistor '{}' has no negative node", resistor_name))?;
        let v_pos = Self::result_voltage_named(result, netlist, node_pos)
            .ok_or_else(|| format!("node '{}' not found in DC result", node_pos))?;
        let v_neg = Self::result_voltage_named(result, netlist, node_neg)
            .ok_or_else(|| format!("node '{}' not found in DC result", node_neg))?;
        Ok(v_pos - v_neg)
    }

    fn result_voltage_named(
        result: &crate::SimulationResult,
        netlist: &Netlist,
        node_name: &str,
    ) -> Option<Value> {
        Self::node_lookup_candidates(netlist, node_name)
            .into_iter()
            .find_map(|candidate| result.try_voltage_named(&candidate))
    }

    fn result_branch_current_named(
        result: &crate::SimulationResult,
        branch_name: &str,
    ) -> Option<Value> {
        result.branch_current_named(branch_name).or_else(|| {
            let normalized = Self::normalize_device_instance_name(branch_name);
            (normalized != branch_name).then(|| result.branch_current_named(&normalized))?
        })
    }

    fn ac_node_voltage_named(
        result: &AcResult,
        netlist: &Netlist,
        node_name: &str,
    ) -> Option<Complex64> {
        if matches!(
            node_name.to_ascii_lowercase().as_str(),
            "0" | "gnd" | "ground"
        ) {
            return Some(Complex64::new(0.0, 0.0));
        }
        Self::node_lookup_candidates(netlist, node_name)
            .into_iter()
            .find_map(|candidate| {
                result
                    .node_names
                    .iter()
                    .position(|name| name.eq_ignore_ascii_case(&candidate))
                    .and_then(|index| result.voltages.get(index).copied())
            })
    }

    fn ac_branch_current_named(result: &AcResult, branch_name: &str) -> Option<Complex64> {
        result
            .branch_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case(branch_name))
            .and_then(|index| result.currents.get(index).copied())
            .or_else(|| {
                let normalized = Self::normalize_device_instance_name(branch_name);
                (normalized != branch_name).then(|| {
                    result
                        .branch_names
                        .iter()
                        .position(|name| name.eq_ignore_ascii_case(&normalized))
                        .and_then(|index| result.currents.get(index).copied())
                })?
            })
    }

    fn netlist_has_recorded_branch_current(netlist: &Netlist, source: &str) -> bool {
        if Self::elements_have_recorded_branch_current(&netlist.elements, source) {
            return true;
        }

        crate::netlist::flatten_netlist_with_models(netlist).is_ok_and(|flattened| {
            Self::elements_have_recorded_branch_current(&flattened.elements, source)
        })
    }

    fn elements_have_recorded_branch_current(
        elements: &[crate::netlist::Element],
        source: &str,
    ) -> bool {
        elements.iter().any(|element| {
            Self::device_instance_names_match(&element.name, source)
                && Self::element_has_recorded_branch_current(&element.kind)
        })
    }

    fn element_has_recorded_branch_current(kind: &ElementKind) -> bool {
        matches!(
            kind,
            ElementKind::VoltageSource(_)
                | ElementKind::Resistor { .. }
                | ElementKind::Capacitor { .. }
                | ElementKind::Inductor { .. }
                | ElementKind::JilesAthertonInductor { .. }
                | ElementKind::Vcvs { .. }
                | ElementKind::Ccvs { .. }
                | ElementKind::VSwitch { .. }
                | ElementKind::ISwitch { .. }
                | ElementKind::GenericSwitch { .. }
                | ElementKind::BehavioralVoltage { .. }
                | ElementKind::BehavioralCurrent { .. }
        )
    }

    fn netlist_has_independent_current_source(netlist: &Netlist, source: &str) -> bool {
        if Self::elements_have_independent_current_source(&netlist.elements, source) {
            return true;
        }

        crate::netlist::flatten_netlist_with_models(netlist).is_ok_and(|flattened| {
            Self::elements_have_independent_current_source(&flattened.elements, source)
        })
    }

    fn elements_have_independent_current_source(
        elements: &[crate::netlist::Element],
        source: &str,
    ) -> bool {
        elements.iter().any(|element| {
            Self::device_instance_names_match(&element.name, source)
                && matches!(&element.kind, ElementKind::CurrentSource(_))
        })
    }

    fn evaluate_independent_current_source_probe(
        netlist: &Netlist,
        result: &TransientResult,
        source: &str,
        time: Value,
    ) -> Option<Value> {
        if Self::tran_uses_uic(netlist)
            && Self::time_is_transient_initial_sample(result, time)
            && Self::netlist_has_independent_current_source(netlist, source)
        {
            return Some(0.0);
        }
        if let Some(value) = Self::evaluate_current_source_probe_from_elements(
            &netlist.elements,
            result,
            source,
            time,
        ) {
            return Some(value);
        }

        let flattened = crate::netlist::flatten_netlist_with_models(netlist).ok()?;
        Self::evaluate_current_source_probe_from_elements(&flattened.elements, result, source, time)
    }

    fn tran_uses_uic(netlist: &Netlist) -> bool {
        netlist.analyses.iter().any(|analysis| {
            matches!(
                analysis,
                crate::netlist::AnalysisCommand::Tran { uic: true, .. }
            )
        })
    }

    fn time_is_transient_initial_sample(result: &TransientResult, time: Value) -> bool {
        let Some(first) = result.time.first().copied() else {
            return false;
        };
        (time - first).abs() <= 1.0e-30
    }

    fn evaluate_current_source_probe_from_elements(
        elements: &[crate::netlist::Element],
        result: &TransientResult,
        source: &str,
        time: Value,
    ) -> Option<Value> {
        let spec = elements.iter().find_map(|element| {
            if Self::device_instance_names_match(&element.name, source)
                && let ElementKind::CurrentSource(spec) = &element.kind
            {
                return Some(spec);
            }
            None
        })?;
        let (tstep, tstop) = Self::transient_result_source_context(result);
        Some(
            crate::circuit::VoltageSources::evaluate_source_spec_at_time_with_dialect(
                spec,
                time,
                tstep,
                tstop,
                SpiceDialect::Xyce,
            ),
        )
    }

    fn transient_result_source_context(result: &TransientResult) -> (Value, Value) {
        let tstop = result.time.last().copied().unwrap_or(1e99).max(1e-18);
        let mut previous: Option<Value> = None;
        let mut min_step: Option<Value> = None;
        for &sample in &result.time {
            if let Some(previous_sample) = previous {
                let step = sample - previous_sample;
                if step.is_finite() && step > 0.0 {
                    min_step = Some(min_step.map_or(step, |current| current.min(step)));
                }
            }
            previous = Some(sample);
        }
        (min_step.unwrap_or(1e-12), tstop)
    }

    fn source_is_voltage_source(netlist: &Netlist, source: &str) -> bool {
        netlist.elements.iter().any(|element| {
            element.name.eq_ignore_ascii_case(source)
                && matches!(&element.kind, ElementKind::VoltageSource(_))
        })
    }

    fn source_is_current_source(netlist: &Netlist, source: &str) -> bool {
        netlist.elements.iter().any(|element| {
            element.name.eq_ignore_ascii_case(source)
                && matches!(&element.kind, ElementKind::CurrentSource(_))
        })
    }

    fn source_is_independent_source(netlist: &Netlist, source: &str) -> bool {
        netlist.elements.iter().any(|element| {
            element.name.eq_ignore_ascii_case(source)
                && matches!(
                    &element.kind,
                    ElementKind::VoltageSource(_) | ElementKind::CurrentSource(_)
                )
        })
    }

    fn independent_source_dc_value(netlist: &Netlist, source: &str) -> Option<Value> {
        netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case(source))
            .and_then(|element| match &element.kind {
                ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => {
                    Some(extract_dc_value(spec))
                }
                _ => None,
            })
    }

    fn independent_source_ac_terms(netlist: &Netlist, source: &str) -> Option<(Value, Value)> {
        netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case(source))
            .and_then(|element| match &element.kind {
                ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => {
                    Some(extract_ac_value(spec))
                }
                _ => None,
            })
    }

    fn evaluate_current_source_current(
        netlist: &Netlist,
        dc: &XyceDcSweep,
        sweep_point: XyceDcSweepPoint,
        source: &str,
    ) -> Option<Value> {
        let element = netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case(source))?;
        let ElementKind::CurrentSource(spec) = &element.kind else {
            return None;
        };

        if source.eq_ignore_ascii_case(&dc.source) {
            return Some(sweep_point.primary);
        }
        if let Some(sweep2) = &dc.sweep2
            && source.eq_ignore_ascii_case(&sweep2.source)
        {
            return sweep_point.secondary;
        }

        Some(extract_dc_value(spec))
    }

    fn scalar_parameter_probe_is_supported(netlist: &Netlist, parameter_name: &str) -> bool {
        parameter_name.eq_ignore_ascii_case("TEMP")
            || parameter_name.eq_ignore_ascii_case("TEMPER")
            || parameter_name.eq_ignore_ascii_case("VT")
            || netlist.params.get(parameter_name).is_some()
    }

    fn scalar_parameter_sweep_source_is_supported(netlist: &Netlist, parameter_name: &str) -> bool {
        netlist
            .params
            .all_params()
            .iter()
            .any(|(name, _)| name.eq_ignore_ascii_case(parameter_name))
    }

    fn effective_resistor_value(netlist: &Netlist, name: &str) -> Option<Value> {
        Engine::new(SimulationConfig {
            spice_dialect: SpiceDialect::Xyce,
            ..SimulationConfig::default()
        })
        .resolved_resistor_value(netlist, name)
        .ok()
        .flatten()
    }

    fn effective_capacitor_value(netlist: &Netlist, name: &str) -> Option<Value> {
        Engine::new(SimulationConfig {
            spice_dialect: SpiceDialect::Xyce,
            ..SimulationConfig::default()
        })
        .resolved_capacitor_value(netlist, name)
        .ok()
        .flatten()
    }

    fn effective_inductor_value(netlist: &Netlist, name: &str) -> Option<Value> {
        Engine::new(SimulationConfig {
            spice_dialect: SpiceDialect::Xyce,
            ..SimulationConfig::default()
        })
        .resolved_inductor_value(netlist, name)
        .ok()
        .flatten()
    }

    fn inductor_instance_params_affect_effective_value(
        instance_params: &[(String, Value)],
    ) -> bool {
        const EFFECTIVE_VALUE_PARAMS: &[&str] = &[
            "L",
            "IND",
            "VALUE",
            "INDUCTANCE",
            "M",
            "MULT",
            "SCALE",
            "TEMP",
            "DTEMP",
            "TC1",
            "TC2",
        ];
        instance_params.iter().any(|(name, _)| {
            EFFECTIVE_VALUE_PARAMS
                .iter()
                .any(|candidate| name.eq_ignore_ascii_case(candidate))
        })
    }

    fn capacitor_instance_params_affect_effective_value(
        instance_params: &[(String, Value)],
    ) -> bool {
        const EFFECTIVE_VALUE_PARAMS: &[&str] = &["L", "W", "M", "TEMP", "DTEMP", "TC1", "TC2"];
        instance_params.iter().any(|(name, _)| {
            EFFECTIVE_VALUE_PARAMS
                .iter()
                .any(|candidate| name.eq_ignore_ascii_case(candidate))
        })
    }

    fn resistor_parameter_r_value(netlist: &Netlist, name: &str) -> Option<Value> {
        let element = Self::find_resistor_element(netlist, name)?;
        let ElementKind::Resistor {
            value,
            value_expr,
            model,
            instance_params,
            ..
        } = &element.kind
        else {
            return None;
        };

        if Self::resistor_uses_xyce_default_marker(instance_params)
            && let Some(resistance) = Self::effective_resistor_value(netlist, name)
        {
            return Some(resistance);
        }
        if let Some(resistance) = Self::instance_param(instance_params, &["R", "VALUE"]) {
            return Some(resistance);
        }
        Self::resistor_parameter_r_value_from_parts(
            *value,
            value_expr.as_deref(),
            model.as_deref(),
            instance_params,
            &Self::print_eval_context(netlist, None, None),
        )
    }

    fn evaluate_resistor_parameter_r_value(
        netlist: &Netlist,
        dc: &XyceDcSweep,
        sweep_point: XyceDcSweepPoint,
        result: &crate::SimulationResult,
        op_report: &crate::circuit::DeviceOpReport,
        name: &str,
    ) -> Result<Value, String> {
        let element = Self::find_resistor_element(netlist, name).ok_or_else(|| {
            format!("resistor parameter probe '{name}:R' targets an unknown resistor")
        })?;
        let ElementKind::Resistor {
            value,
            value_expr,
            model,
            instance_params,
            ..
        } = &element.kind
        else {
            return Err(format!(
                "resistor parameter probe '{name}:R' targets a non-resistor element"
            ));
        };

        if Self::resistor_uses_xyce_default_marker(instance_params)
            && let Some(resistance) = Self::effective_resistor_value(netlist, name)
        {
            return Ok(resistance);
        }
        if let Some(resistance) = Self::instance_param(instance_params, &["R", "VALUE"]) {
            return Ok(resistance);
        }
        if value.is_finite() {
            return Ok(*value);
        }
        if let Some(expression) = value_expr.as_deref() {
            let context = Self::print_eval_context(netlist, Some(dc), Some(sweep_point));
            let mut call_value = |call: &str| {
                let normalized = Self::normalize_probe(call);
                Self::evaluate_atomic_dc_probe(
                    &normalized,
                    netlist,
                    dc,
                    sweep_point,
                    result,
                    op_report,
                )
            };
            return Self::evaluate_print_expression_with_probe_calls(
                expression,
                context,
                &mut call_value,
            )
            .map_err(|err| {
                format!("failed to evaluate resistor parameter probe '{name}:R': {err}")
            });
        }
        if model.is_some()
            && let Some(resistance) = Self::resistor_parameter_r_value(netlist, name)
        {
            return Ok(resistance);
        }

        Err(format!(
            "resistor parameter probe '{name}:R' could not resolve a resistance value"
        ))
    }

    fn resistor_uses_xyce_default_marker(instance_params: &[(String, Value)]) -> bool {
        Self::instance_param(
            instance_params,
            &[crate::netlist::XYCE_DEFAULT_RESISTOR_VALUE_MARKER],
        )
        .is_some()
    }

    fn resistor_parameter_r_value_from_parts(
        value: Value,
        value_expr: Option<&str>,
        model: Option<&str>,
        instance_params: &[(String, Value)],
        context: &crate::netlist::ParamContext,
    ) -> Option<Value> {
        if let Some(resistance) = Self::instance_param(instance_params, &["R", "VALUE"]) {
            return Some(resistance);
        }
        if value.is_finite() {
            return Some(value);
        }
        if let Some(expression) = value_expr {
            return crate::netlist::expr::eval_expression(expression, context).ok();
        }
        if model.is_some() {
            return Some(1000.0);
        }
        None
    }

    fn resistor_temperature_value(netlist: &Netlist, name: &str) -> Option<Value> {
        let element = Self::find_resistor_element(netlist, name)?;
        let ElementKind::Resistor {
            instance_params, ..
        } = &element.kind
        else {
            return None;
        };

        if let Some(temp) = Self::instance_param(instance_params, &["TEMP"]) {
            return Some(Self::normalize_temperature_param_to_celsius(temp));
        }
        if let Some(dtemp) = Self::instance_param(instance_params, &["DTEMP"]) {
            return Some(Self::netlist_temperature_c(netlist) + dtemp);
        }
        Some(Self::netlist_temperature_c(netlist))
    }

    fn resistor_instance_parameter_probe_is_supported(
        netlist: &Netlist,
        name: &str,
        parameter: &str,
    ) -> bool {
        Self::resistor_instance_parameter_value(netlist, name, parameter).is_some()
    }

    fn evaluate_resistor_instance_parameter_probe(
        netlist: &Netlist,
        name: &str,
        parameter: &str,
    ) -> Result<Value, String> {
        Self::resistor_instance_parameter_value(netlist, name, parameter).ok_or_else(|| {
            format!(
                "resistor parameter probe '{}:{}' targets an unknown or unset resistor instance parameter",
                name, parameter
            )
        })
    }

    fn resistor_instance_parameter_value(
        netlist: &Netlist,
        name: &str,
        parameter: &str,
    ) -> Option<Value> {
        let element = Self::find_resistor_element(netlist, name)?;
        let ElementKind::Resistor {
            instance_params, ..
        } = &element.kind
        else {
            return None;
        };

        if parameter.eq_ignore_ascii_case("M") {
            return Some(Self::instance_param(instance_params, &[parameter]).unwrap_or(1.0));
        }
        Self::instance_param(instance_params, &[parameter])
    }

    fn find_recorded_two_terminal_branch_element(
        netlist: &Netlist,
        name: &str,
    ) -> Option<crate::netlist::Element> {
        if let Some(element) = netlist.elements.iter().find(|element| {
            Self::device_instance_names_match(&element.name, name)
                && element.nodes.len() >= 2
                && Self::element_has_recorded_branch_current(&element.kind)
        }) {
            return Some(element.clone());
        }

        crate::netlist::flatten_netlist_with_models(netlist)
            .ok()?
            .elements
            .into_iter()
            .find(|element| {
                Self::device_instance_names_match(&element.name, name)
                    && element.nodes.len() >= 2
                    && Self::element_has_recorded_branch_current(&element.kind)
            })
    }

    fn find_resistor_element(netlist: &Netlist, name: &str) -> Option<crate::netlist::Element> {
        if let Some(element) = netlist.elements.iter().find(|element| {
            Self::device_instance_names_match(&element.name, name)
                && matches!(&element.kind, ElementKind::Resistor { .. })
        }) {
            return Some(element.clone());
        }

        crate::netlist::flatten_netlist_with_models(netlist)
            .ok()?
            .elements
            .into_iter()
            .find(|element| {
                Self::device_instance_names_match(&element.name, name)
                    && matches!(&element.kind, ElementKind::Resistor { .. })
            })
    }

    fn find_capacitor_element(netlist: &Netlist, name: &str) -> Option<crate::netlist::Element> {
        if let Some(element) = netlist.elements.iter().find(|element| {
            Self::device_instance_names_match(&element.name, name)
                && matches!(&element.kind, ElementKind::Capacitor { .. })
        }) {
            return Some(element.clone());
        }

        crate::netlist::flatten_netlist_with_models(netlist)
            .ok()?
            .elements
            .into_iter()
            .find(|element| {
                Self::device_instance_names_match(&element.name, name)
                    && matches!(&element.kind, ElementKind::Capacitor { .. })
            })
    }

    fn find_inductor_element(netlist: &Netlist, name: &str) -> Option<crate::netlist::Element> {
        if let Some(element) = netlist.elements.iter().find(|element| {
            Self::device_instance_names_match(&element.name, name)
                && matches!(&element.kind, ElementKind::Inductor { .. })
        }) {
            return Some(element.clone());
        }

        crate::netlist::flatten_netlist_with_models(netlist)
            .ok()?
            .elements
            .into_iter()
            .find(|element| {
                Self::device_instance_names_match(&element.name, name)
                    && matches!(&element.kind, ElementKind::Inductor { .. })
            })
    }

    fn find_bare_device_parameter_element(
        netlist: &Netlist,
        probe: &str,
    ) -> Option<crate::netlist::Element> {
        let probe_name = Self::parse_bare_device_parameter_probe(probe)?;
        if let Some(element) = netlist.elements.iter().find(|element| {
            Self::device_instance_names_match(&element.name, &probe_name)
                && Self::element_has_bare_device_parameter(&element.kind)
        }) {
            return Some(element.clone());
        }

        crate::netlist::flatten_netlist_with_models(netlist)
            .ok()?
            .elements
            .into_iter()
            .find(|element| {
                Self::device_instance_names_match(&element.name, &probe_name)
                    && Self::element_has_bare_device_parameter(&element.kind)
            })
    }

    fn element_has_bare_device_parameter(kind: &ElementKind) -> bool {
        matches!(
            kind,
            ElementKind::VoltageSource(_)
                | ElementKind::CurrentSource(_)
                | ElementKind::Resistor { .. }
                | ElementKind::Capacitor { .. }
                | ElementKind::Inductor { .. }
        )
    }

    fn instance_param(params: &[(String, Value)], names: &[&str]) -> Option<Value> {
        names.iter().find_map(|candidate| {
            params
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case(candidate))
                .map(|(_, value)| *value)
        })
    }

    fn normalize_temperature_param_to_celsius(value: Value) -> Value {
        value
    }

    fn step_commands(netlist: &Netlist) -> Result<Vec<StepCommand>, String> {
        let step_commands = netlist
            .analyses
            .iter()
            .filter_map(|analysis| match analysis {
                AnalysisCommand::Step(step) => Some(step),
                _ => None,
            })
            .cloned()
            .collect::<Vec<_>>();

        for step in &step_commands {
            match &step.sweep {
                StepSweep::Data { table_name } => {
                    let table = netlist
                        .data_tables
                        .iter()
                        .find(|table| table.name.eq_ignore_ascii_case(table_name))
                        .ok_or_else(|| format!(".STEP DATA table '{table_name}' not found"))?;
                    if table.params.is_empty() {
                        return Err(format!(".STEP DATA table '{}' has no columns", table.name));
                    }
                    if table.rows.is_empty() {
                        return Err(format!(".STEP DATA table '{}' has no rows", table.name));
                    }
                    for (row_index, row) in table.rows.iter().enumerate() {
                        if row.len() != table.params.len() {
                            return Err(format!(
                                ".STEP DATA table '{}' row {} has {} value(s), expected {}",
                                table.name,
                                row_index,
                                row.len(),
                                table.params.len()
                            ));
                        }
                    }
                }
                _ if step.sweep.values().is_empty() => {
                    return Err("deck has invalid .STEP sweep bounds".to_string());
                }
                _ => {}
            }
        }

        Ok(step_commands)
    }

    fn single_tran_analysis(netlist: &Netlist) -> Result<XyceTranAnalysis, String> {
        let analyses = netlist
            .analyses
            .iter()
            .filter_map(|analysis| match analysis {
                AnalysisCommand::Tran {
                    step,
                    stop,
                    start,
                    max_step,
                    uic,
                } => Some(XyceTranAnalysis {
                    step: *step,
                    stop: *stop,
                    start: *start,
                    max_step: *max_step,
                    uic: *uic,
                }),
                _ => None,
            })
            .collect::<Vec<_>>();

        match analyses.len() {
            0 => Err("deck has no .TRAN analysis for static .PRINT TRAN output".to_string()),
            1 => Ok(analyses[0]),
            _ => Err(
                "deck has multiple .TRAN analyses; multi-analysis transient comparison is not implemented yet"
                    .to_string(),
            ),
        }
    }

    fn single_ac_analysis(netlist: &Netlist) -> Result<XyceAcAnalysis, String> {
        let mut analyses = Vec::new();
        for analysis in &netlist.analyses {
            match analysis {
                AnalysisCommand::Ac {
                    variation,
                    points,
                    start_freq,
                    stop_freq,
                } => analyses.push(XyceAcAnalysis {
                    frequencies: ac_sweep_frequencies(*variation, *points, *start_freq, *stop_freq),
                    data_points: None,
                }),
                AnalysisCommand::AcData { table_name } => {
                    let data_points = Self::ac_data_table_points(netlist, table_name)?;
                    analyses.push(XyceAcAnalysis {
                        frequencies: data_points
                            .iter()
                            .map(|point| point.frequency)
                            .collect::<Vec<_>>(),
                        data_points: Some(data_points),
                    });
                }
                _ => {}
            }
        }

        match analyses.len() {
            0 => Err("deck has no .AC analysis for static .PRINT AC output".to_string()),
            1 => Ok(analyses[0].clone()),
            _ => Err(
                "deck has multiple .AC analyses; multi-analysis AC comparison is not implemented yet"
                    .to_string(),
            ),
        }
    }

    fn ac_data_table_points(
        netlist: &Netlist,
        table_name: &str,
    ) -> Result<Vec<XyceAcDataPoint>, String> {
        let table = netlist
            .data_tables
            .iter()
            .find(|table| table.name.eq_ignore_ascii_case(table_name))
            .ok_or_else(|| format!(".AC DATA references unknown .DATA table '{table_name}'"))?;
        if table.params.is_empty() {
            return Err(format!(".AC DATA table '{}' has no columns", table.name));
        }
        if table.rows.is_empty() {
            return Err(format!(".AC DATA table '{}' has no rows", table.name));
        }
        let mut unique_params = BTreeSet::new();
        for param in &table.params {
            if !unique_params.insert(param.to_ascii_uppercase()) {
                return Err(format!(
                    ".AC DATA table '{}' has duplicate column '{}'",
                    table.name, param
                ));
            }
        }
        let freq_column = table
            .params
            .iter()
            .position(|param| param.eq_ignore_ascii_case("FREQ"))
            .ok_or_else(|| format!(".AC DATA table '{}' has no FREQ column", table.name))?;
        let points = table
            .rows
            .iter()
            .enumerate()
            .map(|(row_index, row)| {
                if row.len() != table.params.len() {
                    return Err(format!(
                        ".AC DATA table '{}' row {} has {} value(s), expected {}",
                        table.name,
                        row_index + 1,
                        row.len(),
                        table.params.len()
                    ));
                }
                if let Some((column_index, value)) =
                    row.iter().enumerate().find(|(_, value)| !value.is_finite())
                {
                    return Err(format!(
                        ".AC DATA table '{}' row {} column '{}' must be finite, got {}",
                        table.name,
                        row_index + 1,
                        table.params[column_index],
                        value
                    ));
                }
                let frequency = row.get(freq_column).copied().ok_or_else(|| {
                    format!(
                        ".AC DATA table '{}' row {} does not contain FREQ column {}",
                        table.name,
                        row_index + 1,
                        freq_column + 1
                    )
                })?;
                let overrides = table
                    .params
                    .iter()
                    .cloned()
                    .zip(row.iter().copied())
                    .collect::<Vec<_>>();
                Ok(XyceAcDataPoint {
                    frequency,
                    overrides,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(points)
    }

    fn single_dc_sweep(netlist: &Netlist) -> Result<XyceDcSweep, String> {
        let mut dimensions = Vec::new();
        for analysis in &netlist.analyses {
            let AnalysisCommand::Dc {
                source,
                start,
                stop,
                step,
                mode,
                sweep2,
            } = analysis
            else {
                continue;
            };

            dimensions.push(XyceDcSweepDimension {
                source: source.clone(),
                start: *start,
                stop: *stop,
                step: *step,
                mode: mode.clone(),
            });
            if let Some(sweep2) = sweep2 {
                dimensions.push(XyceDcSweepDimension {
                    source: sweep2.source.clone(),
                    start: sweep2.start,
                    stop: sweep2.stop,
                    step: sweep2.step,
                    mode: sweep2.mode.clone(),
                });
            }
        }

        if dimensions.is_empty() {
            if netlist
                .analyses
                .iter()
                .any(|analysis| matches!(analysis, AnalysisCommand::Op))
            {
                return Self::synthetic_op_dc_sweep(netlist);
            }
            return Err("deck has no .DC or .OP analysis for static .PRINT DC output".to_string());
        }
        if dimensions.len() > 2 {
            return Err(format!(
                "deck has {} .DC sweep dimensions; native Xyce static adapter currently supports one or two",
                dimensions.len()
            ));
        }

        for (index, dimension) in dimensions.iter().enumerate() {
            if dimension.spec().points().is_empty() {
                if index == 0 {
                    return Err("deck has invalid .DC sweep bounds".to_string());
                }
                return Err("deck has invalid secondary .DC sweep bounds".to_string());
            }
        }

        let primary = dimensions.remove(0);
        let sweep2 = dimensions
            .pop()
            .map(XyceDcSweepDimension::into_second_sweep);
        Ok(XyceDcSweep {
            source: primary.source,
            start: primary.start,
            stop: primary.stop,
            step: primary.step,
            mode: primary.mode,
            sweep2,
        })
    }

    fn synthetic_op_dc_sweep(netlist: &Netlist) -> Result<XyceDcSweep, String> {
        for element in &netlist.elements {
            let dc_value = match &element.kind {
                ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => {
                    extract_dc_value(spec)
                }
                _ => continue,
            };
            if !dc_value.is_finite() {
                return Err(format!(
                    ".OP source '{}' has non-finite DC value {}",
                    element.name, dc_value
                ));
            }
            return Ok(XyceDcSweep {
                source: element.name.clone(),
                start: dc_value,
                stop: dc_value,
                step: 1.0,
                mode: crate::netlist::DcSweepMode::Linear,
                sweep2: None,
            });
        }

        Err(".OP static .PRINT DC output requires at least one independent source for the native one-point adapter".to_string())
    }

    fn single_dc_print_request(source: &str) -> Result<XycePrintRequest, String> {
        let request = Self::single_dc_print_output_request(source)?;
        Ok(XycePrintRequest {
            probes: request.probes,
        })
    }

    fn single_dc_print_output_request(source: &str) -> Result<XycePrintOutputRequest, String> {
        let requests = Self::print_output_requests(source, "DC")?
            .into_iter()
            .filter(|request| request.file.is_none())
            .collect::<Vec<_>>();

        match requests.len() {
            0 => Err("deck has no .PRINT DC statement with static columns".to_string()),
            1 => Ok(requests.into_iter().next().expect("one request")),
            _ => Err("deck has multiple .PRINT DC statements; multi-table comparison is not implemented yet".to_string()),
        }
    }

    fn single_dc_or_file_output_request(source: &str) -> Result<XycePrintOutputRequest, String> {
        match Self::single_dc_print_output_request(source) {
            Ok(request) => Ok(request),
            Err(primary_error) => {
                let mut side_outputs = Self::prn_compatible_side_output_requests(source)?
                    .into_iter()
                    .collect::<Vec<_>>();
                if side_outputs.is_empty() {
                    return Err(primary_error);
                }
                side_outputs.sort_by(|left, right| left.file.cmp(&right.file));
                Ok(side_outputs.remove(0))
            }
        }
    }

    fn single_tran_print_output_request(source: &str) -> Result<XycePrintOutputRequest, String> {
        let requests = Self::print_output_requests(source, "TRAN")?
            .into_iter()
            .filter(|request| request.file.is_none())
            .collect::<Vec<_>>();

        match requests.len() {
            0 => Err("deck has no .PRINT TRAN statement with static columns".to_string()),
            1 => Ok(requests.into_iter().next().expect("one request")),
            _ => Err("deck has multiple .PRINT TRAN statements; multi-table comparison is not implemented yet".to_string()),
        }
    }

    #[cfg(test)]
    fn single_ac_print_request(source: &str) -> Result<XycePrintRequest, String> {
        let request = Self::single_ac_print_output_request(source)?;
        Ok(XycePrintRequest {
            probes: request.probes,
        })
    }

    #[cfg(test)]
    fn single_ac_print_output_request(source: &str) -> Result<XycePrintOutputRequest, String> {
        match Self::primary_print_output_request(source, "AC")? {
            Some(request) => Ok(request),
            None => Err("deck has no .PRINT AC statement with static columns".to_string()),
        }
    }

    #[cfg(test)]
    fn primary_print_output_request(
        source: &str,
        analysis: &str,
    ) -> Result<Option<XycePrintOutputRequest>, String> {
        Self::canonical_print_output_request(source, analysis, false)
    }

    fn canonical_print_output_request(
        source: &str,
        analysis: &str,
        allow_single_file_output: bool,
    ) -> Result<Option<XycePrintOutputRequest>, String> {
        let requests = Self::aggregate_print_output_requests(
            Self::print_output_requests(source, analysis)?,
            analysis,
        )?;
        let primary_requests = requests
            .iter()
            .filter(|request| request.file.is_none())
            .cloned()
            .collect::<Vec<_>>();

        match primary_requests.len() {
            0 => Ok(None),
            1 => Ok(Some(
                primary_requests.into_iter().next().expect("one request"),
            )),
            _ => Err(format!(
                "deck has multiple primary .PRINT {analysis} output destinations after aggregation"
            )),
        }
        .and_then(|primary| {
            if primary.is_some() || !allow_single_file_output {
                return Ok(primary);
            }
            let file_requests = requests
                .into_iter()
                .filter(|request| request.file.is_some())
                .collect::<Vec<_>>();
            match file_requests.len() {
                0 => Ok(None),
                1 => Ok(file_requests.into_iter().next()),
                _ => Err(format!(
                    "deck has multiple .PRINT {analysis} FILE= outputs and no primary output"
                )),
            }
        })
    }

    fn output_override_print_output_request(
        source: &str,
        analysis: &str,
    ) -> Result<Option<XycePrintOutputRequest>, String> {
        let mut probes = Vec::new();
        for request in Self::print_output_requests(source, analysis)? {
            probes.extend(request.probes);
        }
        if probes.is_empty() {
            return Ok(None);
        }
        Ok(Some(XycePrintOutputRequest {
            format: None,
            file: None,
            probes,
        }))
    }

    fn aggregate_print_output_requests(
        requests: Vec<XycePrintOutputRequest>,
        analysis: &str,
    ) -> Result<Vec<XycePrintOutputRequest>, String> {
        let mut aggregated: Vec<XycePrintOutputRequest> = Vec::new();
        for request in requests {
            if let Some(existing) = aggregated
                .iter_mut()
                .find(|existing| existing.file == request.file)
            {
                Self::validate_print_output_format_compatible(
                    existing.format.as_deref(),
                    request.format.as_deref(),
                    analysis,
                    request.file.as_deref(),
                )?;
                existing.probes.extend(request.probes);
                if existing.format.is_none() {
                    existing.format = request.format;
                }
            } else {
                aggregated.push(request);
            }
        }
        Ok(aggregated)
    }

    fn validate_print_output_format_compatible(
        existing: Option<&str>,
        incoming: Option<&str>,
        analysis: &str,
        file: Option<&str>,
    ) -> Result<(), String> {
        let existing_key = Self::print_format_key(existing);
        let incoming_key = Self::print_format_key(incoming);
        if existing_key == incoming_key {
            return Ok(());
        }
        let destination = file
            .map(|file| format!("FILE={file}"))
            .unwrap_or_else(|| "primary output".to_string());
        Err(format!(
            "multiple .PRINT {analysis} statements for {destination} use different FORMAT values ({existing_key} and {incoming_key})"
        ))
    }

    fn print_format_key(format: Option<&str>) -> String {
        format.unwrap_or("STD").trim().to_ascii_lowercase()
    }

    fn dc_print_output_requests(source: &str) -> Result<Vec<XycePrintOutputRequest>, String> {
        Self::print_output_requests(source, "DC")
    }

    fn deck_has_print_analysis(&self, deck: &XyceDeck, analysis: &str) -> bool {
        fs::read_to_string(&deck.path).is_ok_and(|source| {
            Self::print_output_requests(&source, analysis).is_ok_and(|requests| {
                requests
                    .into_iter()
                    .any(|request| request.file.is_none() && !request.probes.is_empty())
            })
        })
    }

    fn print_output_requests(
        source: &str,
        expected_analysis: &str,
    ) -> Result<Vec<XycePrintOutputRequest>, String> {
        let mut requests = Vec::new();
        for line in Self::logical_netlist_lines(source) {
            let trimmed = Self::strip_netlist_comment(&line).trim().to_string();
            if trimmed.is_empty() {
                continue;
            }
            let Some(command) = trimmed.split_whitespace().next() else {
                continue;
            };
            if !command.eq_ignore_ascii_case(".print") {
                continue;
            }
            let tokens = Self::split_print_fields(&trimmed)?;
            let token_refs = tokens.iter().map(String::as_str).collect::<Vec<_>>();
            let Some(analysis) = token_refs.get(1).copied() else {
                return Err(".PRINT statement has no analysis type".to_string());
            };
            if !analysis.eq_ignore_ascii_case(expected_analysis) {
                continue;
            }

            let mut format = None;
            let mut file = None;
            let mut probes = Vec::new();
            let mut index = 2usize;
            while index < tokens.len() {
                if let Some((raw_key, raw_value, consumed)) =
                    Self::print_option_assignment(&token_refs, index)
                {
                    let key = raw_key.trim().to_ascii_lowercase();
                    let value = raw_value.trim().trim_matches(['"', '\'']).to_string();
                    match key.as_str() {
                        "format" => format = Some(value),
                        "file" => file = Some(value),
                        _ => {}
                    }
                    index += consumed;
                    continue;
                }

                let normalized = token_refs[index].to_ascii_lowercase();
                if Self::is_print_option_token(&normalized) {
                    index += 1;
                    continue;
                }
                probes.push(tokens[index].to_string());
                index += 1;
            }

            if probes.is_empty() {
                continue;
            }
            requests.push(XycePrintOutputRequest {
                format,
                file,
                probes,
            });
        }

        Ok(requests)
    }

    fn split_print_fields(line: &str) -> Result<Vec<String>, String> {
        Self::split_grouped_whitespace_fields(line, ".PRINT statement")
    }

    fn split_prn_header_fields(line: &str) -> Result<Vec<String>, String> {
        Self::split_grouped_whitespace_fields(line, "Xyce .prn header")
    }

    fn split_grouped_whitespace_fields(
        line: &str,
        source_label: &str,
    ) -> Result<Vec<String>, String> {
        let mut fields = Vec::new();
        let mut current = String::new();
        let mut single_quote = false;
        let mut double_quote = false;
        let mut escaped = false;
        let mut brace_depth = 0usize;
        let mut paren_depth = 0usize;

        for ch in line.chars() {
            if escaped {
                current.push(ch);
                escaped = false;
                continue;
            }

            match ch {
                '\\' if single_quote || double_quote => {
                    current.push(ch);
                    escaped = true;
                }
                '\'' if !double_quote => {
                    single_quote = !single_quote;
                    current.push(ch);
                }
                '"' if !single_quote => {
                    double_quote = !double_quote;
                    current.push(ch);
                }
                '{' if !single_quote && !double_quote => {
                    brace_depth += 1;
                    current.push(ch);
                }
                '}' if !single_quote && !double_quote => {
                    brace_depth = brace_depth.checked_sub(1).ok_or_else(|| {
                        format!("unmatched closing brace in {source_label}: {line}")
                    })?;
                    current.push(ch);
                }
                '(' if !single_quote && !double_quote => {
                    paren_depth += 1;
                    current.push(ch);
                }
                ')' if !single_quote && !double_quote => {
                    paren_depth = paren_depth.checked_sub(1).ok_or_else(|| {
                        format!("unmatched closing parenthesis in {source_label}: {line}")
                    })?;
                    current.push(ch);
                }
                ch if ch.is_whitespace()
                    && !single_quote
                    && !double_quote
                    && brace_depth == 0
                    && paren_depth == 0 =>
                {
                    if !current.is_empty() {
                        fields.push(std::mem::take(&mut current));
                    }
                }
                _ => current.push(ch),
            }
        }

        if single_quote || double_quote {
            return Err(format!("unterminated quote in {source_label}: {line}"));
        }
        if brace_depth != 0 {
            return Err(format!(
                "unterminated brace expression in {source_label}: {line}"
            ));
        }
        if paren_depth != 0 {
            return Err(format!(
                "unterminated parenthesized probe in {source_label}: {line}"
            ));
        }
        if !current.is_empty() {
            fields.push(current);
        }

        Ok(fields)
    }

    fn gnuplot_splot_print_pair(
        source: &str,
    ) -> Result<(XycePrintOutputRequest, XycePrintOutputRequest), String> {
        let requests = Self::dc_print_output_requests(source)?;
        let mut primary = None;
        let mut side = None;
        for request in requests {
            let format = request.format.as_deref().unwrap_or("STD");
            if request.file.is_none() && format.eq_ignore_ascii_case("GNUPLOT") {
                if primary.replace(request).is_some() {
                    return Err(
                        "GNUPLOT/SPLOT contract requires exactly one primary GNUPLOT .PRINT DC"
                            .to_string(),
                    );
                }
                continue;
            }
            if request.file.is_some() && format.eq_ignore_ascii_case("SPLOT") {
                if side.replace(request).is_some() {
                    return Err(
                        "GNUPLOT/SPLOT contract requires exactly one named SPLOT .PRINT DC"
                            .to_string(),
                    );
                }
                continue;
            }
            return Err(format!(
                "GNUPLOT/SPLOT contract does not cover .PRINT DC FORMAT={format} FILE={}",
                request.file.as_deref().unwrap_or("<default>")
            ));
        }

        match (primary, side) {
            (Some(primary), Some(side)) => Ok((primary, side)),
            _ => Err(
                "GNUPLOT/SPLOT contract requires one primary GNUPLOT and one named SPLOT .PRINT DC"
                    .to_string(),
            ),
        }
    }

    fn is_print_option_token(token: &str) -> bool {
        token.starts_with("format=")
            || token.starts_with("width=")
            || token.starts_with("precision=")
            || token.starts_with("delimiter=")
            || token.starts_with("noindex")
            || token.starts_with("index=")
    }

    fn parse_prn_file(path: &Path) -> Result<XycePrnTable, String> {
        let content =
            fs::read_to_string(path).map_err(|err| format!("{}: {err}", path.display()))?;
        Self::parse_prn_table(&content)
    }

    fn parse_dc_reference_file(
        contract: XyceStaticDcContract,
        path: &Path,
    ) -> Result<XycePrnTable, String> {
        match contract {
            XyceStaticDcContract::WrapperRaw => Self::parse_raw_file(path),
            XyceStaticDcContract::PlainCsv | XyceStaticDcContract::WrapperCsv => {
                Self::parse_csv_file(path)
            }
            XyceStaticDcContract::PlainCsd | XyceStaticDcContract::WrapperCsd => {
                Self::parse_csd_file(path)
            }
            _ => Self::parse_prn_file(path),
        }
    }

    fn parse_ac_reference_file(
        contract: XyceStaticAcContract,
        path: &Path,
    ) -> Result<XycePrnTable, String> {
        match contract {
            XyceStaticAcContract::PlainCsd | XyceStaticAcContract::WrapperCsd => {
                Self::parse_ac_csd_file(path)
            }
            _ => Self::parse_prn_file(path),
        }
    }

    fn parse_tran_reference_file(
        contract: XyceStaticTranContract,
        path: &Path,
    ) -> Result<XycePrnTable, String> {
        match contract {
            XyceStaticTranContract::PlainCsv | XyceStaticTranContract::WrapperCsv => {
                Self::parse_csv_file(path)
            }
            XyceStaticTranContract::PlainCsd | XyceStaticTranContract::WrapperCsd => {
                Self::parse_tran_csd_file(path)
            }
            _ => Self::parse_tran_prn_or_legacy_probe_file(path),
        }
    }

    fn parse_tran_prn_or_legacy_probe_file(path: &Path) -> Result<XycePrnTable, String> {
        let content =
            fs::read_to_string(path).map_err(|err| format!("{}: {err}", path.display()))?;
        Self::parse_tran_prn_or_legacy_probe_table(&content)
    }

    fn parse_tran_prn_or_legacy_probe_table(content: &str) -> Result<XycePrnTable, String> {
        let first_nonempty = content.lines().map(str::trim).find(|line| !line.is_empty());
        if first_nonempty.is_some_and(|line| line.eq_ignore_ascii_case("#H")) {
            return Self::parse_tran_csd_table(content);
        }
        Self::parse_prn_table(content)
    }

    fn parse_csv_file(path: &Path) -> Result<XycePrnTable, String> {
        let content =
            fs::read_to_string(path).map_err(|err| format!("{}: {err}", path.display()))?;
        Self::parse_prn_table(&content)
    }

    fn parse_raw_file(path: &Path) -> Result<XycePrnTable, String> {
        let bytes = fs::read(path).map_err(|err| format!("{}: {err}", path.display()))?;
        Self::parse_raw_table(&bytes)
    }

    fn parse_csd_file(path: &Path) -> Result<XycePrnTable, String> {
        let content =
            fs::read_to_string(path).map_err(|err| format!("{}: {err}", path.display()))?;
        Self::parse_csd_table(&content)
    }

    fn parse_ac_csd_file(path: &Path) -> Result<XycePrnTable, String> {
        let content =
            fs::read_to_string(path).map_err(|err| format!("{}: {err}", path.display()))?;
        Self::parse_ac_csd_table(&content)
    }

    fn parse_tran_csd_file(path: &Path) -> Result<XycePrnTable, String> {
        let content =
            fs::read_to_string(path).map_err(|err| format!("{}: {err}", path.display()))?;
        Self::parse_tran_csd_table(&content)
    }

    fn parse_tran_csd_table(content: &str) -> Result<XycePrnTable, String> {
        let lines = content
            .lines()
            .enumerate()
            .map(|(line_number, line)| (line_number + 1, line.trim()))
            .filter(|(_, line)| !line.is_empty())
            .collect::<Vec<_>>();
        if lines.is_empty() {
            return Err("empty Xyce transient CSDF table".to_string());
        }

        let mut columns: Option<Vec<String>> = None;
        let mut rows = Vec::new();
        let mut index = 0usize;
        while index < lines.len() {
            let (line_number, line) = lines[index];
            if !line.eq_ignore_ascii_case("#H") {
                return Err(format!(
                    "Xyce transient CSDF section must start with #H at line {line_number}, got '{line}'"
                ));
            }
            index += 1;

            let mut complex_values = false;
            let mut sweep_column = "TIME".to_string();
            while index < lines.len() {
                let (_, header_line) = lines[index];
                if header_line.eq_ignore_ascii_case("#N") {
                    break;
                }
                for (key, value) in Self::parse_csd_header_assignments(header_line) {
                    if key.eq_ignore_ascii_case("COMPLEXVALUES")
                        && value.eq_ignore_ascii_case("YES")
                    {
                        complex_values = true;
                    }
                    if key.eq_ignore_ascii_case("SWEEPVAR") {
                        sweep_column = value.to_ascii_uppercase();
                    }
                }
                index += 1;
            }
            if complex_values {
                return Err("Xyce transient CSDF COMPLEXVALUES=YES is not supported".to_string());
            }
            if index >= lines.len() {
                return Err("Xyce transient CSDF section has no #N column block".to_string());
            }

            index += 1;
            let Some((column_line_number, column_line)) = lines.get(index).copied() else {
                return Err("Xyce transient CSDF #N block has no column line".to_string());
            };
            let section_input_columns = Self::parse_csd_columns(column_line).map_err(|err| {
                format!("invalid Xyce transient CSDF column line {column_line_number}: {err}")
            })?;
            if section_input_columns.is_empty() {
                return Err(format!(
                    "Xyce transient CSDF column line {column_line_number} has no columns"
                ));
            }
            let mut section_columns = Vec::with_capacity(section_input_columns.len() + 1);
            section_columns.push(sweep_column);
            section_columns.extend(section_input_columns.iter().cloned());
            if let Some(columns) = &columns {
                if !Self::same_prn_columns(columns, &section_columns) {
                    return Err(format!(
                        "Xyce transient CSDF section changes columns from {:?} to {:?}",
                        columns, section_columns
                    ));
                }
            } else {
                columns = Some(section_columns);
            }
            index += 1;

            while index < lines.len() {
                let (line_number, line) = lines[index];
                if line.eq_ignore_ascii_case("#;") {
                    index += 1;
                    break;
                }
                if line.eq_ignore_ascii_case("#H") {
                    break;
                }
                if !line.starts_with("#C") {
                    return Err(format!(
                        "expected Xyce transient CSDF #C row header at line {line_number}, got '{line}'"
                    ));
                }
                let (time, expected_count) =
                    Self::parse_csd_sweep_row_header(line).map_err(|err| {
                        format!(
                            "invalid Xyce transient CSDF #C row header at line {line_number}: {err}"
                        )
                    })?;
                index += 1;

                let mut row = Vec::with_capacity(expected_count + 1);
                row.push(time);
                while row.len() <= expected_count {
                    let Some((data_line_number, data_line)) = lines.get(index).copied() else {
                        return Err(format!(
                            "Xyce transient CSDF row beginning at line {line_number} ended after {} value(s), expected {expected_count}",
                            row.len().saturating_sub(1)
                        ));
                    };
                    if data_line.starts_with('#') {
                        return Err(format!(
                            "Xyce transient CSDF row beginning at line {line_number} ended before {expected_count} value(s) at line {data_line_number}"
                        ));
                    }
                    for token in data_line.split_whitespace() {
                        if row.len() > expected_count {
                            return Err(format!(
                                "Xyce transient CSDF row beginning at line {line_number} has more than {expected_count} value(s)"
                            ));
                        }
                        let expected_position = row.len();
                        let value = Self::parse_csd_complex_value_token(token, expected_position)
                            .map_err(|err| {
                                format!(
                                    "invalid Xyce transient CSDF data token '{token}' at line {data_line_number}: {err}"
                                )
                            })?;
                        if value.im.abs() > f64::EPSILON {
                            return Err(format!(
                                "Xyce transient CSDF token '{token}' at line {data_line_number} has nonzero imaginary component {}",
                                value.im
                            ));
                        }
                        row.push(value.re);
                    }
                    index += 1;
                }
                if let Some(columns) = &columns {
                    if row.len() != columns.len() {
                        return Err(format!(
                            "Xyce transient CSDF row beginning at line {line_number} has {} column value(s), expected {}",
                            row.len(),
                            columns.len()
                        ));
                    }
                }
                rows.push(row);
            }
        }

        let columns =
            columns.ok_or_else(|| "Xyce transient CSDF table has no columns".to_string())?;
        if rows.is_empty() {
            return Err("Xyce transient CSDF table has no data rows".to_string());
        }
        Ok(XycePrnTable { columns, rows })
    }

    fn parse_ac_csd_table(content: &str) -> Result<XycePrnTable, String> {
        let lines = content
            .lines()
            .enumerate()
            .map(|(line_number, line)| (line_number + 1, line.trim()))
            .filter(|(_, line)| !line.is_empty())
            .collect::<Vec<_>>();
        if lines.is_empty() {
            return Err("empty Xyce AC CSDF table".to_string());
        }

        let mut columns: Option<Vec<String>> = None;
        let mut expansion: Option<Vec<XyceAcCsdColumnExpansion>> = None;
        let mut rows = Vec::new();
        let mut index = 0usize;
        while index < lines.len() {
            let (line_number, line) = lines[index];
            if !line.eq_ignore_ascii_case("#H") {
                return Err(format!(
                    "Xyce AC CSDF section must start with #H at line {line_number}, got '{line}'"
                ));
            }
            index += 1;

            let mut complex_values = false;
            let mut sweep_column = "FREQ".to_string();
            while index < lines.len() {
                let (_, header_line) = lines[index];
                if header_line.eq_ignore_ascii_case("#N") {
                    break;
                }
                for (key, value) in Self::parse_csd_header_assignments(header_line) {
                    if key.eq_ignore_ascii_case("COMPLEXVALUES")
                        && value.eq_ignore_ascii_case("YES")
                    {
                        complex_values = true;
                    }
                    if key.eq_ignore_ascii_case("SWEEPVAR") {
                        sweep_column = value.to_ascii_uppercase();
                    }
                }
                index += 1;
            }
            if index >= lines.len() {
                return Err("Xyce AC CSDF section has no #N column block".to_string());
            }

            index += 1;
            let Some((column_line_number, column_line)) = lines.get(index).copied() else {
                return Err("Xyce AC CSDF #N block has no column line".to_string());
            };
            let section_input_columns = Self::parse_csd_columns(column_line).map_err(|err| {
                format!("invalid Xyce AC CSDF column line {column_line_number}: {err}")
            })?;
            if section_input_columns.is_empty() {
                return Err(format!(
                    "Xyce AC CSDF column line {column_line_number} has no columns"
                ));
            }
            let section_expansion =
                Self::ac_csd_column_expansion(&section_input_columns, complex_values);
            let section_columns = Self::expanded_ac_csd_columns(
                &sweep_column,
                &section_input_columns,
                &section_expansion,
            );
            if let Some(columns) = &columns {
                if !Self::same_prn_columns(columns, &section_columns) {
                    return Err(format!(
                        "Xyce AC CSDF section changes columns from {:?} to {:?}",
                        columns, section_columns
                    ));
                }
            } else {
                columns = Some(section_columns);
                expansion = Some(section_expansion);
            }
            let expansion = expansion
                .as_ref()
                .expect("AC CSDF expansion initialized with columns");
            index += 1;

            while index < lines.len() {
                let (line_number, line) = lines[index];
                if line.eq_ignore_ascii_case("#;") {
                    index += 1;
                    break;
                }
                if line.eq_ignore_ascii_case("#H") {
                    break;
                }
                if !line.starts_with("#C") {
                    return Err(format!(
                        "expected Xyce AC CSDF #C row header at line {line_number}, got '{line}'"
                    ));
                }
                let (frequency, expected_count) =
                    Self::parse_csd_sweep_row_header(line).map_err(|err| {
                        format!("invalid Xyce AC CSDF #C row header at line {line_number}: {err}")
                    })?;
                index += 1;

                let mut tokens = Vec::with_capacity(expected_count);
                while tokens.len() < expected_count {
                    let Some((data_line_number, data_line)) = lines.get(index).copied() else {
                        return Err(format!(
                            "Xyce AC CSDF row beginning at line {line_number} ended after {} value(s), expected {expected_count}",
                            tokens.len()
                        ));
                    };
                    if data_line.starts_with('#') {
                        return Err(format!(
                            "Xyce AC CSDF row beginning at line {line_number} ended before {expected_count} value(s) at line {data_line_number}"
                        ));
                    }
                    for token in data_line.split_whitespace() {
                        if tokens.len() >= expected_count {
                            return Err(format!(
                                "Xyce AC CSDF row beginning at line {line_number} has more than {expected_count} value(s)"
                            ));
                        }
                        let expected_position = tokens.len() + 1;
                        tokens.push(
                            Self::parse_csd_complex_value_token(token, expected_position).map_err(
                                |err| {
                                    format!(
                                        "invalid Xyce AC CSDF data token '{token}' at line {data_line_number}: {err}"
                                    )
                                },
                            )?,
                        );
                    }
                    index += 1;
                }
                if tokens.len() != section_input_columns.len() {
                    return Err(format!(
                        "Xyce AC CSDF row beginning at line {line_number} has {} value(s), expected {} column(s)",
                        tokens.len(),
                        section_input_columns.len()
                    ));
                }

                let mut row = Vec::new();
                row.push(frequency);
                for (value, expansion) in tokens.into_iter().zip(expansion.iter()) {
                    match expansion {
                        XyceAcCsdColumnExpansion::Scalar => {
                            if value.im.abs() > f64::EPSILON {
                                return Err(format!(
                                    "Xyce AC CSDF scalar row beginning at line {line_number} has nonzero imaginary component {}",
                                    value.im
                                ));
                            }
                            row.push(value.re);
                        }
                        XyceAcCsdColumnExpansion::Complex => {
                            row.push(value.re);
                            row.push(value.im);
                        }
                    }
                }
                rows.push(row);
            }
        }

        let columns = columns.ok_or_else(|| "Xyce AC CSDF table has no columns".to_string())?;
        if rows.is_empty() {
            return Err("Xyce AC CSDF table has no data rows".to_string());
        }
        Ok(XycePrnTable { columns, rows })
    }

    fn ac_csd_column_expansion(
        columns: &[String],
        complex_values: bool,
    ) -> Vec<XyceAcCsdColumnExpansion> {
        columns
            .iter()
            .map(|column| {
                if complex_values && Self::ac_csd_column_is_complex_probe(column) {
                    XyceAcCsdColumnExpansion::Complex
                } else {
                    XyceAcCsdColumnExpansion::Scalar
                }
            })
            .collect()
    }

    fn expanded_ac_csd_columns(
        sweep_column: &str,
        input_columns: &[String],
        expansion: &[XyceAcCsdColumnExpansion],
    ) -> Vec<String> {
        let mut columns = Vec::with_capacity(
            1 + input_columns.len()
                + expansion
                    .iter()
                    .filter(|kind| matches!(kind, XyceAcCsdColumnExpansion::Complex))
                    .count(),
        );
        columns.push(sweep_column.to_string());
        for (column, expansion) in input_columns.iter().zip(expansion.iter()) {
            match expansion {
                XyceAcCsdColumnExpansion::Scalar => columns.push(column.clone()),
                XyceAcCsdColumnExpansion::Complex => {
                    columns.push(format!("Re({column})"));
                    columns.push(format!("Im({column})"));
                }
            }
        }
        columns
    }

    fn ac_csd_column_is_complex_probe(column: &str) -> bool {
        let normalized = Self::normalize_probe(column);
        if let Some(voltage_probe) = Self::parse_ac_voltage_probe(&normalized) {
            return voltage_probe.accessor == XyceVoltageAccessor::Value;
        }
        if let Some(current_probe) = Self::parse_ac_current_probe(&normalized) {
            return current_probe.accessor == XyceCurrentAccessor::Value;
        }
        false
    }

    fn parse_csd_table(content: &str) -> Result<XycePrnTable, String> {
        let lines = content
            .lines()
            .enumerate()
            .map(|(line_number, line)| (line_number + 1, line.trim()))
            .filter(|(_, line)| !line.is_empty())
            .collect::<Vec<_>>();
        if lines.is_empty() {
            return Err("empty Xyce CSDF table".to_string());
        }

        let mut columns: Option<Vec<String>> = None;
        let mut rows = Vec::new();
        let mut index = 0usize;
        while index < lines.len() {
            let (line_number, line) = lines[index];
            if !line.eq_ignore_ascii_case("#H") {
                return Err(format!(
                    "Xyce CSDF section must start with #H at line {line_number}, got '{line}'"
                ));
            }
            index += 1;

            let mut complex_values = false;
            while index < lines.len() {
                let (_, header_line) = lines[index];
                if header_line.eq_ignore_ascii_case("#N") {
                    break;
                }
                for (key, value) in Self::parse_csd_header_assignments(header_line) {
                    if key.eq_ignore_ascii_case("COMPLEXVALUES")
                        && value.eq_ignore_ascii_case("YES")
                    {
                        complex_values = true;
                    }
                }
                index += 1;
            }
            if complex_values {
                return Err("Xyce CSDF COMPLEXVALUES=YES is not supported".to_string());
            }
            if index >= lines.len() {
                return Err("Xyce CSDF section has no #N column block".to_string());
            }

            index += 1;
            let Some((column_line_number, column_line)) = lines.get(index).copied() else {
                return Err("Xyce CSDF #N block has no column line".to_string());
            };
            let section_columns = Self::parse_csd_columns(column_line).map_err(|err| {
                format!("invalid Xyce CSDF column line {column_line_number}: {err}")
            })?;
            if section_columns.is_empty() {
                return Err(format!(
                    "Xyce CSDF column line {column_line_number} has no columns"
                ));
            }
            if let Some(columns) = &columns {
                if !Self::same_prn_columns(columns, &section_columns) {
                    return Err(format!(
                        "Xyce CSDF section changes columns from {:?} to {:?}",
                        columns, section_columns
                    ));
                }
            } else {
                columns = Some(section_columns);
            }
            index += 1;

            while index < lines.len() {
                let (line_number, line) = lines[index];
                if line.eq_ignore_ascii_case("#;") {
                    index += 1;
                    break;
                }
                if line.eq_ignore_ascii_case("#H") {
                    break;
                }
                if !line.starts_with("#C") {
                    return Err(format!(
                        "expected Xyce CSDF #C row header at line {line_number}, got '{line}'"
                    ));
                }
                let (_, expected_count) =
                    Self::parse_csd_sweep_row_header(line).map_err(|err| {
                        format!("invalid Xyce CSDF #C row header at line {line_number}: {err}")
                    })?;
                index += 1;

                let mut row = Vec::with_capacity(expected_count);
                while row.len() < expected_count {
                    let Some((data_line_number, data_line)) = lines.get(index).copied() else {
                        return Err(format!(
                            "Xyce CSDF row beginning at line {line_number} ended after {} value(s), expected {expected_count}",
                            row.len()
                        ));
                    };
                    if data_line.starts_with('#') {
                        return Err(format!(
                            "Xyce CSDF row beginning at line {line_number} ended before {expected_count} value(s) at line {data_line_number}"
                        ));
                    }
                    for token in data_line.split_whitespace() {
                        if row.len() >= expected_count {
                            return Err(format!(
                                "Xyce CSDF row beginning at line {line_number} has more than {expected_count} value(s)"
                            ));
                        }
                        let expected_position = row.len() + 1;
                        let value = Self::parse_csd_complex_value_token(token, expected_position)
                            .map_err(|err| {
                                format!(
                                    "invalid Xyce CSDF data token '{token}' at line {data_line_number}: {err}"
                                )
                            })?;
                        if value.im.abs() > f64::EPSILON {
                            return Err(format!(
                                "Xyce CSDF real table token '{token}' at line {data_line_number} has nonzero imaginary component {}",
                                value.im
                            ));
                        }
                        row.push(value.re);
                    }
                    index += 1;
                }
                if let Some(columns) = &columns {
                    if row.len() != columns.len() {
                        return Err(format!(
                            "Xyce CSDF row beginning at line {line_number} has {} value(s), expected {} column(s)",
                            row.len(),
                            columns.len()
                        ));
                    }
                }
                rows.push(row);
            }
        }

        let columns = columns.ok_or_else(|| "Xyce CSDF table has no columns".to_string())?;
        if rows.is_empty() {
            return Err("Xyce CSDF table has no data rows".to_string());
        }
        Ok(XycePrnTable { columns, rows })
    }

    fn parse_csd_header_assignments(line: &str) -> Vec<(&str, &str)> {
        line.split_whitespace()
            .filter_map(|field| {
                let (key, value) = field.split_once('=')?;
                Some((key.trim(), value.trim_matches(['"', '\''])))
            })
            .collect()
    }

    fn parse_csd_columns(line: &str) -> Result<Vec<String>, String> {
        if !line.contains('\'') {
            return Ok(line.split_whitespace().map(str::to_string).collect());
        }

        let mut columns = Vec::new();
        let mut rest = line;
        loop {
            let Some(start) = rest.find('\'') else {
                if rest.trim().is_empty() {
                    return Ok(columns);
                }
                return Err(format!(
                    "unexpected unquoted text after CSDF column list: '{}'",
                    rest.trim()
                ));
            };
            if !rest[..start].trim().is_empty() {
                return Err(format!(
                    "unexpected text before quoted CSDF column: '{}'",
                    rest[..start].trim()
                ));
            }
            let after_start = &rest[start + 1..];
            let Some(end) = after_start.find('\'') else {
                return Err("unterminated quoted CSDF column".to_string());
            };
            columns.push(after_start[..end].to_string());
            rest = &after_start[end + 1..];
        }
    }

    fn parse_csd_sweep_row_header(line: &str) -> Result<(f64, usize), String> {
        let fields = line.split_whitespace().collect::<Vec<_>>();
        if fields.len() != 3 || !fields[0].eq_ignore_ascii_case("#C") {
            return Err("expected '#C <sweep-value> <value-count>'".to_string());
        }
        let sweep_value = Self::parse_xyce_numeric_token(fields[1])
            .map_err(|err| format!("invalid sweep value '{}': {err}", fields[1]))?;
        let value_count = fields[2]
            .parse::<usize>()
            .map_err(|err| format!("invalid value count '{}': {err}", fields[2]))?;
        Ok((sweep_value, value_count))
    }

    fn parse_csd_complex_value_token(
        token: &str,
        expected_position: usize,
    ) -> Result<Complex64, String> {
        let (value, position) = token
            .split_once(':')
            .ok_or_else(|| "expected '<value>:<position>'".to_string())?;
        let position = position
            .parse::<usize>()
            .map_err(|err| format!("invalid position '{}': {err}", position))?;
        if position != expected_position {
            return Err(format!(
                "position {position} does not match expected position {expected_position}"
            ));
        }
        let (real, imaginary) = value.split_once('/').unwrap_or((value, "0"));
        let real = Self::parse_xyce_numeric_token(real)
            .map_err(|err| format!("invalid real value '{}': {err}", real))?;
        let imaginary = Self::parse_xyce_numeric_token(imaginary)
            .map_err(|err| format!("invalid imaginary value '{}': {err}", imaginary))?;
        Ok(Complex64::new(real, imaginary))
    }

    fn parse_raw_table(bytes: &[u8]) -> Result<XycePrnTable, String> {
        let mut offset = 0usize;
        let mut columns: Option<Vec<String>> = None;
        let mut rows = Vec::new();
        let mut plot_index = 0usize;

        while offset < bytes.len() {
            Self::skip_raw_blank_lines(bytes, &mut offset);
            if offset >= bytes.len() {
                break;
            }

            let (plot_columns, plot_rows) = Self::parse_raw_plot(bytes, &mut offset, plot_index)?;
            if let Some(columns) = &columns {
                if !Self::same_prn_columns(columns, &plot_columns) {
                    return Err(format!(
                        "Xyce RAW plot {} changes variables from {:?} to {:?}",
                        plot_index + 1,
                        columns,
                        plot_columns
                    ));
                }
            } else {
                columns = Some(plot_columns);
            }
            rows.extend(plot_rows);
            plot_index += 1;
        }

        let columns = columns.ok_or_else(|| "empty Xyce RAW table".to_string())?;
        if rows.is_empty() {
            return Err("Xyce RAW table has no data rows".to_string());
        }

        Ok(XycePrnTable { columns, rows })
    }

    fn parse_raw_plot(
        bytes: &[u8],
        offset: &mut usize,
        plot_index: usize,
    ) -> Result<(Vec<String>, Vec<Vec<f64>>), String> {
        let mut flags = None;
        let mut variable_count = None;
        let mut point_count = None;

        loop {
            let line = Self::read_raw_line(bytes, offset).ok_or_else(|| {
                format!(
                    "Xyce RAW plot {} ended before Variables header",
                    plot_index + 1
                )
            })?;
            let trimmed = line.trim();
            if trimmed.is_empty()
                || trimmed.starts_with("Title:")
                || trimmed.starts_with("Date:")
                || trimmed.starts_with("Version:")
            {
                continue;
            }
            if trimmed.starts_with("Plotname:") {
                continue;
            }
            if let Some(value) = trimmed.strip_prefix("Flags:") {
                flags = Some(value.trim().to_string());
                continue;
            }
            if let Some(value) = trimmed.strip_prefix("No. Variables:") {
                variable_count = Some(value.trim().parse::<usize>().map_err(|err| {
                    format!(
                        "invalid RAW variable count '{}' in plot {}: {err}",
                        value.trim(),
                        plot_index + 1
                    )
                })?);
                continue;
            }
            if let Some(value) = trimmed.strip_prefix("No. Points:") {
                point_count = Some(value.trim().parse::<usize>().map_err(|err| {
                    format!(
                        "invalid RAW point count '{}' in plot {}: {err}",
                        value.trim(),
                        plot_index + 1
                    )
                })?);
                continue;
            }
            if trimmed.eq_ignore_ascii_case("Variables:") {
                break;
            }
            return Err(format!(
                "unexpected RAW header line in plot {}: '{}'",
                plot_index + 1,
                trimmed
            ));
        }

        let flags =
            flags.ok_or_else(|| format!("RAW plot {} has no Flags line", plot_index + 1))?;
        if !flags
            .split_whitespace()
            .any(|flag| flag.eq_ignore_ascii_case("real"))
        {
            return Err(format!(
                "RAW plot {} uses unsupported Flags: {flags}; only real-valued RAW is supported",
                plot_index + 1
            ));
        }
        if flags
            .split_whitespace()
            .any(|flag| flag.eq_ignore_ascii_case("complex"))
        {
            return Err(format!(
                "RAW plot {} is complex-valued; DC RAW comparison currently supports real data",
                plot_index + 1
            ));
        }
        let variable_count = variable_count
            .ok_or_else(|| format!("RAW plot {} has no variable count", plot_index + 1))?;
        let point_count =
            point_count.ok_or_else(|| format!("RAW plot {} has no point count", plot_index + 1))?;
        if variable_count == 0 {
            return Err(format!("RAW plot {} has no variables", plot_index + 1));
        }

        let mut columns = Vec::with_capacity(variable_count);
        for variable_index in 0..variable_count {
            let line = Self::read_raw_line(bytes, offset).ok_or_else(|| {
                format!(
                    "RAW plot {} ended while reading variable {}",
                    plot_index + 1,
                    variable_index
                )
            })?;
            let fields = line.split_whitespace().collect::<Vec<_>>();
            if fields.len() < 3 {
                return Err(format!(
                    "invalid RAW variable line in plot {}: '{}'",
                    plot_index + 1,
                    line.trim()
                ));
            }
            let parsed_index = fields[0].parse::<usize>().map_err(|err| {
                format!(
                    "invalid RAW variable index '{}' in plot {}: {err}",
                    fields[0],
                    plot_index + 1
                )
            })?;
            if parsed_index != variable_index {
                return Err(format!(
                    "RAW variable index {} appears where {} was expected in plot {}",
                    parsed_index,
                    variable_index,
                    plot_index + 1
                ));
            }
            columns.push(fields[1].to_string());
        }

        let data_marker = loop {
            let line = Self::read_raw_line(bytes, offset).ok_or_else(|| {
                format!(
                    "RAW plot {} ended before Values/Binary marker",
                    plot_index + 1
                )
            })?;
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            break trimmed.to_ascii_lowercase();
        };

        let rows = match data_marker.as_str() {
            "values:" => {
                Self::parse_ascii_raw_rows(bytes, offset, plot_index, point_count, variable_count)?
            }
            "binary:" => {
                Self::parse_binary_raw_rows(bytes, offset, plot_index, point_count, variable_count)?
            }
            other => {
                return Err(format!(
                    "RAW plot {} expected Values: or Binary:, got '{}'",
                    plot_index + 1,
                    other
                ));
            }
        };

        Ok((columns, rows))
    }

    fn parse_ascii_raw_rows(
        bytes: &[u8],
        offset: &mut usize,
        plot_index: usize,
        point_count: usize,
        variable_count: usize,
    ) -> Result<Vec<Vec<f64>>, String> {
        let mut rows = Vec::with_capacity(point_count);
        for point_index in 0..point_count {
            let first_line = Self::read_next_nonempty_raw_line(bytes, offset).ok_or_else(|| {
                format!(
                    "RAW plot {} ended while reading point {}",
                    plot_index + 1,
                    point_index
                )
            })?;
            let first_fields = first_line.split_whitespace().collect::<Vec<_>>();
            if first_fields.is_empty() {
                return Err(format!(
                    "RAW plot {} has empty point line for point {}",
                    plot_index + 1,
                    point_index
                ));
            }
            let parsed_index = first_fields[0].parse::<usize>().map_err(|err| {
                format!(
                    "invalid RAW point index '{}' in plot {}: {err}",
                    first_fields[0],
                    plot_index + 1
                )
            })?;
            if parsed_index != point_index {
                return Err(format!(
                    "RAW point index {} appears where {} was expected in plot {}",
                    parsed_index,
                    point_index,
                    plot_index + 1
                ));
            }

            let mut row = first_fields
                .iter()
                .skip(1)
                .map(|token| {
                    Self::parse_xyce_numeric_token(token).map_err(|err| {
                        format!(
                            "invalid RAW numeric token '{}' at plot {}, point {}: {err}",
                            token,
                            plot_index + 1,
                            point_index
                        )
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
            while row.len() < variable_count {
                let line = Self::read_next_nonempty_raw_line(bytes, offset).ok_or_else(|| {
                    format!(
                        "RAW plot {} ended while reading values for point {}",
                        plot_index + 1,
                        point_index
                    )
                })?;
                for token in line.split_whitespace() {
                    row.push(Self::parse_xyce_numeric_token(token).map_err(|err| {
                        format!(
                            "invalid RAW numeric token '{}' at plot {}, point {}: {err}",
                            token,
                            plot_index + 1,
                            point_index
                        )
                    })?);
                }
            }
            if row.len() != variable_count {
                return Err(format!(
                    "RAW plot {} point {} has {} values, expected {}",
                    plot_index + 1,
                    point_index,
                    row.len(),
                    variable_count
                ));
            }
            rows.push(row);
        }
        Ok(rows)
    }

    fn parse_binary_raw_rows(
        bytes: &[u8],
        offset: &mut usize,
        plot_index: usize,
        point_count: usize,
        variable_count: usize,
    ) -> Result<Vec<Vec<f64>>, String> {
        let value_count = point_count
            .checked_mul(variable_count)
            .ok_or_else(|| format!("RAW plot {} point/variable count overflows", plot_index + 1))?;
        let byte_count = value_count
            .checked_mul(std::mem::size_of::<f64>())
            .ok_or_else(|| format!("RAW plot {} binary byte count overflows", plot_index + 1))?;
        if bytes.len().saturating_sub(*offset) < byte_count {
            return Err(format!(
                "RAW plot {} binary payload has {} byte(s), expected {}",
                plot_index + 1,
                bytes.len().saturating_sub(*offset),
                byte_count
            ));
        }

        let mut rows = Vec::with_capacity(point_count);
        for point_index in 0..point_count {
            let mut row = Vec::with_capacity(variable_count);
            for variable_index in 0..variable_count {
                let start = *offset
                    + (point_index * variable_count + variable_index) * std::mem::size_of::<f64>();
                let bytes: [u8; 8] = bytes[start..start + 8]
                    .try_into()
                    .expect("slice length checked");
                row.push(f64::from_le_bytes(bytes));
            }
            rows.push(row);
        }
        *offset += byte_count;
        Ok(rows)
    }

    fn read_next_nonempty_raw_line(bytes: &[u8], offset: &mut usize) -> Option<String> {
        loop {
            let line = Self::read_raw_line(bytes, offset)?;
            if !line.trim().is_empty() {
                return Some(line);
            }
        }
    }

    fn skip_raw_blank_lines(bytes: &[u8], offset: &mut usize) {
        while *offset < bytes.len() {
            let mut cursor = *offset;
            let Some(line) = Self::read_raw_line(bytes, &mut cursor) else {
                return;
            };
            if !line.trim().is_empty() {
                return;
            }
            *offset = cursor;
        }
    }

    fn read_raw_line(bytes: &[u8], offset: &mut usize) -> Option<String> {
        if *offset >= bytes.len() {
            return None;
        }
        let start = *offset;
        let mut end = start;
        while end < bytes.len() && bytes[end] != b'\n' {
            end += 1;
        }
        *offset = end.saturating_add(usize::from(end < bytes.len()));
        let mut line = &bytes[start..end];
        if line.ends_with(b"\r") {
            line = &line[..line.len().saturating_sub(1)];
        }
        Some(String::from_utf8_lossy(line).into_owned())
    }

    fn is_parameter_sweep_summary_reference(path: &Path) -> bool {
        let Ok(content) = fs::read_to_string(path) else {
            return false;
        };
        let mut lines = content
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty());
        let Some(header) = lines.next() else {
            return false;
        };
        header.to_ascii_uppercase().starts_with("STEP")
            && content
                .to_ascii_lowercase()
                .contains("end of xyce(tm) parameter sweep")
    }

    fn parse_prn_table(content: &str) -> Result<XycePrnTable, String> {
        let nonempty_lines = content
            .lines()
            .enumerate()
            .map(|(line_number, line)| (line_number + 1, line.trim()))
            .filter(|(_, line)| !line.is_empty())
            .collect::<Vec<_>>();
        if nonempty_lines.is_empty() {
            return Err("empty Xyce .prn table".to_string());
        }

        let Some((header_index, (_header_line_number, header))) = nonempty_lines
            .iter()
            .enumerate()
            .find(|(_, (_, line))| Self::is_prn_header_line(line))
        else {
            return Err("Xyce .prn table has no header".to_string());
        };
        let delimiter = Self::prn_header_delimiter(header)
            .ok_or_else(|| format!("invalid Xyce .prn header '{}'", header))?;
        let columns = Self::parse_prn_columns(header, delimiter)?;
        if columns.is_empty() {
            return Err("Xyce .prn header has no columns".to_string());
        }

        let mut rows = Vec::new();
        for (line_number, line) in nonempty_lines.iter().skip(header_index + 1).copied() {
            if line.to_ascii_lowercase().starts_with("end of xyce") {
                break;
            }
            if !rows.is_empty() && Self::is_prn_footer_line(line) {
                break;
            }
            if Self::is_prn_separator_line(line) {
                continue;
            }
            if Self::is_prn_header_line(line) {
                let repeated_delimiter = Self::prn_header_delimiter(line)
                    .ok_or_else(|| format!("invalid repeated Xyce .prn header '{}'", line))?;
                let repeated_columns = Self::parse_prn_columns(line, repeated_delimiter)?;
                if Self::same_prn_columns(&columns, &repeated_columns) {
                    continue;
                }
                return Err(format!(
                    "Xyce .prn table changes columns at line {}; multi-table .prn output is not supported",
                    line_number
                ));
            }
            let values = Self::split_prn_fields(line, delimiter)
                .map(|token| {
                    Self::parse_xyce_numeric_token(token).map_err(|err| {
                        format!(
                            "invalid numeric token '{}' on data line {}: {err}",
                            token, line_number
                        )
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
            if values.len() != columns.len() {
                return Err(format!(
                    "data line {} has {} values, expected {}",
                    line_number,
                    values.len(),
                    columns.len()
                ));
            }
            rows.push(values);
        }

        if rows.is_empty() {
            return Err("Xyce .prn table has no data rows".to_string());
        }

        Ok(XycePrnTable { columns, rows })
    }

    fn parse_prn_columns(line: &str, delimiter: XycePrnDelimiter) -> Result<Vec<String>, String> {
        match delimiter {
            XycePrnDelimiter::Whitespace => Self::split_prn_header_fields(line),
            XycePrnDelimiter::Comma => {
                Ok(line.split(',').map(str::trim).map(str::to_string).collect())
            }
        }
    }

    fn is_prn_header_line(line: &str) -> bool {
        Self::prn_header_delimiter(line).is_some()
    }

    fn prn_header_delimiter(line: &str) -> Option<XycePrnDelimiter> {
        if line
            .split(',')
            .next()
            .is_some_and(|token| Self::is_prn_metadata_header_token(token.trim()))
        {
            return Some(XycePrnDelimiter::Comma);
        }
        let whitespace_fields = Self::split_prn_header_fields(line).ok()?;
        if whitespace_fields
            .first()
            .is_some_and(|token| Self::is_prn_metadata_header_token(token))
        {
            return Some(XycePrnDelimiter::Whitespace);
        }
        let comma_fields = line.split(',').map(str::trim).collect::<Vec<_>>();
        if comma_fields.len() > 1
            && comma_fields
                .first()
                .is_some_and(|token| Self::looks_like_reference_probe_header(token))
        {
            return Some(XycePrnDelimiter::Comma);
        }
        if whitespace_fields.len() > 1
            && whitespace_fields
                .first()
                .is_some_and(|token| Self::looks_like_reference_probe_header(token))
        {
            return Some(XycePrnDelimiter::Whitespace);
        }
        None
    }

    fn is_prn_metadata_header_token(token: &str) -> bool {
        token.eq_ignore_ascii_case("index") || token.eq_ignore_ascii_case("stepnum")
    }

    fn reference_columns_are_compact_probe_table(reference: &XycePrnTable) -> bool {
        reference
            .columns
            .first()
            .is_some_and(|column| Self::looks_like_reference_probe_header(column))
    }

    fn looks_like_reference_probe_header(token: &str) -> bool {
        let normalized = Self::normalize_probe(token);
        normalized == "temp"
            || normalized == "time"
            || normalized == "freq"
            || normalized == "frequency"
            || normalized == "sweep"
            || normalized.starts_with("v(")
            || normalized.starts_with("i(")
            || normalized.starts_with("n(")
            || Self::compact_reference_probe_alias(&normalized).is_some()
    }

    fn split_prn_fields(
        line: &str,
        delimiter: XycePrnDelimiter,
    ) -> Box<dyn Iterator<Item = &str> + '_> {
        match delimiter {
            XycePrnDelimiter::Whitespace => Box::new(line.split_whitespace()),
            XycePrnDelimiter::Comma => Box::new(line.split(',').map(str::trim)),
        }
    }

    fn same_prn_columns(left: &[String], right: &[String]) -> bool {
        left.len() == right.len()
            && left
                .iter()
                .zip(right.iter())
                .all(|(left, right)| left.eq_ignore_ascii_case(right))
    }

    fn is_prn_separator_line(line: &str) -> bool {
        line.chars()
            .all(|ch| ch == '-' || ch == '=' || ch.is_whitespace())
    }

    fn is_prn_footer_line(line: &str) -> bool {
        let normalized = line.to_ascii_lowercase();
        normalized.starts_with("cpu time")
            || normalized.starts_with("total cpu time")
            || normalized.starts_with("current dynamic memory usage")
            || normalized.starts_with("dynamic memory limit")
    }

    fn static_prn_reference_path(&self, deck_path: &Path) -> Option<PathBuf> {
        self.static_output_reference_path(deck_path, "prn")
    }

    fn tran_gsfile_reference_path(deck_path: &Path) -> Option<PathBuf> {
        let candidate = deck_path.with_extension("cir.GSfile");
        candidate.is_file().then_some(candidate)
    }

    fn static_output_reference_path(&self, deck_path: &Path, extension: &str) -> Option<PathBuf> {
        let netlists_root = self.root.join("Netlists");
        let canonical = deck_path
            .canonicalize()
            .unwrap_or_else(|_| deck_path.to_path_buf());
        let relative = canonical
            .strip_prefix(&netlists_root)
            .or_else(|_| deck_path.strip_prefix(&netlists_root))
            .ok()?;
        let parent = relative.parent().unwrap_or_else(|| Path::new(""));
        let file_name = relative.file_name()?.to_string_lossy();
        Some(
            self.root
                .join("OutputData")
                .join(parent)
                .join(format!("{file_name}.{extension}")),
        )
    }

    fn collect_circuit_files(dir: &Path, out: &mut Vec<PathBuf>) {
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                Self::collect_circuit_files(&path, out);
            } else if path
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| ext.eq_ignore_ascii_case("cir"))
            {
                out.push(path);
            }
        }
    }

    fn section_for_relative_path(relative_path: &str) -> XyceDeckSection {
        if relative_path.starts_with("Netlists/") {
            XyceDeckSection::Netlists
        } else {
            XyceDeckSection::Other
        }
    }

    fn load_upstream_wrapper_decks(root: &Path) -> BTreeSet<String> {
        let manifest_path = root.join(HARNESS_MANIFEST_FILE);
        let Ok(content) = fs::read_to_string(manifest_path) else {
            return BTreeSet::new();
        };

        let mut decks = BTreeSet::new();
        for raw_line in content.lines() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let Some((path, contract)) = line.split_once('\t') else {
                continue;
            };
            if contract.trim() == REQUIRES_UPSTREAM_WRAPPER_CONTRACT {
                decks.insert(Self::normalize_manifest_key(path));
            }
        }
        decks
    }

    fn baseline_family_contract(&self, deck: &XyceDeck) -> Option<XyceBaselineFamilyContract> {
        self.subckt_family_contract(deck)
            .or_else(|| self.supernode_family_contract(deck))
    }

    fn subckt_family_contract(&self, deck: &XyceDeck) -> Option<XyceBaselineFamilyContract> {
        let relative_path = Self::normalize_manifest_key(&deck.relative_path);
        if !relative_path.starts_with("netlists/subckt/") {
            return None;
        }

        let file_name = deck.path.file_name()?.to_str()?;
        let parent = deck.path.parent()?;
        if self.requires_upstream_wrapper(&deck.relative_path)
            && let Some(family) = Self::parse_subckt_wrapper_file_name(file_name)
            && fs::metadata(&deck.path)
                .ok()
                .is_some_and(|metadata| metadata.len() == 0)
        {
            return self.subckt_family_contract_for(parent, &family, None);
        }

        let family = Self::parse_subckt_family_member_file_name(file_name)?;
        let wrapper_relative = format!("Netlists/SUBCKT/subckt_{family}.cir");
        if !self.requires_upstream_wrapper(&wrapper_relative) {
            return None;
        }
        self.subckt_family_contract_for(parent, &family, Some(deck.path.clone()))
    }

    fn subckt_family_contract_for(
        &self,
        parent: &Path,
        family: &str,
        target_path: Option<PathBuf>,
    ) -> Option<XyceBaselineFamilyContract> {
        let mut member_paths = Vec::new();
        for entry in fs::read_dir(parent).ok()?.flatten() {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let file_name = path.file_name().and_then(|name| name.to_str())?;
            if Self::parse_subckt_family_member_file_name(file_name)
                .is_some_and(|candidate| candidate.eq_ignore_ascii_case(family))
            {
                member_paths.push(path);
            }
        }
        member_paths.sort_by(|left, right| {
            left.file_name()
                .unwrap_or_default()
                .cmp(right.file_name().unwrap_or_default())
        });

        let baseline_path = parent.join(format!("subckt_{family}0.cir"));
        if !member_paths
            .iter()
            .any(|member| Self::same_path(member, &baseline_path))
        {
            return None;
        }

        Some(XyceBaselineFamilyContract {
            kind: XyceBaselineFamilyKind::Subckt,
            family: family.to_string(),
            baseline_path,
            member_paths,
            target_path,
        })
    }

    fn supernode_family_contract(&self, deck: &XyceDeck) -> Option<XyceBaselineFamilyContract> {
        let relative_path = Self::normalize_manifest_key(&deck.relative_path);
        if !relative_path.starts_with("netlists/supernode/") {
            return None;
        }

        let file_name = deck.path.file_name()?.to_str()?;
        let parent = deck.path.parent()?;
        if file_name.eq_ignore_ascii_case("supernode1.cir")
            && self.requires_upstream_wrapper(&deck.relative_path)
        {
            return self.supernode1_family_contract_for(parent, None);
        }

        if matches!(
            file_name.to_ascii_lowercase().as_str(),
            "supernode1a.cir" | "supernode1b.cir"
        ) {
            let wrapper_relative = "Netlists/SUPERNODE/supernode1.cir";
            if self.requires_upstream_wrapper(wrapper_relative) {
                return self.supernode1_family_contract_for(parent, Some(deck.path.clone()));
            }
        }

        None
    }

    fn supernode1_family_contract_for(
        &self,
        parent: &Path,
        target_path: Option<PathBuf>,
    ) -> Option<XyceBaselineFamilyContract> {
        let mut member_paths = Vec::new();
        for file_name in ["supernode1.cir", "supernode1a.cir", "supernode1b.cir"] {
            let path = parent.join(file_name);
            if !path.is_file() {
                return None;
            }
            member_paths.push(path);
        }

        Some(XyceBaselineFamilyContract {
            kind: XyceBaselineFamilyKind::Supernode,
            family: "supernode1".to_string(),
            baseline_path: parent.join("supernode1.cir"),
            member_paths,
            target_path,
        })
    }

    fn parse_subckt_wrapper_file_name(file_name: &str) -> Option<String> {
        let stem = file_name.strip_suffix(".cir")?;
        let family = stem.strip_prefix("subckt_")?;
        if !family.is_empty()
            && family
                .chars()
                .all(|ch| ch.is_ascii_alphabetic() || ch == '_')
        {
            Some(family.to_string())
        } else {
            None
        }
    }

    fn parse_subckt_family_member_file_name(file_name: &str) -> Option<String> {
        let stem = file_name.strip_suffix(".cir")?;
        let rest = stem.strip_prefix("subckt_")?;
        let digit_index = rest.find(|ch: char| ch.is_ascii_digit())?;
        let family = &rest[..digit_index];
        if family.is_empty() {
            return None;
        }
        let suffix = &rest[digit_index + 1..];
        if !matches!(suffix, "" | "_hs" | "_dup") {
            return None;
        }
        Some(family.to_string())
    }

    fn same_path(left: &Path, right: &Path) -> bool {
        let left = left.canonicalize().unwrap_or_else(|_| left.to_path_buf());
        let right = right.canonicalize().unwrap_or_else(|_| right.to_path_buf());
        left == right
    }

    fn normalize_manifest_key(path: &str) -> String {
        path.trim().replace('\\', "/").to_ascii_lowercase()
    }

    fn relative_key(&self, path: &Path) -> String {
        let canonical = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
        canonical
            .strip_prefix(&self.root)
            .or_else(|_| path.strip_prefix(&self.root))
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/")
    }

    fn display_path(&self, path: &Path) -> String {
        path.strip_prefix(&self.root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/")
    }

    fn deck_name(path: &Path) -> String {
        path.file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string()
    }

    fn contains_control_block(source: &str) -> bool {
        source.lines().any(|line| {
            let normalized = Self::strip_netlist_comment(line)
                .trim()
                .to_ascii_lowercase();
            normalized == ".control" || normalized == ".endc"
        })
    }

    fn source_has_op_analysis(source: &str) -> bool {
        Self::logical_netlist_lines(source).iter().any(|line| {
            let Some(command) = Self::strip_netlist_comment(line).split_whitespace().next() else {
                return false;
            };
            command.eq_ignore_ascii_case(".op")
        })
    }

    fn logical_netlist_lines(source: &str) -> Vec<String> {
        let mut lines: Vec<String> = Vec::new();
        for raw in source.lines() {
            let line = raw
                .split_once(';')
                .map(|(head, _)| head)
                .unwrap_or(raw)
                .trim_end();
            if line.trim().is_empty() {
                continue;
            }
            if Self::strip_netlist_comment(line).trim().is_empty() {
                continue;
            }
            if line.trim_start().starts_with('+') {
                if let Some(previous) = lines.last_mut() {
                    previous.push(' ');
                    previous.push_str(line.trim_start().trim_start_matches('+').trim_start());
                } else {
                    lines.push(line.to_string());
                }
            } else {
                lines.push(line.to_string());
            }
        }
        lines
    }

    fn strip_netlist_comment(line: &str) -> &str {
        let no_inline = line.split_once(';').map(|(head, _)| head).unwrap_or(line);
        let trimmed = no_inline.trim_start();
        if trimmed.starts_with('*') || trimmed.starts_with("//") {
            ""
        } else {
            no_inline
        }
    }

    fn normalize_probe(probe: &str) -> String {
        probe
            .chars()
            .filter(|ch| !ch.is_whitespace())
            .collect::<String>()
            .to_ascii_lowercase()
    }

    fn parse_voltage_probe(probe: &str) -> Option<XyceVoltageProbe> {
        let normalized = Self::normalize_probe(probe);
        let open_index = normalized.find('(')?;
        if !normalized.ends_with(')') {
            return None;
        }
        let accessor = XyceVoltageAccessor::from_function_name(&normalized[..open_index])?;
        let inner = &normalized[open_index + 1..normalized.len() - 1];
        if inner.is_empty() {
            return None;
        }
        let (node_pos, node_neg) = if let Some((a, b)) = inner.split_once(',') {
            (a.to_string(), Some(b.to_string()))
        } else {
            (inner.to_string(), None)
        };
        Some(XyceVoltageProbe {
            accessor,
            node_pos,
            node_neg,
        })
    }

    fn parse_tran_voltage_probe(probe: &str) -> Option<XyceVoltageProbe> {
        let normalized = Self::normalize_probe(probe);
        if !normalized.starts_with("n(") {
            return Self::parse_voltage_probe(&normalized);
        }
        if !normalized.ends_with(')') {
            return None;
        }
        let inner = &normalized[2..normalized.len() - 1];
        if inner.is_empty() || inner.contains(':') {
            return None;
        }
        let (node_pos, node_neg) = if let Some((a, b)) = inner.split_once(',') {
            (a.to_string(), Some(b.to_string()))
        } else {
            (inner.to_string(), None)
        };
        Some(XyceVoltageProbe {
            accessor: XyceVoltageAccessor::Value,
            node_pos,
            node_neg,
        })
    }

    fn parse_ac_voltage_probe(probe: &str) -> Option<XyceVoltageProbe> {
        let normalized = Self::normalize_probe(probe);
        if !normalized.starts_with("n(") {
            return Self::parse_voltage_probe(&normalized);
        }
        if !normalized.ends_with(')') {
            return None;
        }
        let inner = &normalized[2..normalized.len() - 1];
        if inner.is_empty() || inner.contains(':') {
            return None;
        }
        let (node_pos, node_neg) = if let Some((a, b)) = inner.split_once(',') {
            (a.to_string(), Some(b.to_string()))
        } else {
            (inner.to_string(), None)
        };
        Some(XyceVoltageProbe {
            accessor: XyceVoltageAccessor::Value,
            node_pos,
            node_neg,
        })
    }

    fn parse_current_probe(probe: &str) -> Option<String> {
        let normalized = Self::normalize_probe(probe);
        if !normalized.starts_with("i(") || !normalized.ends_with(')') {
            return None;
        }
        let inner = &normalized[2..normalized.len() - 1];
        (!inner.is_empty()).then(|| inner.to_string())
    }

    fn parse_ac_current_probe(probe: &str) -> Option<XyceAcCurrentProbe> {
        let normalized = Self::normalize_probe(probe);
        let open_index = normalized.find('(')?;
        if !normalized.ends_with(')') {
            return None;
        }
        let accessor = XyceCurrentAccessor::from_function_name(&normalized[..open_index])?;
        let inner = &normalized[open_index + 1..normalized.len() - 1];
        (!inner.is_empty()).then(|| XyceAcCurrentProbe {
            accessor,
            element_name: inner.to_string(),
        })
    }

    fn parse_power_probe(probe: &str) -> Option<String> {
        let normalized = Self::normalize_probe(probe);
        if !(normalized.starts_with("p(") || normalized.starts_with("w("))
            || !normalized.ends_with(')')
        {
            return None;
        }
        let inner = &normalized[2..normalized.len() - 1];
        (!inner.is_empty()).then(|| inner.to_string())
    }

    fn parse_lead_current_probe(probe: &str) -> Option<XyceLeadCurrentProbe> {
        let normalized = Self::normalize_probe(probe);
        for function in ["id", "ig", "is", "ib", "ic", "ie"] {
            let prefix = format!("{function}(");
            if !normalized.starts_with(&prefix) || !normalized.ends_with(')') {
                continue;
            }
            let inner = &normalized[prefix.len()..normalized.len() - 1];
            if inner.is_empty() {
                return None;
            }
            return Some(XyceLeadCurrentProbe {
                terminal: XyceLeadCurrentTerminal::from_function_name(function)?,
                element_name: inner.to_string(),
            });
        }
        None
    }

    fn parse_device_operating_point_probe(probe: &str) -> Option<(String, String)> {
        let normalized = Self::normalize_probe(probe);
        if !normalized.starts_with("n(") || !normalized.ends_with(')') {
            return None;
        }
        let inner = &normalized[2..normalized.len() - 1];
        let (element, parameter) = inner.rsplit_once(':')?;
        if element.is_empty() || parameter.is_empty() {
            return None;
        }
        Some((element.to_string(), parameter.to_string()))
    }

    fn parse_device_parameter_probe(probe: &str) -> Option<(String, String)> {
        let normalized = Self::normalize_probe(probe);
        let unwrapped = normalized
            .strip_prefix('{')
            .and_then(|value| value.strip_suffix('}'))
            .unwrap_or(&normalized);
        let (element, parameter) = unwrapped.rsplit_once(':')?;
        if element.is_empty() || parameter.is_empty() {
            return None;
        }
        if element
            .chars()
            .chain(parameter.chars())
            .any(|ch| matches!(ch, '(' | ')' | '+' | '-' | '*' | '/' | '^' | ','))
        {
            return None;
        }
        Some((element.to_string(), parameter.to_string()))
    }

    fn parse_bare_device_parameter_probe(probe: &str) -> Option<String> {
        let normalized = Self::normalize_probe(probe);
        let unwrapped = normalized
            .strip_prefix('{')
            .and_then(|value| value.strip_suffix('}'))
            .unwrap_or(&normalized);
        if unwrapped.is_empty()
            || unwrapped.starts_with(':')
            || unwrapped.ends_with(':')
            || unwrapped.contains("::")
            || unwrapped
                .chars()
                .any(|ch| matches!(ch, '(' | ')' | '+' | '-' | '*' | '/' | '^' | ',' | '='))
        {
            return None;
        }
        let first = unwrapped.chars().next()?;
        if !(first.is_ascii_alphabetic() || first == '_') {
            return None;
        }
        Some(unwrapped.to_string())
    }

    fn parse_scalar_parameter_probe(probe: &str) -> Option<String> {
        let normalized = Self::normalize_probe(probe);
        let unwrapped = normalized
            .strip_prefix('{')
            .and_then(|value| value.strip_suffix('}'))
            .unwrap_or(&normalized);
        if unwrapped.is_empty()
            || unwrapped.contains(':')
            || unwrapped.contains('(')
            || unwrapped.contains(')')
            || unwrapped
                .chars()
                .any(|ch| matches!(ch, '+' | '-' | '*' | '/' | '^' | ','))
        {
            return None;
        }
        let mut chars = unwrapped.chars();
        let first = chars.next()?;
        if !(first.is_ascii_alphabetic() || first == '_') {
            return None;
        }
        if chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '.') {
            Some(unwrapped.to_string())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prn_parser_accepts_preamble_and_repeated_page_headers() {
        let table = XyceTestRunner::parse_prn_table(
            r#"Circuit: metadata line
Date: metadata line

--------------------------------------------------------------------------------
Index   v-sweep         v(2)            v(1)
--------------------------------------------------------------------------------
0       0.000000e+00    0.000000e+00    2.000000e-01
1       2.000000e-02    2.000000e-02    2.000000e-01

Index   v-sweep         v(2)            v(1)
--------------------------------------------------------------------------------
2       4.000000e-02    4.000000e-02    2.000000e-01
CPU time since last call: 0.110 seconds.
Total CPU time: 0.110 seconds.
"#,
        )
        .expect("parser accepts Xyce page headers");

        assert_eq!(
            table.columns,
            ["Index", "v-sweep", "v(2)", "v(1)"]
                .into_iter()
                .map(str::to_string)
                .collect::<Vec<_>>()
        );
        assert_eq!(table.rows.len(), 3);
        assert_eq!(table.rows[2], vec![2.0, 0.04, 0.04, 0.2]);
    }

    #[test]
    fn prn_parser_accepts_comma_delimited_dc_output() {
        let table = XyceTestRunner::parse_prn_table(
            r#"Index,V(2),V(1),I(VDS)
0,0.00000000e+00,0.00000000e+00,0.00000000e+00
1,5.00000000e-02,0.00000000e+00,-3.05537186e-09
End of Xyce(TM) Simulation
"#,
        )
        .expect("parser accepts comma-delimited Xyce PRN output");

        assert_eq!(
            table.columns,
            ["Index", "V(2)", "V(1)", "I(VDS)"]
                .into_iter()
                .map(str::to_string)
                .collect::<Vec<_>>()
        );
        assert_eq!(table.rows.len(), 2);
        assert_eq!(table.rows[1], vec![1.0, 0.05, 0.0, -3.05537186e-9]);
    }

    #[test]
    fn prn_parser_accepts_trailing_commas_in_whitespace_fd_output() {
        let table = XyceTestRunner::parse_prn_table(
            r#"Index     FREQ           Re(V(1))         Im(V(1))
0	1.000000e+00,		-9.998421e+02,	1.256439e+01
1	1.258925e+00,		-9.997498e+02,	1.581616e+01
End of Xyce(TM) Simulation
"#,
        )
        .expect("parser accepts Xyce whitespace-delimited FD output with trailing commas");

        assert_eq!(
            table.columns,
            ["Index", "FREQ", "Re(V(1))", "Im(V(1))"]
                .into_iter()
                .map(str::to_string)
                .collect::<Vec<_>>()
        );
        assert_eq!(table.rows.len(), 2);
        assert_eq!(table.rows[0], vec![0.0, 1.0, -9.998421e2, 1.256439e1]);
    }

    #[test]
    fn prn_parser_preserves_braced_expression_headers() {
        let table = XyceTestRunner::parse_prn_table(
            r#"Index        TIME        {20.0 + V(30)}
0       0.00000000e+00   2.00000000e+01
1       5.00000000e-06   2.30901699e+01
End of Xyce(TM) Simulation
"#,
        )
        .expect("parser accepts whitespace-delimited braced expression headers");

        assert_eq!(
            table.columns,
            ["Index", "TIME", "{20.0 + V(30)}"]
                .into_iter()
                .map(str::to_string)
                .collect::<Vec<_>>()
        );
        assert_eq!(table.rows.len(), 2);
        assert_eq!(table.rows[1], vec![1.0, 5.0e-6, 23.0901699]);
    }

    #[test]
    fn raw_parser_accepts_ascii_repeated_plots() {
        let table = XyceTestRunner::parse_raw_table(
            br#"Title: RAW ASCII
Date: today
Plotname: Step 1
Flags: real
No. Variables: 3
No. Points: 2
Variables:
    0   sweep   voltage
    1   V(1)    voltage
    2   V1#branch current
Values:
0   0.00000000e+00
    0.00000000e+00
    0.00000000e+00

1   1.00000000e-01
    1.00000000e-01
    -1.00000000e-02

Plotname: Step 2
Flags: real
No. Variables: 3
No. Points: 2
Variables:
    0   sweep   voltage
    1   V(1)    voltage
    2   V1#branch current
Values:
0   0.00000000e+00
    0.00000000e+00
    0.00000000e+00

1   1.00000000e-01
    1.00000000e-01
    -1.00000000e-02
"#,
        )
        .expect("RAW ASCII parser accepts repeated plots");

        assert_eq!(
            table.columns,
            ["sweep", "V(1)", "V1#branch"]
                .into_iter()
                .map(str::to_string)
                .collect::<Vec<_>>()
        );
        assert_eq!(table.rows.len(), 4);
        assert_eq!(table.rows[3], vec![0.1, 0.1, -0.01]);
    }

    #[test]
    fn raw_parser_accepts_binary_real_payload() {
        let mut raw = b"Title: RAW binary\nVersion: test\nPlotname: DC\nFlags: real\nNo. Variables: 2\nNo. Points: 2\nVariables:\n\t0\tsweep\tvoltage\n\t1\tV(1)\tvoltage\nBinary:\n".to_vec();
        for value in [0.0_f64, 0.0, 0.1, 0.1] {
            raw.extend_from_slice(&value.to_le_bytes());
        }

        let table =
            XyceTestRunner::parse_raw_table(&raw).expect("RAW parser accepts binary payload");

        assert_eq!(
            table.columns,
            ["sweep", "V(1)"]
                .into_iter()
                .map(str::to_string)
                .collect::<Vec<_>>()
        );
        assert_eq!(table.rows, vec![vec![0.0, 0.0], vec![0.1, 0.1]]);
    }

    #[test]
    fn print_output_requests_preserve_grouped_probe_fields() {
        let requests = XyceTestRunner::print_output_requests(
            r#".PRINT TRAN V( 1 , 0 ) {20.0 + V(30)} I(VSENSE)"#,
            "TRAN",
        )
        .expect("grouped transient .PRINT probes parse");

        assert_eq!(
            requests,
            vec![XycePrintOutputRequest {
                format: None,
                file: None,
                probes: ["V( 1 , 0 )", "{20.0 + V(30)}", "I(VSENSE)"]
                    .into_iter()
                    .map(str::to_string)
                    .collect(),
            }]
        );

        let requests = XyceTestRunner::print_output_requests(
            r#".PRINT DC FORMAT = STD FILE = "out data.prn" V(1) {V(2) + 1.0}"#,
            "DC",
        )
        .expect("spaced .PRINT options and braced DC probes parse");

        assert_eq!(
            requests,
            vec![XycePrintOutputRequest {
                format: Some("STD".to_string()),
                file: Some("out data.prn".to_string()),
                probes: ["V(1)", "{V(2) + 1.0}"]
                    .into_iter()
                    .map(str::to_string)
                    .collect(),
            }]
        );
    }

    #[test]
    fn print_output_requests_join_continuations_across_comment_lines() {
        let requests = XyceTestRunner::print_output_requests(
            r#"
.PRINT AC
*+ V(1)
*+ V(2)
+ {0.001 + abs(VREAL - VR(1))}
+ {0.001 + abs(VIMAG - VI(1))}
"#,
            "AC",
        )
        .expect("comment-separated .PRINT AC continuations parse");

        assert_eq!(
            requests,
            vec![XycePrintOutputRequest {
                format: None,
                file: None,
                probes: [
                    "{0.001 + abs(VREAL - VR(1))}",
                    "{0.001 + abs(VIMAG - VI(1))}",
                ]
                .into_iter()
                .map(str::to_string)
                .collect(),
            }]
        );
    }

    #[test]
    fn blank_print_lines_are_ignored_and_side_outputs_aggregate() {
        let source = r#"
.PRINT TRAN V(1) V(2)
.PRINT TRAN
.PRINT TRAN FILE=blank.prn
.PRINT TRAN FILE=blank.prn V(3)
.PRINT TRAN FILE=blank.prn v(4) V(5)
"#;

        let primary = XyceTestRunner::single_tran_print_output_request(source)
            .expect("primary transient print survives inert blank .PRINT lines");
        assert_eq!(
            primary,
            XycePrintOutputRequest {
                format: None,
                file: None,
                probes: ["V(1)", "V(2)"].into_iter().map(str::to_string).collect(),
            }
        );

        let side_outputs = XyceTestRunner::prn_compatible_tran_side_output_requests(source)
            .expect("transient side-output requests aggregate by FILE target");
        assert_eq!(
            side_outputs,
            vec![XycePrintOutputRequest {
                format: None,
                file: Some("blank.prn".to_string()),
                probes: ["V(3)", "v(4)", "V(5)"]
                    .into_iter()
                    .map(str::to_string)
                    .collect(),
            }]
        );
    }

    #[test]
    fn transient_output_override_aggregates_primary_and_file_prints() {
        let source = r#"
.TRAN 0 1
.PRINT TRAN FORMAT=CSV V(1)
.PRINT TRAN FILE=nooverwriteFoo V(2)
"#;

        XyceTestRunner::validate_native_output_override_prn_tran_wrapper_contract(source)
            .expect("output override TRAN wrapper contract validates");
        let request = XyceTestRunner::output_override_print_output_request(source, "TRAN")
            .expect("output override request parses")
            .expect("output override has probes");

        assert_eq!(
            request,
            XycePrintOutputRequest {
                format: None,
                file: None,
                probes: ["V(1)", "V(2)"].into_iter().map(str::to_string).collect(),
            }
        );
    }

    #[test]
    fn transient_output_override_rejects_unrelated_print_analysis() {
        let source = r#"
.TRAN 0 1
.PRINT TRAN V(1)
.PRINT AC V(1)
"#;

        let err = XyceTestRunner::validate_native_output_override_prn_tran_wrapper_contract(source)
            .expect_err("output override TRAN contract should reject mixed print analyses");
        assert!(
            err.contains(".PRINT AC"),
            "unexpected validation error: {err}"
        );
    }

    #[test]
    fn reference_time_grid_uses_transient_prn_time_column() {
        let table = XycePrnTable {
            columns: ["Index", "TIME", "V(1)"]
                .into_iter()
                .map(str::to_string)
                .collect(),
            rows: vec![
                vec![0.0, 0.0, 1.0],
                vec![1.0, 1.0e-9, 2.0],
                vec![2.0, 4.0e-9, 3.0],
            ],
        };

        let grid = XyceTestRunner::reference_time_grid(&table)
            .expect("transient PRN time grid should parse");

        assert_eq!(grid, vec![0.0, 1.0e-9, 4.0e-9]);
    }

    #[test]
    fn wrapper_tran_analysis_extends_to_reference_stop() {
        let tran = XyceTranAnalysis {
            step: 1.0e-9,
            stop: 10.0e-9,
            start: None,
            max_step: None,
            uic: false,
        };

        let wrapper = XyceTestRunner::tran_analysis_for_reference_stop(
            XyceStaticTranContract::WrapperStatic,
            tran,
            &[0.0, 10.0e-9, 10.0e-6],
        );
        assert_eq!(wrapper.stop, 10.0e-6);

        let plain = XyceTestRunner::tran_analysis_for_reference_stop(
            XyceStaticTranContract::PlainStatic,
            tran,
            &[0.0, 10.0e-9, 10.0e-6],
        );
        assert_eq!(plain.stop, 10.0e-9);
    }

    #[test]
    fn reference_time_grid_accepts_stepnum_transient_prn_metadata() {
        let table = XycePrnTable {
            columns: ["STEPNUM", "Index", "TIME", "V(1)"]
                .into_iter()
                .map(str::to_string)
                .collect(),
            rows: vec![
                vec![2.0, 0.0, 0.0, 1.0],
                vec![2.0, 1.0, 1.0e-9, 2.0],
                vec![2.0, 2.0, 4.0e-9, 3.0],
            ],
        };

        let layout = XyceTestRunner::transient_reference_layout(&table)
            .expect("STEPNUM/Index/TIME layout should parse");
        let grid = XyceTestRunner::reference_time_grid(&table)
            .expect("transient PRN time grid should parse");

        assert_eq!(layout.stepnum_column, Some(0));
        assert_eq!(layout.index_column, Some(1));
        assert_eq!(layout.time_column, 2);
        assert_eq!(layout.data_column_offset, 3);
        assert_eq!(grid, vec![0.0, 1.0e-9, 4.0e-9]);
    }

    #[test]
    fn transient_reference_columns_accept_snapshot_extra_probes() {
        let table = XycePrnTable {
            columns: ["TIME", "{I(V1)}", "{V(1)}", "{V(2)}"]
                .into_iter()
                .map(str::to_string)
                .collect(),
            rows: vec![vec![0.0, 0.0, 0.0, 0.0]],
        };
        let print = XycePrintRequest {
            probes: vec!["V(1)".to_string(), "V(2)".to_string()],
        };
        let layout = XyceTestRunner::transient_reference_layout(&table)
            .expect("TIME-prefixed transient reference should parse");

        let data_columns =
            XyceTestRunner::reference_tran_data_columns(&table, &print, layout.data_column_offset)
                .expect("snapshot-added source current column should be accepted");

        assert_eq!(data_columns, vec!["{I(V1)}", "V(1)", "V(2)"]);
    }

    #[test]
    fn reference_column_matching_preserves_braced_expressions() {
        assert!(XyceTestRunner::reference_column_matches_probe(
            "{R0}", "{r0}"
        ));
        assert!(XyceTestRunner::reference_column_matches_probe(
            "{V(1)}", "V(1)"
        ));
        assert!(XyceTestRunner::reference_column_matches_probe(
            "V(0,3)", "V(GND,3)"
        ));
        assert!(XyceTestRunner::reference_column_matches_probe(
            "N(0)", "N(GND)"
        ));
    }

    #[test]
    fn split_transient_step_reference_validates_stepnum_metadata() {
        let table = XycePrnTable {
            columns: ["STEPNUM", "TIME", "V(1)"]
                .into_iter()
                .map(str::to_string)
                .collect(),
            rows: vec![
                vec![0.0, 0.0, 1.0],
                vec![0.0, 1.0e-9, 2.0],
                vec![1.0, 0.0, 3.0],
                vec![1.0, 1.0e-9, 4.0],
            ],
        };

        let steps = XyceTestRunner::split_transient_step_reference(&table, 2)
            .expect("STEPNUM/TIME transient reference should split");

        assert_eq!(steps.len(), 2);
        assert_eq!(
            steps[0].rows,
            vec![vec![0.0, 0.0, 1.0], vec![0.0, 1.0e-9, 2.0]]
        );
        assert_eq!(
            steps[1].rows,
            vec![vec![1.0, 0.0, 3.0], vec![1.0, 1.0e-9, 4.0]]
        );
    }

    #[test]
    fn reference_time_grid_rejects_nonmonotonic_time() {
        let table = XycePrnTable {
            columns: ["Index", "TIME", "V(1)"]
                .into_iter()
                .map(str::to_string)
                .collect(),
            rows: vec![
                vec![0.0, 0.0, 1.0],
                vec![1.0, 2.0e-9, 2.0],
                vec![2.0, 1.0e-9, 3.0],
            ],
        };

        let err = XyceTestRunner::reference_time_grid(&table)
            .expect_err("nonmonotonic transient PRN time grid must fail");

        assert!(err.contains("not monotonic"), "unexpected error: {err}");
    }

    #[test]
    fn transient_max_step_ignores_infeasible_source_transition_oversampling() {
        let netlist = Netlist::parse(
            "\
source transition envelope
VIN 1 0 PULSE(0 1 10U 1N 1N 30U)
.TRAN 1N 20U
.PRINT TRAN V(1)
.END
",
        )
        .expect("test netlist parses");
        let tran = XyceTranAnalysis {
            step: 1.0e-9,
            stop: 20.0e-6,
            start: None,
            max_step: None,
            uic: false,
        };
        let reference = XycePrnTable {
            columns: ["Index", "TIME", "V(1)"]
                .into_iter()
                .map(str::to_string)
                .collect(),
            rows: vec![
                vec![0.0, 0.0, 0.0],
                vec![1.0, 1.0e-9, 0.0],
                vec![2.0, 10.0000004e-6, 1.0],
                vec![3.0, 20.0e-6, 1.0],
            ],
        };

        let max_step =
            XyceTestRunner::transient_max_step_for_reference(&netlist, &tran, &reference)
                .expect("requested .TRAN step remains feasible");

        assert_eq!(max_step, 1.0e-9);
    }

    #[test]
    fn transient_max_step_does_not_treat_expensive_print_step_as_tmax() {
        let netlist = Netlist::parse(
            "\
long rc envelope
V1 in 0 0
R1 in out 1k
C1 out 0 40u IC=1
.TRAN 0.5U 400ms
.PRINT TRAN V(out)
.END
",
        )
        .expect("test netlist parses");
        let tran = XyceTranAnalysis {
            step: 0.5e-6,
            stop: 400.0e-3,
            start: None,
            max_step: None,
            uic: false,
        };
        let reference = XycePrnTable {
            columns: ["Index", "TIME", "V(out)"]
                .into_iter()
                .map(str::to_string)
                .collect(),
            rows: vec![
                vec![0.0, 0.0, 1.0],
                vec![1.0, 0.5e-6, 1.0],
                vec![2.0, 400.0e-3, 0.0],
            ],
        };

        let max_step =
            XyceTestRunner::transient_max_step_for_reference(&netlist, &tran, &reference)
                .expect("long transient falls back to bounded native step count");

        assert_eq!(max_step, 400.0e-6);
    }

    #[test]
    fn transient_max_step_rejects_oversized_flattened_work_envelope() {
        let mut deck = String::from("oversized hierarchical transient\nV1 in 0 SIN(0 1 1)\n");
        deck.push_str(".subckt ladder a b\n");
        for index in 0..40 {
            let pos = if index == 0 {
                "a".to_string()
            } else {
                format!("n{index}")
            };
            let neg = if index == 39 {
                "b".to_string()
            } else {
                format!("n{}", index + 1)
            };
            deck.push_str(&format!("R{index} {pos} {neg} 1\n"));
        }
        deck.push_str(".ends\n");
        for index in 0..100 {
            let input = if index == 0 {
                "in".to_string()
            } else {
                format!("out{}", index - 1)
            };
            deck.push_str(&format!("X{index} {input} out{index} ladder\n"));
        }
        deck.push_str(".tran 1m 100\n.print tran v(out99)\n.end\n");

        let netlist = Netlist::parse(&deck).expect("test netlist parses");
        let tran = XyceTranAnalysis {
            step: 1.0e-3,
            stop: 100.0,
            start: None,
            max_step: None,
            uic: false,
        };
        let reference = XycePrnTable {
            columns: ["Index", "TIME", "V(out99)"]
                .into_iter()
                .map(str::to_string)
                .collect(),
            rows: vec![
                vec![0.0, 0.0, 0.0],
                vec![1.0, 1.0e-3, 0.0],
                vec![2.0, 100.0, 0.0],
            ],
        };

        let err = XyceTestRunner::transient_max_step_for_reference(&netlist, &tran, &reference)
            .expect_err("oversized native transient work envelope should be unsupported");

        assert!(
            err.contains("transient harness execution envelope"),
            "unexpected error: {err}"
        );
        assert!(
            err.contains("native element-step"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn reference_columns_accept_primary_sweep_and_branch_labels() {
        let runner = XyceTestRunner::new(".", XyceRunnerConfig::default());
        let netlist = Netlist::default();
        let reference = XycePrnTable {
            columns: ["Index", "v-sweep", "v(2)", "vds_branch", "vmon1#branch"]
                .into_iter()
                .map(str::to_string)
                .collect(),
            rows: Vec::new(),
        };
        let print = XycePrintRequest {
            probes: ["v(2)", "i(vds)", "I(VMON1)"]
                .into_iter()
                .map(str::to_string)
                .collect(),
        };

        let columns = runner
            .reference_data_columns(&reference, &print, &netlist, 1, true)
            .expect("reference columns map to Xyce DC probes");

        assert!(matches!(
            &columns[0],
            XyceReferenceColumn::PrimarySweep { name } if name == "v-sweep"
        ));
        assert!(matches!(
            &columns[1],
            XyceReferenceColumn::Probe { name } if name == "v(2)"
        ));
        assert!(matches!(
            &columns[2],
            XyceReferenceColumn::Probe { name } if name == "i(vds)"
        ));
        assert!(matches!(
            &columns[3],
            XyceReferenceColumn::Probe { name } if name == "I(VMON1)"
        ));
    }

    #[test]
    fn reference_columns_omit_empty_wildcard_lead_current_probes() {
        let runner = XyceTestRunner::new(".", XyceRunnerConfig::default());
        let netlist = Netlist::default();
        let reference = XycePrnTable {
            columns: ["Index", "V(1)"].into_iter().map(str::to_string).collect(),
            rows: Vec::new(),
        };
        let print = XycePrintRequest {
            probes: ["V(1)", "ID(*)", "IG(*)", "IS(*)", "IB(*)", "IC(*)", "IE(*)"]
                .into_iter()
                .map(str::to_string)
                .collect(),
        };

        let columns = runner
            .reference_data_columns(&reference, &print, &netlist, 1, true)
            .expect("empty Xyce wildcard lead-current probes are omitted from PRN output");

        assert_eq!(columns.len(), 1);
        assert!(matches!(
            &columns[0],
            XyceReferenceColumn::Probe { name } if name == "V(1)"
        ));
        for probe in &print.probes[1..] {
            assert!(
                XyceTestRunner::dc_probe_is_omitted_empty_wildcard(probe, &netlist),
                "{probe} should be classified as an omitted empty wildcard"
            );
        }
    }

    #[test]
    fn uic_initial_current_source_probe_reports_zero() {
        let netlist = Netlist::parse(
            "uic current source probe\n\
             I1 1 0 10\n\
             R1 1 0 1\n\
             .tran 1m 2m uic\n\
             .end\n",
        )
        .expect("deck parses");
        let result = TransientResult {
            time: vec![0.0, 1.0e-3],
            voltages: vec![vec![0.0, -10.0]],
            branch_currents: Vec::new(),
            num_nodes: 1,
            node_names: vec!["1".to_string()],
            branch_names: Vec::new(),
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
        };

        let initial = XyceTestRunner::evaluate_tran_probe("i(i1)", &netlist, &result, 0.0)
            .expect("initial current-source probe evaluates");
        let running = XyceTestRunner::evaluate_tran_probe("i(i1)", &netlist, &result, 1.0e-3)
            .expect("running current-source probe evaluates");

        assert_eq!(initial, 0.0);
        assert_eq!(running, 10.0);
    }

    #[test]
    fn transient_resistor_power_probe_uses_recorded_branch_current() {
        let netlist = Netlist::parse(
            "transient resistor power probe\n\
             R1 out 0 100\n\
             .tran 1m 1m\n\
             .end\n",
        )
        .expect("deck parses");
        let result = TransientResult {
            time: vec![0.0, 1.0e-3],
            voltages: vec![vec![1.0, 3.0]],
            branch_currents: vec![vec![0.01, 0.03]],
            num_nodes: 1,
            node_names: vec!["out".to_string()],
            branch_names: vec!["R1".to_string()],
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
        };

        XyceTestRunner::validate_tran_probe("P(R1)", &netlist)
            .expect("resistor power is a supported transient probe");
        let power = XyceTestRunner::evaluate_tran_probe("P(R1)", &netlist, &result, 1.0e-3)
            .expect("resistor power evaluates from branch current and voltage drop");

        assert!((power - 0.09).abs() <= 1.0e-15, "power was {power}");
    }

    #[test]
    fn transient_voltage_source_power_probe_uses_recorded_branch_current() {
        let netlist = Netlist::parse(
            "transient voltage source power probe\n\
             V1 out 0 1\n\
             .tran 1m 1m\n\
             .end\n",
        )
        .expect("deck parses");
        let result = TransientResult {
            time: vec![0.0, 1.0e-3],
            voltages: vec![vec![2.0, 3.0]],
            branch_currents: vec![vec![0.5, -0.25]],
            num_nodes: 1,
            node_names: vec!["out".to_string()],
            branch_names: vec!["V1".to_string()],
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
        };

        XyceTestRunner::validate_tran_probe("P(V1)", &netlist)
            .expect("voltage-source power is a supported transient probe");
        XyceTestRunner::validate_tran_probe("W(V1)", &netlist)
            .expect("W is a Xyce power-probe alias");
        XyceTestRunner::validate_tran_probe("{W(V1)}", &netlist)
            .expect("braced W power expression validates through probe rewriting");

        let direct = XyceTestRunner::evaluate_tran_probe("P(V1)", &netlist, &result, 1.0e-3)
            .expect("P(V1) evaluates from branch current and voltage drop");
        let alias = XyceTestRunner::evaluate_tran_probe("W(V1)", &netlist, &result, 1.0e-3)
            .expect("W(V1) evaluates from branch current and voltage drop");
        let braced = XyceTestRunner::evaluate_tran_probe("{W(V1)}", &netlist, &result, 1.0e-3)
            .expect("{W(V1)} evaluates through expression rewriting");

        assert!((direct + 0.75).abs() <= 1.0e-15, "power was {direct}");
        assert_eq!(alias, direct);
        assert_eq!(braced, direct);
    }

    #[test]
    fn nested_device_then_param_step_rebinds_source_expression() {
        let netlist = Netlist::parse(
            "nested mixed step source binding\n\
             R1 a 0 10\n\
             Va a 0 SIN(0 {v_amplitude} 1)\n\
             .global_param v_amplitude=2\n\
             .step R1 10 11 1\n\
             .step v_amplitude 1 2 1\n\
             .end\n",
        )
        .expect("deck parses");
        let steps = XyceTestRunner::step_commands(&netlist).expect("steps parse");
        let engine = Engine::new(SimulationConfig::default());

        let runs = XyceTestRunner::nested_step_runs_for_commands(&engine, &netlist, &steps)
            .expect("nested steps expand");

        assert_eq!(runs.len(), 4);
        for (run, (expected_r, expected_amp)) in
            runs.iter()
                .zip([(10.0, 1.0), (11.0, 1.0), (10.0, 2.0), (11.0, 2.0)])
        {
            let resistor = run
                .netlist
                .elements
                .iter()
                .find(|element| element.name.eq_ignore_ascii_case("R1"))
                .expect("R1 is present");
            match &resistor.kind {
                ElementKind::Resistor { value, .. } => assert_eq!(*value, expected_r),
                other => panic!("unexpected R1 kind: {other:?}"),
            }

            let source = run
                .netlist
                .elements
                .iter()
                .find(|element| element.name.eq_ignore_ascii_case("Va"))
                .expect("Va is present");
            match &source.kind {
                ElementKind::VoltageSource(crate::netlist::SourceSpec::Sin {
                    amplitude, ..
                }) => assert_eq!(*amplitude, expected_amp),
                other => panic!("unexpected Va kind: {other:?}"),
            }
        }
    }

    #[test]
    fn comp_tolerances_apply_zero_tolerance_to_matching_print_probe() {
        let runner = XyceTestRunner::new(".", XyceRunnerConfig::default());
        let columns = vec![XyceReferenceColumn::Probe {
            name: "I(VIDS)".to_string(),
        }];
        let tolerances = runner
            .comp_tolerances("*COMP I(VIDS) zerotol=1.0e-11\n", &columns)
            .expect("COMP tolerance parses");
        let tolerance = *tolerances
            .get("i(vids)")
            .expect("COMP tolerance keyed by normalized probe");

        assert_eq!(tolerance.zero, Some(1.0e-11));
        assert!(
            runner.value_mismatch(1.0e-24, 1.8e-12, tolerance).is_none(),
            "zerotol should accept near-zero values on both sides"
        );
    }

    #[test]
    fn transient_comparison_accounts_for_printed_time_quantization() {
        let runner = XyceTestRunner::new(".", XyceRunnerConfig::default());
        let result = TransientResult {
            time: vec![2.99999996, 3.0],
            voltages: vec![vec![1.7336955179822779e-3, 0.0]],
            branch_currents: Vec::new(),
            num_nodes: 1,
            node_names: vec!["1".to_string()],
            branch_names: Vec::new(),
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
        };
        let tolerance = runner.default_comparison_tolerance("v(1)");
        let time_tolerance = XyceTestRunner::default_prn_time_quantization_tolerance(2.99999996);

        assert!(
            runner
                .transient_probe_matches_within_time_quantization(
                    "V(1)",
                    &Netlist::default(),
                    &result,
                    2.99999996,
                    1.52218075e-3,
                    1.7336955179822779e-3,
                    tolerance,
                    time_tolerance,
                )
                .expect("time-window comparison should evaluate"),
            "expected value falls inside the waveform interval induced by printed PRN time precision"
        );
    }

    #[test]
    fn transient_comparison_uses_local_samples_inside_prn_time_neighborhood() {
        let runner = XyceTestRunner::new(".", XyceRunnerConfig::default());
        let time = 2.96913385e-6;
        let result = TransientResult {
            time: vec![time, 2.96913387e-6],
            voltages: vec![vec![3.984804681e-4, 0.0]],
            branch_currents: Vec::new(),
            num_nodes: 1,
            node_names: vec!["3".to_string()],
            branch_names: Vec::new(),
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
        };
        let tolerance = runner
            .default_comparison_tolerance("v(3)")
            .with_relative(0.02)
            .with_absolute(1.0e-6);
        let time_tolerance = XyceTestRunner::default_prn_time_quantization_tolerance(time);

        assert!(
            runner
                .transient_probe_matches_within_time_quantization(
                    "V(3)",
                    &Netlist::default(),
                    &result,
                    time,
                    2.38339292e-4,
                    3.984804681e-4,
                    tolerance,
                    time_tolerance,
                )
                .expect("time-neighborhood comparison should evaluate"),
            "expected value falls inside adjacent PRN-rounded transition samples"
        );
    }

    #[test]
    fn transient_comparison_accepts_duplicate_reference_time_envelope() {
        let runner = XyceTestRunner::new(".", XyceRunnerConfig::default());
        let reference = XycePrnTable {
            columns: ["Index", "TIME", "V(1)"]
                .into_iter()
                .map(str::to_string)
                .collect(),
            rows: vec![vec![0.0, 1.0e-9, 1.0], vec![1.0, 1.0e-9, 3.0]],
        };
        let tolerance = runner.default_comparison_tolerance("v(1)");

        assert!(
            runner.transient_probe_matches_reference_time_neighborhood(
                &reference, 1, 0, 2, 2.0, tolerance, 0.0, 1.0,
            ),
            "actual value inside duplicate printed-time oracle envelope should be accepted"
        );
    }

    #[test]
    fn transient_comparison_rejects_outside_duplicate_reference_time_envelope() {
        let runner = XyceTestRunner::new(".", XyceRunnerConfig::default());
        let reference = XycePrnTable {
            columns: ["Index", "TIME", "V(1)"]
                .into_iter()
                .map(str::to_string)
                .collect(),
            rows: vec![vec![0.0, 1.0e-9, 1.0], vec![1.0, 1.0e-9, 3.0]],
        };
        let tolerance = runner.default_comparison_tolerance("v(1)");

        assert!(
            !runner.transient_probe_matches_reference_time_neighborhood(
                &reference, 1, 0, 2, 4.0, tolerance, 0.0, 1.0,
            ),
            "actual value outside duplicate printed-time oracle envelope should be rejected"
        );
    }

    #[test]
    fn transient_comparison_rejects_single_reference_time_row_neighborhood() {
        let runner = XyceTestRunner::new(".", XyceRunnerConfig::default());
        let reference = XycePrnTable {
            columns: ["Index", "TIME", "V(1)"]
                .into_iter()
                .map(str::to_string)
                .collect(),
            rows: vec![vec![0.0, 1.0e-9, 1.0], vec![1.0, 2.0e-9, 3.0]],
        };
        let tolerance = runner.default_comparison_tolerance("v(1)");

        assert!(
            !runner.transient_probe_matches_reference_time_neighborhood(
                &reference, 1, 0, 2, 2.0, tolerance, 0.0, 1.0,
            ),
            "a single oracle row must still fail normal comparison"
        );
    }

    #[test]
    fn transient_comparison_accepts_rounded_reference_time_neighborhood() {
        let runner = XyceTestRunner::new(".", XyceRunnerConfig::default());
        let time = 2.83500038e-8;
        let time_tolerance = XyceTestRunner::default_prn_time_quantization_tolerance(time);
        let reference = XycePrnTable {
            columns: ["Index", "TIME", "V(1)"]
                .into_iter()
                .map(str::to_string)
                .collect(),
            rows: vec![
                vec![0.0, time - time_tolerance, 0.017],
                vec![1.0, time, 0.019],
                vec![2.0, time + time_tolerance, 0.023],
            ],
        };
        let tolerance = runner.default_comparison_tolerance("v(1)");

        assert!(
            runner.transient_probe_matches_reference_time_neighborhood(
                &reference,
                1,
                1,
                2,
                0.020293594015632306,
                tolerance,
                time_tolerance,
                1.0,
            ),
            "actual value inside rounded printed-time oracle envelope should be accepted"
        );
    }

    #[test]
    fn transient_comparison_accepts_decimal_boundary_reference_time_neighborhood() {
        let runner = XyceTestRunner::new(".", XyceRunnerConfig::default());
        let time = 1.31654818e-7;
        let neighbor_time = 1.31654820e-7;
        let time_tolerance = XyceTestRunner::default_prn_time_quantization_tolerance(time);
        let reference = XycePrnTable {
            columns: ["Index", "TIME", "V(1)"]
                .into_iter()
                .map(str::to_string)
                .collect(),
            rows: vec![
                vec![0.0, time, 7.68303997e-3],
                vec![1.0, neighbor_time, 1.14131863e-2],
            ],
        };
        let tolerance = runner.default_comparison_tolerance("v(1)");

        assert!(
            runner.transient_probe_matches_reference_time_neighborhood(
                &reference,
                1,
                0,
                2,
                8.189634498984992e-3,
                tolerance,
                time_tolerance,
                1.0,
            ),
            "decimal timestamps at the PRN-neighborhood boundary should not fail due to binary roundoff"
        );
    }

    #[test]
    fn output_initial_interval_parser_accepts_spaced_assignment() {
        let interval = XyceTestRunner::output_initial_interval(
            "\
interval output
.OPTIONS OUTPUT INITIAL_INTERVAL = 0.5ms
.END
",
        )
        .expect("output option parses")
        .expect("initial interval is detected");

        assert!((interval - 5.0e-4).abs() <= 1.0e-15);
    }

    #[test]
    fn tran_print_time_scale_factor_parser_accepts_spaced_assignment() {
        let factor = XyceTestRunner::tran_print_time_scale_factor(
            "\
timescale output
.PRINT TRAN TIMESCALEFACTOR = 10 V(1)
.END
",
        )
        .expect("TRAN print time scale factor parses");

        assert!((factor - 10.0).abs() <= 1.0e-15);
    }

    #[test]
    fn tran_print_time_scale_factor_defaults_to_one() {
        let factor = XyceTestRunner::tran_print_time_scale_factor(
            "\
default output
.PRINT TRAN V(1)
.END
",
        )
        .expect("TRAN print line without a time scale factor parses");

        assert!((factor - 1.0).abs() <= 1.0e-15);
    }

    #[test]
    fn output_snapshots_option_is_detected_for_csv_guardrails() {
        assert!(
            XyceTestRunner::line_declares_output_snapshots(
                ".OPTIONS OUTPUT SNAPSHOTS=true INITIAL_INTERVAL=0.01"
            )
            .expect("snapshot option parses")
        );
        assert!(
            !XyceTestRunner::line_declares_output_snapshots(
                ".OPTIONS OUTPUT SNAPSHOTS = false INITIAL_INTERVAL=0.01"
            )
            .expect("disabled snapshot option parses")
        );
    }

    #[test]
    fn transient_output_interval_corridor_uses_adjacent_reference_rows() {
        let runner = XyceTestRunner::new(".", XyceRunnerConfig::default());
        let reference = XycePrnTable {
            columns: ["Index", "TIME", "V(1)"]
                .into_iter()
                .map(str::to_string)
                .collect(),
            rows: vec![
                vec![0.0, 0.0, 1.0],
                vec![1.0, 0.5, 0.75],
                vec![2.0, 1.0, 0.0],
            ],
        };
        let result = TransientResult {
            time: vec![0.0, 0.25, 0.5, 0.75, 1.0],
            voltages: vec![vec![1.0, 0.75, 0.5, 0.25, 0.0]],
            branch_currents: Vec::new(),
            num_nodes: 1,
            node_names: vec!["1".to_string()],
            branch_names: Vec::new(),
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
        };
        let tolerance = runner.default_comparison_tolerance("v(1)");

        assert!(
            runner
                .transient_probe_matches_output_interval_corridor(
                    "V(1)",
                    &Netlist::default(),
                    &result,
                    &reference,
                    1,
                    1,
                    0.75,
                    tolerance,
                    0.5,
                    1.0,
                )
                .expect("corridor comparison evaluates"),
            "expected interval-interpolated value should be accepted inside adjacent output rows"
        );
        assert!(
            !runner
                .transient_probe_matches_output_interval_corridor(
                    "V(1)",
                    &Netlist::default(),
                    &result,
                    &reference,
                    1,
                    1,
                    1.5,
                    tolerance,
                    0.5,
                    1.0,
                )
                .expect("corridor comparison evaluates"),
            "value outside the adjacent output-row envelope must not be accepted"
        );
    }

    #[test]
    fn transient_interpolation_preserves_subpicosecond_samples() {
        let times = [0.0, 5.0e-13, 1.0e-12];
        let values = [1.1, 1.5, 2.0];

        let exact = XyceTestRunner::interpolate_transient_waveform_at(&times, &values, 5.0e-13)
            .expect("exact sub-picosecond sample interpolates");
        let midpoint = XyceTestRunner::interpolate_transient_waveform_at(&times, &values, 2.5e-13)
            .expect("interior sub-picosecond sample interpolates");

        assert!(
            (exact - 1.5).abs() <= 1.0e-15,
            "exact sample collapsed to {exact:.12e}"
        );
        assert!(
            (midpoint - 1.3).abs() <= 1.0e-15,
            "midpoint sample collapsed to {midpoint:.12e}"
        );
    }

    #[test]
    fn comp_tolerances_parse_case_and_spaced_assignments() {
        let runner = XyceTestRunner::new(".", XyceRunnerConfig::default());
        let columns = vec![XyceReferenceColumn::Probe {
            name: "i(vds)".to_string(),
        }];
        let tolerances = runner
            .comp_tolerances(
                "*COMP i(vds) RELTOL = 0.25 abstol=1e-5 absdifftol=2e-5\n",
                &columns,
            )
            .expect("COMP tolerances parse");
        let tolerance = *tolerances
            .get("i(vds)")
            .expect("COMP tolerance keyed by normalized probe");

        assert_eq!(tolerance.relative, 0.25);
        assert_eq!(tolerance.absolute, 2.0e-5);
    }

    #[test]
    fn default_tolerances_are_unit_aware() {
        let runner = XyceTestRunner::new(".", XyceRunnerConfig::default());
        let voltage_tolerance = runner.default_comparison_tolerance("v(1)");
        let magnitude_tolerance = runner.default_comparison_tolerance("vm(1)");
        let phase_tolerance = runner.default_comparison_tolerance("vp(1)");
        let current_tolerance = runner.default_comparison_tolerance("i(v1)");
        let power_tolerance = runner.default_comparison_tolerance("p(l1)");
        let braced_power_tolerance = runner.default_comparison_tolerance("{p(l1)}");

        assert_eq!(voltage_tolerance.absolute, crate::constants::VNTOL);
        assert_eq!(magnitude_tolerance.absolute, crate::constants::VNTOL);
        assert_eq!(phase_tolerance.absolute, 1.0e-12);
        assert_eq!(current_tolerance.absolute, 1.0e-12);
        assert_eq!(current_tolerance.zero, Some(2.0e-12));
        assert_eq!(power_tolerance.absolute, 1.0e-9);
        assert_eq!(braced_power_tolerance.absolute, 1.0e-9);
        assert!(
            runner
                .value_mismatch(3.98095099e-12, 0.0, voltage_tolerance)
                .is_none(),
            "picovolt-scale voltage differences should be inside default VNTOL"
        );
        assert!(
            runner
                .value_mismatch(1.27073881e-10, 6.193804438227198e-12, power_tolerance)
                .is_none(),
            "sub-nanowatt derived power differences should stay inside default power tolerance"
        );
        assert!(
            runner
                .value_mismatch(3.98095099e-12, 0.0, current_tolerance)
                .is_some(),
            "current differences keep the stricter ITOL-scale default"
        );
        assert!(
            runner
                .value_mismatch(1.87829126e-12, -5.0e-15, current_tolerance)
                .is_none(),
            "near-zero current residuals inside both simulators' ABSTOL budgets should compare equal"
        );
    }

    #[test]
    fn source_directive_rejection_fails_closed_on_loca_continuation_options() {
        let source = ".options nonlin continuation=1\n.options loca stepper=natural\n";

        let err = XyceTestRunner::reject_unsupported_source_directives(source)
            .expect_err("LOCA continuation options are outside the native contract");

        assert!(
            err.contains("LOCA continuation"),
            "unexpected rejection message: {err}"
        );
    }

    #[test]
    fn voltage_probe_parses_xyce_dc_accessors() {
        assert_eq!(
            XyceTestRunner::parse_voltage_probe("VR(A,B)"),
            Some(XyceVoltageProbe {
                accessor: XyceVoltageAccessor::Real,
                node_pos: "a".to_string(),
                node_neg: Some("b".to_string()),
            })
        );
        assert_eq!(
            XyceTestRunner::parse_voltage_probe(" vm( OUT ) "),
            Some(XyceVoltageProbe {
                accessor: XyceVoltageAccessor::Magnitude,
                node_pos: "out".to_string(),
                node_neg: None,
            })
        );
        assert_eq!(
            XyceTestRunner::parse_voltage_probe("VP(n1,n2)")
                .expect("phase probe parses")
                .accessor,
            XyceVoltageAccessor::Phase
        );
        assert_eq!(
            XyceTestRunner::parse_voltage_probe("VDB(out)")
                .expect("dB probe parses")
                .accessor,
            XyceVoltageAccessor::Decibels
        );
        assert!(XyceTestRunner::parse_voltage_probe("VX(A)").is_none());
        assert_eq!(
            XyceTestRunner::parse_tran_voltage_probe("N(OUT)"),
            Some(XyceVoltageProbe {
                accessor: XyceVoltageAccessor::Value,
                node_pos: "out".to_string(),
                node_neg: None,
            })
        );
        assert!(
            XyceTestRunner::parse_tran_voltage_probe("N(M1:GM)").is_none(),
            "transient N(...) node alias must not steal device operating-point probes"
        );
    }

    #[test]
    fn ac_data_table_analysis_resolves_frequency_column() {
        let netlist = Netlist::parse(
            "\
xyce ac data
I1 1 0 AC 1
R1 1 0 1k
.AC DATA=eric
.DATA eric
+ FREQ unused
+ 1 7
+ 10 8
+ 100 9
.ENDDATA
.PRINT AC V(1)
.END
",
        )
        .expect(".AC DATA deck should parse");

        let ac = XyceTestRunner::single_ac_analysis(&netlist)
            .expect(".AC DATA table should resolve to a frequency list");

        assert_eq!(ac.frequencies(), vec![1.0, 10.0, 100.0]);
        let data_points = ac.data_points().expect(".AC DATA rows are retained");
        assert_eq!(data_points.len(), 3);
        assert_eq!(
            data_points[1].overrides,
            vec![("FREQ".to_string(), 10.0), ("unused".to_string(), 8.0)]
        );
    }

    #[test]
    fn ac_frequency_bindings_allow_freq_dependent_global_params_to_parse() {
        let source = "\
xyce ac freq binding
.GLOBAL_PARAM OMEGA={2*PI*FREQ}
R1 a 0 {OMEGA}
V1 a 0 AC 1
.AC DEC 1 10 100
.PRINT AC FREQ HERTZ R1:R
.END
";

        let err = XyceTestRunner::parse_xyce_netlist(source, Path::new("freq.cir"))
            .expect_err("plain parse should reject unbound FREQ");
        assert!(XyceTestRunner::parse_error_is_undefined_ac_frequency_symbol(&err));

        let rebound = XyceTestRunner::source_with_ac_frequency_bindings(source, 10.0);
        let netlist = XyceTestRunner::parse_xyce_netlist(&rebound, Path::new("freq.cir"))
            .expect("frequency-bound source parses");
        let ac = XyceTestRunner::single_ac_analysis(&netlist)
            .expect("AC analysis is still detected after frequency binding");

        let frequencies = ac.frequencies();
        assert_eq!(frequencies.len(), 2);
        assert!((frequencies[0] - 10.0).abs() < 1.0e-12);
        assert!((frequencies[1] - 100.0).abs() < 1.0e-10);
        assert!((netlist.params.get("FREQ").expect("FREQ bound") - 10.0).abs() < 1.0e-12);
        assert!(
            (netlist.params.get("OMEGA").expect("OMEGA evaluates")
                - 2.0 * std::f64::consts::PI * 10.0)
                .abs()
                < 1.0e-12
        );
    }

    #[test]
    fn ac_probe_evaluates_node_alias_and_current_accessors() {
        let netlist = Netlist::parse(
            "\
ac accessor probes
V1 a 0 AC 1
R1 a b 1
R2 b 0 2
.AC DEC 1 1 10
.PRINT AC N(B) IR(V1) II(V1) IM(V1) IP(V1) IDB(V1)
.END
",
        )
        .expect("AC accessor deck parses");
        let result = AcResult {
            frequency: 1.0,
            node_names: vec!["a".to_string(), "b".to_string()],
            branch_names: vec!["v1".to_string()],
            voltages: vec![Complex64::new(1.0, 0.0), Complex64::new(3.0, 4.0)],
            currents: vec![Complex64::new(-3.0, -4.0)],
        };

        assert_eq!(
            XyceTestRunner::parse_ac_voltage_probe("N(B)"),
            Some(XyceVoltageProbe {
                accessor: XyceVoltageAccessor::Value,
                node_pos: "b".to_string(),
                node_neg: None,
            })
        );
        assert!(
            XyceTestRunner::parse_ac_voltage_probe("N(M1:GM)").is_none(),
            "AC N(...) voltage alias must not steal device operating-point probes"
        );
        assert_eq!(
            XyceTestRunner::parse_ac_current_probe("IDB(V1)"),
            Some(XyceAcCurrentProbe {
                accessor: XyceCurrentAccessor::Decibels,
                element_name: "v1".to_string(),
            })
        );

        let n_b = XyceTestRunner::evaluate_ac_complex_probe("N(B)", &netlist, &result)
            .expect("N(B) evaluates as an AC voltage alias");
        assert_eq!(n_b, Complex64::new(3.0, 4.0));
        assert_eq!(
            XyceTestRunner::evaluate_ac_probe("IR(V1)", &netlist, &result, false)
                .expect("real current accessor evaluates"),
            -3.0
        );
        assert_eq!(
            XyceTestRunner::evaluate_ac_probe("II(V1)", &netlist, &result, false)
                .expect("imaginary current accessor evaluates"),
            -4.0
        );
        assert_eq!(
            XyceTestRunner::evaluate_ac_probe("IM(V1)", &netlist, &result, false)
                .expect("magnitude current accessor evaluates"),
            5.0
        );
        assert!(
            (XyceTestRunner::evaluate_ac_probe("IP(V1)", &netlist, &result, false)
                .expect("phase current accessor evaluates")
                + 126.86989764584402)
                .abs()
                < 1.0e-12
        );
        assert!(
            (XyceTestRunner::evaluate_ac_probe("IP(V1)", &netlist, &result, true)
                .expect("radian phase current accessor evaluates")
                + 2.214297435588181)
                .abs()
                < 1.0e-12
        );
        assert!(
            (XyceTestRunner::evaluate_ac_probe("IDB(V1)", &netlist, &result, false)
                .expect("decibel current accessor evaluates")
                - 13.979400086720377)
                .abs()
                < 1.0e-12
        );
        assert!(
            XyceTestRunner::validate_ac_complex_probe("IDB(V1)", &netlist)
                .expect_err("scalar current accessor is not a complex probe")
                .contains("complex I")
        );
    }

    #[test]
    fn ac_print_expression_evaluates_voltage_component_accessors() {
        let netlist = Netlist::parse(
            "\
ac expression accessor probes
V1 a 0 AC 1
R1 a 0 1
.AC DEC 1 1 10
.PRINT AC {0.001 + abs(2.0 - VR(A)) + abs(0.25 - VI(A))}
.END
",
        )
        .expect("AC expression accessor deck parses");
        let result = AcResult {
            frequency: 1.0,
            node_names: vec!["a".to_string()],
            branch_names: vec!["v1".to_string()],
            voltages: vec![Complex64::new(2.0, 0.25)],
            currents: vec![Complex64::new(-2.0, -0.25)],
        };
        let probe = "{0.001 + abs(2.0 - VR(A)) + abs(0.25 - VI(A))}";

        XyceTestRunner::validate_ac_probe(probe, &netlist)
            .expect("AC expression accepts scalar voltage component accessors");
        assert!(
            (XyceTestRunner::evaluate_ac_probe(probe, &netlist, &result, false)
                .expect("AC expression evaluates scalar voltage component accessors")
                - 0.001)
                .abs()
                < 1.0e-12
        );
    }

    #[test]
    fn ac_probe_evaluates_independent_source_ac_parameters() {
        let netlist = Netlist::parse(
            "\
ac source parameter probes
I1 a 0 AC 2 45
R1 a 0 1k
.AC DEC 1 1 10
.PRINT AC {I1:ACMAG} {I1:ACPHASE}
.END
",
        )
        .expect("AC source parameter deck parses");
        let result = AcResult {
            frequency: 1.0,
            node_names: vec!["a".to_string()],
            branch_names: Vec::new(),
            voltages: vec![Complex64::new(0.0, 0.0)],
            currents: Vec::new(),
        };

        XyceTestRunner::validate_ac_probe("{I1:ACMAG}", &netlist)
            .expect("AC source magnitude parameter validates");
        XyceTestRunner::validate_ac_probe("{I1:ACPHASE}", &netlist)
            .expect("AC source phase parameter validates");
        assert_eq!(
            XyceTestRunner::evaluate_ac_probe("{I1:ACMAG}", &netlist, &result, false)
                .expect("AC source magnitude parameter evaluates"),
            2.0
        );
        assert!(
            (XyceTestRunner::evaluate_ac_probe("{I1:ACPHASE}", &netlist, &result, false)
                .expect("AC source phase parameter evaluates")
                - 45.0)
                .abs()
                < 1.0e-12
        );
    }

    #[test]
    fn ac_phase_output_radians_option_is_detected() {
        assert!(XyceTestRunner::source_requests_ac_phase_output_radians(
            ".OPTIONS OUTPUT PHASE_OUTPUT_RADIANS=TRUE\n"
        ));
        assert!(!XyceTestRunner::source_requests_ac_phase_output_radians(
            ".OPTIONS OUTPUT PHASE_OUTPUT_RADIANS=TRUE\n\
             .OPTIONS OUTPUT PHASE_OUTPUT_RADIANS=FALSE\n"
        ));
    }

    #[test]
    fn ac_csv_contract_selects_csv_oracle_extension() {
        assert_eq!(
            XyceTestRunner::static_ac_contract_for_print_format(true, Some("CSV"))
                .expect("CSV wrapper contract is supported"),
            XyceStaticAcContract::WrapperCsv
        );
        assert_eq!(
            XyceStaticAcContract::WrapperCsv.reference_extension(),
            "FD.csv"
        );
        assert_eq!(
            XyceTestRunner::static_ac_contract_for_print_format(false, Some("CSV"))
                .expect("plain CSV contract is supported"),
            XyceStaticAcContract::PlainCsv
        );
    }

    #[test]
    fn ac_probe_contract_selects_csd_oracle_extension() {
        assert_eq!(
            XyceTestRunner::static_ac_contract_for_print_format(true, Some("PROBE"))
                .expect("PROBE wrapper contract is supported"),
            XyceStaticAcContract::WrapperCsd
        );
        assert_eq!(
            XyceStaticAcContract::WrapperCsd.reference_extension(),
            "csd"
        );
        assert_eq!(
            XyceTestRunner::static_ac_contract_for_print_format(false, Some("PROBE"))
                .expect("plain PROBE contract is supported"),
            XyceStaticAcContract::PlainCsd
        );
    }

    #[test]
    fn tran_probe_contract_selects_csd_oracle_extension() {
        assert_eq!(
            XyceTestRunner::static_tran_contract_for_print_format(true, Some("PROBE"))
                .expect("PROBE wrapper contract is supported"),
            XyceStaticTranContract::WrapperCsd
        );
        assert_eq!(
            XyceStaticTranContract::WrapperCsd.reference_extension(),
            "csd"
        );
        assert_eq!(
            XyceTestRunner::static_tran_contract_for_print_format(false, Some("PROBE"))
                .expect("plain PROBE contract is supported"),
            XyceStaticTranContract::PlainCsd
        );
    }

    #[test]
    fn tran_csv_contract_selects_csv_oracle_extension() {
        assert_eq!(
            XyceTestRunner::static_tran_contract_for_print_format(true, Some("CSV"))
                .expect("CSV wrapper contract is supported"),
            XyceStaticTranContract::WrapperCsv
        );
        assert_eq!(
            XyceStaticTranContract::WrapperCsv.reference_extension(),
            "csv"
        );
        assert_eq!(
            XyceTestRunner::static_tran_contract_for_print_format(false, Some("CSV"))
                .expect("plain CSV contract is supported"),
            XyceStaticTranContract::PlainCsv
        );
        let snapshots_source = "\
snapshot CSV
.OPTIONS OUTPUT SNAPSHOTS=true INITIAL_INTERVAL=0.01
.PRINT TRAN FORMAT=CSV V(1)
.TRAN 0 1
.END
";
        XyceTestRunner::validate_native_static_csv_tran_wrapper_contract(snapshots_source)
            .expect("snapshot CSV validates through the native CSV wrapper contract");
    }

    #[test]
    fn tran_csv_wrapper_classifier_accepts_primary_csv_output() {
        let source = "\
CSV transient
V1 1 0 SIN(0 1 1)
R1 1 2 1
R2 2 0 1
.OPTIONS OUTPUT INITIAL_INTERVAL=0.01
.PRINT TRAN FORMAT= CSV V(1) V(2)
.TRAN 0 1
.END
";

        XyceTestRunner::validate_native_static_csv_tran_wrapper_contract(source)
            .expect("primary transient CSV wrapper contract validates");
        assert_eq!(
            XyceTestRunner::native_static_prn_tran_wrapper_contract(
                Path::new("tran-csv.cir"),
                "Netlists/Output/TRAN/tran-csv.cir",
                source,
                true,
            ),
            Some(XyceStaticTranContract::WrapperCsv)
        );
    }

    #[test]
    fn tran_wrapper_accepts_primary_probe_format_prn_oracle_when_checked_prn_exists() {
        let source = "\
wrapper-origin hspice probe transient print with prn oracle
VA 1 0 PULSE(0 1 0 1e-3 1e-3 5e-3 1s)
R1 1 2 10
C1 2 0 1u
.TRAN 10u 10ms 0 UIC
.PRINT TRAN FORMAT=PROBE V(1) V(2)
.END
";

        XyceTestRunner::validate_native_static_prn_tran_wrapper_contract(source)
            .expect_err("strict transient PRN wrapper contract keeps PROBE as non-PRN");
        XyceTestRunner::validate_native_static_prn_tran_wrapper_contract_with_format_mode(
            source, true,
        )
        .expect("wrapper transient PRN contract can normalize primary PROBE with a PRN oracle");
        assert_eq!(
            XyceTestRunner::native_static_prn_tran_wrapper_contract(
                Path::new("subcircuit_node_delineation.cir"),
                "Netlists/XDM/HSPICE/OTHER_PARSING/subcircuit_node_delineation.cir",
                source,
                true,
            ),
            Some(XyceStaticTranContract::WrapperStatic)
        );
        assert_eq!(
            XyceTestRunner::native_static_prn_tran_wrapper_contract(
                Path::new("subcircuit_node_delineation.cir"),
                "Netlists/XDM/HSPICE/OTHER_PARSING/subcircuit_node_delineation.cir",
                source,
                false,
            ),
            None
        );
        assert_eq!(
            XyceTestRunner::static_tran_contract_for_print_format(true, Some("PROBE"))
                .expect("generic wrapper PROBE contract resolves"),
            XyceStaticTranContract::WrapperCsd,
            "generic PROBE output remains CSDF; only the wrapper-origin PRN classifier normalizes this case"
        );
    }

    #[test]
    fn dc_csv_contract_selects_csv_oracle_extension() {
        assert_eq!(
            XyceTestRunner::static_dc_contract_for_print_format(true, Some("CSV"))
                .expect("CSV wrapper contract is supported"),
            XyceStaticDcContract::WrapperCsv
        );
        assert_eq!(
            XyceStaticDcContract::WrapperCsv.reference_extension(),
            "csv"
        );
        assert_eq!(
            XyceTestRunner::static_dc_contract_for_print_format(false, Some("CSV"))
                .expect("plain CSV contract is supported"),
            XyceStaticDcContract::PlainCsv
        );
    }

    #[test]
    fn dc_probe_contract_selects_csd_oracle_extension() {
        assert_eq!(
            XyceTestRunner::static_dc_contract_for_print_format(true, Some("PROBE"))
                .expect("PROBE wrapper contract is supported"),
            XyceStaticDcContract::WrapperCsd
        );
        assert_eq!(
            XyceStaticDcContract::WrapperCsd.reference_extension(),
            "csd"
        );
        assert_eq!(
            XyceTestRunner::static_dc_contract_for_print_format(false, Some("PROBE"))
                .expect("plain PROBE contract is supported"),
            XyceStaticDcContract::PlainCsd
        );
        assert!(!XyceTestRunner::dc_print_format_is_prn_compatible("PROBE"));
    }

    #[test]
    fn csd_parser_reads_multisection_probe_tables() {
        let table = XyceTestRunner::parse_csd_table(
            "\
#H
SOURCE='Xyce' VERSION='7.0'
COMPLEXVALUES='NO' NODES='2'
#N
'V(1)' 'I(V1)'
#C 0.00000000e+00 2
0.00000000e+00:1 -1.00000000e-01:2
#;
#H
SOURCE='Xyce' VERSION='7.0'
COMPLEXVALUES='NO' NODES='2'
#N
'V(1)' 'I(V1)'
#C 1.00000000e+00 2
1.00000000e+00:1 -2.00000000e-01:2
#;
",
        )
        .expect("CSDF table parses");

        assert_eq!(table.columns, vec!["V(1)", "I(V1)"]);
        assert_eq!(table.rows.len(), 2);
        assert_eq!(table.rows[0], vec![0.0, -0.1]);
        assert_eq!(table.rows[1], vec![1.0, -0.2]);
    }

    #[test]
    fn tran_csd_parser_injects_time_column() {
        let table = XyceTestRunner::parse_tran_csd_table(
            "\
#H
SOURCE='Xyce' VERSION='7.0'
COMPLEXVALUES='NO' SWEEPVAR='Time'
#N
'V(1)' 'I(V1)'
#C 2.00000000e-01 2
2.00000000e-01:1 -2.00000000e-01:2
#;
",
        )
        .expect("transient CSDF table parses");

        assert_eq!(table.columns, vec!["TIME", "V(1)", "I(V1)"]);
        assert_eq!(table.rows, vec![vec![0.2, 0.2, -0.2]]);
    }

    #[test]
    fn transient_prn_reference_parser_accepts_legacy_probe_content() {
        let table = XyceTestRunner::parse_tran_prn_or_legacy_probe_table(
            "\
#H
SOURCE='Xyce' VERSION='1.1'
COMPLEXVALUES='NO' SWEEPVAR='Time'
#N
'V(1)' 'V(2)'
#C 1.00000000e-06 2
5.00000000e-01:1 2.50000000e-01:2
#;
",
        )
        .expect("legacy FORMAT=PROBE content can be carried in a .prn oracle");

        assert_eq!(table.columns, vec!["TIME", "V(1)", "V(2)"]);
        assert_eq!(table.rows, vec![vec![1.0e-6, 0.5, 0.25]]);
    }

    #[test]
    fn ac_csd_parser_expands_complex_probe_values() {
        let table = XyceTestRunner::parse_ac_csd_table(
            "\
#H
SOURCE='Xyce' VERSION='7.0'
COMPLEXVALUES='YES' SWEEPVAR='FREQ'
#N
'V(B)' 'VR(B)' 'I(V1)' 'VM(B)'
#C 1.00000000e+02 4
1.00000000e+00/2.00000000e+00:1 1.00000000e+00/0.00000000e+00:2
-3.00000000e+00/4.00000000e+00:3 2.23606798e+00/0.00000000e+00:4
#;
",
        )
        .expect("AC CSDF table parses");

        assert_eq!(
            table.columns,
            vec![
                "FREQ",
                "Re(V(B))",
                "Im(V(B))",
                "VR(B)",
                "Re(I(V1))",
                "Im(I(V1))",
                "VM(B)"
            ]
        );
        assert_eq!(table.rows.len(), 1);
        assert_eq!(
            table.rows[0],
            vec![100.0, 1.0, 2.0, 1.0, -3.0, 4.0, 2.23606798]
        );
    }

    #[test]
    fn dc_file_only_wrapper_detects_prn_side_output_contract() {
        let source = "\
R1 1 0 10
V1 1 0 DC 0
.PRINT DC FILE=out1.prn V(1)
.PRINT DC FILE=out2.prn V(1) I(V1)
.DC V1 0 1 1
.END
";

        assert!(XyceTestRunner::is_native_file_only_prn_wrapper_candidate(
            "Netlists/Output/DC/dc-multiprn.cir",
            source
        ));
        XyceTestRunner::validate_file_only_prn_wrapper_source(source)
            .expect("PRN-compatible file-only DC output validates");
    }

    #[test]
    fn ac_probe_evaluates_static_device_parameter() {
        let netlist = Netlist::parse(
            "\
ac device parameter output
V1 a 0 AC 1
R1 a b 2
R2 b 0 3
.AC DEC 1 1 10
.PRINT AC R1:R
.END
",
        )
        .expect("AC device parameter deck parses");
        let result = AcResult {
            frequency: 1.0,
            node_names: vec!["a".to_string(), "b".to_string()],
            branch_names: Vec::new(),
            voltages: vec![Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
            currents: Vec::new(),
        };

        XyceTestRunner::validate_ac_probe("R1:R", &netlist)
            .expect("static resistor parameter validates for AC output");
        assert_eq!(
            XyceTestRunner::evaluate_ac_probe("R1:R", &netlist, &result, false)
                .expect("static resistor parameter evaluates for AC output"),
            2.0
        );
    }

    #[test]
    fn hierarchical_node_lookup_resolves_nested_sibling_subcircuit_definition() {
        let netlist = Netlist::parse(
            "\
nested sibling subckt probe
X2 4 5 IC_SubSubckt
.SUBCKT IC_Subckt in out
R1 in mid 10
C1 mid out 1u
.ENDS
.SUBCKT IC_SubSubckt in out
R1 in a 1
X1 a b IC_Subckt
R2 b out 1
.ENDS
.END
",
        )
        .expect("test deck parses");

        let candidates = XyceTestRunner::node_lookup_candidates(&netlist, "X2:X1:out");

        assert!(
            candidates
                .iter()
                .any(|candidate| candidate.eq_ignore_ascii_case("X2.B")),
            "nested sibling subcircuit probe should resolve to flattened interface node, got {candidates:?}"
        );
    }

    #[test]
    fn native_transient_contract_accepts_multi_element_passive_subcircuit() {
        let netlist = Netlist::parse(
            "\
multi-element passive subckt
V1 1 0 PULSE(0 1 0 1n 1n 1u)
X1 1 2 RC
R2 2 0 10
.SUBCKT RC in out
R1 in mid 10
C1 mid out 1u
.ENDS
.TRAN 0 1u
.PRINT TRAN V(2)
.END
",
        )
        .expect("test deck parses");

        XyceTestRunner::validate_native_transient_contract(&netlist)
            .expect("flattened multi-element passive subcircuit should be supported");
    }

    #[test]
    fn device_parameter_probe_splits_on_rightmost_colon() {
        assert_eq!(
            XyceTestRunner::parse_device_parameter_probe("{Xtest:Xtest2:Rinside:R}"),
            Some(("xtest:xtest2:rinside".to_string(), "r".to_string()))
        );
    }

    #[test]
    fn print_expression_rewrites_bare_device_parameter_tokens() {
        let mut call_value = |call: &str| match XyceTestRunner::normalize_probe(call).as_str() {
            "r1:r" => Ok(2.0),
            "l1:l" => Ok(3.0e-3),
            other => Err(format!("unexpected probe {other}")),
        };
        let value = XyceTestRunner::evaluate_print_expression_with_probe_calls(
            "{R1:R*L1:L}",
            crate::netlist::ParamContext::default(),
            &mut call_value,
        )
        .expect("device-parameter expression evaluates");

        assert!((value - 6.0e-3).abs() < 1.0e-15, "value {value}");
    }

    #[test]
    fn lead_current_probe_parses_xyce_terminal_accessors() {
        assert_eq!(
            XyceTestRunner::parse_lead_current_probe("ID(Mfoo)"),
            Some(XyceLeadCurrentProbe {
                terminal: XyceLeadCurrentTerminal::Drain,
                element_name: "mfoo".to_string(),
            })
        );
        assert_eq!(
            XyceTestRunner::parse_lead_current_probe(" ib( X1:M1 ) "),
            Some(XyceLeadCurrentProbe {
                terminal: XyceLeadCurrentTerminal::Bulk,
                element_name: "x1:m1".to_string(),
            })
        );
        assert!(XyceTestRunner::parse_lead_current_probe("I(V1)").is_none());
        assert_eq!(XyceLeadCurrentTerminal::Drain.op_parameter(), Some("id"));
        assert_eq!(XyceLeadCurrentTerminal::Source.op_parameter(), Some("is"));
        assert_eq!(XyceLeadCurrentTerminal::Gate.op_parameter(), None);
    }

    #[test]
    fn print_expression_evaluates_xyce_lead_current_probe_calls() {
        let mut context = crate::netlist::ParamContext::new();
        context.set("scale", 2.0);
        let mut call_value = |call: &str| match XyceTestRunner::normalize_probe(call).as_str() {
            "id(m1)" => Ok(3.0),
            "i(vsense)" => Ok(5.0),
            other => Err(format!("unexpected probe {other}")),
        };

        let value = XyceTestRunner::evaluate_print_expression_with_probe_calls(
            "ID(M1) * scale + I(VSENSE)",
            context,
            &mut call_value,
        )
        .expect("lead-current probe expression evaluates");

        assert_eq!(value, 11.0);
    }

    #[test]
    fn print_expression_evaluates_xyce_voltage_accessor_probe_calls() {
        let context = crate::netlist::ParamContext::new();
        let mut call_value = |call: &str| match XyceTestRunner::normalize_probe(call).as_str() {
            "vr(a)" => Ok(2.0),
            "vi(a)" => Ok(0.25),
            "vm(a)" => Ok(2.0),
            "vp(a)" => Ok(180.0),
            other => Err(format!("unexpected probe {other}")),
        };

        let value = XyceTestRunner::evaluate_print_expression_with_probe_calls(
            "VR(A) + VI(A) + VM(A) + VP(A)",
            context,
            &mut call_value,
        )
        .expect("voltage-accessor probe expression evaluates");

        assert_eq!(value, 184.25);
    }

    #[test]
    fn ac_print_validation_accepts_complex_parameter_expression() {
        let source = "complex ac expression\n\
             Isrc 1 0 AC 1 0\n\
             R1 1 0 1e3\n\
             C1 1 0 2e-6\n\
             .param r0={log10(-1)}\n\
             .param r1={m(r0)}\n\
             .AC DEC 10 1 1e5\n\
             .print ac v(1) {r0} {r1}\n\
             .END\n";
        let netlist = XyceTestRunner::parse_xyce_netlist(source, Path::new("test.cir"))
            .expect("complex AC expression deck parses");
        let print =
            XyceTestRunner::single_ac_print_request(source).expect("AC print request parses");

        assert_eq!(
            print.probes,
            vec!["v(1)".to_string(), "{r0}".to_string(), "{r1}".to_string()]
        );
        XyceTestRunner::validate_ac_probe("{r0}", &netlist)
            .expect("braced complex parameter expression validates");
        XyceTestRunner::validate_ac_probe("r0", &netlist)
            .expect("reference-normalized complex parameter expression validates");
        XyceTestRunner::validate_ac_probe("{r1}", &netlist)
            .expect("AC parameter expression wins over same-named resistor");
    }

    #[test]
    fn print_request_keeps_continued_probes_after_inline_comments() {
        let source = "continued print comments\n\
             .param r0={3.0+2.0J}\n\
             .param r1={m(r0)}\n\
             .print ac\n\
             + v(1)          ; complex voltage\n\
             + {r0}          ; complex expression\n\
             + {re(r0)}      ; real projection\n\
             + {v(1)/r1}     ; expression with probe call\n\
             .END\n";

        let print =
            XyceTestRunner::single_ac_print_request(source).expect("AC print request parses");

        assert_eq!(
            print.probes,
            vec![
                "v(1)".to_string(),
                "{r0}".to_string(),
                "{re(r0)}".to_string(),
                "{v(1)/r1}".to_string(),
            ]
        );
    }

    #[test]
    fn ac_reference_columns_match_braced_complex_print_expression() {
        let source = "complex ac expression\n\
             .print ac\n\
             + v(1)          ; complex voltage\n\
             + {r0}          ; complex expression\n\
             + {0.1+r2}      ; complex expression with literal offset\n\
             .END\n";
        let print =
            XyceTestRunner::single_ac_print_request(source).expect("AC print request parses");
        let reference = XycePrnTable {
            columns: vec![
                "Index".to_string(),
                "FREQ".to_string(),
                "Re(V(1))".to_string(),
                "Im(V(1))".to_string(),
                "Re({R0})".to_string(),
                "Im({R0})".to_string(),
                "Re({0.1+R2})".to_string(),
                "Im({0.1+R2})".to_string(),
            ],
            rows: Vec::new(),
        };

        let columns = XyceTestRunner::reference_ac_data_columns(&reference, &print, 2)
            .expect("complex reference headers map to AC print expressions");

        assert_eq!(columns.len(), 6);
        assert_eq!(columns[2].probe_name(), "{r0}");
        assert_eq!(columns[4].probe_name(), "{0.1+r2}");
    }

    #[test]
    fn dc_complex_param_wrapper_accepts_unlabeled_source_tail() {
        let source = "\
testing of complex expressions in parameters and on the .PRINT line
Vsrc 1 0 1 0
R1 1 0 1e3
C1 1 0 2e-6
.param r0={3.0+2.0J}
.param r1={m(r0)}
.param r2={sqrt(-1.00000)}
.param r3={re(0.1+r2)}
.param r4={img(r2)}
.DC Vsrc 1 1 1
.print DC
+ v(1)
+ {r0}
+ {re(r0)}
+ {img(r0)}
+ {r1}
+ {v(1)/r0}
+ {v(1)/r1}
+ {ph(v(1)/r1)}
+ {db(v(1))}
+ {r(v(1))}
+ {r3}
+ {r4}
.END
";

        XyceTestRunner::validate_plain_static_dc_prn_wrapper_source(source)
            .expect("plain static DC wrapper source validates");
        let print = XyceTestRunner::single_dc_print_request(source)
            .expect("single DC print request parses");
        let netlist = XyceTestRunner::parse_xyce_netlist(source, Path::new("test4.cir"))
            .expect("test4 deck parses");
        assert!(
            !XyceTestRunner::braced_expression_is_atomic_real_probe("r1", &netlist),
            "braced scalar parameter r1 must not be shadowed by resistor R1"
        );
        XyceTestRunner::validate_plain_static_dc_prn_wrapper_netlist(&netlist)
            .expect("plain static DC netlist validates");
        let dc = XyceTestRunner::single_dc_sweep(&netlist).expect("single DC sweep parses");
        XyceTestRunner::validate_static_dc_contract(&netlist, &dc, &print)
            .expect("complex parameter DC probes validate");
    }

    #[test]
    fn plain_static_dc_wrapper_accepts_primary_probe_format_prn_oracle() {
        let source = "\
wrapper-origin hspice probe print with prn oracle
VA IN 0 DC 0
E1 1 0 IN 0 2
R1 1 2 10
R2 2 0 10
.DC VA 0 10 1
.PRINT DC FORMAT=PROBE v(in) v(2)
.END
";

        XyceTestRunner::validate_plain_static_dc_prn_wrapper_source(source)
            .expect("plain static DC wrapper source accepts primary FORMAT=PROBE");
        assert_eq!(
            XyceTestRunner::static_dc_contract_for_print_format(true, Some("PROBE"))
                .expect("generic wrapper PROBE contract resolves"),
            XyceStaticDcContract::WrapperCsd,
            "generic PROBE output remains CSDF; only the wrapper-origin PRN source validator normalizes this case"
        );
    }

    #[test]
    fn plain_static_dc_wrapper_accepts_native_legacy_diode_model() {
        let source = "\
plain static diode dc
VIN 1 0 DC 5V
R1 1 2 2K
D1 3 0 DMOD
VMON 2 3 0
.MODEL DMOD D (IS=100FA)
.DC VIN 5 5
.PRINT DC V(1) I(VMON) V(3)
.END
";

        XyceTestRunner::validate_plain_static_dc_prn_wrapper_source(source)
            .expect("plain static DC wrapper source accepts legacy diode model type");
        let netlist = XyceTestRunner::parse_xyce_netlist(source, Path::new("bug204.cir"))
            .expect("legacy diode DC deck parses");
        XyceTestRunner::validate_plain_static_dc_prn_wrapper_netlist(&netlist)
            .expect("plain static DC netlist accepts native legacy diode model");
    }

    #[test]
    fn plain_static_dc_wrapper_rejects_advanced_diode_model_level() {
        let source = "\
plain static advanced diode dc
VIN 1 0 DC 5V
D1 1 0 DMOD
.MODEL DMOD D (LEVEL=200 IS=1e-14)
.DC VIN 5 5
.PRINT DC V(1)
.END
";

        XyceTestRunner::validate_plain_static_dc_prn_wrapper_source(source)
            .expect("source-level model type validation accepts diode syntax");
        let netlist = XyceTestRunner::parse_xyce_netlist(source, Path::new("advanced_diode.cir"))
            .expect("advanced diode model deck parses at netlist level");
        let err = XyceTestRunner::validate_plain_static_dc_prn_wrapper_netlist(&netlist)
            .expect_err("advanced diode levels stay outside the native plain DC wrapper contract");
        assert!(
            err.contains("advanced diode model"),
            "unexpected validation error: {err}"
        );
    }

    #[test]
    fn no_output_dc_wrapper_accepts_dc_without_print() {
        let source = "\
no output dc
R1 1 0 10
V1 1 0 DC 0
.DC V1 0 10 1
.END
";

        XyceTestRunner::validate_no_output_dc_wrapper_source(source)
            .expect("no-output DC source validates");
        assert!(XyceTestRunner::is_native_no_output_dc_wrapper_candidate(
            "Netlists/Output/DC/dc-noprn.cir",
            source
        ));
    }

    #[test]
    fn no_output_dc_wrapper_rejects_printed_deck() {
        let source = "\
printed dc
R1 1 0 10
V1 1 0 DC 0
.DC V1 0 10 1
.PRINT DC V(1)
.END
";

        let err = XyceTestRunner::validate_no_output_dc_wrapper_source(source)
            .expect_err("printed deck is not no-output");
        assert!(
            err.contains(".PRINT") || err.contains(".print"),
            "unexpected validation error: {err}"
        );
    }

    #[test]
    fn expected_missing_inductor_value_error_accepts_intended_diagnostic() {
        let source = "\
missing inductor value
V1 1 0 DC 1
L1 1 0
.TRAN 1n 1u
.PRINT TRAN V(1)
.END
";

        assert!(
            XyceTestRunner::is_expected_missing_inductor_value_error_deck(
                "Netlists/INDUCTOR/ErrorMessageTest.cir",
                "L value missing from instance line\n"
            )
        );
        XyceTestRunner::validate_expected_missing_inductor_value_error_source(
            source,
            Path::new("ErrorMessageTest.cir"),
        )
        .expect("missing inductor value diagnostic validates");
    }

    #[test]
    fn expected_missing_inductor_value_error_rejects_valid_inductor() {
        let source = "\
valid inductor
V1 1 0 DC 1
L1 1 0 1u
.TRAN 1n 1u
.PRINT TRAN V(1)
.END
";

        let err = XyceTestRunner::validate_expected_missing_inductor_value_error_source(
            source,
            Path::new("valid_inductor.cir"),
        )
        .expect_err("valid inductor deck must not satisfy expected-error contract");
        assert!(
            err.contains("parsed successfully"),
            "unexpected validation error: {err}"
        );
    }

    #[test]
    fn dc_data_tables_materialize_device_value_rows() {
        let source = "\
dc data table rows
V1 1 0 1
R1 1 2 1
R2 2 0 1
.DATA R1table
+ r1
+ 1
+ 2
.ENDDATA
.DATA R2table
+ r2
+ 3
+ 4
.ENDDATA
.DC DATA=R1table
.DC DATA=R2table
.PRINT DC {R1:R} {R2:R} V(2)
.END
";

        let netlist = XyceTestRunner::parse_xyce_netlist(source, Path::new("dc_data.cir"))
            .expect(".DC DATA deck parses");
        let dc_data = XyceTestRunner::dc_data_sweep_for_source(source, &netlist)
            .expect(".DC DATA tables validate")
            .expect(".DC DATA sweep is detected");
        assert_eq!(dc_data.rows.len(), 2);

        let engine = Engine::new(SimulationConfig {
            spice_dialect: SpiceDialect::Xyce,
            ..SimulationConfig::default()
        });
        let row_netlist =
            XyceTestRunner::materialize_dc_data_row_netlist(&engine, &netlist, &dc_data.rows[1])
                .expect("second .DC DATA row materializes");

        let r1 = row_netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case("R1"))
            .expect("R1 retained");
        let r2 = row_netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case("R2"))
            .expect("R2 retained");
        assert!(
            matches!(&r1.kind, ElementKind::Resistor { value, .. } if (*value - 2.0).abs() <= Value::EPSILON)
        );
        assert!(
            matches!(&r2.kind, ElementKind::Resistor { value, .. } if (*value - 4.0).abs() <= Value::EPSILON)
        );
    }

    #[test]
    fn dc_data_tables_reject_mismatched_table_lengths() {
        let source = "\
dc data mismatched rows
V1 1 0 1
R1 1 2 1
R2 2 0 1
.DATA R1table
+ r1
+ 1
+ 2
.ENDDATA
.DATA R2table
+ r2
+ 3
.ENDDATA
.DC DATA=R1table
.DC DATA=R2table
.PRINT DC {R1:R} {R2:R} V(2)
.END
";

        let netlist = XyceTestRunner::parse_xyce_netlist(source, Path::new("dc_data.cir"))
            .expect(".DC DATA deck parses");
        let err = XyceTestRunner::dc_data_sweep_for_source(source, &netlist)
            .expect_err("mismatched TABLE-style .DC DATA row counts must reject");
        assert!(
            err.contains("expected 2"),
            "unexpected validation error: {err}"
        );
    }

    #[test]
    fn ac_reference_columns_preserve_current_magnitude_accessor() {
        let source = "current accessor output\n\
             .print ac I(V1) IM(V1)\n\
             .END\n";
        let print =
            XyceTestRunner::single_ac_print_request(source).expect("AC print request parses");
        let reference = XycePrnTable {
            columns: vec![
                "Index".to_string(),
                "FREQ".to_string(),
                "Re(I(V1))".to_string(),
                "Im(I(V1))".to_string(),
                "IM(V1)".to_string(),
            ],
            rows: Vec::new(),
        };

        let columns = XyceTestRunner::reference_ac_data_columns(&reference, &print, 2)
            .expect("current component and magnitude headers map to AC print probes");

        assert_eq!(columns.len(), 3);
        assert_eq!(columns[0].probe_name(), "i(v1)");
        assert_eq!(columns[1].probe_name(), "i(v1)");
        assert_eq!(columns[2].probe_name(), "IM(V1)");
    }

    #[test]
    fn print_ddx_evaluates_probe_derivative_at_operating_point() {
        let mut context = crate::netlist::ParamContext::new();
        context.set("SCALAR", 2.0);
        let mut call_value = |call: &str| match XyceTestRunner::normalize_probe(call).as_str() {
            "v(cntl)" => Ok(2.0),
            "v(2)" => Ok(5.0 / 3.0),
            other => Err(format!("unexpected probe {other}")),
        };

        let derivative = XyceTestRunner::evaluate_print_expression_with_probe_calls(
            "ddx(V(2)/(1.0+scalar*V(cntl)),v(cntl))",
            context,
            &mut call_value,
        )
        .expect("DDX print expression evaluates");

        assert!((derivative + 2.0 / 15.0).abs() < 1.0e-9);
    }

    #[test]
    fn xyce_runner_engine_uses_xyce_device_dialect() {
        let runner = XyceTestRunner::new(".", XyceRunnerConfig::default());
        let engine = runner.create_dc_engine();

        assert_eq!(engine.config().spice_dialect, SpiceDialect::Xyce);
        assert_eq!(
            engine.config().integration_method,
            crate::analysis::IntegrationMethod::TrapGear
        );
        assert_eq!(
            engine.config().resolved_jfet_level2_model(),
            crate::engine::JfetLevel2Model::XyceModifiedShockley
        );
    }

    #[test]
    fn subckt_family_names_match_upstream_wrapper_globs() {
        assert_eq!(
            XyceTestRunner::parse_subckt_wrapper_file_name("subckt_b.cir").as_deref(),
            Some("b")
        );
        assert_eq!(
            XyceTestRunner::parse_subckt_family_member_file_name("subckt_b0.cir").as_deref(),
            Some("b")
        );
        assert_eq!(
            XyceTestRunner::parse_subckt_family_member_file_name("subckt_b2_hs.cir").as_deref(),
            Some("b")
        );
        assert_eq!(
            XyceTestRunner::parse_subckt_family_member_file_name("subckt_a1_dup.cir").as_deref(),
            Some("a")
        );
        assert!(
            XyceTestRunner::parse_subckt_family_member_file_name("subckt_a2_dup_error.cir")
                .is_none(),
            "error-checking wrapper decks are not part of the diff-against-baseline family glob"
        );
        assert!(
            XyceTestRunner::parse_subckt_family_member_file_name("subckt_j1.cir").is_some(),
            "matching the filename glob is separate from requiring a sibling wrapper manifest entry"
        );
    }
}
