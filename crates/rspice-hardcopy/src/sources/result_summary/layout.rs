//! Where a printed result summary's words fall: its title, then each table
//! under its own title and header row, running on to as many pages as its
//! rows need, with the header row repeated at the top of every page it runs
//! on to. Nothing is left off and nothing is set over its neighbour.

use super::super::report_layout::set_width;
use super::super::{PLOT_HEIGHT_UM, PLOT_WIDTH_UM, REPORT_PAGE_GAP_UM};
use crate::sources::{SemanticResultSummary, SemanticTable};
use rspice_fonts::Role;
use rspice_results::number_format::leaves_prefix_slot_blank;

/// A summary page is the landscape page a plot prints on: a summary prints
/// on the same paper, so it is laid out to fill it.
pub(crate) const SUMMARY_PAGE_WIDTH_UM: u64 = PLOT_WIDTH_UM.unsigned_abs();
pub(crate) const SUMMARY_PAGE_HEIGHT_UM: u64 = PLOT_HEIGHT_UM.unsigned_abs();
/// Where each page starts below the last.
pub(crate) const SUMMARY_PAGE_PITCH_UM: u64 =
    SUMMARY_PAGE_HEIGHT_UM + REPORT_PAGE_GAP_UM.unsigned_abs();
pub(crate) const SUMMARY_MARGIN_UM: u64 = 12_000;
const FOOT_UM: u64 = 8_000;
pub(crate) const SUMMARY_TITLE_UM: u64 = 4_500;
pub(crate) const TABLE_TITLE_UM: u64 = 3_200;
const TABLE_TITLE_LEAD_UM: u64 = 5_000;
const HEADER_UM: u64 = 2_300;
const CELL_UM: u64 = 2_100;
const ROW_UM: u64 = 4_500;
const PAD_UM: u64 = 800;

/// One cell of a laid row: its box across the page and its words.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LaidCell {
    pub x: u64,
    pub width: u64,
    pub text: String,
    /// Set against the right of its box, as the sheet sets a quantity.
    pub right: bool,
    /// Room kept blank after the words, as the sheet keeps a blank prefix
    /// slot, so `1.000 V` ends its digits where `707.1 mV` does.
    pub slot: u64,
}

/// One thing a summary page shows, at its height down the stacked pages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LaidSummary {
    Title {
        y: u64,
        text: String,
    },
    TableTitle {
        y: u64,
        text: String,
    },
    Row {
        y: u64,
        height: u64,
        /// The size its words set at: a table wider than the page sets
        /// smaller rather than run its words into each other.
        size: u64,
        pad: u64,
        header: bool,
        cells: Vec<LaidCell>,
    },
}

