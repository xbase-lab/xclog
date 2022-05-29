use xcodebuild::parser::*;

#[test]
fn test() {
    let lines = include_str!("./case_a.log").split("\n");
    println!("starting");
    for line in lines {
        if let Some(parser) = MATCHER.get_parser_for(line) {
            if let Some(line) = parser.format(line) {
                println!("{line}");
            }
        }
    }
}
