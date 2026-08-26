//! Specialist Models & PDKs page: qualification.
//!
//! The page answers one question — which model families a release may be
//! signed against, and on what evidence — and it answers it as a matrix rather
//! than as a verdict. A family's row states what it retains per analysis
//! domain, so "qualified" and "has never been run against a transient vector"
//! are visibly different rows instead of two shades of the same badge.
//!
//! Nothing here derives a fact. Every number on the page comes from
//! [`super::super::qualification_summaries`], which is the one derivation of a
//! model's gate in this workspace; this module is the composition and the
//! painters over it.

use super::*;

use super::super::{
    QualificationDomain, QualificationGate, QualificationModelSummary, QualificationPageAction,
    QualificationRailRow, execute_qualification_action, qualification_action_block_reason,
    qualification_rail_rows, qualification_summaries, selected_qualification_summary,
};

/// The matrix lists every model in every loaded library, which on a foundry
/// corpus is thousands of rows. Two lines: the family and its library over the
/// gate and what is pending against it, with the per-domain counts centred
/// between them.
///
/// The band between the groups is a row of the same height for the same
/// reason: the list is virtualised, and a row that measured differently would
/// put every row after it in the wrong place.
const MATRIX_ROW_H: f32 = 38.0;

/// The selection strip under the matrix: one property row and the family's own
/// two workflows.
const SELECTION_FOOTER_H: f32 = 78.0;

/// The matrix's share of the body, from the mockup's `.qualification-body`
/// (`grid-template-columns: minmax(0, 1.5fr) minmax(0, 1fr)`).
const MATRIX_TRACK_FRACTION: f32 = 0.60;
/// The narrowest either track is laid out at before the split stops being
/// clamped and simply divides what there is.
const MATRIX_TRACK_MIN_W: f32 = 360.0;
const CONTRACT_TRACK_MIN_W: f32 = 260.0;

/// The matrix's columns, shared by its header and its rows so a cell can never
/// land under the wrong heading.
///
/// The four domain headings are read off [`QualificationDomain`] itself rather
/// than spelled again here, which is what stops a column from outliving the
/// analyses it counts.
///
/// There is no temperature column: the product's vector vocabulary has no
/// temperature analysis, so a column that read "—" on every family forever
/// would pose a coverage question the page can never answer. What temperature
/// a noise vector runs at is a condition on that vector, stated with it.
const MATRIX_COLUMNS: [(&str, f32); 7] = [
    ("MODEL FAMILY", 0.24),
    (QualificationDomain::Dc.column_label(), 0.09),
    (QualificationDomain::Ac.column_label(), 0.12),
    (QualificationDomain::Transient.column_label(), 0.11),
    (QualificationDomain::Noise.column_label(), 0.09),
    ("REFERENCES", 0.12),
    ("GATE", 0.23),
];

/// Where each domain's cell sits in [`MATRIX_COLUMNS`].
const DOMAIN_COLUMN: usize = 1;
const REFERENCES_COLUMN: usize = 5;
const GATE_COLUMN: usize = 6;

