#!/usr/bin/env python

#-------------------------------------------------------------------------------
#
# File: compareDevParams.py
#
# Purpose: Intelligently compare Xyce's "-param" output to gold standards
#
#
# Usage:  compareDevParams.py gold_param test_param
#
# Author: Tom Russo, SNL/NM, Electrical and Microsystems Modeling
#
# Date: $Date$
# Revision: $Revision$
# Owner: $Author$
#-------------------------------------------------------------------------------
"""
This script compares Xyce "-param" output data

Usage:  compareDevParams.py gold_param test_param
"""
__author__ = "Thomas V. Russo (tvrusso@sandia.gov)"
__version__ = "$Revision$"
__date__ = "$Date$"

import sys
import re
import os
from getopt import getopt

def parseParams(paramsFileName,paramsDict,*args):
    """
    Read a params file into a dictionary-of-dictionaries:
      paramsDict["DeviceType"] is a dict for the device type
      paramsDict["DeviceType"]["Block"] is a dict for the given block of  data
        Block may be "Configuration", "Model Parameters" or "Instance Parameters"
      paramsDict["DeviceType"]["Block"]["Name"] is a tuple of type,default for
        the named parameter.  Type is always "string" for block Configuration
    """
    try:
        theFile=open(paramsFileName,'r')
    except:
        print("Open failed on %s"%filename)
        sys.exit(1)

    if len(args)>0:
        excludelist=args[0]
    else:
        excludelist=[]
        
    inDevice=False
    blockName=''
    deviceName=''
    compositeName=''
    
    for line in theFile:
        line=line.rstrip()
        # skip blank lines, or lines before any device (e.g. header)
        if len(line) == 0 or (not inDevice and blockName=='' and not ('{' in line)):
            continue
        if (not inDevice) and ('{' in line):
            deviceName=re.match('^ *(.*) {',line).group(1)
            if not (deviceName in excludelist):
                paramsDict[deviceName]={}
            inDevice=True
        elif (inDevice) and (blockName == '') and line.endswith('}'):
            inDevice=False
            deviceName=''
        elif (inDevice) and (blockName=='') and ('{' in line):
            blockName=re.match('^ *(.*) {',line).group(1)
            if not (deviceName in excludelist):
                paramsDict[deviceName][blockName]={}
        elif (inDevice) and (blockName != '') and (compositeName=='') and line.endswith('}'):
            blockName=''
        else:
            extra1=''
            extra2=''
            if blockName != 'Configuration':

                # handle closure of composite block:
                if compositeName!='' and line.endswith('}'):
                    value=compositeDict
                    param=compositeName
                    type='composite'
                    # print ("closing composite block %s"%compositeName)
                    compositeName=''
                else:
                    splitline=line.split(',')
                    if splitline[1]==' composite {':
                        compositeName=splitline[0]
                        compositeDict={}
                        # print ("Opening composite block %s"%compositeName)
                        continue
                    else:
                        param,type,value=splitline[0],splitline[1],splitline[2]
                        if not ('vector' in type):
                            value=re.match('^ *default (.*)',value).group(1)
                            type=re.match('^ *(.*)',type).group(1)
                            if type=='double':
                                value = float(value)
                            if type=='int':
                                value = int(value)
                        if len(splitline)>3:
                            extra1=splitline[3]
                        if len(splitline)>4:
                            extra2=splitline[4]

            else:
                type='string'
                param,value = line.split(':')
                param=re.match('^ *(.*)',param).group(1).rstrip()
                value=re.match('^ *(.*)',value).group(1).rstrip()

            if compositeName == '' and not (deviceName in excludelist):
                paramsDict[deviceName][blockName][param]=(type,value,extra1,extra2)
            elif compositeName != '':
                #print ("Adding param %s to composite %s block %s device %s"%(param,compositeName,blockName,deviceName))
                compositeDict[param]=(type,value,extra1,extra2)
            else:
                #print "Excluding device parameter for device %s"%deviceName
                True
                
    theFile.close()


