//! The value-returning system functions of analog code: the random and
//! distribution functions (LRM 2.4 section 9.13, IEEE 1364-2005 section
//! 17.9), `$realtobits` (section 17.8), the plusarg queries (section 17.10)
//! and `$fopen` and `$fscanf` (LRM 2.4 section 9.5).
//!
//! Every analog route evaluates a call through [`AnalogSystemState::call`],
//! so the interpreter, the native code and the browser's WebAssembly give
//! the same value for the same call.
//!
//! # Transactions
//!
//! A seed variable is an ordinary transactional variable: every evaluation
//! reads it from the accepted point. What the calls keep for themselves (the
//! internal seed of a parameter or constant seed, the stream of a call with no
//! seed, and the open files) is kept the same way. An evaluation starts from
//! the accepted state and accepting the point keeps its last evaluation's, so
//! a rejected point leaves no draw and no file position behind (LRM 2.4
//! section 9.3). A pass that only observes a point starts from the accepted
//! state on a copy and leaves the candidate as it found it, so it sees the
//! values the evaluation saw.
//!
//! # Streams
//!
//! A call without a seed draws from a stream (LRM 2.4 section 9.13.1 lets
//! the simulator pick the seed). Each instance has its own, started by
//! [`instance_stream_seed`] from the run's seed and the instance's
//! hierarchical name, so two instances of a module draw different values, and
//! the values depend on neither the order the deck lists the instances in nor
//! the order they are evaluated in. A `"global"` type string draws from the
//! design's stream instead, which starts from the run's seed itself in every
//! instance, so every instance sees the same value at the same draw;
//! `"instance"` draws from the instance's stream. A type string on a call
//! with a seed scopes the seed the same way: the call draws the section
//! 17.9.3 sequence that starts from the seed exclusive-ored with the
//! instance's stream seed (`"instance"`) or with the run's seed
//! (`"global"`), and the seed it leaves is that sequence's next seed,
//! exclusive-ored back.

use std::sync::Arc;

use crate::analog_files::{AnalogFileOpen, AnalogFileTable, AnalogScanValue};
use crate::random;

/// One value-returning system function analog code can call.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AnalogSystemFunction {
    Random,
    Arandom,
    DistUniform,
    DistNormal,
    DistExponential,
    DistPoisson,
    DistChiSquare,
    DistT,
    DistErlang,
    RdistUniform,
    RdistNormal,
    RdistExponential,
    RdistPoisson,
    RdistChiSquare,
    RdistT,
    RdistErlang,
    /// The low 32 bits of the IEEE 754 pattern, which is what an analog
    /// integer holds of the 64-bit result.
    RealToBits,
    TestPlusargs,
    ValuePlusargs,
    Fopen,
    Fscanf,
}

impl AnalogSystemFunction {
    pub const ALL: [Self; 21] = [
        Self::Random,
        Self::Arandom,
        Self::DistUniform,
        Self::DistNormal,
        Self::DistExponential,
        Self::DistPoisson,
        Self::DistChiSquare,
        Self::DistT,
        Self::DistErlang,
        Self::RdistUniform,
        Self::RdistNormal,
        Self::RdistExponential,
        Self::RdistPoisson,
        Self::RdistChiSquare,
        Self::RdistT,
        Self::RdistErlang,
        Self::RealToBits,
        Self::TestPlusargs,
        Self::ValuePlusargs,
        Self::Fopen,
        Self::Fscanf,
    ];

