#!/usr/bin/env perl
# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

use XyceRegression::Tools;
use Cwd;

$Tools = XyceRegression::Tools->new();

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

$dirname = getcwd;
# the next substition accounts for testing Xyce on Windows under cygwin, msys2, MINGW64
$dirname =~ s/^\/cygdrive\/([A-Z,a-z])/\1:/;
$dirname =~ s/^\/([A-Z,a-z])\//\1:\//;

# hard code the name of the include file
$includeFileName = "sub1/sub2/include2_abs_path";
$libFileName = "sub1/sub2/lib2_abs_path";
$libName = "LIB_ABS";

# remove old output files
system("rm -f $CIRFILE\_modified*");

open(CIRFILE,"<$CIRFILE");
$CIRFILE2=$CIRFILE."_modified";
open(CIRFILE2,">$CIRFILE2");
while(<CIRFILE>)
{
  if (/^.INC/i)
  {
    $fullFileName = "$dirname/$includeFileName";
    print CIRFILE2 ".INC $fullFileName\n";
  }
  elsif (/^.LIB/i)
  {
    $fullFileName = "$dirname/$libFileName";
    print CIRFILE2 ".LIB $fullFileName $libName\n";
  }
  else
  {
    print CIRFILE2 $_;
  }
}
close(CIRFILE);
close(CIRFILE2);

# Run the modified netlist, which has the updated .INC line in it.
$retval=$Tools->wrapXyceAndVerify($XYCE,$XYCE_VERIFY,$GOLDPRN,$CIRFILE2);

print "Exit code = $retval\n";
exit $retval;
