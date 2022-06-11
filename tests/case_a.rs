use xcodebuild::parser::*;
// Right now manual is 1040 faster then regex!

#[test]
#[cfg(feature = "with_regex")]
fn test_regex() {
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

#[test]
#[cfg(feature = "manual")]
fn test_old() {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        use async_stream::stream;
        use process_stream::{ProcessItem, StreamExt};

        let lines = include_str!("./case_a.log").split("\n");
        let mut stream = stream! {
            for line in lines {
                yield ProcessItem::Output(line.to_string())
            }
        }
        .boxed();

        while let Some(update) = stream.next().await {
            if let ProcessItem::Output(line) = update {
                if !line.is_empty() {
                    match parse_step_from_stream(line, &mut stream).await {
                        Ok(v) => {
                            if let Some(steps) = v {
                                for step in steps.into_iter() {
                                    println!("{step}")
                                }
                            }
                        }
                        Err(e) => {
                            tracing::error!("Fail to parse step {e}");
                            tracing::error!("{e}")
                        }
                    }
                }
            }
        }
    })
}
