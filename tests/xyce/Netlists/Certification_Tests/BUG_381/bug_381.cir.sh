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

$CIR1="comparator2.cir";
$CIR2="comparator3.cir";

print "Running $CIR1...\n";
$retval=$Tools->wrapXyce($XYCE,$CIR1);
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }
print "Running $CIR2...\n";
$retval=$Tools->wrapXyce($XYCE,$CIR2);
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }

# Now produce the gold standard for comparator2.cir (and also comparator3.cir):
@CIRLIST = ( "comparator0_7_-10.cir",
             "comparator0_6_-10.cir",
             "comparator0_5_-10.cir",
             "comparator0_7_0.cir",
             "comparator0_6_0.cir",
             "comparator0_5_0.cir",
             "comparator0_7_10.cir",
             "comparator0_6_10.cir",
             "comparator0_5_10.cir");

@STEPRES1 = (   7,  6,  5 ); # vdd
@STEPRES2 = ( -10,  0, 10 ); # temp

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

print "Generating comparator2.cir.res.gs\n";
$GOLDRES = "comparator2.cir.res.gs";
open(GRES,">$GOLDRES");
print GRES "STEP     vdd      temp\n";
for ($i=0 ; $i<= $#STEPRES2 ; $i++)
{
  for ($j=0 ; $j<= $#STEPRES1 ; $j++)
  {
    printf GRES ("%g        %g       %g\n",$j+$i*3,$STEPRES1[$j],$STEPRES2[$i]);
  }
}
print GRES "End of Xyce(TM) Parameter Sweep\n";
close(GRES);

print "Comparing comparator2.cir...\n";
$CMD="$XYCE_VERIFY --goodres=$GOLDRES --testres=comparator2.cir.res comparator2.cir comparator2.cir.prn.gs comparator2.cir.prn > comparator2.cir.prn.out 2> comparator2.cir.prn.err";
if (system("$CMD") != 0) { $failure = 1; }

print "Comparing comparator3.cir...\n";
$CMD="$XYCE_VERIFY --goodres=$GOLDRES --testres=comparator3.cir.res comparator3.cir comparator2.cir.prn.gs comparator3.cir.prn > comparator3.cir.prn.out 2> comparator3.cir.prn.err";
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

