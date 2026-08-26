//! Specialist Models & PDKs page: bins.

use super::*;

/// Geometry findings the audit column lists before reporting the remainder.
const FINDING_ROWS: usize = 10;

/// Absolute slack on a bin boundary comparison, in metres.
///
/// The engine's own `BIN_BOUND_TOLERANCE` — `const BIN_BOUND_TOLERANCE` in
/// `rspice-core/src/engine/builder/model_resolution.rs` — which is private, as
/// is the `bin_bound_equal` that reads it. The engine publishes one geometry
/// predicate, [`overlaps_with_positive_area`][overlap], and that is the one
/// this page asks for a *finding*. It publishes nothing for the other question
/// the audit reports — two cards whose inclusive edges touch without enclosing
/// area — so that classification is computed here, at the same slack. It is
/// display-only: it moves a count on this page and nothing a simulation does.
///
/// [overlap]: rspice_core::engine::ModelBinCardGeometry::overlaps_with_positive_area
const BIN_BOUND_TOLERANCE: f64 = 1e-9;

/// One card's place in the simulator's exact executable bin family.
#[derive(Clone)]
struct BinCard {
    model: String,
    /// The card's SPICE model type, as the receipt spells it: `NMOS`, `PMOS`.
    device: String,
    geometry: rspice_core::engine::ModelBinCardGeometry,
    declaration_order: usize,
}

/// The set of cards the engine would consider for one instance reference.
///
/// Keyed by library and by the family base name — `nch` for `nch.1`, `nch.2` —
/// because that is the set core's `resolve_binned_model_def` collects
/// candidates from. Grouping by device type instead, which this page used to
/// do, put every NMOS card in every attached foundry library into one family
/// and reported them as overlapping each other.
struct BinFamily {
    library: String,
    family: String,
    cards: Vec<BinCard>,
}

impl BinFamily {
    fn key(&self) -> String {
        format!("{} · {}", self.library, self.family)
    }

    /// The device type the family's cards declare, or how many they declare.
    ///
    /// A family whose cards disagree is not a presentation problem to smooth
    /// over: the engine selects one of these cards for one instance of one
    /// device, so two device types under one base name is something the reader
    /// has to see rather than a cell that picks the first.
    fn device(&self) -> String {
        let mut kinds = self
            .cards
            .iter()
            .map(|card| card.device.as_str())
            .collect::<BTreeSet<_>>();
        match kinds.len() {
            0 => "—".to_owned(),
            1 => kinds.pop_first().unwrap_or_default().to_owned(),
            count => format!("{count} device types"),
        }
    }
}

fn bin_families(
    app: &ManagerRenderContext<'_>,
    inspection: &rspice_core::engine::ModelBinInspection,
) -> Result<Vec<BinFamily>, String> {
    let mut families = BTreeMap::<(String, String), Vec<BinCard>>::new();
    for card in &inspection.cards {
        let provider = app
            .state
            .model_library_manager
            .effective_definition_provider(ModelConsumerScope::PrimitiveModel, &card.model)?
            .ok_or_else(|| {
                format!(
                    "Prepared model-bin card '{}' has no executable provider in the project catalog",
                    card.model
                )
            })?;
        families
            .entry((provider.library, card.family.clone()))
            .or_default()
            .push(BinCard {
                model: card.model.clone(),
                device: card.model_type.clone(),
                geometry: card.geometry,
                declaration_order: card.declaration_order,
            });
    }
    Ok(families
        .into_iter()
        .map(|((library, family), mut cards)| {
            cards.sort_by_key(|card| card.declaration_order);
            BinFamily {
                library,
                family,
                cards,
            }
        })
        .collect::<Vec<_>>())
}

#[derive(Clone)]
struct ModelBinInspectionCache {
    input_digest: crate::product::ContentDigest,
    result: Result<rspice_core::engine::ModelBinInspection, String>,
}

/// Inspect the exact deck prepared for the active design and simulation plan.
/// The engine owns expression evaluation, NFIN handling, hierarchy flattening,
/// tolerance, declaration order, and instance selection; this surface only
/// presents the resulting immutable receipt.
///
/// Preparing that deck is far too expensive to repeat per frame, hence the
/// cache — but the key is asked for on every frame whether the cache hits or
/// not, so it has to be cheaper than the miss it prevents. It used to include
/// the execution catalogue digest, which serializes every library in the corpus
/// through `serde_json::Value`; at production scale this page paid the corpus
/// on every frame to decide it did not need to. See
/// [`ModelLibraryManager::design_inspection_catalog_key`][key] for the cheap
/// key that replaced it and for why hashing content — rather than a revision
/// counter a wholesale replacement could carry in with it — is what makes the
/// cached verdict expire when it must.
///
/// [key]: crate::state::model_library::ModelLibraryManager::design_inspection_catalog_key
fn authoritative_bin_inspection(
    ui: &Ui,
    app: &ManagerRenderContext<'_>,
) -> Result<rspice_core::engine::ModelBinInspection, String> {
    let input_digest =
        crate::simulation::controller::prepared_run::design_inspection_input_digest(app.state);
    let cache_id = egui::Id::new("models-authoritative-bin-inspection");
    if let Some(cached) = ui
        .ctx()
        .data(|data| data.get_temp::<ModelBinInspectionCache>(cache_id))
        && cached.input_digest == input_digest
    {
        return cached.result;
    }

    let result = (|| {
        let source = crate::simulation::controller::SimulationController::
            prepare_design_netlist_for_inspection(app.state)
            .map_err(|error| error.to_string())?;
        let netlist = rspice_core::Netlist::parse(&source).map_err(|error| error.to_string())?;
        rspice_core::Engine::new(rspice_core::SimulationConfig::default())
            .inspect_model_bins(&netlist)
            .map_err(|error| error.to_string())
    })();
    ui.ctx().data_mut(|data| {
        data.insert_temp(
            cache_id,
            ModelBinInspectionCache {
                input_digest,
                result: result.clone(),
            },
        );
    });
    result
}

