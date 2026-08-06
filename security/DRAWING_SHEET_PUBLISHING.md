# Organization drawing-sheet publishing

RSpice imports organization-scoped drawing-sheet presets only when their
schema-1 package has a valid Ed25519 signature from a trusted, non-revoked
publisher key. The desktop/browser/mobile GUI contains verification keys only;
it has no private-key input, storage, or signing path.

The `rspice-sheet-publisher` executable is the offline administrative boundary
for this workflow. Build and run a release binary from a controlled publishing
machine:

```text
cargo build --release -p rspice-sheet-publisher
target/release/rspice-sheet-publisher --help
```

## Key ceremony

Generate the 32-byte Ed25519 seed with an organization-approved cryptographic
random-number generator. Do not pass it as a command-line value, paste it into
the RSpice GUI, commit it, or store it beside published packages.

On Unix, place the raw 32-byte seed or 64 hexadecimal characters in a file
owned by the publishing user and set mode `0600`. The publisher rejects
symlinks, extra hard links, a different owner, and any group/other permission.
Derive the public-key document once:

```text
chmod 600 organization-sheet-publisher.seed
rspice-sheet-publisher public-key \
  --private-key-file organization-sheet-publisher.seed \
  --output organization-sheet-publisher.pub
```

On Windows, signing accepts only an RSpice DPAPI-sealed seed bound to the
current Windows user and the drawing-sheet publisher domain. Seal a seed and
derive its public-key document in one operation:

```text
rspice-sheet-publisher seal-key ^
  --input-key-file organization-sheet-publisher.seed ^
  --output organization-sheet-publisher.dpapi ^
  --public-key-output organization-sheet-publisher.pub
```

`--input-key-file -` reads the seed from standard input for integration with an
approved secret-delivery process. The tool never deletes a plaintext source;
dispose of it using the organization's approved media-sanitization procedure.

The public-key document contains identical hexadecimal and base64 encodings.
RSpice's **PDK Technology Administration > Publisher trust-root lifecycle**
accepts the base64 value (exactly 32 decoded bytes). Provision it with the
exact publisher ID and key ID that the signing command will use, recording the
required administrator authority and reason. Distribution must not begin until
that provisioning receipt has been independently reviewed.

## Publish

Export the selected project/personal custom sheet formats from RSpice. Review
the unsigned JSON and record the SHA-256 digest reported by the export receipt,
then bind the signing ceremony to that exact digest:

```text
rspice-sheet-publisher sign \
  --input reviewed-sheet-formats.json \
  --output acme-sheet-formats.signed.json \
  --private-key-file organization-sheet-publisher.seed \
  --public-key-file organization-sheet-publisher.pub \
  --publisher-id acme.eda \
  --key-id drawing-sheets-2026 \
  --expected-input-digest <64-character-export-digest> \
  --promote-to-organization \
  --format canonical
```

Use the DPAPI file instead of the seed file on Windows. Publisher and key IDs
must be 1–128 lowercase ASCII identifier characters (`a-z`, `0-9`, `-`, `_`,
`.`, `:`, or `@`). `--format human-review` emits the signed package with a
redundant review table; both representations cover the same canonical contract.

The tool deliberately:

- refuses already signed inputs and unavailable custom-size definitions;
- changes scope only when `--promote-to-organization` is present;
- requires the private key to match a separately pinned public-key file;
- requires the reviewed unsigned-export digest as an independent input;
- signs the importer's domain-separated canonical bytes, then self-verifies;
- writes through a flushed same-directory staging file and byte-for-byte
  read-back check;
- refuses existing destinations, in-place signing, symlink inputs, oversized
  files, malformed schemas, digest mismatches, and unknown JSON fields; and
- zeroizes plaintext seed buffers and the Ed25519 signing key on drop.

Output paths are never overwritten. This makes retries and release evidence
unambiguous: choose a new path for each publishing attempt.

## Independent verification

Verify the exact identity as well as the signature before distribution:

```text
rspice-sheet-publisher verify \
  --input acme-sheet-formats.signed.json \
  --public-key-file organization-sheet-publisher.pub \
  --publisher-id acme.eda \
  --key-id drawing-sheets-2026
```

Record the reported SHA-256 contract digest in the release manifest. A second
operator should repeat verification from a separately obtained public key and
compare the identity and digest. RSpice repeats schema, digest, publisher,
revocation, and Ed25519 verification during import.

## Rotation and incident response

Publish new material with a newly provisioned key ID before retiring an old
key. Revocation is irreversible in the RSpice trust store and immediately makes
packages signed solely by that key unacceptable for new imports. Preserve
signed releases, public keys, administrative provisioning/revocation receipts,
source review evidence, and reported digests according to the organization's
document-control retention policy. Never reuse this seed or key ID for a
different signature domain.
