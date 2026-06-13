# site/ — the deployed rspice.app pages

This tree IS the public site (Cloudflare Pages, production branch
`gh-pages`). It deploys **verbatim** — no transform layer. Edit a page
here, run the deploy workflow, and that exact byte content goes live.

- `index.html` / `interior.html` / `parity.html` / `docs.html` /
  `changelog.html` / `download.html` / `404.html` — marketing + docs pages.
- `ide/` — the browser IDE shell. `ide/pkg/` is **generated** (rspice-ui
  wasm bundle) by the deploy build; never committed here.
- `play/` — the engine playground shell. `play/pkg/` is generated
  (rspice-wasm bundle) likewise.
- `assets/` — fonts, og-card, shared css/js.

The mockups under `design/` (see `design/README.md` for the index) are
design explorations with a sandbox lifecycle — they are NOT the site.
When a design lands, port it here deliberately. `parity.html`'s design
source is `design/internal/rspice-parity-dashboard.html`; the two are kept
in sync by hand, with the oracle labels deliberately differing.

Deploying: `gh workflow run deploy-site` (or push a `site-v*` tag).
The workflow builds both wasm bundles from the tagged/dispatched ref,
assembles `_site/`, refuses to publish unless the bundles pass the
magic/export gates and the playground completes a live headless solve,
then force-publishes `_site` to `gh-pages`, which Cloudflare picks up.
Local dry-run: `bash tools/deploy/build_site.sh` (add `--skip-headless`
without a local Chrome). Rollback: re-run the workflow on an older ref.

If wasm threads ever land (SharedArrayBuffer), add a `_headers` file
here with COOP/COEP — Cloudflare Pages serves it natively.
