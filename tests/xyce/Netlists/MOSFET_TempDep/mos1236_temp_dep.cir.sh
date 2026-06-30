#!/usr/bin/env perl

# Tests the MOSFET level 1 temperature interpolation.
# Compares runs at single temperatures to runs interpolated to the same
# temperature.  
#
# GAP: the temperatures tested are exactly the points specified as data for
# setting up the interpolation, not points in between.  A more thorough
# test would test in between those points, too.
#

#For run_xyce_regression
use XyceRegression::Tools;

$Tools = XyceRegression::Tools->new();
$Tools->setDebug(1);
$Tools->setVerbose(1);

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script 
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
#$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];


# Model parameters at varying temperatures
$mosparms{27} = "TNOM=27 UO=566.5 VTO=-1.40 TOX=1.1E-07 KP=2.3e-5 NSUB=1.379E+15 LD=0 NSS=1E10 RSH=0 RS=0 RD=1 IS=1E-14 LAMBDA=0.3 CGDO=1PF CGSO=1PF CGBO=1PF CBD=1PF CBS=1PF";
$mosparms{15} = "TNOM=15 UO=466.5 VTO=-1.45 TOX=1.1E-07 KP=2.3e-5 NSUB=1.379E+16 LD=0 NSS=1E10 RSH=0 RS=0 RD=10 IS=1E-14 LAMBDA=0.3 CGDO=1PF CGSO=1PF CGBO=1PF CBD=1PF CBS=1PF";
$mosparms{66} = "TNOM=66 UO=666.5 VTO=-1.55 TOX=1.1E-07 KP=2.3e-5 NSUB=1.379E+14 LD=0 NSS=1E10 RSH=0 RS=0 RD=0 IS=1E-14 LAMBDA=0.3 CGDO=1PF CGSO=1PF CGBO=1PF CBD=1PF CBS=1PF";

$xyceexit=0;

foreach $level (1,2,3,6)
{
    foreach $temperature (15,27,66)
    {
        @netlists=outputNetlists($temperature,$level,"bug_1471");
        foreach $CIRFILE (@netlists)
        {
            $CMD="$XYCE $CIRFILE > $CIRFILE.out 2>$CIRFILE.err";
            if (system($CMD) != 0)
            {
                `echo "Xyce EXITED WITH ERROR! on case with level=$level, temperature=$temperature, netlist $CIRFILE" >> $CIRFILE.err`;
                $xyceexit=1;
            }
            else
            {
                if (-z "$CIRFILE.err" ) {`rm -f $CIRFILE.err`;}
            }
            
            if ($xyceexit!=0) 
            {
                print "Exit code = 10\n"; 
                exit 10;
            }
            
        }
        # Now compare them:
        $CMD="$XYCE_VERIFY $netlists[0] $netlists[0].prn $netlists[1].prn > $netlists[1].prn.out 2>$netlists[1].prn.err";
        $retval=system($CMD);
        
        if ($retval != 0)
        {
            print "Exit code = 2\n";
            exit 2;
        }
        if (-z "$netlists[1].prn.err" ) {`rm -f $netlists[1].prn.err`;}
    }
}

print "Exit code = 0\n";
exit 0;

sub outputNetlists
{
  my ($temperature,$level,$basename)=@_;
  my $basecir,$interpcir;

  $basecir="$basename" . "_${level}_${temperature}.cir";
  $interpcir="$basename" . "_${level}_${temperature}_interp.cir";

  foreach $filename ($basecir,$interpcir)
  {
      open(CIROUT,">$filename") || die "Cannot open $filename for output\n";
      print CIROUT "CD4020B PMOS Test Circuit - Level $level Mosfet with temp effects\n";

      print CIROUT "VD 1 0 DC 0\n";
      print CIROUT "VG 3 0 DC 0\n";
      print CIROUT "VS 4 0 DC 0\n";
      print CIROUT "VID 1 2 DC 0\n";
      print CIROUT ".options device TEMP=$temperature\n";
      print CIROUT "M1 2 3 4 4 CD4020 W=80u L=6.3u \n";
      
      # now write out the correct models
      if ($filename eq $basecir)
      {
          print CIROUT ".MODEL CD4020 PMOS ( LEVEL=$level $mosparms{$temperature} )\n";
      }
      else
      {
          print CIROUT ".MODEL CD4020 PMOS ( LEVEL=$level TEMPMODEL=QUADRATIC $mosparms{27} )\n";
          print CIROUT ".MODEL CD4020 PMOS ( LEVEL=$level TEMPMODEL=QUADRATIC $mosparms{15} )\n";
          print CIROUT ".MODEL CD4020 PMOS ( LEVEL=$level TEMPMODEL=QUADRATIC $mosparms{66} )\n";
      }

      print CIROUT ".DC VD 0 -1.0 -0.1v VG -1.6 -2.01 -0.1v\n";
      print CIROUT ".PRINT DC V(1) V(3) I(VID)\n";
      print CIROUT ".END\n";
  }

  return ($basecir,$interpcir);
}
