use super::util::define_pattern;
use lazy_regex::*;

define_pattern! {
    ident: ANALYZE,
    desc: "Analyze/AnalyzeShallow",
    captures: [ filepath, filename, target?, project? ],
    pattern: r"(?x)
      Analyze(?:Shallow)?\s
      # Filepath and filename
      ( ?P<filepath>.*/( ?P<filename>.*\.(?:mm|m|cc|cpp|c|cxx) ) )
      ( ?:\s.*
        \((?:
           # Target Name
           in\starget\s      '(?P<target>.*)'\s
           # Project Name
           from\sproject\s   '(?P<project>.*)'
           )\) 
      ) ?",
    tests: {
        r"AnalyzeShallow /path/to/file.m normal x86_64 (in target 'MyTarget' from project 'MyProject')" =>
            |captures| {
                assert_eq!("/path/to/file.m", &captures["filepath"]);
                assert_eq!("file.m", &captures["filename"]);
                assert_eq!("MyTarget", &captures["target"]);
                assert_eq!("MyProject", &captures["project"]);
            },
        r"AnalyzeShallow /path/to/file.c" =>
            |captures| {
                assert_eq!("/path/to/file.c", &captures["filepath"]);
                assert_eq!("file.c", &captures["filename"]);
            },
        r"Analyze /path/to/file.mm" =>
            |captures| {
                assert_eq!("/path/to/file.mm", &captures["filepath"]);
                assert_eq!("file.mm", &captures["filename"]);
            }
    }
}

define_pattern! {
    ident: BUILD_TARGET,
    desc: "BUILD TARGET",
    captures: [ target, project, configuration ],
    pattern: r"={3}\sBUILD\sTARGET\s(?P<target>.*)\sOF\sPROJECT\s(?P<project>.*)\sWITH.*CONFIGURATION\s(?P<configuration>.*)\s={3}",
    tests: {
        "=== BUILD TARGET ExampleTarget OF PROJECT ExampleProject WITH THE DEFAULT CONFIGURATION Local ===" =>
            |captures| {
                assert_eq!("ExampleTarget", &captures["target"]);
                assert_eq!("ExampleProject", &captures["project"]);
                assert_eq!("Local", &captures["configuration"]);
            }
    }
}

define_pattern! {
    ident: AGGREGATE_TARGET,
    desc: "BUILD AGGREGATE TARGET",
    captures: [ target, project, configuration ],
    pattern: r"={3}\sBUILD\sAGGREGATE\sTARGET\s(?P<target>.*)\sOF\sPROJECT\s(?P<project>.*)\sWITH.*CONFIGURATION\s(?P<configuration>.*)\s={3}",
    tests: {
        "=== BUILD AGGREGATE TARGET Example Target Name OF PROJECT AggregateTarget WITH CONFIGURATION Debug ===" =>
            |captures| {
                assert_eq!("Example Target Name", &captures["target"]);
                assert_eq!("AggregateTarget", &captures["project"]);
                assert_eq!("Debug", &captures["configuration"]);
            }
    }
}

define_pattern! {
    ident: ANALYZE_TARGET,
    desc: "ANALYZE TARGET",
    captures: [ target, project, configuration ],
    pattern: r"={3}\sANALYZE\sTARGET\s(?P<target>.*)\sOF\sPROJECT\s(?P<project>.*)\sWITH.*CONFIGURATION\s(?P<configuration>.*)\s={3}",
    tests: {
        "=== ANALYZE TARGET X OF PROJECT Y WITH THE DEFAULT CONFIGURATION Z ===" =>
            |captures| {
                assert_eq!("X", &captures["target"]);
                assert_eq!("Y", &captures["project"]);
                assert_eq!("Z", &captures["configuration"]);
            }
    }
}

/// Dependencies Check
pub static CHECK_DEPENDENCIES: Lazy<Regex> = lazy_regex!(r"Check dependencies");

