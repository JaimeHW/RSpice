//! A working view: how one plan's results are drawn, or one imported run's.
//!
//! It is a result document like any other, kept in the project beside the
//! saved ones and edited the same way; the document bar does not list it.
//! Each analysis has a pane that plots only the signals the reader plots.
//! Following a new run moves each pane onto the same analysis of that run
//! and each trace onto the signal of the same name there; a signal the new
//! run lacks keeps its trace, marked missing.

use super::*;

/// Whose working view a document is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum WorkingViewSource {
    /// The runs of one simulation plan.
    Plan { plan_id: SimulationPlanId },
    /// One imported run, which no plan produced.
    Imported { dataset_id: DatasetId },
}

impl WorkingViewSource {
    /// The title the view is kept under: unique in its project, and never
    /// shown, since the view is reached through its runs.
    #[must_use]
    pub fn title(self) -> String {
        match self {
            Self::Plan { plan_id } => format!("View of plan {plan_id}"),
            Self::Imported { dataset_id } => format!("View of run {dataset_id}"),
        }
    }
}

/// One analysis of a new working view: the pane that draws it and the
/// signals it plots to begin with.
#[derive(Debug, Clone, PartialEq)]
pub struct WorkingPane {
    pub title: String,
    pub kind: PaneKind,
    pub viewer_id: String,
    pub analysis_id: AnalysisInstanceId,
    pub plotted: Vec<String>,
}

impl VisualizationDocument {
    /// The working view of `source` over `dataset`, a reference to one run,
    /// with one pane for each of `panes`, stacked in their order.
    pub fn new_working_view(
        source: WorkingViewSource,
        dataset: SourceDataset,
        tracking: ResultDocumentTracking,
        panes: Vec<WorkingPane>,
    ) -> Result<Self, VisualizationError> {
        tracking.validate()?;
        let binding = dataset.binding();
        let mut document = Self::new(source.title(), vec![dataset])?;
        document.working_view = Some(source);
        document.tracking = tracking;
        let page_id = document.pages[0].id;
        // The generic constructor's unbound placeholder is not one of the
        // view's analyses.
        document.panes.clear();
        let mut created = Vec::new();
        let mut previous = None;
        for pane in panes {
            let placement = previous.map_or(PanePlacement::Primary, |anchor_pane_id| {
                PanePlacement::Below { anchor_pane_id }
            });
            let pane_id = document.add_view_pane(
                NewPane {
                    page_id,
                    title: pane.title,
                    kind: pane.kind,
                    viewer_id: pane.viewer_id,
                    binding: Some(PaneDataBinding {
                        analysis_id: pane.analysis_id,
                        dataset: binding,
                    }),
                    placement,
                    plotted: Vec::new(),
                },
                &mut created,
            )?;
            document.set_signals_plotted(pane_id, &pane.plotted, true, &mut created)?;
            previous = Some(pane_id);
        }
        document.validate()?;
        Ok(document)
    }

    /// Whose working view this is; `None` for a saved document.
    #[must_use]
    pub const fn working_view(&self) -> Option<WorkingViewSource> {
        self.working_view
    }

    /// A saved document copied whole from this working view under `title`:
    /// every pane, trace (a hidden one stays hidden), marker, axis, cursor
    /// and expression, and whether it follows its plan's newest run or stays
    /// on the run it shows.
    pub fn saved_copy(&self, title: impl Into<String>) -> Result<Self, VisualizationError> {
        if self.working_view.is_none() {
            return Err(VisualizationError::InvalidValue {
                field: "visualization-document.working-view",
                message: "only a working view is saved as a document".to_owned(),
            });
        }
        let title = title.into();
        validate_label("visualization-document.title", &title)?;
        let mut document = self.clone();
        document.id = ResultDocumentId::new();
        document.revision = ObjectRevision::INITIAL;
        document.title = title;
        document.working_view = None;
        document.validate()?;
        Ok(document)
    }

