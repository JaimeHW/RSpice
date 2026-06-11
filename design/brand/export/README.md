# RSpice App-Icon Export Set

Rasterized from the final brand SVGs in `design/brand/` (viewBox 0 0 96 96) with
headless Chrome at exact target pixel sizes, transparent rounded corners preserved.
Sources are unmodified.

## PNG provenance

| File | Source SVG | Size | Notes |
| --- | --- | --- | --- |
| `icon-512.png` | `run-icon.svg` | 512x512 | standard cut (yellow tile, black mark) |
| `icon-256.png` | `run-icon.svg` | 256x256 | standard cut |
| `icon-128.png` | `run-icon.svg` | 128x128 | standard cut |
| `icon-64.png` | `run-icon.svg` | 64x64 | standard cut |
| `icon-48.png` | `run-icon.svg` | 48x48 | standard cut |
| `icon-32.png` | `run-icon.svg` | 32x32 | standard cut |
| `icon-16.png` | `run-icon-16.svg` | 16x16 | **dedicated small cut** — heavier strokes (10.5 vs 8), wider corner radius, nudged lead geometry for legibility at tiny sizes |
| `icon-dark-512.png` | `run-icon-dark.svg` | 512x512 | dark bezel tile, yellow mark |
| `icon-dark-256.png` | `run-icon-dark.svg` | 256x256 | dark bezel tile, yellow mark |

## rspice.ico

Multi-size Windows icon containing 256, 128, 64, 48, 32, and 16 px entries.
Each entry was embedded from the individually rendered PNG above — **the 16 px
entry uses the dedicated small cut (`run-icon-16.svg`), not a downscale of the
standard cut.** Verified: all six sizes present, every embedded frame is
pixel-identical to its source PNG, and all corner pixels are alpha=0.
