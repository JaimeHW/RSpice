# Certification test for SON Bug 1301

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

argv= ['bug_1301_son.cir']
print( "calling initialize with netlist %s" % argv[0] )

result = xyceObj.initialize(argv)
print( "return value from initialize is %d" % result )

(result, names) = xyceObj.getDeviceNames("R")
print( "return value and names for all R devices is %d %s" % (result,names) )

(result, names) = xyceObj.getDeviceNames("B")
print( "return value and names for all B devices is %d %s" % (result, names) )

(result, names) = xyceObj.getDeviceNames("C")
print( "return value and names for all C devices is %d %s" % (result, names) )

(result, names) = xyceObj.getDeviceNames("D")
print( "return value and names for all D devices is %d %s" % (result, names) )

(result, names) = xyceObj.getDeviceNames("G")
print( "return value and names for all G devices is %d %s" % (result, names) )

(result, names) = xyceObj.getDeviceNames("J")
print( "return value and names for all J devices is %d %s" % (result, names) )

(result, names) = xyceObj.getDeviceNames("K")
print( "return value and names for all K devices is %d %s" % (result, names) )

(result, names) = xyceObj.getDeviceNames("L")
print( "return value and names for all L devices is %d %s" % (result, names) )

(result, names) = xyceObj.getDeviceNames("M")
print( "return value and names for all M devices is %d %s" % (result, names) )

(result, names) = xyceObj.getDeviceNames("X")
print( "return value and names for all X devices is %d %s" % (result, names) )

(result, names) = xyceObj.getDeviceNames("YLIN")
print( "return value and names for all YLIN devices is %d %s" % (result, names) )

(result, names) = xyceObj.getDeviceNames("Z")
print( "return value and names for all D devices is %d %s" % (result, names) )

print( "calling close")
xyceObj.close()