    /// The source spelling.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Random => "$random",
            Self::Arandom => "$arandom",
            Self::DistUniform => "$dist_uniform",
            Self::DistNormal => "$dist_normal",
            Self::DistExponential => "$dist_exponential",
            Self::DistPoisson => "$dist_poisson",
            Self::DistChiSquare => "$dist_chi_square",
            Self::DistT => "$dist_t",
            Self::DistErlang => "$dist_erlang",
            Self::RdistUniform => "$rdist_uniform",
            Self::RdistNormal => "$rdist_normal",
            Self::RdistExponential => "$rdist_exponential",
            Self::RdistPoisson => "$rdist_poisson",
            Self::RdistChiSquare => "$rdist_chi_square",
            Self::RdistT => "$rdist_t",
            Self::RdistErlang => "$rdist_erlang",
            Self::RealToBits => "$realtobits",
            Self::TestPlusargs => "$test$plusargs",
            Self::ValuePlusargs => "$value$plusargs",
            Self::Fopen => "$fopen",
            Self::Fscanf => "$fscanf",
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|function| function.name() == name)
    }

    /// A random or distribution function, which takes a seed.
    pub const fn is_random(self) -> bool {
        matches!(
            self,
            Self::Random
                | Self::Arandom
                | Self::DistUniform
                | Self::DistNormal
                | Self::DistExponential
                | Self::DistPoisson
                | Self::DistChiSquare
                | Self::DistT
                | Self::DistErlang
                | Self::RdistUniform
                | Self::RdistNormal
                | Self::RdistExponential
                | Self::RdistPoisson
                | Self::RdistChiSquare
                | Self::RdistT
                | Self::RdistErlang
        )
    }

    /// A real distribution (LRM 2.4 section 9.13.2), which may end with a
    /// Monte Carlo type string.
    pub const fn is_real_distribution(self) -> bool {
        matches!(
            self,
            Self::RdistUniform
                | Self::RdistNormal
                | Self::RdistExponential
                | Self::RdistPoisson
                | Self::RdistChiSquare
                | Self::RdistT
                | Self::RdistErlang
        )
    }

    /// Whether the result is an integer; every other function returns a real.
    pub const fn returns_integer(self) -> bool {
        !self.is_real_distribution()
    }

    /// How many distribution parameters follow the seed.
    pub const fn distribution_parameters(self) -> usize {
        match self {
            Self::DistUniform
            | Self::DistNormal
            | Self::DistErlang
            | Self::RdistUniform
            | Self::RdistNormal
            | Self::RdistErlang => 2,
            Self::DistExponential
            | Self::DistPoisson
            | Self::DistChiSquare
            | Self::DistT
            | Self::RdistExponential
            | Self::RdistPoisson
            | Self::RdistChiSquare
            | Self::RdistT => 1,
            _ => 0,
        }
    }
}

/// A random call's type string (LRM 2.4 section 9.13): which stream a call
/// without a seed draws from, and how a seed is scoped.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AnalogDrawScope {
    /// No type string: the instance's stream, or the seed as given.
    #[default]
    Unscoped,
    /// `"instance"`: the instance's stream, or the seed scoped to the
    /// instance.
    Instance,
    /// `"global"`: the design's stream, or the seed scoped to the design.
    Global,
}

/// The seed an instance's stream starts from: the run's seed exclusive-ored
/// with the 32-bit FNV-1a hash of the instance's hierarchical name, ASCII
/// lower-cased because SPICE names are case-insensitive, then advanced once
/// by IEEE 1364-2005 section 17.9.3's seed update. It is fixed by the run's
/// seed and the name alone, so a rerun draws the same values and reordering
/// the deck changes nothing.
pub fn instance_stream_seed(run_seed: i32, instance: &str) -> i32 {
    let mut hash = 0x811c_9dc5_u32;
    for byte in instance.bytes() {
        hash ^= u32::from(byte.to_ascii_lowercase());
        hash = hash.wrapping_mul(0x0100_0193);
    }
    let mut seed = run_seed ^ hash as i32;
    random::random(&mut seed);
    seed
}

/// Where a random call's seed lives.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AnalogSeed {
    /// No seed: the instance's stream, which starts from the run's seed.
    Stream,
    /// An integer variable, which is the call's first operand and first
    /// output.
    Variable,
    /// A parameter or a constant: the call's internal seed starts from it and
    /// the call updates only the internal seed (LRM 2.4 section 9.13.1).
    Internal,
}

/// One call site: what it calls and the text its literals hold.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AnalogSystemSite {
    pub function: AnalogSystemFunction,
    pub seed: AnalogSeed,
    /// A random call's type string.
    pub scope: AnalogDrawScope,
    /// `$fopen`'s name and mode, `$fscanf`'s format, or the plusarg name or
    /// format.
    pub text: Vec<String>,
    /// `$fscanf` and `$value$plusargs` targets, `true` for an integer one.
    pub targets: Vec<bool>,
    /// Where the call was written, for messages.
    pub file: String,
    /// One-based.
    pub line: u32,
}

