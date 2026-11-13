use super::*;

#[cfg(feature = "veriloga")]
// Bump whenever a persisted runtime artifact or its integrity contract changes.
// Version 16 adds model/instance/dual parameter-storage semantics to canonical IR.
// Version 17 adds exact legacy Laplace Jacobian semantics.
// Version 18 retains exact transfer-function DC actions in compiled Laplace filters.
// Version 19 adds signed slew IR, runtime, and checkpoint semantics.
// Version 20 adds transactional event-state variable metadata.
// Version 21 retracts front-end acceptance: reserved Verilog-AMS digital
// keywords, discrete-discipline `posedge`/`negedge`, and `analog final` are now
// refused. The record carries no compiler identity of its own, so this constant
// is the only thing standing between a stale artifact and a source the current
// compiler will not compile.
// Version 22 extends front-end acceptance the other way: the IEEE 1364 digital
// subset (`always`, `initial`, `reg`, `wire`, `assign`, four-state literals,
// vector ranges) now parses and is semantically analyzed, so the same source
// text produces a different parse than version 21 did.
// Version 23 gives the digital subset a canonical form: `always` and
// `initial` processes lower to CFG functions with a `Wait` terminator, and the
// artifact carries them in a new `digital` field. The same source that
// version 22 refused at the artifact boundary now produces an artifact, so the
// record's identity has to change even though a continuous-domain model's
// artifact is byte-identical to the one version 22 wrote.
// Version 24 widens that canonical form. Process-local variables, `for`,
// `while`, `repeat`, and the wildcard `case` forms lower where version 23
// refused them, a continuous assignment lowers to a driver process and a new
// `drivers` list, and two IEEE 1364-2005 defects are corrected: a
// concatenation target now resizes its right-hand side (section 5.2.1) and a
// case item is matched by identity rather than by `==` (section 9.5). Source
// that version 23 refused now compiles, and source it compiled can produce a
// different artifact, so the record's identity has to change.
// Version 25 elaborates a digital module hierarchy. A module that instantiates
// another module with discrete-domain content was refused by version 24; it now
// flattens into one plan whose signals carry instance-qualified names and whose
// processes and drivers carry per-instance identities. Source that version 24
// refused compiles, and a hierarchy's artifact is a different artifact, so the
// record's identity has to change.
// Version 26 adds generalized transient integration and exact accepted history
// to compiled Laplace filters. Cached version-25 realizations do not carry the
// older state and derivative lanes required by trapezoidal and Gear-2 stepping.
// Version 27 widens the digital front end at the declaration boundary. A port
// with no net declaration of its own is now the implicit net of IEEE 1364-2005
// section 12.3.3 rather than an undeclared name; `output reg [3:0] q;` is now
// section 12.3.4's compact spelling of the two-declaration form; a packed range
// and a replication count may name a parameter, which section 12.2 fixes at
// elaboration; and the eight gate primitives of section 7.2 lower to the same
// drivers their operator spellings do. Source that version 26 refused now
// compiles, and a structural design's artifact is a different artifact, so the
// record's identity has to change.
// Version 28 closes the last of the digital-Verilog language gaps the oracle
// corpus named. The bitwise XNOR of IEEE 1364-2005 section 4.1.9 in both its
// spellings, the case equality of section 4.1.8, and the reduction operators of
// section 4.1.10 all lex, parse and lower; a generate region of section 12.4 is
// unrolled at elaboration time; a port may be connected to a bit- or
// part-select of a net (section 12.3.9); a child module's own parameters are
// fixed at their declared defaults (section 12.2) instead of being refused; and
// a sized literal keeps the width its author wrote (section 3.5.1) rather than
// widening to thirty-two. Source that version 27 refused now compiles, and
// source it compiled can produce a different artifact — a concatenation holding
// a sized literal is a different width and therefore a different value — so the
// record's identity has to change.
// Version 29 sizes a digital expression by its context, IEEE 1364-2005 section
// 5.4.1. The assignment's left-hand side is part of the expression's context,
// so the operands of a context-determined operator are extended to the width of
// the largest expression including the target and the operation runs at that
// width; an unsized literal takes a context wider than its 32-bit floor. A
// version-28 artifact computed `a * b` at the operand width and widened the
// product afterwards, which is a different number rather than a narrower one —
// `{cout, sum} = a + b` had a `cout` that could never be 1 — so every cached
// digital artifact has to be rebuilt and the record's identity has to change.
// Version 30 signs a digital expression, IEEE 1364-2005 section 5.4.2. A net,
// variable or port declared `signed`, an `integer`, a plain decimal number and
// a literal carrying the `s` base marker are signed; a bit-select, a
// part-select and a concatenation are not, and one unsigned operand makes the
// whole expression unsigned. A signed operand is sign-extended to its context
// instead of zero-extended, a comparison between two signed operands is made
// on signed numbers, division truncates toward zero with the modulus following
// its first operand, and `>>>` shifts in the sign bit. A version-29 artifact
// carried every one of those declarations and read none of them, so it
// compiled a `reg signed` into an unsigned device: `p = a` widened -1 into 15
// and `a < 0` was false for every value. `4'd9` also decodes to its declared
// four bits now rather than to the 32-bit unsized floor. Both change the value
// an artifact computes, so every cached digital artifact has to be rebuilt and
// the record's identity has to change.
// Version 31 groups each syntactic Verilog-A noise primitive as one coherent
// process and serializes its derivative-shadow replay plus every signed circuit
// injection. A version-30 artifact has neither the schema marker nor those
// routing programs, so accepting it would silently revert repeated uses of one
// process to independent sources and lose cancellation/correlation.
// Version 32 gives the discrete domain a second value type: Verilog-AMS
// LRM 2.4 section 3.7's real net. `wreal`, and the four resolved spellings
// beside it, declare a net carrying a real rather than four-state bits; a
// plan signal now records which, a real net starts at 0.0 rather than at
// `z`, and real arithmetic, real comparison and a real conditional are
// three new value kinds a process function can hold. A version-31 artifact
// could not contain any of them — `wreal` was refused at the keyword — so
// nothing cached is being reinterpreted here. What changes is the *shape*
// of the record: a plan serialized by version 32 carries a field version
// 31's reader does not know, and a version-31 plan decodes under version 32
// only because that field defaults. Rebuilding rather than leaning on the
// default is the fail-closed reading, so the record's identity changes.
// Version 33 gives the discrete domain a real *variable*, which is where a
// real-number model keeps state. `real` is now a discrete-domain signal class
// beside `wreal` — written procedurally rather than driven, per IEEE 1364-2005
// section 6.2 — reached either by `output real` or by a module-level `real` a
// process writes in a module with no analog block. `$realtobits` and
// `$bitstoreal` are two more value kinds, a `parameter real` folds into a real
// expression, and a deferred nonblocking update carries either a four-state
// value or a real. A version-32 artifact could contain none of that: every one
// of those constructs was refused by name, so nothing cached is being
// reinterpreted. What changes is the shape of the record again — a plan
// serialized by version 33 can carry a real variable and a deferred real update
// that version 32's reader has no case for — so the record's identity changes
// rather than relying on a decode that would silently drop them.
// Version 34 reserves the six Verilog-AMS connect keywords — `connect`,
// `connectrules`, `endconnectrules`, `resolveto`, `merged` and `split` — and
// reads the two constructs they belong to: `connectmodule`, which LRM 2.4
// Syntax 7-4 makes a third `module_keyword` and which the parser refused
// outright before, and the `connectrules` block of Syntax 7-5. It also reads
// `` `default_discipline `` (section 10.2), which every module now carries.
//
// Unlike versions 32 and 33, this is not a new shape inside the plan: no
// executable form of a connect module exists yet, so nothing new reaches a
// cached artifact. What changes is what the *front end accepts* — a source
// that was a hard parse error under version 33 compiles under 34, and six
// identifiers that were legal names are not any more. A cached artifact
// therefore no longer stands for the same compile, and the fail-closed
// reading is to rebuild rather than to reason about which sources are
// unaffected.
//
// Version 35 is version 34's situation again, for one line of the front end.
// A compiler directive the preprocessor does not know now takes its operand
// with it, per the preprocessor's own rule that a line opening with a backtick
// is a directive line. Under version 34 the operand stayed in the token
// stream, so `` `default_nettype wire `` was a hard parse error — the `wire`
// reached the top-level item loop alone — and under 35 the whole line is
// dropped and the file compiles. That is the same kind of change as 34's:
// nothing in a cached plan is reinterpreted, but a cached artifact no longer
// stands for the same compile, and the fail-closed reading is to rebuild.
//
// Version 36 is that situation once more, and this time the change is on *this*
// side of the boundary rather than the compiler's: a `.VERILOGA` include is now
// compiled with `enable_ams` on (see [`deck_include_compiler_options`]), so a
// `.va` whose module carries digital content compiles instead of being refused
// at code generation, and reaches the mixed host that executes it.
//
// Nothing cached is being reinterpreted, and for a stronger reason than 34's
// and 35's. A mixed module under version 35 did not compile at all, so no
// version-35 record for one exists to be misread; and for an analog-only module
// the option changes nothing an artifact can carry, because its entire effect
// in the compiler is to skip a check that an analog-only module passes. What
// changes is once again what the front end accepts, so a cached artifact no
// longer stands for the same compile, and the fail-closed reading is to rebuild
// rather than to reason about which sources are unaffected.
//
// Version 37 changes the shape of a cached plan and what the front end
// accepts, both at once, for the same construct: Verilog-AMS LRM 2.4 section
// 7.3.3's probe of a continuous net from a discrete context. A plan now carries
// an analog-probe table and process functions carry a value kind that reads
// one, neither of which a version-36 reader has a case for; and a process that
// wrote `V(p, n)` was refused by name under 36, so a source that did not
// compile now does. It also moves the ownership rule for a module-level `real`
// off the *module* and onto the *name*, per section 7.3's "Write operations of
// nets and variables are only allowed from the context of their domain", which
// makes a third class of source compile that did not: one whose analog body
// neither reads nor writes the variable a process owns.
//
// Nothing cached is reinterpreted — every construct involved was refused under
// 36, so no version-36 record contains one — but a cached artifact no longer
// stands for the same compile, and the fail-closed reading is to rebuild.
//
// Version 38 follows the canonical IR's own schema from 12 to 13. The HIR now
// carries `HirExecutedCorrespondence`: which executed expression each
// structured-body expression is a second lowering of, which is what lets a
// CFG-sourced backend name the state record an operator owns. A version-37
// record predates the map and would deserialize with an empty one, and an empty
// correspondence is not "no operators" — it is "every operator unmapped", which
// a CFG-sourced consumer must refuse. Rebuilding is both the fail-closed and the
// only useful reading. `CANONICAL_IR_SCHEMA_VERSION` would refuse such a record
// on its own; this constant moves with it so the refusal happens at the cache
// boundary, where the diagnostic names the cache file.
//
// Version 39 re-indexes the compiled model itself. The bytecode generator
// numbered a module's analog-operator records once per *emission*; the compiler
// now rewrites every slot index into the canonical per-*site* numbering before
// the model leaves `compile_file_runtime_with_metadata`. A version-38 record
// holds a `CompiledModel` whose `DdtState`, `CrossState`, `LaplaceState` and
// sibling instructions carry the emission numbers, and nothing in the record
// says which numbering it is in. Deserializing one beside a runtime that
// allocates per site would read a record at one index and write it at another,
// so the record has to be rebuilt rather than reinterpreted.
//
// Version 40 changes what a compiled model's Zi placeholder means. A cached
// version-39 model records every Zi site's unfrozen placeholder as a one-by-one
// filter regardless of the coefficient lists the source declares; the site's
// real widths only appeared once an evaluation froze the per-instance values.
// A transient resume validates a checkpoint against a rebuilt device before it
// has evaluated anything, so a version-39 placeholder reports the wrong shape
// at exactly the point the comparison is made. The widths are syntactic, so
// they now travel with the compiled model. Rebuilding is the only reading: the
// old record's placeholder shape is not recoverable from the record.
// Version 41 changes which value an equation reads. Through version 40 the
// bytecode's stamp entries read a variable's slot after the whole assignment
// pass had run, so an equation over a scratch variable a later statement
// reassigns read the last write rather than the definition reaching the point
// the contribution was written at. The compiler now redirects such a read to a
// snapshot slot filled where the definition reaches. A version-40 record holds
// a `CompiledModel` with neither the snapshot variables nor the redirected
// stamp programs, and nothing in the record distinguishes one written before
// the repair from one written after — reading it would silently stamp the
// defective value on the VM and JIT routes. Rebuilding is the only reading.
// Version 42 carries the same repair onto the JIT and WASM routes. A version-41
// record holds the snapshot slots and the redirected stamp programs but not the
// plan saying where each copy belongs in the statement sequence, and the
// executable routes replay statements rather than steps: they would run a
// version-41 model with the copies never made and the equations still reading
// the slot the assignment pass finishes with. The plan is not recoverable from
// the record — it names the statement each definition was written at, and the
// record keeps no statement sequence. Rebuilding is the only reading.
//
// Version 43 changes how a non-finite float is written. This record is JSON,
// and JSON has no spelling for an infinity: through version 42 `serde_json`
// wrote one as `null`, so a `from (0:inf)` parameter — which the compact models
// are full of — was cached with `CompiledParameter.max` as `null` and read back
// as `None`, the declared bound simply gone. The compiler now encodes a
// non-finite float as a string (`rspice_veriloga::json_float`), but a
// version-42 record's `null` is indistinguishable from a parameter that never
// declared a bound, so the loss is not recoverable from the record. Rebuilding
// is the only reading.
//
// The compiler leaving a mirrored `CompiledModel::noise_assignment_steps` empty
// does *not* move the version, and the rule is why. Every bump above names a
// record the new runtime would read *wrongly*; this one changes what a new
// record contains, not how an old one reads. A version-43 record written before
// the convention carries the full twin, which was byte-identical to
// `assignment_steps` after state renumbering, and the noise replay executes
// `noise_assignment_steps` when it is non-empty — so the old record replays
// exactly the steps it always did, and the new one replays the same steps
// through the ordinary list. Nothing in the record becomes ambiguous, so
// rebuilding it would buy nothing.
// Version 44 invalidates models compiled while analog $fatal, $stop, and
// initialization $error could be discarded. Their calls cannot be recovered
// from a cached CompiledModel. This also covers portable entries without a
// canonical artifact, which cannot rely on the canonical schema check.
// Version 45 gives global-event filters distinct physical-analysis query IDs;
// old bytecode uses phase-sensitive predicates and must be rebuilt from source.
// Version 46 requires the model-defined nodeset capability bit.
// Version 47 rebuilds reactive bytecode for k(x)*ddt(q(x)): old cached entries
// differentiate k*q and retain the incorrect q*dk/dx term.
// Version 48 rebuilds derivative shadows for finite nested ddx; older
// portable records can silently truncate higher derivatives.
// Version 49 rebuilds real-modulo Jacobians and noise gains: older portable
// bytecode treats the result as constant and silently drops both slopes.
// Version 50 inserts real-to-integer assignment conversions before AD and
// preserves analog function argument types at separate declarations.
// Version 51 converts integer parameter defaults before dependent expressions
// and leaves placeholder range validation to final instance construction.
// Version 52 preserves signed integer arithmetic before real conversion.
// Version 53 retains checked primal dependencies when resolving ddx.
// Version 54 preserves signed zero in primal factors during differentiation.
// Version 56 preserves hypot and stabilizes homogeneous/division derivatives.
// Version 57 retains authored Laplace coefficients for range-safe AC responses;
// older realizations cannot recover coefficients lost during normalization.
// Version 58 normalizes language-level Laplace root factors as 1 - s/root.
// Version 59 uses a consistently rounded thermal voltage scale for explicit
// temperature arguments; older bytecode embeds a truncated k/q constant.
// Version 60 evaluates simulator queries and selected fallbacks at runtime.
// Version 61 retains range-protected sum-products quotient bytecode and derivatives.
// Version 62 represents idt without an explicit IC by an internal solver
// equation; older records irrecoverably substitute an explicit zero IC.
// Version 63 binds an explicit module selection into memory and disk identities.
// Version 64 resets simulator-control tasks once before all runtime loops.
// Version 65 distinguishes transient discontinuities from Newton convergence hints.
// Version 66 retains limiter affine residual corrections for Newton stamping.
// Version 67 solves flow probes as simultaneous branch equations.
// Version 68 also retains mixed parameter elaboration source and discrete analog inputs.
// Version 69 retains the active connection source closure in the canonical artifact.
// Version 70 fingerprints the source bytes actually consumed by preprocessing.
// Version 71 retains instance-local branches, nested port flows, and coincident-probe derivatives.
// Version 72 records potential branch identity in each compiled solver source.
// Version 73 resolves declared grounds and rejects conflicting net disciplines.
// Version 74 retains declared module timing and delays in resolved design ticks.
// Version 75 resolves module time queries and scales retained analog $realtime.
// Version 76 links canonical equations to shared physical branch unknowns.
// Version 80 carries event-controlled nonblocking subscriptions.
// Version 79 carries independent delayed nonblocking updates.
// Version 78 evaluates typed runtime procedural delays.
// Version 77 carries digital clock queries and exact wide digital expressions.
// Version 81 retains switch-branch source order and accepted-mode discontinuities.
// Version 83 carries computed event expressions and runtime bit indices.
// Version 82 rejects direct/indirect source conflicts across parallel branches.
// Version 84 carries normalized repeat counts and repeated event subscriptions.
// Version 85 validates indirect constraint controls, left sides, and uniqueness.
// Version 86 exposes indirect source activation to runtime topology guards.
// Version 87 validates and retains nature inheritance and physical declaration closures.
// Version 88 retains indirect-equation absolute tolerances.
// Version 89 preserves real-valued last_crossing results and runtime direction operands.
// Version 90 binds analog block-local declarations that shadow a module integer or
// real signal to the local, and retains per-site ddt tangent history. It also
// drops the persisted `Instruction::DdtJacobian` and `CfgValueKind::DdtScale`,
// so a record that carries either no longer deserializes at all and is
// discarded as unreadable rather than at the version check.
// Version 91 carries a named `$limit`'s limiting function in the bytecode
// entry, as an executable previous-iterate read and candidate publish, so the
// limiter is applied by every lowering instead of only the canonical one. It
// drops the persisted `Instruction::CanonicalLimitState`, so a record that
// carries one no longer deserializes at all and is discarded as unreadable
// rather than at the version check.
// Version 92 lowers a digital `!` to the one negation section 4.1.8 describes
// instead of negating a `!!x` truth value, so a cached process function carries
// two fewer `DigitalLogicalNot` values per logical negation (2026-09-12).
// Version 93 retains ordinary procedural entry values separately from current
// assignments and records their zero-seeded derivative lifetimes (HIR 59).
// Version 94 uses canonical checked array selections and canonical readback (HIR 60).
// Version 95 recompiles digital parameters with exact widths, signedness and
// inferred real/integral values, including localparams (HIR 61).
// Version 96 records event-owned derivative slots, which start each numerical
// evaluation at zero while observations retain the current candidate's slope.
// Version 97 preserves private inlined-function storage rather than treating
// task-only call inputs as externally observable variables (HIR 62).
// Version 98 records guarded parameter-bounded derivative loops (HIR 63).
// Version 99 qualifies higher-order canonical postfix derivatives (Wasm emitter 55).
// Version 100 supports higher absdelay input derivatives with fixed timing.
// Version 101 carries mixed transport-delay actions (Wasm ABI 18).
// Version 102 supports higher table input derivatives (Wasm emitter 58).
// Version 103 retains distinct tiny table intervals (Wasm emitter 59).
// Version 104 retains table derivative payload actions (Wasm ABI 19).
// Version 105 retains compiler diagnostics. Older records cannot distinguish
// a warning-free source from one whose warnings were discarded.
// Version 106 separates connection-configuration and source-specialization variants.
// Version 107 preserves physical vector lanes and topology-bound selectors (HIR 125).
// Version 113 preserves local packed net views through positional aliases (HIR 131).
// Version 114 preserves computed expressions in mixed packed inputs (HIR 132).
// Version 121 preserves conservative array topology and ranged grounds (HIR 139).
// Version 122 resolves wire/tri connections to real net types per occurrence (HIR 140).
// Version 123 retains resolved net types through structural concatenations (HIR 141).
// Version 124 validates connected disciplines and preserves local branch natures (HIR 142).
// Version 125 resolves inherited digital-net disciplines before conversion selection (HIR 143).
// Version 126 resolves structural wire domains before real-net promotion (HIR 144).
// Version 127 preserves implicit-net policy and generated net scopes (HIR 145).
// Version 128 normalizes explicitly continuous wire storage (HIR 146).
// Version 129 retains generated local declarations and analog bodies (HIR 147).
// Version 130 preserves generated analog order and scope identities (HIR 148).
//
// Version 140 follows the canonical IR's schema from 158 to 159, which changes
// what a cached digital plan means. A plan now carries a
// site table, one row per system task or function call with its arguments,
// outputs, `%m` scope, `$finish` level and source line; a scope table, which
// is always present because the root module is a scope; and observer functions
// for `$strobe` and `$monitor`. Process functions gain four value kinds —
// system task, system function, call output and pure function — and linking
// relocates every site, scope and observer and renames each instance's scope
// root to the instance name. A version-139 record predates all of this: it was
// refused at every system call, so no record holds one, but its plan
// deserializes with empty tables and its content identity is computed over a
// tuple that no longer includes them. Reading it beside a linker that numbers
// sites and scopes per instance, and a host that authenticates a checkpoint
// against the plan's identity, would answer to an identity this build never
// produces for that source. The identity is not recoverable from the record,
// so rebuilding is the only reading; this constant moves with the schema so
// the refusal happens at the cache boundary, where the diagnostic names the
// cache file.
//
// Version 141 follows the canonical IR's schema from 159 to 160, which changes
// what a cached analog half means. An analog `@(posedge d)` or `@(negedge d)`
// on a discrete signal now reads a digital edge flag the mixed host raises
// (`HirModel::discrete_edges`); a version-140 record of a module whose operand
// was a discrete `reg` or `wire` with no discrete discipline holds a `cross`
// detector on that signal's value instead, which compiles and never fires as
// an edge. A `slew` operand now reads a discrete value's left limit
// (`HirModel::discrete_left_limits`); a version-140 record reads the value
// itself and ramps from the timepoint before the edge. A packed grouping or
// part-select of 32 bits or more now reads as a real, and an analog-owned
// integer output port carries its driver (`HirModel::analog_drivers`); both
// were refused before, so no record holds one. The stale readings are not
// recoverable from the record, so it is rebuilt.
//
// Version 142 follows the canonical IR's schema to 161, which changes what a
// cached CFG means for an event control with nothing in its body. The
// optimizer now keeps every `cross`, `above` and `timer` detector as a root,
// because the detector is evaluated for what it asks of the stepper — a point
// at the crossing, the timer's next breakpoint — whether or not anything reads
// its value. A version-141 record compiled `@(cross(e));` with the detector
// pruned alongside the empty branch it guards, so on the compiled route the
// run put no point at the crossing that the interpreter, and this build, put
// there. The record deserializes and runs; it is wrong, not unreadable, and no
// field of it says whether a pruned detector is inside. Rebuilding is the only
// reading, so this constant moves with the schema and the record is refused at
// the cache boundary.
//
// Version 143 follows the canonical IR's schema to 162, which changes what a
// cached analog half's discrete inputs mean. Every discrete value an analog
// block reads is now sampled at its left limit during a transient trial, and
// only the reads inside analog event bodies see the settled value, through
// twins the record lists (`HirModel::discrete_event_reads`). A version-142
// record lists twins for `slew` operands instead and has none for event
// bodies, so its event bodies would read the value before the edge that fired
// them. Rebuilding is the only reading.
//
// Version 144 follows the canonical IR's schema to 163, which changes what a
// cached model's condition snapshots compute. A `__guardN` snapshot of an `if`,
// `case` or event condition nested in another block is now evaluated only under
// that block's guards and holds zero otherwise; a version-143 record evaluates
// it on every path, reading whatever the untaken block did not write. Its
// consumers are gated either way, so the scalar answer is the same, but the
// small-signal replay evaluates the stale snapshot in complex arithmetic, where
// diode_cmc's `abs(-fbbt / Fmaxr)` over a never-written `Fmaxr` is NaN and the
// device refuses AC. The record runs; it is wrong, not unreadable, so it is
// rebuilt.
//
// Version 145 follows the canonical IR's schema to 164, which adds digital
// tasks and functions to the plan. A version-144 record refused every module
// that declared one, so none holds a routine; rebuilding keeps the record
// and the schema in step.
//
// Version 147 is version 34's situation for `$fread`: the front end now
// accepts a reg as its first argument (IEEE 1364-2005 section 17.2.4.4),
// which was a compile error, and lowers it to the plan's existing output
// argument. No artifact changes shape, so the schema stays, but a cached
// artifact no longer stands for the same compile, and the fail-closed
// reading is to rebuild.
//
// Version 149 keys a record by the compilation unit it was compiled in. The
// HDL sources of a run are one unit (IEEE 1364-2005 clause 19), so a module
// compiled beside other files saw their macros, `timescale` and modules; the
// record carries the identity of that ordered set of files, and a record
// without the field cannot say which unit it came from.
//
// Version 151 follows the canonical IR's schema to 166, whose source
// specialization can assign a `string` parameter. A version-149 record refused
// every module that declared one, so none holds a string parameter; rebuilding
// keeps the record and the schema in step.
//
// Version 152 keys a unit a library map arranged by each file's library, the
// map's declaration order and its configurations (IEEE 1364-2005 clause 13),
// which decide what every instance binds. The compiler now also parses
// `config` blocks, and a system task's scope argument may name a module by
// the cell of its `library.cell`, so a source an earlier build compiled one
// way can now resolve another. No artifact changes shape, so the schema
// stays; the fail-closed reading is to rebuild.
//
// Version 153 follows the canonical IR's schema to 167, whose digital plan
// carries procedural continuous assignments. A version-152 record refused
// every module that wrote one; rebuilding keeps the record and the schema in
// step.
//
// Version 158 follows the canonical IR's schema to 171, whose digital plan
// carries gated drivers. A version-157 record refused those sources.
//
// Version 157 follows the canonical IR's schema to 170, whose digital plan
// carries drive strengths and net types. A version-154 record refused those
// sources or would replay a plan without its strengths.
//
// Version 154 carries VA-SEM-RETAINED-VARIABLE, the finding that names a
// module's variables read before they are assigned. A version-153 record of
// such a module was compiled without it and would replay a clean report.
//
// Version 159 follows the canonical IR's schema to 172, whose digital plan
// records where each process was written. A version-158 record would replay
// a plan whose processes a diagnostic cannot place.
//
// Version 160 keys a record by the resolution the deck gives a plain `wreal`
// (`.options wreal_resolution`), which the compiler applies to the
// declarations. The compiler's options were not part of the key, so a
// version-159 record names no resolution and one compiled under another
// would read as this deck's.
//
// Version 161 follows the canonical IR's schema to 173, whose digital plan
// carries the bidirectional switches. A version-160 record refused them.
//
// Version 164 follows the canonical IR's schema to 175, whose digital plan
// carries `trireg` nets. A version-161 record refused them.
//
// Version 165 follows the canonical IR's schema to 176, whose ports carry
// their `supplySensitivity` and `groundSensitivity` attributes. A version-164
// record would replay ports without them, and the converters on those ports
// would convert against the deck's supply rather than the named net.
//
// Version 167 follows the canonical IR's schema to 177, which lists the SPICE
// leaves a design instantiates and the discrete ports it links on circuit
// nodes. A version-165 record would replay a design without them. A record is
// keyed by the cells the deck offers a design (its `.subckt`s and model
// cards), which decide what an instance of an undeclared module compiles to;
// a deck that offers none keys its records as before.
//
// Version 168 follows the canonical IR's schema to 178, whose digital plan
// identity covers the strength table. A version-167 record carries an
// identity that left it out. The plan also carries driver and net delays,
// which a version-167 record refused, and a record is keyed by the deck's
// min:typ:max selection (`.options hdl_delay_select`), which the parser
// applies.
//
// Version 170 follows the canonical IR's schema to 180, whose digital plan
// carries the module path delays of `specify` blocks. A version-168 record
// refused `specify`.
//
// Version 171 follows the schema to 181, whose digital plan carries the
// timing checks of `specify` blocks. A version-170 record refused them.
//
// Version 172 follows the schema to 182, whose digital plan may read a deck
// node or branch by path (`$cds_get_analog_value`) and whose HIR carries the
// local nets `analog initial` binds to deck nodes as hidden terminals. A
// version-171 record refused all three calls.
//
// Version 173 reads `log` as the decimal logarithm (Verilog-AMS LRM 2.4
// section 4.3.2). The serialized form is unchanged, but a version-172 record
// carries bytecode, folded defaults and lowered code that took it as `ln`.
//
// Version 174 follows the schema to 183, whose digital plan records where
// each signal was declared. A version-173 record would replay a plan whose
// signals a waveform cannot place.
//
// Version 175 follows the schema to 184, whose timing checks carry their
// delayed signals' delays computed across the checks sharing them, and
// their timestamp and timecheck conditions, and whose conditions read any
// discrete signal. A version-174 record refused a delayed net two checks
// name, both conditions, and a condition other than a scalar signal's.
//
// Version 176 is version 147's situation the other way round: the front end
// now refuses programs that IEEE 1364-2005 makes illegal and that it used to
// compile, among them a port whose two declarations state different ranges
// (section 12.3.3), a name used above its declaration (section 4.5), an
// unsized or real concatenation operand (sections 5.1.14 and 5.1.1) and a
// declaration in an unnamed `begin` block (section 9.8.1). No artifact
// changes shape, so the schema stays, but a cached artifact for such a
// program would still be served, and the fail-closed reading is to rebuild.
//
// Version 177 lowers a one-bit reduction of `z` to `x`, a real `$itor`
// argument through its rounded 32-bit integer, and a process written in a
// generate block under that block's scope, which `%m` names. The serialized
// form is unchanged, but an earlier record carries digital plans that
// passed the `z` through, kept the real's fraction, and printed the module's
// name.
//
// Version 178 follows the schema to 185, whose digital plan marks a net of
// a SystemVerilog real nettype apart from a `wreal`, so that a deck's
// `wreal_resolution` never refuses or folds one.
// Version 179 follows the schema to 186, which ties to ground an internal node
// that no contribution reaches.
//
// Version 180 opens a potential branch only for a guard on the contribution
// statement, never for a conditional inside its value. The serialized form is
// unchanged, but an earlier record carries activation conditions that opened
// `V(o) <+ c ? x : 0` whenever `c` was false.
//
// Version 181 differentiates `limexp` past its threshold to the constant
// slope of the straight line it continues as. The serialized form is
// unchanged, but an earlier record carries Jacobians that grew with the
// argument there, which let a junction driven far past its knee report a
// solution that violates KCL.
//
// Version 182 writes the record in the compact binary encoding of
// `veriloga_record`, sealed by a header that names the record version and the
// compiler build, with a checksum of every byte. The model's assignment
// passes, most of a large record, are set apart in chunks that a reader
// decodes in parallel. The compiler build is part of the file name too, so
// from here on a compiler change no longer needs a bump of this constant to
// keep its records apart.
pub(super) const VERILOGA_CACHE_RECORD_VERSION: u32 = 182;
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) const VERILOGA_CACHE_RECORD_EXTENSION: &str = "record";
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) const VERILOGA_CACHE_NATIVE_EXTENSION: &str = "native";
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) const VERILOGA_CACHE_TEMPORARY_EXTENSION: &str = "tmp";
/// A temporary file this old is a crashed writer's, never a write in progress.
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) const VERILOGA_CACHE_ABANDONED_TEMPORARY: Duration = Duration::from_secs(3600);
/// The fields of a compiled model its record sets apart in chunks: the
/// assignment passes, which are nearly all of a large model's record.
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
const DETACHED_MODEL_FIELDS: [&str; 2] = ["assignment_steps", "noise_assignment_steps"];
/// Small enough that a large model's passes spread over every core.
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
const RECORD_CHUNK_BYTES: usize = 8 * 1024 * 1024;
/// Decoding threads beyond this many stop paying for their memory traffic.
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
const RECORD_DECODE_THREADS: usize = 16;
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) const VERILOGA_CACHE_RENAME_RETRIES: u32 = 20;
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) const VERILOGA_CACHE_RENAME_BACKOFF: Duration = Duration::from_millis(25);
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) const VERILOGA_CACHE_DEFAULT_MAX_ENTRIES: usize = 512;
/// The records and native images of all the shipped CMC models together take
/// about 2 GB, the three HiSIM-HV variants most of it, so this holds every
/// one of them with room for a second compiler build's: the entries an older
/// build wrote are never read again and are the first to be evicted.
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) const VERILOGA_CACHE_DEFAULT_MAX_BYTES: u64 = 4 * 1024 * 1024 * 1024;
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) const VERILOGA_CACHE_MAX_ENTRIES_ENV: &str = "RSPICE_VERILOGA_CACHE_MAX_ENTRIES";
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) const VERILOGA_CACHE_MAX_BYTES_ENV: &str = "RSPICE_VERILOGA_CACHE_MAX_BYTES";

