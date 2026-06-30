#!/usr/bin/env perl

use POSIX qw(round);

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script 
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

# This script runs a netlist that uses int, floor, and ceil in an expression
# and checks the output against perl's posix int, floor, and ceil functions.
#


$XYCE=$ARGV[0];
$CIRFILE=$ARGV[3];

# We're using wrapXyce for its error handling
use XyceRegression::Tools;
$Tools = XyceRegression::Tools->new();

$retval=$Tools->wrapXyce($XYCE,$CIRFILE);
if ($retval != 0)
{
    print "Exit code = $retval\n"; 
    exit $retval; 
}
if (not -s "$CIRFILE.prn" ) 
{ 
    print "Exit code = 14\n"; 
    exit 14; 
}

# OK, we have a valid run now.  Did we actually produce good output?
# We're going to be very demanding here: int, floor, and ceil must match
# exactly
$retval=0;

open(INPUT,"$CIRFILE.prn");
while ($line = <INPUT>)
{
  # Handle first and last lines:
  if ($line =~ m/^[IE]/) 
  { 
    next; 
  }
  @s = split(" ",$line);
  if ($s[3] != POSIX::round($s[2]))
  {
      $retval=2;
      print STDERR "Compare failure at line $s[0], time $s[1], v $s[2]:  nint $s[3]\n";
      printf STDERR "  Correct data:  nint %14.8e \n",POSIX::round($s[2]);
  }
}
close INPUT;

print "Exit code = $retval\n";
exit $retval;
