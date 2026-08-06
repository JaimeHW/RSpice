# Immutable carrier for the RSpice publication renderer component.
#
# This image is never executed. RSpice-Cloud's release pipeline verifies the
# signed render-component attestation on it and then copies /rspice-publish
# into its render-production runtime, re-checking the exported bytes against
# the attested SHA-256. The build context contains exactly one file: the
# renderer binary built inside the pinned Debian bookworm Rust toolchain so
# its glibc requirement never exceeds the consuming runtime image.
FROM scratch

ARG RSPICE_BUILD_SHA

COPY --chmod=0555 rspice-publish /rspice-publish

LABEL org.opencontainers.image.title="rspice-publish" \
      org.opencontainers.image.description="RSpice publication renderer component exporting /rspice-publish" \
      org.opencontainers.image.source="https://github.com/JaimeHW/RSpice" \
      org.opencontainers.image.revision="${RSPICE_BUILD_SHA}" \
      org.opencontainers.image.licenses="LicenseRef-RSpice-Proprietary"
