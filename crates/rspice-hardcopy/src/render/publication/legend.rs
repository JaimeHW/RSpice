//! The printed legend's layout, shared by the vector and raster writers.
//!
//! Entries fill the plan's [`LegendGrid`] column by column. A column takes the
//! width its longest label needs, never less than the grid's narrowest
//! column, and the columns share the printable width out when together they
//! need more. A label still too long for its column keeps its end, the
//! object's own name, behind an ellipsis.

use unicode_segmentation::UnicodeSegmentation as _;

use super::*;

/// Legend labels are set in Plex Sans at this size.
pub(super) const LEGEND_TEXT_UM: u64 = 2_600;
/// A swatch starts this far inside its column.
const SWATCH_INSET_UM: u64 = 2_000;
/// A label starts this far right of its swatch's start.
const LABEL_OFFSET_UM: u64 = 15_000;
/// Clear space between a column's longest label and the next column.
const COLUMN_GAP_UM: u64 = 2_000;
/// The first row's swatch line lies this far below the band's top.
const FIRST_ROW_UM: u64 = 3_500;
/// A label's baseline lies this far below its swatch line.
const BASELINE_DROP_UM: u64 = 900;
/// Joins the source path, sheet and object of a mapped object's name.
const SEPARATOR: &str = " · ";
const ELLIPSIS: &str = "…";

pub(super) struct LegendSlot<'a> {
    pub(super) entry: &'a LegendEntry,
    /// The swatch's left end and the height of its line.
    pub(super) swatch: (u64, u64),
    /// The label's start and baseline.
    pub(super) origin: (u64, u64),
    pub(super) label: String,
}

pub(super) struct LegendLayout<'a> {
    /// The band's top edge on the page.
    pub(super) top_um: u64,
    pub(super) slots: Vec<LegendSlot<'a>>,
}

/// One label measured grapheme by grapheme, in font units.
struct MeasuredLabel<'l> {
    text: &'l str,
    graphemes: Vec<(usize, u64)>,
}

impl MeasuredLabel<'_> {
    fn width(&self) -> u64 {
        self.graphemes.iter().map(|(_, width)| width).sum()
    }

    /// The label whole when it fits `budget`, otherwise its first `head`
    /// graphemes, an ellipsis and as much of its end as fits.
    fn shortened(&self, budget: u64, ellipsis: u64, head: usize) -> Option<String> {
        if self.width() <= budget {
            return Some(self.text.to_owned());
        }
        let head = head.min(self.graphemes.len());
        let head_width: u64 = self.graphemes[..head].iter().map(|(_, width)| width).sum();
        let mut room = budget.checked_sub(head_width.checked_add(ellipsis)?)?;
        let mut start = self.graphemes.len();
        while start > head && self.graphemes[start - 1].1 <= room {
            start -= 1;
            room -= self.graphemes[start].1;
        }
        let offset = |index: usize| {
            self.graphemes
                .get(index)
                .map_or(self.text.len(), |(offset, _)| *offset)
        };
        // The kept end starts at a word rather than inside one, where it can.
        let after_space = |index: usize| {
            self.text[offset(index - 1)..offset(index)]
                .trim()
                .is_empty()
        };
        if start > head
            && !after_space(start)
            && let Some(word) = (start + 1..self.graphemes.len()).find(|index| after_space(*index))
        {
            start = word;
        }
        let tail = self.text[offset(start)..].trim_start_matches([' ', '·']);
        Some(format!("{}{ELLIPSIS}{tail}", &self.text[..offset(head)]))
    }
}

