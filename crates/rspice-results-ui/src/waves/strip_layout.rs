//! The reader's arrangement of an analysis' waveform stack into strips: what
//! each strip menu entry and each drag in the stack makes of the strips the
//! view keeps, or why it cannot.
//!
//! An analysis the reader never arranged keeps no strips and gives each unit
//! a strip of its own; the first edit writes every strip down as drawn. A
//! strip holds up to four Y axes, each reading in one unit, as ViVA's strips
//! do; ADS stacks its plots the same way, one axis per unit on a plot.

use rspice_results::visualization_document::{MAX_STRIP_AXES, Strip, StripAxis, StripMember};

use super::{StripModel, UnitPane};

/// A curve of the stack: a signal's trace, by its index in the strip model,
/// or an expression, by its place in the expression list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Curve {
    Trace(usize),
    Expression(usize),
}

impl Curve {
    /// What a strip lists the curve by.
    pub fn member(self, model: &StripModel) -> Option<StripMember> {
        match self {
            Self::Trace(index) => model.trace_member(index),
            Self::Expression(slot) => model.expression_member(slot),
        }
    }
}

/// Where a curve is drawn: the strip, top to bottom, and its axis there.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Place {
    pub strip: usize,
    pub axis: usize,
}

/// Why a strip cannot take a curve of another unit.
fn four_axes(unit: &str) -> String {
    let unit = if unit.is_empty() {
        "plain-number"
    } else {
        unit
    };
    format!(
        "A strip reads against four Y axes at most. Move a curve off it \
         first, or give the {unit} curve a strip of its own."
    )
}

/// The unit a pane's first axis reads in, as a strip keeps it.
fn first_axis_unit<'a>(pane: &UnitPane<'a>) -> &'a str {
    if pane.unlabeled { pane.unit } else { pane.axis }
}

/// The members a strip lists for `traces` and `expressions`, each once:
/// a family's traces and a run overlay share their signal's.
fn members(model: &StripModel, traces: &[usize], expressions: &[usize]) -> Vec<StripMember> {
    let mut listed: Vec<StripMember> = Vec::new();
    let all = traces
        .iter()
        .filter_map(|&index| model.trace_member(index))
        .chain(
            expressions
                .iter()
                .filter_map(|&slot| model.expression_member(slot)),
        );
    for member in all {
        if !listed.contains(&member) {
            listed.push(member);
        }
    }
    listed
}

/// A key no strip in `used` has yet, which it then holds.
fn fresh_key(used: &mut Vec<String>) -> String {
    let key = (1..)
        .map(|serial| format!("strip-{serial}"))
        .find(|key| !used.contains(key))
        .expect("an unused strip key");
    used.push(key.clone());
    key
}

/// The strips as the stack draws them now, written down: the reader's own,
/// and a strip for each unit that has none. A curve an arranged strip lists
/// but draws nowhere now, a hidden one, stays listed where it was, and a
/// strip whose every curve is hidden keeps its place.
pub fn layout(model: &StripModel) -> Vec<Strip> {
    written(model).0
}

