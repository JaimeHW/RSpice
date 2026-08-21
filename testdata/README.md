# Test cryptographic material

The key pair in this directory is public, deterministic test material used only
to exercise the native PS256 verifier. It is not a secret and must never be
configured for a deployed RSpice environment. Production private keys are
created outside the repository and admitted through the hardened licensing
bootstrap described in `docs/licensing.md` of the RSpice-Cloud repository.

This file is vendored verbatim into the RSpice application workspace beside
the key pair it describes, where there is no `docs/` directory to follow. The
pointer above therefore names its repository rather than a bare path.
