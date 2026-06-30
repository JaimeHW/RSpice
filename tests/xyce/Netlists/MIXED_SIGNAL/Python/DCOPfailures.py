# This file is used to test the Python simulateUntil() method

from xyce_interface import xyce_interface

# this calls the xyce_interface.open() method to
# make a xyce object

xyceObj = xyce_interface()
print( xyceObj )

argv= ['DCOPfailures_simulateUntil.cir']
print( "calling initialize with, %s" % argv[0] )

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

