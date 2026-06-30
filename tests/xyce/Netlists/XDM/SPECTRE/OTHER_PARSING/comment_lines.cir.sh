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

# Note: must escape ( ) and * and . with \\ to get the CheckError() function to work.
# Must escape a double quote " with a single backslash \
# Do not have to escape { } or _.

# Strings to test for in the diagnostic output from xdm.
# Need to test for something in the xdm output, so I can test for the
# commented-out lines in the translated Xyce netlist.  So, just test for
# the "success string".
@xdmOutputSearchStrings = (
  "Total critical issues reported 			 = 0:", 
  "Total          errors reported 			 = 0:", 
  "Total          warnings reported 			 = 14:", 
  "Total          information messages reported 	 = 1:", 
  "SUCCESS: xdm completion status flag = 0"
);
$xdmOutputSearchStringsPtr=\@xdmOutputSearchStrings;

# Strings to test for in the translated Xyce netlist
# Make sure that the Spectre comment lines (that start with *) are correctly 
# commented out in the translated Xyce netlist.  The third line had an in-line
# Spectre comment that is omitted from the translated Xyce netlist.
# The last line had an in-line Spectre comment.
# We also test that the first line of the Spectre netlist appears in the
# translated Xyce netlist.  See SRN Bug 2065 for more details.
@translatedXyceNetlistSearchStrings = (
  "\\*// Generated for: spectre",
  "\\*\\*Comment lines start with \\* and have a device name resistor in it",
  "\\*\\*Simple model for metal5 resistor",
  "RR1 net7 V2 R=1K"
);
$translatedXyceNetlistSearchStringsPtr=\@translatedXyceNetlistSearchStrings;

XdmCommon::verifyXDMtranslation($XYCE,$XYCE_VERIFY,$CIRFILE,"spectre",$absTol,$relTol,$zeroTol,
                                $verbose,$xdmOutputSearchStringsPtr,
                                $translatedXyceNetlistSearchStringsPtr);
