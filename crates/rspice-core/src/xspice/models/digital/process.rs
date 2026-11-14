use crate::xspice::external::{
    DigitalProcessRuntime, DigitalProcessSpec, start_digital_process_runtime,
};
use crate::xspice::{
    CmContext, CmError, CmResult, CodeModel, DigitalState, DigitalStrength, DigitalValue,
    EvaluationPhase, ParamSpec, PortSpec, PortType, TransactionalContextResource,
};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex, MutexGuard};

/// External digital process code model.
#[derive(Debug, Default)]
pub struct DigitalProcess;

const RESOURCE_CHANNEL: &str = "d_process.channel";
const STATE_PREV_CLK: usize = 0;
const STATE_PREV_RESET: usize = 1;
const STATE_RNG: usize = 2;
const STATE_DOUT_START: usize = 3;
const STATE_ZERO: i64 = 0;
const STATE_ONE: i64 = 1;
const STATE_UNKNOWN: i64 = 2;
const D_PROCESS_RNG_SEED: i64 = 0x4d59_5df4;
const D_PROCESS_MAX_PROCESS_PARAMS: usize = 1022;

fn d_process_error(message: impl Into<String>) -> CmError {
    CmError::EvaluationError(format!("d_process: {}", message.into()))
}

fn d_process_delay(ctx: &CmContext, name: &str, default: f64) -> CmResult<f64> {
    let value = ctx.param_or(name, default);
    if !value.is_finite() {
        return Err(CmError::InvalidParameter {
            name: name.to_string(),
            message: format!("value must be finite, got {value}"),
        });
    }
    Ok(value)
}

/// One instance's connection to its external process: the runtime, the
/// packet buffers its exchanges reuse, and the journal of what the process
/// has answered.
///
/// The connection is one transactional resource shared by every copy of the
/// instance's context. The engine keeps a copy of every context it may have
/// to undo, in the acceptance rollback image or taken just before the model
/// call, and every copy shares this connection, so nothing in it is reached
/// through the copy-on-write resource map: the lock serializes the
/// exchanges, and nothing in the buffers survives from one exchange to the
/// next.
struct DigitalProcessConnection(Mutex<DigitalProcessChannel>);

struct DigitalProcessChannel {
    runtime: Box<dyn DigitalProcessRuntime>,
    input_bytes: Vec<u8>,
    output_bytes: Vec<u8>,
    journal: ExchangeJournal,
}

impl DigitalProcessConnection {
    fn new(runtime: Box<dyn DigitalProcessRuntime>) -> Self {
        Self(Mutex::new(DigitalProcessChannel {
            runtime,
            input_bytes: Vec::new(),
            output_bytes: Vec::new(),
            journal: ExchangeJournal::default(),
        }))
    }

    fn lock(&self) -> CmResult<MutexGuard<'_, DigitalProcessChannel>> {
        self.0
            .lock()
            .map_err(|_| d_process_error("runtime lock is poisoned"))
    }
}

/// The process protocol has no undo, so a rollback cannot take an exchange
/// back from the process. The journal keeps every answer an open transaction
/// can still rewind past; a rollback moves only the cursor back, and a retried
/// trial replays the answers ahead of it.
impl TransactionalContextResource for DigitalProcessConnection {
    fn capture_transaction_state(&self) -> CmResult<Vec<u8>> {
        Ok(self.lock()?.journal.open_mark().encode())
    }

    fn restore_transaction_state(&self, state: &[u8]) -> CmResult<()> {
        let mark = JournalMark::decode(state)?;
        self.lock()?.journal.rewind(mark)
    }

    fn commit_transaction_state(&self, state: &[u8]) {
        let (Ok(mark), Ok(mut channel)) = (JournalMark::decode(state), self.0.lock()) else {
            return;
        };
        channel.journal.close_mark(mark);
    }
}

/// One answered exchange: the signed time and packed inputs the process was
/// sent and the packed outputs it returned.
struct JournalEntry {
    signed_time: f64,
    inputs: Box<[u8]>,
    outputs: Box<[u8]>,
}

/// Where an open transaction rewinds the journal to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct JournalMark {
    id: u64,
    cursor: u64,
}

impl JournalMark {
    fn encode(self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(16);
        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.extend_from_slice(&self.cursor.to_le_bytes());
        bytes
    }

    fn decode(state: &[u8]) -> CmResult<Self> {
        let (Some(id), Some(cursor)) = (state.get(..8), state.get(8..16)) else {
            return Err(d_process_error("exchange journal mark is malformed"));
        };
        if state.len() != 16 {
            return Err(d_process_error("exchange journal mark is malformed"));
        }
        let word = |bytes: &[u8]| {
            let mut word = [0; 8];
            word.copy_from_slice(bytes);
            u64::from_le_bytes(word)
        };
        Ok(Self {
            id: word(id),
            cursor: word(cursor),
        })
    }
}

