//! Geometry-bin audit and instance-to-card traceability.
//!
//! The surface projects the engine's inspection receipt for the exact
//! analysis-independent executable design deck. It does not carry a second
//! editable bin database: evaluated limits, source order, flattened instance
//! geometry, and winners come from the same resolver used by circuit
//! construction. Loaded-library metadata is used only to link cards back to
//! their editable source locations.

use super::*;
use rspice_core::engine::{ModelBinCardInspection, ModelBinInspection, ModelBinSelectionKind};

const BIN_BOUND_TOLERANCE: f64 = 1.0e-9;
const MAX_COVERAGE_GRID_CELLS: usize = 500_000;
const BIN_AUDIT_STRIP_H: f32 = 72.0;
const BIN_FAMILY_TABLE_H: f32 = 150.0;
const BIN_MAP_H: f32 = 310.0;
const BIN_CARD_TABLE_H: f32 = 250.0;
const BIN_FINDINGS_H: f32 = 180.0;
const BIN_INSTANCE_TABLE_H: f32 = 210.0;
const BIN_WIDE_BREAKPOINT: f32 = 920.0;
const MAX_BIN_HIERARCHY_DEPTH: usize = 128;
const MAX_BIN_HIERARCHY_INSTANCES: usize = 1_000_000;

#[derive(Debug, Clone, Copy, PartialEq)]
struct BinRange {
    min: Option<f64>,
    max: Option<f64>,
}

impl BinRange {
    const fn new(min: Option<f64>, max: Option<f64>) -> Self {
        Self { min, max }
    }

    fn is_declared(self) -> bool {
        self.min.is_some() || self.max.is_some()
    }

    fn is_finite_and_ordered(self) -> bool {
        self.min.is_none_or(f64::is_finite)
            && self.max.is_none_or(f64::is_finite)
            && match (self.min, self.max) {
                (Some(min), Some(max)) => min <= max || bin_bound_equal(min, max),
                _ => true,
            }
    }

    fn is_complete_positive_area(self) -> bool {
        matches!(
            (self.min, self.max),
            (Some(min), Some(max))
                if min.is_finite()
                    && max.is_finite()
                    && min > 0.0
                    && max > min
                    && !bin_bound_equal(min, max)
        )
    }
}

#[derive(Debug, Clone)]
struct BinCard {
    model: String,
    source: String,
    source_order: usize,
    source_line: usize,
    length: BinRange,
    width: BinRange,
    nfin: BinRange,
}

impl BinCard {
    fn invalid_reason(&self) -> Option<&'static str> {
        if !self.length.is_finite_and_ordered()
            || !self.width.is_finite_and_ordered()
            || !self.nfin.is_finite_and_ordered()
        {
            return Some("non-finite or reversed bound");
        }
        if !self.length.is_declared() && !self.width.is_declared() && !self.nfin.is_declared() {
            return Some("no binning bounds");
        }
        None
    }

    fn full_lw_rectangle(&self) -> Option<BinRegion> {
        if !self.length.is_complete_positive_area() || !self.width.is_complete_positive_area() {
            return None;
        }
        Some(BinRegion {
            l_min: self.length.min?,
            l_max: self.length.max?,
            w_min: self.width.min?,
            w_max: self.width.max?,
        })
    }
}

#[derive(Debug, Clone)]
struct BinFamily {
    key: String,
    library: String,
    name: String,
    device: String,
    cards: Vec<BinCard>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct BinRegion {
    l_min: f64,
    l_max: f64,
    w_min: f64,
    w_max: f64,
}

impl BinRegion {
    fn label(self) -> String {
        format!(
            "L {} to {} | W {} to {}",
            format_length(self.l_min),
            format_length(self.l_max),
            format_length(self.w_min),
            format_length(self.w_max),
        )
    }
}

#[derive(Debug, Clone)]
struct BinOverlap {
    left: String,
    right: String,
    region: BinRegion,
}

#[derive(Debug, Clone, Default)]
struct BinAudit {
    invalid_cards: Vec<(String, &'static str)>,
    overlaps: Vec<BinOverlap>,
    gaps: Vec<BinRegion>,
    shared_boundaries: usize,
    envelope: Option<BinRegion>,
    coverage_available: bool,
    coverage_too_large: bool,
}

impl BinAudit {
    fn fault_count(&self) -> usize {
        self.invalid_cards.len() + self.overlaps.len() + self.gaps.len()
    }

    fn is_clean(&self) -> bool {
        self.coverage_available && self.fault_count() == 0
    }

    fn status(&self) -> &'static str {
        if self.fault_count() > 0 {
            "geometry fault"
        } else if self.coverage_available {
            "geometry clean"
        } else {
            "coverage review"
        }
    }
}

#[derive(Debug, Clone)]
struct BinInstance {
    component_id: u64,
    owner: CellViewRef,
    name: String,
    path: String,
    length: Option<f64>,
    width: Option<f64>,
    nfin: Option<f64>,
    multiplier: Option<f64>,
    outcome: BinResolution,
}

impl BinInstance {
    fn row_key(&self) -> String {
        self.path.to_ascii_lowercase()
    }
}

#[derive(Debug, Clone)]
struct PlacedModelInstance {
    component_id: u64,
    owner: CellViewRef,
    name: String,
    path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum BinResolution {
    Resolved(String),
    SharedBoundary { winner: String, matches: usize },
}

impl BinResolution {
    fn winner(&self) -> Option<&str> {
        match self {
            Self::Resolved(winner) | Self::SharedBoundary { winner, .. } => Some(winner),
        }
    }

    fn label(&self) -> String {
        match self {
            Self::Resolved(_) => "resolved".to_owned(),
            Self::SharedBoundary { matches, .. } => {
                format!("shared boundary ({matches})")
            }
        }
    }

    fn deterministic(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct BinTotals {
    families: usize,
    cards: usize,
    instances: usize,
    deterministic_instances: usize,
    overlaps: usize,
    gaps: usize,
    invalid_cards: usize,
    shared_boundaries: usize,
    review_families: usize,
    inspection_blocked: bool,
}

#[derive(Debug, Clone)]
struct AuthoritativeBinInspection {
    input_digest: crate::product::ContentDigest,
    source_digest: Option<crate::product::ContentDigest>,
    receipt: Option<ModelBinInspection>,
    diagnostic: Option<String>,
}

pub(super) fn bins(ui: &mut Ui, app: &mut RSpiceApp) {
    let authoritative = authoritative_bin_inspection(ui, app);
    let families = authoritative
        .receipt
        .as_ref()
        .map(|receipt| collect_authoritative_bin_families(app, receipt))
        .unwrap_or_else(|| collect_bin_families(app));
    let placed_instances = if authoritative.receipt.is_some() {
        collect_hierarchical_model_instances(app)
    } else {
        Err(authoritative
            .diagnostic
            .clone()
            .unwrap_or_else(|| "authoritative model-bin inspection is unavailable".to_owned()))
    };
    let inspection_error = placed_instances.as_ref().err().cloned();
    let selected_index = selected_bin_family_index(
        &families,
        app.state.model_library_manager.selected_library.as_deref(),
        app.state.workbench.selected_model.as_deref(),
    );
    let selected_family = selected_index
        .and_then(|index| families.get(index))
        .cloned();
    let selected_family_is_loaded = selected_family.as_ref().is_some_and(|family| {
        app.state
            .model_library_manager
            .get_library(&family.library)
            .is_some()
    });
    let family_audits = families.iter().map(audit_bin_family).collect::<Vec<_>>();
    let family_instances = families
        .iter()
        .map(|family| {
            authoritative
                .receipt
                .as_ref()
                .zip(placed_instances.as_ref().ok())
                .map(|(receipt, instances)| {
                    collect_authoritative_bin_instances(instances, family, receipt)
                })
                .unwrap_or_default()
        })
        .collect::<Vec<_>>();
    let totals = bin_totals(
        &families,
        &family_audits,
        &family_instances,
        inspection_error.is_some(),
    );
    let mut import_requested = false;
    let mut edit_requested = false;
    let mut audit_requested = false;
    let mut export_receipt_requested = false;
    let receipt_available = app
        .state
        .model_library_manager
        .latest_model_bin_audit_receipt()
        .is_some();

    surface_title_with_action_reserve(
        ui,
        &format!(
            "{} binned families - {} cards - {} placed instances",
            totals.families, totals.cards, totals.instances
        ),
        "Bins & geometry",
        "Audit the parsed card ranges that drive model selection and trace placed device geometry to the winning card.",
        true,
        520.0,
        |ui| {
            if Button::new("Import bin map...")
                .show(ui)
                .on_hover_text("Open model-library sources and load a .lib or .scs bin map.")
                .clicked()
            {
                import_requested = true;
            }
            ui.add_enabled_ui(selected_family_is_loaded, |ui| {
                if Button::new("Edit cards...")
                    .show(ui)
                    .on_hover_text(
                        "Open the selected card in the governed model editor when its source is project-owned.",
                    )
                    .clicked()
                {
                    edit_requested = true;
                }
            });
            if Button::new("Audit current design")
                .accent()
                .show(ui)
                .clicked()
            {
                audit_requested = true;
            }
            ui.add_enabled_ui(receipt_available, |ui| {
                if Button::new("Export receipt...").show(ui).clicked() {
                    export_receipt_requested = true;
                }
            });
        },
    );

    if import_requested {
        Command::PdkSettings.execute(app);
    }
    if edit_requested {
        if let Some(family) = &selected_family
            && let Some(card) =
                selected_bin_card(family, app.state.workbench.selected_model.as_deref())
        {
            app.state
                .model_library_manager
                .select_library(&family.library);
            app.state.workbench.selected_model = Some(card.model.clone());
        }
        Command::ModelEditor.execute(app);
    }
    if audit_requested {
        let draft = build_bin_audit_draft(app, &authoritative, &families, &family_audits);
        match record_bin_audit(app, draft) {
            Ok(receipt) => {
                announce_bin_audit(app, totals, inspection_error.as_deref(), Some(&receipt));
            }
            Err(error) => {
                app.state.push_user_message(ConsoleMessage::error(format!(
                    "Geometry-bin audit could not be recorded: {error}"
                )));
            }
        }
    }
    if export_receipt_requested && let Err(error) = export_latest_bin_audit_receipt(app) {
        app.state.push_user_message(ConsoleMessage::error(format!(
            "Model-bin audit export failed: {error}"
        )));
    }

    if let Some(error) = inspection_error.as_deref() {
        let t = Tokens::get(ui.ctx());
        let notice = format!(
            "Authoritative engine inspection is blocked. Source cards remain visible for remediation, but no instance-resolution claim is made: {error}"
        );
        let response = ui.add(
            egui::Label::new(
                egui::RichText::new(&notice)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.err),
            )
            .wrap(),
        );
        accessible_model_text(ui, &response, &notice);
        ui.add_space(6.0);
    }

    if families.is_empty() {
        let available = ui.available_size();
        let empty_message = if inspection_error.is_some() {
            "Authoritative engine inspection is blocked; no executable geometry-bin result is available."
        } else {
            "No executable model source contains geometry-binned model families."
        };
        data_table(
            ui,
            "models.bins.empty",
            GENERAL_TABLE_MIN_W,
            &[("Family", 0.35), ("Cards", 0.15), ("Geometry", 0.50)],
            &[],
            available,
            empty_message,
        );
        return;
    }

    bin_audit_strip(
        ui,
        totals,
        app.state
            .model_library_manager
            .latest_model_bin_audit_receipt(),
    );
    let available = ui.available_size().max(Vec2::splat(1.0));
    let (viewport, _) = ui.allocate_exact_size(available, Sense::hover());
    let mut body = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(viewport)
            .layout(Layout::top_down(Align::Min)),
    );
    body.spacing_mut().item_spacing = Vec2::ZERO;

