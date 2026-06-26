# Verilog-A Generated Runtime Performance

## Goal

Reduce user-visible simulation wait time for generated Verilog-A semiconductor models by eliminating avoidable hot-path allocation and zero-initialization in generated Rust stamp functions.

## Current Bottleneck

Generated `stamp()` and `stamp_reactive()` methods create fresh `Scratch` or `ReactiveScratch` arrays every call. Large models such as BSIM-class devices have thousands of variables and many derivative slots, so this zeroes a substantial dense scratch buffer during every nonlinear evaluation.

## Implementation Plan

1. Change backend tests so the expected generated shape is reusable instance-owned scratch, not per-stamp `Scratch::new()`.
2. Emit `scratch` and `reactive_scratch` fields in generated `Instance` state using the configured runtime support path.
3. Remove `Copy` from generated `Instance` and emit a manual `Clone` that copies semantic nonlinear state while creating fresh scratch buffers, keeping solver snapshots from copying large ephemeral arrays.
4. Emit `let s = &mut self.scratch;` and `let s = &mut self.reactive_scratch;` in stamp methods.
5. Regenerate built-in Verilog-A models and verify large generated devices no longer construct scratch buffers inside hot stamp methods.
6. Run backend and core checks covering generation, runtime ABI, and built-in generated models.

## Validation

- `cargo test -p rspice-veriloga --test rust_backend -- --nocapture`
- `cargo check -p rspice-core --features veriloga-builtins`
- Focused inspection of generated `bsimbulk` stamp/state output for scratch reuse.
