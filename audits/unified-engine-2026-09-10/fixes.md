# Unified-engine audit fixes

Implemented against `5b1d703ef228879bb50feef5ae469f00cc2613b5` in an isolated checkout. The existing Verilog-AMS implementation roadmap remains the broader work plan.

## Changes

- Scheduled digital analog reads now use the current Newton candidate. Time-zero processes that read analog quantities wait for that candidate too. Rejected evaluations replay from the original digital state.
- An analog crossing settles its zero-delay digital consequences without draining unrelated timers at the crossing's rounded reporting tick. Positive delays retain their digital timestamps; active, inactive, and nonblocking updates retain their ordering and scheduler limits.
- Include wrappers and macro-bearing sources are examined for connect rules after preprocessing. Plain sources without rules or preprocessing still take the inexpensive filter; sources without rules avoid duplicate semantic analysis.
- A `Z` D/A output releases its analog node. `X` retains the existing midpoint-drive policy.
- Numeric mixed-instance parameters specialize both analog and digital behavior before elaboration. Dependent defaults, digital expressions, and packed dimensions see the same overrides; analog construction retains `$param_given`. Authenticated preprocessed source travels with parameterized mixed artifacts, including serialized artifacts. Identical specializations share compiled artifacts within a circuit build.
- Scalar discrete variables and real variables can supply analog equations through the existing canonical state-variable ABI. Input changes invalidate the analog evaluation, and rejection restores the input bank. Candidate inspection and acceptance settle digital behavior before evaluating the analog equations they commit.

The canonical schema and disk-cache versions advance to invalidate incompatible artifacts. Shared four-state reads currently support signed values up to 32 bits and unsigned values up to 31 bits; wider reads are refused. X/Z values used directly in numeric analog equations produce a diagnostic. Two-domain writes remain illegal.

## Verification

- 10 focused circuit regressions pass with both portable and native execution: the original seven probes, solved time-zero sampling, state-dependent analog conductance/Jacobian behavior, and transitions between driven, released, and unknown outputs.
- 40 existing mixed-route/hardening integration tests pass on each backend.
- 64 native digital/mixed-host tests pass, including the new shared-real rejection and checkpoint regression.
- Compiler coverage passed: 135 canonical IR, 75 digital grammar, 61 digital process IR, 14 hierarchy, 4 mixed runtime compilation, 29 runtime-report, and 13 virtual-source tests. The mixed compilation tests cover serialized parameter specialization, dimension changes, source tampering, malformed overrides, and unsupported shared-variable forms.
- 14 connect-library tests pass.
- `rspice-core` checks successfully for `wasm32-unknown-unknown` with `veriloga-wasm-jit`. This is a compilation check, not a browser or physical-tablet execution qualification.

Representative commands (run from the isolated checkout, with separate target directories):

```text
cargo test -p rspice-core --locked --no-default-features --features veriloga --test veriloga_mixed_signal_regressions --test veriloga_mixed_signal_route --test veriloga_mixed_signal_hardening
cargo test -p rspice-core --locked --no-default-features --features veriloga-native --test veriloga_mixed_signal_regressions --test veriloga_mixed_signal_route --test veriloga_mixed_signal_hardening
cargo test -p rspice-core --locked --no-default-features --features veriloga-native --lib xspice::verilog::
cargo check -p rspice-core --locked --no-default-features --features veriloga-wasm-jit --target wasm32-unknown-unknown
```

These fixes close the six reproduced failures. They do not establish full Verilog-AMS conformance, Spectre equivalence, or overall production readiness. The broader architecture and qualification gaps in the existing roadmap remain separate work.
