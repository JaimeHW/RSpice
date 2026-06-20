# Native MOS3 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace RSpice's `LEVEL=3` simplified MOS fallback with a native ngspice-46-compatible Berkeley MOS3 evaluator and oracle-backed integration tests.

**Architecture:** Add a focused MOS3 module under the existing `Mosfet` implementation and dispatch `LEVEL=3` through the normal MOS stamping, body-junction, series-resistance, AC, and transient plumbing. Use ngspice 46 `mos3load.c`, `mos3temp.c`, and `mos3mpar.c` as the equation source of truth, with neospice's Rust MOS3 port only as a secondary readability aid.

**Tech Stack:** Rust, `rspice-core`, ngspice 46 reference source, Cargo tests, PowerShell.

---

## Source References

- RSpice MOS entry point: `crates/rspice-core/src/device/mosfet/mosfet.rs`
- RSpice MOS current dispatch: `crates/rspice-core/src/device/mosfet/mosfet/current.rs`
- RSpice MOS construction and model parsing: `crates/rspice-core/src/device/mosfet/mosfet/construction.rs`
- RSpice capacitance routing: `crates/rspice-core/src/device/mosfet/mosfet/capacitance.rs`
- RSpice MOS builder policy: `crates/rspice-core/src/engine/builder.rs`
- Existing level-policy tests: `crates/rspice-core/tests/mos_level_policy.rs`
- ngspice MOS3 model parameters: `C:/Users/James/Desktop/ngspice-46-release/ngspice-46/src/spicelib/devices/mos3/mos3mpar.c`
- ngspice MOS3 temperature setup: `C:/Users/James/Desktop/ngspice-46-release/ngspice-46/src/spicelib/devices/mos3/mos3temp.c`
- ngspice MOS3 DC and Meyer equations: `C:/Users/James/Desktop/ngspice-46-release/ngspice-46/src/spicelib/devices/mos3/mos3load.c`
- neospice secondary reference: `C:/Users/James/Desktop/neospice-main/src/devices/mos3`

## Ground Rules

- Keep every change scoped to native MOS3 support and tests. Do not alter unsupported MOS9, HiSIM, HICUM2, BSIM3v32, PSP, or general fallback policy.
- Do not stage unrelated current workspace changes. When committing, stage only the paths named by the current task.
- Preserve the existing MOSFET stamping API: MOS3 should produce the same `MosfetOperatingPoint` shape used by level 1, level 2, level 6, and legacy BSIM paths.
- Treat ngspice 46 output as authoritative whenever neospice and ngspice disagree.
- Run the full ngspice regression suite with `--release`.

## Current Audit Notes

- 2026-06-19 audit: current code routes `LEVEL=3` through native MOS3, with focused ngspice 46 and Xyce 7.10 tests in `crates/rspice-core/tests/mos3_native.rs`.
- Additional post-audit MOS3 oracle coverage now checks `.options temp`, source/drain series resistance with instance `M`, body-junction current, Xyce PMOS polarity, and Xyce inverse-mode DC points.
- The original `test_ngspice_mos_suite` command below is stale: `crates/rspice-core/tests/ngspice_regression.rs` exposes `test_ngspice_mos6_suite`, while MOS3 oracle coverage is in `mos3_native.rs`.
- MOS3 charge/capacitance behavior is intentionally handled by the shared Berkeley Meyer helpers using MOS3 effective geometry plus MOS3 `von`/`vdsat`; `Mos3State` now only carries the DC/small-signal state it owns.

## Task 1: Add Red MOS3 Oracle Tests

- [ ] Create `crates/rspice-core/tests/mos3_native.rs`.
- [ ] Add DC operating-point cases covering:
  - [ ] NMOS normal mode with nonzero `ETA`, `THETA`, `KAPPA`, `NFS`, `VMAX`, `XJ`, and `DELTA`.
  - [ ] PMOS polarity using the same parameter family.
  - [ ] Inverse mode where drain/source terminal voltages are swapped.
  - [ ] A short-channel case where `VMAX` changes current relative to the same deck with `VMAX=0`.
- [ ] Add AC/transient-facing cases that check the MOS3 model-space onset and saturation values are not the simplified fallback values:
  - [ ] `von` should reflect the MOS3 narrow-width and fast-surface-state terms.
  - [ ] `vdsat` should reflect the MOS3 velocity-saturation and channel-length-modulation terms.
- [ ] Generate oracle values with the local ngspice 46 reference binary or, if no binary is present, by building ngspice 46 once and running the same decks.
- [ ] Store oracle constants directly in `mos3_native.rs` with a comment naming ngspice 46 as the source.

