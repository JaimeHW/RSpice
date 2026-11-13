//! Staging a placed instance's Value for the parameter tuner.
//!
//! Design's instance inspector offers Tune on a Value row; what Tune stages
//! is the tuner's: a candidate design variable for the row, or the variable
//! the row already names, and the binding the tuning review would commit. The
//! inspector raises the request and reads whether staging would be refused;
//! nothing here touches the schematic or the plan. It ranks below the slots,
//! so simulate's port answers the inspector from here; the Verify surface
//! states a refusal.

use crate::state::{CellViewRef, Component, ComponentType};
use crate::workbench::app_state::Model;
use crate::workbench::state::VerificationSessionState;

/// Why the tuner would refuse to stage `component`'s Value as `model` and
/// `session` stand, or `None` when it would stage it.
pub(in crate::workbench) fn component_tuning_block_reason(
    model: &Model,
    session: &VerificationSessionState,
    component: &Component,
) -> Option<String> {
    prepare_component_tuning(model, session, component).err()
}

fn tunable_value_quantity(kind: ComponentType) -> Option<crate::state::DesignVariableQuantity> {
    use crate::state::DesignVariableQuantity as Quantity;

    match kind {
        ComponentType::Resistor | ComponentType::Ccvs => Some(Quantity::Resistance),
        ComponentType::Capacitor => Some(Quantity::Capacitance),
        ComponentType::VoltageSource | ComponentType::VoltageSourceAc => Some(Quantity::Voltage),
        ComponentType::CurrentSource | ComponentType::CurrentSourceAc => Some(Quantity::Current),
        ComponentType::Vcvs | ComponentType::Cccs | ComponentType::OpAmp => {
            Some(Quantity::Dimensionless)
        }
        _ => None,
    }
}

/// Component value fields carry their quantity through the owning device, while
/// design variables are deliberately self-describing. Preserve an explicit
/// unit when one is already present and otherwise add the unit implied by the
/// selected component before constructing the typed variable.
fn typed_tuning_expression(value: &str, quantity: crate::state::DesignVariableQuantity) -> String {
    use crate::state::DesignVariableQuantity as Quantity;

    let value = value.trim();
    let has_ascii_suffix = |suffix: &str| {
        value
            .get(value.len().saturating_sub(suffix.len())..)
            .is_some_and(|candidate| candidate.eq_ignore_ascii_case(suffix))
    };
    match quantity {
        Quantity::Resistance if has_ascii_suffix("ohm") || value.ends_with('Ω') => {
            value.to_owned()
        }
        Quantity::Resistance => format!("{value} ohm"),
        Quantity::Capacitance if value.ends_with('F') => value.to_owned(),
        Quantity::Capacitance => format!("{value}F"),
        Quantity::Voltage if value.ends_with('V') => value.to_owned(),
        Quantity::Voltage => format!("{value}V"),
        Quantity::Current if value.ends_with('A') => value.to_owned(),
        Quantity::Current => format!("{value}A"),
        Quantity::Temperature | Quantity::Dimensionless => value.to_owned(),
    }
}

fn is_parameter_identifier(value: &str) -> bool {
    let mut characters = value.chars();
    let Some(first) = characters.next() else {
        return false;
    };
    (first.is_ascii_alphabetic() || first == '_')
        && characters.all(|character| character.is_ascii_alphanumeric() || character == '_')
}

fn simple_parameter_reference(value: &str) -> Option<&str> {
    let value = value.trim();
    let candidate = value
        .strip_prefix('{')
        .and_then(|body| body.strip_suffix('}'))
        .map(str::trim)
        .unwrap_or(value);
    is_parameter_identifier(candidate).then_some(candidate)
}