impl AnalogSystemSite {
    /// `file:line: $name`, the way a message names the call.
    pub fn label(&self) -> String {
        format!("{}:{}: {}", self.file, self.line, self.function.name())
    }
}

/// What a call keeps between calls besides seed variables.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Seeds {
    /// The instance's stream.
    stream: i32,
    /// The design's stream, as this instance has drawn it.
    global: i32,
    /// Each site's internal seed, once its first call has started it.
    internal: Vec<Option<i32>>,
}

impl Seeds {
    fn new(stream: i32, global: i32, sites: usize) -> Self {
        Self {
            stream,
            global,
            internal: vec![None; sites],
        }
    }
}

/// What a type string exclusive-ors into a seed: the seeds the instance's and
/// the design's streams start from.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct ScopeKeys {
    instance: i32,
    global: i32,
}

impl ScopeKeys {
    fn key(self, scope: AnalogDrawScope) -> i32 {
        match scope {
            AnalogDrawScope::Unscoped => 0,
            AnalogDrawScope::Instance => self.instance,
            AnalogDrawScope::Global => self.global,
        }
    }
}

/// An observation pass's private state: the accepted seeds and, once a call
/// needs them, the accepted files.
#[derive(Debug, Clone)]
struct Observation {
    seeds: Seeds,
    files: Option<Box<dyn AnalogFileTable>>,
}

/// One instance's system-function state. See the module documentation.
#[derive(Debug, Clone)]
pub struct AnalogSystemState {
    sites: Arc<[AnalogSystemSite]>,
    /// The host's answer for each plusarg site: `Some` when a plusarg
    /// matched, with the converted value for `$value$plusargs`.
    plusargs: Arc<[Option<f64>]>,
    keys: ScopeKeys,
    initial: Seeds,
    accepted: Seeds,
    candidate: Seeds,
    observation: Option<Observation>,
    /// The outputs of the latest call, which the assignments after it read.
    output_site: Option<u32>,
    outputs: Vec<Option<f64>>,
}

/// The checkpoint image's version.
const CHECKPOINT_VERSION: u64 = 2;

impl AnalogSystemState {
    pub fn new(sites: Arc<[AnalogSystemSite]>) -> Self {
        let seeds = Seeds::new(0, 0, sites.len());
        Self {
            plusargs: vec![None; sites.len()].into(),
            keys: ScopeKeys::default(),
            sites,
            initial: seeds.clone(),
            accepted: seeds.clone(),
            candidate: seeds,
            observation: None,
            output_site: None,
            outputs: Vec::new(),
        }
    }

    pub fn sites(&self) -> &[AnalogSystemSite] {
        &self.sites
    }

    /// The seeds the streams start from (see the module documentation): the
    /// instance's, from [`instance_stream_seed`], and the design's, the run's
    /// seed itself, as the digital kernel's `$random` starts from it.
    pub fn set_stream_seeds(&mut self, instance: i32, global: i32) {
        self.keys = ScopeKeys { instance, global };
        for seeds in [&mut self.initial, &mut self.accepted, &mut self.candidate] {
            seeds.stream = instance;
            seeds.global = global;
        }
    }

    /// The host's plusarg answers, one per site (`None` for a site that is not
    /// a plusarg query or that no plusarg matched).
    pub fn set_plusargs(&mut self, resolved: Vec<Option<f64>>) -> Result<(), String> {
        if resolved.len() != self.sites.len() {
            return Err(format!(
                "{} plusarg answers for {} system-function sites",
                resolved.len(),
                self.sites.len()
            ));
        }
        self.plusargs = resolved.into();
        Ok(())
    }

    /// An evaluation begins from the accepted state.
    pub fn begin_evaluation(&mut self) {
        self.candidate.clone_from(&self.accepted);
        self.observation = None;
        self.output_site = None;
    }

    /// An observation begins: it runs on a copy of the accepted state and
    /// leaves the candidate alone.
    pub fn begin_observation(&mut self) {
        self.observation = Some(Observation {
            seeds: self.accepted.clone(),
            files: None,
        });
        self.output_site = None;
    }

    /// The point is accepted.
    pub fn accept(&mut self) {
        self.accepted.clone_from(&self.candidate);
        self.observation = None;
    }

