# 
# Unit Tests for run_xyce_regression and XyceRegression::Tools
#
package XyceRegression::UnitTests;
use strict;

use XyceRegression::Tools;

my $debug;
sub debugPrint { print @_ if ($debug); }

sub runTests {
  my ($debugFlag) = @_;
  $debug = $debugFlag;
  my $retval = 0;
  $retval = containsTag_test();
  $retval += setEqual_test();
  $retval += uniqueSet_test();
  $retval += parseTagList_test(); 
  $retval += getDate_test();
  $retval += createTempFile_test();
  $retval += parseOptionsFile_test();
  $retval += parseTagsFile_test(); 
  $retval += listToHash_test();
  $retval += checkTestTags_test(); 
  $retval += getExcludeHash_test();
  $retval += enableTestByTags_test(); 
  $retval += uniqueTags_test();
  $retval += combineTags_test();
  $retval += setDefaultTags_test();
  $retval += setRunOptions_test();
  $retval += hashEqual_test();
  $retval += mergeHash_test();
  $retval += readAndSetRunOptions_test();
  $retval += getRunOptions_test();
  $retval += configureXyceTestHome_test();
  $retval += excludeListToHash_test();
  $retval += figureCirName_test();
  $retval += parseTestList_test();
# more tests here....
  return $retval;
}

sub containsTag_test {
  my $Tools = XyceRegression::Tools->new();
  my $retval = 0;
  my @taglist = ("serial","nightly");
  print "Self Test:  sub >>containsTag<< ";
  if (not $Tools->containsTag("serial",@taglist)) {
    print " not finding valid entries:  FAILED\n";
    $retval = 1;
  } else {
    print ".";
  }
  if ($Tools->containsTag("exclude",@taglist)) {
    print " finding invalid entries:  FAILED\n";
    $retval = 1;
  } else {
    print ".";
  }
  if ($retval == 0) {
    print " passed\n";
  }
  return $retval;
}

sub parseTagList_test {
  my $Tools = XyceRegression::Tools->new();
  my $retval = 1;
  my (@taglist,@taglistAnswer);
  my (@withtag,@withouttag,@withoptionaltag); 
  my ($withtagref,$withouttagref,$withoptionaltagref); 
  print "Self Test:  sub >>parseTagList<< ";
  @taglist=(
      "+tagA-tagB?tagC", 
      "+tagA-tagB-tagC-tagD?tagE+tagF",
      " + tagA    -tagB   - tagC  ?			tagD+tagE-tagF 		?	 tagG",
      "tagA",
      "tagA,tagB,tagC",
      "+tagA tagB",
      "+tagA+tagB+tagA",
      "+tagA?tagA-tagA",
      "+mos6",
      );
  my @taglistAnswer = ( 
    [["tagA"],["tagB"],["tagC"]], 
    [["tagA","tagF"],["tagB","tagC","tagD"],["tagE"]],
    [["tagA","tagE"],["tagB","tagC","tagF"],["tagD","tagG"]],
    [[],[],[]],
    [[],[],[]],
    [["tagAtagB"],[],[]],
    [["tagA","tagB"],[],[]],
    [["tagA"],["tagA"],["tagA"]],
    [["mos6"],[],[]],
    );
  my $N = $#taglist+1;
  my $i;
  my $correct = 0;
  for ($i=0; $i<$N ; $i++) {
    my $status;
    my $tags = $taglist[$i];
    debugPrint("parseTagList_test:  \$tags = $tags\n");
    ($withtagref,$withouttagref,$withoptionaltagref) = $Tools->parseTagList($tags); 
    my @withtagAnswer = @{$taglistAnswer[$i]->[0]};
    debugPrint("parseTagList_test:  \@withtag = @$withtagref\n");
    debugPrint("parseTagList_test:  \@withtagAnswer = @withtagAnswer\n");
    my @withouttagAnswer = @{$taglistAnswer[$i]->[1]};
    debugPrint("parseTagList_test:  \@withouttag = @$withouttagref\n");
    debugPrint("parseTagList_test:  \@withouttagAnswer = @withouttagAnswer\n");
    my @withoptionaltagAnswer = @{$taglistAnswer[$i]->[2]};
    debugPrint("parseTagList_test:  \@withoptionaltag = @$withoptionaltagref\n");
    debugPrint("parseTagList_test:  \@withoptionaltagAnswer = @withoptionaltagAnswer\n");
    $status = $Tools->setEqual($withtagref,\@withtagAnswer);
    $status += $Tools->setEqual($withouttagref,\@withouttagAnswer);
    $status += $Tools->setEqual($withoptionaltagref,\@withoptionaltagAnswer);
    if ($status == 0) { 
      $correct++; 
      print ".";
    } else {
      print "X";
    }
  }
  if ($correct == $N) { 
    $retval = 0; 
    print " passed\n";
  } else {
    print " FAILED!\n";
  }
  return $retval;
}

sub uniqueSet_test {
  my $Tools = XyceRegression::Tools->new();
  my $retval = 1;
  my (@itemList, @itemListAnswer);
  print "Self Test:  sub >>uniqueSet<< ";
  @itemList = (
      ["A","B","C","A"],
      ["A","B","B","C","A"],
      [],
      [1,2,3,2,1,2,2,2,5],
      ["hello","hello","hello"],
      ["hello","Hello","hello"],
      ["A",undef,"B",undef,undef],
  );
  @itemListAnswer = (
      ["A","B","C"],
      ["A","B","C"],
      [],
      [1,2,3,5],
      ["hello"],
      ["hello","Hello"],
      ["A",undef,"B"],
  );
  my $correct = 0;
  my $totalTests = $#itemList+1;
  for (my $i=0; $i<=$#itemList ; $i++) {
    my @testList = @{$itemList[$i]};
    my @answerList = @{$itemListAnswer[$i]};
    debugPrint "uniqueSet_test:          \@testList   = @testList\n";
    debugPrint "uniqueSet_test:          \@answerList = @answerList\n";
    @testList = $Tools->uniqueSet(@testList);
    debugPrint "uniqueSet_test:  updated \@testList   = @testList\n";
    my $status = $Tools->setEqual(\@testList,\@answerList);
    debugPrint "uniqueSet_test:  \$status = $status\n";
    if ($status eq 0) { 
      print ".";
      $correct++; 
    } else {
      print "X";
    }
  }
  if ($correct == $totalTests) { 
    print " passed\n";
    $retval = 0; 
  } else {
    print " FAILED!\n";
  }
  return $retval;
}

sub setEqual_test {
  my $Tools = XyceRegression::Tools->new();
  my $retval = 1;
  my (%taghashA,%taghashB,%taghashAns);
  print "Self Test:  sub >>setEqual<< ";
  debugPrint "\n";
  my @testSetA = (
    ["A","B","C"],
    ["A","B","C"],
    ["A","B","C"],
    ["A"],
    [],
    ["C","B"],
    ["A","Bb","C"],
    );
  my @testSetB = (
    ["A","B","C"],
    ["B","A","C"],
    ["A","B"],
    ["B"],
    [],
    ["C","B","A"],
    ["A","B","C"],
  );
  my @testSetAnswer = (
    0,
    0,
    1,
    1,
    0,
    1,
    1,
  );
  my $correct = 0;
  my $totalTests = $#testSetA+1;
  for (my $i=0 ; $i<$totalTests ; $i++) {
    my @sublistA = @{$testSetA[$i]};
    my @sublistB = @{$testSetB[$i]};
    my $answer = $testSetAnswer[$i];
    debugPrint "setEqual_test:  \@sublistA = @sublistA\n";
    debugPrint "setEqual_test:  \@sublistB = @sublistB\n";
    debugPrint "setEqual_test:  \$answer = $answer\n";
    my $status = $Tools->setEqual(\@sublistA,\@sublistB);
    debugPrint "setEqual_test:  \$status = $status\n";
    if ($status == $answer) { 
      print ".";
      $correct++; 
    } else {
      print "X";
    }
  }
  if ($correct eq $totalTests) { 
    print " passed\n";
    $retval = 0; 
  } else {
    print " FAILED!\n";
  }
  return $retval;
}

