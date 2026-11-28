//! The selected analysis's editor: its header, toolbar and form, and the
//! netlist names the form offers.

use super::*;
use crate::simulation::plan::NumericOverrideOption;
use crate::workbench::state::{DraftEdit, OpenDraft, OpenField};

#[derive(Clone)]
pub(super) struct SelectedAnalysis {
    pub(super) id: AnalysisInstanceId,
    pub(super) kind: AnalysisKind,
    /// What this instance is called, resolved through the plan's fallback.
    pub(super) name: String,
    pub(super) draft: AnalysisDraft,
    pub(super) dependencies: Vec<AnalysisDependency>,
    /// What each bound prerequisite is called, so a row names the analysis it
    /// is bound to rather than its instance id.
    pub(super) dependency_names: Vec<(AnalysisInstanceId, String)>,
    pub(super) prerequisite_roles: Vec<AnalysisKind>,
    pub(super) prerequisite_candidates: Vec<(AnalysisKind, Vec<DependencyCandidate>)>,
    pub(super) lifecycle: AnalysisLifecycleState,
    pub(super) position: usize,
    pub(super) plan_length: usize,
    pub(super) issues: Vec<AnalysisPlanIssue>,
    pub(super) contextual_dependency_error: Option<String>,
    pub(super) repair_label: Option<String>,
    pub(super) configure_pss_label: Option<String>,
    pub(super) enabled: bool,
}

#[derive(Clone)]
pub(super) struct EnvelopeSourceCatalog {
    pub(super) source_digest: ContentDigest,
    pub(super) names: Vec<String>,
    pub(super) netlist_source: Option<String>,
    pub(super) diagnostic: Option<String>,
}

impl EnvelopeSourceCatalog {
    pub(super) fn dependency_repair_context(&self) -> AnalysisDependencyRepairContext {
        if let Some(diagnostic) = &self.diagnostic {
            return AnalysisDependencyRepairContext::periodic_sources_unavailable(format!(
                "the netlist's periodic sources could not be listed: {diagnostic}"
            ));
        }
        let Some(source) = &self.netlist_source else {
            return AnalysisDependencyRepairContext::periodic_sources_unavailable(
                "the netlist is not available, so its periodic sources cannot be listed",
            );
        };
        AnalysisDependencyRepairContext::exact_periodic_sources(source.clone())
            .unwrap_or_else(AnalysisDependencyRepairContext::periodic_sources_unavailable)
    }

    pub(super) fn selection_error(&self, requested: &[String]) -> Option<String> {
        if requested.is_empty() {
            return None;
        }
        if let Some(diagnostic) = &self.diagnostic {
            return Some(format!(
                "The circuit modulation-source catalog is unavailable: {diagnostic}"
            ));
        }
        let missing = requested
            .iter()
            .filter(|requested| {
                !self
                    .names
                    .iter()
                    .any(|available| available.eq_ignore_ascii_case(requested))
            })
            .cloned()
            .collect::<Vec<_>>();
        (!missing.is_empty()).then(|| {
            format!(
                "Unknown or DC-only circuit modulation source{}: {}",
                if missing.len() == 1 { "" } else { "s" },
                missing.join(", ")
            )
        })
    }

    #[cfg(test)]
    pub(super) fn exact_periodic_selection_error(&self, requested: &[String]) -> Option<String> {
        if let Some(error) = self.selection_error(requested) {
            return Some(error.replace("modulation source", "periodic tone source"));
        }
        if let Some(diagnostic) = &self.diagnostic {
            return Some(format!(
                "The circuit periodic-source catalog is unavailable: {diagnostic}"
            ));
        }
        let omitted = self
            .names
            .iter()
            .filter(|available| {
                !requested
                    .iter()
                    .any(|requested| available.eq_ignore_ascii_case(requested))
            })
            .cloned()
            .collect::<Vec<_>>();
        (!omitted.is_empty()).then(|| {
            format!(
                "PSS Tones must include every elaborated periodic source; omitted: {}",
                omitted.join(", ")
            )
        })
    }
}

fn envelope_source_catalog(ui: &Ui, model: &Model) -> EnvelopeSourceCatalog {
    let source_digest = envelope_source_catalog_input_digest(model);
    let cache_id = egui::Id::new("simulation-envelope-source-catalog");
    if let Some(cached) = ui
        .ctx()
        .data(|data| data.get_temp::<EnvelopeSourceCatalog>(cache_id))
        && cached.source_digest == source_digest
    {
        return cached;
    }

    let catalog = build_envelope_source_catalog_with_digest(model, source_digest);
    ui.ctx()
        .data_mut(|data| data.insert_temp(cache_id, catalog.clone()));
    catalog
}