    /// The candidate is thrown away.
    pub fn discard(&mut self) {
        self.candidate.clone_from(&self.accepted);
        self.observation = None;
    }

    /// The analysis starts over from the instance's first state.
    pub fn reset_analysis(&mut self) {
        self.accepted.clone_from(&self.initial);
        self.candidate.clone_from(&self.initial);
        self.observation = None;
        self.output_site = None;
    }

    /// Call `site` with its numeric operands. `files` is the instance's open
    /// file table, when the host gave it one.
    pub fn call(
        &mut self,
        files: Option<&mut Box<dyn AnalogFileTable>>,
        site: u32,
        operands: &[f64],
    ) -> Result<f64, String> {
        let index = site as usize;
        let sites = Arc::clone(&self.sites);
        let Some(call) = sites.get(index) else {
            return Err(format!("system-function site {site} is not in the model"));
        };
        self.outputs.clear();
        self.output_site = Some(site);
        let operand = |position: usize| -> Result<f64, String> {
            operands
                .get(position)
                .copied()
                .ok_or_else(|| format!("{} is missing operand {position}", call.label()))
        };
        let function = call.function;
        if function.is_random() {
            return self.draw(index, call, operands);
        }
        match function {
            AnalogSystemFunction::RealToBits => {
                let bits = operand(0)?.to_bits() as u32;
                Ok(f64::from(bits as i32))
            }
            AnalogSystemFunction::TestPlusargs => {
                Ok(if self.plusargs.get(index).copied().flatten().is_some() {
                    1.0
                } else {
                    0.0
                })
            }
            AnalogSystemFunction::ValuePlusargs => {
                let value = self.plusargs.get(index).copied().flatten();
                self.outputs.push(value);
                Ok(if value.is_some() { 1.0 } else { 0.0 })
            }
            AnalogSystemFunction::Fopen => {
                let Some(table) = self.files(files) else {
                    return Ok(0.0);
                };
                let name = call.text.first().map(String::as_str).unwrap_or_default();
                let mode = call.text.get(1).map(String::as_str);
                Ok(match table.open(name, mode) {
                    AnalogFileOpen::Opened(descriptor) => f64::from(descriptor as i32),
                    AnalogFileOpen::Failed(_) => 0.0,
                })
            }
            AnalogSystemFunction::Fscanf => {
                let descriptor = integer(operand(0)?) as u32;
                let Some(table) = self.files(files) else {
                    self.outputs.resize(call.targets.len(), None);
                    return Ok(-1.0);
                };
                let format = call.text.first().map(String::as_str).unwrap_or_default();
                let scan = table.scan(descriptor, format, &call.targets);
                self.outputs.extend(scan.values.iter().map(|value| {
                    Some(match *value {
                        AnalogScanValue::Integer(value) => f64::from(value),
                        AnalogScanValue::Real(value) => value,
                    })
                }));
                self.outputs.resize(call.targets.len(), None);
                Ok(f64::from(scan.count))
            }
            _ => unreachable!("every function other than a random one is matched"),
        }
    }

    /// Output `index` of the latest call at `site`, or `fallback` (the
    /// target's own value) when the call did not assign it.
    pub fn output(&self, site: u32, index: usize, fallback: f64) -> f64 {
        self.assigned_output(site, index).unwrap_or(fallback)
    }

    /// Output `index` of the latest call at `site`, when it assigned one.
    pub fn assigned_output(&self, site: u32, index: usize) -> Option<f64> {
        if self.output_site != Some(site) {
            return None;
        }
        self.outputs.get(index).copied().flatten()
    }

