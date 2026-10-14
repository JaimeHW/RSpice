//! A panel's geometry read back from a render: where each element landed and
//! the rhythm between them, as numbers a review can quote.
//!
//! Elements are what the panel published to assistive technology — a heading,
//! a row with a value, a field, a button — so a measurement names what a reader
//! meets rather than whichever shapes happened to paint it. A banded section
//! header that publishes no heading is found by its band.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use egui::accesskit::{Node, Role};
use egui::{Color32, Rect, Shape};

/// What a measured element is to a reader.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PanelElementKind {
    /// A section header.
    Header,
    /// A group's header inside a section's body: the explicit second level.
    Subheader,
    /// A label beside a value, or a measurement-table row.
    Row,
    /// A line of text that is neither: a note, a problem, a caption.
    Note,
    /// A full-width row that folds or unfolds a group.
    Disclosure,
    /// A text field.
    Field,
    /// A select or combo box.
    Select,
    /// A button.
    Button,
    /// A checkbox or switch.
    Toggle,
}

impl PanelElementKind {
    /// Whether the element's edges are where its text is, rather than where
    /// its frame is: a row is as wide as the panel, its words are not.
    fn measured_by_text(self) -> bool {
        matches!(
            self,
            Self::Header | Self::Subheader | Self::Row | Self::Note | Self::Disclosure
        )
    }
}

/// One element, where it landed, and the text painted inside it.
#[derive(Debug, Clone)]
pub struct PanelElement {
    pub kind: PanelElementKind,
    pub rect: Rect,
    /// The union of every text run painted inside the element.
    pub text: Option<Rect>,
}

impl PanelElement {
    fn left(&self) -> f32 {
        match self.text {
            Some(text) if self.kind.measured_by_text() => text.left(),
            _ => self.rect.left(),
        }
    }

    fn right(&self) -> f32 {
        match self.text {
            Some(text) if self.kind.measured_by_text() => text.right(),
            _ => self.rect.right(),
        }
    }
}

/// Every element a panel render published, top to bottom.
#[derive(Debug, Clone, Default)]
pub struct PanelLayout {
    pub width: f32,
    pub elements: Vec<PanelElement>,
    /// Every visible text run, with its layout box.
    pub texts: Vec<(String, Rect)>,
}

impl PanelLayout {
    fn read(width: f32, output: &egui::FullOutput, band: Color32) -> Self {
        let texts = visible_texts(&output.shapes);
        let mut elements = Vec::new();
        if let Some(update) = &output.platform_output.accesskit_update {
            for (_, node) in &update.nodes {
                if let Some(element) = element_of(node, width) {
                    elements.push(element);
                }
            }
        }
        for rect in band_rects(&output.shapes, band, width) {
            let known = elements.iter().any(|element| {
                element.kind == PanelElementKind::Header
                    && element.rect.intersects(rect.shrink(2.0))
            });
            if !known {
                elements.push(PanelElement {
                    kind: PanelElementKind::Header,
                    rect,
                    text: None,
                });
            }
        }
        // Each run belongs to the smallest element around its centre.
        for (_, rect) in &texts {
            let owner = elements
                .iter()
                .enumerate()
                .filter(|(_, element)| element.rect.expand(0.5).contains(rect.center()))
                .min_by(|(_, a), (_, b)| a.rect.area().total_cmp(&b.rect.area()))
                .map(|(owner, _)| owner);
            if let Some(owner) = owner {
                let element = &mut elements[owner];
                element.text = Some(element.text.map_or(*rect, |union| union.union(*rect)));
            }
        }
        elements.sort_by(|a, b| {
            a.rect
                .top()
                .total_cmp(&b.rect.top())
                .then(a.rect.left().total_cmp(&b.rect.left()))
        });
        Self {
            width,
            elements,
            texts,
        }
    }

    fn of_kind(&self, kind: PanelElementKind) -> impl Iterator<Item = &PanelElement> {
        self.elements
            .iter()
            .filter(move |element| element.kind == kind)
    }

