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

$CIRFILE="rlc_series_vccs.cir";
$CIRFILE2="rlc_series_vccs_nogenext.cir";

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
print("XYCE_LIBTEST = $XYCE_LIBTEST\n");
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
  $CMD="$XYCE_LIBTEST -iotest1 $CIRFILE > $CIRFILE.out 2> $CIRFILE.err";
  print "$CMD\n";
  $result = system("$CMD");
  if ($result == 0){ $retval=0;} else {$retval=10;}

} else {
  print "ERROR:    cannot find $XYCE_LIBTEST\n";
  $retval = 1;
}

if ($retval==0)
{

# Now run the Xyce-only version
    print "NOTICE:   running Xyce equivalent----------------------\n";
    $CMD="$XYCE $CIRFILE2 > $CIRFILE2.out 2> $CIRFILE2.err";
    $result = system("$CMD");
    if ($result == 0){ $retval=0;} else {$retval=10;}
}

# Now compare them:
if ($retval==0 && $result == 0)
{
    print "NOTICE:  Comparing ------------------------------------\n";
    `echo "NOTICE:  Comparing ------------------------------------" > $CIRFILE.prn.out`;
    `echo "NOTICE:  Comparing ------------------------------------" > $CIRFILE.prn.err`;
    $CMD="$XYCE_VERIFY $CIRFILE $CIRFILE2.prn $CIRFILE.prn > $CIRFILE.prn.out 2> $CIRFILE.prn.err";
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
        print "NOTICE:  Comparing .print and external out----------------\n";
        `echo "NOTICE:  Comparing .print and external out----------------" >> $CIRFILE.prn.out`;
        `echo "NOTICE:  Comparing .print and external out----------------" >> $CIRFILE.prn.err`;
        $CMD="$XYCE_VERIFY $CIRFILE $CIRFILE.prn ioTest1.out >> $CIRFILE.prn.out 2>> $CIRFILE.prn.err";
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
            print STDERR "Comparison failed between GenExt .print output and ExternalOutput mechanism.\n";
            $retval=2;
        }
    }
    if ($retval == 0)
    {
        print "NOTICE:  Comparing .print starvars and external out----------------\n";
        `echo "NOTICE:  Comparing .print starvars and external out----------------" >> $CIRFILE.prn.out`;
        `echo "NOTICE:  Comparing .print starvars and external out----------------" >> $CIRFILE.prn.err`;

        # Annoyance:  xyce_verify does not handle the case where
        # there are two .print tran lines to different files.  It only
        # looks at the last one for the purpose of header verification.
        # It also has no concept of V(*) and I(*).
        # So we're going to fake this out by constructing a FAKE netlist
        # with a fake print line that wasn't actually used, by taking
        # the header from starvars.prn and stripping off Index and TIME.

        open(STARVARS,"<starvar.prn") || die "Cannot open starvar.prn";
        $svhead=<STARVARS>;
        close STARVARS;
        chomp($svhead);
        @svvars=split(" ",$svhead);
        shift @svvars; shift @svvars;
        
        open (CIRFILE,"<$CIRFILE");
        `rm -f fakecir`;
        open (FAKECIR,">fakecir");
        while (<CIRFILE>)
        {
            if (! (/^.print tran/i))
            {
                print FAKECIR;
            }
            else
            {
                print FAKECIR ".print tran ";
                foreach $var (@svvars)
                {
                    print FAKECIR " $var";
                }
                print FAKECIR "\n.end\n";
                last;
            }
        }
        close FAKECIR;
        close CIRFILE;
        
        $CMD="$XYCE_VERIFY fakecir starvar.prn ioTest1a.out >> $CIRFILE.prn.out 2>> $CIRFILE.prn.err";
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
            print STDERR "Comparison passed between GenExt .print starvar output and ExternalOutput mechanism, but duplicate lines present so marking as failure.\n";
                $retval=2;
            }
            else
            {
                $retval=0;
            }
        } 
        else 
        {
            print STDERR "Comparison failed between GenExt .print starvar output and ExternalOutput mechanism.\n";
            $retval=2;
        }
    }
    if ($retval == 0)
    {
        $fc = $XYCE_VERIFY;
        $fc=~ s/xyce_verify/file_compare/;
        print "NOTICE:  Comparing .print NOINDEX line and external out with prepended time ----------------\n";
        `echo "NOTICE:  Comparing .print NOINDEX line and external out with prepended time ----------------" >> $CIRFILE.prn.out`;
        `echo "NOTICE:  Comparing .print NOINDEX line and external out with prepended time ----------------" >> $CIRFILE.prn.err`;
        $absTol=1e-5;
        $relTol=1e-3;
        $zeroTol=1e-16;
        $CMD="$fc ioTest1_noindex.out noindex.prn $absTol $relTol $zeroTol >> $CIRFILE.prn.out 2>> $CIRFILE.prn.err";
        $retval = system("$CMD");
        $retval = $retval >> 8;
        if ( $retval != 0 )
        {
            print STDERR "Comparison between noindex print line and external output with prepended time failed.";
            $retval=2;
        }
    }
}    

print "Exit code = $retval\n"; exit $retval;

