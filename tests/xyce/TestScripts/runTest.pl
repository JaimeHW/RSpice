#!/usr/bin/env perl
########################### runTest.pl ###########################
#
#   NAME:      runTest.pl <Xyce_Path> <Xyce_Test_Home> <directory> <test_file>
#   Purpose:   This script is used to run a single test
#              in the directory <directory> called <circuit file>
#              XYCE_TEST_HOME is passed it to locate gold standard
#              data (if any is needed for the test.
#
#              optional flags
#   
##################################################################
#
# Revision Information
# --------------------
# Date:  $Date$
# Last revision by: $Author$
# Revision: $Revision$
##################################################################
##################################################################


package XyceRegression;
use strict; 

#use Env;          # import all variables
use File::Basename;
use File::Copy 'cp';
use Getopt::Long;
use FindBin;
use lib "$FindBin::Bin";
use XyceRegression::Tools;


my $version = "0.1";

my @time0=times;
my $wtime0 = time;
# modify these variables
my $RunXYCE = "true";
my $runCompare="true";
my @copyARGV = @ARGV;

my $XYCE_PATH;        
my $XYCE_TEST_HOME;
my $outputDirectory;
my $testDirectory;
my $testFile;

my ($verbose, $printVersion, $debug, $quiet, $helpflag, $extraHelpFlag, $textwidth);
my ($testtime, $testprogress, $xyce_verify, $fork, $wrapshellscripts, $ignoreshellscripts);
my ($indent, $passFileName, $failFileName, $resultFileName, $notest, $fulldiff, $fulldiffregex, $selftest, $saveoutput);

#default values
$indent="";
$textwidth=65;

&GetOptions( "xyce_path=s" => \$XYCE_PATH,
             "xyce_test_home=s" => \$XYCE_TEST_HOME,
             "outputdir=s" => \$outputDirectory,
             "testdir=s" => \$testDirectory,
             "testfile=s" => \$testFile,
             "verbose!" => \$verbose, 
             "version!" => \$printVersion, 
             "debug!" => \$debug, 
             "quiet!" => \$quiet,
             "textwidth=s" => \$textwidth,
             "indent=s" => \$indent,
             "help" => \$helpflag, 
             "extrahelp" => \$extraHelpFlag,
             "timelimit=s" => \$testtime,
             "progresslimit=s" => \$testprogress,
             "xyce_verify=s" => \$xyce_verify,
             "passfile=s" => \$passFileName,
             "failfile=s" => \$failFileName,
             "resultfile=s" => \$resultFileName,
             "indent=s" => \$indent,
             "notest!" => \$notest,
             "fulldiff=s" => \$fulldiff,
             "fulldiffregex=s" => \$fulldiffregex,
             "saveoutput!" => \$saveoutput,
             "fork!" => \$fork,
             "forkshellscripts!" => \$wrapshellscripts,
             "ignoreshellscripts!" => \$ignoreshellscripts,
             "selftest!" => \$selftest );

if ((defined $helpflag) or (defined $extraHelpFlag)) {
  print "Usage:\n";
  print "runTest.pl [options] XYCE_TEST_HOME TestDirectory TestCircuitFile \n";
  print "--Xyce_Path=\"path to Xyce\" # Path to Xyce executable\n";
  print "--Xyce_Test_Home=\"dir\" # Directory for Xyce_Regression\n";
  print "--testDir=\"dir\";       # Directory where test will be run\n"; 
  print "--testFile=\"filename\"  # File that starts the test\n";
  print "--passfilee=\"filename\" # File to record test passes\n";
  print "--failfile=\"filename\"  # File to record test failures\n";
  print "--resultfile=\"filename\"# File to record test results\n";
  print "--version                # Print version information\n";
  print "--verbose                # Enable verbose output\n";
  print "--extrahelp              # Full list of command-line options\n";
  print "XYCE_TEST_HOME           # absolute path to Xyce_Regression\n";
  print "TestDirectory            # absolute path to where the test will be run\n";
  print "TestFile                 # file that starts the test\n";
}