fn proposed_tuning_variable_name(
    component: &Component,
    variables: &[crate::state::DesignVariable],
) -> String {
    let mut stem = component
        .name
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || character == '_' {
                character.to_ascii_uppercase()
            } else {
                '_'
            }
        })
        .collect::<String>();
    if !stem
        .chars()
        .next()
        .is_some_and(|character| character.is_ascii_alphabetic() || character == '_')
    {
        stem.insert_str(0, "P_");
    }
    stem.truncate(116);
    let base = format!("{stem}_VALUE");
    if !variables
        .iter()
        .any(|variable| variable.name.eq_ignore_ascii_case(&base))
    {
        return base;
    }
    for suffix in 2_u32.. {
        let suffix = format!("_{suffix}");
        let keep = 128_usize.saturating_sub(suffix.len());
        let mut candidate = base.clone();
        candidate.truncate(keep);
        candidate.push_str(&suffix);
        if !variables
            .iter()
            .any(|variable| variable.name.eq_ignore_ascii_case(&candidate))
        {
            return candidate;
        }
    }
    unreachable!("the finite variable set cannot exhaust every numeric suffix")
}

enum ComponentTuningPreparation {
    ExistingBinding {
        variable_id: crate::product::DesignVariableId,
    },
    StageBinding {
        /// Boxed: a staged variable is an order of magnitude larger than the
        /// existing-binding case's identifier.
        variable: Box<crate::state::DesignVariable>,
        creates_variable: bool,
    },
}

struct PreparedComponentTuning {
    plan_id: crate::product::SimulationPlanId,
    plan_revision: crate::product::ObjectRevision,
    variables: Vec<crate::state::DesignVariable>,
    source_view: CellViewRef,
    source_topology_version: u64,
    active_plan_run: Option<crate::product::RunId>,
    preparation: ComponentTuningPreparation,
}

/// Resolve the exact Tune transaction without mutating the sandbox or either
/// authoritative document. The inspector button and the click handler both
/// use this preflight so a control cannot advertise a workflow that staging
/// will later reject.
fn prepare_component_tuning(
    model: &Model,
    session: &VerificationSessionState,
    component: &Component,
) -> Result<PreparedComponentTuning, String> {
    if model.schematic_edit_read_only() {
        return Err("the active schematic is read-only".to_owned());
    }
    let quantity = tunable_value_quantity(component.kind).ok_or_else(|| {
        format!(
            "{} values do not have a truthful typed design-variable mapping",
            component.kind.display_name()
        )
    })?;
    let (plan_id, plan_revision) = model
        .sim_setup
        .stable_analysis_plan()
        .map(|plan| (plan.id(), plan.revision()))
        .map_err(|error| format!("the active simulation plan is unavailable: {error}"))?;
    let variables = model
        .workspace
        .content
        .plan_data(plan_id)
        .map(|payload| payload.design_variables.clone())
        .ok_or_else(|| "the active simulation plan has no configuration payload".to_owned())?;
    let source_view = model.workspace.content.active_schematic_reference();
    let source_topology_version = model.schematic.topology_version();
    let active_plan_run = model
        .simulation
        .active_run()
        .filter(|run| {
            run.prepared_receipt()
                .and_then(|receipt| receipt.simulation_plan_id())
                == Some(plan_id)
        })
        .map(|run| run.run_id);

    let current_session = session.tuning_plan_id == Some(plan_id)
        && session.tuning_plan_revision == Some(plan_revision);
    if current_session && let Some(pending) = session.tuning_instance_binding.as_ref() {
        if pending.component_id == component.id
            && pending.source_view == source_view
            && pending.source_topology_version == source_topology_version
            && pending.source_value == component.value
        {
            return Ok(PreparedComponentTuning {
                plan_id,
                plan_revision,
                variables,
                source_view,
                source_topology_version,
                active_plan_run,
                preparation: ComponentTuningPreparation::ExistingBinding {
                    variable_id: pending.variable.id,
                },
            });
        }
        if pending.creates_variable || pending.requires_schematic_edit() {
            return Err(format!(
                "{} already has an uncommitted Value binding; commit or revert it before tuning another instance",
                pending.component_name
            ));
        }
    }

    let (variable, creates_variable) = if let Some(reference) =
        simple_parameter_reference(&component.value)
    {
        let variable = variables
            .iter()
            .find(|variable| variable.name.eq_ignore_ascii_case(reference))
            .cloned()
            .ok_or_else(|| {
                format!(
                    "the Value row references parameter '{reference}', but the active plan does not define it"
                )
            })?;
        if variable.quantity != quantity {
            return Err(format!(
                "parameter '{}' is typed as {}, but {} requires {}",
                variable.name,
                variable.quantity.label(),
                component.kind.display_name(),
                quantity.label()
            ));
        }
        (variable, false)
    } else {
        let name = proposed_tuning_variable_name(component, &variables);
        let typed_expression = typed_tuning_expression(&component.value, quantity);
        let variable = crate::state::DesignVariable::new(
            &name,
            &typed_expression,
            quantity,
            // Generated run decks emit project/testbench parameters at
            // the configured root. The explicit instance binding remains
            // the sole consumer, while project scope keeps a child-cell
            // edit executable from its parent simulation root.
            crate::state::DesignVariableScope::Project,
            format!(
                "Value of {} in {}",
                component.name,
                source_view.display_path()
            ),
            None,
            crate::state::DesignVariableSweepEligibility::NestedSweepAndOptimization,
            crate::state::DesignVariableOverridePolicy::ExplicitTestLocalOverride,
        )
        .map_err(|error| {
            format!(
                "the current Value '{}' cannot become a typed {} variable: {error}",
                component.value,
                quantity.label()
            )
        })?;
        (variable, true)
    };

    Ok(PreparedComponentTuning {
        plan_id,
        plan_revision,
        variables,
        source_view,
        source_topology_version,
        active_plan_run,
        preparation: ComponentTuningPreparation::StageBinding {
            variable: Box::new(variable),
            creates_variable,
        },
    })
}

