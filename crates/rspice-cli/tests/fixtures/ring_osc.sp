* Ring Oscillator for PSS Analysis Test
* 3-stage CMOS inverter ring oscillator
* Expected oscillation frequency ~1 GHz

.param VDD=1.8

* Power supply
VDD vdd 0 DC {VDD}

* Stage 1
M1 out1 in vdd vdd pmos W=200n L=100n
M2 out1 in 0 0 nmos W=100n L=100n

* Stage 2  
M3 out2 out1 vdd vdd pmos W=200n L=100n
M4 out2 out1 0 0 nmos W=100n L=100n

* Stage 3 (feedback to input)
M5 in out2 vdd vdd pmos W=200n L=100n
M6 in out2 0 0 nmos W=100n L=100n

* Load capacitors
C1 out1 0 50f
C2 out2 0 50f
C3 in 0 50f

* MOSFET models
.model nmos nmos (VTH0=0.4 KP=200u)
.model pmos pmos (VTH0=-0.4 KP=100u)

.END
