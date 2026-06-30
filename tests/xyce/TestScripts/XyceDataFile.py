"""
This module reads  Xyce output files in several different formats, and provides
access to the data as an iterable.

It also provides a conversion function that will output the data back in the 
standard (prn) format.

"""
__author__ = "Thomas V. Russo"
__version__ = "1.1"
__date__ = "12 Oct 2016"

import re
import struct

from findBlock import findBlock

def outputStdDataFile(xyceDataObject,outputFileName):
    """
    Take any object from the XyceDataFile module and write a standard 
    format output file from its data
    """
    xyceDataObject.rewind()
    stepRanges=xyceDataObject.getStepRanges()
    columns=xyceDataObject.getColumns()
    # lowercols=map(lambda each:each.lower(),columns)

    outfile=open(outputFileName,'w')
    
    startCol=0
#    if lowercols[0] == 'index':
    if columns[0].lower() == 'index':
        startCol=1

    tagstring = "Index    "
    for tag in columns[startCol:]:
        tagstring += tag + "    "

    tagstring += "\n"

    outfile.write(tagstring)
    
    currentRow=0
    for steprange in stepRanges:
        index=0
        while currentRow <= steprange[1]:
            numberstring=str(index) + "    "
            data=xyceDataObject.next()
            for datum in data[startCol:]:
                numberstring += '%12.8e'%datum + "    "
            numberstring += "\n"
            outfile.write(numberstring)
            index += 1
            currentRow += 1

    outfile.write("End of Xyce(TM) Simulation\n")
            
class XyceStdDataFile:
    """
    Class for handling STD format Xyce output

    Xyce data files ALWAYS have a header line, but the header may or may not
    have an "index" column.
    """
    
    def __init__(self,filename):
        """
        Constructor for XyceStdDataFile object

        This only opens the data file, it does not read all the data into
        an array.
        """
        self.myFileName=filename
        try:
            self.myFile=open(filename,'r')
        except:
            print ("Open failed on %s\n"%filename)
            exit(1)
        self.Columns=[]
        self.setColumns()
        self.excludeCols=[]
        self.indepVar=0
        self.title='A Standard Format, untitled Xyce Output File'
        self.lccols=list(map(lambda each:each.lower(),self.Columns))
        if "index" in self.lccols:
            self.excludeCols.append(self.lccols.index("index"))
            self.indepVar += 1
        if "HOMOTOPY" in filename and "time" in self.lccols:
            self.excludeCols.append(self.lccols.index("time"))
            self.indepVar += 1
        self.excludeCols.append(self.indepVar)

        self.setStepRanges()

    def __iter__(self):
        return self

    def __next__(self):
        """
        Python 3.X iteration operator
        """
        return self.next()
        
    def next(self):
        """ 
        Python 2.X iteration operator
        Returns the next line of data in the file as a list of values
        """
        line=self.myFile.readline()
        if (len(line) == 0):
            raise StopIteration
        splitline=line.split()
        if splitline[0] == 'End':
            raise StopIteration
        else:
            data = [float(j) for j in splitline]

        return data
                
    def rewind(self):
        """
        Rewind the associated file to the first data entry (skipping header)
        """
        self.myFile.seek(0)
        junk=self.myFile.readline()


    def setIndepVar(self,tag):
        """
        Try to reset the independent variable for this datafile to the one with
        the given name
        """
        if tag.lower() in self.lccols:
            self.indepVar=self.lccols.index(tag.lower())
        else:
            print ("Error.  Could not find variable %s.  Using default x-axis variable.\n"%tag)
        self.setStepRanges()
        
    def setColumns(self):
        """
        Retrieve the column names for this data file
        """
        self.myFile.seek(0)     # Rewind, just in case
        line0=self.myFile.readline().rstrip()
        # remove whitespace from expressions
        tags=line0.split()
        self.Columns=[re.sub(r'[ ]',r'',tag) for tag in tags]

    def setStepRanges(self):
        """ 
        Find the step loop ranges for a .step loop output file
        This sets a list of (begin,end) tuples for all the steps present.

        Note that this method is only needed for *plotting* programs, not 
        for conversion programs.
        """
        if 'index' in self.lccols:
            useIndex=self.lccols.index('index')
        else:
            useIndex=self.indepVar

        self.stepRanges=[]
        self.rewind()

        data=self.next()
        stepBegin=0
        stepEnd=0
        lastval=data[useIndex]
        for data in self:
            stepEnd += 1
            if data[useIndex] < lastval:
                self.stepRanges.append((stepBegin,stepEnd-1))
                stepBegin=stepEnd
                stepEnd=stepEnd
            lastval=data[useIndex]
        # No matter what, stepEnd now points to the end of file, and stepBegin
        # points either to the beginning of the last step, or the beginning
        # of the file.  So we have one more step tuple to create
        self.stepRanges.append((stepBegin,stepEnd))
        self.rewind()
        
        
    def getColumns(self):
        return self.Columns

    def getStepRanges(self):
        return self.stepRanges

    def getTitle(self):
        return self.title
    
    def getAllData(self):
        """ 
        Reads all lines of the data file into a single list.  Each element
        of the list is one line of data parsed out into a list of values.

        Provided as a separate method so that enormous data files can be
        read one line at a time instead of pulling the entire file into
        memory.
        """
        self.rewind()
        data=[]
        for row in self:
            data.append(list(row))

        return data

