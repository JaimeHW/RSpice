* Simple RC test 

.subckt rc 1 2
R 1 2 1
C 2 0 1e-9
.ends

.subckt rc10 1 11
X1 1 2 rc
X2 2 3 rc
X3 3 4 rc
X4 4 5 rc
X5 5 6 rc
X6 6 7 rc
X7 7 8 rc
X8 8 9 rc
X9 9 10 rc
X10 10 11 rc
.ends

.subckt rc100 1 11
X1 1 2 rc10
X2 2 3 rc10
X3 3 4 rc10
X4 4 5 rc10
X5 5 6 rc10
X6 6 7 rc10
X7 7 8 rc10
X8 8 9 rc10
X9 9 10 rc10
X10 10 11 rc10
.ends

.include rcd_ladder_lib.inc
