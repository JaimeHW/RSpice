#
# This is the common code used in the POWER regression testing 
# 

package PowerCommon;
use strict;

#
# compare the calculated power value with the measured P() and W() values
# inputs are:
#    a 2-D array with the numeric values from the .prn file
#    the indexes for the calculated power, P() and W() values
#    desired absTol, relTol and zeroTol 
#
sub comparePowerColumns{ 
  my ($prnDataRef,$pCalcColIdx,$pXyceColIdx,$wXyceColIdx,$absTol,$relTol,$zeroTol)=@_;
  my @prnData = @$prnDataRef;
  my $idx;
  my $pCalc; my $pXyce; my $wXyce;
  my $absDiffP; my $relDiffP; my $absDiffW; my $relDiffW;
  my $retval = 0;

  foreach $idx (0 .. $#prnData)
  {
    $pCalc = $prnData[$idx][$pCalcColIdx];
    $pXyce = $prnData[$idx][$pXyceColIdx];
    $wXyce = $prnData[$idx][$wXyceColIdx];
    
    if (abs($pCalc) >= $zeroTol)
    {
      $absDiffP = abs($pXyce - $pCalc);
      $relDiffP = $absDiffP / abs($pCalc);   
      if ( ( $absDiffP < $absTol ) && ( $relDiffP < $relTol ) )
      {
        # debug output, to verify correct operation of this subroutine
        # calculated and Xyce P measurement match to absolute tolerance and relative tolerance
        #print STDERR "Numeric comparison for P() succeeded for Index $idx: \n";
        #print STDERR "Power calculated = $pCalc, Xyce P() Output = $pXyce\n";
        #print STDERR "Calculated absDiff and relDiff = ($absDiffP,$relDiffP)\n"; 
      }
      else
      {
        print STDERR "Numeric comparison for P() failed for time Index: $idx\n";
        print STDERR "Column Indexes are $pCalcColIdx and $pXyceColIdx\n";
        print STDERR "Power calculated = $pCalc, Xyce P() Output = $pXyce\n";
        print STDERR "Calculated absDiff and relDiff = ($absDiffP,$relDiffP)\n"; 
        $retval=2;
        last;
      }
      $absDiffW = abs($wXyce - $pCalc);
      $relDiffW = $absDiffW / abs($pCalc);   
      if ( ( $absDiffW < $absTol ) && ( $relDiffW < $relTol ) )
      {
        # calculated and Xyce W measurement match to absolute tolerance and relative tolerance
      }
      else
      {
        print STDERR "Numeric comparison for W() failed for time Index: $idx\n";
        print STDERR "Column Indexes are $pCalcColIdx and $wXyceColIdx\n";
        print STDERR "Power calculated = $pCalc, Xyce P() Output = $wXyce\n";
        print STDERR "Calculated absDiff and relDiff = ($absDiffW,$relDiffW)\n"; 
        $retval=2;
        last;
      } 
    }   
  }

  return($retval);
}

#
# compare two columns.  One should be a calculated value (like lead current) from a
# device while the other would be a solution variable (like current through a voltage source)
# inputs are:
#    a 2-D array with the numeric values from the .prn file
#    the indexes for the columns (i.e. i(dev) and i(vsrc))
#    desired absTol, relTol and zeroTol 
#
sub compareTwoColumns{ 
  my ($prnDataRef,$pCalcColIdx,$pXyceColIdx,$absTol,$relTol,$zeroTol)=@_;
  my @prnData = @$prnDataRef;
  my $idx;
  my $pCalc; my $pXyce; my $wXyce;
  my $absDiffP; my $relDiffP; my $absDiffW; my $relDiffW;
  my $retval = 0;

  foreach $idx (0 .. $#prnData)
  {
    $pCalc = $prnData[$idx][$pCalcColIdx];
    $pXyce = $prnData[$idx][$pXyceColIdx];
    
    if (abs($pCalc) >= $zeroTol)
    {
      $absDiffP = abs($pXyce - $pCalc);
      $relDiffP = $absDiffP / abs($pCalc);   
      if ( ( $absDiffP < $absTol ) && ( $relDiffP < $relTol ) )
      {
        # debug output, to verify correct operation of this subroutine
        # calculated and Xyce P measurement match to absolute tolerance and relative tolerance
        #print STDERR "Numeric comparison for P() succeeded for Index $idx: \n";
        #print STDERR "Power calculated = $pCalc, Xyce P() Output = $pXyce\n";
        #print STDERR "Calculated absDiff and relDiff = ($absDiffP,$relDiffP)\n"; 
      }
      else
      {
        print STDERR "Numeric comparison for columns $pCalcColIdx and $pXyceColIdx failed for time Index $idx: \n";
        print STDERR "Values were  = $pXyce and $pCalc\n";
        print STDERR "Calculated absDiff and relDiff = ($absDiffP,$relDiffP)\n"; 
        $retval=2;
        last;
      }
    }   
  }

  return($retval);
}


#
# Check that the total power is zero
# inputs are:
#    a 2-D array with the numeric values from the .prn file
#    the indexes for the power columns that should sum to zero
#    desired absTol  
# Check only uses absTol, since the comparison is to 0.
#
sub checkPowerBalance{ 
  my ($prnDataRef,$colListRef,$absTol)=@_;
  my @prnData = @$prnDataRef;
  my @colList = @$colListRef;
  my $idx;
  my $colIdx;
  my $pTotal;

  my $retval = 0;

  foreach $idx (0 .. $#prnData)
  {
    $pTotal = 0;
    foreach $colIdx (0 .. $#colList)
    {
      $pTotal = $pTotal + $prnData[$idx][$colList[$colIdx]];
      # debug output, to verify correct operation of this subroutine
      #print STDERR "\tpTotal and current addition from column $colList[$colIdx] = $pTotal , $prnData[$idx][$colList[$colIdx]]\n";  
    }
    if (abs($pTotal) < $absTol )
    {
      # Power balances at this time step 
      # debug output, to verify correct operation of this subroutine
      #print STDERR "Power Balance at Index $idx with power balance diff = $pTotal\n"; 
    }
    else
    {
      print "Power not balanced at Index $idx with difference = $pTotal\n"; 
      $retval=2;
      return($retval);
    } 
  }

  return($retval);
}

#
# This parses data from a Xyce .PRN file generated by .TRAN  This subroutine
# assume that the subroutine checkTranFilesExist() has already been run. 
#
sub parseTranPrnFile{
  my ($CIRFILE)=@_;  
    
  open(XYCE_RESULTS, "$CIRFILE.prn");

  my @dataFromXyce;
  my @lineOfDataFromXyce;
  my $line;
  my $varNum;
  my $numPts = 0;

  while( $line=<XYCE_RESULTS> )
  {
    if( $line =~ /TIME/)
    {  
      # no op.  Skip first line
    }
    elsif($line =~ /End of/)
    {
      # end of Xyce simulation line 
      # do nothing in this case
    }
    else
    {
      # save var values 
      chop $line;
      # Remove leading spaces on line, otherwise the spaces become element 0
      # of "@lineOfDataFromXyce" instead of the first column of data.
      $line =~ s/^\s*//;
      @lineOfDataFromXyce = (split(/\s+/, $line));
      #    print STDERR "Line of data = \'$line\'\n";
      for ($varNum=0; $varNum <= $#lineOfDataFromXyce; $varNum++)
      {
        #print STDERR "setting dataFromXyce[$numPts][$varNum] = $lineOfDataFromXyce[$varNum] \n";
        $dataFromXyce[$numPts][$varNum] = $lineOfDataFromXyce[$varNum];
      }
      $numPts++;
    }
  }
  close(XYCE_RESULTS); 

  return(@dataFromXyce);
}


#
# We need the 1; at the end because, when Perl loads a module, Perl checks
# to see that the module returns a true value to ensure it loaded OK. 
#
1;

#
# End of common code used in each POWER test
#  
