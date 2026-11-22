//! Retained SOA rule projections, table, inspector and stress-history presentation.

use crate::{
    optimization::mono,
    presentation::{PlotView, panel_note, stat_table, table_row_menu, well_hint, with_table_state},
    strip::StripHeader,
};
use egui::{RichText, Ui};
use rspice_results::{
    analysis_payload::AnalysisResultPayload,
    family_metadata::AnalysisResultFamilyMetadata,
    result_presentation::AnalysisPresentationKey,
    safety::{SoAParameter, dynamic_active_interval_indices},
    soa_evidence::{SoaEvaluationEvidence, SoaParameterEvidence, SoaRuleVerdictEvidence},
    waveform::RetainedTrace,
};
use rspice_ui_kit::widgets::table::{Cell, Column, DataTable, LineHeights, SortKey, Tone};
use rspice_ui_kit::{
    panels::section_header,
    plot::{self, Axis, LimitLine, Marker, PlotSpec, Trace, XScale, fmt_si_significant},
    tokens::Tokens,
};
use rspice_vocabulary::quantity::QuantityPresentationPolicy;

use crate::requirement_lines::{self, RequirementLine};
type AnalysisResult = rspice_results::analysis_result::AnalysisResult<RetainedTrace>;

/// The reader's notation for every number the SOA sheet states: engineering
/// prefixes at the displayed significant digits, and a temperature in the
/// unit Units ▸ Temperature chose. Full precision belongs to copy and export.
#[derive(Debug, Clone, Copy)]
pub struct SoaNumbers {
    pub significant_digits: usize,
    pub quantity_policy: QuantityPresentationPolicy,
}

impl SoaNumbers {
    /// A stress, limit, temperature or instant.
    fn value(self, value: f64, unit: &str) -> String {
        if unit == "K" {
            let (scale, offset, shown) = self.quantity_policy.celsius_display_transform();
            return fmt_si_significant(
                (value - 273.15) * scale + offset,
                shown,
                self.significant_digits,
            );
        }
        fmt_si_significant(value, unit, self.significant_digits)
    }

    /// A difference of two values, such as a margin: a temperature
    /// difference scales with the display unit but takes no offset.
    fn difference(self, value: f64, unit: &str) -> String {
        if unit == "K" {
            let (scale, _, shown) = self.quantity_policy.celsius_display_transform();
            return fmt_si_significant(value * scale, shown, self.significant_digits);
        }
        fmt_si_significant(value, unit, self.significant_digits)
    }
}

/// Which safe-operating-area rules the SOA table lists.
///
/// Attention and passing are complements, so a rule is in exactly one of them
/// and the two counts always sum to the evaluated total.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SoaRuleFilter {
    #[default]
    All,
    Violations,
    Passing,
}

/// One SOA rule picked out of the evidence table.
///
/// A rule is identified by its device and the stressed parameter, never by row
/// ordinal: the filter reorders the table and the analysis may be re-run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoaRuleSelection {
    pub analysis: AnalysisPresentationKey,
    pub device_id: String,
    pub parameter: SoaParameterEvidence,
}

/// What one rule's retained stress history says, once someone has read it.
///
/// Both facts cost a scan: locating the history means comparing a candidate
/// waveform's complete time axis and values against the rule's evidence, and
/// the interval means walking outward from the worst sample until the stress
/// drops back under the limit. The table asked for both on every row of every
/// frame, twice — once for the cell and once to decide whether the row's
/// button could be pressed.
#[derive(Debug, Clone, PartialEq)]
pub struct SoaRuleFacts {
    /// Index into the analysis' waveforms of this rule's verified stress
    /// history, when one is retained.
    pub stress_waveform: Option<usize>,
    pub limit_waveform: Option<usize>,
    pub temperature_waveform: Option<usize>,
    /// Where in time the rule's worst stress lies, which the table and the
    /// inspector print in the reader's notation.
    pub worst_interval: SoaWorstInterval,
    /// The stress card's padded axis extents. Both walk the whole history,
    /// and the card asked for them on every frame it was open.
    pub stress_axes: Option<((f64, f64), (f64, f64))>,
    /// Decimation-cache identity for the stress polyline. Without one the
    /// renderer re-reduces every retained sample per frame.
    pub stress_cache_key: u64,
}

/// Where in time a rule's worst stress lies, as its retained history states
/// it. Held as numbers so the plan, built once per dataset generation, does
/// not freeze one notation into the text.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SoaWorstInterval {
    /// The rule passed; only its worst instant is of interest.
    Passed { worst_s: f64 },
    /// The complete stress history was not retained.
    NotRetained { worst_s: f64 },
    /// The retained history does not reproduce the rule's verdict.
    NotReproduced { worst_s: f64 },
    /// The duration-qualified excursion holding the worst sample, between
    /// its linear threshold crossings.
    Excursion {
        qualified: bool,
        start_s: f64,
        end_s: f64,
    },
    /// The run of samples above the warning band or the limit around the
    /// worst sample.
    AboveThreshold {
        warning: bool,
        start_s: f64,
        end_s: f64,
    },
}

impl SoaWorstInterval {
    /// The table's cell.
    fn compact(self, numbers: SoaNumbers) -> String {
        let instant = |seconds| numbers.value(seconds, "s");
        match self {
            Self::Passed { worst_s }
            | Self::NotRetained { worst_s }
            | Self::NotReproduced { worst_s } => format!("at {}", instant(worst_s)),
            Self::Excursion { start_s, end_s, .. }
            | Self::AboveThreshold { start_s, end_s, .. } => {
                format!("{} \u{2026} {}", instant(start_s), instant(end_s))
            }
        }
    }

    /// The inspector's sentence.
    fn full(self, numbers: SoaNumbers) -> String {
        let instant = |seconds| numbers.value(seconds, "s");
        match self {
            Self::Passed { .. } => "Not applicable (rule passed)".to_owned(),
            Self::NotRetained { .. } => {
                "Complete interval was not retained; rerun the analysis".to_owned()
            }
            Self::NotReproduced { .. } => {
                "Retained history does not reproduce the rule verdict".to_owned()
            }
            Self::Excursion {
                qualified,
                start_s,
                end_s,
            } => format!(
                "{} excursion: {} to {} (linear threshold crossings)",
                if qualified {
                    "Qualified"
                } else {
                    "Unqualified"
                },
                instant(start_s),
                instant(end_s)
            ),
            Self::AboveThreshold {
                warning,
                start_s,
                end_s,
            } => format!(
                "{}: {} to {}",
                if warning {
                    "Warning band"
                } else {
                    "Limit exceedance"
                },
                instant(start_s),
                instant(end_s)
            ),
        }
    }
}

/// Scanned stress facts and visible rule ordinals for one retained analysis.
#[derive(Debug, Clone, PartialEq)]
pub struct SoaPlan {
    rules: Vec<SoaRuleFacts>,
    visible: [Vec<usize>; SoaRuleFilter::ALL.len()],
    /// Each device's path against its safe operating area, where its rules
    /// retained the histories to draw one.
    loci: Vec<SoaLocus>,
}

impl SoaPlan {
    pub fn facts(&self, rule: usize) -> Option<&SoaRuleFacts> {
        self.rules.get(rule)
    }

    /// Every device's locus, in the order the evaluations first name it.
    pub fn loci(&self) -> &[SoaLocus] {
        &self.loci
    }

    /// The locus the sheet draws: the selected rule's device's, else the
    /// first that leaves its area, else the first.
    fn locus_for(&self, device: Option<&str>) -> Option<&SoaLocus> {
        device
            .and_then(|device| self.loci.iter().find(|locus| locus.device_id == device))
            .or_else(|| self.loci.iter().find(|locus| locus.crossing.is_some()))
            .or_else(|| self.loci.first())
    }

    /// The rules one filter shows, in retained order.
    fn visible(&self, filter: SoaRuleFilter) -> &[usize] {
        &self.visible[filter.ordinal()]
    }
}

pub fn build_soa_plan(
    version: u64,
    analysis_key: AnalysisPresentationKey,
    analysis: &AnalysisResult,
    evaluations: &[SoaEvaluationEvidence],
    mut on_stress_scan: impl FnMut(),
) -> SoaPlan {
    let identity = {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        version.hash(&mut hasher);
        analysis_key.hash(&mut hasher);
        hasher.finish()
    };
    let rules: Vec<SoaRuleFacts> = evaluations
        .iter()
        .enumerate()
        .map(|(rule, evaluation)| {
            on_stress_scan();
            let stress = stress_waveform(analysis, evaluation);
            let limits = derating_waveform(analysis, evaluation, false);
            let temperature = derating_waveform(analysis, evaluation, true);
            SoaRuleFacts {
                limit_waveform: limits.as_ref().map(|found| found.index),
                temperature_waveform: temperature.as_ref().map(|found| found.index),
                stress_waveform: stress.as_ref().map(|found| found.index),
                worst_interval: worst_interval(
                    stress.as_ref(),
                    limits.as_ref().map(|w| &w.y[..]),
                    evaluation,
                ),
                stress_axes: stress.as_ref().map(|waveform| {
                    let (x_min, x_max) = padded_range(waveform.x.iter().copied(), None);
                    let (y_min, y_max) = padded_range(
                        waveform
                            .y
                            .iter()
                            .copied()
                            .chain(limits.iter().flat_map(|wave| wave.y.iter().copied())),
                        Some([0.0, evaluation.limit_value, evaluation.worst_actual_value]),
                    );
                    ((x_min.max(0.0), x_max), (y_min.max(0.0), y_max))
                }),
                stress_cache_key: identity ^ (rule as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15),
            }
        })
        .collect();
    let visible = SoaRuleFilter::ALL.map(|filter| {
        evaluations
            .iter()
            .enumerate()
            .filter(|(_, evaluation)| filter.matches(evaluation.verdict))
            .map(|(index, _)| index)
            .collect()
    });
    let loci = device_loci(identity, analysis, evaluations, &rules);
    SoaPlan {
        rules,
        visible,
        loci,
    }
}

/// Borrowed validated SOA evidence and its cached presentation facts.
pub struct SoaSource<'a> {
    pub analysis: &'a AnalysisResult,
    pub analysis_key: AnalysisPresentationKey,
    pub evaluations: &'a [SoaEvaluationEvidence],
    pub violation_count: usize,
    pub plan: &'a SoaPlan,
}

pub struct SoaControls<'a> {
    pub selected: Option<&'a SoaRuleSelection>,
    pub filter: SoaRuleFilter,
    pub trace_open: bool,
    pub stress_view: PlotView,
    pub numbers: SoaNumbers,
}

