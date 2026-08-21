//! The two analysis forms that read the plan's declared run space.
//!
//! Corner and Temperature are the only analyses whose subject *is* the run
//! space, so they are the only two that have to state it. Neither owns it:
//! PVT, sweeps & variation declares the space once, and these forms report what
//! that declaration resolves to and route to it for editing.
//!
//! Every row here is backed by state something else already acts on. A form
//! that stated a nominal point, a concurrency or a failure policy of its own
//! invention would read exactly like one that worked, which is the defect the
//! projection ratchet exists to catch and the reason no field below is allowed
//! a literal.

use egui::Ui;

use crate::simulation::run_set::{
    ReferencePoint, RunSetDimensionKind, RunSetState, nominal_point_key, resolve,
};
use crate::state::NominalFailurePolicy;
use crate::workbench::state::SimulationPage;

use super::{action_line, choice_row, property_row, sub_header};

/// The plan facts the run-space forms read, resolved once by the caller.
///
/// Borrowed rather than copied: these are the plan's own values, and a form
/// holding its own duplicate of a run space is the ownership split this module
/// was written to end.
pub(in crate::workbench::surfaces::simulate) struct RunSpaceContext<'a> {
    /// The plan's one declared run space.
    pub(in crate::workbench::surfaces::simulate) run_set: &'a RunSetState,
    /// The point every undeclared axis resolves through.
    pub(in crate::workbench::surfaces::simulate) reference: ReferencePoint,
    /// What a nominal-point specification failure does to plan acceptance.
    pub(in crate::workbench::surfaces::simulate) nominal_failure: NominalFailurePolicy,
    /// How many content-pinned model libraries this plan binds.
    pub(in crate::workbench::surfaces::simulate) model_binding_count: usize,
    /// The execution target's name and how many tasks it runs at once.
    pub(in crate::workbench::surfaces::simulate) parallelism: (&'static str, u64),
}

impl RunSpaceContext<'_> {
    /// The declared point the plan calls nominal, as the point table labels it.
    ///
    /// `None` has a specific meaning that the caller must not paper over: the
    /// declared space contains no point sitting on the reference condition, and
    /// a nominal-only instance refuses rather than picking one. The refusal is
    /// [`crate::simulation::run_set::participating_point_keys`]'s; this only
    /// reports the same fact before a run is attempted.
    fn nominal_point_label(&self) -> Option<String> {
        let points = resolve(self.run_set)?;
        let key = nominal_point_key(&points, self.reference)?;
        points
            .iter()
            .find(|point| point.point_key() == key)
            .map(|point| point.label())
    }

    /// Where the process axis's values are authored, and how many it declares.
    ///
    /// The process axis is the one that decides which model sections are
    /// materialized, so "which corner set is in force" is exactly this axis's
    /// source authority read together with the plan's model bindings.
    fn imported_corner_set(&self) -> String {
        let bindings = match self.model_binding_count {
            0 => "no model binding".to_owned(),
            1 => "1 model binding".to_owned(),
            count => format!("{count} model bindings"),
        };
        // Declared and enabled are different facts and the row states which.
        // Reporting a declared-but-disabled axis as "no process axis declared"
        // would be false on a form whose whole job is to state the plan's
        // declaration accurately.
        match self
            .run_set
            .dimensions
            .iter()
            .find(|dimension| dimension.kind == RunSetDimensionKind::ProcessSection)
        {
            Some(dimension) if dimension.enabled => format!(
                "{} sections · {} · {bindings}",
                dimension.values.len(),
                dimension.source,
            ),
            Some(dimension) => format!(
                "{} sections declared, axis disabled · {} · {bindings}",
                dimension.values.len(),
                dimension.source,
            ),
            None => format!("no process axis declared · {bindings}"),
        }
    }

    fn parallelism_label(&self) -> String {
        let (target, at_once) = self.parallelism;
        match at_once {
            1 => format!("1 task at a time · {target}"),
            many => format!("{many} tasks at a time · {target}"),
        }
    }

    fn nominal_failure_label(&self) -> &'static str {
        match self.nominal_failure {
            NominalFailurePolicy::Block => "Nominal failure blocks plan acceptance",
            NominalFailurePolicy::RecordDisposition => {
                "Nominal failure records a review disposition"
            }
        }
    }
}