/// The strips written down, and where each drawn strip is among them.
fn written(model: &StripModel) -> (Vec<Strip>, Vec<usize>) {
    let panes = model.unit_panes();
    let mut used: Vec<String> = model
        .arrangement
        .iter()
        .map(|strip| strip.key.clone())
        .collect();
    let mut drawn: Vec<StripMember> = Vec::new();
    for pane in &panes {
        let traces: Vec<usize> = pane.all_traces().collect();
        let expressions: Vec<usize> = pane.all_expressions().collect();
        drawn.extend(members(model, &traces, &expressions));
    }
    let undrawn = |held: &[StripMember]| -> Vec<StripMember> {
        held.iter()
            .filter(|member| !drawn.contains(member))
            .cloned()
            .collect()
    };
    let mut strips: Vec<Strip> = panes
        .iter()
        .map(|pane| {
            let stored = model
                .arrangement
                .iter()
                .find(|strip| pane.arranged && strip.key == pane.unit);
            let key = stored.map_or_else(|| fresh_key(&mut used), |strip| strip.key.clone());
            let mut axes = vec![StripAxis {
                unit: first_axis_unit(pane).to_owned(),
                members: members(model, &pane.traces, &pane.expressions),
            }];
            axes.extend(pane.secondary.iter().map(|axis| StripAxis {
                unit: axis.unit.to_owned(),
                members: members(model, &axis.traces, &axis.expressions),
            }));
            for held in stored.map_or(&[][..], |strip| strip.axes.as_slice()) {
                let Some(axis) = axes.iter_mut().find(|axis| axis.unit == held.unit) else {
                    continue;
                };
                for member in undrawn(&held.members) {
                    if !axis.members.contains(&member) {
                        axis.members.push(member);
                    }
                }
            }
            Strip {
                key,
                weight: stored.and_then(|strip| strip.weight),
                locked: pane.locked,
                axes,
            }
        })
        .collect();
    let mut of_pane: Vec<usize> = (0..strips.len()).collect();
    // A strip drawing nothing now goes back after the arranged strip before
    // it, so showing its curves again draws it where it was.
    for (position, stored) in model.arrangement.iter().enumerate() {
        if strips.iter().any(|strip| strip.key == stored.key) {
            continue;
        }
        let axes: Vec<StripAxis> = stored
            .axes
            .iter()
            .map(|axis| StripAxis {
                unit: axis.unit.clone(),
                members: undrawn(&axis.members),
            })
            .filter(|axis| !axis.members.is_empty())
            .collect();
        if axes.is_empty() {
            continue;
        }
        let at = model.arrangement[..position]
            .iter()
            .rev()
            .find_map(|before| strips.iter().position(|strip| strip.key == before.key))
            .map_or(0, |index| index + 1);
        strips.insert(
            at,
            Strip {
                axes,
                ..stored.clone()
            },
        );
        for index in &mut of_pane {
            if *index >= at {
                *index += 1;
            }
        }
    }
    (strips, of_pane)
}

/// Where `curve` is drawn now, on the first strip that draws it.
pub fn place_of(model: &StripModel, curve: Curve) -> Option<Place> {
    model
        .unit_panes()
        .iter()
        .enumerate()
        .find_map(|(strip, pane)| {
            let on = |traces: &[usize], expressions: &[usize]| match curve {
                Curve::Trace(index) => traces.contains(&index),
                Curve::Expression(slot) => expressions.contains(&slot),
            };
            if on(&pane.traces, &pane.expressions) {
                return Some(Place { strip, axis: 0 });
            }
            pane.secondary
                .iter()
                .position(|axis| on(&axis.traces, &axis.expressions))
                .map(|axis| Place {
                    strip,
                    axis: axis + 1,
                })
        })
}

/// The strips written down, for an edit to make something new of.
struct Arranging {
    strips: Vec<Strip>,
    used: Vec<String>,
    /// Where each strip the stack draws is among `strips`.
    of_pane: Vec<usize>,
}

impl Arranging {
    fn of(model: &StripModel) -> Self {
        let (strips, of_pane) = written(model);
        let used = strips.iter().map(|strip| strip.key.clone()).collect();
        Self {
            strips,
            used,
            of_pane,
        }
    }

    /// Where the drawn strip `pane` is among the strips written down.
    fn index(&self, pane: usize) -> Result<usize, String> {
        self.of_pane
            .get(pane)
            .copied()
            .ok_or_else(|| "That strip is no longer drawn.".to_owned())
    }

    /// Where a curve dropped above the drawn strip `pane` goes among the
    /// strips written down: past the last of them below the last one drawn.
    fn gap(&self, pane: usize) -> usize {
        self.of_pane.get(pane).copied().unwrap_or(self.strips.len())
    }

    fn new_strip(&mut self, unit: &str, members: Vec<StripMember>) -> Strip {
        Strip {
            key: fresh_key(&mut self.used),
            weight: None,
            locked: false,
            axes: vec![StripAxis {
                unit: unit.to_owned(),
                members,
            }],
        }
    }

    /// The strips with every empty axis and strip gone, or an empty list when
    /// that is exactly a strip per unit, as an unarranged analysis draws.
    fn finish(mut self) -> Vec<Strip> {
        for strip in &mut self.strips {
            strip.axes.retain(|axis| !axis.members.is_empty());
        }
        self.strips.retain(|strip| !strip.axes.is_empty());
        self.strips
    }
}

/// Take `member` off the axis `axis` of `strip`.
fn take(strip: &mut Strip, axis: usize, member: &StripMember) {
    if let Some(axis) = strip.axes.get_mut(axis) {
        axis.members.retain(|held| held != member);
    }
}