pub(super) fn bins_page(ui: &mut Ui, app: &mut ManagerRenderContext<'_>) {
    let prepared = authoritative_bin_inspection(ui, app).and_then(|inspection| {
        bin_families(app, &inspection).map(|families| (inspection, families))
    });
    match prepared {
        Ok((inspection, families)) => bins_body(ui, app, &inspection, &families),
        Err(error) => bins_blocked(ui, app, &error),
    }
}

/// The page when the simulator refused to inspect this design at all.
///
/// The receipt is all-or-nothing by construction — see
/// [`rspice_core::engine::ModelBinInspection`], whose "construction fails
/// instead of returning a partial receipt" is what makes every number on the
/// healthy page evidence rather than an estimate. So an ambiguous or uncovered
/// family arrives *here*, as the whole page, and never as a row in a table
/// above: there is no receipt for that table to have come from. That is why
/// this state carries the engine's own sentence and both routes that repair it
/// rather than a bare "unavailable".
fn bins_blocked(ui: &mut Ui, app: &mut ManagerRenderContext<'_>, error: &str) {
    section_title(
        ui,
        "Bins & geometry",
        "exact inspection refused · no partial receipt is presented",
        // Outermost-right first: the band lays its actions out right to left.
        |ui| {
            if Button::new("Edit cards…").accent().show(ui).clicked() {
                app.queue_command(Command::ModelEditor);
            }
            if Button::new("Import bin map")
                .enabled(!app.state.workbench.models_view.model_import_in_progress)
                .show(ui)
                .on_hover_text(
                    "Import authenticated binned model cards; L, W, and NFIN bounds are evaluated by the simulator from the retained source.",
                )
                .clicked()
            {
                app.queue_model_source_import();
            }
        },
    );
    ui.add_space(6.0);
    ui.label(
        RichText::new(
            "The simulator inspects the exact deck this design would run and reports either every \
             card and every instance decision or nothing at all. Repair the bounds it names in \
             Model Editor, or re-import the bin map, and this page fills in.",
        )
        .small()
        .color(Tokens::get(ui.ctx()).color.text_dim),
    );
    page_empty_state(
        ui,
        "The simulator refused this design's bin inspection",
        error,
    );
}

fn bins_body(
    ui: &mut Ui,
    app: &mut ManagerRenderContext<'_>,
    inspection: &rspice_core::engine::ModelBinInspection,
    families: &[BinFamily],
) {
    let audit = BinAudit::derive(families, inspection);
    section_title(
        ui,
        "Bins & geometry",
        &format!(
            "{} binned families · {} cards · {} placed instances · {} findings",
            families.len(),
            families
                .iter()
                .map(|family| family.cards.len())
                .sum::<usize>(),
            audit.instances,
            audit.findings.len()
        ),
        // Outermost-right first: the band lays its actions out right to left.
        |ui| {
            if Button::new("Audit all families")
                .accent()
                .show(ui)
                .clicked()
            {
                receipt(
                    app,
                    if audit.findings.is_empty() {
                        Ok(
                            "Geometry audit completed with no overlapping card envelopes."
                                .to_owned(),
                        )
                    } else {
                        Err(format!(
                            "Geometry audit found {} overlapping or incomplete envelopes.",
                            audit.findings.len()
                        ))
                    },
                );
            }
            // Gated on the fact it traces — the selected model — rather than
            // on a schematic selection it never reads, and it says which.
            let traced = app.state.workbench.selected_model.clone();
            if Button::new("Trace schematic")
                .enabled(traced.is_some())
                .show(ui)
                .on_disabled_hover_text("Select a model in the catalog first.")
                .clicked()
                && let Some(model) = traced
            {
                app.state.workbench.models_view.dialog =
                    Some(ModelsWorkbenchDialog::BindingTrace {
                        consumers: effective_model_consumers(app, &model),
                        model,
                    });
            }
            if Button::new("Edit cards…").show(ui).clicked() {
                app.queue_command(Command::ModelEditor);
            }
            if Button::new("Import bin map")
                .enabled(!app.state.workbench.models_view.model_import_in_progress)
                .show(ui)
                .on_hover_text(
                    "Import authenticated binned model cards; L, W, and NFIN bounds are evaluated by the simulator from the retained source.",
                )
                .clicked()
            {
                app.queue_model_source_import();
            }
        },
    );
    if families.is_empty() {
        page_empty_state(
            ui,
            "No geometry envelopes are loaded",
            "Attach a binned PDK library or author L/W bounds in Model Editor.",
        );
        return;
    }
    bin_audit_strip(ui, &audit);

    let selected = app
        .state
        .workbench
        .models_view
        .selected_bin_family
        .clone()
        .filter(|selected| families.iter().any(|family| family.key() == *selected))
        .or_else(|| families.first().map(BinFamily::key))
        .unwrap_or_default();
    app.state.workbench.models_view.selected_bin_family = Some(selected.clone());
    let Some(family) = families.iter().find(|family| family.key() == selected) else {
        return;
    };

    // The mockup's `.bin-workspace`: `minmax(0, 1.25fr)` of map beside
    // `minmax(320px, 0.75fr)` of rail, with the instance table across the full
    // width beneath both.
    let body = ui.available_height();
    let row_h = (body * 0.55).clamp(260.0, 430.0);
    let track = ui.available_width();
    let gap = 8.0;
    let map_w = ((track - gap) * 0.625).max(240.0);
    let rail_w = (track - gap - map_w).max(1.0);
    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = gap;
        ui.allocate_ui_with_layout(
            egui::vec2(map_w, row_h),
            Layout::top_down(Align::Min),
            |ui| {
                ui.set_min_width(map_w);
                ui.set_max_width(map_w);
                geometry_map(ui, family, &audit, inspection, row_h);
            },
        );
        ui.allocate_ui_with_layout(
            egui::vec2(rail_w, row_h),
            Layout::top_down(Align::Min),
            |ui| {
                ui.set_min_width(rail_w);
                ui.set_max_width(rail_w);
                family_table(ui, app, families, &audit, &selected, row_h * 0.56);
                ui.add_space(gap);
                findings_card(ui, family, &audit);
            },
        );
    });
    ui.add_space(gap);
    geometry_instance_table(ui, app, family, &audit, inspection);
}

