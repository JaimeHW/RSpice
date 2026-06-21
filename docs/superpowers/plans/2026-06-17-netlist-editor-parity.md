# Netlist Editor Parity Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make the RSpice netlist editor execute authored decks, tune parameters, report structured diagnostics, and present VOLTA-quality run feedback from the same source of truth.

**Architecture:** Add an explicit run intent so the Netlist workspace can execute the edited deck while Simulate keeps the generated run-set path. Build a small manual-deck bridge from `rspice_core::AnalysisCommand` into the existing `QueuedAnalysis` pipeline, then layer last-run baselines, tuner summaries, and structured diagnostics on top of the same buffer snapshot. Keep rendering in the existing egui/token system and split new behavior into small testable modules under the current controller and netlist view boundaries.

**Tech Stack:** Rust, egui, `rspice-core` netlist parser, `rspice-ui` simulation controller, existing `ui::plot` Bode helpers, Cargo unit tests.

---

## File Structure

- Create `crates/rspice-ui/src/simulation/controller/manual_deck.rs`: converts parsed deck analyses into controller queue entries and composes manual run source without appending Simulate analysis lines.
- Modify `crates/rspice-ui/src/simulation/controller/mod.rs`: consumes a run intent, branches to manual-deck or Simulate run-set startup, tracks the active manual deck snapshot, and promotes successful baselines.
- Modify `crates/rspice-ui/src/simulation/controller/analysis_plan.rs`: keeps Simulate run-set planning unchanged and exposes helpers needed by the manual-deck bridge.
- Modify `crates/rspice-ui/src/simulation/controller/analysis_commands.rs`: removes manual-source analysis appending from the Netlist Run path while preserving generated Simulate command generation.
- Modify `crates/rspice-ui/src/state/simulation/state_model.rs`: adds `SimulationRunIntent` and manual-run metadata fields.
- Modify `crates/rspice-ui/src/state/simulation/state_impl.rs`: initializes, consumes, and resets the run intent safely.
- Modify `crates/rspice-ui/src/state/mod.rs`: re-exports the run-intent enum for UI callers.
- Create `crates/rspice-ui/src/shell/views/netlist/baseline.rs`: computes line diffs and parameter baselines from last successful manual deck run snapshots.
- Create `crates/rspice-ui/src/shell/views/netlist/summary.rs`: shared AC/measurement summary for mini Bode, "As tuned", and delta chips.
- Create `crates/rspice-ui/src/shell/views/netlist/diagnostics.rs`: editor diagnostic types, parser mapping, lint diagnostics, and fix actions.
- Modify `crates/rspice-ui/src/shell/views/netlist/mod.rs`: wires run intent, progress bar, diff pips, baseline sync, and shared delta chip rendering.
- Modify `crates/rspice-ui/src/shell/views/netlist/editor.rs`: uses structured diagnostics and span-level highlighting.
- Modify `crates/rspice-ui/src/shell/views/netlist/highlight.rs`: accepts diagnostic spans instead of line-only error sets.
- Modify `crates/rspice-ui/src/shell/views/netlist/tuner.rs`: uses annotated ranges, baseline reset, coalesced reruns, mini Bode, and "As tuned".
- Create `crates/rspice-core/src/netlist/source_map.rs`: optional editor source map and reference harvesting.
- Modify `crates/rspice-core/src/netlist/mod.rs`: exports source-map types and span-aware parser helpers without slowing normal parse.
- Modify parser files under `crates/rspice-core/src/netlist/parser/`: record spans for dot commands, definitions, and model/subckt references.

---

### Task 1: Run Intent and Netlist Run Plumbing

**Files:**
- Modify: `crates/rspice-ui/src/state/simulation/state_model.rs`
- Modify: `crates/rspice-ui/src/state/simulation/state_impl.rs`
- Modify: `crates/rspice-ui/src/state/simulation/mod.rs`
- Modify: `crates/rspice-ui/src/state/mod.rs`
- Modify: `crates/rspice-ui/src/shell/views/netlist/mod.rs`
- Modify: `crates/rspice-ui/src/shell/views/simulate.rs`
- Modify: `crates/rspice-ui/src/shell/toolbar.rs`
- Modify: `crates/rspice-ui/src/shell/menubar.rs`
- Modify: `crates/rspice-ui/src/common/app/app_actions.rs`

- [x] **Step 1: Add the failing intent tests**

Add this test module near the bottom of `crates/rspice-ui/src/shell/views/netlist/mod.rs`:

```rust
#[cfg(test)]
mod run_intent_tests {
    use super::*;
    use crate::state::SimulationRunIntent;

    #[test]
    fn netlist_request_run_sets_manual_deck_intent_without_enabled_run_set() {
        let mut state = AppState::default();
        state.sim_setup.enabled.clear();
        state.simulation.netlist_content =
            "deck\nR1 out 0 1k\nV1 out 0 1\n.op\n.end\n".to_string();
        state.workspace.netlist_source = Some(state.simulation.netlist_content.clone());

        request_run(&mut state);

        assert!(state.simulation.trigger_simulation);
        assert_eq!(state.simulation.run_intent, SimulationRunIntent::ManualDeck);
        assert!(!state.shell.netlist.rerun_queued);
    }

    #[test]
    fn netlist_request_run_queues_one_manual_deck_rerun_while_running() {
        let mut state = AppState::default();
        state.simulation.is_running = true;
        state.workspace.netlist_source = Some(".op\n.end\n".to_string());

        request_run(&mut state);
        request_run(&mut state);

        assert!(!state.simulation.trigger_simulation);
        assert_eq!(state.simulation.run_intent, SimulationRunIntent::ManualDeck);
        assert!(state.shell.netlist.rerun_queued);
    }
}
```

- [x] **Step 2: Run tests and confirm they fail**

Run:

```powershell
cargo test -p rspice-ui shell::views::netlist::run_intent_tests --lib
```

Expected: fails because `SimulationRunIntent` and `run_intent` do not exist and `request_run` still returns early on an empty Simulate run set.

- [x] **Step 3: Add the run-intent enum and state field**

In `crates/rspice-ui/src/state/simulation/state_model.rs`, add the enum above `SimulationState`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SimulationRunIntent {
    #[default]
    SimulateRunSet,
    ManualDeck,
}
```

Add this field to `SimulationState` after `trigger_abort`:

```rust
    /// Which workflow requested the next simulation start.
    pub run_intent: SimulationRunIntent,
```

Ensure `crates/rspice-ui/src/state/simulation/mod.rs` re-exports the enum:

```rust
pub use state_model::{SimulationRunIntent, SimulationState};
```

Ensure `crates/rspice-ui/src/state/mod.rs` includes the enum in the existing simulation re-export list:

```rust
pub use simulation::{
    AnalysisResult, AnalysisType, DcOpResult, OperatingPointValue, SimulationRun,
    SimulationRunIntent, SimulationState, WaveformData,
};
```

- [x] **Step 4: Add helper methods for all non-netlist callers**

In `crates/rspice-ui/src/state/simulation/state_impl.rs`, add:

```rust
    pub fn request_simulate_run_set(&mut self) {
        self.run_intent = SimulationRunIntent::SimulateRunSet;
        self.trigger_simulation = true;
    }

    pub fn request_manual_deck_run(&mut self) {
        self.run_intent = SimulationRunIntent::ManualDeck;
        self.trigger_simulation = true;
    }
```

Update Simulate view, toolbar, menubar, automation, and F5 shortcut callers that currently set `trigger_simulation = true` to call `request_simulate_run_set()`. Keep Netlist Run using `request_manual_deck_run()`.

- [x] **Step 5: Replace Netlist Run request gating**

In `crates/rspice-ui/src/shell/views/netlist/mod.rs`, replace `request_run` and `flush_queued_run` with:

```rust
pub(super) fn request_run(state: &mut AppState) {
    state.simulation.run_intent = crate::state::SimulationRunIntent::ManualDeck;
    if state.simulation.is_running {
        state.shell.netlist.rerun_queued = true;
    } else {
        state.simulation.request_manual_deck_run();
    }
}

fn flush_queued_run(state: &mut AppState) {
    if state.shell.netlist.rerun_queued && !state.simulation.is_running {
        state.shell.netlist.rerun_queued = false;
        state.simulation.request_manual_deck_run();
    }
}
```

- [x] **Step 6: Run the intent tests**

Run:

```powershell
cargo test -p rspice-ui shell::views::netlist::run_intent_tests --lib
```

Expected: both tests pass.

- [ ] **Step 7: Commit**

```powershell
git add crates/rspice-ui/src/state/simulation/state_model.rs crates/rspice-ui/src/state/simulation/state_impl.rs crates/rspice-ui/src/state/simulation/mod.rs crates/rspice-ui/src/state/mod.rs crates/rspice-ui/src/shell/views/netlist/mod.rs crates/rspice-ui/src/shell/views/simulate.rs crates/rspice-ui/src/shell/toolbar.rs crates/rspice-ui/src/shell/menubar.rs crates/rspice-ui/src/common/app/app_actions.rs
git commit -m "feat(netlist): distinguish manual deck run intent"
```

---

### Task 2: Manual Deck Analysis Bridge

**Files:**
- Create: `crates/rspice-ui/src/simulation/controller/manual_deck.rs`
- Modify: `crates/rspice-ui/src/simulation/controller/mod.rs`
- Modify: `crates/rspice-ui/src/simulation/controller/analysis_plan.rs`
- Modify: `crates/rspice-ui/src/simulation/controller/analysis_commands.rs`

- [x] **Step 1: Add conversion tests**

Create `crates/rspice-ui/src/simulation/controller/manual_deck.rs` with this initial test module and public signatures:

```rust
use super::*;
use rspice_core::netlist::{AnalysisCommand, FreqVariation};

pub(super) fn build_manual_deck_queue(
    state: &AppState,
    source: &str,
) -> Result<Vec<QueuedAnalysis>, Vec<String>> {
    let _ = (state, source);
    unimplemented!("manual deck bridge will be implemented in this task")
}