Use these decks as the first oracle fixtures:

```spice
* mos3_nmos_op
M1 d g s b MOD W=12U L=1.2U
VDS d 0 2.5
VGS g 0 3.0
VBS b 0 -0.6
VS  s 0 0
.MODEL MOD NMOS LEVEL=3 VTO=0.72 KP=55U GAMMA=0.62 PHI=0.68
+ TOX=22N LD=0.08U ETA=0.18 THETA=0.05 KAPPA=0.35
+ NFS=8E11 VMAX=8E4 XJ=0.18U DELTA=0.22
.OP
.END
```

```spice
* mos3_pmos_op
M1 d g s b MOD W=18U L=1.5U
VSD s 0 3.0
VSG g 0 0.6
VSB b 0 3.3
VD  d 0 0.2
.MODEL MOD PMOS LEVEL=3 VTO=-0.82 KP=32U GAMMA=0.55 PHI=0.7
+ TOX=24N LD=0.06U ETA=0.12 THETA=0.04 KAPPA=0.28
+ NFS=5E11 VMAX=7E4 XJ=0.2U DELTA=0.18
.OP
.END
```

```spice
* mos3_inverse_mode
M1 d g s b MOD W=10U L=1.0U
VD d 0 0.15
VG g 0 2.4
VS s 0 1.8
VB b 0 -0.2
.MODEL MOD NMOS LEVEL=3 VTO=0.65 KP=70U GAMMA=0.5 PHI=0.65
+ TOX=20N LD=0.05U ETA=0.2 THETA=0.08 KAPPA=0.4
+ NFS=6E11 VMAX=6E4 XJ=0.15U DELTA=0.25
.OP
.END
```

Test skeleton, following the existing `bsim3_native.rs` and `jfet2_native.rs` pattern:

```rust
use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

fn engine() -> Engine {
    Engine::new(SimulationConfig::default())
}

fn assert_close(actual: f64, expected: f64, rel: f64, abs: f64) {
    let diff = (actual - expected).abs();
    let tol = abs.max(rel * expected.abs().max(actual.abs()));
    assert!(
        diff <= tol,
        "actual={actual:.12e} expected={expected:.12e} diff={diff:.12e} tol={tol:.12e}"
    );
}

#[test]
fn mos3_accepts_native_parameter_family() {
    let deck = r#"
M1 d g 0 b MOD W=12U L=1.2U
VDS d 0 2.5
VGS g 0 3.0
VBS b 0 -0.6
.MODEL MOD NMOS LEVEL=3 VTO=0.72 KP=55U GAMMA=0.62 PHI=0.68
+ TOX=22N LD=0.08U ETA=0.18 THETA=0.05 KAPPA=0.35
+ NFS=8E11 VMAX=8E4 XJ=0.18U DELTA=0.22
.OP
.END
"#;
    let netlist = Netlist::parse(deck).expect("deck parses");
    let (_, report) = engine()
        .run_dc_op_with_report(&netlist)
        .expect("op converges");
    let entry = report
        .entries
        .iter()
        .find(|entry| entry.name.eq_ignore_ascii_case("m1"))
        .expect("m1 op entry");
    assert_ne!(
        entry.device_kind, "MOS simplified",
        "native MOS3 must not report the simplified fallback"
    );
}
```

- [ ] Run the red test command and confirm it fails because `LEVEL=3` still uses the simplified path or still emits the fallback warning:

```powershell
cargo test -p rspice-core --test mos3_native -- --nocapture
```

Commit after Task 1:

```powershell
git add crates/rspice-core/tests/mos3_native.rs
git commit -m "test: add mos3 native oracle coverage"
```

## Task 2: Add MOS3 Model Fields and Parser Support

- [ ] Add `mod mos3;` to `crates/rspice-core/src/device/mosfet/mosfet.rs` in the same style as `mos2`.
- [ ] Add a new file `crates/rspice-core/src/device/mosfet/mosfet/mos3.rs`.
- [ ] Add a private state struct in `mos3.rs`:

```rust
#[derive(Clone, Copy, Debug, Default)]
pub(super) struct Mos3State {
    pub ids: f64,
    pub gm: f64,
    pub gds: f64,
    pub gmb: f64,
    pub von: f64,
    pub vdsat: f64,
    pub qgs: f64,
    pub qgd: f64,
    pub qgb: f64,
    pub cgs: f64,
    pub cgd: f64,
    pub cgb: f64,
}
```

- [ ] Add MOS3-specific model fields to `Mosfet` in `mosfet.rs`:

