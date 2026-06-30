*Simple RCD test 

.MODEL DMOD D (IS=100FA)

.subckt rcd100 1 2
X1 1 3  rc100
X2 3 2 rc10 
D1 2 0 DMOD
.ends

