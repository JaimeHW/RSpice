# Netlist Editor Parity Design

## Purpose

The netlist editor must become a first-class, text-first simulation workflow, not a decorative view of generated schematic output. The reference in `design/app/volta-netlist-editor.html` describes a professional SPICE editing loop: edit the deck, run the deck, tune `.param` values, see structured diagnostics at the exact source location, and compare tuned results against the last successful run.

This design completes that milestone for RSpice. The goal is commercial-quality behavior for the netlist editor surface: the editor must be credible in front of users who expect Spectre, ADS, Virtuoso, and similar tools to respect authored text as the source of truth. The milestone does not claim to implement every feature of those full EDA products. It makes the RSpice netlist editor match the VOLTA reference in appearance, workflow, and supported behavior where the current parser and runner already have the required foundations.

## Requirements

- Manual netlist source is authoritative for the netlist workspace after any text edit. Pressing Run in the netlist workspace executes the edited buffer, not the Simulate view's configured run set.
- Deck-authored analysis commands are runnable. At minimum, `.op`, `.dc`, `.ac`, `.tran`, `.noise`, `.pz`, `.sens`, `.tf`, `.stb`, `.disto`, `.four`, `.step`, `.temp`, and `.mc` must be parsed into the existing analysis execution model when the current parser supports them.
- Manual deck runs must not append duplicate Simulate-generated analysis lines. The command lines in the manual deck remain the visible document of record.
- Schematic and Simulate workflows continue to use the existing run-set path and generated netlist flow.
- A manual deck with no runnable analysis command reports a clear netlist-editor diagnostic and console message. It must not silently fall back to the Simulate run set.
- Run progress is visible in the netlist docbar while a netlist run is active: running phase, percentage when known, and a compact progress track matching the VOLTA run bar.
- Diff pips compare against the last successfully run buffer snapshot. They are not merely "lines touched since data_version" and they clear only when the run that used the current buffer completes successfully.
- The tuner panel is generated from `.param` lines and respects optional range annotations in nearby comments, using the design grammar `* @tune name min..max` and `* @tune min..max` for the next `.param`.
- Tuner slider edits keep the `.param` source line in sync, support live and on-release rerun modes, queue at most one rerun while the engine is busy, and never flood the runner.
- The tuner includes reset-to-last-run behavior, mini Bode, "As tuned" metrics, and delta chips. These must all derive from one shared run summary so the numbers never disagree.
- Diagnostics are structured: severity, byte span, line/column location, message, and optional fix. The gutter pip, squiggle/highlight, bottom strip, and fix action all read the same diagnostic vector.
- Parser errors use real spans when available. Additional editor lints detect unknown model/subckt references in the edited deck and suggest the nearest known symbol when confidence is high.
- Completion continues to use parser-derived symbols and parameter assignments. Structured diagnostics must not regress completion behavior.
- Visual treatment follows the existing RSpice token system and the VOLTA reference: instrument-panel density, mono source text, small focused controls, restrained borders, yellow accent for action/diff, severity colors for errors and warnings, and no marketing-style layout.
- All changes are covered by focused unit tests and verified with `cargo check -p rspice-ui`, relevant `cargo test` subsets, and visual/manual QA of the netlist editor workflow.

## Architecture

### Run Intent

Add an explicit simulation launch intent instead of overloading `SimulationState::trigger_simulation` as the only signal. The two initial intents are:

- `SimulateRunSet`: the existing Simulate/schematic path. It builds an `AnalysisPlan` from `state.sim_setup.enabled`, generates analysis command lines from UI configs, and uses the generated schematic netlist flow.
- `ManualDeck`: the netlist editor path. It parses `state.workspace.netlist_source` or `state.simulation.netlist_content`, derives the analysis queue from `rspice_core::Netlist::analyses`, and runs the source deck without appending Simulate view analysis lines.