```rust
pub mos3_eta: f64,
pub mos3_theta: f64,
pub mos3_kappa: f64,
pub mos3_delta: f64,
pub mos3_fast_surface_state_density: f64,
pub mos3_max_drift_velocity: f64,
pub mos3_junction_depth: f64,
pub mos3_narrow_factor: f64,
pub mos3_length_adjust: f64,
pub mos3_width_adjust: f64,
pub mos3_width_narrow: f64,
```

- [ ] Initialize those fields in `Mosfet::new` and level-specific constructors:

```rust
mos3_eta: 0.0,
mos3_theta: 0.0,
mos3_kappa: 0.2,
mos3_delta: 0.0,
mos3_fast_surface_state_density: 0.0,
mos3_max_drift_velocity: 0.0,
mos3_junction_depth: 0.0,
mos3_narrow_factor: 0.0,
mos3_length_adjust: 0.0,
mos3_width_adjust: 0.0,
mos3_width_narrow: 0.0,
```

- [ ] In `construction.rs`, route level 3 to MOS3 defaults when `.MODEL ... LEVEL=3` is parsed:

```rust
fn with_mos3_defaults(mut self) -> Self {
    self.level = 3;
    self.mos3_eta = 0.0;
    self.mos3_theta = 0.0;
    self.mos3_kappa = 0.2;
    self.mos3_delta = 0.0;
    self.mos3_fast_surface_state_density = 0.0;
    self.mos3_max_drift_velocity = 0.0;
    self.mos3_junction_depth = 0.0;
    self.mos3_narrow_factor = 0.0;
    self
}
```

- [ ] Parse these model parameters into MOS3 fields for level 3:

| SPICE parameter | RSpice field |
| --- | --- |
| `ETA` | `mos3_eta` |
| `THETA` | `mos3_theta` |
| `KAPPA` | `mos3_kappa` |
| `DELTA` | `mos3_delta` |
| `NFS` | `mos3_fast_surface_state_density` |
| `VMAX` | `mos3_max_drift_velocity` |
| `XJ` | `mos3_junction_depth` |
| `XL` | `mos3_length_adjust` |
| `XW` | `mos3_width_adjust` |
| `WD` | `mos3_width_narrow` |

- [ ] Compute `mos3_narrow_factor` using ngspice `mos3temp.c` line 113:

```rust
const EPSSIL: f64 = 11.7 * 8.854_214_871e-12;
let oxide_cap_factor = self.cox;
self.mos3_narrow_factor = if oxide_cap_factor > 0.0 {
    self.mos3_delta * 0.5 * std::f64::consts::PI * EPSSIL / oxide_cap_factor
} else {
    0.0
};
```

- [ ] Add focused construction unit tests in `construction.rs` or `mos3.rs` for parameter parsing and narrow-factor calculation.
- [ ] Run:

```powershell
cargo test -p rspice-core mos3 -- --nocapture
```

Commit after Task 2:

```powershell
git add crates/rspice-core/src/device/mosfet/mosfet.rs `
        crates/rspice-core/src/device/mosfet/mosfet/construction.rs `
        crates/rspice-core/src/device/mosfet/mosfet/mos3.rs
git commit -m "feat: parse mos3 model parameters"
```

## Task 3: Route LEVEL=3 as a Native Model

- [ ] In `crates/rspice-core/src/engine/builder.rs`, change the native-level predicate:

```rust
fn native_bulk_mos_level(level: usize) -> bool {
    matches!(level, 1 | 2 | 3 | 4 | 5 | 6)
}
```

- [ ] Remove the `LEVEL=3` simplified-fallback warning branch that names `THETA`, `ETA`, `KAPPA`, `NFS`, `VMAX`, `XJ`, and `DELTA` as ignored.
- [ ] Preserve warning behavior for unsupported levels not listed in `native_bulk_mos_level`.
- [ ] In `current.rs`, dispatch level 3 before the simplified path:

```rust
if self.level == 3 {
    return self.calculate_id_mos3(vgs, vds, vbs);
}
```

- [ ] Change the simplified fallback unit test currently using level 3 so it uses an unsupported level such as 7:

```rust
let mos = Mosfet::new("M1", "D", "G", "S", "B").with_level(7);
```

- [ ] Update `crates/rspice-core/tests/mos_level_policy.rs`:
  - [ ] Rename `level3_runs_with_warning_not_error` to `level3_runs_without_fallback_warning`.
  - [ ] Assert `LEVEL=3` does not produce the old simplified warning.
  - [ ] Keep unsupported-level warning coverage on a different level.
- [ ] Run:

```powershell
cargo test -p rspice-core --test mos_level_policy -- --nocapture
cargo test -p rspice-core mosfet::mosfet::current -- --nocapture
```

