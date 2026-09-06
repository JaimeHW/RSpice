# rspice-pack

The `.rspicepack` model pack format and its trust boundary. A pack is a signed,
immutable archive of authored SPICE model sources; this crate defines the
container and is the only code allowed to canonicalize, sign, or verify one. The
desktop client, the browser client, the cloud service, and the packer tool reach
the same verdict because they run these exact functions.

What it does not have is as deliberate as what it does. There is no HTTP client,
filesystem access outside the optional `packer` feature, clock, async runtime,
platform binding, general-purpose ZIP implementation, or random-number source in
the portable path, so the crate compiles for `wasm32-unknown-unknown` and
behaves identically there. `#![forbid(unsafe_code)]`.

## Container

```text
manifest.json          canonical JSON, the signed document
signature.ed25519      64 raw bytes over the exact manifest.json bytes
models/...             authored model sources
LICENSE, NOTICE        optional
```

Verifying one:

```rust
use rspice_pack::{Limits, Pack, verifying_key_from_hex};

let key = verifying_key_from_hex(public_key_hex)?;
let pack = Pack::verify(&archive, &key, &Limits::default())?;
for part in &pack.manifest.parts {
    println!("{} from {}", part.id, part.source.path);
}
```

## Reading hostile bytes

The ZIP reader is written for this format alone and refuses everything outside
it, naming the rejected `ArchiveFeature`: zip64, encryption, masked local
headers, trailing data descriptors, general-purpose flags, any compression
method but DEFLATE, extra fields, comments, multiple disks, non-zero external
attributes. Paths are validated against `validate_path` and `MAX_PATH_BYTES`
before use, and `MANIFEST_ENTRY`/`SIGNATURE_ENTRY` are reserved.

`Limits::SPEC` is a set of format constants rather than tuning knobs. A pack
that exceeds them is invalid everywhere, so producers and consumers must agree:

| Bound | Value |
| :--- | :--- |
| `max_archive_bytes` | 64 MiB, on the compressed bytes as received |
| `max_expanded_bytes` | 256 MiB, checked against declared sizes *before* inflating, then re-proved per entry |
| `max_files` | 10 000, including the manifest and signature |
| `max_snapshot_bytes` | 32 MiB |

`Limits` is injectable only so tests can prove each cap fires without
materializing hundreds of megabytes.

## The signed catalog snapshot

A `Snapshot` is the signed catalog the service publishes and every client
resolves against. Parts ride on the release rather than in the pack archive, so
a client shelf can search the whole catalog without downloading a single pack.
Three fields make a stale or malicious catalog detectable: `serial` orders two
authentic catalogs so an older one cannot be replayed over a newer one,
`expires_at` bounds how long any one of them may be believed, and `revocations`
names releases that must be recalled even though their bytes remain downloadable
for a project that pinned them.

## The `packer` feature and binary

`packer` adds filesystem walking, key generation, and the `rspice-pack`
executable. It is off by default so verifying clients link no entropy source at
all.

```text
rspice-pack keygen --out <secret-file>
rspice-pack build <dir> --template <template.json> --key <secret-file> -o <pack>
rspice-pack verify <pack> --pubkey <hex>
```

```text
cargo run -p rspice-pack --features packer -- verify models.rspicepack --pubkey <hex>
```

## Testing

```text
cargo test -p rspice-pack --all-features
```

Licensed under the [RSpice Personal Use License](../../LICENSE).
