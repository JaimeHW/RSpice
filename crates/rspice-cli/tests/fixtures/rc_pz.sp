* RC Low-Pass Filter for Pole-Zero Analysis
* Single-pole RC filter with f_3dB = 159.15 kHz
* Pole at s = -1e6 rad/s

* Input source
Vin in 0 DC 1 AC 1

* R = 1k ohm
R1 in out 1k

* C = 1nF
C1 out 0 1n

* Expected: One pole at s = -1/(R*C) = -1e6 rad/s
* DC gain: 1

.END
