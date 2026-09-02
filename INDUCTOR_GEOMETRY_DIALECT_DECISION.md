# Linear-inductor geometry dialect compatibility decision

- Status: accepted
- Decision date: 2026-09-01
- Finding: AR-14
- Scope: native linear-inductor value resolution for the `Ngspice`,
  `BestAvailable`, and `Xyce` compatibility policies

## Decision

RSpice implements `NT` plus `LENGTH` and `DIA`/`CSECT` geometry synthesis as
an ngspice-compatible linear-inductor feature. The internal `BestAvailable`
policy also enables that qualified synthesis. The Xyce compatibility policy
must not synthesize a linear inductance from those parameters.

Under `SpiceDialect::Xyce`, a linear inductor requires a base `L` on the
instance. Model `L` is a dimensionless multiplier. Model parameters `NT`,
`LENGTH`, `DIA`, `CSECT`, and `MU` are unsupported and must fail closed even
when an instance base `L` is present. Silently ignoring those parameters would
misrepresent an authored geometry model as an ordinary Xyce inductor.

Nonlinear mutual-inductor `K`/`CORE` geometry and winding-turn semantics are a
separate device contract and are not changed by this decision.

## Compatibility evidence

The current Xyce Reference Guide 7.10 (SAND2025-05993, May 2025), section
2.3.5 and tables 2-39 and 2-40, defines the linear-inductor contract as:

- a required instance base inductance `L`;
- instance parameters `DTEMP`, `IC`, `L`, `M`, `TC1`, `TC2`, and `TEMP`;
- model parameters `IC`, `L`, `TC1`, `TC2`, and `TNOM`; and
- effective inductance equal to the instance base value times the model `L`
  multiplier and the documented temperature polynomial, divided by instance
  multiplicity `M`.

The guide contains no linear-inductor `NT`, `LENGTH`, `DIA`, `CSECT`, or `MU`
parameter. Its following mutual-inductor section documents the distinct
nonlinear `CORE` model, where component-inductor values become winding turns
and core geometry uses parameters such as `AREA` and `PATH`.

The official Xyce implementation in `N_DEV_Inductor.C` independently confirms
the same parameter registrations, model multiplier equation, and error when
an instance `L` is absent. The bundled Xyce regression corpus contains no
linear-inductor `NT`/`LENGTH`/`CSECT` geometry oracle, so it is not used as
evidence for semantics that Xyce does not advertise.

- [Official Xyce documentation page][xyce-current-docs]
- [Xyce Reference Guide 7.10][xyce-reference-710]
- [Official Xyce linear-inductor source][xyce-inductor-source]

[xyce-current-docs]: https://xyce.sandia.gov/documentation-tutorials/
[xyce-reference-710]: https://xyce.sandia.gov/download/2068/
[xyce-inductor-source]: https://github.com/Xyce/Xyce/blob/master/src/DeviceModelPKG/OpenModels/N_DEV_Inductor.C

## Enforcement and qualification

- `crates/rspice-core/src/engine/builder/model_resolution/inductors.rs`
  enables geometry synthesis only outside the Xyce branch and explicitly
  rejects Xyce model cards containing any ngspice geometry parameter.
- Unit tests in that module qualify ngspice and `BestAvailable` synthesis
  against the same analytical Lundin oracle.
- Xyce unit tests cover geometry-only rejection, geometry-plus-base-`L`
  rejection, the required-base-value boundary, and the valid base-`L` times
  model-`L` times temperature divided by multiplicity contract.
- `crates/rspice-core/tests/inductor_geometry.rs` qualifies the synthesized
  ngspice geometry through an AC impedance solve.

## Reconsideration triggers

Reopen this decision only if a supported Xyce release documents and implements
linear-inductor geometry synthesis, or if RSpice introduces another explicitly
named compatibility policy with authoritative geometry semantics. A nonlinear
`CORE` model requirement must be implemented through the mutual-inductor path,
not by widening the Xyce linear-inductor contract.
