# This netlist is used to test the Python setADCWidths() and 
# getADCWidths() methods for both valid and invalid YADC 
# device names.  It also tests the getTimeStatePairsADCLimitData() 
# method for valid YADC names.  It also tests that the width 
# value on the instance line takes precedence over the width 
# value in the model card.  This netlist also tests the 
# getADCMap() method when the netlist has YADC devices in it.
# It also tests the getNumDevices() method when the netlist has
# YADC devices in it.

# file will be used to compare timeArray and stateArray against
# a gold standard
f= open("ADCStateShortHist.cir.TVarrayData","w+")

import sys
from xyce_interface import xyce_interface

# this calls the xyce_interface.open() method to
# make a xyce object

libDirectory = sys.argv[1]
xyceObj = xyce_interface(libdir=libDirectory)
print( xyceObj )

argv= ['ADCStateShortHist.cir']
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

# get ADC widths.  This verifies that the width value on the
# instance line takes precedence over the width value in the
# model card.
(result, adcWidths) = xyceObj.getADCWidths(names)
print( adcWidths )

# get all instance parameters for all ADCs
(result, names1, widths, resistances, upperVLimits, lowerVLimits, settlingTimes) = xyceObj.getADCMap()
print( "return value from getADCMap is %d" % result )
print( names1)
print( widths) 
print( ['%.2e'% r for r in resistances] ) 
print( upperVLimits) 
print( lowerVLimits) 
print( ['%.2e'% s for s in settlingTimes] )


# Redo getADCMap() call after the widths have been updated.
# Check that the widths values are correctly reported.
(result, names1, widths, resistances, upperVLimits, lowerVLimits, settlingTimes) = xyceObj.getADCMap()
print( "return value from getADCMap is %d" % result )
print( names1)
print( widths) 



# get ADC widths, with two valid names
(result, adcWidths) = xyceObj.getADCWidths(names)
print( "return value from getADCWidths is %d" % result )
print( adcWidths )


stepSize = 1e-5
steps = range(0,10)
for i in steps:
  requested_time = 0.0 + (i+1) * stepSize
  print( "Calling simulateUntil for requested_time = %.5f" % requested_time )
  actual_time = 0.0
  (result, actual_time) = xyceObj.simulateUntil( requested_time )
  print( "simulateUntil status = %d and actual_time = %.5f" % (result, actual_time) )
  (result, numPoints1) = xyceObj.getTimeVoltagePairsADCsz()
  print( "number of points returned by getTimeVoltagePairsADCsz is %d" % numPoints1 )
  
  (result, ADCnames, numADCnames, numPointsArray, timeArray, stateArray) = xyceObj.getTimeStatePairsADCLimitData(maxNumTimeStatePairs=10)
  print( "return value from getTimeStatePairsADCLimitData is %d" % result )
  print( "number of ADC names returned by getTimeStatePairsADC is %d" % numADCnames )
  print( ADCnames )
  # output to stdout (for human readability)
  
  # numPointsArray is the  number of time points for each ADC.  
  for adcnum in range(0, numADCnames ):
    print( "number of points for ADC %d by getTimeStatePairsADCLimitData is %d" % (adcnum, numPointsArray[adcnum]) )
    for j in range(0, numPointsArray[adcnum]):  
      print( "ADC %d: Time and state array %d values are %.3e %d" %((adcnum+1), j, timeArray[adcnum][j] , stateArray[adcnum][j]) )
      f.write( "ADC %d: Time and state array %d values are %.3e %d\n" %((adcnum+1), j, timeArray[adcnum][j] , stateArray[adcnum][j]) )
  
print( "calling close")
xyceObj.close()

# close file also
f.close()