pub(super) fn build_envelope_source_catalog(model: &Model) -> EnvelopeSourceCatalog {
    build_envelope_source_catalog_with_digest(model, envelope_source_catalog_input_digest(model))
}

pub(super) fn envelope_source_catalog_input_digest(model: &Model) -> ContentDigest {
    crate::simulation::controller::prepared_run::design_inspection_input_digest(model)
}

fn build_envelope_source_catalog_with_digest(
    model: &Model,
    source_digest: ContentDigest,
) -> EnvelopeSourceCatalog {
    let catalog = (|| -> Result<(Vec<String>, String), String> {
        let source =
            crate::simulation::controller::SimulationController::prepare_design_netlist_for_inspection(
                model,
            )
            .map_err(|error| error.to_string())?;
        let netlist = rspice_core::Netlist::parse(&source).map_err(|error| error.to_string())?;
        let names = rspice_core::Engine::new(rspice_core::SimulationConfig::default())
            .transient_source_names(&netlist)
            .map_err(|error| error.to_string())?;
        Ok((names, source))
    })();
    let (names, netlist_source, diagnostic) = match catalog {
        Ok((names, source)) => (names, Some(source), None),
        Err(error) => (Vec::new(), None, Some(error)),
    };
    EnvelopeSourceCatalog {
        source_digest,
        names,
        netlist_source,
        diagnostic,
    }
}

/// The elaborated node and independent-source vocabulary the noise form
/// offers, measured from the same design deck the run would execute.
#[derive(Clone)]
struct NoiseDomainCatalog {
    source_digest: ContentDigest,
    nodes: Vec<String>,
    sources: Vec<String>,
    /// Why the design could not be elaborated, when it could not be.
    diagnostic: Option<String>,
}

impl NoiseDomainCatalog {
    fn domain(&self) -> analysis_form::NoiseDomain<'_> {
        analysis_form::NoiseDomain {
            nodes: &self.nodes,
            sources: &self.sources,
            unavailable: self.diagnostic.as_deref(),
        }
    }
}

/// Elaborating a design costs a full circuit build, so the catalog is keyed by
/// the same cheap digest the periodic-source catalog uses and rebuilt only
/// when the design behind it changes.
fn noise_domain_catalog(ui: &Ui, model: &Model) -> NoiseDomainCatalog {
    let source_digest = envelope_source_catalog_input_digest(model);
    let cache_id = egui::Id::new("simulation-noise-domain-catalog");
    if let Some(cached) = ui
        .ctx()
        .data(|data| data.get_temp::<NoiseDomainCatalog>(cache_id))
        && cached.source_digest == source_digest
    {
        return cached;
    }

    let catalog = build_noise_domain_catalog_with_digest(model, source_digest);
    ui.ctx()
        .data_mut(|data| data.insert_temp(cache_id, catalog.clone()));
    catalog
}

fn build_noise_domain_catalog_with_digest(
    model: &Model,
    source_digest: ContentDigest,
) -> NoiseDomainCatalog {
    let catalog = (|| -> Result<(Vec<String>, Vec<String>), String> {
        let source =
            crate::simulation::controller::SimulationController::prepare_design_netlist_for_inspection(
                model,
            )
            .map_err(|error| error.to_string())?;
        let netlist = rspice_core::Netlist::parse(&source).map_err(|error| error.to_string())?;
        let circuit = rspice_core::Engine::new(rspice_core::SimulationConfig::default())
            .build_circuit(&netlist)
            .map_err(|error| error.to_string())?;
        // `node_names_sorted` orders by MNA index, which is deck order. A
        // picker is read by eye, so the offered prefix is alphabetical -- the
        // same case-insensitive order the elaborated source list already
        // arrives in.
        let mut nodes = circuit.node_names_sorted();
        nodes.sort_by(|left, right| {
            left.to_ascii_lowercase()
                .cmp(&right.to_ascii_lowercase())
                .then_with(|| left.cmp(right))
        });
        Ok((nodes, circuit.independent_source_names()))
    })();
    let (nodes, sources, diagnostic) = match catalog {
        Ok((nodes, sources)) => (nodes, sources, None),
        Err(error) => (Vec::new(), Vec::new(), Some(error)),
    };
    NoiseDomainCatalog {
        source_digest,
        nodes,
        sources,
        diagnostic,
    }
}

/// What the transfer-function form's "Infer from deck" action would fill, or
/// why the design offers it nothing.
///
/// Measured from the same design deck the run would execute. A design that
/// cannot be elaborated at all is a refusal like any other: the form paints
/// the reason where the offer would have been.
#[derive(Clone)]
struct TfInferenceCatalog {
    source_digest: ContentDigest,
    inference: Result<TfRunConfig, String>,
}