This distinction keeps the user-facing model clean: Simulate Run executes the Simulate run set, while Netlist Run executes the text deck. It also prevents regressions in automation, toolbar shortcuts, and schematic cross-probing because those callers can keep using the existing run-set intent.

### Deck Analysis Bridge

Create a focused conversion layer in the simulation controller, owned near `analysis_plan.rs` and `analysis_commands.rs`, that converts `rspice_core::netlist::AnalysisCommand` into the existing `AnalysisSpec`, `AnalysisConfig`, and `QueuedAnalysis` structures.

The bridge should preserve command order from the deck. It should initially support every `AnalysisCommand` variant that can be represented by the current UI runner types:

- `.op` to `AnalysisSpec::DcOp` and `AnalysisConfig::DcOp`.
- `.dc` to `AnalysisSpec::DcSweep` and DC sweep config, including the optional second source when supported by the existing config path.
- `.ac` to `AnalysisSpec::Ac`, preserving `LIN`, `DEC`, and `OCT`.
- `.tran` to `AnalysisSpec::Transient`, preserving step, stop, start, max step, and `UIC`.
- `.noise` to `AnalysisSpec::Noise` where the existing runner can represent it. If the runner cannot preserve the input source or reference node yet, the bridge must report that limitation rather than run a misleading approximation.
- `.pz`, `.sens`, `.tf`, `.stb`, `.disto`, `.four`, `.step`, `.temp`, and `.mc` to the closest existing `AnalysisSpec` path when the data can be preserved.

Unsupported or lossy conversions become structured configuration errors attached to the source command line when source location is available. The bridge must never silently drop a deck command or execute a different analysis than the user authored.

### Manual Deck Composition

Manual deck execution should pass the edited source to the runner as the deck of record. The existing `compose_manual_netlist` behavior appends configured analysis lines and marks deck-authored dot analyses as inert; this must be retired for the netlist-editor run intent.

The composition rules are:

1. Normalize the final `.end` only if the runner requires it and the source lacks it.
2. Apply global simulation options in a way that does not duplicate or override explicit deck `.options` unexpectedly.
3. Preserve deck-authored `.save`, `.probe`, `.print`, `.plot`, `.meas`, `.ic`, `.nodeset`, `.include`, `.lib`, and analysis commands exactly as authored.
4. Record the exact source snapshot used by the run before execution starts.

The Simulate run-set path may continue to inject generated analysis lines into generated schematic netlists.

### Run Progress and Baselines

Extend netlist editor state with a last-run baseline:

- `last_run_buffer`: the exact source text used by the last successful manual deck run.
- `last_run_params`: parsed `.param` values from that snapshot.
- `pending_run_buffer`: source text captured when a manual deck run starts.
- `pending_run_id`: the active run id or local token associated with that captured source.

Diff pips compare the current buffer line-by-line against `last_run_buffer`. Parameter value readouts compare against `last_run_params`. When a manual deck run completes successfully and its token matches the pending buffer, promote `pending_run_buffer` to `last_run_buffer`, refresh `last_run_params`, and clear diff pips. Failed or aborted runs leave the last successful baseline intact.

The docbar progress track reads the existing runner status and batch indices. When exact progress is unknown, it shows phase and an indeterminate-safe track without lying about completion.

### Tuner Panel

The tuner remains in `shell/views/netlist/tuner.rs`, but the parsing and baseline model should be split into small testable units:

- `ParamRow`: source line, byte value span, parsed value, raw value, unit suffix, optional tune range, and baseline value.
- `TuneRange`: lower bound, upper bound, scale, and whether the slider should be logarithmic.
- `TunerSnapshot`: current params, last-run params, and changed rows.

Range annotations use comments so existing SPICE parsers ignore them:

- `* @tune itail 5u..60u`
- `* @tune cl 0.5p..8p`
- `* @tune 16u..160u` immediately before `.param w_in=64u`

If a numeric parameter has no annotation, the current decade heuristic remains a fallback. Expression-bound parameters stay read-only until the expression can be safely edited.

