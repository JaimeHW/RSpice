# Native MOS3 Design

## Purpose

RSpice currently accepts `LEVEL=3` MOSFET model cards, but routes them through the simplified short-channel fallback and warns that MOS3-specific parameters are not honored. That is acceptable for compatibility smoke tests, but not for commercial-grade SPICE accuracy: a deck that asks for Berkeley MOS3 should run the MOS3 equations, not a plausible approximation.

This design adds a native ngspice-compatible MOS3 path for bulk MOSFET `LEVEL=3`. Ngspice 46 is the primary source of truth; neospice's MOS3 port is useful as a secondary structural reference. The slice is intentionally bounded to MOS3 so it removes one known approximation without entangling larger model-family ports such as HiSIM, HICUM, or MOS9.

## Requirements

- `.model ... NMOS/PMOS (LEVEL=3 ...)` routes to native MOS3 behavior without requiring `.options allow_simplified_mos=1` and without emitting the current simplified-fallback warning.
- The native path honors the MOS3 parameter family currently ignored by the fallback: `ETA`, `DELTA`, `NFS`, `THETA`, `VMAX`, `KAPPA`, and `XJ`, along with the shared Berkeley MOS geometry, junction, capacitance, and temperature parameters already parsed for MOS1/MOS2/MOS6.
- The implementation follows ngspice 46 MOS3 equations for DC operating point, DC sweep, small-signal AC, and transient charge behavior as far as those analyses are represented in RSpice's existing MOSFET infrastructure.
- PMOS polarity, inverse mode, source/drain sheet resistance, instance multiplier, instance geometry, `.TEMP`/`TNOM`, and body-diode behavior remain wired through the simulator consistently with existing bulk MOS models.
- Unsupported MOS levels outside the existing native set continue to reject or require the simplified fallback opt-in exactly as they do today.
- Existing native MOS1/MOS2/MOS6, BSIM1/BSIM2, BSIM3, BSIM4, BSIM3-SOI, VDMOS, and JFET/JFET2 behavior must not regress.
- Documentation and model-support tables must describe MOS3 as native once the implementation is complete.

## Architecture

### Model Routing

Update the MOS model-level policy so `native_bulk_mos_level` includes level 3. The builder should no longer take the simplified fallback branch for `LEVEL=3`; instead it should construct a normal `Mosfet` instance with a native level-3 evaluator selected by `mosfet.level == 3`.

The policy tests should change from "LEVEL=3 runs with warning" to "LEVEL=3 runs natively." BSIM-class unsupported-level behavior remains unchanged: unported BSIM levels still error unless `allow_simplified_mos=1` is explicit.

### Parameter Surface

Extend the existing MOSFET parameter storage only where necessary. Shared parameters already handled by `Mosfet::apply_model_params` should remain shared. MOS3-only parameters should have explicit fields with defaults matching ngspice:

- `eta`: drain-induced barrier lowering coefficient.
- `delta` / narrow-channel factor input.
- `nfs`: fast surface-state density.
- `theta`: mobility degradation coefficient.
- `vmax`: maximum carrier drift velocity.
- `kappa`: channel-length modulation shaping.
- `xj`: junction depth.

Use existing uppercase parameter-map conventions and aliases only where ngspice accepts them. Keep field names MOS3-specific when the meaning differs from BSIM-style parameters already present in the generic `Mosfet` struct.

### DC Evaluator

Add a native `level3_operating_point` evaluator that returns `(id, region, gm, gds, gmb)` in the same terminal-orientation convention as the existing level-1/2/6 evaluators.

The evaluator should port the ngspice 46 `mos3load.c` channel-current and derivative equations into a small, testable Rust module. It should preserve ngspice's normal/inverse mode handling, threshold calculation, mobility degradation, velocity saturation, DIBL, narrow-channel terms, fast surface-state subthreshold branch, channel-length modulation, and finite-derivative safeguards.

Junction currents and conductances should continue to use the existing MOSFET body-diode stamping path unless the current shared path is proven insufficient against MOS3 oracle data. If a mismatch is traced to MOS3-specific junction preprocessing, the MOS3 path should add the missing preprocessing explicitly rather than perturbing other MOS levels.

### Charge, AC, And Transient

MOS3 uses the Berkeley Meyer-style capacitance path in ngspice. RSpice already has MOS capacitance and transient companion infrastructure for legacy bulk MOS models, so the first implementation should reuse it where it matches ngspice's effective `von` and `vdsat` inputs.

If oracle AC or transient data show that the generic path is not equivalent, add MOS3-specific capacitance helpers behind the level-3 branch. Do not broaden the generic MOS capacitance behavior for all levels to force a MOS3 match.

### Introspection And Reporting

Operating-point report rows should continue to identify the device as a MOSFET while preserving level-specific metadata where available. Documentation and support tables should list Berkeley MOS3 beside MOS1/MOS2/MOS6 after oracle-backed tests pass.

## Data Flow

1. Netlist parsing stores model parameters in the existing model-parameter map.
2. `Engine::build_circuit` resolves the MOS model type and level.
3. `LEVEL=3` is recognized as native and builds a standard `Mosfet` with `level = 3`.
4. `Mosfet::apply_model_params` stores shared Berkeley parameters plus MOS3-specific fields.
5. DC, AC, transient residual, and report paths call `calculate_id`, `small_signal`, and capacitance helpers. Those dispatch to the MOS3 evaluator when `level == 3`.
6. Existing source/drain resistance, multiplier, temperature, and polarity plumbing remain outside the evaluator, as they do for other bulk MOS levels.

## Error Handling

- Invalid or non-finite MOS3 parameters should follow the existing MOSFET convention: ignore nonsensical optional values only where existing Berkeley levels already do, clamp only where ngspice clamps, and surface construction errors when a parameter makes the model impossible to evaluate safely.
- Numerical safeguards must avoid NaN/Inf stamps. A MOS3 evaluator that encounters an out-of-domain intermediate should return a finite, conservative value and log only if the existing MOSFET paths would log in the same class of situation.
- The simplified fallback warning is removed only for `LEVEL=3`. All other unsupported levels keep their current diagnostics.

## Validation

Add focused oracle tests before or alongside implementation:

- A DC operating-point deck with MOS3-only parameters that currently have no effect in RSpice, proving native MOS3 changes the result and matches ngspice.
- A DC sweep deck covering subthreshold, linear, saturation, inverse-mode, and PMOS polarity points.
- An AC deck that checks small-signal gain over frequency for a MOS3 common-source stage.
- A transient deck that checks charge/capacitance behavior away from source edges to avoid comparing timestep interpolation noise.
- A build-policy test proving `LEVEL=3` no longer uses the simplified fallback while unsupported levels retain their current policy.

Run the focused tests first, then run:

```powershell
cargo test -p rspice-core --test mos_level_policy native_levels_unaffected -- --nocapture
cargo test -p rspice-core --release --test ngspice_regression test_full_ngspice_suite_summary -- --nocapture
```

The release form is the required gate because debug builds make the ngspice suite unnecessarily slow and can turn build-profile timeouts into noise.

## Non-Goals

- This slice does not implement MOS9, HiSIM, HiSIM-HV, HICUM2, BSIM3v32, PSP, or other remaining model-family gaps.
- This slice does not replace RSpice's MOS infrastructure with a neospice-style UCB shim.
- This slice does not change the global simplified-MOS fallback policy except for promoting `LEVEL=3` to native.
