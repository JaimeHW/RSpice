//! Capture and verify numerical fingerprints of the generated Verilog-A built-ins.
//!
//! `capture` writes one fixture per model; `verify` re-evaluates and compares
//! against them; `audit` runs the independent derivative oracle and reports what
//! it could and could not confirm.
//!
//! The three exist as separate verbs because they answer different questions and
//! are trusted differently. `verify` says the backend still computes what it
//! used to — necessary during a rewrite, but it can only ever confirm that a
//! wrong answer stayed wrong. `audit` says the derivative is actually the
//! derivative of the current, which is the property the goldens themselves were
//! never able to establish.

use std::collections::BTreeSet;
use std::fs::{self, File};
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Args, Parser};
use rspice_conformance::suites::veriloga::fixture::{GoldenCase, GoldenFixture, GoldenPoint};
use rspice_conformance::suites::veriloga::golden::GoldenHarness;
use rspice_core::device::veriloga_builtins::builtins;
use serde::Serialize;

use tempfile::{Builder as TempBuilder, NamedTempFile};

#[derive(Debug)]
struct GoldenError {
    message: String,
}

impl std::fmt::Display for GoldenError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for GoldenError {}

/// Bias points captured per model per case.
const DEFAULT_PROBE_POINTS: usize = 6;
const MAX_FIXTURE_BYTES: u64 = 64 * 1024 * 1024;

/// Tolerances a replay must meet, matching the backend-rewrite gates.
const PRIMAL_TOLERANCE: f64 = 1.0e-12;
const JACOBIAN_TOLERANCE: f64 = 1.0e-9;
const NOISE_TOLERANCE: f64 = 1.0e-10;

/// Relative agreement demanded of the derivative oracle where the numerical
/// difference converged.
const AUDIT_BASE_TOLERANCE: f64 = 1.0e-5;

/// A fixture that records the capturing backend's defect rather than the model.
///
/// Listing one is a claim that the *replay* is right and the *baseline* is
/// wrong, which is the opposite of what this command is for, so each entry has
/// to say how that was established by something other than the replay itself.
/// The alternative — re-capturing the fixture — would make `verify` compare a
/// backend against itself for that model, and silently retire the only
/// cross-backend check there is.
///
/// Checked in both directions. A model that starts deviating is a failure, and
/// so is a listed model that stops: an allowlist nobody prunes stops describing
/// anything.
struct FixtureDeviation {
    model: &'static str,
    /// Relative Jacobian disagreement to tolerate, rounded up from the measured
    /// value.
    jacobian: f64,
    /// The same for the reactive block, which is a separate quantity with its
    /// own floor and must not be excused by a conductance allowance.
    capacitance: f64,
    why: &'static str,
}

const FIXTURE_DEVIATIONS: &[FixtureDeviation] = &[];

fn fixture_deviation(model: &str) -> Option<&'static FixtureDeviation> {
    FIXTURE_DEVIATIONS
        .iter()
        .find(|deviation| deviation.model == model)
}

/// Models whose default card is known to produce non-finite values at exact
/// equilibrium.
///
/// A matching `NaN` is not ordinary numerical agreement and is never accepted
/// by [`relative`]. These entries make the exception explicit and fail closed:
/// finite/non-finite changes still fail, and an entry whose fixture no longer
/// contains a matching non-finite value must be removed.
struct NonFiniteFixtureDeviation {
    model: &'static str,
    why: &'static str,
}

const NON_FINITE_FIXTURE_DEVIATIONS: &[NonFiniteFixtureDeviation] = &[
    NonFiniteFixtureDeviation {
        model: "asmesd",
        why: "the independently checked default-card equilibrium stamp is non-finite",
    },
    NonFiniteFixtureDeviation {
        model: "asmesd_dio",
        why: "the independently checked default-card equilibrium stamp is non-finite",
    },
    NonFiniteFixtureDeviation {
        model: "bsimimg",
        why: "the independently checked default-card equilibrium stamp is non-finite",
    },
];

fn non_finite_fixture_deviation(model: &str) -> Option<&'static NonFiniteFixtureDeviation> {
    NON_FINITE_FIXTURE_DEVIATIONS
        .iter()
        .find(|deviation| deviation.model == model)
}

#[derive(Parser, Debug)]
#[command(
    name = "rspice-veriloga-golden",
    version,
    about = "Capture, verify, and independently audit generated Verilog-A numerics"
)]
struct Cli {
    #[command(subcommand)]
    command: GoldenCommand,
}