/// A refusal raised while resolving, compiling or reading back one Verilog-A
/// source, with that source as the subject and the span.
///
/// No instance is in scope here either: a source is compiled once for the
/// whole design and every X-card that names it shares the result, so blaming
/// one of them would be arbitrary.
#[cfg(feature = "veriloga")]
fn cache_refusal(
    path: &Path,
    kind: crate::ElaborationErrorKind,
    detail: impl Into<String>,
) -> SimulationError {
    crate::ElaborationError::new(kind, detail)
        .in_source(path)
        .into()
}

/// Preserve source admission outcomes for both discovery and compilation.
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) fn source_admission_error(
    path: &Path,
    context: &str,
    error: rspice_veriloga::PreprocessorError,
) -> SimulationError {
    use rspice_veriloga::preprocessor::SourceResource;
    if error.cancelled {
        return SimulationError::Aborted;
    }
    if let Some(limit) = error.resource_limit {
        return SimulationError::ResourceLimit(ResourceLimitError {
            resource: match limit.resource {
                SourceResource::IncludeDepth => ResourceKind::IncludeDepth,
                SourceResource::ExpandedBytes => ResourceKind::ExpandedSourceBytes,
                SourceResource::RootLines => ResourceKind::NetlistLines,
                SourceResource::RootBytes
                | SourceResource::TotalSourceBytes
                | SourceResource::Dependencies => ResourceKind::DependencySourceBytes,
            },
            requested: limit.requested,
            limit: limit.limit,
        });
    }
    crate::ElaborationError::new(crate::ElaborationErrorKind::CompileRefusal, context)
        .in_source(path)
        .with_compiler_diagnostics(vec![rspice_veriloga::SourceCompileDiagnostic::from(&error)])
        .into()
}

/// Process-lifetime telemetry for runtime Verilog-A cache lookups and compiles.
///
/// Counters are monotonic and lock-free. Take two snapshots to measure an
/// interval without globally resetting data used by another simulation.
#[cfg(feature = "veriloga")]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct VerilogACacheTelemetry {
    /// All attempted runtime lookups, including cancelled and invalid requests.
    pub lookups: u64,
    /// Fresh entries served from the bounded in-memory cache.
    pub memory_hits: u64,
    /// In-memory entries rejected after dependency fingerprint validation.
    pub stale_memory_entries: u64,
    /// Fresh, integrity-checked records restored from disk.
    pub disk_hits: u64,
    /// Lookups for which neither cache tier supplied an entry.
    pub misses: u64,
    /// Compiler invocations entered after a cache miss.
    pub compilations_started: u64,
    /// Compiler invocations that produced both runtime artifacts.
    pub compilations_succeeded: u64,
    /// Compiler invocations that returned a non-cancellation error.
    pub compilations_failed: u64,
    /// Compiler invocations stopped through cooperative cancellation.
    pub compilations_cancelled: u64,
    /// Saturating sum of compiler wall time across all completed invocations.
    pub total_compilation_nanos: u64,
    /// Compiled entries retained in memory but not persisted successfully.
    pub persistence_failures: u64,
}

#[cfg(feature = "veriloga")]
struct VerilogACacheTelemetryCounters {
    lookups: std::sync::atomic::AtomicU64,
    memory_hits: std::sync::atomic::AtomicU64,
    stale_memory_entries: std::sync::atomic::AtomicU64,
    disk_hits: std::sync::atomic::AtomicU64,
    misses: std::sync::atomic::AtomicU64,
    compilations_started: std::sync::atomic::AtomicU64,
    compilations_succeeded: std::sync::atomic::AtomicU64,
    compilations_failed: std::sync::atomic::AtomicU64,
    compilations_cancelled: std::sync::atomic::AtomicU64,
    total_compilation_nanos: std::sync::atomic::AtomicU64,
    persistence_failures: std::sync::atomic::AtomicU64,
}

#[cfg(feature = "veriloga")]
impl VerilogACacheTelemetryCounters {
    const fn new() -> Self {
        Self {
            lookups: std::sync::atomic::AtomicU64::new(0),
            memory_hits: std::sync::atomic::AtomicU64::new(0),
            stale_memory_entries: std::sync::atomic::AtomicU64::new(0),
            disk_hits: std::sync::atomic::AtomicU64::new(0),
            misses: std::sync::atomic::AtomicU64::new(0),
            compilations_started: std::sync::atomic::AtomicU64::new(0),
            compilations_succeeded: std::sync::atomic::AtomicU64::new(0),
            compilations_failed: std::sync::atomic::AtomicU64::new(0),
            compilations_cancelled: std::sync::atomic::AtomicU64::new(0),
            total_compilation_nanos: std::sync::atomic::AtomicU64::new(0),
            persistence_failures: std::sync::atomic::AtomicU64::new(0),
        }
    }
}

#[cfg(feature = "veriloga")]
static VERILOGA_CACHE_TELEMETRY: VerilogACacheTelemetryCounters =
    VerilogACacheTelemetryCounters::new();

/// Snapshot process-lifetime cache and compiler telemetry.
#[cfg(feature = "veriloga")]
pub fn veriloga_cache_telemetry() -> VerilogACacheTelemetry {
    use std::sync::atomic::Ordering::Relaxed;

    VerilogACacheTelemetry {
        lookups: VERILOGA_CACHE_TELEMETRY.lookups.load(Relaxed),
        memory_hits: VERILOGA_CACHE_TELEMETRY.memory_hits.load(Relaxed),
        stale_memory_entries: VERILOGA_CACHE_TELEMETRY.stale_memory_entries.load(Relaxed),
        disk_hits: VERILOGA_CACHE_TELEMETRY.disk_hits.load(Relaxed),
        misses: VERILOGA_CACHE_TELEMETRY.misses.load(Relaxed),
        compilations_started: VERILOGA_CACHE_TELEMETRY.compilations_started.load(Relaxed),
        compilations_succeeded: VERILOGA_CACHE_TELEMETRY
            .compilations_succeeded
            .load(Relaxed),
        compilations_failed: VERILOGA_CACHE_TELEMETRY.compilations_failed.load(Relaxed),
        compilations_cancelled: VERILOGA_CACHE_TELEMETRY
            .compilations_cancelled
            .load(Relaxed),
        total_compilation_nanos: VERILOGA_CACHE_TELEMETRY
            .total_compilation_nanos
            .load(Relaxed),
        persistence_failures: VERILOGA_CACHE_TELEMETRY.persistence_failures.load(Relaxed),
    }
}

#[cfg(feature = "veriloga")]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct VerilogADependencyFingerprint {
    pub(super) canonical_path: PathBuf,
    pub(super) modified_ns: Option<u128>,
    pub(super) file_len: u64,
    pub(super) content_hash: [u8; 32],
}

#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct VerilogADiskCacheRecord {
    pub(super) version: u32,
    pub(super) source_path: PathBuf,
    pub(super) selected_module: Option<String>,
    pub(super) configuration: Option<[u8; 32]>,
    pub(super) unit: Option<[u8; 32]>,
    pub(super) wreal_resolution: rspice_veriloga::canonical_ir::digital::DigitalRealResolution,
    #[serde(default)]
    pub(super) external_cells: Option<[u8; 32]>,
    #[serde(default)]
    pub(super) delay_select: rspice_veriloga::canonical_ir::digital::DigitalDelaySelect,
    /// Absent from a record compiled without a line's macros.
    #[serde(default)]
    pub(super) defines: Option<[u8; 32]>,
    pub(super) dependencies: Vec<VerilogADependencyFingerprint>,
    pub(super) model: rspice_veriloga::CompiledModel,
    pub(super) canonical_ir: Option<rspice_veriloga::canonical_ir::CanonicalIrArtifact>,
    pub(super) diagnostics: Vec<rspice_veriloga::SourceCompileDiagnostic>,
}

#[cfg(feature = "veriloga")]
#[derive(Debug, Clone)]
pub(super) struct CachedVerilogAModel {
    // Every production constructor validates the model/artifact pair before
    // this entry is admitted to the in-memory cache. Cache hits therefore only
    // need the comparatively cheap dependency freshness check.
    pub(super) dependencies: Vec<VerilogADependencyFingerprint>,
    pub(super) model: std::sync::Arc<rspice_veriloga::CompiledModel>,
    pub(super) canonical_ir:
        Option<std::sync::Arc<rspice_veriloga::canonical_ir::CanonicalIrArtifact>>,
    pub(super) diagnostics: std::sync::Arc<[rspice_veriloga::SourceCompileDiagnostic]>,
}

#[cfg(feature = "veriloga")]
impl CachedVerilogAModel {
    /// Compare executable artifacts only when independent includes share a name.
    /// Repeated includes normally share their Arcs and need no serialization.
    pub(super) fn has_same_artifact(&self, other: &Self) -> Result<bool, String> {
        if self.model.name != other.model.name {
            return Ok(false);
        }
        let same_canonical = match (&self.canonical_ir, &other.canonical_ir) {
            (Some(left), Some(right)) => std::sync::Arc::ptr_eq(left, right),
            (None, None) => true,
            _ => false,
        };
        if std::sync::Arc::ptr_eq(&self.model, &other.model) && same_canonical {
            return Ok(true);
        }
        Ok(
            runtime_artifact_fingerprint(&self.model, self.canonical_ir.as_deref())?
                == runtime_artifact_fingerprint(&other.model, other.canonical_ir.as_deref())?,
        )
    }
}

#[cfg(feature = "veriloga")]
type VerilogASourceCache = crate::resource::BoundedCache<VerilogASourceKey, CachedVerilogASource>;

/// Device code and standalone connection source share one retention budget and
/// registration transaction. A library never impersonates a device module.
#[cfg(feature = "veriloga")]
#[derive(Debug, Clone)]
pub(super) enum CachedVerilogASource {
    Runtime(CachedVerilogAModel),
    Connections(std::sync::Arc<rspice_veriloga::ConnectionLibraryArtifact>),
}

#[cfg(feature = "veriloga")]
impl CachedVerilogASource {
    fn runtime(&self) -> Option<&CachedVerilogAModel> {
        match self {
            Self::Runtime(model) => Some(model),
            Self::Connections(_) => None,
        }
    }

    fn artifact_fingerprint(&self) -> Result<[u8; 32], String> {
        match self {
            Self::Runtime(model) => {
                runtime_artifact_fingerprint(&model.model, model.canonical_ir.as_deref())
            }
            Self::Connections(library) => Ok(*library.identity()),
        }
    }
}

#[cfg(feature = "veriloga")]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct VerilogASourceKey {
    source_path: PathBuf,
    selected_module: Option<String>,
    configuration: Option<[u8; 32]>,
    /// The compilation unit of several files the module was compiled in;
    /// `None` when its own file was the whole unit.
    unit: Option<[u8; 32]>,
    /// The resolution the deck gives a plain `wreal` net, which the compiler
    /// applies to the declarations (`.options wreal_resolution`).
    wreal_resolution: rspice_veriloga::canonical_ir::digital::DigitalRealResolution,
    /// The SPICE cells the deck offers a design as leaves, by identity.
    external_cells: Option<[u8; 32]>,
    /// The value the deck keeps of each min:typ:max expression, which the
    /// parser applies (`.options hdl_delay_select`).
    delay_select: rspice_veriloga::canonical_ir::digital::DigitalDelaySelect,
    /// The macros the deck line predefined for the source, by identity;
    /// `None` when it defined none.
    defines: Option<[u8; 32]>,
}

#[cfg(feature = "veriloga")]
impl VerilogASourceKey {
    fn new(path: &Path, selected_module: Option<&str>) -> Self {
        Self {
            source_path: canonicalize_for_cache(path),
            selected_module: selected_module.map(str::to_owned),
            configuration: None,
            unit: None,
            wreal_resolution: Default::default(),
            external_cells: None,
            delay_select: Default::default(),
            defines: None,
        }
    }

    /// The same source compiled as `deck` compiles it.
    fn under(mut self, deck: &DeckHdl) -> Self {
        self.wreal_resolution = deck.wreal_resolution;
        self.external_cells = deck.cells_identity();
        self.delay_select = deck.delay_select;
        self.defines = deck.defines_identity();
        self
    }

    fn display(&self) -> std::path::Display<'_> {
        self.source_path.display()
    }
}

#[cfg(feature = "veriloga")]
impl From<PathBuf> for VerilogASourceKey {
    fn from(path: PathBuf) -> Self {
        Self::new(&path, None)
    }
}

#[cfg(feature = "veriloga")]
#[derive(Serialize)]
struct BorrowedVerilogACacheRecord<'a> {
    version: u32,
    source_path: &'a Path,
    selected_module: Option<&'a str>,
    configuration: Option<[u8; 32]>,
    unit: Option<[u8; 32]>,
    wreal_resolution: rspice_veriloga::canonical_ir::digital::DigitalRealResolution,
    external_cells: Option<[u8; 32]>,
    delay_select: rspice_veriloga::canonical_ir::digital::DigitalDelaySelect,
    // A record compiled without a line's macros is written as it always was.
    #[serde(skip_serializing_if = "Option::is_none")]
    defines: Option<[u8; 32]>,
    dependencies: &'a [VerilogADependencyFingerprint],
    model: &'a rspice_veriloga::CompiledModel,
    canonical_ir: Option<&'a rspice_veriloga::canonical_ir::CanonicalIrArtifact>,
    diagnostics: &'a [rspice_veriloga::SourceCompileDiagnostic],
}

#[cfg(feature = "veriloga")]
impl<'a> BorrowedVerilogACacheRecord<'a> {
    fn new(source: &'a VerilogASourceKey, entry: &'a CachedVerilogAModel) -> Self {
        Self {
            version: VERILOGA_CACHE_RECORD_VERSION,
            source_path: &source.source_path,
            selected_module: source.selected_module.as_deref(),
            configuration: source.configuration,
            unit: source.unit,
            wreal_resolution: source.wreal_resolution,
            external_cells: source.external_cells,
            delay_select: source.delay_select,
            defines: source.defines,
            dependencies: &entry.dependencies,
            model: entry.model.as_ref(),
            canonical_ir: entry.canonical_ir.as_deref(),
            diagnostics: &entry.diagnostics,
        }
    }
}

#[cfg(feature = "veriloga")]
#[derive(Default)]
struct CountingWriter {
    bytes: usize,
}