sub getDate_test {
  my $Tools = XyceRegression::Tools->new();
  my $retval = 1;
  my $totalcorrect = 0;
  print "Self Test:  sub >>getDate<< ";
  debugPrint("\n");
  my %delims = (
      "1" => [ ':', '-', ':' ],
      "2" => [ '/', ':', '.' ],
      "3" => [ '_', '.', ':' ]
      );
  # Verify the delimeters were used correctly.
  foreach my $delim (keys %delims) {
    my $correct = 0;
    my $d1 = $delims{"$delim"}->[0]; 
    my $d2 = $delims{"$delim"}->[1]; 
    my $d3 = $delims{"$delim"}->[2]; 
    debugPrint("getDate_test:  \$d1 = >>$d1<<\n");
    debugPrint("getDate_test:  \$d2 = >>$d2<<\n");
    debugPrint("getDate_test:  \$d3 = >>$d3<<\n");
    my $DATE = $Tools->getDate($d1,$d2,$d3);
    my $splitstr = "\\$d1\\$d2\\$d3";
    debugPrint("getDate_test:  \$DATE = >>$DATE<<\n");
    my @datelist = split(/([$splitstr])/,$DATE);
    my ($t1,$t2,$t3);
    shift(@datelist);
    $t1 = shift(@datelist);
    debugPrint("getDate_test:  \$t1 = >>$t1<<\n");
    if ($t1 =~ m/$d1/) { $correct++; }
    shift(@datelist);
    $t1 = shift(@datelist);
    debugPrint("getDate_test:  \$t1 = >>$t1<<\n");
    if ($t1 =~ m/$d1/) { $correct++; }
    shift(@datelist);
    $t2 = shift(@datelist);
    debugPrint("getDate_test:  \$t2 = >>$t2<<\n");
    if ($t2 =~ m/$d2/) { $correct++; }
    shift(@datelist);
    $t3 = shift(@datelist);
    debugPrint("getDate_test:  \$t3 = >>$t3<<\n");
    if ($t3 =~ m/$d3/) { $correct++; }
    shift(@datelist);
    $t3 = shift(@datelist);
    debugPrint("getDate_test:  \$t3 = >>$t3<<\n");
    if ($t3 =~ m/$d3/) { $correct++; }
    if ($correct == 5) {
      print ".";
    } else {
      print "X";
    }
    $totalcorrect += $correct;
  }
  my @delimset = keys(%delims);
  my $N = $#delimset+1;
  if ($totalcorrect == $N*5) {
    $retval = 0;
    print " passed\n";
  } else {
    print " FAILED!\n";
  }
  return $retval;
}

sub parseOptionsFile_test {
  my $Tools = XyceRegression::Tools->new();
  print "Self Test:  sub >>parseOptionsFile<< ";
  debugPrint("\n");
  my $correct = 0;
  my $retval = 2;
  my @testcases = (
      "timelimit=5,progresslimit=10",
      "timelimit = 5 progresslimit =  10",
      "timelimit = 5		progresslimit =  10",
      "timelimit =  5\nprogresslimit   =10",
      "timelimit=0,progresslimit=10",
      "timelimit=360,progresslimit=0",
      "timelimit=0,progresslimit=0",
      "timelimit\n=\n240,progresslimit\n=\n25",
      "timelimit=15",
      "progresslimit=46",
      "progresslimit=5.2, timelimit=62.3",
      " "
      );
  my @testanswers = (
      { 'timelimit' => 5, 'progresslimit' => 10 },
      { 'timelimit' => 5, 'progresslimit' => 10 },
      { 'timelimit' => 5, 'progresslimit' => 10 },
      { 'timelimit' => 5, 'progresslimit' => 10 },
      { 'timelimit' => 0, 'progresslimit' => 10 },
      { 'timelimit' => 360, 'progresslimit' => 0 },
      { 'timelimit' => 0, 'progresslimit' => 0 },
      { 'timelimit' => -1, 'progresslimit' => -1 },
      { 'timelimit' => 15, 'progresslimit' => -1 },
      { 'timelimit' => -1, 'progresslimit' => 46 },
      { 'timelimit' => 62.3, 'progresslimit' => 5.2 },
      { 'timelimit' => -1, 'progresslimit' => -1 },
    );

  my $N = $#testcases+1;
  for (my $i=0 ; $i<$N ; $i++) {
    my $test = $testcases[$i];
    my $tmpfile = $Tools->createTempFile("$test");
    my $optionsref = $Tools->parseOptionsFile($tmpfile);
    my %options = %$optionsref;
    if (not defined $options{'timelimit'}) {
      $options{'timelimit'} = -1;
    } 
    if (not defined $options{'progresslimit'}) {
      $options{'progresslimit'} = -1;
    }
    debugPrint("parseOptionsFile_test:  options hash = \n");
    for my $key (keys %options) {
      debugPrint("parseOptionsFile_test:  $key => $options{$key}\n");
    }
    `rm -f $tmpfile`;
    my %testans = %{$testanswers[$i]};
    debugPrint("parseOptionsFile_test:  \%testans = \n");
    for my $k (keys %testans) {
      debugPrint("parseOptionsFile_test:  $k => $testans{$k}\n");
    }
    if (($options{'timelimit'} eq $testans{'timelimit'}) and 
        ($options{'progresslimit'} eq $testans{'progresslimit'})  ) { 
      $correct++;
      print ".";
    } else {
      print "X";
    }
  }

  if ($correct == $N) { 
    $retval = 0;
    print " passed\n";
  } else {
    print " FAILED!\n";
  }
  return $retval;
}

sub createTempFile_test {
  my $Tools = XyceRegression::Tools->new();
  print "Self Test:  sub >>createTempFile<< ";
  debugPrint("\n");
  my $correct = 0;
  my $retval = 1;
  my @testlist = (
      "foo",
      "foo\nbar",
      "\n\nhello world!\n\n# Just kidding.\nEND",
      "",
      );
  my $totalTests = $#testlist+1;

  for (my $i=0 ; $i <= $#testlist ; $i++) {
    my $test = $testlist[$i];
    debugPrint("createTempFile_test:  \$test = >>$test<<\n");
    my $tmpfile = $Tools->createTempFile($test);
    if ( -f "$tmpfile" )  {
      open(TMPFILE,"<$tmpfile");
      my @filelines = <TMPFILE>;
      close(TMPFILE);
      my $fileline = join("",@filelines);
      debugPrint("createTempFile_test:  \$fileline = >>$fileline<<\n");
      if ($fileline =~ m/$test/) {
        $correct++;
        print ".";
      } else {
        print "X";
      }
      `rm -f $tmpfile`;
    }
  }

  if ($correct == $totalTests) {
    print " passed\n";
    $retval = 0;
  } else {
    print " FAILED!\n";
  }
  return $retval;
}

sub parseTagsFile_test {
  my $Tools = XyceRegression::Tools->new();
  print "Self Test:  sub >>parseTagsFile<< ";
  debugPrint("\n");
  my $correct = 0;
  my $retval = 1;
  my $excludenotagsflag;
  my @tagsTest = (
      "serial,nightly",
      "serial,NIGHTLY",
      "serial\nnightly",
      "serial nightly",
      "serial;nightly",
      "serial\nnightly\nparallel\n\n",
      "#comment\nserial\n",
      "required:regression",
      "serial,parallel\nrequired:regression\nklu",
      "serial, #disabling parallel\nklu",
      "\n\n\n#\n#\nSerial\nREQUIRED:Regression\n\nparallel,klu\n\n",
      "serial,\nnightly",
      "serial,,nightly",
      ",,serial,,nightly",
      "serial,,nightly,,,",
      "serial		,		nightly",
      );
  my @tagsAns = (
      [["serial","nightly" ],[]],
      [["serial","nightly" ],[]],
      [["serial","nightly" ],[]],
      [["serialnightly" ],[]],
      [["serial;nightly" ],[]],
      [["serial","nightly","parallel" ],[]],
      [["serial" ],[]],
      [["regression"],["regression"]],
      [["serial","parallel","klu","regression" ],["regression"]],
      [["serial","klu" ],[]],
      [["serial","regression","parallel","klu"],["regression"]],
      [["serial","nightly" ],[]],
      [["serial","nightly" ],[]],
      [["serial","nightly" ],[]],
      [["serial","nightly" ],[]],
      [["serial","nightly" ],[]],
      );
  my $totalTests = $#tagsTest+4;
# First test for a non-existant file
  my $tmpfile = $Tools->createTempFile("");
  debugPrint "parseTagsFile_test:  \$tmpfile = $tmpfile\n";
  `rm -f $tmpfile`;
# when excludetags=1 and there is no tags file, we should get skipchecks=undef
  $excludenotagsflag = 1;
  my ($taghashref,$requiredtaghashref,$skipchecks) = $Tools->parseTagsFile('.',$tmpfile,$excludenotagsflag);
  if (not defined $skipchecks) {
    $correct++;
    print ".";
  } else {
    print "X";
  }
# when excludenotags=undef and there is no tags file, we should get skipchecks=1
  $excludenotagsflag = undef;
  my ($taghashref,$requiredtaghashref,$skipchecks) = $Tools->parseTagsFile('.',$tmpfile,$excludenotagsflag);
  if ($skipchecks == 1) {
    $correct++;
    print ".";
  } else {
    print "X";
  }
# Then test for valid tags files:
  for (my $i=0; $i<=$#tagsTest; $i++) {
    my $test = $tagsTest[$i];
    my $tmpfile = $Tools->createTempFile($test);
    my ($taghashref,$requiredtaghashref,$skipchecks) = $Tools->parseTagsFile('.',$tmpfile,$excludenotagsflag);
    `rm -f $tmpfile`;
    my %taghash = %$taghashref;
    my %requiredtaghash = %$requiredtaghashref;
    my @taglist = keys(%taghash);
    my @requiredtaglist = keys(%requiredtaghash);

    my @taglistAns = @{$tagsAns[$i]->[0]};
    my @requiredtaglistAns = @{$tagsAns[$i]->[1]};
    debugPrint("parseTagsFile_test:  \@taglist    = @taglist\n");
    debugPrint("parseTagsFile_test:  \@taglistAns = @taglistAns\n");
    debugPrint("parseTagsFile_test:  \@requiredtaglist    = @requiredtaglist\n");
    debugPrint("parseTagsFile_test:  \@requiredtaglistAns = @requiredtaglistAns\n");
    if (($Tools->setEqual(\@taglist,\@taglistAns) eq 0) and 
        ($Tools->setEqual(\@requiredtaglist,\@requiredtaglistAns)eq 0) and
        ($skipchecks == 0)
        ) {
      $correct++;
      print ".";
    } else {
      print "X";
    }
  }
# Lastly, test for an invalid identifier before ":"
  my $tmpfile = $Tools->createTempFile("certainly:serial");
  eval {
    my ($taghashref,$requiredtaghashref,$skipchecks) = $Tools->parseTagsFile('.',$tmpfile,$excludenotagsflag);
  };
  if ($@) {
    $correct++;
    print ".";
  } else {
    print "X";
  }
  `rm -f $tmpfile`;
# Now we check for the right number of tests and return.
  if ($correct == $totalTests) {
    print " passed\n";
    $retval = 0;
  } else {
    print " FAILED!\n";
  }
  return $retval;
}

