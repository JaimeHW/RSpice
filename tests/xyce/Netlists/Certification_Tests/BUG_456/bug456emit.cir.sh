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
$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
#$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

$baseline="emitter_baseline.cir";
$restarted="emitter_restart.cir";

`rm -f $baseline.prn  $restarted.prn `;
`rm -f $baseline.err  $restarted.err `;
`rm -f emitter0*`;
# --------------------------------
# Run Xyce on the circuits, baseline first, then restarted.

$CMD="$XYCE $baseline > $baseline.out 2> $baseline.err";
#
# lower byte is not relevent here.  xyce's return code is in
# the upper byte to shift it 8 bits.
$retval = system("$CMD");
$retval = $retval >> 8;
if ($retval != 0) { print "Xyce failed running $baseline.\nExit code = 10\n"; exit 10; }

# These next two conditionals are here because on Losedows with Intel compilers
# the exponent in the file name gets an extra zero.  For simplicity, just
# discard it so the netlists can find the restart file they're looking for.
if (-f "emitter2e-005")
{
    system("mv emitter2e-005 emitter2e-05");
}

$CMD="$XYCE $restarted > $restarted.out 2> $restarted.err";
#
# lower byte is not relevent here.  xyce's return code is in
# the upper byte to shift it 8 bits.
$retval = system("$CMD");
$retval = $retval >> 8;
if ($retval != 0) { print "Xyce failed running $restarted.\nExit code = 10\n"; exit 10; }


if ( not -s "$baseline.prn" ) { print "Exit code = 14\n"; exit 14; }
if ( not -s "$restarted.prn" ) { print "Exit code = 14\n"; exit 14; }
# --------------------------------
# Run xyce_verify on the output

$CMD="$XYCE_VERIFY $baseline $baseline.prn $restarted.prn > $restarted.prn.out 2> $restarted.prn.err";
#
# lower byte is not relevent here.  xyce_verify's return code is in
# the upper byte to shift it 8 bits.
$retval = system("$CMD");
$retval = $retval >> 8;
if ($retval != 0)
{
  # failed compare 
  print "failed comparison \n";
  print "Exit code = 2\n"; 
  exit 2;
}

open(CIRPRN, "$restarted.prn");    
open(GOLDPRN, "$baseline.prn");

$N1 = 0;
$zerotol = 10e-15;
$failed=0;

while ($line = <GOLDPRN> )
{

#  $line1 = <CIRPRN>;

  if ($line =~ m/^index/i) {$line1 = <CIRPRN>; next; }
  if ($line =~ m/^end/i) { next; }

  @linelist = split(" ",$line);
  $time = $linelist[1];
  

#  if ($N1 == 0) 
#  { 
#    $zerotol = 10e-15;
#  }

  if ( ($time >= 2e-5-$zerotol) )
  {
    $line1 = <CIRPRN>;

    @linelist1 = split(" ",$line1);
    $time1 = $linelist1[1];

    if ( ($time >=  $time1-$zerotol) and ($time <$time1+$zerotol) )
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

