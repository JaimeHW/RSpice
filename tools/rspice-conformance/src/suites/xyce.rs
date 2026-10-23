//! Xyce regression corpus runner.
//!
//! The upstream Xyce suite is vendored as a runtime corpus. RSpice keeps the
//! netlists, reference output data, and licensing/provenance files, but omits
//! upstream platform-specific harness scripts. Regression execution is
//! Rust-native: every retained `.cir` deck is discovered and reported. Decks
//! are numerically executed only when their checked-in, relational, or
//! explicitly qualified generated-oracle contract can be reproduced without
//! the removed platform harness.

// The replay helpers below take one parameter per independent input a deck's
// contract can vary: the plan, the print request, the step runs and their
// references, the abort signal, whether the time grid is locked, and which
// comparison mode applies. Bundling them into a struct would hide at the call
// site which knob a given deck is exercising, which is the one thing these
// helpers exist to make legible.
use rspice_core::abort_signal::AbortSignal;
use rspice_core::analysis::ac::ac_sweep_frequencies;
use rspice_core::analysis::{AcResult, AcSensitivityOutput};
use rspice_core::config::ExpressionDialect;
use rspice_core::engine::{
    ConvergenceConfig, DcSweepPointResult, SimulationConfig, SimulationError, SpiceDialect,
    StepPlanLimits, TransientCheckpoint, TransientResult, TransientStartupMode,
    XyceTraInterpolation, extract_ac_value, extract_dc_value,
};
use rspice_core::expr::{
    BinaryOp, CompiledExpr, Context, Expr, Vm, compile, parse_expression_strict,
};
use rspice_core::io::{
    XycePrnFooter, XycePrnLimits, XycePrnScientificStyle, XycePrnTable, format_xyce_prn_scientific,
    serialize_legacy_compact_prn_for_comparison, serialize_xyce_prn_sequence,
};
use rspice_core::netlist::expr::ComplexValue as ExprComplexValue;
use rspice_core::netlist::expr::is_real as expression_value_is_real;
use rspice_core::netlist::expr::{
    behavioral_expression_references_unbound_frequency, prepare_behavioral_expression,
};
use rspice_core::netlist::{
    AnalysisCommand, DcSecondSweep, DcSweepMode, DeviceInitialConditionError,
    DeviceInitialConditionSource, DuplicateSubcircuitPortBindingError, ElementKind,
    ElementProvenance, FreqVariation, MissingSubcircuitEndsBoundary, MissingSubcircuitEndsError,
    Netlist, NetlistParseOptions, OutputAnalysisKind, OutputDirectiveKind, OutputExpressionIssue,
    OutputSymbolKind, ParameterRedefinitionPolicy, ParametricValue, ParseError, PrintDelimiter,
    SealedSourceBundle, SealedSourceEdge, StartupDiagnosticCode, StartupDiagnosticStage,
    StartupDirectiveKind, StartupDirectiveScope, StatisticalParamMode, StepCommand, StepSweep,
    StepTarget, SubcircuitDef, UnresolvedSubcircuitParameterError,
    XYCE_DEFAULT_ZERO_RESISTANCE_TOL, flatten_netlist, flatten_netlist_with_models,
    flatten_netlist_with_models_with_abort, validate_output_expressions_with_abort,
    validate_output_symbols, validate_output_symbols_with_abort,
};
use rspice_core::numerics::integration::TransientLteReference;
use rspice_core::{Complex64, Engine, Value};
use sha2::{Digest as _, Sha256};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;
// The per-contract test modules build deadline fixtures from a fixed offset.
#[cfg(test)]
use std::time::Duration;

const EXPECTED_UNSUPPORTED_MARKER: &str = "EXPECTED_UNSUPPORTED:";
const UPSTREAM_EXCLUDED_MARKER: &str = "UPSTREAM_EXCLUDED:";
const HARNESS_MANIFEST_FILE: &str = "RSPICE-HARNESS-MANIFEST.tsv";
const UPSTREAM_EXCLUSIONS_MANIFEST_FILE: &str = "RSPICE-UPSTREAM-EXCLUSIONS.tsv";
const UPSTREAM_EXCLUSIONS_SCHEMA_VERSION: &str = "1";
const UPSTREAM_EXCLUSIONS_SOURCE_COMMIT: &str = "80115a9277c0ddb3409acceb3d4e745fd11cddd4";
const UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE: &str = "3e34bfaafa890cb2e4457137b6a0e325c8c1e87d";
const MAX_UPSTREAM_EXCLUSIONS_MANIFEST_BYTES: usize = 1_048_576;
const UPSTREAM_EXCLUDED_DISPOSITION: &str = "upstream_excluded";
const RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION: &str = "rspice_independently_qualified";
const REQUIRES_UPSTREAM_WRAPPER_CONTRACT: &str = "requires_upstream_wrapper";
const MAX_NATIVE_TRAN_ORACLE_STEPS: f64 = 250_000.0;
const MAX_NATIVE_TRAN_TARGET_COMPACT_DEVICE_STEPS: f64 = 10_000.0;
const MAX_NATIVE_TRAN_ELEMENT_STEPS: f64 = 250_000_000.0;
const MAX_NATIVE_TRAN_COMPACT_DEVICE_STEPS: f64 = 2_500_000.0;
const MAX_NATIVE_TRAN_NODE_SOLVE_STEPS: f64 = 2_500_000_000.0;
// Eager harness expansion clones one netlist per run. The checked-in corpus
// currently peaks at 21 STEP rows, so 4096 preserves more than 195x headroom
// while failing closed before an adversarial deck can allocate unbounded runs.
const XYCE_STEP_PLAN_MAX_RUNS: usize = 4_096;
const XYCE_STEP_PLAN_MAX_DIMENSIONS: usize = 256;
const XYCE_STEP_PLAN_MAX_BINDINGS_PER_RUN: usize = 16_384;
const XYCE_STEP_PLAN_MAX_STORED_VALUES: usize = 4_000_000;

fn xyce_step_plan_limits() -> StepPlanLimits {
    StepPlanLimits::new(
        XYCE_STEP_PLAN_MAX_RUNS,
        XYCE_STEP_PLAN_MAX_DIMENSIONS,
        XYCE_STEP_PLAN_MAX_BINDINGS_PER_RUN,
        XYCE_STEP_PLAN_MAX_STORED_VALUES,
    )
}
const TRAN_ORACLE_STEPS_PER_SOURCE_PERIOD: f64 = 64.0;
const TRAN_ORACLE_STEPS_PER_SOURCE_TRANSITION: f64 = 200.0;
const XYCE_DEFAULT_PRN_FRACTION_DIGITS: f64 = 8.0;
const XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION: usize = 8;
const XYCE_MAX_IEEE754_PRN_SCIENTIFIC_PRECISION: usize = 16;

#[derive(Debug)]
struct StatefulTranPrintExpression {
    program: CompiledExpr,
    vm: Vm,
}

/// Cached names used while validating a static DC print request.
///
/// A large Xyce `.PRINT DC` table can contain one probe per device.  Looking
/// up every branch-current probe by scanning the complete element list turns
/// validation into O(probes × elements), which is needlessly expensive for
/// otherwise straightforward circuits. Keep the common top-level lookup
/// eager and materialize the flattened fallback only if a probe actually
/// needs it.
#[derive(Debug, Default)]
struct XyceDcProbeIndex {
    diode_names: HashSet<String>,
    recorded_branch_names: HashSet<String>,
    flattened_diode_names: Option<HashSet<String>>,
    flattened_recorded_branch_names: Option<HashSet<String>>,
    flattened_lookup_attempted: bool,
}
const PRN_TIME_NEIGHBOR_HALF_ULPS: f64 = 4.0;
const XYCE_VERIFY_DEFAULT_RELATIVE_TOLERANCE: f64 = 1.0e-2;
const XYCE_VERIFY_DEFAULT_ABSOLUTE_TOLERANCE: f64 = 1.0e-12;
const XYCE_VERIFY_DEFAULT_ZERO_TOLERANCE: f64 = 1.0e-12;
const XYCE_VERIFY_DEFAULT_ABSOLUTE_DIFFERENCE_TOLERANCE: f64 = 1.0e-12;
const XYCE_VERIFY_COMP_NO_PRINTED_PROBE: &str =
    "Xyce integrated-RMS *COMP contract has no directive for a printed probe";
const XYCE_SELF_VERIFIED_MEASUREMENT_TRAN_CONTRACT: &str = "self_verified_measure_tran";
const XYCE_NONLINEAR_CORE_MODEL_STEP_WRAPPER_CONTRACT: &str =
    "nonlinear_core_model_step_reference_wrapper";
const XYCE_NONLINEAR_CORE_MODEL_STEP_BASELINE_CONTRACT: &str =
    "nonlinear_core_model_step_reference_baseline";
const XYCE_BUG1190_MUTUAL_INDUCTOR_WRAPPER_CONTRACT: &str =
    "bug1190_mutual_inductor_parameter_alias_wrapper";
const XYCE_BUG1190_MUTUAL_INDUCTOR_BASELINE_CONTRACT: &str =
    "bug1190_mutual_inductor_parameter_alias_baseline";
const XYCE_CLASSIC_MOS_DTEMP_WRAPPER_CONTRACT: &str =
    "classic_mos_level1_dtemp_relational_wrapper_owner";
const XYCE_CLASSIC_MOS_DTEMP_REFERENCE_CONTRACT: &str =
    "classic_mos_level1_dtemp_relational_wrapper_reference";
const XYCE_LEGACY_BJT_DTEMP_WRAPPER_CONTRACT: &str =
    "legacy_gummel_poon_bjt_dtemp_relational_wrapper_owner";
const XYCE_LEGACY_BJT_DTEMP_REFERENCE_CONTRACT: &str =
    "legacy_gummel_poon_bjt_dtemp_relational_wrapper_reference";
const XYCE_LEGACY_BJT_DTEMP_NPN_OWNER_RECORD: &str = "netlists/dtemp/npn_dtemp.cir";
const XYCE_LEGACY_BJT_DTEMP_NPN_REFERENCE_RECORD: &str = "netlists/dtemp/npn_ref.cir";
const XYCE_LEGACY_BJT_DTEMP_PNP_OWNER_RECORD: &str = "netlists/dtemp/pnp_dtemp.cir";
const XYCE_LEGACY_BJT_DTEMP_PNP_REFERENCE_RECORD: &str = "netlists/dtemp/pnp_ref.cir";
const XYCE_SYDNEY_LEVEL1_JFET_DTEMP_WRAPPER_CONTRACT: &str =
    "xyce_sydney_level1_jfet_dtemp_relational_wrapper_owner";
const XYCE_SYDNEY_LEVEL1_JFET_DTEMP_REFERENCE_CONTRACT: &str =
    "xyce_sydney_level1_jfet_dtemp_relational_wrapper_reference";
const XYCE_SYDNEY_LEVEL1_JFET_DTEMP_NJF_OWNER_RECORD: &str = "netlists/dtemp/njfet_dtemp.cir";
const XYCE_SYDNEY_LEVEL1_JFET_DTEMP_NJF_REFERENCE_RECORD: &str = "netlists/dtemp/njfet_ref.cir";
const XYCE_SYDNEY_LEVEL1_JFET_DTEMP_PJF_OWNER_RECORD: &str = "netlists/dtemp/pjfet_dtemp.cir";
const XYCE_SYDNEY_LEVEL1_JFET_DTEMP_PJF_REFERENCE_RECORD: &str = "netlists/dtemp/pjfet_ref.cir";
// The removed Release 7.10 PARAMS1 wrapper used a byte comparison only as a
// fast path, then accepted the pair through default `xyce_verify`. The native
// reconstruction therefore binds these exact sources and applies the strict
// default-tolerance relational comparator rather than making serialized PRN
// identity the final oracle.
const XYCE_PARAMS1_WRAPPER_OWNER_CONTRACT: &str = "params1_parameter_equivalence_wrapper_owner";
const XYCE_PARAMS1_LITERAL_BASELINE_CONTRACT: &str =
    "params1_parameter_equivalence_literal_baseline";
const XYCE_PARAMS1_PARAMETERIZED_MEMBER_CONTRACT: &str =
    "params1_parameter_equivalence_parameterized_member";
const XYCE_PARAMS1_OWNER_RECORD: &str = "netlists/params1/params_a.cir";
const XYCE_PARAMS1_LITERAL_BASELINE_RECORD: &str = "netlists/params1/params_a0.cir";
const XYCE_PARAMS1_PARAMETERIZED_MEMBER_RECORD: &str = "netlists/params1/params_a1.cir";
const XYCE_PARAMS1_LITERAL_BASELINE_CONTENT_BLAKE3: &str =
    "c8c470a5f69c608d8a1b6dfceb6f74cd000e5f10279c16f7361ac10e3a618e04";
const XYCE_PARAMS1_PARAMETERIZED_MEMBER_CONTENT_BLAKE3: &str =
    "ff7f4422bab651a252c2a1f26d85de3f5dcb2283c80abaaa409d6b7d6bfcf27d";
// The removed Release 7.10 nakedAlgebra wrapper used byte comparisons as
// fast paths, then accepted both mixed-expression parameter spellings through
// default `xyce_verify`. The native reconstruction binds the three canonical
// sources and applies the same strict default-tolerance relational oracle.
const XYCE_NAKED_ALGEBRA_WRAPPER_OWNER_CONTRACT: &str =
    "naked_algebra_parameter_equivalence_wrapper_owner";
const XYCE_NAKED_ALGEBRA_BRACED_BASELINE_CONTRACT: &str =
    "naked_algebra_parameter_equivalence_braced_baseline";
const XYCE_NAKED_ALGEBRA_GLOBAL_MEMBER_CONTRACT: &str =
    "naked_algebra_parameter_equivalence_global_member";
const XYCE_NAKED_ALGEBRA_OWNER_RECORD: &str = "netlists/parser/nakedalgebra.cir";
const XYCE_NAKED_ALGEBRA_BRACED_BASELINE_RECORD: &str = "netlists/parser/nakedalgebrabaseline.cir";
const XYCE_NAKED_ALGEBRA_GLOBAL_MEMBER_RECORD: &str = "netlists/parser/nakedalgebraglobal.cir";
const XYCE_NAKED_ALGEBRA_OWNER_CONTENT_BLAKE3: &str =
    "f0f94c056f62bee96b49b46f1a1bc693b6bfc79d7e0a699741fc90334579a094";
const XYCE_NAKED_ALGEBRA_BRACED_BASELINE_CONTENT_BLAKE3: &str =
    "112fac431981b869aad379659da705edfbe28f0f5a2ab4f3119fe2e6bfa7e8ee";
const XYCE_NAKED_ALGEBRA_GLOBAL_MEMBER_CONTENT_BLAKE3: &str =
    "3161ad78989d933ea20e80abb61165ade9a932c90c17604721439b1b1c530f65";
// BUG 1826's removed Release 7.10 wrapper compares the GLOBAL_PARAM member
// against the ordinary PARAM member by byte identity first and default
// `xyce_verify` second. The comparator is directional: the sorted global deck
// is GOODFILE and the local-parameter deck is TESTFILE. These identities bind
// both executable roots, their shared copper material model, and that exact
// historical oracle before native execution is admitted.
const XYCE_BUG1826_THERMAL_PARAMETER_WRAPPER_OWNER_CONTRACT: &str =
    "bug1826_thermal_parameter_scope_wrapper_owner";
const XYCE_BUG1826_THERMAL_PARAMETER_GLOBAL_BASELINE_CONTRACT: &str =
    "bug1826_thermal_parameter_scope_global_baseline";
const XYCE_BUG1826_THERMAL_PARAMETER_LOCAL_MEMBER_CONTRACT: &str =
    "bug1826_thermal_parameter_scope_local_member";
const XYCE_BUG1826_THERMAL_PARAMETER_OWNER_RECORD: &str =
    "netlists/certification_tests/bug_1826/linear_simple.cir";
const XYCE_BUG1826_THERMAL_PARAMETER_GLOBAL_BASELINE_RECORD: &str =
    "netlists/certification_tests/bug_1826/linear_simple_global.cir";
const XYCE_BUG1826_THERMAL_PARAMETER_LOCAL_MEMBER_RECORD: &str =
    "netlists/certification_tests/bug_1826/linear_simple_param.cir";
const XYCE_BUG1826_THERMAL_PARAMETER_GLOBAL_BASELINE_CONTENT_BLAKE3: &str =
    "0498a73d52c4e66391b4920baf96c0838f08aa007c33fa5573be616609beca0a";
const XYCE_BUG1826_THERMAL_PARAMETER_LOCAL_MEMBER_CONTENT_BLAKE3: &str =
    "fcc455267359d3ab906fe0aee58f4967e46c867a6fbef4f0420a6667f348ca45";
// Release 7.10 qualified these current-source multiplier decks by running the
// authored M= owner as GOODFILE and its explicit 0.2-Siemens control as
// TESTFILE. The BSRC and VCCS directories also contain unrelated native
// regressions, so the family uses an exact-record selector.
const XYCE_SOURCE_MULTIPLICITY_WRAPPER_CONTRACT: &str = "source_multiplicity_family_wrapper";
const XYCE_SOURCE_MULTIPLICITY_BASELINE_CONTRACT: &str = "source_multiplicity_family_baseline";

// Release 7.10's four ABM_FREQ wrappers run the authored FREQ/HERTZ deck as
// ACComparator's directional GOODFILE and the corresponding .AC DATA deck as
// TESTFILE.
const XYCE_ABM_FREQUENCY_WRAPPER_OWNER_CONTRACT: &str = "abm_frequency_relational_wrapper_owner";
const XYCE_ABM_FREQUENCY_DATA_CONTROL_CONTRACT: &str = "abm_frequency_relational_data_control";
const XYCE_ABM_FREQUENCY_GRID: [Value; 6] = [1.0, 10.0, 100.0, 1.0e3, 1.0e4, 1.0e5];
const XYCE_ABM_FREQUENCY_GRID_RELATIVE_ROUNDOFF: Value = 64.0 * f64::EPSILON;

// Release 7.10's BUG_1043_SON wrapper runs the frequency-expression deck as
// ACComparator's directional GOODFILE and the AC DATA parameter-sweep deck as
// TESTFILE.  The analytic sibling has its own checked-in .FD.prn oracle and is
// deliberately not a member of this relational pair.
const XYCE_BUG1043_AC_DATA_PARAMETER_WRAPPER_OWNER_CONTRACT: &str =
    "bug1043_ac_data_parameter_relational_wrapper_owner";
const XYCE_BUG1043_AC_DATA_PARAMETER_EXPRESSION_BASELINE_CONTRACT: &str =
    "bug1043_ac_data_parameter_relational_expression_baseline";
const XYCE_BUG1043_OWNER_PATH: &str = "Netlists/Certification_Tests/BUG_1043_SON/RC_AC_params.cir";
const XYCE_BUG1043_EXPRESSION_BASELINE_PATH: &str =
    "Netlists/Certification_Tests/BUG_1043_SON/RC_AC_params_expr.cir";
#[cfg(test)]
const XYCE_BUG1043_ANALYTIC_PATH: &str =
    "Netlists/Certification_Tests/BUG_1043_SON/RC_AC_params_analytic.cir";
const XYCE_BUG1043_OWNER_RECORD: &str =
    "netlists/certification_tests/bug_1043_son/rc_ac_params.cir";
const XYCE_BUG1043_EXPRESSION_BASELINE_RECORD: &str =
    "netlists/certification_tests/bug_1043_son/rc_ac_params_expr.cir";
const XYCE_BUG1043_FREQUENCY_GRID: [Value; 6] = [1.0, 10.0, 100.0, 1.0e3, 1.0e4, 1.0e5];
const XYCE_BUG1043_FREQUENCY_GRID_RELATIVE_ROUNDOFF: Value = 64.0 * f64::EPSILON;

// Release 7.10's ABM_SPLINES ordering wrappers run each authored out-of-order
// inline lookup deck first, then its ordered control, and require their default
// PRN files to be byte-identical; the pair contract reproduces that natively.
const XYCE_ABM_LOOKUP_ORDER_WRAPPER_OWNER_CONTRACT: &str =
    "abm_splines_inline_lookup_order_wrapper_owner";
const XYCE_ABM_LOOKUP_ORDER_SORTED_CONTROL_CONTRACT: &str =
    "abm_splines_inline_lookup_order_sorted_control";
const XYCE_ABM_LOOKUP_ORDER_GRID: [Value; 11] =
    [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];

// Release 7.10's BUG_38_SON wrapper executes the ordinary SUBCKT-formal
// spelling and the HSpice-compatible parenthesized spelling independently,
// then applies `diff -i` to their default PRN files. Bind the exact retained
// pair and the historical README/wrapper/exclude artifacts before reproducing
// that relational oracle natively.
const XYCE_BUG38_WRAPPER_OWNER_CONTRACT: &str = "bug38_subckt_formal_parentheses_wrapper_owner";
const XYCE_BUG38_PARENTHESIZED_CONTROL_CONTRACT: &str = "bug38_subckt_formal_parentheses_control";
const XYCE_BUG38_OWNER_PATH: &str = "Netlists/Certification_Tests/BUG_38_SON/bug_38_son.cir";
const XYCE_BUG38_CONTROL_PATH: &str = "Netlists/Certification_Tests/BUG_38_SON/bug_38_son_p.cir";
const XYCE_BUG38_OWNER_RECORD: &str = "netlists/certification_tests/bug_38_son/bug_38_son.cir";
const XYCE_BUG38_CONTROL_RECORD: &str = "netlists/certification_tests/bug_38_son/bug_38_son_p.cir";
// Release 7.10's two BUG_39_SON random-function placeholders are independent
// wrapper owners. Each removed Perl wrapper generates a 10,000-resistor,
// one-point DC deck and accepts only the historical population mean/sigma
// predicate. Bind both empty retained anchors and the exact historical
// generated-oracle artifacts before reproducing either contract in memory.
const XYCE_BUG39_AGAUSS_CONTRACT: &str = "bug39_agauss_generated_mean_sigma_wrapper";
const XYCE_BUG39_GAUSS_CONTRACT: &str = "bug39_gauss_generated_mean_sigma_wrapper";
const XYCE_BUG39_INT_CONTRACT: &str = "bug39_int_single_point_xyce_verify_dc_wrapper";
const XYCE_BUG39_LIMIT_CONTRACT: &str = "bug39_limit_nominal_single_point_xyce_verify_dc_wrapper";
const XYCE_BUG39_POW_CONTRACT: &str = "bug39_pow_single_point_xyce_verify_dc_wrapper";
const XYCE_BUG39_SIGN_CONTRACT: &str = "bug39_sign_single_point_xyce_verify_dc_wrapper";
const XYCE_BUG39_AGAUSS_PATH: &str = "Netlists/Certification_Tests/BUG_39_SON/agauss.cir";
const XYCE_BUG39_GAUSS_PATH: &str = "Netlists/Certification_Tests/BUG_39_SON/gauss.cir";
const XYCE_BUG39_AGAUSS_RECORD: &str = "netlists/certification_tests/bug_39_son/agauss.cir";
const XYCE_BUG39_GAUSS_RECORD: &str = "netlists/certification_tests/bug_39_son/gauss.cir";
const XYCE_BUG39_INT_PATH: &str = "Netlists/Certification_Tests/BUG_39_SON/bug39_int.cir";
const XYCE_BUG39_LIMIT_PATH: &str = "Netlists/Certification_Tests/BUG_39_SON/bug39_limit.cir";
const XYCE_BUG39_POW_PATH: &str = "Netlists/Certification_Tests/BUG_39_SON/bug39_pow.cir";
const XYCE_BUG39_SIGN_PATH: &str = "Netlists/Certification_Tests/BUG_39_SON/bug39_sign.cir";
const XYCE_BUG39_INT_RECORD: &str = "netlists/certification_tests/bug_39_son/bug39_int.cir";
const XYCE_BUG39_LIMIT_RECORD: &str = "netlists/certification_tests/bug_39_son/bug39_limit.cir";
const XYCE_BUG39_POW_RECORD: &str = "netlists/certification_tests/bug_39_son/bug39_pow.cir";
const XYCE_BUG39_SIGN_RECORD: &str = "netlists/certification_tests/bug_39_son/bug39_sign.cir";
const XYCE_BUG39_SAMPLE_COUNT: usize = 10_000;
const XYCE_BUG39_MEAN: Value = 100.0;
const XYCE_BUG39_EXPECTED_SIGMA: Value = 1.0;
const XYCE_BUG39_MOMENT_TOLERANCE: Value = 5.0e-2;

// Release 7.10's BUG_402_SON wrapper runs the canonical Xyce
// `.OPTIONS DEVICE TEMP=35` deck first, then the legacy SPICE
// `.OPTIONS TEMP=35` deck, and directionally compares their DC tables with
// xyce_verify. The checked-in bug402son.cir is only the wrapper owner; it is
// never simulated. Bind the complete retained family and every historical
// harness artifact that defines this relational oracle before reproducing it.
const XYCE_BUG402_OWNER_CONTRACT: &str = "bug402_temperature_option_scope_relational_wrapper_owner";
const XYCE_BUG402_XYCE_REFERENCE_CONTRACT: &str =
    "bug402_temperature_option_scope_relational_xyce_reference";
const XYCE_BUG402_SPICE_MEMBER_CONTRACT: &str =
    "bug402_temperature_option_scope_relational_spice_member";
const XYCE_BUG402_OWNER_PATH: &str = "Netlists/Certification_Tests/BUG_402_SON/bug402son.cir";
const XYCE_BUG402_XYCE_REFERENCE_PATH: &str =
    "Netlists/Certification_Tests/BUG_402_SON/bug402XyceTempOptions.cir";
const XYCE_BUG402_SPICE_MEMBER_PATH: &str =
    "Netlists/Certification_Tests/BUG_402_SON/bug402SpiceTempOptions.cir";
const XYCE_BUG402_OWNER_RECORD: &str = "netlists/certification_tests/bug_402_son/bug402son.cir";
const XYCE_BUG402_XYCE_REFERENCE_RECORD: &str =
    "netlists/certification_tests/bug_402_son/bug402xycetempoptions.cir";
const XYCE_BUG402_SPICE_MEMBER_RECORD: &str =
    "netlists/certification_tests/bug_402_son/bug402spicetempoptions.cir";
const XYCE_BUG402_DC_POINT_COUNT: usize = 51;

#[cfg(test)]
const XYCE_BUG354_FUNCTION_PATH: &str = "Netlists/Certification_Tests/BUG_354_SON/bad_function.cir";
#[cfg(test)]
const XYCE_BUG354_LEAD_CURRENT_PATH: &str =
    "Netlists/Certification_Tests/BUG_354_SON/bad_leadcurrent.cir";
#[cfg(test)]
const XYCE_BUG354_PARAMETER_PATH: &str =
    "Netlists/Certification_Tests/BUG_354_SON/bad_parameter.cir";
const XYCE_BUG354_FUNCTION_RECORD: &str =
    "netlists/certification_tests/bug_354_son/bad_function.cir";
const XYCE_BUG354_LEAD_CURRENT_RECORD: &str =
    "netlists/certification_tests/bug_354_son/bad_leadcurrent.cir";
const XYCE_BUG354_PARAMETER_RECORD: &str =
    "netlists/certification_tests/bug_354_son/bad_parameter.cir";
const XYCE_BUG354_HISTORICAL_TIMEOUT_MS: u128 = 30_000;

// BUG_48_SON proves that the historical MOS LEVEL=54 alias selects BSIM4.
// Release 7.10 required only a successful simulator exit; its comparator was
// commented out and no numerical gold was authoritative. RSpice strengthens
// that success predicate with a typed native-device and finite-DC contract.
const XYCE_BUG48_CONTRACT: &str = "bug48_level54_native_bsim4_success_wrapper";
#[cfg(test)]
const XYCE_BUG48_PATH: &str = "Netlists/Certification_Tests/BUG_48_SON/test.cir";
const XYCE_BUG48_RECORD: &str = "netlists/certification_tests/bug_48_son/test.cir";

// BUG_159 is a removed-shell relational oracle. The empty owner deck selects
// a wrapper that runs the two real worker decks in order and byte-diffs their
// nonempty default PRN output. The only intended semantic difference is an
// explicit BJT model TNOM=27 C versus the same omitted default.
const XYCE_BUG159_CONTRACT: &str = "bug159_bjt_tnom_default_equivalence_wrapper";
const XYCE_BUG159_EXPLICIT_PATH: &str = "Netlists/Certification_Tests/BUG_159/bug_159_1.cir";
const XYCE_BUG159_IMPLICIT_PATH: &str = "Netlists/Certification_Tests/BUG_159/bug_159_2.cir";
const XYCE_BUG159_OWNER_RECORD: &str = "netlists/certification_tests/bug_159/bug_159.cir";

// BUG_267 is an include-backed global-parameter success regression. The
// removed shell wrapper only treats a nonzero simulator exit as failure (its
// final missing-PRN branch accidentally returns success), and supplies no
// numerical gold. RSpice preserves that success contract while strengthening
// it with the exact typed include/parameter graph and analytic six-point DC
// response.
const XYCE_BUG267_CONTRACT: &str = "bug267_global_parameter_include_success_wrapper";
#[cfg(test)]
const XYCE_BUG267_PATH: &str = "Netlists/Certification_Tests/BUG_267/bug267.cir";
const XYCE_BUG267_RECORD: &str = "netlists/certification_tests/bug_267/bug267.cir";

// BUG_302 is an output-formatting wrapper. Its zero-byte owner runs two
// four-member DC/TRAN cohorts and proves that COMMA and TAB are exact textual
// transformations of default PRN output. Invalid delimiters retain the
// default layout and emit Xyce's historical warning. The eight workers remain
// independently qualified by their existing numerical contracts; only the
// removed wrapper owner is reconstructed here.
const XYCE_BUG302_CONTRACT: &str = "bug302_print_delimiter_relational_wrapper";
const XYCE_BUG302_RECORD: &str = "netlists/certification_tests/bug_302/bug_302.cir";

// BUG_352 proves that a diode model parameter authored as an expression is
// resolved before device construction. The removed wrapper runs the
// expression deck followed by its literal control and raw-diffs their default
// PRN files; no numerical gold participates in the executable oracle.
const XYCE_BUG352_OWNER_CONTRACT: &str = "bug352_diode_model_expression_equivalence_wrapper_owner";
const XYCE_BUG352_CONTROL_CONTRACT: &str =
    "bug352_diode_model_expression_equivalence_literal_control";
const XYCE_BUG352_OWNER_PATH: &str = "Netlists/Certification_Tests/BUG_352/BUG_352a.cir";
const XYCE_BUG352_CONTROL_PATH: &str = "Netlists/Certification_Tests/BUG_352/BUG_352b.cir";
const XYCE_BUG352_OWNER_RECORD: &str = "netlists/certification_tests/bug_352/bug_352a.cir";
const XYCE_BUG352_CONTROL_RECORD: &str = "netlists/certification_tests/bug_352/bug_352b.cir";

const XYCE_BUG1797_OWNER_CONTRACT: &str = "bug1797_bsim3_level_alias_relational_wrapper";
const XYCE_BUG1797_LEVEL9_CONTRACT: &str = "bug1797_bsim3_level9_relational_worker";
const XYCE_BUG1797_LEVEL49_CONTRACT: &str = "bug1797_bsim3_level49_relational_worker";
const XYCE_BUG1797_OWNER_PATH: &str = "Netlists/Certification_Tests/BUG_1797/one-shot.cir";
const XYCE_BUG1797_LEVEL9_PATH: &str = "Netlists/Certification_Tests/BUG_1797/one-shot_lev9.cir";
const XYCE_BUG1797_LEVEL49_PATH: &str = "Netlists/Certification_Tests/BUG_1797/one-shot_lev49.cir";
const XYCE_BUG1797_OWNER_RECORD: &str = "netlists/certification_tests/bug_1797/one-shot.cir";
const XYCE_BUG1797_LEVEL9_RECORD: &str = "netlists/certification_tests/bug_1797/one-shot_lev9.cir";
const XYCE_BUG1797_LEVEL49_RECORD: &str =
    "netlists/certification_tests/bug_1797/one-shot_lev49.cir";

const XYCE_BUG981_OWNER_CONTRACT: &str =
    "bug981_outputtimepoints_breakpoints_relational_wrapper_owner";
const XYCE_BUG981_OUTPUT_CONTRACT: &str =
    "bug981_outputtimepoints_breakpoints_relational_output_control";
const XYCE_BUG981_BREAKPOINT_CONTRACT: &str =
    "bug981_outputtimepoints_breakpoints_relational_breakpoint_control";
const XYCE_BUG981_OWNER_PATH: &str = "Netlists/Certification_Tests/BUG_981_SON/bug981.cir";
const XYCE_BUG981_OUTPUT_PATH: &str = "Netlists/Certification_Tests/BUG_981_SON/bug981A.cir";
const XYCE_BUG981_BREAKPOINT_PATH: &str = "Netlists/Certification_Tests/BUG_981_SON/bug981B.cir";
const XYCE_BUG981_OWNER_RECORD: &str = "netlists/certification_tests/bug_981_son/bug981.cir";
const XYCE_BUG981_OUTPUT_RECORD: &str = "netlists/certification_tests/bug_981_son/bug981a.cir";
const XYCE_BUG981_BREAKPOINT_RECORD: &str = "netlists/certification_tests/bug_981_son/bug981b.cir";

// BUG_986_SON's blank owner delegates to two transient workers. The
// Release wrapper requires both simulations and their default PRN files to
// succeed, then byte-compares the complete files. Worker A authors TIMEINT
// BREAKPOINTS while worker B obtains the same schedule from a disconnected,
// zero-valued PWL source. Both select ERROPTION=1, so this qualification also
// binds the implicit ten-step-per-breakpoint-span policy.
const XYCE_BUG986_OWNER_CONTRACT: &str =
    "bug986_erroption_breakpoint_source_relational_wrapper_owner";
const XYCE_BUG986_BREAKPOINT_CONTRACT: &str =
    "bug986_erroption_explicit_breakpoints_relational_worker";
const XYCE_BUG986_PWL_CONTRACT: &str =
    "bug986_erroption_disconnected_pwl_breakpoints_relational_worker";
const XYCE_BUG986_OWNER_PATH: &str = "Netlists/Certification_Tests/BUG_986_SON/bug986.cir";
const XYCE_BUG986_BREAKPOINT_PATH: &str = "Netlists/Certification_Tests/BUG_986_SON/bug986A.cir";
const XYCE_BUG986_PWL_PATH: &str = "Netlists/Certification_Tests/BUG_986_SON/bug986B.cir";
const XYCE_BUG986_OWNER_RECORD: &str = "netlists/certification_tests/bug_986_son/bug986.cir";
const XYCE_BUG986_BREAKPOINT_RECORD: &str = "netlists/certification_tests/bug_986_son/bug986a.cir";
const XYCE_BUG986_PWL_RECORD: &str = "netlists/certification_tests/bug_986_son/bug986b.cir";

// ISSUE_202's Release-7.10 wrapper exercises every supported
// `-redefined_params` mode against first- and last-definition controls.
const XYCE_ISSUE202_OWNER_CONTRACT: &str = "issue202_redefined_params_wrapper_owner";
const XYCE_ISSUE202_FIRST_CONTRACT: &str = "issue202_redefined_params_first_control";
const XYCE_ISSUE202_LAST_CONTRACT: &str = "issue202_redefined_params_last_control";
const XYCE_ISSUE202_OWNER_PATH: &str = "Netlists/Certification_Tests/ISSUE_202/first_last.cir";
const XYCE_ISSUE202_FIRST_PATH: &str = "Netlists/Certification_Tests/ISSUE_202/first.cir";
const XYCE_ISSUE202_LAST_PATH: &str = "Netlists/Certification_Tests/ISSUE_202/last.cir";
const XYCE_ISSUE202_OWNER_RECORD: &str = "netlists/certification_tests/issue_202/first_last.cir";
const XYCE_ISSUE202_FIRST_RECORD: &str = "netlists/certification_tests/issue_202/first.cir";
const XYCE_ISSUE202_LAST_RECORD: &str = "netlists/certification_tests/issue_202/last.cir";

// ISSUE_451's Release-7.10 wrapper compares a hierarchical-node circuit to
// an explicit in-subcircuit reference. It first requires a byte-exact PRN
// diff and only then falls back to xyce_verify.
const XYCE_ISSUE451_OWNER_CONTRACT: &str = "issue451_hierarchical_node_wrapper_owner";
const XYCE_ISSUE451_REFERENCE_CONTRACT: &str = "issue451_hierarchical_node_explicit_reference";
const XYCE_ISSUE451_OWNER_PATH: &str = "Netlists/Certification_Tests/ISSUE_451/issue451.cir";
const XYCE_ISSUE451_REFERENCE_PATH: &str =
    "Netlists/Certification_Tests/ISSUE_451/issue451_ref.cir";
const XYCE_ISSUE451_OWNER_RECORD: &str = "netlists/certification_tests/issue_451/issue451.cir";
const XYCE_ISSUE451_REFERENCE_RECORD: &str =
    "netlists/certification_tests/issue_451/issue451_ref.cir";

// BUG_1455 proves that SPICE model parameters separated by whitespace parse
// identically to their `name=value` spellings. The upstream Release wrapper
// executes both MOS1 decks and applies a case-insensitive byte diff, with no
// numerical gold. RSpice adds a closed-form MOS/load-line check so shared
// parser or device failures cannot false-pass.
const XYCE_BUG1455_OWNER_CONTRACT: &str = "bug1455_model_equals_wrapper_owner";
const XYCE_BUG1455_REFERENCE_CONTRACT: &str = "bug1455_model_whitespace_reference";
const XYCE_BUG1455_OWNER_PATH: &str = "Netlists/Certification_Tests/BUG_1455/bug_1455.cir";
const XYCE_BUG1455_REFERENCE_PATH: &str = "Netlists/Certification_Tests/BUG_1455/bug_1455_neq.cir";
const XYCE_BUG1455_OWNER_RECORD: &str = "netlists/certification_tests/bug_1455/bug_1455.cir";
const XYCE_BUG1455_REFERENCE_RECORD: &str =
    "netlists/certification_tests/bug_1455/bug_1455_neq.cir";

