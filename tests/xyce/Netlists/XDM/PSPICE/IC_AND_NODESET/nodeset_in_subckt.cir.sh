#!/usr/bin/env perl

use XdmCommon;

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script  (This is NEVER used and can be ignored)
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$CIRFILE=$ARGV[3]; 
#$GOLDPRN=$ARGV[4];

# Comparison tolerances
$absTol = 1.0e-5;
$relTol = 1.0e-3;
$zeroTol = 1.0e-10;

# This is in here, so I can also search on a string in the translated Xyce netlist.
# It does have the side benefit of checking what the xdm return code is though.
@xdmOutputSearchStrings = ("SUCCESS: xdm completion status flag = 0");
$xdmOutputSearchStringsPtr=\@xdmOutputSearchStrings;

# Note: must escape ( ) and * and . with \\ to get the CheckError() function to work.
# Must escape a double quote " with a single backslash \
# Do not have to escape { } or _.
# Strings to test for in the translated Xyce netlist.  Goal is to check that the
# .NODESET statement is not moved to the top-level of the circuit in the
# translated Xyce netlist.
@translatedXyceNetlistSearchStrings = (".NODESET V\\(in\\_1\\)=5",
    ".NODESET V\\(X2:in\\_1\\)=0",
    ".NODESET V\\(X3:in\\_1\\)=0",
    ".NODESET V\\(in\\_1\\)=5",
    ".NODESET V\\(in\\_8\\)=\\{ic\\_test}",
    ".NODESET V\\(in\\_1\\)=\\{ic\\_test}");
$translatedXyceNetlistSearchStringsPtr=\@translatedXyceNetlistSearchStrings;

XdmCommon::verifyXDMtranslation($XYCE,$XYCE_VERIFY,$CIRFILE,"pspice",$absTol,$relTol,$zeroTol,
                                $verbose,$xdmOutputSearchStringsPtr,
                                $translatedXyceNetlistSearchStringsPtr);