    /// The pane that draws `analysis_id`, the authored analysis every run of
    /// a plan solves under the same identity.
    #[must_use]
    pub fn pane_of_analysis(&self, analysis_id: AnalysisInstanceId) -> Option<&Pane> {
        self.panes.iter().find(|pane| {
            pane.binding
                .is_some_and(|binding| binding.analysis_id == analysis_id)
        })
    }

    /// The pane whose analysis the data browser last showed, which a run of
    /// the plan opens on.
    #[must_use]
    pub fn shown_pane(&self) -> Option<&Pane> {
        self.panes.iter().find(|pane| pane.shown)
    }

    /// A bound pane with its canonical axes and no traces: the reader
    /// chooses what it plots.
    pub(super) fn add_view_pane(
        &mut self,
        pane: NewPane,
        created: &mut Vec<EntityRef>,
    ) -> Result<PaneId, VisualizationError> {
        if pane.binding.is_none() {
            return Err(VisualizationError::InvalidValue {
                field: "pane.binding",
                message: "a view pane draws one analysis of a run".to_owned(),
            });
        }
        let viewer_id = pane.viewer_id.clone();
        let pane_id = self.create_pane(pane)?;
        created.push(EntityRef::Pane(pane_id));
        self.provision_axes(pane_id, &viewer_id, created)?;
        Ok(pane_id)
    }

    /// Plot or take off the signals `names` of the analysis `pane_id` draws.
    /// Plotting shows each one's traces, or adds a trace for a signal the
    /// pane has none of; taking off hides every trace of each. The pane's
    /// traces and its dataset's signals are each read once, however many
    /// signals the edit names.
    pub(super) fn set_signals_plotted(
        &mut self,
        pane_id: PaneId,
        names: &[String],
        plotted: bool,
        created: &mut Vec<EntityRef>,
    ) -> Result<(), VisualizationError> {
        let pane = self
            .panes
            .iter()
            .find(|pane| pane.id == pane_id)
            .ok_or(VisualizationError::EntityNotFound(EntityRef::Pane(pane_id)))?;
        let (binding, viewer_id) = (pane.binding, pane.viewer_id.clone());
        for name in names {
            validate_label("trace.signal", name)?;
        }
        let named: HashSet<&str> = names.iter().map(String::as_str).collect();
        let mut held = HashSet::new();
        for trace in &mut self.traces {
            let signal = trace.source_signal().unwrap_or(&trace.label);
            if trace.pane_id == pane_id && named.contains(signal) {
                held.insert(signal.to_owned());
                trace.visible = plotted;
            }
        }
        let mut added = HashSet::new();
        let new: Vec<&String> = names
            .iter()
            .filter(|name| !held.contains(name.as_str()) && added.insert(name.as_str()))
            .collect();
        if !plotted || new.is_empty() {
            return Ok(());
        }
        let binding = binding.ok_or_else(|| VisualizationError::InvalidValue {
            field: "pane.binding",
            message: "only a pane bound to an analysis plots its signals".to_owned(),
        })?;
        let mut traces = Vec::with_capacity(new.len());
        {
            let dataset = self.dataset_for_binding(binding.dataset)?;
            let signals = apply::SignalNames::of(dataset, binding.analysis_id, "pane's")?;
            let coordinate = dataset
                .columns
                .iter()
                .find(|column| column.role == ColumnRole::Coordinate);
            for name in new {
                let trace = if !signals.is_wide() {
                    signals
                        .predicates(name, SignalComponent::Display)
                        .map(|predicates| ("y".to_owned(), "x".to_owned(), predicates))
                } else {
                    dataset
                        .columns
                        .iter()
                        .find(|column| column.role == ColumnRole::Signal && column.label == *name)
                        .zip(coordinate)
                        .map(|(signal, coordinate)| {
                            (signal.key.clone(), coordinate.key.clone(), Vec::new())
                        })
                };
                let (signal_key, coordinate_key, row_predicates) =
                    trace.ok_or_else(|| VisualizationError::InvalidValue {
                        field: "trace.signal",
                        message: format!("the pane's analysis has no signal named {name:?}"),
                    })?;
                traces.push((name.clone(), signal_key, coordinate_key, row_predicates));
            }
        }
        let (x_axis_id, y_axis_id) = self.provision_axes(pane_id, &viewer_id, created)?;
        for (label, signal_key, coordinate_key, row_predicates) in traces {
            self.add_trace(
                NewTrace {
                    pane_id,
                    binding: binding.dataset,
                    signal_key,
                    coordinate_key,
                    row_predicates,
                    x_axis_id,
                    y_axis_id,
                    label,
                },
                created,
            )?;
        }
        Ok(())
    }

