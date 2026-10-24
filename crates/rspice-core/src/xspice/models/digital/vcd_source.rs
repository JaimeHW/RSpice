//! `d_vcd`: a digital stimulus that replays a Value Change Dump.
//!
//! ```text
//! a1 [clk start d3 d2 d1 d0] vstim
//! .model vstim d_vcd (file="stim.vcd" signals="tb.clk tb.start tb.d[3:0]" toffset=0)
//! ```
//!
//! It is an ordinary XSPICE digital code model, so everything downstream of an
//! XSPICE event output applies to it unchanged: event nets, the mapping into
//! an HDL's strength values, the boundary converter on an electrical net, and
//! checkpoints. The stimulus sets no levels; a converter does.
//!
//! # Mapping
//!
//! `signals` names variables by their scoped name, as a waveform viewer shows
//! it (`tb.d`, or `tb.d[3:0]`; whitespace inside a declared name is ignored).
//! Each signal takes the next ports in order: a logic variable as many
//! consecutive elements of `out` as it is wide, most significant bit first,
//! and a `real` variable one element of `real_out`. A width that disagrees
//! with the file, a logic variable that would land on `real_out`, a real one
//! that would land on `out`, and a count that does not fill the connected
//! ports are each refused, naming the signal. Two names for one identifier
//! code are one timeline.
//!
//! # Values and time
//!
//! `0`, `1` and `x` drive strong 0, 1 and X, and `z` drives high impedance,
//! the value an HDL reads as `HiZ`. A change at tick `n` is an event at
//! `n * timescale + toffset`, with the tick converted by
//! [`VcdTimescale::seconds_at_tick`]; a tick that conversion cannot place
//! exactly is refused. Before its first change a logic signal is X, the IEEE
//! 1364 meaning of a value not yet dumped; a real signal leaves its node at
//! its initial value. After its last change a signal holds.
//!
//! A `$dumpoff` stretch is refused, naming its ticks: the `x` a dump writes
//! there means "not recorded", and replaying it as X would invent a state.
//!
//! # State
//!
//! The file is read once, at initialization, into a table kept in the
//! instance's own parameters, so a checkpoint carries no live resource. What
//! changes as the run advances is two integers per port element, the change
//! last driven and the one queued next, in the context's integer state.

use super::*;
use crate::Value;
use crate::io::vcd::{
    VcdBit, VcdDocument, VcdSignal, VcdSignalKind, VcdTimescale, VcdValue,
    parse_vcd_bytes_with_limits,
};
use crate::xspice::{
    CmError, EvaluationPhase, InputDataFileParameter, XspiceCheckpointSupport, XspiceInstance,
    data_file,
};
use std::sync::OnceLock;

/// Digital stimulus replayed from a VCD file.
#[derive(Debug, Default)]
pub(crate) struct DigitalVcdSource;

const MODEL_NAME: &str = "d_vcd";
const FILE_PARAM: &str = "file";
const SIGNALS_PARAM: &str = "signals";
const TOFFSET_PARAM: &str = "toffset";
const DIGITAL_PORT: &str = "out";
const REAL_PORT: &str = "real_out";

// The replay table. A `#` cannot begin a deck parameter name, so no card can
// set or shadow these.
/// Per element, where its changes start in the flat arrays; one more entry
/// than there are elements.
const OFFSETS_KEY: &str = "#d_vcd offsets";
/// Each change's event time, as the bits of its `f64`.
const TIMES_KEY: &str = "#d_vcd times";
/// Each change's value: an event code on `out`, the bits of an `f64` on
/// `real_out`.
const VALUES_KEY: &str = "#d_vcd values";
/// The BLAKE3 digest of the bytes the table was read from, as four words.
const DIGEST_KEY: &str = "#d_vcd digest";

/// Cursor value: nothing driven or queued yet.
const NOTHING: i64 = -1;
/// Cursor value: the element is driving its value from before the first change.
const BEFORE_FIRST: i64 = -2;