/// The exchanges an instance's process has answered, by sequence number.
///
/// Entries before the cursor belong to the run the circuit has kept or is
/// trying; entries from the cursor on were answered for a trial that was
/// rolled back and wait to be replayed. Entries older than every point an
/// open transaction can rewind to are dropped, so an accepted run keeps
/// nothing.
#[derive(Default)]
struct ExchangeJournal {
    entries: VecDeque<JournalEntry>,
    /// Sequence number of `entries[0]`.
    first: u64,
    /// Sequence number of the next exchange the circuit makes.
    cursor: u64,
    /// Every open transaction's mark.
    open: Vec<JournalMark>,
    next_mark: u64,
}

impl ExchangeJournal {
    fn open_mark(&mut self) -> JournalMark {
        let mark = JournalMark {
            id: self.next_mark,
            cursor: self.cursor,
        };
        self.next_mark = self.next_mark.wrapping_add(1);
        self.open.push(mark);
        mark
    }

    fn rewind(&mut self, mark: JournalMark) -> CmResult<()> {
        self.forget_mark(mark);
        let end = self.first + self.entries.len() as u64;
        if mark.cursor < self.first || mark.cursor > end {
            self.prune();
            return Err(d_process_error(format!(
                "the exchange journal no longer holds rollback point {} (it holds {} to {end})",
                mark.cursor, self.first
            )));
        }
        self.cursor = mark.cursor;
        self.prune();
        Ok(())
    }

    fn close_mark(&mut self, mark: JournalMark) {
        self.forget_mark(mark);
        self.prune();
    }

    fn forget_mark(&mut self, mark: JournalMark) {
        if let Some(index) = self.open.iter().position(|open| open.id == mark.id) {
            self.open.swap_remove(index);
        }
    }

    /// Drop the entries no open transaction can rewind to.
    fn prune(&mut self) {
        let keep = self
            .open
            .iter()
            .map(|mark| mark.cursor)
            .fold(self.cursor, u64::min);
        while self.first < keep && self.entries.pop_front().is_some() {
            self.first += 1;
        }
    }

    /// The answer the process already gave to the next exchange, if a
    /// rolled-back trial asked it.
    fn pending(&self) -> Option<&JournalEntry> {
        self.cursor
            .checked_sub(self.first)
            .and_then(|index| usize::try_from(index).ok())
            .and_then(|index| self.entries.get(index))
    }

    fn record(&mut self, signed_time: f64, inputs: &[u8], outputs: &[u8]) {
        self.entries.push_back(JournalEntry {
            signed_time,
            inputs: inputs.into(),
            outputs: outputs.into(),
        });
        self.cursor += 1;
        self.prune();
    }
}

impl DigitalProcessChannel {
    /// Exchange the packed inputs for the outputs at `signed_time`, from the
    /// journal when a rolled-back trial already asked the process this.
    fn exchange(&mut self, signed_time: f64, input_width: usize) -> CmResult<()> {
        let Self {
            runtime,
            input_bytes,
            output_bytes,
            journal,
        } = self;
        if let Some(entry) = journal.pending() {
            if entry.signed_time.to_bits() != signed_time.to_bits()
                || *entry.inputs != input_bytes[..]
            {
                return Err(replay_refusal(entry, signed_time, input_bytes, input_width));
            }
            output_bytes.copy_from_slice(&entry.outputs);
            journal.cursor += 1;
            journal.prune();
            return Ok(());
        }
        if let Err(err) = runtime.exchange(signed_time, input_bytes, output_bytes) {
            log::warn!(
                "d_process external exchange failed at {signed_time:.16e}; applying the returned output buffer like ngspice: {err}"
            );
        }
        journal.record(signed_time, input_bytes, output_bytes);
        Ok(())
    }
}

/// A retried exchange the process has already answered differently: sending
/// it would ask the process a second time, and replaying the answer would
/// give the wrong one.
fn replay_refusal(
    entry: &JournalEntry,
    signed_time: f64,
    inputs: &[u8],
    input_width: usize,
) -> CmError {
    let reason = if entry.signed_time.to_bits() != signed_time.to_bits() {
        "at a different time"
    } else {
        "with different inputs"
    };
    d_process_error(format!(
        "a retried circuit trial asks its process {reason} than it already answered: the \
         process answered {} and the trial now asks {}; the process protocol has no undo, so \
         the answer cannot be taken back and the exchange cannot be asked again",
        describe_exchange(entry.signed_time, &entry.inputs, input_width),
        describe_exchange(signed_time, inputs, input_width),
    ))
}

