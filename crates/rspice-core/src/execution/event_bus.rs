//! Reassembling a digital bus out of the member histories that carry it.
//!
//! A bus is a declaration, not a recording: the engine commits state per node,
//! so a bus has no points of its own and its value at a time is each member's
//! held value in declaration order. Every route that shows a bus — a VCD
//! vector, a rawfile bus plot, an events sheet, a binding's accessor —
//! therefore has to perform the same reassembly, and this module is the one
//! place it is written. Two routes that reassembled a bus separately would be
//! two chances to disagree about what the run held.
//!
//! # Why event codes rather than values
//!
//! A member's value crosses this boundary as the `0..=12` code
//! [`crate::xspice::DigitalValue::event_code`] produces. That is the encoding
//! every consumer downstream already speaks — the typed document tags event
//! points with it, a rawfile event plot stores it as its value column, the
//! live sample hook publishes it, and the GUI's worker contract carries it —
//! so a caller holding any of those calls this function without first
//! converting to a type it may not be allowed to name.
//!
//! # What "not yet observed" means
//!
//! Members change at times of their own. At the first time any member of a bus
//! changes, another member may have no recorded point at all, and a code is
//! `None` until its member's first point. That is a bit whose value the run has
//! not stated, which is not the same as a bit that is zero; a caller decides
//! how to spell it, and a VCD dump spells it `x`.

use thiserror::Error;

use crate::Value;

/// The most member values one bus reassembly materializes.
///
/// A bus costs the *product* of its two sizes, not either one: [`bus_events`]
/// holds one code per member per event, so a 4,096-member declaration — the
/// widest [`crate::engine::MAX_DIGITAL_BUS_WIDTH`] admits — over the two
/// million events the bindings already cap a single node's history at would be
/// eight billion entries, tens of gigabytes, from a document a few megabytes
/// long. Neither existing bound catches that, because neither number is
/// unreasonable on its own.
///
/// The ceiling is that same two-million-row scale at a sixty-four-bit word:
/// the widest bus whose value still fits a machine register, and comfortably
/// wider than the 53 bits a table column holds exactly. It costs about 256 MB
/// of codes plus the per-event row headers, and it trades the two sizes off
/// against each other rather than fixing either — a two-bit bus keeps 64
/// million events, the widest bus keeps 31,250.
///
/// It is enforced inside [`bus_events`] so that every route that shows a bus —
/// the VCD projection, the rawfile bus plots, the Python accessor, the browser
/// handle — refuses the same document for the same reason and with the same
/// numbers. The bindings' own per-node row bounds sit above this and still
/// apply; this is the one bound that knows the width.
pub const MAX_BUS_EVENT_CELLS: usize = 128_000_000;

/// A digital bus whose reassembly would not fit [`MAX_BUS_EVENT_CELLS`].
///
/// This is a budget, not a defect in the data: the declaration is well formed
/// and every member history is sound, there is simply more of it than one
/// reassembly holds at once. A caller that needs such a bus reads its members
/// individually, or narrows the run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
#[error(
    "reassembling a digital bus of {width} member(s) would materialize at least {cells} member \
     values, past the {MAX_BUS_EVENT_CELLS} this build holds at once"
)]
pub struct BusReassemblyTooLarge {
    /// Members the bus declares.
    pub width: usize,
    /// Member values the reassembly needs — the exact product where the event
    /// count is known, and the recorded-change total where the refusal came
    /// before the event times were even collected. Never an overstatement.
    pub cells: usize,
}

/// One bus member's history, as the times and event codes it was recorded at.
///
/// The points must be in non-decreasing time order, which is the order every
/// producer in this crate records them in. Nothing here sorts them: a history
/// that ran backwards is a defect in whatever produced it, and silently
/// reordering one would hide it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BusMemberHistory<'a> {
    /// Accepted event times in seconds, each with the code committed at it.
    pub points: &'a [(Value, u8)],
}

impl<'a> BusMemberHistory<'a> {
    /// The code this member held at `time`, or `None` before its first point.
    ///
    /// "Held" is the last point at or before `time`: a digital history records
    /// only changes, so a member keeps its value until the next one.
    fn held_at(&self, time: Value) -> Option<u8> {
        let boundary = self
            .points
            .partition_point(|(point_time, _)| *point_time <= time);
        boundary
            .checked_sub(1)
            .and_then(|index| self.points.get(index))
            .map(|(_, code)| *code)
    }
}

