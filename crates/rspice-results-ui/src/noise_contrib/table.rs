//! Every ranked contributor in a table the reader sorts, selects and pins.

use crate::fft::view::columns::Fitting;
use crate::presentation::{table_row_menu, with_table_state};
use egui::Ui;
use rspice_results::visualization_document::PinnedContributor;
use rspice_ui_kit::widgets::table::{Align, Cell, Column, DataTable, Group, RowKey, SortKey};

/// One source a run ranked: its place in the run's ranking, its
/// band-integrated figure and its share of the total.
pub struct ContributorLine<'a> {
    /// 1 for the source the run ranks first.
    pub rank: usize,
    pub device: &'a str,
    /// Empty when the run ranks the source without naming its mechanism.
    pub mechanism: &'a str,
    pub figure: Option<f64>,
    /// Why the run holds no figure for the source, when it says.
    pub figure_absent: Option<&'a str>,
    pub share_pct: Option<f64>,
}

impl ContributorLine<'_> {
    fn is(&self, pin: &PinnedContributor) -> bool {
        self.device == pin.device && self.mechanism == pin.mechanism
    }

    fn pin(&self) -> PinnedContributor {
        PinnedContributor {
            device: self.device.to_owned(),
            mechanism: self.mechanism.to_owned(),
        }
    }
}

/// What the reader asked of the contributor table this frame.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct ContributorResponse {
    /// The reader pinned or unpinned a contributor.
    pub pins_changed: bool,
    /// The device whose row the reader opened (a double-click, Enter, or
    /// "Show on schematic" in its menu), to be shown on the schematic.
    pub shown: Option<String>,
}

