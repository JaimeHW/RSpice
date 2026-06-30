#!/usr/bin/env perl

use Cwd;

use XyceRegression::Tools;

$Tools = XyceRegression::Tools->new();
#$Tools->setFork(1);
#$Tools->setDebug(1);

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

$XYCE="/Users/erkeite/XYCE/GitRepos/BUILD/OSX_CLANG/src/Xyce";
$XYCE_VERIFY="/Users/erkeite/XYCE/GitRepos/Xyce_Regression/TestScripts/xyce_verify.pl";
$XYCE_COMPARE="/Users/erkeite/XYCE/GitRepos/Xyce_Regression/TestScripts/file_compare.pl";
$CIRFILE="resInner.cir";
$GOLDPRN="resInner.cir.prn";
$retval=0;

$TESTROOT = cwd;

# DEBUG: paths are hardcoded!
$XYCEROOT="missing ";
$BUILDTYPE="";

# Try to decode Xyce root directory by stripping off bin/Xyce or src/Xyce
$XYCE =~ m/(.*)\/bin\/Xyce.*/;
if (-d "$1") { $XYCEROOT=$1; }

print "$XYCE\n";
print "$1\n";

$XYCE =~ m/(.*)\/src\/Xyce.*/;
if (-d "$1") { $XYCEROOT=$1; }

$XYCE =~ m/(.*)\/src\/Release\/Xyce.*/;
if (-d "$1") { $XYCEROOT=$1; $BUILDTYPE="Release/";}

print "$XYCE\n";
print "$XYCEROOT\n";

#Check if we need a ".exe" extension (simply check for cygwin in uname)
#$EXT="";
#$result=system("uname | grep -i -q cygwin");
#if ($result == 0)
#{
#  $EXT=".exe";
#}
#
# set build dir and bin name
$MAKEROOT = "$XYCEROOT/src/test/TwoLevelNewton/";
print "$MAKEROOT\n";

#UGH
if (-d "$MAKEROOT/CMakeFiles")
{
  $EXT="";
}
$TestProgram="twoLevelNewtonLinearDC$EXT";
$XYCE_LIBTEST = "$MAKEROOT/$BUILDTYPE$TestProgram";

if (-d "$MAKEROOT") {
  if ( (-e "$MAKEROOT/Makefile") and not (-d "$MAKEROOT/CMakeFiles") ) {
    print "The MAKEROOT directory exists\n";
    chdir($MAKEROOT);
    print "NOTICE:   make clean -------------------\n";
    $result = system("make clean");
    print "NOTICE:   make -------------------------\n";
    $result += system("make $TestProgram");
    if($result) {
      print "WARNING:  make failures! ---------------\n";
      $retval = $result;
    }
  } elsif (-d "$MAKEROOT/CMakeFiles") {
    print "Using CMake, so assuming pre-built $TestProgram binary\n";
  } else {
    print "ERROR:    No build files!\n";
    $retval = 1;
  }
  chdir($TESTROOT);
} 
else 
  {
    print "ERROR: cannot chdir to $MAKEROOT\n";
  $retval = 1;
}

#
#
# Run the Xygra version:
#if (-x $XYCE_LIBTEST) {
#  print "NOTICE:   running ----------------------\n";
#  $CMD="$XYCE_LIBTEST $CIRFILE > $CIRFILE.out 2> $CIRFILE.err";
#  print "$CMD\n";
#  $result = system("$CMD");
#  if ($result == 0){ $retval=0;} else {$retval=10;}
#
#} else {
#  print "ERROR:    cannot find $XYCE_LIBTEST\n";
#  $retval = 1;
#}
#
#if ($retval==0)
#{
# Now run the Xyce-only version
#    print "NOTICE:   running Xyce equivalent----------------------\n";
#    $CMD="$XYCE $CIRFILE2 > $CIRFILE2.out 2> $CIRFILE2.err";
#    $result = system("$CMD");
#    if ($result == 0){ $retval=0;} else {$retval=10;}
#}

# Now compare them:
#if ($retval==0 && $result == 0)
#{
#    print "NOTICE:  Comparing ------------------------------------\n";
#    $CMD="$XYCE_VERIFY $CIRFILE $CIRFILE2.prn $CIRFILE.prn > $CIRFILE.prn.out 2> $CIRFILE.prn.err";
#    $result = system("$CMD");
#    if ($result == 0)
#    { 
#        # Comparison passed... but did we get any warnings about duplicate 
#        # lines?
#        $CMD="grep 'WARNING.*Throwing away line with duplicate' $CIRFILE.prn.err";
#        $result= system("$CMD");
#        if ( $result==0)
#        {
#            #Found such a line, report as failure.
#            $retval=2;
#        }
#        else
#        {
#            $retval=0;
#        }
#    } 
#    else 
#    {
#        $retval=2;
#    }
#}    

print "Exit code = $retval\n"; exit $retval;