define_pattern! {
    ident: SHELL_COMMAND,
    desc: "Shell commands like cd setenv under a compile step",
    captures: [ command,arguments ],
    pattern: r"\s{4}(?P<command>cd|setenv|(?:[\w/:\s\-.]+?/)?[\w\-]+)\s(?P<arguments>.*)$",
    tests: {
        "    cd /foo/bar/baz" =>
            |captures| {
                assert_eq!("cd", &captures["command"]);
                assert_eq!("/foo/bar/baz", &captures["arguments"]);
            }
    }
}

define_pattern! {
    ident: CLEAN_REMOVE,
    desc: "CLEAN REMOVE",
    captures: [ filepath, filename ],
    pattern: r"(?x)Clean.Remove\sclean\s
      # filepath and filename
      ( ?P<filepath>.*/ ( ?P<filename>.*\.(?:build) ))",
    tests: {
        "Clean.Remove clean /path/to/MyLibrary.build/Debug-iphonesimulator/MyLibraryTests.build" =>
            |captures| {
                assert_eq!("/path/to/MyLibrary.build/Debug-iphonesimulator/MyLibraryTests.build", &captures["filepath"]);
                assert_eq!("MyLibraryTests.build", &captures["filename"]);
            }
    }
}

define_pattern! {
    ident: CLEAN_TARGET,
    desc: "CLEAN TARGET",
    captures: [ target, project, configuration ],
    pattern: r"={3}\sCLEAN\sTARGET\s(?P<target>.*)\sOF\sPROJECT\s(?P<project>.*)\sWITH.*CONFIGURATION\s(?P<configuration>.*)\s={3}",
    tests: {
        "=== CLEAN TARGET X OF PROJECT Y WITH THE DEFAULT CONFIGURATION Z ===" =>
            |captures| {
                assert_eq!("X", &captures["target"]);
                assert_eq!("Y", &captures["project"]);
                assert_eq!("Z", &captures["configuration"]);
            }
    }
}

define_pattern! {
    ident: CODE_SIGN,
    desc: "CodeSign Phase",
    captures: [ filename, target, project ],
    pattern: r"CodeSign\s(:?.*/(?P<filename>.*\.(?:app)))(?:\s.*\((?:in\starget\s'(?P<target>.*)'\sfrom\sproject\s'(?P<project>.*)')\))?",
    tests: {
        "CodeSign path/to/DemoTarget.app (in target 'DemoTarget' from project 'DemoProject')" =>
            |captures| {
                assert_eq!("DemoTarget.app", &captures["filename"]);
                assert_eq!("DemoTarget", &captures["target"]);
                assert_eq!("DemoProject", &captures["project"]);
            }
    }
}

define_pattern! {
    ident: COMPILE,
    desc: r"Compile(Swift|C|\w) Step",
    captures: [ type, filename, filepath, target, project ],
    pattern: r"(?x)
        # Compile <type>
        Compile(?P<type>[\w]+)\s.+?\s
        # <filepath>
        (?P<filepath>(?:\.|[^\s])+/(?P<filename>(?:\.|[^\s])+\.(?:m|mm|c|cc|cpp|cxx|swift)))
        (?:\s.*\((?:in\starget\s'(?P<target>.*)'\sfrom\sproject\s'(?P<project>.*)')\))?",
    tests: {
        "CompileSwift normal arm64 /path/to/ToastView.swift (in target 'Example' from project 'Example')" =>
            |captures| {
                assert_eq!("Swift", &captures["type"]);
                assert_eq!("/path/to/ToastView.swift", &captures["filepath"]);
                assert_eq!("ToastView.swift", &captures["filename"]);
                assert_eq!("Example", &captures["project"]);
                assert_eq!("Example", &captures["target"]);
            },
        "CompileC /path/to/output/arm64/bridge.o /path/to/bridge.c normal arm64 c com.apple.compilers.llvm.clang.1_0.compiler (in target 'Example' from project 'Example')" =>
            |captures| {
                assert_eq!("C", &captures["type"]);
                assert_eq!("/path/to/bridge.c", &captures["filepath"]);
                assert_eq!("bridge.c", &captures["filename"]);
                assert_eq!("Example", &captures["project"]);
                assert_eq!("Example", &captures["target"]);
            }
            // "CompileAssetCatalog /output/Example.app /input/Assets.xcassets (in target 'Example' from project 'Example')" =>
            //     |captures| {
            //     assert_eq!("AssetCatalog", &captures["type"]);
            //     assert_eq!("/input/Assets.xcassets", &captures["filepath"]);
            //     assert_eq!("Assets.xcassets", &captures["filename"]);
            //     assert_eq!("Example", &captures["project"]);
            //     assert_eq!("Example", &captures["target"]);
            // }

    }
}