// ---------------------------------------------------------------------------
// The audit: one derivation, read by the strip, the map, and both tables.
// ---------------------------------------------------------------------------

/// Two cards of one family whose declared envelopes enclose a shared region.
///
/// A *successful* receipt carries no instance that resolved through such a
/// region: `resolve_binned_model_def` refuses the whole construction when two
/// matching cards overlap with positive area. So a finding here names a latent
/// defect — the region exists and nothing has landed in it yet.
struct GeometryFinding {
    family_key: String,
    left: String,
    right: String,
    /// The L/W projection of the shared region, for the map to paint. NFIN
    /// decides whether the pair overlaps at all but has no axis on this plane.
    length: rspice_core::engine::ModelBinAxisRange,
    width: rspice_core::engine::ModelBinAxisRange,
}

impl GeometryFinding {
    fn sentence(&self) -> String {
        format!(
            "{} and {} enclose a shared L/W/NFIN region",
            self.left, self.right
        )
    }
}

/// What one family's cards and instances add up to.
#[derive(Default, Clone, Copy)]
struct FamilyTally {
    findings: usize,
    /// Card pairs whose inclusive edges touch without enclosing area.
    boundaries: usize,
    instances: usize,
    /// Instances the engine settled by declaration order because more than one
    /// inclusive card met on the boundary they sit on.
    shared: usize,
}

impl FamilyTally {
    /// Instances that landed inside exactly one card.
    const fn one_card(self) -> usize {
        self.instances.saturating_sub(self.shared)
    }
}

/// Everything derived from one receipt, once, for every pane on the page.
struct BinAudit {
    findings: Vec<GeometryFinding>,
    boundaries: usize,
    instances: usize,
    one_card: usize,
    shared: usize,
    by_family: BTreeMap<String, FamilyTally>,
    /// Indices into the receipt's own instance list, per family key, in
    /// receipt order. Indices rather than clones: the receipt outlives every
    /// pane that reads it, and it holds one row per placed MOS instance.
    instances_by_family: BTreeMap<String, Vec<usize>>,
}

impl BinAudit {
    fn derive(
        families: &[BinFamily],
        inspection: &rspice_core::engine::ModelBinInspection,
    ) -> Self {
        let mut findings = Vec::new();
        let mut by_family = BTreeMap::<String, FamilyTally>::new();
        // Card name and family name both point back at the family that owns
        // them, because an instance names one or the other: a request for the
        // complete card resolves as `ExactCard`, a request for the base name
        // is resolved by geometry.
        let mut by_card = BTreeMap::<String, String>::new();
        let mut by_name = BTreeMap::<String, String>::new();
        for family in families {
            let key = family.key();
            let tally = by_family.entry(key.clone()).or_default();
            by_name
                .entry(family.family.to_ascii_lowercase())
                .or_insert_with(|| key.clone());
            for (index, left) in family.cards.iter().enumerate() {
                by_card.insert(left.model.to_ascii_lowercase(), key.clone());
                for right in family.cards.iter().skip(index + 1) {
                    // The overlap verdict is the engine's own predicate, so a
                    // finding here is exactly the condition that refuses a
                    // netlist. Only the boundary classification is local.
                    if left.geometry.overlaps_with_positive_area(right.geometry) {
                        tally.findings += 1;
                        findings.push(GeometryFinding {
                            family_key: key.clone(),
                            left: left.model.clone(),
                            right: right.model.clone(),
                            length: axis_intersection(left.geometry.length, right.geometry.length),
                            width: axis_intersection(left.geometry.width, right.geometry.width),
                        });
                    } else if shares_a_boundary(left.geometry, right.geometry) {
                        tally.boundaries += 1;
                    }
                }
            }
        }

        let mut instances_by_family = BTreeMap::<String, Vec<usize>>::new();
        for (index, instance) in inspection.instances.iter().enumerate() {
            let Some(key) = by_card
                .get(&instance.selected_model.to_ascii_lowercase())
                .or_else(|| by_name.get(&instance.requested_model.to_ascii_lowercase()))
                .cloned()
            else {
                continue;
            };
            let tally = by_family.entry(key.clone()).or_default();
            tally.instances += 1;
            if instance.selection == rspice_core::engine::ModelBinSelectionKind::SharedBoundary {
                tally.shared += 1;
            }
            instances_by_family.entry(key).or_default().push(index);
        }

        let boundaries = by_family.values().map(|tally| tally.boundaries).sum();
        let instances = by_family
            .values()
            .map(|tally| tally.instances)
            .sum::<usize>();
        let shared = by_family.values().map(|tally| tally.shared).sum::<usize>();
        Self {
            findings,
            boundaries,
            instances,
            one_card: instances.saturating_sub(shared),
            shared,
            by_family,
            instances_by_family,
        }
    }

    fn tally(&self, family_key: &str) -> FamilyTally {
        self.by_family.get(family_key).copied().unwrap_or_default()
    }

