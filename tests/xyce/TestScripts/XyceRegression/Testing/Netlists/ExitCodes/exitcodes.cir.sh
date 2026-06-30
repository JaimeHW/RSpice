#!/usr/bin/env perl

@exitlist = (0,1,2,10,11,12,13,14,15,16,255);

$testlist = "exitcodes_list";
$outfile = "exitcodes.out";

use XyceRegression::Tools;
$Tools = XyceRegression::Tools->new();
@stringlist = ();
@filelist = ();

# Create the circuit files & testlist & verify strings
open(TESTLIST,">$testlist");
foreach $exitnumber (@exitlist) {
  my $file = "exit_$exitnumber";
  open(SHELLFILE,">$file.cir.sh");
  printf SHELLFILE "#!/bin/sh\n";
  printf SHELLFILE "echo \"Exit code = $exitnumber\"\n";
  printf SHELLFILE "exit $exitnumber\n";
  close(SHELLFILE);
  system("touch $file.cir");
  printf TESTLIST "ExitCodes $file.cir\n";
# Create verify strings
  my $message = $Tools->lookupStatus($exitnumber);
  my $reg = '[\.]+';
  push(@stringlist,"ExitCodes/$file$reg${message}\\\[sh\\\]");
# Create file list
  push(@filelist,"$file.cir");
}
close(TESTLIST);

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$XYCE_COMPARE=$ARGV[2];
$CKT=$ARGV[3];
$GOLD_PRN=$ARGV[4];

$XYCE_TEST=$XYCE_VERIFY;
$XYCE_TEST =~ s/TestScripts\/xyce_verify.pl//;

# Set up output to go to current directory.
my $output = `pwd`; chomp($output);
$output =~ s-Netlists/ExitCodes--;

# Run the tests to produce the output
system("$XYCE_TEST/TestScripts/run_xyce_regression --output=$output --ignoreparsewarnings --xyce_test=$XYCE_TEST --xyce_verify=$XYCE_VERIFY --xyce_compare=$XYCE_COMPARE --resultfile=$outfile --skipmake --testlist=$testlist \"$XYCE\"");

# Verify that the output matches our expectations
$exitcode = $Tools->checkError("$outfile",@stringlist);

# Clean up extra .cir files:
if ($exitcode == 0) {
  foreach $file (@filelist) {
    `rm -f $file $file.sh $file.stdout $file.stderr`;
  }
}

# Exit
print "Exit code = $exitcode\n";
exit $exitcode;