define_pattern! {
    ident: COMPILE_COMMAND,
    desc: r"Clang and swiftc command",
    captures: [ command, arguments ],
    pattern: r"\s{4}(:?[^\s]+/(?P<command>\w+))\s(?P<arguments>.*)",
    tests: {
        "    /TOOLCHAIN_BIN/clang -target arm64-apple-macos10.10 -r -isysroot /MACOS_SDK -L/BUILD_ROOT -L/MACOS_SDK/lib -o /BUILD_ROOT/file.o" =>
            |captures| {
                assert_eq!("clang", &captures["command"]);
                assert_eq!("-target arm64-apple-macos10.10 -r -isysroot /MACOS_SDK -L/BUILD_ROOT -L/MACOS_SDK/lib -o /BUILD_ROOT/file.o", &captures["arguments"]);
            },
        r"    /TOOLCHAIN_BIN/swiftc -incremental -module-name Example -Onone -enable-batch-mode -enforce-exclusivity\=checked -working-directory /PROJECT_ROOT" =>
            |captures| {
                assert_eq!("swiftc", &captures["command"]);
                assert_eq!(r"-incremental -module-name Example -Onone -enable-batch-mode -enforce-exclusivity\=checked -working-directory /PROJECT_ROOT", &captures["arguments"]);
            }
            // NOTE: Won't match  /TOOLCHAIN_BIN/swift-frontend -frontend -c file.swift
    }
}

define_pattern! {
    ident: COMPILE_XIB,
    desc: r"CompileXIB",
    captures: [ filename, filepath, project, target ],
    pattern: r"CompileXIB\s(?P<filepath>.*/(?P<filename>.*\.xib))(?:\s.*\((?:in\starget\s'(?P<target>.*)'\sfrom\sproject\s'(?P<project>.*)')\))?",
    tests: {
        "CompileXIB /path/to/MainMenu.xib (in target 'Example' from project 'Example')" =>
            |captures| {
                assert_eq!("/path/to/MainMenu.xib", &captures["filepath"]);
                assert_eq!("MainMenu.xib", &captures["filename"]);
                assert_eq!("Example", &captures["project"]);
                assert_eq!("Example", &captures["target"]);
            }
    }
}

define_pattern! {
    ident: COMPILE_STORYBOARD,
    desc: r"CompileStoryboard",
    captures: [ filename, filepath, project, target ],
    pattern: r"CompileStoryboard\s(?P<filepath>.*/(?P<filename>[^/].*\.storyboard))(?:\s.*\((?:in\starget\s'(?P<target>.*)'\sfrom\sproject\s'(?P<project>.*)')\))?",
    tests: {
        "CompileStoryboard /path/to/LaunchScreen.storyboard (in target 'Example' from project 'Example')" =>
            |captures| {
                assert_eq!("/path/to/LaunchScreen.storyboard", &captures["filepath"]);
                assert_eq!("LaunchScreen.storyboard", &captures["filename"]);
                assert_eq!("Example", &captures["project"]);
                assert_eq!("Example", &captures["target"]);
            }
    }
}