pub struct SoaResponse {
    pub trace_open: bool,
    pub selection: Option<SoaRuleSelection>,
    pub cross_probe: Option<SoaRuleSelection>,
    pub fit_clicked: bool,
    pub stress_view_change: Option<plot::ViewChange>,
    /// The reader followed the rule's limit line to where Verify judges it.
    pub open_check: bool,
    /// What the stress trace's axes spanned this frame, when it was drawn:
    /// the sheet's one pane with a viewport the reader can set.
    pub stress_axes: Option<((f64, f64), (f64, f64))>,
}

struct StressPlot<'a> {
    view: PlotView,
    cache: &'a mut plot::DecimationCache,
    numbers: SoaNumbers,
}

pub struct SoaInspector<'a> {
    pub evaluation: &'a SoaEvaluationEvidence,
    pub facts: Option<&'a SoaRuleFacts>,
    pub event_count: usize,
    pub schematic_available: bool,
    pub numbers: SoaNumbers,
}

pub struct SoaInspectorResponse {
    pub open_trace: bool,
    pub locate_device: bool,
}

pub enum SoaSelectionAbsence {
    Unselected,
    Unavailable,
    Removed,
}

pub fn selection_note(ui: &mut Ui, reason: SoaSelectionAbsence) {
    section_header(ui, "SOA rule selection", None);
    panel_note(
        ui,
        match reason {
            SoaSelectionAbsence::Unselected => "Select a rule.",
            SoaSelectionAbsence::Unavailable => {
                "Select the rule's analysis with valid, successful SOA evidence to inspect it."
            }
            SoaSelectionAbsence::Removed => "The selected SOA rule is no longer retained.",
        },
    );
}

/// What the SOA sheet's bar changed.
pub struct SoaBarResponse {
    pub filter: SoaRuleFilter,
    pub trace_open: bool,
    /// The new filter hides the selected rule, so the selection goes.
    pub deselect: bool,
}

/// Width of the rule filter: its longest choice.
const SOA_FILTER_WIDTH: f32 = 112.0;

/// The SOA sheet's own controls in its bar: which rules the table lists,
/// how many rules need attention and how many pass (the two make the
/// whole), and closing the stress trace while one is open.
pub fn domain_bar(
    ui: &mut Ui,
    source: &SoaSource<'_>,
    selected: Option<&SoaRuleSelection>,
    filter: SoaRuleFilter,
    trace_open: bool,
) -> SoaBarResponse {
    let mut out = SoaBarResponse {
        filter,
        trace_open,
        deselect: false,
    };
    crate::chrome::bars::bar_label(ui, "Show");
    let names = SoaRuleFilter::ALL
        .iter()
        .map(|candidate| candidate.label().to_owned())
        .collect::<Vec<_>>();
    if let Some(picked) = rspice_ui_kit::widgets::select(
        ui,
        "rspice.results.soa-filter",
        "Rules shown",
        filter.label(),
        &names,
        SOA_FILTER_WIDTH,
    ) {
        out.filter = SoaRuleFilter::ALL[picked];
    }
    let attention = source.plan.visible(SoaRuleFilter::Violations).len();
    let passing = source.evaluations.len().saturating_sub(attention);
    ui.add_space(8.0);
    ui.label(
        RichText::new(format!("{attention} attention · {passing} passing"))
            .font(rspice_ui_kit::theme::sans(
                rspice_ui_kit::tokens::FS_0,
                rspice_ui_kit::theme::FontWeight::Regular,
            ))
            .color(Tokens::get(ui.ctx()).color.text),
    );
    if trace_open {
        ui.add_space(8.0);
        if rspice_ui_kit::widgets::Button::new("Close stress trace")
            .show(ui)
            .clicked()
        {
            out.trace_open = false;
        }
    }
    if out.filter != filter
        && let Some(selection) = selected
        && let Some(evaluation) = source.evaluations.iter().find(|evaluation| {
            selection.analysis == source.analysis_key
                && selection.device_id == evaluation.device_id
                && selection.parameter == evaluation.parameter
        })
        && !out.filter.matches(evaluation.verdict)
    {
        out.deselect = true;
        out.trace_open = false;
    }
    out
}

/// The rule table's columns' least widths, which together are the width the
/// table needs before it scrolls sideways.
const RULE_COLUMN_WIDTHS: [f32; 7] = [96.0, 150.0, 104.0, 104.0, 104.0, 176.0, 76.0];

pub fn show(
    ui: &mut Ui,
    source: SoaSource<'_>,
    controls: SoaControls<'_>,
    cache: &mut plot::DecimationCache,
    schematic_available: impl Fn(AnalysisPresentationKey, &str) -> bool,
) -> SoaResponse {
    let SoaSource {
        analysis,
        analysis_key,
        evaluations,
        violation_count: _,
        plan,
    } = source;
    let mut plot = StressPlot {
        view: controls.stress_view,
        cache,
        numbers: controls.numbers,
    };
    let mut stress_view_change = None;
    let mut open_check = false;
    let numbers = controls.numbers;
    let selected = controls.selected;
    let filter = controls.filter;
    let mut trace_open = controls.trace_open;
    let mut requested = selected.cloned();
    let mut cross_probe = None;
    let mut fit_clicked = false;
    let mut stress_axes = None;

    // One layout at every size: the rule table stands at the foot of the
    // sheet, exactly its head and rows tall and ending on the sheet's bottom
    // edge, and the plot fills everything above it: the stress trace the
    // reader opened, else the device's I-V locus. Past its cap the table
    // shows whole rows and scrolls.
    let locus = plan.locus_for(
        selected
            .filter(|selection| selection.analysis == analysis_key)
            .map(|selection| selection.device_id.as_str()),
    );
    let area = ui.available_rect_before_wrap();
    let lines = LineHeights::grid(&Tokens::get(ui.ctx()));
    let rows = plan.visible(filter).len().max(1) as f32;
    let cap = ((area.height() * RULE_TABLE_SHARE - lines.head) / lines.row)
        .floor()
        .max(RULE_TABLE_LEAST_ROWS);
    let table_height = lines.head + rows.min(cap) * lines.row;
    let table_area = egui::Rect::from_min_max(
        egui::pos2(area.left(), (area.bottom() - table_height).max(area.top())),
        area.max,
    );
    let plot_area = egui::Rect::from_min_max(area.min, egui::pos2(area.right(), table_area.top()));
    let mut plot_ui = ui.new_child(
        egui::UiBuilder::new()
            .id_salt("rspice.results.soa-plot")
            .max_rect(plot_area)
            .layout(egui::Layout::top_down(egui::Align::Min)),
    );
    let mut stress_drawn = false;
    if trace_open
        && let Some(selection) = requested.as_ref()
        && selection.analysis == analysis_key
        && let Some((rule, evaluation)) = evaluations.iter().enumerate().find(|(_, evaluation)| {
            selection.device_id == evaluation.device_id
                && selection.parameter == evaluation.parameter
        })
    {
        // The plan already located and verified this rule's history; drawing
        // it must not repeat that scan on every frame the card is open.
        if let Some((facts, waveform)) = plan.facts(rule).and_then(|facts| {
            facts
                .stress_waveform
                .and_then(|index| evidence_trace(analysis, index))
                .map(|waveform| (facts, waveform))
        }) {
            let card = stress_trace_card(
                &mut plot_ui,
                &mut plot,
                waveform,
                evaluation,
                facts,
                facts
                    .limit_waveform
                    .and_then(|i| evidence_trace(analysis, i)),
                facts
                    .temperature_waveform
                    .and_then(|i| evidence_trace(analysis, i)),
            );
            stress_view_change = card.view_change;
            open_check = card.open_check;
            fit_clicked = card.fit_clicked;
            stress_axes = Some(card.axes);
            stress_drawn = true;
        } else {
            legacy_stress_history_note(&mut plot_ui);
        }
    }
    if !stress_drawn && let Some(locus) = locus {
        locus_card(
            &mut plot_ui,
            &mut *plot.cache,
            analysis,
            evaluations,
            locus,
            numbers,
        );
    }
    // With nothing to plot, the table has the whole sheet.
    let table_area = if stress_drawn || locus.is_some() {
        table_area
    } else {
        area
    };

    let mut rule_table = |ui: &mut Ui| {
        let visible_rules = plan.visible(filter);

        // The device and the rule share what the numbers leave and elide first,
        // their whole words on hover; the table scrolls sideways only once the
        // sheet is narrower than every column's floor. A row's actions are its
        // menu: Enter or a double-click opens its stress trace.
        let floor = RULE_COLUMN_WIDTHS;
        let columns = [
            Column::name("Device", floor[0]),
            Column::name("Rule", floor[1]),
            Column::number("Observed", floor[2]),
            Column::number("Limit", floor[3]),
            Column::number("Margin", floor[4]),
            Column::number("Worst interval", floor[5]),
            Column::words("Status", floor[6]),
        ];
        let rule_at = |row: usize| {
            let rule = *visible_rules.get(row)?;
            Some((evaluations.get(rule)?, plan.facts(rule)?))
        };
        let selection_of = |evaluation: &SoaEvaluationEvidence| SoaRuleSelection {
            analysis: analysis_key,
            device_id: evaluation.device_id.clone(),
            parameter: evaluation.parameter,
        };
        let is_selected = |evaluation: &SoaEvaluationEvidence| {
            selected.as_ref().is_some_and(|selection| {
                selection.analysis == analysis_key
                    && selection.device_id == evaluation.device_id
                    && selection.parameter == evaluation.parameter
            })
        };
        let revision =
            egui::Id::new(("soa", analysis_key, filter.label(), visible_rules.len())).value();
        let response = with_table_state(ui, "soa", |ui, state| {
            // The page's selected rule is the table's: one row, looked for once
            // per change of the rule or the rows shown.
            match selected.filter(|selection| selection.analysis == analysis_key) {
                None => state.selection = None,
                Some(selection) => {
                    let lead = state.selected_row().and_then(rule_at);
                    let sought = egui::Id::new((
                        selection.device_id.as_str(),
                        parameter_label(selection.parameter),
                        revision,
                    ))
                    .value();
                    let sought_id = ui.id().with("rspice.results.soa-sought");
                    if lead.is_none_or(|(evaluation, _)| !is_selected(evaluation))
                        && ui.data(|data| data.get_temp::<u64>(sought_id)) != Some(sought)
                    {
                        ui.data_mut(|data| data.insert_temp(sought_id, sought));
                        match (0..visible_rules.len()).find(|row| {
                            rule_at(*row).is_some_and(|(evaluation, _)| is_selected(evaluation))
                        }) {
                            Some(row) => state.select(row, None),
                            None => state.selection = None,
                        }
                    }
                }
            }
            let response = DataTable::new("rspice.results.soa", &columns, visible_rules.len())
                .state(state)
                .freeze(1)
                .revision(revision)
                .max_height(table_height)
                .show(ui, |row| {
                    // `rule` is an ordinal from the memoized plan and
                    // `evaluations` is the live retained evidence. A row the
                    // sheet cannot vouch for is a row of dashes, not a panic.
                    let Some((evaluation, facts)) = rule_at(row) else {
                        return vec![Cell::absent(); 7];
                    };
                    let unit = &evaluation.unit;
                    let margin = evaluation.limit_value - evaluation.worst_actual_value;
                    vec![
                        Cell::text(&evaluation.device_id),
                        Cell::text(parameter_label(evaluation.parameter)),
                        Cell::number(numbers.value(evaluation.worst_actual_value, unit))
                            .sort_by(SortKey::Number(evaluation.worst_actual_value)),
                        Cell::number(numbers.value(evaluation.limit_value, unit))
                            .sort_by(SortKey::Number(evaluation.limit_value)),
                        Cell::number(numbers.difference(margin, unit))
                            .sort_by(SortKey::Number(margin)),
                        Cell::number(facts.worst_interval.compact(numbers)),
                        Cell::text(evaluation.verdict.label())
                            .toned(verdict_tone(evaluation.verdict)),
                    ]
                });
            if (response.clicked.is_some() || response.selection_changed)
                && let Some((evaluation, _)) = state.selected_row().and_then(rule_at)
            {
                requested = Some(selection_of(evaluation));
            }
            if let Some((evaluation, facts)) = response.activate.and_then(rule_at)
                && facts.stress_waveform.is_some()
            {
                requested = Some(selection_of(evaluation));
                trace_open = true;
            }
            response
        });
        table_row_menu(ui, "soa", response.context_menu, |ui, row| {
            let Some((evaluation, facts)) = rule_at(row) else {
                ui.close();
                return;
            };
            let stress = ui
            .add_enabled(
                facts.stress_waveform.is_some(),
                egui::Button::new("Open stress trace"),
            )
            .on_disabled_hover_text(
                "This dataset predates exact SOA stress-history retention; run the analysis again.",
            );
            if stress.clicked() {
                requested = Some(selection_of(evaluation));
                trace_open = true;
                ui.close();
            }
            let schematic = ui
            .add_enabled(
                schematic_available(analysis_key, &evaluation.device_id),
                egui::Button::new("Locate in schematic"),
            )
            .on_disabled_hover_text(
                "The result is stale or the retained device has no exact identity in the active schematic.",
            );
            if schematic.clicked() {
                let selection = selection_of(evaluation);
                requested = Some(selection.clone());
                cross_probe = Some(selection);
                ui.close();
            }
            ui.separator();
            if ui.button("Copy exact row").clicked() {
                ui.ctx().copy_text(format!(
                    "{}\t{}\t{:.17e}\t{:.17e}\t{:.17e}\t{}\t{}",
                    evaluation.device_id,
                    parameter_label(evaluation.parameter),
                    evaluation.worst_actual_value,
                    evaluation.limit_value,
                    evaluation.limit_value - evaluation.worst_actual_value,
                    facts.worst_interval.full(numbers),
                    evaluation.verdict.label(),
                ));
                ui.close();
            }
        });
        if visible_rules.is_empty() {
            well_hint(ui, "No retained SOA rules match the selected filter");
        }
    };
    let mut table_ui = ui.new_child(
        egui::UiBuilder::new()
            .id_salt("rspice.results.soa-rules")
            .max_rect(table_area)
            .layout(egui::Layout::top_down(egui::Align::Min)),
    );
    rule_table(&mut table_ui);
    ui.advance_cursor_after_rect(area);

    SoaResponse {
        trace_open,
        selection: requested,
        cross_probe,
        fit_clicked,
        stress_view_change,
        open_check,
        stress_axes,
    }
}

