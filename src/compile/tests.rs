use super::*;

fn get_compile_commands(content: &str) -> XCCompilationDatabase {
    XCCompilationDatabase::try_from_lines(
        content
            .split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>(),
    )
}

async fn get_compile_commands_from_local_case_d() -> XCCompilationDatabase {
    let root = "/Users/tami5/repos/swift/yabaimaster";
    XCCompilationDatabase::generate(root, &[
        "clean",
        "build",
        "-configuration",
        "Debug",
        "-target",
        "YabaiMaster",
        "SYMROOT=/Users/tami5/Library/Caches/Xbase/swift_yabaimaster/YabaiMaster_Debug",
        "CONFIGURATION_BUILD_DIR=/Users/tami5/Library/Caches/Xbase/swift_yabaimaster/YabaiMaster_Debug",
        "BUILD_DIR=/Users/tami5/Library/Caches/Xbase/swift_yabaimaster/YabaiMaster_Debug"
        ]).await.unwrap()
}

mod compile_commands {
    use super::*;
    macro_rules! test_compile_commands_output {
        ($cmd:ident, $($idx:literal: $key:ident, $value:expr),*) => {
            $(
                assert_eq!($cmd[$idx].$key, $value);
             )*
        };
    }

    #[tokio::test]
    #[tracing_test::traced_test]
    async fn case_a() {
        let compile_commands = get_compile_commands(include_str!("../../tests/case_a.log"));

        assert_eq!(compile_commands.len(), 3);
        test_compile_commands_output! { compile_commands,
            0: name, Some("Logging".to_string()),
            0: directory, String::from("/PACKAGES_DIR/swift-log"),
            0: file_lists, vec![PathBuf::from("/BUILD_ROOT/swift-log.build/Debug-iphoneos/Logging.build/Objects-normal/armv7/Logging.SwiftFileList")],
            1: name, Some("Logging".to_string()),
            1: directory, String::from("/PACKAGES_DIR/swift-log"),
            1: file_lists, vec![PathBuf::from("/BUILD_ROOT/swift-log.build/Debug-iphoneos/Logging.build/Objects-normal/arm64/Logging.SwiftFileList")],
            2: name, Some("Example".to_string()),
            2: directory, String::from("/PROJECT_ROOT"),
            2: file_lists, vec![PathBuf::from("/BUILD_ROOT/Example.build/Debug-iphoneos/Example.build/Objects-normal/arm64/Example.SwiftFileList")]
        };

        let file_path = PathBuf::from("/tmp/case_a_compile_commands.json");
        compile_commands.write(&file_path).await.unwrap();
        assert!(file_path.exists());
        let from_filepath = XCCompilationDatabase::try_from_filepath(&file_path).unwrap();
        assert_eq!(compile_commands.len(), from_filepath.len());
        std::fs::remove_file(file_path).unwrap();
    }

