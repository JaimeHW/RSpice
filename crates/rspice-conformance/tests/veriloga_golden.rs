//! The generated Verilog-A built-ins, judged by something other than themselves.
//!
//! A compact model's Jacobian is produced by the same compiler pass that
//! produced its currents, so agreement between them proves nothing. These tests
//! recover each device's current vector from its own stamp
//! (`I = J*V - rhs`, in which the stamped `J` cancels) and differentiate it
//! numerically, sharing no code with the chain rule under test.
//!
//! Entries where the numerical derivative has not itself converged are held to
//! their own error bar rather than to a fixed tolerance — junction exponentials
//! have derivatives large enough that no step size is free of both truncation
//! and round-off, and demanding better agreement than the difference achieved
//! would manufacture failures out of arithmetic. Entries the difference learned
//! nothing about are reported as coverage gaps instead, and a gap only counts
//! where the stamped entry is large enough for a solver to notice.
//!
//! The reactive block gets the same treatment through a different recovery: a
//! `ddt` is driven with a bare companion weight so the operator returns
//! `scale * q`, which makes the stored charge fall out of the difference between
//! two residuals and leaves it differentiable without touching the reactive
//! stamp. See [`GoldenHarness::charge_audit`].
//!
//! ## What the default card does not reach
//!
//! Every model here is instantiated at its bare defaults, which is what makes
//! the numbers comparable across revisions. It also means a model whose
//! capacitance parameters default to zero stores no charge and stamps nothing
//! reactive — VBIC is the clearest case, with `cje` defaulting to `0.0`. For
//! those models the reactive gate confirms only that nothing is stamped where
//! nothing is stored, which is worth asserting and is not coverage of the
//! charge path. Reaching it needs a per-model card, which these do not carry.
//!
//! ## Known deviations
//!
//! The backend these run against is scheduled for replacement
//! (`design/VERILOGA_BACKEND_PLAN.md`), and the deviations recorded below are
//! its defects, not this gate's tolerances. They are listed rather than fixed
//! because the emitter that would have to be repaired is deleted in Phase 6;
//! the new one must clear the list without exemptions.
//!
//! The list is checked in both directions. A model that starts deviating is a
//! failure, and so is a listed model that stops — an allowlist nobody prunes
//! stops describing anything.

#![cfg(feature = "veriloga-builtins-base")]

use rspice_conformance::suites::veriloga::golden::GoldenHarness;
use rspice_core::constants::GMIN;
use rspice_core::device::veriloga_builtins::{GeneratedSimulationParameters, builtins};

/// Bias points per model. One is equilibrium; the rest are drawn from the
/// model's own deterministic stream.
const PROBE_POINTS: usize = 4;

/// Relative agreement required of an entry the difference resolved tightly.
const JACOBIAN_TOLERANCE: f64 = 1.0e-5;

/// Models whose stamp is not finite at zero bias.
///
/// A DC solve starts near equilibrium, so this is a real defect rather than an
/// artificial probe. Recorded for the rewrite to clear.
const NON_FINITE_AT_EQUILIBRIUM: &[&str] = &["asmesd", "asmesd_dio", "bsimimg"];

/// A model whose stamped Jacobian is known to disagree with the difference.
struct KnownDeviation {
    model: &'static str,
    /// Worst relative disagreement measured on the current backend, rounded up.
    relative_error: f64,
    /// Significant entries the difference could not verify at all.
    unverified_significant: usize,
    why: &'static str,
}