pub fn right_panel(ui: &mut Ui, source: SoaInspector<'_>) -> SoaInspectorResponse {
    let SoaInspector {
        evaluation,
        facts,
        event_count,
        numbers,
        ..
    } = source;
    let margin = evaluation.limit_value - evaluation.worst_actual_value;

    // The limit's own warning and critical fractions are rule settings and
    // live on the verdict's hover; the sample count and the clipped and
    // unqualified excursion counts are bookkeeping.
    let margin_percent = if evaluation.limit_value.abs() > 0.0 {
        format!(" ({:+.1} %)", 100.0 * margin / evaluation.limit_value.abs())
    } else {
        String::new()
    };
    rspice_ui_kit::panels::section_header_explained(
        ui,
        "Selected SOA rule",
        evaluation.verdict.label(),
        &format!(
            "Warning above {}; critical above {}.",
            threshold_text(evaluation.thresholds.warning_fraction),
            threshold_text(evaluation.thresholds.critical_fraction)
        ),
    );
    stat_table(
        ui,
        &[
            ("Device", evaluation.device_id.clone(), true),
            (
                "Quantity",
                parameter_label(evaluation.parameter).to_owned(),
                false,
            ),
            (
                "Observed",
                numbers.value(evaluation.worst_actual_value, &evaluation.unit),
                true,
            ),
            (
                "Limit",
                numbers.value(evaluation.limit_value, &evaluation.unit),
                false,
            ),
            ("Rating", evaluation.description.clone(), false),
            (
                "Signed margin",
                format!(
                    "{}{margin_percent}",
                    numbers.difference(margin, &evaluation.unit)
                ),
                true,
            ),
            (
                "Worst at",
                format!(
                    "{} ({})",
                    numbers.value(evaluation.worst_time_s, "s"),
                    facts
                        .map_or_else(
                            || worst_interval(None, None, evaluation),
                            |facts| facts.worst_interval,
                        )
                        .full(numbers)
                ),
                false,
            ),
            ("Violation events", event_count.to_string(), false),
        ],
    );
    if let Some(duration) = evaluation.duration {
        stat_table(
            ui,
            &[
                (
                    "Longest excursion",
                    numbers.value(duration.longest_excursion_s, "s"),
                    true,
                ),
                (
                    "Total time above limit",
                    numbers.value(duration.total_exceedance_s, "s"),
                    false,
                ),
                (
                    "Qualified excursions",
                    duration.qualified_excursions.to_string(),
                    true,
                ),
            ],
        );
        if let Some(cumulative) = duration.cumulative {
            stat_table(
                ui,
                &[(
                    "Peak exposure",
                    numbers.value(cumulative.peak_exposure_s, "s"),
                    true,
                )],
            );
        }
    }

    let trace_available = facts.is_some_and(|facts| facts.stress_waveform.is_some());
    let schematic_available = source.schematic_available;
    let mut open_trace = false;
    let mut locate_device = false;
    ui.horizontal_wrapped(|ui| {
        let response = ui
            .add_enabled(trace_available, egui::Button::new("Open stress trace"))
            .on_disabled_hover_text(
                "This legacy dataset does not retain the complete SOA stress history; run it again.",
            );
        open_trace = response.clicked();
        let response = ui
            .add_enabled(
                schematic_available,
                egui::Button::new("Locate in schematic"),
            )
            .on_disabled_hover_text(
                "The result is stale or the retained device has no exact identity in the active schematic.",
            );
        locate_device = response.clicked();
    });
    SoaInspectorResponse {
        open_trace,
        locate_device,
    }
}

impl SoaRuleFilter {
    const ALL: [Self; 3] = [Self::Violations, Self::Passing, Self::All];

    /// Position in [`Self::ALL`], which indexes the plan's visible-row lists.
    const fn ordinal(self) -> usize {
        match self {
            Self::Violations => 0,
            Self::Passing => 1,
            Self::All => 2,
        }
    }

    const fn label(self) -> &'static str {
        match self {
            Self::Violations => "Violations",
            Self::Passing => "Passing",
            Self::All => "All rules",
        }
    }

    const fn matches(self, verdict: SoaRuleVerdictEvidence) -> bool {
        match self {
            Self::Violations => !matches!(verdict, SoaRuleVerdictEvidence::Pass),
            Self::Passing => matches!(verdict, SoaRuleVerdictEvidence::Pass),
            Self::All => true,
        }
    }
}

fn threshold_text(fraction: Option<f64>) -> String {
    fraction
        .map(|value| format!("{}% of limit", value * 100.0))
        .unwrap_or_else(|| "Disabled".into())
}

/// Borrow either the complete SOA observations or a legacy full-grid trace.
#[derive(Clone)]
struct SoaTrace<'a> {
    index: usize,
    name: &'a str,
    x: rspice_results::waveform::SampleSlice<'a>,
    y: rspice_results::waveform::SampleSlice<'a>,
}

impl<'a> SoaTrace<'a> {
    /// The trace of `wave`, once its samples have arrived: a stored trace is read off the frame.
    fn of(wave: &'a RetainedTrace) -> Option<Self> {
        Some(Self {
            index: usize::MAX,
            name: &wave.name,
            x: wave.x.fetch_whole().ready()?.into(),
            y: wave.y.fetch_whole().ready()?.into(),
        })
    }
}

fn evidence_trace(analysis: &AnalysisResult, index: usize) -> Option<SoaTrace<'_>> {
    if let Some(AnalysisResultPayload::Soa {
        source_history: Some(source),
        ..
    }) = &analysis.result_payload
    {
        let wave = source.waveforms.get(index)?;
        Some(SoaTrace {
            index,
            name: &wave.name,
            x: source.time.as_slice().into(),
            y: wave.values.as_slice().into(),
        })
    } else {
        let mut trace = SoaTrace::of(analysis.waveforms.get(index)?)?;
        trace.index = index;
        Some(trace)
    }
}

fn evidence_traces(analysis: &AnalysisResult) -> impl Iterator<Item = SoaTrace<'_>> {
    let count = match &analysis.result_payload {
        Some(AnalysisResultPayload::Soa {
            source_history: Some(source),
            ..
        }) => source.waveforms.len(),
        _ => analysis.waveforms.len(),
    };
    (0..count).filter_map(|index| evidence_trace(analysis, index))
}

