#!/usr/bin/env perl

use POSIX qw(ceil floor);

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
  if ($s[3] != int($s[2]) || $s[4] != POSIX::floor($s[2]) || $s[5] != POSIX::ceil($s[2]))
  {
      $retval=2;
      print STDERR "Compare failure at line $s[0], time $s[1], v $s[2]:  int $s[3], floor $s[4], ceil $s[5]\n";
      printf STDERR "  Correct data:  int %14.8e   floor %14.8e  ceil %14.8e\n",int($s[2]),POSIX::floor($s[2]), POSIX::ceil($s[2]);
  }
}
close INPUT;

print "Exit code = $retval\n";
exit $retval;
