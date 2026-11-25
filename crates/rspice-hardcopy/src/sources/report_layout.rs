//! How a report sets on paper: its blocks flowed down pages of the size the
//! report is published on, continuing onto the next page wherever one fills,
//! with prose broken where its measured width runs out.
//!
//! The source stage lays a report out to learn how many pages it fills,
//! which is the extent the print plan is sealed against; the scene compiler
//! lays it out again, identically, and draws what this placed.

use super::*;
use rspice_fonts::Role;
use rspice_results::report_document::{
    FigureSizing, PlotFigureBlock, ReportBlock, ReportPublicationPageSize, ReportSourceId,
    RequirementDisposition, ReviewNoteStatus, SpecificationDisposition, TableCell,
};

/// The type scale, in micrometres of em: each heading a step above the next
/// and the smallest of them above the body.
pub(crate) const TITLE_UM: u64 = 6_350; // 18 pt
pub(crate) const SECTION_UM: u64 = 4_939; // 14 pt
pub(crate) const HEADING_UM: [u64; 4] = [4_586, 4_233, 3_881, 3_704]; // 13, 12, 11, 10.5 pt
pub(crate) const BODY_UM: u64 = 3_528; // 10 pt
pub(crate) const TABLE_UM: u64 = 3_175; // 9 pt
pub(crate) const SMALL_UM: u64 = 2_822; // 8 pt
/// The page margin on every side.
const MARGIN_UM: u64 = 20_000;
/// The gap after a paragraph, and after a block.
const PARAGRAPH_GAP_UM: u64 = 2_100;
const BLOCK_GAP_UM: u64 = 4_200;
/// A table cell's padding: across, and above and below its text.
const CELL_PAD_X_UM: u64 = 1_500;
const CELL_PAD_Y_UM: u64 = 900;
/// The indent of a list item's text, a quotation and a code line.
const INDENT_UM: u64 = 5_000;
/// A figure's pixels print at 96 to the inch at natural size.
const FIGURE_PIXELS_PER_INCH: u64 = 96;
const MICROMETRES_PER_INCH: u64 = 25_400;

/// One stretch of report text set in one face.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Run {
    pub text: String,
    pub role: Role,
}

impl Run {
    fn new(text: impl Into<String>, role: Role) -> Self {
        Self {
            text: text.into(),
            role,
        }
    }
}

/// What a report page holds, placed from the page's top-left corner.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Laid {
    /// Text set from `x` on the baseline `y`.
    Text {
        x: u64,
        y: u64,
        size: u64,
        role: Role,
        text: String,
    },
    /// A horizontal rule of a table.
    Rule {
        x: u64,
        y: u64,
        width: u64,
        heavy: bool,
    },
    /// The figure at `figure` in the report's figures.
    Figure {
        figure: usize,
        x: u64,
        y: u64,
        width: u64,
        height: u64,
    },
}

#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct LaidPage {
    pub items: Vec<Laid>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ReportLayout {
    pub width: u64,
    pub height: u64,
    pub pages: Vec<LaidPage>,
}

/// A page of the size a report is published on, in micrometres.
pub(crate) const fn page_size(size: ReportPublicationPageSize) -> (u64, u64) {
    match size {
        ReportPublicationPageSize::A4Portrait => (210_000, 297_000),
        ReportPublicationPageSize::UsLetterPortrait => (215_900, 279_400),
        ReportPublicationPageSize::A3Landscape => (420_000, 297_000),
    }
}

/// Lay `report` out on its pages.
pub(crate) fn lay_out(report: &SemanticReport) -> ReportLayout {
    let (width, height) = page_size(report.page_size);
    let mut flow = Flow {
        width,
        height,
        pages: Vec::new(),
        y: MARGIN_UM,
        fresh: true,
        title: String::new(),
        figures: 0,
    };
    for page in &report.pages {
        flow.title = page.title().to_owned();
        flow.start_page();
        flow.heading(page.title(), TITLE_UM, 0);
        for section in page.sections() {
            flow.heading(section.title(), SECTION_UM, 3_000);
            for block in section.blocks() {
                flow.block(block, report);
                flow.space(BLOCK_GAP_UM);
            }
        }
    }
    flow.number_pages();
    ReportLayout {
        width,
        height,
        pages: flow.pages,
    }
}

/// How wide `text` sets in `role` at `size_um`, in micrometres.
pub(crate) fn set_width(role: Role, text: &str, size_um: u64) -> u64 {
    (rspice_fonts::print::width_em(role, text) * size_um as f64).ceil() as u64
}

/// The distance from one line's baseline to the next.
const fn leading(size_um: u64) -> u64 {
    size_um * 135 / 100
}

