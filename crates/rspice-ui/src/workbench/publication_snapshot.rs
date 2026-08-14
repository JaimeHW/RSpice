//! Publication snapshot builder.
//!
//! Resolves the open project into the sealed interchange format defined by
//! `rspice-publication-contract`: every printable schematic sheet and result
//! plot becomes a resolved display-list scene via the same semantic-resolution
//! and scene-compilation pipeline hardcopy uses, and the active run's
//! datasets, measurements, and deck become exact typed records. The builder
//! never renders, never consults a clock, and never invents content — what it
//! cannot faithfully capture it reports as an error instead of approximating.

use rspice_publication_contract::{
    AnalysisRecord, AxisScale, ComponentPin, ComponentRecord, Dataset, Disclosure,
    EngineeringPublication, Figure, FigureContent, Measurement, ModelReference, NetConnection,
    NetRecord, NetlistSection, Paint, PaintRole, PathPrimitive, PathSegment, PlotFigure,
    PlotHydration, PlotTraceBinding, Point, Primitive, PrimitiveGroup, PublicationMetadata,
    PublicationOverview, PublicationPresentation, PublicationSection, PublicationSnapshot,
    ResultsSection, Scene, SchematicSection, SheetScene, SignalIdentity, SignalTarget,
    SimulationProvenance, SimulationSetting, Specification, Stroke, StrokePattern, SweepAxis,
    TextAnchor, TextFont, TextPrimitive, Trace, TraceTransform, TraceValues, Validate as _,
};

use crate::hardcopy::HardcopyScope;
use crate::quantity::engineering::format_engineering_value;
use crate::simulation::netlist_gen::{
    HierarchySource, component_pin_names_with_hierarchy, design_nets_with_hierarchy,
};
use crate::state::{AnalysisResult, AnalysisType, SimulationRun, SpecEntry};
use crate::workbench::app_state::AppState;
use crate::workbench::hardcopy_adapters::render::{
    HardcopyScene, HardcopySceneMetadata, SceneFill, SceneFont, ScenePoint, ScenePrimitive,
    SceneTextRotation, SemanticColor, StrokePattern as SceneStrokePattern, StrokeStyle,
    TextAnchor as SceneTextAnchor, scene_from_resolved,
};
use crate::workbench::hardcopy_adapters::sources::{
    HardcopySemanticDocument, enumerate_retained_hardcopy_sources,
    prepare_retained_hardcopy_resolution,
};

/// Everything the caller decides about the publication; the builder derives
/// the rest from project state. `created_utc` is supplied here so the
/// builder itself stays clock-free and deterministic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PublicationDraft {
    pub title: String,
    pub description: String,
    pub author_display: String,
    pub created_utc: String,
    pub license: rspice_publication_contract::ContentLicense,
    pub overview_narrative: String,
    pub specification_label: String,
    pub specification_value: String,
    pub specification_unit: String,
    pub include_schematic: bool,
    pub include_results: bool,
    pub include_netlist: bool,
    pub activate_featured: bool,
}

impl Default for PublicationDraft {
    fn default() -> Self {
        Self {
            title: String::new(),
            description: String::new(),
            author_display: String::new(),
            created_utc: String::new(),
            license: rspice_publication_contract::ContentLicense::default(),
            overview_narrative: String::new(),
            specification_label: String::new(),
            specification_value: String::new(),
            specification_unit: String::new(),
            include_schematic: true,
            include_results: true,
            include_netlist: true,
            activate_featured: false,
        }
    }
}

/// UTC RFC 3339 stamp for snapshot provenance, from the wasm-safe clock and
/// the hardcopy pipeline's exact civil-date transform.
pub(crate) fn publication_timestamp_utc() -> String {
    let seconds = crate::time_compat::unix_epoch().as_secs();
    crate::workbench::hardcopy_adapters::render::HardcopyPublicationTimestamp::from_unix_seconds(
        seconds,
    )
    .map(|stamp| {
        format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
            stamp.year, stamp.month, stamp.day, stamp.hour, stamp.minute, stamp.second
        )
    })
    .unwrap_or_else(|_| "1970-01-01T00:00:00Z".to_string())
}

/// Why a snapshot could not be built. Every variant names the boundary that
/// refused, so the publish surface can show an actionable message.
#[derive(Debug)]
pub(crate) enum PublicationBuildError {
    /// A printable source failed hardcopy resolution.
    SourceResolution { source: String, reason: String },
    /// Scene compilation failed for a resolved source.
    SceneCompilation { source: String, reason: String },
    /// The scene contains a primitive publication cannot carry yet.
    UnsupportedPrimitive {
        source: String,
        primitive: &'static str,
    },
    /// A scene coordinate left the contract's integral range.
    CoordinateRange { source: String },
    /// A result trace carries a non-finite sample.
    NonFiniteSample { trace: String },
    /// The assembled snapshot failed contract validation.
    Contract(rspice_publication_contract::ContractError),
    /// Nothing publishable exists: no scenes, no results, no deck.
    NothingToPublish,
}

impl std::fmt::Display for PublicationBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SourceResolution { source, reason } => {
                write!(f, "cannot resolve {source}: {reason}")
            }
            Self::SceneCompilation { source, reason } => {
                write!(f, "cannot compile a scene for {source}: {reason}")
            }
            Self::UnsupportedPrimitive { source, primitive } => {
                write!(
                    f,
                    "{source} uses {primitive}, which publication does not carry yet"
                )
            }
            Self::CoordinateRange { source } => {
                write!(f, "{source} has coordinates outside the publishable range")
            }
            Self::NonFiniteSample { trace } => {
                write!(f, "trace {trace} carries a non-finite sample")
            }
            Self::Contract(error) => write!(f, "snapshot rejected by the contract: {error}"),
            Self::NothingToPublish => {
                write!(
                    f,
                    "the project has no schematic, results, or netlist to publish"
                )
            }
        }
    }
}