fn describe_exchange(signed_time: f64, inputs: &[u8], input_width: usize) -> String {
    let bits: String = (0..input_width)
        .map(|bit| {
            let set = inputs
                .get(bit >> 3)
                .is_some_and(|byte| (byte >> (bit & 7)) & 1 != 0);
            if set { '1' } else { '0' }
        })
        .collect();
    let reset = if signed_time.is_sign_negative() {
        " with reset high"
    } else {
        ""
    };
    format!(
        "t = {:e} s{reset}, in[0..{input_width}] = {}",
        signed_time.abs(),
        if bits.is_empty() { "none" } else { &bits }
    )
}

fn packed_byte_len(bit_count: usize) -> usize {
    if bit_count == 0 {
        0
    } else {
        (bit_count - 1) / 8 + 1
    }
}

fn digital_state_code(value: DigitalValue) -> i64 {
    match value.state.logic_level() {
        Some(false) => STATE_ZERO,
        Some(true) => STATE_ONE,
        None => STATE_UNKNOWN,
    }
}

fn output_value_from_code(code: i64) -> DigitalValue {
    match code {
        STATE_ZERO => DigitalValue::zero(),
        STATE_ONE => DigitalValue::one(),
        _ => DigitalValue::new(DigitalState::Unknown, DigitalStrength::HighZ),
    }
}

fn unknown_high_z() -> DigitalValue {
    DigitalValue::new(DigitalState::Unknown, DigitalStrength::HighZ)
}

fn set_unknown_outputs(ctx: &mut CmContext, output_width: usize) -> CmResult<()> {
    ctx.set_output_digital_vector_from_context_fn("out", output_width, 0.0, |_, _| unknown_high_z())
}

fn validate_shape(input_width: usize, output_width: usize) -> CmResult<()> {
    if input_width > u8::MAX as usize {
        return Err(d_process_error(format!(
            "input width {input_width} exceeds the official maximum of 255"
        )));
    }
    if output_width == 0 || output_width > u8::MAX as usize {
        return Err(d_process_error(format!(
            "output width {output_width} must be between 1 and 255"
        )));
    }
    Ok(())
}

fn next_unknown_input_bit(seed: &mut u32) -> bool {
    if *seed == 0 {
        *seed = D_PROCESS_RNG_SEED as u32;
    }
    *seed ^= *seed << 13;
    *seed ^= *seed >> 17;
    *seed ^= *seed << 5;
    *seed & 1 != 0
}

fn resize_zeroed(bytes: &mut Vec<u8>, len: usize) -> CmResult<()> {
    bytes.clear();
    if bytes.capacity() < len {
        let additional = len - bytes.capacity();
        bytes.try_reserve_exact(additional).map_err(|err| {
            d_process_error(format!("unable to reserve {len} I/O byte(s): {err}"))
        })?;
    }
    bytes.resize(len, 0);
    Ok(())
}

fn pack_input_bits(ctx: &mut CmContext, input_width: usize, bytes: &mut Vec<u8>) -> CmResult<()> {
    resize_zeroed(bytes, packed_byte_len(input_width))?;
    let inputs = ctx.input_digital_vector_values("in").unwrap_or(&[]);
    let mut rng_seed = ctx.int_state(STATE_RNG) as u32;
    let mut rng_changed = false;

    for bit_index in 0..input_width {
        let value = inputs.get(bit_index).copied().unwrap_or_default();
        let bit = match digital_state_code(value) {
            STATE_ZERO => false,
            STATE_ONE => true,
            _ => {
                rng_changed = true;
                next_unknown_input_bit(&mut rng_seed)
            }
        };
        if bit {
            bytes[bit_index >> 3] |= 1u8 << (bit_index & 7);
        }
    }

    if rng_changed {
        ctx.set_int_state(STATE_RNG, rng_seed as i64);
    }
    Ok(())
}

fn output_code(output_bytes: &[u8], bit_index: usize) -> i64 {
    if ((output_bytes[bit_index >> 3] >> (bit_index & 7)) & 1) != 0 {
        STATE_ONE
    } else {
        STATE_ZERO
    }
}

impl CodeModel for DigitalProcess {
    fn name(&self) -> &str {
        "d_process"
    }

