#!/usr/bin/env perl

use XyceRegression::Tools;

$Tools = XyceRegression::Tools->new();
#$Tools->setDebug(1);
#$Tools->setVerbose(1);

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script 
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

# If Xyce does not produce a prn file, then we return exit code 10.
# If Xyce succeeds, but the test fails, then we return exit code 2.
# If the shell script fails for some reason, then we return exit code 1.
# Otherwise we return the exit code of compare or xyce_verify.pl

# Since the shell script runs Xyce also, it is responsible for capturing any
# error output from Xyce.  The script run_xyce_regression captures the test
# output and handles the resulting files.  

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
#$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

@CIRFILES = ("bug_307_d.cir",
             "bug_307_e.cir",
             "bug_307_f.cir",
             "bug_307_g.cir",
             "bug_307_h.cir",
             "bug_307_i.cir");

foreach $CIR (@CIRFILES)
{
  $retval=$Tools->wrapXyce($XYCE,$CIR);
  if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }
  if ($CIR =~ "bug_307_d")
  {
    $CIRDEFAULT=$CIR;
    next; 
  }
  $CMD="$XYCE_VERIFY $CIR $CIRDEFAULT.prn $CIR.prn > $CIR.prn.out 2> $CIR.prn.err"; 
  if (system("$CMD") != 0) { $failure=1; }
}
if ($failure == 1) { print "Exit code = 2\n"; exit 2; }

# Now we need to check for some warnings in the g and h tests:
@searchstrings = (  "Reading model named SW in the subcircuit OR and found one or",
		    "more models previously defined in this scope");
$found_OR = $Tools->checkError("$CIRFILES[3].out",@searchstrings);

@searchstrings = (  "Reading model named SW in the subcircuit AND and found one or",
		    "more models previously defined in this scope");
$found_AND = $Tools->checkError("$CIRFILES[3].out",@searchstrings);

@searchstrings = (  "Reading model named SW in the subcircuit XOR and found one or",
		    "more models previously defined in this scope");
$found_XOR = $Tools->checkError("$CIRFILES[3].out",@searchstrings);

if (not defined($found_OR) or not defined($found_AND) or not defined($found_XOR))
{
  print "Exit code = 2\n"; exit 2;
}

@searchstrings = (  "Reading model named SW in the subcircuit ONEBIT and found one or", 
		    "more models previously defined in this scope");
$found_ONEBIT = $Tools->checkError("$CIRFILES[4].out",@searchstrings);

@searchstrings = (  "Reading model named SW in the subcircuit OR and found one or",
		    "more models previously defined in this scope");
$found_OR = $Tools->checkError("$CIRFILES[4].out",@searchstrings);

@searchstrings = (  "Reading model named SW in the subcircuit AND and found one or",
		    "more models previously defined in this scope");
$found_AND = $Tools->checkError("$CIRFILES[4].out",@searchstrings);

@searchstrings = (  "Reading model named SW in the subcircuit XOR and found one or",
		    "more models previously defined in this scope");
$found_XOR = $Tools->checkError("$CIRFILES[4].out",@searchstrings);

if (not defined($found_ONEBIT) or not defined($found_OR) or not defined($found_AND) or not defined($found_XOR))
{
  print "Exit code = 2\n"; exit 2;
}

# We haven't received an error, so we exit with 0.
print "Exit code = 0\n"; exit 0;

