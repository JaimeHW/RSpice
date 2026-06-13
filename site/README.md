# site/ — the deployed rspice.app pages

This tree IS the public site (Cloudflare Pages, production branch
`cf-pages`). It deploys **verbatim** — no transform layer. Edit a page
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

Deploying: `python3 tools/deploy/deploy.py` (Windows: `py tools\deploy\deploy.py`)
— git only, no `gh` required: it pushes your branch and a new `site-vN` tag, and
the tag push is what triggers the workflow. It prints the Actions URL to watch.
By hand: push a `site-v*` tag (or `gh workflow run deploy-site` if you use the CLI).
The workflow builds both wasm bundles from the pushed ref with
`tools/deploy/build_site.py`, assembles `_site/`, refuses to publish unless the
bundles pass the magic/export gates and the playground completes a live headless
solve, then force-publishes `_site` to `cf-pages`, which Cloudflare picks up.
Local dry-run: `python3 tools/deploy/build_site.py` (add `--skip-headless`
without a local Chrome). Rollback: re-run the workflow on an older ref.

If wasm threads ever land (SharedArrayBuffer), add a `_headers` file
here with COOP/COEP — Cloudflare Pages serves it natively.