pub(super) fn compose_manual_deck_source(source: &str) -> String {
    let _ = source;
    unimplemented!("manual deck source composition will be implemented in this task")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::multi_run::FrequencySweep;

    fn specs_for(source: &str) -> Vec<AnalysisSpec> {
        let state = AppState::default();
        build_manual_deck_queue(&state, source)
            .expect("manual deck queue")
            .into_iter()
            .map(|q| q.spec)
            .collect()
    }

    #[test]
    fn manual_deck_preserves_common_analysis_order() {
        let specs = specs_for(
            "deck\nR1 out 0 1k\nV1 out 0 1 AC 1\n.op\n.ac dec 20 1 1g\n.tran 1n 1u\n.end\n",
        );

        assert!(matches!(specs[0], AnalysisSpec::DcOp));
        assert!(matches!(
            specs[1],
            AnalysisSpec::Ac {
                start_freq,
                stop_freq,
                points_per_unit: 20,
                sweep: FrequencySweep::Decade
            } if (start_freq - 1.0).abs() < 1e-12 && (stop_freq - 1e9).abs() < 1.0
        ));
        assert!(matches!(
            specs[2],
            AnalysisSpec::Transient {
                step_time,
                stop_time,
                start_time,
                max_timestep: None,
                uic: false
            } if (step_time - 1e-9).abs() < 1e-21
                && (stop_time - 1e-6).abs() < 1e-18
                && start_time == 0.0
        ));
    }

    #[test]
    fn manual_deck_dc_and_noise_build_configs_without_dialog_state() {
        let state = AppState::default();
        let queue = build_manual_deck_queue(
            &state,
            "deck\nV1 in 0 0 AC 1\nR1 in out 1k\n.ac lin 5 1 5\n.noise v(out) V1 dec 10 1 1e6\n.end\n",
        )
        .expect("queue builds");

        assert!(matches!(queue[0].spec, AnalysisSpec::Ac { .. }));
        assert!(matches!(queue[1].config, Some(AnalysisConfig::Noise(_))));
    }

    #[test]
    fn manual_deck_source_does_not_append_analysis_lines() {
        let source = "deck\nR1 out 0 1k\n.op\n.end\n";
        assert_eq!(compose_manual_deck_source(source), source);
    }

    #[test]
    fn manual_deck_adds_end_only_when_missing() {
        assert_eq!(compose_manual_deck_source("deck\n.op"), "deck\n.op\n.end\n");
    }

    #[test]
    fn manual_deck_reports_no_analysis() {
        let state = AppState::default();
        let err = build_manual_deck_queue(&state, "deck\nR1 a 0 1k\n.end\n")
            .expect_err("no analysis should fail");
        assert!(err.iter().any(|e| e.contains("No analysis command in netlist")));
    }
}
```

- [x] **Step 2: Register the module and run failing tests**

In `crates/rspice-ui/src/simulation/controller/mod.rs`, add:

```rust
mod manual_deck;
```

Run:

```powershell
cargo test -p rspice-ui simulation::controller::manual_deck --lib
```

Expected: tests compile but fail at the `unimplemented!` calls.

- [x] **Step 3: Implement manual source composition**

Replace `compose_manual_deck_source` with:

```rust
pub(super) fn compose_manual_deck_source(source: &str) -> String {
    let has_end = source
        .lines()
        .any(|line| line.trim_start().eq_ignore_ascii_case(".end"));
    if has_end {
        source.to_string()
    } else if source.ends_with('\n') {
        format!("{source}.end\n")
    } else {
        format!("{source}\n.end\n")
    }
}
```

- [x] **Step 4: Implement frequency and analysis conversion helpers**

Add these helpers in `manual_deck.rs`:

```rust
fn frequency_sweep(variation: FreqVariation) -> FrequencySweep {
    match variation {
        FreqVariation::Lin => FrequencySweep::Linear,
        FreqVariation::Oct => FrequencySweep::Octave,
        FreqVariation::Dec => FrequencySweep::Decade,
    }
}

fn ac_sweep(variation: FreqVariation) -> AcSweepType {
    match variation {
        FreqVariation::Lin => AcSweepType::Linear,
        FreqVariation::Oct => AcSweepType::Octave,
        FreqVariation::Dec => AcSweepType::Decade,
    }
}

fn command_to_queue_item(
    state: &AppState,
    command: &AnalysisCommand,
) -> Result<QueuedAnalysis, String> {
    let spec_options = SpecExecutionOptions::default();
    match command {
        AnalysisCommand::Op => Ok(QueuedAnalysis {
            spec: AnalysisSpec::DcOp,
            config: Some(AnalysisConfig::DcOp),
            spec_options,
            analysis_line: ".op".to_string(),
        }),
        AnalysisCommand::Ac {
            variation,
            points,
            start_freq,
            stop_freq,
        } => {
            let spec = AnalysisSpec::Ac {
                start_freq: *start_freq,
                stop_freq: *stop_freq,
                points_per_unit: *points,
                sweep: frequency_sweep(*variation),
            };
            Ok(QueuedAnalysis {
                config: Some(AnalysisConfig::Ac(AcAnalysisConfig {
                    start_freq: *start_freq,
                    stop_freq: *stop_freq,
                    num_points: *points,
                    sweep_type: ac_sweep(*variation),
                })),
                analysis_line: ".ac".to_string(),
                spec,
                spec_options,
            })
        }
        AnalysisCommand::Tran {
            step,
            stop,
            start,
            max_step,
            uic,
        } => {
            let spec = AnalysisSpec::Transient {
                stop_time: *stop,
                step_time: *step,
                start_time: start.unwrap_or(0.0),
                max_timestep: *max_step,
                uic: *uic,
            };
            Ok(QueuedAnalysis {
                config: Some(AnalysisConfig::Transient(TransientAnalysisConfig {
                    stop_time: *stop,
                    step_time: *step,
                    start_time: start.unwrap_or(0.0),
                    max_timestep: *max_step,
                    uic: *uic,
                })),
                analysis_line: ".tran".to_string(),
                spec,
                spec_options,
            })
        }
        AnalysisCommand::Noise {
            output_node,
            reference_node,
            input_source,
            variation,
            points,
            start_freq,
            stop_freq,
        } => {
            let spec = AnalysisSpec::Noise {
                output_node: output_node.clone(),
                start_freq: *start_freq,
                stop_freq: *stop_freq,
                points_per_decade: *points,
                temperature: state.sim_setup.options.temp + 273.15,
            };
            Ok(QueuedAnalysis {
                config: Some(AnalysisConfig::Noise(NoiseAnalysisConfig {
                    output_node: output_node.clone(),
                    reference_node: reference_node.clone().unwrap_or_default(),
                    input_source: input_source.clone(),
                    sweep_type: ac_sweep(*variation),
                    num_points: *points,
                    start_freq: *start_freq,
                    stop_freq: *stop_freq,
                })),
                analysis_line: ".noise".to_string(),
                spec,
                spec_options,
            })
        }
        other => Err(format!(
            "{:?} cannot be run from the netlist editor until its execution parameters can be preserved",
            std::mem::discriminant(other)
        )),
    }
}
```

Add these additional match arms before the final `other =>` arm:

```rust
        AnalysisCommand::Dc {
            source,
            start,
            stop,
            step,
            sweep2,
        } => {
            let (source2, start2, stop2, step2) = match sweep2 {
                Some(second) => (
                    Some(second.source.clone()),
                    Some(second.start),
                    Some(second.stop),
                    Some(second.step),
                ),
                None => (None, None, None, None),
            };
            let spec = AnalysisSpec::DcSweep {
                source_name: source.clone(),
                start: *start,
                stop: *stop,
                step: *step,
                source2,
                start2,
                stop2,
                step2,
            };
            Ok(QueuedAnalysis {
                config: Some(AnalysisConfig::DcSweep(DcSweepConfig {
                    source: source.clone(),
                    start: *start,
                    stop: *stop,
                    step: *step,
                    source2: spec_source2(&spec),
                    start2: spec_start2(&spec),
                    stop2: spec_stop2(&spec),
                    step2: spec_step2(&spec),
                })),
                analysis_line: ".dc".to_string(),
                spec,
                spec_options,
            })
        }
        AnalysisCommand::PoleZero {
            input_pos,
            input_neg,
            output_pos,
            output_neg,
            transfer_type,
            analysis_type,
        } => {
            let transfer_type = match transfer_type {
                rspice_core::netlist::PoleZeroTransferType::Voltage => "VOL",
                rspice_core::netlist::PoleZeroTransferType::Current => "CUR",
            }
            .to_string();
            let analysis_type = match analysis_type {
                rspice_core::netlist::PoleZeroAnalysisType::PoleZero => "PZ",
                rspice_core::netlist::PoleZeroAnalysisType::PolesOnly => "POL",
                rspice_core::netlist::PoleZeroAnalysisType::ZerosOnly => "ZER",
            }
            .to_string();
            let spec = AnalysisSpec::PoleZero {
                input_node: input_pos.clone(),
                input_ref: input_neg.clone(),
                output_node: output_pos.clone(),
                output_ref: output_neg.clone(),
                transfer_type: transfer_type.clone(),
                analysis_type: analysis_type.clone(),
            };
            Ok(QueuedAnalysis {
                config: Some(AnalysisConfig::PoleZero(PoleZeroConfig {
                    input_node: input_pos.clone(),
                    input_ref: input_neg.clone(),
                    output_node: output_pos.clone(),
                    output_ref: output_neg.clone(),
                    transfer_type,
                    analysis_type: match analysis_type.as_str() {
                        "POL" => PzAnalysisType::PolesOnly,
                        "ZER" => PzAnalysisType::ZerosOnly,
                        _ => PzAnalysisType::PoleZero,
                    },
                })),
                analysis_line: ".pz".to_string(),
                spec,
                spec_options,
            })
        }
        AnalysisCommand::Sensitivity {
            output_node,
            reference_node,
            ac_sweep,
        } => {
            if ac_sweep.is_some() {
                return Err(
                    ".sens AC sweep cannot be launched from the netlist editor without losing sweep parameters"
                        .to_string(),
                );
            }
            let output_var = reference_node
                .as_ref()
                .map(|reference| format!("v({output_node},{reference})"))
                .unwrap_or_else(|| format!("v({output_node})"));
            let spec = AnalysisSpec::Sensitivity {
                output_var: output_var.clone(),
                ac_mode: false,
                frequency: None,
            };
            Ok(QueuedAnalysis {
                config: Some(AnalysisConfig::Sensitivity(SensitivityConfig {
                    output_var,
                    ac_mode: false,
                    frequency: None,
                })),
                analysis_line: ".sens".to_string(),
                spec,
                spec_options,
            })
        }
        AnalysisCommand::Stb {
            points,
            start_freq,
            stop_freq,
            probe,
            ..
        } => Ok(QueuedAnalysis {
            spec: AnalysisSpec::Stb {
                probe_node: probe.clone(),
                start_freq: *start_freq,
                stop_freq: *stop_freq,
                points_per_decade: *points,
            },
            config: None,
            spec_options,
            analysis_line: ".stb".to_string(),
        }),
        AnalysisCommand::Disto {
            variation,
            points,
            start_freq,
            stop_freq,
            f2_over_f1,
        } => Ok(QueuedAnalysis {
            spec: AnalysisSpec::Disto {
                start_freq: *start_freq,
                stop_freq: *stop_freq,
                points_per_unit: *points,
                sweep: frequency_sweep(*variation),
                f2_over_f1: *f2_over_f1,
            },
            config: None,
            spec_options,
            analysis_line: ".disto".to_string(),
        }),
        AnalysisCommand::Tf { .. } => Ok(QueuedAnalysis {
            spec: AnalysisSpec::Tf,
            config: None,
            spec_options,
            analysis_line: ".tf".to_string(),
        }),
        AnalysisCommand::Four {
            fundamental,
            outputs,
            num_harmonics,
        } => {
            let Some(output_node) = outputs.first() else {
                return Err(".four requires at least one output".to_string());
            };
            if outputs.len() > 1 {
                return Err(
                    ".four with multiple outputs is refused until all outputs can be preserved"
                        .to_string(),
                );
            }
            Ok(QueuedAnalysis {
                spec: AnalysisSpec::Fourier {
                    fundamental_freq: *fundamental,
                    num_harmonics: *num_harmonics,
                    output_node: output_node.clone(),
                    output_ref: "0".to_string(),
                    start_time: 0.0,
                    stop_time: 0.0,
                },
                config: None,
                spec_options,
                analysis_line: ".four".to_string(),
            })
        }
        AnalysisCommand::Step(_) => Err(
            ".step deck execution is refused until the manual path can expand the sweep without altering the authored deck"
                .to_string(),
        ),
        AnalysisCommand::Temp { .. } => Err(
            ".temp deck execution is refused until the manual path can preserve the deck temperature list"
                .to_string(),
        ),
        AnalysisCommand::MonteCarlo(_) => Err(
            ".mc deck execution is refused until the manual path can preserve Monte Carlo deck parameters"
                .to_string(),
        ),
