//! Dataset-driven Create Result Document workflow.
//!
//! The modal is a transaction editor, not a navigation shortcut. Every
//! dataset, family, viewer, layout, and title is re-resolved when Create is
//! pressed; only a fully validated [`VisualizationDocument`] enters the
//! project-owned collection. Solver samples and retained run history are
//! never mutated.

use egui::{Color32, RichText, Sense, Ui, UiBuilder, vec2};

use crate::diagnostics::ConsoleMessage;
use crate::product::{AnalysisInstanceId, DatasetBinding, DatasetId, ResultDocumentId};
use crate::results::viewer_catalog::{
    VIEWER_DOCUMENTS, ViewerArt, ViewerCapabilities, ViewerCompatibility, ViewerDocumentDefinition,
    ViewerGroup, viewer_compatibility, viewer_document,
};
use crate::results::visualization_document::{
    AxisOrientation, AxisScale, ColumnRole, DocumentEdit, EntityRef, LinkKind,
    MAX_SOURCE_CELLS_PER_DATASET, MAX_SOURCE_ROWS, NewAxis, NewPane, PageLayout, PageUpdatePolicy,
    PaneDataBinding, PaneKind, PanePlacement, ResultDocumentTracking, ResultDocumentTrackingMode,
    SourceColumn, SourceDataset, SourceRow, TypedValue, ValueType, VisualizationDocument,
    VisualizationError,
};
use crate::state::workspace::VisualizationDocumentPersistenceError;
use crate::state::{AnalysisResult, AnalysisType, SimulationRun, SimulationRunLifecycle};
use crate::ui::tokens::Tokens;
use crate::workbench::state::{CreateResultDocumentDialogState, Workspace, WorkspaceDocumentId};
use crate::workbench::{AppState, RSpiceApp, ResultViewer};

const MAX_DOCUMENT_NAME_BYTES: usize = 120;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ResultDocumentFamily {
    WaveformWorksheet,
    FrequencyAndStability,
    RfAndNetwork,
    StatisticsAndYield,
    DigitalAndAmsEvents,
    FieldsAndPhysical,
    Photonics,
    ReportPage,
}

impl ResultDocumentFamily {
    const ALL: [Self; 8] = [
        Self::WaveformWorksheet,
        Self::FrequencyAndStability,
        Self::RfAndNetwork,
        Self::StatisticsAndYield,
        Self::DigitalAndAmsEvents,
        Self::FieldsAndPhysical,
        Self::Photonics,
        Self::ReportPage,
    ];