/// Build a complete, validated publication snapshot from the open project:
/// every currently printable schematic sheet and plot pane, plus the active
/// run's analyses, datasets, and measurements, plus the effective deck.
pub(crate) fn build_publication_snapshot(
    state: &AppState,
    draft: &PublicationDraft,
) -> Result<PublicationSnapshot, PublicationBuildError> {
    let (mut sheets, mut plot_figures) = collect_scenes(state)?;
    if !draft.include_schematic {
        sheets.clear();
    }
    if !draft.include_results {
        plot_figures.clear();
    }
    let netlist = draft
        .include_netlist
        .then(|| effective_deck(state))
        .flatten();
    let results = if draft.include_results {
        let specifications = active_specs(state);
        results_section(state.simulation.active_run(), &specifications)?
    } else {
        None
    };

    if sheets.is_empty() && plot_figures.is_empty() && netlist.is_none() && results.is_none() {
        return Err(PublicationBuildError::NothingToPublish);
    }

    let mut figures = Vec::new();
    let mut next_figure_id = 1_u64;
    for (index, sheet) in sheets.iter().enumerate() {
        figures.push(Figure {
            id: next_figure_id,
            title: sheet.name.clone(),
            content: FigureContent::SchematicSheet {
                sheet_index: index as u32,
            },
        });
        next_figure_id += 1;
    }
    for plot in plot_figures {
        let hydration = hydrate_plot(&plot.traces, results.as_ref());
        figures.push(Figure {
            id: next_figure_id,
            title: plot.title,
            content: FigureContent::Plot(PlotFigure {
                scene: plot.scene,
                hydration,
            }),
        });
        next_figure_id += 1;
    }

    let engineering = engineering_publication(state, draft.include_schematic, results.as_ref());
    let mut section_order = vec![PublicationSection::Overview];
    if !sheets.is_empty() {
        section_order.push(PublicationSection::Schematic);
    }
    if results.is_some() {
        section_order.push(PublicationSection::Results);
    }
    if !engineering.components.is_empty() {
        section_order.push(PublicationSection::Components);
    }
    if netlist.is_some() || results.is_some() {
        section_order.push(PublicationSection::Files);
    }
    section_order.push(PublicationSection::Details);
    let default_section = section_order
        .iter()
        .copied()
        .find(|section| *section != PublicationSection::Overview)
        .unwrap_or(PublicationSection::Details);
    let narrative = if draft.overview_narrative.trim().is_empty() {
        draft.description.trim()
    } else {
        draft.overview_narrative.trim()
    };
    let specifications = if draft.specification_label.trim().is_empty()
        || draft.specification_value.trim().is_empty()
    {
        Vec::new()
    } else {
        vec![Specification {
            label: draft.specification_label.trim().to_string(),
            value: draft.specification_value.trim().to_string(),
            unit: (!draft.specification_unit.trim().is_empty())
                .then(|| draft.specification_unit.trim().to_string()),
        }]
    };
    let featured_figure_id = figures
        .iter()
        .find(|figure| matches!(figure.content, FigureContent::Plot(_)))
        .or_else(|| figures.first())
        .map(|figure| figure.id);
    let figure_details = figures
        .iter()
        .map(|figure| rspice_publication_contract::FigurePresentation {
            figure_id: figure.id,
            caption: None,
            accessible_summary: match &figure.content {
                FigureContent::SchematicSheet { .. } => {
                    format!("Published schematic sheet: {}.", figure.title)
                }
                FigureContent::Plot(_) => format!("Published simulation plot: {}.", figure.title),
            },
            default_interactive: draft.activate_featured && featured_figure_id == Some(figure.id),
        })
        .collect();
    let snapshot = PublicationSnapshot {
        schema_version: rspice_publication_contract::PUBLICATION_SNAPSHOT_SCHEMA_VERSION,
        metadata: PublicationMetadata {
            title: draft.title.clone(),
            description: draft.description.clone(),
            author_display: draft.author_display.clone(),
            app_version: env!("CARGO_PKG_VERSION").to_string(),
            created_utc: draft.created_utc.clone(),
            license: draft.license,
        },
        disclosure: Disclosure {
            schematic: !sheets.is_empty(),
            netlist: netlist.is_some(),
            results: results.is_some(),
            archive: false,
        },
        schematic: if sheets.is_empty() {
            None
        } else {
            Some(SchematicSection { sheets })
        },
        netlist,
        results,
        figures,
        presentation: Some(PublicationPresentation {
            overview: (!narrative.is_empty() || !specifications.is_empty()).then(|| {
                PublicationOverview {
                    narrative: if narrative.is_empty() {
                        "Engineering publication".to_string()
                    } else {
                        narrative.to_string()
                    },
                    specifications,
                }
            }),
            section_order,
            default_section,
            featured_figure_id,
            figure_details,
        }),
        engineering: Some(engineering),
    };
    snapshot
        .validate()
        .map_err(PublicationBuildError::Contract)?;
    Ok(snapshot)
}

/// Project engineering identity published alongside the visual scenes.
///
/// Connectivity and pin names come from the same hierarchy-aware netlist
/// resolver used by simulation. This keeps the Components view truthful for
/// real native publications instead of emitting a permanently empty shell.
fn engineering_publication(
    state: &AppState,
    include_schematic: bool,
    results: Option<&ResultsSection>,
) -> EngineeringPublication {
    let simulation = results.map(|_| simulation_provenance(state.simulation.active_run()));
    if !include_schematic {
        return EngineeringPublication {
            components: Vec::new(),
            nets: Vec::new(),
            signals: signal_identities(results, &[], &[]),
            simulation,
        };
    }

    let hierarchy = HierarchySource::from_workspace_with_connectivity(
        &state.library_manager,
        &state.workspace.schematic_buffers,
        &state.workspace.connectivity,
    );
    let design_nets = design_nets_with_hierarchy(&state.schematic, &hierarchy);
    let resolved_pin_names = component_pin_names_with_hierarchy(&state.schematic, &hierarchy);
    let published_components = state
        .schematic
        .components
        .iter()
        .filter(|component| !component.kind.spice_prefix().is_empty())
        .collect::<Vec<_>>();

    let components = published_components
        .iter()
        .map(|component| {
            let mut pin_names = resolved_pin_names
                .get(&component.id)
                .cloned()
                .unwrap_or_default();
            for terminal in design_nets
                .iter()
                .flat_map(|net| &net.terminals)
                .filter(|terminal| terminal.component_id == component.id)
            {
                if !pin_names
                    .iter()
                    .any(|name| name.eq_ignore_ascii_case(&terminal.pin))
                {
                    pin_names.push(terminal.pin.clone());
                }
            }
            let pins = pin_names
                .into_iter()
                .map(|name| {
                    let net = design_nets.iter().find_map(|net| {
                        net.terminals
                            .iter()
                            .any(|terminal| {
                                terminal.component_id == component.id
                                    && terminal.pin.eq_ignore_ascii_case(&name)
                            })
                            .then(|| net.name.clone())
                    });
                    ComponentPin {
                        name,
                        number: None,
                        net,
                    }
                })
                .collect();
            ComponentRecord {
                reference: component.name.trim().to_owned(),
                value: component.value.trim().to_owned(),
                device: component.kind.display_name().to_owned(),
                model: model_reference(component),
                pins,
            }
        })
        .collect::<Vec<_>>();

    let nets = design_nets
        .iter()
        .map(|net| NetRecord {
            name: net.name.clone(),
            connections: net
                .terminals
                .iter()
                .filter_map(|terminal| {
                    published_components
                        .iter()
                        .find(|component| component.id == terminal.component_id)
                        .map(|component| NetConnection {
                            component_reference: component.name.trim().to_owned(),
                            pin_name: terminal.pin.clone(),
                        })
                })
                .collect(),
        })
        .collect::<Vec<_>>();
    let signals = signal_identities(results, &components, &nets);

    EngineeringPublication {
        components,
        nets,
        signals,
        simulation,
    }
}

fn model_reference(component: &crate::state::Component) -> Option<ModelReference> {
    let binding = component.library_cell.as_ref()?;
    let name = binding
        .generated_veriloga
        .as_ref()
        .map(|generated| generated.model_name.as_str())
        .or_else(|| {
            binding
                .builtin_xspice
                .as_ref()
                .map(|builtin| builtin.model_type.as_str())
        })
        .or(binding.module_name.as_deref())
        .unwrap_or(binding.cell.as_str());
    Some(ModelReference {
        name: name.to_owned(),
        device_class: component.kind.display_name().to_owned(),
        library: Some(binding.library.clone()),
    })
}

fn signal_identities(
    results: Option<&ResultsSection>,
    components: &[ComponentRecord],
    nets: &[NetRecord],
) -> Vec<SignalIdentity> {
    let Some(results) = results else {
        return Vec::new();
    };
    results
        .datasets
        .iter()
        .flat_map(|dataset| {
            dataset
                .traces
                .iter()
                .enumerate()
                .filter_map(move |(trace_index, trace)| {
                    let trace_index = u32::try_from(trace_index).ok()?;
                    let target = signal_target(&trace.label, components, nets);
                    Some(SignalIdentity {
                        dataset_id: dataset.id,
                        trace_index,
                        target,
                    })
                })
        })
        .collect()
}

