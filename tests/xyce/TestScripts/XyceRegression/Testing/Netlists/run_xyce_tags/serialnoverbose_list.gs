#run_xyce/run_xyce.cir                            # exclude in "tags"
noverbose/noverbose.cir                          # serial,nightly,required:noverbose
#SerialAndParallel/serial.cir                     # .tags: serial,nightly (exclude in "tags")
#SerialAndParallel/parallel.cir                   # .tags: mpich,mpilam,nightly (exclude in "tags")
#SerialAndParallel/serialandparallel.cir          # .tags: serial,mpich,mpilam,nightly (exclude in "tags")
#klu/klu.cir                                      # serial,mpich,mpilam,nightly,klu
Certification_Tests/A/a.cir                      # no tags file
#Certification_Tests/Bserialnightly/b.cir         # serial,nightly
#Certification_Tests/Cparallelnightly/c.cir       # nightly,mpich,mpilam
#Certification_Tests/Dserialparallelweekly/d.cir  # weekly,serial,mpich,mpilam
#windows/windows.cir                              # serial,nightly,required:windows
#valgrind/valgrind.cir                            # serial,nightly,valgrind
#stress_tests/AA/aa.cir                           # exclude=aa.cir serial,nightly in "tags"
#stress_tests/BB/bb.cir                           # exclude=bb.cir
#stress_tests/CC/cc.cir                           # exclude=cc.cir
#stress_tests/DD/dd.cir                           # exclude in "tags"
#dakota/dakota.cir                                # serial,nightly,required:dakota
#icc/icc.cir                                      # mpich,required:icc
#NoTags/serialnightly.cir                         # .tags: serial,nightly (no tags file)
#NoTags/serialweekly.cir                          # .tags: serial,weekly (no tags file)
#exclude/include.cir                              # serial,nightly in "tags"
#exclude/exclude.cir                              # exclude=exclude.cir  ( serial,nightly in "tags")
#exclude/includelocal.cir                         # weekly,mpich,mpilam ( serial,nightly in "tags")
#OUOTests/radAnightly/radA.cir                    # serial,nightly,mpich,mpilam,rad in "tags"
#OUOTests/radBweekly/radB.cir                     # serial,weekly,mpich,mpilam,rad in "tags"
OUOTests/Certification_Tests/E/e.cir             # no tags
OUOTests/Certification_Tests/F/f.cir             # no tags
OUOTests/Certification_Tests/G/g.cir             # no tags
OUOTests/Certification_Tests/H/h.cir             # no tags
#OUOTests/stress_tests/EE/ee.cir                  # exclude in "tags"
#OUOTests/stress_tests/FF/ff.cir                  # exclude in "tags"
#OUOTests/stress_tests/GG/gg.cir                  # exclude in "tags"
#OUOTests/stress_tests/HH/hh.cir                  # exclude in "tags"