    const fn id(self) -> &'static str {
        match self {
            Self::WaveformWorksheet => "waveform-worksheet",
            Self::FrequencyAndStability => "frequency-stability",
            Self::RfAndNetwork => "rf-network",
            Self::StatisticsAndYield => "statistics-yield",
            Self::DigitalAndAmsEvents => "digital-ams-events",
            Self::FieldsAndPhysical => "fields-physical",
            Self::Photonics => "photonics",
            Self::ReportPage => "report-page",
        }
    }

    const fn label(self) -> &'static str {
        match self {
            Self::WaveformWorksheet => "Waveform worksheet",
            Self::FrequencyAndStability => "Frequency & stability",
            Self::RfAndNetwork => "RF & network",
            Self::StatisticsAndYield => "Statistics & yield",
            Self::DigitalAndAmsEvents => "Digital & AMS events",
            Self::FieldsAndPhysical => "Fields & physical",
            Self::Photonics => "Photonics",
            Self::ReportPage => "Report page",
        }
    }

    const fn description(self) -> &'static str {
        match self {
            Self::WaveformWorksheet => "Time, DC, parametric and linked-pane review",
            Self::FrequencyAndStability => "Bode, noise, transfer, Nyquist and pole-zero",
            Self::RfAndNetwork => "Smith, polar, spectra, sidebands and load pull",
            Self::StatisticsAndYield => "Histogram, CDF, scatter, wafer and sensitivity",
            Self::DigitalAndAmsEvents => "Logic, buses, assertions and analog correlation",
            Self::FieldsAndPhysical => "EM, current density, voltage drop, thermal and mesh views",
            Self::Photonics => "Optical spectra, transfer and mode profiles",
            Self::ReportPage => "Reviewable plots, tables, equations and provenance",
        }
    }

    fn from_id(id: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|family| family.id() == id)
    }

    /// Resolve the family a persistent page belongs to. Pages are titled with
    /// the family label at creation; a page the user has renamed, or one
    /// imported from another build, resolves to `None` and is scoped by its own
    /// retained panes instead.
    pub(super) fn from_label(label: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|family| family.label() == label)
    }

    /// Which workspace sheets this family's docbar offers.
    ///
    /// Every viewer document [`Self::includes`] admits has to be offered here:
    /// that is the same list the Create dialog binds a new document's first
    /// pane from, so a docbar refusing it would strand the document RSpice had
    /// just built with no reachable sheet. What follows are quick modes — they
    /// read the bound dataset through a different sheet without introducing a
    /// pane type the family does not compose.
    pub(super) fn offers_sheet(self, viewer: ResultViewer) -> bool {
        // Dataset-native sheets are evidence the bound dataset either carries
        // or it does not, never one of a family's plot modes. No family claims
        // or excludes them; `viewer_availability` is their only gate.
        let Some(document_id) = viewer.viewer_document_id() else {
            return true;
        };
        if viewer_document(document_id).is_some_and(|document| self.includes(document)) {
            return true;
        }
        match self {
            // Exact samples and the scalar DC gains behind them are how a
            // waveform review is checked; neither adds a pane to the sheet.
            Self::WaveformWorksheet => {
                matches!(viewer, ResultViewer::TransferFunction | ResultViewer::Table)
            }
            _ => false,
        }
    }

    fn includes(self, viewer: &ViewerDocumentDefinition) -> bool {
        match self {
            Self::WaveformWorksheet => matches!(
                viewer.id,
                "viewer-waveform"
                    | "viewer-spectrogram"
                    | "viewer-tdr"
                    | "eye-viewer"
                    | "bathtub-viewer"
                    | "margin-viewer"
                    | "dynamic-droop-viewer"
            ),
            Self::FrequencyAndStability => matches!(
                viewer.id,
                "viewer-bode"
                    | "viewer-phase-noise"
                    | "viewer-transfer-function"
                    | "viewer-network-quality"
                    | "viewer-pz"
            ),
            Self::RfAndNetwork => {
                viewer.group == ViewerGroup::RfAndNetwork
                    || matches!(viewer.id, "viewer-spectrum" | "viewer-phase-noise")
            }
            Self::StatisticsAndYield => viewer.group == ViewerGroup::StatisticalAndTabular,
            Self::DigitalAndAmsEvents => matches!(
                viewer.id,
                "viewer-waveform"
                    | "viewer-spectrogram"
                    | "eye-viewer"
                    | "bathtub-viewer"
                    | "margin-viewer"
                    | "dynamic-droop-viewer"
            ),
            Self::FieldsAndPhysical => viewer.group == ViewerGroup::FieldsAndPhysical,
            Self::Photonics => viewer.group == ViewerGroup::Photonics,
            // A report page can embed any viewer the selected dataset can
            // truthfully satisfy.
            Self::ReportPage => true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ResultDocumentLayout {
    TwoLinkedPanes,
    SinglePane,
    EngineeringGrid,
    FreeformReviewPage,
}

impl ResultDocumentLayout {
    const ALL: [Self; 4] = [
        Self::TwoLinkedPanes,
        Self::SinglePane,
        Self::EngineeringGrid,
        Self::FreeformReviewPage,
    ];

    const fn id(self) -> &'static str {
        match self {
            Self::TwoLinkedPanes => "two-linked-panes",
            Self::SinglePane => "single-pane",
            Self::EngineeringGrid => "engineering-grid-2x2",
            Self::FreeformReviewPage => "freeform-review-page",
        }
    }

    const fn label(self) -> &'static str {
        match self {
            Self::TwoLinkedPanes => "Two linked panes",
            Self::SinglePane => "Single pane",
            Self::EngineeringGrid => "2 × 2 engineering sheet",
            Self::FreeformReviewPage => "Freeform review page",
        }
    }

    fn from_id(id: &str) -> Option<Self> {
        match id {
            "two-linked-panes" => Some(Self::TwoLinkedPanes),
            "single-pane" => Some(Self::SinglePane),
            "engineering-grid-2x2" => Some(Self::EngineeringGrid),
            "freeform-review-page" => Some(Self::FreeformReviewPage),
            _ => None,
        }
    }

    const fn pane_count(self) -> usize {
        match self {
            Self::SinglePane | Self::FreeformReviewPage => 1,
            Self::TwoLinkedPanes => 2,
            Self::EngineeringGrid => 4,
        }
    }

    const fn page_layout(self) -> PageLayout {
        match self {
            Self::SinglePane => PageLayout::SinglePane,
            Self::TwoLinkedPanes => PageLayout::Columns,
            Self::EngineeringGrid => PageLayout::Grid { columns: 2 },
            // The review template owns free placement; Rows is its
            // deterministic initial flow before the user moves objects.
            Self::FreeformReviewPage => PageLayout::Rows,
        }
    }

    const fn template_id(self) -> &'static str {
        match self {
            Self::FreeformReviewPage => "review-freeform",
            Self::SinglePane | Self::TwoLinkedPanes | Self::EngineeringGrid => "engineering-dark",
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum CreateResultDocumentError {
    #[error("{0}")]
    InvalidDraft(String),
    #[error(transparent)]
    Visualization(#[from] VisualizationError),
    #[error(transparent)]
    Persistence(#[from] VisualizationDocumentPersistenceError),
}

#[derive(Debug, Clone, Copy)]
struct ResolvedDraft<'a> {
    name: &'a str,
    run: &'a SimulationRun,
    analysis: &'a AnalysisResult,
    viewer: &'static ViewerDocumentDefinition,
    family: ResultDocumentFamily,
    layout: ResultDocumentLayout,
}

/// Open a fresh, dataset-aware transaction draft.
pub(crate) fn open(app: &mut RSpiceApp) {
    let dataset_id = app
        .state
        .simulation
        .active_run()
        .or_else(|| app.state.simulation.runs.last())
        .map(|run| run.dataset_id);
    let family = ResultDocumentFamily::WaveformWorksheet;
    let viewer_id =
        first_compatible_viewer(&app.state, dataset_id, family).unwrap_or("viewer-waveform");
    let name = next_document_name(&app.state, family);
    app.state.workbench.create_result_document = CreateResultDocumentDialogState {
        open: true,
        name,
        name_touched: false,
        dataset_id,
        family_id: family.id().to_owned(),
        viewer_id: viewer_id.to_owned(),
        layout_id: ResultDocumentLayout::TwoLinkedPanes.id().to_owned(),
        validation_error: None,
    };
}

/// Render the modal transaction editor.
pub(crate) fn show(ctx: &egui::Context, app: &mut RSpiceApp) {
    if !app.state.workbench.create_result_document.open {
        return;
    }

    let mut draft = app.state.workbench.create_result_document.clone();
    let mut window_open = true;
    let mut submit = false;
    let mut cancel = false;
    let t = Tokens::get(ctx);
    let validation = resolve_draft(&app.state, &draft);
    let validation_message = validation.as_ref().err().map(ToString::to_string);
    drop(validation);

    egui::Window::new("New result document")
        .id(egui::Id::new("rspice.create-result-document"))
        .open(&mut window_open)
        .collapsible(false)
        .resizable(true)
        .default_width(920.0)
        .min_width(720.0)
        .min_height(620.0)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(ctx, |ui| {
            ui.label(
                RichText::new("RESULTS · DATASET-DRIVEN DOCUMENT")
                    .monospace()
                    .color(t.color.text_dim),
            );
            ui.add_space(4.0);
            ui.label(
                "Choose the engineering question first. RSpice selects a compatible viewer family from the immutable dataset and keeps specialist tools available inside the resulting document.",
            );
            ui.add_space(10.0);

            ui.label(RichText::new("Result document type").strong());
            for row in ResultDocumentFamily::ALL.chunks(2) {
                ui.columns(2, |columns| {
                    for (column, family) in row.iter().copied().enumerate() {
                        let selected = draft.family_id == family.id();
                        let response = family_card(&mut columns[column], family, selected);
                        if response.clicked() && !selected {
                            draft.family_id = family.id().to_owned();
                            draft.name = next_document_name(&app.state, family);
                            draft.name_touched = false;
                            draft.viewer_id =
                                first_compatible_viewer(&app.state, draft.dataset_id, family)
                                    .unwrap_or_default()
                                    .to_owned();
                            draft.validation_error = None;
                        }
                    }
                });
                ui.add_space(6.0);
            }

            result_setting_row(
                ui,
                "Dataset",
                "Document types and viewers update with the selected immutable result.",
                |ui| {
                let selected_dataset_label = draft
                    .dataset_id
                    .and_then(|id| retained_run(&app.state, id))
                    .map_or_else(|| "Select retained dataset".to_owned(), dataset_label);
                egui::ComboBox::from_id_salt("rspice.create-result-document.dataset")
                    .selected_text(selected_dataset_label)
                    .width(ui.available_width())
                    .show_ui(ui, |ui| {
                        for run in &app.state.simulation.runs {
                            let selected = draft.dataset_id == Some(run.dataset_id);
                            if ui
                                .selectable_label(selected, dataset_label(run))
                                .clicked()
                            {
                                draft.dataset_id = Some(run.dataset_id);
                                let family =
                                    ResultDocumentFamily::from_id(&draft.family_id)
                                        .unwrap_or(ResultDocumentFamily::WaveformWorksheet);
                                draft.viewer_id =
                                    first_compatible_viewer(&app.state, draft.dataset_id, family)
                                        .unwrap_or_default()
                                        .to_owned();
                                draft.validation_error = None;
                            }
                        }
                    });
                },
            );
            result_setting_row(
                ui,
                "Document layout",
                "The document stays editable without changing source samples.",
                |ui| {
                let layout = ResultDocumentLayout::from_id(&draft.layout_id)
                    .unwrap_or(ResultDocumentLayout::TwoLinkedPanes);
                egui::ComboBox::from_id_salt("rspice.create-result-document.layout")
                    .selected_text(layout.label())
                    .width(ui.available_width())
                    .show_ui(ui, |ui| {
                        for candidate in ResultDocumentLayout::ALL {
                            ui.selectable_value(
                                &mut draft.layout_id,
                                candidate.id().to_owned(),
                                candidate.label(),
                            );
                        }
                    });
                },
            );

            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new("Complete viewer catalog").strong());
                ui.separator();
                ui.label(
                    RichText::new(format!("{} canonical viewers", VIEWER_DOCUMENTS.len()))
                        .small()
                        .color(t.color.text_dim),
                );
            });

            let capabilities = draft
                .dataset_id
                .and_then(|id| retained_run(&app.state, id))
                .map(run_analysis_ids)
                .unwrap_or_default();
            egui::ScrollArea::vertical()
                .id_salt("rspice.create-result-document.viewer-catalog")
                .max_height(285.0)
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    egui::Grid::new("rspice.create-result-document.viewer-table")
                        .num_columns(3)
                        .striped(true)
                        .min_col_width(120.0)
                        .show(ui, |ui| {
                            ui.strong("Viewer / family");
                            ui.strong("Required result");
                            ui.strong("Status");
                            ui.end_row();

                            for viewer in VIEWER_DOCUMENTS {
                                let compatibility = viewer_compatibility(
                                    viewer.id,
                                    ViewerCapabilities {
                                        analysis_ids: &capabilities,
                                        external_capabilities: &[],
                                    },
                                );
                                ui.vertical(|ui| {
                                    ui.strong(viewer.title);
                                    ui.label(
                                        RichText::new(viewer.group.label())
                                            .small()
                                            .color(t.color.text_dim),
                                    );
                                });
                                ui.label(viewer_requirement(viewer));
                                let renderer_available = draft
                                    .dataset_id
                                    .and_then(|id| retained_run(&app.state, id))
                                    .is_some_and(|run| {
                                        run.analyses.iter().any(|analysis| {
                                            super::persistent_document::renderer_supports_analysis(
                                                viewer.id,
                                                analysis,
                                            )
                                        })
                                    });
                                let (status, color) =
                                    viewer_status(compatibility, renderer_available, &t);
                                ui.label(RichText::new(status).color(color));
                                ui.end_row();
                            }
                        });
                });

            ui.add_space(8.0);
            if let Some(message) = draft
                .validation_error
                .as_deref()
                .or(validation_message.as_deref())
            {
                ui.label(RichText::new(message).color(t.color.err));
            }

            ui.separator();
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                submit = ui
                    .add_enabled(
                        resolve_draft(&app.state, &draft).is_ok(),
                        egui::Button::new("Create result document"),
                    )
                    .clicked();
                cancel = ui.button("Cancel").clicked();
            });
        });

    if !window_open || cancel {
        draft.open = false;
    } else if submit {
        app.state.workbench.create_result_document = draft;
        match commit(app) {
            Ok(document_id) => {
                app.state.push_user_message(ConsoleMessage::info(format!(
                    "Created project result document {document_id}"
                )));
                return;
            }
            Err(error) => {
                app.state.workbench.create_result_document.validation_error =
                    Some(error.to_string());
                return;
            }
        }
    }
    app.state.workbench.create_result_document = draft;
}