    let mut family_event = None;
    let mut card_event = None;
    let mut instance_event = None;
    ScrollArea::vertical()
        .id_salt("models.bins.body")
        .auto_shrink([false, false])
        .show(&mut body, |ui| {
            ui.set_min_width(viewport.width());
            family_event = bin_family_table(
                ui,
                &families,
                &family_audits,
                &family_instances,
                selected_index,
            );

            let selected_index = selected_index.unwrap_or(0);
            let family = &families[selected_index];
            let audit = &family_audits[selected_index];
            let instances = &family_instances[selected_index];
            let wide = viewport.width() >= BIN_WIDE_BREAKPOINT;
            if wide {
                ui.horizontal_top(|ui| {
                    ui.spacing_mut().item_spacing.x = 1.0;
                    let map_width = (viewport.width() * 0.56).max(1.0);
                    table_card(
                        ui,
                        &format!("{} bin map", family.name),
                        Some((
                            audit.status(),
                            bin_audit_status_color(&Tokens::get(ui.ctx()), audit),
                        )),
                        egui::vec2(map_width, BIN_MAP_H),
                        |ui, size| draw_bin_map(ui, family, audit, size),
                    );
                    let card_width = (viewport.width() - map_width - 1.0).max(1.0);
                    card_event = bin_card_table(
                        ui,
                        family,
                        audit,
                        app.state.workbench.selected_model.as_deref(),
                        egui::vec2(card_width, BIN_MAP_H),
                    );
                });
            } else {
                table_card(
                    ui,
                    &format!("{} bin map", family.name),
                    Some((
                        audit.status(),
                        bin_audit_status_color(&Tokens::get(ui.ctx()), audit),
                    )),
                    egui::vec2(viewport.width(), BIN_MAP_H),
                    |ui, size| draw_bin_map(ui, family, audit, size),
                );
                card_event = bin_card_table(
                    ui,
                    family,
                    audit,
                    app.state.workbench.selected_model.as_deref(),
                    egui::vec2(viewport.width(), BIN_CARD_TABLE_H),
                );
            }
            bin_findings_table(ui, family, audit, viewport.width());
            instance_event = bin_instance_table(
                ui,
                family,
                instances,
                inspection_error.as_deref(),
                viewport.width(),
                &Tokens::get(ui.ctx()),
            );
        });

