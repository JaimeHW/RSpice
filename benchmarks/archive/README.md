# Legacy benchmark evidence

`legacy-results/` preserves historical optimization evidence produced before
versioned suites, complete host/tool provenance, raw samples, and correctness
preflight were required. These files use multiple incompatible schemas; some
record failed simulator runs. They are not accepted as regression baselines and
must not be used for commercial performance claims.

The archive includes the 2026-07-28 generated-stamp baseline formerly stored
beside external ngspice anchors. It is an RSpice result with an obsolete schema,
so archive ownership is the honest classification.

Git history is the long-term archive. New benchmark output must not be added
here.
