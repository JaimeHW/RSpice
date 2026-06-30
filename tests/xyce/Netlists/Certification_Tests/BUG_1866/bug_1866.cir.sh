#!/usr/bin/env perl

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script 
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file
# $ARGV[5] = current taglist

$XYCE=$ARGV[0];
$TAGSLIST=$ARGV[5];


# This test checks for the presence of version number annotations
# and Xyce/XyceNF/XyceRad name

# get build capabilities:
$capabilities=`$XYCE -capabilities`;
print STDERR "Xyce Capabilities:\n$capabilities\n";

$haverad = ($capabilities =~ m/Radiation models/);
$havenonfree = ($capabilities =~ m/Non-Free device models/);
$haveATHENA = ($capabilities =~ m/ATHENA/);
$haveDakota = ($capabilities =~ m/Dakota/);

print "build has radiation\n" if ($haverad);
print "build has no radiation\n" if (!$haverad);
print "build has nonfree\n" if ($havenonfree);
print "build has no nonfree\n" if (!$havenonfree);
print "build has ATHENA\n" if ($haveATHENA);
print "build has no ATHENA\n" if (!$haveATHENA);
print "build has Dakota\n" if ($haveDakota);
print "build has no Dakota\n" if (!$haveDakota);

$XYCE_VER = `$XYCE -v | tail -1 `;
$XYCE_VER =~ s/\r//g;
chomp($XYCE_VER);

print "Got version $XYCE_VER\n";

$haveXyceName=($XYCE_VER =~ m/^(Xyce|XyceNF|XyceRad) /);
$XyceName=$1;
$annotation="";
$have_annotation=($XYCE_VER =~ m/(-noATHENA$|-nononfree$|-noATHENA-nononfree$|-opensource$|-dakota$)/);
$annotation=$1 if $have_annotation;

$ExpectedName="Xyce" if (!$haverad && !$havenonfree);
$ExpectedName="XyceNF" if (!$haverad && $havenonfree);
$ExpectedName="XyceRad" if $haverad;
$ExpectedAnnotation="-noATHENA-nononfree" if ($haverad && !$haveATHENA && !$havenonfree);
$ExpectedAnnotation="-nononfree" if ($haverad && $haveATHENA && !$havenonfree);
$ExpectedAnnotation="-noATHENA" if ($haverad && !$haveATHENA && $havenonfree);
$ExpectedAnnotation="-opensource" if (!$haverad && !$havenonfree);
$ExpectedAnnotation="-dakota" if ($haveDakota);

if ($have_annotation)
{
    print STDERR "Xyce version has an annotation, and it is $annotation\n";
}

# So, now we know what annotation Xyce puts on the version, and what we
# *should* expect.

$annotation_correct=0;

# First check that annotation actually matches expected
$annotation_correct=1 if (($annotation eq $ExpectedAnnotation) && ($XyceName eq $ExpectedName));

print STDERR "Annotation is ";
if ($annotation_correct)
{
    print STDERR "correct\n";
} else {
    print STDERR "incorrect.  Expected $ExpectedAnnotation, got $annotation\n";
}

print STDERR "Name is ";
if ($XyceName eq $ExpectedName)
{
    print STDERR "correct\n";
} else {
    print STDERR "incorrect.  Expected $ExpectedName, got $XyceName\n";
}

# Now, we need to select an appropriate error code.  
$exitcode = 0;

# FAIL if annotation is incorrect
$exitcode = 2 if (!$annotation_correct);

print "Exit code = $exitcode\n";
exit 0;

    
