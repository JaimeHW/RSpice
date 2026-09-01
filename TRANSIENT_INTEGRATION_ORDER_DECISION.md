# Transient integration-order compatibility decision

- Status: accepted
- Decision date: 2026-09-01
- Finding: AR-16
- Scope: the Xyce-compatible `TIMEINT` contract and RSpice's native
  backward-Euler, trapezoidal, and Gear integration modes

## Decision

RSpice intentionally supports transient integration orders 1 and 2 only.
`TIMEINT MINORD` and `TIMEINT MAXORD` values outside that range must continue
to fail with a typed configuration or parse error. Higher-order Gear/BDF is not
part of the current Xyce-compatibility contract and must not be inferred from a
numeric option range alone.

This closes AR-16 as an evidence-backed product boundary. Phase 9.1 through
9.3 are not implementation requirements while this decision remains active.

## Compatibility evidence

The current Xyce Users' Guide 7.10 (SAND2025-05995, May 2025), section 7.3.4
and table 7-3, defines the available transient methods as:

- variable-order trapezoidal, combining backward Euler and trapezoidal rule;
- Gear with orders 1 and 2;
- backward Euler selected by `METHOD=TRAP MAXORD=1`;
- trapezoidal-only selected by `METHOD=TRAP MINORD=2`; and
- Gear2-only selected by `METHOD=GEAR MINORD=2`.

The corresponding Xyce Reference Guide 7.10 documents `TIMEINT MAXORD` with a
default of 2 for both variable trapezoid and Gear, and describes `MINORD` only
in terms of movement between orders 1 and 2. Both current guides are published
from the [official Xyce documentation page][xyce-current-docs].

The Xyce command-line reference also exposes a generic `-maxord` numeric range
through 5. That range is not treated as evidence that Xyce's documented
transient methods implement or qualify orders 3 through 5: the method-specific
Users' Guide table explicitly limits Gear to orders 1 and 2. RSpice follows the
documented method semantics.

An independently archived earlier guide reaches the same conclusion: the
[Xyce Users' Guide 7.6][xyce-76-doi] documents Gear orders 1 and 2 in its
transient-integration method table.

[xyce-current-docs]: https://xyce.sandia.gov/documentation-tutorials/
[xyce-76-doi]: https://doi.org/10.2172/1895029

## Existing enforcement and qualification

The boundary is enforced at both authored and programmatic entry points:

- `crates/rspice-core/src/netlist/parser/commands.rs` accepts only integer
  `MINORD`/`MAXORD` values 1 or 2 and rejects `MAXORD=3`, invalid fractional
  values, and inverted bounds.
- `crates/rspice-core/src/config.rs` validates programmatic order bounds before
  execution and returns `InvalidTransientIntegrationOrder` on violation.
- `crates/rspice-core/src/engine/transient.rs` implements order startup,
  promotion, demotion, breakpoint restart, and resume behavior for this bounded
  order set.
- `crates/rspice-core/tests/transient_integration_order.rs` qualifies
  second-order behavior against an ngspice BSIM3 timing oracle, while core unit
  and checkpoint tests cover fixed-order and restart semantics.

No silent clamping or fallback is permitted. A request for an unsupported
order remains an explicit error so the caller cannot mistake an order-2 run for
the requested higher-order analysis.

## Reconsideration triggers

Reopen this decision only if at least one of the following occurs:

1. a supported compatibility dialect documents and qualifies a method above
   order 2;
2. a product requirement explicitly calls for native higher-order BDF/Gear;
   or
3. measured customer workloads establish a material need that cannot be met by
   the existing adaptive order-1/order-2 methods.

Reopening requires the complete Phase 9.1 through 9.3 program: generalized
history and coefficients, every dynamic-device and checkpoint state update,
stability/demotion policy, and the full manufactured-solution, stiff,
oscillatory, switching, breakpoint, and resume qualification matrix. Merely
widening the parser or configuration range is prohibited.
