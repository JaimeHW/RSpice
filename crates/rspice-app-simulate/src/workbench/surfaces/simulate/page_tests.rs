//! Contract tests for the Simulation Studio setup pages.
//!
//! These check what the pages promise rather than how they paint: every route
//! renders without an egui identity clash, the page's own subject reaches the
//! screen, and the numerical controls resolve to exactly the values the run
//! will use.

pub(crate) mod capture_groups;
mod field_editing;
mod plan_removal;
mod settings_switch;
mod solver_options;

use egui::{Rect, vec2};

use super::fixtures::plan_id;
use super::page_kit::column_rects;
use crate::simulation::dialog::SimulationOptions;
use crate::simulation::plan::OptionsDialogState;
use crate::workbench::framework::feature::harness::FeatureHarness;
use crate::workbench::simulate::SimulateFeature;
use crate::workbench::simulate::test_host::SimulateTestHost;

/// The two temperature buffers are separate values with separate meanings, and
/// the page commits each through the same transaction the rest of the card set
/// uses. Editing one must not move the other.
#[test]
fn the_model_reference_temperature_commits_without_moving_the_run_temperature() {
    let mut harness = SimulateTestHost::harness();
    let run_temperature = harness.model.sim_setup.options.temp;

    harness.model.sim_setup.session.options_draft.tnom = "40.0".to_owned();
    super::page_solver::commit_draft(&mut harness.cx());

    assert_eq!(harness.model.sim_setup.options.tnom, 40.0);
    assert_eq!(harness.model.sim_setup.options.temp, run_temperature);
    assert!(
        !harness.model.analysis_lifecycle_status.is_refusal(),
        "{}",
        harness.model.analysis_lifecycle_status.message()
    );
}

/// The bypass bounds are only stated in the deck while bypass is enabled, so
/// the page's floor has to travel with the toggle rather than on its own.
#[test]
fn the_bypass_voltage_floor_commits_through_the_pages_own_channel() {
    let mut harness = SimulateTestHost::harness();

    harness.model.sim_setup.session.options_draft.bypass_enabled = true;
    harness.model.sim_setup.session.options_draft.bypass_abstol = "4e-9".to_owned();
    super::page_solver::commit_draft(&mut harness.cx());

    assert!(harness.model.sim_setup.options.bypass_enabled);
    assert_eq!(harness.model.sim_setup.options.bypass_abstol, 4e-9);
    let deck = rspice_simulation::analysis_preparation::apply_simulation_options_to_netlist(
        "bypass options\nV1 1 0 1\nR1 1 0 1k\n.op\n.end\n",
        &harness.model.sim_setup.options,
    );
    let parsed = rspice_core::netlist::parse_netlist(&deck).expect("the committed options parse");
    assert_eq!(parsed.options.bypass_abstol, Some(4e-9));
}

/// Every option a page edits must survive a draft round trip.
///
/// A field with no draft buffer was silently reset to a literal on the next
/// commit, so a value the user set never reached the engine. The pivot
/// thresholds, the DC-sweep budget and arc-length continuation all behaved
/// that way before they were given buffers.
#[test]
fn every_edited_option_survives_a_draft_round_trip() {
    let options = SimulationOptions {
        pivrel: 0.125,
        pivtol: 2.5e-14,
        itl1: 77,
        itl4: 21,
        arc_length: true,
        gmin_stepping: false,
        source_stepping: false,
        pseudo_transient: false,
        reltol: 4.5e-5,
        residual_reltol: 6.5e-5,
        vntol: 3.5e-7,
        abstol: 7.5e-13,
        iabstol: 8.5e-13,
        chgtol: 9.5e-15,
        ..SimulationOptions::default()
    };
    let round_tripped = OptionsDialogState::from_options(&options)
        .to_options()
        .expect("a draft built from valid options must parse");
    assert_eq!(
        serde_json::to_value(&round_tripped).expect("serializable"),
        serde_json::to_value(&options).expect("serializable"),
        "a draft round trip changed the effective options"
    );
}

/// The solver strip pairs each preset with its intent by position, so the two
/// lists have to stay the same length and in the same order. Getting this
/// wrong would describe one policy while applying another.
#[test]
fn preset_intents_line_up_with_the_named_presets() {
    assert_eq!(
        SimulationOptions::PRESETS.len(),
        super::page_solver::PRESET_INTENT.len()
    );
    let labels: Vec<&str> = SimulationOptions::PRESETS
        .iter()
        .map(|(label, _)| *label)
        .collect();
    assert_eq!(labels, ["Fast", "Balanced", "Accurate", "Robust"]);

    // Each preset must recognize itself, and an edited set must recognize none.
    for (label, build) in SimulationOptions::PRESETS {
        assert_eq!(build().preset_name(), Some(label));
        assert_eq!(build().preset_label(), label);
    }
    let mut edited = SimulationOptions::default();
    edited.reltol *= 2.0;
    assert_eq!(edited.preset_name(), None);
    assert_eq!(edited.preset_label(), "Custom");
}

/// Presets must be distinguishable by exact comparison, because that is how
/// the strip decides which chip is active. Two presets that serialize
/// identically would make the strip report the wrong policy.
#[test]
fn the_named_presets_are_pairwise_distinct() {
    let presets = [
        SimulationOptions::fast(),
        SimulationOptions::default(),
        SimulationOptions::accurate(),
        SimulationOptions::robust(),
    ];
    for (index, left) in presets.iter().enumerate() {
        for right in presets.iter().skip(index + 1) {
            assert_ne!(
                serde_json::to_value(left).expect("serializable"),
                serde_json::to_value(right).expect("serializable"),
                "two named presets resolve to the same option set"
            );
        }
    }
}