const KNOWN_DEVIATIONS: &[KnownDeviation] = &[
    KnownDeviation {
        model: "asmesd",
        relative_error: 2.0e-5,
        unverified_significant: 12,
        why: "was a whole missing thermal column on the old backend, at a full \
              unit of disagreement; the canonical backend stamps it and what \
              remains is ordinary difference precision",
    },
    KnownDeviation {
        model: "asmesd_dio",
        relative_error: 2.0e-5,
        unverified_significant: 8,
        why: "same missing thermal column as asmesd, and fixed with it",
    },
    KnownDeviation {
        model: "bsimimg",
        relative_error: 2.0e-4,
        unverified_significant: 5,
        why: "non-finite at equilibrium; small disagreements away from it",
    },
    KnownDeviation {
        model: "hisimsotb_va",
        relative_error: 1.0,
        unverified_significant: 0,
        why: "same zero-bias kink, with the stamped entry exactly zero",
    },
    KnownDeviation {
        model: "EPFL_HEMT_10a",
        relative_error: 1.0e-5,
        unverified_significant: 10,
        why: "equilibrium entries the difference cannot resolve",
    },
];

fn known(model: &str) -> Option<&'static KnownDeviation> {
    KNOWN_DEVIATIONS
        .iter()
        .find(|deviation| deviation.model == model)
}

/// Relative agreement required of a reactive entry the difference resolved
/// tightly.
///
/// Looser than the conduction gate by two orders, and the looseness is in the
/// measurement rather than in the stamp: a charge is recovered by subtracting
/// two current vectors, so it starts having already lost the digits that
/// cancelled, and the difference then divides by a step.
const CAPACITANCE_TOLERANCE: f64 = 1.0e-3;

/// Charge, in coulombs, below which a device counts as storing none.
///
/// Well under the smallest junction capacitance any shipped model carries at a
/// volt, and well over what the recovery's own cancellation leaves behind.
const CHARGE_SIGNIFICANCE: f64 = 1.0e-24;

/// A model whose stamped reactive block is known to disagree with the
/// difference, or whose charge the oracle cannot recover.
struct KnownCapacitanceDeviation {
    model: &'static str,
    relative_error: f64,
    unverified_significant: usize,
    why: &'static str,
}