class XyceRawDataFile:
    """
    Class for handling RAW format Xyce output
    """

    def __init__(self,filename):
        """
        Constructor for XyceRawDataFile object

        """

        self.myFileName=filename
        try:
            self.myFile=open(filename,'r')
        except:
            print ("Open failed on %s\n"%filename)
            exit(1)

        self.Columns=[]
        self.excludeCols=[]
        self.indepVar=0
        self.isBinary=False
        self.title=''
        self.xyceVersion='Unknown'
        self.readHeader()
        self.lccols=list(map(lambda each:each.lower(),self.Columns))
        self.setStepRanges()
        
    def readHeader(self):
        """
        Parse a rawfile header, extract information about the run needed
        to work with it.

        sets Columns, detects binary format, determines size of data, etc.
        """
        self.myFile.seek(0)
        self.headerSkip=0
        self.Columns=[]
        keepReading=True
        while keepReading:
            line=self.myFile.readline().rstrip()
            self.headerSkip += 1
            if line.startswith('Title:'):
                self.title=line.rstrip().replace('Title: ','')
            elif line.startswith('No. Variables:'):
                self.numVars=int(line.split()[2])
            elif line.startswith('No. Points:'):
                self.numPoints=int(line.split()[2])
            elif  line.startswith('Version:'):
                self.xyceVersion=line.replace('Version: ','')
            elif line.startswith('Variables:'):
                keepReading=False

        # We are now at the Variables: line
        var=0
        while var < self.numVars:
            line=self.myFile.readline().rstrip()
            self.headerSkip += 1
            var += 1
            fields=line.split('\t')
            self.Columns.append(fields[2].strip('{} '))

        # Now determine binary or not
        line=self.myFile.readline().rstrip()
        self.headerSkip += 1
        self.isBinary=(line.startswith('Binary:'))

        if (self.isBinary):
            self.myFile.close()
            self.myFile = open(self.myFileName,'rb')
            self.binFormat='d'*self.numVars
            self.recordSize=struct.calcsize(self.binFormat)
        
        self.rewind()
        
    def __iter__(self):
        return self

    def __next__(self):
        return self.next()

    def next(self):
        """
        Python 2.X iteration function
        """
        if (self.isBinary):
            try:
                buffer=self.myFile.read(self.recordSize)
                # Gotta check if we're at the start of a new plot, and
                # skip over plot header
                if buffer.startswith('Plotname:'):
                    # Rewind back to beginning of last read data
                    self.myFile.seek(-len(buffer),1)
                    line=self.myFile.readline()
                    while not line.startswith('Binary:'):
                        line=self.myFile.readline()
                    buffer=self.myFile.read(self.recordSize)
                    
                data=struct.unpack(self.binFormat,buffer)
            except:
                raise StopIteration

        else:
            data=[]
            line=self.myFile.readline()
            if (len(line) == 0):
                raise StopIteration
            # gotta check if we're at a new plot
            if line.startswith('Plotname:'):
                while not line.startswith('Values'):
                    line=self.myFile.readline()
                line=self.myFile.readline()
                
            splitline=line.split('\t')
            data.append(float(splitline[1]))
            for varIdx in range(self.numVars-1):
                line=self.myFile.readline()
                if (len(line) == 0):
                    raise EOFError
                data.append(float(line.split()[0]))
            # skip the blank line after each record.
            line=self.myFile.readline()
            if (len(line) == 0):
                raise EOFError
        return data
    
    def rewind(self):
        """ 
        Reset file to beginning, and skip over all header lines.  This
        sets the file up to be read one record at a time by the iterator.
        """
        self.myFile.seek(0)
        for i in range(self.headerSkip):
            junk=self.myFile.readline()

            
    def getAllData(self):
        """ 
        Read all data elements in the entire file, return in one big list of
        lists
        """
        self.rewind()
        data=[]
        for row in self:
            data.append(list(row))

        return data

    def setStepRanges(self):
        """
        Find the step loop ranges for a .step loop output raw file
        This sets a list of (begin,end) tuples for all the steps present.

        Note that this requires reading the whole file, and is only needed
        for *plotting* programs, not for conversion programs.  But we do it
        upon creating the object anyway.

        """
        useIndex=self.indepVar

        self.stepRanges=[]
        self.rewind()

        data=self.next()
        stepBegin=0
        stepEnd=0
        lastval=data[useIndex]
        for data in self:
            stepEnd += 1
            if data[useIndex] < lastval:
                self.stepRanges.append((stepBegin,stepEnd-1))
                stepBegin=stepEnd
                stepEnd=stepEnd
            lastval=data[useIndex]
        # No matter what, stepEnd now points to the end of file, and stepBegin
        # points either to the beginning of the last step, or the beginning
        # of the file.  So we have one more step tuple to create
        self.stepRanges.append((stepBegin,stepEnd))
        self.rewind()

    def setIndepVar(self,tag):
        """
        Try to reset the independent variable for this datafile to the one with
        the given name
        """
        if tag.lower() in self.lccols:
            self.indepVar=self.lccols.index(tag.lower())
        else:
            print ("Error.  Could not find variable %s.  Using default x-axis variable.\n"%tag)
        self.setStepRanges()

    def getColumns(self):
        return self.Columns

    def getStepRanges(self):
        return self.stepRanges

    def getTitle(self):
        return self.title
    