/// A registry that can only grow is not an editor. Removal has to be a real,
/// validated transaction on the plan payload — not a view-local filter.
#[test]
fn a_saved_output_can_be_removed_and_the_removal_is_validated() {
    use crate::product::SavedOutputId;
    use crate::state::{
        SavedOutput, SavedOutputCompatibility, SavedOutputKind, SavedOutputPolicy,
        SavedOutputPrecision, SavedOutputStreaming,
    };

    let mut harness = SimulateTestHost::harness();
    let plan_id = harness
        .model
        .sim_setup
        .stable_analysis_plan()
        .expect("test instance has a stable plan")
        .id();
    let output = SavedOutput::new(
        SavedOutputKind::RawVoltageOrCurrent,
        "vout",
        "V(out)",
        SavedOutputCompatibility::AllCompatibleAnalyses,
        SavedOutputPolicy::ALL[0],
        SavedOutputPrecision::ALL[0],
        SavedOutputStreaming::StoreOnly,
    )
    .expect("valid saved output");
    let output_id = output.id;
    harness
        .model
        .workspace
        .content
        .add_saved_output(plan_id, output)
        .expect("the plan accepts a valid output");
    assert_eq!(
        harness
            .model
            .workspace
            .content
            .plan_data(plan_id)
            .expect("payload")
            .saved_outputs
            .len(),
        1
    );

    let removed = harness
        .model
        .workspace
        .content
        .remove_saved_output(plan_id, output_id)
        .expect("the output it just accepted can be removed");
    assert_eq!(removed.name, "vout");
    assert!(
        harness
            .model
            .workspace
            .content
            .plan_data(plan_id)
            .expect("payload")
            .saved_outputs
            .is_empty()
    );

    // Removing the same identity twice must fail rather than silently succeed.
    assert!(
        harness
            .model
            .workspace
            .content
            .remove_saved_output(plan_id, output_id)
            .is_err()
    );
    assert!(
        harness
            .model
            .workspace
            .content
            .remove_saved_output(plan_id, SavedOutputId::new())
            .is_err()
    );
}

/// Duplicating exists so a second signal can inherit a capture contract that
/// was already tuned — the expression is the one thing the copy is meant to
/// change. If the copy shared the original's identity it would be a second
/// reference rather than a new record, and renaming one would rename both.
#[test]
fn a_duplicated_saved_output_inherits_everything_but_its_identity_and_name() {
    use crate::product::{ObjectRevision, SavedOutputId};
    use crate::state::{
        SavedOutput, SavedOutputCompatibility, SavedOutputKind, SavedOutputPolicy,
        SavedOutputPrecision, SavedOutputStreaming,
    };

    let mut harness = SimulateTestHost::harness();
    let plan_id = plan_id(&harness.model);
    // Deliberately non-default capture decisions: a copy that silently reset
    // them to the defaults would still pass a weaker assertion.
    let output = SavedOutput::new(
        SavedOutputKind::RawVoltageOrCurrent,
        "vout",
        "V(out)",
        SavedOutputCompatibility::AllCompatibleAnalyses,
        SavedOutputPolicy::ALL[SavedOutputPolicy::ALL.len() - 1],
        SavedOutputPrecision::ALL[SavedOutputPrecision::ALL.len() - 1],
        SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation,
    )
    .expect("valid saved output");
    let output_id = output.id;
    let source = output.clone();
    harness
        .model
        .workspace
        .content
        .add_saved_output(plan_id, output)
        .expect("the plan accepts a valid output");

    let copy_id = harness
        .model
        .workspace
        .content
        .duplicate_saved_output(plan_id, output_id, "vout_copy")
        .expect("a free name duplicates");
    assert!(copy_id != output_id, "the copy is its own record");

    let payload = harness
        .model
        .workspace
        .content
        .plan_data(plan_id)
        .expect("payload");
    assert_eq!(payload.saved_outputs.len(), 2);
    let copy = payload
        .saved_outputs
        .iter()
        .find(|output| output.id == copy_id)
        .expect("the committed copy is in the registry");
    assert_eq!(copy.name, "vout_copy");
    assert!(copy.revision == ObjectRevision::INITIAL);
    // Comparing the whole record, rather than listing the fields, is what keeps
    // this honest when SavedOutput grows another capture decision.
    let mut expected = source;
    expected.id = copy.id;
    expected.revision = copy.revision;
    expected.name.clone_from(&copy.name);
    assert!(
        *copy == expected,
        "the copy differs from its source only in identity and name"
    );

    // The registry validates uniqueness case-insensitively, so a differently
    // cased collision has to be refused too — and refused atomically.
    assert!(
        harness
            .model
            .workspace
            .content
            .duplicate_saved_output(plan_id, output_id, "VOUT")
            .is_err()
    );
    assert!(
        harness
            .model
            .workspace
            .content
            .duplicate_saved_output(plan_id, SavedOutputId::new(), "orphan")
            .is_err()
    );
    assert_eq!(
        harness
            .model
            .workspace
            .content
            .plan_data(plan_id)
            .expect("payload")
            .saved_outputs
            .len(),
        2,
        "a refused duplicate leaves the registry exactly as it was"
    );
}

/// Both registries derive the copy's name rather than opening a dialog over a
/// table that is about to change, so the derivation has to agree with the
/// case-insensitive uniqueness rule the plan itself enforces.
#[test]
fn a_duplicate_name_skips_every_name_the_registry_already_holds() {
    use super::workflows::unique_copy_name;

    let held = |names: &'static [&'static str]| {
        move |candidate: &str| {
            names
                .iter()
                .any(|name| name.eq_ignore_ascii_case(candidate))
        }
    };

    assert_eq!(unique_copy_name("vout", held(&[])), "vout_copy");
    assert_eq!(unique_copy_name("vout", held(&["vout"])), "vout_copy");
    assert_eq!(
        unique_copy_name("vout", held(&["vout", "vout_copy"])),
        "vout_copy_2"
    );
    assert_eq!(
        unique_copy_name("vout", held(&["VOUT_COPY", "vout_copy_2"])),
        "vout_copy_3"
    );
}

