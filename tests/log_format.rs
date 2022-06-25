use process_stream::*;
use std::pin::Pin;
use xclog::parser::XCOutput;
use xclog::parser::{parse, XCOutputTask};

async fn get_stream(lines: Vec<String>) -> Pin<Box<dyn Stream<Item = XCOutput> + Send>> {
    let mut lines = lines.into_iter();
    let mut output_stream = async_stream::stream! {
        while let Some(line) = lines.next() { yield ProcessItem::Output(line) }
    }
    .boxed();

    async_stream::stream! {
        while let Some(output) = output_stream.next().await {
            match output {
                ProcessItem::Output(line) | ProcessItem::Error(line) => {
                    match parse(line, &mut output_stream).await {
                        Ok(Some(lines)) => { for line in lines.into_iter() { yield line } },
                        Err(e) => tracing::error!("ParseError: {e}"),
                        _ => ()
                    }
                },
                ProcessItem::Exit(status) => yield XCOutput {
                    value: format!("[Exit] {status}"), kind: XCOutputTask::Result
                }
            }
        }
    }
    .boxed()
}

fn get_case_lines(content: &str) -> Vec<String> {
    content
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}

#[test]
#[ignore = "passing"]
fn case_a() {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let lines = get_case_lines(include_str!("./case_a.log"));
        let mut stream = get_stream(lines).await;
        while let Some(line) = stream.next().await {
            println!("{line}");
        }
    })
}

#[test]
#[ignore = "passing"]
fn case_b() {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let lines = get_case_lines(include_str!("./case_b.log"));
        let mut stream = get_stream(lines).await;
        while let Some(line) = stream.next().await {
            println!("{line}");
        }
    })
}

#[test]
fn case_c() {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let lines = get_case_lines(include_str!("./case_c.log"));
        let mut stream = get_stream(lines).await;
        while let Some(line) = stream.next().await {
            println!("{line}");
        }
    })
}