class XyceTecplotDataFile:
    """
    Class for handling Tecplot format Xyce output
    """

    def __init__(self,filename):
        """
        Constructor for XyceTecplotDataFile object

        """

        self.myFileName=filename
        try:
            self.myFile=open(filename,'r')
        except:
            print ("Open failed on %s\n"%filename)
            exit(1)

        self.Columns=[]
        self.excludeCols=[]
        self.indepVar=0
        self.title=''

        self.readHeader()
        self.lccols=list(map(lambda each:each.lower(),self.Columns))
        if "index" in self.lccols:
            self.excludeCols.append(self.lccols.index("index"))
            self.indepVar += 1
        if "HOMOTOPY" in filename and "time" in self.lccols:
            self.excludeCols.append(self.lccols.index("time"))
            self.indepVar += 1
        self.excludeCols.append(self.indepVar)

        self.setStepRanges()


    def readHeader(self):
        """ 
        Read the opening header of the Tecplot file
        """
        self.myFile.seek(0)

        self.headerSkip=0
        
        line=self.myFile.readline().rstrip()
        self.headerSkip += 1
        # Find quote-delimited strings.  These are only present in header lines
        match=findBlock(line,delim='"')
        # Stop when we find no more quoted data
        while match[0] != None:
            fields=line.split()
            if fields[0] != "DATASETAUXDATA" and fields[0] != "ZONE" and fields[0] != "AUXDATA":
                beg=match[0]
                end=match[1]
                tag=line[beg+1:end-1]

                # remove spaces from expressions
                subtag=tag

                if fields[0] == "TITLE":
                    self.title=tag
                else:
                    self.Columns.append(re.sub(r'[ ]',r'',subtag))
            line=self.myFile.readline().rstrip()
            match=findBlock(line,delim='"')
            self.headerSkip += 1

        self.numVars=len(self.Columns)
        self.headerSkip -= 1  # we overcounted

    def __iter__(self):
        return self

    def __next__(self):
        return self.next()

    def next(self):
        line=self.myFile.readline()
        if (len(line) == 0):
            raise StopIteration

        if line.startswith('End'):
            raise StopIteration

        while line.startswith('ZONE') or line.startswith('AUXDATA'):
            line=self.myFile.readline()

        splitline=line.split()
        data=[float(j) for j in splitline]

        return data

    
    def rewind(self):
        """ 
        Reset file to beginning, and skip over all header lines.  This
        sets the file up to be read one record at a time by the iterator.
        """
        self.myFile.seek(0)
        for i in range(self.headerSkip):
            junk=self.myFile.readline()
    
    def setIndepVar(self,tag):
        """
        Try to reset the independent variable for this datafile to the one with
        the given name
        """
        if tag.lower() in self.lccols:
            self.indepVar=self.lccols.index(tag.lower())
        else:
            print ("Error.  Could not find variable %s.  Using default x-axis variable.\n"%tag)


    def setStepRanges(self):
        """
        Find data index ranges for each zone of the Tecplot file.
        Does this by reading whole file.
        """
        dataIndex=-1
        rangeBegin=0
        rangeEnd=-1

        self.stepRanges=[]
        self.myFile.seek(0)
        for line in self.myFile:
            if line.startswith('ZONE') or line.startswith('End'):
                rangeEnd=dataIndex
                if rangeEnd > rangeBegin:
                    self.stepRanges.append((rangeBegin,rangeEnd))
                    rangeBegin=rangeEnd+1
                if line.startswith('End'):
                    break
            else:
                match=findBlock(line,delim='"')
                # Any line without a quote-delimited string in it is data
                if match[0]==None:
                    dataIndex += 1
        self.rewind()

    def getColumns(self):
        return self.Columns

    def getStepRanges(self):
        return self.stepRanges

    def getTitle(self):
        return self.title

    def getAllData(self):
        """ 
        Reads all lines of the data file into a single list.  Each element
        of the list is one line of data parsed out into a list of values.

        Provided as a separate method so that enormous data files can be
        read one line at a time instead of pulling the entire file into
        memory.
        """
        self.rewind()
        data=[]
        for row in self:
            data.append(list(row))

        return data