#[derive(clap::Subcommand, Debug)]
enum GoldenCommand {
    /// Write a fingerprint fixture per model.
    Capture(CaptureArgs),
    /// Re-evaluate every model and compare against the checked-in fixtures.
    Verify(VerifyArgs),
    /// Check stamped Jacobians against a numerical derivative of the stamped currents.
    Audit(AuditArgs),
}

#[derive(Args, Debug)]
struct CaptureArgs {
    /// New directory to publish transactionally. It must not already exist.
    #[arg(long)]
    out: PathBuf,
    /// Restrict to these models; empty means every compiled-in built-in.
    #[arg(long, value_delimiter = ',')]
    models: Vec<String>,
    /// Bias points per case.
    #[arg(long, default_value_t = DEFAULT_PROBE_POINTS, value_parser = parse_positive_usize)]
    points: usize,
}

#[derive(Args, Debug)]
struct VerifyArgs {
    /// Directory holding the fixtures.
    #[arg(long)]
    fixtures: Option<PathBuf>,
    /// Restrict to these models; empty means every compiled-in model.
    #[arg(long, value_delimiter = ',')]
    models: Vec<String>,
    /// Require bit-identical replay instead of the gate tolerances.
    #[arg(long)]
    exact: bool,
}

#[derive(Args, Debug)]
struct AuditArgs {
    /// Restrict to these models; empty means every compiled-in built-in.
    #[arg(long, value_delimiter = ',')]
    models: Vec<String>,
    /// Bias points per model.
    #[arg(long, default_value_t = DEFAULT_PROBE_POINTS, value_parser = parse_positive_usize)]
    points: usize,
    /// Write a JSON report here.
    #[arg(long)]
    out: Option<PathBuf>,
}

