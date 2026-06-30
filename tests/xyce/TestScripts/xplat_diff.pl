#!/usr/bin/env perl

# Wrapper for "diff" to be used when comparing Xyce output files created on
# one platform to files created on another.  Specifically, when comparing
# output on Losedows to output files ("gold standards") created on proper
# operating systems.

# The issues that tends to bite us are:
#  - Losedows floating point output always has three digits of exponent,
#    other platforms only two
#  - Losedows uses CRLF line breaks
#
# Attempting to use "diff" alone will guarantee failure when attempting
# to compare to gold standards.
#
# It is not necessary to use this script to diff two files created on
# the same machine, as most uses of diff in the test suite should be.

# Usage:
# xplat_diff.pl [diff options] file1 file2

# We first *force* "-bw" into the diff options, telling diff to ignore
# differences in white space altogether.  It takes care of the CRLF issue.
$diffopts=" -bw";

# anything that starts with a -, except a raw - alone (indicating STDIN),
# is presumed to be a diff option.  STOP assuming this at the first argument
# that *doesn't* start with -, or that IS a "-" alone.
     
while ( $ARGV[0] ne "-" && $ARGV[0] =~ /^-/ )
{
    $diffopts .= " $ARGV[0]";
    shift @ARGV;
}

if ($#ARGV != 1)
{
    print STDERR "Two file names not found on command line after options processing.\n";
    print STDERR "Usage:  $0 [diff options] file1 file2\n";
    exit 1;
}

@files=($ARGV[0],$ARGV[1]);
@outfiles=();

foreach $file (@files)
{
    $outfilename=$file . ".xplatdiff";
    push (@outfiles,$outfilename);
    open(THEFILE,"<$file") || die "Cannot open $file for reading.";
    open(THEOUTFILE,">$outfilename") || die "Cannot open $outfilename for writing.";

    while (<THEFILE>)
    {
        s/\r//g;                                    # CR removal
        s/(-*\d\.\d+e[\+-])0([0-9][0-9])/ \1\2/g;   # number fix
        s/[ \t]+/ /g;                               # whitespace fix
        print THEOUTFILE;
    }
}


# on some HPC systems with distributed IO the files written in the prior loop
# may not yet be fully written by the time the system call to diff is done
# check that the files are ready and if they are not then pause the script 
# for a few seconds.  
if( !(-e $outfiles[0]) || !(-e $outfiles[1]))
{
  # files haven't finished being written to file system yet.  pause script for a few seconds.
  sleep(2);
}
    
# now actually do the diff on the processed files.
$cmdstat=system("diff $diffopts $outfiles[0] $outfiles[1] > $$.tmpout 2> $$.tmperr");
$exitcode= $? >> 8;

open(THEOUT,"<$$.tmpout")   || die "Cannot open diff standard output file.";
open(THEERR,"<$$.tmperr")   || die "Cannot open diff standard error file.";

while (<THEOUT>)
{
    print;
}
close(THEOUT);

while (<THEERR>)
{
    print STDERR;
}
close (THEERR);

`rm -f $$.tmpout $$.tmperr $outfiles[0] $outfiles[1]`;

exit $exitcode;
