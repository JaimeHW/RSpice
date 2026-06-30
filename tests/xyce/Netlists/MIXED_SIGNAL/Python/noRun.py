# This netlist is used to test the return codes from the
# Python initialize() method for the -norun case.
# Note: this test currently causes the Python interface
# to segfault.  See SON Bug 1065 for more details.

from xyce_interface import xyce_interface

# this calls the xyce_interface.open() method to
# make a xyce object

xyceObj = xyce_interface()
print( xyceObj )

argv= ['-norun','noRun.cir']
print( "calling initialize with option %s and netlist %s" % (argv[0], argv[1]) )

result = xyceObj.initialize(argv)
print( "return value from initialize is %d" % result )

# for the no-run case, result should equal 2
if result == 1:
  print( "Calling runSimulation..." )
  result = xyceObj.runSimulation()
  print( "return value from runSimulation is %d" % result )
  print( "calling close")
  xyceObj.close()