/// The order the table lists `lines` in before the reader sorts it: the
/// pinned contributors first, first pinned first, then the rest in the
/// run's order. Every line is listed once. Returns how many lead as pinned.
pub(super) fn listed_order(
    lines: &[ContributorLine<'_>],
    pinned: &[PinnedContributor],
) -> (Vec<usize>, usize) {
    let mut order: Vec<usize> = pinned
        .iter()
        .filter_map(|pin| lines.iter().position(|line| line.is(pin)))
        .collect();
    let leading = order.len();
    let mut listed = vec![false; lines.len()];
    for &index in &order {
        listed[index] = true;
    }
    order.extend((0..lines.len()).filter(|&index| !listed[index]));
    (order, leading)
}

/// Every contributor in a table whose every column sorts. Opening a row
/// shows its device on the schematic, as its menu's first item does; the
/// menu also pins its contributor above the rest, whatever the sort, and
/// copies the row. Without `pinned` the table offers no pins. The figure
/// column reads in `figure.1`, named in its head.
pub(crate) fn contributor_table(
    ui: &mut Ui,
    salt: &str,
    lines: &[ContributorLine<'_>],
    figure: (&str, &str),
    digits: usize,
    mut pinned: Option<&mut Vec<PinnedContributor>>,
) -> ContributorResponse {
    let (order, leading) = listed_order(lines, pinned.as_deref().map_or(&[], Vec::as_slice));
    let groups = if leading > 0 {
        vec![Group::new("Pinned", 0), Group::new("Contributors", leading)]
    } else {
        Vec::new()
    };
    // The selection follows its contributor when a pin moves its row.
    let keys: Vec<RowKey> = order
        .iter()
        .map(|&index| {
            let line = &lines[index];
            RowKey::Text(format!("{}\u{1f}{}", line.device, line.mechanism))
        })
        .collect();
    let (head, unit) = figure;
    let number = |value: Option<f64>, unit: &str| {
        value.map_or_else(Cell::absent, |value| {
            Cell::number(crate::phase_noise::plain(value, unit, digits))
                .sort_by(SortKey::Number(value))
        })
    };
    let rows: Vec<Vec<Cell>> = order
        .iter()
        .map(|&index| {
            let line = &lines[index];
            let mut figure = number(line.figure, unit);
            if let (None, Some(why)) = (line.figure, line.figure_absent) {
                figure = figure.hint(why);
            }
            vec![
                Cell::number(line.rank.to_string()).sort_by(SortKey::Number(line.rank as f64)),
                Cell::text(line.device).sort_by(SortKey::Text(line.device.to_owned())),
                if line.mechanism.is_empty() {
                    Cell::empty()
                } else {
                    Cell::text(line.mechanism).sort_by(SortKey::Text(line.mechanism.to_owned()))
                },
                figure,
                line.share_pct.map_or_else(Cell::absent, |share| {
                    Cell::number(crate::presentation::fixed_places(
                        share,
                        crate::presentation::SHARE_PLACES,
                    ))
                    .sort_by(SortKey::Number(share))
                }),
            ]
        })
        .collect();
    let columns = contributor_columns(ui, format!("{head} ({unit})"), &rows);
    let revision = {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::hash::DefaultHasher::new();
        (salt, digits, &keys, leading).hash(&mut hasher);
        for line in lines {
            line.figure.map(f64::to_bits).hash(&mut hasher);
            line.share_pct.map(f64::to_bits).hash(&mut hasher);
        }
        hasher.finish()
    };
    let id = format!("rspice.results.noise.{salt}");
    let response = with_table_state(ui, salt, |ui, state| {
        DataTable::new(&id, &columns, order.len())
            .state(state)
            .keys(&keys)
            .freeze(2)
            .groups(&groups)
            .revision(revision)
            .show(ui, |row| rows[row].clone())
    });
    let mut changed = false;
    let mut shown = response
        .activate
        .and_then(|row| order.get(row))
        .map(|&index| lines[index].device.to_owned());
    table_row_menu(ui, salt, response.context_menu, |ui, row| {
        let Some(line) = order.get(row).map(|&index| &lines[index]) else {
            return;
        };
        if ui.button("Show on schematic").clicked() {
            shown = Some(line.device.to_owned());
            ui.close();
        }
        if let Some(pins) = pinned.as_mut() {
            let at = pins.iter().position(|pin| line.is(pin));
            let label = if at.is_some() { "Unpin" } else { "Pin to top" };
            if ui.button(label).clicked() {
                match at {
                    Some(at) => {
                        pins.remove(at);
                    }
                    None => pins.push(line.pin()),
                }
                changed = true;
                ui.close();
            }
        }
        if ui.button("Copy row").clicked() {
            let exact =
                |value: Option<f64>| value.map_or_else(String::new, |value| value.to_string());
            ui.ctx().copy_text(format!(
                "{}\t{}\t{}\t{}\t{}",
                line.rank,
                line.device,
                line.mechanism,
                exact(line.figure),
                exact(line.share_pct)
            ));
            ui.close();
        }
    });
    ContributorResponse {
        pins_changed: changed,
        shown,
    }
}

/// The contributor table's columns: the rank and the two figures as wide as
/// their heads and widest values, and the device and mechanism, which share
/// what the figures leave, as wide as their longest names when the pane
/// allows. In a narrower pane a long name elides, never a figure.
fn contributor_columns(ui: &Ui, figure_head: String, rows: &[Vec<Cell>]) -> [Column; 5] {
    let heads = ["#", "Device", "Noise", figure_head.as_str(), "Share (%)"];
    let named = [false, true, true, false, false];
    let fitting: Vec<Fitting> = heads
        .iter()
        .zip(named)
        .enumerate()
        .map(|(column, (head, name))| Fitting {
            head: (*head).to_owned(),
            tooltip: None,
            align: if name { Align::Left } else { Align::Right },
            cells: rows
                .iter()
                .map(|row| (row[column].text.clone(), !name))
                .collect(),
        })
        .collect();
    let widths = crate::presentation::details_column_widths(
        ui,
        &[&fitting],
        &named,
        crate::presentation::details_table_room(ui),
    );
    [
        Column::number("#", widths[0]),
        Column::name("Device", widths[1]),
        Column::name("Noise", widths[2]),
        Column::number(figure_head, widths[3]),
        Column::number("Share (%)", widths[4]),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pin(device: &str, mechanism: &str) -> PinnedContributor {
        PinnedContributor {
            device: device.to_owned(),
            mechanism: mechanism.to_owned(),
        }
    }

    #[test]
    fn pinned_contributors_lead_in_the_order_pinned_and_every_contributor_is_listed_once() {
        let devices: Vec<String> = (1..=40).map(|rank| format!("D{rank}")).collect();
        let lines: Vec<_> = devices
            .iter()
            .enumerate()
            .map(|(index, device)| ContributorLine {
                rank: index + 1,
                device,
                mechanism: "thermal",
                figure: Some(1.0 / (index + 1) as f64),
                figure_absent: None,
                share_pct: None,
            })
            .collect();
        // A pin names its device and mechanism both; a source the run no
        // longer ranks is skipped.
        let pinned = [
            pin("Dgone", "shot"),
            pin("D20", "thermal"),
            pin("D5", "flicker"),
            pin("D37", "thermal"),
        ];

        let (order, leading) = listed_order(&lines, &pinned);
        assert_eq!(leading, 2);
        assert_eq!(&order[..2], &[19, 36]);
        let mut listed = order.clone();
        listed.sort_unstable();
        assert_eq!(listed, (0..40).collect::<Vec<_>>());
        assert_eq!(&order[2..5], &[0, 1, 2]);

        let (order, leading) = listed_order(&lines, &[]);
        assert_eq!(leading, 0);
        assert_eq!(order, (0..40).collect::<Vec<_>>());
    }
    /// A double-click on a contributor's row asks for its device on the
    /// schematic, as a waveform's probe does.
    #[test]
    fn a_double_click_on_a_contributor_asks_for_its_device_on_the_schematic() {
        let ctx = egui::Context::default();
        rspice_ui_kit::Theme::default().apply(&ctx);
        let lines = [(1, "R1"), (2, "M1")].map(|(rank, device)| ContributorLine {
            rank,
            device,
            mechanism: "thermal",
            figure: Some(1e-12),
            figure_absent: None,
            share_pct: Some(50.0),
        });
        let frame = |events: Vec<egui::Event>| {
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(600.0, 400.0),
                )),
                events,
                ..Default::default()
            };
            let mut response = ContributorResponse::default();
            let (_, geometry) = rspice_ui_kit::probe::pass(&ctx, input, |ui| {
                response = contributor_table(ui, "test", &lines, ("Power", "V2"), 4, None);
            });
            (response, geometry)
        };
        let (_, geometry) = frame(Vec::new());
        let row = geometry
            .one("table.line.1")
            .expect("M1's row is shown")
            .center();
        let button = |pressed| egui::Event::PointerButton {
            pos: row,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        };
        let (response, _) = frame(vec![
            egui::Event::PointerMoved(row),
            button(true),
            button(false),
            button(true),
            button(false),
        ]);
        assert_eq!(response.shown.as_deref(), Some("M1"));
        assert!(!response.pins_changed);
    }
}