fn family_card(ui: &mut Ui, family: ResultDocumentFamily, selected: bool) -> egui::Response {
    use crate::ui::theme::mix;

    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(vec2(ui.available_width(), 52.0), Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::RadioButton,
            ui.is_enabled(),
            selected,
            family.label(),
        )
    });
    if ui.is_rect_visible(rect) {
        let fill = if selected {
            t.color.accent_dim
        } else if response.hovered() {
            t.color.bg_hover
        } else {
            t.color.bg_panel
        };
        let border = if selected {
            t.color.accent
        } else {
            t.color.border
        };
        ui.painter().rect_filled(rect, 2.0, fill);
        ui.painter().rect_stroke(
            rect,
            2.0,
            egui::Stroke::new(1.0, border),
            egui::StrokeKind::Inside,
        );
        if selected {
            ui.painter().rect_filled(
                egui::Rect::from_min_max(
                    rect.left_top(),
                    egui::pos2(rect.left() + 3.0, rect.bottom()),
                ),
                1.0,
                t.color.accent,
            );
        }
        ui.scope_builder(
            UiBuilder::new()
                .max_rect(rect.shrink2(vec2(12.0, 7.0)))
                .layout(egui::Layout::top_down(egui::Align::Min)),
            |ui| {
                ui.spacing_mut().item_spacing.y = 2.0;
                ui.label(RichText::new(family.label()).strong().color(if selected {
                    t.color.text
                } else {
                    mix(t.color.text, t.color.text_dim, 0.15)
                }));
                ui.label(
                    RichText::new(family.description())
                        .small()
                        .color(t.color.text_dim),
                );
            },
        );
    }
    crate::ui::theme::paint_focus_ring(ui, &response, rect);
    response
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .on_hover_text(family.description())
}

