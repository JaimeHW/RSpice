* RC Lowpass Filter - AC Analysis Test
* Cutoff frequency = 1/(2*pi*R*C) = 159.15 Hz

V1 1 0 AC 1 0
R1 1 2 1k
C1 2 0 1u

.AC DEC 10 1 10k
.end