#[cfg(feature = "veriloga")]
impl std::io::Write for CountingWriter {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.bytes = self.bytes.saturating_add(bytes.len());
        Ok(bytes.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[cfg(feature = "veriloga")]
fn veriloga_model_cache_entry_bytes(
    key: &VerilogASourceKey,
    entry: &CachedVerilogAModel,
) -> Result<usize, String> {
    let encoded_bytes = veriloga_record::encoded_len(&BorrowedVerilogACacheRecord::new(key, entry))
        .map_err(|error| format!("failed to size Verilog-A cache entry: {error}"))?;
    Ok(veriloga_retained_entry_bytes(key, encoded_bytes))
}

/// What the shared memory cache charges for an entry whose record encodes to
/// `encoded_bytes`: the record is the compact measure of everything it holds.
#[cfg(feature = "veriloga")]
fn veriloga_retained_entry_bytes(key: &VerilogASourceKey, encoded_bytes: usize) -> usize {
    let key_bytes = key
        .source_path
        .to_string_lossy()
        .len()
        .saturating_add(key.selected_module.as_ref().map_or(0, String::len));
    crate::resource::estimated_cache_entry_bytes::<VerilogASourceKey, CachedVerilogASource>(
        key_bytes,
        encoded_bytes,
    )
}

#[cfg(feature = "veriloga")]
fn veriloga_source_cache_entry_bytes(
    key: &VerilogASourceKey,
    entry: &CachedVerilogASource,
) -> Result<usize, String> {
    match entry {
        CachedVerilogASource::Runtime(model) => veriloga_model_cache_entry_bytes(key, model),
        CachedVerilogASource::Connections(library) => {
            let mut counter = CountingWriter::default();
            serde_json::to_writer(&mut counter, library.as_ref())
                .map_err(|error| format!("failed to size connection library artifact: {error}"))?;
            Ok(crate::resource::estimated_cache_entry_bytes::<
                VerilogASourceKey,
                CachedVerilogASource,
            >(
                key.source_path.to_string_lossy().len(), counter.bytes
            ))
        }
    }
}

#[cfg(feature = "veriloga")]
fn retain_veriloga_model(
    key: impl Into<VerilogASourceKey>,
    entry: CachedVerilogAModel,
    max_bytes: usize,
    required: bool,
) -> Result<bool, String> {
    let key = key.into();
    let retained_bytes = veriloga_model_cache_entry_bytes(&key, &entry)?;
    retain_veriloga_model_sized(key, entry, retained_bytes, max_bytes, required)
}

/// [`retain_veriloga_model`] for an entry whose retained size is known.
#[cfg(feature = "veriloga")]
fn retain_veriloga_model_sized(
    key: VerilogASourceKey,
    entry: CachedVerilogAModel,
    retained_bytes: usize,
    max_bytes: usize,
    required: bool,
) -> Result<bool, String> {
    if let Err(error) =
        ResourceLimitError::ensure(ResourceKind::SharedCacheBytes, retained_bytes, max_bytes)
    {
        if required {
            return Err(error.to_string());
        }
        return Ok(false);
    }

    let mut cache = veriloga_source_cache()
        .write()
        .map_err(|_| "failed to acquire Verilog-A cache lock".to_owned())?;
    if matches!(cache.get(&key), Some(CachedVerilogASource::Connections(_))) {
        return Err(format!(
            "Verilog-A source '{}' is registered as a connection library and cannot be replaced by a device",
            key.display()
        ));
    }
    cache.enforce_limit(max_bytes);
    cache.remove(&key);
    cache.insert_or_get(
        key.clone(),
        CachedVerilogASource::Runtime(entry),
        retained_bytes,
        max_bytes,
    );
    let retained = cache.get(&key).is_some();
    if required && !retained {
        return Err(format!(
            "unable to retain Verilog-A runtime '{}' in the shared cache",
            key.display()
        ));
    }
    Ok(retained)
}

/// Verify that cached bytecode and canonical IR can safely be paired at runtime.
///
/// A persisted entry is an optimization, never an authority. In particular,
/// older compiler builds can deserialize after an IR-digest change while still
/// looking structurally valid to serde. Treat that as a cache miss here rather
/// than allowing a stale artifact to reach the native JIT.
#[cfg(feature = "veriloga")]
fn validate_runtime_artifact_pair(
    model: &rspice_veriloga::CompiledModel,
    canonical_ir: Option<&rspice_veriloga::canonical_ir::CanonicalIrArtifact>,
) -> Result<(), String> {
    let Some(artifact) = required_canonical_artifact(canonical_ir)? else {
        return Ok(());
    };
    artifact.validate().map_err(artifact_integrity_failure)?;
    artifact_pair_agrees(model, artifact)
}

#[cfg(feature = "veriloga")]
fn artifact_integrity_failure(
    diagnostics: Vec<rspice_veriloga::canonical_ir::IrDiagnostic>,
) -> String {
    let detail = diagnostics
        .first()
        .map(|diagnostic| diagnostic.message.as_str())
        .unwrap_or("canonical artifact validation failed");
    format!("canonical Verilog-A artifact failed integrity validation: {detail}")
}

/// Verify a pair read back from this build's own disk cache.
///
/// The record header proves the bytes are the ones this compiler build wrote
/// after [`validate_runtime_artifact_pair`] passed them, so the canonical
/// artifact's full self-validation, a third of a second for the largest
/// compact models, would only repeat itself. The pairing is still checked,
/// and so is the digital plan: its stored identity is what a device build
/// compares, so a plan the record did not carry whole has to be a cache miss
/// here and not a source the device refuses.
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
fn validate_cached_artifact_pair(
    model: &rspice_veriloga::CompiledModel,
    canonical_ir: Option<&rspice_veriloga::canonical_ir::CanonicalIrArtifact>,
) -> Result<(), String> {
    match required_canonical_artifact(canonical_ir)? {
        Some(artifact) => {
            artifact
                .digital
                .validate()
                .map_err(artifact_integrity_failure)?;
            artifact_pair_agrees(model, artifact)
        }
        None => Ok(()),
    }
}

/// The canonical artifact a runtime pair must carry where the native
/// backend runs it, which has no interpreter to fall back on.
#[cfg(feature = "veriloga")]
fn required_canonical_artifact(
    canonical_ir: Option<&rspice_veriloga::canonical_ir::CanonicalIrArtifact>,
) -> Result<Option<&rspice_veriloga::canonical_ir::CanonicalIrArtifact>, String> {
    #[cfg(feature = "veriloga-native")]
    if canonical_ir.is_none() {
        return Err(
            "native Verilog-A runtime cache entry requires canonical IR (no interpreter fallback)"
                .to_string(),
        );
    }
    Ok(canonical_ir)
}

#[cfg(feature = "veriloga")]
fn artifact_pair_agrees(
    model: &rspice_veriloga::CompiledModel,
    artifact: &rspice_veriloga::canonical_ir::CanonicalIrArtifact,
) -> Result<(), String> {
    if artifact.metadata.source_digest != model.source_digest {
        return Err(format!(
            "canonical Verilog-A source digest '{}' does not match compiled model digest '{}'",
            artifact.metadata.source_digest, model.source_digest
        ));
    }
    if artifact.mir.module_name != model.name {
        return Err(format!(
            "canonical Verilog-A module '{}' does not match compiled model '{}'",
            artifact.mir.module_name, model.name
        ));
    }
    if artifact.mir.equations.len() != model.stamp_programs.len() {
        return Err(format!(
            "canonical Verilog-A equation count {} does not match compiled stamp count {}",
            artifact.mir.equations.len(),
            model.stamp_programs.len()
        ));
    }

    Ok(())
}

#[cfg(feature = "veriloga")]
pub(super) fn veriloga_source_cache() -> &'static RwLock<VerilogASourceCache> {
    static CACHE: OnceLock<RwLock<VerilogASourceCache>> = OnceLock::new();
    CACHE.get_or_init(|| RwLock::new(VerilogASourceCache::default()))
}

#[cfg(feature = "veriloga")]
pub(super) fn canonicalize_for_cache(path: &Path) -> PathBuf {
    if is_sealed_veriloga_virtual_path(path) {
        return PathBuf::from(path.to_string_lossy().replace('\\', "/"));
    }
    std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}

#[cfg(feature = "veriloga")]
pub(super) fn is_sealed_veriloga_virtual_path(path: &Path) -> bool {
    let normalized = path.to_string_lossy().replace('\\', "/");
    normalized.split_once('/').is_some_and(|(root, _)| {
        root.eq_ignore_ascii_case("__rspice_project__")
            || root.eq_ignore_ascii_case("__rspice_pdk__")
            || root.eq_ignore_ascii_case("__rspice_model_library__")
    })
}

#[cfg(feature = "veriloga")]
pub(super) fn normalize_model_key(name: &str) -> String {
    name.to_ascii_uppercase()
}

#[cfg(feature = "veriloga")]
pub(super) fn metadata_modified_ns(metadata: &std::fs::Metadata) -> Option<u128> {
    use std::time::UNIX_EPOCH;

    metadata
        .modified()
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_nanos())
}

#[cfg(feature = "veriloga")]
enum VerilogADependencyReadError {
    Io(std::io::Error),
    ResourceLimit(ResourceLimitError),
    Cancelled,
}

#[cfg(feature = "veriloga")]
fn hash_dependency_file_with_limits(
    path: &Path,
    bytes_already_read: usize,
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<([u8; 32], std::fs::Metadata, usize), VerilogADependencyReadError> {
    if abort.is_aborted() {
        return Err(VerilogADependencyReadError::Cancelled);
    }
    let mut file = std::fs::File::open(path)?;
    let metadata = file.metadata()?;
    let metadata_bytes = usize::try_from(metadata.len()).unwrap_or(usize::MAX);
    ResourceLimitError::ensure(
        ResourceKind::DependencySourceBytes,
        bytes_already_read.saturating_add(metadata_bytes),
        limits.max_dependency_source_bytes,
    )?;
    let mut hasher = blake3::Hasher::new();
    let mut buffer = [0_u8; 64 * 1024];
    let mut bytes_read = 0_usize;

    loop {
        if abort.is_aborted() {
            return Err(VerilogADependencyReadError::Cancelled);
        }
        let read = file.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        bytes_read = bytes_read.saturating_add(read);
        ResourceLimitError::ensure(
            ResourceKind::DependencySourceBytes,
            bytes_already_read.saturating_add(bytes_read),
            limits.max_dependency_source_bytes,
        )?;
        hasher.update(&buffer[..read]);
    }

    Ok((*hasher.finalize().as_bytes(), metadata, bytes_read))
}

#[cfg(feature = "veriloga")]
impl From<std::io::Error> for VerilogADependencyReadError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

#[cfg(feature = "veriloga")]
impl From<ResourceLimitError> for VerilogADependencyReadError {
    fn from(error: ResourceLimitError) -> Self {
        Self::ResourceLimit(error)
    }
}

#[cfg(feature = "veriloga")]
// Reached only through the not-wasm32 `fingerprint_paths` below; the allow
// covers the browser target's view, where there is no disk to fingerprint.
#[allow(dead_code)]
fn fingerprint_paths_with_limits(
    paths: &[PathBuf],
    limits: ResourceLimits,
) -> Result<Vec<VerilogADependencyFingerprint>, SimulationError> {
    fingerprint_paths_with_limits_and_abort(paths, limits, &NoAbort)
}

#[cfg(feature = "veriloga")]
fn fingerprint_paths_with_limits_and_abort(
    paths: &[PathBuf],
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<Vec<VerilogADependencyFingerprint>, SimulationError> {
    let mut canonical_paths: Vec<PathBuf> =
        paths.iter().map(|p| canonicalize_for_cache(p)).collect();
    canonical_paths.sort();
    canonical_paths.dedup();

    let mut fingerprints = Vec::with_capacity(canonical_paths.len());
    let mut dependency_bytes = 0_usize;
    for canonical_path in canonical_paths {
        match hash_dependency_file_with_limits(&canonical_path, dependency_bytes, limits, abort) {
            Ok((content_hash, metadata, bytes_read)) => {
                dependency_bytes = dependency_bytes.saturating_add(bytes_read);
                fingerprints.push(VerilogADependencyFingerprint {
                    canonical_path,
                    modified_ns: metadata_modified_ns(&metadata),
                    file_len: metadata.len(),
                    content_hash,
                });
            }
            Err(VerilogADependencyReadError::ResourceLimit(error)) => return Err(error.into()),
            Err(VerilogADependencyReadError::Cancelled) => {
                return Err(SimulationError::Aborted);
            }
            Err(VerilogADependencyReadError::Io(error)) => {
                return Err(cache_refusal(
                    &canonical_path,
                    crate::ElaborationErrorKind::MissingSource,
                    format!("this dependency does not exist or is unreadable: {error}"),
                ));
            }
        }
    }

    Ok(fingerprints)
}

#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) fn fingerprint_paths(
    paths: &[PathBuf],
) -> Result<Vec<VerilogADependencyFingerprint>, SimulationError> {
    fingerprint_paths_with_limits(paths, ResourceLimits::default())
}

#[cfg(feature = "veriloga")]
fn dependencies_are_fresh_with_limits_and_abort(
    dependencies: &[VerilogADependencyFingerprint],
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<bool, SimulationError> {
    let mut dependency_bytes = 0_usize;
    for dependency in dependencies {
        match hash_dependency_file_with_limits(
            &dependency.canonical_path,
            dependency_bytes,
            limits,
            abort,
        ) {
            Ok((content_hash, _, bytes_read)) => {
                dependency_bytes = dependency_bytes.saturating_add(bytes_read);
                if content_hash != dependency.content_hash {
                    return Ok(false);
                }
            }
            Err(VerilogADependencyReadError::ResourceLimit(error)) => return Err(error.into()),
            Err(VerilogADependencyReadError::Cancelled) => {
                return Err(SimulationError::Aborted);
            }
            Err(VerilogADependencyReadError::Io(_)) => return Ok(false),
        }
    }
    Ok(true)
}

#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) fn parse_cache_env_usize(name: &str, default: usize) -> usize {
    std::env::var(name)
        .ok()
        .and_then(|raw| raw.trim().parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default)
}

#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) fn parse_cache_env_u64(name: &str, default: u64) -> u64 {
    std::env::var(name)
        .ok()
        .and_then(|raw| raw.trim().parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default)
}

#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) fn veriloga_cache_limits() -> (usize, u64) {
    (
        parse_cache_env_usize(
            VERILOGA_CACHE_MAX_ENTRIES_ENV,
            VERILOGA_CACHE_DEFAULT_MAX_ENTRIES,
        ),
        parse_cache_env_u64(
            VERILOGA_CACHE_MAX_BYTES_ENV,
            VERILOGA_CACHE_DEFAULT_MAX_BYTES,
        ),
    )
}

// `std::env::temp_dir` and the file system abort on wasm32-unknown-unknown
// rather than erroring, so the entire disk side of the cache only exists on
// native targets.
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) fn veriloga_cache_root() -> PathBuf {
    if let Some(override_dir) = std::env::var_os("RSPICE_VERILOGA_CACHE_DIR") {
        return PathBuf::from(override_dir);
    }

    if let Some(cache_dir) = dirs::cache_dir() {
        return cache_dir.join("rspice").join("veriloga");
    }

    std::env::temp_dir().join("rspice-veriloga-cache")
}

#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) fn cache_record_path_with_root(
    source: &VerilogASourceKey,
    cache_root: &Path,
) -> PathBuf {
    let mut hasher = blake3::Hasher::new();
    // Another compiler build writes its records under other names, so builds
    // sharing one cache directory never read or replace each other's.
    hasher.update(veriloga_compiler_identity());
    hasher.update(b"\0source\0");
    hasher.update(source.source_path.to_string_lossy().as_bytes());
    if let Some(module) = &source.selected_module {
        // The zero delimiter cannot occur in a filesystem path. Module names
        // retain Verilog's case sensitivity, independently of SPICE aliases.
        hasher.update(b"\0module\0");
        hasher.update(module.as_bytes());
    }
    if let Some(configuration) = source.configuration {
        hasher.update(b"\0configuration\0");
        hasher.update(&configuration);
    }
    if let Some(unit) = source.unit {
        hasher.update(b"\0unit\0");
        hasher.update(&unit);
    }
    if source.wreal_resolution != Default::default() {
        hasher.update(b"\0wreal\0");
        hasher.update(source.wreal_resolution.keyword().as_bytes());
    }
    if let Some(cells) = source.external_cells {
        hasher.update(b"\0external-cells\0");
        hasher.update(&cells);
    }
    if let Some(defines) = source.defines {
        hasher.update(b"\0defines\0");
        hasher.update(&defines);
    }
    if source.delay_select != Default::default() {
        hasher.update(b"\0delay-select\0");
        hasher.update(source.delay_select.keyword().as_bytes());
    }
    let key = hasher.finalize().to_hex().to_string();
    cache_root.join(format!("{key}.{VERILOGA_CACHE_RECORD_EXTENSION}"))
}

/// The Verilog-A compiler this build links: a digest of its sources, its
/// dependency lock, its version and the target, taken by the build script.
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) fn veriloga_compiler_identity() -> &'static [u8; 32] {
    static IDENTITY: OnceLock<[u8; 32]> = OnceLock::new();
    IDENTITY.get_or_init(|| {
        *blake3::hash(env!("RSPICE_VERILOGA_COMPILER_IDENTITY").as_bytes()).as_bytes()
    })
}

#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
#[derive(Debug, Clone)]
pub(super) struct VerilogACacheFileInfo {
    path: PathBuf,
    size_bytes: u64,
    modified_ns: Option<u128>,
    temporary: bool,
}

/// Cache files this build manages: records, native images, and the
/// temporaries they are written through. Anything else in the directory is
/// left alone.
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) fn list_cache_files(cache_root: &Path) -> Result<Vec<VerilogACacheFileInfo>, String> {
    let dir_iter = match std::fs::read_dir(cache_root) {
        Ok(dir_iter) => dir_iter,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => {
            return Err(format!(
                "failed to list cache directory '{}': {error}",
                cache_root.display()
            ));
        }
    };
    let mut files = Vec::new();
    for entry in dir_iter {
        let entry = entry.map_err(|e| format!("failed to read cache directory entry: {}", e))?;
        let path = entry.path();
        let temporary = match path.extension().and_then(|e| e.to_str()) {
            Some(VERILOGA_CACHE_RECORD_EXTENSION | VERILOGA_CACHE_NATIVE_EXTENSION) => false,
            Some(VERILOGA_CACHE_TEMPORARY_EXTENSION) => true,
            _ => continue,
        };
        // A file another process removed since the listing is simply gone.
        let Ok(metadata) = entry.metadata() else {
            continue;
        };
        if !metadata.is_file() {
            continue;
        }
        files.push(VerilogACacheFileInfo {
            path,
            size_bytes: metadata.len(),
            modified_ns: metadata_modified_ns(&metadata),
            temporary,
        });
    }
    Ok(files)
}

#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
enum VerilogACacheRecordReadError {
    Missing,
    Invalid(String),
    ResourceLimit(ResourceLimitError),
    Cancelled,
}

/// The message an interrupted or oversized cache-record read carries.
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
const CACHE_RECORD_READ: &str = "Verilog-A cache record";

/// What every record file starts with.
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
const RECORD_MAGIC: [u8; 8] = *b"RSPVAREC";

/// What every native image file starts with.
#[cfg(all(feature = "veriloga-native", not(target_arch = "wasm32")))]
const NATIVE_IMAGE_MAGIC: [u8; 8] = *b"RSPVANAT";

/// Magic, record version, compiler identity, payload length, digest.
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
const RECORD_HEADER_BYTES: usize = 8 + 4 + 32 + 8 + 32;

/// The header that seals a payload of `payload_len` bytes vouched for by
/// `digest`: whoever reads it back checks that this kind of file, record
/// version and compiler wrote it, and that the bytes are the ones written.
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
fn sealing_header(
    magic: [u8; 8],
    payload_len: usize,
    digest: &blake3::Hash,
) -> [u8; RECORD_HEADER_BYTES] {
    let mut header = [0_u8; RECORD_HEADER_BYTES];
    header[..8].copy_from_slice(&magic);
    header[8..12].copy_from_slice(&VERILOGA_CACHE_RECORD_VERSION.to_le_bytes());
    header[12..44].copy_from_slice(veriloga_compiler_identity());
    header[44..52].copy_from_slice(&(payload_len as u64).to_le_bytes());
    header[52..].copy_from_slice(digest.as_bytes());
    header
}

/// A sealed file's payload and the digest its header declares for it, once
/// the header names this kind of file, record version and compiler build.
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
fn sealed_payload_and_digest(
    magic: [u8; 8],
    bytes: &[u8],
) -> Result<(&[u8], [u8; 32]), &'static str> {
    if bytes.len() < RECORD_HEADER_BYTES || bytes[..8] != magic {
        return Err("it is not that kind of Verilog-A cache file");
    }
    if bytes[8..12] != VERILOGA_CACHE_RECORD_VERSION.to_le_bytes() {
        return Err("it was written in another record version");
    }
    if bytes[12..44] != *veriloga_compiler_identity() {
        return Err("another compiler build wrote it");
    }
    let payload = &bytes[RECORD_HEADER_BYTES..];
    if bytes[44..52] != (payload.len() as u64).to_le_bytes() {
        return Err("it is truncated");
    }
    let mut digest = [0_u8; 32];
    digest.copy_from_slice(&bytes[52..RECORD_HEADER_BYTES]);
    Ok((payload, digest))
}

/// The payload of a sealed file whose digest covers the whole payload.
#[cfg(all(feature = "veriloga-native", not(target_arch = "wasm32")))]
fn sealed_payload(magic: [u8; 8], bytes: &[u8]) -> Result<&[u8], &'static str> {
    let (payload, digest) = sealed_payload_and_digest(magic, bytes)?;
    if *blake3::hash(payload).as_bytes() != digest {
        return Err("its contents do not match their checksum");
    }
    Ok(payload)
}

/// A record payload: a table, then the sections it lists.
///
/// The first section is the record with its detached fields empty; each
/// detached field follows as its chunks, in [`DETACHED_MODEL_FIELDS`] order.
/// The header's digest covers the table, and the table carries a digest for
/// every section, so the sections are verified where they are decoded, side
/// by side, and nothing is decoded before its bytes are vouched for.
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
struct RecordSections<'a> {
    chunk_counts: [usize; DETACHED_MODEL_FIELDS.len()],
    sections: Vec<(&'a [u8], [u8; 32])>,
}

#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
const SECTION_ENTRY_BYTES: usize = 8 + 32;

#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
fn record_section_table(encoded: &veriloga_record::DetachedEncoding) -> Vec<u8> {
    let sections: Vec<&[u8]> = std::iter::once(&encoded.main)
        .chain(encoded.chunks.iter().flatten())
        .map(Vec::as_slice)
        .collect();
    let mut table = Vec::with_capacity(8 + 4 * encoded.chunks.len() + 40 * sections.len());
    for chunks in &encoded.chunks {
        table.extend_from_slice(&(chunks.len() as u32).to_le_bytes());
    }
    table.extend_from_slice(&(sections.len() as u32).to_le_bytes());
    for section in sections {
        table.extend_from_slice(&(section.len() as u64).to_le_bytes());
        table.extend_from_slice(blake3::hash(section).as_bytes());
    }
    table
}