fn result_setting_row(ui: &mut Ui, title: &str, detail: &str, value: impl FnOnce(&mut Ui)) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let label_width = (width * 0.46).clamp(250.0, 390.0);
    ui.horizontal(|ui| {
        ui.allocate_ui_with_layout(
            vec2(label_width, 48.0),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                ui.spacing_mut().item_spacing.y = 2.0;
                ui.label(RichText::new(title).strong());
                ui.label(RichText::new(detail).small().color(t.color.text_dim));
            },
        );
        ui.add_space(12.0);
        ui.allocate_ui_with_layout(
            vec2(ui.available_width(), 48.0),
            egui::Layout::top_down_justified(egui::Align::Center),
            value,
        );
    });
}

fn viewer_status(
    compatibility: ViewerCompatibility,
    renderer_available: bool,
    tokens: &Tokens,
) -> (&'static str, Color32) {
    if compatibility.is_compatible() && !renderer_available {
        return ("viewer integration required", tokens.color.warn);
    }
    match compatibility {
        ViewerCompatibility::Compatible => ("available", tokens.color.ok),
        ViewerCompatibility::MissingAnalysis { .. } => ("analysis required", tokens.color.warn),
        ViewerCompatibility::MissingExternalCapability { .. } => {
            ("specialist dataset required", tokens.color.warn)
        }
        ViewerCompatibility::UnknownDocument => ("unregistered", tokens.color.err),
    }
}

fn viewer_requirement(viewer: &ViewerDocumentDefinition) -> String {
    if let Some(capability) = viewer.external_capability {
        format!("{capability} result contract")
    } else if viewer.analysis_ids.is_empty() {
        "any completed dataset".to_owned()
    } else {
        viewer.analysis_ids.join(" or ").to_ascii_uppercase()
    }
}

fn retained_run(state: &AppState, dataset_id: DatasetId) -> Option<&SimulationRun> {
    state
        .simulation
        .runs
        .iter()
        .find(|run| run.dataset_id == dataset_id)
}

fn dataset_label(run: &SimulationRun) -> String {
    let lifecycle = match run.lifecycle {
        SimulationRunLifecycle::LegacyUnknown => "legacy retained",
        SimulationRunLifecycle::Preparing => "preparing",
        SimulationRunLifecycle::Running => "running",
        SimulationRunLifecycle::Cancelling => "cancelling",
        SimulationRunLifecycle::Completed => "immutable",
        SimulationRunLifecycle::Failed => "failed · partial",
        SimulationRunLifecycle::Aborted => "aborted · partial",
        SimulationRunLifecycle::Interrupted => "interrupted · partial",
    };
    format!(
        "Run {} · {} · {} analyses · {lifecycle}",
        run.id,
        run.label,
        run.analyses.len()
    )
}

fn run_analysis_ids(run: &SimulationRun) -> Vec<&'static str> {
    let mut ids = run
        .analyses
        .iter()
        .map(|analysis| analysis_manifest_id(analysis.analysis_type))
        .collect::<Vec<_>>();
    ids.sort_unstable();
    ids.dedup();
    ids
}

const fn analysis_manifest_id(analysis: AnalysisType) -> &'static str {
    match analysis {
        AnalysisType::DcOp => "op",
        AnalysisType::DcSweep | AnalysisType::Parametric => "dc",
        AnalysisType::Ac => "ac",
        AnalysisType::Disto => "disto",
        AnalysisType::Transient => "tran",
        AnalysisType::Noise => "noise",
        AnalysisType::PoleZero => "pz",
        AnalysisType::Tf => "xf",
        AnalysisType::Sensitivity => "sens",
        AnalysisType::Pac => "pac",
        AnalysisType::Pnoise => "pnoise",
        AnalysisType::Pxf => "pxf",
        AnalysisType::Pstb => "pstb",
        AnalysisType::Stb => "stb",
        AnalysisType::MonteCarlo => "mc",
        AnalysisType::Corner => "corner",
        AnalysisType::Reliability => "reliability",
        AnalysisType::Optimization => "opt",
        AnalysisType::Soa => "soa",
        AnalysisType::SParameter => "sp",
        AnalysisType::Envelope => "envelope",
        AnalysisType::Fourier => "fourier",
        AnalysisType::HarmonicBalance => "hb",
        AnalysisType::Pss => "pss",
        AnalysisType::Qpss => "qpss",
        AnalysisType::Hbsp => "hbsp",
        AnalysisType::Hbnoise => "hbnoise",
        AnalysisType::Psp => "psp",
        AnalysisType::Qpac => "qpac",
        AnalysisType::Qpnoise => "qpnoise",
        AnalysisType::Qpxf => "qpxf",
        AnalysisType::TransientNoise => "tnoise",
        AnalysisType::DcMismatch => "dcmatch",
    }
}