fn signal_target(label: &str, components: &[ComponentRecord], nets: &[NetRecord]) -> SignalTarget {
    if let Some(candidate) = function_argument(label, "V")
        && let Some(net) = nets
            .iter()
            .find(|net| net.name.eq_ignore_ascii_case(candidate))
    {
        return SignalTarget::NetVoltage {
            net: net.name.clone(),
        };
    }
    if let Some(candidate) = function_argument(label, "I")
        && let Some(component) = components
            .iter()
            .find(|component| component.reference.eq_ignore_ascii_case(candidate))
    {
        return SignalTarget::DeviceCurrent {
            reference: component.reference.clone(),
        };
    }
    SignalTarget::Expression {
        label: label.to_owned(),
    }
}

fn function_argument<'a>(label: &'a str, function: &str) -> Option<&'a str> {
    let label = label.trim();
    let (head, remainder) = label.split_at_checked(function.len())?;
    if !head.eq_ignore_ascii_case(function) {
        return None;
    }
    let argument = remainder.strip_prefix('(')?.strip_suffix(')')?.trim();
    (!argument.is_empty() && !argument.contains(['(', ')'])).then_some(argument)
}

fn simulation_provenance(run: Option<&SimulationRun>) -> SimulationProvenance {
    let common_point = run.and_then(|run| {
        let mut successful = run.analyses.iter().filter(|analysis| analysis.success);
        let first = successful.next()?.provenance()?.pvt_point()?;
        successful
            .all(|analysis| {
                analysis
                    .provenance()
                    .and_then(|provenance| provenance.pvt_point())
                    == Some(first)
            })
            .then_some(first)
    });
    let settings = run
        .and_then(|run| run.execution_target)
        .map(|target| {
            vec![SimulationSetting {
                name: "Execution target".to_owned(),
                value: target.label().to_owned(),
            }]
        })
        .unwrap_or_default();
    SimulationProvenance {
        engine: "RSpice".to_owned(),
        engine_version: env!("CARGO_PKG_VERSION").to_owned(),
        temperature_c_bits: common_point.map(|point| point.temperature_celsius().to_bits()),
        corner: common_point.map(|point| point.process().to_owned()),
        settings,
        warnings: Vec::new(),
    }
}

// ---------------------------------------------------------------------------
// Scenes
// ---------------------------------------------------------------------------

/// One retained result trace of a resolved plot pane, kept alongside the
/// compiled scene so hydration can bind it to the published datasets by
/// exact sample identity.
struct PlotTraceSource {
    label: String,
    x_bits: Vec<u64>,
    y_bits: Vec<u64>,
}

/// A compiled plot scene plus the semantic trace samples it was drawn from.
struct PlotSceneSource {
    title: String,
    scene: Scene,
    traces: Vec<PlotTraceSource>,
}

/// Resolve every printable schematic sheet and plot pane through the
/// hardcopy source registry, one source at a time so no aggregate clipping
/// is ever involved, and compile each into a contract scene.
fn collect_scenes(
    state: &AppState,
) -> Result<(Vec<SheetScene>, Vec<PlotSceneSource>), PublicationBuildError> {
    let mut sheets = Vec::new();
    let mut plots = Vec::new();
    for descriptor in enumerate_retained_hardcopy_sources(state) {
        let scope = if descriptor
            .allowed_scopes
            .contains(&HardcopyScope::CurrentSheet)
        {
            HardcopyScope::CurrentSheet
        } else if descriptor
            .allowed_scopes
            .contains(&HardcopyScope::ActivePlotDocument)
        {
            HardcopyScope::ActivePlotDocument
        } else {
            continue;
        };
        let prepared =
            match prepare_retained_hardcopy_resolution(state, &descriptor.source_key, scope) {
                Ok(prepared) => prepared,
                Err(error) => {
                    return Err(PublicationBuildError::SourceResolution {
                        source: descriptor.display_name.clone(),
                        reason: error.to_string(),
                    });
                }
            };
        let resolved =
            prepared
                .resolve_owned()
                .map_err(|error| PublicationBuildError::SourceResolution {
                    source: descriptor.display_name.clone(),
                    reason: error.to_string(),
                })?;

        let is_schematic = matches!(
            resolved.semantic_document(),
            HardcopySemanticDocument::Schematic(_)
        );
        let is_plot = matches!(
            resolved.semantic_document(),
            HardcopySemanticDocument::Plot(_)
        );
        if !is_schematic && !is_plot {
            continue;
        }

        let metadata = HardcopySceneMetadata::try_new(descriptor.display_name.clone(), "RSpice")
            .map_err(|error| PublicationBuildError::SceneCompilation {
                source: descriptor.display_name.clone(),
                reason: error.to_string(),
            })?;
        let setup = publication_schematic_setup();
        let scene =
            scene_from_resolved(&resolved, resolved.default_print_mapping(), setup, metadata)
                .map_err(|error| PublicationBuildError::SceneCompilation {
                    source: descriptor.display_name.clone(),
                    reason: error.to_string(),
                })?;
        let converted = convert_scene(&scene, &descriptor.display_name)?;
        if is_schematic {
            sheets.push(SheetScene {
                name: descriptor.display_name.clone(),
                page_label: None,
                scene: converted,
            });
        } else {
            let traces = match resolved.semantic_document() {
                HardcopySemanticDocument::Plot(plot) => plot
                    .traces
                    .iter()
                    .map(|trace| PlotTraceSource {
                        label: trace.label.clone(),
                        x_bits: trace.source_samples.iter().map(|(x, _)| *x).collect(),
                        y_bits: trace.source_samples.iter().map(|(_, y)| *y).collect(),
                    })
                    .collect(),
                _ => Vec::new(),
            };
            plots.push(PlotSceneSource {
                title: descriptor.display_name.clone(),
                scene: converted,
                traces,
            });
        }
    }
    Ok((sheets, plots))
}

// ---------------------------------------------------------------------------
// Plot hydration
// ---------------------------------------------------------------------------

/// Bind a plot figure's drawn traces to the published datasets, or return
/// `None` and leave the figure a static scene.
///
/// A binding is emitted only when the published dataset values reproduce the
/// drawn ordinates bit-exactly under one declared transform, so a hydrated
/// figure can never disagree with the reviewed static scene. Traces the
/// datasets cannot reproduce exactly — a pane bound to a non-active run, or
/// a derived trace such as a differential probe's magnitude difference —
/// fail the whole figure closed to its static scene.
fn hydrate_plot(
    traces: &[PlotTraceSource],
    results: Option<&ResultsSection>,
) -> Option<PlotHydration> {
    let results = results?;
    if traces.is_empty() {
        return None;
    }
    let mut bindings = Vec::with_capacity(traces.len());
    let mut x_axis: Option<&SweepAxis> = None;
    let mut y_unit: Option<&str> = None;
    for trace in traces {
        let (dataset, trace_index, transform) = bind_trace(trace, results)?;
        x_axis.get_or_insert(&dataset.sweep);
        let unit = dataset.traces[trace_index as usize].unit.as_str();
        y_unit = match y_unit {
            None => Some(unit),
            Some(existing) if existing == unit => Some(existing),
            Some(_) => Some(""),
        };
        bindings.push(PlotTraceBinding {
            dataset_id: dataset.id,
            trace_index,
            transform,
        });
    }
    let sweep = x_axis?;
    // The compiled scene maps both axes linearly (`map_plot_point`), so the
    // hydrated instrument declares exactly that mapping.
    Some(PlotHydration {
        x_scale: AxisScale::Linear,
        y_scale: AxisScale::Linear,
        x_label: if sweep.unit.is_empty() {
            sweep.label.clone()
        } else {
            format!("{} ({})", sweep.label, sweep.unit)
        },
        y_label: y_unit.unwrap_or_default().to_owned(),
        bindings,
    })
}