Reset to last run rewrites every tuned `.param` whose baseline value is known back to the last-run value. It marks changed lines, refreshes diagnostics, and triggers a run according to the active tuner mode.

### Shared Run Summary

Add a shared summary builder for AC and measurement-derived metrics so mini Bode, "As tuned", and delta chips read one object. This should reuse the existing Bode calculations in `shell/results/bode.rs` and `ui::plot` rather than duplicating crossing math in the tuner.

The summary should include:

- AC signal name and sampled frequency/gain/phase data when an AC result exists.
- Stability metrics already computed for the full Bode view: DC gain, unity-gain frequency, phase margin, gain margin, f180, and f3db.
- Measurement values by name from the active run.
- Verdicts against `workspace.specs`, where good/bad color follows spec violation distance rather than numeric sign.
- Baseline values from the previous successful manual deck run or previous simulation run, depending on the visible comparison mode.

The right panel should show the mini Bode only when AC data exists. It should show an unobtrusive empty state when no compatible result is available.

### Structured Diagnostics

Replace the current netlist editor diagnostic shape with:

- `severity`: error, warning, or info.
- `span`: byte start and end in the current buffer when known.
- `line` and `column`: cached display location derived from the span.
- `message`: concise user-facing message.
- `source`: parser, lint, runner configuration, or runtime.
- `fix`: optional fix action with label and replacement span/text.

The editor must continue to parse on a debounce, but every visual diagnostic affordance reads this one vector:

- Gutter severity pip on the diagnostic line.
- Span-level underline or highlight in the source.
- Bottom strip row with severity, location, message, and fix label.
- Click-to-focus behavior for the diagnostic location.
- Fix application for safe replacements such as unknown-model typo corrections.

The first lint layer should use parser/source-map data from the current buffer to collect model definitions, subcircuit definitions, and references from model-bearing elements. Unknown references should suggest the nearest known model or subckt name only when the edit distance/confidence threshold is strong enough. It must not warn on built-in models or references that will be resolved from `.include`/`.lib` without available file IO in the editor pass.

### Parser Source Map

The parser already tracks token spans in the lexer, but public parse errors and model references do not consistently expose them. Add a source-map path that does not disturb normal netlist execution:

- Carry source spans for parser errors where available.
- Record `.model`, `.subckt`, `.param`, and analysis command spans.
- Record model/subckt reference spans for diodes, BJTs, MOSFETs, JFETs, MESFETs, model-based R/C/L, switches, transmission lines, XSPICE instances, and subcircuit instances.
- Preserve correct physical-line offsets through continuation lines.

The source map should be optional metadata on parse output or a parallel editor parse helper. It must not make the simulation engine slower in normal batch execution.

### Visual Quality

The editor should stay dense and tool-like:

- Keep source text in the mono editor with gutter, line numbers, pips, and completion popover.
- Add progress as a compact docbar element, not a large panel.
- Keep the tuner panel at the right side with compact rows, sliders, bounds, reset, mini Bode, and metric table.
- Use existing `Tokens`, `Button`, `chip`, `section_header`, and plot helpers where possible.
- Avoid nested cards and oversized decorative surfaces.
- Ensure all labels fit in the right panel at normal desktop sizes and degrade gracefully if the panel narrows.

## Data Flow

1. The user edits the netlist buffer. The workspace enters manual source mode and stores the buffer in `workspace.netlist_source`.
2. The editor debounce parses the buffer into `Netlist`, parser symbols, source-map data, and structured diagnostics.
3. The tuner scans `.param` rows and `@tune` comments from the same buffer. Slider changes rewrite source spans in the buffer.
4. Netlist Run captures the current buffer as a pending manual deck snapshot and sends a `ManualDeck` run intent to the simulation controller.
5. The controller parses the pending deck, converts deck analyses into queued analyses, applies safe simulation options, and starts the runner.
6. Runner status updates drive the docbar phase/progress display.
7. Completed analysis results enter the existing run history.
8. The shared run summary reads the active run and last baseline, feeding delta chips, mini Bode, and "As tuned" rows.
9. On successful completion of the captured manual deck run, the pending buffer becomes the last-run baseline and diff pips clear.