fn first_compatible_viewer(
    state: &AppState,
    dataset_id: Option<DatasetId>,
    family: ResultDocumentFamily,
) -> Option<&'static str> {
    let run = dataset_id.and_then(|id| retained_run(state, id))?;
    let analysis_ids = run_analysis_ids(run);
    VIEWER_DOCUMENTS
        .iter()
        .find(|viewer| {
            family.includes(viewer)
                && viewer_compatibility(
                    viewer.id,
                    ViewerCapabilities {
                        analysis_ids: &analysis_ids,
                        external_capabilities: &[],
                    },
                )
                .is_compatible()
                && run.analyses.iter().any(|analysis| {
                    super::persistent_document::renderer_supports_analysis(viewer.id, analysis)
                })
        })
        .map(|viewer| viewer.id)
}

fn next_document_name(state: &AppState, family: ResultDocumentFamily) -> String {
    for sequence in 1..=u32::MAX {
        let candidate = format!("{} · {sequence:02}", family.label());
        if !state
            .workspace
            .visualization_documents
            .iter()
            .any(|document| document.title().eq_ignore_ascii_case(&candidate))
        {
            return candidate;
        }
    }
    family.label().to_owned()
}

fn resolve_draft<'a>(
    state: &'a AppState,
    draft: &'a CreateResultDocumentDialogState,
) -> Result<ResolvedDraft<'a>, CreateResultDocumentError> {
    let name = draft.name.as_str();
    if name.trim().is_empty() {
        return Err(CreateResultDocumentError::InvalidDraft(
            "Enter a document name.".to_owned(),
        ));
    }
    if name != name.trim() {
        return Err(CreateResultDocumentError::InvalidDraft(
            "Document names cannot begin or end with whitespace.".to_owned(),
        ));
    }
    if name.len() > MAX_DOCUMENT_NAME_BYTES {
        return Err(CreateResultDocumentError::InvalidDraft(format!(
            "Document names cannot exceed {MAX_DOCUMENT_NAME_BYTES} UTF-8 bytes."
        )));
    }
    if name.chars().any(char::is_control) {
        return Err(CreateResultDocumentError::InvalidDraft(
            "Document names cannot contain control characters.".to_owned(),
        ));
    }
    if state
        .workspace
        .visualization_documents
        .iter()
        .any(|document| document.title().eq_ignore_ascii_case(name))
    {
        return Err(CreateResultDocumentError::InvalidDraft(format!(
            "A result document named {name:?} already exists."
        )));
    }

    let family = ResultDocumentFamily::from_id(&draft.family_id).ok_or_else(|| {
        CreateResultDocumentError::InvalidDraft(
            "Select a registered result-document family.".to_owned(),
        )
    })?;
    let layout = ResultDocumentLayout::from_id(&draft.layout_id).ok_or_else(|| {
        CreateResultDocumentError::InvalidDraft(
            "Select a registered result-document layout.".to_owned(),
        )
    })?;
    let dataset_id = draft.dataset_id.ok_or_else(|| {
        CreateResultDocumentError::InvalidDraft("Select a retained result dataset.".to_owned())
    })?;
    let run = retained_run(state, dataset_id).ok_or_else(|| {
        CreateResultDocumentError::InvalidDraft(
            "The selected result dataset is no longer retained.".to_owned(),
        )
    })?;
    if matches!(
        run.lifecycle,
        SimulationRunLifecycle::Preparing
            | SimulationRunLifecycle::Running
            | SimulationRunLifecycle::Cancelling
    ) {
        return Err(CreateResultDocumentError::InvalidDraft(
            "A result document requires a retained, non-running dataset.".to_owned(),
        ));
    }
    if run.analyses.is_empty() {
        return Err(CreateResultDocumentError::InvalidDraft(
            "The selected dataset contains no retained analyses.".to_owned(),
        ));
    }

    let viewer = viewer_document(&draft.viewer_id).ok_or_else(|| {
        CreateResultDocumentError::InvalidDraft("Select a canonical viewer type.".to_owned())
    })?;
    if !family.includes(viewer) {
        return Err(CreateResultDocumentError::InvalidDraft(format!(
            "{} is not part of the {} document family.",
            viewer.title,
            family.label()
        )));
    }
    let analysis_ids = run_analysis_ids(run);
    match viewer_compatibility(
        viewer.id,
        ViewerCapabilities {
            analysis_ids: &analysis_ids,
            external_capabilities: &[],
        },
    ) {
        ViewerCompatibility::Compatible => {}
        ViewerCompatibility::MissingAnalysis {
            accepted_analysis_ids,
        } => {
            return Err(CreateResultDocumentError::InvalidDraft(format!(
                "{} requires {} analysis data.",
                viewer.title,
                accepted_analysis_ids.join(" / ").to_ascii_uppercase()
            )));
        }
        ViewerCompatibility::MissingExternalCapability { capability_id } => {
            return Err(CreateResultDocumentError::InvalidDraft(format!(
                "{} requires a qualified {capability_id} specialist dataset.",
                viewer.title
            )));
        }
        ViewerCompatibility::UnknownDocument => {
            return Err(CreateResultDocumentError::InvalidDraft(
                "The selected viewer is not registered.".to_owned(),
            ));
        }
    }
    let analysis = run.analyses.iter().find(|analysis| {
        (viewer.analysis_ids.is_empty()
            || viewer
                .analysis_ids
                .contains(&analysis_manifest_id(analysis.analysis_type)))
            && super::persistent_document::renderer_supports_analysis(viewer.id, analysis)
    })
    .ok_or_else(|| {
        CreateResultDocumentError::InvalidDraft(format!(
            "{} cannot render any retained analysis in the selected dataset without substituting a different result contract.",
            viewer.title
        ))
    })?;

    Ok(ResolvedDraft {
        name,
        run,
        analysis,
        viewer,
        family,
        layout,
    })
}

fn source_analysis_identity(run: &SimulationRun, analysis: &AnalysisResult) -> AnalysisInstanceId {
    analysis.provenance().map_or_else(
        || {
            let name = format!("legacy-analysis-v1/{}", analysis.id);
            AnalysisInstanceId::from_namespace(run.dataset_id.as_uuid(), name.as_bytes())
        },
        |provenance| provenance.source_instance_id(),
    )
}