    /// The headers, top to bottom, each with the space from its bottom to the
    /// first element under it and from the last element of the section above
    /// to its top.
    fn header_rhythm(&self) -> (Vec<f32>, Vec<f32>) {
        let headers: Vec<&PanelElement> = self.of_kind(PanelElementKind::Header).collect();
        let mut first_gaps = Vec::new();
        let mut section_gaps = Vec::new();
        for (index, header) in headers.iter().enumerate() {
            let next_header_top = headers
                .get(index + 1)
                .map_or(f32::INFINITY, |next| next.rect.top());
            let first = self
                .elements
                .iter()
                .filter(|element| {
                    element.kind != PanelElementKind::Header
                        && element.rect.top() >= header.rect.bottom() - 0.5
                        && element.rect.top() < next_header_top
                })
                .map(|element| element.rect.top())
                .fold(f32::INFINITY, f32::min);
            if first.is_finite() {
                first_gaps.push(first - header.rect.bottom());
            }
            if index > 0 {
                let previous = headers[index - 1];
                let last = self
                    .elements
                    .iter()
                    .filter(|element| {
                        element.kind != PanelElementKind::Header
                            && element.rect.top() >= previous.rect.bottom() - 0.5
                            && element.rect.bottom() <= header.rect.top() + 0.5
                    })
                    .map(|element| element.rect.bottom())
                    .fold(f32::NEG_INFINITY, f32::max);
                if last.is_finite() {
                    section_gaps.push(header.rect.top() - last);
                }
            }
        }
        (first_gaps, section_gaps)
    }

    /// The space from each subheader's bottom to the first element of its
    /// group, when the group has one.
    fn subheader_first_gaps(&self) -> Vec<f32> {
        let is_header =
            |kind| matches!(kind, PanelElementKind::Header | PanelElementKind::Subheader);
        self.of_kind(PanelElementKind::Subheader)
            .filter_map(|subheader| {
                let next_header = self
                    .elements
                    .iter()
                    .filter(|element| {
                        is_header(element.kind) && element.rect.top() >= subheader.rect.bottom()
                    })
                    .map(|element| element.rect.top())
                    .fold(f32::INFINITY, f32::min);
                let first = self
                    .elements
                    .iter()
                    .filter(|element| {
                        !is_header(element.kind)
                            && element.rect.top() >= subheader.rect.bottom() - 0.5
                            && element.rect.top() < next_header
                    })
                    .map(|element| element.rect.top())
                    .fold(f32::INFINITY, f32::min);
                first.is_finite().then(|| first - subheader.rect.bottom())
            })
            .collect()
    }

