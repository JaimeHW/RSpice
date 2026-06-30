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

# Since the shell script runs Xyce and the comparison program, it is
# responsible for capturing any error output from Xyce and the STDOUT & STDERR
# from the comparison program.  The outside script, run_xyce_regression,
# expects the STDERR output from Xyce to go into $CIRFILE.err, the STDOUT
# output from comparison to go into $CIRFILE.prn.out and the STDERR output from
# comparison to go into $CIRFILE.prn.err.  

use Time::Local;
$XYCE=$ARGV[0];
#$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

%month = ( 1  =>"Jan", 2  =>"Feb", 3  =>"Mar", 4  =>"Apr", 5  =>"May", 6  =>"Jun", 
           7  =>"Jul", 8  =>"Aug", 9  =>"Sep", 10 =>"Oct", 11 =>"Nov", 12 =>"Dec");
($S,$M,$H,$d,$m,$y) = (localtime) [0..5];
#$S = 59; $M = 32; $H = 23; $d = 31; $m = 11; $y = 106;
$time2 = timelocal($S,$M,$H,$d,$m,$y)+1;
$y %= 100; $y += 2000;
$m += 1;
if ($H < 12) { $AMPM = "AM" } else { $AMPM = "PM"; if ($H > 12) { $H -= 12; } }
$TIME = sprintf("%2.2d:%2.2d:%2.2d %s",$H,$M,$S,$AMPM);
$DATE = sprintf("%s %2.2d, %4.4d",$month{$m},$d,$y);

($S2,$M2,$H2,$d2,$m2,$y2) = (localtime($time2)) [0..5];
$y2 %= 100; $y2 += 2000;
$m2 += 1;
if ($H2 < 12) { $AMPM = "AM" } else { $AMPM = "PM"; if ($H2 > 12) { $H2 -= 12; } }
$TIME2 = sprintf("%2.2d:%2.2d:%2.2d %s",$H2,$M2,$S2,$AMPM2);
$DATE2 = sprintf("%s %2.2d, %4.4d",$month{$m2},$d2,$y2);

#sleep(1);

$CMD="$XYCE $CIRFILE > $CIRFILE.out 2> $CIRFILE.err";
if (system("$CMD") != 0) { print "Exit code = 10\n"; exit 10 }

$failure = 1;
open(PRN,"$CIRFILE.prn");
while ($line = <PRN>)
{
  if ($line =~ m/^TIME/)
  {
    $line =~ s/\'([ ])[DT]/=/g;
    @splitline = split("=",$line);
    $testTIME = $splitline[1];
    $testTIME =~ s/\'//g;
    $testDATE = $splitline[3];
    $testDATE =~ s/\'//g;
    if ( ( ($DATE  eq $testDATE) and ($TIME  eq $testTIME) ) or 
         ( ($DATE2 eq $testDATE) and ($TIME2 eq $testTIME) )     )
    { undef $failure; }
  }
  if (not defined($failure)) { break; }
}

if (defined($failure)) 
{
  print "Exit code = 2\n";
  exit 2
}
else
{
  print "Exit code = 0\n";
  exit 0
}

print "Exit code = 1\n";
exit 1
