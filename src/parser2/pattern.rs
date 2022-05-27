use lazy_regex::*;

/**
    Analyze/AnalyzeShallow Command capture groups:

    - $file_path: Analyzed file path;
    - $file_name: Analyzed file name;
    - $target: Target Name;
    - $project: Project Name;
*/
pub static ANALYZE: Lazy<Regex> = lazy_regex! {
    r"(?x)
      Analyze(?:Shallow)?\s

      # File path and file name
      (
          # File Path
          ?P<file_path>.*/

          # File Name
          (?P<file_name>.*\.(?:mm|m|cc|cpp|c|cxx))
      )

      # Optional Whitespace
      (?:\s.*\(
          (?:
           # Target Name
           in\starget\s      '(?P<target>.*)'\s
           # Project Name
           from\sproject\s   '(?P<project>.*)'
           )
      \))?
      "
};

#[test]
fn test_analyze() {
    let text = r#"AnalyzeShallow /path/to/file.m normal x86_64 (in target 'MyTarget' from project 'MyProject')"#;
    let captures = ANALYZE.captures(text).unwrap();
    assert_eq!("/path/to/file.m", &captures["file_path"]);
    assert_eq!("file.m", &captures["file_name"]);
    assert_eq!("MyTarget", &captures["target"]);
    assert_eq!("MyProject", &captures["project"]);

    let text = r#"AnalyzeShallow /path/to/file.c"#;
    let captures = ANALYZE.captures(text).unwrap();
    assert_eq!("/path/to/file.c", &captures["file_path"]);
    assert_eq!("file.c", &captures["file_name"]);

    let text = "Analyze /path/to/file.mm";
    let captures = ANALYZE.captures(text).unwrap();
    assert_eq!("/path/to/file.mm", &captures["file_path"]);
    assert_eq!("file.mm", &captures["file_name"]);
}
