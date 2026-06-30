# This file is used to test the use of Xyce command line
# options, such as -o commandLine.prnOutput, in the
# Python initialize() method
import sys
from xyce_interface import xyce_interface

# this calls the xyce_interface.open() method to
# make a xyce object

libDirectory = sys.argv[1]
xyceObj = xyce_interface(libdir=libDirectory)
print( xyceObj )

argv= ['-o','commandLine.prnOutput','commandLine.cir']
print( "calling initialize with argv = %s, %s, %s" % (argv[0], argv[1], argv[2]) )

result = xyceObj.initialize(argv)
print( "return value from initialize is %d" % result )

print( "Calling runSimulation..." )
result = xyceObj.runSimulation()
print( "return value from runSimulation is %d" % result )

print( "calling close")
xyceObj.close()