/// A capture policy an engineer can read but not change is a report, not a
/// control. The Save & streaming page totals these exact fields, so each one
/// has to be editable through a validated transaction that moves the plan
/// revision.
#[test]
fn a_saved_output_capture_policy_can_be_changed_after_it_is_authored() {
    use crate::state::{
        SavedOutput, SavedOutputCompatibility, SavedOutputKind, SavedOutputPolicy,
        SavedOutputPrecision, SavedOutputStreaming,
    };

    let mut harness = SimulateTestHost::harness();
    let plan_id = harness
        .model
        .sim_setup
        .stable_analysis_plan()
        .expect("test instance has a stable plan")
        .id();
    let output = SavedOutput::new(
        SavedOutputKind::RawVoltageOrCurrent,
        "vout",
        "V(out)",
        SavedOutputCompatibility::AllCompatibleAnalyses,
        SavedOutputPolicy::EveryAcceptedPoint,
        SavedOutputPrecision::FullSourcePrecision,
        SavedOutputStreaming::StoreOnly,
    )
    .expect("valid saved output");
    let output_id = output.id;
    let original_revision = output.revision;
    harness
        .model
        .workspace
        .content
        .add_saved_output(plan_id, output.clone())
        .expect("the plan accepts a valid output");

    let mut replacement = output.clone();
    replacement.save_policy = SavedOutputPolicy::FailureDiagnosticsOnly;
    replacement.stored_precision = SavedOutputPrecision::DisplayCacheWithFullSourcePrecision;
    replacement.streaming = SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation;
    let revision = harness
        .model
        .workspace
        .content
        .replace_saved_output(plan_id, output_id, replacement)
        .expect("a valid capture contract is accepted");
    assert_ne!(
        revision, original_revision,
        "an accepted edit must advance the output's own revision"
    );
    let stored = harness
        .model
        .workspace
        .content
        .plan_data(plan_id)
        .expect("payload")
        .saved_outputs
        .iter()
        .find(|candidate| candidate.id == output_id)
        .expect("the output survives the edit")
        .clone();
    assert_eq!(
        stored.save_policy,
        SavedOutputPolicy::FailureDiagnosticsOnly
    );
    assert_eq!(
        stored.stored_precision,
        SavedOutputPrecision::DisplayCacheWithFullSourcePrecision
    );
    assert_eq!(
        stored.streaming,
        SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation
    );
    assert_eq!(
        stored.name, "vout",
        "editing the capture policy must not rename the output"
    );

    // Re-committing the same contract is a semantic no-op: it must not spend a
    // revision, or every repaint that re-selects the same option would.
    let unchanged = harness
        .model
        .workspace
        .content
        .replace_saved_output(plan_id, output_id, stored.clone())
        .expect("an unchanged contract is accepted");
    assert_eq!(
        unchanged, revision,
        "re-committing an identical contract must not advance the revision"
    );

    // An edit that collides with another output's name is refused whole.
    let other = SavedOutput::new(
        SavedOutputKind::RawVoltageOrCurrent,
        "vin",
        "V(in)",
        SavedOutputCompatibility::AllCompatibleAnalyses,
        SavedOutputPolicy::EveryAcceptedPoint,
        SavedOutputPrecision::FullSourcePrecision,
        SavedOutputStreaming::StoreOnly,
    )
    .expect("valid saved output");
    harness
        .model
        .workspace
        .content
        .add_saved_output(plan_id, other)
        .expect("the plan accepts a second output");
    let mut collision = stored;
    collision.name = "VIN".to_owned();
    assert!(
        harness
            .model
            .workspace
            .content
            .replace_saved_output(plan_id, output_id, collision)
            .is_err(),
        "a name that collides case-insensitively with another output must be refused"
    );
    assert_eq!(
        harness
            .model
            .workspace
            .content
            .plan_data(plan_id)
            .expect("payload")
            .saved_outputs
            .iter()
            .find(|candidate| candidate.id == output_id)
            .expect("the output is untouched by the refused edit")
            .name,
        "vout"
    );
}

/// An output's name and expression are the two things most likely to be wrong
/// after authoring — a typo'd node, a signal renamed in the schematic. Both
/// have to be fixable without deleting and re-authoring the contract.
#[test]
fn a_saved_output_can_be_renamed_and_its_expression_rewritten() {
    use crate::state::{
        SavedOutput, SavedOutputCompatibility, SavedOutputKind, SavedOutputPolicy,
        SavedOutputPrecision, SavedOutputStreaming,
    };

    let mut harness = SimulateTestHost::harness();
    let plan_id = plan_id(&harness.model);
    let output = SavedOutput::new(
        SavedOutputKind::RawVoltageOrCurrent,
        "vout",
        "V(otu)",
        SavedOutputCompatibility::AllCompatibleAnalyses,
        SavedOutputPolicy::EveryAcceptedPoint,
        SavedOutputPrecision::FullSourcePrecision,
        SavedOutputStreaming::StoreOnly,
    )
    .expect("valid saved output");
    let output_id = output.id;
    harness
        .model
        .workspace
        .content
        .add_saved_output(plan_id, output.clone())
        .expect("the plan accepts it");

    let mut corrected = output;
    corrected.name = "v_out".to_owned();
    corrected.source_expression = "V(out)".to_owned();
    harness
        .model
        .workspace
        .content
        .replace_saved_output(plan_id, output_id, corrected)
        .expect("a corrected contract is accepted");

    let stored = harness
        .model
        .workspace
        .content
        .plan_data(plan_id)
        .expect("payload")
        .saved_outputs
        .iter()
        .find(|candidate| candidate.id == output_id)
        .expect("the output keeps its identity across a rename")
        .clone();
    assert_eq!(stored.name, "v_out");
    assert_eq!(stored.source_expression, "V(out)");
    assert_eq!(
        stored.save_policy,
        SavedOutputPolicy::EveryAcceptedPoint,
        "correcting the expression must not disturb the capture policy"
    );
}

