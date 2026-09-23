# Reliability model packs

In Simulation Studio, choose **Reliability & aging**, load a JSON model pack,
assign its model IDs to circuit devices, and define the ordered mission phases.
The project saves the full pack. Imported calibration must match the device's
process, geometry, compact model and fresh parameter baseline.

A pack declares `schema_version: 1`, `id`, `process`, `qualification`, `source`,
`license`, `characterization`, and a `models` array. Qualification is the data
provider's claim: `public_reference`, `user_characterized`, or
`foundry_qualified`. It is not verified by the simulator. Each model supplies
its `id`, `mechanism`, `applicability`, `validity`, and `law`.

The validity object has inclusive `{ "min": ..., "max": ... }` ranges for
signed `gate_source_v`, signed `drain_source_v`, `temperature_k` in Kelvin,
and absolute `current_density_a_per_m2`. Stress outside these ranges is rejected.
`max_equivalent_seconds` bounds reference exposure for irreversible laws and
total elapsed time, including recovery, for a two-state table.

## Laws

`equivalent_time_power` accumulates irreversible reference exposure. Its
`reference_time_s`, `reference_gate_magnitude_v`,
`reference_drain_magnitude_v`, `reference_temperature_k`, `gate_polarity`,
`clock_gate_exponent`, `clock_drain_exponent`,
`clock_activation_energy_ev`, and `time_exponent` define:

```
acceleration = (abs(Vgs)/Vg_ref)^clock_gate_exponent
             * (abs(Vds)/Vd_ref)^clock_drain_exponent
             * exp(Ea/kB * (1/T_ref - 1/T))
equivalent_seconds = integral(acceleration dt)
shift = scale_at_reference_time * (equivalent_seconds/reference_time_s)^time_exponent
```

Each entry in `parameters` names the exact compact-model `parameter`, its
`update` (`additive` or `relative`), and `scale_at_reference_time`. Clock
exponents are not interchangeable with exponents fitted directly to the shift.
This law does not recover. NBTI requires `gate_polarity: "negative"`.

`black_electromigration` uses `reference_lifetime_s`,
`reference_current_density_a_per_m2`, `reference_temperature_k`,
`current_exponent`, and `activation_energy_ev`. Its acceleration is
`(J/J_ref)^current_exponent * exp(Ea/kB*(1/T_ref - 1/T))`. Integrated reference
exposure divided by reference lifetime is consumed lifetime, not a failure
probability. The device assignment must supply a physical conductor area.

## Capture and recovery tables

`tabulated_two_state` is an NBTI kinetics adapter. Supply independently
characterized capture and emission rates for each trap population. It solves
`dp/dt = capture*(1-p) - emission*p` and carries occupancy through all mission
phases in order. The following **synthetic law fragment illustrates the format;
it is not process calibration**:

```json
{
  "kind": "tabulated_two_state",
  "table": {
    "gate_source_v": [-1.0, 0.0],
    "drain_source_v": [0.0],
    "temperature_k": [300.0],
    "interpolation": "linear",
    "traps": [{
      "id": "example-population",
      "initial_occupancy": 0.0,
      "capture_rates_per_s": [2.0, 0.0],
      "emission_rates_per_s": [0.0, 3.0],
      "parameters": [{
        "parameter": "VTO",
        "update": "additive",
        "shift_per_occupancy": -0.1
      }]
    }]
  }
}
```

- Axes must increase strictly and have endpoints equal to the corresponding
  validity bounds. A singleton axis permits only that exact stress coordinate.
- Rate arrays are rectangular: Vgs changes fastest, then Vds, then temperature.
  Each array has `number_of_Vgs * number_of_Vds * number_of_temperatures` values.
- Rates are finite inverse seconds. `linear` interpolation allows exact zeros.
  `logarithmic` interpolates log rates and requires strictly positive entries.
  Voltage interpolation is linear; temperature interpolation uses `1/T`, so
  logarithmic interpolation follows Arrhenius behavior between temperature knots.
- `initial_occupancy` is in `[0,1]` and describes the fresh compact-model baseline.
  A population's contribution is
  `(occupancy - initial_occupancy) * shift_per_occupancy`. The scale includes
  its density, quadrature weight, and coupling to the named compact parameter.
  Contributions to the same parameter sum; their update modes must agree.
