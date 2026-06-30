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
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

# remove old files if they exist
`rm -f $CIRFILE.HOMOTOPY.dat $CIRFILE.err $CIRFILE.res`;
`rm -f $CIRFILE.homotopy.out $CIRFILE.homotopy.err`;
`rm -f $CIRFILE.trim $CIRFILE.goldtrim`;

# run Xyce
$CMD="$XYCE $CIRFILE > $CIRFILE.out 2> $CIRFILE.err";
$retval = system("$CMD");
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

# Exit if the HOMOTOPY.dat file was not made
if (not -s "$CIRFILE.HOMOTOPY.dat" )
{
  print "$CIRFILE.HOMOTOPY.dat file is missing\n"; 
  print "Exit code = 14\n"; 
  exit 14;
}

#If this is a VALGRIND run, we don't do our normal verification, we
# merely run "valgrind_check.sh" as if it were xyce_verify.pl
if ($XYCE_VERIFY =~ m/valgrind_check/)
{
    print STDERR "DOING VALGRIND RUN INSTEAD OF REAL RUN!";
    if (system("$XYCE_VERIFY $CIRFILE $GOLDPRN $CIRFILE.prn > $CIRFILE.prn.out 2>&1 $CIRFILE.prn.err"))
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

# Now check the .HOMOTOPY.dat file
$absTol=1e-5;
$relTol=1e-3;
$zeroTol=1e-16;

# must remove the data line from the gold file and test file
$fc = $XYCE_VERIFY;
$fc=~ s/xyce_verify/file_compare/;
$GOLDTOPY = substr($GOLDPRN,0,-3)."HOMOTOPY.dat";

`grep -v "TIME" $CIRFILE.HOMOTOPY.dat > $CIRFILE.trim`;
`grep -v "TIME" $GOLDTOPY > $CIRFILE.goldtrim`;

$CMD="$fc $CIRFILE.trim $CIRFILE.goldtrim $absTol $relTol $zeroTol > $CIRFILE.homotopy.out 2> $CIRFILE.homotopy.err";
$retval = system("$CMD");
if ( $retval != 0 )
{
  print "test Failed comparison of HOMOTOPY.dat file vs. gold HOMOTOPY.dat file!\n";
  print "Exit code = 2\n";
  exit 2;
}
else
{
  print "Passed comparison of HOMOTOPY.dat files\n";
  print "Exit code = 0\n";
  exit 0;
}