fn derating_waveform<'a>(
    analysis: &'a AnalysisResult,
    evaluation: &SoaEvaluationEvidence,
    temperature: bool,
) -> Option<SoaTrace<'a>> {
    let name = if evaluation.envelope.is_some() && !temperature {
        rspice_results::safety::soa_envelope_limit_waveform_name(
            &evaluation.device_id,
            evaluation.parameter.runtime_parameter(),
        )
    } else if temperature {
        evaluation.derating?;
        rspice_results::safety::soa_derating_temperature_waveform_name(&evaluation.device_id)
    } else {
        evaluation.derating?;
        rspice_results::safety::soa_power_limit_waveform_name(&evaluation.device_id)
    };
    let AnalysisResultFamilyMetadata::Soa { time } = analysis.family_metadata.as_ref()? else {
        return None;
    };
    evidence_traces(analysis).find(|wave| {
        wave.name == name && &wave.x[..] == time.as_slice() && wave.y.len() == time.len()
    })
}

fn stress_waveform<'a>(
    analysis: &'a AnalysisResult,
    evaluation: &SoaEvaluationEvidence,
) -> Option<SoaTrace<'a>> {
    let expected_samples = usize::try_from(evaluation.sample_count).ok()?;
    if expected_samples == 0 {
        return None;
    }
    let AnalysisResultFamilyMetadata::Soa { time } = analysis.family_metadata.as_ref()? else {
        return None;
    };
    let name = rspice_results::safety::soa_stress_waveform_name(
        &evaluation.device_id,
        evaluation.parameter.runtime_parameter(),
    );
    let limits = derating_waveform(analysis, evaluation, false);
    if (evaluation.derating.is_some() || evaluation.envelope.is_some()) && limits.is_none() {
        return None;
    }
    evidence_traces(analysis).find(|waveform| {
        if waveform.name != name
            || &waveform.x[..] != time.as_slice()
            || waveform.x.len() != expected_samples
            || waveform.y.len() != expected_samples
        {
            return false;
        }
        let Some(worst_index) = waveform
            .x
            .iter()
            .position(|value| value.to_bits() == evaluation.worst_time_s.to_bits())
        else {
            return false;
        };
        waveform.y[worst_index].to_bits() == evaluation.worst_actual_value.to_bits()
            && (evaluation.duration.is_some()
                || waveform.y.iter().enumerate().all(|(index, value)| {
                    rspice_results::safety::compare_soa_stress(
                        *value,
                        limits
                            .as_ref()
                            .map_or(evaluation.limit_value, |wave| wave.y[index]),
                        evaluation.worst_actual_value,
                        evaluation.limit_value,
                    )
                    .is_le()
                }))
    })
}

fn worst_interval(
    stress: Option<&SoaTrace<'_>>,
    limits: Option<&[f64]>,
    evaluation: &SoaEvaluationEvidence,
) -> SoaWorstInterval {
    let worst_s = evaluation.worst_time_s;
    if evaluation.verdict == SoaRuleVerdictEvidence::Pass {
        return SoaWorstInterval::Passed { worst_s };
    }
    let Some(waveform) = stress else {
        return SoaWorstInterval::NotRetained { worst_s };
    };
    let worst_index = nearest_sample_index(&waveform.x[..], worst_s);
    if let Some(duration) = evaluation.duration {
        use rspice_results::safety::{SoaLimitTrace, scan_soa_duration_with_mode};
        let limit_trace = limits.map_or(
            SoaLimitTrace::Constant(evaluation.limit_value),
            SoaLimitTrace::Samples,
        );
        if let Ok(scan) = scan_soa_duration_with_mode(
            &waveform.x[..],
            &waveform.y[..],
            limit_trace,
            duration.minimum_duration_s,
            duration.mode(),
            || false,
        ) && let Some(excursion) = scan.excursions.iter().find(|excursion| {
            (excursion.first_sample..=excursion.last_sample).contains(&worst_index)
        }) {
            return SoaWorstInterval::Excursion {
                qualified: excursion.qualified,
                start_s: excursion.start_s,
                end_s: excursion.end_s,
            };
        }
    }
    let warning = evaluation.verdict == SoaRuleVerdictEvidence::Warning;
    let threshold = |index: usize| {
        let limit = limits.map_or(evaluation.limit_value, |limits| limits[index]);
        if warning {
            limit
                * evaluation
                    .thresholds
                    .warning_fraction
                    .expect("validated warning threshold")
        } else {
            limit
        }
    };
    let Some((start, end)) =
        dynamic_active_interval_indices(&waveform.y[..], worst_index, threshold)
    else {
        return SoaWorstInterval::NotReproduced { worst_s };
    };
    SoaWorstInterval::AboveThreshold {
        warning,
        start_s: waveform.x[start],
        end_s: waveform.x[end],
    }
}

fn legacy_stress_history_note(ui: &mut Ui) {
    egui::Frame::new()
        .fill(Tokens::get(ui.ctx()).color.bg_panel)
        .stroke(egui::Stroke::new(
            1.0,
            Tokens::get(ui.ctx()).color.border,
        ))
        .inner_margin(egui::Margin::symmetric(11, 8))
        .show(ui, |ui| {
            ui.label(RichText::new("Stress history unavailable").strong());
            ui.label(
                "This legacy dataset retains the exact worst point but not the complete rule history. Run the SOA analysis again to enable the linked trace.",
            );
        });
}

/// What the reader did on the open stress trace.
struct StressCard {
    view_change: Option<plot::ViewChange>,
    open_check: bool,
    fit_clicked: bool,
    axes: ((f64, f64), (f64, f64)),
}

/// Head room over the stress history, as a share of its span: the limit's
/// words and the worst point's tag stand above the curve, not on it.
const STRESS_HEAD_ROOM: f64 = 0.12;

fn stress_trace_card(
    ui: &mut Ui,
    plot: &mut StressPlot<'_>,
    waveform: SoaTrace<'_>,
    evaluation: &SoaEvaluationEvidence,
    facts: &SoaRuleFacts,
    limits: Option<SoaTrace<'_>>,
    temperatures: Option<SoaTrace<'_>>,
) -> StressCard {
    let unit = evaluation.unit.as_str();
    let numbers = plot.numbers;
    let t = Tokens::get(ui.ctx());
    let subtitle = format!(
        "{} · {} · worst {} at {} · limit {} · {} exact samples",
        evaluation.device_id,
        parameter_label(evaluation.parameter),
        numbers.value(evaluation.worst_actual_value, unit),
        numbers.value(evaluation.worst_time_s, "s"),
        numbers.value(evaluation.limit_value, unit),
        waveform.x.len()
    );
    let header = StripHeader::new("Stress", &subtitle, &[])
        .id_salt("rspice.results.soa-stress")
        .maximizable(false)
        .zoomed(plot.view.is_zoomed())
        .show(ui);
    if header.fit_clicked {
        plot.view = PlotView::default();
    }
    let fit_clicked = header.fit_clicked;
    egui::Frame::new()
        .fill(t.color.canvas_bg)
        .inner_margin(egui::Margin::symmetric(8, 7))
        .show(ui, |ui| {
            // The card is a canvas: its text takes the canvas theme's inks,
            // which a light interface over a dark canvas does not share.
            ui.visuals_mut().override_text_color = Some(t.color.canvas_text);

            // The card's own extents are resolved with the plan; deriving
            // them here walked the whole history twice per frame.
            let ((x_min, x_max), (y_min, y_max)) =
                facts.stress_axes.unwrap_or(((0.0, 1.0), (0.0, 1.0)));
            let view = plot.view;
            let (x_min, x_max) = view.x.unwrap_or((x_min, x_max));
            let (y_min, y_max) = view
                .y
                .unwrap_or((y_min, y_max + (y_max - y_min) * STRESS_HEAD_ROOM));
            let title = format!(
                "SOA stress history for {} {}",
                evaluation.device_id,
                parameter_label(evaluation.parameter)
            );
            let detail = format!(
                "Exact retained stress samples; limit at the worst point is {}. Worst point {} at {}. A varying limit uses its retained trace at each time.",
                numbers.value(evaluation.limit_value, unit),
                numbers.value(evaluation.worst_actual_value, unit),
                numbers.value(evaluation.worst_time_s, "s")
            );
            let mut spec = PlotSpec::new(
                Axis::linear(x_min, x_max, "s").with_label("Time"),
                XScale::Linear,
                Axis::linear(y_min, y_max, &evaluation.unit)
                    .with_label(parameter_label(evaluation.parameter)),
            )
            .accessible_name(&title)
            .accessible_detail(&detail);
            spec.traces.push(
                Trace::new(
                    &waveform.x[..],
                    &waveform.y[..],
                    t.color.traces[0],
                )
                // Without an identity the renderer re-reduces every retained
                // sample on every frame the card is open.
                .cache_key(facts.stress_cache_key),
            );
            if let Some(limits) = &limits {
                spec.traces.push(Trace::new(&limits.x[..], &limits.y[..], t.color.err).cache_key(facts.stress_cache_key ^ 0x7BD3_815F_A904_260C));
                mono(ui, if evaluation.duration.is_some() { "Red trace: allowed limit at each sample. Worst point follows duration-qualified severity, then utilization." } else { "Red trace: allowed limit at each sample. Worst point: highest stress / allowed-limit ratio." });
            } else {
                spec.limit_lines.push(LimitLine {
                    y: evaluation.limit_value,
                    color: t.color.err,
                    label: format!("LIMIT {}", numbers.value(evaluation.limit_value, unit)),
                });
            }
            spec.markers.push(Marker::point(
                evaluation.worst_time_s,
                evaluation.worst_actual_value,
                t.color.accent,
                "WORST",
            ));
            let readout = |hover_x: f64| {
                let index = nearest_sample_index(&waveform.x[..], hover_x);
                let mut readout = vec![
                    ("t".to_owned(), numbers.value(waveform.x[index], "s")),
                    (
                        parameter_label(evaluation.parameter).to_owned(),
                        numbers.value(waveform.y[index], unit),
                    ),
                    (
                        "Margin".to_owned(),
                        numbers.difference(
                            limits.as_ref().map_or(evaluation.limit_value, |wave| wave.y[index]) - waveform.y[index],
                            unit,
                        ),
                    ),
                ];
                if let Some(temperatures) = &temperatures {
                    readout.push(("Device temperature".into(), numbers.value(temperatures.y[index], "K")));
                    readout.push(("Allowed power".into(), numbers.value(limits.as_ref().expect("derated limit").y[index], "W")));
                }
                readout
            };
            let height = ui.available_height().max(PLOT_LEAST_HEIGHT);
            let response = ui
                .allocate_ui(egui::vec2(ui.available_width(), height), |ui| {
                    rspice_ui_kit::plot::show(
                        ui,
                        &spec,
                        plot.cache,
                        None,
                        Some(&readout),
                    )
                })
                .inner;
            // A constant limit is the rule's own line, and Verify judges the
            // rule as one of the run's checks.
            let action = format!(
                "Open the {} {} check in Verify",
                evaluation.device_id,
                parameter_label(evaluation.parameter)
            );
            let open_check = limits.is_none()
                && requirement_lines::follow(
                    ui,
                    &response,
                    XScale::Linear,
                    &[(RequirementLine::across(evaluation.limit_value), action)],
                )
                .is_some();
            StressCard {
                view_change: response.view.any().then_some(response.view),
                open_check,
                fit_clicked,
                axes: response.axes,
            }
        }).inner
}

