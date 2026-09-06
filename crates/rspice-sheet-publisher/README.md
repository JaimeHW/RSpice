# rspice-sheet-publisher

The offline signing ceremony for organization-managed RSpice drawing-sheet
packages. A native-only binary that does one narrow job: promote an unsigned
export to organization scope, sign it, and immediately verify the emitted
artifact against the exact package contract the importer uses.

The GUI never accepts private publisher material. That is why this exists as a
separate program rather than a menu item, and why its trusted computing base is
[`rspice-design-model`](../rspice-design-model) plus `clap`, not the desktop
application.

## Commands

```text
rspice-sheet-publisher sign --input <unsigned> --output <signed> \
    --private-key-file <seed> --public-key-file <pub> \
    --publisher-id <id> --key-id <id> \
    --expected-input-digest <sha256> --promote-to-organization

rspice-sheet-publisher verify --input <signed> --public-key-file <pub> \
    --publisher-id <id> --key-id <id>

rspice-sheet-publisher public-key --private-key-file <seed> --output <file>

rspice-sheet-publisher seal-key --input-key-file <seed|-> \
    --output <sealed> --public-key-output <pub>          # Windows only
```

- **`sign`** promotes every exported preset to organization scope and signs the
  package. It cross-checks the private key against the pinned public key file,
  so signing with the wrong seed fails before an artifact exists, and
  `--expected-input-digest` binds the signature to the exact unsigned export
  that was reviewed. `--format` selects the canonical or human-review
  representation.
- **`verify`** checks a package against an *explicitly pinned* public key,
  publisher identity, and key identity. The identity carried in the payload is
  never trusted implicitly.
- **`public-key`** derives the public key to provision into the RSpice trust
  store.
- **`seal-key`** wraps a plaintext 32-byte (or 64-hex-character) seed with
  Windows DPAPI so subsequent signing runs never touch it in the clear; `-`
  reads the seed from stdin.

## Handling rules

Private key material is read only from a protected file, never accepted as a
command-line value; a process listing is not a place for a signing seed. Output
paths must not already exist, so a ceremony never silently replaces a previously
published package.

## Testing

```text
cargo test -p rspice-sheet-publisher
```

Licensed under the [RSpice Personal Use License](../../LICENSE).