sub listToHash_test {
  my $Tools = XyceRegression::Tools->new();
  print "Self Test:  sub >>listToHash<< ";
  my $correct = 0;
  my $retval = 1;
  my @testlist = (
      ["a","b"],
      [],
      );
  my $totalTests = $#testlist+1;
  foreach my $test (@testlist) {
    my @locallist = @$test;
    my %localhash = $Tools->listToHash(@locallist);
    my @localhashkeys = keys(%localhash);
    my $localTotalTests = $#locallist+2;
    my $localcorrect = 0;
# First we check the number of keys in the output hash is the same as the
# number of entries in the list
    if ($#localhashkeys == $#locallist) {
      $localcorrect++;
    }
# Then we check that every entry in the list is in the hash
    foreach my $val (@locallist) {
      if (defined $localhash{$val}) {
        $localcorrect++;
      }
    }
# Then, we accumulate the results.
    if ($localcorrect == $localTotalTests) {
      print ".";
      $correct++;
    } else {
      print "X";
    }
  }
  if ($correct == $totalTests) {
    print " passed\n";
    $retval = 0;
  } else {
    print " FAILED!\n";
  }
  return $retval;
}

sub checkTestTags_test {
  my $Tools = XyceRegression::Tools->new();
  print "Self Test:  sub >>checkTestTags<< ";
  debugPrint("\n");
  my $correct = 0;
  my $retval = 1;
  my (@testtaglist,@testtagfile,@testAnswers);
  my $excludenotagsflag;
  my @testAllTagsAnswers;

  push(@testtaglist,"+serial+nightly");
  push(@testtagfile,"serial,nightly");
  push(@testAnswers,['1','1','1']);
  push(@testAllTagsAnswers,['serial','nightly']);

  push(@testtaglist,"+serial+nightly");
  push(@testtagfile,"parallel,nightly");
  push(@testAnswers,['0','1','1']);
  push(@testAllTagsAnswers,['parallel','nightly']);

  push(@testtaglist,"+serial");
  push(@testtagfile,"serial,nightly");
  push(@testAnswers,['1','1','1']);
  push(@testAllTagsAnswers,['serial','nightly']);

  push(@testtaglist,"+serial");
  push(@testtagfile,"");
  push(@testAnswers,['1','1','1']);
  push(@testAllTagsAnswers,[ ]);

  push(@testtaglist,"?regression");
  push(@testtagfile,"");
  push(@testAnswers,['1','1','1']);
  push(@testAllTagsAnswers,[ ]);

  push(@testtaglist,"-verbose");
  push(@testtagfile,"");
  push(@testAnswers,['1','1','1']);
  push(@testAllTagsAnswers,[ ]);

  push(@testtaglist,"+serial");
  push(@testtagfile," ");
  push(@testAnswers,['0','1','1']);
  push(@testAllTagsAnswers,[ ]);

  push(@testtaglist,"+serial-exclude");
  push(@testtagfile,"exclude");
  push(@testAnswers,['0','0','1']);
  push(@testAllTagsAnswers,['exclude']);

  push(@testtaglist,"+serial+nightly");
  push(@testtagfile,"serial,nightly,required:klu");
  push(@testAnswers,['1','1','0']);
  push(@testAllTagsAnswers,['serial','nightly','klu']);

  push(@testtaglist,"+serial+nightly?klu");
  push(@testtagfile,"serial,nightly,required:klu");
  push(@testAnswers,['1','1','1']);
  push(@testAllTagsAnswers,['serial','nightly','klu']);

  push(@testtaglist,"+serial+nightly-verbose");
  push(@testtagfile,"weekly,parallel,verbose,required:klu");
  push(@testAnswers,['0','0','0']);
  push(@testAllTagsAnswers,['weekly','parallel','verbose','klu']);

  push(@testtaglist,"+serial-verbose?klu");
  push(@testtagfile,"serial,verbose,required:klu");
  push(@testAnswers,['1','0','1']);
  push(@testAllTagsAnswers,['serial','verbose','klu']);

  push(@testtaglist,"+serial-verbose");
  push(@testtagfile,"parallel,required:klu");
  push(@testAnswers,['0','1','0']);
  push(@testAllTagsAnswers,['parallel','klu']);

  push(@testtaglist,"+serial-verbose");
  push(@testtagfile,"serial,verbose,required:klu");
  push(@testAnswers,['1','0','0']);
  push(@testAllTagsAnswers,['serial','verbose','klu']);

  my $totalTests = $#testtaglist+2;
  
# Test the case when excludenotags = 1
  $excludenotagsflag = 1;
  my $tmpfile = $Tools->createTempFile("");
  `rm -f $tmpfile`;
  my ($withtagsmatch,$withouttagsmatch,$requiredtagsmatch,$allTagsRef) = $Tools->checkTestTags('.',$tmpfile,"+serial+nightly",$excludenotagsflag);
  debugPrint("checkTestTags_test:  \$withtagsmatch = >>$withtagsmatch<<\n");
  debugPrint("checkTestTags_test:  \$withouttagsmatch = >>$withouttagsmatch<<\n");
  debugPrint("checkTestTags_test:  \$requiredtagsmatch = >>$requiredtagsmatch<<\n");
  debugPrint("checkTestTags_test:  \@allTags = >>@$allTagsRef<<\n");
  my @testAnswer;
  if ( (not defined($withtagsmatch)) and
       (defined($withouttagsmatch)) and
       (defined($requiredtagsmatch)) and
       ($Tools->setEqual($allTagsRef,\@testAnswer) eq 0)
     ) {
    $correct++;
    print ".";
  } else {
    print "X";
  }
  $excludenotagsflag = undef;
  for (my $i=0; $i<=$#testtaglist ; $i++) {
    debugPrint("checkTestTags_test:  test tags = $testtaglist[$i]\n");
    debugPrint("checkTestTags_test:  test tag file = $testtagfile[$i]\n");
    my $tmpfile = $Tools->createTempFile("$testtagfile[$i]");
    if ($testtagfile[$i] =~ m/^$/) {
      `rm -f $tmpfile`;
    }
    my ($withtagsmatch,$withouttagsmatch,$requiredtagsmatch,$allTagsRef) = $Tools->checkTestTags('.',$tmpfile,$testtaglist[$i],$excludenotagsflag);
    if (not defined($withtagsmatch)) { $withtagsmatch = '0'; }
    if (not defined($withouttagsmatch)) { $withouttagsmatch = '0'; }
    if (not defined($requiredtagsmatch)) { $requiredtagsmatch = '0'; }
    debugPrint("checkTestTags_test:  \$withtagsmatch = $withtagsmatch\n");
    debugPrint("checkTestTags_test:  \$withouttagsmatch = $withouttagsmatch\n");
    debugPrint("checkTestTags_test:  \$requiredtagsmatch = $requiredtagsmatch\n");
    debugPrint("checkTestTags_test:  \@allTags = >>@$allTagsRef<<\n");
    `rm -f $tmpfile`;
    if (($withtagsmatch =~ m/$testAnswers[$i]->[0]/) and
        ($withouttagsmatch =~ m/$testAnswers[$i]->[1]/) and
        ($requiredtagsmatch =~ m/$testAnswers[$i]->[2]/) and
        ($Tools->setEqual($allTagsRef,$testAllTagsAnswers[$i]) eq 0)
        ) {
      $correct++;
      print ".";
    } else {
      print "X";
    }
  }
  if ($correct == $totalTests) {
    print " passed\n";
    $retval = 0;
  } else {
    print " FAILED!\n";
  }
  return $retval;
}