    /// The review table: one line per element kind with every distinct left
    /// and right edge, then the vertical rhythm, then any text that starts
    /// left of `inset`.
    pub fn report(&self, name: &str, inset: f32) -> String {
        let mut out = String::new();
        let _ = writeln!(out, "{name} ({} px)", self.width);
        let mut kinds: BTreeMap<PanelElementKind, Vec<&PanelElement>> = BTreeMap::new();
        for element in &self.elements {
            kinds.entry(element.kind).or_default().push(element);
        }
        let _ = writeln!(out, "  {:<11} {:>3}  {:<28} right x", "kind", "n", "left x");
        for (kind, elements) in &kinds {
            let lefts = distinct(elements.iter().map(|element| element.left()));
            let rights = distinct(elements.iter().map(|element| element.right()));
            let _ = writeln!(
                out,
                "  {:<11} {:>3}  {:<28} {}",
                format!("{kind:?}"),
                elements.len(),
                lefts,
                rights
            );
        }
        let heights = |kind| distinct(self.of_kind(kind).map(|element| element.rect.height()));
        let (first_gaps, section_gaps) = self.header_rhythm();
        let _ = writeln!(out, "  header height {}", heights(PanelElementKind::Header));
        let _ = writeln!(out, "  header to first {}", distinct(first_gaps));
        let _ = writeln!(
            out,
            "  subheader height {}",
            heights(PanelElementKind::Subheader)
        );
        let _ = writeln!(
            out,
            "  subheader to first {}",
            distinct(self.subheader_first_gaps())
        );
        let _ = writeln!(out, "  row height {}", heights(PanelElementKind::Row));
        let _ = writeln!(out, "  note height {}", heights(PanelElementKind::Note));
        let _ = writeln!(out, "  section gap {}", distinct(section_gaps));
        // Text outside the headers: where its lines start, and the pitch of
        // the lines that start on the inset — a row's label, a note's first
        // line — read off the paint, which every row has whether or not it
        // published its bounds.
        let headers: Vec<Rect> = self
            .of_kind(PanelElementKind::Header)
            .map(|header| header.rect)
            .collect();
        let body: Vec<Rect> = self
            .texts
            .iter()
            .map(|(_, rect)| *rect)
            .filter(|rect| !headers.iter().any(|header| header.contains(rect.center())))
            .collect();
        let _ = writeln!(
            out,
            "  text left x {}",
            distinct(body.iter().map(|rect| rect.left()))
        );
        let mut column: Vec<f32> = body
            .iter()
            .filter(|rect| (rect.left() - inset).abs() <= 1.0)
            .map(|rect| rect.center().y)
            .collect();
        column.sort_by(f32::total_cmp);
        let pitches = column
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .filter(|pitch| (1.0..48.0).contains(pitch));
        let _ = writeln!(out, "  inset line pitch {}", distinct(pitches));
        let stray: Vec<String> = self
            .texts
            .iter()
            .filter(|(_, rect)| rect.left() < inset - 0.5)
            .map(|(text, rect)| format!("{:.1} {text:?}", rect.left()))
            .collect();
        if stray.is_empty() {
            let _ = writeln!(out, "  text left of {inset}: none");
        } else {
            let _ = writeln!(out, "  text left of {inset}: {}", stray.join("; "));
        }
        // Every element where it landed, so a column with two values can be
        // traced to the element that put the second one there.
        let _ = writeln!(out, "  elements: kind top..bottom left..right");
        for element in &self.elements {
            let _ = writeln!(
                out,
                "    {:<10} {:.1}..{:.1} {:.1}..{:.1}",
                format!("{:?}", element.kind),
                element.rect.top(),
                element.rect.bottom(),
                element.left(),
                element.right()
            );
        }
        out
    }
}

