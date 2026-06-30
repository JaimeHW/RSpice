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

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

$retval = -1;
$retval=$Tools->wrapXyce($XYCE,$CIRFILE);
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }
if (not -s "$CIRFILE.prn" ) { print "Exit code = 14\n"; exit 14; }

#
# The simulation should have used a variable max time step
# as specified by the {schedule()} expression.  We also
# have this same expression on the .print line so that the
# output looks like:
#
# INDEX Time V(a) V(b) current_max_time_step
#
# so, we'll look through each line of output can check if:
#
#  t(i) - t(i-1) < current_max_time_step(i)
#
open(OUTPUTFILE, "$CIRFILE.prn");
$lineNumber = 0;
$lastTime = -1.0;
$lastSpecifiedMaxDeltaTime=0.0;
$numFailures=0;
while( $line=<OUTPUTFILE> )
{
  $lineNumber++;
  if( ($line =~ /Index/i) || ($line =~ /End/i) )
  {  
    # don't do anything for header or footer line of output
  }
  else
  {
    # chop up the line to get at individual numbers
    chomp $line;
    @vars = split " ", $line;
    if( $lastTime < 0.0 )
    {
      # we haven't parsed the first line of output yet, so save
      # this time as lastTime for calculating the t(i) - t(i-1)
      $lastTime = $vars[1];
      $lastMaxDeltaTime = $vars[4];
    }
    else
    {
      $thisTime = $vars[1];
      $currentMaxDeltaTime = $vars[4];
      # print "actual delta t: ", ($thisTime - $lastTime), " specified delta t: ", $currentMaxDeltaTime, "\n";
      # if currentMaxDeltaTime == 0.0 then none was specified for this interval. 
      # also if lastMaxDeltaTime == 0.0, then none was specified when we made that time step. 
      # in both cases, the rest of the test doesn't apply
      if( ($currentMaxDeltaTime > 0.0) && ($lastMaxDeltaTime > 0.0) &&
        ( ($thisTime - $lastTime) - $currentMaxDeltaTime  > 1e-16) )
      {
        # When the max time step changes due to a schedule() function, it can
        # take a few steps before the new max time is obeyed.  This ok as the
        # integrator is taking other factors into consideration before it
        # applies the new max time step.  Just count the errors here and
        # fail if it's over a max
        print "Max Delta Time surpassed: Delta Time Used ", ($thisTime - $lastTime), 
          " Specified Max: ", $currentMaxDeltaTime, " near line ", $lineNumber, "\n";
        $numFailures++;
      }
      $lastTime = $thisTime;
      $lastMaxDeltaTime = $currentMaxDeltaTime;
    }
  }
}
close(OUTPUTFILE);

if( $numFailures > 10 )
{
  print "Exit code = 1\n";
  exit 17;
}
print "Exit code = 0\n"; 
exit 0;