define_pattern! {
    ident: COPY_COMMAND,
    desc: r"CpResource|CpHeader|CopyStringsFile|CopyPlistFile",
    captures: [ type, filename, filepath, project, target ],
    pattern: r"(?x)
               (:?Cp|Copy)(?P<type>Resource|Header|PlistFile|StringsFile)\s.*\s
               (?P<filepath>.*/(?P<filename>.*\.(?:\w+)))
               (?:\s.*\((?:in\starget\s'(?P<target>.*)'\sfrom\sproject\s'(?P<project>.*)')\))?",
    tests: {
        "CpResource /output/EnWords.txt /path/to/EnWords.txt (in target 'Example' from project 'Example')" =>
            |captures| {
                assert_eq!("Resource", &captures["type"]);
                assert_eq!("/path/to/EnWords.txt", &captures["filepath"]);
                assert_eq!("EnWords.txt", &captures["filename"]);
                assert_eq!("Example", &captures["project"]);
                assert_eq!("Example", &captures["target"]);
            },
        "CpHeader /output/file.h /path/to/file.h (in target 'Example' from project 'Example')" =>
            |captures| {
                assert_eq!("Header", &captures["type"]);
                assert_eq!("/path/to/file.h", &captures["filepath"]);
                assert_eq!("file.h", &captures["filename"]);
                assert_eq!("Example", &captures["project"]);
                assert_eq!("Example", &captures["target"]);
            },
         "CopyStringsFile /output/InfoPlist.strings path/to/en.lproj/InfoPlist.strings (in target 'Example' from project 'Example')" => |captures| {
                assert_eq!("StringsFile", &captures["type"]);
                assert_eq!("path/to/en.lproj/InfoPlist.strings", &captures["filepath"]);
                assert_eq!("InfoPlist.strings", &captures["filename"]);
                assert_eq!("Example", &captures["project"]);
                assert_eq!("Example", &captures["target"]);

            }
    }
}

define_pattern! {
    ident: TEST_EXECUTED,
    desc: r"Executed number of tests",
    captures: [ tests_count, failed_tests_count, unexpected_test_count, total_exec_time ],
    pattern: r"(?x)\s*Executed\s
        (?P<tests_count>\d+)\stest[s]?,\swith\s
        (?P<failed_tests_count>\d+)\sfailure[s]?\s
        \((?P<unexpected_test_count>\d+)\sunexpected\)\sin\s\d+\.\d{3}\s
        \((?P<total_exec_time>\d+\.\d{3})\)\sseconds",
    tests: {
        "     Executed 3 tests, with 1 failure (0 unexpected) in 0.258 (0.259) seconds" =>
            |captures| {
                assert_eq!("3", &captures["tests_count"]);
                assert_eq!("1", &captures["failed_tests_count"]);
                assert_eq!("0", &captures["unexpected_test_count"]);
                assert_eq!("0.259", &captures["total_exec_time"]);
            },
        "Executed 4 tests, with 0 failures (0 unexpected) in 0.003 (0.004) seconds" =>
            |captures| {
                assert_eq!("4", &captures["tests_count"]);
                assert_eq!("0", &captures["failed_tests_count"]);
                assert_eq!("0", &captures["unexpected_test_count"]);
                assert_eq!("0.004", &captures["total_exec_time"]);
            }
    }
}

define_pattern! {
    ident: TEST_EXECUTED_WITH_SKIPPED,
    desc: r"Executed number of tests with skipped teats",
    captures: [ tests_count, skipped_test_count, failed_tests_count, unexpected_test_count, total_exec_time ],
    pattern: r"(?x)
        \s*Executed\s
        (?P<tests_count>\d+)\stest[s]?,\swith\s
        (?P<skipped_test_count>\d+)\stest[s]?\sskipped\sand\s
        (?P<failed_tests_count>\d+)\sfailure[s]?\s
        \((?P<unexpected_test_count>\d+)\sunexpected\)\sin\s\d+\.\d{3}\s
        \((?P<total_exec_time>\d+\.\d{3})\)\sseconds",
    tests: {
        "    Executed 56 tests, with 3 test skipped and 2 failures (1 unexpected) in 1.029 (1.029) seconds" =>
            |captures| {
                assert_eq!("56", &captures["tests_count"]);
                assert_eq!("3", &captures["skipped_test_count"]);
                assert_eq!("2", &captures["failed_tests_count"]);
                assert_eq!("1", &captures["unexpected_test_count"]);
                assert_eq!("1.029", &captures["total_exec_time"]);
            },
        "Executed 1 test, with 1 test skipped and 1 failure (1 unexpected) in 3.000 (3.000) seconds" =>
            |captures| {
                assert_eq!("1", &captures["tests_count"]);
                assert_eq!("1", &captures["skipped_test_count"]);
                assert_eq!("1", &captures["failed_tests_count"]);
                assert_eq!("1", &captures["unexpected_test_count"]);
                assert_eq!("3.000", &captures["total_exec_time"]);
            }
    }
}

