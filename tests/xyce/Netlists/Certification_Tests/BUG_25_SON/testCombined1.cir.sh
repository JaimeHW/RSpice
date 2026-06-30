#!/usr/bin/env perl
#
#
# Test harness for bug 25 (SON) tests.
#
# We use the "Transformers" module in this directory to generate objects
# that have netlist and analytic-solution generating methods.  Descriptions
# of the circuits so produced are in the Transformers.pm file.
#
# Bug 25 is related to improper handling of multiple K lines, some of which
# are coupled to others.  This script generates three variants that should
# be identical if Xyce is handling mutual inductance lines properly (which
# it was not doing prior to the fix of bug 25).
#
# In addition to checking that the ordering of K lines is unimportant, this
# test checks Xyce solutions against analytic solutions of the problems.
#

# This script depends on a perl module in the current directory.  
# In Perl 5.26 they removed the current directory from the default search
# path for security reasons:  https://metacpan.org/pod/perl5260delta#Removal-of-the-current-directory-(%22.%22)-from-@INC
# As a result, this script started failing when perl was updated on Windows
# on 19 Dec 2017.  The now-recommended to make a script find modules in
# the current working directory is on the next two lines.
use FindBin qw( $RealBin );
use lib $RealBin;

require Transformers;

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script 
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

#set up for Xyce regression testing
use XyceRegression::Tools;
$Tools=XyceRegression::Tools->new();
$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
#$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

# Now generate transformer models
$frequency=100000;
# A tapped transformer named "Tapped1"
#                                 I0,  f       ,R1  , Lp    , R2 , Ls1   , Ls2
#                             KPS1,KPS2,nodeMin,ID
$theTapped=TappedTransformer->new(5 ,$frequency,1000,.000001,1000,.000001,.000002,
                              .5  ,.8  ,1      ,"Tapped1");

# Figure out where to start numbering nodes in the next model:
$nextNode = $theTapped->getMaxNode() + 1;

# A 1:1 isolation transformer called "1"
#                                I0,   f      , R1 , Lp    , R2 , Ls    , 
#                                 nodeMin ,ID
$theIT=IsolationTransformer->new(5 ,$frequency,1000,.000001,1000,.000001,
                                 $nextNode,"1");

# theNetlist is the netlist fragment for the tapped transformer circuit
# theNetlist2 is the netlist fragment for the isolation transformer circuit
@theNetlist=$theTapped->generateNetlist;
@theNetlist2=$theIT->generateNetlist;

# Now determine the minimum/max values of signals of concern (analytically)
# to choose a shift value.  Use one period of oscillation to do it.  generate
# lots of points

