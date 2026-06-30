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

$TMPCIRFILE="printLine_for_embedded_cmosInverter.cir";
$TECPLOTFILE="embedded_cmosInverter.cir_ensemble.dat";
$GOLDDAT="goldprn_embedded_cmosInverter.cir_ensemble.dat";

# Some platforms we test on (esp Windows) use a really bad random number generator.
# To accomodate that I've made the comparison tolerances loose.  Also, if the test
# initially fails the comparison, I run it one more time.  Usually, even if it 
# fails the first time, it will pass the second time.

$num_tries=0;
$finished=0;

while ($num_tries < 2 && $finished == 0)
{
  $passed=1;

  # Run netlist:
  $CMD="$XYCE $CIRFILE > $CIRFILE.out 2>$CIRFILE.err";
  if (system($CMD) != 0)
  {
    `echo "Xyce EXITED WITH ERROR! on $CIRFILE" >> $CIRFILE.err`;
    $passed=0;
  }
  else
  {
    if (-z "$CIRFILE.err" ) {`rm -f $CIRFILE.err`;}
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
    $passed=0;
  }

  # convert and compare the *dat file.
  $result = system( "$TRANSLATE $TECPLOTFILE");
  #$result = system( "$TRANSLATE capacitor.cir.dat");
  if ( $result != 0 )
  {
    print "Failed to translate TECPLOT to STD\n";
    $passed=0;
  }
  else
  {
    $CMD="$XYCE_VERIFY $TMPCIRFILE test_embedded_cmosInverter.cir_ensemble.dat $GOLDDAT > $CIRFILE.prn.out 2> $CIRFILE.prn.err";
    if (system("$CMD") != 0) 
    {
      print STDERR "Verification failed on file test_$CIRFILE.dat with $GOLDDAT, see $CIRFILE.prn.err\n";
      #print STDERR "Command $CMD\n";
      $passed=0;
    }
    else
    {
      #printf "Success! The test passed.\n";
    }
  }

  if($passed==1)
  {
    $finished=1;
  }
  $num_tries++;
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

if ($passed==1)
{
#    print STDERR "Ran $num_tries, passed the last one.\n";
    print "Exit code = 0\n";
    exit 0;
}
else
{
#    print STDERR "Ran $num_tries times, failed all of them.\n";
    print "Exit code = 2\n";
    exit 2;
}

