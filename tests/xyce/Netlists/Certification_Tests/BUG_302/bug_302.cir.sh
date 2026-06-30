#!/usr/bin/env perl

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
#$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

$XYCE="$XYCE -error-test";

@CIRFILES = ("DC_defaults.cir",
             "DC_comma.cir",
             "DC_tab.cir",
             "DC_delimiter_invalid.cir",
             "transient_defaults.cir",
             "transient_comma.cir",
             "transient_tab.cir",
             "transient_delimiter_invalid.cir");

foreach $CIR (@CIRFILES)
{
  `rm -f $CIR.prn $CIR.out $CIR.err`;
  if ($CIR =~ "default")
  {
    $CMD="$XYCE $CIR > $CIR.out 2> $CIR.err";
    if (system("$CMD") != 0) { print "Exit code = 10\n"; exit 10; }
    $CIRDEFAULT=$CIR;
    # remove any \r from the line endings of the default file
    open (INFILE, "<$CIRDEFAULT.prn") or die "ERROR:  Cannot open $CIRDEFAULT.prn\n";
    open (OUTFILE, ">$CIRDEFAULT.bak") or die "ERROR:  Cannot open $CIRDEFAULT.bak\n";
    while(<INFILE>)
    {
      s/\r//g;           # line ending fix
      print OUTFILE;
    }
    close(INFILE);
    close(OUTFILE);
    rename("$CIRDEFAULT.bak", "$CIRDEFAULT.prn");
  }
  elsif ($CIR =~ "comma")
  {
    $CMD="$XYCE $CIR > $CIR.out 2> $CIR.err";
    if (system("$CMD") != 0) { print "Exit code = 10\n"; exit 10; }
    # remove any \r from the line endings of the just produced file
    open (INFILE, "<$CIR.prn") or die "ERROR:  Cannot open $CIR.prn\n";
    open (OUTFILE, ">$CIR.bak") or die "ERROR:  Cannot open $CIR.bak\n";
    while(<INFILE>)
    {
      s/\r//g;           # line ending fix
      print OUTFILE;
    }
    close(INFILE);
    close(OUTFILE);
    rename("$CIR.bak", "$CIR.prn");

    # Convert default output to comma delimited output
    open(PRN,"$CIRDEFAULT.prn") or die "ERROR:  Cannot open $CIRDEFAULT.prn\n";
    open(PRNGS,">$CIR.prn.gs") or die "ERROR:  Cannot open $CIR.prn.gs\n";
    while ($line=<PRN>)
    {
      $line =~ s/\s+$//;
      if ($line =~ "Xyce")
      {
        print PRNGS "$line\n";
        next;
      }
      $line =~ s/\s+/,/g;
      print PRNGS "$line\n";
    }
    close(PRN);
    close(PRNGS);
    $CMD="diff $CIR.prn.gs $CIR.prn > $CIR.prn.out 2> $CIR.prn.err";
    if (system("$CMD") != 0) { print STDERR "$CIR.prn.gs does not match $CIR.prn\n" ; $failure=1; }
  }
  elsif ($CIR =~ "tab")
  {
    $CMD="$XYCE $CIR > $CIR.out 2> $CIR.err";
    if (system("$CMD") != 0) { print "Exit code = 10\n"; exit 10; }
    # remove any \r from the line endings of the just produced file
    open (INFILE, "<$CIR.prn") or die "ERROR:  Cannot open $CIR.prn\n";
    open (OUTFILE, ">$CIR.bak") or die "ERROR:  Cannot open $CIR.bak\n";
    while(<INFILE>)
    {
      s/\r//g;           # line ending fix
      print OUTFILE;
    }
    close(INFILE);
    close(OUTFILE);
    rename("$CIR.bak", "$CIR.prn");

    # Convert default output to tab delimited output
    open(PRN,"$CIRDEFAULT.prn") or die "ERROR:  Cannot open $CIRDEFAULT.prn\n";
    open(PRNGS,">$CIR.prn.gs") or die "ERROR:  Cannot open $CIR.prn.gs\n";
    while ($line=<PRN>)
    {
      $line =~ s/\s+$//;
      if ($line =~ "Xyce")
      {
        print PRNGS "$line\n";
        next;
      }
      $line =~ s/\s+/\t/g;
      print PRNGS "$line\n";
    }
    close(PRN);
    close(PRNGS);
    $CMD="diff $CIR.prn.gs $CIR.prn > $CIR.prn.out 2> $CIR.prn.err";
    if (system("$CMD") != 0) { print STDERR "$CIR.prn.gs does not match $CIR.prn\n" ; $failure=1; }
  }
  elsif ($CIR =~ "invalid")
  {
    $CMD="$XYCE $CIR > $CIR.out 2> $CIR.err";
    if (system("$CMD") != 0) { print "Exit code = 10\n"; exit 10; }
    # remove any \r from the line endings of the just produced file
    open (INFILE, "<$CIR.prn") or die "ERROR:  Cannot open $CIR.prn\n";
    open (OUTFILE, ">$CIR.bak") or die "ERROR:  Cannot open $CIR.bak\n";
    while(<INFILE>)
    {
      s/\r//g;           # line ending fix
      print OUTFILE;
    }
    close(INFILE);
    close(OUTFILE);
    rename("$CIR.bak", "$CIR.prn");

    # Now we check for the warning
    $searchstring = "Invalid value of DELIMITER in .PRINT statment, ignoring";
    open(OUT,"$CIR.out") or die "ERROR:  Cannot open $CIR.out\n";
    $subfailure = 1;
    while ($line = <OUT>)
    {
      if ($line =~ $searchstring)
      {
        undef $subfailure;
        #print "Found warning\n";
        break;
      }
    }
    if ($subfailure) {
        print STDERR "Message '$searchstring' not found in netlist $CIR.out\n" ; 
        $failure=1; 
    }
    close(OUT);
  }
}

if ($failure) { print "Exit code = 2\n"; exit 2; } else { print "Exit code = 0\n"; exit 0; }

print "Exit code = 1\n"; exit 1;


