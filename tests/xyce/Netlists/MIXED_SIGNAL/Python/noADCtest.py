# Test that the Python methods setADCWidths(), getADCWidths(), getADCMap()
# getTimeStatePairsADC(), getTimeVoltagePairsADC() and getNumDevices()
# return 0 if there are no ADCs in the netlist.  Also, that the Python
# interface doesn't crash in that case.  Note that the .py file
# only initializes the N_CIR_Xyce object.  It does not
# run the simulation.  So, there is no Xyce .prn file
# made by this test.
import sys
from xyce_interface import xyce_interface

# this calls the xyce_interface.open() method to
# make a xyce object

libDirectory = sys.argv[1]
xyceObj = xyce_interface(libdir=libDirectory)
print( xyceObj )

argv= ['noADCtest.cir']
print( "calling initialize with netlist %s" % argv[0] )

result = xyceObj.initialize(argv)
print( "return value from initialize is %d" % result )

# get number of ADCs and max name length
(result, numDevices, maxDeviceNameLength) = xyceObj.getNumDevices("YADC")
print( "return value from getNumDevices is %d" % result )
print( "Num devices and max device name length are %d %d" %(numDevices, maxDeviceNameLength) )

# get ADC names
(result, names) = xyceObj.getDeviceNames("YADC")
print( "return value from getDeviceNames is %d" % result )
print( names )

#set ADC widths
width=[]
result = xyceObj.setADCWidths(names,width)
print( "return value from setADCWidths is %d" % result )

#get ADC widths
(result, adcWidths) = xyceObj.getADCWidths(names)
print( "return value from getADCWidths is %d" % result )
print( adcWidths)

#get ADC instance parameters
(result, names1, widths, resistances, upperVLimits, lowerVLimits, settlingTimes) = xyceObj.getADCMap()
print( "return value from getADCMap is %d" % result )
print( names1)
print( widths) 
print( resistances) 
print( upperVLimits) 
print( lowerVLimits) 
print( settlingTimes)

(result, ADCnames, numADCnames, numPoints, timeArray, voltageArray) = xyceObj.getTimeVoltagePairsADC()
print( "return value from getTimeVoltagePairsADC is %d" % result )
print( "number of ADC names returned by getTimeVoltagePairsADC is %d" % numADCnames )
print( "number of points returned by getTimeVoltagePairsADC is %d" % numPoints )

(result, ADCnames, numADCnames, numPoints, timeArray, stateArray) = xyceObj.getTimeStatePairsADC()
print( "return value from getTimeStatePairsADC is %d" % result )
print( "number of ADC names returned by getTimeStatePairsADC is %d" % numADCnames )
print( "number of points returned by getTimeStatePairsADC is %d" % numPoints )

print( "calling close")
xyceObj.close()

