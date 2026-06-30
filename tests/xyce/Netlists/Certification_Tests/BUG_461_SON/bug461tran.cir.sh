#!/usr/bin/env perl

use XyceRegression::Tools;

$Tools = XyceRegression::Tools->new();

# Notes on what this test does:
#
# This script runs two variantes of test circuit bug461.cir in 4 ways:
#  1. bug461_printFormatRaw.cir with ".print format=raw" in ascii raw mode 
#     The raw file is saved as bug269_printFormatRaw.cir.ascii.raw
#  2. bug461_printFormatRaw.cir with ".print format=raw" in binary raw mode 
#     The raw file is saved as bug269_printFormatRaw.cir.raw
#  3. bug461.cir with the command line option "-a -r" to generate an ascii raw 
#     output file containing all of the simulation data.
#  4. bug461.cir with the command line option "-r" to generate a binary raw 
#     output file containing all of the simulation data.

#  Then a similar pair of netlists are run that have
#  ".options output outputversioninrawfile=true"  in the netlist (in the
#  same 4 variations as above to test ascii and binary raw as well as
#  print line vs command line raw).

# After all those runs the following RAW files are checked for the
# version string:
#
# bug461tran_printFormatRaw.cir.ascii.raw
# bug461tran_printFormatRaw.cir.aw
# bug461tran.cir.ascii.raw
# bug461tran.cir.raw
#
#
# bug461tran_printFormatRaw_versionOn.cir.ascii.raw
# bug461tran_printFormatRaw_versionOn.cir.aw
# bug461tran.cir.ascii_versionOn.raw
# bug461tran.cir_versionOn.raw
#

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

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

# set an absTol and relTol for comparing the values from the raw files
# absTol may seem high, but some value are of the order 1e4 and we are limited
# in accuracy by what is printed to the file (i.e. about 8 digits of accuracy)
my $absTol=1.0e-4;
my $relTol=1.0e-3;

#
# Run Xyce on bug461_printFormatRaw.cir WITH command line option "-a" to make an 
# ascii raw file which outputs just what is on the print line.
$retval = -1;
$CIRFILEwithVersion="bug461tran_versionOn.cir";
$CIRFILEwithPrintRaw="bug461tran_printFormatRaw.cir";
$CIRFILEwithPrintRawAndVersion="bug461tran_printFormatRaw_versionOn.cir";



# wrapXyce is not fully compatible with AC as it
# signals an error if the output file doesn't end with End Of Xyce(TM) Simulation
# which AC does not output.
#$retval=$Tools->wrapXyce($XYCE,$CIRFILEwithPrintRaw);
$CMD="$XYCE -a $CIRFILEwithPrintRaw > $CIRFILEwithPrintRaw.ascii.out 2> $CIRFILEwithPrintRaw.ascii.err";
#
# lower byte is not relevent here.  xyce_verify's return code is in
# the upper byte to shift it 8 bits.
$retval = system("$CMD");
$retval = $retval >> 8;

# check that Xyce exited without error
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }
# check that Xyce made a raw file
if (not -s "$CIRFILEwithPrintRaw.raw" ) { print STDERR "Missing -a mode $CIRFILEwithPrintRaw.raw using command $CMD\n"; print "Exit code = 14\n"; exit 14; }
rename( "$CIRFILEwithPrintRaw.raw", "$CIRFILEwithPrintRaw.ascii.raw");

#
# Run Xyce on bug461_printFormatRaw.cir WITHOUT command line option "-a" to make a
# binary raw file which outputs just what is on the print line.
$CMD="$XYCE $CIRFILEwithPrintRaw > $CIRFILEwithPrintRaw.out 2> $CIRFILEwithPrintRaw.err";
#
# lower byte is not relevent here.  xyce_verify's return code is in
# the upper byte to shift it 8 bits.
$retval = system("$CMD");
$retval = $retval >> 8;

# check that Xyce exited without error
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }
# check that Xyce made a prn file
if (not -s "$CIRFILEwithPrintRaw.raw" ) { print STDERR "Missing $CIRFILEwithPrintRaw.raw\n"; print "Exit code = 14\n"; exit 14; }

#
#
# Run Xyce on bug461.cir WITH command line option "-a -r <raw file name>" to make an 
# ascii raw file which outputs all of the simulation data. 
#
$CMD="$XYCE -a -r $CIRFILE.ascii.raw $CIRFILE > $CIRFILE.ascii.out 2> $CIRFILE.ascii.err";
#
# lower byte is not relevent here.  xyce_verify's return code is in
# the upper byte to shift it 8 bits.
$retval = system("$CMD");
$retval = $retval >> 8;

