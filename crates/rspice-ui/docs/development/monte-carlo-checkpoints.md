# Monte Carlo checkpoint implementation

Developer reference for the continuation journal and its integration boundaries.

The configured-study runner now supports a lossless internal continuation journal
(`simulation/runner/study/monte_carlo.rs`). It binds the core population to the
canonical configured base, prerequisite, measurement and saved-OP contracts. A
caller can retain completed trials, publish snapshots at a chosen trial cadence,
resume only missing circuits, and select a new trial range against the same frozen
circuit. Histogram changes reuse the observations; statistics and confidence are
recomputed over the requested range. Source and effective configuration changes
still require a new population. The evaluator compatibility digest ignores the
plan revisions of the selected base, its configured OP prerequisite and its
postprocessor producer. Their full configurations and stable instance identities
remain bound, including the original source, snapshot, result and numerical
values of a saved OP state. The ordinary prepared-task digest still records all
authored revisions. This compatibility contract uses evaluator domain v2; the
earlier internal v1 journal identity is rejected rather than silently reclassified.

The Studio binary envelope retains every trial's measurement verdict alongside
its exact core values. A finite `.MEAS` value that misses GOAL/TOL remains a failed
measurement after resume. Sparse indices, failed trials and exact floating-point
bits survive the portable round trip. Pooling combines matching trial populations,
counts matching overlaps once and rejects conflicting values or verdicts.
Decoding and merging enforce byte, trial and value budgets. Completed rows remain
in the caller-owned journal when cancellation or checkpoint publication stops a
run; interrupted trials remain unfinished.

Checkpoint capture cadence, explicit trial range and resume-input content identity
now belong to the immutable prepared-request digest. Configured Monte Carlo
requests reach the checkpoint runner through the normal dispatch path. Resume
inputs use checked portable bytes; a request without a checkpoint destination is
rejected before trial execution.

Native execution keeps only the latest complete snapshot in a separate queue,
independent of terminal success. Browser request protocol 32 transfers resume
bytes in a Uint8Array, separately from numerical dependency buffers and JSON
metadata. Workers publish accepted snapshots through a dedicated message before
returning their terminal result. Ingress checks byte budgets and content identity;
old worker epochs and request IDs cannot publish into a new run. Browser hard
cancellation retains the latest snapshot already received by the application;
unpublished work is not a durable checkpoint.

The controller retains each delivered journal on the exact prepared analysis,
including cancellation and error outcomes, and drains the final snapshot before
advancing to the next task. Checkpoints obey the run's storage ceiling. A request
for checkpoint capture cannot report successful completion without a retained
journal matching its completed trial measurements.

Project result schema 35 stores the checked portable bytes as bounded Base64 with
their content identity. Result and dataset digests bind the journal, and storage
accounting includes its full binary size. Older projects preserve absence and
their existing digests. Saving during execution keeps the committed trials;
reopening restores an interrupted result, never a completed statistical population.
Project and session snapshots share this result representation.

Monte Carlo forms expose trial retention and checkpoint cadence (default:
every ten newly completed trials). A second section selects retained journals
from the same authored analysis. Preparation validates the selected journals and
pools matching populations into owned request bytes; subsequent history pruning
cannot mutate that request. After materializing the Run Set, preparation matches
each point against its exact population without executing a trial. Each worker
receives only its matching pool; unselected points run fresh. Missing selections
or a selected population matching no requested point reject preparation. Changing
solver, sampler or circuit settings cannot silently discard selected trials. Inactive editor buffers survive
JSON/RON round trips without influencing execution. A fully cached run publishes
its journal once even when no new circuit is evaluated.

Checkpoint inspection/import/export and browser runtime verification still need
integration. A focused three-point
Run Set check verifies that two resumed populations solve only their missing
trials, the third point runs fresh, and all observations match fresh runs. It also
checks refusal of incompatible solver settings and omitted selected points.

Core population domain v2 permits literal `.MC` run counts, START, CONFIDENCE,
CI, RESAMPLES and BOOTSEED to change while retaining exact trial identity. The
command parser records the reporting spans; source normalization requires the
same recorded owner and exact logical card, assembled with the parser's comment
and continuation rules. Sampler fields, parameter names, expressions, other
source bytes and post-parse AST changes remain bound. Expressions in confidence
fields are deliberately not normalized because they can consume random draws.
Includes without an exact root-card match remain conservatively source-bound.
The ordinary source used for trial replay is never rewritten. Internal core v1
journals are rejected rather than silently migrated.

Four focused core identity checks cover reporting edits, continuations, scope,
source/AST mutation, includes, Xyce comments and parameter names. A Studio AC
resume check verifies that changing to bootstrap confidence reuses completed
trials, solves only missing indices and matches a fresh run's observations and
confidence intervals.

The default all-node OP study uses the same continuation path as configured
studies. It elaborates the voltage roster without solving a nominal OP, retains
authored node names and non-conflicting numeric aliases, and excludes event-only
placeholder rows. The population binds the full node order and analog/event
classification; a trial that changes that basis stops the study. Actual trials
run the ordinary core OP solver on each materialized circuit.

Histogram bins are configurable for the default voltage study as well as for
selected bases. The option is frozen in the prepared request and transported to
the worker. Constant observations still produce the engine's single-bin
histogram. Histogram and confidence changes can reuse retained circuit trials.

Focused checks compare resumed voltage studies against both fresh studies and
the existing service runner for parameter and deck-statistics sampling, including
Run Set temperature/supply, mixed-signal filtering, and authored numeric names.
They also cover a nominal circuit with no real OP solution, changed-basis refusal,
form validation, and the synchronized Rust/JavaScript request protocol.
