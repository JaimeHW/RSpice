#!/bin/sh

# If Xyce does not produce a prn file, then we return exit code 10.
# If Xyce succeeds, but the test fails, then we return exit code 2.
# If the shell script fails for some reason, then we return exit code 1.

# Since the shell script runs Xyce and the comparison program, it is
# responsible for capturing any error output from Xyce and the STDOUT & STDERR
# from the comparison program.  The outside script, run_xyce_regression,
# expects the STDERR output from Xyce to go into $CIRFILE.err, the STDOUT
# output from comparison to go into $CIRFILE.prn.out and the STDERR output from
# comparison to go into $CIRFILE.prn.err.

XYCE=$1
XYCE_VERIFY=$2

CIRFILE=gstep_minconduct.cir

$XYCE $CIRFILE > /dev/null 2>$CIRFILE.err

if [ "$?" -ne "0" ]; then
  echo "Exit code = 10"
  exit 10
else
  echo "Exit code = 0"

  exit 0
fi