// BUG_28_SON's son3 wrapper proves that subcircuit-local, literal, and
// top-level global parameter spellings resolve the two mutually coupled
// inductors to one circuit. The Release-7.10 shell first attempts a raw PRN
// diff and falls back to xyce_verify for each owner/control relation.
const XYCE_BUG28SON_OWNER_CONTRACT: &str = "bug28son_subcircuit_parameter_wrapper_owner";
const XYCE_BUG28SON_LITERAL_CONTRACT: &str = "bug28son_subcircuit_parameter_literal_control";
const XYCE_BUG28SON_GLOBAL_CONTRACT: &str = "bug28son_subcircuit_parameter_global_control";
const XYCE_BUG28SON_OWNER_PATH: &str = "Netlists/Certification_Tests/BUG_28_SON/bug_28_son3.cir";
const XYCE_BUG28SON_LITERAL_PATH: &str =
    "Netlists/Certification_Tests/BUG_28_SON/bug_28_son3_noparams.cir";
const XYCE_BUG28SON_GLOBAL_PATH: &str =
    "Netlists/Certification_Tests/BUG_28_SON/bug_28_son3_globalp.cir";
const XYCE_BUG28SON_OWNER_RECORD: &str = "netlists/certification_tests/bug_28_son/bug_28_son3.cir";
const XYCE_BUG28SON_LITERAL_RECORD: &str =
    "netlists/certification_tests/bug_28_son/bug_28_son3_noparams.cir";
const XYCE_BUG28SON_GLOBAL_RECORD: &str =
    "netlists/certification_tests/bug_28_son/bug_28_son3_globalp.cir";

// BUG_1398's active Release-7.10 wrapper compares a PSpice-style inductor
// model-card multiplier/temperature deck against the equivalent literal
// inductances. There is no checked-in numerical gold; the control is the
// executable oracle.
const XYCE_BUG1398_OWNER_CONTRACT: &str = "bug1398_inductor_model_wrapper_owner";
const XYCE_BUG1398_CONTROL_CONTRACT: &str = "bug1398_inductor_model_literal_control";
const XYCE_BUG1398_OWNER_PATH: &str = "Netlists/Certification_Tests/BUG_1398/RLC.cir";
const XYCE_BUG1398_CONTROL_PATH: &str = "Netlists/Certification_Tests/BUG_1398/RLC_simple.cir";
const XYCE_BUG1398_OWNER_RECORD: &str = "netlists/certification_tests/bug_1398/rlc.cir";
const XYCE_BUG1398_CONTROL_RECORD: &str = "netlists/certification_tests/bug_1398/rlc_simple.cir";

// BUG_271_SON's active Release-7.10 shell wrapper is a success-only oracle:
// it runs the exact tab-comment RLC/PULSE deck and passes iff Xyce exits zero.
const XYCE_BUG271_CONTRACT: &str = "bug271_tab_comment_rlc_success_wrapper";
#[cfg(test)]
const XYCE_BUG271_PATH: &str = "Netlists/Certification_Tests/BUG_271_SON/bug_271.cir";
const XYCE_BUG271_RECORD: &str = "netlists/certification_tests/bug_271_son/bug_271.cir";

// BUG_1661's active Release-7.10 Perl wrapper runs this one transient deck,
// requires the generated PRN, and compares the serialized V($G_1) and V(1)
// columns for exact numeric equality. It does not use a checked-in gold file.
const XYCE_BUG1661_CONTRACT: &str = "bug1661_globalnode_behavioral_expression_equality_wrapper";
#[cfg(test)]
const XYCE_BUG1661_PATH: &str = "Netlists/Certification_Tests/BUG_1661/globalnode_expr_toplev.cir";
const XYCE_BUG1661_RECORD: &str =
    "netlists/certification_tests/bug_1661/globalnode_expr_toplev.cir";

// BUG_519_SON's active Release-7.10 wrapper runs this transient once with
// binary RAW output and once with ASCII RAW output, converts both through the
// retained RAW reader boundary, and requires V(1) to equal I(R1) in each.
// It does not use a checked-in numerical gold file.
const XYCE_BUG519_CONTRACT: &str = "bug519_binary_ascii_raw_column_equality_wrapper";
#[cfg(test)]
const XYCE_BUG519_PATH: &str = "Netlists/Certification_Tests/BUG_519_SON/bug_519_SON.cir";
const XYCE_BUG519_RECORD: &str = "netlists/certification_tests/bug_519_son/bug_519_son.cir";

// BUG_1040_SON's Release-7.10 wrapper proves that the NOOP spelling of
// transient startup produces the same diode-capacitor discharge waveform as
// ordinary operating-point startup. The zero-byte .cir file is only the
// wrapper owner; the two retained .net files are the executable workers.
const XYCE_BUG1040_CONTRACT: &str = "bug1040_noop_operating_point_transient_wrapper";
const XYCE_BUG1040_OP_PATH: &str =
    "Netlists/Certification_Tests/BUG_1040_SON/rc_discharge_diode_op.net";
const XYCE_BUG1040_NOOP_PATH: &str =
    "Netlists/Certification_Tests/BUG_1040_SON/rc_discharge_diode_noop.net";
const XYCE_BUG1040_OWNER_RECORD: &str =
    "netlists/certification_tests/bug_1040_son/bug_1040_son.cir";

// BUG_1162_SON's wrapper proves that inconsistent LIN/DEC/OCT DC sweep
// directions warn and execute exactly the authored start point. The empty
// owner runs one ordinary baseline followed by the three defective controls.
const XYCE_BUG1162_OWNER_CONTRACT: &str = "bug1162_inconsistent_dc_sweep_wrapper_owner";
const XYCE_BUG1162_BASELINE_CONTRACT: &str = "bug1162_inconsistent_dc_sweep_baseline_control";
const XYCE_BUG1162_LINEAR_CONTRACT: &str = "bug1162_inconsistent_dc_sweep_linear_control";
const XYCE_BUG1162_DECADE_CONTRACT: &str = "bug1162_inconsistent_dc_sweep_decade_control";
const XYCE_BUG1162_OCTAVE_CONTRACT: &str = "bug1162_inconsistent_dc_sweep_octave_control";
const XYCE_BUG1162_OWNER_PATH: &str = "Netlists/Certification_Tests/BUG_1162_SON/bug_1162_son.cir";
const XYCE_BUG1162_BASELINE_PATH: &str = "Netlists/Certification_Tests/BUG_1162_SON/baseline.cir";
const XYCE_BUG1162_LINEAR_PATH: &str =
    "Netlists/Certification_Tests/BUG_1162_SON/defective_lin.cir";
const XYCE_BUG1162_DECADE_PATH: &str =
    "Netlists/Certification_Tests/BUG_1162_SON/defective_dec.cir";
const XYCE_BUG1162_OCTAVE_PATH: &str =
    "Netlists/Certification_Tests/BUG_1162_SON/defective_oct.cir";
const XYCE_BUG1162_OWNER_RECORD: &str =
    "netlists/certification_tests/bug_1162_son/bug_1162_son.cir";
const XYCE_BUG1162_BASELINE_RECORD: &str = "netlists/certification_tests/bug_1162_son/baseline.cir";
const XYCE_BUG1162_LINEAR_RECORD: &str =
    "netlists/certification_tests/bug_1162_son/defective_lin.cir";
const XYCE_BUG1162_DECADE_RECORD: &str =
    "netlists/certification_tests/bug_1162_son/defective_dec.cir";
const XYCE_BUG1162_OCTAVE_RECORD: &str =
    "netlists/certification_tests/bug_1162_son/defective_oct.cir";

// BUG_307's serial shell wrapper proves that an unused sibling subcircuit's
// same-named local resistor model cannot escape its lexical scope. It runs the
// collision owner followed by the active-only control and raw-diffs their
// default PRN files; no numerical gold or xyce_verify result participates.
const XYCE_BUG307_OWNER_CONTRACT: &str = "bug307_subcircuit_model_scope_wrapper_owner";
const XYCE_BUG307_CONTROL_CONTRACT: &str = "bug307_subcircuit_model_scope_control";
const XYCE_BUG307_OWNER_PATH: &str = "Netlists/Certification_Tests/BUG_307/bug_307_a.cir";
const XYCE_BUG307_CONTROL_PATH: &str = "Netlists/Certification_Tests/BUG_307/bug_307_b.cir";
const XYCE_BUG307_OWNER_RECORD: &str = "netlists/certification_tests/bug_307/bug_307_a.cir";
const XYCE_BUG307_CONTROL_RECORD: &str = "netlists/certification_tests/bug_307/bug_307_b.cir";

// BUG_864_SON is a bounded expected-error contract. Release 7.10 executes
// the deck, requires a nonzero exit, and accepts the run only when either
// complete output stream contains the unresolved-definition diagnostic. The
// retained `options` sidecar supplies the historical 30-second outer limit.
const XYCE_BUG864_CONTRACT: &str = "expected_failure_bug864_unresolved_subcircuit_parameter_build";
#[cfg(test)]
const XYCE_BUG864_PATH: &str = "Netlists/Certification_Tests/BUG_864_SON/bug_864_son.cir";
const XYCE_BUG864_RECORD: &str = "netlists/certification_tests/bug_864_son/bug_864_son.cir";
const XYCE_BUG864_UPSTREAM_DIAGNOSTIC: &str =
    "Unable to resolve parameter FOO found in .PARAM statement";
const XYCE_BUG864_HISTORICAL_TIMEOUT_MS: u128 = 30_000;

// BUG_1025 is an active Release-7.10 error-exit wrapper. Its zero-byte deck
// must terminate without hanging and report that no analysis was specified.
const XYCE_BUG1025_CONTRACT: &str = "expected_failure_bug1025_no_analysis_selection";
#[cfg(test)]
const XYCE_BUG1025_PATH: &str = "Netlists/Certification_Tests/BUG_1025/null.cir";
const XYCE_BUG1025_RECORD: &str = "netlists/certification_tests/bug_1025/null.cir";
const XYCE_BUG1025_UPSTREAM_DIAGNOSTIC: &str = "No analysis specified";
const XYCE_BUG1025_HISTORICAL_TIMEOUT_MS: u128 = 30_000;

// BUG_636_SON is an active Release-7.10 error-exit regression. Its wrapper
// requires a nonzero simulator exit followed by these two ordered diagnostics
// for the incomplete two-field `.TRAN` line.
const XYCE_BUG636_CONTRACT: &str = "expected_failure_bug636_incomplete_tran_parse";
#[cfg(test)]
const XYCE_BUG636_PATH: &str = "Netlists/Certification_Tests/BUG_636_SON/bug636.cir";
const XYCE_BUG636_RECORD: &str = "netlists/certification_tests/bug_636_son/bug636.cir";
const XYCE_BUG636_UPSTREAM_DIAGNOSTIC_ARITY: &str = ".TRAN line has an unexpected number of fields";
const XYCE_BUG636_UPSTREAM_DIAGNOSTIC_IGNORED: &str = "Unrecognized dot line will be ignored";

// BUG_206 is an active Release-7.10 error-exit regression. Its wrapper
// requires a nonzero simulator exit followed by these two ordered hierarchy
// diagnostics for the malformed X instance on source line three.
const XYCE_BUG206_CONTRACT: &str = "expected_failure_bug206_undefined_subcircuit_build";
#[cfg(test)]
const XYCE_BUG206_PATH: &str = "Netlists/Certification_Tests/BUG_206/bug_206.cir";
const XYCE_BUG206_RECORD: &str = "netlists/certification_tests/bug_206/bug_206.cir";
const XYCE_BUG206_UPSTREAM_DIAGNOSTIC_LOCATION: &str = "in file bug_206.cir at or near line 3";
const XYCE_BUG206_UPSTREAM_DIAGNOSTIC_UNDEFINED: &str =
    "Subcircuit 0 has not been defined for instance X1";

// BUG_1116 is an active Release-7.10 error-exit regression. Its malformed
// diode card supplies only two tokens after the instance name, leaving the
// mandatory model field absent.
const XYCE_BUG1116_CONTRACT: &str = "expected_failure_bug1116_missing_diode_model_parse";
#[cfg(test)]
const XYCE_BUG1116_PATH: &str = "Netlists/Certification_Tests/BUG_1116/bug_1116.cir";
const XYCE_BUG1116_RECORD: &str = "netlists/certification_tests/bug_1116/bug_1116.cir";
const XYCE_BUG1116_UPSTREAM_DIAGNOSTIC: &str =
    "Model is required for device D1 and no valid model card found";

// ISSUE_61 is the active Xyce regression for a behavioral current expression
// attempting to read the nonexistent MNA branch of another current-output B
// source. Release-7.10 requires a clean error instead of the historical crash.
const XYCE_ISSUE61_CONTRACT: &str =
    "expected_failure_issue61_behavioral_lead_current_reference_build";
#[cfg(test)]
const XYCE_ISSUE61_PATH: &str = "Netlists/Certification_Tests/ISSUE_61/issue61.cir";
const XYCE_ISSUE61_RECORD: &str = "netlists/certification_tests/issue_61/issue61.cir";
const XYCE_ISSUE61_UPSTREAM_DIAGNOSTIC: &str =
    "Device instance B2: Problem with value for B1 in B2";

// BUG_784 is an archived error-exit regression: Release-7.10 retains its
// wrapper and exact ordered diagnostic, while `tags=exclude` deliberately
// leaves its generated CMake file without an active CTest registration.
const XYCE_BUG784_CONTRACT: &str = "archived_expected_failure_bug784_duplicate_subcircuit_port";
#[cfg(test)]
const XYCE_BUG784_PATH: &str = "Netlists/Certification_Tests/BUG_784/bug_784.cir";
const XYCE_BUG784_RECORD: &str = "netlists/certification_tests/bug_784/bug_784.cir";
const XYCE_BUG784_UPSTREAM_DIAGNOSTIC_PREFIX: &str = "in file bug_784.cir at or near line 7";
const XYCE_BUG784_UPSTREAM_DIAGNOSTIC: &str = "Duplicate node in .SUBCKT line: b";

// DIODE_ANALYTIC is an active three-member Release-7.10 generated-gold
// family. Each wrapper evaluates the analytic diode law on the simulator's
// own transient output grid before invoking xyce_verify.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceDiodeAnalyticKind {
    Forward,
    Reverse,
    Breakdown,
}

impl XyceDiodeAnalyticKind {
    const ALL: [Self; 3] = [Self::Forward, Self::Reverse, Self::Breakdown];

    #[cfg(test)]
    fn path(self) -> &'static str {
        match self {
            Self::Forward => "Netlists/DIODE_ANALYTIC/forward_model.cir",
            Self::Reverse => "Netlists/DIODE_ANALYTIC/reverse_model.cir",
            Self::Breakdown => "Netlists/DIODE_ANALYTIC/breakdown_model.cir",
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::Forward => "netlists/diode_analytic/forward_model.cir",
            Self::Reverse => "netlists/diode_analytic/reverse_model.cir",
            Self::Breakdown => "netlists/diode_analytic/breakdown_model.cir",
        }
    }

    fn result_contract(self) -> &'static str {
        match self {
            Self::Forward => "analytic_diode_forward_tran_wrapper",
            Self::Reverse => "analytic_diode_reverse_tran_wrapper",
            Self::Breakdown => "analytic_diode_breakdown_tran_wrapper",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Forward => "DIODE_ANALYTIC forward-region oracle",
            Self::Reverse => "DIODE_ANALYTIC reverse-region oracle",
            Self::Breakdown => "DIODE_ANALYTIC avalanche-breakdown oracle",
        }
    }

    fn for_record(record: &str) -> Option<Self> {
        let normalized = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL
            .into_iter()
            .find(|kind| normalized == kind.record())
    }
}

// BJT_ANALYTIC and NMOS_ANALYTIC are active Release-7.10 generated-gold
// families. Their wrappers evaluate the corresponding legacy compact-model
// law on the simulator's own default-PRN transient grid before invoking the
// shared xyce_verify comparator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceLegacyDeviceAnalyticKind {
    BjtRamp1,
    BjtRamp2,
    NmosLevel1,
}

impl XyceLegacyDeviceAnalyticKind {
    const ALL: [Self; 3] = [Self::BjtRamp1, Self::BjtRamp2, Self::NmosLevel1];

