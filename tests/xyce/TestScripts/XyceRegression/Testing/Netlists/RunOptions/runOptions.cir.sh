#!/usr/bin/env perl

use strict;
use XyceRegression::Tools;
my $Tools = XyceRegression::Tools->new();

my $XYCE=$ARGV[0];
my $XYCE_VERIFY=$ARGV[1];
my $XYCE_COMPARE=$ARGV[2];
my $CKT=$ARGV[3];
my $GOLD_PRN=$ARGV[4];

my $reg = '[\.]+';
my ($file,@filelist);
my ($message,@stringList);

# slowXyce has a 2.0 second sleep
my $slowXyce = "slowXyce";
createFile("$slowXyce","#!/usr/bin/env perl\nif (\$ARGV[0] =~ m/[-]v/) { exit 0 }\nselect(undef,undef,undef,2.0);\nprint \"$0 @ARGV\n\";\nexit 0\n");
`chmod +x $slowXyce`;
# fastXyce has a no sleep
my $fastXyce = "fastXyce";
createFile("$fastXyce","#!/usr/bin/env perl\nif (\$ARGV[0] =~ m/[-]v/) { exit 0 }\nprint \"$0 @ARGV\n\";\nexit 0\n");
`chmod +x $fastXyce`;

createFile("options","timelimit=1,progresslimit=1\n");

# Set up test scripts... --------------------------------------------------
$file = "default_finish";
push(@filelist,"$file.cir");
createFile("$file.cir","");
createFile("$file.cir.sh","#!/bin/sh\n./$fastXyce\necho \"Exit Code = 0\"\nexit 0\n");
$message = "passed";
push(@stringList,"RunOptions/$file$reg$message\\\[sh\\\]");

$file = "default_stall";
push(@filelist,"$file.cir");
createFile("$file.cir","");
createFile("$file.cir.sh","#!/bin/sh\n./$slowXyce\necho \"Exit Code = 0\"\nexit 0\n");
$message = "TIME LIMIT";
push(@stringList,"RunOptions/$file$reg$message\\\[sh\\\]");

$file = "option_finish";
push(@filelist,"$file.cir");
createFile("$file.cir","");
createFile("$file.cir.sh","#!/bin/sh\n./$slowXyce\necho \"Exit Code = 0\"\nexit 0\n");
createFile("$file.cir.options","timelimit=5,progresslimit=5\n");
$message = "passed";
push(@stringList,"RunOptions/$file$reg$message\\\[sh\\\]");

$file = "option_stall";
push(@filelist,"$file.cir");
createFile("$file.cir","");
createFile("$file.cir.sh","#!/bin/sh\n./$slowXyce\necho \"Exit Code = 0\"\nexit 0\n");
createFile("$file.cir.options","timelimit=1,progresslimit=1\n");
$message = "TIME LIMIT";
push(@stringList,"RunOptions/$file$reg$message\\\[sh\\\]");

my $testlist="testlist";
`rm -f $testlist`;
open(TESTLIST,">$testlist");
foreach my $tmpfile (@filelist) {
  print TESTLIST "RunOptions $tmpfile\n";
}
close(TESTLIST);

my $XYCE_TEST=$XYCE_VERIFY;
$XYCE_TEST =~ s/TestScripts\/xyce_verify.pl//;

my $outfile = "testresults";

my $output = `pwd`; chomp($output);
$output =~ s-Netlists/RunOptions--;

# Run the tests to produce the output
system("$XYCE_TEST/TestScripts/run_xyce_regression --output=$output --ignoreparsewarnings --xyce_test=$XYCE_TEST --xyce_verify=$XYCE_VERIFY --xyce_compare=$XYCE_COMPARE --resultfile=$outfile --skipmake --testlist=$testlist \"$XYCE\"");

# Verify that the output matches our expectations
my $exitcode = $Tools->checkError("$outfile",@stringList);

# Clean up extra .cir files:
`rm -f options`;
if ($exitcode == 0) {
  `rm -f $testlist $outfile $slowXyce $fastXyce`;
  foreach my $foo (@filelist) {
    `rm -f $foo $foo.sh $foo.stdout $foo.stderr $foo.options`;
  }
}

# Exit
print "Exit code = $exitcode\n";
exit $exitcode;

sub createFile {
  my ($file,$str) = @_;
  open(TMPFILE,">$file");
  print TMPFILE "$str";
  close(TMPFILE);
}
