#!/usr/bin/env python
"""
Convert a Xyce output file in CSV, PRN, Tecplot, or RAW format to prn.

The output file name will be the input file name with "_converted.prn"
appended.

"""
import os, sys, re
import XyceDataFile

filename=sys.argv[1]
foo=XyceDataFile.XyceDataFileFactory(filename)
outname=filename+"_converted.prn"


XyceDataFile.outputStdDataFile(foo,outname)