define_pattern! {
    ident: KIWI_FAILING_TEST,
    desc: r"Executed number of tests with skipped teats",
    captures: [ filepath, suite, case, reason ],
    pattern: r"(?x)\s*
        (?P<filepath>.+:\d+):\serror:\s[\+\-]
        \[
          (?P<suite>.*)\s
          (?P<case>.*)
         \]\s:(?:\s'.*'\s\[FAILED\],)?\s
        (?P<reason>.*)",
    tests: {
        "/path/to/tests.m:49: error: -[TestSuite TestCase] : 'Iterators, times： iterates the exact number of times' [FAILED], expected subject to equal 4, got 5" =>
            |captures| {
                assert_eq!("/path/to/tests.m:49", &captures["filepath"]);
                assert_eq!("TestSuite", &captures["suite"]);
                assert_eq!("TestCase", &captures["case"]);
                assert_eq!("expected subject to equal 4, got 5", &captures["reason"]);
            }
    }
}

define_pattern! {
    ident: UI_FAILING_TEST,
    desc: r"UI Test failing",
    captures: [ filepath, reason ],
    pattern: r"\s*t = \s+\d+\.\d+s\s+Assertion Failure: (?P<filepath>.*:\d+): (?P<reason>.*)$",
    tests: {
        "t =    22.27s             Assertion Failure: <unknown>:0: UI Testing Failure - Unable to find hit point for element Button 0x608001165880: {{74.0, -54.0}, {44.0, 38.0}}, label: 'Disconnect'" =>
            |captures| {
                assert_eq!("<unknown>:0", &captures["filepath"]);
                assert_eq!("UI Testing Failure - Unable to find hit point for element Button 0x608001165880: {{74.0, -54.0}, {44.0, 38.0}}, label: 'Disconnect'", &captures["reason"]);
            }
    }
}

/// Restarting tests
pub static RESTARTING_TESTS: Lazy<Regex> = lazy_regex!(r"Restarting after unexpected exit.+$");

/// Coverage Data Generation
pub static COVERAGE_DATA_GENERATION: Lazy<Regex> = lazy_regex!(r"generating\s+coverage\s+data\.*");

define_pattern! {
    ident: COVERAGE_REPORT_GENERATION,
    desc: r"Coverage report generation",
    captures: [ filepath ],
    pattern: r"(?i)generated\s+coverage\s+report:\s+(?P<filepath>.+)",
    tests: {
        "Generated coverage report: /path/to/code coverage.xccovreport" =>
            |captures| {
                assert_eq!("/path/to/code coverage.xccovreport", &captures["filepath"]);
            }
    }
}

define_pattern! {
    ident: DSYM_GENERATION,
    desc: r"GenerateDSYMFile",
    captures: [ filename, target, project ],
    pattern: r"(?x)
        GenerateDSYMFile\s/.*/
        (?P<filename>.*\.dSYM)\s/.*
        \((?:in\starget\s'(?P<target>.*)'\sfrom\sproject\s'(?P<project>.*)')\)?",
    tests: {
        "GenerateDSYMFile /$BUILD/Release/DemoTarget.app.dSYM /$BUILD/Release/DemoTarget.app/Contents/MacOS/DemoTarget (in target 'DemoTarget' from project 'DemoProject')" =>
            |captures| {
                assert_eq!("DemoTarget.app.dSYM", &captures["filename"]);
                assert_eq!("DemoTarget", &captures["target"]);
                assert_eq!("DemoProject", &captures["project"]);
            }
    }
}

