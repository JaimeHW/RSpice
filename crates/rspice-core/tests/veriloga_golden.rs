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

use rspice_core::device::veriloga_generated::builtins;
use rspice_core::device::veriloga_harness::golden::GoldenHarness;

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
        relative_error: 1.0,
        unverified_significant: 16,
        why: "d(I)/d(V thermal node) is not stamped though shmod defaults to 2; \
              the current demonstrably depends on it (rows sum to zero, so the \
              omission is KCL-consistent and therefore a whole missing column)",
    },
    KnownDeviation {
        model: "asmesd_dio",
        relative_error: 1.0,
        unverified_significant: 10,
        why: "same missing thermal column as asmesd",
    },
    KnownDeviation {
        model: "bsimimg",
        relative_error: 1.0e-3,
        unverified_significant: 8,
        why: "non-finite at equilibrium; small disagreements away from it",
    },
    KnownDeviation {
        model: "hisimsoi_va__5be18005",
        relative_error: 2.0e-3,
        unverified_significant: 0,
        why: "a symmetric 2x2 conductance block stamps as zero at exactly zero \
              bias while the difference sees it on both sides — a guarded term \
              that vanishes only at the origin",
    },
    KnownDeviation {
        model: "hisimsoi_va__242bc21d",
        relative_error: 2.0e-3,
        unverified_significant: 0,
        why: "same zero-bias kink as hisimsoi_va__5be18005",
    },
    KnownDeviation {
        model: "hisimsoi_va__38074d06",
        relative_error: 2.0e-3,
        unverified_significant: 0,
        why: "same zero-bias kink as hisimsoi_va__5be18005",
    },
    KnownDeviation {
        model: "hisimsotb_va",
        relative_error: 1.0,
        unverified_significant: 0,
        why: "same zero-bias kink, with the stamped entry exactly zero",
    },
    KnownDeviation {
        model: "l_utsoi__485e0ac9",
        relative_error: 9.0e-2,
        unverified_significant: 4,
        why: "branch incidence stamps as -1 where the difference reads -0.914; \
              may still be the oracle on a stiff NQS branch rather than the model",
    },
    KnownDeviation {
        model: "bsimsoi_va",
        relative_error: 5.0e-5,
        unverified_significant: 0,
        why: "one near-floor entry disagrees just above the base tolerance",
    },
    KnownDeviation {
        model: "vbic13",
        relative_error: 2.0e-5,
        unverified_significant: 0,
        why: "one entry at 1e-12 magnitude disagrees just above the base tolerance",
    },
    KnownDeviation {
        model: "EPFL_HEMT_10a",
        relative_error: 1.0e-5,
        unverified_significant: 10,
        why: "equilibrium entries the difference cannot resolve",
    },
    KnownDeviation {
        model: "r3_cmc",
        relative_error: 1.0e-5,
        unverified_significant: 4,
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

const KNOWN_CAPACITANCE_DEVIATIONS: &[KnownCapacitanceDeviation] = &[];

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
                            failures.push(format!(
                                "{model_name}: point {index}: stamp is not finite"
                            ));
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
                    failures.push(format!("{model_name}: point {index}: audit failed: {error}"));
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
            "{model_name:<24} entries {entries:>6}  verified {:>5.1}%  gaps {gaps:>4}  worst {worst:.2e}",
            if entries == 0 { 100.0 } else { tight as f64 * 100.0 / entries as f64 },
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
        if allowance.is_some() && gaps == 0 && strict == 0 {
            clean.push(*model_name);
        }
    }

    assert!(
        clean.is_empty(),
        "these models are listed in KNOWN_DEVIATIONS but now meet the gate; \
         remove them so the list keeps describing something:\n{}",
        clean.join("\n")
    );
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
                    failures.push(format!("{model_name}: point {index}: audit failed: {error}"));
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
            "{model_name:<24} charge {stored_charge:.2e} reactive {}  verified {:>5.1}%  gaps {gaps:>4}  worst {worst:.2e}",
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
        if allowance.is_some() && gaps == 0 && strict == 0 {
            clean.push(*model_name);
        }
    }

    eprintln!("{reactive_models} built-ins stamp a reactive block");
    assert!(
        clean.is_empty(),
        "these models are listed in KNOWN_CAPACITANCE_DEVIATIONS but now meet the gate; \
         remove them so the list keeps describing something:\n{}",
        clean.join("\n")
    );
    assert!(
        failures.is_empty(),
        "stamped reactive blocks must match a numerical derivative of the stored charge:\n{}",
        failures.join("\n")
    );
}