/// One report table cell as the page prints it. A number reads as a sheet's
/// column reads it, at the default significant digits with every digit kept
/// and an SI prefix on its unit: 1.500 fF, never "0.000000000000 F"; a
/// decibel or an angle in degrees to the hundredth. A report leaves the
/// screen, so it does not follow one reader's digits preference.
pub(crate) fn format_table_cell(cell: &TableCell) -> String {
    match cell {
        TableCell::Empty => String::new(),
        TableCell::Text(value) => value.clone(),
        TableCell::Number { value, unit } => rspice_results::number_format::fmt_column_reading(
            *value,
            unit.as_deref().unwrap_or_default(),
            usize::from(rspice_results::display_preferences::DisplayedSignificantDigits::DEFAULT),
        ),
        TableCell::Integer(value) => value.to_string(),
        TableCell::Boolean(value) => if *value { "Yes" } else { "No" }.to_owned(),
    }
}

/// A requirement's verdict as a reader says it.
const fn requirement_word(disposition: RequirementDisposition) -> &'static str {
    match disposition {
        RequirementDisposition::NotEvaluated => "Not evaluated",
        RequirementDisposition::Passed => "Pass",
        RequirementDisposition::Failed => "Fail",
        RequirementDisposition::Waived => "Waived",
    }
}

/// A specification's verdict as a reader says it.
const fn specification_word(disposition: SpecificationDisposition) -> &'static str {
    match disposition {
        SpecificationDisposition::NotEvaluated => "Not evaluated",
        SpecificationDisposition::InSpecification => "Pass",
        SpecificationDisposition::OutOfSpecification => "Fail",
        SpecificationDisposition::Informational => "Informational",
    }
}

const fn review_word(status: ReviewNoteStatus) -> &'static str {
    match status {
        ReviewNoteStatus::Open => "Open",
        ReviewNoteStatus::Addressed => "Addressed",
        ReviewNoteStatus::Accepted => "Accepted",
    }
}

/// The runs a line of report Markdown sets as. `**bold**` and `` `code` ``
/// are set in their own faces, only where they are marked; a link prints as
/// its text with its address after it, and an image as its alternative text.
/// A mark that is never closed is the character it is, so ordinary prose
/// such as `|S21|`, "< 1 dB" or a lone asterisk prints as written.
pub(crate) fn inline_runs(text: &str, base: Role) -> Vec<Run> {
    let mut runs = Vec::<Run>::new();
    let mut current = String::new();
    let (mut bold, mut code) = (false, false);
    let role = |bold: bool, code: bool| {
        if code {
            Role::Mono
        } else if bold {
            Role::SansSemibold
        } else {
            base
        }
    };
    let flush = |runs: &mut Vec<Run>, current: &mut String, role: Role| {
        if !current.is_empty() {
            push_run(runs, current, role);
            current.clear();
        }
    };
    let mut rest = text;
    while let Some(character) = rest.chars().next() {
        let after = &rest[character.len_utf8()..];
        if !code && rest.starts_with("**") && (bold || after[1..].contains("**")) {
            flush(&mut runs, &mut current, role(bold, code));
            bold = !bold;
            rest = &rest[2..];
            continue;
        }
        if character == '`' && (code || after.contains('`')) {
            flush(&mut runs, &mut current, role(bold, code));
            code = !code;
            rest = after;
            continue;
        }
        if !code
            && (character == '[' || rest.starts_with("!["))
            && let Some((label, address, remainder)) = link(rest)
        {
            current.push_str(label);
            if character == '[' && !address.is_empty() && address != label {
                current.push_str(" (");
                current.push_str(address);
                current.push(')');
            }
            rest = remainder;
            continue;
        }
        current.push(character);
        rest = after;
    }
    flush(&mut runs, &mut current, role(bold, code));
    runs
}

fn push_run(runs: &mut Vec<Run>, text: &str, role: Role) {
    match runs.last_mut() {
        Some(last) if last.role == role => last.text.push_str(text),
        _ => runs.push(Run::new(text, role)),
    }
}

/// `[label](address)` or `![label](address)` at the start of `text`: its
/// label, its address and what follows it.
fn link(text: &str) -> Option<(&str, &str, &str)> {
    let open = text.find('[')?;
    let close = open + text[open..].find("](")?;
    if text[open + 1..close].contains('[') {
        return None;
    }
    let end = close + 2 + text[close + 2..].find(')')?;
    Some((
        &text[open + 1..close],
        &text[close + 2..end],
        &text[end + 1..],
    ))
}

/// A word, which may change face part way, and whether a space came before it.
struct Word {
    pieces: Vec<Run>,
    spaced: bool,
}

