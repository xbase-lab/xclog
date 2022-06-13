use xcodebuild::parser::*;

#[test]
#[cfg(feature = "with_regex")]
fn test_regex_format_case_a() {
    let lines = include_str!("./case_a.log").split("\n");
    for line in lines {
        if let Some(m) = MATCHER.capture(line) {
            if let Some(line) = m.output().value {
                println!("{line}");
            }
        }
    }
}

#[test]
#[cfg(feature = "with_regex")]
fn test_regex_format_case_b() {
    let lines = include_str!("./case_b.log").split("\n");
    for line in lines {
        if let Some(m) = MATCHER.capture(line) {
            if let Some(line) = m.output().value {
                println!("{line}");
            }
        }
    }
}

#[test]
#[cfg(feature = "manual")]
fn test_manual_case_a() {
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