    if let Some(event) = family_event
        && let Some(family) = families.iter().find(|family| family.key == event.key)
        && let Some(first_card) = family.cards.first()
    {
        app.state
            .model_library_manager
            .select_library(&family.library);
        app.state.workbench.selected_model = Some(first_card.model.clone());
    }
    if let Some(event) = card_event
        && let Some(family) = &selected_family
        && let Some(card) = family.cards.iter().find(|card| card.model == event.key)
    {
        app.state
            .model_library_manager
            .select_library(&family.library);
        app.state.workbench.selected_model = Some(card.model.clone());
        if event.activate {
            Command::ModelEditor.execute(app);
        }
    }
    if let Some(event) = instance_event
        && event.activate
        && let Some(instance) = selected_index
            .and_then(|index| family_instances.get(index))
            .and_then(|instances| {
                instances
                    .iter()
                    .find(|instance| instance.row_key() == event.key)
            })
    {
        trace_bin_instance(app, instance);
    }
}

fn authoritative_bin_inspection(ui: &Ui, app: &RSpiceApp) -> AuthoritativeBinInspection {
    let input_digest =
        crate::simulation::controller::prepared_run::design_inspection_input_digest(&app.state);
    let cache_id = egui::Id::new("models-authoritative-bin-inspection");
    if let Some(cached) = ui
        .ctx()
        .data(|data| data.get_temp::<AuthoritativeBinInspection>(cache_id))
        && cached.input_digest == input_digest
    {
        return cached;
    }

    let (source_digest, receipt, diagnostic) =
        match crate::simulation::controller::SimulationController::inspect_current_model_bins(
            &app.state,
        ) {
            Ok(inspection) => (
                Some(inspection.source_digest),
                Some(inspection.receipt),
                None,
            ),
            Err(error) => (None, None, Some(error.to_string())),
        };
    let inspection = AuthoritativeBinInspection {
        input_digest,
        source_digest,
        receipt,
        diagnostic,
    };
    ui.ctx()
        .data_mut(|data| data.insert_temp(cache_id, inspection.clone()));
    inspection
}

fn collect_authoritative_bin_families(
    app: &RSpiceApp,
    receipt: &ModelBinInspection,
) -> Vec<BinFamily> {
    let mut cards_by_family = BTreeMap::<String, Vec<&ModelBinCardInspection>>::new();
    for card in &receipt.cards {
        cards_by_family
            .entry(card.family.to_ascii_lowercase())
            .or_default()
            .push(card);
    }

    let libraries = app.state.model_library_manager.libraries_sorted();
    let mut families = Vec::with_capacity(cards_by_family.len());
    for cards in cards_by_family.into_values() {
        let family_name = cards
            .first()
            .map(|card| card.family.clone())
            .unwrap_or_default();
        let library = libraries
            .iter()
            .filter_map(|library| {
                let matches = cards
                    .iter()
                    .filter(|card| {
                        library
                            .models
                            .values()
                            .any(|model| model.name.eq_ignore_ascii_case(&card.model))
                    })
                    .count();
                (matches > 0).then_some((*library, matches))
            })
            .max_by(|(left, left_count), (right, right_count)| {
                left_count.cmp(right_count).then_with(|| {
                    right
                        .name
                        .to_ascii_lowercase()
                        .cmp(&left.name.to_ascii_lowercase())
                })
            })
            .map(|(library, _)| library);
        let library_name = library
            .map(|library| library.name.clone())
            .unwrap_or_else(|| "Executable model set".to_owned());
        let cards = cards
            .into_iter()
            .map(|card| {
                let source_model = library.and_then(|library| {
                    library
                        .models
                        .values()
                        .find(|model| model.name.eq_ignore_ascii_case(&card.model))
                });
                BinCard {
                    model: card.model.clone(),
                    source: source_model.map_or_else(
                        || format!("Executable declaration {}", card.declaration_order + 1),
                        |model| {
                            model_source_label(
                                library.expect("source model is bound to this library"),
                                model,
                            )
                        },
                    ),
                    source_order: card.declaration_order,
                    source_line: source_model
                        .and_then(|model| model.source_line)
                        .unwrap_or(usize::MAX),
                    length: BinRange::new(card.geometry.length.min, card.geometry.length.max),
                    width: BinRange::new(card.geometry.width.min, card.geometry.width.max),
                    nfin: BinRange::new(card.geometry.nfin.min, card.geometry.nfin.max),
                }
            })
            .collect::<Vec<_>>();
        families.push(BinFamily {
            key: model_key(&library_name, &family_name),
            library: library_name,
            name: family_name,
            device: cards
                .first()
                .and_then(|card| {
                    receipt
                        .cards
                        .iter()
                        .find(|source| source.model.eq_ignore_ascii_case(&card.model))
                })
                .map(|card| card.model_type.clone())
                .unwrap_or_else(|| "MOSFET".to_owned()),
            cards,
        });
    }
    families.sort_by(|left, right| {
        left.library
            .to_ascii_lowercase()
            .cmp(&right.library.to_ascii_lowercase())
            .then_with(|| {
                left.name
                    .to_ascii_lowercase()
                    .cmp(&right.name.to_ascii_lowercase())
            })
    });
    families
}

fn collect_bin_families(app: &RSpiceApp) -> Vec<BinFamily> {
    let mut families = BTreeMap::<(String, String), BinFamily>::new();
    for library in app.state.model_library_manager.libraries_sorted() {
        let source_order = library
            .source_closure
            .iter()
            .enumerate()
            .map(|(index, source)| (source.path.clone(), index))
            .collect::<HashMap<_, _>>();
        for model in library.models.values() {
            let Some(family_name) = binned_family_name(model) else {
                continue;
            };
            let path = model.file_path.as_deref().or(library.root_path.as_deref());
            let card = BinCard {
                model: model.name.clone(),
                source: model_source_label(library, model),
                source_order: path
                    .and_then(|path| source_order.get(path).copied())
                    .unwrap_or(usize::MAX),
                source_line: model.source_line.unwrap_or(usize::MAX),
                length: BinRange::new(model.l_min, model.l_max),
                width: BinRange::new(model.w_min, model.w_max),
                nfin: BinRange::new(
                    model_numeric_parameter(model, "NFINMIN"),
                    model_numeric_parameter(model, "NFINMAX"),
                ),
            };
            let key = (library.name.clone(), family_name.to_ascii_lowercase());
            families
                .entry(key)
                .or_insert_with(|| BinFamily {
                    key: model_key(&library.name, family_name),
                    library: library.name.clone(),
                    name: family_name.to_owned(),
                    device: model.model_type.display_name().to_owned(),
                    cards: Vec::new(),
                })
                .cards
                .push(card);
        }
    }

    let mut families = families.into_values().collect::<Vec<_>>();
    for family in &mut families {
        family.cards.sort_by(|left, right| {
            left.source_order
                .cmp(&right.source_order)
                .then_with(|| left.source_line.cmp(&right.source_line))
                .then_with(|| {
                    left.model
                        .to_ascii_lowercase()
                        .cmp(&right.model.to_ascii_lowercase())
                })
        });
    }
    families.sort_by(|left, right| {
        left.library
            .to_ascii_lowercase()
            .cmp(&right.library.to_ascii_lowercase())
            .then_with(|| {
                left.name
                    .to_ascii_lowercase()
                    .cmp(&right.name.to_ascii_lowercase())
            })
    });
    families
}

fn binned_family_name(model: &DeviceModel) -> Option<&str> {
    let geometry_declared = model.l_min.is_some()
        || model.l_max.is_some()
        || model.w_min.is_some()
        || model.w_max.is_some()
        || model_numeric_parameter(model, "NFINMIN").is_some()
        || model_numeric_parameter(model, "NFINMAX").is_some();
    let (family, suffix) = model.name.rsplit_once('.')?;
    (geometry_declared && !family.trim().is_empty() && !suffix.trim().is_empty()).then_some(family)
}

fn model_numeric_parameter(model: &DeviceModel, name: &str) -> Option<f64> {
    model
        .parameters
        .iter()
        .find(|(candidate, _)| candidate.eq_ignore_ascii_case(name))
        .map(|(_, value)| *value)
}

fn selected_bin_family_index(
    families: &[BinFamily],
    selected_library: Option<&str>,
    selected_model: Option<&str>,
) -> Option<usize> {
    families
        .iter()
        .position(|family| {
            selected_library.is_some_and(|library| library == family.library)
                && selected_model.is_some_and(|model| {
                    model.eq_ignore_ascii_case(&family.name)
                        || family
                            .cards
                            .iter()
                            .any(|card| card.model.eq_ignore_ascii_case(model))
                })
        })
        .or((!families.is_empty()).then_some(0))
}

fn selected_bin_card<'a>(
    family: &'a BinFamily,
    selected_model: Option<&str>,
) -> Option<&'a BinCard> {
    selected_model
        .and_then(|selected| {
            family
                .cards
                .iter()
                .find(|card| card.model.eq_ignore_ascii_case(selected))
        })
        .or_else(|| family.cards.first())
}

fn audit_bin_family(family: &BinFamily) -> BinAudit {
    let mut audit = BinAudit::default();
    for card in &family.cards {
        if let Some(reason) = card.invalid_reason() {
            audit.invalid_cards.push((card.model.clone(), reason));
        }
    }

    let rectangles = family
        .cards
        .iter()
        .filter_map(|card| card.full_lw_rectangle().map(|region| (card, region)))
        .collect::<Vec<_>>();
    for left_index in 0..rectangles.len() {
        for right_index in (left_index + 1)..rectangles.len() {
            let (left, left_region) = rectangles[left_index];
            let (right, right_region) = rectangles[right_index];
            let l_min = left_region.l_min.max(right_region.l_min);
            let l_max = left_region.l_max.min(right_region.l_max);
            let w_min = left_region.w_min.max(right_region.w_min);
            let w_max = left_region.w_max.min(right_region.w_max);
            let l_span = l_max - l_min;
            let w_span = w_max - w_min;
            if l_span > BIN_BOUND_TOLERANCE && w_span > BIN_BOUND_TOLERANCE {
                audit.overlaps.push(BinOverlap {
                    left: left.model.clone(),
                    right: right.model.clone(),
                    region: BinRegion {
                        l_min,
                        l_max,
                        w_min,
                        w_max,
                    },
                });
            } else if (l_span.abs() < BIN_BOUND_TOLERANCE && w_span > BIN_BOUND_TOLERANCE)
                || (w_span.abs() < BIN_BOUND_TOLERANCE && l_span > BIN_BOUND_TOLERANCE)
            {
                audit.shared_boundaries += 1;
            }
        }
    }

    let full_lw_count = rectangles.len();
    if full_lw_count != family.cards.len() || rectangles.is_empty() {
        return audit;
    }
    let l_min = rectangles
        .iter()
        .map(|(_, region)| region.l_min)
        .fold(f64::INFINITY, f64::min);
    let l_max = rectangles
        .iter()
        .map(|(_, region)| region.l_max)
        .fold(f64::NEG_INFINITY, f64::max);
    let w_min = rectangles
        .iter()
        .map(|(_, region)| region.w_min)
        .fold(f64::INFINITY, f64::min);
    let w_max = rectangles
        .iter()
        .map(|(_, region)| region.w_max)
        .fold(f64::NEG_INFINITY, f64::max);
    audit.envelope = Some(BinRegion {
        l_min,
        l_max,
        w_min,
        w_max,
    });

    let mut l_edges = rectangles
        .iter()
        .flat_map(|(_, region)| [region.l_min, region.l_max])
        .collect::<Vec<_>>();
    let mut w_edges = rectangles
        .iter()
        .flat_map(|(_, region)| [region.w_min, region.w_max])
        .collect::<Vec<_>>();
    sort_and_deduplicate_bounds(&mut l_edges);
    sort_and_deduplicate_bounds(&mut w_edges);
    let columns = l_edges.len().saturating_sub(1);
    let rows = w_edges.len().saturating_sub(1);
    let Some(cell_count) = columns.checked_mul(rows) else {
        audit.coverage_too_large = true;
        return audit;
    };
    if cell_count > MAX_COVERAGE_GRID_CELLS {
        audit.coverage_too_large = true;
        return audit;
    }

    let stride = columns + 1;
    let mut difference = vec![0_i32; (columns + 1) * (rows + 1)];
    for (_, region) in &rectangles {
        let x0 = nearest_bound_index(&l_edges, region.l_min);
        let x1 = nearest_bound_index(&l_edges, region.l_max);
        let y0 = nearest_bound_index(&w_edges, region.w_min);
        let y1 = nearest_bound_index(&w_edges, region.w_max);
        difference[y0 * stride + x0] += 1;
        difference[y0 * stride + x1] -= 1;
        difference[y1 * stride + x0] -= 1;
        difference[y1 * stride + x1] += 1;
    }
    for y in 0..=rows {
        for x in 0..=columns {
            let index = y * stride + x;
            if x > 0 {
                difference[index] += difference[index - 1];
            }
            if y > 0 {
                difference[index] += difference[index - stride];
            }
            if x > 0 && y > 0 {
                difference[index] -= difference[index - stride - 1];
            }
        }
    }
    let mut coverage = Vec::with_capacity(cell_count);
    for y in 0..rows {
        for x in 0..columns {
            coverage.push(difference[y * stride + x]);
        }
    }
    audit.gaps = connected_fault_regions(&l_edges, &w_edges, &coverage, |count| count == 0);
    audit.coverage_available = true;
    audit
}

