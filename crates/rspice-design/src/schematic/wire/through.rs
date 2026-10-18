//! Which wires pass through a point, without asking every wire.

use std::collections::HashMap;

use rspice_design_model::Point;

use super::Wire;

/// Which wires pass through a point, answered from the segments' rows and
/// columns instead of by asking every wire.
///
/// A pass that asks this of many points would otherwise be the product of
/// the point count and the wire count. A wire with a slanted segment, or
/// with no segment at all, is asked directly.
pub(crate) struct WiresThrough<'a> {
    wires: &'a [Wire],
    /// Horizontal segments by row: the span they cover and their wire.
    rows: HashMap<i32, Vec<(i32, i32, usize)>>,
    /// Vertical segments by column, likewise.
    columns: HashMap<i32, Vec<(i32, i32, usize)>>,
    /// Wires the rows and columns cannot answer for.
    others: Vec<usize>,
}

impl<'a> WiresThrough<'a> {
    pub(crate) fn new(wires: &'a [Wire]) -> Self {
        let mut index = Self {
            wires,
            rows: HashMap::new(),
            columns: HashMap::new(),
            others: Vec::new(),
        };
        for (position, wire) in wires.iter().enumerate() {
            let square = wire.points.len() >= 2
                && wire
                    .points
                    .windows(2)
                    .all(|pair| pair[0].x == pair[1].x || pair[0].y == pair[1].y);
            if !square {
                index.others.push(position);
                continue;
            }
            for pair in wire.points.windows(2) {
                let (a, b) = (pair[0], pair[1]);
                if a.y == b.y {
                    index
                        .rows
                        .entry(a.y)
                        .or_default()
                        .push((a.x.min(b.x), a.x.max(b.x), position));
                } else {
                    index.columns.entry(a.x).or_default().push((
                        a.y.min(b.y),
                        a.y.max(b.y),
                        position,
                    ));
                }
            }
        }
        index
    }

    /// The positions in the wire list of every wire through `point`, in
    /// list order.
    pub(crate) fn indices(&self, point: Point) -> Vec<usize> {
        let mut found = Vec::new();
        spanning(&self.rows, point.y, point.x, &mut found);
        spanning(&self.columns, point.x, point.y, &mut found);
        found.extend(
            self.others
                .iter()
                .copied()
                .filter(|position| self.wires[*position].contains_point(point)),
        );
        found.sort_unstable();
        found.dedup();
        found
    }
}

/// Push the wire of every segment on `line` whose span covers `along`.
fn spanning(
    lines: &HashMap<i32, Vec<(i32, i32, usize)>>,
    line: i32,
    along: i32,
    found: &mut Vec<usize>,
) {
    if let Some(segments) = lines.get(&line) {
        found.extend(
            segments
                .iter()
                .filter(|(low, high, _)| (*low..=*high).contains(&along))
                .map(|(_, _, position)| *position),
        );
    }
}
