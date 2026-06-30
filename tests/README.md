RSpice test corpora
===================

This directory contains vendored upstream simulator test corpora. Each corpus
keeps its own upstream layout, provenance, and license terms.

- `ngspice/` contains the ngspice-46 regression corpus used by
  `crates/rspice-core/tests/ngspice_regression.rs`.
- `xyce/` contains the Xyce Regression Suite. It is vendored for future Xyce
  corpus support and is not run by the ngspice regression harness.

Do not mix validation manifests, generated outputs, or harness sidecars between
corpora. Add a corpus-specific README or vendoring note when importing another
upstream suite.
