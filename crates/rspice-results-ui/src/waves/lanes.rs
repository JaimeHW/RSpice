//! The logic and bus lanes a strip shows above its panes: which rows, in what
//! order, read how.
//!
//! The rows follow from the run and the reader. A `D()` column the reader
//! plots is a logic lane. A bus is one lane once every member is plotted, its
//! members under it rather than beside it: a declared bus by the run's
//! declaration, a user bus by the reader's grouping. A plotted `E()` column
//! is a step trace in the pane of its unit. [`EventLaneLayout`] holds what
//! the run cannot say: the order, each bus's radix and expansion, the
//! reader's buses and the selection. The host keeps one per strip, and for a
//! project result document keeps it in the document.

use super::logic::{
    BUS_ROW_HEIGHT, BusLane, BusOrigin, BusRadix, EventLane, EventLanes, LOGIC_ROW_HEIGHT, LaneRow,
    LaneStackResponse, real_step_trace,
};
use super::pane::ResolvedExpr;
use super::readout::{READOUT_ABSENT, ReadoutRow};
use super::{LaneTraceKind, StripModel, UnitPane, fmt_in_unit_readout, stable_hash};
use crate::derived::DerivedSeries;
use rspice_ui_kit::plot::CursorPair;
use std::collections::{BTreeMap, HashSet};
use std::sync::Arc;

/// A lane row's identity in a strip's layout.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LaneKey {
    /// A scalar net's logic lane, by its `D()` column.
    Signal(String),
    /// A bus by its label: a declared `name[msb:lsb]`, or the reader's name.
    Bus(String),
}

/// How the reader reads one bus.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BusView {
    pub radix: BusRadix,
    pub expanded: bool,
}

impl Default for BusView {
    /// A bus word reads in hexadecimal until the reader picks another
    /// radix, as the reference viewers start one.
    fn default() -> Self {
        Self {
            radix: BusRadix::Hex,
            expanded: false,
        }
    }
}

/// Logic lanes the reader grouped into one word.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserLaneBus {
    pub name: String,
    /// The members' `D()` columns, MSB first.
    pub members: Vec<String>,
}

/// What a strip's lane stack shows beyond what the run states.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EventLaneLayout {
    /// The rows top to bottom, as last drawn. A row the strip no longer shows
    /// is skipped; one it newly shows joins at the bottom.
    pub order: Vec<LaneKey>,
    /// Radix and expansion per bus label, declared and user buses alike.
    pub buses: BTreeMap<String, BusView>,
    /// The reader's groupings, in the order they were made.
    pub user_buses: Vec<UserLaneBus>,
    /// The selected rows; the first is the one the cursor steps along.
    pub selected: Vec<LaneKey>,
}

impl EventLaneLayout {
    /// How the reader reads the bus `label`.
    pub fn bus_view(&self, label: &str) -> BusView {
        self.buses.get(label).copied().unwrap_or_default()
    }
}

/// One member row of a bus lane.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaneMember {
    /// The member's `D()` column.
    pub signal: String,
    pub net: String,
}

/// One row a strip's lane stack draws, before its history is borrowed.
#[derive(Debug, Clone, PartialEq)]
pub enum LaneSpec {
    Logic {
        /// The `D()` column the reader plotted.
        signal: String,
        net: String,
        color: egui::Color32,
    },
    Bus {
        label: String,
        origin: BusOrigin,
        /// MSB first.
        members: Vec<LaneMember>,
        view: BusView,
        color: egui::Color32,
    },
}

impl LaneSpec {
    pub fn key(&self) -> LaneKey {
        match self {
            Self::Logic { signal, .. } => LaneKey::Signal(signal.clone()),
            Self::Bus { label, .. } => LaneKey::Bus(label.clone()),
        }
    }

    /// The row's name as its gutter and the cursor table print it.
    pub fn label(&self) -> &str {
        match self {
            Self::Logic { net, .. } => net,
            Self::Bus { label, .. } => label,
        }
    }

    pub fn color(&self) -> egui::Color32 {
        match self {
            Self::Logic { color, .. } | Self::Bus { color, .. } => *color,
        }
    }

    /// Height of the row with any member rows it shows.
    pub fn height(&self) -> f32 {
        match self {
            Self::Logic { .. } => LOGIC_ROW_HEIGHT,
            Self::Bus { members, view, .. } => {
                let shown = if view.expanded { members.len() } else { 0 };
                (shown as f32).mul_add(LOGIC_ROW_HEIGHT, BUS_ROW_HEIGHT)
            }
        }
    }
}