Commit after Task 3:

```powershell
git add crates/rspice-core/src/engine/builder.rs `
        crates/rspice-core/src/device/mosfet/mosfet/current.rs `
        crates/rspice-core/tests/mos_level_policy.rs
git commit -m "feat: route mos level 3 as native"
```

## Task 4: Implement MOS3 DC Operating-Point Equations

- [ ] Port the ngspice 46 MOS3 evaluator from `mos3load.c` into `mos3.rs` using scalar helper functions first.
- [ ] Implement effective geometry helpers:

```rust
impl Mosfet {
    pub(super) fn mos3_effective_length(&self) -> f64 {
        (self.l - 2.0 * self.ld + self.mos3_length_adjust).max(1e-12)
    }

    pub(super) fn mos3_effective_width(&self) -> f64 {
        (self.w - 2.0 * self.mos3_width_narrow + self.mos3_width_adjust).max(1e-12)
    }
}
```

- [ ] Implement a normalized polarity wrapper so the core equations operate on NMOS-like voltages and convert signs at the boundary:

```rust
let type_sign = if self.mtype < 0.0 { -1.0 } else { 1.0 };
let vgs_n = type_sign * vgs;
let vds_n = type_sign * vds;
let vbs_n = type_sign * vbs;
```

- [ ] Implement inverse-mode handling by swapping drain/source in normalized space when `vds_n < 0.0`, matching the existing MOS1 and MOS2 approach.
- [ ] Implement MOS3 threshold/onset terms from `mos3load.c` lines around the `vbix`, `vth`, `von`, `NFS`, and `ETA` calculations:
  - [ ] Body-effect threshold with `GAMMA` and `PHI`.
  - [ ] Drain-induced threshold term from `ETA`.
  - [ ] Narrow-width threshold term from `mos3_narrow_factor`.
  - [ ] Fast-surface-state correction from `NFS`.
- [ ] Implement mobility degradation from `THETA`.
- [ ] Implement velocity saturation from `VMAX`.
- [ ] Implement channel-length modulation from `KAPPA` and `XJ`.
- [ ] Implement weak/subthreshold continuity from the exponential branch in `mos3load.c` lines around the `nfs` and `vgs - von` checks.
- [ ] Clamp all square-root arguments and denominators using the same defensive style already present in `mos2.rs` and `level6.rs`.
- [ ] Use finite differences initially for `gm`, `gds`, and `gmb`, with adaptive deltas matching the scale used in `current.rs`. This keeps the first native implementation correct against DC oracle tests while analytic derivative cleanup remains optional.
- [ ] Add an internal helper:

```rust
impl Mosfet {
    pub(super) fn mos3_state(&self, vgs: f64, vds: f64, vbs: f64) -> Mos3State {
        let base = self.mos3_bias_scalar(vgs, vds, vbs);
        let ids = base.ids;
        let von = base.von;
        let vdsat = base.vdsat;
        let gm = central_diff(|vg| self.mos3_bias_scalar(vg, vds, vbs).ids, vgs);
        let gds = central_diff(|vd| self.mos3_bias_scalar(vgs, vd, vbs).ids, vds);
        let gmb = central_diff(|vb| self.mos3_bias_scalar(vgs, vds, vb).ids, vbs);
        Mos3State { ids, gm, gds, gmb, von, vdsat, ..Mos3State::default() }
    }
}
```

- [ ] Hook `calculate_id_mos3`, `small_signal`, and `linearized_operating_point` into the new state helper.
- [ ] Run:

```powershell
cargo test -p rspice-core --test mos3_native -- --nocapture
cargo test -p rspice-core mos3 -- --nocapture
```

Commit after Task 4:

```powershell
git add crates/rspice-core/src/device/mosfet/mosfet/mos3.rs `
        crates/rspice-core/src/device/mosfet/mosfet/current.rs
git commit -m "feat: implement mos3 dc evaluator"
```

## Task 5: Wire MOS3 Capacitance, Onset, and Saturation State

- [ ] In `capacitance.rs`, route `classic_meyer_effective_length` through `mos3_effective_length()` for level 3.
- [ ] Route `model_space_onset_voltage` through `mos3_state(...).von` for level 3.
- [ ] Route `transient_capacitance_halves_at` through `mos3_state(...).vdsat` for level 3.
- [ ] In `mos3.rs`, compute Meyer charge and capacitance state using the same region rules as ngspice `DEVqmeyer` call in `mos3load.c`.
- [ ] Reuse existing `ClassicMeyerCapacitances` when its region semantics match ngspice. Add MOS3-specific preconditioning only for the `von` and `vdsat` inputs.
- [ ] Add tests to `mos3_native.rs` that compare level 3 transient/AC capacitance behavior against ngspice within a tolerance that covers integration-method differences but still rejects the old fallback.
- [ ] Run:

```powershell
cargo test -p rspice-core --test mos3_native -- --nocapture
cargo test -p rspice-core --test ngspice_regression test_ngspice_mos_suite -- --nocapture
```

Commit after Task 5:

```powershell
git add crates/rspice-core/src/device/mosfet/mosfet/capacitance.rs `
        crates/rspice-core/src/device/mosfet/mosfet/mos3.rs `
        crates/rspice-core/tests/mos3_native.rs
