#!/usr/bin/env perl

use XyceRegression::Tools;

$Tools = XyceRegression::Tools->new();

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

# make Xyce output an ASCI raw file so we can check it as part of this test.
$XYCE="$ARGV[0] -a ";
$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

$retval = -1;
$failure = 0;

$cmd="$XYCE $CIRFILE > $CIRFILE.out 2> $CIRFILE.err";
system($cmd);
$retval=$?;

if ($retval != 0) { print "Exit code = $retval\n"; exit $retval>>8; }

if (not -s "$CIRFILE.raw" ) { print "Exit code = 14\n"; exit 14; }

# we're not comparing this to a gold standard. 
#$CMD="$XYCE_VERIFY --verbose $CIRFILE $GOLDPRN.gs $CIRFILE.prn > $CIRFILE.prn.out 2> $CIRFILE.prn.err";
#if (system("$CMD") == 0)  # in the unlikely event that this passes, exit now.
#{ 
#  verbosePrint "Wow, this test passed xyce_verify.pl!!!\n";
#  exit 0 
#}

# To pass this circuit we need to verify that there are 3 "Plotname" section markers

# Now we still want to pass this circuit, so we just need to check that
# non-trivial output was created.
open(ERR,">>$CIRFILE.prn.err");

if (not -f "$CIRFILE.raw") # was the raw file created?
{ 
  print ERR "The file >>$CIRFILE.raw<< was not created!\n";
  $failure = 1; 
} 

# the following should work if the system on which this test is running has grep and wc.
# I'm not sure if our windows test machines have this.
#
#$numPlotnames = `grep "Plotname" $CIRFILE.raw | wc -l`;
#if ( $numPlotnames ne "3" )  # Did Xyce finish integrating?
#{ 
#  print ERR "Xyce never printed \"End of Xyce(TM) Simulation\" at the end of $CIRFILE.raw\n";
#  $failure = 1; 
#}

# thus we'll manually open the output raw file and check that.

# Check that the the output is not trivially zero:
$numPlotnames=0;
open(OUT,"$CIRFILE.raw");
while ($line = <OUT>)

{
  @contents = split( " ", $line );
  if( $contents[0] eq "Plotname:" )
  {
    $numPlotnames++;
  }
}

if( $numPlotnames != 3 )
{
  print ERR "Incorrect number of Plotname sections in raw file $CIRFILE.raw\n";
  $failure = 1;
}

close(OUT);
close(ERR);

if ($failure)
{
  print "Exit code = 2\n"; exit 2;
}
else
{
  print "Exit code = 0\n"; exit 0;
}

