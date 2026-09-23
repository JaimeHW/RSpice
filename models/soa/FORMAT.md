# Current/voltage SOA curves

In Simulation Studio, add a scoped SOA rule for drain current (`Id`), collector
current (`Ic`) or diode anode current (`Ia`), then enable **Use current/voltage
SOA curves**. Directional current rules also support curves.

Enter the source and rated operating conditions, including case temperature and
single-pulse or repetitive-duty conditions. Use curves already rated for those
conditions. The curve check does not infer case temperature from the device's
junction temperature or calculate thermal recovery between pulses. Manufacturers
specify SOA boundaries for particular voltage, current and stress-time conditions;
see [Infineon's SOA application note](https://www.infineon.com/assets/row/public/documents/24/42/infineon-applicationnote-linear-mode-operation-safe-operation-diagram-mosfets-applicationnotes-en.pdf).

- **Voltage columns** are increasing magnitudes in volts, starting at zero.
  They use the external drain–source, collector–emitter or anode–cathode voltage.
- Each **current row** gives allowed amperes at those same voltage columns.
  Space or comma separators and SI suffixes are accepted.
- The **DC row** is optional unless continuous operation is selected. Leaving
  **Selected pulse width** empty selects DC.
- Pulse rows must have increasing positive durations. Allowed current cannot
  increase with duration at any voltage column. A DC row cannot exceed the
  longest pulse row.
- The selected pulse width must cover the entire checked window, including gaps.
  To inspect one pulse, set **Start checks at** and **Stop time** around it and
  supply a curve appropriate for its initial thermal condition. Multiple pulses
  in the window do not each receive a fresh single-pulse allowance.
- The existing maximum-current field remains an independent cap. At each sample
  the allowed current is the smaller of that cap and the curve value. Current
  polarity and duration-screening settings retain their usual meaning.

Voltage/current interpolation can be linear or logarithmic. A voltage segment
touching zero in any row uses linear interpolation for every row, preserving
their duration ordering between columns. Pulse selection defaults to the next longer
supplied duration; optional interpolation uses logarithmic duration and current.
A selected duration shorter than the first row uses that first row. Above the
longest duration, a DC row is required and used without extrapolation. Above the
last voltage column, allowed current is zero; retain a separate voltage rule to
check blocking-voltage ratings even when current is zero.

There may be 2–1,024 voltage columns and up to 128 pulse rows. Every current must
be finite and nonnegative, with at least one positive value in each row. The
origin is explicit: enter the zero-voltage allowance appropriate for the supplied
boundary instead of extending a plotted log-axis curve automatically.

For example, these are **synthetic format values, not device ratings**:

| Duration | 0 V | 1 V | 10 V |
| --- | ---: | ---: | ---: |
| 1 µs | 0 A | 4 A | 0.4 A |
| 100 µs | 0 A | 2 A | 0.2 A |
| DC | 0 A | 1 A | 0.1 A |

Results retain the authored source, conditions, all curve rows, selected duration
and interpolation settings. The stress plot shows the allowed-current trace at
each time. CSV exports include terminal-voltage samples, actual current, allowed
current, the independent cap and the complete curve definition. Printed results
include the supplied curve rows. Saved results are checked by recomputing each
limit from their retained voltage samples.
