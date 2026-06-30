#!/usr/bin/env perl

use XyceRegression::Tools;
$Tools = XyceRegression::Tools->new();

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = unused
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

# If Xyce does not produce a prn file, then we return exit code 10.
# If Xyce succeeds, but the test fails, then we return exit code 2.
# If the shell script fails for some reason, then we return exit code 1.
# If xyce_verify fails we return exit code 2.  If all passes we return exit
# code 0.

# This test checks that the two variants of multiply-connected mutual inductor
# syntax produce the same results, and that those results are correct.
#
# Ideally, we would generate an analytic solution, but instead we'll use
# a gold standard generated from the SPICE3F5-compatible syntax.  As long
# as both of our netlists match this gold standard, we'll call them acceptable.

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$CIRFILE=$ARGV[3];
# We don't use the goldprn name passed in, we have our own gold standard
# right here

# The CIRFILE is the dummy circuit name.  Construct the real names
$S3CIR=$CIRFILE;
$PSCIR=$CIRFILE;
$S3CIR =~ s/odd/odd_s3f5/;
$PSCIR =~ s/odd/odd_ps/;

$GOLDPRN = $S3CIR . ".prn.gs";

# If this is a VALGRIND run, we don't do our normal verification, we
# merely run "valgrind_check.sh" as if it were xyce_verify.pl
# we also only care about running $PSCIR
if ($XYCE_VERIFY =~ m/valgrind_check/)
{
  print STDERR "DOING VALGRIND RUN INSTEAD OF REAL RUN!";
  $retval = -1;
  $retval=$Tools->wrapXyce($XYCE,$PSCIR);
  if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }
  if (not -s "$PSCIR.prn" ) { print "Exit code = 14\n"; exit 14; }

  if (system("$XYCE_VERIFY $PSCIR $GOLDPRN $PSCIR.prn > $PSCIR.prn.out 2>&1 $PSCIR.prn.err"))
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

# Otherwise doing a normal run
# Run the netlists
$CMD="$XYCE $S3CIR > $CIRFILE.out 2>$CIRFILE.err";

if (system($CMD) != 0)
{
    `echo "Xyce EXITED WITH ERROR! on $S3CIR" >> $CIRFILE.err`;
    $xyceexit=1;
}
else
{
    if (-z "$CIRFILE.err" ) {`rm -f $CIRFILE.err`;}
}

if ($xyceexit==1)
{
    print "Exit code = 10\n";
    print STDERR "Xyce exited with error on $S3CIR\n";
    exit 10;
}

`echo "\n\n\n=====PS version=====" >> $CIRFILE.out`;

$CMD="$XYCE $PSCIR >> $CIRFILE.out 2>$CIRFILE.err";

if (system($CMD) != 0)
{
    `echo "Xyce EXITED WITH ERROR! on $PSCIR" >> $CIRFILE.err`;
    $xyceexit=1;
}
else
{
    if (-z "$CIRFILE.err" ) {`rm -f $CIRFILE.err`;}
}

if ($xyceexit==1)
{
    print "Exit code = 10\n";
    print STDERR "Xyce exited with error on $PSCIR\n";
    exit 10;
}

#If this is a VALGRIND run, we don't do our normal verification, we
# merely run "valgrind_check.sh" as if it were xyce_verify.pl
if ($XYCE_VERIFY =~ m/valgrind_check/)
{
  print STDERR "DOING VALGRIND RUN INSTEAD OF REAL RUN!";
  if (system("$XYCE_VERIFY $PSCIR $GOLDPRN $PSCIR.prn > $CIRFILE.prn.out 2>&1 $CIRFILE.prn.err"))
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

# Now check against gold standards
$CMD="$XYCE_VERIFY $S3CIR $GOLDPRN $S3CIR.prn > $CIRFILE.prn.out 2>$CIRFILE.prn.err";
$retval=system("$CMD");

if ($retval!=0)
{
    print STDERR "verification of $S3CIR.prn failed.\n";
    print "Exit code = 2\n";
    exit 2;
}
 
$CMD="$XYCE_VERIFY $PSCIR $GOLDPRN $PSCIR.prn > $CIRFILE.prn.out 2>$CIRFILE.prn.err";
$retval=system("$CMD");

if ($retval!=0)
{
    print STDERR "verification of $PSCIR.prn failed.\n";
    print "Exit code = 2\n";
    exit 2;
}


print "Exit code = 0\n";
exit 0;