fn invalid(name: &str, message: impl Into<String>) -> CmError {
    CmError::InvalidParameter {
        name: name.to_string(),
        message: message.into(),
    }
}

/// The value a VCD bit drives: P3.5's four-state mapping, so `z` is the
/// disconnected state the HDL reads as `HiZ`.
fn digital_value(bit: VcdBit) -> DigitalValue {
    match bit {
        VcdBit::Zero => DigitalValue::zero(),
        VcdBit::One => DigitalValue::one(),
        VcdBit::Unknown => DigitalValue::unknown(),
        VcdBit::HighImpedance => DigitalValue::high_z(),
    }
}

/// Split a trailing bit range off a name: `tb.d[3:0]` is `tb.d` and `3:0`.
fn split_range(name: &str) -> (&str, Option<&str>) {
    if let Some(inner) = name.strip_suffix(']')
        && let Some(open) = inner.rfind('[')
    {
        return (&name[..open], Some(&inner[open + 1..]));
    }
    (name, None)
}

/// How many bits a `[msb:lsb]` or `[bit]` range selects.
fn range_width(range: &str) -> Option<u64> {
    let bound = |text: &str| text.trim().parse::<u64>().ok();
    match range.split_once(':') {
        Some((msb, lsb)) => Some(bound(msb)?.abs_diff(bound(lsb)?) + 1),
        None => bound(range).map(|_| 1),
    }
}

/// A variable's scoped name with the whitespace a declaration may put before
/// its range removed: `d [3:0]` in scope `tb` is `tb.d[3:0]`.
fn compact_name(signal: &VcdSignal, variable: usize) -> String {
    signal.variables[variable]
        .scoped_name()
        .split_whitespace()
        .collect()
}