/// Every time at which at least one member changes, with the whole bus at it.
///
/// The returned times are the union of the members' own event times, in
/// ascending order with no duplicates, and each carries one entry per member
/// in the order the members were given — which is declaration order, declared
/// MSB first. An entry is the code that member held at that time, or `None`
/// when the member has not been observed yet.
///
/// Two members changing at one accepted time produce one event, because they
/// did change at one time: the engine commits the whole event state at an
/// accepted point, so co-timed member changes are one change of the bus.
///
/// A bus with no members, or whose members recorded nothing, has no events.
///
/// # Refusal
///
/// A bus needing more than [`MAX_BUS_EVENT_CELLS`] member values is refused
/// rather than materialized. The check runs twice, and neither run overstates
/// what the bus costs: once on the recorded-change total, which is a lower
/// bound on the product because every recorded change lands in some event, and
/// so refuses before even the event times are collected; then once on the
/// exact product, before the codes are.
pub fn bus_events(
    members: &[BusMemberHistory<'_>],
) -> Result<Vec<(Value, Vec<Option<u8>>)>, BusReassemblyTooLarge> {
    let total = members
        .iter()
        .map(|member| member.points.len())
        .fold(0usize, usize::saturating_add);
    if total == 0 {
        return Ok(Vec::new());
    }
    // Every recorded change is one member value at one event, so the total is
    // never more than the product — refusing on it cannot refuse a bus that
    // would have fit, and it costs nothing to check before the first
    // allocation.
    if total > MAX_BUS_EVENT_CELLS {
        return Err(BusReassemblyTooLarge {
            width: members.len(),
            cells: total,
        });
    }

    let mut times: Vec<Value> = Vec::with_capacity(total);
    for member in members {
        times.extend(member.points.iter().map(|(time, _)| *time));
    }
    // `total_cmp` rather than `partial_cmp`: the times come from histories this
    // crate validated, but a total order is what makes the sort a sort at all,
    // and it costs nothing.
    times.sort_unstable_by(Value::total_cmp);
    times.dedup_by(|left, right| left.total_cmp(right).is_eq());

    // The event count is exact now, so this is the real cost, checked before
    // the first row is built.
    let cells = times.len().saturating_mul(members.len());
    if cells > MAX_BUS_EVENT_CELLS {
        return Err(BusReassemblyTooLarge {
            width: members.len(),
            cells,
        });
    }

    let mut cursors = vec![0usize; members.len()];
    let mut held: Vec<Option<u8>> = vec![None; members.len()];
    let mut events = Vec::with_capacity(times.len());
    for time in times {
        for ((member, cursor), slot) in members.iter().zip(cursors.iter_mut()).zip(held.iter_mut())
        {
            while let Some((point_time, code)) = member.points.get(*cursor) {
                if *point_time > time {
                    break;
                }
                *slot = Some(*code);
                *cursor = cursor.saturating_add(1);
            }
        }
        events.push((time, held.clone()));
    }
    Ok(events)
}

/// Split `name[msb:lsb]` into its name and the range it declares.
///
/// Core spells a bus in exactly one place per format that needs a single
/// field — a VCD `$var` reference and a rawfile bus plot's `Title:` — and both
/// take this grammar, so there is one thing to parse on the way back in. The
/// space form `name [msb:lsb]` is accepted too: it is what the reference field
/// of a `$var` holds after the reader joins its tokens, and what the common
/// Verilog dumpers write.
///
/// Only a bracketed pair separated by a colon is a range. `data[3]` is a
/// bit-select — the name of one conductor — and comes back whole, which is
/// what lets a member trace be named that way without being mistaken for a
/// one-bit vector declaration.
pub(crate) fn split_bus_notation(text: &str) -> (&str, Option<(i64, i64)>) {
    let trimmed = text.trim_end();
    let Some(open) = trimmed.rfind('[') else {
        return (text, None);
    };
    let Some(body) = trimmed[open + 1..].strip_suffix(']') else {
        return (text, None);
    };
    let Some((msb, lsb)) = body.split_once(':') else {
        return (text, None);
    };
    let (Ok(msb), Ok(lsb)) = (msb.trim().parse::<i64>(), lsb.trim().parse::<i64>()) else {
        return (text, None);
    };
    (trimmed[..open].trim_end(), Some((msb, lsb)))
}

/// The whole bus at one time, in the order the members were given.
///
/// Each entry is the code its member held at `time` — the last point at or
/// before it — or `None` when the member had not been observed by then. This
/// answers a single time without materializing the bus's whole timeline, which
/// is what a cursor, a probe or a table cell needs.
pub fn bus_value_at(members: &[BusMemberHistory<'_>], time: Value) -> Vec<Option<u8>> {
    members.iter().map(|member| member.held_at(time)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn history(points: &[(Value, u8)]) -> Vec<(Value, u8)> {
        points.to_vec()
    }

    /// Every bus in this module fits the cell ceiling by inspection, so the
    /// refusal is not what these tests are about.
    fn reassembled(members: &[BusMemberHistory<'_>]) -> Vec<(Value, Vec<Option<u8>>)> {
        bus_events(members).expect("a bus of a handful of events fits the ceiling")
    }

    #[test]
    fn a_time_where_one_member_changes_carries_the_others_held_values() {
        let high = history(&[(0.0, 0), (2e-9, 1)]);
        let low = history(&[(0.0, 1), (1e-9, 0), (3e-9, 1)]);
        let events = reassembled(&[
            BusMemberHistory { points: &high },
            BusMemberHistory { points: &low },
        ]);
        assert_eq!(
            events,
            vec![
                (0.0, vec![Some(0), Some(1)]),
                (1e-9, vec![Some(0), Some(0)]),
                (2e-9, vec![Some(1), Some(0)]),
                (3e-9, vec![Some(1), Some(1)]),
            ]
        );
    }

    #[test]
    fn co_timed_member_changes_are_one_bus_event() {
        // The engine commits the whole event state at an accepted point, so
        // eight bits that changed together changed at one time.
        let high = history(&[(0.0, 0), (5e-9, 1)]);
        let low = history(&[(0.0, 0), (5e-9, 1)]);
        let events = reassembled(&[
            BusMemberHistory { points: &high },
            BusMemberHistory { points: &low },
        ]);
        assert_eq!(
            events,
            vec![
                (0.0, vec![Some(0), Some(0)]),
                (5e-9, vec![Some(1), Some(1)]),
            ]
        );
    }

    #[test]
    fn a_member_with_no_point_yet_is_missing_rather_than_zero() {
        let early = history(&[(0.0, 1)]);
        let late = history(&[(4e-9, 0)]);
        let events = reassembled(&[
            BusMemberHistory { points: &early },
            BusMemberHistory { points: &late },
        ]);
        assert_eq!(
            events,
            vec![(0.0, vec![Some(1), None]), (4e-9, vec![Some(1), Some(0)]),],
            "a bit the run has not stated is not a bit the run stated as zero"
        );
    }

    #[test]
    fn a_bus_with_nothing_recorded_has_no_events() {
        assert!(reassembled(&[]).is_empty());
        let empty: Vec<(Value, u8)> = Vec::new();
        assert!(
            reassembled(&[
                BusMemberHistory { points: &empty },
                BusMemberHistory { points: &empty }
            ])
            .is_empty()
        );
    }

    #[test]
    fn a_value_at_a_time_holds_the_last_point_at_or_before_it() {
        let member = history(&[(1e-9, 3), (5e-9, 7)]);
        let members = [BusMemberHistory { points: &member }];
        assert_eq!(bus_value_at(&members, 0.0), vec![None]);
        assert_eq!(bus_value_at(&members, 1e-9), vec![Some(3)]);
        assert_eq!(bus_value_at(&members, 4e-9), vec![Some(3)]);
        assert_eq!(bus_value_at(&members, 5e-9), vec![Some(7)]);
        assert_eq!(bus_value_at(&members, 1.0), vec![Some(7)]);
    }

    #[test]
    fn the_two_entry_points_agree_at_every_event_time() {
        let a = history(&[(0.0, 0), (2e-9, 1), (6e-9, 12)]);
        let b = history(&[(1e-9, 4), (2e-9, 5)]);
        let c = history(&[(6e-9, 2)]);
        let members = [
            BusMemberHistory { points: &a },
            BusMemberHistory { points: &b },
            BusMemberHistory { points: &c },
        ];
        for (time, codes) in reassembled(&members) {
            assert_eq!(bus_value_at(&members, time), codes, "at {time}");
        }
    }

    /// The ceiling is a *product*, so the cheapest bus that crosses it is the
    /// widest one: 4,096 members need only 31,251 event times to want more
    /// than [`MAX_BUS_EVENT_CELLS`] member values, and the refusal comes
    /// before a single row is built — one member carrying every time is
    /// enough to trip it.
    ///
    /// The accepted side of the boundary is stated as arithmetic rather than
    /// materialized: one event fewer is 128,000,000 cells, a quarter of a
    /// gigabyte, which is the whole reason the ceiling exists and not
    /// something a unit test should allocate. What *is* exercised is that a
    /// full-width bus is not refused for being wide — 4,096 members over a
    /// thousand events pass — so the refusal is the product talking and not
    /// the width.
    #[test]
    fn a_bus_past_the_cell_ceiling_is_refused_by_number() {
        const WIDTH: usize = 4_096;
        const OVER: usize = 31_251;
        assert!(OVER * WIDTH > MAX_BUS_EVENT_CELLS);
        assert!((OVER - 1) * WIDTH <= MAX_BUS_EVENT_CELLS);

        // One member holds every time; the other 4,095 recorded nothing, so
        // the input is 31,251 points rather than 128 million.
        let carrier: Vec<(Value, u8)> = (0..OVER).map(|step| (step as Value * 1e-9, 1)).collect();
        let empty: Vec<(Value, u8)> = Vec::new();
        let mut members = vec![BusMemberHistory { points: &carrier }];
        members.resize(WIDTH, BusMemberHistory { points: &empty });

        let error = bus_events(&members).expect_err("31,251 events over 4,096 members is over");
        assert_eq!(
            error,
            BusReassemblyTooLarge {
                width: WIDTH,
                cells: OVER * WIDTH,
            }
        );
        let message = error.to_string();
        assert!(
            message.contains(&WIDTH.to_string())
                && message.contains(&(OVER * WIDTH).to_string())
                && message.contains(&MAX_BUS_EVENT_CELLS.to_string()),
            "the refusal must name the width, the cost and the ceiling: {message}"
        );

        // The same width, well inside the ceiling, reassembles.
        let short: Vec<(Value, u8)> = (0..1_000).map(|step| (step as Value * 1e-9, 1)).collect();
        let mut members = vec![BusMemberHistory { points: &short }];
        members.resize(WIDTH, BusMemberHistory { points: &empty });
        assert_eq!(
            bus_events(&members)
                .expect("a full-width bus is not refused for its width")
                .len(),
            1_000
        );
    }

    #[test]
    fn a_repeated_time_across_members_yields_one_event_with_both_changes() {
        // Two histories may share a time exactly; the union keeps one row.
        let a = history(&[(0.0, 0), (0.0, 1)]);
        let b = history(&[(0.0, 2)]);
        let events = reassembled(&[
            BusMemberHistory { points: &a },
            BusMemberHistory { points: &b },
        ]);
        assert_eq!(events, vec![(0.0, vec![Some(1), Some(2)])]);
    }
}

#[cfg(test)]
mod notation_tests {
    use super::split_bus_notation;

    #[test]
    fn a_range_is_split_and_a_bit_select_is_not() {
        assert_eq!(split_bus_notation("d[1:0]"), ("d", Some((1, 0))));
        assert_eq!(split_bus_notation("d [1:0]"), ("d", Some((1, 0))));
        assert_eq!(split_bus_notation("d[0:7]"), ("d", Some((0, 7))));
        assert_eq!(split_bus_notation("d[-1:-3]"), ("d", Some((-1, -3))));
        assert_eq!(split_bus_notation("d[3]"), ("d[3]", None));
        assert_eq!(split_bus_notation("plain"), ("plain", None));
        assert_eq!(split_bus_notation("d[a:b]"), ("d[a:b]", None));
        assert_eq!(
            split_bus_notation("x1.count[1:0]"),
            ("x1.count", Some((1, 0)))
        );
    }
}