if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }
# check that Xyce made a prn file
if (not -s "$CIRFILE.ascii.raw" ) { print "Missing -a -r mode $CIRFILEwithPrintRaw.ascii.raw\n"; print "Exit code = 14\n"; exit 14; }

#
# Run Xyce on bug461.cir WITH command line option "-r <raw file name>" to make a
# binary raw file which outputs all of the simulation data. 
#
#
$CMD="$XYCE -r $CIRFILE.raw $CIRFILE > $CIRFILE.out 2> $CIRFILE.err";
#
# lower byte is not relevent here.  xyce_verify's return code is in
# the upper byte to shift it 8 bits.
$retval = system("$CMD");
$retval = $retval >> 8;

if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }
# check that Xyce made a prn file
if (not -s "$CIRFILE.raw" ) { print "Missing -r mode $CIRFILEwithPrintRaw.raw\n"; print "Exit code = 14\n"; exit 14; }


# now do the same 4 runs of Xyce but with netlists that are set to output the version string in 
# the raw file header.

#
# Run Xyce on bug461_printFormatRaw_versionOn.cir WITH command line option "-a" to make an 
# ascii raw file which outputs just what is on the print line.
$retval = -1;
# wrapXyce is not fully compatible with AC as it
# signals an error if the output file doesn't end with End Of Xyce(TM) Simulation
# which AC does not output.
#$retval=$Tools->wrapXyce($XYCE,$CIRFILEwithPrintRaw);
$CMD="$XYCE -a $CIRFILEwithPrintRawAndVersion > $CIRFILEwithPrintRawAndVersion.ascii.out 2> $CIRFILEwithPrintRawAndVersion.ascii.err";
#
# lower byte is not relevent here.  xyce_verify's return code is in
# the upper byte to shift it 8 bits.
$retval = system("$CMD");
$retval = $retval >> 8;

# check that Xyce exited without error
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }
# check that Xyce made a raw file
if (not -s "$CIRFILEwithPrintRawAndVersion.raw" ) { print STDERR "Missing -a mode $CIRFILEwithPrintRawAndVersion.raw using command $CMD\n"; print "Exit code = 14\n"; exit 14; }
rename( "$CIRFILEwithPrintRawAndVersion.raw", "$CIRFILEwithPrintRawAndVersion.ascii.raw");

#
# Run Xyce on bug461_printFormatRaw_versionON.cir WITHOUT command line option "-a" to make a
# binary raw file which outputs just what is on the print line.
$CMD="$XYCE $CIRFILEwithPrintRawAndVersion > $CIRFILEwithPrintRawAndVersion.out 2> $CIRFILEwithPrintRawAndVersion.err";
#
# lower byte is not relevent here.  xyce_verify's return code is in
# the upper byte to shift it 8 bits.
$retval = system("$CMD");
$retval = $retval >> 8;

# check that Xyce exited without error
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }
# check that Xyce made a prn file
if (not -s "$CIRFILEwithPrintRawAndVersion.raw" ) { print STDERR "Missing $CIRFILEwithPrintRawAndVersion.raw\n"; print "Exit code = 14\n"; exit 14; }

#
#
# Run Xyce on bug461.cir WITH command line option "-a -r <raw file name>" to make an 
# ascii raw file which outputs all of the simulation data. 
#CIRFILEwithVersion
$CMD="$XYCE -a -r $CIRFILEwithVersion.ascii.raw $CIRFILEwithVersion > $CIRFILEwithVersion.ascii.out 2> $CIRFILEwithVersion.ascii.err";
#
# lower byte is not relevent here.  xyce_verify's return code is in
# the upper byte to shift it 8 bits.
$retval = system("$CMD");
$retval = $retval >> 8;

if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }
# check that Xyce made a prn file
if (not -s "$CIRFILEwithVersion.ascii.raw" ) { print "Missing -a -r mode $CIRFILEwithVersion.ascii.raw\n"; print "Exit code = 14\n"; exit 14; }

#
# Run Xyce on bug461.cir WITH command line option "-r <raw file name>" to make a
# binary raw file which outputs all of the simulation data. 
#
#
$CMD="$XYCE -r $CIRFILEwithVersion.raw $CIRFILEwithVersion > $CIRFILEwithVersion.out 2> $CIRFILEwithVersion.err";
#
# lower byte is not relevent here.  xyce_verify's return code is in
# the upper byte to shift it 8 bits.
$retval = system("$CMD");
$retval = $retval >> 8;