/// What a rule bounds in the plane of its device's output voltage and
/// current.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LocusRole {
    Voltage,
    Current,
    Power,
}

/// The bound `parameter` draws on its device's locus: the output voltage,
/// the output current or the dissipation. A reverse-polarity part lies in
/// another quadrant and draws none.
fn locus_role(parameter: SoaParameterEvidence) -> Option<LocusRole> {
    let parameter = parameter.runtime_parameter();
    if parameter.polarity() == Some(false) {
        return None;
    }
    match parameter.base_parameter() {
        SoAParameter::Vds | SoAParameter::Vce => Some(LocusRole::Voltage),
        SoAParameter::Id | SoAParameter::Ic => Some(LocusRole::Current),
        SoAParameter::Pdiss => Some(LocusRole::Power),
        _ => None,
    }
}

/// Where a device's path first leaves its safe operating area.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SoaCrossing {
    /// The rule crossed there, as an index into the evaluations: of the
    /// rules exceeded at that sample, the one exceeded by the widest ratio.
    pub rule: usize,
    pub sample: usize,
    pub time_s: f64,
    pub voltage: f64,
    pub current: f64,
}

/// One device's path through the plane of its output voltage and current,
/// read off the stress histories its rules retained, against the boundary
/// those rules draw.
#[derive(Debug, Clone, PartialEq)]
pub struct SoaLocus {
    pub device_id: String,
    /// Waveform indices of the voltage and the current history, which share
    /// the analysis' time axis.
    voltage: usize,
    current: usize,
    /// The axes' quantities, by the terminal symbols that fit the X
    /// gutter: a drain's or a collector's.
    labels: (&'static str, &'static str),
    /// The first sample outside the area, when the path leaves it.
    pub crossing: Option<SoaCrossing>,
    /// The allowed current against voltage across the plot, ending in the
    /// drop at the voltage limit.
    boundary: (Vec<f64>, Vec<f64>),
    /// Both axes' decade-aligned extents.
    axes: ((f64, f64), (f64, f64)),
    /// Decimation identities of the path and of the boundary.
    cache_keys: [u64; 2],
}

impl SoaLocus {
    /// The boundary's allowed current at `voltage`, read off its outline:
    /// straight between samples in log-log, where a current limit and a
    /// power limit both are.
    fn allowed_at(&self, voltage: f64) -> Option<f64> {
        let (x, y) = (&self.boundary.0, &self.boundary.1);
        let (first, last) = (*x.first()?, *x.last()?);
        if !(first..=last).contains(&voltage) {
            return None;
        }
        let upper = x.partition_point(|value| *value < voltage).min(x.len() - 1);
        if upper == 0 || x[upper] == voltage || x[upper] == x[upper - 1] {
            return Some(y[upper]);
        }
        let (x0, x1, y0, y1) = (x[upper - 1], x[upper], y[upper - 1], y[upper]);
        let along = (voltage / x0).ln() / (x1 / x0).ln();
        Some(if y0 > 0.0 && y1 > 0.0 {
            y0 * (y1 / y0).powf(along)
        } else {
            y0 + (y1 - y0) * along
        })
    }
}

/// How many decades each axis of the locus spans below its top, unless the
/// path lies wholly lower: the span of a datasheet's SOA chart. A path that
/// idles at leakage would otherwise squeeze the boundary into a corner.
const LOCUS_DECADES: f64 = 4.0;

/// Samples of the boundary between the plot's left edge and the voltage
/// limit, besides the corners, which land on samples of their own.
const BOUNDARY_SAMPLES: usize = 96;

/// Keeps the loci's decimation identities apart from the stress traces'.
const LOCUS_CACHE_SALT: u64 = 0x10C5_5EED_A11E_7A9B;

/// Every device's locus, where its rules retained an output voltage and an
/// output current history.
fn device_loci(
    identity: u64,
    analysis: &AnalysisResult,
    evaluations: &[SoaEvaluationEvidence],
    rules: &[SoaRuleFacts],
) -> Vec<SoaLocus> {
    let mut devices: Vec<(&str, Vec<usize>)> = Vec::new();
    let mut seen = std::collections::HashMap::new();
    for (rule, evaluation) in evaluations.iter().enumerate() {
        if locus_role(evaluation.parameter).is_none() {
            continue;
        }
        let at = *seen
            .entry(evaluation.device_id.as_str())
            .or_insert_with(|| {
                devices.push((evaluation.device_id.as_str(), Vec::new()));
                devices.len() - 1
            });
        devices[at].1.push(rule);
    }
    devices
        .into_iter()
        .enumerate()
        .filter_map(|(ordinal, (device, members))| {
            let base = identity ^ LOCUS_CACHE_SALT;
            device_locus(
                [
                    plot::trace_cache_key(base, 2 * ordinal),
                    plot::trace_cache_key(base, 2 * ordinal + 1),
                ],
                analysis,
                evaluations,
                rules,
                device,
                &members,
            )
        })
        .collect()
}

/// The voltage a current rule's curve was read at, which the analysis
/// retains beside the curve's limit.
fn envelope_voltage(
    analysis: &AnalysisResult,
    evaluation: &SoaEvaluationEvidence,
) -> Option<usize> {
    evaluation.envelope.as_ref()?;
    let AnalysisResultFamilyMetadata::Soa { time } = analysis.family_metadata.as_ref()? else {
        return None;
    };
    let name = rspice_results::safety::soa_envelope_voltage_waveform_name(
        &evaluation.device_id,
        evaluation.parameter.runtime_parameter(),
    );
    evidence_traces(analysis)
        .find(|wave| {
            wave.name == name && &wave.x[..] == time.as_slice() && wave.y.len() == time.len()
        })
        .map(|wave| wave.index)
}

/// A log axis' decade-aligned extent: from under the lowest positive value
/// to over `top`, no deeper than [`LOCUS_DECADES`] unless the path's top lies
/// lower, and at least a decade.
fn decade_extent(low: f64, high: f64, top: f64) -> (f64, f64) {
    let upper = (top * 1.25).log10().ceil();
    let floor = upper - LOCUS_DECADES;
    let under = |value: f64| {
        if value.is_finite() && value > 0.0 {
            (value * 0.8).log10().floor()
        } else {
            floor
        }
    };
    let lower = under(low).max(floor).min(under(high)).min(upper - 1.0);
    (10f64.powf(lower), 10f64.powf(upper))
}

fn device_locus(
    cache_keys: [u64; 2],
    analysis: &AnalysisResult,
    evaluations: &[SoaEvaluationEvidence],
    rules: &[SoaRuleFacts],
    device: &str,
    members: &[usize],
) -> Option<SoaLocus> {
    // Of a device's rules for one bound, one whose history is retained: a
    // curve-bounded current before a constant one, and a magnitude before
    // its positive part.
    let pick = |role: LocusRole| {
        members
            .iter()
            .copied()
            .filter(|rule| {
                locus_role(evaluations[*rule].parameter) == Some(role)
                    && rules[*rule].stress_waveform.is_some()
            })
            .min_by_key(|rule| {
                let evaluation = &evaluations[*rule];
                (
                    evaluation.envelope.is_none(),
                    evaluation
                        .parameter
                        .runtime_parameter()
                        .polarity()
                        .is_some(),
                )
            })
    };
    let current = pick(LocusRole::Current)?;
    let voltage_rule = pick(LocusRole::Voltage);
    let power_rule = pick(LocusRole::Power).or_else(|| {
        members
            .iter()
            .copied()
            .find(|rule| locus_role(evaluations[*rule].parameter) == Some(LocusRole::Power))
    });
    let envelope = evaluations[current].envelope.as_ref();
    // A curve's limit is a function of the voltage it was read at; without
    // a curve the voltage rule's own history is the axis.
    let voltage = envelope
        .and_then(|_| envelope_voltage(analysis, &evaluations[current]))
        .or_else(|| voltage_rule.and_then(|rule| rules[rule].stress_waveform))?;
    let current_waveform = rules[current].stress_waveform?;
    let volts = evidence_trace(analysis, voltage)?;
    let amps = evidence_trace(analysis, current_waveform)?;
    let samples = volts.y.len().min(amps.y.len()).min(volts.x.len());

    // Each bound is judged on its own rule's history and limit, as the rule
    // table judges it: the path leaves the area where the first of them is
    // exceeded.
    let bounds: Vec<(usize, SoaTrace<'_>, Option<SoaTrace<'_>>)> =
        [voltage_rule, Some(current), power_rule]
            .into_iter()
            .flatten()
            .filter_map(|rule| {
                let facts = &rules[rule];
                let stress = evidence_trace(analysis, facts.stress_waveform?)?;
                let limit = facts
                    .limit_waveform
                    .and_then(|index| evidence_trace(analysis, index));
                Some((rule, stress, limit))
            })
            .collect();
    let crossing = (0..samples).find_map(|sample| {
        bounds
            .iter()
            .filter_map(|(rule, stress, limit)| {
                let actual = *stress.y.get(sample)?;
                let allowed = match limit {
                    Some(limit) => *limit.y.get(sample)?,
                    None => evaluations[*rule].limit_value,
                };
                (actual.is_finite() && allowed.is_finite() && actual > allowed)
                    .then(|| (*rule, actual / allowed))
            })
            .max_by(|left, right| left.1.total_cmp(&right.1))
            .map(|(rule, _)| SoaCrossing {
                rule,
                sample,
                time_s: volts.x[sample],
                voltage: volts.y[sample],
                current: amps.y[sample],
            })
    });

    let positive = |value: f64| (value.is_finite() && value > 0.0).then_some(value);
    let v_max = voltage_rule.and_then(|rule| positive(evaluations[rule].limit_value));
    let p_max = power_rule.and_then(|rule| positive(evaluations[rule].limit_value));
    let i_max = match envelope {
        Some(envelope) => positive(envelope.maximum_current_a),
        None => positive(evaluations[current].limit_value),
    };
    let curve_end = envelope
        .and_then(|envelope| envelope.curve.voltages_v.last().copied())
        .and_then(positive);
    let allowed = |voltage: f64| {
        let mut amps = i_max.unwrap_or(f64::INFINITY);
        if let Some(envelope) = envelope {
            amps = amps.min(envelope.curve.limit(voltage).unwrap_or(0.0));
        }
        if let Some(power) = p_max {
            amps = amps.min(power / voltage);
        }
        amps
    };

    let mut v_range = (f64::INFINITY, 0.0_f64);
    let mut i_range = (f64::INFINITY, 0.0_f64);
    for sample in 0..samples {
        if let (Some(v), Some(i)) = (positive(volts.y[sample]), positive(amps.y[sample])) {
            v_range = (v_range.0.min(v), v_range.1.max(v));
            i_range = (i_range.0.min(i), i_range.1.max(i));
        }
    }
    let x_top = [v_max, curve_end]
        .into_iter()
        .flatten()
        .fold(v_range.1, f64::max);
    let y_top = i_max.into_iter().fold(i_range.1, f64::max);
    if !(x_top > 0.0 && y_top > 0.0) {
        return None;
    }
    let x_axis = decade_extent(v_range.0, v_range.1, x_top);
    let y_axis = decade_extent(i_range.0, i_range.1, y_top);

    // The outline runs from the plot's left edge to the voltage limit (or
    // the curve's last voltage, past which nothing is allowed) and drops
    // there to the floor.
    let ((x_low, x_high), (y_low, _)) = (x_axis, y_axis);
    let end = [v_max, curve_end]
        .into_iter()
        .flatten()
        .fold(x_high, f64::min);
    let mut volts_axis: Vec<f64> = (0..=BOUNDARY_SAMPLES)
        .map(|step| x_low * (end / x_low).powf(step as f64 / BOUNDARY_SAMPLES as f64))
        .collect();
    volts_axis.extend(i_max.zip(p_max).map(|(amps, watts)| watts / amps));
    if let Some(envelope) = envelope {
        volts_axis.extend(envelope.curve.voltages_v.iter().copied());
    }
    volts_axis.retain(|voltage| (x_low..=end).contains(voltage));
    volts_axis.sort_by(f64::total_cmp);
    volts_axis.dedup();
    let mut amps_axis: Vec<f64> = volts_axis.iter().map(|voltage| allowed(*voltage)).collect();
    if end < x_high && !volts_axis.is_empty() {
        volts_axis.push(end);
        amps_axis.push(y_low);
    }

    let labels = if matches!(
        evaluations[current]
            .parameter
            .runtime_parameter()
            .base_parameter(),
        SoAParameter::Ic
    ) {
        ("Vce", "Ic")
    } else {
        ("Vds", "Id")
    };
    Some(SoaLocus {
        device_id: device.to_owned(),
        voltage,
        current: current_waveform,
        labels,
        crossing,
        boundary: (volts_axis, amps_axis),
        axes: (x_axis, y_axis),
        cache_keys,
    })
}