/// The rows a strip's lane stack shows, top to bottom.
pub fn lane_specs(model: &StripModel, layout: Option<&EventLaneLayout>) -> Vec<LaneSpec> {
    let Some(lanes) = model.event_lanes.as_deref() else {
        return Vec::new();
    };
    let plotted: Vec<_> = model
        .lane_traces
        .iter()
        .filter(|lane| lane.visible && lane.kind == LaneTraceKind::Logic)
        .collect();
    let member = |net: &str| {
        plotted
            .iter()
            .find(|lane| lane.net == net)
            .map(|lane| LaneMember {
                signal: lane.source_waveform_name.clone(),
                net: lane.net.clone(),
            })
    };
    // A bus is drawn in its MSB's colour.
    let color_of = |members: &[LaneMember]| {
        members
            .first()
            .and_then(|first| plotted.iter().find(|lane| lane.net == first.net))
            .map(|lane| lane.color)
    };
    let mut buses: Vec<LaneSpec> = Vec::new();
    let push_bus = |buses: &mut Vec<LaneSpec>, label: &str, origin, members: Vec<LaneMember>| {
        let Some(color) = color_of(&members) else {
            return;
        };
        if buses.iter().any(|bus| bus.label() == label) {
            return;
        }
        buses.push(LaneSpec::Bus {
            label: label.to_owned(),
            origin,
            color,
            view: layout.map_or_else(BusView::default, |layout| layout.bus_view(label)),
            members,
        });
    };
    for user in layout.map_or(&[][..], |layout| layout.user_buses.as_slice()) {
        let members = user
            .members
            .iter()
            .map(|signal| {
                plotted
                    .iter()
                    .find(|lane| lane.source_waveform_name == *signal)
                    .and_then(|lane| member(&lane.net))
            })
            .collect::<Option<Vec<_>>>();
        if let Some(members) = members {
            push_bus(&mut buses, &user.name, BusOrigin::User, members);
        }
    }
    // A declared bus too large to reassemble shows its members on their own,
    // as the Events sheet does.
    for bus in lanes.buses.iter().filter(|bus| bus.refusal().is_none()) {
        let members = bus
            .members()
            .iter()
            .map(|net| member(net))
            .collect::<Option<Vec<_>>>();
        if let Some(members) = members {
            push_bus(&mut buses, bus.name(), BusOrigin::Declared, members);
        }
    }
    let grouped: HashSet<&str> = buses
        .iter()
        .flat_map(|bus| match bus {
            LaneSpec::Bus { members, .. } => members.iter().map(|member| member.net.as_str()),
            LaneSpec::Logic { .. } => unreachable!("only buses are collected here"),
        })
        .collect();
    // The natural order: the columns' own, a bus where its first plotted
    // member falls.
    let mut natural = Vec::new();
    let mut placed = vec![false; buses.len()];
    for lane in &plotted {
        for (index, bus) in buses.iter().enumerate() {
            if !placed[index]
                && matches!(bus, LaneSpec::Bus { members, .. }
                    if members.iter().any(|member| member.net == lane.net))
            {
                placed[index] = true;
                natural.push(bus.clone());
            }
        }
        if !grouped.contains(lane.net.as_str()) {
            natural.push(LaneSpec::Logic {
                signal: lane.source_waveform_name.clone(),
                net: lane.net.clone(),
                color: lane.color,
            });
        }
    }
    let Some(layout) = layout else {
        return natural;
    };
    let mut ordered = Vec::with_capacity(natural.len());
    for key in &layout.order {
        if let Some(index) = natural.iter().position(|spec| spec.key() == *key) {
            ordered.push(natural.remove(index));
        }
    }
    ordered.extend(natural);
    ordered
}

/// Height of a lane stack of `specs`.
pub fn stack_height(specs: &[LaneSpec]) -> f32 {
    specs.iter().map(LaneSpec::height).sum()
}