if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }
# check that Xyce made a prn file
if (not -s "$CIRFILEwithVersion.raw" ) { print "Missing -r mode $CIRFILEwithVersion.raw\n"; print "Exit code = 14\n"; exit 14; }


#
# now examine the raw files to see that they are valid
# Certification_Tests/BUG_269_SON checks the details of the raw file. This script cobbled from
# that bug has been reduced to only check that the version string is or is not in a
# given raw file as appropriate.

# These raw files should NOT have the version string
#
# $CIRFILE.raw
# $CIRFILE.ascii.raw
# $CIRFILEwithPrintRaw.raw
# $CIRFILEwithPrintRaw.ascii.raw
#
# And these should have the version string
#
# $CIRFILEwithVersion.raw
# $CIRFILEwithVersion.ascii.raw
# $CIRFILEwithPrintRawAndVersion.raw
# $CIRFILEwithPrintRawAndVersion.ascii.raw
#



# This subroutine just checks the 
# headers for a "Version:" tag and if found it returns a positive value
# and the value of the string after the ":". 
#

sub checkVersionInRawFile {
  # 
  # tries to read a version string from a raw file 
  # and returns the $version variable as either blank "" or with the 
  # contents of the version string in the header. 
  my $filename = @_[0];
 
  # things I'll return to the caller. 
  my $version;
  my $versionFound = 0;
  open(INPUT_ASCII,$filename) or die "Can't open file $filename.\n";
  while ( my $line_ascii = <INPUT_ASCII> )
  {  
    if( $line_ascii =~ "Version:" )
    {
      $versionFound = 2;
      $version = $line_ascii;
      chomp $version;
      $version =~ s/Version://;
    }
  }
  close( INPUT_ASCII );
  return ($versionFound, $version );

}


# read ascii print format raw file
($retVal, $version) = checkVersionInRawFile("$CIRFILEwithPrintRaw.ascii.raw" );
if( $retval != 0 )
{
  print "Error: file $CIRFILEwithPrintRaw.ascii.raw had a version string: $version\n";
  print "Exit code = $retval\n";
  exit $retval;
}

# read binary print format raw file
($retVal, $version) = checkVersionInRawFile("$CIRFILEwithPrintRaw.raw" );
if( $retval != 0 )
{
  print "Error: file $CIRFILEwithPrintRaw.raw had a version string: $version\n";
  print "Exit code = $retval\n";
  exit $retval;
}

# read ascii format raw file
($retVal, $version) = checkVersionInRawFile("$CIRFILE.ascii.raw" );
if( $retval != 0 )
{
  print "Error: file $CIRFILE.ascii.raw had a version string: $version\n";
  print "Exit code = $retval\n";
  exit $retval;
}

# read binary print format raw file
($retVal, $version) = checkVersionInRawFile("$CIRFILE.raw" );
if( $retval != 0 )
{
  print "Error: file $CIRFILE.raw had a version string: $version\n";
  print "Exit code = $retval\n";
  exit $retval;
}


# these files SHOULD have the Version tag in the raw header
# read ascii print format raw file
($retVal, $version) = checkVersionInRawFile("$CIRFILEwithPrintRawAndVersion.ascii.raw" );
if( $retval != 2 )
{
  print "Error: file $CIRFILEwithPrintRawAndVersion.ascii.raw did not have a version string: $version\n";
  print "Exit code = $retval\n";
  exit $retval;
}

# read binary print format raw file
($retVal, $version) = checkVersionInRawFile("$CIRFILEwithPrintRawAndVersion.raw" );
if( $retval != 2 )
{
  print "Error: file $CIRFILEwithPrintRawAndVersion.raw did not have a version string: $version\n";
  print "Exit code = $retval\n";
  exit $retval;
}

# read ascii format raw file
($retVal, $version) = checkVersionInRawFile("$CIRFILEwithVersion.ascii.raw" );
if( $retval != 2 )
{
  print "Error: file $CIRFILEwithVersion.ascii.raw did not have a version string: $version\n";
  print "Exit code = $retval\n";
  exit $retval;
}

# read binary print format raw file
($retVal, $version) = checkVersionInRawFile("$CIRFILEwithVersion.raw" );
if( $retval != 2 )
{
  print "Error: file $CIRFILEwithVersion.raw did not have a version string: $version\n";
  print "Exit code = $retval\n";
  exit $retval;
}





if ($retval != 0)
{
  print "Exit code = $retval\n";
  exit $retval;
}
print "Exit code = 0\n";
exit 0;


