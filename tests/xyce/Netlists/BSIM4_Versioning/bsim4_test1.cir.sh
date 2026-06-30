#!/usr/bin/env perl

use XyceRegression::Tools;

$Tools = XyceRegression::Tools->new();

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script  (This is NEVER used and can be ignored)
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

# This test simply checks that Xyce issues appropriate warnings

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$CIRFILE=$ARGV[3]; 

# check various error cases
# this string should be in the warning messages of this Xyce run  
@searchstrings = ( ["Netlist warning in file bsim4_test1.cir at or near line 10",
                    "Device model PCH.2: Model card specifies BSIM4 version 4.9 which is newer"],
                   ["Netlist warning in file bsim4_test1.cir at or near line 8",
                    " Device model NCH.2: Model card specifies BSIM4 version 4.65 not supported by"],
                   ["Netlist warning in file bsim4_test1.cir at or near line 8",
                    "Device model NCH.2: Parameter TEMPEOT not valid for device of version 4.61."],
                   ["Netlist warning in file bsim4_test1.cir at or near line 7",
                    "Device model NCH.1: Model card specifies BSIM4 version 4.5 which is older"],
                   ["Netlist warning in file bsim4_test1.cir at or near line 7",
                    "Device model NCH.1: Parameter TEMPEOT not valid for device of version 4.61."]
                 );

print "Checking warning messages for DC measure line\n";
$retval = $Tools->runAndCheckGroupedWarning($CIRFILE,$XYCE,@searchstrings);

if ($retval != 0)
{
  print "Exit code = $retval\n"; 
  exit $retval;
}

print "Exit code = 0\n";
exit 0;