/// Bind the selected Value row to the non-destructive parameter sandbox.
///
/// This function writes only runtime proposal state. The schematic and active
/// plan remain byte-for-byte authoritative until the existing review dialog
/// commits the complete transaction.
pub fn stage_component_tuning(
    model: &Model,
    session: &mut VerificationSessionState,
    component_id: u64,
) -> Result<(), String> {
    let component = model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == component_id)
        .cloned()
        .ok_or_else(|| "the selected instance no longer exists".to_owned())?;
    let prepared = prepare_component_tuning(model, session, &component)?;

    if session.tuning_plan_id != Some(prepared.plan_id)
        || session.tuning_plan_revision != Some(prepared.plan_revision)
    {
        session.tuning_plan_id = Some(prepared.plan_id);
        session.tuning_plan_revision = Some(prepared.plan_revision);
        session.tuning_variables = prepared
            .variables
            .iter()
            .map(|variable| crate::workbench::state::TuningVariableDraft {
                variable_id: variable.id,
                baseline_expression: variable.expression.clone(),
                candidate_expression: variable.expression.clone(),
                validation_error: None,
                proposed: false,
            })
            .collect();
        session.tuning_instance_binding = None;
        session.tuning_selected_variable = None;
        session.tuning_focus_variable = None;
        session.tuning_baseline_run = prepared.active_plan_run;
        session.tuning_review_open = false;
    }

    let (variable, creates_variable) = match prepared.preparation {
        ComponentTuningPreparation::ExistingBinding { variable_id } => {
            session.tuning_selected_variable = Some(variable_id);
            session.tuning_focus_variable = Some(variable_id);
            return Ok(());
        }
        ComponentTuningPreparation::StageBinding {
            variable,
            creates_variable,
        } => (*variable, creates_variable),
    };
    if creates_variable {
        session.tuning_variables.retain(|draft| !draft.proposed);
        session
            .tuning_variables
            .push(crate::workbench::state::TuningVariableDraft {
                variable_id: variable.id,
                baseline_expression: variable.expression.clone(),
                candidate_expression: variable.expression.clone(),
                validation_error: None,
                proposed: true,
            });
    }

    let binding_expression = format!("{{{}}}", variable.name);
    session.tuning_instance_binding = Some(crate::workbench::state::TuningInstanceBindingDraft {
        component_id,
        component_name: component.name.clone(),
        source_view: prepared.source_view,
        source_topology_version: prepared.source_topology_version,
        source_value: component.value.clone(),
        binding_expression,
        variable: variable.clone(),
        creates_variable,
    });
    session.tuning_selected_variable = Some(variable.id);
    session.tuning_focus_variable = Some(variable.id);
    session.action_problem = None;
    Ok(())
}