/// Reading a design costs a netlist build and a parse, so this is keyed by the
/// same cheap digest the noise and periodic-source catalogs use and rebuilt
/// only when the design behind it changes.
fn tf_inference_catalog(ui: &Ui, model: &Model) -> TfInferenceCatalog {
    let source_digest = envelope_source_catalog_input_digest(model);
    let cache_id = egui::Id::new("simulation-tf-inference-catalog");
    if let Some(cached) = ui
        .ctx()
        .data(|data| data.get_temp::<TfInferenceCatalog>(cache_id))
        && cached.source_digest == source_digest
    {
        return cached;
    }

    let inference = (|| -> Result<TfRunConfig, String> {
        let source =
            crate::simulation::controller::SimulationController::prepare_design_netlist_for_inspection(
                model,
            )
            .map_err(|error| error.to_string())?;
        let netlist = rspice_core::Netlist::parse(&source).map_err(|error| error.to_string())?;
        infer_tf_run_config(&netlist)
    })();
    let catalog = TfInferenceCatalog {
        source_digest,
        inference,
    };
    ui.ctx()
        .data_mut(|data| data.insert_temp(cache_id, catalog.clone()));
    catalog
}

pub(super) fn analysis_editor(
    ui: &mut Ui,
    cx: &mut Cx<'_, SimulateFeature>,
    viewport_width: f32,
    scroll_content_origin_y: f32,
    resolved_participation: &participation::PlanParticipation,
) {
    cx.state.form_committed = false;
    // A field left open on another analysis lets go before this one draws:
    // the selection moved off it. Text there that does not read takes the
    // selection back to it, as it would refuse Run.
    if let Some(held) = cx
        .state
        .open_draft
        .as_ref()
        .map(|open| open.edit.instance())
        && Some(held) != cx.state.active_analysis_instance
    {
        match drafts::commit_open_drafts(cx) {
            Ok(()) => cx.state.form_committed = true,
            Err(_) => cx.state.active_analysis_instance = Some(held),
        }
    }
    let dependency_sources = envelope_source_catalog(ui, cx.model);
    let selected = match selected_analysis(cx.model, cx.state, &dependency_sources) {
        Ok(Some(selected)) => selected,
        Ok(None) => {
            cx.state.simulation_surface_editor_anchor_y = None;
            flat_notice(ui, |ui| {
                page_kit::note_line(ui, "Add an analysis to start.", page_kit::Tone::Dim);
            });
            return;
        }
        Err(error) => {
            cx.state.simulation_surface_editor_anchor_y = None;
            record_failure(cx, "Analysis editor", &error);
            flat_notice(ui, |ui| {
                page_kit::note_line(ui, "Plan unavailable", page_kit::Tone::Error);
                page_kit::note_line(ui, &error, page_kit::Tone::Dim);
            });
            return;
        }
    };

    // The card's summary and the hop under it are the same run, read once.
    // The summary used to find its own, over a predicate that accepted a run
    // holding nothing — so a started run made the card claim a retained
    // immutable dataset while the hop beside it stayed disabled.
    let prior_run = prior_plan_run(cx.model);
    let prior_datasets = prior_run.map_or_else(
        || "No prior datasets".to_owned(),
        PriorPlanRun::contract_summary,
    );
    // The form shows what its open field holds, typed and not committed yet;
    // otherwise what the plan holds.
    let held = cx
        .state
        .open_draft
        .as_mut()
        .filter(|open| open.edit.instance() == selected.id);
    let held_option = held.as_deref().and_then(|open| match &open.edit {
        DraftEdit::AnalysisOption { option, text, .. } => Some((*option, text.clone())),
        DraftEdit::Analysis { .. } => None,
    });
    let mut draft = match held.as_deref().map(|open| &open.edit) {
        Some(DraftEdit::Analysis { draft, .. }) => (**draft).clone(),
        _ => selected.draft.clone(),
    };
    drafts::begin_form(ui, held);
    let mut action = None;
    let mut open_prior_dataset = false;
    let envelope_sources = matches!(draft, AnalysisDraft::Envelope(_) | AnalysisDraft::Pss(_))
        .then_some(&dependency_sources);
    let validation_error = analysis_validation_error(cx.model, &draft, envelope_sources);
    // Distinct fields, borrowed separately, so the projection can be written
    // into the state while the controller that reads it stays shared.
    let plan_statement = plan_statement_for(cx.model, cx.handles.run_controller, &draft);
    let mut participation_action = None;
    // The run-space forms route to the page that owns the declaration rather
    // than editing it in place. Collected here and applied below, for the same
    // reason the participation action is: the frame is already borrowing `cx`.
    let mut run_space_route = None;
    let mut option_fields = analysis_form::options::OptionFields::default();
    let mut checkpoint_action = None;

    let t = Tokens::get(ui.ctx());
    let editor_response = egui::Frame::new().fill(t.color.bg_app).show(ui, |ui| {
        ui.spacing_mut().item_spacing.y = 0.0;
        let rename_requested = analysis_form_header(ui, &selected, validation_error.as_deref());
        lifecycle_toolbar(ui, &selected, &mut action);
        analysis_contract(
            ui,
            &selected,
            &plan_statement,
            ContractDatasets {
                summary: &prior_datasets,
                sequence: prior_run.map(|prior| prior.sequence),
                open: &mut open_prior_dataset,
            },
            resolved_participation,
            &mut participation_action,
            viewport_width <= 760.0,
            &mut action,
        );
        let form_anchor_y = analysis_form_body(
            ui,
            cx.model,
            cx.wb,
            &mut draft,
            envelope_sources,
            &mut run_space_route,
            selected.id,
            held_option
                .as_ref()
                .map(|(option, text)| (*option, text.as_str())),
            &mut option_fields,
            &mut checkpoint_action,
        );
        (form_anchor_y, rename_requested)
    });
    let open_field = drafts::end_form(ui);
    if let Some(action) = checkpoint_action {
        checkpoint_exchange::apply(ui.ctx(), cx, action);
    }
    if let Some(page) = run_space_route {
        cx.state.simulation_page = page;
    }
    let (form_anchor_y, rename_requested) = editor_response.inner;
    let form_anchor_content_y = content_space_anchor(form_anchor_y, scroll_content_origin_y);
    let editor_response = editor_response.response;
    ui.painter().hline(
        editor_response.rect.x_range(),
        editor_response.rect.top(),
        Stroke::new(1.0, t.color.border_strong),
    );

    if let Some((anchor_id, anchor_y)) = cx.state.simulation_surface_editor_anchor_y.take()
        && anchor_id == selected.id
    {
        cx.state.simulation_surface_pending_scroll_delta_y +=
            editor_anchor_scroll_delta(anchor_y, form_anchor_content_y);
    }
    // Retain the selected form's content-space anchor every frame. Any edit,
    // dependency bind, enable toggle, insertion, removal, or repair that changes
    // content above it is then compensated on the next frame without maintaining
    // a fragile action whitelist.
    cx.state.simulation_surface_editor_anchor_y = Some((selected.id, form_anchor_content_y));

    if settle_fields(ui, cx, &selected, draft, open_field, option_fields.open) {
        cx.state.form_committed = true;
        ui.ctx().request_repaint();
    }
    // A switch or a chooser among the options commits at once.
    if !option_fields.edits.is_empty() {
        advanced_options::commit(cx, selected.id, &option_fields.edits);
        cx.state.form_committed = true;
        ui.ctx().request_repaint();
    }
    if open_prior_dataset
        && let Some(PriorPlanRun { index, .. }) = prior_run
        && cx.model.simulation.select_run(index)
    {
        cx.execute(ids::OPEN_RUN_IN_RESULTS);
        return;
    }
    if rename_requested {
        // Opened on what the header showed, and told which analysis it is by
        // the two facts the header's own second line states.
        cx.state.simulation_workflow = Some(SimulationWorkflowDialog::RenameAnalysis(
            RenameAnalysisDraft::for_instance(
                selected.id,
                format!("{} · {}", selected.kind.label(), selected.id),
                &selected.name,
            ),
        ));
        ui.ctx().request_repaint();
        return;
    }
    if let Some(asked) = participation_action {
        match asked {
            participation::ParticipationAction::Set(run_at) => {
                let _ = participation::set_run_at(cx, selected.id, run_at);
            }
            participation::ParticipationAction::ChoosePoints => {
                participation::open_point_picker(cx, selected.id);
            }
        }
        ui.ctx().request_repaint();
        return;
    }
    if let Some(action) = action {
        apply_analysis_action(cx, selected.id, action);
        ui.ctx().request_repaint();
    }
}