```

After adding the `.dc` arm, add these small accessors below `command_to_queue_item` so the `DcSweepConfig` receives the same optional second-sweep values as the spec:

```rust
fn spec_source2(spec: &AnalysisSpec) -> Option<String> {
    match spec {
        AnalysisSpec::DcSweep { source2, .. } => source2.clone(),
        _ => None,
    }
}

fn spec_start2(spec: &AnalysisSpec) -> Option<f64> {
    match spec {
        AnalysisSpec::DcSweep { start2, .. } => *start2,
        _ => None,
    }
}

fn spec_stop2(spec: &AnalysisSpec) -> Option<f64> {
    match spec {
        AnalysisSpec::DcSweep { stop2, .. } => *stop2,
        _ => None,
    }
}

fn spec_step2(spec: &AnalysisSpec) -> Option<f64> {
    match spec {
        AnalysisSpec::DcSweep { step2, .. } => *step2,
        _ => None,
    }
}
```

- [x] **Step 5: Implement queue building**

Replace `build_manual_deck_queue` with:

```rust
pub(super) fn build_manual_deck_queue(
    state: &AppState,
    source: &str,
) -> Result<Vec<QueuedAnalysis>, Vec<String>> {
    let netlist = rspice_core::Netlist::parse(source)
        .map_err(|err| vec![format!("Netlist parse failed: {err}")])?;
    if netlist.analyses.is_empty() {
        return Err(vec![
            "No analysis command in netlist; add .op, .ac, .tran, or another supported analysis."
                .to_string(),
        ]);
    }

    let mut queue = Vec::with_capacity(netlist.analyses.len());
    let mut errors = Vec::new();
    for command in &netlist.analyses {
        match command_to_queue_item(state, command) {
            Ok(item) => queue.push(item),
            Err(err) => errors.push(err),
        }
    }

    if errors.is_empty() {
        Ok(queue)
    } else {
        Err(errors)
    }
}
```

- [x] **Step 6: Run bridge tests**

Run:

```powershell
cargo test -p rspice-ui simulation::controller::manual_deck --lib
```

Expected: all tests pass.

- [ ] **Step 7: Commit**

```powershell
git add crates/rspice-ui/src/simulation/controller/manual_deck.rs crates/rspice-ui/src/simulation/controller/mod.rs
git commit -m "feat(netlist): convert deck analyses for manual runs"
```

---

### Task 3: Controller Manual-Deck Execution Path

**Files:**
- Modify: `crates/rspice-ui/src/simulation/controller/mod.rs`
- Modify: `crates/rspice-ui/src/simulation/controller/analysis_commands.rs`
- Modify: `crates/rspice-ui/src/simulation/controller/manual_deck.rs`

- [x] **Step 1: Add controller tests for manual deck startup**

Add this test module near the bottom of `crates/rspice-ui/src/simulation/controller/mod.rs`:

```rust
#[cfg(test)]
mod manual_deck_start_tests {
    use super::*;
    use crate::state::SimulationRunIntent;

    #[test]
    fn manual_deck_start_uses_deck_queue_with_empty_simulate_set() {
        let mut controller = SimulationController::new();
        let mut state = AppState::default();
        state.sim_setup.enabled.clear();
        state.simulation.run_intent = SimulationRunIntent::ManualDeck;
        state.workspace.netlist_source = Some(
            "deck\nV1 out 0 1 AC 1\nR1 out 0 1k\n.ac dec 3 1 1k\n.end\n".to_string(),
        );

        let queue = controller
            .build_start_queue_for_test(&mut state)
            .expect("manual deck queue");

        assert_eq!(queue.len(), 1);
        assert!(matches!(queue[0].spec, AnalysisSpec::Ac { .. }));
    }

    #[test]
    fn simulate_run_set_still_refuses_empty_enabled_set() {
        let mut controller = SimulationController::new();
        let mut state = AppState::default();
        state.sim_setup.enabled.clear();
        state.simulation.run_intent = SimulationRunIntent::SimulateRunSet;

        let err = controller
            .build_start_queue_for_test(&mut state)
            .expect_err("empty Simulate run set should fail");

        assert!(err.iter().any(|e| e.contains("Nothing in the run set")));
    }
}
```

Expose this test-only helper inside `impl SimulationController`:

```rust
#[cfg(test)]
fn build_start_queue_for_test(
    &mut self,
    state: &mut AppState,
) -> Result<Vec<QueuedAnalysis>, Vec<String>> {
    self.build_start_queue(state)
}
```

- [x] **Step 2: Run tests and confirm failure**

Run:

```powershell
cargo test -p rspice-ui simulation::controller::manual_deck_start_tests --lib
```

Expected: fails because `build_start_queue` does not exist and `start_simulation` still has one Simulate-only path.

- [x] **Step 3: Extract queue selection**

In `crates/rspice-ui/src/simulation/controller/mod.rs`, add:

```rust
fn build_start_queue(&self, state: &mut AppState) -> Result<Vec<QueuedAnalysis>, Vec<String>> {
    match state.simulation.run_intent {
        crate::state::SimulationRunIntent::ManualDeck => {
            let source = state
                .workspace
                .netlist_source
                .as_deref()
                .unwrap_or(state.simulation.netlist_content.as_str());
            manual_deck::build_manual_deck_queue(state, source)
        }
        crate::state::SimulationRunIntent::SimulateRunSet => {
            if state.sim_setup.enabled.is_empty() {
                return Err(vec![
                    "Nothing in the run set - tick an analysis in the Simulate view".to_string(),
                ]);
            }
            let plan = self.build_analysis_plan(state)?;
            self.build_queue_from_plan(state, &plan)
        }
    }
}
```

Refactor `start_simulation` so it calls `build_start_queue(state)` before netlist composition and logs returned errors.

- [x] **Step 4: Branch netlist composition by intent**

Inside `start_simulation`, replace the existing manual-source composition block with:

```rust
let intent = state.simulation.run_intent;
let mut netlist = match intent {
    crate::state::SimulationRunIntent::ManualDeck => {
        let source = state
            .workspace
            .netlist_source
            .clone()
            .unwrap_or_else(|| state.simulation.netlist_content.clone());
        state.push_sim_message(ConsoleMessage::info(
            "Running netlist editor deck".to_string(),
        ));
        manual_deck::compose_manual_deck_source(&source)
    }
    crate::state::SimulationRunIntent::SimulateRunSet => {
        let hierarchy = crate::simulation::netlist_gen::HierarchySource::from_workspace(
            &state.library_manager,
            &state.workspace.schematic_buffers,
        );
        let result = crate::simulation::netlist_gen::generate_netlist_hierarchical(
            &state.schematic,
            &analysis_lines,
            &hierarchy,
        );
        if !result.errors.is_empty() {
            for err in result.errors {
                state.push_sim_message(ConsoleMessage::error(err));
            }
            state.simulation.status = "Netlist error".to_string();
            return;
        }
        for warning in result.warnings {
            state.push_sim_message(ConsoleMessage::warning(warning));
        }
        state.schematic.net_mapping = result.point_to_net.clone();
        state.simulation.cross_probe.update(
            result.point_to_net,
            result.nets,
            result.net_segments,
        );
        result.netlist
    }
};
```

After the run has been queued, reset the intent:

```rust
state.simulation.run_intent = crate::state::SimulationRunIntent::SimulateRunSet;
```

- [x] **Step 5: Retire manual analysis appending**

Keep `compose_manual_netlist` only if some generated path still uses it. If no caller remains, delete it from `analysis_commands.rs`. If a caller remains, rename it to `compose_generated_netlist_with_analysis_lines` so no Netlist Run path can accidentally call it.

- [x] **Step 6: Run controller tests**

Run:

```powershell
cargo test -p rspice-ui simulation::controller::manual_deck_start_tests simulation::controller::manual_deck --lib
```

Expected: all tests pass.

- [ ] **Step 7: Commit**

```powershell
git add crates/rspice-ui/src/simulation/controller/mod.rs crates/rspice-ui/src/simulation/controller/analysis_commands.rs crates/rspice-ui/src/simulation/controller/manual_deck.rs
git commit -m "feat(netlist): execute manual deck run path"
```

Completed verification for Tasks 1-3:

```powershell
cargo test -p rspice-ui shell::views::netlist::run_intent_tests --lib -- --nocapture
cargo test -p rspice-ui simulation::controller::manual_deck --lib -- --nocapture
cargo test -p rspice-ui simulation::controller::tests --lib -- --nocapture
cargo test -p rspice-ui common::app::tests --lib -- --nocapture
cargo test -p rspice-ui --lib
```

Result: all listed commands passed; full `rspice-ui` library suite passed 224/224.

---

### Task 4: Last-Run Baseline and True Diff Pips

Implementation note: completed in this pass for exact manual-run buffer snapshots and line diff pips. Numeric `.param` baselines and reset-to-last-run remain in Task 5 with tuner reset work.

**Files:**
- Create: `crates/rspice-ui/src/shell/views/netlist/baseline.rs`
- Modify: `crates/rspice-ui/src/shell/views/netlist/mod.rs`
- Modify: `crates/rspice-ui/src/shell/views/netlist/editor.rs`
- Modify: `crates/rspice-ui/src/simulation/controller/mod.rs`

- [x] **Step 1: Add baseline tests**

Create `crates/rspice-ui/src/shell/views/netlist/baseline.rs`:

```rust
use std::collections::{HashMap, HashSet};

