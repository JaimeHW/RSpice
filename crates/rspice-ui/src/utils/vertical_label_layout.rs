//! Collision-aware label placement for vertical plot cursors.
//!
//! This helper places small callout labels associated with vertical lines
//! (cursors/markers) so labels avoid line and label collisions while preferring
//! top lanes and minimal anchor offset.

use egui::{Pos2, Rect, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelSide {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub struct VerticalLabelRequest {
    pub anchor_x: f32,
    pub size: Vec2,
}

#[derive(Debug, Clone, Copy)]
pub struct VerticalLabelPlacement {
    pub rect: Rect,
    pub side: LabelSide,
}

#[derive(Debug, Clone, Copy)]
pub struct VerticalLabelLayoutConfig {
    /// Minimum separation between label body and any vertical cursor line.
    pub line_clearance: f32,
    /// Distance from plot top to first label row.
    pub top_margin: f32,
    /// Vertical spacing between label rows.
    pub row_gap: f32,
    /// Number of top rows to prefer before searching lower rows.
    pub preferred_rows: usize,
    /// Horizontal step for trying farther placements.
    pub nudge_step: f32,
    /// Number of horizontal nudge attempts per side.
    pub nudge_steps: usize,
    /// Minimum gap between adjacent labels.
    pub label_gap: f32,
}

impl Default for VerticalLabelLayoutConfig {
    fn default() -> Self {
        Self {
            line_clearance: 4.0,
            top_margin: 2.0,
            row_gap: 3.0,
            preferred_rows: 4,
            nudge_step: 8.0,
            nudge_steps: 8,
            label_gap: 2.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Candidate {
    placement: VerticalLabelPlacement,
    score: f32,
}

/// Place labels for vertical cursor lines.
///
/// Hard constraints:
/// - Keep labels inside `plot_rect`.
/// - Avoid collisions with `line_x_positions`.
/// - Avoid overlap with previously placed labels.
///
/// Soft preference:
/// - Prefer low rows and small horizontal offsets.
/// - Prefer candidates that avoid `soft_obstacles` (e.g., top-band trace samples).
pub fn place_vertical_line_labels(
    plot_rect: Rect,
    requests: &[VerticalLabelRequest],
    line_x_positions: &[f32],
    soft_obstacles: &[Rect],
    config: VerticalLabelLayoutConfig,
) -> Vec<VerticalLabelPlacement> {
    if requests.is_empty() || !plot_rect.is_positive() {
        return Vec::new();
    }

    let mut placements = Vec::with_capacity(requests.len());
    let mut placed_rects = Vec::with_capacity(requests.len());

    for request in requests {
        let placement = place_single_label(
            plot_rect,
            *request,
            line_x_positions,
            soft_obstacles,
            &placed_rects,
            config,
        );
        placed_rects.push(placement.rect);
        placements.push(placement);
    }

    placements
}

fn place_single_label(
    plot_rect: Rect,
    request: VerticalLabelRequest,
    line_x_positions: &[f32],
    soft_obstacles: &[Rect],
    placed_rects: &[Rect],
    config: VerticalLabelLayoutConfig,
) -> VerticalLabelPlacement {
    let valid_size = Vec2::new(request.size.x.max(1.0), request.size.y.max(1.0));
    let max_rows = max_rows_for_height(plot_rect, valid_size.y, config.top_margin, config.row_gap);

    if max_rows == 0 {
        let rect = Rect::from_min_size(
            Pos2::new(plot_rect.min.x, plot_rect.min.y),
            Vec2::new(
                valid_size.x.min(plot_rect.width()),
                valid_size.y.min(plot_rect.height()),
            ),
        );
        return VerticalLabelPlacement {
            rect,
            side: LabelSide::Right,
        };
    }

    let preferred_rows = config.preferred_rows.max(1).min(max_rows);

    let row_ranges = [
        (0, preferred_rows),
        (preferred_rows, max_rows),
        (0, max_rows), // final fallback with relaxed line constraints
    ];

    for (pass_idx, (row_start, row_end)) in row_ranges.iter().enumerate() {
        let relax_line_constraint = pass_idx == 2;
        if *row_start >= *row_end {
            continue;
        }
        if let Some(candidate) = find_best_candidate(
            plot_rect,
            request.anchor_x,
            valid_size,
            *row_start,
            *row_end,
            line_x_positions,
            soft_obstacles,
            placed_rects,
            config,
            relax_line_constraint,
        ) {
            return candidate.placement;
        }
    }

    // Last-resort clamp: top-right of anchor, inside plot.
    let mut left = request.anchor_x + config.line_clearance;
    left = left.clamp(
        plot_rect.min.x,
        (plot_rect.max.x - valid_size.x).max(plot_rect.min.x),
    );
    let rect = Rect::from_min_size(
        Pos2::new(left, plot_rect.min.y + config.top_margin),
        valid_size,
    );
    VerticalLabelPlacement {
        rect,
        side: LabelSide::Right,
    }
}

#[allow(clippy::too_many_arguments)]
fn find_best_candidate(
    plot_rect: Rect,
    anchor_x: f32,
    size: Vec2,
    row_start: usize,
    row_end: usize,
    line_x_positions: &[f32],
    soft_obstacles: &[Rect],
    placed_rects: &[Rect],
    config: VerticalLabelLayoutConfig,
    relax_line_constraint: bool,
) -> Option<Candidate> {
    let mut best: Option<Candidate> = None;

    for row in row_start..row_end {
        let y = plot_rect.min.y + config.top_margin + row as f32 * (size.y + config.row_gap);
        if y + size.y > plot_rect.max.y {
            break;
        }

        for side in [LabelSide::Right, LabelSide::Left] {
            for nudge_step in 0..=config.nudge_steps {
                let nudge = nudge_step as f32 * config.nudge_step;
                let raw_left = match side {
                    LabelSide::Right => anchor_x + config.line_clearance + nudge,
                    LabelSide::Left => anchor_x - config.line_clearance - nudge - size.x,
                };
                let left = raw_left.clamp(
                    plot_rect.min.x,
                    (plot_rect.max.x - size.x).max(plot_rect.min.x),
                );
                let rect = Rect::from_min_size(Pos2::new(left, y), size);

                if rect.max.y > plot_rect.max.y || rect.min.y < plot_rect.min.y {
                    continue;
                }

                if !relax_line_constraint
                    && intersects_any_vertical_line(
                        rect,
                        line_x_positions,
                        config.line_clearance * 0.5,
                    )
                {
                    continue;
                }

                if overlaps_any_rect(rect, placed_rects, config.label_gap) {
                    continue;
                }

                let soft_area = total_overlap_area(rect, soft_obstacles);
                let mut score = row as f32 * 100.0 + nudge * 0.5;
                // Strongly prefer avoiding trace/harmonic obstacles if possible.
                if soft_area > 0.01 {
                    score += 250.0 + soft_area;
                }
                // Prefer labels staying near their anchor line.
                score += (rect.center().x - anchor_x).abs() * 0.08;

                let candidate = Candidate {
                    placement: VerticalLabelPlacement { rect, side },
                    score,
                };
                if best.is_none_or(|current| candidate.score < current.score) {
                    best = Some(candidate);
                }
            }
        }
    }

    best
}

fn max_rows_for_height(plot_rect: Rect, label_height: f32, top_margin: f32, row_gap: f32) -> usize {
    let usable_height = (plot_rect.height() - top_margin).max(0.0);
    if usable_height <= 0.0 || label_height <= 0.0 {
        return 0;
    }
    ((usable_height + row_gap) / (label_height + row_gap)).floor() as usize
}

fn overlaps_any_rect(rect: Rect, occupied: &[Rect], gap: f32) -> bool {
    occupied
        .iter()
        .any(|occ| rects_overlap_with_gap(rect, *occ, gap))
}

fn rects_overlap_with_gap(a: Rect, b: Rect, gap: f32) -> bool {
    a.max.x + gap > b.min.x
        && a.min.x < b.max.x + gap
        && a.max.y + gap > b.min.y
        && a.min.y < b.max.y + gap
}

fn intersects_any_vertical_line(rect: Rect, lines: &[f32], clearance: f32) -> bool {
    lines
        .iter()
        .copied()
        .filter(|x| x.is_finite())
        .any(|x| x >= rect.min.x - clearance && x <= rect.max.x + clearance)
}

fn total_overlap_area(rect: Rect, obstacles: &[Rect]) -> f32 {
    obstacles
        .iter()
        .copied()
        .map(|other| overlap_area(rect, other))
        .sum()
}

fn overlap_area(a: Rect, b: Rect) -> f32 {
    let min_x = a.min.x.max(b.min.x);
    let max_x = a.max.x.min(b.max.x);
    let min_y = a.min.y.max(b.min.y);
    let max_y = a.max.y.min(b.max.y);
    if max_x <= min_x || max_y <= min_y {
        0.0
    } else {
        (max_x - min_x) * (max_y - min_y)
    }
}