class XyceCsvDataFile:
    """
    Class for handling CSV format Xyce output

    Xyce CSV data files ALWAYS have a header line, but the header does not
    have an "index" column.
    """
    
    def __init__(self,filename):
        """
        Constructor for XyceCSVDataFile object

        This only opens the data file, it does not read all the data into
        an array.
        """
        self.myFileName=filename
        try:
            self.myFile=open(filename,'r')
        except:
            print ("Open failed on %s\n"%filename)
            exit(1)
        self.Columns=[]
        self.setColumns()
        self.excludeCols=[]
        self.indepVar=0
        self.title='A Standard Format, untitled Xyce Output File'
        self.lccols=list(map(lambda each:each.lower(),self.Columns))
        if "HOMOTOPY" in filename and "time" in self.lccols:
            self.excludeCols.append(self.lccols.index("time"))
            self.indepVar += 1
        self.excludeCols.append(self.indepVar)

        self.setStepRanges()

    def __iter__(self):
        return self

    def __next__(self):
        """
        Python 3.X iteration operator
        """
        return self.next()
        
    def next(self):
        """ 
        Python 2.X iteration operator
        Returns the next line of data in the file as a list of values
        """
        line=self.myFile.readline()
        if (len(line) == 0):
            raise StopIteration
        splitline=line.split(',')
        data = [float(j) for j in splitline]

        return data
                
    def rewind(self):
        """
        Rewind the associated file to the first data entry (skipping header)
        """
        self.myFile.seek(0)
        junk=self.myFile.readline()


    def setIndepVar(self,tag):
        """
        Try to reset the independent variable for this datafile to the one with
        the given name
        """
        if tag.lower() in self.lccols:
            self.indepVar=self.lccols.index(tag.lower())
        else:
            print ("Error.  Could not find variable %s.  Using default x-axis variable.\n"%tag)
        self.setStepRanges()
        
    def setColumns(self):
        """
        Retrieve the column names for this data file
        """
        self.myFile.seek(0)     # Rewind, just in case
        line0=self.myFile.readline().rstrip()

        #Temporarily turn all commas inside parenthesized expressions to
        # semicolons (to make them distinct from commas separating fields)
        match=findBlock(line0,delim="\(")
        while match[0]:
            beg=match[0]
            end=match[1]
            group=line0[beg+1:end-1]
            line0=line0[:beg+1] + re.sub(r',',r';',group) + line0[end-1:]
            match=findBlock(line0,end,delim="\(")
        
        tags=line0.split(',')
        # turn any semicolons back into commas
        self.Columns=[re.sub(r';',r',',tag) for tag in tags]

    def setStepRanges(self):
        """ 
        Find the step loop ranges for a .step loop output file
        This sets a list of (begin,end) tuples for all the steps present.

        Note that this method is only needed for *plotting* programs, not 
        for conversion programs.
        """
        if 'index' in self.lccols:
            useIndex=self.lccols.index('index')
        else:
            useIndex=self.indepVar

        self.stepRanges=[]
        self.rewind()

        data=self.next()
        stepBegin=0
        stepEnd=0
        lastval=data[useIndex]
        for data in self:
            stepEnd += 1
            if data[useIndex] < lastval:
                self.stepRanges.append((stepBegin,stepEnd-1))
                stepBegin=stepEnd
                stepEnd=stepEnd
            lastval=data[useIndex]
        # No matter what, stepEnd now points to the end of file, and stepBegin
        # points either to the beginning of the last step, or the beginning
        # of the file.  So we have one more step tuple to create
        self.stepRanges.append((stepBegin,stepEnd))
        self.rewind()
        
        
    def getColumns(self):
        return self.Columns

    def getStepRanges(self):
        return self.stepRanges

    def getTitle(self):
        return self.title
    
    def getAllData(self):
        """ 
        Reads all lines of the data file into a single list.  Each element
        of the list is one line of data parsed out into a list of values.

        Provided as a separate method so that enormous data files can be
        read one line at a time instead of pulling the entire file into
        memory.
        """
        self.rewind()
        data=[]
        for row in self:
            data.append(list(row))

        return data

def XyceDataFileFactory(filename):
    input=open(filename,'r')
    fields=input.readline().split()
    fields[0]=re.sub(r'[ ]',r'',fields[0])
    #HACK:  If we're a CSV file, there WERE no spaces to split on,
    # and fields[0] will be then entire line, and it will contain commas
    input.close()

    if fields[0] == "#H":
        print ("Sorry, probe format not supported yet.")
        return None
    elif fields[0] == "TITLE":
        return XyceTecplotDataFile(filename)
    elif fields[0] == "Title:":
        return XyceRawDataFile(filename)
    elif "," in fields[0]:
        return XyceCsvDataFile(filename)
    else:
        return XyceStdDataFile(filename)
