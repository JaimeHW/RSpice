//! Portable digital continuations and subscriptions for a circuit checkpoint.
//!
//! Images contain semantic identities and exact value bits, never interpreter
//! scratch or executable instruction lists. Restore rebuilds those lists from
//! the validated compiled plan without executing any process or reading storage.
//! The enclosing circuit reader must bound input bytes before deserialization
//! and install all restored components together only after every check succeeds.

use std::collections::{BTreeSet, HashSet};

use serde::{Deserialize, Serialize};

use super::super::cfg::CfgValueType;
use super::analog_samples::AnalogReadPlan;
use super::*;

const VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Header {
    version: u32,
    plan: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct PackedValue {
    width: u32,
    aval: Vec<u32>,
    bval: Vec<u32>,
}

impl From<&FourStateValue> for PackedValue {
    fn from(value: &FourStateValue) -> Self {
        Self {
            width: value.width(),
            aval: value.aval().to_vec(),
            bval: value.bval().to_vec(),
        }
    }
}

/// An exact, potentially wider-than-machine-word event count.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DigitalEventCountCheckpoint(PackedValue);

impl DigitalEventCountCheckpoint {
    pub fn capture(count: &digital_value::DigitalEventCount) -> Self {
        Self(count.remaining().into())
    }
}

/// Exact scalar wire representation; real numbers retain all IEEE 754 bits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DigitalScalarCheckpoint(ScalarImage);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum ScalarImage {
    FourState(PackedValue),
    Integer(i32),
    Real(u64),
    Effect,
}

impl DigitalScalarCheckpoint {
    pub fn capture(value: &DigitalScalar) -> Self {
        Self(match value {
            DigitalScalar::FourState(value) => ScalarImage::FourState(value.into()),
            DigitalScalar::Integer(value) => ScalarImage::Integer(*value),
            DigitalScalar::Real(value) => ScalarImage::Real(value.to_bits()),
            DigitalScalar::Effect => ScalarImage::Effect,
        })
    }
}

/// A process cursor and the values captured at its suspension point.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DigitalResumeCheckpoint {
    header: Header,
    process: DigitalProcessId,
    block: BlockId,
    analog_instruction: Option<u64>,
    arguments: Vec<DigitalScalarCheckpoint>,
}