/// The composed run space decides how many points a dispatch executes, so a
/// change to it has to move the plan revision. Otherwise an authorized
/// preflight could be followed by a sweep change and a dispatch that ran a
/// different run space than the one that was checked.
#[test]
fn a_run_space_change_moves_the_plan_revision() {
    let mut harness = SimulateTestHost::harness();
    let before = harness
        .model
        .sim_setup
        .stable_analysis_plan()
        .expect("stable plan")
        .revision();
    let id = harness.model.sim_setup.run_set.dimensions[2].id.clone();
    let transaction = rspice_simulation_contract::run_set::dispatch(
        &mut harness.model.sim_setup.run_set,
        rspice_simulation_contract::run_set::RunSetAction::SetValues {
            id,
            text: "-40\n0\n25\n85\n125".to_owned(),
        },
        1,
    );
    assert!(transaction.was_adopted(), "{:?}", transaction.receipt);
    harness
        .model
        .sim_setup
        .commit_active_plan_configuration_change("Run set · set-dimension-values".to_owned())
        .expect("a run-space change commits as a configuration change");
    let after = harness
        .model
        .sim_setup
        .stable_analysis_plan()
        .expect("stable plan")
        .revision();
    assert_ne!(
        before, after,
        "a run-space change must advance the plan revision so preflight is invalidated"
    );
}

/// An expression an engineer cannot change is a create-only registry.
#[test]
fn a_design_variable_expression_can_be_edited_in_place() {
    use crate::state::{
        DesignVariable, DesignVariableOverridePolicy, DesignVariableQuantity, DesignVariableScope,
        DesignVariableSweepEligibility,
    };

    let mut harness = SimulateTestHost::harness();
    let plan_id = harness
        .model
        .sim_setup
        .stable_analysis_plan()
        .expect("stable plan")
        .id();
    let variable = DesignVariable::new(
        "rload",
        "1kohm",
        DesignVariableQuantity::Resistance,
        DesignVariableScope::Testbench,
        "load resistance",
        None,
        DesignVariableSweepEligibility::NestedSweepAndOptimization,
        DesignVariableOverridePolicy::ExplicitTestLocalOverride,
    )
    .expect("valid design variable");
    let variable_id = variable.id;
    let original_revision = variable.revision;
    harness
        .model
        .workspace
        .content
        .add_design_variable(plan_id, variable)
        .expect("the plan accepts a valid variable");

    let revision = harness
        .model
        .workspace
        .content
        .update_design_variable_expression(plan_id, variable_id, "2kohm")
        .expect("a valid expression is accepted");
    assert_ne!(
        revision, original_revision,
        "an accepted edit must advance the variable's own revision"
    );
    let stored = harness
        .model
        .workspace
        .content
        .plan_data(plan_id)
        .expect("payload")
        .design_variables
        .iter()
        .find(|candidate| candidate.id == variable_id)
        .expect("the variable survives the edit");
    assert_eq!(stored.expression, "2kohm");
    assert_eq!(
        stored.name, "rload",
        "editing the expression must not rename the variable"
    );
}

/// A variable's typed contract — quantity, scope, sweep role, override policy
/// and bounds — decides what a sweep may do with it and what preflight may
/// reject. All of it has to be reachable after authoring, and an unbounded
/// variable has to be able to become bounded.
#[test]
fn a_design_variable_contract_can_be_retyped_and_bounded_after_it_is_authored() {
    use crate::state::{
        DesignVariable, DesignVariableOverridePolicy, DesignVariableQuantity, DesignVariableRange,
        DesignVariableScope, DesignVariableSweepEligibility,
    };

    let mut harness = SimulateTestHost::harness();
    let plan_id = harness
        .model
        .sim_setup
        .stable_analysis_plan()
        .expect("stable plan")
        .id();
    let variable = DesignVariable::new(
        "rload",
        "1kohm",
        DesignVariableQuantity::Resistance,
        DesignVariableScope::Testbench,
        "load resistance",
        None,
        DesignVariableSweepEligibility::FixedParameter,
        DesignVariableOverridePolicy::InheritOwnerOnly,
    )
    .expect("valid design variable");
    let variable_id = variable.id;
    harness
        .model
        .workspace
        .content
        .add_design_variable(plan_id, variable.clone())
        .expect("the plan accepts a valid variable");
    assert!(
        variable.allowed_range.is_none(),
        "the variable starts unbounded"
    );

    let mut replacement = variable.clone();
    replacement.sweep_eligibility = DesignVariableSweepEligibility::NestedSweepAndOptimization;
    replacement.override_policy = DesignVariableOverridePolicy::ExplicitTestLocalOverride;
    replacement.scope = DesignVariableScope::Project;
    replacement.allowed_range = Some(DesignVariableRange {
        minimum: "500ohm".to_owned(),
        maximum: "2kohm".to_owned(),
    });
    harness
        .model
        .workspace
        .content
        .replace_design_variable(plan_id, variable_id, replacement)
        .expect("a valid contract is accepted");

    let stored = harness
        .model
        .workspace
        .content
        .plan_data(plan_id)
        .expect("payload")
        .design_variables
        .iter()
        .find(|candidate| candidate.id == variable_id)
        .expect("the variable survives the edit")
        .clone();
    assert_eq!(
        stored.sweep_eligibility,
        DesignVariableSweepEligibility::NestedSweepAndOptimization
    );
    assert_eq!(
        stored.override_policy,
        DesignVariableOverridePolicy::ExplicitTestLocalOverride
    );
    assert_eq!(stored.scope, DesignVariableScope::Project);
    assert_eq!(
        stored
            .allowed_range
            .as_ref()
            .map(|range| (range.minimum.as_str(), range.maximum.as_str())),
        Some(("500ohm", "2kohm")),
        "a variable that was authored unbounded must be able to gain a bound"
    );
    assert_eq!(
        stored.expression, "1kohm",
        "retyping the contract must not disturb the expression"
    );

    // Clearing both bounds is a real choice: it makes the variable unbounded
    // again rather than leaving a half-open range preflight cannot check.
    let mut unbounded = stored;
    unbounded.allowed_range = None;
    harness
        .model
        .workspace
        .content
        .replace_design_variable(plan_id, variable_id, unbounded)
        .expect("removing the bound is accepted");
    assert!(
        harness
            .model
            .workspace
            .content
            .plan_data(plan_id)
            .expect("payload")
            .design_variables
            .iter()
            .find(|candidate| candidate.id == variable_id)
            .expect("the variable survives")
            .allowed_range
            .is_none()
    );
}