fn sort_and_deduplicate_bounds(bounds: &mut Vec<f64>) {
    bounds.sort_by(f64::total_cmp);
    bounds.dedup_by(|left, right| bin_bound_equal(*left, *right));
}

fn nearest_bound_index(bounds: &[f64], value: f64) -> usize {
    bounds
        .iter()
        .enumerate()
        .min_by(|(_, left), (_, right)| (*left - value).abs().total_cmp(&(*right - value).abs()))
        .map(|(index, _)| index)
        .unwrap_or(0)
}

fn connected_fault_regions(
    l_edges: &[f64],
    w_edges: &[f64],
    coverage: &[i32],
    is_fault: impl Fn(i32) -> bool,
) -> Vec<BinRegion> {
    let columns = l_edges.len().saturating_sub(1);
    let rows = w_edges.len().saturating_sub(1);
    let mut visited = vec![false; columns * rows];
    let mut regions = Vec::new();
    for start in 0..coverage.len() {
        if visited[start] || !is_fault(coverage[start]) {
            continue;
        }
        let mut stack = vec![start];
        visited[start] = true;
        let mut min_x = start % columns;
        let mut max_x = min_x;
        let mut min_y = start / columns;
        let mut max_y = min_y;
        while let Some(index) = stack.pop() {
            let x = index % columns;
            let y = index / columns;
            min_x = min_x.min(x);
            max_x = max_x.max(x);
            min_y = min_y.min(y);
            max_y = max_y.max(y);
            for neighbor in [
                (x > 0).then(|| index - 1),
                (x + 1 < columns).then(|| index + 1),
                (y > 0).then(|| index - columns),
                (y + 1 < rows).then(|| index + columns),
            ]
            .into_iter()
            .flatten()
            {
                if !visited[neighbor] && is_fault(coverage[neighbor]) {
                    visited[neighbor] = true;
                    stack.push(neighbor);
                }
            }
        }
        regions.push(BinRegion {
            l_min: l_edges[min_x],
            l_max: l_edges[max_x + 1],
            w_min: w_edges[min_y],
            w_max: w_edges[max_y + 1],
        });
    }
    regions
}

fn collect_hierarchical_model_instances(
    app: &RSpiceApp,
) -> Result<Vec<PlacedModelInstance>, String> {
    let projection = app
        .state
        .workspace
        .configuration_execution_projection(
            &app.state.library_manager,
            &app.state.workspace.active_view,
            &app.state.schematic,
        )
        .map_err(|error| error.to_string())?;
    let root = projection.root().clone();
    let root_schematic = projection
        .root_schematic()
        .ok_or_else(|| format!("hierarchy root {} is unavailable", root.display_path()))?;
    let mut instances = Vec::new();
    let mut expanded_instances = 0;
    let mut ancestors = vec![root.key().to_ascii_lowercase()];
    collect_model_instances_from_schematic(
        &projection,
        root_schematic,
        &root,
        "/top",
        0,
        &mut ancestors,
        &mut expanded_instances,
        &mut instances,
    )?;
    instances.sort_by(|left, right| {
        left.path
            .to_ascii_lowercase()
            .cmp(&right.path.to_ascii_lowercase())
            .then_with(|| left.owner.key().cmp(&right.owner.key()))
            .then_with(|| left.component_id.cmp(&right.component_id))
    });
    Ok(instances)
}

#[allow(clippy::too_many_arguments)]
fn collect_model_instances_from_schematic(
    projection: &crate::state::workspace::ConfigurationExecutionProjection,
    schematic: &crate::state::SchematicState,
    owner: &CellViewRef,
    hierarchy_path: &str,
    depth: usize,
    ancestors: &mut Vec<String>,
    expanded_instances: &mut usize,
    instances: &mut Vec<PlacedModelInstance>,
) -> Result<(), String> {
    if depth > MAX_BIN_HIERARCHY_DEPTH {
        return Err(format!(
            "hierarchy exceeds the supported depth of {MAX_BIN_HIERARCHY_DEPTH}"
        ));
    }
    for component in &schematic.components {
        *expanded_instances = expanded_instances.saturating_add(1);
        if *expanded_instances > MAX_BIN_HIERARCHY_INSTANCES {
            return Err(format!(
                "hierarchy exceeds the supported limit of {MAX_BIN_HIERARCHY_INSTANCES} expanded instances"
            ));
        }
        let component_path = format!("{hierarchy_path}/{}", component.name);
        if component_has_model_binding(component) {
            instances.push(PlacedModelInstance {
                component_id: component.id,
                owner: owner.clone(),
                name: component.name.clone(),
                path: component_path.clone(),
            });
        }
        if component.kind != crate::state::ComponentType::CellInstance {
            continue;
        }
        let Some(child_reference) =
            hierarchy_child_reference(projection, component, &component_path)?
        else {
            continue;
        };
        let canonical_reference = child_reference.key().to_ascii_lowercase();
        if ancestors.contains(&canonical_reference) {
            return Err(format!(
                "recursive schematic hierarchy encountered at {component_path} ({})",
                child_reference.display_path()
            ));
        }
        let child_schematic = projection
            .schematic_buffers()
            .iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(&child_reference.key()))
            .map(|(_, schematic)| schematic)
            .ok_or_else(|| {
                format!(
                    "schematic master {} at {component_path} is unavailable",
                    child_reference.display_path()
                )
            })?;
        ancestors.push(canonical_reference);
        let result = collect_model_instances_from_schematic(
            projection,
            child_schematic,
            &child_reference,
            &component_path,
            depth + 1,
            ancestors,
            expanded_instances,
            instances,
        );
        ancestors.pop();
        result?;
    }
    Ok(())
}

fn hierarchy_child_reference(
    projection: &crate::state::workspace::ConfigurationExecutionProjection,
    component: &crate::state::Component,
    instance_path: &str,
) -> Result<Option<CellViewRef>, String> {
    if let Some(plan) = projection.plan() {
        let execution = plan.binding(instance_path).ok_or_else(|| {
            format!("configuration execution plan has no exact binding at {instance_path}")
        })?;
        if execution.stop_boundary()
            || !matches!(
                execution.resolved_view_type(),
                ViewType::Schematic | ViewType::Testbench
            )
        {
            return Ok(None);
        }
        return Ok(Some(execution.resolved_reference().clone()));
    }
    if component
        .library_cell
        .as_ref()
        .is_some_and(|binding| binding.source_path.is_some())
    {
        return Ok(None);
    }
    Ok(component.library_cell.as_ref().and_then(|binding| {
        projection
            .schematic_buffers()
            .keys()
            .find(|key| {
                let mut parts = key.split('/');
                parts
                    .next()
                    .is_some_and(|library| library.eq_ignore_ascii_case(&binding.library))
                    && parts
                        .next()
                        .is_some_and(|cell| cell.eq_ignore_ascii_case(&binding.cell))
                    && parts
                        .next()
                        .is_some_and(|view| view.eq_ignore_ascii_case("schematic"))
            })
            .map(|_| CellViewRef::new(&binding.library, &binding.cell, "schematic"))
    }))
}