if (defined $extraHelpFlag) {
  print "\n";
  print "Extended Help:\n";
  print "--Xyce_Path=\"path to Xyce\" # Path to Xyce executable\n";
  print "--Xyce_Test_Home=\"dir\" # Directory for Xyce_Regression\n";
  print "--testDir=\"dir\";       # Directory where test will be run\n"; 
  print "--testFile=\"filename\"  # File that starts the test\n";
  print "--passfilee=\"filename\" # File to record test passes\n";
  print "--failfile=\"filename\"  # File to record test failures\n";
  print "--resultfile=\"filename\"# File to record test results\n";
  print "--version                # Print version information\n";
  print "--verbose                # Enable verbose output\n";
  print "--extrahelp              # Full list of command-line options\n";
  print "XYCE_TEST_HOME           # absolute path to Xyce_Regression\n";
  print "TestDirectory            # absolute path to where the test will be run\n";
  print "TestFile                 # file that starts the test\n";
  print "----------------------------------------------------------------------\n";
  print "  Options that control how Xyce is run:\n";
  print "----------------------------------------------------------------------\n";
  print "--timelimit=7200      # Time limit for each test to complete (sec)\n";
  print "--progresslimit=1800  # Time limit for non-shell script .prn updates\n";
  print "                        Note:  timelimit/progresslimit require fork*\n";
  print "--fork                # [default] Use fork when running Xyce\n";
  print "--forkshellscripts    # [default] Use fork when running shell scripts\n";
  print "--resultfile=filename # Also print output to this file\n";
  print "----------------------------------------------------------------------\n";
  print "  Report formatting options:\n";
  print "----------------------------------------------------------------------\n";
  print "--textwidth=50        # Width of text for test name and directory\n";
  print "--indent=string       # Indent string to put before output lines\n";
  print "----------------------------------------------------------------------\n";
  print " Locations of prerequisites:\n";
  print "----------------------------------------------------------------------\n";
  print "--xyce_test=directory # Location of Xyce_Regression to use for testing\n";
  print "--xyce_verify=file    # Location of xyce_verify.pl\n";
  print "--skipmake            # Skip the make process for compare\n";
  print "----------------------------------------------------------------------\n";
  print "  Refactoring support options:\n";
  print "----------------------------------------------------------------------\n";
  print "--saveoutput          # Save all output from runs, (use with fulldiff)\n";
  print "--fulldiff            # Diff output with previous --saveoutput run\n";
  print "--fulldiffregex=file  # File with regex to remove extraneous data from\n";
  print "                        output files (e.g. dates, timings, etc)\n";
  print "----------------------------------------------------------------------\n";
  print "  Screen/debug options:\n";
  print "----------------------------------------------------------------------\n";
  print "--quiet               # Disable most normal output\n";
  print "--debug               # Enable debug output\n";
  print "--ignoreshellscripts  # Ignore shell scripts when running tests\n";
  print "----------------------------------------------------------------------\n";
  print "  Self Testing:\n";
  print "----------------------------------------------------------------------\n";
  print "--selftest            # Run a set of internal self-tests\n";
  print "----------------------------------------------------------------------\n";
}
if ((defined $helpflag) or (defined $extraHelpFlag)) {
  exit 0;
}



# Set up common regression tools flags:
my $Tools = XyceRegression::Tools->new();
if (defined ($resultFileName)) { $Tools->setResultFile($resultFileName); }
if (defined ($quiet))      { $Tools->setQuiet(1); }
if (defined ($verbose))    { $Tools->setVerbose(1); }
if (defined ($debug))      { $Tools->setDebug(1); }
if (defined ($fork))       { $Tools->setFork(1); }
if (defined ($testtime))   { $Tools->setChildWait($testtime); }
if (defined ($testprogress)) { $Tools->setChildProgress($testprogress); }

$Tools->setIndent($indent);
$Tools->setFork(1);

#foreach my $arg (@copyARGV) {
#  if( $arg =~/--*/ ) {
#    shift @copyARGV;
#  }
#}

if (defined $printVersion) {
  $Tools->iPrint("run_xyce_regression version $version\n");
  exit 0
}