/// Apply the commit rule to what the form holds once it has drawn: keep it
/// while one of its text fields has the keyboard, commit it when that field
/// lets go, and commit a switch's or a chooser's change at once. Text that
/// does not read stays in its field with the reason, uncommitted. Whether
/// the plan or the open field changed.
fn settle_fields(
    ui: &Ui,
    cx: &mut Cx<'_, SimulateFeature>,
    selected: &SelectedAnalysis,
    working: AnalysisDraft,
    open_field: Option<OpenField>,
    open_option: Option<(NumericOverrideOption, String)>,
) -> bool {
    let instance = selected.id;
    let committed = &selected.draft;
    let held = if cx
        .state
        .open_draft
        .as_ref()
        .is_some_and(|open| open.edit.instance() == instance)
    {
        cx.state.open_draft.take()
    } else {
        None
    };
    let mut changed = false;
    // The field open before this frame lets go when the keyboard went
    // elsewhere: to nothing, or to another field.
    let let_go = held.as_ref().filter(|held| {
        open_field
            .as_ref()
            .is_none_or(|open| open.id != held.field.id)
    });
    let mut refusal = None;
    if let Some(held) = let_go {
        let still_refused = held.refusal.is_some()
            && match &held.edit {
                DraftEdit::Analysis { draft, .. } => drafts::same_draft(draft, &working),
                DraftEdit::AnalysisOption { .. } => true,
            };
        let outcome = if still_refused {
            Err(held.refusal.clone().unwrap_or_default())
        } else {
            match &held.edit {
                DraftEdit::Analysis { .. } if drafts::same_draft(&working, committed) => Ok(()),
                DraftEdit::Analysis { .. } => {
                    match drafts::new_problem(cx.model, &working, committed) {
                        None => {
                            commit_draft(cx, instance, working.clone(), Some(&held.field.label));
                            Ok(())
                        }
                        Some(problem) => Err(drafts::field_reason(&held.field.label, &problem)),
                    }
                }
                DraftEdit::AnalysisOption { option, text, .. } => {
                    drafts::commit_option(cx, instance, *option, text)
                        .map_err(|problem| drafts::field_reason(&held.field.label, &problem))
                }
            }
        };
        match outcome {
            Ok(()) => changed = true,
            Err(reason) => refusal = Some(reason),
        }
    }
    if let (Some(reason), Some(held)) = (refusal, held.as_ref()) {
        // Set aside, uncommitted and outlined, in its field. Enter there
        // keeps the keyboard in it. A move to another of the analysis's own
        // fields or a press elsewhere leaves it, so what is typed next lands
        // where it is typed; the form holds one typed option at a time, so a
        // move between an option and another field comes back to it.
        let crosses_option = open_field.is_some()
            && (open_option.is_some() || matches!(held.edit, DraftEdit::AnalysisOption { .. }));
        let entered = ui.input(|input| input.key_pressed(egui::Key::Enter));
        let edit = match &held.edit {
            DraftEdit::Analysis { .. } => DraftEdit::Analysis {
                instance,
                draft: Box::new(working),
            },
            other @ DraftEdit::AnalysisOption { .. } => other.clone(),
        };
        cx.state.open_draft = Some(OpenDraft {
            field: held.field.clone(),
            edit,
            refusal: Some(reason),
            release: false,
            refocus: entered || crosses_option,
        });
        return true;
    }
    if let Some(field) = open_field {
        // A field has the keyboard: what is typed stays here until it lets
        // go. A refusal stays on it until its text changes.
        let kept = |edit: &DraftEdit| {
            held.as_ref()
                .filter(|held| held.field.id == field.id)
                .filter(|held| match (&held.edit, edit) {
                    (
                        DraftEdit::Analysis { draft: had, .. },
                        DraftEdit::Analysis { draft: has, .. },
                    ) => drafts::same_draft(had, has),
                    (
                        DraftEdit::AnalysisOption { text: had, .. },
                        DraftEdit::AnalysisOption { text: has, .. },
                    ) => had == has,
                    _ => false,
                })
                .and_then(|held| held.refusal.clone())
        };
        let edit = match open_option {
            Some((option, text)) => DraftEdit::AnalysisOption {
                instance,
                option,
                text,
            },
            None => DraftEdit::Analysis {
                instance,
                draft: Box::new(working),
            },
        };
        let refusal = kept(&edit);
        cx.state.open_draft = Some(OpenDraft {
            field,
            edit,
            refusal,
            release: false,
            refocus: false,
        });
        return changed;
    }
    // Nothing had the keyboard: a switch, a chooser or a press changed the
    // form, and that commits at once.
    if held.is_none() && !drafts::same_draft(&working, committed) {
        commit_draft(cx, instance, working, None);
        changed = true;
    }
    changed
}