fn component_has_model_binding(component: &crate::state::Component) -> bool {
    if !matches!(
        component.kind,
        crate::state::ComponentType::Nmos
            | crate::state::ComponentType::Pmos
            | crate::state::ComponentType::NVdmos
            | crate::state::ComponentType::PVdmos
            | crate::state::ComponentType::NmosSoi
            | crate::state::ComponentType::PmosSoi
    ) {
        return false;
    }
    let params = crate::state::parse_params_string(&component.params);
    params
        .get("model")
        .map(String::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .or_else(|| {
            let value = component.value.trim();
            (!value.is_empty()).then_some(value)
        })
        .is_some()
}

fn collect_authoritative_bin_instances(
    placed_instances: &[PlacedModelInstance],
    family: &BinFamily,
    receipt: &ModelBinInspection,
) -> Vec<BinInstance> {
    let mut instances = Vec::new();
    for decision in &receipt.instances {
        if !family
            .cards
            .iter()
            .any(|card| card.model.eq_ignore_ascii_case(&decision.selected_model))
        {
            continue;
        }
        let Some(placed) = placed_instances.iter().find(|placed| {
            bin_engine_element_name(&placed.path).eq_ignore_ascii_case(&decision.element)
        }) else {
            continue;
        };
        let outcome = match decision.selection {
            ModelBinSelectionKind::ExactCard | ModelBinSelectionKind::FamilyMatch => {
                BinResolution::Resolved(decision.selected_model.clone())
            }
            ModelBinSelectionKind::SharedBoundary => BinResolution::SharedBoundary {
                winner: decision.selected_model.clone(),
                matches: decision.match_count,
            },
        };
        instances.push(BinInstance {
            component_id: placed.component_id,
            owner: placed.owner.clone(),
            name: placed.name.clone(),
            path: placed.path.clone(),
            length: decision.length,
            width: decision.width,
            nfin: decision.nfin,
            multiplier: decision.multiplier,
            outcome,
        });
    }
    instances.sort_by(|left, right| {
        left.path
            .to_ascii_lowercase()
            .cmp(&right.path.to_ascii_lowercase())
            .then_with(|| left.component_id.cmp(&right.component_id))
    });
    instances
}

fn bin_engine_element_name(path: &str) -> String {
    path.trim_start_matches('/')
        .strip_prefix("top/")
        .unwrap_or_else(|| path.trim_start_matches('/'))
        .split('/')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(".")
}

fn bin_totals(
    families: &[BinFamily],
    audits: &[BinAudit],
    instances: &[Vec<BinInstance>],
    inspection_blocked: bool,
) -> BinTotals {
    BinTotals {
        families: families.len(),
        cards: families.iter().map(|family| family.cards.len()).sum(),
        instances: instances.iter().map(Vec::len).sum(),
        deterministic_instances: instances
            .iter()
            .flatten()
            .filter(|instance| instance.outcome.deterministic())
            .count(),
        overlaps: audits.iter().map(|audit| audit.overlaps.len()).sum(),
        gaps: audits.iter().map(|audit| audit.gaps.len()).sum(),
        invalid_cards: audits.iter().map(|audit| audit.invalid_cards.len()).sum(),
        shared_boundaries: audits.iter().map(|audit| audit.shared_boundaries).sum(),
        review_families: audits
            .iter()
            .filter(|audit| !audit.coverage_available)
            .count(),
        inspection_blocked,
    }
}

fn build_bin_audit_draft(
    app: &RSpiceApp,
    authoritative: &AuthoritativeBinInspection,
    families: &[BinFamily],
    audits: &[BinAudit],
) -> ModelBinAuditDraft {
    let project_id = app.state.workspace.project.id();
    let project_revision = app.state.workspace.project.revision().get();
    let simulation_root = app.state.workspace.simulation_root_reference().clone();
    let process = app.state.sim_setup.reference_pvt.process;
    let temperature = app.state.sim_setup.reference_pvt.temperature_celsius;

    let (Some(source_digest), Some(inspection)) =
        (authoritative.source_digest, authoritative.receipt.as_ref())
    else {
        return ModelBinAuditDraft::blocked(
            authoritative.input_digest,
            project_id,
            project_revision,
            simulation_root,
            process,
            temperature,
            authoritative
                .diagnostic
                .clone()
                .unwrap_or_else(|| "Authoritative model-bin inspection is unavailable".to_owned()),
        );
    };

    let mut findings = Vec::new();
    for (family, audit) in families.iter().zip(audits) {
        for (model, reason) in &audit.invalid_cards {
            findings.push(ModelBinAuditFinding::new(
                ModelBinAuditFindingKind::InvalidCard,
                &family.name,
                vec![model.clone()],
                *reason,
                None,
                None,
            ));
        }
        for overlap in &audit.overlaps {
            findings.push(ModelBinAuditFinding::new(
                ModelBinAuditFindingKind::PositiveAreaOverlap,
                &family.name,
                vec![overlap.left.clone(), overlap.right.clone()],
                "Two cards cover the same positive-area L/W region",
                Some(ModelBinAuditAxisRange {
                    min: Some(overlap.region.l_min),
                    max: Some(overlap.region.l_max),
                }),
                Some(ModelBinAuditAxisRange {
                    min: Some(overlap.region.w_min),
                    max: Some(overlap.region.w_max),
                }),
            ));
        }
        for gap in &audit.gaps {
            findings.push(ModelBinAuditFinding::new(
                ModelBinAuditFindingKind::CoverageGap,
                &family.name,
                Vec::new(),
                "No card covers this L/W region inside the declared family envelope",
                Some(ModelBinAuditAxisRange {
                    min: Some(gap.l_min),
                    max: Some(gap.l_max),
                }),
                Some(ModelBinAuditAxisRange {
                    min: Some(gap.w_min),
                    max: Some(gap.w_max),
                }),
            ));
        }
        if !audit.coverage_available {
            findings.push(ModelBinAuditFinding::new(
                ModelBinAuditFindingKind::CoverageUnavailable,
                &family.name,
                family.cards.iter().map(|card| card.model.clone()).collect(),
                if audit.coverage_too_large {
                    "Coverage grid exceeds the bounded audit limit"
                } else {
                    "Complete positive-area L/W rectangles are not declared for every card"
                },
                None,
                None,
            ));
        }
    }

    ModelBinAuditDraft::from_engine(
        authoritative.input_digest,
        source_digest,
        project_id,
        project_revision,
        simulation_root,
        process,
        temperature,
        inspection,
        findings,
    )
}

fn record_bin_audit(
    app: &mut RSpiceApp,
    draft: ModelBinAuditDraft,
) -> Result<ModelBinAuditReceipt, String> {
    if app.state.project_lifecycle.project_open && app.state.workbench.safe_mode.project_read_only()
    {
        return Err("The project is read-only".to_owned());
    }
    let mut candidate = app.state.model_library_manager.clone();
    let receipt = candidate.record_model_bin_audit(draft)?;
    if app.state.project_lifecycle.project_open {
        app.state
            .workspace
            .project
            .next_revision()
            .map_err(|error| error.to_string())?;
        app.state
            .workspace
            .project
            .advance_revision()
            .map_err(|error| error.to_string())?;
        app.state.workspace.project_metadata_dirty = true;
    }
    app.state.model_library_manager = candidate;
    Ok(receipt)
}

fn export_latest_bin_audit_receipt(app: &mut RSpiceApp) -> Result<(), String> {
    let receipt = app
        .state
        .model_library_manager
        .latest_model_bin_audit_receipt()
        .cloned()
        .ok_or_else(|| "No recorded model-bin audit receipt is available".to_owned())?;
    let json = receipt.to_json_pretty()?;
    let digest = receipt.semantic_digest().to_string();
    let default_name = format!("model-bin-audit-{}.json", &digest[..12]);
    let Some(mut path) = app.export_workflow_io.show_save_dialog(
        crate::workbench::workflows::export_workflow::SaveDialogConfig {
            title: "Export model-bin audit receipt",
            default_name: &default_name,
            filter_name: "JSON audit receipt",
            filter_extensions: &["json"],
        },
    )?
    else {
        return Ok(());
    };
    if path.extension().is_none() {
        path.set_extension("json");
    }
    let destination = app.export_workflow_io.observe_destination(&path)?;
    app.export_workflow_io
        .write_text_file_observed(&destination, &json)?;
    app.state.push_user_message(ConsoleMessage::info(format!(
        "Exported model-bin audit receipt {} to {}",
        receipt.semantic_digest(),
        path.display()
    )));
    Ok(())
}

fn announce_bin_audit(
    app: &mut RSpiceApp,
    totals: BinTotals,
    inspection_error: Option<&str>,
    receipt: Option<&ModelBinAuditReceipt>,
) {
    let faults = totals.overlaps + totals.gaps + totals.invalid_cards;
    let message = format!(
        "Geometry-bin audit: {} families, {} cards, {} overlap regions, {} uncovered regions, {} invalid cards, {} of {} placed instances resolve deterministically{}{}.",
        totals.families,
        totals.cards,
        totals.overlaps,
        totals.gaps,
        totals.invalid_cards,
        totals.deterministic_instances,
        totals.instances,
        if totals.review_families > 0 {
            format!(
                "; {} families use partial/non-L-W bounds or exceed the coverage-grid limit and require review",
                totals.review_families
            )
        } else {
            String::new()
        },
        inspection_error.map_or_else(
            || {
                receipt.map_or_else(String::new, |receipt| {
                    format!(
                        "; recorded receipt {} ({})",
                        receipt.semantic_digest(),
                        if receipt.passed() {
                            "passed"
                        } else {
                            "findings"
                        }
                    )
                })
            },
            |error| { format!("; authoritative inspection is blocked: {error}") },
        ),
    );
    let console_message = if faults > 0
        || totals.deterministic_instances < totals.instances
        || totals.review_families > 0
        || totals.inspection_blocked
    {
        ConsoleMessage::warning(message)
    } else {
        ConsoleMessage::info(message)
    };
    app.state.push_user_message(console_message);
}

fn bin_audit_strip(ui: &mut Ui, totals: BinTotals, latest_receipt: Option<&ModelBinAuditReceipt>) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(width, BIN_AUDIT_STRIP_H), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    let faults = totals.overlaps + totals.gaps + totals.invalid_cards;
    let summary = format!(
        "Bin audit: {}. {} overlaps, {} gaps, {} invalid cards, {} shared boundaries, {} of {} instances deterministic{}.",
        if totals.inspection_blocked {
            "inspection blocked"
        } else if faults == 0 && totals.review_families == 0 {
            "clean"
        } else if faults > 0 {
            "faults"
        } else {
            "review required"
        },
        totals.overlaps,
        totals.gaps,
        totals.invalid_cards,
        totals.shared_boundaries,
        totals.deterministic_instances,
        totals.instances,
        if totals.inspection_blocked {
            "; authoritative instance coverage is unavailable"
        } else {
            ""
        }
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), summary.clone())
    });
    let cells = [
        (
            "BIN AUDIT",
            if totals.inspection_blocked {
                "blocked".to_owned()
            } else if faults == 0 && totals.review_families == 0 {
                "clean".to_owned()
            } else if faults > 0 {
                format!("{faults} faults")
            } else {
                "review".to_owned()
            },
            if faults > 0 || totals.inspection_blocked {
                t.color.err
            } else if totals.review_families > 0 {
                t.color.warn
            } else {
                t.color.ok
            },
        ),
        (
            "AMBIGUOUS",
            totals.overlaps.to_string(),
            if totals.overlaps > 0 {
                t.color.err
            } else {
                t.color.ok
            },
        ),
        (
            "UNCOVERED",
            totals.gaps.to_string(),
            if totals.gaps > 0 {
                t.color.err
            } else {
                t.color.ok
            },
        ),
        (
            "RECEIPT",
            latest_receipt.map_or_else(
                || "not recorded".to_owned(),
                |receipt| {
                    format!(
                        "#{} {}",
                        receipt.sequence(),
                        if receipt.passed() {
                            "passed"
                        } else {
                            "findings"
                        }
                    )
                },
            ),
            latest_receipt.map_or(t.color.text_faint, |receipt| {
                if receipt.passed() {
                    t.color.ok
                } else {
                    t.color.warn
                }
            }),
        ),
        (
            "INSTANCES",
            if totals.inspection_blocked {
                "blocked".to_owned()
            } else {
                format!("{} / {}", totals.deterministic_instances, totals.instances)
            },
            if totals.inspection_blocked || totals.deterministic_instances < totals.instances {
                t.color.err
            } else {
                t.color.ok
            },
        ),
    ];
    let cell_width = rect.width() / cells.len() as f32;
    for (index, (label, value, color)) in cells.into_iter().enumerate() {
        let left = rect.left() + index as f32 * cell_width;
        if index > 0 {
            ui.painter().vline(
                left,
                rect.y_range().shrink(10.0),
                Stroke::new(1.0, t.color.border),
            );
        }
        ui.painter().text(
            egui::pos2(left + 12.0, rect.top() + 20.0),
            Align2::LEFT_CENTER,
            label,
            theme::sans(tokens::FS_0, FontWeight::Medium),
            t.color.text_faint,
        );
        ui.painter().text(
            egui::pos2(left + 12.0, rect.top() + 47.0),
            Align2::LEFT_CENTER,
            value,
            theme::mono(tokens::FS_2, FontWeight::SemiBold),
            color,
        );
    }
}

