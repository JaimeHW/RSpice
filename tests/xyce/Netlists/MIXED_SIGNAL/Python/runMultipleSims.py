# This file is used to test the Python initialize(),
# runSimulation() and close() methods when there are
# multiple xyce objects.  Note that -quiet is used to
# suppress the Xyce time-progress output.
import sys
from xyce_interface import xyce_interface

# this calls the xyce_interface.open() method to
# make a xyce object

libDirectory = sys.argv[1]
xyceObj1 = xyce_interface(libdir=libDirectory)
print( xyceObj1)

xyceObj2 = xyce_interface(libdir=libDirectory)
print( xyceObj2)

argv1= ['-quiet','runMultipleSims1.cir']
print( "calling initialize for xyceObj1 with netlist %s" % argv1[1] )
result = xyceObj1.initialize(argv1)
print( "return value from initialize for xyceObj1 is %d" % result )

argv2= ['-quiet','runMultipleSims2.cir']
print( "calling initialize for xyceObj2 with netlist %s" % argv2[1] )
result = xyceObj2.initialize(argv2)
print( "return value from initialize for xyceObj2 is %d" % result )

print( "Calling runSimulation for xyceObj1..." )
result = xyceObj1.runSimulation()
print( "return value from runSimulation for xyceObj1 is %d" % result )

print( "Calling runSimulation for xyceObj2..." )
result = xyceObj2.runSimulation()
print( "return value from runSimulation for xyceObj2 is %d" % result )

print( "calling close for xyceObj1")
xyceObj1.close()

print( "calling close for xyceObj2")
xyceObj2.close()

