#!/usr/bin/env python
#-------------------------------------------------------------------------------
#
# File: compareHomotopy.py
#
# Purpose: To quickly compare output files
#
#          This program suppots Xyce's 3 major text 
#          file formats: std, tecplot, probe (csd), and raw.
#
#          It also supports .step output.
#       
#          Spice rawfiles (ascii and binary) are supported.
#          Binary files are architecture specific, so users should only
#          use rawfiles created on the same type of hardware.
#
# Usage:  compareHomotopy.py [options] netlist goodfile testfile
#
# Author: Eric Keiter, SNL/NM, Electrical and Microsystems Modeling
#
# Date: $Date$
# Revision: $Revision$
# Owner: $Author$
#-------------------------------------------------------------------------------
"""
This script compares standard Xyce homotopy output files

Usage:  compareHomotopy.py [options] netlist goodfile testfile
options:
  -h or --help                    this display
  -v or --verbose                 print verbose output
  -c or --column                  specify dependent variable 
  -i or --indep                   specify the independent variable
  -o or --outputstd               output standard (default) data file.
  -t or --outputtecplot           output tecplot data file.
"""
__author__ = "Eric R. Keiter (erkeite@sandia.gov)"
__version__ = "$Revision$"
__date__ = "$Date$"

import os, sys, re
import numpy
import XyceDataFile

from scipy import interpolate 

#-------------------------------------------------------------------------------
def setupPlotVars (tags,poptions):
  """ 
  Index the user-specified plot variables.
  """
  varindex=[]
  if poptions["column"]:
    column = poptions["column"];
    for j in range(len(column)):
      found=0
      for i in range(len(tags)):
        if tags[i] == column[j]:
          varindex.append(i)
          found=1
      if (not found):
        print ("Warning.  Could not find variable:" + column[j])
    if (not varindex):
      sys.exit(1)

  return varindex

#-------------------------------------------------------------------------------
def rhsErrorArc(ugood, good, utest, test, reltol, abstol):
  """ This function computes an RMS error, but with a normalized arc length
      as the independent variable.
  """

  size = len(ugood)
  arcX = numpy.zeros(size)

  # create an arclength vector to serve as the independent variable (X)
  for i in range(0,size):
    if i > 0:
      du = ugood[i]-ugood[i-1]
      dg = good[i]-good[i-1]
      distance = numpy.sqrt(du*du+dg*dg)
      arcX[i] = arcX[i-1] + distance

  # normalize the arcX vector
  arcX /= numpy.max(arcX)

  # use (good-test)
  difference = (good-test)
  differenceSq = (difference/(reltol*abs(good) + abstol))**2.0
  denominator = (reltol*abs(good) + abstol);

  if hasattr(numpy, 'trapezoid'): 
    errorSum = numpy.trapezoid(differenceSq, arcX)
  else: 
    errorSum = numpy.trapz(differenceSq, arcX)
 
  retValue = numpy.sqrt( errorSum )

  return retValue

#-------------------------------------------------------------------------------
# a debug output
def printOutArc(good, test):
  """ 
  """

  size = len(good[:,0])
  arcXgood = numpy.zeros((size, 2), 'double')

  # create a good arclength vector to serve as the independent variable (X)
  for i in range(0,size):
    if i > 0:
      du = good[i,0]-good[i-1,0]
      dg = good[i,1]-good[i-1,1]
      distance = numpy.sqrt(du*du+dg*dg)
      arcXgood[i,0] = arcXgood[i-1,0] + distance

  # normalize the arcXgood vector
  arcXgood[:,0] /= numpy.max(arcXgood[:,0])

  arcXgood[:,1] = good[:,1]

  print ("arcXgood:")
  print (arcXgood)

  size = len(test[:,0])
  arcXtest = numpy.zeros((size, 2), 'double')

  # create a test arclength vector to serve as the independent variable (X)
  for i in range(0,size):
    if i > 0:
      du = test[i,0]-test[i-1,0]
      dg = test[i,1]-test[i-1,1]
      distance = numpy.sqrt(du*du+dg*dg)
      arcXtest[i,0] = arcXtest[i-1,0] + distance

  # normalize the arcXtest vector
  arcXtest[:,0] /= numpy.max(arcXtest[:,0])

  arcXtest[:,1] = test[:,1]

  print ("arcXtest:")
  print (arcXtest)

  return


#-------------------------------------------------------------------------------
def computeErrors(gooddata, testdata, goodtags, goodStepRanges, testStepRanges, reltols, abstols, poptions):
  """ This function performs a parmetric interpolation  of the good data to the 
      test points and then computes the RMS error.
      
  """

  numvars = len(goodtags)
  goodSizeStepRanges = len(goodStepRanges)
  testSizeStepRanges = len(testStepRanges)

  interpgooddata = []
  interptestdata = []
  errors = []