def main():
    options="e:"
    long_options = ["excludefiles="]
    try:
        opts,args=getopt(sys.argv[1:],options,long_options)
        excludefiles=[]
        excludedevs=[]
        for flag in opts:
            if flag[0] in ("-e","--excludefiles"):
                excludefiles=flag[1].split(',')
                for file in excludefiles:
                    excludefile=open(file,'r')
                    for line in excludefile:
                        excludedevs.append(line.rstrip())
                    excludefile.close()
                
        goldfile,testfile=args[0:2]
    except:
        print ("Unrecognized command line.")
        print (__doc__)
        sys.exit(1)

    goldDict={}
    testDict={}
    parseParams(goldfile,goldDict,excludedevs)
    parseParams(testfile,testDict)

    equivalent=True

    # check that test file has all devices that gold does, and all params match
    for device in goldDict.keys():
        if not (device in testDict):
            print ("Param file under test is MISSING device %s."%device)
            equivalent=False
            continue
            
        # Make sure all blocks for this device exist in the test file
        for block in goldDict[device].keys():
            if not (block in testDict[device]):
                print ("Param file under test is MISSING block %s from device %s"%(block,device))
                equivalent=False
                continue

            # Make sure all params for this device/block exist in the test file
            # and have same value and extra decorators
            for param in goldDict[device][block].keys():
                if not (param in testDict[device][block]):
                    print ("Param file under test is MISSING parameter %s from block %s of device %s"%(param,block,device))
                    equivalent=False
                    continue

                # Configuration block has different format, just param:value
                if block == 'Configuration':
                    goldValue=goldDict[device][block][param][1]
                    testValue=testDict[device][block][param][1]
                    if goldValue != testValue:
                        print ("Param file under test has value %s for parameter %s of Configuration block of device %s, differs from gold value of %s"%(testValue,param,device,goldValue))
                        equivalent=False
                        continue
                else:
                    # Other blocks are param, type, value and extras.
                    # with composite parameters being the complication
                    goldType=goldDict[device][block][param][0]
                    testType=testDict[device][block][param][0]
                    if goldType != testType:
                        print ("Param file under test has type %s for parameter %s of %s block of device %s, differs from gold type of %s"%(testType,param,block,device,goldType))
                        equivalent=False
                        continue

                    if goldType != 'composite':
                        goldValue,goldextra1,goldextra2=goldDict[device][block][param][1:]
                        testValue,testextra1,testextra2=testDict[device][block][param][1:]

                        if not (goldValue==testValue and goldextra1==testextra1 and goldextra2==testextra2):
                            print ("Parameter value mismatch for parameter %s of block %s of device %s: gold=(%s,%s,%s), test=(%s,%s,%s)"%(param,block,device,goldValue,goldextra1,goldextra2,testValue,testextra1,testextra2))
                            equivalent=False
                            continue

                    else:
                        # print ("Sigh.  param %s of block %s of device %s is a composite."%(param,block,device))
                        # Argh, composites.  Gotta compare all the elements
                        # of the composite param
                        # Only thing we have going for us here is that
                        # composite params can't contain other composites
                        compositeDictGold=goldDict[device][block][param][1]
                        compositeDictTest=testDict[device][block][param][1]

                        for subParam in compositeDictGold.keys():
                            #print ("Testing subparam %s of %s of %s of %s"%(subParam,param,block,device))
                            if not (subParam in compositeDictTest):
                                print ("Subparameter %s of composite parameter %s of block %s of device %s does not exist in param file under test."%(subParam,param,block,device))
                                equivalent=False
                                continue
                        
                            goldSubType,goldValue,goldextra1,goldextra2=compositeDictGold[subParam]
                            testSubType,testValue,testextra1,testextra2=compositeDictTest[subParam]

                            if not (goldSubType==testSubType and goldValue==testValue and goldextra1==testextra1 and goldextra2==testextra2):
                                print ("SubParameter %s of param %s of block %s of device %s mismatch: gold=(%s,%s,%s,%s) test=(%s,%s,%s,%s)"%(subParam,param,block,device,goldSubType,goldValue,goldextra1,goldextra2,testSubType,testValue,testextra1,testextra2))
                                equivalent=False
                                continue


    # We have now checked that the test file contains everything that is in
    # the gold file, and that all parameters agree.  We now have to check
    # that the test file doesn't have anything that the gold doesn't.
    for device in testDict.keys():
        if not (device in goldDict):
            print ("Param file under test has extra device %s."%device)
            equivalent=False
            continue
            
        # Make sure all blocks for this device exist in the test file
        for block in testDict[device].keys():
            if not (block in goldDict[device]):
                print ("Param file under test has extra block %s in device %s"%(block,device))
                equivalent=False
                continue

            for param in testDict[device][block].keys():
                if not (param in goldDict[device][block]):
                    print ("Param file under test has extra parameter %s in block %s of device %s"%(param,block,device))
                    equivalent=False
                    continue

                # Don't need to check anything else unless this parameter is
                # a composite, in which case we need to make sure that
                # the composite sub-elements all match
                goldType=goldDict[device][block][param][0]
                testType=testDict[device][block][param][0]

                if testType == 'composite':
                    compositeDictGold=goldDict[device][block][param][1]
                    compositeDictTest=testDict[device][block][param][1]
            
                    for subParam in compositeDictTest.keys():
                        if not (subParam in compositeDictGold):
                            print ("Param file under test has extra subparameter %s of composite parameter %s of block %s of device %s."%(subParam,param,block,device))
                            equivalent=False
                            continue
                    
    
    if equivalent:
        print ("Successful Compare")
        sys.exit(0)
    else:
        print ("Failed compare")
        sys.exit(1)
        
if __name__ == "__main__":
  main()
    
