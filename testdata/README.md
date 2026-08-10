# Test cryptographic material

The key pair in this directory is public, deterministic test material used only
to exercise the native PS256 verifier. It is not a secret and must never be
configured for a deployed RSpice environment. Production private keys are
created outside the repository; the verifier that admits them is
`crates/rspice-ui/src/services/license.rs`, whose `PRODUCTION_VERIFYING_KEYS`
table is deliberately empty until the production issuing ceremony publishes
its public verifier.