## Error Handling

- Parse errors block manual deck execution and surface as structured diagnostics.
- A manual deck with no runnable analyses reports "No analysis command in netlist" with a fix hint to add `.op`, `.ac`, `.tran`, or another supported command.
- Unsupported deck commands report exactly which command cannot yet be run and why.
- Lossy command conversions are refused. Running a different analysis than the authored deck is worse than refusing the run.
- Live tuner reruns are coalesced. If the runner is busy, the editor records one pending rerun and executes only the newest buffer after the current run finishes.
- Aborted or failed runs do not update last-run baselines or clear diff pips.
- Unknown-model diagnostics avoid false positives for unresolved include/lib content. When in doubt, warn less in the editor and let runtime resolution report the authoritative error.
- Completion remains usable while diagnostics are stale during debounce. The last clean symbol table stays available until a new clean parse replaces it.

## Testing Strategy

Use TDD for implementation.

Core tests:

- Manual netlist Run with `.ac` and `.tran` works when `state.sim_setup.enabled` is empty.
- Netlist Run derives the queue from deck command order.
- Manual deck execution does not append duplicate analysis lines.
- Simulate Run still uses the Simulate run set and generated netlist path.
- Empty manual deck run set reports the no-analysis diagnostic.
- Each supported `AnalysisCommand` conversion preserves its numeric fields and sweep mode.
- Unsupported or lossy conversions fail with a source-local diagnostic.
- Diff pips compare against the last successful run snapshot and do not clear on failed runs.
- `@tune` annotations produce exact slider ranges, with the decade heuristic only as fallback.
- Reset to last run rewrites tuned params and leaves expression params untouched.
- Live tuner reruns coalesce to one queued run while the engine is busy.
- Shared Bode summary returns the same UGF, phase margin, and gain margin as the full Bode view.
- Delta verdicts use spec violation distance rather than numeric sign.
- Parser diagnostics expose spans, and the editor underlines only the span rather than the whole line.
- Unknown model/subckt lint suggests a nearest correction when confidence is high and stays quiet for included-library uncertainty.
- Completion symbols still update after clean parses.

Manual QA:

- Open the netlist workspace, edit a deck with `.ac` and `.tran`, press Run, and confirm results appear without touching Simulate view checkboxes.
- Drag a tuned `.param` in live mode and confirm source text, diff pips, progress, results, and delta chips update coherently.
- Switch to on-release mode and confirm no runs start until the drag commits.
- Use reset to last run and confirm values, diff pips, and rerun behavior.
- Introduce a syntax error and confirm the gutter pip, span highlight, strip row, and run refusal all point to the same issue.
- Introduce a model typo and confirm the suggested fix changes only the misspelled token.
- Compare the mini Bode values against the full Bode results view after an AC run.
- Confirm generated schematic regeneration still works and discards manual edits only through the explicit Regenerate affordance.

Verification commands:

- `cargo fmt --check`
- `cargo check -p rspice-core`
- `cargo check -p rspice-ui`
- `cargo test -p rspice-core netlist`
- `cargo test -p rspice-ui shell::views::netlist --lib`
- `cargo test -p rspice-ui simulation::controller --lib`
- `cargo test -p rspice-ui --lib`

## Scope Boundaries

This work completes the netlist editor, live tuner, and manual deck run loop described by `design/app/volta-netlist-editor.html`. It does not implement a complete Spectre/ADS/Virtuoso clone, a full PDK model-management system, all possible vendor-specific SPICE dialects, or a new waveform database. It should, however, make the supported netlist editor workflow polished, honest, text-first, and robust enough to demonstrate confidently to serious EDA users.
