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
# output from comparison to go into $CIRFILE.dat.out and the STDERR output from
# comparison to go into $CIRFILE.dat.err.

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];

$TRANSLATESCRIPT=$XYCE_VERIFY;
$TRANSLATESCRIPT =~ s/xyce_verify.pl/convertToPrn.py/;
$TRANSLATE="python $TRANSLATESCRIPT   ";

$TMPCIRFILE="printLine_for_cmosInverter_pce.cir";
$TECPLOTFILE="cmosInverter_pce.cir.PCE.dat";
$GOLDDAT="goldprn_cmosInverter_pce.cir.PCE.dat";

system("rm -f test_$CIRFILE* $CIRFILE.out $CIRFILE.err $CIRFILE.PCE.dat");

# Run netlist:
$CMD="$XYCE $CIRFILE > $CIRFILE.out 2>$CIRFILE.err";
if (system($CMD) != 0)
{
  `echo "Xyce EXITED WITH ERROR! on $CIRFILE" >> $CIRFILE.err`;
  $xyceexit=1;
}
else
{
  if (-z "$CIRFILE.err" ) {`rm -f $CIRFILE.err`;}
}


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
#
# check the number of TITLE outputs
$titlecount = `grep -ic title $TECPLOTFILE 2>/dev/null`;
if ($titlecount =~ 1)
{
  printf "Tecplot file contained correct number(%d) of titles.\n", $titlecount;
}
else
{
  printf "Tecplot file contained wrong number(%d) of titles.\n", $titlecount;
  $retcode = 2;
}

# convert and compare the *dat file.
$result = system( "$TRANSLATE $TECPLOTFILE");
if ( $result != 0 )
{
  print "Failed to translate TECPLOT to STD\n";
  $retcode = 2;
}
else
{
    $CMD="$XYCE_VERIFY $TMPCIRFILE test_cmosInverter_pce.cir.PCE.dat $GOLDDAT > $CIRFILE.prn.out 2> $CIRFILE.prn.err";
    if (system("$CMD") != 0) 
    {
      print STDERR "Verification failed on file test_$CIRFILE.PCE.dat with $GOLDDAT, see $CIRFILE.prn.err\n";
      #print STDERR "Command $CMD\n";
      $retcode = 2;
    }
    else
    {
      #printf "Success! The test passed.\n";
      $retcode = 0;
    }

}

print "Exit code = $retcode\n"; exit $retcode;


