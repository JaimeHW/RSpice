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

# Note: must escape ( ) and * and . with \\ to get the CheckError() function to work.
# Must escape a double quote " with a single backslash \
# Do not have to escape { } or _.
# Strings to test for in the diagnostic output from xdm.
@xdmOutputSearchStrings = ("Converting ITL1 into NONLIN MAXSTEP",
  "Converting ITL4 into NONLIN-TRAN MAXSTEP");
$xdmOutputSearchStringsPtr=\@xdmOutputSearchStrings;

# these strings should be in the translated Xyce netlist
@translatedXyceNetlistSearchStrings= (".OPTIONS NONLIN MAXSTEP=200",
  ".OPTIONS NONLIN-TRAN MAXSTEP=20"
);
$translatedXyceNetlistSearchStringsPtr=\@translatedXyceNetlistSearchStrings;

XdmCommon::verifyXDMtranslation($XYCE,$XYCE_VERIFY,$CIRFILE,"pspice",$absTol,$relTol,$zeroTol,
                                $verbose,$xdmOutputSearchStringsPtr,
                                $translatedXyceNetlistSearchStringsPtr);
