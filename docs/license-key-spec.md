# RSpice license keys — format & verification spec (v1 draft)

> Draft 2026-06-10 for the `rspice-platform` signer and the in-app activation
> flow. Companion design artifact: `design/app/volta-license-dialog.html`.
> Goal: **offline-first, account-less, tamper-evident, boring to operate.**

## 1. Principles

- A license is a **signed statement, not a server check**. The app verifies an
  Ed25519 signature against public keys compiled into the binary. No phone-home,
  no activation server, works air-gapped — this is a selling point on the
  pricing page and must stay true.
- Keys are **possession tokens**: whoever has the key string can activate.
  Anti-abuse comes from watermarking (licensee name shown in-app and embedded
  in exports' metadata header) and release-embedded denylists, not DRM.
- Format is **versioned and TLV-extensible** so new fields never break old
  binaries (old binaries ignore unknown TLVs).

## 2. Wire format

```
RSPICE-K1.<base32(payload)>.<base32(signature)>
```

- Prefix `RSPICE-K1` = format version 1. Crockford base32, uppercase, no
  padding; rendered in groups of 5 separated by `-` for human transcription
  (grouping is cosmetic; the parser strips `-` and whitespace).
- `signature` = Ed25519 over `"rspice-license-v1" || payload` (64 bytes).
  Domain-separation tag prevents cross-protocol reuse.

### Payload (binary, little-endian)

| Field        | Size | Notes                                                      |
|--------------|------|------------------------------------------------------------|
| version      | u8   | payload layout version = 1                                 |
| key_id       | u8   | which signer public key (rotation; binary ships all valid) |
| license_id   | u64  | random; the denylist + support handle                      |
| tier         | u8   | 1=Pro, 2=Team, 3=Enterprise (Community needs no key)       |
| seats        | u16  | 0 = unlimited within licensee org (Enterprise floating)    |
| issued       | u32  | unix days                                                  |
| expires      | u32  | unix days; **updates-until date, not a kill switch**       |
| features     | u32  | bitfield: 1=RF suite, 2=Python API, 4=encrypted models,    |
|              |      | 8=cloud runners (reserved)                                 |
| TLVs…        | var  | type:u8, len:u8, bytes. 0x01=licensee display name (UTF-8  |
|              |      | ≤ 48 B), 0x02=licensee email hash (16 B), others ignored   |

Typical Pro key ≈ 60 B payload + 64 B sig ≈ 200 chars rendered — long but
paste-once. Example rendering:

```
RSPICE-K1.01A4G-...-9XKQM.ZB5RW-...-D27TC   (sample, not a valid key)
```

## 3. Verification (in-app, `rspice-license` crate)

1. Strip grouping; split on `.`; check prefix.
2. Decode payload + sig; verify Ed25519 with the public key named by `key_id`
   (reject unknown ids).
3. Check `license_id` against the release-embedded denylist (leaked keys;
   shipped as a static array, updated each release — no network).
4. **Perpetual-fallback semantics**: the key activates any build whose
   release date ≤ `expires`. Builds released after expiry run with the last
   in-term feature set… i.e. compare against the binary's compiled-in release
   date, NOT the wall clock. No clock games, no expiry surprises offline.
5. Store the key verbatim at `{config_dir}/rspice/license.key` (plain text,
   user-ownable, survives reinstall, syncs via dotfiles if the user wants).
6. Surface state via one enum: `Community | Licensed { tier, name, updates_until } |
   Invalid(reason)` — the UI never re-implements policy.

Failure messages are specific and human: "signature check failed (key may be
truncated — paste the whole string)", "this key's update window ended
2027-06-01; it activates releases up to that date", never bare "invalid".

## 4. Signer (`rspice-platform`, private)

- Cold key ceremony: generate keypair offline; public keys committed into the
  app; secret key in the platform secret store (and a paper backup).
  `key_id` rotation: add new pubkey to app ≥1 release before first use.
- `POST /paddle/webhook` (order completed) → issue key → email via Resend +
  show on the thank-you page. Idempotent on Paddle order id; key + order
  stored in Postgres for reissue/lookup.
- CLI mirror for manual issuance: `platform issue --tier pro --name "..."
  --email ... [--seats N] [--expires +1y]`.
- Revocation = append `license_id` to the denylist file in the app repo
  (takes effect next release) + refund handling in Paddle. Accept the
  window; it is the price of offline-first and it is fine.

## 5. In-app activation UX (see design artifact)

- Entry points: Help ▸ Enter License…, the Pro-gated analysis picker rows
  (HB/PSS/PNoise/PAC show a small `PRO` chip; choosing one opens the dialog),
  and About.
- Dialog (VOLTA Dialog grammar — kicker `LICENSE`, title, body, footer
  [ghost][primary]): one large mono textarea (paste-tolerant: accepts line
  breaks/grouping), live validation on paste — on success the body swaps to a
  summary card (licensee, tier, updates-until, feature list) and the primary
  becomes **Activate**; on failure an inline specific error under the field.
- After activation: statusbar stays clean; About + Simulate picker reflect
  the tier. Deactivate = delete the key file (button in About, with confirm).

## 6. Open decisions for James

1. Subscription vs perpetual-fallback framing on the pricing page (spec
   implements perpetual-fallback; the $499/yr renews the update window).
2. Seat enforcement for Team: honor-system seats (count shown in About) vs
   floating tokens (needs the cloud tier anyway).
3. Whether Community requires any key at all (current answer: no — zero
   friction beats telemetry-free registration data).