#if ( (not defined $XYCE_PATH) and (scalar @copyARGV > 0) ) {
#  $XYCE_PATH = shift @copyARGV ;
#}
if (not defined $XYCE_PATH) { 
  $Tools->iPrint("Must specify XYCE_Path either with --Xyce_Path= or as an argument\n");
  exit -1;
}

#if ( (not defined $XYCE_TEST_HOME) and (scalar @copyARGV > 0) ) {
#  $XYCE_TEST_HOME= shift @copyARGV;
#} 

# I don't really need to do this as having $XYCE_TEST_HOME set is a 
# requirement for this script. However other parts of Tools may use it
$Tools->verbosePrint("Setting PERL5LIB variable to $FindBin::Bin\n");
$ENV{'PERL5LIB'} = "$FindBin::Bin";
$XYCE_TEST_HOME = $Tools->configureXyceTestHome($XYCE_TEST_HOME,\%ENV);
if (not defined $XYCE_TEST_HOME) { 
  $Tools->iPrint("Must specify XYCE_TEST_HOME either with --Xyce_Test_Home= or as an argument\n");
  exit -1;
}

#if ( (not defined $testDirectory ) and (scalar @copyARGV > 0) ) {
#  $testDirectory= shift @copyARGV;
#}
if (not defined $testDirectory) { 
  $Tools->iPrint("Must specify testDirectory either with --testDir= or as an argument\n");
  exit -1;
}

#if ( (not defined $testFile ) and (scalar @copyARGV > 0) ) {
#  $testFile= shift @copyARGV;
#}
if (not defined $testDirectory) { 
  $Tools->iPrint("Must specify testFile either with --testFile= or as an argument\n");
  exit -1;
}

if( not defined $xyce_verify ) {
  my ($volume, $directories, $file );
  my $nofile=1; # indicates that there is not a filename at the end of this path.
  ($volume, $directories, $file ) = File::Spec->splitpath( $XYCE_TEST_HOME, $nofile );
  my @dirs =($directories,"TestScripts");
  $xyce_verify = File::Spec->catpath( $volume, @dirs, "xyce_verify.pl" );
}

# all set up now run the test
my $exitCode = runTest( $XYCE_PATH, $XYCE_TEST_HOME, $outputDirectory, $testDirectory, $testFile, $Tools, $passFileName, $failFileName, $resultFileName, $xyce_verify, $textwidth, $saveoutput );

exit $exitCode;


# main used to call sub-routine runTest
#

