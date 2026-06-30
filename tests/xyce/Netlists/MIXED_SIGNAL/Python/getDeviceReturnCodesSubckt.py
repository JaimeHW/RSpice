# This netlist is used to test the return codes for the
# Python methods getDeviceNames() and getDACDeviceNames()
# for the cases of devices in subcircuits:
#
#    1) Success (1) for a valid non-Y device type (L)
#       that exists in the netlist.
#
#    2) Success for a valid Y device types (e.g. YADC
#       and YDAC) that exist in the netlist.  This also
#       tests that just DAC works for a Y device.
#
#    3) Success for a valid U device type (e.g. BUF) that
#       exists in the netlist.
#
# It also tests the getTotalNumDevices(), getAllDeviceNames(),
# checkDeviceParamName() and getDeviceParamVal() methods with
# subcircuits.

import sys
from xyce_interface import xyce_interface

# this calls the xyce_interface.open() method to
# make a xyce object

libDirectory = sys.argv[1]
xyceObj = xyce_interface(libdir=libDirectory)
print( xyceObj )

argv= ['-quiet','getDeviceReturnCodesSubckt.cir']
print( "calling initialize with netlist %s" % argv[1] )

result = xyceObj.initialize(argv)
print( "return value from initialize is %d" % result )

print( "Calling runSimulation..." )
result = xyceObj.runSimulation()
print( "return value from runSimulation is %d" % result )

# call should succeed since the netlist does have an L device (inductor)
(result, names) = xyceObj.getDeviceNames("L")
print( "return value from getDeviceNames for L model group is %d" % result )
print( names )

(result, names) = xyceObj.getDeviceNames("YADC")
print( "return value from getDeviceNames for YADC model group is %d" % result )
print( names )

# This call should work without the "Y" in the modelGroupName
(result, names) = xyceObj.getDeviceNames("DAC")
print( "return value from getDeviceNames for YDAC model group is %d" % result )
print( names )

(result, names) = xyceObj.getDACDeviceNames()
print( "return value from getDACDeviceNames for YDAC model group is %d" % result )
print( names )

(result, names) = xyceObj.getDeviceNames("BUF")
print( "return value from getDeviceNames for BUF model group is %d" % result )
print( names )

(result, numDevices, maxDeviceNameLength) = xyceObj.getTotalNumDevices()
print( "return value from getTotalNumDevices is %d" % result )
print( "Total number devices and max name length are %d %d" % (numDevices, maxDeviceNameLength) )

(result, names) = xyceObj.getAllDeviceNames()
print( "return value from getAllDeviceNames is %d" % result )
print( names )

result = xyceObj.checkDeviceParamName("X1:R2:R")
print ("Return value for checkDeviceParamName for X1:R2:R is %d" % result)

(result, value) = xyceObj.getDeviceParamVal("X1:R2:R")
print ("Return value for getDeviceParamVal for X1:R2:R is %d" % result)
print ("X1:R2:R value is %d" % value)

print( "calling close")
xyceObj.close()

