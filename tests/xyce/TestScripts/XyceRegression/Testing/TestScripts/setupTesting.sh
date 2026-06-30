#!/bin/sh

CPFLAGS="-v"

cp $CPFLAGS ../../../run_xyce_regression .
cp $CPFLAGS ../../../xyce_verify.pl .
mkdir -p XyceRegression
cp $CPFLAGS ../../../XyceRegression/UnitTests.pm XyceRegression/
cp $CPFLAGS ../../../XyceRegression/Tools.pm XyceRegression/
mkdir -p XyceVerify
cp $CPFLAGS ../../../XyceVerify/DCSources.pm  XyceVerify/
cp $CPFLAGS ../../../XyceVerify/DCSweep.pm XyceVerify/
cp $CPFLAGS ../../../XyceVerify/StepSweep.pm XyceVerify/
cp $CPFLAGS ../../../compare.c . 
cp $CPFLAGS ../../../Make_util .


rm -f Xyce
echo "#!/usr/bin/env perl" > Xyce
echo 'if ($ARGV[0] =~ m/[-]v/)' >> Xyce
echo "{" >> Xyce
echo "  exit 0" >> Xyce
echo "}" >> Xyce
echo 'print "$0 @ARGV\n";' >> Xyce
echo "exit 0" >> Xyce
chmod +x Xyce

rm -f test_run_xyce_regression.sh
export XYCEPATH=`pwd`
echo "#!/bin/sh" > test_run_xyce_regression.sh
echo "./run_xyce_regression $XYCEPATH/Xyce --taglist=\"+regression\" --excludenotags \$*" >> test_run_xyce_regression.sh
chmod +x test_run_xyce_regression.sh


echo "Run >>test_run_xyce_regression.sh<< to start testing"
echo "E.g.     ./test_run_xyce_regression.sh"
