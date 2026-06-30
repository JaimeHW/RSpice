#!/usr/bin/env perl

use Digest::MD5;

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
$CIRFILE=$ARGV[3];

# Store the MD5 checksum of the original netlist
$ctx = Digest::MD5->new;
open( $fh, '<', $CIRFILE);
$ctx->addfile($fh);
$orig_digest=$ctx->hexdigest;
close($fh);

$CMD="$XYCE $CIRFILE > $CIRFILE.out 2> $CIRFILE.err";
if (system($CMD) != 0)
{
    `echo "Xyce EXITED WITH ERROR! on $CIRFILE" >> $CIRFILE.err`;
    print "Exit code = 10\n"; exit 10;
}
    
# Now check that we haven't screwed up the netlist
# Store the MD5 checksum of the original netlist
$ctx2 = Digest::MD5->new;
open( $fh, '<', $CIRFILE);
$ctx2->addfile($fh);
$final_digest=$ctx2->hexdigest;
close($fh);

if ( $final_digest eq $orig_digest)
{
    print "Exit code = 0\n"; exit 0;
}
else
{
    print "Xyce overwrote the input!\n";
    print "Exit code = 2\n"; exit 2;
}