pub(super) fn source_dataset(
    run: &SimulationRun,
    analysis: &AnalysisResult,
) -> Result<SourceDataset, VisualizationError> {
    let binding = DatasetBinding::new(run.dataset_id, run.dataset_content_digest());
    if analysis.waveforms.is_empty() {
        return SourceDataset::new(
            binding,
            vec![
                SourceColumn::new(
                    "analysis",
                    "Analysis",
                    ValueType::Text,
                    ColumnRole::Coordinate,
                    None,
                )?,
                SourceColumn::new(
                    "retained-items",
                    "Retained items",
                    ValueType::Integer,
                    ColumnRole::Signal,
                    None,
                )?,
            ],
            vec![SourceRow::new(vec![
                TypedValue::Text(format!(
                    "{} · {}",
                    analysis_manifest_id(analysis.analysis_type).to_ascii_uppercase(),
                    analysis.id
                )),
                TypedValue::Integer(i64::try_from(analysis.measurements.len()).unwrap_or(i64::MAX)),
            ])],
        );
    }

    let columns = vec![
        SourceColumn::new(
            "trace-index",
            "Trace index",
            ValueType::Integer,
            ColumnRole::Coordinate,
            None,
        )?,
        SourceColumn::new(
            "trace-name",
            "Trace",
            ValueType::Text,
            ColumnRole::Coordinate,
            None,
        )?,
        SourceColumn::new(
            "component",
            "Component",
            ValueType::Text,
            ColumnRole::Coordinate,
            None,
        )?,
        SourceColumn::new(
            "sample",
            "Sample",
            ValueType::Integer,
            ColumnRole::Coordinate,
            None,
        )?,
        SourceColumn::new("x", "X", ValueType::Real, ColumnRole::Signal, None)?,
        SourceColumn::new("y", "Y", ValueType::Real, ColumnRole::Signal, None)?,
    ];
    let maximum_rows = MAX_SOURCE_ROWS.min(MAX_SOURCE_CELLS_PER_DATASET / columns.len());
    let projected_rows = analysis
        .waveforms
        .iter()
        .try_fold(0_usize, |total, waveform| {
            let display = waveform.x.len().min(waveform.y.len());
            let complex = waveform.complex.as_ref().map_or(0, |components| {
                waveform
                    .x
                    .len()
                    .min(components.real.len())
                    .saturating_add(waveform.x.len().min(components.imag.len()))
            });
            total.checked_add(display)?.checked_add(complex)
        });
    let projected_rows = projected_rows.ok_or_else(|| VisualizationError::InvalidValue {
        field: "source-dataset.rows",
        message: "retained source row count overflowed the supported address space".to_owned(),
    })?;
    if projected_rows > maximum_rows {
        return Err(VisualizationError::InvalidValue {
            field: "source-dataset.rows",
            message: format!(
                "the selected analysis projects {projected_rows} exact rows; this document format supports {maximum_rows}"
            ),
        });
    }

    let mut rows = Vec::with_capacity(projected_rows);
    for (trace_index, waveform) in analysis.waveforms.iter().enumerate() {
        append_component_rows(
            &mut rows,
            trace_index,
            &waveform.name,
            "display",
            &waveform.x,
            &waveform.y,
        )?;
        if let Some(complex) = &waveform.complex {
            append_component_rows(
                &mut rows,
                trace_index,
                &waveform.name,
                "real",
                &waveform.x,
                &complex.real,
            )?;
            append_component_rows(
                &mut rows,
                trace_index,
                &waveform.name,
                "imaginary",
                &waveform.x,
                &complex.imag,
            )?;
        }
    }
    SourceDataset::new(binding, columns, rows)
}

fn append_component_rows(
    rows: &mut Vec<SourceRow>,
    trace_index: usize,
    trace_name: &str,
    component: &str,
    x: &[f64],
    y: &[f64],
) -> Result<(), VisualizationError> {
    let trace_index = i64::try_from(trace_index).map_err(|_| VisualizationError::InvalidValue {
        field: "source-dataset.trace-index",
        message: "trace index exceeds the supported signed integer range".to_owned(),
    })?;
    for (sample, (&x, &y)) in x.iter().zip(y).enumerate() {
        if !x.is_finite() || !y.is_finite() {
            return Err(VisualizationError::InvalidValue {
                field: "source-dataset.samples",
                message: format!(
                    "trace {trace_name:?} contains a non-finite {component} sample at index {sample}"
                ),
            });
        }
        rows.push(SourceRow::new(vec![
            TypedValue::Integer(trace_index),
            TypedValue::Text(trace_name.to_owned()),
            TypedValue::Text(component.to_owned()),
            TypedValue::Integer(i64::try_from(sample).map_err(|_| {
                VisualizationError::InvalidValue {
                    field: "source-dataset.sample",
                    message: "sample index exceeds the supported signed integer range".to_owned(),
                }
            })?),
            TypedValue::Real(x),
            TypedValue::Real(y),
        ]));
    }
    Ok(())
}

fn pane_kind(art: ViewerArt) -> PaneKind {
    match art {
        ViewerArt::Smith => PaneKind::Smith,
        ViewerArt::Polar => PaneKind::Polar,
        ViewerArt::Histogram => PaneKind::Histogram,
        ViewerArt::Table => PaneKind::Table,
        ViewerArt::Wave
        | ViewerArt::Bode
        | ViewerArt::Spectrum
        | ViewerArt::Phase
        | ViewerArt::Field
        | ViewerArt::Contour
        | ViewerArt::Wireless
        | ViewerArt::Scatter
        | ViewerArt::Eye
        | ViewerArt::Bathtub
        | ViewerArt::Margin
        | ViewerArt::PoleZero
        | ViewerArt::Thermal
        | ViewerArt::Mesh => PaneKind::Cartesian,
    }
}

