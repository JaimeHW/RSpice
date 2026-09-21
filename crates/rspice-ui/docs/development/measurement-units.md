# Measurement units

Scalar `.MEAS` results retain separate physical units for their published value,
pre-projection raw value, and independent axis. TRAN, AC, DC, and NOISE adapters
attach these after equation and current-observation overlays. DC uses the swept
quantity; input-referred noise uses the selected voltage/current source. Noise
operators remain power densities, not amplitude densities.

The specification unit now controls conversion of measured values before limits
and guard bands are evaluated. The result table, console, and publication view
use the same physical conversion. Authored GOAL/TOL and FAILVALUE contracts stay
in their original value/raw-value units. A blank specification unit uses the
native value. Unknown or incompatible units cannot produce a passing bound.

Measurement units survive worker response protocol 30, result schema 37, and
Monte Carlo checkpoint envelope v2. Checkpoints without units keep their v1
encoding. An optional result-digest extension authenticates units without changing
historical identities. Missing historical metadata keeps the original numeric
interpretation; it is distinct from a newly inferred unknown unit. Source schemas
before 37 reject attached unit metadata.

Explicit study selectors retain units too. `last:`, `bin:`, and `tuple:` read
the selected trace's unit, with degrees for phase and the FFT producer's
normalization rule. `scalar:` uses the analysis quantity: OP voltage/current,
TF gain normalization and resistance, DC mismatch output, normalized sensitivity,
periodic frequency/period/counts, FFT figures, and voltage/current noise RMS.
QPNOISE contributor percentages retain percent rather than a fractional ratio.

NOISE, HBNOISE, and PNOISE producers state the output spectrum unit before worker
transport. Phase noise uses dBc/Hz, which is dimensionally distinct from dB/Hz
and linear power densities. Retained traces preserve that unit. Integrated noise
in the console uses the producer's RMS result; it is never a sum of density samples.

Inference covers native probes, reductions, derivatives, integrals, axis
projections, and dimensionally compatible expressions. Undeclared parameter
units and unsupported expressions remain unknown. Raw sensitivity parameter
dimensions, PZ transfer-gain dimensions, continuous event streams, and native
scalar payloads outside study selections still require producer-specific coverage.