/// The most of the sheet's height the rule table takes before it scrolls,
/// and the fewest rows it shows when it does.
const RULE_TABLE_SHARE: f32 = 0.5;
const RULE_TABLE_LEAST_ROWS: f32 = 3.0;
/// The shortest a plot over the rule table is drawn, however little room
/// the table leaves it.
const PLOT_LEAST_HEIGHT: f32 = 120.0;

/// The device's path in its voltage-current plane against the boundary its
/// rules draw, the first crossing marked and named as the rule table names
/// that rule. The canvas fills the room under its header.
fn locus_card(
    ui: &mut Ui,
    cache: &mut plot::DecimationCache,
    analysis: &AnalysisResult,
    evaluations: &[SoaEvaluationEvidence],
    locus: &SoaLocus,
    numbers: SoaNumbers,
) {
    let (Some(volts), Some(amps)) = (
        evidence_trace(analysis, locus.voltage),
        evidence_trace(analysis, locus.current),
    ) else {
        return;
    };
    let crossed = locus
        .crossing
        .and_then(|crossing| Some((crossing, evaluations.get(crossing.rule)?)));
    let subtitle = match crossed {
        Some((crossing, evaluation)) => format!(
            "{} · leaves the area at {} · {}",
            locus.device_id,
            numbers.value(crossing.time_s, "s"),
            parameter_label(evaluation.parameter)
        ),
        None => format!(
            "{} · inside the area · {} samples",
            locus.device_id,
            volts.y.len()
        ),
    };
    StripHeader::new("I\u{2013}V locus", &subtitle, &[])
        .id_salt("rspice.results.soa-locus")
        .maximizable(false)
        .show(ui);
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .fill(t.color.canvas_bg)
        .inner_margin(egui::Margin::symmetric(8, 7))
        .show(ui, |ui| {
            ui.visuals_mut().override_text_color = Some(t.color.canvas_text);
            let ((x_low, x_high), (y_low, y_high)) = locus.axes;
            let title = format!("SOA locus of {}", locus.device_id);
            let detail = match crossed {
                Some((crossing, evaluation)) => format!(
                    "{} against its safe operating area: the path leaves it at {} and {} ({}), past its {} limit.",
                    locus.device_id,
                    numbers.value(crossing.voltage, "V"),
                    numbers.value(crossing.current, "A"),
                    numbers.value(crossing.time_s, "s"),
                    parameter_label(evaluation.parameter).to_lowercase()
                ),
                None => format!(
                    "{} against its safe operating area: the path stays inside it.",
                    locus.device_id
                ),
            };
            let mut spec = PlotSpec::new(
                Axis::log_decades(x_low, x_high, "V").with_label(locus.labels.0),
                XScale::Log10,
                Axis::log_decades(y_low, y_high, "A").with_label(locus.labels.1),
            )
            .with_log_y()
            .accessible_name(&title)
            .accessible_detail(&detail);
            spec.traces.push(
                Trace::new(&locus.boundary.0, &locus.boundary.1, t.color.err)
                    .cache_key(locus.cache_keys[1]),
            );
            spec.traces.push(
                Trace::new(&volts.y[..], &amps.y[..], t.color.traces[0])
                    .parametric()
                    .cache_key(locus.cache_keys[0]),
            );
            if let Some((crossing, evaluation)) = crossed {
                spec.markers.push(Marker::point(
                    crossing.voltage,
                    crossing.current,
                    t.color.accent,
                    parameter_label(evaluation.parameter),
                ));
            }
            let readout = |voltage: f64| {
                let mut rows = vec![("V".to_owned(), numbers.value(voltage, "V"))];
                if let Some(allowed) = locus.allowed_at(voltage) {
                    rows.push(("Allowed".to_owned(), numbers.value(allowed, "A")));
                }
                rows
            };
            let height = ui.available_height().max(PLOT_LEAST_HEIGHT);
            ui.allocate_ui(egui::vec2(ui.available_width(), height), |ui| {
                plot::show(ui, &spec, cache, None, Some(&readout))
            });
        });
}

fn padded_range(values: impl Iterator<Item = f64>, include: Option<[f64; 3]>) -> (f64, f64) {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    for value in values.chain(include.into_iter().flatten()) {
        if value.is_finite() {
            min = min.min(value);
            max = max.max(value);
        }
    }
    if !min.is_finite() || !max.is_finite() {
        return (0.0, 1.0);
    }
    if min == max {
        let pad = min.abs().max(1.0) * 0.05;
        return (min - pad, max + pad);
    }
    let pad = (max - min) * 0.06;
    (min - pad, max + pad)
}

fn nearest_sample_index(x: &[f64], query: f64) -> usize {
    debug_assert!(!x.is_empty());
    let upper = x.partition_point(|value| *value < query).min(x.len() - 1);
    if upper == 0 || (x[upper] - query).abs() < (query - x[upper - 1]).abs() {
        upper
    } else {
        upper - 1
    }
}