git commit -m "feat: wire mos3 capacitance state"
```

## Task 6: Update Documentation and Introspection

- [ ] Update `README.md` support tables so MOS3 / Berkeley level 3 is listed as native instead of approximated.
- [ ] Update `crates/rspice-core/README.md` in the same way.
- [ ] Update `docs/manual/02-netlists.md` so supported MOS model notes include `LEVEL=3`.
- [ ] Update `crates/rspice-core/src/circuit/introspection.rs` only if the public support metadata has a MOS-level list there.
- [ ] Ensure no user-facing documentation claims MOS3 is unsupported or simplified.
- [ ] Run:

```powershell
rg -n "LEVEL=3|MOS3|Berkeley level 3|simplified" README.md crates/rspice-core/README.md docs/manual/02-netlists.md crates/rspice-core/src
```

Commit after Task 6:

```powershell
git add README.md `
        crates/rspice-core/README.md `
        docs/manual/02-netlists.md `
        crates/rspice-core/src/circuit/introspection.rs
git commit -m "docs: document native mos3 support"
```

## Task 7: Full Verification

- [ ] Run the focused MOS3 tests:

```powershell
cargo test -p rspice-core --test mos3_native -- --nocapture
```

- [ ] Run the MOS level policy tests:

```powershell
cargo test -p rspice-core --test mos_level_policy -- --nocapture
```

- [ ] Run the MOS regression slice:

```powershell
cargo test -p rspice-core --test ngspice_regression test_ngspice_mos_suite -- --nocapture
```

- [ ] Run the full ngspice suite in release mode as requested:

```powershell
cargo test -p rspice-core --release --test ngspice_regression test_full_ngspice_suite_summary -- --nocapture
```

- [ ] Run the repository-level formatting and whitespace checks:

```powershell
cargo fmt --all -- --check
git diff --check
```

- [ ] If all tests pass, inspect staged MOS3 diffs:

```powershell
git diff --stat
git diff -- crates/rspice-core/src/device/mosfet/mosfet.rs `
           crates/rspice-core/src/device/mosfet/mosfet/current.rs `
           crates/rspice-core/src/device/mosfet/mosfet/construction.rs `
           crates/rspice-core/src/device/mosfet/mosfet/capacitance.rs `
           crates/rspice-core/src/device/mosfet/mosfet/mos3.rs `
           crates/rspice-core/src/engine/builder.rs `
           crates/rspice-core/tests/mos3_native.rs `
           crates/rspice-core/tests/mos_level_policy.rs
```

- [ ] Record the verification commands and outcomes in the final response.

Commit after Task 7:

```powershell
git add README.md `
        crates/rspice-core/README.md `
        docs/manual/02-netlists.md `
        crates/rspice-core/src/circuit/introspection.rs `
        crates/rspice-core/src/device/mosfet/mosfet.rs `
        crates/rspice-core/src/device/mosfet/mosfet/current.rs `
        crates/rspice-core/src/device/mosfet/mosfet/construction.rs `
        crates/rspice-core/src/device/mosfet/mosfet/capacitance.rs `
        crates/rspice-core/src/device/mosfet/mosfet/mos3.rs `
        crates/rspice-core/src/engine/builder.rs `
        crates/rspice-core/tests/mos3_native.rs `
        crates/rspice-core/tests/mos_level_policy.rs
git commit -m "feat: add native mos3 support"
```

## Expected Final State

- `.MODEL ... LEVEL=3` is a native RSpice model with no simplified-fallback warning.
- MOS3 model parameters `ETA`, `THETA`, `KAPPA`, `DELTA`, `NFS`, `VMAX`, `XJ`, `XL`, `XW`, and `WD` are parsed and affect simulation results.
- NMOS, PMOS, normal mode, inverse mode, DC, small-signal, AC-facing capacitance, transient-facing capacitance, body junctions, multiplier, and series resistances all continue through the existing MOSFET engine plumbing.
- Unsupported MOS levels still warn or error exactly as before.
- The release-mode full ngspice regression suite passes.
