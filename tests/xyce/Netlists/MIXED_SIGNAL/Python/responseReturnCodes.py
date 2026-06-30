# This netlist is used to test the return codes from the
# Python method checkResponseVarName() for the cases
# where the measure names are valid or invalid.  It also
# checks obtainResponse() for the case where the measure
# name is invalid.
import sys
from xyce_interface import xyce_interface

# this calls the xyce_interface.open() method to
# make a xyce object

libDirectory = sys.argv[1]
xyceObj = xyce_interface(libdir=libDirectory)
print( xyceObj )

argv= ['-quiet','responseReturnCodes.cir']
print( "calling initialize with netlist %s" % argv[1] )

result = xyceObj.initialize(argv)
print( "return value from initialize is %d" % result )

# Check return codes for valid (MAXV1) and invalid (BOGO) names.
# Used mixed case for the valid measure name.
result = xyceObj.checkResponseVarName('maxV1')
print( "return value from checkResponseVarName for MAXV1 is %d" % result )

result = xyceObj.checkResponseVarName('BOGO')
print( "return value from checkResponseVarName for BOGO is %d" % result )

print( "Calling runSimulation..." )
result = xyceObj.runSimulation()
print( "return value from runSimulation is %d" % result )

# check return codes for a non-existent measure name
(result,value) = xyceObj.obtainResponse('BOGO')
print( "return value from obtainResponse for BOGO is %d" % result )
print( "obtainResponse value for BOGO = %.3f" %value )

print( "calling close")
xyceObj.close()