/// `values` rounded to a tenth, each distinct one once with how often it
/// occurs: `10 x7, 20 x2`.
fn distinct(values: impl IntoIterator<Item = f32>) -> String {
    let mut counts: BTreeMap<i64, usize> = BTreeMap::new();
    for value in values {
        *counts.entry((value * 10.0).round() as i64).or_default() += 1;
    }
    if counts.is_empty() {
        return "-".to_owned();
    }
    counts
        .into_iter()
        .map(|(tenths, count)| {
            let value = tenths as f32 / 10.0;
            if count == 1 {
                format!("{value}")
            } else {
                format!("{value} x{count}")
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn element_of(node: &Node, width: f32) -> Option<PanelElement> {
    let bounds = node.bounds()?;
    let rect = Rect::from_min_max(
        egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
        egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
    );
    if rect.width() < 1.0 || rect.height() < 1.0 {
        return None;
    }
    let labelled = node.label().is_some_and(|label| !label.is_empty());
    let kind = match node.role() {
        // The panel's own title is a level-2 heading; sections are level 3.
        Role::Heading if node.level() == Some(2) => return None,
        Role::Heading if node.level() == Some(4) => PanelElementKind::Subheader,
        Role::Heading => PanelElementKind::Header,
        Role::TextInput | Role::MultilineTextInput | Role::SearchInput => PanelElementKind::Field,
        Role::ComboBox => PanelElementKind::Select,
        Role::CheckBox | Role::Switch | Role::RadioButton => PanelElementKind::Toggle,
        Role::Button if rect.width() >= 0.9 * width => PanelElementKind::Disclosure,
        Role::Button => PanelElementKind::Button,
        Role::Row => PanelElementKind::Row,
        _ if node.value().is_some() && labelled => PanelElementKind::Row,
        // egui publishes a label's words as its value, not its label.
        Role::Label if labelled || node.value().is_some_and(|value| !value.is_empty()) => {
            PanelElementKind::Note
        }
        _ => return None,
    };
    Some(PanelElement {
        kind,
        rect,
        text: None,
    })
}

/// Full-width fills in a section band's colour.
fn band_rects(shapes: &[egui::epaint::ClippedShape], band: Color32, width: f32) -> Vec<Rect> {
    fn walk(shape: &Shape, band: Color32, width: f32, out: &mut Vec<Rect>) {
        match shape {
            Shape::Vec(shapes) => shapes
                .iter()
                .for_each(|shape| walk(shape, band, width, out)),
            Shape::Rect(rect)
                if rect.fill == band
                    && rect.rect.width() >= 0.8 * width
                    && (16.0..=48.0).contains(&rect.rect.height()) =>
            {
                out.push(rect.rect);
            }
            _ => {}
        }
    }
    let mut out = Vec::new();
    for shape in shapes {
        walk(&shape.shape, band, width, &mut out);
    }
    out
}

fn visible_texts(shapes: &[egui::epaint::ClippedShape]) -> Vec<(String, Rect)> {
    fn walk(shape: &Shape, clip: Rect, out: &mut Vec<(String, Rect)>) {
        match shape {
            Shape::Vec(shapes) => shapes.iter().for_each(|shape| walk(shape, clip, out)),
            Shape::Text(text) => {
                let color = text.override_text_color.unwrap_or_else(|| {
                    text.galley
                        .job
                        .sections
                        .iter()
                        .map(|section| section.format.color)
                        .find(|color| *color != Color32::PLACEHOLDER)
                        .unwrap_or(text.fallback_color)
                });
                let rect = Rect::from_min_size(text.pos, text.galley.size());
                if color.a() > 0
                    && text.opacity_factor > 0.0
                    && !text.galley.text().trim().is_empty()
                    && rect.intersects(clip)
                {
                    out.push((text.galley.text().to_owned(), rect));
                }
            }
            _ => {}
        }
    }
    let mut out = Vec::new();
    for shape in shapes {
        walk(&shape.shape, shape.clip_rect, &mut out);
    }
    out
}

/// Render `pass` at `size`, write it to `directory` as `<stem>.png` cropped to
/// what it painted, measure it, write the measurements beside it as
/// `<stem>.txt`, and return them.
pub fn write_panel_review(
    directory: &std::path::Path,
    stem: &str,
    theme: crate::Theme,
    size: egui::Vec2,
    inset: f32,
    mut pass: impl FnMut(&mut egui::Ui, Color32),
) -> String {
    let mut band = Color32::TRANSPARENT;
    let (canvas, output) = super::render_with_output(theme, size, None, true, |ui, background| {
        let fill = crate::tokens::Tokens::get(ui.ctx()).color.bg_panel_2;
        band = Color32::from_rgba_unmultiplied(fill.r(), fill.g(), fill.b(), 204);
        pass(ui, background);
    });
    let layout = PanelLayout::read(size.x, &output, band);
    let report = layout.report(stem, inset);
    let height = canvas.content_height().max(1);
    std::fs::create_dir_all(directory).expect("raster output directory");
    std::fs::write(directory.join(format!("{stem}.png")), canvas.png(height))
        .expect("write review render");
    std::fs::write(directory.join(format!("{stem}.txt")), &report)
        .expect("write review measurements");
    eprintln!("{report}");
    report
}