sub getExcludeHash_test {
  my $Tools = XyceRegression::Tools->new();
  print "Self Test:  sub >>getExcludeHash<< ";
  debugPrint("\n");
  my $correct = 0;
  my $retval = 1;
  my @excludeTest = (
      "foo.cir",
      "a\nb\nc\n",
      "#comment\n\nfoo.cir # keep this\n 	  bar.cir		 	#more stuff\n",
      "",
      " ",
      "foo.cir,bar.cir",
      );
  my @excludeHashAnswer = (
      { "foo.cir" => 1 },
      { "a" => 1, "b" => 1, "c" => 1},
      { "foo.cir" => 1, "bar.cir" => 1 },
      { },
      { },
      { "foo.cir,bar.cir" => 1 },
      );
  my $totalTests = $#excludeTest+1;
  for (my $i=0; $i<=$#excludeTest; $i++) {
    my $test = $excludeTest[$i];
    my $tmpfile = $Tools->createTempFile($test);
    if ($test =~ m/^$/) {
      debugPrint("getExclueHash_test:  Removing $tmpfile because its empty\n");
      `rm -f $tmpfile`;
    }
    my %excludeHash = $Tools->getExcludeHash('.',$tmpfile);
    `rm -f $tmpfile`;
    my @excludeHashList = keys(%excludeHash);
    my @excludeHashListAnswer = keys(%{$excludeHashAnswer[$i]});
    debugPrint("getExcludeHash_test:  \@excludeHashList ($#excludeHashList)       = >>@excludeHashList<<\n");
    debugPrint("getExcludeHash_test:  \@excludeHashListAnswer ($#excludeHashListAnswer) = >>@excludeHashListAnswer<<\n");
    my $status = $Tools->setEqual(\@excludeHashList,\@excludeHashListAnswer);
    if ($status == 0) {
      $correct++;
      print ".";
    } else {
      print "X";
    }
  }
  if ($correct == $totalTests) {
    print " passed\n";
    $retval = 0;
  } else {
    print " FAILED!\n";
  }
  return $retval;
}

sub enableTestByTags_test {
  my $Tools = XyceRegression::Tools->new();
  print "Self Test:  sub >>enableTestByTags<< ";
  debugPrint("\n");
  my $correct = 0;
  my $retval = 1;
  my (@globalTagsTest,@dirTagsFileTest,@cktTagsFileTest,@enableTestAnswer);
  my (@excludeFileTest,@excludeNoTagsTest);
  my (@allTagsAnswer);

# When there's no tag file, any tests are enabled by default unless excludenotagsflag=1
  push(@globalTagsTest,"+serial+nightly-exclude");
  push(@dirTagsFileTest,"");
  push(@cktTagsFileTest,"");
  push(@excludeFileTest, "");
  push(@excludeNoTagsTest, undef);
  push(@enableTestAnswer,"1");
  push(@allTagsAnswer,[ ]);

  push(@globalTagsTest,"+serial+nightly-exclude");
  push(@dirTagsFileTest,"");
  push(@cktTagsFileTest,"");
  push(@excludeFileTest, "");
  push(@excludeNoTagsTest, 1 );
  push(@enableTestAnswer,"0");
  push(@allTagsAnswer,[ ]);

  push(@globalTagsTest,"");
  push(@dirTagsFileTest,"");
  push(@cktTagsFileTest,"");
  push(@excludeFileTest, "");
  push(@excludeNoTagsTest, undef);
  push(@enableTestAnswer,"1");
  push(@allTagsAnswer,[ ]);

  push(@globalTagsTest,"");
  push(@dirTagsFileTest,"");
  push(@cktTagsFileTest,"");
  push(@excludeFileTest, "");
  push(@excludeNoTagsTest, 1 );
  push(@enableTestAnswer,"1");
  push(@allTagsAnswer,[ ]);

# Directory tags should be observed.
  push(@globalTagsTest,"+serial+nightly-exclude");
  push(@dirTagsFileTest,"exclude");
  push(@cktTagsFileTest,"");
  push(@excludeFileTest, "");
  push(@excludeNoTagsTest, undef );
  push(@enableTestAnswer,"0");
  push(@allTagsAnswer,['exclude']);

  push(@globalTagsTest,"+serial+nightly-exclude");
  push(@dirTagsFileTest,"serial");
  push(@cktTagsFileTest,"");
  push(@excludeFileTest, "");
  push(@excludeNoTagsTest, undef );
  push(@enableTestAnswer,"0");
  push(@allTagsAnswer,['serial']);

  push(@globalTagsTest,"+serial+nightly-exclude");
  push(@dirTagsFileTest,"nightly");
  push(@cktTagsFileTest,"");
  push(@excludeFileTest, "");
  push(@excludeNoTagsTest, undef );
  push(@enableTestAnswer,"0");
  push(@allTagsAnswer,['nightly']);

  push(@globalTagsTest,"+serial+nightly-exclude");
  push(@dirTagsFileTest,"serial,nightly");
  push(@cktTagsFileTest,"");
  push(@excludeFileTest, "");
  push(@excludeNoTagsTest, undef );
  push(@enableTestAnswer,"1");
  push(@allTagsAnswer,['serial','nightly']);

# File tags should be observed
  push(@globalTagsTest,"+serial+nightly-exclude");
  push(@dirTagsFileTest,"");
  push(@cktTagsFileTest,"exclude");
  push(@excludeFileTest, "");
  push(@excludeNoTagsTest, undef );
  push(@enableTestAnswer,"0");
  push(@allTagsAnswer,['exclude']);

  push(@globalTagsTest,"+serial+nightly-exclude");
  push(@dirTagsFileTest,"");
  push(@cktTagsFileTest,"serial");
  push(@excludeFileTest, "");
  push(@excludeNoTagsTest, undef );
  push(@enableTestAnswer,"0");
  push(@allTagsAnswer,['serial']);

  push(@globalTagsTest,"+serial+nightly-exclude");
  push(@dirTagsFileTest,"");
  push(@cktTagsFileTest,"nightly");
  push(@excludeFileTest, "");
  push(@excludeNoTagsTest, undef );
  push(@enableTestAnswer,"0");
  push(@allTagsAnswer,['nightly']);

  push(@globalTagsTest,"+serial+nightly-exclude");
  push(@dirTagsFileTest,"");
  push(@cktTagsFileTest,"serial,nightly");
  push(@excludeFileTest, "");
  push(@excludeNoTagsTest, undef );
  push(@enableTestAnswer,"1");
  push(@allTagsAnswer,['serial','nightly']);

# exclude file should be observed
  push(@globalTagsTest,"+serial+nightly-exclude");
  push(@dirTagsFileTest,"serial,nightly");
  push(@cktTagsFileTest,"");
  push(@excludeFileTest, "1");
  push(@excludeNoTagsTest, undef );
  push(@enableTestAnswer,"0");
  push(@allTagsAnswer,[ 'serial','nightly' ]); 

  push(@globalTagsTest,"+serial+nightly-exclude");
  push(@dirTagsFileTest,"");
  push(@cktTagsFileTest,"serial,nightly");
  push(@excludeFileTest, "1");
  push(@excludeNoTagsTest, undef );
  push(@enableTestAnswer,"0");
  push(@allTagsAnswer,[ ]); # foo.cir.tags not read if it is excluded in "exclude" file

# exclude file trumps all
  push(@globalTagsTest,"+serial+nightly-exclude");
  push(@dirTagsFileTest,"serial,nightly");
  push(@cktTagsFileTest,"serial,nightly,verbose");
  push(@excludeFileTest, "1");
  push(@excludeNoTagsTest, undef );
  push(@enableTestAnswer,"0");
  push(@allTagsAnswer,['serial','nightly']); # foo.cir.tags not read if it is excluded in "exclude" file

  push(@globalTagsTest,"+serial+nightly-exclude");
  push(@dirTagsFileTest,"");
  push(@cktTagsFileTest,"");
  push(@excludeFileTest, "1");
  push(@excludeNoTagsTest, 1);
  push(@enableTestAnswer,"0");
  push(@allTagsAnswer,[ ]);

# file tags trump directory tags
  push(@globalTagsTest,"+serial+nightly-exclude");
  push(@dirTagsFileTest,"weekly,parallel");
  push(@cktTagsFileTest,"serial,nightly");
  push(@excludeFileTest, "");
  push(@excludeNoTagsTest, undef);
  push(@enableTestAnswer,"1");
  push(@allTagsAnswer,['weekly','parallel','serial','nightly']);

  push(@globalTagsTest,"+serial+nightly-exclude");
  push(@dirTagsFileTest,"serial,nightly");
  push(@cktTagsFileTest,"exclude");
  push(@excludeFileTest, "");
  push(@excludeNoTagsTest, undef);
  push(@enableTestAnswer,"0");
  push(@allTagsAnswer,['serial','nightly','exclude']);

  push(@globalTagsTest,"+serial+nightly-exclude");
  push(@dirTagsFileTest,"");
  push(@cktTagsFileTest,"serial,nightly");
  push(@excludeFileTest, "");
  push(@excludeNoTagsTest, undef);
  push(@enableTestAnswer,"1");
  push(@allTagsAnswer,['serial','nightly']);


  my $totalTests = $#globalTagsTest+1;
  for (my $i=0; $i<=$#globalTagsTest; $i++) {
    my $dir = '.';
    my $ckt = $Tools->createTempFile("");
    my $dirtagsfile = $Tools->createTempFile("$dirTagsFileTest[$i]");
    my $ckttagsfile = $Tools->createTempFile("$cktTagsFileTest[$i]");
    my $excludefile;
    if ($excludeFileTest[$i] eq "") {
      $excludefile = $Tools->createTempFile("");
      debugPrint("enableTestByTags_test:  removing excludefile because its empty\n");
      `rm -f $excludefile`;
    } else {
      $excludefile = $Tools->createTempFile("$ckt");
    }
    if ($dirTagsFileTest[$i] =~ m/^$/) {
      debugPrint("enableTestByTags_test:  removing dirtagsfile because its empty\n");
      `rm -f $dirtagsfile`;
    }
    if ($cktTagsFileTest[$i] =~ m/^$/) {
      debugPrint("enableTestByTags_test:  removing ckttagsfile because its empty\n");
      `rm -f $ckttagsfile`;
    }
    my $excludenotagsflag = $excludeNoTagsTest[$i];
    debugPrint("enableTestByTags_test:  dir = $dir\n");
    debugPrint("enableTestByTags_test:  ckt = $ckt\n");
    debugPrint("enableTestByTags_test:  dirtagsfile = $dirtagsfile\n");
    debugPrint("enableTestByTags_test:  dirtags     = $dirTagsFileTest[$i]\n");
    debugPrint("enableTestByTags_test:  ckttagsfile = $ckttagsfile\n");
    debugPrint("enableTestByTags_test:  ckttags     = $cktTagsFileTest[$i]\n");
    debugPrint("enableTestByTags_test:  excludefile = $excludefile\n");
    debugPrint("enableTestByTags_test:  exclude     = $excludeFileTest[$i]\n");
    debugPrint("enableTestByTags_test:  global tags = $globalTagsTest[$i]\n");
    debugPrint("enableTestByTags_test:  \$excludenotags = $excludenotagsflag\n");
    my ($success,$allTagsListRef) = $Tools->enableTestByTags($dir,$ckt,$excludefile,$dirtagsfile,$ckttagsfile,$globalTagsTest[$i],$excludenotagsflag);
    `rm -f $ckt $dirtagsfile $ckttagsfile $excludefile`;
    if (not defined $success) {
      $success = "0";
    }
    debugPrint("enableTestByTags_test:  \$success    = $success\n");
    debugPrint("enableTestByTags_test:  correct ans = $enableTestAnswer[$i]\n");
    debugPrint("enableTestByTags_test:  all tags = >>@$allTagsListRef<<\n");
    debugPrint("enableTestByTags_test:  all tags answer = >>@{$allTagsAnswer[$i]}<<\n");
    if (
        ($success eq $enableTestAnswer[$i]) and
        ($Tools->setEqual($allTagsListRef,$allTagsAnswer[$i]) eq 0)
       ) {
      print ".";
      $correct++;
    } else {
      print "X";
    }
  }
# Now we check for the right number of tests and return.
  if ($correct == $totalTests) {
    print " passed\n";
    $retval = 0;
  } else {
    print " FAILED!\n";
  }
  return $retval;

}