    fn findings_for<'a>(
        &'a self,
        family_key: &'a str,
    ) -> impl Iterator<Item = &'a GeometryFinding> {
        self.findings
            .iter()
            .filter(move |finding| finding.family_key == family_key)
    }

    fn instances_of(&self, family_key: &str) -> &[usize] {
        self.instances_by_family
            .get(family_key)
            .map_or(&[], Vec::as_slice)
    }
}

/// The region two axis ranges have in common, an absent bound meaning
/// unbounded — the same arithmetic the engine's own overlap predicate does.
fn axis_intersection(
    left: rspice_core::engine::ModelBinAxisRange,
    right: rspice_core::engine::ModelBinAxisRange,
) -> rspice_core::engine::ModelBinAxisRange {
    rspice_core::engine::ModelBinAxisRange {
        min: match (left.min, right.min) {
            (Some(left), Some(right)) => Some(left.max(right)),
            (left, right) => left.or(right),
        },
        max: match (left.max, right.max) {
            (Some(left), Some(right)) => Some(left.min(right)),
            (left, right) => left.or(right),
        },
    }
}

/// How two ranges meet on one axis.
#[derive(PartialEq, Eq, Clone, Copy)]
enum AxisMeeting {
    /// Nothing in common.
    Separated,
    /// One point in common: the inclusive edge both cards declare.
    Edge,
    /// A span in common.
    Area,
}

fn axis_meeting(
    left: rspice_core::engine::ModelBinAxisRange,
    right: rspice_core::engine::ModelBinAxisRange,
) -> AxisMeeting {
    let shared = axis_intersection(left, right);
    // An axis where either side is unbounded has infinite room in common,
    // which is how the engine reads it too.
    let (Some(min), Some(max)) = (shared.min, shared.max) else {
        return AxisMeeting::Area;
    };
    if max - min > BIN_BOUND_TOLERANCE {
        AxisMeeting::Area
    } else if max - min >= -BIN_BOUND_TOLERANCE {
        AxisMeeting::Edge
    } else {
        AxisMeeting::Separated
    }
}

/// Whether two cards meet on an inclusive edge without enclosing area.
///
/// This is the normal shape of a tiled family — `lmax=5e-7` beside
/// `lmin=5e-7` — and it is deterministic: `resolve_binned_model_def` takes the
/// first declared card. It is counted because it is the population the
/// "resolving to one card" ratio is measured against, not because it is wrong.
fn shares_a_boundary(
    left: rspice_core::engine::ModelBinCardGeometry,
    right: rspice_core::engine::ModelBinCardGeometry,
) -> bool {
    let axes = [
        axis_meeting(left.length, right.length),
        axis_meeting(left.width, right.width),
        axis_meeting(left.nfin, right.nfin),
    ];
    !axes.contains(&AxisMeeting::Separated) && axes.contains(&AxisMeeting::Edge)
}

// ---------------------------------------------------------------------------
// Painters
// ---------------------------------------------------------------------------

/// The mockup's `.bin-audit-strip` (`min-height: 44px`).
const AUDIT_STRIP_H: f32 = 44.0;

/// What a successful receipt says about every binned card in the design.
///
/// Every cell is a count the receipt supports. The two the mockup carried that
/// it cannot — "N ambiguous regions" and "N uncovered regions" over a live
/// table — are impossible by construction: either being non-zero is exactly the
/// condition that stops the receipt existing, so that case is the whole page
/// ([`bins_blocked`]) rather than a cell in this strip.
fn bin_audit_strip(ui: &mut Ui, audit: &BinAudit) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), AUDIT_STRIP_H),
        Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        Stroke::new(1.0, t.color.border),
    );

    let clean = audit.findings.is_empty();
    let cells: [(&str, String, egui::Color32); 4] = [
        (
            "BIN AUDIT",
            if clean {
                "clean".to_owned()
            } else {
                "faults".to_owned()
            },
            if clean { t.color.ok } else { t.color.err },
        ),
        (
            "overlapping envelopes",
            audit.findings.len().to_string(),
            if clean { t.color.ok } else { t.color.err },
        ),
        (
            "shared boundaries",
            audit.boundaries.to_string(),
            t.color.text,
        ),
        (
            "instances resolving to one card",
            format!("{} / {}", audit.one_card, audit.instances),
            if audit.shared == 0 {
                t.color.ok
            } else {
                t.color.warn
            },
        ),
    ];
    // The rule those numbers are measured under, hard against the right, read
    // out of the engine rather than restated: `bin_range_contains` is
    // inclusive at both ends and `resolve_binned_model_def` takes
    // `matches.first()`. The cell is one line wide, so the strip carries the
    // short form and the announcement below carries the whole sentence.
    const RULE: &str = "inclusive edges · first match";
    const RULE_SENTENCE: &str =
        "bin ranges are inclusive at both edges, and the first declared matching card wins";
    let rule_width = 212.0f32.min(rect.width() * 0.32);
    let cell_width = ((rect.width() - rule_width) / cells.len() as f32).max(1.0);
    for (index, (caption, value, color)) in cells.iter().enumerate() {
        let cell = egui::Rect::from_min_max(
            egui::pos2(rect.left() + cell_width * index as f32, rect.top()),
            egui::pos2(rect.left() + cell_width * (index + 1) as f32, rect.bottom()),
        );
        if index > 0 {
            ui.painter().vline(
                cell.left(),
                cell.y_range().shrink(8.0),
                Stroke::new(1.0, t.color.border),
            );
        }
        // Caption above the number, the way every other metric strip in this
        // workspace reads: the reader is looking for a quantity and has to
        // know which one before the digits mean anything.
        ui.painter().text(
            egui::pos2(cell.left() + 10.0, cell.top() + 13.0),
            egui::Align2::LEFT_CENTER,
            elide(ui, caption, cell.width() - 20.0, false),
            theme::sans(tokens::FS_0, FontWeight::SemiBold),
            t.color.text_faint,
        );
        ui.painter().text(
            egui::pos2(cell.left() + 10.0, cell.bottom() - 13.0),
            egui::Align2::LEFT_CENTER,
            elide(ui, value, cell.width() - 20.0, true),
            theme::mono(tokens::FS_1, FontWeight::SemiBold),
            *color,
        );
    }
    ui.painter().text(
        egui::pos2(rect.right() - 10.0, rect.top() + 13.0),
        egui::Align2::RIGHT_CENTER,
        "bin rule",
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text_faint,
    );
    ui.painter().text(
        egui::pos2(rect.right() - 10.0, rect.bottom() - 13.0),
        egui::Align2::RIGHT_CENTER,
        elide(ui, RULE, rule_width - 20.0, true),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );

    // Every glyph above is painter text, which publishes nothing. This strip is
    // the page's headline verdict, so it is announced whole.
    let announcement = format!(
        "Bin audit: {}. {} overlapping envelopes, {} shared boundaries, {} of {} instances \
         resolving to one card. Bin rule: {RULE_SENTENCE}.",
        if clean { "clean" } else { "faults" },
        audit.findings.len(),
        audit.boundaries,
        audit.one_card,
        audit.instances,
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), &announcement)
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Label);
        node.set_label(announcement.clone());
    });
}