    fn path(self) -> &'static str {
        match self {
            Self::BjtRamp1 => "Netlists/BJT_ANALYTIC/ramp_test1.cir",
            Self::BjtRamp2 => "Netlists/BJT_ANALYTIC/ramp_test2.cir",
            Self::NmosLevel1 => "Netlists/NMOS_ANALYTIC/mosfet_level1.cir",
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::BjtRamp1 => "netlists/bjt_analytic/ramp_test1.cir",
            Self::BjtRamp2 => "netlists/bjt_analytic/ramp_test2.cir",
            Self::NmosLevel1 => "netlists/nmos_analytic/mosfet_level1.cir",
        }
    }

    fn result_contract(self) -> &'static str {
        match self {
            Self::BjtRamp1 => "analytic_bjt_level1_depletion_charge_tran_wrapper",
            Self::BjtRamp2 => "analytic_bjt_level1_transit_charge_tran_wrapper",
            Self::NmosLevel1 => "analytic_nmos_level1_regions_tran_wrapper",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::BjtRamp1 => "BJT_ANALYTIC Level-1 depletion-charge oracle",
            Self::BjtRamp2 => "BJT_ANALYTIC Level-1 transit-charge oracle",
            Self::NmosLevel1 => "NMOS_ANALYTIC Level-1 saturation/triode oracle",
        }
    }

    fn for_record(record: &str) -> Option<Self> {
        let normalized = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL
            .into_iter()
            .find(|kind| normalized == kind.record())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct XyceAbmLookupOrderCaseSpec {
    family: &'static str,
    owner_path: &'static str,
    control_path: &'static str,
    owner_record: &'static str,
    control_record: &'static str,
    kind: XyceAbmLookupKind,
}

const XYCE_ABM_LOOKUP_ORDER_CASES: [XyceAbmLookupOrderCaseSpec; 2] = [
    XyceAbmLookupOrderCaseSpec {
        family: "ABM_SPLINES/akimaOutOfOrder",
        owner_path: "Netlists/ABM_SPLINES/akimaOutOfOrder.cir",
        control_path: "Netlists/ABM_SPLINES/akimaOutOfOrder_baseline.cir",
        owner_record: "netlists/abm_splines/akimaoutoforder.cir",
        control_record: "netlists/abm_splines/akimaoutoforder_baseline.cir",
        kind: XyceAbmLookupKind::Akima,
    },
    XyceAbmLookupOrderCaseSpec {
        family: "ABM_SPLINES/tableOutOfOrder2",
        owner_path: "Netlists/ABM_SPLINES/tableOutOfOrder2.cir",
        control_path: "Netlists/ABM_SPLINES/tableOutOfOrder2_baseline.cir",
        owner_record: "netlists/abm_splines/tableoutoforder2.cir",
        control_record: "netlists/abm_splines/tableoutoforder2_baseline.cir",
        kind: XyceAbmLookupKind::Table,
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct XyceAbmFrequencyCaseSpec {
    family: &'static str,
    owner_path: &'static str,
    control_path: &'static str,
    owner_record: &'static str,
    control_record: &'static str,
    kind: XyceAbmFrequencyKind,
    variable: XyceAbmFrequencyVariable,
}

const XYCE_ABM_FREQUENCY_CASES: [XyceAbmFrequencyCaseSpec; 4] = [
    XyceAbmFrequencyCaseSpec {
        family: "ABM_FREQ/abmfreq1",
        owner_path: "Netlists/ABM_FREQ/abmfreq1.cir",
        control_path: "Netlists/ABM_FREQ/abmfreq1_data.cir",
        owner_record: "netlists/abm_freq/abmfreq1.cir",
        control_record: "netlists/abm_freq/abmfreq1_data.cir",
        kind: XyceAbmFrequencyKind::BehavioralCurrent,
        variable: XyceAbmFrequencyVariable::Freq,
    },
    XyceAbmFrequencyCaseSpec {
        family: "ABM_FREQ/abmhertz1",
        owner_path: "Netlists/ABM_FREQ/abmhertz1.cir",
        control_path: "Netlists/ABM_FREQ/abmhertz1_data.cir",
        owner_record: "netlists/abm_freq/abmhertz1.cir",
        control_record: "netlists/abm_freq/abmhertz1_data.cir",
        kind: XyceAbmFrequencyKind::BehavioralCurrent,
        variable: XyceAbmFrequencyVariable::Hertz,
    },
    XyceAbmFrequencyCaseSpec {
        family: "ABM_FREQ/RC_simple",
        owner_path: "Netlists/ABM_FREQ/RC_simple.cir",
        control_path: "Netlists/ABM_FREQ/RC_data.cir",
        owner_record: "netlists/abm_freq/rc_simple.cir",
        control_record: "netlists/abm_freq/rc_data.cir",
        kind: XyceAbmFrequencyKind::Resistor,
        variable: XyceAbmFrequencyVariable::Freq,
    },
    XyceAbmFrequencyCaseSpec {
        family: "ABM_FREQ/RC_simple_hertz",
        owner_path: "Netlists/ABM_FREQ/RC_simple_hertz.cir",
        control_path: "Netlists/ABM_FREQ/RC_data_hertz.cir",
        owner_record: "netlists/abm_freq/rc_simple_hertz.cir",
        control_record: "netlists/abm_freq/rc_data_hertz.cir",
        kind: XyceAbmFrequencyKind::Resistor,
        variable: XyceAbmFrequencyVariable::Hertz,
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct XyceBug1043AcDataParameterCaseSpec {
    family: &'static str,
    owner_path: &'static str,
    baseline_path: &'static str,
    owner_record: &'static str,
    baseline_record: &'static str,
}

const XYCE_BUG1043_AC_DATA_PARAMETER_CASE: XyceBug1043AcDataParameterCaseSpec =
    XyceBug1043AcDataParameterCaseSpec {
        family: "Certification_Tests/BUG_1043_SON/RC_AC_params",
        owner_path: XYCE_BUG1043_OWNER_PATH,
        baseline_path: XYCE_BUG1043_EXPRESSION_BASELINE_PATH,
        owner_record: XYCE_BUG1043_OWNER_RECORD,
        baseline_record: XYCE_BUG1043_EXPRESSION_BASELINE_RECORD,
    };

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct XyceSourceMultiplicityCaseSpec {
    family: &'static str,
    owner_record: &'static str,
    baseline_record: &'static str,
    wrapper_path: &'static str,
    representation: XyceSourceMultiplicityRepresentation,
    analysis: XyceSourceMultiplicityAnalysis,
}

impl XyceSourceMultiplicityCaseSpec {
    fn owner_relative_path(self) -> &'static str {
        self.wrapper_path
            .strip_suffix(".sh")
            .expect("source-multiplicity wrapper path must end in .sh")
    }

    fn baseline_relative_path(self) -> String {
        let owner = self.owner_relative_path();
        format!(
            "{}_baseline.cir",
            owner
                .strip_suffix(".cir")
                .expect("source-multiplicity owner path must end in .cir")
        )
    }
}

const XYCE_SOURCE_MULTIPLICITY_CASES: [XyceSourceMultiplicityCaseSpec; 10] = [
    XyceSourceMultiplicityCaseSpec {
        family: "BSRC/bsrc1_m",
        owner_record: "netlists/bsrc/bsrc1_m.cir",
        baseline_record: "netlists/bsrc/bsrc1_m_baseline.cir",
        wrapper_path: "Netlists/BSRC/bsrc1_m.cir.sh",
        representation: XyceSourceMultiplicityRepresentation::BehavioralDirect,
        analysis: XyceSourceMultiplicityAnalysis::Dc,
    },
    XyceSourceMultiplicityCaseSpec {
        family: "BSRC/bsrc2_m",
        owner_record: "netlists/bsrc/bsrc2_m.cir",
        baseline_record: "netlists/bsrc/bsrc2_m_baseline.cir",
        wrapper_path: "Netlists/BSRC/bsrc2_m.cir.sh",
        representation: XyceSourceMultiplicityRepresentation::BehavioralFormal,
        analysis: XyceSourceMultiplicityAnalysis::Dc,
    },
    XyceSourceMultiplicityCaseSpec {
        family: "BSRC/bsrc3_m",
        owner_record: "netlists/bsrc/bsrc3_m.cir",
        baseline_record: "netlists/bsrc/bsrc3_m_baseline.cir",
        wrapper_path: "Netlists/BSRC/bsrc3_m.cir.sh",
        representation: XyceSourceMultiplicityRepresentation::BehavioralInherited,
        analysis: XyceSourceMultiplicityAnalysis::Dc,
    },
    XyceSourceMultiplicityCaseSpec {
        family: "BSRC/bsrc4_m",
        owner_record: "netlists/bsrc/bsrc4_m.cir",
        baseline_record: "netlists/bsrc/bsrc4_m_baseline.cir",
        wrapper_path: "Netlists/BSRC/bsrc4_m.cir.sh",
        representation: XyceSourceMultiplicityRepresentation::BehavioralNested,
        analysis: XyceSourceMultiplicityAnalysis::Dc,
    },
    XyceSourceMultiplicityCaseSpec {
        family: "VCCS/vccs_m",
        owner_record: "netlists/vccs/vccs_m.cir",
        baseline_record: "netlists/vccs/vccs_m_baseline.cir",
        wrapper_path: "Netlists/VCCS/vccs_m.cir.sh",
        representation: XyceSourceMultiplicityRepresentation::LinearDirect,
        analysis: XyceSourceMultiplicityAnalysis::Dc,
    },
    XyceSourceMultiplicityCaseSpec {
        family: "VCCS/vccs_nl1_m",
        owner_record: "netlists/vccs/vccs_nl1_m.cir",
        baseline_record: "netlists/vccs/vccs_nl1_m_baseline.cir",
        wrapper_path: "Netlists/VCCS/vccs_nl1_m.cir.sh",
        representation: XyceSourceMultiplicityRepresentation::ExpressionDirect,
        analysis: XyceSourceMultiplicityAnalysis::Dc,
    },
    XyceSourceMultiplicityCaseSpec {
        family: "VCCS/vccs_nl2_m",
        owner_record: "netlists/vccs/vccs_nl2_m.cir",
        baseline_record: "netlists/vccs/vccs_nl2_m_baseline.cir",
        wrapper_path: "Netlists/VCCS/vccs_nl2_m.cir.sh",
        representation: XyceSourceMultiplicityRepresentation::ExpressionFormal,
        analysis: XyceSourceMultiplicityAnalysis::Dc,
    },
    XyceSourceMultiplicityCaseSpec {
        family: "VCCS/vccs_nl3_m",
        owner_record: "netlists/vccs/vccs_nl3_m.cir",
        baseline_record: "netlists/vccs/vccs_nl3_m_baseline.cir",
        wrapper_path: "Netlists/VCCS/vccs_nl3_m.cir.sh",
        representation: XyceSourceMultiplicityRepresentation::ExpressionInherited,
        analysis: XyceSourceMultiplicityAnalysis::Dc,
    },
    XyceSourceMultiplicityCaseSpec {
        family: "VCCS/vccs_nl4_m",
        owner_record: "netlists/vccs/vccs_nl4_m.cir",
        baseline_record: "netlists/vccs/vccs_nl4_m_baseline.cir",
        wrapper_path: "Netlists/VCCS/vccs_nl4_m.cir.sh",
        representation: XyceSourceMultiplicityRepresentation::ExpressionNested,
        analysis: XyceSourceMultiplicityAnalysis::Dc,
    },
    XyceSourceMultiplicityCaseSpec {
        family: "VCCS/vccs_tran_m",
        owner_record: "netlists/vccs/vccs_tran_m.cir",
        baseline_record: "netlists/vccs/vccs_tran_m_baseline.cir",
        wrapper_path: "Netlists/VCCS/vccs_tran_m.cir.sh",
        representation: XyceSourceMultiplicityRepresentation::LinearDirect,
        analysis: XyceSourceMultiplicityAnalysis::Tran,
    },
];

const XYCE_LEVEL2_DIODE_DTEMP_WRAPPER_CONTRACT: &str =
    "level2_diode_dtemp_relational_wrapper_owner";
const XYCE_LEVEL2_DIODE_DTEMP_REFERENCE_CONTRACT: &str =
    "level2_diode_dtemp_relational_wrapper_reference";
const XYCE_LEVEL2_DIODE_DTEMP_OWNER_RECORD: &str = "netlists/dtemp/level2_diode_dtemp.cir";
const XYCE_LEVEL2_DIODE_DTEMP_REFERENCE_RECORD: &str = "netlists/dtemp/level2_diode_ref.cir";
const XYCE_CAPACITOR_DTEMP_WRAPPER_CONTRACT: &str = "capacitor_dtemp_relational_wrapper_owner";
const XYCE_CAPACITOR_DTEMP_REFERENCE_CONTRACT: &str =
    "capacitor_dtemp_relational_wrapper_reference";
const XYCE_CAPACITOR_DTEMP_OWNER_RECORD: &str = "netlists/dtemp/cap_dtemp.cir";
const XYCE_CAPACITOR_DTEMP_REFERENCE_RECORD: &str = "netlists/dtemp/cap_ref.cir";
// The removed Release 7.10 sidecar emits `exp(-TIME/0.001)`. These constants
// describe that generated oracle; the path-independent candidate detector does
// not use them as deck-name or value allowlists.
const XYCE_ANALYTIC_RC_ORACLE_INITIAL_VALUE: f64 = 1.0;
const XYCE_ANALYTIC_RC_ORACLE_FINAL_VALUE: f64 = 0.0;
const XYCE_ANALYTIC_RC_ORACLE_TIME_CONSTANT: f64 = 1.0e-3;
// The removed rc_osc sidecar is deliberately a fixed analytic generator. Keep
// its decimal constants distinct from the simulator's source evaluation: in
// particular, the Perl oracle uses 3.1415927 rather than a platform PI value.
const XYCE_ANALYTIC_SINUSOIDAL_RC_ORACLE_RESISTANCE: f64 = 1.0e3;
const XYCE_ANALYTIC_SINUSOIDAL_RC_ORACLE_CAPACITANCE: f64 = 2.0e-6;
const XYCE_ANALYTIC_SINUSOIDAL_RC_ORACLE_SOURCE_OFFSET: f64 = 0.0;
const XYCE_ANALYTIC_SINUSOIDAL_RC_ORACLE_SOURCE_AMPLITUDE: f64 = 1.0;
const XYCE_ANALYTIC_SINUSOIDAL_RC_ORACLE_FREQUENCY: f64 = 1.0e5;
// Not `std::f64::consts::PI`: this reproduces the Perl oracle's own 8-digit
// literal, and substituting a full-precision value moves the reference the
// suite is comparing against.
#[allow(clippy::approx_constant)]
const XYCE_ANALYTIC_SINUSOIDAL_RC_ORACLE_PI: f64 = 3.1415927;
const XYCE_ANALYTIC_SINUSOIDAL_RC_ORACLE_PRINT_OFFSET: f64 = 2.0e-3;
const XYCE_ANALYTIC_SINUSOIDAL_RC_ORACLE_STOP: f64 = 2.0e-4;
const XYCE_ANALYTIC_SINUSOIDAL_RC_TIMEINT_TOLERANCE: f64 = 1.0e-4;
const XYCE_ANALYTIC_SINUSOIDAL_RC_VERIFY_TOLERANCE: f64 = 1.0e-6;
const XYCE_ANALYTIC_FMOD_DC_RECORD: &str = "netlists/abm_nint_fmod/fmod.cir";
const XYCE_ANALYTIC_INT_FLOOR_CEIL_DC_RECORD: &str =
    "netlists/abm_int_floor_ceil/int_floor_ceil_bsrc.cir";
const XYCE_ANALYTIC_INT_FLOOR_CEIL_TRAN_RECORD: &str =
    "netlists/abm_int_floor_ceil/int_floor_ceil.cir";
const XYCE_MEASURE_CONT_STEP_NOISE_DERIV_RECORD: &str =
    "netlists/measure_cont/step/derivtestnoise.cir";
const XYCE_MEASURE_NOISE_STEP_DERIV_RECORD: &str = "netlists/measure_noise/step/derivtestnoise.cir";
const XYCE_RESISTOR_DTEMP_OWNER_RECORD: &str = "netlists/dtemp/res_dtemp.cir";
const XYCE_RESISTOR_DTEMP_REFERENCE_RECORD: &str = "netlists/dtemp/res_ref.cir";
const XYCE_BUG647_RESISTOR_OWNER_RECORD: &str =
    "netlists/certification_tests/bug_647_son/semic_resistor.cir";
const XYCE_BUG647_RESISTOR_REFERENCE_RECORD: &str =
    "netlists/certification_tests/bug_647_son/semic_resistor_modpar.cir";
const XYCE_BUG655_CONTINUATION_OWNER_RECORD: &str =
    "netlists/certification_tests/bug_655_son/contline.cir";
const XYCE_BUG655_CONTINUATION_REFERENCE_RECORD: &str =
    "netlists/certification_tests/bug_655_son/contline_with_spaces.cir";
const XYCE_BUG662_LONG_HEADER_OWNER_RECORD: &str =
    "netlists/certification_tests/bug_662/headerlinelengthmorethan256.cir";
const XYCE_BUG662_SHORT_HEADER_REFERENCE_RECORD: &str =
    "netlists/certification_tests/bug_662/headerlinelengthlessthan256.cir";
const XYCE_BUG667_NODESET_OWNER_RECORD: &str =
    "netlists/certification_tests/bug_667_son/nodeset_in_subckt.cir";
const XYCE_BUG667_NODESET_REFERENCE_RECORD: &str =
    "netlists/certification_tests/bug_667_son/nodeset_not_in_subckt.cir";
const XYCE_IC_MISSING_NODE_WARNING_RECORD: &str =
    "netlists/message/input/ic_at_missing_node_warning.cir";
const XYCE_IC_EMPTY_WARNING_RECORD: &str = "netlists/message/input/ic_no_args_warning.cir";
const XYCE_NODESET_MISSING_NODE_WARNING_RECORD: &str =
    "netlists/message/input/nodeset_at_missing_node_warning.cir";
const XYCE_NODESET_EMPTY_WARNING_RECORD: &str =
    "netlists/message/input/nodeset_no_args_warning.cir";
const XYCE_BUG667_SCOPED_GLOBAL_WARNING_RECORD: &str =
    "netlists/certification_tests/bug_667_son/ic_in_subckt_warning.cir";
const XYCE_IC_NODESET_CONFLICT_RECORD: &str = "netlists/message/input/ic_and_nodeset_specified.cir";

#[cfg(test)]
const XYCE_XDM_REPLACEGROUND_FAMILY_PREFIX: &str = "netlists/xdm/hspice/other_parsing/";
#[cfg(test)]
const XYCE_REMOVEUNUSED_FAMILY_PREFIX: &str = "netlists/redund_remove/";

// The four removed XDM wrappers declare abs=1e-5, rel=1e-3, and zero=1e-10,
// but `verifyXDMtranslation` never forwards those variables for HSPICE.  It
// invokes Release 7.10 `xyce_verify` without tolerance flags, so the effective
// comparison is the verifier default below.  Keep the declared values visible
// to prevent a future adapter from accidentally claiming that they governed
// the authoritative run.
const XYCE_XDM_REPLACEGROUND_DECLARED_ABSOLUTE_TOLERANCE: Value = 1.0e-5;
const XYCE_XDM_REPLACEGROUND_DECLARED_RELATIVE_TOLERANCE: Value = 1.0e-3;
const XYCE_XDM_REPLACEGROUND_DECLARED_ZERO_TOLERANCE: Value = 1.0e-10;

const XYCE_BUG754_GLOBAL_PARAMETER_OWNER_RECORD: &str =
    "netlists/certification_tests/bug_754_son/dcsweep_globalpar.cir";
const XYCE_BUG754_LITERAL_REFERENCE_RECORD: &str =
    "netlists/certification_tests/bug_754_son/dcsweep_nopar.cir";
const XYCE_BUG67_EXPECTED_FAILURE_RECORD: &str = "netlists/certification_tests/bug_67/bug_67.cir";
const XYCE_BUG671_EXPECTED_FAILURE_RECORD: &str =
    "netlists/certification_tests/bug_671/vpwl_binaryfile.cir";
const XYCE_BUG726_EXPECTED_FAILURE_RECORD: &str =
    "netlists/certification_tests/bug_726/adjacent.cir";
const XYCE_BUG744_EXPECTED_FAILURE_RECORD: &str =
    "netlists/certification_tests/bug_744/bad_dc_op.cir";
const XYCE_BUG75_EXPECTED_FAILURE_RECORD: &str =
    "netlists/certification_tests/bug_75_son/bug75.cir";
#[cfg(test)]
const XYCE_BUG1595_EXPECTED_FAILURE_PATH: &str =
    "Netlists/Certification_Tests/BUG_1595/bug1595.cir";
const XYCE_BUG1595_EXPECTED_FAILURE_RECORD: &str =
    "netlists/certification_tests/bug_1595/bug1595.cir";
const XYCE_BUG1595_EXPECTED_FAILURE_CONTRACT: &str =
    "expected_failure_bug1595_hierarchical_mutual_inductor_reference_parse";
const XYCE_BUG1595_HISTORICAL_TIMEOUT_MS: u128 = 30_000;
const XYCE_BUG1148_EXPECTED_FAILURE_RECORD: &str =
    "netlists/certification_tests/bug_1148/bug_1148.cir";
const XYCE_BUG40_EXPECTED_FAILURE_RECORD: &str = "netlists/certification_tests/bug_40/bug_40.cir";
const XYCE_BUG718_INVALID_NODES_EXPECTED_FAILURE_RECORD: &str =
    "netlists/certification_tests/bug_718_son/invalidnodes.cir";
const XYCE_MESSAGE_PRINT_BAD_NODENAME_EXPECTED_FAILURE_RECORD: &str =
    "netlists/message/print/bad_nodename.cir";
const XYCE_MESSAGE_PRINT_BAD_VARIABLE_EXPECTED_FAILURE_RECORD: &str =
    "netlists/message/print/bad_variable.cir";
const XYCE_LEAD_CURRENTS_INVALID_DEVICE_EXPECTED_FAILURE_RECORD: &str =
    "netlists/lead_currents/lead_for_invalid_device.cir";
const XYCE_MEASURE_INVALID_NODES_EXPECTED_FAILURE_RECORD: &str =
    "netlists/measure/invalid_nodes.cir";
const XYCE_FOURIER_BAD_LINE3_EXPECTED_FAILURE_RECORD: &str =
    "netlists/fourier/bad_dot_four_line3.cir";
const XYCE_BUG387_EXPECTED_FAILURE_RECORD: &str =
    "netlists/certification_tests/bug_387_son/bug_387.cir";
const XYCE_SUBCKT_NONAME_EXPECTED_FAILURE_RECORD: &str =
    "netlists/message/subcircuit/subckt_noname.cir";
const XYCE_SUBCKT_MISSING_ENDS_END_CARD_EXPECTED_FAILURE_RECORD: &str =
    "netlists/message/subcircuit/subckt_missing_ends.cir";
const XYCE_SUBCKT_MISSING_ENDS_INCLUDE_EOF_EXPECTED_FAILURE_RECORD: &str =
    "netlists/message/subcircuit/subckt_missing_ends2.cir";
const XYCE_SUBCKT_MISSING_ENDS_TOPLEVEL_EOF_EXPECTED_FAILURE_RECORD: &str =
    "netlists/message/subcircuit/subckt_missing_ends3.cir";
const XYCE_SUBCKT_MISSING_ENDS_TS_INV_EOF_EXPECTED_FAILURE_RECORD: &str =
    "netlists/message/subcircuit/subckt_missing_ends4.cir";
const XYCE_SUBCKT_A2_DUP_BINDING_EXPECTED_FAILURE_RECORD: &str =
    "netlists/message/subcircuit/subckt_a2_dup_error.cir";
const XYCE_SUBCKT_J1_DUP_BINDING_EXPECTED_FAILURE_RECORD: &str =
    "netlists/message/subcircuit/subckt_j1_dup_error.cir";
const XYCE_DC_EXCESS_ARGS_EXPECTED_FAILURE_RECORD: &str =
    "netlists/message/input/dc_excessargs.cir";
const XYCE_AC_UNSUPPORTED_SWEEP_EXPECTED_FAILURE_RECORD: &str =
    "netlists/message/input/ac_setupsweepparam.cir";
const XYCE_NOISE_UNSUPPORTED_SWEEP_EXPECTED_FAILURE_RECORD: &str =
    "netlists/message/input/noise_setupsweepparam.cir";
const XYCE_MESSAGE_MISSING_LIBRARY_ENDL_EXPECTED_FAILURE_RECORD: &str =
    "netlists/message/input/circuitblock_parseincludefile_2a.cir";
const XYCE_MESSAGE_MISSING_LIBRARY_FILE_UNQUOTED_EXPECTED_FAILURE_RECORD: &str =
    "netlists/message/input/circuitblock_parseincludefile_2b.cir";
const XYCE_MESSAGE_MISSING_LIBRARY_FILE_QUOTED_EXPECTED_FAILURE_RECORD: &str =
    "netlists/message/input/circuitblock_parseincludefile_2c.cir";
const XYCE_MESSAGE_DUPLICATE_DEVICE_EXPECTED_FAILURE_RECORD: &str =
    "netlists/message/device/circuitblock_addtabledata_1.cir";
const XYCE_MESSAGE_MISSING_DEVICE_NODES_EXPECTED_FAILURE_RECORD: &str =
    "netlists/message/device/deviceblock_extractnodes_1.cir";
const XYCE_BUG702_DUP_EXTERNAL_EXPECTED_FAILURE_RECORD: &str =
    "netlists/certification_tests/bug_702/dup-external.cir";
const XYCE_BUG702_DUP_INLINED_EXPECTED_FAILURE_RECORD: &str =
    "netlists/certification_tests/bug_702/dup-inlined.cir";
const XYCE_BUG702_EMPTY_INITCOND_EXPECTED_FAILURE_RECORD: &str =
    "netlists/certification_tests/bug_702/empty-initcond.cir";
const XYCE_BUG702_MISSING_INITCOND_EXPECTED_FAILURE_RECORD: &str =
    "netlists/certification_tests/bug_702/missing-initcond.cir";
const XYCE_BUG702_EXTERNAL_POSITIVE_RECORD: &str =
    "netlists/certification_tests/bug_702/external.cir";
const XYCE_BUG702_INLINED_MULTIPLE_POSITIVE_RECORD: &str =
    "netlists/certification_tests/bug_702/inlined-multiple.cir";
const XYCE_BUG702_INLINED_SINGLE_POSITIVE_RECORD: &str =
    "netlists/certification_tests/bug_702/inlined-single.cir";
const XYCE_BUG702_PRECEDENCE_POSITIVE_RECORD: &str =
    "netlists/certification_tests/bug_702/precedence.cir";
const XYCE_ISSUE455_EXPECTED_FAILURE_RECORD: &str =
    "netlists/certification_tests/issue_455/issue455.cir";
const XYCE_BUG204_EXPECTED_FAILURE_RECORD: &str = "netlists/certification_tests/bug_204/bug204.cir";
const XYCE_BUG281_EXPECTED_FAILURE_RECORD: &str =
    "netlists/certification_tests/bug_281/bug_281.cir";
const XYCE_BUG401_BAD_DEVICE_EXPECTED_FAILURE_RECORD: &str =
    "netlists/certification_tests/bug_401/bad-device-line.cir";
const XYCE_BUG401_EXTRA_SPACE_EXPECTED_FAILURE_RECORD: &str =
    "netlists/certification_tests/bug_401/extra-space.cir";
const XYCE_BUG401_WORSE_DEVICE_EXPECTED_FAILURE_RECORD: &str =
    "netlists/certification_tests/bug_401/worse-device-line.cir";
const XYCE_BUG701_TOPLEVEL_EXPECTED_FAILURE_RECORD: &str =
    "netlists/certification_tests/bug_701/dup-toplevel.cir";
const XYCE_BUG701_SUBCIRCUIT_EXPECTED_FAILURE_RECORD: &str =
    "netlists/certification_tests/bug_701/dup-subcircuit.cir";
const XYCE_BUG769_NODE_VOLTAGE_EXPECTED_FAILURE_RECORD: &str =
    "netlists/certification_tests/bug_769/bug_769a.cir";
const XYCE_BUG769_DEVICE_CURRENT_EXPECTED_FAILURE_RECORD: &str =
    "netlists/certification_tests/bug_769/bug_769b.cir";
const XYCE_BUG769_LEAD_CURRENT_EXPECTED_FAILURE_RECORD: &str =
    "netlists/certification_tests/bug_769/bug_769c.cir";
const XYCE_BUG1578_EXPECTED_FAILURE_RECORD: &str =
    "netlists/certification_tests/bug_1578/bug_1578.cir";
const XYCE_BUG198_EXPECTED_FAILURE_RECORD: &str =
    "netlists/certification_tests/bug_198/bug_198.cir";
const XYCE_BUG258_EXPECTED_FAILURE_RECORD: &str =
    "netlists/certification_tests/bug_258/bug_198.cir";
const XYCE_BUG587_EXPECTED_FAILURE_RECORD: &str = "netlists/certification_tests/bug_587/587.cir";
const XYCE_BUG671_OLE_MAGIC: [u8; 8] = [0xd0, 0xcf, 0x11, 0xe0, 0xa1, 0xb1, 0x1a, 0xe1];
const XYCE_BUG662_CANONICAL_LONG_TITLE: &str = r#"** Converted using XDM 0.20rc from /home/rrlober/xdmwork/xdm/data-model/src/python/test/unit/resources/pspice_9_1.xml to /home/rrlober/xdmwork/xdm/data-model/src/python/test/unit/resources/xyce_6_3.xml ** ** Profile: "SCHEMATIC1-bias"  [ H:\Xyce\PSpice\Netlists\TransmissionLine-PSpiceFiles\SCHEMATIC1\bias.sim ]"#;
const XYCE_BUG655_CANONICAL_OWNER_SOURCE: &str = "*** Simple amplifier ***\n\
\n\
vcc 1 0 dc 12V\n\
i1 2 0 dc 0A\n\
\n\
r1 1 3 5k\n\
r2 2 3 20k\n\
\n\
q1 3 2 0 2n3510\n\
\n\
.model 2n3510 npn\n\
+ bf=100 br=1.35e-4 xtb=1.5 is=8.35e-14 eg=1.11 cjc=9.63e-12\n\
+ cje=9.47e-12 rb=16.7 rc=1.66 vaf=90 tf=1e-10 tr=1.27e-4\n\
+ cjs=1e-15 vjs=0.8 mjs=0.5 var=100 ise=4.77e-11 isc=1e-16\n\
+ ikf=0.18 ikr=1000 irb=1 rbm=0 vtf=1000\n\
\n\
.DC i1 -100uA 100uA 10uA\n\
\n\
.PRINT DC I(I1) V(3)\n\
\n\
.end\n";
const XYCE_PWL_REPEAT_VALUE_ERROR: &str =
    "PWL source repeat value (R) must be >= 0 and < last value in time-voltage list";

/// Removed-wrapper startup contracts whose observable is a diagnostic rather
/// than a numeric output artifact. Warning contracts still execute the full
/// native transient and require a successful simulation; the conflict
/// contract requires the typed Xyce parse failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceStartupOracleKind {
    IcMissingNodeWarning,
    IcEmptyWarning,
    NodeSetMissingNodeWarning,
    NodeSetEmptyWarning,
    Bug667ScopedGlobalWarning,
    IcNodeSetConflict,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct XyceStartupWarningExpectation {
    directive: StartupDirectiveKind,
    code: StartupDiagnosticCode,
    stage: StartupDiagnosticStage,
    line: usize,
    canonical_nodes: &'static [&'static str],
    ordered_upstream_patterns: &'static [&'static str],
}

impl XyceStartupOracleKind {
    fn for_record(relative_path: &str) -> Option<Self> {
        match XyceTestRunner::normalize_manifest_key(relative_path).as_str() {
            XYCE_IC_MISSING_NODE_WARNING_RECORD => Some(Self::IcMissingNodeWarning),
            XYCE_IC_EMPTY_WARNING_RECORD => Some(Self::IcEmptyWarning),
            XYCE_NODESET_MISSING_NODE_WARNING_RECORD => Some(Self::NodeSetMissingNodeWarning),
            XYCE_NODESET_EMPTY_WARNING_RECORD => Some(Self::NodeSetEmptyWarning),
            XYCE_BUG667_SCOPED_GLOBAL_WARNING_RECORD => Some(Self::Bug667ScopedGlobalWarning),
            XYCE_IC_NODESET_CONFLICT_RECORD => Some(Self::IcNodeSetConflict),
            _ => None,
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::IcMissingNodeWarning => XYCE_IC_MISSING_NODE_WARNING_RECORD,
            Self::IcEmptyWarning => XYCE_IC_EMPTY_WARNING_RECORD,
            Self::NodeSetMissingNodeWarning => XYCE_NODESET_MISSING_NODE_WARNING_RECORD,
            Self::NodeSetEmptyWarning => XYCE_NODESET_EMPTY_WARNING_RECORD,
            Self::Bug667ScopedGlobalWarning => XYCE_BUG667_SCOPED_GLOBAL_WARNING_RECORD,
            Self::IcNodeSetConflict => XYCE_IC_NODESET_CONFLICT_RECORD,
        }
    }

    fn result_contract(self) -> &'static str {
        match self {
            Self::IcMissingNodeWarning => "expected_warning_ic_undefined_node_success",
            Self::IcEmptyWarning => "expected_warning_ic_empty_success",
            Self::NodeSetMissingNodeWarning => "expected_warning_nodeset_undefined_node_success",
            Self::NodeSetEmptyWarning => "expected_warning_nodeset_empty_success",
            Self::Bug667ScopedGlobalWarning => "expected_warning_bug667_scoped_global_ic_success",
            Self::IcNodeSetConflict => "expected_failure_message_ic_nodeset_conflict_parse",
        }
    }

    fn warning_expectation(self) -> Option<XyceStartupWarningExpectation> {
        const MISSING_PATTERNS: &[&str] = &[
            "Netlist warning: Initial conditions specified at nodes not present in circuit.",
            "May be error in .IC or .NODESET line. Ignoring nodes:",
            "BLEEM",
        ];
        match self {
            Self::IcMissingNodeWarning => Some(XyceStartupWarningExpectation {
                directive: StartupDirectiveKind::Ic,
                code: StartupDiagnosticCode::UndefinedNode,
                stage: StartupDiagnosticStage::StartupTopology,
                line: 16,
                canonical_nodes: &["BLEEM"],
                ordered_upstream_patterns: MISSING_PATTERNS,
            }),
            Self::IcEmptyWarning => Some(XyceStartupWarningExpectation {
                directive: StartupDirectiveKind::Ic,
                code: StartupDiagnosticCode::EmptyDirective,
                stage: StartupDiagnosticStage::Parse,
                line: 15,
                canonical_nodes: &[],
                ordered_upstream_patterns: &[
                    "Netlist warning in file IC_No_Args_Warning.cir",
                    "Ignored .IC and/or .DCVOLT, no arguments provided.",
                ],
            }),
            Self::NodeSetMissingNodeWarning => Some(XyceStartupWarningExpectation {
                directive: StartupDirectiveKind::NodeSet,
                code: StartupDiagnosticCode::UndefinedNode,
                stage: StartupDiagnosticStage::StartupTopology,
                line: 16,
                canonical_nodes: &["BLEEM"],
                ordered_upstream_patterns: MISSING_PATTERNS,
            }),
            Self::NodeSetEmptyWarning => Some(XyceStartupWarningExpectation {
                directive: StartupDirectiveKind::NodeSet,
                code: StartupDiagnosticCode::EmptyDirective,
                stage: StartupDiagnosticStage::Parse,
                line: 15,
                canonical_nodes: &[],
                ordered_upstream_patterns: &[
                    "Netlist warning in file NODESET_No_Args_Warning.cir",
                    "Ignored .NODESET, no arguments provided.",
                ],
            }),
            Self::Bug667ScopedGlobalWarning => Some(XyceStartupWarningExpectation {
                directive: StartupDirectiveKind::Ic,
                code: StartupDiagnosticCode::ScopedGlobalNode,
                stage: StartupDiagnosticStage::Parse,
                line: 31,
                canonical_nodes: &["$G_VCC"],
                ordered_upstream_patterns: &[
                    "Ignored .IC and/or .DCVOLT on global node",
                    "move statement to global scope",
                ],
            }),
            Self::IcNodeSetConflict => None,
        }
    }

    fn conflict_error_policy(self) -> Option<XyceUpstreamExpectedErrorPolicy> {
        (self == Self::IcNodeSetConflict).then_some(
            XyceUpstreamExpectedErrorPolicy::NonzeroExitWithOrderedPatterns {
                search_streams: XyceUpstreamErrorSearchStreams::EitherCompleteStdoutOrStderr,
                ordered_patterns: &["Cannot set both .IC and .NODESET simultaneously"],
            },
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceExpectedFailureKind {
    Bug67BehavioralExpression,
    Bug671InvalidPwlFile,
    Bug726AdjacentCouplings,
    Bug744DcOperatingPoint,
    Bug75UndefinedMutualInductorReference,
    Bug1595HierarchicalMutualInductorReference,
    Bug1148UndefinedPrintNode,
    Bug40UndefinedPrintNode,
    Bug718InvalidPrintNodes,
    MessagePrintBadNodeName,
    MessagePrintBadVariable,
    LeadCurrentsInvalidDevice,
    MeasureInvalidNodes,
    FourierBadLine3OutputSymbols,
    Bug387MissingLibraryEndl,
    MessageSubcircuitMissingName,
    MessageSubcircuitMissingEndsEndCard,
    MessageSubcircuitMissingEndsIncludeEof,
    MessageSubcircuitMissingEndsTopLevelEof,
    MessageSubcircuitMissingEndsTsInvEof,
    MessageSubcircuitDuplicateBindingA2,
    MessageSubcircuitDuplicateBindingJ1,
    MessageDcExcessArguments,
    MessageAcUnsupportedSweepType,
    MessageNoiseUnsupportedSweepType,
    MessageMissingLibraryEndl,
    MessageMissingLibraryFileUnquoted,
    MessageMissingLibraryFileQuoted,
    MessageDuplicateDevice,
    MessageMissingDeviceNodes,
    Bug702DuplicateExternalInitcond,
    Bug702DuplicateInlinedInitcond,
    Bug702MalformedInitcondFile,
    Bug702MissingInitcondFile,
    Issue455DuplicateDcSourceFunction,
    Bug204InvalidDcSweepArity,
    Bug281InvalidDcSweepArity,
    Bug354BadFunction,
    Bug354BadLeadCurrent,
    Bug354BadParameter,
    Bug401BadDeviceLine,
    Bug401ExtraSpace,
    Bug401WorseDeviceLine,
    Bug701DuplicateTopLevelDevice,
    Bug701DuplicateSubcircuitDevice,
    Bug769ParameterNodeVoltage,
    Bug769ParameterDeviceCurrent,
    Bug769ParameterLeadCurrent,
    Bug1578InvalidDeviceType,
    Bug198UnrecognizedLine,
    Bug258UnrecognizedLine,
    Bug587InvalidNumericNotation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct XyceExpectedOutputSymbol {
    directive: OutputDirectiveKind,
    operator: &'static str,
    symbol: &'static str,
    kind: OutputSymbolKind,
    file_name: &'static str,
    line: usize,
}

impl XyceExpectedOutputSymbol {
    fn identifier(self) -> String {
        format!(
            "{}|{}|{}|{}|{}:{}",
            self.directive, self.operator, self.symbol, self.kind, self.file_name, self.line
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceBug702PositiveKind {
    External,
    InlinedMultiple,
    InlinedSingle,
    Precedence,
}

impl XyceBug702PositiveKind {
    fn for_record(relative_path: &str) -> Option<Self> {
        match XyceTestRunner::normalize_manifest_key(relative_path).as_str() {
            XYCE_BUG702_EXTERNAL_POSITIVE_RECORD => Some(Self::External),
            XYCE_BUG702_INLINED_MULTIPLE_POSITIVE_RECORD => Some(Self::InlinedMultiple),
            XYCE_BUG702_INLINED_SINGLE_POSITIVE_RECORD => Some(Self::InlinedSingle),
            XYCE_BUG702_PRECEDENCE_POSITIVE_RECORD => Some(Self::Precedence),
            _ => None,
        }
    }

    fn result_contract(self) -> &'static str {
        match self {
            Self::External => "bug702_external_initcond_alias_tran",
            Self::InlinedMultiple => "bug702_inlined_multiple_initcond_alias_tran",
            Self::InlinedSingle => "bug702_inlined_single_initcond_alias_tran",
            Self::Precedence => "bug702_initcond_precedence_alias_tran",
        }
    }

    fn canonical_source_record(self) -> &'static str {
        match self {
            Self::InlinedMultiple => "Netlists/BUG_174/nlrcs10.cir",
            Self::External | Self::InlinedSingle | Self::Precedence => {
                "Netlists/INIT_CONDS/inv1xIC.cir"
            }
        }
    }

    fn alias_reference_name(self) -> &'static str {
        match self {
            Self::InlinedMultiple => "nlrcs10.cir.prn",
            Self::External | Self::InlinedSingle | Self::Precedence => "inv1xIC.cir.prn",
        }
    }

    fn scientific_precision(self) -> usize {
        match self {
            Self::InlinedMultiple => XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
            Self::External | Self::InlinedSingle | Self::Precedence => 10,
        }
    }
}

#[derive(Debug, Clone)]
struct XyceBug702PositiveContract {
    kind: XyceBug702PositiveKind,
    netlist: Netlist,
    plan: XyceStaticTranPlan,
}

impl XyceExpectedFailureKind {
    fn for_record(relative_path: &str) -> Option<Self> {
        match XyceTestRunner::normalize_manifest_key(relative_path).as_str() {
            XYCE_BUG67_EXPECTED_FAILURE_RECORD => Some(Self::Bug67BehavioralExpression),
            XYCE_BUG671_EXPECTED_FAILURE_RECORD => Some(Self::Bug671InvalidPwlFile),
            XYCE_BUG726_EXPECTED_FAILURE_RECORD => Some(Self::Bug726AdjacentCouplings),
            XYCE_BUG744_EXPECTED_FAILURE_RECORD => Some(Self::Bug744DcOperatingPoint),
            XYCE_BUG75_EXPECTED_FAILURE_RECORD => Some(Self::Bug75UndefinedMutualInductorReference),
            XYCE_BUG1595_EXPECTED_FAILURE_RECORD => {
                Some(Self::Bug1595HierarchicalMutualInductorReference)
            }
            XYCE_BUG1148_EXPECTED_FAILURE_RECORD => Some(Self::Bug1148UndefinedPrintNode),
            XYCE_BUG40_EXPECTED_FAILURE_RECORD => Some(Self::Bug40UndefinedPrintNode),
            XYCE_BUG718_INVALID_NODES_EXPECTED_FAILURE_RECORD => {
                Some(Self::Bug718InvalidPrintNodes)
            }
            XYCE_MESSAGE_PRINT_BAD_NODENAME_EXPECTED_FAILURE_RECORD => {
                Some(Self::MessagePrintBadNodeName)
            }
            XYCE_MESSAGE_PRINT_BAD_VARIABLE_EXPECTED_FAILURE_RECORD => {
                Some(Self::MessagePrintBadVariable)
            }
            XYCE_LEAD_CURRENTS_INVALID_DEVICE_EXPECTED_FAILURE_RECORD => {
                Some(Self::LeadCurrentsInvalidDevice)
            }
            XYCE_MEASURE_INVALID_NODES_EXPECTED_FAILURE_RECORD => Some(Self::MeasureInvalidNodes),
            XYCE_FOURIER_BAD_LINE3_EXPECTED_FAILURE_RECORD => {
                Some(Self::FourierBadLine3OutputSymbols)
            }
            XYCE_BUG387_EXPECTED_FAILURE_RECORD => Some(Self::Bug387MissingLibraryEndl),
            XYCE_SUBCKT_NONAME_EXPECTED_FAILURE_RECORD => Some(Self::MessageSubcircuitMissingName),
            XYCE_SUBCKT_MISSING_ENDS_END_CARD_EXPECTED_FAILURE_RECORD => {
                Some(Self::MessageSubcircuitMissingEndsEndCard)
            }
            XYCE_SUBCKT_MISSING_ENDS_INCLUDE_EOF_EXPECTED_FAILURE_RECORD => {
                Some(Self::MessageSubcircuitMissingEndsIncludeEof)
            }
            XYCE_SUBCKT_MISSING_ENDS_TOPLEVEL_EOF_EXPECTED_FAILURE_RECORD => {
                Some(Self::MessageSubcircuitMissingEndsTopLevelEof)
            }
            XYCE_SUBCKT_MISSING_ENDS_TS_INV_EOF_EXPECTED_FAILURE_RECORD => {
                Some(Self::MessageSubcircuitMissingEndsTsInvEof)
            }
            XYCE_SUBCKT_A2_DUP_BINDING_EXPECTED_FAILURE_RECORD => {
                Some(Self::MessageSubcircuitDuplicateBindingA2)
            }
            XYCE_SUBCKT_J1_DUP_BINDING_EXPECTED_FAILURE_RECORD => {
                Some(Self::MessageSubcircuitDuplicateBindingJ1)
            }
            XYCE_DC_EXCESS_ARGS_EXPECTED_FAILURE_RECORD => Some(Self::MessageDcExcessArguments),
            XYCE_AC_UNSUPPORTED_SWEEP_EXPECTED_FAILURE_RECORD => {
                Some(Self::MessageAcUnsupportedSweepType)
            }
            XYCE_NOISE_UNSUPPORTED_SWEEP_EXPECTED_FAILURE_RECORD => {
                Some(Self::MessageNoiseUnsupportedSweepType)
            }
            XYCE_MESSAGE_MISSING_LIBRARY_ENDL_EXPECTED_FAILURE_RECORD => {
                Some(Self::MessageMissingLibraryEndl)
            }
            XYCE_MESSAGE_MISSING_LIBRARY_FILE_UNQUOTED_EXPECTED_FAILURE_RECORD => {
                Some(Self::MessageMissingLibraryFileUnquoted)
            }
            XYCE_MESSAGE_MISSING_LIBRARY_FILE_QUOTED_EXPECTED_FAILURE_RECORD => {
                Some(Self::MessageMissingLibraryFileQuoted)
            }
            XYCE_MESSAGE_DUPLICATE_DEVICE_EXPECTED_FAILURE_RECORD => {
                Some(Self::MessageDuplicateDevice)
            }
            XYCE_MESSAGE_MISSING_DEVICE_NODES_EXPECTED_FAILURE_RECORD => {
                Some(Self::MessageMissingDeviceNodes)
            }
            XYCE_BUG702_DUP_EXTERNAL_EXPECTED_FAILURE_RECORD => {
                Some(Self::Bug702DuplicateExternalInitcond)
            }
            XYCE_BUG702_DUP_INLINED_EXPECTED_FAILURE_RECORD => {
                Some(Self::Bug702DuplicateInlinedInitcond)
            }
            XYCE_BUG702_EMPTY_INITCOND_EXPECTED_FAILURE_RECORD => {
                Some(Self::Bug702MalformedInitcondFile)
            }
            XYCE_BUG702_MISSING_INITCOND_EXPECTED_FAILURE_RECORD => {
                Some(Self::Bug702MissingInitcondFile)
            }
            XYCE_ISSUE455_EXPECTED_FAILURE_RECORD => Some(Self::Issue455DuplicateDcSourceFunction),
            XYCE_BUG204_EXPECTED_FAILURE_RECORD => Some(Self::Bug204InvalidDcSweepArity),
            XYCE_BUG281_EXPECTED_FAILURE_RECORD => Some(Self::Bug281InvalidDcSweepArity),
            XYCE_BUG354_FUNCTION_RECORD => Some(Self::Bug354BadFunction),
            XYCE_BUG354_LEAD_CURRENT_RECORD => Some(Self::Bug354BadLeadCurrent),
            XYCE_BUG354_PARAMETER_RECORD => Some(Self::Bug354BadParameter),
            XYCE_BUG401_BAD_DEVICE_EXPECTED_FAILURE_RECORD => Some(Self::Bug401BadDeviceLine),
            XYCE_BUG401_EXTRA_SPACE_EXPECTED_FAILURE_RECORD => Some(Self::Bug401ExtraSpace),
            XYCE_BUG401_WORSE_DEVICE_EXPECTED_FAILURE_RECORD => Some(Self::Bug401WorseDeviceLine),
            XYCE_BUG701_TOPLEVEL_EXPECTED_FAILURE_RECORD => {
                Some(Self::Bug701DuplicateTopLevelDevice)
            }
            XYCE_BUG701_SUBCIRCUIT_EXPECTED_FAILURE_RECORD => {
                Some(Self::Bug701DuplicateSubcircuitDevice)
            }
            XYCE_BUG769_NODE_VOLTAGE_EXPECTED_FAILURE_RECORD => {
                Some(Self::Bug769ParameterNodeVoltage)
            }
            XYCE_BUG769_DEVICE_CURRENT_EXPECTED_FAILURE_RECORD => {
                Some(Self::Bug769ParameterDeviceCurrent)
            }
            XYCE_BUG769_LEAD_CURRENT_EXPECTED_FAILURE_RECORD => {
                Some(Self::Bug769ParameterLeadCurrent)
            }
            XYCE_BUG1578_EXPECTED_FAILURE_RECORD => Some(Self::Bug1578InvalidDeviceType),
            XYCE_BUG198_EXPECTED_FAILURE_RECORD => Some(Self::Bug198UnrecognizedLine),
            XYCE_BUG258_EXPECTED_FAILURE_RECORD => Some(Self::Bug258UnrecognizedLine),
            XYCE_BUG587_EXPECTED_FAILURE_RECORD => Some(Self::Bug587InvalidNumericNotation),
            _ => None,
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::Bug67BehavioralExpression => XYCE_BUG67_EXPECTED_FAILURE_RECORD,
            Self::Bug671InvalidPwlFile => XYCE_BUG671_EXPECTED_FAILURE_RECORD,
            Self::Bug726AdjacentCouplings => XYCE_BUG726_EXPECTED_FAILURE_RECORD,
            Self::Bug744DcOperatingPoint => XYCE_BUG744_EXPECTED_FAILURE_RECORD,
            Self::Bug75UndefinedMutualInductorReference => XYCE_BUG75_EXPECTED_FAILURE_RECORD,
            Self::Bug1595HierarchicalMutualInductorReference => {
                XYCE_BUG1595_EXPECTED_FAILURE_RECORD
            }
            Self::Bug1148UndefinedPrintNode => XYCE_BUG1148_EXPECTED_FAILURE_RECORD,
            Self::Bug40UndefinedPrintNode => XYCE_BUG40_EXPECTED_FAILURE_RECORD,
            Self::Bug718InvalidPrintNodes => XYCE_BUG718_INVALID_NODES_EXPECTED_FAILURE_RECORD,
            Self::MessagePrintBadNodeName => {
                XYCE_MESSAGE_PRINT_BAD_NODENAME_EXPECTED_FAILURE_RECORD
            }
            Self::MessagePrintBadVariable => {
                XYCE_MESSAGE_PRINT_BAD_VARIABLE_EXPECTED_FAILURE_RECORD
            }
            Self::LeadCurrentsInvalidDevice => {
                XYCE_LEAD_CURRENTS_INVALID_DEVICE_EXPECTED_FAILURE_RECORD
            }
            Self::MeasureInvalidNodes => XYCE_MEASURE_INVALID_NODES_EXPECTED_FAILURE_RECORD,
            Self::FourierBadLine3OutputSymbols => XYCE_FOURIER_BAD_LINE3_EXPECTED_FAILURE_RECORD,
            Self::Bug387MissingLibraryEndl => XYCE_BUG387_EXPECTED_FAILURE_RECORD,
            Self::MessageSubcircuitMissingName => XYCE_SUBCKT_NONAME_EXPECTED_FAILURE_RECORD,
            Self::MessageSubcircuitMissingEndsEndCard => {
                XYCE_SUBCKT_MISSING_ENDS_END_CARD_EXPECTED_FAILURE_RECORD
            }
            Self::MessageSubcircuitMissingEndsIncludeEof => {
                XYCE_SUBCKT_MISSING_ENDS_INCLUDE_EOF_EXPECTED_FAILURE_RECORD
            }
            Self::MessageSubcircuitMissingEndsTopLevelEof => {
                XYCE_SUBCKT_MISSING_ENDS_TOPLEVEL_EOF_EXPECTED_FAILURE_RECORD
            }
            Self::MessageSubcircuitMissingEndsTsInvEof => {
                XYCE_SUBCKT_MISSING_ENDS_TS_INV_EOF_EXPECTED_FAILURE_RECORD
            }
            Self::MessageSubcircuitDuplicateBindingA2 => {
                XYCE_SUBCKT_A2_DUP_BINDING_EXPECTED_FAILURE_RECORD
            }
            Self::MessageSubcircuitDuplicateBindingJ1 => {
                XYCE_SUBCKT_J1_DUP_BINDING_EXPECTED_FAILURE_RECORD
            }
            Self::MessageDcExcessArguments => XYCE_DC_EXCESS_ARGS_EXPECTED_FAILURE_RECORD,
            Self::MessageAcUnsupportedSweepType => {
                XYCE_AC_UNSUPPORTED_SWEEP_EXPECTED_FAILURE_RECORD
            }
            Self::MessageNoiseUnsupportedSweepType => {
                XYCE_NOISE_UNSUPPORTED_SWEEP_EXPECTED_FAILURE_RECORD
            }
            Self::MessageMissingLibraryEndl => {
                XYCE_MESSAGE_MISSING_LIBRARY_ENDL_EXPECTED_FAILURE_RECORD
            }
            Self::MessageMissingLibraryFileUnquoted => {
                XYCE_MESSAGE_MISSING_LIBRARY_FILE_UNQUOTED_EXPECTED_FAILURE_RECORD
            }
            Self::MessageMissingLibraryFileQuoted => {
                XYCE_MESSAGE_MISSING_LIBRARY_FILE_QUOTED_EXPECTED_FAILURE_RECORD
            }
            Self::MessageDuplicateDevice => XYCE_MESSAGE_DUPLICATE_DEVICE_EXPECTED_FAILURE_RECORD,
            Self::MessageMissingDeviceNodes => {
                XYCE_MESSAGE_MISSING_DEVICE_NODES_EXPECTED_FAILURE_RECORD
            }
            Self::Bug702DuplicateExternalInitcond => {
                XYCE_BUG702_DUP_EXTERNAL_EXPECTED_FAILURE_RECORD
            }
            Self::Bug702DuplicateInlinedInitcond => XYCE_BUG702_DUP_INLINED_EXPECTED_FAILURE_RECORD,
            Self::Bug702MalformedInitcondFile => XYCE_BUG702_EMPTY_INITCOND_EXPECTED_FAILURE_RECORD,
            Self::Bug702MissingInitcondFile => XYCE_BUG702_MISSING_INITCOND_EXPECTED_FAILURE_RECORD,
            Self::Issue455DuplicateDcSourceFunction => XYCE_ISSUE455_EXPECTED_FAILURE_RECORD,
            Self::Bug204InvalidDcSweepArity => XYCE_BUG204_EXPECTED_FAILURE_RECORD,
            Self::Bug281InvalidDcSweepArity => XYCE_BUG281_EXPECTED_FAILURE_RECORD,
            Self::Bug354BadFunction => XYCE_BUG354_FUNCTION_RECORD,
            Self::Bug354BadLeadCurrent => XYCE_BUG354_LEAD_CURRENT_RECORD,
            Self::Bug354BadParameter => XYCE_BUG354_PARAMETER_RECORD,
            Self::Bug401BadDeviceLine => XYCE_BUG401_BAD_DEVICE_EXPECTED_FAILURE_RECORD,
            Self::Bug401ExtraSpace => XYCE_BUG401_EXTRA_SPACE_EXPECTED_FAILURE_RECORD,
            Self::Bug401WorseDeviceLine => XYCE_BUG401_WORSE_DEVICE_EXPECTED_FAILURE_RECORD,
            Self::Bug701DuplicateTopLevelDevice => XYCE_BUG701_TOPLEVEL_EXPECTED_FAILURE_RECORD,
            Self::Bug701DuplicateSubcircuitDevice => XYCE_BUG701_SUBCIRCUIT_EXPECTED_FAILURE_RECORD,
            Self::Bug769ParameterNodeVoltage => XYCE_BUG769_NODE_VOLTAGE_EXPECTED_FAILURE_RECORD,
            Self::Bug769ParameterDeviceCurrent => {
                XYCE_BUG769_DEVICE_CURRENT_EXPECTED_FAILURE_RECORD
            }
            Self::Bug769ParameterLeadCurrent => XYCE_BUG769_LEAD_CURRENT_EXPECTED_FAILURE_RECORD,
            Self::Bug1578InvalidDeviceType => XYCE_BUG1578_EXPECTED_FAILURE_RECORD,
            Self::Bug198UnrecognizedLine => XYCE_BUG198_EXPECTED_FAILURE_RECORD,
            Self::Bug258UnrecognizedLine => XYCE_BUG258_EXPECTED_FAILURE_RECORD,
            Self::Bug587InvalidNumericNotation => XYCE_BUG587_EXPECTED_FAILURE_RECORD,
        }
    }

    fn result_contract(self) -> &'static str {
        match self {
            Self::Bug67BehavioralExpression => "expected_failure_behavioral_expression_build",
            Self::Bug671InvalidPwlFile => "expected_failure_external_pwl_load",
            Self::Bug726AdjacentCouplings => "expected_failure_adjacent_coupling_parse",
            Self::Bug744DcOperatingPoint => "expected_failure_dc_operating_point",
            Self::Bug75UndefinedMutualInductorReference => {
                "expected_failure_bug75_undefined_mutual_inductor_reference_parse"
            }
            Self::Bug1595HierarchicalMutualInductorReference => {
                XYCE_BUG1595_EXPECTED_FAILURE_CONTRACT
            }
            Self::Bug1148UndefinedPrintNode => {
                "expected_failure_bug1148_undefined_print_node_parse"
            }
            Self::Bug40UndefinedPrintNode => "expected_failure_bug40_undefined_print_node_parse",
            Self::Bug718InvalidPrintNodes => "expected_failure_bug718_invalid_print_nodes_parse",
            Self::MessagePrintBadNodeName => "expected_failure_message_print_bad_nodename_parse",
            Self::MessagePrintBadVariable => "expected_failure_message_print_bad_variable_parse",
            Self::LeadCurrentsInvalidDevice => {
                "expected_failure_lead_currents_invalid_device_parse"
            }
            Self::MeasureInvalidNodes => "expected_failure_measure_invalid_nodes_parse",
            Self::FourierBadLine3OutputSymbols => {
                "expected_failure_fourier_bad_dot_four_line3_symbols_parse"
            }
            Self::Bug387MissingLibraryEndl => "expected_failure_missing_library_endl_parse",
            Self::MessageSubcircuitMissingName => "expected_failure_missing_subcircuit_name_parse",
            Self::MessageSubcircuitMissingEndsEndCard => {
                "expected_failure_subckt_missing_ends_end_card_parse"
            }
            Self::MessageSubcircuitMissingEndsIncludeEof => {
                "expected_failure_subckt_missing_ends_include_eof_parse"
            }
            Self::MessageSubcircuitMissingEndsTopLevelEof => {
                "expected_failure_subckt_missing_ends_toplevel_eof_parse"
            }
            Self::MessageSubcircuitMissingEndsTsInvEof => {
                "expected_failure_subckt_missing_ends_ts_inv_eof_parse"
            }
            Self::MessageSubcircuitDuplicateBindingA2 => {
                "expected_failure_message_subckt_a2_duplicate_binding_build"
            }
            Self::MessageSubcircuitDuplicateBindingJ1 => {
                "expected_failure_message_subckt_j1_duplicate_binding_build"
            }
            Self::MessageDcExcessArguments => "expected_failure_dc_excess_arguments_parse",
            Self::MessageAcUnsupportedSweepType => {
                "expected_failure_ac_unsupported_sweep_type_parse"
            }
            Self::MessageNoiseUnsupportedSweepType => {
                "expected_failure_noise_unsupported_sweep_type_parse"
            }
            Self::MessageMissingLibraryEndl => {
                "expected_failure_message_missing_library_endl_parse"
            }
            Self::MessageMissingLibraryFileUnquoted => {
                "expected_failure_message_missing_library_file_unquoted_parse"
            }
            Self::MessageMissingLibraryFileQuoted => {
                "expected_failure_message_missing_library_file_quoted_parse"
            }
            Self::MessageDuplicateDevice => "expected_failure_message_duplicate_device_parse",
            Self::MessageMissingDeviceNodes => {
                "expected_failure_message_missing_device_nodes_parse"
            }
            Self::Bug702DuplicateExternalInitcond => {
                "expected_failure_bug702_duplicate_external_initcond_parse"
            }
            Self::Bug702DuplicateInlinedInitcond => {
                "expected_failure_bug702_duplicate_inlined_initcond_parse"
            }
            Self::Bug702MalformedInitcondFile => {
                "expected_failure_bug702_malformed_initcond_file_load"
            }
            Self::Bug702MissingInitcondFile => "expected_failure_bug702_missing_initcond_file_load",
            Self::Issue455DuplicateDcSourceFunction => {
                "expected_failure_duplicate_dc_source_function_parse"
            }
            Self::Bug204InvalidDcSweepArity => {
                "expected_failure_bug204_invalid_dc_sweep_arity_parse"
            }
            Self::Bug281InvalidDcSweepArity => {
                "expected_failure_bug281_invalid_dc_sweep_arity_parse"
            }
            Self::Bug354BadFunction => {
                "expected_failure_bug354_unknown_print_function_output_validation"
            }
            Self::Bug354BadLeadCurrent => {
                "expected_failure_bug354_unknown_iv_print_function_output_validation"
            }
            Self::Bug354BadParameter => {
                "expected_failure_bug354_unresolved_print_identifier_output_validation"
            }
            Self::Bug401BadDeviceLine => "expected_failure_bug401_bad_device_line_build",
            Self::Bug401ExtraSpace => "expected_failure_bug401_extra_space_build",
            Self::Bug401WorseDeviceLine => "expected_failure_bug401_worse_device_line_parse",
            Self::Bug701DuplicateTopLevelDevice => {
                "expected_failure_bug701_duplicate_toplevel_device_parse"
            }
            Self::Bug701DuplicateSubcircuitDevice => {
                "expected_failure_bug701_duplicate_subcircuit_device_parse"
            }
            Self::Bug769ParameterNodeVoltage => {
                "expected_failure_bug769_parameter_node_voltage_parse"
            }
            Self::Bug769ParameterDeviceCurrent => {
                "expected_failure_bug769_parameter_device_current_parse"
            }
            Self::Bug769ParameterLeadCurrent => {
                "expected_failure_bug769_parameter_lead_current_parse"
            }
            Self::Bug1578InvalidDeviceType => "expected_failure_bug1578_invalid_device_type_parse",
            Self::Bug198UnrecognizedLine => "expected_failure_bug198_unrecognized_line_parse",
            Self::Bug258UnrecognizedLine => "expected_failure_bug258_unrecognized_line_parse",
            Self::Bug587InvalidNumericNotation => {
                "expected_failure_bug587_invalid_numeric_notation_parse"
            }
        }
    }

    fn upstream_error_policy(self) -> XyceUpstreamExpectedErrorPolicy {
        if self.is_bug354_family() {
            return XyceUpstreamExpectedErrorPolicy::NonzeroExitOnly;
        }
        let ordered_patterns = match self {
            Self::Bug67BehavioralExpression => {
                &[r"Syntax error in number of nodes in expression: \{POLY I[(]V6[)] 300u 1\}"][..]
            }
            Self::Bug671InvalidPwlFile => &["Failed to successfully read vpwl-word.csv"][..],
            Self::Bug726AdjacentCouplings => &[
                "in file adjacent.cir at or near line 13",
                "Specified model not found for device K1",
            ][..],
            Self::Bug744DcOperatingPoint => &["DC Operating Point Failed"][..],
            Self::Bug75UndefinedMutualInductorReference => {
                &["Undefined inductor L2 in mutual inductor K3 definition"][..]
            }
            Self::Bug1595HierarchicalMutualInductorReference => &[
                "Netlist error in file bug1595.cir at or near line 20",
                r"Subcircuit calls \('X' devices\) are not allowed in mutual inductor",
                " definitions",
            ][..],
            Self::Bug1148UndefinedPrintNode => {
                &[r"There was 1 undefined symbol in \.PRINT command: node 2"][..]
            }
            Self::Bug40UndefinedPrintNode => {
                &[r"There was 1 undefined symbol in \.PRINT command: node BAD"][..]
            }
            Self::Bug718InvalidPrintNodes => &[
                "Function or variable V(BOGO1) is not defined",
                "Function or variable V(BOGO2) is not defined",
                "Function or variable V(BOGO3) is not defined",
                "Function or variable V(BOGO4) is not defined",
                "Function or variable V(BOGO5) is not defined",
                "Function or variable N(BOGO6) is not defined",
                "Function or variable V(BOGO7) is not defined",
                "Function or variable V(BOGO8) is not defined",
                "Function or variable V(BOGO9) is not defined",
                "Function or variable V(BOGO9) is not defined",
                "Function or variable V(BOGO10) is not defined",
                "Function or variable V(BOGO11) is not defined",
                "Function or variable V(BOGO12) is not defined",
                "Function or variable V(BOGO13) is not defined",
                "Function or variable V(BOGO14) is not defined",
                "Function or variable V(BOGO15) is not defined",
            ][..],
            Self::MessagePrintBadNodeName | Self::MessagePrintBadVariable => &[
                r"There were 2 undefined symbols in \.PRINT command: node C",
                "node D",
            ][..],
            Self::LeadCurrentsInvalidDevice => &[
                r"There were 2 undefined symbols in \.PRINT command: device RBOGO",
                "node 2",
            ][..],
            Self::MeasureInvalidNodes => &[
                "Function or variable V(BOGONODE) is not defined",
                "Function or variable N(MISSINGNODE) is not defined",
                "Function or variable V(GND) is not defined",
            ][..],
            Self::FourierBadLine3OutputSymbols => &[
                "Function or variable I(BOGODEVICE1) is not defined",
                "Function or variable P(BOGODEVICE2) is not defined",
                "Function or variable W(BOGODEVICE3) is not defined",
                "Function or variable V(2) is not defined",
                "Function or variable N(3) is not defined",
                "Function or variable V(GND) is not defined",
            ][..],
            Self::Bug387MissingLibraryEndl => {
                &[r"Could not find \.ENDL statement for \'\.LIB NOM\.LIB\'"][..]
            }
            Self::MessageSubcircuitMissingName => &["Subcircuit name required"][..],
            Self::MessageSubcircuitMissingEndsEndCard
            | Self::MessageSubcircuitMissingEndsTopLevelEof => {
                &["Subcircuit TESTSUB missing .ENDS"][..]
            }
            Self::MessageSubcircuitMissingEndsIncludeEof => &[
                "Netlist error in file missing.ends",
                "Subcircuit TESTSUB missing .ENDS",
            ][..],
            Self::MessageSubcircuitMissingEndsTsInvEof => &["Subcircuit TS_INV missing .ENDS"][..],
            Self::MessageSubcircuitDuplicateBindingA2 => &[
                "Duplicate nodes in .subckt INV1 point to different nodes in X line invocation",
                "Error invoking subcircuit INV1 instance XINV1",
            ][..],
            Self::MessageSubcircuitDuplicateBindingJ1 => &[
                "Duplicate nodes in .subckt ONEBIT point to different nodes in X line invocation",
                "Error invoking subcircuit ONEBIT instance X1",
            ][..],
            Self::MessageDcExcessArguments => &["Extraneous values"][..],
            Self::MessageAcUnsupportedSweepType => &["Unsupported AC sweep type: BOGO"][..],
            Self::MessageNoiseUnsupportedSweepType => &["Unsupported NOISE sweep type: BOGO"][..],
            Self::MessageMissingLibraryEndl => {
                &[r"Could not find \.ENDL statement for \'\.LIB PLUGH\.LIB\'"][..]
            }
            Self::MessageMissingLibraryFileUnquoted | Self::MessageMissingLibraryFileQuoted => {
                &[r"Could not find include file plugh\.lib"][..]
            }
            Self::MessageDuplicateDevice => &["Duplicate device DA"][..],
            Self::MessageMissingDeviceNodes => {
                &["Not enough fields on input line for device R2"][..]
            }
            Self::Bug702DuplicateExternalInitcond | Self::Bug702DuplicateInlinedInitcond => {
                &[".INITCOND line may appear only once."][..]
            }
            Self::Bug702MalformedInitcondFile => {
                &[r"\.INITCOND file \'noinits\.dat\' is not formatted properly"][..]
            }
            Self::Bug702MissingInitcondFile => &["Could not open the .INITCOND file ic.dat"][..],
            Self::Issue455DuplicateDcSourceFunction => &[
                "Netlist error in file issue455.cir at or near line 4",
                "No such source function dc in V2",
            ][..],
            Self::Bug204InvalidDcSweepArity => &[
                "in file bug204.cir at or near line 14",
                ".DC line not formatted correctly, found unexpected number of fields",
            ][..],
            Self::Bug281InvalidDcSweepArity => &[
                "in file bug_281.cir at or near line 7",
                ".DC line not formatted correctly, found unexpected number of fields",
            ][..],
            Self::Bug354BadFunction | Self::Bug354BadLeadCurrent | Self::Bug354BadParameter => {
                unreachable!("BUG354 uses the nonzero-only wrapper")
            }
            Self::Bug401BadDeviceLine => &[
                "in file bad-device-line.cir at or near line 5",
                "Invalid device type for device AN",
            ][..],
            Self::Bug401ExtraSpace => &[
                "in file extra-space.cir at or near line 2",
                "Invalid device type for device APERFECT",
            ][..],
            Self::Bug401WorseDeviceLine => &[
                "in file worse-device-line.cir at or near line 6",
                "Illegal value found for device REALLY",
            ][..],
            Self::Bug701DuplicateTopLevelDevice => &["Duplicate device V1"][..],
            Self::Bug701DuplicateSubcircuitDevice => &["Duplicate device XVNODES:R1"][..],
            Self::Bug769ParameterNodeVoltage => &[
                "in file bug_769a.cir at or near line 69",
                "Node Voltage may not be used in parameter expression [(]RVAL[)]",
            ][..],
            Self::Bug769ParameterDeviceCurrent => &[
                "in file bug_769b.cir at or near line 69",
                "Device Current may not be used in parameter expression [(]RVAL[)]",
            ][..],
            Self::Bug769ParameterLeadCurrent => &[
                "in file bug_769c.cir at or near line 69",
                "Lead Current may not be used in parameter expression [(]RVAL[)]",
            ][..],
            Self::Bug1578InvalidDeviceType => &[
                "in file bug_1578.cir at or near line 10",
                "Invalid device type for device NETLIST",
            ][..],
            Self::Bug198UnrecognizedLine | Self::Bug258UnrecognizedLine => {
                &["in file bug_198.cir at or near line 3", "Unrecognized line"][..]
            }
            Self::Bug587InvalidNumericNotation => &[
                "in file 587.cir at or near line 43",
                "Invalid notation encountered",
            ][..],
        };
        XyceUpstreamExpectedErrorPolicy::NonzeroExitWithOrderedPatterns {
            search_streams: XyceUpstreamErrorSearchStreams::EitherCompleteStdoutOrStderr,
            ordered_patterns,
        }
    }

    fn is_bug354_family(self) -> bool {
        matches!(
            self,
            Self::Bug354BadFunction | Self::Bug354BadLeadCurrent | Self::Bug354BadParameter
        )
    }

    fn expected_output_symbols(self) -> Option<&'static [XyceExpectedOutputSymbol]> {
        use OutputDirectiveKind::{Four, Measure, Print};
        use OutputSymbolKind::{Device, Node};
        const BUG1148: &[XyceExpectedOutputSymbol] = &[XyceExpectedOutputSymbol {
            directive: Print,
            operator: "V",
            symbol: "2",
            kind: Node,
            file_name: "bug_1148.cir",
            line: 5,
        }];
        const BUG40: &[XyceExpectedOutputSymbol] = &[XyceExpectedOutputSymbol {
            directive: Print,
            operator: "V",
            symbol: "bad",
            kind: Node,
            file_name: "bug_40.cir",
            line: 39,
        }];
        const BUG718: &[XyceExpectedOutputSymbol] = &[
            XyceExpectedOutputSymbol {
                directive: Print,
                operator: "V",
                symbol: "bogo1",
                kind: Node,
                file_name: "invalidNodes.cir",
                line: 10,
            },
            XyceExpectedOutputSymbol {
                directive: Print,
                operator: "V",
                symbol: "bogo2",
                kind: Node,
                file_name: "invalidNodes.cir",
                line: 10,
            },
            XyceExpectedOutputSymbol {
                directive: Print,
                operator: "V",
                symbol: "GND",
                kind: Node,
                file_name: "invalidNodes.cir",
                line: 10,
            },
            XyceExpectedOutputSymbol {
                directive: Print,
                operator: "V",
                symbol: "bogo3",
                kind: Node,
                file_name: "invalidNodes.cir",
                line: 10,
            },
            XyceExpectedOutputSymbol {
                directive: Print,
                operator: "V",
                symbol: "bogo4",
                kind: Node,
                file_name: "invalidNodes.cir",
                line: 10,
            },
            XyceExpectedOutputSymbol {
                directive: Print,
                operator: "V",
                symbol: "bogo5",
                kind: Node,
                file_name: "invalidNodes.cir",
                line: 10,
            },
            XyceExpectedOutputSymbol {
                directive: Print,
                operator: "N",
                symbol: "bogo6",
                kind: Node,
                file_name: "invalidNodes.cir",
                line: 10,
            },
            XyceExpectedOutputSymbol {
                directive: Print,
                operator: "VR",
                symbol: "bogo7",
                kind: Node,
                file_name: "invalidNodes.cir",
                line: 10,
            },
            XyceExpectedOutputSymbol {
                directive: Print,
                operator: "VI",
                symbol: "bogo8",
                kind: Node,
                file_name: "invalidNodes.cir",
                line: 10,
            },
            XyceExpectedOutputSymbol {
                directive: Print,
                operator: "VP",
                symbol: "bogo9",
                kind: Node,
                file_name: "invalidNodes.cir",
                line: 10,
            },
            XyceExpectedOutputSymbol {
                directive: Print,
                operator: "VM",
                symbol: "bogo9",
                kind: Node,
                file_name: "invalidNodes.cir",
                line: 10,
            },
            XyceExpectedOutputSymbol {
                directive: Print,
                operator: "VDB",
                symbol: "bogo10",
                kind: Node,
                file_name: "invalidNodes.cir",
                line: 10,
            },
            XyceExpectedOutputSymbol {
                directive: Print,
                operator: "VR",
                symbol: "bogo11",
                kind: Node,
                file_name: "invalidNodes.cir",
                line: 10,
            },
            XyceExpectedOutputSymbol {
                directive: Print,
                operator: "VI",
                symbol: "bogo12",
                kind: Node,
                file_name: "invalidNodes.cir",
                line: 10,
            },
            XyceExpectedOutputSymbol {
                directive: Print,
                operator: "VP",
                symbol: "bogo13",
                kind: Node,
                file_name: "invalidNodes.cir",
                line: 10,
            },
            XyceExpectedOutputSymbol {
                directive: Print,
                operator: "VM",
                symbol: "bogo14",
                kind: Node,
                file_name: "invalidNodes.cir",
                line: 10,
            },
            XyceExpectedOutputSymbol {
                directive: Print,
                operator: "VDB",
                symbol: "bogo15",
                kind: Node,
                file_name: "invalidNodes.cir",
                line: 10,
            },
        ];
        const BAD_NODENAME: &[XyceExpectedOutputSymbol] = &[
            XyceExpectedOutputSymbol {
                directive: Print,
                operator: "V",
                symbol: "C",
                kind: Node,
                file_name: "bad_nodename.cir",
                line: 8,
            },
            XyceExpectedOutputSymbol {
                directive: Print,
                operator: "VM",
                symbol: "D",
                kind: Node,
                file_name: "bad_nodename.cir",
                line: 8,
            },
        ];
        const BAD_VARIABLE: &[XyceExpectedOutputSymbol] = &[
            XyceExpectedOutputSymbol {
                directive: Print,
                operator: "V",
                symbol: "C",
                kind: Node,
                file_name: "bad_variable.cir",
                line: 8,
            },
            XyceExpectedOutputSymbol {
                directive: Print,
                operator: "VM",
                symbol: "D",
                kind: Node,
                file_name: "bad_variable.cir",
                line: 8,
            },
        ];
        const LEAD: &[XyceExpectedOutputSymbol] = &[
            XyceExpectedOutputSymbol {
                directive: Print,
                operator: "I",
                symbol: "RBogo",
                kind: Device,
                file_name: "lead_for_invalid_device.cir",
                line: 20,
            },
            XyceExpectedOutputSymbol {
                directive: Print,
                operator: "V",
                symbol: "2",
                kind: Node,
                file_name: "lead_for_invalid_device.cir",
                line: 20,
            },
        ];
        const MEASURE_NODES: &[XyceExpectedOutputSymbol] = &[
            XyceExpectedOutputSymbol {
                directive: Measure,
                operator: "V",
                symbol: "bogoNode",
                kind: Node,
                file_name: "invalid_nodes.cir",
                line: 14,
            },
            XyceExpectedOutputSymbol {
                directive: Measure,
                operator: "N",
                symbol: "missingNode",
                kind: Node,
                file_name: "invalid_nodes.cir",
                line: 15,
            },
            XyceExpectedOutputSymbol {
                directive: Measure,
                operator: "V",
                symbol: "GND",
                kind: Node,
                file_name: "invalid_nodes.cir",
                line: 20,
            },
        ];
        const FOURIER: &[XyceExpectedOutputSymbol] = &[
            XyceExpectedOutputSymbol {
                directive: Four,
                operator: "I",
                symbol: "BogoDevice1",
                kind: Device,
                file_name: "bad_dot_four_line3.cir",
                line: 22,
            },
            XyceExpectedOutputSymbol {
                directive: Four,
                operator: "P",
                symbol: "BogoDevice2",
                kind: Device,
                file_name: "bad_dot_four_line3.cir",
                line: 22,
            },
            XyceExpectedOutputSymbol {
                directive: Four,
                operator: "W",
                symbol: "BogoDevice3",
                kind: Device,
                file_name: "bad_dot_four_line3.cir",
                line: 22,
            },
            XyceExpectedOutputSymbol {
                directive: Four,
                operator: "V",
                symbol: "2",
                kind: Node,
                file_name: "bad_dot_four_line3.cir",
                line: 22,
            },
            XyceExpectedOutputSymbol {
                directive: Four,
                operator: "N",
                symbol: "3",
                kind: Node,
                file_name: "bad_dot_four_line3.cir",
                line: 22,
            },
            XyceExpectedOutputSymbol {
                directive: Four,
                operator: "V",
                symbol: "GND",
                kind: Node,
                file_name: "bad_dot_four_line3.cir",
                line: 22,
            },
        ];
        match self {
            Self::Bug1148UndefinedPrintNode => Some(BUG1148),
            Self::Bug40UndefinedPrintNode => Some(BUG40),
            Self::Bug718InvalidPrintNodes => Some(BUG718),
            Self::MessagePrintBadNodeName => Some(BAD_NODENAME),
            Self::MessagePrintBadVariable => Some(BAD_VARIABLE),
            Self::LeadCurrentsInvalidDevice => Some(LEAD),
            Self::MeasureInvalidNodes => Some(MEASURE_NODES),
            Self::FourierBadLine3OutputSymbols => Some(FOURIER),
            _ => None,
        }
    }

    fn expected_observation(self) -> XyceExpectedFailureObservation {
        match self {
            Self::Bug1148UndefinedPrintNode
            | Self::Bug40UndefinedPrintNode
            | Self::Bug718InvalidPrintNodes
            | Self::MessagePrintBadNodeName
            | Self::MessagePrintBadVariable
            | Self::LeadCurrentsInvalidDevice
            | Self::MeasureInvalidNodes
            | Self::FourierBadLine3OutputSymbols => XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::NetlistParse,
                category: XyceExpectedFailureCategory::UndefinedOutputSymbols,
                identifiers: self
                    .expected_output_symbols()
                    .expect("output-symbol kind has a contract")
                    .iter()
                    .copied()
                    .map(XyceExpectedOutputSymbol::identifier)
                    .collect(),
            },
            Self::Bug67BehavioralExpression => XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::CircuitBuild,
                category: XyceExpectedFailureCategory::BehavioralExpressionSyntax,
                identifiers: vec!["X1.B6".to_string(), "X1.V6".to_string()],
            },
            Self::Bug671InvalidPwlFile => XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::ExternalDataLoad,
                category: XyceExpectedFailureCategory::InvalidPwlFileEncoding,
                identifiers: vec!["VPWL".to_string(), "vpwl-word.csv".to_string()],
            },
            Self::Bug726AdjacentCouplings => XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::NetlistParse,
                category: XyceExpectedFailureCategory::AdjacentCouplingSyntax,
                identifiers: vec!["K1".to_string(), "K2".to_string(), "line 13".to_string()],
            },
            Self::Bug744DcOperatingPoint => XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::DcOperatingPoint,
                category: XyceExpectedFailureCategory::ConflictingIdealVoltageConstraints,
                identifiers: vec!["Vsrc1".to_string(), "Vsrc2".to_string(), "1".to_string()],
            },
            Self::Bug75UndefinedMutualInductorReference => XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::NetlistParse,
                category: XyceExpectedFailureCategory::UndefinedMutualInductorReference,
                identifiers: vec![
                    "K3".to_string(),
                    "K3".to_string(),
                    "K3".to_string(),
                    "L2".to_string(),
                    "L2".to_string(),
                    "L2".to_string(),
                    "TOP_LEVEL".to_string(),
                    "2".to_string(),
                    "line 12".to_string(),
                ],
            },
            Self::Bug1595HierarchicalMutualInductorReference => XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::NetlistParse,
                category: XyceExpectedFailureCategory::UndefinedMutualInductorReference,
                identifiers: vec![
                    "K1".to_string(),
                    "K1".to_string(),
                    "K1".to_string(),
                    "X1:L1".to_string(),
                    "X1:L1".to_string(),
                    "X1:L1".to_string(),
                    "TOP_LEVEL".to_string(),
                    "1".to_string(),
                    "line 20".to_string(),
                ],
            },
            Self::Bug387MissingLibraryEndl => XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::NetlistParse,
                category: XyceExpectedFailureCategory::MissingLibraryEndl,
                identifiers: vec!["nom.lib".to_string(), "line 3".to_string()],
            },
            Self::MessageSubcircuitMissingName => XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::NetlistParse,
                category: XyceExpectedFailureCategory::MissingSubcircuitName,
                identifiers: vec![".SUBCKT".to_string(), "line 21".to_string()],
            },
            Self::MessageSubcircuitMissingEndsEndCard => XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::NetlistParse,
                category: XyceExpectedFailureCategory::MissingSubcircuitEnds,
                identifiers: vec![
                    "testsub".to_string(),
                    "TESTSUB".to_string(),
                    "TESTSUB".to_string(),
                    "subckt_missing_ends.cir:12".to_string(),
                    "subckt_missing_ends.cir:21".to_string(),
                    "END_CARD".to_string(),
                ],
            },
            Self::MessageSubcircuitMissingEndsIncludeEof => XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::NetlistParse,
                category: XyceExpectedFailureCategory::MissingSubcircuitEnds,
                identifiers: vec![
                    "testsub".to_string(),
                    "TESTSUB".to_string(),
                    "TESTSUB".to_string(),
                    "missing.ends:1".to_string(),
                    "missing.ends:4".to_string(),
                    "END_OF_SOURCE".to_string(),
                ],
            },
            Self::MessageSubcircuitMissingEndsTopLevelEof => XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::NetlistParse,
                category: XyceExpectedFailureCategory::MissingSubcircuitEnds,
                identifiers: vec![
                    "testsub".to_string(),
                    "TESTSUB".to_string(),
                    "TESTSUB".to_string(),
                    "subckt_missing_ends3.cir:17".to_string(),
                    "subckt_missing_ends3.cir:21".to_string(),
                    "END_OF_SOURCE".to_string(),
                ],
            },
            Self::MessageSubcircuitMissingEndsTsInvEof => XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::NetlistParse,
                category: XyceExpectedFailureCategory::MissingSubcircuitEnds,
                identifiers: vec![
                    "TS_INV".to_string(),
                    "TS_INV".to_string(),
                    "TS_INV".to_string(),
                    "subckt_missing_ends4.cir:22".to_string(),
                    "subckt_missing_ends4.cir:32".to_string(),
                    "END_OF_SOURCE".to_string(),
                ],
            },
            Self::MessageSubcircuitDuplicateBindingA2 => XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::CircuitBuild,
                category: XyceExpectedFailureCategory::DuplicateSubcircuitPortBinding,
                identifiers: vec![
                    "INV1".to_string(),
                    "INV1".to_string(),
                    "Xinv1".to_string(),
                    "XINV1".to_string(),
                    "Xinv1".to_string(),
                    "GND".to_string(),
                    "4".to_string(),
                    "8".to_string(),
                    "0".to_string(),
                    "VDD".to_string(),
                ],
            },
            Self::MessageSubcircuitDuplicateBindingJ1 => XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::CircuitBuild,
                category: XyceExpectedFailureCategory::DuplicateSubcircuitPortBinding,
                identifiers: vec![
                    "ONEBIT".to_string(),
                    "ONEBIT".to_string(),
                    "X1".to_string(),
                    "X1".to_string(),
                    "X1".to_string(),
                    "6".to_string(),
                    "6".to_string(),
                    "8".to_string(),
                    "99".to_string(),
                    "1".to_string(),
                ],
            },
            Self::MessageDcExcessArguments => XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::NetlistParse,
                category: XyceExpectedFailureCategory::DcExcessArguments,
                identifiers: vec!["V1".to_string(), "4.0".to_string(), "line 6".to_string()],
            },
            Self::MessageAcUnsupportedSweepType => XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::NetlistParse,
                category: XyceExpectedFailureCategory::InvalidFrequencySweepType,
                identifiers: vec!["AC".to_string(), "BOGO".to_string(), "line 14".to_string()],
            },
            Self::MessageNoiseUnsupportedSweepType => XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::NetlistParse,
                category: XyceExpectedFailureCategory::InvalidFrequencySweepType,
                identifiers: vec![
                    "NOISE".to_string(),
                    "BOGO".to_string(),
                    "line 17".to_string(),
                ],
            },
            Self::MessageMissingLibraryEndl => XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::NetlistParse,
                category: XyceExpectedFailureCategory::MissingLibraryEndl,
                identifiers: vec![
                    "plugh.lib".to_string(),
                    "UNQUOTED".to_string(),
                    "line 3".to_string(),
                ],
            },
            Self::MessageMissingLibraryFileUnquoted => XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::NetlistParse,
                category: XyceExpectedFailureCategory::MissingLibraryFile,
                identifiers: vec![
                    "plugh.lib".to_string(),
                    "x".to_string(),
                    "UNQUOTED".to_string(),
                    "line 3".to_string(),
                ],
            },
            Self::MessageMissingLibraryFileQuoted => XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::NetlistParse,
                category: XyceExpectedFailureCategory::MissingLibraryFile,
                identifiers: vec![
                    "plugh.lib".to_string(),
                    "x".to_string(),
                    "SINGLE_QUOTED".to_string(),
                    "line 3".to_string(),
                ],
            },
            Self::MessageDuplicateDevice => XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::NetlistParse,
                category: XyceExpectedFailureCategory::DuplicateDeviceName,
                identifiers: vec![
                    "DA".to_string(),
                    "TOP_LEVEL".to_string(),
                    "line 8".to_string(),
                    "line 9".to_string(),
                ],
            },
            Self::MessageMissingDeviceNodes => XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::NetlistParse,
                category: XyceExpectedFailureCategory::MissingDeviceNodes,
                identifiers: vec!["R2".to_string(), "OUT_1".to_string(), "line 14".to_string()],
            },
            Self::Bug702DuplicateExternalInitcond => XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::NetlistParse,
                category: XyceExpectedFailureCategory::DuplicateDeviceInitialCondition,
                identifiers: vec![
                    "EXTERNAL".to_string(),
                    "dup-external.cir:20".to_string(),
                    "dup-external.cir:29".to_string(),
                ],
            },
            Self::Bug702DuplicateInlinedInitcond => XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::NetlistParse,
                category: XyceExpectedFailureCategory::DuplicateDeviceInitialCondition,
                identifiers: vec![
                    "INLINE".to_string(),
                    "dup-inlined.cir:20".to_string(),
                    "dup-inlined.cir:29".to_string(),
                ],
            },
            Self::Bug702MalformedInitcondFile => XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::ExternalDataLoad,
                category: XyceExpectedFailureCategory::MalformedDeviceInitialConditionFile,
                identifiers: vec![
                    "noinits.dat".to_string(),
                    "empty-initcond.cir:19".to_string(),
                    "noinits.dat:1".to_string(),
                ],
            },
            Self::Bug702MissingInitcondFile => XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::ExternalDataLoad,
                category: XyceExpectedFailureCategory::MissingDeviceInitialConditionFile,
                identifiers: vec!["ic.dat".to_string(), "missing-initcond.cir:21".to_string()],
            },
            Self::Issue455DuplicateDcSourceFunction => XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::NetlistParse,
                category: XyceExpectedFailureCategory::DuplicateDcSourceFunction,
                identifiers: vec!["V2".to_string(), "DC".to_string(), "line 4".to_string()],
            },
            Self::Bug204InvalidDcSweepArity => XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::NetlistParse,
                category: XyceExpectedFailureCategory::InvalidDcSweepArity,
                identifiers: vec!["VIN".to_string(), "STEP".to_string(), "line 14".to_string()],
            },
            Self::Bug281InvalidDcSweepArity => XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::NetlistParse,
                category: XyceExpectedFailureCategory::InvalidDcSweepArity,
                identifiers: vec!["VIN".to_string(), "STEP".to_string(), "line 7".to_string()],
            },
            Self::Bug354BadFunction => XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::OutputValidation,
                category: XyceExpectedFailureCategory::UnknownOutputFunction,
                identifiers: vec!["FABS".to_string(), "bad_function.cir:9".to_string()],
            },
            Self::Bug354BadLeadCurrent => XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::OutputValidation,
                category: XyceExpectedFailureCategory::UnknownOutputFunction,
                identifiers: vec![
                    "IV".to_string(),
                    "RB".to_string(),
                    "bad_leadcurrent.cir:9".to_string(),
                ],
            },
            Self::Bug354BadParameter => XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::OutputValidation,
                category: XyceExpectedFailureCategory::UnresolvedOutputIdentifier,
                identifiers: vec!["BAR".to_string(), "bad_parameter.cir:10".to_string()],
            },
            Self::Bug401BadDeviceLine => XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::CircuitBuild,
                category: XyceExpectedFailureCategory::UnknownXspiceModel,
                identifiers: vec!["AN".to_string(), "USER!".to_string(), "line 5".to_string()],
            },
            Self::Bug401ExtraSpace => XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::CircuitBuild,
                category: XyceExpectedFailureCategory::UnknownXspiceModel,
                identifiers: vec![
                    "APERFECT".to_string(),
                    "USER!".to_string(),
                    "line 2".to_string(),
                ],
            },
            Self::Bug401WorseDeviceLine => XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::NetlistParse,
                category: XyceExpectedFailureCategory::MalformedResistorSpecification,
                identifiers: vec![
                    "REALLY".to_string(),
                    "PERFECT".to_string(),
                    "line 6".to_string(),
                ],
            },
            Self::Bug701DuplicateTopLevelDevice => XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::NetlistParse,
                category: XyceExpectedFailureCategory::DuplicateDeviceName,
                identifiers: vec![
                    "V1".to_string(),
                    "TOP_LEVEL".to_string(),
                    "line 5".to_string(),
                    "line 6".to_string(),
                ],
            },
            Self::Bug701DuplicateSubcircuitDevice => XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::NetlistParse,
                category: XyceExpectedFailureCategory::DuplicateDeviceName,
                identifiers: vec![
                    "R1".to_string(),
                    "SUBCIRCUIT:RNODES".to_string(),
                    "XVNODES:R1".to_string(),
                    "line 7".to_string(),
                    "line 8".to_string(),
                ],
            },
            Self::Bug769ParameterNodeVoltage => XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::NetlistParse,
                category: XyceExpectedFailureCategory::ParameterNodeVoltage,
                identifiers: vec![
                    "RVAL".to_string(),
                    "V(3)".to_string(),
                    "line 69".to_string(),
                ],
            },
            Self::Bug769ParameterDeviceCurrent => XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::NetlistParse,
                category: XyceExpectedFailureCategory::ParameterDeviceCurrent,
                identifiers: vec![
                    "RVAL".to_string(),
                    "I(V2)".to_string(),
                    "line 69".to_string(),
                ],
            },
            Self::Bug769ParameterLeadCurrent => XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::NetlistParse,
                category: XyceExpectedFailureCategory::ParameterLeadCurrent,
                identifiers: vec![
                    "RVAL".to_string(),
                    "I(C2)".to_string(),
                    "line 69".to_string(),
                ],
            },
            Self::Bug1578InvalidDeviceType => XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::NetlistParse,
                category: XyceExpectedFailureCategory::UnknownDeviceType,
                identifiers: vec![
                    "NETLIST".to_string(),
                    "N".to_string(),
                    "line 10".to_string(),
                ],
            },
            Self::Bug198UnrecognizedLine | Self::Bug258UnrecognizedLine => {
                XyceExpectedFailureObservation {
                    stage: XyceExpectedFailureStage::NetlistParse,
                    category: XyceExpectedFailureCategory::InvalidNetlistLinePrefix,
                    identifiers: vec!["#".to_string(), "line 3".to_string()],
                }
            }
            Self::Bug587InvalidNumericNotation => XyceExpectedFailureObservation {
                stage: XyceExpectedFailureStage::NetlistParse,
                category: XyceExpectedFailureCategory::InvalidNumericNotation,
                identifiers: vec![
                    "R1".to_string(),
                    "2.0e+".to_string(),
                    "PRIMARY".to_string(),
                    "line 43".to_string(),
                ],
            },
        }
    }

    fn is_bug1595(self) -> bool {
        self == Self::Bug1595HierarchicalMutualInductorReference
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceExpectedFailureStage {
    NetlistParse,
    CircuitBuild,
    ExternalDataLoad,
    OutputValidation,
    DcOperatingPoint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceExpectedFailureCategory {
    BehavioralExpressionSyntax,
    InvalidPwlFileEncoding,
    AdjacentCouplingSyntax,
    ConflictingIdealVoltageConstraints,
    UndefinedOutputSymbols,
    MissingLibraryEndl,
    MissingSubcircuitName,
    MissingSubcircuitEnds,
    DuplicateSubcircuitPortBinding,
    DcExcessArguments,
    InvalidFrequencySweepType,
    MissingLibraryFile,
    MissingDeviceNodes,
    DuplicateDeviceInitialCondition,
    MalformedDeviceInitialConditionFile,
    MissingDeviceInitialConditionFile,
    DuplicateDcSourceFunction,
    InvalidDcSweepArity,
    UnknownXspiceModel,
    MalformedResistorSpecification,
    DuplicateDeviceName,
    ParameterNodeVoltage,
    ParameterDeviceCurrent,
    ParameterLeadCurrent,
    UnknownDeviceType,
    InvalidNetlistLinePrefix,
    InvalidNumericNotation,
    UndefinedMutualInductorReference,
    ConflictingStartupDirectives,
    UnknownOutputFunction,
    UnresolvedOutputIdentifier,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceExpectedFailureObservation {
    stage: XyceExpectedFailureStage,
    category: XyceExpectedFailureCategory,
    identifiers: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceUpstreamErrorSearchStreams {
    EitherCompleteStdoutOrStderr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceUpstreamExpectedErrorPolicy {
    NonzeroExitOnly,
    NonzeroExitWithOrderedPatterns {
        search_streams: XyceUpstreamErrorSearchStreams,
        ordered_patterns: &'static [&'static str],
    },
}

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
    /// Maximum number of mismatches to retain in one result. The runner
    /// normalizes zero to one so diagnostic truncation can never erase a
    /// failure.
    pub max_mismatches: usize,
    /// Maximum wall-clock time allowed for a numerically executed deck.
    pub max_time_per_test_ms: u128,
    /// Print per-deck execution details.
    pub verbose: bool,
    /// Lossless TRA history interpolation for the Xyce version represented by
    /// the selected corpus oracle.
    pub xyce_tra_interpolation: XyceTraInterpolation,
}

impl Default for XyceRunnerConfig {
    fn default() -> Self {
        Self {
            relative_tolerance: 0.02,
            absolute_tolerance: 1.0e-12,
            voltage_absolute_tolerance: rspice_core::constants::VNTOL,
            power_absolute_tolerance: 1.0e-9,
            max_mismatches: 20,
            max_time_per_test_ms: 180_000,
            verbose: false,
            xyce_tra_interpolation: XyceTraInterpolation::default(),
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

/// How a transient `.prn` comparison lined the engine's samples up with the
/// reference table's rows.
///
/// This is not a tolerance, and the difference between the two is the whole
/// reason it is surfaced. [`Self::IndexAligned`] compares reference row `i`
/// against accepted step `i` of the run: the engine had to *choose the same
/// steps* Xyce chose, which is the strictest claim this suite can make about a
/// transient. [`Self::Interpolated`] evaluates the run at each reference time
/// instead — a materially weaker claim that the waveform passes through the
/// right points, saying nothing about where the integrator put them.
///
/// The mode is derived rather than declared: [`XyceTestRunner`] picks it per
/// comparison from the two time axes it was handed. A change to the engine's
/// accepted-step sequence can therefore move a deck from the strict mode to
/// the lenient one with no deck, tolerance, or contract edit anywhere — the
/// gate quietly weakens and every test still passes. `tests/xyce/
/// grid-alignment-manifest.tsv` pins which decks are entitled to the strict
/// mode, checked in both directions, so that transition has to be a conscious
/// one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XyceTransientGridAlignment {
    /// Reference row `i` was compared against accepted step `i`.
    IndexAligned,
    /// The run was interpolated onto each reference time.
    Interpolated,
}

impl XyceTransientGridAlignment {
    /// The manifest and report-file spelling.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::IndexAligned => "index_aligned",
            Self::Interpolated => "interpolated",
        }
    }

    /// Inverse of [`Self::as_str`]; `None` for any other token.
    pub fn parse(token: &str) -> Option<Self> {
        match token {
            "index_aligned" => Some(Self::IndexAligned),
            "interpolated" => Some(Self::Interpolated),
            _ => None,
        }
    }
}

thread_local! {
    /// Grid alignment observed since the current deck started running.
    ///
    /// A deck can drive more than one transient comparison — stepped runs and
    /// the two-table contracts each call the comparison path repeatedly — so
    /// the per-deck answer is a fold rather than a single sample, and the fold
    /// is conservative: a deck counts as index-aligned only when *every*
    /// comparison it made was. One interpolated fallback anywhere is the
    /// weakening the manifest exists to notice.
    ///
    /// Thread-local rather than a field on the runner because the observation
    /// is a property of the comparison that just ran, not of the runner's
    /// configuration, and because `XyceTestRunner` is shared as `&self`
    /// throughout the suite.
    static OBSERVED_TRANSIENT_GRID_ALIGNMENT: std::cell::Cell<Option<XyceTransientGridAlignment>> =
        const { std::cell::Cell::new(None) };
}

fn reset_observed_transient_grid_alignment() {
    OBSERVED_TRANSIENT_GRID_ALIGNMENT.with(|observed| observed.set(None));
}

fn record_transient_grid_alignment(alignment: XyceTransientGridAlignment) {
    OBSERVED_TRANSIENT_GRID_ALIGNMENT.with(|observed| {
        let folded = match (observed.get(), alignment) {
            (Some(XyceTransientGridAlignment::Interpolated), _)
            | (_, XyceTransientGridAlignment::Interpolated) => {
                XyceTransientGridAlignment::Interpolated
            }
            _ => XyceTransientGridAlignment::IndexAligned,
        };
        observed.set(Some(folded));
    });
}

fn observed_transient_grid_alignment() -> Option<XyceTransientGridAlignment> {
    OBSERVED_TRANSIENT_GRID_ALIGNMENT.with(std::cell::Cell::get)
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
    /// Whether the upstream Xyce regression harness explicitly excludes this
    /// retained deck. This is provenance, not an RSpice feature gap.
    pub upstream_excluded: bool,
    /// Original upstream `exclude` file, when the deck has upstream exclusion
    /// provenance. Independently qualified RSpice executions retain this
    /// metadata even though `upstream_excluded` is false for their result.
    pub upstream_exclusion_source: Option<String>,
    /// Error or expected-unsupported reason.
    pub error: Option<String>,
    /// Numeric mismatches for executed decks.
    pub mismatches: Vec<XyceValueMismatch>,
    /// Execution/classification duration.
    pub duration_ms: u128,
    /// Contract label applied by the runner.
    pub contract: String,
    /// Which geometry the deck's transient `.prn` comparisons used, folded
    /// across every comparison the deck made. `None` for decks that never
    /// reached a transient reference comparison at all.
    pub transient_grid_alignment: Option<XyceTransientGridAlignment>,
}

/// Aggregate Xyce corpus statistics.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct XyceStatistics {
    pub total: usize,
    pub executed: usize,
    pub passed: usize,
    pub failed: usize,
    pub expected_unsupported: usize,
    pub upstream_excluded: usize,
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
    upstream_exclusions: Result<BTreeMap<String, XyceUpstreamExclusion>, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceUpstreamExclusion {
    source: String,
    disposition: XyceUpstreamExclusionDisposition,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum XyceUpstreamExclusionDisposition {
    Excluded,
    RspiceIndependentlyQualified { expected_contract: String },
}

#[derive(Debug, Clone)]
struct XyceExecutionPlan {
    deck_path: PathBuf,
    execution_dir: Option<PathBuf>,
    reference_path: PathBuf,
    measurement_reference_paths: Vec<PathBuf>,
    continuous_measurement_reference_paths: Vec<PathBuf>,
    measurement_tolerance: XyceFileCompareTolerance,
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
    parameter_redefinition_policy: ParameterRedefinitionPolicy,
    parameter_redefinition_diagnostic_policy:
        rspice_core::netlist::ParameterRedefinitionDiagnosticPolicy,
    print: XycePrintRequest,
    print_format: Option<String>,
    dc: XyceDcSweep,
    dc_data: Option<XyceDcDataSweep>,
    steps: Vec<StepCommand>,
    diagnostics: Vec<rspice_core::netlist::ParseDiagnostic>,
    /// Authenticated include/library closure used during plan construction.
    /// When present, execution must replay from this bundle without consulting
    /// the live filesystem.
    sealed_sources: Option<SealedSourceBundle>,
}

/// Native static DC sensitivity contract.  Xyce emits a sensitivity table
/// alongside the ordinary `.PRINT DC` table; the table contains the requested
/// base probes followed by one nominal objective and one derivative column per
/// selected parameter/mode.  Keeping this contract separate from
/// `XyceStaticDcPlan` lets ordinary DC output retain its existing oracle path
/// while sharing the parsed sweep, STEP, and netlist execution envelope.
#[derive(Debug, Clone)]
struct XyceStaticDcSensitivityPlan {
    dc: XyceStaticDcPlan,
    reference_path: PathBuf,
    reference_format: XyceDcSensitivityReferenceFormat,
    print: XycePrintRequest,
    objectives: Vec<XyceAcSensitivityObjective>,
    parameters: Vec<String>,
    direct: bool,
    adjoint: bool,
    no_index: bool,
    add_stepnum_col: bool,
    side_outputs: Vec<XyceStaticDcSensitivitySideOutput>,
}

#[derive(Debug, Clone)]
struct XyceStaticDcSensitivitySideOutput {
    file: String,
    reference_path: PathBuf,
    reference_format: XyceDcSensitivityReferenceFormat,
    print: XycePrintRequest,
    no_index: bool,
}

#[derive(Debug, Clone)]
struct XyceDcSensitivityEvaluation {
    netlist: Netlist,
    point: DcSweepPointResult,
    objectives: Vec<rspice_core::analysis::SensitivityResult>,
    step_index: usize,
    local_index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceDcSensitivityReferenceFormat {
    Prn,
    Csv,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceXdmReplaceGroundKind {
    GndExclamation,
    Gnd,
    Ground,
    SubcircuitInstantiation,
}

impl XyceXdmReplaceGroundKind {
    const ALL: [Self; 4] = [
        Self::GndExclamation,
        Self::Gnd,
        Self::Ground,
        Self::SubcircuitInstantiation,
    ];

    fn for_record(relative_path: &str) -> Option<Self> {
        let record = XyceTestRunner::normalize_manifest_key(relative_path);
        Self::ALL.into_iter().find(|kind| kind.record() == record)
    }

    fn record(self) -> &'static str {
        match self {
            Self::GndExclamation => {
                "netlists/xdm/hspice/other_parsing/gnd_exclamation_point_node_symbol.cir"
            }
            Self::Gnd => "netlists/xdm/hspice/other_parsing/gnd_node_symbol.cir",
            Self::Ground => "netlists/xdm/hspice/other_parsing/ground_node_symbol.cir",
            Self::SubcircuitInstantiation => {
                "netlists/xdm/hspice/other_parsing/ground_node_synonym_in_subckt_instantiation.cir"
            }
        }
    }

    fn authored_alias(self) -> &'static str {
        match self {
            Self::GndExclamation => "GND!",
            Self::Gnd | Self::SubcircuitInstantiation => "GND",
            Self::Ground => "GROUND",
        }
    }

    fn requires_subcircuit(self) -> bool {
        self == Self::SubcircuitInstantiation
    }

    fn expected_flattened_snapshot(self) -> Vec<XyceXdmReplaceGroundElementSnapshot> {
        let element = |name: &str, nodes: &[&str], kind: &str, value: Value| {
            XyceXdmReplaceGroundElementSnapshot {
                name: name.to_string(),
                nodes: nodes.iter().map(|node| (*node).to_string()).collect(),
                kind: kind.to_string(),
                value_bits: value.to_bits(),
            }
        };
        if self.requires_subcircuit() {
            vec![
                element("r1", &["1", "2"], "R", 15.0),
                element("va", &["1", "0"], "V:DC", 0.0),
                element("x1.r1", &["2", "x1.a1"], "R", 5.0),
                element("x1.r2", &["x1.a1", "x1.a2"], "R", 5.0),
                element("x1.r3", &["x1.a2", "0"], "R", 5.0),
            ]
        } else {
            vec![
                element("r1", &["1", "2"], "R", 10.0),
                element("r2", &["2", "0"], "R", 10.0),
                element("va", &["1", "0"], "V:DC", 0.0),
            ]
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceXdmReplaceGroundElementSnapshot {
    name: String,
    nodes: Vec<String>,
    kind: String,
    value_bits: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceAbmPowKind {
    UnaryMinusPrecedence,
    NegativeIntegerExponent,
    FractionalPrincipalComplex,
}

impl XyceAbmPowKind {
    const ALL: [Self; 3] = [
        Self::UnaryMinusPrecedence,
        Self::NegativeIntegerExponent,
        Self::FractionalPrincipalComplex,
    ];

    fn for_record(relative_path: &str) -> Option<Self> {
        let record = XyceTestRunner::normalize_manifest_key(relative_path);
        Self::ALL.into_iter().find(|kind| kind.record() == record)
    }

    fn record(self) -> &'static str {
        match self {
            Self::UnaryMinusPrecedence => "netlists/abm_pow/abmpow1.cir",
            Self::NegativeIntegerExponent => "netlists/abm_pow/abmpow2.cir",
            Self::FractionalPrincipalComplex => "netlists/abm_pow/abmpow3.cir",
        }
    }

    fn expected_columns(self) -> &'static [&'static str] {
        match self {
            Self::UnaryMinusPrecedence => &["Index", "V(1)", "V(4)"],
            Self::NegativeIntegerExponent => &["Index", "V(1)", "V(5)", "V(6)"],
            Self::FractionalPrincipalComplex => &["Index", "V(1)", "V(5)", "V(6)"],
        }
    }

    fn expected_rows(self) -> usize {
        match self {
            Self::UnaryMinusPrecedence => 51,
            Self::NegativeIntegerExponent | Self::FractionalPrincipalComplex => 26,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceAbmTransientKind {
    DirectTime,
    ParameterTime,
    SquareRoot,
}

impl XyceAbmTransientKind {
    const ALL: [Self; 3] = [Self::DirectTime, Self::ParameterTime, Self::SquareRoot];

    // Text identities use canonical LF bytes. This is the representation in
    // Git and the Release-7.10.0 artifacts; validation normalizes a CRLF
    // checkout before comparing lengths and digests.

    fn for_record(relative_path: &str) -> Option<Self> {
        let record = XyceTestRunner::normalize_manifest_key(relative_path);
        Self::ALL.into_iter().find(|kind| kind.record() == record)
    }

    fn record(self) -> &'static str {
        match self {
            Self::DirectTime => "netlists/abm_time/time.cir",
            Self::ParameterTime => "netlists/abm_time/time_param.cir",
            Self::SquareRoot => "netlists/abm_sqrt/sqrt.cir",
        }
    }

    fn expected_columns(self) -> &'static [&'static str] {
        match self {
            Self::DirectTime | Self::ParameterTime => &["Index", "TIME", "V(1)", "V(2)"],
            Self::SquareRoot => &["Index", "TIME", "V(1)", "V(2)", "V(3)"],
        }
    }

    fn stop(self) -> Value {
        match self {
            Self::DirectTime | Self::ParameterTime => 6.0,
            Self::SquareRoot => 12.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceMeasureContTranKind {
    Derivative,
    FindWhen,
    TriggerTarget,
}

impl XyceMeasureContTranKind {
    const ALL: [Self; 3] = [Self::Derivative, Self::FindWhen, Self::TriggerTarget];

    fn for_record(relative_path: &str) -> Option<Self> {
        let record = XyceTestRunner::normalize_manifest_key(relative_path);
        Self::ALL.into_iter().find(|kind| kind.record() == record)
    }

    fn record(self) -> &'static str {
        match self {
            Self::Derivative => "netlists/measure_cont/derivtesttran.cir",
            Self::FindWhen => "netlists/measure_cont/findwhentesttran.cir",
            Self::TriggerTarget => "netlists/measure_cont/trigtargtesttran.cir",
        }
    }

    fn gs_relative_path(self) -> &'static str {
        match self {
            Self::Derivative => "Netlists/MEASURE_CONT/DerivTestTranGSfile",
            Self::FindWhen => "Netlists/MEASURE_CONT/FindWhenTestTranGSfile",
            Self::TriggerTarget => "Netlists/MEASURE_CONT/TrigTargTestTranGSfile",
        }
    }

    fn mt0_relative_path(self) -> &'static str {
        match self {
            Self::Derivative => "OutputData/MEASURE_CONT/DerivTestTran.cir.mt0",
            Self::FindWhen => "OutputData/MEASURE_CONT/FindWhenTestTran.cir.mt0",
            Self::TriggerTarget => "OutputData/MEASURE_CONT/TrigTargTestTran.cir.mt0",
        }
    }

    fn prn(self) -> Option<&'static str> {
        match self {
            Self::Derivative => Some("OutputData/MEASURE_CONT/DerivTestTran.cir.prn"),
            Self::FindWhen => Some("OutputData/MEASURE_CONT/FindWhenTestTran.cir.prn"),
            Self::TriggerTarget => None,
        }
    }

    fn expected_measurement_counts(self) -> (usize, usize) {
        match self {
            Self::Derivative => (18, 32),
            Self::FindWhen => (22, 42),
            Self::TriggerTarget => (0, 18),
        }
    }

    fn expected_print_probes(self) -> &'static [&'static str] {
        match self {
            Self::Derivative => &[
                "v(1)",
                "v(2)",
                "derivcrossconttest2",
                "derivcrossconttest2",
                "derivcrossconttest3",
                "derivcrossconttest4",
                "derivcrossneg2",
                "derivcrosscontneg2",
                "derivcrossneg5",
                "derivcrosscontneg5",
            ],
            Self::FindWhen => &[
                "v(1)",
                "v(2)",
                "whencrossconttest2",
                "whencrossconttest2",
                "whencrossconttest3",
                "whencrossconttest4",
                "whencrossneg2",
                "whencrosscontneg2",
                "whencrossneg5",
                "whencrosscontneg5",
            ],
            Self::TriggerTarget => &["v(1)", "v(2)"],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceMeasureContStepTranKind {
    Derivative,
    FindWhen,
    TriggerTarget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceMeasureContStepTranRole {
    Main,
    Control0,
    Control1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct XyceMeasureContStepTranMember {
    kind: XyceMeasureContStepTranKind,
    role: XyceMeasureContStepTranRole,
}

impl XyceMeasureContStepTranMember {
    const ALL: [Self; 9] = [
        Self::new(
            XyceMeasureContStepTranKind::Derivative,
            XyceMeasureContStepTranRole::Main,
        ),
        Self::new(
            XyceMeasureContStepTranKind::Derivative,
            XyceMeasureContStepTranRole::Control0,
        ),
        Self::new(
            XyceMeasureContStepTranKind::Derivative,
            XyceMeasureContStepTranRole::Control1,
        ),
        Self::new(
            XyceMeasureContStepTranKind::FindWhen,
            XyceMeasureContStepTranRole::Main,
        ),
        Self::new(
            XyceMeasureContStepTranKind::FindWhen,
            XyceMeasureContStepTranRole::Control0,
        ),
        Self::new(
            XyceMeasureContStepTranKind::FindWhen,
            XyceMeasureContStepTranRole::Control1,
        ),
        Self::new(
            XyceMeasureContStepTranKind::TriggerTarget,
            XyceMeasureContStepTranRole::Main,
        ),
        Self::new(
            XyceMeasureContStepTranKind::TriggerTarget,
            XyceMeasureContStepTranRole::Control0,
        ),
        Self::new(
            XyceMeasureContStepTranKind::TriggerTarget,
            XyceMeasureContStepTranRole::Control1,
        ),
    ];

    const fn new(kind: XyceMeasureContStepTranKind, role: XyceMeasureContStepTranRole) -> Self {
        Self { kind, role }
    }

    fn for_record(relative_path: &str) -> Option<Self> {
        let record = XyceTestRunner::normalize_manifest_key(relative_path);
        Self::ALL
            .into_iter()
            .find(|member| member.record() == record)
    }

    fn record(self) -> &'static str {
        match (self.kind, self.role) {
            (XyceMeasureContStepTranKind::Derivative, XyceMeasureContStepTranRole::Main) => {
                "netlists/measure_cont/step/derivtesttran.cir"
            }
            (XyceMeasureContStepTranKind::Derivative, XyceMeasureContStepTranRole::Control0) => {
                "netlists/measure_cont/step/derivtesttran.s0.cir"
            }
            (XyceMeasureContStepTranKind::Derivative, XyceMeasureContStepTranRole::Control1) => {
                "netlists/measure_cont/step/derivtesttran.s1.cir"
            }
            (XyceMeasureContStepTranKind::FindWhen, XyceMeasureContStepTranRole::Main) => {
                "netlists/measure_cont/step/findwhentesttran.cir"
            }
            (XyceMeasureContStepTranKind::FindWhen, XyceMeasureContStepTranRole::Control0) => {
                "netlists/measure_cont/step/findwhentesttran.s0.cir"
            }
            (XyceMeasureContStepTranKind::FindWhen, XyceMeasureContStepTranRole::Control1) => {
                "netlists/measure_cont/step/findwhentesttran.s1.cir"
            }
            (XyceMeasureContStepTranKind::TriggerTarget, XyceMeasureContStepTranRole::Main) => {
                "netlists/measure_cont/step/trigtargtesttran.cir"
            }
            (XyceMeasureContStepTranKind::TriggerTarget, XyceMeasureContStepTranRole::Control0) => {
                "netlists/measure_cont/step/trigtargtesttran.s0.cir"
            }
            (XyceMeasureContStepTranKind::TriggerTarget, XyceMeasureContStepTranRole::Control1) => {
                "netlists/measure_cont/step/trigtargtesttran.s1.cir"
            }
        }
    }

    fn source_relative_path(self) -> &'static str {
        match (self.kind, self.role) {
            (XyceMeasureContStepTranKind::Derivative, XyceMeasureContStepTranRole::Main) => {
                "Netlists/MEASURE_CONT/STEP/DerivTestTran.cir"
            }
            (XyceMeasureContStepTranKind::Derivative, XyceMeasureContStepTranRole::Control0) => {
                "Netlists/MEASURE_CONT/STEP/DerivTestTran.s0.cir"
            }
            (XyceMeasureContStepTranKind::Derivative, XyceMeasureContStepTranRole::Control1) => {
                "Netlists/MEASURE_CONT/STEP/DerivTestTran.s1.cir"
            }
            (XyceMeasureContStepTranKind::FindWhen, XyceMeasureContStepTranRole::Main) => {
                "Netlists/MEASURE_CONT/STEP/FindWhenTestTran.cir"
            }
            (XyceMeasureContStepTranKind::FindWhen, XyceMeasureContStepTranRole::Control0) => {
                "Netlists/MEASURE_CONT/STEP/FindWhenTestTran.s0.cir"
            }
            (XyceMeasureContStepTranKind::FindWhen, XyceMeasureContStepTranRole::Control1) => {
                "Netlists/MEASURE_CONT/STEP/FindWhenTestTran.s1.cir"
            }
            (XyceMeasureContStepTranKind::TriggerTarget, XyceMeasureContStepTranRole::Main) => {
                "Netlists/MEASURE_CONT/STEP/TrigTargTestTran.cir"
            }
            (XyceMeasureContStepTranKind::TriggerTarget, XyceMeasureContStepTranRole::Control0) => {
                "Netlists/MEASURE_CONT/STEP/TrigTargTestTran.s0.cir"
            }
            (XyceMeasureContStepTranKind::TriggerTarget, XyceMeasureContStepTranRole::Control1) => {
                "Netlists/MEASURE_CONT/STEP/TrigTargTestTran.s1.cir"
            }
        }
    }

    fn main(kind: XyceMeasureContStepTranKind) -> Self {
        Self::new(kind, XyceMeasureContStepTranRole::Main)
    }

    fn control(kind: XyceMeasureContStepTranKind, index: usize) -> Option<Self> {
        let role = match index {
            0 => XyceMeasureContStepTranRole::Control0,
            1 => XyceMeasureContStepTranRole::Control1,
            _ => return None,
        };
        Some(Self::new(kind, role))
    }

    fn expected_measurement_counts(self) -> (usize, usize) {
        match self.kind {
            XyceMeasureContStepTranKind::Derivative => (12, 32),
            XyceMeasureContStepTranKind::FindWhen => (16, 42),
            XyceMeasureContStepTranKind::TriggerTarget => (0, 7),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceMeasureStepFindWhenRole {
    Owner,
    Control0,
    Control1,
    Control2,
    Control3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct XyceMeasureStepFindWhenMember {
    role: XyceMeasureStepFindWhenRole,
}

impl XyceMeasureStepFindWhenMember {
    const ALL: [Self; 5] = [
        Self::new(XyceMeasureStepFindWhenRole::Owner),
        Self::new(XyceMeasureStepFindWhenRole::Control0),
        Self::new(XyceMeasureStepFindWhenRole::Control1),
        Self::new(XyceMeasureStepFindWhenRole::Control2),
        Self::new(XyceMeasureStepFindWhenRole::Control3),
    ];

    const fn new(role: XyceMeasureStepFindWhenRole) -> Self {
        Self { role }
    }

    fn for_record(relative_path: &str) -> Option<Self> {
        let record = XyceTestRunner::normalize_manifest_key(relative_path);
        Self::ALL
            .into_iter()
            .find(|member| member.record() == record)
    }

    fn owner() -> Self {
        Self::new(XyceMeasureStepFindWhenRole::Owner)
    }

    fn control(index: usize) -> Option<Self> {
        let role = match index {
            0 => XyceMeasureStepFindWhenRole::Control0,
            1 => XyceMeasureStepFindWhenRole::Control1,
            2 => XyceMeasureStepFindWhenRole::Control2,
            3 => XyceMeasureStepFindWhenRole::Control3,
            _ => return None,
        };
        Some(Self::new(role))
    }

    fn is_owner(self) -> bool {
        self.role == XyceMeasureStepFindWhenRole::Owner
    }

    fn record(self) -> &'static str {
        match self.role {
            XyceMeasureStepFindWhenRole::Owner => "netlists/measure/step/findwhentest.cir",
            XyceMeasureStepFindWhenRole::Control0 => "netlists/measure/step/findwhentest.s0.cir",
            XyceMeasureStepFindWhenRole::Control1 => "netlists/measure/step/findwhentest.s1.cir",
            XyceMeasureStepFindWhenRole::Control2 => "netlists/measure/step/findwhentest.s2.cir",
            XyceMeasureStepFindWhenRole::Control3 => "netlists/measure/step/findwhentest.s3.cir",
        }
    }

    fn source_relative_path(self) -> &'static str {
        match self.role {
            XyceMeasureStepFindWhenRole::Owner => "Netlists/MEASURE/STEP/FindWhenTest.cir",
            XyceMeasureStepFindWhenRole::Control0 => "Netlists/MEASURE/STEP/FindWhenTest.s0.cir",
            XyceMeasureStepFindWhenRole::Control1 => "Netlists/MEASURE/STEP/FindWhenTest.s1.cir",
            XyceMeasureStepFindWhenRole::Control2 => "Netlists/MEASURE/STEP/FindWhenTest.s2.cir",
            XyceMeasureStepFindWhenRole::Control3 => "Netlists/MEASURE/STEP/FindWhenTest.s3.cir",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceRemoveUnusedKind {
    ReplaceGround,
    LiteralGroundNames,
}

impl XyceRemoveUnusedKind {
    const ALL: [Self; 2] = [Self::ReplaceGround, Self::LiteralGroundNames];

    fn for_record(relative_path: &str) -> Option<Self> {
        let record = XyceTestRunner::normalize_manifest_key(relative_path);
        Self::ALL.into_iter().find(|kind| kind.record() == record)
    }

    fn record(self) -> &'static str {
        match self {
            Self::ReplaceGround => "netlists/redund_remove/gnd_and_redund.cir",
            Self::LiteralGroundNames => "netlists/redund_remove/just_redund.cir",
        }
    }

    fn replace_ground(self) -> bool {
        self == Self::ReplaceGround
    }

    fn expected_divider_ratio(self) -> Value {
        if self.replace_ground() { 0.5 } else { 1.0 }
    }

    fn expected_flattened_element_count(self) -> usize {
        if self.replace_ground() { 6 } else { 12 }
    }

    fn expected_flattened_snapshot(self, policy_off: bool) -> Vec<XyceRemoveUnusedElementSnapshot> {
        let element =
            |name: &str, nodes: &[&str], kind: &str, value: Option<Value>, model: Option<&str>| {
                XyceRemoveUnusedElementSnapshot {
                    name: name.to_ascii_lowercase(),
                    nodes: nodes.iter().map(|node| node.to_ascii_lowercase()).collect(),
                    kind: kind.to_string(),
                    value_bits: value.map(Value::to_bits),
                    model: model.map(str::to_ascii_lowercase),
                }
            };
        let ground = if self.replace_ground() { "0" } else { "gnd" };
        let ground_word = if self.replace_ground() { "0" } else { "ground" };
        let mut expected = vec![
            element("V1", &["1", "0"], "V", Some(1.0), None),
            element("R1", &["1", "2"], "R", Some(1.0), None),
            element("R2", &["2", ground], "R", Some(2.0), None),
            element("C1", &["2", ground], "C", Some(1.0), None),
            element("X1.R1", &["2", ground_word], "R", Some(2.0), None),
            element("X1.X2.C1", &["2", ground_word], "C", Some(1.0), None),
        ];
        if policy_off || !self.replace_ground() {
            for (prefix, port1, port2, local3) in [
                ("", "1", "2", "3"),
                ("X1.", "2", ground_word, "X1.3"),
                ("X1.X2.", "2", ground_word, "X1.X2.3"),
            ] {
                let (alias_gnd, alias_gnd_bang, alias_ground) = if self.replace_ground() {
                    ("0".to_string(), "0".to_string(), "0".to_string())
                } else {
                    (
                        format!("{prefix}gnd"),
                        format!("{prefix}gnd!"),
                        format!("{prefix}ground"),
                    )
                };
                let junk = [
                    element(
                        &format!("{prefix}C11"),
                        &[&alias_gnd_bang, &alias_ground],
                        "C",
                        Some(1.0),
                        None,
                    ),
                    element(
                        &format!("{prefix}D11"),
                        &[port1, port1],
                        "D",
                        None,
                        Some("Dmod"),
                    ),
                    element(&format!("{prefix}I11"), &["0", "0"], "I", Some(4.0), None),
                    element(
                        &format!("{prefix}L11"),
                        &[port2, port2],
                        "L",
                        Some(3.0),
                        None,
                    ),
                    element(
                        &format!("{prefix}M11"),
                        &[&alias_gnd, &alias_ground, &alias_gnd_bang, port2],
                        "M",
                        None,
                        Some("Nmod"),
                    ),
                    element(
                        &format!("{prefix}Q11"),
                        &[port2, port2, port2, local3],
                        "Q",
                        None,
                        Some("Qmod"),
                    ),
                    element(
                        &format!("{prefix}R11"),
                        &[port1, port1],
                        "R",
                        Some(1.0),
                        None,
                    ),
                    element(
                        &format!("{prefix}V11"),
                        &[local3, local3],
                        "V",
                        Some(4.0),
                        None,
                    ),
                ];
                if policy_off {
                    expected.extend(junk);
                } else {
                    expected.extend(
                        junk.into_iter()
                            .filter(|snapshot| matches!(snapshot.kind.as_str(), "C" | "M")),
                    );
                }
            }
        }
        expected.sort_by(|left, right| left.name.cmp(&right.name));
        expected
    }

    fn expected_authored_snapshots(
        self,
    ) -> (
        Vec<XyceRemoveUnusedElementSnapshot>,
        Vec<(String, Vec<XyceRemoveUnusedElementSnapshot>)>,
    ) {
        let element =
            |name: &str, nodes: &[&str], kind: &str, value: Option<Value>, model: Option<&str>| {
                XyceRemoveUnusedElementSnapshot {
                    name: name.to_ascii_lowercase(),
                    nodes: nodes.iter().map(|node| node.to_ascii_lowercase()).collect(),
                    kind: kind.to_string(),
                    value_bits: value.map(Value::to_bits),
                    model: model.map(str::to_ascii_lowercase),
                }
            };
        let ground = if self.replace_ground() { "0" } else { "gnd" };
        let ground_word = if self.replace_ground() { "0" } else { "ground" };
        let mut top = vec![
            element("V1", &["1", "0"], "V", Some(1.0), None),
            element("R1", &["1", "2"], "R", Some(1.0), None),
            element("R2", &["2", ground], "R", Some(2.0), None),
            element("X1", &["2", ground_word], "X", None, Some("resistor")),
            element("C1", &["2", ground], "C", Some(1.0), None),
        ];
        let mut resistor = vec![
            element("R1", &["1", "2"], "R", Some(2.0), None),
            element("X2", &["1", "2"], "X", None, Some("capacitor")),
        ];
        let mut capacitor = vec![element("C1", &["1", "2"], "C", Some(1.0), None)];
        if !self.replace_ground() {
            let c11 = element("C11", &["gnd!", "ground"], "C", Some(1.0), None);
            let m11 = element(
                "M11",
                &["gnd", "ground", "gnd!", "2"],
                "M",
                None,
                Some("Nmod"),
            );
            top.extend([c11.clone(), m11.clone()]);
            resistor.extend([c11.clone(), m11.clone()]);
            capacitor.extend([c11, m11]);
        }
        for snapshot in [&mut top, &mut resistor, &mut capacitor] {
            snapshot.sort_by(|left, right| left.name.cmp(&right.name));
        }
        (
            top,
            vec![
                ("capacitor".to_string(), capacitor),
                ("resistor".to_string(), resistor),
            ],
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceRemoveUnusedElementSnapshot {
    name: String,
    nodes: Vec<String>,
    kind: String,
    value_bits: Option<u64>,
    model: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceAddResistorsKind {
    NoDcPath,
    OneTerminal,
    RedundantBridge,
}

impl XyceAddResistorsKind {
    const ALL: [Self; 3] = [Self::NoDcPath, Self::OneTerminal, Self::RedundantBridge];

    fn for_record(relative_path: &str) -> Option<Self> {
        let record = XyceTestRunner::normalize_manifest_key(relative_path);
        Self::ALL.into_iter().find(|kind| kind.record() == record)
    }

    fn record(self) -> &'static str {
        match self {
            Self::NoDcPath => "netlists/preproc_addres/nodcpath.cir",
            Self::OneTerminal => "netlists/preproc_addres/oneterm.cir",
            Self::RedundantBridge => "netlists/redund_remove/gnd_and_redund_addres.cir",
        }
    }

    fn is_transient(self) -> bool {
        !matches!(self, Self::RedundantBridge)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceAddResistorsElementSnapshot {
    name: String,
    nodes: Vec<String>,
    kind: String,
    value_bits: Option<u64>,
    initial_value_bits: Option<u64>,
    model: Option<String>,
    provenance: String,
}

#[derive(Debug, Clone)]
struct XyceStaticTranPlan {
    deck_path: PathBuf,
    oracle: XyceStaticTranOracle,
    source: String,
    print: Option<XycePrintRequest>,
    output_override: bool,
    timeint_conststep: bool,
    tran: XyceTranAnalysis,
    steps: Vec<StepCommand>,
    contract: XyceStaticTranContract,
    wrapper_tolerance: Option<XyceComparisonTolerance>,
    comparison_mode: XyceStaticTranComparisonMode,
}

#[derive(Debug, Clone)]
enum XyceStaticTranOracle {
    None,
    Waveform(PathBuf),
    ScalarMeasurements {
        reference_paths: Vec<PathBuf>,
        tolerance: XyceFileCompareTolerance,
        input: XyceScalarTranMeasurementInput,
    },
    AuthoredFailValue {
        measurements: Vec<XyceAuthoredFailValueMeasurement>,
    },
}

#[derive(Debug, Clone, PartialEq)]
struct XyceAuthoredFailValueMeasurement {
    name: String,
    failure_limit: Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum XyceScalarTranMeasurementInput {
    Simulation,
    Remeasure(PathBuf),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceStaticTranComparisonMode {
    Pointwise,
    Release710IntegratedRms {
        scientific_precision: usize,
    },
    Release710IntegratedRmsComp {
        scientific_precision: usize,
        error_bounds: XyceVerifyCompErrorBounds,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceVerifyCompErrorBounds {
    Release710Default,
    DeckOverrides,
}

impl XyceStaticTranComparisonMode {
    fn uses_integrated_rms_verifier(self) -> bool {
        matches!(
            self,
            Self::Release710IntegratedRms { .. } | Self::Release710IntegratedRmsComp { .. }
        )
    }
}

impl XyceStaticTranPlan {
    fn is_scalar_measurement_only(&self) -> bool {
        matches!(self.oracle, XyceStaticTranOracle::ScalarMeasurements { .. })
    }

    fn is_authored_fail_value_measurement_only(&self) -> bool {
        matches!(self.oracle, XyceStaticTranOracle::AuthoredFailValue { .. })
    }

    fn require_print(&self, consumer: &str) -> Result<&XycePrintRequest, String> {
        self.print
            .as_ref()
            .ok_or_else(|| format!("{consumer} requires an authored primary .PRINT TRAN request"))
    }

    fn require_waveform_reference_path(&self, consumer: &str) -> Result<&Path, String> {
        match &self.oracle {
            XyceStaticTranOracle::Waveform(path) => Ok(path),
            XyceStaticTranOracle::None => Err(format!(
                "{consumer} requires a waveform oracle, but this plan has no file oracle"
            )),
            XyceStaticTranOracle::ScalarMeasurements { .. } => Err(format!(
                "{consumer} requires a waveform oracle, but this plan has a scalar measurement oracle"
            )),
            XyceStaticTranOracle::AuthoredFailValue { .. } => Err(format!(
                "{consumer} requires a waveform oracle, but this plan has an authored FAILVALUE oracle"
            )),
        }
    }

    fn has_waveform_reference_file(&self) -> bool {
        matches!(&self.oracle, XyceStaticTranOracle::Waveform(path) if path.is_file())
    }

    fn replace_waveform_reference_path(
        &mut self,
        path: PathBuf,
        consumer: &str,
    ) -> Result<(), String> {
        match &mut self.oracle {
            XyceStaticTranOracle::Waveform(reference_path) => {
                *reference_path = path;
                Ok(())
            }
            XyceStaticTranOracle::None
            | XyceStaticTranOracle::ScalarMeasurements { .. }
            | XyceStaticTranOracle::AuthoredFailValue { .. } => Err(format!(
                "{consumer} cannot replace a non-waveform transient oracle"
            )),
        }
    }

    fn scalar_measurement_oracle(
        &self,
    ) -> Option<(
        &[PathBuf],
        XyceFileCompareTolerance,
        &XyceScalarTranMeasurementInput,
    )> {
        match &self.oracle {
            XyceStaticTranOracle::ScalarMeasurements {
                reference_paths,
                tolerance,
                input,
            } => Some((reference_paths, *tolerance, input)),
            XyceStaticTranOracle::None
            | XyceStaticTranOracle::Waveform(_)
            | XyceStaticTranOracle::AuthoredFailValue { .. } => None,
        }
    }

    fn authored_fail_value_oracle(&self) -> Option<&[XyceAuthoredFailValueMeasurement]> {
        match &self.oracle {
            XyceStaticTranOracle::AuthoredFailValue { measurements } => Some(measurements),
            XyceStaticTranOracle::None
            | XyceStaticTranOracle::Waveform(_)
            | XyceStaticTranOracle::ScalarMeasurements { .. } => None,
        }
    }

    fn validate_oracle_contract(
        &self,
        purpose: XyceStaticTranPlanPurpose,
        requires_wrapper: bool,
    ) -> Result<(), String> {
        match &self.oracle {
            XyceStaticTranOracle::ScalarMeasurements {
                reference_paths,
                input,
                ..
            } => {
                if purpose != XyceStaticTranPlanPurpose::AbsoluteOracle
                    || !requires_wrapper
                    || self.contract != XyceStaticTranContract::WrapperStatic
                    || self.output_override
                    || !self.steps.is_empty()
                    || self.comparison_mode != XyceStaticTranComparisonMode::Pointwise
                    || reference_paths.len() != 1
                    || reference_paths[0]
                        .extension()
                        .is_none_or(|extension| !extension.eq_ignore_ascii_case("mt0"))
                    || !reference_paths[0].is_file()
                    || matches!(
                        input,
                        XyceScalarTranMeasurementInput::Remeasure(path) if !path.is_file()
                    )
                {
                    return Err(
                        "scalar TRAN measurement oracle requires an absolute, wrapper-origin, unstepped, pointwise WrapperStatic plan with one checked-in .mt0 file and no output override"
                            .to_string(),
                    );
                }
                Ok(())
            }
            XyceStaticTranOracle::Waveform(path) => {
                if !path.is_file() {
                    return Err(format!(
                        "transient waveform oracle does not exist at {}",
                        path.display()
                    ));
                }
                self.require_print("waveform transient plan")?;
                Ok(())
            }
            XyceStaticTranOracle::AuthoredFailValue { measurements } => {
                if purpose != XyceStaticTranPlanPurpose::AbsoluteOracle
                    || requires_wrapper
                    || measurements.is_empty()
                    || self.output_override
                    || !self.steps.is_empty()
                    || self.comparison_mode != XyceStaticTranComparisonMode::Pointwise
                    || !matches!(
                        self.contract,
                        XyceStaticTranContract::PlainStatic
                            | XyceStaticTranContract::PlainCsv
                            | XyceStaticTranContract::PlainCsd
                    )
                {
                    return Err(
                        "authored FAILVALUE TRAN oracle requires an ordinary, absolute, unstepped, pointwise plan with at least one typed measurement and no output override"
                            .to_string(),
                    );
                }
                Ok(())
            }
            XyceStaticTranOracle::None => {
                let no_file_shape = matches!(
                    (purpose, requires_wrapper, self.contract),
                    (
                        XyceStaticTranPlanPurpose::AbsoluteOracle,
                        true,
                        XyceStaticTranContract::WrapperNoIndexHeader
                    ) | (
                        XyceStaticTranPlanPurpose::AnalyticOracle,
                        true,
                        XyceStaticTranContract::WrapperStatic
                    ) | (
                        XyceStaticTranPlanPurpose::DiodeAnalyticOracle,
                        true,
                        XyceStaticTranContract::WrapperStatic
                    ) | (
                        XyceStaticTranPlanPurpose::LegacyDeviceAnalyticOracle,
                        true,
                        XyceStaticTranContract::WrapperStatic
                    ) | (
                        XyceStaticTranPlanPurpose::PassiveTemperatureAnalyticOracle,
                        false,
                        XyceStaticTranContract::PlainStatic
                    ) | (
                        XyceStaticTranPlanPurpose::GeneratedReferenceRelationalFamily,
                        true,
                        XyceStaticTranContract::WrapperStatic
                            | XyceStaticTranContract::WrapperCsv
                            | XyceStaticTranContract::WrapperCsd
                    ) | (
                        XyceStaticTranPlanPurpose::Bug308SonSteppedTempOutputFramingRelationalFamily,
                        true,
                        XyceStaticTranContract::WrapperStatic
                    ) | (
                        XyceStaticTranPlanPurpose::Bug372MultiplicityRelationalFamily,
                        true,
                        XyceStaticTranContract::WrapperStatic
                    ) | (
                        XyceStaticTranPlanPurpose::RelationalFamily
                            | XyceStaticTranPlanPurpose::AgeCapRelationalFamily
                            | XyceStaticTranPlanPurpose::ScopedModelRelationalFamily
                            | XyceStaticTranPlanPurpose::Bug1797RelationalFamily
                            | XyceStaticTranPlanPurpose::Bug805RelationalFamily
                            | XyceStaticTranPlanPurpose::ClassicMosParameterAliasRelationalFamily
                            | XyceStaticTranPlanPurpose::Bug1190SonProcessParameterRelationalFamily
                            | XyceStaticTranPlanPurpose::Bug1284TransientRestartRelationalFamily,
                        false,
                        XyceStaticTranContract::PlainStatic
                            | XyceStaticTranContract::PlainCsv
                            | XyceStaticTranContract::PlainCsd
                    ) | (
                        XyceStaticTranPlanPurpose::Bug308SonSteppedTempOutputFramingRelationalFamily,
                        false,
                        XyceStaticTranContract::PlainStatic
                    ) | (
                        XyceStaticTranPlanPurpose::Bug372MultiplicityRelationalFamily,
                        false,
                        XyceStaticTranContract::PlainStatic
                    ) | (
                        XyceStaticTranPlanPurpose::RelationalFamily
                            | XyceStaticTranPlanPurpose::AgeCapRelationalFamily
                            | XyceStaticTranPlanPurpose::ScopedModelRelationalFamily
                            | XyceStaticTranPlanPurpose::Bug1797RelationalFamily
                            | XyceStaticTranPlanPurpose::Bug805RelationalFamily
                            | XyceStaticTranPlanPurpose::ClassicMosParameterAliasRelationalFamily
                            | XyceStaticTranPlanPurpose::Bug1190SonProcessParameterRelationalFamily,
                        true,
                        XyceStaticTranContract::WrapperStatic
                            | XyceStaticTranContract::WrapperCsv
                            | XyceStaticTranContract::WrapperCsd
                    )
                );
                if !no_file_shape {
                    return Err(format!(
                        "transient plan purpose {purpose:?} and contract {:?} require an explicit waveform or scalar oracle",
                        self.contract
                    ));
                }
                self.require_print("fileless transient plan")?;
                Ok(())
            }
        }
    }

    fn validate_executable_oracle_shape(&self) -> Result<(), String> {
        match &self.oracle {
            XyceStaticTranOracle::Waveform(path) => {
                if !path.is_file() {
                    return Err(format!(
                        "transient waveform oracle does not exist at {}",
                        path.display()
                    ));
                }
                self.require_print("waveform transient execution")?;
                Ok(())
            }
            XyceStaticTranOracle::ScalarMeasurements {
                reference_paths,
                input,
                ..
            } => {
                if reference_paths.len() != 1
                    || !reference_paths[0].is_file()
                    || !self.steps.is_empty()
                    || self.output_override
                    || self.contract != XyceStaticTranContract::WrapperStatic
                    || self.comparison_mode != XyceStaticTranComparisonMode::Pointwise
                    || matches!(
                        input,
                        XyceScalarTranMeasurementInput::Remeasure(path) if !path.is_file()
                    )
                {
                    return Err(
                        "scalar TRAN execution requires one checked oracle, no STEP/output override, and the pointwise WrapperStatic contract"
                            .to_string(),
                    );
                }
                Ok(())
            }
            XyceStaticTranOracle::AuthoredFailValue { measurements } => {
                if measurements.is_empty()
                    || !self.steps.is_empty()
                    || self.output_override
                    || self.comparison_mode != XyceStaticTranComparisonMode::Pointwise
                    || !matches!(
                        self.contract,
                        XyceStaticTranContract::PlainStatic
                            | XyceStaticTranContract::PlainCsv
                            | XyceStaticTranContract::PlainCsd
                    )
                {
                    return Err(
                        "authored FAILVALUE TRAN execution requires an ordinary unstepped, pointwise plan with at least one typed measurement"
                            .to_string(),
                    );
                }
                Ok(())
            }
            XyceStaticTranOracle::None
                if self.contract == XyceStaticTranContract::WrapperNoIndexHeader =>
            {
                self.require_print("NOINDEX transient execution")?;
                Ok(())
            }
            XyceStaticTranOracle::None => Err(
                "ordinary transient execution requires an explicit waveform or scalar oracle"
                    .to_string(),
            ),
        }
    }

    fn result_contract(&self) -> &'static str {
        if self.is_authored_fail_value_measurement_only() {
            return XYCE_SELF_VERIFIED_MEASUREMENT_TRAN_CONTRACT;
        }
        if self.is_scalar_measurement_only() {
            return "wrapper_scalar_measure_tran";
        }
        if !self.steps.is_empty() {
            // Comparison mode is an implementation detail of the upstream
            // verifier. Preserve the established stepped-output contract name
            // so callers can identify the deck/output shape independently of
            // whether its values use pointwise or integrated-RMS comparison.
            return self.contract.result_contract(true);
        }
        match self.comparison_mode {
            XyceStaticTranComparisonMode::Release710IntegratedRms { .. }
            | XyceStaticTranComparisonMode::Release710IntegratedRmsComp { .. } => {
                "static_xyce_verify_prn_tran"
            }
            XyceStaticTranComparisonMode::Pointwise => {
                self.contract.result_contract(!self.steps.is_empty())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceAnalyticRcSourceContract {
    capacitor_name: String,
    capacitor_nodes: [String; 2],
    capacitance_bits: u64,
    initial_voltage_bits: u64,
    resistor_name: String,
    resistor_nodes: [String; 2],
    resistance_bits: u64,
    source_name: String,
    source_nodes: [String; 2],
    source_value_bits: u64,
    probe_node: String,
    tran_step_bits: u64,
    tran_stop_bits: u64,
    reltol_bits: u64,
    abstol_bits: u64,
    transient_lte_reference: Option<TransientLteReference>,
}

#[derive(Debug, Clone)]
struct XyceAnalyticRcSpecification {
    output_node: String,
    source_value: Value,
    initial_voltage: Value,
    resistance: Value,
    capacitance: Value,
    time_constant: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceAnalyticRcKind {
    GeneratedWrapper,
    PassiveTemperature,
}

impl XyceAnalyticRcKind {
    fn result_contract(self) -> &'static str {
        match self {
            Self::GeneratedWrapper => "analytic_first_order_rc_tran_wrapper",
            Self::PassiveTemperature => "analytic_passive_temperature_rc_tran",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::GeneratedWrapper => "analytic first-order RC",
            Self::PassiveTemperature => "analytic passive-temperature RC",
        }
    }
}

#[derive(Debug, Clone)]
struct XyceAnalyticRcContract {
    plan: XyceStaticTranPlan,
    specification: XyceAnalyticRcSpecification,
    kind: XyceAnalyticRcKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceBug546TemperatureRcMember {
    Model,
    ScalarInstance,
    VectorInstance,
}

impl XyceBug546TemperatureRcMember {
    const ALL: [Self; 3] = [Self::Model, Self::ScalarInstance, Self::VectorInstance];

    fn for_record(relative_path: &str) -> Option<Self> {
        let record = XyceTestRunner::normalize_manifest_key(relative_path);
        Self::ALL
            .into_iter()
            .find(|member| member.record() == record)
    }

    fn record(self) -> &'static str {
        match self {
            Self::Model => "netlists/certification_tests/bug_546_son/tempcap.cir",
            Self::ScalarInstance => "netlists/certification_tests/bug_546_son/tempcap_instance.cir",
            Self::VectorInstance => {
                "netlists/certification_tests/bug_546_son/tempcap_instance2.cir"
            }
        }
    }

    fn source_relative_path(self) -> &'static str {
        match self {
            Self::Model => "Netlists/Certification_Tests/BUG_546_SON/tempcap.cir",
            Self::ScalarInstance => "Netlists/Certification_Tests/BUG_546_SON/tempcap_instance.cir",
            Self::VectorInstance => {
                "Netlists/Certification_Tests/BUG_546_SON/tempcap_instance2.cir"
            }
        }
    }

    fn representation(self) -> XycePassiveTemperatureRepresentation {
        match self {
            Self::Model => XycePassiveTemperatureRepresentation::Model,
            Self::ScalarInstance => XycePassiveTemperatureRepresentation::ScalarInstance,
            Self::VectorInstance => XycePassiveTemperatureRepresentation::VectorInstance,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceAnalyticSinusoidalRcSourceContract {
    capacitor_name: String,
    capacitor_nodes: [String; 2],
    capacitance_bits: u64,
    resistor_name: String,
    resistor_nodes: [String; 2],
    resistance_bits: u64,
    source_name: String,
    source_nodes: [String; 2],
    source_offset_bits: u64,
    source_amplitude_bits: u64,
    source_frequency_bits: u64,
    source_delay_bits: u64,
    source_damping_bits: u64,
    probe_node: String,
    print_expression: String,
    print_offset_bits: u64,
    tran_step_bits: u64,
    tran_stop_bits: u64,
    timeint_reltol_bits: u64,
    timeint_abstol_bits: u64,
    method_selector: String,
    verify_reltol_bits: u64,
    verify_abstol_bits: u64,
}

#[derive(Debug, Clone)]
struct XyceAnalyticSinusoidalRcSpecification {
    output_node: String,
    print_expression: String,
    resistance: Value,
    capacitance: Value,
    source_frequency: Value,
    print_offset: Value,
}

#[derive(Debug, Clone)]
struct XyceAnalyticSinusoidalRcContract {
    plan: XyceStaticTranPlan,
    specification: XyceAnalyticSinusoidalRcSpecification,
    tolerance: XyceVerifyTransientTolerance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceAnalyticIntegerKind {
    Fmod,
    IntFloorCeil,
}

impl XyceAnalyticIntegerKind {
    fn dc_result_contract(self) -> &'static str {
        match self {
            Self::Fmod => "analytic_fmod_dc_wrapper",
            Self::IntFloorCeil => "analytic_int_floor_ceil_bsource_dc_wrapper",
        }
    }
}

#[derive(Debug, Clone)]
struct XyceAnalyticIntegerDcContract {
    plan: XyceStaticDcPlan,
    kind: XyceAnalyticIntegerKind,
}

#[derive(Debug, Clone)]
struct XyceAnalyticIntFloorCeilTranContract {
    plan: XyceStaticTranPlan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceResistorDtempRole {
    Owner,
    Reference,
}

impl XyceResistorDtempRole {
    fn result_contract(self) -> &'static str {
        match self {
            Self::Owner => "resistor_dtemp_relational_wrapper_owner",
            Self::Reference => "resistor_dtemp_relational_wrapper_reference",
        }
    }
}

#[derive(Debug, Clone)]
struct XyceResistorDtempContract {
    owner_plan: XyceStaticDcPlan,
    reference_plan: XyceStaticDcPlan,
    role: XyceResistorDtempRole,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceResistorDtempSnapshot {
    resistor_name: String,
    resistor_nodes: [String; 2],
    resistance_bits: u64,
    model_name: String,
    source_name: String,
    source_nodes: [String; 2],
    source_value_bits: u64,
    model_type: String,
    model_params: Vec<(String, u64)>,
    dc_source: String,
    dc_start_bits: u64,
    dc_stop_bits: u64,
    dc_step_bits: u64,
    probes: Vec<String>,
    effective_temperature_bits: Vec<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceBug647ResistorRole {
    Owner,
    ModelParameterReference,
}

impl XyceBug647ResistorRole {
    fn result_contract(self) -> &'static str {
        match self {
            Self::Owner => "bug647_resistor_relational_wrapper_owner",
            Self::ModelParameterReference => "bug647_resistor_relational_wrapper_model_reference",
        }
    }
}

#[derive(Debug, Clone)]
struct XyceBug647ResistorContract {
    owner_plan: XyceStaticDcPlan,
    reference_plan: XyceStaticDcPlan,
    role: XyceBug647ResistorRole,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceBug655ContinuationRole {
    ColumnZeroOwner,
    LeadingSpaceReference,
}

impl XyceBug655ContinuationRole {
    fn result_contract(self) -> &'static str {
        match self {
            Self::ColumnZeroOwner => "bug655_continuation_relational_wrapper_owner",
            Self::LeadingSpaceReference => {
                "bug655_continuation_relational_wrapper_spaced_reference"
            }
        }
    }
}

#[derive(Debug, Clone)]
struct XyceBug655ContinuationContract {
    owner_plan: XyceStaticDcPlan,
    reference_plan: XyceStaticDcPlan,
    role: XyceBug655ContinuationRole,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceBug655ContinuationSnapshot {
    title: String,
    elements: BTreeMap<String, XyceRelationalElementFingerprint>,
    model_name: String,
    model_type: String,
    model_params: Vec<(String, u64)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceBug662HeaderRole {
    LongHeaderOwner,
    ShortHeaderReference,
}

impl XyceBug662HeaderRole {
    fn result_contract(self) -> &'static str {
        match self {
            Self::LongHeaderOwner => "bug662_long_header_relational_wrapper_owner",
            Self::ShortHeaderReference => "bug662_long_header_relational_wrapper_short_reference",
        }
    }
}

#[derive(Debug, Clone)]
struct XyceBug662HeaderContract {
    owner_plan: XyceStaticTranPlan,
    reference_plan: XyceStaticTranPlan,
    role: XyceBug662HeaderRole,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceBug667NodesetRole {
    ScopedOwner,
    ExplicitHierarchicalReference,
}

impl XyceBug667NodesetRole {
    fn result_contract(self) -> &'static str {
        match self {
            Self::ScopedOwner => "bug667_nodeset_relational_wrapper_owner",
            Self::ExplicitHierarchicalReference => {
                "bug667_nodeset_relational_wrapper_explicit_reference"
            }
        }
    }
}

#[derive(Debug, Clone)]
struct XyceBug667NodesetContract {
    owner_plan: XyceStaticTranPlan,
    reference_plan: XyceStaticTranPlan,
    role: XyceBug667NodesetRole,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceBug754GlobalParameterRole {
    GlobalParameterOwner,
    LiteralReference,
}

impl XyceBug754GlobalParameterRole {
    fn result_contract(self) -> &'static str {
        match self {
            Self::GlobalParameterOwner => "bug754_global_parameter_dc_relational_wrapper_owner",
            Self::LiteralReference => {
                "bug754_global_parameter_dc_relational_wrapper_literal_reference"
            }
        }
    }
}

#[derive(Debug, Clone)]
struct XyceBug754GlobalParameterContract {
    owner_plan: XyceStaticDcPlan,
    reference_plan: XyceStaticDcPlan,
    role: XyceBug754GlobalParameterRole,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceBug754GlobalParameterSnapshot {
    elements: BTreeMap<String, XyceRelationalElementFingerprint>,
    model_name: String,
    model_type: String,
    model_params: Vec<(String, u64)>,
    dc_source: String,
    dc_start_bits: u64,
    dc_stop_bits: u64,
    dc_step_bits: u64,
    probes: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceStaticTranPlanPurpose {
    /// Compare a simulation directly with a checked-in Xyce waveform. This
    /// path admits only device families whose absolute transient behavior is
    /// covered by the native oracle contract.
    AbsoluteOracle,
    /// Compare a checked-in ordinary PRN waveform for the narrowly qualified
    /// bare Xyce LEVEL=9 BSIM3 subset using Release 7.10's default integrated
    /// `xyce_verify` contract on the simulator's own adaptive output grid.
    /// This is an oracle-bounded compatibility envelope, not a declaration of
    /// general BSIM3 3.2.2/3.3.0 model-version equivalence.
    DefaultLevel9XyceVerifyOracle,
    /// Compare two independently simulated representations of the same
    /// circuit. The freshly simulated baseline is the oracle, so a redundant
    /// per-member gold file and absolute device-model eligibility are neither
    /// required nor consulted.
    RelationalFamily,
    /// Exact structural AGE/D-versus-expression capacitor equivalence. AGE/D
    /// eligibility is scoped to this dedicated proof.
    AgeCapRelationalFamily,
    /// Parse a manifest-marked wrapper whose reference waveforms are generated
    /// by independently simulated sibling decks. Admission to this purpose is
    /// deliberately available only after a dedicated family selector has
    /// proven the wrapper/sibling provenance and naming contract.
    GeneratedReferenceRelationalFamily,
    /// Reproduce the exact Certification BUG 308 SON stepped-TEMP wrapper
    /// against its two independently executed fixed-TEMP controls. Admission
    /// to the native LEVEL=9 BSIM3 comparator envelope is confined to the
    /// dedicated provenance-bound output-framing contract.
    Bug308SonSteppedTempOutputFramingRelationalFamily,
    /// Reproduce the exact Certification BUG 372 explicit-parallel GOOD
    /// versus instance-multiplicity TEST wrappers. Admission is confined to
    /// the dedicated four-owner provenance/topology contract; this purpose
    /// does not widen the ordinary relational MOS envelope.
    Bug372MultiplicityRelationalFamily,
    /// Compare the exact Certification BUG 1190 SON process-parameter alias
    /// owners with their direct model-parameter controls. Admission is scoped
    /// to the dedicated provenance-bound diode family contract.
    Bug1190SonProcessParameterRelationalFamily,
    /// Reproduce the exact BUG 1284 save/resume wrapper using independently
    /// simulated worker decks and no checked-in numerical gold.
    Bug1284TransientRestartRelationalFamily,
    /// Compare scoped-model and explicitly expanded representations under an
    /// exact qualified-topology, model-parameter, and waveform-parity
    /// contract. This purpose has a separately qualified native BJT envelope
    /// so ordinary relational families cannot gain BJT eligibility by
    /// association.
    ScopedModelRelationalFamily,
    /// Compare the exact Certification BUG 1797 bare BSIM3 LEVEL=9 and
    /// LEVEL=49 one-shot representations.  This dedicated purpose admits no
    /// general BSIM3 relational family; provenance and the complete typed
    /// circuit envelope are owned by `contracts_bug1797`.
    Bug1797RelationalFamily,
    /// Compare the exact Certification BUG 805 canonical, HSPICE-alias, and
    /// PSPICE-alias legacy BJT oscillator decks. This dedicated purpose keeps
    /// the expanded alias surface confined to the provenance-bound family.
    Bug805RelationalFamily,
    /// Compare the exact MOSFET_ParamAliases UO/VTO and U0/VT0 model-card
    /// representations across Berkeley MOS LEVEL=1/2/3/6. This purpose is
    /// confined to the provenance-bound eight-deck family and does not widen
    /// the ordinary relational MOSFET envelope.
    ClassicMosParameterAliasRelationalFamily,
    /// Execute a wrapper-origin transient deck whose oracle is generated
    /// analytically on the simulator's own default-PRN time grid. The
    /// dedicated analytic contract supplies the missing reference and proves
    /// the exact bounded circuit/source/options envelope separately.
    AnalyticOracle,
    /// Execute one exact DIODE_ANALYTIC Release-7.10 generated-gold wrapper.
    /// This purpose admits only the dedicated three-card legacy-diode envelope;
    /// it does not widen the ordinary absolute waveform contract.
    DiodeAnalyticOracle,
    /// Execute one exact BJT_ANALYTIC or NMOS_ANALYTIC Release-7.10
    /// generated-gold wrapper. The dedicated contract owns the bounded
    /// legacy-device topology, startup, and analytic-law proof.
    LegacyDeviceAnalyticOracle,
    /// Execute one ordinary BUG546 passive-temperature RC member against an
    /// independently derived first-order analytic waveform on the simulator's
    /// own default-PRN grid. Admission is limited to the exact three-member
    /// family and validates model/instance temperature precedence separately.
    PassiveTemperatureAnalyticOracle,
}

impl XyceStaticTranPlanPurpose {
    fn requires_reference_file(self) -> bool {
        matches!(
            self,
            Self::AbsoluteOracle | Self::DefaultLevel9XyceVerifyOracle
        )
    }

    fn validates_absolute_device_contract(self) -> bool {
        matches!(
            self,
            Self::AbsoluteOracle
                | Self::DefaultLevel9XyceVerifyOracle
                | Self::AnalyticOracle
                | Self::DiodeAnalyticOracle
                | Self::LegacyDeviceAnalyticOracle
                | Self::PassiveTemperatureAnalyticOracle
        )
    }

    fn admits_default_level9_bsim3(self) -> bool {
        matches!(
            self,
            Self::DefaultLevel9XyceVerifyOracle
                | Self::Bug308SonSteppedTempOutputFramingRelationalFamily
        )
    }
}

#[derive(Debug, Clone)]
struct XyceStaticAcPlan {
    deck_path: PathBuf,
    reference_path: Option<PathBuf>,
    measurement_reference_paths: Vec<PathBuf>,
    continuous_measurement_reference_paths: Vec<PathBuf>,
    measurement_tolerance: XyceFileCompareTolerance,
    source: String,
    print: Option<XycePrintRequest>,
    primary_ac_file: Option<String>,
    primary_ac_ic_file: Option<String>,
    sensitivity: Option<XyceStaticAcSensitivityPlan>,
    output_override: bool,
    ac: XyceAcAnalysis,
    frequency_bound: bool,
    steps: Vec<StepCommand>,
    contract: XyceStaticAcContract,
}

#[derive(Debug, Clone)]
struct XyceStaticAcSensitivityPlan {
    reference_path: PathBuf,
    reference_format: XyceAcSensitivityReferenceFormat,
    print: XycePrintRequest,
    objectives: Vec<XyceAcSensitivityObjective>,
    parameters: Vec<String>,
    direct: bool,
    adjoint: bool,
    no_index: bool,
    /// Additional `.PRINT SENS FILE=...` destinations.  Xyce creates one
    /// sensitivity table per destination, while the numerical sensitivities
    /// are shared; each side output therefore reuses the canonical solve but
    /// has its own probe schema and checked-in oracle.
    side_outputs: Vec<XyceStaticAcSensitivitySideOutput>,
}

#[derive(Debug, Clone)]
struct XyceStaticAcSensitivitySideOutput {
    file: String,
    reference_path: PathBuf,
    reference_format: XyceAcSensitivityReferenceFormat,
    print: XycePrintRequest,
    no_index: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceAcSensitivityReferenceFormat {
    Prn,
    Csv,
}

#[derive(Debug, Clone)]
struct XyceAcSensitivityObjective {
    authored_name: String,
    spec: XyceAcSensitivityObjectiveSpec,
}

#[derive(Debug, Clone)]
enum XyceAcSensitivityObjectiveSpec {
    Voltage {
        positive: String,
        negative: Option<String>,
    },
    BranchCurrent(String),
}

#[derive(Debug, Clone)]
struct XyceRelationalAcPlan {
    deck_path: PathBuf,
    source: String,
    print: XycePrintRequest,
    ac: XyceAcAnalysis,
    frequency_bound: bool,
}

#[derive(Debug, Clone)]
struct XyceStaticNoisePlan {
    deck_path: PathBuf,
    source: String,
    print: Option<XycePrintRequest>,
    output_override: bool,
    reference_path: Option<PathBuf>,
    side_references: Vec<XyceStaticNoiseSideReference>,
    measurement_reference_paths: Vec<PathBuf>,
    continuous_measurement_reference_paths: Vec<PathBuf>,
    gs_reference_path: Option<PathBuf>,
    measurement_tolerance: XyceFileCompareTolerance,
    output_node: String,
    reference_node: Option<String>,
    input_source: String,
    frequencies: Vec<Value>,
    data_points: Option<Vec<XyceFrequencyDataPoint>>,
    data_table_name: Option<String>,
    steps: Vec<StepCommand>,
    contract: XyceStaticNoiseContract,
}

#[derive(Debug, Clone)]
struct XyceStaticNoiseSideReference {
    file: String,
    print: XycePrintRequest,
    reference_path: PathBuf,
    contract: XyceStaticNoiseContract,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceStaticNoiseContract {
    StdPrn,
    NoIndexPrn,
    ProbeFallbackPrn,
    RawFallbackPrn,
    TouchstoneFallbackPrn,
    GnuplotPrn,
    SplotPrn,
    Csv,
    Tecplot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceStaticNoiseOutputFamily {
    Prn,
    Csv,
    Tecplot,
}

impl XyceStaticNoiseContract {
    fn for_format(format: Option<&str>) -> Result<Self, String> {
        let normalized = format.unwrap_or("STD").trim();
        match normalized.to_ascii_uppercase().as_str() {
            "STD" => Ok(Self::StdPrn),
            "NOINDEX" => Ok(Self::NoIndexPrn),
            // Xyce NOISE deliberately falls these output selectors back to
            // its ordinary whitespace PRN writer.
            "PROBE" => Ok(Self::ProbeFallbackPrn),
            "RAW" => Ok(Self::RawFallbackPrn),
            "TOUCHSTONE" | "TOUCHSTONE2" => Ok(Self::TouchstoneFallbackPrn),
            "GNUPLOT" => Ok(Self::GnuplotPrn),
            "SPLOT" => Ok(Self::SplotPrn),
            "CSV" => Ok(Self::Csv),
            "TECPLOT" => Ok(Self::Tecplot),
            _ => Err(format!(
                "native NOISE oracle does not cover FORMAT={normalized}"
            )),
        }
    }

    fn output_family(self) -> XyceStaticNoiseOutputFamily {
        match self {
            Self::Csv => XyceStaticNoiseOutputFamily::Csv,
            Self::Tecplot => XyceStaticNoiseOutputFamily::Tecplot,
            _ => XyceStaticNoiseOutputFamily::Prn,
        }
    }

    fn reference_extension(self) -> &'static str {
        match self {
            Self::Csv => "NOISE.csv",
            Self::Tecplot => "NOISE.dat",
            _ => "NOISE.prn",
        }
    }

    fn result_contract(self, stepped: bool) -> &'static str {
        match (self, stepped) {
            (Self::Csv, false) => "static_csv_noise",
            (Self::Csv, true) => "static_csv_step_noise",
            (Self::NoIndexPrn, false) => "static_noindex_prn_noise",
            (Self::NoIndexPrn, true) => "static_noindex_prn_step_noise",
            (Self::GnuplotPrn | Self::SplotPrn, false) => "static_gnuplot_prn_noise",
            (Self::GnuplotPrn | Self::SplotPrn, true) => "static_gnuplot_prn_step_noise",
            (Self::Tecplot, false) => "static_tecplot_noise",
            (Self::Tecplot, true) => "static_tecplot_step_noise",
            (
                Self::StdPrn
                | Self::ProbeFallbackPrn
                | Self::RawFallbackPrn
                | Self::TouchstoneFallbackPrn,
                _,
            ) => {
                if stepped {
                    "static_prn_step_noise"
                } else {
                    "static_prn_noise"
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum XyceMeasurementReferenceValue {
    Failed,
    Numeric {
        value: Value,
        /// Smallest decimal unit represented by the checked-in artifact.
        /// Xyce measurement files normally print seven significant digits,
        /// so the parsed binary value alone does not retain the oracle's
        /// rounding uncertainty.
        quantization: Option<Value>,
    },
}

#[derive(Debug, Clone, PartialEq)]
struct XyceMeasurementReference {
    name: String,
    value: XyceMeasurementReferenceValue,
}

#[derive(Debug, Clone, PartialEq)]
struct XyceContinuousMeasurementReferenceRecord {
    value: XyceMeasurementReferenceValue,
    trigger_axis: Option<XyceMeasurementReferenceValue>,
    target_axis: Option<XyceMeasurementReferenceValue>,
}

#[derive(Debug, Clone, PartialEq)]
struct XyceContinuousMeasurementReference {
    name: String,
    records: Vec<XyceContinuousMeasurementReferenceRecord>,
}

#[derive(Debug, Clone, PartialEq)]
struct XyceMixedMeasurementReferenceRow {
    name: String,
    value: XyceMeasurementReferenceValue,
    trigger_axis: Option<XyceMeasurementReferenceValue>,
    target_axis: Option<XyceMeasurementReferenceValue>,
}

#[derive(Debug, Clone, PartialEq)]
struct XyceMeasureContGsRow {
    mixed: XyceMixedMeasurementReferenceRow,
    event_axis: Option<XyceMeasurementReferenceValue>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct XyceFileCompareTolerance {
    absolute: Value,
    relative: Value,
    zero: Value,
}

impl XyceFileCompareTolerance {
    const BUG1085_USER_FUNCTION: Self = Self {
        absolute: 1.0e-6,
        relative: 1.0e-2,
        zero: 1.0e-12,
    };

    const BUG1190_MUTUAL_INDUCTOR: Self = Self {
        absolute: 1.0e-6,
        relative: 1.0e-2,
        zero: 1.0e-12,
    };

    const BUG1190_SON_PROCESS_PARAMETER: Self = Self {
        absolute: 1.0e-6,
        relative: 1.0e-2,
        zero: 1.0e-12,
    };

    const MEASURE_COMMON_DEFAULT: Self = Self {
        absolute: 1.0e-5,
        relative: 1.0e-3,
        zero: 1.0e-10,
    };

    const MEASURE_CONT_STEP_REMEASURE: Self = Self {
        absolute: 3.0e-3,
        relative: 2.0e-2,
        zero: 1.0e-5,
    };

    const MEASURE_COMMON_AC_INTEGRATION: Self = Self {
        absolute: 1.0e-2,
        relative: 1.0e-3,
        zero: 1.0e-10,
    };

    // A DERIV result is a difference quotient of two independently solved
    // points. Compound expressions can therefore accumulate roughly twice
    // the base absolute solver error while remaining well inside the relative
    // contract. Keep this narrow tolerance separate from scalar measures.
    const MEASURE_COMMON_DERIVATIVE: Self = Self {
        absolute: 2.0e-5,
        relative: 1.0e-3,
        zero: 1.0e-10,
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceStaticHbOutputFormat {
    StdPrn,
    GnuplotPrn,
}

impl XyceStaticHbOutputFormat {
    fn for_format(format: Option<&str>) -> Result<Self, String> {
        match format.unwrap_or("STD").trim().to_ascii_uppercase().as_str() {
            "STD" => Ok(Self::StdPrn),
            // Xyce's HB GNUPLOT writer uses the ordinary whitespace PRN
            // representation for the numerical artifacts.  Keep this
            // contract explicit so other format selectors cannot silently
            // inherit PRN semantics without a matching oracle parser.
            "GNUPLOT" => Ok(Self::GnuplotPrn),
            normalized => Err(format!(
                "native HB oracle does not cover FORMAT={normalized}"
            )),
        }
    }

    fn reference_extension(self) -> &'static str {
        match self {
            Self::StdPrn | Self::GnuplotPrn => "HB.FD.prn",
        }
    }

    fn result_contract(self, wrapper: bool) -> &'static str {
        match (wrapper, self) {
            (true, Self::StdPrn) => "wrapper_static_prn_hb",
            (true, Self::GnuplotPrn) => "wrapper_static_gnuplot_prn_hb",
            (false, Self::StdPrn) => "static_prn_hb",
            (false, Self::GnuplotPrn) => "static_gnuplot_prn_hb",
        }
    }
}

#[derive(Debug, Clone)]
struct XyceStaticHbPlan {
    deck_path: PathBuf,
    source: String,
    print: XycePrintRequest,
    frequency: Value,
    num_harmonics: usize,
    fd_reference_path: PathBuf,
    td_reference_path: PathBuf,
    ic_reference_path: PathBuf,
    output_format: XyceStaticHbOutputFormat,
    wrapper: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceStaticTranContract {
    PlainStatic,
    PlainCsv,
    PlainCsd,
    WrapperStatic,
    WrapperNoIndexHeader,
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
            // Header-only wrapper contracts retain the existing public result
            // label. The distinction is an internal oracle/execution detail,
            // not a new externally reported analysis format.
            (Self::WrapperNoIndexHeader, false) => "wrapper_static_prn_tran",
            (Self::WrapperNoIndexHeader, true) => "wrapper_static_prn_step_tran",
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

    fn requires_reference_file(self) -> bool {
        !matches!(self, Self::WrapperNoIndexHeader)
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
    comparison: XyceBaselineFamilyComparison,
    family: String,
    baseline_path: PathBuf,
    member_paths: Vec<PathBuf>,
    target_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
struct XyceSharedSteppedDcFamilyContract {
    family: String,
    owner_path: PathBuf,
    baseline_path: PathBuf,
    member_paths: Vec<PathBuf>,
    prn_reference_path: PathBuf,
    res_reference_path: PathBuf,
    role: XyceSharedSteppedDcFamilyRole,
}

#[derive(Debug, Clone)]
struct XyceNumberedRedefinitionDcFamilyContract {
    family: String,
    owner_path: PathBuf,
    baseline_path: PathBuf,
    member_paths: Vec<PathBuf>,
    parameter_redefinition_policy: ParameterRedefinitionPolicy,
    role: XyceNumberedRedefinitionDcFamilyRole,
}

#[derive(Debug, Clone)]
struct XyceVbicDcWrapperFamilyContract {
    family: String,
    owner_path: PathBuf,
    multiplicity_path: PathBuf,
    polarity_path: PathBuf,
    reference_path: PathBuf,
    role: XyceVbicDcWrapperFamilyRole,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceVbicDcWrapperFamilyRole {
    Owner,
    MultiplicityControl,
    PolarityControl,
}

impl XyceVbicDcWrapperFamilyRole {
    fn contract(self) -> &'static str {
        match self {
            Self::Owner => "vbic_dc_wrapper_equivalence_family_owner",
            Self::MultiplicityControl => "vbic_dc_wrapper_equivalence_family_multiplicity_control",
            Self::PolarityControl => "vbic_dc_wrapper_equivalence_family_polarity_control",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceNumberedRedefinitionDcFamilyRole {
    Owner,
    Baseline,
    Member(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceNumberedRedefinitionRepresentation {
    LiteralBaseline,
    DependentFormalExpression,
    DependentInstanceExpression,
}

impl XyceNumberedRedefinitionDcFamilyRole {
    fn contract(self) -> &'static str {
        match self {
            Self::Owner => "numbered_redefinition_dc_family_owner",
            Self::Baseline => "numbered_redefinition_dc_family_baseline",
            Self::Member(_) => "numbered_redefinition_dc_family_member",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceSharedSteppedDcFamilyRole {
    Owner,
    Baseline(XyceSharedSteppedDcRepresentation),
    Member(XyceSharedSteppedDcRepresentation),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceSharedSteppedDcRepresentation {
    DirectIdentity,
    HierarchicalIdentity,
    DirectTransform,
    TransformInSubcircuitBody,
    FunctionCallInSubcircuitBody,
    FunctionCallAtInstance,
}

impl XyceSharedSteppedDcFamilyRole {
    fn contract(self) -> &'static str {
        match self {
            Self::Owner => "shared_stepped_dc_oracle_family_owner",
            Self::Baseline(_) => "shared_stepped_dc_oracle_family_baseline",
            Self::Member(_) => "shared_stepped_dc_oracle_family_member",
        }
    }
}

#[derive(Debug, Clone)]
struct XyceAgeCapFamilyContract {
    relational: XyceBaselineFamilyContract,
    role: XyceAgeCapFamilyRole,
}

#[derive(Debug, Clone)]
struct XyceParams1FamilyContract {
    relational: XyceBaselineFamilyContract,
    role: XyceParams1Role,
}

#[derive(Debug, Clone)]
struct XyceNakedAlgebraFamilyContract {
    relational: XyceBaselineFamilyContract,
    role: XyceNakedAlgebraRole,
}

#[derive(Debug, Clone)]
struct XyceBug1826ThermalParameterFamilyContract {
    relational: XyceBaselineFamilyContract,
    role: XyceBug1826ThermalParameterRole,
}

#[derive(Debug, Clone)]
struct XyceSourceMultiplicityFamilyContract {
    relational: XyceBaselineFamilyContract,
    role: XyceSourceMultiplicityRole,
}

#[derive(Debug, Clone)]
struct XyceAbmFrequencyFamilyContract {
    relational: XyceBaselineFamilyContract,
    owner_path: PathBuf,
    control_path: PathBuf,
    spec: &'static XyceAbmFrequencyCaseSpec,
    role: XyceAbmFrequencyRole,
}

#[derive(Debug, Clone)]
struct XyceBug1043AcDataParameterFamilyContract {
    relational: XyceBaselineFamilyContract,
    owner_path: PathBuf,
    baseline_path: PathBuf,
    role: XyceBug1043AcDataParameterRole,
}

#[derive(Debug, Clone)]
struct XyceAbmLookupOrderFamilyContract {
    relational: XyceBaselineFamilyContract,
    role: XyceAbmLookupOrderRole,
}

#[derive(Debug, Clone)]
struct XyceBug38FamilyContract {
    relational: XyceBaselineFamilyContract,
    owner_path: PathBuf,
    control_path: PathBuf,
    role: XyceBug38Role,
}

#[derive(Debug, Clone)]
struct XyceBug39GaussianContract {
    anchor_path: PathBuf,
    role: XyceBug39GaussianRole,
}

#[derive(Debug, Clone)]
struct XyceBug39DeterministicContract {
    deck_path: PathBuf,
    reference_path: PathBuf,
    role: XyceBug39DeterministicRole,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceBug39DeterministicRole {
    Int,
    LimitNominal,
    Pow,
    Sign,
}

impl XyceBug39DeterministicRole {
    #[cfg(test)]
    const ALL: [Self; 4] = [Self::Int, Self::LimitNominal, Self::Pow, Self::Sign];

    fn result_contract(self) -> &'static str {
        match self {
            Self::Int => XYCE_BUG39_INT_CONTRACT,
            Self::LimitNominal => XYCE_BUG39_LIMIT_CONTRACT,
            Self::Pow => XYCE_BUG39_POW_CONTRACT,
            Self::Sign => XYCE_BUG39_SIGN_CONTRACT,
        }
    }

    fn path(self) -> &'static str {
        match self {
            Self::Int => XYCE_BUG39_INT_PATH,
            Self::LimitNominal => XYCE_BUG39_LIMIT_PATH,
            Self::Pow => XYCE_BUG39_POW_PATH,
            Self::Sign => XYCE_BUG39_SIGN_PATH,
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::Int => XYCE_BUG39_INT_RECORD,
            Self::LimitNominal => XYCE_BUG39_LIMIT_RECORD,
            Self::Pow => XYCE_BUG39_POW_RECORD,
            Self::Sign => XYCE_BUG39_SIGN_RECORD,
        }
    }

    fn reference_file_name(self) -> &'static str {
        match self {
            Self::Int => "bug39_int.cir.prn.gs",
            Self::LimitNominal => "bug39_limit.cir.prn.gs",
            Self::Pow => "bug39_pow.cir.prn.gs",
            Self::Sign => "bug39_sign.cir.prn.gs",
        }
    }

    fn expected_resistance(self) -> Value {
        match self {
            Self::Int => 3.0,
            Self::LimitNominal | Self::Sign => 1.0,
            Self::Pow => 8.0,
        }
    }

    fn for_record(relative_path: &str) -> Option<Self> {
        match XyceTestRunner::normalize_manifest_key(relative_path).as_str() {
            XYCE_BUG39_INT_RECORD => Some(Self::Int),
            XYCE_BUG39_LIMIT_RECORD => Some(Self::LimitNominal),
            XYCE_BUG39_POW_RECORD => Some(Self::Pow),
            XYCE_BUG39_SIGN_RECORD => Some(Self::Sign),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceBug39GaussianRole {
    AgaussAbsolute,
    GaussRelative,
}

impl XyceBug39GaussianRole {
    fn result_contract(self) -> &'static str {
        match self {
            Self::AgaussAbsolute => XYCE_BUG39_AGAUSS_CONTRACT,
            Self::GaussRelative => XYCE_BUG39_GAUSS_CONTRACT,
        }
    }

    fn path(self) -> &'static str {
        match self {
            Self::AgaussAbsolute => XYCE_BUG39_AGAUSS_PATH,
            Self::GaussRelative => XYCE_BUG39_GAUSS_PATH,
        }
    }

    fn generated_file_name(self) -> &'static str {
        match self {
            Self::AgaussAbsolute => "testagauss.cir",
            Self::GaussRelative => "testgauss.cir",
        }
    }

    fn for_record(relative_path: &str) -> Option<Self> {
        match XyceTestRunner::normalize_manifest_key(relative_path).as_str() {
            XYCE_BUG39_AGAUSS_RECORD => Some(Self::AgaussAbsolute),
            XYCE_BUG39_GAUSS_RECORD => Some(Self::GaussRelative),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct XyceBug39GaussianMoments {
    mean: Value,
    population_standard_deviation: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceBug402TemperatureRole {
    WrapperOwner,
    XyceDeviceReference,
    SpiceCompatibilityMember,
}

impl XyceBug402TemperatureRole {
    const ALL: [Self; 3] = [
        Self::WrapperOwner,
        Self::XyceDeviceReference,
        Self::SpiceCompatibilityMember,
    ];

    fn result_contract(self) -> &'static str {
        match self {
            Self::WrapperOwner => XYCE_BUG402_OWNER_CONTRACT,
            Self::XyceDeviceReference => XYCE_BUG402_XYCE_REFERENCE_CONTRACT,
            Self::SpiceCompatibilityMember => XYCE_BUG402_SPICE_MEMBER_CONTRACT,
        }
    }

    fn path(self) -> &'static str {
        match self {
            Self::WrapperOwner => XYCE_BUG402_OWNER_PATH,
            Self::XyceDeviceReference => XYCE_BUG402_XYCE_REFERENCE_PATH,
            Self::SpiceCompatibilityMember => XYCE_BUG402_SPICE_MEMBER_PATH,
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::WrapperOwner => XYCE_BUG402_OWNER_RECORD,
            Self::XyceDeviceReference => XYCE_BUG402_XYCE_REFERENCE_RECORD,
            Self::SpiceCompatibilityMember => XYCE_BUG402_SPICE_MEMBER_RECORD,
        }
    }

    fn for_record(relative_path: &str) -> Option<Self> {
        let record = XyceTestRunner::normalize_manifest_key(relative_path);
        Self::ALL.into_iter().find(|role| role.record() == record)
    }
}

#[derive(Debug, Clone)]
struct XyceBug402TemperatureContract {
    xyce_reference_plan: XyceStaticDcPlan,
    spice_member_plan: XyceStaticDcPlan,
    role: XyceBug402TemperatureRole,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceBug402ModelFingerprint {
    model_type: String,
    numeric_params: Vec<(String, u64)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceBug402TemperatureSnapshot {
    temperature_bits: u64,
    gmin_bits: u64,
    sweep_source: String,
    sweep_start_bits: u64,
    sweep_stop_bits: u64,
    sweep_step_bits: u64,
    probes: Vec<String>,
    top_level_elements: BTreeMap<String, XyceRelationalElementFingerprint>,
    subcircuit_name: String,
    subcircuit_ports: Vec<String>,
    subcircuit_elements: BTreeMap<String, XyceRelationalElementFingerprint>,
    flattened_elements: BTreeMap<String, XyceRelationalElementFingerprint>,
    models: BTreeMap<String, XyceBug402ModelFingerprint>,
    flattened_models: BTreeMap<String, XyceBug402ModelFingerprint>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceBug38Role {
    WrapperOwner,
    ParenthesizedControl,
}

impl XyceBug38Role {
    fn result_contract(self) -> &'static str {
        match self {
            Self::WrapperOwner => XYCE_BUG38_WRAPPER_OWNER_CONTRACT,
            Self::ParenthesizedControl => XYCE_BUG38_PARENTHESIZED_CONTROL_CONTRACT,
        }
    }

    fn for_record(relative_path: &str) -> Option<Self> {
        match XyceTestRunner::normalize_manifest_key(relative_path).as_str() {
            XYCE_BUG38_OWNER_RECORD => Some(Self::WrapperOwner),
            XYCE_BUG38_CONTROL_RECORD => Some(Self::ParenthesizedControl),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceAbmLookupOrderRole {
    WrapperOwner,
    SortedControl,
}

impl XyceAbmLookupOrderRole {
    fn result_contract(self) -> &'static str {
        match self {
            Self::WrapperOwner => XYCE_ABM_LOOKUP_ORDER_WRAPPER_OWNER_CONTRACT,
            Self::SortedControl => XYCE_ABM_LOOKUP_ORDER_SORTED_CONTROL_CONTRACT,
        }
    }

    fn for_record(relative_path: &str) -> Option<(&'static XyceAbmLookupOrderCaseSpec, Self)> {
        let relative = XyceTestRunner::normalize_manifest_key(relative_path);
        XYCE_ABM_LOOKUP_ORDER_CASES.iter().find_map(|spec| {
            if relative == spec.owner_record {
                Some((spec, Self::WrapperOwner))
            } else if relative == spec.control_record {
                Some((spec, Self::SortedControl))
            } else {
                None
            }
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceAbmFrequencyRole {
    WrapperOwner,
    DataControl,
}

impl XyceAbmFrequencyRole {
    fn result_contract(self) -> &'static str {
        match self {
            Self::WrapperOwner => XYCE_ABM_FREQUENCY_WRAPPER_OWNER_CONTRACT,
            Self::DataControl => XYCE_ABM_FREQUENCY_DATA_CONTROL_CONTRACT,
        }
    }

    fn for_record(relative_path: &str) -> Option<(&'static XyceAbmFrequencyCaseSpec, Self)> {
        let relative = XyceTestRunner::normalize_manifest_key(relative_path);
        XYCE_ABM_FREQUENCY_CASES.iter().find_map(|spec| {
            if relative == spec.owner_record {
                Some((spec, Self::WrapperOwner))
            } else if relative == spec.control_record {
                Some((spec, Self::DataControl))
            } else {
                None
            }
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceBug1043AcDataParameterRole {
    DataWrapperOwner,
    ExpressionBaseline,
}

impl XyceBug1043AcDataParameterRole {
    fn result_contract(self) -> &'static str {
        match self {
            Self::DataWrapperOwner => XYCE_BUG1043_AC_DATA_PARAMETER_WRAPPER_OWNER_CONTRACT,
            Self::ExpressionBaseline => XYCE_BUG1043_AC_DATA_PARAMETER_EXPRESSION_BASELINE_CONTRACT,
        }
    }

    fn for_record(
        relative_path: &str,
    ) -> Option<(&'static XyceBug1043AcDataParameterCaseSpec, Self)> {
        let relative = XyceTestRunner::normalize_manifest_key(relative_path);
        let spec = &XYCE_BUG1043_AC_DATA_PARAMETER_CASE;
        if relative == spec.owner_record {
            Some((spec, Self::DataWrapperOwner))
        } else if relative == spec.baseline_record {
            Some((spec, Self::ExpressionBaseline))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceSourceMultiplicityRole {
    WrapperOwner,
    Baseline,
}

impl XyceSourceMultiplicityRole {
    fn result_contract(self) -> &'static str {
        match self {
            Self::WrapperOwner => XYCE_SOURCE_MULTIPLICITY_WRAPPER_CONTRACT,
            Self::Baseline => XYCE_SOURCE_MULTIPLICITY_BASELINE_CONTRACT,
        }
    }

    fn for_record(relative_path: &str) -> Option<(&'static XyceSourceMultiplicityCaseSpec, Self)> {
        let relative = XyceTestRunner::normalize_manifest_key(relative_path);
        XYCE_SOURCE_MULTIPLICITY_CASES.iter().find_map(|spec| {
            if relative == spec.owner_record {
                Some((spec, Self::WrapperOwner))
            } else if relative == spec.baseline_record {
                Some((spec, Self::Baseline))
            } else {
                None
            }
        })
    }
}

#[derive(Debug, Clone)]
struct XyceSwitchStateCaseFamilyContract {
    relational: XyceBaselineFamilyContract,
    role: XyceSwitchStateCaseFamilyRole,
}

#[derive(Debug, Clone)]
struct XyceDiodeModelAliasFamilyContract {
    relational: XyceBaselineFamilyContract,
    role: XyceDiodeModelAliasFamilyRole,
}

#[derive(Debug, Clone)]
struct XyceNestedIncludeIdentityFamilyContract {
    relational: XyceBaselineFamilyContract,
    role: XyceNestedIncludeIdentityFamilyRole,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceNestedIncludeIdentityFamilyRole {
    Anchor,
    RepeatedTargetBaseline,
    SplitIdenticalTargetsMember,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceNestedIncludeIdentityRepresentation {
    RepeatedCanonicalTarget,
    SplitIdenticalTargets,
}

#[derive(Debug, Clone)]
struct XyceNestedIncludeProvenance {
    representation: XyceNestedIncludeIdentityRepresentation,
    canonical_source: String,
    expanded_source: String,
    support_paths: BTreeSet<PathBuf>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceDiodeModelAliasFamilyRole {
    Anchor,
    CanonicalBaseline,
    AliasMember,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceSwitchStateCaseFamilyRole {
    Anchor,
    UppercaseBaseline,
    LowercaseMember,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceAgeCapFamilyRole {
    Anchor,
    AgedBaseline,
    EquivalentMember,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceParams1Role {
    WrapperOwner,
    LiteralBaseline,
    ParameterizedMember,
}

impl XyceParams1Role {
    fn result_contract(self) -> &'static str {
        match self {
            Self::WrapperOwner => XYCE_PARAMS1_WRAPPER_OWNER_CONTRACT,
            Self::LiteralBaseline => XYCE_PARAMS1_LITERAL_BASELINE_CONTRACT,
            Self::ParameterizedMember => XYCE_PARAMS1_PARAMETERIZED_MEMBER_CONTRACT,
        }
    }

    fn for_record(relative_path: &str) -> Option<Self> {
        match XyceTestRunner::normalize_manifest_key(relative_path).as_str() {
            XYCE_PARAMS1_OWNER_RECORD => Some(Self::WrapperOwner),
            XYCE_PARAMS1_LITERAL_BASELINE_RECORD => Some(Self::LiteralBaseline),
            XYCE_PARAMS1_PARAMETERIZED_MEMBER_RECORD => Some(Self::ParameterizedMember),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceNakedAlgebraRole {
    WrapperOwner,
    BracedBaseline,
    GlobalMember,
}

impl XyceNakedAlgebraRole {
    fn result_contract(self) -> &'static str {
        match self {
            Self::WrapperOwner => XYCE_NAKED_ALGEBRA_WRAPPER_OWNER_CONTRACT,
            Self::BracedBaseline => XYCE_NAKED_ALGEBRA_BRACED_BASELINE_CONTRACT,
            Self::GlobalMember => XYCE_NAKED_ALGEBRA_GLOBAL_MEMBER_CONTRACT,
        }
    }

    fn for_record(relative_path: &str) -> Option<Self> {
        match XyceTestRunner::normalize_manifest_key(relative_path).as_str() {
            XYCE_NAKED_ALGEBRA_OWNER_RECORD => Some(Self::WrapperOwner),
            XYCE_NAKED_ALGEBRA_BRACED_BASELINE_RECORD => Some(Self::BracedBaseline),
            XYCE_NAKED_ALGEBRA_GLOBAL_MEMBER_RECORD => Some(Self::GlobalMember),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceBug1826ThermalParameterRole {
    WrapperOwner,
    GlobalBaseline,
    LocalMember,
}

impl XyceBug1826ThermalParameterRole {
    fn result_contract(self) -> &'static str {
        match self {
            Self::WrapperOwner => XYCE_BUG1826_THERMAL_PARAMETER_WRAPPER_OWNER_CONTRACT,
            Self::GlobalBaseline => XYCE_BUG1826_THERMAL_PARAMETER_GLOBAL_BASELINE_CONTRACT,
            Self::LocalMember => XYCE_BUG1826_THERMAL_PARAMETER_LOCAL_MEMBER_CONTRACT,
        }
    }

    fn for_record(relative_path: &str) -> Option<Self> {
        match XyceTestRunner::normalize_manifest_key(relative_path).as_str() {
            XYCE_BUG1826_THERMAL_PARAMETER_OWNER_RECORD => Some(Self::WrapperOwner),
            XYCE_BUG1826_THERMAL_PARAMETER_GLOBAL_BASELINE_RECORD => Some(Self::GlobalBaseline),
            XYCE_BUG1826_THERMAL_PARAMETER_LOCAL_MEMBER_RECORD => Some(Self::LocalMember),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
struct XyceSubcktParameterResolutionFamilyContract {
    family: String,
    anchor_path: PathBuf,
    error_path: PathBuf,
    baseline_path: PathBuf,
    valid_paths: Vec<PathBuf>,
    role: XyceSubcktParameterResolutionRole,
    target_path: PathBuf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceSubcktParameterResolutionRole {
    Anchor,
    Baseline,
    Member,
    ExpectedError,
}

#[derive(Debug, Clone)]
struct XyceSteppedIcReferenceContract {
    family: String,
    owner_path: PathBuf,
    member_paths: Vec<PathBuf>,
    target_path: PathBuf,
}

#[derive(Debug, Clone)]
struct XyceNonlinearCoreModelStepReferenceContract {
    family: String,
    owner_path: PathBuf,
    member_paths: Vec<PathBuf>,
    target_path: PathBuf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceClassicMosDtempRole {
    Owner,
    Reference,
}

impl XyceClassicMosDtempRole {
    fn result_contract(self) -> &'static str {
        match self {
            Self::Owner => XYCE_CLASSIC_MOS_DTEMP_WRAPPER_CONTRACT,
            Self::Reference => XYCE_CLASSIC_MOS_DTEMP_REFERENCE_CONTRACT,
        }
    }

    fn for_record(relative_path: &str) -> Option<(String, Self)> {
        let normalized = relative_path.replace('\\', "/").to_ascii_lowercase();
        let file_name = normalized.strip_prefix("netlists/dtemp/")?;
        if file_name.contains('/') {
            return None;
        }
        let (family, role) = if let Some(family) = file_name.strip_suffix("_dtemp.cir") {
            (family, Self::Owner)
        } else if let Some(family) = file_name.strip_suffix("_ref.cir") {
            (family, Self::Reference)
        } else {
            return None;
        };
        let level_tag = family
            .strip_prefix("nmos")
            .or_else(|| family.strip_prefix("pmos"))?;
        if !matches!(level_tag, "1" | "2" | "3" | "6") {
            return None;
        }
        Some((family.to_string(), role))
    }
}

#[derive(Debug, Clone)]
struct XyceClassicMosDtempContract {
    owner_plan: XyceStaticDcPlan,
    reference_plan: XyceStaticDcPlan,
    family: String,
    role: XyceClassicMosDtempRole,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum XyceClassicMosDtempElementSnapshot {
    Resistor {
        name: String,
        nodes: Vec<String>,
        value_bits: u64,
    },
    VoltageSource {
        name: String,
        nodes: Vec<String>,
        dc_value_bits: u64,
    },
    Mosfet {
        name: String,
        nodes: Vec<String>,
        model: String,
        polarity: String,
        instance_params: Vec<(String, u64)>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceClassicMosDtempSnapshot {
    elements: Vec<XyceClassicMosDtempElementSnapshot>,
    model_name: String,
    model_type: String,
    model_params: Vec<(String, u64)>,
    dc_primary: (String, u64, u64, u64),
    dc_secondary: Option<(String, u64, u64, u64)>,
    probes: Vec<String>,
    effective_temperature_bits: Vec<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceLegacyBjtDtempFamily {
    Npn,
    Pnp,
}

impl XyceLegacyBjtDtempFamily {
    fn owner_file(self) -> &'static str {
        match self {
            Self::Npn => "npn_dtemp.cir",
            Self::Pnp => "pnp_dtemp.cir",
        }
    }

    fn reference_file(self) -> &'static str {
        match self {
            Self::Npn => "npn_ref.cir",
            Self::Pnp => "pnp_ref.cir",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Npn => "NPN",
            Self::Pnp => "PNP",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceLegacyBjtDtempRole {
    Owner,
    Reference,
}

impl XyceLegacyBjtDtempRole {
    fn result_contract(self) -> &'static str {
        match self {
            Self::Owner => XYCE_LEGACY_BJT_DTEMP_WRAPPER_CONTRACT,
            Self::Reference => XYCE_LEGACY_BJT_DTEMP_REFERENCE_CONTRACT,
        }
    }

    fn for_record(relative_path: &str) -> Option<(XyceLegacyBjtDtempFamily, Self)> {
        match XyceTestRunner::normalize_manifest_key(relative_path).as_str() {
            XYCE_LEGACY_BJT_DTEMP_NPN_OWNER_RECORD => {
                Some((XyceLegacyBjtDtempFamily::Npn, Self::Owner))
            }
            XYCE_LEGACY_BJT_DTEMP_NPN_REFERENCE_RECORD => {
                Some((XyceLegacyBjtDtempFamily::Npn, Self::Reference))
            }
            XYCE_LEGACY_BJT_DTEMP_PNP_OWNER_RECORD => {
                Some((XyceLegacyBjtDtempFamily::Pnp, Self::Owner))
            }
            XYCE_LEGACY_BJT_DTEMP_PNP_REFERENCE_RECORD => {
                Some((XyceLegacyBjtDtempFamily::Pnp, Self::Reference))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
struct XyceLegacyBjtDtempContract {
    owner_plan: XyceStaticDcPlan,
    reference_plan: XyceStaticDcPlan,
    family: XyceLegacyBjtDtempFamily,
    role: XyceLegacyBjtDtempRole,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceLegacyBjtDtempSnapshot {
    elements: BTreeMap<String, XyceRelationalElementFingerprint>,
    model_name: String,
    model_type: String,
    model_params: Vec<(String, u64)>,
    dc_primary: (String, u64, u64, u64),
    dc_secondary: Option<(String, u64, u64, u64)>,
    probes: Vec<String>,
    effective_temperature_bits: Vec<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceSydneyLevel1JfetDtempFamily {
    Njf,
    Pjf,
}

impl XyceSydneyLevel1JfetDtempFamily {
    fn owner_file(self) -> &'static str {
        match self {
            Self::Njf => "njfet_dtemp.cir",
            Self::Pjf => "pjfet_dtemp.cir",
        }
    }

    fn reference_file(self) -> &'static str {
        match self {
            Self::Njf => "njfet_ref.cir",
            Self::Pjf => "pjfet_ref.cir",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Njf => "NJF",
            Self::Pjf => "PJF",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceSydneyLevel1JfetDtempRole {
    Owner,
    Reference,
}

impl XyceSydneyLevel1JfetDtempRole {
    fn result_contract(self) -> &'static str {
        match self {
            Self::Owner => XYCE_SYDNEY_LEVEL1_JFET_DTEMP_WRAPPER_CONTRACT,
            Self::Reference => XYCE_SYDNEY_LEVEL1_JFET_DTEMP_REFERENCE_CONTRACT,
        }
    }

    fn for_record(relative_path: &str) -> Option<(XyceSydneyLevel1JfetDtempFamily, Self)> {
        match XyceTestRunner::normalize_manifest_key(relative_path).as_str() {
            XYCE_SYDNEY_LEVEL1_JFET_DTEMP_NJF_OWNER_RECORD => {
                Some((XyceSydneyLevel1JfetDtempFamily::Njf, Self::Owner))
            }
            XYCE_SYDNEY_LEVEL1_JFET_DTEMP_NJF_REFERENCE_RECORD => {
                Some((XyceSydneyLevel1JfetDtempFamily::Njf, Self::Reference))
            }
            XYCE_SYDNEY_LEVEL1_JFET_DTEMP_PJF_OWNER_RECORD => {
                Some((XyceSydneyLevel1JfetDtempFamily::Pjf, Self::Owner))
            }
            XYCE_SYDNEY_LEVEL1_JFET_DTEMP_PJF_REFERENCE_RECORD => {
                Some((XyceSydneyLevel1JfetDtempFamily::Pjf, Self::Reference))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
struct XyceSydneyLevel1JfetDtempContract {
    owner_plan: XyceStaticDcPlan,
    reference_plan: XyceStaticDcPlan,
    family: XyceSydneyLevel1JfetDtempFamily,
    role: XyceSydneyLevel1JfetDtempRole,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceSydneyLevel1JfetDtempSnapshot {
    elements: BTreeMap<String, XyceRelationalElementFingerprint>,
    model_name: String,
    model_type: String,
    model_params: Vec<(String, u64)>,
    dc_primary: (String, u64, u64, u64),
    dc_secondary: Option<(String, u64, u64, u64)>,
    probes: Vec<String>,
    effective_temperature_bits: Vec<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceLevel2DiodeDtempRole {
    Owner,
    Reference,
}

impl XyceLevel2DiodeDtempRole {
    fn result_contract(self) -> &'static str {
        match self {
            Self::Owner => XYCE_LEVEL2_DIODE_DTEMP_WRAPPER_CONTRACT,
            Self::Reference => XYCE_LEVEL2_DIODE_DTEMP_REFERENCE_CONTRACT,
        }
    }

    fn for_record(relative_path: &str) -> Option<Self> {
        match XyceTestRunner::normalize_manifest_key(relative_path).as_str() {
            XYCE_LEVEL2_DIODE_DTEMP_OWNER_RECORD => Some(Self::Owner),
            XYCE_LEVEL2_DIODE_DTEMP_REFERENCE_RECORD => Some(Self::Reference),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
struct XyceLevel2DiodeDtempContract {
    owner_plan: XyceStaticTranPlan,
    reference_plan: XyceStaticTranPlan,
    role: XyceLevel2DiodeDtempRole,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceLevel2DiodeDtempSnapshot {
    elements: BTreeMap<String, XyceRelationalElementFingerprint>,
    model_name: String,
    model_type: String,
    model_params: Vec<(String, u64)>,
    tran_step_bits: u64,
    tran_stop_bits: u64,
    tran_start_bits: Option<u64>,
    tran_max_step_bits: Option<u64>,
    probes: Vec<String>,
    effective_temperature_bits: Vec<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceCapacitorDtempRole {
    Owner,
    Reference,
}

impl XyceCapacitorDtempRole {
    fn result_contract(self) -> &'static str {
        match self {
            Self::Owner => XYCE_CAPACITOR_DTEMP_WRAPPER_CONTRACT,
            Self::Reference => XYCE_CAPACITOR_DTEMP_REFERENCE_CONTRACT,
        }
    }

    fn for_record(relative_path: &str) -> Option<Self> {
        match XyceTestRunner::normalize_manifest_key(relative_path).as_str() {
            XYCE_CAPACITOR_DTEMP_OWNER_RECORD => Some(Self::Owner),
            XYCE_CAPACITOR_DTEMP_REFERENCE_RECORD => Some(Self::Reference),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
struct XyceCapacitorDtempContract {
    owner_plan: XyceStaticTranPlan,
    reference_plan: XyceStaticTranPlan,
    role: XyceCapacitorDtempRole,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceCapacitorDtempSnapshot {
    elements: BTreeMap<String, XyceRelationalElementFingerprint>,
    model_name: String,
    model_type: String,
    model_params: Vec<(String, u64)>,
    tran_step_bits: u64,
    tran_stop_bits: u64,
    tran_start_bits: Option<u64>,
    tran_max_step_bits: Option<u64>,
    probes: Vec<String>,
    effective_temperature_bits: Vec<u64>,
    timeint_reltol_bits: u64,
    timeint_abstol_bits: u64,
}

#[derive(Debug, Clone)]
struct XyceBug1190MutualInductorContract {
    family: String,
    owner_path: PathBuf,
    baseline_path: PathBuf,
    target_path: PathBuf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceBug1190MutualInductorKind {
    Linear,
    NonlinearCore,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceBug1190ModelFingerprint {
    model_type: String,
    numeric_bits: Vec<(String, u64)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceBug1190MutualInductorSnapshot {
    kind: XyceBug1190MutualInductorKind,
    title: String,
    elements: BTreeMap<String, XyceRelationalElementFingerprint>,
    models: BTreeMap<String, XyceBug1190ModelFingerprint>,
    swept_inductor_bits: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceNonlinearCoreModelStepSnapshot {
    title: String,
    elements: BTreeMap<String, XyceRelationalElementFingerprint>,
    source_name: String,
    inductor_names: [String; 2],
    inductor_signal_nodes: [String; 2],
    model_name: String,
    model_level: u8,
    model_numeric_bits: Vec<(String, u64)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceSteppedIcSnapshot {
    elements: BTreeMap<String, XyceRelationalElementFingerprint>,
    initial_conditions: Vec<(String, u64)>,
    capacitor_name: String,
    capacitor_value_bits: u64,
}

#[derive(Debug, Clone)]
struct XycePassivePrimaryCompositeContract {
    family: String,
    owner_path: PathBuf,
    capacitor_tran: XyceBaselineFamilyContract,
    resistor_dc: XyceBaselineFamilyContract,
    target_path: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceScopedModelFamilySnapshot {
    elements: BTreeMap<String, XyceRelationalElementFingerprint>,
    bjt_model_bits: BTreeMap<String, (u64, u64)>,
    diode_model_bits: BTreeMap<String, u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceSinExpressionFamilySnapshot {
    resistor: XyceRelationalElementFingerprint,
    resistor_name: String,
    source_nodes: Vec<String>,
    waveform_bits: [u64; 6],
    representation: XyceSinExpressionRepresentation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceParamExpressionFamilySnapshot {
    title: String,
    parameter_name: String,
    parameter_bits: u64,
    subcircuit_name: String,
    subcircuit_ports: Vec<String>,
    flattened_elements: BTreeMap<String, XyceRelationalElementFingerprint>,
    representation: XyceParamExpressionRepresentation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceParams1Snapshot {
    representation: XyceParams1Representation,
    title: String,
    elements: BTreeMap<String, XyceRelationalElementFingerprint>,
    tran_step_bits: u64,
    tran_stop_bits: u64,
    ordered_probes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceNakedAlgebraSnapshot {
    representation: XyceNakedAlgebraRepresentation,
    title: String,
    parameter_bits: BTreeMap<String, u64>,
    elements: BTreeMap<String, XyceRelationalElementFingerprint>,
    tran_step_bits: u64,
    tran_stop_bits: u64,
    ordered_probes: Vec<String>,
    option_directives: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceBug1826ThermalParameterSnapshot {
    representation: XyceBug1826ThermalParameterRepresentation,
    title: String,
    parameter_name: String,
    parameter_bits: u64,
    elements: BTreeMap<String, XyceRelationalElementFingerprint>,
    model_name: String,
    model_type: String,
    model_numeric_bits: Vec<(String, u64)>,
    model_expressions: BTreeMap<String, XyceExpressionAstFingerprint>,
    runtime_resistor: XyceThermalResistorRuntimeFingerprint,
    tran_step_bits: u64,
    tran_stop_bits: u64,
    ordered_probes: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceSourceMultiplicityAnalysis {
    Dc,
    Tran,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceSourceMultiplicityRepresentation {
    LinearBaseline,
    LinearDirect,
    BehavioralDirect,
    BehavioralFormal,
    BehavioralInherited,
    BehavioralNested,
    ExpressionDirect,
    ExpressionFormal,
    ExpressionInherited,
    ExpressionNested,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceSourceMultiplicitySnapshot {
    analysis: XyceSourceMultiplicityAnalysis,
    representation: XyceSourceMultiplicityRepresentation,
    flattened_elements: BTreeMap<String, XyceRelationalElementFingerprint>,
    effective_gain_bits: u64,
    source_nodes: [String; 2],
    control_nodes: [String; 2],
    authored_multiplicity_bits: u64,
    authored_multiplicity_given: bool,
    flattened_multiplicity_bits: u64,
    flattened_multiplicity_given: bool,
    hierarchy_multiplicity_bits: Vec<u64>,
    ordered_probes: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceAbmFrequencyKind {
    BehavioralCurrent,
    Resistor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceAbmFrequencyVariable {
    Freq,
    Hertz,
}

impl XyceAbmFrequencyVariable {
    fn name(self) -> &'static str {
        match self {
            Self::Freq => "freq",
            Self::Hertz => "hertz",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceAbmFrequencyRepresentation {
    RuntimeDecadeExpression,
    DataTableControl,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceAbmFrequencySnapshot {
    kind: XyceAbmFrequencyKind,
    variable: XyceAbmFrequencyVariable,
    representation: XyceAbmFrequencyRepresentation,
    frequency_bits: Vec<u64>,
    effective_resistance_bits: Vec<u64>,
    source_nodes: [String; 2],
    source_ac_bits: [u64; 2],
    source_transient_bits: [u64; 6],
    load_nodes: [String; 2],
    capacitance_bits: u64,
    runtime_expression: Option<XyceExpressionAstFingerprint>,
    data_overrides: Vec<Vec<(String, u64)>>,
    ordered_probes: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceBug1043AcDataParameterRepresentation {
    DataTableOwner,
    RuntimeExpressionBaseline,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceBug1043AcDataParameterRow {
    frequency_bits: u64,
    magnitude_bits: u64,
    phase_bits: u64,
    resistance_bits: u64,
    capacitance_bits: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceBug1043AcDataParameterSnapshot {
    representation: XyceBug1043AcDataParameterRepresentation,
    frequency_bits: Vec<u64>,
    effective_rows: Vec<XyceBug1043AcDataParameterRow>,
    source_nodes: [String; 2],
    resistor_nodes: [String; 2],
    capacitor_nodes: [String; 2],
    runtime_expressions: BTreeMap<String, XyceExpressionAstFingerprint>,
    data_overrides: Vec<Vec<(String, u64)>>,
    ordered_probes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceThermalResistorRuntimeFingerprint {
    name: String,
    length_bits: u64,
    area_bits: u64,
    thermal_length_bits: u64,
    thermal_area_bits: u64,
    multiplicity_bits: u64,
    scale_bits: u64,
    temperature_celsius_bits: u64,
    resistivity_bits: u64,
    heat_capacity_bits: u64,
    thermal_heat_capacity_bits: u64,
    reported_resistance_bits: u64,
    output_resistance_bits: u64,
    output_conductance_bits: u64,
    tnom_celsius_bits: u64,
    model_numeric_bits: Vec<(String, u64)>,
    model_expressions: BTreeMap<String, XyceExpressionAstFingerprint>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceAgeCapFamilySnapshot {
    representation: XyceAgeCapRepresentation,
    elements: BTreeMap<String, XyceRelationalElementFingerprint>,
    ordered_probes: Vec<String>,
    option_directives: Vec<String>,
    age_semantics: [u64; 4],
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceSwitchStateCaseFamilySnapshot {
    representation: XyceSwitchStateCaseRepresentation,
    canonical_source: String,
    elements: BTreeMap<String, XyceRelationalElementFingerprint>,
    control_expression: XyceExpressionAstFingerprint,
    model_name: String,
    model_type: String,
    model_numeric_bits: Vec<(String, u64)>,
    ordered_probes: Vec<String>,
    runtime_switch: XyceGenericSwitchRuntimeFingerprint,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceDiodeModelAliasFamilySnapshot {
    representation: XyceDiodeModelAliasRepresentation,
    canonical_source: String,
    elements: BTreeMap<String, XyceRelationalElementFingerprint>,
    model_name: String,
    model_type: String,
    canonical_model_bits: Vec<(String, u64)>,
    ordered_probes: Vec<String>,
    runtime_diode: XyceNativeDiodeRuntimeFingerprint,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceNativeDiodeRuntimeFingerprint {
    name: String,
    node_anode: usize,
    node_cathode: usize,
    numeric_bits: Vec<u64>,
    boolean_state: Vec<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceGenericSwitchRuntimeFingerprint {
    name: String,
    node_pos: usize,
    node_neg: usize,
    numeric_bits: [u64; 6],
    hysteresis_enabled: bool,
    time_breakpoint_bits: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XycePassivePrimaryValueSnapshot {
    title: String,
    device_kind: XycePassivePrimaryKind,
    representation: XycePassivePrimaryRepresentation,
    active_source_fingerprint: Vec<String>,
    model_name: String,
    model_type: String,
    model_numeric_bits: Vec<(String, u64)>,
    elements: BTreeMap<String, XyceRelationalElementFingerprint>,
    effective_primary_bits: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XycePassiveTemperatureOverrideSnapshot {
    title: String,
    device_kind: XycePassiveTemperatureDeviceKind,
    representation: XycePassiveTemperatureRepresentation,
    elements: BTreeMap<String, XyceRelationalElementFingerprint>,
    model_name: String,
    model_type: String,
    model_tc_bits: [u64; 2],
    model_tnom_bits: Option<u64>,
    winning_tc_bits: [u64; 2],
    effective_primary_bits: u64,
    option_directives: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceTransientAnalysisExpressionSnapshot {
    title: String,
    representation: XyceTransientAnalysisRepresentation,
    elements: BTreeMap<String, XyceRelationalElementFingerprint>,
    option_directives: Vec<String>,
    parameter_bits: BTreeMap<String, u64>,
    nonrepresentation_source: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceDcAnalysisExpressionSnapshot {
    representation: XyceDcAnalysisRepresentation,
    parameter_bits: BTreeMap<String, u64>,
    nonrepresentation_source: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceDelimitedExpressionFamilySnapshot {
    representation: XyceDelimitedExpressionRepresentation,
    expression_sites: BTreeMap<String, XyceExpressionAstFingerprint>,
    parameter_bits: BTreeMap<String, u64>,
    elements: BTreeMap<String, XyceRelationalElementFingerprint>,
    print_probes: Vec<XycePrintSemanticFingerprint>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum XycePrintSemanticFingerprint {
    Atomic(String),
    Expression(XyceExpressionAstFingerprint),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum XyceExpressionAstFingerprint {
    Number(u64),
    Complex(u64, u64),
    String(String),
    Parameter(String),
    Binary(
        rspice_core::netlist::expr::BinOpKind,
        Box<XyceExpressionAstFingerprint>,
        Box<XyceExpressionAstFingerprint>,
    ),
    Unary(
        rspice_core::netlist::expr::UnaryOpKind,
        Box<XyceExpressionAstFingerprint>,
    ),
    Function(String, Vec<XyceExpressionAstFingerprint>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceAcAnalysisExpressionSnapshot {
    representation: XyceAcAnalysisRepresentation,
    parameter_bits: BTreeMap<String, u64>,
    nonrepresentation_source: Vec<String>,
    footer_suppressed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum XyceStrictTransientFamilySnapshot {
    AgeCap(XyceAgeCapFamilySnapshot),
    DiodeModelAlias(XyceDiodeModelAliasFamilySnapshot),
    SwitchStateCase(XyceSwitchStateCaseFamilySnapshot),
    ScopedModel(XyceScopedModelFamilySnapshot),
    SinExpression(XyceSinExpressionFamilySnapshot),
    ParamExpression(XyceParamExpressionFamilySnapshot),
    Params1(XyceParams1Snapshot),
    NakedAlgebra(XyceNakedAlgebraSnapshot),
    Bug1826ThermalParameter(XyceBug1826ThermalParameterSnapshot),
    SourceMultiplicity(XyceSourceMultiplicitySnapshot),
    PassivePrimaryValue(XycePassivePrimaryValueSnapshot),
    PassiveTemperatureOverride(XycePassiveTemperatureOverrideSnapshot),
    TransientAnalysisExpression(XyceTransientAnalysisExpressionSnapshot),
    Bug1085(contracts_bug1085::XyceBug1085UserFunctionSnapshot),
    Bug38(XyceBug38FamilySnapshot),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceBug38FamilySnapshot {
    representation: XyceBug38SubcktRepresentation,
    semantic_source: Vec<String>,
    top_level_elements: BTreeMap<String, XyceRelationalElementFingerprint>,
    subcircuit_name: String,
    subcircuit_ports: Vec<String>,
    subcircuit_elements: BTreeMap<String, XyceRelationalElementFingerprint>,
    flattened_elements: BTreeMap<String, XyceRelationalElementFingerprint>,
    tran_step_bits: u64,
    tran_stop_bits: u64,
    ordered_probes: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceBug38SubcktRepresentation {
    BareFormals,
    ParenthesizedFormals,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceAgeCapRepresentation {
    NativeAge,
    ParameterExpression,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceSwitchStateCaseRepresentation {
    Lowercase,
    Uppercase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceDiodeModelAliasRepresentation {
    Canonical,
    Alias,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum XyceStrictDcFamilySnapshot {
    AbmLookupOrder(XyceAbmLookupOrderSnapshot),
    BjtExternalNode(XyceBjtExternalNodeFamilySnapshot),
    DcAnalysisExpression(XyceDcAnalysisExpressionSnapshot),
    DelimitedExpression(XyceDelimitedExpressionFamilySnapshot),
    PassivePrimaryValue(XycePassivePrimaryValueSnapshot),
    SubcktParameterPrecedence(XyceSubcktParameterPrecedenceSnapshot),
    SubcktParameterResolution(XyceSubcktParameterResolutionSnapshot),
    NestedIncludeIdentity(XyceNestedIncludeIdentityFamilySnapshot),
    SourceMultiplicity(XyceSourceMultiplicitySnapshot),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceAbmLookupOrderSnapshot {
    kind: XyceAbmLookupKind,
    representation: XyceAbmLookupRepresentation,
    authored_points_bits: Vec<(u64, u64)>,
    canonical_points_bits: Vec<(u64, u64)>,
    elements: BTreeMap<String, XyceRelationalElementFingerprint>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceAbmLookupKind {
    Akima,
    Table,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceAbmLookupRepresentation {
    OutOfOrderOwner,
    SortedControl,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceNestedIncludeIdentityFamilySnapshot {
    title: String,
    hierarchy: Vec<XyceNestedIncludeSubcircuitFingerprint>,
    flattened_elements: BTreeMap<String, XyceRelationalElementFingerprint>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct XyceNestedIncludeSubcircuitFingerprint {
    name: String,
    ports: Vec<String>,
    elements: BTreeMap<String, XyceRelationalElementFingerprint>,
    nested_names: Vec<String>,
}

// One variant per AC family the strict suite qualifies, and the families
// really do differ in payload size — the ABM one is already boxed for that
// reason. Each snapshot is built once, compared once, and never collected, so
// the spread costs a qualification run nothing; equalizing it would mean
// boxing a payload across nine construction and match sites in three modules
// to move bytes that are never copied in bulk.
#[expect(
    clippy::large_enum_variant,
    reason = "each snapshot is built once and compared once; the variant spread is never collected"
)]
#[derive(Debug, Clone, PartialEq, Eq)]
enum XyceStrictAcFamilySnapshot {
    AcAnalysisExpression(XyceAcAnalysisExpressionSnapshot),
    AbmFrequency(Box<XyceAbmFrequencySnapshot>),
    Bug1043AcDataParameters(XyceBug1043AcDataParameterSnapshot),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceSubcktParameterPrecedenceSnapshot {
    elements: Vec<XyceRelationalElementFingerprint>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceSubcktParameterResolutionSnapshot {
    representation: XyceSubcktParameterResolutionRepresentation,
    parameter_name: String,
    flattened_elements: Vec<XyceRelationalElementFingerprint>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum XyceSubcktParameterResolutionRepresentation {
    FormalDefaultAndInstanceOverride,
    ImplicitInstanceBinding,
    GlobalBinding,
    InstanceOverridesGlobal,
    UnusedInstanceBinding,
    UndefinedBinding,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceSinExpressionRepresentation {
    IndependentSin,
    BehavioralSpiceSin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceParamExpressionRepresentation {
    ParameterCoefficient,
    LiteralCoefficient,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceParams1Representation {
    LiteralValues,
    GlobalParameters,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceNakedAlgebraRepresentation {
    BracedLocalBaseline,
    MixedLocalParameters,
    MixedGlobalParameters,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceBug1826ThermalParameterRepresentation {
    GlobalParameter,
    LocalParameter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XycePassivePrimaryKind {
    CapacitorTran,
    ResistorDc,
}

impl XycePassivePrimaryKind {
    fn primary_parameter(self) -> &'static str {
        match self {
            Self::CapacitorTran => "C",
            Self::ResistorDc => "R",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XycePassivePrimaryRepresentation {
    Named,
    Positional,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XycePassiveTemperatureDeviceKind {
    Capacitor,
    Inductor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XycePassiveTemperatureRepresentation {
    Model,
    ScalarInstance,
    VectorInstance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceTransientAnalysisRepresentation {
    DirectNumeric,
    ParameterExpression,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceDcAnalysisRepresentation {
    DirectNumeric,
    ParameterExpression,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceDelimitedExpressionRepresentation {
    Braced,
    SingleQuoted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceAcAnalysisRepresentation {
    DirectNumeric,
    ParameterExpression,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct XyceRelationalElementFingerprint {
    kind: String,
    nodes: Vec<String>,
    numeric_bits: Vec<u64>,
    text: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceBjtExternalNodeFamilySnapshot {
    title: String,
    elements: BTreeMap<String, XyceRelationalElementFingerprint>,
    bjt_model_bits: BTreeMap<String, u64>,
    representation: XyceBjtExternalNodeRepresentation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceVbicDcFamilyPlanSnapshot {
    sweep_source: String,
    sweep_start_bits: u64,
    sweep_stop_bits: u64,
    sweep_step_bits: u64,
    probes: Vec<String>,
    steps: Vec<XyceVbicDcStepSnapshot>,
    subcircuit_ports: usize,
    subcircuit_bjt_nodes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XyceVbicDcStepSnapshot {
    target: StepTarget,
    name: String,
    param_name: Option<String>,
    values_bits: Vec<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceBjtExternalNodeRepresentation {
    OmittedGround,
    ExplicitGround,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceBaselineFamilyKind {
    AbmFrequency,
    AbmLookupOrder,
    Bug1043AcDataParameters,
    Bug1085UserFunctionI0,
    AgeCap,
    DiodeModelAlias,
    SwitchStateCase,
    AcAnalysisExpression,
    BjtExternalNode,
    DcAnalysisExpression,
    DelimitedExpression,
    SinExpression,
    ParamExpression,
    Params1,
    NakedAlgebra,
    Bug1826ThermalParameter,
    SourceMultiplicity,
    PassiveCapPrimaryValue,
    PassiveResPrimaryValue,
    PassiveTemperatureOverride,
    TransientAnalysisExpression,
    Subckt,
    Supernode,
    ScopedModel,
    SubcktParameterPrecedence,
    SubcktParameterResolution,
    NestedIncludeIdentity,
    Bug38SubcktFormalParentheses,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceBaselineFamilyAnalysis {
    Ac,
    Dc,
    Tran,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum XyceBaselineFamilyComparison {
    AcComparator(XyceAcComparatorTolerance),
    Release710FileCompare(XyceFileCompareTolerance),
    Toleranced,
    TolerancedStrict,
    Exact,
    ExactPrn,
    ExactPrnCaseInsensitive,
}

impl XyceBaselineFamilyComparison {
    fn strict_qualification(self) -> bool {
        !matches!(self, Self::Toleranced)
    }

    fn requires_semantic_snapshot(self) -> bool {
        !matches!(self, Self::Toleranced)
    }

    fn requires_exact_plan_equivalence(self) -> bool {
        !matches!(self, Self::Toleranced)
    }

    fn compares_waveforms_exactly(self) -> bool {
        matches!(
            self,
            Self::Exact | Self::ExactPrn | Self::ExactPrnCaseInsensitive
        )
    }

    fn ac_comparator_tolerance(self) -> Option<XyceAcComparatorTolerance> {
        match self {
            Self::AcComparator(tolerance) => Some(tolerance),
            _ => None,
        }
    }

    fn compares_serialized_prn_exactly(self) -> bool {
        matches!(self, Self::ExactPrn | Self::ExactPrnCaseInsensitive)
    }

    fn compares_prn_case_insensitively(self) -> bool {
        matches!(self, Self::ExactPrnCaseInsensitive)
    }

    fn uses_xyce_verify_transient_oracle(self) -> bool {
        matches!(self, Self::TolerancedStrict)
    }

    fn release_710_file_compare_tolerance(self) -> Option<XyceFileCompareTolerance> {
        match self {
            Self::Release710FileCompare(tolerance) => Some(tolerance),
            _ => None,
        }
    }

    fn permits_locked_time_retry(self) -> bool {
        matches!(self, Self::Toleranced)
    }
}

impl XyceBaselineFamilyKind {
    fn name(self) -> &'static str {
        match self {
            Self::AbmFrequency => "ABM_FREQUENCY_RELATIONAL",
            Self::AbmLookupOrder => "ABM_SPLINES_INLINE_LOOKUP_ORDER",
            Self::Bug1043AcDataParameters => "BUG1043_AC_DATA_PARAMETERS",
            Self::Bug1085UserFunctionI0 => "BUG1085_USER_FUNCTION_I0_ALPHA_EQUIVALENCE",
            Self::AgeCap => "AGE_CAP_EQUIVALENCE",
            Self::DiodeModelAlias => "DIODE_MODEL_ALIAS_EQUIVALENCE",
            Self::SwitchStateCase => "SWITCH_STATE_CASE_EQUIVALENCE",
            Self::AcAnalysisExpression => "AC_ANALYSIS_EXPRESSION",
            Self::BjtExternalNode => "BJT_EXTNODE",
            Self::DcAnalysisExpression => "DC_ANALYSIS_EXPRESSION",
            Self::DelimitedExpression => "DELIMITED_EXPRESSION",
            Self::SinExpression => "SIN_EXPRESSION",
            Self::ParamExpression => "PARAM_EXPRESSION",
            Self::Params1 => "PARAMS1_PARAMETER_EQUIVALENCE",
            Self::NakedAlgebra => "NAKED_ALGEBRA_PARAMETER_EQUIVALENCE",
            Self::Bug1826ThermalParameter => "BUG1826_THERMAL_PARAMETER_SCOPE_EQUIVALENCE",
            Self::SourceMultiplicity => "SOURCE_MULTIPLICITY_EQUIVALENCE",
            Self::PassiveCapPrimaryValue => "PASSIVE_CAP_PRIMARY_VALUE",
            Self::PassiveResPrimaryValue => "PASSIVE_RES_PRIMARY_VALUE",
            Self::PassiveTemperatureOverride => "PASSIVE_TEMPERATURE_OVERRIDE",
            Self::TransientAnalysisExpression => "TRANSIENT_ANALYSIS_EXPRESSION",
            Self::Subckt => "SUBCKT",
            Self::Supernode => "SUPERNODE",
            Self::ScopedModel => "SCOPED_MODEL",
            Self::SubcktParameterPrecedence => "SUBCKT_PARAMETER_PRECEDENCE",
            Self::SubcktParameterResolution => "SUBCKT_PARAMETER_RESOLUTION",
            Self::NestedIncludeIdentity => "NESTED_INCLUDE_IDENTITY",
            Self::Bug38SubcktFormalParentheses => "BUG38_SUBCKT_FORMAL_PARENTHESES",
        }
    }

    fn wrapper_contract(self) -> &'static str {
        match self {
            Self::AbmFrequency => XYCE_ABM_FREQUENCY_WRAPPER_OWNER_CONTRACT,
            Self::AbmLookupOrder => XYCE_ABM_LOOKUP_ORDER_WRAPPER_OWNER_CONTRACT,
            Self::Bug1043AcDataParameters => XYCE_BUG1043_AC_DATA_PARAMETER_WRAPPER_OWNER_CONTRACT,
            Self::Bug1085UserFunctionI0 => contracts_bug1085::XYCE_BUG1085_WRAPPER_OWNER_CONTRACT,
            Self::AgeCap => "age_cap_family_anchor",
            Self::DiodeModelAlias => "diode_model_alias_family_anchor",
            Self::SwitchStateCase => "switch_state_case_family_anchor",
            Self::AcAnalysisExpression => "ac_analysis_expression_family_wrapper",
            Self::BjtExternalNode => "bjt_external_node_family_wrapper",
            Self::DcAnalysisExpression => "dc_analysis_expression_family_wrapper",
            Self::DelimitedExpression => "delimited_expression_family_wrapper",
            Self::SinExpression => "sin_expression_family_wrapper",
            Self::ParamExpression => "param_expression_family_wrapper",
            Self::Params1 => XYCE_PARAMS1_WRAPPER_OWNER_CONTRACT,
            Self::NakedAlgebra => XYCE_NAKED_ALGEBRA_WRAPPER_OWNER_CONTRACT,
            Self::Bug1826ThermalParameter => XYCE_BUG1826_THERMAL_PARAMETER_WRAPPER_OWNER_CONTRACT,
            Self::SourceMultiplicity => XYCE_SOURCE_MULTIPLICITY_WRAPPER_CONTRACT,
            Self::PassiveCapPrimaryValue => "passive_primary_value_capacitor_tran_wrapper",
            Self::PassiveResPrimaryValue => "passive_primary_value_resistor_dc_wrapper",
            Self::PassiveTemperatureOverride => "passive_temperature_override_family_wrapper",
            Self::TransientAnalysisExpression => "transient_analysis_expression_family_wrapper",
            Self::Subckt => "subckt_family_wrapper",
            Self::Supernode => "supernode_family_wrapper",
            Self::ScopedModel => "scoped_model_family_wrapper",
            Self::SubcktParameterPrecedence => "subckt_parameter_precedence_wrapper",
            Self::SubcktParameterResolution => "subckt_parameter_resolution_family_wrapper",
            Self::NestedIncludeIdentity => "nested_include_identity_family_anchor",
            Self::Bug38SubcktFormalParentheses => XYCE_BUG38_WRAPPER_OWNER_CONTRACT,
        }
    }

    fn baseline_contract(self) -> &'static str {
        match self {
            Self::AbmFrequency => XYCE_ABM_FREQUENCY_DATA_CONTROL_CONTRACT,
            Self::AbmLookupOrder => XYCE_ABM_LOOKUP_ORDER_SORTED_CONTROL_CONTRACT,
            Self::Bug1043AcDataParameters => {
                XYCE_BUG1043_AC_DATA_PARAMETER_EXPRESSION_BASELINE_CONTRACT
            }
            Self::Bug1085UserFunctionI0 => {
                contracts_bug1085::XYCE_BUG1085_REFERENCE_BASELINE_CONTRACT
            }
            Self::AgeCap => "age_cap_family_aged_baseline",
            Self::DiodeModelAlias => "diode_model_alias_family_canonical_baseline",
            Self::SwitchStateCase => "switch_state_case_family_uppercase_baseline",
            Self::AcAnalysisExpression => "ac_analysis_expression_family_baseline",
            Self::BjtExternalNode => "bjt_external_node_family_baseline",
            Self::DcAnalysisExpression => "dc_analysis_expression_family_baseline",
            Self::DelimitedExpression => "delimited_expression_family_baseline",
            Self::SinExpression => "sin_expression_family_baseline",
            Self::ParamExpression => "param_expression_family_baseline",
            Self::Params1 => XYCE_PARAMS1_LITERAL_BASELINE_CONTRACT,
            Self::NakedAlgebra => XYCE_NAKED_ALGEBRA_BRACED_BASELINE_CONTRACT,
            Self::Bug1826ThermalParameter => {
                XYCE_BUG1826_THERMAL_PARAMETER_GLOBAL_BASELINE_CONTRACT
            }
            Self::SourceMultiplicity => XYCE_SOURCE_MULTIPLICITY_BASELINE_CONTRACT,
            Self::PassiveCapPrimaryValue => "passive_primary_value_capacitor_tran_baseline",
            Self::PassiveResPrimaryValue => "passive_primary_value_resistor_dc_baseline",
            Self::PassiveTemperatureOverride => "passive_temperature_override_family_baseline",
            Self::TransientAnalysisExpression => "transient_analysis_expression_family_baseline",
            Self::Subckt => "subckt_family_baseline",
            Self::Supernode => "supernode_family_baseline",
            Self::ScopedModel => "scoped_model_family_baseline",
            Self::SubcktParameterPrecedence => "subckt_parameter_precedence_baseline",
            Self::SubcktParameterResolution => "subckt_parameter_resolution_family_baseline",
            Self::NestedIncludeIdentity => {
                "nested_include_identity_family_repeated_target_baseline"
            }
            Self::Bug38SubcktFormalParentheses => XYCE_BUG38_PARENTHESIZED_CONTROL_CONTRACT,
        }
    }

    fn compares_baseline_oracle(self) -> bool {
        matches!(self, Self::Supernode)
    }

    fn compares_transient_baseline_oracle(self) -> bool {
        matches!(self, Self::PassiveTemperatureOverride)
    }

    fn xyce_verify_member_is_good_waveform(self) -> bool {
        // Release 7.10's nakedAlgebra wrapper invokes xyce_verify with each
        // mixed-expression member as GOODFILE and the braced baseline as
        // TESTFILE. The normalized RMS denominator is directional, so this
        // ordering is part of the oracle rather than an interchangeable pair.
        matches!(
            self,
            Self::AbmLookupOrder | Self::NakedAlgebra | Self::SourceMultiplicity
        )
    }

    /// Whether ACComparator's non-baseline member is the directional
    /// GOODFILE.  ABM_FREQ's Release 7.10 wrappers invoke the comparator as
    /// `owner.FD.prn data-control.FD.prn`; the relative denominator, zero
    /// handling, and frequency clauses make this ordering observable.
    fn ac_comparator_member_is_good_waveform(self) -> bool {
        matches!(self, Self::AbmFrequency)
    }

    fn transient_plan_purpose(self) -> XyceStaticTranPlanPurpose {
        match self {
            Self::ScopedModel => XyceStaticTranPlanPurpose::ScopedModelRelationalFamily,
            Self::AgeCap => XyceStaticTranPlanPurpose::AgeCapRelationalFamily,
            Self::AbmFrequency
            | Self::AbmLookupOrder
            | Self::Bug1043AcDataParameters
            | Self::Bug1085UserFunctionI0
            | Self::AcAnalysisExpression
            | Self::BjtExternalNode
            | Self::DcAnalysisExpression
            | Self::DelimitedExpression
            | Self::DiodeModelAlias
            | Self::SinExpression
            | Self::ParamExpression
            | Self::Params1
            | Self::NakedAlgebra
            | Self::Bug1826ThermalParameter
            | Self::SourceMultiplicity
            | Self::PassiveCapPrimaryValue
            | Self::PassiveTemperatureOverride
            | Self::TransientAnalysisExpression
            | Self::PassiveResPrimaryValue
            | Self::Subckt
            | Self::Supernode
            | Self::SubcktParameterPrecedence
            | Self::SubcktParameterResolution => XyceStaticTranPlanPurpose::RelationalFamily,
            Self::NestedIncludeIdentity => XyceStaticTranPlanPurpose::RelationalFamily,
            Self::SwitchStateCase => XyceStaticTranPlanPurpose::RelationalFamily,
            Self::Bug38SubcktFormalParentheses => XyceStaticTranPlanPurpose::RelationalFamily,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceStaticDcContract {
    PlainStatic,
    SharedStepOracle,
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
    WrapperTopLevelExecutionDirWorker,
    WrapperVoltageAccessor,
}

impl XyceStaticDcContract {
    fn result_contract(self, stepped: bool) -> &'static str {
        match (self, stepped) {
            (Self::PlainStatic, false) => "static_prn_dc",
            (Self::PlainStatic, true) => "static_prn_step_dc",
            (Self::SharedStepOracle, false) => "shared_step_oracle_prn_dc",
            (Self::SharedStepOracle, true) => "shared_step_oracle_prn_step_dc",
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
            (Self::WrapperTopLevelExecutionDirWorker, false) => {
                "wrapper_top_level_execution_dir_worker_prn_dc"
            }
            (Self::WrapperTopLevelExecutionDirWorker, true) => {
                "wrapper_top_level_execution_dir_worker_prn_step_dc"
            }
            (Self::WrapperVoltageAccessor, false) => "wrapper_voltage_accessor_prn_dc",
            (Self::WrapperVoltageAccessor, true) => "wrapper_voltage_accessor_prn_step_dc",
        }
    }

    fn compares_step_res_reference(self) -> bool {
        matches!(
            self,
            Self::SharedStepOracle | Self::WrapperDefault | Self::WrapperRaw
        )
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

/// How a deck's text is to be read: which expression dialect governs it and
/// what a repeated `.PARAM` means. Xyce's answer to all three is one decision
/// about the deck, and no caller sets one without setting the others.
#[derive(Clone, Copy)]
struct XyceParsePolicy {
    expression_dialect: ExpressionDialect,
    redefinition: ParameterRedefinitionPolicy,
    redefinition_diagnostic: rspice_core::netlist::ParameterRedefinitionDiagnosticPolicy,
}

/// One probe observation on the transient axis: when it was taken, what the
/// reference says, and what RSpice produced.
#[derive(Clone, Copy)]
struct XyceProbeSample {
    time: Value,
    expected: Value,
    actual: Value,
}

/// A passive device parameter being resolved for a probe: which device and
/// parameter, the literal or expression the deck gave, and the instance
/// overrides that may replace it.
#[derive(Clone, Copy)]
struct XycePassiveParameter<'a> {
    device_kind: &'a str,
    element_name: &'a str,
    parameter_name: &'a str,
    value: Value,
    value_expr: Option<&'a str>,
    instance_params: &'a [(String, Value)],
}

/// A stepped-transient comparison: the plan, the runs it produced, the
/// cancellation source, and whether the reference grid is being replayed.
struct XyceStepTranComparison<'a> {
    plan: &'a XyceStaticTranPlan,
    step_runs: &'a [XyceStepRun],
    abort: &'a dyn AbortSignal,
    locked_time_grid: bool,
}

/// Where a DC sensitivity reference lives and how to decode it.
#[derive(Clone, Copy)]
struct XyceSensitivityReference<'a> {
    path: &'a Path,
    format: XyceDcSensitivityReferenceFormat,
    side_file: Option<&'a str>,
}

/// Which `.STEP` row a comparison is on, when the run was stepped.
#[derive(Clone, Copy)]
struct XyceStepContext<'a> {
    row_netlists: Option<&'a [Netlist]>,
    expected_step_index: Option<usize>,
}

/// The measurement artifacts a comparison reads and the terms it holds them to.
#[derive(Clone, Copy)]
struct XyceMeasureArtifacts<'a> {
    paths: &'a [PathBuf],
    tolerance: XyceFileCompareTolerance,
    declarations: &'a [rspice_core::analysis::MeasureStatement],
}

/// Xyce's two knobs for how a failed `.MEASURE` is reported.
#[derive(Clone, Copy)]
struct XyceMeasurePolicy {
    fail_output: Option<bool>,
    default_value: Option<Value>,
}

/// A lossless transmission line as a deck can spell it: characteristic
/// impedance plus exactly one of a delay, or a frequency and its electrical
/// length.
#[derive(Clone, Copy)]
struct LosslessLineSpec<'a> {
    z0: Option<Value>,
    td: Option<Value>,
    freq: Option<Value>,
    nl: Option<Value>,
    model: Option<&'a str>,
}

/// One row of a reference table, addressed by its time column.
#[derive(Clone, Copy)]
struct XyceReferenceRow<'a> {
    table: &'a XycePrnTable,
    time_column: usize,
    row_index: usize,
}

/// A value tolerance plus the output-interval corridor it is applied over.
#[derive(Clone, Copy)]
struct XyceCorridorTolerance {
    value: XyceComparisonTolerance,
    output_interval: Value,
    time_scale_factor: Value,
}

/// A value tolerance plus the time-axis neighborhood it is searched over.
#[derive(Clone, Copy)]
struct XyceNeighborhoodTolerance {
    value: XyceComparisonTolerance,
    time_tolerance: Value,
    time_scale_factor: Value,
}

#[derive(Debug, Clone, Copy)]
struct XyceComparisonTolerance {
    relative: f64,
    absolute: f64,
    zero: Option<f64>,
}

/// Point-wise tolerances used by Xyce 7.10's `ACComparator.pl`.  The values
/// are stored as IEEE-754 bits so a relational family contract can carry an
/// exact, validated sidecar policy while remaining `Eq`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct XyceAcComparatorTolerance {
    absolute_bits: u64,
    relative_bits: u64,
    zero_bits: u64,
    frequency_relative_bits: u64,
}

impl XyceAcComparatorTolerance {
    fn new(
        absolute: Value,
        relative: Value,
        zero: Value,
        frequency_relative: Value,
    ) -> Result<Self, String> {
        if !absolute.is_finite()
            || !relative.is_finite()
            || !zero.is_finite()
            || !frequency_relative.is_finite()
            || absolute < 0.0
            || relative < 0.0
            || zero < 0.0
            || frequency_relative < 0.0
        {
            return Err(format!(
                "ACComparator tolerances must be finite and nonnegative, got abs={absolute}, rel={relative}, zero={zero}, freqrel={frequency_relative}"
            ));
        }
        Ok(Self {
            absolute_bits: absolute.to_bits(),
            relative_bits: relative.to_bits(),
            zero_bits: zero.to_bits(),
            frequency_relative_bits: frequency_relative.to_bits(),
        })
    }

    fn values(self) -> (Value, Value, Value, Value) {
        (
            Value::from_bits(self.absolute_bits),
            Value::from_bits(self.relative_bits),
            Value::from_bits(self.zero_bits),
            Value::from_bits(self.frequency_relative_bits),
        )
    }
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

/// Release 7.10 transient `xyce_verify` uses an integrated normalized RMS
/// contract. These fields intentionally remain separate from the runner's
/// pointwise [`XyceComparisonTolerance`], whose absolute and zero semantics
/// are different. Qualified native generated-oracle contracts require
/// positive RELTOL/ABSTOL rather than reproducing the Perl verifier's
/// degenerate zero-RELTOL final comparison.
#[derive(Debug, Clone, Copy, PartialEq)]
struct XyceVerifyTransientTolerance {
    relative: Value,
    absolute: Value,
    zero: Value,
    absolute_difference: Value,
    offset: Value,
}

impl XyceVerifyTransientTolerance {
    const fn release_7_10_default() -> Self {
        Self {
            relative: XYCE_VERIFY_DEFAULT_RELATIVE_TOLERANCE,
            absolute: XYCE_VERIFY_DEFAULT_ABSOLUTE_TOLERANCE,
            zero: XYCE_VERIFY_DEFAULT_ZERO_TOLERANCE,
            absolute_difference: XYCE_VERIFY_DEFAULT_ABSOLUTE_DIFFERENCE_TOLERANCE,
            offset: 0.0,
        }
    }

    fn has_nondefault_error_bounds(self) -> bool {
        let default = Self::release_7_10_default();
        self.relative.to_bits() != default.relative.to_bits()
            || self.absolute.to_bits() != default.absolute.to_bits()
            || self.zero.to_bits() != default.zero.to_bits()
            || self.absolute_difference.to_bits() != default.absolute_difference.to_bits()
    }

    fn validate(self) -> Result<Self, String> {
        if !self.relative.is_finite()
            || self.relative <= 0.0
            || !self.absolute.is_finite()
            || self.absolute <= 0.0
            || !self.zero.is_finite()
            || self.zero < 0.0
            || !self.absolute_difference.is_finite()
            || self.absolute_difference < 0.0
            || !self.offset.is_finite()
        {
            return Err(format!(
                "xyce_verify transient tolerances must have positive finite RELTOL/ABSTOL, nonnegative finite ZEROTOL/ABSDIFFTOL, and finite OFFSET, got {self:?}"
            ));
        }
        Ok(self)
    }
}

#[derive(Debug, Clone)]
struct XyceDcSweep {
    source: String,
    start: Value,
    stop: Value,
    step: Value,
    mode: rspice_core::netlist::DcSweepMode,
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
    compact_device_count: usize,
    node_count: usize,
}

#[derive(Debug, Clone)]
struct XyceAcAnalysis {
    frequencies: Vec<Value>,
    data_points: Option<Vec<XyceFrequencyDataPoint>>,
}

impl XyceAcAnalysis {
    fn frequencies(&self) -> Vec<Value> {
        self.frequencies.clone()
    }

    fn data_points(&self) -> Option<&[XyceFrequencyDataPoint]> {
        self.data_points.as_deref()
    }
}

#[derive(Debug, Clone)]
struct XyceFrequencyDataPoint {
    frequency: Value,
    overrides: Vec<(String, Value)>,
}

/// Xyce's `.AC DATA` analysis-initialization diagnostics when the referenced
/// table cannot be resolved.  These are semantic failures after parsing, not
/// parser errors, and therefore have a dedicated native oracle contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceAcDataAnalysisInitFailure {
    NoDataTables,
    UnknownTable,
}

impl XyceAcDataAnalysisInitFailure {
    fn result_contract(self) -> &'static str {
        match self {
            Self::NoDataTables => "expected_error_ac_data_analysis_init_no_data",
            Self::UnknownTable => "expected_error_ac_data_analysis_init_unknown_table",
        }
    }
}

#[derive(Debug, Clone)]
struct XyceNoiseAnalysis {
    output_node: String,
    reference_node: Option<String>,
    input_source: String,
    frequencies: Vec<Value>,
    data_points: Option<Vec<XyceFrequencyDataPoint>>,
    data_table_name: Option<String>,
}

impl XyceAcReferenceColumn {
    fn probe_name(&self) -> &str {
        match self {
            Self::Probe { name, .. } => name,
        }
    }

    fn component(&self) -> XyceAcProbeComponent {
        match self {
            Self::Probe { component, .. } => *component,
        }
    }
}

#[derive(Debug, Clone)]
struct XyceDcSweepDimension {
    source: String,
    start: Value,
    stop: Value,
    step: Value,
    mode: rspice_core::netlist::DcSweepMode,
}

impl XyceDcSweepDimension {
    fn spec(&self) -> rspice_core::netlist::DcSweepSpec {
        rspice_core::netlist::DcSweepSpec {
            start: self.start,
            stop: self.stop,
            step: self.step,
            mode: self.mode.clone(),
            restart: false,
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
    fn primary_spec(&self) -> rspice_core::netlist::DcSweepSpec {
        rspice_core::netlist::DcSweepSpec {
            start: self.start,
            stop: self.stop,
            step: self.step,
            mode: self.mode.clone(),
            restart: false,
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
struct XyceStepTranEvaluation {
    step_values: Vec<Value>,
    netlist: Netlist,
    transient: TransientResult,
    scalar: Vec<rspice_core::analysis::MeasureResult>,
    continuous: Vec<rspice_core::analysis::ContinuousMeasureResult>,
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

    fn op_parameter(self) -> &'static str {
        match self {
            Self::Drain => "id",
            Self::Gate => "ig",
            Self::Source => "is",
            Self::Bulk => "ib",
            Self::Collector => "ic",
            Self::Emitter => "ie",
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

#[derive(Debug)]
struct XyceTranRemeasureInput {
    time: Vec<Value>,
    signals: HashMap<String, Vec<Value>>,
}

impl XyceTranRemeasureInput {
    fn signal_slices(&self) -> HashMap<String, &[Value]> {
        self.signals
            .iter()
            .map(|(name, values)| (name.clone(), values.as_slice()))
            .collect()
    }
}

#[derive(Debug, Clone)]
struct XyceTecplotReference {
    table: XycePrnTable,
    zones: Vec<XyceTecplotZone>,
}

#[derive(Debug, Clone)]
struct XyceTecplotZone {
    title: String,
    auxdata: BTreeMap<String, XyceTecplotBinding>,
    row_start: usize,
    row_count: usize,
}

#[derive(Debug, Clone, Copy)]
struct XyceTecplotBinding {
    value: Value,
    quantization: Option<Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XycePrnDelimiter {
    Whitespace,
    Comma,
}

use super::deadline::DeadlineAbort;

mod analysis_support;
mod comparison;
mod contracts;
mod contracts_bug1025;
mod contracts_bug1035_son;
mod contracts_bug1040;
mod contracts_bug1043;
mod contracts_bug1085;
mod contracts_bug1116;
mod contracts_bug113;
mod contracts_bug1152;
mod contracts_bug1162;
mod contracts_bug1190_son;
mod contracts_bug1284;
mod contracts_bug1398;
mod contracts_bug141;
mod contracts_bug1455;
mod contracts_bug159;
mod contracts_bug1595;
mod contracts_bug1661;
mod contracts_bug1692;
mod contracts_bug1797;
mod contracts_bug1957;
mod contracts_bug206;
mod contracts_bug267;
mod contracts_bug271;
mod contracts_bug28;
mod contracts_bug302;
mod contracts_bug306_son;
mod contracts_bug307;
mod contracts_bug308_son;
mod contracts_bug325_son;
mod contracts_bug340;
mod contracts_bug352;
mod contracts_bug354;
mod contracts_bug372;
mod contracts_bug38;
mod contracts_bug389;
mod contracts_bug39;
mod contracts_bug402;
mod contracts_bug411;
mod contracts_bug412;
mod contracts_bug42_son;
mod contracts_bug440;
mod contracts_bug442;
mod contracts_bug45;
mod contracts_bug456;
mod contracts_bug48;
mod contracts_bug519;
mod contracts_bug636;
mod contracts_bug667_ic;
mod contracts_bug689;
mod contracts_bug706;
mod contracts_bug784;
mod contracts_bug805;
mod contracts_bug805_son;
mod contracts_bug806;
mod contracts_bug864;
mod contracts_bug907_son;
mod contracts_bug981;
mod contracts_bug986;
mod contracts_dc;
mod contracts_diode_analytic;
mod contracts_frequency;
mod contracts_issue202;
mod contracts_issue451;
mod contracts_issue565_566;
mod contracts_issue61;
mod contracts_legacy_device_analytic;
mod contracts_mosfet_param_aliases;
mod contracts_sources;
mod contracts_splines;
mod contracts_tia_passive_analytic;
mod contracts_tr_tran;
mod contracts_transient;
mod discovery;
mod execution;
mod expected_failures;
mod family_oracles;
mod family_snapshots;
mod netlist_support;
mod output;
mod reference;
mod support;

use contracts_bug667_ic::Bug667IcRole;
use contracts_tia_passive_analytic::XyceTiaPassiveAnalyticKind;

#[cfg(test)]
mod tests;