#[test]
fn ledger_columns_tile_their_row_without_a_gap_or_an_overhang() {
    let row = Rect::from_min_size(egui::Pos2::ZERO, vec2(640.0, 28.0));
    for fractions in [
        vec![0.36, 0.34, 0.30],
        vec![0.18, 0.52, 0.30],
        vec![0.20, 0.13, 0.28, 0.17, 0.22],
    ] {
        let columns = column_rects(row, &fractions);
        assert_eq!(columns.len(), fractions.len());
        assert_eq!(columns[0].left(), row.left());
        assert_eq!(
            columns[columns.len() - 1].right(),
            row.right(),
            "the last column must absorb the remainder so no strip of the row is unclaimed"
        );
        assert!(columns.iter().all(|column| column.width() > 0.0));
        assert!(
            columns
                .windows(2)
                .all(|pair| pair[0].right() == pair[1].left())
        );
    }
}

/// An override the analysis's solve would never read must be refused, not
/// stored. A stored-and-ignored bound is indistinguishable from one that works,
/// and the ledger would then report a policy no run resolves to.
///
/// The record is built against a transient and then offered to an AC sweep,
/// because that is the shape a restored project or a retyped analysis takes:
/// the authoring gate is not the only place the pairing has to hold.
#[test]
fn an_override_the_analysis_cannot_use_is_refused_and_leaves_the_plan_unchanged() {
    use crate::simulation::plan::{AnalysisKind, AnalysisNumericOverride, NumericOverrideOption};

    let mut harness = SimulateTestHost::harness();
    let plan = harness
        .model
        .sim_setup
        .stable_analysis_plan_mut()
        .expect("the test instance has a stable plan");
    let (ac, _) = plan.insert(AnalysisKind::Ac).expect("an AC sweep inserts");

    let mut record = AnalysisNumericOverride::default();
    record
        .set_for_instance(
            AnalysisKind::Transient,
            crate::simulation::plan::SolverOwnership::NONE,
            NumericOverrideOption::Itl4,
            "12",
        )
        .expect("a transient takes timesteps");

    let before = serde_json::to_vec(&plan).expect("the plan serializes");
    let error = plan
        .set_numeric_override(ac, Some(record))
        .expect_err("an AC sweep never takes a timestep");
    let after = serde_json::to_vec(&plan).expect("the plan serializes");

    assert_eq!(
        before, after,
        "a refused override must leave the plan byte-for-byte unchanged"
    );
    assert!(
        error.to_string().contains("ITL4"),
        "the refusal must name the option it refused: {error}"
    );
}

/// Projects written before per-analysis numerics existed have to keep opening.
/// `AnalysisInstance` denies unknown fields, so the new one has to default when
/// absent — and stay absent on the way out, so re-saving an untouched plan does
/// not rewrite every instance.
#[test]
fn a_plan_written_before_per_analysis_numerics_still_loads() {
    use crate::simulation::plan::{
        AnalysisDraft, AnalysisKind, AnalysisNumericOverride, NumericOverrideOption, SimulationPlan,
    };

    let mut plan = SimulationPlan::new();
    let serialized = serde_json::to_string(&plan).expect("the plan serializes");
    assert!(
        !serialized.contains("numeric_override"),
        "an inheriting plan must serialize exactly as it did before the field existed"
    );
    let restored: SimulationPlan =
        serde_json::from_str(&serialized).expect("a plan without the field still loads");
    assert!(restored.instances()[0].numeric_override().is_none());

    let id = plan
        .instances()
        .iter()
        .find(|instance| matches!(instance.draft(), AnalysisDraft::Transient(_)))
        .map(|instance| instance.id())
        .expect("a fresh plan holds one transient");
    let mut record = AnalysisNumericOverride::default();
    record
        .set_for_instance(
            AnalysisKind::Transient,
            crate::simulation::plan::SolverOwnership::NONE,
            NumericOverrideOption::Trtol,
            "3",
        )
        .expect("a transient has a truncation bound");
    plan.set_numeric_override(id, Some(record))
        .expect("the plan accepts it");
    let round_tripped: SimulationPlan =
        serde_json::from_str(&serde_json::to_string(&plan).expect("serializes"))
            .expect("an authored plan round trips");
    assert_eq!(
        round_tripped
            .instance(id)
            .and_then(|instance| instance.numeric_override())
            .and_then(|record| record.stated(NumericOverrideOption::Trtol)),
        Some(crate::simulation::plan::OverrideValue::Real(3.0))
    );
}

