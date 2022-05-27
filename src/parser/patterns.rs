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
