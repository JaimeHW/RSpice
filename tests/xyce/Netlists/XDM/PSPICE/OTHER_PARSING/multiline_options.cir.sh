#!/usr/bin/env perl

use XyceRegression::Tools;
use XdmCommon;

#$Tools = XyceRegression::Tools->new();
#$Tools->setDebug(1);

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script  (This is NEVER used and can be ignored)
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

use Getopt::Long;
&GetOptions( "verbose!" => \$verbose );
$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$CIRFILE=$ARGV[3]; 
#$GOLDPRN=$ARGV[4];

# Comparison tolerances
$absTol = 1.0e-5;
$relTol = 1.0e-3;
$zeroTol = 1.0e-10;

# Note: must escape ( ) and * with \\ to get the CheckError() function to work.
# Must escape a double quote " with a single backslash \
# Do not have to escape { } or _.
# Strings to test for in the diagnostic output from xdm.
@xdmOutputSearchStrings = ("Removing ABSTOL.  PSPice ABSTOL is not the equivalent of Xyce ABSTOL",
  "Could not accept .OPTIONS \"ABSTOL\". Retained \\(as a comment\\). Continuing.",
  "Converting ITL4 into NONLIN-TRAN MAXSTEP",
  "Converting VNTOL into NONLIN/NONLIN-TRAN ABSTOL",
  "Total critical issues reported 			 = 0:", 
  "Total          errors reported 			 = 0:", 
  "Total          warnings reported 			 = 1:", 
  "Total          information messages reported 	 = 3:", 
  "SUCCESS: xdm completion status flag = 0:" 
);
$xdmOutputSearchStringsPtr=\@xdmOutputSearchStrings;

# these strings should be in the translated Xyce netlist
@translatedXyceNetlistSearchStrings= (".OPTIONS DEVICE GMIN=1.0E-9",
  ".OPTIONS NONLIN-TRAN MAXSTEP=20 RELTOL=0.01 ABSTOL=100u",
  ".OPTIONS NONLIN RELTOL=0.01 ABSTOL=100u",
  ".OPTIONS TIMEINT RELTOL=0.01 METHOD=TRAP",
);
$translatedXyceNetlistSearchStringsPtr=\@translatedXyceNetlistSearchStrings;

XdmCommon::verifyXDMtranslation($XYCE,$XYCE_VERIFY,$CIRFILE,"pspice",$absTol,$relTol,$zeroTol,
                                $verbose,$xdmOutputSearchStringsPtr,
                                $translatedXyceNetlistSearchStringsPtr);