impl Word {
    fn width(&self, size_um: u64) -> u64 {
        self.pieces
            .iter()
            .map(|piece| set_width(piece.role, &piece.text, size_um))
            .sum()
    }
}

/// The words of `runs`, each knowing whether a space comes before it.
fn words(runs: &[Run]) -> Vec<Word> {
    let mut words = Vec::<Word>::new();
    let mut word: Option<Word> = None;
    let mut spaced = false;
    for run in runs {
        for character in run.text.chars() {
            if character.is_whitespace() {
                if let Some(done) = word.take() {
                    words.push(done);
                }
                spaced = true;
                continue;
            }
            let current = word.get_or_insert_with(|| Word {
                pieces: Vec::new(),
                spaced: std::mem::take(&mut spaced),
            });
            let mut buffer = [0_u8; 4];
            push_run(
                &mut current.pieces,
                character.encode_utf8(&mut buffer),
                run.role,
            );
        }
    }
    words.extend(word);
    words
}

/// How wide `runs` set on one line at `size_um`, measured as `wrap_runs`
/// measures a line, so that a column this wide holds them unbroken.
fn unbroken_width(runs: &[Run], size_um: u64) -> u64 {
    words(runs)
        .iter()
        .enumerate()
        .map(|(index, word)| {
            let space = if index > 0 && word.spaced {
                set_width(word.pieces[0].role, " ", size_um)
            } else {
                0
            };
            space + word.width(size_um)
        })
        .sum()
}

/// `runs` broken into lines no wider than `width_um` when set at `size_um`,
/// between words; a word wider than a whole line is broken where it fills
/// one.
pub(crate) fn wrap_runs(runs: &[Run], size_um: u64, width_um: u64) -> Vec<Vec<Run>> {
    let words = words(runs);
    let mut lines = Vec::<Vec<Run>>::new();
    let mut line = Vec::<Run>::new();
    let mut used = 0_u64;
    for word in words {
        let width = word.width(size_um);
        let space_role = word.pieces[0].role;
        let space = if word.spaced && !line.is_empty() {
            set_width(space_role, " ", size_um)
        } else {
            0
        };
        if !line.is_empty() && used + space + width > width_um {
            lines.push(std::mem::take(&mut line));
            used = 0;
        }
        if line.is_empty() && width > width_um {
            // Too wide for any line: break it where each line fills.
            for piece in word.pieces {
                for character in piece.text.chars() {
                    let mut buffer = [0_u8; 4];
                    let character = character.encode_utf8(&mut buffer);
                    let advance = set_width(piece.role, character, size_um);
                    if !line.is_empty() && used + advance > width_um {
                        lines.push(std::mem::take(&mut line));
                        used = 0;
                    }
                    push_run(&mut line, character, piece.role);
                    used += advance;
                }
            }
            continue;
        }
        if !line.is_empty() && word.spaced {
            push_run(&mut line, " ", space_role);
            used += space;
        }
        for piece in &word.pieces {
            push_run(&mut line, &piece.text, piece.role);
        }
        used += width;
    }
    if !line.is_empty() || lines.is_empty() {
        lines.push(line);
    }
    lines
}

fn plain(runs: &[Run]) -> String {
    runs.iter().map(|run| run.text.as_str()).collect()
}

/// A table as the page sets it.
struct Table {
    title: String,
    header: Vec<String>,
    rows: Vec<Vec<String>>,
    /// Columns whose cells align on their right edge, as numbers do.
    right: Vec<bool>,
}

/// Where the text of a block is going, page by page.
struct Flow {
    width: u64,
    height: u64,
    pages: Vec<LaidPage>,
    /// The top of the next line.
    y: u64,
    /// Nothing has been set on this page past its running head.
    fresh: bool,
    /// The authored page the flow is on, for the head of a page it runs on to.
    title: String,
    figures: usize,
}

impl Flow {
    const fn measure(&self) -> u64 {
        self.width - 2 * MARGIN_UM
    }

    const fn bottom(&self) -> u64 {
        self.height - MARGIN_UM
    }

    /// The height a page holds below its running head.
    const fn page_room(&self) -> u64 {
        self.bottom() - MARGIN_UM - leading(SMALL_UM) - SMALL_UM
    }

    fn items(&mut self) -> &mut Vec<Laid> {
        if self.pages.is_empty() {
            self.pages.push(LaidPage::default());
        }
        &mut self.pages.last_mut().expect("a page was just made").items
    }

    fn start_page(&mut self) {
        self.pages.push(LaidPage::default());
        self.y = MARGIN_UM;
        self.fresh = true;
    }

