#!/usr/bin/env perl

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

use XyceRegression::Tools;
$Tools = XyceRegression::Tools->new();


$XYCE=$ARGV[0];
$CIRFILE=$ARGV[3];

#This script runs the ring oscillator BSIM6 test case, which has a transient
# solution that one should probably not be comparing across simulators and
# platforms --- the phase of the oscillations is dependent rather critically
# on the initial conditions.  Instead, all we'll do is check that Xyce is
# producing a waveform with the "right" period and "delay per stage".
#
# "Correct" means within .4ns of the HSpice-calculated delay_per_stage provided
# by the BSIM group, and within 20ns of the period.  Xyce does not produce
# a wave form that *exactly* matches HSpice.

$retval=$Tools->wrapXyce($XYCE,$CIRFILE);
if ($retval != 0)
{
    print "Exit code = $retval\n";
    exit $retval;
}

#check that we made a measure file
if (not -s "$CIRFILE.mt0")
{
    print "Exit code = 14\n";
    exit $retval;
}

$retval=0;
open (MEASURE_FILE,"$CIRFILE.mt0");
while (<MEASURE_FILE>)
{
    chomp;
    ($name, $sep, $value) = split(/s+/);
    if ($name eq "DELAY_PER_STAGE")
    {
        if (abs($value-2.9744e-8)>.4e-9)
        {
            print STDERR "Delay per stage $value out of acceptable range\n";
            $retval=2;
        }
    }
    if ($name eq "PERIOD")
    {
        if (abs($value-1.0113e-6)>20e-9)
        {
            print STDERR "Period $value out of acceptable range\n";
            $retval=2;
        }
    }
}

print "Exit code = $retval\n";
exit $retval;
