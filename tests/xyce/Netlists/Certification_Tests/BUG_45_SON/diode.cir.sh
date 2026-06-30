#!/usr/bin/env perl

use XyceRegression::Tools;
$Tools = XyceRegression::Tools->new();

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script  (This is NEVER used and can be ignored)
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$CIRFILE=$ARGV[3]; 
$GOLDPRN=$ARGV[4];

$retval = -1;
$retval=$Tools->wrapXyce($XYCE,$CIRFILE);
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }

# First test that can parse the W() or P() on the .print line
if (not -s "$CIRFILE.prn" ) { print "Exit code = 14\n"; exit 14; }

# do a valgrind run on the diode device.  This is done for each of the
# three time integrators, in three separate tests.
if ($XYCE_VERIFY =~ m/valgrind_check/)
{
  print STDERR "DOING VALGRIND RUN INSTEAD OF REAL RUN!";
  if (system("$XYCE_VERIFY $CIRFILE junk $CIRFILE.prn > $CIRFILE.prn.out 2>&1 $CIRFILE.prn.err"))
  {
    #valgrind_check.sh has reported a memory error, test is a failure
    print "Exit code = 2 \n";
    exit 2;
  }
  else
  {
    #valgrind_check.sh has reported no memory errors, pass
    print "Exit code = 0 \n";
    exit 0;
  }
}

# these strings should be in the output of this successful Xyce run
@searchstrings = ("Netlist warning in file diode.cir at or near line 29",
   "No model parameter BOGOPARAM found for model D1N3940 of type D, parameter"
);

$retval = $Tools->checkError("$CIRFILE.out",@searchstrings);
if ($retval != 0)
{
  print "Check on warning message failed\n";
  print "Exit code = $retval\n"; exit $retval; 
} 

# also make sure that the answer is correct
$CMD="$XYCE_VERIFY $CIRFILE $CIRFILE.prn $GOLDPRN 2> $CIRFILE.err";
$retval=system($CMD)>>8;
if ($retval != 0)
{
  print "Xyce Verify failed\n"; 
}
else
{
  print "Xyce Verify also passed\n";
}

print "Exit code = $retval\n"; exit $retval;  
  
