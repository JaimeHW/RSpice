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
# error output.  Note that parens are escaped with \\
@searchstrings = ("Netlist error: Function or variable W\\(YACC!ACC1\\) is not defined",
  "Netlist error: Function or variable P\\(K1\\) is not defined",
  "Netlist error: Function or variable W\\(K2\\) is not defined",
  "Netlist error: Function or variable P\\(OLINE1\\) is not defined",
  "Netlist error: Function or variable P\\(UAND1\\) is not defined",
  "Netlist error: Function or variable P\\(YOR!OR1\\) is not defined");

$XYCE=$ARGV[0];
#$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

# check for the Xyce error messages in the .out file
$retval = $Tools->runAndCheckError($CIRFILE,$XYCE,@searchstrings);

# Verify that only two copies of the P(K1) error message are produced.
# See SON Bug 822 for more details.
if ($retval == 0)
{
  $k1count = `grep -c "Netlist error: Function or variable P\(K1\)" $CIRFILE.out 2>/dev/null`;
  if ($k1count =~ 2)
  {
    printf "$CIRFILE.out contained correct number (2) of P(K1) warnings\n", $k1count;
  }
  else
  {
    printf "$CIRFILE.out contained incorrect number (%d) of P(K1) warnings\n", $k1count;
    $retval = 2;
  }
}

print "Exit code = $retval\n";
exit $retval


