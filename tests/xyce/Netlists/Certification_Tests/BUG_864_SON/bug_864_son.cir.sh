#!/usr/bin/env perl

use XyceRegression::Tools;

$Tools = XyceRegression::Tools->new();
#$Tools->setDebug(1);

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script 
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file 

# Here we are testing only that Xyce fails with the appropriate error message.

# these search strings are supposed to occur one right after the other in the
# error output.
@searchstrings = ( "Unable to resolve parameter FOO found in .PARAM statement"
);

$XYCE=$ARGV[0];
$CIRFILE=$ARGV[3];

$retval = $Tools->runAndCheckError($CIRFILE,$XYCE,@searchstrings);
print "Exit code = $retval\n";
exit $retval