    #[tokio::test]
    #[tracing_test::traced_test]
    async fn case_b() {
        let compile_commands = get_compile_commands(include_str!("../../tests/case_b.log"));

        assert_eq!(compile_commands.len(), 12);
        let file_path = PathBuf::from("/tmp/case_b_compile_commands.json");
        compile_commands.write(&file_path).await.unwrap();
        assert!(file_path.exists());
        let from_filepath = XCCompilationDatabase::try_from_filepath(&file_path).unwrap();
        assert_eq!(compile_commands.len(), from_filepath.len());
        std::fs::remove_file(file_path).unwrap();

        test_compile_commands_output! {compile_commands,
        0: name, Some("ArgumentParserToolInfo".to_string()),
        0: directory, String::from("/PACKAGES_DIR/swift-argument-parser"),
        0: file_lists, vec![PathBuf::from("/BUILD_ROOT/swift-argument-parser.build/Debug/ArgumentParserToolInfo.build/Objects-normal/x86_64/ArgumentParserToolInfo.SwiftFileList")],
        1: name, Some("ArgumentParserToolInfo".to_string()),
        1: directory, String::from("/PACKAGES_DIR/swift-argument-parser"),
        1: file_lists, vec![PathBuf::from("/BUILD_ROOT/swift-argument-parser.build/Debug/ArgumentParserToolInfo.build/Objects-normal/arm64/ArgumentParserToolInfo.SwiftFileList")],
        2: name, Some("ArgumentParser".to_string()),
        2: directory, String::from("/PACKAGES_DIR/swift-argument-parser"),
        2: file_lists, vec![PathBuf::from("/BUILD_ROOT/swift-argument-parser.build/Debug/ArgumentParser.build/Objects-normal/x86_64/ArgumentParser.SwiftFileList")],
        3: name, Some("ArgumentParser".to_string()),
        3: directory, String::from("/PACKAGES_DIR/swift-argument-parser"),
        3: file_lists, vec![PathBuf::from("/BUILD_ROOT/swift-argument-parser.build/Debug/ArgumentParser.build/Objects-normal/arm64/ArgumentParser.SwiftFileList")],
        4: name, Some("Socket".to_string()),
        4: directory, String::from("/PACKAGES_DIR/BlueSocket"),
        4: file_lists, vec![PathBuf::from("/BUILD_ROOT/Socket.build/Debug/Socket.build/Objects-normal/arm64/Socket.SwiftFileList")],
        5: name, Some("Socket".to_string()),
        5: directory, String::from("/PACKAGES_DIR/BlueSocket"),
        5: file_lists, vec![PathBuf::from("/BUILD_ROOT/Socket.build/Debug/Socket.build/Objects-normal/x86_64/Socket.SwiftFileList")],
        6: name, Some("SwiftyBeaver".to_string()),
        6: directory, String::from("/PACKAGES_DIR/SwiftyBeaver"),
        6: file_lists, vec![PathBuf::from("/BUILD_ROOT/SwiftyBeaver.build/Debug/SwiftyBeaver.build/Objects-normal/x86_64/SwiftyBeaver.SwiftFileList")],
        7: name, Some("SwiftyBeaver".to_string()),
        7: directory, String::from("/PACKAGES_DIR/SwiftyBeaver"),
        7: file_lists, vec![PathBuf::from("/BUILD_ROOT/SwiftyBeaver.build/Debug/SwiftyBeaver.build/Objects-normal/arm64/SwiftyBeaver.SwiftFileList")],
        8: name, Some("Example".to_string()),
        8: directory, String::from("/PROJECT_ROOT"),
        8: file_lists, vec![PathBuf::from("/BUILD_ROOT/Example.build/Debug/Example.build/Objects-normal/x86_64/Example.SwiftFileList")],
        9: name, Some("Example".to_string()),
        9: directory, String::from("/PROJECT_ROOT"),
        9: file_lists, vec![PathBuf::from("/BUILD_ROOT/Example.build/Debug/Example.build/Objects-normal/arm64/Example.SwiftFileList")],
        10: name, None,
        10: directory, String::from("/"),
        10: file, Some(PathBuf::from("/PROJECT_ROOT/src/client/bridge.c")),
        11: name, None,
        11: directory, String::from("/"),
        11: file, Some(PathBuf::from("/PROJECT_ROOT/src/client/bridge.c"))
        }
    }

    #[tokio::test]
    #[tracing_test::traced_test]
    async fn case_c() {
        let compile_commands = get_compile_commands(include_str!("../../tests/case_c.log"));
        // THIS DOESN'T FEEL CORRECT
        assert_eq!(compile_commands.len(), 104)
    }

    #[tokio::test]
    #[tracing_test::traced_test]
    #[ignore = "Local tests"]
    async fn test_get_compile_commands() {
        let compile_commands = get_compile_commands_from_local_case_d().await;

        println!("{:#?}", compile_commands.len());
        for command in compile_commands.iter() {
            if let Some(ref command) = command.name {
                println!("{:?}", command);
            } else if let Some(ref file) = command.file {
                println!("{:?}", file);
            } else {
                println!("{:?}", command);
            }
        }
        // In the case above the compile commands is indeed 12
        assert_eq!(compile_commands.len(), 12);
    }
}