    /// Read `next`, a newer run, wherever this document reads the same
    /// analysis of another: each pane onto that analysis of `next` and each
    /// trace onto its signal of the same name, a trace whose signal `next`
    /// lacks kept and marked missing. A pane whose analysis `next` does not
    /// solve stays on the run it reads. `next` may be the run the document
    /// reads already, grown since (it kept running, or finished): it is read
    /// again in place, with the signals it has now.
    pub(super) fn follow_run(&mut self, next: SourceDataset) -> Result<(), VisualizationError> {
        let next_binding = next.binding();
        if let Some(held) = self.datasets.iter().position(|dataset| {
            dataset.binding.dataset_id == next_binding.dataset_id && dataset.binding != next_binding
        }) {
            next.validate()?;
            if self.datasets[held].run.is_none() || next.run.is_none() {
                return Err(VisualizationError::InvalidValue {
                    field: "visualization-document.datasets",
                    message: "a view follows a run reference".to_owned(),
                });
            }
            let grown_from = self.datasets[held].binding;
            self.datasets[held] = next;
            return self.rebind_consumers(
                grown_from,
                next_binding,
                |analysis| analysis,
                "grown",
                apply::RebindMode::Follow,
            );
        }
        let solved: HashSet<AnalysisInstanceId> = next
            .run
            .as_ref()
            .ok_or_else(|| VisualizationError::InvalidValue {
                field: "visualization-document.datasets",
                message: "a view follows a run reference".to_owned(),
            })?
            .analyses()
            .iter()
            .map(RunSourceAnalysis::id)
            .collect();
        let mut previous: Vec<DatasetBinding> = self
            .panes
            .iter()
            .filter_map(|pane| pane.binding)
            .filter(|binding| {
                binding.dataset.dataset_id != next_binding.dataset_id
                    && solved.contains(&binding.analysis_id)
            })
            .map(|binding| binding.dataset)
            .collect();
        previous.sort_by_key(|binding| binding.dataset_id.as_uuid());
        previous.dedup();
        if previous.is_empty() {
            return Ok(());
        }
        if self
            .datasets
            .iter()
            .any(|dataset| dataset.binding.dataset_id == next_binding.dataset_id)
        {
            self.merge_dataset_projection(next)?;
        } else {
            self.attach_dataset(next)?;
        }
        for previous in previous {
            self.rebind_consumers(
                previous,
                next_binding,
                |analysis| analysis.filter(|analysis| solved.contains(analysis)),
                "new",
                apply::RebindMode::Follow,
            )?;
            if !self.reads_dataset(previous) {
                self.drop_unreferenced_dataset(previous);
            }
        }
        Ok(())
    }

    /// Whether a pane, a trace or an expression still reads `binding`.
    fn reads_dataset(&self, binding: DatasetBinding) -> bool {
        self.panes
            .iter()
            .any(|pane| pane.binding.is_some_and(|bound| bound.dataset == binding))
            || self.traces.iter().any(|trace| trace.binding == binding)
            || self
                .expression_traces
                .iter()
                .any(|expression| expression.datasets.contains(&binding))
    }
}