/// Each user bus row's word, reassembled once per data version.
pub fn user_bus_lanes(
    derived: &mut DerivedSeries,
    model: &StripModel,
    specs: &[LaneSpec],
) -> Vec<Option<Arc<BusLane>>> {
    let Some(lanes) = model.event_lanes.as_deref() else {
        return vec![None; specs.len()];
    };
    specs
        .iter()
        .map(|spec| match spec {
            LaneSpec::Bus {
                label,
                origin: BusOrigin::User,
                members,
                ..
            } => {
                let nets: Vec<&str> = members.iter().map(|member| member.net.as_str()).collect();
                let key = stable_hash(&(model.analysis_key, label, &nets));
                // Every member is a plotted logic lane of this run, which is
                // what made the row, so the word always reassembles.
                Some(derived.bus_lane_or(key, || {
                    lanes
                        .user_bus(label, &nets)
                        .unwrap_or_else(|| BusLane::from_logic_lanes(label.clone(), &[]))
                }))
            }
            _ => None,
        })
        .collect()
}

/// The widget rows of `specs`, borrowing the run's lanes and the user buses
/// [`user_bus_lanes`] reassembled. A row whose history is gone is left out.
pub fn lane_rows<'a>(
    lanes: &'a EventLanes,
    specs: &'a [LaneSpec],
    user: &'a [Option<Arc<BusLane>>],
) -> Vec<LaneRow<'a>> {
    specs
        .iter()
        .zip(user)
        .filter_map(|(spec, user)| match spec {
            LaneSpec::Logic { net, color, .. } => lanes.logic(net).map(|lane| LaneRow::Logic {
                lane,
                color: *color,
            }),
            LaneSpec::Bus {
                label,
                origin,
                members,
                view,
                color,
            } => {
                let lane = match origin {
                    BusOrigin::Declared => lanes.bus(label)?,
                    BusOrigin::User => user.as_deref()?,
                };
                Some(LaneRow::Bus {
                    lane,
                    radix: view.radix,
                    expanded: view.expanded,
                    members: members
                        .iter()
                        .filter_map(|member| lanes.logic(&member.net))
                        .collect(),
                    color: *color,
                })
            }
        })
        .collect()
}

/// The `E()` lanes plotted in `pane`, as step traces on its unit axis
/// through the strip's last instant.
pub fn real_step_traces(model: &StripModel, pane: &UnitPane) -> Vec<ResolvedExpr> {
    let Some(lanes) = model.event_lanes.as_deref() else {
        return Vec::new();
    };
    model
        .lane_traces
        .iter()
        .filter(|lane| {
            lane.visible && lane.kind == LaneTraceKind::Real && model.lane_unit(lane) == pane.unit
        })
        .filter_map(|lane| {
            let real = lanes.real(&lane.net)?;
            let end = model
                .x_range
                .map(|range| range.1)
                .or_else(|| lane.x.last().copied())?;
            let key = stable_hash(&(model.analysis_key, &lane.source_waveform_name, "step"));
            Some(real_step_trace(real, end, lane.color, key))
        })
        .collect()
}

/// The cursor table's rows for a strip's lanes: what each logic and bus
/// lane reads at A and at B, and each `E()` lane's value, difference and
/// unit. A level or a word has no difference or slope, so those cells say
/// so.
pub fn lane_readout_rows(
    model: &StripModel,
    specs: &[LaneSpec],
    user: &[Option<Arc<BusLane>>],
    cursors: CursorPair,
    significant_digits: usize,
) -> Vec<ReadoutRow> {
    let Some(lanes) = model.event_lanes.as_deref() else {
        return Vec::new();
    };
    let read = |lane: EventLane<'_>, at: Option<f64>| {
        at.and_then(|t| lane.readout_at(t, significant_digits))
            .map_or_else(|| READOUT_ABSENT.to_owned(), |readout| readout.text)
    };
    let rows = lane_rows(lanes, specs, user);
    let mut out: Vec<ReadoutRow> = specs
        .iter()
        .zip(&rows)
        .map(|(spec, row)| {
            let lane = row.event_lane();
            ReadoutRow {
                name: spec.label().to_owned(),
                swatch: spec.color(),
                swatch_style: None,
                a: read(lane, cursors.a),
                b: read(lane, cursors.b),
                delta: READOUT_ABSENT.to_owned(),
                slope: READOUT_ABSENT.to_owned(),
            }
        })
        .collect();
    for lane in model
        .lane_traces
        .iter()
        .filter(|lane| lane.visible && lane.kind == LaneTraceKind::Real)
    {
        let Some(real) = lanes.real(&lane.net) else {
            continue;
        };
        let unit = model.lane_unit(lane);
        let value = |at: Option<f64>| at.and_then(|t| real.value_at(t));
        let format = |value: Option<f64>| {
            value.map_or_else(
                || READOUT_ABSENT.to_owned(),
                |value| fmt_in_unit_readout(value, unit, significant_digits),
            )
        };
        let (a, b) = (value(cursors.a), value(cursors.b));
        out.push(ReadoutRow {
            name: lane.source_waveform_name.clone(),
            swatch: lane.color,
            swatch_style: None,
            a: format(a),
            b: format(b),
            delta: format(a.zip(b).map(|(a, b)| b - a)),
            slope: READOUT_ABSENT.to_owned(),
        });
    }
    out
}

