# HB current outputs

Studio retains current spectra on the same frequency grid and in the same
peak-amplitude convention as HB node voltages. DC is real; positive-frequency
coefficients retain their rectangular components and carry amperes through
worker transport, saved outputs, project results and study measurements.

`I(device)` selects the directed current of an independent source, resistor,
capacitor, inductor, VCCS or CCCS, or an existing exact MNA branch. For a native
nonlinear device it selects the first lead: diode anode, MOS/JFET drain, BJT
collector, or switch positive terminal. Positive current enters that terminal.
Individual MOS leads use `@M1[id]`, `@M1[ig]`, `@M1[is]`, `@M1[ib]`; BJT leads use
`@Q1[ic]`, `@Q1[ib]`, `@Q1[ie]`, `@Q1[is]`. These are raw current outputs, while
non-current operating-point parameters remain in the operating-point category.
Hierarchical expressions such as `@/X1/M1[ig]` bind to the corresponding engine
trace and follow instance renames. Expressions can quote the complete trace
name when arithmetic is needed. Study selectors include `bin:1:real:@M1[ig]`.

The core evaluates nonlinear lead F/Q with the solved periodic state, resolved
model parameters, and the residual's transform grid, then forms `F + jωQ`.
Observation coordinates preserve distinct leads even when their circuit nodes
are tied. Classic MOS overlap charge is added from the linear capacitance operator.
Native BJT observations include authored series networks, external BC charge,
and external contributions of delay states. This does not change the solved
equations or the retained operating-point identity.

Native BSIM3 (`LEVEL=8/9/49`, subject to the selected SPICE dialect) contributes
its own channel, junction, substrate and displacement currents. `CAPMOD=0..3`,
`XPART`, geometry, temperature, multiplier and series resistance retain their
native model meanings. `NQSMOD=1` includes the channel charge-deficit state in
HB and QPSS. AC-only `ACNQSMOD=1` does not change the carrier equations; driven
periodic AC, transfer and noise analyses include its channel-current and
intrinsic-charge relaxation. This also applies when reusing a retained HB,
QPSS or shooting-PSS operating point. Periodic noise supports `NOIMOD=1..6`, including
channel thermal noise, empirical and physical flicker laws, `KF/AF/EF`, and
source/drain resistor noise. The native `ID` and `FN` mechanisms retain their
identities as the bias varies; colored noise includes amplitude modulation and
sideband correlations. Envelope reconstructs BSIM3 terminal and NQS charge
history from its solved carrier and supports portable checkpoint continuation.
PSS supports quasi-static and NQS BSIM3 charge, including coupled terminal,
overlap and junction storage, and portable transient continuation. NQS shooting
retains the stored channel charge while allowing algebraic terminal voltages
to settle. Generic saved HB phase projections
retain the BSIM3 continuation limit; the authenticated Envelope initializer
supplies the additional history.

The current observer covers the native devices already admitted by HB. It does
not enable previously unsupported model families or provide Verilog-A terminal
observations. A requested trace absent from a result remains an unavailable
saved output, rather than becoming a zero-valued current.

Focused checks use `hb_device_current` in the core and UI library tests. They
compare linear and nonlinear currents with MNA KCL, check tied MOS lead charge,
exercise GP/VBIC series networks, and verify worker spectra, study selectors,
hierarchical output binding, project receipts and rename behavior.