    fn description(&self) -> &str {
        "External digital process"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::vector_input("in", PortType::Digital)
                    .nullable()
                    .with_description("Input vector"),
                PortSpec::input("clk", PortType::Digital).with_description("Clock input"),
                PortSpec::input("reset", PortType::Digital)
                    .nullable()
                    .with_description("Synchronous reset input"),
                PortSpec::vector_output("out", PortType::Digital)
                    .with_vector_min_len(1)
                    .with_description("Output vector"),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::real("clk_delay", 1.0e-9),
                ParamSpec::real("reset_delay", 1.0e-9),
                ParamSpec::string("process_file", "")
                    .required()
                    .with_description("Executable process path or provider identifier"),
                ParamSpec::string_vector("process_params", Vec::new())
                    .with_description("Executable process arguments"),
                ParamSpec::integer("reset_state", 0),
                ParamSpec::real("input_load", 1.0e-12),
                ParamSpec::real("clk_load", 1.0e-12),
                ParamSpec::real("reset_load", 1.0e-12),
            ]
        })
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        let input_width = ctx.port_width("in");
        let output_width = ctx.port_width("out");
        validate_shape(input_width, output_width)?;

        ctx.allocate_int_states(STATE_DOUT_START + output_width);
        ctx.set_int_state(STATE_PREV_CLK, STATE_ZERO);
        ctx.set_int_state(STATE_PREV_RESET, STATE_ZERO);
        ctx.set_int_state(STATE_RNG, D_PROCESS_RNG_SEED);
        for index in 0..output_width {
            ctx.set_int_state(STATE_DOUT_START + index, STATE_UNKNOWN);
        }

        let process_file = ctx
            .string_param("process_file")
            .ok_or_else(|| CmError::InvalidParameter {
                name: "process_file".to_string(),
                message: "must be set".to_string(),
            })?
            .to_string();
        let process_params = ctx
            .string_vector_param("process_params")
            .map(|values| {
                values
                    .iter()
                    .take(D_PROCESS_MAX_PROCESS_PARAMS)
                    .cloned()
                    .collect()
            })
            .unwrap_or_default();
        let spec = DigitalProcessSpec {
            process_file,
            process_params,
            input_count: input_width,
            output_count: output_width,
        };
        let runtime = start_digital_process_runtime(&spec)?;
        ctx.set_transactional_resource(
            RESOURCE_CHANNEL,
            Arc::new(DigitalProcessConnection::new(runtime)),
        );
        set_unknown_outputs(ctx, output_width)?;
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let input_width = ctx.port_width("in");
        let output_width = ctx.port_width("out");
        validate_shape(input_width, output_width)?;

        if ctx.evaluation_phase() == EvaluationPhase::RollbackableProbe {
            return Ok(());
        }

        if ctx.time == 0.0 {
            for index in 0..output_width {
                ctx.set_int_state(STATE_DOUT_START + index, STATE_UNKNOWN);
            }
            set_unknown_outputs(ctx, output_width)?;
            return Ok(());
        }

        let clk_code = ctx
            .input_digital("clk")
            .map(digital_state_code)
            .unwrap_or(STATE_ZERO);
        let reset_code = ctx
            .input_digital("reset")
            .map(digital_state_code)
            .unwrap_or(STATE_ZERO);
        let prev_clk_code = ctx.int_state(STATE_PREV_CLK);

        if prev_clk_code != STATE_ONE && clk_code == STATE_ONE {
            // Fetching the connection enlists it in the evaluation's
            // transaction, which marks the journal before this exchange.
            let connection = ctx
                .transactional_resource::<DigitalProcessConnection>(RESOURCE_CHANNEL)?
                .ok_or_else(|| d_process_error("runtime is not initialized"))?;
            let mut channel = connection.lock()?;
            pack_input_bits(ctx, input_width, &mut channel.input_bytes)?;
            resize_zeroed(&mut channel.output_bytes, packed_byte_len(output_width))?;
            let signed_time = if reset_code == STATE_ONE {
                -ctx.time
            } else {
                ctx.time
            };
            channel.exchange(signed_time, input_width)?;
            let output_bytes = &channel.output_bytes;

            let clk_delay = d_process_delay(ctx, "clk_delay", 1.0e-9)?;
            for index in 0..output_width {
                let new_code = output_code(output_bytes, index);
                if new_code != ctx.int_state(STATE_DOUT_START + index) {
                    ctx.set_output_digital_vector_element(
                        "out",
                        index,
                        output_value_from_code(new_code),
                        clk_delay,
                    );
                    ctx.set_int_state(STATE_DOUT_START + index, new_code);
                }
            }
        }

        ctx.set_int_state(STATE_PREV_CLK, clk_code);
        ctx.set_int_state(STATE_PREV_RESET, reset_code);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xspice::AnalysisType;
    use crate::xspice::context::InputValue;
    use crate::xspice::{ParamType, PortDirection};

    struct StaticProcessRuntime;

    impl DigitalProcessRuntime for StaticProcessRuntime {
        fn exchange(
            &mut self,
            _signed_time: f64,
            _input_bytes: &[u8],
            output_bytes: &mut [u8],
        ) -> CmResult<()> {
            output_bytes.fill(1);
            Ok(())
        }
    }

    struct FailingProcessRuntime;

    impl DigitalProcessRuntime for FailingProcessRuntime {
        fn exchange(
            &mut self,
            _signed_time: f64,
            _input_bytes: &[u8],
            _output_bytes: &mut [u8],
        ) -> CmResult<()> {
            Err(CmError::EvaluationError(
                "synthetic process error".to_string(),
            ))
        }
    }

    fn install_channel(ctx: &mut CmContext, runtime: Box<dyn DigitalProcessRuntime>) {
        ctx.set_transactional_resource(
            RESOURCE_CHANNEL,
            Arc::new(DigitalProcessConnection::new(runtime)),
        );
    }

    /// A clocked context one edge away from an exchange, as the transient
    /// holds it at an accepted point.
    fn rising_edge_context(runtime: Box<dyn DigitalProcessRuntime>) -> CmContext {
        let mut ctx = CmContext::new();
        ctx.analysis = AnalysisType::Transient;
        ctx.time = 1.0e-9;
        ctx.set_evaluation_phase(EvaluationPhase::AcceptedStep);
        ctx.set_port_width("in", 0);
        ctx.set_port_width("out", 1);
        ctx.allocate_int_states(STATE_DOUT_START + 1);
        ctx.set_int_state(STATE_PREV_CLK, STATE_ZERO);
        ctx.set_int_state(STATE_PREV_RESET, STATE_ZERO);
        ctx.set_int_state(STATE_RNG, D_PROCESS_RNG_SEED);
        ctx.set_int_state(STATE_DOUT_START, STATE_UNKNOWN);
        ctx.set_input_digital("clk", DigitalValue::one());
        ctx.set_input_digital("reset", DigitalValue::zero());
        install_channel(&mut ctx, runtime);
        ctx
    }

    /// The engine keeps a copy of every context it may have to undo, and
    /// every copy shares the context's resources. An exchange must go through
    /// while such a copy is alive; it used to stop the transient with "I/O
    /// scratch is not initialized or is not uniquely owned".
    #[test]
    fn d_process_exchanges_while_a_rollback_image_shares_its_context() {
        let mut ctx = rising_edge_context(Box::new(StaticProcessRuntime));
        let rollback_image = ctx.clone();

        DigitalProcess
            .evaluate(&mut ctx)
            .expect("d_process exchanges while a copy of its context is alive");

        assert_eq!(ctx.int_state(STATE_DOUT_START), STATE_ONE);
        let events = ctx.take_pending_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].values, vec![DigitalValue::one()]);
        assert_eq!(events[0].delay, 1.0e-9);
        assert_eq!(
            rollback_image.int_state(STATE_DOUT_START),
            STATE_UNKNOWN,
            "the rollback image keeps the state from before the edge"
        );
    }

    #[test]
    fn d_process_metadata_matches_ngspice46_interface() {
        let ports = DigitalProcess.ports();
        assert_eq!(
            ports
                .iter()
                .map(|port| port.name.as_str())
                .collect::<Vec<_>>(),
            vec!["in", "clk", "reset", "out"]
        );
        assert_eq!(ports[0].direction, PortDirection::In);
        assert_eq!(ports[0].default_type, PortType::Digital);
        assert!(ports[0].is_vector);
        assert!(ports[0].null_allowed);
        assert_eq!(ports[0].vector_min_len, None);
        assert_eq!(ports[1].direction, PortDirection::In);
        assert!(!ports[1].is_vector);
        assert!(!ports[1].null_allowed);
        assert_eq!(ports[2].direction, PortDirection::In);
        assert!(!ports[2].is_vector);
        assert!(ports[2].null_allowed);
        assert_eq!(ports[3].direction, PortDirection::Out);
        assert!(ports[3].is_vector);
        assert!(!ports[3].null_allowed);
        assert_eq!(ports[3].vector_min_len, Some(1));

        let params = DigitalProcess.parameters();
        assert_eq!(
            params
                .iter()
                .map(|param| (param.name.as_str(), &param.param_type))
                .collect::<Vec<_>>(),
            vec![
                ("clk_delay", &ParamType::Real),
                ("reset_delay", &ParamType::Real),
                ("process_file", &ParamType::String),
                ("process_params", &ParamType::StringVector),
                ("reset_state", &ParamType::Integer),
                ("input_load", &ParamType::Real),
                ("clk_load", &ParamType::Real),
                ("reset_load", &ParamType::Real),
            ]
        );
        assert_eq!(params[0].default, 1.0e-9);
        assert_eq!(params[1].default, 1.0e-9);
        assert!(params[2].required);
        assert_eq!(params[4].default, 0.0);
        assert_eq!(params[5].default, 1.0e-12);
        assert_eq!(params[6].default, 1.0e-12);
        assert_eq!(params[7].default, 1.0e-12);
    }

    #[test]
    fn d_process_rejects_nonfinite_clk_delay_before_queueing_outputs() {
        let mut ctx = CmContext::new();
        ctx.analysis = AnalysisType::Transient;
        ctx.time = 1.0e-9;
        ctx.set_port_width("in", 0);
        ctx.set_port_width("out", 1);
        ctx.allocate_int_states(STATE_DOUT_START + 1);
        ctx.set_int_state(STATE_PREV_CLK, STATE_ZERO);
        ctx.set_int_state(STATE_PREV_RESET, STATE_ZERO);
        ctx.set_int_state(STATE_RNG, D_PROCESS_RNG_SEED);
        ctx.set_int_state(STATE_DOUT_START, STATE_UNKNOWN);
        ctx.set_param("clk_delay", f64::NAN);
        ctx.set_input_digital("clk", DigitalValue::one());
        ctx.set_input_digital("reset", DigitalValue::zero());
        install_channel(&mut ctx, Box::new(StaticProcessRuntime));

        let err = DigitalProcess
            .evaluate(&mut ctx)
            .expect_err("nonfinite d_process clk_delay must fail evaluation");

        assert!(
            err.to_string().contains("clk_delay"),
            "error should identify clk_delay, got {err:?}"
        );
        assert!(
            ctx.take_pending_events().is_empty(),
            "d_process must not queue outputs after rejecting clk_delay"
        );
        assert_eq!(ctx.int_state(STATE_DOUT_START), STATE_UNKNOWN);
    }

    #[test]
    fn d_process_pack_input_bits_reuses_buffer_and_commits_rng_once() {
        let mut ctx = CmContext::new();
        ctx.allocate_int_states(STATE_DOUT_START);
        ctx.set_int_state(STATE_RNG, D_PROCESS_RNG_SEED);
        ctx.set_input(
            "in",
            InputValue::DigitalVector(vec![
                DigitalValue::one(),
                DigitalValue::zero(),
                DigitalValue::unknown(),
                DigitalValue::one(),
                DigitalValue::unknown(),
                DigitalValue::zero(),
                DigitalValue::unknown(),
                DigitalValue::unknown(),
                DigitalValue::one(),
            ]),
        );

        let mut expected_seed = D_PROCESS_RNG_SEED as u32;
        let unknown_2 = next_unknown_input_bit(&mut expected_seed);
        let unknown_4 = next_unknown_input_bit(&mut expected_seed);
        let unknown_6 = next_unknown_input_bit(&mut expected_seed);
        let unknown_7 = next_unknown_input_bit(&mut expected_seed);
        let mut expected_first_byte = 0b0000_1001;
        if unknown_2 {
            expected_first_byte |= 1 << 2;
        }
        if unknown_4 {
            expected_first_byte |= 1 << 4;
        }
        if unknown_6 {
            expected_first_byte |= 1 << 6;
        }
        if unknown_7 {
            expected_first_byte |= 1 << 7;
        }

        let mut bytes = Vec::new();
        pack_input_bits(&mut ctx, 9, &mut bytes).expect("pack input bits");

        assert_eq!(bytes, vec![expected_first_byte, 0b0000_0001]);
        assert_eq!(ctx.int_state(STATE_RNG) as u32, expected_seed);

        let first_ptr = bytes.as_ptr();
        let first_capacity = bytes.capacity();
        ctx.set_input("in", InputValue::DigitalVector(vec![DigitalValue::zero()]));
        pack_input_bits(&mut ctx, 9, &mut bytes).expect("pack input bits");

        assert_eq!(bytes, vec![0, 0]);
        assert_eq!(bytes.as_ptr(), first_ptr);
        assert_eq!(bytes.capacity(), first_capacity);
    }

    #[test]
    fn d_process_time_zero_rollbackable_probe_does_not_reset_outputs_or_state() {
        let mut ctx = CmContext::new();
        ctx.analysis = AnalysisType::Transient;
        ctx.time = 0.0;
        ctx.set_evaluation_phase(EvaluationPhase::RollbackableProbe);
        ctx.set_port_width("in", 0);
        ctx.set_port_width("out", 1);
        ctx.allocate_int_states(STATE_DOUT_START + 1);
        ctx.set_int_state(STATE_DOUT_START, STATE_ONE);
        ctx.set_output_digital_vector("out", vec![DigitalValue::one()], 0.0);
        ctx.take_pending_events();

        DigitalProcess
            .evaluate(&mut ctx)
            .expect("time-zero rollbackable probe evaluates");

        assert_eq!(
            ctx.int_state(STATE_DOUT_START),
            STATE_ONE,
            "time-zero rollbackable probe must not reset committed output state"
        );
        assert!(
            ctx.take_pending_events().is_empty(),
            "time-zero rollbackable probe must not schedule reset output events"
        );
    }

    #[test]
    fn d_process_time_zero_direct_evaluation_streams_unknown_outputs() {
        let mut ctx = CmContext::new();
        ctx.analysis = AnalysisType::Transient;
        ctx.time = 0.0;
        ctx.set_port_width("in", 0);
        ctx.set_port_width("out", 2);
        ctx.allocate_int_states(STATE_DOUT_START + 2);
        ctx.set_int_state(STATE_DOUT_START, STATE_ONE);
        ctx.set_int_state(STATE_DOUT_START + 1, STATE_ZERO);

        DigitalProcess
            .evaluate(&mut ctx)
            .expect("time-zero direct evaluation resets outputs");

        assert_eq!(ctx.int_state(STATE_DOUT_START), STATE_UNKNOWN);
        assert_eq!(ctx.int_state(STATE_DOUT_START + 1), STATE_UNKNOWN);
        let events = ctx.take_pending_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].port_name, "out");
        assert_eq!(events[0].delay, 0.0);
        assert_eq!(events[0].values, vec![unknown_high_z(), unknown_high_z()]);
    }

    #[test]
    fn d_process_failed_exchange_applies_zeroed_output_buffer_like_ngspice() {
        let mut ctx = CmContext::new();
        ctx.analysis = AnalysisType::Transient;
        ctx.time = 1.0e-9;
        ctx.set_port_width("in", 0);
        ctx.set_port_width("out", 1);
        ctx.allocate_int_states(STATE_DOUT_START + 1);
        ctx.set_int_state(STATE_PREV_CLK, STATE_ZERO);
        ctx.set_int_state(STATE_PREV_RESET, STATE_ZERO);
        ctx.set_int_state(STATE_RNG, D_PROCESS_RNG_SEED);
        ctx.set_int_state(STATE_DOUT_START, STATE_UNKNOWN);
        ctx.set_input_digital("clk", DigitalValue::one());
        ctx.set_input_digital("reset", DigitalValue::zero());
        install_channel(&mut ctx, Box::new(FailingProcessRuntime));

        DigitalProcess
            .evaluate(&mut ctx)
            .expect("ngspice keeps evaluating d_process output buffer after exchange errors");

        assert_eq!(ctx.int_state(STATE_DOUT_START), STATE_ZERO);
        let events = ctx.take_pending_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].port_name, "out");
        assert_eq!(events[0].values, vec![DigitalValue::zero()]);
        assert_eq!(events[0].delay, 1.0e-9);
    }

    /// Every exchange a process was asked: the signed time and packed inputs.
    type Asked = Arc<Mutex<Vec<(f64, Vec<u8>)>>>;

    /// The toggle peer: answers each exchange with a bit that flips on every
    /// call, so a repeated exchange would show in its answers.
    struct CountingTogglePeer {
        q: u8,
        asked: Asked,
    }

    impl DigitalProcessRuntime for CountingTogglePeer {
        fn exchange(
            &mut self,
            signed_time: f64,
            input_bytes: &[u8],
            output_bytes: &mut [u8],
        ) -> CmResult<()> {
            self.asked
                .lock()
                .unwrap()
                .push((signed_time, input_bytes.to_vec()));
            self.q ^= 1;
            output_bytes.fill(0);
            output_bytes[0] = self.q;
            Ok(())
        }
    }

    fn toggle_context(asked: &Asked) -> CmContext {
        let mut ctx = rising_edge_context(Box::new(CountingTogglePeer {
            q: 0,
            asked: Arc::clone(asked),
        }));
        ctx.set_port_width("in", 2);
        ctx.set_input(
            "in",
            InputValue::DigitalVector(vec![DigitalValue::one(), DigitalValue::zero()]),
        );
        ctx
    }

    fn scoped(ctx: &mut CmContext, transaction: &crate::xspice::ResourceTransaction) {
        ctx.set_resource_transaction(Some(crate::xspice::ResourceTransactionScope {
            transaction: transaction.clone(),
            owner: 0,
        }));
    }

    fn journal_len(ctx: &CmContext) -> usize {
        let connection = ctx
            .transactional_resource::<DigitalProcessConnection>(RESOURCE_CHANNEL)
            .unwrap()
            .unwrap();
        connection.lock().unwrap().journal.entries.len()
    }

    /// A circuit trial exchanges with the process and is rolled back; the
    /// acceptance at the same time with the same inputs replays the answer
    /// from the journal. The process is asked once, the outputs are the ones
    /// it gave, and the accepted journal keeps nothing.
    #[test]
    fn d_process_replays_a_rolled_back_trial_from_its_journal_without_asking_the_process_again() {
        let asked = Asked::default();
        let accepted = toggle_context(&asked);

        let mut trial = accepted.clone();
        trial.set_evaluation_phase(EvaluationPhase::CircuitTrial);
        let rollback = crate::xspice::ResourceTransaction::default();
        scoped(&mut trial, &rollback);
        DigitalProcess
            .evaluate(&mut trial)
            .expect("the trial exchanges");
        assert_eq!(trial.int_state(STATE_DOUT_START), STATE_ONE);
        assert_eq!(asked.lock().unwrap().len(), 1);
        assert!(rollback.rollback().is_empty(), "the journal rewinds");
        drop(trial);

        let mut acceptance = accepted.clone();
        let commit = crate::xspice::ResourceTransaction::default();
        scoped(&mut acceptance, &commit);
        DigitalProcess
            .evaluate(&mut acceptance)
            .expect("the acceptance replays the trial's exchange");
        assert_eq!(
            asked.lock().unwrap().as_slice(),
            &[(1.0e-9, vec![0b01])],
            "the process answered the exchange once"
        );
        assert_eq!(
            acceptance.int_state(STATE_DOUT_START),
            STATE_ONE,
            "the replay gives the process's own answer"
        );
        let events = acceptance.take_pending_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].values, vec![DigitalValue::one()]);
        assert_eq!(journal_len(&acceptance), 1, "the open acceptance keeps it");
        commit.commit();
        acceptance.set_resource_transaction(None);
        assert_eq!(
            journal_len(&acceptance),
            0,
            "an accepted exchange is dropped"
        );

        // The next edge is a new exchange, answered by the process.
        acceptance.set_int_state(STATE_PREV_CLK, STATE_ZERO);
        acceptance.time = 2.0e-9;
        DigitalProcess.evaluate(&mut acceptance).expect("next edge");
        assert_eq!(asked.lock().unwrap().len(), 2);
        assert_eq!(acceptance.int_state(STATE_DOUT_START), STATE_ZERO);
        assert_eq!(journal_len(&acceptance), 0);
    }

    /// A retried trial that asks with other inputs, or at another time, than
    /// the process already answered is refused with both exchanges, and the
    /// process is not asked again.
    #[test]
    fn d_process_refuses_a_retry_its_process_answered_differently_with_both_exchanges() {
        for (time, inputs, reason, asks) in [
            (
                1.0e-9,
                [DigitalValue::zero(), DigitalValue::one()],
                "with different inputs",
                "t = 1e-9 s, in[0..2] = 01",
            ),
            (
                1.5e-9,
                [DigitalValue::one(), DigitalValue::zero()],
                "at a different time",
                "t = 1.5e-9 s, in[0..2] = 10",
            ),
        ] {
            let asked = Asked::default();
            let accepted = toggle_context(&asked);
            let mut trial = accepted.clone();
            trial.set_evaluation_phase(EvaluationPhase::CircuitTrial);
            let rollback = crate::xspice::ResourceTransaction::default();
            scoped(&mut trial, &rollback);
            DigitalProcess
                .evaluate(&mut trial)
                .expect("the trial exchanges");
            assert!(rollback.rollback().is_empty());

            let mut retry = accepted.clone();
            retry.set_evaluation_phase(EvaluationPhase::CircuitTrial);
            retry.time = time;
            retry.set_input("in", InputValue::DigitalVector(inputs.to_vec()));
            let transaction = crate::xspice::ResourceTransaction::default();
            scoped(&mut retry, &transaction);
            let error = DigitalProcess
                .evaluate(&mut retry)
                .expect_err("a different retry is refused")
                .to_string();
            assert!(error.contains(reason), "{error}");
            assert!(
                error.contains("answered t = 1e-9 s, in[0..2] = 10"),
                "{error}"
            );
            assert!(error.contains(&format!("now asks {asks}")), "{error}");
            assert!(error.contains("no undo"), "{error}");
            assert_eq!(
                asked.lock().unwrap().len(),
                1,
                "the process is not asked again"
            );
            assert!(transaction.rollback().is_empty());
        }
    }

    /// A rollback inside an open outer transaction keeps the entries the
    /// outer one can still rewind past, and the outer rollback replays them.
    #[test]
    fn d_process_journal_keeps_what_an_open_outer_transaction_can_still_rewind() {
        let asked = Asked::default();
        let accepted = toggle_context(&asked);
        let outer = crate::xspice::ResourceTransaction::default();
        let mut ctx = accepted.clone();
        scoped(&mut ctx, &outer);
        let connection = ctx
            .transactional_resource::<DigitalProcessConnection>(RESOURCE_CHANNEL)
            .unwrap()
            .unwrap();

        let inner = crate::xspice::ResourceTransaction::default();
        scoped(&mut ctx, &inner);
        DigitalProcess.evaluate(&mut ctx).expect("exchange");
        inner.commit();
        assert_eq!(connection.lock().unwrap().journal.entries.len(), 1);

        assert!(outer.rollback().is_empty());
        let mut retry = accepted.clone();
        DigitalProcess.evaluate(&mut retry).expect("replay");
        assert_eq!(asked.lock().unwrap().len(), 1);
        assert_eq!(retry.int_state(STATE_DOUT_START), STATE_ONE);
        assert_eq!(connection.lock().unwrap().journal.entries.len(), 0);
    }
}