/// Locate the one published trace whose values reproduce this drawn trace.
fn bind_trace<'results>(
    trace: &PlotTraceSource,
    results: &'results ResultsSection,
) -> Option<(&'results Dataset, u32, TraceTransform)> {
    for dataset in &results.datasets {
        if dataset.sweep.values_bits != trace.x_bits {
            continue;
        }
        for (index, candidate) in dataset.traces.iter().enumerate() {
            if candidate.label != trace.label {
                continue;
            }
            if let Some(transform) = verify_transform(candidate, &trace.y_bits) {
                return Some((dataset, index as u32, transform));
            }
        }
    }
    None
}

/// The transform under which `candidate`'s stored values reproduce the drawn
/// ordinates bit-for-bit, if one exists. Formulas match the run-ingestion
/// producers exactly (`results_convert::build_ac_waveforms_owned`), so a
/// faithful pairing verifies and everything else is rejected.
fn verify_transform(candidate: &Trace, y_bits: &[u64]) -> Option<TraceTransform> {
    match &candidate.values {
        TraceValues::Real { bits } => (bits == y_bits).then_some(TraceTransform::Identity),
        TraceValues::Complex {
            real_bits,
            imaginary_bits,
        } => {
            if real_bits.len() != y_bits.len() || imaginary_bits.len() != y_bits.len() {
                return None;
            }
            type TransformFormula = fn(f64, f64) -> f64;
            const CANDIDATES: [(TraceTransform, TransformFormula); 5] = [
                (TraceTransform::Magnitude, |r, i| (r * r + i * i).sqrt()),
                (TraceTransform::MagnitudeDb, |r, i| {
                    20.0 * (r * r + i * i).sqrt().log10()
                }),
                (TraceTransform::PhaseDegrees, |r, i| i.atan2(r).to_degrees()),
                (TraceTransform::RealPart, |r, _| r),
                (TraceTransform::ImaginaryPart, |_, i| i),
            ];
            CANDIDATES.iter().find_map(|(transform, formula)| {
                real_bits
                    .iter()
                    .zip(imaginary_bits)
                    .zip(y_bits)
                    .all(|((real, imaginary), y)| {
                        formula(f64::from_bits(*real), f64::from_bits(*imaginary)).to_bits() == *y
                    })
                    .then_some(*transform)
            })
        }
    }
}

/// Publication compiles headlessly, so the interactive `Ask` overflow policy
/// is replaced with `ExtendOutput`: everything the author drew is retained
/// and the sheet extent grows if content sits outside it. Crop marks and the
/// editing grid are print apparatus and stay off.
fn publication_schematic_setup() -> crate::hardcopy::SchematicHardcopySetup {
    crate::hardcopy::SchematicHardcopySetup::new(
        crate::hardcopy::SchematicHardcopyExtent::AuthoredDrawingSheet,
        crate::hardcopy::OutsideSheetContentPolicy::ExtendOutput,
        false,
        true,
        true,
        true,
        true,
        false,
    )
}

fn point(value: ScenePoint, source: &str) -> Result<Point, PublicationBuildError> {
    let range = |_| PublicationBuildError::CoordinateRange {
        source: source.to_string(),
    };
    Ok(Point {
        x_um: i64::try_from(value.x.micrometres()).map_err(range)?,
        y_um: i64::try_from(value.y.micrometres()).map_err(range)?,
    })
}

fn paint(color: SemanticColor) -> Paint {
    match color {
        SemanticColor::Foreground => Paint::Role(PaintRole::Foreground),
        SemanticColor::Secondary => Paint::Role(PaintRole::Secondary),
        SemanticColor::Grid => Paint::Role(PaintRole::Grid),
        SemanticColor::Accent => Paint::Role(PaintRole::Accent),
        SemanticColor::Warning => Paint::Role(PaintRole::Warning),
        SemanticColor::Success => Paint::Role(PaintRole::Success),
        SemanticColor::Trace(index) => Paint::Role(PaintRole::TraceSeries((index % 256) as u8)),
        SemanticColor::Exact(rgb) => Paint::Rgba([rgb.red, rgb.green, rgb.blue, 255]),
    }
}

fn stroke(style: &StrokeStyle) -> Stroke {
    Stroke {
        width_um: style.width.micrometres(),
        paint: paint(style.color),
        pattern: match style.pattern {
            SceneStrokePattern::Solid => StrokePattern::Solid,
            SceneStrokePattern::Dashed => StrokePattern::Dashed,
            SceneStrokePattern::Dotted => StrokePattern::Dotted,
            SceneStrokePattern::DashDot => StrokePattern::DashDot,
        },
    }
}

fn fill(value: &SceneFill) -> Paint {
    match value {
        SceneFill::Solid { color } => paint(*color),
        // Cross-hatch is a print-redundancy affordance; the color page
        // carries the same information as a solid fill of the mapped color.
        SceneFill::CrossHatch { color, .. } => paint(*color),
    }
}

fn convert_primitive(
    primitive: &ScenePrimitive,
    source: &str,
    out: &mut Vec<Primitive>,
) -> Result<(), PublicationBuildError> {
    match primitive {
        ScenePrimitive::Line {
            from,
            to,
            stroke: style,
        } => out.push(Primitive::Path(PathPrimitive {
            segments: vec![
                PathSegment::MoveTo {
                    to: point(*from, source)?,
                },
                PathSegment::LineTo {
                    to: point(*to, source)?,
                },
            ],
            stroke: Some(stroke(style)),
            fill: None,
        })),
        ScenePrimitive::Polyline {
            points,
            closed,
            stroke: style,
            fill: fill_style,
        } => {
            let mut segments = Vec::with_capacity(points.len() + usize::from(*closed));
            for (index, vertex) in points.iter().enumerate() {
                let to = point(*vertex, source)?;
                segments.push(if index == 0 {
                    PathSegment::MoveTo { to }
                } else {
                    PathSegment::LineTo { to }
                });
            }
            if *closed {
                segments.push(PathSegment::Close);
            }
            out.push(Primitive::Path(PathPrimitive {
                segments,
                stroke: Some(stroke(style)),
                fill: fill_style.as_ref().map(fill),
            }));
        }
        ScenePrimitive::Rect {
            rect,
            stroke: style,
            fill: fill_style,
        } => {
            let x = i64::try_from(rect.x.micrometres());
            let y = i64::try_from(rect.y.micrometres());
            let width = i64::try_from(rect.width.micrometres());
            let height = i64::try_from(rect.height.micrometres());
            let (Ok(x), Ok(y), Ok(width), Ok(height)) = (x, y, width, height) else {
                return Err(PublicationBuildError::CoordinateRange {
                    source: source.to_string(),
                });
            };
            out.push(Primitive::Path(PathPrimitive {
                segments: vec![
                    PathSegment::MoveTo {
                        to: Point { x_um: x, y_um: y },
                    },
                    PathSegment::LineTo {
                        to: Point {
                            x_um: x + width,
                            y_um: y,
                        },
                    },
                    PathSegment::LineTo {
                        to: Point {
                            x_um: x + width,
                            y_um: y + height,
                        },
                    },
                    PathSegment::LineTo {
                        to: Point {
                            x_um: x,
                            y_um: y + height,
                        },
                    },
                    PathSegment::Close,
                ],
                stroke: style.as_ref().map(stroke),
                fill: fill_style.as_ref().map(fill),
            }));
        }
        ScenePrimitive::Circle {
            center,
            radius,
            stroke: style,
            fill: fill_style,
        } => out.push(Primitive::Path(PathPrimitive {
            segments: vec![PathSegment::Arc {
                center: point(*center, source)?,
                radius_um: radius.micrometres(),
                start_millideg: 0,
                sweep_millideg: 360_000,
            }],
            stroke: style.as_ref().map(stroke),
            fill: fill_style.as_ref().map(fill),
        })),
        ScenePrimitive::Text {
            origin,
            text,
            font,
            size,
            color,
            anchor,
            rotation,
        } => out.push(Primitive::Text(TextPrimitive {
            origin: point(*origin, source)?,
            text: text.clone(),
            height_um: size.micrometres(),
            font: match font {
                SceneFont::Sans => TextFont::Sans,
                SceneFont::SansSemibold => TextFont::SansSemibold,
                SceneFont::Monospace => TextFont::Monospace,
            },
            anchor: match anchor {
                SceneTextAnchor::Start => TextAnchor::Start,
                SceneTextAnchor::Middle => TextAnchor::Middle,
                SceneTextAnchor::End => TextAnchor::End,
            },
            rotation_millideg: match rotation {
                SceneTextRotation::Upright => 0,
                SceneTextRotation::Clockwise90 => -90_000,
                SceneTextRotation::CounterClockwise90 => 90_000,
            },
            paint: paint(*color),
        })),
        ScenePrimitive::RasterImage { .. } => {
            return Err(PublicationBuildError::UnsupportedPrimitive {
                source: source.to_string(),
                primitive: "an embedded raster image",
            });
        }
        ScenePrimitive::ClippedGroup { .. } => {
            return Err(PublicationBuildError::UnsupportedPrimitive {
                source: source.to_string(),
                primitive: "a clipped composite group",
            });
        }
    }
    Ok(())
}

