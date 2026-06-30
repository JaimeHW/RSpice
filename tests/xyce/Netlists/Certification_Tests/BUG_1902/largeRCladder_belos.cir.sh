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

# Error for small memory on Linux.
my $err2;
# Error for small memory on Windows.
my $err3;

$CMD="$XYCE $CIRFILE > $CIRFILE.out 2> $CIRFILE.err";
if (system("$CMD") != 0) 
{ 
  $err2 = `grep "OUT OF MEMORY" $CIRFILE.err`;
  if ( $err2 ) { $retcode = 0; }
  else { print "Exit code = 10\n"; exit 10; }
}

$GOLDPRN =~ s/\.prn$//; # remove the .prn at the end.

#if (( !$err2 && (-f "$CIRFILE.HB.TD.prn") && (-f "$CIRFILE.HB.FD.prn")) && (-f "$CIRFILE.hb_ic.prn"))
if ( -f "$CIRFILE.hb_ic.prn" )
{
  $CMD="$XYCE_VERIFY $CIRFILE $CIRFILE.hb_ic.prn $GOLDPRN.hb_ic.prn > $CIRFILE.hb_ic.prn.out 2> $CIRFILE.hb_ic.prn.err";
  $retval = system("$CMD");

  # Check to see that the code is NOT resizing the Krylov subspace, this is not a problem for Belos.
  # Some platforms do not have enough memory to allocate for this problem, so see if they run out of memory.
  $err1 = `grep "resizing Krylov subspace" $CIRFILE.out`;
  if ( !$err1 && $err2 && $retval==0 )
  {
    # The code correctly resized the Krylov subspace, but there is not enough memory to solve this problem.
    # This receives a pass because it cannot be helped by Xyce and a reasonable error is output by AztecOO.
    $retcode = 0;
  }
  elsif ( !$err1 && $retval==0 )
  { 
    $CMD="$XYCE_VERIFY $CIRFILE $CIRFILE.HB.TD.prn $GOLDPRN.HB.TD.prn > $CIRFILE.TD.prn.out 2> $CIRFILE.TD.prn.err";
    $retval = system("$CMD");
    if ($retval == 0) { $retcode = 0; }
    else { $retcode = 2; }
  }
  else { $retcode = 2; }
}
else { $retcode = 14; }

print "Exit code = $retcode\n"; exit $retcode;
  