pub(super) fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    let summaries = qualification_summaries(app);
    if selected_qualification_summary(app, &summaries).is_none()
        && let Some(first) = summaries.first()
    {
        app.state.select_model_library(&first.library);
        app.state.workbench.selected_model = Some(first.model.clone());
    }
    let selected = selected_qualification_summary(app, &summaries).cloned();
    let total_vectors = summaries
        .iter()
        .map(|summary| summary.vectors)
        .sum::<usize>();
    let subtitle = format!(
        "{} model families · {} vectors · source-owned release evidence",
        summaries.len(),
        total_vectors
    );
    let mut requested_action = None;
    let compare_blocker = qualification_action_block_reason(
        app,
        selected.as_ref(),
        QualificationPageAction::CompareRelease,
    );
    let release_blocker = qualification_action_block_reason(
        app,
        selected.as_ref(),
        QualificationPageAction::ReviewReleaseBinding,
    );
    let run_blocker = qualification_action_block_reason(
        app,
        selected.as_ref(),
        QualificationPageAction::RunSuite,
    );
    let review_blocker = qualification_action_block_reason(
        app,
        selected.as_ref(),
        QualificationPageAction::ReviewVectors,
    );

    // Authored outermost-right first: the band lays its action cluster out
    // right to left, so the page's one accent primary is written first and
    // lands hard against the right edge.
    section_title(ui, "Model qualification", &subtitle, |ui| {
        let run = Button::new("Run suite")
            .accent()
            .enabled(run_blocker.is_none())
            .show(ui);
        if let Some(reason) = run_blocker.as_deref() {
            run.on_disabled_hover_text(reason);
        } else if run.clicked() {
            requested_action = Some(QualificationPageAction::RunSuite);
        }

        let release = Button::new("Release closure")
            .enabled(release_blocker.is_none())
            .show(ui);
        if let Some(reason) = release_blocker.as_deref() {
            release.on_disabled_hover_text(reason);
        } else if release.clicked() {
            requested_action = Some(QualificationPageAction::ReviewReleaseBinding);
        }

        let compare = Button::new("Compare approved")
            .enabled(compare_blocker.is_none())
            .show(ui);
        if let Some(reason) = compare_blocker.as_deref() {
            compare.on_disabled_hover_text(reason);
        } else if compare.clicked() {
            requested_action = Some(QualificationPageAction::CompareRelease);
        }
    });

    closure_ledger(ui, &summaries, selected.as_ref());
    if summaries.is_empty() {
        page_empty_state(
            ui,
            "No qualification suites are loaded",
            "Attach a project-owned model source and retain versioned vectors before making release claims.",
        );
    } else {
        // One row of two tracks, each filling the region: the matrix and the
        // contract rail both reach the panel's bottom edge and scroll their own
        // content. A body that stopped at its content left five hundred pixels
        // of bare panel under a page whose whole subject is coverage.
        let body = ui.available_height().max(1.0);
        let track = ui.available_width().max(1.0);
        let matrix_w = if track >= MATRIX_TRACK_MIN_W + CONTRACT_TRACK_MIN_W {
            (track * MATRIX_TRACK_FRACTION).clamp(MATRIX_TRACK_MIN_W, track - CONTRACT_TRACK_MIN_W)
        } else {
            track * MATRIX_TRACK_FRACTION
        };
        let contract_w = (track - matrix_w).max(1.0);
        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            ui.allocate_ui_with_layout(
                egui::vec2(matrix_w, body),
                Layout::top_down(Align::Min),
                |ui| {
                    ui.set_min_size(egui::vec2(matrix_w, body));
                    family_matrix(
                        ui,
                        &mut app.state,
                        &summaries,
                        selected.as_ref(),
                        review_blocker.as_deref(),
                        &mut requested_action,
                        body,
                    );
                },
            );
            ui.allocate_ui_with_layout(
                egui::vec2(contract_w, body),
                Layout::top_down(Align::Min),
                |ui| {
                    ui.set_min_size(egui::vec2(contract_w, body));
                    contract_rail(ui, app, selected.as_ref(), &mut requested_action, body);
                },
            );
        });
    }

    if let Some(action) = requested_action {
        execute_qualification_action(app, action);
    }
}

/// The height of the closure band, and of nothing else on the page.
const CLOSURE_LEDGER_H: f32 = 54.0;

/// The band's own padding, and the gaps that hold the chain apart: between a
/// segment's label and its value, between segments, and between the last
/// segment and the verdict that terminates the chain.
const LEDGER_PAD: f32 = 12.0;
const SEGMENT_LABEL_GAP: f32 = 6.0;
const SEGMENT_GAP: f32 = 18.0;
const VERDICT_GAP: f32 = 28.0;

/// One input release closure consumes, as the band states it.
///
/// Held as data rather than painted where it is derived because the band is
/// laid out right to left: the whole cluster has to be measured before its
/// first segment can be placed.
struct ClosureInput {
    label: &'static str,
    value: String,
    /// The fact the band has room for only on hover, and the one a reader who
    /// cannot see the band is read along with the ratio.
    detail: String,
    color: Color32,
}

