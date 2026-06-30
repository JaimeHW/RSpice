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
# Otherwise we return the exit code of compare or xyce_verify.pl

# Since the shell script runs Xyce and the comparison program, it is
# responsible for capturing any error output from Xyce and the STDOUT & STDERR
# from the comparison program.  The outside script, run_xyce_regression,
# expects the STDERR output from Xyce to go into $CIRFILE.err, the STDOUT
# output from comparison to go into $CIRFILE.prn.out and the STDERR output from
# comparison to go into $CIRFILE.prn.err.  

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
#$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

$CIR1="ic_cap_step.cir";

print "Running $CIR1...\n";
$retval=$Tools->wrapXyce($XYCE,$CIR1);
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }

# Now produce the gold standard for ic_cap2.cir (and also ic_cap3.cir):
@CIRLIST = ( "ic_cap0.cir",
             "ic_cap1.cir",
             "ic_cap2.cir");

@STEPRES1 = (1.00e-06, 1.00e-05,  1.00e-04);# C value

print "Generating ic_cap_step.cir.prn.gs\n";
open(GS,">$CIR1.prn.gs");
$first_run=1;
foreach $CIR (@CIRLIST)
{
  print "Running $CIR...\n";
  $retval=$Tools->wrapXyce($XYCE,$CIR);
  if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }
  if (not -s "$CIR.prn") { print "Exit code = 14\n"; exit 14; }
  open(CIR,"$CIR.prn") or die "ERROR:  Cannot open $CIR.prn\n";
  while ($line = <CIR>)
  {
    if ($line =~ "Index" && $first_run==0) { next; }
    if ($line =~ "Xyce") { next; }
    print GS "$line";
    $first_run=0;
  }
  close(CIR);
}
print GS   "End of Xyce(TM) Parameter Sweep\n";
close(GS);

print "Generating ic_cap_step.cir.res.gs\n";
$GOLDRES = "ic_cap_step.cir.res.gs";
open(GRES,">$GOLDRES");
print GRES "STEP                 C1:C\n";
for ($i=0 ; $i<= $#STEPRES1 ; $i++)
{
  printf GRES ("%g        %g\n",$i,$STEPRES1[$i]);
}
print GRES "End of Xyce(TM) Parameter Sweep\n";
close(GRES);

print "Comparing ic_cap_step.cir...\n";
$CMD="$XYCE_VERIFY --goodres=$GOLDRES --testres=ic_cap_step.cir.res ic_cap_step.cir ic_cap_step.cir.prn.gs ic_cap_step.cir.prn > ic_cap_step.cir.prn.out 2> ic_cap_step.cir.prn.err";
if (system("$CMD") != 0) { $failure = 1; }


if ($failure)
{
  print "Exit code = 2\n"; exit 2;
}
else
{
  print "Exit code = 0\n"; exit 0;
}

print "Exit code = 1\n"; exit 1;