use crate::properties::engineering::parse_engineering_value;

pub(super) fn changed_lines_since_baseline(current: &str, baseline: Option<&str>) -> HashSet<usize> {
    let _ = (current, baseline);
    unimplemented!("line diff will be implemented in this task")
}

pub(super) fn param_values(buffer: &str) -> HashMap<String, f64> {
    let _ = buffer;
    unimplemented!("param baseline parsing will be implemented in this task")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn changed_lines_compare_against_snapshot() {
        let baseline = "deck\n.param r=1k\nR1 a 0 {r}\n.op\n.end\n";
        let current = "deck\n.param r=2k\nR1 a 0 {r}\n.op\n.end\n";
        assert_eq!(changed_lines_since_baseline(current, Some(baseline)), HashSet::from([1]));
    }

    #[test]
    fn changed_lines_empty_when_no_baseline() {
        assert!(changed_lines_since_baseline("deck\n.op\n", None).is_empty());
    }

    #[test]
    fn param_values_parse_numeric_assignments_case_insensitively() {
        let values = param_values(".param Itail=20u cl = 2p expr={w*2}\n");
        assert!((values["itail"] - 20e-6).abs() < 1e-15);
        assert!((values["cl"] - 2e-12).abs() < 1e-21);
        assert!(!values.contains_key("expr"));
    }
}
```

- [x] **Step 2: Register the module and run failing tests**

In `netlist/mod.rs`, add:

```rust
pub(crate) mod baseline;
```

Run:

```powershell
cargo test -p rspice-ui shell::views::netlist::baseline --lib
```

Expected: fails at `unimplemented!`.

- [x] **Step 3: Implement line baseline helper**

Replace both functions with:

```rust
pub(crate) fn changed_lines_since_baseline(current: &str, baseline: Option<&str>) -> HashSet<usize> {
    let Some(baseline) = baseline else {
        return HashSet::new();
    };
    let current_lines: Vec<&str> = current.split('\n').collect();
    let baseline_lines: Vec<&str> = baseline.split('\n').collect();
    let max_len = current_lines.len().max(baseline_lines.len());
    (0..max_len)
        .filter(|&idx| current_lines.get(idx) != baseline_lines.get(idx))
        .collect()
}

pub(crate) fn param_values(buffer: &str) -> HashMap<String, f64> {
    let mut out = HashMap::new();
    for line in buffer.lines() {
        if let Some(assignments) = super::tuner::scan_assignments_for_baseline(line) {
            for (name, start, end) in assignments {
                let raw = &line[start..end];
                if raw.starts_with('{') {
                    continue;
                }
                if let Ok(value) = parse_engineering_value(raw) {
                    out.insert(name.to_ascii_lowercase(), value);
                }
            }
        }
    }
    out
}
```

Expose a small wrapper in `tuner.rs`:

```rust
pub(crate) fn scan_assignments_for_baseline(line: &str) -> Option<Vec<(String, usize, usize)>> {
    scan_assignments(line)
}
```

- [x] **Step 4: Add manual run snapshot fields**

Extend `NetlistEditorState` in `netlist/mod.rs`:

```rust
    /// Exact buffer snapshot used by the last successful manual deck run.
    pub last_run_buffer: Option<String>,
    /// Numeric `.param` values from `last_run_buffer`.
    pub last_run_params: HashMap<String, f64>,
    /// Buffer captured when the current manual deck run started.
    pub pending_run_buffer: Option<String>,
    /// Run id associated with the pending manual deck snapshot.
    pub pending_run_id: Option<u64>,
```

- [x] **Step 5: Promote baseline on successful manual deck completion**

In `SimulationController::start_simulation`, after `state.simulation.start_run()`, capture manual pending state:

```rust
if intent == crate::state::SimulationRunIntent::ManualDeck {
    if let Some(run) = state.simulation.active_run() {
        state.shell.netlist.pending_run_id = Some(run.id);
    }
    state.shell.netlist.pending_run_buffer = Some(netlist.clone());
}
```

In `finish_simulation_batch`, after `complete_run()` and before clearing controller state:

```rust
if run_success {
    if let (Some(run), Some(pending_id), Some(buffer)) = (
        state.simulation.active_run(),
        state.shell.netlist.pending_run_id,
        state.shell.netlist.pending_run_buffer.clone(),
    ) {
        if run.id == pending_id {
            state.shell.netlist.last_run_params = crate::shell::views::netlist::baseline::param_values(&buffer);
            state.shell.netlist.last_run_buffer = Some(buffer);
            state.shell.netlist.edited_lines.clear();
        }
    }
}
state.shell.netlist.pending_run_id = None;
state.shell.netlist.pending_run_buffer = None;
```

- [x] **Step 6: Compute diff pips from snapshots**

In `editor.rs`, replace the cloned `edited_lines` source with:

```rust
let edited_lines = baseline::changed_lines_since_baseline(
    &state.simulation.netlist_content,
    state.shell.netlist.last_run_buffer.as_deref(),
);
```

Keep inserts into `netlist.edited_lines` during typing only as transient feedback before a baseline exists, or remove that field once all callers use snapshot diffs.

- [x] **Step 7: Run tests**

Run:

```powershell
cargo test -p rspice-ui shell::views::netlist::baseline --lib
cargo test -p rspice-ui shell::views::netlist --lib
```

Expected: all tests pass.

- [ ] **Step 8: Commit**

```powershell
git add crates/rspice-ui/src/shell/views/netlist/baseline.rs crates/rspice-ui/src/shell/views/netlist/mod.rs crates/rspice-ui/src/shell/views/netlist/editor.rs crates/rspice-ui/src/shell/views/netlist/tuner.rs crates/rspice-ui/src/simulation/controller/mod.rs
git commit -m "feat(netlist): baseline editor diffs on successful runs"
```

Completed verification for Task 4 line snapshot/diff-pip slice:

```powershell
cargo test -p rspice-ui shell::views::netlist::baseline --lib -- --nocapture
cargo test -p rspice-ui simulation::controller::tests --lib -- --nocapture
cargo test -p rspice-ui shell::views::netlist --lib -- --nocapture
cargo test -p rspice-ui --lib
```

Result: all listed commands passed; full `rspice-ui` library suite passed 229/229.

---

### Task 5: Tuner Annotations, Reset, and Coalesced Reruns

**Files:**
- Modify: `crates/rspice-ui/src/shell/views/netlist/tuner.rs`
- Modify: `crates/rspice-ui/src/shell/views/netlist/mod.rs`

- [x] **Step 1: Add range annotation tests**

Add tests to `tuner.rs`:

```rust
#[test]
fn tune_annotation_by_name_sets_exact_range() {
    let rows = scan_params("* @tune itail 5u..60u\n.param itail=20u\n");
    assert_eq!(rows[0].range, Some((5e-6, 60e-6)));
}

#[test]
fn tune_annotation_before_param_targets_next_assignment() {
    let rows = scan_params("* @tune 0.5p..8p\n.param cl=2p\n");
    assert_eq!(rows[0].range, Some((0.5e-12, 8e-12)));
}

