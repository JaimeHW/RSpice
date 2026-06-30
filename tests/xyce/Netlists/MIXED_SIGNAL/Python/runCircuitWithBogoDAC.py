# This file is used to test the Python method 

# This file is used to test the Python updateTimeVoltagePairs() 
# method when it is invoked with a DAC name that does not exist 
# in the netlist.

import sys
from xyce_interface import xyce_interface

# this calls the xyce_interface.open() method to
# make a xyce object

libDirectory = sys.argv[1]
xyceObj = xyce_interface(libdir=libDirectory)
print( xyceObj )

argv= ['runCircuitWithBogoDAC.cir']
print( "calling initialize with netlist %s" % argv[0] )

result = xyceObj.initialize(argv)
print( "return value from initialize is %d" % result )

# get DAC names;
(result, DACnames) = xyceObj.getDACDeviceNames()
print( "return value from getDACDeviceNames is %d" % result )
print( DACnames )

#
# A bug in the DAC device (put there for Habinero support) only takes
# the last time in the time voltage pairs list if the current sim time is 0.
# So simulate a bit first.
requested_time = 1.0e-10
(result, actual_time) = xyceObj.simulateUntil( requested_time )
print( "simulationUntil status = %d and actual_time = %f" % (result, actual_time) )

#set up the DAC to pulse twice
timeArray = [ 0.0, 1e-5, 2e-5, 3e-5, 4e-5, 6e-5, 7e-5, 8.0e-5, 9e-5 ]
voltageArray= [ 0.0, 0.0,    3.0,    3.0,    0.0,    0.0,    3.0,    3.0,    0.0    ] 

steps = range(0,10)
total_sim_time = 1e-4
for i in steps:
  # the DAC name 'BOGODAC' does not exist in the netlist
  result = xyceObj.updateTimeVoltagePairs( 'BOGODAC', timeArray, voltageArray )
  print ( "return value from updateTimeVoltagePairs is %d" % result )
  requested_time = 0.0 + (i+1) * 0.1 * total_sim_time
  print( "Calling simulateUntil for requested_time %f" % requested_time )
  actual_time = 0.0
  (result, actual_time) = xyceObj.simulateUntil( requested_time )
  print( "simulateUntil status = %d and actual_time = %f" % (result, actual_time) )
  
print( "calling close")
xyceObj.close()