define_pattern! {
    ident: LINKING,
    desc: r"Ld",
    captures: [ filename, target, project ],
    pattern: r"Ld\s(?P<filepath>.*/(?P<filename>\w+\.\w+)).*\((?:in\starget\s'(?P<target>.*)'\sfrom\sproject\s'(?P<project>.*)')\)?",
    tests: {
        "Ld /path/to/file.o normal x86_64 (in target 'Example' from project 'Example')" =>
            |captures| {
                assert_eq!("/path/to/file.o", &captures["filepath"]);
                assert_eq!("file.o", &captures["filename"]);
                assert_eq!("Example", &captures["target"]);
                assert_eq!("Example", &captures["project"]);
            }
    }
}

define_pattern! {
    ident: OCUNIT_TEST_CASE_STARTED,
    desc: r"Test Case Started",
    captures: [ suite, case],
    pattern: r"\s*Test Case '-\[(?P<suite>.*) (?P<case>.*)\]' started.$",
    tests: {
        "Test Case '-[viewUITests.vmtAboutWindow testConnectToDesktop]' started." =>
            |captures| {
                assert_eq!("viewUITests.vmtAboutWindow", &captures["suite"]);
                assert_eq!("testConnectToDesktop", &captures["case"]);
            }
    }
}

define_pattern! {
    ident: OCUNIT_TEST_CASE_PASSED,
    desc: r"Test Case Passed",
    captures: [ suite, case, time ],
    pattern: r"\s*Test Case\s'-\[(?P<suite>.*)\s(?P<case>.*)\]'\spassed\s\((?P<time>\d*\.\d{3})\sseconds\).",
    tests: {
        "Test Case '-[TestSuite TestCase]' passed (0.001 seconds)." =>
            |captures| {
                assert_eq!("TestSuite", &captures["suite"]);
                assert_eq!("TestCase", &captures["case"]);
                assert_eq!("0.001", &captures["time"]);
            }
    }
}

define_pattern! {
    ident: KIWI_TEST_CASE_PENDING,
    desc: r"Ld",
    captures: [ suite, case ],
    pattern: r"Test Case\s'-\[(?P<suite>.*)\s(?P<case>.*)PENDING\]'\spassed",
    tests: {
        "Test Case '-[TestSuite TestCasePENDING]' passed (0.001 seconds)." =>
            |captures| {
                assert_eq!("TestSuite", &captures["suite"]);
                assert_eq!("TestCase", &captures["case"]);
            }
    }
}

define_pattern! {
    ident: TEST_CASE_MEASURE,
    desc: r"Test case measuring",
    captures: [ suite, case, time ],
    pattern: r"[^:]*:[^:]*:\sTest Case\s'-\[(?P<suite>.*)\s(?P<case>.*)\]'\smeasured\s\[Time,\sseconds\]\saverage:\s(?P<time>\d*\.\d{3})(.*){4}",
    tests: {
        r#"<unknown>:0: Test Case '-[TestSuite TestCase]' measured [Time, seconds] average: 0.013, relative standard deviation: 26.773%, values: [0.023838, 0.012034, ], performanceMetricID:com.apple.XCTPerformanceMetric_WallClockTime, baselineName: "", baselineAverage: , maxPercentRegression: 10.000%, maxPercentRelativeStandardDeviation: 10.000%, maxRegression: 0.100, maxStandardDeviation: 0.100"# =>
            |captures| {
                assert_eq!("TestSuite", &captures["suite"]);
                assert_eq!("TestCase", &captures["case"]);
                assert_eq!("0.013", &captures["time"]);
            }
    }
}

define_pattern! {
    ident: PARALLEL_TEST_CASE_PASSED,
    desc: r"Parallel TestCase passed",
    captures: [ suite, case, time, medium ],
    pattern: r"Test\s+case\s+'(?P<suite>.*)\.(?P<case>.*)\(\)'\s+passed\s+on\s+'(?P<medium>.*)'\s+\((?P<time>\d*\.(.*){3})\s+seconds\)",
    tests: {
        "Test case 'TestSuite.testCase()' passed on 'xctest (49438)' (0.131 seconds)" =>
            |captures| {
                assert_eq!("TestSuite", &captures["suite"]);
                assert_eq!("testCase", &captures["case"]);
                assert_eq!("0.131", &captures["time"]);
                assert_eq!("xctest (49438)", &captures["medium"]);
            }
    }
}