sub uniqueTags_test {
  my $Tools = XyceRegression::Tools->new();
  my $retval = 1;
  my (@itemList, @itemListAnswer);
  print "Self Test:  sub >>uniqueTags<< ";
  debugPrint("\n");
  @itemList = (
      "+tagA-tagB+tagA",
      "+tagA-tagB?tagA",
      "+tagA-tagA?tagA",
      "+tagA-tagB+tagC-tagB",
      "+tagA-tagB?tagC-tagD?tagC",
  );
  @itemListAnswer = (
      "+tagA-tagB",
      "+tagA-tagB?tagA",
      "+tagA-tagA?tagA",
      "+tagA-tagB+tagC",
      "+tagA-tagB?tagC-tagD",
  );
  my $correct = 0;
  my $totalTests = $#itemList+1;
  for (my $i=0; $i<=$#itemList ; $i++) {
    my $test = $itemList[$i];
    my $answer = $itemListAnswer[$i];
    debugPrint("uniqueTags_test:          \$test   = $test\n");
    debugPrint("uniqueTags_test:          \$answer = $answer\n");
    $test = $Tools->uniqueTags($test);
    debugPrint("uniqueTags_test:  updated \$test   = $test\n");
    if ($test eq $answer)  {
      print ".";
      $correct++; 
    } else {
      print "X";
    }
  }
  if ($correct == $totalTests) { 
    print " passed\n";
    $retval = 0; 
  } else {
    print " FAILED!\n";
  }
  return $retval;
}

sub combineTags_test {
  my $Tools = XyceRegression::Tools->new();
  $Tools->setQuiet(1);
  my $retval = 1;
  my (@withTagsTests, @tagListAnswer);
  print "Self Test:  sub >>combineTags<< ";
  debugPrint("\n");
  @withTagsTests = (
      [["tagA"],["tagB"],["tagC"],"+tagD"],
      [["tagA"],["tagB"],["tagC"],"+tagA"],
      [["tagA"],["tagB"],["tagC"],""],
      [["tagA"],["tagB"],["tagC"],undef],
      [[],[],[],"+tagA-tagB?tagC"],
      [[],[],[],undef],
      [["A"],["B"],["C"],"  +A"],
      [["A"],["B"],["C"],"  -B"],
      [["A"],["B"],["C"],"  ?C"],
      [["A"],["B"],["C"],"+D+A"],
      [["A"],["B"],["C"],"-D-B"],
      [["A"],["B"],["C"],"?D?C"],
      [["A"],["B"],["C"],"  +A+D"],
      [["A"],["B"],["C"],"  -B-D"],
      [["A"],["B"],["C"],"  ?C?D"],
      );
  @tagListAnswer = (
      "+tagA-tagB?tagC+tagD",
      "+tagA-tagB?tagC",
      "+tagA-tagB?tagC",
      "+tagA-tagB?tagC",
      "+tagA-tagB?tagC",
      undef,
      "+A-B?C",
      "+A-B?C",
      "+A-B?C",
      "+A-B?C+D",
      "+A-B?C-D",
      "+A-B?C?D",
      "+A-B?C+D",
      "+A-B?C-D",
      "+A-B?C?D",
      );
  my $correct = 0;
  my $totalTests = $#withTagsTests+1;
  for (my $i=0; $i<=$#withTagsTests ; $i++) {
    my @localwithtags = @{$withTagsTests[$i]->[0]};
    my @localwithouttags = @{$withTagsTests[$i]->[1]};
    my @localwithoptionaltags = @{$withTagsTests[$i]->[2]};
    my $localtaglist = $withTagsTests[$i]->[3];
    my $localtaglistanswer = $tagListAnswer[$i];
    debugPrint("combineTags_test:  \@localwithtags = @localwithtags\n");
    debugPrint("combineTags_test:  \@localwithouttags = @localwithouttags\n");
    debugPrint("combineTags_test:  \@localwithoptionaltags = @localwithoptionaltags\n");
    debugPrint("combineTags_test:  \$localtaglist = $localtaglist\n");
    my $newlocaltaglist = $Tools->combineTags(\@localwithtags,\@localwithouttags,\@localwithoptionaltags,$localtaglist);
    debugPrint("combineTags_test:  \$newlocaltaglist    = $newlocaltaglist\n");
    debugPrint("combineTags_test:  \$localtaglistanswer = $localtaglistanswer\n");
    if ($newlocaltaglist eq $localtaglistanswer)  {
      print ".";
      $correct++; 
    } else {
      print "X";
    }
  }
  if ($correct == $totalTests) { 
    print " passed\n";
    $retval = 0; 
  } else {
    print " FAILED!\n";
  }
  return $retval;
}