#[test]
fn reset_payload_uses_last_run_values_only_for_numeric_params() {
    let rows = scan_params(".param a=2 b={expr}\n");
    let mut baseline = std::collections::HashMap::new();
    baseline.insert("a".to_string(), 1.0);
    assert_eq!(reset_values(&rows, &baseline), vec![("a".to_string(), 1.0)]);
}
```

- [x] **Step 2: Run tests and confirm failure**

Run:

```powershell
cargo test -p rspice-ui shell::views::netlist::tuner --lib
```

Expected: fails because `ParamRow` lacks `range` and `reset_values` does not exist.

- [x] **Step 3: Extend `ParamRow`**

Update `ParamRow` in `tuner.rs`:

```rust
struct ParamRow {
    name: String,
    line: usize,
    value: Option<f64>,
    raw: String,
    range: Option<(f64, f64)>,
    baseline: Option<f64>,
}
```

- [x] **Step 4: Parse `@tune` comments**

Add:

```rust
fn parse_tune_comment(line: &str) -> Option<(Option<String>, (f64, f64))> {
    let text = line.trim_start().strip_prefix('*')?.trim();
    let tail = text.strip_prefix("@tune")?.trim();
    let mut parts = tail.split_whitespace();
    let first = parts.next()?;
    let (name, range_text) = if first.contains("..") {
        (None, first)
    } else {
        (Some(first.to_ascii_lowercase()), parts.next()?)
    };
    let (lo, hi) = range_text.split_once("..")?;
    let lo = parse_engineering_value(lo).ok()?;
    let hi = parse_engineering_value(hi).ok()?;
    (hi > lo).then_some((name, (lo, hi)))
}
```

Replace the start of `scan_params` with this range-aware loop:

```rust
fn scan_params(buffer: &str) -> Vec<ParamRow> {
    let mut rows = Vec::new();
    let mut named_ranges: HashMap<String, (f64, f64)> = HashMap::new();
    let mut pending_next_range: Option<(f64, f64)> = None;

    for (idx, line) in buffer.lines().enumerate() {
        if let Some((name, range)) = parse_tune_comment(line) {
            if let Some(name) = name {
                named_ranges.insert(name, range);
            } else {
                pending_next_range = Some(range);
            }
            continue;
        }

        let Some(assignments) = scan_assignments(line) else {
            continue;
        };
        for (name, start, end) in assignments {
            let raw = line[start..end].to_owned();
            let value = if raw.starts_with('{') {
                None
            } else {
                parse_engineering_value(&raw).ok()
            };
            let key = name.to_ascii_lowercase();
            let range = named_ranges.remove(&key).or_else(|| pending_next_range.take());
            rows.push(ParamRow {
                name,
                line: idx,
                value,
                raw,
                range,
                baseline: None,
            });
        }
    }

    rows
}
```

- [x] **Step 5: Make slider range prefer annotations**

Change `slider_range` call sites to:

```rust
let (lo, hi) = row
    .range
    .unwrap_or_else(|| slider_range(state, &row.name, value));
```

- [x] **Step 6: Add reset-to-last-run action**

Add helper:

```rust
fn reset_values(rows: &[ParamRow], baseline: &HashMap<String, f64>) -> Vec<(String, f64)> {
    rows.iter()
        .filter(|row| row.value.is_some())
        .filter_map(|row| baseline.get(&row.name.to_ascii_lowercase()).map(|v| (row.name.clone(), *v)))
        .collect()
}
```

In `right_panel`, after tuner rows:

```rust
let reset = crate::ui::widgets::Button::new("reset to last run")
    .enabled(!state.shell.netlist.last_run_params.is_empty())
    .show(ui);
if reset.clicked() {
    let values = reset_values(&rows, &state.shell.netlist.last_run_params);
    for (name, value) in values {
        if let Some(row) = rows.iter().find(|row| row.name.eq_ignore_ascii_case(&name)) {
            apply_param_edit(ui, state, row, value, false);
        }
    }
    super::request_run(state);
}
```

- [x] **Step 7: Run tuner tests**

Run:

```powershell
cargo test -p rspice-ui shell::views::netlist::tuner --lib
```

Expected: all tuner tests pass.

- [ ] **Step 8: Commit**

```powershell
git add crates/rspice-ui/src/shell/views/netlist/tuner.rs crates/rspice-ui/src/shell/views/netlist/mod.rs
git commit -m "feat(netlist): add annotated tuner ranges and reset"
```

Completed verification for Task 5:

```powershell
cargo test -p rspice-ui shell::views::netlist::tuner --lib -- --nocapture
cargo test -p rspice-ui shell::views::netlist --lib -- --nocapture
cargo test -p rspice-ui --lib
cargo fmt --check
cargo check -p rspice-ui
cargo check -p rspice-ui --target wasm32-unknown-unknown
```

Result: all listed commands passed; full `rspice-ui` library suite passed 232/232 after dead-code cleanup.

---

### Task 6: Shared Summary, Mini Bode, and Delta Chips

Implementation note: completed in this pass for shared measurement deltas and spec-aware docbar delta chips. AC curve summary, mini Bode, and "As tuned" rendering remain open.

Sidecar audit note: current AC results are converted into `|signal|` magnitude traces and `phase(signal)` degree traces in `simulation/controller/results_convert.rs`. The full Bode viewer computes/caches stability metrics privately in `shell/results/bode.rs` / `shell/results/mod.rs`, so netlist summaries cannot safely reuse those values unless the Results view has already populated its cache. The next production slice should extract a UI-free helper under `state/simulation` (for example `ac_bode.rs`) that accepts `AnalysisResult` or `SimulationRun`, resolves the AC magnitude/phase pair, converts magnitude to dB, computes ADC gain, UGF, PM, f180, GM, and f3db, and returns the same values to both Results Bode and netlist "As tuned" rendering. Add tests for log-frequency crossing interpolation, missing phase, trace selection, and identical values between Results Bode and netlist summary.

**Files:**
- Create: `crates/rspice-ui/src/shell/views/netlist/summary.rs`
- Modify: `crates/rspice-ui/src/shell/views/netlist/mod.rs`
- Modify: `crates/rspice-ui/src/shell/views/netlist/tuner.rs`
- Modify: `crates/rspice-ui/src/shell/results/bode.rs`
- Modify: `crates/rspice-ui/src/shell/results/mod.rs`

- [x] **Step 1: Add shared measurement summary tests**

Create `summary.rs` with:

```rust
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(super) struct StabilitySummary {
    pub adc_db: Option<f64>,
    pub ugf: Option<f64>,
    pub pm_deg: Option<f64>,
    pub gm_db: Option<f64>,
    pub f180: Option<f64>,
    pub f3db: Option<f64>,
}

pub(super) fn crossing(frequency: &[f64], series: &[f64], level: f64) -> Option<f64> {
    let _ = (frequency, series, level);
    unimplemented!("crossing will be extracted from Bode in this task")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crossing_interpolates_log_frequency() {
        let f = [1.0, 10.0, 100.0];
        let y = [20.0, 0.0, -20.0];
        assert_eq!(crossing(&f, &y, 0.0), Some(10.0));
    }
}
```

- [x] **Step 2: Run tests and confirm failure**

Run:

```powershell
cargo test -p rspice-ui shell::views::netlist::summary --lib
```

Expected: fails at `unimplemented!`.

Evidence (2026-06-17): `cargo test -p rspice-ui shell::views::netlist::summary --lib -- --nocapture` first failed on the new `active_run_summary_carries_measurements_and_ac_metrics` test with the summary stub returning `None`. `cargo test -p rspice-ui shell::views::netlist::tuner --lib -- --nocapture` then failed because `TunedMetricRow`, `as_tuned_rows`, and `mini_bode_spec` did not exist.

- [x] **Step 3: Move crossing math into shared module**

Replace `crossing` with the exact logic currently in `shell/results/bode.rs`:

```rust
pub(super) fn crossing(frequency: &[f64], series: &[f64], level: f64) -> Option<f64> {
    let n = frequency.len().min(series.len());
    for i in 1..n {
        let (y0, y1) = (series[i - 1] - level, series[i] - level);
        if y0 == 0.0 {
            return Some(frequency[i - 1]);
        }
        if y0 * y1 < 0.0 {
            let t = y0 / (y0 - y1);
            let (l0, l1) = (frequency[i - 1].log10(), frequency[i].log10());
            return Some(10f64.powf(l0 + t * (l1 - l0)));
        }
    }
    None
}
```

In `bode.rs`, remove the local `crossing` function and replace all calls with:

```rust
use crate::shell::views::netlist::summary::crossing;
```

Make `summary` visible from `netlist/mod.rs` with:

```rust
pub(crate) mod summary;
```

Evidence (2026-06-17): the shared AC/Bode extraction is implemented in `crates/rspice-ui/src/state/simulation/ac_bode.rs` as `log_frequency_crossing`, `ac_bode_summary_for_run`, and `AcBodeMetrics`, with Bode and netlist summary consumers reading the same module.

- [x] **Step 4: Build run summary**

Add to `summary.rs`:

```rust
pub(super) struct NetlistRunSummary {
    pub stability: StabilitySummary,
    pub measurements: HashMap<String, (String, f64)>,
}