fn bin_family_table(
    ui: &mut Ui,
    families: &[BinFamily],
    audits: &[BinAudit],
    instances: &[Vec<BinInstance>],
    selected_index: Option<usize>,
) -> Option<TableEvent> {
    let t = Tokens::get(ui.ctx());
    let rows = families
        .iter()
        .enumerate()
        .map(|(index, family)| {
            let audit = &audits[index];
            let color = bin_audit_status_color(&t, audit);
            DataRow {
                key: family.key.clone(),
                selected: selected_index == Some(index),
                cells: vec![
                    DataCell::mono(&family.name),
                    DataCell::plain(&family.library),
                    DataCell::plain(&family.device),
                    DataCell::mono(family.cards.len().to_string()),
                    DataCell::mono(instances[index].len().to_string()),
                    DataCell::mono_colored(audit.status(), color),
                ],
            }
        })
        .collect::<Vec<_>>();
    data_table(
        ui,
        "models.bins.families",
        GENERAL_TABLE_MIN_W,
        &[
            ("Family", 0.22),
            ("Library", 0.20),
            ("Device", 0.18),
            ("Cards", 0.10),
            ("Placed", 0.10),
            ("Audit", 0.20),
        ],
        &rows,
        egui::vec2(ui.available_width(), BIN_FAMILY_TABLE_H),
        "No loaded model libraries contain geometry-binned model families.",
    )
}

fn bin_card_table(
    ui: &mut Ui,
    family: &BinFamily,
    audit: &BinAudit,
    selected_model: Option<&str>,
    size: Vec2,
) -> Option<TableEvent> {
    let t = Tokens::get(ui.ctx());
    let rows = family
        .cards
        .iter()
        .map(|card| {
            let invalid = card.invalid_reason();
            DataRow {
                key: card.model.clone(),
                selected: selected_model
                    .is_some_and(|selected| card.model.eq_ignore_ascii_case(selected)),
                cells: vec![
                    DataCell::mono(&card.model),
                    DataCell::mono(format_bin_range(card.length)),
                    DataCell::mono(format_bin_range(card.width)),
                    DataCell::mono(format_bin_range(card.nfin)),
                    DataCell::mono(&card.source),
                    DataCell::mono_colored(
                        invalid.unwrap_or("parsed"),
                        if invalid.is_some() {
                            t.color.err
                        } else {
                            t.color.ok
                        },
                    ),
                ],
            }
        })
        .collect::<Vec<_>>();
    let mut event = None;
    table_card(
        ui,
        "Cards in source order",
        Some((
            &format!(
                "{} cards - {} shared boundaries",
                family.cards.len(),
                audit.shared_boundaries
            ),
            if audit.invalid_cards.is_empty() {
                t.color.text_dim
            } else {
                t.color.err
            },
        )),
        size,
        |ui, body_size| {
            event = data_table(
                ui,
                "models.bins.cards",
                760.0,
                &[
                    ("Card", 0.19),
                    ("Length", 0.17),
                    ("Width", 0.17),
                    ("NFIN", 0.13),
                    ("Source", 0.20),
                    ("Status", 0.14),
                ],
                &rows,
                body_size,
                "The selected family has no parsed cards.",
            );
        },
    );
    event
}

fn bin_findings_table(ui: &mut Ui, family: &BinFamily, audit: &BinAudit, width: f32) {
    let t = Tokens::get(ui.ctx());
    let mut rows = Vec::new();
    for overlap in &audit.overlaps {
        rows.push(DataRow {
            key: format!("overlap:{}:{}", overlap.left, overlap.right),
            selected: false,
            cells: vec![
                DataCell::mono_colored("ambiguous", t.color.err),
                DataCell::mono(format!("{} + {}", overlap.left, overlap.right)),
                DataCell::mono(overlap.region.label()),
                DataCell::plain(
                    "Positive-area overlap; any placed instance in this region blocks circuit construction.",
                ),
            ],
        });
    }
    for (index, gap) in audit.gaps.iter().enumerate() {
        rows.push(DataRow {
            key: format!("gap:{index}"),
            selected: false,
            cells: vec![
                DataCell::mono_colored("uncovered", t.color.err),
                DataCell::mono("no card"),
                DataCell::mono(gap.label()),
                DataCell::plain("No card covers this part of the family envelope."),
            ],
        });
    }
    for (card, reason) in &audit.invalid_cards {
        rows.push(DataRow {
            key: format!("invalid:{card}"),
            selected: false,
            cells: vec![
                DataCell::mono_colored("invalid", t.color.err),
                DataCell::mono(card),
                DataCell::mono(reason.to_string()),
                DataCell::plain("The card is excluded from deterministic resolution."),
            ],
        });
    }
    if !audit.coverage_available && audit.invalid_cards.is_empty() {
        rows.push(DataRow {
            key: "coverage-review".to_owned(),
            selected: false,
            cells: vec![
                DataCell::mono_colored("review", t.color.warn),
                DataCell::mono(&family.name),
                DataCell::mono(if audit.coverage_too_large {
                    "coverage grid limit"
                } else {
                    "partial or non-L/W bounds"
                }),
                DataCell::plain(
                    "Per-instance resolution remains exact; full L/W envelope coverage is not asserted.",
                ),
            ],
        });
    }
    let status = if audit.is_clean() {
        ("clean", t.color.ok)
    } else if audit.fault_count() > 0 {
        ("review required", t.color.err)
    } else {
        ("coverage review", t.color.warn)
    };
    table_card(
        ui,
        "Geometry findings",
        Some(status),
        egui::vec2(width, BIN_FINDINGS_H),
        |ui, body_size| {
            data_table(
                ui,
                "models.bins.findings",
                840.0,
                &[
                    ("Finding", 0.13),
                    ("Cards", 0.22),
                    ("Region", 0.31),
                    ("Consequence", 0.34),
                ],
                &rows,
                body_size,
                "Every point inside the declared L/W envelope is covered by one card.",
            );
        },
    );
}

