#!/usr/bin/env perl

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
#$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
#$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

@CIRFILES = ("bug_307_d.cir",
             "bug_307_e.cir",
             "bug_307_f.cir",
             "bug_307_g.cir",
             "bug_307_h.cir",
             "bug_307_i.cir");
$CIRDEFAULT=$CIRFILES[0];

foreach $CIR (@CIRFILES)
{
  $CMD="$XYCE $CIR > $CIR.out 2> $CIR.err";
  if (system("$CMD") != 0) { 
    print "Exit code = 10\n";
    exit 10;
  }
  if ("$CIR" eq "$CIRDEFAULT") { next; }
  $CMD="diff $CIR.prn $CIRDEFAULT.prn > $CIR.prn.out 2> $CIR.prn.err";
  if (system("$CMD") != 0) { $failure=1; }
}

# Now we need to check for some warnings in the g and h tests:
$CIR=$CIRFILES[3];
$searchstring2 = "more models previously defined in this scope.";
open(OUT,"$CIR.out") or die "Error:  Cannot open $CIR.out\n";
while ($line = <OUT>)
{
  if ($line =~ "Reading model named SW in the subcircuit OR and found one or")
  {
    $line = <OUT>;
    if ($line =~ "$searchstring2")
    { $found_OR = 1; }
  }
  if ($line =~ "Reading model named SW in the subcircuit AND and found one or")
  {
    $line = <OUT>;
    if ($line =~ "$searchstring2")
    { $found_AND = 1; }
  }
  if ($line =~ "Reading model named SW in the subcircuit XOR and found one or")
  {
    $line = <OUT>;
    if ($line =~ "$searchstring2")
    { $found_XOR = 1; }
  }
  if (defined($found_OR) and defined($found_AND) and defined($found_XOR))
  { break; }
}
close(OUT);

if (not defined($found_OR) or not defined($found_AND) or not defined($found_XOR))
{
  $failure = 1;
}

$CIR=$CIRFILES[4];
$searchstring2 = "more models previously defined in this scope.";
open(OUT,"$CIR.out") or die "Error:  Cannot open $CIR.out\n";
while ($line = <OUT>)
{
  if ($line =~ "Reading model named SW in the subcircuit ONEBIT and found one or")
  {
    $line = <OUT>;
    if ($line =~ "$searchstring2")
    { $found_ONEBIT = 1; }
  }
  if ($line =~ "Reading model named SW in the subcircuit OR and found one or")
  {
    $line = <OUT>;
    if ($line =~ "$searchstring2")
    { $found_OR = 1; }
  }
  if ($line =~ "Reading model named SW in the subcircuit AND and found one or")
  {
    $line = <OUT>;
    if ($line =~ "$searchstring2")
    { $found_AND = 1; }
  }
  if ($line =~ "Reading model named SW in the subcircuit XOR and found one or")
  {
    $line = <OUT>;
    if ($line =~ "$searchstring2")
    { $found_XOR = 1; }
  }
  if (defined($found_ONEBIT) and defined($found_OR) and defined($found_AND) and defined($found_XOR))
  { break; }
}
close(OUT);

if (not defined($found_ONEBIT) or not defined($found_OR) or not defined($found_AND) or not defined($found_XOR))
{
  $failure = 1;
}

if ($failure)
{
  print "Exit code = 2\n";
  exit 2
}
else
{
  print "Exit code = 0\n";
  exit 0
}

print "Exit code = 1\n";
exit 1