/// The map's footer legend (`.bin-map-foot`).
const MAP_LEGEND_H: f32 = 20.0;

fn geometry_map(
    ui: &mut Ui,
    family: &BinFamily,
    audit: &BinAudit,
    inspection: &rspice_core::engine::ModelBinInspection,
    height: f32,
) {
    let key = family.key();
    let tally = audit.tally(&key);
    card(ui, |ui| {
        card_title(
            ui,
            "LOG L/W MAP",
            Some(&format!(
                "{} · {} cards · {}",
                family.family,
                family.cards.len(),
                if tally.findings == 0 {
                    "geometry clean".to_owned()
                } else {
                    format!("{} geometry findings", tally.findings)
                }
            )),
        );
        let plot_h = (height - MAP_LEGEND_H - 48.0).max(140.0);
        let (rect, response) =
            ui.allocate_exact_size(egui::vec2(ui.available_width(), plot_h), Sense::hover());
        let t = Tokens::get(ui.ctx());
        ui.painter().rect(
            rect,
            2.0,
            t.color.bg_inset,
            Stroke::new(1.0, t.color.border),
            egui::StrokeKind::Inside,
        );
        // The left gutter is wider than the others because the W axis writes
        // its decades outside the plot, right-aligned against it: a symmetric
        // inset clipped `100µm` to `00µm` against the card's own edge.
        let plot = egui::Rect::from_min_max(
            rect.min + egui::vec2(46.0, 16.0),
            rect.max - egui::vec2(16.0, 22.0),
        );
        ui.painter().line_segment(
            [plot.left_bottom(), plot.right_bottom()],
            Stroke::new(1.0, t.color.border_strong),
        );
        ui.painter().line_segment(
            [plot.left_bottom(), plot.left_top()],
            Stroke::new(1.0, t.color.border_strong),
        );
        let finite = family
            .cards
            .iter()
            .filter_map(|card| {
                Some((
                    card.geometry.length.min?.max(f64::MIN_POSITIVE).log10(),
                    card.geometry.length.max?.max(f64::MIN_POSITIVE).log10(),
                    card.geometry.width.min?.max(f64::MIN_POSITIVE).log10(),
                    card.geometry.width.max?.max(f64::MIN_POSITIVE).log10(),
                    card.model.as_str(),
                    card.declaration_order,
                ))
            })
            .collect::<Vec<_>>();
        let min_l = finite
            .iter()
            .map(|value| value.0)
            .reduce(f64::min)
            .unwrap_or(-9.0);
        let max_l = finite
            .iter()
            .map(|value| value.1)
            .reduce(f64::max)
            .unwrap_or(-3.0);
        let min_w = finite
            .iter()
            .map(|value| value.2)
            .reduce(f64::min)
            .unwrap_or(-9.0);
        let max_w = finite
            .iter()
            .map(|value| value.3)
            .reduce(f64::max)
            .unwrap_or(-3.0);
        let map_x = |value: f64| {
            plot.left()
                + plot.width()
                    * (((value - min_l) / (max_l - min_l).max(1e-12)) as f32).clamp(0.0, 1.0)
        };
        let map_y = |value: f64| {
            plot.bottom()
                - plot.height()
                    * (((value - min_w) / (max_w - min_w).max(1e-12)) as f32).clamp(0.0, 1.0)
        };
        // One neutral fill for every card, so the only colour on this plane is
        // the tone that means something. The five-colour cycle this used to
        // paint made a clean family read as four different verdicts.
        for (l0, l1, w0, w1, name, order) in &finite {
            let bin = egui::Rect::from_min_max(
                egui::pos2(map_x(*l0), map_y(*w1)),
                egui::pos2(map_x(*l1), map_y(*w0)),
            );
            ui.painter().rect(
                bin,
                1.0,
                t.color.text.linear_multiply(0.06),
                Stroke::new(0.8, t.color.border_strong),
                egui::StrokeKind::Inside,
            );
            // The declaration index is the tie-break the strip just stated, so
            // it is what makes that rule legible on the plot; the card's own
            // name follows underneath wherever the cell has room for it.
            ui.painter().text(
                bin.center() - egui::vec2(0.0, 6.0),
                egui::Align2::CENTER_CENTER,
                format!("{}", order + 1),
                theme::mono(tokens::FS_0, FontWeight::SemiBold),
                t.color.text_dim,
            );
            let label = elide(ui, name, bin.width() - 6.0, true);
            if bin.height() > 26.0 && !label.is_empty() {
                ui.painter().text(
                    bin.center() + egui::vec2(0.0, 7.0),
                    egui::Align2::CENTER_CENTER,
                    label,
                    theme::mono(tokens::FS_0, FontWeight::Regular),
                    t.color.text_faint,
                );
            }
        }
        // Where two envelopes enclose a shared region, that region — not either
        // whole card — carries the tone, because that is the part of the plane
        // a placed instance must not land in.
        for finding in audit.findings_for(&key) {
            let decade = |bound: Option<f64>, fallback: f64| {
                bound.map_or(fallback, |bound| bound.max(f64::MIN_POSITIVE).log10())
            };
            let region = egui::Rect::from_min_max(
                egui::pos2(
                    map_x(decade(finding.length.min, min_l)),
                    map_y(decade(finding.width.max, max_w)),
                ),
                egui::pos2(
                    map_x(decade(finding.length.max, max_l)),
                    map_y(decade(finding.width.min, min_w)),
                ),
            );
            ui.painter().rect(
                region,
                0.0,
                t.color.err.linear_multiply(0.26),
                Stroke::new(1.0, t.color.err),
                egui::StrokeKind::Inside,
            );
        }
        // The instances the receipt placed, at the L and W they declared. One
        // with no L or no W has no position on this plane and is left off
        // rather than plotted at an invented one; the table below lists it.
        let mut plotted = 0usize;
        for index in audit.instances_of(&key) {
            let instance = &inspection.instances[*index];
            let (Some(length), Some(width)) = (instance.length, instance.width) else {
                continue;
            };
            if length <= 0.0 || width <= 0.0 {
                continue;
            }
            plotted += 1;
            ui.painter().circle(
                egui::pos2(map_x(length.log10()), map_y(width.log10())),
                2.6,
                if instance.selection == rspice_core::engine::ModelBinSelectionKind::SharedBoundary
                {
                    t.color.warn
                } else {
                    t.color.text
                },
                Stroke::new(1.0, t.color.bg_inset),
            );
        }
        // The decades the map actually spans. Without them the plot states
        // that one card sits left of another and nothing about what either
        // geometry *is*, which is the number an engineer came here for.
        for (anchor, position, decade) in [
            (
                egui::Align2::LEFT_TOP,
                plot.left_bottom() + egui::vec2(0.0, 3.0),
                min_l,
            ),
            (
                egui::Align2::RIGHT_TOP,
                plot.right_bottom() + egui::vec2(0.0, 3.0),
                max_l,
            ),
            (
                egui::Align2::RIGHT_BOTTOM,
                plot.left_bottom() + egui::vec2(-3.0, 0.0),
                min_w,
            ),
            (
                egui::Align2::RIGHT_TOP,
                plot.left_top() + egui::vec2(-3.0, 0.0),
                max_w,
            ),
        ] {
            ui.painter().text(
                position,
                anchor,
                engineering_quantity(10f64.powf(decade), "m"),
                theme::mono(tokens::FS_0, FontWeight::Regular),
                t.color.text_faint,
            );
        }
        let announcement = format!(
            "Log L over W map for {}: {} cards between {} and {} length and {} and {} width, \
             {} overlapping regions, {plotted} placed instances plotted.",
            family.family,
            finite.len(),
            engineering_quantity(10f64.powf(min_l), "m"),
            engineering_quantity(10f64.powf(max_l), "m"),
            engineering_quantity(10f64.powf(min_w), "m"),
            engineering_quantity(10f64.powf(max_w), "m"),
            tally.findings,
        );
        response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), &announcement)
        });
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_role(egui::accesskit::Role::Label);
            node.set_label(announcement.clone());
        });
        map_legend(ui, tally.findings > 0, plotted);
    });
}

