# Application crate refactor baseline

This is the first evidence snapshot for [the implementation plan](app-crate-refactor-plan.md). It records the state *before* production source is changed; R00 remains open until the inventories and platform measurements in the plan are complete. Raw, host-specific results live under the ignored `target/app-refactor/43d4d30c9/` directory.

## Reproduction identity

| Item | Value |
| --- | --- |
| Base commit | `43d4d30c97afe83fa987e96ea25ef38e6f7bf743` (`origin/main` at checkout) |
| Workspace manifest Git blob | `288f91ffe9e84609a8f4756af53f5ced26fa14fb` |
| UI crate Git tree | `05c93c0117b764b988e73a69fea8f041043843af` |
| CI workflow Git blob | `c223b4dcade2c6947599d50d6c53ef1b2c875069` |
| Lockfile Git blob | `f608b3fbf5f45d3b6886f30951de898a13aee0b2` |
| Lockfile SHA-256 | `1CA801E962EDB674268D75AB77B1D604C2A9F796E9F525FE3E3548F2967918DA` |
| Host | `x86_64-pc-windows-msvc` |
| Rust / Cargo | `rustc 1.94.0 (4a4ef493e 2026-03-02)`; `cargo 1.94.0 (85eff7c80 2026-01-15)` |
| Toolchain target | `wasm32-unknown-unknown` installed |
| Checkout | isolated managed worktree, branch `codex/app-crate-refactor`; base tree clean |

The workspace has 69 packages. `rspice-ui` contains 1,678 Rust source files under `src/`, one library, a desktop binary, a feature-gated worker binary, and five integration-test targets. Its manifest records 90 dependency entries across common and target sections. No other workspace package directly depends on `rspice-ui`. The normal/build dependency trees were captured with the commands below for the native app, browser app, and `browser-worker` feature. The tree files preserve target-specific dependency resolution, including repeated subtrees; their 1,347, 979, and 989 printed lines are **not** unique-package counts or size metrics.

```powershell
cargo tree -p rspice-ui --locked --target x86_64-pc-windows-msvc -e normal,build --prefix none
cargo tree -p rspice-ui --locked --target wasm32-unknown-unknown -e normal,build --prefix none
cargo tree -p rspice-ui --locked --target wasm32-unknown-unknown --features browser-worker -e normal,build --prefix none
```

## Existing architecture gate

The unchanged `tests/module_layering.rs` was run as a standalone Rust test with `CARGO_MANIFEST_DIR` set to the UI crate. Result: **6 passed, 5 failed**. This is a pre-existing failure at the pinned base, not a refactor regression.

| Check | Baseline failure |
| --- | --- |
| Layer order | `io → simulation` 15/13; `services → simulation` 32/8; `simulation → workbench` 44/28; `state → io` 7/5; `state → services` 46/9; `state → simulation` 19/9 |
| Lint suppressions | 91, ceiling 54 |
| Public modules | Three nested `pub mod` declarations, ceiling zero |
| Module descriptions | 27 `.rs` files lack a `//!` header |
| New oversized files | Seven files above 2,500 lines |
| Previously oversized files that grew | Five files above their individual ceilings |

The full file list and exact assertion output are in `target/app-refactor/43d4d30c9/module_layering.log`. The passing checks cover module ownership/layer declarations, workbench layering, source encoding, sibling-module layout, and the mutable-app-access ratchet. R01 must restore the failed guards through ownership repair, without raising ceilings to mask growth.

## Contract inventory started

The current browser worker has explicit `wasm_bindgen` export names in `src/worker_main.rs`; the rename must preserve those JavaScript names independently of the Cargo package name. Persisted schema examples already found include visualization documents at version 7 with a legacy default reader, hardcopy source sets and print mappings at version 1, hardcopy setup stores at version 1, and prepared hardcopy worker snapshots at version 8. These readers are not candidates for deletion simply because they are named legacy. R00 still needs the full schema, digest, browser-key, delivery-name, and installer-path register before R02.

The initial test inventory includes `browser_clock`, `browser_worker_transport`, `hierarchy_guards`, `module_layering`, and `simulation_configuration_contract` integration targets, plus unit tests inside the 1,678 source files. `.github/CI.md` documents the broader native, browser, mobile, generated-model, and release qualification lanes. Full test and timing baselines remain to be collected. The first native `cargo check -p rspice-ui --locked` is in progress at the time of this snapshot; its result belongs in a follow-up evidence update.

## Legacy-removal register

No source has been deleted yet. A removal entry must cite the symbol/path, search across ordinary/feature/target consumers, serialized-data compatibility decision, and verification. The migration readers above are explicitly retained pending a support-policy decision. A `legacy` name alone does not prove dead code.