fn main() -> ExitCode {
    match run(&Cli::parse()) {
        Ok(code) => code,
        Err(error) => {
            eprintln!("rspice-veriloga-golden: error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn parse_positive_usize(value: &str) -> Result<usize, String> {
    let parsed = value
        .parse::<usize>()
        .map_err(|error| format!("'{value}' is not a positive integer: {error}"))?;
    if parsed == 0 {
        return Err("value must be at least 1".to_string());
    }
    Ok(parsed)
}

fn run(args: &Cli) -> Result<ExitCode, GoldenError> {
    match &args.command {
        GoldenCommand::Capture(capture) => run_capture(capture),
        GoldenCommand::Verify(verify) => run_verify(verify),
        GoldenCommand::Audit(audit) => run_audit(audit),
    }
}

fn selected_models(requested: &[String]) -> Result<Vec<&'static str>, GoldenError> {
    let available = builtins::builtin_names();
    if requested.is_empty() {
        if available.is_empty() {
            return Err(GoldenError {
                message: "no generated models are compiled in; build with the conformance crate's `veriloga-builtins-models` feature".to_string(),
            });
        }
        return Ok(available.to_vec());
    }
    let mut selected = Vec::with_capacity(requested.len());
    let mut unique = BTreeSet::new();
    for name in requested {
        let found = available
            .iter()
            .find(|candidate| candidate.eq_ignore_ascii_case(name))
            .copied()
            .ok_or_else(|| GoldenError {
                message: format!("'{name}' is not a compiled-in generated built-in"),
            })?;
        if !unique.insert(found.to_ascii_lowercase()) {
            return Err(GoldenError {
                message: format!("model '{name}' was selected more than once"),
            });
        }
        selected.push(found);
    }
    Ok(selected)
}

/// Fixture file name for a model.
///
/// Model names come from the registry and are already identifier-shaped, but
/// they are folded to lower case so a case-insensitive file system cannot make
/// two models collide silently.
fn fixture_path(root: &Path, model_name: &str) -> PathBuf {
    root.join(format!("{}.txt", model_name.to_ascii_lowercase()))
}

fn build_fixture(model_name: &'static str, points: usize) -> Result<GoldenFixture, GoldenError> {
    let mut harness = GoldenHarness::new(model_name, &[]).map_err(|error| GoldenError {
        message: error.to_string(),
    })?;

    let mut captured = Vec::new();
    for point in harness.probe_points(points) {
        let record = harness.evaluate(&point).map_err(|error| GoldenError {
            message: error.to_string(),
        })?;
        captured.push(GoldenPoint {
            unknowns: point,
            record,
        });
    }

    Ok(GoldenFixture {
        model_name: model_name.to_string(),
        node_count: harness.node_count(),
        branch_count: harness.branch_count(),
        cases: vec![GoldenCase {
            options: Vec::new(),
            points: captured,
        }],
    })
}

fn fixture_root(requested: Option<&Path>) -> PathBuf {
    requested.map_or_else(
        || Path::new(env!("CARGO_MANIFEST_DIR")).join("testdata/veriloga-golden"),
        Path::to_path_buf,
    )
}

fn run_capture(args: &CaptureArgs) -> Result<ExitCode, GoldenError> {
    let models = selected_models(&args.models)?;
    let out = &args.out;
    if out.exists() {
        return Err(GoldenError {
            message: format!(
                "{} already exists; capture requires a new directory so fixture updates can be reviewed as a complete set",
                out.display()
            ),
        });
    }

    let mut captured = Vec::with_capacity(models.len());
    let mut failed = Vec::new();
    for model_name in models {
        match build_fixture(model_name, args.points) {
            Ok(fixture) => captured.push((model_name, fixture.render())),
            Err(error) => failed.push(format!("{model_name}: {error}")),
        }
    }
    for failure in &failed {
        eprintln!("FAIL {failure}");
    }
    if !failed.is_empty() {
        return Ok(ExitCode::FAILURE);
    }

    let parent = out
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent).map_err(|error| GoldenError {
        message: format!("creating {}: {error}", parent.display()),
    })?;
    let staging = TempBuilder::new()
        .prefix("rspice-veriloga-golden-")
        .tempdir_in(parent)
        .map_err(|error| GoldenError {
            message: format!(
                "creating capture staging directory in {}: {error}",
                parent.display()
            ),
        })?;
    for (model_name, rendered) in &captured {
        let path = fixture_path(staging.path(), model_name);
        let mut file = File::create(&path).map_err(|error| GoldenError {
            message: format!("creating staged fixture {}: {error}", path.display()),
        })?;
        file.write_all(rendered.as_bytes())
            .and_then(|()| file.flush())
            .and_then(|()| file.sync_all())
            .map_err(|error| GoldenError {
                message: format!("flushing staged fixture {}: {error}", path.display()),
            })?;
    }
    let staging_path = staging.keep();
    fs::rename(&staging_path, out).map_err(|error| GoldenError {
        message: format!(
            "publishing complete fixture set {} -> {}: {error}; staged files remain for recovery",
            staging_path.display(),
            out.display()
        ),
    })?;
    println!(
        "captured {} fixtures into {}",
        captured.len(),
        out.display()
    );
    Ok(ExitCode::SUCCESS)
}

/// The worst disagreement seen in one array, and which entry it was.
///
/// A bare maximum is not a diagnosis. "primal deviation 7.7e-10" says a number
/// moved and nothing about whether it moved somewhere a solver looks, which is
/// the only question worth asking of it — so the entry, both values, and the
/// bias point that produced them travel with the number.
#[derive(Debug, Default, Clone)]
struct Worst {
    relative: f64,
    witness: String,
}

impl Worst {
    fn observe(&mut self, label: &str, array: &str, index: usize, expected: f64, actual: f64) {
        let relative = relative(expected, actual);
        if relative <= self.relative {
            return;
        }
        self.relative = relative;
        self.witness = format!("{label}: {array}[{index}] {expected:e} -> {actual:e}");
    }
}

#[derive(Debug, Default)]
struct Deviation {
    primal: Worst,
    jacobian: Worst,
    /// Kept apart from `jacobian`, which it used to be folded into.
    ///
    /// A conductance and a capacitance are different physical quantities that
    /// happen to be stamped into matrices of the same shape, and merging their
    /// maxima meant a capacitance disagreement was reported as "jacobian
    /// deviation". That is not a cosmetic mislabel: it sent a diagnosis looking
    /// for a conductance defect that was not there, and it hid the fact that the
    /// two already have separate absolute floors here because their units
    /// differ.
    capacitance: Worst,
    noise: Worst,
    structural: Vec<String>,
    matching_known_non_finite: usize,
}

fn relative(left: f64, right: f64) -> f64 {
    if left == right {
        return 0.0;
    }
    if !left.is_finite() || !right.is_finite() {
        return f64::INFINITY;
    }
    let scale = left.abs().max(right.abs());
    if scale == 0.0 {
        0.0
    } else {
        (left - right).abs() / scale
    }
}

fn matching_non_finite(left: f64, right: f64) -> bool {
    (left.is_nan() && right.is_nan())
        || (left.is_infinite() && right.is_infinite() && left == right)
}

/// Below this, an entry carries no information and comparing it measures
/// arithmetic rather than physics.
///
/// `relative` has no absolute floor by construction — zero against anything at
/// all is a full unit of disagreement — so a residual of 4.6e-27 A against an
/// exact zero reads as 100% wrong. It is not wrong; it is eight orders below
/// the smallest current a solver represents (`abstol` is 1e-12) and fifteen
/// below the entries around it. Two backends that agree everywhere a solver can
/// see and differ only here have not disagreed about anything.
///
/// The Phase 0 derivative oracle already works this way, for the same reason
/// written down in its own terms: an entry the difference learned nothing about
/// is a coverage gap rather than a failure, and a gap only matters where the
/// entry is large enough for a solver to notice. This is that rule, applied to
/// the replay.
/// Absolute floors, one per quantity, taken from the simulator's own tolerances
/// rather than chosen.
///
/// Per-array significance alone is not enough, and `hisimsotb_va` is why: its
/// residual at one bias holds exactly two entries, both about 8e-16 A, and they
/// differ by 6%. Judged against their own array they are the largest thing in
/// it and so fully significant; judged against the solver they are four orders
/// below `abstol` and cannot change any decision it makes. When a whole record
/// is noise, a relative rule declares the noise significant.
///
/// The floors differ because the quantities do. A current or a conductance
/// below `abstol`/`GMIN` — both 1e-12 — is beneath the gmin conductance every
/// node already carries in parallel. A *capacitance* of 1e-15 F is an ordinary
/// device, so charge is floored near `chgtol` instead; getting this wrong in the
/// other direction would silently retire the capacitance checks that found
/// today's defects.
///
/// The current and conductance floors sat at 1e-14 until 2026-07-28 — two orders
/// below the tolerances this comment already claimed they came from, which made
/// the prose true of the intent and false of the numbers. `hicumL2va` is what
/// exposed it: its entry is `1.0000000000000002e-12`, one ulp above the old
/// threshold and exactly the `GMIN` the model asks for and the backend correctly
/// supplies. The gate was failing a model for being right, on a knife-edge. The
/// floors now are the tolerances, so an entry the solver provably cannot act on
/// cannot fail the replay.
const CURRENT_FLOOR: f64 = 1.0e-12;
const CONDUCTANCE_FLOOR: f64 = 1.0e-12;
const CHARGE_FLOOR: f64 = 1.0e-21;

/// Share of the largest entry in the same record below which an entry carries
/// no information.
///
/// A flat absolute floor cannot serve here, and trying one is how this was got
/// wrong the first time: a residual is amps and a capacitance is farads, so the
/// same number means "cancellation noise" in one and "a real device" in the
/// other. `mvsg_cmc` disagrees on two residual entries of 1.39e-17 — which is
/// 2^-56, the residue of subtracting two numbers that nearly cancel — against a
/// row whose largest entry is 0.2. `angelov_gan` has genuine capacitances of
/// 1e-15 against a row scale of the same order. Judged against their own row,
/// the first is 7e-17 of it and the second is most of it.
const SIGNIFICANCE: f64 = 1.0e-12;

/// The largest magnitude in a record, which is what an entry is judged against.
fn record_scale(expected: &[f64], actual: &[f64]) -> f64 {
    expected
        .iter()
        .chain(actual)
        .filter(|value| value.is_finite())
        .fold(0.0f64, |scale, value| scale.max(value.abs()))
}

/// Relative slack on the floor comparison.
///
/// A floor written as the tolerance it means would exclude the very value it
/// exists to excuse. `GMIN` reaches a stamp as a *computed* quantity, not as the
/// literal: hicumL2va's conductance arrives as `1.0000000000000002e-12`, one ulp
/// above `1e-12`, so `<= 1e-12` rejects it and the gate fails a model for
/// supplying exactly the gmin it was asked for. Raising the floor to the
/// tolerance was necessary and, on its own, not sufficient.
///
/// A relative 1e-9 is four thousand ulps of room at this magnitude and still
/// 1e-21 in absolute terms, which is twenty-one orders below anything a solve
/// can act on. It buys the comparison out of exact-representation arguments
/// without widening it in any sense a simulator would notice.
const FLOOR_SLACK: f64 = 1.0 + 1.0e-9;

fn negligible(left: f64, right: f64, scale: f64, floor: f64) -> bool {
    left.abs().max(right.abs()) <= (scale * SIGNIFICANCE).max(floor) * FLOOR_SLACK
}

fn compare_point(
    label: &str,
    expected: &GoldenPoint,
    actual: &GoldenPoint,
    allow_matching_non_finite: bool,
    deviation: &mut Deviation,
) {
    if expected.unknowns.len() != actual.unknowns.len() {
        deviation.structural.push(format!(
            "{label}: probe width {} -> {}",
            expected.unknowns.len(),
            actual.unknowns.len()
        ));
    }
    for (index, (want, got)) in expected
        .unknowns
        .iter()
        .zip(actual.unknowns.iter())
        .enumerate()
    {
        if want.to_bits() != got.to_bits() {
            deviation.structural.push(format!(
                "{label}: probe[{index}] changed from {want:e} to {got:e}"
            ));
        }
    }
    for (array, expected_len, actual_len) in [
        ("rhs", expected.record.rhs.len(), actual.record.rhs.len()),
        (
            "jacobian",
            expected.record.jacobian.len(),
            actual.record.jacobian.len(),
        ),
        (
            "capacitance",
            expected.record.capacitance.len(),
            actual.record.capacitance.len(),
        ),
    ] {
        if expected_len != actual_len {
            deviation.structural.push(format!(
                "{label}: {array} length {expected_len} -> {actual_len}"
            ));
        }
    }
    let rhs_scale = record_scale(&expected.record.rhs, &actual.record.rhs);
    for (index, (want, got)) in expected
        .record
        .rhs
        .iter()
        .zip(actual.record.rhs.iter())
        .enumerate()
    {
        if allow_matching_non_finite && matching_non_finite(*want, *got) {
            deviation.matching_known_non_finite += 1;
            continue;
        }
        // A value that used to be finite and now is not never shows up as a
        // large relative error, so it is checked separately rather than being
        // folded into the maximum.
        if want.is_finite() != got.is_finite() {
            deviation
                .structural
                .push(format!("{label}: rhs[{index}] finiteness changed"));
        }
        if negligible(*want, *got, rhs_scale, CURRENT_FLOOR) {
            continue;
        }
        deviation.primal.observe(label, "rhs", index, *want, *got);
    }

    let jacobian_scale = record_scale(&expected.record.jacobian, &actual.record.jacobian);
    for (index, (want, got)) in expected
        .record
        .jacobian
        .iter()
        .zip(actual.record.jacobian.iter())
        .enumerate()
    {
        if allow_matching_non_finite && matching_non_finite(*want, *got) {
            deviation.matching_known_non_finite += 1;
            continue;
        }
        if negligible(*want, *got, jacobian_scale, CONDUCTANCE_FLOOR) {
            continue;
        }
        if (*want == 0.0) != (*got == 0.0) {
            deviation
                .structural
                .push(format!("{label}: jacobian[{index}] appeared or vanished"));
        }
        deviation
            .jacobian
            .observe(label, "jacobian", index, *want, *got);
    }

    let capacitance_scale = record_scale(&expected.record.capacitance, &actual.record.capacitance);
    for (index, (want, got)) in expected
        .record
        .capacitance
        .iter()
        .zip(actual.record.capacitance.iter())
        .enumerate()
    {
        if allow_matching_non_finite && matching_non_finite(*want, *got) {
            deviation.matching_known_non_finite += 1;
            continue;
        }
        if negligible(*want, *got, capacitance_scale, CHARGE_FLOOR) {
            continue;
        }
        if (*want == 0.0) != (*got == 0.0) {
            deviation.structural.push(format!(
                "{label}: capacitance[{index}] appeared or vanished"
            ));
        }
        deviation
            .capacitance
            .observe(label, "capacitance", index, *want, *got);
    }

    if expected.record.noise.len() != actual.record.noise.len() {
        deviation.structural.push(format!(
            "{label}: noise source count {} -> {}",
            expected.record.noise.len(),
            actual.record.noise.len()
        ));
    }
    for (index, (want, got)) in expected
        .record
        .noise
        .iter()
        .zip(actual.record.noise.iter())
        .enumerate()
    {
        if want.mechanism != got.mechanism {
            deviation.structural.push(format!(
                "{label}: noise mechanism '{}' -> '{}'",
                want.mechanism, got.mechanism
            ));
        }
        if want.active != got.active {
            deviation.structural.push(format!(
                "{label}: noise '{}' active {} -> {}",
                want.mechanism, want.active, got.active
            ));
        }
        deviation
            .noise
            .observe(label, "noise psd", index, want.psd, got.psd);
    }
}

fn run_verify(args: &VerifyArgs) -> Result<ExitCode, GoldenError> {
    let models = selected_models(&args.models)?;
    let fixtures = fixture_root(args.fixtures.as_deref());
    let mut failures = if args.models.is_empty() {
        fixture_inventory_failures(&fixtures, &models)?
    } else {
        Vec::new()
    };
    let mut verified = 0usize;

    for model_name in models {
        let path = fixture_path(&fixtures, model_name);
        let text = match read_fixture(&path) {
            Ok(text) => text,
            Err(error) => {
                failures.push(format!("{model_name}: {error}"));
                continue;
            }
        };
        let expected = match GoldenFixture::parse(&text) {
            Ok(fixture) => fixture,
            Err(error) => {
                failures.push(format!("{model_name}: {}: {error}", path.display()));
                continue;
            }
        };

        let actual = match build_fixture(model_name, expected_point_count(&expected)) {
            Ok(fixture) => fixture,
            Err(error) => {
                failures.push(format!("{model_name}: {error}"));
                continue;
            }
        };

        if !expected.model_name.eq_ignore_ascii_case(model_name) {
            failures.push(format!(
                "{model_name}: fixture declares model '{}'",
                expected.model_name
            ));
            continue;
        }

        if expected.node_count != actual.node_count || expected.branch_count != actual.branch_count
        {
            failures.push(format!(
                "{model_name}: topology changed: {}+{} unknowns captured, {}+{} now",
                expected.node_count, expected.branch_count, actual.node_count, actual.branch_count
            ));
            continue;
        }

        let mut deviation = Deviation::default();
        let known_non_finite = non_finite_fixture_deviation(model_name);
        if expected.cases.len() != actual.cases.len() {
            deviation.structural.push(format!(
                "case count {} -> {}",
                expected.cases.len(),
                actual.cases.len()
            ));
        }
        for (case_index, (want_case, got_case)) in
            expected.cases.iter().zip(actual.cases.iter()).enumerate()
        {
            if want_case.options != got_case.options {
                deviation.structural.push(format!(
                    "case {case_index}: options changed from {:?} to {:?}",
                    want_case.options, got_case.options
                ));
            }
            if want_case.points.len() != got_case.points.len() {
                deviation.structural.push(format!(
                    "case {case_index}: point count {} -> {}",
                    want_case.points.len(),
                    got_case.points.len()
                ));
            }
            for (point_index, (want, got)) in want_case
                .points
                .iter()
                .zip(got_case.points.iter())
                .enumerate()
            {
                compare_point(
                    &format!("case {case_index} point {point_index}"),
                    want,
                    got,
                    known_non_finite.is_some(),
                    &mut deviation,
                );
            }
        }

        let known = fixture_deviation(model_name);
        let (primal_limit, jacobian_limit, capacitance_limit, noise_limit) = if args.exact {
            (0.0, 0.0, 0.0, 0.0)
        } else {
            (
                PRIMAL_TOLERANCE,
                known.map_or(JACOBIAN_TOLERANCE, |deviation| deviation.jacobian),
                known.map_or(JACOBIAN_TOLERANCE, |deviation| deviation.capacitance),
                NOISE_TOLERANCE,
            )
        };

        let mut model_failures = deviation.structural.clone();
        if let Some(known) = known_non_finite
            && deviation.matching_known_non_finite == 0
        {
            model_failures.push(format!(
                "listed as a non-finite fixture deviation but no matching non-finite value remains; drop the entry ({})",
                known.why
            ));
        }
        // The other direction: a fixture that stops recording its defect means
        // it has been re-captured, or the backend has changed under it, and
        // either way the entry is now describing nothing.
        if let Some(known) = known
            && !args.exact
            && deviation.jacobian.relative <= JACOBIAN_TOLERANCE
            && deviation.capacitance.relative <= JACOBIAN_TOLERANCE
        {
            model_failures.push(format!(
                "listed as a fixture deviation but now agrees to {:.0e}; drop the entry ({})",
                JACOBIAN_TOLERANCE, known.why
            ));
        }
        for (quantity, worst, limit) in [
            ("primal", &deviation.primal, primal_limit),
            ("jacobian", &deviation.jacobian, jacobian_limit),
            ("capacitance", &deviation.capacitance, capacitance_limit),
            ("noise", &deviation.noise, noise_limit),
        ] {
            if worst.relative > limit {
                model_failures.push(format!(
                    "{quantity} deviation {:.3e} exceeds {limit:.0e} at {}",
                    worst.relative, worst.witness
                ));
            }
        }

        if model_failures.is_empty() {
            verified += 1;
        } else {
            for failure in model_failures.iter().take(5) {
                failures.push(format!("{model_name}: {failure}"));
            }
        }
    }

    println!("verified {verified} models against {}", fixtures.display());
    for failure in &failures {
        eprintln!("FAIL {failure}");
    }
    Ok(if failures.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}

fn read_fixture(path: &Path) -> Result<String, String> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|error| format!("inspecting {}: {error}", path.display()))?;
    if !metadata.file_type().is_file() {
        return Err(format!(
            "{} must be a regular, non-symlink fixture file",
            path.display()
        ));
    }
    if metadata.len() == 0 || metadata.len() > MAX_FIXTURE_BYTES {
        return Err(format!(
            "{} must contain 1..={} bytes",
            path.display(),
            MAX_FIXTURE_BYTES
        ));
    }
    let bytes = fs::read(path).map_err(|error| format!("reading {}: {error}", path.display()))?;
    String::from_utf8(bytes).map_err(|error| format!("{} is not UTF-8: {error}", path.display()))
}