#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
impl<'a> RecordSections<'a> {
    fn parse(payload: &'a [u8], digest: &[u8; 32]) -> Result<Self, String> {
        fn take<'p>(payload: &'p [u8], at: &mut usize, len: usize) -> Result<&'p [u8], String> {
            let bytes = at
                .checked_add(len)
                .and_then(|end| payload.get(*at..end))
                .ok_or_else(|| "its section table is truncated".to_string())?;
            *at += len;
            Ok(bytes)
        }
        fn count(bytes: &[u8]) -> usize {
            u32::from_le_bytes(bytes.try_into().expect("four bytes")) as usize
        }

        let mut at = 0;
        let mut chunk_counts = [0; DETACHED_MODEL_FIELDS.len()];
        for chunks in &mut chunk_counts {
            *chunks = count(take(payload, &mut at, 4)?);
        }
        let section_count = count(take(payload, &mut at, 4)?);
        let detached = chunk_counts
            .iter()
            .try_fold(1_usize, |sum, chunks| sum.checked_add(*chunks));
        if detached != Some(section_count) {
            return Err(format!(
                "its table lists {section_count} sections for {chunk_counts:?} chunks"
            ));
        }
        let entries = take(
            payload,
            &mut at,
            section_count
                .checked_mul(SECTION_ENTRY_BYTES)
                .ok_or_else(|| "its section table is truncated".to_string())?,
        )?;
        if *blake3::hash(&payload[..at]).as_bytes() != *digest {
            return Err("its contents do not match their checksum".into());
        }
        let mut sections = Vec::with_capacity(section_count);
        for entry in entries.chunks_exact(SECTION_ENTRY_BYTES) {
            let len = usize::try_from(u64::from_le_bytes(
                entry[..8].try_into().expect("eight bytes"),
            ))
            .map_err(|_| "a section is longer than memory".to_string())?;
            let mut section_digest = [0_u8; 32];
            section_digest.copy_from_slice(&entry[8..]);
            sections.push((take(payload, &mut at, len)?, section_digest));
        }
        if at != payload.len() {
            return Err(format!(
                "{} bytes follow its last section",
                payload.len() - at
            ));
        }
        Ok(Self {
            chunk_counts,
            sections,
        })
    }

    /// Verify and decode every section on several threads, then join the
    /// detached chunks back into the record.
    fn decode(self) -> Result<VerilogADiskCacheRecord, String> {
        use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

        enum Decoded {
            Record(Box<VerilogADiskCacheRecord>),
            Steps(Vec<rspice_veriloga::codegen::AssignmentStep>),
        }

        let sections = &self.sections;
        let next = AtomicUsize::new(0);
        let decode_section = |index: usize| -> Result<Decoded, String> {
            let (bytes, digest) = sections[index];
            if *blake3::hash(bytes).as_bytes() != digest {
                return Err("its contents do not match their checksum".into());
            }
            let decoded = if index == 0 {
                veriloga_record::decode(bytes).map(|record| Decoded::Record(Box::new(record)))
            } else {
                veriloga_record::decode(bytes).map(Decoded::Steps)
            };
            decoded.map_err(|error| error.to_string())
        };
        let threads = std::thread::available_parallelism()
            .map_or(1, |threads| threads.get())
            .min(RECORD_DECODE_THREADS)
            .min(sections.len());
        let mut decoded: Vec<Option<Decoded>> = (0..sections.len()).map(|_| None).collect();
        let finished = std::thread::scope(|scope| {
            let workers: Vec<_> = (0..threads)
                .map(|_| {
                    scope.spawn(|| {
                        let mut finished = Vec::new();
                        loop {
                            let index = next.fetch_add(1, Relaxed);
                            if index >= sections.len() {
                                return finished;
                            }
                            finished.push((index, decode_section(index)));
                        }
                    })
                })
                .collect();
            workers
                .into_iter()
                .flat_map(|worker| {
                    worker
                        .join()
                        .unwrap_or_else(|panic| std::panic::resume_unwind(panic))
                })
                .collect::<Vec<_>>()
        });
        for (index, section) in finished {
            decoded[index] = Some(section?);
        }

        let mut decoded = decoded
            .into_iter()
            .map(|section| section.expect("every section"));
        let Some(Decoded::Record(mut record)) = decoded.next() else {
            return Err("its first section is not a record".into());
        };
        let model = &mut record.model;
        let fields = [
            &mut model.assignment_steps,
            &mut model.noise_assignment_steps,
        ];
        for ((field, chunks), steps) in DETACHED_MODEL_FIELDS
            .iter()
            .zip(self.chunk_counts)
            .zip(fields)
        {
            if chunks == 0 {
                continue;
            }
            if !steps.is_empty() {
                return Err(format!("its {field} are both inline and detached"));
            }
            for _ in 0..chunks {
                match decoded.next() {
                    Some(Decoded::Steps(chunk)) => steps.extend(chunk),
                    _ => return Err(format!("a chunk of its {field} is not a pass")),
                }
            }
        }
        Ok(*record)
    }
}

/// Read and decode one record, returning it with its encoded size.
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
fn read_cache_record_with_limits_and_abort(
    path: &Path,
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<(VerilogADiskCacheRecord, usize), VerilogACacheRecordReadError> {
    let bytes = read_cache_file_with_limits_and_abort(path, limits, abort)?;
    unseal_record(&bytes).map_err(|reason| {
        VerilogACacheRecordReadError::Invalid(format!(
            "discarding Verilog-A cache record '{}': {reason}",
            path.display()
        ))
    })
}

/// Decode a sealed record file, returning the record with its payload size.
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
fn unseal_record(bytes: &[u8]) -> Result<(VerilogADiskCacheRecord, usize), String> {
    let (payload, digest) = sealed_payload_and_digest(RECORD_MAGIC, bytes)?;
    let record = RecordSections::parse(payload, &digest)?.decode()?;
    Ok((record, payload.len()))
}

/// A record encoded and sealed for the disk: its header, section table and
/// sections, written one after another.
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
struct SealedRecord {
    header: [u8; RECORD_HEADER_BYTES],
    table: Vec<u8>,
    sections: veriloga_record::DetachedEncoding,
}

#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
impl SealedRecord {
    fn new<T: serde::Serialize + ?Sized>(record: &T) -> Result<Self, String> {
        let sections = veriloga_record::encode_detached(
            record,
            "model",
            &DETACHED_MODEL_FIELDS,
            RECORD_CHUNK_BYTES,
        )
        .map_err(|error| format!("failed to encode Verilog-A cache record: {error}"))?;
        let table = record_section_table(&sections);
        let mut sealed = Self {
            header: [0; RECORD_HEADER_BYTES],
            table,
            sections,
        };
        sealed.header = sealing_header(
            RECORD_MAGIC,
            sealed.payload_len(),
            &blake3::hash(&sealed.table),
        );
        Ok(sealed)
    }

    fn parts(&self) -> impl Iterator<Item = &[u8]> {
        [&self.table, &self.sections.main]
            .into_iter()
            .chain(self.sections.chunks.iter().flatten())
            .map(Vec::as_slice)
    }

    fn payload_len(&self) -> usize {
        self.parts().map(<[u8]>::len).sum()
    }

    fn file_parts(&self) -> Vec<&[u8]> {
        std::iter::once(&self.header[..])
            .chain(self.parts())
            .collect()
    }
}

/// Read one whole cache file within the shared cache budget.
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
fn read_cache_file_with_limits_and_abort(
    path: &Path,
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<Vec<u8>, VerilogACacheRecordReadError> {
    let file = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Err(VerilogACacheRecordReadError::Missing);
        }
        Err(error) => {
            return Err(VerilogACacheRecordReadError::Invalid(format!(
                "failed to read Verilog-A cache file '{}': {}",
                path.display(),
                error
            )));
        }
    };
    let metadata_bytes = file
        .metadata()
        .map(|metadata| usize::try_from(metadata.len()).unwrap_or(usize::MAX))
        .map_err(|error| {
            VerilogACacheRecordReadError::Invalid(format!(
                "failed to inspect Verilog-A cache file '{}': {}",
                path.display(),
                error
            ))
        })?;
    ResourceLimitError::ensure(
        ResourceKind::SharedCacheBytes,
        metadata_bytes,
        limits.max_shared_cache_bytes,
    )
    .map_err(VerilogACacheRecordReadError::ResourceLimit)?;

    let mut bytes = Vec::with_capacity(metadata_bytes);
    let mut reader = crate::abort_signal::AbortReader::with_byte_cap(
        file,
        abort,
        CACHE_RECORD_READ,
        limits.max_shared_cache_bytes,
    );
    if let Err(error) = reader.read_to_end(&mut bytes) {
        if reader.was_cancelled() {
            return Err(VerilogACacheRecordReadError::Cancelled);
        }
        if reader.exceeded_cap() {
            return Err(VerilogACacheRecordReadError::ResourceLimit(
                ResourceLimitError {
                    resource: ResourceKind::SharedCacheBytes,
                    requested: limits.max_shared_cache_bytes.saturating_add(1),
                    limit: limits.max_shared_cache_bytes,
                },
            ));
        }
        return Err(VerilogACacheRecordReadError::Invalid(format!(
            "failed to read Verilog-A cache file '{}': {}",
            path.display(),
            error
        )));
    }
    Ok(bytes)
}

/// Publish a cache file whole or not at all.
///
/// The bytes go to a temporary file of this writer's own, which is then
/// renamed over the destination. A rename within one directory replaces the
/// destination atomically, so a reader in another process opens either the
/// file that was there or the new one, never a mixture, and writers never
/// wait on readers or on each other. The header checksum covers what the
/// rename cannot: a file the system lost part of in a crash is refused on
/// read, never decoded.
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
fn write_cache_file_atomically(path: &Path, parts: &[&[u8]]) -> Result<(), String> {
    use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
    static NEXT_TEMPORARY: AtomicU64 = AtomicU64::new(0);

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|error| format!("failed to create cache directory: {error}"))?;
    }
    let temporary = path.with_extension(format!(
        "{}-{}.{VERILOGA_CACHE_TEMPORARY_EXTENSION}",
        std::process::id(),
        NEXT_TEMPORARY.fetch_add(1, Relaxed)
    ));
    let written = std::fs::File::create(&temporary).and_then(|mut file| {
        for part in parts {
            file.write_all(part)?;
        }
        file.flush()
    });
    if let Err(error) = written {
        let _ = std::fs::remove_file(&temporary);
        return Err(format!("failed to write Verilog-A cache file: {error}"));
    }

    // Another program (a virus scanner, an indexer) may hold the destination
    // open without sharing deletion for a moment; that passes.
    let mut attempt = 0;
    loop {
        match std::fs::rename(&temporary, path) {
            Ok(()) => return Ok(()),
            Err(error)
                if attempt < VERILOGA_CACHE_RENAME_RETRIES
                    && error.kind() == std::io::ErrorKind::PermissionDenied =>
            {
                attempt += 1;
                std::thread::sleep(VERILOGA_CACHE_RENAME_BACKOFF);
            }
            Err(error) => {
                let _ = std::fs::remove_file(&temporary);
                return Err(format!(
                    "failed to publish Verilog-A cache file '{}': {error}",
                    path.display()
                ));
            }
        }
    }
}

#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
fn persist_model_to_disk_at_with_limits(
    source_path: &VerilogASourceKey,
    entry: &CachedVerilogAModel,
    cache_root: &Path,
    limits: ResourceLimits,
) -> Result<(), String> {
    let sealed = SealedRecord::new(&BorrowedVerilogACacheRecord::new(source_path, entry))?;
    ensure_cache_write_limit("record", sealed.payload_len(), limits)?;
    write_cache_file_atomically(
        &cache_record_path_with_root(source_path, cache_root),
        &sealed.file_parts(),
    )
}

/// Refuse a cache file larger than the disk cache or the shared budget.
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
fn ensure_cache_write_limit(
    kind: &str,
    payload_bytes: usize,
    limits: ResourceLimits,
) -> Result<(), String> {
    let (_, disk_max_bytes) = veriloga_cache_limits();
    let resource_max_bytes = u64::try_from(limits.max_shared_cache_bytes).unwrap_or(u64::MAX);
    let write_limit = disk_max_bytes.min(resource_max_bytes);
    let file_bytes = (RECORD_HEADER_BYTES + payload_bytes) as u64;
    if file_bytes > write_limit {
        return Err(format!(
            "Verilog-A cache {kind} of {file_bytes} bytes exceeds the {write_limit} byte write limit"
        ));
    }
    Ok(())
}

/// Where the native image filed under `key` lives. Like a record's, the name
/// carries the compiler build, which is what keeps one build's machine code
/// from another's.
#[cfg(all(feature = "veriloga-native", not(target_arch = "wasm32")))]
fn native_image_path_with_root(key: &[u8; 32], cache_root: &Path) -> PathBuf {
    let mut hasher = blake3::Hasher::new();
    hasher.update(veriloga_compiler_identity());
    hasher.update(b"\0native\0");
    hasher.update(key);
    let name = hasher.finalize().to_hex();
    cache_root.join(format!("{name}.{VERILOGA_CACHE_NATIVE_EXTENSION}"))
}

#[cfg(all(feature = "veriloga-native", not(target_arch = "wasm32")))]
fn persist_native_image_at(
    key: &[u8; 32],
    image: &rspice_veriloga::native::StoredNativeImage,
    cache_root: &Path,
    limits: ResourceLimits,
) -> Result<(), String> {
    let payload = veriloga_record::encode(image)
        .map_err(|error| format!("failed to encode native Verilog-A image: {error}"))?;
    ensure_cache_write_limit("native image", payload.len(), limits)?;
    write_cache_file_atomically(
        &native_image_path_with_root(key, cache_root),
        &[
            &sealing_header(NATIVE_IMAGE_MAGIC, payload.len(), &blake3::hash(&payload)),
            &payload,
        ],
    )
}

/// The native image filed under `key`, or `None`. A file that fails its seal
/// or does not decode is removed so the next compile can replace it.
#[cfg(all(feature = "veriloga-native", not(target_arch = "wasm32")))]
fn load_native_image_at(
    key: &[u8; 32],
    cache_root: &Path,
    limits: ResourceLimits,
) -> Option<rspice_veriloga::native::StoredNativeImage> {
    let path = native_image_path_with_root(key, cache_root);
    let bytes = match read_cache_file_with_limits_and_abort(&path, limits, &NoAbort) {
        Ok(bytes) => bytes,
        Err(VerilogACacheRecordReadError::Missing | VerilogACacheRecordReadError::Cancelled) => {
            return None;
        }
        Err(VerilogACacheRecordReadError::ResourceLimit(error)) => {
            log::debug!(
                "not reading native Verilog-A image '{}': {error}",
                path.display()
            );
            return None;
        }
        Err(VerilogACacheRecordReadError::Invalid(error)) => {
            log::warn!("{error}");
            return None;
        }
    };
    let decoded = sealed_payload(NATIVE_IMAGE_MAGIC, &bytes)
        .map_err(str::to_string)
        .and_then(|payload| {
            veriloga_record::decode::<rspice_veriloga::native::StoredNativeImage>(payload)
                .map_err(|error| error.to_string())
        });
    match decoded {
        Ok(image) => {
            touch_cache_file(&path);
            Some(image)
        }
        Err(reason) => {
            log::warn!(
                "discarding native Verilog-A image '{}': {reason}",
                path.display()
            );
            remove_cache_file(&path);
            None
        }
    }
}

/// The disk cache's store for native images. Records and images share one
/// directory, one budget and one least-recently-used eviction.
#[cfg(all(feature = "veriloga-native", not(target_arch = "wasm32")))]
struct DiskNativeImageStore;

#[cfg(all(feature = "veriloga-native", not(target_arch = "wasm32")))]
impl rspice_veriloga::native::NativeImageStore for DiskNativeImageStore {
    fn load(
        &self,
        key: &rspice_veriloga::native::NativeImageKey,
    ) -> Option<rspice_veriloga::native::StoredNativeImage> {
        load_native_image_at(
            key.as_bytes(),
            &veriloga_cache_root(),
            ResourceLimits::default(),
        )
    }

    fn store(
        &self,
        key: &rspice_veriloga::native::NativeImageKey,
        image: &rspice_veriloga::native::StoredNativeImage,
    ) {
        let cache_root = veriloga_cache_root();
        match persist_native_image_at(
            key.as_bytes(),
            image,
            &cache_root,
            ResourceLimits::default(),
        ) {
            Ok(()) => {
                if let Err(error) = prune_veriloga_cache(&cache_root) {
                    log::warn!("failed to prune Verilog-A cache after write: {error}");
                }
            }
            Err(error) => log::warn!("{error}"),
        }
    }
}

/// Let every native image this process compiles outlive it.
#[cfg(all(feature = "veriloga-native", not(target_arch = "wasm32")))]
fn install_disk_native_image_store() {
    static STORE: DiskNativeImageStore = DiskNativeImageStore;
    rspice_veriloga::native::install_native_image_store(&STORE);
}

#[cfg(all(test, feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) fn persist_model_to_disk_at(
    source_path: &Path,
    entry: &CachedVerilogAModel,
    cache_root: &Path,
) -> Result<(), String> {
    persist_model_to_disk_at_with_limits(
        &VerilogASourceKey::new(source_path, None),
        entry,
        cache_root,
        ResourceLimits::default(),
    )
}

#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) fn remove_cache_file(path: &Path) {
    if let Err(err) = std::fs::remove_file(path)
        && err.kind() != std::io::ErrorKind::NotFound
    {
        log::warn!(
            "failed to remove stale/corrupt Verilog-A cache file '{}': {}",
            path.display(),
            err
        );
    }
}

/// Mark a record as just used, so eviction takes the least recently used.
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
fn touch_cache_file(path: &Path) {
    let touched = std::fs::File::options()
        .write(true)
        .open(path)
        .and_then(|file| file.set_modified(std::time::SystemTime::now()));
    if let Err(error) = touched {
        log::debug!(
            "failed to refresh the use time of Verilog-A cache record '{}': {error}",
            path.display()
        );
    }
}

/// Keep the cache within its entry and byte budgets, evicting the least
/// recently used records first.
///
/// A hit refreshes its record's modification time, so the oldest records are
/// the ones no run has read for longest. A temporary file is someone's write
/// in progress until it has been abandoned long enough to be a crashed
/// writer's. Several processes may prune at once: each removes the same
/// oldest files, and one already gone is not an error.
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) fn prune_veriloga_cache(cache_root: &Path) -> Result<(), String> {
    let (max_entries, max_bytes) = veriloga_cache_limits();
    let now_ns = metadata_now_ns();
    let (temporaries, mut files): (Vec<_>, Vec<_>) = list_cache_files(cache_root)?
        .into_iter()
        .partition(|file| file.temporary);
    for temporary in temporaries {
        let abandoned = temporary.modified_ns.is_some_and(|modified| {
            now_ns.saturating_sub(modified) > VERILOGA_CACHE_ABANDONED_TEMPORARY.as_nanos()
        });
        if abandoned {
            remove_cache_file(&temporary.path);
        }
    }
    files.sort_by(|a, b| {
        let left = a.modified_ns.unwrap_or(0);
        let right = b.modified_ns.unwrap_or(0);
        left.cmp(&right).then_with(|| a.path.cmp(&b.path))
    });

    let mut entry_count = files.len();
    let mut total_bytes = files
        .iter()
        .map(|file| file.size_bytes)
        .fold(0_u64, u64::saturating_add);

    for file in files {
        if entry_count <= max_entries && total_bytes <= max_bytes {
            break;
        }

        match std::fs::remove_file(&file.path) {
            Ok(()) => {
                entry_count = entry_count.saturating_sub(1);
                total_bytes = total_bytes.saturating_sub(file.size_bytes);
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                entry_count = entry_count.saturating_sub(1);
                total_bytes = total_bytes.saturating_sub(file.size_bytes);
            }
            Err(err) => {
                log::warn!(
                    "failed to evict Verilog-A cache record '{}': {}",
                    file.path.display(),
                    err
                );
            }
        }
    }

    Ok(())
}

/// Now, on the clock file modification times are read against.
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
fn metadata_now_ns() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_nanos())
}

#[cfg(feature = "veriloga")]
fn persist_model_to_disk_with_limits(
    source_path: &VerilogASourceKey,
    entry: &CachedVerilogAModel,
    limits: ResourceLimits,
) -> Result<(), String> {
    // No disk in the browser build: the in-memory cache is the cache.
    #[cfg(target_arch = "wasm32")]
    {
        let _ = source_path;
        let _ = entry.canonical_ir.as_ref();
        let _ = limits;
        Ok(())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let cache_root = veriloga_cache_root();
        persist_model_to_disk_at_with_limits(source_path, entry, &cache_root, limits)?;
        if let Err(err) = prune_veriloga_cache(&cache_root) {
            log::warn!("failed to prune Verilog-A cache after write: {}", err);
        }
        Ok(())
    }
}

#[cfg(feature = "veriloga")]
pub(super) fn persist_model_to_disk(
    source_path: &Path,
    entry: &CachedVerilogAModel,
) -> Result<(), String> {
    persist_model_to_disk_with_limits(
        &VerilogASourceKey::new(source_path, None),
        entry,
        ResourceLimits::default(),
    )
}

/// A record read back from disk, with the size of its encoding, which is
/// what the shared memory cache charges for it.
#[cfg(feature = "veriloga")]
#[derive(Debug)]
struct DiskCacheHit {
    entry: CachedVerilogAModel,
    encoded_bytes: usize,
}

#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
fn load_model_from_disk_at_with_limits(
    source_path: &VerilogASourceKey,
    cache_root: &Path,
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<Option<DiskCacheHit>, SimulationError> {
    if abort.is_aborted() {
        return Err(SimulationError::Aborted);
    }
    let cache_path = cache_record_path_with_root(source_path, cache_root);
    let (record, encoded_bytes) =
        match read_cache_record_with_limits_and_abort(&cache_path, limits, abort) {
            Ok(read) => read,
            Err(VerilogACacheRecordReadError::Missing) => return Ok(None),
            Err(VerilogACacheRecordReadError::Invalid(error)) => {
                log::warn!("{}", error);
                remove_cache_file(&cache_path);
                return Ok(None);
            }
            Err(VerilogACacheRecordReadError::ResourceLimit(error)) => {
                log::debug!(
                    "skipping Verilog-A cache record '{}' under the active resource policy: {}",
                    cache_path.display(),
                    error
                );
                return Ok(None);
            }
            Err(VerilogACacheRecordReadError::Cancelled) => {
                return Err(SimulationError::Aborted);
            }
        };

    let requested_source = &source_path.source_path;
    let record_source = canonicalize_for_cache(&record.source_path);
    if *requested_source != record_source
        || source_path.selected_module != record.selected_module
        || source_path.configuration != record.configuration
        || source_path.unit != record.unit
        || source_path.wreal_resolution != record.wreal_resolution
        || source_path.external_cells != record.external_cells
        || source_path.delay_select != record.delay_select
        || source_path.defines != record.defines
        || record.configuration.is_some_and(|identity| {
            record.canonical_ir.as_ref().and_then(|artifact| {
                artifact.connections.configuration().map(|configuration| {
                    configuration_identity(artifact.runtime_source_identity(), configuration)
                })
            }) != Some(identity)
        })
        || source_path
            .selected_module
            .as_ref()
            .is_some_and(|module| record.model.name.as_str() != module)
    {
        remove_cache_file(&cache_path);
        return Ok(None);
    }

    if abort.is_aborted() {
        return Err(SimulationError::Aborted);
    }
    if !dependencies_are_fresh_with_limits_and_abort(&record.dependencies, limits, abort)? {
        remove_cache_file(&cache_path);
        return Ok(None);
    }

    if let Err(error) = validate_cached_artifact_pair(&record.model, record.canonical_ir.as_ref()) {
        log::warn!(
            "discarding invalid Verilog-A cache record '{}': {}",
            cache_path.display(),
            error
        );
        remove_cache_file(&cache_path);
        return Ok(None);
    }
    touch_cache_file(&cache_path);

    Ok(Some(DiskCacheHit {
        entry: CachedVerilogAModel {
            dependencies: record.dependencies,
            model: std::sync::Arc::new(record.model),
            canonical_ir: record.canonical_ir.map(std::sync::Arc::new),
            diagnostics: record.diagnostics.into(),
        },
        encoded_bytes,
    }))
}

#[cfg(feature = "veriloga")]
fn load_model_from_disk_with_limits_and_abort(
    source_path: &VerilogASourceKey,
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<Option<DiskCacheHit>, SimulationError> {
    // No disk in the browser build: only the in-memory cache can hit.
    #[cfg(target_arch = "wasm32")]
    {
        let _ = source_path;
        let _ = limits;
        let _ = abort;
        Ok(None)
    }
    #[cfg(not(target_arch = "wasm32"))]
    load_model_from_disk_at_with_limits(source_path, &veriloga_cache_root(), limits, abort)
}

#[cfg(all(test, feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) fn load_model_from_disk_at(
    source_path: &Path,
    cache_root: &Path,
) -> Result<Option<CachedVerilogAModel>, String> {
    load_model_from_disk_at_with_limits(
        &VerilogASourceKey::new(source_path, None),
        cache_root,
        ResourceLimits::default(),
        &NoAbort,
    )
    .map(|hit| hit.map(|hit| hit.entry))
    .map_err(|error| error.to_string())
}

#[cfg(all(test, feature = "veriloga"))]
pub(super) fn resolve_cached_or_compile_veriloga(
    path: &Path,
) -> Result<CachedVerilogAModel, SimulationError> {
    resolve_cached_or_compile_veriloga_with_limits(path, ResourceLimits::default())
}

/// How a `.VERILOGA` include named by a deck is compiled.
///
/// The one departure from the compiler's own defaults is `enable_ams`, and it
/// is what gives a mixed module a deck route at all. With it off, code
/// generation refuses the first digital declaration it meets, so a `.va` whose
/// module carries both an analog block and a process could not be read by a
/// deck; with it on, the front end emits the analog half as bytecode and hands
/// the discrete half to the canonical plan, which is what
/// [`MixedSignalHost`](crate::xspice::verilog::MixedSignalHost) executes.
///
/// Enabling it is only sound because the *builder* decides which half of the
/// artifact each module reaches: a module whose canonical plan is empty is
/// built as an ordinary [`VerilogADevice`](crate::device::veriloga::VerilogADevice)
/// exactly as before, and a module whose plan is not empty is built as a mixed
/// host that executes it. What `enable_ams` must never do is let an analog-only
/// device route silently drop a process, and here it cannot: the same predicate
/// that lets the digital half through the compiler selects the route that runs
/// it.
///
/// For a module with no digital content the option changes nothing at all. Its
/// entire effect inside the compiler is to skip `reject_digital_content`, which
/// on an analog-only module is a no-op — so the `CompiledModel` and the
/// canonical artifact a deck's analog `.va` produces are the ones it produced
/// before.
///
/// `deck` is what the deck gives every compile beyond its sources, which keys
/// the cache record too.
#[cfg(feature = "veriloga")]
pub(super) fn deck_include_compiler_options(deck: &DeckHdl) -> rspice_veriloga::CompilerOptions {
    rspice_veriloga::CompilerOptions {
        enable_ams: true,
        wreal_resolution: deck.wreal_resolution,
        external_cells: deck.external_cells.as_deref().cloned(),
        delay_select: deck.delay_select,
        defines: deck.defines.clone(),
        ..rspice_veriloga::CompilerOptions::default()
    }
}

/// What a deck gives every HDL compile beyond its sources: the resolution of
/// a plain `wreal` (`.options wreal_resolution`), the SPICE cells a design
/// may instantiate as leaves (its `.subckt`s and model cards), and the value
/// each min:typ:max expression keeps (`.options hdl_delay_select` or
/// `--hdl-delay-select`).
#[cfg(feature = "veriloga")]
#[derive(Debug, Clone, Default)]
pub(super) struct DeckHdl {
    pub(super) wreal_resolution: rspice_veriloga::canonical_ir::digital::DigitalRealResolution,
    /// `None` compiles no leaf, as a host that builds none does.
    pub(super) external_cells: Option<std::sync::Arc<Vec<rspice_veriloga::ExternalCell>>>,
    pub(super) delay_select: rspice_veriloga::canonical_ir::digital::DigitalDelaySelect,
    /// The macros one line's `define=` options predefine for its source alone.
    pub(super) defines: Vec<(String, Option<String>)>,
}

#[cfg(feature = "veriloga")]
impl DeckHdl {
    /// The deck defaults but for `wreal_resolution`: no leaves, and each
    /// min:typ:max expression keeps its typical value.
    pub(super) fn resolving_wreal(
        wreal_resolution: rspice_veriloga::canonical_ir::digital::DigitalRealResolution,
    ) -> Self {
        Self {
            wreal_resolution,
            external_cells: None,
            delay_select: Default::default(),
            defines: Vec::new(),
        }
    }

    /// This deck compiling one source under the macros its line defines.
    pub(super) fn with_source_defines(&self, defines: &[(String, Option<String>)]) -> Self {
        Self {
            defines: defines.to_vec(),
            ..self.clone()
        }
    }

    /// The identity of the macros a line predefines for its source, in the
    /// order written: part of the source's cache key in memory and on disk,
    /// so a changed define recompiles and an unchanged one hits.
    fn defines_identity(&self) -> Option<[u8; 32]> {
        fn text(hash: &mut blake3::Hasher, text: &str) {
            hash.update(&(text.len() as u64).to_le_bytes());
            hash.update(text.as_bytes());
        }
        if self.defines.is_empty() {
            return None;
        }
        let mut hash = blake3::Hasher::new();
        hash.update(b"rspice.hdl-defines\0");
        for (name, value) in &self.defines {
            text(&mut hash, name);
            match value {
                Some(value) => {
                    hash.update(&[1]);
                    text(&mut hash, value);
                }
                None => {
                    hash.update(&[0]);
                }
            }
        }
        Some(*hash.finalize().as_bytes())
    }

    /// The identity of the cell table a record's key names: `None` for no
    /// cells at all, so a deck without any reads the records it always did.
    fn cells_identity(&self) -> Option<[u8; 32]> {
        let cells = self
            .external_cells
            .as_deref()
            .filter(|cells| !cells.is_empty())?;
        let mut hasher = blake3::Hasher::new();
        let mut field = |text: &str| {
            hasher.update(&(text.len() as u64).to_le_bytes());
            hasher.update(text.as_bytes());
        };
        for cell in cells {
            field(&cell.name);
            for names in [&cell.ports, &cell.parameters] {
                field(&names.len().to_string());
                names.iter().for_each(|name| field(name));
            }
        }
        Some(*hasher.finalize().as_bytes())
    }
}

#[cfg(feature = "veriloga")]
pub(super) struct VerilogACompileControl<'a> {
    pub(super) abort: &'a dyn AbortSignal,
}

