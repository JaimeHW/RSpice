# This netlist is used to test the error messages from the Python setADCWidths() 
# and updateTimeVoltagePairs() methods, generated from within xyce_interface.py, 
# when their input array sizes are not the same size. 
import sys
from xyce_interface import xyce_interface

# this calls the xyce_interface.open() method to
# make a xyce object

libDirectory = sys.argv[1]
xyceObj = xyce_interface(libdir=libDirectory)
print( xyceObj )

argv= ['interfaceErrorMessages.cir']
print( "calling initialize with netlist %s" % argv[0] )

result = xyceObj.initialize(argv)
print( "return value from initialize is %d" % result )

# get ADC and DAC names
(result, adcNames) = xyceObj.getDeviceNames("YADC")
(result, dacNames) = xyceObj.getDeviceNames("YDAC")

# set ADC widths array to have length of 2, which will cause an error in
# the Python interface since adcNames only has length 1.
widths=[3,2]
result = xyceObj.setADCWidths(adcNames,widths)
print( "return value from setADCWidths with mismatched name and width arrays is %d" % result )

# time and voltage arrays have mismatched lengths, which will cause an error
# in the Python interface
timeArray = [ 0.0, 1e-5, 2e-5]
voltageArray= [ 0.0, 0.0] 
result = xyceObj.updateTimeVoltagePairs( dacNames[0], timeArray, voltageArray )
print( "return value from updateTimeVoltagePairs with mismatched time and voltage arrays is %d" % result )

print( "calling close")
xyceObj.close()