sub setDefaultTags_test {
  my $Tools = XyceRegression::Tools->new();
  my $retval = 1;
  my (@itemList, @itemListAnswer);
  print "Self Test:  sub >>setDefaultTags<< ";
  debugPrint("\n");
  @itemList = (
      "",
      "+serial",
      "+nightly",
      "-parallel",
      "?regression",
      "+exclude",
      "-exclude",
      "?exclude",
      "+serial+nightly-exclude",
  );
  @itemListAnswer = (
      "+serial+nightly-exclude",
      "+serial-exclude",
      "+nightly-exclude",
      "-parallel-exclude",
      "?regression-exclude",
      "+exclude",
      "-exclude",
      "?exclude",
      "+serial+nightly-exclude",
  );
  my $correct = 0;
  my $totalTests = $#itemList+1;
  for (my $i=0; $i<=$#itemList ; $i++) {
    my $test = $itemList[$i];
    my $answer = $itemListAnswer[$i];
    debugPrint("setDefaultTags_test:          \$test   = $test\n");
    debugPrint("setDefaultTags_test:          \$answer = $answer\n");
    $test = $Tools->setDefaultTags($test);
    debugPrint("setDefaultTags_test:  updated \$test   = $test\n");
    if ($test eq $answer)  {
      print ".";
      $correct++; 
    } else {
      print "X";
    }
  }
  if ($correct == $totalTests) { 
    print " passed\n";
    $retval = 0; 
  } else {
    print " FAILED!\n";
  }
  return $retval;
}

sub setRunOptions_test {
  print "Self Test:  sub >>setRunOptions<< ";
  debugPrint("\n");
  my $correct = 0;
  my $retval = 2;
  my @testcases = (
      { },
      { 'timelimit' => 20, 'progresslimit' => 10 },
      { 'timelimit' => 20, },
      { 'progresslimit' => 10 },
      );
  my @testanswers = (
      { 'timelimit' => -1, 'progresslimit' => -1 },
      { 'timelimit' => 20, 'progresslimit' => 10 },
      { 'timelimit' => 20, 'progresslimit' => -1 },
      { 'timelimit' => -1, 'progresslimit' => 10 },
      );

  my $N = $#testcases+1;
  for (my $i=0 ; $i<$N ; $i++) {
    my $tools = XyceRegression::Tools->new();
    my $defaultTimeLimit = $tools->getChildWait();
    my $defaultProgressLimit = $tools->getChildProgress();
    my %testans = %{$testanswers[$i]};
    if ($testans{'timelimit'} eq -1) {
      $testans{'timelimit'} = $defaultTimeLimit;
    }
    if ($testans{'progresslimit'} eq -1) {
      $testans{'progresslimit'} = $defaultProgressLimit;
    }
    my %test = %{$testcases[$i]};
    my $test_timelimit = $test{'timelimit'};
    my $test_progresslimit = $test{'progresslimit'};
    debugPrint("setRunOptions_test:  \$test{'timelimit'} = >>$test_timelimit<<\n");
    debugPrint("setRunOptions_test:  \$test{'progresslimit'} = >>$test_progresslimit<<\n");
    $tools->setRunOptions(\%test);
    my $testans_timelimit = $testans{'timelimit'};
    my $testans_progresslimit = $testans{'progresslimit'};
    my $result_timelimit = $tools->getChildWait();
    my $result_progresslimit = $tools->getChildProgress();
    debugPrint("setRunOptions_test:  \$testans{'timelimit'} = $testans_timelimit\n");
    debugPrint("setRunOptions_test:  \$testans{'progresslimit'} = $testans_progresslimit\n");
    debugPrint("setRunOptions_test:  \$tools->getChildWait() = $result_timelimit\n");
    debugPrint("setRunOptions_test:  \$tools->getChildProgress() = $result_progresslimit\n");
    if ( 
        ( $testans_timelimit eq $result_timelimit ) and 
        ( $testans_progresslimit eq $result_progresslimit ) 
       ) {
      $correct++;
      print ".";
    } else {
      print "X";
    }
  }

  if ($correct == $N) { 
    $retval = 0;
    print " passed\n";
  } else {
    print " FAILED!\n";
  }
  return $retval;
}

sub readAndSetRunOptions_test {
# Set up a directory options file
# Set up a file options file
# Call readAndSetRunOptions
# Verify $Tools has the options set correctly
  print "Self Test:  sub >>readAndSetRunOptions<< ";
  my $correct = 0;
  my $retval = 2;
  # directory options are first, followed by ckt options:
  my @testcases = (
      [ "", "" ],
      [ " ", "" ],
      [ "", " " ],
      [ " ", " " ],
      [ "timelimit=5,progresslimit=10", ""],
      [ "", "timelimit=5,progresslimit=10" ],
      [ "timelimit=5","progresslimit=10" ],
      [ "progresslimit=5","timelimit=10" ],
      [ "timelimit=250", "timelimit=25" ],
      [ "progresslimit=250", "progresslimit=25" ],
      [ "timelimit=2,progresslimit=1", "timelimit=20" ],
      [ "timelimit=2,progresslimit=1", "progresslimit=20" ],
      [ "timelimit=2,progresslimit=1", "timelimit=20,progresslimit=20" ],
      );
  my @testanswers = (
      { 'timelimit' => -1, 'progresslimit' => -1 },
      { 'timelimit' => -1, 'progresslimit' => -1 },
      { 'timelimit' => -1, 'progresslimit' => -1 },
      { 'timelimit' => -1, 'progresslimit' => -1 },
      { 'timelimit' => 5, 'progresslimit' => 10 },
      { 'timelimit' => 5, 'progresslimit' => 10 },
      { 'timelimit' => 5, 'progresslimit' => 10 },
      { 'timelimit' => 10, 'progresslimit' => 5 },
      { 'timelimit' => 25, 'progresslimit' => -1 },
      { 'timelimit' => -1, 'progresslimit' => 25 },
      { 'timelimit' => 20, 'progresslimit' => 1 },
      { 'timelimit' => 2, 'progresslimit' => 20 },
      { 'timelimit' => 20, 'progresslimit' => 20 },
    );

  my $N = $#testcases+1;
  for (my $i=0 ; $i<$N ; $i++) {
    my $tools = XyceRegression::Tools->new();
    my $dirOptions = $testcases[$i]->[0];
    my $cktOptions = $testcases[$i]->[1];
    my $dirTmpFile = $tools->createTempFile("$dirOptions");
    if ($dirOptions eq "") { `rm -f $dirTmpFile`; }
    my $cktTmpFile = $tools->createTempFile("$cktOptions");
    if ($cktOptions eq "") { `rm -f $cktTmpFile`; }
    my $defaultTimeLimit = $tools->getChildWait();
    my $defaultProgressLimit = $tools->getChildProgress();
    my %testans = %{$testanswers[$i]};
    if ($testans{'timelimit'} eq -1) {
      $testans{'timelimit'} = $defaultTimeLimit;
    }
    if ($testans{'progresslimit'} eq -1) {
      $testans{'progresslimit'} = $defaultProgressLimit;
    }
    $tools->readAndSetRunOptions($dirTmpFile,$cktTmpFile);
    `rm -f $dirTmpFile $cktTmpFile`;
    if ( 
        ( $testans{'timelimit'} eq $tools->getChildWait() ) and 
        ( $testans{'progresslimit'} eq $tools->getChildProgress() ) 
       ) {
      $correct++;
      print ".";
    } else {
      print "X";
    }
  }

  if ($correct == $N) { 
    $retval = 0;
    print " passed\n";
  } else {
    print " FAILED!\n";
  }
  return $retval;
}

