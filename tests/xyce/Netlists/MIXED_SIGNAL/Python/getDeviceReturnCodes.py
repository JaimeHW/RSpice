# This netlist is used to test the return codes for the
# Python methods getDeviceNames() and getDACDeviceNames()
# for the cases of:
#
#    1) Success (1) for a valid non-Y device type (B) that
#       exists in the netlist.
#
#    2) Success for a valid Y device type (e.g. YADC) that
#       exists in the netlist.
#
#    3) Success for a valid U device type (e.g. BUF) that
#       exists in the netlist.
#
#    4) failure (0) since this netlist has no YDAC devices,
#       but YADC is a valid Y device type.
#
#    5) failure if the device type (BOGO) does not exist
#       in Xyce.
#
#    6) Failure for a valid non-Y device type (e.g. I) that
#       does not exist in the netlist.
#
# It also tests the getTotalNumDevices(), getAllDeviceNames(),
# checkDeviceParamName() and getDeviceParamVal() methods for both
# valid and invalid device names and parameter names.

import sys
from xyce_interface import xyce_interface

# this calls the xyce_interface.open() method to
# make a xyce object

libDirectory = sys.argv[1]
xyceObj = xyce_interface(libdir=libDirectory)
print( xyceObj )

argv= ['-quiet','getDeviceReturnCodes.cir']
print( "calling initialize with netlist %s" % argv[1] )

result = xyceObj.initialize(argv)
print( "return value from initialize is %d" % result )

print( "Calling runSimulation..." )
result = xyceObj.runSimulation()
print( "return value from runSimulation is %d" % result )

# call should succeed since the netlist does have a B-source
(result, names) = xyceObj.getDeviceNames("B")
print( "return value from getDeviceNames for B model group is %d" % result )
print( names )

# use mixed case
(result, names) = xyceObj.getDeviceNames("Yadc")
print( "return value from getDeviceNames for YADC model group is %d" % result )
print( names )

(result, names) = xyceObj.getDeviceNames("BUF")
print( "return value from getDeviceNames for BUF model group is %d" % result )
print( names )

# Also test these methods when they should return 0 (for failure).
# There are no YDAC or I devices in the netlist
(result, names) = xyceObj.getDeviceNames("YDAC")
print( "return value from getDeviceNames for YDAC model group is %d" % result )

(result, DACnames) = xyceObj.getDACDeviceNames()
print( "return value from getDACDeviceNames is %d" % result )

(result, names) = xyceObj.getDeviceNames("BOGO")
print( "return value from getDeviceNames for BOGO model group is %d" % result )

(result, names) = xyceObj.getDeviceNames("I")
print( "return value from getDeviceNames for I model group is %d" % result )

(result, numDevices, maxDeviceNameLength) = xyceObj.getTotalNumDevices()
print( "return value from getTotalNumDevices is %d" % result )
print( "Total number devices and max name length are %d %d" % (numDevices, maxDeviceNameLength) )

(result, names) = xyceObj.getAllDeviceNames()
print( "return value from getAllDeviceNames is %d" % result )
print( names )

# valid devices and parameter.  Use mixed case.
result = xyceObj.checkDeviceParamName("R1:r")
print ("Return value for checkDeviceParamName for R1:R is %d" % result)

result = xyceObj.checkDeviceParamName("YADC!adc1:WIDTH")
print ("Return value for checkDeviceParamName for YADC!ADC1:WIDTH is %d" % result)

# test invalid device and invalid parameter
result = xyceObj.checkDeviceParamName("RBOGO:R")
print ("Return value for checkDeviceParamName for RBOGO:R is %d" % result)

result = xyceObj.checkDeviceParamName("R1:BOGO")
print ("Return value for checkDeviceParamName for R1:BOGO is %d" % result)

# get param values for valid devices.  Use some mixed case
(result, value) = xyceObj.getDeviceParamVal("r1:R")
print ("Return value for getDeviceParamVal for R1:R is %d" % result)
print ("R1:R value is %d" % value)

(result, value) = xyceObj.getDeviceParamVal("YADC!adc1:WIDTH")
print ("Return value for getDeviceParamVal for YADC!ADC1:WIDTH is %d" % result)
print ("YADC!ADC1:WIDTH value is %d" % value)

# test invalid device and invalid parameter
(result, value) = xyceObj.getDeviceParamVal("RBOGO:R")
print ("Return value for getDeviceParamVal for RBOGO:R is %d" % result)
print ("RBOGO:R value is %d" % value)

(result, value) = xyceObj.getDeviceParamVal("R1:BOGO")
print ("Return value for getDeviceParamVal for R1:BOGO is %d" % result)
print ("R1:BOGO value is %d" % value)

print( "calling close")
xyceObj.close()