/// Lay out the legend band of `page`, or nothing when the page prints none.
pub(super) fn legend_layout<'a>(
    plan: &HardcopyPlan,
    scene: &'a HardcopyScene,
    page: &PreviewPage,
) -> Result<Option<LegendLayout<'a>>, HardcopyRenderError> {
    if !plan.setup().decorations().includes_legends() || scene.legend.is_empty() {
        return Ok(None);
    }
    let geometry = page.geometry();
    let printable = geometry.printable_rect();
    let content = geometry.content_rect();
    let top_um = content
        .y
        .micrometres()
        .saturating_add(content.height.micrometres());
    let grid = LegendGrid::of_band(printable.width, geometry.legend_band());
    let rows = usize::try_from(grid.rows().max(1)).unwrap_or(usize::MAX);
    let shown = usize::try_from(grid.capacity())
        .unwrap_or(usize::MAX)
        .min(scene.legend.len());
    let face = FontCoverage::load()?.sans;
    let units = u64::from(face.units_per_em());
    let to_um = |value: u64| value.saturating_mul(LEGEND_TEXT_UM).div_ceil(units);
    let to_units = |value: u64| value.saturating_mul(units) / LEGEND_TEXT_UM;
    let measure = |text: &str| -> Result<Vec<(usize, u64)>, HardcopyRenderError> {
        text.grapheme_indices(true)
            .map(|(offset, grapheme)| {
                grapheme_width(&face, "IBM Plex Sans Regular", grapheme, "print legend")
                    .map(|width| (offset, width))
            })
            .collect()
    };
    let labels = page_labels(scene)
        .into_iter()
        .take(shown)
        .map(|text| {
            Ok(MeasuredLabel {
                text,
                graphemes: measure(text)?,
            })
        })
        .collect::<Result<Vec<_>, HardcopyRenderError>>()?;
    let ellipsis = measure(ELLIPSIS)?.iter().map(|(_, width)| width).sum();
    let chrome = SWATCH_INSET_UM + LABEL_OFFSET_UM + COLUMN_GAP_UM;
    let mut natural = vec![LegendGrid::MIN_COLUMN_WIDTH.micrometres(); shown.div_ceil(rows)];
    for (index, label) in labels.iter().enumerate() {
        let column = &mut natural[index / rows];
        *column = (*column).max(chrome.saturating_add(to_um(label.width())));
    }
    let widths = share_width(&natural, printable.width.micrometres());
    let mut lefts = Vec::with_capacity(widths.len());
    let mut left = printable.x.micrometres();
    for width in &widths {
        lefts.push(left);
        left = left.saturating_add(*width);
    }
    let budgets: Vec<u64> = (0..shown)
        .map(|index| to_units(widths[index / rows].saturating_sub(chrome)))
        .collect();
    let mut printed: Vec<String> = labels
        .iter()
        .zip(&budgets)
        .map(|(label, budget)| {
            label
                .shortened(*budget, ellipsis, 0)
                .unwrap_or_else(|| ELLIPSIS.to_owned())
        })
        .collect();
    // Two labels that differ only before the ends they keep would read the
    // same: each keeps its start through the first grapheme where it differs.
    let mut alike: BTreeMap<&str, Vec<usize>> = BTreeMap::new();
    for (index, text) in printed.iter().enumerate() {
        alike.entry(text.as_str()).or_default().push(index);
    }
    let mut heads = Vec::new();
    for group in alike.values().filter(|group| group.len() > 1) {
        for &index in group {
            let head = group
                .iter()
                .filter(|other| **other != index)
                .map(|&other| {
                    labels[index]
                        .text
                        .graphemes(true)
                        .zip(labels[other].text.graphemes(true))
                        .take_while(|(left, right)| left == right)
                        .count()
                        + 1
                })
                .max()
                .unwrap_or(0);
            heads.push((index, head));
        }
    }
    for (index, head) in heads {
        if let Some(shortened) = labels[index].shortened(budgets[index], ellipsis, head) {
            printed[index] = shortened;
        }
    }
    let slots = scene
        .legend
        .iter()
        .zip(printed)
        .enumerate()
        .map(|(index, (entry, label))| {
            let x = lefts[index / rows].saturating_add(SWATCH_INSET_UM);
            let y = top_um
                .saturating_add(FIRST_ROW_UM)
                .saturating_add(LegendGrid::ROW_PITCH.micrometres() * (index % rows) as u64);
            LegendSlot {
                entry,
                swatch: (x, y),
                origin: (x + LABEL_OFFSET_UM, y + BASELINE_DROP_UM),
                label,
            }
        })
        .collect();
    Ok(Some(LegendLayout { top_um, slots }))
}