/// Convert one compiled hardcopy scene into a contract scene. The compiled
/// scene is a flat painter's-order list, so the result is a single untagged
/// group; semantic hover tags arrive when compilation grows group identity.
fn convert_scene(scene: &HardcopyScene, source: &str) -> Result<Scene, PublicationBuildError> {
    let mut primitives = Vec::with_capacity(scene.primitives().len());
    for primitive in scene.primitives() {
        convert_primitive(primitive, source, &mut primitives)?;
    }
    Ok(Scene {
        width_um: scene.extent().width().micrometres(),
        height_um: scene.extent().height().micrometres(),
        groups: vec![PrimitiveGroup {
            tag: None,
            primitives,
        }],
    })
}

// ---------------------------------------------------------------------------
// Results
// ---------------------------------------------------------------------------

fn active_specs(state: &AppState) -> Vec<SpecEntry> {
    if let Some(receipt) = state
        .simulation
        .active_run()
        .and_then(crate::state::SimulationRun::prepared_receipt)
    {
        return receipt
            .specifications()
            .iter()
            .map(|specification| specification.entry().clone())
            .collect();
    }
    state
        .sim_setup
        .analysis_plan
        .as_ref()
        .map(|plan| state.workspace.active_specs(plan.id()))
        .unwrap_or(&[])
        .to_vec()
}

/// The sweep-axis identity the results views derive for each analysis kind.
fn sweep_axis_identity(analysis_type: AnalysisType) -> (&'static str, &'static str) {
    match analysis_type {
        AnalysisType::Ac | AnalysisType::Noise | AnalysisType::Pnoise => ("frequency", "Hz"),
        AnalysisType::Transient => ("time", "s"),
        AnalysisType::DcSweep => ("v-sweep", "V"),
        _ => ("x", ""),
    }
}

/// The unit a published trace is labelled with: what the waveform states, or
/// else the naming convention the results views derive from a trace name.
fn trace_unit<'a>(name: &str, retained_unit: Option<&'a str>) -> &'a str {
    if let Some(unit) = retained_unit {
        return unit;
    }
    let lower = name.to_ascii_lowercase();
    if lower.starts_with("i(") {
        "A"
    } else if lower.starts_with("v(") {
        "V"
    } else if lower.starts_with("p(") {
        "W"
    } else {
        ""
    }
}

fn bits_of(values: &[f64], trace: &str) -> Result<Vec<u64>, PublicationBuildError> {
    if values.iter().any(|value| !value.is_finite()) {
        return Err(PublicationBuildError::NonFiniteSample {
            trace: trace.to_string(),
        });
    }
    Ok(values.iter().map(|value| value.to_bits()).collect())
}

fn results_section(
    run: Option<&SimulationRun>,
    specs: &[SpecEntry],
) -> Result<Option<ResultsSection>, PublicationBuildError> {
    let Some(run) = run else {
        return Ok(None);
    };
    let mut analyses = Vec::new();
    let mut datasets = Vec::new();
    let mut measurements = Vec::new();
    let mut next_dataset_id = 1_u64;

    for (index, analysis) in run.analyses.iter().enumerate() {
        if !analysis.success {
            continue;
        }
        let analysis_id = index as u64 + 1;
        analyses.push(AnalysisRecord {
            id: analysis_id,
            label: analysis.label.clone(),
            card: analysis.analysis_type.spice_command().to_string(),
        });

        datasets.append(&mut analysis_datasets(
            analysis,
            analysis_id,
            &mut next_dataset_id,
        )?);

        for measure in &analysis.measurements {
            let spec = specs
                .iter()
                .find(|entry| entry.measurement.eq_ignore_ascii_case(&measure.name));
            measurements.push(publication_measurement(measure, spec, analysis_id));
        }
    }

    if analyses.is_empty() {
        return Ok(None);
    }
    Ok(Some(ResultsSection {
        analyses,
        datasets,
        measurements,
    }))
}

/// Group one analysis's waveforms into rectangular datasets by identical
/// sweep vectors, preserving encounter order.
fn analysis_datasets(
    analysis: &AnalysisResult,
    analysis_id: u64,
    next_dataset_id: &mut u64,
) -> Result<Vec<Dataset>, PublicationBuildError> {
    let (sweep_label, sweep_unit) = sweep_axis_identity(analysis.analysis_type);
    let mut datasets: Vec<Dataset> = Vec::new();
    let mut sweep_keys: Vec<Vec<u64>> = Vec::new();

    for waveform in &analysis.waveforms {
        if waveform.x.is_empty() || waveform.y.len() != waveform.x.len() {
            continue;
        }
        let sweep_bits = bits_of(&waveform.x, &waveform.name)?;
        let values = match &waveform.complex {
            Some(components) if components.real.len() == waveform.x.len() => TraceValues::Complex {
                real_bits: bits_of(&components.real, &waveform.name)?,
                imaginary_bits: bits_of(&components.imag, &waveform.name)?,
            },
            _ => TraceValues::Real {
                bits: bits_of(&waveform.y, &waveform.name)?,
            },
        };
        let trace = Trace {
            label: waveform.name.clone(),
            unit: trace_unit(&waveform.name, waveform.unit.as_deref()).to_string(),
            values,
        };

        if let Some(position) = sweep_keys.iter().position(|key| *key == sweep_bits) {
            datasets[position].traces.push(trace);
        } else {
            datasets.push(Dataset {
                id: *next_dataset_id,
                analysis_id,
                name: analysis.label.clone(),
                variant: if sweep_keys.is_empty() {
                    None
                } else {
                    Some(format!("series {}", sweep_keys.len() + 1))
                },
                sweep: SweepAxis {
                    label: sweep_label.to_string(),
                    unit: sweep_unit.to_string(),
                    values_bits: sweep_bits.clone(),
                },
                traces: vec![trace],
            });
            sweep_keys.push(sweep_bits);
            *next_dataset_id += 1;
        }
    }
    Ok(datasets)
}

