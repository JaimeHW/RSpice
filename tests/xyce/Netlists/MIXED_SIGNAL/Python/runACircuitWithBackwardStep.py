# This file is used to test the Python simulateUntil() method
# with a backward time step.  This will cause the simulation to
# emit a warning mesange and then run to completion, after that
# particular simulateUntil() call.

import sys

# import the Xyce libraries
from xyce_interface import xyce_interface

# this calls the xyce_interface.open() method to
# make a xyce object
if (len(sys.argv) > 1):
  libDirectory = sys.argv[1]
  xyceObj = xyce_interface(libdir=libDirectory)
else:
  xyceObj = xyce_interface()
print( xyceObj )

argv= ['runACircuitWithBackwardStep.cir']
print( "calling initialize with, %s" % argv[0] )

result = xyceObj.initialize(argv)
print( "return value from initialize is %d" % result )

steps = range(0,5)
for i in steps:
  requested_time = 0.0 + (i+1) * 0.1
  print( "Calling simulateUntil for requested_time = %f" % requested_time )
  actual_time = 0.0
  (result, actual_time) = xyceObj.simulateUntil( requested_time )
  print( "simulateUntil status = %d and actual_time = %f" % (result, actual_time) )
  if (result == 0):
    break

# Now step backwards, and see what happens.  The simulation should abort cleanly.
requested_time = requested_time - 0.1
print( "Calling simulateUntil for backwards requested_time = %f" % requested_time )
(result, actual_time) = xyceObj.simulateUntil( requested_time )
print( "simulateUntil status = %d and actual_time = %f" % (result, actual_time) )

# this statement will not be reached
print( "calling close")
xyceObj.close()