/// What applying a lane stack's response asks of the host beyond the layout.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LaneRequests {
    /// Move the stepping cursor to this exact event instant.
    pub cursor_to: Option<f64>,
    /// Cross-probe this `D()` signal.
    pub probe: Option<super::logic::LaneProbe>,
}

/// Apply what the reader asked of the lane stack to `layout`, whose rows
/// `specs` are as drawn, and hand back what only the host can do.
pub fn apply_response(
    layout: &mut EventLaneLayout,
    specs: &[LaneSpec],
    response: LaneStackResponse,
) -> LaneRequests {
    let key = |index: usize| specs.get(index).map(LaneSpec::key);
    if let Some((index, extend)) = response.select
        && let Some(key) = key(index)
    {
        if extend {
            if let Some(held) = layout.selected.iter().position(|held| *held == key) {
                layout.selected.remove(held);
            } else {
                layout.selected.push(key);
            }
        } else {
            layout.selected = vec![key];
        }
    }
    if let Some(index) = response.toggle_expanded
        && let Some(LaneSpec::Bus { label, .. }) = specs.get(index)
    {
        let view = layout.bus_view(label);
        layout.buses.insert(
            label.clone(),
            BusView {
                expanded: !view.expanded,
                ..view
            },
        );
    }
    if let Some((index, radix)) = response.set_radix
        && let Some(LaneSpec::Bus { label, .. }) = specs.get(index)
    {
        let view = layout.bus_view(label);
        layout
            .buses
            .insert(label.clone(), BusView { radix, ..view });
    }
    if let Some(indices) = response.group {
        let members: Vec<String> = indices
            .iter()
            .filter_map(|index| match specs.get(*index) {
                Some(LaneSpec::Logic { signal, .. }) => Some(signal.clone()),
                _ => None,
            })
            .collect();
        if members.len() >= 2 {
            let name = (1..)
                .map(|n| format!("bus{n}"))
                .find(|name| {
                    !layout.user_buses.iter().any(|bus| bus.name == *name)
                        && !specs.iter().any(|spec| spec.label() == name)
                })
                .expect("an unbounded sequence has a free name");
            layout.selected = vec![LaneKey::Bus(name.clone())];
            layout.user_buses.push(UserLaneBus { name, members });
        }
    }
    if let Some(index) = response.ungroup
        && let Some(LaneSpec::Bus {
            label,
            origin: BusOrigin::User,
            ..
        }) = specs.get(index)
    {
        layout.user_buses.retain(|bus| bus.name != *label);
        layout.buses.remove(label);
        layout
            .selected
            .retain(|held| *held != LaneKey::Bus(label.clone()));
    }
    if let Some((index, down)) = response.move_row {
        let mut keys: Vec<LaneKey> = specs.iter().map(LaneSpec::key).collect();
        let to = if down {
            index + 1
        } else {
            index.wrapping_sub(1)
        };
        if index < keys.len() && to < keys.len() {
            keys.swap(index, to);
            layout.order = keys;
        }
    }
    LaneRequests {
        cursor_to: response.cursor_to,
        probe: response.probe,
    }
}

/// Bring `layout` in line with the rows drawn: the order is theirs, and a
/// selection of a row no longer shown is dropped.
pub fn reconcile(layout: &mut EventLaneLayout, specs: &[LaneSpec]) {
    let keys: Vec<LaneKey> = specs.iter().map(LaneSpec::key).collect();
    layout.selected.retain(|held| keys.contains(held));
    if layout.order != keys {
        layout.order = keys;
    }
}

/// The row indices of `layout`'s selection among `specs`, first first.
pub fn selected_rows(layout: &EventLaneLayout, specs: &[LaneSpec]) -> Vec<usize> {
    layout
        .selected
        .iter()
        .filter_map(|key| specs.iter().position(|spec| spec.key() == *key))
        .collect()
}

#[cfg(test)]
mod tests;
