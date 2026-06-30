#!/usr/bin/env perl

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script 
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

# If Xyce does not produce the appropriate output files, then we return exit code 14.
# If Xyce succeeds, but the test fails, then we return exit code 2.
# If the shell script fails for some reason, then we return exit code 1.

# Since the shell script runs Xyce and the comparison program, it is
# responsible for capturing any error output from Xyce and the STDOUT & STDERR
# from the comparison program.  The outside script, run_xyce_regression,
# expects the STDERR output from Xyce to go into $CIRFILE.err, the STDOUT
# output from comparison to go into $CIRFILE.raw.out and the STDERR output from
# comparison to go into $CIRFILE.raw.err.

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDBASE=$ARGV[4];

$GOLDBASE =~ s/.prn$//; # remove the .prn at the end.

# Remove the previous output files.
system("rm -f $CIRFILE.err $CIRFILE.out $CIRFILE.errmsg.* $CIRFILE.NOISE.prn"); 

# run Xyce and check for output files
$CMD="$XYCE $CIRFILE > $CIRFILE.out 2>$CIRFILE.err";
$retval=system($CMD);

if ($retval != 0)
{
  if ($retval & 127)
  {
    print "Exit code = 13\n"; 
    printf STDERR "Xyce crashed with signal %d on file %s\n",($retval&127),$CIRFILE; 
    exit 13;
  }
  else
  {
    print "Exit code = 10\n"; 
    printf STDERR "Xyce exited with exit code %d on %s\n",$retval>>8,$CIRFILE; 
    exit 10;
  }
}

if ( !(-f "$CIRFILE.NOISE.prn")) {
      print STDERR "Missing output file $CIRFILE.NOISE.prn\n";
      print "Exit code = 14\n"; exit 14;
}

# Use file_compare.pl to compare the .NOISE.prn files
$fc=$XYCE_VERIFY;
$fc =~ s/xyce_verify/file_compare/;
$abstol=1e-6;
$reltol=1e-3;
$zerotol=1e-8;

$retcode=0;
$CMD="$fc $CIRFILE.NOISE.prn $GOLDBASE.NOISE.prn $abstol $reltol $zerotol > $CIRFILE.errmsg.out 2> $CIRFILE.errmsg.err";
if (system($CMD) != 0){
  print STDERR "Verification failed on file $CIRFILE.NOISE.prn, see $CIRFILE.errmsg.err\n";
  $retcode = 2;
}

print "Exit code = $retcode\n"; exit $retcode;