# Run test script for each circuit:
sub runTest {
  my ($XYCE_PATH, $XYCE_TEST_HOME, $outputDir, $testDir, $ckt, $ToolsObj, $passFileName, $failFileName, $resultFileName, $XYCE_VERIFY, $textwidth, $saveoutput)=@_;

  # $dir is a full path to the test directory.  This needs to be separated into base_dir/testdir
  # There can be more than one directory on base_dir as in Netlits/GROUP1/group2/test.cir 
  # need baseOutputDir to be GROUP1/group2 
  # my ($volume, $baseOutputDir, $testDir ) = File::Spec->splitpath( $dir );
  # print( "V=$volume B=\"$baseOutputDir\" T=\"$testDir\" C=\"$ckt\"" );

  my (@otimes,@ntimes,$walltime0,$walltime1,$vtime0,$vtime1);
    
  my $printCirName = $ToolsObj->figureCirName($testDir,$ckt,$textwidth);
  # this is set by the calling function.  Need a better way to get it down to here.
  my $wrapshellscripts=1;

  # old globals that don't appy anymore when running one test in isolation
  my $num_code_failure;
  my $numberfailed;

  $ToolsObj->userControlExit();
  $vtime0 = 0.0;
  $vtime1 = 0.0;
  $ToolsObj->iPrint("$printCirName");
  $ToolsObj->debugPrint("runTest:  \$ckt = >>$ckt<<\n");
  $ToolsObj->debugPrint("runTest:  \$XYCE_PATH = >>$XYCE_PATH<<\n");
  #print("runTest:  \$ckt = >>$ckt<<\n");
  #print("runTest:  \$XYCE_PATH = >>$XYCE_PATH<<\n");


  my ($pref,$dir1,$ext) = fileparse("$testDir/$ckt",'\..*') ;
  if(defined($RunXYCE)) {
      `rm -f $pref\.prn`;  # remove output files
  }
  
  my $failure=0;
  my $status=-1;
  my $timespace = "";
  my $GOLDPRNFILE = "$XYCE_TEST_HOME/OutputData/$testDir/$ckt.prn";
  # not right.  Need to refactor this routine to assume it starts in "$dir"
  my $output = $outputDir;
  # need full path here to ensure we check for shell file
  my $SHELLFILE = "$outputDir/$testDir/$ckt.sh";
  if ( -f $SHELLFILE and not defined $ignoreshellscripts ) {
  
    #
    # inspect this carefully.  Something in this scope isn't thread safe
    #
    
    $timespace = "[sh]";
    #system("rm -f $ckt.prn $ckt.prn.out $ckt.prn.err");
    $walltime0 = time;
    @otimes=times;
    my $retval = -1;
    if (defined $wrapshellscripts) {
      my $taglist="taglist";
      my $XYCE_COMPARE="xycecompare";
      $retval = $ToolsObj->wrapShellScript($SHELLFILE,$XYCE_PATH,$XYCE_VERIFY,$XYCE_COMPARE,$ckt,$GOLDPRNFILE,$taglist);
    } else {
      my $taglist="taglist";
      my $XYCE_COMPARE="xycecompare";
      my $CMD = "$SHELLFILE \'$XYCE_PATH\' $XYCE_VERIFY $XYCE_COMPARE $ckt $GOLDPRNFILE $taglist &> $ckt.stdouterr";
      #print("shell command: $CMD");
      $retval = system("$CMD");
    }
    $walltime1 = time;
    @ntimes=times;
    if( $retval == 0 ) {
        $status = 0;
        if (not defined($saveoutput)) {
          if ( -z "$ckt.err" ) { `rm -f $ckt.err`; }
          if ( -z "$ckt.prn.err" ) { `rm -f $ckt.prn.err`; }
          if ( -z "$ckt.prn.out" ) { `rm -f $ckt.prn.out`; }
        }
    } elsif (not defined($fulldiff)) {
        $status = $retval;
        if (($status == 1) or ($status == 10)) {
          $num_code_failure++;
        } else {
          $numberfailed++;
        }
        # 03/15/06 tscoffe:  Now I want to check for test_results in the
        # directory, which indicates run_xyce_regression was run recursively
        # for this test and I would like to include this file in the output.
        # Specifically this is to support the VALGRIND test.
        if ( -f "test_results" ) {
          open(testresults,"test_results");
          my @testresultlist = <testresults>;
          close(testresults);
          my $localindent = "    ";
          foreach my $testres (@testresultlist) {
            print results "$localindent$testres";
            $ToolsObj->resultPrint( "$localindent$testres"); 
          }
        }
    }
  } else {
    if ( defined ($RunXYCE) ) {
      $walltime0 = time;
      @otimes=times;
      #print ( "about to call wrapXyce $XYCE_PATH $ckt ");
      my $retval = $ToolsObj->wrapXyce($XYCE_PATH,$ckt);
      $walltime1 = time;
      @ntimes=times;
      $ToolsObj->debugPrint("wrapXyce retval = $retval\n");
      if($retval != 0) { 
        $failure=1;
        if (not defined($fulldiff)) {
          $num_code_failure++;
        }
        $status = $retval;
      }
    }
    # compare output
    if ( defined ($runCompare) && $failure != 1 ) {
      my $GOLDPRN = "$XYCE_TEST_HOME/OutputData/$testDir/$ckt.prn";
      if ( -x "$ckt.prn.gs.pl" ) {
        $ToolsObj->debugPrint("Attempting to use $ckt.prn.gs.pl\n");
        `rm -f $ckt.prn.gs $ckt.prn.gs.pl.out $ckt.prn.gs.pl.err`;
        my $currdir = `pwd`;
        # $ToolsObj->debugPrint("current directory = $currdir\n");
        my $CMD="./$ckt.prn.gs.pl $ckt.prn > $ckt.prn.gs.pl.out 2> $ckt.prn.gs.pl.err";
        # $ToolsObj->debugPrint "Running >>$CMD<<\n");
        my $exitstatus = system("$CMD");
        # $ToolsObj->debugPrint("Command returned exit status = $exitstatus\n");
        if ( -f "$ckt.prn.gs" ) {
          $ToolsObj->debugPrint("Successfully created $ckt.prn.gs\n");
          $GOLDPRN = "$ckt.prn.gs";
        } else {
          $ToolsObj->debugPrint("Failed to create $ckt.prn.gs\n");
        }
      } 
      `rm -f $ckt.prn.out $ckt.prn.err`;
      if( -e $GOLDPRN )
      {
        my $CMD = "$XYCE_VERIFY $ckt $GOLDPRN $ckt.prn > $ckt.prn.out 2> $ckt.prn.err";
        #`echo $CMD > $ckt.prn.com`;
        $vtime0 = time;
        my $retval = system("$CMD");
        $vtime1 = time;
        if ($retval == 0) {
          $status = 0;
        } elsif (not defined($fulldiff)) {
          $numberfailed++;
          $status = 2;
        }
        if (not defined($saveoutput)) {
          if ( -z "$ckt.err" ) { `rm -f $ckt.err`; }
          if ( -z "$ckt.prn.err" ) { `rm -f $ckt.prn.err`; }
          if ( -z "$ckt.prn.out" ) { `rm -f $ckt.prn.out`; }
          if ( -z "$ckt.prn.gs.pl.err" ) { `rm -f $ckt.prn.gs.pl.err`; }
          if ( -z "$ckt.prn.gs.pl.out" ) { `rm -f $ckt.prn.gs.pl.out`; }
        }
      } elsif( $status == -1 ) {
        # status is still the default so the run phase went fine and there was not a verify stage needed.
        # return passed.
        $status = 0
      }
    }
  }
  if (defined($fulldiff)) {
    $ToolsObj->debugPrint("Calling fullDiff for directory $testDir\n");
    $status = $ToolsObj->fullDiff("$fulldiff/Netlists/$testDir","$output/Netlists/$testDir",$fulldiffregex);
    $ToolsObj->debugPrint("fullDiff returned status = $status\n");
    if ($status != 0) {
      $numberfailed++;
    }
  }
  if ($ToolsObj->checkControlExit()) {
    $status = 16;
    print "\n";
    $ToolsObj->userControlExit();
    $ToolsObj->iPrint("$printCirName");
  }
  my $message = $ToolsObj->lookupStatus($status);
  my $usertime=$ntimes[2]-$otimes[2];
  my $systemtime=$ntimes[3]-$otimes[3];
  my $walltime = $walltime1 - $walltime0;
  my $verifytime = $vtime1 - $vtime0;
  #  my $timestr = sprintf("(Time:w=%3ds, u=%6.2fs, s=%6.2fs, v=%3ds)",$walltime,$usertime,$systemtime,$verifytime);
  my $timestr = sprintf("(Time: %3ds = %6.2fcs + %3dvs)",$walltime+$verifytime,$usertime+$systemtime,$verifytime);
  my $timespacenum = 16 - length($timespace) - length($message);
  if ($timespacenum < 1 ) { $timespacenum = 1; }
  $timespace = $timespace . " " x $timespacenum;
  $ToolsObj->defaultPrint("$message$timespace$timestr\n");
  if ($status != 0) {
    if( defined $resultFileName ) {
      open( my $fileHandle, '>', $resultFileName ) or die "Can't open $resultFileName";
      print $fileHandle "$printCirName$message$timespace$timestr\n";
      close $fileHandle;
    }
    if( defined $failFileName ) {
      open( my $failFileHandle, '>', $failFileName) or die "Can't open $failFileName";
      print $failFileHandle "$testDir $ckt\n";
      close $failFileHandle ;
    }
  } else {
    if( defined $passFileName ) {
      open( my $passFileHandle, '>', $passFileName) or die "Can't open $passFileName";
      print $passFileHandle "$testDir $ckt\n";
      close $passFileHandle;
    }
  }
}

