#!/bin/sh

if [ "x$1" = "x--help" ]; then
  exit 0
fi

# get rid of arguments like "--goodres" or other valid xyce_verify options, 
# and get to the actual circuit name
dashes=`echo $1|grep ^-- |wc -c`
while [ $dashes -gt 0 ] 
do
  shift
  dashes=`echo $1|grep ^-- |wc -c`
done

COUNT=`grep "ERROR SUMMARY: 0 errors" $1.err | wc -l | sed 's/ //g'`

# There should be three lines of leak info ("directly", "indirectly" and 
# "possibly" lost).  But it doesn't always print all three, so let's
# check that it prints 0 byte for whatever it *does* print
LOSTLINES=`cat $1.err | grep "ly lost:" | wc -l | sed 's/ //g'`
NOLEAKS=`cat $1.err | grep "ly lost: 0 bytes" | wc -l | sed 's/ //g'`

if [ "$COUNT" -eq "1" -a $NOLEAKS -eq $LOSTLINES ]; then 
  exit 0
fi

cp $1.err ../VALGRIND
exit 1