/// Measured on the canonical backend, 2026-07-28, all 42 models.
///
/// **Coverage gaps only. There is no capacitance defect in the corpus**: every
/// model reports zero failures against the strict tolerance, so no entry here
/// relaxes it except `PSP104TVA`.
///
/// This list previously named two "defect clusters" — the `bjt505` family
/// stamping `9.294565833142348e-16` against a difference of
/// `2.096719149589923e-15`, and the `bsimsoi` family disagreeing in sign,
/// `-1.44e-15` against `+3.25e-15`. Neither was real. Both were the oracle
/// differencing across a *kink*, and both dissolved the day it learned to
/// recognise one.
///
/// Every one of those failures sat at probe point 0, which is exact
/// equilibrium, and compact models put their numerical-safety guards exactly
/// there. Mextram's epilayer is the clearest case: inside
/// `abs(Vc1c2) < 1e-5 * Vt` it swaps `xi_w = Ec / (Ec + Vc1c2)` for the limit
/// form `pav / (pav + 1)`. The two agree in value at the boundary and not in
/// slope, so the function is continuous and not differentiable, and the step
/// ladder's smallest step is a thousand times wider than the guard. The device
/// stamps the derivative of the branch it is in, which is what Newton needs; the
/// central difference straddles the boundary and converges — smoothly, with a
/// convincing error bar — on the average of two one-sided slopes that were never
/// the same number. The shortfall was equal and opposite across the two columns,
/// which is the signature of a `Vc1c2` term, and that is exactly what the guard
/// switches on.
///
/// The oracle now detects this directly: a one-sided slope gap that is large
/// against the entry and *does not shrink* when the step falls fourfold is a
/// kink, and such entries are reported as coverage gaps rather than failures.
/// Curvature shrinks with the step and a genuinely dropped term is smooth, so
/// neither can hide behind it — see `AUDIT_DISCONTINUITY_PERSISTENCE`. The gap
/// counts below rose by exactly the number of formerly-failing entries in each
/// model, which is the reclassification and not new blindness.
const KNOWN_CAPACITANCE_DEVIATIONS: &[KnownCapacitanceDeviation] = &[
    KnownCapacitanceDeviation {
        model: "bjt505_va",
        relative_error: CAPACITANCE_TOLERANCE,
        unverified_significant: 9,
        why: "coverage gaps only; four of them are the epilayer kink at \
              equilibrium, where Mextram's `abs(Vc1c2) < 1e-5 * Vt` guard leaves \
              the charge continuous but not differentiable",
    },
    KnownCapacitanceDeviation {
        model: "bjt505t_va",
        relative_error: CAPACITANCE_TOLERANCE,
        unverified_significant: 4,
        why: "the same four epilayer-kink entries; the body is shared",
    },
    KnownCapacitanceDeviation {
        model: "bjtd505_va",
        relative_error: CAPACITANCE_TOLERANCE,
        unverified_significant: 13,
        why: "coverage gaps only, four of them the shared epilayer kink",
    },
    KnownCapacitanceDeviation {
        model: "bjtd505t_va",
        relative_error: CAPACITANCE_TOLERANCE,
        unverified_significant: 15,
        why: "coverage gaps only, four of them the shared epilayer kink",
    },
    KnownCapacitanceDeviation {
        model: "bsimsoi__18c250bc",
        relative_error: CAPACITANCE_TOLERANCE,
        unverified_significant: 3,
        why: "coverage gaps only; the former sign disagreement was a kink, not \
              an absent capacitance",
    },
    KnownCapacitanceDeviation {
        model: "bsimsoi_va",
        relative_error: CAPACITANCE_TOLERANCE,
        unverified_significant: 11,
        why: "coverage gaps only; same kink as bsimsoi__18c250bc",
    },
    KnownCapacitanceDeviation {
        model: "PSP104TVA",
        relative_error: 1.0e-2,
        unverified_significant: 1,
        why: "1.6157e-19 F against 1.6314e-19 F; small, but the difference \
              claims to have resolved it an order tighter than they disagree",
    },
    KnownCapacitanceDeviation {
        model: "PSPNQS104VA",
        relative_error: CAPACITANCE_TOLERANCE,
        unverified_significant: 3,
        why: "coverage gaps only; every comparable entry agrees",
    },
    KnownCapacitanceDeviation {
        model: "asmesd",
        relative_error: CAPACITANCE_TOLERANCE,
        unverified_significant: 1,
        why: "coverage gaps only",
    },
    KnownCapacitanceDeviation {
        model: "asmesd_dio",
        relative_error: CAPACITANCE_TOLERANCE,
        unverified_significant: 1,
        why: "coverage gaps only",
    },
    KnownCapacitanceDeviation {
        model: "bsimimg",
        relative_error: CAPACITANCE_TOLERANCE,
        unverified_significant: 32,
        why: "coverage gaps only, and the most of any model; it is also the one \
              that is non-finite at equilibrium",
    },
    KnownCapacitanceDeviation {
        model: "hicumL0va",
        relative_error: CAPACITANCE_TOLERANCE,
        unverified_significant: 8,
        why: "coverage gaps only; its whole reactive block sits at 1e-20 F",
    },
    KnownCapacitanceDeviation {
        model: "hicumL2va",
        relative_error: CAPACITANCE_TOLERANCE,
        unverified_significant: 9,
        why: "coverage gaps only; its whole reactive block sits at 1e-20 F",
    },
    KnownCapacitanceDeviation {
        model: "hisimhv_va",
        relative_error: CAPACITANCE_TOLERANCE,
        unverified_significant: 3,
        why: "coverage gaps only",
    },
    KnownCapacitanceDeviation {
        model: "l_utsoi__485e0ac9",
        relative_error: CAPACITANCE_TOLERANCE,
        unverified_significant: 28,
        why: "coverage gaps only; NQS internal nodes carry 1 nF regularising \
              capacitors against femtofarad physics, which is a hard block to \
              difference",
    },
    KnownCapacitanceDeviation {
        model: "l_utsoi__832ce87d",
        relative_error: CAPACITANCE_TOLERANCE,
        unverified_significant: 3,
        why: "coverage gaps only",
    },
];

