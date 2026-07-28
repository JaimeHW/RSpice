# rspice-core architecture

This document records decisions that the code cannot state for itself: why the
crate is shaped the way it is, and which of its apparent oddities are
deliberate. For what each module *contains*, read the module docs.

## One crate, enforced by test

`rspice-core` is a single crate holding the parser, the device models, the
sparse assembly, the solvers, and the analyses. Splitting them into published
crates would let the compiler enforce layering for free. We do not, because the
pieces are one numerical program: a device model, the companion stamp it
produces, and the Newton loop that consumes it are co-designed and change
together. A crate boundary between them would buy layering at the cost of a
versioning surface across a seam that has no independent consumers.

The cost of that choice is that nothing stops an import that inverts the
architecture. [`tests/module_layering.rs`](tests/module_layering.rs) is the
substitute: it declares the intended order and fails when a module reaches
sideways or upward. Every violation present today is recorded there with an
exact count that may fall but never rise, tagged with the work that retires it.

Three sibling tests guard the same boundary from other angles:

| Test | Fails when |
| --- | --- |
| `module_layering` | a module references one at or above its rank |
| `module_reachability` | a `src/**/*.rs` file is not declared by `mod` or `include!` |
| `public_surface` | the export count grows, or the ceiling goes stale |
| `doc_examples` | the crate-root example stops working |

`module_reachability` exists because three files had drifted out of the crate
entirely, and one of them silently absorbed a bug fix: commit `ec566f064`
rewrote `expect_model_name` in a file nothing declared, so none of it reached
the binary. A stale duplicate of live code answers "where does this behavior
live?" with a confident wrong answer.

## Layer order

Lowest first. A module may reference any module of strictly lower rank.

```
engine        facade: config resolution, dispatch, health, abort plumbing
compat        foreign-format IO (LTspice RAW)
analysis      analysis algorithms and result types
circuit       struct-of-arrays storage and stamping
xspice        XSPICE code models (a device extension with an event queue)
device        device model evaluation; Verilog-A and FFI extension points
library       .lib and Verilog-A pack discovery
netlist       deck text to AST
expr          bytecode expression VM
solver        sparse LU, Newton, damping, continuation
simd          SIMD kernels
constants  naming  resource  abort_signal  time_compat  diagnostics  builtin_lib
```

The order is **total wherever an edge exists**. Ties are how a cycle hides
inside a layer: `solver -> simd` and `library -> netlist` were both invisible
while those pairs shared a rank. The layer-0 leaves may share rank 0 only
because they genuinely do not reference each other.

Ranks encode the target, not today's tree. Where the two differ, the gap is in
`ALLOWED_VIOLATIONS`, not in this document.

## Deliberate decisions that look like defects

### `CircuitData` is a wide struct on purpose

`CircuitData` carries roughly sixty fields, about one per device family, and
devices are stored struct-of-arrays rather than behind a trait object. This is
the design, not an accumulation.

Newton iterates over every device on every step. Struct-of-arrays keeps a
family's parameters contiguous so evaluation is cache-friendly and
vectorizable, and the precomputed `CscIndex` positions let a device stamp
directly into the factored matrix with no intermediate triplet buffer. Virtual
dispatch per device per iteration would cost more than the abstraction is
worth.

The consequence is real and should be understood before adding a device family:
there is no single extension point. A new family touches `CircuitData`,
construction, stamping, the transient state machinery, snapshot/restore, and
introspection. That is the price of the layout, and it is paid deliberately.

Do not "fix" this by introducing `Box<dyn Device>`. If the per-family
boilerplate becomes the bottleneck, the direction is a batch-first contract
over slices — `eval_batch(&mut self, bias: &[f64], out: &mut StampBuffer)` —
which preserves the layout and gives SIMD and rayon a seam. Two earlier
attempts at that (`device::batch`, `solver::parallel`) were written, never
wired to anything, and deleted in `daa95ad98`; the git history has them if the
idea is revived.

### There are two expression evaluators, and that is correct

`expr` is a bytecode compiler and VM. `netlist::expr` is a complex-valued
tree-walking evaluator. Both handle behavioral sources, and they are not
redundant.

They mirror ngspice's own split: `numparam` evaluates `.PARAM` at parse time,
`inpptree` evaluates B-source expressions during the solve. The two disagree on
purpose in ways that are oracle-pinned — operator associativity (`2^-3^2`),
whether `LOG` is natural or base 10 under the Xyce dialect, and the meaning of
`mil`. Unifying them would be a conformance regression, not a cleanup.

If you are here because the duplication looked like an obvious win: it is not.
Check `expr/parser.rs` for the pinned associativity cases first.

### The ground-name predicate is literal

`naming::is_spice_ground_name` matches `"0"` and nothing else — not `GND`, not
`gnd!`, not `00`. Dialect aliases are folded during elaboration, where the
`REPLACEGROUND` setting is known. Folding them in the predicate would fold them
even when Xyce `REPLACEGROUND` is off, silently collapsing an ordinary node
named `GND` to ground.

### Generated Verilog-A is excluded from architectural review

`device/veriloga_generated/` is 172 files and 49 MB of machine output. The
layering and reachability tests skip it. Generated code is not subject to
architectural review; the generator is. Its size is gated separately by the
`generated-rust` budget in CI.

### Unit tests do not run by default

`[lib] test = false` and `doctest = false` in `Cargo.toml`. Run unit tests with
`cargo test -p rspice-core --lib`; CI does this explicitly. Doctests are off
for build cost, which means every `rust` block in the docs is checked by review
alone — so the examples that make a promise about the API are mirrored as real
tests in [`tests/doc_examples.rs`](tests/doc_examples.rs).

## Working on this crate

- New code is `pub(crate)` unless a frontend needs it. The five frontends name
  roughly two hundred distinct paths; the crate exports far more than that, and
  `public_surface` keeps the number from growing further.
- A new top-level module must be given a rank in `LAYERS`. A module with no
  declared position is a module nobody decided the position of.
- When an allowlisted violation reaches zero, delete its entry. The test fails
  until you do, so the list cannot rot into a record of problems that no longer
  exist.
