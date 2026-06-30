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
# output from comparison to go into $CIRFILE.prn.out and the STDERR output from
# comparison to go into $CIRFILE.prn.err.

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

$XPLAT_DIFF = $ARGV[1];
$XPLAT_DIFF =~ s/xyce_verify/xplat_diff/;

$GOLDCIRFILE="tran-prn2.cir";

system("rm -f $CIRFILE.prn $CIRFILE.err");

# Run the main file:
$CMD="$XYCE $CIRFILE > $CIRFILE.out 2>$CIRFILE.err";
if (system($CMD) != 0) 
{
  `echo "Xyce EXITED WITH ERROR! on $CIRFILE" >> $CIRFILE.err`;
  $xyceexit=1;
}
if (defined ($xyceexit)) {print "Exit code = 10\n"; exit 10;}

if ( !(-f "$CIRFILE.prn")) 
{
    print STDERR "Missing output file $CIRFILE.prn\n";
    $xyceexit=14;
}
if (defined ($xyceexit)) {print "Exit code = $xyceexit\n"; exit $xyceexit;}

#If this is a VALGRIND run, we don't do our normal verification, we
# merely run "valgrind_check.sh" as if it were xyce_verify.pl
if ($XYCE_VERIFY =~ m/valgrind_check/)
{
  print STDERR "DOING VALGRIND RUN INSTEAD OF REAL RUN!";
  if (system("$XYCE_VERIFY $CIRFILE junk $CIRFILE.prn > $CIRFILE.prn.out 2>&1 $CIRFILE.prn.err"))
  {
    print "Exit code = 2 \n";
    exit 2;
  }
  else
  {
    print "Exit code = 0 \n";
    exit 0;
  }
}

# Run the gold standard file:
$CMD="$XYCE $GOLDCIRFILE > $GOLDCIRFILE.extra.out 2>$GOLDCIRFILE.extra.err";
if (system($CMD) != 0) 
{
  `echo "Xyce EXITED WITH ERROR! on $GOLDCIRFILE" >> $GOLDCIRFILE.extra.err`;
  $xyceexit=1;
}

if (defined ($xyceexit)) {print "Exit code = 10\n"; exit 10;}

if ( !(-f "$GOLDCIRFILE.prn")) 
{
    print STDERR "Missing output file $GOLDCIRFILE.prn\n";
    $xyceexit=14;
}

if (defined ($xyceexit)) {print "Exit code = $xyceexit\n"; exit $xyceexit;}


# strip the header and footer from the gold standard file:
$CMD="grep -v Index $GOLDCIRFILE.prn | grep -v End > strippedGold.prn";
if (system($CMD) != 0) 
{
  `echo "Xyce EXITED WITH ERROR! on $GOLDCIRFILE" >> $GOLDCIRFILE.extra.prn.err`;
  $xyceexit=1;
}
if (defined ($xyceexit)) {print "Exit code = 10\n"; exit 10;}


$retcode = 0;

# do a comparison between the stripped gold standard and the noheader/nofooter output file.
#$CMD="diff strippedGold.prn $CIRFILE.prn";
$CMD="$XPLAT_DIFF $CIRFILE.prn strippedGold.prn > $CIRFILE.prn.out 2> $CIRFILE.prn.err";
if (system("$CMD") != 0) 
{
    print STDERR "Verification failed on file $CIRFILE.prn, see $CIRFILE.prn.err\n";
    $retcode = 2;
}

print "Exit code = $retcode\n"; exit $retcode;
