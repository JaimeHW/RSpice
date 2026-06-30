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

# Since the shell script runs Xyce and the comparison program, it is
# responsible for capturing any error output from Xyce and the STDOUT & STDERR
# from the comparison program.  The outside script, run_xyce_regression,
# expects the STDERR output from Xyce to go into $CIRFILE.err, the STDOUT
# output from comparison to go into $CIRFILE.prn.out and the STDERR output from
# comparison to go into $CIRFILE.prn.err.  

# these search strings are supposed to occur one right after the other in the
# error output.  Note that parens are escaped with \\
@searchstrings = ( "Netlist error in file PRINT_Line_Missing_Paren.cir at or near line 18",
                   "Unrecognized voltage specification in .print near .PRINT TRAN V \\( 1",
                   "Netlist error in file PRINT_Line_Missing_Paren.cir at or near line 19",
                   "Unrecognized voltage specification in .print near .PRINT TRAN V \\( \\)",
                   "Netlist error in file PRINT_Line_Missing_Paren.cir at or near line 20",
                   "Unrecognized voltage specification in .print near .PRINT TRAN V \\(",
                   "Netlist error in file PRINT_Line_Missing_Paren.cir at or near line 21",
                   " Unrecognized current specification in .print near .PRINT TRAN I \\( V1",
                   "Netlist error in file PRINT_Line_Missing_Paren.cir at or near line 22",
                   "Unrecognized current specification in .print near .PRINT TRAN I \\( \\)",
                   "Netlist error in file PRINT_Line_Missing_Paren.cir at or near line 23",
                   "Unrecognized current specification in .print near .PRINT TRAN I \\(",
                   "Netlist error in file PRINT_Line_Missing_Paren.cir at or near line 24",
                   "Unrecognized parenthetical specification in .print near .PRINT TRAN N \\( R1",
                   "Netlist error in file PRINT_Line_Missing_Paren.cir at or near line 25",
                   "Unrecognized parenthetical specification in .print near .PRINT TRAN N \\( \\)",
                   "Netlist error in file PRINT_Line_Missing_Paren.cir at or near line 26",
                   "Unrecognized parenthetical specification in .print near .PRINT TRAN N \\(",
                   "Netlist error in file PRINT_Line_Missing_Paren.cir at or near line 27",
                   "Unrecognized power specification in .print near .PRINT TRAN P \\( R1",
                   "Netlist error in file PRINT_Line_Missing_Paren.cir at or near line 28",
                   "Unrecognized power specification in .print near .PRINT TRAN P \\( \\)",
                   "Netlist error in file PRINT_Line_Missing_Paren.cir at or near line 29",
                   "Unrecognized power specification in .print near .PRINT TRAN P \\(",
                   "Netlist error in file PRINT_Line_Missing_Paren.cir at or near line 30",
                   "Unrecognized power specification in .print near .PRINT TRAN W \\( R1",
                   "Netlist error in file PRINT_Line_Missing_Paren.cir at or near line 31",
                   "Unrecognized power specification in .print near .PRINT TRAN W \\( \\)",
                   "Netlist error in file PRINT_Line_Missing_Paren.cir at or near line 32",
                   "Unrecognized power specification in .print near .PRINT TRAN W \\(" );

$XYCE=$ARGV[0];
#$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

print "Testing $CIRFILE\n";

$retval = $Tools->runAndCheckError($CIRFILE,$XYCE,@searchstrings);
print "Exit code = $retval\n";
exit $retval