define_pattern! {
    ident: PARALLEL_TEST_CASE_APPKIT_PASSED,
    desc: r"Parallel TestCase AppKit Passed",
    captures: [ suite, case, time, medium ],
    pattern: r"\s*Test case\s'-\[(?P<suite>.*)\s(?P<case>.*)\]'\spassed\son\s'(?P<medium>.*)'\s\((?P<time>\d*\.\d{3})\sseconds\)",
    tests: {
        "Test case '-[TestSuite testCase]' passed on 'xctest (49438)' (0.131 seconds)." =>
            |captures| {
                assert_eq!("TestSuite", &captures["suite"]);
                assert_eq!("testCase", &captures["case"]);
                assert_eq!("xctest (49438)", &captures["medium"]);
                assert_eq!("0.131", &captures["time"]);
            }
    }
}

define_pattern! {
    ident: PARALLEL_TEST_CASE_FAILED,
    desc: r"Parallel TestCase Failed",
    captures: [ suite, case, time, medium ],
    pattern: r"Test\s+case\s+'(?P<suite>.*)\.(?P<case>.*)\(\)'\s+failed\s+on\s+'(?P<medium>.*)'\s+\((?P<time>\d*\.(.*){3})\s+seconds\)",
    tests: {
        "Test case 'TestSuite.testCase()' failed on 'iPhone 11' (7.158 seconds)" =>
            |captures| {
                assert_eq!("TestSuite", &captures["suite"]);
                assert_eq!("testCase", &captures["case"]);
                assert_eq!("iPhone 11", &captures["medium"]);
                assert_eq!("7.158", &captures["time"]);
            }
    }
}

define_pattern! {
    ident: PARALLEL_TESTING_STARTED,
    desc: r"Parallel Testing Started",
    captures: [ suite, case, time, medium ],
    pattern: r"Testing\s+started\s+on\s+'(?P<medium>.*)'",
    tests: {
        "Testing started on 'iPhone X'" =>
            |captures| {
                assert_eq!("iPhone X", &captures["medium"]);
            }
    }
}

define_pattern! {
    ident: PARALLEL_TESTING_PASSED,
    desc: r"Parallel Testing Passed",
    captures: [ suite, case, time, medium ],
    pattern: r"Testing\s+passed\s+on\s+'(?P<medium>.*)'",
    tests: {
        "Testing passed on 'iPhone X'" =>
            |captures| {
                assert_eq!("iPhone X", &captures["medium"]);
            }
    }
}

define_pattern! {
    ident: PARALLEL_TESTING_FAILED,
    desc: r"Parallel Testing Failed",
    captures: [ suite, case, time, medium ],
    pattern: r"Testing\s+failed\s+on\s+'(?P<medium>.*)'",
    tests: {
        "Testing failed on 'iPhone X'" =>
            |captures| {
                assert_eq!("iPhone X", &captures["medium"]);
            }
    }
}

define_pattern! {
    ident: PARALLEL_TEST_FAILED,
    desc: r"Parallel Testing Failed",
    captures: [ suite, case, time, medium ],
    pattern: r"(?i)\s*Test\s+Suite\s+'(?P<suite>.*)'\s+started\s+on\s+'(?P<medium>.*)'",
    tests: {
        "Test suite 'TestSuite (iOS).xctest' started on 'iPhone X'" =>
            |captures| {
                assert_eq!("TestSuite (iOS).xctest", &captures["suite"]);
                assert_eq!("iPhone X", &captures["medium"]);
            }
    }
}

/// Dependencies Check
pub static CHECK_DEPENDENCIES: Lazy<Regex> = lazy_regex!(r"Check dependencies");

/// Restarting tests
pub static RESTARTING_TESTS: Lazy<Regex> = lazy_regex!(r"Restarting after unexpected exit.+$");

/// Coverage Data Generation
pub static COVERAGE_DATA_GENERATION: Lazy<Regex> = lazy_regex!(r"generating\s+coverage\s+data\.*");