# this needs to be re-arranged!
  for istepRange in range( 0, goodSizeStepRanges):

    goodTuple = goodStepRanges[istepRange]
    testTuple = testStepRanges[istepRange]

    iminGood = int(goodTuple[0])
    imaxGood = int(goodTuple[1])

    iminTest = int(testTuple[0])
    imaxTest = int(testTuple[1])

    Xgood = gooddata[iminGood:imaxGood+1,0]
    Xtest = testdata[iminTest:imaxTest+1,0]

    n_rows = testdata.shape[0]
    n_cols = testdata.shape[1]

    interpgooddata = numpy.zeros((n_rows, n_cols), 'double')
    interptestdata = numpy.zeros((n_rows, n_cols), 'double')
    errors = numpy.zeros(numvars-1)

    for j in range(1, numvars):

      goodview = gooddata[iminGood:imaxGood+1,j]
      testview = testdata[iminTest:imaxTest+1,j]

      fgood,ugood = interpolate.splprep([Xgood,goodview], s=0)
      ftest,utest = interpolate.splprep([Xtest,testview], s=0)

      outGood = interpolate.splev(utest, fgood)
      outTest = interpolate.splev(utest, ftest)

      if j==1:
        interpgooddata[:,0] = outGood[0]
        interptestdata[:,0] = outTest[0]

      interpgooddata[:,j] = outGood[1]
      interptestdata[:,j] = outTest[1]
    
      reltol = reltols[j]
      abstol = abstols[j]

      error = rhsErrorArc(outGood[0], outGood[1], outTest[0], outTest[1], reltol, abstol)
      errors[j-1] = error

  return (interpgooddata, interptestdata, errors)

#-------------------------------------------------------------------------------
def errorReport(errors, cols, reltols):

  for i in range(len(errors)):
    rmsError = reltols[i]*errors[i]*100.0
    name = cols[i+1]

    print ("RMS relative error in ", name, " is ", rmsError)

    returnval=0
    if (rmsError > reltols[i]*100):
      reltol=reltols[i]*100
      returnval=-20
      outputString = "Column %s failed compare, tolerance is %.2f%%, integrated error is %.2f%%\n"%(name,reltol,rmsError)
      sys.stderr.write(outputString)

  return returnval

#-------------------------------------------------------------------------------
from getopt import getopt
import mmap
def main():

  poptions = { "plotopts":[], 
               "verbose":False, 
               "outputstd":False,
               "outputtecplot":False,
               "indep":[],  
               "column":[] }

  progDir,progName = os.path.split(sys.argv[0])
  options = "hvtosaic:p:f"
  long_options = ["help",
                  "verbose", 
                  "outputstd",
                  "outputtecplot",
                  "indep=", 
                  "column=",
                  "plot="]
  try:
    opts,args = getopt(sys.argv[1:],options,long_options)
  except:
    print ("Unrecognized argument")
    print (__doc__)
    sys.exit(1)

  for flag in opts:
    if flag[0] in ("-h","--help"):
      print (__doc__)
      sys.exit()
    elif flag[0] in ("-p","--plot"):
      poptions["plotopts"].append(flag[1])
    elif flag[0] in ("-v","--verbose"):
      poptions["verbose"] = True
    elif flag[0] in ("-c","--column"):
      poptions["column"].append(flag[1])
    elif flag[0] in ("-i","--indep"):
      poptions["indep"].append(flag[1])
    elif flag[0] in ("-o","--outputstd"):
      poptions["outputstd"] = True
    elif flag[0] in ("-t","--outputtecplot"):
      poptions["outputtecplot"] = True
    else:
      print ("Unrecognized flag:", flag[0])
      print (__doc__)
      sys.exit(1)
  if len(args)==0:
    print ("No prn file specified")
    print (__doc__)
    sys.exit(1)

  netlist = args[0]
  gooddatafile = args[1]
  testdatafile = args[2]

  goodfoobar=XyceDataFile.XyceDataFileFactory(gooddatafile)
  testfoobar=XyceDataFile.XyceDataFileFactory(testdatafile)

  goodStepRanges=goodfoobar.getStepRanges()
  testStepRanges=testfoobar.getStepRanges()

  gd = goodfoobar.getAllData()
  td = testfoobar.getAllData()
  gooddata = numpy.array(gd)
  testdata = numpy.array(td)
  
  goodtags = goodfoobar.getColumns()
  testtags = testfoobar.getColumns()

  goodvarindex = setupPlotVars (goodtags,poptions)
  testvarindex = setupPlotVars (testtags,poptions)

  reltol = 1.0e-2
  abstol = 1.0e-12
  reltols = numpy.ones(len(goodtags))*reltol
  abstols = numpy.ones(len(goodtags))*abstol

  # find user-specified reltols and abstols in the netlist, if any exist.
  # if one is found, then substitute the user-specified values from the
  # netlist into the appropriate locations in the reltols and abstols arrays.
  netlist_file_object = open(netlist)
  try:
    for line in netlist_file_object:
      # process line
      strippedLine = line.rstrip()
      if strippedLine.lower().find("comp") != -1:
        s = strippedLine.split()
        tag = ""
        rtol = 1.0e-2
        atol = 1.0e-12
        for i in range(len(s)):
          if s[i].lower().find("comp") != -1 and len(s) > i+1:
            tag = s[i+1]
          if s[i].lower().find("reltol") != -1:
            rtol = float(s[i].split('=')[1])
          if s[i].lower().find("abstol") != -1:
            atol = float(s[i].split('=')[1])
        if tag != "":
          tmptags = [element.lower() for element in goodtags]
          try:
            ind = tmptags.index(tag.lower())
            reltols[ind] = rtol
            abstols[ind] = atol
          except:
            print ("can't find ", tag, " in the output file. Ignoring this one.")

  finally:
    netlist_file_object.close()

  # compute the error for each non-independent column
  interpgooddata,interptestdata, errors = computeErrors(gooddata, testdata, goodtags, goodStepRanges, testStepRanges, reltols, abstols, poptions)

  retcode = errorReport(errors,goodtags, reltols)

  goodtitle=gooddatafile
  testtitle=testdatafile

  # finaly error handling.  This needs more work.
  if retcode == -20:
    sys.stderr.write("Failed compare.\n")
    sys.exit(-retcode)
  else:
    sys.exit(0)

if __name__ == "__main__":
  main()

