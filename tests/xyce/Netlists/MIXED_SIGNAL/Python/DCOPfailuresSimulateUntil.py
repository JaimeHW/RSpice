# This file is used to test the behavior of of the Python 
# method simulateUntil() for a netlist that has a DCOP
# failure.
import sys
from xyce_interface import xyce_interface

# this calls the xyce_interface.open() method to
# make a xyce object

libDirectory = sys.argv[1]
xyceObj = xyce_interface(libdir=libDirectory)
print( xyceObj )

argv= ['DCOPfailures.cir']
print( "calling initialize with netlist %s" % argv[0] )

result = xyceObj.initialize(argv)
print( "return value from initialize is %d" % result )

steps = range(0,1)
for i in steps:
  requested_time = 0.0 + (i+1) * 0.1
  print( "Calling simulateUntil for requested_time = %.3f" % requested_time )
  actual_time = 0.0
  (result, actual_time) = xyceObj.simulateUntil( requested_time )
  print( "simulateUntil status = %d and actual_time = %.3f" % (result, actual_time) )

print( "calling close")
xyceObj.close()