fn parameter_label(parameter: SoaParameterEvidence) -> &'static str {
    match parameter {
        SoaParameterEvidence::GateSourceVoltage => "Gate-source voltage",
        SoaParameterEvidence::DrainSourceVoltage => "Drain-source voltage",
        SoaParameterEvidence::GateDrainVoltage => "Gate-drain voltage",
        SoaParameterEvidence::BaseEmitterVoltage => "Base-emitter voltage",
        SoaParameterEvidence::CollectorEmitterVoltage => "Collector-emitter voltage",
        SoaParameterEvidence::BaseCollectorVoltage => "Base-collector voltage",
        SoaParameterEvidence::DrainCurrent => "Drain current",
        SoaParameterEvidence::CollectorCurrent => "Collector current",
        SoaParameterEvidence::PowerDissipation => "Power dissipation",
        SoaParameterEvidence::Temperature => "Temperature",
        SoaParameterEvidence::GateSourceVoltagePositive => "Gate-source voltage · positive",
        SoaParameterEvidence::GateSourceVoltageNegative => "Gate-source voltage · negative",
        SoaParameterEvidence::DrainSourceVoltagePositive => "Drain-source voltage · positive",
        SoaParameterEvidence::DrainSourceVoltageNegative => "Drain-source voltage · negative",
        SoaParameterEvidence::GateDrainVoltagePositive => "Gate-drain voltage · positive",
        SoaParameterEvidence::GateDrainVoltageNegative => "Gate-drain voltage · negative",
        SoaParameterEvidence::BaseEmitterVoltagePositive => "Base-emitter voltage · positive",
        SoaParameterEvidence::BaseEmitterVoltageNegative => "Base-emitter voltage · negative",
        SoaParameterEvidence::CollectorEmitterVoltagePositive => {
            "Collector-emitter voltage · positive"
        }
        SoaParameterEvidence::CollectorEmitterVoltageNegative => {
            "Collector-emitter voltage · negative"
        }
        SoaParameterEvidence::BaseCollectorVoltagePositive => "Base-collector voltage · positive",
        SoaParameterEvidence::BaseCollectorVoltageNegative => "Base-collector voltage · negative",
        SoaParameterEvidence::DrainCurrentPositive => "Drain current · positive",
        SoaParameterEvidence::DrainCurrentNegative => "Drain current · negative",
        SoaParameterEvidence::CollectorCurrentPositive => "Collector current · positive",
        SoaParameterEvidence::CollectorCurrentNegative => "Collector current · negative",
        SoaParameterEvidence::GateCurrent => "Gate current",
        SoaParameterEvidence::GateCurrentPositive => "Gate current · positive",
        SoaParameterEvidence::GateCurrentNegative => "Gate current · negative",
        SoaParameterEvidence::SourceCurrent => "Source current",
        SoaParameterEvidence::SourceCurrentPositive => "Source current · positive",
        SoaParameterEvidence::SourceCurrentNegative => "Source current · negative",
        SoaParameterEvidence::BaseCurrent => "Base current",
        SoaParameterEvidence::BaseCurrentPositive => "Base current · positive",
        SoaParameterEvidence::BaseCurrentNegative => "Base current · negative",
        SoaParameterEvidence::EmitterCurrent => "Emitter current",
        SoaParameterEvidence::EmitterCurrentPositive => "Emitter current · positive",
        SoaParameterEvidence::EmitterCurrentNegative => "Emitter current · negative",
        SoaParameterEvidence::CollectorSubstrateVoltage => "Collector-substrate voltage",
        SoaParameterEvidence::CollectorSubstrateVoltagePositive => {
            "Collector-substrate voltage · positive"
        }
        SoaParameterEvidence::CollectorSubstrateVoltageNegative => {
            "Collector-substrate voltage · negative"
        }
        SoaParameterEvidence::BaseSubstrateVoltage => "Base-substrate voltage",
        SoaParameterEvidence::BaseSubstrateVoltagePositive => "Base-substrate voltage · positive",
        SoaParameterEvidence::BaseSubstrateVoltageNegative => "Base-substrate voltage · negative",
        SoaParameterEvidence::EmitterSubstrateVoltage => "Emitter-substrate voltage",
        SoaParameterEvidence::EmitterSubstrateVoltagePositive => {
            "Emitter-substrate voltage · positive"
        }
        SoaParameterEvidence::EmitterSubstrateVoltageNegative => {
            "Emitter-substrate voltage · negative"
        }
        SoaParameterEvidence::SubstrateCurrent => "Substrate current",
        SoaParameterEvidence::SubstrateCurrentPositive => "Substrate current · positive",
        SoaParameterEvidence::SubstrateCurrentNegative => "Substrate current · negative",
        SoaParameterEvidence::AnodeCathodeVoltage => "Diode anode-cathode voltage",
        SoaParameterEvidence::AnodeCathodeVoltagePositive => {
            "Diode anode-cathode voltage · positive"
        }
        SoaParameterEvidence::AnodeCathodeVoltageNegative => {
            "Diode anode-cathode voltage · negative"
        }
        SoaParameterEvidence::AnodeCurrent => "Diode anode current",
        SoaParameterEvidence::AnodeCurrentPositive => "Diode anode current · positive",
        SoaParameterEvidence::AnodeCurrentNegative => "Diode anode current · negative",

        SoaParameterEvidence::BodySourceVoltage => "Body-source voltage",
        SoaParameterEvidence::BodySourceVoltagePositive => "Body-source voltage · positive",
        SoaParameterEvidence::BodySourceVoltageNegative => "Body-source voltage · negative",
        SoaParameterEvidence::BodyDrainVoltage => "Body-drain voltage",
        SoaParameterEvidence::BodyDrainVoltagePositive => "Body-drain voltage · positive",
        SoaParameterEvidence::BodyDrainVoltageNegative => "Body-drain voltage · negative",
        SoaParameterEvidence::GateBodyVoltage => "Gate-body voltage",
        SoaParameterEvidence::GateBodyVoltagePositive => "Gate-body voltage · positive",
        SoaParameterEvidence::GateBodyVoltageNegative => "Gate-body voltage · negative",
        SoaParameterEvidence::BulkCurrent => "Body-contact current",
        SoaParameterEvidence::BulkCurrentPositive => "Body-contact current · positive",
        SoaParameterEvidence::BulkCurrentNegative => "Body-contact current · negative",
        SoaParameterEvidence::BackgateSourceVoltage => "Backgate-source voltage",
        SoaParameterEvidence::BackgateSourceVoltagePositive => "Backgate-source voltage · positive",
        SoaParameterEvidence::BackgateSourceVoltageNegative => "Backgate-source voltage · negative",
        SoaParameterEvidence::BackgateDrainVoltage => "Backgate-drain voltage",
        SoaParameterEvidence::BackgateDrainVoltagePositive => "Backgate-drain voltage · positive",
        SoaParameterEvidence::BackgateDrainVoltageNegative => "Backgate-drain voltage · negative",
        SoaParameterEvidence::GateBackgateVoltage => "Gate-backgate voltage",
        SoaParameterEvidence::GateBackgateVoltagePositive => "Gate-backgate voltage · positive",
        SoaParameterEvidence::GateBackgateVoltageNegative => "Gate-backgate voltage · negative",
        SoaParameterEvidence::BackgateCurrent => "Backgate current",
        SoaParameterEvidence::BackgateCurrentPositive => "Backgate current · positive",
        SoaParameterEvidence::BackgateCurrentNegative => "Backgate current · negative",
        SoaParameterEvidence::BodyBackgateVoltage => "Body-backgate voltage",
        SoaParameterEvidence::BodyBackgateVoltagePositive => "Body-backgate voltage · positive",
        SoaParameterEvidence::BodyBackgateVoltageNegative => "Body-backgate voltage · negative",
    }
}

/// A verdict cell's tone in the rule table: a violation in the error tone,
/// a rule near its limit in the warning tone, and a pass neutral, because a
/// rule that holds has nothing to say.
fn verdict_tone(value: SoaRuleVerdictEvidence) -> Tone {
    match value {
        SoaRuleVerdictEvidence::Pass => Tone::Neutral,
        SoaRuleVerdictEvidence::Warning => Tone::Caution,
        SoaRuleVerdictEvidence::Violation | SoaRuleVerdictEvidence::Critical => Tone::Error,
    }
}

#[cfg(any(test, feature = "test-support"))]
pub mod test_support {
    use super::*;
    use rspice_results::soa_evidence::{SoaViolationEvidence, SoaViolationSeverityEvidence};