fn known_capacitance(model: &str) -> Option<&'static KnownCapacitanceDeviation> {
    KNOWN_CAPACITANCE_DEVIATIONS
        .iter()
        .find(|deviation| deviation.model == model)
}

#[test]
fn every_builtin_evaluates_finitely_away_from_its_known_defects() {
    let mut failures = Vec::new();
    let mut observed_non_finite = Vec::new();

    for model_name in builtins::builtin_names() {
        let mut harness = match GoldenHarness::new(model_name, &[]) {
            Ok(harness) => harness,
            Err(error) => {
                failures.push(format!("{model_name}: setup: {error}"));
                continue;
            }
        };

        let mut model_non_finite = false;
        for (index, point) in harness.probe_points(PROBE_POINTS).into_iter().enumerate() {
            match harness.evaluate(&point) {
                Ok(record) => {
                    let non_finite = record
                        .jacobian
                        .iter()
                        .chain(record.rhs.iter())
                        .chain(record.capacitance.iter())
                        .any(|value| !value.is_finite());
                    if non_finite {
                        model_non_finite = true;
                        if !NON_FINITE_AT_EQUILIBRIUM.contains(model_name) {
                            failures
                                .push(format!("{model_name}: point {index}: stamp is not finite"));
                        }
                    }
                }
                Err(error) => failures.push(format!("{model_name}: point {index}: {error}")),
            }
        }
        if model_non_finite {
            observed_non_finite.push(*model_name);
        }
    }

    // The allowlist must describe reality in both directions — but only about
    // models this build actually contains. Every model is independently
    // selectable, so a build carrying one device would otherwise fail for all
    // the ones it left out.
    for model_name in NON_FINITE_AT_EQUILIBRIUM {
        if !builtins::builtin_names().contains(model_name) {
            continue;
        }
        assert!(
            observed_non_finite.contains(model_name),
            "{model_name} is listed as non-finite at equilibrium but now evaluates \
             finitely; remove it from NON_FINITE_AT_EQUILIBRIUM"
        );
    }

    assert!(
        failures.is_empty(),
        "generated built-ins must evaluate finitely at their probe points:\n{}",
        failures.join("\n")
    );
}

