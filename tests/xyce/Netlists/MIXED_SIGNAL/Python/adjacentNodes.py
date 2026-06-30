# This netlist is used to test the Python methods getNumAdjNodesForDevice()
# and getAdjGIDsForDevice() for the cases of valid and invalid devices,
# including an R device in a subcircuit.  It also tests a valid Y device.

import sys

from xyce_interface import xyce_interface

# this calls the xyce_interface.open() method to
# make a xyce object
if (len(sys.argv) > 1):
  libDirectory = sys.argv[1]
  xyceObj = xyce_interface(libdir=libDirectory)
else:
  xyceObj = xyce_interface()
print( xyceObj )

argv= ['adjacentNodes.cir']
print( "calling initialize with netlist %s" % argv[0] )

result = xyceObj.initialize(argv)
print( "return value from initialize is %d" % result )

(result, numAdjNodes) = xyceObj.getNumAdjNodesForDevice("V1")
print ( "return value and numAdjNodes for V1 is %d %d" % (result, numAdjNodes) )

(result, GIDs) = xyceObj.getAdjGIDsForDevice("V1")
print ( "return value for getAdjGIDsForDevice for V1 is %d" % result )
print (GIDs)

(result, numAdjNodes) = xyceObj.getNumAdjNodesForDevice("R1")
print ( "return value and numAdjNodes is for R1 %d %d" % (result, numAdjNodes) )

(result, GIDs) = xyceObj.getAdjGIDsForDevice("R1")
print ( "return value for getAdjGIDsForDevice for R1 is %d" % result )
print (GIDs)

(result, numAdjNodes) = xyceObj.getNumAdjNodesForDevice("X1:R1")
print ( "return value and numAdjNodes for X1:R1 is %d %d" % (result, numAdjNodes) )

(result, GIDs) = xyceObj.getAdjGIDsForDevice("X1:R1")
print ( "return value for getAdjGIDsForDevice for X1:R1 is %d" % result )
print (GIDs)

(result, numAdjNodes) = xyceObj.getNumAdjNodesForDevice("r3")
print ( "return value and numAdjNodes for r3 is %d %d" % (result, numAdjNodes) )

(result, GIDs) = xyceObj.getAdjGIDsForDevice("r3")
print ( "return value for getAdjGIDsForDevice for r3 is %d" % result )
print (GIDs)

(result, numAdjNodes) = xyceObj.getNumAdjNodesForDevice("YADC!adc1")
print ( "return value and numAdjNodes for YADC!adc1 is %d %d" % (result, numAdjNodes) )

(result, GIDs) = xyceObj.getAdjGIDsForDevice("YADC!adc1")
print ( "return value for getAdjGIDsForDevice for YADC!adc1 is %d" % result )
print (GIDs)

# test error cases
(result, numAdjNodes) = xyceObj.getNumAdjNodesForDevice("RBOGO")
print ( "return value and numAdjNodes for RBOGO is %d %d" % (result, numAdjNodes) )

(result, GIDs) = xyceObj.getAdjGIDsForDevice("RBOGO")
print ( "return value for getAdjGIDsForDevice for RBOGO is %d" % result )
print (GIDs)

print( "calling close")
xyceObj.close()