pub(super) fn active_run_summary(state: &mut crate::common::AppState) -> Option<NetlistRunSummary> {
    let run = state.simulation.active_run()?;
    let mut measurements = HashMap::new();
    for analysis in &run.analyses {
        for measurement in &analysis.measurements {
            if let Some(value) = measurement.value {
                measurements.insert(
                    measurement.name.to_ascii_lowercase(),
                    (measurement.name.clone(), value),
                );
            }
        }
    }

    Some(NetlistRunSummary {
        stability: StabilitySummary::default(),
        measurements,
    })
}
```

Add this helper in `summary.rs` and call it from `active_run_summary` when an AC analysis is present:

```rust
fn stability_from_curves(frequency: &[f64], gain_db: &[f64], phase_deg: Option<&[f64]>) -> StabilitySummary {
    let adc_db = gain_db.first().copied();
    let ugf = crossing(frequency, gain_db, 0.0);
    let f3db = adc_db.and_then(|adc| crossing(frequency, gain_db, adc - 3.0));
    let (pm_deg, f180, gm_db) = match phase_deg {
        Some(phase) => {
            let pm = ugf.map(|f| 180.0 + crate::ui::plot::sample_at(frequency, phase, f));
            let f180 = crossing(frequency, phase, -180.0);
            let gm = f180.map(|f| -crate::ui::plot::sample_at(frequency, gain_db, f));
            (pm, f180, gm)
        }
        None => (None, None, None),
    };
    StabilitySummary {
        adc_db,
        ugf,
        pm_deg,
        gm_db,
        f180,
        f3db,
    }
}
```

Evidence (2026-06-17): `crates/rspice-ui/src/shell/views/netlist/summary.rs` now exposes `NetlistRunSummary` and `active_run_summary`, carrying shared AC/Bode metrics, optional Bode curves, and successful measurements. `measurement_deltas` and `bode_deltas` now read through the same `run_summary` path.

- [x] **Step 5: Render mini Bode and "As tuned"**

In `tuner.rs`, after the reset action, call:

```rust
if let Some(summary) = super::summary::active_run_summary(state) {
    render_mini_bode(ui, state, &summary);
    render_as_tuned(ui, &summary);
}
```

Add these render helpers in `tuner.rs` and use the existing token colors:

```rust
fn render_as_tuned(ui: &mut Ui, summary: &super::summary::NetlistRunSummary) {
    section_header(ui, "As tuned", None);
    let rows = [
        ("UGF", fmt_opt(summary.stability.ugf, |v| crate::ui::plot::fmt_si(v, "Hz", 2)), true),
        ("PM", fmt_opt(summary.stability.pm_deg, |v| format!("{v:.1} deg")), true),
        ("GM", fmt_opt(summary.stability.gm_db, |v| format!("{v:.1} dB")), false),
    ];
    let t = Tokens::get(ui.ctx());
    for (name, value, highlight) in rows {
        ui.horizontal(|ui| {
            ui.add_space(8.0);
            ui.label(
                egui::RichText::new(name)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new(value)
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        .color(if highlight { t.color.accent } else { t.color.text }),
                );
            });
        });
    }
}

fn fmt_opt(value: Option<f64>, f: impl FnOnce(f64) -> String) -> String {
    value.map(f).unwrap_or_else(|| "-".to_string())
}
```

For `render_mini_bode`, use `ui.allocate_ui(egui::vec2(ui.available_width(), 130.0), |ui| { ... })`, build a `PlotSpec` with the AC frequency/gain arrays carried on `NetlistRunSummary`, and call `crate::ui::plot::show(ui, &spec, &mut state.shell.results.cache, None, None)`.

Evidence (2026-06-17): `crates/rspice-ui/src/shell/views/netlist/tuner.rs` now calls `active_run_summary`, renders a 130px mini Bode plot from `mini_bode_spec`, and renders the `As tuned` UGF/PM/GM rows. Added tests cover row formatting and dual-axis mini Bode spec construction.

- [x] **Step 6: Point docbar delta chips at the same summary**

Replace `run_measurements` and ad hoc delta chip extraction in `mod.rs` with `summary::active_run_summary` plus previous-run summary lookup. Keep `delta_verdict` behavior but move it into `summary.rs` for tests.

- [x] **Step 7: Run measurement summary tests**

Run:

```powershell
cargo test -p rspice-ui shell::views::netlist::summary --lib
cargo test -p rspice-ui shell::views::netlist --lib
```

Expected: all netlist summary tests pass.

- [ ] **Step 8: Commit**

```powershell
git add crates/rspice-ui/src/shell/views/netlist/summary.rs crates/rspice-ui/src/shell/views/netlist/mod.rs crates/rspice-ui/src/shell/views/netlist/tuner.rs crates/rspice-ui/src/shell/results/bode.rs crates/rspice-ui/src/shell/results/mod.rs
git commit -m "feat(netlist): share tuned summary and mini bode"
```

Completed verification for Task 6 measurement-summary slice:

```powershell
cargo test -p rspice-ui shell::views::netlist::summary --lib -- --nocapture
cargo test -p rspice-ui shell::views::netlist --lib -- --nocapture
cargo test -p rspice-ui --lib
cargo fmt --check
cargo check -p rspice-ui
cargo check -p rspice-ui --target wasm32-unknown-unknown
```

Result: all listed commands passed; full `rspice-ui` library suite passed 234/234.

Additional verification for the mini-Bode/As-tuned slice (2026-06-17):

```powershell
cargo test -p rspice-ui shell::views::netlist::summary --lib -- --nocapture
cargo test -p rspice-ui shell::views::netlist::tuner --lib -- --nocapture
cargo test -p rspice-ui shell::views::netlist --lib -- --nocapture
cargo test -p rspice-ui --lib
cargo fmt --all -- --check
$env:RUSTFLAGS='-D warnings'; cargo check -p rspice-ui --target wasm32-unknown-unknown
cargo clippy -p rspice-ui --all-targets --message-format short -- -D warnings
```

Result: summary tests passed 5/5, tuner tests passed 9/9, netlist tests passed 29/29, full `rspice-ui` library suite passed 269/269, wasm check passed with `-D warnings`, and clippy passed with `-D warnings`.

---

### Task 7: Structured Diagnostics and Span Highlighting

Implementation note: completed in this pass for structured UI diagnostics, parser-error conversion, severity-aware gutter/strip rendering, span-aware underline splitting, and span-derived line/column fallback. Source-map backed lint diagnostics remain Task 8.

**Files:**
- Create: `crates/rspice-ui/src/shell/views/netlist/diagnostics.rs`
- Modify: `crates/rspice-ui/src/shell/views/netlist/mod.rs`
- Modify: `crates/rspice-ui/src/shell/views/netlist/editor.rs`
- Modify: `crates/rspice-ui/src/shell/views/netlist/highlight.rs`

- [x] **Step 1: Add diagnostic type tests**

Create `diagnostics.rs`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DiagnosticSeverity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DiagnosticFix {
    pub label: String,
    pub span: std::ops::Range<usize>,
    pub replacement: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Diagnostic {
    pub severity: DiagnosticSeverity,
    pub span: Option<std::ops::Range<usize>>,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub message: String,
    pub fix: Option<DiagnosticFix>,
}

pub(super) fn line_column_for_span(buffer: &str, offset: usize) -> (usize, usize) {
    let _ = (buffer, offset);
    unimplemented!("line-column mapping will be implemented in this task")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_column_maps_byte_offsets() {
        assert_eq!(line_column_for_span("a\nbc\n", 3), (1, 1));
    }
}
```

- [x] **Step 2: Run tests and confirm failure**

Run:

```powershell
cargo test -p rspice-ui shell::views::netlist::diagnostics --lib
```

Expected: fails at `unimplemented!`.

- [x] **Step 3: Implement line-column mapping**

Replace `line_column_for_span` with:

```rust
pub(super) fn line_column_for_span(buffer: &str, offset: usize) -> (usize, usize) {
    let mut line = 0usize;
    let mut line_start = 0usize;
    for (idx, ch) in buffer.char_indices() {
        if idx >= offset {
            break;
        }
        if ch == '\n' {
            line += 1;
            line_start = idx + ch.len_utf8();
        }
    }
    (line, offset.saturating_sub(line_start))
}
```

- [x] **Step 4: Replace the old diagnostic struct**

Remove the old `Diagnostic` struct from `mod.rs` and import:

```rust
mod diagnostics;
pub(super) use diagnostics::{Diagnostic, DiagnosticFix, DiagnosticSeverity};
```

Update every diagnostic creation in `editor.rs` to construct the new shape. Parser errors without spans use `span: None`, `column: None`, and severity `Error`.

- [x] **Step 5: Make highlighting span-aware**

Change `highlight::layout_job` signature:

```rust
pub fn layout_job(
    text: &str,
    font: FontId,
    c: &Palette,
    diagnostics: &[super::Diagnostic],
) -> LayoutJob
```

Build a `HashMap<usize, Vec<Range<usize>>>` from diagnostics and underline only segments that intersect a diagnostic range. Keep whole-line underline only for diagnostics without a span but with a line.

- [x] **Step 6: Render strip severity and fix labels**

In `diagnostics_strip`, render severity color via:

```rust
let sev_color = match diagnostic.severity {
    DiagnosticSeverity::Error => c.err,
    DiagnosticSeverity::Warning => c.warn,
    DiagnosticSeverity::Info => c.text_dim,
};
```

Append `diagnostic.fix.as_ref().map(|f| f.label.as_str())` as an accent label at the right edge.

- [x] **Step 7: Run tests**

Run:

```powershell
cargo test -p rspice-ui shell::views::netlist::diagnostics --lib
cargo test -p rspice-ui shell::views::netlist --lib
```

Expected: all netlist tests pass.

- [ ] **Step 8: Commit**

```powershell
git add crates/rspice-ui/src/shell/views/netlist/diagnostics.rs crates/rspice-ui/src/shell/views/netlist/mod.rs crates/rspice-ui/src/shell/views/netlist/editor.rs crates/rspice-ui/src/shell/views/netlist/highlight.rs
git commit -m "feat(netlist): structure editor diagnostics"
```

Completed verification for Task 7 structured-diagnostics slice:

```powershell
cargo test -p rspice-ui shell::views::netlist::diagnostics --lib -- --nocapture
cargo test -p rspice-ui shell::views::netlist::highlight --lib -- --nocapture
cargo test -p rspice-ui shell::views::netlist --lib -- --nocapture
cargo fmt --check
cargo test -p rspice-ui --lib
cargo check -p rspice-ui
cargo check -p rspice-ui --target wasm32-unknown-unknown
git diff --check -- crates/rspice-ui/src/shell/views/netlist/diagnostics.rs crates/rspice-ui/src/shell/views/netlist/editor.rs crates/rspice-ui/src/shell/views/netlist/highlight.rs crates/rspice-ui/src/shell/views/netlist/mod.rs
```

Result: all listed commands passed; full `rspice-ui` library suite passed 239/239. `git diff --check` reported only LF-to-CRLF working-copy notices.