/// The four closure inputs over the whole loaded corpus.
fn closure_inputs(t: &Tokens, summaries: &[QualificationModelSummary]) -> [ClosureInput; 4] {
    let total_vectors = summaries
        .iter()
        .map(|summary| summary.vectors)
        .sum::<usize>();
    let passing_vectors = summaries
        .iter()
        .map(|summary| summary.passing_vectors)
        .sum::<usize>();
    let evidenced_vectors = summaries
        .iter()
        .map(|summary| summary.evidenced_vectors)
        .sum::<usize>();
    let open_dispositions = summaries
        .iter()
        .map(|summary| summary.open_dispositions)
        .sum::<usize>();
    let open_failed = summaries
        .iter()
        .map(|summary| summary.open_failed)
        .sum::<usize>();
    let open_stale = summaries
        .iter()
        .map(|summary| summary.open_stale)
        .sum::<usize>();
    let qualified = summaries
        .iter()
        .filter(|summary| summary.gate == QualificationGate::Qualified)
        .count();
    let parity = summaries
        .iter()
        .filter(|summary| summary.suites > 0 && summary.parity_suites == summary.suites)
        .count();
    // The last two segments are ratios over the population the gate governs.
    // Totalling the exempt models into the denominator is what made a fresh
    // project read "0 / 16 qualified" — a failure claim assembled entirely out
    // of models the gate has nothing to say about.
    let gate_subjects = summaries
        .iter()
        .filter(|summary| summary.gate.is_gate_subject())
        .count();
    let exempt = summaries.len() - gate_subjects;
    // A ratio over an empty population carries no verdict, so it is painted in
    // neither verdict's colour: "0/0" in the warning tone is a warning about
    // nothing, and on the fresh project above it was four of them.
    let verdict = |whole: usize, part: usize| {
        if whole == 0 {
            t.color.text_dim
        } else if part == whole {
            t.color.ok
        } else {
            t.color.warn
        }
    };
    [
        ClosureInput {
            label: "vectors",
            value: format!("{passing_vectors}/{total_vectors}"),
            // The split the retained record actually carries. A disposition
            // holds a cause and a required action and nothing else — no
            // severity, no age, no measured miss — so "worst open" is not a
            // fact this page could state, and the two causes are.
            detail: if open_dispositions == 0 {
                "no open dispositions".to_owned()
            } else {
                format!("{open_failed} failed · {open_stale} stale open")
            },
            color: verdict(total_vectors, passing_vectors),
        },
        ClosureInput {
            label: "references",
            value: format!("{evidenced_vectors}/{total_vectors}"),
            detail: "exact retained evidence".to_owned(),
            color: verdict(total_vectors, evidenced_vectors),
        },
        ClosureInput {
            label: "qualified",
            value: format!("{qualified}/{gate_subjects}"),
            detail: if exempt == 0 {
                "source-owned release gates".to_owned()
            } else {
                format!("{exempt} engine-owned exempt")
            },
            color: verdict(gate_subjects, qualified),
        },
        ClosureInput {
            label: "parity",
            value: format!("{parity}/{gate_subjects}"),
            detail: "desktop · WebAssembly".to_owned(),
            color: verdict(gate_subjects, parity),
        },
    ]
}