    /// A page the authored page runs on to, headed with its title.
    fn continue_page(&mut self) {
        self.start_page();
        let head = format!("{} (continued)", self.title);
        self.put_line(MARGIN_UM, self.y, &[Run::new(head, Role::Sans)], SMALL_UM);
        self.y += leading(SMALL_UM) + SMALL_UM;
        self.fresh = true;
    }

    /// Go on to the next page unless `needed` still fits on this one; a page
    /// with nothing on it takes whatever comes.
    fn room(&mut self, needed: u64) {
        if self.y + needed > self.bottom() && !self.fresh {
            self.continue_page();
        }
    }

    /// Leave `gap` before what comes next, except at the top of a page.
    fn space(&mut self, gap: u64) {
        if !self.fresh {
            self.y += gap;
        }
    }

    /// Set `line` from `x` in the line whose top is `top`, each run where
    /// the one before it ended and its spaces as the room they take.
    fn put_line(&mut self, x: u64, top: u64, line: &[Run], size: u64) {
        let baseline = top + size * 4 / 5;
        let mut x = x;
        for run in line {
            let advance = set_width(run.role, &run.text, size);
            let words = run.text.trim_start();
            let lead = &run.text[..run.text.len() - words.len()];
            if !words.trim_end().is_empty() {
                let start = x + set_width(run.role, lead, size);
                self.items().push(Laid::Text {
                    x: start,
                    y: baseline,
                    size,
                    role: run.role,
                    text: words.trim_end().to_owned(),
                });
            }
            x += advance;
        }
    }

    /// Set `runs` from `x`, `width` wide, breaking lines and pages.
    fn lines(&mut self, x: u64, width: u64, runs: &[Run], size: u64) {
        for line in wrap_runs(runs, size, width) {
            self.room(leading(size));
            self.put_line(x, self.y, &line, size);
            self.y += leading(size);
            self.fresh = false;
        }
    }

    /// A heading, kept on a page with the two lines after it.
    fn heading(&mut self, text: &str, size: u64, before: u64) {
        if text.trim().is_empty() {
            return;
        }
        self.room(before + leading(size) + 2 * leading(BODY_UM));
        self.space(before);
        let runs = inline_runs(text, Role::SansSemibold);
        self.lines(MARGIN_UM, self.measure(), &runs, size);
        self.y += size / 3;
    }

    fn paragraph(&mut self, text: &str, indent: u64) {
        if text.trim().is_empty() {
            return;
        }
        let runs = inline_runs(text, Role::Sans);
        self.lines(MARGIN_UM + indent, self.measure() - indent, &runs, BODY_UM);
        self.y += PARAGRAPH_GAP_UM;
    }

