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

# Note:  When this test is run by run_xyce_regression, and you pass it a Xyce
# binary, then this test is run on the Xyce binary and not on runxyce.bat.  If
# you instead give run_xyce_regression the runxyce.bat file, it will be tested
# correctly. 


$XYCE=$ARGV[0];
#$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

$arch = `uname -o`;
if ($arch !~ "Windows")
{
  $D = "\/";
}
else
{
  $D = "\\";
}


$pathwithspaces = "path with spaces";
mkdir "$pathwithspaces";
`cp $CIRFILE "$pathwithspaces$D$CIRFILE"`;
$CMD="$XYCE $pathwithspaces$D$CIRFILE > $CIRFILE.out1 2> $CIRFILE.err";
#print "CMD = >>$CMD<<\n";
if (system("$CMD") == 0) 
{ 
  #print "Xyce ran without quotes\n";
  print "Exit code = 2\n";
  exit 2 
}
else
{
  #print "Xyce exited due to no quotes\n";
}

$CMD="$XYCE \"$pathwithspaces$D$CIRFILE\" > $CIRFILE.out2 2> $CIRFILE.err";
#print "CMD = >>$CMD<<\n";
if (system("$CMD") != 0) 
{ 
  #print "Xyce EXITED WITH ERROR\n";
  print "Exit code = 10\n";
  exit 10 
}
else
{
  #print "Xyce ran correctly with quotes\n";
  `rm -f "$pathwithspaces$D$CIRFILE"`;
  `rm -f "$pathwithspaces$D$CIRFILE.prn"`;
  `rmdir "$pathwithspaces"`;
  print "Exit code = 0\n";
  exit 0
}

print "shell script exited incorrectly\n";
print "Exit code = 1\n";
exit 1


