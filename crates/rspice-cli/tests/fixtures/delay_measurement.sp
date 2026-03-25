* Delay measurement fixture

V1 in 0 PULSE(0 5 50u 1u 1u 200u 500u)
R1 in out 1k
C1 out 0 100n

.MEAS TRAN prop_delay TRIG V(in) VAL=2.5 RISE=1 TARG V(out) VAL=2.5 RISE=1
.TRAN 1u 600u
.end