fn fixture_inventory_failures(
    fixtures: &Path,
    models: &[&'static str],
) -> Result<Vec<String>, GoldenError> {
    let expected = models
        .iter()
        .map(|model| format!("{}.txt", model.to_ascii_lowercase()))
        .collect::<BTreeSet<_>>();
    let entries = fs::read_dir(fixtures).map_err(|error| GoldenError {
        message: format!("reading fixture directory {}: {error}", fixtures.display()),
    })?;
    let mut actual = BTreeSet::new();
    let mut failures = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|error| GoldenError {
            message: format!("reading fixture directory {}: {error}", fixtures.display()),
        })?;
        let file_type = entry.file_type().map_err(|error| GoldenError {
            message: format!("inspecting fixture {}: {error}", entry.path().display()),
        })?;
        let name = entry.file_name().to_string_lossy().to_string();
        if !file_type.is_file() || !name.to_ascii_lowercase().ends_with(".txt") {
            failures.push(format!(
                "unexpected non-fixture entry in {}: {name}",
                fixtures.display()
            ));
            continue;
        }
        if !actual.insert(name.to_ascii_lowercase()) {
            failures.push(format!("duplicate case-insensitive fixture name: {name}"));
        }
    }
    for missing in expected.difference(&actual) {
        failures.push(format!("missing fixture: {missing}"));
    }
    for extra in actual.difference(&expected) {
        failures.push(format!("fixture has no compiled-in model: {extra}"));
    }
    Ok(failures)
}