/// Draw the temperature sweep as a reader of the plan's axis, with the switch
/// that lets this instance own its temperatures instead. Returns the note.
pub(super) fn temperature_form(
    ui: &mut Ui,
    setup: &mut crate::simulation::dialog::temp::TempDialogState,
    context: &RunSpaceContext<'_>,
    route: &mut Option<SimulationPage>,
    policy: crate::quantity::QuantityPresentationPolicy,
    locale: crate::quantity::UiNumberLocale,
) -> &'static str {
    use crate::simulation::dialog::temp::TempAxisMode;

    let labels: Vec<&str> = TempAxisMode::ALL
        .iter()
        .map(|mode| mode.display_name())
        .collect();
    choice_row(ui, "Temperatures", &labels, &mut setup.axis_mode_idx);

    // What the plan declares, stated next to the choice, so the reader can see
    // what inheriting would give them and what overriding is departing from.
    let declared = context
        .run_set
        .declared_temperatures_celsius(context.reference);
    property_row(ui, "Run-set axis", &axis_state(context, setup, &declared));

    match setup.axis_mode() {
        TempAxisMode::InheritRunSetAxis => {
            if action_line(ui, "Edit in PVT, sweeps & variation") {
                *route = Some(SimulationPage::RunSet);
            }
            super::choice_row(ui, "Base", &["op", "tran", "ac", "dc"], &mut setup.base_idx);
            "Repeats the base analysis across the temperatures the plan declares. Inheriting \
             authors those temperatures once; it does not move who walks them, so this instance \
             still owns a point expansion and the plan is refused while any global run-set axis \
             is enabled. Edit the temperatures in PVT, sweeps & variation."
        }
        TempAxisMode::Explicit => {
            super::quantity_input_row(
                ui,
                "Start",
                &mut setup.temp_start,
                crate::quantity::QuantityInputKind::Temperature,
                policy,
                locale,
            );
            super::quantity_input_row(
                ui,
                "Stop",
                &mut setup.temp_stop,
                crate::quantity::QuantityInputKind::Temperature,
                policy,
                locale,
            );
            super::quantity_input_row(
                ui,
                "Step",
                &mut setup.temp_step,
                crate::quantity::QuantityInputKind::TemperatureDelta,
                policy,
                locale,
            );
            super::choice_row(ui, "Base", &["op", "tran", "ac", "dc"], &mut setup.base_idx);
            super::input_row(ui, "Explicit list", &mut setup.specific_temps);
            "Repeats the base analysis across temperature. An explicit list replaces the range \
             above; leave it empty to sweep the range. This instance owns a point expansion \
             either way, so the plan is refused while any global run-set axis is enabled."
        }
    }
}

/// The one line that says which axis is in force, and what it costs to differ.
///
/// In override mode this is the delta the brief calls for: both counts, so a
/// reader can see at a glance that this instance is running a different number
/// of temperatures from the one the plan declares.
fn axis_state(
    context: &RunSpaceContext<'_>,
    setup: &crate::simulation::dialog::temp::TempDialogState,
    declared: &Option<Vec<f64>>,
) -> String {
    use crate::simulation::dialog::temp::TempAxisMode;

    let Some(declared) = declared else {
        return "declared axis has a value that is not a temperature".to_owned();
    };
    let plan_declares_axis = context.run_set.dimensions.iter().any(|dimension| {
        dimension.kind == RunSetDimensionKind::Temperature && !dimension.values.is_empty()
    });
    let stated = declared
        .iter()
        .map(|celsius| format!("{celsius}"))
        .collect::<Vec<_>>()
        .join(", ");

    match setup.axis_mode() {
        TempAxisMode::InheritRunSetAxis if plan_declares_axis => {
            format!("inherits · {stated} °C")
        }
        TempAxisMode::InheritRunSetAxis => {
            format!("no axis declared · runs at the reference, {stated} °C")
        }
        TempAxisMode::Explicit if plan_declares_axis => {
            let here = setup
                .to_config(context.run_set, context.reference)
                .map_or_else(
                    |_| "an invalid sweep".to_owned(),
                    |config| format!("{} here", config.num_temps()),
                );
            format!(
                "overrides run-set axis · {here} vs {} declared ({stated} °C)",
                declared.len(),
            )
        }
        TempAxisMode::Explicit => "no axis declared · this instance owns its sweep".to_owned(),
    }
}

/// Draw the declared space, the conditions it runs under, and the route to its
/// editor. Returns the form's note.
pub(super) fn corner_form(
    ui: &mut Ui,
    base_analysis_idx: &mut usize,
    context: &RunSpaceContext<'_>,
    route: &mut Option<SimulationPage>,
) -> &'static str {
    sub_header(ui, "Run space");
    for dimension in context.run_set.dimensions.iter() {
        let values = if dimension.values.is_empty() {
            "no values".to_owned()
        } else {
            dimension
                .values
                .iter()
                .map(|value| value.lexical.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        };
        property_row(
            ui,
            &dimension.name,
            &if dimension.enabled {
                values
            } else {
                format!("{values} · disabled")
            },
        );
    }
    property_row(ui, "Composition", context.run_set.composition.mode.label());
    property_row(
        ui,
        "Resolved points",
        &context.run_set.point_count().to_string(),
    );
    property_row(
        ui,
        "Nominal point",
        &context.nominal_point_label().unwrap_or_else(|| {
            format!(
                "none · no declared point sits on {} · {} °C",
                context.reference.process.short_name(),
                context.reference.temperature_celsius,
            )
        }),
    );
    if action_line(ui, "Edit in PVT, sweeps & variation") {
        *route = Some(SimulationPage::RunSet);
    }

    sub_header(ui, "Conditions");
    property_row(ui, "Corner set", &context.imported_corner_set());
    property_row(ui, "Parallelism", &context.parallelism_label());
    property_row(ui, "Failure policy", context.nominal_failure_label());

    sub_header(ui, "At every point");
    choice_row(ui, "Base", &["tran", "ac", "dc", "op"], base_analysis_idx);
    "Repeats the base analysis at every point of the plan's declared run space."
}