#[cfg(feature = "veriloga")]
impl rspice_veriloga::PipelineControl for VerilogACompileControl<'_> {
    fn is_cancelled(&self) -> bool {
        self.abort.is_aborted()
    }

    fn phase_completed(
        &self,
        _timing: rspice_veriloga::PhaseTiming,
        metrics: &rspice_veriloga::PipelineMetrics,
    ) {
        const EXPECTED_RUNTIME_PHASES: f64 = 9.0;
        self.abort.observe_progress(
            (metrics.phases.len() as f64 / EXPECTED_RUNTIME_PHASES).clamp(0.0, 1.0),
        );
    }
}

#[cfg(all(test, feature = "veriloga"))]
pub(super) fn resolve_cached_or_compile_veriloga_with_limits(
    path: &Path,
    limits: ResourceLimits,
) -> Result<CachedVerilogAModel, SimulationError> {
    resolve_cached_or_compile_veriloga_with_limits_and_abort(path, None, limits, &NoAbort)
}

#[cfg(feature = "veriloga")]
pub(super) fn lookup_cached_veriloga_with_limits_and_abort(
    path: &Path,
    selected_module: Option<&str>,
    deck: &DeckHdl,
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<Option<CachedVerilogAModel>, SimulationError> {
    lookup_veriloga_key_with_limits_and_abort(
        &VerilogASourceKey::new(path, selected_module).under(deck),
        limits,
        abort,
    )
}

#[cfg(feature = "veriloga")]
fn lookup_veriloga_key_with_limits_and_abort(
    canonical: &VerilogASourceKey,
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<Option<CachedVerilogAModel>, SimulationError> {
    use std::sync::atomic::Ordering::Relaxed;
    let path = canonical.source_path.as_path();
    let selected_module = canonical.selected_module.as_deref();
    VERILOGA_CACHE_TELEMETRY.lookups.fetch_add(1, Relaxed);
    #[cfg(all(feature = "veriloga-native", not(target_arch = "wasm32")))]
    install_disk_native_image_store();
    check_build_abort(abort)?;
    if selected_module.is_some() && is_sealed_veriloga_virtual_path(path) {
        return Err(cache_refusal(
            path,
            crate::ElaborationErrorKind::ModuleNotSelected,
            "this sealed Verilog-A runtime already fixes its module; a module override is not permitted",
        ));
    }
    let memory_entry = if let Ok(mut cache) = veriloga_source_cache().write() {
        cache.enforce_limit(limits.max_shared_cache_bytes);
        cache.get_cloned(canonical)
    } else {
        None
    };

    if let Some(CachedVerilogASource::Connections(_)) = &memory_entry {
        return Err(cache_refusal(
            path,
            crate::ElaborationErrorKind::ModuleNotSelected,
            "this source is a connection library, not a device module",
        ));
    }
    if let Some(CachedVerilogASource::Runtime(entry)) = memory_entry {
        if dependencies_are_fresh_with_limits_and_abort(&entry.dependencies, limits, abort)? {
            VERILOGA_CACHE_TELEMETRY.memory_hits.fetch_add(1, Relaxed);
            log::debug!("Verilog-A cache hit (memory): '{}'", canonical.display());
            return Ok(Some(entry));
        }
        VERILOGA_CACHE_TELEMETRY
            .stale_memory_entries
            .fetch_add(1, Relaxed);
        if let Ok(mut cache) = veriloga_source_cache().write()
            && cache
                .get(canonical)
                .and_then(CachedVerilogASource::runtime)
                .is_some_and(|current| current.dependencies == entry.dependencies)
        {
            cache.remove(canonical);
        }
    }
    if let Some(entry) = registered_at_default_resolution(canonical, limits, abort)? {
        VERILOGA_CACHE_TELEMETRY.memory_hits.fetch_add(1, Relaxed);
        log::debug!(
            "Verilog-A cache hit (registered at the default wreal resolution): '{}'",
            canonical.display()
        );
        return Ok(Some(entry));
    }

    // A sealed virtual key is an authenticated in-memory capability,
    // never a filesystem locator. A missing registration must fail closed;
    // disk cache and ambient files are not eligible fallbacks.
    if is_sealed_veriloga_virtual_path(path) {
        VERILOGA_CACHE_TELEMETRY.misses.fetch_add(1, Relaxed);
        if canonical.configuration.is_some() {
            return Ok(None);
        }
        return Err(cache_refusal(
            path,
            crate::ElaborationErrorKind::MissingSource,
            "this sealed Verilog-A runtime is not installed for this execution",
        ));
    }

    check_build_abort(abort)?;
    if let Some(DiskCacheHit {
        entry,
        encoded_bytes,
    }) = load_model_from_disk_with_limits_and_abort(canonical, limits, abort)?
    {
        VERILOGA_CACHE_TELEMETRY.disk_hits.fetch_add(1, Relaxed);
        if let Err(error) = retain_veriloga_model_sized(
            canonical.clone(),
            entry.clone(),
            veriloga_retained_entry_bytes(canonical, encoded_bytes),
            limits.max_shared_cache_bytes,
            false,
        ) {
            log::warn!(
                "failed to retain Verilog-A disk cache hit for '{}': {}",
                canonical.display(),
                error
            );
        }
        log::debug!("Verilog-A cache hit (disk): '{}'", canonical.display());
        return Ok(Some(entry));
    }

    VERILOGA_CACHE_TELEMETRY.misses.fetch_add(1, Relaxed);
    Ok(None)
}

/// A registration made at the default `wreal` resolution and without a cell
/// table, for a deck that sets another resolution or offers its cells.
///
/// A sealed source is installed once, under its key alone, for every deck
/// that later runs it, and a browser compiles nothing of its own, so neither
/// has a record at the deck's resolution to find. The option rewrites exactly
/// the plain `wreal` declarations, so a runtime with no plain `wreal` net is
/// the one the deck's resolution would compile, and it runs. One that has a
/// plain `wreal` net would run with a resolution the deck did not ask for,
/// so it is refused, naming the net and both resolutions.
///
/// A registration names the min:typ:max selection it was compiled under,
/// which keys it as the deck's selection keys the deck's own compile. One
/// compiled under another selection than the deck's kept other values of the
/// expressions the parser chose between, and which expressions those were is
/// not recorded, so it is refused, naming both selections, rather than run.
#[cfg(feature = "veriloga")]
fn registered_at_default_resolution(
    canonical: &VerilogASourceKey,
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<Option<CachedVerilogAModel>, SimulationError> {
    use rspice_veriloga::canonical_ir::digital::{
        DigitalDelaySelect, DigitalRealResolution, DigitalSignalKind,
    };
    let path = canonical.source_path.as_path();
    let sealed = is_sealed_veriloga_virtual_path(path);
    if !(sealed || cfg!(target_arch = "wasm32")) {
        return Ok(None);
    }
    // One compiled against the deck's own cells, then one with no leaf,
    // which the deck's cells build as any other deck's do: under the deck's
    // selection first, then under each other one, to be refused. A runtime
    // registered without the deck's cell table compiled no leaf: it is the
    // one the table would compile, or it never compiled at all.
    let selections = std::iter::once(canonical.delay_select).chain(
        [
            DigitalDelaySelect::Min,
            DigitalDelaySelect::Typ,
            DigitalDelaySelect::Max,
        ]
        .into_iter()
        .filter(|selection| *selection != canonical.delay_select),
    );
    let entry = veriloga_source_cache().read().ok().and_then(|cache| {
        selections
            .flat_map(|delay_select| {
                let registered = canonical.clone().under(&DeckHdl {
                    delay_select,
                    ..DeckHdl::default()
                });
                let mut against_deck_cells = registered.clone();
                against_deck_cells.external_cells = canonical.external_cells;
                [against_deck_cells, registered]
            })
            .filter(|key| key != canonical)
            .find_map(|key| Some((key.delay_select, cache.get_cloned(&key)?)))
    });
    let Some((compiled_under, CachedVerilogASource::Runtime(entry))) = entry else {
        return Ok(None);
    };
    if !dependencies_are_fresh_with_limits_and_abort(&entry.dependencies, limits, abort)? {
        return Ok(None);
    }
    if compiled_under != canonical.delay_select {
        let option = canonical.delay_select.keyword();
        return Err(cache_refusal(
            path,
            crate::ElaborationErrorKind::MissingSource,
            format!(
                "this {} Verilog-A runtime was compiled under `hdl_delay_select={}`, and the \
                 deck's `hdl_delay_select={option}` keeps other min:typ:max values; register \
                 it compiled under `{option}`",
                if sealed { "sealed" } else { "registered" },
                compiled_under.keyword(),
            ),
        ));
    }
    let plain = entry.canonical_ir.as_deref().and_then(|artifact| {
        artifact.digital.signals.iter().find(|signal| {
            signal.kind == DigitalSignalKind::Real(DigitalRealResolution::Single)
                && !signal.procedurally_assignable
                && signal.local.is_none()
                // A real nettype net (IEEE 1800-2017 section 6.6.7) keeps its
                // one driver under every `wreal_resolution`.
                && !signal.nettype
        })
    });
    let Some(net) = plain.filter(|_| canonical.wreal_resolution != DigitalRealResolution::Single)
    else {
        return Ok(Some(entry));
    };
    let option = match canonical.wreal_resolution {
        DigitalRealResolution::Single => "single",
        DigitalRealResolution::Sum => "sum",
        DigitalRealResolution::Average => "avg",
        DigitalRealResolution::Minimum => "min",
        DigitalRealResolution::Maximum => "max",
    };
    Err(cache_refusal(
        path,
        crate::ElaborationErrorKind::MissingSource,
        format!(
            "this {} Verilog-A runtime was compiled under `wreal_resolution=single`, and the \
             deck's `.options wreal_resolution={option}` changes its plain `wreal` net '{}'; \
             register it compiled under `{option}`, or declare that net `{}`",
            if sealed { "sealed" } else { "registered" },
            net.name,
            canonical.wreal_resolution.keyword(),
        ),
    ))
}

#[cfg(feature = "veriloga")]
pub(super) fn lookup_registered_connection_library(
    path: &Path,
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<Option<std::sync::Arc<rspice_veriloga::ConnectionLibraryArtifact>>, SimulationError> {
    check_build_abort(abort)?;
    if !is_sealed_veriloga_virtual_path(path) {
        return Ok(None);
    }
    let mut cache = veriloga_source_cache().write().map_err(|_| {
        cache_refusal(
            path,
            crate::ElaborationErrorKind::Internal,
            "failed to acquire the Verilog-A source cache lock",
        )
    })?;
    cache.enforce_limit(limits.max_shared_cache_bytes);
    match cache.get(&VerilogASourceKey::new(path, None)) {
        Some(CachedVerilogASource::Connections(library)) => {
            ResourceLimitError::ensure(
                ResourceKind::ExpandedSourceBytes,
                library.preprocessed_source().len(),
                limits.max_expanded_source_bytes,
            )?;
            Ok(Some(library.clone()))
        }
        _ => Ok(None),
    }
}

#[cfg(all(test, feature = "veriloga"))]
fn resolve_cached_or_compile_veriloga_with_limits_and_abort(
    path: &Path,
    selected_module: Option<&str>,
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<CachedVerilogAModel, SimulationError> {
    if let Some(entry) = lookup_cached_veriloga_with_limits_and_abort(
        path,
        selected_module,
        &DeckHdl::default(),
        limits,
        abort,
    )? {
        return Ok(entry);
    }
    let prepared = prepare_veriloga_source(path, limits, abort)?;
    compile_and_cache_prepared_veriloga(path, selected_module, &prepared, limits, abort)
}

/// Prepare one source with the deck defaults: a plain `wreal` is the LRM's.
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) fn prepare_veriloga_source(
    path: &Path,
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<rspice_veriloga::PreparedRuntimeSource, SimulationError> {
    prepare_veriloga_unit(&[path], None, &DeckHdl::default(), limits, abort)
}

/// Prepare HDL sources as one compilation unit, read in the order given
/// (IEEE 1364-2005 clause 19), or in the libraries a library map put them
/// in (clause 13). One path without a map prepares that file alone. The
/// files a map names are read under the same limits and refusals as the
/// deck's own. `deck` is what the deck gives the compile beyond its sources.
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) fn prepare_veriloga_unit(
    paths: &[&Path],
    libraries: Option<&rspice_veriloga::UnitLibraries>,
    deck: &DeckHdl,
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<rspice_veriloga::PreparedRuntimeSource, SimulationError> {
    check_build_abort(abort)?;
    let path = paths[0];
    let mut source_bytes = 0_usize;
    for &path in paths {
        if is_sealed_veriloga_virtual_path(path) {
            return Err(cache_refusal(
                path,
                crate::ElaborationErrorKind::MissingSource,
                "this sealed Verilog-A runtime requires its registered canonical source context; filesystem preparation is not permitted",
            ));
        }
        let source_metadata = std::fs::metadata(path).map_err(|error| {
            cache_refusal(
                path,
                crate::ElaborationErrorKind::MissingSource,
                format!("this source does not exist or is unreadable: {error}"),
            )
        })?;
        source_bytes = source_bytes
            .saturating_add(usize::try_from(source_metadata.len()).unwrap_or(usize::MAX));
        ResourceLimitError::ensure(
            ResourceKind::DependencySourceBytes,
            source_bytes,
            limits.max_dependency_source_bytes,
        )?;
    }

    let compiler = rspice_veriloga::VerilogACompiler::new(deck_include_compiler_options(deck));
    let control = VerilogACompileControl { abort };
    let preparation_started = crate::time_compat::Instant::now();
    let provider_limits = rspice_veriloga::SourceProviderLimits {
        max_dependencies: usize::MAX,
        max_total_source_bytes: limits.max_dependency_source_bytes,
        max_include_depth: limits.max_include_depth,
        max_expanded_bytes: limits.max_expanded_source_bytes,
    };
    let prepared = match libraries {
        Some(libraries) => compiler.prepare_library_unit_runtime_source_with_diagnostics(
            paths,
            libraries,
            provider_limits,
            &control,
        ),
        None => {
            compiler.prepare_unit_runtime_source_with_diagnostics(paths, provider_limits, &control)
        }
    };
    record_veriloga_compilation_time(preparation_started.elapsed());
    prepared.map_err(|error| {
        use std::sync::atomic::Ordering::Relaxed;
        // A failed preparation is a failed compilation attempt. Successful
        // preparation is shared; each later module emission counts once.
        VERILOGA_CACHE_TELEMETRY
            .compilations_started
            .fetch_add(1, Relaxed);
        match error {
            rspice_veriloga::ProviderCompileError::Compile {
                source: rspice_veriloga::CompileError::Cancelled(_),
                ..
            } => {
                VERILOGA_CACHE_TELEMETRY
                    .compilations_cancelled
                    .fetch_add(1, Relaxed);
                SimulationError::Aborted
            }
            rspice_veriloga::ProviderCompileError::Source(error) => {
                if error.cancelled {
                    VERILOGA_CACHE_TELEMETRY
                        .compilations_cancelled
                        .fetch_add(1, Relaxed);
                } else {
                    VERILOGA_CACHE_TELEMETRY
                        .compilations_failed
                        .fetch_add(1, Relaxed);
                }
                source_admission_error(path, "preparation failed", error)
            }
            // A unit of several files refuses a module name two files of one
            // library declare, at the second declaration.
            rspice_veriloga::ProviderCompileError::Compile {
                source: rspice_veriloga::CompileError::ModuleSelection(message),
                diagnostics,
            } if paths.len() > 1 || libraries.is_some() => {
                VERILOGA_CACHE_TELEMETRY
                    .compilations_failed
                    .fetch_add(1, Relaxed);
                let second = diagnostics
                    .first()
                    .and_then(|diagnostic| diagnostic.path.as_deref())
                    .map_or_else(|| path.to_path_buf(), PathBuf::from);
                crate::ElaborationError::new(
                    crate::ElaborationErrorKind::ModuleNotSelected,
                    message,
                )
                .in_source(second)
                .with_compiler_diagnostics(diagnostics)
                .into()
            }
            rspice_veriloga::ProviderCompileError::Compile {
                source,
                diagnostics,
            } => {
                VERILOGA_CACHE_TELEMETRY
                    .compilations_failed
                    .fetch_add(1, Relaxed);
                crate::ElaborationError::new(
                    crate::ElaborationErrorKind::CompileRefusal,
                    if diagnostics.is_empty() {
                        format!("preparation failed: {source}")
                    } else {
                        "preparation failed".to_owned()
                    },
                )
                .in_source(path)
                .with_compiler_diagnostics(diagnostics)
                .into()
            }
        }
    })
}

#[cfg(all(feature = "veriloga", target_arch = "wasm32"))]
pub(super) fn prepare_veriloga_source(
    path: &Path,
    _limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<rspice_veriloga::PreparedRuntimeSource, SimulationError> {
    check_build_abort(abort)?;
    Err(cache_refusal(
        path,
        crate::ElaborationErrorKind::MissingSource,
        "this source is not registered for browser execution; compile and register its virtual runtime before building the circuit",
    ))
}

#[cfg(all(feature = "veriloga", target_arch = "wasm32"))]
pub(super) fn prepare_veriloga_unit(
    paths: &[&Path],
    _libraries: Option<&rspice_veriloga::UnitLibraries>,
    _deck: &DeckHdl,
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<rspice_veriloga::PreparedRuntimeSource, SimulationError> {
    prepare_veriloga_source(paths[0], limits, abort)
}

#[cfg(feature = "veriloga")]
fn record_veriloga_compilation_time(elapsed: std::time::Duration) {
    use std::sync::atomic::Ordering::Relaxed;
    let nanos = u64::try_from(elapsed.as_nanos()).unwrap_or(u64::MAX);
    let _ = VERILOGA_CACHE_TELEMETRY
        .total_compilation_nanos
        .fetch_update(Relaxed, Relaxed, |total| Some(total.saturating_add(nanos)));
}

#[cfg(all(test, feature = "veriloga"))]
pub(super) fn compile_and_cache_prepared_veriloga(
    path: &Path,
    selected_module: Option<&str>,
    prepared: &rspice_veriloga::PreparedRuntimeSource,
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<CachedVerilogAModel, SimulationError> {
    compile_and_cache_prepared_with_connections(
        path,
        selected_module,
        prepared,
        None,
        None,
        &[],
        &DeckHdl::default(),
        limits,
        abort,
    )
}

#[cfg(feature = "veriloga")]
pub(super) fn configuration_identity(
    source: [u8; 32],
    configuration: &rspice_veriloga::ConnectionConfiguration,
) -> [u8; 32] {
    let mut hash = blake3::Hasher::new();
    hash.update(b"rspice.configured-runtime-cache\0");
    hash.update(&source);
    hash.update(&configuration.identity());
    *hash.finalize().as_bytes()
}

#[cfg(feature = "veriloga")]
pub(super) fn lookup_configured_veriloga(
    path: &Path,
    selected_module: Option<&str>,
    identity: [u8; 32],
    unit: Option<[u8; 32]>,
    deck: &DeckHdl,
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<Option<CachedVerilogAModel>, SimulationError> {
    let mut key = VerilogASourceKey::new(path, selected_module).under(deck);
    key.configuration = Some(identity);
    key.unit = unit;
    lookup_veriloga_key_with_limits_and_abort(&key, limits, abort)
}

/// The identity of a compilation unit of several files: the files, in the
/// order they are read. Their contents are the record's dependencies, so the
/// identity names only which files and in what order.
#[cfg(feature = "veriloga")]
pub(super) fn unit_identity<'a>(
    paths: impl IntoIterator<Item = &'a Path>,
    libraries: Option<&rspice_veriloga::UnitLibraries>,
) -> [u8; 32] {
    fn text(hash: &mut blake3::Hasher, text: &str) {
        hash.update(&(text.len() as u64).to_le_bytes());
        hash.update(text.as_bytes());
    }
    let mut hash = blake3::Hasher::new();
    hash.update(b"rspice.hdl-unit\0");
    for path in paths {
        let path = canonicalize_for_cache(path);
        text(&mut hash, &path.to_string_lossy());
    }
    // A library map's placement, search order and configurations decide what
    // each module binds, so a unit is keyed by them as by its files.
    if let Some(libraries) = libraries {
        hash.update(b"libraries\0");
        for library in libraries.files.iter().chain(&libraries.declared) {
            text(&mut hash, library);
        }
        hash.update(&(libraries.declared.len() as u64).to_le_bytes());
        for config in &libraries.configs {
            text(&mut hash, &config.path.to_string_lossy());
            hash.update(&(config.line as u64).to_le_bytes());
            text(&mut hash, &config.text);
        }
    }
    *hash.finalize().as_bytes()
}

/// A module of a compilation unit of several files, as one include names it.
#[cfg(feature = "veriloga")]
pub(super) fn lookup_unit_veriloga(
    path: &Path,
    selected_module: Option<&str>,
    unit: [u8; 32],
    deck: &DeckHdl,
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<Option<CachedVerilogAModel>, SimulationError> {
    let mut key = VerilogASourceKey::new(path, selected_module).under(deck);
    key.unit = Some(unit);
    lookup_veriloga_key_with_limits_and_abort(&key, limits, abort)
}

/// A module compiled from a compilation unit of several files: the unit
/// keys its record, and `module` is what the include selects, its file's
/// only module when it names none.
#[cfg(feature = "veriloga")]
#[derive(Debug, Clone, Copy)]
pub(super) struct UnitSelection<'a> {
    pub(super) identity: [u8; 32],
    pub(super) module: &'a str,
}

#[cfg(feature = "veriloga")]
#[allow(clippy::too_many_arguments)]
pub(super) fn compile_and_cache_prepared_with_connections(
    path: &Path,
    selected_module: Option<&str>,
    prepared: &rspice_veriloga::PreparedRuntimeSource,
    configuration: Option<&rspice_veriloga::ConnectionConfiguration>,
    unit: Option<UnitSelection<'_>>,
    retained_dependencies: &[VerilogADependencyFingerprint],
    deck: &DeckHdl,
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<CachedVerilogAModel, SimulationError> {
    use std::sync::atomic::Ordering::Relaxed;
    check_build_abort(abort)?;
    // `prepared` compiles with the options it was prepared with; the key
    // names the same resolution and cells.
    let mut canonical = VerilogASourceKey::new(path, selected_module).under(deck);
    canonical.unit = unit.map(|unit| unit.identity);
    let selected_module = unit.map(|unit| unit.module).or(selected_module);
    if let Some(configuration) = configuration {
        let source = prepared
            .runtime_source_identity(selected_module)
            .map_err(|error| {
                cache_refusal(
                    path,
                    crate::ElaborationErrorKind::ModuleNotSelected,
                    error.to_string(),
                )
            })?;
        canonical.configuration = Some(configuration_identity(source, configuration));
    }
    let control = VerilogACompileControl { abort };
    log::info!("Verilog-A cache miss, compiling '{}'", canonical.display());
    VERILOGA_CACHE_TELEMETRY
        .compilations_started
        .fetch_add(1, Relaxed);
    let compile_started = crate::time_compat::Instant::now();
    let compiled = match configuration {
        Some(configuration) => {
            prepared.compile_runtime_with_connections(selected_module, configuration, &control)
        }
        None => prepared.compile_runtime_with_control(selected_module, &control),
    };
    record_veriloga_compilation_time(compile_started.elapsed());
    let compiled = match compiled {
        Ok(compiled) => {
            VERILOGA_CACHE_TELEMETRY
                .compilations_succeeded
                .fetch_add(1, Relaxed);
            compiled
        }
        Err(rspice_veriloga::CompileError::Cancelled(_)) => {
            VERILOGA_CACHE_TELEMETRY
                .compilations_cancelled
                .fetch_add(1, Relaxed);
            return Err(SimulationError::Aborted);
        }
        Err(error) => {
            VERILOGA_CACHE_TELEMETRY
                .compilations_failed
                .fetch_add(1, Relaxed);
            let mut diagnostics = match configuration {
                Some(configuration) => {
                    prepared.diagnostics_for_error_with_connections(configuration, &error)
                }
                None => prepared.diagnostics_for_error(&error),
            };
            // A deck names the module of a several-module source on the
            // directive that loads it.
            if selected_module.is_none() {
                for diagnostic in &mut diagnostics {
                    diagnostic.explain_module_selection(
                        "with module=<name> on the line that loads it",
                        path,
                    );
                }
            }
            let detail = if diagnostics.is_empty() {
                format!("compilation failed: {error}")
            } else {
                "compilation failed".into()
            };
            return Err(crate::ElaborationError::new(
                crate::ElaborationErrorKind::CompileRefusal,
                detail,
            )
            .in_source(path)
            .with_compiler_diagnostics(diagnostics)
            .into());
        }
    };

    check_build_abort(abort)?;
    validate_runtime_artifact_pair(&compiled.model, Some(&compiled.canonical_ir)).map_err(
        |error| {
            cache_refusal(
                path,
                crate::ElaborationErrorKind::CacheCorrupt,
                format!("compiled runtime artifacts failed integrity validation: {error}"),
            )
        },
    )?;
    let mut dependency_bytes = 0_usize;
    let mut dependencies = retained_dependencies.to_vec();
    for dependency in retained_dependencies {
        dependency_bytes = dependency_bytes
            .saturating_add(usize::try_from(dependency.file_len).unwrap_or(usize::MAX));
    }
    ResourceLimitError::ensure(
        ResourceKind::DependencySourceBytes,
        dependency_bytes,
        limits.max_dependency_source_bytes,
    )?;
    for dependency in &compiled.source_dependencies {
        check_build_abort(abort)?;
        dependency_bytes = dependency_bytes.saturating_add(dependency.byte_len);
        ResourceLimitError::ensure(
            ResourceKind::DependencySourceBytes,
            dependency_bytes,
            limits.max_dependency_source_bytes,
        )?;
        dependencies.push(VerilogADependencyFingerprint {
            canonical_path: dependency.path.clone(),
            modified_ns: None,
            file_len: u64::try_from(dependency.byte_len).unwrap_or(u64::MAX),
            content_hash: dependency.content_identity,
        });
    }
    let entry = CachedVerilogAModel {
        dependencies,
        model: std::sync::Arc::new(compiled.model),
        canonical_ir: Some(std::sync::Arc::new(compiled.canonical_ir)),
        diagnostics: compiled.diagnostics.into(),
    };

    check_build_abort(abort)?;
    if let Err(error) = retain_veriloga_model(
        canonical.clone(),
        entry.clone(),
        limits.max_shared_cache_bytes,
        false,
    ) {
        log::warn!(
            "failed to retain compiled Verilog-A runtime for '{}': {}",
            canonical.display(),
            error
        );
    }

    if !is_sealed_veriloga_virtual_path(path)
        && let Err(err) = persist_model_to_disk_with_limits(&canonical, &entry, limits)
    {
        VERILOGA_CACHE_TELEMETRY
            .persistence_failures
            .fetch_add(1, Relaxed);
        log::warn!(
            "Failed to persist Verilog-A cache entry for '{}': {}",
            canonical.display(),
            err
        );
    }

    Ok(entry)
}

#[cfg(feature = "veriloga")]
fn register_precompiled_veriloga_entry_with_dependencies(
    source_path: impl AsRef<Path>,
    dependencies: &[PathBuf],
    model: rspice_veriloga::CompiledModel,
    canonical_ir: Option<rspice_veriloga::canonical_ir::CanonicalIrArtifact>,
) -> Result<(), String> {
    #[cfg(feature = "veriloga-native")]
    if canonical_ir.is_none() {
        return Err(
            "native Verilog-A registration requires canonical IR; use register_precompiled_veriloga_runtime_with_dependencies"
                .to_string(),
        );
    }

    validate_runtime_artifact_pair(&model, canonical_ir.as_ref())?;

    let canonical_source = canonicalize_for_cache(source_path.as_ref());

    #[cfg(not(target_arch = "wasm32"))]
    let dependency_fingerprints = {
        let mut dependency_paths = dependencies.to_vec();
        if dependency_paths.is_empty() {
            dependency_paths.push(canonical_source.clone());
        }
        fingerprint_paths(&dependency_paths)
            .map_err(|e| format!("dependency fingerprinting failed: {}", e))?
    };
    // The browser build has no filesystem to fingerprint against; an empty
    // dependency set always reads as fresh, so the registered artifact is
    // simply trusted for the session.
    #[cfg(target_arch = "wasm32")]
    let dependency_fingerprints = {
        let _ = dependencies;
        Vec::new()
    };

    let entry = CachedVerilogAModel {
        dependencies: dependency_fingerprints,
        model: std::sync::Arc::new(model),
        canonical_ir: canonical_ir.map(std::sync::Arc::new),
        diagnostics: Vec::new().into(),
    };

    retain_veriloga_model(
        canonical_source.clone(),
        entry.clone(),
        ResourceLimits::default().max_shared_cache_bytes,
        true,
    )?;

    if let Err(err) = persist_model_to_disk(&canonical_source, &entry) {
        log::warn!(
            "Failed to persist precompiled Verilog-A cache for '{}': {}",
            canonical_source.display(),
            err
        );
    }

    Ok(())
}

/// Register a precompiled Verilog-A runtime artifact in the global engine cache.
///
/// Native builds require this paired model/canonical-IR artifact so the runtime
/// cannot silently fall back to bytecode-only construction.
#[cfg(feature = "veriloga")]
pub fn register_precompiled_veriloga_runtime_with_dependencies(
    source_path: impl AsRef<Path>,
    dependencies: &[PathBuf],
    model: rspice_veriloga::CompiledModel,
    canonical_ir: rspice_veriloga::canonical_ir::CanonicalIrArtifact,
) -> Result<(), String> {
    register_precompiled_veriloga_entry_with_dependencies(
        source_path,
        dependencies,
        model,
        Some(canonical_ir),
    )
}

/// One exact sealed Verilog-A runtime prepared for atomic registration.
///
/// `aliases` are the model identities that a generated deck may use for this
/// artifact. They are checked case-insensitively within each project or signed
/// package authority represented by one submitted runtime set; the compiled
/// module name is always included automatically.
#[cfg(feature = "veriloga")]
#[derive(Debug, Clone)]
pub struct ProjectVerilogARuntimeRegistration {
    /// Normalized project- or PDK-scoped virtual source identity.
    pub source_key: PathBuf,
    /// Additional netlist model aliases claimed by this artifact.
    pub aliases: Vec<String>,
    /// Validated executable bytecode model.
    pub model: rspice_veriloga::CompiledModel,
    /// Canonical IR paired with `model` for the native runtime.
    pub canonical_ir: rspice_veriloga::canonical_ir::CanonicalIrArtifact,
    /// The cell table the artifact was compiled against
    /// (`rspice_veriloga::CompilerOptions::external_cells`), when its design
    /// instantiates SPICE cells as leaves. The table is part of the
    /// artifact's key: only a deck that offers the same cells
    /// ([`crate::engine::hdl_external_cells`]) builds it. `None` for an
    /// artifact with no leaf, which every deck builds alike.
    pub external_cells: Option<Vec<rspice_veriloga::ExternalCell>>,
    /// The value of each min:typ:max expression the artifact was compiled
    /// keeping (`rspice_veriloga::CompilerOptions::delay_select`). It is part
    /// of the artifact's key: only a deck whose `.options hdl_delay_select`
    /// keeps the same values builds it, and a deck keeping others refuses it
    /// by naming both.
    pub delay_select: rspice_veriloga::canonical_ir::digital::DigitalDelaySelect,
}

/// Standalone connection declarations under an exact sealed source key.
#[cfg(feature = "veriloga")]
#[derive(Debug, Clone)]
pub struct ProjectVerilogAConnectionLibraryRegistration {
    pub source_key: PathBuf,
    pub aliases: Vec<String>,
    pub artifact: rspice_veriloga::ConnectionLibraryArtifact,
}

/// One source in an atomic project/package registration transaction.
#[cfg(feature = "veriloga")]
#[derive(Debug, Clone)]
#[allow(
    clippy::large_enum_variant,
    reason = "every variant already owns a whole compiled artifact, and this \
              one-shot install list is moved straight into the transaction, so \
              boxing would only add an allocation and take the `Runtime` \
              constructor away from the callers that pass it to `map`"
)]
pub enum ProjectVerilogASourceRegistration {
    Runtime(ProjectVerilogARuntimeRegistration),
    Connections(ProjectVerilogAConnectionLibraryRegistration),
}

#[cfg(feature = "veriloga")]
#[derive(Debug)]
struct PreparedProjectVerilogARegistration {
    key: PathBuf,
    folded_key: String,
    authority_scope: String,
    aliases: BTreeSet<String>,
    artifact_fingerprint: [u8; 32],
    /// The identity of the cell table the artifact was compiled against.
    cells: Option<[u8; 32]>,
    /// The min:typ:max selection the artifact was compiled under.
    delay_select: rspice_veriloga::canonical_ir::digital::DigitalDelaySelect,
    entry: CachedVerilogASource,
}

#[cfg(feature = "veriloga")]
impl PreparedProjectVerilogARegistration {
    /// The cache key the source is installed under: its sealed path, with
    /// the cells and the min:typ:max selection it was compiled under.
    fn cache_key(&self) -> VerilogASourceKey {
        let mut key = VerilogASourceKey::new(&self.key, None);
        key.external_cells = self.cells;
        key.delay_select = self.delay_select;
        key
    }
}

#[cfg(feature = "veriloga")]
#[derive(Default)]
struct ArtifactFingerprintWriter(blake3::Hasher);

#[cfg(feature = "veriloga")]
impl std::io::Write for ArtifactFingerprintWriter {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.0.update(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[cfg(feature = "veriloga")]
fn runtime_artifact_fingerprint(
    model: &rspice_veriloga::CompiledModel,
    canonical_ir: Option<&rspice_veriloga::canonical_ir::CanonicalIrArtifact>,
) -> Result<[u8; 32], String> {
    let mut writer = ArtifactFingerprintWriter::default();
    serde_json::to_writer(&mut writer, &(model, canonical_ir))
        .map_err(|error| format!("failed to fingerprint Verilog-A runtime artifact: {error}"))?;
    Ok(*writer.0.finalize().as_bytes())
}

#[cfg(feature = "veriloga")]
fn validate_sealed_runtime_source_key(
    source_key: &Path,
) -> Result<(PathBuf, String, String), String> {
    let key_text = source_key
        .to_str()
        .ok_or_else(|| "sealed Verilog-A runtime keys must contain valid UTF-8 text".to_owned())?;
    let components = key_text.split('/').collect::<Vec<_>>();
    let namespace = components.first().copied().unwrap_or_default();
    if key_text.contains('\\')
        || key_text.chars().any(char::is_control)
        || components.len() < 3
        || !matches!(
            namespace,
            "__rspice_project__" | "__rspice_pdk__" | "__rspice_model_library__"
        )
        || components
            .iter()
            .any(|component| component.is_empty() || matches!(*component, "." | ".."))
    {
        return Err(
            "sealed Verilog-A runtime keys must be normalized content-addressed virtual paths under __rspice_project__/, __rspice_pdk__/, or __rspice_model_library__/"
                .to_owned(),
        );
    }

    let normalized = canonicalize_for_cache(source_key);
    Ok((
        normalized,
        key_text.to_ascii_lowercase(),
        format!(
            "{}:{}",
            namespace.to_ascii_lowercase(),
            components[1].to_ascii_lowercase()
        ),
    ))
}

#[cfg(feature = "veriloga")]
fn validate_project_runtime_alias(alias: &str) -> Result<String, String> {
    if alias.is_empty()
        || alias.trim() != alias
        || alias
            .chars()
            .any(|character| character.is_control() || character.is_whitespace())
    {
        return Err(format!(
            "sealed Verilog-A runtime alias '{alias}' is not a valid model identity"
        ));
    }
    Ok(normalize_model_key(alias))
}

#[cfg(feature = "veriloga")]
fn prepare_project_veriloga_registration(
    registration: ProjectVerilogARuntimeRegistration,
) -> Result<PreparedProjectVerilogARegistration, String> {
    let (key, folded_key, authority_scope) =
        validate_sealed_runtime_source_key(&registration.source_key)?;
    validate_runtime_artifact_pair(&registration.model, Some(&registration.canonical_ir))?;
    let cells = DeckHdl {
        external_cells: registration.external_cells.map(std::sync::Arc::new),
        ..DeckHdl::default()
    }
    .cells_identity();
    // A leaf is built from the deck's cell of that name, so an artifact
    // that has one is good only for a deck that offers the cells it was
    // compiled against.
    if cells.is_none() && !registration.canonical_ir.hir.external_leaves.is_empty() {
        return Err(format!(
            "sealed Verilog-A runtime '{}' instantiates SPICE cells as leaves but names no cell table it was compiled against",
            key.display()
        ));
    }

    let mut aliases = BTreeSet::new();
    aliases.insert(validate_project_runtime_alias(
        registration.model.name.as_str(),
    )?);
    for alias in registration.aliases {
        aliases.insert(validate_project_runtime_alias(&alias)?);
    }
    let artifact_fingerprint =
        runtime_artifact_fingerprint(&registration.model, Some(&registration.canonical_ir))?;
    Ok(PreparedProjectVerilogARegistration {
        key,
        folded_key,
        authority_scope,
        aliases,
        artifact_fingerprint,
        cells,
        delay_select: registration.delay_select,
        entry: CachedVerilogASource::Runtime(CachedVerilogAModel {
            dependencies: Vec::new(),
            model: std::sync::Arc::new(registration.model),
            canonical_ir: Some(std::sync::Arc::new(registration.canonical_ir)),
            diagnostics: Vec::new().into(),
        }),
    })
}

/// Sealed Verilog-A sources an execution installed, held in the shared cache
/// until this is dropped.
///
/// The cache makes room for an install by evicting its oldest entries, and a
/// sealed source is never compiled again from anywhere: a build that misses
/// one fails. A held source is never evicted, so an execution that installs
/// and then builds, as often as its analysis rebuilds, finds every source it
/// installed for as long as it holds them. An install that could fit only by
/// evicting a held source is refused instead.
#[cfg(feature = "veriloga")]
#[derive(Debug, Default)]
#[must_use = "dropping the hold lets the installed sources be evicted"]
pub struct HeldVerilogASources {
    keys: Vec<VerilogASourceKey>,
}

#[cfg(feature = "veriloga")]
impl Drop for HeldVerilogASources {
    fn drop(&mut self) {
        if self.keys.is_empty() {
            return;
        }
        let mut cache = veriloga_source_cache()
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        for key in &self.keys {
            cache.release(key);
        }
    }
}

/// Atomically install sealed device and connection-library sources under
/// caller-selected source and cache limits, holding every installed source
/// until the returned value is dropped.
///
/// This is the only way to install a sealed source: one installed and not
/// held could be evicted by the next install before the execution that
/// installed it builds. Both kinds share key and alias collision checks and
/// the bounded in-memory cache: duplicate or case-colliding keys, and model
/// aliases within one project, signed package or model library, may only
/// identify identical artifacts. Validation, collision and budget failures
/// leave the installed set unchanged. File-backed cache persistence is
/// bypassed for these sealed artifacts.
#[cfg(feature = "veriloga")]
pub fn hold_project_veriloga_sources_for_session(
    registrations: impl IntoIterator<Item = ProjectVerilogASourceRegistration>,
    limits: ResourceLimits,
) -> Result<HeldVerilogASources, String> {
    install_project_veriloga_sources(registrations, limits).map(|keys| HeldVerilogASources { keys })
}

/// Install the set in one transaction under the cache lock, holding each
/// installed key before the lock is released, so no other install can evict
/// it in between. Returns the held keys.
#[cfg(feature = "veriloga")]
fn install_project_veriloga_sources(
    registrations: impl IntoIterator<Item = ProjectVerilogASourceRegistration>,
    limits: ResourceLimits,
) -> Result<Vec<VerilogASourceKey>, String> {
    let max_shared_cache_bytes = limits.max_shared_cache_bytes;
    let mut prepared_by_key = BTreeMap::<PathBuf, PreparedProjectVerilogARegistration>::new();
    for registration in registrations {
        let prepared = match registration {
            ProjectVerilogASourceRegistration::Runtime(runtime) => {
                prepare_project_veriloga_registration(runtime)?
            }
            ProjectVerilogASourceRegistration::Connections(library) => {
                let (key, folded_key, authority_scope) =
                    validate_sealed_runtime_source_key(&library.source_key)?;
                ResourceLimitError::ensure(
                    ResourceKind::ExpandedSourceBytes,
                    library.artifact.preprocessed_source().len(),
                    limits.max_expanded_source_bytes,
                )
                .map_err(|error| error.to_string())?;
                library.artifact.connect_specification()?;
                let aliases = library
                    .aliases
                    .iter()
                    .map(|alias| validate_project_runtime_alias(alias))
                    .collect::<Result<_, _>>()?;
                PreparedProjectVerilogARegistration {
                    key,
                    folded_key,
                    authority_scope,
                    aliases,
                    artifact_fingerprint: *library.artifact.identity(),
                    cells: None,
                    delay_select: Default::default(),
                    entry: CachedVerilogASource::Connections(std::sync::Arc::new(library.artifact)),
                }
            }
        };
        if let Some(existing) = prepared_by_key.get_mut(&prepared.key) {
            if existing.artifact_fingerprint != prepared.artifact_fingerprint
                || existing.cells != prepared.cells
                || existing.delay_select != prepared.delay_select
            {
                return Err(format!(
                    "sealed Verilog-A runtime key '{}' is claimed by differing artifacts",
                    prepared.key.display()
                ));
            }
            existing.aliases.extend(prepared.aliases);
        } else {
            prepared_by_key.insert(prepared.key.clone(), prepared);
        }
    }

    let prepared = prepared_by_key.into_values().collect::<Vec<_>>();
    if prepared.is_empty() {
        return Ok(Vec::new());
    }
    let mut folded_keys = BTreeMap::<&str, [u8; 32]>::new();
    let mut scoped_aliases = BTreeMap::<(&str, &str), [u8; 32]>::new();
    for runtime in &prepared {
        if let Some(existing) =
            folded_keys.insert(runtime.folded_key.as_str(), runtime.artifact_fingerprint)
            && existing != runtime.artifact_fingerprint
        {
            return Err(format!(
                "case-colliding sealed Verilog-A runtime key '{}' is claimed by differing artifacts",
                runtime.key.display()
            ));
        }
        for alias in &runtime.aliases {
            if let Some(existing) = scoped_aliases.insert(
                (runtime.authority_scope.as_str(), alias.as_str()),
                runtime.artifact_fingerprint,
            ) && existing != runtime.artifact_fingerprint
            {
                return Err(format!(
                    "sealed Verilog-A alias '{alias}' is claimed by differing artifacts in authority scope '{}'",
                    runtime.authority_scope
                ));
            }
        }
    }

    let incoming_bytes = prepared
        .iter()
        .map(|runtime| veriloga_source_cache_entry_bytes(&runtime.cache_key(), &runtime.entry))
        .collect::<Result<Vec<_>, _>>()?;
    let aggregate_incoming_bytes = incoming_bytes
        .iter()
        .copied()
        .fold(0usize, usize::saturating_add);
    ResourceLimitError::ensure(
        ResourceKind::SharedCacheBytes,
        aggregate_incoming_bytes,
        max_shared_cache_bytes,
    )
    .map_err(|error| error.to_string())?;

    let mut cache = veriloga_source_cache()
        .write()
        .map_err(|_| "failed to acquire Verilog-A cache lock".to_owned())?;

    // Compare the whole candidate set against the live registry while holding
    // the one cache lock. Nothing can change between validation and commit.
    // The same source compiled against another cell table, or under another
    // min:typ:max selection, is another key.
    for (cached_key, cached_entry) in cache.iter() {
        if !is_sealed_veriloga_virtual_path(&cached_key.source_path) {
            continue;
        }
        let (_, cached_folded_key, _) =
            validate_sealed_runtime_source_key(&cached_key.source_path)?;
        let cached_fingerprint = cached_entry.artifact_fingerprint()?;
        for runtime in &prepared {
            if cached_folded_key == runtime.folded_key
                && cached_key.external_cells == runtime.cells
                && cached_key.delay_select == runtime.delay_select
                && cached_fingerprint != runtime.artifact_fingerprint
            {
                return Err(format!(
                    "sealed Verilog-A runtime key '{}' collides with a differing installed artifact",
                    runtime.key.display()
                ));
            }
        }
    }

    let mut replacements = Vec::with_capacity(prepared.len());
    for runtime in prepared {
        let key = runtime.cache_key();
        let entry = if let Some(installed) = cache.get(&key) {
            let installed_fingerprint = installed.artifact_fingerprint()?;
            if installed_fingerprint != runtime.artifact_fingerprint {
                return Err(format!(
                    "sealed Verilog-A runtime key '{}' is already installed with a differing artifact",
                    runtime.key.display()
                ));
            }
            installed.clone()
        } else {
            runtime.entry
        };
        let retained_bytes = veriloga_source_cache_entry_bytes(&key, &entry)?;
        replacements.push((key, entry, retained_bytes));
    }

    let replacement_bytes = replacements
        .iter()
        .map(|(_, _, retained_bytes)| *retained_bytes)
        .fold(0usize, usize::saturating_add);
    ResourceLimitError::ensure(
        ResourceKind::SharedCacheBytes,
        replacement_bytes,
        max_shared_cache_bytes,
    )
    .map_err(|error| error.to_string())?;
    let keys = replacements
        .iter()
        .map(|(key, _, _)| key.clone())
        .collect::<Vec<VerilogASourceKey>>();
    cache.try_replace_batch(replacements, max_shared_cache_bytes)?;
    for key in &keys {
        cache.hold(key);
    }
    Ok(keys)
}

/// Register a precompiled Verilog-A model in the global engine cache.
///
/// This allows UI workflows to compile once on import and reuse the compiled
/// artifact during simulation without recompilation. Native JIT builds should
/// use [`register_precompiled_veriloga_runtime_with_dependencies`] so the cache
/// carries canonical IR as well as the compiled model.
#[cfg(feature = "veriloga")]
pub fn register_precompiled_veriloga_model_with_dependencies(
    source_path: impl AsRef<Path>,
    dependencies: &[PathBuf],
    model: rspice_veriloga::CompiledModel,
) -> Result<(), String> {
    register_precompiled_veriloga_entry_with_dependencies(source_path, dependencies, model, None)
}

/// Register a precompiled Verilog-A model in the global engine cache.
///
/// This compatibility wrapper fingerprints only the source file path.
#[cfg(feature = "veriloga")]
pub fn register_precompiled_veriloga_model(
    source_path: impl AsRef<Path>,
    model: rspice_veriloga::CompiledModel,
) -> Result<(), String> {
    let dependency = vec![canonicalize_for_cache(source_path.as_ref())];
    register_precompiled_veriloga_model_with_dependencies(source_path, &dependency, model)
}

#[cfg(all(test, feature = "veriloga", not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_test_root(name: &str) -> PathBuf {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time is after the Unix epoch")
            .as_nanos();
        let sequence = NEXT.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!(
            "rspice-veriloga-cache-{name}-{}-{nonce}-{sequence}",
            std::process::id()
        ))
    }

    fn compiled_entry(source_path: &Path) -> CachedVerilogAModel {
        compiled_entry_from(
            source_path,
            r#"
`include "disciplines.vams"
module cached_resistor(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 1000.0 from (0:inf);
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#,
        )
    }

    /// A model with assignment passes, which a record sets apart.
    fn compiled_assigning_entry(source_path: &Path) -> CachedVerilogAModel {
        compiled_entry_from(
            source_path,
            r#"
`include "disciplines.vams"
module cached_conductance(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 1000.0 from (0:inf);
    real current;
    analog begin
        current = V(p, n) / r;
        I(p, n) <+ current;
    end
endmodule
"#,
        )
    }

    fn compiled_entry_from(source_path: &Path, source: &str) -> CachedVerilogAModel {
        std::fs::write(source_path, source).expect("write temporary Verilog-A source");

        let runtime = rspice_veriloga::VerilogACompiler::default()
            .compile_file_runtime_with_metadata(source_path, None)
            .expect("compile runtime artifacts");
        let dependencies = fingerprint_paths(&runtime.dependencies).expect("fingerprint source");
        CachedVerilogAModel {
            dependencies,
            model: std::sync::Arc::new(runtime.model),
            canonical_ir: Some(std::sync::Arc::new(runtime.canonical_ir)),
            diagnostics: runtime.diagnostics.into(),
        }
    }

    /// Rewrite a record file after `edit`, sealed as record `version` would
    /// have sealed it.
    fn reseal_record(path: &Path, version: u32, edit: impl FnOnce(&mut VerilogADiskCacheRecord)) {
        let bytes = std::fs::read(path).expect("read cache record");
        let (mut record, _) = unseal_record(&bytes).expect("a valid record");
        edit(&mut record);
        let mut sealed = SealedRecord::new(&record).expect("encode record");
        sealed.header[8..12].copy_from_slice(&version.to_le_bytes());
        std::fs::write(path, sealed.file_parts().concat()).expect("write record");
    }

    #[test]
    fn runtime_preparation_preserves_source_admission_outcomes() {
        let root = unique_test_root("preparation-admission");
        std::fs::create_dir_all(&root).unwrap();
        let source = root.join("root.va");
        std::fs::write(&source, "`include \"child.va\"\n").unwrap();
        std::fs::write(
            root.join("child.va"),
            format!(
                "module m;\n{}endmodule\n",
                (0..2048)
                    .map(|index| format!("real r{index};\n"))
                    .collect::<String>()
            ),
        )
        .unwrap();
        for (limits, resource, limit) in [
            (
                ResourceLimits::default().with(|limits| limits.max_include_depth = 1),
                ResourceKind::IncludeDepth,
                1,
            ),
            (
                ResourceLimits::default().with(|limits| limits.max_expanded_source_bytes = 256),
                ResourceKind::ExpandedSourceBytes,
                256,
            ),
            (
                ResourceLimits::default().with(|limits| limits.max_dependency_source_bytes = 256),
                ResourceKind::DependencySourceBytes,
                256,
            ),
        ] {
            let error = prepare_veriloga_source(&source, limits, &NoAbort).unwrap_err();
            let SimulationError::ResourceLimit(error) = error else {
                panic!("{error:?}")
            };
            assert_eq!(error.resource, resource);
            assert_eq!(error.limit, limit);
            assert!(error.requested > limit);
        }
        struct CancelDuringRead(std::sync::atomic::AtomicUsize);
        impl AbortSignal for CancelDuringRead {
            fn is_aborted(&self) -> bool {
                self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst) >= 8
            }
        }
        let error = prepare_veriloga_source(
            &source,
            ResourceLimits::default(),
            &CancelDuringRead(std::sync::atomic::AtomicUsize::new(0)),
        )
        .unwrap_err();
        assert!(matches!(error, SimulationError::Aborted));
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn a_design_with_only_timing_checks_survives_the_disk_record() {
        let root = unique_test_root("digital-checks");
        std::fs::create_dir_all(&root).unwrap();
        let source = root.join("stamp.va");
        std::fs::write(
            &source,
            "`timescale 1ns/1ns
             module chk(input d, input c);
             reg ntf;
             specify $setup(d, posedge c, 3, ntf); endspecify
             endmodule
             module stamp(p, n);
             inout p, n;
             electrical p, n;
             reg d, c;
             chk u(d, c);
             analog I(p, n) <+ V(p, n) / 1000.0;
             endmodule
",
        )
        .unwrap();
        let report = rspice_veriloga::VerilogACompiler::new(rspice_veriloga::CompilerOptions {
            enable_ams: true,
            ..Default::default()
        })
        .compile_file_runtime_with_metadata(&source, Some("stamp"))
        .unwrap();
        let plan = report.canonical_ir.digital.clone();
        assert!(plan.delays.is_empty() && !plan.delays.checks.is_empty());
        let key = VerilogASourceKey::new(&source, Some("stamp"));
        let entry = CachedVerilogAModel {
            dependencies: fingerprint_paths(&report.dependencies).unwrap(),
            model: std::sync::Arc::new(report.model),
            canonical_ir: Some(std::sync::Arc::new(report.canonical_ir)),
            diagnostics: report.diagnostics.into(),
        };
        persist_model_to_disk_at_with_limits(&key, &entry, &root, ResourceLimits::default())
            .unwrap();
        let load = || {
            load_model_from_disk_at_with_limits(&key, &root, ResourceLimits::default(), &NoAbort)
                .unwrap()
        };
        let hit = load().expect("a whole record is a hit").entry;
        assert_eq!(hit.canonical_ir.unwrap().digital, plan);

        let cache_path = cache_record_path_with_root(&key, &root);
        reseal_record(&cache_path, VERILOGA_CACHE_RECORD_VERSION, |record| {
            let artifact = record.canonical_ir.as_mut().unwrap();
            artifact.digital.delays.checks.clear();
        });
        assert!(load().is_none(), "a plan that lost content is a miss");
        assert!(!cache_path.exists(), "and its record is discarded");
    }

    #[test]
    fn selected_modules_have_distinct_validated_disk_cache_entries() {
        let root = unique_test_root("module-selection");
        assert!(
            root.is_absolute()
                && root.starts_with(std::env::temp_dir())
                && root != std::env::temp_dir()
        );
        std::fs::create_dir_all(&root).unwrap();
        let source = root.join("modules.va");
        std::fs::write(&source, "module one(p,n); inout p,n; electrical p,n; analog I(p,n)<+V(p,n); endmodule\nmodule two(p,n); inout p,n; electrical p,n; analog I(p,n)<+2*V(p,n); endmodule\n").unwrap();
        let compiler = rspice_veriloga::VerilogACompiler::default();
        let keys = ["one", "two"].map(|name| VerilogASourceKey::new(&source, Some(name)));
        assert_ne!(
            cache_record_path_with_root(&keys[0], &root),
            cache_record_path_with_root(&keys[1], &root)
        );
        for key in &keys {
            let report = compiler
                .compile_file_runtime_with_metadata(&source, key.selected_module.as_deref())
                .unwrap();
            let entry = CachedVerilogAModel {
                dependencies: fingerprint_paths(&report.dependencies).unwrap(),
                model: std::sync::Arc::new(report.model),
                canonical_ir: Some(std::sync::Arc::new(report.canonical_ir)),
                diagnostics: report.diagnostics.into(),
            };
            persist_model_to_disk_at_with_limits(key, &entry, &root, ResourceLimits::default())
                .unwrap();
        }
        for key in keys.iter().rev() {
            let entry = load_model_from_disk_at_with_limits(
                key,
                &root,
                ResourceLimits::default(),
                &NoAbort,
            )
            .unwrap()
            .unwrap()
            .entry;
            assert_eq!(
                Some(entry.model.name.as_str()),
                key.selected_module.as_deref()
            );
        }
        for selection in [None, Some("ONE"), Some("missing")] {
            let key = VerilogASourceKey::new(&source, selection);
            assert!(
                load_model_from_disk_at_with_limits(
                    &key,
                    &root,
                    ResourceLimits::default(),
                    &NoAbort
                )
                .unwrap()
                .is_none()
            );
        }
        let cache_path = cache_record_path_with_root(&keys[0], &root);
        reseal_record(&cache_path, VERILOGA_CACHE_RECORD_VERSION, |record| {
            record.selected_module = Some("two".into());
        });
        assert!(
            load_model_from_disk_at_with_limits(
                &keys[0],
                &root,
                ResourceLimits::default(),
                &NoAbort
            )
            .unwrap()
            .is_none()
        );
        assert!(
            !cache_path.exists(),
            "a mismatched module identity is discarded"
        );
        std::fs::remove_dir_all(root).unwrap();
    }

    /// A deck line's `define=` macros key its source's disk record: the same
    /// file compiled under other macros is another record, a record answers
    /// only the macros it was compiled under, and a line without defines keys
    /// and writes its record exactly as before.
    #[test]
    fn deck_defines_have_distinct_validated_disk_cache_entries() {
        let root = unique_test_root("deck-defines");
        std::fs::create_dir_all(&root).unwrap();
        let source = root.join("defined.va");
        std::fs::write(&source, "`include \"disciplines.vams\"\nmodule defined(p,n); inout p,n; electrical p,n;\n`ifdef GAIN\nanalog I(p,n) <+ `GAIN*V(p,n);\n`else\nanalog I(p,n) <+ V(p,n);\n`endif\nendmodule\n").unwrap();
        let gain = |value: &str| {
            DeckHdl::default().with_source_defines(&[("GAIN".to_owned(), Some(value.to_owned()))])
        };
        assert_eq!(DeckHdl::default().defines_identity(), None);
        assert_ne!(gain("3").defines_identity(), gain("4").defines_identity());
        assert_eq!(
            VerilogASourceKey::new(&source, None).under(&DeckHdl::default()),
            VerilogASourceKey::new(&source, None),
            "no defines keys the source as it was keyed before"
        );
        let decks = [DeckHdl::default(), gain("3"), gain("4")];
        let keys = decks
            .each_ref()
            .map(|deck| VerilogASourceKey::new(&source, None).under(deck));
        let paths = keys
            .each_ref()
            .map(|key| cache_record_path_with_root(key, &root));
        assert!(paths[0] != paths[1] && paths[1] != paths[2] && paths[0] != paths[2]);
        let mut fingerprints = Vec::new();
        for (key, deck) in keys.iter().zip(&decks) {
            let report =
                rspice_veriloga::VerilogACompiler::new(deck_include_compiler_options(deck))
                    .compile_file_runtime_with_metadata(&source, None)
                    .unwrap();
            let entry = CachedVerilogAModel {
                dependencies: fingerprint_paths(&report.dependencies).unwrap(),
                model: std::sync::Arc::new(report.model),
                canonical_ir: Some(std::sync::Arc::new(report.canonical_ir)),
                diagnostics: report.diagnostics.into(),
            };
            fingerprints.push(
                runtime_artifact_fingerprint(&entry.model, entry.canonical_ir.as_deref()).unwrap(),
            );
            persist_model_to_disk_at_with_limits(key, &entry, &root, ResourceLimits::default())
                .unwrap();
        }
        assert!(
            fingerprints[0] != fingerprints[1] && fingerprints[1] != fingerprints[2],
            "the macros change what the source compiles to"
        );
        for (key, fingerprint) in keys.iter().zip(&fingerprints).rev() {
            let entry = load_model_from_disk_at_with_limits(
                key,
                &root,
                ResourceLimits::default(),
                &NoAbort,
            )
            .unwrap()
            .expect("each set of macros finds its own record");
            assert_eq!(
                &runtime_artifact_fingerprint(
                    &entry.entry.model,
                    entry.entry.canonical_ir.as_deref()
                )
                .unwrap(),
                fingerprint
            );
        }
        // A record that claims other macros than its key's is discarded.
        let other = gain("4").defines_identity();
        reseal_record(&paths[1], VERILOGA_CACHE_RECORD_VERSION, |record| {
            record.defines = other;
        });
        assert!(
            load_model_from_disk_at_with_limits(
                &keys[1],
                &root,
                ResourceLimits::default(),
                &NoAbort
            )
            .unwrap()
            .is_none()
        );
        assert!(
            !paths[1].exists(),
            "a mismatched define identity is discarded"
        );
        std::fs::remove_dir_all(root).unwrap();
    }

    fn project_registration(
        source_key: impl Into<PathBuf>,
        module_name: &str,
        aliases: &[&str],
    ) -> ProjectVerilogARuntimeRegistration {
        let report = rspice_veriloga::VerilogACompiler::default()
            .compile_runtime(
                &format!(
                    "module {module_name}(p, n); inout p, n; electrical p, n; analog I(p,n) <+ V(p,n); endmodule\n"
                ),
                None,
            )
            .expect("compile project runtime");
        ProjectVerilogARuntimeRegistration {
            source_key: source_key.into(),
            aliases: aliases.iter().map(|alias| (*alias).to_owned()).collect(),
            model: report.model,
            canonical_ir: report.canonical_ir,
            external_cells: None,
            delay_select: Default::default(),
        }
    }

    fn remove_project_runtime_keys(keys: &[&Path]) {
        let mut cache = veriloga_source_cache().write().expect("cache lock");
        for key in keys {
            cache.remove(&VerilogASourceKey::new(key, None));
        }
    }

    /// Install a runtime set and release the hold at once. The set stays
    /// installed until a later install evicts it, so an entry a check still
    /// finds after a refused install was left by the refusal, not kept by a
    /// hold, and these tests hold nothing against the others' budgets.
    fn install_runtimes(
        registrations: impl IntoIterator<Item = ProjectVerilogARuntimeRegistration>,
    ) -> Result<(), String> {
        hold_project_veriloga_sources_for_session(
            registrations
                .into_iter()
                .map(ProjectVerilogASourceRegistration::Runtime),
            ResourceLimits::default(),
        )
        .map(drop)
    }

    /// [`install_runtimes`] for one runtime, its module name its only alias.
    fn install_runtime(
        source_key: &Path,
        model: rspice_veriloga::CompiledModel,
        canonical_ir: rspice_veriloga::canonical_ir::CanonicalIrArtifact,
    ) -> Result<(), String> {
        install_runtimes([ProjectVerilogARuntimeRegistration {
            source_key: source_key.to_path_buf(),
            aliases: Vec::new(),
            model,
            canonical_ir,
            external_cells: None,
            delay_select: Default::default(),
        }])
    }

    #[test]
    fn disk_cache_loads_a_valid_paired_runtime_artifact() {
        let root = unique_test_root("valid");
        let source_path = root.join("model.va");
        std::fs::create_dir_all(&root).expect("create temporary cache root");
        let entry = compiled_entry(&source_path);
        let cache_root = root.join("cache");

        persist_model_to_disk_at(&source_path, &entry, &cache_root)
            .expect("persist valid cache record");
        let loaded = load_model_from_disk_at(&source_path, &cache_root)
            .expect("load valid cache record")
            .expect("valid cache record is retained");

        assert_eq!(loaded.model.name, entry.model.name);
        assert_eq!(
            loaded
                .canonical_ir
                .as_ref()
                .expect("canonical IR")
                .hir_digest,
            entry
                .canonical_ir
                .as_ref()
                .expect("canonical IR")
                .hir_digest
        );

        std::fs::remove_dir_all(root).expect("remove temporary cache root");
    }

    #[test]
    fn cancelled_disk_cache_read_preserves_the_valid_record() {
        let root = unique_test_root("cancelled-read");
        let source_path = root.join("model.va");
        std::fs::create_dir_all(&root).expect("create temporary cache root");
        let entry = compiled_entry(&source_path);
        let cache_root = root.join("cache");
        persist_model_to_disk_at(&source_path, &entry, &cache_root)
            .expect("persist valid cache record");
        let source_key = VerilogASourceKey::new(&source_path, None);
        let cache_path = cache_record_path_with_root(&source_key, &cache_root);
        let original = std::fs::read(&cache_path).expect("read cache fixture");
        // The first poll admits the lookup; the second is the record read's.
        let abort = crate::abort_signal::CountingAbort::new(1);

        let error = load_model_from_disk_at_with_limits(
            &source_key,
            &cache_root,
            ResourceLimits::default(),
            &abort,
        )
        .expect_err("partial cache decoding must return cancellation");

        assert!(matches!(error, SimulationError::Aborted));
        assert_eq!(abort.observed_at(), Some(2));
        assert_eq!(abort.polls_after_abort(), 0);
        assert_eq!(
            std::fs::read(&cache_path).expect("cache retained"),
            original
        );
        assert!(
            load_model_from_disk_at(&source_path, &cache_root)
                .expect("read the retained record")
                .is_some()
        );
        std::fs::remove_dir_all(root).expect("remove temporary cache root");
    }

    /// A native image is a cache file like a record: listed, so it is evicted
    /// with them, and refused and removed when its seal or its contents fail.
    #[cfg(feature = "veriloga-native")]
    #[test]
    fn disk_cache_refuses_and_removes_a_damaged_native_image() {
        let root = unique_test_root("native-image");
        std::fs::create_dir_all(&root).expect("create temporary cache root");
        let key = [7_u8; 32];
        let path = native_image_path_with_root(&key, &root);
        assert!(load_native_image_at(&key, &root, ResourceLimits::default()).is_none());

        let payload = veriloga_record::encode(&42_u64).expect("encode payload");
        let sealed_as_image = [
            &sealing_header(NATIVE_IMAGE_MAGIC, payload.len(), &blake3::hash(&payload))[..],
            &payload,
        ]
        .concat();
        std::fs::write(&path, &sealed_as_image).expect("write native image file");
        let listed = list_cache_files(&root).expect("list cache files");
        assert_eq!(listed.len(), 1);
        assert!(
            !listed[0].temporary,
            "a native image is evicted like a record"
        );
        assert!(
            load_native_image_at(&key, &root, ResourceLimits::default()).is_none(),
            "a payload that is not an image must not load"
        );
        assert!(!path.exists(), "an undecodable image is removed");

        let sealed_as_record = [
            &sealing_header(RECORD_MAGIC, payload.len(), &blake3::hash(&payload))[..],
            &payload,
        ]
        .concat();
        std::fs::write(&path, &sealed_as_record).expect("write record under an image name");
        assert!(load_native_image_at(&key, &root, ResourceLimits::default()).is_none());
        assert!(!path.exists(), "a record is never read as an image");

        std::fs::remove_dir_all(root).expect("remove temporary cache root");
    }

    /// A record's assignment passes are written apart from the rest, in
    /// sections the reader verifies and decodes side by side, and they read
    /// back as the record that was written.
    #[test]
    fn disk_cache_record_sets_its_passes_apart_and_reads_them_back() {
        let root = unique_test_root("record-sections");
        let source_path = root.join("model.va");
        std::fs::create_dir_all(&root).expect("create temporary cache root");
        let entry = compiled_assigning_entry(&source_path);
        assert!(!entry.model.assignment_steps.is_empty());
        let key = VerilogASourceKey::new(&source_path, None);
        let sealed =
            SealedRecord::new(&BorrowedVerilogACacheRecord::new(&key, &entry)).expect("seal");
        assert_eq!(
            sealed.sections.chunks.len(),
            DETACHED_MODEL_FIELDS.len(),
            "every detached field has its chunks"
        );
        assert!(!sealed.sections.chunks[0].is_empty());

        let (record, payload_len) =
            unseal_record(&sealed.file_parts().concat()).expect("unseal record");
        assert_eq!(payload_len, sealed.payload_len());
        assert_eq!(
            serde_json::to_value(&record.model).expect("serialize read model"),
            serde_json::to_value(&*entry.model).expect("serialize written model"),
        );
        std::fs::remove_dir_all(root).expect("remove temporary cache root");
    }

    /// Any byte of a record changed on disk, by a crash, a full disk or bad
    /// memory, is caught by a checksum before any value it vouches for is
    /// decoded: the table's in the header, each section's in the table.
    #[test]
    fn disk_cache_discards_a_damaged_record() {
        let root = unique_test_root("damaged-record");
        let source_path = root.join("model.va");
        std::fs::create_dir_all(&root).expect("create temporary cache root");
        let entry = compiled_assigning_entry(&source_path);
        let cache_root = root.join("cache");
        let cache_path =
            cache_record_path_with_root(&VerilogASourceKey::new(&source_path, None), &cache_root);
        persist_model_to_disk_at(&source_path, &entry, &cache_root).expect("persist cache record");
        let record_len = std::fs::read(&cache_path).expect("read cache record").len();

        for damage in [
            3,
            RECORD_HEADER_BYTES - 1,
            RECORD_HEADER_BYTES + 7,
            record_len / 2,
            record_len - 1,
        ] {
            persist_model_to_disk_at(&source_path, &entry, &cache_root)
                .expect("persist cache record");
            let mut bytes = std::fs::read(&cache_path).expect("read cache record");
            bytes[damage] ^= 0x10;
            std::fs::write(&cache_path, &bytes).expect("damage cache record");
            assert!(
                load_model_from_disk_at(&source_path, &cache_root)
                    .expect("a damaged record is a recoverable miss")
                    .is_none(),
                "a record damaged at byte {damage} must not load"
            );
            assert!(
                !cache_path.exists(),
                "a damaged record must be removed to prevent repeated failures"
            );
        }

        persist_model_to_disk_at(&source_path, &entry, &cache_root).expect("persist cache record");
        let bytes = std::fs::read(&cache_path).expect("read cache record");
        std::fs::write(&cache_path, &bytes[..bytes.len() - 1]).expect("truncate cache record");
        assert!(
            load_model_from_disk_at(&source_path, &cache_root)
                .expect("a truncated record is a recoverable miss")
                .is_none()
        );

        std::fs::remove_dir_all(root).expect("remove temporary cache root");
    }

    #[test]
    fn disk_cache_discards_records_from_older_semantic_contracts() {
        let root = unique_test_root("stale-event-state-version");
        let source_path = root.join("model.va");
        std::fs::create_dir_all(&root).expect("create temporary cache root");
        let entry = compiled_entry(&source_path);
        let cache_root = root.join("cache");
        let cache_path =
            cache_record_path_with_root(&VerilogASourceKey::new(&source_path, None), &cache_root);

        for (version, omit_canonical) in [
            (19_u32, false),
            (57, false),
            (57, true),
            (VERILOGA_CACHE_RECORD_VERSION - 1, false),
            (VERILOGA_CACHE_RECORD_VERSION - 1, true),
            (VERILOGA_CACHE_RECORD_VERSION + 1, false),
        ] {
            persist_model_to_disk_at(&source_path, &entry, &cache_root)
                .expect("persist current cache record");
            reseal_record(&cache_path, version, |record| {
                record.version = version;
                if omit_canonical {
                    record.canonical_ir = None;
                }
            });

            assert!(
                load_model_from_disk_at(&source_path, &cache_root)
                    .expect("stale-version cache load is recoverable")
                    .is_none(),
                "old semantic contracts must force a miss, including portable records without IR"
            );
            assert!(
                !cache_path.exists(),
                "stale-version cache record must be removed"
            );
        }

        std::fs::remove_dir_all(root).expect("remove temporary cache root");
    }

    #[test]
    fn oversized_disk_cache_read_becomes_a_miss_without_deleting_the_record() {
        let root = unique_test_root("bounded-read");
        let source_path = root.join("model.va");
        std::fs::create_dir_all(&root).expect("create temporary cache root");
        let entry = compiled_entry(&source_path);
        let cache_root = root.join("cache");
        persist_model_to_disk_at(&source_path, &entry, &cache_root)
            .expect("persist valid cache record");
        let cache_path =
            cache_record_path_with_root(&VerilogASourceKey::new(&source_path, None), &cache_root);
        let cache_bytes = usize::try_from(
            std::fs::metadata(&cache_path)
                .expect("cache metadata")
                .len(),
        )
        .expect("cache record fits usize");
        let limits = ResourceLimits::default()
            .with(|limits| limits.max_shared_cache_bytes = cache_bytes.saturating_sub(1));

        assert!(
            load_model_from_disk_at_with_limits(
                &VerilogASourceKey::new(&source_path, None),
                &cache_root,
                limits,
                &NoAbort,
            )
            .expect("an oversized optimization is a recoverable cache miss")
            .is_none()
        );
        assert!(
            cache_path.is_file(),
            "a caller-specific limit must not destroy a valid shared record"
        );

        std::fs::remove_dir_all(root).expect("remove temporary cache root");
    }

    #[test]
    fn disk_cache_write_streams_within_the_resource_budget() {
        let root = unique_test_root("bounded-write");
        let source_path = root.join("model.va");
        std::fs::create_dir_all(&root).expect("create temporary cache root");
        let entry = compiled_entry(&source_path);
        let cache_root = root.join("cache");
        let cache_path =
            cache_record_path_with_root(&VerilogASourceKey::new(&source_path, None), &cache_root);
        let limits = ResourceLimits::default().with(|limits| limits.max_shared_cache_bytes = 1);

        let error = persist_model_to_disk_at_with_limits(
            &VerilogASourceKey::new(&source_path, None),
            &entry,
            &cache_root,
            limits,
        )
        .expect_err("oversized cache record must not be persisted");
        assert!(error.contains("exceeds the 1 byte write limit"), "{error}");
        assert!(!cache_path.exists());
        assert!(
            list_cache_files(&cache_root)
                .expect("list cache files")
                .is_empty(),
            "a refused write leaves no file behind"
        );

        std::fs::remove_dir_all(root).expect("remove temporary cache root");
    }

    #[test]
    fn dependency_fingerprints_enforce_aggregate_source_bytes() {
        let root = unique_test_root("dependency-budget");
        std::fs::create_dir_all(&root).expect("create temporary dependency root");
        let first = root.join("first.va");
        let second = root.join("second.va");
        std::fs::write(&first, b"0123456789").expect("write first dependency");
        std::fs::write(&second, b"abcdefghij").expect("write second dependency");
        let limits =
            ResourceLimits::default().with(|limits| limits.max_dependency_source_bytes = 15);

        let error = fingerprint_paths_with_limits(&[first, second], limits)
            .expect_err("aggregate dependency bytes must be bounded");
        let SimulationError::ResourceLimit(error) = error else {
            panic!("unexpected fingerprint error: {error}");
        };
        assert_eq!(error.resource, ResourceKind::DependencySourceBytes);
        assert_eq!(error.requested, 20);
        assert_eq!(error.limit, 15);

        std::fs::remove_dir_all(root).expect("remove temporary dependency root");
    }

    #[test]
    fn oversized_model_is_not_admitted_to_the_shared_memory_cache() {
        let root = unique_test_root("memory-budget");
        let source_path = root.join("model.va");
        std::fs::create_dir_all(&root).expect("create temporary cache root");
        let entry = compiled_entry(&source_path);
        let key = VerilogASourceKey::new(&source_path, None);
        let retained_bytes =
            veriloga_model_cache_entry_bytes(&key, &entry).expect("size compiled model");

        assert!(
            !retain_veriloga_model(
                key.clone(),
                entry.clone(),
                retained_bytes.saturating_sub(1),
                false,
            )
            .expect("optional cache insertion is recoverable")
        );
        assert!(
            veriloga_source_cache()
                .read()
                .expect("cache lock")
                .get(&key)
                .is_none()
        );
        let error = retain_veriloga_model(key, entry, retained_bytes.saturating_sub(1), true)
            .expect_err("required project retention must fail closed");
        assert!(
            error.contains("shared_cache_bytes limit exceeded"),
            "{error}"
        );

        std::fs::remove_dir_all(root).expect("remove temporary cache root");
    }

    #[test]
    fn project_owned_runtime_registration_requires_no_ambient_source_file() {
        let source_key = PathBuf::from(
            "__rspice_project__/00000000-0000-0000-0000-000000000001/0123456789abcdef/model.va",
        );
        assert!(!source_key.exists());
        let report = rspice_veriloga::VerilogACompiler::default()
            .compile_runtime(
                "module owned(p, n); inout p, n; electrical p, n; analog I(p,n) <+ V(p,n); endmodule\n",
                None,
            )
            .expect("compile in-memory project source");

        install_runtime(&source_key, report.model, report.canonical_ir)
            .expect("register in-memory project runtime");

        let error = resolve_cached_or_compile_veriloga_with_limits_and_abort(
            &source_key,
            Some("owned"),
            ResourceLimits::default(),
            &NoAbort,
        )
        .expect_err("sealed runtimes must not accept a module override");
        assert!(
            error.to_string().contains("already fixes its module"),
            "{error}"
        );

        let key = VerilogASourceKey::new(&source_key, None);
        let mut cache = veriloga_source_cache().write().expect("cache lock");
        let entry = cache
            .get(&key)
            .and_then(CachedVerilogASource::runtime)
            .expect("session device entry");
        assert!(entry.dependencies.is_empty());
        assert_eq!(entry.model.name.as_str(), "owned");
        assert!(entry.canonical_ir.is_some());
        cache.remove(&key);
    }

    #[test]
    fn model_library_runtime_registration_requires_no_ambient_source_file() {
        let source_key =
            PathBuf::from("__rspice_model_library__/0123456789abcdef/root-digest/retained.va");
        assert!(!source_key.exists());
        let report = rspice_veriloga::VerilogACompiler::default()
            .compile_runtime(
                "module retained(p, n); inout p, n; electrical p, n; analog I(p,n) <+ V(p,n); endmodule\n",
                None,
            )
            .expect("compile in-memory model-library source");

        install_runtime(&source_key, report.model, report.canonical_ir)
            .expect("register sealed model-library runtime");

        let key = VerilogASourceKey::new(&source_key, None);
        let mut cache = veriloga_source_cache().write().expect("cache lock");
        let entry = cache
            .get(&key)
            .and_then(CachedVerilogASource::runtime)
            .expect("session device entry");
        assert!(entry.dependencies.is_empty());
        assert_eq!(entry.model.name.as_str(), "retained");
        cache.remove(&key);
    }

    #[test]
    fn same_file_name_in_distinct_projects_resolves_only_its_registered_runtime() {
        let compile = |module_name: &str| {
            rspice_veriloga::VerilogACompiler::default()
                .compile_runtime(
                    &format!(
                        "module {module_name}(p, n); inout p, n; electrical p, n; analog I(p,n) <+ V(p,n); endmodule\n"
                    ),
                    None,
                )
                .expect("compile project runtime")
        };
        let first_key = PathBuf::from(
            "__rspice_project__/00000000-0000-0000-0000-000000000011/digest/model.va",
        );
        let second_key = PathBuf::from(
            "__rspice_project__/00000000-0000-0000-0000-000000000022/digest/model.va",
        );
        let first = compile("first_owned");
        let second = compile("second_owned");
        install_runtime(&first_key, first.model, first.canonical_ir).unwrap();
        install_runtime(&second_key, second.model, second.canonical_ir).unwrap();

        assert_eq!(
            resolve_cached_or_compile_veriloga(&first_key)
                .unwrap()
                .model
                .name
                .as_str(),
            "first_owned"
        );
        assert_eq!(
            resolve_cached_or_compile_veriloga(&second_key)
                .unwrap()
                .model
                .name
                .as_str(),
            "second_owned"
        );

        let mut cache = veriloga_source_cache().write().unwrap();
        cache.remove(&VerilogASourceKey::new(&first_key, None));
        cache.remove(&VerilogASourceKey::new(&second_key, None));
    }

    #[test]
    fn plural_project_runtime_registration_installs_the_entire_batch() {
        let first_key =
            PathBuf::from("__rspice_project__/00000000-0000-0000-0000-000000000101/a/first.va");
        let second_key =
            PathBuf::from("__rspice_project__/00000000-0000-0000-0000-000000000101/b/second.va");

        install_runtimes([
            project_registration(&first_key, "batch_first", &["FIRST_ALIAS"]),
            project_registration(&second_key, "batch_second", &["SECOND_ALIAS"]),
        ])
        .expect("register complete project runtime batch");

        assert_eq!(
            resolve_cached_or_compile_veriloga(&first_key)
                .expect("first runtime")
                .model
                .name
                .as_str(),
            "batch_first"
        );
        assert_eq!(
            resolve_cached_or_compile_veriloga(&second_key)
                .expect("second runtime")
                .model
                .name
                .as_str(),
            "batch_second"
        );
        remove_project_runtime_keys(&[&first_key, &second_key]);
    }

    #[test]
    fn plural_registration_rolls_back_every_entry_on_an_installed_key_collision() {
        let project = "00000000-0000-0000-0000-000000000102";
        let installed_key = PathBuf::from(format!("__rspice_project__/{project}/stable/model.va"));
        let candidate_key =
            PathBuf::from(format!("__rspice_project__/{project}/candidate/model.va"));
        install_runtimes([project_registration(&installed_key, "stable_model", &[])])
            .expect("install stable runtime");

        let error = install_runtimes([
            project_registration(&candidate_key, "candidate_model", &[]),
            project_registration(&installed_key, "conflicting_model", &[]),
        ])
        .expect_err("a differing installed artifact must reject the whole batch");
        assert!(error.contains("differing installed artifact"), "{error}");
        assert_eq!(
            resolve_cached_or_compile_veriloga(&installed_key)
                .expect("stable runtime remains installed")
                .model
                .name
                .as_str(),
            "stable_model"
        );
        assert!(
            resolve_cached_or_compile_veriloga(&candidate_key).is_err(),
            "a rejected batch must not publish an earlier candidate"
        );
        remove_project_runtime_keys(&[&installed_key, &candidate_key]);
    }

    #[test]
    fn plural_registration_rolls_back_on_aggregate_resource_failure() {
        let stable_key = PathBuf::from(
            "__rspice_project__/00000000-0000-0000-0000-000000000103/stable/model.va",
        );
        let candidate_key = PathBuf::from(
            "__rspice_project__/00000000-0000-0000-0000-000000000103/candidate/model.va",
        );
        install_runtimes([project_registration(&stable_key, "resource_stable", &[])])
            .expect("install stable runtime");

        let candidate = project_registration(&candidate_key, "resource_candidate", &[]);
        let prepared = prepare_project_veriloga_registration(candidate.clone())
            .expect("prepare candidate runtime");
        let required = veriloga_source_cache_entry_bytes(
            &VerilogASourceKey::new(&prepared.key, None),
            &prepared.entry,
        )
        .expect("size candidate runtime");
        let error = hold_project_veriloga_sources_for_session(
            [ProjectVerilogASourceRegistration::Runtime(candidate)],
            ResourceLimits::default()
                .with(|limits| limits.max_shared_cache_bytes = required.saturating_sub(1)),
        )
        .expect_err("an undersized aggregate budget must reject the whole batch");
        assert!(
            error.contains("shared_cache_bytes limit exceeded"),
            "{error}"
        );
        assert_eq!(
            resolve_cached_or_compile_veriloga(&stable_key)
                .expect("stable runtime remains installed")
                .model
                .name
                .as_str(),
            "resource_stable"
        );
        assert!(resolve_cached_or_compile_veriloga(&candidate_key).is_err());
        remove_project_runtime_keys(&[&stable_key, &candidate_key]);
    }

    #[test]
    fn differing_case_colliding_keys_are_rejected_without_partial_installation() {
        let upper_key = PathBuf::from(
            "__rspice_project__/00000000-0000-0000-0000-000000000104/digest/Model.va",
        );
        let lower_key = PathBuf::from(
            "__rspice_project__/00000000-0000-0000-0000-000000000104/digest/model.va",
        );
        let error = install_runtimes([
            project_registration(&upper_key, "upper_model", &[]),
            project_registration(&lower_key, "lower_model", &[]),
        ])
        .expect_err("case-colliding keys with differing artifacts must fail");
        assert!(error.contains("case-colliding"), "{error}");
        assert!(resolve_cached_or_compile_veriloga(&upper_key).is_err());
        assert!(resolve_cached_or_compile_veriloga(&lower_key).is_err());
    }

    #[test]
    fn differing_case_colliding_aliases_are_rejected_within_one_project() {
        let first_key =
            PathBuf::from("__rspice_project__/00000000-0000-0000-0000-000000000105/a/first.va");
        let second_key =
            PathBuf::from("__rspice_project__/00000000-0000-0000-0000-000000000105/b/second.va");
        let error = install_runtimes([
            project_registration(&first_key, "alias_first", &["SharedAlias"]),
            project_registration(&second_key, "alias_second", &["sharedalias"]),
        ])
        .expect_err("case-colliding aliases with differing artifacts must fail");
        assert!(error.contains("alias 'SHAREDALIAS'"), "{error}");
        assert!(resolve_cached_or_compile_veriloga(&first_key).is_err());
        assert!(resolve_cached_or_compile_veriloga(&second_key).is_err());
    }

    #[test]
    fn identical_reinstall_is_idempotent_and_preserves_the_cached_artifact() {
        let key = PathBuf::from(
            "__rspice_project__/00000000-0000-0000-0000-000000000106/digest/model.va",
        );
        let registration = project_registration(&key, "idempotent_model", &["stable_alias"]);
        install_runtimes([registration.clone()]).expect("initial registration");
        let before = resolve_cached_or_compile_veriloga(&key).expect("initial cached runtime");

        install_runtimes([registration]).expect("identical reinstall");
        let after = resolve_cached_or_compile_veriloga(&key).expect("reinstalled cached runtime");
        assert!(std::sync::Arc::ptr_eq(&before.model, &after.model));
        remove_project_runtime_keys(&[&key]);
    }

    #[test]
    fn cache_telemetry_reports_memory_hits_as_monotonic_deltas() {
        let key = PathBuf::from(
            "__rspice_project__/00000000-0000-0000-0000-000000000109/digest/telemetry.va",
        );
        install_runtimes([project_registration(&key, "telemetry_model", &[])])
            .expect("register telemetry runtime");
        let before = veriloga_cache_telemetry();

        resolve_cached_or_compile_veriloga(&key).expect("first memory hit");
        resolve_cached_or_compile_veriloga(&key).expect("second memory hit");
        let after = veriloga_cache_telemetry();

        assert!(after.lookups.saturating_sub(before.lookups) >= 2);
        assert!(after.memory_hits.saturating_sub(before.memory_hits) >= 2);
        remove_project_runtime_keys(&[&key]);
    }

    #[test]
    fn cache_resolution_honors_cancellation_before_io() {
        let before = veriloga_cache_telemetry();
        let error = resolve_cached_or_compile_veriloga_with_limits_and_abort(
            Path::new("must-not-be-read.va"),
            None,
            ResourceLimits::default(),
            &crate::abort_signal::ImmediateAbort,
        )
        .expect_err("immediate cancellation must stop cache resolution");
        let after = veriloga_cache_telemetry();

        assert!(matches!(error, SimulationError::Aborted));
        assert!(after.lookups > before.lookups);
    }

    struct AbortOnCompilerProgress {
        aborted: std::sync::atomic::AtomicBool,
    }

    impl AbortSignal for AbortOnCompilerProgress {
        fn is_aborted(&self) -> bool {
            self.aborted.load(std::sync::atomic::Ordering::Relaxed)
        }

        fn observe_progress(&self, fraction: f64) {
            if fraction > 0.0 {
                self.aborted
                    .store(true, std::sync::atomic::Ordering::Relaxed);
            }
        }
    }

    #[test]
    fn cache_maps_compiler_cancellation_to_simulation_abort() {
        let root = unique_test_root("compile-cancellation");
        std::fs::create_dir_all(&root).expect("create cancellation fixture root");
        let source = root.join("cancel.va");
        std::fs::write(
            &source,
            "module cancel(p,n); inout p,n; electrical p,n; analog I(p,n) <+ V(p,n); endmodule\n",
        )
        .expect("write cancellation fixture");
        let abort = AbortOnCompilerProgress {
            aborted: std::sync::atomic::AtomicBool::new(false),
        };
        let before = veriloga_cache_telemetry();

        let error = resolve_cached_or_compile_veriloga_with_limits_and_abort(
            &source,
            None,
            ResourceLimits::default(),
            &abort,
        )
        .expect_err("compiler progress callback must cancel cache resolution");
        let after = veriloga_cache_telemetry();

        assert!(matches!(error, SimulationError::Aborted));
        assert!(after.compilations_started > before.compilations_started);
        assert!(after.compilations_cancelled > before.compilations_cancelled);
        std::fs::remove_dir_all(root).expect("remove cancellation fixture root");
    }

    #[test]
    fn same_filename_and_alias_remain_isolated_between_projects() {
        let first_key = PathBuf::from(
            "__rspice_project__/00000000-0000-0000-0000-000000000107/digest/device.va",
        );
        let second_key = PathBuf::from(
            "__rspice_project__/00000000-0000-0000-0000-000000000108/digest/device.va",
        );
        install_runtimes([
            project_registration(&first_key, "isolated_first", &["device_alias"]),
            project_registration(&second_key, "isolated_second", &["DEVICE_ALIAS"]),
        ])
        .expect("project-scoped aliases may be reused across projects");

        assert_eq!(
            resolve_cached_or_compile_veriloga(&first_key)
                .expect("first project runtime")
                .model
                .name
                .as_str(),
            "isolated_first"
        );
        assert_eq!(
            resolve_cached_or_compile_veriloga(&second_key)
                .expect("second project runtime")
                .model
                .name
                .as_str(),
            "isolated_second"
        );
        remove_project_runtime_keys(&[&first_key, &second_key]);
    }

    #[test]
    fn project_virtual_cache_identity_is_lexical_even_if_an_ambient_path_exists() {
        let unique = unique_test_root("virtual-lexical")
            .file_name()
            .expect("unique component")
            .to_string_lossy()
            .into_owned();
        let source_key = PathBuf::from(format!(
            "__rspice_project__/{unique}/0123456789abcdef/model.va"
        ));
        let parent = source_key.parent().expect("virtual key parent");
        std::fs::create_dir_all(parent).expect("materialize adversarial ambient directory");
        std::fs::write(
            &source_key,
            "ambient bytes must never define cache identity",
        )
        .expect("materialize adversarial ambient file");

        assert_eq!(canonicalize_for_cache(&source_key), source_key);

        std::fs::remove_dir_all(PathBuf::from("__rspice_project__").join(&unique))
            .expect("remove adversarial ambient path");
        assert_eq!(canonicalize_for_cache(&source_key), source_key);
    }

    #[test]
    fn missing_project_virtual_runtime_never_falls_back_to_ambient_io() {
        let source_key = PathBuf::from(
            "__rspice_project__/00000000-0000-0000-0000-000000000099/missing/model.va",
        );
        let error = resolve_cached_or_compile_veriloga(&source_key)
            .expect_err("unregistered project runtime must fail closed");
        assert!(
            error
                .to_string()
                .contains("is not installed for this execution")
        );
    }

    #[test]
    fn case_altered_project_virtual_path_never_compiles_an_ambient_file() {
        let unique = unique_test_root("virtual-case")
            .file_name()
            .expect("unique component")
            .to_string_lossy()
            .into_owned();
        let exact_source_key = PathBuf::from(format!(
            "__rspice_project__/{unique}/0123456789abcdef/model.va"
        ));
        std::fs::create_dir_all(exact_source_key.parent().expect("virtual key parent"))
            .expect("materialize adversarial ambient directory");
        std::fs::write(
            &exact_source_key,
            "module ambient_escape(p, n); inout p, n; electrical p, n; analog I(p,n) <+ V(p,n); endmodule\n",
        )
        .expect("materialize compilable adversarial ambient source");

        let altered_source_key = PathBuf::from(format!(
            "__RSPICE_PROJECT__/{unique}/0123456789abcdef/model.va"
        ));
        let error = resolve_cached_or_compile_veriloga(&altered_source_key)
            .expect_err("case-altered project key must fail before ambient compilation");
        assert!(
            error
                .to_string()
                .contains("is not installed for this execution"),
            "unexpected error: {error}"
        );

        std::fs::remove_dir_all(PathBuf::from("__rspice_project__").join(unique))
            .expect("remove adversarial ambient path");
    }

    /// The record key is the file, not the words the deck used to name it.
    ///
    /// Two decks in two directories both write `.va "d.va"`; one file reached
    /// from two directories is written two ways. The netlist parser resolves
    /// each spelling against the file that wrote it before the key is taken,
    /// which is what makes the first pair two records and the second pair one.
    #[test]
    fn a_cache_key_follows_the_resolved_source_path_not_its_spelling() {
        let root = unique_test_root("resolved-key");
        let first_directory = root.join("first");
        let second_directory = root.join("second");
        std::fs::create_dir_all(&first_directory).expect("create the first source directory");
        std::fs::create_dir_all(&second_directory).expect("create the second source directory");
        let first = first_directory.join("d.va");
        let second = second_directory.join("d.va");
        std::fs::write(&first, "// the first deck's model\n").expect("write the first source");
        std::fs::write(&second, "// the second deck's model\n").expect("write the second source");

        let first_key = VerilogASourceKey::new(&first, None);
        let second_key = VerilogASourceKey::new(&second, None);
        assert_ne!(
            first_key, second_key,
            "two files of one name in two directories are two records"
        );
        assert_ne!(
            cache_record_path_with_root(&first_key, &root),
            cache_record_path_with_root(&second_key, &root),
            "two keys must address two record files"
        );

        let detour = second_directory.join("..").join("first").join("d.va");
        let detour_key = VerilogASourceKey::new(&detour, None);
        assert_eq!(
            detour_key, first_key,
            "one file written two ways is one record"
        );
        assert_eq!(
            cache_record_path_with_root(&detour_key, &root),
            cache_record_path_with_root(&first_key, &root),
            "one key must address one record file"
        );

        // A module selection still separates records taken from one file.
        assert_ne!(
            VerilogASourceKey::new(&first, Some("one")),
            VerilogASourceKey::new(&first, Some("two")),
            "a module selection is part of the key"
        );

        // And the reason resolution has to happen first: an unresolved
        // relative spelling has nothing to canonicalize against, so it becomes
        // its own key and every deck that wrote it would share one record.
        assert_eq!(
            canonicalize_for_cache(Path::new("no-such-relative-source.va")),
            PathBuf::from("no-such-relative-source.va"),
            "an unresolvable relative spelling is its own key"
        );

        std::fs::remove_dir_all(&root).expect("remove the fixture root");
    }
}
