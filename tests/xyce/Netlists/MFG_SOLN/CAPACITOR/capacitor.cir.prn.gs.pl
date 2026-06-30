#!/usr/bin/env perl

#use XyceRegression::Tools;
#$Tools = XyceRegression::Tools->new();

#$debug = 1;

sub debugPrint {
  print @_ if ($debug);
}

$pi = 4*atan2(1.0,1.0);

# Figure out what R, C, s, a are.
open(CIR,"capacitor.cir");
while ($line = <CIR>) {
  if ($line =~ m/^\.param/i) {
    debugPrint $line;
    @lines = split(/\s+/,$line);
    foreach $token (@lines) {
      debugPrint "\$token = $token\n";
      my @expr = split('=',$token);
#      my $value = $Tools->modVal2Float($expr[1]);
      my $value = modVal2Float($expr[1]);
      debugPrint "\$expr[0] = $expr[0]\n";
      debugPrint "\$expr[1] = $expr[1]\n";
      debugPrint "\$value = $value\n";
      SWITCH: for ($expr[0])
      {
        m/freq/i && do { $freq_val = $value; last; } ;
        m/C/i && do { $C_val = $value; last; } ;
        m/R/i && do { $R_val = $value; last; } ;
        m/a/i && do { $a_val = $value; last; } ;
        m/b/i && do { $b_val = $value; last; } ;
      }
    }
  }
}
close(CIR);
$s_val = 2*$pi*$freq_val;
debugPrint("\$C_val = $C_val\n");
debugPrint("\$R_val = $R_val\n");
debugPrint("\$a_val = $a_val\n");
debugPrint("\$b_val = $b_val\n");
debugPrint("\$freq_val = $freq_val\n");
debugPrint("\$s_val = $s_val\n");

open(INPUT,"capacitor.cir.prn");
open(OUTPUT,">capacitor.cir.prn.gs");
while ($line = <INPUT>)
{
  # Handle first and last lines:
  if ($line =~ m/^[IE]/) 
  { 
    print OUTPUT "$line"; 
    next; 
  }
  my @token = split(" ",$line);
  my $time = $token[1];
  my $v = $a_val*cos($s_val*$time);
#  my $v = $b_val + $a_val*cos($s_val*$time);
#  my $v = $a_val*$time*$time;
#  my $v = $a_val*sin($s_val*$time);
#  my $v = $b_val + exp(-$a_val*$time);
#  my $v = $b_val + exp($a_val*$time);
  debugPrint "\$token[1] = $token[1]\n";
  debugPrint "\$v = $v\n";
  printf OUTPUT "%3d   %14.8e   %14.8e   \n",$token[0],$token[1],$v;
}
close(INPUT);
close(OUTPUT);

sub modVal2Float
{
#  my ($self) = @_;
#  shift @_;
  my ($i)=$_[0];
  my ($mod)="";

  ($mod=$i) =~ s/([\d-+Ee]+)(.*)/$2/;
  if ( $mod ne "")
  {
    $mod=lc($mod);
    SWITCH: for($mod) 
    {
      /t/ && do { $i = $i*1e12; last ;} ;
      /g/ && do { $i = $i*1e9; last ;} ;
      /meg/ && do { $i = $i*1e6; last ;} ;
      /k/ && do { $i = $i*1e3; last ;} ;
      /m/ && do { $i = $i*1e-3; last ;} ;
      /mil/ && do { $i = $i*25.4e-6; last ;} ;
      /u/ && do { $i = $i*1e-6; last ;} ;
      /n/ && do { $i = $i*1e-9; last ;} ;
      /p/ && do { $i = $i*1e-12; last ;} ;
      /f/ && do { $i = $i*1e-15; last ;} ;
# ignore unrecognized modifiers
    }
  }
  return $i;
}