#[test]
fn stamped_jacobians_match_a_numerical_derivative_of_the_stamped_currents() {
    let mut failures = Vec::new();
    let mut clean = Vec::new();

    for model_name in builtins::builtin_names() {
        let Ok(mut harness) = GoldenHarness::new(model_name, &[]) else {
            // Setup failures are the other test's business; reporting them
            // twice would double-count one problem.
            continue;
        };
        let allowance = known(model_name);
        let tolerance = allowance.map_or(JACOBIAN_TOLERANCE, |known| known.relative_error);
        let allowed_gaps = allowance.map_or(0, |known| known.unverified_significant);

        let mut worst = 0.0_f64;
        let mut gaps = 0usize;
        let mut entries = 0usize;
        let mut tight = 0usize;
        let mut strict = 0usize;

        for (index, point) in harness.probe_points(PROBE_POINTS).into_iter().enumerate() {
            let audit = match harness.jacobian_audit(&point) {
                Ok(audit) => audit,
                Err(error) => {
                    failures.push(format!(
                        "{model_name}: point {index}: audit failed: {error}"
                    ));
                    continue;
                }
            };
            entries += audit.entries.len();
            tight += audit.checked().count();
            gaps += audit.unverified_significant().len();
            worst = worst.max(audit.worst_relative_error());
            strict += audit.failures(JACOBIAN_TOLERANCE).len();

            for entry in audit.failures(tolerance).into_iter().take(3) {
                failures.push(format!(
                    "{model_name}: point {index}: d(row {})/d(col {}) stamped {:e}, numeric {:e}, relative error {:.3e} (tolerance {tolerance:.1e})",
                    entry.row, entry.col, entry.stamped, entry.numeric, entry.relative_error
                ));
            }
        }

        eprintln!(
            "{model_name:<24} entries {entries:>6}  verified {:>5.1}%  gaps {gaps:>4}  strict {strict:>3}  worst {worst:.2e}",
            if entries == 0 {
                100.0
            } else {
                tight as f64 * 100.0 / entries as f64
            },
        );

        if gaps > allowed_gaps {
            failures.push(format!(
                "{model_name}: {gaps} significant entries unverifiable, allowance is {allowed_gaps}"
            ));
        }
        // Judged by the same predicate the failure check uses, not by
        // `worst_relative_error`. The two disagree: `worst` looks only at
        // entries the difference resolved tightly, so an entry that missed
        // convergence by a hair can fail the gate while leaving `worst` clean,
        // and a model can then be dropped from the list and immediately fail.
        if let Some(allowance) = allowance
            && gaps == 0
            && strict == 0
        {
            clean.push((*model_name, allowance.why));
        }
    }

    // Reported together rather than as two asserts, so one run says everything
    // it found instead of only whichever tripped first. These runs are not
    // cheap.
    for (model_name, why) in &clean {
        failures.push(format!(
            "{model_name}: listed in KNOWN_DEVIATIONS but now meets the gate; \
             remove the entry so the list keeps describing something ({why})"
        ));
    }
    assert!(
        failures.is_empty(),
        "stamped Jacobians must match a numerical derivative of the stamped currents:\n{}",
        failures.join("\n")
    );
}

#[test]
fn stamped_reactive_blocks_match_a_numerical_derivative_of_the_stored_charge() {
    let mut failures = Vec::new();
    let mut clean = Vec::new();
    let mut reactive_models = 0usize;

    for model_name in builtins::builtin_names() {
        let Ok(mut harness) = GoldenHarness::new(model_name, &[]) else {
            continue;
        };
        let allowance = known_capacitance(model_name);
        let tolerance = allowance.map_or(CAPACITANCE_TOLERANCE, |known| known.relative_error);
        let allowed_gaps = allowance.map_or(0, |known| known.unverified_significant);

        let mut worst = 0.0_f64;
        let mut gaps = 0usize;
        let mut entries = 0usize;
        let mut tight = 0usize;
        let mut strict = 0usize;
        let mut stamped_anything = false;
        let mut stored_charge = 0.0_f64;

        for (index, point) in harness.probe_points(PROBE_POINTS).into_iter().enumerate() {
            match harness.stored_charges(&point) {
                Ok(charges) => {
                    stored_charge = charges
                        .iter()
                        .filter(|charge| charge.is_finite())
                        .fold(stored_charge, |worst, charge| worst.max(charge.abs()));
                }
                Err(error) => failures.push(format!(
                    "{model_name}: point {index}: charge recovery failed: {error}"
                )),
            }

            let audit = match harness.charge_audit(&point) {
                Ok(audit) => audit,
                Err(error) => {
                    failures.push(format!(
                        "{model_name}: point {index}: audit failed: {error}"
                    ));
                    continue;
                }
            };
            entries += audit.entries.len();
            tight += audit.checked().count();
            gaps += audit.unverified_significant().len();
            worst = worst.max(audit.worst_relative_error());
            strict += audit.failures(CAPACITANCE_TOLERANCE).len();
            stamped_anything |= audit
                .entries
                .iter()
                .any(|entry| entry.stamped.abs() > audit.significance_floor);

            for entry in audit.failures(tolerance).into_iter().take(3) {
                failures.push(format!(
                    "{model_name}: point {index}: d(q row {})/d(col {}) stamped {:e}, numeric {:e}, relative error {:.3e} (tolerance {tolerance:.1e})",
                    entry.row, entry.col, entry.stamped, entry.numeric, entry.relative_error
                ));
            }
        }

        if stamped_anything {
            reactive_models += 1;
        }
        eprintln!(
            "{model_name:<24} charge {stored_charge:.2e} reactive {}  verified {:>5.1}%  gaps {gaps:>4}  strict {strict:>3}  worst {worst:.2e}",
            if stamped_anything { "yes" } else { " no" },
            if entries == 0 {
                100.0
            } else {
                tight as f64 * 100.0 / entries as f64
            },
        );

        // The non-vacuity check, and the one that catches a whole dropped
        // capacitance. A model that stores charge must stamp something reactive;
        // a model that stamps nothing must store nothing. Stated per model
        // rather than as a corpus-wide count, because a build selecting one
        // resistor is a legitimate configuration and a corpus-wide count would
        // either fail it or be too weak to catch anything.
        if stored_charge > CHARGE_SIGNIFICANCE && !stamped_anything {
            failures.push(format!(
                "{model_name}: stores charge up to {stored_charge:.3e} C but stamps \
                 no reactive entry at all"
            ));
        }

        if gaps > allowed_gaps {
            failures.push(format!(
                "{model_name}: {gaps} significant reactive entries unverifiable, allowance is {allowed_gaps}"
            ));
        }
        if let Some(allowance) = allowance
            && gaps == 0
            && strict == 0
        {
            clean.push((*model_name, allowance.why));
        }
    }

    eprintln!("{reactive_models} built-ins stamp a reactive block");
    for (model_name, why) in &clean {
        failures.push(format!(
            "{model_name}: listed in KNOWN_CAPACITANCE_DEVIATIONS but now meets the \
             gate; remove the entry so the list keeps describing something ({why})"
        ));
    }
    assert!(
        failures.is_empty(),
        "stamped reactive blocks must match a numerical derivative of the stored charge:\n{}",
        failures.join("\n")
    );
}