/// Column widths: each its natural width while they fit `available`;
/// otherwise the narrow ones keep theirs and the wide ones share the rest
/// evenly.
fn share_width(natural: &[u64], available: u64) -> Vec<u64> {
    if natural.iter().sum::<u64>() <= available {
        return natural.to_vec();
    }
    let mut sorted = natural.to_vec();
    sorted.sort_unstable();
    let mut remaining = available;
    let mut level = 0;
    for (index, width) in sorted.iter().enumerate() {
        let share = remaining / (sorted.len() - index) as u64;
        if *width > share {
            level = share;
            break;
        }
        remaining -= width;
    }
    natural.iter().map(|width| (*width).min(level)).collect()
}

/// The legend's labels without the source path every one of them begins
/// with: a page of one cell view's sheets prints "First · Wires", not the
/// cell view on every row. A label names its source by the aggregate
/// section it begins with; when any label names none, all print whole.
/// Taking the same prefix off every label keeps distinct labels distinct.
fn page_labels(scene: &HardcopyScene) -> Vec<&str> {
    let labels: Vec<&str> = scene
        .legend
        .iter()
        .map(|entry| entry.label.as_str())
        .collect();
    let mut shared: Option<Vec<&str>> = None;
    for label in &labels {
        let Some(source) = scene
            .aggregate_sections
            .iter()
            .map(|section| section.display_name.as_str())
            .filter(|source| {
                label
                    .strip_prefix(source)
                    .and_then(|rest| rest.strip_prefix(SEPARATOR))
                    .is_some_and(|own_name| !own_name.is_empty())
            })
            .max_by_key(|source| source.len())
        else {
            return labels;
        };
        let segments: Vec<&str> = source.split(SEPARATOR).collect();
        shared = Some(match shared {
            None => segments,
            Some(shared) => shared
                .into_iter()
                .zip(segments)
                .take_while(|(left, right)| left == right)
                .map(|(segment, _)| segment)
                .collect(),
        });
    }
    let Some(shared) = shared.filter(|shared| !shared.is_empty()) else {
        return labels;
    };
    let prefix =
        shared.iter().map(|segment| segment.len()).sum::<usize>() + shared.len() * SEPARATOR.len();
    labels.into_iter().map(|label| &label[prefix..]).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scene_of(sources: &[&str], labels: &[&str]) -> HardcopyScene {
        let extent = ContentExtent::try_new(
            Length::from_micrometres(10_000),
            Length::from_micrometres(10_000),
        )
        .unwrap();
        let stroke = StrokeStyle::try_new(
            SemanticColor::Foreground,
            Length::from_micrometres(250),
            StrokePattern::Solid,
            None,
        )
        .unwrap();
        HardcopyScene {
            extent,
            metadata: HardcopySceneMetadata::try_new("Sheets", "RSpice tests").unwrap(),
            primitives: Vec::new(),
            legend: labels
                .iter()
                .map(|label| LegendEntry::try_new(*label, stroke).unwrap())
                .collect(),
            aggregate_sections: sources
                .iter()
                .enumerate()
                .map(|(ordinal, source)| AggregateSection {
                    ordinal: ordinal as u32,
                    source_key: format!("child:{ordinal}"),
                    display_name: (*source).to_owned(),
                    content_digest: ContentDigest::from_bytes([0; 32]),
                    origin: ScenePoint::new(Length::ZERO, Length::ZERO),
                    extent,
                    page_break_before: ordinal > 0,
                    primitive_start: 0,
                    primitive_end: 0,
                })
                .collect(),
        }
    }

    #[test]
    fn a_page_of_one_cell_views_sheets_does_not_repeat_the_cell_view() {
        let sheets = ["user/top/schematic · First", "user/top/schematic · Second"];
        let scene = scene_of(
            &sheets,
            &[
                "user/top/schematic · First · Drawing sheet title block",
                "user/top/schematic · Second · Drawing sheet title block",
                "user/top/schematic · Second · Wires and junctions",
            ],
        );
        assert_eq!(
            page_labels(&scene),
            [
                "First · Drawing sheet title block",
                "Second · Drawing sheet title block",
                "Second · Wires and junctions",
            ]
        );
        // Every entry from one sheet: the sheet is shared too.
        let scene = scene_of(
            &sheets,
            &[
                "user/top/schematic · Second · Drawing sheet paper",
                "user/top/schematic · Second · Wires and junctions",
            ],
        );
        assert_eq!(
            page_labels(&scene),
            ["Drawing sheet paper", "Wires and junctions"]
        );
    }

    #[test]
    fn labels_print_whole_unless_every_one_shares_a_source() {
        // Different cell views share no source path.
        let scene = scene_of(
            &["lib/amp/schematic · Main", "lib/bias/schematic · Main"],
            &[
                "lib/amp/schematic · Main · Wires and junctions",
                "lib/bias/schematic · Main · Wires and junctions",
            ],
        );
        assert_eq!(
            page_labels(&scene),
            [
                "lib/amp/schematic · Main · Wires and junctions",
                "lib/bias/schematic · Main · Wires and junctions",
            ]
        );
        // A name with a separator of its own is not a source path.
        let scene = scene_of(&[], &["report review note · leader"]);
        assert_eq!(page_labels(&scene), ["report review note · leader"]);
        let scene = scene_of(
            &["user/top/schematic · First"],
            &["user/top/schematic · First · Wires and junctions", "V(out)"],
        );
        assert_eq!(
            page_labels(&scene),
            ["user/top/schematic · First · Wires and junctions", "V(out)"]
        );
    }

    #[test]
    fn a_shortened_label_keeps_its_end_and_where_it_differs() {
        let face = FontCoverage::load().unwrap().sans;
        let measure = |text: &'static str| MeasuredLabel {
            text,
            graphemes: text
                .grapheme_indices(true)
                .map(|(offset, grapheme)| {
                    (
                        offset,
                        grapheme_width(&face, "sans", grapheme, "test").unwrap(),
                    )
                })
                .collect(),
        };
        let label = measure("lib/amp/schematic · Main · Wires and junctions");
        let own_name = measure("Wires and junctions");
        let ellipsis = measure(ELLIPSIS).width();
        assert_eq!(
            label.shortened(label.width(), ellipsis, 0).unwrap(),
            label.text
        );
        let budget = own_name.width() + ellipsis;
        assert_eq!(
            label.shortened(budget, ellipsis, 0).unwrap(),
            "…Wires and junctions"
        );
        // The end it keeps starts at a word, never inside one.
        assert_eq!(
            label
                .shortened(measure("nd junctions").width() + ellipsis, ellipsis, 0)
                .unwrap(),
            "…junctions"
        );
        let head = measure("lib/a").width();
        assert_eq!(
            label.shortened(budget + head, ellipsis, 5).unwrap(),
            "lib/a…Wires and junctions"
        );
        assert_eq!(label.shortened(ellipsis - 1, ellipsis, 0), None);
    }

    #[test]
    fn columns_keep_their_width_until_they_must_share() {
        assert_eq!(share_width(&[64, 90, 70], 300), vec![64, 90, 70]);
        assert_eq!(share_width(&[64, 120, 130], 250), vec![64, 93, 93]);
        assert_eq!(share_width(&[100, 100], 150), vec![75, 75]);
    }
}
