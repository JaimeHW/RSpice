# RSpice licensing rework — DRAFT for counsel review

> **Status: DRAFT · 2026-06-10 · NOT IN EFFECT.**
> This document proposes replacement licensing for launch. It was prepared as
> engineering/business research, **not legal advice**. The live `LICENSE` file
> (RSpice Personal Use License v1.1) remains in force until James swaps it after
> attorney review. Do not publish binaries under these terms before that review.

## 1. Why the current license must change

The commercialization plan sells a **free Community tier whose entire point is
commercial use** ("$0 forever — commercial use allowed", matching and beating
LTspice's position). The live LICENSE v1.1 **prohibits all commercial use,
evaluation, and prototyping** (§1(c), §3(a)) and prohibits redistribution of
binaries (§3(b)) — which would even forbid a colleague forwarding the
installer. Every funnel touchpoint on the new website contradicts the license
in the repo. This is the single highest-priority legal fix before launch.

## 2. Proposed structure: two documents, not one

Mixing source rights and binary rights in one file is how v1.1 got into
trouble. Split them:

| Artifact | Audience | Proposed terms |
|---|---|---|
| **Repo source code** | developers reading/building the public repo | **FSL-1.1-ALv2** (Functional Source License) — recommended, see §3 |
| **Distributed binaries** (Community/Pro/Team/Enterprise builds) | end users | **RSpice EULA v2.0** (bespoke, drafted in §4) |

The repo carries `LICENSE` (FSL) and the installers/app carry the EULA; Pro
features are additionally gated by the signed license key, which is an access
mechanism, not a license document.

## 3. Source license — recommend FSL-1.1-ALv2, not bespoke text

**Recommendation: adopt the Functional Source License, version 1.1, with
Apache 2.0 future grant (FSL-1.1-ALv2) verbatim.**

Why FSL over alternatives considered:

- **It permits everything the funnel needs**: reading, building, internal use,
  commercial *use* of the software — while prohibiting exactly the one thing
  that matters: offering a *competing* product/service built from the source.
- **Two-year Apache-2.0 conversion** is a powerful trust signal for an
  engineering audience ("the code you depend on eventually becomes truly
  open"), and directly supports the validation story — anyone can rerun the
  parity evidence.
- **Lawyer-vetted, publicly drafted, recognizable** (Sentry et al.). A bespoke
  source-available license would cost review time and earn community distrust.
- Alternatives: *PolyForm Internal Use* (no conversion, less goodwill),
  *BUSL-1.1* (parameter soup, GPL-conversion default needs configuring),
  *keeping v1.1* (kills the funnel), *Apache-2.0 now* (gives away the Pro
  moat — HB/PSS could be forked and resold immediately).

Caveats for counsel:
- Confirm FSL's "Competing Use" definition covers an EDA vendor embedding the
  engine in their commercial tool suite.
- Decide the copyright owner of record ("RSpice Contributors" is not a legal
  person — see checklist §6.1).
- The repo's vendored third-party material (ngspice test decks, IBM Plex
  fonts) keeps its own licenses and must be carved out via `NOTICE` — see the
  provenance audit.

## 4. Binary EULA — RSpice End User License Agreement v2.0 (DRAFT)

> Full proposed text below. Bracketed items are decisions for James/counsel.

```
RSPICE END USER LICENSE AGREEMENT
Version 2.0-draft (2026-06-10)

This agreement is between you and [LEGAL ENTITY — see checklist §6.1]
("we", "us") and governs the RSpice application binaries, including the
desktop, browser-delivered (WebAssembly), and mobile/tablet builds
(the "Software").

1. LICENSE GRANT
   (a) Community Edition. We grant you a worldwide, non-exclusive,
       non-transferable license to install and use the Community Edition
       free of charge, for any purpose, INCLUDING COMMERCIAL USE.
   (b) Paid Editions (Pro, Team, Enterprise). The same grant, with the
       features, seat counts, and term defined by your license key or
       written agreement. Paid features are enabled by a digitally
       signed license key; the key is yours to keep and works offline.
   (c) Each personal license may be activated on any number of machines
       used by the licensed engineer; floating/pooled licensing is
       available under Team/Enterprise terms.

2. WHAT YOU MAY ALSO DO
   (a) Redistribute the UNMODIFIED Community installer/binaries inside
       your organization and to colleagues, free of charge.
   (b) Use simulation results, netlists, schematics, and exports you
       create with the Software for any purpose without restriction or
       royalty. Your designs are yours.
   (c) Use the Software in CI or automated pipelines [Community: yes;
       rate/scale limits? — decide].

3. RESTRICTIONS
   You may not:
   (a) circumvent, remove, or tamper with license-key verification or
       feature gating;
   (b) sell, rent, sublicense, or offer the Software itself (or its
       simulation engine) to third parties as a product or hosted
       service;
   (c) misrepresent the origin of the Software or use our marks except
       to truthfully describe it (see Trademark Policy);
   (d) use the Software in violation of applicable export-control law.
   Note deliberately ABSENT restrictions: no benchmarking gag clause
   (publish benchmarks freely — we publish ours), no telemetry consent
   (there is no telemetry), no reverse-engineering clause beyond the
   license-gating circumvention in (a) [counsel: confirm this scope].

4. THIRD-PARTY COMPONENTS
   The Software includes third-party components under their own terms,
   listed in the NOTICE file distributed with the Software.

5. UPDATES & VERSIONS
   A perpetual key activates all versions released during its term
   [fallback license model: keys activate the last version released
   before expiry, forever — decide subscription vs perpetual-fallback;
   the pricing page currently implies subscription for Pro].

6. WARRANTY DISCLAIMER
   THE SOFTWARE IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND...
   [standard disclaimer; counsel to supply jurisdiction-appropriate text.
   Consider an engineering-honest preamble: simulation results depend on
   models and settings; verify silicon-bound designs independently.]

7. LIABILITY LIMIT
   [Standard cap-at-fees-paid clause; counsel to draft. Important for an
   EDA tool — designs taped out on simulation results.]

8. TERMINATION
   The license ends automatically on material breach. On termination you
   stop using the Software; sections 2(b)-results, 4, 6, 7 survive.
   Your data and exports remain yours.

9. GOVERNING LAW / VENUE
   [Decide with counsel — likely James's state/country.]
```

## 5. Trademark policy (one paragraph, publish alongside)

Permit: truthful references ("imports RSpice netlists", "validated against
RSpice"), screenshots, benchmarks. Forbid: naming a product/fork "RSpice",
using the Run mark or confusingly similar marks, implying endorsement.
(Blocked on the knockout search → see `trademark-knockout.md`.)

## 6. Pre-launch legal checklist

1. **Form a legal entity** (LLC or equivalent) to be the copyright holder,
   EULA counterparty, and Paddle merchant account holder. "RSpice
   Contributors" cannot sign contracts or hold a trademark. *(Costs money —
   parked until James green-lights spending.)*
2. **Swap `LICENSE`** → FSL-1.1-ALv2 (or counsel's pick) after review; add
   `LICENSE-EULA` to installer artifacts; keep `NOTICE` current.
3. **Trademark**: knockout search (in progress) → attorney full search →
   file class 9 + 42 in priority markets.
4. **ngspice provenance**: close out the audit's action items; keep the
   "never port from KLU/numparam/xspice-table" rule in CONTRIBUTING/agent
   instructions.
5. **AI-codegen provenance**: keep all generation under James's own accounts;
   retain session records; note that pure-AI output's copyright status varies
   by jurisdiction — entity assignment + human curation records mitigate.
6. **Paddle MoR contract** covers consumer-law/tax exposure for checkout
   (parked — costs money only at transaction time, but account setup needs the
   entity first).
7. **Export control**: EDA simulation software is generally EAR99/uncontrolled,
   but confirm once HB/cloud runners ship; add the standard restriction (§3(d)).
8. **Privacy**: launch is account-less with no telemetry → no GDPR surface
   beyond the website (use a no-cookie analytics or none; the current design
   uses none). Revisit at cloud tier.

## 7. What changes on the website when this lands

- Pricing page footnote already matches ("licenses are signed keys that work
  offline — no account, no telemetry, no phone-home") ✓
- Add `/license` page rendering the EULA + FSL text (designed in the interior
  pages spec).
- Community tier card: "Commercial use allowed" stays — it becomes true the
  day the swap lands, and not before.
