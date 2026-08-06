# Immutable carrier for the RSpice self-contained engine adapter component.
#
# This image is never executed. RSpice-Cloud's release pipeline verifies the
# signed simulator-component attestation on it and then copies
# /rspice-engine-adapter into its worker-production-self-contained runtime,
# re-checking the exported bytes against the attested SHA-256. The build
# context contains exactly one file: the adapter binary built inside the
# pinned Debian bookworm Rust toolchain so its glibc requirement never
# exceeds the consuming runtime image.
FROM scratch

ARG RSPICE_BUILD_SHA

COPY --chmod=0555 rspice-engine-adapter /rspice-engine-adapter

LABEL org.opencontainers.image.title="rspice-engine-adapter" \
      org.opencontainers.image.description="RSpice self-contained simulation engine component exporting /rspice-engine-adapter" \
      org.opencontainers.image.source="https://github.com/JaimeHW/RSpice" \
      org.opencontainers.image.revision="${RSPICE_BUILD_SHA}" \
      org.opencontainers.image.licenses="LicenseRef-RSpice-Proprietary"
