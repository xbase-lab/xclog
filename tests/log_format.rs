use process_stream::StreamExt;
use xclog::*;

#[test]
#[ignore = "passing"]
fn case_a() {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let lines = get_case_lines(include_str!("./case_a.log"));
        let mut stream = get_log_stream_from_lines(lines).await.unwrap();
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
        let mut stream = get_log_stream_from_lines(lines).await.unwrap();
        while let Some(line) = stream.next().await {
            println!("{line}");
        }
    })
}

#[test]
fn case_c() {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let lines = get_case_lines(include_str!("./case_c.log"));
        let mut stream = get_log_stream_from_lines(lines).await.unwrap();
        while let Some(line) = stream.next().await {
            println!("{line}");
        }
    })
}

fn get_case_lines(content: &str) -> Vec<String> {
    content
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}
