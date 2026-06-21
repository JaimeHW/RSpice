# Platform Support Matrix

RSpice targets desktop, browser, and smaller touch devices, but support is
tracked by evidence rather than marketing labels. This page is the source of
truth for what is automated, manually verified, experimental, or launch-only.

| Surface | Current support level | Evidence gate |
| --- | --- | --- |
| Native desktop: Windows | Supported for source builds and CI checks | `.github/workflows/ci.yml` runs `cargo check --workspace --exclude rspice-python --exclude rspice-wasm` and `cargo test -p rspice-core --lib` on `windows-latest` |
| Native desktop: Linux | Supported for source builds and primary CI | `.github/workflows/ci.yml` runs format, static CI tests, workspace check, clippy, fast tests, and core library tests on `ubuntu-latest` |
| Native desktop: macOS | Supported for source builds; release artifact workflow is not yet part of the gate | `.github/workflows/ci.yml` runs `cargo check -p rspice-cli -p rspice-ui` on `macos-latest` |
| Browser IDE | Experimental browser product surface; requires a WebGPU-capable current browser | CI runs `tools/ci/test_ide_worker.py`, `cargo check -p rspice-ui --target wasm32-unknown-unknown`, and deploy runs IDE worker source/static gates |
| WASM playground | Supported demo/playground subset for summary, `.op`, `.ac`, and `.tran` | CI runs `tools/ci/test_wasm_playground.py`; deploy builds `rspice-wasm`, checks the bundle, and live-loads `/play/` in headless Chrome |
| mobile/tablet browser use | Experimental until a repeatable device/browser matrix exists | Current evidence is responsive/static/browser smoke coverage only; do not treat iPad, Android, or phone layouts as fully supported release targets yet |
| Signed installers and packaged release artifacts | Launch-only, not current-source support | The download page describes the intended launch manifest; this repository does not yet gate DMG, MSI, AppImage, signing, notarization, or checksum publication |

## Policy

- Public claims must not imply a stronger platform guarantee than the matrix.
- Adding a platform to "supported" requires an automated gate or documented
  manual QA checklist with current evidence.
- Browser and mobile claims must distinguish the full IDE from the smaller WASM
  playground; the playground currently exposes a narrower OP/AC/TRAN subset.
- Release artifacts should stay described as launch work until the workflow
  builds, signs, publishes, and verifies them.