/// Put `member`, read in `unit`, on `strip`: on its first axis of the unit,
/// else on a new axis while it has fewer than four.
fn give(strip: &mut Strip, unit: &str, member: StripMember) -> Result<(), String> {
    if let Some(axis) = strip.axes.iter_mut().find(|axis| axis.unit == unit) {
        if !axis.members.contains(&member) {
            axis.members.push(member);
        }
        return Ok(());
    }
    let occupied = strip
        .axes
        .iter()
        .filter(|axis| !axis.members.is_empty())
        .count();
    if occupied >= MAX_STRIP_AXES {
        return Err(four_axes(unit));
    }
    strip.axes.push(StripAxis {
        unit: unit.to_owned(),
        members: vec![member],
    });
    Ok(())
}

/// The curve's member, the unit of the axis it reads against, and where it
/// is among the strips written down.
fn locate(
    model: &StripModel,
    arranging: &Arranging,
    curve: Curve,
) -> Result<(StripMember, String, Place), String> {
    let gone = || "That curve is no longer drawn.".to_owned();
    let member = curve.member(model).ok_or_else(gone)?;
    let drawn = place_of(model, curve).ok_or_else(gone)?;
    let place = Place {
        strip: arranging.index(drawn.strip)?,
        axis: drawn.axis,
    };
    let unit = arranging
        .strips
        .get(place.strip)
        .and_then(|strip| strip.axes.get(place.axis))
        .map(|axis| axis.unit.clone())
        .ok_or_else(gone)?;
    Ok((member, unit, place))
}

/// "Split strip": each curve of strip `index` on a strip of its own, where
/// the strip was.
pub fn split_strip(model: &StripModel, index: usize) -> Result<Vec<Strip>, String> {
    let mut arranging = Arranging::of(model);
    let index = arranging.index(index)?;
    let strip = arranging.strips[index].clone();
    let curves: Vec<(String, StripMember)> = strip
        .axes
        .iter()
        .flat_map(|axis| {
            axis.members
                .iter()
                .map(|member| (axis.unit.clone(), member.clone()))
        })
        .collect();
    if curves.len() < 2 {
        return Err("This strip draws a single curve.".to_owned());
    }
    let split: Vec<Strip> = curves
        .into_iter()
        .map(|(unit, member)| arranging.new_strip(&unit, vec![member]))
        .collect();
    arranging.strips.splice(index..=index, split);
    Ok(arranging.finish())
}

/// "Split all strips": every curve on a strip of its own, in the order the
/// strips draw them.
pub fn split_all_strips(model: &StripModel) -> Result<Vec<Strip>, String> {
    let arranging = Arranging::of(model);
    if arranging
        .strips
        .iter()
        .all(|strip| strip.members().count() < 2)
    {
        return Err("Every strip already draws a single curve.".to_owned());
    }
    let mut arranging = arranging;
    let strips = std::mem::take(&mut arranging.strips);
    for strip in strips {
        for axis in strip.axes {
            for member in axis.members {
                let split = arranging.new_strip(&axis.unit, vec![member]);
                arranging.strips.push(split);
            }
        }
    }
    Ok(arranging.finish())
}

/// "Strip by unit": a strip per unit, as an analysis the reader never
/// arranged draws.
pub fn strip_by_unit(model: &StripModel) -> Result<Vec<Strip>, String> {
    if model.arrangement.is_empty() {
        return Err("The strips already go one to a unit.".to_owned());
    }
    Ok(Vec::new())
}

/// "Strip by trace": a strip per signal, holding each form of it drawn, an
/// AC signal's magnitude and phase on two axes, and a strip per expression.
pub fn strip_by_trace(model: &StripModel) -> Result<Vec<Strip>, String> {
    let mut arranging = Arranging::of(model);
    let strips = std::mem::take(&mut arranging.strips);
    let mut by_signal: Vec<(Option<String>, Strip)> = Vec::new();
    for strip in strips {
        for axis in strip.axes {
            for member in axis.members {
                let signal = match &member {
                    StripMember::Signal { name, .. } => Some(name.clone()),
                    StripMember::Expression { .. } => None,
                };
                match by_signal
                    .iter_mut()
                    .find(|(held, _)| signal.is_some() && *held == signal)
                {
                    Some((_, strip)) => give(strip, &axis.unit, member)?,
                    None => {
                        let strip = arranging.new_strip(&axis.unit, vec![member]);
                        by_signal.push((signal, strip));
                    }
                }
            }
        }
    }
    arranging.strips = by_signal.into_iter().map(|(_, strip)| strip).collect();
    Ok(arranging.finish())
}

