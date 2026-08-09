# RSpice device support and release matrix

This document is the release contract between the simulation engine, schematic
editor, symbol library, persistence format, netlister, and supported targets. It
describes executable product coverage; it does not replace compact-model
numerical validation against independent reference data.

## Authoritative inventories

| Inventory | Authority | Current contract | GUI contract | Symbol contract |
| :--- | :--- | :--- | :--- | :--- |
| Native, synthesized, structural, and legacy XSPICE schematic kinds | `ComponentType::ALL` and `ComponentType::descriptor()` | 74 stable device descriptors, including 20 legacy XSPICE kinds | Every kind is placeable and uses the same descriptor for properties, persistence, hierarchy, DRC, netlisting, preview, export, and hardcopy | Every directly placeable kind has a loadable embedded SVG. `CellInstance` intentionally obtains its symbol from its bound library cell or the generated cell fallback. |
| Additional built-in XSPICE devices | `CodeModelRegistry` plus `ENGINE_ONLY_XSPICE_DEVICES` | 64 canonical catalog entries; together with the 20 legacy kinds they classify all 113 registered canonical and alias names | Every canonical entry is placeable. Registry port types, directions, nullable ports, vector limits, and parameter schemas are frozen into the instance and revalidated before use. | Every catalog descriptor names a loadable embedded SVG with compatible anchors. Aliases share their canonical device's symbol and placement contract. |
| Generated Verilog-A built-ins | `rspice_veriloga_runtime::BuiltinModelDescriptor` and `rspice_veriloga_models::BUILTIN_DESCRIPTORS` | ABI 1, manifest schema 6, 42 generated models | All 42 are exposed when the UI is built with `generated-veriloga-catalog`, which is the release feature. Exact terminals and source-declared parameters drive placement, properties, persistence, hierarchy, DRC, and `X`-line netlisting. | A deterministic cell symbol is generated from the exact terminal count and direction metadata. A unique SVG is not required. A family SVG may only replace it when its anchors exactly match that model's terminal contract. |
| User/project Verilog-A | Verilog-A workspace compile result and bound library-cell interface | Compiled per project rather than added to the built-in enum | Treat as a versioned library cell with an immutable compiled interface and source receipt. Do not create a new `ComponentType` for each module. | Use the library-provided symbol when present; otherwise generate the same exact-pin cell fallback used by built-ins. |

The UI must fail closed if a stable id, executable registry entry, schema
signature, terminal contract, parameter contract, or symbol asset cannot be
resolved. It must never silently substitute a generic two-terminal element or
drop unsupported parameters.

## Generated Verilog-A built-ins

The shipped generated catalog contains these 42 descriptor identities:

- `DIODE_CMC`, `EPFL_HEMT_10a`, `JUNCAP200`, `PSP104TVA`, `PSP104VA`,
  `PSPNQS104VA`
- `angelov`, `angelov_gan`, `asmesd`, `asmesd_dio`, `asmhemt`
- `bjt505_va`, `bjt505t_va`, `bjtd505_va`, `bjtd505t_va`
- `bsimbulk`, `bsimcmg_va`, `bsimimg`, `bsimsoi__18c250bc`,
  `bsimsoi__e2aff994`, `bsimsoi_va`
- `ekv_va`, `hicumL0va`, `hicumL2va`, `hisimhv_n4_va`, `hisimhv_n5_va`,
  `hisimhv_va`
- `hisimsoi_va__5be18005`, `hisimsoi_va__242bc21d`,
  `hisimsoi_va__38074d06`, `hisimsotb_va`
- `l_utsoi__832ce87d`, `l_utsoi__485e0ac9`, `mosvar`, `mvsg_cmc`
- `r2_cmc`, `r2_et_cmc`, `r3_cmc`, `vbic13`, `vbic13_3t_et`, `vbic13_4t`,
  `vbic_4T_et_cf`

Generated models are catalog cells, not hard-coded schematic enum variants.
The generator emits terminal, internal-node, parameter, source-identity, and
checkpoint-identity metadata beside the executable model. The registry and GUI
consume the same descriptors. An instance is emitted by module name as an
`X` device with descriptor terminal order; it is not mapped to an `M`, `Q`, or
other native level selector.