    fn block(&mut self, block: &ReportBlock, report: &SemanticReport) {
        match block.kind() {
            ReportBlockKind::PlotFigure(value) => {
                if let Some(index) = report
                    .figures
                    .iter()
                    .position(|figure| figure.block_id == block.id())
                {
                    self.figure(index, &report.figures[index], value);
                }
            }
            ReportBlockKind::DataTable(value) => {
                let right = (0..value.columns.len())
                    .map(|column| {
                        value.rows.iter().all(|row| {
                            matches!(
                                row.get(column),
                                None | Some(
                                    TableCell::Empty
                                        | TableCell::Number { .. }
                                        | TableCell::Integer(_)
                                )
                            )
                        })
                    })
                    .collect();
                self.table(Table {
                    title: value.title.clone(),
                    header: value
                        .columns
                        .iter()
                        .map(|column| column.heading.clone())
                        .collect(),
                    rows: value
                        .rows
                        .iter()
                        .map(|row| row.iter().map(format_table_cell).collect())
                        .collect(),
                    right,
                });
            }
            ReportBlockKind::Datasheet(value) => self.table(Table {
                title: value.title.clone(),
                header: vec!["Parameter".to_owned(), "Value".to_owned()],
                rows: value
                    .fields
                    .iter()
                    .map(|field| {
                        let value = match field.unit.as_deref() {
                            Some(unit) if !unit.is_empty() => format!("{} {unit}", field.value),
                            _ => field.value.clone(),
                        };
                        vec![field.label.clone(), value]
                    })
                    .collect(),
                right: vec![false, true],
            }),
            ReportBlockKind::Requirements(value) => self.table(Table {
                title: value.title.clone(),
                header: ["Requirement", "Statement", "Result", "Evidence"]
                    .map(str::to_owned)
                    .to_vec(),
                rows: value
                    .entries
                    .iter()
                    .map(|entry| {
                        vec![
                            entry.requirement_id.clone(),
                            entry.statement.clone(),
                            requirement_word(entry.disposition).to_owned(),
                            entry.evidence_label.clone().unwrap_or_default(),
                        ]
                    })
                    .collect(),
                right: vec![false; 4],
            }),
            ReportBlockKind::Specifications(value) => self.table(Table {
                title: value.title.clone(),
                header: ["Expression", "Limit", "Measured", "Result"]
                    .map(str::to_owned)
                    .to_vec(),
                rows: value
                    .entries
                    .iter()
                    .map(|entry| {
                        vec![
                            entry.expression.clone(),
                            entry.limit.clone(),
                            entry
                                .measured
                                .clone()
                                .unwrap_or_else(|| "\u{2014}".to_owned()),
                            specification_word(entry.disposition).to_owned(),
                        ]
                    })
                    .collect(),
                right: vec![false, true, true, false],
            }),
            ReportBlockKind::Prose(value) => self.markdown(&value.markdown),
            ReportBlockKind::ReviewNote(value) => {
                let head = format!("Review \u{b7} {} \u{b7} ", value.author);
                self.room(leading(TABLE_UM) + leading(BODY_UM));
                self.lines(
                    MARGIN_UM,
                    self.measure(),
                    &[
                        Run::new(head, Role::Sans),
                        Run::new(review_word(value.status), Role::SansSemibold),
                    ],
                    TABLE_UM,
                );
                for paragraph in value.message.split("\n\n") {
                    let text = paragraph.split_whitespace().collect::<Vec<_>>().join(" ");
                    if !text.is_empty() {
                        self.lines(
                            MARGIN_UM,
                            self.measure(),
                            &[Run::new(text, Role::Sans)],
                            BODY_UM,
                        );
                        self.y += PARAGRAPH_GAP_UM;
                    }
                }
            }
            ReportBlockKind::Evidence(value) => {
                self.room(leading(BODY_UM) * 2 + leading(SMALL_UM));
                self.lines(
                    MARGIN_UM,
                    self.measure(),
                    &[Run::new(value.title.clone(), Role::SansSemibold)],
                    BODY_UM,
                );
                self.paragraph(&value.summary, 0);
                let source = match &value.reference.snapshot().source {
                    ReportSourceId::VisualizationDocument { .. } => "result view".to_owned(),
                    ReportSourceId::Dataset { .. } => "dataset".to_owned(),
                    ReportSourceId::VerificationEvidence { .. } => {
                        "verification evidence".to_owned()
                    }
                    ReportSourceId::ExternalRecord { namespace, key } => {
                        format!("{namespace} record {key}")
                    }
                };
                let kept = if value.reference.is_frozen() {
                    "frozen into this report"
                } else {
                    "linked to its current revision"
                };
                self.lines(
                    MARGIN_UM,
                    self.measure(),
                    &[Run::new(format!("Source: {source}, {kept}."), Role::Sans)],
                    SMALL_UM,
                );
            }
        }
    }

    /// A figure at the size its block asks for, its caption under it. "Fit
    /// page" gives it a page of its own and fills it.
    fn figure(&mut self, index: usize, figure: &SemanticReportFigure, value: &PlotFigureBlock) {
        self.figures += 1;
        let caption = vec![
            Run::new(format!("Figure {}. ", self.figures), Role::SansSemibold),
            Run::new(value.caption.clone(), Role::Sans),
        ];
        let caption_lines = wrap_runs(&caption, TABLE_UM, self.measure());
        let caption_height = 1_500 + caption_lines.len() as u64 * leading(TABLE_UM);
        let natural_width = (u64::from(figure.width_pixels) * MICROMETRES_PER_INCH)
            .div_ceil(FIGURE_PIXELS_PER_INCH);
        let natural_height = (u64::from(figure.height_pixels) * MICROMETRES_PER_INCH)
            .div_ceil(FIGURE_PIXELS_PER_INCH);
        if natural_width == 0 || natural_height == 0 {
            return;
        }
        if value.sizing == FigureSizing::FitPage && !self.fresh {
            self.continue_page();
        }
        let tall = match value.sizing {
            FigureSizing::FitPage => self.bottom().saturating_sub(self.y),
            FigureSizing::Natural | FigureSizing::FitWidth => self.page_room(),
        }
        .saturating_sub(caption_height);
        let wide = self.measure();
        let fill = (wide as f64 / natural_width as f64).min(tall as f64 / natural_height as f64);
        let scale = match value.sizing {
            FigureSizing::Natural => fill.min(1.0),
            FigureSizing::FitWidth | FigureSizing::FitPage => fill,
        };
        let width = (natural_width as f64 * scale).floor() as u64;
        let height = (natural_height as f64 * scale).floor() as u64;
        self.room(height + caption_height);
        let x = MARGIN_UM + (wide - width.min(wide)) / 2;
        let y = self.y;
        self.items().push(Laid::Figure {
            figure: index,
            x,
            y,
            width,
            height,
        });
        self.fresh = false;
        self.y += height + 1_500;
        for line in caption_lines {
            let set: u64 = line
                .iter()
                .map(|run| set_width(run.role, &run.text, TABLE_UM))
                .sum();
            self.put_line(
                MARGIN_UM + (wide - set.min(wide)) / 2,
                self.y,
                &line,
                TABLE_UM,
            );
            self.y += leading(TABLE_UM);
        }
    }