/// Coverage Data Generation
pub static PHASE_SUCCESS: Lazy<Regex> = lazy_regex!(r"\*\*\s(.*)\sSUCCEEDED\s\*\*");

define_pattern! {
    ident: PHASE_SCRIPT_EXECUTION,
    desc: r"PhaseScriptExecution",
    captures: [ name, target, project ],
    pattern: r"(?x)PhaseScriptExecution\s(?P<name>.*)\s/.*\.sh( ?:\s.* \((?:in\starget\s      '(?P<target>.*)'\s  from\sproject\s   '(?P<project>.*)' )\)  ) ?",
    tests: {
        "PhaseScriptExecution Format\\ Swift\\ Files /path/to/file.sh (in target 'DemoTarget' from project 'DemoProject')" =>
            |captures| {
                assert_eq!("Format\\ Swift\\ Files", &captures["name"]);
                assert_eq!("DemoTarget", &captures["target"]);
                assert_eq!("DemoProject", &captures["project"]);
            },
        "PhaseScriptExecution [CP]\\ Check\\ Pods\\ Manifest.lock /path/to/file.sh (in target 'App' from project 'App')" =>
            |captures| {
                assert_eq!("[CP]\\ Check\\ Pods\\ Manifest.lock", &captures["name"]);
                assert_eq!("App", &captures["target"]);
                assert_eq!("App", &captures["project"]);
            }

    }
}

define_pattern! {
    ident: PROCESS_PCH,
    desc: r"ProcessPCH",
    captures: [ filename, target, project ],
    pattern: r"(?x)ProcessPCH(?:\+\+)?\s.*\s/.*/(?P<filename>.*.pch)( ?:\s.* \((?:in\starget\s      '(?P<target>.*)'\s  from\sproject\s   '(?P<project>.*)' )\)  ) ?",
    tests: {
        "ProcessPCH /path/to/file.pch.gch /path/to/file.pch normal x86_64 objective-c com.apple.compilers.llvm.clang.1_0.analyzer (in target 'App' from project 'App')" =>
            |captures| {
                assert_eq!("file.pch", &captures["filename"]);
                assert_eq!("App", &captures["target"]);
                assert_eq!("App", &captures["project"]);
            }
    }
}

define_pattern! {
    ident: PROCESS_PCH_COMMAND,
    desc: r"ProcessPchCommand",
    captures: [ ],
    pattern: r"\s*.*/usr/bin/clang\s.*\s\-c\s(.*.pch)\s.*\-o\s.*",
    tests: { }
}

define_pattern! {
    ident: PBX_COPY,
    desc: r"PBXCp",
    captures: [ filename, target, project ],
    pattern: r"(?x)PBXCp\s(?P<filepath>/.*)\s/.*\((?:in\starget\s'(?P<target>.*)'\sfrom\sproject\s'(?P<project>.*)')\)",
    tests: {
        "PBXCp /path/to/header.h /path/to/output.h (in target 'App' from project 'App')" =>
            |captures| {
                assert_eq!("App" ,&captures["target"]);
                assert_eq!("App" ,&captures["project"]);
                assert_eq!("/path/to/header.h" ,&captures["filepath"]);
            }
    }
}

define_pattern! {
    ident: PROCESS_INFO_PLIST_FILE,
    desc: r"ProcessInfoPlistFile",
    captures: [ filename, filepath, target, project ],
    pattern: r"(?x)ProcessInfoPlistFile\s.*\s
        (?P<filepath>/(?:\.|[^\s])+/(?P<filename>(?:\.|[^\s])+\.(?:plist)))
        (?:\s.*\((?:in\starget\s'(?P<target>.*)'\sfrom\sproject\s'(?P<project>.*)')\))?",
    tests: {
        "ProcessInfoPlistFile /path/to/output/Info.plist /path/to/Info.plist (in target 'Example' from project 'Example')" =>
            |captures| {
                assert_eq!("/path/to/Info.plist", &captures["filepath"]);
                assert_eq!("Info.plist", &captures["filename"]);
                assert_eq!("Example", &captures["project"]);
                assert_eq!("Example", &captures["target"]);
            }


    }
}