sub mergeHash_test {
  my $Tools = XyceRegression::Tools->new();
  print "Self Test:  sub >>mergeHash<< ";
  my $correct = 0;
  my $retval = 2;
  my @testcases = (
      [ { }, { } ],
      [ { 'timelimit' => 20, 'progresslimit' => 10 }, { } ],
      [ { }, { 'timelimit' => 20, 'progresslimit' => 10 } ],
      [ { }, { 'timelimit' => 20 } ],
      [ { 'timelimit' => 20 }, { 'progresslimit' => 10 } ],
      [ { 'progresslimit' => 10 }, { 'timelimit' => 20 } ],
      [ { 'timelimit' => 250 }, { 'timelimit' => 25 } ],
      [ { 'progresslimit' => 250 }, { 'progresslimit' => 25 } ],
      [ { 'timelimit' => 20, 'progresslimit' => 10 }, { 'timelimit' => 250 } ],
      [ { 'timelimit' => 20, 'progresslimit' => 10 }, { 'progresslimit' => 15 } ],
      [ { 'timelimit' => 20, 'progresslimit' => 10 }, { 'timelimit' => 250, 'progresslimit' => 100 } ],
      );
  my @testanswers = (
      { },
      { 'timelimit' => 20, 'progresslimit' => 10 },
      { 'timelimit' => 20, 'progresslimit' => 10 },
      { 'timelimit' => 20 },
      { 'timelimit' => 20, 'progresslimit' => 10 },
      { 'timelimit' => 20, 'progresslimit' => 10 },
      { 'timelimit' => 25 },
      { 'progresslimit' => 25 },
      { 'timelimit' => 250, 'progresslimit' => 10 },
      { 'timelimit' => 20, 'progresslimit' => 15 },
      { 'timelimit' => 250, 'progresslimit' => 100 },
    );

  my $N = $#testcases+1;
  for (my $i=0 ; $i<$N ; $i++) {
    my $dirRef = $testcases[$i]->[0];
    my $cktRef = $testcases[$i]->[1];
    my $optionsRef = $Tools->mergeHash($dirRef,$cktRef);
    if ($Tools->hashEqual($testanswers[$i],$optionsRef) eq 0) {
      $correct++;
      print ".";
    } else {
      print "X";
    }
  }

  if ($correct == $N) { 
    $retval = 0;
    print " passed\n";
  } else {
    print " FAILED!\n";
  }
  return $retval;

}

sub hashEqual_test {
  my $Tools = XyceRegression::Tools->new();
  print "Self Test:  sub >>hashEqual<< ";
  my $correct = 0;
  my $retval = 2;
  my @testcases = (
      [ { }, { } ],
      [ { 'a' => 1 }, { } ],
      [ { }, { 'a' => 1 } ],
      [ { 'a' => 1 }, { 'a' => 2 } ],
      [ { 'a' => 1 }, { 'b' => 1 } ],
      [ { 'A' => 'a', 'B' => 'b' }, { 'A' => 'a', 'B' => 'b' } ],
      [ { 'A' => 'a', 'B' => 'b' }, { 'A' => 'a', 'B' => 'b', 'C' => 'c' } ],
      [ { 'A' => 'a', 'B' => 'b', 'C' => 'c' }, { 'A' => 'a', 'B' => 'b' } ],
      );
  my @testanswers = (
      0,
      1,
      1,
      1,
      1,
      0,
      1,
      1,
    );

  my $N = $#testcases+1;
  for (my $i=0 ; $i<$N ; $i++) {
    my $hash1Ref = $testcases[$i]->[0];
    my $hash2Ref = $testcases[$i]->[1];
    my $ans = $testanswers[$i];
    my $result = $Tools->hashEqual($hash1Ref,$hash2Ref);
    if ($ans eq $result) {
      $correct++;
      print ".";
    } else {
      print "X";
    }
  }

  if ($correct == $N) { 
    $retval = 0;
    print " passed\n";
  } else {
    print " FAILED!\n";
  }
  return $retval;

}

sub getRunOptions_test {
  print "Self Test:  sub >>getRunOptions<< ";
  debugPrint("\n");
  my $correct = 0;
  my $retval = 2;
  my @testcases = (
      { },
      { 'timelimit' => 20, 'progresslimit' => 10 },
      { 'timelimit' => 20, },
      { 'progresslimit' => 10 },
      );
  my @testanswers = (
      { 'timelimit' => -1, 'progresslimit' => -1 },
      { 'timelimit' => 20, 'progresslimit' => 10 },
      { 'timelimit' => 20, 'progresslimit' => -1 },
      { 'timelimit' => -1, 'progresslimit' => 10 },
      );

  my $N = $#testcases+1;
  for (my $i=0 ; $i<$N ; $i++) {
    my $tools = XyceRegression::Tools->new(); 
    my $defaultTimeLimit = $tools->getChildWait();
    my $defaultProgressLimit = $tools->getChildProgress();
    debugPrint("getRunOptions_test:  \$defaultTimeLimit = $defaultTimeLimit\n");
    debugPrint("getRunOptions_test:  \$defaultProgressLimit = $defaultProgressLimit\n");
    my %testans = %{$testanswers[$i]};
    if ($testans{'timelimit'} eq -1) {
      $testans{'timelimit'} = $defaultTimeLimit;
    }
    if ($testans{'progresslimit'} eq -1) {
      $testans{'progresslimit'} = $defaultProgressLimit;
    }
    my %test = %{$testcases[$i]};
    if (defined $test{'timelimit'}) {
      $tools->setChildWait($test{'timelimit'})
    } 
    if (defined $test{'progresslimit'}) {
      $tools->setChildProgress($test{'progresslimit'});
    }
    my $runOptionsRef = $tools->getRunOptions();
    my %runOptions = %$runOptionsRef;
    if ( 
        ( $testans{'timelimit'} eq $runOptions{'timelimit'} ) and 
        ( $testans{'progresslimit'} eq $runOptions{'progresslimit'} ) 
       ) {
      $correct++;
      print ".";
    } else {
      print "X";
    }
  }

  if ($correct == $N) { 
    $retval = 0;
    print " passed\n";
  } else {
    print " FAILED!\n";
  }
  return $retval;
}

sub configureXyceTestHome_test {
  my $Tools = XyceRegression::Tools->new();
  print "Self Test:  sub >>configureXyceTestHome<< ";
  debugPrint("\n");
  my $correct = 0;
  my $retval = 2;
  my @testcases = (
      [ "foo", { } ],
      [ undef, { 'XYCE_TEST_HOME' => 'bar' } ],
      [ "foo", { 'XYCE_TEST_HOME' => 'bar' } ],
      [ undef, { } ],
      );
  my @testanswers = (
      "foo",
      "bar",
      "foo",
      undef,
    );

  my $N = $#testcases+1;
  for (my $i=0 ; $i<$N ; $i++) {
    my $xyce_test = $testcases[$i]->[0];
    my %env = %{$testcases[$i]->[1]};
    debugPrint("configureXyceTestHome_test:  \$xyce_test = >>$xyce_test<<\n");
    debugPrint("configureXyceTestHome_test:  \$env{'XYCE_TEST_HOME'} = >>$env{'XYCE_TEST_HOME'}<<\n");
    my $xyce_test_home = $Tools->configureXyceTestHome($xyce_test,\%env);
    my $ans = $testanswers[$i];
    if (not defined $ans) {
      $ans = `pwd`;
      chomp($ans);
      if (not ($ans =~ s-/TestScripts$--)) {
        $ans = undef;
      }
    }
    debugPrint("configureXyceTestHome_test:  \$xyce_test_home = >>$xyce_test_home<<\n");
    debugPrint("configureXyceTestHome_test:  \$ans = >>$ans<<\n");
    if ($xyce_test_home eq $ans) {
      $correct++;
      print ".";
    } else {
      print "X";
    }
  }

  if ($correct == $N) { 
    $retval = 0;
    print " passed\n";
  } else {
    print " FAILED!\n";
  }
  return $retval;
}

sub excludeListToHash_test {
  my $Tools = XyceRegression::Tools->new();
  print "Self Test:  sub >>excludeListToHash<< ";
  debugPrint("\n");
  my $correct = 0;
  my $retval = 2;
  my @testcases = (
      [ "A.cir" ],
      [ "A A.cir" ],
      [ "A/A.cir" ],
      [ "A/A.cir","B.cir","B B.cir","B/B.cir" ],
      [ "A  B.cir" ],
      [ "A/ B.cir" ],
      );
  my @testanswers = (
      { "A.cir" => 1 },
      { "A/A.cir" => 1 },
      { "A/A.cir" => 1 },
      { "A/A.cir" => 1, "B.cir" => 1, "B/B.cir" => 1, "B/B.cir" => 1 },
      { "A/ B.cir" => 1 },
      { "A//B.cir" => 1 },
    );

  my $N = $#testcases+1;
  for (my $i=0 ; $i<$N ; $i++) {
    my @testlist = @{$testcases[$i]};
    my %testans = %{$testanswers[$i]};
    debugPrint("excludeListToHash_test:  \$testlist = >>@testlist<<\n");
    debugPrint("excludeListToHash_test:  answer hash:\n");
    for my $k (keys %testans) {
      debugPrint("excludeListToHash_test:  $k => $testans{$k}\n");
    }
    my %testresult = $Tools->excludeListToHash(@testlist);
    debugPrint("excludeListToHash_test:  result hash:\n");
    for my $k (keys %testresult) {
      debugPrint("excludeListToHash_test:  $k => $testresult{$k}\n");
    }
    if ($Tools->hashEqual(\%testresult,\%testans) eq 0) {
      $correct++;
      print ".";
    } else {
      print "X";
    }
  }

  if ($correct == $N) { 
    $retval = 0;
    print " passed\n";
  } else {
    print " FAILED!\n";
  }
  return $retval;
}

