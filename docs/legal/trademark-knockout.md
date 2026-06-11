# Preliminary Trademark Knockout Search: "RSpice"

**Date:** 2026-06-10
**Mark under review:** RSPICE (word mark) for a commercial circuit simulator — EDA software (Nice classes 9 software / 42 SaaS-design services)
**Secondary marks reviewed:** VOLTA (internal design-system name), "Run" logo (op-amp triangle styled as a play button)
**Method:** Free public web search and direct URL fetches only. No TESS/TMview database queries, no paid clearance services, no third parties contacted.

> **This is preliminary research, not legal advice.** It is a knockout-level screen by a non-attorney using only public web indexes. A registrability opinion requires a licensed trademark attorney and full database searches (see "What a real attorney search would add").

---

## Executive Summary

**Verdict: GO-WITH-CAUTION** for RSPICE as a commercial software mark in classes 9/42.

- **No live registered trademark "RSPICE" surfaced** in any public index checked (Justia site-search, uspto.report site-search, general web for EUIPO/TMview/WIPO mentions). The name appears legally unclaimed as a registered mark, subject to confirmation in TESS/TMview by an attorney.
- **"SPICE" is effectively generic in EDA.** The market has coexisted for decades on single-letter+SPICE names: PSpice (Cadence), HSPICE (Synopsys), LTspice (Analog Devices), QSPICE® (Qorvo, launched 2023 and registered), SmartSpice (Silvaco), T-Spice, IsSpice, ngspice, XSPICE. Qorvo's recent QSPICE® registration is a strong precedent that a new {letter}SPICE mark is registrable and can coexist with Cadence's PSpice.
- **The caution flags are practical, not legal:** (1) the crates.io package name `rspice` is already taken by a third-party *Rust circuit-simulation* crate (Oct 2025, AGPL) — a direct namespace/SEO collision in our exact niche; (2) a defunct 1990s commercial SPICE simulator named RSPICE (Ron Kielkowski / RCG Research, bundled with McGraw-Hill's *Inside SPICE*) creates prior-use history in the identical goods class, though no live mark or business was found; (3) an academic 1999 "RSPICE" timing-simulator paper and a small GitHub R-wrapper dilute search results.
- None of the collisions found is a going commercial concern using RSPICE as a brand today. The two active code projects (R package, Rust crate) are tiny, non-commercial, and do not assert trademark rights — but they sit on the GitHub/crates.io names and will share search results with us.
- **Recommended posture:** proceed with the name; file a US intent-to-use application in classes 9 and 42 promptly after attorney confirmation; buy rspice.com (listed at $695 on HugeDomains — cheap by aftermarket standards); resolve the crates.io name question before public launch.

---

## 1. Software / EDA Collisions

### 1a. QianqianShan/RSpice — R package wrapping ngspice (the known collision)

- Repo: https://github.com/QianqianShan/RSpice (bundle: RSpiceBundle on the same account); project page: https://qianqianshan.com/projects/rspice/
- What it is: an R interface to the ngspice shared library so circuit simulations can be driven from R for statistical analysis. Author is a statistics PhD (Iowa State) now at Amazon — a side/academic project, not a company.
- **Not on CRAN** (searches of CRAN's package index return no `rspice`/`RSpice`), so it has no official package-registry claim — GitHub only.
- **Severity: LOW as a trademark matter, MODERATE as a search/SEO matter.** It is exactly our niche (SPICE simulation tooling), so it will appear alongside us in search results and could cause user confusion ("is RSpice the R thing?"). But it is free, unmaintained-looking, never commercialized, and "RSpice" there transparently means "R + SPICE" — descriptive use, no brand-building, no registration. Trademark rights in the US require use in commerce as a source identifier; nothing found suggests that.

### 1b. aerkiaga/rspice — Rust crate "Pure-Rust circuit simulation backend" (NEW, most directly relevant)

- crates.io: https://crates.io/crates/rspice — v0.1.0 published **2025-10-11**, AGPL-3.0-only, 282 total downloads, ~663 lines of code. Repo: https://github.com/aerkiaga/rspice — 1 commit, 0 stars, 0 forks. Supports passive components and transient analysis only; active/non-linear components are "To Do".
- **Severity: LOW legally, HIGH practically.** It is an embryonic hobby project with no commercial activity and no plausible trademark rights. But it **occupies the `rspice` crate name on crates.io** — the registry a Rust-based simulator would most want — and it is *also* a Rust circuit simulator, the worst-case namespace twin. Options (attorney/business decision): publish under a different crate name (e.g. `rspice-core`, `rspice-sim`), politely ask the owner to transfer (crates.io does not arbitrate name disputes except via its policy for trademark holders — a registered mark would strengthen any request), or treat crates.io as non-essential distribution.
- Note: the GitHub *organization/user* name `rspice` is separately taken by an unrelated individual (IoT/JavaScript repos): https://github.com/rspice. The npm *user* `rspice` exists (https://www.npmjs.com/~rspice) but **no npm package `rspice` exists** (registry returns 404). **No PyPI package `rspice` exists** (API returns 404). The X/Twitter handle @rspice is taken by an individual (https://x.com/rspice).

### 1c. Historical: RSPICE by Ron Kielkowski / RCG Research (1990s commercial SPICE simulator)

- McGraw-Hill's *Inside SPICE* (1st ed. 1993, 2nd ed. 1998) shipped with "RSPICE for Windows" and RGraph on disk/CD — the author's own commercial SPICE simulator. Listings: https://www.thriftbooks.com/w/inside-spice-with-rspice-for-windows-sample-spice-models_ron-m-kielkowski/1247623/ , https://books.google.com/books/about/Inside_SPICE.html?id=4uVSAAAAMAAJ , https://www.amazon.com/Inside-Spice-Overcoming-Obstacles-Simulation/dp/007911525X
- **Severity: LOW.** This is the most legally interesting hit because it is identical goods (commercial SPICE simulator) under the identical name — but it appears to have been dead for ~25 years. No current company, website, product, or live registration surfaced. US trademark rights lapse with abandonment (3 years of non-use is presumptive abandonment). An attorney should confirm in TESS that any 1990s registration is DEAD and check for residual common-law use; assuming abandonment, this history does not block us, though it slightly muddies "first use" storytelling.

### 1d. Academic: "RSPICE: A Fast and Robust Timing Simulator for Digital MOS VLSI"

- IEICE Transactions on Fundamentals (1999): https://globals.ieice.org/en_transactions/fundamentals/10.1587/e82-a_11_2492/_p
- **Severity: NEGLIGIBLE.** A paper title, not a product or mark. Adds minor dilution to literature searches.

### 1e. Aerospace check: NASA SPICE toolkit "RSPICE"

- **No official "RSPICE" exists in the NASA/NAIF SPICE ecosystem.** The toolkit ships in C/FORTRAN/IDL/MATLAB/JNI; community wrappers are SpiceyPy (Python), and the Rust-world re-implementation is named **ANISE**, not RSPICE. Sources: https://naif.jpl.nasa.gov/naif/toolkit.html , https://en.wikipedia.org/wiki/SPICE_(observation_geometry_system)
- An R wrapper named "rspice" for NAIF SPICE was *not* found on CRAN or in search results. **No collision.** (Different industry anyway — likelihood-of-confusion would be low even if one existed.)

### 1f. Other entities named "RSpice"

- **RSPICE CROSSWAY LIMITED** — UK private company (licensed restaurant), incorporated June 2023, **dissolved 5 Aug 2025**: https://find-and-update.company-information.service.gov.uk/company/14958067 . Wrong class (food service), dissolved. No conflict.
- No US/EU software or electronics company named RSpice/R-Spice was found.

### 1g. The *SPICE naming family and confusion risk with PSpice / HSPICE

- SPICE (Berkeley, 1973) is the generic name of the simulation technology; Cadence's own marketing page treats "SPICE simulation" as a category term: https://www.cadence.com/en_US/home/explore/spice-simulation.html . Wikipedia: https://en.wikipedia.org/wiki/SPICE
- The field is intentionally crowded with {prefix}SPICE marks owned by different companies that coexist without (publicly known) litigation: **PSpice** (Cadence, https://www.cadence.com/en_US/home/tools/pcb-design-and-analysis/analog-mixed-signal-simulation/pspice.html), **PrimeSim HSPICE** (Synopsys, https://www.synopsys.com/implementation-and-signoff/ams-simulation/primesim-hspice.html — "40 years of HSPICE"), **LTspice** (Analog Devices, https://www.analog.com/en/resources/design-tools-and-calculators/ltspice-simulator.html), **QSPICE®** (Qorvo, 2023, https://www.qorvo.com/design-hub/design-tools/interactive/qspice), **SmartSpice** (Silvaco, https://en.wikipedia.org/wiki/SmartSpice), plus ngspice, XSPICE, IsSpice, T-Spice.
- **Why this matters legally:** in a crowded field built around a generic root, each mark's protectable scope shrinks to its distinctive prefix. "R" vs "P" differ visually and phonetically ("ar-spice" vs "pee-spice"); consumers of EDA tools are sophisticated purchasers, which further reduces confusion risk. **Qorvo's QSPICE® — a brand-new single-letter+SPICE mark registered in the 2020s while PSpice, HSPICE, and the rest were all live — is the best single piece of evidence that RSPICE is registrable.**
- **Residual risk:** Cadence is large and has counsel on retainer; a Section 2(d) citation of PSPICE by a USPTO examiner is conceivable even if ultimately overcome. Rate this risk low-to-moderate; an attorney's likelihood-of-confusion analysis is the key remaining check.

---

## 2. Registered-Mark Signals (public indexes only)

- **Justia Trademarks site-search (`site:trademarks.justia.com rspice`): no RSPICE record.** Closest hits are unrelated food/pharma marks (DOYOURSPICE, RESPICORT, SPICE WORLD, etc.). A direct fetch of Justia's search page returned HTTP 403 (bot-blocked), so this is index-level evidence only.
- **uspto.report site-search (`site:uspto.report rspice`): no RSPICE record.** Direct fetch also 403-blocked.
- **EUIPO / TMview / WIPO Global Brand Database:** not queryable via web search; no web pages mentioning an RSPICE EU/international registration were found. Tools an attorney (or you, manually) would use: https://www.tmdn.org/tmview/ , https://euipo.europa.eu/eSearch/ , https://www.wipo.int/en/web/global-brand-database
- **Interpretation:** absence from these public mirrors is a good signal but NOT proof — Justia/uspto.report lag and 403-block automated checks, and dead 1990s registrations (e.g., a possible Kielkowski-era RSPICE filing) may not be web-indexed. Direct TESS and TMview queries are mandatory before filing.

---

## 3. Domains

| Domain | Status observed (2026-06-10) | Evidence |
|---|---|---|
| **rspice.com** | **Taken — parked for sale on HugeDomains at $695** (or $28.96/mo x 24) | https://rspice.com redirects to https://www.hugedomains.com/domain_profile.cfm?d=rspice.com |
| **rspice.io** | No website resolves (connection refused) — likely available; confirm at a registrar | Direct fetch failed: ECONNREFUSED |
| **rspice.dev** | No website resolves — likely available; confirm at a registrar | Direct fetch failed: ECONNREFUSED |
| **getrspice.com** | No website resolves — likely available; confirm at a registrar | Direct fetch failed: ECONNREFUSED |

$695 for the exact-match .com is inexpensive for an aftermarket 6-letter brandable; acquiring it early (before any public launch raises the price) is the obvious move. "Connection refused" means no web server answered — it does not conclusively prove the names are unregistered; verify via WHOIS/registrar before relying on availability. (Per scope, no registrar checks were performed.)

---

## 4. Secondary Name "VOLTA" — confirm keep-internal plan

The plan to keep VOLTA strictly internal (design-system codename only, never customer-facing) is **wise**. The name is saturated:

- **NVIDIA Volta** GPU microarchitecture (2017) — and instructively, NVIDIA **lost** its USPTO "Volta" application in the AI field on 3 July 2023 to prior registrant **Volta Robots**: https://en.wikipedia.org/wiki/Volta_(microarchitecture) . If NVIDIA couldn't clear VOLTA in tech, nobody should try casually.
- **Volta — the dead macOS schematic-capture + SPICE app** (KulFX, developed 2007–2013, open-sourced at https://github.com/robo-fish/Volta). This one is *in our exact product category* (circuit design with SPICE simulation) — the single worst Volta collision for an EDA company.
- **Volta.sh** — the popular JavaScript toolchain manager: https://volta.sh/
- **Volta Charging / Volta Inc.** — EV charging network, acquired by Shell for $169M (2023), wound down and sold to Jolt (2025): https://en.wikipedia.org/wiki/Volta_Charging
- Plus: volta.net (desktop app), Volta battery/power utilities for macOS (volta.garymathews.com, cupcakearmy/volta), Volta Trucks (EV trucks), and others.

**Recommendation:** never use VOLTA in marketing, file names shipped to customers, public docs, or UI strings visible to users. If the design system ever needs a public name, run a fresh clearance on a different word.

---

## 5. "Run" Logo Mark — op-amp triangle as play button (low priority)

- The bare right-pointing triangle is **generic** for "play" (media playback) and for "amplifier" (the standard op-amp schematic symbol — see Analog Devices' essay "It's Just a Triangle": https://www.analog.com/en/resources/analog-dialogue/articles/its-just-a-triangle-or-what-does-a-symbol-really-mean.html). Genericness cuts both ways: hard for anyone to stop us using a triangle, and hard for us to claim exclusivity in one without distinctive styling.
- **YouTube** protects its specific play-button lockup (white triangle inside the rounded red rectangle) and publishes restrictive brand guidelines: https://www.youtube.com/howyoutubeworks/resources/brand-resources/ , https://support.google.com/youtube/answer/6154218 . Avoid: red rounded-rect container + white triangle. A triangle in any other treatment is far from their trade dress.
- No famous *registered* bare-op-amp-triangle logo in EDA was found in searching; the symbol is ubiquitous in the industry as an engineering glyph, not as anyone's brand.
- **Designer checklist:** (a) stay away from YouTube's red lockup and from Google Play's multicolor triangle; (b) add distinctive elements (the op-amp's +/− input pins, feedback arc, color, negative space) so the logo is protectable; (c) run a design-mark (Vienna-code) search at filing time if the logo will be registered.

---

## What a Real Trademark Attorney Search Would Add

1. **Direct USPTO TESS query** for RSPICE and phonetic/visual equivalents (R-SPICE, ARSPICE, RSPYCE, R SPICE), including DEAD marks — to confirm the Kielkowski-era name left no live registration and to surface anything Justia/uspto.report mirrors missed.
2. **Section 2(d) likelihood-of-confusion opinion** versus PSPICE, HSPICE, QSPICE and the rest of the crowded {X}SPICE field — the legal crux of this filing.
3. **TMview/EUIPO, UKIPO, WIPO Madrid, CIPO, JPO, KIPO, CNIPA queries** for priority countries.
4. **Common-law / state / business-registry search** (Dun & Bradstreet, Secretary of State filings, domain-history, archived web) for unregistered users with priority — especially any residual RCG Research/Kielkowski activity.
5. **Specimen and ID-of-goods drafting** for classes 9 (downloadable EDA software) and 42 (SaaS, design services), structured to survive examiner citations.
6. **Watch service** post-filing, and advice on whether the crates.io occupant matters legally (it almost certainly doesn't, but a demand-letter-free resolution path is worth scripting).

---

## Recommended Next Steps (research suggestions, not legal advice)

1. **Engage a trademark attorney** for a full clearance + filing; bring this memo as the knockout layer.
2. **File US ITU (intent-to-use) application** for RSPICE, classes 9 + 42, as soon as cleared — US is the home market and where Cadence/Synopsys/Qorvo precedents live. ITU locks priority before public launch.
3. **Priority countries beyond US** (matching the commercialization plan's likely markets): EU (EUIPO), UK, then Madrid Protocol extensions to JP/KR/TW/CN — semiconductor-heavy jurisdictions where EDA sells. Stagger via Madrid within the 6-month Paris priority window to spread cost.
4. **Buy rspice.com now (~$695)** and register rspice.io / rspice.dev / getrspice.com if registrar checks confirm availability — total likely under $800, and exposure only grows after launch.
5. **Resolve the crates.io `rspice` name** before any public Rust-crate release: pick `rspice-*` naming, or open a friendly dialogue with the crate owner (after the mark is filed, which strengthens crates.io's trademark-dispute pathway).
6. **Secure social/registry handles** where still free (npm package name and PyPI name are free as of today; GitHub org and X handle are taken — decide on `rspice-eda`/`rspicehq` style fallbacks).
7. **Document first use in commerce** (dated screenshots, invoices, release announcements) from day one — it anchors priority against any future challenger.
8. **Keep VOLTA internal forever** (Section 4) and give the designer the logo checklist (Section 5).

---

*Prepared via free web search (WebSearch/WebFetch) on 2026-06-10. No trademark databases were queried directly; no registrars were consulted; no third parties were contacted. This document is research support for counsel, not legal advice.*