pub(super) fn editor_anchor_scroll_delta(anchor_y: f32, current_y: f32) -> f32 {
    current_y - anchor_y
}

pub(super) fn content_space_anchor(screen_y: f32, scroll_content_origin_y: f32) -> f32 {
    screen_y - scroll_content_origin_y
}

/// Draw the analysis editor's header; `true` when its one control was pressed.
fn analysis_form_header(
    ui: &mut Ui,
    selected: &SelectedAnalysis,
    validation_error: Option<&str>,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(vec2(ui.available_width(), 42.0), Sense::hover());
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    let icon_rect = Rect::from_min_size(
        egui::pos2(rect.left() + 11.0, rect.center().y - 12.0),
        Vec2::splat(24.0),
    );
    analysis_icon(selected.kind).paint(ui.painter(), icon_rect.shrink(4.0), t.color.text_dim);
    let text_left = icon_rect.right() + 9.0;
    // The workbench's own button rather than egui's, laid out right-to-left
    // from the header's trailing inset. The rect this replaces was authored
    // beside the row at 26 points, but the egui button put into it drew at its
    // own 30, so the header's control stood taller than the lifecycle row
    // directly beneath it.
    let mut trailing = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(Rect::from_min_max(
                egui::pos2(text_left, rect.top()),
                egui::pos2(rect.right() - 11.0, rect.bottom()),
            ))
            .layout(Layout::right_to_left(Align::Center)),
    );
    trailing.spacing_mut().item_spacing.x = 6.0;
    let rename = Button::new("Name\u{2026}")
        .show(&mut trailing)
        .on_hover_text("Name this analysis instance so every surface reports it by that name")
        .clicked();
    let actions_left = trailing.min_rect().left();
    // A blocked analysis says so with its mark; a production one says nothing
    // about its availability, and any other availability is named neutrally.
    let blocked = !selected.issues.is_empty()
        || selected.contextual_dependency_error.is_some()
        || validation_error.is_some();
    let status = if blocked {
        Some("preflight blocked")
    } else {
        Some(availability_label(selected.kind)).filter(|label| *label != "Production")
    };
    let status_right = actions_left - 8.0;
    let status_left = status.map_or(status_right, |status| {
        let status_font = theme::sans(tokens::FS_0, FontWeight::Regular);
        let galley = ui
            .painter()
            .layout_no_wrap(status.to_owned(), status_font, t.color.text_dim);
        let text_left = status_right - galley.size().x;
        ui.painter().galley(
            egui::pos2(text_left, rect.center().y - galley.size().y * 0.5),
            galley,
            t.color.text_dim,
        );
        if blocked {
            paint_row_mark(
                ui,
                text_left - ROW_MARK_ADVANCE,
                rect.center().y,
                Problem::Failure,
            ) - ROW_MARK_ADVANCE
        } else {
            text_left
        }
    });
    let text_right = (status_left - 10.0).max(text_left);
    // The title is the instance, the line under it is the kind. When nothing
    // has been named the two agree, which is the state every plan starts in.
    paint_text_elided_to_fit(
        ui,
        Rect::from_min_max(
            egui::pos2(text_left, rect.top() + 5.0),
            egui::pos2(text_right, rect.top() + 22.0),
        ),
        &selected.name,
        theme::sans(tokens::FS_1, FontWeight::SemiBold),
        t.color.text,
    );
    paint_text_elided_to_fit(
        ui,
        Rect::from_min_max(
            egui::pos2(text_left, rect.top() + 21.0),
            egui::pos2(text_right, rect.bottom() - 3.0),
        ),
        &format!("{} · {}", selected.kind.label(), selected.lifecycle),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
    if let Some(error) = validation_error {
        response.on_hover_text(error);
    }
    rename
}

