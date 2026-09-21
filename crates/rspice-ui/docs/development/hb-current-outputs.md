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
are tied. MOS overlap charge is added from the linear capacitance operator.
Native BJT observations include authored series networks, external BC charge,
and external contributions of delay states. This does not change the solved
equations or the retained operating-point identity.

The current observer covers the native devices already admitted by HB. It does
not enable previously unsupported model families or provide Verilog-A terminal
observations. A requested trace absent from a result remains an unavailable
saved output, rather than becoming a zero-valued current.

Focused checks use `hb_device_current` in the core and UI library tests. They
compare linear and nonlinear currents with MNA KCL, check tied MOS lead charge,
exercise GP/VBIC series networks, and verify worker spectra, study selectors,
hierarchical output binding, project receipts and rename behavior.