/// The two tones this map can actually paint, and what the dots are.
///
/// The mockup's legend also offered "override-provided" and "uncovered".
/// Neither can render: the receipt carries no per-edge override provenance, and
/// an uncovered region is the condition that refuses the receipt outright. A
/// swatch for a state the page cannot enter is a legend entry that never
/// matches anything on the plot beside it.
fn map_legend(ui: &mut Ui, has_findings: bool, plotted: usize) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), MAP_LEGEND_H),
        Sense::hover(),
    );
    ui.painter()
        .hline(rect.x_range(), rect.top(), Stroke::new(1.0, t.color.border));
    let mut x = rect.left() + 2.0;
    let mut swatch = |fill: egui::Color32, stroke: egui::Color32, label: &str| {
        let mark =
            egui::Rect::from_min_size(egui::pos2(x, rect.center().y - 4.5), egui::vec2(12.0, 9.0));
        ui.painter().rect(
            mark,
            0.0,
            fill,
            Stroke::new(1.0, stroke),
            egui::StrokeKind::Inside,
        );
        x += 17.0;
        let width = ui
            .painter()
            .layout_no_wrap(
                label.to_owned(),
                theme::sans(tokens::FS_0, FontWeight::Regular),
                t.color.text_faint,
            )
            .size()
            .x;
        ui.painter().text(
            egui::pos2(x, rect.center().y),
            egui::Align2::LEFT_CENTER,
            label,
            theme::sans(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
        );
        x += width + 12.0;
    };
    swatch(
        t.color.text.linear_multiply(0.06),
        t.color.border_strong,
        "declared card",
    );
    if has_findings {
        swatch(
            t.color.err.linear_multiply(0.26),
            t.color.err,
            "overlapping envelopes",
        );
    }
    ui.painter().text(
        egui::pos2(rect.right() - 2.0, rect.center().y),
        egui::Align2::RIGHT_CENTER,
        if plotted == 0 {
            "no placed instance declares both L and W".to_owned()
        } else {
            format!("{plotted} instances plotted at their placed L·W")
        },
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
}

/// Where the AUDIT column starts, as a fraction of the family table's width.
const FAMILY_AUDIT_START: f32 = 0.79;
/// How wide it is.
const FAMILY_AUDIT_WIDTH: f32 = 0.21;

fn family_table(
    ui: &mut Ui,
    app: &mut ManagerRenderContext<'_>,
    families: &[BinFamily],
    audit: &BinAudit,
    selected: &str,
    height: f32,
) {
    let mut picked = None;
    card(ui, |ui| {
        card_title(
            ui,
            "BINNED FAMILIES",
            Some(&format!("{} · library · family", families.len())),
        );
        table_header(
            ui,
            &[
                ("FAMILY", 0.24),
                ("LIBRARY", 0.20),
                ("DEVICE", 0.14),
                ("CARDS", 0.10),
                ("PLACED", 0.11),
                ("AUDIT", FAMILY_AUDIT_WIDTH),
            ],
        );
        ScrollArea::vertical()
            .id_salt("models-bin-families")
            .max_height((height - 60.0).max(ROW_H * 2.0))
            .show_rows(ui, ROW_H, families.len(), |ui, range| {
                let t = Tokens::get(ui.ctx());
                for family in &families[range] {
                    let key = family.key();
                    let tally = audit.tally(&key);
                    // Worst first: a family with a finding is not "clean apart
                    // from", and the cell has room for one phrase.
                    let (phrase, tone) = if tally.findings > 0 {
                        (
                            format!(
                                "{} finding{}",
                                tally.findings,
                                if tally.findings == 1 { "" } else { "s" }
                            ),
                            t.color.err,
                        )
                    } else if tally.shared > 0 {
                        (format!("{} on a boundary", tally.shared), t.color.warn)
                    } else {
                        ("clean".to_owned(), t.color.ok)
                    };
                    let cards = family.cards.len().to_string();
                    let placed = if tally.instances == 0 {
                        "—".to_owned()
                    } else {
                        tally.instances.to_string()
                    };
                    let device = family.device();
                    // The audit cell is painted over the row rather than passed
                    // through it: the row painter has one colour, and this
                    // cell's colour is the finding.
                    let response = selectable_data_row(
                        ui,
                        key == selected,
                        &[
                            (&family.family, 0.24, true),
                            (&family.library, 0.20, false),
                            (&device, 0.14, false),
                            (&cards, 0.10, true),
                            (&placed, 0.11, true),
                            ("", FAMILY_AUDIT_WIDTH, false),
                        ],
                    );
                    ui.painter().text(
                        egui::pos2(
                            response.rect.left() + response.rect.width() * FAMILY_AUDIT_START + 5.0,
                            response.rect.center().y,
                        ),
                        egui::Align2::LEFT_CENTER,
                        elide(
                            ui,
                            &phrase,
                            response.rect.width() * FAMILY_AUDIT_WIDTH - 9.0,
                            false,
                        ),
                        theme::sans(tokens::FS_0, FontWeight::Regular),
                        tone,
                    );
                    let announcement =
                        format!("{key} · {device} · {cards} cards · {placed} placed · {phrase}");
                    ui.ctx().accesskit_node_builder(response.id, |node| {
                        node.set_label(announcement.clone());
                    });
                    if response.clicked() {
                        picked = Some(key);
                    }
                }
            });
    });
    if let Some(picked) = picked {
        app.state.workbench.models_view.selected_bin_family = Some(picked);
    }
}

fn findings_card(ui: &mut Ui, family: &BinFamily, audit: &BinAudit) {
    let key = family.key();
    let tally = audit.tally(&key);
    let findings = audit.findings_for(&key).collect::<Vec<_>>();
    card(ui, |ui| {
        card_title(
            ui,
            "GEOMETRY FINDINGS",
            Some(&format!(
                "{} · {} shared boundar{}",
                family.family,
                tally.boundaries,
                if tally.boundaries == 1 { "y" } else { "ies" }
            )),
        );
        let t = Tokens::get(ui.ctx());
        if findings.is_empty() {
            ui.label(
                RichText::new(format!(
                    "Every declared envelope in {} is free of enclosed overlap. Its {} shared \
                     boundaries are adjacent cards whose inclusive edges touch without enclosing \
                     area; the first declared card wins there, deterministically.",
                    family.family, tally.boundaries
                ))
                .small()
                .color(t.color.text_dim),
            );
            return;
        }
        for finding in findings.iter().take(FINDING_ROWS) {
            ui.label(RichText::new(finding.sentence()).small().color(t.color.err));
        }
        if findings.len() > FINDING_ROWS {
            ui.label(
                RichText::new(format!(
                    "…and {} more, all counted above",
                    findings.len() - FINDING_ROWS
                ))
                .small()
                .color(t.color.text_faint),
            );
        }
        ui.label(
            RichText::new(
                "Nothing has landed in these regions yet: an instance resolving through the family \
                 inside one would have refused the receipt instead of picking a card. Re-bin \
                 before one does.",
            )
            .small()
            .color(t.color.text_faint),
        );
    });
}

/// Where the instance table remembers which row is selected.
///
/// View-local and deliberately not durable: which row a reader last clicked on
/// a page derived from a prepared deck is not engineering state to restore into
/// a session that may hold a different design.
fn selected_instance_id(family_key: &str) -> egui::Id {
    egui::Id::new(("models-bin-selected-instance", family_key))
}

/// Where the OUTCOME column starts, as a fraction of the instance table.
const INSTANCE_OUTCOME_START: f32 = 0.85;
/// How wide it is.
const INSTANCE_OUTCOME_WIDTH: f32 = 0.15;

fn geometry_instance_table(
    ui: &mut Ui,
    app: &mut ManagerRenderContext<'_>,
    family: &BinFamily,
    audit: &BinAudit,
    inspection: &rspice_core::engine::ModelBinInspection,
) {
    let key = family.key();
    let tally = audit.tally(&key);
    let matches = audit.instances_of(&key);
    let selection_id = selected_instance_id(&key);
    let selected = ui
        .ctx()
        .data(|data| data.get_temp::<String>(selection_id))
        .unwrap_or_default();
    let mut picked = None;
    let mut trace = None;
    card(ui, |ui| {
        ui.horizontal(|ui| {
            let t = Tokens::get(ui.ctx());
            ui.label(
                RichText::new("INSTANCE RESOLUTION")
                    .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                    .color(t.color.text_dim),
            );
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                // Routes through the same dialog the page bar's "Trace
                // schematic" opens, on the model *this row* asked for rather
                // than on whatever the catalog happens to have selected.
                if Button::new("Trace in schematic…")
                    .enabled(!selected.is_empty())
                    .show(ui)
                    .on_disabled_hover_text("Select an instance row first.")
                    .clicked()
                {
                    trace = Some(selected.clone());
                }
                ui.label(
                    RichText::new(format!(
                        "{} · {} / {} resolving to one card",
                        family.family,
                        tally.one_card(),
                        tally.instances
                    ))
                    .small()
                    .monospace()
                    .color(if tally.shared == 0 {
                        t.color.text_faint
                    } else {
                        t.color.warn
                    }),
                );
            });
        });
        ui.separator();
        if matches.is_empty() {
            empty_state(
                ui,
                "No placed instance resolves to this family.",
                "The exact prepared design contains no simulator-resolved instance for this family.",
            );
            return;
        }
        table_header(
            ui,
            &[
                ("INSTANCE", 0.15),
                ("REQUESTED", 0.16),
                ("LENGTH", 0.11),
                ("WIDTH", 0.11),
                ("NFIN", 0.08),
                ("M", 0.07),
                ("RESOLVES TO", 0.17),
                ("OUTCOME", INSTANCE_OUTCOME_WIDTH),
            ],
        );
        ScrollArea::vertical()
            .id_salt("models-bin-instances")
            .max_height((ui.available_height() - ROW_H).max(ROW_H * 3.0))
            .show_rows(ui, ROW_H, matches.len(), |ui, range| {
                let t = Tokens::get(ui.ctx());
                for index in &matches[range] {
                    let instance = &inspection.instances[*index];
                    let (outcome, tone) = match instance.selection {
                        rspice_core::engine::ModelBinSelectionKind::ExactCard => {
                            ("exact card", t.color.text_dim)
                        }
                        rspice_core::engine::ModelBinSelectionKind::FamilyMatch => {
                            ("family match", t.color.ok)
                        }
                        rspice_core::engine::ModelBinSelectionKind::SharedBoundary => {
                            ("boundary · first card", t.color.warn)
                        }
                    };
                    let length = optional_bin_length(instance.length);
                    let width = optional_bin_length(instance.width);
                    let nfin = optional_bin_count(instance.nfin);
                    let multiplier = optional_bin_count(instance.multiplier);
                    let response = selectable_data_row(
                        ui,
                        selected == instance.element,
                        &[
                            (&instance.element, 0.15, true),
                            (&instance.requested_model, 0.16, true),
                            (&length, 0.11, true),
                            (&width, 0.11, true),
                            (&nfin, 0.08, true),
                            (&multiplier, 0.07, true),
                            (&instance.selected_model, 0.17, true),
                            ("", INSTANCE_OUTCOME_WIDTH, false),
                        ],
                    );
                    ui.painter().text(
                        egui::pos2(
                            response.rect.left()
                                + response.rect.width() * INSTANCE_OUTCOME_START
                                + 5.0,
                            response.rect.center().y,
                        ),
                        egui::Align2::LEFT_CENTER,
                        elide(
                            ui,
                            outcome,
                            response.rect.width() * INSTANCE_OUTCOME_WIDTH - 9.0,
                            false,
                        ),
                        theme::sans(tokens::FS_0, FontWeight::Regular),
                        tone,
                    );
                    let announcement = format!(
                        "{} requested {}, resolved to {} · {outcome} · L={length} W={width} \
                         NFIN={nfin} M={multiplier}",
                        instance.element, instance.requested_model, instance.selected_model,
                    );
                    ui.ctx().accesskit_node_builder(response.id, |node| {
                        node.set_label(announcement.clone());
                    });
                    if response.clicked() {
                        picked = Some(instance.element.clone());
                    }
                }
            });
        ui.label(
            RichText::new(
                "Resolution is the netlister's own lookup: inclusive on both edges, first declared \
                 card wins. An ambiguous or uncovered instance refuses the netlist rather than \
                 picking a card.",
            )
            .small()
            .color(Tokens::get(ui.ctx()).color.text_faint),
        );
    });
    if let Some(picked) = picked {
        ui.ctx()
            .data_mut(|data| data.insert_temp(selection_id, picked));
    }
    if let Some(element) = trace
        && let Some(instance) = inspection
            .instances
            .iter()
            .find(|instance| instance.element == element)
    {
        let model = instance.requested_model.clone();
        app.state.workbench.models_view.dialog = Some(ModelsWorkbenchDialog::BindingTrace {
            consumers: effective_model_consumers(app, &model),
            model,
        });
    }
}

/// A device dimension the instance declared, in metres.
fn optional_bin_length(value: Option<f64>) -> String {
    value.map_or_else(|| "—".to_owned(), |value| engineering_quantity(value, "m"))
}

/// A count the instance declared — fins, multiplicity — which carries no unit
/// and no decade prefix.
fn optional_bin_count(value: Option<f64>) -> String {
    value.map_or_else(|| "—".to_owned(), engineering_value)
}

#[cfg(test)]
mod tests;