/// The one signal `request` names, by its full name or, failing that, by the
/// name without a range.
fn find_signal<'a>(
    document: &'a VcdDocument,
    request: &str,
    path: &str,
) -> CmResult<&'a VcdSignal> {
    let named = |matches: &dyn Fn(&str) -> bool| -> Vec<(usize, String)> {
        let mut found: Vec<(usize, String)> = Vec::new();
        for (index, signal) in document.signals.iter().enumerate() {
            for variable in 0..signal.variables.len() {
                let name = compact_name(signal, variable);
                if matches(&name) && found.iter().all(|(seen, _)| *seen != index) {
                    found.push((index, name));
                }
            }
        }
        found
    };
    let mut found = named(&|name| name == request);
    if found.is_empty() {
        let (base, _) = split_range(request);
        found = named(&|name| split_range(name).0 == base);
    }
    match found.as_slice() {
        [(index, _)] => Ok(&document.signals[*index]),
        [] => Err(invalid(
            SIGNALS_PARAM,
            format!("'{request}' names no variable in '{path}'"),
        )),
        several => Err(invalid(
            SIGNALS_PARAM,
            format!(
                "'{request}' names {} variables in '{path}' ({}); name one",
                several.len(),
                several
                    .iter()
                    .map(|(_, name)| name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        )),
    }
}

/// The refusal of the first stretch over which the dump was switched off.
fn refuse_dump_off(document: &VcdDocument, path: &str) -> CmResult<()> {
    let Some(stretch) = document
        .dump_off
        .iter()
        .find(|stretch| stretch.end != Some(stretch.start))
    else {
        return Ok(());
    };
    let end = stretch
        .end
        .map_or_else(|| "the end of the file".to_string(), |end| end.to_string());
    Err(invalid(
        FILE_PARAM,
        format!(
            "'{path}' switches the dump off from tick {} to {end}; the values there were not \
             recorded, so d_vcd cannot replay them",
            stretch.start
        ),
    ))
}

/// One port element's changes, in time order, each a change of value.
#[derive(Default)]
struct Timeline {
    times: Vec<Value>,
    values: Vec<i64>,
}

impl Timeline {
    /// Append the value at `time`. A change at the time of the one before it
    /// replaces it, as the later of two changes in one time step does; a
    /// value equal to the one in force is no change at all.
    fn push(&mut self, time: Value, value: i64, before_first: Option<i64>) {
        if self.times.last() == Some(&time) {
            self.times.pop();
            self.values.pop();
        }
        if self.values.last().copied().or(before_first) != Some(value) {
            self.times.push(time);
            self.values.push(value);
        }
    }
}

/// The exact event time of a change at `tick`.
fn change_time(
    timescale: VcdTimescale,
    tick: u64,
    offset: Value,
    request: &str,
) -> CmResult<Value> {
    let time = timescale.seconds_at_tick(tick).ok_or_else(|| {
        invalid(
            FILE_PARAM,
            format!(
                "tick {tick} of '{request}' has no exact binary64 time at timescale {timescale}"
            ),
        )
    })? + offset;
    if time.is_finite() {
        Ok(time)
    } else {
        Err(invalid(
            TOFFSET_PARAM,
            format!("tick {tick} of '{request}' moved by toffset is not a finite time"),
        ))
    }
}

/// The table the instance replays: one timeline per connected element, `out`
/// first and then `real_out`.
fn build_timelines(
    document: &VcdDocument,
    requests: &[&str],
    path: &str,
    offset: Value,
    digital: usize,
    real: usize,
) -> CmResult<Vec<Timeline>> {
    let mut timelines: Vec<Timeline> = Vec::new();
    for request in requests {
        let signal = find_signal(document, request, path)?;
        let next = timelines.len();
        match signal.kind {
            VcdSignalKind::Logic => {
                let width = signal.width as usize;
                if let Some(stated) = split_range(request).1.and_then(range_width)
                    && stated != u64::from(signal.width)
                {
                    return Err(invalid(
                        SIGNALS_PARAM,
                        format!(
                            "'{request}' is {stated} bits wide, but '{path}' declares it {} bits \
                             wide",
                            signal.width
                        ),
                    ));
                }
                if next >= digital {
                    return Err(invalid(
                        SIGNALS_PARAM,
                        format!(
                            "'{request}' is a logic variable, but it would drive \
                             {REAL_PORT}[{}], a real port",
                            next - digital
                        ),
                    ));
                }
                if next + width > digital {
                    return Err(invalid(
                        SIGNALS_PARAM,
                        format!(
                            "'{request}' is {width} bits wide, but {DIGITAL_PORT} has {} of its \
                             {digital} ports left for it",
                            digital - next
                        ),
                    ));
                }
                let before_first = i64::from(DigitalValue::unknown().event_code());
                let mut bits: Vec<Timeline> = (0..width).map(|_| Timeline::default()).collect();
                for change in &signal.changes {
                    let time = change_time(document.timescale, change.tick, offset, request)?;
                    let VcdValue::Logic(values) = &change.value else {
                        return Err(CmError::Internal(format!(
                            "'{request}' is a logic variable with a real change"
                        )));
                    };
                    for (bit, value) in bits.iter_mut().zip(values) {
                        let code = i64::from(digital_value(*value).event_code());
                        bit.push(time, code, Some(before_first));
                    }
                }
                timelines.extend(bits);
            }
            VcdSignalKind::Real => {
                if split_range(request).1.is_some() {
                    return Err(invalid(
                        SIGNALS_PARAM,
                        format!("'{request}' selects bits of a real variable"),
                    ));
                }
                if next < digital {
                    return Err(invalid(
                        SIGNALS_PARAM,
                        format!(
                            "'{request}' is a real variable, but it would drive \
                             {DIGITAL_PORT}[{next}], a digital port; a real variable drives \
                             {REAL_PORT}"
                        ),
                    ));
                }
                if next >= digital + real {
                    return Err(invalid(
                        SIGNALS_PARAM,
                        format!("'{request}' has no {REAL_PORT} port left to drive"),
                    ));
                }
                let mut timeline = Timeline::default();
                for change in &signal.changes {
                    let time = change_time(document.timescale, change.tick, offset, request)?;
                    let VcdValue::Real(value) = change.value else {
                        return Err(CmError::Internal(format!(
                            "'{request}' is a real variable with a logic change"
                        )));
                    };
                    timeline.push(time, value.to_bits() as i64, None);
                }
                timelines.push(timeline);
            }
        }
    }
    if timelines.len() != digital + real {
        return Err(invalid(
            SIGNALS_PARAM,
            format!(
                "the signals drive {} ports, but the instance connects {digital} on \
                 {DIGITAL_PORT} and {real} on {REAL_PORT}",
                timelines.len()
            ),
        ));
    }
    Ok(timelines)
}

fn digest_words(digest: &blake3::Hash) -> Vec<i64> {
    digest
        .as_bytes()
        .chunks_exact(8)
        .map(|word| i64::from_le_bytes(word.try_into().expect("eight bytes")))
        .collect()
}

/// Read the file, refuse what cannot be replayed, and keep the table.
fn load(ctx: &mut CmContext) -> CmResult<()> {
    let path = ctx.string_param(FILE_PARAM).unwrap_or("").to_string();
    if path.trim().is_empty() {
        return Err(invalid(FILE_PARAM, "d_vcd names no VCD file"));
    }
    let requests_text = ctx.string_param(SIGNALS_PARAM).unwrap_or("").to_string();
    let requests: Vec<&str> = requests_text.split_whitespace().collect();
    if requests.is_empty() {
        return Err(invalid(SIGNALS_PARAM, "d_vcd selects no signal"));
    }
    let offset = ctx.param(TOFFSET_PARAM);
    if !offset.is_finite() {
        return Err(invalid(TOFFSET_PARAM, "toffset must be finite"));
    }
    let limits = ctx.resource_limits();
    let contents = data_file::read_to_string_limited(&path, limits.max_external_data_bytes)
        .map_err(|error| invalid(FILE_PARAM, format!("cannot read '{path}': {error}")))?;
    let document = parse_vcd_bytes_with_limits(contents.as_bytes(), limits)
        .map_err(|error| invalid(FILE_PARAM, format!("'{path}': {error}")))?;
    refuse_dump_off(&document, &path)?;
    let digital = ctx.port_width(DIGITAL_PORT);
    let real = ctx.port_width(REAL_PORT);
    let timelines = build_timelines(&document, &requests, &path, offset, digital, real)?;

    let mut offsets = Vec::with_capacity(timelines.len() + 1);
    let mut times = Vec::new();
    let mut values = Vec::new();
    offsets.push(0);
    for timeline in timelines {
        times.extend(timeline.times.iter().map(|time| time.to_bits() as i64));
        values.extend(timeline.values);
        offsets.push(times.len() as i64);
    }
    ctx.set_integer_vector_param(OFFSETS_KEY, offsets);
    ctx.set_integer_vector_param(TIMES_KEY, times);
    ctx.set_integer_vector_param(VALUES_KEY, values);
    ctx.set_integer_vector_param(DIGEST_KEY, digest_words(&blake3::hash(contents.as_bytes())));
    Ok(())
}

/// The delay that lands an output queued at `now` exactly on `at`, if any
/// does.
///
/// The queue dates an output `now + delay`, and `at - now` can round so that
/// the sum misses `at` by an ulp. Each step of the delay moves the sum by at
/// most one ulp of `at`, so a step or two either way usually lands on it. But
/// when `now` carries a bit below `at`'s last and the delay shares `at`'s
/// binade, every sum is a tie that rounds to an even neighbour of an odd
/// `at`, and no delay reaches it.
fn delay_landing_on(now: Value, at: Value) -> Option<Value> {
    let mut delay = at - now;
    for _ in 0..4 {
        let landed = now + delay;
        if landed == at {
            return Some(delay);
        }
        delay = if landed < at {
            delay.next_up()
        } else {
            delay.next_down()
        };
    }
    None
}

/// The change in force at the current instant and the next one after it, for
/// an element whose cursor last drove change `driven`. A change the clock is
/// within the landing window of is in force ([`CmContext::is_due`]).
///
/// The search restarts from the first change when the cursor is ahead of
/// the clock, so a clock that moved back re-derives the change in force.
fn changes_around(times: &[i64], ctx: &CmContext, driven: i64) -> (Option<usize>, Option<usize>) {
    let at = |index: usize| f64::from_bits(times[index] as u64);
    let start = match usize::try_from(driven)
        .ok()
        .filter(|index| *index < times.len())
    {
        Some(index) if ctx.is_due(at(index)) => index + 1,
        _ => 0,
    };
    let reached =
        times[start..].partition_point(|change| ctx.is_due(f64::from_bits(*change as u64)));
    let next = start + reached;
    (next.checked_sub(1), (next < times.len()).then_some(next))
}

/// One output this evaluation drives.
struct Drive {
    element: usize,
    value: i64,
    delay: Value,
}

fn set_cursor(ctx: &mut CmContext, index: usize, value: i64) {
    if ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe {
        ctx.set_int_state(index, value);
    }
}

impl DigitalVcdSource {
    fn table(ctx: &CmContext) -> CmResult<(&[i64], &[i64], &[i64])> {
        match (
            ctx.integer_vector_param(OFFSETS_KEY),
            ctx.integer_vector_param(TIMES_KEY),
            ctx.integer_vector_param(VALUES_KEY),
        ) {
            (Some(offsets), Some(times), Some(values)) => Ok((offsets, times, values)),
            _ => Err(CmError::Internal(
                "d_vcd evaluated before it read its file".into(),
            )),
        }
    }
}

impl CodeModel for DigitalVcdSource {
    fn input_data_file_parameters(&self) -> &[InputDataFileParameter] {
        &[InputDataFileParameter {
            name: FILE_PARAM,
            trim: false,
            empty_uses_default: false,
        }]
    }

    fn name(&self) -> &str {
        MODEL_NAME
    }

    fn description(&self) -> &str {
        "Digital stimulus replayed from a VCD file"
    }

    fn ports(&self) -> &[PortSpec] {
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::vector_output(DIGITAL_PORT, PortType::Digital).nullable(),
                PortSpec::vector_output(REAL_PORT, PortType::Real).nullable(),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::string(FILE_PARAM, ""),
                ParamSpec::string(SIGNALS_PARAM, ""),
                ParamSpec::real(TOFFSET_PARAM, 0.0),
            ]
        })
    }

    fn checkpoint_support(&self, _ctx: &CmContext) -> XspiceCheckpointSupport {
        // The cursors are integer state and the table is parameters; the
        // outer image binds the file's bytes.
        XspiceCheckpointSupport::Serializable
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        load(ctx)?;
        let elements = ctx.port_width(DIGITAL_PORT) + ctx.port_width(REAL_PORT);
        ctx.allocate_int_states(2 * elements);
        for index in 0..2 * elements {
            ctx.set_int_state(index, NOTHING);
        }
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let now = ctx.time;
        let digital = ctx.port_width(DIGITAL_PORT);
        let mut drives = Vec::new();
        let mut cursors = Vec::new();
        let mut breakpoints = Vec::new();
        {
            let (offsets, times, values) = Self::table(ctx)?;
            for element in 0..offsets.len().saturating_sub(1) {
                let (start, end) = (offsets[element] as usize, offsets[element + 1] as usize);
                let (times, values) = (&times[start..end], &values[start..end]);
                let driven = ctx.int_state(2 * element);
                let queued = ctx.int_state(2 * element + 1);
                let (current, next) = changes_around(times, ctx, driven);
                match current {
                    // Queued ahead at its exact time, and reached: already driven.
                    Some(index) if driven != index as i64 && queued == index as i64 => {
                        cursors.push((2 * element, index as i64));
                    }
                    Some(index) if driven != index as i64 => {
                        // In force on a step landed just before it: still
                        // driven at its own time where a delay reaches it.
                        let at = f64::from_bits(times[index] as u64);
                        let delay = if at > now {
                            delay_landing_on(now, at).unwrap_or(0.0)
                        } else {
                            0.0
                        };
                        drives.push(Drive {
                            element,
                            value: values[index],
                            delay,
                        });
                        cursors.push((2 * element, index as i64));
                    }
                    Some(_) => {}
                    None if driven != BEFORE_FIRST => {
                        if element < digital {
                            drives.push(Drive {
                                element,
                                value: i64::from(DigitalValue::unknown().event_code()),
                                delay: 0.0,
                            });
                        }
                        cursors.push((2 * element, BEFORE_FIRST));
                    }
                    None => {}
                }
                match next {
                    Some(index) if queued != index as i64 => {
                        let at = f64::from_bits(times[index] as u64);
                        // A change no delay reaches from here is not queued:
                        // its breakpoint brings the clock to it, and it is
                        // driven there as the change in force.
                        if let Some(delay) = delay_landing_on(now, at) {
                            drives.push(Drive {
                                element,
                                value: values[index],
                                delay,
                            });
                            cursors.push((2 * element + 1, index as i64));
                        }
                        breakpoints.push(at);
                    }
                    Some(_) => {}
                    None if queued != NOTHING => cursors.push((2 * element + 1, NOTHING)),
                    None => {}
                }
            }
        }
        for Drive {
            element,
            value,
            delay,
        } in drives
        {
            if element < digital {
                let value = DigitalValue::from_event_code(value as u8).ok_or_else(|| {
                    CmError::Internal(format!("d_vcd table holds event code {value}"))
                })?;
                ctx.set_output_digital_vector_element(DIGITAL_PORT, element, value, delay);
            } else {
                ctx.set_output_real_vector_element(
                    REAL_PORT,
                    element - digital,
                    f64::from_bits(value as u64),
                    delay,
                );
            }
        }
        for (index, value) in cursors {
            set_cursor(ctx, index, value);
        }
        // Ask for each queued change's instant as well as publishing them all
        // through `transient_breakpoints`, as the other file stimuli do.
        if ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe {
            for at in breakpoints {
                ctx.request_breakpoint(at);
            }
        }
        Ok(())
    }

    fn transient_breakpoints(&self, ctx: &CmContext) -> CmResult<Vec<Value>> {
        let Ok((_, times, _)) = Self::table(ctx) else {
            return Ok(Vec::new());
        };
        let mut times: Vec<Value> = times
            .iter()
            .map(|time| f64::from_bits(*time as u64))
            .filter(|time| *time > 0.0)
            .collect();
        times.sort_by(Value::total_cmp);
        times.dedup_by(|left, right| left.to_bits() == right.to_bits());
        Ok(times)
    }
}