/// The scope is part of the requirement, so changing it has to move the plan
/// the same way every other specification edit does: one validated
/// transaction that leaves a receipt and invalidates preflight. A rejected
/// scope must leave the plan exactly as it was.
#[test]
fn a_scope_change_commits_as_a_plan_transaction_and_a_rejected_one_changes_nothing() {
    use crate::state::{SpecEntry, SpecPointScope};

    let mut harness = SimulateTestHost::harness();
    let id = plan_id(&harness.model);
    harness.model.workspace.content.replace_active_specs(
        id,
        vec![SpecEntry {
            measurement: "gain".to_owned(),
            expression: "meas ac gain max V(out)".to_owned(),
            min: Some(10.0),
            max: None,
            unit: "dB".to_owned(),
            scope: SpecPointScope::AllPoints,
        }],
    );
    let revision = |harness: &FeatureHarness<SimulateFeature>| {
        harness
            .model
            .sim_setup
            .stable_analysis_plan()
            .expect("stable plan")
            .revision()
    };
    let before = revision(&harness);

    // Matched case-insensitively, exactly as the selection resolves it.
    super::page_specs::commit_scope(&mut harness.cx(), "GAIN", SpecPointScope::Nominal);
    assert_eq!(
        harness.model.workspace.content.active_specs(id)[0].scope,
        SpecPointScope::Nominal
    );
    assert_ne!(
        revision(&harness),
        before,
        "an accepted edit moves the plan revision"
    );
    assert!(
        !harness.model.analysis_lifecycle_status.is_refusal(),
        "the transaction reports its receipt"
    );

    let after_accept = revision(&harness);
    super::page_specs::commit_scope(
        &mut harness.cx(),
        "gain",
        SpecPointScope::SelectedCorners {
            corners: Vec::new(),
        },
    );
    assert_eq!(
        harness.model.workspace.content.active_specs(id)[0].scope,
        SpecPointScope::Nominal,
        "a rejected scope leaves the stored requirement untouched"
    );
    assert_eq!(
        revision(&harness),
        after_accept,
        "a rejected edit does not move the plan revision"
    );
}

#[test]
fn specification_policy_commits_atomically_and_rejects_an_invalid_yield_gate() {
    use crate::state::{
        MissingMeasurementPolicy, MonteCarloSpecificationGate, NominalFailurePolicy,
        RegressionSpecificationPolicy, SpecificationPolicy,
    };

    let mut harness = SimulateTestHost::harness();
    let id = plan_id(&harness.model);
    let revision = |harness: &FeatureHarness<SimulateFeature>| {
        harness
            .model
            .sim_setup
            .stable_analysis_plan()
            .expect("stable plan")
            .revision()
    };
    let before = revision(&harness);
    let accepted = SpecificationPolicy {
        nominal_failure: NominalFailurePolicy::RecordDisposition,
        monte_carlo: MonteCarloSpecificationGate::YieldAtLeast { percent: 99.9 },
        regression: RegressionSpecificationPolicy::LimitOnly,
        missing_measurement: MissingMeasurementPolicy::ReportUnmapped,
    };

    super::page_specs::commit_specification_policy(&mut harness.cx(), accepted.clone());

    let stored = &harness
        .model
        .workspace
        .content
        .plan_data(id)
        .expect("plan payload")
        .specification_policy;
    assert!(stored.bitwise_eq(&accepted));
    assert_ne!(
        revision(&harness),
        before,
        "an accepted policy moves the plan revision"
    );

    let after_accept = revision(&harness);
    super::page_specs::commit_specification_policy(
        &mut harness.cx(),
        SpecificationPolicy {
            monte_carlo: MonteCarloSpecificationGate::YieldAtLeast { percent: 100.1 },
            ..accepted.clone()
        },
    );
    let stored = &harness
        .model
        .workspace
        .content
        .plan_data(id)
        .expect("plan payload")
        .specification_policy;
    assert!(stored.bitwise_eq(&accepted));
    assert_eq!(
        revision(&harness),
        after_accept,
        "a rejected policy cannot move the plan revision"
    );
    assert!(harness.model.analysis_lifecycle_status.is_refusal());
}

/// A solver value the page refuses to apply has to say so on the channel the
/// surface drains. The page is the only editor of these options, so this
/// channel is the only place a refusal can surface: a value that vanished
/// silently would leave the field showing text no run will use.
#[test]
fn a_refused_solver_value_reports_on_the_plan_lifecycle_channel() {
    let plan_records = |harness: &FeatureHarness<SimulateFeature>| {
        harness
            .model
            .log_buffer
            .entries()
            .filter(|entry| entry.source == crate::diagnostics::LogSource::Plan)
            .count()
    };
    let mut harness = SimulateTestHost::harness();
    let committed = plan_records(&harness);
    harness.model.sim_setup.session.options_draft.reltol = "not a number".to_owned();

    super::page_solver::commit_draft(&mut harness.cx());

    let outcome = &harness.model.analysis_lifecycle_status;
    assert!(outcome.is_refusal(), "{}", outcome.message());
    assert!(
        plan_records(&harness) > committed,
        "a refusal is an event the Console records"
    );
    assert!(
        outcome.message().contains("were not applied"),
        "{}",
        outcome.message()
    );
    assert_eq!(
        harness.model.sim_setup.options.reltol,
        SimulationOptions::default().reltol,
        "a refused value must leave the effective options alone"
    );

    // Repairing the value applies it, and the channel carries the receipt.
    harness.model.sim_setup.session.options_draft.reltol = "2e-3".to_owned();
    super::page_solver::commit_draft(&mut harness.cx());
    let outcome = &harness.model.analysis_lifecycle_status;
    assert!(!outcome.is_refusal(), "{}", outcome.message());
    assert_eq!(harness.model.sim_setup.options.reltol, 2e-3);
}

