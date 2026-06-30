#!/usr/bin/env perl

$verbose = 1;

use XyceRegression::Tools;
use Time::Local;

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
#$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

# save start of date/time window
$startTime = time;

# run xyce and create the probe file with timestamp
$CMD="$XYCE $CIRFILE > /dev/null 2> $CIRFILE.err";
if (system ("$CMD") != 0) 
{ 
  print "Exit code = 10\n"; 
  exit 10; 
}

# save end of date/time window
$endTime = time;



# open and read probe file
open (PRN, "$CIRFILE.prn");
while ($line = <PRN>)
{
  # examine the timestamp line only
  if ($line =~ m/^TIME/)
  {
    # reverse month hash (January starts at zero) 
    %monthTable = ("Jan" => 0, "Feb" => 1,  "Mar" => 2, 
                   "Apr" => 3, "May" => 4,  "Jun" => 5,
                   "Jul" => 6, "Aug" => 7,  "Sep" => 8, 
                   "Oct" => 9, "Nov" => 10, "Dec" => 11);

    # extract time/date components from the line
    $line =~ s/\'([ ])[DT]/=/g;
    @splitline = split ("=", $line);

    # parse time subcomponents
    $tempTIME = $splitline[1];
    $tempTIME =~ s/\'//g;
    $tempTIME =~ s/\s([AP]M)//g;
    ($hours, $minutes, $seconds) = split (":", $tempTIME);

    # AM/PM to 24H change
    if ($1 eq "PM" and $hours != 12 ) { $hours += 12; }
    if ($1 eq "AM" and $hours == 12 ) { $hours = 0; }

    # parse date subcomponents
    $tempDATE = $splitline[3];
    $tempDATE =~ s/\'//g;
    $tempDATE =~ s/\,//g;
    ($month, $day, $year) = split (" ", $tempDATE);


    # convert time/date subcomponents into epoch seconds
    $testTime = timelocal ($seconds, $minutes, $hours, 
                           $day, $monthTable{$month}, $year);


    # test that time fits within run-time window
    if ( ($startTime <= $testTime) and ($testTime <= $endTime) )
    { 
      print "Test Passed!\n";
      print "Exit code = 0\n"; 
      exit 0;
    }

    else 
    {
      print "Test Failed!\n";
      print "Exit code = 2\n"; 
      exit 2;
    } 
  }
}


# empty or malformed output file
print "Script error.  Output file is missing?  TIME line is missing?\n";
print "Exit code = 1\n"; 

exit 1;