fn bin_instance_table(
    ui: &mut Ui,
    family: &BinFamily,
    instances: &[BinInstance],
    inspection_error: Option<&str>,
    width: f32,
    t: &Tokens,
) -> Option<TableEvent> {
    let rows = instances
        .iter()
        .map(|instance| {
            let tone = if instance.outcome.deterministic() {
                t.color.ok
            } else {
                t.color.err
            };
            DataRow {
                key: instance.row_key(),
                selected: false,
                cells: vec![
                    DataCell::mono(&instance.name),
                    DataCell::mono(&instance.path),
                    DataCell::mono(format_optional_length(instance.length)),
                    DataCell::mono(format_optional_length(instance.width)),
                    DataCell::mono(
                        instance
                            .nfin
                            .map_or_else(|| "-".to_owned(), format_axis_value),
                    ),
                    DataCell::mono(
                        instance
                            .multiplier
                            .map_or_else(|| "-".to_owned(), format_axis_value),
                    ),
                    DataCell::mono(instance.outcome.winner().unwrap_or("-")),
                    DataCell::mono_colored(instance.outcome.label(), tone),
                ],
            }
        })
        .collect::<Vec<_>>();
    let deterministic = instances
        .iter()
        .filter(|instance| instance.outcome.deterministic())
        .count();
    let mut event = None;
    table_card(
        ui,
        "Instance resolution - activate a row to trace it in the schematic",
        Some((
            &inspection_error.map_or_else(
                || {
                    format!(
                        "{} - {deterministic} / {} resolving",
                        family.name,
                        instances.len()
                    )
                },
                |_| format!("{} - inspection blocked", family.name),
            ),
            if inspection_error.is_some() || deterministic != instances.len() {
                t.color.err
            } else {
                t.color.ok
            },
        )),
        egui::vec2(width, BIN_INSTANCE_TABLE_H),
        |ui, body_size| {
            event = data_table(
                ui,
                "models.bins.instances",
                900.0,
                &[
                    ("Instance", 0.12),
                    ("Path", 0.17),
                    ("Length", 0.12),
                    ("Width", 0.12),
                    ("NFIN", 0.08),
                    ("m", 0.07),
                    ("Winning card", 0.17),
                    ("Outcome", 0.15),
                ],
                &rows,
                body_size,
                inspection_error
                    .unwrap_or("No placed schematic instance explicitly binds this binned family."),
            );
        },
    );
    ui.add_space(6.0);
    let note = "Resolution uses the engine's hierarchy-aware lookup: inclusive edges and declaration order apply only to shared boundaries. An ambiguous or uncovered instance blocks prepared-run publication.";
    let response = ui.add(
        egui::Label::new(
            egui::RichText::new(note)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_dim),
        )
        .wrap(),
    );
    accessible_model_text(ui, &response, note);
    event
}

fn draw_bin_map(ui: &mut Ui, family: &BinFamily, audit: &BinAudit, size: Vec2) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(size.max(Vec2::splat(1.0)), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_inset);
    let Some(envelope) = audit.envelope else {
        let message = if audit.coverage_too_large {
            "The exact L/W coverage grid exceeds the interactive audit limit. Card and instance outcomes remain available in the tables."
        } else {
            "A two-dimensional map requires complete positive LMIN/LMAX and WMIN/WMAX bounds on every card. Partial and NFIN-only cards remain visible in source order."
        };
        let mut child = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(rect.shrink(18.0))
                .layout(Layout::centered_and_justified(egui::Direction::LeftToRight)),
        );
        let label = child.add(
            egui::Label::new(
                egui::RichText::new(message)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
            )
            .wrap(),
        );
        accessible_model_text(&child, &label, message);
        return;
    };
    let plot = Rect::from_min_max(
        egui::pos2(rect.left() + 60.0, rect.top() + 16.0),
        egui::pos2(rect.right() - 18.0, rect.bottom() - 38.0),
    );
    ui.painter().rect(
        plot,
        0.0,
        t.color.bg_panel,
        Stroke::new(1.0, t.color.border_strong),
        egui::StrokeKind::Inside,
    );
    for tick in 0..=4 {
        let fraction = tick as f32 / 4.0;
        let x = egui::lerp(plot.left()..=plot.right(), fraction);
        let y = egui::lerp(plot.bottom()..=plot.top(), fraction);
        ui.painter().vline(
            x,
            plot.y_range(),
            Stroke::new(1.0, t.color.border.gamma_multiply(0.65)),
        );
        ui.painter().hline(
            plot.x_range(),
            y,
            Stroke::new(1.0, t.color.border.gamma_multiply(0.65)),
        );
        let l_value = log_interpolate(envelope.l_min, envelope.l_max, fraction as f64);
        let w_value = log_interpolate(envelope.w_min, envelope.w_max, fraction as f64);
        ui.painter().text(
            egui::pos2(x, plot.bottom() + 13.0),
            Align2::CENTER_CENTER,
            compact_engineering(l_value),
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
        );
        ui.painter().text(
            egui::pos2(plot.left() - 8.0, y),
            Align2::RIGHT_CENTER,
            compact_engineering(w_value),
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
        );
    }
    for (index, card) in family.cards.iter().enumerate() {
        let Some(region) = card.full_lw_rectangle() else {
            continue;
        };
        let card_rect = map_bin_region(plot, envelope, region);
        let fill = if index % 2 == 0 {
            t.color.accent_dim.gamma_multiply(0.72)
        } else {
            t.color.info.gamma_multiply(0.22)
        };
        ui.painter().rect(
            card_rect,
            1.0,
            fill,
            Stroke::new(1.0, t.color.accent.gamma_multiply(0.75)),
            egui::StrokeKind::Inside,
        );
        let suffix = card
            .model
            .strip_prefix(&format!("{}.", family.name))
            .unwrap_or(&card.model);
        ui.painter().text(
            card_rect.center(),
            Align2::CENTER_CENTER,
            suffix,
            theme::mono(tokens::FS_0, FontWeight::SemiBold),
            t.color.text,
        );
    }
    for gap in &audit.gaps {
        ui.painter().rect_filled(
            map_bin_region(plot, envelope, *gap),
            0.0,
            t.color.warn.gamma_multiply(0.32),
        );
    }
    for overlap in &audit.overlaps {
        ui.painter().rect_filled(
            map_bin_region(plot, envelope, overlap.region),
            0.0,
            t.color.err.gamma_multiply(0.38),
        );
    }
    ui.painter().text(
        egui::pos2(plot.center().x, rect.bottom() - 9.0),
        Align2::CENTER_CENTER,
        "L",
        theme::sans(tokens::FS_0, FontWeight::Medium),
        t.color.text_dim,
    );
    ui.painter().text(
        egui::pos2(rect.left() + 14.0, plot.center().y),
        Align2::CENTER_CENTER,
        "W",
        theme::sans(tokens::FS_0, FontWeight::Medium),
        t.color.text_dim,
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Image,
            ui.is_enabled(),
            format!(
                "Logarithmic L/W bin map for {}, {} cards, {} overlap regions, {} uncovered regions.",
                family.name,
                family.cards.len(),
                audit.overlaps.len(),
                audit.gaps.len()
            ),
        )
    });
}

fn map_bin_region(plot: Rect, envelope: BinRegion, region: BinRegion) -> Rect {
    let x = |value: f64| {
        let fraction = (value.log10() - envelope.l_min.log10())
            / (envelope.l_max.log10() - envelope.l_min.log10());
        egui::lerp(plot.left()..=plot.right(), fraction as f32)
    };
    let y = |value: f64| {
        let fraction = (value.log10() - envelope.w_min.log10())
            / (envelope.w_max.log10() - envelope.w_min.log10());
        egui::lerp(plot.bottom()..=plot.top(), fraction as f32)
    };
    Rect::from_min_max(
        egui::pos2(x(region.l_min), y(region.w_max)),
        egui::pos2(x(region.l_max), y(region.w_min)),
    )
}

fn log_interpolate(min: f64, max: f64, fraction: f64) -> f64 {
    10.0_f64.powf(min.log10() + (max.log10() - min.log10()) * fraction)
}

fn compact_engineering(value: f64) -> String {
    for (scale, suffix) in [(1.0e-3, "m"), (1.0e-6, "u"), (1.0e-9, "n"), (1.0e-12, "p")] {
        if value >= scale {
            return format!("{:.3}{}", value / scale, suffix)
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_owned();
        }
    }
    format!("{value:.3e}")
}

fn format_bin_range(range: BinRange) -> String {
    match (range.min, range.max) {
        (Some(min), Some(max)) => {
            format!(
                "{} to {}",
                compact_engineering(min),
                compact_engineering(max)
            )
        }
        (Some(min), None) => format!(">= {}", compact_engineering(min)),
        (None, Some(max)) => format!("<= {}", compact_engineering(max)),
        (None, None) => "-".to_owned(),
    }
}

fn format_optional_length(value: Option<f64>) -> String {
    value.map_or_else(|| "-".to_owned(), format_length)
}

fn format_length(value: f64) -> String {
    format!("{}m", crate::quantity::format_engineering_value(value))
}

fn bin_audit_status_color(t: &Tokens, audit: &BinAudit) -> Color32 {
    if audit.fault_count() > 0 {
        t.color.err
    } else if audit.coverage_available {
        t.color.ok
    } else {
        t.color.warn
    }
}

