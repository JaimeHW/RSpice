#!/usr/bin/env perl

use XyceRegression::Tools;

$Tools = XyceRegression::Tools->new();
# $Tools->setDebug(1);
# $Tools->setVerbose(1);

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
#$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

$CIR1=$CIRFILE;
$CIR2=$CIRFILE;
$CIR2 =~ s/a.cir/b.cir/;
$CIR3=$CIRFILE;
$CIR3 =~ s/a.cir/c.cir/;

`rm -f $CIR1.hb-test-hb-fd.txt`;
`rm -f $CIR1.hb-test-hb-td.txt`;
`rm -f $CIR1.hb_ic.prn`;
`rm -f $CIR1.startup.prn`;
`$XYCE $CIR1`;

`rm -f $CIR2.hb-test-hb-fd.txt.*`;
`rm -f $CIR2.HB.TD.prn`;
`$XYCE $CIR2`;

`rm -f $CIR3.hb-test-hb-fd.txt`;
`rm -f $CIR3.hb-test-hb-td.txt`;
`rm -f $CIR3.hb-test-hb-ic.txt`;
`rm -f $CIR3.hb-test-hb-startup.txt`;
`$XYCE $CIR3`;

$exitcode = 0;
if ( ! (-f "$CIR1.hb-test-hb-fd.txt" )) { print STDERR "Could not find $CIR1.hb-test-hb-fd.txt\n"; $exitcode = 14; }
if ( ! (-f "$CIR1.hb-test-hb-td.txt" )) { print STDERR "Could not find $CIR1.hb-test-hb-td.txt\n"; $exitcode = 14; }
if ( (-f "$CIR1.hb_ic.prn" )) { print STDERR "Made $CIR1.hb_ic.prn when it should not\n"; $exitcode = 2; }
if ( (-f "$CIR1.startup.prn" )) { print STDERR "Made $CIR1.startup.prn when it should not\n"; $exitcode = 2; }

if ( ! (-f "$CIR2.hb-test-hb-fd.txt.HB.FD.prn" )) { print STDERR "Could not find $CIR2.hb-test-hb-fd.txt.HB.FD.prn\n"; $exitcode = 14; }
if ( ! (-f "$CIR2.hb-test-hb-fd.txt.HB.TD.prn" )) { print STDERR "Could not find $CIR2.hb-test-hb-fd.txt.HB.TD.prn\n"; $exitcode = 14; }
if ( ! (-f "$CIR2.hb-test-hb-fd.txt.hb_ic.prn" )) { print STDERR "Could not find $CIR2.hb-test-hb-fd.txt.hb_ic.prn\n"; $exitcode = 14; }
if ( ! (-f "$CIR2.hb-test-hb-fd.txt.startup.prn" )) { print STDERR "Could not find $CIR2.hb-test-hb-fd.txt.startup.prn\n"; $exitcode = 14; }

if ( ! (-f "$CIR3.hb-test-hb-fd.txt" )) { print STDERR "Could not find $CIR3.hb-test-hb-fd.txt\n"; $exitcode = 14; }
if ( ! (-f "$CIR3.hb-test-hb-td.txt" )) { print STDERR "Could not find $CIR3.hb-test-hb-td.txt\n"; $exitcode = 14; }
if ( ! (-f "$CIR3.hb-test-hb-ic.txt" )) { print STDERR "Could not find $CIR3.hb-test-hb-ic.txt\n"; $exitcode = 14; }
if ( ! (-f "$CIR3.hb-test-hb-startup.txt" )) { print STDERR "Could not find $CIR3.hb-test-hb-startup.txt\n"; $exitcode = 14; }
if ( -f "foo") { print STDERR "Made file foo, when it should not\n"; $exitcode = 14; }

print "Exit code = $exitcode\n";
exit $exitcode;
