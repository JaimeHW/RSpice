//! What the engine refuses about the decks a run dispatches, decided by
//! building them.
//!
//! A structural refusal stated before a run has to be the refusal the engine
//! makes, and nothing the engine accepts may be refused. A second reading of
//! the drawing cannot promise either: it refused coupling capacitors the engine
//! solves and missed loops the engine cannot. So each distinct deck a task
//! dispatches is parsed and built the way the runner builds it, and the built
//! circuit — or the build's own error — says what the engine will refuse.
//! Nothing is solved.

use std::collections::{HashMap, HashSet};

use rspice_core::circuit::DcBranchVoltageLoop;
use rspice_core::engine::ModelBinCoverageGap;
use rspice_simulation_contract::config::OpNodeInitialization;

use super::*;

/// One refusal the engine makes about a dispatched deck.
#[derive(Debug, Clone, PartialEq)]
pub enum EngineRefusalKind {
    /// Nodes an independent source forces a net current into, with no DC path
    /// to ground and no controlled current to balance it
    /// (`CircuitData::fatal_no_dc_path_nodes`). The shared DC-bias boundary
    /// refuses them before solving unless the deck sets `.OPTIONS RSHUNT`.
    /// Unforced capacitive islands are solved and are not listed.
    NoDcPath { nodes: Vec<String> },
    /// A loop of fixed DC branch voltages: independent voltage sources and
    /// DC-shorted inductors.
    BranchVoltageLoop(DcBranchVoltageLoop),
    /// A MOS instance no bin card of its family covers at this deck's
    /// geometry and temperature. Boxed: the gap carries the nearest card's
    /// whole geometry, several times the size of every other refusal.
    UncoveredModelBin(Box<ModelBinCoverageGap>),
    /// A Verilog-AMS connect-rule refusal (clause 7), in the engine's words:
    /// a mixed-discipline boundary the selected rules cannot bridge, or no
    /// single block to select. Apart from [`Self::Build`] because its remedy
    /// is the run's `.OPTIONS CONNECTRULES` as well as the design.
    ConnectRules {
        message: String,
        /// The instance whose port the boundary is. `None` for a refusal
        /// about the rules themselves, which no instance owns.
        instance: Option<String>,
    },
    /// Any other refusal the build raised, in the engine's words.
    Build {
        message: String,
        /// The deck instance the engine named, when it named one.
        instance: Option<String>,
    },
}

/// One task that dispatches a refused deck.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefusedTask {
    /// The authored analysis the task runs.
    pub analysis: AnalysisInstanceId,
    /// The task as the run labels it, which names the point it solves when it
    /// solves one.
    pub label: String,
}

/// One refusal, with every task it refuses.
#[derive(Debug, Clone, PartialEq)]
pub struct EngineRefusal {
    pub kind: EngineRefusalKind,
    /// In dispatch order, each once.
    pub tasks: Vec<RefusedTask>,
}

/// How far a pass over a run's decks has got.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DeckBuildProgress {
    /// Distinct decks built so far.
    pub built: usize,
    /// Distinct decks the pass builds: one per deck and temperature the run
    /// dispatches.
    pub total: usize,
}

/// The temperature the runner applies to a task's deck.
#[derive(Clone, Copy, PartialEq)]
enum RunTemperature {
    /// The deck's own `.TEMP`.
    Deck,
    /// A Run Set point's temperature, set on the parsed deck, of an analysis
    /// whose deck already states it or that takes it from its operating
    /// point.
    Point(f64),
    /// A temperature the runner writes into the source before parsing it
    /// (`runner::spliced_run_temperature`), so parameter evaluation and
    /// `.if` selection see it, and sets on the parsed deck.
    Spliced(f64),
}

impl RunTemperature {
    fn key(self) -> (u8, u64) {
        match self {
            Self::Deck => (0, 0),
            Self::Point(celsius) => (1, celsius.to_bits()),
            Self::Spliced(celsius) => (2, celsius.to_bits()),
        }
    }

    fn celsius(self) -> Option<f64> {
        match self {
            Self::Deck => None,
            Self::Point(celsius) | Self::Spliced(celsius) => Some(celsius),
        }
    }
}