fn build_document(
    resolved: ResolvedDraft<'_>,
    tracking: ResultDocumentTracking,
) -> Result<VisualizationDocument, VisualizationError> {
    let source = source_dataset(resolved.run, resolved.analysis)?;
    let binding = PaneDataBinding {
        analysis_id: source_analysis_identity(resolved.run, resolved.analysis),
        dataset: source.binding(),
    };
    let mut document = VisualizationDocument::new(resolved.name, vec![source])?;
    let page_id = document.pages()[0].id;
    let first_pane_id = document.panes()[0].id;
    document.transact(
        document.revision(),
        vec![
            DocumentEdit::SetTracking(tracking),
            DocumentEdit::Rename {
                entity: EntityRef::Page(page_id),
                value: resolved.family.label().to_owned(),
            },
            DocumentEdit::SetPageComposition {
                page_id,
                layout: resolved.layout.page_layout(),
                template_id: resolved.layout.template_id().to_owned(),
                update_policy: PageUpdatePolicy::RefreshLinkedFigures,
            },
            DocumentEdit::SetPaneSource {
                pane_id: first_pane_id,
                viewer_id: resolved.viewer.id.to_owned(),
                binding: Some(binding),
            },
            DocumentEdit::Rename {
                entity: EntityRef::Pane(first_pane_id),
                value: resolved.viewer.title.to_owned(),
            },
        ],
    )?;

    let mut pane_ids = vec![first_pane_id];
    for pane_index in 1..resolved.layout.pane_count() {
        let anchor = if resolved.layout == ResultDocumentLayout::EngineeringGrid && pane_index == 2
        {
            first_pane_id
        } else {
            *pane_ids.last().expect("the initial pane always exists")
        };
        let placement =
            if resolved.layout == ResultDocumentLayout::EngineeringGrid && pane_index % 2 == 1 {
                PanePlacement::RightOf {
                    anchor_pane_id: anchor,
                }
            } else {
                PanePlacement::Below {
                    anchor_pane_id: anchor,
                }
            };
        let receipt = document.transact(
            document.revision(),
            vec![DocumentEdit::AddBoundPane(NewPane {
                page_id,
                title: format!("{} {}", resolved.viewer.title, pane_index + 1),
                kind: pane_kind(resolved.viewer.art),
                viewer_id: resolved.viewer.id.to_owned(),
                binding: Some(binding),
                placement,
            })],
        )?;
        let pane_id = receipt
            .created
            .into_iter()
            .find_map(|entity| match entity {
                EntityRef::Pane(id) => Some(id),
                _ => None,
            })
            .expect("AddBoundPane transaction returns its stable pane identity");
        pane_ids.push(pane_id);
    }

    let mut horizontal_axes = Vec::with_capacity(pane_ids.len());
    for pane_id in pane_ids {
        let x_scale = if resolved
            .viewer
            .x_axis
            .to_ascii_lowercase()
            .contains("frequency")
        {
            AxisScale::Logarithmic
        } else {
            AxisScale::Linear
        };
        let y_scale = if resolved.viewer.y_axis.to_ascii_lowercase().contains("db") {
            AxisScale::Decibels
        } else {
            AxisScale::Linear
        };
        let receipt = document.transact(
            document.revision(),
            vec![
                DocumentEdit::AddAxis(NewAxis {
                    pane_id,
                    label: resolved.viewer.x_axis.to_owned(),
                    orientation: AxisOrientation::Horizontal,
                    scale: x_scale,
                    unit: None,
                    range: None,
                }),
                DocumentEdit::AddAxis(NewAxis {
                    pane_id,
                    label: resolved.viewer.y_axis.to_owned(),
                    orientation: AxisOrientation::VerticalLeft,
                    scale: y_scale,
                    unit: None,
                    range: None,
                }),
            ],
        )?;
        if let Some(axis_id) = receipt.created.into_iter().find_map(|entity| match entity {
            EntityRef::Axis(id) => Some(id),
            _ => None,
        }) {
            horizontal_axes.push(EntityRef::Axis(axis_id));
        }
    }
    if resolved.layout == ResultDocumentLayout::TwoLinkedPanes {
        document.transact(
            document.revision(),
            vec![DocumentEdit::AddLinkGroup {
                label: "Linked horizontal viewport".to_owned(),
                kind: LinkKind::HorizontalViewport,
                members: horizontal_axes,
            }],
        )?;
    }
    Ok(document)
}

fn document_tracking(
    runs: &[SimulationRun],
    selected_run: &SimulationRun,
    selected_analysis: &AnalysisResult,
) -> ResultDocumentTracking {
    let identities = selected_run
        .prepared_receipt()
        .and_then(|receipt| receipt.simulation_plan_id())
        .zip(
            selected_analysis
                .provenance()
                .map(|provenance| provenance.authored_source_instance_id()),
        );
    let Some((simulation_plan_id, authored_analysis_id)) = identities else {
        return ResultDocumentTracking::pinned();
    };
    let newest_for_plan = runs
        .iter()
        .find(|run| {
            run.prepared_receipt()
                .and_then(|receipt| receipt.simulation_plan_id())
                == Some(simulation_plan_id)
        })
        .is_some_and(|run| run.dataset_id == selected_run.dataset_id);
    ResultDocumentTracking::for_plan(
        if newest_for_plan {
            ResultDocumentTrackingMode::Latest
        } else {
            ResultDocumentTrackingMode::Pinned
        },
        simulation_plan_id,
        authored_analysis_id,
    )
}