    /// The table a call reads: the candidate, or an observation's copy of the
    /// accepted one.
    fn files<'a>(
        &'a mut self,
        files: Option<&'a mut Box<dyn AnalogFileTable>>,
    ) -> Option<&'a mut Box<dyn AnalogFileTable>> {
        match &mut self.observation {
            None => files,
            Some(observation) => {
                if observation.files.is_none() {
                    let mut copy = files?.clone_table();
                    copy.begin_evaluation();
                    observation.files = Some(copy);
                }
                observation.files.as_mut()
            }
        }
    }

    fn draw(
        &mut self,
        index: usize,
        call: &AnalogSystemSite,
        operands: &[f64],
    ) -> Result<f64, String> {
        let function = call.function;
        let key = self.keys.key(call.scope);
        let global = call.scope == AnalogDrawScope::Global;
        let seeds = match &mut self.observation {
            Some(observation) => &mut observation.seeds,
            None => &mut self.candidate,
        };
        let (mut seed, parameters) = match call.seed {
            AnalogSeed::Stream if global => (seeds.global, operands),
            AnalogSeed::Stream => (seeds.stream, operands),
            AnalogSeed::Variable => {
                let Some((&seed, parameters)) = operands.split_first() else {
                    return Err(format!("{} has no seed operand", call.label()));
                };
                (integer(seed) ^ key, parameters)
            }
            AnalogSeed::Internal => {
                let Some((&start, parameters)) = operands.split_first() else {
                    return Err(format!("{} has no seed operand", call.label()));
                };
                let internal = seeds.internal.get(index).copied().flatten();
                (internal.unwrap_or_else(|| integer(start)) ^ key, parameters)
            }
        };
        if parameters.len() != function.distribution_parameters() {
            return Err(format!(
                "{} takes {} distribution parameters, not {}",
                call.label(),
                function.distribution_parameters(),
                parameters.len()
            ));
        }
        let real = |position: usize| parameters[position];
        let whole = |position: usize| integer(parameters[position]);
        let integer_draw = |outcome: random::Distribution| match outcome {
            random::Distribution::Value(value) => Ok(f64::from(value)),
            random::Distribution::Invalid(message) => Err(message),
        };
        let value = match function {
            AnalogSystemFunction::Random | AnalogSystemFunction::Arandom => {
                Ok(f64::from(random::random(&mut seed)))
            }
            AnalogSystemFunction::DistUniform => Ok(f64::from(random::dist_uniform(
                &mut seed,
                whole(0),
                whole(1),
            ))),
            AnalogSystemFunction::DistNormal => {
                integer_draw(random::dist_normal(&mut seed, whole(0), whole(1)))
            }
            AnalogSystemFunction::DistExponential => {
                integer_draw(random::dist_exponential(&mut seed, whole(0)))
            }
            AnalogSystemFunction::DistPoisson => {
                integer_draw(random::dist_poisson(&mut seed, whole(0)))
            }
            AnalogSystemFunction::DistChiSquare => {
                integer_draw(random::dist_chi_square(&mut seed, whole(0)))
            }
            AnalogSystemFunction::DistT => integer_draw(random::dist_t(&mut seed, whole(0))),
            AnalogSystemFunction::DistErlang => {
                integer_draw(random::dist_erlang(&mut seed, whole(0), whole(1)))
            }
            AnalogSystemFunction::RdistUniform => {
                Ok(random::rdist_uniform(&mut seed, real(0), real(1)))
            }
            AnalogSystemFunction::RdistNormal => {
                Ok(random::rdist_normal(&mut seed, real(0), real(1)))
            }
            AnalogSystemFunction::RdistExponential => random::rdist_exponential(&mut seed, real(0)),
            AnalogSystemFunction::RdistPoisson => random::rdist_poisson(&mut seed, real(0)),
            AnalogSystemFunction::RdistChiSquare => random::rdist_chi_square(&mut seed, real(0)),
            AnalogSystemFunction::RdistT => random::rdist_t(&mut seed, real(0)),
            AnalogSystemFunction::RdistErlang => random::rdist_erlang(&mut seed, real(0), real(1)),
            _ => unreachable!("draw is called for random functions"),
        }
        .map_err(|message| format!("{}: {message}", call.label()))?;
        match call.seed {
            AnalogSeed::Stream if global => seeds.global = seed,
            AnalogSeed::Stream => seeds.stream = seed,
            AnalogSeed::Variable => self.outputs.push(Some(f64::from(seed ^ key))),
            AnalogSeed::Internal => {
                if let Some(slot) = seeds.internal.get_mut(index) {
                    *slot = Some(seed ^ key);
                }
            }
        }
        Ok(value)
    }

    /// The accepted state, for a checkpoint: the two streams and each
    /// started internal seed.
    pub fn checkpoint_words(&self) -> Vec<u64> {
        let mut words = vec![
            CHECKPOINT_VERSION,
            u64::from(self.accepted.stream as u32),
            u64::from(self.accepted.global as u32),
            self.accepted.internal.len() as u64,
        ];
        for seed in &self.accepted.internal {
            words.push(match seed {
                Some(seed) => (1 << 32) | u64::from(*seed as u32),
                None => 0,
            });
        }
        words
    }

    /// Reopen a checkpoint's accepted state; the candidate restarts from it.
    pub fn restore_checkpoint_words(&mut self, words: &[u64]) -> Result<(), String> {
        let [version, stream, global, count, internal @ ..] = words else {
            return Err("system-function checkpoint is truncated".into());
        };
        if *version != CHECKPOINT_VERSION {
            return Err(format!(
                "system-function checkpoint version {version} is not {CHECKPOINT_VERSION}"
            ));
        }
        if *count as usize != self.sites.len() || internal.len() != self.sites.len() {
            return Err(format!(
                "system-function checkpoint has {count} sites for a model with {}",
                self.sites.len()
            ));
        }
        let mut seeds = Seeds::new(
            *stream as u32 as i32,
            *global as u32 as i32,
            self.sites.len(),
        );
        for (slot, &word) in seeds.internal.iter_mut().zip(internal) {
            *slot = match word >> 32 {
                0 => None,
                1 => Some(word as u32 as i32),
                _ => return Err("system-function checkpoint seed is malformed".into()),
            };
        }
        self.accepted = seeds.clone();
        self.candidate = seeds;
        self.observation = None;
        self.output_site = None;
        Ok(())
    }
}