/// The operating point's Newton budget is not the plan's ITL1.
///
/// `AccuracyPolicy::apply` assigns `max_iterations` from the analysis's tier
/// *after* the deck's `.OPTIONS` are resolved, so the plan's ITL1 never binds
/// an `.op` solve. The ledger used to report it as "plan preset" anyway, which
/// is a number no operating point has ever run. The row now states the tier's
/// budget and names the tier as its owner, in the same sentence the override
/// gate refuses ITL1 with.
#[test]
fn the_operating_points_newton_budget_is_reported_as_its_accuracy_tiers() {
    use crate::simulation::accuracy::AnalysisAccuracy;
    use crate::simulation::plan::{AnalysisKind, NumericOverrideOption};

    let mut harness = SimulateTestHost::harness();
    harness.model.sim_setup.options.itl1 = 50;
    super::lifecycle::insert_analysis_instance(&mut harness.cx(), AnalysisKind::OperatingPoint)
        .expect("an operating point inserts into the test instance plan");

    let budget_label = NumericOverrideOption::Itl1.label();
    let rows = super::page_solver::analysis_overrides(&harness.model);
    let row = rows
        .iter()
        .find(|row| row.option == budget_label)
        .expect("an enabled operating point states the budget it will run");

    assert_eq!(row.preset, "50", "the plan policy it departs from");
    assert_eq!(
        row.effective,
        format!(
            "{} · Balanced",
            AnalysisAccuracy::Balanced.solver_policy().iteration_budget
        ),
        "the tier's budget is what the solve runs"
    );
    // Pinned verbatim: one sentence owns this wording, and the override gate
    // refuses ITL1 with the very same string.
    assert_eq!(
        row.origin,
        "its accuracy tier owns the Newton budget and is applied after the deck's options"
    );
    assert_eq!(
        row.origin,
        NumericOverrideOption::ACCURACY_TIER_OWNS_ITERATIONS
    );

    // The plan-level ITL1 row is still stated, honestly, for the DC solves no
    // tier touches — and it no longer claims the operating point.
    let plan_rows = super::page_solver::plan_policy_rows(&harness.model);
    let plan_row = plan_rows
        .iter()
        .find(|row| row.option == budget_label)
        .expect("ITL1 remains a plan-level policy");
    assert_eq!(plan_row.analysis, "DC · untiered solves");
    assert_eq!(plan_row.effective, "50");
    assert_eq!(plan_row.origin, "plan preset");
    assert!(
        !plan_rows
            .iter()
            .any(|row| row.analysis.contains("operating point")),
        "no plan-level row may claim the operating point resolves to a plan value"
    );
}

/// The transfer function resolves the same tier through the same
/// `AccuracyPolicy::apply`, so its Newton budget is owned exactly as the
/// operating point's is. The ledger must not acquire a row claiming otherwise,
/// and the override gate must not offer ITL1 for it.
#[test]
fn the_transfer_functions_newton_budget_is_never_claimed_for_the_plan() {
    use crate::simulation::plan::{AnalysisKind, NumericOverrideOption};

    let mut harness = SimulateTestHost::harness();
    harness.model.sim_setup.options.itl1 = 50;
    super::lifecycle::insert_analysis_instance(&mut harness.cx(), AnalysisKind::TransferFunction)
        .expect("a transfer function inserts into the test instance plan");

    let budget_label = NumericOverrideOption::Itl1.label();
    let rows = super::page_solver::analysis_overrides(&harness.model);
    let row = rows
        .iter()
        .find(|row| row.option == budget_label)
        .expect("an enabled transfer function states the budget it will run");
    assert_ne!(
        row.effective, "50",
        "the plan's ITL1 is overwritten before the first Newton step"
    );
    assert_eq!(
        row.origin,
        NumericOverrideOption::ACCURACY_TIER_OWNS_ITERATIONS
    );

    assert_eq!(
        NumericOverrideOption::Itl1.refusal_for(AnalysisKind::TransferFunction),
        Some(NumericOverrideOption::ACCURACY_TIER_OWNS_ITERATIONS),
        "an ITL1 authored against a tiered analysis would be stored and then ignored"
    );
    assert!(
        !NumericOverrideOption::applicable_to_instance(
            AnalysisKind::TransferFunction,
            crate::simulation::plan::SolverOwnership::NONE
        )
        .contains(&NumericOverrideOption::Itl1)
    );
}

/// A step ceiling is the one option an analysis cannot replace, only tighten:
/// the plan's ceiling and the analysis's own reach the engine as two separate
/// fields and the transient clamps against both. So a looser override does not
/// bind, and the ledger has to report the bound the run will actually honour
/// rather than the one that was typed.
#[test]
fn a_step_ceiling_looser_than_the_plans_reports_the_bound_the_run_honours() {
    use crate::simulation::plan::AnalysisDraft;

    fn ceiling_row(harness: &FeatureHarness<SimulateFeature>) -> (String, String, &'static str) {
        let rows = super::page_solver::analysis_overrides(&harness.model);
        let row = rows
            .iter()
            .find(|row| row.option == "Step ceiling")
            .expect("an authored step ceiling earns a ledger row");
        (row.preset.clone(), row.effective.clone(), row.origin)
    }

    fn author_ceiling(harness: &mut FeatureHarness<SimulateFeature>, authored: &str) {
        let plan = harness
            .model
            .sim_setup
            .stable_analysis_plan_mut()
            .expect("the test instance has a stable plan");
        let id = plan
            .instances()
            .iter()
            .find(|instance| matches!(instance.draft(), AnalysisDraft::Transient(_)))
            .map(|instance| instance.id())
            .expect("the default plan runs a transient");
        let _ = plan.edit(id, |draft| {
            if let AnalysisDraft::Transient(setup) = draft {
                setup.max_step = authored.to_owned();
            }
        });
    }

    let mut harness = SimulateTestHost::harness();
    // The plan's own ceiling, so the two bounds can be compared against a
    // stated value rather than against whatever the default happens to be.
    harness.model.sim_setup.options.max_timestep = 1.0e-6;

    author_ceiling(&mut harness, "5n");
    let (preset, effective, origin) = ceiling_row(&harness);
    assert_eq!(preset, "1u");
    assert_eq!(
        effective, "5n",
        "a tighter override is the bound that binds"
    );
    assert_eq!(origin, "analysis override");

    author_ceiling(&mut harness, "10u");
    let (_, effective, origin) = ceiling_row(&harness);
    assert_eq!(
        effective, "1u",
        "the run steps at the plan's ceiling, so the ledger may not report 10u"
    );
    assert_eq!(
        origin, "plan preset · tighter than the override",
        "and it has to say which of the two bounds won"
    );

    // `auto` is not "no ceiling", and it is not the plan's either: the bridge
    // writes no `.tran` max-step, the deck carries the analysis's own step
    // time in that field, and the engine mins the two. The ledger emitted no
    // row at all here, and the advanced-options panel printed the plan preset
    // — a millisecond, for a run that steps at ten nanoseconds.
    author_ceiling(&mut harness, "auto");
    let (preset, effective, origin) = ceiling_row(&harness);
    assert_eq!(preset, "1u");
    assert_eq!(
        effective, "10n",
        "an `auto` transient steps at its own step time where that is tighter"
    );
    assert_eq!(origin, "transient step time · no ceiling authored");

    // And the panel that reports the same option for the same analysis reads
    // the same owner rather than a second account of it.
    let draft = harness
        .model
        .sim_setup
        .stable_analysis_plan()
        .expect("the test instance has a stable plan")
        .instances()
        .iter()
        .find(|instance| matches!(instance.draft(), AnalysisDraft::Transient(_)))
        .map(|instance| instance.draft().clone())
        .expect("the default plan runs a transient");
    assert_eq!(
        super::advanced_options::refused_effective(
            crate::simulation::plan::NumericOverrideOption::MaximumTimestep,
            &draft,
            crate::simulation::plan::SolverOwnership::NONE,
            &harness.model.sim_setup.options,
        ),
        effective,
        "the Solver ledger and the advanced-options panel state one ceiling"
    );
}