    /// A table whose columns are as wide as their contents, narrowed where
    /// the page is not wide enough by wrapping their text, and whose header
    /// repeats on each page it runs on to.
    fn table(&mut self, table: Table) {
        let columns = table.header.len();
        if columns == 0 {
            return;
        }
        let cell_runs = |text: &str, role: Role| vec![Run::new(text.to_owned(), role)];
        let rows = table
            .rows
            .iter()
            .map(|row| {
                (0..columns)
                    .map(|column| row.get(column).map(String::as_str).unwrap_or_default())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        // What each column would take unbroken, and the least it can take
        // without breaking a word.
        let mut natural = vec![0_u64; columns];
        let mut least = vec![0_u64; columns];
        let cells = std::iter::once((
            table.header.iter().map(String::as_str).collect::<Vec<_>>(),
            Role::SansSemibold,
        ))
        .chain(rows.iter().map(|row| (row.clone(), Role::Sans)));
        for (row, role) in cells {
            for (column, text) in row.iter().enumerate() {
                let runs = cell_runs(text, role);
                natural[column] = natural[column].max(unbroken_width(&runs, TABLE_UM));
                let longest = words(&runs)
                    .iter()
                    .map(|word| word.width(TABLE_UM))
                    .max()
                    .unwrap_or(0);
                least[column] = least[column].max(longest);
            }
        }
        let widths = column_widths(&natural, &least, self.measure(), 2 * CELL_PAD_X_UM);
        let header = table
            .header
            .iter()
            .zip(&widths)
            .map(|(text, width)| {
                wrap_runs(
                    &cell_runs(text, Role::SansSemibold),
                    TABLE_UM,
                    width - 2 * CELL_PAD_X_UM,
                )
            })
            .collect::<Vec<_>>();
        let header_height = row_height(&header);
        let body = rows
            .iter()
            .map(|row| {
                row.iter()
                    .zip(&widths)
                    .map(|(text, width)| {
                        wrap_runs(
                            &cell_runs(text, Role::Sans),
                            TABLE_UM,
                            width - 2 * CELL_PAD_X_UM,
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let first = body.first().map_or(0, |row| row_height(row));
        self.room(leading(BODY_UM) + header_height + first);
        if !table.title.trim().is_empty() {
            self.lines(
                MARGIN_UM,
                self.measure(),
                &[Run::new(table.title.clone(), Role::SansSemibold)],
                BODY_UM,
            );
            self.y += 600;
        }
        let total: u64 = widths.iter().sum();
        self.table_row(&header, &widths, &table.right, true);
        for row in &body {
            let height = row_height(row);
            if self.y + height > self.bottom() {
                self.rule(total, true);
                self.continue_page();
                self.table_row(&header, &widths, &table.right, true);
            }
            self.table_row(row, &widths, &table.right, false);
        }
        self.rule(total, true);
    }

    fn rule(&mut self, width: u64, heavy: bool) {
        let y = self.y;
        self.items().push(Laid::Rule {
            x: MARGIN_UM,
            y,
            width,
            heavy,
        });
    }

    /// One row of a table; a header row is ruled above and below.
    fn table_row(&mut self, cells: &[Vec<Vec<Run>>], widths: &[u64], right: &[bool], header: bool) {
        let total: u64 = widths.iter().sum();
        if header {
            self.rule(total, true);
        }
        let top = self.y + CELL_PAD_Y_UM;
        let mut x = MARGIN_UM;
        for (column, (lines, width)) in cells.iter().zip(widths).enumerate() {
            let mut line_top = top;
            for line in lines {
                let set: u64 = line
                    .iter()
                    .map(|run| set_width(run.role, &run.text, TABLE_UM))
                    .sum();
                let start = if right.get(column).copied().unwrap_or(false) {
                    x + width - CELL_PAD_X_UM - set.min(width - CELL_PAD_X_UM)
                } else {
                    x + CELL_PAD_X_UM
                };
                self.put_line(start, line_top, line, TABLE_UM);
                line_top += leading(TABLE_UM);
            }
            x += width;
        }
        self.y += row_height(cells);
        self.fresh = false;
        if header {
            self.rule(total, false);
        }
    }

    /// Report Markdown, set block by block. Nothing in it is refused: a
    /// construct print has no form for prints as its text.
    fn markdown(&mut self, markdown: &str) {
        let mut paragraph = String::new();
        let mut quote = String::new();
        let mut code = false;
        let mut lines = markdown.lines().peekable();
        while let Some(raw) = lines.next() {
            let line = raw.trim_end();
            let trimmed = line.trim();
            let is_quote = trimmed.starts_with('>') && !code;
            if !is_quote && !quote.is_empty() {
                self.paragraph(&std::mem::take(&mut quote), INDENT_UM);
            }
            if trimmed.starts_with("```") {
                self.paragraph(&std::mem::take(&mut paragraph), 0);
                code = !code;
                continue;
            }
            if code {
                self.code_line(line);
                continue;
            }
            if trimmed.is_empty() {
                self.paragraph(&std::mem::take(&mut paragraph), 0);
                continue;
            }
            if is_quote {
                self.paragraph(&std::mem::take(&mut paragraph), 0);
                if !quote.is_empty() {
                    quote.push(' ');
                }
                quote.push_str(trimmed.trim_start_matches('>').trim());
                continue;
            }
            if is_table_row(trimmed) {
                self.paragraph(&std::mem::take(&mut paragraph), 0);
                let mut rows = vec![table_cells(trimmed)];
                while let Some(next) = lines.next_if(|next| is_table_row(next.trim())) {
                    rows.push(table_cells(next.trim()));
                }
                self.markdown_table(rows);
                continue;
            }
            if let Some((size, text)) = heading_level(trimmed) {
                self.paragraph(&std::mem::take(&mut paragraph), 0);
                self.heading(text, size, 2_400);
                continue;
            }
            if let Some((depth, marker, text)) = list_item(line) {
                self.paragraph(&std::mem::take(&mut paragraph), 0);
                self.list_item(depth, &marker, text);
                continue;
            }
            if !paragraph.is_empty() {
                paragraph.push(' ');
            }
            paragraph.push_str(trimmed);
        }
        self.paragraph(&quote, INDENT_UM);
        self.paragraph(&paragraph, 0);
        if code {
            self.y += PARAGRAPH_GAP_UM;
        }
    }

    fn code_line(&mut self, line: &str) {
        let x = MARGIN_UM + INDENT_UM;
        let width = self.measure() - INDENT_UM;
        let mut piece = String::new();
        let mut pieces = Vec::new();
        for character in line.chars() {
            piece.push(character);
            if set_width(Role::Mono, &piece, TABLE_UM) > width && piece.chars().count() > 1 {
                piece.pop();
                pieces.push(std::mem::replace(&mut piece, character.to_string()));
            }
        }
        pieces.push(piece);
        for piece in pieces {
            self.room(leading(TABLE_UM));
            self.put_line(x, self.y, &[Run::new(piece, Role::Mono)], TABLE_UM);
            self.y += leading(TABLE_UM);
            self.fresh = false;
        }
    }

    fn list_item(&mut self, depth: u64, marker: &str, text: &str) {
        let x = MARGIN_UM + depth * INDENT_UM;
        let text_x = x + INDENT_UM.max(set_width(Role::Sans, marker, BODY_UM) + 1_500);
        let width = (MARGIN_UM + self.measure()).saturating_sub(text_x);
        let lines = wrap_runs(&inline_runs(text, Role::Sans), BODY_UM, width);
        for (index, line) in lines.iter().enumerate() {
            self.room(leading(BODY_UM));
            if index == 0 {
                self.put_line(x, self.y, &[Run::new(marker, Role::Sans)], BODY_UM);
            }
            self.put_line(text_x, self.y, line, BODY_UM);
            self.y += leading(BODY_UM);
            self.fresh = false;
        }
        self.y += PARAGRAPH_GAP_UM / 3;
    }

    /// A Markdown pipe table: its first row heads it, and a rule row under
    /// that sets which columns align right.
    fn markdown_table(&mut self, rows: Vec<Vec<String>>) {
        let mut rows = rows.into_iter();
        let header = rows.next().unwrap_or_default();
        let mut rows = rows.collect::<Vec<_>>();
        let mut right = vec![false; header.len()];
        if let Some(rule) = rows.first()
            && rule.iter().all(|cell| is_rule_cell(cell))
        {
            for (column, cell) in rule.iter().enumerate() {
                if let Some(slot) = right.get_mut(column) {
                    *slot = cell.ends_with(':') && !cell.starts_with(':');
                }
            }
            rows.remove(0);
        }
        let flatten = |cell: &String| plain(&inline_runs(cell, Role::Sans));
        self.table(Table {
            title: String::new(),
            header: header.iter().map(flatten).collect(),
            rows: rows
                .iter()
                .map(|row| row.iter().map(flatten).collect())
                .collect(),
            right,
        });
        self.y += PARAGRAPH_GAP_UM;
    }

    /// "Page n of N" at the foot of every page.
    fn number_pages(&mut self) {
        let count = self.pages.len();
        let foot = self.height - MARGIN_UM / 2;
        let right = self.width - MARGIN_UM;
        for (index, page) in self.pages.iter_mut().enumerate() {
            let text = format!("Page {} of {count}", index + 1);
            let x = right - set_width(Role::Sans, &text, SMALL_UM);
            page.items.push(Laid::Text {
                x,
                y: foot,
                size: SMALL_UM,
                role: Role::Sans,
                text,
            });
        }
    }
}

/// The height of a table row whose cells set as `cells`.
fn row_height(cells: &[Vec<Vec<Run>>]) -> u64 {
    let lines = cells.iter().map(Vec::len).max().unwrap_or(1).max(1) as u64;
    lines * leading(TABLE_UM) + 2 * CELL_PAD_Y_UM
}

/// Column widths for contents `natural` wide, which can narrow to `least`
/// without breaking a word, on a measure `available` wide, each with `pad`
/// around its text. Every column gets what it needs unbroken where the page
/// allows; otherwise each keeps its longest word and the rest of the room is
/// shared in proportion to what each still wants; only where even the words
/// do not fit are they broken, every column narrowed alike.
fn column_widths(natural: &[u64], least: &[u64], available: u64, pad: u64) -> Vec<u64> {
    let natural = natural.iter().map(|width| width + pad).collect::<Vec<_>>();
    let least = least.iter().map(|width| width + pad).collect::<Vec<_>>();
    let wanted: u64 = natural.iter().sum();
    if wanted <= available {
        return natural;
    }
    let floor: u64 = least.iter().sum();
    if floor <= available {
        let spare = (available - floor) as f64;
        let extra = (wanted - floor).max(1) as f64;
        return natural
            .iter()
            .zip(&least)
            .map(|(natural, least)| {
                least + ((natural - least) as f64 * spare / extra).floor() as u64
            })
            .collect();
    }
    least
        .iter()
        .map(|least| {
            ((*least as f64 * available as f64 / floor as f64).floor() as u64).max(pad + 1)
        })
        .collect()
}

fn is_table_row(line: &str) -> bool {
    line.len() > 2 && line.starts_with('|') && line.ends_with('|') && line.matches('|').count() >= 3
}

fn table_cells(line: &str) -> Vec<String> {
    line[1..line.len() - 1]
        .split('|')
        .map(|cell| cell.trim().to_owned())
        .collect()
}

fn is_rule_cell(cell: &str) -> bool {
    let core = cell.trim_start_matches(':').trim_end_matches(':');
    !core.is_empty() && core.chars().all(|character| character == '-')
}

/// A Markdown heading's size and text: `#` to `####`, deeper ones set as the
/// deepest.
fn heading_level(line: &str) -> Option<(u64, &str)> {
    let hashes = line
        .chars()
        .take_while(|character| *character == '#')
        .count();
    if !(1..=6).contains(&hashes) {
        return None;
    }
    let text = line[hashes..].strip_prefix(' ')?;
    Some((HEADING_UM[(hashes - 1).min(3)], text.trim()))
}

/// A Markdown list item: how deep it is nested, its marker and its text.
fn list_item(line: &str) -> Option<(u64, String, &str)> {
    let indent = line.len() - line.trim_start().len();
    let depth = (indent as u64 / 2).min(4);
    let trimmed = line.trim_start();
    if let Some(text) = ["- ", "* ", "+ "]
        .iter()
        .find_map(|marker| trimmed.strip_prefix(marker))
    {
        return Some((depth, "\u{2022}".to_owned(), text.trim()));
    }
    let digits = trimmed.chars().take_while(char::is_ascii_digit).count();
    if digits == 0 || digits > 9 {
        return None;
    }
    let rest = &trimmed[digits..];
    let text = rest
        .strip_prefix(". ")
        .or_else(|| rest.strip_prefix(") "))?;
    Some((depth, format!("{}.", &trimmed[..digits]), text.trim()))
}

#[cfg(test)]
mod tests;
