//! The engine's own structural refusals, stated against the drawing.
//!
//! Once a run is prepared, every deck it dispatches is built the way the
//! runner builds it, and what the engine refuses there is a blocker
//! (`PreparedRunSnapshot::engine_refusals`). That replaced two checks this
//! module used to make from drawing connectivity with a DC-conduction model of
//! its own: they refused coupling capacitors the engine solves, ignored
//! `.OPTIONS RSHUNT`, and restated the voltage-source loop the design rules
//! already own. A refusal stated here is now one the engine makes.
//!
//! The engine names deck nodes and elements. Each is read back to the object
//! drawn in the occurrence it belongs to, through the configured hierarchy the
//! deck was written from, so a finding opens that occurrence's cell view with
//! the object selected. A name with no drawn object — a node inside a library
//! model, an element the deck adds — is located in the deck instead.

use std::sync::Arc;

use rspice_app_types::hierarchy_path::{InstancePath, ProbeTarget};
use rspice_design::connectivity::summary::DesignNet;
use rspice_simulation::execution::{EngineRefusal, EngineRefusalKind, RefusedTask};

use crate::state::{CellViewRef, Component, ViewType};
use crate::workbench::app_state::AppState;
use crate::workbench::state::{DrawnObject, PreflightIssue, PreflightRemediation};

/// Row labels the Solver surface states as enforced contract. The refusal and
/// the statement of the rule use one vocabulary.
const NO_DC_PATH_CHECK: &str = "Nodes without a DC path";
const LOOP_CHECK: &str = "Voltage-source loops";
const BIN_CHECK: &str = "Model-bin coverage";
const BUILD_CHECK: &str = "Engine elaboration";

/// How many refused tasks a finding names before it counts the rest.
const QUOTED_TASK_LIMIT: usize = 4;

/// One blocker per object each refusal names.
pub(super) fn engine_refusal_issues(
    state: &AppState,
    refusals: &[EngineRefusal],
) -> Vec<PreflightIssue> {
    if refusals.is_empty() {
        return Vec::new();
    }
    let drawing = DrawnDesign::configured(state);
    refusals
        .iter()
        .flat_map(|refusal| issues_for(refusal, &drawing))
        .collect()
}