/// The file a `d_vcd` instance replays and the BLAKE3 digest of the bytes it
/// read, for a transient checkpoint to bind.
pub(crate) fn vcd_source_file(instance: &XspiceInstance) -> Option<(String, blake3::Hash)> {
    if !instance.model_name().eq_ignore_ascii_case(MODEL_NAME) {
        return None;
    }
    let words = instance.integer_vector_param(DIGEST_KEY)?;
    let mut bytes = [0u8; 32];
    for (chunk, word) in bytes.chunks_exact_mut(8).zip(words) {
        chunk.copy_from_slice(&word.to_le_bytes());
    }
    Some((
        instance.string_param(FILE_PARAM)?.to_string(),
        blake3::Hash::from_bytes(bytes),
    ))
}

/// The BLAKE3 digest of the file at `path` now, read as `d_vcd` reads it.
pub(crate) fn vcd_source_digest(path: &str) -> Result<blake3::Hash, String> {
    data_file::read_to_string_limited(path, usize::MAX)
        .map(|contents| blake3::hash(contents.as_bytes()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_delay_lands_its_output_on_the_exact_instant() {
        let lands = |now: f64, at: f64| {
            let delay = delay_landing_on(now, at)
                .unwrap_or_else(|| panic!("no delay from {now:e} reaches {at:e}"));
            assert_eq!((now + delay).to_bits(), at.to_bits());
        };
        let instants = [
            (0.0, 5.0e-9),
            (2.0e-9, 3.0e-9),
            (0.1, 0.3),
            (2.5e-12 * 3.0, 7.0e-12 * 11.0),
            (1.0e-6 / 3.0, 1.0e-6 * 2.0 / 3.0),
        ];
        for (now, at) in instants {
            lands(now, at);
        }
        for k in 1..2000u32 {
            let at = f64::from(k) * 5000.0 * 1e-12 + 1e-9;
            let now = f64::from(k - 1) * 5000.0 * 1e-12 + 1e-9;
            lands(now, at);
        }
    }

    /// 1 ns carries a bit below 3 ns's last, odd, bit, and every delay that
    /// could reach 3 ns shares its binade, so each sum is a tie that rounds
    /// to an even neighbour.
    #[test]
    fn a_delay_no_sum_reaches_is_none() {
        assert_eq!(delay_landing_on(1.0e-9, 3.0e-9), None);
        let mut delay = 3.0e-9_f64 - 1.0e-9;
        for _ in 0..8 {
            delay = delay.next_down();
        }
        for _ in 0..16 {
            assert_ne!(1.0e-9 + delay, 3.0e-9);
            delay = delay.next_up();
        }
    }

    #[test]
    fn names_split_their_range_and_ranges_count_their_bits() {
        assert_eq!(split_range("tb.d[3:0]"), ("tb.d", Some("3:0")));
        assert_eq!(split_range("tb.clk"), ("tb.clk", None));
        assert_eq!(range_width("3:0"), Some(4));
        assert_eq!(range_width("0:7"), Some(8));
        assert_eq!(range_width("5"), Some(1));
        assert_eq!(range_width("a:0"), None);
    }

    #[test]
    fn the_change_in_force_rewinds_when_the_clock_does() {
        let times: Vec<i64> = [1.0e-9, 2.0e-9, 3.0e-9]
            .iter()
            .map(|time: &f64| time.to_bits() as i64)
            .collect();
        let at = |time: Value| {
            let mut ctx = CmContext::new();
            ctx.time = time;
            ctx
        };
        assert_eq!(
            changes_around(&times, &at(0.5e-9), NOTHING),
            (None, Some(0))
        );
        assert_eq!(changes_around(&times, &at(2.0e-9), 0), (Some(1), Some(2)));
        assert_eq!(changes_around(&times, &at(1.5e-9), 1), (Some(0), Some(1)));
        assert_eq!(changes_around(&times, &at(9.0e-9), 1), (Some(2), None));
    }

    #[test]
    fn a_timeline_keeps_the_last_change_of_a_time_and_drops_repeats() {
        let mut timeline = Timeline::default();
        let x = i64::from(DigitalValue::unknown().event_code());
        timeline.push(0.0, x, Some(x));
        timeline.push(0.0, 0, Some(x));
        timeline.push(1.0, 0, Some(x));
        timeline.push(2.0, 1, Some(x));
        timeline.push(2.0, 0, Some(x));
        timeline.push(3.0, 1, Some(x));
        assert_eq!(timeline.times, vec![0.0, 3.0]);
        assert_eq!(timeline.values, vec![0, 1]);
    }
}