fn trace_bin_instance(app: &mut RSpiceApp, instance: &BinInstance) {
    if app.state.workspace.active_schematic_reference() != instance.owner {
        app.state.open_workspace_view(instance.owner.clone());
    }
    let Some((component_name, component_position)) = app
        .state
        .schematic
        .components
        .iter()
        .find(|component| component.id == instance.component_id)
        .map(|component| (component.name.clone(), component.pos))
    else {
        app.state
            .push_user_message(ConsoleMessage::warning(
                "Cannot trace the selected bin binding because the schematic instance no longer exists.",
            ));
        return;
    };
    app.state.schematic.selection.clear();
    app.state
        .schematic
        .selection
        .components
        .insert(instance.component_id);
    app.state.schematic.net_highlight.clear();
    app.state.schematic.center_request = Some(component_position);
    app.state.workbench.activate(Workspace::Design);
    app.state.push_user_message(ConsoleMessage::info(format!(
        "Selected schematic instance {component_name} at {} from the geometry-bin audit.",
        instance.path
    )));
}

fn bin_bound_equal(left: f64, right: f64) -> bool {
    (left - right).abs() < BIN_BOUND_TOLERANCE
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(not(target_arch = "wasm32"))]
    use crate::workbench::workflows::export_workflow::{ExportWorkflowIo, SaveDialogConfig};
    #[cfg(not(target_arch = "wasm32"))]
    use std::cell::RefCell;
    #[cfg(not(target_arch = "wasm32"))]
    use std::path::{Path, PathBuf};
    #[cfg(not(target_arch = "wasm32"))]
    use std::rc::Rc;

    #[cfg(not(target_arch = "wasm32"))]
    #[derive(Debug, Clone)]
    struct ReceiptExportIo {
        path: PathBuf,
        dialogs: Rc<RefCell<Vec<(String, String, Vec<String>)>>>,
        writes: Rc<RefCell<Vec<(PathBuf, String)>>>,
    }

    #[cfg(not(target_arch = "wasm32"))]
    impl ExportWorkflowIo for ReceiptExportIo {
        fn show_save_dialog(
            &self,
            config: SaveDialogConfig<'_>,
        ) -> Result<Option<PathBuf>, String> {
            self.dialogs.borrow_mut().push((
                config.title.to_owned(),
                config.default_name.to_owned(),
                config
                    .filter_extensions
                    .iter()
                    .map(|extension| (*extension).to_owned())
                    .collect(),
            ));
            Ok(Some(self.path.clone()))
        }

        fn write_text_file(&self, path: &Path, contents: &str) -> Result<(), String> {
            self.writes
                .borrow_mut()
                .push((path.to_path_buf(), contents.to_owned()));
            Ok(())
        }

        fn write_waveform_csv(
            &self,
            _dataset: &crate::io::WaveformDataset,
            _path: &Path,
        ) -> Result<(), String> {
            Err("waveform export is outside this test".to_owned())
        }
    }

    fn card(
        model: &str,
        l_min: f64,
        l_max: f64,
        w_min: f64,
        w_max: f64,
        source_line: usize,
    ) -> BinCard {
        BinCard {
            model: model.to_owned(),
            source: "models.lib".to_owned(),
            source_order: 0,
            source_line,
            length: BinRange::new(Some(l_min), Some(l_max)),
            width: BinRange::new(Some(w_min), Some(w_max)),
            nfin: BinRange::new(None, None),
        }
    }

    fn family(cards: Vec<BinCard>) -> BinFamily {
        BinFamily {
            key: "test\u{1f}nch".to_owned(),
            library: "test".to_owned(),
            name: "nch".to_owned(),
            device: "NMOS".to_owned(),
            cards,
        }
    }

    #[test]
    fn family_audit_distinguishes_shared_edges_from_positive_overlap() {
        let clean = family(vec![
            card("nch.1", 100e-9, 200e-9, 200e-9, 400e-9, 1),
            card("nch.2", 200e-9, 400e-9, 200e-9, 400e-9, 2),
        ]);
        let audit = audit_bin_family(&clean);
        assert!(audit.is_clean());
        assert_eq!(audit.shared_boundaries, 1);
        assert!(audit.overlaps.is_empty());
        assert!(audit.gaps.is_empty());

        let overlapping = family(vec![
            card("nch.1", 100e-9, 300e-9, 200e-9, 400e-9, 1),
            card("nch.2", 200e-9, 400e-9, 200e-9, 400e-9, 2),
        ]);
        let audit = audit_bin_family(&overlapping);
        assert_eq!(audit.overlaps.len(), 1);
        assert!(!audit.is_clean());
    }

    #[test]
    fn family_audit_finds_uncovered_cells_inside_the_declared_envelope() {
        let family = family(vec![
            card("nch.1", 100e-9, 200e-9, 200e-9, 300e-9, 1),
            card("nch.2", 200e-9, 300e-9, 300e-9, 400e-9, 2),
        ]);
        let audit = audit_bin_family(&family);
        assert!(!audit.gaps.is_empty());
        assert!(!audit.is_clean());
    }

    #[test]
    fn authoritative_instance_rows_use_engine_receipt_geometry_and_hierarchy_name() {
        let family = family(vec![
            card("nch.1", 100e-9, 200e-9, 200e-9, 400e-9, 1),
            card("nch.2", 200e-9, 400e-9, 200e-9, 400e-9, 2),
        ]);
        let placed = vec![PlacedModelInstance {
            component_id: 71,
            owner: CellViewRef::new("work", "child", "schematic"),
            name: "MCHILD".to_owned(),
            path: "/top/XAMP/MCHILD".to_owned(),
        }];
        let receipt = ModelBinInspection {
            cards: Vec::new(),
            instances: vec![rspice_core::engine::ModelBinInstanceInspection {
                element: "XAMP.MCHILD".to_owned(),
                requested_model: "nch".to_owned(),
                selected_model: "nch.1".to_owned(),
                selection: ModelBinSelectionKind::SharedBoundary,
                match_count: 2,
                length: Some(200e-9),
                width: Some(300e-9),
                nfin: None,
                multiplier: Some(2.0),
            }],
        };

        let rows = collect_authoritative_bin_instances(&placed, &family, &receipt);
        let [row] = rows.as_slice() else {
            panic!("expected one authoritative row");
        };
        assert_eq!(row.length, Some(200e-9));
        assert_eq!(row.width, Some(300e-9));
        assert_eq!(row.multiplier, Some(2.0));
        assert_eq!(
            row.outcome,
            BinResolution::SharedBoundary {
                winner: "nch.1".to_owned(),
                matches: 2,
            }
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn receipt_export_uses_json_dialog_extension_and_exact_published_receipt() {
        let mut app = RSpiceApp::test_instance();
        let receipt = app
            .state
            .model_library_manager
            .record_model_bin_audit(ModelBinAuditDraft::blocked(
                crate::product::ContentDigest::from_bytes([0x42; 32]),
                app.state.workspace.project.id(),
                app.state.workspace.project.revision().get(),
                CellViewRef::default_top(),
                crate::simulation::dialog::corner::ProcessCorner::TT,
                27.0,
                "inspection blocked for export test",
            ))
            .expect("receipt records");
        let dialogs = Rc::new(RefCell::new(Vec::new()));
        let writes = Rc::new(RefCell::new(Vec::new()));
        app.export_workflow_io = Box::new(ReceiptExportIo {
            path: PathBuf::from("exports").join("bin-audit"),
            dialogs: Rc::clone(&dialogs),
            writes: Rc::clone(&writes),
        });

        export_latest_bin_audit_receipt(&mut app).expect("receipt exports");

        let dialogs = dialogs.borrow();
        assert_eq!(dialogs.len(), 1);
        assert_eq!(dialogs[0].0, "Export model-bin audit receipt");
        assert!(dialogs[0].1.starts_with("model-bin-audit-"));
        assert_eq!(dialogs[0].2, vec!["json"]);

        let writes = writes.borrow();
        let [(path, contents)] = writes.as_slice() else {
            panic!("expected one receipt publication");
        };
        assert_eq!(path, &PathBuf::from("exports").join("bin-audit.json"));
        let exported: ModelBinAuditReceipt =
            serde_json::from_str(contents).expect("exported receipt parses");
        assert_eq!(exported.semantic_digest(), receipt.semantic_digest());
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn placed_instance_projection_expands_nested_schematic_mos_devices() {
        let mut app = RSpiceApp::test_instance();
        let mut child = crate::state::SchematicState::default();
        let mut mos = crate::state::Component::new(
            71,
            crate::state::ComponentType::Nmos,
            crate::state::Point::new(30, 40),
        )
        .with_name_value("MCHILD", "nch");
        mos.params = "l=180n w=1u m=2".to_owned();
        child.components.push(mos);
        app.state
            .workspace
            .schematic_buffers
            .insert("work/child/schematic".to_owned(), child);

        let binding = crate::state::LibraryCellInstance::new("work", "child", "schematic");
        let parent = crate::state::Component::new(
            12,
            crate::state::ComponentType::CellInstance,
            crate::state::Point::origin(),
        )
        .with_name_value("XAMP", "")
        .with_library_cell(binding);
        app.state.schematic.components.push(parent);

        let projected =
            collect_hierarchical_model_instances(&app).expect("hierarchy should project");
        assert_eq!(projected.len(), 1);
        assert_eq!(projected[0].path, "/top/XAMP/MCHILD");
        assert_eq!(projected[0].owner.key(), "work/child/schematic");
        assert_eq!(projected[0].component_id, 71);
    }
}