Only parameters declared by the Verilog-A source are exposed or netlisted.
In particular, RSpice must not synthesize an `m` multiplicity parameter. A
generated model supports `m` only when that exact model declares and implements
it.

## Symbol policy

Embedded SVG assets live only in:

`crates/rspice-ui/assets/component_symbols/`

Release tests load each mapped SVG, validate its schema and anchors, and cover
every directly placeable legacy and XSPICE descriptor. Generated and custom
Verilog-A cells use an exact-pin generated symbol, so devices with different
terminal counts never share incompatible fixed SVG anchors.

The symbol renderer used by the canvas is also used by placement previews,
export, and hardcopy. New rendering paths must not independently reinterpret
SVG geometry or invent terminal locations.

## Target matrix

| Target | Release configuration | Catalog status |
| :--- | :--- | :--- |
| Windows, Linux, macOS desktop | `rspice-ui --features generated-veriloga-catalog` | All 42 generated models compile into the GUI catalog; native and XSPICE catalogs are always present. |
| Browser / WebAssembly | `wasm32-unknown-unknown`, UI release feature enabled | All 42 generated models and the complete GUI catalog compile without a native JIT dependency. |
| Android ARM64 | `rspice-core --no-default-features --features veriloga-builtins` | All 42 generated model crates compile with the portable hash implementation. |
| iOS ARM64 | `rspice-core --no-default-features --features veriloga-builtins` | All 42 generated model crates compile with the portable hash implementation. |

## Required release gates

Run from the workspace root. Tools belong in the top-level `tools/` directory;
do not add crate-local `examples/` programs for release audits.

```powershell
cargo run --offline -p rspice-veriloga --profile generator --bin rspice-veriloga-gen -- check-builtins
cargo run --offline -p rspice-veriloga-catalog-dump -- --validate-only
cargo run --offline -p rspice-xspice-catalog-dump
cargo test --offline -p rspice-ui --features generated-veriloga-catalog --lib generated_veriloga --no-fail-fast
cargo test --offline -p rspice-ui --lib embedded_library_loads_and_covers_mapped_types --no-fail-fast
cargo check --offline -p rspice-ui --features generated-veriloga-catalog --lib
cargo check --offline -p rspice-ui --features generated-veriloga-catalog --target wasm32-unknown-unknown
cargo check --offline -p rspice-core --target aarch64-linux-android --no-default-features --features veriloga-builtins
cargo check --offline -p rspice-core --target aarch64-apple-ios --no-default-features --features veriloga-builtins
```

The XSPICE dump is intentionally verbose: successful execution proves that its
machine-readable registry can enumerate the full port and parameter contract.
CI additionally tests that every registry name is assigned to exactly one
canonical GUI entry or alias disposition, every catalog binding round-trips,
custom vector widths remain exact, and tampered bindings are rejected.

## Updating a device family

1. Change the engine registry or canonical Verilog-A source first.
2. For Verilog-A, regenerate all affected built-ins with the generator. Never
   edit checked-in generated model state by hand.
3. Add or update stable catalog metadata. Never reuse a released stable id for
   a different device.
4. Add an SVG under the component-symbol asset directory only when a fixed
   symbol is appropriate and its terminal anchors exactly match. Otherwise use
   the generated exact-pin cell symbol.
5. Verify placement, property editing, serialization round-trip, replacement,
   hierarchy, DRC, flat netlisting, engine construction, preview, export, and
   hardcopy from the same descriptor.
6. Run every applicable release gate above. A target or feature that was not
   built is not qualified by a successful desktop build.

## Qualification boundary

Catalog, schema, symbol, round-trip, netlist, construction, and cross-target
checks prove that a device is reachable and represented consistently. They do
not by themselves establish semiconductor-model accuracy. Golden operating
point, DC, AC, transient, noise, temperature, corner, convergence, and stress
comparisons against independent references remain required per compact-model
version before RSpice makes numerical-equivalence claims. Existing project
documentation specifically identifies generated ASM-HEMT and MVSG-CMC as not
yet oracle-qualified; their catalog availability must not be presented as
completed numerical certification.
