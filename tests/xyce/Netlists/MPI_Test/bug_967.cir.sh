#!/usr/bin/env perl

use Cwd;

use XyceRegression::Tools;

$Tools = XyceRegression::Tools->new();

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script 
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

$TESTROOT = cwd;

# DEBUG: paths are hardcoded!
$PREFIX="";
$TARGETDIR="";
$XYCEROOT="missing ";


# Get anything before the path, which should be the mpirun/mpiexec command
$XYCE =~ m/^(.*) (.*Xyce)$/;
$MPIRUN = $1;
$XYCE = $2;

# Try to decode Xyce root directory by stripping off bin/Xyce or src/Xyce

$XYCE =~ m/(.*)\/bin\/Xyce.*/;
if (-d "$1") { $XYCEROOT=$1; }

$XYCE =~ m/([^\/]*)(.*)\/src\/(Release\/|Debug\/|RelWithDebInfo\/|)Xyce.*/;
if (-d "$2") { $PREFIX=$1; $XYCEROOT=$2; $TARGETDIR=$3}

print "XYCE: $XYCE\n"; 
print "PREFIX = $PREFIX\n";
print "XYCEROOT: $XYCEROOT\n";
print "TARGETDIR = $TARGETDIR\n";
print "MPIRUN: $MPIRUN\n";

#Check if we need a ".exe" extension (simply check for cygwin in uname)
$EXT="";
$result=system("uname | grep -i -q cygwin");
if ($result == 0)
{
  $EXT=".exe";
}

# set build dir and bin name
$MAKEROOT = "$XYCEROOT/src/test/MPITest/";
#UGH
if (-d "$MAKEROOT/CMakeFiles")
{
  $EXT="";
}
$TestProgram="testMPI$EXT";
$MPI_TEST = "$MAKEROOT/$TARGETDIR$TestProgram";

if (-d "$MAKEROOT") {
  if ( (-e "$MAKEROOT/Makefile") and not (-d "$MAKEROOT/CMakeFiles") ) {
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
    print "Using CMake, so assuming pre-built testMPI binary\n";
  } else {
    print "ERROR:    No build files!\n";
    $retval = 1;
  }
  chdir($TESTROOT);
} else {
    print "ERROR:    cannot chdir to $MAKEROOT\n";
  $retval = 1;
}

# Run the MPI test:
if (-x $MPI_TEST) {
  print "NOTICE:   running ----------------------\n";
  $CMD="$MPIRUN $MPI_TEST > testMPI.out 2> testMPI.err";
  print "$CMD\n";
  $result = system("$CMD");
  if ($result == 0){ $retval=0;} else {$retval=10;}

  # Now count if BAD was printed by this test.
  $err1 = `grep "BAD" testMPI.out`;
  if ( $err1 ) {
    # The test does not perform the communication properly, so the MPI installation is suspect.
    $retval = 2;
  }
  else {
    # The test correctly performs the communication.
    $retval = 0;
  }

} else {
  print "ERROR:    cannot find $MPI_TEST\n";
  $retval = 1;
}


print "Exit code = $retval\n"; exit $retval;