fn analysis_icon(kind: AnalysisKind) -> WorkbenchIcon {
    match kind {
        AnalysisKind::OperatingPoint
        | AnalysisKind::PoleZero
        | AnalysisKind::Stb
        | AnalysisKind::SParameter
        | AnalysisKind::Hbsp
        | AnalysisKind::Psp
        | AnalysisKind::Optimization
        | AnalysisKind::Soa
        | AnalysisKind::DcMismatch => WorkbenchIcon::Target,
        AnalysisKind::MonteCarlo | AnalysisKind::Temperature | AnalysisKind::Corner => {
            WorkbenchIcon::Grid
        }
        AnalysisKind::Ac
        | AnalysisKind::AcData
        | AnalysisKind::DcSweep
        | AnalysisKind::Fourier
        | AnalysisKind::Fft
        | AnalysisKind::TransferFunction
        | AnalysisKind::Disto => WorkbenchIcon::Results,
        AnalysisKind::Transient
        | AnalysisKind::Noise
        | AnalysisKind::Sensitivity
        | AnalysisKind::Pss
        | AnalysisKind::Qpss
        | AnalysisKind::HarmonicBalance
        | AnalysisKind::Hbnoise
        | AnalysisKind::Pac
        | AnalysisKind::Pnoise
        | AnalysisKind::Pxf
        | AnalysisKind::Pstb
        | AnalysisKind::Qpac
        | AnalysisKind::Qpnoise
        | AnalysisKind::Qpxf
        | AnalysisKind::TransientNoise
        | AnalysisKind::Envelope => WorkbenchIcon::Simulate,
    }
}