---

### Task 8: Core Source Map and Unknown-Model Lint

Implementation note: completed in this pass with a lightweight editor scanner that preserves byte spans across physical-line continuations, records definition/reference scope for subckt-local model visibility, and feeds unknown model/subckt diagnostics into the netlist editor after clean parses. The scanner maps semiconductor, switch, legacy transmission-line, XSPICE, subckt, and explicit passive `MODEL=...` references; bare passive identifiers are intentionally not linted as model references to avoid false positives on parameterized values such as `R1 in out rload`.

**Files:**
- Create: `crates/rspice-core/src/netlist/source_map.rs`
- Modify: `crates/rspice-core/src/netlist/mod.rs`
- Modify: `crates/rspice-core/src/netlist/parser/elements.rs`
- Modify: `crates/rspice-core/src/netlist/parser/commands.rs`
- Modify: `crates/rspice-ui/src/shell/views/netlist/diagnostics.rs`
- Modify: `crates/rspice-ui/src/shell/views/netlist/editor.rs`

- [x] **Step 1: Add source-map tests**

Create `source_map.rs`:

```rust
use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReferenceKind {
    Model,
    Subcircuit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetlistReference {
    pub name: String,
    pub kind: ReferenceKind,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NetlistSourceMap {
    pub model_defs: Vec<(String, Range<usize>)>,
    pub subckt_defs: Vec<(String, Range<usize>)>,
    pub references: Vec<NetlistReference>,
}

pub fn source_map_for_editor(buffer: &str) -> NetlistSourceMap {
    let _ = buffer;
    unimplemented!("editor source map will be implemented in this task")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_map_records_mos_model_reference_span() {
        let src = "deck\nM1 d g s b nch W=1u L=1u\n.model nch nmos\n.end\n";
        let map = source_map_for_editor(src);
        let reference = map.references.iter().find(|r| r.name.eq_ignore_ascii_case("nch")).unwrap();
        assert_eq!(&src[reference.span.clone()], "nch");
    }

    #[test]
    fn source_map_records_subckt_reference_span() {
        let src = "deck\nX1 a b inv\n.subckt inv a b\n.ends\n.end\n";
        let map = source_map_for_editor(src);
        let reference = map.references.iter().find(|r| r.kind == ReferenceKind::Subcircuit).unwrap();
        assert_eq!(&src[reference.span.clone()], "inv");
    }
}
```

- [x] **Step 2: Register module and run failing tests**

In `netlist/mod.rs`:

```rust
pub mod source_map;
pub use source_map::{NetlistReference, NetlistSourceMap, ReferenceKind};
```

Run:

```powershell
cargo test -p rspice-core netlist::source_map
```

Expected: fails at `unimplemented!`.

- [x] **Step 3: Implement a lightweight editor source scanner**

Implement `source_map_for_editor` as a physical-line scanner first. It must handle comments and continuation lines by keeping byte offsets:

```rust
pub fn source_map_for_editor(buffer: &str) -> NetlistSourceMap {
    let mut map = NetlistSourceMap::default();
    let mut offset = 0usize;
    for line in buffer.split_inclusive('\n') {
        let raw = line.trim_end_matches('\n');
        let trimmed = raw.trim_start();
        let lead = raw.len() - trimmed.len();
        if trimmed.starts_with('*') || trimmed.is_empty() {
            offset += line.len();
            continue;
        }
        let lower = trimmed.to_ascii_lowercase();
        if lower.starts_with(".model ") {
            if let Some((name_start, name_end)) = nth_token_span(trimmed, 1) {
                map.model_defs.push((
                    trimmed[name_start..name_end].to_string(),
                    offset + lead + name_start..offset + lead + name_end,
                ));
            }
        } else if lower.starts_with(".subckt ") {
            if let Some((name_start, name_end)) = nth_token_span(trimmed, 1) {
                map.subckt_defs.push((
                    trimmed[name_start..name_end].to_string(),
                    offset + lead + name_start..offset + lead + name_end,
                ));
            }
        } else if let Some(reference) = reference_from_element_line(trimmed, offset + lead) {
            map.references.push(reference);
        }
        offset += line.len();
    }
    map
}
```

Add these helper functions below `source_map_for_editor`:

```rust
fn nth_token_span(line: &str, n: usize) -> Option<(usize, usize)> {
    let mut index = 0usize;
    let mut in_token = false;
    let mut token_start = 0usize;
    let mut token_index = 0usize;
    for (byte, ch) in line.char_indices() {
        if ch.is_whitespace() || ch == ',' {
            if in_token {
                if token_index == n {
                    return Some((token_start, byte));
                }
                token_index += 1;
                in_token = false;
            }
            index = byte + ch.len_utf8();
            continue;
        }
        if !in_token {
            in_token = true;
            token_start = byte;
        }
        index = byte + ch.len_utf8();
    }
    if in_token && token_index == n {
        Some((token_start, index))
    } else {
        None
    }
}

fn reference_from_element_line(line: &str, base: usize) -> Option<NetlistReference> {
    let first = line.chars().next()?.to_ascii_uppercase();
    let (kind, token_index) = match first {
        'D' => (ReferenceKind::Model, 3),
        'Q' => (ReferenceKind::Model, 4),
        'M' => (ReferenceKind::Model, 5),
        'J' | 'Z' => (ReferenceKind::Model, 4),
        'S' => (ReferenceKind::Model, 5),
        'W' => (ReferenceKind::Model, 4),
        'X' => {
            let spans = token_spans(line);
            let candidate = spans
                .iter()
                .rev()
                .find(|(start, end)| !line[*start..*end].contains('='))?;
            return Some(NetlistReference {
                name: line[candidate.0..candidate.1].to_string(),
                kind: ReferenceKind::Subcircuit,
                span: base + candidate.0..base + candidate.1,
            });
        }
        _ => return None,
    };
    let (start, end) = nth_token_span(line, token_index)?;
    Some(NetlistReference {
        name: line[start..end].to_string(),
        kind,
        span: base + start..base + end,
    })
}

fn token_spans(line: &str) -> Vec<(usize, usize)> {
    let mut spans = Vec::new();
    let mut start = None;
    for (byte, ch) in line.char_indices() {
        if ch.is_whitespace() || ch == ',' {
            if let Some(s) = start.take() {
                spans.push((s, byte));
            }
        } else if start.is_none() {
            start = Some(byte);
        }
    }
    if let Some(s) = start {
        spans.push((s, line.len()));
    }
    spans
}
```

- [x] **Step 4: Add unknown-reference lint tests**

In `diagnostics.rs`, add:

```rust
pub(super) fn unknown_reference_diagnostics(buffer: &str) -> Vec<Diagnostic> {
    let _ = buffer;
    unimplemented!("unknown-reference lint will be implemented in this task")
}

#[test]
fn unknown_model_lint_suggests_nearest_known_model() {
    let src = "deck\nM1 d g s b nchh W=1u L=1u\n.model nch nmos\n.end\n";
    let diagnostics = unknown_reference_diagnostics(src);
    assert_eq!(diagnostics.len(), 1);
    assert!(diagnostics[0].message.contains("nchh"));
    assert_eq!(diagnostics[0].fix.as_ref().unwrap().replacement, "nch");
}
```

- [x] **Step 5: Implement lint**

Use `rspice_core::netlist::source_map_for_editor(buffer)`. Build definition sets from `model_defs` and `subckt_defs`. For each reference missing from the corresponding set, compute a simple Levenshtein distance against known definitions and add a fix only when distance is 1 or 2 and the known-name set is not empty. Skip the lint if the buffer contains `.include` or `.lib` because editor-only file IO is not authoritative.

- [x] **Step 6: Merge lints into parse diagnostics**

In `editor.rs::parse_buffer`, after a clean parse, append `unknown_reference_diagnostics(buffer)` to the returned diagnostics. On parser error, do not run the lint.

- [x] **Step 7: Run tests**

Run:

```powershell
cargo test -p rspice-core netlist::source_map
cargo test -p rspice-ui shell::views::netlist::diagnostics --lib
```

Expected: all tests pass.

- [ ] **Step 8: Commit**

```powershell
git add crates/rspice-core/src/netlist/source_map.rs crates/rspice-core/src/netlist/mod.rs crates/rspice-ui/src/shell/views/netlist/diagnostics.rs crates/rspice-ui/src/shell/views/netlist/editor.rs
git commit -m "feat(netlist): add source-map backed model lint"
```

Completed verification for Task 8 source-map-backed lint slice:

```powershell
cargo test -p rspice-core netlist::source_map --lib -- --nocapture
cargo test -p rspice-ui shell::views::netlist::diagnostics --lib -- --nocapture
cargo test -p rspice-ui shell::views::netlist::editor --lib -- --nocapture
cargo test -p rspice-ui shell::views::netlist --lib -- --nocapture
cargo fmt --check
cargo check -p rspice-core
cargo check -p rspice-ui
cargo test -p rspice-core --lib
cargo test -p rspice-ui --lib
cargo check -p rspice-ui --target wasm32-unknown-unknown
git diff --check -- crates/rspice-core/src/netlist/source_map.rs crates/rspice-core/src/netlist/mod.rs crates/rspice-ui/src/shell/views/netlist/diagnostics.rs crates/rspice-ui/src/shell/views/netlist/editor.rs
```

Result: all listed commands passed; `rspice-core --lib` passed 325 tests with 4 ignored, and `rspice-ui --lib` passed 244/244. `git diff --check` reported only LF-to-CRLF working-copy notices.

---

### Task 9: Netlist Run Progress Bar and Visual Polish

**Files:**
- Modify: `crates/rspice-ui/src/simulation/controller/mod.rs`
- Modify: `crates/rspice-ui/src/shell/views/netlist/mod.rs`
- Modify: `crates/rspice-ui/src/shell/views/netlist/tuner.rs`
- Modify: `crates/rspice-ui/src/shell/views/netlist/editor.rs`

- [x] **Step 1: Keep UI progress state current**

