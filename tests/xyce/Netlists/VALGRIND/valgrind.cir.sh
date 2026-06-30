#!/bin/sh

# The input arguments to this script are:
# $1 = location of Xyce binary
# $2 = location of xyce_verify.pl script
# $3 = location of compare script 
# $4 = location of circuit file to test
# $5 = location of gold standard prn file
# $6 = current taglist

XYCE_IN=$1
SUPPFILE=`pwd`/Xyce.supp
XYCE="valgrind --tool=memcheck --leak-check=yes --gen-suppressions=all --suppressions=$SUPPFILE $XYCE_IN"
XYCE_TEST=`echo $2 | sed 's/TestScripts\/xyce_verify.pl//'`
#echo "XYCE_TEST = $XYCE_TEST"
XYCE_VERIFY=`echo $2 | sed -e 's/xyce_verify.pl/valgrind_check.sh/'`
#echo "XYCE_VERIFY = $XYCE_VERIFY"
OUTPUT=`pwd`
mkdir -p $OUTPUT/Netlists/VALGRIND
#echo "OUTPUT = $OUTPUT"
rm -f valgrind.out valgrind.err
echo " "


# create valgrind taglist
TAGLIST="+valgrind"
if [ "$6" == "+valgrindmaster-exclude" ]; then

  echo "WARNING:  Using full valgrindmaster taglist..."
  for i in `$XYCE_TEST/TestScripts/run_xyce_regression --xyce_test=$XYCE_TEST --notest --listalltags`; do 
    TAGLIST="$TAGLIST?$i"  
  done 
  TAGLIST=`echo $TAGLIST | sed s/?List?of?all?tags?present?in?tests://g`
else
  echo "NOTE:  Using augmented user taglist..."
  TAGLIST="$TAGLIST$6"
fi
TAGLIST=`echo $TAGLIST | sed s/.exclude//g | sed s/.valgrindmaster//g`
echo $TAGLIST
echo
echo


$XYCE_TEST/TestScripts/run_xyce_regression --ignoreparsewarnings --xyce_test=$XYCE_TEST --xyce_verify=$XYCE_VERIFY --xyce_compare=$3 --resultfile=valgrind.out --output=$OUTPUT --skipmake --indent="    " --taglist="$TAGLIST" "$XYCE" 2> valgrind.err
STAT=`cat valgrind.out | grep PASSED | wc -l | sed 's/ //g'`
if [ "$STAT" -eq "1" ]; then
  rm -rf valgrind.err
  echo "Exit code = 0"
  exit 0
else
  echo "Valgrind failed test results are located in $OUTPUT/Netlists/VALGRIND" >> valgrind.err
fi
echo "Exit code = 2"
exit 2