/// The registry note claims a bound is refused wherever a variable is built or
/// read for a run. Each of those paths is checked here rather than trusted.
#[test]
fn a_bounded_variable_is_refused_on_every_path_a_test_can_reach() {
    use crate::state::{
        CellViewRef, DesignVariable, DesignVariableOverridePolicy, DesignVariableQuantity,
        DesignVariableRange, DesignVariableScope, DesignVariableSweepEligibility,
    };
    use rspice_simulation::netlist_gen::{
        DesignVariableNetlistContext, design_variable_parameter_lines,
    };

    let build = |expression: &str| {
        DesignVariable::new(
            "rload",
            expression,
            DesignVariableQuantity::Resistance,
            DesignVariableScope::Project,
            "load resistance",
            Some(DesignVariableRange {
                minimum: "100ohm".to_owned(),
                maximum: "10kohm".to_owned(),
            }),
            DesignVariableSweepEligibility::FixedParameter,
            DesignVariableOverridePolicy::InheritOwnerOnly,
        )
    };

    assert!(
        build("50kohm").is_err(),
        "construction refuses a value outside its own bound"
    );
    let mut variable = build("1kohm").expect("an in-range variable is accepted");

    // Forced past the constructor, exactly as a hand-edited project file would
    // arrive, so the remaining gates are the ones under test.
    variable.expression = "50kohm".to_owned();

    let cell = CellViewRef::default_top();
    assert!(
        design_variable_parameter_lines(
            std::slice::from_ref(&variable),
            DesignVariableNetlistContext {
                active_cell: &cell,
                analysis_instances: &[],
            },
        )
        .is_err(),
        "netlist generation refuses to emit a parameter line for it"
    );

    let mut harness = SimulateTestHost::harness();
    let id = plan_id(&harness.model);
    assert!(
        harness
            .model
            .workspace
            .content
            .validate_simulation_configuration()
            .is_ok(),
        "the seeded workspace is valid before the tampered record is planted"
    );
    harness
        .model
        .workspace
        .content
        .ensure_active_plan_data(id)
        .design_variables = vec![variable];
    assert!(
        harness
            .model
            .workspace
            .content
            .validate_simulation_configuration()
            .is_err(),
        "workspace validation refuses the stored configuration"
    );
}

/// The same rule for participation: absent means "every point", and an
/// untouched plan must serialize exactly as it did before the field existed.
///
/// A project written before participation ran every analysis at every point, so
/// a default of anything else would narrow a run nobody narrowed on the first
/// reload after upgrading.
#[test]
fn a_plan_written_before_run_set_participation_still_loads() {
    use crate::simulation::plan::{AnalysisDraft, AnalysisInstance, SimulationPlan};
    use rspice_simulation_contract::analysis_run_at::AnalysisRunAt;

    let mut plan = SimulationPlan::new();
    let serialized = serde_json::to_string(&plan).expect("the plan serializes");
    assert!(
        !serialized.contains("run_at"),
        "a plan that runs everywhere must serialize exactly as it did before the field existed"
    );
    let restored: SimulationPlan =
        serde_json::from_str(&serialized).expect("a plan without the field still loads");
    assert_eq!(restored.instances()[0].run_at(), &AnalysisRunAt::AllPoints);

    let id = plan
        .instances()
        .iter()
        .find(|instance| matches!(instance.draft(), AnalysisDraft::Transient(_)))
        .map(|instance| instance.id())
        .expect("a fresh plan holds one transient");
    let chosen = vec!["axis-a-value-1+axis-b-value-3".to_owned()];
    plan.set_run_at(id, AnalysisRunAt::SelectedPoints(chosen.clone()))
        .expect("the plan accepts a point selection");
    let round_tripped: SimulationPlan =
        serde_json::from_str(&serde_json::to_string(&plan).expect("serializes"))
            .expect("a scoped plan round trips");
    assert_eq!(
        round_tripped.instance(id).map(AnalysisInstance::run_at),
        Some(&AnalysisRunAt::SelectedPoints(chosen)),
        "a point selection survives the file by identity, not by position"
    );

    // A frozen projection is what preparation reads, so it has to carry the
    // participation too — a projection that widened it would dispatch points
    // the plan does not declare itself at.
    let frozen = plan.freeze().expect("the fixture plan freezes");
    assert_eq!(
        frozen
            .instances()
            .iter()
            .find(|instance| instance.id() == id)
            .map(crate::simulation::plan::FrozenAnalysisInstance::run_at),
        round_tripped.instance(id).map(AnalysisInstance::run_at),
    );
}