In `SimulationController::update`, after `self.poll_completion(state);`, add:

```rust
state.simulation.progress = self
    .runner
    .progress_fraction()
    .map(f64::from)
    .unwrap_or_else(|| if state.simulation.is_running { 0.08 } else { 0.0 });
```

- [x] **Step 2: Add docbar progress rendering helper**

In `netlist/mod.rs`, add:

```rust
fn run_progress(ui: &mut Ui, state: &AppState) {
    if !state.simulation.is_running {
        return;
    }
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let label = if state.simulation.status.is_empty() {
        "running".to_string()
    } else {
        state.simulation.status.clone()
    };
    let (rect, _) = ui.allocate_exact_size(egui::vec2(120.0, 4.0), egui::Sense::hover());
    ui.painter().rect_filled(rect, 2.0, c.bg_inset);
    let progress = state.simulation.progress.clamp(0.0, 1.0) as f32;
    let fill = egui::Rect::from_min_max(
        rect.min,
        egui::pos2(rect.left() + rect.width() * progress.max(0.08), rect.bottom()),
    );
    ui.painter().rect_filled(fill, 2.0, c.accent);
    ui.label(
        egui::RichText::new(label)
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(c.text_dim),
    );
}
```

- [x] **Step 3: Wire progress into the docbar**

Call `run_progress(ui, state)` near the Run button in `show_docbar`. Keep it compact and do not create a card.

- [x] **Step 4: Ensure text fits in tuner**

For each tuner row, keep name and value on one horizontal row, but clamp the slider width:

```rust
ui.spacing_mut().slider_width = (ui.available_width() - 24.0).clamp(80.0, 220.0);
```

Use `theme::mono(tokens::FS_0, ...)` for bounds and annotations so long parameter names do not crowd the panel.

- [x] **Step 5: Verify compilation**

Run:

```powershell
cargo check -p rspice-ui
```

Expected: command exits 0.

Completed verification for Task 9 progress/layout slice:

```powershell
cargo test -p rspice-ui simulation::controller::tests::ui_progress_fraction_uses_runner_fraction_or_running_floor --lib -- --exact --nocapture
cargo test -p rspice-ui shell::views::netlist::run_intent_tests::progress_label_falls_back_to_running_when_status_is_empty --lib -- --exact --nocapture
cargo test -p rspice-ui shell::views::netlist::tuner::tests::slider_width_is_bounded_for_compact_panels --lib -- --exact --nocapture
cargo test -p rspice-ui shell::views::netlist --lib -- --nocapture
```

Result: focused controller/docbar/tuner tests passed; full netlist view suite passed 26/26. Broader formatting/native/wasm verification is tracked under Task 10 and rerun after this slice.

- [ ] **Step 6: Commit**

```powershell
git add crates/rspice-ui/src/simulation/controller/mod.rs crates/rspice-ui/src/shell/views/netlist/mod.rs crates/rspice-ui/src/shell/views/netlist/tuner.rs crates/rspice-ui/src/shell/views/netlist/editor.rs
git commit -m "feat(netlist): polish run progress and tuner layout"
```

---

### Task 10: Final Verification and Conference-Quality QA

**Files:**
- Verify: entire workspace
- Update if needed: `docs/superpowers/plans/2026-06-17-netlist-editor-parity.md`

- [x] **Step 1: Run formatting**

```powershell
cargo fmt --check
```

Expected: exits 0. If it fails, run `cargo fmt`, inspect `git diff`, and rerun `cargo fmt --check`.

- [x] **Step 2: Run core checks**

```powershell
cargo check -p rspice-core
cargo check -p rspice-ui
```

Expected: both exit 0.

- [x] **Step 3: Run focused tests**

```powershell
cargo test -p rspice-core netlist
cargo test -p rspice-ui shell::views::netlist --lib
cargo test -p rspice-ui simulation::controller --lib
```

Expected: all focused suites pass.

- [x] **Step 4: Run full UI library tests**

```powershell
cargo test -p rspice-ui --lib
```

Expected: exits 0.

Evidence (2026-06-17): `cargo fmt --all -- --check`, `cargo check -p rspice-core`, `cargo check -p rspice-ui`, `cargo test -p rspice-core netlist --lib`, `cargo test -p rspice-ui shell::views::netlist --lib -- --nocapture`, `cargo test -p rspice-ui simulation::controller --lib`, and `cargo test -p rspice-ui --lib` all passed. Counts: core netlist 49/49, UI netlist 29/29, simulation controller 12/12, full UI library 269/269.

- [x] **Step 5: Manual QA checklist**

Use the app or available UI harness to verify:

```text
1. Netlist Run executes a manual deck with .ac and .tran while Simulate enabled set is empty.
2. Simulate Run still executes the Simulate run set and generated schematic netlist.
3. Manual deck Run does not append duplicate .ac or .tran lines.
4. Live tuner slider updates source text, queues one rerun while busy, and refreshes summary.
5. On-release tuner mode waits until drag commit.
6. Reset to last run rewrites tuned params and reruns once.
7. Diff pips clear only after a successful run of the current buffer.
8. Syntax errors, unknown model lints, gutter pips, span highlight, and strip rows agree.
9. Mini Bode and full Bode report matching UGF and margins.
10. Right panel text and docbar progress fit without overlap at normal desktop widths.
```

Manual/browser QA update (2026-06-17): in-app browser smoke against the rebuilt wasm IDE verified a manually pasted `.op` + `.ac` + `.tran` + `.meas` deck runs from the Netlist tab with the Simulate enabled set empty, populates the Results badge, mini Bode, "As tuned" rows, and console run log without warning/error console entries. On-release tuner drag updated `.param rload`, queued one rerun after drag commit, refreshed the summary/delta chips, and produced no console warnings/errors. A diagnostic deck with unknown MOS model `nchh` showed matching gutter marker, span underline, and strip row with a suggested fix. Full Results/Bode views displayed the same AC/transient run data and dashed stability metrics as the mini Bode for the flat/no-crossing case. A 390x844 browser smoke initially exposed Netlist docbar clipping and a crowded diagnostic quick-fix suffix; TDD fixes added width-aware `Full`/`Compact`/`Minimal` docbar presentations and bounded diagnostic-strip text so phone width shows compact `manual`/`Copy`/`Regen`/`Run` actions and truncates long diagnostic rows with `...` instead of clipping partial text. The same QA/review pass found and fixed newest-first run-history delta ordering and manual deck source authority when Simulate options are non-default. Follow-up code review found no Critical or Important issues; its Minor performance note about recomputing docbar deltas was fixed by computing a single `NetlistDocbarDeltas` bundle per docbar frame and reusing it for both width selection and chip rendering.

Additional verification after QA fixes:

```powershell
cargo test -p rspice-ui shell::views::netlist --lib -- --nocapture
cargo test -p rspice-ui simulation::controller --lib -- --nocapture
cargo test -p rspice-ui --lib
cargo fmt --all -- --check
cargo check -p rspice-ui
cargo check -p rspice-ui --target wasm32-unknown-unknown
cargo clippy -p rspice-ui --all-targets --message-format short -- -D warnings
cargo build -p rspice-ui --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/rspice-ui.wasm --out-dir crates/rspice-ui/web/pkg --target web --no-typescript --out-name rspice-ui
```

Result: netlist view suite passed 36/36, simulation controller suite passed 13/13, full `rspice-ui --lib` passed 303/303, native/wasm checks passed, clippy passed with `-D warnings`, release wasm build passed, and the regenerated browser bundle passed the 390x844 Netlist docbar/diagnostic-strip smoke with no horizontal DOM overflow and no warning/error console entries.

Additional browser manual-QA closure (2026-06-17): reopened the rebuilt browser IDE and completed the previously missing checklist evidence. Loaded `Hierarchical RC Filter`, opened `user / hierarchical_rc_filter_tb / schematic`, and ran the generated Simulate run set from the Schematic workspace; the Simulate run history showed a successful transient run and the Results badge updated with zero browser warning/error console entries. Reopened the Netlist tab, pasted a manual `.op` + `.ac` + `.tran` deck with `.param rload`/`cdom`, and verified the source retained one `.ac` and one `.tran` line across manual runs. Ran the tuner in live mode; `rload` was rewritten from `7k` to `1.25k`, run logs refreshed, and browser warning/error count remained 0. Verified reset-to-last-run by editing the source to `rload=5k` without running, observing the `reset` chip, clicking it, and confirming the buffer returned to `rload=10k` and reran without warning/error entries. Verified diff-pip baseline semantics by editing to a structurally invalid resistor line, observing parser error `Expected node name, found Eof` with pips still visible after the failed run, then fixing the deck, rerunning successfully, and observing the gutter pips clear only after the success. Evidence PNGs were saved under `diagnostics/`: `netlist-generated-simulate-run.png`, `netlist-reset-to-last-run.png`, `netlist-diff-pips-failed-run.png`, `netlist-diff-pips-success-run.png`, and `netlist-live-tuner-run.png`; the files were rendered from disk and hashed with `Get-FileHash`.

Fresh verification after manual-QA closure (2026-06-17): `cargo fmt --all -- --check` passed; `cargo test -p rspice-ui shell::views::netlist --lib -- --nocapture` passed 36/36; `cargo test -p rspice-ui shell::views::symbol::tests --lib -- --nocapture` passed 8/8; `cargo test -p rspice-ui simulation::controller --lib -- --nocapture` passed 13/13; `cargo test -p rspice-ui --lib` passed 303/303; `cargo check -p rspice-ui --target wasm32-unknown-unknown` passed; and `cargo clippy -p rspice-ui --all-targets --message-format short -- -D warnings` passed.

- [ ] **Step 6: Final commit if verification changed files**

If formatting or QA fixes changed code, commit them:

```powershell
git add crates/rspice-core crates/rspice-ui docs/superpowers/plans/2026-06-17-netlist-editor-parity.md
git commit -m "fix(netlist): close parity verification issues"
```

Expected: no commit is created if the worktree is already clean.
