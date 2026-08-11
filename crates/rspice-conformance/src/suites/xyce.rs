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
    StepPlanLimits, TransientResult, XyceTraInterpolation, extract_ac_value, extract_dc_value,
};
use rspice_core::expr::{
    BinaryOp, CompiledExpr, Context, Expr, Vm, compile, parse_expression_strict,
};
use rspice_core::netlist::expr::ComplexValue as ExprComplexValue;
use rspice_core::netlist::expr::{
    behavioral_expression_references_unbound_frequency, prepare_behavioral_expression,
};
use rspice_core::netlist::{
    AnalysisCommand, DcSecondSweep, DcSweepMode, DeviceInitialConditionError,
    DeviceInitialConditionSource, DuplicateSubcircuitPortBindingError, ElementKind,
    ElementProvenance, MissingSubcircuitEndsBoundary, MissingSubcircuitEndsError, Netlist,
    NetlistParseOptions, OutputDirectiveKind, OutputSymbolKind, ParameterRedefinitionPolicy,
    ParametricValue, ParseError, StartupDiagnosticCode, StartupDiagnosticStage,
    StartupDirectiveKind, StartupDirectiveScope, StatisticalParamMode, StepCommand, StepSweep,
    StepTarget, SubcircuitDef, XYCE_DEFAULT_ZERO_RESISTANCE_TOL, flatten_netlist,
    flatten_netlist_with_models, validate_output_symbols,
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
const UPSTREAM_EXCLUSIONS_QUALIFIED_DECK_COUNT: usize = 191;
const UPSTREAM_EXCLUSIONS_RETAINED_PATHS_SHA256: &str =
    "eb3eb203f0974a430cdea3924e921aecdc1f71c5c9ce4de2f78f282c57291997";
const UPSTREAM_EXCLUSIONS_PROMOTIONS_SHA256: &str =
    "4462b3b1fdcff3131d162f40e36969b2c731089558f44f54a93918e94a6d85b1";
const UPSTREAM_EXCLUSIONS_RECORDS_SHA256: &str =
    "5620d0b15c0f99671b8f7dcc6f24d40d63a258ea665b46aa3c92f0e7a4e3a39d";
const UPSTREAM_EXCLUSIONS_MANIFEST_SHA256: &str =
    "c9b146c488397073cd4038df4f6f3fa7d22dd71222755b72a9795414ae8239c3";
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
        (self == Self::IcNodeSetConflict).then_some(XyceUpstreamExpectedErrorPolicy {
            requires_nonzero_exit: true,
            search_streams: XyceUpstreamErrorSearchStreams::EitherCompleteStdoutOrStderr,
            ordered_patterns: &["Cannot set both .IC and .NODESET simultaneously"],
        })
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
        XyceUpstreamExpectedErrorPolicy {
            requires_nonzero_exit: true,
            search_streams: XyceUpstreamErrorSearchStreams::EitherCompleteStdoutOrStderr,
            ordered_patterns,
        }
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
struct XyceUpstreamExpectedErrorPolicy {
    requires_nonzero_exit: bool,
    search_streams: XyceUpstreamErrorSearchStreams,
    ordered_patterns: &'static [&'static str],
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
    print: XycePrintRequest,
    print_format: Option<String>,
    dc: XyceDcSweep,
    dc_data: Option<XyceDcDataSweep>,
    steps: Vec<StepCommand>,
    diagnostics: Vec<rspice_core::netlist::ParseDiagnostic>,
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
                        XyceStaticTranPlanPurpose::RelationalFamily
                            | XyceStaticTranPlanPurpose::AgeCapRelationalFamily
                            | XyceStaticTranPlanPurpose::ScopedModelRelationalFamily,
                        false,
                        XyceStaticTranContract::PlainStatic
                            | XyceStaticTranContract::PlainCsv
                            | XyceStaticTranContract::PlainCsd
                    ) | (
                        XyceStaticTranPlanPurpose::RelationalFamily
                            | XyceStaticTranPlanPurpose::AgeCapRelationalFamily
                            | XyceStaticTranPlanPurpose::ScopedModelRelationalFamily,
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
enum XyceAnalyticIntegerDcKind {
    Fmod,
    IntFloorCeilBehavioralSources,
}

impl XyceAnalyticIntegerDcKind {
    fn result_contract(self) -> &'static str {
        match self {
            Self::Fmod => "analytic_fmod_dc_wrapper",
            Self::IntFloorCeilBehavioralSources => "analytic_int_floor_ceil_bsource_dc_wrapper",
        }
    }
}

