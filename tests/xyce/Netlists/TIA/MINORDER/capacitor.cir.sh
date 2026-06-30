#!/usr/bin/env perl

use List::Util qw(min max);

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

$failed1 = runTest($XYCE,$CIRFILE);
$failed2 = runTest($XYCE,"invert.cir");
if (defined($failed1) or defined($failed2)) {
  print "Exit code = 2\n";
  exit 2;
} else {
  print "Exit code = 0\n";
  exit 0;
}


sub runTest {
  my ($XYCE,$CIRFILE) = @_;
  my ($CMD,$minord,$maxord,$failed,$line,@opts);
  my ($op,$foundOrder, $phase, $currOrder, $iter);
  my (@orders,$NEWiter,$NEWusedOrder,$NEWnextOrder);

  $CMD="$XYCE $CIRFILE > $CIRFILE.out 2> $CIRFILE.err";
  if (system("$CMD") != 0) { print "Exit code = 10\n"; exit 10 }

  $minord = 1;
  $maxord = 5;

  undef $failed;
  open(CIRFILE,"$CIRFILE");
  while ($line = <CIRFILE>) {
    chomp($line);
    if ($line =~ m/^\s*\*/) { next; }
    if ($line =~ m/^\s*\.options\s+timeint/i) {
      while ($line =~ s/\+(\s*)$/\1/) {
        $newline = <CIRFILE>;
        $line = "$line $newline";
      }
      $line =~ s/\s*=\s*/=/g;
      @opts = split(/\s/,$line);
      foreach $op (@opts) {
        if ($op =~ s/maxord=([0-9])/\1/i) {
          $maxord = $op;
        } elsif ($op =~ s/minord=([0-9])/\1/i) {
          $minord = $op;
        }
      }
    }
  }
  close(CIRFILE);
  $minord = max(1,$minord);
  $maxord = min(5,$maxord);
  if ($minord > $maxord) { $minord = $maxord; }
  print "\$minord = $minord\n";
  print "\$maxord = $maxord\n";

  $foundOrder = 0;
  $phase = 0; # Initial phase
  $currOrder = 1;
  $iter = 0;
  $failed = undef;

  open(CIROUT,"$CIRFILE.out");
  while ($line = <CIROUT>) {
    chomp($line);
    if ($line =~ s/\(([0-9]+)\)\s+Used,\s+Next Order:\s+([0-9]),\s+([0-9])/\1 \2 \3/) {
      $foundOrder++;
      @orders = split(/\s/,$line);
      $NEWiter = $orders[0];
      $NEWusedOrder = $orders[1];
      $NEWnextOrder = $orders[2];
      if ($newiter == $iter+1) {
        $iter++;
        if ($NEWusedOrder != $currOrder) {
          print "FAILED:  Iteration count increased, but previous current order ($currOrder) != usedOrder ($NEWusedOrder)!\n";
          $failed =1;
          break;
        } else {
          $currOrder = $NEWusedOrder;
        }
        if ($NEWusedOrder >= $minord) { $phase = 1; }
        if ($phase == 0) {
          if ($NEWnextOrder != $currOrder+1) {
            print "FAILED:  In initial ramp-up phase and next order = $NEWnextOrder did not increase from current order = $currOrder\n";
            $failed = 1;
            break;
          }
        } else {
          if (($NEWnextOrder < $minord) or ($NEWnextOrder > $maxord)) {
            print "FAILED:  next order = $NEWnextOrder is not in [$minord,$maxord]!\n";
            $failed = 1;
            break;
          }
        }
      }
    }
  }
  close(CIROUT);

  if ($foundOrder == 0) {
    print "Found no order information!\n";
    $failed = 1;
  } else {
    print "Found $foundOrder lines containing order information\n";
  }
  return $failed;
}