/// The page's one aggregate: what the gate is, the inputs it consumes, and the
/// verdict over the selection — one band, read left to right.
///
/// The inputs are inline segments rather than bordered cells because four
/// boxed ratios over a page whose subject is already a coverage matrix say
/// nothing the matrix does not, and say it in the register of a dashboard.
/// What the band adds is the chain: these numbers are what closure reads, and
/// that verdict is what it returns.
fn closure_ledger(
    ui: &mut Ui,
    summaries: &[QualificationModelSummary],
    selected: Option<&QualificationModelSummary>,
) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), CLOSURE_LEDGER_H),
        Sense::hover(),
    );
    ui.painter().rect(
        rect,
        0.0,
        t.color.bg_inset,
        Stroke::new(1.0, t.color.border_strong),
        egui::StrokeKind::Inside,
    );

    let title_font = theme::sans(tokens::FS_0, FontWeight::SemiBold);
    let title_w = ui
        .painter()
        .layout_no_wrap("Release closure".to_owned(), title_font.clone(), t.color.text)
        .size()
        .x;
    ui.painter().text(
        egui::pos2(rect.left() + LEDGER_PAD, rect.top() + 16.0),
        egui::Align2::LEFT_CENTER,
        "Release closure",
        title_font,
        t.color.text,
    );

    // The verdict owns the right edge and is placed first, because everything
    // to its left is laid out against where it ends.
    let gate = selected.map(|summary| summary.gate);
    let verdict_color = gate.map_or(t.color.text_faint, |gate| gate_color(gate, &t));
    let verdict_label = gate
        .map_or("NO SELECTION", QualificationGate::label)
        .to_uppercase();
    let verdict_font = theme::mono(tokens::FS_0, FontWeight::SemiBold);
    let verdict_w = ui
        .painter()
        .layout_no_wrap(verdict_label.clone(), verdict_font.clone(), verdict_color)
        .size()
        .x;
    ui.painter().text(
        egui::pos2(rect.right() - LEDGER_PAD, rect.center().y),
        egui::Align2::RIGHT_CENTER,
        verdict_label,
        verdict_font,
        verdict_color,
    );

    let inputs = closure_inputs(&t, summaries);
    let label_font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let value_font = theme::mono(tokens::FS_0, FontWeight::SemiBold);
    let measured: [(f32, f32); 4] = std::array::from_fn(|index| {
        let input = &inputs[index];
        (
            ui.painter()
                .layout_no_wrap(
                    input.label.to_owned(),
                    label_font.clone(),
                    t.color.text_faint,
                )
                .size()
                .x,
            ui.painter()
                .layout_no_wrap(input.value.clone(), value_font.clone(), input.color)
                .size()
                .x,
        )
    });
    let cluster_w = measured
        .iter()
        .map(|(label, value)| label + SEGMENT_LABEL_GAP + value)
        .sum::<f32>()
        + SEGMENT_GAP * (measured.len() - 1) as f32;
    let cluster_right = rect.right() - LEDGER_PAD - verdict_w - VERDICT_GAP;
    let cluster_left = cluster_right - cluster_w;
    // The cluster is anchored off the verdict so the chain always terminates in
    // the same place, and floored at the title so a band too narrow to hold
    // both clips its leftmost segment rather than painting one over the band's
    // own heading.
    let floor = rect.left() + LEDGER_PAD + title_w + 16.0;
    let clip_left = cluster_left.max(floor);
    let clip = egui::Rect::from_min_max(
        egui::pos2(clip_left, rect.top()),
        egui::pos2(cluster_right.max(clip_left), rect.bottom()),
    )
    .intersect(ui.clip_rect());
    let painter = ui.painter().with_clip_rect(clip);

    let mut x = cluster_left;
    for (index, (input, (label_w, value_w))) in inputs.iter().zip(measured).enumerate() {
        let segment = egui::Rect::from_min_max(
            egui::pos2(x, rect.top() + 1.0),
            egui::pos2(x + label_w + SEGMENT_LABEL_GAP + value_w, rect.bottom() - 1.0),
        );
        x = segment.right() + SEGMENT_GAP;
        painter.text(
            egui::pos2(segment.left(), rect.center().y),
            egui::Align2::LEFT_CENTER,
            input.label,
            label_font.clone(),
            t.color.text_faint,
        );
        painter.text(
            egui::pos2(segment.left() + label_w + SEGMENT_LABEL_GAP, rect.center().y),
            egui::Align2::LEFT_CENTER,
            &input.value,
            value_font.clone(),
            input.color,
        );
        // Painted glyphs publish no accessibility node, so each segment states
        // its own whole line — the count, the ratio, and the fact the band has
        // room for only on hover.
        let hover = ui.interact(
            segment.intersect(clip),
            ui.id().with(("qualification-closure-input", index)),
            Sense::hover(),
        );
        let announced = format!("{} {}, {}", input.label, input.value, input.detail);
        hover.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), &announced)
        });
        hover.on_hover_text(&input.detail);
    }

    // The verdict is the chain's terminal rather than a fifth input, and a
    // hairline says so where a wider gap alone would not.
    ui.painter().vline(
        cluster_right + VERDICT_GAP / 2.0,
        egui::Rangef::new(rect.center().y - 9.0, rect.center().y + 9.0),
        Stroke::new(1.0, t.color.border),
    );

    // What the band says is a statement about the *selected* model, so for one
    // the gate does not govern it states the exemption rather than reciting a
    // closure contract that will never be applied to it.
    let contract = if selected.is_some_and(|summary| !summary.gate.is_gate_subject()) {
        "Engine-owned: this card is compiled into the simulator and is exempt from the source-owned release gate."
    } else {
        "Release closure consumes exact source revisions, retained references, runtime parity, and governed dispositions."
    };
    // The sentence is the band's first casualty: it is clipped to stop before
    // the cluster begins, so a long contract can never reach the ledger.
    let sentence_x = rect.left() + LEDGER_PAD;
    let sentence_w = (clip_left - LEDGER_PAD - sentence_x).max(1.0);
    ui.painter().text(
        egui::pos2(sentence_x, rect.bottom() - 13.0),
        egui::Align2::LEFT_CENTER,
        elide(ui, contract, sentence_w, false),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
}

