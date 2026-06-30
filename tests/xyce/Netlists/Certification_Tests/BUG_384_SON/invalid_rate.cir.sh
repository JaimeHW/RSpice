#!/usr/bin/env perl

use XyceRegression::Tools;

$Tools = XyceRegression::Tools->new();

# The input arguments to this script are: 
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script 
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

# If Xyce does not produce a prn file, then we return exit code 10.
# If Xyce succeeds, but the test fails, then we return exit code 2.
# If the shell script fails for some reason, then we return exit code 1.

# Since the shell script runs Xyce and the comparison program, it is
# responsible for capturing any error output from Xyce and the STDOUT & STDERR
# from the comparison program.  The outside script, run_xyce_regression,
# expects the STDERR output from Xyce to go into $CIRFILE.err, the STDOUT
# output from comparison to go into $CIRFILE.prn.out and the STDERR output from
# comparison to go into $CIRFILE.prn.err.  

# these search strings are supposed to occur one right after the other in the
# error output.
@searchstrings = (" in file invalid_rate_react at line 10.2.-2.",
                  " Invalid rate type \"sample\"");

$XYCE=$ARGV[0];
#$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

$retval = system("$XYCE $CIRFILE > $CIRFILE.out 2> $CIRFILE.err");
if ($retval)
{
    # The Xyce run failed (as it should), now check for error messages
    $retval2=$Tools->checkError("$CIRFILE.out",@searchstrings);
    print "Exit code = $retval2\n"; exit $retval2;
}
else
{
    print STDERR "Xyce did not fail as expected!\n";
    print "Exit code = $2\n"; exit $2;
}
    
