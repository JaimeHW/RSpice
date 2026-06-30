* The classic RC filter!
V1 net1 0 DC 0 SIN(0 1 1k)
R1 net1 net2 1
C2 net2 0 0.1m
.tran 1u 2m
.print tran V(*) I(*)
.END