/// The family × domain matrix, filling the track it was handed.
fn family_matrix(
    ui: &mut Ui,
    state: &mut crate::workbench::AppState,
    summaries: &[QualificationModelSummary],
    selected: Option<&QualificationModelSummary>,
    review_blocker: Option<&str>,
    requested_action: &mut Option<QualificationPageAction>,
    height: f32,
) {
    let selected_key = selected.map(|summary| summary.key.as_str());
    let rows = qualification_rail_rows(summaries);
    let exempt = summaries
        .iter()
        .filter(|summary| !summary.gate.is_gate_subject())
        .count();
    // "16 source revisions" over sixteen compiled-in cards names something none
    // of them has, so the meta counts the two populations apart.
    let meta = if exempt == 0 {
        format!("{} source revisions", summaries.len())
    } else {
        format!(
            "{} source · {exempt} engine-owned",
            summaries.len() - exempt
        )
    };
    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border))
        .inner_margin(egui::Margin::ZERO)
        .show(ui, |ui| {
            // The caller measures the track it has; the pane's own border is
            // drawn inside that, so the content box is the track less the two
            // hairlines.
            let height = (height - 2.0).max(DETAIL_PANE_MIN_H);
            ui.set_min_width(ui.available_width().max(1.0));
            ui.set_min_height(height);
            ui.set_max_height(height);
            egui::Frame::NONE
                .inner_margin(egui::Margin::symmetric(12, 6))
                .show(ui, |ui| {
                    card_title(ui, "FAMILY COVERAGE BY DOMAIN", Some(&meta));
                });
            table_header(ui, &MATRIX_COLUMNS);
            // The rows take what the title band and the header left, less the
            // footer. Asking the ui for the remainder rather than subtracting
            // a copy of the design system's own band height is what keeps the
            // matrix reaching the pane's bottom edge when that height moves.
            let footer = if selected.is_some() {
                SELECTION_FOOTER_H
            } else {
                0.0
            };
            let list_height = (ui.available_height() - footer).max(MATRIX_ROW_H * 2.0);
            ScrollArea::vertical()
                .id_salt("models-qualification-family-matrix")
                .max_height(list_height)
                .auto_shrink([false, false])
                .show_rows(ui, MATRIX_ROW_H, rows.len(), |ui, range| {
                    for row in &rows[range] {
                        let summary = match row {
                            QualificationRailRow::Band(label) => {
                                matrix_band(ui, label);
                                continue;
                            }
                            QualificationRailRow::Model(summary) => *summary,
                        };
                        if matrix_row(ui, summary, selected_key == Some(summary.key.as_str()))
                            .clicked()
                        {
                            state.select_model_library(&summary.library);
                            state.workbench.selected_model = Some(summary.model.clone());
                        }
                    }
                });
            if let Some(selected) = selected {
                selection_footer(ui, review_blocker, selected, requested_action);
            }
        });
}

/// The label over a group of matrix rows.
///
/// Painted like [`table_header`] — this workspace's one spelling of "this is a
/// label over the rows below it" — and it publishes its own accessibility node,
/// because a band that only exists as painted glyphs would leave the exemption
/// legible to sighted readers alone.
fn matrix_band(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), MATRIX_ROW_H),
        Sense::hover(),
    );
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), label));
    let band = egui::Rect::from_min_max(
        egui::pos2(rect.left(), rect.bottom() - 22.0),
        rect.right_bottom(),
    );
    ui.painter().rect_filled(band, 0.0, t.color.bg_inset);
    ui.painter()
        .hline(band.x_range(), band.top(), Stroke::new(1.0, t.color.border));
    ui.painter().text(
        egui::pos2(band.left() + 8.0, band.center().y),
        egui::Align2::LEFT_CENTER,
        elide(ui, label, (band.width() - 16.0).max(1.0), false),
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text_faint,
    );
}