/// A raised junction gmin, standing in for the solver's convergence assistance.
///
/// The engine does not read a single named endpoint — `junction_gmin` is passed
/// down as a parameter from whichever homotopy is running
/// (`engine/convergence/stamping.rs:107`), so there is no constant to borrow.
/// This is the magnitude the crate's own unit test for the plumbing uses
/// (`circuit/nonlinear.rs`, `generated_simparam_gmin_is_solver_controlled_...`),
/// and the exact value does not matter: what is under test is whether the device
/// sees a change at all.
const GMIN_RAISED: f64 = 1.0e-6;

/// Models whose stamp does not move when gmin does, at their probe biases.
///
/// Measured 2026-07-28 across all 42: twenty do not move and twenty-two do.
/// Eight of these twenty carry no `$simparam` read in their generated source at
/// all and are correctly inert — `EPFL_HEMT_10a`, `JUNCAP200`, `angelov`,
/// `angelov_gan`, `ekv_va`, `mosvar`, `r2_cmc`, `r2_et_cmc`. The rest do read
/// gmin, behind a guard their probe biases do not take.
///
/// `ekv3_rf` joined the corpus after that census (8be91f655) and is a ninth of
/// the first kind. The EKV3 302.00 source calls `$simparam` nowhere, so it
/// cannot see the solver's value; the `gmin` its parameter list carries is a
/// model parameter of its own and answers to the card, not to homotopy.
///
/// Checked in both directions, and scoped to models this build contains.
const GMIN_INERT: &[&str] = &[
    "EPFL_HEMT_10a",
    "JUNCAP200",
    "angelov",
    "angelov_gan",
    "asmhemt",
    "bsimcmg_va",
    "bsimsoi_va",
    "ekv3_rf",
    "ekv_va",
    "hisimhv_n4_va",
    "hisimhv_n5_va",
    "hisimhv_va",
    "hisimsoi_va__242bc21d",
    "hisimsoi_va__38074d06",
    "hisimsoi_va__5be18005",
    "hisimsotb_va",
    "mosvar",
    "r2_cmc",
    "r2_et_cmc",
    "r3_cmc",
    "vbic_4T_et_cf",
];

