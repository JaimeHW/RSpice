* Named RC transient with measurement

V1 in 0 5
R1 in out 1k
C1 out 0 1u

.MEAS TRAN vout_max MAX V(out)
.TRAN 10u 2m
.end
