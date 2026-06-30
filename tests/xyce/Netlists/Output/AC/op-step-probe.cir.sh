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
# output from comparison to go into $CIRFILE.csd.out and the STDERR output from
# comparison to go into $CIRFILE.csd.err.

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDCSD=$ARGV[4];

$GOLDCSD =~ s/\.prn$//; # remove the .prn at the end.

# Comparison tolerances
$absTol=1e-5;
$relTol=1e-3;
$zeroTol=1e-10;
$fc = $XYCE_VERIFY;
$fc=~ s/xyce_verify/compare_csd_files/;

system("rm -f $CIRFILE.csd $CIRFILE.TD.csd $CIRFILE.out $CIRFILE.err");

$CMD="$XYCE $CIRFILE > $CIRFILE.out 2>$CIRFILE.err";
$retval = system($CMD);
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

if ( !(-f "$CIRFILE.TD.csd")) {
    print STDERR "Missing output file $CIRFILE.TD.csd\n";
    $xyceexit=14;
}

if ( !(-f "$CIRFILE.csd")) {
    print STDERR "Missing output file $CIRFILE.csd\n";
    $xyceexit=14;
}

if (defined ($xyceexit)) {print "Exit code = $xyceexit\n"; exit $xyceexit;}

$retcode = 0;

# compare the test and gold files
$CMD="$fc $CIRFILE.csd $GOLDCSD.csd $absTol $relTol $zeroTol";
$retval = system($CMD);
$retval = $retval >> 8;
if ($retval == 0) 
{ 
   $retcode = 0; 
}
else 
{ 
  print STDERR "Comparator exited with exit code $retval on file $CIRFILE.csd\n";
  $retcode = 2; 
}

$CMD="$fc $CIRFILE.TD.csd $GOLDCSD.TD.csd $absTol $relTol $zeroTol";
$retval = system($CMD);
$retval = $retval >> 8;
if ($retval == 0) 
{ 
   $retcode = 0; 
}
else 
{ 
  print STDERR "Comparator exited with exit code $retval on file $CIRFILE.TD.csd\n";
  $retcode = 2; 
}

print "Exit code = $retcode\n"; exit $retcode;