/// What building one deck said.
enum DeckVerdict {
    Built {
        fatal_no_dc_path_nodes: Vec<String>,
        loops: Vec<DcBranchVoltageLoop>,
    },
    Refused(Vec<EngineRefusalKind>),
}

/// What makes two dispatched decks one build: the source, and the
/// temperature it is built at ([`RunTemperature::key`]).
type DeckKey<'a> = (&'a str, (u8, u64));

/// The deck a task dispatches, and the temperature the runner applies to it.
struct DispatchedDeck<'a> {
    source: &'a str,
    temperature: RunTemperature,
}

impl PreparedRunSnapshot {
    /// Every refusal the engine makes when it builds the decks this run
    /// dispatches, each with the tasks it refuses.
    ///
    /// One build per distinct deck and temperature. A refusal that holds at
    /// several points is reported once, listing every task it refuses.
    pub fn engine_refusals(
        &self,
        abort: &dyn rspice_core::abort_signal::AbortSignal,
    ) -> Vec<EngineRefusal> {
        self.engine_refusals_reporting(abort, &|_| {})
    }

    /// [`Self::engine_refusals`], telling `progress` how many distinct decks
    /// it builds and each time it has built one. Empty when `abort` stopped
    /// it.
    pub fn engine_refusals_reporting(
        &self,
        abort: &dyn rspice_core::abort_signal::AbortSignal,
        progress: &dyn Fn(DeckBuildProgress),
    ) -> Vec<EngineRefusal> {
        let mut refusals: Vec<EngineRefusal> = Vec::new();
        // Held until every deck is built: each build reads the sources
        // installed here.
        let _installed = match self.project_veriloga_runtimes.install() {
            Ok(installed) => installed,
            Err(error) => {
                let tasks = self.tasks.iter().map(refused_task).collect();
                refusals.push(EngineRefusal {
                    kind: EngineRefusalKind::Build {
                        message: format!(
                            "Could not install prepared project Verilog-A runtimes: {error}"
                        ),
                        instance: None,
                    },
                    tasks,
                });
                return refusals;
            }
        };

        // Every deck every task dispatches, in dispatch order, so the pass can
        // say how many it builds before it builds the first. A recorded FFT
        // transforms its producer's waveform and builds no circuit of its own.
        // A corner run or temperature step whose points were expanded is not
        // dispatched either: its turn assembles the results of its points,
        // which are the solves (`PointFamilyRegistry::declares`).
        let declarations: HashSet<AnalysisInstanceId> = self
            .tasks
            .iter()
            .filter(|task| task.declared_point.is_some())
            .map(|task| task.authored_instance_id)
            .collect();
        let dispatched: Vec<(&PreparedTask, DispatchedDeck<'_>)> = self
            .tasks
            .iter()
            .filter(|task| {
                !matches!(task.task.spec, AnalysisSpec::Fft { .. })
                    && !declarations.contains(&task.instance_id)
            })
            .map(|task| (task, self.dispatched_deck(task)))
            .collect();
        let total = dispatched
            .iter()
            .map(|(_, deck)| (deck.source, deck.temperature.key()))
            .collect::<HashSet<_>>()
            .len();
        let mut built = 0;
        progress(DeckBuildProgress { built, total });

        let mut verdicts: HashMap<DeckKey<'_>, Option<DeckVerdict>> = HashMap::new();
        for (task, deck) in dispatched {
            if abort.is_aborted() {
                return Vec::new();
            }
            let key = (deck.source, deck.temperature.key());
            let fresh = !verdicts.contains_key(&key);
            let verdict = verdicts.entry(key).or_insert_with_key(|(source, _)| {
                self.build_verdict(source, deck.temperature, abort)
            });
            if fresh {
                built += 1;
                progress(DeckBuildProgress { built, total });
            }
            let Some(verdict) = verdict else {
                continue;
            };
            let refused = refused_task(task);
            for kind in task_refusals(task, verdict) {
                record(&mut refusals, kind, &refused);
            }
        }
        refusals
    }

    /// The deck one task's engine call builds, as the runner composes it: the
    /// deck the task's preparation composed for it, where it composed one,
    /// else the run's. A declared corner point's deck already carries its
    /// process's cards, and the runner adds none.
    fn dispatched_deck<'a>(&'a self, task: &'a PreparedTask) -> DispatchedDeck<'a> {
        let source = task
            .executable_netlist_override
            .as_deref()
            .unwrap_or(&self.executable_netlist);
        let environment = task.execution_environment.as_ref();
        let temperature = match crate::runner::spliced_run_temperature(&task.task, environment) {
            Some(celsius) => RunTemperature::Spliced(celsius),
            None => environment.map_or(RunTemperature::Deck, |point| {
                RunTemperature::Point(point.temperature_celsius)
            }),
        };
        DispatchedDeck {
            source,
            temperature,
        }
    }

    /// Parse and build one deck exactly as the runner does, without solving.
    /// `None` when the pass was stopped.
    fn build_verdict(
        &self,
        source: &str,
        temperature: RunTemperature,
        abort: &dyn rspice_core::abort_signal::AbortSignal,
    ) -> Option<DeckVerdict> {
        let mut limits = self.execution_limits;
        limits.max_netlist_bytes = limits.max_expanded_source_bytes;
        let spliced;
        let source = match temperature {
            RunTemperature::Spliced(celsius) => {
                // A temperature the splice refuses fails the runner's
                // materialization as a configuration error, and is that refusal
                // here.
                spliced = match crate::netlist_preparation::source_with_run_temperature_with_abort(
                    source, celsius, abort,
                ) {
                    Ok(spliced) => spliced,
                    Err(error) if error.is_aborted() => return None,
                    Err(error) => {
                        return Some(DeckVerdict::Refused(vec![EngineRefusalKind::Build {
                            message: error.to_string(),
                            instance: None,
                        }]));
                    }
                };
                spliced.as_str()
            }
            RunTemperature::Deck | RunTemperature::Point(_) => source,
        };
        let mut netlist = match crate::netlist_preparation::parse_analysis_netlist_with_abort(
            source,
            None,
            limits,
            &self.measurement_references,
            abort,
        ) {
            Ok(netlist) => netlist,
            Err(crate::error::SimulationError::Aborted) => return None,
            Err(error) => {
                return Some(DeckVerdict::Refused(vec![EngineRefusalKind::Build {
                    message: error.to_string(),
                    instance: None,
                }]));
            }
        };
        if let Some(celsius) = temperature.celsius() {
            netlist.options.temp = Some(celsius);
        }
        let engine = rspice_core::Engine::new(rspice_core::SimulationConfig {
            resource_limits: limits,
            ..Default::default()
        })
        .resolved_for_netlist(&netlist);
        match engine.build_circuit_with_abort(&netlist, abort) {
            Ok(circuit) => Some(DeckVerdict::Built {
                fatal_no_dc_path_nodes: circuit.fatal_no_dc_path_nodes().to_vec(),
                loops: circuit.dc_branch_voltage_loops(),
            }),
            Err(
                rspice_core::SimulationError::Aborted
                | rspice_core::SimulationError::TimeLimitExceeded,
            ) => None,
            Err(error) => {
                // The scan walks the whole design again, so a Stop ends it as
                // it ends the build. A card it cannot read failed the build
                // too, whose own refusal states it.
                let gaps = match engine.model_bin_coverage_gaps(&netlist, abort) {
                    Ok(gaps) => gaps,
                    Err(
                        rspice_core::SimulationError::Aborted
                        | rspice_core::SimulationError::TimeLimitExceeded,
                    ) => return None,
                    Err(_) => Vec::new(),
                };
                let message = error.to_string();
                // The build stops at the first instance no bin covers. When
                // that is what it stopped on, the gaps say it with the
                // geometry; anything else it stopped on is its own refusal.
                let stopped_on_a_gap = gaps.first().is_some_and(|gap| {
                    message.contains(&format!("MOSFET '{}' geometry is uncovered", gap.element))
                });
                let mut refused: Vec<EngineRefusalKind> = gaps
                    .into_iter()
                    .map(|gap| EngineRefusalKind::UncoveredModelBin(Box::new(gap)))
                    .collect();
                if !stopped_on_a_gap {
                    let instance = error.instance_name().map(str::to_owned);
                    refused.push(match &error {
                        rspice_core::SimulationError::Elaboration(elaboration)
                            if elaboration.kind
                                == rspice_core::ElaborationErrorKind::ConnectRule =>
                        {
                            EngineRefusalKind::ConnectRules { message, instance }
                        }
                        _ => EngineRefusalKind::Build { message, instance },
                    });
                }
                Some(DeckVerdict::Refused(refused))
            }
        }
    }
}

/// What one task refuses of a built deck.
///
/// A build refusal refuses every task that builds the deck. A forced floating
/// node is refused only by the analyses that solve their own deck's DC bias
/// through the shared boundary that checks it before solving. A loop of fixed
/// branch voltages is refused by every analysis that solves a DC point of its
/// own deck, a `.DC` sweep among them: every such solve is singular, whatever
/// the sweep value.
fn task_refusals(task: &PreparedTask, verdict: &DeckVerdict) -> Vec<EngineRefusalKind> {
    match verdict {
        DeckVerdict::Refused(kinds) => kinds.clone(),
        DeckVerdict::Built {
            fatal_no_dc_path_nodes,
            loops,
        } => {
            let checks_dc_bias = checks_dc_bias_before_solving(task);
            let mut refused = Vec::new();
            if checks_dc_bias && !fatal_no_dc_path_nodes.is_empty() {
                refused.push(EngineRefusalKind::NoDcPath {
                    nodes: fatal_no_dc_path_nodes.clone(),
                });
            }
            let solves_dc_point = checks_dc_bias
                || (matches!(task.task.spec, AnalysisSpec::DcSweep { .. })
                    && task.transient_state.is_none()
                    && task.dependency_bindings.is_empty());
            if solves_dc_point {
                refused.extend(
                    loops
                        .iter()
                        .cloned()
                        .map(EngineRefusalKind::BranchVoltageLoop),
                );
            }
            refused
        }
    }
}

/// Whether the task's engine call solves its own deck's DC bias through the
/// shared boundary that refuses a forced floating node before solving
/// (`Engine::ensure_dc_paths_to_ground`).
///
/// Only the analyses the runner hands straight to the engine are claimed: the
/// operating point and the AC, noise, pole-zero and sensitivity analyses
/// seeded by it. A forced `.IC` operating point skips the check; a task bound
/// to a producer's state or to a transient state solves no bias of its own.
/// Transient startup and `.DC` sweeps refuse the same nodes too, but from the
/// `t = 0` waveform and from each sweep value, which a build does not know, so
/// they are left to the engine.
fn checks_dc_bias_before_solving(task: &PreparedTask) -> bool {
    if task.transient_state.is_some() || !task.dependency_bindings.is_empty() {
        return false;
    }
    match &task.task.spec {
        AnalysisSpec::LegacyDcOp
        | AnalysisSpec::Ac { .. }
        | AnalysisSpec::Noise { .. }
        | AnalysisSpec::PoleZero { .. }
        | AnalysisSpec::Sensitivity { .. } => true,
        AnalysisSpec::DcOp {
            node_initialization,
            ..
        } => *node_initialization != OpNodeInitialization::ForceIcValues,
        _ => false,
    }
}

fn refused_task(task: &PreparedTask) -> RefusedTask {
    RefusedTask {
        analysis: task.authored_instance_id,
        label: task.label.clone(),
    }
}

fn record(refusals: &mut Vec<EngineRefusal>, kind: EngineRefusalKind, task: &RefusedTask) {
    match refusals.iter_mut().find(|refusal| refusal.kind == kind) {
        Some(refusal) => {
            if !refusal.tasks.contains(task) {
                refusal.tasks.push(task.clone());
            }
        }
        None => refusals.push(EngineRefusal {
            kind,
            tasks: vec![task.clone()],
        }),
    }
}
