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
$GOLD=$ARGV[4];

$COMPARESCRIPT=$XYCE_VERIFY;
$COMPARESCRIPT =~ s/xyce_verify.pl/compareHomotopy.py/;
$COMPARE="python $COMPARESCRIPT ";

$TRUEGOLD = $GOLD;
$TRUEGOLD =~ s/.prn/.HOMOTOPY.prn/;

#$CIR1="arclengthSimple.cir";
$CIR1=$ARGV[3];

# create the output files
$result = system("$XYCE $CIR1");
if ( ( $result != 0 ) ) 
{
  if ($result & 127)
  {
    print "Exit code = 13\n"; 
    printf STDERR "Xyce crashed with signal %d on file %s\n",($result&127),$CIR1; 
    exit 13;
  }
  else
  {
    print "Exit code = 10\n"; 
    printf STDERR "Xyce exited with exit code %d on %s\n",$result>>8,$CIR1; 
    exit 10;
  }
}
if ( ! -e "$CIR1.HOMOTOPY.prn")
{
  print "Failed to produce tecplot format \n";
  print "Exit code = 14\n";
  exit 14;
}

#If this is a VALGRIND run, we don't do our normal verification, we
# merely run "valgrind_check.sh" as if it were xyce_verify.pl
if ($XYCE_VERIFY =~ m/valgrind_check/)
{
    print STDERR "DOING VALGRIND RUN INSTEAD OF REAL RUN!";
    if (system("$XYCE_VERIFY $CIR1 $GOLD $CIR1.prn > $CIR1.prn.out 2>&1 $CIR1.prn.err"))
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

$result = system( "$COMPARE $CIR1 $TRUEGOLD  $CIR1.HOMOTOPY.prn" );
if ( $result != 0 )
{
  print $result;
  print "\n";
  print "Failed compare\n";
  print "Exit code = 2\n";
  exit 2;
}

# output final result
print "Exit code = $result\n";
exit $result;

