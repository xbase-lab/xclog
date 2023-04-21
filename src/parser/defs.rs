use super::gen::define;

define! [
{
    ident: Analyze,
    kind: Task,
    desc: "Analyze/AnalyzeShallow",
    captures: [ filepath, filename, target, project ],
    format: "[{target}] Analyzing {filename}",
    pattern: r"(?x)
        Analyze(?:Shallow)?\s
        # Filepath and filename
        ( ?P<filepath>.*/( ?P<filename>.*\.(?:mm|m|cc|cpp|c|cxx) ) )
        ( ?:\s.* \((?:in\starget\s      '(?P<target>.*)'\s  from\sproject\s   '(?P<project>.*)' )\)  ) ?",
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
},
{
    ident: BuildTarget,
    kind: Task,
    desc: "BUILD TARGET",
    captures: [ target, project, configuration ],
    format: "[{target}] Build with {configuration}",
    pattern: r"={3}\sBUILD\sTARGET\s(?P<target>.*)\sOF\sPROJECT\s(?P<project>.*)\sWITH.*CONFIGURATION\s(?P<configuration>.*)\s={3}",
    tests: {
        "=== BUILD TARGET ExampleTarget OF PROJECT ExampleProject WITH THE DEFAULT CONFIGURATION Local ===" =>
            |captures| {
                assert_eq!("ExampleTarget", &captures["target"]);
                assert_eq!("ExampleProject", &captures["project"]);
                assert_eq!("Local", &captures["configuration"]);
            }
    }
},
{
    ident: AggregateTarget,
    kind: Task,
    desc: "BUILD AGGREGATE TARGET",
    captures: [ target, project, configuration ],
    format: "[{target}] Aggregate Build with {configuration}",
    pattern: r"={3}\sBUILD\sAGGREGATE\sTARGET\s(?P<target>.*)\sOF\sPROJECT\s(?P<project>.*)\sWITH.*CONFIGURATION\s(?P<configuration>.*)\s={3}",
    tests: {
        "=== BUILD AGGREGATE TARGET Example Target Name OF PROJECT AggregateTarget WITH CONFIGURATION Debug ===" =>
            |captures| {
                assert_eq!("Example Target Name", &captures["target"]);
                assert_eq!("AggregateTarget", &captures["project"]);
                assert_eq!("Debug", &captures["configuration"]);
            }
    }
},
{
    ident: AnalyzeTarget,
    kind: Task,
    desc: "ANALYZE TARGET",
    captures: [ target, project, configuration ],
    format: "[{target}] Analyze with {configuration}",
    pattern: r"={3}\sANALYZE\sTARGET\s(?P<target>.*)\sOF\sPROJECT\s(?P<project>.*)\sWITH.*CONFIGURATION\s(?P<configuration>.*)\s={3}",
    tests: {
        "=== ANALYZE TARGET X OF PROJECT Y WITH THE DEFAULT CONFIGURATION Z ===" =>
            |captures| {
                assert_eq!("X", &captures["target"]);
                assert_eq!("Y", &captures["project"]);
                assert_eq!("Z", &captures["configuration"]);
            }
    }
},
{
    ident: CleanRemove,
    kind: Task,
    desc: "CLEAN REMOVE",
    captures: [ filepath, filename ],
    format: "Cleaning {filename}",
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
},
{
    ident: CleanTarget,
    kind: Task,
    desc: "CLEAN TARGET",
    captures: [ target, project, configuration ],
    format: "[{target}] Clean with {configuration}",
    pattern: r"={3}\sCLEAN\sTARGET\s(?P<target>.*)\sOF\sPROJECT\s(?P<project>.*)\sWITH.*CONFIGURATION\s(?P<configuration>.*)\s={3}",
    tests: {
        "=== CLEAN TARGET X OF PROJECT Y WITH THE DEFAULT CONFIGURATION Z ===" =>
            |captures| {
                assert_eq!("X", &captures["target"]);
                assert_eq!("Y", &captures["project"]);
                assert_eq!("Z", &captures["configuration"]);
            }
    }
},
{
    ident: CodeSign,
    kind: Task,
    desc: "CodeSign Phase",
    captures: [ filename, target, project ],
    format: "[{target}] Signing {filename}",
    pattern: r"CodeSign\s(:?.*/(?P<filename>.*\.(?:app)))(?:\s.*\((?:in\starget\s'(?P<target>.*)'\sfrom\sproject\s'(?P<project>.*)')\))?",
    tests: {
        "CodeSign path/to/DemoTarget.app (in target 'DemoTarget' from project 'DemoProject')" =>
            |captures| {
                assert_eq!("DemoTarget.app", &captures["filename"]);
                assert_eq!("DemoTarget", &captures["target"]);
                assert_eq!("DemoProject", &captures["project"]);
            }
    }
},
{
    ident: Compile,
    kind: Task,
    desc: r"Compile(Swift|C|\w) Step",
    captures: [ kind, filename, filepath, target, project ],
    format: "[{target}] Compiling {filename}",
    pattern: r"(?x)
        # Compile <kind>
        Compile(?P<kind>[\w]+)\s.+?\s
        # <filepath>
        (?P<filepath>(?:\.|[^\s])+/(?P<filename>(?:\.|[^\s])+\.(?:mm|m|cpp|cxx|cc|c|swift)))
        (?:\s.*\((?:in\starget\s'(?P<target>.*)'\sfrom\sproject\s'(?P<project>.*)')\))?",
    tests: {
        "CompileSwift normal arm64 /path/to/ToastView.swift (in target 'Example' from project 'Example')" =>
            |captures| {
                assert_eq!("Swift", &captures["kind"]);
                assert_eq!("/path/to/ToastView.swift", &captures["filepath"]);
                assert_eq!("ToastView.swift", &captures["filename"]);
                assert_eq!("Example", &captures["project"]);
                assert_eq!("Example", &captures["target"]);
            },
        "CompileC /path/to/output/arm64/bridge.o /path/to/bridge.c normal arm64 c com.apple.compilers.llvm.clang.1_0.compiler (in target 'Example' from project 'Example')" =>
            |captures| {
                assert_eq!("C", &captures["kind"]);
                assert_eq!("/path/to/bridge.c", &captures["filepath"]);
                assert_eq!("bridge.c", &captures["filename"]);
                assert_eq!("Example", &captures["project"]);
                assert_eq!("Example", &captures["target"]);
            },
        "CompileC /Users/xpeng/Library/Developer/Xcode/DerivedData/ToScanner-ftdaekkkwvklasgvfxoiqxewrlzg/Build/Intermediates.noindex/Pods.build/Debug-iphonesimulator/FlexLayout.build/Objects-normal/arm64/UIView+Yoga.o /Users/xpeng/Project/ToScanner/Pods/FlexLayout/Sources/YogaKit/UIView+Yoga.mm normal arm64 objective-c++ com.apple.compilers.llvm.clang.1_0.compiler (in target 'FlexLayout' from project 'Pods')" =>
            |captures| {
                assert_eq!("C", &captures["kind"]);
                assert_eq!("/Users/xpeng/Project/ToScanner/Pods/FlexLayout/Sources/YogaKit/UIView+Yoga.mm", &captures["filepath"]);
                assert_eq!("UIView+Yoga.mm", &captures["filename"]);
                assert_eq!("Pods", &captures["project"]);
                assert_eq!("FlexLayout", &captures["target"]);
        }
    }
},
{
    ident: SwiftDriver,
    kind: Task,
    desc: r"Matches Swift driver step",
    captures: [kind, target, project],
    format: "[{target}] using swift deriver",
    pattern: r"(?x)
        # Swift driver step
        (?P<kind>\w+)Driver\s(?P<target>[\w]+)\s.+?
        # (in target '<target>' from project '<project>')
        \(in\starget\s'(.*)'\sfrom\sproject\s'(?P<project>.*)'\)",
    tests: {
        "SwiftDriver XX normal arm64 com.apple.xcode.tools.swift.compiler (in target 'XX' from project 'XX')" =>
            |captures| {
                assert_eq!("Swift", &captures["kind"]);
                assert_eq!("XX", &captures["target"]);
                assert_eq!("XX", &captures["project"]);
            }
    }
},
{
    ident: CompileCommand,
    kind: Task,
    desc: r"Clang and swiftc command",
    captures: [ command, name, arguments ],
    format: "",
    pattern: r"^\s{4}(builtin-SwiftDriver -- )?(?P<command>[^\s]+/(?P<name>swift-frontend|swiftc|clang\+\+|clang))\s(?P<arguments>.*)",
    tests: {
        "    /TOOLCHAIN_BIN/clang -target arm64-apple-macos10.10 -r -isysroot /MACOS_SDK -L/BUILD_ROOT -L/MACOS_SDK/lib -o /BUILD_ROOT/file.o" =>
            |captures| {
                assert_eq!("/TOOLCHAIN_BIN/clang", &captures["command"]);
                assert_eq!("-target arm64-apple-macos10.10 -r -isysroot /MACOS_SDK -L/BUILD_ROOT -L/MACOS_SDK/lib -o /BUILD_ROOT/file.o", &captures["arguments"]);
            },
        r"    /TOOLCHAIN_BIN/swiftc -incremental -module-name Example -Onone -enable-batch-mode -enforce-exclusivity\=checked -working-directory /PROJECT_ROOT" =>
            |captures| {
                assert_eq!("/TOOLCHAIN_BIN/swiftc", &captures["command"]);
                assert_eq!(r"-incremental -module-name Example -Onone -enable-batch-mode -enforce-exclusivity\=checked -working-directory /PROJECT_ROOT", &captures["arguments"]);
            },
        r"    builtin-SwiftDriver -- /TOOLCHAIN_BIN/swiftc -module-name XX -Onone -enforce-exclusivity\=checked" =>
            |captures| {
                assert_eq!("/TOOLCHAIN_BIN/swiftc", &captures["command"]);
                assert_eq!(r"-module-name XX -Onone -enforce-exclusivity\=checked", &captures["arguments"]);
            },
        r"    /TOOLCHAIN_BIN/swift-frontend -c /source/HotReloaderMiddleware.swift /source/Extensions/UIApplication.swift" =>  |captures| {
                assert_eq!("/TOOLCHAIN_BIN/swift-frontend", &captures["command"]);

            },
        r"    /TOOLCHAIN_BIN/swiftc -incremental -module-name Logging -Onone -enable-batch-mode -enforce-exclusivity\=checked @/BUILD_ROOT/swift-log.build/Debug-iphoneos/Logging.build/Objects-normal/armv7/Logging.SwiftFileList -DSWIFT_PACKAGE -DDEBUG -DXcode -sdk /IPHONE_SDK -target armv7-apple-ios9.0 -g -Xfrontend -serialize-debugging-options -embed-bitcode-marker -enable-testing -swift-version 5 -I /BUILD_ROOT -I /Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/usr/lib -F /BUILD_ROOT -F /Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/Library/Frameworks -F /IPHONE_SDK/Developer/Library/Frameworks -c -j8 -output-file-map /BUILD_ROOT/swift-log.build/Debug-iphoneos/Logging.build/Objects-normal/armv7/Logging-OutputFileMap.json -parseable-output -serialize-diagnostics -emit-dependencies -emit-module -emit-module-path /BUILD_ROOT/swift-log.build/Debug-iphoneos/Logging.build/Objects-normal/armv7/Logging.swiftmodule -Xcc -I/BUILD_ROOT/swift-log.build/Debug-iphoneos/Logging.build/swift-overrides.hmap -Xcc -I/BUILD_ROOT/include -Xcc -I/BUILD_ROOT/swift-log.build/Debug-iphoneos/Logging.build/DerivedSources-normal/armv7 -Xcc -I/BUILD_ROOT/swift-log.build/Debug-iphoneos/Logging.build/DerivedSources/armv7 -Xcc -I/BUILD_ROOT/swift-log.build/Debug-iphoneos/Logging.build/DerivedSources -Xcc -DSWIFT_PACKAGE -Xcc -DDEBUG\=1 -emit-objc-header -emit-objc-header-path /BUILD_ROOT/swift-log.build/Debug-iphoneos/Logging.build/Objects-normal/armv7/Logging-Swift.h -working-directory /DERIVED_DATA_ROOT/SourcePackages/checkouts/swift-log"
            => |captures| {
                assert_eq!("/TOOLCHAIN_BIN/swiftc", &captures["command"]);
                assert_eq!(r"-incremental -module-name Logging -Onone -enable-batch-mode -enforce-exclusivity\=checked @/BUILD_ROOT/swift-log.build/Debug-iphoneos/Logging.build/Objects-normal/armv7/Logging.SwiftFileList -DSWIFT_PACKAGE -DDEBUG -DXcode -sdk /IPHONE_SDK -target armv7-apple-ios9.0 -g -Xfrontend -serialize-debugging-options -embed-bitcode-marker -enable-testing -swift-version 5 -I /BUILD_ROOT -I /Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/usr/lib -F /BUILD_ROOT -F /Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/Library/Frameworks -F /IPHONE_SDK/Developer/Library/Frameworks -c -j8 -output-file-map /BUILD_ROOT/swift-log.build/Debug-iphoneos/Logging.build/Objects-normal/armv7/Logging-OutputFileMap.json -parseable-output -serialize-diagnostics -emit-dependencies -emit-module -emit-module-path /BUILD_ROOT/swift-log.build/Debug-iphoneos/Logging.build/Objects-normal/armv7/Logging.swiftmodule -Xcc -I/BUILD_ROOT/swift-log.build/Debug-iphoneos/Logging.build/swift-overrides.hmap -Xcc -I/BUILD_ROOT/include -Xcc -I/BUILD_ROOT/swift-log.build/Debug-iphoneos/Logging.build/DerivedSources-normal/armv7 -Xcc -I/BUILD_ROOT/swift-log.build/Debug-iphoneos/Logging.build/DerivedSources/armv7 -Xcc -I/BUILD_ROOT/swift-log.build/Debug-iphoneos/Logging.build/DerivedSources -Xcc -DSWIFT_PACKAGE -Xcc -DDEBUG\=1 -emit-objc-header -emit-objc-header-path /BUILD_ROOT/swift-log.build/Debug-iphoneos/Logging.build/Objects-normal/armv7/Logging-Swift.h -working-directory /DERIVED_DATA_ROOT/SourcePackages/checkouts/swift-log", &captures["arguments"]);
        }

    }
},
{
    ident: OtherCompileCommands,
    kind: Task,
    desc: r"SwiftFrontend Compile Commands",
    captures: [ ],
    format: "",
    pattern: r"^\s{4}/[^\s]+.*\s.*",
    tests: { }
},
{
    ident: CompileXIB,
    kind: Task,
    desc: r"CompileXIB",
    captures: [ filename, filepath, project, target ],
    format: "[{target}] Compiling {filename}",
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
},
{
    ident: SwiftCompile,
    kind: Task,
    desc: r"SwiftCompile",
    captures: [ arch, filename, project, target ],
    format: "[{target}] Compiling {filename}",
    pattern: r"SwiftCompile\s(?P<arch>\S+)\s(?P<filepath>.+/)(?P<filename>[^/]+)\s\(in\starget\s'(?P<target>.*)'\sfrom\sproject\s'(?P<project>.*)'\)",
    tests: {
        "SwiftCompile normal arm64 /source/Home/State/HomeStateAction.swift (in target 'xxx' from project 'xxx')" =>
            |captures| {
                assert_eq!("normal", &captures["arch"]);
                assert_eq!("HomeStateAction.swift", &captures["filename"]);
                assert_eq!("xxx", &captures["project"]);
                assert_eq!("xxx", &captures["target"]);
            }
    }
},
{
    ident: CompileStoryboard,
    kind: Task,
    desc: r"CompileStoryboard",
    captures: [ filename, filepath, project, target ],
    format: "[{target}] Compiling {filename}",
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
},
{
    ident: CopyCommand,
    kind: Task,
    desc: r"CpResource|CpHeader|CopyStringsFile|CopyPlistFile",
    captures: [ kind, filename, filepath, project, target ],
    format: "[{target}] Copying {filename}",
    pattern: r"(?x)
               (:?Cp|Copy)(?P<kind>Resource|Header|PlistFile|StringsFile)\s.*\s
               (?P<filepath>.*/(?P<filename>.*\.(?:\w+)))
               (?:\s.*\((?:in\starget\s'(?P<target>.*)'\sfrom\sproject\s'(?P<project>.*)')\))?",
    tests: {
        "CpResource /output/EnWords.txt /path/to/EnWords.txt (in target 'Example' from project 'Example')" =>
            |captures| {
                assert_eq!("Resource", &captures["kind"]);
                assert_eq!("/path/to/EnWords.txt", &captures["filepath"]);
                assert_eq!("EnWords.txt", &captures["filename"]);
                assert_eq!("Example", &captures["project"]);
                assert_eq!("Example", &captures["target"]);
            },
        "CpHeader /output/file.h /path/to/file.h (in target 'Example' from project 'Example')" =>
            |captures| {
                assert_eq!("Header", &captures["kind"]);
                assert_eq!("/path/to/file.h", &captures["filepath"]);
                assert_eq!("file.h", &captures["filename"]);
                assert_eq!("Example", &captures["project"]);
                assert_eq!("Example", &captures["target"]);
            },
         "CopyStringsFile /output/InfoPlist.strings path/to/en.lproj/InfoPlist.strings (in target 'Example' from project 'Example')" => |captures| {
                assert_eq!("StringsFile", &captures["kind"]);
                assert_eq!("path/to/en.lproj/InfoPlist.strings", &captures["filepath"]);
                assert_eq!("InfoPlist.strings", &captures["filename"]);
                assert_eq!("Example", &captures["project"]);
                assert_eq!("Example", &captures["target"]);
            }
    }
},
{
    ident: CoverageReportGeneration,
    kind: Task,
    desc: r"Coverage report generation",
    captures: [ filepath ],
    format: "Generated code coverage report: {filepath}",
    pattern: r"(?i)generated\s+coverage\s+report:\s+(?P<filepath>.+)",
    tests: {
        "Generated coverage report: /path/to/code coverage.xccovreport" =>
            |captures| {
                assert_eq!("/path/to/code coverage.xccovreport", &captures["filepath"]);
            }
    }
},
{
    ident: GenerateDsymFile,
    kind: Task,
    desc: r"GenerateDSYMFile",
    captures: [ filename, target, project ],
    format: "[{target}] Generating {filename}",
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
},
{
    ident: Linking,
    kind: Task,
    desc: r"Ld",
    captures: [ filename, target, project ],
    format: "[{target}] Linking {filename}",
    pattern: r"Ld\s(?P<filepath>.*/(?P<filename>\w+\.\w+)).*\((?:in\starget\s'(?P<target>.*)'\sfrom\sproject\s'(?P<project>.*)')\)?",
    tests: {
        "Ld /path/to/file.o normal x86_64 (in target 'Example' from project 'Example')" =>
            |captures| {
                assert_eq!("/path/to/file.o", &captures["filepath"]);
                assert_eq!("file.o", &captures["filename"]);
                assert_eq!("Example", &captures["target"]);
                assert_eq!("Example", &captures["project"]);
            },
        "Ld /path/to/derive_data/Products/Debug-iphonesimulator/LookinServer.o normal (in target 'LookinServer' from project 'LookinServer')" =>
            |captures| {
                assert_eq!("/path/to/derive_data/Products/Debug-iphonesimulator/LookinServer.o", &captures["filepath"]);
                assert_eq!("LookinServer.o", &captures["filename"]);
                assert_eq!("LookinServer", &captures["target"]);
                assert_eq!("LookinServer", &captures["project"]);
            }
    }
},
{
    ident: RegisterExecutionPolicyException,
    kind: Task,
    desc: r"RegisterExecutionPolicyException",
    captures: [ ],
    format: "",
    pattern: r"RegisterExecutionPolicyException.*",
    tests: {}

},
// - TESTING ----------------------------------------------------------------------
{
    ident: TestExecuted,
    kind: Result,
    desc: r"Executed number of tests",
    captures: [ tests_count, failed_tests_count, unexpected_test_count, total_exec_time ],
    format: "",
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
},
{
    ident: TestExecutedWithSkipped,
    kind: Result,
    desc: r"Executed number of tests with skipped teats",
    captures: [ tests_count, skipped_test_count, failed_tests_count, unexpected_test_count, total_exec_time ],
    format: "",
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
},
{
    ident: KiwiFailingTest,
    kind: Error,
    desc: r"Kiwi Test failing",
    captures: [ filepath, suite, case, reason ],
    format: "",
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
},
{
    ident: UIFailingTest,
    kind: Error,
    desc: r"UI Test failing",
    captures: [ filepath, reason ],
    format: "",
    pattern: r"\s*t = \s+\d+\.\d+s\s+Assertion Failure: (?P<filepath>.*:\d+): (?P<reason>.*)$",
    tests: {
        "t =    22.27s             Assertion Failure: <unknown>:0: UI Testing Failure - Unable to find hit point for element Button 0x608001165880: {{74.0, -54.0}, {44.0, 38.0}}, label: 'Disconnect'" =>
            |captures| {
                assert_eq!("<unknown>:0", &captures["filepath"]);
                assert_eq!("UI Testing Failure - Unable to find hit point for element Button 0x608001165880: {{74.0, -54.0}, {44.0, 38.0}}, label: 'Disconnect'", &captures["reason"]);
            }
    }
},
{
    ident: TestSuiteStarted,
    kind: Test,
    desc: r"Test Suites Started",
    captures: [ name, time ],
    format: "",
    pattern: r"\s*Test Suite '(?:.*/)?(?P<name>.*[ox]ctest.*)' started at (?P<time>.*)",
    tests: {
        "Test Suite 'ObjectiveRecordTests.xctest' started at 2013-12-10 06:15:39 +0000" =>
            |captures| {
                assert_eq!("ObjectiveRecordTests.xctest", &captures["name"]);
                assert_eq!("2013-12-10 06:15:39 +0000", &captures["time"]);
            }
    }
},
{
    ident: TestSuiteCompleted,
    kind: Test,
    desc: r"Test Suites Completed",
    captures: [ name, time ],
    format: "",
    pattern: r"\s*Test Suite '(?:.*/)?(?P<name>.*[ox]ctest.*)' (finished|passed|failed) at (?P<time>.*)\.",
    tests: {
        "Test Suite 'ObjectiveRecordTests.xctest' finished at 2013-12-10 06:15:42 +0000." =>
            |captures| {
                assert_eq!("ObjectiveRecordTests.xctest", &captures["name"]);
                assert_eq!("2013-12-10 06:15:42 +0000", &captures["time"]);
            }
    }
},
{
    ident: TestCaseStarted,
    kind: Test,
    desc: r"Test Case Started",
    captures: [ suite, case],
    format: "",
    pattern: r"\s*Test Case '-\[(?P<suite>.*) (?P<case>.*)\]' started.$",
    tests: {
        "Test Case '-[viewUITests.vmtAboutWindow testConnectToDesktop]' started." =>
            |captures| {
                assert_eq!("viewUITests.vmtAboutWindow", &captures["suite"]);
                assert_eq!("testConnectToDesktop", &captures["case"]);
            }
    }
},
{
    ident: TestCasePassed,
    kind: Test,
    desc: r"Test Case Passed",
    captures: [ suite, case, time ],
    format: "",
    pattern: r"\s*Test Case\s'-\[(?P<suite>.*)\s(?P<case>.*)\]'\spassed\s\((?P<time>\d*\.\d{3})\sseconds\).",
    tests: {
        "Test Case '-[TestSuite TestCase]' passed (0.001 seconds)." =>
            |captures| {
                assert_eq!("TestSuite", &captures["suite"]);
                assert_eq!("TestCase", &captures["case"]);
                assert_eq!("0.001", &captures["time"]);
            }
    }
},
{
    ident: KiwiTestCasePending,
    kind: Test,
    desc: r"Kiwi test case pending",
    captures: [ suite, case ],
    format: "",
    pattern: r"Test Case\s'-\[(?P<suite>.*)\s(?P<case>.*)PENDING\]'\spassed",
    tests: {
        "Test Case '-[TestSuite TestCasePENDING]' passed (0.001 seconds)." =>
            |captures| {
                assert_eq!("TestSuite", &captures["suite"]);
                assert_eq!("TestCase", &captures["case"]);
            }
    }
},
{
    ident: TestCaseMeasure,
    kind: Test,
    desc: r"Test case measuring",
    captures: [ suite, case, time ],
    format: "",
    pattern: r"[^:]*:[^:]*:\sTest Case\s'-\[(?P<suite>.*)\s(?P<case>.*)\]'\smeasured\s\[Time,\sseconds\]\saverage:\s(?P<time>\d*\.\d{3})(.*){4}",
    tests: {
        r#"<unknown>:0: Test Case '-[TestSuite TestCase]' measured [Time, seconds] average: 0.013, relative standard deviation: 26.773%, values: [0.023838, 0.012034, ], performanceMetricID:com.apple.XCTPerformanceMetric_WallClockTime, baselineName: "", baselineAverage: , maxPercentRegression: 10.000%, maxPercentRelativeStandardDeviation: 10.000%, maxRegression: 0.100, maxStandardDeviation: 0.100"# =>
            |captures| {
                assert_eq!("TestSuite", &captures["suite"]);
                assert_eq!("TestCase", &captures["case"]);
                assert_eq!("0.013", &captures["time"]);
            }
    }
},
{
    ident: ParallelTestCasePassed,
    kind: Test,
    desc: r"Parallel TestCase passed",
    captures: [ suite, case, time, medium ],
    format: "",
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
},
{
    ident: ParallelTestCaseAppKitPassed,
    kind: Test,
    desc: r"Parallel TestCase AppKit Passed",
    captures: [ suite, case, time, medium ],
    format: "",
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
},
{
    ident: ParallelTestCaseFailed,
    kind: Error,
    desc: r"Parallel TestCase Failed",
    captures: [ suite, case, time, medium ],
    format: "",
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
},
{
    ident: ParallelTestingStarted,
    kind: Error,
    desc: r"Parallel Testing Started",
    captures: [ suite, case, time, medium ],
    format: "",
    pattern: r"Testing\s+started\s+on\s+'(?P<medium>.*)'",
    tests: {
        "Testing started on 'iPhone X'" =>
            |captures| {
                assert_eq!("iPhone X", &captures["medium"]);
            }
    }
},
{
    ident: ParallelTestingPassed,
    kind: Test,
    desc: r"Parallel Testing Passed",
    captures: [ suite, case, time, medium ],
    format: "",
    pattern: r"Testing\s+passed\s+on\s+'(?P<medium>.*)'",
    tests: {
        "Testing passed on 'iPhone X'" =>
            |captures| {
                assert_eq!("iPhone X", &captures["medium"]);
            }
    }
},
{
    ident: ParallelTestingFailed,
    kind: Error,
    desc: r"Parallel Testing Failed",
    captures: [ suite, case, time, medium ],
    format: "",
    pattern: r"Testing\s+failed\s+on\s+'(?P<medium>.*)'",
    tests: {
        "Testing failed on 'iPhone X'" =>
            |captures| {
                assert_eq!("iPhone X", &captures["medium"]);
            }
    }
},
{
    ident: ParallelTestFailed,
    kind: Error,
    desc: r"Parallel Testing Failed",
    captures: [ suite, case, time, medium ],
    format: "",
    pattern: r"(?i)\s*Test\s+Suite\s+'(?P<suite>.*)'\s+started\s+on\s+'(?P<medium>.*)'",
    tests: {
        "Test suite 'TestSuite (iOS).xctest' started on 'iPhone X'" =>
            |captures| {
                assert_eq!("TestSuite (iOS).xctest", &captures["suite"]);
                assert_eq!("iPhone X", &captures["medium"]);
            }
    }
},
{
    ident: PhaseScriptExecution,
    kind: Task,
    desc: r"PhaseScriptExecution",
    captures: [ name, target, project ],
    format: "[{target}] Executing {name}",
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
},
{
    ident: ProcessPCH,
    kind: Task,
    desc: r"ProcessPCH",
    captures: [ filename, target, project ],
    format: "[{target}] Processing {filename}",
    pattern: r"(?x)ProcessPCH(?:\+\+)?\s.*\s/.*/(?P<filename>.*.pch)( ?:\s.* \((?:in\starget\s      '(?P<target>.*)'\s  from\sproject\s   '(?P<project>.*)' )\)  ) ?",
    tests: {
        "ProcessPCH /path/to/file.pch.gch /path/to/file.pch normal x86_64 objective-c com.apple.compilers.llvm.clang.1_0.analyzer (in target 'App' from project 'App')" =>
            |captures| {
                assert_eq!("file.pch", &captures["filename"]);
                assert_eq!("App", &captures["target"]);
                assert_eq!("App", &captures["project"]);
            }
    }
},
{
    ident: ProcessPCHCommand,
    kind: Task,
    desc: r"ProcessPchCommand",
    captures: [ ],
    format: "",
    pattern: r"\s*.*/usr/bin/clang\s.*\s\-c\s(.*.pch)\s.*\-o\s.*",
    tests: {}
},
{
    ident: PbxCopy,
    kind: Task,
    desc: r"PBXCp",
    captures: [ filename, target, project ],
    format: "[{target}] Copying {filename}",
    pattern: r"(?x)PBXCp\s(?P<filepath>/.*)\s/.*\((?:in\starget\s'(?P<target>.*)'\sfrom\sproject\s'(?P<project>.*)')\)",
    tests: {
        "PBXCp /path/to/header.h /path/to/output.h (in target 'App' from project 'App')" =>
            |captures| {
                assert_eq!("App" ,&captures["target"]);
                assert_eq!("App" ,&captures["project"]);
                assert_eq!("/path/to/header.h" ,&captures["filepath"]);
            }
    }
},
{
    ident: ProcessInfoPlistFile,
    kind: Task,
    desc: r"ProcessInfoPlistFile",
    captures: [ filename, filepath, target, project ],
    format: "[{target}] Processing {filename}",
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
},
{
    ident: CheckDependencies,
    kind: Task,
    desc: r"Check dependencies",
    captures: [],
    format: "Checking dependencies",
    pattern: r"Check dependencies",
    tests: {}
},
{
    ident: RestartingTests,
    kind: Test,
    desc: r"Test restarting",
    captures: [],
    format: "",
    pattern:r"Restarting after unexpected exit.+$",
    tests: {}
},
{
    ident: CoverageDataGeneration,
    kind: Task,
    desc: r"Coverage Data Generation",
    captures: [],
    format: "Generating code coverage data...",
    pattern: r"generating\s+coverage\s+data\.*",
    tests: {}
},
{
    ident: PhaseSuccess,
    kind: Result,
    desc: r"Phase Success",
    captures: [ name ],
    format: "[{name}] Succeeded",
    pattern: r"\*\*\s(P?<name>.*)\sSUCCEEDED\s\*\*",
    tests:{}
},
{
    ident: TestSuiteAllTestsPassed,
    kind: Test,
    desc: r"Test Suite All Tests Passed",
    captures: [],
    format: "",
    pattern: r"\s*Test Suite 'All tests' passed at",
    tests:{}
},
{
    ident: TestSuiteAllTestsFailed,
    kind: Error,
    desc: r"Test Suite All Tests Passed",
    captures: [],
    format: "",
    pattern: r"\s*Test Suite 'All tests' failed at",
    tests: {}
},
{
    ident: Touch,
    kind: Task,
    desc: r"Touch file",
    captures: [ filename, filepath, target, project ],
    format: "[{target}] Touching {filename}",
    pattern: r"(?x)Touch\s(?P<filepath>/(?:\.|[^\s])+/(?P<filename>(?:\.|[^\s])+\.(?:\w+)))
        (?:\s.*\((?:in\starget\s'(?P<target>.*)'\sfrom\sproject\s'(?P<project>.*)')\))?",
    tests: {
        "Touch /BUILD_ROOT/Example.app (in target 'Example' from project 'Example')" =>
            |captures| {
                assert_eq!("/BUILD_ROOT/Example.app", &captures["filepath"]);
                assert_eq!("Example.app", &captures["filename"]);
                assert_eq!("Example", &captures["project"]);
                assert_eq!("Example", &captures["target"]);
            }
    }
},
// - Warning ----------------------------------------------------------------------
{
    ident: CompileWarning,
    kind: Warning,
    desc: r"Compile Warning",
    captures: [ location, filepath, message ],
    format: "{location}: {message}",
    pattern: r"(?P<location>(?P<filepath>[^:]*):\d*:\d*):\swarning:\s(?P<message>.*)$",
    tests: {
        "/path/file.swift:64:69: warning: 'flatMap' is deprecated: Please use compactMap(_:) for the case where closure returns an optional value" =>
            |captures| {
                assert_eq!("/path/file.swift:64:69", &captures["location"]);
                assert_eq!("/path/file.swift", &captures["filepath"]);
                assert_eq!("'flatMap' is deprecated: Please use compactMap(_:) for the case where closure returns an optional value", &captures["message"]);
            }
    }
},
{
    ident: LdWarning,
    kind: Warning,
    desc: r"Linking Warning",
    captures: [ prefix, message ],
    format: "{prefix}{message}",
    pattern: r"(P?<prefix>ld:.*)warning: (?P<message>.*)",
    tests: {}
},
{
    ident: GenericWarning,
    kind: Warning,
    desc: r"Generic Error (catch all)",
    captures: [ message ],
    format: "{message}",
    pattern: r"warning:\s(?P<message>.*)$",
    tests: {}
},
{
    ident: CodeSignWarning,
    kind: Warning,
    desc: r"Sign warning",
    captures: [ message ],
    format: "{message}",
    pattern: r"(?P<message>.* will not be code signed because .*)$",
    tests: {}
},
// - Error ------------------------------------------------------------------------
{
    ident: ClangError,
    kind: Error,
    desc: r"Clang Error",
    captures: [ message ],
    format: "{message}",
    pattern: r"(?P<message>clang: error:.*)$",
    tests: {
        "clang: error: linker command failed with exit code 1 (use -v to see invocation)" =>
        |captures| {
            assert_eq!("clang: error: linker command failed with exit code 1 (use -v to see invocation)", &captures["message"])
        }
    }
},
{
    ident: CheckDependenciesError,
    kind: Error,
    desc: r"Check Dependencies error",
    captures: [ message ],
    format: "{message}",
    pattern: r"(?P<message>Code\s?Sign error:.*|Code signing is required for product type .* in SDK .*|No profile matching .* found:.*|Provisioning profile .* doesn't .*|Swift is unavailable on .*|.?Use Legacy Swift Language Version.*)$",
    tests: {}
},
{
    ident: ProvisioningProfileRequiredError,
    kind: Error,
    desc: r"General Check Depeds error",
    captures: [ message ],
    format: "{message}",
    pattern: r"(?P<message>.*requires a provisioning profile.*)$",
    tests: {}
},
{
    ident: NoCertificateError,
    kind: Error,
    desc: r"No certificate error",
    captures: [ message ],
    format: "{message}",
    pattern: r"(?P<message>No certificate matching.*)$",
    tests: {}
},
{
    ident: CompileError,
    kind: Error,
    desc: r"Compile Error",
    captures: [ location, filepath, message ],
    format: "{location}: {message}",
    pattern: r"\s*(?P<location>(?P<filepath>[^:]*):\d*:\d*):\s(?:fatal\s)?error:\s(?P<message>.*)$",
    tests: {
        "/path/file.swift:64:69: error: cannot find 'input' in scope" =>
        |captures| {
            assert_eq!("/path/file.swift:64:69", &captures["location"]);
            assert_eq!("/path/file.swift", &captures["filepath"]);
            assert_eq!("cannot find 'input' in scope", &captures["message"]);
        }
    }
},
{
    ident: Cursor,
    kind: Warning,
    desc: r"Cursor",
    captures: [ content ],
    format: " {content}",
    pattern: r"(?P<content>[\s~]*\^[\s~]*)$",
    tests: {}
},
{
    ident: FatalError,
    kind: Error,
    desc: r"Compile Error",
    captures: [ message ],
    format: "{message}",
    pattern: r"(?P<message>fatal error:.*)$",
    tests: {}
},
{
    ident: FileMissingError,
    kind: Error,
    desc: r"File missing Error",
    captures: [ message, filepath ],
    format: "{filepath}: {message}",
    pattern: r"<unknown>:0:\s(?P<message>error:\s.*)\s'(?P<filepath>/.+/.*\..*)'$",
    tests: {}
},
{
    ident: LdError,
    kind: Error,
    desc: r"Ld Error",
    captures: [ message ],
    format: "{message}",
    pattern: r"(P<message>ld:.*)",
    tests: {}
},
{
    // TODO: Add tests
    // NOTE: Bad format, the command ran is the message
    ident: LinkerDuplicateSymbolsLocationError,
    kind: Error,
    desc: r"duplicate symbols location",
    captures: [ message ],
    format: "{message}",
    pattern: r"  (?P<message>/.*\.o[\)]?)$",
    tests: {}
},
{
    ident: LinkerDuplicateSymbolsError,
    kind: Error,
    desc: r"Linker Duplicate Symbols Error",
    captures: [ message ],
    format: "{message}",
    pattern: r"(?P<message>duplicate symbol .*):$",
    tests: {}
},
{
    ident: LinkerUndefinedSymbolsLocationError,
    kind: Error,
    desc: r"Linker Undefined Symbols Location Error",
    captures: [ message ],
    format: "{message}",
    pattern: r"(P?<message>.* in .*\.o)$",
    tests: {}
},
{
    ident: LinkerUndefinedSymbolsError,
    kind: Error,
    desc: r"Undefined symbols",
    captures: [ message ],
    format: "{message}",
    pattern: r"(P?<message>.* in .*\.o)$",
    tests: {}
},
{
    ident: PodsError,
    kind: Error,
    desc: r"Pods error",
    captures: [ message ],
    format: "{message}",
    pattern: r"(P?<message>error:\s.*)",
    tests: {}
},
{
    ident: SymbolReferencedFrom,
    kind: Error,
    desc: r"Symbol reference from error",
    captures: [ message ],
    format: "{message}",
    pattern: "\\s+\"(?P<message>.*)\", referenced from:$",
    tests: {}
},
{
    ident: ModuleIncludesError,
    kind: Error,
    desc: r"module includes error",
    captures: [ message ],
    format: "{message}",
    pattern: r"<module-includes>:.*?:.*?:\s(?:fatal\s)?(P?<message>error:\s.*)$/",
    tests: {}
},
{
    ident: UndefinedSymbolLocationError,
    kind: Error,
    desc: r"Undefined symol location",
    captures: [ message ],
    format: "{message}",
    pattern: r".+ in (.+)\((.+)\.o\)$",
    tests: {}
},
{
    ident: PackageGraphResolvingStart,
    kind: Task,
    desc: r"Package Graph Resolving Start",
    captures: [  ],
    format: "Resolving Packages",
    pattern: r"\s*Resolve Package Graph\s*$",
    tests: {}
},
{
    ident: PackageGraphResolvingEnd,
    kind: Task,
    desc: r"Package Graph Resolving Ended",
    captures: [  ],
    format: "",
    pattern: r"Resolved source packages:$",
    tests: {}
},
{
    ident: PackageGraphResolvedItem,
    kind: Task,
    desc: r"Package Graph Resolved Item",
    captures: [ ],
    format: "",
    pattern: r"\s*([^\s:]+):\s([^ ]+)\s@\s(\d+\.\d+\.\d+)",
    tests: {}
}
];