impl DigitalResumeCheckpoint {
    pub fn capture(state: &DigitalResumeState) -> Self {
        Self {
            header: Header {
                version: VERSION,
                plan: state.plan_identity,
            },
            process: state.process,
            block: state.block,
            analog_instruction: state.analog_instruction.map(|index| index as u64),
            arguments: state
                .arguments
                .iter()
                .map(DigitalScalarCheckpoint::capture)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ExpressionState {
    term: DigitalEventExpression,
    previous: DigitalScalarCheckpoint,
}

/// Captured baselines, authenticated against a compiled event-expression site.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DigitalExpressionCheckpoint {
    header: Header,
    process: DigitalProcessId,
    states: Vec<ExpressionState>,
}

impl DigitalExpressionCheckpoint {
    pub fn capture(wait: &DigitalExpressionWait) -> Self {
        Self {
            header: Header {
                version: VERSION,
                plan: wait.plan_identity,
            },
            process: wait.process,
            states: wait
                .states
                .iter()
                .map(|state| ExpressionState {
                    term: DigitalEventExpression {
                        value: state.program.root,
                        edge: state.edge,
                        assignment: state.assignment,
                    },
                    previous: DigitalScalarCheckpoint::capture(&state.previous),
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum WaitBase {
    AnalogSample(DigitalAnalogProbeId),
    Event(Vec<DigitalSensitivityTerm>),
    Expressions(DigitalExpressionCheckpoint),
    Delay(i64),
}

/// Repetition is flattened on the wire so decoded nesting cannot exhaust the stack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DigitalWaitCheckpoint {
    header: Header,
    counts: Vec<PackedValue>,
    base: WaitBase,
}

impl DigitalWaitCheckpoint {
    pub fn capture(plan: &CanonicalDigitalPlan, wait: &DigitalWaitRequest) -> Self {
        let mut counts = Vec::new();
        let mut wait = wait;
        while let DigitalWaitRequest::Repeated { count, event } = wait {
            counts.push(count.remaining().into());
            wait = event;
        }
        let base = match wait {
            DigitalWaitRequest::AnalogSample(probe) => WaitBase::AnalogSample(*probe),
            DigitalWaitRequest::Event(terms) => WaitBase::Event(terms.clone()),
            DigitalWaitRequest::Expressions(wait) => {
                WaitBase::Expressions(DigitalExpressionCheckpoint::capture(wait))
            }
            DigitalWaitRequest::Delay(ticks) => WaitBase::Delay(*ticks),
            DigitalWaitRequest::Repeated { .. } => unreachable!(),
        };
        Self {
            header: Header {
                version: VERSION,
                plan: plan.content_identity,
            },
            counts,
            base,
        }
    }
}

/// An evaluated nonblocking assignment, including its captured target and wait.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DigitalDeferredCheckpoint {
    header: Header,
    target: DigitalWriteTarget,
    value: DigitalScalarCheckpoint,
    region: DigitalSchedulingRegion,
    wait: Option<DigitalWaitCheckpoint>,
}

impl DigitalDeferredCheckpoint {
    pub fn capture(plan: &CanonicalDigitalPlan, update: &DigitalDeferredUpdate) -> Self {
        let value = match &update.value {
            DigitalUpdate::FourState(value) => ScalarImage::FourState(value.into()),
            DigitalUpdate::Real(value) => ScalarImage::Real(value.to_bits()),
        };
        Self {
            header: Header {
                version: VERSION,
                plan: plan.content_identity,
            },
            target: update.target.clone(),
            value: DigitalScalarCheckpoint(value),
            region: update.region,
            wait: update
                .wait
                .as_ref()
                .map(|wait| DigitalWaitCheckpoint::capture(plan, wait)),
        }
    }
}

/// Cumulative budgets for one plan's restored runtime, in addition to the
/// enclosing file reader's byte/allocation limits. Failed attempts consume budget.
#[derive(Debug, Clone, Copy)]
pub struct DigitalCheckpointLimits {
    pub max_items: usize,
    pub max_packed_bits: u64,
}

impl Default for DigitalCheckpointLimits {
    fn default() -> Self {
        Self {
            max_items: 1_048_576,
            max_packed_bits: 67_108_864,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DigitalCheckpointError(String);

impl std::fmt::Display for DigitalCheckpointError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid digital checkpoint: {}", self.0)
    }
}
impl std::error::Error for DigitalCheckpointError {}

type Result<T> = std::result::Result<T, DigitalCheckpointError>;
fn invalid(message: &str) -> DigitalCheckpointError {
    DigitalCheckpointError(message.into())
}

/// Restore context shared by all frames, waits and pending updates of one plan.
/// Derived programs are cached once; no live environment is accessed or mutated.
pub struct DigitalCheckpointReader<'a> {
    plan: &'a CanonicalDigitalPlan,
    limits: DigitalCheckpointLimits,
    items: usize,
    packed_bits: u64,
    analog: HashMap<DigitalProcessId, AnalogReadPlan>,
    programs: HashMap<(DigitalProcessId, ValueId), Arc<DigitalEventProgram>>,
    sites: HashMap<DigitalProcessId, HashSet<Vec<DigitalEventExpression>>>,
}

impl<'a> DigitalCheckpointReader<'a> {
    pub fn new(plan: &'a CanonicalDigitalPlan, limits: DigitalCheckpointLimits) -> Result<Self> {
        plan.validate().map_err(|errors| {
            DigitalCheckpointError(format!("compiled plan is invalid: {errors:?}"))
        })?;
        Ok(Self {
            plan,
            limits,
            items: 0,
            packed_bits: 0,
            analog: HashMap::new(),
            programs: HashMap::new(),
            sites: HashMap::new(),
        })
    }

    fn header(&self, header: &Header) -> Result<()> {
        if header.version != VERSION {
            return Err(invalid("unsupported schema version"));
        }
        if header.plan != self.plan.content_identity {
            return Err(invalid("compiled plan identity differs"));
        }
        Ok(())
    }

    fn charge(&mut self, items: usize) -> Result<()> {
        self.items = self
            .items
            .checked_add(items)
            .filter(|n| *n <= self.limits.max_items)
            .ok_or_else(|| invalid("restored item budget exceeded"))?;
        Ok(())
    }

    fn packed(&mut self, value: &PackedValue) -> Result<FourStateValue> {
        if value.width == 0 || value.width > crate::semantic::MAX_DIGITAL_VECTOR_WIDTH {
            return Err(invalid("packed value width is outside supported bounds"));
        }
        self.packed_bits = self
            .packed_bits
            .checked_add(u64::from(value.width))
            .filter(|bits| *bits <= self.limits.max_packed_bits)
            .ok_or_else(|| invalid("restored packed-bit budget exceeded"))?;
        FourStateValue::from_checked_planes(value.width, &value.aval, &value.bval)
            .ok_or_else(|| invalid("packed planes are incomplete or noncanonical"))
    }

    pub fn restore_scalar(&mut self, value: &DigitalScalarCheckpoint) -> Result<DigitalScalar> {
        self.charge(1)?;
        Ok(match &value.0 {
            ScalarImage::FourState(value) => DigitalScalar::FourState(self.packed(value)?),
            ScalarImage::Integer(value) => DigitalScalar::Integer(*value),
            ScalarImage::Real(bits) => DigitalScalar::Real(f64::from_bits(*bits)),
            ScalarImage::Effect => DigitalScalar::Effect,
        })
    }

    pub fn restore_count(
        &mut self,
        image: &DigitalEventCountCheckpoint,
    ) -> Result<digital_value::DigitalEventCount> {
        self.charge(1)?;
        let value = self.packed(&image.0)?;
        if value.has_unknown() {
            return Err(invalid("repeat count contains unknown bits"));
        }
        Ok(digital_value::DigitalEventCount::new(value))
    }

    fn typed(
        &mut self,
        value: &DigitalScalarCheckpoint,
        expected: CfgValueType,
    ) -> Result<DigitalScalar> {
        let value = self.restore_scalar(value)?;
        let matches = match (&value, expected) {
            (DigitalScalar::FourState(value), CfgValueType::FourState { width }) => {
                value.width() == width
            }
            (DigitalScalar::Integer(_), CfgValueType::Integer)
            | (DigitalScalar::Real(_), CfgValueType::Real)
            | (DigitalScalar::Effect, CfgValueType::Effect) => true,
            _ => false,
        };
        if !matches {
            return Err(invalid(
                "captured scalar type or width differs from compiled value",
            ));
        }
        Ok(value)
    }

    pub fn restore_resume(
        &mut self,
        image: &DigitalResumeCheckpoint,
    ) -> Result<DigitalResumeState> {
        self.header(&image.header)?;
        self.charge(image.arguments.len().saturating_add(1))?;
        let process = self
            .plan
            .process(image.process)
            .ok_or_else(|| invalid("unknown process"))?;
        let function = &process.function;
        let block = function
            .blocks
            .get(usize::from(image.block))
            .ok_or_else(|| invalid("unknown resume block"))?;
        let instruction = image
            .analog_instruction
            .map(usize::try_from)
            .transpose()
            .map_err(|_| invalid("analog instruction index is not representable"))?;
        let expected: Arc<[ValueId]> = if let Some(index) = instruction {
            let instruction = block
                .instructions
                .get(index)
                .ok_or_else(|| invalid("unknown analog instruction"))?;
            if !matches!(
                function.value(instruction.result).kind,
                CfgValueKind::DigitalAnalogVariable { .. }
            ) {
                return Err(invalid(
                    "resume instruction is not an analog sample barrier",
                ));
            }
            self.analog
                .entry(image.process)
                .or_insert_with(|| AnalogReadPlan::build(function))
                .captures
                .get(&instruction.result)
                .cloned()
                .ok_or_else(|| invalid("missing analog capture shape"))?
        } else {
            if !function.blocks.iter().any(|b| matches!(b.terminator, CfgTerminator::Wait { resume, .. } if resume == image.block)) {
                return Err(invalid("block is not a compiled timing continuation"));
            }
            block.params.clone().into()
        };
        if image.arguments.len() != expected.len() {
            return Err(invalid("resume argument count differs"));
        }
        let mut arguments = Vec::with_capacity(expected.len());
        for (value, id) in image.arguments.iter().zip(expected.iter()) {
            arguments.push(self.typed(value, function.value(*id).value_type)?);
        }
        Ok(DigitalResumeState {
            plan_identity: image.header.plan,
            process: image.process,
            block: image.block,
            analog_instruction: instruction,
            arguments,
        })
    }

    pub fn restore_expression(
        &mut self,
        image: &DigitalExpressionCheckpoint,
    ) -> Result<DigitalExpressionWait> {
        self.header(&image.header)?;
        self.charge(image.states.len().saturating_add(1))?;
        let process = self
            .plan
            .process(image.process)
            .ok_or_else(|| invalid("unknown event process"))?;
        let function = &process.function;
        let sites = self
            .sites
            .entry(image.process)
            .or_insert_with(|| expression_sites(function));
        let terms: Vec<_> = image.states.iter().map(|state| state.term).collect();
        if !sites.contains(&terms) {
            return Err(invalid(
                "event expressions do not match a compiled subscription",
            ));
        }
        let mut states = Vec::with_capacity(image.states.len());
        let mut dependencies = BTreeSet::new();
        for state in &image.states {
            let key = (image.process, state.term.value);
            let program = if let Some(program) = self.programs.get(&key) {
                Arc::clone(program)
            } else {
                let (instructions, dependencies) =
                    super::super::digital_validate::event_expression_schedule(
                        function,
                        state.term.value,
                    )
                    .map_err(|detail| {
                        DigitalCheckpointError(format!("invalid event expression: {detail}"))
                    })?;
                let program = Arc::new(DigitalEventProgram {
                    root: state.term.value,
                    instructions,
                    dependencies,
                });
                self.programs.insert(key, Arc::clone(&program));
                program
            };
            self.charge(program.dependencies.len())?;
            dependencies.extend(program.dependencies.iter().copied());
            if let Some(selection) = state.term.assignment {
                let range = selection
                    .array
                    .cell_range()
                    .ok_or_else(|| invalid("invalid assignment-event extent"))?;
                self.charge((range.end - range.start) as usize)?;
                dependencies.extend(range.map(DigitalSignalId::new));
            }
            let previous =
                self.typed(&state.previous, function.value(state.term.value).value_type)?;
            states.push(DigitalExpressionState {
                program,
                previous,
                edge: state.term.edge,
                assignment: state.term.assignment,
            });
        }
        Ok(DigitalExpressionWait {
            plan_identity: image.header.plan,
            process: image.process,
            states,
            dependencies: dependencies.into_iter().collect(),
        })
    }

    pub fn restore_wait(&mut self, image: &DigitalWaitCheckpoint) -> Result<DigitalWaitRequest> {
        self.header(&image.header)?;
        self.charge(image.counts.len().saturating_add(1))?;
        // The compiled language and host accept one repeated event control.
        // Reject nested wire payloads before constructing recursive ownership.
        if image.counts.len() > 1 {
            return Err(invalid("nested repeat controls are not supported"));
        }
        let mut wait = match &image.base {
            WaitBase::AnalogSample(probe) => {
                if self.plan.analog_probe(*probe).is_none_or(|probe| {
                    !matches!(
                        probe.target,
                        super::super::digital::DigitalAnalogProbeTarget::Variable { .. }
                    )
                }) {
                    return Err(invalid("sample barrier does not name an analog variable"));
                }
                DigitalWaitRequest::AnalogSample(*probe)
            }
            WaitBase::Event(terms) => {
                self.charge(terms.len())?;
                for term in terms {
                    let signal = self
                        .plan
                        .signal(term.signal)
                        .ok_or_else(|| invalid("unknown event signal"))?;
                    if signal.kind.is_real() && term.edge.is_some() {
                        return Err(invalid("edge subscription names a real signal"));
                    }
                }
                DigitalWaitRequest::Event(terms.clone())
            }
            WaitBase::Expressions(wait) => {
                DigitalWaitRequest::Expressions(self.restore_expression(wait)?)
            }
            WaitBase::Delay(ticks) => {
                if *ticks < 0 {
                    return Err(invalid("negative wait delay"));
                }
                DigitalWaitRequest::Delay(*ticks)
            }
        };
        if !image.counts.is_empty()
            && !matches!(
                wait,
                DigitalWaitRequest::Event(_) | DigitalWaitRequest::Expressions(_)
            )
        {
            return Err(invalid("repeat must wrap an event subscription"));
        }
        for count in image.counts.iter().rev() {
            let count = self.packed(count)?;
            if count.has_unknown() {
                return Err(invalid("repeat count contains unknown bits"));
            }
            if count.aval().iter().all(|word| *word == 0) {
                return Err(invalid("zero repeat count cannot remain suspended"));
            }
            wait = DigitalWaitRequest::Repeated {
                count: digital_value::DigitalEventCount::new(count),
                event: Box::new(wait),
            };
        }
        Ok(wait)
    }

    pub fn restore_deferred(
        &mut self,
        image: &DigitalDeferredCheckpoint,
    ) -> Result<DigitalDeferredUpdate> {
        self.header(&image.header)?;
        self.charge(1)?;
        if image.region != DigitalSchedulingRegion::NonBlockingAssign {
            return Err(invalid(
                "deferred assignment is outside the nonblocking region",
            ));
        }
        let signal = self
            .plan
            .signal(image.target.signal)
            .ok_or_else(|| invalid("unknown update target"))?;
        if !signal.procedurally_assignable {
            return Err(invalid("update target is not procedurally assignable"));
        }
        let expected = if signal.kind.is_real() {
            if image.target.select != DigitalWriteSelect::Whole {
                return Err(invalid("selected real update"));
            }
            CfgValueType::Real
        } else {
            let width = image
                .target
                .select
                .checked_width(signal.declared_range())
                .ok_or_else(|| invalid("invalid update selection"))?;
            CfgValueType::FourState { width }
        };
        let value = match self.typed(&image.value, expected)? {
            DigitalScalar::FourState(value) => DigitalUpdate::FourState(value),
            DigitalScalar::Real(value) => DigitalUpdate::Real(value),
            _ => unreachable!(),
        };
        let wait = image
            .wait
            .as_ref()
            .map(|wait| self.restore_wait(wait))
            .transpose()?;
        if matches!(wait, Some(DigitalWaitRequest::AnalogSample(_))) {
            return Err(invalid(
                "nonblocking update cannot wait for an analog sample",
            ));
        }
        Ok(DigitalDeferredUpdate {
            target: image.target.clone(),
            value,
            region: image.region,
            wait,
        })
    }
}

fn expression_sites(function: &CfgFunction) -> HashSet<Vec<DigitalEventExpression>> {
    let mut sites = HashSet::new();
    let mut add = |mut wait: &DigitalWait| {
        while let DigitalWait::Repeat { event, .. } = wait {
            wait = event;
        }
        if let DigitalWait::Expressions(terms) = wait {
            sites.insert(terms.clone());
        }
    };
    for block in &function.blocks {
        if let CfgTerminator::Wait { wait, .. } = &block.terminator {
            add(wait);
        }
    }
    for value in &function.values {
        if let CfgValueKind::DigitalNonblockingWrite {
            wait: Some(wait), ..
        }
        | CfgValueKind::DigitalBitNonblockingWrite {
            wait: Some(wait), ..
        }
        | CfgValueKind::DigitalArrayNonblockingWrite {
            wait: Some(wait), ..
        } = &value.kind
        {
            add(wait);
        }
    }
    sites
}