/// One family's row: what it retains in each domain, and the gate over it.
fn matrix_row(ui: &mut Ui, summary: &QualificationModelSummary, selected: bool) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), MATRIX_ROW_H),
        Sense::click(),
    );
    if selected {
        ui.painter()
            .rect_filled(rect, 0.0, t.color.accent.linear_multiply(0.14));
        ui.painter().vline(
            rect.left(),
            rect.y_range(),
            Stroke::new(2.0, t.color.accent),
        );
    } else if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }

    let column = |index: usize| {
        let left = rect.left()
            + rect.width()
                * MATRIX_COLUMNS[..index]
                    .iter()
                    .map(|(_, fraction)| fraction)
                    .sum::<f32>();
        (left + 5.0, rect.width() * MATRIX_COLUMNS[index].1 - 10.0)
    };

    let (family_x, family_w) = column(0);
    ui.painter().text(
        egui::pos2(family_x, rect.top() + 12.0),
        egui::Align2::LEFT_CENTER,
        elide(ui, &summary.model, family_w.max(1.0), true),
        theme::mono(tokens::FS_0, FontWeight::SemiBold),
        t.color.text,
    );
    ui.painter().text(
        egui::pos2(family_x, rect.bottom() - 9.0),
        egui::Align2::LEFT_CENTER,
        elide(ui, &summary.library, family_w.max(1.0), false),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );

    for (offset, domain) in QualificationDomain::ALL.into_iter().enumerate() {
        let (x, width) = column(DOMAIN_COLUMN + offset);
        let (text, color) = domain_cell(summary, domain, &t);
        ui.painter().text(
            egui::pos2(x, rect.center().y),
            egui::Align2::LEFT_CENTER,
            elide(ui, &text, width.max(1.0), true),
            theme::mono(tokens::FS_0, FontWeight::Regular),
            color,
        );
    }

    let (references_x, references_w) = column(REFERENCES_COLUMN);
    let references = if summary.gate.is_gate_subject() && summary.references > 0 {
        format!("{} refs", summary.references)
    } else {
        "—".to_owned()
    };
    ui.painter().text(
        egui::pos2(references_x, rect.center().y),
        egui::Align2::LEFT_CENTER,
        elide(ui, &references, references_w.max(1.0), true),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        if summary.references > 0 {
            t.color.text_dim
        } else {
            t.color.text_faint
        },
    );

    let (gate_x, gate_w) = column(GATE_COLUMN);
    ui.painter().text(
        egui::pos2(gate_x, rect.top() + 12.0),
        egui::Align2::LEFT_CENTER,
        elide(ui, summary.gate.label(), gate_w.max(1.0), true),
        theme::mono(tokens::FS_0, FontWeight::SemiBold),
        gate_color(summary.gate, &t),
    );
    let detail = gate_detail(summary);
    ui.painter().text(
        egui::pos2(gate_x, rect.bottom() - 9.0),
        egui::Align2::LEFT_CENTER,
        elide(ui, &detail, gate_w.max(1.0), false),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );

    // Painted glyphs publish no accessibility node, so the row states its
    // whole line — every domain included. A reader who cannot see the matrix
    // is looking for exactly the coverage the cells carry.
    let announced = row_announcement(summary);
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            announced.clone(),
        )
    });
    theme::paint_focus_ring(ui, &response, rect);
    response
}

/// One family's retained coverage in one domain, and the tone that says how it
/// stands.
///
/// Four states, because four things can be true: the family declares no vector
/// here at all, evidence says a vector failed, vectors exist that no evidence
/// describes, or every vector passed. The middle two are separated because
/// they are different work — a failure is repaired, a gap is run.
fn domain_cell(
    summary: &QualificationModelSummary,
    domain: QualificationDomain,
    t: &Tokens,
) -> (String, Color32) {
    let Some(retained) = summary.domain(domain) else {
        return ("—".to_owned(), t.color.text_faint);
    };
    let color = if retained.open_dispositions > 0
        || retained.passing_vectors < retained.evidenced_vectors
    {
        t.color.err
    } else if retained.evidenced_vectors < retained.vectors {
        t.color.warn
    } else {
        t.color.text
    };
    (
        format!("{} / {}", retained.passing_vectors, retained.vectors),
        color,
    )
}