$period=1/$frequency;
$deltat=$period/10000;
for ($t=0;$t<$period+$deltat;$t+=$deltat)
{
    @timepoints[$#timepoints+1]=$t;
}

@results=$theTapped->evaluateAnalytic(@timepoints);

@resultTemp = @{$results[0]};
$returnLast = $#resultTemp ;
undef @resultTemp;

for (my $i = 1 ; $i <= $returnLast; $i++)
{
    $TappedMin[$i] = 100000;
    $TappedMax[$i] = -100000;
}

foreach $resultref (@results)
{
    my @result=@{$resultref};
    for ($i = 1; $i <= $#result; $i++)
    {
        $TappedMin[$i-1] = $result[$i] if ($result[$i] < $TappedMin[$i-1]);
        $TappedMax[$i-1] = $result[$i] if ($result[$i] > $TappedMax[$i-1]);
    }
}
#Trash the results array
$#results=-1;

# Now do the same min/max computation for the isolation transformer
# circuit

@results=$theIT->evaluateAnalytic(@timepoints);

@resultTemp = @{$results[0]};
$returnLast = $#resultTemp ;
undef @resultTemp;

for (my $i = 1 ; $i <= $returnLast; $i++)
{
    $ITMin[$i] = 100000;
    $ITMax[$i] = -100000;
}

foreach $resultref (@results)
{
    my @result=@{$resultref};
    for ($i = 1; $i <= $#result; $i++)
    {
        $ITMin[$i-1] = $result[$i] if ($result[$i] < $ITMin[$i-1]);
        $ITMax[$i-1] = $result[$i] if ($result[$i] > $ITMax[$i-1]);
    }
}
#Trash the results and timepoints arrays
$#results=-1;
$#timepoints=-1;


#Get the names of signals to print
@theTappedPrint = $theTapped->getPrintTerms();
@theITPrint = $theIT->getPrintTerms();

# Now construct an array full of the lines of control cards.
@controlCards = &generateControlCards();


#--------------------------------------
# we now have all the information we need to make a combined netlist.
# We'll do this three times, with different orderings  of K lines.

#--------------------------------------
# First try: put the isolation transformer's K line first, then the tapped.
# This was seriously broken before Bug 25 was fixed.

open (NETLIST,">testCombined_1.cir") || die "Cannot open netlist file for writing.";

print NETLIST "Test netlist for mutual inductance\n";
foreach $line (@theNetlist,@theNetlist2)
{
# Print everything but the K lines
    if (! ($line =~ /^K/))
    {
        print NETLIST $line."\n";
    }
}

# Now print the K lines
print NETLIST $theNetlist2[$#theNetlist2]."\n";
print NETLIST $theNetlist[$#theNetlist-1]."\n";
print NETLIST $theNetlist[$#theNetlist]."\n";

foreach $line (@controlCards)
{
    print NETLIST $line;
}
close (NETLIST);

#--------------------------------------
# Now with a reordered set of K lines, putting the tapped transformer
# K lines first.  This apparently worked even before bug 25 was fixed.
open (NETLIST,">testCombined_2.cir") || die "Cannot open netlist file for writing.";
print NETLIST "Test netlist for mutual inductance\n";
foreach $line (@theNetlist,@theNetlist2)
{
# Print everything but the K lines
    if (! ($line =~ /^K/))
    {
        print NETLIST $line."\n";
    }
}

# Now print the K lines
print NETLIST $theNetlist[$#theNetlist-1]."\n";
print NETLIST $theNetlist[$#theNetlist]."\n";
print NETLIST $theNetlist2[$#theNetlist2]."\n";

foreach $line (@controlCards)
{
    print NETLIST $line."\n";
}
close (NETLIST);

#--------------------------------------
# Now with a reordered set of K lines, putting the isolation transformer's
# K line in between those of the tapped.

open (NETLIST,">testCombined_3.cir") || die "Cannot open netlist file for writing.";
print NETLIST "Test netlist for mutual inductance\n";
foreach $line (@theNetlist,@theNetlist2)
{
# Print everything but the K lines
    if (! ($line =~ /^K/))
    {
        print NETLIST $line."\n";
    }
}

# Now print the K lines
print NETLIST $theNetlist[$#theNetlist-1]."\n";
print NETLIST $theNetlist2[$#theNetlist2]."\n";
print NETLIST $theNetlist[$#theNetlist]."\n";

foreach $line (@controlCards)
{
    print NETLIST $line."\n";
}
close (NETLIST);




# Now we run each of these netlists.
@netlistBaseNames=("testCombined_1","testCombined_2","testCombined_3");

foreach $netlistBaseName (@netlistBaseNames)
{

    # Run the netlist:
    $netlistName=$netlistBaseName . ".cir";
    $netlistPRNName=$netlistBaseName . ".cir.prn";
    $netlistAnalyticName=$netlistBaseName ."_analytic" . ".cir.prn";
    $CMD="$XYCE $netlistName > $netlistName.out 2> $netlistName.err";

    if (system($CMD) != 0)
    {
        print STDERR "Xyce EXITED WITH ERROR on $netlistName\n";
        print "Exit code = 10\n"; 
        exit 10;
    }
    else
    {
        if (-z "$netlistName.err") {`rm -f $netlistName.err`;}
    }

    
# Ok, we get here, the code didn't crash.
# Determine what time points it generated:
    open(PRNFILE,"<$netlistPRNName") || die "Cannot open output file $netlistPRNName";

    while (<PRNFILE>)
    {
        my @line=split(' ');
        if ($line[0] ne "Index" && $line[0] ne "End")
        {
            @timepoints[$#timepoints+1]=$line[1];
        }
        if ($line[0] eq "Index")
        {
            $theFirstLine=$_;
        }
        if ($line[0] eq "End")
        {
            $theLastLine=$_;
        }
    }
    
    close (PRNFILE);

# Now generate an analytic version of the prn file:
    open(ANALYTIC,">$netlistAnalyticName") || die "cannot open analytic file for writing";
    print ANALYTIC $theFirstLine;

    @results=$theTapped->evaluateAnalytic(@timepoints);
    @results2=$theIT->evaluateAnalytic(@timepoints);
    
    for ($i=0; $i<=$#results; $i++)
    {
        my $resultref=$results[$i];
        my @result=@{$resultref};
        print ANALYTIC "$i $result[0] ";
        for (my $j=1; $j<=$#result; $j++)
        {
            print ANALYTIC $result[$j]-$TappedMin[$j-1]*1.1;
            print ANALYTIC " ";
        }
        $resultref=$results2[$i];
        @result=@{$resultref};
        for (my $j=1; $j<=$#result; $j++)
        {
            print ANALYTIC $result[$j]-$ITMin[$j-1]*1.1;
            print ANALYTIC " ";
        }
        print ANALYTIC "\n";
    }
    print ANALYTIC $theLastLine;
    close (ANALYTIC);

    # must zero out timepoints array
    $#timepoints=-1;
    

    $CMD="$XYCE_VERIFY $netlistName $netlistAnalyticName $netlistPRNName > $netlistPRNName.out 2> $netlistPRNName.err";

    $retval=system($CMD);
    if ($retval != 0)
    {
        print STDERR "Verification failed for $netlistName\n";
        print STDERR `cat $netlistPRNName.err`;
        print "Exit code = 2\n";
        exit 2;
    }
}

# If we get here, everything passed.
print "Exit code = 0\n";
exit 0;


# EEEEEEVil use of global variables here.  Shame on me.
sub generateControlCards
{
    my @controlCards;
    my $tempStr;

    #run for four periods of the driving frequency
    $tempStr = ".tran .1u ";
    $tempStr .= $period*4;
    $tempStr .= "\n";
    push @controlCards,$tempStr;

    # Print all the signals that our circuit fragments need, shifted up
    # by slightly mooe than their minimum values, which is OK to do this
    # way as long as all of those values are negative (which they should be).
    $tempStr=".print TRAN ";
    for (my $i=0; $i<=$#theTappedPrint;$i++)
    {
        $tempStr .= "{$theTappedPrint[$i]-1.1*($TappedMin[$i])} ";
    }
    for (my $i=0; $i<=$#theITPrint;$i++)
    {
        $tempStr .= "{$theITPrint[$i]-1.1*($ITMin[$i])} ";
    }
    $tempStr .= "\n";
    push @controlCards,$tempStr;

    # Define comparator tolerances
    for (my $i=0; $i<=$#theTappedPrint;$i++)
    {
        push @controlCards,"*COMP {$theTappedPrint[$i]-1.1*($TappedMin[$i])} abstol=1e-6 reltol=2e-2\n";
    }
    
    for (my $i=0; $i<=$#theITPrint;$i++)
    {
        push @controlCards, "*COMP {$theITPrint[$i]-1.1*($ITMin[$i])} abstol=1e-6 reltol=2e-2\n";
    }

    # Define solution tolerances
    push @controlCards, ".options timeint reltol=1e-4 newbpstepping=1 newlte=1 method=7\n";
    push @controlCards, ".options nonlin-tran abstol=1e-6 reltol=1e-3\n";
    push @controlCards, ".end\n";
    
    return @controlCards;
}