/// Verilog's real-to-integer conversion: round half away from zero.
fn integer(value: f64) -> i32 {
    if value.is_finite() {
        value.round() as i32
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn site(function: AnalogSystemFunction, seed: AnalogSeed) -> AnalogSystemSite {
        AnalogSystemSite {
            function,
            seed,
            scope: AnalogDrawScope::Unscoped,
            text: Vec::new(),
            targets: Vec::new(),
            file: "t.va".into(),
            line: 1,
        }
    }

    #[test]
    fn a_seed_variable_draws_the_ieee_stream_and_returns_its_new_seed() {
        let mut state = AnalogSystemState::new(
            vec![site(AnalogSystemFunction::Random, AnalogSeed::Variable)].into(),
        );
        state.begin_evaluation();
        let mut seed = 42;
        let expected = random::random(&mut seed);
        assert_eq!(state.call(None, 0, &[42.0]), Ok(f64::from(expected)));
        assert_eq!(state.output(0, 0, 0.0), f64::from(seed));
    }

    #[test]
    fn an_internal_seed_advances_per_call_and_a_rejected_point_leaves_no_draw() {
        let mut state = AnalogSystemState::new(
            vec![site(
                AnalogSystemFunction::RdistNormal,
                AnalogSeed::Internal,
            )]
            .into(),
        );
        let mut seed = 7;
        let first = random::rdist_normal(&mut seed, 0.0, 1.0);
        let second = random::rdist_normal(&mut seed, 0.0, 1.0);
        state.begin_evaluation();
        assert_eq!(state.call(None, 0, &[7.0, 0.0, 1.0]), Ok(first));
        // Newton evaluates the point again: same draw.
        state.begin_evaluation();
        assert_eq!(state.call(None, 0, &[7.0, 0.0, 1.0]), Ok(first));
        state.accept();
        state.begin_evaluation();
        assert_eq!(state.call(None, 0, &[7.0, 0.0, 1.0]), Ok(second));
        // Rejected: the next point draws the second value again.
        state.discard();
        state.begin_evaluation();
        assert_eq!(state.call(None, 0, &[7.0, 0.0, 1.0]), Ok(second));
        // An observation sees the evaluation's value and changes nothing.
        state.begin_observation();
        assert_eq!(state.call(None, 0, &[7.0, 0.0, 1.0]), Ok(second));
        state.begin_observation();
        assert_eq!(state.call(None, 0, &[7.0, 0.0, 1.0]), Ok(second));
    }

    #[test]
    fn the_stream_restores_from_a_checkpoint() {
        let sites: Arc<[AnalogSystemSite]> = vec![
            site(AnalogSystemFunction::Random, AnalogSeed::Stream),
            site(AnalogSystemFunction::Arandom, AnalogSeed::Internal),
        ]
        .into();
        let mut state = AnalogSystemState::new(Arc::clone(&sites));
        state.set_stream_seeds(instance_stream_seed(5, "xa"), 5);
        state.begin_evaluation();
        state.call(None, 0, &[]).unwrap();
        state.call(None, 1, &[3.0]).unwrap();
        state.accept();
        let words = state.checkpoint_words();
        state.begin_evaluation();
        let next = (state.call(None, 0, &[]), state.call(None, 1, &[3.0]));
        let mut restored = AnalogSystemState::new(sites);
        restored.restore_checkpoint_words(&words).unwrap();
        restored.begin_evaluation();
        assert_eq!(
            (restored.call(None, 0, &[]), restored.call(None, 1, &[3.0])),
            next
        );
    }

    /// Two instances draw different streams, each fixed by the run's seed and
    /// the instance's name; `"global"` draws the design's stream, the same in
    /// both, and scopes a seed to the design, where `"instance"` scopes it to
    /// the instance.
    #[test]
    fn streams_are_per_instance_and_a_global_type_string_shares_the_designs() {
        let scoped = |function, seed, scope| AnalogSystemSite {
            scope,
            ..site(function, seed)
        };
        let sites: Arc<[AnalogSystemSite]> = vec![
            site(AnalogSystemFunction::Random, AnalogSeed::Stream),
            scoped(
                AnalogSystemFunction::Arandom,
                AnalogSeed::Stream,
                AnalogDrawScope::Global,
            ),
            scoped(
                AnalogSystemFunction::RdistNormal,
                AnalogSeed::Internal,
                AnalogDrawScope::Global,
            ),
            scoped(
                AnalogSystemFunction::RdistNormal,
                AnalogSeed::Internal,
                AnalogDrawScope::Instance,
            ),
        ]
        .into();
        let draws = |name: &str| {
            let mut state = AnalogSystemState::new(Arc::clone(&sites));
            state.set_stream_seeds(instance_stream_seed(3, name), 3);
            (0..3)
                .map(|_| {
                    state.begin_evaluation();
                    let drawn = [
                        state.call(None, 0, &[]).unwrap(),
                        state.call(None, 1, &[]).unwrap(),
                        state.call(None, 2, &[7.0, 0.0, 1.0]).unwrap(),
                        state.call(None, 3, &[7.0, 0.0, 1.0]).unwrap(),
                    ];
                    state.accept();
                    drawn
                })
                .collect::<Vec<_>>()
        };
        let (a, b) = (draws("xa"), draws("XB"));
        assert_eq!(draws("xa"), a, "a rerun draws the same values");
        assert_eq!(draws("XA"), a, "names are case-insensitive");
        for (a, b) in a.iter().zip(&b) {
            assert_ne!(a[0], b[0], "each instance has its own stream");
            assert_eq!(a[1], b[1], "\"global\" is the design's stream");
            assert_eq!(a[2], b[2], "a \"global\" seed is the design's");
            assert_ne!(a[3], b[3], "an \"instance\" seed is the instance's");
        }
        // The design's stream starts from the run's seed, as the digital
        // kernel's `$random` does.
        let mut seed = 3;
        assert_eq!(a[0][1], f64::from(random::random(&mut seed)));
    }

    #[test]
    fn a_distribution_parameter_outside_its_domain_is_an_error_naming_the_call() {
        let mut state = AnalogSystemState::new(
            vec![site(
                AnalogSystemFunction::RdistExponential,
                AnalogSeed::Internal,
            )]
            .into(),
        );
        state.begin_evaluation();
        let error = state.call(None, 0, &[1.0, -1.0]).unwrap_err();
        assert!(error.starts_with("t.va:1: $rdist_exponential:"), "{error}");
    }

    #[test]
    fn realtobits_keeps_the_low_word_as_icarus_assigns_it_to_an_integer() {
        let mut state = AnalogSystemState::new(
            vec![site(AnalogSystemFunction::RealToBits, AnalogSeed::Stream)].into(),
        );
        state.begin_evaluation();
        // Icarus 12: `j = $realtobits(-2.75e-3)` displays 34359738.
        assert_eq!(state.call(None, 0, &[-2.75e-3]), Ok(34_359_738.0));
        assert_eq!(state.call(None, 0, &[1.5]), Ok(0.0));
    }
}