#[derive(Debug, Clone)]
struct XyceAnalyticIntegerDcContract {
    plan: XyceStaticDcPlan,
    kind: XyceAnalyticIntegerDcKind,
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
    /// Compare scoped-model and explicitly expanded representations under an
    /// exact qualified-topology, model-parameter, and waveform-parity
    /// contract. This purpose has a separately qualified native BJT envelope
    /// so ordinary relational families cannot gain BJT eligibility by
    /// association.
    ScopedModelRelationalFamily,
    /// Execute a wrapper-origin transient deck whose oracle is generated
    /// analytically on the simulator's own default-PRN time grid. The
    /// dedicated analytic contract supplies the missing reference and proves
    /// the exact bounded circuit/source/options envelope separately.
    AnalyticOracle,
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
                | Self::PassiveTemperatureAnalyticOracle
        )
    }

    fn admits_default_level9_bsim3(self) -> bool {
        matches!(self, Self::DefaultLevel9XyceVerifyOracle)
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
    const BUG1190_MUTUAL_INDUCTOR: Self = Self {
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
    PassivePrimaryValue(XycePassivePrimaryValueSnapshot),
    PassiveTemperatureOverride(XycePassiveTemperatureOverrideSnapshot),
    TransientAnalysisExpression(XyceTransientAnalysisExpressionSnapshot),
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
    BjtExternalNode(XyceBjtExternalNodeFamilySnapshot),
    DcAnalysisExpression(XyceDcAnalysisExpressionSnapshot),
    DelimitedExpression(XyceDelimitedExpressionFamilySnapshot),
    PassivePrimaryValue(XycePassivePrimaryValueSnapshot),
    SubcktParameterPrecedence(XyceSubcktParameterPrecedenceSnapshot),
    SubcktParameterResolution(XyceSubcktParameterResolutionSnapshot),
    NestedIncludeIdentity(XyceNestedIncludeIdentityFamilySnapshot),
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

#[derive(Debug, Clone, PartialEq, Eq)]
enum XyceStrictAcFamilySnapshot {
    AcAnalysisExpression(XyceAcAnalysisExpressionSnapshot),
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceBaselineFamilyAnalysis {
    Ac,
    Dc,
    Tran,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceBaselineFamilyComparison {
    AcComparator(XyceAcComparatorTolerance),
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

    fn permits_locked_time_retry(self) -> bool {
        matches!(self, Self::Toleranced)
    }
}

impl XyceBaselineFamilyKind {
    fn name(self) -> &'static str {
        match self {
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
        }
    }

    fn wrapper_contract(self) -> &'static str {
        match self {
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
        }
    }

    fn baseline_contract(self) -> &'static str {
        match self {
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
        matches!(self, Self::NakedAlgebra)
    }

    fn transient_plan_purpose(self) -> XyceStaticTranPlanPurpose {
        match self {
            Self::ScopedModel => XyceStaticTranPlanPurpose::ScopedModelRelationalFamily,
            Self::AgeCap => XyceStaticTranPlanPurpose::AgeCapRelationalFamily,
            Self::AcAnalysisExpression
            | Self::BjtExternalNode
            | Self::DcAnalysisExpression
            | Self::DelimitedExpression
            | Self::DiodeModelAlias
            | Self::SinExpression
            | Self::ParamExpression
            | Self::Params1
            | Self::NakedAlgebra
            | Self::Bug1826ThermalParameter
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

    fn op_parameter(self) -> Option<&'static str> {
        match self {
            Self::Drain => Some("id"),
            Self::Gate => Some("ig"),
            Self::Source => Some("is"),
            Self::Bulk => Some("ib"),
            Self::Collector => Some("ic"),
            Self::Emitter => Some("ie"),
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
mod contracts;
mod contracts_dc;
mod contracts_frequency;
mod contracts_sources;
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

#[cfg(test)]
mod tests;
