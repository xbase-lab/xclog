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