    pub fn soa_analysis(rules: usize, samples: usize, peak: f64) -> AnalysisResult {
        let time: Vec<f64> = (0..samples).map(|index| index as f64 * 1.0e-12).collect();
        let mut waveforms = Vec::new();
        let mut evaluations = Vec::new();
        let mut violations = Vec::new();
        for rule in 0..rules {
            let device_id = format!("M{rule:04}");
            let y: Vec<f64> = (0..samples)
                .map(|index| peak * (index as f64 + 1.0) / samples as f64)
                .collect();
            let worst_actual_value = y[samples - 1];
            let worst_time_s = time[samples - 1];
            waveforms.push(RetainedTrace::new(
                rspice_results::safety::soa_stress_waveform_name(
                    &device_id,
                    rspice_results::safety::SoAParameter::Vds,
                ),
                time.clone(),
                y,
                "#00aaff",
            ));
            evaluations.push(SoaEvaluationEvidence {
                duration: None,
                thresholds: Default::default(),
                envelope: None,
                derating: None,
                device_id: device_id.clone(),
                parameter: SoaParameterEvidence::DrainSourceVoltage,
                limit_value: 3.3,
                worst_actual_value,
                worst_time_s,
                sample_count: samples as u64,
                unit: "V".to_owned(),
                description: "Maximum drain-source voltage".to_owned(),
                verdict: SoaRuleVerdictEvidence::Warning,
            });
            violations.push(SoaViolationEvidence {
                device_id,
                parameter: SoaParameterEvidence::DrainSourceVoltage,
                limit_value: 3.3,
                actual_value: worst_actual_value,
                time_s: worst_time_s,
                severity: SoaViolationSeverityEvidence::Warning,
            });
        }

        AnalysisResult::new(
            1,
            rspice_results::analysis_type::AnalysisType::Soa,
            "SOA",
            0.0,
        )
        .with_family_metadata(AnalysisResultFamilyMetadata::Soa { time })
        .with_waveforms(waveforms)
        .with_result_payload(AnalysisResultPayload::Soa {
            source_history: None,
            evaluations,
            violations,
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn soa_filter_keeps_attention_and_passing_rules_disjoint() {
        for verdict in [
            SoaRuleVerdictEvidence::Warning,
            SoaRuleVerdictEvidence::Violation,
            SoaRuleVerdictEvidence::Critical,
        ] {
            assert!(SoaRuleFilter::Violations.matches(verdict));
            assert!(!SoaRuleFilter::Passing.matches(verdict));
            assert!(SoaRuleFilter::All.matches(verdict));
        }
        assert!(SoaRuleFilter::Passing.matches(SoaRuleVerdictEvidence::Pass));
        assert!(!SoaRuleFilter::Violations.matches(SoaRuleVerdictEvidence::Pass));
        assert!(SoaRuleFilter::All.matches(SoaRuleVerdictEvidence::Pass));
    }

    #[test]
    fn evidence_parameters_map_to_stable_stress_waveform_names() {
        let cases = [
            (SoaParameterEvidence::GateSourceVoltage, "SOA_VGS(M1)"),
            (SoaParameterEvidence::DrainSourceVoltage, "SOA_VDS(M1)"),
            (SoaParameterEvidence::GateDrainVoltage, "SOA_VGD(M1)"),
            (SoaParameterEvidence::BaseEmitterVoltage, "SOA_VBE(M1)"),
            (SoaParameterEvidence::CollectorEmitterVoltage, "SOA_VCE(M1)"),
            (SoaParameterEvidence::BaseCollectorVoltage, "SOA_VBC(M1)"),
            (SoaParameterEvidence::DrainCurrent, "SOA_ID(M1)"),
            (SoaParameterEvidence::CollectorCurrent, "SOA_IC(M1)"),
            (SoaParameterEvidence::PowerDissipation, "SOA_PDISS(M1)"),
            (SoaParameterEvidence::Temperature, "SOA_TEMP(M1)"),
        ];
        for (parameter, expected) in cases {
            assert_eq!(
                rspice_results::safety::soa_stress_waveform_name(
                    "M1",
                    parameter.runtime_parameter()
                ),
                expected
            );
        }
    }

    #[test]
    fn nearest_sample_selection_is_exact_and_deterministic() {
        let x = [0.0, 1.0, 2.0, 4.0];
        assert_eq!(nearest_sample_index(&x, -1.0), 0);
        assert_eq!(nearest_sample_index(&x, 1.5), 1);
        assert_eq!(nearest_sample_index(&x, 3.9), 3);
        assert_eq!(nearest_sample_index(&x, 9.0), 3);
    }

    #[test]
    fn soa_thresholds_control_the_retained_warning_interval() {
        let analysis = test_support::soa_analysis(1, 4, 3.0);
        let Some(AnalysisResultPayload::Soa { evaluations, .. }) = &analysis.result_payload else {
            unreachable!()
        };
        let mut evaluation = evaluations[0].clone();
        evaluation.thresholds.warning_fraction = Some(0.5);
        let stress = SoaTrace::of(&analysis.waveforms[0]).expect("in-memory trace");
        let interval = worst_interval(Some(&stress), None, &evaluation);
        assert_eq!(
            interval,
            SoaWorstInterval::AboveThreshold {
                warning: true,
                start_s: 2e-12,
                end_s: 3e-12
            }
        );
        let numbers = SoaNumbers {
            significant_digits: 4,
            quantity_policy: QuantityPresentationPolicy::default(),
        };
        assert_eq!(interval.full(numbers), "Warning band: 2 ps to 3 ps");
        assert_eq!(interval.compact(numbers), "2 ps…3 ps");
        assert_eq!(threshold_text(None), "Disabled");
        assert_eq!(threshold_text(Some(1.5)), "150% of limit");
    }

    #[test]
    fn the_sheet_states_numbers_at_the_readers_digits_and_temperature_unit() {
        use rspice_vocabulary::quantity::{TemperatureDisplay, UnitsPreferences};
        let mut units = UnitsPreferences::default();
        let celsius = SoaNumbers {
            significant_digits: 4,
            quantity_policy: units.presentation_policy(),
        };
        assert_eq!(celsius.value(-0.012_345_678_9, "A"), "\u{2212}12.35 mA");
        assert_eq!(celsius.value(398.15, "K"), "125 °C");
        assert_eq!(celsius.difference(10.0, "K"), "10 °C");
        units.set_temperature_display(TemperatureDisplay::Fahrenheit);
        let fahrenheit = SoaNumbers {
            significant_digits: 4,
            quantity_policy: units.presentation_policy(),
        };
        assert_eq!(fahrenheit.value(398.15, "K"), "257 °F");
        assert_eq!(fahrenheit.difference(10.0, "K"), "18 °F");
    }

    #[test]
    fn the_plan_states_what_a_direct_scan_of_each_rule_states() {
        let analysis = test_support::soa_analysis(6, 32, 3.0);
        let run = rspice_results::run::SimulationRun::<RetainedTrace>::new(
            1,
            0.0,
            rspice_results::run::ExecutionTarget::LocalDesktop,
        );
        let key = AnalysisPresentationKey::new(run.dataset_id, &analysis);
        let Some(AnalysisResultPayload::Soa { evaluations, .. }) = analysis.result_payload.as_ref()
        else {
            panic!("the fixture retains an SOA payload");
        };
        let plan = build_soa_plan(0, key, &analysis, evaluations, || {});
        for (rule, evaluation) in evaluations.iter().enumerate() {
            let scanned = stress_waveform(&analysis, evaluation);
            let facts = plan.facts(rule).expect("one fact set per rule");
            assert_eq!(facts.stress_waveform.is_some(), scanned.is_some(), "{rule}");
            assert_eq!(
                facts.worst_interval,
                worst_interval(scanned.as_ref(), None, evaluation),
                "{rule}"
            );
            let expected_axes = scanned.map(|waveform| {
                let (x_min, x_max) = padded_range(waveform.x.iter().copied(), None);
                let (y_min, y_max) = padded_range(
                    waveform.y.iter().copied(),
                    Some([0.0, evaluation.limit_value, evaluation.worst_actual_value]),
                );
                ((x_min.max(0.0), x_max), (y_min.max(0.0), y_max))
            });
            assert_eq!(facts.stress_axes, expected_axes, "{rule}");
        }
        // Every rule's stress polyline needs its own decimation identity, or
        // one rule's reduction is served under another rule's name.
        let keys: std::collections::HashSet<u64> = plan
            .rules
            .iter()
            .map(|facts| facts.stress_cache_key)
            .collect();
        assert_eq!(keys.len(), plan.rules.len());
        assert_eq!(plan.visible(SoaRuleFilter::Violations).len(), 6);
        assert_eq!(plan.visible(SoaRuleFilter::Passing).len(), 0);
        assert_eq!(plan.visible(SoaRuleFilter::All).len(), 6);
    }

    /// One device's rules over a shared time axis: each a parameter, its
    /// constant limit and its stress history, judged at the default
    /// thresholds. Every history peaks at its last sample, which is the worst
    /// point each rule states.
    fn device_analysis(
        time: &[f64],
        rules: Vec<(SoaParameterEvidence, f64, Vec<f64>)>,
    ) -> AnalysisResult {
        use rspice_results::safety::{SoARuleVerdict, SoaThresholds};
        let last = time.len() - 1;
        let mut waveforms = Vec::new();
        let mut evaluations = Vec::new();
        for (parameter, limit_value, stress) in rules {
            let thresholds = SoaThresholds::default();
            let verdict = match thresholds.verdict(stress[last], limit_value) {
                SoARuleVerdict::Pass => SoaRuleVerdictEvidence::Pass,
                SoARuleVerdict::Warning => SoaRuleVerdictEvidence::Warning,
                SoARuleVerdict::Violation => SoaRuleVerdictEvidence::Violation,
                SoARuleVerdict::Critical => SoaRuleVerdictEvidence::Critical,
            };
            evaluations.push(SoaEvaluationEvidence {
                duration: None,
                thresholds,
                envelope: None,
                derating: None,
                device_id: "M1".to_owned(),
                parameter,
                limit_value,
                worst_actual_value: stress[last],
                worst_time_s: time[last],
                sample_count: time.len() as u64,
                unit: match locus_role(parameter) {
                    Some(LocusRole::Voltage) => "V",
                    Some(LocusRole::Current) => "A",
                    _ => "W",
                }
                .to_owned(),
                description: parameter_label(parameter).to_owned(),
                verdict,
            });
            waveforms.push(RetainedTrace::new(
                rspice_results::safety::soa_stress_waveform_name(
                    "M1",
                    parameter.runtime_parameter(),
                ),
                time.to_vec(),
                stress,
                "#00aaff",
            ));
        }
        AnalysisResult::new(
            1,
            rspice_results::analysis_type::AnalysisType::Soa,
            "SOA",
            0.0,
        )
        .with_family_metadata(AnalysisResultFamilyMetadata::Soa {
            time: time.to_vec(),
        })
        .with_waveforms(waveforms)
        .with_result_payload(AnalysisResultPayload::Soa {
            source_history: None,
            evaluations,
            violations: Vec::new(),
        })
    }

    fn plan_of(analysis: &AnalysisResult) -> (SoaPlan, &[SoaEvaluationEvidence]) {
        let run = rspice_results::run::SimulationRun::<RetainedTrace>::new(
            1,
            0.0,
            rspice_results::run::ExecutionTarget::LocalDesktop,
        );
        let key = AnalysisPresentationKey::new(run.dataset_id, analysis);
        let Some(AnalysisResultPayload::Soa { evaluations, .. }) = analysis.result_payload.as_ref()
        else {
            panic!("the fixture retains an SOA payload");
        };
        (
            build_soa_plan(0, key, analysis, evaluations, || {}),
            evaluations,
        )
    }

    /// A drain path that climbs past its power limit while its voltage and
    /// current stay inside theirs is marked where it first crosses, and the
    /// mark names the rule the table lists as exceeded; the boundary there
    /// allows the power limit's current, and a path inside every limit is
    /// not marked.
    #[test]
    fn a_locus_crossing_names_the_rule_the_table_lists() {
        use SoaParameterEvidence::{DrainCurrent, DrainSourceVoltage, PowerDissipation};
        let samples = 16;
        let time: Vec<f64> = (0..samples).map(|step| step as f64 * 1.0e-6).collect();
        let ramp = |from: f64, to: f64| -> Vec<f64> {
            (0..samples)
                .map(|step| from + (to - from) * step as f64 / (samples - 1) as f64)
                .collect()
        };
        let (volts, amps) = (ramp(1.0, 8.0), ramp(0.1, 0.5));
        let watts: Vec<f64> = volts.iter().zip(&amps).map(|(v, i)| v * i).collect();
        let with_power_limit = |watt_limit: f64| {
            device_analysis(
                &time,
                vec![
                    (DrainSourceVoltage, 10.0, volts.clone()),
                    (DrainCurrent, 1.0, amps.clone()),
                    (PowerDissipation, watt_limit, watts.clone()),
                ],
            )
        };

        let analysis = with_power_limit(2.0);
        let (plan, evaluations) = plan_of(&analysis);
        let [locus] = plan.loci() else {
            panic!("one device, one locus: {:?}", plan.loci());
        };
        assert_eq!(locus.device_id, "M1");
        assert_eq!(locus.labels, ("Vds", "Id"));
        let crossing = locus.crossing.expect("the path leaves the area");
        let first = watts
            .iter()
            .position(|watts| *watts > 2.0)
            .expect("the ramp passes 2 W");
        assert_eq!(crossing.sample, first);
        assert_eq!(crossing.time_s, time[first]);
        assert_eq!(
            (crossing.voltage, crossing.current),
            (volts[first], amps[first])
        );
        // The rule table's Rule cell for that row, and its verdict.
        let crossed = &evaluations[crossing.rule];
        assert_eq!(parameter_label(crossed.parameter), "Power dissipation");
        assert_ne!(crossed.verdict, SoaRuleVerdictEvidence::Pass);
        assert!(
            evaluations
                .iter()
                .filter(|evaluation| evaluation.parameter != PowerDissipation)
                .all(|evaluation| evaluation.verdict == SoaRuleVerdictEvidence::Pass)
        );

        // At the crossing's voltage the boundary allows the power limit's
        // current, under the current limit; past the voltage limit it drops.
        let allowed = locus
            .allowed_at(crossing.voltage)
            .expect("the crossing lies on the plot");
        assert!(
            (allowed - 2.0 / crossing.voltage).abs() < 1e-9 * allowed,
            "{allowed}"
        );
        assert_eq!(locus.boundary.0.last(), Some(&10.0));
        assert_eq!(locus.boundary.1.last(), Some(&locus.axes.1.0));
        assert_eq!(locus.allowed_at(1.0), Some(1.0));

        let inside = with_power_limit(10.0);
        let (plan, _) = plan_of(&inside);
        assert_eq!(plan.loci().len(), 1);
        assert_eq!(plan.loci()[0].crossing, None);
    }
}
