#!/usr/bin/env perl

use Cwd;

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

$XYCE=$ARGV[0];

# This test exists SOLELY to verify that Xyce/ADMS now correctly
# bombs when a user has specified an invalid "xyceModelGroup" attribute.

# We only test that buildxyceplugin exits with an appropriate error.
# Prior to the fix of issue 399, it would bomb in the C++ compilation
# of the module instead of early, during ADMS processing.

# At this point we need to make sure we've actually been given a Xyce binary
# that is properly installed (via "make install"), and that the buildxyceplugin
# script exists along side it.

# First let us isolate the directory name of the Xyce binary given
# Before we can do that, we have to tease out the binary from the command line,
# which could include mpirun commands:
if ($XYCE =~ m/^(.*) (.*Xyce)$/)
{
  $MPIRUN=$1;
  $XYCE=$2;
}

print "XYCE: $XYCE\n";
print "MPIRUN: $MPIRUN\n";

$TESTROOT = cwd;

# Now, if $XYCE is identically "Xyce" we're running an installed binary
# that is in fact already in our PATH, and not being given a complete
# path name.  We do different things in that case.
if ($XYCE eq "Xyce")
{
    print "Running Xyce right out of path!\n";
    # is buildxyceplugin actually in our path, too?

    `which buildxyceplugin > /dev/null`;
    if ( $? != 0 )
    {
	print "buildxyceplugin is not in your path, so cannot build plugins!\n";
	print "Exit code = 1\n";
	exit 1;
    }
    else
    {
	print "Running buildxyceplugin right out of path!\n";
	$BUILDXYCEPLUGIN="buildxyceplugin";
    }

    # If buildxyceplugin is in the path, assume all else is fine (i.e. that
    # that libraries and headers are installed, too)
}
else
{
    # Otherwise it's an actual path, decode it and sanity check
    $XyceDir=`dirname $XYCE`;
    chomp($XyceDir);
    $XyceRoot=`dirname $XyceDir`;
    chomp($XyceRoot);
    $BinOrSrc=`basename $XyceDir`;
    chomp($BinOrSrc);

    if ($BinOrSrc != "bin")
    {
	print "The binary is in $XyceDir, and ";
	print "it is not in a bin directory --- this test cannot work.\n";
	print "Exit code = 1\n";
	exit 1;
    }

# Now, does buildxyceplugin exist?
    if (! (-x "$XyceDir/buildxyceplugin"))
    {
	print "the buildxyceplugin script was not found in $XyceDir\n";
	print "Exit code = 1\n";
	exit 1;
    }
    $BUILDXYCEPLUGIN="$XyceDir/buildxyceplugin";

# Are the libraries there?
    if (! (-d "$XyceRoot/lib" && (-e "$XyceRoot/lib/libxyce.so" || -e "$XyceRoot/lib/libxyce.dylib")))
    {
	print "the libraries were not found in $XyceRoot/lib\n";
	print "Exit code = 1\n";
	exit 1;
    }
}

# is admsXml actually in our path?
`admsXml -v > /dev/null`;
if ( $? != 0 )
{
    print "admsXml is not in your path, so cannot build plugins!\n";
    print "Exit code = 1\n";
    exit 1;
}

# Now reassemble the Xyce command we were given
if ($MPIRUN ne "")
{
    $XYCE="$MPIRUN $XYCE";
}

#Hooray, we can run this test.
$VERILOG_SOURCES="toys/Issue399.va";

print "Building the plugin...\n";

$result=system("$BUILDXYCEPLUGIN -o toys_Issue399 $VERILOG_SOURCES .");
if ($result != 0)
{
    print "Plugin creation failed as it should have";
    `grep "Xyce-specific module attribute xyceModelGroup given but has unknown value" buildxyceplugin.log`;
    if ($? == 0)
    {
        print "Appropriate error message found in buildxyceplugin.log\n";
        print "Exit code = 0";
        exit 0;
    }
    else
    {
        print "Appropriate error message NOT found in buildxyceplugin.log\n";
        print "Exit code = 1";
        exit 1;
    }

}
else
{
    print "Oddly, plugin creation worked, and it shouldn't have.\n";
    print "Exit code = 1";
    exit 1;
}