/// Revalidate and commit the current draft into the project authority.
pub(crate) fn commit(app: &mut RSpiceApp) -> Result<ResultDocumentId, CreateResultDocumentError> {
    let resolved = resolve_draft(&app.state, &app.state.workbench.create_result_document)?;
    let tracking = document_tracking(&app.state.simulation.runs, resolved.run, resolved.analysis);
    let document = build_document(resolved, tracking)?;
    let document_id = app
        .state
        .workspace
        .insert_visualization_document(document)?;

    let workspace_document = WorkspaceDocumentId::VisualizationDocument(document_id);
    if app.state.workbench.current_route().surface_id() != crate::workbench::SurfaceId::Results {
        app.state.workbench.activate(Workspace::Results);
    }
    if !crate::workbench::chrome::document_bar::activate_document_by_id(
        &mut app.state,
        &workspace_document,
    ) {
        return Err(CreateResultDocumentError::InvalidDraft(
            "The created result document could not resolve its retained dataset binding."
                .to_owned(),
        ));
    }
    app.state.workbench.create_result_document.open = false;
    app.state.workbench.create_result_document.validation_error = None;
    Ok(document_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{AnalysisResult, SimulationRun, WaveformData};

    fn retained_run_with(analysis_type: AnalysisType) -> SimulationRun {
        let mut run = SimulationRun::new(7);
        run.lifecycle = SimulationRunLifecycle::Completed;
        let (name, x) = if analysis_type.is_bode_response() {
            ("|V(out)|", vec![1.0, 2.0, 3.0])
        } else if analysis_type.is_raw_frequency_curve() {
            ("V(out)", vec![1.0, 2.0, 3.0])
        } else {
            ("V(out)", vec![0.0, 0.5, 1.0])
        };
        run.analyses.push(
            AnalysisResult::new(11, analysis_type, "analysis").with_waveforms(vec![
                WaveformData::new(name, x, vec![0.0, 1.0, 0.0], "#55aaff"),
            ]),
        );
        run
    }

    #[test]
    fn all_catalog_rows_are_covered_by_family_classification() {
        for viewer in VIEWER_DOCUMENTS {
            assert!(
                ResultDocumentFamily::ALL
                    .into_iter()
                    .any(|family| family.includes(viewer)),
                "{} has no result-document family",
                viewer.id
            );
        }
    }

    /// The Create dialog binds a new document's first pane from the family's
    /// own `includes` list, and the persistent docbar scopes that page with
    /// [`ResultDocumentFamily::offers_sheet`]. A docbar refusing the pane the
    /// dialog had just bound would strand the document with no reachable sheet
    /// — which is what the digital family did while the two lists were
    /// maintained apart: it composed waveform and eye panes, then admitted
    /// neither sheet.
    #[test]
    fn every_family_offers_the_sheets_its_create_path_can_bind() {
        for family in ResultDocumentFamily::ALL {
            for viewer in ResultViewer::every() {
                let Some(document_id) = viewer.viewer_document_id() else {
                    continue;
                };
                let composes = VIEWER_DOCUMENTS
                    .iter()
                    .any(|document| document.id == document_id && family.includes(document));
                assert!(
                    !composes || family.offers_sheet(viewer),
                    "{} composes {document_id} but its docbar refuses {viewer:?}",
                    family.label()
                );
            }
        }
    }

    /// Pages are titled with their family label at creation and nothing else
    /// records the family, so this coupling is what makes the scoping work at
    /// all. If either side is renamed, every page of that family silently
    /// widens to offering all sheets.
    #[test]
    fn each_family_label_round_trips_to_the_family_it_titles_pages_with() {
        for family in ResultDocumentFamily::ALL {
            assert_eq!(
                ResultDocumentFamily::from_label(family.label()),
                Some(family)
            );
        }
    }

    #[test]
    fn compatible_filter_uses_the_selected_dataset_not_the_active_dataset() {
        let mut app = RSpiceApp::test_instance();
        let transient = retained_run_with(AnalysisType::Transient);
        let transient_id = transient.dataset_id;
        let ac = retained_run_with(AnalysisType::Ac);
        let ac_id = ac.dataset_id;
        app.state.simulation.runs = vec![transient, ac];

        assert_eq!(
            first_compatible_viewer(
                &app.state,
                Some(transient_id),
                ResultDocumentFamily::WaveformWorksheet,
            ),
            Some("viewer-waveform")
        );
        assert_eq!(
            first_compatible_viewer(
                &app.state,
                Some(ac_id),
                ResultDocumentFamily::FrequencyAndStability,
            ),
            Some("viewer-bode")
        );
    }

    #[test]
    fn commit_is_project_owned_dirty_and_stably_selected() {
        let mut app = RSpiceApp::test_instance();
        let run = retained_run_with(AnalysisType::Transient);
        let dataset_id = run.dataset_id;
        app.state.simulation.runs = vec![run];
        app.state.workbench.create_result_document = CreateResultDocumentDialogState {
            open: true,
            name: "Transient review".to_owned(),
            name_touched: true,
            dataset_id: Some(dataset_id),
            family_id: ResultDocumentFamily::WaveformWorksheet.id().to_owned(),
            viewer_id: "viewer-waveform".to_owned(),
            layout_id: ResultDocumentLayout::TwoLinkedPanes.id().to_owned(),
            validation_error: None,
        };

        let document_id = commit(&mut app).expect("document commits");

        assert!(app.state.workspace.visualization_documents_dirty);
        let document = app
            .state
            .workspace
            .visualization_document(document_id)
            .expect("project owns created document");
        assert_eq!(document.title(), "Transient review");
        assert_eq!(document.panes().len(), 2);
        assert_eq!(document.tracking(), ResultDocumentTracking::pinned());
        assert_eq!(
            app.state.workbench.documents.active(Workspace::Results),
            Some(&WorkspaceDocumentId::VisualizationDocument(document_id))
        );
        assert_eq!(
            app.state.workbench.current_route().surface_id(),
            crate::workbench::SurfaceId::Results
        );
        assert_eq!(
            app.state.ui.results.viewer,
            crate::workbench::ResultViewer::Waves
        );
    }

    #[test]
    fn duplicate_name_revalidation_leaves_project_unchanged() {
        let mut app = RSpiceApp::test_instance();
        let run = retained_run_with(AnalysisType::Transient);
        let dataset_id = run.dataset_id;
        app.state.simulation.runs = vec![run];
        let draft = CreateResultDocumentDialogState {
            open: true,
            name: "Transient review".to_owned(),
            name_touched: true,
            dataset_id: Some(dataset_id),
            family_id: ResultDocumentFamily::WaveformWorksheet.id().to_owned(),
            viewer_id: "viewer-waveform".to_owned(),
            layout_id: ResultDocumentLayout::SinglePane.id().to_owned(),
            validation_error: None,
        };
        app.state.workbench.create_result_document = draft.clone();
        commit(&mut app).expect("first document commits");
        app.state.workbench.create_result_document = draft;
        let before = app.state.workspace.visualization_documents.len();

        assert!(commit(&mut app).is_err());
        assert_eq!(app.state.workspace.visualization_documents.len(), before);
    }
}