fn expected_point_count(fixture: &GoldenFixture) -> usize {
    fixture
        .cases
        .first()
        .map_or(DEFAULT_PROBE_POINTS, |case| case.points.len())
}

#[derive(Serialize)]
struct AuditReport {
    models: Vec<AuditModelReport>,
}

#[derive(Serialize)]
struct AuditModelReport {
    model_name: &'static str,
    entries: usize,
    tightly_verified_fraction: f64,
    unverified_significant: usize,
    worst_relative_error: f64,
}

fn run_audit(args: &AuditArgs) -> Result<ExitCode, GoldenError> {
    let models = selected_models(&args.models)?;
    let mut reports = Vec::new();
    let mut failures = Vec::new();

    println!(
        "{:<26} {:>8} {:>10} {:>12} {:>12}",
        "model", "entries", "verified", "gaps", "worst"
    );

    for model_name in models {
        let mut harness = GoldenHarness::new(model_name, &[]).map_err(|error| GoldenError {
            message: error.to_string(),
        })?;

        let mut entries = 0usize;
        let mut tight = 0usize;
        let mut gaps = 0usize;
        let mut worst = 0.0_f64;

        for (index, point) in harness.probe_points(args.points).into_iter().enumerate() {
            let audit = match harness.jacobian_audit(&point) {
                Ok(audit) => audit,
                Err(error) => {
                    failures.push(format!("{model_name}: point {index}: {error}"));
                    continue;
                }
            };
            entries += audit.entries.len();
            tight += audit.checked().count();
            gaps += audit.unverified_significant().len();
            worst = worst.max(audit.worst_relative_error());

            for failure in audit.failures(AUDIT_BASE_TOLERANCE).into_iter().take(3) {
                failures.push(format!(
                    "{model_name}: point {index}: d(row {})/d(col {}) stamped {:e}, numeric {:e}, relative error {:.3e}",
                    failure.row, failure.col, failure.stamped, failure.numeric, failure.relative_error
                ));
            }
        }

        let fraction = if entries == 0 {
            1.0
        } else {
            tight as f64 / entries as f64
        };
        println!(
            "{model_name:<26} {entries:>8} {:>9.1}% {gaps:>12} {worst:>12.2e}",
            fraction * 100.0
        );
        reports.push(AuditModelReport {
            model_name,
            entries,
            tightly_verified_fraction: fraction,
            unverified_significant: gaps,
            worst_relative_error: worst,
        });
    }

    if let Some(path) = &args.out {
        let json =
            serde_json::to_string_pretty(&AuditReport { models: reports }).map_err(|error| {
                GoldenError {
                    message: format!("serializing audit report: {error}"),
                }
            })?;
        write_atomic(path, format!("{json}\n").as_bytes())?;
    }

    for failure in &failures {
        eprintln!("FAIL {failure}");
    }
    Ok(if failures.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}

fn write_atomic(path: &Path, bytes: &[u8]) -> Result<(), GoldenError> {
    if path.exists() {
        return Err(GoldenError {
            message: format!(
                "{} already exists; audit artifacts are immutable",
                path.display()
            ),
        });
    }
    let directory = path.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(directory).map_err(|error| GoldenError {
        message: format!("creating {}: {error}", directory.display()),
    })?;
    let mut temp = NamedTempFile::new_in(directory).map_err(|error| GoldenError {
        message: format!(
            "creating temporary output in {}: {error}",
            directory.display()
        ),
    })?;
    temp.write_all(bytes)
        .and_then(|()| temp.flush())
        .and_then(|()| temp.as_file().sync_all())
        .map_err(|error| GoldenError {
            message: format!("flushing {}: {error}", path.display()),
        })?;
    temp.persist_noclobber(path).map_err(|error| GoldenError {
        message: format!("publishing {}: {}", path.display(), error.error),
    })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relative_difference_is_symmetric_and_zero_on_equality() {
        assert_eq!(relative(1.0, 1.0), 0.0);
        assert_eq!(relative(0.0, 0.0), 0.0);
        assert_eq!(relative(1.0, 2.0), relative(2.0, 1.0));
        assert_eq!(relative(1.0, 2.0), 0.5);
    }

    #[test]
    fn identical_infinities_do_not_report_a_deviation() {
        // `inf - inf` is NaN, which would compare false against every
        // tolerance and silently pass. Equality short-circuits it instead.
        assert_eq!(relative(f64::INFINITY, f64::INFINITY), 0.0);
    }

    #[test]
    fn unequal_non_finite_values_are_never_silently_accepted() {
        assert_eq!(relative(f64::NAN, f64::NAN), f64::INFINITY);
        assert_eq!(relative(f64::INFINITY, f64::NEG_INFINITY), f64::INFINITY);
        assert_eq!(relative(1.0, f64::NAN), f64::INFINITY);
    }

    #[test]
    fn known_non_finite_replay_requires_the_same_non_finite_class() {
        assert!(matching_non_finite(f64::NAN, f64::NAN));
        assert!(matching_non_finite(f64::INFINITY, f64::INFINITY));
        assert!(matching_non_finite(f64::NEG_INFINITY, f64::NEG_INFINITY));
        assert!(!matching_non_finite(f64::INFINITY, f64::NEG_INFINITY));
        assert!(!matching_non_finite(f64::NAN, 0.0));
        assert!(!matching_non_finite(0.0, 0.0));
    }

    #[test]
    fn fixture_paths_are_case_folded() {
        let root = Path::new("fixtures");
        assert_eq!(
            fixture_path(root, "PSP104VA"),
            fixture_path(root, "psp104va")
        );
    }

    #[test]
    fn cli_rejects_zero_probe_points_for_capture_and_audit() {
        for command in ["capture", "audit"] {
            let mut arguments = vec!["rspice-veriloga-golden", command];
            if command == "capture" {
                arguments.extend(["--out", "unused"]);
            }
            arguments.extend(["--points", "0"]);
            let error = Cli::try_parse_from(arguments).expect_err("zero points must fail");
            assert!(error.to_string().contains("value must be at least 1"));
        }
    }

    #[test]
    fn cli_accepts_one_probe_point() {
        let parsed = Cli::try_parse_from(["rspice-veriloga-golden", "audit", "--points", "1"])
            .expect("one point is valid");
        let GoldenCommand::Audit(arguments) = parsed.command else {
            panic!("audit command expected");
        };
        assert_eq!(arguments.points, 1);
    }
}