fn lifecycle_toolbar(
    ui: &mut Ui,
    selected: &SelectedAnalysis,
    action: &mut Option<AnalysisAction>,
) {
    let t = Tokens::get(ui.ctx());
    let content_width = (ui.available_width() - 16.0).max(1.0);
    let response = egui::Frame::new()
        .fill(t.color.bg_panel)
        .inner_margin(egui::Margin::same(8))
        .show(ui, |ui| {
            ui.set_width(content_width);
            ui.spacing_mut().item_spacing.x = 6.0;
            ui.spacing_mut().item_spacing.y = 6.0;
            ui.horizontal_wrapped(|ui| {
                if Button::new("Clone").icon(Icon::Copy).show(ui).clicked() {
                    *action = Some(AnalysisAction::Clone);
                }
                if Button::new("Earlier")
                    .enabled(selected.position > 0)
                    .show(ui)
                    .clicked()
                {
                    *action = Some(AnalysisAction::Earlier(selected.position - 1));
                }
                if Button::new("Later")
                    .enabled(selected.position + 1 < selected.plan_length)
                    .show(ui)
                    .clicked()
                {
                    *action = Some(AnalysisAction::Later(selected.position + 1));
                }
                if Button::new("Validate").icon(Icon::Check).show(ui).clicked() {
                    *action = Some(AnalysisAction::Validate);
                }
                if Button::new("Remove").show(ui).clicked() {
                    *action = Some(AnalysisAction::Remove);
                }
            });
        })
        .response;
    ui.painter().hline(
        response.rect.x_range(),
        response.rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
}

fn analysis_form_body(
    ui: &mut Ui,
    model: &Model,
    wb: &WorkbenchSession,
    draft: &mut AnalysisDraft,
    envelope_sources: Option<&EnvelopeSourceCatalog>,
    route: &mut Option<crate::state::SimulationPage>,
    // The analysis's advanced options are fields of this form, and an edit to
    // one is not part of the draft: it lands on the instance's own override
    // record, through the plan transaction, after this frame has closed.
    selected: AnalysisInstanceId,
    held_option: Option<(NumericOverrideOption, &str)>,
    option_fields: &mut analysis_form::options::OptionFields,
    checkpoint_action: &mut Option<checkpoint_exchange::Action>,
) -> f32 {
    let project_revision = model.workspace.content.project.revision();
    let previous_state = model
        .simulation
        .retained
        .has_retained_op_state(project_revision, false);
    let compatible_previous_state = model
        .simulation
        .retained
        .has_retained_op_state(project_revision, true);
    let soa_violations = model
        .simulation
        .active_soa_violation_context(project_revision)
        .is_some();
    let op_context = analysis_form::OpContextAvailability {
        previous_state,
        compatible_previous_state,
        soa_violations,
    };
    // Resolved from the plan, once, and handed to the form to read. The Corner
    // and Temperature forms state the declared run space; they must state the
    // plan's, and the only way to guarantee that is to give them no copy of
    // their own to state instead.
    let run_space = analysis_form::RunSpaceContext {
        run_set: &model.sim_setup.run_set,
        reference: model.sim_setup.reference_pvt,
        // Borrowed, not `plan_payload`: that clones the whole payload, and
        // this runs once per analysis form per frame for one Copy enum.
        nominal_failure: model
            .sim_setup
            .stable_analysis_plan()
            .ok()
            .map(|plan| plan.id())
            .and_then(|plan_id| model.workspace.content.plan_data(plan_id))
            .map(|payload| payload.specification_policy.nominal_failure)
            .unwrap_or_default(),
        model_binding_count: model.sim_setup.model_bindings.len(),
        parallelism: crate::simulation::execution::execution_target_parallelism(),
    };
    // The noise and transfer-function forms ask for the same two quantities —
    // one independent source, one measured expression — so they offer the one
    // elaborated vocabulary rather than a copy each. Measuring it costs a
    // circuit build, so it is keyed by the design digest and every other
    // analysis leaves it unmeasured.
    let noise_domain = matches!(
        draft,
        AnalysisDraft::Noise(_) | AnalysisDraft::TransferFunction(_)
    )
    .then(|| noise_domain_catalog(ui, model));
    // Only the stability forms offer the drawing's loop probes, and the scan
    // allocates. Every other analysis leaves it unmeasured.
    let placed_loop_probes = if analysis_form::offers_placed_loop_probes(draft) {
        model.schematic.placed_loop_probe_names()
    } else {
        Vec::new()
    };
    // Only the S-parameter form reads the design's RF ports, and the
    // derivation walks the design's nets. Every other analysis leaves it
    // unmeasured.
    let placed_rf_ports = matches!(draft, AnalysisDraft::SParameter(_))
        .then(|| crate::simulation::placed_sources::placed_rf_ports(&model.schematic, None))
        .unwrap_or_default();
    // Only the transfer-function form offers to fill itself from the deck, and
    // reading one costs a netlist build. Every other analysis leaves it
    // unmeasured.
    let tf_inference = matches!(draft, AnalysisDraft::TransferFunction(_))
        .then(|| tf_inference_catalog(ui, model));
    let study_bases = if matches!(
        draft,
        AnalysisDraft::MonteCarlo(_)
            | AnalysisDraft::Optimization(_)
            | AnalysisDraft::Temperature(_)
            | AnalysisDraft::Corner(_)
    ) {
        model
            .sim_setup
            .analysis_plan
            .as_ref()
            .map(|plan| {
                plan.instances()
                    .iter()
                    .filter(|instance| {
                        instance.enabled()
                            && if matches!(
                                draft,
                                AnalysisDraft::Temperature(_) | AnalysisDraft::Corner(_)
                            ) {
                                instance.kind().supports_pvt_base()
                            } else {
                                instance.kind().supports_study_base()
                            }
                    })
                    .map(|instance| (instance.id(), instance.display_name().to_owned()))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    } else {
        Vec::new()
    };
    let t = Tokens::get(ui.ctx());
    let content_width = (ui.available_width() - 16.0).max(1.0);
    egui::Frame::new()
        .fill(t.color.bg_app)
        .inner_margin(egui::Margin {
            left: 8,
            right: 8,
            top: 7,
            bottom: 10,
        })
        .show(ui, |ui| {
            ui.set_width(content_width);
            ui.spacing_mut().item_spacing.y = 0.0;
            // Each kind's form gets its own widget identities. A field's
            // identity is otherwise where it sits in the grid, so the Nth
            // control of one kind's form and the Nth of another's were one
            // egui widget: they shared its retained state, and AccessKit
            // published two different controls under one node.
            ui.push_id(draft.kind(), |ui| {
                analysis_form::form(
                    ui,
                    draft,
                    model.preferences.quantity_presentation_policy(),
                    wb.number_locale,
                    envelope_sources.map_or(&[], |catalog| catalog.names.as_slice()),
                    &placed_loop_probes,
                    &placed_rf_ports,
                    noise_domain
                        .as_ref()
                        .map(NoiseDomainCatalog::domain)
                        .unwrap_or_default(),
                    tf_inference.as_ref().map(|catalog| &catalog.inference),
                    op_context,
                    &run_space,
                    &study_bases,
                    route,
                );
                // Under the analysis's own parameters, because that is what
                // they are: a bound this analysis states, in the same grid and
                // with the same label over the same well.
                if let AnalysisDraft::MonteCarlo(setup) = draft {
                    *checkpoint_action =
                        analysis_form::checkpoint_sources(ui, setup, &model.simulation, selected);
                }
                if let AnalysisDraft::Transient(setup) = draft {
                    *checkpoint_action = analysis_form::transient_checkpoint_sources(
                        ui,
                        setup,
                        &model.simulation,
                        selected,
                    )
                    .map(checkpoint_exchange::Action::Transient);
                }
                *option_fields =
                    analysis_form::options::fields(ui, model, draft, selected, held_option);
            });
        })
        .response
        .rect
        .top()
}

pub(super) fn analysis_validation_error(
    model: &Model,
    draft: &AnalysisDraft,
    envelope_sources: Option<&EnvelopeSourceCatalog>,
) -> Option<String> {
    model
        .sim_setup
        .analysis_draft_validation_error(draft)
        .or_else(|| match (draft, envelope_sources) {
            (AnalysisDraft::Envelope(setup), Some(catalog)) => setup
                .to_config()
                .ok()
                .and_then(|config| catalog.selection_error(&config.modulation_sources)),
            (AnalysisDraft::Pss(setup), Some(catalog)) => catalog
                .dependency_repair_context()
                .validate_pss_sources(setup)
                .err(),
            _ => None,
        })
}