/// "Merge with strip above": strip `index`'s curves join the strip above
/// it, on its axes of their units, or on new ones while it has room.
pub fn merge_with_strip_above(model: &StripModel, index: usize) -> Result<Vec<Strip>, String> {
    if index == 0 {
        return Err("The top strip has no strip above it.".to_owned());
    }
    let mut arranging = Arranging::of(model);
    let above = arranging.index(index - 1)?;
    let index = arranging.index(index)?;
    let lower = arranging.strips[index].clone();
    let upper = &mut arranging.strips[above];
    for axis in lower.axes {
        for member in axis.members {
            give(upper, &axis.unit, member)?;
        }
    }
    arranging.strips.remove(index);
    Ok(arranging.finish())
}

/// "Combine all strips": every curve on one strip, on an axis per unit.
pub fn combine_all_strips(model: &StripModel) -> Result<Vec<Strip>, String> {
    let mut arranging = Arranging::of(model);
    if arranging.strips.len() < 2 {
        return Err("There is a single strip.".to_owned());
    }
    let rest = arranging.strips.split_off(1);
    let first = &mut arranging.strips[0];
    for strip in rest {
        for axis in strip.axes {
            for member in axis.members {
                give(first, &axis.unit, member)?;
            }
        }
    }
    Ok(arranging.finish())
}

/// "Move to new strip": `curve` off its strip onto a new one under it.
pub fn move_to_new_strip(model: &StripModel, curve: Curve) -> Result<Vec<Strip>, String> {
    let mut arranging = Arranging::of(model);
    let (member, unit, place) = locate(model, &arranging, curve)?;
    if arranging.strips[place.strip].members().count() < 2 {
        return Err("The curve already has a strip of its own.".to_owned());
    }
    take(&mut arranging.strips[place.strip], place.axis, &member);
    let strip = arranging.new_strip(&unit, vec![member]);
    arranging.strips.insert(place.strip + 1, strip);
    Ok(arranging.finish())
}

/// "Copy to new strip": `curve` also on a new strip under its own.
pub fn copy_to_new_strip(model: &StripModel, curve: Curve) -> Result<Vec<Strip>, String> {
    let mut arranging = Arranging::of(model);
    let (member, unit, place) = locate(model, &arranging, curve)?;
    let strip = arranging.new_strip(&unit, vec![member]);
    arranging.strips.insert(place.strip + 1, strip);
    Ok(arranging.finish())
}

/// A curve dropped between strips: off its strip onto a new one at `at`,
/// counted among the strips before the drop.
pub fn move_to_gap(model: &StripModel, curve: Curve, at: usize) -> Result<Vec<Strip>, String> {
    let mut arranging = Arranging::of(model);
    let (member, unit, place) = locate(model, &arranging, curve)?;
    let at = arranging.gap(at);
    let alone = arranging.strips[place.strip].members().count() < 2;
    if alone && (at == place.strip || at == place.strip + 1) {
        return Err("The curve already has a strip of its own there.".to_owned());
    }
    take(&mut arranging.strips[place.strip], place.axis, &member);
    let strip = arranging.new_strip(&unit, vec![member]);
    arranging.strips.insert(at, strip);
    Ok(arranging.finish())
}

/// A curve dropped on strip `to`: off its strip onto `to`'s axis of its
/// unit, or a new axis there while `to` has fewer than four.
pub fn move_to_strip(model: &StripModel, curve: Curve, to: usize) -> Result<Vec<Strip>, String> {
    let mut arranging = Arranging::of(model);
    let (member, unit, place) = locate(model, &arranging, curve)?;
    let to = arranging.index(to)?;
    if place.strip == to {
        return Err("The curve is on this strip already.".to_owned());
    }
    take(&mut arranging.strips[place.strip], place.axis, &member);
    give(&mut arranging.strips[to], &unit, member)?;
    Ok(arranging.finish())
}

