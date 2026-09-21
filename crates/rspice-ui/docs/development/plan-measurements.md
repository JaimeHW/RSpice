# Authored plan measurements

The Specs editor can explicitly define a measurement using a complete `.MEAS`
card. Existing expressions remain descriptions until **Define measurement when
running the plan** is enabled. Measurement names must match the card, and names
must be unique across the design and authored plan definitions.

Generated-primary preview and export retain these cards. Prepared schematic runs
validate the composed deck after include expansion, using the actual design
parameter context, then seal those same cards into the execution source. The
browser worker receives the source through its existing netlist contract. Manual
decks continue to own their measurement cards. Design inspection is independent
of measurement definitions.

Executable measurement families are TRAN, AC, DC and NOISE. A specification bound
to a configured Monte Carlo or optimization analysis uses its selected base
analysis family. The selected producer must be enabled. Other result families
can still be referenced by specifications using their existing scalar evidence.

Definitions, the explicit authoring flag and producer identity survive project
save/load. Enabling authoring changes generation and prepared-run identities;
legacy/default definitions keep the previous serialized and canonical forms.

Evaluation uses the native measurement parser and evaluator, including its
accepted-sample windows and dialect semantics. For example, an INTEG window
integrates between selected accepted samples; it does not manufacture samples
at arbitrary FROM/TO boundaries. Measurement goal failure retains the computed
value and its failed status. The specification unit remains a display label;
values and limits are expressed in the measurement's native units.

Focused verification: `cargo test -p rspice-ui --lib authored_plan_measurements`.
The tests cover editing, generation identity, sealed source, project persistence,
real transient/AC solves, measurement goals, expressions using design parameters,
study producer resolution, duplicate names and invalid definitions.
