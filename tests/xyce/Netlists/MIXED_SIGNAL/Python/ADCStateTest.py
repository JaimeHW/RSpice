# This netlist is used to test the getTimeStatePairsADC() 
# method for valid YADC names.  It also tests that the width 
# value on the instance line takes precedence over the width 
# value in the model card, as set by the device models.  It
# also tests the setting of the quantization levels works
# correctly in the constructor for the YADC device.

import sys
# file will be used to compare timeArray and stateArray against
# a gold standard
f= open("ADCStateTest.cir.TSarrayData","w+")

from xyce_interface import xyce_interface

# this calls the xyce_interface.open() method to
# make a xyce object

libDirectory = sys.argv[1]
xyceObj = xyce_interface(libdir=libDirectory)
print( xyceObj )

argv= ['ADCStateTest.cir']
print( "calling initialize with netlist %s" % argv[0] )

result = xyceObj.initialize(argv)
print( "return value from initialize is %d" % result )

# get ADC names;
(result, names) = xyceObj.getDeviceNames("YADC")
print( "return value from getDeviceNames is %d" % result )
print( names )

# get the ADC widths.  This is hard-coded for two ADCs.  The ordering in
# widths seems reversed but names came back as:
#   ['YADC!ADC2', 'YADC!ADC1']
widths=[3,2]
(result,widths) = xyceObj.getADCWidths(names)
print( "return value from getADCWidths is %d" % result )
print( widths )

stepSize = 1e-5
steps = range(0,10)
for i in steps:
  requested_time = 0.0 + (i+1) * stepSize
  print( "Calling simulateUntil for requested_time = %.5f" % requested_time )
  actual_time = 0.0
  (result, actual_time) = xyceObj.simulateUntil( requested_time )
  print( "simulateUntil status = %d and actual_time = %.5f" % (result, actual_time) )
  (result, ADCnames, numADCnames, numPoints, timeArray, stateArray) = xyceObj.getTimeStatePairsADC()
  print( "return value from getTimeStatePairsADC is %d" % result )
  print( "number of ADC names returned by getTimeStatePairsADC is %d" % numADCnames )
  print( ADCnames )
  # output to stdout (for human readability)
  print( "number of points returned by getTimeStatePairsADC is %d" % numPoints )
  # numPoints is the maximum number of time points for an ADC.  Others may have fewer points
  # so for each time array returned for a given ADC search for the current time (within some error)
  for adcnum in range(0, numADCnames ):
    prevPoint = -1;
    currPoint = 0;
    for j in range(0, numPoints):
      if (abs(timeArray[adcnum][j] - requested_time) < 1.0e-11):
        currPoint=j
        prevPoint=j-1
        break
    
    # don't output index to -1 as it is NOT a valid timepoint.
    # output to file (for comparison against a gold standard)
    if( prevPoint != -1):
      print( "ADC %d: Time and state array %d values are %.3e %d" %((adcnum+1), prevPoint, timeArray[adcnum][prevPoint] , stateArray[adcnum][prevPoint]) )
      f.write( "ADC %d: Time and state array %d values are %.3e %d\n" %((adcnum+1), prevPoint, timeArray[adcnum][prevPoint] , stateArray[adcnum][prevPoint]) )
    
    print( "ADC %d: Time and state array %d values are %.3e %d" %((adcnum+1), currPoint, timeArray[adcnum][currPoint] , stateArray[adcnum][currPoint]) )
    f.write( "ADC %d: Time and state array %d values are %.3e %d\n" %((adcnum+1), currPoint, timeArray[adcnum][currPoint] , stateArray[adcnum][currPoint]) )

print( "calling close")
xyceObj.close()

# close file also
f.close()

