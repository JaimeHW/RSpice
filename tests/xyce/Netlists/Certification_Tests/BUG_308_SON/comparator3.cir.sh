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

# Since the shell script runs Xyce and the comparison program, it is
# responsible for capturing any error output from Xyce and the STDOUT & STDERR
# from the comparison program.  The outside script, run_xyce_regression,
# expects the STDERR output from Xyce to go into $CIRFILE.err, the STDOUT
# output from comparison to go into $CIRFILE.prn.out and the STDERR output from
# comparison to go into $CIRFILE.prn.err.  

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

$retval=$Tools->wrapXyce($XYCE,$CIRFILE);
if ($retval != 0) { print "Exit code = 10\n"; exit 10; }
$GOLDRES = $GOLDPRN ;
$GOLDRES =~ s/prn/res/;

@CIRLIST = ("comparator0.cir", "comparator.cir");

$GOLDPRN = "$CIRFILE.prn.gs";

# remove $CIRFILE.prn.gs if it exists from a previous test run 
`rm -f $GOLDPRN`;

foreach $CIR (@CIRLIST)
{
  $CMD="$XYCE $CIR > $CIR.out 2>$CIR.err";
  $retval = system($CMD);

  if ($retval != 0) { print "Exit code = 10\n"; exit 10; }
  `cat $CIR.prn >> $GOLDPRN`;
}
$CMD="$XYCE_VERIFY --goodres=$GOLDRES --testres=$CIRFILE.res $CIRFILE $GOLDPRN $CIRFILE.prn";
$retval = system("$CMD");
if ($retval != 0) 
{
  # failed compare 
  print "failed comparison \n";
  print "Exit code = 2\n"; 
  exit 2;
}

open(CIRPRN,"$CIRFILE.prn");
open(GOLDPRN, "$GOLDPRN");  
# count the output lines to ensure that output interpolation was followed 
# and produced exactly the number expected.
$N1 = 0;

$failed=0;

while ($line = <CIRPRN>)
{

  $line1 = <GOLDPRN>;

  if ($line =~ m/^index/i) { next; }
  if ($line =~ m/^end/i) { next; } 

  @linelist = split(" ",$line);
  $time = $linelist[1];
  
  @linelist1 = split(" ",$line1);
  $time1 = $linelist1[1];

  if ($N1 == 0) 
  { 
    $zerotol = 10e-15;
  }

  if ( ($time >=  $time1-$zerotol) and ($time <=$time1+$zerotol) )
  { 
  #print "N1 = $N1, time = $time\n";
    $N1++; 
  }
  else
  {
    $failed=1;   

    break;
  }
}

close(CIRPRN);
close(GOLDPRN);

print "N1 = $N1 \n";

if ($failed != 1)
{
  #print "Test Passed!\n";
  print "Exit code = 0\n"; exit 0;
}
else
{
  print "Exit code = 2\n"; exit 2;
}


