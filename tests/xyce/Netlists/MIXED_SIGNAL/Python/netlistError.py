# This file is used to test how xyce_interface.py
# behaves when the Xyce netlist has an error in it.
# In this case, Xyce will error out during netlist
# parsing.
import sys
from xyce_interface import xyce_interface

# this calls the xyce_interface.open() method to
# make a xyce object

libDirectory = sys.argv[1]
xyceObj = xyce_interface(libdir=libDirectory)
print( xyceObj )

argv= ['netlistError.cir']
print( "calling initialize with netlist %s" % argv[0] )

result = xyceObj.initialize(argv)
# the rest of this .py file will not be reached
print( "return value from initialize is %d" % result )

print( "Calling runSimulation..." )
result = xyceObj.runSimulation()
print( "return value from runSimulation is %d" % result )

print( "calling close")
xyceObj.close()

