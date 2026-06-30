#!/usr/bin/env perl

use List::Util qw(min max);

use XyceRegression::Tools;
$Tools = XyceRegression::Tools->new();

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
#$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

$verbose = 1;
$tempfile = $Tools->modifyTest($CIRFILE,('timestepsreversal=1'));
print "TESTING timestepreversal=1 with temp cirfile = $tempfile\n";
$failed1 = runTest($XYCE,$tempfile);

$tempfile = $Tools->modifyTest($CIRFILE,('timestepsreversal=0'));
print "TESTING timestepreversal=0 with temp cirfile = $tempfile\n";
$failed2 = runTest($XYCE,$tempfile);

$tempfile = $Tools->modifyTest($CIRFILE,('nlmin=3','nlmax=2'));
print "TESTING nlmin > nlmax with temp cirfile = $tempfile\n";
@searchstrings = ( "User Fatal:  [\*]+ ERROR: .options timeint NLMIN = 3 > 2 = NLMAX!");
$retval = $Tools->runAndCheckError($tempfile,$XYCE,@searchstrings);
if ($retval != 0) { $failed3 = 1; }

if (defined($failed1) or defined($failed2) or defined($failed3)) {
  print "Exit code = 2\n";
  exit 2;
} else {
  print "Exit code = 0\n";
  exit 0;
}

sub runTest {
  my ($XYCE,$CIRFILE) = @_;
  my ($CMD, $dtgrow, $dtcut, $nlmin, $nlmax);
  my ($delmax, $doRejectStep, $nlits, $newdt);
  my ($olddt, $dt, $settings, $goodsteps);
  my ($beginningIntegration, $eps, $rejectingStep);
  my ($failed,$line);

  $CMD="$XYCE $CIRFILE > $CIRFILE.out 2> $CIRFILE.err";
  if (system("$CMD") != 0) { print "Exit code = 10\n"; exit 10 }

  $dtgrow=2;
  $dtcut=0.125;
  $nlmin=3;
  $nlmax=10;
  $delmax=1.2e-3;
  $doRejectStep = 0;

  $nlits = -1;
  $newdt = 0.0;
  $olddt = 0.0;
  $dt = 0.0;
  $settings = 0;
  $goodsteps = 0;
  $beginningIntegration = 1;
  $eps = 1.0e-6; # Fudge factor in equality comparisons
  $rejectingStep = 0;
  undef $failed;

  open(CIROUT,"$CIRFILE.out");
  while ($line = <CIROUT>)
  {
    chomp($line);
    if ($line =~ s/ERROROPTION=[01]:\s+DeltaT Grow =\s+([0-9e+-\.]+)/\1/) {
      $dtgrow = $line;
      $settings++;
    }
    elsif ($line =~ s/ERROROPTION=[01]:\s+DeltaT Cut =\s+([0-9e+-\.]+)/\1/) {
      $dtcut = $line;
      $settings++;
    } 
    elsif ($line =~ s/ERROROPTION=[01]:\s+NL MIN =\s+([0-9]+)/\1/) {
      $nlmin = $line;
      $settings++;
    }
    elsif ($line =~ s/ERROROPTION=[01]:\s+NL MAX =\s+([0-9]+)/\1/) {
      $nlmax = $line;
      $settings++;
      if ($nlmax < $nlmin) {
        print "Exit code = 2";
        exit 2;
      }
    }
    elsif ($line =~ s/ERROROPTION=[01]:\s+DELMAX =\s+([0-9e+-\.]+)/\1/) {
      $delmax = $line;
      $settings++;
    }
    elsif ($line =~ s/ERROROPTION=[01]:\s+DOREJECTSTEP =\s+([01])/\1/) {
      $doRejectStep = $line;
      $settings++;
    }
    elsif ($line =~ s/ERROROPTION=[01]:\s+TimeStepLimitedbyBP =\s+([01])/\1/) {
      if (($bpLimited == 1) and ($line == 0)) {
        $beginningIntegration = 2;
      } else {
        $beginningIntegration = max(0,$beginningIntegration-1);
      }
      $bpLimited = $line;
    }
    elsif ($line =~ m/Transient Analysis:  rejecting time step/) {
      $rejectingStep = 1;
    }
    elsif ($line =~ m/Transient Analysis:  accepting time step/) {
      $rejectingStep = 0;
    }
    elsif ($line =~ s/ERROROPTION=[01]:\s+NL Its =\s+([0-9]+)/\1/) {
      $nlits = $line;
    }
    elsif ($line =~ s/ERROROPTION=[01]:\s+New DeltaT =\s+([0-9e+-\.]+)/\1/) {
#   We can't determine what the step-size should be in these cases:
      $olddt = $newdt;
      $newdt = $line;
      if ($bpLimited == 1) { 
        next; 
      } 
      if ($beginningIntegration > 0) { 
        next; 
      }  
      if ($olddt == 0) { 
        next; 
      } 
      $dt = $olddt;
      if ($nlits <= $nlmin) {
        $dt = $dtgrow*$olddt;
      } 
      elsif ($nlits > $nlmax) {
        $dt = $dtcut*$olddt;
        if (($doRejectStep == 1) and ($rejectingStep == 0)) {
          $failed = 1;
        } elsif (($doRejectStep == 1) and ($rejectingStep == 1)) {
        } elsif (($doRejectStep == 0) and ($rejectingStep == 1)) {
        } elsif (($doRejectStep == 0) and ($rejectingStep == 0)) {
        }
      }
      if ($dt > $delmax) {
      }
      $dt = min($dt,$delmax);
      if (fuzzyEquality($newdt,$dt,$eps)) {
        $goodsteps++;
      } else {
        $failed = 1;
      }
    }
  }
  close(CIROUT);

  if ($goodsteps == 0) {
    $failed = 1;
  }
  if ($settings < 6) {
    $failed = 1;
  }
  return $failed;
}


sub fuzzyEquality { 
  my ($left,$right,$eps) = @_;
  my $success=undef;
  if ((abs($left-$right)/max(abs($left),abs($right))) < $eps) {
    $success = 1;
  }
  return $success;
}


