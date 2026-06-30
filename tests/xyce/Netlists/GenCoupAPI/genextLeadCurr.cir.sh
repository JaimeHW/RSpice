#!/usr/bin/env perl

use Cwd;

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

$CIRFILE="genextLeadCurr.cir";

$TESTROOT = cwd;

# DEBUG: paths are hardcoded!
$PREFIX="";
$TARGETDIR="";
$XYCEROOT="missing ";

print "XYCE = $XYCE\n";

# Try to decode Xyce root directory by stripping off bin/Xyce or src/Xyce
$XYCE =~ m/([^\/]*)(.*)\/bin\/Xyce.*/;
if (-d "$2") { $PREFIX=$1; $XYCEROOT=$2; }

$XYCE =~ m/([^\/]*)(.*)\/src\/(Release\/|Debug\/|RelWithDebInfo\/|)Xyce.*/;
if (-d "$2") { $PREFIX=$1; $XYCEROOT=$2; $TARGETDIR=$3}

print "PREFIX = $PREFIX\n";
print "XYCEROOT = $XYCEROOT\n";
print "TARGETDIR = $TARGETDIR\n";

#Check if we need a ".exe" extension (simply check for cygwin in uname)
$EXT="";
$result=system("uname | grep -i -q cygwin");
if ($result == 0)
{
  $EXT=".exe";
}

# set build dir and bin name
$MAKEROOT = "$XYCEROOT/src/test/GenExtTestHarnesses/";
#UGH
if (-d "$MAKEROOT/CMakeFiles")
{
  $EXT="";
}
$TestProgram="testGenCoup$EXT";
$XYCE_LIBTEST = "$MAKEROOT/$TARGETDIR$TestProgram";

if (-d "$MAKEROOT") {
  if ( (-e "$MAKEROOT/Makefile") and not (-d "$MAKEROOT/CMakeFiles") ) {
    chdir($MAKEROOT);
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
} else {
    print "ERROR:    cannot chdir to $MAKEROOT\n";
  $retval = 1;
}

# Run the GenExt version:
if (-x $XYCE_LIBTEST) {
  print "NOTICE:   running ----------------------\n";
  $CMD="$XYCE_LIBTEST -iotest2 $CIRFILE > $CIRFILE.out 2> $CIRFILE.err";
  print "$CMD\n";
  $result = system("$CMD");
  if ($result == 0){ $retval=0;} else {$retval=10;}

} else {
  print "ERROR:    cannot find $XYCE_LIBTEST\n";
  $retval = 1;
}

if ($retval==0)
{

    # Now generate a fake gold standard with all columns equal to zero
    $GSFILE="$CIRFILE.prn.gs";
    open (INPUT, "$CIRFILE.prn");
    open (OUTPUT, ">$GSFILE");
    while ($line = <INPUT>)
    {
        #handle first and last lines
        if ($line =~ m/^[IE]/)
        {
            print OUTPUT "$line";
            next;
        }
        @s = split(" ",$line);
        $a=0;
        printf OUTPUT "%3d  %14.8e   %14.8e   %14.8e   %14.8e   %14.8e   %14.8e\n",$s[0],$s[1],$a,$a,$a,$a,$a;
    }
    close(INPUT);
    close(OUTPUT);


    # Now compare them:
    print "NOTICE:  Comparing ------------------------------------\n";
    $CMD="$XYCE_VERIFY $CIRFILE $GSFILE $CIRFILE.prn > $CIRFILE.prn.out 2> $CIRFILE.prn.err";
    $result = system("$CMD");
    if ($result == 0)
    { 
        # Comparison passed... but did we get any warnings about duplicate 
        # lines?
        $CMD="grep 'WARNING.*Throwing away line with duplicate' $CIRFILE.prn.err";
        $result= system("$CMD");
        if ( $result==0)
        {
            #Found such a line, report as failure.
            print STDERR "Comparison passed between Xyce-only and GenExt test output, but duplicate lines present so marking as failure.\n";
            $retval=2;
        }
        else
        {
            $retval=0;
        }
    } 
    else 
    {
        print STDERR "Comparison failed between Xyce-only and GenExt test output.\n";
        $retval=2;
    }
    if ($retval == 0)
    {
        print "NOTICE:  Comparing .print and external output----------------\n";
        $CMD="$XYCE_VERIFY $CIRFILE $CIRFILE.prn ioTest2.out >> $CIRFILE.prn.out 2>> $CIRFILE.prn.err";
        $result = system("$CMD");
        if ($result == 0)
        { 
            # Comparison passed... but did we get any warnings about duplicate 
            # lines?
            $CMD="grep 'WARNING.*Throwing away line with duplicate' $CIRFILE.prn.err";
            $result= system("$CMD");
            if ( $result==0)
            {
                #Found such a line, report as failure.
                print STDERR "Comparison passed between GenExt .print output and ExternalOutput mechanism, but duplicate lines present so marking as failure.\n";
                $retval=2;
            }
            else
            {
                $retval=0;
            }
        } 
        else 
        {
            $retval=2;
            print STDERR "Comparison failed between GenExt .print output and ExternalOutput mechanism.\n";
        }
    }
}    

print "Exit code = $retval\n"; exit $retval;