/// "Move to new Y axis": `curve` read against an axis of its own on its
/// strip, scaled to it alone: a millivolt ripple beside its volt rail.
pub fn move_to_new_axis(model: &StripModel, curve: Curve) -> Result<Vec<Strip>, String> {
    let mut arranging = Arranging::of(model);
    let (member, unit, place) = locate(model, &arranging, curve)?;
    let strip = &mut arranging.strips[place.strip];
    if strip.axes[place.axis].members.len() < 2 {
        return Err("The curve already reads against an axis of its own.".to_owned());
    }
    if strip.axes.len() >= MAX_STRIP_AXES {
        return Err(four_axes(&unit));
    }
    take(strip, place.axis, &member);
    strip.axes.push(StripAxis {
        unit,
        members: vec![member],
    });
    Ok(arranging.finish())
}

/// "Move to axis": `curve` onto axis `axis` of its strip, which reads in
/// the curve's unit.
pub fn move_to_axis(model: &StripModel, curve: Curve, axis: usize) -> Result<Vec<Strip>, String> {
    let mut arranging = Arranging::of(model);
    let (member, unit, place) = locate(model, &arranging, curve)?;
    if place.axis == axis {
        return Err("The curve reads against that axis already.".to_owned());
    }
    let strip = &mut arranging.strips[place.strip];
    let Some(target) = strip.axes.get(axis) else {
        return Err("The strip has no such axis.".to_owned());
    };
    if target.unit != unit {
        return Err(format!(
            "That axis reads in {}; a curve reads against an axis of its own unit.",
            if target.unit.is_empty() {
                "plain numbers"
            } else {
                target.unit.as_str()
            }
        ));
    }
    take(strip, place.axis, &member);
    strip.axes[axis].members.push(member);
    Ok(arranging.finish())
}

/// A strip header dragged: strip `from` drawn at `to` instead.
pub fn reorder(model: &StripModel, from: usize, to: usize) -> Result<Vec<Strip>, String> {
    let mut arranging = Arranging::of(model);
    let from = arranging.index(from)?;
    let to = arranging.index(to)?;
    if from == to {
        return Err("The strip is there already.".to_owned());
    }
    let strip = arranging.strips.remove(from);
    arranging.strips.insert(to, strip);
    Ok(arranging.finish())
}

/// "Lock strip size": strip `index` keeps its height when its neighbours
/// are resized or the stack is equalised.
pub fn set_locked(model: &StripModel, index: usize, locked: bool) -> Result<Vec<Strip>, String> {
    let mut arranging = Arranging::of(model);
    let index = arranging.index(index)?;
    arranging.strips[index].locked = locked;
    Ok(arranging.finish())
}

/// The divider under strip `index` dragged: the two strips beside it share
/// their height as `upper` to `lower`, each kept above a sliver.
pub fn resize(
    model: &StripModel,
    index: usize,
    (upper, lower): (f32, f32),
) -> Result<Vec<Strip>, String> {
    let mut arranging = Arranging::of(model);
    let below = arranging.index(index + 1)?;
    let index = arranging.index(index)?;
    if arranging.strips[index].locked || arranging.strips[below].locked {
        return Err("A locked strip keeps its height. Unlock it first.".to_owned());
    }
    let share = |strip: &Strip| strip.weight.unwrap_or(1.0);
    let total = share(&arranging.strips[index]) + share(&arranging.strips[below]);
    let sum = upper + lower;
    if !(sum.is_finite() && sum > 0.0) {
        return Err("The strips need a height.".to_owned());
    }
    let floor = total * 0.05;
    let top = (total * upper / sum).clamp(floor, total - floor);
    arranging.strips[index].weight = Some(top);
    arranging.strips[below].weight = Some(total - top);
    Ok(arranging.finish())
}

/// A divider double-clicked: every unlocked strip at an equal height.
pub fn equalise(model: &StripModel) -> Result<Vec<Strip>, String> {
    let mut arranging = Arranging::of(model);
    if arranging
        .strips
        .iter()
        .all(|strip| strip.locked || strip.weight.is_none())
    {
        return Err("The strips are at an equal height already.".to_owned());
    }
    for strip in arranging.strips.iter_mut().filter(|strip| !strip.locked) {
        strip.weight = None;
    }
    Ok(arranging.finish())
}

#[cfg(test)]
#[path = "strip_layout_tests.rs"]
mod tests;