fn issues_for(refusal: &EngineRefusal, drawing: &DrawnDesign) -> Vec<PreflightIssue> {
    let refused_by = refused_by(&refusal.tasks);
    match &refusal.kind {
        EngineRefusalKind::NoDcPath { nodes } => nodes
            .iter()
            .map(|node| {
                let (named, remediation) = drawing.node(node);
                PreflightIssue {
                    check: NO_DC_PATH_CHECK.to_owned(),
                    observed: format!(
                        "No DC path to ground from node {named}, and a source forces current \
                         into it: capacitors and current sources do not conduct at DC, so \
                         nothing sets its operating-point voltage. Refused by {refused_by}."
                    ),
                    required: "A conducting path from the node to ground, or .OPTIONS \
                               RSHUNT=<ohms> in the deck to shunt every node"
                        .to_owned(),
                    remediation,
                }
            })
            .collect(),
        EngineRefusalKind::BranchVoltageLoop(branch_loop) => {
            let members: Vec<String> = branch_loop
                .members
                .iter()
                .map(|member| drawing.element(member).0)
                .collect();
            // The element that closed the loop is where it is shown.
            let closing = branch_loop
                .members
                .last()
                .map_or(PreflightRemediation::NetlistSource { line: None }, |member| {
                    drawing.element(member).1
                });
            let (what, why) = if branch_loop.through_inductor {
                (
                    "Voltage sources and inductors",
                    "the DC solve shorts each inductor, so the loop's current is undetermined",
                )
            } else {
                (
                    "Voltage sources",
                    "the loop states two voltages for one node pair",
                )
            };
            vec![PreflightIssue {
                check: LOOP_CHECK.to_owned(),
                observed: format!(
                    "{what} {} form a closed loop: {why}. Refused by {refused_by}.",
                    members.join(", ")
                ),
                required: "A resistive element inside the loop".to_owned(),
                remediation: closing,
            }]
        }
        EngineRefusalKind::UncoveredModelBin(gap) => {
            let (named, remediation) = drawing.element(&gap.element);
            let nearest = &gap.nearest.geometry;
            let mut evaluated = vec![
                format!("L={}", length(gap.length)),
                format!("W={}", length(gap.width)),
            ];
            let mut covered = vec![
                format!("L {}", span(nearest.length.min, nearest.length.max)),
                format!("W {}", span(nearest.width.min, nearest.width.max)),
            ];
            if gap.nfin.is_some() || nearest.nfin.min.is_some() || nearest.nfin.max.is_some() {
                evaluated.push(format!("NFIN={}", count(gap.nfin)));
                covered.push(format!(
                    "NFIN {}–{}",
                    count(nearest.nfin.min),
                    count(nearest.nfin.max)
                ));
            }
            vec![PreflightIssue {
                check: BIN_CHECK.to_owned(),
                observed: format!(
                    "{named} at {} falls in no bin of model {}; the nearest bin, {}, covers \
                     {}. Refused by {refused_by}.",
                    evaluated.join(", "),
                    gap.family,
                    gap.nearest.model,
                    covered.join(", ")
                ),
                required: "Device geometry inside a bin of its model at every dispatched point"
                    .to_owned(),
                remediation,
            }]
        }
        EngineRefusalKind::Build { message, instance } => {
            let remediation = instance.as_deref().map_or(
                PreflightRemediation::NetlistSource { line: None },
                |instance| drawing.element(instance).1,
            );
            vec![PreflightIssue {
                check: BUILD_CHECK.to_owned(),
                observed: format!(
                    "{}. Refused by {refused_by}.",
                    message.trim_end_matches('.')
                ),
                required: "A deck the engine can elaborate at every dispatched point".to_owned(),
                remediation,
            }]
        }
    }
}

/// The refused tasks, as the run labels them, quoted up to the limit.
fn refused_by(tasks: &[RefusedTask]) -> String {
    let quoted = tasks
        .iter()
        .take(QUOTED_TASK_LIMIT)
        .map(|task| task.label.as_str())
        .collect::<Vec<_>>()
        .join("; ");
    match tasks.len().saturating_sub(QUOTED_TASK_LIMIT) {
        0 => quoted,
        remaining => format!("{quoted} (and {remaining} more)"),
    }
}

/// A length in micrometres, the unit model bins are authored in.
fn length(metres: Option<f64>) -> String {
    metres.map_or_else(
        || "unset".to_owned(),
        |metres| format!("{} µm", significant(metres * 1e6)),
    )
}

fn span(min: Option<f64>, max: Option<f64>) -> String {
    let bound = |metres: Option<f64>| {
        metres.map_or_else(|| "any".to_owned(), |metres| significant(metres * 1e6))
    };
    format!("{}–{} µm", bound(min), bound(max))
}

fn count(value: Option<f64>) -> String {
    value.map_or_else(|| "unset".to_owned(), significant)
}

/// Four significant figures, without trailing zeros.
fn significant(value: f64) -> String {
    let text = format!("{value:.4}");
    let text = text.trim_end_matches('0').trim_end_matches('.');
    if text.is_empty() || text == "-" {
        "0".to_owned()
    } else {
        text.to_owned()
    }
}

/// One occurrence of the configured design: where it sits, the cell view that
/// draws it, and what that view draws.
struct DrawnOccurrence {
    path: InstancePath,
    cell_view: CellViewRef,
    components: Vec<Component>,
    nets: Arc<Vec<DesignNet>>,
}

/// The configured design as the deck was written from it.
struct DrawnDesign {
    occurrences: Vec<DrawnOccurrence>,
}

