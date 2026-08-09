# Managed native Python runtime payload

`rspice_worker.py` is the trusted native Automation protocol worker. Release assembly places it
inside an application-local CPython 3.14 runtime at `runtimes/python/worker/rspice_worker.py`.
RSpice never searches `PATH`, the Windows Python launcher, registry installations, virtual
environments, or user site-packages for product execution.

The staged directory must contain the complete interpreter, standard library, approved packages,
license notices, and the worker before it is signed. Inventory and signing use the repository-owned
Rust tool so release creation and installed verification share the exact digest implementation:

```text
uv 0.12.1 python install --managed-python --python-preference only-managed 3.14.6
python tools/release/stage_managed_python.py \
  --python <uv-managed-cpython-3.14.6-executable> \
  --runtime-root <fresh-stage>/runtimes/python \
  --worker assets/automation_runtime/rspice_worker.py \
  --expected-build <standalone-BUILD-identity>
```

The staging tool launches the candidate with `-I -S`, verifies exact CPython 3.14.6 and `cp314`,
requires the python-build-standalone `BUILD` identity, rejects escaping symlinks, strips installer
and test payloads that are not part of the product runtime, copies license evidence, emits the
runtime notice and CycloneDX 1.5 component SBOM, and re-probes the relocated interpreter before
signing. The release workflow pins both uv and the setup action; it does not accept whatever Python
happens to be installed on the build host.

The staged result is then signed:

```text
cargo run --locked --release -p rspice-automation-runtime \
  --bin rspice-managed-runtime-packager -- \
  --runtime-root <fresh-stage>/runtimes/python \
  --runtime-build rspice-cpython-3.14.6+<release-id> \
  --target <rust-target-triple> \
  --architecture <x86_64-or-aarch64> \
  --python-version 3.14.6 \
  --python-abi cp314 \
  --api-version 1.0.0 \
  --python-executable <portable-relative-python-path> \
  --worker-bootstrap worker/rspice_worker.py \
  --environment-digest d445b1443965be4e6b1b191ee023176dbd35430ac3cd00603458384ea03b8518 \
  --key-id <release-key-id> \
  --signing-key-file <protected-ed25519-seed-file>
```

The signing seed file contains exactly 64 hexadecimal characters representing a 32-byte Ed25519
seed. It is a release-authority input and must never be committed, passed on the command line, or
stored in a job-wide or reusable general CI environment variable. The release workflow may expose
the secret only to its isolated signing step, writes it to a protected temporary file, unsets the
step variable before launching the packager, and removes the file on every exit path. The packager
refuses symlinks, non-regular members,
pre-existing manifests, duplicate environment identities, nonportable paths, and any payload that
does not self-verify after signing.

Release builds compile the printed `signing_key_id` and `public_key_hex` into `rspice-ui` through
`RSPICE_AUTOMATION_RUNTIME_KEY_ID` and `RSPICE_AUTOMATION_RUNTIME_PUBLIC_KEY_HEX`. The private seed
is not present in the application or archive. Development builds without an explicit trust key
correctly report native managed Automation as unavailable; they never fall back to a system Python.