/// What stands between this family and a clean gate, in one phrase.
fn gate_detail(summary: &QualificationModelSummary) -> String {
    if !summary.gate.is_gate_subject() {
        return "compiled in".to_owned();
    }
    if summary.source_error.is_some() {
        return "source not resolved".to_owned();
    }
    if summary.open_dispositions > 0 {
        return format!(
            "{} open disposition{}",
            summary.open_dispositions,
            if summary.open_dispositions == 1 {
                ""
            } else {
                "s"
            }
        );
    }
    if summary.suites == 0 {
        return "no retained suite".to_owned();
    }
    let pending = summary.vectors.saturating_sub(summary.passing_vectors);
    if pending > 0 {
        return format!("{pending} vectors pending");
    }
    if summary.parity_suites < summary.suites {
        return format!(
            "{} of {} suites at parity",
            summary.parity_suites, summary.suites
        );
    }
    format!(
        "{} suite{} · {} vectors",
        summary.suites,
        if summary.suites == 1 { "" } else { "s" },
        summary.vectors
    )
}

fn row_announcement(summary: &QualificationModelSummary) -> String {
    let mut announced = format!("{} in {}", summary.model, summary.library);
    for domain in QualificationDomain::ALL {
        announced.push_str(", ");
        announced.push_str(domain.label());
        match summary.domain(domain) {
            Some(retained) => announced.push_str(&format!(
                " {} of {} passing",
                retained.passing_vectors, retained.vectors
            )),
            None => announced.push_str(" not declared"),
        }
    }
    announced.push_str(&format!(
        ", {} references, {}, {}",
        summary.references,
        summary.gate.label(),
        gate_detail(summary)
    ));
    announced
}

/// The selected family, and the two workflows that open its evidence.
///
/// Both controls stay drawn when they are unavailable and state the reason on
/// hover, which is this page's rule everywhere else: "no button" and "not
/// qualified" are different facts to an engineer looking for why.
fn selection_footer(
    ui: &mut Ui,
    review_blocker: Option<&str>,
    selected: &QualificationModelSummary,
    requested_action: &mut Option<QualificationPageAction>,
) {
    ui.separator();
    egui::Frame::NONE
        .inner_margin(egui::Margin::symmetric(12, 6))
        .show(ui, |ui| {
            property(ui, "Selected", &selected.model, &selected.library);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                let review = Button::new("Review qualification")
                    .enabled(review_blocker.is_none())
                    .show(ui);
                if let Some(reason) = review_blocker {
                    review.on_disabled_hover_text(reason);
                } else if review.clicked() {
                    *requested_action = Some(QualificationPageAction::ReviewVectors);
                }
                if Button::new("Measurement correlation").show(ui).clicked() {
                    *requested_action = Some(QualificationPageAction::OpenCorrelation);
                }
            });
        });
}

/// The three contract cards, filling the track they were handed.
fn contract_rail(
    ui: &mut Ui,
    app: &RSpiceApp,
    selected: Option<&QualificationModelSummary>,
    requested_action: &mut Option<QualificationPageAction>,
    height: f32,
) {
    let Some(selected) = selected else {
        page_empty_state(
            ui,
            "Select a qualification suite",
            "Choose a model family to inspect its dispositions, tolerance policy, and retained evidence.",
        );
        return;
    };
    let card_h = (height / 3.0).max(DETAIL_PANE_MIN_H);
    if card_h * 3.0 <= height + 1.0 {
        dispositions_pane(ui, app, selected, requested_action, card_h);
        tolerance_pane(ui, selected, card_h);
        contract_pane(ui, selected, card_h);
    } else {
        // Three cards at their minimum are taller than a short viewport, so the
        // rail scrolls rather than pushing the last card off the panel.
        ScrollArea::vertical()
            .id_salt("models-qualification-contract-rail")
            .auto_shrink([false, false])
            .max_height(height)
            .show(ui, |ui| {
                dispositions_pane(ui, app, selected, requested_action, card_h);
                tolerance_pane(ui, selected, card_h);
                contract_pane(ui, selected, card_h);
            });
    }
}

// "not configured" and "fail closed" are findings against a model that was
// supposed to declare something. An engine-owned card was never supposed to, so
// each pane below states the exemption rather than reporting the absence it was
// designed to have.