sub figureCirName_test {
  my $Tools = XyceRegression::Tools->new();
  print "Self Test:  sub >>figureCirName<< ";
  debugPrint("\n");
  my $correct = 0;
  my $retval = 2;
  my @testcases = (
      [ "A","A.cir", 20 ],
      [ "A/B/C", "C.cir", 20],
      [ "Certification_Tests/BigDirectoryName", "BigFileName.cir", 20 ],
      [ "Certification_Tests/BigDirectoryName", "BigFileName.cir", 30 ],
      [ "Certification_Tests/BigDirectoryName", "BigFileName.cir", 40 ],
      [ "Certification_Tests/BigDirectoryName", "BigFileName.cir", 50 ],
      [ "Certification_Tests/BigDirectoryName", "BigFileName.cir", 51 ],
      [ "ReallyLongDirectoryName", "A.cir", 26 ],
      [ "ReallyLongDirectoryName", "A.cir", 27 ],
      [ "ReallyLongDirectoryName", "A.cir", 28 ],
      [ "ReallyLongDirectoryName", "A.cir", 23 ],
      [ "SandiaTests/Foo", "foo.cir", 30 ],
      [ "SandiaTests/Foo", "foo.cir", 20 ],
      [ "SandiaTests/Certification_Tests/Foo", "foo.cir", 42 ],
      [ "SandiaTests/Certification_Tests/Foo", "foo.cir", 41 ],
      [ "SandiaTests/Certification_Tests/Foo", "foo.cir", 26 ],
      [ "SandiaTests/Certification_Tests/Foo", "foo.cir", 25 ],
      [ "SandiaTests/Certification_Tests/Foo", "foo.cir", 17 ],
      [ "SandiaTests/Certification_Tests/Foo", "foo.cir", 16 ],
      [ "SandiaTests/Certification_Tests/Foo", "foo.cir", 15 ],
      [ "SandiaTests/Certification_Tests/Foo", "foo.cir", 13 ],
      [ "SandiaTests/Certification_Tests/Foo", "foo.cir", 12 ],
      [ "SandiaTests/Certification_Tests/Foo", "foo.cir", 11 ],
      [ "SandiaTests/Certification_Tests/Foo", "foo.cir", 10 ],
      [ "SandiaTests/Certification_Tests/Foo", "foo.cir", 9 ],
      [ "SandiaTests/Certification_Tests/Foo", "foo.cir", 8 ],
      [ "SandiaTests/Certification_Tests/Foo", "foo.cir", 7 ],
      [ "SandiaTests/Certification_Tests/Foo", "foo.cir", 6 ],
      [ "SandiaTests/Certification_Tests/Foo", "foo.cir", 5 ],
      [ "SandiaTests/Certification_Tests/Foo", "foo.cir", 4 ],
      [ "SandiaTests/Certification_Tests/BUG_1214", "SuperLongBugName.cir", 60 ],
      [ "SandiaTests/Certification_Tests/BUG_1214", "SuperLongBugName.cir", 44 ],
      [ "SandiaTests/Certification_Tests/BUG_1214", "SuperLongBugName.cir", 31 ],
      [ "SandiaTests/Certification_Tests/BUG_1214", "SuperLongBugName.cir", 21 ],
      );
  my @testanswers = (
      "A/A.................",
      "A/B/C/C.............",
      "C_T/BigDirectory+...",
      "C_T/BigDirectoryName/BigFi+...",
      "C_T/BigDirectoryName/BigFileName........",
      "C_T/BigDirectoryName/BigFileName..................",
      "Certification_Tests/BigDirectoryName/BigFileName...",
      "ReallyLongDirectoryName...",
      "ReallyLongDirectoryName....",
      "ReallyLongDirectoryName/A...",
      "ReallyLongDirectory+...",
      "SandiaTests/Foo/foo...........",
      "ST/Foo/foo..........",
      "SandiaTests/Certification_Tests/Foo/foo...",
      "SandiaTests/C_T/Foo/foo..................",
      "SandiaTests/C_T/Foo/foo...",
      "ST/C_T/Foo/foo...........",
      "ST/C_T/Foo/foo...",
      "ST/C_T/Foo/f+...",
      "ST/C_T/Foo.....",
      "ST/C_T/Foo...",
      "ST/C_T/F+...",
      "ST/C_T/+...",
      "ST/C_T+...",
      "ST/C_+...",
      "ST/C+...",
      "ST/+...",
      "ST+...",
      "S+...",
      "+...",
      "SandiaTests/Certification_Tests/BUG_1214/SuperLongBugName...",
      "SandiaTests/C_T/BUG_1214/SuperLongBugName...",
      "ST/C_T/1214/SuperLongBugName...",
      "ST/C_T/1214/Super+...",
    );

  my $N = $#testcases+1;
  for (my $i=0 ; $i<$N ; $i++) {
    my $test_dir = $testcases[$i]->[0];
    my $test_ckt = $testcases[$i]->[1];
    my $test_col = $testcases[$i]->[2];
    my $testans = $testanswers[$i];
    debugPrint("figureCirName_test:  \$test_dir = >>$test_dir<<\n");
    debugPrint("figureCirName_test:  \$test_ckt = >>$test_ckt<<\n");
    debugPrint("figureCirName_test:  \$test_col = >>$test_col<<\n");
    debugPrint("figureCirName_test:  answer = >>$testans<<\n");
    my $testresult = $Tools->figureCirName($test_dir,$test_ckt,$test_col);
    debugPrint("figureCirName_test:  result = >>$testresult<<\n");
    if ( $testresult eq $testans ) {
      $correct++;
      print ".";
    } else {
      print "X";
    }
  }

  if ($correct == $N) { 
    $retval = 0;
    print " passed\n";
  } else {
    print " FAILED!\n";
  }
  return $retval;
}

sub parseTestList_test {
  my $Tools = XyceRegression::Tools->new();
  print "Self Test:  sub >>parseTestList<< ";
  my $correct = 0;
  my $retval = 2;
  my @testcases = (
      ["A A.cir", undef],
      ["A/A.cir", undef],
      ["A/A.cir", 1],
      [ "#stuff\nA/A.cir", undef ],
      [ "A/A.cir#stuff", undef ],
      [ "#stuff\nA/A.cir#more stuff", undef ],
      [ "\n\n\nB/B.cir\n\n\nC/C.cir\n\n", undef ],
      [ "    B/B.cir", undef ],
      [ "B/B.cir    ", undef ],
      [ "./B/B.cir", undef ],
      [ "A///B//C//foo.cir", undef ],
      [ "A B C foo.cir", undef ],
      [ "A B C/foo.cir", undef ],
      [ "A	 	 B    C 		  foo.cir", undef ],
      );
  my @testanswers = (
      [ "A A.cir" ],
      [ "A A.cir" ],
      [ "A A.cir" ],
      [ "A A.cir" ],
      [ "A A.cir" ],
      [ "A A.cir" ],
      [ "B B.cir", "C C.cir" ],
      [ "B B.cir" ],
      [ "B B.cir" ],
      [ "B B.cir" ],
      [ "A/B/C foo.cir" ],
      [ "A/B/C foo.cir" ],
      [ "A/B/C foo.cir" ],
      [ "A/B/C foo.cir" ],
    );

  my $N = $#testcases+1;
  for (my $i=0 ; $i<$N ; $i++) {
    my $teststring = $testcases[$i]->[0];
    my $testflag = $testcases[$i]->[1];
    my @testans = @{$testanswers[$i]};
    my $tmpfile = $Tools->createTempFile("$teststring");
    if ($tmpfile eq "") { `rm -f $tmpfile`; }
    my @testresult = $Tools->parseTestList($tmpfile,$testflag);
    debugPrint("parseTestList_test:  \$teststring = >>$teststring<<\n");
    debugPrint("parseTestList_test:  \$testflag = >>$testflag<<\n");
    debugPrint("parseTestList_test:  \$testanswer = >>@testans<<\n");
    debugPrint("parseTestList_test:  \$testresult = >>@testresult<<\n");
    `rm -f $tmpfile`;
    if ($Tools->setEqual(\@testans,\@testresult) eq 0) {
      $correct++;
      print ".";
    } else {
      print "X";
    }
  }

  if ($correct == $N) { 
    $retval = 0;
    print " passed\n";
  } else {
    print " FAILED!\n";
  }
  return $retval;
}

1;
