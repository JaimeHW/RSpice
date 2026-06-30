#!/usr/bin/perl
$n_nands=$ARGV[0];


print <<ENDOFPRINT;
THIS CIRCUIT TESTS A CHAIN of 2-INPUT NAND GATES WITH BJT MODEL
* This is a 2-input NAND.  Input 1 is at node 1, while input 2 is at node 2.
* VIN1 and VIN2 are the input signals.  Both signals have to be high to have
* a low ouput. 
*
* This circuit has been intentionally modified so that the BJT capacitors
* are not specified in the model card, rendering their values to zero.  It
* has also been modified to have fast rise times (0.5ns) in the two 
* pulsed sources.
*
* These modifications will cause the circuit to have difficulty (fail with 
* time-step-too small)  in transient, when the transient reaches a pulse 
* transition.   This can be remedied by applying mincap.
*

** Analysis setup **
*
.tran 20ns 10us

* this circuit will run if mincap is set.
*.options device mincap=1nf

VDD 	\$G_VDDNODE	0	5V
VIN1          1 0  5V PULSE (5V 0V 3us 0.5ns 0.5ns 4us 8.275us)
VIN2          2 0  0V PULSE (0V 5V 2us 0.5ns 0.5ns 4us 8.275us)

XN1  1  2  OUT1  NAND
ENDOFPRINT
for ($i=1;$i<$n_nands;$i++)
{
    $n1=$i;
    $n2=$i+1;

    print "XN$n2  1   OUT$n1  OUT$n2  NAND\n";

}
print ".print tran V(OUT1) v(OUT2) V(OUT$n1) v(OUT$n2) V(1) V(2)\n";

print <<ENDOFPRINT;

.subckt nand   INP1  INP2  VOUT 
RB1	\$G_VDDNODE	VB1	4K
QIN1	VB2	VB1	INP1	NPN
QIN2	VB2	VB1	INP2	NPN
RC2	\$G_VDDNODE	VC2	1.6K
Q3	VC2	VB2	VB3	NPN
RC3	\$G_VDDNODE	VOUT	4K
Q4	VOUT	VB3	0	NPN
RB3	VB3	0	1K
.ENDS

**************************
.MODEL NPN NPN (           IS     = 3.97589E-14
+ BF     = 195.3412        NF     = 1.0040078       VAF    = 53.081
+ IKF    = 0.976           ISE    = 1.60241E-14     NE     = 1.4791931
+ BR     = 1.1107942       NR     = 0.9928261       VAR    = 11.3571702
+ IKR    = 2.4993953       ISC    = 1.88505E-12     NC     = 1.1838278
+ RB     = 56.5826472      IRB    = 1.50459E-4      RBM    = 5.2592283
+ RE     = 0.0402974       RC     = 0.4208          XTI    = 5.8315
+ EG     = 1.11            XTB    = 1.6486                            )
**************************
.END
ENDOFPRINT
