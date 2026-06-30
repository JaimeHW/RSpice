#!/usr/bin/env python


import sys,os,re
import subprocess

(XYCE,XYCE_VERIFY,COMPARE,CIRFILE,GOLDPRN)=sys.argv[1:6]
GOLDPARAMS=GOLDPRN.replace("dev_par.cir.prn","gold_par")

#Strip off trailing whitespace that seems to be inserted by nightly suite 
XYCE=XYCE.rstrip()
XYCE_VERIFY=XYCE_VERIFY.rstrip()
CIRFILE=CIRFILE.rstrip()
GOLDPRN=GOLDPRN.rstrip()

DevParCompare=XYCE_VERIFY.replace("xyce_verify.pl","compareDevParams.py")

parOut=open("par",'w')

commandList=[XYCE,"-param","Q","1"]
returncode=subprocess.call(commandList,stdout=parOut)

if returncode != 0:
   print ("Exit code = 10")
   sys.exit(10)

# We now have a successful par file.  Close it.
parOut.close()

compareCommandList=[sys.executable,DevParCompare]

compareCommandList.append(GOLDPARAMS)
compareCommandList.append("par")

returncode=subprocess.call(compareCommandList,stderr=subprocess.STDOUT)

if returncode==0:
   print ("Exit code = 0")
   sys.exit(0)
else:
   print ("Exit code = 2")
   sys.exit(2)