impl LaidSummary {
    /// The page it falls on.
    pub(crate) fn page(&self) -> u64 {
        let (Self::Title { y, .. } | Self::TableTitle { y, .. } | Self::Row { y, .. }) = self;
        y / SUMMARY_PAGE_PITCH_UM
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SummaryLayout {
    pub pages: u64,
    pub items: Vec<LaidSummary>,
}

impl SummaryLayout {
    /// How tall the pages stand, one above the next with the gap between.
    pub(crate) fn height(&self) -> u64 {
        self.pages.max(1) * SUMMARY_PAGE_PITCH_UM - REPORT_PAGE_GAP_UM.unsigned_abs()
    }
}

/// Lay `summary` out on as many pages as it takes.
pub(crate) fn lay_out_summary(summary: &SemanticResultSummary) -> SummaryLayout {
    let mut flow = Flow {
        page: 0,
        y: SUMMARY_MARGIN_UM,
        items: vec![LaidSummary::Title {
            y: SUMMARY_MARGIN_UM,
            text: summary.title.clone(),
        }],
    };
    flow.y += 9_000;
    for table in &summary.tables {
        flow.table(table);
    }
    SummaryLayout {
        pages: flow.page + 1,
        items: flow.items,
    }
}

struct Flow {
    page: u64,
    /// Down the current page.
    y: u64,
    items: Vec<LaidSummary>,
}

impl Flow {
    /// Down the stacked pages.
    fn at(&self) -> u64 {
        self.page * SUMMARY_PAGE_PITCH_UM + self.y
    }

    /// Whether `height` more fits above the foot of this page.
    fn fits(&self, height: u64) -> bool {
        self.y + height <= SUMMARY_PAGE_HEIGHT_UM - FOOT_UM
    }

    fn next_page(&mut self) {
        self.page += 1;
        self.y = SUMMARY_MARGIN_UM;
    }

    fn table(&mut self, table: &SemanticTable) {
        let columns = table.columns.len().max(1);
        let available = SUMMARY_PAGE_WIDTH_UM - 2 * SUMMARY_MARGIN_UM;
        let right = (0..columns)
            .map(|column| reads_as_quantity(table, column))
            .collect::<Vec<_>>();
        // One prefix glyph's room after a reading that prints no prefix in a
        // quantity column, as the sheet keeps it.
        let slot = |column: usize, text: &str, size: u64| {
            if right[column] && leaves_prefix_slot_blank(text) {
                set_width(Role::Mono, "m", size)
            } else {
                0
            }
        };
        let natural = (0..columns)
            .map(|column| {
                let header = table.columns.get(column).map_or(0, |heading| {
                    set_width(Role::SansSemibold, heading, HEADER_UM)
                });
                let cells = table
                    .rows
                    .iter()
                    .filter_map(|row| row.get(column))
                    .map(|cell| set_width(Role::Mono, cell, CELL_UM) + slot(column, cell, CELL_UM))
                    .max()
                    .unwrap_or(0);
                header.max(cells) + 2 * PAD_UM
            })
            .collect::<Vec<_>>();
        let wanted = natural.iter().sum::<u64>();
        // The columns run the page's width, as the sheet's run its pane's;
        // a table wider than the page sets smaller, every column alike.
        let (scale, widths) = if wanted > available {
            let scale = available as f64 / wanted as f64;
            let widths = natural
                .iter()
                .map(|width| (*width as f64 * scale).floor() as u64)
                .collect::<Vec<_>>();
            (scale, widths)
        } else {
            let spare = (available - wanted) / columns as u64;
            (1.0, natural.iter().map(|width| width + spare).collect())
        };
        let scaled = |value: u64| ((value as f64 * scale).round() as u64).max(1);
        let (row, header_size, cell_size, pad) = (
            scaled(ROW_UM),
            scaled(HEADER_UM),
            scaled(CELL_UM),
            scaled(PAD_UM),
        );
        // A header names its column; only a value keeps a prefix slot.
        let lay = |texts: &[String], header: bool| {
            let mut x = SUMMARY_MARGIN_UM;
            widths
                .iter()
                .enumerate()
                .map(|(column, width)| {
                    let text = texts.get(column).cloned().unwrap_or_default();
                    let cell = LaidCell {
                        x,
                        width: *width,
                        slot: if header {
                            0
                        } else {
                            slot(column, &text, cell_size)
                        },
                        text,
                        right: right[column],
                    };
                    x += width;
                    cell
                })
                .collect::<Vec<_>>()
        };
        let header = lay(&table.columns, true);
        // A table's title, its header and its first row stand together.
        let title = if table.title.is_empty() {
            0
        } else {
            TABLE_TITLE_LEAD_UM
        };
        if !self.fits(title + 2 * row) && self.y > SUMMARY_MARGIN_UM {
            self.next_page();
        }
        if title > 0 {
            self.items.push(LaidSummary::TableTitle {
                y: self.at(),
                text: table.title.clone(),
            });
            self.y += title;
        }
        let header_row = |at: u64| LaidSummary::Row {
            y: at,
            height: row,
            size: header_size,
            pad,
            header: true,
            cells: header.clone(),
        };
        self.items.push(header_row(self.at()));
        self.y += row;
        for cells in &table.rows {
            if !self.fits(row) {
                self.next_page();
                self.items.push(header_row(self.at()));
                self.y += row;
            }
            self.items.push(LaidSummary::Row {
                y: self.at(),
                height: row,
                size: cell_size,
                pad,
                header: false,
                cells: lay(cells, false),
            });
            self.y += row;
        }
        self.y += 5_000;
    }
}

/// Whether a column reads as quantities, which the sheet sets against the
/// right of their cells: every value it holds a number, a sign or infinity
/// first, past the cursor flag a sample row may carry.
fn reads_as_quantity(table: &SemanticTable, column: usize) -> bool {
    let mut values = table
        .rows
        .iter()
        .filter_map(|row| row.get(column))
        .map(|cell| {
            let cell = cell.trim();
            ["A/B ", "A ", "B "]
                .iter()
                .find_map(|flag| cell.strip_prefix(flag))
                .unwrap_or(cell)
                .trim_start()
        })
        .filter(|cell| !cell.is_empty() && *cell != "\u{2014}")
        .peekable();
    values.peek().is_some()
        && values.all(|cell| {
            cell.starts_with(|first: char| {
                first.is_ascii_digit() || matches!(first, '\u{2212}' | '-' | '+' | '.' | '\u{221e}')
            })
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_results::result_presentation::ResultViewer;

    fn summary(rows: usize) -> SemanticResultSummary {
        SemanticResultSummary {
            viewer: ResultViewer::Table,
            title: "Samples".to_owned(),
            tables: vec![SemanticTable {
                title: String::new(),
                columns: vec!["Row".to_owned(), "time".to_owned(), "V(out)".to_owned()],
                rows: (0..rows)
                    .map(|row| {
                        vec![
                            row.to_string(),
                            format!("{row} ms"),
                            "\u{2212}1.000 V".to_owned(),
                        ]
                    })
                    .collect(),
            }],
            payload: None,
        }
    }

    /// A long table runs on to further pages with its header row at the top
    /// of each, and leaves none of its rows off.
    #[test]
    fn a_long_table_runs_on_with_its_header_on_every_page() {
        let layout = lay_out_summary(&summary(200));
        assert!(layout.pages > 1, "{} pages", layout.pages);
        let rows = layout
            .items
            .iter()
            .filter(|item| matches!(item, LaidSummary::Row { header: false, .. }))
            .count();
        assert_eq!(rows, 200);
        let headers = layout
            .items
            .iter()
            .filter_map(|item| match item {
                LaidSummary::Row {
                    header: true, y, ..
                } => Some(y / SUMMARY_PAGE_PITCH_UM),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(headers, (0..layout.pages).collect::<Vec<_>>());
        // Every row stands on a page, above its foot.
        for item in &layout.items {
            if let LaidSummary::Row { y, height, .. } = item {
                assert!(y % SUMMARY_PAGE_PITCH_UM + height <= SUMMARY_PAGE_HEIGHT_UM - FOOT_UM);
            }
        }
        assert_eq!(
            layout.height(),
            layout.pages * SUMMARY_PAGE_PITCH_UM - REPORT_PAGE_GAP_UM.unsigned_abs()
        );
    }

    /// Numbers stand against the right of their cells, as on the sheet; a
    /// column of names stands left.
    #[test]
    fn a_column_of_quantities_is_set_right() {
        let table = SemanticTable {
            title: String::new(),
            columns: vec!["Out / In".to_owned(), "P1".to_owned()],
            rows: vec![
                vec![
                    "P1 \u{b7} 50 \u{3a9}".to_owned(),
                    "0.5000 \u{2220} \u{2212}90.00\u{b0}".to_owned(),
                ],
                vec![
                    "P2 \u{b7} 50 \u{3a9}".to_owned(),
                    "A  \u{2212}0.25".to_owned(),
                ],
            ],
        };
        assert!(!reads_as_quantity(&table, 0));
        assert!(reads_as_quantity(&table, 1));
    }

    /// A reading that prints no prefix ends its digits where a prefixed one
    /// does, as on the sheet: `1.000 V` keeps a prefix's room after it.
    #[test]
    fn a_reading_without_a_prefix_keeps_its_slot() {
        let mut summary = summary(0);
        summary.tables[0].rows = vec![
            vec!["0".to_owned(), "1.000 ns".to_owned(), "1.000 V".to_owned()],
            vec!["1".to_owned(), "2.000 ns".to_owned(), "707.1 mV".to_owned()],
        ];
        let rows = lay_out_summary(&summary)
            .items
            .into_iter()
            .filter_map(|item| match item {
                LaidSummary::Row {
                    header, size, cells, ..
                } => Some((header, size, cells)),
                _ => None,
            })
            .collect::<Vec<_>>();
        let [(true, _, head), (false, size, volt), (false, _, millivolt)] = &rows[..] else {
            panic!("{rows:?}");
        };
        assert!(head.iter().all(|cell| cell.slot == 0), "{head:?}");
        let digits_end = |cell: &LaidCell| {
            let unit = cell.text.rsplit_once(' ').unwrap().1;
            cell.x + cell.width - cell.slot - set_width(Role::Mono, &format!(" {unit}"), *size)
        };
        assert!(volt[2].slot > 0);
        assert_eq!(millivolt[2].slot, 0);
        // Each width is rounded up to the micrometre on its own.
        let (volt, millivolt) = (digits_end(&volt[2]), digits_end(&millivolt[2]));
        assert!(volt.abs_diff(millivolt) <= 1, "{volt} {millivolt}");
    }
}