/// Gmin stepping has to reach the devices, and for the shipped corpus it did not.
///
/// A DC solve that will not converge raises the junction gmin and walks it back
/// down, and `circuit/nonlinear.rs:542` hands the current value to every
/// generated device through `GeneratedSimulationParameters::set_gmin`, which the
/// stamp then reads (`circuit/nonlinear.rs:1093`). Thirty of the forty-two
/// generated devices read `$simparam("gmin")`. The backend that produced the
/// shipped corpus folded that call to its literal fallback at generation time —
/// its emitted source contains no `simparam` at all for `diode_cmc`,
/// `bsimbulk` or `hicumL2va` — so the solver could raise gmin as far as it liked
/// and those devices behaved as though it were still zero. The homotopy was
/// running against models that could not feel it.
///
/// This pins the fix. It is a convergence property, so nothing in the numerical
/// gates would ever have caught it: a device that ignores gmin stamps a
/// perfectly correct Jacobian for the wrong gmin.
#[test]
fn gmin_stepping_reaches_the_devices_that_read_it() {
    let mut failures = Vec::new();
    let mut evaluated = Vec::new();
    let mut inert = Vec::new();

    for model_name in builtins::builtin_names() {
        let Ok(mut harness) = GoldenHarness::new(model_name, &[]) else {
            continue;
        };
        evaluated.push(*model_name);

        let mut response = 0.0_f64;
        for point in harness.probe_points(PROBE_POINTS) {
            let mut target = GeneratedSimulationParameters::new();
            target.set_gmin(GMIN);
            let mut start = GeneratedSimulationParameters::new();
            start.set_gmin(GMIN_RAISED);

            let (Ok((settled, _)), Ok((stepped, _))) = (
                harness.stamp_with(&point, target),
                harness.stamp_with(&point, start),
            ) else {
                continue;
            };
            let scale = settled
                .iter()
                .chain(stepped.iter())
                .filter(|value| value.is_finite())
                .fold(0.0_f64, |worst, value| worst.max(value.abs()));
            if scale == 0.0 {
                continue;
            }
            for (left, right) in settled.iter().zip(stepped.iter()) {
                if left.is_finite() && right.is_finite() {
                    response = response.max((left - right).abs() / scale);
                }
            }
        }

        // Exactly zero, not a tolerance. A device that never reads gmin
        // recomputes its whole stamp from inputs that did not change, so its
        // output is bit-identical and the difference is exactly `0.0`. Measured,
        // the corpus splits at fourteen orders of clear air: every responding
        // model moves by at least 2.5e-14 and every inert one by exactly nothing.
        // A threshold here would only be a way to get that wrong — the first
        // attempt used 1e-6 and reported DIODE_CMC, whose response is exactly
        // 1.00e-6, as inert.
        eprintln!("{model_name:<24} gmin response {response:.2e}");
        if response == 0.0 {
            inert.push(*model_name);
            if !GMIN_INERT.contains(model_name) {
                failures.push(format!(
                    "{model_name}: stamp does not move when gmin steps from \
                     {GMIN_RAISED:e} to {GMIN:e}, so the device cannot benefit \
                     from gmin stepping"
                ));
            }
        }
    }

    for model_name in GMIN_INERT {
        if !evaluated.contains(model_name) {
            continue;
        }
        if !inert.contains(model_name) {
            failures.push(format!(
                "{model_name}: listed in GMIN_INERT but now responds to gmin; \
                 remove the entry"
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "a device that reads $simparam(\"gmin\") must see the solver's value:\n{}",
        failures.join("\n")
    );
}