fn dispositions_pane(
    ui: &mut Ui,
    app: &RSpiceApp,
    selected: &QualificationModelSummary,
    requested_action: &mut Option<QualificationPageAction>,
    height: f32,
) {
    filled_detail_pane(
        ui,
        "OPEN DISPOSITIONS",
        Some(&format!("{} pending", selected.open_dispositions)),
        height,
        "models-qualification-dispositions",
        |ui| {
            if !selected.gate.is_gate_subject() {
                property(ui, "Domains", "not applicable", "engine-owned");
                return;
            }
            if selected.domains.is_empty() {
                property(ui, "Domains", "not configured", "suite contract");
                return;
            }
            for domain in &selected.domains {
                property(
                    ui,
                    domain.domain.label(),
                    &domain.disposition,
                    &domain.reference_coverage,
                );
            }
            let blocker = qualification_action_block_reason(
                app,
                Some(selected),
                QualificationPageAction::ReviewVectors,
            );
            let review = Button::new("Review dispositions")
                .enabled(blocker.is_none())
                .show(ui);
            if let Some(reason) = blocker.as_deref() {
                review.on_disabled_hover_text(reason);
            } else if review.clicked() {
                *requested_action = Some(QualificationPageAction::ReviewVectors);
            }
        },
    );
}

fn tolerance_pane(ui: &mut Ui, selected: &QualificationModelSummary, height: f32) {
    filled_detail_pane(
        ui,
        "TOLERANCE POLICY",
        Some("domain-owned contracts"),
        height,
        "models-qualification-tolerance",
        |ui| {
            if !selected.gate.is_gate_subject() {
                property(ui, "Policy", "not applicable", "engine-owned");
                return;
            }
            if selected.domains.is_empty() {
                property(ui, "Policy", "not declared", "fail closed");
                return;
            }
            for domain in &selected.domains {
                property(
                    ui,
                    domain.domain.label(),
                    &domain.tolerance,
                    &format!("{} vectors", domain.vectors),
                );
            }
        },
    );
}

fn contract_pane(ui: &mut Ui, selected: &QualificationModelSummary, height: f32) {
    filled_detail_pane(
        ui,
        "QUALIFICATION CONTRACT",
        Some(selected.gate.label()),
        height,
        "models-qualification-contract",
        |ui| {
            if !selected.gate.is_gate_subject() {
                // Every row below reads a retained artefact that an
                // engine-owned card has none of. Four "not retained" lines are
                // four findings; the exemption is one fact.
                property(
                    ui,
                    "Model revision",
                    "engine-owned",
                    "compiled into the simulator",
                );
                property(ui, "Release gate", "exempt", "not source-owned by design");
                property(
                    ui,
                    "Qualification",
                    "engine equation defaults",
                    "author a project copy to gate one",
                );
                return;
            }
            property(
                ui,
                "Model revision",
                &selected.source_revision,
                "exact source",
            );
            property(
                ui,
                "Runtime parity",
                &format!(
                    "desktop {}/{} · WASM {}/{}",
                    selected.desktop_passing,
                    selected.vectors,
                    selected.wasm_passing,
                    selected.vectors
                ),
                &format!("{} suites", selected.parity_suites),
            );
            property(
                ui,
                "Evidence set",
                selected
                    .evidence_digest
                    .as_deref()
                    .unwrap_or("not retained"),
                &format!("{} references", selected.references),
            );
            property(
                ui,
                "Correlation",
                &selected.correlation_status,
                selected
                    .correlation_evidence_digest
                    .as_deref()
                    .unwrap_or("not retained"),
            );
            property(
                ui,
                "Approved releases",
                &selected.releases.to_string(),
                "source-owned",
            );
        },
    );
}

fn gate_color(gate: QualificationGate, t: &Tokens) -> Color32 {
    match gate {
        QualificationGate::Qualified => t.color.ok,
        QualificationGate::Review | QualificationGate::Unqualified => t.color.warn,
        QualificationGate::Blocked => t.color.err,
        // Neither a pass nor a finding: an engine-owned card is outside the
        // gate, so it takes the page's plain text tone rather than borrowing
        // either verdict's colour.
        QualificationGate::EngineOwned => t.color.text_dim,
    }
}

#[cfg(test)]
mod tests;
