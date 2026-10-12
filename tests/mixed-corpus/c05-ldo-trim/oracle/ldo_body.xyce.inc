* C5 LDO body for the Xyce O5-O4 decks: net/ldo_body.inc with the switch
* model in Xyce's VSWITCH form (on above vt+vh = 1.0 V, off below
* vt-vh = 0.8 V; Xyce has no hysteretic SW model). Keep the two files in step.
* Models: oracle/models-bsim4.xyce.inc (NBSIM4/PBSIM4 at LEVEL 54).

* reference
vref vr0 0 0.5
rref vr0 vref 100k
cref vref 0 10p
* bias
ibias vin nb 10u
mb nb nb 0 0 NBSIM4 w=2u l=1u
* tail
mt tail nb 0 0 NBSIM4 w=4u l=1u
* input pair
m1 d1 fb tail 0 NBSIM4 w=8u l=1u
m2 og vref tail 0 NBSIM4 w=8u l=1u
* mirror
m3 d1 d1 vin vin PBSIM4 w=4u l=1u
m4 og d1 vin vin PBSIM4 w=4u l=1u
* loop probe
vprobe og og2 0
* pass device
mp vout og2 vin vin PBSIM4 w=10u l=0.2u m=60
* feedback: a set trim bit shorts its resistor, R_bot = 60k + sum (1 - b_k) R_k
rtop vout fb 100k
rb0 fb l1 60k
rt3 l1 l2 8k
rt2 l2 l3 4k
rt1 l3 l4 2k
rt0 l4 0 1k
* trim switches
s3 l1 l2 tr3 0 swt
s2 l2 l3 tr2 0 swt
s1 l3 l4 tr1 0 swt
s0 l4 0 tr0 0 swt
.model swt vswitch(von=1.0 voff=0.8 ron=10 roff=1e12)