- Additive updates use `aged = fresh + shift`; relative updates use
  `aged = fresh * (1 + shift)`. A relative shift at or below `-1` is rejected.
  Circuit re-simulation uses the model-family mappings below. An unknown or
  inactive parameter is never silently ignored.
- **Min stress V** suppresses capture below the gate magnitude threshold.
  Emission/recovery continues at the actual bias and temperature. Choose zero
  to use both table rates at every bias.
- Constant-stress steps are exact. For transient stress, each retained interval
  advances through its two endpoint rates for half the interval each. Reduce
  the stress time step to resolve changes in kinetics. A partial final interval
  uses interpolated terminal stress at its endpoint.
- A complete transient window repeats inside a mission phase; each new phase
  restarts that phase's waveform. Repeated missions preserve all phases' order.
  Repetition composes state transitions analytically, including very slow traps,
  rather than multiplying each phase's duration separately.
- Results retain each trap occupancy, parameter shifts, and fresh/aged circuit
  observations. Occupancy plots are dimensionless; CSV includes
  `trap_occupancy` records. Trapping history is labeled elapsed time, not an
  equivalent reference age. Worker transport and project saves retain the pack
  and primary stress evidence and reconstruct/check the same history.

Limits: 4 MiB per imported pack, 1,024 models, 1,024 populations per table,
131,072 total rate values per table (capture and emission combined), and
128 parameter couplings per population. No extrapolation is performed.

## Circuit parameter mappings

These mappings apply to both power-law and trapping fits. A parameter must have
an explicit numeric fresh value on the selected model card, including a resolved
expression. The instance keeps its geometry, geometry-bin selection, temperature
coefficients and instance modifiers. Other instances sharing that card stay fresh.

| Native family | Aging parameters |
| --- | --- |
| Classic MOS levels 1–3 | `VTO`, `KP`, `GAMMA`, `PHI` |
| BSIM3 (levels 8/49, or Xyce-selected level 9) | `VTH0`, `U0`, `VSAT`, `RDSW` |
| BSIM4 (levels 14/54), `RDSMOD=0` | `VTH0`, `U0`, `VSAT`, `RDSW`, `RDSWMIN` |
| BSIM4, `RDSMOD=1` | `VTH0`, `U0`, `VSAT`, `RDW`, `RSW`, `RDWMIN`, `RSWMIN` |

The simulator dialect still controls model availability and equations. Ngspice
level 9 is MOS9, so it does not use the BSIM3 adapter. A canonical `VTH0` aging
coordinate can read a fresh `VTHO` alias; explicit `VTH0` takes precedence.

Shifts apply to the nominal card coefficient before the native L/W/cross-term
binning, temperature dependence, and instance modifiers. A negative nominal
coefficient is not automatically invalid if its binned physical value is valid.
Threshold voltage is in volts and saturation velocity is in metres per second.
Resistance coordinates retain the selected model's width normalization.

BSIM mobility shifts retain the fresh instance's input-unit convention: the
native model interprets binned `U0 > 1` as cm²/(V·s), otherwise m²/(V·s).
That fresh convention also applies to additive shifts and their reported aged
values. The adapter re-encodes all mobility coefficients on the private aged
card when necessary, so an aging shift across 1 cannot accidentally change
mobility by a factor of 10,000. L/W/cross-term coefficients are converted
together, preserving their physical bin contributions. The fitted shift changes
the nominal coefficient. `MULU0` remains an instance modifier.

The native BSIM3/BSIM4 ports use circuit temperature, so their reliability stress
uses the mission phase temperature, including when the instance contains a
`DTEMP` value that the native equations do not use. Classic MOS retains its native
instance `TEMP`/`DTEMP` behavior. Stress remains based on the fresh circuit.

The kinetics and periodic-history equations are described in
[Gerhard Rzepa's thesis, section 4.5](https://www.iue.tuwien.ac.at/phd/rzepa/).
This adapter does not implement Comphy's electrostatics or turn its defect-energy
parameters directly into terminal-voltage rate tables. Rates, population weights,
and compact-parameter couplings must be calibrated together before using a pack
for device predictions.