fn publication_measurement(
    measure: &rspice_core::MeasureResult,
    spec: Option<&SpecEntry>,
    analysis_id: u64,
) -> Measurement {
    let display = match measure.value {
        Some(value) => {
            let formatted = format_engineering_value(value);
            match spec {
                Some(entry) if !entry.unit.trim().is_empty() => {
                    format!("{formatted} {}", entry.unit.trim())
                }
                _ => formatted,
            }
        }
        None => "not computed".to_string(),
    };
    let spec_display = spec.and_then(|entry| {
        let unit = if entry.unit.trim().is_empty() {
            String::new()
        } else {
            format!(" {}", entry.unit.trim())
        };
        match (entry.min, entry.max) {
            (Some(min), Some(max)) => Some(format!(
                "{}–{}{unit}",
                format_engineering_value(min),
                format_engineering_value(max)
            )),
            (Some(min), None) => Some(format!("≥ {}{unit}", format_engineering_value(min))),
            (None, Some(max)) => Some(format!("≤ {}{unit}", format_engineering_value(max))),
            (None, None) => None,
        }
    });
    let passed = match (spec, measure.value) {
        (Some(entry), Some(value)) => Some(entry.passes(value)),
        (Some(_), None) => Some(false),
        (None, _) if measure.expected.is_some() => Some(measure.passed),
        _ => None,
    };
    Measurement {
        analysis_id,
        name: measure.name.clone(),
        value_bits: measure
            .value
            .filter(|value| value.is_finite())
            .map(f64::to_bits),
        display,
        spec_display,
        passed,
    }
}

// ---------------------------------------------------------------------------
// Netlist
// ---------------------------------------------------------------------------