impl DrawnDesign {
    /// Every occurrence of the configured root that draws a schematic. Empty
    /// when the configuration no longer projects, in which case every name is
    /// located in the deck.
    fn configured(state: &AppState) -> Self {
        let Ok(projection) = state.workspace.configuration_execution_projection(
            &state.library_manager,
            &state.workspace.content.active_view,
            &state.schematic,
        ) else {
            return Self {
                occurrences: Vec::new(),
            };
        };
        let catalog = state.library_manager.catalog();
        let schematic_of = |cell_view: &CellViewRef| {
            projection
                .schematic_buffers()
                .iter()
                .find(|(key, _)| key.eq_ignore_ascii_case(&cell_view.key()))
                .map(|(_, schematic)| schematic.document().components.clone())
        };
        let drawn = |path: InstancePath, cell_view: &CellViewRef| {
            Some(DrawnOccurrence {
                components: schematic_of(cell_view)?,
                nets: rspice_design::connectivity::summary::projection_nets(
                    catalog,
                    &projection,
                    &cell_view.key(),
                ),
                cell_view: cell_view.clone(),
                path,
            })
        };
        let root = drawn(InstancePath::root(), projection.root());
        let below = projection
            .plan()
            .bindings()
            .filter(|binding| {
                !binding.instance_path().is_root()
                    && matches!(
                        binding.resolved_view_type(),
                        ViewType::Schematic | ViewType::Testbench
                    )
            })
            .filter_map(|binding| {
                drawn(
                    binding.instance_path().clone(),
                    binding.resolved_reference(),
                )
            });
        Self {
            occurrences: root.into_iter().chain(below).collect(),
        }
    }

    /// The occurrence an engine name belongs to, and the name below it.
    fn occurrence(&self, engine_name: &str) -> Option<(&DrawnOccurrence, String)> {
        let target = ProbeTarget::parse(engine_name.trim()).ok()?;
        self.occurrences
            .iter()
            .find(|occurrence| occurrence.path.same_engine_scope(&target.scope))
            .map(|occurrence| (occurrence, target.leaf))
    }

    /// How a finding names a deck node, and where it is shown.
    fn node(&self, engine_name: &str) -> (String, PreflightRemediation) {
        let drawn = self.occurrence(engine_name).and_then(|(occurrence, leaf)| {
            let net = occurrence
                .nets
                .iter()
                .find(|net| net.name.eq_ignore_ascii_case(&leaf))?;
            Some((
                quoted_in(&net.name, occurrence),
                PreflightRemediation::Drawing {
                    cell_view: occurrence.cell_view.clone(),
                    object: DrawnObject::Net {
                        name: net.name.clone(),
                        wire: net.wire_ids.first().copied(),
                        component: net.terminals.first().map(|terminal| terminal.component_id),
                    },
                },
            ))
        });
        drawn.unwrap_or_else(|| {
            (
                format!("'{engine_name}'"),
                PreflightRemediation::NetlistSource { line: None },
            )
        })
    }

    /// How a finding names a deck element, and where it is shown.
    fn element(&self, engine_name: &str) -> (String, PreflightRemediation) {
        let drawn = self.occurrence(engine_name).and_then(|(occurrence, _)| {
            let component = rspice_design::schematic::component::component_for_engine_device(
                &occurrence.components,
                &occurrence.path,
                engine_name,
            )?;
            let name = if component.name.trim().is_empty() {
                format!("component #{}", component.id)
            } else {
                component.name.clone()
            };
            Some((
                quoted_in(&name, occurrence),
                PreflightRemediation::Drawing {
                    cell_view: occurrence.cell_view.clone(),
                    object: DrawnObject::Component {
                        id: component.id,
                        name,
                    },
                },
            ))
        });
        drawn.unwrap_or_else(|| {
            (
                format!("'{engine_name}'"),
                PreflightRemediation::NetlistSource { line: None },
            )
        })
    }
}

/// A drawn name, with the occurrence it is drawn in when that is not the
/// configured root.
fn quoted_in(name: &str, occurrence: &DrawnOccurrence) -> String {
    if occurrence.path.is_root() {
        format!("'{name}'")
    } else {
        format!("'{name}' in {}", occurrence.path)
    }
}
