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
#![allow(clippy::too_many_arguments)]

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
use std::time::{Duration, Instant};

const EXPECTED_UNSUPPORTED_MARKER: &str = "EXPECTED_UNSUPPORTED:";
const UPSTREAM_EXCLUDED_MARKER: &str = "UPSTREAM_EXCLUDED:";
const HARNESS_MANIFEST_FILE: &str = "RSPICE-HARNESS-MANIFEST.tsv";
const UPSTREAM_EXCLUSIONS_MANIFEST_FILE: &str = "RSPICE-UPSTREAM-EXCLUSIONS.tsv";
const UPSTREAM_EXCLUSIONS_SCHEMA_VERSION: &str = "1";
const UPSTREAM_EXCLUSIONS_SOURCE_COMMIT: &str = "80115a9277c0ddb3409acceb3d4e745fd11cddd4";
const UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE: &str = "3e34bfaafa890cb2e4457137b6a0e325c8c1e87d";
const UPSTREAM_EXCLUSIONS_RETAINED_DECK_COUNT: usize = 1_143;
const UPSTREAM_EXCLUSIONS_QUALIFIED_DECK_COUNT: usize = 279;
const UPSTREAM_EXCLUSIONS_RETAINED_PATHS_SHA256: &str =
    "eb3eb203f0974a430cdea3924e921aecdc1f71c5c9ce4de2f78f282c57291997";
const UPSTREAM_EXCLUSIONS_PROMOTIONS_SHA256: &str =
    "da3dad9eaf71f0bf0bbf4d954c23730b9d2df1726e94e6cfc451dde495b82349";
const UPSTREAM_EXCLUSIONS_RECORDS_SHA256: &str =
    "ce0fbdbe351388129dc2f752ce2fd7575d503be77850ec7ce2959d5f838971a2";
const UPSTREAM_EXCLUSIONS_MANIFEST_SHA256: &str =
    "96d2a002a2dc5ef07515d11fcb70fb5865918033817375dc47f0dd4c7f8ef2b9";
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
const XYCE_NONLINEAR_CORE_MODEL_STEP_WRAPPER_CONTRACT: &str =
    "nonlinear_core_model_step_reference_wrapper";
const XYCE_NONLINEAR_CORE_MODEL_STEP_BASELINE_CONTRACT: &str =
    "nonlinear_core_model_step_reference_baseline";
const XYCE_NONLINEAR_CORE_MODEL_STEP_CANDIDATE_COUNT: usize = 24;
const XYCE_NONLINEAR_CORE_MODEL_STEP_CANDIDATE_BLAKE3: &str =
    "a514fabac8dd61a804b137aa836f2592210bfadac040101e1d8463a93e878b11";
const XYCE_NONLINEAR_CORE_MODEL_STEP_CANDIDATE_CONTENT_BLAKE3: &str =
    "dfd1ff2e37507c06ddfe8d2df7d5a313eae667b728d698e947aea992a177aa9b";
const XYCE_NONLINEAR_CORE_MODEL_STEP_OWNER_COUNT: usize = 6;
const XYCE_NONLINEAR_CORE_MODEL_STEP_OWNER_MANIFEST_BLAKE3: &str =
    "a015cf6b2395c9a5c62224b832bff244822c419b1cb199293f1e961bbd43e381";
const XYCE_NONLINEAR_CORE_MODEL_STEP_EXCLUSION_COUNT: usize = 18;
const XYCE_NONLINEAR_CORE_MODEL_STEP_HISTORICAL_EXCLUSION_BLAKE3: &str =
    "a38a113c8f72fe3bb98a863de111674d446019c6fd0950d29f6f93da9b8ac65e";
const XYCE_BUG1190_MUTUAL_INDUCTOR_WRAPPER_CONTRACT: &str =
    "bug1190_mutual_inductor_parameter_alias_wrapper";
const XYCE_BUG1190_MUTUAL_INDUCTOR_BASELINE_CONTRACT: &str =
    "bug1190_mutual_inductor_parameter_alias_baseline";
const XYCE_BUG1190_MUTUAL_INDUCTOR_CANDIDATE_COUNT: usize = 4;
const XYCE_BUG1190_MUTUAL_INDUCTOR_CANDIDATE_BLAKE3: &str =
    "c9e772e38bb06d933f7fa8d47fe58fee6d9dc885534b5c12a9d7edd1b04495f2";
const XYCE_BUG1190_MUTUAL_INDUCTOR_CONTENT_BLAKE3: &str =
    "df40a7890f29033c29c938f59dc77a4b7258b1a349a56acea131e920afc60d77";
const XYCE_BUG1190_MUTUAL_INDUCTOR_OWNER_COUNT: usize = 2;
const XYCE_BUG1190_MUTUAL_INDUCTOR_OWNER_MANIFEST_BLAKE3: &str =
    "e7c00713e950019d0b4144fc003a7ffeef38c7a5e3f432117b8f00c2c607263f";
const XYCE_BUG1190_MUTUAL_INDUCTOR_EXCLUSION_COUNT: usize = 2;
const XYCE_BUG1190_MUTUAL_INDUCTOR_HISTORICAL_EXCLUSION_BLAKE3: &str =
    "6d2ca1af02efba66823b08c1597c07f8b7c45dbae4346076343d3245240f0d33";
const XYCE_CLASSIC_MOS_DTEMP_WRAPPER_CONTRACT: &str =
    "classic_mos_level1_dtemp_relational_wrapper_owner";
const XYCE_CLASSIC_MOS_DTEMP_REFERENCE_CONTRACT: &str =
    "classic_mos_level1_dtemp_relational_wrapper_reference";
const XYCE_CLASSIC_MOS_DTEMP_CANDIDATE_COUNT: usize = 16;
const XYCE_CLASSIC_MOS_DTEMP_CANDIDATE_BLAKE3: &str =
    "3d6d5a6e298f6e768fe45180e038e115bf341ce1b351351b406a714da6927f50";
const XYCE_CLASSIC_MOS_DTEMP_CONTENT_BLAKE3: &str =
    "ece370c9a2cf3e650ccadd819f6e08262f340ee44c83ffebc1a704e302104b06";
const XYCE_CLASSIC_MOS_DTEMP_OWNER_COUNT: usize = 8;
const XYCE_CLASSIC_MOS_DTEMP_OWNER_MANIFEST_BLAKE3: &str =
    "1519db656b566549d712c797adf62cd8f3a128b9c2189aba3830956e0ce0f345";
const XYCE_CLASSIC_MOS_DTEMP_EXCLUSION_COUNT: usize = 8;
const XYCE_CLASSIC_MOS_DTEMP_HISTORICAL_EXCLUSION_BLAKE3: &str =
    "1150bc1fda0dd8db1f2091b15b5c280ae881ebdbd8aad31df4b0ec491a75e3fa";
const XYCE_LEGACY_BJT_DTEMP_WRAPPER_CONTRACT: &str =
    "legacy_gummel_poon_bjt_dtemp_relational_wrapper_owner";
const XYCE_LEGACY_BJT_DTEMP_REFERENCE_CONTRACT: &str =
    "legacy_gummel_poon_bjt_dtemp_relational_wrapper_reference";
const XYCE_LEGACY_BJT_DTEMP_NPN_OWNER_RECORD: &str = "netlists/dtemp/npn_dtemp.cir";
const XYCE_LEGACY_BJT_DTEMP_NPN_REFERENCE_RECORD: &str = "netlists/dtemp/npn_ref.cir";
const XYCE_LEGACY_BJT_DTEMP_PNP_OWNER_RECORD: &str = "netlists/dtemp/pnp_dtemp.cir";
const XYCE_LEGACY_BJT_DTEMP_PNP_REFERENCE_RECORD: &str = "netlists/dtemp/pnp_ref.cir";
const XYCE_LEGACY_BJT_DTEMP_CANDIDATE_COUNT: usize = 4;
const XYCE_LEGACY_BJT_DTEMP_CANDIDATE_BLAKE3: &str =
    "57301c7c863570abfbce995121166c364792cf9b47d95275b5155eed84e9819c";
const XYCE_LEGACY_BJT_DTEMP_CONTENT_BLAKE3: &str =
    "0469c5b96dc5ec35562ac6e84422c5c810205f2a70c037a55298ffae84ab3e94";
const XYCE_LEGACY_BJT_DTEMP_OWNER_COUNT: usize = 2;
const XYCE_LEGACY_BJT_DTEMP_OWNER_MANIFEST_BLAKE3: &str =
    "8c415000b5df6ebd473ae1828b92ab03fb61fb175357bc44e8c62dbcdca4e512";
const XYCE_LEGACY_BJT_DTEMP_EXCLUSION_COUNT: usize = 2;
const XYCE_LEGACY_BJT_DTEMP_HISTORICAL_EXCLUSION_BLAKE3: &str =
    "69ef9ba2bf3cf5c275c546f5cea47a684ee90ca07f72926c2b8542997ac10e78";
const XYCE_SYDNEY_LEVEL1_JFET_DTEMP_WRAPPER_CONTRACT: &str =
    "xyce_sydney_level1_jfet_dtemp_relational_wrapper_owner";
const XYCE_SYDNEY_LEVEL1_JFET_DTEMP_REFERENCE_CONTRACT: &str =
    "xyce_sydney_level1_jfet_dtemp_relational_wrapper_reference";
const XYCE_SYDNEY_LEVEL1_JFET_DTEMP_NJF_OWNER_RECORD: &str = "netlists/dtemp/njfet_dtemp.cir";
const XYCE_SYDNEY_LEVEL1_JFET_DTEMP_NJF_REFERENCE_RECORD: &str = "netlists/dtemp/njfet_ref.cir";
const XYCE_SYDNEY_LEVEL1_JFET_DTEMP_PJF_OWNER_RECORD: &str = "netlists/dtemp/pjfet_dtemp.cir";
const XYCE_SYDNEY_LEVEL1_JFET_DTEMP_PJF_REFERENCE_RECORD: &str = "netlists/dtemp/pjfet_ref.cir";
const XYCE_SYDNEY_LEVEL1_JFET_DTEMP_CANDIDATE_COUNT: usize = 4;
const XYCE_SYDNEY_LEVEL1_JFET_DTEMP_CANDIDATE_BLAKE3: &str =
    "bf6231fca25d849be8da71e8a6cf8325f6acd7943e0d5682830316992d1df1dc";
const XYCE_SYDNEY_LEVEL1_JFET_DTEMP_CONTENT_BLAKE3: &str =
    "863a33c867f670cd527fe448b0e39d60e338d0a53c6a86cf7401605955e90360";
const XYCE_SYDNEY_LEVEL1_JFET_DTEMP_OWNER_COUNT: usize = 2;
const XYCE_SYDNEY_LEVEL1_JFET_DTEMP_OWNER_MANIFEST_BLAKE3: &str =
    "fd62e0ecd815ffec85dea49ebace91df14654e4ab289fbe81d0b8c230f85b462";
const XYCE_SYDNEY_LEVEL1_JFET_DTEMP_EXCLUSION_COUNT: usize = 2;
const XYCE_SYDNEY_LEVEL1_JFET_DTEMP_HISTORICAL_EXCLUSION_BLAKE3: &str =
    "cab5d8a6b0743784cdc19c143d5068ccee23569733ec7ae6c61eb3f51971dfa4";
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
const XYCE_PARAMS1_OWNER_CONTENT_BLAKE3: &str =
    "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262";
const XYCE_PARAMS1_LITERAL_BASELINE_CONTENT_BLAKE3: &str =
    "c8c470a5f69c608d8a1b6dfceb6f74cd000e5f10279c16f7361ac10e3a618e04";
const XYCE_PARAMS1_PARAMETERIZED_MEMBER_CONTENT_BLAKE3: &str =
    "ff7f4422bab651a252c2a1f26d85de3f5dcb2283c80abaaa409d6b7d6bfcf27d";
const XYCE_PARAMS1_CANDIDATE_COUNT: usize = 3;
const XYCE_PARAMS1_CANDIDATE_BLAKE3: &str =
    "906ea8fd3d5f23664e117e5cc94db1764bd2c0f9428fe32d29f584ab7434c7de";
const XYCE_PARAMS1_CANDIDATE_CONTENT_BLAKE3: &str =
    "6f736f2fba7684dc732344f3a11febf3ce2565db27d4ae701ce58d20887ccc92";
const XYCE_PARAMS1_OWNER_COUNT: usize = 1;
const XYCE_PARAMS1_OWNER_MANIFEST_BLAKE3: &str =
    "8d99d169acdd490904026dab9fb0c4567b2bbec6b997d23fc3779894406ae084";
const XYCE_PARAMS1_EXCLUSION_COUNT: usize = 2;
const XYCE_PARAMS1_HISTORICAL_EXCLUSION_BLAKE3: &str =
    "bd29860b038fefd235030eafe83743f181dc811345fa62f6066f6c103fe25fea";
const XYCE_PARAMS1_UPSTREAM_REGRESSION_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const XYCE_PARAMS1_UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const XYCE_PARAMS1_HISTORICAL_WRAPPER_PATH: &str = "Netlists/PARAMS1/params_a.cir.sh";
const XYCE_PARAMS1_HISTORICAL_WRAPPER_BYTES: usize = 2_131;
const XYCE_PARAMS1_HISTORICAL_WRAPPER_SHA256: &str =
    "3e0d4db14de269d0b717f2ebdca11497f7e55c5c945c7f12765a96b87d4a8d56";
const XYCE_PARAMS1_HISTORICAL_WRAPPER_BLAKE3: &str =
    "a4322707a6b38e60db47b6b2524b69452846562e44e98531d5932bb401de057b";
const XYCE_RELEASE_710_XYCE_VERIFY_PATH: &str = "TestScripts/xyce_verify.pl";
const XYCE_RELEASE_710_XYCE_VERIFY_BYTES: usize = 59_566;
const XYCE_RELEASE_710_XYCE_VERIFY_SHA256: &str =
    "6e5f84b1646b30d0e12879848d7653584b39472d640a14916ae8fda6e1df12b3";
const XYCE_RELEASE_710_XYCE_VERIFY_BLAKE3: &str =
    "5eadb6dab06ed3091ea114146bd4a574de83784f87be9843ad7b721b0a793665";
const XYCE_PARAMS1_HISTORICAL_ORACLE_RECORD_COUNT: usize = 2;
const XYCE_PARAMS1_HISTORICAL_ORACLE_BLAKE3: &str =
    "b085c4d44ae81be36abe39da9b70c41a32411d8f3671bd2b9a47353a64a44744";
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
const XYCE_NAKED_ALGEBRA_CANDIDATE_COUNT: usize = 3;
const XYCE_NAKED_ALGEBRA_CANDIDATE_BLAKE3: &str =
    "5c299428d945e9d2d617d6cb803982b9edd78d532d0f1a7a3887fed061148cc6";
const XYCE_NAKED_ALGEBRA_CANDIDATE_CONTENT_BLAKE3: &str =
    "cc328d723a5681e6b677a558870cd506d65fd0b3b0720ec3932a402c89403206";
const XYCE_NAKED_ALGEBRA_OWNER_COUNT: usize = 1;
const XYCE_NAKED_ALGEBRA_OWNER_MANIFEST_BLAKE3: &str =
    "f585384fb812bdb6dcdc8fe5468ab75a66d00cb9cc947ee605408856c5cf28cd";
const XYCE_NAKED_ALGEBRA_EXCLUSION_COUNT: usize = 2;
const XYCE_NAKED_ALGEBRA_HISTORICAL_EXCLUSION_BLAKE3: &str =
    "a217cc15c973d4ac5654aa1a20d314cf429a29e7397d2e787220d23bb61c88e9";
const XYCE_NAKED_ALGEBRA_UPSTREAM_REGRESSION_COMMIT: &str =
    "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const XYCE_NAKED_ALGEBRA_UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const XYCE_NAKED_ALGEBRA_HISTORICAL_WRAPPER_PATH: &str = "Netlists/PARSER/nakedAlgebra.cir.sh";
const XYCE_NAKED_ALGEBRA_HISTORICAL_WRAPPER_BYTES: usize = 2_026;
const XYCE_NAKED_ALGEBRA_HISTORICAL_WRAPPER_SHA256: &str =
    "938f88ab56ab023d93a0cde882cb98391320d769087666b61bc7a4ed212b3e30";
const XYCE_NAKED_ALGEBRA_HISTORICAL_WRAPPER_BLAKE3: &str =
    "70e79ac1632a283e8af23bb303da9797ef1974f4adbd7af0948378863810dfe0";
const XYCE_NAKED_ALGEBRA_HISTORICAL_ORACLE_RECORD_COUNT: usize = 2;
const XYCE_NAKED_ALGEBRA_HISTORICAL_ORACLE_BLAKE3: &str =
    "0c56df6d66baca7083b8ef80ab6db52cc82d92c1087b8aff9e5eb8cad6c956c3";
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
const XYCE_BUG1826_THERMAL_PARAMETER_SUPPORT_RECORD: &str =
    "netlists/certification_tests/bug_1826/copper.linear";
const XYCE_BUG1826_THERMAL_PARAMETER_OWNER_CONTENT_BLAKE3: &str =
    "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262";
const XYCE_BUG1826_THERMAL_PARAMETER_GLOBAL_BASELINE_CONTENT_BLAKE3: &str =
    "0498a73d52c4e66391b4920baf96c0838f08aa007c33fa5573be616609beca0a";
const XYCE_BUG1826_THERMAL_PARAMETER_LOCAL_MEMBER_CONTENT_BLAKE3: &str =
    "fcc455267359d3ab906fe0aee58f4967e46c867a6fbef4f0420a6667f348ca45";
const XYCE_BUG1826_THERMAL_PARAMETER_SUPPORT_CONTENT_BLAKE3: &str =
    "4754b245583f213148103d11e3b76484916fe57bf63d18c307a381d708741e3d";
const XYCE_BUG1826_THERMAL_PARAMETER_CANDIDATE_COUNT: usize = 3;
const XYCE_BUG1826_THERMAL_PARAMETER_CANDIDATE_BLAKE3: &str =
    "2f37d60ff33cb0f8516e44d24cef01316847b48e8d5e99fa73354f4638be1293";
const XYCE_BUG1826_THERMAL_PARAMETER_CANDIDATE_CONTENT_BLAKE3: &str =
    "c297b34e5e4611fde354bbd41b019d59eda9fef8ae01d03d8a1cd1eee3b04f1d";
const XYCE_BUG1826_THERMAL_PARAMETER_OWNER_COUNT: usize = 1;
const XYCE_BUG1826_THERMAL_PARAMETER_OWNER_MANIFEST_BLAKE3: &str =
    "954fb7ec9f57f9bf266813a7a85592c2b19548b54652e1126b8a58c89be59ffd";
const XYCE_BUG1826_THERMAL_PARAMETER_EXCLUSION_COUNT: usize = 2;
const XYCE_BUG1826_THERMAL_PARAMETER_HISTORICAL_EXCLUSION_BLAKE3: &str =
    "db80e2386563f743b021b1c1400d050254e5644de87361b2b41f327a0324e3f7";
const XYCE_BUG1826_THERMAL_PARAMETER_UPSTREAM_REGRESSION_COMMIT: &str =
    "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const XYCE_BUG1826_THERMAL_PARAMETER_UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const XYCE_BUG1826_THERMAL_PARAMETER_HISTORICAL_WRAPPER_PATH: &str =
    "Netlists/Certification_Tests/BUG_1826/linear_simple.cir.sh";
const XYCE_BUG1826_THERMAL_PARAMETER_HISTORICAL_WRAPPER_BYTES: usize = 1_994;
const XYCE_BUG1826_THERMAL_PARAMETER_HISTORICAL_WRAPPER_SHA256: &str =
    "155f0ba65fc5c1ab5e750f36b6905a54dc50eac250acfb2813ae853063a9e860";
const XYCE_BUG1826_THERMAL_PARAMETER_HISTORICAL_WRAPPER_BLAKE3: &str =
    "e9f1460c2a494dc9998dcfa1c07b7a24cefad947c779ace0377f8ee24f8a541b";
const XYCE_BUG1826_THERMAL_PARAMETER_HISTORICAL_EXCLUDE_PATH: &str =
    "Netlists/Certification_Tests/BUG_1826/exclude";
const XYCE_BUG1826_THERMAL_PARAMETER_HISTORICAL_EXCLUDE_BYTES: usize = 49;
const XYCE_BUG1826_THERMAL_PARAMETER_HISTORICAL_EXCLUDE_SHA256: &str =
    "f65dcf286bb349dc23ba25b0f5a7ad70c1ab2a1c3f209d9d73b5dec740f73d37";
const XYCE_BUG1826_THERMAL_PARAMETER_HISTORICAL_EXCLUDE_BLAKE3: &str =
    "84f595a0c23dbf23318e472473347fec6f36f1c5fc535f87b2e36c0f95390397";
const XYCE_BUG1826_THERMAL_PARAMETER_HISTORICAL_ORACLE_RECORD_COUNT: usize = 3;
const XYCE_BUG1826_THERMAL_PARAMETER_HISTORICAL_ORACLE_BLAKE3: &str =
    "7ef9daf7fa72a71ca5981eff8c863aba6e8a87873b2f390f9708fbc0fe41303c";
// Release 7.10 qualified these current-source multiplier decks by running the
// authored M= owner as GOODFILE and its explicit 0.2-Siemens control as
// TESTFILE.  Keep the complete selected corpus, removed wrapper/exclude
// artifacts, and default xyce_verify implementation cryptographically bound;
// the BSRC and VCCS directories also contain unrelated native regressions and
// therefore deliberately use an exact-record selector rather than a physical
// whole-directory census.
const XYCE_SOURCE_MULTIPLICITY_WRAPPER_CONTRACT: &str = "source_multiplicity_family_wrapper";
const XYCE_SOURCE_MULTIPLICITY_BASELINE_CONTRACT: &str = "source_multiplicity_family_baseline";
const XYCE_SOURCE_MULTIPLICITY_CANDIDATE_COUNT: usize = 20;
const XYCE_SOURCE_MULTIPLICITY_CANDIDATE_BLAKE3: &str =
    "d4310f2dddfffbfaa2080d48de6cef07d61098c6b17b68bd22f88cbe40286e1d";
const XYCE_SOURCE_MULTIPLICITY_CANDIDATE_CONTENT_BLAKE3: &str =
    "1c234afda4454cc64bd9123b425e48c29244d39c9e7cc1f7397442844078c232";
const XYCE_SOURCE_MULTIPLICITY_OWNER_COUNT: usize = 10;
const XYCE_SOURCE_MULTIPLICITY_OWNER_MANIFEST_BLAKE3: &str =
    "a775dba28575547cda2cebd65a10de0725143ba027dc602dba92da6914cc6590";
const XYCE_SOURCE_MULTIPLICITY_EXCLUSION_COUNT: usize = 10;
const XYCE_SOURCE_MULTIPLICITY_HISTORICAL_EXCLUSION_BLAKE3: &str =
    "87ee5ea84b7aa6b9be9d5b46c966148aeb83fb53716e34c0f2775f59b7e65699";
const XYCE_SOURCE_MULTIPLICITY_UPSTREAM_REGRESSION_COMMIT: &str =
    "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const XYCE_SOURCE_MULTIPLICITY_UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const XYCE_SOURCE_MULTIPLICITY_HISTORICAL_ORACLE_RECORD_COUNT: usize = 13;
const XYCE_SOURCE_MULTIPLICITY_HISTORICAL_ORACLE_BLAKE3: &str =
    "04535bab1b5685bb32d586a8c9456413b62b18252d988de73b35c972fb1f6493";

// Release 7.10's four ABM_FREQ wrappers run the authored FREQ/HERTZ deck as
// ACComparator's directional GOODFILE and the corresponding .AC DATA deck as
// TESTFILE.  The selected current corpus, the removed wrappers/exclude file,
// and the exact comparator implementation are all bound to RSpice's pinned
// pre-trim source tree before native relational execution is admitted.
const XYCE_ABM_FREQUENCY_WRAPPER_OWNER_CONTRACT: &str = "abm_frequency_relational_wrapper_owner";
const XYCE_ABM_FREQUENCY_DATA_CONTROL_CONTRACT: &str = "abm_frequency_relational_data_control";
const XYCE_ABM_FREQUENCY_PRETRIM_COMMIT: &str = "80115a9277c0ddb3409acceb3d4e745fd11cddd4";
const XYCE_ABM_FREQUENCY_UPSTREAM_REGRESSION_COMMIT: &str =
    "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const XYCE_ABM_FREQUENCY_UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const XYCE_ABM_FREQUENCY_CANDIDATE_COUNT: usize = 8;
const XYCE_ABM_FREQUENCY_CANDIDATE_BLAKE3: &str =
    "7efc124b0af802d1376d54a8afc39362c79ee194cb187ed33ecb995c77e536ec";
const XYCE_ABM_FREQUENCY_CANDIDATE_CONTENT_BLAKE3: &str =
    "72b7c09c288f0b9747c3f38bfb981e5f8d4c147fe846e4d0c4a0fe05497114e6";
const XYCE_ABM_FREQUENCY_OWNER_COUNT: usize = 4;
const XYCE_ABM_FREQUENCY_OWNER_MANIFEST_BLAKE3: &str =
    "5ec62eef7013051119a34cc59f57685045cba5a3e68b37db753a7f5e2b291047";
const XYCE_ABM_FREQUENCY_EXCLUSION_COUNT: usize = 4;
const XYCE_ABM_FREQUENCY_HISTORICAL_EXCLUSION_BLAKE3: &str =
    "80cf056fce5978e4f1a11342f3ec96d2d064c87eca98777496288f47bb6acd47";
const XYCE_ABM_FREQUENCY_HISTORICAL_EXCLUDE_PATH: &str = "Netlists/ABM_FREQ/exclude";
const XYCE_ABM_FREQUENCY_HISTORICAL_EXCLUDE_BYTES: usize = 86;
const XYCE_ABM_FREQUENCY_HISTORICAL_EXCLUDE_SHA256: &str =
    "66ec2ffe0a69ab047be056920a56e45ccdab2b7884f81e49c131b0c383678b92";
const XYCE_ABM_FREQUENCY_HISTORICAL_EXCLUDE_BLAKE3: &str =
    "4843fb1c7996364566527b5bb18fcb9c733be24053bb95fa69120aba5697a9a0";
const XYCE_ABM_FREQUENCY_AC_COMPARATOR_PATH: &str = "TestScripts/ACComparator.pl";
const XYCE_ABM_FREQUENCY_AC_COMPARATOR_BYTES: usize = 14_308;
const XYCE_ABM_FREQUENCY_AC_COMPARATOR_SHA256: &str =
    "265c0c24ac886ad44bf3827f2cbe0c0f1c75c80971d5bdb3429e8048b36e1571";
const XYCE_ABM_FREQUENCY_AC_COMPARATOR_BLAKE3: &str =
    "6a1c8fdfa65116f6729343a759d172be939eb81a8617cea5a52f3572577ba926";
const XYCE_ABM_FREQUENCY_HISTORICAL_ORACLE_RECORD_COUNT: usize = 6;
const XYCE_ABM_FREQUENCY_HISTORICAL_ORACLE_BLAKE3: &str =
    "e08dd112070ff4b275d29c41cb5cc623c81687dda49aeb40460a4092c4655676";
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
const XYCE_BUG1043_PRETRIM_COMMIT: &str = "80115a9277c0ddb3409acceb3d4e745fd11cddd4";
const XYCE_BUG1043_UPSTREAM_REGRESSION_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const XYCE_BUG1043_UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const XYCE_BUG1043_FAMILY_DIR: &str = "Netlists/Certification_Tests/BUG_1043_SON";
const XYCE_BUG1043_OWNER_PATH: &str = "Netlists/Certification_Tests/BUG_1043_SON/RC_AC_params.cir";
const XYCE_BUG1043_EXPRESSION_BASELINE_PATH: &str =
    "Netlists/Certification_Tests/BUG_1043_SON/RC_AC_params_expr.cir";
const XYCE_BUG1043_ANALYTIC_PATH: &str =
    "Netlists/Certification_Tests/BUG_1043_SON/RC_AC_params_analytic.cir";
const XYCE_BUG1043_ANALYTIC_ORACLE_PATH: &str =
    "OutputData/Certification_Tests/BUG_1043_SON/RC_AC_params_analytic.cir.FD.prn";
const XYCE_BUG1043_HISTORICAL_EXCLUDE_PATH: &str =
    "Netlists/Certification_Tests/BUG_1043_SON/exclude";
const XYCE_BUG1043_OWNER_RECORD: &str =
    "netlists/certification_tests/bug_1043_son/rc_ac_params.cir";
const XYCE_BUG1043_EXPRESSION_BASELINE_RECORD: &str =
    "netlists/certification_tests/bug_1043_son/rc_ac_params_expr.cir";
const XYCE_BUG1043_ANALYTIC_RECORD: &str =
    "netlists/certification_tests/bug_1043_son/rc_ac_params_analytic.cir";
const XYCE_BUG1043_FREQUENCY_GRID: [Value; 6] = [1.0, 10.0, 100.0, 1.0e3, 1.0e4, 1.0e5];
const XYCE_BUG1043_FREQUENCY_GRID_RELATIVE_ROUNDOFF: Value = 64.0 * f64::EPSILON;
const XYCE_BUG1043_HISTORICAL_RECORD_COUNT: usize = 7;
const XYCE_BUG1043_HISTORICAL_RECORD_BYTES: usize = 1_716;
const XYCE_BUG1043_HISTORICAL_RECORDS_SHA256: &str =
    "3d7d4bc314d8e1a6a83aa0341b8ff118d6ef0d44165e8d61b1e23bbf7344e31e";
const XYCE_BUG1043_HISTORICAL_RECORDS_BLAKE3: &str =
    "594c953743f5b4dc7ce3e13f3402a55da3507e6db0819bfe8b6842a2edefd164";

const XYCE_BUG1043_HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 7] = [
    (
        "Netlists/Certification_Tests/BUG_1043_SON/Manifest.txt",
        158,
        "6b34b89d6f8adeae3c6434280a8f9fac8c85c23e5a229cf895b605fb98dbfe67",
        "d629139ef9c93c48e9d640bdb1b6ab805078b9030bac86881909f0f4cde6b9a0",
    ),
    (
        "Netlists/Certification_Tests/BUG_1043_SON/RC_AC_params.cir.sh",
        2_876,
        "b703dcca118b5a16f6c1ffb8d402c4fb99ec568db21b2dccd825939f80114b57",
        "0ae40d014a8b9ffaa74627d710ce3b2e6593ed6496655306b2a2c3a3623cba70",
    ),
    (
        "Netlists/Certification_Tests/BUG_1043_SON/RC_AC_params_analytic.cir.sh",
        2_196,
        "bcdf24c7279b9c4702a7f59ee174495a4f2a3c510d2a4fe77bf0840ca075a95f",
        "33bc99393271bee1e5160fb6c2891d6b312a122ea2b8ae7292b073c1c7d9adca",
    ),
    (
        "Netlists/Certification_Tests/BUG_1043_SON/RC_AC_params_analytic.cir.tags",
        55,
        "c38bdc58bde3bee682c7196d3bce6bb0caf028c19a6444b71952439fa8c005cf",
        "00c81a30052f4110e4d8c66513ea01747f65b3fceeb431d527e568feb23cfde5",
    ),
    (
        XYCE_BUG1043_HISTORICAL_EXCLUDE_PATH,
        22,
        "b78d4e00b18ef27ff71bcf5c4d823d80946364421414c6c4b8dc4db4c28733d7",
        "1052573d2b22c1371958f3ecb1ec656c15f72e77f3ae242665cd0fc06ec3968f",
    ),
    (
        "Netlists/Certification_Tests/BUG_1043_SON/tags",
        45,
        "596cae8652fd11a8d05526f881f5e02cdf39f1275fc953c36d298790e94a92b7",
        "7e72fa9e9bada1a6eb458907e99ccaeafdbcd3439c4f1957bd8943be939530ba",
    ),
    (
        "TestScripts/ACComparator.pl",
        14_308,
        "265c0c24ac886ad44bf3827f2cbe0c0f1c75c80971d5bdb3429e8048b36e1571",
        "6a1c8fdfa65116f6729343a759d172be939eb81a8617cea5a52f3572577ba926",
    ),
];

const XYCE_BUG1043_RETAINED_SOURCE_ARTIFACTS: [(&str, usize, &str, &str); 3] = [
    (
        "RC_AC_params.cir",
        604,
        "8488f1f8e782ddf8697d346f72faf18afecb0f9da72341feea405dc1dc009fd7",
        "9af9c44bb2d799ecc2e1bf25ea9535c48cdac2323e81c24f0c3372b2cfbae057",
    ),
    (
        "RC_AC_params_analytic.cir",
        1_196,
        "caf82e0f07da37e3398988244bd3ef386c36154c7de76596eb642d2fbdd60758",
        "485931b49884a5cfcf81ddd0dc12c7e42d3d64602c85127b6cc1e26dcbfa6efe",
    ),
    (
        "RC_AC_params_expr.cir",
        460,
        "f7c2cfbcaf1f524d46d624b171f89dc771e830a22c9bb458550dce2b462ab2c2",
        "0a049b4a280380dbf2d3027e5aaef6fd4ea97be804d1f1f9371799b0b3d1485d",
    ),
];

const XYCE_BUG1043_ANALYTIC_ORACLE_ARTIFACT: (&str, usize, &str, &str) = (
    "RC_AC_params_analytic.cir.FD.prn",
    471,
    "4b43d6e4d3489148a88749aa2d449f3b80ec8e07b902f5845d7016d7f102d286",
    "2ee40573c63fd9fd8ac46081edb528589ed4fef43596cf602e8a6676f223a1d0",
);

// Release 7.10's ABM_SPLINES ordering wrappers run each authored out-of-order
// inline lookup deck first, then its ordered control, and require their default
// PRN files to be byte-identical. Bind the exact four retained sources and the
// removed wrappers/exclude/xyce_verify artifacts before reproducing that
// directional pair contract natively.
const XYCE_ABM_LOOKUP_ORDER_WRAPPER_OWNER_CONTRACT: &str =
    "abm_splines_inline_lookup_order_wrapper_owner";
const XYCE_ABM_LOOKUP_ORDER_SORTED_CONTROL_CONTRACT: &str =
    "abm_splines_inline_lookup_order_sorted_control";
const XYCE_ABM_LOOKUP_ORDER_PRETRIM_COMMIT: &str = "80115a9277c0ddb3409acceb3d4e745fd11cddd4";
const XYCE_ABM_LOOKUP_ORDER_UPSTREAM_REGRESSION_COMMIT: &str =
    "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const XYCE_ABM_LOOKUP_ORDER_UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const XYCE_ABM_LOOKUP_ORDER_CANDIDATE_COUNT: usize = 4;
const XYCE_ABM_LOOKUP_ORDER_CANDIDATE_BLAKE3: &str =
    "9ee216508da63b2a0a0abcdfca52cb029c64c7bf947754689241327cfcf791cd";
const XYCE_ABM_LOOKUP_ORDER_CANDIDATE_CONTENT_BLAKE3: &str =
    "be2a4505167a38bad432fcbbf100e88e59da0b598c685834bd67fb1d3fb0c691";
const XYCE_ABM_LOOKUP_ORDER_OWNER_COUNT: usize = 2;
const XYCE_ABM_LOOKUP_ORDER_OWNER_MANIFEST_BLAKE3: &str =
    "286412075c19ef51b86d1e30d368c7b58cd7e610dc5fc5aa80d8edf1ff010d9d";
const XYCE_ABM_LOOKUP_ORDER_EXCLUSION_COUNT: usize = 2;
const XYCE_ABM_LOOKUP_ORDER_HISTORICAL_EXCLUSION_BLAKE3: &str =
    "bcddcc20cfa2a46933f6598d047cb685b2a93f200a881d8e0bd0d51cea2beb72";
const XYCE_ABM_LOOKUP_ORDER_HISTORICAL_EXCLUDE_PATH: &str = "Netlists/ABM_SPLINES/exclude";
const XYCE_ABM_LOOKUP_ORDER_HISTORICAL_EXCLUDE_BYTES: usize = 59;
const XYCE_ABM_LOOKUP_ORDER_HISTORICAL_EXCLUDE_SHA256: &str =
    "4ab0d89ccf9cd4348d3fd0290c511d6a32e6ab0fe988751bd5c78bc8bea94402";
const XYCE_ABM_LOOKUP_ORDER_HISTORICAL_EXCLUDE_BLAKE3: &str =
    "16c24099b5b183a50760d0780895488630fc6681fa56919fff9c423871946f58";
const XYCE_ABM_LOOKUP_ORDER_HISTORICAL_ORACLE_RECORD_COUNT: usize = 4;
const XYCE_ABM_LOOKUP_ORDER_HISTORICAL_ORACLE_BLAKE3: &str =
    "897989a3f58c9339b2dcf88fc3333987ec1b1b57988cd6a78c54d1f56db88277";
const XYCE_ABM_LOOKUP_ORDER_GRID: [Value; 11] =
    [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];

// Release 7.10's BUG_38_SON wrapper executes the ordinary SUBCKT-formal
// spelling and the HSpice-compatible parenthesized spelling independently,
// then applies `diff -i` to their default PRN files. Bind the exact retained
// pair and the historical README/wrapper/exclude artifacts before reproducing
// that relational oracle natively.
const XYCE_BUG38_WRAPPER_OWNER_CONTRACT: &str = "bug38_subckt_formal_parentheses_wrapper_owner";
const XYCE_BUG38_PARENTHESIZED_CONTROL_CONTRACT: &str = "bug38_subckt_formal_parentheses_control";
const XYCE_BUG38_PRETRIM_COMMIT: &str = "80115a9277c0ddb3409acceb3d4e745fd11cddd4";
const XYCE_BUG38_UPSTREAM_REGRESSION_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const XYCE_BUG38_UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const XYCE_BUG38_OWNER_PATH: &str = "Netlists/Certification_Tests/BUG_38_SON/bug_38_son.cir";
const XYCE_BUG38_CONTROL_PATH: &str = "Netlists/Certification_Tests/BUG_38_SON/bug_38_son_p.cir";
const XYCE_BUG38_OWNER_RECORD: &str = "netlists/certification_tests/bug_38_son/bug_38_son.cir";
const XYCE_BUG38_CONTROL_RECORD: &str = "netlists/certification_tests/bug_38_son/bug_38_son_p.cir";
const XYCE_BUG38_OWNER_CONTENT_BLAKE3: &str =
    "92ee9edf950c7f319c7d73bb3307b8792f5fad512f3127fd20149fc28e392a57";
const XYCE_BUG38_CONTROL_CONTENT_BLAKE3: &str =
    "15f55cf395f52fcb1a9f36bfe8819e26bc66616d1ca781af0792331d0cbec9e5";
const XYCE_BUG38_HISTORICAL_EXCLUDE_PATH: &str = "Netlists/Certification_Tests/BUG_38_SON/exclude";
const XYCE_BUG38_CANDIDATE_COUNT: usize = 2;
const XYCE_BUG38_CANDIDATE_BLAKE3: &str =
    "c1dda42002414671ded07873bba2c55d4efd8b8a3267eb02cff9e7d8597d6d8a";
const XYCE_BUG38_CANDIDATE_CONTENT_BLAKE3: &str =
    "2ec24f64a38c27dd2c73416783c24c40fd816ac97dd1b12e736939454db2c172";
const XYCE_BUG38_OWNER_MANIFEST_BLAKE3: &str =
    "7ee59f89e3bb85ece40a4dc090b518349194913e7a8c898b0046e4cef58b6615";
const XYCE_BUG38_HISTORICAL_EXCLUSION_BLAKE3: &str =
    "c106473166f096b0fd22fff4d50c18902a59f93dccb1d71e77b21f7d7f465c4a";
const XYCE_BUG38_REQUIRED_ORACLE_RECORD_COUNT: usize = 6;
const XYCE_BUG38_REQUIRED_ORACLE_BYTES: usize = 1_447;
const XYCE_BUG38_REQUIRED_ORACLE_SHA256: &str =
    "1b7cc7a326fb07591388a4e3c81bf2adc667616f32d88ef5e9f7c894866f0347";
const XYCE_BUG38_REQUIRED_ORACLE_BLAKE3: &str =
    "f15a0c610efd2b5def26fc489522c555f685653924554bbd659b1f26b30d2de1";
const XYCE_BUG38_HISTORICAL_ORACLE_RECORD_COUNT: usize = 7;
const XYCE_BUG38_HISTORICAL_ORACLE_BLAKE3: &str =
    "b3aad59f1109003205077dc418a34d40426a3332c9e39770b72b90d4b77fde52";

const XYCE_BUG38_HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 7] = [
    (
        "Netlists/Certification_Tests/BUG_38_SON/README",
        630,
        "13e535a162d5218d0e53fb1274617f34e9d5b7715d0a86a8fe8b05b49ba9432e",
        "d8c7340d9e24ded977e7aedb7838937f09497508cbc5eb67bc8e163356780869",
    ),
    (
        "Netlists/Certification_Tests/BUG_38_SON/Manifest.txt",
        70,
        "2f2ae96c7842a02c48cd8a5ca935eaeba94bd74604a958a5dd884fc706799f1b",
        "07497706a574f62298ef5e4dbf83872f20b027f145ff9b4c149d680ef91f7094",
    ),
    (
        XYCE_BUG38_OWNER_PATH,
        187,
        "f9a56497613ef618fcbf552c755a51fa460a1e61c430e3331b8cb0348ccd0a62",
        XYCE_BUG38_OWNER_CONTENT_BLAKE3,
    ),
    (
        "Netlists/Certification_Tests/BUG_38_SON/bug_38_son.cir.sh",
        1_445,
        "47ff59fa3801f4c8291e03e4029d78b75bebc56357993014afc570bb1b495b7e",
        "bfa2c5f0b5f735b699177726dad357a4c5cc1cb1c5c2cbd8d07c467ccc2b0fb9",
    ),
    (
        XYCE_BUG38_CONTROL_PATH,
        203,
        "ba92a6d955a1179bab285d4eddd583e4fbc8d70ac56e83d37b0bd0d9912cfb05",
        XYCE_BUG38_CONTROL_CONTENT_BLAKE3,
    ),
    (
        XYCE_BUG38_HISTORICAL_EXCLUDE_PATH,
        17,
        "8c51a1344c808cf7ef0acb904d45c10f54a223817cfc77a08cd3d499aca3b347",
        "802378e620d5e2c38b752ff311e969fccef89cad2324a5269f3c0107defa4cee",
    ),
    (
        "TestScripts/XyceRegression/Tools.pm",
        68_108,
        "5b5f86c02d46a1f3bdad5292e7e91d25a9e08e71490643d8d5ed7ae20f9d55e3",
        "13bd274632744ddc4b8baee680ddc9770902793ed7ee892ecdedd4dcb3828667",
    ),
];

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
const XYCE_BUG39_PRETRIM_COMMIT: &str = "80115a9277c0ddb3409acceb3d4e745fd11cddd4";
const XYCE_BUG39_UPSTREAM_REGRESSION_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const XYCE_BUG39_UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
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
const XYCE_BUG39_HISTORICAL_EXCLUDE_PATH: &str = "Netlists/Certification_Tests/BUG_39_SON/exclude";
const XYCE_BUG39_EMPTY_CONTENT_BLAKE3: &str =
    "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262";
const XYCE_BUG39_CANDIDATE_COUNT: usize = 2;
const XYCE_BUG39_CANDIDATE_BLAKE3: &str =
    "bdfd3ca2dfe1884ee426617f009dfc23e60a306ce21ce78bd7024506e4848f8f";
const XYCE_BUG39_CANDIDATE_CONTENT_BLAKE3: &str =
    "ac62ef2c80d39221b0869b597488050562de2326329280399c16cb3b89932bc4";
const XYCE_BUG39_OWNER_MANIFEST_BLAKE3: &str =
    "106b025524124c209ed099f9d13e8c0523494320c91811e091b3e0657356fe68";
const XYCE_BUG39_HISTORICAL_EXCLUSION_BLAKE3: &str =
    "c39fe61b2e9e34fa277f15016d40925329a3965a53083237959ecc9cac7daadd";
const XYCE_BUG39_DETERMINISTIC_CANDIDATE_COUNT: usize = 4;
const XYCE_BUG39_DETERMINISTIC_CANDIDATE_BLAKE3: &str =
    "60eaf1c149554b9a114a4348101c4f6e31f9531ffd66547d03d13488f126d267";
const XYCE_BUG39_DETERMINISTIC_CONTENT_BLAKE3: &str =
    "15c9f12196f084ffaf6cf329aab3fdc86c78786836b70774de20f890cd973540";
const XYCE_BUG39_DETERMINISTIC_OWNER_BLAKE3: &str =
    "f01c785308ef7ff2d9477720cd4fefead2531f9699a3580d46fa40472bcf77b6";
const XYCE_BUG39_REQUIRED_ORACLE_RECORD_COUNT: usize = 23;
const XYCE_BUG39_REQUIRED_ORACLE_BYTES: usize = 5_559;
const XYCE_BUG39_REQUIRED_ORACLE_SHA256: &str =
    "9942679e83b82ebcb3ad5d749fc61ba58af88ff3e23d08e550f83d80fc876f7f";
const XYCE_BUG39_REQUIRED_ORACLE_BLAKE3: &str =
    "e43bcfb707f28a3fdf35004dc01157e67940fdf1a7c6e74727cff3756ff438f0";
const XYCE_BUG39_HISTORICAL_ORACLE_RECORD_COUNT: usize = 24;
const XYCE_BUG39_HISTORICAL_ORACLE_BLAKE3: &str =
    "f4a60b0bbb5d306718efd7371cfd84181650c71617c8b3ad908859a5e33f5b21";
const XYCE_BUG39_RETAINED_RECORD_COUNT: usize = 11;
const XYCE_BUG39_RETAINED_RECORDS_BLAKE3: &str =
    "6ea5836525a3b58413107c49bf1819d150a1e1f5b7dffa8985e30b78bc92a832";
const XYCE_BUG39_SAMPLE_COUNT: usize = 10_000;
const XYCE_BUG39_MEAN: Value = 100.0;
const XYCE_BUG39_EXPECTED_SIGMA: Value = 1.0;
const XYCE_BUG39_MOMENT_TOLERANCE: Value = 5.0e-2;

const XYCE_BUG39_HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 24] = [
    (
        "Netlists/Certification_Tests/BUG_39_SON/README",
        879,
        "148d072b44042d70d47ad191f462cd356b1868f4a3dddf49f9d25c271e0b13a0",
        "37bf9fd8b2e8b91e8f7baa45e985fe19450b18208753eb8533ac473ab6709a70",
    ),
    (
        "Netlists/Certification_Tests/BUG_39_SON/Manifest.txt",
        285,
        "57d8660cdfea15e033bf4138f493e92c19beb68a676911232f4d15eef27d7b7d",
        "ccbf49ce2050f94549bb6d2fbab51d40c4332bd7615585826fd49d10fcdd40cc",
    ),
    (
        XYCE_BUG39_AGAUSS_PATH,
        0,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        XYCE_BUG39_EMPTY_CONTENT_BLAKE3,
    ),
    (
        "Netlists/Certification_Tests/BUG_39_SON/agauss.cir.sh",
        5_235,
        "2173930707b4a5a349fb03d1f324860f0b2f71180d02ffb88a9510615e0d5055",
        "c779be31922b96ba6cfe8acfa07ce002852d9cffac791ceddce88480a22ad0df",
    ),
    (
        "Netlists/Certification_Tests/BUG_39_SON/bug39_int.cir",
        168,
        "ed26197d4f299a496f0c714a08c328cda560c1194c25ae0b415f73ff8bb1898c",
        "62367769e743519c676f1283ee41d33a7310f9556b330f9a5da1178eaac5ec3b",
    ),
    (
        "Netlists/Certification_Tests/BUG_39_SON/bug39_int.cir.prn.gs",
        105,
        "340ab7510b526e583cac2b6ac15388df3217514dc96fe4620514ac47defa015c",
        "2b05b73a06220bce643b7961dcfe068fe7c14a9bf806100d23e6ee2f66b05629",
    ),
    (
        "Netlists/Certification_Tests/BUG_39_SON/bug39_int.cir.sh",
        1_097,
        "7c38fcf5e57bbbc015ff4d7c099205c6f0801d4eaee0ac87ea5c1b22baeba9f0",
        "cf22359501be453ee5822bb84a002554a1fd844c504bb1c90cfabc08146f0380",
    ),
    (
        "Netlists/Certification_Tests/BUG_39_SON/bug39_limit.cir",
        482,
        "9b1f3329062d0d7dc1f49b8a94fd88262a796e8dd92ae1c6a2d4fc579f12910f",
        "32e9d468ab49ec892c36f7672d1d2962f54557c9df8c89f1103185863cc1c007",
    ),
    (
        "Netlists/Certification_Tests/BUG_39_SON/bug39_limit.cir.prn.gs",
        111,
        "8eba99b9cfba84e66b518177060d38dc0a8fde786fd0bfc86d9818a35bee212f",
        "190fe153a2f5887f562ed8d1fdd2546acbab3b2d733bbc9fb2e0bb9f47e703aa",
    ),
    (
        "Netlists/Certification_Tests/BUG_39_SON/bug39_limit.cir.sh",
        1_099,
        "2bba1611c424ea3332f38cd930a4227599848b98ef84c566874f0c718f22dbc2",
        "ab0acfba22a5fbc0c43d10295f607ec93068f3dfaf25cb02b383a1c52342dd2e",
    ),
    (
        "Netlists/Certification_Tests/BUG_39_SON/bug39_pow.cir",
        149,
        "76625ca95d8a4f51f6161ba610e9d569c0c161bd228cff987f6c049cc21c0f78",
        "175baf98941dd1e24a7942aef300a9ecc4edfa7a6e8c5cf06094e4ed55606916",
    ),
    (
        "Netlists/Certification_Tests/BUG_39_SON/bug39_pow.cir.prn.gs",
        105,
        "ec7adbfa8d794ff0a8c10f0f119c74b57b3f7effb843472c6148fee4cfc2c0d6",
        "356a59eaec3ff2e5a3dee35f0510dce0b73b512ed5f21e96bef03fc10ff149af",
    ),
    (
        "Netlists/Certification_Tests/BUG_39_SON/bug39_pow.cir.sh",
        1_097,
        "9f12693fd5d0690197861e8a51612a44c84a59b7a3d7ee94bdc4bbec6fccc11c",
        "57de1dc11d431315e6a3f8155c6c1510785afa0e2ab35e0095c043fd7fd5e4ce",
    ),
    (
        "Netlists/Certification_Tests/BUG_39_SON/bug39_sign.cir",
        152,
        "ea2e5d157bf39d575b2c33c3ef179dfbf9f879f21383ba001eed4bfc300e142d",
        "cb8c1dfdb32f8e36242b05d475735df124d818337bf9798246ee5e6bcd75ab51",
    ),
    (
        "Netlists/Certification_Tests/BUG_39_SON/bug39_sign.cir.prn.gs",
        105,
        "9d1a472e7b465d0c2caa45a16d99328334ea0a894c3fa7da844faa465b393236",
        "cc173d820146330a519e07328559bf51061a815906837487f06be67c0b7cf545",
    ),
    (
        "Netlists/Certification_Tests/BUG_39_SON/bug39_sign.cir.sh",
        1_098,
        "308f59a5d46011dd8cd80a4be0b175c49cb58904b82b5f96241b32697a670f15",
        "d6a1c069603d12b1bc7e9089de080381ff20bf0e20b3b4e0db96f180bb1f14fd",
    ),
    (
        XYCE_BUG39_GAUSS_PATH,
        0,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        XYCE_BUG39_EMPTY_CONTENT_BLAKE3,
    ),
    (
        "Netlists/Certification_Tests/BUG_39_SON/gauss.cir.sh",
        5_345,
        "3c6f0797dfb473cedb1b874b54fa53897b0d154c6233ecaf36b079f0f85b9966",
        "b35c1382598a5a766c72608d166b89f112a727ed5a401007725aa813b2812874",
    ),
    (
        "Netlists/Certification_Tests/BUG_39_SON/tags",
        46,
        "99021efbf1ed8e6a6563e2cf21f130d010a308452e1b7a2895660e3ac8d904d5",
        "2b5db3920215b065bab2f4ca1f443963363fcab5e31c9788788e6ca7cf664e34",
    ),
    (
        XYCE_BUG39_HISTORICAL_EXCLUDE_PATH,
        21,
        "20533602deb07138a58ed4a233abb4f381c2a1ee77d2d5d2bf4a599fe24446db",
        "193cec42858538b08aea1096c2a0fd7ddffd4666761078b36ace1a7ffee5ec87",
    ),
    (
        "TestScripts/XyceRegression/Tools.pm",
        68_108,
        "5b5f86c02d46a1f3bdad5292e7e91d25a9e08e71490643d8d5ed7ae20f9d55e3",
        "13bd274632744ddc4b8baee680ddc9770902793ed7ee892ecdedd4dcb3828667",
    ),
    (
        "TestScripts/XyceVerify/DCSources.pm",
        2_739,
        "b2ddcab5ad5a89c428b9b4430190fa27ef7106da7e7afeb31452c81890a9a006",
        "0905f9dc79d7c5bdbe17e3c2360cd063d6fcbf41823a410f98b236783d109ad7",
    ),
    (
        "TestScripts/XyceVerify/DCSweep.pm",
        9_301,
        "2246da2374e6cce3ea516a50e472fb07f7481e8b0effb20d4a650e6b6cb1eda0",
        "b9cc7d905d001ebe2ace44936b9631e4bdcbf42bca4d4b34c5866262cd11d9a3",
    ),
    (
        XYCE_RELEASE_710_XYCE_VERIFY_PATH,
        XYCE_RELEASE_710_XYCE_VERIFY_BYTES,
        XYCE_RELEASE_710_XYCE_VERIFY_SHA256,
        XYCE_RELEASE_710_XYCE_VERIFY_BLAKE3,
    ),
];

const XYCE_BUG39_RETAINED_ARTIFACTS: [(&str, usize, &str, &str); 11] = [
    (
        "agauss.cir",
        0,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        XYCE_BUG39_EMPTY_CONTENT_BLAKE3,
    ),
    (
        "bug39_int.cir",
        168,
        "ed26197d4f299a496f0c714a08c328cda560c1194c25ae0b415f73ff8bb1898c",
        "62367769e743519c676f1283ee41d33a7310f9556b330f9a5da1178eaac5ec3b",
    ),
    (
        "bug39_int.cir.prn.gs",
        105,
        "340ab7510b526e583cac2b6ac15388df3217514dc96fe4620514ac47defa015c",
        "2b05b73a06220bce643b7961dcfe068fe7c14a9bf806100d23e6ee2f66b05629",
    ),
    (
        "bug39_limit.cir",
        482,
        "9b1f3329062d0d7dc1f49b8a94fd88262a796e8dd92ae1c6a2d4fc579f12910f",
        "32e9d468ab49ec892c36f7672d1d2962f54557c9df8c89f1103185863cc1c007",
    ),
    (
        "bug39_limit.cir.prn.gs",
        111,
        "8eba99b9cfba84e66b518177060d38dc0a8fde786fd0bfc86d9818a35bee212f",
        "190fe153a2f5887f562ed8d1fdd2546acbab3b2d733bbc9fb2e0bb9f47e703aa",
    ),
    (
        "bug39_pow.cir",
        149,
        "76625ca95d8a4f51f6161ba610e9d569c0c161bd228cff987f6c049cc21c0f78",
        "175baf98941dd1e24a7942aef300a9ecc4edfa7a6e8c5cf06094e4ed55606916",
    ),
    (
        "bug39_pow.cir.prn.gs",
        105,
        "ec7adbfa8d794ff0a8c10f0f119c74b57b3f7effb843472c6148fee4cfc2c0d6",
        "356a59eaec3ff2e5a3dee35f0510dce0b73b512ed5f21e96bef03fc10ff149af",
    ),
    (
        "bug39_sign.cir",
        152,
        "ea2e5d157bf39d575b2c33c3ef179dfbf9f879f21383ba001eed4bfc300e142d",
        "cb8c1dfdb32f8e36242b05d475735df124d818337bf9798246ee5e6bcd75ab51",
    ),
    (
        "bug39_sign.cir.prn.gs",
        105,
        "9d1a472e7b465d0c2caa45a16d99328334ea0a894c3fa7da844faa465b393236",
        "cc173d820146330a519e07328559bf51061a815906837487f06be67c0b7cf545",
    ),
    (
        "gauss.cir",
        0,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        XYCE_BUG39_EMPTY_CONTENT_BLAKE3,
    ),
    (
        "README",
        879,
        "148d072b44042d70d47ad191f462cd356b1868f4a3dddf49f9d25c271e0b13a0",
        "37bf9fd8b2e8b91e8f7baa45e985fe19450b18208753eb8533ac473ab6709a70",
    ),
];

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
const XYCE_BUG402_PRETRIM_COMMIT: &str = "80115a9277c0ddb3409acceb3d4e745fd11cddd4";
const XYCE_BUG402_UPSTREAM_REGRESSION_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const XYCE_BUG402_UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
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
const XYCE_BUG402_HISTORICAL_EXCLUDE_PATH: &str =
    "Netlists/Certification_Tests/BUG_402_SON/exclude";
const XYCE_BUG402_HISTORICAL_RECORD_COUNT: usize = 13;
const XYCE_BUG402_HISTORICAL_RECORD_BYTES: usize = 3_111;
const XYCE_BUG402_HISTORICAL_RECORDS_SHA256: &str =
    "1ff1a9451de049f1156dafc4c6ed183f46119020a18f011afc39ba3889f18326";
const XYCE_BUG402_HISTORICAL_RECORDS_BLAKE3: &str =
    "d709cca8f53047ea80f2eb83c3bf0e96edaca874302f1985e375e26874a59704";
const XYCE_BUG402_RETAINED_RECORD_COUNT: usize = 4;
const XYCE_BUG402_RETAINED_RECORD_BYTES: usize = 611;
const XYCE_BUG402_RETAINED_RECORDS_SHA256: &str =
    "8ce7e9cddff050048c59065652c124ef2d49a22710a1738d136f7092443b2b78";
const XYCE_BUG402_RETAINED_RECORDS_BLAKE3: &str =
    "21d37da187ca098ed60788969dcbafca286b7d093c9d425529f1d9a813af8c0c";
const XYCE_BUG402_DC_POINT_COUNT: usize = 51;

const XYCE_BUG402_HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 13] = [
    (
        "Netlists/Certification_Tests/BUG_402_SON/CMakeLists.txt",
        1_679,
        "f527618218b4c20f09c7e081af2a889b2c96d20c06de57913fb59aa2b035ed94",
        "2ee4f59fc80286e0a26ed85babb5b5e97f079af6ee79a89ef226794fdc82d485",
    ),
    (
        "Netlists/Certification_Tests/BUG_402_SON/Manifest.txt",
        104,
        "52be93a816570755e87fdcbddfe264761a40747998f34f9ee2857d51c1339db3",
        "3a763c27d89cd630fed8180a36b6346d0af065d29464a32312ac17709c26366a",
    ),
    (
        "Netlists/Certification_Tests/BUG_402_SON/README",
        385,
        "869fcd0742641e619af164c016a9dee520b581c9cd272ace32cc2a2457bc835a",
        "de6005465a28610ffa126624506fe999e0f28b59083eaea37ac4688845b07156",
    ),
    (
        XYCE_BUG402_SPICE_MEMBER_PATH,
        1_096,
        "4fffcf69d172c4d614673de74539634997dd35776743408ebe160701ff339d4c",
        "dd4b124276431c8b9baca2fb9840b79768ec5843ef4afb20e68ccba17549e007",
    ),
    (
        XYCE_BUG402_XYCE_REFERENCE_PATH,
        1_102,
        "aefc2180e4bb6a5b39e3f4105546f3c5ba22af3b74f003d2d4d3c55f45938e9e",
        "8624a2ec9239bc664b979f01400729da865ce90a887128c92f42da4d21c9af1d",
    ),
    (
        XYCE_BUG402_OWNER_PATH,
        115,
        "95f417f0cc5ff658ef4a79e88f69bb9964be771688d6a75629483c3ddd2cd72d",
        "9f19d4388692a257a348ab24a50885198ce2f10854eafc9f44ada4c96b37c2a7",
    ),
    (
        "Netlists/Certification_Tests/BUG_402_SON/bug402son.cir.sh",
        1_995,
        "798c64c762c996fb82935fcf320921e6151e3917daa0652d26af90556372598b",
        "aa134307353972d33f0cc8b3ccacf486258ab66ed85399fb021ab3751e98b710",
    ),
    (
        XYCE_BUG402_HISTORICAL_EXCLUDE_PATH,
        54,
        "f79abe0ee683e65894a7a4d4d96e55bf1ebc8c545601f40c5f3c9e8c1de984b2",
        "49588e88f0ffe165e0a0bfd101fc689f91523be673a1435321e208333877b4b2",
    ),
    (
        "Netlists/Certification_Tests/BUG_402_SON/tags",
        32,
        "a1b81f4136c15ef5d8965a5ea19cb9245b6e7f8297476a01b4dd5743327cfd23",
        "f3260384fe5fe9b068601a1b5828440bf38cbdda3bc6ff0f1cceb6165dcb233e",
    ),
    (
        "TestScripts/XyceRegression/Tools.pm",
        68_108,
        "5b5f86c02d46a1f3bdad5292e7e91d25a9e08e71490643d8d5ed7ae20f9d55e3",
        "13bd274632744ddc4b8baee680ddc9770902793ed7ee892ecdedd4dcb3828667",
    ),
    (
        "TestScripts/XyceVerify/DCSources.pm",
        2_739,
        "b2ddcab5ad5a89c428b9b4430190fa27ef7106da7e7afeb31452c81890a9a006",
        "0905f9dc79d7c5bdbe17e3c2360cd063d6fcbf41823a410f98b236783d109ad7",
    ),
    (
        "TestScripts/XyceVerify/DCSweep.pm",
        9_301,
        "2246da2374e6cce3ea516a50e472fb07f7481e8b0effb20d4a650e6b6cb1eda0",
        "b9cc7d905d001ebe2ace44936b9631e4bdcbf42bca4d4b34c5866262cd11d9a3",
    ),
    (
        XYCE_RELEASE_710_XYCE_VERIFY_PATH,
        XYCE_RELEASE_710_XYCE_VERIFY_BYTES,
        XYCE_RELEASE_710_XYCE_VERIFY_SHA256,
        XYCE_RELEASE_710_XYCE_VERIFY_BLAKE3,
    ),
];

const XYCE_BUG402_RETAINED_ARTIFACTS: [(&str, usize, &str, &str); 4] = [
    (
        "bug402son.cir",
        115,
        "95f417f0cc5ff658ef4a79e88f69bb9964be771688d6a75629483c3ddd2cd72d",
        "9f19d4388692a257a348ab24a50885198ce2f10854eafc9f44ada4c96b37c2a7",
    ),
    (
        "bug402SpiceTempOptions.cir",
        1_096,
        "4fffcf69d172c4d614673de74539634997dd35776743408ebe160701ff339d4c",
        "dd4b124276431c8b9baca2fb9840b79768ec5843ef4afb20e68ccba17549e007",
    ),
    (
        "bug402XyceTempOptions.cir",
        1_102,
        "aefc2180e4bb6a5b39e3f4105546f3c5ba22af3b74f003d2d4d3c55f45938e9e",
        "8624a2ec9239bc664b979f01400729da865ce90a887128c92f42da4d21c9af1d",
    ),
    (
        "README",
        385,
        "869fcd0742641e619af164c016a9dee520b581c9cd272ace32cc2a2457bc835a",
        "de6005465a28610ffa126624506fe999e0f28b59083eaea37ac4688845b07156",
    ),
];

const XYCE_BUG354_FUNCTION_PATH: &str = "Netlists/Certification_Tests/BUG_354_SON/bad_function.cir";
const XYCE_BUG354_LEAD_CURRENT_PATH: &str =
    "Netlists/Certification_Tests/BUG_354_SON/bad_leadcurrent.cir";
const XYCE_BUG354_PARAMETER_PATH: &str =
    "Netlists/Certification_Tests/BUG_354_SON/bad_parameter.cir";
const XYCE_BUG354_FUNCTION_RECORD: &str =
    "netlists/certification_tests/bug_354_son/bad_function.cir";
const XYCE_BUG354_LEAD_CURRENT_RECORD: &str =
    "netlists/certification_tests/bug_354_son/bad_leadcurrent.cir";
const XYCE_BUG354_PARAMETER_RECORD: &str =
    "netlists/certification_tests/bug_354_son/bad_parameter.cir";
const XYCE_BUG354_FUNCTION_SOURCE_BLAKE3: &str =
    "ebaa47b9ad2236255aa47c2f7e3dab8d3bd544bbcf0d83ca5b443859fcf9ecc2";
const XYCE_BUG354_LEAD_CURRENT_SOURCE_BLAKE3: &str =
    "7b6dd2464c7638740c94a2c2f711855d5076f8ff3f83c8af6130c4668daf76cc";
const XYCE_BUG354_PARAMETER_SOURCE_BLAKE3: &str =
    "fe21209b8cce1da0fdcfa110995a7ee3598f26c20f1157560c662dca5428821e";
const XYCE_BUG354_PHYSICAL_CENSUS_BLAKE3: &str =
    "b18f00d6d7641e631c96fd17db56bbd9b5a09dbe4ffbea9295f120c806bd7ffa";
const XYCE_BUG354_MANIFEST_CENSUS_BLAKE3: &str =
    "d895cc88602ef2af2f180ac3849d9896f577dbaf37c9f65b34140c008ab7864b";
const XYCE_BUG354_UPSTREAM_REGRESSION_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const XYCE_BUG354_UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const XYCE_BUG354_HISTORICAL_TIMEOUT_MS: u128 = 30_000;
const XYCE_BUG354_HISTORICAL_RECORD_COUNT: usize = 11;
const XYCE_BUG354_HISTORICAL_RECORD_BYTES: usize = 2_699;
const XYCE_BUG354_HISTORICAL_RECORDS_SHA256: &str =
    "bf8a3aea6966f912948443da0361ff159bc989ed11ef93b7343a2f45c6156a06";
const XYCE_BUG354_HISTORICAL_RECORDS_BLAKE3: &str =
    "2f4903a613cb20f1e27285fa2634adbd43e75d74d15cb01b8d19f71bfa5b2bd8";
const XYCE_BUG354_HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 11] = [
    (
        "Netlists/Certification_Tests/BUG_354_SON/CMakeLists.txt",
        4_699,
        "e73d08da637d40031570ef7b24a428ffa65ae6be87766f48349d0c5ffdbb8952",
        "6e6f23bd0f47b8dc30e463184c6615b895acef67f6a1a02c73d4f415aa915760",
    ),
    (
        "Netlists/Certification_Tests/BUG_354_SON/Manifest.txt",
        132,
        "413ab42380dd5161d9009914b0412504f57638874ac6d4687eb3e2f7fef5a6c5",
        "aee7bfe7537430cc438b7ae34bca14ff3c963dcc12eb2c9fa8f2ba4a0b558faa",
    ),
    (
        XYCE_BUG354_FUNCTION_PATH,
        117,
        "de0a3b29d62c8aa5ca2204bcc1a1a78c02a58bb8150774d050482619860310fb",
        "eb4dcb54436aeb0d2479cd3d8adae364a0f2d31e277b637b3d972a886c188c4f",
    ),
    (
        "Netlists/Certification_Tests/BUG_354_SON/bad_function.cir.sh",
        1_369,
        "f81a348237f09a695df74391f6f41a6108d2c26e1e0c742730115c559f8c9eae",
        "d050e20cc6bc981d8dae880c74c3b022cb894ea42fff2f92210105a769a18392",
    ),
    (
        XYCE_BUG354_LEAD_CURRENT_PATH,
        119,
        "34c89b53637f25b02797aeef5fbbd6701dc64f505297113e84a3dd7b8f68d5a0",
        "ee6893cba54dd03be7f2576ea25b2a15de9466db930e40cbfa01632162ba3b89",
    ),
    (
        "Netlists/Certification_Tests/BUG_354_SON/bad_leadcurrent.cir.sh",
        1_369,
        "f81a348237f09a695df74391f6f41a6108d2c26e1e0c742730115c559f8c9eae",
        "d050e20cc6bc981d8dae880c74c3b022cb894ea42fff2f92210105a769a18392",
    ),
    (
        XYCE_BUG354_PARAMETER_PATH,
        113,
        "61059a561c3622d046a7b39966976997f0c18b8f90bbd0494d91d94787f44901",
        "5adf7955644a95ea09536f0d8ba4865a6fcb1a55b9d18a7ef5c2b3314e9ba5cc",
    ),
    (
        "Netlists/Certification_Tests/BUG_354_SON/bad_parameter.cir.sh",
        1_369,
        "f81a348237f09a695df74391f6f41a6108d2c26e1e0c742730115c559f8c9eae",
        "d050e20cc6bc981d8dae880c74c3b022cb894ea42fff2f92210105a769a18392",
    ),
    (
        "Netlists/Certification_Tests/BUG_354_SON/options",
        13,
        "381cd29ca4d9097c73fccc5f46cea0c37bd3e71da803e56ccad41d8270de9c0e",
        "8e9c4c362e6a201344f7fd4b55680c6db23a1ba99121d41b9dae7573cff78b81",
    ),
    (
        "Netlists/Certification_Tests/BUG_354_SON/tags",
        37,
        "5aa3c799dbcc4d28aa47a0c6afe4708f86e4a59e36691e8312c7bca63bfc262b",
        "979508f32619a518a60b1e89576d249d9f292fdfbd580936d7a07785f532bca0",
    ),
    (
        "TestScripts/XyceRegression/Tools.pm",
        68_108,
        "5b5f86c02d46a1f3bdad5292e7e91d25a9e08e71490643d8d5ed7ae20f9d55e3",
        "13bd274632744ddc4b8baee680ddc9770902793ed7ee892ecdedd4dcb3828667",
    ),
];
const XYCE_BUG354_RETAINED_ARTIFACTS: [(&str, usize, &str, &str); 4] = [
    (
        "bad_function.cir",
        117,
        "de0a3b29d62c8aa5ca2204bcc1a1a78c02a58bb8150774d050482619860310fb",
        "eb4dcb54436aeb0d2479cd3d8adae364a0f2d31e277b637b3d972a886c188c4f",
    ),
    (
        "bad_leadcurrent.cir",
        119,
        "34c89b53637f25b02797aeef5fbbd6701dc64f505297113e84a3dd7b8f68d5a0",
        "ee6893cba54dd03be7f2576ea25b2a15de9466db930e40cbfa01632162ba3b89",
    ),
    (
        "bad_parameter.cir",
        113,
        "61059a561c3622d046a7b39966976997f0c18b8f90bbd0494d91d94787f44901",
        "5adf7955644a95ea09536f0d8ba4865a6fcb1a55b9d18a7ef5c2b3314e9ba5cc",
    ),
    (
        "options",
        13,
        "381cd29ca4d9097c73fccc5f46cea0c37bd3e71da803e56ccad41d8270de9c0e",
        "8e9c4c362e6a201344f7fd4b55680c6db23a1ba99121d41b9dae7573cff78b81",
    ),
];

// BUG_48_SON proves that the historical MOS LEVEL=54 alias selects BSIM4.
// Release 7.10 required only a successful simulator exit; its comparator was
// commented out and no numerical gold was authoritative. RSpice strengthens
// that success predicate with a typed native-device and finite-DC contract.
const XYCE_BUG48_CONTRACT: &str = "bug48_level54_native_bsim4_success_wrapper";
const XYCE_BUG48_PATH: &str = "Netlists/Certification_Tests/BUG_48_SON/test.cir";
const XYCE_BUG48_RECORD: &str = "netlists/certification_tests/bug_48_son/test.cir";
const XYCE_BUG48_UPSTREAM_REGRESSION_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const XYCE_BUG48_UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const XYCE_BUG48_HISTORICAL_RECORD_COUNT: usize = 7;
const XYCE_BUG48_HISTORICAL_RECORD_BYTES: usize = 1_668;
const XYCE_BUG48_HISTORICAL_RECORDS_SHA256: &str =
    "f876dec2f93e404e5c6654d062f16697804e61893dc38b030ab0a78415f85235";
const XYCE_BUG48_HISTORICAL_RECORDS_BLAKE3: &str =
    "90d55291b11798a47c9cb097a57bc50a9de9636effcd4a680fa0fa1b093cf418";
const XYCE_BUG48_HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 7] = [
    (
        "Netlists/Certification_Tests/BUG_48_SON/CMakeLists.txt",
        1_657,
        "87e1411241e7fd69350ed26f9459808dbda68310fb5d59347f73644654f72e10",
        "8ff147aab4c92da1443d80b4a89d489d4f376301e81a085fe82e74ab9ebe2d17",
    ),
    (
        "Netlists/Certification_Tests/BUG_48_SON/Manifest.txt",
        33,
        "65900e27f04a97ab6c441dde45ba3a30dc5095f2479d90578f384fda71f6e467",
        "54b3b72677a577f8d7f7e22513a89a6ddf286b281d9c884d5d3569e31a0fe929",
    ),
    (
        "Netlists/Certification_Tests/BUG_48_SON/README",
        109,
        "3d6c753db8e5ed4fab8a8be91b462203b8cc248ab23e2abd029c69f41aee0bde",
        "10303ccc21e8d89817bc3044a71bb48c33e2d4c11934fa188b659b25e1b1f752",
    ),
    (
        "Netlists/Certification_Tests/BUG_48_SON/tags",
        46,
        "a725c65e1fba0d3241656fedb9caf217f464e8a2b9e14b21bf823f43f83c1047",
        "4933a5eed19ded7b1fe188959c216a5d1d7ab77b700e72aeb6fc8ae583e27923",
    ),
    (
        XYCE_BUG48_PATH,
        168,
        "7c3361111e5e1687568aa8925623c90ac67093bc24c9942fceadf7a284f96f9e",
        "2884f341df19d6adb50604ca6186f337f1a1c52f5ae1682245c005e7d48bcbdc",
    ),
    (
        "Netlists/Certification_Tests/BUG_48_SON/test.cir.sh",
        1_172,
        "32e11b81f2e9d456ee862afcab7d6be1582ad1b260b4f14d2761d62794766683",
        "d741a973bd78501657a6ffecf54ce0c3c5660612e2704cf96bd860421f667312",
    ),
    (
        "TestScripts/XyceRegression/Tools.pm",
        68_108,
        "5b5f86c02d46a1f3bdad5292e7e91d25a9e08e71490643d8d5ed7ae20f9d55e3",
        "13bd274632744ddc4b8baee680ddc9770902793ed7ee892ecdedd4dcb3828667",
    ),
];
const XYCE_BUG48_RETAINED_ARTIFACTS: [(&str, usize, &str, &str); 2] = [
    (
        "README",
        109,
        "3d6c753db8e5ed4fab8a8be91b462203b8cc248ab23e2abd029c69f41aee0bde",
        "10303ccc21e8d89817bc3044a71bb48c33e2d4c11934fa188b659b25e1b1f752",
    ),
    (
        "test.cir",
        168,
        "7c3361111e5e1687568aa8925623c90ac67093bc24c9942fceadf7a284f96f9e",
        "2884f341df19d6adb50604ca6186f337f1a1c52f5ae1682245c005e7d48bcbdc",
    ),
];

// BUG_159 is a removed-shell relational oracle. The empty owner deck selects
// a wrapper that runs the two real worker decks in order and byte-diffs their
// nonempty default PRN output. The only intended semantic difference is an
// explicit BJT model TNOM=27 C versus the same omitted default.
const XYCE_BUG159_CONTRACT: &str = "bug159_bjt_tnom_default_equivalence_wrapper";
const XYCE_BUG159_OWNER_PATH: &str = "Netlists/Certification_Tests/BUG_159/bug_159.cir";
const XYCE_BUG159_EXPLICIT_PATH: &str = "Netlists/Certification_Tests/BUG_159/bug_159_1.cir";
const XYCE_BUG159_IMPLICIT_PATH: &str = "Netlists/Certification_Tests/BUG_159/bug_159_2.cir";
const XYCE_BUG159_OWNER_RECORD: &str = "netlists/certification_tests/bug_159/bug_159.cir";
const XYCE_BUG159_EXPLICIT_RECORD: &str = "netlists/certification_tests/bug_159/bug_159_1.cir";
const XYCE_BUG159_IMPLICIT_RECORD: &str = "netlists/certification_tests/bug_159/bug_159_2.cir";
const XYCE_BUG159_EXCLUSION_SOURCE: &str = "Netlists/Certification_Tests/BUG_159/exclude";
const XYCE_BUG159_UPSTREAM_REGRESSION_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const XYCE_BUG159_UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const XYCE_BUG159_HISTORICAL_RECORD_COUNT: usize = 12;
const XYCE_BUG159_HISTORICAL_RECORD_BYTES: usize = 2_864;
const XYCE_BUG159_HISTORICAL_RECORDS_SHA256: &str =
    "793a771a9dfc706533d109d3292e45142ab3213b7d4480d1313a7ba06819378a";
const XYCE_BUG159_HISTORICAL_RECORDS_BLAKE3: &str =
    "569cdcdb0a3bd2f236246e0b6184cc01e54dd54ae78b13bf99534c2664b00eca";
const XYCE_BUG159_HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 12] = [
    (
        "Netlists/Certification_Tests/BUG_159/CMakeLists.txt",
        1_551,
        "0536246875f0f282a02d7ba1a71489c848c84ac7c4875c1f894e97af0c3eebda",
        "d1b944849c0b2af29ca578ab3b3d234d6af0fe652e579f01ec2f9e337e878602",
    ),
    (
        "Netlists/Certification_Tests/BUG_159/Manifest.txt",
        75,
        "5dd8bbb4fd2eeded4c316fe5001cf385e2190b89dad2be4ed8b2782c9e8662bf",
        "0d583ff0b7679e7e95f29413acdf6c386ce5925ba95a34ed3ac9c61729c5b940",
    ),
    (
        "Netlists/Certification_Tests/BUG_159/README",
        979,
        "5d6116dd296862775d9e4d300ef9d7c566d0e789c1abfcbf560f7f99da228043",
        "c9cfa060797b9c6bc0b8e959b4c05e4dc6b93727a1c6a0b2ee09f374467e70e3",
    ),
    (
        XYCE_BUG159_OWNER_PATH,
        0,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262",
    ),
    (
        "Netlists/Certification_Tests/BUG_159/bug_159.cir.sh",
        1_372,
        "0be49b3e814edf2792bbe74af877d25e67648053ecff987826b806ba16f0fc8d",
        "e3dcd59e70656e4613938f1f7d5d8543120de04a09c71bd39a68e3c48a4bd8fd",
    ),
    (
        XYCE_BUG159_EXPLICIT_PATH,
        676,
        "838fd5c7594c56a5f57453bb9525a034b5bb34063200b562dce410d1b8c4fabe",
        "b831d59a60fef20cddbd91393bdf422eafd9d9f775b009bef2c0e01517ea39e4",
    ),
    (
        XYCE_BUG159_IMPLICIT_PATH,
        667,
        "c81fe2dfd5ad56947dc9605afa0911e90e6dd2e3b85ac9d6bb65fd426883beea",
        "0c136fb6399d922fb88efea349b5989db7b1ea7739d988ee7e0c8563d1653988",
    ),
    (
        "Netlists/Certification_Tests/BUG_159/exclude",
        28,
        "d1312e057f924f59537f82da2b137352b9493e643890379797c92a483d78e309",
        "a87fc1e2a9df29c7fc653c0cbbf6d70eebde6e687401fcfca135bfe2be962293",
    ),
    (
        "Netlists/Certification_Tests/BUG_159/tags",
        16,
        "fb8b1ab6aa8b694212335a76b1b87c077f22be7543f15c12de32a2da40b4f345",
        "a5f2cee6f41471429bc22c4c40d36881f4c11d2387b20adbdc14efe2509f6589",
    ),
    (
        "OutputData/Certification_Tests/BUG_159/bug_159_1.cir.prn",
        213,
        "ab4c34ff74ac0e474cf7d953e6533e51763f2776b345d64e8985fe871d9389d9",
        "0e3ef59d22062c90b8ea6a0786fb4f58f6cda0c35f6ff0b1c3642ec1ca966a69",
    ),
    (
        "OutputData/Certification_Tests/BUG_159/bug_159_2.cir.prn",
        213,
        "ab4c34ff74ac0e474cf7d953e6533e51763f2776b345d64e8985fe871d9389d9",
        "0e3ef59d22062c90b8ea6a0786fb4f58f6cda0c35f6ff0b1c3642ec1ca966a69",
    ),
    (
        "TestScripts/XyceRegression/Tools.pm",
        68_108,
        "5b5f86c02d46a1f3bdad5292e7e91d25a9e08e71490643d8d5ed7ae20f9d55e3",
        "13bd274632744ddc4b8baee680ddc9770902793ed7ee892ecdedd4dcb3828667",
    ),
];
const XYCE_BUG159_RETAINED_ARTIFACTS: [(&str, usize, &str, &str); 4] = [
    (
        "README",
        979,
        "5d6116dd296862775d9e4d300ef9d7c566d0e789c1abfcbf560f7f99da228043",
        "c9cfa060797b9c6bc0b8e959b4c05e4dc6b93727a1c6a0b2ee09f374467e70e3",
    ),
    (
        "bug_159.cir",
        0,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262",
    ),
    (
        "bug_159_1.cir",
        676,
        "838fd5c7594c56a5f57453bb9525a034b5bb34063200b562dce410d1b8c4fabe",
        "b831d59a60fef20cddbd91393bdf422eafd9d9f775b009bef2c0e01517ea39e4",
    ),
    (
        "bug_159_2.cir",
        667,
        "c81fe2dfd5ad56947dc9605afa0911e90e6dd2e3b85ac9d6bb65fd426883beea",
        "0c136fb6399d922fb88efea349b5989db7b1ea7739d988ee7e0c8563d1653988",
    ),
];
const XYCE_BUG159_RETAINED_OUTPUTS: [(&str, usize, &str, &str); 2] = [
    (
        "bug_159_1.cir.prn",
        213,
        "ab4c34ff74ac0e474cf7d953e6533e51763f2776b345d64e8985fe871d9389d9",
        "0e3ef59d22062c90b8ea6a0786fb4f58f6cda0c35f6ff0b1c3642ec1ca966a69",
    ),
    (
        "bug_159_2.cir.prn",
        213,
        "ab4c34ff74ac0e474cf7d953e6533e51763f2776b345d64e8985fe871d9389d9",
        "0e3ef59d22062c90b8ea6a0786fb4f58f6cda0c35f6ff0b1c3642ec1ca966a69",
    ),
];

// BUG_267 is an include-backed global-parameter success regression. The
// removed shell wrapper only treats a nonzero simulator exit as failure (its
// final missing-PRN branch accidentally returns success), and supplies no
// numerical gold. RSpice preserves that success contract while strengthening
// it with the exact typed include/parameter graph and analytic six-point DC
// response.
const XYCE_BUG267_CONTRACT: &str = "bug267_global_parameter_include_success_wrapper";
const XYCE_BUG267_PATH: &str = "Netlists/Certification_Tests/BUG_267/bug267.cir";
const XYCE_BUG267_RECORD: &str = "netlists/certification_tests/bug_267/bug267.cir";
const XYCE_BUG267_UPSTREAM_REGRESSION_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const XYCE_BUG267_UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const XYCE_BUG267_HISTORICAL_RECORD_COUNT: usize = 7;
const XYCE_BUG267_HISTORICAL_RECORD_BYTES: usize = 1_661;
const XYCE_BUG267_HISTORICAL_RECORDS_SHA256: &str =
    "6f44b1e7c6895c933c71040e2c281131c5af5b1b05650597dda78d7df3ef1d07";
const XYCE_BUG267_HISTORICAL_RECORDS_BLAKE3: &str =
    "e2f556e28faf17e5382dfc5f4c43c6441083f20c99ddcfe6eb1b319fbd6148c9";
const XYCE_BUG267_HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 7] = [
    (
        "Netlists/Certification_Tests/BUG_267/CMakeLists.txt",
        1_647,
        "825daed5de9c62722dedfa65f0fbeecc2712a9ee2d5f85cdc71c6f079b204940",
        "0aecc8b1f3a4e84b69928eb45b018f13485955dbe7abdb002d0db99c90e9180d",
    ),
    (
        "Netlists/Certification_Tests/BUG_267/Manifest.txt",
        44,
        "9586977dd9baebc7e88f810409e469706cb55e32078eaca9da142f1382bfd67b",
        "bf2e982998532ae10e5389b0f0240cdc54646fda89cfdae37d62231d8b7fd0b7",
    ),
    (
        "Netlists/Certification_Tests/BUG_267/analysis.cmds",
        42,
        "3b6f9f91c14b60d6ad5fdbd757711695dd0b3e77a63f1fccbd08e5ed15a55d9c",
        "0f7d1b0ff3e6b9240cd05addc2397140a3887cd6a39ee671c35ca32490b8dbf6",
    ),
    (
        XYCE_BUG267_PATH,
        1_401,
        "3748075f2831433673802f5d46aa76b2d9a8764b6b295ca2161cbff06c72ef86",
        "696265490951770a677336288998054fadc57ca8c76f559037cca7251a96a494",
    ),
    (
        "Netlists/Certification_Tests/BUG_267/bug267.cir.sh",
        1_057,
        "37118e725cb539b00f1a6a21464e8af0b966d8ff9cf2a4e3b617a1af481cc3fd",
        "dede22cb8c7c633b47730afa2088dd5ec1c9fe5fde7f561eec5dc2be26d2bb2c",
    ),
    (
        "Netlists/Certification_Tests/BUG_267/tags",
        28,
        "43901b249a3892a461ba23bc3dc95c74bdece2bfaee8f7b6e35d9f22359893ac",
        "2deb82428ebb81c18d9fb719f046d5b461735881ab0d03644891ca3389f06420",
    ),
    // The direct shell wrapper does not source Perl, but the release
    // regression framework still owns its process/result envelope. Keep the
    // same framework-provenance binding as adjacent reconstructed wrappers.
    (
        "TestScripts/XyceRegression/Tools.pm",
        68_108,
        "5b5f86c02d46a1f3bdad5292e7e91d25a9e08e71490643d8d5ed7ae20f9d55e3",
        "13bd274632744ddc4b8baee680ddc9770902793ed7ee892ecdedd4dcb3828667",
    ),
];
const XYCE_BUG267_RETAINED_ARTIFACTS: [(&str, usize, &str, &str); 2] = [
    (
        "analysis.cmds",
        42,
        "3b6f9f91c14b60d6ad5fdbd757711695dd0b3e77a63f1fccbd08e5ed15a55d9c",
        "0f7d1b0ff3e6b9240cd05addc2397140a3887cd6a39ee671c35ca32490b8dbf6",
    ),
    (
        "bug267.cir",
        1_401,
        "3748075f2831433673802f5d46aa76b2d9a8764b6b295ca2161cbff06c72ef86",
        "696265490951770a677336288998054fadc57ca8c76f559037cca7251a96a494",
    ),
];

// BUG_302 is an output-formatting wrapper. Its zero-byte owner runs two
// four-member DC/TRAN cohorts and proves that COMMA and TAB are exact textual
// transformations of default PRN output. Invalid delimiters retain the
// default layout and emit Xyce's historical warning. The eight workers remain
// independently qualified by their existing numerical contracts; only the
// removed wrapper owner is reconstructed here.
const XYCE_BUG302_CONTRACT: &str = "bug302_print_delimiter_relational_wrapper";
const XYCE_BUG302_PATH: &str = "Netlists/Certification_Tests/BUG_302/bug_302.cir";
const XYCE_BUG302_RECORD: &str = "netlists/certification_tests/bug_302/bug_302.cir";
const XYCE_BUG302_EXCLUSION_SOURCE: &str = "Netlists/Certification_Tests/BUG_302/exclude";
const XYCE_BUG302_UPSTREAM_REGRESSION_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const XYCE_BUG302_UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const XYCE_BUG302_HISTORICAL_RECORD_COUNT: usize = 24;
const XYCE_BUG302_HISTORICAL_RECORD_BYTES: usize = 5_903;
const XYCE_BUG302_HISTORICAL_RECORDS_SHA256: &str =
    "f44cf8622021c04f16698e1bde928e0eba720f3203f877f1c3378dd3dbabb2ec";
const XYCE_BUG302_HISTORICAL_RECORDS_BLAKE3: &str =
    "cb83a9ff9d437c3a1d90fb713ce282bb09b3f5505344504c9e422c67babc90c6";
const XYCE_BUG302_HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 24] = [
    (
        "Netlists/Certification_Tests/BUG_302/CMakeLists.txt",
        2_409,
        "b7e76dc46d95b8770f1dd38c48bc06cc6069e2d8a64acbb5789b4e9abf08bb06",
        "c56d738bf5a1e694025f4be75b1ed8c7fab804aaa143413b45a11958ff1ad025",
    ),
    (
        "Netlists/Certification_Tests/BUG_302/DC_comma.cir",
        1_361,
        "576fd732409aef8363f1decfadddf636fa1b525922a3f6dd1820d9e2dc040f3d",
        "1b5910ba8cbaa523ce8aeb7f114a5fe993433c9779320ae3e8f2201b3f096f3e",
    ),
    (
        "Netlists/Certification_Tests/BUG_302/DC_defaults.cir",
        1_297,
        "00a7314ed5201c2ac0786ee396d86277de0ef3fe52f402e21dec5351a9816e60",
        "4c031996066b6e45e06a77a4655d37e94752c7008617c68b602fd25222c80308",
    ),
    (
        "Netlists/Certification_Tests/BUG_302/DC_delimiter_invalid.cir",
        1_389,
        "579411aaab9b91e298a878bd95ba94d457807f574249722f048548083b100aaf",
        "f93d00eac0f6a4b6385d0f55c82c8cc56e62edc20d2dc3992750b8fea069a4a2",
    ),
    (
        "Netlists/Certification_Tests/BUG_302/DC_tab.cir",
        1_357,
        "a6f7eafbf54aea6ad9a773c304daf288ffb5ad0e7e044bb09526209f4f05c59c",
        "ed8dfc2890c36d76561137a22f340dd259b1947fe306d94353e38215b82f8d8e",
    ),
    (
        "Netlists/Certification_Tests/BUG_302/Manifest.txt",
        205,
        "35bf395fa34c0d71654c1c7fac274c257b017e33421fb03115c33c0add284cfe",
        "6265882d882622cdaa3dbdd2190f10dddeb54f788d59db06d2fba57c86986aaf",
    ),
    (
        "Netlists/Certification_Tests/BUG_302/README",
        2_545,
        "8728ead6436c00599afe301c5f4deae92f336d4808f8b3c166a11c2a864ee3e0",
        "cb4aa630012d2814cd24856a67a8e7e478f309b6f2c9b47a0390849f4e350700",
    ),
    (
        XYCE_BUG302_PATH,
        0,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262",
    ),
    (
        "Netlists/Certification_Tests/BUG_302/bug_302.cir.sh",
        5_529,
        "1048df29a91bae4bb6d2241d2ad095f246072316573c141d933e307c313a6fdd",
        "969733bc7a34e752df9eab7f8ed78d36e6f5978f9b7c35e5265b53a4b75c6c1a",
    ),
    (
        "Netlists/Certification_Tests/BUG_302/exclude",
        199,
        "e93976af27802fe2548ba96016c341d35febaf64d476001a0bb4f8a3bfbbc702",
        "e570ab7eed43a3a5e838bcda614ee90d0d3a8848a30e62e897c8956a4fe2b457",
    ),
    (
        "Netlists/Certification_Tests/BUG_302/tags",
        46,
        "d4dfec2dad975e8cf478d3f74ed8d5cf54efc2787671fbbab41f4e3ad7e73e06",
        "22d1263551ed12233d6e6dd7fe27e20768b742f997f04b45cd9e2eb7294bfbf9",
    ),
    (
        "Netlists/Certification_Tests/BUG_302/transient_comma.cir",
        362,
        "333bf43c003628e28b474fca7ce0faff32439b4030acb05efb2208b1f665f52c",
        "9d4033979f95656d878feca50f34686ca471b7d60a84a9cd79766991c989417a",
    ),
    (
        "Netlists/Certification_Tests/BUG_302/transient_defaults.cir",
        271,
        "c0b62f4babac639a79a7168451adb974dd8d33dc2bf34701e01a8062d346a21e",
        "1724f72e22aba6c57fcfb598c26741b818abf4708d84b8a13bbedaa29a1b7616",
    ),
    (
        "Netlists/Certification_Tests/BUG_302/transient_delimiter_invalid.cir",
        362,
        "a35072f2fcfab6bc8a7fddf39313d3ae7ef55a7b3604687ecfb467e893618d9d",
        "5c4325f6ba74c7517cd31b6668798c3537dd48788f5907ab440d4c7840c249f0",
    ),
    (
        "Netlists/Certification_Tests/BUG_302/transient_tab.cir",
        360,
        "9904be03558e7abf27fe03da8eefc51abc66fb9b11cff7aad41c330ea868e7b3",
        "cbe63f1ac2be529a771bdf396834e56fe2aa94fb88a3a688c16f2db41791ea35",
    ),
    (
        "OutputData/Certification_Tests/BUG_302/DC_comma.cir.prn",
        28_334,
        "b8765663f204ccb112c1242be41a29371bf00b65346e6eb0f4313a2d8cff1531",
        "5afe7b575153389589a8e34d7ce60b17dd03654ad9b07fabb6fe398a146f11ba",
    ),
    (
        "OutputData/Certification_Tests/BUG_302/DC_defaults.cir.prn",
        34_163,
        "02f63a35008a7d4decdf8cb19a52ec85ec006bd9bb3fac16b89d88b10adb5a6b",
        "ef47fbc7a31d8b6d6de2f5ae1195f933a6f7a701cbbf8a051e4bfb029afbf8bf",
    ),
    (
        "OutputData/Certification_Tests/BUG_302/DC_delimiter_invalid.cir.prn",
        34_163,
        "02f63a35008a7d4decdf8cb19a52ec85ec006bd9bb3fac16b89d88b10adb5a6b",
        "ef47fbc7a31d8b6d6de2f5ae1195f933a6f7a701cbbf8a051e4bfb029afbf8bf",
    ),
    (
        "OutputData/Certification_Tests/BUG_302/DC_tab.cir.prn",
        28_334,
        "865acf8210101456644a958268946dfc88c4c1c45785ada9ec2d9c674cbe2e8b",
        "e91b4f886bb217b0609d1a8a910b4de641fcf61d9ab1f6a9090b76b4240b83bc",
    ),
    (
        "OutputData/Certification_Tests/BUG_302/transient_comma.cir.prn",
        2_277,
        "663590ebb3db66ae99e4df223558acef7a808ee29b6afccbca6db236b2bc791a",
        "55629f198cb511bbcfef12d61d7c999b713e7cc979903a91ef27e301f13ebe06",
    ),
    (
        "OutputData/Certification_Tests/BUG_302/transient_defaults.cir.prn",
        2_982,
        "ae915e93d317ba29fb3db575621939138536b572d8361d2847d147f56c903732",
        "d4736c97adc60bc5cf79c6bc7157a0168cad22e10a7da1696bcaa36f521707ce",
    ),
    (
        "OutputData/Certification_Tests/BUG_302/transient_delimiter_invalid.cir.prn",
        2_982,
        "ae915e93d317ba29fb3db575621939138536b572d8361d2847d147f56c903732",
        "d4736c97adc60bc5cf79c6bc7157a0168cad22e10a7da1696bcaa36f521707ce",
    ),
    (
        "OutputData/Certification_Tests/BUG_302/transient_tab.cir.prn",
        2_277,
        "1cbbd899838c8fccdc351431e85f3e8508a547a5c0aeae11cfffa586e98e0b5e",
        "15ae9ab1fe2680cef41d9aabe64f421256e7653247cb3c64b3857626a47f7319",
    ),
    (
        "TestScripts/XyceRegression/Tools.pm",
        68_108,
        "5b5f86c02d46a1f3bdad5292e7e91d25a9e08e71490643d8d5ed7ae20f9d55e3",
        "13bd274632744ddc4b8baee680ddc9770902793ed7ee892ecdedd4dcb3828667",
    ),
];
const XYCE_BUG302_RETAINED_SOURCES: [(&str, usize, &str, &str); 10] = [
    (
        "DC_comma.cir",
        1_361,
        "576fd732409aef8363f1decfadddf636fa1b525922a3f6dd1820d9e2dc040f3d",
        "1b5910ba8cbaa523ce8aeb7f114a5fe993433c9779320ae3e8f2201b3f096f3e",
    ),
    (
        "DC_defaults.cir",
        1_297,
        "00a7314ed5201c2ac0786ee396d86277de0ef3fe52f402e21dec5351a9816e60",
        "4c031996066b6e45e06a77a4655d37e94752c7008617c68b602fd25222c80308",
    ),
    (
        "DC_delimiter_invalid.cir",
        1_389,
        "579411aaab9b91e298a878bd95ba94d457807f574249722f048548083b100aaf",
        "f93d00eac0f6a4b6385d0f55c82c8cc56e62edc20d2dc3992750b8fea069a4a2",
    ),
    (
        "DC_tab.cir",
        1_357,
        "a6f7eafbf54aea6ad9a773c304daf288ffb5ad0e7e044bb09526209f4f05c59c",
        "ed8dfc2890c36d76561137a22f340dd259b1947fe306d94353e38215b82f8d8e",
    ),
    (
        "README",
        2_545,
        "8728ead6436c00599afe301c5f4deae92f336d4808f8b3c166a11c2a864ee3e0",
        "cb4aa630012d2814cd24856a67a8e7e478f309b6f2c9b47a0390849f4e350700",
    ),
    (
        "bug_302.cir",
        0,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262",
    ),
    (
        "transient_comma.cir",
        362,
        "333bf43c003628e28b474fca7ce0faff32439b4030acb05efb2208b1f665f52c",
        "9d4033979f95656d878feca50f34686ca471b7d60a84a9cd79766991c989417a",
    ),
    (
        "transient_defaults.cir",
        271,
        "c0b62f4babac639a79a7168451adb974dd8d33dc2bf34701e01a8062d346a21e",
        "1724f72e22aba6c57fcfb598c26741b818abf4708d84b8a13bbedaa29a1b7616",
    ),
    (
        "transient_delimiter_invalid.cir",
        362,
        "a35072f2fcfab6bc8a7fddf39313d3ae7ef55a7b3604687ecfb467e893618d9d",
        "5c4325f6ba74c7517cd31b6668798c3537dd48788f5907ab440d4c7840c249f0",
    ),
    (
        "transient_tab.cir",
        360,
        "9904be03558e7abf27fe03da8eefc51abc66fb9b11cff7aad41c330ea868e7b3",
        "cbe63f1ac2be529a771bdf396834e56fe2aa94fb88a3a688c16f2db41791ea35",
    ),
];
const XYCE_BUG302_RETAINED_OUTPUTS: [(&str, usize, &str, &str); 8] = [
    (
        "DC_comma.cir.prn",
        28_334,
        "b8765663f204ccb112c1242be41a29371bf00b65346e6eb0f4313a2d8cff1531",
        "5afe7b575153389589a8e34d7ce60b17dd03654ad9b07fabb6fe398a146f11ba",
    ),
    (
        "DC_defaults.cir.prn",
        34_163,
        "02f63a35008a7d4decdf8cb19a52ec85ec006bd9bb3fac16b89d88b10adb5a6b",
        "ef47fbc7a31d8b6d6de2f5ae1195f933a6f7a701cbbf8a051e4bfb029afbf8bf",
    ),
    (
        "DC_delimiter_invalid.cir.prn",
        34_163,
        "02f63a35008a7d4decdf8cb19a52ec85ec006bd9bb3fac16b89d88b10adb5a6b",
        "ef47fbc7a31d8b6d6de2f5ae1195f933a6f7a701cbbf8a051e4bfb029afbf8bf",
    ),
    (
        "DC_tab.cir.prn",
        28_334,
        "865acf8210101456644a958268946dfc88c4c1c45785ada9ec2d9c674cbe2e8b",
        "e91b4f886bb217b0609d1a8a910b4de641fcf61d9ab1f6a9090b76b4240b83bc",
    ),
    (
        "transient_comma.cir.prn",
        2_277,
        "663590ebb3db66ae99e4df223558acef7a808ee29b6afccbca6db236b2bc791a",
        "55629f198cb511bbcfef12d61d7c999b713e7cc979903a91ef27e301f13ebe06",
    ),
    (
        "transient_defaults.cir.prn",
        2_982,
        "ae915e93d317ba29fb3db575621939138536b572d8361d2847d147f56c903732",
        "d4736c97adc60bc5cf79c6bc7157a0168cad22e10a7da1696bcaa36f521707ce",
    ),
    (
        "transient_delimiter_invalid.cir.prn",
        2_982,
        "ae915e93d317ba29fb3db575621939138536b572d8361d2847d147f56c903732",
        "d4736c97adc60bc5cf79c6bc7157a0168cad22e10a7da1696bcaa36f521707ce",
    ),
    (
        "transient_tab.cir.prn",
        2_277,
        "1cbbd899838c8fccdc351431e85f3e8508a547a5c0aeae11cfffa586e98e0b5e",
        "15ae9ab1fe2680cef41d9aabe64f421256e7653247cb3c64b3857626a47f7319",
    ),
];

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
const XYCE_BUG352_EXCLUSION_SOURCE: &str = "Netlists/Certification_Tests/BUG_352/exclude";
const XYCE_BUG352_UPSTREAM_REGRESSION_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const XYCE_BUG352_UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const XYCE_BUG352_HISTORICAL_RECORD_COUNT: usize = 9;
const XYCE_BUG352_HISTORICAL_RECORD_BYTES: usize = 2_132;
const XYCE_BUG352_HISTORICAL_RECORDS_SHA256: &str =
    "877197b218aecdd03eb4eec89cfffebd4dcc4de480425f210d613fd8f233e6df";
const XYCE_BUG352_HISTORICAL_RECORDS_BLAKE3: &str =
    "2841b048fb8a0b1ae78571ecc8a0f341013fcc9491e3b2c97f4639792072b502";
const XYCE_BUG352_HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 9] = [
    (
        "Netlists/Certification_Tests/BUG_352/CMakeLists.txt",
        1_947,
        "bcc6b130ed1ee65220b92581af3ff6d771ef5f4f90db300cc2404d0f014d4665",
        "04424eacdcdef62b81f291869c5dd8f0f9777811b2f8ef33036a653d3b15bf08",
    ),
    (
        "Netlists/Certification_Tests/BUG_352/Manifest.txt",
        62,
        "2577f06f92166598a4199ff6328bd69d13100207a7631823a13ad83d36dcbd7b",
        "835ad44ed1a5672564a40367742f753a7593b2fd7fedae92e29a58e98936209a",
    ),
    (
        "Netlists/Certification_Tests/BUG_352/README",
        831,
        "adea694684c225d8cc84527b21205708c8891709f98e71838599920e1537884c",
        "51b8ed0ed86354d73459f4220498fdc8a6c6f8eb25d0f06ba6f6a932bb187f1b",
    ),
    (
        XYCE_BUG352_OWNER_PATH,
        161,
        "26b8ffcee2b2e3c8f667a58d7c19c3aed5e8afcdc973a5ca905b52eab8fa12b1",
        "f3a709f923870e76226ce430323725128d241c6e11b4259a6c364d6b50fafe06",
    ),
    (
        "Netlists/Certification_Tests/BUG_352/BUG_352a.cir.sh",
        1_412,
        "33f0dba1f3201c9899c59116118c153c74ec7c892b8dc5cace6fb9aa1fdd9774",
        "070d5e90b85909d4c3aa71ee4b4824cf9804eaaed3a64aaa323673d89cec7a73",
    ),
    (
        XYCE_BUG352_CONTROL_PATH,
        130,
        "7693247e71be35c0eae15bce8dbc1187663db39d3981c554454f93a8a250b215",
        "9d3f638aa4df5e86c4b29243219b8f1a6af89019419ebc2c61728b8653a12beb",
    ),
    (
        "Netlists/Certification_Tests/BUG_352/exclude",
        41,
        "f4cb80e294a2e917da749acd1e3cf26c44b7d3b420f6907c4700574346d9f065",
        "d171440460d8afd9c65f336b91d6f3475c015a9d6a3dbb85ec63e13e395e9e3f",
    ),
    (
        "Netlists/Certification_Tests/BUG_352/tags",
        28,
        "43901b249a3892a461ba23bc3dc95c74bdece2bfaee8f7b6e35d9f22359893ac",
        "2deb82428ebb81c18d9fb719f046d5b461735881ab0d03644891ca3389f06420",
    ),
    (
        "TestScripts/XyceRegression/Tools.pm",
        68_108,
        "5b5f86c02d46a1f3bdad5292e7e91d25a9e08e71490643d8d5ed7ae20f9d55e3",
        "13bd274632744ddc4b8baee680ddc9770902793ed7ee892ecdedd4dcb3828667",
    ),
];
const XYCE_BUG352_RETAINED_ARTIFACTS: [(&str, usize, &str, &str); 3] = [
    (
        "BUG_352a.cir",
        161,
        "26b8ffcee2b2e3c8f667a58d7c19c3aed5e8afcdc973a5ca905b52eab8fa12b1",
        "f3a709f923870e76226ce430323725128d241c6e11b4259a6c364d6b50fafe06",
    ),
    (
        "BUG_352b.cir",
        130,
        "7693247e71be35c0eae15bce8dbc1187663db39d3981c554454f93a8a250b215",
        "9d3f638aa4df5e86c4b29243219b8f1a6af89019419ebc2c61728b8653a12beb",
    ),
    (
        "README",
        831,
        "adea694684c225d8cc84527b21205708c8891709f98e71838599920e1537884c",
        "51b8ed0ed86354d73459f4220498fdc8a6c6f8eb25d0f06ba6f6a932bb187f1b",
    ),
];

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
const XYCE_BUG1797_EXCLUSION_SOURCE: &str = "Netlists/Certification_Tests/BUG_1797/exclude";
const XYCE_BUG1797_UPSTREAM_REGRESSION_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const XYCE_BUG1797_UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const XYCE_BUG1797_HISTORICAL_RECORD_COUNT: usize = 10;
const XYCE_BUG1797_HISTORICAL_RECORD_BYTES: usize = 2_381;
const XYCE_BUG1797_HISTORICAL_RECORDS_SHA256: &str =
    "ef09bc3deff030161eba7e51c697c3bda30bea107f56602518cdd2ea70e849dc";
const XYCE_BUG1797_HISTORICAL_RECORDS_BLAKE3: &str =
    "515dd9dbb60e3a648bbeddb25857440344958c7c05876750189941c55cd4493d";
const XYCE_BUG1797_HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 10] = [
    (
        "Netlists/Certification_Tests/BUG_1797/CMakeLists.txt",
        2_057,
        "a7f8cb3fbb63c4e2e5a6004e5a16e5ea945506adf830e7383ddd6b9e80fa757a",
        "b5d811ac2a3f0b4833877798547ab41b3e164897e4cfe85d2483c0087a6cf99e",
    ),
    (
        "Netlists/Certification_Tests/BUG_1797/Manifest.txt",
        86,
        "feae63a70c21630b0ddc8cd9ec3d32392350e2abf818161baa7c8f1992d632fd",
        "77af32a6824cd25fcfa6b7801ecfec77950db7c3827c15409a831b699c3ea890",
    ),
    (
        "Netlists/Certification_Tests/BUG_1797/README",
        291,
        "8d7bbcfee5d564732d82fafef0ede4c44807e5766b90154c3fabfc6cfaeb59a7",
        "9b9bea05458b7c2862cd00179b2421459e1f75ac7fe6d4d91a2efa871a158001",
    ),
    (
        "Netlists/Certification_Tests/BUG_1797/exclude",
        37,
        "b060e161f951de612c91474529912e6c84137a9d38a57251acda99bf6ef65bb0",
        "5fd44ea4650bb66c3cde2a8b42403fd68c7fdd4d7e31d1b46f17f612f78d59a7",
    ),
    (
        XYCE_BUG1797_OWNER_PATH,
        1,
        "01ba4719c80b6fe911b091a7c05124b64eeece964e09c058ef8f9805daca546b",
        "295192ea1ec8566d563b1a7587e5f0198580cdbd043842f5090a4c197c20c67a",
    ),
    (
        "Netlists/Certification_Tests/BUG_1797/one-shot.cir.sh",
        1_432,
        "77346929dc86a41166bc8ee079e8fcd85898cf3aed67e9afbace0fc348af2d11",
        "d325eb36ac25e1b904cf42cd1791af83e7f8eb5e44aadf5279a9a30867629cdf",
    ),
    (
        XYCE_BUG1797_LEVEL49_PATH,
        997,
        "c2a82d25b6774959fe22f1001c64ecce18e9571cf2de1ae50c0a296193587b3b",
        "db72727f63d8216f23c3ac2971df52c66e7011bb5471d9739d8132e800f4b65a",
    ),
    (
        XYCE_BUG1797_LEVEL9_PATH,
        995,
        "a174dc0f6c68d3979407b739a1f97e457f37bb3ee3ea3699ede31733639a025f",
        "aa3e56fb3775d321962cb1b702bea44b85428d8508cb35e326c9fd1540dc1f2f",
    ),
    (
        "Netlists/Certification_Tests/BUG_1797/tags",
        43,
        "50662c44729f3312a23d4a634e1786ad5de7aec963cc0a1961dd60ad65e10fd8",
        "c24cf5925bdd2de8cf8d81be30f0d6e1490f1af67ed08ad54a6195cdf3c7bc77",
    ),
    (
        "TestScripts/file_compare.pl",
        7_465,
        "a700143baddab265ca2e74d69541432fb27ae66600c3fee71968797fc78efcb0",
        "04dd69b4e4cfe543a39f663966229be877fa595a7c6c885dadf2173814f85895",
    ),
];
const XYCE_BUG1797_RETAINED_ARTIFACTS: [(&str, usize, &str, &str); 4] = [
    (
        "README",
        291,
        "8d7bbcfee5d564732d82fafef0ede4c44807e5766b90154c3fabfc6cfaeb59a7",
        "9b9bea05458b7c2862cd00179b2421459e1f75ac7fe6d4d91a2efa871a158001",
    ),
    (
        "one-shot.cir",
        1,
        "01ba4719c80b6fe911b091a7c05124b64eeece964e09c058ef8f9805daca546b",
        "295192ea1ec8566d563b1a7587e5f0198580cdbd043842f5090a4c197c20c67a",
    ),
    (
        "one-shot_lev49.cir",
        997,
        "c2a82d25b6774959fe22f1001c64ecce18e9571cf2de1ae50c0a296193587b3b",
        "db72727f63d8216f23c3ac2971df52c66e7011bb5471d9739d8132e800f4b65a",
    ),
    (
        "one-shot_lev9.cir",
        995,
        "a174dc0f6c68d3979407b739a1f97e457f37bb3ee3ea3699ede31733639a025f",
        "aa3e56fb3775d321962cb1b702bea44b85428d8508cb35e326c9fd1540dc1f2f",
    ),
];

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
const XYCE_BUG981_EXCLUSION_SOURCE: &str = "Netlists/Certification_Tests/BUG_981_SON/exclude";
const XYCE_BUG981_UPSTREAM_REGRESSION_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const XYCE_BUG981_UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const XYCE_BUG981_HISTORICAL_RECORD_COUNT: usize = 9;
const XYCE_BUG981_HISTORICAL_RECORD_BYTES: usize = 2_172;
const XYCE_BUG981_HISTORICAL_RECORDS_SHA256: &str =
    "953ce232f72f982e74d1b38afc7cc81c4a2b88cb8cbe5923055ece14123d235e";
const XYCE_BUG981_HISTORICAL_RECORDS_BLAKE3: &str =
    "444122c070972b4828bc78bfe6e5c0183f8b77f0ddf6668ba997bc0a850e905d";
const XYCE_BUG981_HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 9] = [
    (
        "Netlists/Certification_Tests/BUG_981_SON/CMakeLists.txt",
        2_051,
        "6695f9741b0e4887e57da00b232d9c8611410449f27c1dbe77a9c5f3f4e739ba",
        "ae7c5165978b5738e016b1b3e81bc81e6da62daa178f3e4ceb592aa6fc23b956",
    ),
    (
        "Netlists/Certification_Tests/BUG_981_SON/Manifest.txt",
        70,
        "dc0dcd6feb371fc89216251da8280c45db48ca49060a1b93ce4741ca75ed83d0",
        "56b03dafaa2450c84d1af760ad50ca37da989a509ecd3a2f4c72c412ebbe9cee",
    ),
    (
        "Netlists/Certification_Tests/BUG_981_SON/bug981.cir",
        2,
        "75a11da44c802486bc6f65640aa48a730f0f684c5c07a42ba3cd1735eb3fb070",
        "5896d7c81fa3a2eee0aa6139c752d40a1408b7e083aa940d1ece11d61d6c0e3e",
    ),
    (
        "Netlists/Certification_Tests/BUG_981_SON/bug981.cir.sh",
        1_449,
        "c04613a2996e3431f4a996d21069a915a2a765ec9138b8f1955e66016cc8a0b6",
        "b8c76c05bf416d4f3b814d36ca78fbbdca2428d502565a32b513ea3622e55771",
    ),
    (
        "Netlists/Certification_Tests/BUG_981_SON/bug981A.cir",
        186,
        "9c5e7b9d0b8fa9f72299c2f01bcb4c014fef0e60d44f3e0e6ab9558cc7bdcf1c",
        "5eb853d4da315b239b78fdec46cfee5417525c68f46d7473ec8a2519de536661",
    ),
    (
        "Netlists/Certification_Tests/BUG_981_SON/bug981B.cir",
        213,
        "93fb996bfe71b4843ccfc38ddf6bd10c7cacadc12aef95b80fd589a59ab33808",
        "4bb4ccbedb28b0bec5649fd1519f9ff7c56ba43b252a939764695f59922f0fc4",
    ),
    (
        "Netlists/Certification_Tests/BUG_981_SON/exclude",
        24,
        "d79ba3c51570c5736f997e842e596b7dfd5fc479a78c237e1348d69d73fabafc",
        "00eb676a5a94ea5a3641c53bf2b9f412087f9feb7b38a04c5284225497db920e",
    ),
    (
        "Netlists/Certification_Tests/BUG_981_SON/patfile",
        84,
        "11a3883c1c30a398347fe4f28735842f0ebf912cf074c268ddd137257ad0b469",
        "81a1d12cfce1ba83ca9aa3bc678a568daea1e910b50d04a69cbdf2008aaf76d4",
    ),
    (
        "Netlists/Certification_Tests/BUG_981_SON/tags",
        56,
        "ed3e07cb52c9ad1d8a2b3e7d84c440f376470b5e8fc59951d522c788efa2753c",
        "59021529f15846003cb16987fb29255dd14a460c1098f4a6159a4e267178437c",
    ),
];
const XYCE_BUG981_RETAINED_ARTIFACTS: [(&str, usize, &str, &str); 6] = [
    (
        "bug981.cir",
        2,
        "75a11da44c802486bc6f65640aa48a730f0f684c5c07a42ba3cd1735eb3fb070",
        "5896d7c81fa3a2eee0aa6139c752d40a1408b7e083aa940d1ece11d61d6c0e3e",
    ),
    (
        "bug981A.cir",
        186,
        "9c5e7b9d0b8fa9f72299c2f01bcb4c014fef0e60d44f3e0e6ab9558cc7bdcf1c",
        "5eb853d4da315b239b78fdec46cfee5417525c68f46d7473ec8a2519de536661",
    ),
    (
        "bug981A.cir.prn",
        243,
        "b8dad5d75a96853ea1d61c5d4e544aa0de1abbcb7ac8b838141ddfda7f3f2ae0",
        "e0e1d1ff14b3d58cbac8eea7838807e3a86ef57e9c2e4bbc2e86060febf3b36a",
    ),
    (
        "bug981B.cir",
        213,
        "93fb996bfe71b4843ccfc38ddf6bd10c7cacadc12aef95b80fd589a59ab33808",
        "4bb4ccbedb28b0bec5649fd1519f9ff7c56ba43b252a939764695f59922f0fc4",
    ),
    (
        "bug981B.cir.prn",
        2_331,
        "ebbe32d36ec543db48ce81601d0c9d4bff963d5dd38f6c5ba63d0b5e4f011985",
        "77b95e8250eb8411c7c36f353870ed46c381c3ffc9ab0e00f821ec2c6b9c245a",
    ),
    (
        "patfile",
        84,
        "11a3883c1c30a398347fe4f28735842f0ebf912cf074c268ddd137257ad0b469",
        "81a1d12cfce1ba83ca9aa3bc678a568daea1e910b50d04a69cbdf2008aaf76d4",
    ),
];

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
const XYCE_BUG986_EXCLUSION_SOURCE: &str = "Netlists/Certification_Tests/BUG_986_SON/exclude";
const XYCE_BUG986_UPSTREAM_REGRESSION_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const XYCE_BUG986_UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const XYCE_BUG986_HISTORICAL_RECORD_COUNT: usize = 8;
const XYCE_BUG986_HISTORICAL_RECORD_BYTES: usize = 1_934;
const XYCE_BUG986_HISTORICAL_RECORDS_SHA256: &str =
    "ab17281bd2abb81d4861f1c1086ba804ccd062f84ac08f7cc7bd422b2afcd8d2";
const XYCE_BUG986_HISTORICAL_RECORDS_BLAKE3: &str =
    "ab55ef2edc8d8c970acbc3e5f19d01608b525c108e7576ce4b87eea8f5017a2a";
const XYCE_BUG986_HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 8] = [
    (
        "Netlists/Certification_Tests/BUG_986_SON/CMakeLists.txt",
        1_470,
        "777743a645d21c33259fbee1d63c97397ad56fa95412d4e4ce0eb7a90fafc889",
        "7332ae97bab0830c9d178355a37b744b6ce886679d1dda0211951f297ed325e9",
    ),
    (
        "Netlists/Certification_Tests/BUG_986_SON/Manifest.txt",
        62,
        "6457a2bfb84b67da4b9be2ff4f935affaa404159351193f08fc1ca2248d450ba",
        "2b909c87a0eb2ef3b32a538b8e314ec93d6ad0e77159a139ece253216d9d945f",
    ),
    (
        XYCE_BUG986_OWNER_PATH,
        2,
        "75a11da44c802486bc6f65640aa48a730f0f684c5c07a42ba3cd1735eb3fb070",
        "5896d7c81fa3a2eee0aa6139c752d40a1408b7e083aa940d1ece11d61d6c0e3e",
    ),
    (
        "Netlists/Certification_Tests/BUG_986_SON/bug986.cir.sh",
        1_375,
        "503e4da47628c5b25677c49def72b487487730790206809e8dc0626c87ed797f",
        "a6cbeb117bb424779f60d587142138a74c338db7c1a830552dd1b7cc6e5a6579",
    ),
    (
        XYCE_BUG986_BREAKPOINT_PATH,
        181,
        "512d5da724c868f0b9b0053174bee474bbb0b32386c04dc282e358d05d888a0e",
        "03d778c9aff2d1c160f28db66703e9071f79932f6d45a5a58392715c8a170b0f",
    ),
    (
        XYCE_BUG986_PWL_PATH,
        287,
        "b48abba94661edc6bd4d2e8f986cbd4c012537dd1abe7284bda1924cb7f06f99",
        "ca2a2eaf2071ce84b529f425115383ac67f932de263f1be20be664e627f94b6c",
    ),
    (
        "Netlists/Certification_Tests/BUG_986_SON/exclude",
        24,
        "4b29d39d098fcb124047124f11f03a03c266a550d4cffd326048983daefa7c6d",
        "ff428e04828eca85fce217ddda5c580b3e55d135af9a7ef3dcf0b97c5d997ba6",
    ),
    (
        "Netlists/Certification_Tests/BUG_986_SON/tags",
        46,
        "c5cb611c2e3e599f90395aef50b7d59f3aa05f125fea8c02bf1f313854a3abaa",
        "0d83e34418c364531bc0a371b85d66ddcb977725f8be28ca90d2946c2e77661f",
    ),
];
const XYCE_BUG986_RETAINED_ARTIFACTS: [(&str, usize, &str, &str); 3] = [
    (
        "bug986.cir",
        2,
        "75a11da44c802486bc6f65640aa48a730f0f684c5c07a42ba3cd1735eb3fb070",
        "5896d7c81fa3a2eee0aa6139c752d40a1408b7e083aa940d1ece11d61d6c0e3e",
    ),
    (
        "bug986A.cir",
        181,
        "512d5da724c868f0b9b0053174bee474bbb0b32386c04dc282e358d05d888a0e",
        "03d778c9aff2d1c160f28db66703e9071f79932f6d45a5a58392715c8a170b0f",
    ),
    (
        "bug986B.cir",
        287,
        "b48abba94661edc6bd4d2e8f986cbd4c012537dd1abe7284bda1924cb7f06f99",
        "ca2a2eaf2071ce84b529f425115383ac67f932de263f1be20be664e627f94b6c",
    ),
];

// ISSUE_202's Release-7.10 wrapper exercises every supported
// `-redefined_params` mode against first- and last-definition controls.  Bind
// the complete family, wrapper runtime, and xyce_verify's eagerly loaded DC
// modules before reproducing that mode matrix natively.
const XYCE_ISSUE202_OWNER_CONTRACT: &str = "issue202_redefined_params_wrapper_owner";
const XYCE_ISSUE202_FIRST_CONTRACT: &str = "issue202_redefined_params_first_control";
const XYCE_ISSUE202_LAST_CONTRACT: &str = "issue202_redefined_params_last_control";
const XYCE_ISSUE202_OWNER_PATH: &str = "Netlists/Certification_Tests/ISSUE_202/first_last.cir";
const XYCE_ISSUE202_FIRST_PATH: &str = "Netlists/Certification_Tests/ISSUE_202/first.cir";
const XYCE_ISSUE202_LAST_PATH: &str = "Netlists/Certification_Tests/ISSUE_202/last.cir";
const XYCE_ISSUE202_OWNER_RECORD: &str = "netlists/certification_tests/issue_202/first_last.cir";
const XYCE_ISSUE202_FIRST_RECORD: &str = "netlists/certification_tests/issue_202/first.cir";
const XYCE_ISSUE202_LAST_RECORD: &str = "netlists/certification_tests/issue_202/last.cir";
const XYCE_ISSUE202_EXCLUSION_SOURCE: &str = "Netlists/Certification_Tests/ISSUE_202/exclude";
const XYCE_ISSUE202_UPSTREAM_REGRESSION_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const XYCE_ISSUE202_UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const XYCE_ISSUE202_HISTORICAL_RECORD_COUNT: usize = 13;
const XYCE_ISSUE202_HISTORICAL_RECORD_BYTES: usize = 3_048;
const XYCE_ISSUE202_HISTORICAL_RECORDS_SHA256: &str =
    "7cd2e0b8775344deca4dda55d597b2fc64a65918d94d1a881f250e0df79571e9";
const XYCE_ISSUE202_HISTORICAL_RECORDS_BLAKE3: &str =
    "fce3a673e02ac5f6e7d1123f1594ff83ed96e751440fa9ecd51b889819341873";
const XYCE_ISSUE202_HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 13] = [
    (
        "Netlists/Certification_Tests/ISSUE_202/CMakeLists.txt",
        1_512,
        "f4dd1b1d9f598c891a22121e10273485a9fb366fa3d8314bf272191c20090599",
        "0abed9b61aa2bce200dca4778e91bad5ffbc75370eb95a90c93b70fe71b32c25",
    ),
    (
        "Netlists/Certification_Tests/ISSUE_202/Manifest.txt",
        65,
        "2f887f96153566add8ddd266664922da9c36e89d57c72f7226de8e271e2ac91d",
        "340b7438d1df64406d988b814c1693f48e99690cde79f9115520aecdb7161ed4",
    ),
    (
        XYCE_ISSUE202_EXCLUSION_SOURCE,
        19,
        "003b5acc32c8ddf2353b8e7e6001aebc9ef253377f8adb4abbe2eef514ec7cb5",
        "d1e35da56beb81c847c502a3df57b013ae1010a5effb15b3aca4e2f93ca759e3",
    ),
    (
        XYCE_ISSUE202_FIRST_PATH,
        91,
        "37475640dc78f903fd4b16d43258108758d3ea65f76ab979f3e240a7b72e2acc",
        "1c0e209be6c743c852f01f285f97f527f4093b7f9163c4a305ef00ffc64ac56e",
    ),
    (
        XYCE_ISSUE202_OWNER_PATH,
        180,
        "6b7936972f661334bf7eeec05cd97f5a182976c297f87960df7fb622c36382da",
        "fcfe81d1bfb4c48be978dc2ee045912d56d490b504be9076efabb34bafb7c5d7",
    ),
    (
        "Netlists/Certification_Tests/ISSUE_202/first_last.cir.sh",
        11_995,
        "fe79a8c90fc50637f18dfa45d15596ed2edc3cfa376b66f3b0a9aacadb740e62",
        "1a46572d34ba7d177b3d2e5d095acebd64efc77044a31f4b3be240149ed785a9",
    ),
    (
        XYCE_ISSUE202_LAST_PATH,
        88,
        "65e351a4ac5983ebb5545ace148a1d5f866023e55c57299aba626fcc9c84e16e",
        "2e7a4013cb2cb17c701b01fe620fe45e81d0b7af58b09b9401f9b54e7d57f546",
    ),
    (
        "Netlists/Certification_Tests/ISSUE_202/tags",
        35,
        "4aa425839be44d5beb2c5f7f17dc4137e484cb67b24530c2b77634095fa1fda7",
        "b3a3b88d04b7b20053c7e3eb738a2f2dfa337f0b23fdf8f7a58e4680a6315728",
    ),
    (
        "TestScripts/XyceRegression/Tools.pm",
        68_108,
        "5b5f86c02d46a1f3bdad5292e7e91d25a9e08e71490643d8d5ed7ae20f9d55e3",
        "13bd274632744ddc4b8baee680ddc9770902793ed7ee892ecdedd4dcb3828667",
    ),
    (
        "TestScripts/XyceVerify/DCSources.pm",
        2_739,
        "b2ddcab5ad5a89c428b9b4430190fa27ef7106da7e7afeb31452c81890a9a006",
        "0905f9dc79d7c5bdbe17e3c2360cd063d6fcbf41823a410f98b236783d109ad7",
    ),
    (
        "TestScripts/XyceVerify/DCSweep.pm",
        9_301,
        "2246da2374e6cce3ea516a50e472fb07f7481e8b0effb20d4a650e6b6cb1eda0",
        "b9cc7d905d001ebe2ace44936b9631e4bdcbf42bca4d4b34c5866262cd11d9a3",
    ),
    (
        "TestScripts/XyceVerify/StepSweep.pm",
        8_731,
        "84b2d485c1848f2e456463de8a5015205d87c3db8a6d070547d6f9464618fed6",
        "db1b142ab3ae9163bbe02bd68b5b3a6311436adbf27c06d71a5c05df9b6973e7",
    ),
    (
        XYCE_RELEASE_710_XYCE_VERIFY_PATH,
        XYCE_RELEASE_710_XYCE_VERIFY_BYTES,
        XYCE_RELEASE_710_XYCE_VERIFY_SHA256,
        XYCE_RELEASE_710_XYCE_VERIFY_BLAKE3,
    ),
];
const XYCE_ISSUE202_RETAINED_RECORD_COUNT: usize = 3;
const XYCE_ISSUE202_RETAINED_RECORD_BYTES: usize = 433;
const XYCE_ISSUE202_RETAINED_RECORDS_SHA256: &str =
    "5899e31d37dd6ee5ae5cfb777be97bca22b477ba885cf141db108c725cc92dbc";
const XYCE_ISSUE202_RETAINED_RECORDS_BLAKE3: &str =
    "233f3696fdb059e0a1399667605e323e3c6da4d9575efeeeab35bf7445888283";
const XYCE_ISSUE202_RETAINED_ARTIFACTS: [(&str, usize, &str, &str); 3] = [
    (
        "first.cir",
        91,
        "37475640dc78f903fd4b16d43258108758d3ea65f76ab979f3e240a7b72e2acc",
        "1c0e209be6c743c852f01f285f97f527f4093b7f9163c4a305ef00ffc64ac56e",
    ),
    (
        "first_last.cir",
        180,
        "6b7936972f661334bf7eeec05cd97f5a182976c297f87960df7fb622c36382da",
        "fcfe81d1bfb4c48be978dc2ee045912d56d490b504be9076efabb34bafb7c5d7",
    ),
    (
        "last.cir",
        88,
        "65e351a4ac5983ebb5545ace148a1d5f866023e55c57299aba626fcc9c84e16e",
        "2e7a4013cb2cb17c701b01fe620fe45e81d0b7af58b09b9401f9b54e7d57f546",
    ),
];

// ISSUE_451's Release-7.10 wrapper compares a hierarchical-node circuit to
// an explicit in-subcircuit reference. It first requires a byte-exact PRN
// diff and only then falls back to xyce_verify, so bind the complete family
// activation plus the verifier and all of its eagerly loaded repository
// modules before reproducing the relation natively.
const XYCE_ISSUE451_OWNER_CONTRACT: &str = "issue451_hierarchical_node_wrapper_owner";
const XYCE_ISSUE451_REFERENCE_CONTRACT: &str = "issue451_hierarchical_node_explicit_reference";
const XYCE_ISSUE451_OWNER_PATH: &str = "Netlists/Certification_Tests/ISSUE_451/issue451.cir";
const XYCE_ISSUE451_REFERENCE_PATH: &str =
    "Netlists/Certification_Tests/ISSUE_451/issue451_ref.cir";
const XYCE_ISSUE451_OWNER_RECORD: &str = "netlists/certification_tests/issue_451/issue451.cir";
const XYCE_ISSUE451_REFERENCE_RECORD: &str =
    "netlists/certification_tests/issue_451/issue451_ref.cir";
const XYCE_ISSUE451_EXCLUSION_SOURCE: &str = "Netlists/Certification_Tests/ISSUE_451/exclude";
const XYCE_ISSUE451_UPSTREAM_REGRESSION_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const XYCE_ISSUE451_UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const XYCE_ISSUE451_HISTORICAL_RECORD_COUNT: usize = 11;
const XYCE_ISSUE451_HISTORICAL_RECORD_BYTES: usize = 2_586;
const XYCE_ISSUE451_HISTORICAL_RECORDS_SHA256: &str =
    "a8be3ab4f5e030330dbc8af5c1921c811fa9f536486e8b044e0da8d524184d61";
const XYCE_ISSUE451_HISTORICAL_RECORDS_BLAKE3: &str =
    "40c60a4625bf10ebab7b4146f65a0db8a8f40046aece740a76e3709ab1f92ca4";
const XYCE_ISSUE451_HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 11] = [
    (
        "Netlists/Certification_Tests/ISSUE_451/CMakeLists.txt",
        1_360,
        "c3a36d9176e2a9f63ff4a7d223db48f30bd4310f21e9c1b9238f56249bdb5a46",
        "e15f8db50bb488130cc9503eed4f92d5a42a3ea4816d4882bbb35f96136f3362",
    ),
    (
        "Netlists/Certification_Tests/ISSUE_451/Manifest.txt",
        59,
        "7cb305c1a5dd98cb3e96ca9a505c8c80fdc1e94c2a02d566f001c6d55d193eb9",
        "e7e09a2f49b41c4b6d27208f35e6c69c9ba4c60a2dacd98aefba165b4d9e9a96",
    ),
    (
        XYCE_ISSUE451_EXCLUSION_SOURCE,
        17,
        "147230167b51c3cd822442628873f82cd46094b924d4f0a78bfc255ee6e863c3",
        "5e1ac817048a6ed43937757ee5976a19f6f3843ac8f48c7ad50abffe0c453ae2",
    ),
    (
        XYCE_ISSUE451_OWNER_PATH,
        371,
        "6df6de48c1cb9b3a07c7b00c79974c498b0bc372e4f0c5cface9bae06623d49a",
        "dd15a6e050f1b1bfb9e256c0e36fdac756469a593c5cc097f5fac982ab0933db",
    ),
    (
        "Netlists/Certification_Tests/ISSUE_451/issue451.cir.sh",
        1_504,
        "12f11e5b7471d8aee5a32553d52e8858dba0f7028386bd0678a4ab3dfbfdbdc6",
        "ee142d134dac30f68a0f2303cdd5eb396b1a6240eb96e6a83487035ce5e66b5e",
    ),
    (
        XYCE_ISSUE451_REFERENCE_PATH,
        421,
        "b4b817871d79dc3f61fe0837159ec9b07eb689564d3b652cfb0f157074404b24",
        "45fd5785ec0a81ebebd5996aa2fc85d93157119da277cada6e5640f1d46a781c",
    ),
    (
        "Netlists/Certification_Tests/ISSUE_451/tags",
        26,
        "fe1c0752e6b37e25c9ffdf07f2f27528606cd2e32317665547187d451f3a9047",
        "f2fe6fa8055dde2c654c4f81c41dfd23b3c3b111f8ec3879cd3cbd2a29b141f1",
    ),
    (
        "TestScripts/XyceVerify/DCSources.pm",
        2_739,
        "b2ddcab5ad5a89c428b9b4430190fa27ef7106da7e7afeb31452c81890a9a006",
        "0905f9dc79d7c5bdbe17e3c2360cd063d6fcbf41823a410f98b236783d109ad7",
    ),
    (
        "TestScripts/XyceVerify/DCSweep.pm",
        9_301,
        "2246da2374e6cce3ea516a50e472fb07f7481e8b0effb20d4a650e6b6cb1eda0",
        "b9cc7d905d001ebe2ace44936b9631e4bdcbf42bca4d4b34c5866262cd11d9a3",
    ),
    (
        "TestScripts/XyceVerify/StepSweep.pm",
        8_731,
        "84b2d485c1848f2e456463de8a5015205d87c3db8a6d070547d6f9464618fed6",
        "db1b142ab3ae9163bbe02bd68b5b3a6311436adbf27c06d71a5c05df9b6973e7",
    ),
    (
        XYCE_RELEASE_710_XYCE_VERIFY_PATH,
        XYCE_RELEASE_710_XYCE_VERIFY_BYTES,
        XYCE_RELEASE_710_XYCE_VERIFY_SHA256,
        XYCE_RELEASE_710_XYCE_VERIFY_BLAKE3,
    ),
];
const XYCE_ISSUE451_RETAINED_RECORD_COUNT: usize = 2;
const XYCE_ISSUE451_RETAINED_RECORD_BYTES: usize = 297;
const XYCE_ISSUE451_RETAINED_RECORDS_SHA256: &str =
    "776cc2670147a620df05c61f55afd6c3553feab6015e5c07a680d2793564134b";
const XYCE_ISSUE451_RETAINED_RECORDS_BLAKE3: &str =
    "f52d2458b599ca6076ab6455092ca348e6eea8833512fe774fe570135718d3c1";
const XYCE_ISSUE451_RETAINED_ARTIFACTS: [(&str, usize, &str, &str); 2] = [
    (
        "issue451.cir",
        371,
        "6df6de48c1cb9b3a07c7b00c79974c498b0bc372e4f0c5cface9bae06623d49a",
        "dd15a6e050f1b1bfb9e256c0e36fdac756469a593c5cc097f5fac982ab0933db",
    ),
    (
        "issue451_ref.cir",
        421,
        "b4b817871d79dc3f61fe0837159ec9b07eb689564d3b652cfb0f157074404b24",
        "45fd5785ec0a81ebebd5996aa2fc85d93157119da277cada6e5640f1d46a781c",
    ),
];

// BUG_1455 proves that SPICE model parameters separated by whitespace parse
// identically to their `name=value` spellings. The Release wrapper executes
// both MOS1 decks and applies a case-insensitive byte diff, with no numerical
// gold. Bind its complete executable dependency set and add a closed-form
// MOS/load-line check so shared parser or device failures cannot false-pass.
const XYCE_BUG1455_OWNER_CONTRACT: &str = "bug1455_model_equals_wrapper_owner";
const XYCE_BUG1455_REFERENCE_CONTRACT: &str = "bug1455_model_whitespace_reference";
const XYCE_BUG1455_OWNER_PATH: &str = "Netlists/Certification_Tests/BUG_1455/bug_1455.cir";
const XYCE_BUG1455_REFERENCE_PATH: &str = "Netlists/Certification_Tests/BUG_1455/bug_1455_neq.cir";
const XYCE_BUG1455_OWNER_RECORD: &str = "netlists/certification_tests/bug_1455/bug_1455.cir";
const XYCE_BUG1455_REFERENCE_RECORD: &str =
    "netlists/certification_tests/bug_1455/bug_1455_neq.cir";
const XYCE_BUG1455_EXCLUSION_SOURCE: &str = "Netlists/Certification_Tests/BUG_1455/exclude";
const XYCE_BUG1455_UPSTREAM_REGRESSION_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const XYCE_BUG1455_UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const XYCE_BUG1455_HISTORICAL_RECORD_COUNT: usize = 9;
const XYCE_BUG1455_HISTORICAL_RECORD_BYTES: usize = 2_144;
const XYCE_BUG1455_HISTORICAL_RECORDS_SHA256: &str =
    "5253c602c7b6a70953242a2be887a4af2a8438b485273b61e1b3da71e9932eed";
const XYCE_BUG1455_HISTORICAL_RECORDS_BLAKE3: &str =
    "96863ac53c17e363161d55c114ca5edc1c33ff7d0b5f5f764f2c1bd84ab3e18c";
const XYCE_BUG1455_HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 9] = [
    (
        "Netlists/Certification_Tests/BUG_1455/CMakeLists.txt",
        1_483,
        "b3c67bb736e612786945c69afb795a85dcb30f5ec1947fb9e4bf836ae7752e7c",
        "a2067cc30ffd5d01d892961619b630ab669f4b11f41a033a9719478001fc1f49",
    ),
    (
        "Netlists/Certification_Tests/BUG_1455/Manifest.txt",
        66,
        "8a52259798c6a2fd06a4d90244704c04dcfcff7bdbab49790ff19faf8d1c5d18",
        "61bb11bdab295840d1a38eb0ba427a54b3284140ce9abcec053216d3069926ee",
    ),
    (
        "Netlists/Certification_Tests/BUG_1455/README",
        555,
        "7de04060151ebaf5c957088f82ac18c7b89e123ae89d8ca77f7df88c66e22369",
        "82c372c31d18700c21fd7fb429f0f233b20a593affbf83b38f2dfa9de7e2b284",
    ),
    (
        XYCE_BUG1455_OWNER_PATH,
        265,
        "707bb73a5bcadbe622fb5b7572dd5d35c12ed39601de9b3b175470a6e3a870fa",
        "ff8b02c75adde9925c7b5a8d4941bcc3658162cf42f96854d64413e82a999c3b",
    ),
    (
        "Netlists/Certification_Tests/BUG_1455/bug_1455.cir.sh",
        1_446,
        "3c1c2cf4d6f8faef706bcc246deddc76438d48c792425d6d438f984d4f6063ed",
        "901e4ec23df4dd39a4a563646b8f47e8ca35174ee744c5180a422b2257d8fd28",
    ),
    (
        XYCE_BUG1455_REFERENCE_PATH,
        260,
        "242be3e5853e4ece1925a8fa895edd9ccf847262312883b3828e737bff8ff6e4",
        "8d394468e092320b35f2dd68e0091f83076250bc11cae62f4bd8e72babfa6337",
    ),
    (
        XYCE_BUG1455_EXCLUSION_SOURCE,
        17,
        "51d9adb6f4c188f65ca5494af397bfb5f7694786a3843f52e38dcad665b237b9",
        "01157bf60692659ecd8bb468195f9de1352f82835b6060f658c6f5fbebc8b6da",
    ),
    (
        "Netlists/Certification_Tests/BUG_1455/tags",
        16,
        "fb8b1ab6aa8b694212335a76b1b87c077f22be7543f15c12de32a2da40b4f345",
        "a5f2cee6f41471429bc22c4c40d36881f4c11d2387b20adbdc14efe2509f6589",
    ),
    (
        "TestScripts/XyceRegression/Tools.pm",
        68_108,
        "5b5f86c02d46a1f3bdad5292e7e91d25a9e08e71490643d8d5ed7ae20f9d55e3",
        "13bd274632744ddc4b8baee680ddc9770902793ed7ee892ecdedd4dcb3828667",
    ),
];
const XYCE_BUG1455_RETAINED_RECORD_COUNT: usize = 3;
const XYCE_BUG1455_RETAINED_RECORD_BYTES: usize = 438;
const XYCE_BUG1455_RETAINED_RECORDS_SHA256: &str =
    "ebf0d1a684c5aa4ad162d7a5927679f7642b9b92e0a014081b70170224d9fb25";
const XYCE_BUG1455_RETAINED_RECORDS_BLAKE3: &str =
    "084be4e08167ae713f4b2ce164f80f078a81917a3c440c041bd68f2073a9876d";
const XYCE_BUG1455_RETAINED_ARTIFACTS: [(&str, usize, &str, &str); 3] = [
    (
        "bug_1455.cir",
        265,
        "707bb73a5bcadbe622fb5b7572dd5d35c12ed39601de9b3b175470a6e3a870fa",
        "ff8b02c75adde9925c7b5a8d4941bcc3658162cf42f96854d64413e82a999c3b",
    ),
    (
        "bug_1455_neq.cir",
        260,
        "242be3e5853e4ece1925a8fa895edd9ccf847262312883b3828e737bff8ff6e4",
        "8d394468e092320b35f2dd68e0091f83076250bc11cae62f4bd8e72babfa6337",
    ),
    (
        "README",
        555,
        "7de04060151ebaf5c957088f82ac18c7b89e123ae89d8ca77f7df88c66e22369",
        "82c372c31d18700c21fd7fb429f0f233b20a593affbf83b38f2dfa9de7e2b284",
    ),
];

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
const XYCE_BUG28SON_EXCLUSION_SOURCE: &str = "Netlists/Certification_Tests/BUG_28_SON/exclude";
const XYCE_BUG28SON_UPSTREAM_REGRESSION_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const XYCE_BUG28SON_UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const XYCE_BUG28SON_HISTORICAL_RECORD_COUNT: usize = 10;
const XYCE_BUG28SON_HISTORICAL_RECORD_BYTES: usize = 2_404;
const XYCE_BUG28SON_HISTORICAL_RECORDS_SHA256: &str =
    "1c6fd3f6346f3ab8ef09228a34653496e78d36025959263abdd92522dc58e904";
const XYCE_BUG28SON_HISTORICAL_RECORDS_BLAKE3: &str =
    "edd96a6a017ed5cecbbb8aa0eeabd3ae720a1472232fc235c463fbf0075a5f3b";
const XYCE_BUG28SON_HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 10] = [
    (
        "Netlists/Certification_Tests/BUG_28_SON/CMakeLists.txt",
        9_986,
        "1d8fd61019741249953546feb58a8929454488c35b73645684dac7d44e732cae",
        "7cf0826064a4f3817f526301e281df587b9c0a77f183614852c281ab73301a26",
    ),
    (
        "Netlists/Certification_Tests/BUG_28_SON/Manifest.txt",
        345,
        "27b4a4488fd30f4be821af3f7a463d11f3cf997d2cc137962f415d9d20a4e998",
        "7bce467bce222dd9d98e290799cbb1d8b4f464d331b14be3f6ded74734855d51",
    ),
    (
        XYCE_BUG28SON_OWNER_PATH,
        1_535,
        "3d98e802801f3bac2446470e7e8e47ce4b10e9a1198f2e233573e5220c87d0df",
        "305a979f760eab73d389be69fc4c968f805f24c7e15dff49259b8378944d61b6",
    ),
    (
        "Netlists/Certification_Tests/BUG_28_SON/bug_28_son3.cir.sh",
        4_046,
        "06eaa8bff2566a04a3844b506277030e980e2038da8465116972e88b26183f99",
        "14895eebf18154704dc8b3d10036c959ffd682e3267920be3e0ee3d2e2b4fa6e",
    ),
    (
        XYCE_BUG28SON_GLOBAL_PATH,
        1_438,
        "1b34c062dcd8707d6010a32536ac31fc9b0f84ae46f66d2e263cc775ca75754b",
        "2e9b01fdf87cae23cb26bc4dda98b31ad10e8214981b28dc0d104e42355a44ef",
    ),
    (
        XYCE_BUG28SON_LITERAL_PATH,
        1_796,
        "457a5c211e4c3db59681fa6ddf4f74f7df02395767a77b7a191d23d7da25e1d4",
        "8b8dec06660f3c299a9f485f16ba6252095fc0cfeaa519b5597432c34d4360a7",
    ),
    (
        "Netlists/Certification_Tests/BUG_28_SON/exclude",
        157,
        "3dda928474c3652d83fba189850ed0cd16786ed3ee266283d36724f94fac3564",
        "3eea433b9403e9857cad4c3c5fff6ecb815d1dc78ede5ac9de81678d8c9ee85b",
    ),
    (
        "Netlists/Certification_Tests/BUG_28_SON/tags",
        50,
        "a4075ac6df3b4a04c24d56ba01f81827dc448a03e3751b4bde77f641bfa8b33f",
        "f0a0019ad6275dce8d53f7562ed2027126afe449888bb4d8197062a40b773fb3",
    ),
    (
        "TestScripts/xplat_diff.pl",
        2_866,
        "8a042dfcf1db979a5a620fae4908e2e063530ab3a8a70c387a3ef97437e43148",
        "62227f94bbaf97e3cda7258bc8041102e08998da24555c3835e6c69ceca672b4",
    ),
    (
        "TestScripts/xyce_verify.pl",
        59_566,
        "6e5f84b1646b30d0e12879848d7653584b39472d640a14916ae8fda6e1df12b3",
        "5eadb6dab06ed3091ea114146bd4a574de83784f87be9843ad7b721b0a793665",
    ),
];
const XYCE_BUG28SON_RETAINED_ARTIFACTS: [(&str, usize, &str, &str); 15] = [
    (
        "bug_28_son1.cir",
        451,
        "7f3e1f328cb2ae677b5430357e01777b3734dc85224e1cda91728045b9c1c756",
        "0ca646cbc1169e88dc4f6bd62d9943d99343d8c0c32f29370417442bfd0a2b4f",
    ),
    (
        "bug_28_son2.cir",
        775,
        "be164d879cda122a49362aff80471f8be462c5b84dbdefa6905655d9d36ca421",
        "5a692f8eca006b51e44b61164b7887ca97d414b432e43a223109c8cb221af6aa",
    ),
    (
        "bug_28_son3.cir",
        1_535,
        "3d98e802801f3bac2446470e7e8e47ce4b10e9a1198f2e233573e5220c87d0df",
        "305a979f760eab73d389be69fc4c968f805f24c7e15dff49259b8378944d61b6",
    ),
    (
        "bug_28_son3_globalp.cir",
        1_438,
        "1b34c062dcd8707d6010a32536ac31fc9b0f84ae46f66d2e263cc775ca75754b",
        "2e9b01fdf87cae23cb26bc4dda98b31ad10e8214981b28dc0d104e42355a44ef",
    ),
    (
        "bug_28_son3_noparams.cir",
        1_796,
        "457a5c211e4c3db59681fa6ddf4f74f7df02395767a77b7a191d23d7da25e1d4",
        "8b8dec06660f3c299a9f485f16ba6252095fc0cfeaa519b5597432c34d4360a7",
    ),
    (
        "bug_28_son4.cir",
        1_771,
        "7e67ac5261ff08692b294852f20a8a23125110d735038cbafde8933faeb7adcc",
        "c9c2f87c8bb7bf80cc32766f9c66cb0d827d863d028045688a7eefb0009ffb1b",
    ),
    (
        "bug_28_son4.cir.res.gs",
        154,
        "5b765d50f5fe56b46082b7b4a7a181a5b39bb58013c25cfd0d990a581ad6daa7",
        "7754c0892faa6f74e3e12b7495612860856b25dacf68d3aff08cca71889e5fec",
    ),
    (
        "bug_28_son4_1.cir",
        1_734,
        "d5d3012a77d7c6e8d030b7f2b36524de1bd4f4ba8d8c3da998158943df34778a",
        "e48ef9d60a93c1f4684fed9263801c7292ba56b74baf7fa9d963a9714be9977a",
    ),
    (
        "bug_28_son4_2.cir",
        1_750,
        "457c5c78ed68ac4ac5234e032ebb201f8d15a32c60972914f042158d045cb947",
        "e80c9d1dc04861f431cbc7197e66866d3ea75b3719d5732567476719b23b48fe",
    ),
    (
        "bug_28_son4_3.cir",
        1_732,
        "ddf180a7bc1b791ddcec38d688143f06545d78d20013d1c4d3cabd54f90a4ace",
        "909be9640bbb9c20dc7019f5ca2531af1a836235bc605bdf844385fbe640a031",
    ),
    (
        "bug_28_son5.cir",
        1_186,
        "547dad138de63437e7bcdf1edfb9cd46466fe9ca5b690c4053210169e3a36dfd",
        "755162a5de1dcbed04f7842323b08dee1cb8e863b8743e433dd1d3905897e8a6",
    ),
    (
        "bug_28_son5.cir.res.gs",
        154,
        "4113e0695db581a36ab58a906df9aeef20c6fb4a874f74f57426ac67aff24ec5",
        "0b6c6af029fc3d90e5c20131aca0746a48582fc83f1e6bc33207fe648ac6161e",
    ),
    (
        "bug_28_son5_1.cir",
        1_084,
        "367d1a26d6e64d5fd76469b959d443ec0e63f5b7f36a95d0e82a245be222ea52",
        "29f19ba249659e6ad177af52ff372a7ec106480fc9ae7bdb1ee5c61617da1236",
    ),
    (
        "bug_28_son5_2.cir",
        1_101,
        "4de9c21ae72ba5246878a8376a321f3853c5798efa679126446ff3ddc5011245",
        "005dac325f16a025aea40590094429b51a592bdcf49b914f3076d2a543e66417",
    ),
    (
        "bug_28_son5_3.cir",
        1_085,
        "b05be10c7d4e8703849f8d4efed3fb8634d4a96d06f101d8fd77205797fe7323",
        "eecfd43220bc3f1f4ea4aa1b41822810185209aafdaa4db98dc01c6f31922701",
    ),
];
const XYCE_BUG28SON_RETAINED_OUTPUTS: [(&str, usize, &str, &str); 2] = [
    (
        "bug_28_son1.cir.prn",
        531,
        "ba89f7a08ffc7d077fbb90194289544eaaadf0804fbcc1b768d73218d12319bf",
        "e11c3b2f1ec22c6f895308ab4c03ee3a925db01936c3e60927d7458930654911",
    ),
    (
        "bug_28_son2.cir.prn",
        531,
        "3307e07168a40371d01793151760a4c87c600428127973f0237f8dafc3eb0b30",
        "45139bb1bb7d4ff441e77f7556f4a190fc8fbd7b54303eadeda7a9fe2184a80a",
    ),
];

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
const XYCE_BUG1398_EXCLUSION_SOURCE: &str = "Netlists/Certification_Tests/BUG_1398/exclude";
const XYCE_BUG1398_UPSTREAM_REGRESSION_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const XYCE_BUG1398_UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const XYCE_BUG1398_HISTORICAL_RECORD_COUNT: usize = 9;
const XYCE_BUG1398_HISTORICAL_RECORD_BYTES: usize = 2_125;
const XYCE_BUG1398_HISTORICAL_RECORDS_SHA256: &str =
    "1d49e5e80e787735c0a493df81591f933386bff03d163de40ce9e70f446a4fc3";
const XYCE_BUG1398_HISTORICAL_RECORDS_BLAKE3: &str =
    "847ec1b72aae005db0723c729d346a096d4dbb355ea888fa6f5530008f315a83";
const XYCE_BUG1398_HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 9] = [
    (
        "Netlists/Certification_Tests/BUG_1398/CMakeLists.txt",
        1_407,
        "3ba7a0043a015de02d06bbfd287ff7aa603f6f4ce8676407a403cb9e6d3535e1",
        "10c2937b7c7011c9a773f1bc7a2a7b92c17994e948303059ca6cfc33f670c200",
    ),
    (
        "Netlists/Certification_Tests/BUG_1398/Manifest.txt",
        54,
        "965055ad79e864b031d44be70eabbae0da035742c8c3a1cad3f56c5597cfd0ba",
        "c0ce8b50741714dca2994cf18b878aa7638eabe962465f784ee0695db0bc5c07",
    ),
    (
        "Netlists/Certification_Tests/BUG_1398/README",
        929,
        "0b9ba507d1c6fe0e3069ca597f5d53ad22e1586efd2f8b93d49d2f87add0542d",
        "2fd50c8db6245af365903c5d528cc663e0827b3b9a408d255be634a9c891b16c",
    ),
    (
        XYCE_BUG1398_OWNER_PATH,
        2_138,
        "fd3f8981c807cfc9f765b1f35b1653ef3a8cd375f44fc9f97cd325d1dc4847c2",
        "a1ac796d496fae47b8b2dcb572b01052f3e35d1393380625cce6b93ff4b1cfba",
    ),
    (
        "Netlists/Certification_Tests/BUG_1398/RLC.cir.sh",
        1_253,
        "c84c60afe9b2a93a7f728e7ec9caa5c34e16139fdcf55ec52f4d8670a9dc5cae",
        "993f47e6eaa7799616ce9187f60fcbc58183bb14397be8812f8166f3860ed2e0",
    ),
    (
        XYCE_BUG1398_CONTROL_PATH,
        2_138,
        "09e0948211087bd4694fc458639c30073801061c5b7f88002670667bd43daec5",
        "f8cb087641c3697c49f96d42e2f32bd742f8e91ed657c85cc9791c00003ccbae",
    ),
    (
        "Netlists/Certification_Tests/BUG_1398/exclude",
        15,
        "77d7b65110d816a59680aac92540fba6b1ab8edf8237084b551c33d524adb0c7",
        "91212edebf08bcfa881f31f15151c181a07dc51eece94caca73993f545427a59",
    ),
    (
        "Netlists/Certification_Tests/BUG_1398/tags",
        25,
        "74e8e1f00eefb3fa603b15f573d3055de3c8acfb459c4c5cf249307e84fc8221",
        "f818cefd67a01c46afa08fd0db3adc873242ea1247675e884339900bbcdbe857",
    ),
    (
        XYCE_RELEASE_710_XYCE_VERIFY_PATH,
        XYCE_RELEASE_710_XYCE_VERIFY_BYTES,
        XYCE_RELEASE_710_XYCE_VERIFY_SHA256,
        XYCE_RELEASE_710_XYCE_VERIFY_BLAKE3,
    ),
];
const XYCE_BUG1398_RETAINED_ARTIFACTS: [(&str, usize, &str, &str); 3] = [
    (
        "README",
        929,
        "0b9ba507d1c6fe0e3069ca597f5d53ad22e1586efd2f8b93d49d2f87add0542d",
        "2fd50c8db6245af365903c5d528cc663e0827b3b9a408d255be634a9c891b16c",
    ),
    (
        "RLC.cir",
        2_138,
        "fd3f8981c807cfc9f765b1f35b1653ef3a8cd375f44fc9f97cd325d1dc4847c2",
        "a1ac796d496fae47b8b2dcb572b01052f3e35d1393380625cce6b93ff4b1cfba",
    ),
    (
        "RLC_simple.cir",
        2_138,
        "09e0948211087bd4694fc458639c30073801061c5b7f88002670667bd43daec5",
        "f8cb087641c3697c49f96d42e2f32bd742f8e91ed657c85cc9791c00003ccbae",
    ),
];

// BUG_271_SON's active Release-7.10 shell wrapper is a success-only oracle:
// it runs the exact tab-comment RLC/PULSE deck and passes iff Xyce exits zero.
const XYCE_BUG271_CONTRACT: &str = "bug271_tab_comment_rlc_success_wrapper";
const XYCE_BUG271_PATH: &str = "Netlists/Certification_Tests/BUG_271_SON/bug_271.cir";
const XYCE_BUG271_RECORD: &str = "netlists/certification_tests/bug_271_son/bug_271.cir";
const XYCE_BUG271_UPSTREAM_REGRESSION_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const XYCE_BUG271_UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const XYCE_BUG271_HISTORICAL_RECORD_COUNT: usize = 6;
const XYCE_BUG271_HISTORICAL_RECORD_BYTES: usize = 1_442;
const XYCE_BUG271_HISTORICAL_RECORDS_SHA256: &str =
    "aed635c95ee18fb7bc07e3013023a788813ae20ca24f1fd365ecdf9b48d2a5bb";
const XYCE_BUG271_HISTORICAL_RECORDS_BLAKE3: &str =
    "f171727c2029c581207c82b383d56714ffd2814e4c78c2a1f616bd42aaab0663";
const XYCE_BUG271_HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 6] = [
    (
        "Netlists/Certification_Tests/BUG_271_SON/CMakeLists.txt",
        1_568,
        "89e6533aba1c92a29ab82655de716f35b61012aeb5210ef24aa308ff6514aa12",
        "0dc0df79c99d2478925dd54d3882319c199f4cb90f47123db6275984fb27f2b8",
    ),
    (
        "Netlists/Certification_Tests/BUG_271_SON/Manifest.txt",
        32,
        "9e9c38f0b58689bf011e086f46458b26e3b1f2306dea16cfd7123c3e5a26824a",
        "d708db8cedaf97833c8bb9a6ed4796721f6d0bc49f2fe84127d81fb589824781",
    ),
    (
        XYCE_BUG271_PATH,
        328,
        "81d71de5399ec46dbcac067d5f48e6f8d61423b0fe5866dc18bb97073d1cea29",
        "2d718077d38a66e04d8b234e91c1c4ec480cbeb4500b45f498b76249635d5f0f",
    ),
    (
        "Netlists/Certification_Tests/BUG_271_SON/bug_271.cir.sh",
        1_030,
        "477003d4bbdad6cc1abff98bd569cb73e1cd49f938cb9d55120b0a9dd11b9cbf",
        "8d9b010dea1a98c1249df35fdfa5a86aa97bd085723b7f4737ba0e1394d98399",
    ),
    (
        "Netlists/Certification_Tests/BUG_271_SON/tags",
        27,
        "4b8ea8000b121cc87df41e65f9ac43dece3eab840c84e864357b3c1b9021cdcb",
        "c3661f9aadf0946ebc823f672267a34f3f9485b8a002ff0a2ff956cf0c9dc430",
    ),
    (
        "TestScripts/XyceRegression/Tools.pm",
        68_108,
        "5b5f86c02d46a1f3bdad5292e7e91d25a9e08e71490643d8d5ed7ae20f9d55e3",
        "13bd274632744ddc4b8baee680ddc9770902793ed7ee892ecdedd4dcb3828667",
    ),
];
const XYCE_BUG271_RETAINED_ARTIFACTS: [(&str, usize, &str, &str); 1] = [(
    "bug_271.cir",
    328,
    "81d71de5399ec46dbcac067d5f48e6f8d61423b0fe5866dc18bb97073d1cea29",
    "2d718077d38a66e04d8b234e91c1c4ec480cbeb4500b45f498b76249635d5f0f",
)];

// BUG_1661's active Release-7.10 Perl wrapper runs this one transient deck,
// requires the generated PRN, and compares the serialized V($G_1) and V(1)
// columns for exact numeric equality. It does not use a checked-in gold file.
const XYCE_BUG1661_CONTRACT: &str = "bug1661_globalnode_behavioral_expression_equality_wrapper";
const XYCE_BUG1661_PATH: &str = "Netlists/Certification_Tests/BUG_1661/globalnode_expr_toplev.cir";
const XYCE_BUG1661_RECORD: &str =
    "netlists/certification_tests/bug_1661/globalnode_expr_toplev.cir";
const XYCE_BUG1661_UPSTREAM_REGRESSION_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const XYCE_BUG1661_UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const XYCE_BUG1661_HISTORICAL_RECORD_COUNT: usize = 5;
const XYCE_BUG1661_HISTORICAL_RECORD_BYTES: usize = 1_229;
const XYCE_BUG1661_HISTORICAL_RECORDS_SHA256: &str =
    "c16712fc1bd34da589181dd2109ac813a6524aaa40bf796425b276fee38da47b";
const XYCE_BUG1661_HISTORICAL_RECORDS_BLAKE3: &str =
    "dec807a0c52ca3b3b02d0a128583b99308b01371df1fe7bcfa9023d55d1db436";
const XYCE_BUG1661_HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 5] = [
    (
        "Netlists/Certification_Tests/BUG_1661/CMakeLists.txt",
        1_320,
        "a487b4650fbc9357fbe0a53ffb7ee5939fd8954ed4cda83d71bcfdadae786479",
        "96460f20062b75eae10f434d25e6b7d60d8d5b98bf86d7f66973c6fbf33f4471",
    ),
    (
        "Netlists/Certification_Tests/BUG_1661/Manifest.txt",
        62,
        "be96035a82a5baefdfcaab1e1a91f89744875bd8a270121a8ed8f6a6c6aa7eea",
        "90983379352516dd523e8212cd5c416e0a87ade91ab34de2485da8c77b3e16af",
    ),
    (
        XYCE_BUG1661_PATH,
        166,
        "a3d847fd48a4c78171b422fd69a449ef4e5c02bcdcc0e0bd0c29fe9349b9d84f",
        "d59abb99dde37a857ce59902ca9ce93068ee27b46b173bcae5fabbd6d8604e1e",
    ),
    (
        "Netlists/Certification_Tests/BUG_1661/globalnode_expr_toplev.cir.sh",
        1_438,
        "3368c0053a3348adea087f6ad398bbbfcfc576294a7ab03574d6aa9174a54837",
        "8d284c65c445a1fa5d2283c8918437d55754a6c252c120c42cad14a7ae66161e",
    ),
    (
        "Netlists/Certification_Tests/BUG_1661/tags",
        49,
        "328d453b60a790e98d14cc0ed0c5e838577156852aa4982cafa9b0a892899787",
        "e33b0893cf7c5e9f2cee4fadd792972124b34d7e4a9e7de38801285962f6304e",
    ),
];
const XYCE_BUG1661_RETAINED_ARTIFACTS: [(&str, usize, &str, &str); 1] = [(
    "globalnode_expr_toplev.cir",
    166,
    "a3d847fd48a4c78171b422fd69a449ef4e5c02bcdcc0e0bd0c29fe9349b9d84f",
    "d59abb99dde37a857ce59902ca9ce93068ee27b46b173bcae5fabbd6d8604e1e",
)];

// BUG_519_SON's active Release-7.10 wrapper runs this transient once with
// binary RAW output and once with ASCII RAW output, converts both through the
// retained RAW reader boundary, and requires V(1) to equal I(R1) in each.
// It does not use a checked-in numerical gold file.
const XYCE_BUG519_CONTRACT: &str = "bug519_binary_ascii_raw_column_equality_wrapper";
const XYCE_BUG519_PATH: &str = "Netlists/Certification_Tests/BUG_519_SON/bug_519_SON.cir";
const XYCE_BUG519_RECORD: &str = "netlists/certification_tests/bug_519_son/bug_519_son.cir";
const XYCE_BUG519_UPSTREAM_REGRESSION_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const XYCE_BUG519_UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const XYCE_BUG519_HISTORICAL_RECORD_COUNT: usize = 7;
const XYCE_BUG519_HISTORICAL_RECORD_BYTES: usize = 1_658;
const XYCE_BUG519_HISTORICAL_RECORDS_SHA256: &str =
    "c1c463b78caa63d4d805c19a846b1128eb2559035dc326cd6c9420eeb73fb067";
const XYCE_BUG519_HISTORICAL_RECORDS_BLAKE3: &str =
    "fb834b337dd8fb61645005b4421f5990a02f7381a8c21634f2721375ace65d3a";
const XYCE_BUG519_HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 7] = [
    (
        "Netlists/Certification_Tests/BUG_519_SON/CMakeLists.txt",
        1_199,
        "8c7e471b0c8b26b6179765f7ab3aa3ca4f2887dabbe2ef097ba5bdc81fa16439",
        "6dfe3c481595e160025778e22b6800de7319340b3e0a272b69def0b98693d62e",
    ),
    (
        "Netlists/Certification_Tests/BUG_519_SON/Manifest.txt",
        40,
        "74cc5dadcdacce4cbb709888eb0892d2ecc0f8bbc0c60481d48785a0f7665a0d",
        "a9786f497874a7fe5f31c844fbcd78d7e4bd9fcc09bb2d1a83053016cb9a45cc",
    ),
    (
        XYCE_BUG519_PATH,
        132,
        "21bc57148169d7effb9bf0e215d1a53b19355bfa3017742a8bf6baad8c9532c1",
        "25ef552125f19dc15fc21ee9a2acd2deb95b9848e96e4cdfac3ce2de918cda3b",
    ),
    (
        "Netlists/Certification_Tests/BUG_519_SON/bug_519_SON.cir.sh",
        2_668,
        "e89688f5012a61746164a51f8d89d416273bc0aaa783b27d5c86266e436aa2b3",
        "db212b1474e70542f908a9c747ee355c577336004e4ad00b283f074e1d366e21",
    ),
    (
        "Netlists/Certification_Tests/BUG_519_SON/tags",
        28,
        "4ec87257b57ad0473d88c43a6e17efca9cf879efc633d48c1188f44f19ba6c9c",
        "8388849297713e605ed3de82616e0f5c4dbdc88d4d967c2650671bd008ce3b0a",
    ),
    (
        "TestScripts/convertToPrn.py",
        22_175,
        "46ba3ec7be7b301deff35ed6cffa17541fc4b9eb98ce84bba0d1b75ea2713735",
        "7256f6c3fbeeee7aded5e704ac85c519a90fdfbe51cff0177a3bd97796d16bbf",
    ),
    (
        // convertToPrn.py imports this parser helper directly.
        "TestScripts/findBlock.py",
        3_648,
        "56045f50bbaf567009ff4096c71d8f4cd0ae04a4a348fcc85b951b7a46ca49de",
        "bced5e9eb6ce2df5a6b23e50593bcddbd03805585cc63f84278ce4fccf4cc6d8",
    ),
];
const XYCE_BUG519_RETAINED_ARTIFACTS: [(&str, usize, &str, &str); 1] = [(
    "bug_519_SON.cir",
    132,
    "21bc57148169d7effb9bf0e215d1a53b19355bfa3017742a8bf6baad8c9532c1",
    "25ef552125f19dc15fc21ee9a2acd2deb95b9848e96e4cdfac3ce2de918cda3b",
)];

// BUG_1040_SON's Release-7.10 wrapper proves that the NOOP spelling of
// transient startup produces the same diode-capacitor discharge waveform as
// ordinary operating-point startup. The zero-byte .cir file is only the
// wrapper owner; the two retained .net files are the executable workers.
const XYCE_BUG1040_CONTRACT: &str = "bug1040_noop_operating_point_transient_wrapper";
const XYCE_BUG1040_OWNER_PATH: &str = "Netlists/Certification_Tests/BUG_1040_SON/bug_1040_son.cir";
const XYCE_BUG1040_OP_PATH: &str =
    "Netlists/Certification_Tests/BUG_1040_SON/rc_discharge_diode_op.net";
const XYCE_BUG1040_NOOP_PATH: &str =
    "Netlists/Certification_Tests/BUG_1040_SON/rc_discharge_diode_noop.net";
const XYCE_BUG1040_OWNER_RECORD: &str =
    "netlists/certification_tests/bug_1040_son/bug_1040_son.cir";
const XYCE_BUG1040_UPSTREAM_REGRESSION_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const XYCE_BUG1040_UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const XYCE_BUG1040_HISTORICAL_RECORD_COUNT: usize = 8;
const XYCE_BUG1040_HISTORICAL_RECORD_BYTES: usize = 1_964;
const XYCE_BUG1040_HISTORICAL_RECORDS_SHA256: &str =
    "81949e4b41df1756864d7b2b7d2cc89dd9e75cd9c3a39eaf0c158cd056ef1f06";
const XYCE_BUG1040_HISTORICAL_RECORDS_BLAKE3: &str =
    "3ba26b8f9fa83653b89a0e657d60629ef6fa1f55d7d742801885f75b7971da11";
const XYCE_BUG1040_HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 8] = [
    (
        "Netlists/Certification_Tests/BUG_1040_SON/CMakeLists.txt",
        1_931,
        "2b19f4aaf9dd481b49687f514873d7149d7410b096abf39a5399b0834f22a382",
        "56d5a786a1609c7bc684699235bdef42c4c3b1bde4a1ebc7a1833b7061c3dda8",
    ),
    (
        "Netlists/Certification_Tests/BUG_1040_SON/Manifest.txt",
        91,
        "c91d7e33be9e696e7cbb9f2603378f81ee8664b567e4b67dc04485104602d2f7",
        "f88d548325ced3d592705b3636f7c39050ec1ab32fd71fbd15d114d3dae1fee1",
    ),
    (
        XYCE_BUG1040_OWNER_PATH,
        0,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262",
    ),
    (
        "Netlists/Certification_Tests/BUG_1040_SON/bug_1040_son.cir.sh",
        1_964,
        "fde5dcfe158d3966594646c25436fee61fc0e056e071a751a0f60f9fe08030b8",
        "75dad870908939721c86a4d0d42bd94407a3a67c975305b20a6211061603da7b",
    ),
    (
        XYCE_BUG1040_NOOP_PATH,
        265,
        "7084016db0779c82ce55b1c072774e330b8d4d25100b18e773429b8d27502c5d",
        "0d73841755795bf8ba769e0f3ca0c77b4509b2674fa7f05b49779a09a0cf7027",
    ),
    (
        XYCE_BUG1040_OP_PATH,
        260,
        "cdaea1743b0ca4046b94b11ee56eb957996631cfbbd3466ba0f296c6cd5ed7ca",
        "81ba29e78edda22d3c14cea1bb4b2a0d7531241d460b62cbbf04f8d9919f4368",
    ),
    (
        "Netlists/Certification_Tests/BUG_1040_SON/tags",
        42,
        "8c7976b6bc7659c5ec9fb9e62e6b4fb8faef36c23efedc8034edd44194c87029",
        "13282063d079de074ec7423f0eb80670541a3a335e4388af7df4796e5873ceba",
    ),
    (
        XYCE_RELEASE_710_XYCE_VERIFY_PATH,
        XYCE_RELEASE_710_XYCE_VERIFY_BYTES,
        XYCE_RELEASE_710_XYCE_VERIFY_SHA256,
        XYCE_RELEASE_710_XYCE_VERIFY_BLAKE3,
    ),
];
const XYCE_BUG1040_RETAINED_ARTIFACTS: [(&str, usize, &str, &str); 3] = [
    (
        "bug_1040_son.cir",
        0,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262",
    ),
    (
        "rc_discharge_diode_noop.net",
        265,
        "7084016db0779c82ce55b1c072774e330b8d4d25100b18e773429b8d27502c5d",
        "0d73841755795bf8ba769e0f3ca0c77b4509b2674fa7f05b49779a09a0cf7027",
    ),
    (
        "rc_discharge_diode_op.net",
        260,
        "cdaea1743b0ca4046b94b11ee56eb957996631cfbbd3466ba0f296c6cd5ed7ca",
        "81ba29e78edda22d3c14cea1bb4b2a0d7531241d460b62cbbf04f8d9919f4368",
    ),
];

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
const XYCE_BUG1162_EXCLUSION_SOURCE: &str = "Netlists/Certification_Tests/BUG_1162_SON/exclude";
const XYCE_BUG1162_UPSTREAM_REGRESSION_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const XYCE_BUG1162_UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const XYCE_BUG1162_HISTORICAL_RECORD_COUNT: usize = 12;
const XYCE_BUG1162_HISTORICAL_RECORD_BYTES: usize = 2_909;
const XYCE_BUG1162_HISTORICAL_RECORDS_SHA256: &str =
    "7a8a9bd2199d36feb330f351a9aa8fde6811caeb3dd177c5ec47c878e7db0f58";
const XYCE_BUG1162_HISTORICAL_RECORDS_BLAKE3: &str =
    "d4ad630a2a9ec582ed30236fd0b0e7dfbe6481612a94993ee39e63687a675362";
const XYCE_BUG1162_HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 12] = [
    (
        "Netlists/Certification_Tests/BUG_1162_SON/CMakeLists.txt",
        1_695,
        "536193553d5f181d3b51d7727414c42d4c4a434c20a8fa03dde87c31fbc261ec",
        "a4b62d4cb26464135d4ab874499a72139f4400495641d79bc8e7f394bebd10aa",
    ),
    (
        "Netlists/Certification_Tests/BUG_1162_SON/Manifest.txt",
        109,
        "590a6d26a4c83827a7625976d2974f4977cff44814810cb475f29e09fb2955aa",
        "00fd10d5ee333284ea33bfc6874fc7315f7b8940b2ba0bab28db64db5a789ba5",
    ),
    (
        XYCE_BUG1162_BASELINE_PATH,
        359,
        "ed0bafa51ab5fae912e0e90b5a50aad3aa7d7565d45f69eb7b4ff88d42df0f23",
        "bf776c9aa9b37b834c0f433135d79a93ca87b1c8afdc79e9f380ca82385f238f",
    ),
    (
        XYCE_BUG1162_OWNER_PATH,
        0,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262",
    ),
    (
        "Netlists/Certification_Tests/BUG_1162_SON/bug_1162_son.cir.sh",
        2_943,
        "1bb150c5bfc1516a9916a734e53a813c6c3d0109a3bac76a9728180a4eda2afd",
        "e38d31859a60d7e83685f1cd3f14ddacfd7b998a8f10f129a28bc74162b00a6b",
    ),
    (
        XYCE_BUG1162_DECADE_PATH,
        460,
        "d31696bedd99454740b3a16ff5b9cc6390be3b80f0d1a7f983981b29257e4926",
        "e58dc957a0e9ae07c04a48b2124868b68fca8acbe49efc95e74547e3212efac7",
    ),
    (
        XYCE_BUG1162_LINEAR_PATH,
        447,
        "3ca53e5d9dd23f29ab057cd3af06604c3b295112d9921ef1675511349db8c3a4",
        "e87bda21a9bc62990e092b5e970b524d4cd87a4e7976f13239cd2d449ac8edf5",
    ),
    (
        XYCE_BUG1162_OCTAVE_PATH,
        459,
        "be51f5602cb79590aec74148ca85ec5740e838dd04bd944dbb66ed2ed76b1940",
        "99babd65cae23941cfbb7ec773dfe554bd5594ee3cf5bb27be257e049a8536af",
    ),
    (
        XYCE_BUG1162_EXCLUSION_SOURCE,
        67,
        "128deb269045ae15c340d47031bdcde02252d0c9dab82353a5e35a8caf3751f7",
        "3d5881a935121bf4eb82ac2728e44eee8cdabc8a3eeef4286f6e919f0d40637f",
    ),
    (
        "Netlists/Certification_Tests/BUG_1162_SON/tags",
        16,
        "fb8b1ab6aa8b694212335a76b1b87c077f22be7543f15c12de32a2da40b4f345",
        "a5f2cee6f41471429bc22c4c40d36881f4c11d2387b20adbdc14efe2509f6589",
    ),
    (
        "TestScripts/XyceRegression/Tools.pm",
        68_108,
        "5b5f86c02d46a1f3bdad5292e7e91d25a9e08e71490643d8d5ed7ae20f9d55e3",
        "13bd274632744ddc4b8baee680ddc9770902793ed7ee892ecdedd4dcb3828667",
    ),
    (
        XYCE_RELEASE_710_XYCE_VERIFY_PATH,
        XYCE_RELEASE_710_XYCE_VERIFY_BYTES,
        XYCE_RELEASE_710_XYCE_VERIFY_SHA256,
        XYCE_RELEASE_710_XYCE_VERIFY_BLAKE3,
    ),
];
const XYCE_BUG1162_RETAINED_ARTIFACTS: [(&str, usize, &str, &str); 5] = [
    (
        "baseline.cir",
        359,
        "ed0bafa51ab5fae912e0e90b5a50aad3aa7d7565d45f69eb7b4ff88d42df0f23",
        "bf776c9aa9b37b834c0f433135d79a93ca87b1c8afdc79e9f380ca82385f238f",
    ),
    (
        "bug_1162_son.cir",
        0,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262",
    ),
    (
        "defective_dec.cir",
        460,
        "d31696bedd99454740b3a16ff5b9cc6390be3b80f0d1a7f983981b29257e4926",
        "e58dc957a0e9ae07c04a48b2124868b68fca8acbe49efc95e74547e3212efac7",
    ),
    (
        "defective_lin.cir",
        447,
        "3ca53e5d9dd23f29ab057cd3af06604c3b295112d9921ef1675511349db8c3a4",
        "e87bda21a9bc62990e092b5e970b524d4cd87a4e7976f13239cd2d449ac8edf5",
    ),
    (
        "defective_oct.cir",
        459,
        "be51f5602cb79590aec74148ca85ec5740e838dd04bd944dbb66ed2ed76b1940",
        "99babd65cae23941cfbb7ec773dfe554bd5594ee3cf5bb27be257e049a8536af",
    ),
];

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
const XYCE_BUG307_EXCLUSION_SOURCE: &str = "Netlists/Certification_Tests/BUG_307/exclude";
const XYCE_BUG307_UPSTREAM_REGRESSION_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const XYCE_BUG307_UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const XYCE_BUG307_HISTORICAL_RECORD_COUNT: usize = 8;
const XYCE_BUG307_HISTORICAL_RECORD_BYTES: usize = 1_910;
const XYCE_BUG307_HISTORICAL_RECORDS_SHA256: &str =
    "d12d9999702cb03a5d8a7c035cd7b117e0de4a2179939ce0682ffec1919f02a9";
const XYCE_BUG307_HISTORICAL_RECORDS_BLAKE3: &str =
    "88ccdc0f9679d72ca35300fd520095689940c4c51b33b41cefd70adcc1bbbeb9";
const XYCE_BUG307_HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 8] = [
    (
        "Netlists/Certification_Tests/BUG_307/CMakeLists.txt",
        3_726,
        "c29d61b2ee4044b6f4f23675f615d2239c5da183efef50fc4e0471d0786f185d",
        "a8b77563dfd31851dee65a2ec285c8cc397dca56d9fdbf99dcdda94d4c324a80",
    ),
    (
        "Netlists/Certification_Tests/BUG_307/Manifest.txt",
        215,
        "8efafd0b6ed4df706276aaf78bddaa63bbc662727ef49002e577b2ff0e559e07",
        "c3e85eacadb2e4ec61b3f9f016b41744923e7d9928cdc9571f4d4ff726b1cadf",
    ),
    (
        "Netlists/Certification_Tests/BUG_307/README",
        2_221,
        "21a3c2eac2ec269b7aca08ed370b36597d19f27ab39e18893652cdcd52134017",
        "03de16d903037f8682982f23d5f10aa83c87a48cd4326071091286359c89d7e5",
    ),
    (
        XYCE_BUG307_OWNER_PATH,
        347,
        "a1679ad22a5e7acbf4bed85efd47f92f3d59326f3f611889a822840b37de3c22",
        "99c850d3ca6c4021d74d697b860fe4dc2b8541f41de98474f18175c08cfa1ef0",
    ),
    (
        "Netlists/Certification_Tests/BUG_307/bug_307_a.cir.sh",
        1_179,
        "736ee04d110134b4133412591f4c94715bdfbf3d8910015905b0e00811fc11e3",
        "9ab7ef6f1b0debea3c708f87ab3c270f53b0b8ea8db6a1a27e3a79fcd9a0d8cd",
    ),
    (
        XYCE_BUG307_CONTROL_PATH,
        263,
        "0dddf35b3f467b1554a795dd94ff5494e9ec5b25369ce797b1d03641f307ba0b",
        "1e56b30e9c542f761b112af245d49e005ddd91713e1a43bf94388b04f8e624c2",
    ),
    (
        "Netlists/Certification_Tests/BUG_307/exclude",
        258,
        "52c58cc2998c3b34b62a17fcea871a7affc41d77058c78b7dfb5d4aab93e965f",
        "ea052f70048b548f3877ffd8904c585b66a8fa986926c24483d29e7f02ad6382",
    ),
    (
        "Netlists/Certification_Tests/BUG_307/tags",
        16,
        "fb8b1ab6aa8b694212335a76b1b87c077f22be7543f15c12de32a2da40b4f345",
        "a5f2cee6f41471429bc22c4c40d36881f4c11d2387b20adbdc14efe2509f6589",
    ),
];
const XYCE_BUG307_RETAINED_ARTIFACTS: [(&str, usize, &str, &str); 11] = [
    (
        "README",
        2_221,
        "21a3c2eac2ec269b7aca08ed370b36597d19f27ab39e18893652cdcd52134017",
        "03de16d903037f8682982f23d5f10aa83c87a48cd4326071091286359c89d7e5",
    ),
    (
        "bug_307_a.cir",
        347,
        "a1679ad22a5e7acbf4bed85efd47f92f3d59326f3f611889a822840b37de3c22",
        "99c850d3ca6c4021d74d697b860fe4dc2b8541f41de98474f18175c08cfa1ef0",
    ),
    (
        "bug_307_b.cir",
        263,
        "0dddf35b3f467b1554a795dd94ff5494e9ec5b25369ce797b1d03641f307ba0b",
        "1e56b30e9c542f761b112af245d49e005ddd91713e1a43bf94388b04f8e624c2",
    ),
    (
        "bug_307_d.cir",
        944,
        "b9d7e9ff10c06d2f35dc227f609d723cb6777c6446d3067564da3f94de012d26",
        "b532796ea4f65c12e5f5712260fc54289ac5237d16bf5acd6813f78a8ea22217",
    ),
    (
        "bug_307_e.cir",
        946,
        "a4694f025a973b5e10172ad60344d24595f3363ce24ae1c196d5cbb8be55a864",
        "79afd80a2d97de4ef7a747c2b3d9f6d842aecb94653e916e850d7a65a5c4be37",
    ),
    (
        "bug_307_f.cir",
        1_052,
        "fa455d1e81bc1990479771e38b46ef217435c540ac54377e993caa63b75ea348",
        "8945c4563decbc1b26a6bf5bfe2ac89f567aaed7e0d0bfba12d5637e6467d5c2",
    ),
    (
        "bug_307_g.cir",
        1_106,
        "76d8a3ce9d13a58756a5b4b2426f1d20abc44e8e8d73dee96e9d27918b07cd4a",
        "bd81fea0f08c9ca161bee4c98771cbcb5a8b0a25c6d3c1dcfb20e23c9f5a8f07",
    ),
    (
        "bug_307_h.cir",
        1_172,
        "1b2f289deba22de22f492701a6730ef1a9b2be688099b66ee2c93ad9cf717fac",
        "2b9e43851db5fa15a9f6424b47424a23222c4c0692d3c2f906e5769b3103b4bf",
    ),
    (
        "bug_307_i.cir",
        480,
        "4d74799d1c37789f8740735ed7c24988e34e73818fe3b93ff45f60b8c1d60bc9",
        "0c6bc5bc4a13a511f66163745bb7a80c9cb12a41198ad5b5104f68e929216237",
    ),
    (
        "bug_307_i.lib1",
        228,
        "aac03ef8a09ddb99aa3b3b6f6aa6152ed7531b4cf4918b522d9b9f2a8007de47",
        "2e084709750590888d9f0015e7b35f29f714c969dd12931a7e4af3ca6e58d391",
    ),
    (
        "bug_307_i.lib2",
        280,
        "2fa7440e06ce352b530e640997e36c8200e5feee86e6e5e79987aaf2b38332e0",
        "8c2ee4a4a5d0ddd0de2c89bf14380bd79ef0395dae481bd4e6e471b640a3980a",
    ),
];
const XYCE_BUG307_RETAINED_OUTPUTS: [(&str, usize, &str, &str); 6] = [
    (
        "bug_307_d.cir.prn",
        306_701,
        "d757e3ffad74d451c22847d4f6179f63ea1c263c074a47f2f6b7f668f23f8070",
        "ebe6ec704c22124d446aa5fbcabde53e88f060a97cac8927705541c51879611b",
    ),
    (
        "bug_307_e.cir.prn",
        306_701,
        "d757e3ffad74d451c22847d4f6179f63ea1c263c074a47f2f6b7f668f23f8070",
        "ebe6ec704c22124d446aa5fbcabde53e88f060a97cac8927705541c51879611b",
    ),
    (
        "bug_307_f.cir.prn",
        306_701,
        "d55abf3b13631a17aabfe6d83cd629dbe39b972816bde0344e3525c5c54a9199",
        "f98be1f2910ed36ef4dfe9acfc0a48add52b9dcb3d78433b6b74b8809ffd9903",
    ),
    (
        "bug_307_g.cir.prn",
        306_701,
        "d55abf3b13631a17aabfe6d83cd629dbe39b972816bde0344e3525c5c54a9199",
        "f98be1f2910ed36ef4dfe9acfc0a48add52b9dcb3d78433b6b74b8809ffd9903",
    ),
    (
        "bug_307_h.cir.prn",
        306_701,
        "d55abf3b13631a17aabfe6d83cd629dbe39b972816bde0344e3525c5c54a9199",
        "f98be1f2910ed36ef4dfe9acfc0a48add52b9dcb3d78433b6b74b8809ffd9903",
    ),
    (
        "bug_307_i.cir.prn",
        306_701,
        "d757e3ffad74d451c22847d4f6179f63ea1c263c074a47f2f6b7f668f23f8070",
        "ebe6ec704c22124d446aa5fbcabde53e88f060a97cac8927705541c51879611b",
    ),
];

// BUG_864_SON is a bounded expected-error contract. Release 7.10 executes
// the deck, requires a nonzero exit, and accepts the run only when either
// complete output stream contains the unresolved-definition diagnostic. The
// retained `options` sidecar supplies the historical 30-second outer limit.
const XYCE_BUG864_CONTRACT: &str = "expected_failure_bug864_unresolved_subcircuit_parameter_build";
const XYCE_BUG864_PATH: &str = "Netlists/Certification_Tests/BUG_864_SON/bug_864_son.cir";
const XYCE_BUG864_RECORD: &str = "netlists/certification_tests/bug_864_son/bug_864_son.cir";
const XYCE_BUG864_UPSTREAM_REGRESSION_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const XYCE_BUG864_UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const XYCE_BUG864_UPSTREAM_DIAGNOSTIC: &str =
    "Unable to resolve parameter FOO found in .PARAM statement";
const XYCE_BUG864_HISTORICAL_TIMEOUT_MS: u128 = 30_000;
const XYCE_BUG864_HISTORICAL_RECORD_COUNT: usize = 7;
const XYCE_BUG864_HISTORICAL_RECORD_BYTES: usize = 1_687;
const XYCE_BUG864_HISTORICAL_RECORDS_SHA256: &str =
    "e9f815b0c4e0f002b505ef103e1bcb507640faf2989717b5cc154623bcccb583";
const XYCE_BUG864_HISTORICAL_RECORDS_BLAKE3: &str =
    "62db0b949913135bfe8d7911a8fa1e272edd1f930905458fbe2c1dd379dd5e3b";
const XYCE_BUG864_HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 7] = [
    (
        "Netlists/Certification_Tests/BUG_864_SON/CMakeLists.txt",
        1_816,
        "e1bfb80a50adf93e40c20cf88a23a940481b2ce744fbc322d9eebcd8ae355563",
        "1f3ec5c8d304b12d7f3ff6f11d8e37de64e8cb63540e55ce8f1556ec09d605b6",
    ),
    (
        "Netlists/Certification_Tests/BUG_864_SON/Manifest.txt",
        48,
        "9bc1a4cf78fcbdab9013fc1b233e1a1b6c24a8b4dd087f8d888068d57c836dc6",
        "fc99f03d3c3a5eed06aa8a239483d36a303aea541fca394b99edf0b8176771ef",
    ),
    (
        XYCE_BUG864_PATH,
        447,
        "71ba444757fc0add31c380fcff3f2dea34842236fce45de9c293311980695bac",
        "3463724af68f93fc11effd6f04344ffb7532a4173c10cfc7181f24ee7edd0dd0",
    ),
    (
        "Netlists/Certification_Tests/BUG_864_SON/bug_864_son.cir.sh",
        780,
        "6fac638aa87b177b8e6f37f39522336a34540bcf72c4f04c4099041d9b261ece",
        "b486481e992b4e9728e254e3115f6f6d891f9a77dbba20f396768d8bdc511875",
    ),
    (
        "Netlists/Certification_Tests/BUG_864_SON/options",
        13,
        "381cd29ca4d9097c73fccc5f46cea0c37bd3e71da803e56ccad41d8270de9c0e",
        "8e9c4c362e6a201344f7fd4b55680c6db23a1ba99121d41b9dae7573cff78b81",
    ),
    (
        "Netlists/Certification_Tests/BUG_864_SON/tags",
        36,
        "38081d2c7c83cc0bf7ff6b2430777e4cfabfca9019dc9638fc59395b9ffb1095",
        "851f6add160cd333cac9c53cb5ffd9d6f14f7f7bbd97185dfe930f14e30aede0",
    ),
    (
        "TestScripts/XyceRegression/Tools.pm",
        68_108,
        "5b5f86c02d46a1f3bdad5292e7e91d25a9e08e71490643d8d5ed7ae20f9d55e3",
        "13bd274632744ddc4b8baee680ddc9770902793ed7ee892ecdedd4dcb3828667",
    ),
];
const XYCE_BUG864_RETAINED_ARTIFACTS: [(&str, usize, &str, &str); 2] = [
    (
        "bug_864_son.cir",
        447,
        "71ba444757fc0add31c380fcff3f2dea34842236fce45de9c293311980695bac",
        "3463724af68f93fc11effd6f04344ffb7532a4173c10cfc7181f24ee7edd0dd0",
    ),
    (
        "options",
        13,
        "381cd29ca4d9097c73fccc5f46cea0c37bd3e71da803e56ccad41d8270de9c0e",
        "8e9c4c362e6a201344f7fd4b55680c6db23a1ba99121d41b9dae7573cff78b81",
    ),
];

// BUG_1025 is an active Release-7.10 error-exit wrapper. Its zero-byte deck
// must terminate without hanging and report that no analysis was specified.
const XYCE_BUG1025_CONTRACT: &str = "expected_failure_bug1025_no_analysis_selection";
const XYCE_BUG1025_PATH: &str = "Netlists/Certification_Tests/BUG_1025/null.cir";
const XYCE_BUG1025_RECORD: &str = "netlists/certification_tests/bug_1025/null.cir";
const XYCE_BUG1025_UPSTREAM_REGRESSION_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const XYCE_BUG1025_UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const XYCE_BUG1025_UPSTREAM_DIAGNOSTIC: &str = "No analysis specified";
const XYCE_BUG1025_HISTORICAL_TIMEOUT_MS: u128 = 30_000;
const XYCE_BUG1025_HISTORICAL_RECORD_COUNT: usize = 8;
const XYCE_BUG1025_HISTORICAL_RECORD_BYTES: usize = 1_889;
const XYCE_BUG1025_HISTORICAL_RECORDS_SHA256: &str =
    "d67ca4c781e711e0d29cc14727d80296cb6290beeff721e7490c7e12989db889";
const XYCE_BUG1025_HISTORICAL_RECORDS_BLAKE3: &str =
    "b1b71953813d7f00717edd434c09d5fb5a4ad6df8382ffe37fa22910e1d73369";
const XYCE_BUG1025_HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 8] = [
    (
        "Netlists/Certification_Tests/BUG_1025/CMakeLists.txt",
        1_356,
        "1ea70ec1cc0f40b76c7e209a35b9ccde6e2490cc3ff62a1cd15805d26accf941",
        "0d2c7be96c8440e8a53dd0a82a00872a1d030e0a9b83962fe0fc7b63fc4946f2",
    ),
    (
        "Netlists/Certification_Tests/BUG_1025/Manifest.txt",
        41,
        "e5558d15bca478eaa99c091ba4083f255b5a68238edd8cc642499aa60dd7b543",
        "370e2d8c4e57b14bc49664148d33b002e5e77b065c0309c1b11d8c31e05d9112",
    ),
    (
        "Netlists/Certification_Tests/BUG_1025/README",
        516,
        "e09c5d7175fcdfa718d02583f15ebf5a77a66b547c9d1ab4722ee96c4010832f",
        "9940ce3431afe2f3af6e146a306a51a8cbd178e9a30966e298a0607b37828867",
    ),
    (
        XYCE_BUG1025_PATH,
        0,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262",
    ),
    (
        "Netlists/Certification_Tests/BUG_1025/null.cir.sh",
        1_341,
        "b1dd593d0bcacd74b59bb9be54f577014d808cbeacca7ec3be8eac361f40b4ae",
        "0e4c4a07cf732f43f8a17cd54b9466b8a8245ccc5b22540a208e5889b6af5c9f",
    ),
    (
        "Netlists/Certification_Tests/BUG_1025/options",
        13,
        "381cd29ca4d9097c73fccc5f46cea0c37bd3e71da803e56ccad41d8270de9c0e",
        "8e9c4c362e6a201344f7fd4b55680c6db23a1ba99121d41b9dae7573cff78b81",
    ),
    (
        "Netlists/Certification_Tests/BUG_1025/tags",
        29,
        "0deb9da8f0cea8fd20aa6d68268d59713531be5740c7b3d33a1cd683f747eedd",
        "66a8be0b3e1d07fd4d908e7e6ed6ced3af2090e699f54236a98d857c60e05dd1",
    ),
    (
        "TestScripts/XyceRegression/Tools.pm",
        68_108,
        "5b5f86c02d46a1f3bdad5292e7e91d25a9e08e71490643d8d5ed7ae20f9d55e3",
        "13bd274632744ddc4b8baee680ddc9770902793ed7ee892ecdedd4dcb3828667",
    ),
];
const XYCE_BUG1025_RETAINED_ARTIFACTS: [(&str, usize, &str, &str); 3] = [
    (
        "README",
        516,
        "e09c5d7175fcdfa718d02583f15ebf5a77a66b547c9d1ab4722ee96c4010832f",
        "9940ce3431afe2f3af6e146a306a51a8cbd178e9a30966e298a0607b37828867",
    ),
    (
        "null.cir",
        0,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262",
    ),
    (
        "options",
        13,
        "381cd29ca4d9097c73fccc5f46cea0c37bd3e71da803e56ccad41d8270de9c0e",
        "8e9c4c362e6a201344f7fd4b55680c6db23a1ba99121d41b9dae7573cff78b81",
    ),
];

// BUG_636_SON is an active Release-7.10 error-exit regression. Its wrapper
// requires a nonzero simulator exit followed by these two ordered diagnostics
// for the incomplete two-field `.TRAN` line.
const XYCE_BUG636_CONTRACT: &str = "expected_failure_bug636_incomplete_tran_parse";
const XYCE_BUG636_PATH: &str = "Netlists/Certification_Tests/BUG_636_SON/bug636.cir";
const XYCE_BUG636_RECORD: &str = "netlists/certification_tests/bug_636_son/bug636.cir";
const XYCE_BUG636_UPSTREAM_REGRESSION_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const XYCE_BUG636_UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const XYCE_BUG636_UPSTREAM_DIAGNOSTIC_ARITY: &str = ".TRAN line has an unexpected number of fields";
const XYCE_BUG636_UPSTREAM_DIAGNOSTIC_IGNORED: &str = "Unrecognized dot line will be ignored";
const XYCE_BUG636_HISTORICAL_RECORD_COUNT: usize = 6;
const XYCE_BUG636_HISTORICAL_RECORD_BYTES: usize = 1_440;
const XYCE_BUG636_HISTORICAL_RECORDS_SHA256: &str =
    "b14b58e42be396acbaa972d89715777c8743ce460fdf5774af2b1559046f5aa8";
const XYCE_BUG636_HISTORICAL_RECORDS_BLAKE3: &str =
    "f3e0e44143ac1276a58d03c809c76187481b35000154e7dc5d5a2861619e806a";
const XYCE_BUG636_HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 6] = [
    (
        "Netlists/Certification_Tests/BUG_636_SON/CMakeLists.txt",
        1_151,
        "e408170ff2ef7e8c6a1104cfab1934ab1834b59900ade517e88da2df69df3e80",
        "63b719cd8f2cecc657e4ec8523ca6feedce9853f3e3d0d50333711f8bb21717d",
    ),
    (
        "Netlists/Certification_Tests/BUG_636_SON/Manifest.txt",
        30,
        "4cde8e876bb763f94faf9c81bf924b72c2fc11fa9d22f225a41c2b54d07bff3d",
        "2c384dbf7e7c4d812cf54f0cd4d4be1f53e7befd7be834243e3521384da3fd48",
    ),
    (
        XYCE_BUG636_PATH,
        111,
        "35c52026df1c0db86c2797c75f309a13e108d1f51f95b20632d64c72b8d67534",
        "19641c3140f2b965f3e85d1145d84475ccb21b3b75d92f87a625203d632a73a8",
    ),
    (
        "Netlists/Certification_Tests/BUG_636_SON/bug636.cir.sh",
        1_658,
        "a972c26fdac0afbe4b501bab71db56d07efe4b02e148788d8a7c531ae668bd79",
        "94ec9d0adc46f2b103f6be57a7a15154dbfb4fb52c4765d153791597e01c755c",
    ),
    (
        "Netlists/Certification_Tests/BUG_636_SON/tags",
        30,
        "345e0af2901a7957f378f2c04303c0d27cd9e7238e751e123101f23a3303bc7c",
        "3e70684e3759f6743f839f366245f8394859533a4505fa866bf8bd7d652e0a75",
    ),
    (
        "TestScripts/XyceRegression/Tools.pm",
        68_108,
        "5b5f86c02d46a1f3bdad5292e7e91d25a9e08e71490643d8d5ed7ae20f9d55e3",
        "13bd274632744ddc4b8baee680ddc9770902793ed7ee892ecdedd4dcb3828667",
    ),
];
const XYCE_BUG636_RETAINED_ARTIFACTS: [(&str, usize, &str, &str); 1] = [(
    "bug636.cir",
    111,
    "35c52026df1c0db86c2797c75f309a13e108d1f51f95b20632d64c72b8d67534",
    "19641c3140f2b965f3e85d1145d84475ccb21b3b75d92f87a625203d632a73a8",
)];

// BUG_206 is an active Release-7.10 error-exit regression. Its wrapper
// requires a nonzero simulator exit followed by these two ordered hierarchy
// diagnostics for the malformed X instance on source line three.
const XYCE_BUG206_CONTRACT: &str = "expected_failure_bug206_undefined_subcircuit_build";
const XYCE_BUG206_PATH: &str = "Netlists/Certification_Tests/BUG_206/bug_206.cir";
const XYCE_BUG206_RECORD: &str = "netlists/certification_tests/bug_206/bug_206.cir";
const XYCE_BUG206_UPSTREAM_REGRESSION_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const XYCE_BUG206_UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const XYCE_BUG206_UPSTREAM_DIAGNOSTIC_LOCATION: &str = "in file bug_206.cir at or near line 3";
const XYCE_BUG206_UPSTREAM_DIAGNOSTIC_UNDEFINED: &str =
    "Subcircuit 0 has not been defined for instance X1";
const XYCE_BUG206_HISTORICAL_RECORD_COUNT: usize = 7;
const XYCE_BUG206_HISTORICAL_RECORD_BYTES: usize = 1_656;
const XYCE_BUG206_HISTORICAL_RECORDS_SHA256: &str =
    "9687c32cbd6f3de14bc2bd104a60c702c95b013e6ada2094d30eca147c64966c";
const XYCE_BUG206_HISTORICAL_RECORDS_BLAKE3: &str =
    "dad83aa25b5a66edfe93f2485c62a1b7577e2ede066909f6b1a1a49247d31325";
const XYCE_BUG206_HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 7] = [
    (
        "Netlists/Certification_Tests/BUG_206/CMakeLists.txt",
        1_240,
        "4cdf84f6768618604e34ff6ee223587d80724517a122b1259ea2024578cd2d1d",
        "61de767d28ddaf73ab33590ac39b19dadd5a747e9985de691eb8da822926d874",
    ),
    (
        "Netlists/Certification_Tests/BUG_206/Manifest.txt",
        39,
        "750dc380137bfa3535b2451c20e5f28389434cd2bab8fdeef9d2c785c079f363",
        "3d2092627588fcd0aff7d3befc1da7f9bf054cd85e9b80a171c75e4fa9c1bfed",
    ),
    (
        "Netlists/Certification_Tests/BUG_206/README",
        505,
        "93f03e9620a7f61bd13286fd6ef41efa1b0942f12b9dadb3c95417335a4c4d5a",
        "afb37041e27da69687279655688ccbec94ca006152ea368f9e485c224d16c15d",
    ),
    (
        XYCE_BUG206_PATH,
        113,
        "028f5609a9fa41f0de85b029e3fa2c25e248777d97752859ba5b98753e6105c6",
        "35adbf753e4e0b1d287f4c0fb28380d80bcecb08a66a12c10a7a1bdbf69d1a4c",
    ),
    (
        "Netlists/Certification_Tests/BUG_206/bug_206.cir.sh",
        1_417,
        "3063b1a2bd51695594aba7cd216a01899df6b67f63653e31f6976380ed2bcf35",
        "d3335d2aa222d9d8d24992bbe10bfcbde15f09bcfd2987597665aa3a4ec37278",
    ),
    (
        "Netlists/Certification_Tests/BUG_206/tags",
        16,
        "fb8b1ab6aa8b694212335a76b1b87c077f22be7543f15c12de32a2da40b4f345",
        "a5f2cee6f41471429bc22c4c40d36881f4c11d2387b20adbdc14efe2509f6589",
    ),
    (
        "TestScripts/XyceRegression/Tools.pm",
        68_108,
        "5b5f86c02d46a1f3bdad5292e7e91d25a9e08e71490643d8d5ed7ae20f9d55e3",
        "13bd274632744ddc4b8baee680ddc9770902793ed7ee892ecdedd4dcb3828667",
    ),
];
const XYCE_BUG206_RETAINED_ARTIFACTS: [(&str, usize, &str, &str); 2] = [
    (
        "README",
        505,
        "93f03e9620a7f61bd13286fd6ef41efa1b0942f12b9dadb3c95417335a4c4d5a",
        "afb37041e27da69687279655688ccbec94ca006152ea368f9e485c224d16c15d",
    ),
    (
        "bug_206.cir",
        113,
        "028f5609a9fa41f0de85b029e3fa2c25e248777d97752859ba5b98753e6105c6",
        "35adbf753e4e0b1d287f4c0fb28380d80bcecb08a66a12c10a7a1bdbf69d1a4c",
    ),
];

// BUG_1116 is an active Release-7.10 error-exit regression. Its malformed
// diode card supplies only two tokens after the instance name, leaving the
// mandatory model field absent.
const XYCE_BUG1116_CONTRACT: &str = "expected_failure_bug1116_missing_diode_model_parse";
const XYCE_BUG1116_PATH: &str = "Netlists/Certification_Tests/BUG_1116/bug_1116.cir";
const XYCE_BUG1116_RECORD: &str = "netlists/certification_tests/bug_1116/bug_1116.cir";
const XYCE_BUG1116_UPSTREAM_REGRESSION_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const XYCE_BUG1116_UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const XYCE_BUG1116_UPSTREAM_DIAGNOSTIC: &str =
    "Model is required for device D1 and no valid model card found";
const XYCE_BUG1116_HISTORICAL_RECORD_COUNT: usize = 8;
const XYCE_BUG1116_HISTORICAL_RECORD_BYTES: usize = 1_899;
const XYCE_BUG1116_HISTORICAL_RECORDS_SHA256: &str =
    "a60748b07472db91a125d5dd60fbd1f00b66abaa55287eb0f808218e85fdaab7";
const XYCE_BUG1116_HISTORICAL_RECORDS_BLAKE3: &str =
    "96e9d35a37d8be9bfff2cbd8039319a12432c2488237abc2bdccc8045776d059";
const XYCE_BUG1116_HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 8] = [
    (
        "Netlists/Certification_Tests/BUG_1116/CMakeLists.txt",
        1_857,
        "6c8a721de3a07d25bfeb98045fc8053f00bdd4f29640b77dcbb7d11f40db1076",
        "21fb5b5cf97be9ed3904c998109118905177d8246a49a6646bffccb09bd490d1",
    ),
    (
        "Netlists/Certification_Tests/BUG_1116/Manifest.txt",
        49,
        "724f9dd504aee2d6aca2c6d7640db251f16c00bc76133af9ad7e6010b838cdee",
        "2f6fb0a097294b0faecf9c3f4352adbc5c89f64bb1de22b06b68adfa9c54eff0",
    ),
    (
        "Netlists/Certification_Tests/BUG_1116/README",
        441,
        "f9c58aafcb8043b1afd432804203eb29104bb246caa6db047d1f090267007089",
        "3d855941607b9cabe338a5cdb2a09ac0eecf3e3e9aa2c5ea768f49ce16198352",
    ),
    (
        XYCE_BUG1116_PATH,
        200,
        "e826c2b0cc5ccc49dc8219a95f03ba58a7be1d9a28d0204a9a7348b57a2388fd",
        "60eff5369669c79d686af8e2b9095cf57833291ddd0ff14880057aa8da3ca42f",
    ),
    (
        "Netlists/Certification_Tests/BUG_1116/bug_1116.cir.sh",
        1_382,
        "cca7d72c275633c40ca5e89fc90835acad51f323bb75303cdb8b7b24eb79056f",
        "631ad8201cb5484377cdbc2659e3443d9b103c3a936b4a284a4826a9c8c4f5da",
    ),
    (
        "Netlists/Certification_Tests/BUG_1116/options",
        13,
        "381cd29ca4d9097c73fccc5f46cea0c37bd3e71da803e56ccad41d8270de9c0e",
        "8e9c4c362e6a201344f7fd4b55680c6db23a1ba99121d41b9dae7573cff78b81",
    ),
    (
        "Netlists/Certification_Tests/BUG_1116/tags",
        36,
        "38081d2c7c83cc0bf7ff6b2430777e4cfabfca9019dc9638fc59395b9ffb1095",
        "851f6add160cd333cac9c53cb5ffd9d6f14f7f7bbd97185dfe930f14e30aede0",
    ),
    (
        "TestScripts/XyceRegression/Tools.pm",
        68_108,
        "5b5f86c02d46a1f3bdad5292e7e91d25a9e08e71490643d8d5ed7ae20f9d55e3",
        "13bd274632744ddc4b8baee680ddc9770902793ed7ee892ecdedd4dcb3828667",
    ),
];
const XYCE_BUG1116_RETAINED_ARTIFACTS: [(&str, usize, &str, &str); 3] = [
    (
        "README",
        441,
        "f9c58aafcb8043b1afd432804203eb29104bb246caa6db047d1f090267007089",
        "3d855941607b9cabe338a5cdb2a09ac0eecf3e3e9aa2c5ea768f49ce16198352",
    ),
    (
        "bug_1116.cir",
        200,
        "e826c2b0cc5ccc49dc8219a95f03ba58a7be1d9a28d0204a9a7348b57a2388fd",
        "60eff5369669c79d686af8e2b9095cf57833291ddd0ff14880057aa8da3ca42f",
    ),
    (
        "options",
        13,
        "381cd29ca4d9097c73fccc5f46cea0c37bd3e71da803e56ccad41d8270de9c0e",
        "8e9c4c362e6a201344f7fd4b55680c6db23a1ba99121d41b9dae7573cff78b81",
    ),
];

// ISSUE_61 is the active Xyce regression for a behavioral current expression
// attempting to read the nonexistent MNA branch of another current-output B
// source. Release-7.10 requires a clean error instead of the historical crash.
const XYCE_ISSUE61_CONTRACT: &str =
    "expected_failure_issue61_behavioral_lead_current_reference_build";
const XYCE_ISSUE61_PATH: &str = "Netlists/Certification_Tests/ISSUE_61/issue61.cir";
const XYCE_ISSUE61_RECORD: &str = "netlists/certification_tests/issue_61/issue61.cir";
const XYCE_ISSUE61_UPSTREAM_REGRESSION_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const XYCE_ISSUE61_UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const XYCE_ISSUE61_UPSTREAM_DIAGNOSTIC: &str =
    "Device instance B2: Problem with value for B1 in B2";
const XYCE_ISSUE61_HISTORICAL_RECORD_COUNT: usize = 6;
const XYCE_ISSUE61_HISTORICAL_RECORD_BYTES: usize = 1_427;
const XYCE_ISSUE61_HISTORICAL_RECORDS_SHA256: &str =
    "c778d83c506695fce669b54a041900ecd612ada666b779744daf59613b38c786";
const XYCE_ISSUE61_HISTORICAL_RECORDS_BLAKE3: &str =
    "4f09ca56e3ef3dc08c4e1a81bcd1da102f7aac7905e97d7cc0ea5781cfd66e4f";
const XYCE_ISSUE61_HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 6] = [
    (
        "Netlists/Certification_Tests/ISSUE_61/CMakeLists.txt",
        1_173,
        "a365a140d5dfc6e0be56f4bdcf8e5b1b5336445be345ddcf1378aacb4c7ea040",
        "255592e5c1c2ca5ba104815776be904c0c596ac97727694705cd6387393c76a5",
    ),
    (
        "Netlists/Certification_Tests/ISSUE_61/Manifest.txt",
        32,
        "7d673910c3536ef6710395dd10720d472fd9b3b9bfdf5f84dca099d527f90841",
        "bd89f5857652ce7610d23497f7cbef17fdb24504e15949ae2814b63459510910",
    ),
    (
        XYCE_ISSUE61_PATH,
        127,
        "b47aa5c601fe34acc7750ff468e3e71c5c65949f1488691fc182c0ef0299d502",
        "91e85ed903e6ba2cc5e0fbaee08ddb074357c11f6cbd5c689bdff4b7226becfe",
    ),
    (
        "Netlists/Certification_Tests/ISSUE_61/issue61.cir.sh",
        1_462,
        "3703fba38de711ad606ee4b7b28dadc123e2148247519dbdd4c19c50ec41db6b",
        "c760f796c158381dd18004a52feb76290b09f02b720b9f0debeab3cc32379776",
    ),
    (
        "Netlists/Certification_Tests/ISSUE_61/tags",
        50,
        "eb1565476d986a5b5fea167d0d8f24d9bcb644faf7ebc7f2627d949f04818ad2",
        "41b6b8d29add052502d5727e57b2cbf99f2089d49358a995363eb47ea29b8a8e",
    ),
    (
        "TestScripts/XyceRegression/Tools.pm",
        68_108,
        "5b5f86c02d46a1f3bdad5292e7e91d25a9e08e71490643d8d5ed7ae20f9d55e3",
        "13bd274632744ddc4b8baee680ddc9770902793ed7ee892ecdedd4dcb3828667",
    ),
];
const XYCE_ISSUE61_RETAINED_ARTIFACTS: [(&str, usize, &str, &str); 1] = [(
    "issue61.cir",
    127,
    "b47aa5c601fe34acc7750ff468e3e71c5c65949f1488691fc182c0ef0299d502",
    "91e85ed903e6ba2cc5e0fbaee08ddb074357c11f6cbd5c689bdff4b7226becfe",
)];

// BUG_784 is an archived error-exit regression: Release-7.10 retains its
// wrapper and exact ordered diagnostic, while `tags=exclude` deliberately
// leaves its generated CMake file without an active CTest registration.
const XYCE_BUG784_CONTRACT: &str = "archived_expected_failure_bug784_duplicate_subcircuit_port";
const XYCE_BUG784_PATH: &str = "Netlists/Certification_Tests/BUG_784/bug_784.cir";
const XYCE_BUG784_RECORD: &str = "netlists/certification_tests/bug_784/bug_784.cir";
const XYCE_BUG784_UPSTREAM_REGRESSION_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const XYCE_BUG784_UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const XYCE_BUG784_UPSTREAM_DIAGNOSTIC_PREFIX: &str = "in file bug_784.cir at or near line 7";
const XYCE_BUG784_UPSTREAM_DIAGNOSTIC: &str = "Duplicate node in .SUBCKT line: b";
const XYCE_BUG784_HISTORICAL_TAGS: &[u8] = b"exclude\n";
const XYCE_BUG784_HISTORICAL_RECORD_COUNT: usize = 7;
const XYCE_BUG784_HISTORICAL_RECORD_BYTES: usize = 1_654;
const XYCE_BUG784_HISTORICAL_RECORDS_SHA256: &str =
    "c26641ba9051f33f9a679b6e0a838571a3398cc8ff2eef563e5e0e15faf48dbc";
const XYCE_BUG784_HISTORICAL_RECORDS_BLAKE3: &str =
    "0dc66de822046b9d5910a754755e979eece906bb52e9141e877bd98336dc7e78";
const XYCE_BUG784_HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 7] = [
    (
        "Netlists/Certification_Tests/BUG_784/CMakeLists.txt",
        221,
        "0a9a790e450a09c7fbd82dbb1dc3364381138f241c48e3b1f75817fd373510ed",
        "d79d375b1f7b5ac8830c1ecb8cf8a599ede1a8d49a1323ba8a41effcc5fceaa1",
    ),
    (
        "Netlists/Certification_Tests/BUG_784/Manifest.txt",
        39,
        "43084f444d2f44172f2d5d27c0194d7aae20ab7e09ecb0820ff059a44121a86d",
        "4d51db0a1643a4a5e8d4ad61871e43b261b728beafac535f8f905f6950467af8",
    ),
    (
        "Netlists/Certification_Tests/BUG_784/README",
        714,
        "9da94f156415b21ac76eb9c7501af4e6aa98c6263c6309b38650944c213253ad",
        "e9e15644dfbf3f69303cf2727151c40a0107ff530e978ae6bbc261829c44f1aa",
    ),
    (
        XYCE_BUG784_PATH,
        102,
        "e97755c1e31c7bc9b061beb1baf48f5b0febca2001129b6f47f3d28934cdad68",
        "e44835e871303f4d78ee52e9d4d6239bf89efe295c454c5091f230d03c690690",
    ),
    (
        "Netlists/Certification_Tests/BUG_784/bug_784.cir.sh",
        1_401,
        "2d9c4ac45e85dd6ed79bc7a91f87bda28be6448c325e1070a2d65d0afa052c0a",
        "ba73d0b67cd2981c83aa02b4b826516ed786bdca6dec36454c5ff54ee25ef69c",
    ),
    (
        "Netlists/Certification_Tests/BUG_784/tags",
        8,
        "4e6c23fcd9140520f152d969561caee073952c6027b65f96b07cd01da70432e1",
        "c76ce284ba1075a571da61a7e0227cb9734e4965b3ebdf09e920a45dccc758f5",
    ),
    (
        "TestScripts/XyceRegression/Tools.pm",
        68_108,
        "5b5f86c02d46a1f3bdad5292e7e91d25a9e08e71490643d8d5ed7ae20f9d55e3",
        "13bd274632744ddc4b8baee680ddc9770902793ed7ee892ecdedd4dcb3828667",
    ),
];
const XYCE_BUG784_RETAINED_ARTIFACTS: [(&str, usize, &str, &str); 2] = [
    (
        "README",
        714,
        "9da94f156415b21ac76eb9c7501af4e6aa98c6263c6309b38650944c213253ad",
        "e9e15644dfbf3f69303cf2727151c40a0107ff530e978ae6bbc261829c44f1aa",
    ),
    (
        "bug_784.cir",
        102,
        "e97755c1e31c7bc9b061beb1baf48f5b0febca2001129b6f47f3d28934cdad68",
        "e44835e871303f4d78ee52e9d4d6239bf89efe295c454c5091f230d03c690690",
    ),
];

// DIODE_ANALYTIC is an active three-member Release-7.10 generated-gold
// family. Each wrapper evaluates the analytic diode law on the simulator's
// own transient output grid before invoking xyce_verify.
const XYCE_DIODE_ANALYTIC_UPSTREAM_REGRESSION_COMMIT: &str =
    "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const XYCE_DIODE_ANALYTIC_UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const XYCE_DIODE_ANALYTIC_HISTORICAL_RECORD_COUNT: usize = 13;
const XYCE_DIODE_ANALYTIC_HISTORICAL_RECORD_BYTES: usize = 3_020;
const XYCE_DIODE_ANALYTIC_HISTORICAL_RECORDS_SHA256: &str =
    "2ef729affb94fdc248b330b9d40e1e75f290e3507b1f3ba181623db3e569642a";
const XYCE_DIODE_ANALYTIC_HISTORICAL_RECORDS_BLAKE3: &str =
    "d4e68c7e1ca1ebbc47d0e3ed2014e0a870633689dd3ad32c869e86cc2d5c2215";
const XYCE_DIODE_ANALYTIC_HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 13] = [
    (
        "Netlists/DIODE_ANALYTIC/CMakeLists.txt",
        5_169,
        "4ca3c2e90c80726ab9a645669c4968363cb0509b080e038bd8b2b3f65a8a701d",
        "83c21dd8dae4f2deb68d30bf10d7c0c4b7b420f9f346288110a398ee2409b7dc",
    ),
    (
        "Netlists/DIODE_ANALYTIC/Manifest.txt",
        200,
        "01d309e6bb67b8fde6762636603e41617bc8ae07bcb4239e4684a9cdf619d43d",
        "459f943734c2cf14b53b13b8490aa7efe87602cffdeea91933740fa38d8c2afc",
    ),
    (
        "Netlists/DIODE_ANALYTIC/breakdown_model.cir",
        2_337,
        "c428a046c15bce99cb0f75a8346327a2becc1adb4c37b175c6d07da133159249",
        "b25207e26ddd7e67c50980c0b53034c5a71b6f344040da120a71e5ffaa96cea0",
    ),
    (
        "Netlists/DIODE_ANALYTIC/breakdown_model.cir.gs.pl",
        507,
        "9b469fddda2dfc87ccbe4beba1ed54d40dceeb445107e486eae5189350a4fc0d",
        "a1a2dd1bca6f8d55b5b5e146d9b2a3fbd5548ada6b525571bb6379d78139de4b",
    ),
    (
        "Netlists/DIODE_ANALYTIC/breakdown_model.cir.sh",
        1_277,
        "bb1ca31a6ff2c11a675a7b871bd6caafe01ab9590ef030c1b55e405f1ebcf3ee",
        "4a9785f678b7a0c1b76e7c5877186aae05cc07193fd59d5a471f3a747a899730",
    ),
    (
        "Netlists/DIODE_ANALYTIC/forward_model.cir",
        1_442,
        "f5e090a36963633e2be6dc76dee709dc51efb4b9fa97dc5d93667c25b64dca9c",
        "25153718dd56bbe9f05d3e5c86769420de8b55bcf6d7c218911042dc7e0bda44",
    ),
    (
        "Netlists/DIODE_ANALYTIC/forward_model.cir.gs.pl",
        511,
        "dbf061409dc5371deafaef55d0687ed07e4b8770450e2d276b5e5a9f9cbed830",
        "b1724024d906f783c548c812d87b3b12b0453fa0dd9f8fb8d9cabc8e87362d6f",
    ),
    (
        "Netlists/DIODE_ANALYTIC/forward_model.cir.sh",
        1_273,
        "7fc7eea6d2164aaeefd80ea2d86b887f9f3be3d43a0329df1ea767c164cb2988",
        "69657a1309232ef50610e76a5599ba7e0f149a6c1415b1709d787ea0d878698e",
    ),
    (
        "Netlists/DIODE_ANALYTIC/reverse_model.cir",
        2_553,
        "84ec00feef59479221276fd0c0b4421135b7ed62bfa0b8f36706acb67cdc63b4",
        "99d3e4ed8f1371a44905644841db8f58ec74dac53642f0b6a9a2cfdaf31183f0",
    ),
    (
        "Netlists/DIODE_ANALYTIC/reverse_model.cir.gs.pl",
        759,
        "7025ff54a22d7be56a1942ab5cfd7e645c40be9c9b4194961e254d62ebb98967",
        "152a090659c78726bb54578dd92e686071e0f6ff35a2faad550b582f081393c8",
    ),
    (
        "Netlists/DIODE_ANALYTIC/reverse_model.cir.sh",
        3_402,
        "c84ffe260bd68460c229ebdc9c81cd6b2954226d143eabe1920fa62d1829a43f",
        "17e0a7af9157cdf8919c46ff5302a31ade92bb6078aef4cc2d3e28f5e05f1c19",
    ),
    (
        "Netlists/DIODE_ANALYTIC/tags",
        64,
        "1ff198c2fd9e6222408cd30d4cdce15c2184fcb0ae18a6f585a91ca9d92261ee",
        "1c58bd56195603e452af41dcd6c30cecb5539bc8eaaafb7bb4641410ab204943",
    ),
    (
        "TestScripts/xyce_verify.pl",
        59_566,
        "6e5f84b1646b30d0e12879848d7653584b39472d640a14916ae8fda6e1df12b3",
        "5eadb6dab06ed3091ea114146bd4a574de83784f87be9843ad7b721b0a793665",
    ),
];
const XYCE_DIODE_ANALYTIC_RETAINED_ARTIFACTS: [(&str, usize, &str, &str); 3] = [
    (
        "breakdown_model.cir",
        2_337,
        "c428a046c15bce99cb0f75a8346327a2becc1adb4c37b175c6d07da133159249",
        "b25207e26ddd7e67c50980c0b53034c5a71b6f344040da120a71e5ffaa96cea0",
    ),
    (
        "forward_model.cir",
        1_442,
        "f5e090a36963633e2be6dc76dee709dc51efb4b9fa97dc5d93667c25b64dca9c",
        "25153718dd56bbe9f05d3e5c86769420de8b55bcf6d7c218911042dc7e0bda44",
    ),
    (
        "reverse_model.cir",
        2_553,
        "84ec00feef59479221276fd0c0b4421135b7ed62bfa0b8f36706acb67cdc63b4",
        "99d3e4ed8f1371a44905644841db8f58ec74dac53642f0b6a9a2cfdaf31183f0",
    ),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceDiodeAnalyticKind {
    Forward,
    Reverse,
    Breakdown,
}

impl XyceDiodeAnalyticKind {
    const ALL: [Self; 3] = [Self::Forward, Self::Reverse, Self::Breakdown];

    fn file_name(self) -> &'static str {
        match self {
            Self::Forward => "forward_model.cir",
            Self::Reverse => "reverse_model.cir",
            Self::Breakdown => "breakdown_model.cir",
        }
    }

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
const XYCE_BJT_ANALYTIC_UPSTREAM_REGRESSION_COMMIT: &str =
    "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const XYCE_BJT_ANALYTIC_UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const XYCE_BJT_ANALYTIC_HISTORICAL_RECORD_COUNT: usize = 10;
const XYCE_BJT_ANALYTIC_HISTORICAL_RECORD_BYTES: usize = 2_271;
const XYCE_BJT_ANALYTIC_HISTORICAL_RECORDS_SHA256: &str =
    "28b5f42e510263b778fdc028b4bade56321b399517fb7bed7f5a19af4042ccb2";
const XYCE_BJT_ANALYTIC_HISTORICAL_RECORDS_BLAKE3: &str =
    "e6a987bf12645161566c4771e7fb7a5ef13f1c5b55e0b5caa7f55ebc75d9f6bd";
const XYCE_BJT_ANALYTIC_HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 10] = [
    (
        "Netlists/BJT_ANALYTIC/CMakeLists.txt",
        3_425,
        "99eb326fa8b22ce6722049a3b2b55dcd0dcb030b6b9b52f433e00cd25db49204",
        "bfcea7ff71d2deffdafa6a61701530b1e5b97b3ac5d0f6ea3cf6cd4f379c852a",
    ),
    (
        "Netlists/BJT_ANALYTIC/Manifest.txt",
        113,
        "c1a6f817863a87284722a98b4922ae0e281c7adf6efb04a8e3e7484fbd678683",
        "b734fd40ccc58c44d8b63f1e1f40386e51408bbdde09556a70fa26e698c805cd",
    ),
    (
        "Netlists/BJT_ANALYTIC/ramp_test1.cir",
        2_906,
        "f5bb4d0e330f0bbb2f5683a870e1312c1de364184dce6aafb147d87e940bf19e",
        "d4ee18831b6631da058ca151909d5b6937a1e28ec302e938bc6013fedbf698a6",
    ),
    (
        "Netlists/BJT_ANALYTIC/ramp_test1.cir.gs.pl",
        711,
        "8f8895cbe88c726e9bb3ced49de5d3c447879d469dbe1e1fa27178b6c11f902a",
        "6b21abad73ebaf159db30d086d2ea1c222cdd9446c250777fe04645f608e93af",
    ),
    (
        "Netlists/BJT_ANALYTIC/ramp_test1.cir.sh",
        1_278,
        "f53275edade218e6c66cfe4e05d59d8961ae51aa24d0921437d1f8ae61a54c83",
        "1186e29aaef87324169947df094be3631c4de64e3baa35cbd03c89b738618f51",
    ),
    (
        "Netlists/BJT_ANALYTIC/ramp_test2.cir",
        2_788,
        "4117823840f391e2e11481991d86efd58583043bbfa1e93f20808d8d77f89ef2",
        "e58df196d817bc4118193419caf8d91d58e24a80eb0bbe608990407b0eca84f9",
    ),
    (
        "Netlists/BJT_ANALYTIC/ramp_test2.cir.gs.pl",
        537,
        "d5f254b62ec976878da825af831ebe2f8811aecdc4592202c0f71bbe2e7f390f",
        "e485c5c8c14d72375760566dc861252a72e306b209cf78f8c135c42e11422725",
    ),
    (
        "Netlists/BJT_ANALYTIC/ramp_test2.cir.sh",
        1_278,
        "de426dba993333412707c9c1bd3e5d1f697eabbaaf368675d5a32558f7ab960a",
        "d321b54657c8f9ecddb9c37c6c5e313921dbe2675ea882d1033624bfe55ff4db",
    ),
    (
        "Netlists/BJT_ANALYTIC/tags",
        68,
        "c248a3ff42cc089d9e4cf2412af952909fda19e3f6dc17c1e75f185fc6b2ef9f",
        "a836eccaa84bdef1d96d6a8fc6a751b4113fa2ab088da0c662c63dee2faac9b7",
    ),
    (
        "TestScripts/xyce_verify.pl",
        59_566,
        "6e5f84b1646b30d0e12879848d7653584b39472d640a14916ae8fda6e1df12b3",
        "5eadb6dab06ed3091ea114146bd4a574de83784f87be9843ad7b721b0a793665",
    ),
];
const XYCE_BJT_ANALYTIC_RETAINED_ARTIFACTS: [(&str, usize, &str, &str); 2] = [
    (
        "ramp_test1.cir",
        2_906,
        "f5bb4d0e330f0bbb2f5683a870e1312c1de364184dce6aafb147d87e940bf19e",
        "d4ee18831b6631da058ca151909d5b6937a1e28ec302e938bc6013fedbf698a6",
    ),
    (
        "ramp_test2.cir",
        2_788,
        "4117823840f391e2e11481991d86efd58583043bbfa1e93f20808d8d77f89ef2",
        "e58df196d817bc4118193419caf8d91d58e24a80eb0bbe608990407b0eca84f9",
    ),
];

const XYCE_NMOS_ANALYTIC_UPSTREAM_REGRESSION_COMMIT: &str =
    "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const XYCE_NMOS_ANALYTIC_UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const XYCE_NMOS_ANALYTIC_HISTORICAL_RECORD_COUNT: usize = 7;
const XYCE_NMOS_ANALYTIC_HISTORICAL_RECORD_BYTES: usize = 1_593;
const XYCE_NMOS_ANALYTIC_HISTORICAL_RECORDS_SHA256: &str =
    "d366c275ef0b7131c5d7342f35a999ce4316e28eacf42189f82c34df760be8bf";
const XYCE_NMOS_ANALYTIC_HISTORICAL_RECORDS_BLAKE3: &str =
    "2e4b198e215c293d06d683c56bc6c8508aa480e44e0f5db6984a2660c267cb6c";
const XYCE_NMOS_ANALYTIC_HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 7] = [
    (
        "Netlists/NMOS_ANALYTIC/CMakeLists.txt",
        1_887,
        "4ed6b055fbda88059dbfdaca044ab84a75dbb3992c65a836470a836ff144d889",
        "db4f0e1f9b040225de65511958296b8c185e89e39785d36cd9ee67f12628f311",
    ),
    (
        "Netlists/NMOS_ANALYTIC/Manifest.txt",
        68,
        "aaac9d14e9b8634e47d5fa507edd18ad35c1e0ace6cf9b3f41c42a068d679890",
        "48a10aee6277dec173f5d239697819cccdeb21d4bf280b4e9ade5d8ca57ad9e3",
    ),
    (
        "Netlists/NMOS_ANALYTIC/mosfet_level1.cir",
        2_029,
        "b4cffa7173991ad37e46b005fb0761d3b07cd9aa3d8734554b0790367b583048",
        "fa694e4e2ec7645fad00286f7cda19beadb6640aa6d0e6a978502f1c1b8885c6",
    ),
    (
        "Netlists/NMOS_ANALYTIC/mosfet_level1.cir.gs.pl",
        611,
        "5c9a6132182064eb7468112b20a3b1890a316ef64e2b5e79dc8ab50d9e8f81dd",
        "beb718b4ee596fbd1dc63e958f96661d33bce1b84803bee51bc5df86ac8f7ebe",
    ),
    (
        "Netlists/NMOS_ANALYTIC/mosfet_level1.cir.sh",
        1_273,
        "6fda3ce1c9bb53b4a7482b9bb38931bd2a5827a0e841a30afa1f2bc5cd34987d",
        "e8eeeb415a41d87acb159a36e0c389081331dad7a3c860ee7d7b3b9a8942a2b5",
    ),
    (
        "Netlists/NMOS_ANALYTIC/tags",
        46,
        "a725c65e1fba0d3241656fedb9caf217f464e8a2b9e14b21bf823f43f83c1047",
        "4933a5eed19ded7b1fe188959c216a5d1d7ab77b700e72aeb6fc8ae583e27923",
    ),
    (
        "TestScripts/xyce_verify.pl",
        59_566,
        "6e5f84b1646b30d0e12879848d7653584b39472d640a14916ae8fda6e1df12b3",
        "5eadb6dab06ed3091ea114146bd4a574de83784f87be9843ad7b721b0a793665",
    ),
];
const XYCE_NMOS_ANALYTIC_RETAINED_ARTIFACTS: [(&str, usize, &str, &str); 1] = [(
    "mosfet_level1.cir",
    2_029,
    "b4cffa7173991ad37e46b005fb0761d3b07cd9aa3d8734554b0790367b583048",
    "fa694e4e2ec7645fad00286f7cda19beadb6640aa6d0e6a978502f1c1b8885c6",
)];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceLegacyDeviceAnalyticKind {
    BjtRamp1,
    BjtRamp2,
    NmosLevel1,
}

impl XyceLegacyDeviceAnalyticKind {
    const ALL: [Self; 3] = [Self::BjtRamp1, Self::BjtRamp2, Self::NmosLevel1];

    fn family(self) -> &'static str {
        match self {
            Self::BjtRamp1 | Self::BjtRamp2 => "BJT_ANALYTIC",
            Self::NmosLevel1 => "NMOS_ANALYTIC",
        }
    }

    fn file_name(self) -> &'static str {
        match self {
            Self::BjtRamp1 => "ramp_test1.cir",
            Self::BjtRamp2 => "ramp_test2.cir",
            Self::NmosLevel1 => "mosfet_level1.cir",
        }
    }

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
    owner_content_blake3: &'static str,
    control_content_blake3: &'static str,
    wrapper_path: &'static str,
    wrapper_bytes: usize,
    wrapper_sha256: &'static str,
    wrapper_blake3: &'static str,
    kind: XyceAbmLookupKind,
}

const XYCE_ABM_LOOKUP_ORDER_CASES: [XyceAbmLookupOrderCaseSpec; 2] = [
    XyceAbmLookupOrderCaseSpec {
        family: "ABM_SPLINES/akimaOutOfOrder",
        owner_path: "Netlists/ABM_SPLINES/akimaOutOfOrder.cir",
        control_path: "Netlists/ABM_SPLINES/akimaOutOfOrder_baseline.cir",
        owner_record: "netlists/abm_splines/akimaoutoforder.cir",
        control_record: "netlists/abm_splines/akimaoutoforder_baseline.cir",
        owner_content_blake3: "351e7255510860fc041b6519fdfad5bf860b0256199789d5406994cf67dcb7cf",
        control_content_blake3: "c6c9d2f35af02f9d42b00117baadedb3f41fb9ba396360e8bd2977848ae20ca4",
        wrapper_path: "Netlists/ABM_SPLINES/akimaOutOfOrder.cir.sh",
        wrapper_bytes: 1_523,
        wrapper_sha256: "d5618b6591666512ee1d7074217643118e08e6756f4dddc22781b1b7d0e96e3b",
        wrapper_blake3: "bba4bda064e9d9073b5815500edc57a3a6ebfccfb0d44357d7fbe988dbf0f17d",
        kind: XyceAbmLookupKind::Akima,
    },
    XyceAbmLookupOrderCaseSpec {
        family: "ABM_SPLINES/tableOutOfOrder2",
        owner_path: "Netlists/ABM_SPLINES/tableOutOfOrder2.cir",
        control_path: "Netlists/ABM_SPLINES/tableOutOfOrder2_baseline.cir",
        owner_record: "netlists/abm_splines/tableoutoforder2.cir",
        control_record: "netlists/abm_splines/tableoutoforder2_baseline.cir",
        owner_content_blake3: "2299905d9cbb3addbf77738748dee112c21e3cce71271a546e4f0943a40d2da5",
        control_content_blake3: "7570b3e44aeb9343c721fecc93be83a3169fb80f1750c1fbe5bea814ad0fadfd",
        wrapper_path: "Netlists/ABM_SPLINES/tableOutOfOrder2.cir.sh",
        wrapper_bytes: 1_525,
        wrapper_sha256: "2e12a860cd86c530caceee6744697e73292bbaaba3c12bdc068e26726f5eaed8",
        wrapper_blake3: "299a94323e811a857b82857201c7c990ebc93f289e69a55dfc91e93212648312",
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
    owner_content_blake3: &'static str,
    control_content_blake3: &'static str,
    wrapper_path: &'static str,
    wrapper_bytes: usize,
    wrapper_sha256: &'static str,
    wrapper_blake3: &'static str,
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
        owner_content_blake3: "a702b96fb4dd7772b74316c0ea3479e81bebb06a1982436cdeb857d461411099",
        control_content_blake3: "8bc43c93af51247e02dad6a271d3252d656a52851b0985dae7f34332cb2e32d0",
        wrapper_path: "Netlists/ABM_FREQ/abmfreq1.cir.sh",
        wrapper_bytes: 2_790,
        wrapper_sha256: "6e1662faee760fadac28bb36eea179d3e95f274b9d18d8b2b85febd800a131d7",
        wrapper_blake3: "22c4ed967cfc47ec81ca28dfbadc990bf9005a4177168960ed7e8b51d3c0d4c0",
        kind: XyceAbmFrequencyKind::BehavioralCurrent,
        variable: XyceAbmFrequencyVariable::Freq,
    },
    XyceAbmFrequencyCaseSpec {
        family: "ABM_FREQ/abmhertz1",
        owner_path: "Netlists/ABM_FREQ/abmhertz1.cir",
        control_path: "Netlists/ABM_FREQ/abmhertz1_data.cir",
        owner_record: "netlists/abm_freq/abmhertz1.cir",
        control_record: "netlists/abm_freq/abmhertz1_data.cir",
        owner_content_blake3: "c0e1ec2e949796a5a15d894c5ed601bf2b6aa5ea1a5882afacc05ef7edfec298",
        control_content_blake3: "69323efca8360bbcbef2c9395d6612d3213cc0c770e603336f2702c9fdb7a06e",
        wrapper_path: "Netlists/ABM_FREQ/abmhertz1.cir.sh",
        wrapper_bytes: 2_792,
        wrapper_sha256: "5e04c31574e5d7a9cebbe77243dc71b0bae9894272aaadf92d03ba56a0de39dc",
        wrapper_blake3: "d5d21c88b6eadd045ab7045b9f0d078b0979ba2e7cf5123a0a3aea6fd65db1c7",
        kind: XyceAbmFrequencyKind::BehavioralCurrent,
        variable: XyceAbmFrequencyVariable::Hertz,
    },
    XyceAbmFrequencyCaseSpec {
        family: "ABM_FREQ/RC_simple",
        owner_path: "Netlists/ABM_FREQ/RC_simple.cir",
        control_path: "Netlists/ABM_FREQ/RC_data.cir",
        owner_record: "netlists/abm_freq/rc_simple.cir",
        control_record: "netlists/abm_freq/rc_data.cir",
        owner_content_blake3: "0fba61011546c024c009d5c3206bff30d2ec36c65993115bc7f3f362a1aa2249",
        control_content_blake3: "09b2d92fd9bdc87751052bdc3707402206c51b449616f915d6ce510ea37eda8c",
        wrapper_path: "Netlists/ABM_FREQ/RC_simple.cir.sh",
        wrapper_bytes: 2_785,
        wrapper_sha256: "dbef44efb30c9fff33bf4bd1e9d60adcfcc664835164d55a06819a20b0790443",
        wrapper_blake3: "f5eb8df6e647999d8f597e91cff6dc5b617fb96e42fed1fa87fb791cee382e14",
        kind: XyceAbmFrequencyKind::Resistor,
        variable: XyceAbmFrequencyVariable::Freq,
    },
    XyceAbmFrequencyCaseSpec {
        family: "ABM_FREQ/RC_simple_hertz",
        owner_path: "Netlists/ABM_FREQ/RC_simple_hertz.cir",
        control_path: "Netlists/ABM_FREQ/RC_data_hertz.cir",
        owner_record: "netlists/abm_freq/rc_simple_hertz.cir",
        control_record: "netlists/abm_freq/rc_data_hertz.cir",
        owner_content_blake3: "b972b03e5f444fdad0271531cdb3d75a549c705dbdf0f5869e96249f8f219929",
        control_content_blake3: "320de333e3ebf9a965bea123f15a44e7f5c4df55ccaa62e811574cc82f4fe8ae",
        wrapper_path: "Netlists/ABM_FREQ/RC_simple_hertz.cir.sh",
        wrapper_bytes: 2_797,
        wrapper_sha256: "680a73dfc6979be0e58e95cac84265a0a1684e1cffa54f5db15aa7163c38e313",
        wrapper_blake3: "01fc0372d1f8e1cece405eff20fe3f01f69e4184931f72451590fdb4483b06da",
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
    owner_content_blake3: &'static str,
    baseline_content_blake3: &'static str,
}

const XYCE_BUG1043_AC_DATA_PARAMETER_CASE: XyceBug1043AcDataParameterCaseSpec =
    XyceBug1043AcDataParameterCaseSpec {
        family: "Certification_Tests/BUG_1043_SON/RC_AC_params",
        owner_path: XYCE_BUG1043_OWNER_PATH,
        baseline_path: XYCE_BUG1043_EXPRESSION_BASELINE_PATH,
        owner_record: XYCE_BUG1043_OWNER_RECORD,
        baseline_record: XYCE_BUG1043_EXPRESSION_BASELINE_RECORD,
        owner_content_blake3: "9af9c44bb2d799ecc2e1bf25ea9535c48cdac2323e81c24f0c3372b2cfbae057",
        baseline_content_blake3: "0a049b4a280380dbf2d3027e5aaef6fd4ea97be804d1f1f9371799b0b3d1485d",
    };

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct XyceSourceMultiplicityCaseSpec {
    family: &'static str,
    owner_record: &'static str,
    baseline_record: &'static str,
    owner_content_blake3: &'static str,
    baseline_content_blake3: &'static str,
    wrapper_path: &'static str,
    wrapper_bytes: usize,
    wrapper_sha256: &'static str,
    wrapper_blake3: &'static str,
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
        owner_content_blake3: "f4b552bd15c90721758eeca377d25af9cbe440d8d3fe2c6b4d74f1183b80aa43",
        baseline_content_blake3: "79ed9fce2f03e8f4207f40e481c0104d2d7056dabb7724ba6d8e29a87c9e3fa6",
        wrapper_path: "Netlists/BSRC/bsrc1_m.cir.sh",
        wrapper_bytes: 1_507,
        wrapper_sha256: "eee1546b6d2dfef86346ab055205d43d0510359eda2aad7a09fcaf19fb219f1a",
        wrapper_blake3: "8efdcf80dda6a2e4156f0df5696effed008f824b8d92819485e08218315c0148",
        representation: XyceSourceMultiplicityRepresentation::BehavioralDirect,
        analysis: XyceSourceMultiplicityAnalysis::Dc,
    },
    XyceSourceMultiplicityCaseSpec {
        family: "BSRC/bsrc2_m",
        owner_record: "netlists/bsrc/bsrc2_m.cir",
        baseline_record: "netlists/bsrc/bsrc2_m_baseline.cir",
        owner_content_blake3: "625c6829c9ba9901c348ac025c819c40169aa613439df53cd67fc4cdac8d865c",
        baseline_content_blake3: "9f3502fa471a6ba24e3fbe8d3fae8cef6ada9018b21997ad803e6e72456133dd",
        wrapper_path: "Netlists/BSRC/bsrc2_m.cir.sh",
        wrapper_bytes: 1_507,
        wrapper_sha256: "ef0e8238efec42455b4a93ecc66a4cdce7c17d48b02e98d5260f52f11c84344f",
        wrapper_blake3: "fb14fbf86b76ec0f5139838db876bb1d73ef17107fda20aec295f0fd7256c0c1",
        representation: XyceSourceMultiplicityRepresentation::BehavioralFormal,
        analysis: XyceSourceMultiplicityAnalysis::Dc,
    },
    XyceSourceMultiplicityCaseSpec {
        family: "BSRC/bsrc3_m",
        owner_record: "netlists/bsrc/bsrc3_m.cir",
        baseline_record: "netlists/bsrc/bsrc3_m_baseline.cir",
        owner_content_blake3: "35920c403dcbdd82a8fc8cb2603219ea6184c7d3f2648d1f61475cb3e4053ec1",
        baseline_content_blake3: "2c2406536e6572e285bfb2aa28c92295c6cecfa886b89e30e30edf73abe318af",
        wrapper_path: "Netlists/BSRC/bsrc3_m.cir.sh",
        wrapper_bytes: 1_507,
        wrapper_sha256: "62792bdceaf62b26b47a29131d3c066f8bb2ffcdbab6dfbbe0a8b46230fb1887",
        wrapper_blake3: "d5cf5022d29dc95f0fa6f4ba521d526f5f449231b9d13704198de1f0fa3ad7c8",
        representation: XyceSourceMultiplicityRepresentation::BehavioralInherited,
        analysis: XyceSourceMultiplicityAnalysis::Dc,
    },
    XyceSourceMultiplicityCaseSpec {
        family: "BSRC/bsrc4_m",
        owner_record: "netlists/bsrc/bsrc4_m.cir",
        baseline_record: "netlists/bsrc/bsrc4_m_baseline.cir",
        owner_content_blake3: "0aad95a37310d6977618e07d79f60fae09eafbd691b47b3af57847159789e2c7",
        baseline_content_blake3: "25bf3ef5d4746dedb55cdbe083a43cd9b42686a7102b929a8c7d5aa8bf26cd6e",
        wrapper_path: "Netlists/BSRC/bsrc4_m.cir.sh",
        wrapper_bytes: 1_507,
        wrapper_sha256: "47492e2c23d5e69b156e60e50694e7db127dca4b303bc86fbbd08d729e1145c5",
        wrapper_blake3: "df4f4e03e2a72b82ceee402aa483444b47b4ce9bb2148075fabeae89ab56f72a",
        representation: XyceSourceMultiplicityRepresentation::BehavioralNested,
        analysis: XyceSourceMultiplicityAnalysis::Dc,
    },
    XyceSourceMultiplicityCaseSpec {
        family: "VCCS/vccs_m",
        owner_record: "netlists/vccs/vccs_m.cir",
        baseline_record: "netlists/vccs/vccs_m_baseline.cir",
        owner_content_blake3: "f812935ead59e7a2b12406d558740053d86d73d008d1ddeb0a08252d7e74f1ed",
        baseline_content_blake3: "6abda75fdb8e4b10468f1beda790188e30b4a558b4c963a67a797e51fe5c0c75",
        wrapper_path: "Netlists/VCCS/vccs_m.cir.sh",
        wrapper_bytes: 1_505,
        wrapper_sha256: "e006106f3eb412eccc545ba6235a54185ecd2813a757ec49e0e72468bb9a45fa",
        wrapper_blake3: "c35e4dc65ea5e269b471b81d4b1034993f025cff92d367b8cbd9bccd6b03a75c",
        representation: XyceSourceMultiplicityRepresentation::LinearDirect,
        analysis: XyceSourceMultiplicityAnalysis::Dc,
    },
    XyceSourceMultiplicityCaseSpec {
        family: "VCCS/vccs_nl1_m",
        owner_record: "netlists/vccs/vccs_nl1_m.cir",
        baseline_record: "netlists/vccs/vccs_nl1_m_baseline.cir",
        owner_content_blake3: "e2b173114a7447c858a03438e41128d969e0eed6b301d9b2111a5a096bfda931",
        baseline_content_blake3: "6abda75fdb8e4b10468f1beda790188e30b4a558b4c963a67a797e51fe5c0c75",
        wrapper_path: "Netlists/VCCS/vccs_nl1_m.cir.sh",
        wrapper_bytes: 1_513,
        wrapper_sha256: "58f91c31303fca150139c1fba81a2cdb46847d340dcdd2260bbba1340e4cbc8c",
        wrapper_blake3: "c5326a2e3a98006d797a9e5462fdbbcba2f750c3ed75fd5d2e1452ce2118d99e",
        representation: XyceSourceMultiplicityRepresentation::ExpressionDirect,
        analysis: XyceSourceMultiplicityAnalysis::Dc,
    },
    XyceSourceMultiplicityCaseSpec {
        family: "VCCS/vccs_nl2_m",
        owner_record: "netlists/vccs/vccs_nl2_m.cir",
        baseline_record: "netlists/vccs/vccs_nl2_m_baseline.cir",
        owner_content_blake3: "c05a5858f5193946e8e95fb8ada1e91a38121aaf8f0ef31183724038c1fd11d9",
        baseline_content_blake3: "6abda75fdb8e4b10468f1beda790188e30b4a558b4c963a67a797e51fe5c0c75",
        wrapper_path: "Netlists/VCCS/vccs_nl2_m.cir.sh",
        wrapper_bytes: 1_513,
        wrapper_sha256: "f55c19a091f33fba8cead3c21aab70b58b7d17104d831e3df8d606468214383d",
        wrapper_blake3: "a34f4217ca26a757d82df2eff09c52cf6d2a3a63885f824d930757d3a79f85f1",
        representation: XyceSourceMultiplicityRepresentation::ExpressionFormal,
        analysis: XyceSourceMultiplicityAnalysis::Dc,
    },
    XyceSourceMultiplicityCaseSpec {
        family: "VCCS/vccs_nl3_m",
        owner_record: "netlists/vccs/vccs_nl3_m.cir",
        baseline_record: "netlists/vccs/vccs_nl3_m_baseline.cir",
        owner_content_blake3: "53f6c0b9c09993e1383d4a64cc07ce6cfca92f403c20f95882c3b77edacbf288",
        baseline_content_blake3: "6abda75fdb8e4b10468f1beda790188e30b4a558b4c963a67a797e51fe5c0c75",
        wrapper_path: "Netlists/VCCS/vccs_nl3_m.cir.sh",
        wrapper_bytes: 1_513,
        wrapper_sha256: "30aafe1137706d5daf7fce968f232788ccf61482e4b62d9cfd17ba08a820117f",
        wrapper_blake3: "83a6efe07cdf21df89dced284b2a7c48e66507486c424e24d8e5625d22bec04b",
        representation: XyceSourceMultiplicityRepresentation::ExpressionInherited,
        analysis: XyceSourceMultiplicityAnalysis::Dc,
    },
    XyceSourceMultiplicityCaseSpec {
        family: "VCCS/vccs_nl4_m",
        owner_record: "netlists/vccs/vccs_nl4_m.cir",
        baseline_record: "netlists/vccs/vccs_nl4_m_baseline.cir",
        owner_content_blake3: "8d5d10cf88304fd2cb3bb491b4fdaf63d02d12776612d7f8edcac1a2d3c434f2",
        baseline_content_blake3: "6abda75fdb8e4b10468f1beda790188e30b4a558b4c963a67a797e51fe5c0c75",
        wrapper_path: "Netlists/VCCS/vccs_nl4_m.cir.sh",
        wrapper_bytes: 1_513,
        wrapper_sha256: "5a978749375eb3d89a91d9bf6aac4c44ba01a0904d54baefd17363eb5f16abbd",
        wrapper_blake3: "2aea6fa37179f2f1f49424c7f2fc6974d2205bd42dbe9526aa4138cc91744a50",
        representation: XyceSourceMultiplicityRepresentation::ExpressionNested,
        analysis: XyceSourceMultiplicityAnalysis::Dc,
    },
    XyceSourceMultiplicityCaseSpec {
        family: "VCCS/vccs_tran_m",
        owner_record: "netlists/vccs/vccs_tran_m.cir",
        baseline_record: "netlists/vccs/vccs_tran_m_baseline.cir",
        owner_content_blake3: "9a7640eb4d5e03a2c6208481cb1dc44a77533b8b52d06d2784f9a84aad791fb5",
        baseline_content_blake3: "ab06d9f0ed36a4616f10430e0a128c0e50cb7b3b85a8f2980c2c6708052357cc",
        wrapper_path: "Netlists/VCCS/vccs_tran_m.cir.sh",
        wrapper_bytes: 1_515,
        wrapper_sha256: "20a644a2b7b5e532e642c672dc7764804938cf67959159824b0fdbed8d6271dd",
        wrapper_blake3: "a3b3ffdacd87222c56c6a05051206989db9e665b2a19552fc87effb6e06be969",
        representation: XyceSourceMultiplicityRepresentation::LinearDirect,
        analysis: XyceSourceMultiplicityAnalysis::Tran,
    },
];

const XYCE_SOURCE_MULTIPLICITY_HISTORICAL_EXCLUDES: [(&str, usize, &str, &str); 2] = [
    (
        "Netlists/BSRC/exclude",
        84,
        "d201ec9fedac09939734e8f5e0746c7a46193ae25bb79d6c75699c68224a38b5",
        "121f27fd1a3e7cbddc4d282aee79cdd98b843cd821551125566ac2196c00f3e6",
    ),
    (
        "Netlists/VCCS/exclude",
        141,
        "918ba1f38f946bc77d7425c176340d5b29253ae9e63ffb3f969553367f63456c",
        "3b8424f66d51107ee38549cea6006789460a8969f85b6cf59af1f40500e943ad",
    ),
];
const XYCE_LEVEL2_DIODE_DTEMP_WRAPPER_CONTRACT: &str =
    "level2_diode_dtemp_relational_wrapper_owner";
const XYCE_LEVEL2_DIODE_DTEMP_REFERENCE_CONTRACT: &str =
    "level2_diode_dtemp_relational_wrapper_reference";
const XYCE_LEVEL2_DIODE_DTEMP_OWNER_RECORD: &str = "netlists/dtemp/level2_diode_dtemp.cir";
const XYCE_LEVEL2_DIODE_DTEMP_REFERENCE_RECORD: &str = "netlists/dtemp/level2_diode_ref.cir";
const XYCE_LEVEL2_DIODE_DTEMP_CANDIDATE_COUNT: usize = 2;
const XYCE_LEVEL2_DIODE_DTEMP_CANDIDATE_BLAKE3: &str =
    "526514a734ea7df6e7257be805f1125ff0235543308aba2aaaf70a4e9e78e968";
const XYCE_LEVEL2_DIODE_DTEMP_CONTENT_BLAKE3: &str =
    "d995c1dc634d0b462b4f2677044fc8e4f4a98640a156de6d543a3dc06dce8118";
const XYCE_LEVEL2_DIODE_DTEMP_OWNER_COUNT: usize = 1;
const XYCE_LEVEL2_DIODE_DTEMP_OWNER_MANIFEST_BLAKE3: &str =
    "fabfaebde10b510461d954d013faa2cf112788258f98144d5dc0f2013948760f";
const XYCE_LEVEL2_DIODE_DTEMP_EXCLUSION_COUNT: usize = 1;
const XYCE_LEVEL2_DIODE_DTEMP_HISTORICAL_EXCLUSION_BLAKE3: &str =
    "89427341a2d1ad7e4c898035eaa0bbab587ce40bafd2394fa5754914c030bdba";
const XYCE_CAPACITOR_DTEMP_WRAPPER_CONTRACT: &str = "capacitor_dtemp_relational_wrapper_owner";
const XYCE_CAPACITOR_DTEMP_REFERENCE_CONTRACT: &str =
    "capacitor_dtemp_relational_wrapper_reference";
const XYCE_CAPACITOR_DTEMP_OWNER_RECORD: &str = "netlists/dtemp/cap_dtemp.cir";
const XYCE_CAPACITOR_DTEMP_REFERENCE_RECORD: &str = "netlists/dtemp/cap_ref.cir";
const XYCE_CAPACITOR_DTEMP_CANDIDATE_COUNT: usize = 2;
const XYCE_CAPACITOR_DTEMP_CANDIDATE_BLAKE3: &str =
    "47e922d813ff50051123f160fe38a291260bc61a61558bb6928774ec2a6ced11";
const XYCE_CAPACITOR_DTEMP_CONTENT_BLAKE3: &str =
    "438d548e950706c07a8903fcade734d87c2dce479eb6234c938164122333f82c";
const XYCE_CAPACITOR_DTEMP_OWNER_COUNT: usize = 1;
const XYCE_CAPACITOR_DTEMP_OWNER_MANIFEST_BLAKE3: &str =
    "cca5a9169aeb4fa751eb639be7a4c3ee0b7d649f498407d4f97e549aa23d43eb";
const XYCE_CAPACITOR_DTEMP_EXCLUSION_COUNT: usize = 1;
const XYCE_CAPACITOR_DTEMP_HISTORICAL_EXCLUSION_BLAKE3: &str =
    "f54cec2065bc764c4517a7d778265d7aa717bb1bbdc5f77defd4bfe87c7b0a73";
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
const XYCE_ANALYTIC_INT_FLOOR_CEIL_FAMILY_PREFIX: &str = "netlists/abm_int_floor_ceil/";
const XYCE_ANALYTIC_INT_FLOOR_CEIL_FAMILY_COUNT: usize = 2;
const XYCE_ANALYTIC_INT_FLOOR_CEIL_FAMILY_NAMES_BLAKE3: &str =
    "19f995371e91255c48b6cded0bd81e2a87156f5824f5d65c0d78ec57f84a8975";
const XYCE_ANALYTIC_INT_FLOOR_CEIL_FAMILY_CONTENT_BLAKE3: &str =
    "a2a146d650cf2217d2bc6f0ca51823a7e161b49d15b2de0a7af598b1be6d243f";
const XYCE_ANALYTIC_INT_FLOOR_CEIL_MANIFEST_BLAKE3: &str =
    "c1cef03cf5de631dbd3aaab904745bc4f0cbf821f9e1fb5e077132999269ca1a";
const XYCE_ANALYTIC_INT_FLOOR_CEIL_TRAN_SOURCE_BYTES: usize = 674;
const XYCE_ANALYTIC_INT_FLOOR_CEIL_TRAN_SOURCE_BLAKE3: &str =
    "d6c9fcceda0eecc7564c9670c6e5e40ef1484f95a45d9db772ffafd0897fd744";
const XYCE_ANALYTIC_INT_FLOOR_CEIL_TRAN_SOURCE_BLOB: &str =
    "616bd37a2a0101cc9cc8e891d2d6c6f4c46c0beb";
const XYCE_ANALYTIC_INT_FLOOR_CEIL_TRAN_SOURCE_SHA256: &str =
    "a81525bef161e2398c21ecf1d7a49607a04c208d0163d8c564811768050ead75";
const XYCE_ANALYTIC_INT_FLOOR_CEIL_UPSTREAM_REGRESSION_COMMIT: &str =
    "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const XYCE_ANALYTIC_INT_FLOOR_CEIL_UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const XYCE_ANALYTIC_INT_FLOOR_CEIL_HISTORICAL_WRAPPER_PATH: &str =
    "Netlists/ABM_INT_FLOOR_CEIL/int_floor_ceil.cir.sh";
const XYCE_ANALYTIC_INT_FLOOR_CEIL_HISTORICAL_WRAPPER_BYTES: usize = 1_558;
const XYCE_ANALYTIC_INT_FLOOR_CEIL_HISTORICAL_WRAPPER_BLOB: &str =
    "a1730b6ba7ef8a1dc3761caccbab8d5298efadba";
const XYCE_ANALYTIC_INT_FLOOR_CEIL_HISTORICAL_WRAPPER_SHA256: &str =
    "4a6b7f98c01ffb1f5d09368e086d14631ec698cae090e1cfecf70a9e5752ebab";
const XYCE_ANALYTIC_INT_FLOOR_CEIL_HISTORICAL_MANIFEST_PATH: &str =
    "Netlists/ABM_INT_FLOOR_CEIL/Manifest.txt";
const XYCE_ANALYTIC_INT_FLOOR_CEIL_HISTORICAL_MANIFEST_BYTES: usize = 97;
const XYCE_ANALYTIC_INT_FLOOR_CEIL_HISTORICAL_MANIFEST_BLOB: &str =
    "c7bca70e3f69ca9ed637af299f9f0ffb8ed627fd";
const XYCE_ANALYTIC_INT_FLOOR_CEIL_HISTORICAL_MANIFEST_SHA256: &str =
    "a4e86d598769593f7902a6f18b68f9f2573de97d2b8077a0a2a42be803e1ef37";
const XYCE_ANALYTIC_INT_FLOOR_CEIL_HISTORICAL_TAGS_PATH: &str = "Netlists/ABM_INT_FLOOR_CEIL/tags";
const XYCE_ANALYTIC_INT_FLOOR_CEIL_HISTORICAL_TAGS_BYTES: usize = 52;
const XYCE_ANALYTIC_INT_FLOOR_CEIL_HISTORICAL_TAGS_BLOB: &str =
    "4b222cb9237e5e7075fbf656c8e33b3d618a5f8c";
const XYCE_ANALYTIC_INT_FLOOR_CEIL_HISTORICAL_TAGS_SHA256: &str =
    "90c933a317bec2b3065068d1c8434f4c024bc07a3d3b7ba5d36f9bb5f992fc85";
const XYCE_ANALYTIC_INT_FLOOR_CEIL_HISTORICAL_TOOLS_PATH: &str =
    "TestScripts/XyceRegression/Tools.pm";
const XYCE_ANALYTIC_INT_FLOOR_CEIL_HISTORICAL_TOOLS_BYTES: usize = 68_108;
const XYCE_ANALYTIC_INT_FLOOR_CEIL_HISTORICAL_TOOLS_BLOB: &str =
    "16fa0adc3fbd03d653de4faaeaaa0fea8f8eee6c";
const XYCE_ANALYTIC_INT_FLOOR_CEIL_HISTORICAL_TOOLS_SHA256: &str =
    "5b5f86c02d46a1f3bdad5292e7e91d25a9e08e71490643d8d5ed7ae20f9d55e3";
const XYCE_ANALYTIC_INT_FLOOR_CEIL_HISTORICAL_RECORD_COUNT: usize = 5;
const XYCE_ANALYTIC_INT_FLOOR_CEIL_HISTORICAL_BLAKE3: &str =
    "b1254efb20cfb1641dbbe95daf2712c4b476d449ea13c3fbd01072b747293e33";
const XYCE_ABM_POW_FAMILY_PREFIX: &str = "netlists/abm_pow/";
const XYCE_ABM_POW_SOURCE_DIRECTORY_COUNT: usize = 3;
const XYCE_ABM_POW_SOURCE_DIRECTORY_BLAKE3: &str =
    "918d2fdc102269c9cf62b83ba5f62f4037ceb53b288ba4772532b9477f84ff39";
const XYCE_ABM_POW_SOURCE_CONTENT_CENSUS_BLAKE3: &str =
    "3c3f42c4ef37d6742df6380a85820b438154c6ed9231cc42beb66714bbe2d917";
const XYCE_ABM_POW_CANDIDATE_BLAKE3: &str =
    "8f4fa483c6b5862124323c57f642e8763050821be3344c4d5ae036733617c144";
const XYCE_ABM_POW_CANDIDATE_CONTENT_BLAKE3: &str =
    "2ffe099c0701a75920d9dd5b26db68736bf85297ce7ae794e673fdbaf93dd7eb";
const XYCE_ABM_POW_MANIFEST_BLAKE3: &str =
    "8f4fa483c6b5862124323c57f642e8763050821be3344c4d5ae036733617c144";
const XYCE_ABM_TRANSIENT_TIME_FAMILY_PREFIX: &str = "netlists/abm_time/";
const XYCE_ABM_TRANSIENT_SQRT_FAMILY_PREFIX: &str = "netlists/abm_sqrt/";
const XYCE_ABM_TRANSIENT_TIME_DIRECTORY_COUNT: usize = 2;
const XYCE_ABM_TRANSIENT_TIME_DIRECTORY_BLAKE3: &str =
    "b33b695a8ea3282c92677ab31dea460e37e77206a6441a8e5194714272132508";
const XYCE_ABM_TRANSIENT_TIME_CONTENT_BLAKE3: &str =
    "c952881c72e510be28bbd3912163a9039b5deee30d0b638355176ba5434f52b6";
const XYCE_ABM_TRANSIENT_SQRT_DIRECTORY_COUNT: usize = 1;
const XYCE_ABM_TRANSIENT_SQRT_DIRECTORY_BLAKE3: &str =
    "caa6b38ddf60870fe4f1244904485c46fc483d5721da36b7b08bfc91258109b8";
const XYCE_ABM_TRANSIENT_SQRT_CONTENT_BLAKE3: &str =
    "286988ba2d0a13300b42021cdcc69d282ae2419f1b64454e7006cd41e72f62a1";
const XYCE_ABM_TRANSIENT_CANDIDATE_BLAKE3: &str =
    "01fa5d35bc700b26933db5f1bfdc7b69a12d7f7eafa7e3b687fbc705f54aa81d";
const XYCE_ABM_TRANSIENT_CANDIDATE_CONTENT_BLAKE3: &str =
    "72f62e2db60be24d51f72b1efecba72f07479ec6d95c43a1f49b43e471a83142";
const XYCE_ABM_TRANSIENT_MANIFEST_BLAKE3: &str =
    "01fa5d35bc700b26933db5f1bfdc7b69a12d7f7eafa7e3b687fbc705f54aa81d";
const XYCE_MEASURE_CONT_TRAN_SOURCE_FAMILY_COUNT: usize = 66;
const XYCE_MEASURE_CONT_TRAN_SOURCE_FAMILY_NAMES_BLAKE3: &str =
    "1fe7b4af298d44520dc978c3e56a434ece02ab895a09d5d873a4f577adfc8bc2";
const XYCE_MEASURE_CONT_TRAN_SOURCE_FAMILY_CONTENT_BLAKE3: &str =
    "b0ea7515be7217d05cd09983da9a4b1a515c8cfce45c4cc78864c8047bef0c0c";
const XYCE_MEASURE_CONT_TRAN_OUTPUT_FAMILY_COUNT: usize = 60;
const XYCE_MEASURE_CONT_TRAN_OUTPUT_FAMILY_NAMES_BLAKE3: &str =
    "fb8fc66f3a69a66991c8c9a2373329b6ebb302d7ad57630e4356972984ba30fd";
const XYCE_MEASURE_CONT_TRAN_OUTPUT_FAMILY_CONTENT_BLAKE3: &str =
    "ba1d604fd57372ef8e990419b201300cb07d80fbf24c3bd9c732d597e6291fd1";
const XYCE_MEASURE_CONT_TRAN_CANDIDATE_BLAKE3: &str =
    "6c1be110ea8bd3607fb739190fb7e4c6328737c10324e8a32952d25daaaab7f0";
const XYCE_MEASURE_CONT_TRAN_CANDIDATE_CONTENT_BLAKE3: &str =
    "5ebcdae90f486952f5b32f8298ffcac675b19ff14982456dc9c85fd9acfd7799";
const XYCE_MEASURE_CONT_TRAN_MANIFEST_BLAKE3: &str =
    "6c1be110ea8bd3607fb739190fb7e4c6328737c10324e8a32952d25daaaab7f0";
const XYCE_MEASURE_CONT_TRAN_ARTIFACT_CONTENT_BLAKE3: &str =
    "5c75dece6ec6a985871ec7a44ba04f474e23b9b73ef9786e27507f45c9bfefd0";
const XYCE_MEASURE_CONT_MANIFEST_FAMILY_COUNT: usize = 31;
const XYCE_MEASURE_CONT_MANIFEST_FAMILY_PATHS_BLAKE3: &str =
    "9cbd9d8d4874de203ab21ef5ef301d5f75b92cb4d9208bc6f6d81e785159990d";
const XYCE_MEASURE_CONT_MANIFEST_FAMILY_LINES_BLAKE3: &str =
    "9d1bfca8151e9c3c16239558b9a34b1ad067d6b236c00d8c334e1703d0ec0a22";
const XYCE_MEASURE_CONT_STEP_TRAN_DIRECTORY_COUNT: usize = 27;
const XYCE_MEASURE_CONT_STEP_TRAN_DIRECTORY_NAMES_BLAKE3: &str =
    "66a4a04b46f70bfe859d2f2861e27d1fdcc9072c41f7c084d9ea88c406dc1b63";
const XYCE_MEASURE_CONT_STEP_TRAN_DIRECTORY_CONTENT_BLAKE3: &str =
    "da58c73c916b4fd6d1968d4d9ee52985079834e8877d911522a9982e7a777315";
const XYCE_MEASURE_CONT_STEP_TRAN_CANDIDATE_BLAKE3: &str =
    "ab99e7673941a6438c75ecf2ef41fae83aa2d3ef20bbdb4c36e73d8065c3d70d";
const XYCE_MEASURE_CONT_STEP_TRAN_CANDIDATE_CONTENT_BLAKE3: &str =
    "b7af2ae67cbc7a334d38db3a888779684218cf18bd53a0228cbabaa5f95c77da";
const XYCE_MEASURE_CONT_STEP_TRAN_MANIFEST_BLAKE3: &str =
    "cf8c3b451cb13d00e97e4e2057eb0502fdcebdf435b44b186bcb450eca539378";
const XYCE_MEASURE_CONT_STEP_HISTORICAL_PROVENANCE_BLAKE3: &str =
    "d5665d4a8d8ecedf36e5ffb47c53b94a0d9fc867af1afc66076b500d645bd0b5";
const XYCE_MEASURE_CONT_STEP_NOISE_DERIV_RECORD: &str =
    "netlists/measure_cont/step/derivtestnoise.cir";
const XYCE_MEASURE_CONT_STEP_NOISE_DERIV_SOURCE_BYTES: usize = 5_513;
const XYCE_MEASURE_CONT_STEP_NOISE_DERIV_SOURCE_BLAKE3: &str =
    "0775c53a3da0aaa588a009a4093f8b3b38e6b941fc69fc2c0bf9a29bc973c6af";
const XYCE_MEASURE_CONT_STEP_NOISE_DERIV_GS_BYTES: usize = 13_934;
const XYCE_MEASURE_CONT_STEP_NOISE_DERIV_GS_BLAKE3: &str =
    "e7d26b08ac8c3c0b79017e35848e4b40f48fac76bfeaf92c0934ff3879189193";
const XYCE_MEASURE_CONT_STEP_NOISE_DERIV_MA0_BYTES: usize = 2_196;
const XYCE_MEASURE_CONT_STEP_NOISE_DERIV_MA0_BLAKE3: &str =
    "a93764f361eb2607864158496e2963d9b6c47f88583cf4b6ec0b8605a80a1a86";
const XYCE_MEASURE_CONT_STEP_NOISE_DERIV_MA1_BYTES: usize = 2_138;
const XYCE_MEASURE_CONT_STEP_NOISE_DERIV_MA1_BLAKE3: &str =
    "56423533d5521d52bc2208b8230084e989de77818aadea97bada8e584d6d2b1f";
const XYCE_MEASURE_NOISE_STEP_DERIV_RECORD: &str = "netlists/measure_noise/step/derivtestnoise.cir";
const XYCE_MEASURE_NOISE_STEP_DERIV_SOURCE_BYTES: usize = 2_115;
const XYCE_MEASURE_NOISE_STEP_DERIV_SOURCE_BLAKE3: &str =
    "ab68163ace0b97eec114870078ca9c8bb13b2f5ddf36ca64f06e7583656e404e";
const XYCE_MEASURE_NOISE_STEP_DERIV_MA0_BYTES: usize = 404;
const XYCE_MEASURE_NOISE_STEP_DERIV_MA0_BLAKE3: &str =
    "121373fe9f0ad0219b1f3ca30884b5ab3935c2e09faf8fc57d6655f250d24956";
const XYCE_MEASURE_NOISE_STEP_DERIV_MA1_BYTES: usize = 404;
const XYCE_MEASURE_NOISE_STEP_DERIV_MA1_BLAKE3: &str =
    "43b79149352880fbe065bcc8fdf7fb787acfbbe71d1a6f63d051abc47391c945";
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

const XYCE_IC_MISSING_NODE_WARNING_BLAKE3: &str =
    "c49c834da18ec5603a8fc748f79d184fd94385acdf12a2ffdf0c9c2b77efa793";
const XYCE_IC_EMPTY_WARNING_BLAKE3: &str =
    "b2f42b66d52f0c2dbf11d075e02f4fa5910b7b0f563cb7533e8759e43c4f708f";
const XYCE_NODESET_MISSING_NODE_WARNING_BLAKE3: &str =
    "9b861bfd6daed7c866918ddfb98f67c8f3968836b674ce1ca42c45c608059ab9";
const XYCE_NODESET_EMPTY_WARNING_BLAKE3: &str =
    "f89293d171168ccc1e958b88a0ca64127d29b06ed44fb50383880e9747f79729";
const XYCE_BUG667_SCOPED_GLOBAL_WARNING_BLAKE3: &str =
    "be5e0b168367716ca744c38417952c0b680ae063d309859100202d2b37242f19";
const XYCE_IC_NODESET_CONFLICT_BLAKE3: &str =
    "1e7597f00478759e105259a832d7aa298c3ffcb3c4d3a8efda4c236504ebed6b";

const XYCE_XDM_REPLACEGROUND_FAMILY_PREFIX: &str = "netlists/xdm/hspice/other_parsing/";
const XYCE_XDM_REPLACEGROUND_SOURCE_DIRECTORY_COUNT: usize = 27;
const XYCE_XDM_REPLACEGROUND_SOURCE_DIRECTORY_BLAKE3: &str =
    "e5ace1dae889fc1b11fa88db05c96d21a62d1ae688e9cde5a97373205cc049e6";
const XYCE_XDM_REPLACEGROUND_SOURCE_CONTENT_CENSUS_BLAKE3: &str =
    "490b8ebcd683dbdcfb6db7ab97f9b3c26e7e8b6356e1ae0f06d1a80fd89388aa";
const XYCE_XDM_REPLACEGROUND_PHYSICAL_COUNT: usize = 13;
const XYCE_XDM_REPLACEGROUND_PHYSICAL_BLAKE3: &str =
    "5157bce8c21053398ed7c64eb02dd2ce217587a254b6e964c843b100ed7c48ab";
const XYCE_XDM_REPLACEGROUND_CANDIDATE_COUNT: usize = 4;
const XYCE_XDM_REPLACEGROUND_CANDIDATE_BLAKE3: &str =
    "7b9fdd009d18bdc864e24e8c14aa1d7f56ab10b13f32209fb3e85963c798a406";
const XYCE_XDM_REPLACEGROUND_MANIFEST_COUNT: usize = 13;
const XYCE_XDM_REPLACEGROUND_MANIFEST_BLAKE3: &str =
    "d9ef24c15d50f61735c7b136c407c5b0ba026ba3831537da5f088b178f726c75";
const XYCE_REMOVEUNUSED_FAMILY_PREFIX: &str = "netlists/redund_remove/";
const XYCE_REMOVEUNUSED_SOURCE_DIRECTORY_COUNT: usize = 7;
const XYCE_REMOVEUNUSED_SOURCE_DIRECTORY_BLAKE3: &str =
    "8b0bde42d563d5c43d66bd4d1b3b4b111181b7f77030a06b69793a098e3e6f48";
const XYCE_REMOVEUNUSED_SOURCE_CONTENT_CENSUS_BLAKE3: &str =
    "3529baaa4c9d889777a7cd2f5d7557d7ac681be90bf1f02a93af8f9503c26fcc";
const XYCE_REMOVEUNUSED_PHYSICAL_COUNT: usize = 5;
const XYCE_REMOVEUNUSED_PHYSICAL_BLAKE3: &str =
    "99ecb313048fdf1faec719760e6ebe979b7220728fdb82ad2d107997315ee8e7";
const XYCE_REMOVEUNUSED_CANDIDATE_COUNT: usize = 2;
const XYCE_REMOVEUNUSED_CANDIDATE_BLAKE3: &str =
    "364ae7c5079dc42ed8516abd4c73da3d0001ecde999c6775fa02b97721b3f5e6";
const XYCE_REMOVEUNUSED_CANDIDATE_CONTENT_BLAKE3: &str =
    "a42d2e75acf718215247b712fcd4212efde2cdff5e64b41c1143eceda05be6ad";
const XYCE_REMOVEUNUSED_MANIFEST_COUNT: usize = 3;
const XYCE_REMOVEUNUSED_MANIFEST_BLAKE3: &str =
    "a9410dae3a65ce0cfe54cbe24524596a121a03f0dad5b4c7419d797352823f64";
const XYCE_ADDRESISTORS_PREPROC_FAMILY_PREFIX: &str = "netlists/preproc_addres/";
const XYCE_ADDRESISTORS_PREPROC_SOURCE_DIRECTORY_COUNT: usize = 2;
const XYCE_ADDRESISTORS_PREPROC_SOURCE_DIRECTORY_BLAKE3: &str =
    "f6ddcc255042ea0fea56657278cf9a6d2860cdaa986449592e71813f3d2c335d";
const XYCE_ADDRESISTORS_PREPROC_SOURCE_CONTENT_CENSUS_BLAKE3: &str =
    "b0f1a0ca3eef8151ed9c31563bac68e6994bb1a5554d8c39276555006febae2e";
const XYCE_ADDRESISTORS_PREPROC_PHYSICAL_COUNT: usize = 2;
const XYCE_ADDRESISTORS_PREPROC_PHYSICAL_BLAKE3: &str =
    "f6ddcc255042ea0fea56657278cf9a6d2860cdaa986449592e71813f3d2c335d";
const XYCE_ADDRESISTORS_PREPROC_MANIFEST_COUNT: usize = 2;
const XYCE_ADDRESISTORS_PREPROC_MANIFEST_BLAKE3: &str =
    "db332ee1914f3484bdbfa150096708888c5122d3adca58ac0e7eefb642f9d60e";
const XYCE_ADDRESISTORS_CANDIDATE_COUNT: usize = 3;
const XYCE_ADDRESISTORS_CANDIDATE_BLAKE3: &str =
    "cd3f76a92f623c439dbdedeb952482596a7528b1df04eae958f796a7d9bcd532";
const XYCE_ADDRESISTORS_CANDIDATE_CONTENT_BLAKE3: &str =
    "84c0a3e97435d373c8d94b678ab8650b04ad599900e13a58368424847d0c5a5e";

// The four removed XDM wrappers declare abs=1e-5, rel=1e-3, and zero=1e-10,
// but `verifyXDMtranslation` never forwards those variables for HSPICE.  It
// invokes Release 7.10 `xyce_verify` without tolerance flags, so the effective
// comparison is the verifier default below.  Keep the declared values visible
// to prevent a future adapter from accidentally claiming that they governed
// the authoritative run.
const XYCE_XDM_REPLACEGROUND_DECLARED_ABSOLUTE_TOLERANCE: Value = 1.0e-5;
const XYCE_XDM_REPLACEGROUND_DECLARED_RELATIVE_TOLERANCE: Value = 1.0e-3;
const XYCE_XDM_REPLACEGROUND_DECLARED_ZERO_TOLERANCE: Value = 1.0e-10;

const XYCE_STARTUP_MESSAGE_INPUT_SOURCE_DIRECTORY_CENSUS_BLAKE3: &str =
    "c1c3aab2bbf03214039de43671c9c36fe3b4e69a2baf92fd60124de6da76e950";
const XYCE_STARTUP_BUG667_PHYSICAL_CENSUS_BLAKE3: &str =
    "e6f6e3062ace9ab02388edc91e9a01dba2979617d480dcaba154d6398f125603";
const XYCE_STARTUP_BUG667_MANIFEST_CENSUS_BLAKE3: &str =
    "2457b1e041aeb407a95238024680799052bf59342381aeb9b67b16ce85df8891";
const XYCE_STARTUP_OPTIONS_BLAKE3: &str =
    "b1d67968e7446e26800d83b2f63ab18f63fd84b5b602758b2b2327bbdf15ef3b";
const XYCE_STARTUP_OPTIONS_BYTES: usize = 14;
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
const XYCE_BUG1595_EXPECTED_FAILURE_PATH: &str =
    "Netlists/Certification_Tests/BUG_1595/bug1595.cir";
const XYCE_BUG1595_EXPECTED_FAILURE_RECORD: &str =
    "netlists/certification_tests/bug_1595/bug1595.cir";
const XYCE_BUG1595_EXPECTED_FAILURE_CONTRACT: &str =
    "expected_failure_bug1595_hierarchical_mutual_inductor_reference_parse";
const XYCE_BUG1595_UPSTREAM_REGRESSION_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const XYCE_BUG1595_UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const XYCE_BUG1595_HISTORICAL_TIMEOUT_MS: u128 = 30_000;
const XYCE_BUG1595_HISTORICAL_RECORD_COUNT: usize = 7;
const XYCE_BUG1595_HISTORICAL_RECORD_BYTES: usize = 1_661;
const XYCE_BUG1595_HISTORICAL_RECORDS_SHA256: &str =
    "a909db4f211a4240f8a96d9c228750e0985e35bcc128778eb172f0367a148e5b";
const XYCE_BUG1595_HISTORICAL_RECORDS_BLAKE3: &str =
    "eabf571c168d4f7e9964a6faf33e507221292f3957763da41e0bfc7ff88c9848";
const XYCE_BUG1595_HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 7] = [
    (
        "Netlists/Certification_Tests/BUG_1595/CMakeLists.txt",
        1_768,
        "92af05c4a69f6cc77a6d9d448a09299af8bc0dee1c28eb2936f286aa4f85b95c",
        "e71270a93a2b59c61ddb8f6469c2c6d2a3fbc0c3e38ddf6625ebb428744ba9ab",
    ),
    (
        "Netlists/Certification_Tests/BUG_1595/Manifest.txt",
        40,
        "c0ff0b44a1af8d83c178fc3fcd176c91ecfbf176f66c972817178dddb7d2861c",
        "c6cba20bfe3767a6d8b2097af10a7d9873ef044a7c369b51fafffe18ac5903d6",
    ),
    (
        XYCE_BUG1595_EXPECTED_FAILURE_PATH,
        586,
        "9a0971993ed4c05156b0f7bf346c74552f021de2ff3cd2d2e092d96ecd420b0f",
        "cfa7de4956a14c54a0f37d947f6b14c1eb2a75368f1f30cc08cd9c5bff9c5f7f",
    ),
    (
        "Netlists/Certification_Tests/BUG_1595/bug1595.cir.sh",
        847,
        "e5e977ec98ddf71d52e786b4fe3f11a471a55c10ffc254631b26363d7bd1e304",
        "7b7f62d3e37cfac341af1060bd96f3ed168cbe549be2a0595b0e2b0686fc6af6",
    ),
    (
        "Netlists/Certification_Tests/BUG_1595/options",
        13,
        "381cd29ca4d9097c73fccc5f46cea0c37bd3e71da803e56ccad41d8270de9c0e",
        "8e9c4c362e6a201344f7fd4b55680c6db23a1ba99121d41b9dae7573cff78b81",
    ),
    (
        "Netlists/Certification_Tests/BUG_1595/tags",
        52,
        "7fb3714b32c37ae9a1d9299f00060bb4af3528cc251c32d3d5763a5975249f75",
        "e7388b1fb5597ceeb4f88ce83e84090ab72cdfedc5f6c95686a710d6324d3a81",
    ),
    (
        "TestScripts/XyceRegression/Tools.pm",
        68_108,
        "5b5f86c02d46a1f3bdad5292e7e91d25a9e08e71490643d8d5ed7ae20f9d55e3",
        "13bd274632744ddc4b8baee680ddc9770902793ed7ee892ecdedd4dcb3828667",
    ),
];
const XYCE_BUG1595_RETAINED_ARTIFACTS: [(&str, usize, &str, &str); 2] = [
    (
        "bug1595.cir",
        586,
        "9a0971993ed4c05156b0f7bf346c74552f021de2ff3cd2d2e092d96ecd420b0f",
        "cfa7de4956a14c54a0f37d947f6b14c1eb2a75368f1f30cc08cd9c5bff9c5f7f",
    ),
    (
        "options",
        13,
        "381cd29ca4d9097c73fccc5f46cea0c37bd3e71da803e56ccad41d8270de9c0e",
        "8e9c4c362e6a201344f7fd4b55680c6db23a1ba99121d41b9dae7573cff78b81",
    ),
];
const XYCE_BUG1595_SOURCE_BLAKE3: &str =
    "ee16db30fa9cf0cca771678bbbabe6417ad5dfda5dded174b4e2b75ba6c3445b";
const XYCE_BUG1595_PHYSICAL_CENSUS_BLAKE3: &str =
    "d5321ed43ca99ca6e3264b9bbee806c6a0699a16e4d0e618248986731f83e7f0";
const XYCE_BUG1595_MANIFEST_CENSUS_BLAKE3: &str =
    "e5ec7f0630965511fb6650c98414999c5d4ef99d22081863f4cb7413456b00ec";
const XYCE_BUG1595_SOURCE_DIRECTORY_CENSUS_BLAKE3: &str =
    "c70aaccb941f7b37e2a7e9ce313ff11f0421a61dca5f5ec323bd122791159adc";
const XYCE_BUG1595_EMPTY_OUTPUT_CENSUS_BLAKE3: &str =
    "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262";
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
const XYCE_BUG67_SOURCE_BLAKE3: &str =
    "29c1c55fcf4a297f2472878ef61e264eae4e43483d734fddbdbbb40161512337";
const XYCE_BUG671_SOURCE_BLAKE3: &str =
    "40af3a1050a8efeb51c7e3052eb1228ad331fd1fa469cde74508798c85f77653";
const XYCE_BUG726_SOURCE_BLAKE3: &str =
    "e55050584c69086bd34d84be82c885902a94eb212ffb6973f1d7786dd8b8ab2f";
const XYCE_BUG744_SOURCE_BLAKE3: &str =
    "b0c2ba9e38e293eb0d9d53cc14713921890e81330e6b06c64d3fc8f0df26afda";
const XYCE_BUG75_SOURCE_BLAKE3: &str =
    "eab31dd9fba0e2020d8ec5f40d5cfd4f3232aac87a54e7c19ced83a20fb9886f";
const XYCE_BUG1148_SOURCE_BLAKE3: &str =
    "de3cfb1443716a2c1bb376213cb54a2dcb38d218b074c6c2986b74c7227f7c8a";
const XYCE_BUG40_SOURCE_BLAKE3: &str =
    "cb77ddca1c3837d9bfc7aad5780567cc2cd2090e7903b1d749927e5f1b0bb658";
const XYCE_BUG718_INVALID_NODES_SOURCE_BLAKE3: &str =
    "69d78900ed75aa899acc0127836e4c2462b6f48d2ed6a34857bb10c4a170f4b2";
const XYCE_MESSAGE_PRINT_BAD_NODENAME_SOURCE_BLAKE3: &str =
    "20c8e3c04b88c993a4c901015eb4dd795a827f62c17692744bd5b015a0545384";
const XYCE_MESSAGE_PRINT_BAD_VARIABLE_SOURCE_BLAKE3: &str =
    "6bb9862735fc3d2444c4b8f819e2c96c9c4b648c8b38aa8802b94f9268531613";
const XYCE_LEAD_CURRENTS_INVALID_DEVICE_SOURCE_BLAKE3: &str =
    "ff72347a18df0f5d471bc20ede53cc3285adf4490707475033bd85bca7f58555";
const XYCE_MEASURE_INVALID_NODES_SOURCE_BLAKE3: &str =
    "aec265c84b6042e692ee6a2f5500561b1d320fcad0f83f7cb71b53d49b626f0d";
const XYCE_FOURIER_BAD_LINE3_SOURCE_BLAKE3: &str =
    "aeb7fd393692890326f1f2cf59663a0783720184927235c719b2e71717db21f1";
const XYCE_BUG387_SOURCE_BLAKE3: &str =
    "4cf9e5605ea32387fb6e670928e057940236b92fe3c240ee64b8b9bdce60e1b0";
const XYCE_SUBCKT_NONAME_SOURCE_BLAKE3: &str =
    "0d6a0bd47a0637d0fb92dab3555532594e026b4ed52b1840596db2acd509e79f";
const XYCE_SUBCKT_MISSING_ENDS_END_CARD_SOURCE_BLAKE3: &str =
    "1cb69bb569491d5aebdf0bdffae2a06c4d199aa5d46c424aef0326e10f5d149d";
const XYCE_SUBCKT_MISSING_ENDS_INCLUDE_EOF_SOURCE_BLAKE3: &str =
    "4d9840ab8e11b5f154fe64d2eaa6076d2ab5b595eb16a4248a84ce39a4a50a71";
const XYCE_SUBCKT_MISSING_ENDS_TOPLEVEL_EOF_SOURCE_BLAKE3: &str =
    "0b74635ac9a645a8e9604152ef35223c5e98b351edab65b49baf3c1008ed62c2";
const XYCE_SUBCKT_MISSING_ENDS_TS_INV_EOF_SOURCE_BLAKE3: &str =
    "16e9eb7f9f462997ea78c9ea2e9974c9816f2810c9de647ee430345575273334";
const XYCE_SUBCKT_A2_DUP_BINDING_SOURCE_BLAKE3: &str =
    "ef26ce0ef8c46541453810ca118f555f600d75f9ade4ff75f02aef26933ff4ee";
const XYCE_SUBCKT_J1_DUP_BINDING_SOURCE_BLAKE3: &str =
    "c1519e204897df0c4002245cdc259549c21751798ebca58e08b79d9affe4d453";
const XYCE_SUBCKT_MISSING_ENDS_INCLUDE_FILE_BLAKE3: &str =
    "7e198f9c1c164fa0a80560c99a8f5e85777ae6e19a1484b954459692db603661";
const XYCE_SUBCKT_MISSING_ENDS_INCLUDE_FILE_BYTES: usize = 43;
const XYCE_DC_EXCESS_ARGS_SOURCE_BLAKE3: &str =
    "472709aa403c4da89e736c47b64eff48fd919f2518d671c79cc728d847812ac1";
const XYCE_AC_UNSUPPORTED_SWEEP_SOURCE_BLAKE3: &str =
    "de4ea71d020e6d7b8cf23bb762f3fa55bfbc60afe7bd19de452537b544a8b043";
const XYCE_NOISE_UNSUPPORTED_SWEEP_SOURCE_BLAKE3: &str =
    "a65a56ba1272ade0546c639ca4efd9782d6edabb0cc8845c9cded1ef8689af0f";
const XYCE_MESSAGE_MISSING_LIBRARY_ENDL_SOURCE_BLAKE3: &str =
    "f9b70136edeec498d0ce7429bc00810a0709c6943f806400bd1bdc4a3ecd06a6";
const XYCE_MESSAGE_MISSING_LIBRARY_FILE_UNQUOTED_SOURCE_BLAKE3: &str =
    "36cc809ea597e944ccc8eb92057c9eacd347fbd5b21ca2b092870db47d9db57e";
const XYCE_MESSAGE_MISSING_LIBRARY_FILE_QUOTED_SOURCE_BLAKE3: &str =
    "c70a89579ff9d39c287fd874bd7a7482a43aaae7e755bf521be46da0952edd2c";
const XYCE_MESSAGE_DUPLICATE_DEVICE_SOURCE_BLAKE3: &str =
    "ce5680dc24782e35dcb2ca9929a895306c47664591f11697ad51114e189f4171";
const XYCE_MESSAGE_MISSING_DEVICE_NODES_SOURCE_BLAKE3: &str =
    "a0564ed5888ea5c23b87ceca58ad61e9cc34cf974c5cb36922b0345994e2f278";
const XYCE_BUG702_DUP_EXTERNAL_SOURCE_BLAKE3: &str =
    "dbde7992544ec0958d58024f2f110606f770b83c7440f447833d20e51712691f";
const XYCE_BUG702_DUP_INLINED_SOURCE_BLAKE3: &str =
    "469a39662513264b899cd706c8fc279d24c3a630a3c274e7e05999b5406618bd";
const XYCE_BUG702_EMPTY_INITCOND_SOURCE_BLAKE3: &str =
    "ffc26554455d078bd6a2f66715ee65fa7f624422efbb97991f791998ece99874";
const XYCE_BUG702_MISSING_INITCOND_SOURCE_BLAKE3: &str =
    "0a75b775d3532734bcbcbdc24f47f156130dbefb32dd4ebee201d4a2c4eaa352";
const XYCE_BUG702_EXTERNAL_SOURCE_BLAKE3: &str =
    "4452f5b7b61579e9b1f9af22e377fbe8981afef85d6bf2d5532f91f0d6bea451";
const XYCE_BUG702_INLINED_MULTIPLE_SOURCE_BLAKE3: &str =
    "54ea0e1913a26ef18a67a1f001de5c8a6246fa0546fdb23a5a73fd58ba6034cd";
const XYCE_BUG702_INLINED_SINGLE_SOURCE_BLAKE3: &str =
    "f3d1cdc6272202931b57a5d1926e0828bb0e8aca6d2cfc43c3d5166dabda4728";
const XYCE_BUG702_PRECEDENCE_SOURCE_BLAKE3: &str =
    "e161ec6f122bfab85b62444b1ee1a0df866d8950717b7430ba01a5eb7626e45e";
const XYCE_BUG702_INITCOND_DATA_BLAKE3: &str =
    "aeb67f1437f1c271fc8803120cb2a4b9b2b476baf4313c3627634cf5732534f9";
const XYCE_BUG702_INITCOND_DATA_BYTES: usize = 26;
const XYCE_BUG702_NOINITS_DATA_BLAKE3: &str =
    "544397224dca2dae387a0754c1e039de23d7d9d8d75269ee9aeaba68eedcffba";
const XYCE_BUG702_NOINITS_DATA_BYTES: usize = 29;
const XYCE_BUG702_INV1XIC_REFERENCE_BLAKE3: &str =
    "14bf3cff491747a8ee5fa3e6e6dedf3a5672fd4773ab4576ecce0bb0f27de984";
const XYCE_BUG702_INV1XIC_REFERENCE_BYTES: usize = 208_655;
const XYCE_BUG702_NLRCS10_REFERENCE_BLAKE3: &str =
    "5c82790a07c8078d3503e3045aa6e79b967e8801423045103171c119c381b437";
const XYCE_BUG702_NLRCS10_REFERENCE_BYTES: usize = 95_773;
const XYCE_BUG702_CANONICAL_INV1XIC_SOURCE_BLAKE3: &str =
    "3d718166f51cc05da45f17893410e2119aa967753c19bd03dd2ffc7284b1a157";
const XYCE_BUG702_CANONICAL_NLRCS10_SOURCE_BLAKE3: &str =
    "2bd3ed0701cd20d41b69856d33d41e6bea50f10a2ccbac48088c9d661d49ac24";
const XYCE_ISSUE455_SOURCE_BLAKE3: &str =
    "9552abaee2c6162c1f1b389708fd6e338fb9ea212e04e1c8be5ea972bf04c875";
const XYCE_BUG204_SOURCE_BLAKE3: &str =
    "ed8984a5badf07bdc1cccab7a56cfa66303b678814dd5efcb7ca7a4b03daf947";
const XYCE_BUG281_SOURCE_BLAKE3: &str =
    "e7a31257432216b07ee6023392590aef62f2006d2111110242839b3d4c1c07d0";
const XYCE_BUG401_BAD_DEVICE_SOURCE_BLAKE3: &str =
    "53aaeb8ac36ee9748870b1eb59923eeae6e3faf19a4034fd1014457a637ff381";
const XYCE_BUG401_EXTRA_SPACE_SOURCE_BLAKE3: &str =
    "934e28a8ccfbd346bf66b6884c818d74d08fce9f2265363887ed9d2b569c07b5";
const XYCE_BUG401_WORSE_DEVICE_SOURCE_BLAKE3: &str =
    "d452d7f4844510bd2c1076ae6f6829d5c5ef421b36f0ee228a476900dd7e8fc3";
const XYCE_BUG701_TOPLEVEL_SOURCE_BLAKE3: &str =
    "0131acaf84d4e1602998aa6fe62660fdbee36d819070b81a470ba697d7dc8355";
const XYCE_BUG701_SUBCIRCUIT_SOURCE_BLAKE3: &str =
    "48e9bdb88c56c133bbbb6c5bfbe1c190b4fc8151c18be36247418d68cb8c847e";
const XYCE_BUG769_NODE_VOLTAGE_SOURCE_BLAKE3: &str =
    "ce0384a441aeaa20e0cce8cb6c9eb25f2965d8e674c092d823533a68c0656607";
const XYCE_BUG769_DEVICE_CURRENT_SOURCE_BLAKE3: &str =
    "769e5db9d7d51f1860c1e308d8c2374fb6dedc22604a47c9eb2eca1c5d5e7f59";
const XYCE_BUG769_LEAD_CURRENT_SOURCE_BLAKE3: &str =
    "fb46538cf279bddce942af6e7521aa276760c05d0d4211995de332be804bb60d";
const XYCE_BUG1578_SOURCE_BLAKE3: &str =
    "b7debe414f4dec2e06c857e139b128db9f7e64024d86d6baeab672250d9bbfc4";
const XYCE_BUG198_SOURCE_BLAKE3: &str =
    "985f4d4efa9b59842c268b526496f40e9f04988a9fc425e5a945bc16c6e24da7";
const XYCE_BUG258_SOURCE_BLAKE3: &str =
    "985f4d4efa9b59842c268b526496f40e9f04988a9fc425e5a945bc16c6e24da7";
const XYCE_BUG587_SOURCE_BLAKE3: &str =
    "c96b62b4de9ca3a7bd65910c29c505394c7d4047ec71e03b3657c7cb351cd4ac";
const XYCE_BUG401_PHYSICAL_CENSUS_BLAKE3: &str =
    "ccdc1cb4b46160d2c993d155a6816c81e5671aecd700bd364ab8b7d512320a13";
const XYCE_BUG401_MANIFEST_CENSUS_BLAKE3: &str =
    "427c97a38a5a66616877760976392652da5ddd5363a9abb64108bff5e40ea50b";
const XYCE_BUG701_PHYSICAL_CENSUS_BLAKE3: &str =
    "95e3fe513d175af26352a58c721926147fa8d61a106f2725baa16875738961e6";
const XYCE_BUG701_MANIFEST_CENSUS_BLAKE3: &str =
    "d0a587a88eb1110aa7f18e230d1be0ee6ae5e8a1f57ae5d9ab7123336c55afcc";
const XYCE_BUG769_PHYSICAL_CENSUS_BLAKE3: &str =
    "fd6b0fc758230e97ae8b2ed878e0a981a5c2d918ac1916408fa5c2b75eb93008";
const XYCE_BUG769_MANIFEST_CENSUS_BLAKE3: &str =
    "5dcd16e7d0b7fb3676533549a171865ac6b2a9aeb338f7f14ef10d73ec75dbc8";
const XYCE_BUG75_PHYSICAL_CENSUS_BLAKE3: &str =
    "5fba33cadd7946f239fa8ca6706f30dbf76ae63c4f5bba9d44278b555682b4fd";
const XYCE_BUG75_MANIFEST_CENSUS_BLAKE3: &str =
    "7edf926587419e6fba45e31c3d50b31c2a1a1d149bd8c2828d03661e04f43759";
const XYCE_BUG75_SOURCE_DIRECTORY_CENSUS_BLAKE3: &str =
    "516d58926a25ccc21284c72c2bc6fb5cf624f69ab056eadfd1d6b069eadb0e1a";
const XYCE_BUG75_EMPTY_OUTPUT_CENSUS_BLAKE3: &str =
    "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262";
const XYCE_BUG75_README_BLAKE3: &str =
    "11eb1f7062b3bb83509aa469fb6101ccb29a4c7681fcaaf5e93dada0f869bdea";
const XYCE_BUG75_README_BYTES: usize = 414;
const XYCE_BUG75_OPTIONS_BLAKE3: &str =
    "b1d67968e7446e26800d83b2f63ab18f63fd84b5b602758b2b2327bbdf15ef3b";
const XYCE_BUG75_OPTIONS_BYTES: usize = 14;
const XYCE_OUTPUT_SYMBOL_OPTIONS_BLAKE3: &str =
    "b1d67968e7446e26800d83b2f63ab18f63fd84b5b602758b2b2327bbdf15ef3b";
const XYCE_OUTPUT_SYMBOL_OPTIONS_BYTES: usize = 14;
const XYCE_BUG1148_PHYSICAL_CENSUS_BLAKE3: &str =
    "ac119116158a8781bda4860d6bce618dee17d497a1b07008255728a27340ebea";
const XYCE_BUG1148_MANIFEST_CENSUS_BLAKE3: &str =
    "444ec7285ebbb7da61300887823b9572d5124a5c4873fda753e193461c63bf95";
const XYCE_BUG40_PHYSICAL_CENSUS_BLAKE3: &str =
    "a99b9de40f7e32b6ba9e7835661a6d22d6067ab58140767d55ad8bbe023e4b39";
const XYCE_BUG40_MANIFEST_CENSUS_BLAKE3: &str =
    "fd7e3ba932274866eb0b1208f461257e5746a5342628a24c57716f80017baab3";
const XYCE_BUG718_PHYSICAL_CENSUS_BLAKE3: &str =
    "20eb41f65004e3e4060543181b2004d74086002327e13963cf70451a52a2c6c9";
const XYCE_BUG718_MANIFEST_CENSUS_BLAKE3: &str =
    "24ba969b3b9febf20a9c6d7daf71f220c303d5b6ce085127e96b311724cc04c7";
const XYCE_MESSAGE_PRINT_PHYSICAL_CENSUS_BLAKE3: &str =
    "e2361e63f931254fb1cb432255c5b684e885b6e4b7de79a74e954fa0d93706e1";
const XYCE_MESSAGE_PRINT_MANIFEST_CENSUS_BLAKE3: &str =
    "110a0757f67d20b148af340af3df80312f7f6e67e8da44155fb51d9523f1e397";
const XYCE_LEAD_CURRENTS_PHYSICAL_CENSUS_BLAKE3: &str =
    "7945aaa6487937d9d9eabcd4690c69643c600a88ae86cdf479f1baafa570eddd";
const XYCE_LEAD_CURRENTS_MANIFEST_CENSUS_BLAKE3: &str =
    "679c3ea6664d25eda3bb4eedbc7a3984bab14833d92bb9c57b1939aaf34c10ef";
const XYCE_MEASURE_PHYSICAL_CENSUS_BLAKE3: &str =
    "c28cba07612ac07b5e934ae6ddb8b2e0a1c1316a18d1d83bf7b78461e9605e41";
const XYCE_MEASURE_MANIFEST_CENSUS_BLAKE3: &str =
    "f6316ce77132d90d5b1142095236bff3584824cdd507e23907bde28b1e0cafa8";
const XYCE_FOURIER_PHYSICAL_CENSUS_BLAKE3: &str =
    "7d2de16d5e506c7bf1f82adfa924c99796844f851621c1900ddd36bea469e2a2";
const XYCE_FOURIER_MANIFEST_CENSUS_BLAKE3: &str =
    "58b62857cf810e7cdbb279d6289852df1c79e7afd9f4e3b0934316ba1822e752";
const XYCE_BUG1148_SOURCE_DIRECTORY_CENSUS_BLAKE3: &str =
    "c0a9d188b9704e6679379ff8e44a06c957c3c562700c5f8ade8a6dedccf2d0ab";
const XYCE_BUG1148_README_BLAKE3: &str =
    "28fae4b412da1d44c559a9b13049228044ea19d49e34c4c5e65244344065bc45";
const XYCE_BUG1148_README_BYTES: usize = 398;
const XYCE_BUG40_SOURCE_DIRECTORY_CENSUS_BLAKE3: &str =
    "37eb9791f65872d7682a7dccebbff69ed1e4f140fb27e6186721a8bb7f723212";
const XYCE_BUG40_README_BLAKE3: &str =
    "9bd772abdd9221479fadcbc3671f824da1a47f409a8c496a10cb28a8eb037a12";
const XYCE_BUG40_README_BYTES: usize = 740;
const XYCE_BUG40_OUT_BLAKE3: &str =
    "faa9b6622c09538c755394cc38da008c6900cd1cce7737d16a18c479e37cf162";
const XYCE_BUG40_OUT_BYTES: usize = 3_416;
const XYCE_BUG40_RETAINED_NON_ORACLE_PRN_BLAKE3: &str =
    "3b9727f72a4c72226dd83bf527675458f3a92a00b1171496d8d79ca1e6567d58";
const XYCE_BUG204_RETAINED_NON_ORACLE_PRN_BLAKE3: &str =
    "bcd3e366443f97db8ccb98d5d9f0102cbb67a5903657382da3ea7770ed666afc";
const XYCE_MESSAGE_SUBCIRCUIT_PHYSICAL_CENSUS_BLAKE3: &str =
    "dc8a7465b5524072cc0ef71b35809e306abc438d3fde69996a6ccc8889967da4";
const XYCE_MESSAGE_SUBCIRCUIT_MANIFEST_CENSUS_BLAKE3: &str =
    "ef67b79684be7a1f5f8e3daa3f448986ba05ab1afa66eac92d830f20afca0cc8";
const XYCE_MESSAGE_INPUT_PHYSICAL_CENSUS_BLAKE3: &str =
    "08e791377b7ae33d9f2611d0c40fad6a63f3d3d5ba2ad81c976c9e4e97402d9c";
const XYCE_MESSAGE_INPUT_MANIFEST_CENSUS_BLAKE3: &str =
    "5dcb3ede7c1a655558ed9b430597d05c088381c8708e2c47da82ddfdbb41417b";
const XYCE_MESSAGE_DEVICE_PHYSICAL_CENSUS_BLAKE3: &str =
    "d54485c64515210437cb32173963bd8c1383dfe4b392981220e4702444834c45";
const XYCE_MESSAGE_DEVICE_MANIFEST_CENSUS_BLAKE3: &str =
    "9ad114c0e228f83b9c5095e653c9692c98a3e5c434bb1f32ad7b71c5894d9843";
const XYCE_BUG702_PHYSICAL_CENSUS_BLAKE3: &str =
    "b0a43d2f55c88c8c32673364f8a7fa361771af4c4ef065ff4783146a1cc31628";
const XYCE_BUG702_MANIFEST_CENSUS_BLAKE3: &str =
    "91bcb74c1c7a203646ec14cd8754b3e520e0ad4ab589d8257f7edc14cb583dd6";
const XYCE_BUG702_SOURCE_DIRECTORY_CENSUS_BLAKE3: &str =
    "aea8d348997e22d1b024d59a9eab58cbf99eeab549119eaff091deb8b0491564";
const XYCE_BUG702_OUTPUT_DIRECTORY_CENSUS_BLAKE3: &str =
    "c896ecc7c724958b95072fd5972035335a23d96d4169bde9b4d07b550759a6c3";
const XYCE_BUG702_README_BLAKE3: &str =
    "599a576884838a0d5c0a57082945108cadb8af280ef31985cdef1058584d54b3";
const XYCE_BUG702_README_BYTES: usize = 2_571;
const XYCE_BUG702_OPTIONS_BLAKE3: &str =
    "b1d67968e7446e26800d83b2f63ab18f63fd84b5b602758b2b2327bbdf15ef3b";
const XYCE_BUG702_OPTIONS_BYTES: usize = 14;
const XYCE_BUG671_FIXTURE_BLAKE3: &str =
    "a20bed61d99b2bd530e4bdfdb096315198e1c4702ac3ff8e320b6ea42efce9ba";
const XYCE_BUG671_FIXTURE_BYTES: usize = 19_456;
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

    fn source_identity(self) -> (usize, &'static str) {
        match self {
            Self::IcMissingNodeWarning => (347, XYCE_IC_MISSING_NODE_WARNING_BLAKE3),
            Self::IcEmptyWarning => (309, XYCE_IC_EMPTY_WARNING_BLAKE3),
            Self::NodeSetMissingNodeWarning => (352, XYCE_NODESET_MISSING_NODE_WARNING_BLAKE3),
            Self::NodeSetEmptyWarning => (319, XYCE_NODESET_EMPTY_WARNING_BLAKE3),
            Self::Bug667ScopedGlobalWarning => (965, XYCE_BUG667_SCOPED_GLOBAL_WARNING_BLAKE3),
            Self::IcNodeSetConflict => (409, XYCE_IC_NODESET_CONFLICT_BLAKE3),
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

    fn is_message_input(self) -> bool {
        !matches!(self, Self::Bug667ScopedGlobalWarning)
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
struct XyceExpectedFailureFamilyCensus {
    physical_cir_count: usize,
    physical_names_blake3: &'static str,
    manifest_owner_count: usize,
    manifest_records_blake3: &'static str,
    require_manifest_bijection: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct XyceExpectedFailureRetainedArtifact {
    file_name: &'static str,
    bytes: usize,
    blake3: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct XyceExpectedFailureSourceSidecar {
    file_name: &'static str,
    bytes: usize,
    blake3: &'static str,
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

    fn record(self) -> &'static str {
        match self {
            Self::External => XYCE_BUG702_EXTERNAL_POSITIVE_RECORD,
            Self::InlinedMultiple => XYCE_BUG702_INLINED_MULTIPLE_POSITIVE_RECORD,
            Self::InlinedSingle => XYCE_BUG702_INLINED_SINGLE_POSITIVE_RECORD,
            Self::Precedence => XYCE_BUG702_PRECEDENCE_POSITIVE_RECORD,
        }
    }

    fn source_blake3(self) -> &'static str {
        match self {
            Self::External => XYCE_BUG702_EXTERNAL_SOURCE_BLAKE3,
            Self::InlinedMultiple => XYCE_BUG702_INLINED_MULTIPLE_SOURCE_BLAKE3,
            Self::InlinedSingle => XYCE_BUG702_INLINED_SINGLE_SOURCE_BLAKE3,
            Self::Precedence => XYCE_BUG702_PRECEDENCE_SOURCE_BLAKE3,
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

    fn alias_reference_identity(self) -> (usize, &'static str) {
        match self {
            Self::InlinedMultiple => (
                XYCE_BUG702_NLRCS10_REFERENCE_BYTES,
                XYCE_BUG702_NLRCS10_REFERENCE_BLAKE3,
            ),
            Self::External | Self::InlinedSingle | Self::Precedence => (
                XYCE_BUG702_INV1XIC_REFERENCE_BYTES,
                XYCE_BUG702_INV1XIC_REFERENCE_BLAKE3,
            ),
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

    fn source_blake3(self) -> &'static str {
        match self {
            Self::Bug67BehavioralExpression => XYCE_BUG67_SOURCE_BLAKE3,
            Self::Bug671InvalidPwlFile => XYCE_BUG671_SOURCE_BLAKE3,
            Self::Bug726AdjacentCouplings => XYCE_BUG726_SOURCE_BLAKE3,
            Self::Bug744DcOperatingPoint => XYCE_BUG744_SOURCE_BLAKE3,
            Self::Bug75UndefinedMutualInductorReference => XYCE_BUG75_SOURCE_BLAKE3,
            Self::Bug1595HierarchicalMutualInductorReference => XYCE_BUG1595_SOURCE_BLAKE3,
            Self::Bug1148UndefinedPrintNode => XYCE_BUG1148_SOURCE_BLAKE3,
            Self::Bug40UndefinedPrintNode => XYCE_BUG40_SOURCE_BLAKE3,
            Self::Bug718InvalidPrintNodes => XYCE_BUG718_INVALID_NODES_SOURCE_BLAKE3,
            Self::MessagePrintBadNodeName => XYCE_MESSAGE_PRINT_BAD_NODENAME_SOURCE_BLAKE3,
            Self::MessagePrintBadVariable => XYCE_MESSAGE_PRINT_BAD_VARIABLE_SOURCE_BLAKE3,
            Self::LeadCurrentsInvalidDevice => XYCE_LEAD_CURRENTS_INVALID_DEVICE_SOURCE_BLAKE3,
            Self::MeasureInvalidNodes => XYCE_MEASURE_INVALID_NODES_SOURCE_BLAKE3,
            Self::FourierBadLine3OutputSymbols => XYCE_FOURIER_BAD_LINE3_SOURCE_BLAKE3,
            Self::Bug387MissingLibraryEndl => XYCE_BUG387_SOURCE_BLAKE3,
            Self::MessageSubcircuitMissingName => XYCE_SUBCKT_NONAME_SOURCE_BLAKE3,
            Self::MessageSubcircuitMissingEndsEndCard => {
                XYCE_SUBCKT_MISSING_ENDS_END_CARD_SOURCE_BLAKE3
            }
            Self::MessageSubcircuitMissingEndsIncludeEof => {
                XYCE_SUBCKT_MISSING_ENDS_INCLUDE_EOF_SOURCE_BLAKE3
            }
            Self::MessageSubcircuitMissingEndsTopLevelEof => {
                XYCE_SUBCKT_MISSING_ENDS_TOPLEVEL_EOF_SOURCE_BLAKE3
            }
            Self::MessageSubcircuitMissingEndsTsInvEof => {
                XYCE_SUBCKT_MISSING_ENDS_TS_INV_EOF_SOURCE_BLAKE3
            }
            Self::MessageSubcircuitDuplicateBindingA2 => XYCE_SUBCKT_A2_DUP_BINDING_SOURCE_BLAKE3,
            Self::MessageSubcircuitDuplicateBindingJ1 => XYCE_SUBCKT_J1_DUP_BINDING_SOURCE_BLAKE3,
            Self::MessageDcExcessArguments => XYCE_DC_EXCESS_ARGS_SOURCE_BLAKE3,
            Self::MessageAcUnsupportedSweepType => XYCE_AC_UNSUPPORTED_SWEEP_SOURCE_BLAKE3,
            Self::MessageNoiseUnsupportedSweepType => XYCE_NOISE_UNSUPPORTED_SWEEP_SOURCE_BLAKE3,
            Self::MessageMissingLibraryEndl => XYCE_MESSAGE_MISSING_LIBRARY_ENDL_SOURCE_BLAKE3,
            Self::MessageMissingLibraryFileUnquoted => {
                XYCE_MESSAGE_MISSING_LIBRARY_FILE_UNQUOTED_SOURCE_BLAKE3
            }
            Self::MessageMissingLibraryFileQuoted => {
                XYCE_MESSAGE_MISSING_LIBRARY_FILE_QUOTED_SOURCE_BLAKE3
            }
            Self::MessageDuplicateDevice => XYCE_MESSAGE_DUPLICATE_DEVICE_SOURCE_BLAKE3,
            Self::MessageMissingDeviceNodes => XYCE_MESSAGE_MISSING_DEVICE_NODES_SOURCE_BLAKE3,
            Self::Bug702DuplicateExternalInitcond => XYCE_BUG702_DUP_EXTERNAL_SOURCE_BLAKE3,
            Self::Bug702DuplicateInlinedInitcond => XYCE_BUG702_DUP_INLINED_SOURCE_BLAKE3,
            Self::Bug702MalformedInitcondFile => XYCE_BUG702_EMPTY_INITCOND_SOURCE_BLAKE3,
            Self::Bug702MissingInitcondFile => XYCE_BUG702_MISSING_INITCOND_SOURCE_BLAKE3,
            Self::Issue455DuplicateDcSourceFunction => XYCE_ISSUE455_SOURCE_BLAKE3,
            Self::Bug204InvalidDcSweepArity => XYCE_BUG204_SOURCE_BLAKE3,
            Self::Bug281InvalidDcSweepArity => XYCE_BUG281_SOURCE_BLAKE3,
            Self::Bug354BadFunction => XYCE_BUG354_FUNCTION_SOURCE_BLAKE3,
            Self::Bug354BadLeadCurrent => XYCE_BUG354_LEAD_CURRENT_SOURCE_BLAKE3,
            Self::Bug354BadParameter => XYCE_BUG354_PARAMETER_SOURCE_BLAKE3,
            Self::Bug401BadDeviceLine => XYCE_BUG401_BAD_DEVICE_SOURCE_BLAKE3,
            Self::Bug401ExtraSpace => XYCE_BUG401_EXTRA_SPACE_SOURCE_BLAKE3,
            Self::Bug401WorseDeviceLine => XYCE_BUG401_WORSE_DEVICE_SOURCE_BLAKE3,
            Self::Bug701DuplicateTopLevelDevice => XYCE_BUG701_TOPLEVEL_SOURCE_BLAKE3,
            Self::Bug701DuplicateSubcircuitDevice => XYCE_BUG701_SUBCIRCUIT_SOURCE_BLAKE3,
            Self::Bug769ParameterNodeVoltage => XYCE_BUG769_NODE_VOLTAGE_SOURCE_BLAKE3,
            Self::Bug769ParameterDeviceCurrent => XYCE_BUG769_DEVICE_CURRENT_SOURCE_BLAKE3,
            Self::Bug769ParameterLeadCurrent => XYCE_BUG769_LEAD_CURRENT_SOURCE_BLAKE3,
            Self::Bug1578InvalidDeviceType => XYCE_BUG1578_SOURCE_BLAKE3,
            Self::Bug198UnrecognizedLine => XYCE_BUG198_SOURCE_BLAKE3,
            Self::Bug258UnrecognizedLine => XYCE_BUG258_SOURCE_BLAKE3,
            Self::Bug587InvalidNumericNotation => XYCE_BUG587_SOURCE_BLAKE3,
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

    fn shared_family_census(self) -> Option<XyceExpectedFailureFamilyCensus> {
        match self {
            Self::MessageSubcircuitMissingName
            | Self::MessageSubcircuitMissingEndsEndCard
            | Self::MessageSubcircuitMissingEndsIncludeEof
            | Self::MessageSubcircuitMissingEndsTopLevelEof
            | Self::MessageSubcircuitMissingEndsTsInvEof
            | Self::MessageSubcircuitDuplicateBindingA2
            | Self::MessageSubcircuitDuplicateBindingJ1 => Some(XyceExpectedFailureFamilyCensus {
                physical_cir_count: 8,
                physical_names_blake3: XYCE_MESSAGE_SUBCIRCUIT_PHYSICAL_CENSUS_BLAKE3,
                manifest_owner_count: 8,
                manifest_records_blake3: XYCE_MESSAGE_SUBCIRCUIT_MANIFEST_CENSUS_BLAKE3,
                require_manifest_bijection: false,
            }),
            Self::MessageDcExcessArguments
            | Self::MessageAcUnsupportedSweepType
            | Self::MessageNoiseUnsupportedSweepType
            | Self::MessageMissingLibraryEndl
            | Self::MessageMissingLibraryFileUnquoted
            | Self::MessageMissingLibraryFileQuoted => Some(XyceExpectedFailureFamilyCensus {
                physical_cir_count: 79,
                physical_names_blake3: XYCE_MESSAGE_INPUT_PHYSICAL_CENSUS_BLAKE3,
                manifest_owner_count: 50,
                manifest_records_blake3: XYCE_MESSAGE_INPUT_MANIFEST_CENSUS_BLAKE3,
                require_manifest_bijection: false,
            }),
            Self::MessageDuplicateDevice | Self::MessageMissingDeviceNodes => {
                Some(XyceExpectedFailureFamilyCensus {
                    physical_cir_count: 28,
                    physical_names_blake3: XYCE_MESSAGE_DEVICE_PHYSICAL_CENSUS_BLAKE3,
                    manifest_owner_count: 26,
                    manifest_records_blake3: XYCE_MESSAGE_DEVICE_MANIFEST_CENSUS_BLAKE3,
                    require_manifest_bijection: false,
                })
            }
            Self::Bug702DuplicateExternalInitcond
            | Self::Bug702DuplicateInlinedInitcond
            | Self::Bug702MalformedInitcondFile
            | Self::Bug702MissingInitcondFile => Some(XyceExpectedFailureFamilyCensus {
                physical_cir_count: 8,
                physical_names_blake3: XYCE_BUG702_PHYSICAL_CENSUS_BLAKE3,
                manifest_owner_count: 8,
                manifest_records_blake3: XYCE_BUG702_MANIFEST_CENSUS_BLAKE3,
                require_manifest_bijection: true,
            }),
            Self::Bug401BadDeviceLine | Self::Bug401ExtraSpace | Self::Bug401WorseDeviceLine => {
                Some(XyceExpectedFailureFamilyCensus {
                    physical_cir_count: 3,
                    physical_names_blake3: XYCE_BUG401_PHYSICAL_CENSUS_BLAKE3,
                    manifest_owner_count: 3,
                    manifest_records_blake3: XYCE_BUG401_MANIFEST_CENSUS_BLAKE3,
                    require_manifest_bijection: true,
                })
            }
            Self::Bug354BadFunction | Self::Bug354BadLeadCurrent | Self::Bug354BadParameter => {
                Some(XyceExpectedFailureFamilyCensus {
                    physical_cir_count: 3,
                    physical_names_blake3: XYCE_BUG354_PHYSICAL_CENSUS_BLAKE3,
                    manifest_owner_count: 3,
                    manifest_records_blake3: XYCE_BUG354_MANIFEST_CENSUS_BLAKE3,
                    require_manifest_bijection: true,
                })
            }
            Self::Bug701DuplicateTopLevelDevice | Self::Bug701DuplicateSubcircuitDevice => {
                Some(XyceExpectedFailureFamilyCensus {
                    physical_cir_count: 2,
                    physical_names_blake3: XYCE_BUG701_PHYSICAL_CENSUS_BLAKE3,
                    manifest_owner_count: 2,
                    manifest_records_blake3: XYCE_BUG701_MANIFEST_CENSUS_BLAKE3,
                    require_manifest_bijection: true,
                })
            }
            Self::Bug769ParameterNodeVoltage
            | Self::Bug769ParameterDeviceCurrent
            | Self::Bug769ParameterLeadCurrent => Some(XyceExpectedFailureFamilyCensus {
                physical_cir_count: 3,
                physical_names_blake3: XYCE_BUG769_PHYSICAL_CENSUS_BLAKE3,
                manifest_owner_count: 3,
                manifest_records_blake3: XYCE_BUG769_MANIFEST_CENSUS_BLAKE3,
                require_manifest_bijection: true,
            }),
            Self::Bug75UndefinedMutualInductorReference => Some(XyceExpectedFailureFamilyCensus {
                physical_cir_count: 1,
                physical_names_blake3: XYCE_BUG75_PHYSICAL_CENSUS_BLAKE3,
                manifest_owner_count: 1,
                manifest_records_blake3: XYCE_BUG75_MANIFEST_CENSUS_BLAKE3,
                require_manifest_bijection: true,
            }),
            Self::Bug1595HierarchicalMutualInductorReference => {
                Some(XyceExpectedFailureFamilyCensus {
                    physical_cir_count: 1,
                    physical_names_blake3: XYCE_BUG1595_PHYSICAL_CENSUS_BLAKE3,
                    manifest_owner_count: 1,
                    manifest_records_blake3: XYCE_BUG1595_MANIFEST_CENSUS_BLAKE3,
                    require_manifest_bijection: true,
                })
            }
            Self::Bug1148UndefinedPrintNode => Some(XyceExpectedFailureFamilyCensus {
                physical_cir_count: 1,
                physical_names_blake3: XYCE_BUG1148_PHYSICAL_CENSUS_BLAKE3,
                manifest_owner_count: 1,
                manifest_records_blake3: XYCE_BUG1148_MANIFEST_CENSUS_BLAKE3,
                require_manifest_bijection: true,
            }),
            Self::Bug40UndefinedPrintNode => Some(XyceExpectedFailureFamilyCensus {
                physical_cir_count: 1,
                physical_names_blake3: XYCE_BUG40_PHYSICAL_CENSUS_BLAKE3,
                manifest_owner_count: 1,
                manifest_records_blake3: XYCE_BUG40_MANIFEST_CENSUS_BLAKE3,
                require_manifest_bijection: true,
            }),
            Self::Bug718InvalidPrintNodes => Some(XyceExpectedFailureFamilyCensus {
                physical_cir_count: 2,
                physical_names_blake3: XYCE_BUG718_PHYSICAL_CENSUS_BLAKE3,
                manifest_owner_count: 2,
                manifest_records_blake3: XYCE_BUG718_MANIFEST_CENSUS_BLAKE3,
                require_manifest_bijection: true,
            }),
            Self::MessagePrintBadNodeName | Self::MessagePrintBadVariable => {
                Some(XyceExpectedFailureFamilyCensus {
                    physical_cir_count: 4,
                    physical_names_blake3: XYCE_MESSAGE_PRINT_PHYSICAL_CENSUS_BLAKE3,
                    manifest_owner_count: 4,
                    manifest_records_blake3: XYCE_MESSAGE_PRINT_MANIFEST_CENSUS_BLAKE3,
                    require_manifest_bijection: true,
                })
            }
            Self::LeadCurrentsInvalidDevice => Some(XyceExpectedFailureFamilyCensus {
                physical_cir_count: 53,
                physical_names_blake3: XYCE_LEAD_CURRENTS_PHYSICAL_CENSUS_BLAKE3,
                manifest_owner_count: 9,
                manifest_records_blake3: XYCE_LEAD_CURRENTS_MANIFEST_CENSUS_BLAKE3,
                require_manifest_bijection: false,
            }),
            Self::MeasureInvalidNodes => Some(XyceExpectedFailureFamilyCensus {
                physical_cir_count: 80,
                physical_names_blake3: XYCE_MEASURE_PHYSICAL_CENSUS_BLAKE3,
                manifest_owner_count: 114,
                manifest_records_blake3: XYCE_MEASURE_MANIFEST_CENSUS_BLAKE3,
                require_manifest_bijection: false,
            }),
            Self::FourierBadLine3OutputSymbols => Some(XyceExpectedFailureFamilyCensus {
                physical_cir_count: 15,
                physical_names_blake3: XYCE_FOURIER_PHYSICAL_CENSUS_BLAKE3,
                manifest_owner_count: 13,
                manifest_records_blake3: XYCE_FOURIER_MANIFEST_CENSUS_BLAKE3,
                require_manifest_bijection: false,
            }),
            _ => None,
        }
    }

    fn retained_non_oracle_artifact(self) -> Option<XyceExpectedFailureRetainedArtifact> {
        match self {
            Self::Bug204InvalidDcSweepArity => Some(XyceExpectedFailureRetainedArtifact {
                file_name: "bug204.cir.prn",
                bytes: 147,
                blake3: XYCE_BUG204_RETAINED_NON_ORACLE_PRN_BLAKE3,
            }),
            Self::Bug40UndefinedPrintNode => Some(XyceExpectedFailureRetainedArtifact {
                file_name: "bug_40.cir.prn",
                bytes: 150,
                blake3: XYCE_BUG40_RETAINED_NON_ORACLE_PRN_BLAKE3,
            }),
            _ => None,
        }
    }

    fn expected_source_sidecar(self) -> Option<XyceExpectedFailureSourceSidecar> {
        let file_name = match self {
            Self::Bug718InvalidPrintNodes => "invalidNodes.cir.options",
            Self::LeadCurrentsInvalidDevice => "lead_for_invalid_device.cir.options",
            Self::MeasureInvalidNodes => "invalid_nodes.cir.options",
            Self::FourierBadLine3OutputSymbols => "bad_dot_four_line3.cir.options",
            _ => return None,
        };
        Some(XyceExpectedFailureSourceSidecar {
            file_name,
            bytes: XYCE_OUTPUT_SYMBOL_OPTIONS_BYTES,
            blake3: XYCE_OUTPUT_SYMBOL_OPTIONS_BLAKE3,
        })
    }

    fn rejects_source_directory_sidecars(self) -> bool {
        matches!(
            self,
            Self::Bug75UndefinedMutualInductorReference
                | Self::Bug1595HierarchicalMutualInductorReference
                | Self::Bug1148UndefinedPrintNode
                | Self::Bug40UndefinedPrintNode
                | Self::Bug718InvalidPrintNodes
                | Self::MessagePrintBadNodeName
                | Self::MessagePrintBadVariable
                | Self::LeadCurrentsInvalidDevice
                | Self::MeasureInvalidNodes
                | Self::FourierBadLine3OutputSymbols
                | Self::MessageAcUnsupportedSweepType
                | Self::MessageNoiseUnsupportedSweepType
                | Self::MessageSubcircuitDuplicateBindingA2
                | Self::MessageSubcircuitDuplicateBindingJ1
                | Self::MessageMissingLibraryEndl
                | Self::MessageMissingLibraryFileUnquoted
                | Self::MessageMissingLibraryFileQuoted
                | Self::MessageDuplicateDevice
                | Self::MessageMissingDeviceNodes
        )
    }

    fn is_bug702_family(self) -> bool {
        matches!(
            self,
            Self::Bug702DuplicateExternalInitcond
                | Self::Bug702DuplicateInlinedInitcond
                | Self::Bug702MalformedInitcondFile
                | Self::Bug702MissingInitcondFile
        )
    }

    fn is_bug75(self) -> bool {
        self == Self::Bug75UndefinedMutualInductorReference
    }

    fn is_bug1595(self) -> bool {
        self == Self::Bug1595HierarchicalMutualInductorReference
    }

    fn has_complete_output_symbol_family_envelope(self) -> bool {
        matches!(
            self,
            Self::Bug1148UndefinedPrintNode | Self::Bug40UndefinedPrintNode
        )
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

    fn source_identity(self) -> (usize, &'static str) {
        match self {
            Self::GndExclamation => (
                455,
                "fb7b552397cff95363430acd8ab5bf0792005cfff40e515ce20b80bb52aab0c2",
            ),
            Self::Gnd => (
                455,
                "d7fc61d258f0ad0f25535ac05a5e185d72a330aed8d6bdbf004da8b2d9e3a258",
            ),
            Self::Ground => (
                460,
                "ffb1e9c3d1c313dc78f60f813421115adc320872b5e4d4b9755d32b3d245e983",
            ),
            Self::SubcircuitInstantiation => (
                732,
                "6ad3d7ff0187cb31c3c663355bf8065d6d190f32cf1d058e6be5041abe3c8c1e",
            ),
        }
    }

    fn hspice_identity(self) -> (usize, &'static str) {
        match self {
            Self::GndExclamation => (
                331,
                "56873a1f3a09f35f150881bb303522bfd86fc6e76a9f4d6fc8b54877776f5081",
            ),
            Self::Gnd => (
                329,
                "f57040ab2f79c2a5c9b97d8926b2f5a09aef9e4885ce5e46c993e5b6639725f6",
            ),
            Self::Ground => (
                384,
                "25e79e5ca36bad4d780f31418c1ee5097eb7c1991feddd673263af27fa783230",
            ),
            Self::SubcircuitInstantiation => (
                581,
                "247d13e7043ebbf8074f457eb9a08bea8a0d197924eae80ddb02d5eb504293f2",
            ),
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

    fn source_relative_path(self) -> &'static str {
        match self {
            Self::UnaryMinusPrecedence => "Netlists/ABM_POW/abmpow1.cir",
            Self::NegativeIntegerExponent => "Netlists/ABM_POW/abmpow2.cir",
            Self::FractionalPrincipalComplex => "Netlists/ABM_POW/abmpow3.cir",
        }
    }

    fn source_identity(self) -> (usize, &'static str) {
        match self {
            Self::UnaryMinusPrecedence => (
                383,
                "26c41b1da2a996e49a877cfe72fca3a8ba4efc2b9cb9b626e0049977c06a349b",
            ),
            Self::NegativeIntegerExponent => (
                481,
                "d9a7f907b68ba8ba393572508efb283a08980daa85cd4906dfaa0530c43489ad",
            ),
            Self::FractionalPrincipalComplex => (
                523,
                "0395a7d3c8f7630e47aa92c7b02d7be840ca2fa258d394d64a7ec9377de12a3b",
            ),
        }
    }

    /// Immutable audit identity of the removed Release 7.10 shell wrapper.
    /// The artifact is intentionally not vendored; this is SHA-256 over the
    /// canonical Xyce-Regression-history bytes.
    fn historical_wrapper_identity(self) -> (usize, &'static str) {
        match self {
            Self::UnaryMinusPrecedence => (
                1317,
                "82cbb932d8e66ead0da5a5b76706b002edc01b729ca43bd7ae5bf59f0c8a87f9",
            ),
            Self::NegativeIntegerExponent => (
                1317,
                "e506a1a09bddb9b87e3d18b42d7bc6fd4cf9c4bc436ce3519363d3c977cc5810",
            ),
            Self::FractionalPrincipalComplex => (
                1317,
                "7e9a7edba0297dff479afd1cbcc7f194f76ca36ea8ea0608580a8915e71f1479",
            ),
        }
    }

    /// Immutable SHA-256 identity of the Perl `.prn.gs` generator invoked by
    /// the corresponding historical shell wrapper.
    fn historical_perl_identity(self) -> (usize, &'static str) {
        match self {
            Self::UnaryMinusPrecedence => (
                482,
                "59773e5ec007455f7284a62006f89a993103dc6a2c045ff3f392b60546e4db23",
            ),
            Self::NegativeIntegerExponent => (
                564,
                "fbe426be52c542c082cbc56f12f713a6e2b6fe0847e7e7b75360a5db0c96481a",
            ),
            Self::FractionalPrincipalComplex => (
                1647,
                "ae6134403646e522b17845aed86c58fdef866dd80a4209241ef7a4868a20d699",
            ),
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

    fn source_relative_path(self) -> &'static str {
        match self {
            Self::DirectTime => "Netlists/ABM_TIME/time.cir",
            Self::ParameterTime => "Netlists/ABM_TIME/time_param.cir",
            Self::SquareRoot => "Netlists/ABM_SQRT/sqrt.cir",
        }
    }

    fn source_identity(self) -> (usize, &'static str) {
        match self {
            Self::DirectTime => (
                1652,
                "8b3000ba77b14f3afbb7ef92517f480f4960d7ece02a32430567787bf7331960",
            ),
            Self::ParameterTime => (
                1678,
                "67db073fdc5f21c0604aaa087b44310ca11efef7ee4b64ebc35100e0f8db1db9",
            ),
            Self::SquareRoot => (
                1444,
                "489b81dfda01f2c8339a7d12042597ed70d1f97462b5da72b8b99fae470e5661",
            ),
        }
    }

    fn historical_wrapper_identity(self) -> (usize, &'static str) {
        match self {
            Self::DirectTime => (
                1259,
                "d81921bdbb1cfc58c17ca024b02dbe2f05fa5f294a9600b46d81da46c1b7ae4f",
            ),
            Self::ParameterTime => (
                1271,
                "7933696b9e61c08d509d1673ac1aa6f040f465ed295ab679fb0b64c30ad098a5",
            ),
            Self::SquareRoot => (
                1259,
                "fb977e06d51ec5f8cf550e2868505b061a85ebfbbd11013a86351eacaea295bd",
            ),
        }
    }

    fn historical_perl_identity(self) -> (usize, &'static str) {
        match self {
            Self::DirectTime => (
                384,
                "7a5793fcdf29d886ff789e7076b7b92b7d8209d8decb03370286768884074730",
            ),
            Self::ParameterTime => (
                396,
                "51aa9bd0f445c1915cdfd769a8348c3aba709ef3266d730bfea2948862c37b3c",
            ),
            Self::SquareRoot => (
                518,
                "22789907f0aa9d9902fa8ba4280e71fa1c9bfd135cf8d83db5cdeabb25786ecf",
            ),
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

    fn source_relative_path(self) -> &'static str {
        match self {
            Self::Derivative => "Netlists/MEASURE_CONT/DerivTestTran.cir",
            Self::FindWhen => "Netlists/MEASURE_CONT/FindWhenTestTran.cir",
            Self::TriggerTarget => "Netlists/MEASURE_CONT/TrigTargTestTran.cir",
        }
    }

    fn source_identity(self) -> (usize, &'static str) {
        match self {
            Self::Derivative => (
                5130,
                "004473ef36c30a6a553d095c6355c6a4dd8c2f7c91f06c08edba31a4c27332ea",
            ),
            Self::FindWhen => (
                5571,
                "4eb1bb52057c9a82891b5ddf2606fed743d97cf949a993ec341d3415f75a5fe6",
            ),
            Self::TriggerTarget => (
                2290,
                "d400ebddf0b080039ef8905a0c31e12050d45912449d0922c703edc0682be7de",
            ),
        }
    }

    fn gs_relative_path(self) -> &'static str {
        match self {
            Self::Derivative => "Netlists/MEASURE_CONT/DerivTestTranGSfile",
            Self::FindWhen => "Netlists/MEASURE_CONT/FindWhenTestTranGSfile",
            Self::TriggerTarget => "Netlists/MEASURE_CONT/TrigTargTestTranGSfile",
        }
    }

    fn gs_identity(self) -> (usize, &'static str) {
        match self {
            Self::Derivative => (
                7253,
                "acd7911ebd8f8a1468b29cf5784e4b09595bfc77519ec2659e991b670a379b54",
            ),
            Self::FindWhen => (
                8318,
                "df50283caa48e7c9c7d5ee10596c0391066ed7ee5797a6abe1f4910e71b9dcde",
            ),
            Self::TriggerTarget => (
                2298,
                "30aa8eedae1aac04089df40d584262f0d1131f0a4c3db7da7bea06a3c9a79c64",
            ),
        }
    }

    fn mt0_relative_path(self) -> &'static str {
        match self {
            Self::Derivative => "OutputData/MEASURE_CONT/DerivTestTran.cir.mt0",
            Self::FindWhen => "OutputData/MEASURE_CONT/FindWhenTestTran.cir.mt0",
            Self::TriggerTarget => "OutputData/MEASURE_CONT/TrigTargTestTran.cir.mt0",
        }
    }

    fn mt0_identity(self) -> (usize, &'static str) {
        match self {
            Self::Derivative => (
                2120,
                "8a7748dd3530a93a72a4293216ab37d92542821c38a99c910f438a1b410eb6ce",
            ),
            Self::FindWhen => (
                2824,
                "563d8958716dea4b2e32ddbb39895546bd329b43455bea1a2154e39c8b4b9567",
            ),
            Self::TriggerTarget => (
                3053,
                "bd021857acf3848c2cb1ba0135182d1e7aea13d0e1b63b3befd804628b859241",
            ),
        }
    }

    fn prn(self) -> Option<(&'static str, (usize, &'static str))> {
        match self {
            Self::Derivative => Some((
                "OutputData/MEASURE_CONT/DerivTestTran.cir.prn",
                (
                    11461,
                    "ea8e721952ec7a338fcae39dd8e642b9eeb4092a5fd69f92829d7318834e6127",
                ),
            )),
            Self::FindWhen => Some((
                "OutputData/MEASURE_CONT/FindWhenTestTran.cir.prn",
                (
                    11047,
                    "ba38099177433ebc963d2e7c72e27f682d52244caa38530d4fdecd67157c380b",
                ),
            )),
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

    fn historical_wrapper_identity(self) -> (usize, &'static str) {
        match self {
            Self::Derivative => (
                5548,
                "0fdf11b63034827f7cf28932d0fc3b10f9e13b9d2dbb71e5d0ff11b3bbd970b3",
            ),
            Self::FindWhen => (
                5554,
                "049425724c3a99978314ea8f6f6f195284ddee5035559faece1311e5f4218911",
            ),
            Self::TriggerTarget => (
                5173,
                "10dfe7487e5c7b0519644326465be4ecc5143f76661822a0ad8f3f08ce463ee6",
            ),
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

    fn source_identity(self) -> (usize, &'static str) {
        match (self.kind, self.role) {
            (XyceMeasureContStepTranKind::Derivative, XyceMeasureContStepTranRole::Main) => (
                4664,
                "d0b8736c42b745cc7a6fb9211c7c590d0edd1419e1536c0a95a8d7bf077108e0",
            ),
            (XyceMeasureContStepTranKind::Derivative, XyceMeasureContStepTranRole::Control0) => (
                4634,
                "db7840ef9561211757089de8e3a613e75912adb1e30d93cdc445117ea940f22c",
            ),
            (XyceMeasureContStepTranKind::Derivative, XyceMeasureContStepTranRole::Control1) => (
                4624,
                "c81ffd44d57f368e3d6f91428ce5fd08a884b5a93fdf462e58f4f06e6271a59f",
            ),
            (XyceMeasureContStepTranKind::FindWhen, XyceMeasureContStepTranRole::Main) => (
                5163,
                "e8e70a2b74f3447115820a91889a71e83e6d52777efce9b0bc4aec226c604204",
            ),
            (XyceMeasureContStepTranKind::FindWhen, XyceMeasureContStepTranRole::Control0) => (
                5143,
                "c6913f3e20e2cea5863569bb3fecce42cdc34663d63f1507a514e70717f93a99",
            ),
            (XyceMeasureContStepTranKind::FindWhen, XyceMeasureContStepTranRole::Control1) => (
                5143,
                "22148f040cb4fcc39198377735ce768fa06412d6fda552f5403847ef8b929329",
            ),
            (XyceMeasureContStepTranKind::TriggerTarget, XyceMeasureContStepTranRole::Main) => (
                1230,
                "3fc469a02df27f52125f87c8e67cf770df7a3577865e4ff563d16d2c6e783f1c",
            ),
            (XyceMeasureContStepTranKind::TriggerTarget, XyceMeasureContStepTranRole::Control0) => {
                (
                    1066,
                    "88ceb4d1da165bbf42262c0fbbe319bcaff9688cb3f47973ff048f5a4cf40737",
                )
            }
            (XyceMeasureContStepTranKind::TriggerTarget, XyceMeasureContStepTranRole::Control1) => {
                (
                    1042,
                    "e7a6d985ed8ded5775f568f754a9dc3862767671fd4946dd9564bc4fdba3757e",
                )
            }
        }
    }

    fn historical_wrapper_identity(self) -> Option<(usize, &'static str)> {
        (self.role == XyceMeasureContStepTranRole::Main).then_some(match self.kind {
            XyceMeasureContStepTranKind::Derivative | XyceMeasureContStepTranKind::FindWhen => (
                1675,
                "ed6d34830f91743b7811803fd5cf17ce3423026735ee419026b023066045d148",
            ),
            XyceMeasureContStepTranKind::TriggerTarget => (
                1673,
                "1f8e950b16a78f30b4895dda83d7a47a91ae7fc1a731cdb8ab008827506fc0fb",
            ),
        })
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

    fn source_identity(self) -> (usize, &'static str) {
        match self.role {
            XyceMeasureStepFindWhenRole::Owner => (
                3388,
                "91b974929771cd6ba2f57d87868664153e8fe47e42146a4bbcced087141da3c6",
            ),
            XyceMeasureStepFindWhenRole::Control0 => (
                3465,
                "39c7e32b64ba2305f105885ddc156c79c204ffd5a7ab5e41f8bd8bf988c5a6f7",
            ),
            XyceMeasureStepFindWhenRole::Control1 => (
                3465,
                "5f2c494542bf786f8672f9f031192c17d84da2059b0b2e598cca6528c8dd3198",
            ),
            XyceMeasureStepFindWhenRole::Control2 => (
                3478,
                "75e76a60a08d70f86979e16d31417b614e1693b2de21fe6384bdc64830da2be6",
            ),
            XyceMeasureStepFindWhenRole::Control3 => (
                3463,
                "0323c7eb942e8bad81072c853dc4eb95fad95ef326924a9fa517a1065ad9f156",
            ),
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

    fn source_identity(self) -> (usize, &'static str) {
        match self {
            Self::ReplaceGround => (
                3846,
                "d717c5eeb4c47ae3ab8e6a857c7af052f9c2233cd223b104042c2616ba55c44b",
            ),
            Self::LiteralGroundNames => (
                1870,
                "99a6eef49614bf18b8414a3d47435dfaf76b8b528705f2ee9dbcd3c04f8d34b2",
            ),
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

    fn source_relative_path(self) -> &'static str {
        match self {
            Self::NoDcPath => "Netlists/PREPROC_ADDRES/nodcpath.cir",
            Self::OneTerminal => "Netlists/PREPROC_ADDRES/oneterm.cir",
            Self::RedundantBridge => "Netlists/REDUND_REMOVE/gnd_and_redund_addres.cir",
        }
    }

    fn source_identity(self) -> (usize, &'static str) {
        match self {
            Self::NoDcPath => (
                3423,
                "45b3c95d6a31422b19b0db8fdf81ef4dbcde42bf9d0d5418b25cbf0a40fd6a74",
            ),
            Self::OneTerminal => (
                3439,
                "81be55762a70572c0c5af4cc4f90cc3934069819010d45016dd6afd02f9901ea",
            ),
            Self::RedundantBridge => (
                4131,
                "1aa527ead8cfa0888d74186ee07881c128141782b4843d330dd672cad54933e6",
            ),
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
            XyceStaticTranOracle::None | XyceStaticTranOracle::ScalarMeasurements { .. } => Err(
                format!("{consumer} cannot replace a non-waveform transient oracle"),
            ),
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
            XyceStaticTranOracle::None | XyceStaticTranOracle::Waveform(_) => None,
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
    owner_path: PathBuf,
    role: XyceParams1Role,
}

#[derive(Debug, Clone)]
struct XyceNakedAlgebraFamilyContract {
    relational: XyceBaselineFamilyContract,
    owner_path: PathBuf,
    role: XyceNakedAlgebraRole,
}

#[derive(Debug, Clone)]
struct XyceBug1826ThermalParameterFamilyContract {
    relational: XyceBaselineFamilyContract,
    owner_path: PathBuf,
    support_path: PathBuf,
    role: XyceBug1826ThermalParameterRole,
}

#[derive(Debug, Clone)]
struct XyceSourceMultiplicityFamilyContract {
    relational: XyceBaselineFamilyContract,
    owner_path: PathBuf,
    baseline_path: PathBuf,
    spec: &'static XyceSourceMultiplicityCaseSpec,
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
    spec: &'static XyceBug1043AcDataParameterCaseSpec,
    role: XyceBug1043AcDataParameterRole,
}

#[derive(Debug, Clone)]
struct XyceAbmLookupOrderFamilyContract {
    relational: XyceBaselineFamilyContract,
    owner_path: PathBuf,
    control_path: PathBuf,
    spec: &'static XyceAbmLookupOrderCaseSpec,
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

    fn record(self) -> &'static str {
        match self {
            Self::AgaussAbsolute => XYCE_BUG39_AGAUSS_RECORD,
            Self::GaussRelative => XYCE_BUG39_GAUSS_RECORD,
        }
    }

    fn generated_file_name(self) -> &'static str {
        match self {
            Self::AgaussAbsolute => "testagauss.cir",
            Self::GaussRelative => "testgauss.cir",
        }
    }

    fn generated_source_blake3(self) -> &'static str {
        match self {
            Self::AgaussAbsolute => {
                "1c1bc1b8d196596b7243940066cdc0ed78466f7828a11428e0abdbd84b822aa4"
            }
            Self::GaussRelative => {
                "43c397a68a44734ca24c199e7ee40cfb3104eafd675644767f91f7db4c01ef2d"
            }
        }
    }

    fn generated_source_sha256(self) -> &'static str {
        match self {
            Self::AgaussAbsolute => {
                "ae8438d949eb59deeb7d7b37542a1d1399a0c1c536c87adb51c158a7fcb8acf0"
            }
            Self::GaussRelative => {
                "607d721eae7f6f9a7082150d9a476c59163bb11276f1c99f6e3fefa027a13ab5"
            }
        }
    }

    fn generated_source_bytes(self) -> usize {
        match self {
            Self::AgaussAbsolute => 377_897,
            Self::GaussRelative => 377_926,
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
    owner_path: PathBuf,
    reference_path: PathBuf,
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
    owner_path: PathBuf,
    reference_path: PathBuf,
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
    owner_path: PathBuf,
    reference_path: PathBuf,
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
    owner_path: PathBuf,
    reference_path: PathBuf,
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
    owner_path: PathBuf,
    reference_path: PathBuf,
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

mod analysis_support;
mod comparison;
mod contract_fs;
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