/// The deck the publication carries: the exact buffer from the last
/// completed manual-deck run when one exists, otherwise the same effective
/// source the run gate would execute. Line endings are normalized to the
/// contract's canonical form.
fn effective_deck(state: &AppState) -> Option<NetlistSection> {
    let raw = state
        .ui
        .netlist
        .last_run_buffer
        .as_deref()
        .or(state.workspace.netlist_source.as_deref())
        .filter(|deck| !deck.trim().is_empty())
        .or_else(|| {
            let live = state.simulation.netlist_content.as_str();
            (!live.trim().is_empty()).then_some(live)
        })?;
    let deck = raw.replace("\r\n", "\n").replace('\r', "\n");
    Some(NetlistSection { deck })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hardcopy::{ContentExtent, Length};
    use crate::state::WaveformData;
    use crate::workbench::hardcopy_adapters::render::{ScenePrimitive, SceneRect};
    use std::sync::Arc;

    fn scene_point(x_um: u64, y_um: u64) -> ScenePoint {
        ScenePoint {
            x: Length::from_micrometres(x_um),
            y: Length::from_micrometres(y_um),
        }
    }

    fn test_scene(primitives: Vec<ScenePrimitive>) -> HardcopyScene {
        let extent = ContentExtent::try_new(
            Length::from_micrometres(100_000),
            Length::from_micrometres(80_000),
        )
        .expect("extent");
        let metadata = HardcopySceneMetadata::try_new("Test scene", "RSpice").expect("metadata");
        HardcopyScene::try_new(extent, metadata, primitives, Vec::new()).expect("scene")
    }

    #[test]
    fn scene_conversion_covers_every_supported_primitive() {
        let stroke_style = |color| {
            StrokeStyle::try_new(
                color,
                Length::from_micrometres(250),
                SceneStrokePattern::Solid,
                None,
            )
            .expect("stroke")
        };
        let scene = test_scene(vec![
            ScenePrimitive::Line {
                from: scene_point(1_000, 1_000),
                to: scene_point(9_000, 1_000),
                stroke: stroke_style(SemanticColor::Foreground),
            },
            ScenePrimitive::Polyline {
                points: vec![
                    scene_point(2_000, 2_000),
                    scene_point(6_000, 2_000),
                    scene_point(6_000, 6_000),
                ],
                closed: true,
                stroke: stroke_style(SemanticColor::Secondary),
                fill: Some(SceneFill::Solid {
                    color: SemanticColor::Accent,
                }),
            },
            ScenePrimitive::Rect {
                rect: SceneRect {
                    x: Length::from_micrometres(10_000),
                    y: Length::from_micrometres(10_000),
                    width: Length::from_micrometres(5_000),
                    height: Length::from_micrometres(4_000),
                },
                stroke: Some(stroke_style(SemanticColor::Grid)),
                fill: None,
            },
            ScenePrimitive::Circle {
                center: scene_point(30_000, 30_000),
                radius: Length::from_micrometres(2_000),
                stroke: Some(stroke_style(SemanticColor::Trace(3))),
                fill: None,
            },
            ScenePrimitive::Text {
                origin: scene_point(40_000, 40_000),
                text: "V(out)".to_string(),
                font: SceneFont::SansSemibold,
                size: Length::from_micrometres(2_800),
                color: SemanticColor::Secondary,
                anchor: SceneTextAnchor::Middle,
                rotation: SceneTextRotation::Clockwise90,
            },
        ]);

        let converted = convert_scene(&scene, "test").expect("conversion");
        assert_eq!(converted.width_um, 100_000);
        assert_eq!(converted.height_um, 80_000);
        assert_eq!(converted.groups.len(), 1);
        let primitives = &converted.groups[0].primitives;
        assert_eq!(primitives.len(), 5);

        let Primitive::Path(line) = &primitives[0] else {
            panic!("line converts to a path");
        };
        assert_eq!(line.segments.len(), 2);

        let Primitive::Path(polygon) = &primitives[1] else {
            panic!("polyline converts to a path");
        };
        assert!(matches!(polygon.segments.last(), Some(PathSegment::Close)));
        assert_eq!(polygon.fill, Some(Paint::Role(PaintRole::Accent)));

        let Primitive::Path(rect) = &primitives[2] else {
            panic!("rect converts to a path");
        };
        assert_eq!(rect.segments.len(), 5);

        let Primitive::Path(circle) = &primitives[3] else {
            panic!("circle converts to a path");
        };
        assert!(matches!(
            circle.segments.as_slice(),
            [PathSegment::Arc {
                radius_um: 2_000,
                sweep_millideg: 360_000,
                ..
            }]
        ));
        assert_eq!(
            circle.stroke.as_ref().map(|s| s.paint),
            Some(Paint::Role(PaintRole::TraceSeries(3)))
        );

        let Primitive::Text(text) = &primitives[4] else {
            panic!("text converts to text");
        };
        assert_eq!(text.font, TextFont::SansSemibold);
        assert_eq!(text.anchor, TextAnchor::Middle);
        assert_eq!(text.rotation_millideg, -90_000);
        assert_eq!(text.paint, Paint::Role(PaintRole::Secondary));
    }

    #[test]
    fn unsupported_primitives_are_refused() {
        let mut out = Vec::new();
        let clipped = ScenePrimitive::ClippedGroup {
            source_origin: scene_point(0, 0),
            destination_origin: scene_point(0, 0),
            clip_extent: ContentExtent::try_new(
                Length::from_micrometres(1_000),
                Length::from_micrometres(1_000),
            )
            .expect("extent"),
            source_extent: ContentExtent::try_new(
                Length::from_micrometres(1_000),
                Length::from_micrometres(1_000),
            )
            .expect("extent"),
            primitives: Vec::new(),
        };
        assert!(matches!(
            convert_primitive(&clipped, "test", &mut out),
            Err(PublicationBuildError::UnsupportedPrimitive { .. })
        ));
    }

    fn waveform(name: &str, x: &[f64], y: &[f64]) -> WaveformData {
        WaveformData {
            name: name.to_string(),
            x: Arc::new(x.to_vec()),
            y: Arc::new(y.to_vec()),
            unit: None,
            color: "#000000".to_string(),
            complex: None,
            visible: true,
            display_cache: None,
        }
    }

    fn analysis(label: &str, waveforms: Vec<WaveformData>) -> AnalysisResult {
        AnalysisResult {
            id: 1,
            analysis_type: AnalysisType::Transient,
            label: label.to_string(),
            timestamp: 0.0,
            waveforms,
            dc_op: None,
            device_op: None,
            noise_summary: None,
            family_metadata: None,
            result_payload: None,
            measurements: Vec::new(),
            saved_output_receipts: Vec::new(),
            success: true,
            error_message: None,
            provenance: None,
        }
    }

    #[test]
    fn waveforms_group_into_datasets_by_identical_sweeps() {
        let shared = analysis(
            "Transient",
            vec![
                waveform("V(out)", &[0.0, 1.0, 2.0], &[0.0, 0.5, 0.8]),
                waveform("I(R1)", &[0.0, 1.0, 2.0], &[0.0, 0.1, 0.2]),
                waveform("V(mid)", &[0.0, 0.5], &[0.0, 0.3]),
            ],
        );
        let mut next_id = 1;
        let datasets = analysis_datasets(&shared, 1, &mut next_id).expect("datasets");
        assert_eq!(datasets.len(), 2, "two distinct sweeps");
        assert_eq!(
            datasets[0].traces.len(),
            2,
            "shared sweep carries both traces"
        );
        assert_eq!(datasets[0].traces[0].unit, "V");
        assert_eq!(datasets[0].traces[1].unit, "A");
        assert_eq!(datasets[0].variant, None);
        assert_eq!(datasets[1].variant.as_deref(), Some("series 2"));
        assert_eq!(datasets[0].sweep.label, "time");
        assert_eq!(datasets[0].sweep.unit, "s");
    }

    #[test]
    fn non_finite_samples_are_refused() {
        let bad = analysis(
            "Transient",
            vec![waveform("V(out)", &[0.0, 1.0], &[0.0, f64::NAN])],
        );
        let mut next_id = 1;
        assert!(matches!(
            analysis_datasets(&bad, 1, &mut next_id),
            Err(PublicationBuildError::NonFiniteSample { .. })
        ));
    }

    #[test]
    fn measurements_join_specs_case_insensitively() {
        let measure = rspice_core::MeasureResult {
            name: "Rise_Time".to_string(),
            value: Some(2.2e-3),
            error: None,
            passed: true,
            expected: None,
            tolerance: None,
            event_axis: None,
        };
        let spec = SpecEntry {
            measurement: "rise_time".to_string(),
            expression: String::new(),
            min: None,
            max: Some(2.5e-3),
            unit: "s".to_string(),
            scope: crate::state::SpecPointScope::AllPoints,
        };
        let published = publication_measurement(&measure, Some(&spec), 1);
        assert_eq!(published.passed, Some(true));
        assert!(
            published
                .spec_display
                .as_deref()
                .is_some_and(|s| s.starts_with('\u{2264}'))
        );
        assert!(published.value_bits.is_some());

        let unspecified = publication_measurement(&measure, None, 1);
        assert_eq!(unspecified.passed, None);
        assert_eq!(unspecified.spec_display, None);
    }

    #[test]
    fn deck_line_endings_normalize_to_canonical_form() {
        let mut state = AppState::default();
        state.simulation.netlist_content = "* RSpice Netlist\r\nR1 a b 1k\r\n.end".to_string();
        let deck = effective_deck(&state).expect("deck").deck;
        assert!(!deck.contains('\r'));
        assert_eq!(deck.lines().count(), 3);
    }

    #[test]
    fn authored_presentation_and_disclosure_choices_are_sealed() {
        let mut state = AppState::default();
        state.simulation.netlist_content = "* Filter\nR1 in out 1k\n.end".to_string();
        let snapshot = build_publication_snapshot(
            &state,
            &PublicationDraft {
                title: "Filter".to_string(),
                description: "Short page description".to_string(),
                author_display: "Test".to_string(),
                created_utc: "2026-08-06T00:00:00Z".to_string(),
                overview_narrative: "A low-pass filter used at the converter input.".to_string(),
                specification_label: "Cutoff".to_string(),
                specification_value: "1.59".to_string(),
                specification_unit: "kHz".to_string(),
                ..Default::default()
            },
        )
        .expect("netlist publication builds");

        let presentation = snapshot.presentation.expect("v3 presentation");
        assert_eq!(
            presentation.section_order,
            vec![
                PublicationSection::Overview,
                PublicationSection::Files,
                PublicationSection::Details,
            ]
        );
        assert_eq!(presentation.default_section, PublicationSection::Files);
        let overview = presentation.overview.expect("authored overview");
        assert_eq!(
            overview.narrative,
            "A low-pass filter used at the converter input."
        );
        assert_eq!(overview.specifications[0].label, "Cutoff");
        assert_eq!(overview.specifications[0].unit.as_deref(), Some("kHz"));

        assert!(matches!(
            build_publication_snapshot(
                &state,
                &PublicationDraft {
                    title: "Withheld".to_string(),
                    author_display: "Test".to_string(),
                    created_utc: "2026-08-06T00:00:00Z".to_string(),
                    include_netlist: false,
                    ..Default::default()
                }
            ),
            Err(PublicationBuildError::NothingToPublish)
        ));
    }

    #[test]
    fn empty_projects_report_nothing_to_publish() {
        let state = AppState::default();
        assert!(matches!(
            build_publication_snapshot(
                &state,
                &PublicationDraft {
                    title: "Empty".to_string(),
                    description: String::new(),
                    author_display: "Test".to_string(),
                    created_utc: "2026-08-06T00:00:00Z".to_string(),
                    license: rspice_publication_contract::ContentLicense::AllRightsReserved,
                    ..Default::default()
                }
            ),
            Err(PublicationBuildError::NothingToPublish)
        ));
    }

    #[test]
    fn native_engineering_projection_preserves_components_nets_and_signal_identity() {
        let mut state = AppState::default();
        state.schematic.add_component(
            crate::state::ComponentType::Resistor,
            crate::state::Point::new(0, 0),
        );

        let projected = engineering_publication(&state, true, None);
        assert_eq!(projected.components.len(), 1);
        let resistor = &projected.components[0];
        assert_eq!(resistor.reference, "R1");
        assert_eq!(resistor.device, "Resistor");
        assert_eq!(resistor.pins.len(), 2);
        assert!(resistor.pins.iter().all(|pin| pin.net.is_some()));
        assert!(projected.nets.iter().all(|net| {
            net.connections.iter().all(|connection| {
                connection.component_reference == "R1"
                    && resistor
                        .pins
                        .iter()
                        .any(|pin| pin.name == connection.pin_name)
            })
        }));

        let net_name = resistor.pins[0].net.as_deref().expect("resolved net");
        let results = ResultsSection {
            analyses: vec![AnalysisRecord {
                id: 1,
                label: "Operating point".to_owned(),
                card: ".op".to_owned(),
            }],
            datasets: vec![Dataset {
                id: 7,
                analysis_id: 1,
                name: "Operating point".to_owned(),
                variant: None,
                sweep: SweepAxis {
                    label: "point".to_owned(),
                    unit: String::new(),
                    values_bits: vec![0.0_f64.to_bits()],
                },
                traces: vec![
                    Trace {
                        label: format!("V({net_name})"),
                        unit: "V".to_owned(),
                        values: TraceValues::Real {
                            bits: vec![1.0_f64.to_bits()],
                        },
                    },
                    Trace {
                        label: "i(r1)".to_owned(),
                        unit: "A".to_owned(),
                        values: TraceValues::Real {
                            bits: vec![0.001_f64.to_bits()],
                        },
                    },
                    Trace {
                        label: "V(out) - V(in)".to_owned(),
                        unit: "V".to_owned(),
                        values: TraceValues::Real {
                            bits: vec![0.5_f64.to_bits()],
                        },
                    },
                ],
            }],
            measurements: Vec::new(),
        };

        let projected = engineering_publication(&state, true, Some(&results));
        assert!(matches!(
            &projected.signals[0].target,
            SignalTarget::NetVoltage { net } if net == net_name
        ));
        assert!(matches!(
            &projected.signals[1].target,
            SignalTarget::DeviceCurrent { reference } if reference == "R1"
        ));
        assert!(matches!(
            &projected.signals[2].target,
            SignalTarget::Expression { label } if label == "V(out) - V(in)"
        ));

        let withheld = engineering_publication(&state, false, Some(&results));
        assert!(withheld.components.is_empty());
        assert!(withheld.nets.is_empty());
        assert!(
            withheld
                .signals
                .iter()
                .all(|signal| matches!(signal.target, SignalTarget::Expression { .. }))
        );
    }

    #[test]
    fn hydration_binds_bit_exact_traces_and_fails_closed() {
        let x = [1.0_f64, 10.0, 100.0];
        let real = [3.0_f64, 0.5, -2.0];
        let imag = [4.0_f64, 0.25, 1.5];
        let magnitude: Vec<f64> = real
            .iter()
            .zip(&imag)
            .map(|(r, i)| (r * r + i * i).sqrt())
            .collect();
        let results = ResultsSection {
            analyses: vec![AnalysisRecord {
                id: 1,
                label: "AC".to_string(),
                card: ".ac".to_string(),
            }],
            datasets: vec![Dataset {
                id: 7,
                analysis_id: 1,
                name: "AC".to_string(),
                variant: None,
                sweep: SweepAxis {
                    label: "frequency".to_string(),
                    unit: "Hz".to_string(),
                    values_bits: x.iter().map(|value| value.to_bits()).collect(),
                },
                traces: vec![Trace {
                    label: "|V(out)|".to_string(),
                    unit: "V".to_string(),
                    values: TraceValues::Complex {
                        real_bits: real.iter().map(|value| value.to_bits()).collect(),
                        imaginary_bits: imag.iter().map(|value| value.to_bits()).collect(),
                    },
                }],
            }],
            measurements: Vec::new(),
        };

        let drawn = PlotTraceSource {
            label: "|V(out)|".to_string(),
            x_bits: x.iter().map(|value| value.to_bits()).collect(),
            y_bits: magnitude.iter().map(|value| value.to_bits()).collect(),
        };
        let hydration = hydrate_plot(std::slice::from_ref(&drawn), Some(&results))
            .expect("magnitude trace binds");
        assert_eq!(
            hydration.bindings,
            vec![PlotTraceBinding {
                dataset_id: 7,
                trace_index: 0,
                transform: TraceTransform::Magnitude,
            }]
        );
        assert_eq!(hydration.x_label, "frequency (Hz)");
        assert_eq!(hydration.y_label, "V");
        assert!(matches!(hydration.x_scale, AxisScale::Linear));

        // An ordinate no transform reproduces (a differential probe's
        // magnitude difference) fails the whole figure closed.
        let unreproducible = PlotTraceSource {
            label: "|V(out)|".to_string(),
            x_bits: x.iter().map(|value| value.to_bits()).collect(),
            y_bits: magnitude
                .iter()
                .map(|value| (value - 0.125).to_bits())
                .collect(),
        };
        assert!(hydrate_plot(&[drawn, unreproducible], Some(&results)).is_none());
        assert!(hydrate_plot(&[], Some(&results)).is_none());
    }

    #[test]
    fn transform_verification_matches_the_producer_formulas_exactly() {
        let bits = |values: &[f64]| {
            values
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>()
        };
        let phase_trace = Trace {
            label: "phase(V(out))".to_string(),
            unit: String::new(),
            values: TraceValues::Real {
                bits: bits(&[45.0, -90.0]),
            },
        };
        assert_eq!(
            verify_transform(&phase_trace, &bits(&[45.0, -90.0])),
            Some(TraceTransform::Identity)
        );
        assert_eq!(verify_transform(&phase_trace, &bits(&[45.0, -91.0])), None);

        let real = [3.0_f64, -1.0];
        let imag = [4.0_f64, 2.0];
        let complex_trace = Trace {
            label: "|V(out)|".to_string(),
            unit: "V".to_string(),
            values: TraceValues::Complex {
                real_bits: bits(&real),
                imaginary_bits: bits(&imag),
            },
        };
        let apply = |formula: fn(f64, f64) -> f64| {
            bits(
                &real
                    .iter()
                    .zip(&imag)
                    .map(|(r, i)| formula(*r, *i))
                    .collect::<Vec<_>>(),
            )
        };
        assert_eq!(
            verify_transform(&complex_trace, &apply(|r, i| (r * r + i * i).sqrt())),
            Some(TraceTransform::Magnitude)
        );
        assert_eq!(
            verify_transform(
                &complex_trace,
                &apply(|r, i| 20.0 * (r * r + i * i).sqrt().log10())
            ),
            Some(TraceTransform::MagnitudeDb)
        );
        assert_eq!(
            verify_transform(&complex_trace, &apply(|r, i| i.atan2(r).to_degrees())),
            Some(TraceTransform::PhaseDegrees)
        );
        assert_eq!(
            verify_transform(&complex_trace, &apply(|r, _| r)),
            Some(TraceTransform::RealPart)
        );
        assert_eq!(
            verify_transform(&complex_trace, &apply(|_, i| i)),
            Some(TraceTransform::ImaginaryPart)
        );
        assert_eq!(verify_transform(&complex_trace, &apply(|r, i| r + i)), None);
    }

    #[test]
    fn run_results_and_deck_build_a_valid_snapshot() {
        let mut state = AppState::default();
        state.simulation.netlist_content = "* RSpice Netlist\nR1 in out 1k\n.end".to_string();
        let mut run = crate::state::SimulationRun::new(1);
        run.analyses.push({
            let mut result = analysis(
                "Transient",
                vec![waveform("V(out)", &[0.0, 1.0, 2.0], &[0.0, 0.5, 0.8])],
            );
            result.measurements.push(rspice_core::MeasureResult {
                name: "final".to_string(),
                value: Some(0.8),
                error: None,
                passed: true,
                expected: None,
                tolerance: None,
                event_axis: None,
            });
            result
        });
        run.success = true;
        state.simulation.runs.push(run);
        state.simulation.active_run_idx = Some(0);

        let snapshot = build_publication_snapshot(
            &state,
            &PublicationDraft {
                title: "RC deck".to_string(),
                description: String::new(),
                author_display: "Test".to_string(),
                created_utc: "2026-08-06T00:00:00Z".to_string(),
                license: rspice_publication_contract::ContentLicense::CcBy40,
                ..Default::default()
            },
        )
        .expect("snapshot builds");
        assert!(snapshot.netlist.is_some());
        let results = snapshot.results.as_ref().expect("results");
        assert_eq!(results.analyses.len(), 1);
        assert_eq!(results.datasets.len(), 1);
        assert_eq!(results.measurements.len(), 1);
        assert!(snapshot.disclosure.netlist);
        assert!(snapshot.disclosure.results);
        assert!(!snapshot.disclosure.schematic);
    }
}
